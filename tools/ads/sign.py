#!/usr/bin/env python3
"""Builds and signs the ad manifest (GAME-ADS "External content").

    python3 tools/ads/sign.py --key ~/secrets/zoo-ads.key --version 3 --valid-days 90

Reads the template (default tools/ads/campaigns.template.json), fills every image's sha256 /
bytes / width / height from the files in --ads-dir, sets version / issued / valid_until, checks
the same limits the game enforces, then writes <ads-dir>/index.json and index.sig
(base64 Ed25519 signature over b"buchstabenzoo-ads/1\\n" + the exact bytes of index.json).
The key file can also be named by the environment variable ZOO_ADS_KEY. The version must be
larger than the one deployed before (the game never accepts a lower one).
"""
import argparse
import base64
import datetime as dt
import hashlib
import json
import os
import re
import struct
from urllib.parse import urlparse
import sys
from pathlib import Path

from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

ROOT = Path(__file__).resolve().parents[2]
DOMAIN = b"buchstabenzoo-ads/1\n"
FORMAT = "buchstabenzoo-ads/1"
MAX_IMAGE_BYTES = 512 * 1024
MAX_DIM = 2048
MAX_TAGLINE = 80
# the campaigns the game knows (web/src/ads.ts KNOWN_CAMPAIGNS): id -> (slot, link host)
KNOWN = {"mathfighter": (1, "mathfighter.rcms.ch"), "abcsmash": (2, "abcsmash.rcms.ch"),
         "edugamegalaxy": (3, "edugamegalaxy.rcms.ch"),
         # native app build only (boards-native/): the own App Store pages, id fixed (web/src/ads.ts KNOWN_CAMPAIGNS)
         "mathfighter-ios": (1, "apps.apple.com/app/id6760628828"), "abcsmash-ios": (2, "apps.apple.com/app/id6790508038"),
         "mathfighter-ios-b": (3, "apps.apple.com/app/id6760628828")}
IOS_HOSTS = ("apps.apple.com", "itunes.apple.com")
IOS_PATH_RE = re.compile(r"^/(?:[a-z]{2}/)?app/(?:[a-z0-9-]{1,60}/)?id\d{6,12}$")
PLAY_RE = re.compile(r"^https://play\.google\.com/store/apps/details\?id=[A-Za-z][A-Za-z0-9_]*(?:\.[A-Za-z0-9_]+)+$")
PATH_RE = re.compile(r"^img/[a-z0-9][a-z0-9._-]{0,63}$")


