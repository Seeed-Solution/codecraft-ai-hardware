# Wio Tracker L2 pins and signals

The assignments below follow Seeed's [Pin List][pin-list] and its [hardware block diagram][diagram]. Numbers are ESP32-S3 GPIOs unless prefixed with `EXP_`, which denotes an I/O expander line. The native Wi-Fi scan uses the integrated radio and requires no application GPIO setup.

| Function | Bus | Signals | Constraints |
| --- | --- | --- | --- |
| IPS display | QSPI | CLK 42, D0–D3 41/40/39/38, CS 46, enable EXP_P05, reset EXP_P06 | Select the driver for the installed panel. The Wiki marks TE as unconnected. |
| E-paper display option | SPI | SCK 42, MOSI 41, CS 40, DC 39, BUSY 38 | Shares the panel connector and signals with the IPS option. |
| Board control | I²C | SCL 48, SDA 47 | Expander lines below are accessed through the expander. |
| SX1262 LoRa | SPI | CS 21, SCK 4, MISO 5, MOSI 6, RESET 7, BUSY 8, DIO1 9 | BUSY and DIO1 are radio outputs. |
| Grove expansion | External connector | DAT1 14, DAT2 13, enable EXP_P07 | The block diagram shows the port supply gated by GROVE_EN. |
| L76K GNSS | UART and expander | GNSS_RX 18, GNSS_TX 17, enable EXP_P15, reset EXP_P11 | RX/TX labels follow the vendor table. |
| Audio codec | I²S | MCLK 10, LRCK 12, SCLK 11, DSDIN 16, SDOUT 15 | Data signal names follow the codec diagram. |
| microSD | SD | CMD 3, CLK 2, DAT0 1, detect EXP_P02 | The documented connection is one data bit wide. |
| USB-C | USB | D− 19, D+ 20 | Used by the native USB console. |
| Status and controls | GPIO and expander | Mesh LED 43, user LED EXP_P12, wake button EXP_P00 | Seeed also lists user/boot, reset and power controls. |

At reset the ESP32-S3 samples GPIO0. High selects normal flash boot. For the serial bootloader, GPIO0 must be low and GPIO46 low or floating. GPIO46 also serves as display CS after boot. Keep external circuitry from forcing the opposite strap state during reset. The pin list does not establish every board enable polarity or startup delay. Check the fitted hardware revision before implementing a peripheral driver. [Espressif boot mode selection][boot]

[pin-list]: https://wiki.seeedstudio.com/meshtastic_wio_tracker_l2_intro/#pin-list
[diagram]: https://wiki.seeedstudio.com/meshtastic_wio_tracker_l2_intro/#hardware-overview
[boot]: https://docs.espressif.com/projects/esptool/en/latest/esp32s3/advanced-topics/boot-mode-selection.html
