# Linux Platform Rules

Applies to everything under `linux/`. The root `AGENTS.md` governs first;
this file adds platform-specific rules.

## Scope

Linux boards here are Seeed Linux devices (reTerminal on CM4, reComputer
series). Support consists of:

- device tree overlays (from or aligned with
  [Seeed-Studio/seeed-linux-dtoverlays](https://github.com/Seeed-Studio/seeed-linux-dtoverlays)),
- GPIO maps using libgpiod (no sysfs GPIO, no hardcoded global line numbers
  without the chip/line pair),
- Python or C demos that run unprivileged wherever possible; document any
  group membership or udev rule a demo needs.

## Rules

- Same hardware-fact priority as ESP32: vendor spec/measured first, never
  guess pins or peripheral numbering.
- Each board directory documents the target OS image and kernel version the
  support was verified on.
- Demos must clean up GPIO/pwmchip claims on exit, including on signals.