def image_info(data: bytes) -> tuple[str, int, int]:
    """(mime, width, height) from the magic bytes and the header (PNG, WebP, JPEG)."""
    if data[:8] == b"\x89PNG\r\n\x1a\n":
        w, h = struct.unpack(">II", data[16:24])
        return "image/png", w, h
    if data[:4] == b"RIFF" and data[8:12] == b"WEBP":
        kind = data[12:16]
        if kind == b"VP8X":
            return "image/webp", 1 + int.from_bytes(data[24:27], "little"), 1 + int.from_bytes(data[27:30], "little")
        if kind == b"VP8L":
            b = data[21:25]
            bits = int.from_bytes(b, "little")
            return "image/webp", (bits & 0x3FFF) + 1, ((bits >> 14) & 0x3FFF) + 1
        if kind == b"VP8 ":
            w, h = struct.unpack("<HH", data[26:30])
            return "image/webp", w & 0x3FFF, h & 0x3FFF
    if data[:2] == b"\xff\xd8":
        i = 2
        while i + 9 < len(data):
            if data[i] != 0xFF:
                i += 1
                continue
            m = data[i + 1]
            if m in (0xC0, 0xC1, 0xC2):
                h, w = struct.unpack(">HH", data[i + 5 : i + 9])
                return "image/jpeg", w, h
            i += 2 + struct.unpack(">H", data[i + 2 : i + 4])[0]
    raise SystemExit("unsupported image (PNG, WebP or JPEG only)")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--key", default=os.environ.get("ZOO_ADS_KEY"), help="private key file (or env ZOO_ADS_KEY)")
    ap.add_argument("--template", default=str(ROOT / "tools/ads/campaigns.template.json"))
    ap.add_argument("--ads-dir", default=str(ROOT / "boards"))
    ap.add_argument("--version", type=int, required=True, help="monotonic manifest version")
    ap.add_argument("--valid-days", type=int, default=90)
    ap.add_argument("--now", help="ISO time for `issued` (tests)")
    args = ap.parse_args()
    if not args.key:
        sys.exit("no private key: pass --key FILE or set ZOO_ADS_KEY")
    ads = Path(args.ads_dir)
    seed = base64.b64decode(Path(args.key).read_text().strip())
    if len(seed) != 32:
        sys.exit("the key file must hold a base64 32-byte Ed25519 seed (tools/ads/keygen.py)")
    key = Ed25519PrivateKey.from_private_bytes(seed)

    manifest = json.loads(Path(args.template).read_text(encoding="utf8"))
    issued = dt.datetime.fromisoformat(args.now) if args.now else dt.datetime.now(dt.timezone.utc)
    issued = issued.astimezone(dt.timezone.utc).replace(microsecond=0)
    manifest["format"] = FORMAT
    manifest["version"] = args.version
    manifest["issued"] = issued.strftime("%Y-%m-%dT%H:%M:%SZ")
    manifest["valid_until"] = (issued + dt.timedelta(days=args.valid_days)).strftime("%Y-%m-%dT%H:%M:%SZ")
    if len(manifest["campaigns"]) > 3:
        sys.exit("at most 3 campaigns")
    for c in manifest["campaigns"]:
        if c["id"] not in KNOWN:
            sys.exit(f"campaign {c['id']!r} is not compiled into the game (web/src/ads.ts KNOWN_CAMPAIGNS)")
        slot, host = KNOWN[c["id"]]
        if c["slot"] != slot or c.get("link") != f"https://{host}":
            sys.exit(f"campaign {c['id']}: slot/link must be {slot} / https://{host}")
        # optional store links per platform (GAME-ADS rule 16): the same rules as web/src/ads.ts checkStoreLink
        for plat, link in (c.get("links") or {}).items():
            if plat == "ios":
                u = urlparse(link)
                ok = u.scheme == "https" and u.hostname in IOS_HOSTS and not (u.query or u.fragment or u.port or u.username) and IOS_PATH_RE.match(u.path)
            elif plat == "android":
                ok = bool(PLAY_RE.match(link))
            else:
                ok = False
            if not ok:
                sys.exit(f"campaign {c['id']}: bad {plat} link {link!r}")
        for lang, text in c["tagline"].items():
            if lang not in ("de", "en") or not text or len(text) > MAX_TAGLINE or re.search(r"[<>\x00-\x1f]", text):
                sys.exit(f"campaign {c['id']}: bad tagline {lang!r}")
        for im in c["images"]:
            if not PATH_RE.match(im["path"]):
                sys.exit(f"bad image path {im['path']!r}")
            data = (ads / im["path"]).read_bytes()
            if len(data) > MAX_IMAGE_BYTES:
                sys.exit(f"{im['path']}: {len(data)} bytes > {MAX_IMAGE_BYTES}")
            mime, w, h = image_info(data)
            if not (64 <= w <= MAX_DIM and 64 <= h <= MAX_DIM):
                sys.exit(f"{im['path']}: {w}x{h} outside 64..{MAX_DIM}")
            im.update(mime=mime, bytes=len(data), width=w, height=h, sha256=hashlib.sha256(data).hexdigest())
    body = (json.dumps(manifest, ensure_ascii=False, indent=2) + "\n").encode("utf8")
    sig = key.sign(DOMAIN + body)
    (ads / "index.json").write_bytes(body)
    (ads / "index.sig").write_text(base64.b64encode(sig).decode() + "\n")
    print(f"signed version {args.version}, valid until {manifest['valid_until']}: {ads / 'index.json'} + index.sig")


if __name__ == "__main__":
    main()
