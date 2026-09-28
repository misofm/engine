#!/usr/bin/env bash
# The AArch64 test legs of issue #1017. qualification.yml's `aarch64-debug` and `aarch64-release`
# jobs run this on a GitHub-hosted arm64 runner (`ubuntu-24.04-arm`), where
# `lane::Backend::current()` is `Simd4` (NEON, the width the browser's `simd128` artifact also runs)
# and `lane::fpenv` pins FPCR. The x86-64-v3 jobs cover `Simd8` and MXCSR. iOS and Android arm64
# compile and lint in scripts/check-cross-targets.sh; this script runs tests.
#
#   run-aarch64-tests.sh debug
#       The product crates' tests (scripts/lib/product-crates.sh: capi's workspace closure) plus
#       the DSP oracles `dsp-reference` and `conformance` and `target-smoke`'s width pin, in the
#       debug profile with the test-support features test-debug-a/b name on x86.
#   run-aarch64-tests.sh release
#       `lane`, `math` and `wasm-gates` (G1-G6, M1-M3, the G5 class-A corpus, G6 under FPCR.FZ) in
#       the shipping release profile, because LANE-3 is an optimizer fold a debug build does not
#       make; `console-workload`'s class-A console digests; and the realtime audits: `audit capi`
#       and the per-effect allocation and syscall audits.
#
# Known defects are expected failures, by name, in EXPECTED_FAILURES below; each row names its
# issue. A named test is skipped (`--exact --skip`) in its leg's main run and then run alone, where
# it must fail as a test. If it passes, this script fails and says to delete the row, so a fixed
# defect cannot leave a stale marker. Nothing else is skipped, and no job is.
#
# Both modes first run a no-silent-skip gate over the packages the legs run: no test returns early
# on a SIMD backend width (`if Backend::current() != Backend::Simd8 { return; }`, or
# `let Some(bank) = prepare_bank_w8(..) else { return; }`), which passes silently on the other
# width. A width-specific test is width-agnostic, or `#[ignore]`s the other width with its reason,
# which the test output reports.
#
# Off arm64 hardware, set CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu and that target's
# CARGO_TARGET_<TRIPLE>_LINKER and _RUNNER (qemu-user). Tests that re-execute their own binary
# then also need binfmt_misc.
set -euo pipefail

