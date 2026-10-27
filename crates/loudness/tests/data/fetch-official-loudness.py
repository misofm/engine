#!/usr/bin/env python3
"""Acquire or verify the complete applicable ITU corpus outside the checkout."""

import argparse
import csv
import hashlib
import io
from pathlib import Path
import struct
import sys
import urllib.request
import zipfile

MANIFEST = Path(__file__).with_name("official-loudness.tsv")


def required_cases():
    with MANIFEST.open(encoding="utf-8", newline="") as source:
        rows = csv.DictReader((line for line in source if not line.startswith("#")), delimiter="\t")
        cases = [row for row in rows if row["status"] == "required"]
    if len(cases) != 19 or len({row["member"] for row in cases}) != 19:
        raise ValueError("manifest must identify all 19 distinct applicable ITU cases")
    for case in cases:
        if Path(case["member"]).name != case["member"]:
            raise ValueError("fixture member must be a basename")
    return cases


def verify_hash(payload, expected, label):
    actual = hashlib.sha256(payload).hexdigest()
    if actual != expected:
        raise ValueError(f"{label}: SHA-256 {actual}, expected {expected}")


def verify_wave(payload, case):
    verify_hash(payload, case["wav_sha256"], case["member"])
    if payload[:4] != b"RIFF" or payload[8:12] != b"WAVE":
        raise ValueError("expected RIFF WAVE")
    declared = struct.unpack_from("<I", payload, 4)[0] + 8
    limit = len(payload)
    if limit - declared != int(case["riff_length_delta"]):
        raise ValueError("RIFF length mismatch against the pinned official header")
    chunks = {}
    offset = 12
    while offset < limit:
        if offset + 8 > limit:
            raise ValueError("truncated RIFF header")
        length = struct.unpack_from("<I", payload, offset + 4)[0]
        end = offset + 8 + length
        if end + (length % 2) > limit:
            raise ValueError("truncated RIFF chunk")
        tag = payload[offset:offset + 4]
        if tag in (b"fmt ", b"data"):
            if tag in chunks:
                raise ValueError("duplicate PCM chunk")
            chunks[tag] = payload[offset + 8:end]
        offset = end + (length % 2)
    fmt, data = chunks[b"fmt "], chunks[b"data"]
    tag, channels, rate, byte_rate, align, bits = struct.unpack_from("<HHIIHH", fmt)
    if tag == 65534:
        pcm_guid = bytes.fromhex("0100000000001000800000aa00389b71")
        if len(fmt) != 40 or struct.unpack_from("<HH", fmt, 16) != (22, 16) or fmt[24:40] != pcm_guid:
            raise ValueError("expected 16-bit extensible PCM")
    elif tag != 1:
        raise ValueError("expected integer PCM")
    actual = (channels, rate, bits, len(data) // align)
    expected = tuple(int(case[key]) for key in ("channels", "rate", "bits", "frames"))
    if actual != expected or align != channels * 2 or byte_rate != rate * align or len(data) % align:
        raise ValueError(f"{case['member']}: PCM metadata {actual}, expected {expected}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path, help="external private fixture/cache directory")
    parser.add_argument("--verify", action="store_true", help="offline: fail if any selected WAV is absent or altered")
    args = parser.parse_args()
    cases = required_cases()
    if not args.verify:
        args.directory.mkdir(parents=True, exist_ok=True)
        (args.directory / "archives").mkdir(exist_ok=True)
    for case in cases:
        target = args.directory / case["member"]
        archive = args.directory / "archives" / f"{case['id']}.zip"
        if archive.exists():
            compressed = archive.read_bytes()
            verify_hash(compressed, case["archive_sha256"], str(archive))
        elif args.verify:
            raise FileNotFoundError(f"required official archive absent: {archive}")
        else:
            with urllib.request.urlopen(case["archive_url"], timeout=60) as response:
                compressed = response.read()
            verify_hash(compressed, case["archive_sha256"], case["archive_url"])
            with archive.open("xb") as output:
                output.write(compressed)
        if target.exists():
            verify_wave(target.read_bytes(), case)
        elif args.verify:
            raise FileNotFoundError(f"required fixture absent: {target}")
        else:
            with zipfile.ZipFile(io.BytesIO(compressed)) as bundle:
                if bundle.namelist() != [case["member"]]:
                    raise ValueError(f"{archive}: unexpected archive members")
                payload = bundle.read(case["member"])
            verify_wave(payload, case)
            with target.open("xb") as output:
                output.write(payload)
        print(f"verified {case['id']}: {case['member']}")
    print(f"verified all {len(cases)} required mono/stereo files; 28 multichannel files outside contract")


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, struct.error, zipfile.BadZipFile) as error:
        sys.exit(f"official loudness fixture error: {error}")
