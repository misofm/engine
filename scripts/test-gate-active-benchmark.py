#!/usr/bin/env python3
"""Hermetic controls for the issue #746 runner.

The fake subject emits schema shaped records without doing DSP and the fake perf executable
forwards argv while writing deterministic counters.  This suite therefore exercises runner
admission and failure ordering without invoking a timed benchmark workload.
"""

from __future__ import annotations

import importlib.util
import json
import os
import stat
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
RUNNER_PATH = ROOT / "scripts" / "run-gate-active-benchmark.py"
spec = importlib.util.spec_from_file_location("issue746_runner", RUNNER_PATH)
assert spec and spec.loader
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def executable(path: Path, source: str) -> Path:
    path.write_text(source, encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR)
    return path


SUBJECT = r'''#!/usr/bin/env python3
import json, os, sys
mode = sys.argv[1:]
if mode == ["gate-active", "--preflight"]:
    phase, blocks, timed = "preflight", 128, 0
elif mode == ["gate-active", "--phase", "warmup"]:
    phase, blocks, timed = "warmup", 8192, 0
elif mode == ["gate-active", "--phase", "1"]:
    phase, blocks, timed = "1", 32768, 32768
elif mode == ["gate-active", "--phase", "2"]:
    phase, blocks, timed = "2", 32768, 32768
else:
    sys.exit(41)
if os.environ.get("STUB_BAD") == "malformed":
    print("not-json")
    sys.exit(0)
for width, backend in ((1, "scalar"), (8, "Simd8")):
    activity = []
    for lane in range(width):
        for channel in ("left", "right"):
            activity.append({
                "lane": lane, "channel": channel, "high_plateaus": 1,
                "low_plateaus": 1, "high_ratio_witness": 1.0,
                "low_ratio_witness": 0.001, "finite_output_samples": blocks * 128,
                "nonzero_input_samples": blocks * 128,
                "nonzero_output_samples": blocks * 128,
            })
    reports = [{"sanitized_main_samples": 0, "sanitized_sidechain_samples": 0,
                "invalid_spans": 0, "nonfinite_left_blocks": 0,
                "nonfinite_right_blocks": 0} for _ in range(width)]
    row = {
        "schema_version": 1, "issue": 746, "record": "gate_active",
        "phase": phase, "round": None if phase in ("preflight", "warmup") else int(phase),
        "width": width, "backend": backend, "sample_rate_hz": 48000,
        "frames": 128, "channels": 2, "blocks": blocks,
        "lane_samples": blocks * 128 * 2 * width, "timed_call_count": timed,
        "process_elapsed_ns": None if timed == 0 else {
            "observations": blocks, "sum_ns": blocks * 1000,
            "min_ns": 1000, "p50_ns": 1000, "p95_ns": 1000,
            "p99_ns": 1000, "p999_ns": 1000, "max_ns": 1000,
        }, "activity": activity, "actual_report_counts": reports,
        "descriptive_only": True, "statistical_method": "stub",
    }
    for name in (
        "CPU_MODEL", "GOVERNOR_OR_POWER_MODE", "RUST_VERSION", "LLVM_VERSION",
        "TARGET_TRIPLE", "TARGET_FEATURES", "PROFILE", "BACKGROUND_LOAD_NOTE",
        "MEASUREMENT_CONTROL", "CPU_AFFINITY", "CANDIDATE_COMMIT",
    ):
        row[name.lower()] = os.environ.get("MISO_ENGINE_BENCH_" + name, "missing")
    row["missing_metadata"] = []
    bad = os.environ.get("STUB_BAD")
    if bad == "all-high" or bad == "identity":
        for item in activity:
            item["low_ratio_witness"] = 1.0
    elif bad == "all-low":
        for item in activity:
            item["high_ratio_witness"] = 0.001
    if bad == "missing-metadata":
        row.pop("cpu_model")
        row["missing_metadata"] = ["cpu_model"]
    elif bad == "missing-report":
        row.pop("actual_report_counts")
    elif bad == "nonzero-report":
        row["actual_report_counts"][0]["invalid_spans"] = 1
    elif bad == "wrong-normalization":
        row["lane_samples"] += 1
    elif bad == "wrong-block":
        row["blocks"] += 1
    elif bad == "wrong-width":
        row["width"] = 7
    elif bad == "wrong-sample-rate":
        row["sample_rate_hz"] = 96_000
    elif bad == "wrong-round":
        row["round"] = 2 if phase == "1" else 1
    elif bad == "negative-ratio":
        row["activity"][0]["low_ratio_witness"] = -1.0
    elif bad == "nan-elapsed":
        row["process_elapsed_ns"] = {
            "observations": blocks, "sum_ns": blocks * 1000,
            "min_ns": 1000, "p50_ns": 1000, "p95_ns": 1000,
            "p99_ns": float("nan"), "p999_ns": 1000, "max_ns": 1000,
        }
    elif bad == "fractional-count":
        row["lane_samples"] = row["lane_samples"] + 0.5
    elif bad == "duplicate-width" and width == 8:
        row["width"] = 1
        row["backend"] = "scalar"
    elif bad == "missing-row" and width == 8:
        continue
    print(json.dumps(row, separators=(",", ":")))
if os.environ.get("STUB_FAIL_PHASE") == phase:
    print("stub phase failure", file=sys.stderr)
    sys.exit(17)
'''


