use crate::scan::DirectoryObject;
use crate::OS;
use std::fs;

pub fn display(
    tree: &DirectoryObject,
    show_endings: &bool,
    show_file_sizes: &bool,
    show_directory_sizes: &bool,
    show_file_counts_in_directories: &bool,
    os: &OS,
) {
    if tree.name == "" {
        // means the name is "../", or some relative path, and not the curent directory, so we need to get the name of that directory
        let current_dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let scanned_dir = fs::canonicalize(format!("{}/{}", current_dir, tree.path)).unwrap();

        let scanned_dir_name = scanned_dir
            .to_str()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .to_string();

        println!("{}", scanned_dir_name);
    } else {
        println!("{}", tree.name);
    }

    display_tree(
        &tree,
        0,
        "",
        &show_endings,
        &show_file_sizes,
        &show_directory_sizes,
        &show_file_counts_in_directories,
        &os,
    );
}

fn display_tree(
    tree: &DirectoryObject,
    depth: i32,
    indent: &str,
    show_endings: &bool,
    show_file_sizes: &bool,
    show_directory_sizes: &bool,
    show_file_counts_in_directories: &bool,
    os: &OS,
) {
    for (i, file) in tree.files.iter().enumerate() {
        let file_display = format!(
            "{} {}",
            if *show_endings {
                &file.full_name
            } else {
                &file.name
            },
            if *show_file_sizes {
                format!("({})", &file.size_string)
            } else {
                "".to_string()
            }
        );

        if i == tree.files.len() - 1 && tree.directories.len() == 0 {
            println!("{}└── {}", indent, file_display);
        } else {
            println!("{}├── {}", indent, file_display);
        }
    }

    for (i, directory) in tree.directories.iter().enumerate() {
        // paths on windows will be the full path, but we just want the directory name
        let directory_name = match os {
            OS::Windows => directory.name.split("\\").last().unwrap().to_string(),
            OS::Mac => directory.name.to_string(),
        };

        let directory_display = format!(
            "{} {} {}",
            directory_name,
            if *show_file_counts_in_directories {
                let file_count = directory.file_count;
                if file_count > 0 {
                    format!("({})", file_count)
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            },
            if *show_directory_sizes {
                format!("({})", directory.size_string)
            } else {
                "".to_string()
            }
        );

        if i == tree.directories.len() - 1 {
            println!("{}└── {}", indent, directory_display);
            display_tree(
                &directory,
                depth + 1,
                &format!("{}    ", indent),
                show_endings,
                show_file_sizes,
                show_directory_sizes,
                show_file_counts_in_directories,
                os,
            );
        } else {
            println!("{}├── {}", indent, directory_display);
            display_tree(
                &directory,
                depth + 1,
                &format!("{}│   ", indent),
                show_endings,
                show_file_sizes,
                show_directory_sizes,
                show_file_counts_in_directories,
                os,
            );
        }
    }
}
