# Private application workspaces

`codecraft new` exports a portable native application into a new `projects/<name>` directory. Explicit external directories remain supported. The chosen directory must not exist. Other paths inside the reference repository, parent traversal and symlinks within an internal workspace path are rejected before writing.

`projects/.gitignore` keeps private applications and brief files out of Git. The reference catalog scans only its declared owners; reference documentation checks and test fixtures exclude private projects. Source dependencies cannot point into projects/, including through an alias. Explicit application verification still checks the application's own inventory. Do not force-add private code, credentials, generated firmware or build output to the reference repository.

## From intent to a native project

Use [codecraft-development-ideate](../../skills/codecraft-development-ideate/SKILL.md) when the application needs clarification. Discover real candidates and collect their source closure:

```bash
codecraft discover
codecraft context --target esp-idf/reterminal-e1001 --task ideate
```

Save a confirmed brief to `projects/desk-display.brief.json`. JSON is the input format; project.yaml is JSON-compatible YAML 1.2. The following describes intent only and makes no hardware compatibility claim:

```json
{
  "name": "Desk display",
  "purpose": "Show local status at a desk",
  "target_users": ["Desk users"],
  "scenarios": ["Read status without opening a computer"],
  "bom": [{"item": "Selected board", "quantity": 1, "source": null}]
}
```

Only these five brief keys are accepted. Name is nonempty text; purpose is nonempty text or null. Target users and scenarios are lists of nonempty strings. BOM entries require item (nonempty text), quantity (positive integer or null) and source (nonempty text or null). Boolean quantities are invalid. Omitted fields use the selected Device ID as the name, null purpose and empty lists. Empty lists mean unknown, not a verified absence of requirements. Keep credentials outside the brief.

```bash
codecraft new --target esp-idf/reterminal-e1001 --example epd_hello --output projects/desk-display --brief projects/desk-display.brief.json
codecraft verify-project projects/desk-display
```

The native application and BSP retain their original paths inside the export. The export contains native inputs, portable guidance and transitive dependencies. The standalone CLI is installed separately and its binary identity is locked. This behavior is shared by ESP-IDF, Arduino and Linux without replacing their native build procedures. No toolchain version or board revision is inferred from the workspace path. Building requires the Target's actual declared toolchain.

## Project context and reviewed updates

`project.yaml` owns the brief. Creation and `codecraft lock-project <project>` render it into the marked CODECRAFT_APPLICATION region of application AGENTS.md. Custom rules outside that region remain intact, including schema 4 applications without markers. Missing brief values remain unknown. A malformed marker pair blocks relocking instead of discarding user rules.

After reviewing application or brief changes, activate the declared toolchain and run:

```bash
codecraft lock-project projects/desk-display --toolchain
codecraft verify-project projects/desk-display
codecraft build-project projects/desk-display
```

The lock preserves initial import provenance separately from the current application inventory. Brief and generated context changes are locked; source drift requires deliberate review and relocking. An unavailable toolchain remains not_run, not proof of a successful build. Exporting, locking, building and packaging never authorize device writes. Project profiles and locks use schema 4 with explicit Skill selections. The installed CLI identity is recorded separately from application sources; the current CLI does not accept older profile/lock formats. Native BSPs, dependencies and toolchain requirements are unchanged.

For reference changes, use [CONTRIBUTION.md](../../CONTRIBUTION.md). Remaining V0.2 cloud, simulator, asset and physical acceptance work is tracked in the [implementation ledger](v0.2-todo.md).
