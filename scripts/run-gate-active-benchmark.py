#!/usr/bin/env python3
"""The bounded issue #746 preflight and exactly-once runner.

This is intentionally an issue-specific orchestrator.  It validates the frozen subject shape and
the machine/counter prerequisites before creating the requested output directory.  A run then
launches exactly one warmup and two measured subject processes; every process and perf CSV is
retained beside the final JSONL evidence.  No shell command is constructed and no privilege is
selected implicitly.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Iterable


ISSUE = 746
SUBJECTS = {
    "gate-active": {
        "issue": 746,
        "record": "gate_active",
        "effect_id": None,
    },
    "multiband-active": {
        "issue": 748,
        "record": "multiband_active",
        "effect_id": "miso.multiband-compressor",
    },
}
SAMPLE_RATE = 48_000
FRAMES = 128
CHANNELS = 2
SCALAR_WIDTH = 1
BANK_WIDTH = 8
PREFLIGHT_BLOCKS = 128
WARMUP_BLOCKS = 8_192
MEASURED_BLOCKS = 32_768
DRIFT_CEILING = 0.03
HIGH_RATIO_LIMIT = 0.9
LOW_RATIO_LIMIT = 0.01
MULTIBAND_BANDS = ("low", "high")
MULTIBAND_HIGH_GAIN_LIMIT_DB = -3.0
MULTIBAND_QUIET_GAIN_FLOOR_DB = -0.1
MULTIBAND_QUIET_GAIN_CEILING_DB = 0.0
METADATA_NAMES = (
    "MISO_ENGINE_BENCH_CPU_MODEL",
    "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE",
    "MISO_ENGINE_BENCH_RUST_VERSION",
    "MISO_ENGINE_BENCH_LLVM_VERSION",
    "MISO_ENGINE_BENCH_TARGET_TRIPLE",
    "MISO_ENGINE_BENCH_TARGET_FEATURES",
    "MISO_ENGINE_BENCH_PROFILE",
    "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
    "MISO_ENGINE_BENCH_MEASUREMENT_CONTROL",
    "MISO_ENGINE_BENCH_CPU_AFFINITY",
    "MISO_ENGINE_BENCH_CANDIDATE_COMMIT",
)


class RunnerError(RuntimeError):
    """A refusal or a preserved run failure."""


def subject_spec(name: str) -> dict[str, Any]:
    try:
        return SUBJECTS[name]
    except KeyError as error:
        raise RunnerError(f"unknown subject: {name}") from error


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def underlying_perf(path: Path) -> Path:
    """Resolve an explicit wrapper's absolute underlying perf path when it declares one."""

    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return path
    candidates = re.findall(
        r"(?<![A-Za-z0-9_.-])(/[A-Za-z0-9_./-]*perf)(?![A-Za-z0-9_.-])", text
    )
    for candidate in candidates:
        resolved = Path(candidate)
        if resolved != path and resolved.is_file() and not resolved.is_symlink():
            return resolved
    return path


def regular_executable(raw: str, label: str) -> Path:
    path = Path(raw)
    if not path.is_absolute():
        raise RunnerError(f"{label} must be an absolute path")
    if path.is_symlink() or not path.is_file() or not os.access(path, os.X_OK):
        raise RunnerError(f"{label} is not an executable regular file: {path}")
    return path


def output_path(raw: str) -> Path:
    path = Path(raw)
    if not path.is_absolute():
        raise RunnerError("output must be an absolute path")
    if path.exists() or path.is_symlink():
        raise RunnerError(f"refusing existing output path: {path}")
    parent = path.parent
    if not parent.is_dir() or parent.is_symlink():
        raise RunnerError(f"output parent is not an existing regular directory: {parent}")
    return path


def parse_cpu(raw: str) -> int:
    try:
        value = int(raw, 10)
    except ValueError as error:
        raise RunnerError("cpu must be a nonnegative integer") from error
    if value < 0:
        raise RunnerError("cpu must be a nonnegative integer")
    return value


def command_text(argv: Iterable[str]) -> str:
    return " ".join(argv)


