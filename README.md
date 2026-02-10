# rutree2

A Rust command-line tool inspired by `lstree` for displaying directory structures in a tree format.

## Features

- Display directory trees with proper indentation and tree characters
- Show hidden files with the `-a` or `--all` flag
- Limit traversal depth with the `-d` or `--depth` option
- Sort entries alphabetically
- Clean, readable output with visual tree structure

## Installation

Build from source:
```bash
cargo build --release
```

The binary will be available at `target/release/rutree2`.

## Usage

```bash
# Display the current directory
rutree2

# Display a specific directory
rutree2 /path/to/directory

# Show hidden files
rutree2 --all

# Limit depth to 2 levels
rutree2 --depth 2

# Combine options
rutree2 --all --depth 1 /path/to/directory
```

## Options

- `PATH` - Path to display (defaults to current directory)
- `-a, --all` - Show hidden files
- `-d, --depth <DEPTH>` - Maximum depth to traverse
- `-h, --help` - Print help information

## Example Output

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── src/
│   └── main.rs
└── target/
    └── debug/
```

## License

MIT License - see LICENSE file for details