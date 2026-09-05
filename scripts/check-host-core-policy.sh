#!/usr/bin/env bash
# Enforces audit #103 F1: the host-preparation pipeline has exactly one definition.
#
# What this catches, and why each rule is here:
#   1. A host re-implementing the compile pipeline. Before #103 the C ABI host and the browser
#      host each carried a ~300-line copy of it; the copies had already diverged (only one
#      rejected source generation 0, only one capped source channels).
#   2. A host re-inventing the do-nothing `GraphRuntimeProcessor` that `GraphNodeBinding::identity`
#      replaced. Three copies existed.
#   3. A host re-implementing the control protocol's wire format by hand. capi carried a private
#      header parser and replay cache until #102 made the protocol's own public.
#   4. The facade making the host-specific control protocol mandatory/default, or any consumer
#      except capi enabling its optional adapter. A default edge would push protocol into the
#      browser artifact. The facade also exports no `no_mangle` symbol: a `cdylib` re-exports every
#      one it links and would push the C ABI's exports into the browser artifact's frozen set.
set -euo pipefail

fail() {
    printf 'host-core policy failure: %s\n' "$1" >&2
    exit 1
}

# `rg` exits 0 on a match, 1 when the pattern is clean, and 2 (or higher) on a search error --
# most commonly a search root that does not exist. The old `! rg ...` idiom reads both 1 and 2 as
# "no violation" (S9): a host source directory that silently stops existing (a rename, a fixture
# missing a mkdir) would read as a clean pass instead of the scan never having run. This mirrors
# scripts/check-workspace-policy.sh's scan_forbidden.
scan_forbidden() {
    local description="$1" pattern="$2"
    shift 2
    local paths=("$@")
    local output rc
    if output="$(rg -n "$pattern" "${paths[@]}" 2>&1)"; then
        rc=0
    else
        rc=$?
    fi
    case "$rc" in
        0)
            printf '%s\n' "$output" >&2
            fail "$description"
            ;;
        1)
            ;;
        *)
            printf '%s\n' "$output" >&2
            fail "$description scan errored (rg exit $rc)"
            ;;
    esac
}

root="${1:-.}"
facade_manifest="$root/crates/host-core/Cargo.toml"
facade_source="$root/crates/host-core/src"
capi_manifest="$root/crates/capi/Cargo.toml"
# Every host under hosts/*/src is scanned; there is no exemption list. host-web was the last
# holdout (tracked as "pending #106") and already depends on host-core
# (hosts/host-web/Cargo.toml), so the exemption is dead and removing it only tightens the gate.

host_sources=("$root/crates/capi/src")
[[ -d "$root/crates/capi/src" ]] || fail "expected host source directory is missing: $root/crates/capi/src"

[[ -d "$root/hosts" ]] || fail "expected hosts/ directory is missing: $root/hosts"
host_count=0
for host in "$root"/hosts/*/; do
    [[ -d "$host" ]] || continue
    host_count=$((host_count + 1))
    host="${host%/}"
    # S9: a missing `src` under a host directory that does exist is a failure, not a silent skip
    # -- the whole point of this gate is that every host is scanned, so a host quietly losing its
    # source directory (and thus its coverage) must not read as "nothing to scan here".
    [[ -d "$host/src" ]] || fail "expected host source directory is missing: $host/src"
    host_sources+=("$host/src")
done
[[ "$host_count" -gt 0 ]] || fail "no host directories found under $root/hosts"

pipeline_entry_points='(^|[^_[:alnum:]])compile_session\(|prepare_native_session_effects\(|prepare_session_builtins\(|compile_with_builtins\(|prepare_graph_source_set\(|into_bound_with_source_set\(|PcmSourceRing::prepare_host_region\('
scan_forbidden 'a host calls the compile pipeline directly; use host-core' \
    "$pipeline_entry_points" "${host_sources[@]}"

scan_forbidden 'a host defines a pass-through processor; use GraphNodeBinding::identity' \
    'impl +GraphRuntimeProcessor|struct +Identity(Processor|Binding)' "${host_sources[@]}"

scan_forbidden 'a host hand-decodes the control wire format; use protocol' \
    'MISOCTL|ReplayEntryRecord|fn +protocol_u(16|32|64)|fn +correlatable_command_header' \
    "${host_sources[@]}"

[[ -f "$facade_manifest" ]] || fail "expected host-core manifest is missing: $facade_manifest"
[[ -f "$capi_manifest" ]] || fail "expected capi manifest is missing: $capi_manifest"
[[ "$(grep -Fxc 'default = []' "$facade_manifest")" == 1 ]] ||
    fail 'host-core must have an empty default feature set'
[[ "$(grep -Fxc 'control-provider = ["dep:protocol"]' "$facade_manifest")" == 1 ]] ||
    fail 'host-core must gate protocol behind exactly control-provider'
[[ "$(grep -Fxc 'protocol = { workspace = true, optional = true }' "$facade_manifest")" == 1 ]] ||
    fail 'host-core protocol dependency must be workspace-scoped and optional'
[[ "$(grep -Fxc 'host-core = { workspace = true, features = ["control-provider"] }' "$capi_manifest")" == 1 ]] ||
    fail 'capi must explicitly enable the host-core control-provider adapter'

# The feature name may appear in exactly the host-core declaration and capi's dependency. This
# closes the browser/default-host leak while leaving comments and unrelated feature names free.
feature_occurrences=$(rg -n -g Cargo.toml 'control-provider' "$root" | wc -l)
[[ "$feature_occurrences" == 2 ]] ||
    fail 'only host-core may declare and capi may enable control-provider'
protocol_dependencies=$(rg -n '^[[:space:]]*protocol[[:space:]]*=' "$facade_manifest" | wc -l)
[[ "$protocol_dependencies" == 1 ]] ||
    fail 'host-core must contain exactly one protocol dependency edge'

[[ -d "$facade_source" ]] || fail "expected host-core source directory is missing: $facade_source"
scan_forbidden 'the host facade must export no C symbols; it links into every host cdylib' \
    '^[^/]*no_mangle' "$facade_source"

grep -Fqx 'crate-type = ["rlib"]' "$facade_manifest" || {
    printf 'host-core policy failure: the host facade must be an rlib only\n' >&2
    exit 1
}

printf 'host-core policy: ok\n'
