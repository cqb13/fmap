use std::fs;

#[derive(Debug)]
pub struct DirectoryObject {
    pub directories: Vec<DirectoryObject>,
    pub files: Vec<FileObject>,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub size_string: String,
    pub file_count: u64,
}

impl DirectoryObject {
    fn new(name: &String, path: &String) -> DirectoryObject {
        DirectoryObject {
            directories: Vec::new(),
            files: Vec::new(),
            name: name.to_string(),
            path: path.to_string(),
            size: 0,
            size_string: String::new(),
            file_count: 0,
        }
    }

    fn add_directory(&mut self, directory: DirectoryObject) {
        self.directories.push(directory);
    }

    fn add_file(&mut self, file: FileObject) {
        self.files.push(file);
    }
}

#[derive(Debug)]
pub struct FileObject {
    pub full_name: String,
    pub name: String,
    pub ending: String,
    pub path: String,
    pub size: u64,
    pub size_string: String,
}

impl FileObject {
    fn new(
        full_name: &String,
        name: &String,
        ending: String,
        path: String,
        size: u64,
    ) -> FileObject {
        FileObject {
            full_name: full_name.to_string(),
            name: name.to_string(),
            ending,
            path,
            size,
            size_string: bytes_to_best_size(size),
        }
    }
}

fn bytes_to_best_size(bytes: u64) -> String {
    let mut size = bytes as f64;
    let mut unit = "B";

    if size > 1024.0 {
        size /= 1024.0;
        unit = "KB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "MB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "GB";
    }

    if size > 1024.0 {
        size /= 1024.0;
        unit = "TB";
    }

    format!("{:.2} {}", size, unit)
}
