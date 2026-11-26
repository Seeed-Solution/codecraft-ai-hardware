# ir_nec_sender — Agent Notes

## Purpose

Send three NEC frames through the onboard 940 nm infrared emitter without an
external IR library. It demonstrates the active-high drive, 38 kHz carrier and
NEC address/command framing.

## Hardware facts

- `WIO_IR` is GPIO 14, active high through Q13 to onboard 940 nm LED D10.
  Source: schematic v1.2 sheet 05 and core 1.8.6
  `variants/wio_terminal/variant.h`; see [`../../README.md`](../../README.md).
- The product page identifies the emitter wavelength as 940 nm.
- The emitter is on the back of the enclosure, to the left of the microSD
  card slot. Source: Seeed wiki, Getting Started with Infrared Emitter.
- The sketch bit-bangs a 38 kHz carrier by toggling GPIO 14 every 13
  microseconds. It sends address `0x00`, inverted address, command, inverted
  command, then a final mark.
- Buttons: A=GPIO 28, B=29, C=30, active low. A sends command `0x45`, B `0x10`
  and C `0x02`.

## Build

From the repository root, run `codecraft validate --target arduino/wio-terminal --example ir_nec_sender`. The exact core and libraries are declared in target.json.

## Expected behavior

- LCD title: `IR NEC SENDER`; it shows `WIO_IR = GPIO 14 / 940 nm`, the NEC
  timing summary and command map.
- Pressing A/B/C changes the last-frame label to `A: cmd 0x45`, `B: cmd 0x10`
  or `C: cmd 0x02` and prints `sending NEC addr=0x00 cmd=45` (or the matching
  command in uppercase hex).
- GPIO 14 is left low when no frame is being sent.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/wio-terminal/observations.md); a successful build does not certify physical behavior.

## Limits

- Transmit-only; there is no IR receiver or NEC decoder in this sketch.
- The frame is bit-banged and blocks the CPU for tens of milliseconds; the
  exact duration depends on the command's mix of 0 and 1 bits. Do not combine
  it with tight real-time tasks without restructuring the timing.
- The 13 microsecond half-period is an approximation generated in software;
  receiver tolerance and CPU load affect the actual carrier.
- No receiver decode was performed; the current on-device evidence is camera
  observation of the emitter only.
