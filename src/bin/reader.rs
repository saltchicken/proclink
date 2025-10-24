use shared_memory::ShmemConf;
use std::str;
use std::sync::atomic::{AtomicU8, Ordering};

mod common;
use common::*;

fn main() {
    let shmem = ShmemConf::new()
        .os_id("my_synchronized_shmem")
        .open()
        .expect("Failed to open shared memory. Is the writer running?");

    println!(
        "[Reader] Attached to shared memory. Pointer: {:p}",
        shmem.as_ptr()
    );

    let flag = unsafe {
        let flag_ptr = shmem.as_ptr().add(FLAG_INDEX); // ‼️ Use .add() to avoid the cast
        &*(flag_ptr as *const AtomicU8)
    };

    // Atomically check if the flag is FLAG_WRITTEN. If it is, set it to FLAG_READ.
    match flag.compare_exchange(FLAG_WRITTEN, FLAG_READ, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(_) => {
            // Success! The flag was 1, and we set it to 0. We can now read.
            let data_ptr = unsafe { shmem.as_ptr().add(DATA_INDEX) };
            let message_len = "Hello from the writer!".len();

            let msg_bytes = unsafe { std::slice::from_raw_parts(data_ptr, message_len) };
            let message = str::from_utf8(msg_bytes).unwrap();

            println!("[Reader] ✅ Read new data: \"{}\"", message);
        }
        Err(_) => {
            // The flag was not 1, meaning there is no new data to read.
            println!("[Reader] ⚠️ No new data to read.");
        }
    }
}
