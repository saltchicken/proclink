use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::sync::atomic::{AtomicU8, Ordering};

/// Flag is 0: The buffer is "empty" or has been read by the reader.
pub const FLAG_READ: u8 = 0;
/// Flag is 1: The buffer has been written to by the writer.
pub const FLAG_WRITTEN: u8 = 1;

/// The index of the flag in shared memory. (1 byte)
pub const FLAG_INDEX: usize = 0;
/// The index where the message length is stored. (4 bytes for u32)
pub const LEN_INDEX: usize = 1;
/// The index where the actual message data starts. (1 + 4)
pub const DATA_INDEX: usize = 5;

pub struct ShmemWriter {
    shmem: Shmem,
    payload_size: usize,
}

impl ShmemWriter {
    /// Creates or opens the shared memory segment for writing.
    pub fn new(os_id: &str, payload_size: usize) -> Result<Self, ShmemError> {
        let total_size = DATA_INDEX + payload_size;
        let shmem = ShmemConf::new()
            .size(total_size)
            .os_id(os_id)
            .open()
            .or_else(|_| ShmemConf::new().size(total_size).os_id(os_id).create())?;

        if shmem.len() != total_size {
            // TODO: Handle error properly
            println!(
                "Shared memory segment size mismatch. Is another process using this ID with a different size?"
            );
        }

        // Initialize flag to READ state so we can write immediately.
        let flag = unsafe { &*(shmem.as_ptr().add(FLAG_INDEX) as *const AtomicU8) };
        flag.store(FLAG_READ, Ordering::SeqCst);

        Ok(Self {
            shmem,
            payload_size,
        })
    }

    /// Attempts to write data to shared memory.
    /// Returns Ok(true) on successful write.
    /// Returns Ok(false) if the buffer is still full (not read yet).
    /// Returns Err if the message is too large.
    pub fn write(&self, message: &[u8]) -> Result<bool, &'static str> {
        // Check if the message fits
        if message.len() > self.payload_size {
            return Err("Message is too large for shared memory segment.");
        }

        let flag = unsafe { &*(self.shmem.as_ptr().add(FLAG_INDEX) as *const AtomicU8) };

        // Try to atomically swap the flag from READ (0) to WRITTEN (1)
        match flag.compare_exchange(FLAG_READ, FLAG_WRITTEN, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                // Success! We now have exclusive access to write.
                unsafe {
                    // Write the length of the message first
                    let len_ptr = self.shmem.as_ptr().add(LEN_INDEX) as *mut u32;
                    len_ptr.write(message.len() as u32);

                    // Write the actual message data
                    let data_ptr = self.shmem.as_ptr().add(DATA_INDEX);
                    std::ptr::copy_nonoverlapping(message.as_ptr(), data_ptr, message.len());
                }
                Ok(true) // Wrote data
            }
            Err(_) => {
                // The flag was not 0, so the reader hasn't read the last message.
                Ok(false) // Did not write
            }
        }
    }
}

pub struct ShmemReader {
    shmem: Shmem,
    payload_size: usize,
}

impl ShmemReader {
    /// Opens the shared memory segment for reading.
    pub fn new(os_id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new().os_id(os_id).open()?;
        let total_size = shmem.len();
        if total_size < DATA_INDEX {
            // TODO: Handle error properly
            println!("Shared memory segment is too small.");
        }
        let payload_size = total_size - DATA_INDEX;
        Ok(Self {
            shmem,
            payload_size,
        })
    }

    /// Attempts to read data from shared memory.
    /// Returns Ok(Some(Vec<u8>)) if new data was read.
    /// Returns Ok(None) if there is no new data to read.
    pub fn read(&self) -> Result<Option<Vec<u8>>, &'static str> {
        let flag = unsafe { &*(self.shmem.as_ptr().add(FLAG_INDEX) as *const AtomicU8) };

        // Try to atomically swap the flag from WRITTEN (1) to READ (0)
        match flag.compare_exchange(FLAG_WRITTEN, FLAG_READ, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                // Success! The flag was 1, we set it to 0. We can read.
                let data;
                unsafe {
                    // Read the message length first
                    let len_ptr = self.shmem.as_ptr().add(LEN_INDEX) as *const u32;
                    let message_len = len_ptr.read() as usize;

                    // Check for potential buffer over-read
                    if message_len > self.payload_size {
                        // This indicates corrupted memory or a misbehaving writer.
                        // We set the flag back to READ but return an error.
                        return Err("Read invalid data length from shared memory.");
                    }

                    // Read the data using the dynamic length
                    let data_ptr = self.shmem.as_ptr().add(DATA_INDEX);
                    let msg_slice = std::slice::from_raw_parts(data_ptr, message_len);
                    data = msg_slice.to_vec(); // Copy data out
                }
                Ok(Some(data)) // Return the data
            }
            Err(_) => {
                // The flag was not 1, meaning no new data to read.
                Ok(None)
            }
        }
    }
}
