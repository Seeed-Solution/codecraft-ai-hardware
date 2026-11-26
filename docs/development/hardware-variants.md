# Sourced hardware variants and native build selection

This guide covers the software portion of OPT-06. A panel or assembly choice
does not identify a physical PCB revision. Current Indicator GX/DX profiles
are based on [pinned vendor evidence](../../devices/sensecap-indicator/AGENTS.md).
Revision-specific compatibility and physical acceptance remain unknown.

## Ownership and selection

Device variants own sourced, typed hardware facts, including an explicit
compatible_revisions fact. Unknown compatibility uses a null value and reason;
it is never a guessed revision list. Common Device facts must not describe one
variant's controller as a fact shared by every assembly.

Target variants bind Device variant identities to native input/dependency
declarations and configuration. The default_variant field describes a build
default only. Indicator declares GX as its build default; new
applications should select the assembly identified from attributable evidence:

```sh
codecraft context --target esp-idf/sensecap-indicator --example display_color_bars --variant dx
codecraft new --target esp-idf/sensecap-indicator --example display_color_bars --variant dx --output /path/to/new-project
codecraft validate --target esp-idf/sensecap-indicator --variant dx
```

Omitting --variant from new selects the documented default and records that
choice explicitly. It does not claim physical compatibility. Unknown variant
names fail, including a variant supplied to a Target without declarations.
Context without a variant/example describes the complete Target; example
context projects the selected or default variant.

## Native build and export contract

The full Target gate builds every example for every declared variant, using
distinct output directories and case IDs ending in the variant name. A focused
--variant run covers only that choice and cannot certify the entire Target.
Complete validation also builds a clean exported project for each variant.
The aggregate project result retains each variant's source digest, native
result and log; omitting one variant cannot certify the Target build.

ESP-IDF variant profiles are native CMake files under the Target's variants/
directory. The current profile format declares Boolean CODECRAFT_* compile
definitions; other configuration shapes require an explicit platform extension.
The native router reads the Target default unless CMake receives
`-DCODECRAFT_HARDWARE_VARIANT=<id>`. Public builds always pass the selection.
After a compiler success, the adapter verifies the CMake cache and every BSP
compile command against the selected profile. It retains actual definitions in
the build result alongside source/requirement composition.

An export includes only the selected variant's native inputs and projects its
Target manifest to that choice. project.yaml, codecraft.lock and immutable
import provenance all identify it. The projected native default agrees with
direct ESP-IDF builds, while build-project checks the lock and actual toolchain.
Editing the profile invalidates the lock. Changing only a selection label does
not import another implementation and is rejected even during relocking.
To change assembly in an application, import the sourced implementation and
metadata together, review the application changes and explicitly relock.

Device variant facts follow [Device document rules](device-documents.md); the
parsed model exposes a variants map, with an omitted Markdown section meaning no
variants. Target manifests use schema_version 3 and require an explicit variants
object, empty when none exist. Platform metadata, adapter envelopes, project
profiles and locks have independent version numbers. Source changes invalidate
matching locks and build evidence.

## Physical evidence and remaining acceptance

Current physical records for a variant Target must name their variant and
observed revision. A known incompatible revision is rejected. An unknown
revision mapping cannot promote physical support, even if functional cases
are recorded as passed. Compatible observations are aggregated independently
per variant; one assembly's results cannot certify another. Aggregate Target
physical support requires the required cases for every registered variant.

Current GX/DX compatible revision lists are unknown and no physical observations
have been made. OPT-06 remains open until attributable revision information,
selection/compatibility evidence and revision-specific observations satisfy its
acceptance criteria. Compile checks and synthetic evidence fixtures are not
those observations. The source/lock/evidence rules preserve this distinction.
