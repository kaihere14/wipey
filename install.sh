#!/bin/sh
# wipey installer — downloads the right prebuilt binary for this machine.
#
#   curl -fsSL https://raw.githubusercontent.com/kaihere14/wipey/main/install.sh | sh
#
# Options (environment variables):
#   WIPEY_VERSION=v0.1.0   install a specific version instead of the latest
#   WIPEY_INSTALL_DIR=...  install somewhere other than the default
#
# POSIX sh on purpose — this has to run under dash, busybox ash, and macOS sh.

set -eu

REPO="kaihere14/wipey"
BIN="wipey"

info() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarn:\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; exit 1; }

have() { command -v "$1" >/dev/null 2>&1; }

# --- fetch helper -----------------------------------------------------------
if have curl; then
    fetch()      { curl -fsSL "$1"; }
    fetch_file() { curl -fsSL "$1" -o "$2"; }
elif have wget; then
    fetch()      { wget -qO- "$1"; }
    fetch_file() { wget -qO "$2" "$1"; }
else
    die "need curl or wget to download"
fi

# --- work out which build to grab -------------------------------------------
os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
    Darwin) platform="apple-darwin" ;;
    Linux)  platform="unknown-linux-musl" ;;
    MINGW*|MSYS*|CYGWIN*)
        die "on Windows, download the .zip from https://github.com/$REPO/releases" ;;
    *) die "unsupported OS: $os" ;;
esac

case "$arch" in
    x86_64|amd64)  cpu="x86_64" ;;
    arm64|aarch64) cpu="aarch64" ;;
    *) die "unsupported architecture: $arch" ;;
esac

target="${cpu}-${platform}"

# --- resolve version --------------------------------------------------------
version="${WIPEY_VERSION:-}"
if [ -z "$version" ]; then
    info "looking up latest release"
    # 2>/dev/null: a repo with no published release 404s here, and the raw
    # curl/wget error is noise next to the explanation below.
    version="$(fetch "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null \
        | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -n1)"
    [ -n "$version" ] || die "could not determine the latest version — is a release published yet? (draft releases don't count)"
fi

asset="${BIN}-${version}-${target}.tar.gz"
url="https://github.com/$REPO/releases/download/$version/$asset"

# --- pick an install directory ----------------------------------------------
if [ -n "${WIPEY_INSTALL_DIR:-}" ]; then
    install_dir="$WIPEY_INSTALL_DIR"
elif [ -w /usr/local/bin ] 2>/dev/null; then
    install_dir="/usr/local/bin"
else
    install_dir="$HOME/.local/bin"
fi

# --- download and install ---------------------------------------------------
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT INT TERM

info "downloading $BIN $version ($target)"
fetch_file "$url" "$tmp/$asset" \
    || die "download failed: $url
Check that $version has a build for $target at https://github.com/$REPO/releases"

tar -xzf "$tmp/$asset" -C "$tmp"

# Archives contain a versioned directory; find the binary wherever it landed.
binary="$(find "$tmp" -type f -name "$BIN" -perm -u+x 2>/dev/null | head -n1)"
[ -n "$binary" ] || die "archive did not contain a $BIN binary"

mkdir -p "$install_dir"
install -m 755 "$binary" "$install_dir/$BIN" 2>/dev/null \
    || { cp "$binary" "$install_dir/$BIN" && chmod 755 "$install_dir/$BIN"; }

info "installed to $install_dir/$BIN"

# --- post-install sanity ----------------------------------------------------
case ":$PATH:" in
    *":$install_dir:"*) ;;
    *)
        warn "$install_dir is not on your PATH"
        printf '  Add this to your shell profile:\n    export PATH="%s:$PATH"\n' "$install_dir"
        ;;
esac

if "$install_dir/$BIN" --version >/dev/null 2>&1; then
    info "$("$install_dir/$BIN" --version) is ready — try: $BIN --help"
else
    warn "installed, but '$BIN --version' did not run cleanly"
fi