PERF = r'''#!/usr/bin/env python3
import json, os, subprocess, sys
log = os.environ.get("STUB_PERF_LOG")
if log:
    with open(log, "a", encoding="utf-8") as stream:
        stream.write(json.dumps(sys.argv[1:]) + "\n")
if sys.argv[1:] == ["--version"]:
    print("stub-perf 1")
    raise SystemExit(0)
if "--" not in sys.argv or "-o" not in sys.argv:
    raise SystemExit(19)
csv = sys.argv[sys.argv.index("-o") + 1]
command = sys.argv[sys.argv.index("--") + 1:]
result = subprocess.run(command, capture_output=True)
sys.stdout.buffer.write(result.stdout)
sys.stderr.buffer.write(result.stderr)
with open(csv, "w", encoding="utf-8") as stream:
    stream.write("100000000,,cycles,1,100.00,1.000,GHz\n")
    stream.write("100.000,msec,task-clock,1,100.00,1.000,CPUs utilized\n")
if os.environ.get("STUB_FAIL_PERF") == "1" and "--phase" in command and command[command.index("--phase") + 1] == "1":
    raise SystemExit(23)
raise SystemExit(result.returncode)
'''


def setup(directory: Path) -> tuple[Path, Path, Path]:
    log = directory / "perf.log"
    subject = executable(directory / "subject", SUBJECT)
    perf = executable(directory / "perf", PERF)
    os.environ["STUB_PERF_LOG"] = str(log)
    return subject, perf, log


def call(subject: Path, perf: Path, output: Path, cpu: int, *extra: str) -> int:
    try:
        return runner.main([*extra, "--binary", str(subject), "--output", str(output), "--cpu", str(cpu), "--perf-executable", str(perf)])
    except SystemExit as error:
        return int(error.code)


def assert_preflight_controls(directory: Path, subject: Path, perf: Path, log: Path, cpu: int) -> None:
    output = directory / "preflight-output"
    assert call(subject, perf, output, cpu, "--preflight") == 0
    assert not output.exists()
    calls = [json.loads(line) for line in log.read_text(encoding="utf-8").splitlines()]
    calls = [call for call in calls if "stat" in call]
    assert len(calls) == 2
    subject_calls = [call for call in calls if "gate-active" in call]
    assert len(subject_calls) == 1
    assert subject_calls[0][-1] == "--preflight"
    before = log.read_text(encoding="utf-8")
    existing = directory / "existing"
    existing.mkdir()
    assert call(subject, perf, existing, cpu, "--preflight") == 1
    assert log.read_text(encoding="utf-8") == before
    assert call(subject, perf, directory / "bad-output", cpu, "--preflight", "--extra") == 2
    assert log.read_text(encoding="utf-8") == before
    assert call(subject, perf, Path("/proc/issue746-output"), cpu, "--preflight") == 1
    assert log.read_text(encoding="utf-8") == before
    for bad in (
        "malformed", "all-high", "all-low", "identity", "missing-metadata", "missing-report",
        "nonzero-report", "wrong-normalization", "wrong-block", "wrong-width", "wrong-sample-rate",
        "wrong-round", "negative-ratio", "duplicate-width", "missing-row", "fractional-count",
    ):
        os.environ["STUB_BAD"] = bad
        assert call(subject, perf, directory / f"bad-{bad}", cpu, "--preflight") == 1
        assert not (directory / f"bad-{bad}").exists()
    os.environ.pop("STUB_BAD", None)

    os.environ["STUB_BAD"] = "nan-elapsed"
    assert call(subject, perf, directory / "bad-nan-elapsed", cpu, "--preflight") == 1
    assert not (directory / "bad-nan-elapsed").exists()
    measured = runner.run_capture(
        [str(subject), "gate-active", "--phase", "1"], check=False
    )
    assert measured.returncode == 0
    measured_rows = runner.json_rows(measured.stdout.encode(), "nan elapsed subject")
    try:
        runner.validate_subject_rows(measured_rows, "1")
    except runner.RunnerError:
        pass
    else:
        raise AssertionError("NaN elapsed percentile was accepted")
    os.environ.pop("STUB_BAD", None)


def assert_executable_controls(directory: Path, perf: Path) -> None:
    assert runner.regular_executable(str(perf), "perf executable") == perf
    for raw in (str(perf.relative_to(directory)), str(directory / "missing-perf")):
        try:
            runner.regular_executable(raw, "perf executable")
        except runner.RunnerError:
            pass
        else:
            raise AssertionError(f"executable mutation {raw!r} was accepted")
    symlink = directory / "perf-link"
    symlink.symlink_to(perf)
    try:
        runner.regular_executable(str(symlink), "perf executable")
    except runner.RunnerError:
        pass
    else:
        raise AssertionError("symlink perf executable was accepted")


