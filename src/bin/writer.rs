use proclink::ShmemWriter;
use std::thread;
use std::time::Duration;
// This main function just sets up the writer and runs the loop.
fn main() {
    let writer =
        ShmemWriter::new("my_synchronized_shmem").expect("Failed to open or create shared memory");
    println!("[Writer] Attached to shared memory. Now writing in a loop. Press Ctrl+C to exit.");
    let mut counter: u64 = 0; // ‼️ Use u64 for 8 bytes
    loop {
        // ‼️ Create a 4096-byte payload, initialized to zero
        let mut message: Vec<u8> = vec![0; 4096];

        // ‼️ As an example, write the counter to the first 8 bytes
        let counter_bytes = counter.to_le_bytes();
        message[0..counter_bytes.len()].copy_from_slice(&counter_bytes);

        // ‼️ Call the library's write function with the 4096-byte slice
        match writer.write(&message) {
            Ok(true) => {
                // ‼️ Updated print message, as we can't print 4096 bytes
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
