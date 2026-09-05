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
    mkdir -p "$copy/scripts" "$copy/crates/fixture/src" "$copy/hosts/later/src" "$copy/sidecars"
    cp "$root/scripts/check-realtime-audit-leak.sh" "$copy/scripts/"
    printf '[workspace]\nmembers = ["crates/fixture", "hosts/later"]\nresolver = "2"\n' >"$copy/Cargo.toml"
    printf '# frozen offline fixture\nversion = 3\n\n[[package]]\nname = "fixture"\nversion = "0.1.0"\n\n[[package]]\nname = "later-fixture"\nversion = "0.1.0"\n' >"$copy/Cargo.lock"
    printf '[package]\nname = "fixture"\nversion = "0.1.0"\nedition = "2021"\n' >"$copy/crates/fixture/Cargo.toml"
    printf '' >"$copy/crates/fixture/src/lib.rs"
    printf '[package]\nname = "later-fixture"\nversion = "0.1.0"\nedition = "2021"\n' >"$copy/hosts/later/Cargo.toml"
    printf '' >"$copy/hosts/later/src/lib.rs"
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
    output="$(run_gate 2>&1)" && status=0 || status=$?
    ((status != 0)) || {
        echo "test-realtime-audit-leak: missing root escaped: $required_root" >&2
        exit 1
    }
    [[ "$output" == *"missing required root: $required_root"* ]] || { printf 'test-realtime-audit-leak: wrong missing-root diagnostic: %s\n' "$output" >&2; exit 96; }
done

new_fixture empty-population
rm -rf "$copy/crates/fixture" "$copy/hosts/later"
output="$(run_gate 2>&1)" && status=0 || status=$?
if ((status == 0)); then
    echo 'test-realtime-audit-leak: empty manifest population escaped' >&2
    exit 1
fi
[[ "$output" == *'manifest discovery produced no packages'* ]] || { printf 'test-realtime-audit-leak: wrong empty-population diagnostic: %s\n' "$output" >&2; exit 96; }
run_targeted_error() {
    local label=$1 shim=$2 expected=$3 witness=${4:-}
    local output status
    if output="$(PATH="$shim:$PATH" run_gate 2>&1)"; then status=0; else status=$?; fi
    ((status != 0)) || return 97
    [[ "$output" == *"$expected"* ]] || { printf 'target %s produced wrong diagnostic: %s\n' "$label" "$output" >&2; return 96; }
    [[ -z "$witness" || "$output" == *"$witness"* ]] || { printf 'target %s omitted witness %s: %s\n' "$label" "$witness" "$output" >&2; return 96; }
    printf 'test-realtime-audit-leak: targeted original status %s: %s\n' "$status" "$label"
}

# The two controls below retain valid-looking output while returning a failing producer status.
# They must fail the original discovery/Cargo assertions, rather than being accepted as empty.
new_fixture status-loss-find
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\n/usr/bin/find "$@"\nprintf "find-partial-sentinel\\n" >&2\nexit 7\n' >"$copy/shim/find"
chmod +x "$copy/shim/find"
run_targeted_error manifest-discovery "$copy/shim" 'hosts/later/Cargo.toml; stderr: find-partial-sentinel' || exit $?

new_fixture manifest-discovery-empty-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "find-empty-error-sentinel\\n" >&2\nexit 7\n' >"$copy/shim/find"
chmod +x "$copy/shim/find"
run_targeted_error manifest-discovery-empty "$copy/shim" 'manifest discovery failed with status 7; output: <empty>; stderr: find-empty-error-sentinel' || exit $?

new_fixture manifest-sort-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\n/usr/bin/sort "$@"\nprintf "sort-error-sentinel\\n" >&2\nexit 6\n' >"$copy/shim/sort"
chmod +x "$copy/shim/sort"
run_targeted_error manifest-sort "$copy/shim" 'manifest sort failed with status 6; output: crates/fixture/Cargo.toml
hosts/later/Cargo.toml; input:' 'sort-error-sentinel' || exit $?

new_fixture manifest-sort-empty-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nprintf "sort-empty-error-sentinel\\n" >&2\nexit 6\n' >"$copy/shim/sort"
chmod +x "$copy/shim/sort"
run_targeted_error manifest-sort-empty "$copy/shim" 'manifest sort failed with status 6; output: <empty>; input:' 'sort-empty-error-sentinel' || exit $?

new_fixture structural-parser-output-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *"file=hosts/later/Cargo.toml"* ]]; then printf "structural-error-sentinel\\n" >&2; exit 5; fi\nexec /usr/bin/awk "$@"\n' >"$copy/shim/awk"
chmod +x "$copy/shim/awk"
run_targeted_error structural-parser "$copy/shim" 'manifest parser failed for hosts/later/Cargo.toml with status 5; output: <empty>; stderr: structural-error-sentinel' || exit $?

new_fixture package-parser-output-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ "$1" == "-F\\\"" && "${@: -1}" == "hosts/later/Cargo.toml" ]]; then printf "later-fixture\\n"; printf "package-error-sentinel\\n" >&2; exit 4; fi\nexec /usr/bin/awk "$@"\n' >"$copy/shim/awk"
chmod +x "$copy/shim/awk"
run_targeted_error package-parser "$copy/shim" 'package-name parser failed for hosts/later/Cargo.toml with status 4; output: later-fixture; stderr: package-error-sentinel' || exit $?

new_fixture package-parser-empty-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ "$1" == "-F\\\"" && "${@: -1}" == "hosts/later/Cargo.toml" ]]; then printf "package-empty-error-sentinel\\n" >&2; exit 4; fi\nexec /usr/bin/awk "$@"\n' >"$copy/shim/awk"
chmod +x "$copy/shim/awk"
run_targeted_error package-parser-empty "$copy/shim" 'package-name parser failed for hosts/later/Cargo.toml with status 4; output: <empty>; stderr: package-empty-error-sentinel' || exit $?

