#!/usr/bin/env python3
"""Semantically validate Issue-114 evidence independently of its checksum manifest."""

from __future__ import annotations

import hashlib
import json
import pathlib
import re
import sys


EXPECTED_ARTIFACTS = {
    "header": (8266, "83880c2fd7b5bc835425a5a64cae19c8a0bba17f49b4802b4033a8e7dfeac37c"),
    "linux-x86_64-static-library": (33321950, "3f1021ac1f980d821965e5e52aa5b839680f828838c57676e5454f6645955ea7"),
    "linux-x86_64-shared-library": (4126480, "cbb06cd39b71cdde728b87a839d5f9657ff34b49884c64c4b2e4f8aedb0f2bc2"),
    "linux-c11-static-consumer": (10410184, "f958f00b7332bfa7ce0c33211d3c07e0049982e9d9257a17ce52b1f1a790e630"),
    "linux-c11-shared-consumer": (21312, "6fcbecd45e43c435517683606d84329c0ec034f3668865808f1ae4829ffe20ae"),
    "linux-cpp17-static-consumer": (10410224, "aabf985097ed44ad3dd33bd6dad77918cfd50aacdb047e93a38f40e0e423b854"),
    "linux-cpp17-shared-consumer": (21344, "f73f6dcb2192f8730f92bffcf5c074df9b5c86e8dba378d276fb252d3099fbb3"),
}
EXPECTED_QUALIFICATION = {
    "product_build_invocations": "1",
    "consumer_fixture_corrections": "1",
    "initial_c11_static_exit": "13",
    "linux_consumer_passes": "4",
    "capi_protocol_test_invocations": "1",
    "capi_unit_tests": "18",
    "capi_external_tests": "3",
    "protocol_unit_tests": "93",
    "protocol_mutation_tests": "1",
    "capi_protocol_log_sha256": "626a33486a31f202dccedb20321c5b25a48b2e9a6a6087ee870652bd434b894c",
    "runner_corpus_invocations": "1",
    "runner_tests": "18",
    "runner_corpus_rows": "5",
    "runner_output_bytes_per_row": "8192",
    "runner_log_sha256": "ed2dd1e4d0e5d4ea0143adcc4bae06ce5c5c82155baf05e61eb0e967222aaec9",
    "capi_audit_invocations": "1",
    "capi_audit_calls": "100000",
    "realtime_audit_invocations": "1",
    "realtime_audit_blocks": "1000000",
    "benchmark_invocations": "0",
    "timing_invocations": "0",
    "playback_invocations": "0",
    "listening_invocations": "0",
    "browser_invocations": "0",
    "device_invocations": "0",
}
EXPECTED_RAW = {
    "ARTIFACTS.generated.sha256": (837, "243431963beeca54f4a0df6ecebfc367d4d1d245b79a7fb6287b58e4620262bc"),
    "static-symbols.txt": (425, "4a66dc4f68070198d4547b9e63ed299a01592d48c5c31685f631ebfb216b1dcb"),
    "shared-symbols.txt": (425, "4a66dc4f68070198d4547b9e63ed299a01592d48c5c31685f631ebfb216b1dcb"),
    "static-nm.txt": (2336050, "6bb1fd99e48351c821f6aa1454996c7a26ce7be6fd165b6183d8ffce1e2bcfdd"),
    "shared-nm.txt": (3358, "62dd1cc89e9fe0b9bf1e25f645dedf38319c5a07d0ced4d24ce8281e65b02c48"),
    "shared-readelf.txt": (6901, "0cf743d7a36e9cb0c95c4e6cb192f2a91ba08ec673aac02b21862351345a726b"),
    "shared-objdump.txt": (3834, "828b8dc90904573c4a450f2b2355e24f78992871d1ea35cf89fe543c88b830fd"),
    "logs/capi-build.log": (2876, "51af757d9ba3fea09b5bc8175f1a88413121de0a63a64ed28590982af3198d14"),
    "logs/capi-regressions.log": (14696, "626a33486a31f202dccedb20321c5b25a48b2e9a6a6087ee870652bd434b894c"),
    "logs/runner-corpus.log": (2243, "ed2dd1e4d0e5d4ea0143adcc4bae06ce5c5c82155baf05e61eb0e967222aaec9"),
    "logs/capi-audit.stderr": (2372, "372d1b5b961662f3a1edf39ea2e70f4d7a1b9c5306e52666eb24f8e94e922359"),
    "logs/realtime-build.log": (160, "74b042bc973ebbccf9813ac98d18239449795c9367df2c8df5a385e789176ca0"),
    "logs/c11-static.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "logs/c11-shared.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "logs/cpp17-static.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "logs/cpp17-shared.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "capi-audit.json": (349, "9bc0b8a1b8a032e0a29fef7154141646be65be29cd5c41901ce66d2666f6c408"),
    "realtime-audit.json": (247, "7d76eb7ce5dea04be6dba308db82cfa9691d18bb520eb499f56475e12e236eb8"),
    "realtime-trace.1760548": (4970, "918095df4d0a030fdce06aa4b84f5ae4267793113042a0c1d3c49200c389c240"),
}
EXPECTED_GATES = {
    "qualification-preflight", "qualification-final", "qualification-preserved",
    "qualification-mutations", "shell-syntax",
    "c11-syntax", "cpp17-syntax", "cargo-fmt", "cargo-clippy-deny-warnings",
    "cargo-rustdoc-deny-warnings", "capi-abi-check", "capi-abi-mutations",
    "runner-check", "runner-mutations", "runner-portability-check",
    "runner-portability-mutations", "realtime-check", "realtime-mutations",
    "workspace-check", "workspace-mutations", "diff-check", "source-artifact-scan",
}
HEX = re.compile(r"[0-9a-f]{64}")


