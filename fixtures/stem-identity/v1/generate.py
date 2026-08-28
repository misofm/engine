#!/usr/bin/env python3
"""Independent stdlib-only canonical-PCM vector generator/checker."""

from __future__ import annotations

import hashlib
import pathlib
import struct
import sys

ROOT = pathlib.Path(__file__).resolve().parent
CHECK = sys.argv[1:] == ["--check"]
if sys.argv[1:] not in ([], ["--check"]):
    raise SystemExit("usage: generate.py [--check]")

VECTORS = {
    "pcm16-mono-boundaries": (16, ((0,), (32767,), (-32768,), (1,), (-1,))),
    "pcm16-stereo-boundaries": (16, ((0, 32767), (-32768, 1), (-1, 0))),
    "pcm24-mono-boundaries": (24, ((0,), (8388607,), (-8388608,), (1,), (-1,))),
    "pcm24-stereo-boundaries": (24, ((0, 8388607), (-8388608, 1), (-1, 0))),
}


def publish(path: pathlib.Path, payload: bytes) -> None:
    if CHECK:
        if path.read_bytes() != payload:
            raise SystemExit(f"fixture drift: {path.name}")
    else:
        path.write_bytes(payload)


def canonical_pcm(bit_depth: int, frames: tuple[tuple[int, ...], ...]) -> bytes:
    width = bit_depth // 8
    return b"".join(
        sample.to_bytes(width, "little", signed=True)
        for frame in frames
        for sample in frame
    )


def wave(bit_depth: int, frames: tuple[tuple[int, ...], ...], pcm: bytes) -> bytes:
    channels = len(frames[0])
    rate = 48000
    block_align = channels * bit_depth // 8
    fmt = struct.pack("<HHIIHH", 1, channels, rate, rate * block_align, block_align, bit_depth)
    body = b"WAVE" + b"fmt " + struct.pack("<I", len(fmt)) + fmt
    body += b"data" + struct.pack("<I", len(pcm)) + pcm
    return b"RIFF" + struct.pack("<I", len(body)) + body


def sample_text(frames: tuple[tuple[int, ...], ...]) -> str:
    return "|".join(",".join(str(sample) for sample in frame) for frame in frames)


def main() -> None:
    rows = [
        "name\tbit_depth\tchannels\tframes\tsamples_by_frame\tcanonical_hex\tidentity\tpcm_file\twave_file"
    ]
    for name, (bit_depth, frames) in sorted(VECTORS.items()):
        pcm = canonical_pcm(bit_depth, frames)
        pcm_name = f"{name}.pcm"
        publish(ROOT / pcm_name, pcm)
        wave_name = "-"
        if "stereo" in name:
            wave_name = f"{name}.wav"
            publish(ROOT / wave_name, wave(bit_depth, frames, pcm))
        identity = "sha256:" + hashlib.sha256(pcm).hexdigest()
        rows.append(
            "\t".join(
                (
                    name,
                    str(bit_depth),
                    str(len(frames[0])),
                    str(len(frames)),
                    sample_text(frames),
                    pcm.hex(),
                    identity,
                    pcm_name,
                    wave_name,
                )
            )
        )
    publish(ROOT / "VECTORS.tsv", ("schema_version\t1\n" + "\n".join(rows) + "\n").encode("ascii"))


if __name__ == "__main__":
    main()
