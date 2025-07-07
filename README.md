# pathcify

[![rust][rust-version-src]][rust-version-href]
[![tests][tests-src]][tests-href]

[Rust](https://www.rust-lang.org/) CLI tool to recursively slugify file and directory names using dots, removing special characters and normalizing names.

Available on [crates.io](https://crates.io/crates/pathcify).

For examples:

- `La Quête d'Ewilan vol.1 : D'un monde à l'autre-·/_,:; (1), [Bottero, Pierre]`Author` @{1} <book> ?!//&` to `la.quete.dewilan.vol.1.dun.monde.a.lautre-._.1.bottero.pierre.author.{1}.book` with lowercase
- `00 - Préface` to `00-Preface`
- `Góðan daginn` to `Godan.Daginn`

## Requirements

- [Rust](https://www.rust-lang.org/)

## Installation

You can install `pathcify` with [Cargo](https://doc.rust-lang.org/cargo/).

```bash
cargo install pathcify
```

## Usage

You can execute `pathcify` on a directory or a file:

```bash
pathcify /path/to/dir
```

All files and directories will be recursively pathcified, converting their names to a format suitable for URLs or filenames.

- Remove all special characters
- Replace all spaces with a dot
- Remove all dots at the beginning and the end of the string
- Replace all dots that are repeated more than once with a single dot
- Keep `-` and `_` characters (and remove spaces before and after them)
- Full lowercase with option `-l`

### Options

```bash
pathcify --help
```

- `-l`, `--lowercase`: Convert all names to lowercase.
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
[tests-src]: https://img.shields.io/github/actions/workflow/status/ewilan-riviere/pathcify/run-tests.yml?branch=main&label=tests&style=flat&colorA=18181B
[tests-href]: https://github.com/ewilan-riviere/pathcify/actions
