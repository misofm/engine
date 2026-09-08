#!/usr/bin/env python3
"""Strict JSONL validator and mutation proof for issue #650."""

from __future__ import annotations

import argparse
import json
import sys
from typing import Any


KEYS = (
    "variant",
    "corpus",
    "round",
    "graph_sha256",
    "diagnostic_sha256",
    "allocations",
    "deallocations",
    "reallocations",
    "requested_bytes",
)
CORPORA = ("zero64", "crossed-small", "banks64")
VARIANTS = ("candidate", "counterfactual")
MAX_U64 = 2**64 - 1


class RecordError(ValueError):
    pass


def reject_duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise RecordError(f"duplicate key: {key}")
        result[key] = value
    return result


def parse(text: str) -> list[dict[str, Any]]:
    if not text.endswith("\n"):
        raise RecordError("JSONL must end with LF")
    lines = text.splitlines()
    if len(lines) != 12:
        raise RecordError("expected exactly 12 records")
    records: list[dict[str, Any]] = []
    for line_number, line in enumerate(lines, 1):
        try:
            value = json.loads(line, object_pairs_hook=reject_duplicates)
        except (json.JSONDecodeError, RecordError) as error:
            raise RecordError(f"line {line_number}: invalid JSON: {error}") from error
        if not isinstance(value, dict) or tuple(value) != KEYS:
            raise RecordError(f"line {line_number}: wrong schema or key order")
        records.append(value)
    return records


def validate(text: str) -> None:
    records = parse(text)
    seen: set[tuple[str, str, int]] = set()
    by_variant: dict[str, list[dict[str, Any]]] = {variant: [] for variant in VARIANTS}
    for record in records:
        variant = record["variant"]
        corpus = record["corpus"]
        round_number = record["round"]
        if variant not in VARIANTS or corpus not in CORPORA:
            raise RecordError("unknown variant or corpus")
        if type(round_number) is not int or round_number not in (1, 2):
            raise RecordError("round must be exactly 1 or 2")
        for key in KEYS[3:]:
            value = record[key]
            if key.endswith("sha256"):
                if (
                    not isinstance(value, str)
                    or len(value) != 64
                    or any(char not in "0123456789abcdef" for char in value)
                ):
                    raise RecordError(f"invalid {key}")
            elif type(value) is not int or not 0 <= value <= MAX_U64:
                raise RecordError(f"invalid counter {key}")
        key = (variant, corpus, round_number)
        if key in seen:
            raise RecordError("duplicate corpus/round")
        seen.add(key)
        by_variant[variant].append(record)
    if len(seen) != 12 or any(len(rows) != 6 for rows in by_variant.values()):
        raise RecordError("missing corpus/round record")
    for variant, rows in by_variant.items():
        for corpus in CORPORA:
            pair = sorted(
                (row for row in rows if row["corpus"] == corpus),
                key=lambda row: row["round"],
            )
            if [row["round"] for row in pair] != [1, 2]:
                raise RecordError(f"{variant}/{corpus}: wrong rounds")
            if (
                pair[0]["graph_sha256"] != pair[1]["graph_sha256"]
                or pair[0]["diagnostic_sha256"] != pair[1]["diagnostic_sha256"]
            ):
                raise RecordError(f"{variant}/{corpus}: unstable identity")
            for field in KEYS[5:]:
                if pair[0][field] != pair[1][field]:
                    raise RecordError(f"{variant}/{corpus}: unstable {field}")
    for corpus in CORPORA:
        candidate = next(row for row in by_variant["candidate"] if row["corpus"] == corpus)
        counterfactual = next(
            row for row in by_variant["counterfactual"] if row["corpus"] == corpus
        )
        if (
            candidate["graph_sha256"] != counterfactual["graph_sha256"]
            or candidate["diagnostic_sha256"] != counterfactual["diagnostic_sha256"]
        ):
            raise RecordError(f"{corpus}: semantic/diagnostic identity drift")
        if corpus == "zero64" and any(
            candidate[field] != counterfactual[field] for field in KEYS[5:]
        ):
            raise RecordError("zero64 control drift")
    small_candidate = next(
        row for row in by_variant["candidate"] if row["corpus"] == "crossed-small"
    )
    small_counterfactual = next(
        row for row in by_variant["counterfactual"] if row["corpus"] == "crossed-small"
    )
    banks_candidate = next(
        row for row in by_variant["candidate"] if row["corpus"] == "banks64"
    )
    banks_counterfactual = next(
        row for row in by_variant["counterfactual"] if row["corpus"] == "banks64"
    )
    for field in ("allocations", "requested_bytes"):
        if not small_candidate[field] < small_counterfactual[field]:
            raise RecordError(f"crossed-small {field} reduction missing")
        if not banks_candidate[field] < banks_counterfactual[field]:
            raise RecordError(f"banks64 {field} reduction missing")
    if (
        banks_counterfactual["allocations"] - banks_candidate["allocations"]
        <= small_counterfactual["allocations"] - small_candidate["allocations"]
    ):
        raise RecordError("banks64 saving is not larger")


