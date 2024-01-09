use crate::system::config::get_user_home_dir;
use crate::utils::get_current_directory_path;
use crate::OS;
use std::io::Write;

pub fn install(os: &OS) {
    println!("starting install on {}", os.get_name());

    let home_dir = get_user_home_dir(os);

    match os {
        OS::Windows => {
            let app_data_path = format!("{}/AppData/Roaming/fmap", home_dir);
            if !std::path::Path::new(&app_data_path).exists() {
                println!("Creating AppData/Roaming/fmap directory");
                std::fs::create_dir_all(&app_data_path).unwrap();
            }

            let new_binary_path = format!("{}/fmap.exe", app_data_path);
            if !std::path::Path::new(&new_binary_path).exists() {
                println!("Moving binary to AppData/Roaming/fmap");
                std::fs::copy(
                    format!("{}/fmap.exe", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            } else {
                println!("Replacing binary in AppData/Roaming/fmap");
                std::fs::remove_file(&new_binary_path).unwrap();
                std::fs::copy(
                    format!("{}/fmap.exe", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            }

            if let Err(e) = add_registry_path(&app_data_path) {
                eprintln!("Failed to modify system PATH: {}", e);
                eprintln!("This action may require administrator permissions.");
                return;
            }
        }
        OS::Mac => {
            let local_bin_path = format!("{}/.local/bin", home_dir);
            if !std::path::Path::new(&local_bin_path).exists() {
                println!("creating .local/bin directory");
                std::fs::create_dir_all(&local_bin_path).unwrap();
            }

            let new_binary_path = format!("{}/fmap", local_bin_path);
            if !std::path::Path::new(&new_binary_path).exists() {
                println!("moving binary to .local/bin");
                std::fs::copy(
                    format!("{}/fmap", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            } else {
                println!("replacing binary in .local/bin");
                std::fs::remove_file(&new_binary_path).unwrap();
                std::fs::copy(
                    format!("{}/fmap", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            }
            let zprofile_path = format!("{}/.zprofile", home_dir);
            let zprofile_content = std::fs::read_to_string(&zprofile_path).unwrap();
            if !zprofile_content.contains("export PATH=\"$PATH:$HOME/.local/bin\"") {
                println!("adding .local/bin to path");
                let mut zprofile_file = std::fs::OpenOptions::new()
                    .append(true)
                    .open(&zprofile_content)
                    .unwrap();
                zprofile_file
                    .write_all(b"export PATH=\"$PATH:$HOME/.local/bin\"\n")
                    .unwrap();
            }
        }
    }

    println!("install complete");
}

pub fn uninstall(os: &OS) {
    println!("starting uninstall on {}", os.get_name());

    let home_dir = get_user_home_dir(os);

    match os {
        OS::Windows => {
            println!("uninstalling on windows");
        }
        OS::Mac => {
            let local_bin_path = format!("{}/.local/bin", home_dir);
            if std::path::Path::new(&local_bin_path).exists() {
                println!("removing binary from .local/bin");
                std::fs::remove_file(&local_bin_path).unwrap();
            }

            // path is not removed as their may be other binaries later added into the directory
        }
    }

    println!("uninstall complete");
}

fn add_registry_path(new_path: &str) -> std::io::Result<()> {
    use std::process::Command;

    // Escape percent signs by doubling them
    let escaped_path = new_path.replace("%", "%%");

    // Prepare the command to modify the registry
    let status = Command::new("reg")
        .args(&[
            "ADD",
            "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
            "/v",
            "Path",
            "/t",
            "REG_EXPAND_SZ",
            "/d",
            &escaped_path,
            "/f",
        ])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to modify registry",
        ));
    }

    Ok(())
}
