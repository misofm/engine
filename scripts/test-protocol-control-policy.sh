#!/usr/bin/env bash
# Mutation tests proving raw-byte provider and message-payload escape hatches are rejected.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-protocol-control-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_fixture() {
    local root="$1"
    mkdir -p "$root/crates/protocol/src"
    printf '%s\n' \
        'pub trait ControlProvider {' \
        '    fn typed(&mut self, value: TypedValue) -> Result<TypedValue, ProviderError>;' \
        '}' \
        '' \
        'pub struct MockProvider {' \
        '    replay_bytes: Vec<u8>,' \
        '}' \
        '' \
        'pub struct ControllerResponse {' \
        '    bytes: Vec<u8>,' \
        '}' \
        '' \
        'pub fn caller_buffer(output: &mut [u8]) {}' \
        >"$root/crates/protocol/src/controller.rs"
}

expect_failure() {
    local name="$1"
    local root="$scratch_root/$name"
    shift
    create_fixture "$root"
    "$@" "$root"
    if bash "$policy_script" "$root" >/dev/null 2>&1; then
        printf 'protocol control policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
}

mutate_provider_raw_bytes() {
    local root="$1"
    sed -i '/fn typed/a\    fn raw(&mut self, payload: &[u8]);' \
        "$root/crates/protocol/src/controller.rs"
}

mutate_mock_public_vector() {
    local root="$1"
    sed -i '/replay_bytes/a\    pub diagnostics: Vec<TypedValue>,' \
        "$root/crates/protocol/src/controller.rs"
}

mutate_public_payload() {
    local root="$1"
    printf '%s\n' \
        'pub struct ArbitraryMessage {' \
        '    pub payload: Vec<u8>,' \
        '}' \
        >>"$root/crates/protocol/src/controller.rs"
}

valid_root="$scratch_root/valid"
create_fixture "$valid_root"
bash "$policy_script" "$valid_root" >/dev/null

expect_failure provider-raw-bytes mutate_provider_raw_bytes
expect_failure mock-public-vector mutate_mock_public_vector
expect_failure public-payload mutate_public_payload

printf 'protocol control policy mutation tests: ok\n'
