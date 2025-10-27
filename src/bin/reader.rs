use proclink::{ShmemLinkError, ShmemReader};
use std::process;

fn main() {
    let reader = match ShmemReader::new("my_synchronized_shmem") {
        Ok(r) => {
            println!("[Reader] Attached to shared memory.");
            r
        }
        Err(e) => {
            match e {
                ShmemLinkError::Shmem(shmem_err) => {
                    eprintln!(
                        "[Reader] ❌ Failed to open shared memory: {}. Is the writer running?",
                        shmem_err
                    );
                }
                ShmemLinkError::TooSmall { found, required } => {
                    eprintln!(
                        "[Reader] ❌ Shared memory segment is too small. Found {}, need at least {}.",
                        found, required
                    );
                }
                // Other arms aren't expected from ShmemReader::new, but we can be exhaustive
                _ => {
                    eprintln!(
                        "[Reader] ❌ An unexpected error occurred on creation: {}",
                        e
                    );
                }
            }
            process::exit(1);
        }
    };

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
