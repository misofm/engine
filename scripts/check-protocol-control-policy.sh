#!/usr/bin/env bash
# Reject raw-byte escape hatches in the public issue-005 provider/controller surface. Caller-owned
# codec buffers and private replay response storage are intentional and are excluded below.
set -euo pipefail
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"

workspace_root="${1:-.}"
cd "$workspace_root"
GATE_FAILURE_PREFIX='protocol control policy failure'

source_file="crates/protocol/src/controller.rs"

fail() {
    printf 'protocol control policy failure: %s\n' "$1" >&2
    exit 1
}

[[ -f "$source_file" ]] || fail "missing $source_file"

if control_provider="$({
    awk '
        /^pub trait ControlProvider[[:space:]]*\{/ { in_trait = 1 }
        in_trait { print }
        in_trait && /^\}/ { exit }
    ' "$source_file"
})"; then :; else gate_fail "ControlProvider scan errored"; exit 1; fi
[[ -n "$control_provider" ]] || { gate_fail 'missing or empty ControlProvider declaration'; exit 1; }

if provider_bytes="$(gate_scan_collect 'ControlProvider raw-byte predicate' \
    '(Vec[[:space:]]*<[[:space:]]*u8|&[[:space:]]*(mut[[:space:]]*)?\[[[:space:]]*u8[[:space:]]*\]|\b(Bytes|ByteBuf)\b)' '' /dev/stdin <<<"$control_provider")"; then :; else exit 1; fi
if [[ -n "$provider_bytes" ]]; then
    fail "public ControlProvider methods must use typed values, never raw bytes"
fi

if mock_provider_fields="$({
    awk '
        /^pub struct MockProvider[[:space:]]*\{/ { in_struct = 1; next }
        in_struct && /^\}/ { exit }
        in_struct && /^[[:space:]]*pub[[:space:]]/ { print }
    ' "$source_file"
})"; then :; else gate_fail 'MockProvider scan errored'; exit 1; fi

if mock_bytes="$(gate_scan_collect 'MockProvider raw-byte predicate' \
    '(Vec[[:space:]]*<|&[[:space:]]*(mut[[:space:]]*)?\[|\[[[:space:]]*u8|\b(Bytes|ByteBuf)\b|bytes)' '' /dev/stdin <<<"$mock_provider_fields")"; then :; else exit 1; fi
if [[ -n "$mock_bytes" ]]; then
    fail "MockProvider public fields must not expose vectors or raw byte storage"
fi

# Public typed messages may not expose an arbitrary raw `payload` field. This deliberately permits
# `ControllerRequest::canonical_bytes`, the private `ControllerResponse::bytes` replay cache, and
# the deliberately separate `wire.rs` caller-buffer decoder view, all of which have explicit
# ownership/validation contracts.
message_sources=("$source_file")
for optional_source in \
    crates/protocol/src/message_wire.rs \
    crates/protocol/src/session_wire.rs; do
    [[ -f "$optional_source" ]] && message_sources+=("$optional_source")
done
if ! gate_scan_forbidden \
    'public arbitrary message payload storage is forbidden' \
    '^[[:space:]]*pub[[:space:]]+[A-Za-z_]*payload[A-Za-z0-9_]*[[:space:]]*:[[:space:]]*(Vec|&|\[|Bytes|ByteBuf)' \
    '' "${message_sources[@]}"; then
    exit 1
fi

printf 'protocol control policy: ok\n'
