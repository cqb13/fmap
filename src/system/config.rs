use crate::utils::{exit_with_error, is_valid_name, Object};
use crate::OS;
use std::fs::{read_to_string, File};
use std::io::prelude::*;

pub const CONFIG_FILE: &str = ".fmap_config";
const DEFAULT_IGNORED_DIRECTORIES: [&str; 12] = [
    ".gradle",
    ".idea",
    "node_modules",
    "target",
    "build",
    "dist",
    "venv",
    ".git",
    ".vscode",
    ".settings",
    ".next",
    ".bin",
];
const DEFAULT_IGNORED_FILES: [&str; 0] = [];

pub enum ConfigOption {
    IgnoredDirectories,
    IgnoredFiles,
}

impl ConfigOption {
    pub fn get_config_option_key(&self) -> String {
        match self {
            Self::IgnoredDirectories => "ignored-directories".to_string(),
            Self::IgnoredFiles => "ignored-files".to_string(),
        }
    }
}

pub struct Setting {
    pub key: String,
    pub key_line_number: usize,
    pub value: String,
}

impl Setting {
    pub fn new(key: String, key_line_number: usize, value: String) -> Self {
        Self {
            key,
            key_line_number,
            value,
        }
    }
}

pub fn create_config_file(os: &OS, force: bool) {
    let config_file_path = get_config_path(os);
    if path_exists(&config_file_path) && !force {
        return;
    }
    let config_file = File::create(config_file_path).unwrap();
    let result = write_default_config_to_file(config_file);
    match result {
        Ok(()) => (),
        Err(e) => panic!("Error creating config file: {}", e),
    }
}

fn write_default_config_to_file(mut config_file: File) -> std::io::Result<()> {
    /*
     * format:
     * - data in sets of two by line
     *  - line 1: key
     *  - line 2: value
     */

    config_file.write_all(b"ignored-directories\n")?;
    let mut value_line = String::new();
    for directory in DEFAULT_IGNORED_DIRECTORIES {
        value_line.push_str(format!("{},", directory).as_str());
    }
    value_line.pop();
    config_file.write_all(value_line.as_bytes())?;

    value_line.clear();

    config_file.write_all(b"\nignored-files\n")?;
    for file in DEFAULT_IGNORED_FILES {
        value_line.push_str(format!("{},", file).as_str());
    }
    value_line.pop();
    config_file.write_all(value_line.as_bytes())?;

    Ok(())
}

pub fn get_setting_from_config(config_option: ConfigOption, os: &OS) -> Setting {
    let config_file_path = get_config_path(os);
    if !path_exists(&config_file_path) {
        exit_with_error(
            "failed to find config, use the -c arg to make a new one",
            true,
        )
    }
    let key = config_option.get_config_option_key();

    for (index, line) in read_to_string(&config_file_path)
        .unwrap()
        .lines()
        .enumerate()
    {
        if index % 2 != 1 && line == key {
            let config_content = read_to_string(&config_file_path).unwrap();
            let value_line = config_content.lines().nth(index + 1).unwrap_or("None");
            return Setting::new(key, index, value_line.to_string());
        }
    }

    exit_with_error("failed to find config option", true);
    // will never reach this
    Setting::new("".to_string(), 0, "".to_string())
}

pub fn remove_value_from_setting(config_option: ConfigOption, value_setting: &str, os: &OS) {
    let object_type = match config_option {
        ConfigOption::IgnoredDirectories => Object::Directory,
        ConfigOption::IgnoredFiles => Object::File,
    };

    if !is_valid_name(&value_setting, object_type) {
        exit_with_error(
            format!("{} is not a valid name", value_setting).as_str(),
            false,
        );
    }

    let setting = get_setting_from_config(config_option, os);

    let mut value = setting.value;

    let mut modified_value = String::new();
    value.split(",").for_each(|v| {
        if v != value_setting {
            modified_value.push_str(format!("{},", v).as_str());
        }
    });

    value = modified_value.trim_end_matches(",").to_string();

    update_config_setting_value(value, setting.key_line_number, os);
    println!("removed {} from {}", value_setting, setting.key);
}

pub fn add_value_to_setting(config_option: ConfigOption, value_setting: &str, os: &OS) {
    let object_type = match config_option {
        ConfigOption::IgnoredDirectories => Object::Directory,
        ConfigOption::IgnoredFiles => Object::File,
    };

    if !is_valid_name(&value_setting, object_type) {
        exit_with_error(
            format!("{} is not a valid name", value_setting).as_str(),
            false,
        );
    }

    let setting = get_setting_from_config(config_option, os);

    let mut value = setting.value;

    // using ,{value}, to ensure that value is not part of another value
    if value.contains(format! {",{},", value_setting}.as_str()) {
        return;
    }

    value.push_str(format!(",{}", value_setting).as_str());

    update_config_setting_value(value, setting.key_line_number, os);
    println!("added {} to {}", value_setting, setting.key);
}

fn update_config_setting_value(value: String, key_line_number: usize, os: &OS) {
    let config_file_path = get_config_path(os);
    let mut config_content = read_to_string(&config_file_path).unwrap();
    let mut lines = config_content.lines().collect::<Vec<&str>>();
    lines[key_line_number + 1] = value.as_str();
    config_content = lines.join("\n");
    let mut config_file = File::create(config_file_path).unwrap();
    config_file.write_all(config_content.as_bytes()).unwrap();
}

fn get_config_path(os: &OS) -> String {
    let home_dir = get_user_home_dir(os);
    format!("{}/{}", home_dir, CONFIG_FILE)
}

pub fn get_user_home_dir(os: &OS) -> String {
    let user_dir = match os {
        OS::Windows => "USERPROFILE",
        OS::Mac => "HOME",
    };
    std::env::var(format!("{}", user_dir)).unwrap()
}

fn path_exists(path: &String) -> bool {
    std::path::Path::new(&path).exists()
}
