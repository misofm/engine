#!/usr/bin/env python3
"""Render CI job telemetry from a GitHub Actions jobs listing (issue #359 WP-1, design §11).

Reads the `repos/.../actions/runs/<id>/jobs` JSON shape (`--jobs <file>`), computes per job:

  queue     = started_at - created_at
  setup     = job start to the end of the last *leading* step whose name starts with
              "Set up", "Run actions/checkout", "Install", "Restore", or "Download"
  execution = completed_at - (end of that leading run, or started_at if none/no steps)
  total     = completed_at - created_at

and prints a Markdown table plus a one-line "longest job" summary. `--out <file>` additionally
writes the same per-job data as JSON, plus `route`/`attempt`/`cache_hits` from `--route`,
`--attempt`, `--cache-hits <string>`.

A job whose timestamps are null (skipped) or whose `steps` list is empty or absent renders as all
zero durations rather than raising -- the verdict job that will call this in stage 1 must never
itself fail qualification because a route left some job legitimately skipped.
"""
from __future__ import annotations

import argparse
import datetime
import json
import pathlib
import sys

LEADING_SETUP_PREFIXES = ("Set up", "Run actions/checkout", "Install", "Restore", "Download")


def parse_timestamp(value: object) -> datetime.datetime | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return datetime.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None


def seconds_between(start: datetime.datetime | None, end: datetime.datetime | None) -> float:
    if start is None or end is None:
        return 0.0
    return max(0.0, (end - start).total_seconds())


def job_metrics(job: dict) -> dict:
    created = parse_timestamp(job.get("created_at"))
    started = parse_timestamp(job.get("started_at"))
    completed = parse_timestamp(job.get("completed_at"))

    queue = seconds_between(created, started)

    steps = job.get("steps") or []
    setup_end = started
    for step in steps:
        name = step.get("name", "") if isinstance(step, dict) else ""
        if not any(name.startswith(prefix) for prefix in LEADING_SETUP_PREFIXES):
            break
        step_completed = parse_timestamp(step.get("completed_at")) if isinstance(step, dict) else None
        if step_completed is not None:
            setup_end = step_completed
    setup = seconds_between(started, setup_end) if started is not None else 0.0

    execution = seconds_between(setup_end, completed)
    total = seconds_between(created, completed)

    return {
        "name": job.get("name", "<unnamed job>"),
        "conclusion": job.get("conclusion") or job.get("status") or "unknown",
        "queue_seconds": round(queue, 1),
        "setup_seconds": round(setup, 1),
        "execution_seconds": round(execution, 1),
        "total_seconds": round(total, 1),
    }


def render_markdown(rows: list[dict]) -> str:
    lines = [
        "| Job | Conclusion | Queue (s) | Setup (s) | Execution (s) | Total (s) |",
        "|---|---|---|---|---|---|",
    ]
    for row in rows:
        lines.append(
            f"| {row['name']} | {row['conclusion']} | {row['queue_seconds']:g} | "
            f"{row['setup_seconds']:g} | {row['execution_seconds']:g} | {row['total_seconds']:g} |"
        )
    if rows:
        longest = max(rows, key=lambda row: row["total_seconds"])
        lines.append("")
        lines.append(
            f"Longest job: {longest['name']} ({longest['total_seconds']:g}s total)."
        )
    else:
        lines.append("")
        lines.append("Longest job: none (no jobs in this run).")
    return "\n".join(lines) + "\n"


def build_report(document: dict) -> list[dict]:
    jobs = document.get("jobs")
    if not isinstance(jobs, list):
        raise ValueError("jobs document is missing a 'jobs' list")
    return [job_metrics(job) for job in jobs if isinstance(job, dict)]


