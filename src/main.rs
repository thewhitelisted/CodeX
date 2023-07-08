use std::fs::File;
use std::io::prelude::*;

fn get_string() -> String {
    let mut value = String::new();
    std::io::stdin()
        .read_line(&mut value)
        .expect("failed to read from std::io::stdin");
    value.trim().to_string()
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
        println!("Code File Creator\n");
        println!("Commands: c, python, java\n");
        let option = String::from(get_string());

        match &option[..] {
            "c" => create_c(),
            "python" => create_python(),
            "java" => create_java(),
            "quit()" => std::process::exit(0),
            _ => clearscreen::clear().expect("failed to clear screen"),
        }
    }
}