def assert_counter_controls(directory: Path) -> None:
    valid = "100,,cycles,1,100.0,1.0,GHz\n1,msec,task-clock,1,100.0,1.0,CPUs utilized\n"
    path = directory / "counter.csv"
    path.write_text(valid, encoding="utf-8")
    assert runner.parse_perf_csv(path) == (100.0, 1.0)
    for index, text in enumerate((
        "100,,cycles,1,100.0,1.0,GHz\n100,,cycles,1,100.0,1.0,GHz\n1,msec,task-clock,1,100.0,1.0,CPUs utilized\n",
        "nan,,cycles,1,100.0,1.0,GHz\n1,msec,task-clock,1,100.0,1.0,CPUs utilized\n",
        "100,,cycles,1,100.0,1.0,GHz\n1,seconds,task-clock,1,100.0,1.0,CPUs utilized\n",
        "100,,cycles,1,100.0,1.0,GHz\n1,msec,other,1,100.0,1.0,CPUs utilized\n",
        "100,,cycles,1,100.0,1.0,GHz\n",
    )):
        path.write_text(text, encoding="utf-8")
        try:
            runner.parse_perf_csv(path)
        except runner.RunnerError:
            pass
        else:
            raise AssertionError(f"counter mutation {index} was accepted")


def assert_known_conversion() -> None:
    row = {
        "phase": "1",
        "blocks": 2,
        "lane_samples": 512,
        "process_elapsed_ns": {
            "observations": 2,
            "sum_ns": 2000,
            "min_ns": 1000,
            "p50_ns": 1000,
            "p95_ns": 1000,
            "p99_ns": 1000,
            "p999_ns": 1000,
            "max_ns": 1000,
        },
    }
    enriched = runner.enrich_rows(
        [row], {"cpu": 0}, Path("/fake/perf"), "stub-perf 1", 2_000_000_000.0, (1.0, 1.0), ["perf"]
    )[0]
    assert enriched["mean_process_elapsed_ns"] == 1000.0
    assert enriched["cycles_per_lane_sample"] == 7.8125


def assert_exactly_once_and_failure(directory: Path, subject: Path, perf: Path, log: Path, cpu: int) -> None:
    context = runner.host_context(ROOT, subject, perf, cpu)
    context["source_dirty"] = False
    original = runner.host_context
    runner.host_context = lambda *_args: dict(context)
    try:
        output = directory / "run-output"
        assert call(subject, perf, output, cpu, "--run") == 0
        accepted = [json.loads(line) for line in (output / "accepted.jsonl").read_text(encoding="utf-8").splitlines()]
        assert len(accepted) == 4
        calls = [json.loads(line) for line in log.read_text(encoding="utf-8").splitlines()]
        phases = [call[call.index("--phase") + 1] for call in calls if "--phase" in call]
        assert phases == ["warmup", "1", "2"]
        os.environ["STUB_FAIL_PHASE"] = "1"
        failed = directory / "failed-output"
        assert call(subject, perf, failed, cpu, "--run") == 1
        status = json.loads((failed / "status.json").read_text(encoding="utf-8"))
        assert status["completed_phases"] == ["warmup"]
        assert (failed / "phase-1.argv.json").is_file()
        assert (failed / "phase-1.exit.json").is_file()
        assert (failed / "phase-1.stdout").read_text(encoding="utf-8").strip()
        assert (failed / "phase-1.stderr").read_text(encoding="utf-8").strip() == "stub phase failure"
        failed_calls = [json.loads(line) for line in log.read_text(encoding="utf-8").splitlines()]
        assert not any("--phase" in call and call[call.index("--phase") + 1] == "2" for call in failed_calls[-2:])
        os.environ["STUB_FAIL_PERF"] = "1"
        perf_failed = directory / "perf-failed-output"
        assert call(subject, perf, perf_failed, cpu, "--run") == 1
        assert json.loads((perf_failed / "phase-1.exit.json").read_text(encoding="utf-8"))["returncode"] == 23
        assert (perf_failed / "phase-1.argv.json").is_file()
        assert (perf_failed / "phase-1.stdout").read_text(encoding="utf-8").strip()
        assert (perf_failed / "phase-1.stderr").read_text(encoding="utf-8").strip() == "stub phase failure"
        os.environ.pop("STUB_FAIL_PERF", None)
    finally:
        os.environ.pop("STUB_FAIL_PHASE", None)
        runner.host_context = original


def main() -> int:
    cpu = min(os.sched_getaffinity(0))
    with tempfile.TemporaryDirectory(prefix="issue746-runner-test-") as raw:
        directory = Path(raw)
        subject, perf, log = setup(directory)
        assert_counter_controls(directory)
        assert_known_conversion()
        assert_executable_controls(directory, perf)
        assert_preflight_controls(directory, subject, perf, log, cpu)
        assert_exactly_once_and_failure(directory, subject, perf, log, cpu)
    print("issue #746 runner tests: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
