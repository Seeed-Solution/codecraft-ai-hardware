"""Check L2 peer selection and protection against incomplete config writes."""
import importlib.util
from pathlib import Path
from types import SimpleNamespace
import unittest
from unittest.mock import MagicMock, patch


path = Path(__file__).resolve().parents[4] / "devices/wio-tracker-l2/targets/clients/l2_meshtastic.py"
spec = importlib.util.spec_from_file_location("l2_meshtastic", path)
client = importlib.util.module_from_spec(spec)
spec.loader.exec_module(client)


class CompanionTests(unittest.TestCase):
    def test_different_board_releases_transport(self):
        interface = MagicMock()
        interface.getMyUser.return_value = {"hwModel": "SEEED_WIO_TRACKER_L1"}
        serial = SimpleNamespace(SerialInterface=MagicMock(return_value=interface))
        with patch.dict("sys.modules", {"meshtastic.serial_interface": serial}):
            with self.assertRaisesRegex(ValueError, "Wio Tracker L2"):
                with client.connect("usb", "l2-port"):
                    self.fail("Different board exposed to the caller")
        serial.SerialInterface.assert_called_once_with(devPath="l2-port")
        interface.__exit__.assert_called_once()

    def test_missing_network_config_is_never_overwritten(self):
        node = MagicMock()
        node.localConfig.HasField.return_value = False
        with self.assertRaisesRegex(ValueError, "network configuration"):
            client.join_wifi(node, "lab", "test-only-passphrase")
        node.writeConfig.assert_not_called()

    def test_unsupported_region_and_radio_override_preserve_settings(self):
        node = MagicMock()
        with self.assertRaises(ValueError):
            client.select_region(node, "EU_433")
        node.localConfig.lora.override_frequency = 900.0
        with self.assertRaisesRegex(ValueError, "overrides"):
            client.select_region(node, "US")
        node.writeConfig.assert_not_called()

    def test_wifi_keeps_other_settings_and_restores_cache_on_write_error(self):
        class Network(SimpleNamespace):
            def CopyFrom(self, other):
                self.__dict__.clear()
                self.__dict__.update(vars(other))

        network = Network(wifi_enabled=False, wifi_ssid="previous", ntp_server="local")
        node = MagicMock()
        node.localConfig.network = network
        node.writeConfig.side_effect = OSError("disconnected")
        with self.assertRaises(OSError):
            client.join_wifi(node, "lab", "test-only-passphrase")
        self.assertEqual(vars(network), dict(wifi_enabled=False, wifi_ssid="previous", ntp_server="local"))
        node.writeConfig.side_effect = None
        client.join_wifi(node, "lab", "test-only-passphrase")
        node.writeConfig.assert_called_with("network")
        self.assertEqual(network.ntp_server, "local")
        self.assertTrue(network.wifi_enabled)
        self.assertEqual(network.wifi_ssid, "lab")
