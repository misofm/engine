#!/usr/bin/env bash
# Mutation tests proving check-realtime-audit-leak.sh discriminates (#84 phase D).
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

# A resolution gate needs a resolvable workspace: copy the working tree (sans build products).
copy="$scratch_root/repo"
mkdir -p "$copy"
tar --exclude='./target' --exclude='./.git' --exclude='./fuzz/target' -C "$root" -cf - . \
    | tar -xf - -C "$copy"

run_gate() {
    (cd "$copy" && bash scripts/check-realtime-audit-leak.sh)
}

restore() {
    local path="$1"
    cp "$root/$path" "$copy/$path"
}

expect_failure() {
    local label="$1"
    local mutation="$2"
    bash -c "$mutation"
    if run_gate; then
        echo "test-realtime-audit-leak: gate stayed green after mutation: $label" >&2
        exit 1
    fi
    echo "test-realtime-audit-leak: red as required: $label"
}

expect_failure_with_path() {
    local label="$1" shim="$2" expected=${3:-}
    local output status
    if output="$(PATH="$shim:$PATH" run_gate 2>&1)"; then status=0; else status=$?; fi
    if ((status == 0)); then
        echo "test-realtime-audit-leak: unexpected success: $label" >&2
        exit 1
    fi
    if [[ -n "$expected" && "$output" != *"$expected"* ]]; then
        printf 'test-realtime-audit-leak: wrong diagnostic: %s\n%s\n' "$label" "$output" >&2
        exit 1
    fi
    echo "test-realtime-audit-leak: red as required: $label"
}

new_fixture() {
    copy="$scratch_root/$1"
    mkdir -p "$copy/scripts" "$copy/crates/fixture/src" "$copy/hosts" "$copy/sidecars"
    cp "$root/scripts/check-realtime-audit-leak.sh" "$copy/scripts/"
    printf '[workspace]\nmembers = ["crates/fixture"]\nresolver = "2"\n' >"$copy/Cargo.toml"
    printf '# frozen offline fixture\nversion = 3\n\n[[package]]\nname = "fixture"\nversion = "0.1.0"\n' >"$copy/Cargo.lock"
    printf '[package]\nname = "fixture"\nversion = "0.1.0"\nedition = "2021"\n' >"$copy/crates/fixture/Cargo.toml"
    printf '' >"$copy/crates/fixture/src/lib.rs"
}

# Baseline: the real tree passes.
run_gate >/dev/null 2>&1 || { echo "test-realtime-audit-leak: baseline gate is red" >&2; exit 1; }

# 1. Conformance silently hard-enables the instrumentation for every dependent again.
expect_failure conformance-regular-enable \
    "sed -i 's|^engine.workspace = true$|engine = { workspace = true, features = [\"realtime-audit\"] }|' \
        \"$copy/crates/conformance/Cargo.toml\""
restore crates/conformance/Cargo.toml

# 2. The C ABI artifact itself asks for the instrumentation.
expect_failure capi-direct-enable \
    "sed -i 's|^engine.workspace = true$|engine = { workspace = true, features = [\"realtime-audit\"] }|' \
        \"$copy/crates/capi/Cargo.toml\""
restore crates/capi/Cargo.toml

# 3. Guard against an over-broad gate: dev-edge enables are the supported test path and must pass.
if ! run_gate >/dev/null 2>&1; then
    echo "test-realtime-audit-leak: gate is red on the unmutated tree (dev edges must stay legal)" >&2
    exit 1
fi

# Required roots and the aggregate discovery population are explicit production inputs.
for required_root in crates hosts sidecars; do
    new_fixture "missing-$required_root"
    rm -rf "$copy/$required_root"
    run_gate >/dev/null 2>&1 && {
        echo "test-realtime-audit-leak: missing root escaped: $required_root" >&2
        exit 1
    }
done

new_fixture empty-population
rm -rf "$copy/crates/fixture"
if run_gate >/dev/null 2>&1; then
    echo 'test-realtime-audit-leak: empty manifest population escaped' >&2
    exit 1
fi
run_targeted_error() {
    local label=$1 shim=$2 expected=$3
    local output status
    if output="$(PATH="$shim:$PATH" run_gate 2>&1)"; then status=0; else status=$?; fi
    ((status != 0)) || return 97
    [[ "$output" == *"$expected"* ]] || { printf 'target %s produced wrong diagnostic: %s\n' "$label" "$output" >&2; return 96; }
    printf 'test-realtime-audit-leak: targeted original status %s: %s\n' "$status" "$label"
}

# The two controls below retain valid-looking output while returning a failing producer status.
# They must fail the original discovery/Cargo assertions, rather than being accepted as empty.
new_fixture status-loss-find
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "crates/capi/Cargo.toml\\n"\nexit 1\n' >"$scratch_root/status-loss-find/shim/find"
chmod +x "$scratch_root/status-loss-find/shim/find"
printf '#!/usr/bin/env bash\nprintf "crates/fixture/Cargo.toml\\n"\nprintf "find-partial-sentinel\\n" >&2\nexit 7\n' >"$copy/shim/find"
chmod +x "$copy/shim/find"
run_targeted_error manifest-discovery "$copy/shim" 'manifest discovery failed with status 7; output: crates/fixture/Cargo.toml; stderr: find-partial-sentinel' || exit $?

new_fixture manifest-sort-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "sort-partial-sentinel\\n"\nprintf "sort-error-sentinel\\n" >&2\nexit 6\n' >"$copy/shim/sort"
chmod +x "$copy/shim/sort"
run_targeted_error manifest-sort "$copy/shim" 'manifest sort failed with status 6; input: crates/fixture/Cargo.toml; stderr: sort-error-sentinel' || exit $?

