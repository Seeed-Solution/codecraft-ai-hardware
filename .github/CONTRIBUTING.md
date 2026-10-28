<p align="right">
  <strong>English</strong> · <a href="CONTRIBUTING.zh_CN.md">简体中文</a>
</p>

# Contributing to CodeCraft AI Hardware

This is a pure hardware capability reference repository: board support
packages, standalone examples, and verified hardware facts for Seeed devices.
There is deliberately no cloud runtime, device pairing, account system, or
telemetry — contributions must not introduce any of these.

## Before you start

- **Human developers**: start at [docs/README.md](../docs/README.md) and your
  board's README under `esp32/boards/` or `linux/boards/`.
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

1. Fork the repository and create your branch from `main`.
2. Fill in [`.github/PULL_REQUEST_TEMPLATE.md`](PULL_REQUEST_TEMPLATE.md).
   Report static checks, per-board builds, and on-device verification
   separately; list unperformed hardware work under `Unverified`.
3. Run the validation gate before opening the PR:

   ```bash
   ./tools/validate.sh --static           # repository checks + host tests
   ./tools/validate.sh --esp32 <board>    # ESP-IDF build for the affected board
   ./tools/validate.sh --linux <board>    # Linux board checks
   ```

4. A successful build is not hardware validation. Pin, display, bus, and
   power changes need the board revision and observed hardware results
   before the PR is ready to merge; attach photos for display changes.
5. Keep bilingual documentation pairs (`.md` / `.zh_CN.md`) in sync for
   maintained documents.

## Issues

Use the GitHub issue templates: bug report, feature request, or usage
question. Always include the board name, hardware revision, and ESP-IDF
version where applicable. Never include credentials, private device data,
or personal data in issues.

## License

By contributing, you agree that your contributions are licensed under the
Apache License 2.0; see [LICENSE](../LICENSE). Third-party attributions are
tracked in [NOTICE](../NOTICE).
