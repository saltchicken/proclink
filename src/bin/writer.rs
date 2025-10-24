use shared_memory::ShmemConf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::time::Duration;

mod common;
use common::*;

fn main() {
    let shmem = ShmemConf::new()
        .size(SHMEM_SIZE)
        .os_id("my_synchronized_shmem")
        .open()
        .or_else(|_| {
            ShmemConf::new()
                .size(SHMEM_SIZE)
                .os_id("my_synchronized_shmem")
                .create()
        })
        .expect("Failed to open or create shared memory");

    println!(
        "[Writer] Attached to shared memory. Pointer: {:p}",
        shmem.as_ptr()
    );
    println!("[Writer] Now running in a loop. Press Ctrl+C to exit.");

    let flag = unsafe { &*(shmem.as_ptr().add(FLAG_INDEX) as *const AtomicU8) };

    // When we first start (or restart), ensure the state is clean.
    // Set the flag to READ so we can write a new message immediately.
    flag.store(FLAG_READ, Ordering::SeqCst);
    println!("[Writer] Initialized flag to READ state.");

    // TODO: When loop breaks, cleanup shared memory.
    loop {
        match flag.compare_exchange(FLAG_READ, FLAG_WRITTEN, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                let message = b"Hello from the writer!";
                let data_ptr = unsafe { shmem.as_ptr().add(DATA_INDEX) };
                unsafe {
                    std::ptr::copy_nonoverlapping(message.as_ptr(), data_ptr, message.len());
                }
                println!("[Writer] ✅ Wrote new data to shared memory.");
            }
            Err(_) => {
                // The flag was not 0, so we do nothing.
            }
        }

        thread::sleep(Duration::from_secs(2));
    }
}
