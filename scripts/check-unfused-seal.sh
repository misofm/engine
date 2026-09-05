#!/usr/bin/env bash
# The unfused multiply-add contract's container (issue #163 phase 2).
#
# Since phase 2 the numeric contract has no fused multiply-add anywhere: `Lane::fma` is
# `(a * b) + c` with two roundings, identically on every backend, and the exact software emulation
# that used to stand behind it on wasm is deleted. `docs/rulings/unfused-multiply-add-audit.md`
# records the nineteen sites and why none of them keeps an exact path.
#
# That makes this seal the inverse shape of the fast dB tier's `clippy.toml` crossings (formerly
# `check-fast-db-seal.sh`, retired once its migration was mutation-proven). The fast dB tier
# admits exactly six named crossings; this one admits *none*. The property being sealed is an
# **absence**,
# and an absence is the easiest thing in a codebase to lose by accident:
#
#   * `f32::mul_add` is one method call away and reads like an optimisation;
#   * `wide`'s `mul_add` is fused on x86 and NEON and unfused elsewhere, so a single forwarded call
#     would silently make the contract per-backend -- the exact split the lane crate exists to
#     prevent, and the one no single-target test can see;
#   * wasm relaxed SIMD's `f32x4_relaxed_madd` is *permitted to fuse or not at the engine's
#     discretion*, so it would make the contract nondeterministic across browsers.
#
# None of those would fail a build, and only the last would even be visible in a diff as an
# obviously numeric change. So the vocabulary is refused mechanically, in both directions: an
# unregistered call fails, and a registered exemption that no longer exists *also* fails, so the
# roster cannot quietly describe a tree that has moved on.
set -euo pipefail

