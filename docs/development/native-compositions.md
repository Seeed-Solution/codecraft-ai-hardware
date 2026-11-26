# ESP-IDF BSP composition and layout

Each ESP-IDF example declares the native implementation it builds. These are
Target/BSP interfaces, not cloud Skill Capabilities. Device facts, pins and
hardware revisions retain their separate ownership.

```text
platforms/esp-idf/
  composition.rs                 # metadata/build-output checks
  cmake/bsp.cmake                 # load the example's native composition
  components/                    # native components shared across boards
devices/<board>/
  targets/
    target.json                  # common and per-example source/dependency scope
    bsp/
      CMakeLists.txt             # native component registration stays here
      idf_component.yml          # no unconditional optional managed packages
      include/                   # public headers and sourced pins
      src/                       # implementation and private headers
    examples/<example>/
      CMakeLists.txt             # minimal native build, selected local components
      bsp.cmake                  # selected BSP sources and native requirements
      sdkconfig.defaults
      dependencies.lock
      main/
        CMakeLists.txt           # application and required system components
        idf_component.yml        # this example's managed package dependencies
        main.c
tests/devices/<board>/targets/
  test_*.py                      # board-specific host contracts
  <suite>/                       # native fixtures and test doubles
```

## Native build contract

`bsp.cmake` declares two explicit lists: `CODECRAFT_BSP_SOURCES` uses paths
relative to the board BSP, such as `src/bsp_knob.c`; `CODECRAFT_BSP_REQUIRES`
lists its native component requirements. Shared board resources appear in
each composition that needs them. For example, Watcher knob input includes
the I/O expander, and Watcher touch includes display and expander support.
There is no implicit all-peripheral fallback.

The board's component-root `CMakeLists.txt` loads the shared helper, then calls
`idf_component_register` from the BSP directory. Public include visibility is
`include/`; private visibility is `src/`. Registering from the shared helper
instead would change ESP-IDF's relative-path context.

Projects enable `MINIMAL_BUILD`, name only selected local component directories,
and explicitly declare application requirements. PSRAM-enabled configurations
retain `esp_psram`; the Wi-Fi example explicitly retains networking and NVS.
These choices follow the [ESP-IDF v6.0.2 build-system contract](https://docs.espressif.com/projects/esp-idf/en/v6.0.2/esp32s3/api-guides/build-system.html), including CMake 3.22 or newer.

Optional managed packages belong to the example's `main/idf_component.yml`.
Version constraints and dependency locks are reviewed independently of directory
layout. Direct package versions and the resolved lock must agree; the selected
dependency graph must include only packages required by the example.
Use ESP-IDF Component Manager to update `dependencies.lock`; do not hand-edit
its resolution or hashes. A native configure step can change the lock, so
complete that step before freezing sources for final validation/evidence.

## Metadata, exports and checks

Target common inputs contain only common BSP build files and pins. Per-example
inputs contain the selected implementation and required public/private headers;
per-example dependencies contain its lock, managed manifest and selected shared
components. Existing `example_implementation` scopes expose only native
interfaces available in that example. Stable source assertions and operations
still point to the same public contracts.

Shared component `resource.json` files define their runtime source closure,
including native build files, headers, implementation and retained attribution.
Shared module tests and SDK doubles remain with their Platform or component;
board-specific host tests live in tests/devices/<board>/targets/.
Neither is implicitly shipped as runtime application inputs.

`composition.rs` checks profile syntax, local header/implementation closure,
Target declarations and native component-directory selection. After a successful
compiler command, the adapter checks `project_description.json` for the actual
BSP sources and requirements. A compiler exit of zero with the wrong composition
is a failed validation result. The actual component list is retained with each
native build result; registration counts alone do not establish firmware size,
build-time improvement or physical behavior.

Use `codecraft context --target <id> --example <name>` to inspect
the chosen closure, and `new --target <id> --example <name> --output <new-path>`
to export it. The export carries the native profile and CMake helper in its
source graph and lock; the composition validator belongs to the locked CLI binary. Missing inputs fail rather than being silently omitted.

When adding an interface to an external application, import the required native
sources, headers and dependencies, then update that application's composition,
Target example scope and native manifests together. Review and explicitly
relock the application before building. An unchanged original reference-project
lock is not approval for new sources or dependencies.

For changes to native composition, run the composition tests and affected native
and exported project builds:

```sh
cargo test --locked --package codecraft --test migration
codecraft validate --target esp-idf/<board> --projects
```

Narrow by example/variant where applicable. Documentation about composition
does not require firmware compilation. Select static checks and any broader
native scope using the [root validation policy](../../AGENTS.md#required-validation-and-delivery);
the complete gate is not a routine prerequisite for every repository change.

## Verification limits

An exported example includes its selected native composition. It does not acquire
all peripherals on the board. Build success and host fault tests do not establish
revision compatibility, combined-peripheral resource budgets or on-device
recovery. These acceptance items remain in the [open TODOs](optimization-plan.md).
