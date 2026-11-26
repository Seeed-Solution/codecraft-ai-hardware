"""Local companion operations for Wio Tracker L2 running Meshtastic (Python client 2.7.11)."""
from contextlib import contextmanager


@contextmanager
def connect(transport, endpoint):
    """Open the selected endpoint, check the peer's model and release it on exit."""
    if not endpoint or not endpoint.strip():
        raise ValueError("Supply an explicit device endpoint")
    if transport == "usb":
        from meshtastic.serial_interface import SerialInterface
        interface = SerialInterface(devPath=endpoint)
    elif transport == "ble":
        from meshtastic.ble_interface import BLEInterface
        interface = BLEInterface(address=endpoint)
    elif transport == "tcp":
        from meshtastic.tcp_interface import TCPInterface
        interface = TCPInterface(hostname=endpoint)
    else:
        raise ValueError("Choose usb, ble or tcp")

    with interface:
        user = interface.getMyUser() or {}
        # meshtastic.protobuf.mesh_pb2.HardwareModel, Meshtastic Python 2.7.11.
        if user.get("hwModel") != "SEEED_WIO_TRACKER_L2":
            raise ValueError("The endpoint did not identify as Wio Tracker L2")
        yield interface.localNode


def _section(node, name):
    if node.localConfig is None or not node.localConfig.HasField(name):
        raise ValueError(f"The device did not supply its {name} configuration")
    return getattr(node.localConfig, name)


@contextmanager
def _edit(node, name):
    section = _section(node, name)
    original = type(section)()
    original.CopyFrom(section)
    try:
        yield section
        node.writeConfig(name)
    except Exception:
        section.CopyFrom(original)
        raise


def region(node):
    """Return the firmware region name, including UNSET on an unconfigured unit."""
    lora = _section(node, "lora")
    return lora.RegionCode.Name(lora.region)


def select_region(node, deployment_region):
    """Apply a Seeed-documented L2 profile. Reconnect to verify persistence."""
    # See the L2 getting-started guide, Region List. Other SKUs need review.
    if deployment_region not in ("US", "EU_868"):
        raise ValueError("Choose the documented profile for this deployment: US or EU_868")
    lora = _section(node, "lora")
    if lora.override_frequency or lora.override_duty_cycle:
        raise ValueError("Review the existing custom radio overrides before selecting a region")
    selected = lora.RegionCode.Value(deployment_region)
    if lora.region != selected:
        with _edit(node, "lora") as config:
            config.region = selected


def join_wifi(node, ssid, password):
    """Apply operator-supplied Wi-Fi credentials via USB. Verify the IP after restart."""
    if not 1 <= len(ssid.encode("utf-8")) <= 32:
        raise ValueError("SSID must be 1 to 32 bytes")
    # L2 personal Wi-Fi workflow uses an 8–63 byte passphrase.
    if not 8 <= len(password.encode("utf-8")) <= 63:
        raise ValueError("Expected an 8 to 63 byte Wi-Fi passphrase")
    with _edit(node, "network") as network:
        network.wifi_ssid = ssid
        network.wifi_psk = password
        network.wifi_enabled = True
