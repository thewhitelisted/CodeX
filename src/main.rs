use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

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

fn create_file(file_path: String) -> Result<File, Error> {
    Ok(File::create(file_path.to_string())?)
}

fn overwrite_file(file_path: String, content: String) -> std::io::Result<()> {
    let mut file = create_file(file_path)?;
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

fn get_filename() -> String {
    println!("\nENTER THE FILE NAME WITHOUT FILE EXTENSION: ");
    let file_name = get_string();
    file_name
}

fn append_filepath(path: String, file: String, file_ext: &str) -> String {
    let full_path = path + "/" + &file + file_ext;
    full_path
}

fn create_c() {
    clearscreen::clear().expect("failed to clear screen");
    println!("C FILE CREATOR");

    let path = get_path();
    let file_name = get_filename();
    let full_path = append_filepath(path, file_name, ".c");

    if let Err(err) = overwrite_file(
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
    clearscreen::clear().expect("failed to clear screen");
    println!("PYTHON FILE CREATOR");

    let path = get_path();
    let file_name = get_filename();
    let full_path = append_filepath(path, file_name, ".py");

    if let Err(err) = overwrite_file(
        full_path,
        "def main(): -> None\n\tpass\n\nif __name__ == '__main__':\n\tmain()".to_string(),
    ) {
        eprint!("\nError: {}", err);
        pause();
    } else {
        print!("\nFile created and written successfully.");
        pause();
    }
}

fn create_java() {
    clearscreen::clear().expect("failed to clear screen");
    println!("JAVA FILE CREATOR");

    let path = get_path();
    let file_name = get_filename();
    let full_path = append_filepath(path, file_name.clone(), ".java");

    if let Err(err) = overwrite_file(
        full_path,
        "public class ".to_owned() + &file_name + " {\n\t\tpublic static void main(String[] args){\n\tSystem.out.println(\"Hello world\");\n\t}\n}",
    ) {
        eprint!("\nError: {}", err);
        pause();
    } else {
        print!("\nFile created and written successfully.");
        pause();
    }
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