# ---------------------------------------------------------------------------------------------
# --self-test: every rule in this file gets a mutation that must turn it red, and the structural
# exemption gets mutations that must stay green -- otherwise the seal could be "hardened" into
# refusing the tree it is supposed to describe. Folded in from the former `test-unfused-seal.sh`
# (issue #104-shape: a self-test cannot drift from its subject, cannot be omitted from a workflow
# separately, and gets its positive control for free from the gate's own green run).
#
# The green cases matter more here than they do for most seals. This seal's vocabulary includes
# the word `mul_add`, which appears in prose all over the workspace precisely *because* the
# operation was removed: `lane/src/lib.rs` explains why the fusion is gone, `lane_math.rs` records
# the precedent, and this file names every spelling. A seal that could be tripped by documenting
# it would be unusable.
unfused_seal_self_test() {
    local scratch_root root
    root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
    scratch_root="$(mktemp -d)"
    trap 'rm -rf -- "$scratch_root"' RETURN
    local passed=0 failed=0


    # Builds a minimal synthetic workspace that the real checker passes: the two dispatch points
    # stating the unfused contract, a retired `softfma.rs`, and both registered exemptions -- the
    # audit's seven marked fused calls and gate G5's one.
    create_fixture() {
        local tree
        tree="$(mktemp -d "$scratch_root/fixture-XXXXXX")"

        mkdir -p "$tree/crates/lane/src" "$tree/tools/audit/src" "$tree/hosts" "$tree/sidecars"

        cat >"$tree/crates/lane/src/wide_impl.rs" <<'EOF'
    //! One `Lane` body for both `wide` widths. `mul_add` is never forwarded.
    macro_rules! impl_lane_for_wide {
        ($simd:ty, $uint:ty, $width:literal, $cascade_depth:literal) => {
            impl $crate::Lane for $simd {
                #[inline(always)]
                fn fma(self, b: Self, c: Self) -> Self {
                    // Two roundings, natively, on every backend.
                    (self * b) + c
                }
            }
        };
}
EOF

    cat >"$tree/crates/lane/src/scalar.rs" <<'EOF'
//! The scalar oracle. `f32::mul_add` is deliberately not called.
impl Lane for f32 {
    #[inline(always)]
    fn fma(self, b: Self, c: Self) -> Self {
        (self * b) + c
    }
}
EOF

    cat >"$tree/crates/lane/src/softfma.rs" <<'EOF'
//! The MXCSR helpers gate G6 needs. The software FMA was retired in #163 phase 2.
pub const MXCSR_FTZ: u32 = 0x8000;
EOF

    # Seven fused calls, two of them on one line, each within six lines of a marker.
    cat >"$tree/tools/audit/src/unfused_fma.rs" <<'EOF'
//! The audit. Keeps the retired fused arm so the unfused contract can be measured against it.
fn step_fused(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    let d1 = a.mul_add(b, c);
    // UNFUSED-SEAL-EXEMPT
    let d2 = a.mul_add(c, b);
    // UNFUSED-SEAL-EXEMPT (two calls)
    let d3 = a.mul_add(d1, b.mul_add(d2, c));
    d3
}
fn one_pole_fused(c: f32, x: f32, y: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    c.mul_add(x - y, y)
}
fn mix_fused(x: f32, g: f32, m: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    m.mul_add(g, x)
}
fn matrix_fused(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    a.mul_add(b, c)
}
EOF
    mkdir -p "$tree/tools/wasm-gates/tests"
    cat >"$tree/tools/wasm-gates/tests/g5_native_corpus.rs" <<'EOF'
//! Gate G5's native leg. Keeps a fused reference so the `lane_fma` case cannot pass vacuously.
fn fused_reference(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    a.mul_add(b, c)
}
EOF
    printf '%s\n' "$tree"
}

expect_success() {
    local name=$1 tree=$2
    if bash "$root/scripts/check-unfused-seal.sh" "$tree" >/dev/null 2>&1; then
        printf 'green ok   %s\n' "$name"
        passed=$((passed + 1))
    else
        printf 'GREEN FAIL %s -- the seal refuses a tree it should accept\n' "$name" >&2
        bash "$root/scripts/check-unfused-seal.sh" "$tree" 2>&1 | sed 's/^/           /' >&2
        failed=$((failed + 1))
    fi
}

expect_failure() {
    local name=$1 tree=$2
    if bash "$0" "$tree" >/dev/null 2>&1; then
        printf 'RED FAIL   %s -- the seal accepted a tree it should refuse\n' "$name" >&2
        failed=$((failed + 1))
    else
        printf 'red ok     %s\n' "$name"
        passed=$((passed + 1))
    fi
}

# -------------------------------------------------------------------------------------------
# Green: the synthetic tree, and the real one.
# -------------------------------------------------------------------------------------------
expect_success baseline-synthetic "$(create_fixture)"
expect_success baseline-real-tree "$root"

# Green: prose may name the vocabulary freely. This is the case the seal must not regress on.
tree=$(create_fixture)
cat >>"$tree/crates/lane/src/lib.rs" <<'EOF'
//! Fusion exists nowhere. `mul_add(` is not called; `f32::mul_add(a, b, c)` would be a call, and
//! `_mm256_fmadd_ps(x, y, z)` would be another. Writing about them is not calling them.
EOF
expect_success prose-may-name-the-vocabulary "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 1 -- a dispatch point that stops stating the unfused contract.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
sed -i 's/(self \* b) + c/self.something(b, c)/' "$tree/crates/lane/src/scalar.rs"
expect_failure scalar-dispatch-no-longer-unfused "$tree"

tree=$(create_fixture)
sed -i 's/(self \* b) + c/self * b + c/' "$tree/crates/lane/src/wide_impl.rs"
expect_failure wide-dispatch-loses-its-parentheses "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 2 -- the contract split by target. This is the mutation that matters most: it is the
# tempting change, and no single-target test would catch it.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
python3 - "$tree" <<'PY'
import sys, pathlib
p = pathlib.Path(sys.argv[1]) / "crates/lane/src/wide_impl.rs"
s = p.read_text().replace(
    "                (self * b) + c\n",
    '                #[cfg(target_arch = "x86_64")]\n'
    "                {\n"
    "                    self.mul_add(b, c)\n"
    "                }\n"
    "                (self * b) + c\n",
)
p.write_text(s)
PY
expect_failure contract-split-by-target "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 3 -- a fused call in an unregistered file.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
mkdir -p "$tree/crates/compressor/src"
cat >"$tree/crates/compressor/src/kernel.rs" <<'EOF'
fn detector(a: f32, b: f32, c: f32) -> f32 { a.mul_add(b, c) }
EOF
expect_failure unregistered-fused-call "$tree"

# The same, through a raw intrinsic rather than the method.
tree=$(create_fixture)
mkdir -p "$tree/crates/compressor/src"
cat >"$tree/crates/compressor/src/kernel.rs" <<'EOF'
unsafe fn detector(a: __m256, b: __m256, c: __m256) -> __m256 { _mm256_fmadd_ps(a, b, c) }
EOF
expect_failure unregistered-fused-intrinsic "$tree"

# And through wasm relaxed SIMD, which may fuse at the engine's discretion.
tree=$(create_fixture)
mkdir -p "$tree/crates/compressor/src"
cat >"$tree/crates/compressor/src/kernel.rs" <<'EOF'
fn detector(a: v128, b: v128, c: v128) -> v128 { f32x4_relaxed_madd(a, b, c) }
EOF
expect_failure unregistered-relaxed-madd "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 4 -- registry rot, wrong count, missing marker.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
rm "$tree/tools/audit/src/unfused_fma.rs"
expect_failure registered-exemption-deleted "$tree"

tree=$(create_fixture)
cat >>"$tree/tools/audit/src/unfused_fma.rs" <<'EOF'
fn extra(a: f32, b: f32, c: f32) -> f32 {
    // UNFUSED-SEAL-EXEMPT
    a.mul_add(b, c)
}
EOF
expect_failure exemption-call-count-grew "$tree"

tree=$(create_fixture)
sed -i '0,/    \/\/ UNFUSED-SEAL-EXEMPT$/{/    \/\/ UNFUSED-SEAL-EXEMPT$/d}' \
    "$tree/tools/audit/src/unfused_fma.rs"
expect_failure exemption-call-without-marker "$tree"

# -------------------------------------------------------------------------------------------
# Red: rule 6 -- the retired emulation regrows in the file that kept its name.
# -------------------------------------------------------------------------------------------
tree=$(create_fixture)
cat >>"$tree/crates/lane/src/softfma.rs" <<'EOF'
pub fn fma_f32_via_f64(a: f32, b: f32, c: f32) -> f32 {
    ((f64::from(a) * f64::from(b)) + f64::from(c)) as f32
}
EOF
expect_failure software-fma-restored "$tree"

# Exact marker boundary: the raw rolling window is the call line plus six preceding lines.
tree=$(create_fixture)
sed -i '0,/UNFUSED-SEAL-EXEMPT/{/UNFUSED-SEAL-EXEMPT/c\    // UNFUSED-SEAL-EXEMPT\
    // one\
    // two\
    // three\
    // four\
    // five
}' "$tree/tools/audit/src/unfused_fma.rs"
expect_success marker-six-lines-before-call "$tree"

