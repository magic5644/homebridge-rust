"""Fictional, offline checks for the owner-run read-only inventory helper."""
import importlib.util
import io
import json
from pathlib import Path
import stat
import tempfile
import unittest
from unittest.mock import patch
import urllib.error
import urllib.request

SCRIPT = Path(__file__).resolve().parents[1] / "scripts/capture_tahoma_setup.py"


class Capture(unittest.TestCase):
    def setUp(self):
        self.assertTrue(SCRIPT.is_file(), "missing owner-run capture helper")
        spec = importlib.util.spec_from_file_location("capture_tahoma_setup", SCRIPT)
        self.capture = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.capture)

    def test_single_get_preserves_declarations_without_commands(self):
        payload = b'{"devices":[{"controllableName":"fictional:Covering","definition":{"commands":[{"commandName":"close","nparams":0}]}}]}'
        seen = []

        class InventoryTransport:
            def open(self, request, timeout):
                seen.append((request.full_url, request.get_method(), request.data,
                             request.get_header("Authorization"), timeout))
                return io.BytesIO(payload)

        result = self.capture.read_setup(InventoryTransport(), "0000-0000-0000", "fictional-token")
        self.assertEqual(result, payload)
        self.assertEqual(seen, [("https://gateway-0000-0000-0000.local:8443/enduser-mobile-web/1/enduserAPI/setup",
                                 "GET", None, "Bearer fictional-token", 10)])

    def test_rejects_invalid_pin_or_header_before_network(self):
        class NoNetwork:
            def open(self, *args, **kwargs):
                raise AssertionError("unexpected network request")

        for pin, token in [("other.example/", "dummy"), ("0000-0000-0000", "bad\nheader"),
                           ("0000-0000-0000", "")]:
            with self.subTest(pin_valid=pin == "0000-0000-0000"), self.assertRaises(ValueError):
                self.capture.read_setup(NoNetwork(), pin, token)

    def test_rejects_oversize_invalid_or_secret_echoed_response(self):
        for payload in [b"x" * (4 * 1024 * 1024 + 1), b"{", b'{"devices":{}}',
                        b'{"devices":[],"unexpected":"fictional-token"}']:
            class InventoryTransport:
                def open(self, *args, **kwargs):
                    return io.BytesIO(payload)
            with self.subTest(size=len(payload)), self.assertRaises(ValueError):
                self.capture.read_setup(InventoryTransport(), "0000-0000-0000", "fictional-token")

    def test_redirect_is_rejected_without_returning_another_request(self):
        request = urllib.request.Request("https://gateway-0000-0000-0000.local/setup",
                                         headers={"Authorization": "Bearer fictional-token"})
        with self.assertRaises(urllib.error.HTTPError):
            self.capture.NoRedirect().redirect_request(request, None, 302, "redirect", {},
                                                       "https://other.example/")

    def test_capture_is_private_and_never_overwrites_or_follows_symlink(self):
        with tempfile.TemporaryDirectory() as temporary:
            folder = Path(temporary) / "private"
            path = self.capture.save_capture(b'{"devices":[]}', folder)
            self.assertEqual(json.loads(path.read_text()), {"devices": []})
            self.assertEqual(stat.S_IMODE(folder.stat().st_mode), 0o700)
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o600)
            with self.assertRaises(FileExistsError):
                self.capture.save_capture(b"replacement", folder)
            self.assertEqual(path.read_bytes(), b'{"devices":[]}')
            link = Path(temporary) / "link"
            link.symlink_to(folder, target_is_directory=True)
            with self.assertRaises(ValueError):
                self.capture.save_capture(b"replacement", link)

    def test_noninteractive_launch_does_not_prompt_or_connect(self):
        with patch.object(self.capture.sys.stdin, "isatty", return_value=False), \
             patch.object(self.capture.getpass, "getpass", side_effect=AssertionError("prompt")), \
             patch("sys.stderr", new_callable=io.StringIO) as error:
            self.assertEqual(self.capture.main(), 1)
        self.assertIn("Terminal", error.getvalue())

    def test_rtk_stderr_pipe_does_not_block_hidden_terminal_input(self):
        class PublicTransport:
            def open(self, *args, **kwargs):
                return io.BytesIO(b"unused-public-ca")

        import ssl
        with patch.object(self.capture.sys.stdin, "isatty", return_value=True), \
             patch.object(self.capture.urllib.request, "build_opener", return_value=PublicTransport()), \
             patch.object(self.capture.ssl, "create_default_context", return_value=ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)), \
             patch.object(self.capture.getpass, "getpass", side_effect=KeyboardInterrupt), \
             patch("sys.stderr", new_callable=io.StringIO) as error:
            self.assertEqual(self.capture.main(), 1)
        self.assertIn("Capture canceled", error.getvalue())
