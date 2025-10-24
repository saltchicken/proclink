/// Flag is 0: The buffer is "empty" or has been read by the reader.
pub const FLAG_READ: u8 = 0;
/// Flag is 1: The buffer has been written to by the writer.
pub const FLAG_WRITTEN: u8 = 1;

/// The index of the flag in shared memory.
pub const FLAG_INDEX: usize = 0;
/// The index where the actual message data starts.
pub const DATA_INDEX: usize = 1;
/// The total size of the shared memory mapping.
pub const SHMEM_SIZE: usize = 4096;