tree=$(create_fixture)
sed -i '0,/UNFUSED-SEAL-EXEMPT/{/UNFUSED-SEAL-EXEMPT/c\    // UNFUSED-SEAL-EXEMPT\
    // one\
    // two\
    // three\
    // four\
    // five\
    // six
}' "$tree/tools/audit/src/unfused_fma.rs"
expect_failure marker-seven-lines-before-call "$tree"

tree=$(create_fixture)
sed -i '0,/UNFUSED-SEAL-EXEMPT/{/UNFUSED-SEAL-EXEMPT/{N;s|.*\n.*|    let d1 = a.mul_add(b, c); // UNFUSED-SEAL-EXEMPT|;}}' \
    "$tree/tools/audit/src/unfused_fma.rs"
expect_success marker-on-call "$tree"

tree=$(create_fixture)
rm -rf "$tree/sidecars"
expect_failure required-root-missing "$tree"

tree=$(create_fixture)
rm "$tree/crates/lane/src/softfma.rs"
expect_failure required-retired-source-missing "$tree"

# Bounded semantic controls for the frozen grammar and population rules.
tree=$(create_fixture)
mkdir -p "$tree/sidecars/empty"
cat >>"$tree/hosts/prose.rs" <<'EOF'
//! `mul_add(` and `_mm256_fmadd_ps(` are vocabulary in prose only.
EOF
expect_success prose-only-and-empty-root "$tree"

tree=$(create_fixture)
spaced_tree="$scratch_root/fixture with spaces"
cp -R "$tree" "$spaced_tree"
expect_success root-path-with-spaces "$spaced_tree"
relative_parent=$(dirname "$spaced_tree")
relative_name=$(basename "$spaced_tree")
(cd "$relative_parent" && expect_success relative-fixture-root "$relative_name")

