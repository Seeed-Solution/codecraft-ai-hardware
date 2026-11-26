# Connect to an L2 running Meshtastic

First identify the installed firmware, UI, physical unit and intended endpoint.
The ESP-IDF scan example runs independently. MeshCore uses a different companion
protocol. The following operations require Meshtastic firmware.

| Task | Existing module or interface | Input and observable result |
| --- | --- | --- |
| Join Wi-Fi | Meshtastic Network configuration in the L2 UI | Enter the SSID and password on the device, then enable Wi-Fi. Use a 2.4 GHz AP. The UI shows an assigned IP after association and DHCP. |
| Pair a phone | Meshtastic client Bluetooth transport | In MUI, tap Settings → Reboot/Shutdown → Bluetooth and wait for the pairing screen. In BaseUI, use Bluetooth with Wi-Fi off. Select the matching node in the app, then enter the code displayed on this unit. Read the node information after connecting. |
| Read node configuration | Meshtastic Client API and its serial, BLE or TCP transports | Use the confirmed USB port, BLE identifier or LAN IP. Use the same protobuf configuration fields over each transport. BaseUI with Wi-Fi enabled uses the Network connection in the app. |
| Select LoRa band | Meshtastic LoRa region setting | Select the region for the place of operation and the installed radio/antenna. US and EU_868 are documented examples. Leave UNSET until the deployment region is known. Read back the region after restart. |

For host automation, reuse the Meshtastic Python client. Its CLI provides the
same operations without an application-specific packet encoder. These examples
use CLI 2.7.11. Replace each shell variable with the confirmed endpoint or
region. The first three commands read one field over alternative transports.

```sh
meshtastic --port "$L2_PORT" --get lora.region
meshtastic --ble "$L2_BLE_ID" --get lora.region
meshtastic --host "$L2_IP" --get lora.region
```

For an authorized region change, use the USB connection so a wireless
reconfiguration does not interrupt the follow-up check.

```sh
meshtastic --port "$L2_PORT" --set lora.region "$DEPLOYMENT_REGION"
# After the unit restarts and the port reappears:
meshtastic --port "$L2_PORT" --get lora.region
```

Region configures the operating band and regulatory profile. Nodes also need
compatible modem presets and channel settings to exchange messages. Pairing
a phone verifies the local link only. For a mesh acceptance test, use a second
configured node and verify receipt of a test message.

[Seeed's L2 connection and configuration guide](https://wiki.seeedstudio.com/get_started_with_meshtastic_wio_tracker_l2/) ·
[Meshtastic Client API](https://meshtastic.org/docs/development/device/client-api/) ·
[Python client](https://github.com/meshtastic/python) ·
[Region by country](https://meshtastic.org/docs/configuration/region-by-country/)

## Reuse in a host application

The [L2 companion module](../targets/clients/l2_meshtastic.py) uses the
Meshtastic Python 2.7.11 transport and generated protobuf classes. Install
`meshtastic==2.7.11` in the host application environment. Import the module
from `targets/clients/`. It verifies the reported hardware model on every
connection. Select the physical L2 endpoint even when several L2 units are nearby.

```python
import os
from getpass import getpass
from l2_meshtastic import connect, join_wifi, region, select_region

with connect("usb", os.environ["L2_PORT"]) as node:
    print(region(node))
```

For an authorized change, call **one** operation on the connected node:
`select_region(node, deployment_region)` or
`join_wifi(node, ssid, getpass("Wi-Fi passphrase: "))`. Reconnect after the
firmware applies the setting. Verify the region with `region(node)` or the
assigned IP on the device. The Wi-Fi helper supports personal networks with a
passphrase. For BLE, complete the on-screen code exchange and use
`connect("ble", identifier)`; for LAN use `connect("tcp", ip_address)`.
