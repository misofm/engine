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

missing_surface="$scratch_root/missing-surface"
create_fixture "$missing_surface"
rm "$missing_surface/crates/protocol/src/controller.rs"
if bash "$policy_script" "$missing_surface" >/dev/null 2>&1; then
    printf 'protocol control policy missing required surface unexpectedly passed\n' >&2; exit 1
fi
missing_trait="$scratch_root/missing-trait"
create_fixture "$missing_trait"
sed -i '/pub trait ControlProvider {/,/^}/d' "$missing_trait/crates/protocol/src/controller.rs"
trait_output="$(bash "$policy_script" "$missing_trait" 2>&1)" && trait_rc=0 || trait_rc=$?
[[ "$trait_rc" -ne 0 && "$trait_output" == *'missing or empty ControlProvider declaration'* ]] || {
    printf 'protocol missing trait was misclassified: %s\n' "$trait_output" >&2; exit 1;
}

# Optional message sources may be absent, and an empty public-field population is valid.
empty_fields="$scratch_root/empty-fields"
create_fixture "$empty_fields"
sed -i '/pub struct MockProvider {/,/^}/c\pub struct MockProvider {\n}' "$empty_fields/crates/protocol/src/controller.rs"
bash "$policy_script" "$empty_fields" >/dev/null

expect_predicate_error() {
    local name="$1" surface="$2" expected="$3" fixture shim
    fixture="$scratch_root/$name"
    shim="$scratch_root/$name-bin"
    create_fixture "$fixture"
    if [[ "$name" == mock-predicate ]]; then
        sed -i '/replay_bytes/a\    pub diagnostics: TypedValue,' "$fixture/crates/protocol/src/controller.rs"
    fi
    mkdir -p "$shim"
    cat >"$shim/rg" <<EOF
#!/usr/bin/env bash
input=\$(cat)
if [[ "\$input" == *'$surface'* ]]; then printf 'valid partial output\n' >&2; exit 7; fi
printf '%s\n' "\$input" | "$(command -v rg)" "\$@"
EOF
    chmod +x "$shim/rg"
    output="$(PATH="$shim:$PATH" bash "$policy_script" "$fixture" 2>&1)" && rc=0 || rc=$?
    [[ "$rc" -ne 0 && "$output" == *"$expected"* && "$output" == *'valid partial output'* ]] || {
        printf 'protocol predicate error misclassified: %s: %s\n' "$name" "$output" >&2; exit 1;
    }
}
expect_predicate_error provider-predicate 'pub trait ControlProvider' 'ControlProvider raw-byte predicate scan errored'
expect_predicate_error mock-predicate 'pub diagnostics' 'MockProvider raw-byte predicate scan errored'

awk_fixture="$scratch_root/awk-error"
create_fixture "$awk_fixture"
mkdir -p "$scratch_root/awk-bin"
cat >"$scratch_root/awk-bin/awk" <<'EOF'
#!/usr/bin/env bash
printf 'pub trait ControlProvider {\n' >&2
exit 6
EOF
chmod +x "$scratch_root/awk-bin/awk"
awk_output="$(PATH="$scratch_root/awk-bin:$PATH" bash "$policy_script" "$awk_fixture" 2>&1)" && awk_rc=0 || awk_rc=$?
[[ "$awk_rc" -ne 0 && "$awk_output" == *'ControlProvider scan errored'* ]] || {
    printf 'protocol awk error escaped: %s\n' "$awk_output" >&2; exit 1;
}

optional_fixture="$scratch_root/optional-read-error"
create_fixture "$optional_fixture"
printf 'pub struct Message { pub value: u32 }\n' >"$optional_fixture/crates/protocol/src/message_wire.rs"
mkdir -p "$scratch_root/optional-rg-bin"
real_rg="$(command -v rg)"
cat >"$scratch_root/optional-rg-bin/rg" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *'message_wire.rs'* ]]; then printf 'crates/protocol/src/controller.rs:1:allowed partial row\n'; exit 7; fi
exec "$real_rg" "\$@"
EOF
chmod +x "$scratch_root/optional-rg-bin/rg"
optional_output="$(PATH="$scratch_root/optional-rg-bin:$PATH" bash "$policy_script" "$optional_fixture" 2>&1)" && optional_rc=0 || optional_rc=$?
[[ "$optional_rc" -ne 0 && "$optional_output" == *'public arbitrary message payload storage is forbidden scan errored (rg exit 7)'* && "$optional_output" == *'allowed partial row'* ]] || {
    printf 'protocol optional source read error escaped: %s\n' "$optional_output" >&2; exit 1;
}

expect_failure provider-raw-bytes mutate_provider_raw_bytes
expect_failure mock-public-vector mutate_mock_public_vector
expect_failure public-payload mutate_public_payload

printf 'protocol control policy mutation tests: ok\n'
