# wipey &nbsp;![Crates.io](https://img.shields.io/crates/v/wipey.svg) ![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Build Status](https://img.shields.io/github/actions/workflow/status/kaihere14/wipey/release.yml) ![Docs.rs](https://img.shields.io/docsrs/wipey)

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

* **Interactive** – you decide which matches to delete via a fuzzy‑select UI.
* **Safe** – a `--dry-run` flag shows exactly what would be removed without touching the filesystem.
* **Fast** – built on `walkdir` for efficient recursive traversal and `clap` for zero‑cost argument parsing.

> **Who should use it?**  
Developers, DevOps engineers, or anyone who regularly needs to purge repetitive folder structures from many projects.

---

## Features
| Feature | Description | Status |
|---------|-------------|--------|
| Recursive search | Walks the entire directory tree starting at a user‑provided root. | ✅ Stable |
| Multiple target names | Match several folder names in one pass — `-t node_modules,target` or repeated `-t` flags. | ✅ Stable |
| Targeted folder matching | Finds directories whose **exact** name matches any supplied target. | ✅ Stable |
| Human‑readable size display | Shows each candidate folder’s size in KiB/MiB/GiB. | ✅ Stable |
| Interactive multi‑select UI | Powered by the `inquire` crate; pick any subset of matches. | ✅ Stable |
| Dry‑run mode (`-d`/`--dry-run`) | Preview deletions without performing them. | ✅ Stable |
| Colorful terminal output | Uses standard `println!` with clear formatting for success/failure. | ✅ Stable |
| Progress spinner | A rotating `/ - \\ |` indicator on stderr while scanning and measuring, so large trees don't look frozen. | ✅ Stable |
| Cross‑platform | Works on Windows, macOS, and Linux (any platform supported by Rust). | ✅ Stable |

---

## Tech Stack
| Layer | Technology | Reason |
|-------|------------|--------|
| Language | **Rust 2024** | Zero‑cost abstractions, safety, and compiled binaries. |
| CLI parsing | `clap = "4.6.5"` (with `derive` feature) | Declarative argument definition, automatic help generation. |
| Interactive prompts | `inquire = "0.9.4"` | Simple, cross‑platform multi‑select UI. |
| Filesystem traversal | `walkdir = "2"` | Efficient recursive directory walking. |
| Packaging | Cargo (Rust’s package manager) | Handles building, testing, and publishing to crates.io. |

---

## Architecture
```
wipey (binary)
│
├─ src/main.rs          ← Entry point, CLI handling, core logic
│   ├─ Args            ← clap‑derived struct for CLI flags
│   ├─ Spinner         ← background progress indicator (stderr, TTY-only)
│   ├─ find_matches()  ← WalkDir recursion, collects paths matching any target
│   ├─ folder_size()   ← Recursively sums file sizes in a folder
│   ├─ human_size()    ← Pretty‑prints bytes → KiB/MiB/GiB
│   └─ main()          ← Orchestrates UI, dry‑run, and deletion
│
└─ Cargo.toml           ← Dependency & metadata definition
```

* **`Args`** – Holds `root`, `target` (a `Vec<String>`, one or more names), and `dry_run` flags parsed by `clap`.  
* **`find_matches`** – Walks the tree, stops descending into a matched directory (`it.skip_current_dir()`) to avoid double‑counting.  
* **`Spinner`** – Runs on its own thread polling an `AtomicUsize` counter, so progress updates without slowing the walk. Silent when stderr isn't a TTY; stops itself on `Drop`.  
* **`FolderEntry`** – Simple struct used for displaying path + size in the selection UI.  
* **Deletion flow** – After the user selects entries, either a dry‑run summary is printed or `std::fs::remove_dir_all` is called for each path.

---

## Getting Started

### Prerequisites
| Requirement | Minimum Version |
|-------------|-----------------|
| Rust toolchain (`rustc`, `cargo`) | 1.78 (edition 2024) |
| Operating System | Windows 10 / macOS 12 / Linux kernel 5.4+ |
| Optional: `git` (for cloning) | any |

