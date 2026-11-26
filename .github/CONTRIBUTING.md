# Contributing to CodeCraft AI Hardware

This repository owns Seeed device knowledge, native Targets and board support,
cloud Skill contracts, and development workflows. Standalone cloud clients live
in their Skill packages; cloud servers, pairing, account management and telemetry
infrastructure remain external. Each change belongs to its Device, Platform or
Skill owner.

## Before you start

- **Human developers**: start at [docs/README.md](../docs/README.md) and your
  device's knowledge entry under `devices/<device>/AGENTS.md` and its native Target.
- **AI coding agents**: start at [AGENTS.md](../AGENTS.md); it is the
  mandatory entry point with the task routing table.
- Hardware facts follow a strict source priority: vendor specifications and
  measured results → the board's pins header → BSP implementation → hardware
  guide → README/demo code. Never invent a pin number; cite the source of
  every hardware fact in a nearby comment.

## Commits

- Use an English imperative subject in Conventional Commit form:
  `type(scope): description`, with `feat`, `fix`, `docs`, `refactor`,
  `perf`, `test`, `chore`, `build`, or `ci`.
- Keep each commit independently reviewable. Describe the final diff, not
  the debugging journey.
- Exclude credentials, generated firmware, build artifacts, and unrelated
  files.

## Pull requests

1. Create a contribution branch named `feat/<version>/<goal>`, for example
   `feat/v0.1/hardware-architecture-refactor`, from the agreed base.
2. Fill in [`.github/PULL_REQUEST_TEMPLATE.md`](PULL_REQUEST_TEMPLATE.md).
   Report static checks, per-board builds, and on-device verification
   separately; list unperformed hardware work under `Unverified`.
3. Select validation from the actual diff using the
   [root validation policy](../AGENTS.md#required-validation-and-delivery).
   Run focused checks and `validate --static`. If firmware and native build
   inputs/behavior are unchanged, skip native builds and the complete gate.
   CI follows [committed-input selection](../docs/development/ci.md); inspect its
   Target decisions and the fixed **CI result** job. Planned skips are not fresh
   build evidence. Use the appropriate command below only for affected native scope:

   ```bash
   codecraft validate --target <platform>/<device>
   codecraft validate --target <platform>/<device> --projects
   ```

   Narrow native checks with `--example` / `--variant` where applicable; add
   `--projects` when native export contents or build behavior changes. These
   native commands are alternatives, not a mandatory sequence. Use no-option
   `codecraft validate` for an explicit full-build request or native
   changes affecting every Target. Opening a PR or merging already validated
   inputs does not itself require recompilation. Report omitted checks as
   `skipped` with the scope reason, not as fresh build success.

4. A successful build is not hardware validation. Pin, display, bus, and
   power changes need the board revision and observed hardware results
   before the PR is ready to merge; attach photos for display changes.
5. Keep maintained `README.md` / `README.zh_CN.md` pairs aligned. Maintain
   all other engineering documents in English only, including Agent rules,
   Skills, API references, plans and reports. Preserve original source evidence
   and third-party documentation in their original language.

## Issues

Use the GitHub issue templates: bug report, feature request, or usage
question. Always include the board name, hardware revision, and ESP-IDF
version where applicable. Never include credentials, private device data,
or personal data in issues.

## License

By contributing, you agree that your contributions are licensed under the
Apache License 2.0; see [LICENSE](../LICENSE). Third-party attributions are
tracked in [NOTICE](../NOTICE).
