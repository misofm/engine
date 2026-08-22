#!/usr/bin/env python3
"""Independent standard-library reference for canonical effect package V1 fixtures."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import pathlib
import struct
import sys

MAGIC = b"MISOEPKG"
HEADER = 96
RECORD = 72
KINDS = {"source": 1, "core-wasm": 2, "target-native": 3}
KIND_NAMES = {value: key for key, value in KINDS.items()}
DESCRIPTOR_DOMAIN = b"miso.engine.effect-descriptor.identity.v1\0"
ROOT = pathlib.Path(__file__).resolve().parent.parent
FIXTURES = ROOT / "fixtures" / "effect-package" / "v1"
DESCRIPTORS = ROOT / "fixtures" / "effect-descriptor" / "v1"


class Rejected(Exception):
    def __init__(self, code: str):
        super().__init__(code)
        self.code = code


def u16(data: bytes, offset: int) -> int:
    return struct.unpack_from("<H", data, offset)[0]


def u32(data: bytes, offset: int) -> int:
    return struct.unpack_from("<I", data, offset)[0]


def u64(data: bytes, offset: int) -> int:
    return struct.unpack_from("<Q", data, offset)[0]


def align8(value: int) -> int:
    return (value + 7) & ~7


def sha(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def descriptor_identity(data: bytes) -> bytes:
    return sha(DESCRIPTOR_DOMAIN + struct.pack("<Q", len(data)) + data)


def valid_path(value: bytes) -> bool:
    allowed = b"abcdefghijklmnopqrstuvwxyz0123456789._-"
    segments = value.split(b"/")
    return (
        1 <= len(value) <= 255
        and value.isascii()
        and all(segment not in (b"", b".", b"..") for segment in segments)
        and all(byte in allowed for segment in segments for byte in segment)
    )


def valid_native_target(value: bytes) -> bool:
    allowed = b"abcdefghijklmnopqrstuvwxyz0123456789_"
    components = value.split(b"-")
    return (
        1 <= len(value) <= 127
        and value.isascii()
        and not value.startswith(b"wasm32-")
        and len(components) in (3, 4)
        and all(component and all(byte in allowed for byte in component) for component in components)
    )


def valid_token(value: bytes) -> bool:
    return (
        1 <= len(value) <= 32
        and value.isascii()
        and 97 <= value[0] <= 122
        and all(97 <= byte <= 122 or 48 <= byte <= 57 or byte == 45 for byte in value[1:])
    )


def valid_features(value: bytes) -> bool:
    if not value:
        return True
    if len(value) > 255:
        return False
    tokens = value.split(b",")
    return all(valid_token(token) for token in tokens) and all(
        left < right for left, right in zip(tokens, tokens[1:])
    )


def artifact_key(artifact: dict) -> tuple:
    return (
        artifact["kind"],
        artifact["target"].encode(),
        artifact["features"].encode(),
        artifact["path"].encode(),
    )


def validate_artifact(artifact: dict) -> None:
    path = artifact["path"].encode()
    target = artifact["target"].encode()
    features = artifact["features"].encode()
    if not valid_path(path):
        raise Rejected("Path")
    if artifact["kind"] == 1:
        target_ok = not target
    elif artifact["kind"] == 2:
        target_ok = target == b"wasm32-unknown-unknown"
    elif artifact["kind"] == 3:
        target_ok = valid_native_target(target)
    else:
        raise Rejected("Enum")
    if not target_ok:
        raise Rejected("Target")
    if (artifact["kind"] == 1 and features) or not valid_features(features):
        raise Rejected("Features")
    if not artifact["content"]:
        raise Rejected("Length")


def encode(descriptor: bytes, artifacts: list[dict], require_source: bool = True) -> bytes:
    if len(descriptor) > 4_194_304 or not descriptor.startswith(b"MISOEFD1"):
        raise Rejected("Descriptor")
    canonical = sorted(artifacts, key=artifact_key)
    for artifact in canonical:
        validate_artifact(artifact)
    if any(artifact_key(left) == artifact_key(right) for left, right in zip(canonical, canonical[1:])):
        raise Rejected("Order")
    if require_source and not any(artifact["kind"] == 1 for artifact in canonical):
        raise Rejected("Unavailable")
    table = bytearray()
    contents = bytearray()
    for artifact in canonical:
        path = artifact["path"].encode()
        target = artifact["target"].encode()
        features = artifact["features"].encode()
        record = bytearray(RECORD)
        struct.pack_into("<I", record, 0, artifact["kind"])
        struct.pack_into("<III", record, 8, len(path), len(target), len(features))
        struct.pack_into("<QQ", record, 24, len(contents), len(artifact["content"]))
        record[40:72] = sha(artifact["content"])
        record += path + target + features
        record += bytes(align8(len(record)) - len(record))
        table += record
        contents += artifact["content"]
    manifest = HEADER + len(descriptor) + len(table)
    total = manifest + len(contents)
    if manifest > 16_777_216 or total > 268_435_456 or len(canonical) > 4096:
        raise Rejected("Limit")
    header = bytearray(HEADER)
    header[:8] = MAGIC
    struct.pack_into("<HHI", header, 8, 1, HEADER, 0)
    struct.pack_into("<QQQQI", header, 16, total, len(descriptor), len(table), len(contents), len(canonical))
    header[56:88] = descriptor_identity(descriptor)
    return bytes(header + descriptor + table + contents)


def parse_records(data: bytes, table_start: int, table_bytes: int, count: int) -> list[dict]:
    table = data[table_start : table_start + table_bytes]
    records = []
    cursor = 0
    for index in range(count):
        if len(table) - cursor < RECORD:
            raise Rejected("Length")
        path_len, target_len, feature_len = struct.unpack_from("<III", table, cursor + 8)
        end = RECORD + path_len + target_len + feature_len
        padded = align8(end)
        if cursor + padded > len(table):
            raise Rejected("Length")
        path_start = cursor + RECORD
        target_start = path_start + path_len
        feature_start = target_start + target_len
        records.append(
            {
                "artifact_index": index,
                "record_offset": table_start + cursor,
                "kind": u32(table, cursor),
                "path_bytes": table[path_start:target_start],
                "target_bytes": table[target_start:feature_start],
                "features_bytes": table[feature_start : feature_start + feature_len],
                "content_offset": u64(table, cursor + 24),
                "content_length": u64(table, cursor + 32),
                "content_sha256": table[cursor + 40 : cursor + 72],
                "padding": table[cursor + end : cursor + padded],
                "reserved_a": table[cursor + 4 : cursor + 8],
                "reserved_b": table[cursor + 20 : cursor + 24],
            }
        )
        cursor += padded
    if cursor != len(table):
        raise Rejected("Length")
    return records


def verify(data: bytes) -> dict:
    if len(data) > 268_435_456:
        raise Rejected("Limit")
    if len(data) < HEADER:
        raise Rejected("Header")
    total, descriptor_bytes, table_bytes, content_bytes = struct.unpack_from("<QQQQ", data, 16)
    count = u32(data, 48)
    if descriptor_bytes > 4_194_304 or count > 4096:
        raise Rejected("Limit")
    manifest = HEADER + descriptor_bytes + table_bytes
    if manifest > 16_777_216:
        raise Rejected("Limit")
    if data[:8] != MAGIC or u16(data, 8) != 1 or u16(data, 10) != HEADER:
        raise Rejected("Header")
    if any(data[12:16]) or any(data[52:56]) or any(data[88:96]):
        raise Rejected("Reserved")
    if total != manifest + content_bytes or total != len(data) or table_bytes & 7:
        raise Rejected("Length")
    descriptor_end = HEADER + descriptor_bytes
    descriptor = data[HEADER:descriptor_end]
    if not descriptor.startswith(b"MISOEFD1") or data[56:88] != descriptor_identity(descriptor):
        raise Rejected("Descriptor")
    records = parse_records(data, descriptor_end, table_bytes, count)
    contents = data[manifest:]
    expected_offset = 0
    for record in records:
        if any(record["reserved_a"]) or any(record["reserved_b"]) or any(record["padding"]):
            raise Rejected("Reserved")
        if record["content_offset"] != expected_offset:
            raise Rejected("Offset")
        if record["content_length"] == 0:
            raise Rejected("Length")
        expected_offset += record["content_length"]
        if expected_offset > content_bytes:
            raise Rejected("Length")
    if expected_offset != content_bytes:
        raise Rejected("Length")
    for record in records:
        if record["kind"] not in KIND_NAMES:
            raise Rejected("Enum")
        if not valid_path(record["path_bytes"]):
            raise Rejected("Path")
        if record["kind"] == 1:
            target_ok = not record["target_bytes"]
        elif record["kind"] == 2:
            target_ok = record["target_bytes"] == b"wasm32-unknown-unknown"
        else:
            target_ok = valid_native_target(record["target_bytes"])
        if not target_ok:
            raise Rejected("Target")
        if (record["kind"] == 1 and record["features_bytes"]) or not valid_features(record["features_bytes"]):
            raise Rejected("Features")
        try:
            record["path"] = record["path_bytes"].decode("ascii")
            record["target"] = record["target_bytes"].decode("ascii")
            record["features"] = record["features_bytes"].decode("ascii")
        except UnicodeDecodeError as error:
            raise Rejected("Path") from error
    keys = [(record["kind"], record["target_bytes"], record["features_bytes"], record["path_bytes"]) for record in records]
    if any(left >= right for left, right in zip(keys, keys[1:])):
        raise Rejected("Order")
    for record in records:
        start = record["content_offset"]
        end = start + record["content_length"]
        record["content"] = contents[start:end]
        if sha(record["content"]) != record["content_sha256"]:
            raise Rejected("Hash")
    if not any(record["kind"] == 1 for record in records):
        raise Rejected("Unavailable")
    return {"descriptor": descriptor, "records": records}


def cid_binary(data: bytes) -> bytes:
    return b"\x01\x55\x12\x20" + sha(data)


def cid_text(data: bytes) -> str:
    return "b" + base64.b32encode(cid_binary(data)).decode("ascii").lower().rstrip("=")


def select(records: list[dict], kind: int, target: str, capabilities: list[str]) -> dict:
    encoded_caps = [capability.encode() for capability in capabilities]
    if any(not valid_token(token) for token in encoded_caps) or any(
        left >= right for left, right in zip(encoded_caps, encoded_caps[1:])
    ) or len(b",".join(encoded_caps)) > 255:
        raise Rejected("Features")
    matches = []
    for record in records:
        features = [] if not record["features"] else record["features"].split(",")
        if record["kind"] == kind and record["target"] == target and all(feature in capabilities for feature in features):
            matches.append(record)
    if not matches:
        raise Rejected("Unavailable")
    return min(matches, key=lambda record: (-len(record["features"].split(",")) if record["features"] else 0, record["features"], record["path"]))


def source(kind: str, path: str, content: bytes, target: str = "", features: str = "") -> dict:
    return {"kind": KINDS[kind], "path": path, "target": target, "features": features, "content": content}


def vector_definitions() -> list[dict]:
    descriptor_a = bytes.fromhex((DESCRIPTORS / "comprehensive-a.wire.hex").read_text())
    descriptor_b = bytes.fromhex((DESCRIPTORS / "comprehensive-b.wire.hex").read_text())
    return [
        {
            "name": "comprehensive-a",
            "descriptor_fixture": "comprehensive-a.wire.hex",
            "descriptor": descriptor_a,
            "artifacts": [
                source("target-native", "native/x86-fma.so", b"A/native/avx2+fma\x00\x81", "x86_64-unknown-linux-gnu", "avx2,fma"),
                source("source", "src/z.rs", b"A/source/z\n"),
                source("core-wasm", "wasm/simd.wasm", b"\x00asmA-simd128", "wasm32-unknown-unknown", "simd128"),
                source("source", "src/a.rs", b"A/source/a\n"),
                source("core-wasm", "wasm/base.wasm", b"\x00asmA-base", "wasm32-unknown-unknown"),
                source("core-wasm", "wasm/bulk.wasm", b"\x00asmA-bulk+simd", "wasm32-unknown-unknown", "bulk-memory,simd128"),
                source("target-native", "native/x86-base.so", b"A/native/base\x7fELF", "x86_64-unknown-linux-gnu"),
            ],
            "selections": [
                (2, "wasm32-unknown-unknown", [], "wasm/base.wasm"),
                (2, "wasm32-unknown-unknown", ["simd128"], "wasm/simd.wasm"),
                (2, "wasm32-unknown-unknown", ["bulk-memory", "simd128"], "wasm/bulk.wasm"),
                (3, "x86_64-unknown-linux-gnu", ["avx2", "fma"], "native/x86-fma.so"),
            ],
        },
        {
            "name": "comprehensive-b",
            "descriptor_fixture": "comprehensive-b.wire.hex",
            "descriptor": descriptor_b,
            "artifacts": [
                source("core-wasm", "module/core.wasm", b"\x00asmB-core-distinct", "wasm32-unknown-unknown", "simd128"),
                source("target-native", "module/arm64.dylib", b"B/native/arm64/distinct", "aarch64-apple-darwin", "neon"),
                source("source", "source/lib.rs", b"B/source/distinct\n"),
                source("core-wasm", "module/base.wasm", b"\x00asmB-base-distinct", "wasm32-unknown-unknown"),
            ],
            "selections": [
                (1, "", [], "source/lib.rs"),
                (2, "wasm32-unknown-unknown", [], "module/base.wasm"),
                (2, "wasm32-unknown-unknown", ["simd128"], "module/core.wasm"),
                (3, "aarch64-apple-darwin", ["neon"], "module/arm64.dylib"),
            ],
        },
    ]


def authoring_from_view(view: dict) -> list[dict]:
    return [
        {"kind": record["kind"], "path": record["path"], "target": record["target"], "features": record["features"], "content": record["content"]}
        for record in view["records"]
    ]


def expect_rejection(data: bytes, expected: str) -> None:
    try:
        verify(data)
    except Rejected as error:
        if error.code != expected:
            raise AssertionError(f"expected {expected}, received {error.code}") from error
    else:
        raise AssertionError(f"expected {expected}, package accepted")


def mutation_matrix(vector: dict, package: bytes, view: dict) -> None:
    def mutated(offset: int, xor: int = 1) -> bytes:
        changed = bytearray(package)
        changed[offset] ^= xor
        return bytes(changed)

    for offset, code in [(0, "Header"), (8, "Header"), (10, "Header"), (12, "Reserved"), (52, "Reserved"), (56, "Descriptor"), (88, "Reserved")]:
        expect_rejection(mutated(offset), code)
    for offset in (16, 24, 32, 40, 48):
        expect_rejection(mutated(offset), "Length")
    expect_rejection(package[:-1], "Length")
    expect_rejection(package + b"\0", "Length")
    expect_rejection(mutated(HEADER), "Descriptor")
    first = view["records"][0]["record_offset"]
    for relative in (8, 12, 16):
        changed = bytearray(package)
        struct.pack_into("<I", changed, first + relative, 0xFFFFFFFF)
        expect_rejection(bytes(changed), "Length")
    for relative, code in [(4, "Reserved"), (20, "Reserved"), (24, "Offset"), (32, "Length"), (40, "Hash")]:
        changed = bytearray(package)
        if relative == 32:
            struct.pack_into("<Q", changed, first + relative, 0)
        else:
            changed[first + relative] ^= 1
        expect_rejection(bytes(changed), code)
    changed = bytearray(package)
    struct.pack_into("<I", changed, first, 99)
    expect_rejection(bytes(changed), "Enum")
    changed = bytearray(package)
    changed[first + RECORD] = ord("S")
    expect_rejection(bytes(changed), "Path")
    padded_end = view["records"][0]["record_offset"] + align8(
        RECORD + len(view["records"][0]["path_bytes"]) + len(view["records"][0]["target_bytes"]) + len(view["records"][0]["features_bytes"])
    )
    unpadded_end = first + RECORD + len(view["records"][0]["path_bytes"]) + len(view["records"][0]["target_bytes"]) + len(view["records"][0]["features_bytes"])
    if unpadded_end < padded_end:
        expect_rejection(mutated(unpadded_end), "Reserved")
    core = next(record for record in view["records"] if record["kind"] == 2 and record["target_bytes"])
    changed = bytearray(package)
    changed[core["record_offset"] + RECORD + len(core["path_bytes"])] = ord("W")
    expect_rejection(bytes(changed), "Target")
    featured = next(record for record in view["records"] if record["features_bytes"])
    changed = bytearray(package)
    feature_at = featured["record_offset"] + RECORD + len(featured["path_bytes"]) + len(featured["target_bytes"])
    changed[feature_at] = ord("A")
    expect_rejection(bytes(changed), "Features")
    content_start = HEADER + len(view["descriptor"]) + u64(package, 32)
    expect_rejection(mutated(content_start), "Hash")
    sources = [record for record in view["records"] if record["kind"] == 1]
    if len(sources) >= 2 and len(sources[0]["path_bytes"]) == len(sources[1]["path_bytes"]):
        changed = bytearray(package)
        path_at = sources[0]["record_offset"] + RECORD
        changed[path_at : path_at + len(sources[0]["path_bytes"])] = sources[1]["path_bytes"]
        expect_rejection(bytes(changed), "Order")
    without_source = [artifact for artifact in authoring_from_view(view) if artifact["kind"] != 1]
    expect_rejection(encode(view["descriptor"], without_source, require_source=False), "Unavailable")

    changed_cids = set()
    base_cid = cid_binary(package)
    artifacts = authoring_from_view(view)
    content_changed = [dict(artifact) for artifact in artifacts]
    content_changed[0]["content"] += b"!"
    changed_cids.add(cid_binary(encode(view["descriptor"], content_changed)))
    path_changed = [dict(artifact) for artifact in artifacts]
    path_changed[0]["path"] = "changed/" + path_changed[0]["path"].replace("/", "-")
    changed_cids.add(cid_binary(encode(view["descriptor"], path_changed)))
    other_descriptor = bytes.fromhex((DESCRIPTORS / ("comprehensive-b.wire.hex" if vector["name"].endswith("a") else "comprehensive-a.wire.hex")).read_text())
    changed_cids.add(cid_binary(encode(other_descriptor, artifacts)))
    added = [dict(artifact) for artifact in artifacts]
    added.append(source("source", "extra/new.rs", b"extra-source"))
    changed_cids.add(cid_binary(encode(view["descriptor"], added)))
    native_changed = [dict(artifact) for artifact in artifacts]
    native = next(artifact for artifact in native_changed if artifact["kind"] == 3)
    native["target"] = "riscv64gc-unknown-linux-gnu"
    changed_cids.add(cid_binary(encode(view["descriptor"], native_changed)))
    features_changed = [dict(artifact) for artifact in artifacts]
    featured = next(artifact for artifact in features_changed if artifact["features"])
    featured["features"] += ",zz"
    changed_cids.add(cid_binary(encode(view["descriptor"], features_changed)))
    assert base_cid not in changed_cids and len(changed_cids) == 6


def render_vector(vector: dict) -> tuple[str, str]:
    package = encode(vector["descriptor"], vector["artifacts"])
    view = verify(package)
    assert encode(view["descriptor"], authoring_from_view(view)) == package
    for kind, target, capabilities, expected in vector["selections"]:
        assert select(view["records"], kind, target, capabilities)["path"] == expected
    mutation_matrix(vector, package, view)
    manifest = {
        "schema": "miso.effect-package.fixture.v1",
        "name": vector["name"],
        "descriptor_fixture": vector["descriptor_fixture"],
        "descriptor_identity_hex": descriptor_identity(vector["descriptor"]).hex(),
        "package_bytes": len(package),
        "package_sha256": sha(package).hex(),
        "cid_binary_hex": cid_binary(package).hex(),
        "cid_text": cid_text(package),
        "artifacts": [
            {
                "artifact_index": record["artifact_index"],
                "kind": KIND_NAMES[record["kind"]],
                "kind_value": record["kind"],
                "path": record["path"],
                "target": record["target"],
                "features": record["features"],
                "content_hex": record["content"].hex(),
                "content_sha256": record["content_sha256"].hex(),
            }
            for record in view["records"]
        ],
        "selections": [
            {"kind_value": kind, "target": target, "capabilities": capabilities, "expected_path": expected}
            for kind, target, capabilities, expected in vector["selections"]
        ],
    }
    return "\n".join(package.hex()[index : index + 64] for index in range(0, len(package.hex()), 64)) + "\n", json.dumps(manifest, indent=2, sort_keys=True) + "\n"


def expected_files() -> dict[str, str]:
    files = {}
    for vector in vector_definitions():
        package_hex, manifest = render_vector(vector)
        files[f"{vector['name']}.package.hex"] = package_hex
        files[f"{vector['name']}.json"] = manifest
    seals = []
    for name in sorted(files):
        seals.append(f"{hashlib.sha256(files[name].encode()).hexdigest()}  fixtures/effect-package/v1/{name}")
    files["MANIFEST.sha256"] = "\n".join(seals) + "\n"
    return files


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    arguments = parser.parse_args()
    files = expected_files()
    if arguments.write:
        FIXTURES.mkdir(parents=True, exist_ok=True)
        for name, content in files.items():
            (FIXTURES / name).write_text(content)
    else:
        for name, expected in files.items():
            path = FIXTURES / name
            if not path.is_file() or path.read_text() != expected:
                raise SystemExit(f"effect package V1 reference mismatch: {path}")
    print("effect package V1 independent reference: ok")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Rejected as error:
        raise SystemExit(f"unexpected package rejection: {error.code}") from error