> **Tip:** Install Rust via [rustup.rs](https://rustup.rs) – it provides the latest stable compiler and Cargo.

### Installation

#### 1️⃣ Install the prebuilt binary (recommended)
No Rust toolchain needed — this downloads the right binary for your machine:

```bash
curl -fsSL https://raw.githubusercontent.com/kaihere14/wipey/main/install.sh | sh
```

Installs to `/usr/local/bin` if it's writable, otherwise `~/.local/bin`. Pin a
specific version with `WIPEY_VERSION=v0.1.0`, or change the destination with
`WIPEY_INSTALL_DIR=~/bin`.

**Windows:** download the `.zip` from the
[releases page](https://github.com/kaihere14/wipey/releases) and put `wipey.exe`
on your `PATH`.

#### 2️⃣ Install with Cargo
```bash
cargo install --git https://github.com/kaihere14/wipey
# Binary will be placed in $HOME/.cargo/bin (add to $PATH if needed)
```

> Not on crates.io yet — the name is available and reserved-in-spirit, so this
> will become `cargo install wipey` once it's published.

#### 3️⃣ Build from source (for the latest commit)
```bash
git clone https://github.com/kaihere14/wipey.git
cd wipey
cargo build --release
```

#### 4️⃣ Verify installation
```bash
wipey --version
```

<<<<<<< HEAD
=======
### Configuration & CLI Flags
| Flag | Short | Long | Description | Example |
|------|-------|------|-------------|---------|
| `root` | `-r` | `--root` | Path to the directory where the search starts. **Required**. | `-r ./projects` |
| `target` | `-t` | `--target` | Folder name(s) to look for. Accepts a comma‑separated list or repeated flags. **Required**. | `-t node_modules,target` |
| `dry_run` | `-d` | `--dry-run` | Show what would be deleted without actually removing anything. | `-d` |

>>>>>>> c8002aa (feat: add installation script and improve CLI functionality)
---

## Usage

```bash
wipey -r <ROOT_DIR> -t <FOLDER_NAME> [OPTIONS]
```

### Example: Delete `target` Folders
```bash
wipey -r . -t target
```

*The program will:*
1. Scan the current directory (`.`) for folders named `target`.
2. Display each match with its size.
3. Prompt you to select which ones to delete.
4. Remove the selected folders.

While it scans, a spinner shows live progress so a big tree never looks frozen:

```text
/ Scanning... (14 found)
- Measuring... (9 sized)
```

Matches are listed largest-first, since the point is reclaiming space.

### Multiple Targets
Clean several kinds of build directory in one pass. Comma-separated:

```bash
wipey -r ~/projects -t node_modules,target,__pycache__
```

...or by repeating the flag, which is handy in shell scripts:

```bash
wipey -r ~/projects -t node_modules -t target -t .venv
```

Both forms are equivalent. A directory matching *any* target is collected, and
`wipey` does not descend into a match — so a `node_modules` nested inside
another one is never counted twice.

### Dry‑Run Example
Preview deletions without touching the filesystem:

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

<<<<<<< HEAD
- **Language**: Rust 2024  
- **CLI Parsing**: `clap` for argument handling  
- **Interactive UI**: `inquire` for cross-platform prompts  
- **Filesystem**: `walkdir` for efficient directory traversal  
- **Build Tool**: Cargo for dependency management and packaging  
=======
USAGE:
    wipey [OPTIONS] --root <ROOT> --target <TARGET>...

OPTIONS:
    -d, --dry-run          Perform a dry run (no deletions)
    -h, --help             Print help information
    -r, --root <ROOT>      Root directory to start the search
    -t, --target <TARGET>...  Folder name(s) to match; repeatable or comma-separated
    -V, --version          Print version information
```
>>>>>>> c8002aa (feat: add installation script and improve CLI functionality)

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

| Problem | Solution |
|---------|----------|
| **`cargo install wipey` fails with “cannot find crate `wipey`”** | Expected — `wipey` isn't on crates.io yet. Use the install script, or `cargo install --git https://github.com/kaihere14/wipey`. |
| **macOS: “cannot be opened because the developer cannot be verified”** | Release binaries are unsigned. The install script avoids this; for a manual download, clear the flag with `xattr -d com.apple.quarantine wipey`. |
| **No folders are listed even though they exist** | The search is *exact*; ensure the `target` argument matches the folder name case‑sensitively. |
| **Permission denied when deleting** | Run the command with sufficient privileges (e.g., `sudo` on Linux/macOS) or choose a different root that you own. |
| **The spinner doesn't appear** | It only draws when stderr is a terminal, so it stays out of piped or redirected output. |
| **The UI hangs or crashes** | `inquire` requires a TTY. Make sure you are running the binary in an interactive terminal, not piping output to a file. |
| **I want to match multiple folder names** | Supported — pass them comma‑separated (`-t node_modules,target`) or repeat the flag (`-t node_modules -t target`). |

If you encounter other issues, feel free to open an **Issue** on GitHub with a minimal reproducible example.

---

## Roadmap
- [x] Support multiple target names in one run.  
- [ ] Support glob patterns for `target` (e.g., `*cache*`).  
- [ ] Add a `--json` flag to output matches in machine‑readable format.  
- [ ] Implement a `--force` flag to skip the interactive prompt.  
- [ ] Provide a `--max-depth` option to limit recursion depth.  
- [ ] Add integration tests covering the full deletion flow.

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