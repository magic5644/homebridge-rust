#!/usr/bin/env python3
"""Owner-run local inventory export; no device commands or credential persistence."""
import getpass
import json
import os
from pathlib import Path
import re
import ssl
import stat
import sys
import urllib.error
import urllib.request
import warnings

CA_URL = "https://ca.overkiz.com/overkiz-root-ca-2048.crt"
PRIVATE = Path(__file__).resolve().parents[1] / "private"


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise urllib.error.HTTPError(req.full_url, code, "Redirect refused", headers, fp)


def read_setup(opener, pin, token):
    if not re.fullmatch(r"[0-9]{4}-[0-9]{4}-[0-9]{4}", pin):
        raise ValueError("Invalid gateway PIN format; expected four-four-four digits.")
    if not re.fullmatch(r"[A-Za-z0-9._~+/-]+=*", token):
        raise ValueError("Invalid token format; paste only the token, without whitespace.")
    request = urllib.request.Request(
        f"https://gateway-{pin}.local:8443/enduser-mobile-web/1/enduserAPI/setup",
        headers={"Authorization": "Bearer " + token, "Accept": "application/json"},
        method="GET",
    )
    with opener.open(request, timeout=10) as response:
        payload = response.read(4 * 1024 * 1024 + 1)
    if len(payload) > 4 * 1024 * 1024:
        raise ValueError("Setup exceeds the 4 MiB limit; nothing saved.")
    try:
        setup = json.loads(payload)
    except (ValueError, UnicodeError):
        raise ValueError("Invalid setup JSON; nothing saved.") from None
    if token in json.dumps(setup):
        raise ValueError("Unexpected secret in response; nothing saved.")
    if not isinstance(setup, dict) or not isinstance(setup.get("devices"), list):
        raise ValueError("Setup must contain a devices array; nothing saved.")
    return payload


def save_capture(payload, folder=PRIVATE):
    folder.mkdir(mode=0o700, exist_ok=True)
    if folder.is_symlink() or not folder.is_dir():
        raise ValueError("Private directory must be a real directory, not a symlink.")
    info = folder.stat()
    if info.st_uid != os.getuid() or stat.S_IMODE(info.st_mode) & 0o077:
        raise ValueError("Private directory must belong to you and have permissions 0700.")
    path = folder / "tahoma-setup.json"
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(descriptor, "wb") as output:
            output.write(payload)
    except OSError:
        path.unlink()
        raise
    return path


def main():
    if not sys.stdin.isatty():
        print("Run this yourself in Terminal; hidden input requires an interactive terminal.", file=sys.stderr)
        return 1
    try:
        public = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())
        with public.open(CA_URL, timeout=10) as response:
            ca = response.read(65537)
        if len(ca) > 65536:
            raise ValueError("Manufacturer CA exceeds size limit; stopped.")
        context = ssl.create_default_context(cadata=ca.decode("ascii"))
        local = urllib.request.build_opener(
            urllib.request.ProxyHandler({}), NoRedirect(),
            urllib.request.HTTPSHandler(context=context),
        )
        with warnings.catch_warnings():
            warnings.simplefilter("error", getpass.GetPassWarning)
            pin = getpass.getpass("TaHoma box PIN (hidden, four-four-four digits): ").strip()
            token = getpass.getpass("Local developer token (hidden): ").strip()
        payload = read_setup(local, pin, token)
        save_capture(payload)
    except urllib.error.HTTPError as error:
        print(f"HTTP {error.code}: no redirect/retry, no capture saved. Check the local token in the app.", file=sys.stderr)
        return 1
    except (urllib.error.URLError, ssl.SSLError, TimeoutError):
        print("Connection/TLS failed. Check the same LAN, gateway PIN and developer mode. Certificate checks remain enabled.", file=sys.stderr)
        return 1
    except FileExistsError:
        print("private/tahoma-setup.json already exists; it was not overwritten.", file=sys.stderr)
        return 1
    except (ValueError, OSError) as error:
        # Do not print upstream errors that could contain private paths or identifiers.
        message = str(error) if isinstance(error, ValueError) else "Local file operation failed; stopped."
        print(message, file=sys.stderr)
        return 1
    except getpass.GetPassWarning:
        print("Hidden input is unavailable. Open a normal Terminal; no token was requested in clear text.", file=sys.stderr)
        return 1
    except (KeyboardInterrupt, EOFError):
        print("Capture canceled; no credential saved.", file=sys.stderr)
        return 1
    print("Read-only capture saved in private/tahoma-setup.json (0600). Do not upload or paste it.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
