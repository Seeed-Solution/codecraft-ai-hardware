<p align="right">
  <strong>English</strong> · <a href="PULL_REQUEST_TEMPLATE.zh_CN.md">简体中文</a>
</p>

## Summary

<!-- What problem does this PR solve, and which board(s) or subsystems change? -->

## Scope and compatibility

- Affected board(s) / platform:
- Affected subsystem (BSP / example / docs / tooling / skills):
- Wiring or pin-map impact: None
- Flash, partition, or persistent-data impact: None
- New hardware facts introduced (each must cite a vendor source or measured result): None
- Backward-compatibility impact: None

## Verification

| Check | Result | Evidence or notes |
| --- | --- | --- |
| Static checks (`./tools/validate.sh --static`) | PASS / FAIL / NOT RUN | |
| Build per affected board (`./tools/validate.sh --esp32 <board>` or `--linux <board>`) | PASS / FAIL / NOT RUN | |
| On-device verification | PASS / FAIL / NOT RUN | |
| Unverified | — | List every remaining board, instrument, or user check |

A successful build is not hardware validation. Do not mark on-device
verification as done unless the change actually ran on the physical board.

## Device evidence

<!-- Board revision, observed behavior, logs, measurements, and photos where
applicable. Never include credentials or personal data. -->

## Checklist

- [ ] I reviewed the complete diff and excluded unrelated or generated files.
- [ ] Hardware facts cite their source (vendor schematic, vendor BSP, or measured result); no pin numbers were invented.
- [ ] No cloud runtime, device pairing, account system, or telemetry was introduced.
- [ ] Bilingual documentation pairs (`.md` / `.zh_CN.md`) were kept in sync for maintained documents.
- [ ] Board READMEs and `docs/vendor-sources.md` were updated for changed hardware facts.
- [ ] I removed credentials, personal data, and unsanitized logs.
