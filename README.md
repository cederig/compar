# Compar

`compar`  is a simple and fast command-line tool written in Rust that compares two text files and identifies lines in the first file that are not present in the second. It offers options to customize the comparison, such as the ability to compare only a defined number of characters at the beginning of each line.

## Features

- Compares two text files line by line.
- Identifies and displays lines from the first file that are absent in the second.
- Option to limit the comparison to the first `N` characters of each line.
- Progress bar to track processing progress.
- Handles different file encodings (UTF-8, UTF-16).
- Debug option for detailed process tracking.

## Dependencies

This project uses the following dependencies (as defined in `Cargo.toml`):

-   `clap` (version `4.5.51`): For command-line argument parsing.
-   `indicatif` (version `0.18.2`): For displaying a progress bar.
-   `encoding_rs` (version `0.8.35`): For text encoding management.
-   `unicode-normalization` (version `0.1.25`): For Unicode string normalization.
-   `memchr` (version `2.7.6`): Indirect dependency for efficient character searching.

## Installation

### Prerequisites

Make sure you have Rust and Cargo installed on your system. You can install them by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compilation for Linux (from Linux/macOS)
1.  Clone this repository:
    ```sh
    git clone https://github.com/cederig/compar.git
    cd compar
    ```
2.  Compile the project:
    ```sh
    cargo build --release
    ```
    The executable will be located in `target/release/compar`.

### Compilation for Windows (from Linux/macOS)

To compile this project for Windows from another operating system (like Linux or macOS), you can use cross-compilation. You will need the Rust target for Windows.

1.  Add the Windows target to your Rust installation:
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2.  Compile the project for the Windows target:
    ```sh
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

The Windows executable will be located in `target/x86_64-pc-windows-gnu/release/compar.exe`.

### Compilation for macOS (from Linux/macOS)

To compile this project for macOS from another operating system (like Linux or macOS), you can use cross-compilation. You will need the Rust target for macOS.

1.  Add the macOS target to your Rust installation (choose the correct architecture):
    *   For Intel Macs (x86_64):
        ```sh
        rustup target add x86_64-apple-darwin
        ```
    *   For Apple Silicon Macs (aarch64):
        ```sh
        rustup target add aarch64-apple-darwin
        ```

2.  Compile the project for the macOS target (choose the correct architecture):
    *   For Intel Macs:
        ```sh
        cargo build --release --target=x86_64-apple-darwin
        ```
    *   For Apple Silicon Macs:
        ```sh
        cargo build --release --target=aarch64-apple-darwin
        ```

The macOS executable will be located in `target/<your_mac_target>/release/compar` (e.g., `target/x86_64-apple-darwin/release/compar`).

## Usage

The basic syntax is as follows:

```bash
compar [OPTIONS] <file1> <file2>
```

### Arguments

-   `<file1>`: The file containing the lines to search for (the "needles").
-   `<file2>`: The file to search within (the "haystack").

### Options

-   `-o, --output <output_file>`: Specifies a file to write the unfound lines to. If this option is not used, the lines will be displayed on standard output.
-   `--debug`: Activates the display of debug information in the terminal.
-   `--length <N>`: Compares only the first `N` characters of each line.
-   `--stat`: Displays detailed comparison statistics at the end of processing.
-   `--found`: Reverses the logic and displays lines from file1 that are found in file2.
-   `-h, --help`: Displays help.
-   `-V, --version`: Displays the tool version.

## Examples

-   Simple comparison:
    ```sh
    ./compar file1.txt file2.txt
    ```

-   Save the result to a file:
    ```sh
    ./compar -o result.txt file1.txt file2.txt
    ```

-   Compare only the first 15 characters:
    ```sh
    ./compar --length 15 file1.txt file2.txt
    ```

-   Use debug mode:
    ```sh
    ./compar --debug file1.txt file2.txt
    ```

## Tests

This project includes unit tests; to run them, use the following command at the project root:

```bash
cargo test
```

This command compiles the program in test mode and executes all test functions.