def run_capture(argv: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    try:
        result = subprocess.run(argv, text=True, capture_output=True, check=False)
    except OSError as error:
        raise RunnerError(f"failed to launch {argv[0]}: {error}") from error
    if check and result.returncode != 0:
        detail = (result.stderr or result.stdout).strip()
        raise RunnerError(f"command failed ({result.returncode}): {command_text(argv)}: {detail}")
    return result


def read_first(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8").strip()
    except (OSError, UnicodeError):
        return ""


def git_value(root: Path, *args: str) -> str:
    result = run_capture(["git", "-C", str(root), *args])
    value = result.stdout.strip()
    if not value:
        raise RunnerError(f"missing git provenance for {' '.join(args)}")
    return value


def cpu_model() -> str:
    try:
        for line in Path("/proc/cpuinfo").read_text(encoding="utf-8").splitlines():
            if line.lower().startswith("model name") and ":" in line:
                value = line.split(":", 1)[1].strip()
                if value:
                    return value
    except (OSError, UnicodeError):
        pass
    result = run_capture(["lscpu"], check=False)
    for line in result.stdout.splitlines():
        if line.startswith("Model name:") and ":" in line:
            value = line.split(":", 1)[1].strip()
            if value:
                return value
    raise RunnerError("CPU model provenance is unavailable")


def cpu_flags() -> set[str]:
    try:
        for line in Path("/proc/cpuinfo").read_text(encoding="utf-8").splitlines():
            if line.lower().startswith(("flags", "features")) and ":" in line:
                return set(line.split(":", 1)[1].split())
    except (OSError, UnicodeError):
        pass
    result = run_capture(["lscpu"], check=False)
    for line in result.stdout.splitlines():
        if line.startswith("Flags:") and ":" in line:
            return set(line.split(":", 1)[1].split())
    return set()


def governor(cpu: int) -> str:
    value = read_first(Path(f"/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_governor"))
    return value or "unknown"


def siblings(cpu: int) -> str:
    value = read_first(Path(f"/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list"))
    return value or "unknown"


def compiler_facts() -> tuple[str, str, str]:
    rust = run_capture(["rustc", "-V"]).stdout.strip()
    verbose = run_capture(["rustc", "-vV"]).stdout.splitlines()
    llvm = next((line.split(":", 1)[1].strip() for line in verbose if line.startswith("LLVM version:")), "")
    target = next((line.split(":", 1)[1].strip() for line in verbose if line.startswith("host:")), "")
    if not rust or not llvm or not target:
        raise RunnerError("compiler provenance is incomplete")
    return rust, llvm, target


def loadavg() -> str:
    value = read_first(Path("/proc/loadavg"))
    return value or "unknown"


def probe_parent(parent: Path, issue: int = ISSUE, subject: str = "gate-active") -> None:
    try:
        probe = Path(tempfile.mkdtemp(prefix=f".issue{issue}-{subject}-parent-probe-", dir=parent))
        try:
            marker = probe / "write-test"
            marker.write_bytes(b"probe")
            marker.unlink()
        finally:
            probe.rmdir()
    except OSError as error:
        raise RunnerError(f"output parent is not writable: {parent}: {error}") from error


def parse_perf_csv(path: Path) -> tuple[float, float]:
    """Return (cycles, task-clock milliseconds), rejecting ambiguous CSV."""

    counters: dict[str, tuple[float, str]] = {}
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except (OSError, UnicodeError) as error:
        raise RunnerError(f"cannot read perf CSV: {path}") from error
    for line in lines:
        if not line.strip() or line.startswith("#"):
            continue
        fields = line.split(",")
        if len(fields) < 3:
            raise RunnerError(f"malformed perf CSV line: {line!r}")
        event = fields[2].strip()
        if event not in ("cycles", "task-clock"):
            raise RunnerError(f"unsupported perf counter in CSV: {event!r}")
        if event in counters:
            raise RunnerError(f"duplicate perf counter in CSV: {event}")
        try:
            value = float(fields[0])
        except ValueError as error:
            raise RunnerError(f"non-numeric perf counter {event}: {fields[0]!r}") from error
        if not math.isfinite(value) or value <= 0.0:
            raise RunnerError(f"invalid perf counter {event}: {fields[0]!r}")
        unit = fields[1].strip()
        if event == "cycles" and unit not in ("", "count"):
            raise RunnerError(f"unexpected cycles unit: {unit!r}")
        if event == "task-clock" and unit not in ("msec", "ms"):
            raise RunnerError(f"task-clock is not reported in milliseconds: {unit!r}")
        counters[event] = (value, unit)
    if set(counters) != {"cycles", "task-clock"}:
        raise RunnerError("perf CSV lacks one or both required cycles/task-clock counters")
    return counters["cycles"][0], counters["task-clock"][0]


def effective_hz(counters: tuple[float, float]) -> float:
    cycles, task_clock_ms = counters
    hz = cycles * 1000.0 / task_clock_ms
    if not math.isfinite(hz) or hz <= 0.0:
        raise RunnerError("perf-derived effective core clock is not finite and positive")
    return hz


def write_bytes(path: Path, data: bytes) -> None:
    with path.open("xb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


def write_json(path: Path, value: Any) -> None:
    write_bytes(path, (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode())


def json_rows(data: bytes, label: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for number, line in enumerate(data.decode("utf-8").splitlines(), 1):
        if not line.strip():
            continue
        try:
            value = json.loads(line)
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            raise RunnerError(f"{label} line {number} is not JSON") from error
        if not isinstance(value, dict):
            raise RunnerError(f"{label} line {number} is not a JSON object")
        rows.append(value)
    return rows


def exact_int(value: Any, label: str, *, nonnegative: bool = True) -> int:
    """Accept JSON integer counts without coercing bools, floats, or numeric strings."""

    if type(value) is not int or (nonnegative and value < 0):
        qualifier = "nonnegative " if nonnegative else ""
        raise RunnerError(f"{label} must be a {qualifier}integer")
    return value


def gate_activity_valid(row: dict[str, Any], width: int) -> bool:
    activity = row.get("activity")
    if not isinstance(activity, list) or len(activity) != width * CHANNELS:
        return False
    try:
        blocks = exact_int(row["blocks"], "blocks")
    except RunnerError:
        return False
    expected_samples = blocks * FRAMES
    seen: set[tuple[int, str]] = set()
    for item in activity:
        if not isinstance(item, dict):
            return False
        try:
            lane = exact_int(item["lane"], "activity lane")
            channel = item["channel"]
            if not isinstance(channel, str):
                return False
            key = (lane, channel)
            high_count = exact_int(item["high_plateaus"], "high plateau count")
            low_count = exact_int(item["low_plateaus"], "low plateau count")
            high_ratio = float(item["high_ratio_witness"])
            low_ratio = float(item["low_ratio_witness"])
            finite = exact_int(item["finite_output_samples"], "finite output sample count")
            nonzero_input = exact_int(item["nonzero_input_samples"], "nonzero input sample count")
            nonzero_output = exact_int(item["nonzero_output_samples"], "nonzero output sample count")
        except (KeyError, TypeError, ValueError):
            return False
        if key in seen or key[0] < 0 or key[0] >= width or key[1] not in ("left", "right"):
            return False
        seen.add(key)
        if (
            high_count <= 0
            or low_count <= 0
            or not math.isfinite(high_ratio)
            or not math.isfinite(low_ratio)
            or high_ratio < 0.0
            or low_ratio < 0.0
            or high_ratio <= HIGH_RATIO_LIMIT
            or low_ratio >= LOW_RATIO_LIMIT
            or finite != expected_samples
            or nonzero_input != expected_samples
            or nonzero_output != expected_samples
        ):
            return False
    return len(seen) == width * CHANNELS


def multiband_activity_valid(row: dict[str, Any], width: int) -> bool:
    activity = row.get("activity")
    if not isinstance(activity, list) or len(activity) != width * CHANNELS * 2:
        return False
    try:
        blocks = exact_int(row["blocks"], "blocks")
    except (KeyError, RunnerError):
        return False
    expected_samples = blocks * FRAMES
    expected_witnesses = blocks // 64
    seen: set[tuple[int, str, str]] = set()
    for item in activity:
        if not isinstance(item, dict):
            return False
        try:
            lane = exact_int(item["lane"], "activity lane")
            channel = item["channel"]
            band = item["band"]
            high_witnesses = exact_int(item["high_witnesses"], "high witness count")
            quiet_witnesses = exact_int(item["quiet_witnesses"], "quiet witness count")
            high_min = float(item["high_gain_min_db"])
            high_max = float(item["high_gain_max_db"])
            quiet_min = float(item["quiet_gain_min_db"])
            quiet_max = float(item["quiet_gain_max_db"])
            finite = exact_int(item["finite_output_samples"], "finite output sample count")
            nonzero_input = exact_int(item["nonzero_input_samples"], "nonzero input sample count")
            nonzero_output = exact_int(item["nonzero_output_samples"], "nonzero output sample count")
            input_energy = float(item["input_energy"])
            output_energy = float(item["output_energy"])
        except (KeyError, TypeError, ValueError, RunnerError):
            return False
        key = (lane, channel, band)
        if key in seen or lane < 0 or lane >= width or channel not in ("left", "right") or band not in MULTIBAND_BANDS:
            return False
        seen.add(key)
        if (
            high_witnesses != expected_witnesses
            or quiet_witnesses != expected_witnesses
            or not math.isfinite(high_min)
            or not math.isfinite(high_max)
            or not math.isfinite(quiet_min)
            or not math.isfinite(quiet_max)
            or high_min > high_max
            or quiet_min > quiet_max
            or high_min > MULTIBAND_QUIET_GAIN_CEILING_DB
            or high_max > MULTIBAND_QUIET_GAIN_CEILING_DB
            or quiet_min > MULTIBAND_QUIET_GAIN_CEILING_DB
            or quiet_max > MULTIBAND_QUIET_GAIN_CEILING_DB
            or high_max >= MULTIBAND_HIGH_GAIN_LIMIT_DB
            or quiet_min <= MULTIBAND_QUIET_GAIN_FLOOR_DB
            or quiet_max > MULTIBAND_QUIET_GAIN_CEILING_DB
            or finite != expected_samples
            or nonzero_input <= 0
            or nonzero_input > expected_samples
            or nonzero_output <= 0
            or nonzero_output > expected_samples
            or not math.isfinite(input_energy)
            or input_energy <= 0.0
            or not math.isfinite(output_energy)
            or output_energy <= 0.0
        ):
            return False
    return len(seen) == width * CHANNELS * 2


def activity_valid(row: dict[str, Any], width: int, subject: str = "gate-active") -> bool:
    return (
        multiband_activity_valid(row, width)
        if subject == "multiband-active"
        else gate_activity_valid(row, width)
    )


def validate_subject_rows(
    rows: list[dict[str, Any]], phase: str, subject: str = "gate-active"
) -> None:
    spec = subject_spec(subject)
    expected_blocks = {
        "preflight": PREFLIGHT_BLOCKS,
        "warmup": WARMUP_BLOCKS,
        "1": MEASURED_BLOCKS,
        "2": MEASURED_BLOCKS,
    }[phase]
    if len(rows) != 2:
        raise RunnerError(f"{phase} must emit exactly two width rows, got {len(rows)}")
    seen: set[int] = set()
    for row in rows:
        if (
            type(row.get("schema_version")) is not int
            or row.get("schema_version") != 1
            or type(row.get("issue")) is not int
            or row.get("issue") != spec["issue"]
            or row.get("record") != spec["record"]
        ):
            raise RunnerError(f"{phase} has the wrong issue/schema record")
        if row.get("phase") != phase:
            raise RunnerError(f"{phase} row has mismatched phase")
        expected_round = None if phase in ("preflight", "warmup") else int(phase)
        if row.get("round") != expected_round or (
            expected_round is not None and type(row.get("round")) is not int
        ):
            raise RunnerError(f"{phase} row has the wrong round identity")
        missing_metadata = row.get("missing_metadata")
        if missing_metadata != []:
            raise RunnerError(f"{phase} row has missing metadata: {missing_metadata!r}")
        for name in METADATA_NAMES:
            key = name.removeprefix("MISO_ENGINE_BENCH_").lower()
            if row.get(key) in (None, ""):
                raise RunnerError(f"{phase} row has empty metadata field: {key}")
        try:
            sample_rate = exact_int(row["sample_rate_hz"], "sample_rate_hz")
            width = exact_int(row["width"], "width")
            blocks = exact_int(row["blocks"], "blocks")
            frames = exact_int(row["frames"], "frames")
            channels = exact_int(row["channels"], "channels")
            lane_samples = exact_int(row["lane_samples"], "lane_samples")
            timed_calls = exact_int(row["timed_call_count"], "timed_call_count")
        except (KeyError, TypeError, ValueError, RunnerError) as error:
            raise RunnerError(f"{phase} has incomplete normalization fields") from error
        if width not in (SCALAR_WIDTH, BANK_WIDTH) or width in seen:
            raise RunnerError(f"{phase} has duplicate or unsupported width")
        seen.add(width)
        expected_backend = "scalar" if width == SCALAR_WIDTH else "Simd8"
        if row.get("backend") != expected_backend:
            raise RunnerError(f"{phase} width {width} has the wrong backend")
        if (
            blocks != expected_blocks
            or sample_rate != SAMPLE_RATE
            or frames != FRAMES
            or channels != CHANNELS
            or lane_samples != blocks * frames * channels * width
            or timed_calls != (blocks if phase in ("1", "2") else 0)
        ):
            raise RunnerError(f"{phase} width {width} has incorrect frozen normalization")
        if spec["effect_id"] is not None:
            if row.get("effect_id") != spec["effect_id"]:
                raise RunnerError(f"{phase} width {width} has the wrong effect identity")
            digest = row.get("input_digest")
            if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest):
                raise RunnerError(f"{phase} width {width} lacks a valid input digest")
            frozen = {
                "stimulus": "square_components_v1",
                "low_component_hz": 125,
                "high_component_hz": 4_000,
                "high_amplitude": 0.25,
                "quiet_amplitude": 1.0 / 4_096.0,
                "crossover_hz": 1_000,
                "threshold_db": -30,
                "ratio": 4,
                "attack_ms": 1,
                "release_ms": 5,
                "makeup_db": 0,
                "state_layout_version": 1,
                "state_common_bytes": 0,
                "state_channel_bytes": 188,
                "scratch_admission_bytes": 1,
                "scratch_bytes": 0,
            }
            if any(row.get(key) != value for key, value in frozen.items()):
                raise RunnerError(f"{phase} width {width} has the wrong frozen multiband identity")
        if not activity_valid(row, width, subject):
            raise RunnerError(f"{phase} width {width} failed active envelope validation")
        reports = row.get("actual_report_counts")
        if not isinstance(reports, list) or len(reports) != width:
            raise RunnerError(f"{phase} width {width} lacks one report per lane")
        report_names = (
            "sanitized_main_samples",
            "sanitized_sidechain_samples",
            "invalid_spans",
            "nonfinite_left_blocks",
            "nonfinite_right_blocks",
        )
        for report in reports:
            if not isinstance(report, dict) or any(
                type(report.get(name)) is not int or report.get(name) != 0 for name in report_names
            ):
                raise RunnerError(f"{phase} width {width} has missing/nonzero report evidence")
        elapsed = row.get("process_elapsed_ns")
        if phase in ("preflight", "warmup"):
            if elapsed is not None:
                raise RunnerError(f"{phase} width {width} masquerades as measured")
        else:
            if not isinstance(elapsed, dict):
                raise RunnerError(f"{phase} width {width} lacks one elapsed observation per block")
            try:
                observations = exact_int(elapsed["observations"], "elapsed observations")
                elapsed_values = [
                    exact_int(elapsed[name], f"elapsed {name}")
                    for name in ("sum_ns", "min_ns", "p50_ns", "p95_ns", "p99_ns", "p999_ns", "max_ns")
                ]
                if observations != blocks or any(value <= 0 for value in elapsed_values):
                    raise RunnerError(f"{phase} width {width} has invalid elapsed time")
            except (KeyError, TypeError, ValueError, RunnerError) as error:
                raise RunnerError(f"{phase} width {width} has malformed elapsed time") from error
    if seen != {SCALAR_WIDTH, BANK_WIDTH}:
        raise RunnerError(f"{phase} did not emit scalar and W8 rows")


def host_context(root: Path, binary: Path, perf: Path, cpu: int) -> dict[str, Any]:
    machine = platform.machine().lower()
    if machine not in ("x86_64", "amd64"):
        raise RunnerError(f"native AVX2 W8 requires x86_64, got {machine}")
    flags = cpu_flags()
    if "avx2" not in flags or "fma" not in flags:
        raise RunnerError("native AVX2 W8 requires AVX2 and FMA host features")
    try:
        allowed = sorted(os.sched_getaffinity(0))
    except (AttributeError, OSError) as error:
        raise RunnerError("CPU affinity provenance is unavailable") from error
    if cpu not in allowed:
        raise RunnerError(f"selected CPU {cpu} is outside current affinity {allowed}")
    rust, llvm, target = compiler_facts()
    commit = git_value(root, "rev-parse", "--verify", "HEAD")
    tree = git_value(root, "rev-parse", "--verify", "HEAD^{tree}")
    status = run_capture(["git", "-C", str(root), "status", "--porcelain=v1", "--untracked-files=normal"]).stdout
    return {
        "source_commit": commit,
        "source_tree": tree,
        "source_dirty": bool(status.strip()),
        "binary": str(binary),
        "binary_sha256": sha256(binary),
        "perf_executable": str(perf),
        "perf_sha256": sha256(perf),
        "perf_wrapper_path": str(perf),
        "perf_wrapper_sha256": sha256(perf),
        "perf_underlying_path": str(underlying_perf(perf)),
        "perf_underlying_sha256": sha256(underlying_perf(perf)),
        "architecture": machine,
        "cpu": cpu,
        "allowed_cpus": allowed,
        "cpu_model": cpu_model(),
        "governor": governor(cpu),
        "smt_siblings": siblings(cpu),
        "loadavg": loadavg(),
        "rust_version": rust,
        "llvm_version": llvm,
        "target_triple": target,
        "target_features": "x86-64-v3;avx2,fma;compile-time",
    }


def metadata_env(context: dict[str, Any], *, control: str = "controlled") -> list[str]:
    note = (
        f"loadavg {context['loadavg']}; cpu {context['cpu']}; allowed_cpus {context['allowed_cpus']}; "
        f"smt_siblings {context['smt_siblings']}; governor {context['governor']}"
    )
    values = {
        "MISO_ENGINE_BENCH_CPU_MODEL": context["cpu_model"],
        "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE": context["governor"],
        "MISO_ENGINE_BENCH_RUST_VERSION": context["rust_version"],
        "MISO_ENGINE_BENCH_LLVM_VERSION": context["llvm_version"],
        "MISO_ENGINE_BENCH_TARGET_TRIPLE": context["target_triple"],
        "MISO_ENGINE_BENCH_TARGET_FEATURES": context["target_features"],
        "MISO_ENGINE_BENCH_PROFILE": "release",
        "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE": note,
        "MISO_ENGINE_BENCH_MEASUREMENT_CONTROL": control,
        "MISO_ENGINE_BENCH_CPU_AFFINITY": str(context["cpu"]),
        "MISO_ENGINE_BENCH_CANDIDATE_COMMIT": context["source_commit"],
    }
    return [f"{name}={values[name]}" for name in METADATA_NAMES]


def perf_version(perf: Path) -> str:
    result = run_capture([str(perf), "--version"])
    value = (result.stdout or result.stderr).strip()
    if not value:
        raise RunnerError("selected perf executable did not report a version")
    return value


def perf_probe(perf: Path, directory: Path, name: str, command: list[str]) -> tuple[float, bytes, bytes, bytes]:
    csv_path = directory / f"{name}.perf.csv"
    argv = [str(perf), "stat", "-x,", "-e", "cycles,task-clock", "-o", str(csv_path), "--", *command]
    result = run_capture(argv, check=False)
    if result.returncode != 0:
        raise RunnerError(f"perf probe failed ({result.returncode}): {command_text(argv)}")
    counters = parse_perf_csv(csv_path)
    return effective_hz(counters), csv_path.read_bytes(), result.stdout.encode(), result.stderr.encode()


def child_command(binary: Path, cpu: int, env_values: list[str], args: list[str]) -> list[str]:
    taskset = shutil.which("taskset")
    env = shutil.which("env")
    if taskset is None or env is None:
        raise RunnerError("taskset and env are required for explicit pinned child metadata")
    return [taskset, "-c", str(cpu), env, *env_values, str(binary), *args]


def launch_subject(
    perf: Path,
    binary: Path,
    cpu: int,
    env_values: list[str],
    subject_args: list[str],
    directory: Path,
    label: str,
) -> tuple[list[dict[str, Any]], float, tuple[float, float], list[str], bytes, bytes, bytes]:
    csv_path = directory / f"phase-{label}.perf.csv"
    stdout_path = directory / f"phase-{label}.stdout"
    stderr_path = directory / f"phase-{label}.stderr"
    argv_path = directory / f"phase-{label}.argv.json"
    exit_path = directory / f"phase-{label}.exit.json"
    child = child_command(binary, cpu, env_values, subject_args)
    argv = [str(perf), "stat", "-x,", "-e", "cycles,task-clock", "-o", str(csv_path), "--", *child]
    write_json(argv_path, {"argv": argv, "child_argv": child, "subject_args": subject_args})
    try:
        result = run_capture(argv, check=False)
    except RunnerError as error:
        write_json(exit_path, {"returncode": None, "error": str(error)})
        raise
    stdout = result.stdout.encode()
    stderr = result.stderr.encode()
    write_bytes(stdout_path, stdout)
    write_bytes(stderr_path, stderr)
    write_json(exit_path, {"returncode": result.returncode})
    if result.returncode != 0:
        raise RunnerError(f"{label} child/perf phase failed ({result.returncode})")
    counters = parse_perf_csv(csv_path)
    rows = json_rows(stdout, f"phase {label} stdout")
    hz = effective_hz(counters)
    return rows, hz, counters, argv, stdout, stderr, csv_path.read_bytes()


def enrich_rows(rows: list[dict[str, Any]], context: dict[str, Any], perf: Path, perf_text: str, hz: float, counters: tuple[float, float], argv: list[str]) -> list[dict[str, Any]]:
    output: list[dict[str, Any]] = []
    for row in rows:
        elapsed = row["process_elapsed_ns"]
        mean_ns = float(elapsed["sum_ns"]) / float(row["blocks"])
        lane_samples_per_block = float(row["lane_samples"]) / float(row["blocks"])
        cycles_per_lane_sample = mean_ns * hz / 1.0e9 / lane_samples_per_block
        enriched = dict(row)
        enriched.update({
            "core_clock_hz": hz,
            "core_clock_source": f"perf stat cycles/task-clock over {row['phase']} launch, cpu {context['cpu']}",
            "cycles": counters[0],
            "task_clock_ms": counters[1],
            "cycles_per_lane_sample": cycles_per_lane_sample,
            "mean_process_elapsed_ns": mean_ns,
            "clock_conversion": "wall render time converted with hardware-counter-derived effective core clock; not a direct per-process PMU count",
            "perf_executable": str(perf),
            "perf_version": perf_text,
            "phase_argv": argv,
            "provenance": context,
        })
        output.append(enriched)
    return output


def preflight(
    args: argparse.Namespace,
    root: Path,
    binary: Path,
    perf: Path,
    output: Path,
    cpu: int,
    subject: str = "gate-active",
) -> dict[str, Any]:
    spec = subject_spec(subject)
    context = host_context(root, binary, perf, cpu)
    perf_text = perf_version(perf)
    probe_directory = Path(
        tempfile.mkdtemp(prefix=f".issue{spec['issue']}-{subject}-preflight-", dir=output.parent)
    )
    probe_hz, probe_csv, probe_stdout, probe_stderr = perf_probe(perf, probe_directory, "no-workload", ["true"])
    _ = (probe_stdout, probe_stderr)
    env_values = metadata_env(context)
    rows, subject_hz, counters, argv, stdout, stderr, csv = launch_subject(
        perf,
        binary,
        cpu,
        env_values,
        [subject, "--preflight"],
        probe_directory,
        "subject-preflight",
    )
    validate_subject_rows(rows, "preflight", subject)
    if not math.isfinite(subject_hz) or subject_hz <= 0.0:
        raise RunnerError("subject preflight did not produce a positive effective clock")
    write_json(probe_directory / "preflight-summary.json", {
        "schema_version": 1,
        "issue": spec["issue"],
        "subject": subject,
        "record": spec["record"],
        "mode": "preflight",
        "binary": str(binary),
        "binary_sha256": context["binary_sha256"],
        "perf_executable": str(perf),
        "perf_sha256": context["perf_sha256"],
        "perf_wrapper_path": context["perf_wrapper_path"],
        "perf_wrapper_sha256": context["perf_wrapper_sha256"],
        "perf_underlying_path": context["perf_underlying_path"],
        "perf_underlying_sha256": context["perf_underlying_sha256"],
        "perf_version": perf_text,
        "no_workload_effective_hz": probe_hz,
        "no_workload_perf_argv": [str(perf), "stat", "-x,", "-e", "cycles,task-clock", "-o", str(probe_directory / "no-workload.perf.csv"), "--", "true"],
        "no_workload_perf_csv_sha256": hashlib.sha256(probe_csv).hexdigest(),
        "subject_effective_hz": subject_hz,
        "subject_counters": counters,
        "subject_argv": argv,
        "subject_stdout_sha256": hashlib.sha256(stdout).hexdigest(),
        "subject_stderr_sha256": hashlib.sha256(stderr).hexdigest(),
        "subject_perf_csv_sha256": hashlib.sha256(csv).hexdigest(),
        "timed_subject_invocations": 0,
        "source": context,
    })
    print(
        json.dumps(
            {
                "issue": spec["issue"],
                "subject": subject,
                "status": "PREFLIGHT_PASS",
                "preflight_directory": str(probe_directory),
                "no_workload_effective_hz": probe_hz,
            },
            sort_keys=True,
        )
    )
    return {
        "context": context,
        "perf_version": perf_text,
        "preflight_directory": str(probe_directory),
        "subject": subject,
    }


def run_once(
    args: argparse.Namespace,
    root: Path,
    binary: Path,
    perf: Path,
    output: Path,
    cpu: int,
    subject: str = "gate-active",
) -> int:
    spec = subject_spec(subject)
    first = preflight(args, root, binary, perf, output, cpu, subject)
    context = first["context"]
    if context["source_dirty"]:
        raise RunnerError("run requires a clean committed source candidate")
    frozen = {
        "binary_sha256": context["binary_sha256"],
        "perf_sha256": context["perf_sha256"],
        "perf_underlying_sha256": context["perf_underlying_sha256"],
        "source_commit": context["source_commit"],
        "source_tree": context["source_tree"],
    }
    current = host_context(root, binary, perf, cpu)
    if any(current[key] != value for key, value in frozen.items()):
        raise RunnerError("binary, selected perf, or source identity changed after preflight")
    try:
        output.mkdir()
    except OSError as error:
        raise RunnerError(f"failed to create output exclusively: {output}: {error}") from error
    manifest = {
        "schema_version": 1,
        "issue": spec["issue"],
        "subject": subject,
        "record": spec["record"],
        "mode": "run",
        "source": context,
        "perf_version": first["perf_version"],
        "perf_counter_command": [str(perf), "stat", "-x,", "-e", "cycles,task-clock"],
        "cpu": cpu,
        "phases": [],
    }
    write_json(output / "launch-manifest.json", manifest)
    raw = output / "raw.jsonl"
    write_bytes(raw, b"")
    status = {
        "schema_version": 1,
        "issue": spec["issue"],
        "subject": subject,
        "record": spec["record"],
        "status": "FAIL",
        "completed_phases": [],
        "source": context,
    }
    try:
        warmup_rows, warmup_hz, warmup_counters, warmup_argv, warmup_stdout, _, _ = launch_subject(
            perf,
            binary,
            cpu,
            metadata_env(context),
            [subject, "--phase", "warmup"],
            output,
            "warmup",
        )
        validate_subject_rows(warmup_rows, "warmup", subject)
        with raw.open("ab") as stream:
            stream.write(warmup_stdout)
        manifest["phases"].append({"phase": "warmup", "argv": warmup_argv, "effective_hz": warmup_hz, "counters": warmup_counters})
        status["completed_phases"].append("warmup")
        if not math.isfinite(warmup_hz) or warmup_hz <= 0.0:
            raise RunnerError("warmup effective clock is invalid")
        accepted: list[dict[str, Any]] = []
        for phase in ("1", "2"):
            rows, hz, counters, phase_argv, phase_stdout, _, _ = launch_subject(
                perf,
                binary,
                cpu,
                metadata_env(context),
                [subject, "--phase", phase],
                output,
                phase,
            )
            validate_subject_rows(rows, phase, subject)
            drift = abs(hz - warmup_hz) / warmup_hz
            if not math.isfinite(drift) or drift > DRIFT_CEILING:
                raise RunnerError(f"{phase} effective clock drift {drift:.6f} exceeds {DRIFT_CEILING:.3f}")
            with raw.open("ab") as stream:
                stream.write(phase_stdout)
            manifest["phases"].append({"phase": phase, "argv": phase_argv, "effective_hz": hz, "counters": counters, "drift": drift})
            status["completed_phases"].append(phase)
            accepted.extend(enrich_rows(rows, context, perf, first["perf_version"], hz, counters, phase_argv))
        if len(accepted) != 4 or sorted((row["phase"], row["width"]) for row in accepted) != [("1", 1), ("1", 8), ("2", 1), ("2", 8)]:
            raise RunnerError("final accepted evidence does not contain exactly four measured rows")
        write_bytes(output / "accepted.jsonl", ("\n".join(json.dumps(row, sort_keys=True, separators=(",", ":")) for row in accepted) + "\n").encode())
        status.update({"status": "PASS", "reason": "complete"})
        write_json(output / "launch-manifest.final.json", manifest)
        write_json(output / "status.json", status)
        print(str(output / "accepted.jsonl"))
        return 0
    except Exception as error:
        status["reason"] = str(error)
        write_json(output / "launch-manifest.final.json", manifest)
        write_json(output / "status.json", status)
        raise


def parser() -> argparse.ArgumentParser:
    command = argparse.ArgumentParser(description="Issue #746/#748 active subject preflight/runner")
    mode = command.add_mutually_exclusive_group(required=True)
    mode.add_argument("--preflight", action="store_true")
    mode.add_argument("--run", action="store_true")
    command.add_argument("--binary", required=True)
    command.add_argument("--output", required=True)
    command.add_argument("--cpu", required=True)
    command.add_argument("--perf-executable")
    command.add_argument("--subject", choices=tuple(SUBJECTS), default="gate-active")
    return command


def main(argv: list[str] | None = None) -> int:
    args = parser().parse_args(argv)
    subject = args.subject
    spec = subject_spec(subject)
    try:
        script_root = Path(__file__).resolve().parent.parent
        binary = regular_executable(args.binary, "binary")
        output = output_path(args.output)
        cpu = parse_cpu(args.cpu)
        probe_parent(output.parent, spec["issue"], subject)
        perf_raw = args.perf_executable or shutil.which("perf")
        if not perf_raw:
            raise RunnerError("perf executable is unavailable")
        perf = regular_executable(perf_raw, "perf executable")
        if args.run:
            return run_once(args, script_root, binary, perf, output, cpu, subject)
        preflight(args, script_root, binary, perf, output, cpu, subject)
        return 0
    except RunnerError as error:
        print(f"issue #{spec['issue']} runner ({subject}): {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
