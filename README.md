# Fmap

Fmap is a command-line tool written in Rust for displaying a tree-like view of files and directories. It provides functionality to scan directories, create a configuration file, manage ignored directories and files, and more.

## Installation

### Build from Source

Clone the repository and build the project using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
git clone https://github.com/cqb13/fmap.git
cd bits
cargo build --release
# The binary will be located at target/release/bits
```

To add the binary to your PATH, run:

```sh
cargo install --path .
```

### Pre-built Binaries

Pre-built binaries are available for Windows, macOS, and Linux on the [releases page](https://github.com/cqb13/ti-tools/releases).

## Usage

```sh

USAGE:
    fmap [COMMAND] [OPTIONS]

COMMANDS:
    help - Prints help information
        command       <COMMAND>                     (optional) The command you want help with

    version - Prints version information

    config - Creates a new config file

    install - Installs fmap

    add - Add a file or directory to the ignore list
        type          <TYPE>                        (required) file (file) or directory (dir)
        name          <NAME>                        (required) The name of the file/file ending or directory

    remove - Remove a file or directory to the ignore list
        type          <TYPE>                        (required) file (file) or directory (dir)
        name          <NAME>                        (required) The name of the file/file ending or directory

    list - List all files or directories in the ignore list
        type          <TYPE>                        (required) file (file) or directory (dir)

    scan - Scan a directory
        path          <PATH>                        (required) path to a directory
        -x            --                                       Disable file extentions
        -f            --                                       Show file sizes
        -d            --                                       Show directory sizes
        -c            --                                       Show the file count in a directory
```

## Contributing

Contributions are welcome! Feel free to fork this repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
