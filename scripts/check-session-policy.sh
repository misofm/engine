#!/usr/bin/env bash
# Enforces issue-004 dependency direction, compiler boundary, and parser feature policy.
set -euo pipefail

session_manifest="crates/miso-engine-session/Cargo.toml"
core_manifest="crates/miso-engine-core/Cargo.toml"
session_source="crates/miso-engine-session/src"

! grep -Fq 'miso-engine-session' "$core_manifest" || {
    printf 'core must not depend on session\n' >&2
    exit 1
}
grep -Fqx 'miso-engine-core.workspace = true' "$session_manifest"
grep -Fqx 'serde = { version = "=1.0.228", features = ["derive"] }' "$session_manifest"
grep -Fqx 'toml = { version = "=0.9.9", default-features = false, features = ["parse", "serde"] }' "$session_manifest"
! grep -Fq 'spec-1.0.0' "$session_manifest" || {
    printf 'spec-1.0.0 is package metadata, not a Cargo feature\n' >&2
    exit 1
}
! rg -n 'use miso_engine_core::.*(PreparedRenderPlan|PlanPublisher)|PlanPublisher<' "$session_source" || {
    printf 'session may not import plan publication APIs\n' >&2
    exit 1
}
! rg -n 'format!|\.to_owned\(|\.to_string\(|String::with_capacity|Vec::with_capacity|\.collect::' "$session_source/estimate.rs" || {
    printf 'successful resource preflight may not allocate temporary diagnostics or collections\n' >&2
    exit 1
}

compile_source="$session_source/compile.rs"
estimate_line="$(rg -n 'let estimate = estimate_session\(session\)' "$compile_source" | head -1 | cut -d: -f1)"
caps_line="$(rg -n 'check_caps\(session, estimate, caps\)' "$compile_source" | head -1 | cut -d: -f1)"
validate_line="$(rg -n 'validate_session\(session\)' "$compile_source" | head -1 | cut -d: -f1)"
canonical_line="$(rg -n 'let canonical_toml = write_canonical\(session\)' "$compile_source" | head -1 | cut -d: -f1)"
clone_line="$(rg -n 'let mut normalized = session\.clone\(\)' "$compile_source" | head -1 | cut -d: -f1)"
(( estimate_line < caps_line && caps_line < validate_line && validate_line < canonical_line && canonical_line < clone_line )) || {
    printf 'resource preflight/cap ordering changed\n' >&2
    exit 1
}
