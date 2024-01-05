pub mod config;

use crate::config::create_config_file;

#[derive(Debug)]
pub enum OS {
    Windows,
    Mac,
    Other,
}

fn main() {
    let os = match std::env::consts::OS {
        "windows" => OS::Windows,
        "macos" => OS::Mac,
        _ => panic!("Unsupported OS"),
    };
    
    create_config_file(os);
}