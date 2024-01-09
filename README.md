# Fmap - File and Directory Mapping Tool

Fmap is a command-line tool written in Rust for displaying a tree-like view of files and directories. It provides functionality to scan directories, create a configuration file, manage ignored directories and files, and more.

## Features

- Display a tree-like view of files and directories.
- Easily customize the scan path.
- Create and manage a configuration file for persistent settings.
- Add, remove, and list ignored directories and files.
- Cross-platform support for Windows and macOS.

## Commands

The following commands are available for use with Fmap:

- **Basic Usage:**

  ```bash
  fmap              # Scans from the current directory
  ```

- **Commands:**

  ```bash
  # Scan Commands
  fmap                    # Scans from the current directory
  fmap -s                # Scans with default options
  fmap -s -e -fs -ds -fc  # Scans with optional features: show file endings, file sizes, directory sizes, and file counts in directories
  fmap -sp "path"        # Scans from a custom relative path

  # Configuration Commands
  fmap -c                # Creates/reset the configuration file

  # Ignored Directories and Files Commands
  fmap -add -dir "directory"  # Adds a directory to the ignored list
  fmap -add -file "filename"  # Adds a file to the ignored list
  fmap -rmv -dir "directory"  # Removes a directory from the ignored list
  fmap -rmv -file "filename"  # Removes a file from the ignored list
  fmap -ls -dir             # Lists all ignored directories
  fmap -ls -file            # Lists all ignored files

  # Other Commands
  fmap -i                # Adds the binary to the PATH environment variable
  fmap -v                # Prints the version
  fmap -h                # Prints the help
  ```

**Note:** Replace "path," "directory," and "filename" with your desired paths, directories, and filenames. You can customize the scan options with flags such as `-e` for file endings, `-fs` for file sizes, `-ds` for directory sizes, and `-fc` for file counts in directories.

## Example Output

### Command

```bash
fmap scan -e -fc
```

### Output

```bash
fmap
├── Cargo.toml
├── LICENSE
├── Cargo.lock
├── README.md
├── .gitignore
└── src (6)
    ├── display.rs
    ├── config.rs
    ├── commands.rs
    ├── main.rs
    ├── scan.rs
    └── utils.rs
```

## Website

You can try out Fmap online at [https://foldermap.cqb13.dev](https://foldermap.cqb13.dev).

## Installation

### macOS

1. Download the latest macOS release from [GitHub](https://github.com/cqb13/fmap/releases) or build from source
2. Extract the zip file
3. Open the folder with the extracted file in your terminal
4. Run the following command:

   ```bash
   ./fmap -i
   ```

5. restart your terminal and run the following command:

   ```bash
   fmap -v
   ```

6. If the version is printed, you have successfully installed Fmap, if not, please submit an issue on [GitHub](https://github.com/cqb13/fmap/issues), with the error message.

### Windows

1. Download the latest windows release from [GitHub](https://github.com/cqb13/fmap/releases) or build from source
2. Extract the zip file
3. Open the folder with the extracted file in your terminal (you must use administrator privileges)
4. Run the following command:

   ```bash
   ./fmap.exe -i
   ```

5. restart your computer
6. Open your terminal and run the following command:

   ```bash
   fmap -v
   ```

7. If the version is printed, you have successfully installed Fmap, if not, please submit an issue on [GitHub](https://github.com/cqb13/fmap/issues), with the error message.

### Alternative Installation

1. Clone the repository: `git clone https://github.com/cqb13/fmap.git`
2. Navigate to the project directory: `cd fmap`
3. Build and install with Cargo: `cargo install --path .`

## Contributing

Feel free to contribute to the project by submitting issues or pull requests on [GitHub](https://github.com/cqb13/fmap).

## License

This project is licensed under the [MIT License](LICENSE).
