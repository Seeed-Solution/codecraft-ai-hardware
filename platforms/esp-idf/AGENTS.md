# esp-idf platform rules

The [root contract](../../AGENTS.md) governs. Device facts belong in devices/<device>/AGENTS.md and its reference files.

These rules also apply to devices/<device>/targets/. Keep each board's BSP, examples and sdkconfig in that Target. Use the release version in target.json and verify the activated SDK. The version matcher accepts that release and its `-dirty` working tree, but not prereleases, additional commits or longer version prefixes. Local SDK edits are allowed for development; the project lock records the SDK commit and tracked diff, so changed environments require deliberate relocking. Preserve LVGL locking and non-blocking callbacks. Shared components belong in platforms/esp-idf/components/; shared CMake helpers remain in platforms/esp-idf/cmake/. Board-specific tests and fixtures belong in tests/devices/<device>/targets/.

Every Target must declare examples, toolchain provenance, native interface references, and dependencies in target.json. Supported operations and their limits are in platform.json. Run `codecraft validate --target esp-idf/<device>`. Never infer device verification from a build.

BSP public contracts stay in include/; implementation/private headers belong in src/. Each example MUST declare its sources and requirements in bsp.cmake, optional managed packages in main/idf_component.yml, and matching Target example inputs/dependencies/interface scopes. Native component registration MUST remain in the board BSP CMakeLists.txt. Preserve required system components such as esp_psram when using MINIMAL_BUILD. Read [native composition rules](../../docs/development/application-workflow.md#native-composition); regenerate locks through the native tool before final source/evidence checks.

Declared hardware variants use native CMake profiles and distinct build cases/output directories. Verify actual cache selection and BSP compile definitions. Follow [hardware variant rules](../../docs/development/application-workflow.md#hardware-variants-and-evidence); keep physical revision compatibility separate from a build profile.

Factory image delivery uses the public firmware plan/merge/verify commands and
image.rs. Use only successful build receipts and native flasher metadata;
never hardcode a board's offsets in the Platform. Current support is unsigned,
unencrypted factory BIN. See [firmware image delivery](../../docs/development/application-workflow.md#firmware-images-and-device-operations).

For changes to the reference repository, also read the root routing entries for
docs/development/native-compositions.md, hardware-variants.md and firmware-images.md
as applicable. Their contribution and evidence requirements remain mandatory.
