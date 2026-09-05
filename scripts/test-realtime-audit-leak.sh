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

expect_failure_with_path() {
    local label="$1" shim="$2"
    if PATH="$shim:$PATH" run_gate; then
        echo "test-realtime-audit-leak: unexpected success: $label" >&2
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

# Required roots and the aggregate discovery population are explicit production inputs.
for required_root in crates hosts sidecars; do
    cp -R "$copy" "$scratch_root/missing-$required_root"
    rm -rf "$scratch_root/missing-$required_root/$required_root"
    (cd "$scratch_root/missing-$required_root" && bash scripts/check-realtime-audit-leak.sh) >/dev/null 2>&1 && {
        echo "test-realtime-audit-leak: missing root escaped: $required_root" >&2
        exit 1
    }
done

find "$copy/crates" "$copy/hosts" "$copy/sidecars" -mindepth 2 -maxdepth 2 -name Cargo.toml -delete
if run_gate; then
    echo 'test-realtime-audit-leak: empty manifest population escaped' >&2
    exit 1
fi

# The two controls below retain valid-looking output while returning a failing producer status.
# They must fail the original discovery/Cargo assertions, rather than being accepted as empty.
cp -R "$root" "$scratch_root/status-loss-find"
mkdir -p "$scratch_root/status-loss-find/shim"
printf '#!/usr/bin/env bash\nprintf "crates/capi/Cargo.toml\\n"\nexit 1\n' >"$scratch_root/status-loss-find/shim/find"
chmod +x "$scratch_root/status-loss-find/shim/find"
(cd "$scratch_root/status-loss-find" && PATH="$scratch_root/status-loss-find/shim:$PATH" bash scripts/check-realtime-audit-leak.sh) >/dev/null 2>&1 && {
    echo 'test-realtime-audit-leak: failed manifest discovery escaped' >&2
    exit 1
}

cp -R "$root" "$scratch_root/status-loss-cargo"
mkdir -p "$scratch_root/status-loss-cargo/shim"
printf '#!/usr/bin/env bash\nprintf "fixture v0.1.0 (resolved)\\n"\nexit 1\n' >"$scratch_root/status-loss-cargo/shim/cargo"
chmod +x "$scratch_root/status-loss-cargo/shim/cargo"
(cd "$scratch_root/status-loss-cargo" && PATH="$scratch_root/status-loss-cargo/shim:$PATH" bash scripts/check-realtime-audit-leak.sh) >/dev/null 2>&1 && {
    echo 'test-realtime-audit-leak: clean-looking failed Cargo graph escaped' >&2
    exit 1
}

# A successful Cargo producer followed by a failing graph grep remains an execution error.
cp -R "$root" "$scratch_root/status-loss-grep"
mkdir -p "$scratch_root/status-loss-grep/shim"
printf '#!/usr/bin/env bash\nfor arg in "$@"; do [[ "$arg" == *realtime-audit* ]] && exit 2; done\nexec /usr/bin/grep "$@"\n' >"$scratch_root/status-loss-grep/shim/grep"
chmod +x "$scratch_root/status-loss-grep/shim/grep"
expect_failure_with_path cargo-graph-grep-error "$scratch_root/status-loss-grep/shim"

echo "test-realtime-audit-leak: OK"