# Every fallible producer is driven through both failure shapes. The shim delegates first, so its
# payload is exactly what an otherwise-valid fixture would produce; FAULT_EMPTY suppresses that
# payload for the paired error-only case. Counters select late consumers after earlier calls pass.
shim="$scratch_root/fault-shim"
mkdir -p "$shim"
cat >"$shim/tool" <<'EOF'
#!/usr/bin/env bash
tool=${0##*/}; real="/usr/bin/$tool"; args=" $* "; hit=0
counter() { local key=$1 n=0 file; file="$FAULT_STATE/$key"; [[ -f "$file" ]] && n=$(<"$file"); n=$((n+1)); printf '%s' "$n" >"$file"; printf '%s' "$n"; }
case "$FAULT_KIND:$tool" in
  strip-wide:sed) [[ "$args" == *wide_impl.rs* ]] && hit=1 ;;
  strip-late:sed) [[ "$args" == *g5_native_corpus.rs* ]] && hit=1 ;;
  dispatch-scalar:rg) [[ "$args" == *" -F "* && "$args" == *"(self * b) + c"* ]] && [[ $(counter dispatch) == 2 ]] && hit=1 ;;
  body-scalar:awk) [[ "$args" == *"fn fma"* ]] && [[ $(counter body) == 2 ]] && hit=1 ;;
  predicate-scalar:rg) [[ "$args" == *target_feature* ]] && [[ $(counter predicate) == 2 ]] && hit=1 ;;
  registry-cat:cat) [[ $# == 0 ]] && hit=1 ;;
  registry-files:awk) [[ "$args" == *'print $1'* ]] && hit=1 ;;
  registry-sort:sort) [[ $(counter sort) == 1 ]] && hit=1 ;;
  discovery:rg) [[ "$args" == *" -l "* ]] && hit=1 ;;
  discovery-sort:sort) [[ $(counter sort) == 2 ]] && hit=1 ;;
  occurrence-late:rg) [[ "$args" == *" -o "* ]] && [[ $(counter occurrence) == 2 ]] && hit=1 ;;
  count-late:wc) [[ $(counter wc) == 2 ]] && hit=1 ;;
  membership-late:grep) [[ "$args" == *g5_native_corpus.rs* ]] && hit=1 ;;
  marker-late:awk) [[ "$args" == *g5_native_corpus.rs* ]] && hit=1 ;;
  aggregate:awk) [[ "$args" == *"total += $2"* ]] && hit=1 ;;
  recount-late:rg) [[ "$args" == *" -o "* ]] && [[ $(counter occurrence) == 6 ]] && hit=1 ;;
  retired:rg) [[ "$args" == *fma_f32_via_f64* && "$args" == *softfma.rs* ]] && hit=1 ;;
esac
if [[ "$hit" == 1 ]]; then
    [[ "${FAULT_EMPTY:-0}" == 1 ]] || "$real" "$@" || true
    printf 'INJECTED-%s\n' "$FAULT_KIND" >&2
    exit 9
fi
exec "$real" "$@"
EOF
chmod +x "$shim/tool"
for tool in sed rg awk sort wc grep cat; do ln -s tool "$shim/$tool"; done

expect_producer_failure() {
    local kind=$1 diagnostic=$2 mode output rc
    for mode in full empty; do
        tree=$(create_fixture); state="$scratch_root/state-$kind-$mode"; mkdir -p "$state"
        if [[ "$mode" == empty ]]; then empty=1; else empty=0; fi
        if output="$(FAULT_KIND="$kind" FAULT_EMPTY="$empty" FAULT_STATE="$state" \
            PATH="$shim:$PATH" bash "$root/scripts/check-unfused-seal.sh" "$tree" 2>&1)"; then rc=0; else rc=$?; fi
        if [[ "$rc" != 0 && "$output" == *"INJECTED-$kind"* && "$output" == *"$diagnostic"* ]]; then
            printf 'producer red %s/%s\n' "$kind" "$mode"; passed=$((passed + 1))
        else
            printf 'PRODUCER FAIL %s/%s status=%s output=%s\n' "$kind" "$mode" "$rc" "$output" >&2
            failed=$((failed + 1))
        fi
    done
}

expect_producer_failure strip-wide 'comment stripping failed for crates/lane/src/wide_impl.rs (sed status 9)'
expect_producer_failure strip-late 'comment stripping failed for tools/wasm-gates/tests/g5_native_corpus.rs (sed status 9)'
expect_producer_failure dispatch-scalar 'crates/lane/src/scalar.rs dispatch search errored (rg status 9)'
expect_producer_failure body-scalar 'crates/lane/src/scalar.rs fma-body extraction errored (awk status 9)'
expect_producer_failure predicate-scalar 'crates/lane/src/scalar.rs fma-body predicate errored (rg status 9)'
expect_producer_failure registry-cat 'exemption registry production failed (cat status 9)'
expect_producer_failure registry-files 'exemption registry filename extraction failed (awk status 9)'
expect_producer_failure registry-sort 'exemption registry sort failed (sort status 9)'
expect_producer_failure discovery 'candidate discovery errored (rg status 9)'
expect_producer_failure discovery-sort 'candidate discovery sort failed (sort status 9)'
expect_producer_failure occurrence-late 'fused-call search failed for tools/wasm-gates/tests/g5_native_corpus.rs (rg status 9)'
expect_producer_failure count-late 'fused-call count failed for tools/wasm-gates/tests/g5_native_corpus.rs (wc status 9)'
expect_producer_failure membership-late 'registration membership search errored for tools/wasm-gates/tests/g5_native_corpus.rs (grep status 9)'
expect_producer_failure marker-late 'tools/wasm-gates/tests/g5_native_corpus.rs marker-window validation errored (awk status 9)'
expect_producer_failure aggregate 'registry aggregate parser failed (awk status 9)'
expect_producer_failure recount-late 'fused-call search failed for tools/wasm-gates/tests/g5_native_corpus.rs (rg status 9)'
expect_producer_failure retired 'retired software-FMA search errored (rg status 9)'

