use crate::scan::DirectoryObject;
use crate::OS;

pub fn display_tree(
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
