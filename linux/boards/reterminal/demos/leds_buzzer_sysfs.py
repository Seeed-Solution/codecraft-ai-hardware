#!/usr/bin/env python3
# Copyright (c) 2026 CodeCraft AI Hardware contributors.
# SPDX-License-Identifier: Apache-2.0
"""reTerminal user LEDs + buzzer over the kernel LED subsystem.

With the peripherals overlay loaded, the MCP23008 expander lines appear as
/sys/class/leds/usr_led{0,1,2} (active low) and /sys/class/leds/usr_buzzer
(active high). Writing 1 turns the LED on / sounds the buzzer; 0 is off.

The demo blinks each LED and beeps the buzzer, then restores everything to
off — also on SIGINT/SIGTERM.

Usage: python3 leds_buzzer_sysfs.py
"""

import signal
import sys
import time

LEDS = ["usr_led0", "usr_led1", "usr_led2"]
BUZZER = "usr_buzzer"
SYSFS = "/sys/class/leds"


def write(led, value):
    with open(f"{SYSFS}/{led}/brightness", "w", encoding="utf-8") as f:
        f.write(str(value))


def all_off():
    for led in LEDS + [BUZZER]:
        try:
            write(led, 0)
        except OSError:
            pass


def main():
    try:
        for led in LEDS:
            write(led, 0)
        write(BUZZER, 0)
    except OSError as e:
        sys.exit(
            f"cannot access {SYSFS}: {e}. Is the peripherals overlay loaded? "
            "See ../README.md."
        )

    def _stop(_signum, _frame):
        all_off()
        sys.exit(0)

    signal.signal(signal.SIGINT, _stop)
    signal.signal(signal.SIGTERM, _stop)

    print("blinking usr_led0..2, then a short beep (Ctrl-C to stop, LEDs off on exit)")
    try:
        for cycle in range(3):
            for led in LEDS:
                write(led, 1)
                time.sleep(0.15)
                write(led, 0)
        write(BUZZER, 1)
        time.sleep(0.1)
        write(BUZZER, 0)
    finally:
        all_off()
    print("done; all LEDs and buzzer off")


if __name__ == "__main__":
    main()
