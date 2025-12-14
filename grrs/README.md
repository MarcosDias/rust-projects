# grrs

**grrs** is a simple command-line tool to search for a pattern in files, inspired by the classic `grep` utility. This project was built as a learning exercise by following the [Rust CLI Book](https://rust-cli.github.io/book/index.html).

## Usage

```sh
grrs <PATTERN> <FILE>
```

- `<PATTERN>`: The text pattern to search for.
- `<FILE>`: The file to search in.

## Example

```sh
grrs foo ./test.txt
```

This will print all lines in `test.txt` that contain the word `foo`.

## Installation

To build and install `grrs` locally, run:

```sh
cargo install --path .
```

## Running Tests

To run the tests:

```sh
cargo test
```

## License

This project is licensed under the MIT License.
