use std::fs::File;
use std::io::prelude::*;

fn get_string(destination: &mut &str) -> () {
    std::io::stdin()
        .read_line(&mut destination.to_string())
        .expect("failed to read from std::io::stdin");
    *destination = destination.trim();
}

#[allow(dead_code)]
fn create_and_write_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn create_c() {
    todo!();
}

fn create_python() {
    todo!()
}

fn create_java() {
    todo!();
}

fn main() {
    /*
    if let Err(err) = create_and_write_file(file_path, content) {
        eprintln!("Error: {}", err);
    } else {
        println!("File created and written successfully.");
    }
    */

    loop {
        clearscreen::clear().expect("failed to clear screen");
        let mut option: &str = &String::new();
        println!("Code File Creator\n");
        println!("Commands: c, python, java\n");
        get_string(&mut option);

        match option {
            "c" => create_c(),
            "python" => create_python(),
            "java" => create_java(),
            _ => clearscreen::clear().expect("failed to clear screen"),
        }
    }
}
