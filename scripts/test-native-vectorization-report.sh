#!/usr/bin/env bash
# Red mutations for the native vectorization report. Each mutation changes one allowlist claim and
# must be rejected by the release artifact, without modifying the artifact or production sources.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
binary="${1:-$repository_root/target/release/audit}"
allowlist="$repository_root/tools/audit/vectorization-allowlist.tsv"

if command -v llvm-objdump >/dev/null 2>&1; then
    objdump="$(command -v llvm-objdump)"
elif command -v objdump >/dev/null 2>&1; then
    objdump="$(command -v objdump)"
else
    printf 'native vectorization mutations require llvm-objdump or objdump\n' >&2
    exit 2
fi

if [[ ! -x "$binary" ]]; then
    (cd "$repository_root" && cargo build --locked --release \
        -p audit --bin audit)
fi

scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

# Optional fourth argument: the failure class the mutation must produce. A mutation that is red
# for the wrong reason (any `"status":"fail"` at all) would otherwise count as proof.
expect_red() {
    local name="$1"
    local mutated="$2"
    local disassembler="${3:-$objdump}"
    local expected_class="${4:-}"
    local output="$scratch_root/$name.json"
    if "$binary" vectorization --artifact "$binary" --allowlist "$mutated" \
        --objdump "$disassembler" >"$output" 2>&1; then
        printf 'native vectorization mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
    rg -q '"status":"fail"|invalid vectorization allowlist' "$output" || {
        printf 'native vectorization mutation produced no failure report: %s\n' "$name" >&2
        sed -n '1,20p' "$output" >&2
        exit 1
    }
    if [[ -n "$expected_class" ]]; then
        rg -q "$expected_class" "$output" || {
            printf 'native vectorization mutation failed with the wrong class: %s\n' "$name" >&2
            sed -n '1,20p' "$output" >&2
            exit 1
        }
    fi
}

missing_family="$scratch_root/missing-family.tsv"
sed 's/vmulps,%ymm/definitely_missing_vector_opcode,%ymm/' "$allowlist" >"$missing_family"
expect_red missing-vector-family "$missing_family"

scalar_objdump="$scratch_root/scalar-objdump"
printf '%s\n' \
    '#!/usr/bin/env bash' \
    'set -euo pipefail' \
    "real_objdump='$objdump'" \
    '"$real_objdump" "$@" | awk '\''/probe_gain_simd8>:/ { print; print "  0: vaddss %xmm0, %xmm1, %xmm2"; next } { print }'\''' \
    >"$scalar_objdump"
chmod +x "$scalar_objdump"
expect_red scalar-fallback "$allowlist" "$scalar_objdump"

fused_objdump="$scratch_root/fused-objdump"
printf '%s\n' \
    '#!/usr/bin/env bash' \
    'set -euo pipefail' \
    "real_objdump='$objdump'" \
    '"$real_objdump" "$@" | awk '\''/probe_svf_simd8>:/ { print; print "  0: vfmadd213ps %ymm0, %ymm1, %ymm2"; next } { print }'\''' \
    >"$fused_objdump"
chmod +x "$fused_objdump"
expect_red fused-multiply-add "$allowlist" "$fused_objdump" "forbidden scalar fallback"

call_objdump="$scratch_root/call-objdump"
printf '%s\n' \
    '#!/usr/bin/env bash' \
    'set -euo pipefail' \
    "real_objdump='$objdump'" \
    '"$real_objdump" "$@" | awk '\''/probe_svf_simd8>:/ { print; print "  0: call   0x1000"; next } { print }'\''' \
    >"$call_objdump"
chmod +x "$call_objdump"
expect_red call-inside-kernel "$allowlist" "$call_objdump" "forbidden call"

incomplete="$scratch_root/incomplete.tsv"
awk -F '\t' '$3 != "probe_sum2_simd8"' "$allowlist" >"$incomplete"
expect_red incomplete-registry "$incomplete"

printf 'native vectorization red mutations: ok\n'
