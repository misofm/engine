#!/usr/bin/env bash
# A policy script with no red mutation is decoration (master plan section 10, "POL").
#
# Every rule in `check-fast-db-seal.sh` gets a mutation that must turn it red, and the structural
# exemptions get mutations that must stay green -- otherwise the seal could be "hardened" into
# refusing the tree it is supposed to describe.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
policy="$root/scripts/check-fast-db-seal.sh"

scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

# Builds a minimal synthetic workspace that the real checker passes: the sealed module, its bound
# gate, the owning crate root, and the three registered crossing files with exactly the registered
# call counts and markers.
# `create_fixture` is always called in a command substitution, so it runs in a subshell and any
# counter it increments is lost when that subshell exits. `mktemp -d` is the fix: uniqueness comes
# from the filesystem rather than from state the caller cannot see.
create_fixture() {
    local tree
    tree="$(mktemp -d "$scratch_root/fixture-XXXXXX")"

    mkdir -p "$tree/crates/math/src" "$tree/crates/math/tests"
    cat >"$tree/crates/math/src/fast_db.rs" <<'EOF'
//! The sealed fast dB tier.
fn fast_exp2(x: f32) -> f32 { x }
fn fast_log2(x: f32) -> f32 { x }
pub fn fast_level_db(x: f32) -> f32 { fast_log2(x) }
pub fn fast_gain_from_db(x: f32) -> f32 { fast_exp2(x) }
EOF
    cat >"$tree/crates/math/tests/f1_fast_db_bounds.rs" <<'EOF'
//! Gate F1.
use math::fast_db::{fast_gain_from_db, fast_level_db};
#[test]
fn bound() { let _ = fast_level_db(1.0) + fast_gain_from_db(0.0); }
EOF
    cat >"$tree/crates/math/src/lib.rs" <<'EOF'
pub mod fast_db;
EOF

    local file
    local -a pairs=(
        "crates/compressor/src/kernel.rs X1 X2"
        "crates/gate-expander/src/kernel.rs X3 X4"
        "crates/multiband-compressor/src/lib.rs X5 X6"
    )
    local row name first second
    for row in "${pairs[@]}"; do
        read -r name first second <<<"$row"
        mkdir -p "$tree/$(dirname "$name")"
        cat >"$tree/$name" <<EOF
use math::fast_db::{fast_gain_from_db, fast_level_db};

fn level(x: f32) -> f32 {
    // FAST-DB-CROSSING $first: the detector level.
    fast_level_db(x)
}

fn gain(x: f32) -> f32 {
    // FAST-DB-CROSSING $second: the applied gain.
    fast_gain_from_db(x)
}
EOF
    done

    # The registered restatement site: a test that restates the tier's law to build a boundary
    # witness. Not a crossing -- no render path involved -- but enumerated all the same.
    mkdir -p "$tree/crates/gate-expander/tests"
    cat >"$tree/crates/gate-expander/tests/contract.rs" <<'EOF'
/// FAST-DB-RESTATEMENT: restates the kernel's law to construct a witness on the boundary.
fn detector_level_db(x: f32) -> f32 {
    math::fast_db::fast_level_db::<f32>(x)
}
#[test]
fn boundary() { assert!(detector_level_db(1.0).is_finite()); }
EOF

    mkdir -p "$tree/hosts" "$tree/tools"
    mkdir -p "$tree/scripts"
    cp "$policy" "$tree/scripts/check-fast-db-seal.sh"
    printf '%s' "$tree"
}

expect_pass() {
    local name=$1 mutation=$2 tree
    tree=$(create_fixture)
    ( cd "$tree" && eval "$mutation" )
    if ! bash "$policy" "$tree" >/dev/null 2>&1; then
        printf 'fast dB seal mutation test: expected GREEN but the policy failed: %s\n' "$name" >&2
        bash "$policy" "$tree" >&2 || true
        exit 1
    fi
}

expect_failure() {
    local name=$1 mutation=$2 tree
    tree=$(create_fixture)
    ( cd "$tree" && eval "$mutation" )
    if bash "$policy" "$tree" >/dev/null 2>&1; then
        printf 'fast dB seal mutation test: expected RED but the policy passed: %s\n' "$name" >&2
        exit 1
    fi
}

# The baseline must be green, or every red below is meaningless.
baseline=$(create_fixture)
bash "$policy" "$baseline" >/dev/null ||
    { printf 'fast dB seal mutation test: the unmutated fixture is not green\n' >&2; exit 1; }

# --- red: the seal refuses the vocabulary outside the registry -------------------------------
expect_failure 'new call site in an unregistered crate' \
    'mkdir -p crates/parametric-eq/src &&
     printf "use math::fast_db::fast_level_db;\nfn f(x: f32) -> f32 { fast_level_db(x) }\n" \
        > crates/parametric-eq/src/lib.rs'

