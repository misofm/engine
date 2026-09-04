#!/usr/bin/env bash
# Thin wrapper, kept so any caller of this name keeps working.
#
# The issue-007 scalar builtins cross-target matrix moved to scripts/check-cross-targets.sh, which
# deduplicates it against scripts/check-parametric-eq-targets.sh's and
# scripts/check-effect-interchange-targets.sh's matrices under one cached target dir per target
# triple.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
bash "$root/scripts/check-cross-targets.sh"
printf 'issue-007 scalar builtins target matrix: PASS (delegated to check-cross-targets.sh)\n'