def self_test() -> None:
    document = {
        "jobs": [
            {
                "name": "route",
                "status": "completed",
                "conclusion": "success",
                "created_at": "2026-09-04T00:00:00Z",
                "started_at": "2026-09-04T00:00:02Z",
                "completed_at": "2026-09-04T00:00:17Z",
                "steps": [
                    {"name": "Set up job", "status": "completed", "conclusion": "success",
                     "started_at": "2026-09-04T00:00:02Z", "completed_at": "2026-09-04T00:00:03Z"},
                    {"name": "Run actions/checkout@v4", "status": "completed", "conclusion": "success",
                     "started_at": "2026-09-04T00:00:03Z", "completed_at": "2026-09-04T00:00:06Z"},
                    {"name": "Install pinned Rust toolchain", "status": "completed",
                     "conclusion": "success", "started_at": "2026-09-04T00:00:06Z",
                     "completed_at": "2026-09-04T00:00:09Z"},
                    {"name": "Classify", "status": "completed", "conclusion": "success",
                     "started_at": "2026-09-04T00:00:09Z", "completed_at": "2026-09-04T00:00:17Z"},
                ],
            },
            {
                # A skipped job: null started_at/completed_at, no steps recorded.
                "name": "sdk",
                "status": "completed",
                "conclusion": "skipped",
                "created_at": "2026-09-04T00:00:00Z",
                "started_at": None,
                "completed_at": None,
                "steps": [],
            },
            {
                # A cancelled job with normal timestamps and one recorded step.
                "name": "browser firefox",
                "status": "completed",
                "conclusion": "cancelled",
                "created_at": "2026-09-04T00:00:00Z",
                "started_at": "2026-09-04T00:00:05Z",
                "completed_at": "2026-09-04T00:03:05Z",
                "steps": [
                    {"name": "Set up job", "status": "completed", "conclusion": "success",
                     "started_at": "2026-09-04T00:00:05Z", "completed_at": "2026-09-04T00:00:08Z"},
                    {"name": "Run tests", "status": "completed", "conclusion": "cancelled",
                     "started_at": "2026-09-04T00:00:08Z", "completed_at": None},
                ],
            },
            {
                # A job with no steps key at all.
                "name": "audit-native",
                "status": "completed",
                "conclusion": "success",
                "created_at": "2026-09-04T00:00:00Z",
                "started_at": "2026-09-04T00:00:03Z",
                "completed_at": "2026-09-04T00:05:03Z",
            },
        ]
    }

    rows = build_report(document)
    assert len(rows) == 4

    route_row = rows[0]
    assert route_row["queue_seconds"] == 2.0
    assert route_row["setup_seconds"] == 7.0  # 00:00:02 -> 00:00:09 (Set up + checkout + Install)
    assert route_row["execution_seconds"] == 8.0  # 00:00:09 -> 00:00:17 (Classify)
    assert route_row["total_seconds"] == 17.0

    sdk_row = rows[1]
    assert sdk_row["conclusion"] == "skipped"
    assert sdk_row["queue_seconds"] == 0.0
    assert sdk_row["setup_seconds"] == 0.0
    assert sdk_row["execution_seconds"] == 0.0
    assert sdk_row["total_seconds"] == 0.0

    browser_row = rows[2]
    assert browser_row["conclusion"] == "cancelled"
    assert browser_row["queue_seconds"] == 5.0
    assert browser_row["setup_seconds"] == 3.0
    assert browser_row["execution_seconds"] == 177.0  # 00:00:08 -> 00:03:05
    assert browser_row["total_seconds"] == 185.0

    audit_row = rows[3]
    assert audit_row["setup_seconds"] == 0.0  # no steps: nothing is "leading setup"
    assert audit_row["execution_seconds"] == 300.0  # the whole job counts as execution
    assert audit_row["total_seconds"] == 303.0

    markdown = render_markdown(rows)
    assert markdown.startswith("| Job | Conclusion |")
    assert "browser firefox" in markdown
    assert "Longest job: audit-native (303s total)." in markdown

    empty_markdown = render_markdown([])
    assert "Longest job: none" in empty_markdown

    payload = {
        "jobs": rows,
        "route": "full",
        "attempt": 2,
        "cache_hits": '{"lint":"true","test-release":"false"}',
    }
    json.dumps(payload)  # must be JSON-serializable

    print("ci-telemetry self-test: ok")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--jobs", type=pathlib.Path)
    parser.add_argument("--out", type=pathlib.Path)
    parser.add_argument("--route", default="")
    parser.add_argument("--attempt", default="")
    parser.add_argument("--cache-hits", default="")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        self_test()
        return 0

    if args.jobs is None:
        print("ci-telemetry: --jobs is required (or --self-test)", file=sys.stderr)
        return 1

    try:
        document = json.loads(args.jobs.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        print(f"ci-telemetry: cannot read --jobs: {error}", file=sys.stderr)
        return 1

    try:
        rows = build_report(document)
    except ValueError as error:
        print(f"ci-telemetry: {error}", file=sys.stderr)
        return 1

    print(render_markdown(rows), end="")

    if args.out is not None:
        attempt: int | str = args.attempt
        try:
            attempt = int(args.attempt)
        except (TypeError, ValueError):
            attempt = args.attempt
        payload = {
            "jobs": rows,
            "route": args.route,
            "attempt": attempt,
            "cache_hits": args.cache_hits,
        }
        args.out.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
