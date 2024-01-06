# Fmap - File and Directory Mapping Tool

Fmap is a command-line tool written in Rust for displaying a tree-like view of files and directories. It provides functionality to scan directories, create a configuration file, manage ignored directories and files, and more.

## Features

- Display a tree-like view of files and directories.
- Easily customize the scan path.
- Create and manage a configuration file for persistent settings.
- Add, remove, and list ignored directories and files.
- Cross-platform support for Windows and macOS.

## Usage

### Basic Usage

```bash
fmap              # Scans from the current directory
```

### Options

- `-p "path"`: Specify a custom relative path to scan instead of the current directory.
- `-c`: Create a new config file, overwriting the old one if it exists.
- `-add -dir/-file "directory/file name"`: Add a directory or file to the respective list.
- `-rmv -dir/-file "directory/file name"`: Remove a directory or file from the respective list.
- `-ls -dir/-file`: List all directories or files in the respective list.
- `-v`: Display the version.
- `-h`: Show help.

## Example Output

```bash
fmap
└── LICENSE
└── Cargo.toml
└── Cargo.lock
└── .gitignore
└── src
    └── config.rs
    └── commands.rs
    └── main.rs
```

## Website

You can try out Fmap online at [https://foldermap.cqb13.dev](https://foldermap.cqb13.dev).

## Installation

1. Clone the repository: `git clone https://github.com/cqb13/fmap.git`
2. Navigate to the project directory: `cd fmap`
3. Build and install with Cargo: `cargo install --path .`

## Contributing

Feel free to contribute to the project by submitting issues or pull requests on [GitHub](https://github.com/cqb13/fmap).

## License

This project is licensed under the [MIT License](LICENSE).
