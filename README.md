# Sik

`sik` is a simple, fast, and concurrent command-line tool for searching patterns within files in a given directory. It is written in Rust and leverages multi-threading to perform searches efficiently.

## Features

- **Concurrent Searching:** Utilizes multiple threads to search files in parallel, making it fast on modern multi-core processors.
- **Simple and Intuitive:** Easy to use with a minimal set of command-line arguments.
- **Cross-Platform:** Built with Rust, it can be compiled and run on Windows, macOS, and Linux.

## Dependencies

This project uses the following external crate:

-   [`regex`](https://crates.io/crates/regex): For regular expression based pattern matching.

## Prerequisites

To build and run `sik`, you need to have the Rust programming language toolchain installed. You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

## Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/mi66mc/sik.git
    cd sik
    ```

2.  **Build the project:**
    For a development build, run:
    ```bash
    cargo build
    ```
    For a release build (recommended for performance), run:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/sik`.

## Usage

The basic syntax for `sik` is:

```
sik [OPTIONS] <PATTERN> [PATH]
```

### Arguments

-   `<PATTERN>`: The pattern to search for within the files, Regex is used to build the pattern. If the pattern contains spaces, it should be enclosed in quotes.
-   `[PATH]`: The directory to search in. If not provided, it defaults to the current directory (`.`).

### Options

-	`--primary`, `--secondary`, `--tertiary`: Arguments you provide to choose what type of style you want to be displayed.
-   `-t, --threads <NUM>`: Sets the number of threads to use for searching. By default, it uses twice the number of available logical processors.
-   `-h, --help`: Prints the help message and exits.

### Examples

-   **Search for a pattern in the current directory:**
    ```bash
    sik "my_pattern"
    ```

-   **Search for a pattern in a specific directory:**
    ```bash
    sik "my pattern with spaces" "/path/to/your/directory"
    ```

-   **Search using a specific number of threads:**
    ```bash
    sik --secondary --threads 8 "error" "./logs"
    ```

## License

This project is licensed under the **GNU Affero General Public License v3.0**. See the [LICENSE](LICENSE) file for more details.
