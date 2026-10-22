#!/usr/bin/env python3
# Copyright (c) 2026 CodeCraft AI Hardware contributors.
# SPDX-License-Identifier: Apache-2.0
"""reTerminal on-board sensors over the IIO subsystem (no dependencies).

With the peripherals overlay loaded:
  - LTR-303ALS ambient light sensor (I2C1 @0x29) -> iio device "ltr303",
    lux in in_illuminance_input.
  - LIS331DLH 3-axis accelerometer (I2C1 @0x19) -> iio device "lis331dlh",
    raw axes in in_accel_{x,y,z}_raw with in_accel_scale.

Usage: python3 sensors_sysfs.py
"""

import glob
import os
import sys


def find_iio_device(name):
    """Return the sysfs directory of the iio device with this name."""
    for name_path in sorted(glob.glob("/sys/bus/iio/devices/iio:device*/name")):
        try:
            with open(name_path, "r", encoding="utf-8") as f:
                if f.read().strip() == name:
                    return os.path.dirname(name_path)
        except OSError:
            continue
    return None


def read_float(path):
    with open(path, "r", encoding="utf-8") as f:
        return float(f.read().strip())


def read_light_lux():
    dev = find_iio_device("ltr303")
    if dev is None:
        return None
    for candidate in ("in_illuminance_input", "in_illuminance0_input"):
        p = os.path.join(dev, candidate)
        if os.path.exists(p):
            return read_float(p)
    return None


def read_accel_ms2():
    dev = find_iio_device("lis331dlh")
    if dev is None:
        return None
    scale = read_float(os.path.join(dev, "in_accel_scale"))
    return [
        read_float(os.path.join(dev, f"in_accel_{axis}_raw")) * scale
        for axis in "xyz"
    ]


def main():
    missing = []

    lux = read_light_lux()
    if lux is None:
        missing.append("LTR-303 light sensor (iio 'ltr303')")
    else:
        print(f"ambient light: {lux:.1f} lux")

    accel = read_accel_ms2()
    if accel is None:
        missing.append("LIS331DLH accelerometer (iio 'lis331dlh')")
    else:
        print(f"acceleration: x={accel[0]:+.2f} y={accel[1]:+.2f} "
              f"z={accel[2]:+.2f} m/s^2")

    if missing:
        print("not found: " + "; ".join(missing), file=sys.stderr)
        print("Is the peripherals overlay loaded? See ../README.md.",
              file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
