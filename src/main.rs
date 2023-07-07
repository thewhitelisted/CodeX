use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(file_path: &str) -> io::Result<String> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let file_path = "test.txt";
    match read_file_contents(file_path) {
        Ok(contents) => {
            println!("File contents:\n{}", contents);
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }
}
