#!/usr/bin/env bash
# The AArch64 test legs of issue #1017. qualification.yml's `aarch64-debug` and `aarch64-release`
# jobs run this on a GitHub-hosted arm64 runner (`ubuntu-24.04-arm`), where
# `lane::Backend::current()` is `Simd4` (NEON, the width the browser's `simd128` artifact also runs)
# and `lane::fpenv` pins FPCR. The x86-64-v3 jobs cover `Simd8` and MXCSR. iOS and Android arm64
# compile and lint in scripts/check-cross-targets.sh; this script runs tests.
#
#   run-aarch64-tests.sh debug [<shard>/3]
#       The product crates' tests (scripts/lib/product-crates.sh: capi's workspace closure) plus
#       the DSP oracles `dsp-reference` and `conformance` and `target-smoke`'s width pin, in the
#       debug profile with the test-support features test-debug-a/b name on x86. Both modes also
#       run the four existing host-web session pins against the shared browser/native corpus.
#       With a shard, the run is one of the three `aarch64-debug` matrix jobs (#2237): shards 1 and
#       2 name their packages, and shard 3 runs every other package of the leg, so the shards
#       partition the leg and a crate that joins the leg runs in shard 3 from the day it joins.
#       Each shard runs its packages' tests, doctests and expected-failure rows; the host-web pins
#       run in shard 2 and the uncaptured `resource_lifecycle` budget run in the shard that holds
#       `capi`. A shard must build every workspace crate with the features the whole leg resolves
#       for it (see "Shards" below). Without a shard, the run is the whole leg.
#   run-aarch64-tests.sh release
#       `lane` and `math` (G1-G4, G6 under FPCR.FZ, P1, M1-M3) in the shipping release profile,
#       because LANE-3 is an optimizer fold a debug build does not make; `console-workload`'s
#       class-A console digests; and the realtime audits: `audit capi` and the per-effect allocation
#       and syscall audits. G5, `wasm-gates`' native owner of the cross-target digest corpora, is
#       `aarch64-release`'s own unfiltered workflow step (#1048), so no row or skip here reaches it.
#
# Known defects are expected failures, by name and by reason, in
# scripts/lib/aarch64-known-defects.py; each row names its issue. Before the main run each row's
# name must name exactly one test in the leg, because `--exact --skip` reaches every test binary.
# The main run skips the rows' tests. Each row's test then runs alone and must fail as that one
# test, with the row's reason (the AArch64 digest or assertion text the defect produces) in its
# panic. A pass, another panic or another failure reason fails the leg, so a fixed defect cannot
# leave a stale row and a new regression cannot hide behind an old one. Nothing else is skipped,
# and no job is.
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
source "$(dirname "${BASH_SOURCE[0]}")/lib/gate.sh"

# #2237: the debug leg's shards. Shards 1 and 2 name their packages; the last shard takes every
# other package of the leg. Each holds about a third of the leg's test time on the arm64 runner
# (run of PR #2236: capi 311 s, host-core 143 s, control-plane 54 s; graph-compiler 217 s,
# builtins 128 s, builtins-compiler 57 s, graph 50 s, effect-compiler 39 s, session 18 s,
# protocol 2 s; the other 25 crates 514 s).
debug_shard_count=3
debug_shard_packages=(
    ''
    'capi control-plane host-core'
    'builtins builtins-compiler effect-compiler graph graph-compiler protocol session'
)
host_web_shard=2

