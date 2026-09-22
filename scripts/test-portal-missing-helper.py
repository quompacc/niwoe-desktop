#!/usr/bin/env python3
"""Run under dbus-run-session; exercises the real release backend on a private bus."""
import os
import pathlib
import subprocess
import time

from gi.repository import Gio, GLib

root = pathlib.Path(__file__).resolve().parent.parent
environment = dict(os.environ, NIWOE_FILE_PICKER="/nonexistent/niwoe-file-picker")
backend = subprocess.Popen([str(root / "target/release/niwoe-portal")], env=environment)
try:
    bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
    for attempt in range(100):
        owned = bus.call_sync(
            "org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus",
            "NameHasOwner", GLib.Variant("(s)", ("org.freedesktop.impl.portal.desktop.niwoe",)),
            None, Gio.DBusCallFlags.NONE, 1000, None,
        ).unpack()[0]
        if owned:
            break
        assert backend.poll() is None, "backend exited before acquiring its name"
        time.sleep(0.05)
    assert owned, "backend did not acquire its new D-Bus name"
    for method in ("OpenFile", "SaveFile", "SaveFiles"):
        options = {}
        if method == "SaveFiles":
            options["files"] = GLib.Variant("aay", [list(b"test.txt\0")])
        response = bus.call_sync(
            "org.freedesktop.impl.portal.desktop.niwoe", "/org/freedesktop/portal/desktop",
            "org.freedesktop.impl.portal.FileChooser", method,
            GLib.Variant("(osssa{sv})", (
                "/org/freedesktop/portal/desktop/request/test/missing", "org.niwoe.Test",
                "", "missing helper test", options,
            )), None, Gio.DBusCallFlags.NONE, 5000, None,
        ).unpack()
        assert response == (2, {}), (method, response)
        print("PASS:", method, "missing helper returns failure and no URIs")
finally:
    backend.terminate()
    try:
        backend.wait(timeout=5)
    except subprocess.TimeoutExpired:
        backend.kill()
        backend.wait()