new_fixture structural-parser-output-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *" -v file="* ]]; then printf "structural-partial-sentinel\\n"; printf "structural-error-sentinel\\n" >&2; exit 5; fi\nexec /usr/bin/awk "$@"\n' >"$copy/shim/awk"
chmod +x "$copy/shim/awk"
run_targeted_error structural-parser "$copy/shim" 'manifest parser failed for crates/fixture/Cargo.toml with status 5; output: structural-partial-sentinel; stderr: structural-error-sentinel' || exit $?

new_fixture package-parser-output-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ "$1" == "-F\\\"" ]]; then printf "fixture\\n"; printf "package-error-sentinel\\n" >&2; exit 4; fi\nexec /usr/bin/awk "$@"\n' >"$copy/shim/awk"
chmod +x "$copy/shim/awk"
run_targeted_error package-parser "$copy/shim" 'package-name parser failed for crates/fixture/Cargo.toml with status 4; output: fixture; stderr: package-error-sentinel' || exit $?

new_fixture unnamed-package
sed -i '/^name = /d' "$copy/crates/fixture/Cargo.toml"
run_targeted_error unnamed-package /nonexistent 'unnamed package manifest: crates/fixture/Cargo.toml' || exit $?

new_fixture status-loss-cargo
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "fixture v0.1.0 (resolved)\\n"\nprintf "cargo-partial-sentinel\\n" >&2\nexit 8\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
run_targeted_error cargo-tree "$copy/shim" 'cargo tree failed for fixture with status 8; output: fixture v0.1.0 (resolved); stderr: cargo-partial-sentinel' || exit $?

new_fixture status-loss-cargo-matching
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "fixture v0.1.0 (resolved) realtime-audit\\n"\nprintf "cargo-matching-error-sentinel\\n" >&2\nexit 8\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
run_targeted_error cargo-tree-matching "$copy/shim" 'cargo tree failed for fixture with status 8; output: fixture v0.1.0 (resolved) realtime-audit; stderr: cargo-matching-error-sentinel' || exit $?

# A successful Cargo producer followed by a failing graph grep remains an execution error.
new_fixture status-loss-grep
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nfor arg in "$@"; do [[ "$arg" == *realtime-audit* ]] && exit 2; done\nexec /usr/bin/grep "$@"\n' >"$scratch_root/status-loss-grep/shim/grep"
chmod +x "$scratch_root/status-loss-grep/shim/grep"
printf '#!/usr/bin/env bash\nfor arg in "$@"; do if [[ "$arg" == *realtime-audit* ]]; then printf "realtime-audit grep-partial-sentinel\\n"; printf "grep-error-sentinel\\n" >&2; exit 9; fi; done\nexec /usr/bin/grep "$@"\n' >"$copy/shim/grep"
chmod +x "$copy/shim/grep"
run_targeted_error cargo-graph-grep "$copy/shim" 'cargo graph scan failed for fixture with status 9; output: realtime-audit grep-partial-sentinel; stderr: grep-error-sentinel; graph: fixture v0.1.0' || exit $?

# The same targeted assertions must reject actual production status-loss mutations.
new_fixture manifest-status-mutant
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "crates/fixture/Cargo.toml\\n"\nprintf "find-partial-sentinel\\n" >&2\nexit 7\n' >"$copy/shim/find"
chmod +x "$copy/shim/find"
[[ "$(grep -Fc '((find_status == 0)) || fail "manifest discovery failed' "$copy/scripts/check-realtime-audit-leak.sh")" == 1 ]] || exit 96
sed -i '/((find_status == 0)) || fail "manifest discovery failed/c\: # MUTANT: suppress manifest discovery status' "$copy/scripts/check-realtime-audit-leak.sh"
if run_targeted_error manifest-discovery "$copy/shim" 'manifest discovery failed with status 7; output: crates/fixture/Cargo.toml; stderr: find-partial-sentinel'; then
    echo 'test-realtime-audit-leak: manifest status mutant was not detected' >&2; exit 1
else status=$?; ((status == 97)) || exit "$status"; fi
printf 'test-realtime-audit-leak: manifest status mutant distinguished status %s\n' "$status"

new_fixture cargo-status-mutant
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "fixture v0.1.0 (resolved)\\n"\nprintf "cargo-partial-sentinel\\n" >&2\nexit 8\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
[[ "$(grep -Fc '((cargo_status == 0)) || fail "cargo tree failed' "$copy/scripts/check-realtime-audit-leak.sh")" == 1 ]] || exit 96
sed -i '/((cargo_status == 0)) || fail "cargo tree failed/c\        : # MUTANT: suppress Cargo status' "$copy/scripts/check-realtime-audit-leak.sh"
if run_targeted_error cargo-tree "$copy/shim" 'cargo tree failed for fixture with status 8; output: fixture v0.1.0 (resolved); stderr: cargo-partial-sentinel'; then
    echo 'test-realtime-audit-leak: Cargo status mutant was not detected' >&2; exit 1
else status=$?; ((status == 97)) || exit "$status"; fi
printf 'test-realtime-audit-leak: Cargo status mutant distinguished status %s\n' "$status"

new_fixture restored-positive
run_gate >/dev/null 2>&1 || { echo 'test-realtime-audit-leak: restored fixture is red' >&2; exit 1; }
echo 'test-realtime-audit-leak: restored positive status 0'

echo "test-realtime-audit-leak: OK"
