#!/usr/bin/env bash
# Bounded mutation pass used by the 2026-09-28 test-value audit and by the issue gates.
#
# usage: TVA_DIR=/some/scratch run-mutants.sh <package> <jobs> <jobserver-tasks> [cargo-mutants args...]
#   e.g. TVA_DIR=$HOME/tva run-mutants.sh graph-compiler 5 10
#        TVA_DIR=$HOME/tva run-mutants.sh host-core 5 10 --features control-provider,test-support \
#             --shard 0/4 --sharding round-robin
# Output goes to $TVA_DIR/mut/${OUTNAME:-out-<package>}/mutants.out. Requires `cargo-mutants`
# (27.x) on PATH. Tests run with --no-fail-fast, so every mutant's log records every failing test;
# parse_mutants.py turns that into a per-test catch matrix.
set -uo pipefail
: "${TVA_DIR:?set TVA_DIR to a scratch directory outside the repository}"
W=$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)
pkg=$1; jobs=$2; tasks=$3; shift 3
export CARGO_INCREMENTAL=0
# opt-level 1 without debuginfo: same semantics as the CI debug profile (debug assertions and
# overflow checks stay on), much faster DSP test bodies.
export CARGO_PROFILE_DEV_OPT_LEVEL=1
export CARGO_PROFILE_DEV_DEBUG=0
export RUST_TEST_THREADS=4
export TMPDIR="$TVA_DIR/mut/tmp-${OUTNAME:-$pkg}"
mkdir -p "$TMPDIR"
cd "$W"
cargo mutants -p "$pkg" -j "$jobs" --jobserver-tasks "$tasks" \
  --cargo-test-arg=--no-fail-fast \
  --timeout-multiplier 4 --minimum-test-timeout 180 --build-timeout 1200 \
  --no-shuffle -o "$TVA_DIR/mut/${OUTNAME:-out-$pkg}" "$@"
echo "exit=$?"
