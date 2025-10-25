use proclink::ShmemReader;
use std::str;

// ‼️ The main logic is moved to src/lib.rs.
// This main function just sets up the reader and calls read.
fn main() {
    let reader = ShmemReader::new("my_synchronized_shmem")
        .expect("Failed to open shared memory. Is the writer running?");

    println!("[Reader] Attached to shared memory.");

    // ‼️ Call the library's read function
    match reader.read() {
        Ok(Some(data)) => {
            // ‼️ Convert the received bytes to a string
            match str::from_utf8(&data) {
                Ok(message) => {
                    println!("[Reader] ✅ Read new data: \"{}\"", message);
                }
                Err(_) => {
                    println!("[Reader] ✅ Read new data (raw bytes): {:?}", data);
                }
            }
        }
        Ok(None) => {
            println!("[Reader] ⚠️ No new data to read.");
        }
        Err(e) => {
            eprintln!("[Reader] ❌ Error reading: {}", e);
        }
    }
}
