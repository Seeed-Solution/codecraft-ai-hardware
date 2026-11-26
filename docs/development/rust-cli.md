# Rust CLI and runtime ownership

The creator entry is the standalone `codecraft` binary (CLI contract 4). Discovery,
Device Markdown parsing, context, project export, source locks and packaging run
in Rust. These commands do not invoke Python, Cargo, platform installers or cloud
services. Data is read from `--root` (the current directory by default), not
compiled into the executable.

## Install and update

Download the appropriate executable from the repository's GitHub Release
(starting with `v0.5.0`), verify it against the attached `SHA256SUMS`, and rename
it to `codecraft` (`codecraft.exe` on Windows). On Unix, run `chmod +x codecraft`
and place it on your PATH. Alternatively, use a completed **CodeCraft CLI binaries**
workflow artifact, or use the checksum-verifying installer:

```sh
sh tools/cli/install.sh
# Add ~/.local/bin to PATH, or set CODECRAFT_BIN_DIR before installation.
codecraft --version
codecraft --root /path/to/codecraft-ai-hardware discover
```

The installer defaults to GitHub's latest stable Release in
`Links17/codecraft-ai-hardware` (drafts and prereleases are excluded). It resolves
the concrete `vX.Y.Z` tag once and downloads both binary and checksums from that
version, so a concurrent new release cannot mix assets. Resolution failure stops
installation without replacing an existing executable. Select a published version with `--version X.Y.Z`
or `-Version X.Y.Z`; select a release repository explicitly with `--repo OWNER/REPO`
or `-Repository OWNER/REPO`. Unsupported hosts fail with a source-build hint.
Checksum/download/version failures leave an existing installation intact.
No SDK, shell profile or PATH configuration is changed automatically.

For source builds, use `sh tools/cli/install.sh --from-source` or
`tools/cli/install.ps1 -FromSource`. Windows installations use `tools/cli/install.ps1`; the default destination
is `%LOCALAPPDATA%/CodeCraft/bin`. A downloaded executable needs neither Rust nor
Python. Source installations require Cargo and a native linker. No binary release
is published automatically by this repository. Releases use the matching Cargo
version and `v<version>` Git tag on a validated main commit; maintainers attach
the three platform binaries from that commit's successful CI run and checksums.

Run the installer with the source option after changing Rust implementation/dependencies. It uses
Cargo's incremental release build; normal `codecraft` invocations never run Cargo.
Device data, application code, Markdown updates do not
rebuild the CLI. Rust sources, Cargo manifests/lock, compiler/configuration or a
new operating-system/architecture target may require a build. CI caches binaries
using those inputs, separately from hardware/data changes. It does not include
Git commit IDs, device files or documentation in the binary cache key.

## Read-only environment preflight

The working-tree CLI provides `codecraft doctor --target <stack>/<device>`,
using `--root` for a repository or exported application. This is not yet in the
published v0.5.0 binary; use a source build for this new command. Skills check
command availability before using it and retain native setup guidance for v0.5.0.

Doctor reuses the platform build probe and reports expected Target requirements,
actual activated-toolchain identity, and setup guidance when unavailable. Exit 0
means the probe passed; exit 2 means the environment is not ready or could not be
inspected; invalid Target/input is exit 1. It does not install, activate, compile,
update locks or access a device. It does not search arbitrary SDK directories.
A passed probe is not a build, source-lock check or device-runtime certification.

## Project lifecycle

```sh
codecraft new --target esp-idf/reterminal-e1001 --example epd_hello --output projects/my-app
codecraft verify-project projects/my-app
# Only when preparing a native build, activate the declared SDK first:
codecraft lock-project projects/my-app --toolchain
codecraft build-project projects/my-app
```

Creation and ordinary relocking do not probe the SDK. `--toolchain` explicitly
records the active environment. A missing toolchain prevents a build; it does not
prevent authoring. Existing toolchain identity is preserved during source-only
relocking when the declared requirement is unchanged. No command silently
installs tools or flashes a board.

Project profiles, source graphs and locks use schema 4. The lock separately stores
original import provenance, current source hashes, declared/resolved toolchain
and CLI version/contract/binary hash. A different CLI can inspect source integrity;
building requires reviewing and relocking its identity. Source edits require
relocking but never rebuilding the CLI. Custom application AGENTS rules outside
the generated region survive relocking.

Schema 3 Python-runtime exports are not reinterpreted as schema 4 projects. Keep
their original tooling to reproduce them, or create a fresh schema 4 export and
review/copy application changes. Never rewrite their original evidence or import
hashes. Target schema 3 and authored Device schema 5 are unchanged.

## Python boundaries

Native C/C++ exports contain no CodeCraft Python runtime, Cargo project or Rust
source. Platform `application_inputs` declares portable guidance and native
helpers; repository maintenance is compiled into the installed tool and is not an export dependency.
Missing declared inputs fail export rather than being filtered out by suffix.

Python remains legitimate in the following explicitly selected scopes:

- ESP-IDF's own SDK tooling, activated before native build/image operations.
- Linux applications whose native example itself is written in Python.
- Explicitly selected standalone cloud Skills whose client is Python. Selection
  verifies and copies their complete released package without executing it.
- Independent Python client and native-host test fixtures under tests/, dispatched
  by the repository tool through an explicitly configured Python/pytest runtime.

Repository maintenance commands (generation, static checks, evidence, CI and
package releases) are implemented in Rust. There is no tools/ Python backend or
second project/catalog implementation. `codecraft validate --static` runs source
checks and independent host suites; Cargo integration tests remain a separate
required gate. Missing host-test dependencies are reported, not skipped as passed.

## Evidence compatibility

New evidence uses record schema 3 and creator graph schema 4, with compiled CLI
identity, source digest, complete cases and actual toolchain/log bindings. Context
and generated inventories read this evidence. Physical promotion additionally
requires revision-specific named observations and compatible variant bindings.

Historical record schema 2 remains immutable archival evidence and cannot promote
the current graph. Its original logs, timestamps and source digests are preserved.
No schema 3 Python project runtime remains; recreate old application exports from
reviewed source using the current schema rather than reinterpret their locks.

## Maintenance candidate compatibility

Release candidates now use schema 3 and bind the maintenance executable plus
validation-source identity. Re-prepare older candidates and review the new digest.
Existing package bytes, release inventories and immutable import records are not
changed merely by this implementation migration. See the
[migration TODO](tools-rust-migration-todo.md) for scope and validation.

Platform metadata now uses schema 3: adapter identifies the platform-owned
compiled implementation and the old Python runtime closure is removed. Existing
exports with platform schema 2 require a fresh export and reviewed application
changes; their original locks remain historical. Creator profile/lock schema 4
and Target schema 3 are unchanged. No mixed platform-schema reader is retained.