# Prove three exact status checks matter. The original checker must emit the focused diagnostic;
# after one verified call-site edit, the same injected run must reach unexpected success (exit 97).
prove_status_mutant() {
    local label=$1 kind=$2 diagnostic=$3 edit=$4 mutant output rc
    tree=$(create_fixture); mutant="$scratch_root/mutant-$label.sh"; cp "$root/scripts/check-unfused-seal.sh" "$mutant"
    before=$(cksum <"$mutant")
    sed -i "$edit" "$mutant"
    after=$(cksum <"$mutant")
    [[ "$before" != "$after" ]] || { printf 'COUNTER FAIL %s edit did not apply\n' "$label" >&2; failed=$((failed+1)); return; }
    state="$scratch_root/state-control-$label"; mkdir -p "$state"
    output="$(FAULT_KIND="$kind" FAULT_EMPTY=0 FAULT_STATE="$state" PATH="$shim:$PATH" bash "$root/scripts/check-unfused-seal.sh" "$tree" 2>&1)" && rc=0 || rc=$?
    [[ "$rc" != 0 && "$output" == *"INJECTED-$kind"* && "$output" == *"$diagnostic"* ]] || { printf 'COUNTER FAIL %s original status=%s output=%s\n' "$label" "$rc" "$output" >&2; failed=$((failed+1)); return; }
    rm -rf "$state"; mkdir -p "$state"
    output="$(FAULT_KIND="$kind" FAULT_EMPTY=0 FAULT_STATE="$state" PATH="$shim:$PATH" bash -c 'if bash "$1" "$2"; then printf "ASSERT %s unexpected success\\n" "$3" >&2; exit 97; fi; exit $?' _ "$mutant" "$tree" "$label" 2>&1)" && rc=0 || rc=$?
    if [[ "$rc" == 97 && "$output" == *"ASSERT $label unexpected success"* ]]; then
        printf 'counter red %s (status 97)\n' "$label"; passed=$((passed+1))
    else
        printf 'COUNTER FAIL %s status=%s output=%s\n' "$label" "$rc" "$output" >&2; failed=$((failed+1))
    fi
}

prove_status_mutant discovery discovery 'candidate discovery errored (rg status 9)' \
    '/if candidates_raw=/,/esac/{s/\*) printf .*candidate discovery errored.* ;;/\*) rc=0 ;;/}'
prove_status_mutant late-occurrence occurrence-late 'fused-call search failed for tools/wasm-gates/tests/g5_native_corpus.rs (rg status 9)' \
    '/while read -r file count/,/done <<<"$registry_raw"/{s/\[\[ "$rc" == 0 \]\] || exit "$rc"/[[ "$rc" == 0 ]] || rc=0/}'
prove_status_mutant retired retired 'retired software-FMA search errored (rg status 9)' \
    '/if retired_match=/,/^fi$/{s/elif \[\[ "$rc" != 1 \]\]; then/elif false; then/}'

# -------------------------------------------------------------------------------------------
printf '\nunfused seal mutations: %s passed, %s failed\n' "$passed" "$failed"
[[ "$failed" == "0" ]] || return 1
}


if [[ "${1:-}" == "--self-test" ]]; then
    unfused_seal_self_test
    exit $?
fi

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"

fail() {
    printf 'unfused seal failure: %s\n' "$1" >&2
    exit 1
}

# The two dispatch points. These define the contract and must state it in the unfused form.
dispatch_wide=crates/lane/src/wide_impl.rs
dispatch_scalar=crates/lane/src/scalar.rs

# The exemption registry: source file, and the exact number of fused calls it may make.
#
# There are exactly two, and both exist for the same reason: to compute the contract that was
# retired, so that the contract that replaced it can be measured *against* something.
#
#   * the audit that justified the change -- without its fused arm the bounds in
#     `docs/rulings/unfused-multiply-add-audit.md` could never be reproduced, because the fused arm
#     no longer exists anywhere else in the tree;
#   * gate G5's native leg -- its `lane_fma` case asserts that `Lane::fma` equals a written-out
#     multiply and add, and that assertion would pass vacuously the moment the corpus operands
#     stopped separating the two forms. The fused reference is what keeps it honest. It lives in
#     the native gate rather than the corpus crate because the corpus is compiled into the wasm
#     guest, where `mul_add` is unfused and a "fused" reference would silently become a second copy
#     of the unfused one.
#
# Both are evidence code: neither is reachable from a render path, and neither is `Lane::fma`.
#
# Adding a row here is the deliberate act the seal exists to require. A new row means someone has
# decided a fused multiply-add belongs in the workspace again, which is an owner-ruling-sized
# decision (the last one moved every pinned bit in the tree).
exemption_registry() {
    cat <<'EOF'
tools/audit/src/unfused_fma.rs 7
tools/wasm-gates/tests/g5_native_corpus.rs 1
EOF
}

