use std::fs::File;
use std::io::prelude::*;

fn create_and_write_file(file_path: &str, content: &str) -> std::io::Result<()> {
    // Create a new file at the specified path
    let mut file = File::create(file_path)?;

    // Write the content to the file
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() {
    let file_path = "test.txt";
    let content = "Hello, World!";

    if let Err(err) = create_and_write_file(file_path, content) {
        eprintln!("Error: {}", err);
    } else {
        println!("File created and written successfully.");
    }
}
