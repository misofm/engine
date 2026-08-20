#!/usr/bin/env python3
"""Independent stdlib-only CIDv1 raw/SHA2-256 reference for exact input bytes."""
import base64
import hashlib
import sys

def cid(data: bytes) -> str:
    raw = bytes((1, 0x55, 0x12, 0x20)) + hashlib.sha256(data).digest()
    return "b" + base64.b32encode(raw).decode("ascii").lower().rstrip("=")

if __name__ == "__main__":
    print(cid(sys.stdin.buffer.read()))
