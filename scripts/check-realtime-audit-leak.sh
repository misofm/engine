#!/usr/bin/env bash
# #84 phase D: core's `realtime-audit` feature compiles render-thread instrumentation (a
# thread-local depth guard consulted by the counting allocator) into `engine`. It exists
# for the audit tools and for test builds only. This gate proves the feature cannot reach a
# shippable artifact: the production dependency graph (dev edges excluded, every target) of every
# crates/, hosts/ and sidecars/ package must resolve without it. tools/ packages are exempt --
# enabling the instrumentation is their job, and they are never linked into an artifact. sidecars/
# is scanned the same as crates/ and hosts/: a sidecar ships, so its production graph must resolve
# without the feature too.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

fail() {
    echo "check-realtime-audit-leak: $1" >&2
    exit 1
}

captured() { local path=$1; [[ -s "$path" ]] && printf '%s' "$(<"$path")" || printf '<empty>'; }

for required_root in crates hosts sidecars; do
    [[ -d "$required_root" ]] || fail "missing required root: $required_root"
done

if find crates hosts sidecars -mindepth 2 -maxdepth 2 -name Cargo.toml >"$scratch/manifests" 2>"$scratch/find.err"; then
    find_status=0
else
    find_status=$?
fi
((find_status == 0)) || fail "manifest discovery failed with status $find_status; output: $(captured "$scratch/manifests"); stderr: $(captured "$scratch/find.err")"
if LC_ALL=C sort "$scratch/manifests" >"$scratch/manifests.sorted" 2>"$scratch/sort.err"; then sort_status=0; else sort_status=$?; fi
((sort_status == 0)) || fail "manifest sort failed with status $sort_status; input: $(captured "$scratch/manifests"); stderr: $(captured "$scratch/sort.err")"
[[ -s "$scratch/manifests.sorted" ]] || fail 'manifest discovery produced no packages'

# Structural half: inside crates/ and hosts/ manifests, only dev-dependency sections (and the
# forwarding `[features]` declaration in conformance itself) may mention the feature.
while IFS= read -r manifest; do
    if awk -v file="$manifest" '
        /^\[/ { section = $0 }
        /realtime-audit/ {
            dev = section ~ /dev-dependencies/
            forwarding = file ~ /conformance\/Cargo.toml/ && section == "[features]"
            declaration = file ~ /engine\/Cargo.toml/ && section == "[features]"
            if (!dev && !forwarding && !declaration) { print file ": " $0; exit }
        }
    ' "$manifest" >"$scratch/awk" 2>"$scratch/awk.err"; then
        awk_status=0
    else
        awk_status=$?
    fi
    ((awk_status == 0)) || fail "manifest parser failed for $manifest with status $awk_status; output: $(captured "$scratch/awk"); stderr: $(captured "$scratch/awk.err")"
    violation="$(<"$scratch/awk")"
    [[ -z "$violation" ]] || fail "non-dev feature enable: $violation"
done < "$scratch/manifests.sorted"

# Resolution half: the compiler's own answer. `-e features,no-dev` is the graph a shipped build
# of the package resolves; `--target all` keeps target-gated edges visible.
while IFS= read -r manifest; do
    if awk -F'"' '/^name = /{print $2; exit}' "$manifest" >"$scratch/package" 2>"$scratch/awk.err"; then
        awk_status=0
    else
        awk_status=$?
    fi
    ((awk_status == 0)) || fail "package-name parser failed for $manifest with status $awk_status; output: $(captured "$scratch/package"); stderr: $(captured "$scratch/awk.err")"
    package="$(<"$scratch/package")"
    [[ -n "$package" ]] || fail "unnamed package manifest: $manifest"
    if cargo tree --locked --offline -p "$package" -e features,no-dev --target all >"$scratch/cargo" 2>"$scratch/cargo.err"; then
        cargo_status=0
    else
        cargo_status=$?
    fi
    ((cargo_status == 0)) || fail "cargo tree failed for $package with status $cargo_status; output: $(captured "$scratch/cargo"); stderr: $(captured "$scratch/cargo.err")"
    [[ -s "$scratch/cargo" ]] || fail "cargo tree produced no graph for $package"
    if grep -n 'realtime-audit' "$scratch/cargo" >"$scratch/grep" 2>"$scratch/grep.err"; then
        fail "production graph of $package resolves core's realtime-audit feature: $(captured "$scratch/grep")"
    else
        grep_status=$?
        ((grep_status == 1)) || fail "cargo graph scan failed for $package with status $grep_status; output: $(captured "$scratch/grep"); stderr: $(captured "$scratch/grep.err"); graph: $(captured "$scratch/cargo")"
    fi
done < "$scratch/manifests.sorted"

echo "check-realtime-audit-leak: OK"
