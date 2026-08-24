#!/usr/bin/env bash
set -euo pipefail
cd "${1:-.}"
bash scripts/check-effect-runtime-policy.sh .
bash scripts/test-effect-runtime-policy.sh .
bash scripts/check-effect-runtime-fixtures.sh .
bash scripts/test-effect-runtime-fixtures.sh .
cargo test --locked -p miso-engine-effect-contract -p miso-engine-effect-compiler -p miso-engine-conformance

# #105 phase 2 F1: the conformance harness runs against EVERY production `NativeEffectFactory`,
# not just against its own reference mock. This loop is what stops the ninth effect from shipping
# without one -- the completeness statement lives here rather than in a hand-maintained list, so a
# new effect crate is failing until it carries the test.
#
# Two directories are deliberately not products:
#   * `crates/miso-engine-conformance` owns the reference mock the harness validates itself with;
#   * `crates/miso-engine-graph-compiler`'s factories are `#[cfg(test)]` mocks inside its own unit
#     tests (bank-bind failure and scalar-only fallbacks), not effects anybody can instantiate.
conformance_crates=()
while IFS= read -r source; do
    crate_dir="${source%%/src/*}"
    case "$crate_dir" in
        crates/miso-engine-conformance | crates/miso-engine-graph-compiler) continue ;;
    esac
    conformance_crates+=("$crate_dir")
    if [[ ! -f "$crate_dir/tests/conformance.rs" ]]; then
        printf 'effect contract failure: missing %s/tests/conformance.rs\n' "$crate_dir" >&2
        printf 'every production NativeEffectFactory must run the shared harness (#105 F1)\n' >&2
        exit 1
    fi
    if ! grep -q 'effect_conformance_test!' "$crate_dir/tests/conformance.rs"; then
        printf 'effect contract failure: %s/tests/conformance.rs does not invoke effect_conformance_test!\n' \
            "$crate_dir" >&2
        exit 1
    fi
done < <(rg -l 'impl NativeEffectFactory for' crates --glob 'crates/*/src/**.rs' | LC_ALL=C sort)
[[ ${#conformance_crates[@]} -ge 8 ]] || {
    printf 'effect contract failure: found only %s production effect factories\n' \
        "${#conformance_crates[@]}" >&2
    exit 1
}

packages=()
for crate_dir in "${conformance_crates[@]}"; do
    packages+=(-p "${crate_dir#crates/}")
done
cargo test --locked "${packages[@]}" --test conformance
cargo run --locked --release -q -p miso-engine-effect-contract-bench -- --conformance
printf 'effect runtime contract/conformance: ok (%s production factories on the harness)\n' \
    "${#conformance_crates[@]}"
