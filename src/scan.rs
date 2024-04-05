use std::fs;

use crate::system::config::{get_setting_from_config, ConfigOption};
use crate::OS;

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

pub fn scan(start_dir_path: &String, os: &OS) -> DirectoryObject {
    let ignored_directories_setting = get_setting_from_config(ConfigOption::IgnoredDirectories, os);
    let ignored_files_setting = get_setting_from_config(ConfigOption::IgnoredFiles, os);

    let ignored_directories: Vec<&str> = ignored_directories_setting.value.split(",").collect();
    let ignored_files: Vec<&str> = ignored_files_setting.value.split(",").collect();

    create_tree(start_dir_path, &ignored_directories, &ignored_files, os)
}

fn create_tree(
    start_dir_path: &String,
    ignored_directories: &Vec<&str>,
    ignored_files: &Vec<&str>,
    os: &OS,
) -> DirectoryObject {
    let mut directory_stack: Vec<String> = Vec::new();

    let mut tree = DirectoryObject::new(
        &start_dir_path.split("/").last().unwrap().to_string(),
        start_dir_path,
    );

    let mut start_dir_contents = match fs::read_dir(start_dir_path) {
        Ok(contents) => contents,
        Err(_) => {
            return tree;
        }
    };

    let mut file_count = 0;
    let mut total_size = 0;

    while let Some(entry) = start_dir_contents.next() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry_name = entry_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        if entry_path.is_dir() {
            if ignored_directories.contains(&entry_name.as_str()) {
                continue;
            }

            directory_stack.push(entry_path.to_str().unwrap().to_string());
            continue;
        } else {
            if ignored_files.contains(&entry_name.as_str()) {
                continue;
            }
        }

        file_count += 1;

        let entry_size = match entry.metadata() {
            Ok(metadata) => metadata.len(),
            Err(_) => panic!("could not get metadata"),
        };

        total_size += entry_size;

        let entry_ending = match entry_path.extension() {
            Some(ending) => ending.to_str().unwrap().to_string(),
            None => String::new(),
        };

        let file = FileObject::new(
            &entry_name,
            &entry_name.replace(&format!(".{}", entry_ending).to_string(), ""),
            entry_ending,
            entry_path.to_str().unwrap().to_string(),
            entry_size,
        );

        tree.add_file(file);
    }

    tree.file_count = file_count;
    tree.size = total_size;
    tree.size_string = bytes_to_best_size(total_size);

    while !directory_stack.is_empty() {
        let current_dir_path = directory_stack.pop().unwrap();

        let sub_dir_tree = create_tree(&current_dir_path, ignored_directories, ignored_files, &os);

        tree.add_directory(sub_dir_tree);
    }

    tree
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