expect_failure 'new call site in an unregistered host' \
    'mkdir -p hosts/host-web/src &&
     printf "fn f(x: f32) -> f32 { math::fast_db::fast_gain_from_db(x) }\n" \
        > hosts/host-web/src/lib.rs'

# --- red: the registered count is exact ------------------------------------------------------
expect_failure 'an extra call inside a registered crossing file' \
    'printf "fn extra(x: f32) -> f32 { fast_level_db(x) }\n" \
        >> crates/compressor/src/kernel.rs'

expect_failure 'a crossing call removed but its marker left behind' \
    'sed -i "s/^    fast_level_db(x)$/    x/" crates/compressor/src/kernel.rs'

# --- red: the roster cannot rot (issue #144 item 4) ------------------------------------------
expect_failure 'a registered crossing file deleted entirely' \
    'rm crates/gate-expander/src/kernel.rs'

expect_failure 'a crossing marker deleted' \
    'sed -i "/FAST-DB-CROSSING X5/d" crates/multiband-compressor/src/lib.rs'

expect_failure 'a crossing identifier duplicated onto two sites' \
    'sed -i "s/FAST-DB-CROSSING X4/FAST-DB-CROSSING X3/" crates/gate-expander/src/kernel.rs'

expect_failure 'a marker moved out of the six-line window above its call' \
    'printf "use math::fast_db::{fast_gain_from_db, fast_level_db};\n\nfn level(x: f32) -> f32 {\n    // FAST-DB-CROSSING X1: the detector level.\n    let a = 1;\n    let b = 2;\n    let c = 3;\n    let d = 4;\n    let e = 5;\n    let f = 6;\n    let g = 7;\n    fast_level_db(x)\n}\n\nfn gain(x: f32) -> f32 {\n    // FAST-DB-CROSSING X2: the applied gain.\n    fast_gain_from_db(x)\n}\n" > crates/compressor/src/kernel.rs'

# --- red: the tier's primitives stay private, and the owner does not call it ------------------
expect_failure 'fast_exp2 made public' \
    'sed -i "s/^fn fast_exp2/pub fn fast_exp2/" crates/math/src/fast_db.rs'

expect_failure 'fast_log2 made crate-public' \
    'sed -i "s/^fn fast_log2/pub(crate) fn fast_log2/" crates/math/src/fast_db.rs'

expect_failure 'the owning crate root calls the tier instead of only declaring it' \
    'printf "pub fn shim(x: f32) -> f32 { fast_db::fast_level_db(x) }\n" \
        >> crates/math/src/lib.rs'

expect_failure 'the sealed module stops exporting a conversion' \
    'sed -i "s/^pub fn fast_level_db/fn fast_level_db/" crates/math/src/fast_db.rs'

expect_failure 'the sealed module deleted' \
    'rm crates/math/src/fast_db.rs'

expect_failure 'an extra call spelled with a turbofish, to evade the count' \
    'printf "fn extra(x: f32) -> f32 { fast_level_db::<f32>(x) }\n" \
        >> crates/compressor/src/kernel.rs'

# --- red: the restatement registry is checked in both directions too -------------------------
expect_failure 'the registered restatement file deleted' \
    'rm crates/gate-expander/tests/contract.rs'

expect_failure 'the restatement marker deleted' \
    'sed -i "/FAST-DB-RESTATEMENT/d" crates/gate-expander/tests/contract.rs'

expect_failure 'an extra call added to the restatement site' \
    'printf "fn extra(x: f32) -> f32 { fast_gain_from_db(x) }\n" \
        >> crates/gate-expander/tests/contract.rs'

expect_failure 'a restatement site carrying a crossing marker as well' \
    'sed -i "s/FAST-DB-RESTATEMENT/FAST-DB-RESTATEMENT and FAST-DB-CROSSING X9/" \
        crates/gate-expander/tests/contract.rs'

# --- green: the structural exemptions still work ---------------------------------------------
expect_pass 'the bound gate may use the vocabulary freely' \
    'printf "#[test]\nfn more() { let _ = fast_level_db(2.0) + fast_gain_from_db(1.0) + fast_level_db(3.0); }\n" \
        >> crates/math/tests/f1_fast_db_bounds.rs'

expect_pass 'a crossing marker may quote the function name in prose' \
    'sed -i "s|// FAST-DB-CROSSING X1: the detector level.|// FAST-DB-CROSSING X1: fast_level_db(0.0) is exact here.|" \
        crates/compressor/src/kernel.rs'

expect_pass 'the sealed module may add another private helper' \
    'printf "fn fast_helper(x: f32) -> f32 { x }\n" >> crates/math/src/fast_db.rs'

printf 'fast dB seal mutation tests: ok\n'
