English · [简体中文](README.zh_CN.md)

# Wio Tracker L2 native target

Activate ESP-IDF 6.0.2 for ESP32-S3. From the repository root, export the
standalone example and build it with its source lock.

```sh
python3 codecraft.py new --target esp-idf/wio-tracker-l2 --example wifi_scan --output projects/l2-scan
python3 codecraft.py build-project projects/l2-scan
```

The example initializes a station, scans once, reports the AP count over USB
Serial/JTAG and releases the station. Zero is a valid result. It uses the default
ESP-IDF partition layout. Flashing this standalone application replaces the
installed application and may overwrite existing firmware data. For operations
on an L2 running Meshtastic, use the [device guide](../reference/meshtastic.md).
