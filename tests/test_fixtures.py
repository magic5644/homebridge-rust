"""Offline checks for the reviewed fictional fixture bundle; no hardware access."""
import hashlib
import json
from pathlib import Path
import re
import shutil
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


def validate(root):
    manifest = root / "tests/fixtures_manifest.json"
    assert manifest.is_file(), "missing fixture manifest"
    entries = json.loads(manifest.read_text())["fixtures"]
    expected = {entry["file"] for entry in entries}
    actual = {str(path.relative_to(root)) for path in (root / "fixtures/tahoma").glob("*.json")}
    assert actual == expected and len(expected) == 10, "fixture file set differs"
    for entry in entries:
        path = root / entry["file"]
        payload = path.read_bytes()
        value = json.loads(payload)
        assert hashlib.sha256(payload).hexdigest() == entry["sha256"], "unreviewed fixture content"
        validate_identifiers(value)


def validate_identifiers(value):
    if isinstance(value, dict):
        for key, item in value.items():
            assert not re.search(r"token|password|cookie|secret|authorization|pin|location|room|account", key, re.I), "private field"
            if key == "gatewayId":
                assert item == "0000-0000-0000", "nonfictional gateway"
            if key == "deviceURL":
                assert re.fullmatch(r"(?:io|rts|ogp)://0000-0000-0000/[1-9][0-9]*", item), "nonfictional device URL"
            if key == "label":
                assert item.startswith("Fictional "), "nonfictional label"
            validate_identifiers(item)
    elif isinstance(value, list):
        for item in value:
            validate_identifiers(item)


class Fixtures(unittest.TestCase):
    def test_reviewed_bundle(self):
        validate(ROOT)

    def test_rejects_changed_missing_extra_and_invalid_files(self):
        for mutation in ("changed", "missing", "extra", "invalid"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                shutil.copytree(ROOT / "tests", root / "tests", ignore=shutil.ignore_patterns("__pycache__"))
                shutil.copytree(ROOT / "fixtures", root / "fixtures")
                path = root / "fixtures/tahoma/setup_io.json"
                if mutation == "missing":
                    path.unlink()
                elif mutation == "extra":
                    (path.parent / "extra.json").write_text("{}")
                elif mutation == "invalid":
                    path.write_text("{")
                else:
                    path.write_text(path.read_text().replace("Fictional io covering", "Unreviewed label"))
                with self.assertRaises((AssertionError, ValueError)):
                    validate(root)

    def test_rejects_private_identifiers_even_if_hash_is_updated(self):
        for value in ({"gatewayId": "unapproved"}, {"deviceURL": "io://unapproved/1"},
                      {"label": "Unapproved name"}, {"token": "dummy"}):
            with self.subTest(field=next(iter(value))), self.assertRaises(AssertionError):
                validate_identifiers(value)