usage() {
    printf 'usage: run-aarch64-tests.sh debug [<shard>/%d] | release\n' "$debug_shard_count" >&2
    exit 2
}
[[ $# -eq 1 || $# -eq 2 ]] || usage
mode=$1
[[ "$mode" == debug || "$mode" == release ]] || usage
shard=
if [[ $# -eq 2 ]]; then
    [[ "$mode" == debug && "$2" =~ ^([1-9][0-9]*)/([1-9][0-9]*)$ ]] || usage
    shard=${BASH_REMATCH[1]}
    ((BASH_REMATCH[2] == debug_shard_count && shard <= debug_shard_count)) || usage
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

leg="$mode leg${shard:+ shard $shard/$debug_shard_count}"
fail() {
    printf 'aarch64 %s failure: %s\n' "$leg" "$1" >&2
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

known_defects=(python3 -B "$root/scripts/lib/aarch64-known-defects.py")
"${known_defects[@]}" --self-test >/dev/null || fail 'the known-defect judges failed their self-test'
# A leg may have no rows left: an empty table is read successfully and names nothing to skip.
row_table="$("${known_defects[@]}" rows "$mode")" || fail "the $mode expected-failure rows could not be read"
rows=()
[[ -z "$row_table" ]] || mapfile -t rows <<<"$row_table"

# --- no silent skip on a backend width ----------------------------------------------------------
silent_skip='(?:Backend::current\(\)|\bbackend|\bdispatch)\s*!=\s*(?:\w+::)*Backend::Simd[48]\s*\{[^{}]*?\breturn\b'
silent_skip+='|let\s+Some\([^=;]*\)\s*=\s*[^;]*?(?:_w[48]\b|Simd[48]|BankWidth::(?:Four|Eight))[^;]*?\belse\s*\{[^{}]*?\breturn\b'
skip_roots=(crates hosts/host-web tools/console-workload tools/wasm-gates tools/wasm-gate-corpus)
if found="$(rg -n -U --pcre2 --glob '*.rs' "$silent_skip" "${skip_roots[@]}")"; then
    printf '%s\n' "$found" >&2
    fail 'a test returns early on a SIMD backend width; make it width-agnostic or #[ignore] the other width with its reason'
else
    status=$?
    ((status == 1)) || fail "the no-silent-skip scan could not run (rg exit $status)"
fi

# --- expected failures ---------------------------------------------------------------------------
# Each row's name must name exactly one test among the binaries `cargo test "$@"` runs.
# A debug shard judges only its own rows, which name its packages (`shard_crates`).
check_skip_names() {
    local listing
    listing="$(mktemp)"
    cargo test "$@" -- --list >"$listing" || fail 'could not list the leg'"'"'s tests'
    "${known_defects[@]}" judge-skips "$mode" "$listing" "${shard_crates[@]}" ||
        fail 'an expected-failure row is ambiguous'
    rm -f "$listing"
}

# Runs `cargo test "$@" <target> -- --exact <name>` for one row, which must fail as that one test
# and for the row's reason.
expect_failure() {
    local row=$1 package selector name log status
    shift
    IFS='|' read -r _ package selector name <<<"$row"
    local select=(--lib)
    [[ "$selector" == test:* ]] && select=(--test "${selector#test:}")
    log="$(mktemp)"
    status=0
    cargo test "$@" "${select[@]}" -- --exact "$name" >"$log" 2>&1 || status=$?
    if ! "${known_defects[@]}" judge-test "$mode" "$name" "$log" "$status"; then
        cat "$log" >&2
        fail "expected failure $package $name did not fail as its row says"
    fi
    rm -f "$log"
}

# Skip arguments for a leg's main run: `--exact`, then one `--skip` per expected failure.
skip_arguments() {
    local row name
    printf '%s\n' --exact
    for row in "${rows[@]}"; do
        IFS='|' read -r _ _ _ name <<<"$row"
        printf '%s\n%s\n' --skip "$name"
    done
}
mapfile -t skips < <(skip_arguments)
shard_crates=()

# --- shards (#2237) -------------------------------------------------------------------------------
# A crate's resolved features depend on every package `cargo test` selects: a dev-dependency of one
# selected package (capi's `engine/test-support`, say) turns the feature on for every unit that
# links the crate. A shard selects fewer packages, so it passes, for each workspace crate the whole
# leg builds, the features the leg resolves for it, wherever cargo accepts them (a selected package
# or a direct dependency of one). It then requires every workspace crate in its own build to resolve
# exactly the leg's features, so a shard runs the same program as the whole leg. A crate that only
# reaches a shard transitively and resolves differently there fails here; move it into that shard
# or the shard of the crate that turns its feature on.
resolved_features() {
    cargo tree --quiet --locked --target "$target" -e normal,dev,build --prefix none \
        --format '{p}|{f}' "$@" |
        grep -F "($root/" | sed -E 's/ v[^|]*\|/|/; s/ \(\*\)$//' | LC_ALL=C sort -u
}

# Narrows `packages`, `features`, `rows` and `skips` from the whole leg to shard `$shard`.
select_debug_shard() {
    local leg_packages=() crate index
    for ((index = 1; index < ${#packages[@]}; index += 2)); do
        leg_packages+=("${packages[index]}")
    done
    local named=()
    shard_crates=()
    for ((index = 1; index < debug_shard_count; index++)); do
        for crate in ${debug_shard_packages[index]}; do
            [[ " ${leg_packages[*]} " == *" $crate "* ]] ||
                fail "shard $index names $crate, which the debug leg does not run"
            [[ " ${named[*]} " != *" $crate "* ]] || fail "$crate is named by two shards"
            named+=("$crate")
            if ((index == shard)); then shard_crates+=("$crate"); fi
        done
    done
    if ((shard == debug_shard_count)); then
        for crate in "${leg_packages[@]}"; do
            [[ " ${named[*]} " == *" $crate "* ]] || shard_crates+=("$crate")
        done
    fi
    ((${#shard_crates[@]} > 0)) || fail 'the shard has no package'

    local leg_features shard_features
    leg_features="$(resolved_features "${packages[@]}" --features "$features")" ||
        fail "could not resolve the whole leg's features"
    packages=()
    for crate in "${shard_crates[@]}"; do
        packages+=(-p "$crate")
    done
    features="$(
        python3 -B - "$root" "$leg_features" "${shard_crates[@]}" <<'PY'
import json
import subprocess
import sys

root, leg, selected = sys.argv[1], sys.argv[2], sys.argv[3:]
metadata = json.loads(subprocess.run(
    ["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1",
     "--manifest-path", f"{root}/Cargo.toml"], check=True, capture_output=True, text=True).stdout)
members = {package["name"]: package for package in metadata["packages"]}
accepted = set(selected)
for name in selected:
    accepted.update(dependency.get("rename") or dependency["name"]
                    for dependency in members[name]["dependencies"])
features = []
for line in leg.splitlines():
    crate, enabled = line.split("|", 1)
    if crate in accepted:
        features += [f"{crate}/{feature}" for feature in enabled.split(",")
                     if feature and feature != "default"]
print(",".join(features))
PY
    )" || fail 'could not derive the shard features'
    shard_features="$(resolved_features "${packages[@]}" ${features:+--features "$features"})" ||
        fail "could not resolve the shard's features"
    local drift
    drift="$(LC_ALL=C comm -13 <(printf '%s\n' "$leg_features") <(printf '%s\n' "$shard_features"))"
    [[ -z "$drift" ]] || {
        printf 'resolved in the shard, not in the whole leg: %s\n' $drift >&2
        fail "the shard builds a workspace crate with other features than the whole leg"
    }
    feature_args=()
    [[ -z "$features" ]] || feature_args=(--features "$features")

    # Each expected-failure row runs in the shard that holds its package.
    local row package kept=()
    for row in "${rows[@]}"; do
        IFS='|' read -r _ package _ _ <<<"$row"
        [[ " ${leg_packages[*]} " == *" $package "* ]] ||
            fail "expected-failure row package $package is not in the debug leg"
        if [[ " ${shard_crates[*]} " == *" $package "* ]]; then kept+=("$row"); fi
    done
    rows=("${kept[@]}")
    mapfile -t skips < <(skip_arguments)
}

# #1670: reuse the single browser/native corpus owner, including #1669's narrowed automation
# transcript. These exact tests run on NEON in both profiles; none is an expected failure.
# A debug shard runs them only if it is the host-web shard.
profile=()
[[ "$mode" == release ]] && profile=(--release)
if [[ -z "$shard" || "$shard" == "$host_web_shard" ]]; then
    cargo test --locked "${profile[@]}" -p host-web --features test-support --lib -- --exact \
        tests::native_identity_session_digest_pins_the_wasm_parity \
        tests::native_set_control_timeline_digest_pins_the_wasm_parity \
        tests::native_observation_timeline_digest_pins_the_wasm_parity \
        tests::native_automation_timeline_digest_pins_the_wasm_parity
fi

if [[ "$mode" == debug ]]; then
    source "$root/scripts/lib/product-crates.sh"
    product_list="$(product_crates "$root")" || fail "could not derive the product crates from capi"
    packages=()
    while read -r crate; do
        packages+=(-p "$crate")
    done <<<"$product_list"
    packages+=(-p dsp-reference -p conformance -p target-smoke)
    features=builtins-compiler/test-support,graph/test-support
    features+=,host-core/test-support,effect-compiler/test-support,protocol/test-support
    features+=,engine/realtime-audit,math/lane,parametric-eq/test-support,builtins/test-support
    feature_args=(--features "$features")
    if [[ -n "$shard" ]]; then
        select_debug_shard
    fi
    check_skip_names --locked --all-targets "${packages[@]}" "${feature_args[@]}"
    cargo test --locked --all-targets "${packages[@]}" "${feature_args[@]}" -- "${skips[@]}"
    # --all-targets runs no doctest (#1422); lane's CanonicalFpEnv guarantee is target-specific,
    # so AArch64 runs the same packages' doctests with the same features.
    cargo test --locked --doc "${packages[@]}" "${feature_args[@]}"
    # The reference session's retained rows at four lanes (#1304): libtest captures a passing
    # test's output, so the main run never shows them. Run the budget test alone, uncaptured, with
    # the main run's packages and features so nothing rebuilds; its four-lane ceilings are set from
    # these rows. It is capi's test, so a shard runs it only if it holds capi.
    if [[ " ${packages[*]} " == *" -p capi "* ]]; then
        cargo test --locked "${packages[@]}" "${feature_args[@]}" --test resource_lifecycle -- \
            --exact reference_session_retained_rows_stay_within_their_budgets --nocapture
    fi
    for row in "${rows[@]}"; do
        expect_failure "$row" --locked "${packages[@]}" "${feature_args[@]}"
    done
    printf 'aarch64 %s: PASS (%d of %d leg crates, %d expected failures)\n' \
        "$leg" "$((${#packages[@]} / 2))" "$(($(wc -l <<<"$product_list") + 3))" "${#rows[@]}"
    exit 0
fi

gates=(-p lane -p math --features math/lane)
check_skip_names --locked --release "${gates[@]}"
cargo test --locked --release "${gates[@]}" -- "${skips[@]}"
for row in "${rows[@]}"; do
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
"${audit[@]}" de-esser | tee "$binary_dir/aarch64-de-esser-audit.json"
rg -qF '"total_violations":0' "$binary_dir/aarch64-de-esser-audit.json" ||
    fail 'audit de-esser reported violations'
rg -qF '"bank_available":true' "$binary_dir/aarch64-de-esser-audit.json" ||
    fail 'audit de-esser bound no bank at the native width'
"${audit[@]}" dynamic-eq | tee "$binary_dir/aarch64-dynamic-eq-audit.json"
rg -qF '"total_violations":0' "$binary_dir/aarch64-dynamic-eq-audit.json" ||
    fail 'audit dynamic-eq reported violations'
rg -qF '"bank_available":true' "$binary_dir/aarch64-dynamic-eq-audit.json" ||
    fail 'audit dynamic-eq bound no bank at the native width'

printf 'aarch64 release leg: PASS (%d expected failures)\n' "${#rows[@]}"
