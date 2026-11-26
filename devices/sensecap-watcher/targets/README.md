<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# esp-idf/sensecap-watcher

Device knowledge: [AGENTS.md](../AGENTS.md). Hardware facts are maintained there and in its references.

Build/check: `codecraft validate --target esp-idf/sensecap-watcher` from the repository root.

See `target.json` for examples, native interface references and exact toolchain selection. Build results and device observations are separate.

BSP public headers live in `bsp/include/`; implementation and private headers live in `bsp/src/`. Each example has its own `bsp.cmake` and managed dependency manifest. Exported projects include that example’s selected interfaces and source closure.

## Single-file image

After `codecraft validate --target esp-idf/sensecap-watcher --example battery_status`, run `codecraft firmware plan --target esp-idf/sensecap-watcher --example battery_status --output .codecraft/watcher-plan.json`, then `codecraft firmware merge --plan .codecraft/watcher-plan.json --output .codecraft/watcher-release` and `codecraft firmware verify .codecraft/watcher-release`. Keep the declared ESP-IDF environment activated. Outputs must be new. The factory BIN and manifest bind the successful build, native layout and input hashes. The workflow never flashes or uploads.

## Target implementation

Device behavior, reference interfaces and example purposes are documented in the [Device implementation references](../AGENTS.md#implementation-references).

The audio BSP uses esp_codec_dev 1.6.2 with native IDF I2C device handles.
The polling SPD2010 touch driver also uses raw native I2C transactions; the BSP
owns the bus, device handle, shared display rail and reset routing.

## Toolchain

The executable version declaration in `target.json` requires ESP-IDF 6.0.2 for esp32s3. Activate that SDK before building.

In an exported application, use `codecraft build-project .`; reference-repository validation commands do not replace application lock checks.
