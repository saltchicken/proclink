// ‼️ Import the new error type and std::process
use proclink::ShmemWriter;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    const PAYLOAD_SIZE: usize = 4096;

    let writer = match ShmemWriter::new("my_synchronized_shmem", PAYLOAD_SIZE) {
        Ok(w) => {
            println!(
                "[Writer] Attached to shared memory. Now writing in a loop. Press Ctrl+C to exit."
            );
            w
        }
        Err(e) => {
            eprintln!("[Writer] ❌ Failed to create or open shared memory: {}", e);
            process::exit(1);
        }
    };

    let mut counter: u64 = 0; // Use u64 for 8 bytes
    loop {
        // Create a 4096-byte payload, initialized to zero
        let mut message: Vec<u8> = vec![0; 4096];
        // As an example, write the counter to the first 8 bytes
        let counter_bytes = counter.to_le_bytes();
        message[0..counter_bytes.len()].copy_from_slice(&counter_bytes);

        match writer.write(&message) {
            Ok(true) => {
                println!(
                    "[Writer] ✅ Wrote new data (4096 bytes). Count: {}",
                    counter
                );
                counter += 1;
            }
            Ok(false) => {
                // Reader hasn't read the last message yet, just wait.
            }
            Err(e) => {
                eprintln!("[Writer] ❌ Error writing: {}", e);
                break;
            }
        }
        thread::sleep(Duration::from_secs(2));
    }
}