def digest(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def table(path: pathlib.Path) -> list[list[str]]:
    rows = [line.split("\t") for line in path.read_text(encoding="utf-8").splitlines()]
    if not rows or rows[0] != ["schema_version", "1"]:
        raise ValueError(f"bad schema in {path}")
    return rows[1:]


def check_artifacts(fixture: pathlib.Path) -> None:
    rows = table(fixture / "ARTIFACTS.tsv")
    if len(rows) != len(EXPECTED_ARTIFACTS) or any(len(row) != 3 for row in rows):
        raise ValueError("artifact evidence membership differs")
    actual = {row[0]: (int(row[1]), row[2]) for row in rows if len(row) == 3}
    if actual != EXPECTED_ARTIFACTS:
        raise ValueError("artifact evidence differs from immutable observed values")


def check_symbols(fixture: pathlib.Path) -> set[str]:
    expected = set((fixture / "EXPECTED_SYMBOLS.txt").read_text(encoding="utf-8").splitlines())
    rows = table(fixture / "SYMBOLS.tsv")
    if len(rows) != 32 or any(len(row) not in {2, 3} for row in rows):
        raise ValueError("symbol evidence membership differs")
    counts = {row[0]: int(row[1]) for row in rows[:4]}
    if counts != {
        "static-defined-count": 14, "static-undefined-miso-prefix-count": 0,
        "shared-defined-count": 14, "shared-undefined-miso-prefix-count": 0,
    }:
        raise ValueError("symbol counters are not exact")
    for linkage in ("static", "shared"):
        names = {row[2] for row in rows[4:] if row[:2] == [linkage, "DEFINED"]}
        if names != expected or len(names) != 14:
            raise ValueError(f"{linkage} semantic symbol set differs")
    return expected


def expected_audits() -> list[dict[str, object]]:
    return [
        {"schema_version": 1, "kind": "issue022_capi_render_audit", "calls": 100000,
         "sample_rate_hz": 48000, "quantum_frames": 128, "stable_output_address": True,
         "pcm_digest": "37380b654988f7cc", "render_errors": 0, "allocations": 0,
         "deallocations": 0, "locks": 0, "feature_detection": 0, "logs": 0,
         "file_io": 0, "network_io": 0, "syscalls": 0, "panic_unwinds": 0,
         "total_violations": 0},
        {"schema_version": 1, "kind": "realtime_audit", "blocks": 1000000,
         "swaps_accepted": 2, "swaps_deferred": 1, "allocations": 0,
         "deallocations": 0, "locks": 0, "logs": 0, "file_io": 0,
         "network_io": 0, "syscalls": 0, "total_violations": 0,
         "armed_trace_syscalls": 0},
    ]


def check_audits(fixture: pathlib.Path) -> None:
    actual = [json.loads(line) for line in (fixture / "AUDITS.jsonl").read_text().splitlines()]
    if actual != expected_audits():
        raise ValueError("audit semantics differ from exact observed results")


def check_qualification(fixture: pathlib.Path) -> None:
    rows = table(fixture / "QUALIFICATION.tsv")
    if len(rows) != len(EXPECTED_QUALIFICATION) or any(len(row) != 2 for row in rows):
        raise ValueError("qualification evidence membership differs")
    actual = {row[0]: row[1] for row in rows}
    if actual != EXPECTED_QUALIFICATION:
        raise ValueError("qualification counters or log hashes differ")
    if actual["capi_protocol_log_sha256"] != EXPECTED_RAW["logs/capi-regressions.log"][1]:
        raise ValueError("CAPI result counters are not bound to the raw log")
    if actual["runner_log_sha256"] != EXPECTED_RAW["logs/runner-corpus.log"][1]:
        raise ValueError("runner result counters are not bound to the raw log")


def check_consumers(fixture: pathlib.Path) -> None:
    rows = table(fixture / "CONSUMER_RESULTS.tsv")
    if len(rows) != 4 or any(len(row) != 12 or row[0] != "result" for row in rows):
        raise ValueError("consumer result membership differs")
    artifacts = EXPECTED_ARTIFACTS
    expected_names = {
        "linux-c11-static": ("C11", "static", "linux-c11-static-consumer", "linux-x86_64-static-library"),
        "linux-c11-shared": ("C11", "shared", "linux-c11-shared-consumer", "linux-x86_64-shared-library"),
        "linux-cpp17-static": ("C++17", "static", "linux-cpp17-static-consumer", "linux-x86_64-static-library"),
        "linux-cpp17-shared": ("C++17", "shared", "linux-cpp17-shared-consumer", "linux-x86_64-shared-library"),
    }
    source_hash = digest(fixture / "runtime_consumer.c")
    seen = set()
    for row in rows:
        _, name, language, linkage, exit_code, status, source, header, library, size, binary, log = row
        if name not in expected_names or name in seen:
            raise ValueError("consumer result name differs")
        seen.add(name)
        exp_language, exp_linkage, binary_name, library_name = expected_names[name]
        if (language, linkage, exit_code, status) != (exp_language, exp_linkage, "0", "PASS"):
            raise ValueError("consumer result is not an exact successful exit")
        if source != source_hash or header != EXPECTED_ARTIFACTS["header"][1]:
            raise ValueError("consumer source/header binding differs")
        if library != artifacts[library_name][1] or (int(size), binary) != artifacts[binary_name]:
            raise ValueError("consumer binary/library binding differs")
        if log != hashlib.sha256(b"").hexdigest():
            raise ValueError("consumer success log binding differs")
    if seen != set(expected_names):
        raise ValueError("consumer result omitted")


def check_raw_table(fixture: pathlib.Path) -> None:
    rows = table(fixture / "RAW_EVIDENCE.tsv")
    if len(rows) != len(EXPECTED_RAW) or any(len(row) != 4 or row[0] != "raw" for row in rows):
        raise ValueError("raw evidence membership differs")
    actual = {row[1]: (int(row[2]), row[3]) for row in rows if len(row) == 4 and row[0] == "raw"}
    if actual != EXPECTED_RAW or any(not HEX.fullmatch(item[1]) for item in actual.values()):
        raise ValueError("raw evidence inventory differs from immutable staging")


def check_gates(fixture: pathlib.Path) -> None:
    rows = table(fixture / "GATES.tsv")
    if len(rows) != len(EXPECTED_GATES) or any(len(row) != 4 or row[0] != "gate" for row in rows):
        raise ValueError("strict gate membership differs")
    actual = {row[1]: row[2:] for row in rows if len(row) == 4 and row[0] == "gate"}
    if set(actual) != EXPECTED_GATES or any(value[0] != "PASS" or not value[1] for value in actual.values()):
        raise ValueError("strict gate record differs")
    if "-D warnings" not in actual["cargo-clippy-deny-warnings"][1]:
        raise ValueError("Clippy is not warning-denied")
    if "-D warnings" not in actual["cargo-rustdoc-deny-warnings"][1]:
        raise ValueError("rustdoc is not warning-denied")
    if "--check" not in actual["cargo-fmt"][1]:
        raise ValueError("format gate is not check-only")


def check_stage(root: pathlib.Path, expected_symbols: set[str]) -> None:
    stage = root / "target/capi-qualification/v1"
    if not stage.is_dir():
        raise ValueError("preserved qualification staging is absent")
    for relative, (size, sha) in EXPECTED_RAW.items():
        path = stage / relative
        if not path.is_file() or path.stat().st_size != size or digest(path) != sha:
            raise ValueError(f"preserved raw evidence drifted: {relative}")
    artifact_paths = {
        "header": stage / "installed/include/miso_engine_v2.h",
        "linux-x86_64-static-library": stage / "installed/lib/libmiso_engine_capi.a",
        "linux-x86_64-shared-library": stage / "installed/lib/libmiso_engine_capi.so",
        "linux-c11-static-consumer": stage / "bin/c11-static",
        "linux-c11-shared-consumer": stage / "bin/c11-shared",
        "linux-cpp17-static-consumer": stage / "bin/cpp17-static",
        "linux-cpp17-shared-consumer": stage / "bin/cpp17-shared",
    }
    for name, path in artifact_paths.items():
        size, sha = EXPECTED_ARTIFACTS[name]
        if path.stat().st_size != size or digest(path) != sha:
            raise ValueError(f"staged artifact differs: {name}")
    manifest_hashes = {
        pathlib.Path(line.split(maxsplit=1)[1]).name: line.split(maxsplit=1)[0]
        for line in (stage / "ARTIFACTS.generated.sha256").read_text().splitlines()
    }
    if set(manifest_hashes.values()) != {sha for _, sha in EXPECTED_ARTIFACTS.values()}:
        raise ValueError("generated artifact manifest does not bind the accepted artifacts")
    for linkage in ("static", "shared"):
        normalized = set((stage / f"{linkage}-symbols.txt").read_text().splitlines())
        if normalized != expected_symbols:
            raise ValueError(f"preserved {linkage} symbols differ")
        defined: set[str] = set()
        imported: set[str] = set()
        for line in (stage / f"{linkage}-nm.txt").read_text().splitlines():
            fields = line.split()
            if len(fields) < 2 or not fields[-1].startswith("miso_engine_v2_"):
                continue
            if fields[-2] in {"U", "u", "w", "v"}:
                imported.add(fields[-1])
            else:
                defined.add(fields[-1])
        if defined != expected_symbols or imported:
            raise ValueError(f"preserved {linkage} nm definitions/imports differ")
    result_pattern = re.compile(
        r"^test result: ok\. ([0-9]+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;"
    )
    capi_counts = [int(match.group(1)) for line in (stage / "logs/capi-regressions.log").read_text().splitlines()
                   if (match := result_pattern.match(line))]
    runner_counts = [int(match.group(1)) for line in (stage / "logs/runner-corpus.log").read_text().splitlines()
                     if (match := result_pattern.match(line))]
    if capi_counts != [18, 3, 93, 0, 1, 0, 0] or runner_counts != [18, 0, 0]:
        raise ValueError("raw test logs do not prove the exact result counters")
    capi_raw = json.loads((stage / "capi-audit.json").read_text())
    realtime_raw = json.loads((stage / "realtime-audit.json").read_text())
    if capi_raw != expected_audits()[0]:
        raise ValueError("raw CAPI audit differs")
    output_address = realtime_raw.pop("output_address", None)
    realtime_raw["armed_trace_syscalls"] = 0
    if not isinstance(output_address, int) or output_address <= 0 or realtime_raw != expected_audits()[1]:
        raise ValueError("raw realtime audit differs")
    trace = (stage / "realtime-trace.1760548").read_text().splitlines()
    begin = [index for index, line in enumerate(trace) if "MISO_ENGINE_RT_BEGIN" in line]
    end = [index for index, line in enumerate(trace) if "MISO_ENGINE_RT_END" in line]
    if len(begin) != 1 or len(end) != 1 or end[0] != begin[0] + 1:
        raise ValueError("armed realtime trace contains a syscall")


def main() -> int:
    if len(sys.argv) != 3 or sys.argv[2] not in {"committed", "preserved"}:
        return 2
    root = pathlib.Path(sys.argv[1]).resolve()
    fixture = root / "fixtures/capi-qualification/v1"
    try:
        check_artifacts(fixture)
        expected_symbols = check_symbols(fixture)
        check_audits(fixture)
        check_qualification(fixture)
        check_consumers(fixture)
        check_raw_table(fixture)
        check_gates(fixture)
        if sys.argv[2] == "preserved":
            check_stage(root, expected_symbols)
    except (OSError, ValueError, KeyError, IndexError, json.JSONDecodeError) as error:
        print(f"qualification semantic evidence failure: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
