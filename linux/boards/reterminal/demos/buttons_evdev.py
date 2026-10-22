#!/usr/bin/env python3
# Copyright (c) 2026 CodeCraft AI Hardware contributors.
# SPDX-License-Identifier: Apache-2.0
"""reTerminal button reader over evdev (no third-party dependencies).

With the peripherals overlay loaded, the four front buttons and the power
button appear as a "gpio-keys" input device. Key mapping (vendor fact):

    usr_btn0 (F1) -> KEY_A  (30)     usr_btn2 (F3) -> KEY_D  (32)
    usr_btn1 (F2) -> KEY_S  (31)     usr_btn3 (F4) -> KEY_F  (33)
    power button  -> KEY_SLEEP (142)

Usage: python3 buttons_evdev.py   (Ctrl-C to exit)
"""

import glob
import os
import struct
import sys

# struct input_event on 64-bit Linux: time (2x unsigned long), type, code, value
EVENT_FMT = "QQHHi"
EVENT_SIZE = struct.calcsize(EVENT_FMT)
EV_KEY = 0x01

KEY_NAMES = {
    30: "usr_btn0 (KEY_A)",
    31: "usr_btn1 (KEY_S)",
    32: "usr_btn2 (KEY_D)",
    33: "usr_btn3 (KEY_F)",
    142: "power_key (KEY_SLEEP)",
}


def find_keys_device():
    """Locate the gpio-keys event node via sysfs (stable across reboots)."""
    for name_path in sorted(glob.glob("/sys/class/input/event*/device/name")):
        try:
            with open(name_path, "r", encoding="utf-8") as f:
                if f.read().strip() == "gpio-keys":
                    event = os.path.basename(os.path.dirname(os.path.dirname(name_path)))
                    return os.path.join("/dev/input", event)
        except OSError:
            continue
    return None


def main():
    path = find_keys_device()
    if path is None:
        sys.exit(
            "gpio-keys device not found. Is the reTerminal peripherals overlay "
            "loaded? See ../README.md."
        )
    print(f"reading buttons from {path} (Ctrl-C to exit)")
    fd = os.open(path, os.O_RDONLY | os.O_NONBLOCK)
    try:
        import selectors

        sel = selectors.DefaultSelector()
        sel.register(fd, selectors.EVENT_READ)
        while True:
            for key, _ in sel.select():
                data = os.read(key.fd, EVENT_SIZE * 8)
                for off in range(0, len(data) - EVENT_SIZE + 1, EVENT_SIZE):
                    _sec, _usec, ev_type, code, value = struct.unpack_from(
                        EVENT_FMT, data, off
                    )
                    if ev_type != EV_KEY or code not in KEY_NAMES:
                        continue
                    action = {0: "released", 1: "pressed", 2: "autorepeat"}.get(
                        value, f"value={value}"
                    )
                    print(f"{KEY_NAMES[code]}: {action}")
    except KeyboardInterrupt:
        pass
    finally:
        os.close(fd)


if __name__ == "__main__":
    main()
