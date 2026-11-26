# sd_card_log — Agent Notes

## Purpose

Mount the dedicated microSD slot, list the root directory and append one line
to `/wio_terminal.log`. It verifies the SD card pin mapping and filesystem
basics without touching the LCD bus.

## Hardware facts

- microSD socket ST-TF-003J uses SPI2: SCK 63, MOSI 62, MISO 61, CS 64;
  card-detect is GPIO 65. Source: schematic v1.2 sheet 02 and core 1.8.6
  `variants/wio_terminal/variant.h` (`SDCARD_*`); see
  [`../../README.md`](../../README.md).
- The product page specifies a maximum card size of 16 GB. The sketch expects
  a FAT32-formatted card.
- The sketch uses Seeed Arduino FS 2.1.5. `SD.begin()` is called with
  `SDCARD_SS_PIN`, `SDCARD_SPI` (SPI2) and 4 MHz. Do not substitute the
  standard Arduino `SD` library: its `begin()` without a CS pin defaults to
  `SS` (GPIO 50), which is not the microSD chip select.
- In Seeed Arduino FS 2.1.5, `FILE_WRITE` expands to
  `FA_CREATE_ALWAYS | FA_WRITE | FA_READ` and truncates an existing file.
  Use `FILE_APPEND` for append semantics. The sketch verifies this by
  reopening the log and printing its byte count and last line.

## Build

From the repository root, run `codecraft validate --target arduino/wio-terminal --example sd_card_log`. The exact core and libraries are declared in target.json.

## Expected behavior

- LCD title: `MICROSD CARD`; it shows the SPI2 pin map and status.
- Serial starts with `sd_card_log boot`. The sketch waits up to five seconds
  for a host to open the CDC serial port so the one-shot mount and root-list
  messages are not missed.
- On a successful mount, serial prints `SD mounted`, up to six root entries,
  `appended /wio_terminal.log`, then `log_bytes=... last_line=...` from a
  read-back of the file. Root entries are printed as basenames because the
  library's `File::name()` returns an internal `0://...` path for the root.
- The LCD status turns green and lists the same root entries; the log path is
  shown at the bottom.
- With no card or an unreadable card, it prints `SD mount failed; retrying`
  every two seconds and keeps the retry loop running. A failed mount calls
  `SD.end()` so failed attempts do not accumulate FatFs drive instances.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/wio-terminal/observations.md); a successful build does not certify physical behavior.

## Limits

- The sketch mounts once and has no hot-swap/remount handling; after a failed
  mount it retries, but after success removal is not detected.
- Card-detect GPIO 65 is documented but not used by this demo.
- The root listing is capped at six entries and this is not a file-copy demo.
- The tested card's capacity and physical filesystem variant were not measured;
  the demo only proves that this card mounted and its log was appended and read
  back. Capacity remains bounded by the documented 16 GB support limit.