# The number of fused calls the container claims exist in the whole workspace. "No fused
# multiply-add anywhere" is only a meaningful statement if the residue is written down and checked.
expected_fused_call_count=8

# A call of a fused multiply-add, in any of its spellings. The `(` is what separates a call from a
# `use` line or a doc mention.
#
# Counted against *comment-stripped* source: this contract is discussed at length in prose all over
# the workspace -- `lib.rs` explains why the fusion is gone, `lane_math.rs` records the precedent,
# this file names every spelling -- and a seal that counts a comment as a call is a seal that can
# be tripped by writing documentation. Stripping `//` to end of line can also blank a `//` inside a
# string literal, which would only ever hide a non-call, never admit a real one.
#
# The optional turbofish is not decoration: without it `mul_add::<f32>(x, y, z)` would not match,
# and the count that makes "exactly N" checkable could be evaded by spelling the type explicitly.
call_pattern='\b(mul_add|mul_neg_add|mul_sub|mul_neg_sub|fmaf|fma_f32_via_f64|fma_f32x4_soft|fma_f32x8_soft|f32x4_relaxed_madd|f32x4_relaxed_nmadd|f64x2_relaxed_madd|_mm256_fmadd_ps|_mm256_fmsub_ps|_mm256_fnmadd_ps|_mm_fmadd_ps|_mm_fmadd_ss|vfmaq_f32|vfmsq_f32)\s*(::<[^>]*>)?\s*\('
# The same pattern in POSIX ERE for `awk`, which has no `\s` and no PCRE escapes.
awk_call_pattern='(mul_add|mul_neg_add|mul_sub|mul_neg_sub|fmaf|fma_f32_via_f64|fma_f32x4_soft|fma_f32x8_soft|f32x4_relaxed_madd|f32x4_relaxed_nmadd|f64x2_relaxed_madd|_mm256_fmadd_ps|_mm256_fmsub_ps|_mm256_fnmadd_ps|_mm_fmadd_ps|_mm_fmadd_ss|vfmaq_f32|vfmsq_f32)([[:space:]]*::<[^>]*>)?[[:space:]]*[(]'
checked_strip() {
    local file="$1" output rc
    if output="$(sed 's://.*::' "$file")"; then rc=0; else rc=$?; fi
    if [[ "$rc" != 0 ]]; then
        printf '%s\n' "$output" >&2
        printf 'unfused seal failure: comment stripping failed for %s (sed status %s)\n' "$file" "$rc" >&2
        return "$rc"
    fi
    printf '%s' "$output"
}

# Keep the historical sed interpretation and count occurrences rather than matching lines.
# Every producer is captured before its consumer runs, including rg's legitimate no-match (1).
count_calls() {
    local file="$1" stripped matches rc count
    stripped="$(checked_strip "$file")" || return $?
    if [[ -n "$stripped" ]]; then
        if matches="$(rg -o -e "$call_pattern" <<<"$stripped")"; then rc=0; else rc=$?; fi
    else
        matches=''; rc=1
    fi
    case "$rc" in
        0|1) ;;
        *)
            printf '%s\n' "$matches" >&2
            printf 'unfused seal failure: fused-call search failed for %s (rg status %s)\n' "$file" "$rc" >&2
            return "$rc" ;;
    esac
    if [[ -z "$matches" ]]; then
        printf '0'
        return 0
    fi
    if count="$(wc -l <<<"$matches")"; then rc=0; else rc=$?; fi
    if [[ "$rc" != 0 ]]; then
        printf '%s\n' "$count" >&2
        printf 'unfused seal failure: fused-call count failed for %s (wc status %s)\n' "$file" "$rc" >&2
        return "$rc"
    fi
    printf '%s' "${count//[[:space:]]/}"
}

[[ -f "$dispatch_wide" ]] || fail "the vector dispatch point $dispatch_wide is missing"
[[ -f "$dispatch_scalar" ]] || fail "the scalar dispatch point $dispatch_scalar is missing"

