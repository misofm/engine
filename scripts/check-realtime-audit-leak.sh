#!/usr/bin/env bash
# #84 phase D: core's `realtime-audit` feature compiles render-thread instrumentation (a
# thread-local depth guard consulted by the counting allocator) into `miso-engine-core`. It exists
# for the audit tools and for test builds only. This gate proves the feature cannot reach a
# shippable artifact: the production dependency graph (dev edges excluded, every target) of every
# crates/ and hosts/ package must resolve without it. tools/ packages are exempt — enabling the
# instrumentation is their job, and they are never linked into an artifact.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

fail() {
    echo "check-realtime-audit-leak: $1" >&2
    exit 1
}

# Structural half: inside crates/ and hosts/ manifests, only dev-dependency sections (and the
# forwarding `[features]` declaration in miso-engine-conformance itself) may mention the feature.
while IFS= read -r manifest; do
    violation="$(awk -v file="$manifest" '
        /^\[/ { section = $0 }
        /realtime-audit/ {
            dev = section ~ /dev-dependencies/
            forwarding = file ~ /miso-engine-conformance\/Cargo.toml/ && section == "[features]"
            declaration = file ~ /miso-engine-core\/Cargo.toml/ && section == "[features]"
            if (!dev && !forwarding && !declaration) { print file ": " $0; exit }
        }
    ' "$manifest")"
    [[ -z "$violation" ]] || fail "non-dev feature enable: $violation"
done < <(find crates hosts -mindepth 2 -maxdepth 2 -name Cargo.toml | sort)

# Resolution half: the compiler's own answer. `-e features,no-dev` is the graph a shipped build
# of the package resolves; `--target all` keeps target-gated edges visible.
while IFS= read -r manifest; do
    package="$(awk -F'"' '/^name = /{print $2; exit}' "$manifest")"
    [[ -n "$package" ]] || fail "unnamed package manifest: $manifest"
    if cargo tree --locked --offline -p "$package" -e features,no-dev --target all 2>/dev/null \
        | grep -q 'realtime-audit'; then
        fail "production graph of $package resolves core's realtime-audit feature"
    fi
done < <(find crates hosts -mindepth 2 -maxdepth 2 -name Cargo.toml | sort)

echo "check-realtime-audit-leak: OK"
