# pathcify

[![rust][rust-version-src]][rust-version-href]
[![tests][tests-src]][tests-href]

[Rust](https://www.rust-lang.org/) CLI tool to recursively slugify file and directory names using dots, removing special characters and normalizing names.

Available on [crates.io](https://crates.io/crates/pathcify).

## Requirements

- [Rust](https://www.rust-lang.org/)

## Installation

You can install `pathcify` with [Cargo](https://doc.rust-lang.org/cargo/).

```bash
cargo install pathcify
```

## Usage

You have to pass the path to the `.m4b` file you want to extract chapters from.

```bash
m4b-extractor /path/to/input.m4b
```

You will get a directory named `<input_file>_chapters` containing:

- Each chapter as a separate `.mp3` file.
- A `metadata.json` file with the metadata of the book.
- A `tags.yaml` file with the tags of the book.
- A `folder.jpg` file with the cover of the book.

```plain
1_Chapter 01.mp3
2_Chapter 02.mp3
3_Chapter 03.mp3
# ...
folder.jpg
metadata.json
tags.yaml
```

### Options

```bash
m4b-extractor --help
```

- `-o`, `--output <OUTPUT>`: Specify the output directory for extracted chapters (default: `<input_file>_chapters`).
- `-k`, `--keep`: Keep the original `.m4b` files without converting them to `.mp3`.
- `-q`, `--quality <QUALITY>`: Specify the conversion quality (1=best, 9=worst) for `.mp3` files (default: `2`).
- `-s`, `--sanitize`: Sanitize filenames by replacing invalid characters with underscores (default: `false`).
- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print the version of the tool.

## Build and publish

Build and test the package:

```bash
cargo test
cargo build --release
```

Test publishing:

```bash
cargo publish --dry-run
```

Publish the package to [crates.io](https://crates.io):

```bash
cargo publish
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

[rust-version-src]: https://img.shields.io/badge/Rust-v1.88.0-000000?colorA=18181B&logo=Rust&logoColor=ffffff
[rust-version-href]: https://www.rust-lang.org/
[tests-src]: https://img.shields.io/github/actions/workflow/status/ewilan-riviere/m4b-extractor/run-tests.yml?branch=main&label=tests&style=flat&colorA=18181B
[tests-href]: https://github.com/ewilan-riviere/m4b-extractor/actions
