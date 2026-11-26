---
name: codecraft-development-develop
description: Create and modify applications for supported Seeed devices. Use when choosing a board, starting a firmware project, adding device features, or continuing an existing application; also route explicit reference-repository contributions.
---

# Develop

Start from the user's repository or application workspace, not this Skill's
installation directory. Read that workspace's AGENTS.md and
`docs/development/application-workflow.md`. Links below refer to the matching
paths in that workspace when this Skill is installed elsewhere. If the workspace
is not available, ask for its location; continue independent application planning.

Use the [workspace entry and routing](../../docs/development/application-workflow.md#workspace-entry-and-routing).
Preserve an existing project's Target, example, variant, custom rules and user
changes. Do not restart device selection or create another project merely because
the user changes agent clients.

| Situation | Next action |
| --- | --- |
| Existing application | Read project.yaml and the current changes; obtain only the relevant Target/task context and implement the requested feature in that application |
| New application, device chosen | Preserve the selected software stack and sourced hardware revision; obtain context and select an existing example matching the requested functions |
| New application, device undecided | Use discover and, when needed, ideate; compare actual candidates and ask only for missing constraints that change the choice |
| Environment missing or mismatched | Use setup guidance; reuse a matching SDK and activate it in the build shell |
| Build requested or needed for changed native inputs | Use build guidance for this workspace; an application uses build-project rather than reference Target validation |
| Failure | Use debug guidance and the actual error; do not repeat an unchanged failed operation |
| Device write requested | Use device-test guidance with the identified board/port and existing valid session authorization |
| Reference repository contribution | Apply the root contribution routing table and affected owner rules; do not impose package-maintenance procedures on ordinary applications |

Commands use `codecraft <command>` from the selected root, or an explicit
`--root` for context/discovery. Run `context --target <id> --task <task>` with
`--example` and `--variant` when applicable. Read the returned Device, Target,
Platform and task sources that affect the requested behavior, rather than loading
all boards or every workflow. A context result is not hardware verification.

For new applications, use `new --target <id> --example <name> --output <new-path>`;
prefer ignored projects/<name> inside the reference checkout and support explicit
external paths. Add `--brief <file.json>` for a confirmed brief and `--variant`
when required. Keep project intent in project.yaml and generated rules in their
marked region; preserve custom rules outside it.

After reviewing intended application/environment changes, explicitly update the
project lock as appropriate and use `build-project <application-path>` for a build.
Preserve immutable import provenance. Reuse current task authorization rather
than asking again for each ordinary edit. A build does not authorize flashing,
full-chip erase, a cloud call or publication.

For inference features, read only the selected independent capability Skill and
contract.json; use explicit `--skills` selections. For SenseCraft platform or Hub
operations, follow the [external platform route](../../docs/development/application-workflow.md#external-platform-operations).
Keep application logic separate from native BSPs and preserve the selected
[composition](../../docs/development/application-workflow.md#native-composition)
and [hardware variant](../../docs/development/application-workflow.md#hardware-variants-and-evidence).
Finish with changed files, actual check/build results and any remaining task limits.
