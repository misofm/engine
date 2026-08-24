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
#   4. The facade depending on the control protocol, which is host-specific transport, or exporting
#      a `no_mangle` symbol -- a `cdylib` re-exports every one it links, so a facade that carried
#      them would push the C ABI's exports into the browser artifact's frozen export set.
set -euo pipefail

root="${1:-.}"
facade_manifest="$root/crates/miso-engine-host-core/Cargo.toml"
facade_source="$root/crates/miso-engine-host-core/src"
# Hosts still carrying their own copy of the pipeline. Each entry names the issue that removes it;
# the entry is deleted in the same commit that converts the host, and nothing may be added here.
pending_conversion=("hosts/miso-engine-host-web") # issue #106

host_sources=("$root/crates/miso-engine-capi/src")
for host in "$root"/hosts/*/src; do
    [ -d "$host" ] || continue
    relative="${host#"$root"/}"
    exempt=""
    for pending in "${pending_conversion[@]}"; do
        [ "${relative%/src}" = "$pending" ] && exempt="yes"
    done
    [ -n "$exempt" ] || host_sources+=("$host")
done

pipeline_entry_points='(^|[^_[:alnum:]])compile_session\(|prepare_native_session_effects\(|prepare_session_builtins\(|compile_with_builtins\(|prepare_graph_source_set\(|into_bound_with_source_set\(|PcmSourceRing::prepare_host_region\('
! rg -n "$pipeline_entry_points" "${host_sources[@]}" || {
    printf 'a host calls the compile pipeline directly; use miso-engine-host-core\n' >&2
    exit 1
}

! rg -n 'impl +GraphRuntimeProcessor|struct +Identity(Processor|Binding)' "${host_sources[@]}" || {
    printf 'a host defines a pass-through processor; use GraphNodeBinding::identity\n' >&2
    exit 1
}

! rg -n 'MISOCTL|ReplayEntryRecord|fn +protocol_u(16|32|64)|fn +correlatable_command_header' "${host_sources[@]}" || {
    printf 'a host hand-decodes the control wire format; use miso-engine-protocol\n' >&2
    exit 1
}

! rg -n 'miso-engine-protocol' "$facade_manifest" || {
    printf 'the host facade must not depend on the control protocol\n' >&2
    exit 1
}

! rg -n '^[^/]*no_mangle' "$facade_source" || {
    printf 'the host facade must export no C symbols; it links into every host cdylib\n' >&2
    exit 1
}

grep -Fqx 'crate-type = ["rlib"]' "$facade_manifest" || {
    printf 'the host facade must be an rlib only\n' >&2
    exit 1
}

printf 'host-core policy: ok\n'