def baseline() -> str:
    rows: list[dict[str, Any]] = []
    for variant in VARIANTS:
        for corpus in CORPORA:
            for round_number in (1, 2):
                counterfactual = variant == "counterfactual"
                banks = corpus == "banks64"
                rows.append(
                    {
                        "variant": variant,
                        "corpus": corpus,
                        "round": round_number,
                        "graph_sha256": "a" * 64,
                        "diagnostic_sha256": "b" * 64,
                        "allocations": (
                            5 if banks and not counterfactual else
                            30 if banks else
                            10 if corpus == "zero64" or not counterfactual else 20
                        ),
                        "deallocations": 4,
                        "reallocations": 1,
                        "requested_bytes": (
                            50 if banks and not counterfactual else
                            300 if banks else
                            100 if corpus == "zero64" or not counterfactual else 200
                        ),
                    }
                )
    return "".join(
        json.dumps(row, separators=(",", ":")) + "\n" for row in rows
    )


def self_test() -> None:
    good = baseline()
    validate(good)
    mutations = [
        ("wrong-schema", good.replace('"variant":"candidate"', '"wrong":"candidate"', 1)),
        ("wrong-key-order", good.replace('{"variant":', '{"corpus":', 1)),
        ("missing-corpus", "\n".join(good.splitlines()[:-2]) + "\n"),
        ("duplicate-corpus-round", good + good.splitlines()[0] + "\n"),
        ("changed-identity", good.replace('"' + "a" * 64 + '"', '"' + "c" * 64 + '"', 1)),
        ("unstable-counters", good.replace('"allocations":10', '"allocations":11', 1)),
        ("zero64-drift", good.replace('"requested_bytes":100', '"requested_bytes":101', 1)),
        ("no-reduction", good.replace('"allocations":10', '"allocations":20', 1)),
        ("trailing-data", good + "{}\n"),
        ("duplicate-json-key", good.replace('"round":1', '"round":1,"round":1', 1)),
        ("wrong-type", good.replace('"allocations":10', '"allocations":"10"', 1)),
        ("negative-range", good.replace('"requested_bytes":100', '"requested_bytes":-1', 1)),
        (
            "boolean-type",
            good.replace('"allocations":10', '"allocations":true', 1),
        ),
        (
            "upper-range",
            good.replace('"requested_bytes":100', '"requested_bytes":18446744073709551616', 1),
        ),
    ]
    for name, mutated in mutations:
        try:
            validate(mutated)
        except RecordError:
            continue
        raise AssertionError(f"mutation escaped validator: {name}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("path", nargs="?")
    args = parser.parse_args()
    if args.self_test:
        if args.path is not None:
            parser.error("--self-test takes no path")
        self_test()
        print("prepared-effect allocation record self-test passed (14 mutations)")
        return 0
    if args.path is None:
        parser.error("record JSONL path is required")
    try:
        with open(args.path, encoding="utf-8") as stream:
            validate(stream.read())
    except (OSError, RecordError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        return 1
    print("prepared-effect allocation records valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
