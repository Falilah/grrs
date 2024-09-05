## Rust Pattern Finder

This Rust project is a demo project that demonstrates a basic pattern-matching function that reads input from a file or string and finds a given pattern within it. It utilizes the anyhow crate for improved error handling, simplifying error reporting and providing better context for debugging.

## Features

- Flexible Input: Reads from both a file and a string.
- Pattern Matching: Searches for a specific pattern within the input.
- Error Handling: Leverages the anyhow crate to handle errors gracefully and provide meaningful error messages.

## Usage

### Running the Pattern Finder from the Command Line

You can run the pattern finder from the command line by passing the _pattern_ and _file path_ as arguments respectively:

for example:
`cargo run <pattern> <file path>


reference: [Rust CLI book](https://rust-cli.github.io/book/index.html)