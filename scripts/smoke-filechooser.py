#!/usr/bin/env python3
"""Interactive public portal smoke test; selects paths without writing files.

Run in the development desktop's session bus. Requires python3-gobject.
Use --cancel and cancel the dialog to check response 1 instead of success.
"""
import argparse
import sys

from gi.repository import Gio, GLib

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("mode", choices=["open", "save", "save-many"])
parser.add_argument("--cancel", action="store_true")
args = parser.parse_args()
bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
loop = GLib.MainLoop()
exit_code = 1
token = "niwoe_filechooser_smoke"
request_path = "/org/freedesktop/portal/desktop/request/" + bus.get_unique_name()[1:].replace(".", "_") + "/" + token


def response(_bus, _sender, _path, _interface, _signal, parameters):
    global exit_code
    code, results = parameters.unpack()
    print("Response:", code, results, flush=True)
    uris = results.get("uris", [])
    if args.cancel:
        valid = code == 1 and not uris
    else:
        valid = code == 0 and bool(uris) and all(uri.startswith("file://") for uri in uris)
        if args.mode == "save":
            valid = valid and len(uris) == 1
        elif args.mode == "save-many":
            valid = valid and len(uris) == 2 and uris[0].endswith("/niwoe%20one.txt") and uris[1].endswith("/niwoe-two.txt")
    exit_code = 0 if valid else 1
    loop.quit()


def timeout():
    print("FAIL: no portal response within 180 seconds", flush=True)
    bus.call_sync("org.freedesktop.portal.Desktop", request_path,
                  "org.freedesktop.portal.Request", "Close", None, None,
                  Gio.DBusCallFlags.NONE, 5000, None)
    loop.quit()
    return GLib.SOURCE_REMOVE


subscription = bus.signal_subscribe(None, "org.freedesktop.portal.Request", "Response",
                                    request_path, None, Gio.DBusSignalFlags.NONE, response)
options = {"handle_token": GLib.Variant("s", token)}
if args.mode == "save":
    options["current_name"] = GLib.Variant("s", "niwoe-portal-test.txt")
elif args.mode == "save-many":
    options["files"] = GLib.Variant("aay", [list(b"niwoe one.txt\0"), list(b"niwoe-two.txt\0")])
method = {"open": "OpenFile", "save": "SaveFile", "save-many": "SaveFiles"}[args.mode]
result = bus.call_sync("org.freedesktop.portal.Desktop", "/org/freedesktop/portal/desktop",
                       "org.freedesktop.portal.FileChooser", method,
                       GLib.Variant("(ssa{sv})", ("", "NIWOE Portal-Test: " + args.mode, options)),
                       GLib.VariantType.new("(o)"), Gio.DBusCallFlags.NONE, 10000, None)
assert result.unpack()[0] == request_path
GLib.timeout_add_seconds(180, timeout)
loop.run()
bus.signal_unsubscribe(subscription)
print("PASS" if exit_code == 0 else "FAIL", flush=True)
sys.exit(exit_code)
