use crate::scan::DirectoryObject;

pub fn display_tree(tree: &DirectoryObject, depth: i32, indent: &str) {
    for (i, file) in tree.files.iter().enumerate() {
        if i == tree.files.len() - 1 && tree.directories.len() == 0 {
            println!("{}└── {}", indent, file.full_name);
        } else {
            println!("{}├── {}", indent, file.full_name);
        }
    }

    for (i, directory) in tree.directories.iter().enumerate() {
        if i == tree.directories.len() - 1 {
            println!("{}└── {}", indent, directory.name);
            display_tree(&directory, depth + 1, &format!("{}    ", indent));
        } else {
            println!("{}├── {}", indent, directory.name);
            display_tree(&directory, depth + 1, &format!("{}│   ", indent));
        }
    }
}
