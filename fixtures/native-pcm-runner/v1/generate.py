#!/usr/bin/env python3
"""Independent stdlib-only Issue 073 fixture generator/checker."""

from __future__ import annotations

import hashlib
import pathlib
import struct
import sys

ROOT = pathlib.Path(__file__).resolve().parent
RATES = (44100, 48000, 88200, 96000)
OUTPUTS = {
    "riff-44100": "fbf9a1482fb224415f7fb96b4f1c2026b3302f7c474eb5b2639d3d63e0a3ce92",
    "riff-48000": "cef2b4282bb8478687b4dec5f764a9f04bc64fc7a35d3a8edd5b398a80494771",
    "riff-88200": "0a2ae7050b4a443e0888281e3953963706f249ffd28a3d9a61cdbbc675d7a0b7",
    "riff-96000": "dcb0de625cb09c064ea424dff6b1eca01896ba1e7ee602c72dc7454ad9b74f16",
    "rf64-48000": "9c3011f06e52c7f1006c2d7710b4d71ac2d928d76ba5f94f24ef55525cce9100",
}
CHECK = sys.argv[1:] == ["--check"]


def publish(path: pathlib.Path, payload: bytes) -> None:
    if CHECK:
        if path.read_bytes() != payload:
            raise SystemExit(f"fixture drift: {path.name}")
    else:
        path.write_bytes(payload)


def samples(frame_count: int) -> list[tuple[float, float]]:
    seed = [
        (0.0, -0.0),
        (0.25, -0.25),
        (float("nan"), 1.0e-40),
        (-0.5, 0.5),
        (0.75, -0.75),
        (-0.125, 0.125),
        (0.375, -0.375),
        (1.0, -1.0),
    ]
    return [seed[index % len(seed)] for index in range(frame_count)]


def canonical_f32(frame_count: int, start: int = 0) -> bytes:
    selected = samples(frame_count + start)[start : start + frame_count]
    return b"".join(struct.pack("<ff", left, right) for left, right in selected)


def wave(rate: int, rf64: bool, frame_count: int, start: int = 0) -> bytes:
    pcm = canonical_f32(frame_count, start)
    fmt = struct.pack("<HHIIHH", 3, 2, rate, rate * 8, 8, 32)
    if not rf64:
        body = b"WAVE" + b"fmt " + struct.pack("<I", len(fmt)) + fmt
        body += b"data" + struct.pack("<I", len(pcm)) + pcm
        return b"RIFF" + struct.pack("<I", len(body)) + body
    ds64 = struct.pack(
        "<QQQI", 4 + 8 + 28 + 8 + len(fmt) + 8 + len(pcm), len(pcm), frame_count, 0
    )
    return (
        b"RF64\xff\xff\xff\xffWAVE"
        + b"ds64"
        + struct.pack("<I", len(ds64))
        + ds64
        + b"fmt "
        + struct.pack("<I", len(fmt))
        + fmt
        + b"data\xff\xff\xff\xff"
        + pcm
    )


def session(rate: int, digest: str, frames: int) -> bytes:
    template = (ROOT / "../../session/v1/parametric-eq-nine-track.toml").resolve().read_text()
    text = template.replace("sample_rate_hz = 48000", f"sample_rate_hz = {rate}")
    text = text.replace(
        "sha256:7e945c107a97cd24135e85dc2f407c5ecd39663a8737bf5b92114ccce38f1ab8",
        f"sha256:{digest}",
    ).replace(
        'bit_depth = "32f", frames = 48000',
        f'bit_depth = "32f", frames = {frames}',
    )
    return text.encode()


def main() -> None:
    entries: list[tuple[str, int, str]] = []
    for rate in RATES:
        name = f"riff-{rate}"
        payload = wave(rate, False, 1_024)
        publish(ROOT / f"{name}.wav", payload)
        content_digest = hashlib.sha256(canonical_f32(1_024)).hexdigest()
        session_bytes = session(rate, content_digest, 1_024)
        publish(ROOT / f"{name}.toml", session_bytes)
        entries.extend(
            ((f"{name}.wav", len(payload), hashlib.sha256(payload).hexdigest()),
             (f"{name}.toml", len(session_bytes), hashlib.sha256(session_bytes).hexdigest()))
        )
    name = "rf64-48000"
    # The old session selected frames [1, 515) from a 516-frame asset. Region selection is gone;
    # pre-slicing retains those exact 514 decoded float bit patterns and therefore the render.
    payload = wave(48000, True, 514, start=1)
    publish(ROOT / f"{name}.wav", payload)
    content_digest = hashlib.sha256(canonical_f32(514, start=1)).hexdigest()
    session_bytes = session(48000, content_digest, 514)
    publish(ROOT / f"{name}.toml", session_bytes)
    entries.extend(
        ((f"{name}.wav", len(payload), hashlib.sha256(payload).hexdigest()),
         (f"{name}.toml", len(session_bytes), hashlib.sha256(session_bytes).hexdigest()))
    )
    entries.extend((f"output:{name}", 8192, digest) for name, digest in OUTPUTS.items())
    manifest = "schema_version\t1\n" + "".join(
        f"{name}\t{size}\t{digest}\n" for name, size, digest in sorted(entries)
    )
    publish(ROOT / "MANIFEST.tsv", manifest.encode("ascii"))


if __name__ == "__main__":
    main()