new_fixture unnamed-package
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ "$1" == "-F\\\"" && "${@: -1}" == "hosts/later/Cargo.toml" ]]; then exit 0; fi\nexec /usr/bin/awk "$@"\n' >"$copy/shim/awk"
chmod +x "$copy/shim/awk"
run_targeted_error unnamed-package "$copy/shim" 'unnamed package manifest: hosts/later/Cargo.toml' || exit $?

new_fixture status-loss-cargo
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *" -p later-fixture "* ]]; then printf "later-fixture v0.1.0 (resolved)\\n"; printf "cargo-partial-sentinel\\n" >&2; exit 8; fi\nexec /home/bl/.cargo/bin/cargo "$@"\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
run_targeted_error cargo-tree "$copy/shim" 'cargo tree failed for later-fixture with status 8; output: later-fixture v0.1.0 (resolved); stderr: cargo-partial-sentinel' || exit $?

new_fixture status-loss-cargo-empty
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *" -p later-fixture "* ]]; then printf "cargo-empty-error-sentinel\\n" >&2; exit 8; fi\nexec /home/bl/.cargo/bin/cargo "$@"\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
run_targeted_error cargo-tree-empty "$copy/shim" 'cargo tree failed for later-fixture with status 8; output: <empty>; stderr: cargo-empty-error-sentinel' || exit $?

new_fixture status-loss-cargo-matching
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *" -p later-fixture "* ]]; then printf "later-fixture v0.1.0 (resolved) realtime-audit\\n"; printf "cargo-matching-error-sentinel\\n" >&2; exit 8; fi\nexec /home/bl/.cargo/bin/cargo "$@"\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
run_targeted_error cargo-tree-matching "$copy/shim" 'cargo tree failed for later-fixture with status 8; output: later-fixture v0.1.0 (resolved) realtime-audit; stderr: cargo-matching-error-sentinel' || exit $?

new_fixture cargo-empty-success
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *" -p later-fixture "* ]]; then exit 0; fi\nexec /home/bl/.cargo/bin/cargo "$@"\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
run_targeted_error cargo-empty-success "$copy/shim" 'cargo tree produced no graph for later-fixture' || exit $?

# A successful Cargo producer followed by a failing graph grep remains an execution error.
new_fixture status-loss-grep
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nif [[ " $* " == *" -p later-fixture "* ]]; then printf "later-fixture v0.1.0 (resolved) realtime-audit\\n"; exit 0; fi\nexec /home/bl/.cargo/bin/cargo "$@"\n' >"$copy/shim/cargo"
chmod +x "$copy/shim/cargo"
printf '#!/usr/bin/env bash\nfor arg in "$@"; do if [[ "$arg" == *realtime-audit* ]] && /usr/bin/grep -q later-fixture "${@: -1}"; then /usr/bin/grep "$@"; printf "grep-error-sentinel\\n" >&2; exit 9; fi; done\nexec /usr/bin/grep "$@"\n' >"$copy/shim/grep"
chmod +x "$copy/shim/grep"
run_targeted_error cargo-graph-grep "$copy/shim" 'cargo graph scan failed for later-fixture with status 9; output: 1:later-fixture v0.1.0 (resolved) realtime-audit; stderr: grep-error-sentinel; graph: later-fixture v0.1.0' || exit $?

new_fixture cargo-graph-grep-empty-error
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\nfor arg in "$@"; do if [[ "$arg" == *realtime-audit* ]] && /usr/bin/grep -q later-fixture "${@: -1}"; then /usr/bin/grep "$@" || true; printf "grep-empty-error-sentinel\\n" >&2; exit 9; fi; done\nexec /usr/bin/grep "$@"\n' >"$copy/shim/grep"
chmod +x "$copy/shim/grep"
run_targeted_error cargo-graph-grep-empty "$copy/shim" 'cargo graph scan failed for later-fixture with status 9; output: <empty>; stderr: grep-empty-error-sentinel; graph: later-fixture v0.1.0' || exit $?

# The same targeted assertions must reject actual production status-loss mutations.
new_fixture manifest-status-mutant
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\n/usr/bin/find "$@"\nprintf "find-partial-sentinel\\n" >&2\nexit 7\n' >"$copy/shim/find"
chmod +x "$copy/shim/find"
[[ "$(grep -Fc '((find_status == 0)) || fail "manifest discovery failed' "$copy/scripts/check-realtime-audit-leak.sh")" == 1 ]] || exit 96
sed -i '/((find_status == 0)) || fail "manifest discovery failed/c\: # MUTANT: suppress manifest discovery status' "$copy/scripts/check-realtime-audit-leak.sh"
if run_targeted_error manifest-discovery "$copy/shim" 'hosts/later/Cargo.toml; stderr: find-partial-sentinel'; then
    echo 'test-realtime-audit-leak: manifest status mutant was not detected' >&2; exit 1
else status=$?; ((status == 97)) || exit "$status"; fi
printf 'test-realtime-audit-leak: manifest status mutant distinguished status %s\n' "$status"

new_fixture cargo-status-mutant
mkdir -p "$copy/shim"
printf '#!/usr/bin/env bash\npackage=\nprevious=\nfor arg in "$@"; do [[ "$previous" == -p ]] && package=$arg; previous=$arg; done\nprintf "%%s v0.1.0 (resolved)\\n" "$package"\nprintf "cargo-partial-sentinel\\n" >&2\nexit 8\n' >"$copy/shim/cargo"
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
