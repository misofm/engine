#!/usr/bin/env python3
"""Validate the frozen lane source rules and emit an explicit lossless protocol."""

from __future__ import annotations

import base64
import re
import sys
import tomllib

EXPECTED_IDS = ("fusion", "relaxed", "architecture", "detection")
REQUIRED_FIELDS = {
    "id",
    "scan_description",
    "forbidden_regex",
    "glob",
    "roots",
    "failure_diagnostic",
}
OPTIONAL_FIELDS = {"exclude_description", "exclude_regex"}
TOP_LEVEL_FIELDS = {"version", "rules"}


def fail(message: str) -> int:
    print(f"lane rule policy invalid: {message}", file=sys.stderr)
    return 1


def encoded(value: str) -> str:
    return base64.b64encode(value.encode("utf-8")).decode("ascii")


def main() -> int:
    if len(sys.argv) != 2:
        return fail("loader requires exactly one TOML path")
    try:
        with open(sys.argv[1], "rb") as policy_file:
            policy = tomllib.load(policy_file)
    except (OSError, tomllib.TOMLDecodeError) as error:
        return fail(str(error))

    if set(policy) != TOP_LEVEL_FIELDS:
        return fail("unknown or missing top-level fields")
    if type(policy["version"]) is not int or policy["version"] != 1:
        return fail("unsupported policy version")
    rules = policy["rules"]
    if not isinstance(rules, list) or len(rules) != len(EXPECTED_IDS):
        return fail("policy must contain exactly four rules")

    output: list[str] = []
    seen: set[str] = set()
    for index, rule in enumerate(rules):
        if not isinstance(rule, dict):
            return fail(f"rule {index + 1} is not a table")
        if set(rule) - REQUIRED_FIELDS - OPTIONAL_FIELDS:
            return fail(f"rule {index + 1} contains an unknown field")
        if not REQUIRED_FIELDS <= set(rule):
            return fail(f"rule {index + 1} is missing a required field")
        rule_id = rule["id"]
        if not isinstance(rule_id, str) or rule_id != EXPECTED_IDS[index] or rule_id in seen:
            return fail(f"rule {index + 1} has the wrong or duplicate id")
        seen.add(rule_id)

        for field in REQUIRED_FIELDS - {"id", "roots"}:
            if not isinstance(rule[field], str) or not rule[field]:
                return fail(f"rule {index + 1} has an invalid {field}")
        try:
            re.compile(rule["forbidden_regex"])
        except re.error:
            return fail(f"rule {index + 1} has an invalid forbidden_regex")
        roots = rule["roots"]
        if (
            not isinstance(roots, list)
            or len(roots) != 4
            or any(not isinstance(root, str) or not root for root in roots)
        ):
            return fail(f"rule {index + 1} has invalid roots")
        has_exclude_description = "exclude_description" in rule
        has_exclude_regex = "exclude_regex" in rule
        if has_exclude_description != has_exclude_regex:
            return fail(f"rule {index + 1} has an incomplete exclusion")
        if has_exclude_description and (
            not isinstance(rule["exclude_description"], str)
            or not isinstance(rule["exclude_regex"], str)
            or not rule["exclude_description"]
            or not rule["exclude_regex"]
        ):
            return fail(f"rule {index + 1} has an invalid exclusion")
        if has_exclude_regex:
            try:
                re.compile(rule["exclude_regex"])
            except re.error:
                return fail(f"rule {index + 1} has an invalid exclude_regex")

        fields = [
            rule_id,
            rule["scan_description"],
            rule["forbidden_regex"],
            rule["glob"],
            *roots,
            rule.get("exclude_description", ""),
            rule.get("exclude_regex", ""),
            rule["failure_diagnostic"],
        ]
        output.append("RULE\t" + "\t".join(encoded(field) for field in fields))

    if len(seen) != len(EXPECTED_IDS):
        return fail("rule population is invalid")
    print("\n".join(output))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
