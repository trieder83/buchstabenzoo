#!/usr/bin/env python3
"""Creates an Ed25519 key pair for signing the ad campaigns (GAME-ADS "External content").

    python3 tools/ads/keygen.py --private ~/secrets/zoo-ads.key

- The PRIVATE key (32-byte seed, base64) is written to the given file (mode 0600, never
  overwritten). Store it OFFLINE (password manager / encrypted USB stick). It must never be
  committed, uploaded, or placed in the repository; anybody with it can show content in the game.
- The PUBLIC key is printed as a TypeScript line for `web/src/ad-keys.ts` (compiled into the game).
"""
import argparse
import base64
import os
import sys

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

RAW = dict(encoding=serialization.Encoding.Raw)


def new_pair() -> tuple[bytes, bytes]:
    key = Ed25519PrivateKey.generate()
    seed = key.private_bytes(format=serialization.PrivateFormat.Raw, encryption_algorithm=serialization.NoEncryption(), **RAW)
    pub = key.public_key().public_bytes(format=serialization.PublicFormat.Raw, **RAW)
    return seed, pub


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--private", required=True, help="file to write the private key to (must not exist)")
    args = ap.parse_args()
    seed, pub = new_pair()
    fd = os.open(args.private, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(fd, "w") as f:
        f.write(base64.b64encode(seed).decode() + "\n")
    print(f"private key written to {args.private} (mode 600) - store it OFFLINE, never commit it.")
    print("public key (paste into web/src/ad-keys.ts, AD_PUBLIC_KEYS):")
    print(f"  '{base64.b64encode(pub).decode()}',")
    sys.stdout.flush()


if __name__ == "__main__":
    main()
