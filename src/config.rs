use crate::OS;
use std::fs::File;
use std::io::prelude::*;

pub const WINDOWS_CONFIG_FILE: &str = ".fmap_config";

pub fn create_config_file(os: OS) {
    match os {
        OS::Windows => create_config_file_for_windows(),
        OS::Mac => println!("Creating config file for Mac"),
        OS::Other => panic!("Unsupported OS"),
    }
}

fn create_config_file_for_windows() {
    let home_dir = std::env::var("USERPROFILE").unwrap();
    let config_file_path = format!("{}/{}", home_dir, WINDOWS_CONFIG_FILE);
    let config_file = File::create(config_file_path).unwrap();
    let result = write_default_config_to_file(config_file);
    match result {
        Ok(()) => (),
        Err(e) => panic!("Error creating config file: {}", e),
    }
}

fn write_default_config_to_file(mut file: File) -> std::io::Result<()> {
    /*
     * format:
     * - data in sets of two by line
     *  - line 1: key
     *  - line 2: value
     */

    let default_ignored_directories = vec!["node_modules", "target", "dist", "venv", ".git"];
    // write test in the file
    file.write_all(b"ignored-directories\n")?;
    for directory in default_ignored_directories {
        file.write_all(format!("{},", directory).as_bytes()).unwrap();
    }

    Ok(())
}

