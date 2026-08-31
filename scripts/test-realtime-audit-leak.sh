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
    (cd "$copy" && bash scripts/check-realtime-audit-leak.sh) >/dev/null 2>&1
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

# Baseline: the real tree passes.
run_gate || { echo "test-realtime-audit-leak: baseline gate is red" >&2; exit 1; }

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
if ! run_gate; then
    echo "test-realtime-audit-leak: gate is red on the unmutated tree (dev edges must stay legal)" >&2
    exit 1
fi

echo "test-realtime-audit-leak: OK"
