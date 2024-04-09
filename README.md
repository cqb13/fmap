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
  fmap                         # Scans from the current directory with default options
  fmap scan                    # Scans the current directory with options for file endings, sizes, and counts
  fmap scan-path --path "path" # Scans a specific path with options for file endings, sizes, and counts

  # Configuration Commands
  fmap config                 # Creates or resets the configuration file

  # Ignored Directories and Files Commands
  fmap add --type "file/dir" --name "name"  # Adds a file or directory to the ignored list
  fmap remove --type "file/dir" --name "name"  # Removes a file or directory from the ignored list
  fmap list --type "file/dir"               # Lists all ignored files or directories

  # Installation Command
  fmap install                # Adds the binary to the PATH environment variable

  # Version and Help Commands
  fmap version                # Prints the version
  fmap help                   # Prints the help
  ```

**Note:** Replace "path," "directory," and "filename" with your desired paths, directories, and filenames. Customize the scan options with flags such as `--endings`, `--file-sizes`, `--directory-sizes`, and `--file-counts-in-directories` for detailed information.

## Example Output

### Command

```bash
fmap scan --endings --file-counts-in-directories
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

Refer to the "Installation" section in the original markdown for platform-specific installation instructions.

## Contributing

Feel free to contribute to the project by submitting issues or pull requests on [GitHub](https://github.com/cqb13/fmap).

## License

This project is licensed under the [MIT License](LICENSE).
