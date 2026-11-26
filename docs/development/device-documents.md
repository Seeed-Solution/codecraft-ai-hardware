# Device documents and shared references

Each `devices/<device>/AGENTS.md` is authored English Markdown. It owns the
Device's knowledge, sourced specifications and acceptance procedures. The
reader produces a schema-5 Device model from explicit sections and tables;
it does not interpret arbitrary prose or ask an Agent to infer machine fields.
There is no embedded JSON, generated region or separate Device/acceptance file.

The host tools in [device.rs](../../tools/cli/src/device.rs) parse
this document; [maintenance_device.rs](../../tools/cli/src/maintenance_device.rs)
validates the resulting Device data, cited assertions, acceptance cases and
assets. Catalog loads this validator directly. Both modules belong to the
shared CLI implementation and are declared in its resource manifest so context,
export and project locks include the same validation code. Device directories
own authored knowledge and native implementation, not the host validation tools.
Citation checks establish consistency with the referenced text; they do not
establish hardware accuracy or replace revision-specific observations.

## One owner per piece of information

| Information | Maintained source | Other documents |
| --- | --- | --- |
| Repository rules and task routing | Root AGENTS.md | Link to the relevant section |
| Device knowledge, facts and acceptance procedures | Device AGENTS.md | Link to the Device or a stable heading |
| Native platform, interfaces, toolchain and build inputs | targets/target.json and native configuration | Reference the Target |
| Shared stack rules and components | platforms/<stack>/ | Target work follows that stack's AGENTS.md |
| Build instructions | Target README pair | Device entries link to it |
| Test observations and source-bound evidence | tests/devices/<device>/ | Link to retained observations; query current evidence through the CLI |
| Document navigation | docs/README.md | Link to the authoritative guide, not a copied rule set |

A link may appear at several entry points. Its destination has one maintained
owner. Use Markdown reference links to define a repeated destination once per
file. Definitions are local to a Markdown file; there is no global alias file or
custom include preprocessor. The documentation gate rejects duplicate labels,
undefined references and broken local destinations, including reference-style
links. Code examples are not navigation. Keep source and revision anchors on
citations where they identify the supporting evidence.

```markdown
See the [native build instructions][target-guide] and use the
[Target's build procedure][target-guide] for the selected example.

[target-guide]: targets/README.md
```

## Device format

The single level-one heading is the device name. These level-two sections are
required exactly once:

- `Overview`, `Usage`, `Physical layout`: ordinary English prose.
- `Profile`: a `Field | Value` table with ID, Format (5), Revision, Maintenance
  and Maturity. Unknown revisions are written as `unknown`.
- `Specifications`: level-three Controller, Connectivity, Inputs, Outputs,
  Power and Accessories tables. Every category has at least one documented,
  measured or explicitly unknown fact.
- `Limitations`: a Markdown list, or `None recorded.`.
- `Physical acceptance`: Preconditions followed by one level-three heading
  per stable case ID; procedures are plans, never observed results.
- `Sources`: a list of named reference links declaring the source closure.
  Local files belong to the repository; external vendor links use HTTPS.

`Hardware notes`, `Implementation references` and additional task-specific
sections contain independently authored guidance. The parser preserves those
sections. `Hardware variants`, `Source conflicts` and `Assets` are optional when
there are no corresponding entries. Do not create empty placeholder folders.

Specification tables use these columns:

```markdown
| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Flash capacity (`flash`) | integer | 32 | MB | documented | [pins][pins] | FLASH_MB {value} |
| Accessories (`accessories`) | string | — | — | unknown | — | No independently sourced specification. |

[pins]: targets/bsp/include/bsp_pins.h
```

The example values illustrate syntax and do not establish facts about a board.
Types are `string`, `integer`, `number`, `boolean` or `strings` (a JSON string
array in a table cell). Numeric values require units; booleans use true/false.
Unknown facts use an em dash for value and source, plus a reason. A known fact
has a named local source and an assertion containing exactly one `{value}`
placeholder. Its rendered assertion must occur in that source. When citing the
Device document itself, only independently authored notes can substantiate the
fact; specification and acceptance fields cannot certify themselves.

Escape table pipes as `&#124;`; other Markdown formatting remains readable.
Stable fact IDs are lower-case identifiers inside the label's parentheses.
Duplicate IDs, wrong table columns, invalid scalar values and missing sections
fail rather than falling back to another format.

A physical acceptance case has this shape:

```markdown
### display-colors

- Interfaces: display
- Source: [target][target]

Run the selected example and record the observed colors on the identified unit.

Expected: The displayed order matches the example's documented sequence.

[target]: targets/target.json
```

Use `none` for a case without native interfaces. Preconditions state device
identity, revision, authorized write scope and necessary peripherals. Preserve
actual procedures and expected observations when reorganizing a document.

Hardware variants use `### <id>: <label>`, `Source: [label][reference]`, then
level-four specification categories with the same fact tables. They require a
sourced or explicitly unknown `compatible_revisions` string-list fact. Source
conflicts use `Source | Resolution`. Real registered PNG assets use
`File | Origin | License | SHA256 | Revision | Width | Height | Alpha` and retain
provenance, checksum, revision and image-dimension validation.

## Generation, export and evidence

`codecraft maintenance generate` updates the device directory, source navigation,
the [native interface inventory](../inventory.md) and its
[JSON bindings](../interface-matrix.json), the Capability matrix, and the CI
matrix. The native interface views are derived from Target declarations and
source-bound evidence; they are not runtime inputs or application export
dependencies. Native discovery reads the owning Target manifests.
Generation never edits authored
Device documents or the root rules. Their links route to those derived indexes.
The shared evidence reader computes verification state from current source-bound
records for the native interface matrix; it is not copied into Device documents.
`codecraft context` derives support from current source/CLI-bound evidence.
Historical graph records remain archived without promotion; see
[evidence compatibility](rust-cli.md#evidence-compatibility).

Creator source graphs and locks use schema 4 and hash complete source files,
including each authored AGENTS.md. Historical records retain their original identity.
Creator export copies the Device document byte for byte, follows its declared
source closure and locks the complete files. No document-content projection or
export-time rewrite is used. A changed procedure or hardware note changes source
identity; old observations and logs retain their original source digest.

Run the relevant Markdown/parser, reference and export/lock checks after changes.
Native builds and physical acceptance remain separate checks under the
[repository contract][rules] and [testing guide][testing].

[rules]: ../../AGENTS.md
[testing]: testing.md
