#!/usr/bin/env python3
"""Semantically validate Issue-114 evidence independently of its checksum manifest."""

from __future__ import annotations

import hashlib
import json
import pathlib
import re
import sys


EXPECTED_ARTIFACTS = {
    "header": (11743, "8447175b599c265f00ef933fbda95f37886e173fe0842e4bf06ac5a5f9d091f1"),
    "linux-x86_64-static-library": (105179470, "4ffb9672dde77b6eb358d27b6b4e8937b18dcd74f5034414bf5ad5fabf0ceef2"),
    "linux-x86_64-shared-library": (29346184, "b43a4bf21c4c537cd22df605221d713a91fbd420f8194a442a2bf8733c299ec0"),
    "linux-c11-static-consumer": (30822280, "cb4c145c8931b10dc3f5012eb17ae13d53e0e834478e2c0c8c55567de78587e8"),
    "linux-c11-shared-consumer": (21312, "12055e4498f7eacea415bbb1f936c58c65abb232fae6a16f0ad7c0a8a6a9a87c"),
    "linux-cpp17-static-consumer": (30822312, "082e5fb85fae3e67a9715d9f9df4c498c6edf0a270ac194ef45bfcc8789a02c4"),
    "linux-cpp17-shared-consumer": (21344, "a3e8475e64220485c8cd21476d1f5057f2ff5c34a3b422c7c4e8ad134e8158e2"),
}
EXPECTED_QUALIFICATION = {
    "product_build_invocations": "1",
    "consumer_fixture_corrections": "1",
    "initial_c11_static_exit": "13",
    "linux_consumer_passes": "4",
    "capi_protocol_test_invocations": "1",
    "capi_unit_tests": "29",
    "capi_external_tests": "3",
    "protocol_unit_tests": "123",
    "protocol_conformance_tests": "3",
    "protocol_controller_response_tests": "1",
    "protocol_mutation_tests": "1",
    "protocol_session_edit_tests": "3",
    "capi_protocol_log_sha256": "86ab12f9a04bfbee8189b659a1c468247cfe50b548d46c03483d37f0d2326463",
    "runner_corpus_invocations": "1",
    "runner_tests": "19",
    "runner_corpus_rows": "5",
    "runner_output_bytes_per_row": "8192",
    "runner_log_sha256": "258b9fcdf3bfccb1d0d965207806c912e0e866079cec9290c95f0c273697b712",
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
    "ARTIFACTS.generated.sha256": (813, "488965a670e84eb30f8965806ab4508ec8fba0eb892a734a29e3915df52e5b74"),
    "static-symbols.txt": (425, "fc377c133fd8e96b9eb7e8109125d247dc500b520b5460af9ac69fe2141b2f56"),
    "shared-symbols.txt": (425, "fc377c133fd8e96b9eb7e8109125d247dc500b520b5460af9ac69fe2141b2f56"),
    "static-nm.txt": (669359, "bd27a50fa9aa40c2e0c6362e3fc72c515b9a274d30eaaf5485ac9b235a89dd92"),
    "shared-nm.txt": (2985, "6ea2d7956d07ecd7858a91d678ef8d08282a09bd8df7485e3836e7a89b5e4246"),
    "shared-readelf.txt": (6128, "2105b9dc878c5e0ae1f1d36a82695ea701412e5e5a35b9ca3b2aea1f0ae00019"),
    "shared-objdump.txt": (3659, "cb220e4377d12252e499d24a6004efcd51e7007734b9ef85258202729bda7cee"),
    "logs/capi-build.log": (2701, "68fd096dc28d0838fbb80e544b57eeedcbb88b39e33c725244d1c87a24ed1693"),
    "logs/capi-regressions.log": (19204, "86ab12f9a04bfbee8189b659a1c468247cfe50b548d46c03483d37f0d2326463"),
    "logs/runner-corpus.log": (2271, "258b9fcdf3bfccb1d0d965207806c912e0e866079cec9290c95f0c273697b712"),
    "logs/capi-audit.stderr": (2099, "ec9fd85db43c02ac7b236294341ad4508d202753127c1362e22468fe0cdac5ee"),
    "logs/realtime-build.log": (74, "9d4884817284ccd145bfdfea8a98000309b205484a7b197a65b7ca435d38dc8c"),
    "logs/c11-static.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "logs/c11-shared.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "logs/cpp17-static.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "logs/cpp17-shared.log": (0, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    "capi-audit.json": (349, "6c1be4989a512252706d85ef0e00d3161fb0beddc17004bc3ee2ebba79abc1aa"),
    "realtime-audit.json": (247, "b3129833ba4a36eb12524914f67ec082d94d92c192f4161411d25a8fe233dec8"),
    "realtime-trace.74104": (11772, "317913b29fd71c084729ab500cadab36814cf6cf37d014df583d7a2302d9b058"),
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
         "pcm_digest": "ff6cdcb96cdcdad5", "render_errors": 0, "allocations": 0,
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
        "header": stage / "installed/include/miso_engine_v1.h",
        "linux-x86_64-static-library": stage / "installed/lib/libcapi.a",
        "linux-x86_64-shared-library": stage / "installed/lib/libcapi.so",
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
            if len(fields) < 2 or not fields[-1].startswith("miso_engine_v1_"):
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
    if capi_counts != [29, 3, 123, 0, 3, 1, 1, 3, 0, 0] or runner_counts != [19, 0, 0]:
        raise ValueError("raw test logs do not prove the exact result counters")
    capi_raw = json.loads((stage / "capi-audit.json").read_text())
    realtime_raw = json.loads((stage / "realtime-audit.json").read_text())
    if capi_raw != expected_audits()[0]:
        raise ValueError("raw CAPI audit differs")
    output_address = realtime_raw.pop("output_address", None)
    realtime_raw["armed_trace_syscalls"] = 0
    if not isinstance(output_address, int) or output_address <= 0 or realtime_raw != expected_audits()[1]:
        raise ValueError("raw realtime audit differs")
    trace = (stage / "realtime-trace.74104").read_text().splitlines()
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
