# Advent of Code 2023

My solutions for advent of code 2023.

## Installing

You need Rust-lang to run this code. Using [rustup](https://rustup.rs/) is recommended.

## Running

To run the code for a specific day, use the following command:

`cargo run <day>` where `<day>` is the day you want to run. By default, the specific day code loads the input from `data/day<day>.txt`. If you want to load the input from somewhere else, you can use `cargo run <day> <input>` where `<input>` is the path to the input file.

## Testing

To run the tests, use the following command:

`cargo test`

By default we have some test coverage on most of the days. If you want to run the tests for a specific day, use the following command: `cargo test day<day>` example: `cargo test day1`.