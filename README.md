# wipey [![Crates.io](https://img.shields.io/crates/v/wipey.svg)](https://crates.io/crates/wipey) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/kaihere14/wipey/blob/main/LICENSE) [![Build Status](https://img.shields.io/github/actions/workflow/status/kaihere14/wipey/rust.yml?branch=main)](https://github.com/kaihere14/wipey/actions)

A **Rust-native CLI tool** for safely locating and deleting folders by exact name. Features an interactive UI, dry-run mode, and human-readable size display.

---

## Key Features

- **Interactive Selection**  
  Choose which folders to delete via a fuzzy-select UI powered by `inquire`.

- **Safe Deletion**  
  Use `--dry-run` to preview deletions before executing.

- **Efficient Scanning**  
  Built on `walkdir` for fast recursive directory traversal.

- **Cross-Platform**  
  Works on Windows, macOS, and Linux.

- **Human-Readable Output**  
  Displays folder sizes in KiB/MiB/GiB for easy decision-making.

---

## Installation

### From crates.io (Recommended)
```bash
cargo install wipey
```

### From Source
```bash
git clone https://github.com/kaihere14/wipey.git
cd wipey
cargo build --release
```

### Verify Installation
```bash
wipey --version
```

---

## Usage

```bash
wipey -r <ROOT_DIR> -t <FOLDER_NAME> [OPTIONS]
```

### Example: Delete `target` Folders
```bash
wipey -r . -t target
```

### Dry-Run Preview
```bash
wipey -r /path/to/projects -t __pycache__ -d
```

### CLI Options
| Flag | Description |
|------|-------------|
| `-r, --root` | Root directory to scan (required) |
| `-t, --target` | Folder name to match (required) |
| `-d, --dry-run` | Preview deletions without modifying files |

---

## Tech Stack

- **Language**: Rust 2024  
- **CLI Parsing**: `clap` for argument handling  
- **Interactive UI**: `inquire` for cross-platform prompts  
- **Filesystem**: `walkdir` for efficient directory traversal  
- **Build Tool**: Cargo for dependency management and packaging  

---

## Development

### Run Locally
```bash
cargo run -- -r ./test_dir -t node_modules
```

### Testing
```bash
cargo test
```

### Code Style
```bash
cargo fmt    # Format code
cargo clippy # Lint for issues
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `cargo install` fails | Ensure Rust ≥ 1.78 and internet access to crates.io |
| No folders found | Check case sensitivity in folder name matching |
| Permission denied | Use elevated privileges (`sudo`) or adjust ownership |
| UI crashes | Run in an interactive terminal (not piped output) |

---

## Roadmap

- [ ] Support glob patterns for folder names  
- [ ] Add `--json` output format  
- [ ] Implement `--force` flag to skip prompts  
- [ ] Add `--max-depth` for recursion limits  
- [ ] Expand test coverage with integration tests  

---

## License

MIT License – see [LICENSE](LICENSE) file.

---

## Acknowledgments

- [`clap`](https://crates.io/crates/clap) for CLI parsing  
- [`inquire`](https://crates.io/crates/inquire) for interactive prompts  
- [`walkdir`](https://crates.io/crates/walkdir) for directory traversal  

---

## Contribute

1. Fork the repo and create a feature branch  
2. Follow Rust formatting and linting standards  
3. Add tests for new features  
4. Submit a PR targeting `main`  

See [CONTRIBUTING.md](https://github.com/kaihere14/wipey/blob/main/CONTRIBUTING.md) for details.