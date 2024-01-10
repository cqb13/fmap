use crate::system::config::get_user_home_dir;
use crate::utils::get_current_directory_path;
use crate::OS;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn install(os: &OS) {
    println!("starting install on {}", os.get_name());

    let home_dir = get_user_home_dir(os);
    let bin_path = match os {
        OS::Windows => "AppData/Roaming/fmap",
        OS::Mac => ".local/bin",
    };

    let local_bin_path = format!("{}/{}", home_dir, bin_path);

    // Create directory if it doesn't exist
    if !Path::new(&local_bin_path).exists() {
        println!("Creating {} directory", local_bin_path);
        fs::create_dir_all(&local_bin_path).unwrap();
    }

    let binary_name = match os {
        OS::Windows => "fmap.exe",
        OS::Mac => "fmap",
    };

    let new_binary_path = format!("{}/{}", local_bin_path, binary_name);

    if Path::new(&new_binary_path).exists() {
        println!("Replacing binary in {}", &local_bin_path);
        fs::remove_file(&new_binary_path).unwrap();
    }

    println!("Moving binary to {}", local_bin_path);
    fs::copy(
        format!("{}/{}", get_current_directory_path(), binary_name),
        &new_binary_path,
    )
    .unwrap();

    match os {
        OS::Windows => {
            if let Err(e) = add_registry_path(&local_bin_path) {
                eprintln!("Failed to modify system PATH: {}", e);
                eprintln!("This action may require administrator permissions.");
                return;
            }
        }
        OS::Mac => {
            let zprofile_path = format!("{}/.zprofile", home_dir);
            if let Ok(zprofile_content) = fs::read_to_string(&zprofile_path) {
                if !zprofile_content.contains("export PATH=\"$PATH:$HOME/.local/bin\"") {
                    println!("Adding .local/bin to path in .zprofile");
                    let mut zprofile_file = File::create(&zprofile_path).unwrap();
                    writeln!(zprofile_file, "export PATH=\"$PATH:$HOME/.local/bin\"").unwrap();
                }
            }
        }
    }

    println!("install complete");
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