# ---------------------------------------------------------------------------------------------
# 1. Both dispatch points state the contract in the unfused form.
#
# A positive check, not just the absence of `mul_add`: deleting the body would pass a pure absence
# test. `(self * b) + c` is the literal contract, and the parentheses are part of it -- they are
# what stop a future reader from "simplifying" the expression into an order the backends could
# reassociate.
# ---------------------------------------------------------------------------------------------
for dispatch in "$dispatch_wide" "$dispatch_scalar"; do
    dispatch_source="$(checked_strip "$dispatch")" || exit $?
    if dispatch_match="$(rg -F '(self * b) + c' <<<"$dispatch_source")"; then rc=0; else rc=$?; fi
    if [[ "$rc" != 0 ]]; then
        printf '%s\n' "$dispatch_match" >&2
        if [[ "$rc" == 1 ]]; then
            fail "$dispatch no longer states 'Lane::fma' as the unfused '(self * b) + c'"
        fi
        fail "$dispatch dispatch search errored (rg status $rc)"
    fi
done

# ---------------------------------------------------------------------------------------------
# 2. The contract is not split by target.
#
# The failure this catches is the tempting one: fuse where the hardware has an instruction, unfuse
# where it does not. That reintroduces a per-backend numeric contract, which no single-target test
# can detect -- only the wasm gate legs would, and only after a re-pin had already hidden it. So
# the fma body is required to contain no conditional compilation at all.
# ---------------------------------------------------------------------------------------------
for dispatch in "$dispatch_wide" "$dispatch_scalar"; do
    dispatch_source="$(checked_strip "$dispatch")" || exit $?
    if body="$(awk '/fn fma\(self, b: Self, c: Self\) -> Self \{/ { capture = 1 }
             capture { print }
             capture && /^ *\}$/ { capture = 0 }' <<<"$dispatch_source")"; then
        :
    else
        rc=$?
        printf '%s\n' "$body" >&2
        fail "$dispatch fma-body extraction errored (awk status $rc)"
    fi
    [[ -n "$body" ]] || fail "$dispatch has no recognisable 'Lane::fma' body"
    if forbidden_body="$(rg 'cfg\s*\(|cfg!|target_arch|target_feature' <<<"$body")"; then rc=0; else rc=$?; fi
    if [[ "$rc" == 0 ]]; then
        fail "$dispatch conditions 'Lane::fma' on the target -- the contract must not be per-backend"
    elif [[ "$rc" != 1 ]]; then
        printf '%s\n' "$forbidden_body" >&2
        fail "$dispatch fma-body predicate errored (rg status $rc)"
    fi
done

# ---------------------------------------------------------------------------------------------
# 3. No unregistered file calls a fused multiply-add.
# ---------------------------------------------------------------------------------------------
if registry_raw="$(exemption_registry)"; then rc=0; else rc=$?; fi
if [[ "$rc" != 0 ]]; then
    printf '%s\n' "$registry_raw" >&2
    fail "exemption registry production failed (cat status $rc)"
fi
if registry_files_unsorted="$(awk '{ print $1 }' <<<"$registry_raw")"; then rc=0; else rc=$?; fi
if [[ "$rc" != 0 ]]; then
    printf '%s\n' "$registry_files_unsorted" >&2
    fail "exemption registry filename extraction failed (awk status $rc)"
fi
if registered_files="$(sort <<<"$registry_files_unsorted")"; then rc=0; else rc=$?; fi
if [[ "$rc" != 0 ]]; then
    printf '%s\n' "$registered_files" >&2
    fail "exemption registry sort failed (sort status $rc)"
fi

if candidates_raw="$(rg -l -e "$call_pattern" crates hosts tools sidecars --glob '*.rs')"; then rc=0; else rc=$?; fi
case "$rc" in
    0|1) ;;
    *) printf '%s\n' "$candidates_raw" >&2; fail "candidate discovery errored (rg status $rc)" ;;
esac
if candidates="$(sort <<<"$candidates_raw")"; then rc=0; else rc=$?; fi
if [[ "$rc" != 0 ]]; then
    printf '%s\n' "$candidates" >&2
    fail "candidate discovery sort failed (sort status $rc)"
