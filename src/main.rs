use std::fs::File;
use std::io::prelude::*;

fn get_string() -> String {
    let mut value = String::new();
    std::io::stdin()
        .read_line(&mut value)
        .expect("failed to read from std::io::stdin");
    value.trim().to_string()
}

fn pause() {
    println!("\nPress ENTER to continue...");
    let _ = get_string();
}

#[allow(dead_code)]
fn create_and_write_file(file_path: String, content: String) -> std::io::Result<()> {
    let mut file = File::create(file_path.to_string())?;

    file.write_all(content.to_string().as_bytes())?;

    Ok(())
}

fn remove_slash(input: &mut String) {
    if input.ends_with('/') {
        input.pop();
    }
}

fn get_path() -> String {
    println!("\nENTER THE FOLDER PATH: ");
    let mut path = get_string();
    remove_slash(&mut path);
    path
}

fn get_full_filepath(path: String, file_ext: &str) -> String {
    println!("\nENTER THE FILE NAME WITHOUT FILE EXTENSION: ");
    let file_name = get_string();
    let full_path = path + "/" + &file_name + file_ext;
    full_path
}

fn create_c() {
    clearscreen::clear().expect("failed to clear screen");
    println!("C FILE CREATOR");

    let path = get_path();

    let full_path = get_full_filepath(path, ".c");

    if let Err(err) = create_and_write_file(
        full_path,
        "#include <stdio.h>\n\nint main()\n{\n\t\n\treturn 0;\n}".to_string(),
    ) {
        eprint!("\nError: {}", err);
        pause();
    } else {
        print!("\nFile created and written successfully.");
        pause();
    }
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
