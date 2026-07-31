# wipey

[![release](https://img.shields.io/github/actions/workflow/status/kaihere14/wipey/release.yml?label=release)](https://github.com/kaihere14/wipey/actions/workflows/release.yml)
[![downloads](https://img.shields.io/github/downloads/kaihere14/wipey/total)](https://github.com/kaihere14/wipey/releases)

Build directories pile up. Every Node project leaves a `node_modules`, every
Rust project a `target`, every Python project a `__pycache__` — and after a year
of side projects there are gigabytes of them scattered across your disk that you
can't easily find, let alone judge which are safe to remove.

`wipey` finds them, tells you how big each one is, and lets you choose.

```text
$ wipey -r ~/projects -t node_modules,target
Found 12 folder(s) matching "node_modules", "target" — 4.81 GB total.
? Select folders to delete:
> [x] /Users/me/projects/web/node_modules       (1.21 GB)
  [ ] /Users/me/projects/api/target             (890.44 MB)
  [x] /Users/me/projects/old-demo/node_modules  (412.83 MB)
[↑↓ to move, space to select one, → to all, ← to none, type to filter]
```

Nothing is deleted unless you tick it.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/kaihere14/wipey/main/install.sh | sh
```

No Rust toolchain required — this downloads a prebuilt binary for your platform.
It installs to `/usr/local/bin` when that's writable, otherwise `~/.local/bin`.

<details>
<summary>Other ways to install</summary>

**Pin a version, or choose where it lands:**

```sh
WIPEY_VERSION=v0.1.0 WIPEY_INSTALL_DIR=~/bin \
  curl -fsSL https://raw.githubusercontent.com/kaihere14/wipey/main/install.sh | sh
```

**With Cargo**, building from source (needs Rust 1.85 or newer for edition 2024):

```sh
cargo install --git https://github.com/kaihere14/wipey
```

**By hand** — grab the archive for your platform from the
[releases page](https://github.com/kaihere14/wipey/releases), extract it, and
move `wipey` somewhere on your `PATH`.

**Windows** — download the `.zip` from releases. The shell installer is
Unix-only.

</details>

## Usage

```sh
wipey --root <DIR> --target <NAME>... [--dry-run]
```

| Flag | Short | Description |
| --- | --- | --- |
| `--root` | `-r` | Directory to search from. Required. |
| `--target` | `-t` | Folder name(s) to match. Required, repeatable. |
| `--dry-run` | `-d` | Print what would be deleted, then exit. |

### Start with a dry run

`--dry-run` does the full scan and shows you the outcome without touching
anything:

```sh
wipey -r ~/projects -t node_modules --dry-run
```

```text
[DRY RUN] The following would be deleted:
  /Users/me/projects/web/node_modules  (1.21 GB)
  /Users/me/projects/api/node_modules  (412.83 MB)
[DRY RUN] Would free 1.61 GB.
```

Drop the flag to delete for real. Either way, selecting nothing and pressing
enter exits without changes.

### Several names at once

Pass a comma-separated list:

```sh
wipey -r ~/projects -t node_modules,target,__pycache__
```

Or repeat the flag, which reads better in scripts:

```sh
wipey -r ~/projects -t node_modules -t target -t .venv
```

The two forms are equivalent — any directory matching *any* of the names is
collected.

### Progress

Large trees take a few seconds to walk, so progress is reported on stderr:

```text
/ Scanning... (14 found)
- Measuring... (9 sized)
```

The spinner only draws when stderr is a terminal, so piped and redirected
output stays clean.

## How it works

Starting at `--root`, `wipey` walks the directory tree looking for directories
whose name exactly matches one of your targets. On a match it records the path
and **stops descending** — so a `node_modules` nested inside another one is
never double-counted, and the scan skips the largest, densest parts of the tree
entirely.

Each match is then measured by summing the sizes of every file beneath it, and
results are sorted **largest first**, because the biggest wins are the ones you
actually care about.

Two things to be aware of:

- Matching is **exact and case-sensitive**. `-t Node_Modules` will not find
  `node_modules`, and glob patterns aren't supported yet.
- Sizes are summed from file lengths, so they may differ slightly from `du` or
  Finder, which account for block allocation.

## Platforms

| OS | Architectures |
| --- | --- |
| macOS | Apple Silicon, Intel |
| Linux | x86_64, aarch64 — static musl builds, no glibc dependency |
| Windows | x86_64 |

Release binaries are unsigned. The install script avoids macOS Gatekeeper, but
if you download an archive manually you may need to clear the quarantine flag:

```sh
xattr -d com.apple.quarantine wipey
```

Windows SmartScreen will warn the first time you run the `.exe`.

## A word of caution

`wipey` removes directories recursively and **does not use the trash**.
Deletions are immediate and permanent.

The interactive prompt is the safeguard — nothing goes without an explicit
selection — but a broad `--root` combined with a common `--target` can surface a
lot at once. Run `--dry-run` first whenever you point it somewhere new.

## Development

```sh
git clone https://github.com/kaihere14/wipey.git
cd wipey
cargo run -- -r ./some/test/dir -t node_modules --dry-run
```

Before opening a pull request:

```sh
cargo fmt
cargo clippy -- -D warnings
```

There are no automated tests yet — `folder_size` and `human_size` are the
natural place to start if you'd like to add some.

## Releasing

Push a version tag. CI builds all five targets on native runners and opens a
**draft** release with binaries and checksums attached; review it on GitHub,
then publish.

```sh
git tag v0.1.0
git push origin v0.1.0
```

See [`.github/workflows/release.yml`](.github/workflows/release.yml).

## Roadmap

- [x] Multiple target names in a single run
- [x] Prebuilt binaries and a one-line installer
- [ ] Glob patterns for `--target`, e.g. `*cache*`
- [ ] `--force` to skip the prompt for scripted use
- [ ] `--max-depth` to limit recursion
- [ ] Publish to crates.io
- [ ] Unit tests

## License

Not yet licensed. Until a `LICENSE` file is added, default copyright applies and
others have no permission to use, copy, or redistribute this code.

## Built with

[clap](https://github.com/clap-rs/clap) for argument parsing,
[inquire](https://github.com/mikaelmello/inquire) for the selection UI, and
[walkdir](https://github.com/BurntSushi/walkdir) for traversal.