fi
while IFS= read -r file; do
    [[ -n "$file" ]] || continue
    # Re-test against comment-stripped source: `rg -l` above matched prose too.
    if calls="$(count_calls "$file")"; then rc=0; else rc=$?; fi
    [[ "$rc" == 0 ]] || exit "$rc"
    # `[[ ... ]] && continue` would be a `set -e` trap here: when the test is false the list
    # returns non-zero and the shell exits silently, which is exactly the failure this seal must
    # never have (an exit code with no message reads as a refusal nobody can act on).
    if [[ "$calls" == "0" ]]; then
        continue
    fi
    if membership="$(grep -xF "$file" <<<"$registered_files")"; then rc=0; else rc=$?; fi
    if [[ "$rc" != 0 ]]; then
        printf '%s\n' "$membership" >&2
        [[ "$rc" == 1 ]] && fail "fused multiply-add in $file -- the contract is unfused everywhere (#163 phase 2); \
see docs/rulings/unfused-multiply-add-audit.md"
        fail "registration membership search errored for $file (grep status $rc)"
    fi
done <<<"$candidates"

# ---------------------------------------------------------------------------------------------
# 4. Every registered exemption still exists, with exactly the registered call count, and every
#    call carries its marker.
#
# The marker must sit on the call or within the six lines above it, so the justification travels
# with the code rather than living in this script.
# ---------------------------------------------------------------------------------------------
while read -r file count; do
    [[ -n "$file" ]] || continue
    [[ -f "$file" ]] ||
        fail "registered exemption file $file does not exist -- the registry has rotted"

    if actual="$(count_calls "$file")"; then rc=0; else rc=$?; fi
    [[ "$rc" == 0 ]] || exit "$rc"
    [[ "$actual" == "$count" ]] ||
        fail "$file has $actual fused calls, the registry says $count"

    if unmarked="$(awk -v pattern="$awk_call_pattern" '
        { source = $0; sub(/\/\/.*/, "", source) }
        { for (i = 6; i >= 1; i--) window[i + 1] = window[i]; window[1] = $0 }
        source ~ pattern {
            marked = 0
            for (i = 1; i <= 7; i++) if (window[i] ~ /UNFUSED-SEAL-EXEMPT/) marked = 1
            if (!marked) print FILENAME ":" FNR
        }
    ' "$file")"; then rc=0; else rc=$?; fi
    if [[ "$rc" != 0 ]]; then
        printf '%s\n' "$unmarked" >&2
        fail "$file marker-window validation errored (awk status $rc)"
    fi
    [[ -z "$unmarked" ]] ||
        fail "fused call without an UNFUSED-SEAL-EXEMPT marker within six lines: $unmarked"
done <<<"$registry_raw"

# ---------------------------------------------------------------------------------------------
# 5. N is what the container says it is, counted two independent ways.
#
# The registry sum and a tree-wide recount must agree. One number is what the roster claims; the
# other is what the tree contains. A seal that only ever consults its own roster is checking its
# arithmetic, not the codebase.
# ---------------------------------------------------------------------------------------------
if declared="$(awk '{ total += $2 } END { print total + 0 }' <<<"$registry_raw")"; then rc=0; else rc=$?; fi
if [[ "$rc" != 0 ]]; then
    printf '%s\n' "$declared" >&2
    fail "registry aggregate parser failed (awk status $rc)"
fi
[[ "$declared" == "$expected_fused_call_count" ]] ||
    fail "the registry declares $declared fused calls, the container claims $expected_fused_call_count"

counted=0
while IFS= read -r file; do
    [[ -n "$file" ]] || continue
    if calls="$(count_calls "$file")"; then rc=0; else rc=$?; fi
    [[ "$rc" == 0 ]] || exit "$rc"
    counted=$((counted + calls))
done <<<"$candidates"
[[ "$counted" == "$expected_fused_call_count" ]] ||
    fail "found $counted fused calls in the tree, expected $expected_fused_call_count"

# ---------------------------------------------------------------------------------------------
# 6. The retired emulation stays retired.
#
# `softfma.rs` survives because it houses the MXCSR helpers gate G6 needs, and it kept its name
# because three policy files name that path. Keeping the name means the file could quietly regrow
# the thing it was named for, so the definition is refused explicitly rather than left to rule 3.
# ---------------------------------------------------------------------------------------------
[[ -f crates/lane/src/softfma.rs ]] || fail 'the retired soft-fma source crates/lane/src/softfma.rs is missing'
if retired_match="$(rg -n 'fn\s+fma_f32_via_f64\b|fn\s+fma_f32x[48]_soft\b' crates/lane/src/softfma.rs)"; then rc=0; else rc=$?; fi
if [[ "$rc" == 0 ]]; then
    fail 'the software FMA is retired (#163 phase 2) -- restoring it needs a ruling, not a commit'
elif [[ "$rc" != 1 ]]; then
    printf '%s\n' "$retired_match" >&2
    fail "retired software-FMA search errored (rg status $rc)"
fi

printf 'unfused seal: ok (no fused multiply-add on any path; %s registered audit calls)\n' \
    "$expected_fused_call_count"