usage() {
    printf 'usage: run-aarch64-tests.sh debug|release\n' >&2
    exit 2
}
[[ $# -eq 1 ]] || usage
mode=$1
[[ "$mode" == debug || "$mode" == release ]] || usage

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

fail() {
    printf 'aarch64 %s leg failure: %s\n' "$mode" "$1" >&2
    exit 1
}

for tool in cargo python3 rg; do
    command -v "$tool" >/dev/null 2>&1 || fail "missing tool $tool"
done

# The leg must really be AArch64: on any other host the tests would pass at another width.
target="${CARGO_BUILD_TARGET:-}"
if [[ -z "$target" ]]; then
    [[ "$(uname -m)" == aarch64 || "$(uname -m)" == arm64 ]] ||
        fail "host is $(uname -m), not aarch64; set CARGO_BUILD_TARGET to emulate"
    target="$(rustc -vV | sed -n 's/^host: //p')"
fi
[[ "$target" == aarch64-* ]] || fail "target $target is not AArch64"
runner=()
if [[ -n "${CARGO_BUILD_TARGET:-}" ]]; then
    runner_variable="CARGO_TARGET_$(tr 'a-z-' 'A-Z_' <<<"$CARGO_BUILD_TARGET")_RUNNER"
    read -r -a runner <<<"${!runner_variable:-}"
fi
binary_dir="${CARGO_TARGET_DIR:-$root/target}${CARGO_BUILD_TARGET:+/$CARGO_BUILD_TARGET}/release"

# issue|package|target|test. `target` is `lib` or `test:<integration test name>`.
EXPECTED_FAILURES_DEBUG=(
    # LANE-3 (#1019). Not the fmaxnm fold (a debug build makes none): AArch64's own NaN rules. An
    # arithmetic NaN is +0x7FC00000 where x86 answers -0xFFC00000, and a signalling operand wins
    # NaN propagation, so these pins, which fold raw NaN words from hostile input, move on AArch64
    # and on nothing else. With every NaN folded as one word the six compressor scenarios and the
    # EQ select legs are identical on both architectures (#1017 attempt 1 evidence). #1019's gate 1
    # owns compressor NaN-payload identity on this leg.
    "1019|compressor|lib|kernel::settled_body_tests::scenario_981_heterogeneous_hostile_render_is_pinned"
    "1019|compressor|lib|kernel::settled_body_tests::scenario_983_chunk_straddling_render_is_pinned"
    "1019|compressor|lib|kernel::settled_body_tests::scenario_985_collapsed_render_is_pinned"
    "1019|compressor|lib|kernel::settled_body_tests::scenario_995_sidechain_render_is_pinned"
    "1019|compressor|lib|kernel::settled_body_tests::scenario_1006_ramping_prefix_is_pinned"
    "1019|parametric-eq|test:bank|admitted_blocks_render_the_base_bits_without_selects"
)
EXPECTED_FAILURES_RELEASE=(
    # LANE-3 (#1019): in release the D8 `select(a > b, a, b)` folds into `fmaxnm`/`fminnm` inside
    # `exp2_lane` and `log2_lane`, scalar and vector alike, which answer differently on NaN and
    # signed-zero inputs. The same tests pass in the debug leg.
    "1019|math|test:m2_lane_identity|m2_exp2_lane_identity"
    "1019|math|test:m2_lane_identity|m2_log2_lane_identity"
)

# --- no silent skip on a backend width ----------------------------------------------------------
silent_skip='(?:Backend::current\(\)|\bbackend|\bdispatch)\s*!=\s*(?:\w+::)*Backend::Simd[48]\s*\{[^{}]*?\breturn\b'
silent_skip+='|let\s+Some\([^=;]*\)\s*=\s*[^;]*?(?:_w[48]\b|Simd[48]|BankWidth::(?:Four|Eight))[^;]*?\belse\s*\{[^{}]*?\breturn\b'
skip_roots=(crates tools/console-workload tools/wasm-gates tools/wasm-gate-corpus)
if skips="$(rg -n -U --pcre2 --glob '*.rs' "$silent_skip" "${skip_roots[@]}")"; then
    printf '%s\n' "$skips" >&2
    fail 'a test returns early on a SIMD backend width; make it width-agnostic or #[ignore] the other width with its reason'
else
    status=$?
    ((status == 1)) || fail "the no-silent-skip scan could not run (rg exit $status)"
fi

# --- expected failures ---------------------------------------------------------------------------
# Runs `cargo test "$@" <target> -- --exact <name>` for one row, which must fail as a test.
expect_failure() {
    local row=$1 issue package selector name log
    shift
    IFS='|' read -r issue package selector name <<<"$row"
    local select=(--lib)
    [[ "$selector" == test:* ]] && select=(--test "${selector#test:}")
    log="$(mktemp)"
    if cargo test "$@" "${select[@]}" -- --exact "$name" >"$log" 2>&1; then
        cat "$log" >&2
        fail "expected failure $package $name (#$issue) now passes: delete its row from scripts/run-aarch64-tests.sh"
    fi
    if ! rg -qxF "test $name ... FAILED" "$log" ||
        [[ "$(rg -c '^test result: FAILED\. 0 passed; 1 failed;' "$log")" != 1 ]]; then
        cat "$log" >&2
        fail "expected failure $package $name (#$issue) did not fail as that one test"
    fi
    rm -f "$log"
    printf 'expected failure (#%s): %s %s\n' "$issue" "$package" "$name"
}

# Skip arguments for a leg's main run: `--exact`, then one `--skip` per expected failure.
skip_arguments() {
    local row name
    printf '%s\n' --exact
    for row in "$@"; do
        IFS='|' read -r _ _ _ name <<<"$row"
        printf '%s\n%s\n' --skip "$name"
    done
}

if [[ "$mode" == debug ]]; then
    source "$root/scripts/lib/product-crates.sh"
    product_list="$(product_crates "$root")" || fail "could not derive the product crates from capi"
    packages=()
    while read -r crate; do
        packages+=(-p "$crate")
    done <<<"$product_list"
    packages+=(-p dsp-reference -p conformance -p target-smoke)
    features=builtins-compiler/test-support,source/test-support,graph/test-support
    features+=,host-core/test-support,effect-compiler/test-support,protocol/test-support
    features+=,engine/realtime-audit,math/lane,parametric-eq/test-support,builtins/test-support
    mapfile -t skips < <(skip_arguments "${EXPECTED_FAILURES_DEBUG[@]}")
    cargo test --locked --all-targets "${packages[@]}" --features "$features" -- "${skips[@]}"
    for row in "${EXPECTED_FAILURES_DEBUG[@]}"; do
        expect_failure "$row" --locked "${packages[@]}" --features "$features"
    done
    printf 'aarch64 debug leg: PASS (%d product crates, %d expected failures)\n' \
        "$(wc -l <<<"$product_list")" "${#EXPECTED_FAILURES_DEBUG[@]}"
    exit 0
fi

gates=(-p lane -p math -p wasm-gates --features math/lane)
mapfile -t skips < <(skip_arguments "${EXPECTED_FAILURES_RELEASE[@]}")
cargo test --locked --release "${gates[@]}" -- "${skips[@]}"
for row in "${EXPECTED_FAILURES_RELEASE[@]}"; do
    expect_failure "$row" --locked --release "${gates[@]}"
done

cargo test --locked --release -p console-workload

# The realtime audits, as audit-native runs them on x86: allocation, lock, log, file, network and
# syscall counters over the render path, which must all read zero.
cargo build --locked --release -p audit
audit=("${runner[@]}" "$binary_dir/audit")
"${audit[@]}" capi | tee "$binary_dir/aarch64-capi-audit.json"
python3 -B - "$binary_dir/aarch64-capi-audit.json" <<'PY'
import json
import sys

lines = open(sys.argv[1], encoding="utf-8").read().splitlines()
if len(lines) != 1:
    raise SystemExit(f"audit capi: expected one record, got {len(lines)} lines")
record = json.loads(lines[0])
if record.get("kind") != "issue022_capi_render_audit":
    raise SystemExit(f"audit capi: kind {record.get('kind')!r}")
if record.get("calls") != 100_000 or record.get("stable_output_address") is not True:
    raise SystemExit("audit capi: the record is not the 100,000-call render audit")
for field in ("render_errors", "allocations", "deallocations", "locks", "feature_detection",
              "logs", "file_io", "network_io", "syscalls", "panic_unwinds", "total_violations"):
    if record.get(field) != 0:
        raise SystemExit(f"audit capi: {field} = {record.get(field)!r}, not 0")
PY
"${audit[@]}" delay --blocks 100000
"${audit[@]}" compressor --blocks 100000
"${audit[@]}" parametric-eq --blocks 100000
# The gate audit binds its bank at the native width; `bank_available` false would mean it audited
# the scalar instance alone.
"${audit[@]}" gate-expander | tee "$binary_dir/aarch64-gate-expander-audit.json"
rg -qF '"total_violations":0' "$binary_dir/aarch64-gate-expander-audit.json" ||
    fail 'audit gate-expander reported violations'
rg -qF '"bank_available":true' "$binary_dir/aarch64-gate-expander-audit.json" ||
    fail 'audit gate-expander bound no bank at the native width'

printf 'aarch64 release leg: PASS (%d expected failures)\n' "${#EXPECTED_FAILURES_RELEASE[@]}"
