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
