use std::env;

#[derive(Debug, PartialEq)]
pub enum Object {
    Directory,
    File,
}

pub fn is_valid_name(name: &str, object: Object) -> bool {
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        return false;
    }

    if name.is_empty() {
        return false;
    }

    if name.starts_with(' ') || name.ends_with(' ') {
        return false;
    }

    if object == Object::Directory && name.contains('.') {
        return false;
    }

    if object == Object::File && name.contains(' ') {
        return false;
    }

    true
}

pub fn get_current_directory_path() -> String {
    let current_dir_path = match env::current_dir() {
        Ok(path) => path,
        Err(_) => panic!("Could not get current directory path"),
    };

    current_dir_path.to_str().unwrap().to_string()
}

pub fn exit_with_error(error: &str, show_help: bool) {
    println!("{}", error);
    if show_help {
        println!("use -h for a list of all commands");
    }
    std::process::exit(0);
}
