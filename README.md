# Password Generator
This project is a command-line password generator written in Rust. It generates random passwords of a specified length using a customizable set of characters. The length can be provided as a command-line argument, and if an invalid length (0 or less) is given, the program will return an error message.

## Requirements
Rust (ensure you have Rust and Cargo installed on your machine).
clap crate for command-line argument parsing.
rand crate for random number generation.

## Installation
Clone the repository:

```bash
git clone https://github.com/yourusername/password_generator.git
cd password_generator
```

Build the project:

```bash
cargo build
```

Run the program:

```bash
cargo run -- -l 16
```