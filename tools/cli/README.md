[简体中文](README.zh_CN.md)

# CodeCraft CLI

Standalone Rust creator CLI. Runtime data is read from the selected repository or
project; it is not embedded in the binary. Normal commands never invoke Cargo.

Install the latest stable Release with `sh tools/cli/install.sh` from the repository root
(`tools/cli/install.ps1` on Windows), then use `codecraft --help`. The latest release tag is resolved once before downloading its binary and checksum.
Select a specific published version with `--version X.Y.Z`
(`-Version X.Y.Z` on Windows). Downloads are verified before replacing an existing
installation; there is no automatic source-build fallback.

For CLI development, use `sh tools/cli/install.sh --from-source`
(`tools/cli/install.ps1 -FromSource` on Windows). Only source builds require Cargo
and a native linker. Downloaded binaries require neither Rust nor Python.

The working-tree CLI adds `codecraft doctor --target <stack>/<device>` for
read-only activated-toolchain inspection. It requires a source installation
until a release containing this command is published; v0.5.0 does not include it.

Use `cargo test --locked --package codecraft` for integration tests. Tests belong
in tests/tools/cli/, platform implementation belongs in platforms/<stack>/native.rs.
Read [runtime and update policy](../../docs/development/rust-cli.md).
