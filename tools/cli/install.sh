#!/bin/sh
# Explicit installation only; ordinary codecraft commands never install or compile.
set -eu
version=latest
release_repo=Seeed-Solution/codecraft-ai-hardware
from_source=false
while [ "$#" -gt 0 ]; do
    case "$1" in
        --version|--repo)
            [ "$#" -ge 2 ] || { printf 'Missing value for %s\n' "$1" >&2; exit 1; }
            case "$1" in --version) version=$2 ;; --repo) release_repo=$2 ;; esac
            shift 2 ;;
        --from-source) from_source=true; shift ;;
        --help)
            printf 'Usage: sh tools/cli/install.sh [--version latest|X.Y.Z] [--repo OWNER/REPO] [--from-source]\n'
            exit 0 ;;
        *) printf 'Unknown option: %s\n' "$1" >&2; exit 1 ;;
    esac
done
[ "$version" = latest ] || printf '%s\n' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$' || { printf 'Use latest or an explicit X.Y.Z version.\n' >&2; exit 1; }
printf '%s\n' "$release_repo" | grep -Eq '^[A-Za-z0-9_-]+/[A-Za-z0-9_.-]+$' || { printf 'Invalid release repository.\n' >&2; exit 1; }
install_dir=${CODECRAFT_BIN_DIR:-"$HOME/.local/bin"}
stage=
work=
cleanup() { [ -z "$stage" ] || rm -f "$stage"; [ -z "$work" ] || rm -rf "$work"; }
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
if [ "$from_source" = true ]; then
    repo=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
    cd "$repo"
    codecraft_host_target=$(rustc -vV | sed -n 's/^host: //p')
    [ -n "$codecraft_host_target" ] || { printf 'Cannot determine Rust host target.\n' >&2; exit 1; }
    cargo build --release --locked --package codecraft --target-dir .codecraft/cargo --target "$codecraft_host_target"
    binary="$repo/.codecraft/cargo/$codecraft_host_target/release/codecraft"
else
    case "$(uname -s)/$(uname -m)" in
        Linux/x86_64) target=x86_64-unknown-linux-musl ;;
        Darwin/arm64|Darwin/aarch64) target=aarch64-apple-darwin ;;
        *) printf 'No published binary for this host; use --from-source explicitly.\n' >&2; exit 1 ;;
    esac
    command -v curl >/dev/null || { printf 'curl is required.\n' >&2; exit 1; }
    if [ "$version" = latest ]; then
        resolved=$(curl --fail --silent --show-error --location --proto '=https' --proto-redir '=https' --retry 2 "https://github.com/$release_repo/releases/latest" -o /dev/null --write-out '%{url_effective}')
        prefix="https://github.com/$release_repo/releases/tag/v"
        case "$resolved" in
            "$prefix"*) version=${resolved#"$prefix"} ;;
            *) printf 'Cannot resolve the latest release tag.\n' >&2; exit 1 ;;
        esac
        printf '%s\n' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$' || { printf 'Latest release must use a vX.Y.Z tag.\n' >&2; exit 1; }
    fi
    printf 'Installing CodeCraft %s from %s\n' "$version" "$release_repo"
    work=$(mktemp -d)
    asset=codecraft-$target
    base="https://github.com/$release_repo/releases/download/v$version"
    for name in "$asset" SHA256SUMS; do
        curl --fail --silent --show-error --location --proto '=https' --proto-redir '=https' --retry 2 "$base/$name" -o "$work/$name"
    done
    expected=$(awk -v name="$asset" '$2 == name { print $1 }' "$work/SHA256SUMS")
    [ "${#expected}" -eq 64 ] && printf '%s\n' "$expected" | grep -Eq '^[a-fA-F0-9]{64}$' || { printf 'Missing or ambiguous SHA256SUMS entry.\n' >&2; exit 1; }
    if command -v shasum >/dev/null; then
        actual=$(shasum -a 256 "$work/$asset" | awk '{print $1}')
    elif command -v sha256sum >/dev/null; then
        actual=$(sha256sum "$work/$asset" | awk '{print $1}')
    else
        printf 'shasum or sha256sum is required.\n' >&2; exit 1
    fi
    [ "$actual" = "$expected" ] || { printf 'Checksum mismatch; existing installation preserved.\n' >&2; exit 1; }
    binary="$work/$asset"
fi
mkdir -p "$install_dir"
stage=$(mktemp "$install_dir/.codecraft.XXXXXX")
cp "$binary" "$stage"
chmod 755 "$stage"
if [ "$from_source" = false ]; then
    [ "$("$stage" --version)" = "codecraft $version" ] || { printf 'Binary version mismatch; existing installation preserved.\n' >&2; exit 1; }
fi
mv -f "$stage" "$install_dir/codecraft"
stage=
printf 'Installed %s/codecraft. Add this directory to PATH.\n' "$install_dir"
