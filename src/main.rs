use proclink::ShmemWriter;
use std::process;

fn main() {
    const PAYLOAD_SIZE: usize = 4096;

    let writer = match ShmemWriter::new("my_app_shmem", PAYLOAD_SIZE) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("[Main] ❌ Failed to create writer: {}", e);
            process::exit(1);
        }
    };

    println!("[Main] Writer created. Writing in a loop...");

    loop {
        match writer.write(b"Data from my new project!") {
            Ok(true) => println!("[Main] My new project wrote successfully!"),
            Ok(false) => println!("[Main] My new project buffer full, reader hasn't read yet."),
            Err(err) => {
                println!("[Main] My new project failed to write: {}", err);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // ...
    // let reader = match ShmemReader::new("my_app_shmem") {
    //     Ok(r) => r,
    //     Err(e) => {
    //         eprintln!("[Main] ❌ Failed to create reader: {}", e);
    //         process::exit(1);
    //     }
    // };
    //
    // match reader.read() {
    //     Ok(Some(data)) => {
    //         // Try to convert the bytes to a valid UTF-8 string
    //         match std::str::from_utf8(&data) {
    //             Ok(message) => {
    //                 // Print the string content using "{}"
    //                 println!("[Main] My new project read: \"{}\"", message);
    //             }
    //             Err(_) => {
    //                 // Fallback in case the data is not valid UTF-8
    //                 println!("[Main] My new project read (non-UTF-8): {:?}", data);
    //             }
    //         }
    //     }
    //     Ok(None) => {
    //         println!("[Main] No new data to read.");
    //     }
    //     Err(e) => {
    //         println!("[Main] Failed to read: {}", e);
    //     }
    // }
}
