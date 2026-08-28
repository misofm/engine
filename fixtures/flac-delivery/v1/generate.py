#!/usr/bin/env python3
"""Derive FLAC delivery fixtures from the one frozen canonical-PCM corpus."""

from __future__ import annotations

import argparse
import hashlib
import shutil
import struct
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
IDENTITY = ROOT / "fixtures" / "stem-identity" / "v1"
BLOCK_SIZES = (32, 4096)
DELIVERY_FRAMES = 4096
GENERATED_ROOTS = ("FLAC_VECTORS.tsv", "flac", "masters", "mini-catalog", "pcm")


def vectors() -> list[dict[str, str]]:
    lines = (IDENTITY / "VECTORS.tsv").read_text(encoding="utf-8").splitlines()
    assert lines[0] == "schema_version\t1"
    names = lines[1].split("\t")
    return [dict(zip(names, line.split("\t"), strict=True)) for line in lines[2:]]


def expanded_pcm(vector: dict[str, str]) -> bytes:
    pcm = (IDENTITY / vector["pcm_file"]).read_bytes()
    channels = int(vector["channels"])
    bit_depth = int(vector["bit_depth"])
    frames = int(vector["frames"])
    frame_bytes = channels * (bit_depth // 8)
    assert len(pcm) == frames * frame_bytes
    assert f"sha256:{hashlib.sha256(pcm).hexdigest()}" == vector["identity"]
    repetitions, remaining_frames = divmod(DELIVERY_FRAMES, frames)
    expanded = pcm * repetitions + pcm[: remaining_frames * frame_bytes]
    assert len(expanded) == DELIVERY_FRAMES * frame_bytes
    return expanded


def wave_bytes(vector: dict[str, str], pcm: bytes) -> bytes:
    channels = int(vector["channels"])
    bit_depth = int(vector["bit_depth"])
    sample_rate = 48_000
    block_align = channels * (bit_depth // 8)
    byte_rate = sample_rate * block_align
    fmt = struct.pack("<HHIIHH", 1, channels, sample_rate, byte_rate, block_align, bit_depth)
    pad = b"\x00" if len(pcm) % 2 else b""
    return b"RIFF" + struct.pack("<I", 36 + len(pcm) + len(pad)) + b"WAVEfmt " + struct.pack(
        "<I", len(fmt)
    ) + fmt + b"data" + struct.pack("<I", len(pcm)) + pcm + pad


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if completed.returncode != 0:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(command)}\n{completed.stderr}"
        )
    return completed


def build_tree(output: Path) -> None:
    (output / "flac").mkdir(parents=True)
    (output / "masters").mkdir()
    (output / "pcm").mkdir()
    mini = output / "mini-catalog"
    (mini / "masters").mkdir(parents=True)
    manifest = [
        "schema_version\t1",
        "vector\tbit_depth\tchannels\tframes\tconfigured_block_frames\tidentity\tpcm_file\tflac_file\tflac_sha256",
    ]
    catalog_rows = ["schema_version\t1", "name\told_identity\tmaster_wave"]
    old_identities: list[tuple[str, str]] = []
    for vector in vectors():
        canonical_pcm = expanded_pcm(vector)
        pcm_name = f'{vector["name"]}.pcm'
        (output / "pcm" / pcm_name).write_bytes(canonical_pcm)
        identity = f"sha256:{hashlib.sha256(canonical_pcm).hexdigest()}"
        master = wave_bytes(vector, canonical_pcm)
        master_name = f'{vector["name"]}.wav'
        (output / "masters" / master_name).write_bytes(master)
        (mini / "masters" / master_name).write_bytes(master)
        old_identity = f"sha256:{hashlib.sha256(master).hexdigest()}"
        old_identities.append((vector["name"], old_identity))
        catalog_rows.append(f'{vector["name"]}\t{old_identity}\tmasters/{master_name}')
        for block_size in BLOCK_SIZES:
            publish = output / f'publish-{vector["name"]}-{block_size}'
            completed = run([
                "cargo", "run", "--locked", "-q", "-p", "miso-engine-stem-publisher", "--",
                "publish", "--input", str(output / "masters" / master_name),
                "--output-dir", str(publish), "--block-frames", str(block_size),
            ])
            identity, delivery_name = completed.stdout.strip().split("\t")
            assert identity == f"sha256:{hashlib.sha256(canonical_pcm).hexdigest()}"
            encoded = (publish / delivery_name).read_bytes()
            flac_name = f'{vector["name"]}-b{block_size}.flac'
            (output / "flac" / flac_name).write_bytes(encoded)
            manifest.append("\t".join([
                vector["name"], vector["bit_depth"], vector["channels"], str(DELIVERY_FRAMES),
                str(block_size), identity, f"pcm/{pcm_name}",
                f"flac/{flac_name}", hashlib.sha256(encoded).hexdigest(),
            ]))
            shutil.rmtree(publish)
    (output / "FLAC_VECTORS.tsv").write_text("\n".join(manifest) + "\n", encoding="utf-8")
    (mini / "catalog.tsv").write_text("\n".join(catalog_rows) + "\n", encoding="utf-8")

    embeddings = ["schema_version\t1", "old_identity\tkind\ttarget"]
    for name, old_identity in old_identities:
        embeddings.extend([
            f"{old_identity}\tmanifest_row\tcatalog/manifests/{name}",
            f"{old_identity}\tmix_document\tmixes/{name}.toml",
            f"{old_identity}\tserver_record\tserver/stems/{name}",
        ])
    embeddings.extend([
        f"{old_identities[0][1]}\tapp_fixture\tsrc/lib/mixer/__fixtures__/wide-open-msf1.json",
        f"{old_identities[1][1]}\tpackage_pin\tsrc/lib/mixer/sessions.ts:39",
        f"{old_identities[2][1]}\tpackage_pin\tsrc/lib/mixer/sessions.ts:56",
    ])
    (mini / "embeddings.tsv").write_text("\n".join(embeddings) + "\n", encoding="utf-8")
    run([
        "cargo", "run", "--locked", "-q", "-p", "miso-engine-catalog-migrate", "--",
        "migrate", "--catalog", str(mini / "catalog.tsv"),
        "--embeddings", str(mini / "embeddings.tsv"),
        "--output-dir", str(mini / "expected"),
    ])


def generated_paths(root: Path) -> list[Path]:
    paths: list[Path] = []
    for name in GENERATED_ROOTS:
        path = root / name
        if path.is_file():
            paths.append(path)
        elif path.is_dir():
            paths.extend(item for item in path.rglob("*") if item.is_file())
    return sorted(paths)


def write_generated(generated: Path) -> None:
    for source in generated_paths(generated):
        relative = source.relative_to(generated)
        destination = HERE / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(source.read_bytes())


def check_generated(generated: Path) -> None:
    actual = [path.relative_to(HERE) for path in generated_paths(HERE)]
    expected = [path.relative_to(generated) for path in generated_paths(generated)]
    assert actual == expected, f"generated path drift: actual={actual}, expected={expected}"
    for relative in expected:
        assert (HERE / relative).read_bytes() == (generated / relative).read_bytes(), relative


def main() -> None:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="miso-flac-delivery-") as temporary:
        generated = Path(temporary) / "v1"
        generated.mkdir()
        build_tree(generated)
        if args.write:
            write_generated(generated)
        else:
            check_generated(generated)


if __name__ == "__main__":
    main()
