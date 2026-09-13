#!/usr/bin/env python3
"""Independent stdlib-only canonical-PCM BLAKE3 vector generator/checker."""

from __future__ import annotations

import pathlib
import struct
import sys

ROOT = pathlib.Path(__file__).resolve().parent
CHECK = sys.argv[1:] == ["--check"]
if sys.argv[1:] not in ([], ["--check"]):
    raise SystemExit("usage: generate.py [--check]")

VECTORS = {
    "f32-mono-edge-bits": ("32f", ((0x7FC00001,), (0x00000001,), (0x80000000,))),
    "f32-stereo-edge-bits": (
        "32f",
        ((0x7FC00001, 0x80000000), (0x00000001, 0xFFC12345)),
    ),
    "pcm16-mono-boundaries": ("16", ((0,), (32767,), (-32768,), (1,), (-1,))),
    "pcm16-stereo-boundaries": ("16", ((0, 32767), (-32768, 1), (-1, 0))),
    "pcm24-mono-boundaries": ("24", ((0,), (8388607,), (-8388608,), (1,), (-1,))),
    "pcm24-stereo-boundaries": ("24", ((0, 8388607), (-8388608, 1), (-1, 0))),
}

# The frozen corpus inputs all fit within one 1024-byte BLAKE3 chunk. Keeping the small,
# one-chunk reference here makes the generated pins independently checkable without a Python
# package while the streaming Rust implementation is qualified separately by its official vectors.
IV = (0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19)
MESSAGE_PERMUTATION = (2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8)
CHUNK_START = 1
CHUNK_END = 2
ROOT_OUTPUT = 8


def rotate_right(value: int, amount: int) -> int:
    return ((value >> amount) | (value << (32 - amount))) & 0xFFFFFFFF


def mix(state: list[int], a: int, b: int, c: int, d: int, x: int, y: int) -> None:
    state[a] = (state[a] + state[b] + x) & 0xFFFFFFFF
    state[d] = rotate_right(state[d] ^ state[a], 16)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = rotate_right(state[b] ^ state[c], 12)
    state[a] = (state[a] + state[b] + y) & 0xFFFFFFFF
    state[d] = rotate_right(state[d] ^ state[a], 8)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = rotate_right(state[b] ^ state[c], 7)


def compress(block: bytes, block_length: int, flags: int) -> bytes:
    words = list(struct.unpack("<16I", block.ljust(64, b"\0")))
    state = list(IV) + list(IV[:4]) + [0, 0, block_length, flags]
    for round_index in range(7):
        mix(state, 0, 4, 8, 12, words[0], words[1])
        mix(state, 1, 5, 9, 13, words[2], words[3])
        mix(state, 2, 6, 10, 14, words[4], words[5])
        mix(state, 3, 7, 11, 15, words[6], words[7])
        mix(state, 0, 5, 10, 15, words[8], words[9])
        mix(state, 1, 6, 11, 12, words[10], words[11])
        mix(state, 2, 7, 8, 13, words[12], words[13])
        mix(state, 3, 4, 9, 14, words[14], words[15])
        if round_index != 6:
            words = [words[index] for index in MESSAGE_PERMUTATION]
    output = [state[index] ^ state[index + 8] for index in range(8)]
    return struct.pack("<8I", *output)


def blake3_256(payload: bytes) -> bytes:
    if len(payload) > 64:
        raise ValueError("fixed corpus BLAKE3 reference accepts one block only")
    return compress(payload, len(payload), CHUNK_START | CHUNK_END | ROOT_OUTPUT)


def publish(path: pathlib.Path, payload: bytes) -> None:
    if CHECK:
        if path.read_bytes() != payload:
            raise SystemExit(f"fixture drift: {path.name}")
    else:
        path.write_bytes(payload)


def canonical_pcm(bit_depth: str, frames: tuple[tuple[int, ...], ...]) -> bytes:
    if bit_depth == "32f":
        return b"".join(
            sample.to_bytes(4, "little", signed=False)
            for frame in frames
            for sample in frame
        )
    width = int(bit_depth) // 8
    return b"".join(
        sample.to_bytes(width, "little", signed=True)
        for frame in frames
        for sample in frame
    )


def wave(bit_depth: str, frames: tuple[tuple[int, ...], ...], pcm: bytes) -> bytes:
    channels = len(frames[0])
    rate = 48000
    bits = 32 if bit_depth == "32f" else int(bit_depth)
    format_tag = 3 if bit_depth == "32f" else 1
    block_align = channels * bits // 8
    fmt = struct.pack(
        "<HHIIHH", format_tag, channels, rate, rate * block_align, block_align, bits
    )
    body = b"WAVE" + b"fmt " + struct.pack("<I", len(fmt)) + fmt
    body += b"data" + struct.pack("<I", len(pcm)) + pcm
    return b"RIFF" + struct.pack("<I", len(body)) + body


def sample_text(bit_depth: str, frames: tuple[tuple[int, ...], ...]) -> str:
    def scalar(sample: int) -> str:
        return f"0x{sample:08x}" if bit_depth == "32f" else str(sample)

    return "|".join(",".join(scalar(sample) for sample in frame) for frame in frames)


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
        identity = "blake3:" + blake3_256(pcm).hex()
        rows.append(
            "\t".join(
                (
                    name,
                    bit_depth,
                    str(len(frames[0])),
                    str(len(frames)),
                    sample_text(bit_depth, frames),
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
