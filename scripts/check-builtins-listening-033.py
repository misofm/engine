#!/usr/bin/env python3
"""Strict stdlib-only Issue-033 record validator and descriptive statistics."""

from __future__ import annotations

import hashlib
import json
import math
import re
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

HEX40 = re.compile(r"[0-9a-f]{40}\Z")
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
TOKEN = re.compile(r"[0-9a-f]{32}\.wav\Z")
UTC = re.compile(r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z\Z")
FORBIDDEN_IDENTITY = re.compile(r"(?:synthetic|not-a-human|\bagent\b|placeholder|unassigned|pending)", re.I)
PROCEDURES = ("issue007-filter-abx-v1", "issue007-matrix-ramp-v1")
ROLES = ("filter-candidate", "filter-comparator", "matrix-candidate", "matrix-comparator")
ISSUE110 = {
    "builtins-benchmark.disposition.json": "361f3a4f612e88dcc8a6dcb9f810528b175a64fbf3eea07122024df7971f274f",
    "builtins-benchmark.jsonl": "8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc",
    "builtins-benchmark.preflight.json": "9a7a78748b32d8a7cdee1bf7e886e38e6a358f6dfd093d93bbd51bdac2eddaa0",
    "builtins-benchmark.raw.jsonl": "8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc",
    "builtins-benchmark.validator.stderr": "7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396",
    "completion.seal.json": "3ce39b2653d6b912b6ede083fe8479e46bcbce665095190bd94d15fe82ca238d",
    "miso_engine_builtins_bench": "a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912",
}
ZERO_COUNTERS = {
    "audio_playback_invocations": 0,
    "completed_listening_records": 0,
    "human_listening_sessions": 0,
    "human_trial_attempts": 0,
    "preflight_invocations": 1,
    "preparation_invocations": 1,
    "reveal_invocations": 0,
    "valid_human_responses": 0,
}
PRE_SEAL_COUNTERS = {**ZERO_COUNTERS, "preflight_invocations": 0, "preparation_invocations": 0}
PREFLIGHT_COUNTERS = {**ZERO_COUNTERS, "preparation_invocations": 0}
QUALIFICATION_AUTHORITIES = {"preparation", "responses", "reveal"}
COPIED_PACKET_INPUTS = {
    "public/FACILITATOR.md": "dsp-research/listening/issue033/FACILITATOR.md",
    "public/filter-preregistration.md": "dsp-research/listening/issue007-filter-abx-preregistration.md",
    "public/matrix-preregistration.md": "dsp-research/listening/issue007-matrix-ramp-preregistration.md",
    "public/preparation.schema.json": "dsp-research/listening/issue033/preparation.schema.json",
    "public/qualification.schema.json": "dsp-research/listening/issue033/qualification.schema.json",
    "public/response-form.jsonl": "dsp-research/listening/issue033/response-form.jsonl",
    "public/response.schema.json": "dsp-research/listening/issue033/response.schema.json",
    "public/reveal.schema.json": "dsp-research/listening/issue033/reveal.schema.json",
}


class Invalid(ValueError):
    pass


def exact_keys(value: dict, expected: set[str], context: str) -> None:
    if type(value) is not dict or set(value) != expected:
        raise Invalid(f"{context}: closed keys")


def canonical(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def load_canonical(path: Path) -> object:
    raw = path.read_bytes()
    try:
        text = raw.decode("utf-8", "strict")
        value = json.loads(text)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise Invalid(f"invalid UTF-8/JSON: {error}") from error
    if raw != canonical(value):
        raise Invalid("record is not canonical sorted-key JSON/LF")
    validate_strings(value)
    return value


def validate_strings(value: object) -> None:
    if isinstance(value, str):
        if any(ord(char) < 0x20 for char in value):
            raise Invalid("control character")
        if value.startswith("/") or re.search(r"(?:^|[ =])/[A-Za-z0-9_.-]", value):
            raise Invalid("absolute path")
    elif isinstance(value, list):
        for item in value:
            validate_strings(item)
    elif isinstance(value, dict):
        for key, item in value.items():
            validate_strings(key)
            validate_strings(item)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require_hash(value: object, context: str) -> str:
    if type(value) is not str or not HEX64.fullmatch(value):
        raise Invalid(f"{context}: SHA-256")
    return value


def validate_provenance(value: object) -> None:
    keys = {
        "conversion_command", "conversion_tool", "conversion_version", "license_or_permission",
        "permission_confirmed", "redistribution_status", "retention_reference", "rights_holder",
        "schema_version", "source_sha256",
    }
    exact_keys(value, keys, "provenance")
    if value["schema_version"] != 1 or value["permission_confirmed"] is not True:
        raise Invalid("provenance version/permission")
    require_hash(value["source_sha256"], "source")
    for key in keys - {"schema_version", "permission_confirmed", "source_sha256"}:
        text = value[key]
        if type(text) is not str or not text or len(text) > 512 or FORBIDDEN_IDENTITY.search(text):
            raise Invalid(f"provenance field: {key}")
    if value["redistribution_status"] not in ("redistributable", "private-no-redistribution"):
        raise Invalid("redistribution status")


def crc32c(data: bytes) -> int:
    crc = 0xFFFFFFFF
    for byte in data:
        crc ^= byte
        for _ in range(8):
            crc = (crc >> 1) ^ (0x82F63B78 if crc & 1 else 0)
    return (~crc) & 0xFFFFFFFF


def validate_source(path: Path, provenance_path: Path) -> None:
    provenance = load_canonical(provenance_path)
    validate_provenance(provenance)
    raw = path.read_bytes()
    if len(raw) != 48 + 480000 * 2 * 4 or raw[:8] != b"MISOEPCM":
        raise Invalid("source length/magic")
    version, header = struct.unpack_from("<HH", raw, 8)
    flags, rate, channels, encoding = struct.unpack_from("<IIHH", raw, 12)
    frames, payload = struct.unpack_from("<QQ", raw, 24)
    checksum, reserved = struct.unpack_from("<II", raw, 40)
    if (version, header, flags, rate, channels, encoding, frames, payload, reserved) != (
        1, 48, 0, 48000, 2, 1, 480000, 3840000, 0
    ):
        raise Invalid("source header")
    checked = bytearray(raw)
    checked[40:44] = b"\0\0\0\0"
    if crc32c(checked) != checksum:
        raise Invalid("source CRC-32C")
    if hashlib.sha256(raw).hexdigest() != provenance["source_sha256"]:
        raise Invalid("source/provenance hash")
    values = struct.iter_unpack("<f", memoryview(raw)[48:])
    for index, (sample,) in enumerate(values):
        frame = index % 480000
        if not math.isfinite(sample) or abs(sample) > 0.5:
            raise Invalid("source finite/peak")
        if (frame < 480 or frame >= 479520) and struct.pack("<f", sample) != b"\0\0\0\0":
            raise Invalid("source exact edge silence")


def validate_render_manifest(value: object) -> None:
    exact_keys(value, {"assignment_key_sha256", "probe_render_sha256", "schedule_sha256", "schema_version", "source_provenance_sha256", "source_sha256", "stimuli"}, "render manifest")
    if value["schema_version"] != 1:
        raise Invalid("render manifest version")
    for key in ("assignment_key_sha256", "schedule_sha256", "source_provenance_sha256", "source_sha256"):
        require_hash(value[key], key)
    probe = value["probe_render_sha256"]
    if type(probe) is not list or len(probe) != 4 or probe != sorted(probe) or len(set(probe)) != 4:
        raise Invalid("render probe hashes")
    for digest in probe:
        require_hash(digest, "render probe")
    stimuli = value["stimuli"]
    if type(stimuli) is not list or len(stimuli) != 4:
        raise Invalid("render stimuli")
    tokens = []
    for stimulus in stimuli:
        exact_keys(stimulus, {"bytes", "frames", "peak", "rms", "sha256", "token"}, "render stimulus")
        if type(stimulus["token"]) is not str or not TOKEN.fullmatch(stimulus["token"]):
            raise Invalid("render token")
        if stimulus["bytes"] != 3840044 or stimulus["frames"] != 480000:
            raise Invalid("render shape")
        if type(stimulus["rms"]) not in (float, int) or not math.isfinite(stimulus["rms"]) or stimulus["rms"] <= 0:
            raise Invalid("render RMS")
        if type(stimulus["peak"]) not in (float, int) or not math.isfinite(stimulus["peak"]) or not 0 <= stimulus["peak"] < 1:
            raise Invalid("render peak")
        require_hash(stimulus["sha256"], "render stimulus")
        tokens.append(stimulus["token"])
    if tokens != sorted(tokens) or len(set(tokens)) != 4:
        raise Invalid("render token order")
    encoded = canonical(value)
    if any(word.encode() in encoded for word in ROLES + ("seed", "source_path")):
        raise Invalid("render manifest mapping leakage")


def assemble_preparation(manifest_path: Path, commit: str, tree: str, output: Path) -> None:
    if not HEX40.fullmatch(commit) or not HEX40.fullmatch(tree):
        raise Invalid("candidate identity")
    manifest = load_canonical(manifest_path)
    validate_render_manifest(manifest)
    members = expected_packet_members(manifest)
    packet_root = manifest_path.parent.parent
    actual_nonself = {
        f"{directory.name}/{path.name}"
        for directory in (packet_root / "public", packet_root / "private")
        for path in directory.iterdir()
    }
    expected_nonself = set(members) - {"public/preparation.json"}
    if actual_nonself != expected_nonself:
        raise Invalid("preparation input membership")
    for member, tracked in COPIED_PACKET_INPUTS.items():
        if sha256(packet_root / member) != sha256(Path.cwd() / tracked):
            raise Invalid("copied packet input drift")
    member_hashes = {
        name: sha256(packet_root / name)
        for name in sorted(expected_nonself)
    }
    preparation = {
        "assignment_key_sha256": manifest["assignment_key_sha256"],
        "candidate_commit": commit,
        "candidate_tree": tree,
        "counters": dict(ZERO_COUNTERS),
        "evidence_kind": "machine_preparation",
        "issue110_artifacts": dict(ISSUE110),
        "packet_members": members,
        "packet_member_sha256": member_hashes,
        "probe_render_sha256": manifest["probe_render_sha256"],
        "record_id": "issue007-listening-preparation-v1",
        "render": {"frames":480000,"matrix_events":[48000,96000,144000,192000,240000,288000,336000,384000,432000],"quantum_frames":128,"sample_rate_hz":48000,"wave_format":"stereo-interleaved-f32le-riff44"},
        "schedule_sha256": manifest["schedule_sha256"],
        "schema_version": 1,
        "source_provenance_sha256": manifest["source_provenance_sha256"],
        "source_sha256": manifest["source_sha256"],
        "status": "prepared",
        "stimuli": manifest["stimuli"],
    }
    validate_preparation(preparation)
    descriptor = canonical(preparation)
    with output.open("xb") as destination:
        destination.write(descriptor)


def authority_projection(root: Path, include_binary: bool) -> dict:
    paths = {
        "cargo_lock": "Cargo.lock",
        "checker": "scripts/check-builtins-listening-033.sh",
        "facilitator": "dsp-research/listening/issue033/FACILITATOR.md",
        "filter_preregistration": "dsp-research/listening/issue007-filter-abx-preregistration.md",
        "fixtures_manifest": "fixtures/builtins/v1/MANIFEST.tsv",
        "legacy_checker": "scripts/check-builtins-listening.sh",
        "listening_template": "dsp-research/listening/TEMPLATE.md",
        "lifecycle": "scripts/test-builtins-listening-033.sh",
        "matrix_preregistration": "dsp-research/listening/issue007-matrix-ramp-preregistration.md",
        "policy_mutation": "scripts/test-builtins-listening-033-policy.sh",
        "preflight": "scripts/preflight-builtins-listening-033.sh",
        "prepare": "scripts/prepare-builtins-listening-033.sh",
        "probe": "fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm",
        "product": "crates/miso-engine-builtins/src/lib.rs",
        "product_compiler": "crates/miso-engine-builtins-compiler/src/lib.rs",
        "provenance_template": "dsp-research/listening/issue033/provenance.template.json",
        "qualification_schema": "dsp-research/listening/issue033/qualification.schema.json",
        "renderer": "tools/miso-engine-builtins-fixture/src/listening_main.rs",
        "response_form": "dsp-research/listening/issue033/response-form.jsonl",
        "response_schema": "dsp-research/listening/issue033/response.schema.json",
        "reveal_schema": "dsp-research/listening/issue033/reveal.schema.json",
        "preparation_schema": "dsp-research/listening/issue033/preparation.schema.json",
        "tool_manifest": "tools/miso-engine-builtins-fixture/Cargo.toml",
        "validator": "scripts/check-builtins-listening-033.py",
    }
    projection = {name: sha256(root / path) for name, path in paths.items()}
    inbox = root / "target/issue33/inbox"
    projection.update({
        "provenance": sha256(inbox / "provenance.json"),
        "seed": sha256(inbox / "seed.txt"),
        "source": sha256(inbox / "source.mepcm"),
    })
    projection["binary"] = sha256(root / "target/issue33/miso_engine_builtins_fixture_listening") if include_binary else None
    return projection


def validate_issue110(root: Path) -> None:
    directory = root / "target/issue110"
    if not directory.is_dir() or directory.is_symlink():
        raise Invalid("Issue-110 directory")
    if sorted(path.name for path in directory.iterdir()) != sorted(ISSUE110):
        raise Invalid("Issue-110 membership")
    for name, digest in ISSUE110.items():
        path = directory / name
        if path.is_symlink() or not path.is_file() or path.stat().st_nlink != 1 or sha256(path) != digest:
            raise Invalid("Issue-110 identity")
    if (directory / "builtins-benchmark.raw.jsonl").stat().st_ino == (directory / "builtins-benchmark.jsonl").stat().st_ino:
        raise Invalid("Issue-110 raw/accepted alias")


def validate_seal(kind: str, path: Path, root: Path, commit: str, tree: str) -> None:
    if kind not in ("preparation", "preflight") or not HEX40.fullmatch(commit) or not HEX40.fullmatch(tree):
        raise Invalid("seal invocation")
    validate_issue110(root)
    value = load_canonical(path)
    exact_keys(value, {"authorities", "branch", "candidate_commit", "candidate_tree", "counters", "issue", "issue110_artifacts", "kind", "schema_version"}, "seal")
    expected_kind = f"issue033_listening_{kind}_seal"
    expected_counters = PRE_SEAL_COUNTERS if kind == "preparation" else PREFLIGHT_COUNTERS
    if value != {
        "authorities": authority_projection(root, kind == "preflight"),
        "branch": "codex/listening-033",
        "candidate_commit": commit,
        "candidate_tree": tree,
        "counters": expected_counters,
        "issue": 33,
        "issue110_artifacts": ISSUE110,
        "kind": expected_kind,
        "schema_version": 1,
    }:
        raise Invalid("seal authority mismatch")


def validate_renderer_output(root: Path, exact_public: bool = True) -> dict:
    public = root / "public"
    private = root / "private"
    if root.is_symlink() or not root.is_dir() or any(
        directory.is_symlink() or not directory.is_dir()
        for directory in (public, private)
    ):
        raise Invalid("renderer directory shape")
    if public.stat().st_mode & 0o777 != 0o755 or private.stat().st_mode & 0o777 != 0o700:
        raise Invalid("renderer directory mode")
    manifest = load_canonical(public / "render-manifest.json")
    validate_render_manifest(manifest)
    public_names = sorted(path.name for path in public.iterdir())
    if exact_public and public_names != sorted(["render-manifest.json", *(item["token"] for item in manifest["stimuli"])]):
        raise Invalid("renderer public membership")
    if sorted(path.name for path in private.iterdir()) != ["assignment-key.json", "source-provenance.json"]:
        raise Invalid("renderer private membership")
    key_path = private / "assignment-key.json"
    key = load_canonical(key_path)
    validate_assignment_key(key)
    token_roles = key["token_roles"]
    if sorted(token_roles) != [item["token"] for item in manifest["stimuli"]]:
        raise Invalid("private token mapping")
    if sha256(key_path) != manifest["assignment_key_sha256"]:
        raise Invalid("assignment key commitment")
    schedule = {
        "filter_x_candidate": key["filter_x_candidate"],
        "matrix_candidate_first": key["matrix_candidate_first"],
        "schema_version": 1,
    }
    if hashlib.sha256(canonical(schedule)).hexdigest() != manifest["schedule_sha256"]:
        raise Invalid("schedule commitment")
    provenance_path = private / "source-provenance.json"
    validate_provenance(load_canonical(provenance_path))
    if sha256(provenance_path) != manifest["source_provenance_sha256"]:
        raise Invalid("provenance commitment")
    stimuli_by_token = {item["token"]: item for item in manifest["stimuli"]}
    for stimulus in manifest["stimuli"]:
        path = public / stimulus["token"]
        raw = path.read_bytes()
        if len(raw) != stimulus["bytes"] or hashlib.sha256(raw).hexdigest() != stimulus["sha256"]:
            raise Invalid("stimulus file identity")
        expected_header = (
            b"RIFF" + (3_840_036).to_bytes(4, "little") + b"WAVEfmt "
            + (16).to_bytes(4, "little") + (3).to_bytes(2, "little")
            + (2).to_bytes(2, "little") + (48_000).to_bytes(4, "little")
            + (384_000).to_bytes(4, "little") + (8).to_bytes(2, "little")
            + (32).to_bytes(2, "little") + b"data" + (3_840_000).to_bytes(4, "little")
        )
        if raw[:44] != expected_header:
            raise Invalid("stimulus WAVE")
        energy = 0.0
        peak = 0.0
        for lane_offset in (44, 48):
            for offset in range(lane_offset, len(raw), 8):
                sample = struct.unpack_from("<f", raw, offset)[0]
                if not math.isfinite(sample):
                    raise Invalid("stimulus finite samples")
                energy += sample * sample
                peak = max(peak, abs(sample))
        rms = math.sqrt(energy / 960_000)
        if rms != stimulus["rms"] or peak != stimulus["peak"]:
            raise Invalid("stimulus metric mismatch")
    role_metrics = {
        role: stimuli_by_token[token]
        for token, role in token_roles.items()
    }
    for prefix in ("filter", "matrix"):
        first = role_metrics[f"{prefix}-candidate"]["rms"]
        second = role_metrics[f"{prefix}-comparator"]["rms"]
        difference_db = abs(20.0 * math.log10(first / second))
        if not math.isfinite(difference_db) or difference_db > 0.1:
            raise Invalid("stimulus RMS match")
    for directory, expected_mode in ((public, 0o444), (private, 0o600)):
        for path in directory.iterdir():
            if path.is_symlink() or not path.is_file() or path.stat().st_nlink != 1 or path.stat().st_mode & 0o777 != expected_mode:
                raise Invalid("renderer output shape/mode")
    return manifest


def validate_assignment_key(key: object) -> None:
    exact_keys(key, {"filter_x_candidate", "matrix_candidate_first", "schema_version", "seed", "token_roles"}, "assignment key")
    if key["schema_version"] != 1 or type(key["seed"]) is not str or not re.fullmatch(r"0|[1-9][0-9]{0,19}", key["seed"]) or int(key["seed"]) > 2**64 - 1:
        raise Invalid("private seed")
    for name in ("filter_x_candidate", "matrix_candidate_first"):
        values = key[name]
        if type(values) is not list or len(values) != 20 or any(type(item) is not bool for item in values) or sum(values) != 10:
            raise Invalid("balanced schedule")
    token_roles = key["token_roles"]
    if type(token_roles) is not dict or sorted(token_roles.values()) != sorted(ROLES) or any(
        type(token) is not str or not TOKEN.fullmatch(token) for token in token_roles
    ):
        raise Invalid("private token mapping")


def validate_packet(root: Path) -> None:
    public = root / "public"
    private = root / "private"
    if root.is_symlink() or not root.is_dir() or any(
        directory.is_symlink() or not directory.is_dir()
        for directory in (public, private)
    ):
        raise Invalid("packet directory shape")
    manifest = validate_renderer_output(root, exact_public=False)
    preparation = load_canonical(public / "preparation.json")
    validate_preparation(preparation)
    if any(preparation[key] != manifest[key] for key in (
        "assignment_key_sha256", "probe_render_sha256", "schedule_sha256",
        "source_provenance_sha256", "source_sha256", "stimuli"
    )):
        raise Invalid("manifest/preparation mismatch")
    actual_members = {}
    for directory, label in ((public, "public-0444"), (private, "private-0600")):
        for path in directory.iterdir():
            if path.is_symlink() or not path.is_file() or path.stat().st_nlink != 1:
                raise Invalid("packet file shape")
            actual_members[f"{directory.name}/{path.name}"] = label
            expected_mode = 0o444 if directory == public else 0o600
            if path.stat().st_mode & 0o777 != expected_mode:
                raise Invalid("packet mode")
    if actual_members != preparation["packet_members"]:
        raise Invalid("packet closed membership")
    validate_packet_member_digests(root, preparation)
    for path in public.iterdir():
        if path.suffix != ".wav":
            raw = path.read_bytes()
            if any(word.encode() in raw for word in ("\"seed\"", "source_path")):
                raise Invalid("public mapping leakage")


def write_seal(kind: str, path: Path, root: Path, commit: str, tree: str) -> None:
    include_binary = kind == "preflight"
    if kind not in ("preparation", "preflight"):
        raise Invalid("seal kind")
    value = {
        "authorities": authority_projection(root, include_binary),
        "branch": "codex/listening-033",
        "candidate_commit": commit,
        "candidate_tree": tree,
        "counters": PRE_SEAL_COUNTERS if kind == "preparation" else PREFLIGHT_COUNTERS,
        "issue": 33,
        "issue110_artifacts": ISSUE110,
        "kind": f"issue033_listening_{kind}_seal",
        "schema_version": 1,
    }
    with path.open("xb") as destination:
        destination.write(canonical(value))
    validate_seal(kind, path, root, commit, tree)


def validate_preparation(value: object) -> None:
    expected = {
        "assignment_key_sha256", "candidate_commit", "candidate_tree", "counters", "evidence_kind",
        "issue110_artifacts", "packet_member_sha256", "packet_members", "probe_render_sha256", "record_id", "render",
        "schedule_sha256", "schema_version", "source_provenance_sha256", "source_sha256", "status",
        "stimuli",
    }
    exact_keys(value, expected, "preparation")
    if (value["schema_version"], value["record_id"], value["evidence_kind"], value["status"]) != (
        1, "issue007-listening-preparation-v1", "machine_preparation", "prepared"
    ):
        raise Invalid("preparation identity")
    if type(value["candidate_commit"]) is not str or not HEX40.fullmatch(value["candidate_commit"]):
        raise Invalid("candidate commit")
    if type(value["candidate_tree"]) is not str or not HEX40.fullmatch(value["candidate_tree"]):
        raise Invalid("candidate tree")
    for key in ("assignment_key_sha256", "schedule_sha256", "source_provenance_sha256", "source_sha256"):
        require_hash(value[key], key)
    if value["issue110_artifacts"] != ISSUE110 or value["counters"] != ZERO_COUNTERS:
        raise Invalid("authority/counters")
    exact_keys(value["render"], {"frames", "matrix_events", "quantum_frames", "sample_rate_hz", "wave_format"}, "render")
    if value["render"] != {
        "frames": 480000,
        "matrix_events": [48000, 96000, 144000, 192000, 240000, 288000, 336000, 384000, 432000],
        "quantum_frames": 128,
        "sample_rate_hz": 48000,
        "wave_format": "stereo-interleaved-f32le-riff44",
    }:
        raise Invalid("render contract")
    stimuli = value["stimuli"]
    if type(stimuli) is not list or len(stimuli) != 4:
        raise Invalid("stimulus count")
    tokens = []
    hashes = set()
    for stimulus in stimuli:
        exact_keys(stimulus, {"bytes", "frames", "peak", "rms", "sha256", "token"}, "stimulus")
        if type(stimulus["token"]) is not str or not TOKEN.fullmatch(stimulus["token"]):
            raise Invalid("opaque token")
        if stimulus["bytes"] != 3_840_044 or stimulus["frames"] != 480_000:
            raise Invalid("stimulus shape")
        if type(stimulus["rms"]) not in (float, int) or not math.isfinite(stimulus["rms"]) or stimulus["rms"] <= 0:
            raise Invalid("stimulus RMS")
        if type(stimulus["peak"]) not in (float, int) or not math.isfinite(stimulus["peak"]) or not 0 <= stimulus["peak"] < 1:
            raise Invalid("stimulus peak")
        hashes.add(require_hash(stimulus["sha256"], "stimulus"))
        tokens.append(stimulus["token"])
    if tokens != sorted(tokens) or len(set(tokens)) != 4 or len(hashes) != 4:
        raise Invalid("stimulus order/uniqueness")
    probe = value["probe_render_sha256"]
    if type(probe) is not list or len(probe) != 4 or probe != sorted(probe) or len(set(probe)) != 4:
        raise Invalid("probe hashes")
    for digest in probe:
        require_hash(digest, "probe")
    members = value["packet_members"]
    if members != expected_packet_members({"stimuli": stimuli}):
        raise Invalid("packet members")
    member_hashes = value["packet_member_sha256"]
    if type(member_hashes) is not dict or set(member_hashes) != set(members) - {"public/preparation.json"}:
        raise Invalid("packet member digest keys")
    for digest in member_hashes.values():
        require_hash(digest, "packet member")
    public_text = canonical({key: value[key] for key in value})
    for forbidden in ROLES + ("seed", "source_path"):
        if forbidden.encode() in public_text:
            raise Invalid("public mapping leakage")


def expected_packet_members(manifest: dict) -> dict[str, str]:
    token_members = {f"public/{item['token']}": "public-0444" for item in manifest["stimuli"]}
    return {
        **token_members,
        "private/assignment-key.json": "private-0600",
        "private/source-provenance.json": "private-0600",
        "public/FACILITATOR.md": "public-0444",
        "public/filter-preregistration.md": "public-0444",
        "public/matrix-preregistration.md": "public-0444",
        "public/preparation.json": "public-0444",
        "public/preparation.schema.json": "public-0444",
        "public/qualification.schema.json": "public-0444",
        "public/render-manifest.json": "public-0444",
        "public/response-form.jsonl": "public-0444",
        "public/response.schema.json": "public-0444",
        "public/reveal.schema.json": "public-0444",
    }


def validate_packet_member_digests(root: Path, preparation: dict) -> None:
    actual_hashes = {
        name: sha256(root / name)
        for name in sorted(preparation["packet_members"])
        if name != "public/preparation.json"
    }
    if actual_hashes != preparation["packet_member_sha256"]:
        raise Invalid("packet member digest")


def load_responses(path: Path) -> list[dict]:
    raw = path.read_bytes()
    if not raw or not raw.endswith(b"\n") or b"\r" in raw:
        raise Invalid("response JSONL framing")
    rows = []
    offset = 0
    for line in raw.splitlines(keepends=True):
        try:
            row = json.loads(line.decode("utf-8", "strict"))
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            raise Invalid("response JSON") from error
        if line != canonical(row):
            raise Invalid("response canonical order")
        validate_strings(row)
        validate_response(row)
        rows.append(row)
        offset += len(line)
    if [row["sequence"] for row in rows] != list(range(1, len(rows) + 1)):
        raise Invalid("response sequence")
    for procedure in PROCEDURES:
        selected = [row for row in rows if row["procedure"] == procedure]
        valid = [row for row in selected if row["valid"]]
        if sorted(row["logical_trial"] for row in valid) != list(range(1, 21)):
            raise Invalid("valid logical trials")
        for trial in range(1, 21):
            trial_rows = [row for row in selected if row["logical_trial"] == trial]
            attempts = [row["attempt"] for row in trial_rows]
            if (
                attempts != list(range(1, len(attempts) + 1))
                or len(attempts) > 2
                or not trial_rows[-1]["valid"]
                or any(row["valid"] for row in trial_rows[:-1])
            ):
                raise Invalid("attempt ordering")
    return rows


def validate_response(row: object) -> None:
    keys = {"answer", "attempt", "confidence", "logical_trial", "observation", "procedure", "reason", "schema_version", "sequence", "valid"}
    exact_keys(row, keys, "response")
    if row["schema_version"] != 1 or row["procedure"] not in PROCEDURES:
        raise Invalid("response identity")
    for key, low, high in (("logical_trial", 1, 20), ("attempt", 1, 2), ("sequence", 1, 80)):
        if type(row[key]) is not int or not low <= row[key] <= high:
            raise Invalid(key)
    if type(row["valid"]) is not bool or type(row["observation"]) is not str or len(row["observation"]) > 512:
        raise Invalid("response type")
    if FORBIDDEN_IDENTITY.search(row["observation"]):
        raise Invalid("fabricated response marker")
    if row["valid"]:
        if row["reason"] is not None or row["answer"] not in ("A", "B") or type(row["confidence"]) is not int or not 0 <= row["confidence"] <= 100:
            raise Invalid("valid response payload")
    elif (
        type(row["reason"]) is not str
        or not row["reason"]
        or len(row["reason"]) > 512
        or FORBIDDEN_IDENTITY.search(row["reason"])
        or row["answer"] is not None
        or row["confidence"] is not None
    ):
        raise Invalid("invalid attempt payload")


def validate_reveal(value: object, response_hash: str | None = None, key: object | None = None, key_hash: str | None = None) -> None:
    keys = {"assignment_key_sha256", "conditions", "record_id", "response_sha256", "reveal_utc", "schema_version", "signoffs", "token_roles", "trials"}
    exact_keys(value, keys, "reveal")
    if value["schema_version"] != 1 or value["record_id"] != "issue007-listening-reveal-v1" or type(value["reveal_utc"]) is not str or not UTC.fullmatch(value["reveal_utc"]):
        raise Invalid("reveal identity")
    if response_hash is not None and value["response_sha256"] != response_hash:
        raise Invalid("post-reveal response drift")
    if key_hash is not None and value["assignment_key_sha256"] != key_hash:
        raise Invalid("assignment key drift")
    require_hash(value["response_sha256"], "response")
    require_hash(value["assignment_key_sha256"], "key")
    roles = value["token_roles"]
    if type(roles) is not dict or len(roles) != 4 or sorted(roles.values()) != sorted(ROLES) or any(not TOKEN.fullmatch(token) for token in roles):
        raise Invalid("token role mapping")
    trials = value["trials"]
    if type(trials) is not list or len(trials) != 40:
        raise Invalid("reveal trials")
    expected_identities = [(procedure, trial) for procedure in PROCEDURES for trial in range(1, 21)]
    identities = []
    for trial in trials:
        exact_keys(trial, {"assignment", "logical_trial", "procedure"}, "reveal trial")
        identity = (trial["procedure"], trial["logical_trial"])
        if trial["procedure"] not in PROCEDURES or type(trial["logical_trial"]) is not int or not 1 <= trial["logical_trial"] <= 20:
            raise Invalid("reveal trial identity")
        allowed = ("x-candidate", "x-comparator") if trial["procedure"] == PROCEDURES[0] else ("candidate-first", "comparator-first")
        if trial["assignment"] not in allowed:
            raise Invalid("reveal assignment")
        identities.append(identity)
    if identities != expected_identities:
        raise Invalid("reveal order/completeness")
    if key is not None:
        validate_assignment_key(key)
        if roles != key["token_roles"]:
            raise Invalid("revealed token mapping")
        expected_assignments = [
            "x-candidate" if candidate else "x-comparator"
            for candidate in key["filter_x_candidate"]
        ] + [
            "candidate-first" if candidate_first else "comparator-first"
            for candidate_first in key["matrix_candidate_first"]
        ]
        if [trial["assignment"] for trial in trials] != expected_assignments:
            raise Invalid("revealed trial mapping")
    exact_keys(value["signoffs"], {"facilitator", "listener", "reveal_verifier"}, "signoffs")
    signoffs = list(value["signoffs"].values())
    if any(type(item) is not str or not item or len(item) > 128 or FORBIDDEN_IDENTITY.search(item) for item in signoffs) or len(set(signoffs)) != 3:
        raise Invalid("real distinct signoffs")
    condition_keys = {"calibration_level_method", "conflicts", "driver_mode", "environmental_notes", "playback_hardware", "room_or_headphone", "transducer"}
    exact_keys(value["conditions"], condition_keys, "playback conditions")
    for name, condition in value["conditions"].items():
        if type(condition) is not str or not condition or len(condition) > 512 or FORBIDDEN_IDENTITY.search(condition):
            raise Invalid(f"playback condition: {name}")


def validate_qualification(value: object) -> None:
    keys = {"adverse_observations", "authorities", "conclusion", "corrective_links", "counts", "deviations", "disposition", "record_id", "schema_version", "statistics"}
    exact_keys(value, keys, "qualification")
    if value["schema_version"] != 1 or value["record_id"] != "issue007-listening-qualification-v1" or value["disposition"] not in ("PASS", "FAIL"):
        raise Invalid("qualification identity")
    if type(value["conclusion"]) is not str or not value["conclusion"] or len(value["conclusion"]) > 1024 or FORBIDDEN_IDENTITY.search(value["conclusion"]):
        raise Invalid("qualification conclusion")
    for key in ("adverse_observations", "deviations", "corrective_links"):
        if type(value[key]) is not list or any(type(item) is not str or len(item) > 512 for item in value[key]):
            raise Invalid(key)
    if type(value["authorities"]) is not dict or set(value["authorities"]) != QUALIFICATION_AUTHORITIES:
        raise Invalid("qualification authorities")
    for digest in value["authorities"].values():
        require_hash(digest, "qualification authority")
    if type(value["counts"]) is not dict or any(type(count) is not int or count < 0 for count in value["counts"].values()):
        raise Invalid("qualification counts")
    count_keys = {"abx_correct", "filter_valid_responses", "human_listening_sessions", "matrix_candidate_preferred", "matrix_valid_responses", "total_attempts"}
    if set(value["counts"]) != count_keys:
        raise Invalid("qualification count keys")
    counts = value["counts"]
    if not 0 <= counts["abx_correct"] <= 20 or not 0 <= counts["matrix_candidate_preferred"] <= 20 or not 40 <= counts["total_attempts"] <= 80:
        raise Invalid("qualification count range")
    if counts["filter_valid_responses"] != 20 or counts["matrix_valid_responses"] != 20 or counts["human_listening_sessions"] != 2:
        raise Invalid("qualification completeness")
    statistics_keys = {"abx_p_denominator", "abx_p_numerator", "abx_p_two_sided", "abx_wilson_high", "abx_wilson_low", "matrix_wilson_high", "matrix_wilson_low"}
    if type(value["statistics"]) is not dict or set(value["statistics"]) != statistics_keys:
        raise Invalid("qualification statistics")
    numerator, denominator, p = exact_p(counts["abx_correct"])
    abx_low, abx_high = wilson(counts["abx_correct"])
    matrix_low, matrix_high = wilson(counts["matrix_candidate_preferred"])
    expected_statistics = {
        "abx_p_denominator": denominator,
        "abx_p_numerator": numerator,
        "abx_p_two_sided": format(p, ".17g"),
        "abx_wilson_high": format(abx_high, ".17g"),
        "abx_wilson_low": format(abx_low, ".17g"),
        "matrix_wilson_high": format(matrix_high, ".17g"),
        "matrix_wilson_low": format(matrix_low, ".17g"),
    }
    if value["statistics"] != expected_statistics:
        raise Invalid("qualification statistic values")
    if value["disposition"] == "PASS" and (value["deviations"] or value["adverse_observations"]):
        raise Invalid("unresolved PASS observation/deviation")


def validate_linked_qualification(
    value: object,
    preparation_path: Path,
    responses_path: Path,
    reveal_path: Path,
    key_path: Path,
) -> None:
    validate_qualification(value)
    validate_preparation(load_canonical(preparation_path))
    rows = load_responses(responses_path)
    reveal = load_canonical(reveal_path)
    key = load_canonical(key_path)
    validate_reveal(reveal, sha256(responses_path), key, sha256(key_path))
    expected_authorities = {
        "preparation": sha256(preparation_path),
        "responses": sha256(responses_path),
        "reveal": sha256(reveal_path),
    }
    if value["authorities"] != expected_authorities:
        raise Invalid("qualification authority linkage")
    assignments = {
        (trial["procedure"], trial["logical_trial"]): trial["assignment"]
        for trial in reveal["trials"]
    }
    valid_rows = [row for row in rows if row["valid"]]
    abx_correct = 0
    matrix_preferred = 0
    for row in valid_rows:
        assignment = assignments[(row["procedure"], row["logical_trial"])]
        if row["procedure"] == PROCEDURES[0]:
            expected = "A" if assignment == "x-candidate" else "B"
            abx_correct += row["answer"] == expected
        else:
            expected = "A" if assignment == "candidate-first" else "B"
            matrix_preferred += row["answer"] == expected
    expected_counts = {
        "abx_correct": abx_correct,
        "filter_valid_responses": 20,
        "human_listening_sessions": 2,
        "matrix_candidate_preferred": matrix_preferred,
        "matrix_valid_responses": 20,
        "total_attempts": len(rows),
    }
    if value["counts"] != expected_counts:
        raise Invalid("qualification linked counts")


def exact_p(k: int) -> tuple[int, int, float]:
    if type(k) is not int or not 0 <= k <= 20:
        raise Invalid("k")
    start = max(k, 20 - k)
    numerator = min(2 ** 20, 2 * sum(math.comb(20, i) for i in range(start, 21)))
    denominator = 2 ** 20
    fraction = Fraction(numerator, denominator)
    return fraction.numerator, fraction.denominator, float(fraction)


def wilson(k: int) -> tuple[float, float]:
    if type(k) is not int or not 0 <= k <= 20:
        raise Invalid("k")
    n = 20.0
    z = 1.959963984540054
    p = k / n
    denominator = 1.0 + z * z / n
    center = (p + z * z / (2.0 * n)) / denominator
    half = z * math.sqrt(p * (1.0 - p) / n + z * z / (4.0 * n * n)) / denominator
    return center - half, center + half


def decimal_wilson(k: int) -> tuple[Decimal, Decimal]:
    getcontext().prec = 60
    n = Decimal(20)
    z = Decimal("1.959963984540054")
    p = Decimal(k) / n
    denominator = Decimal(1) + z * z / n
    center = (p + z * z / (Decimal(2) * n)) / denominator
    half = z * (p * (Decimal(1) - p) / n + z * z / (Decimal(4) * n * n)).sqrt() / denominator
    return center - half, center + half


def base_preparation() -> dict:
    token_names = [f"{index:032x}.wav" for index in range(1, 5)]
    preparation = {
        "assignment_key_sha256": "a" * 64,
        "candidate_commit": "b" * 40,
        "candidate_tree": "c" * 40,
        "counters": dict(ZERO_COUNTERS),
        "evidence_kind": "machine_preparation",
        "issue110_artifacts": dict(ISSUE110),
        "probe_render_sha256": sorted(hashlib.sha256(role.encode()).hexdigest() for role in ROLES),
        "record_id": "issue007-listening-preparation-v1",
        "render": {"frames": 480000, "matrix_events": [48000,96000,144000,192000,240000,288000,336000,384000,432000], "quantum_frames": 128, "sample_rate_hz": 48000, "wave_format": "stereo-interleaved-f32le-riff44"},
        "schedule_sha256": "d" * 64,
        "schema_version": 1,
        "source_provenance_sha256": "e" * 64,
        "source_sha256": "f" * 64,
        "status": "prepared",
        "stimuli": [{"bytes": 3840044, "frames": 480000, "peak": 0.5, "rms": 0.1, "sha256": hashlib.sha256(token.encode()).hexdigest(), "token": token} for token in token_names],
    }
    preparation["packet_members"] = expected_packet_members(preparation)
    preparation["packet_member_sha256"] = {
        name: hashlib.sha256(name.encode()).hexdigest()
        for name in preparation["packet_members"]
        if name != "public/preparation.json"
    }
    return preparation


def self_test() -> None:
    for k in range(21):
        numerator, denominator, p = exact_p(k)
        oracle = Fraction(min(2 ** 20, 2 * sum(math.comb(20, i) for i in range(max(k, 20-k), 21))), 2 ** 20)
        if Fraction(numerator, denominator) != oracle or p != float(oracle):
            raise AssertionError("binomial oracle")
        actual = wilson(k)
        expected = decimal_wilson(k)
        if any(abs(a - float(e)) > 2e-16 for a, e in zip(actual, expected)):
            raise AssertionError("Wilson oracle")
        if any(len(format(value, ".17g")) == 0 for value in (p, *actual)):
            raise AssertionError("17-digit format")
    preparation = base_preparation()
    validate_preparation(preparation)
    import tempfile
    with tempfile.TemporaryDirectory() as directory:
        packet = Path(directory)
        for name in preparation["packet_member_sha256"]:
            path = packet / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(name.encode())
        validate_packet_member_digests(packet, preparation)
        changed_name = next(iter(preparation["packet_member_sha256"]))
        (packet / changed_name).write_bytes(b"changed")
        try:
            validate_packet_member_digests(packet, preparation)
        except Invalid:
            pass
        else:
            raise AssertionError("packet member drift accepted")
    mutations = []
    for key in preparation:
        changed = dict(preparation)
        changed.pop(key)
        mutations.append(changed)
    changed = dict(preparation); changed["extra"] = 1; mutations.append(changed)
    changed = json.loads(json.dumps(preparation)); changed["counters"]["human_trial_attempts"] = 1; mutations.append(changed)
    changed = json.loads(json.dumps(preparation)); changed["stimuli"][0]["token"] = "filter-candidate.wav"; mutations.append(changed)
    changed = json.loads(json.dumps(preparation)); changed["stimuli"].reverse(); mutations.append(changed)
    for changed in mutations:
        try:
            validate_preparation(changed)
        except Invalid:
            pass
        else:
            raise AssertionError("preparation mutation accepted")
    valid_row = {"answer":"A","attempt":1,"confidence":50,"logical_trial":1,"observation":"","procedure":PROCEDURES[0],"reason":None,"schema_version":1,"sequence":1,"valid":True}
    validate_response(valid_row)
    for key, bad in (("confidence",101),("attempt",3),("valid",1),("observation","agent response")):
        changed = dict(valid_row); changed[key] = bad
        try:
            validate_response(changed)
        except Invalid:
            pass
        else:
            raise AssertionError("response mutation accepted")
    rows = []
    for procedure in PROCEDURES:
        for trial in range(1, 21):
            rows.append({"answer":"A" if trial % 2 else "B","attempt":1,"confidence":50,"logical_trial":trial,"observation":"","procedure":procedure,"reason":None,"schema_version":1,"sequence":len(rows)+1,"valid":True})
    with tempfile.TemporaryDirectory() as directory:
        response_path = Path(directory) / "responses.jsonl"
        response_path.write_bytes(b"".join(canonical(row) for row in rows))
        load_responses(response_path)
        dual_rows = [
            {
                "answer": None,
                "attempt": 1,
                "confidence": None,
                "logical_trial": 1,
                "observation": "",
                "procedure": PROCEDURES[0],
                "reason": "interrupted presentation",
                "schema_version": 1,
                "sequence": 1,
                "valid": False,
            }
        ] + [
            {
                **row,
                "attempt": 2 if row["procedure"] == PROCEDURES[0] and row["logical_trial"] == 1 else row["attempt"],
                "sequence": row["sequence"] + 1,
            }
            for row in rows
        ]
        dual_path = Path(directory) / "dual.jsonl"
        dual_path.write_bytes(b"".join(canonical(row) for row in dual_rows))
        load_responses(dual_path)
        invalid_retry = [dict(row) for row in dual_rows]
        invalid_retry[0], invalid_retry[1] = invalid_retry[1], invalid_retry[0]
        invalid_retry[0]["attempt"] = 1
        invalid_retry[0]["sequence"] = 1
        invalid_retry[1]["attempt"] = 2
        invalid_retry[1]["sequence"] = 2
        invalid_retry_path = Path(directory) / "invalid-retry.jsonl"
        invalid_retry_path.write_bytes(b"".join(canonical(row) for row in invalid_retry))
        try:
            load_responses(invalid_retry_path)
        except Invalid:
            pass
        else:
            raise AssertionError("post-valid retry accepted")
        response_hash = sha256(response_path)
        tokens = [f"{index:032x}.wav" for index in range(1, 5)]
        key = {
            "filter_x_candidate": [True] * 10 + [False] * 10,
            "matrix_candidate_first": [True] * 10 + [False] * 10,
            "schema_version": 1,
            "seed": "42",
            "token_roles": dict(zip(tokens, ROLES)),
        }
        key_path = Path(directory) / "assignment-key.json"
        key_path.write_bytes(canonical(key))
        reveal = {
            "assignment_key_sha256": sha256(key_path),
            "conditions": {"calibration_level_method":"documented method","conflicts":"none declared","driver_mode":"exclusive mode","environmental_notes":"quiet session","playback_hardware":"interface model","room_or_headphone":"headphone session","transducer":"headphone model"},
            "record_id": "issue007-listening-reveal-v1",
            "response_sha256": response_hash,
            "reveal_utc": "2026-08-22T12:00:00Z",
            "schema_version": 1,
            "signoffs": {"facilitator":"person-f","listener":"person-l","reveal_verifier":"person-r"},
            "token_roles": dict(zip(tokens, ROLES)),
            "trials": [{"assignment":("x-candidate" if procedure == PROCEDURES[0] and trial <= 10 else "x-comparator" if procedure == PROCEDURES[0] else "candidate-first" if trial <= 10 else "comparator-first"),"logical_trial":trial,"procedure":procedure} for procedure in PROCEDURES for trial in range(1,21)],
        }
        validate_reveal(reveal, response_hash, key, sha256(key_path))
        reveal_mutations = []
        changed = dict(reveal); changed["extra"] = 1; reveal_mutations.append(changed)
        changed = json.loads(json.dumps(reveal)); changed["signoffs"]["listener"] = "person-f"; reveal_mutations.append(changed)
        changed = json.loads(json.dumps(reveal)); changed["conditions"]["driver_mode"] = "agent placeholder"; reveal_mutations.append(changed)
        changed = json.loads(json.dumps(reveal)); changed["trials"].pop(); reveal_mutations.append(changed)
        changed = json.loads(json.dumps(reveal)); changed["trials"][0]["assignment"] = "candidate-first"; reveal_mutations.append(changed)
        changed = json.loads(json.dumps(reveal)); changed["trials"][0], changed["trials"][1] = changed["trials"][1], changed["trials"][0]; reveal_mutations.append(changed)
        for changed in reveal_mutations:
            try:
                validate_reveal(changed, response_hash, key, sha256(key_path))
            except Invalid:
                pass
            else:
                raise AssertionError("reveal mutation accepted")
    numerator, denominator, p = exact_p(10)
    abx_low, abx_high = wilson(10)
    matrix_low, matrix_high = wilson(10)
    qualification = {
        "adverse_observations": [],
        "authorities": {"preparation": "a" * 64, "responses": "b" * 64, "reveal": "c" * 64},
        "conclusion": "Bounded description of the preregistered questions.",
        "corrective_links": [],
        "counts": {"abx_correct":10,"filter_valid_responses":20,"human_listening_sessions":2,"matrix_candidate_preferred":10,"matrix_valid_responses":20,"total_attempts":40},
        "deviations": [],
        "disposition": "PASS",
        "record_id": "issue007-listening-qualification-v1",
        "schema_version": 1,
        "statistics": {"abx_p_denominator":denominator,"abx_p_numerator":numerator,"abx_p_two_sided":format(p,".17g"),"abx_wilson_high":format(abx_high,".17g"),"abx_wilson_low":format(abx_low,".17g"),"matrix_wilson_high":format(matrix_high,".17g"),"matrix_wilson_low":format(matrix_low,".17g")},
    }
    validate_qualification(qualification)
    with tempfile.TemporaryDirectory() as directory:
        directory = Path(directory)
        preparation_path = directory / "preparation.json"
        responses_path = directory / "responses.jsonl"
        reveal_path = directory / "reveal.json"
        key_path = directory / "assignment-key.json"
        preparation_path.write_bytes(canonical(preparation))
        responses_path.write_bytes(b"".join(canonical(row) for row in rows))
        key_path.write_bytes(canonical(key))
        reveal["assignment_key_sha256"] = sha256(key_path)
        reveal["response_sha256"] = sha256(responses_path)
        reveal_path.write_bytes(canonical(reveal))
        linked = json.loads(json.dumps(qualification))
        linked["authorities"] = {
            "preparation": sha256(preparation_path),
            "responses": sha256(responses_path),
            "reveal": sha256(reveal_path),
        }
        validate_linked_qualification(
            linked, preparation_path, responses_path, reveal_path, key_path
        )
        for mutation in ("authority", "count"):
            changed = json.loads(json.dumps(linked))
            if mutation == "authority":
                changed["authorities"]["responses"] = "0" * 64
            else:
                changed["counts"]["abx_correct"] = 9
            try:
                validate_linked_qualification(
                    changed, preparation_path, responses_path, reveal_path, key_path
                )
            except Invalid:
                pass
            else:
                raise AssertionError("linked qualification mutation accepted")
    for path, bad in (("disposition","PENDING"),("statistics",{}),("adverse_observations",["unresolved"]),("counts",{**qualification["counts"],"filter_valid_responses":19})):
        changed = dict(qualification); changed[path] = bad
        try:
            validate_qualification(changed)
        except Invalid:
            pass
        else:
            raise AssertionError("qualification mutation accepted")
    print("Issue-033 validator/statistics self-test: PASS (21 count rows; format-only synthetic data)")


def main(arguments: list[str]) -> int:
    try:
        if arguments == ["--self-test"]:
            self_test()
        elif len(arguments) == 2 and arguments[0] == "--provenance":
            validate_provenance(load_canonical(Path(arguments[1])))
        elif len(arguments) == 3 and arguments[0] == "--source":
            validate_source(Path(arguments[1]), Path(arguments[2]))
        elif len(arguments) == 2 and arguments[0] == "--render-manifest":
            validate_render_manifest(load_canonical(Path(arguments[1])))
        elif len(arguments) == 5 and arguments[0] == "--assemble":
            assemble_preparation(Path(arguments[1]), arguments[2], arguments[3], Path(arguments[4]))
        elif len(arguments) == 6 and arguments[0] == "--seal":
            validate_seal(arguments[1], Path(arguments[2]), Path(arguments[3]), arguments[4], arguments[5])
        elif len(arguments) == 6 and arguments[0] == "--write-seal":
            write_seal(arguments[1], Path(arguments[2]), Path(arguments[3]), arguments[4], arguments[5])
        elif len(arguments) == 2 and arguments[0] == "--packet":
            validate_packet(Path(arguments[1]))
        elif len(arguments) == 2 and arguments[0] == "--renderer-output":
            validate_renderer_output(Path(arguments[1]))
        elif len(arguments) == 2 and arguments[0] == "--preparation":
            validate_preparation(load_canonical(Path(arguments[1])))
        elif len(arguments) == 2 and arguments[0] == "--responses":
            load_responses(Path(arguments[1]))
        elif len(arguments) == 4 and arguments[0] == "--reveal":
            value = load_canonical(Path(arguments[1]))
            key_path = Path(arguments[3])
            validate_reveal(value, sha256(Path(arguments[2])), load_canonical(key_path), sha256(key_path))
        elif len(arguments) == 6 and arguments[0] == "--qualification":
            validate_linked_qualification(
                load_canonical(Path(arguments[1])),
                Path(arguments[2]),
                Path(arguments[3]),
                Path(arguments[4]),
                Path(arguments[5]),
            )
        elif len(arguments) == 2 and arguments[0] == "--stats":
            k = int(arguments[1], 10)
            numerator, denominator, p = exact_p(k)
            low, high = wilson(k)
            print(json.dumps({"k":k,"n":20,"p_denominator":denominator,"p_numerator":numerator,"p_two_sided":format(p,".17g"),"wilson_high":format(high,".17g"),"wilson_low":format(low,".17g")}, sort_keys=True, separators=(",",":")))
        else:
            raise Invalid("usage: check-builtins-listening-033.py --self-test|--provenance FILE|--source MEPCM PROVENANCE|--render-manifest FILE|--assemble MANIFEST COMMIT TREE OUTPUT|--seal KIND FILE ROOT COMMIT TREE|--preparation FILE|--responses FILE|--reveal FILE RESPONSES KEY_FILE|--qualification FILE PREPARATION RESPONSES REVEAL KEY_FILE|--stats K")
    except (Invalid, OSError, ValueError) as error:
        print(f"Issue-033 validation failure: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
