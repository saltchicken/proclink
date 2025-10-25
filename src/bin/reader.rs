use proclink::ShmemReader;

fn main() {
    let reader = ShmemReader::new("my_synchronized_shmem")
        .expect("Failed to open shared memory. Is the writer running?");
    println!("[Reader] Attached to shared memory.");
    match reader.read() {
        Ok(Some(data)) => {
            println!("[Reader] ✅ Read new data. Size: {} bytes.", data.len());

            // Optional: Read the counter from the first 8 bytes
            if data.len() >= 8 {
                match data[0..8].try_into() {
                    Ok(counter_bytes) => {
                        let counter = u64::from_le_bytes(counter_bytes);
                        println!("[Reader] 🔍 Payload starts with counter: {}", counter);
                    }
                    Err(_) => {
                        println!("[Reader] ⚠️ Could not read counter from payload.");
                    }
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
