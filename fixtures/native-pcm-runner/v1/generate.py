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
    "riff-44100": "49663d8451e470a7a05511e68388ebff7b4d844db42d38e9632473f897a0b91d",
    "riff-48000": "1e856978bbd412daebd2ac9dd81f554e4c3512244ce36b7437bb65cc5f43c99e",
    "riff-88200": "bc8aa669d31090d7cc9a0abf740e6c63cf719db47cf5dc071fc724e19dfe6fff",
    "riff-96000": "5645de29f441710a3a7b67f2e4a24e93086c9baa34426d8963e3f278ceb9d516",
    "rf64-48000": "43fa3c4ed46228d1ee13050b118f379f82a021e85f5dfff6f72593912e298ad0",
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


def wave(rate: int, rf64: bool, frame_count: int) -> bytes:
    pcm = b"".join(struct.pack("<ff", left, right) for left, right in samples(frame_count))
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


def session(rate: int, name: str, digest: str, start: int, length: int) -> bytes:
    template = (ROOT / "../../session/v1/parametric-eq-nine-track.toml").resolve().read_text()
    text = template.replace("sample_rate_hz = 48000", f"sample_rate_hz = {rate}")
    text = text.replace(
        'identity = "sha256:parametric-eq-nine-track"', f'identity = "sha256:{digest}"'
    ).replace(
        'locator = "host:parametric-eq-nine-track"', f'locator = "file:{name}.wav"'
    ).replace(
        "start_sample = 0, length_samples = 48000",
        f"start_sample = {start}, length_samples = {length}",
    )
    return text.encode()


def main() -> None:
    entries: list[tuple[str, int, str]] = []
    for rate in RATES:
        name = f"riff-{rate}"
        payload = wave(rate, False, 1_024)
        publish(ROOT / f"{name}.wav", payload)
        digest = hashlib.sha256(payload).hexdigest()
        session_bytes = session(rate, name, digest, 0, 1_024)
        publish(ROOT / f"{name}.toml", session_bytes)
        entries.extend(
            ((f"{name}.wav", len(payload), digest),
             (f"{name}.toml", len(session_bytes), hashlib.sha256(session_bytes).hexdigest()))
        )
    name = "rf64-48000"
    payload = wave(48000, True, 516)
    publish(ROOT / f"{name}.wav", payload)
    digest = hashlib.sha256(payload).hexdigest()
    session_bytes = session(48000, name, digest, 1, 514)
    publish(ROOT / f"{name}.toml", session_bytes)
    entries.extend(
        ((f"{name}.wav", len(payload), digest),
         (f"{name}.toml", len(session_bytes), hashlib.sha256(session_bytes).hexdigest()))
    )
    entries.extend((f"output:{name}", 8192, digest) for name, digest in OUTPUTS.items())
    manifest = "schema_version\t1\n" + "".join(
        f"{name}\t{size}\t{digest}\n" for name, size, digest in sorted(entries)
    )
    publish(ROOT / "MANIFEST.tsv", manifest.encode("ascii"))


if __name__ == "__main__":
    main()
