use proclink::{ShmemReader, ShmemWriter};

fn main() {
    const PAYLOAD_SIZE: usize = 4096;
    // You can now use the clean API
    let writer = ShmemWriter::new("my_app_shmem", PAYLOAD_SIZE).unwrap();
    loop {
        match writer.write(b"Data from my new project!") {
            Ok(true) => println!("My new project wrote successfully!"),
            Ok(false) => println!("My new project failed to write!"),
            Err(err) => println!("My new project failed to write: {}", err),
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // ...

    // let reader = ShmemReader::new("my_app_shmem").unwrap();
    // if let Ok(Some(data)) = reader.read() {
    //     // ‼️ Try to convert the bytes to a valid UTF-8 string
    //     match std::str::from_utf8(&data) {
    //         Ok(message) => {
    //             // ‼️ Print the string content using "{}"
    //             println!("My new project read: \"{}\"", message);
    //         }
    //         Err(_) => {
    //             // Fallback in case the data is not valid UTF-8
    //             println!("My new project read (non-UTF-8): {:?}", data);
    //         }
    //     }
    // }
}
