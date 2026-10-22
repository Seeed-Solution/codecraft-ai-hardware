#!/usr/bin/env python3
# Copyright (c) 2026 CodeCraft AI Hardware contributors.
# SPDX-License-Identifier: Apache-2.0
"""Blink an LED on the reTerminal 40-pin header using libgpiod v2.

Demonstrates the repository rule for Linux GPIO: always address a line by
(gpiochip, line offset) — never by a global/sysfs number. BCM4 is used
here (physical pin 7), a header pin the vendor overlay leaves free.

Wire an LED + series resistor between physical pin 7 (BCM4) and GND.

Requires: python3-libgpiod (gpiod 2.x, Raspberry Pi OS trixie).
Usage: python3 header_gpio_blink.py
"""

import time

import gpiod
from gpiod.line import Direction, Value

CHIP = "/dev/gpiochip0"  # SoC GPIO controller on the CM4
LINE = 4                 # BCM4 = physical pin 7; chip/line pair, not global


def main():
    with gpiod.request_lines(
        CHIP,
        consumer="reterminal-blink",
        config={LINE: gpiod.LineSettings(direction=Direction.OUTPUT,
                                         output_value=Value.INACTIVE)},
    ) as request:
        print(f"blinking {CHIP} line {LINE} (BCM4, physical pin 7); Ctrl-C to stop")
        try:
            for _ in range(10):
                request.set_value(LINE, Value.ACTIVE)
                time.sleep(0.25)
                request.set_value(LINE, Value.INACTIVE)
                time.sleep(0.25)
        except KeyboardInterrupt:
            pass
        finally:
            request.set_value(LINE, Value.INACTIVE)
    print("done; line released")


if __name__ == "__main__":
    main()
