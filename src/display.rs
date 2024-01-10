use crate::scan::DirectoryObject;
use crate::utils::get_current_directory_path;
use crate::OS;
use std::fs;
use std::path::PathBuf;

pub fn display(
    tree: &DirectoryObject,
    show_endings: &bool,
    show_file_sizes: &bool,
    show_directory_sizes: &bool,
    show_file_counts_in_directories: &bool,
    os: &OS,
) {
    let display_name = if tree.name.is_empty() {
        // Handle relative paths
        let current_dir = PathBuf::from(get_current_directory_path());
        let scanned_dir =
            fs::canonicalize(current_dir.join(&tree.path)).expect("Unable to canonicalize path");

        scanned_dir
            .file_name()
            .and_then(|name| name.to_str())
            .map_or_else(|| "".to_string(), |s| s.to_string())
    } else {
        tree.name.clone()
    };

    println!("{}", display_name);

    display_tree(
        &tree,
        0,
        "",
        show_endings,
        show_file_sizes,
        show_directory_sizes,
        show_file_counts_in_directories,
        os,
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

        let is_last_file = i == tree.files.len() - 1 && tree.directories.is_empty();
        println!(
            "{}{}── {}",
            indent,
            if is_last_file { '└' } else { '├' },
            file_display
        );
    }

    for (i, directory) in tree.directories.iter().enumerate() {
        let directory_name = match os {
            OS::Windows => directory
                .name
                .split("\\")
                .last()
                .unwrap_or_default()
                .to_string(),
            OS::Mac => directory.name.clone(),
        };

        let directory_display = format!(
            "{} {} {}",
            directory_name,
            if *show_file_counts_in_directories && directory.file_count > 0 {
                format!("({})", directory.file_count)
            } else {
                "".to_string()
            },
            if *show_directory_sizes {
                format!("({})", directory.size_string)
            } else {
                "".to_string()
            }
        );

        let is_last_directory = i == tree.directories.len() - 1;
        let connector = if is_last_directory { '└' } else { '├' };

        println!("{}{}── {}", indent, connector, directory_display);
        let child_indent = format!(
            "{}{}",
            indent,
            if is_last_directory { "    " } else { "│   " }
        );
        display_tree(
            &directory,
            depth + 1,
            &child_indent,
            show_endings,
            show_file_sizes,
            show_directory_sizes,
            show_file_counts_in_directories,
            os,
        );
    }
}
