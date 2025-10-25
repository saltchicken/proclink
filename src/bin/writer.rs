use proclink::ShmemWriter;
use std::thread;
use std::time::Duration;

// This main function just sets up the writer and runs the loop.
fn main() {
    let writer =
        ShmemWriter::new("my_synchronized_shmem").expect("Failed to open or create shared memory");

    println!("[Writer] Attached to shared memory. Now writing in a loop. Press Ctrl+C to exit.");

    let mut counter = 0;
    loop {
        // ‼️ Create a dynamic message
        let message_str = format!("Hello from the writer! Count: {}", counter);
        let message = message_str.as_bytes();

        // ‼️ Call the library's write function
        match writer.write(message) {
            Ok(true) => {
                println!("[Writer] ✅ Wrote new data: \"{}\"", message_str);
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
