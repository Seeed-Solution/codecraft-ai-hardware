# Use CodeCraft with a coding agent

Codex, Claude Code, Cursor, ZCode, WorkBuddy, Muse and other general-purpose agents
use the same CodeCraft development Skills, project files and CLI workflow.
Choose an entry method your client provides. There is no client-specific build
implementation, mandatory plugin, MCP server or subagent arrangement.

Client experience testing is left to users. This guide supplies common entry
instructions rather than a client certification matrix.

## Prepare the workspace

For a new application, open a CodeCraft reference checkout containing the board
catalog. For an existing exported application, open that application's root.
Use [CLI installation](rust-cli.md) if CodeCraft is missing; the default installer
selects the latest formal Release, while an explicit version remains available.
Native SDKs are needed when preparing a build, not just to discuss an application.

The required development workflow packages remain develop, setup, build,
device-test and debug under skills/codecraft-development-*. Ideate, discover and
validate support their matching tasks. Follow the workspace's AGENTS.md for
availability and installation requirements. Install complete package directories
using the client's documented mechanism when it supports native Skills. Do not
copy an isolated SKILL.md and assume its workspace references become package-local.

Keep the hardware checkout or exported application accessible separately from
the client's installed Skills. These workflows use existing workspace documents;
they do not bundle a private copy of the board catalog. ReCamera SDK and cloud
capability packages retain their own independent usage instructions.

## Choose an entry method

| Client facility | Entry |
| --- | --- |
| Native Skill discovery | Select codecraft-development-develop, or describe the application task so the client can select it |
| Project instructions or rules | Add a short pointer to the workspace's existing Skill and workflow, using that client's documented rule format |
| Explicit file reading | Give the agent the workspace path and the reusable request below |

These are alternatives, not steps that must all be configured. A client-specific
rule only needs to point to the common workflow. Do not duplicate device facts,
build commands or contribution rules. Preserve existing user instructions; do
not replace a whole rule file to add CodeCraft. No client settings are changed by
reading this guide. Skill names in a request do not imply identical slash-command
or tool syntax across clients.

## Reusable request

Replace the workspace path and application request. For an existing application,
provide its root rather than the original reference checkout.

```text
Use CodeCraft to develop my application in <absolute workspace path>.
Read that workspace's AGENTS.md, then
skills/codecraft-development-develop/SKILL.md and
docs/development/application-workflow.md.
Follow their task routing and read additional guidance only when needed.
Preserve existing project choices, custom rules and my changes.
My application request: <describe the feature or application>.
```

The same text can be used as a project-rule pointer after removing the final
request line. Keep paths relative to the chosen workspace where the client
supports that; otherwise use its explicit absolute path. Do not hardcode the
location of another user's installation.

For example, a new application request can specify a selected E1001 and a desired
display interaction. An existing application request can ask to change a button
handler without reselecting a board. If the board is undecided, state the purpose
and known constraints; unknown accessories or revisions remain unknown.

## Continue development

The [portable application workflow](application-workflow.md) is the shared source
for workspace resolution, device context, application creation, locks, builds and
device-operation boundaries. The agent should reuse the current Target/example,
installed toolchain and task authorization where applicable.

Project intent, reviewed sources and build outputs stay in the existing project
files and records. Switching clients does not require a new profile, schema,
export or lock reset. A new agent can read the same project without the previous
chat transcript. Access to an actual device and a successful physical observation
remain separate from editing or compiling code.

If your client cannot follow a Skill link, point it directly at the corresponding
document in the workspace. If it cannot execute commands, use it for planning or
editing and run the commands in your own terminal; do not report an unexecuted
build as successful. If a specific client integration causes trouble, optionally
report the product/version, environment and concrete issue. There is no required
client test report or compatibility registration before using the workflow.
