<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/ir_nec_sender
```

No external library is required. See the board [AGENTS.md](../../AGENTS.md)
for authorized upload and setup.

## Expected behavior

- LCD title: `IR NEC SENDER`; it shows `WIO_IR = GPIO 14 / 940 nm`, the NEC
  timing summary and command map.
- Pressing A/B/C changes the last-frame label to `A: cmd 0x45`, `B: cmd 0x10`
  or `C: cmd 0x02` and prints `sending NEC addr=0x00 cmd=45` (or the matching
  command in uppercase hex).
- GPIO 14 is left low when no frame is being sent.

## On-device verification

1. Build and upload with explicit authorization; open serial at 115200 baud.
2. Point the **back** of the board (emitter left of the microSD slot)
   directly at a known 38 kHz demodulating receiver. A phone camera may show
   only a faint pulse and is not proof of a valid NEC frame; iPhone rear
   cameras often filter 940 nm. Use a TV remote as a camera control.
3. Press A, B and C one at a time and confirm the receiver decodes address
   `0x00` and commands `0x45`, `0x10`, `0x02` respectively.
4. Confirm the receiver does not decode a frame when no button is pressed.
5. If no frame is decoded, check line of sight, receiver supply and the GPIO
   14 drive before changing the NEC timings.

Verified on 2026-10-28: the user observed the 940 nm emission on an iPhone
camera after pointing it at the back emitter, left of the microSD slot. This
proves LED activity, not NEC protocol validity; decoding with a 38 kHz
receiver remains pending.

## Limits

- Transmit-only; there is no IR receiver or NEC decoder in this sketch.
- The frame is bit-banged and blocks the CPU for tens of milliseconds; the
  exact duration depends on the command's mix of 0 and 1 bits. Do not combine
  it with tight real-time tasks without restructuring the timing.
- The 13 microsecond half-period is an approximation generated in software;
  receiver tolerance and CPU load affect the actual carrier.
- No receiver decode was performed; the current on-device evidence is camera
  observation of the emitter only.
