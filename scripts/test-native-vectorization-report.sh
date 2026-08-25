#!/usr/bin/env bash
# Red mutations for the native vectorization certification (issue #144 item 3).
#
# Every rule the report claims must have a mutation here that turns it red, and each mutation must
# be rejected for the reason it was designed to trigger -- a mutation that fails for a different
# reason proves nothing about the rule it names.
#
# The binary is REBUILT, never reused. This suite has twice in this repository's history been run
# against a stale `target/release/miso_engine_audit` that predated the subcommand under test; the
# suite then "passed" by refusing an argument it did not understand. The rebuild below is the fix,
# and the first assertion is that the freshly built binary really carries this subcommand.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "${1:-$script_directory/..}" && pwd)"
cd "$repository_root"

resolve_tool() {
    local preferred="$1" fallback="$2"
    if command -v "$preferred" >/dev/null 2>&1; then command -v "$preferred"
    elif command -v "$fallback" >/dev/null 2>&1; then command -v "$fallback"
    else
        printf 'native vectorization mutations require %s or %s\n' "$preferred" "$fallback" >&2
        exit 2
    fi
}
objdump="$(resolve_tool llvm-objdump objdump)"
nm="$(resolve_tool llvm-nm nm)"

cargo build --locked --release -p miso-engine-audit --bin miso_engine_audit
binary="$repository_root/target/release/miso_engine_audit"
[[ -x "$binary" ]] || {
    printf 'native vectorization mutations: %s was not built\n' "$binary" >&2
    exit 1
}
# A binary that does not know the subcommand must not be able to make this suite pass.
subject_status=0
"$binary" vectorization --registry-directory /nonexistent-registry-directory >/dev/null 2>&1 \
    || subject_status=$?
if [[ "$subject_status" -ne 2 ]]; then
    printf 'native vectorization mutations: the built binary does not carry the vectorization \
subject (exit %s, expected the argument-error exit 2)\n' "$subject_status" >&2
    exit 1
fi

evidence="$repository_root/target/ci/native-vectorization"
bash "$script_directory/run-native-vectorization-report.sh" "$repository_root" "$evidence" \
    >/dev/null
baseline="$evidence/report.json"
grep -q '"status":"pass"' "$baseline" || {
    printf 'native vectorization mutations need a green baseline; it is red:\n' >&2
    sed -n '1,4p' "$baseline" >&2
    exit 1
}

probes_host="$evidence/probes-host/release/deps"
host_ir="$(find "$probes_host" -maxdepth 1 -name 'miso_engine_vectorization_probes-*.ll' | head -n 1)"
host_object="$(find "$probes_host" -maxdepth 1 -name 'miso_engine_vectorization_probes-*.o' | head -n 1)"
capi="$evidence/products/release/libmiso_engine_capi.so"
web="$evidence/products/release/libmiso_engine_host_web.so"

scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

# Runs the subject over a mutated registry directory and requires a red report naming `expected`.
expect_red() {
    local name="$1" registry="$2" expected="$3"
    shift 3
    local output="$scratch/$name.json"
    if "$binary" vectorization \
        --registry-directory "$registry" \
        --kernel-root "$repository_root/crates/miso-engine-lane/src" \
        --nm "$nm" --objdump "$objdump" \
        --backend "x86_64-avx2=$host_ir,$host_object" \
        --skip-backend "aarch64-neon=not exercised by this mutation" \
        --product "capi-cdylib=$capi" --product "web-native-twin=$web" \
        "$@" >"$output" 2>&1; then
        printf 'native vectorization mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
    grep -q -- "$expected" "$output" || {
        printf 'native vectorization mutation %s did not fail for its reason (%s):\n' \
            "$name" "$expected" >&2
        head -c 1200 "$output" >&2
        printf '\n' >&2
        exit 1
    }
    printf '  red: %s\n' "$name"
}

registry_copy() {
    local name="$1"
    local directory="$scratch/$name"
    mkdir -p "$directory"
    cp "$repository_root/tools/miso-engine-audit"/vectorization-*.tsv "$directory/"
    printf '%s\n' "$directory"
}

# 1. Family completeness: a kernel family the lane crate exposes but the registry does not name.
mutation="$(registry_copy drop-family)"
grep -v $'\tsvf_block\t' "$mutation/vectorization-families.tsv" >"$mutation/tmp"
mv "$mutation/tmp" "$mutation/vectorization-families.tsv"
expect_red drop-family "$mutation" 'kernels::svf_block is public in the lane crate and is not registered'

# 2. Family completeness, the other direction: a registered kernel the lane crate does not expose.
mutation="$(registry_copy phantom-family)"
printf 'kernels\tsvf_block_quadratic\tphantom\tcertified\t-\n' \
    >>"$mutation/vectorization-families.tsv"
printf 'x86_64-avx2\tphantom\tmiso_engine_vectorization_probes::probe_svf_block\twide::f32x8_::f32x8\tvector-arith\tvector-arith\n' \
    >>"$mutation/vectorization-allowlist.tsv"
printf 'aarch64-neon\tphantom\tmiso_engine_vectorization_probes::probe_svf_block\twide::f32x4_::f32x4\tvector-arith\tvector-arith\n' \
    >>"$mutation/vectorization-allowlist.tsv"
expect_red phantom-family "$mutation" 'which the lane crate no longer exposes'

# 3. Backend coverage: a certified family with no rule at a backend.
mutation="$(registry_copy uncovered-backend)"
grep -v $'^aarch64-neon\trecursive-svf\t' "$mutation/vectorization-allowlist.tsv" >"$mutation/tmp"
mv "$mutation/tmp" "$mutation/vectorization-allowlist.tsv"
expect_red uncovered-backend "$mutation" "certified family 'recursive-svf' has no rule"

# 4. Probe identity: a renamed probe no longer resolves to a symbol, and the family goes unproven.
mutation="$(registry_copy renamed-probe)"
sed -i 's/probe_svf_block\t/probe_svf_block_renamed\t/' "$mutation/vectorization-allowlist.tsv"
expect_red renamed-probe "$mutation" 'expected exactly one defined symbol'

# 5. Structural class: a family that really does vector arithmetic declared free of floating point.
mutation="$(registry_copy wrong-class)"
sed -i $'s/^x86_64-avx2\trecursive-svf\t\\(.*\\)\tvector-arith\tvector-arith$/x86_64-avx2\trecursive-svf\t\\1\tno-float\tno-float/' \
    "$mutation/vectorization-allowlist.tsv"
expect_red wrong-class "$mutation" 'in a family declared free of them'

# 6. Backend completeness: a backend that is neither certified nor explicitly skipped is not a pass.
output="$scratch/missing-backend.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$objdump" \
    --backend "x86_64-avx2=$host_ir,$host_object" \
    --product "capi-cdylib=$capi" --product "web-native-twin=$web" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: missing-backend\n' >&2
    exit 1
fi
grep -q "was neither certified nor explicitly skipped" "$output" || {
    printf 'native vectorization mutation missing-backend failed for the wrong reason\n' >&2
    head -c 800 "$output" >&2
    exit 1
}
printf '  red: missing-backend\n'

# 7. Shipped binding: a product the registry knows about but the run never certified.
output="$scratch/missing-product.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$objdump" \
    --backend "x86_64-avx2=$host_ir,$host_object" \
    --skip-backend "aarch64-neon=not exercised by this mutation" \
    --product "capi-cdylib=$capi" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: missing-product\n' >&2
    exit 1
fi
grep -q "which this run did not certify" "$output" || {
    printf 'native vectorization mutation missing-product failed for the wrong reason\n' >&2
    head -c 800 "$output" >&2
    exit 1
}
printf '  red: missing-product\n'

# 8. Shipped binding: a scalar-loop bank. The floor is raised beyond what the shipped symbol does,
#    which is the shape "a bank that stopped vectorizing" presents as.
mutation="$(registry_copy scalar-bank)"
awk -F'\t' -v OFS='\t' '$3 == "kernel-host" && $1 == "capi-cdylib" { $5 = 100000 } { print }' \
    "$mutation/vectorization-shipped.tsv" >"$mutation/tmp"
mv "$mutation/tmp" "$mutation/vectorization-shipped.tsv"
expect_red scalar-bank "$mutation" 'below the registered floor'

# 9. Shipped binding: a render entry that is not exported.
mutation="$(registry_copy hidden-entry)"
sed -i 's/miso_engine_v2_render_f32_planar/miso_engine_v2_render_f32_planar_absent/' \
    "$mutation/vectorization-shipped.tsv"
expect_red hidden-entry "$mutation" 'expected exactly one definition'

# 10. Adversarial disassembly: a second symbol header carrying the certified name. The subject must
#     refuse the ambiguity rather than read whichever body came first.
duplicate_objdump="$scratch/duplicate-objdump"
{
    printf '#!/usr/bin/env bash\n'
    printf 'set -euo pipefail\n'
    printf "real='%s'\n" "$objdump"
    cat <<'AWK'
"$real" "$@" | awk '
  /probe_svf_block::<wide::f32x8_::f32x8>>:$/ && !seen {
      seen = 1
      print
      print ""
      print "0000000000000000 <miso_engine_vectorization_probes::probe_svf_block::<wide::f32x8_::f32x8>>:"
      next
  }
  { print }
'
AWK
} >"$duplicate_objdump"
chmod +x "$duplicate_objdump"
output="$scratch/duplicate-header.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$duplicate_objdump" \
    --backend "x86_64-avx2=$host_ir,$host_object" \
    --skip-backend "aarch64-neon=not exercised by this mutation" \
    --product "capi-cdylib=$capi" --product "web-native-twin=$web" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: duplicate-header\n' >&2
    exit 1
fi
grep -q "disassembled bodies are named" "$output" || {
    printf 'native vectorization mutation duplicate-header failed for the wrong reason\n' >&2
    head -c 900 "$output" >&2
    exit 1
}
printf '  red: duplicate-header\n'

# 11. Adversarial disassembly: a scalar fused multiply-add injected into a certified body. An
#     opcode-prefix scan reads `vfmadd213ss` as the packed `vfmadd213ps` it shares six characters
#     with; the mnemonic-suffix classification must call it scalar.
scalar_objdump="$scratch/scalar-objdump"
{
    printf '#!/usr/bin/env bash\n'
    printf 'set -euo pipefail\n'
    printf "real='%s'\n" "$objdump"
    cat <<'AWK'
"$real" "$@" | awk '
  /probe_gain_block::<wide::f32x8_::f32x8>>:$/ { print; print "       4:      \tvfmadd213ss\t%xmm0, %xmm1, %xmm2"; next }
  { print }
'
AWK
} >"$scalar_objdump"
chmod +x "$scalar_objdump"
output="$scratch/scalar-fma.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$scalar_objdump" \
    --backend "x86_64-avx2=$host_ir,$host_object" \
    --skip-backend "aarch64-neon=not exercised by this mutation" \
    --product "capi-cdylib=$capi" --product "web-native-twin=$web" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: scalar-fma\n' >&2
    exit 1
fi
grep -q "scalar floating-point arithmetic instructions" "$output" || {
    printf 'native vectorization mutation scalar-fma failed for the wrong reason\n' >&2
    head -c 900 "$output" >&2
    exit 1
}
printf '  red: scalar-fma\n'

# 12. LLVM IR: fast-math flags and a forbidden contraction intrinsic injected into a certified body.
mutated_ir="$scratch/fast-math.ll"
awk '
  /^define / && /15probe_svf_block/ { print; injected = 1; next }
  injected { print "  %inject = fmul reassoc <8 x float> zeroinitializer, zeroinitializer"; injected = 0 }
  { print }
' "$host_ir" >"$mutated_ir"
output="$scratch/fast-math.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$objdump" \
    --backend "x86_64-avx2=$mutated_ir,$host_object" \
    --skip-backend "aarch64-neon=not exercised by this mutation" \
    --product "capi-cdylib=$capi" --product "web-native-twin=$web" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: fast-math\n' >&2
    exit 1
fi
grep -q "fast-math flags" "$output" || {
    printf 'native vectorization mutation fast-math failed for the wrong reason\n' >&2
    head -c 900 "$output" >&2
    exit 1
}
printf '  red: fast-math\n'

# 13. LLVM IR: a math-library call injected into a certified body.
libm_ir="$scratch/libm.ll"
awk '
  /^define / && /15probe_svf_block/ { print; injected = 1; next }
  injected { print "  %inject = call float @expf(float 0.0)"; injected = 0 }
  { print }
' "$host_ir" >"$libm_ir"
output="$scratch/libm.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$objdump" \
    --backend "x86_64-avx2=$libm_ir,$host_object" \
    --skip-backend "aarch64-neon=not exercised by this mutation" \
    --product "capi-cdylib=$capi" --product "web-native-twin=$web" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: libm-call\n' >&2
    exit 1
fi
grep -q "forbidden intrinsic or math-library symbols" "$output" || {
    printf 'native vectorization mutation libm-call failed for the wrong reason\n' >&2
    head -c 900 "$output" >&2
    exit 1
}
printf '  red: libm-call\n'

# 14. LLVM IR: a narrow vector operation, the "half the lanes" regression an eight-lane backend
#     would otherwise report as vectorized.
narrow_ir="$scratch/narrow.ll"
awk '
  /^define / && /15probe_svf_block/ { print; injected = 1; next }
  injected { print "  %inject = fmul <4 x float> zeroinitializer, zeroinitializer"; injected = 0 }
  { print }
' "$host_ir" >"$narrow_ir"
output="$scratch/narrow.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$objdump" \
    --backend "x86_64-avx2=$narrow_ir,$host_object" \
    --skip-backend "aarch64-neon=not exercised by this mutation" \
    --product "capi-cdylib=$capi" --product "web-native-twin=$web" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: narrow-vector\n' >&2
    exit 1
fi
grep -q "narrower vector type" "$output" || {
    printf 'native vectorization mutation narrow-vector failed for the wrong reason\n' >&2
    head -c 900 "$output" >&2
    exit 1
}
printf '  red: narrow-vector\n'

# 15. The AArch64 skip must be explicit. An unknown backend name in a skip is not a silent pass.
output="$scratch/unknown-skip.json"
if "$binary" vectorization \
    --registry-directory "$repository_root/tools/miso-engine-audit" \
    --kernel-root "$repository_root/crates/miso-engine-lane/src" \
    --nm "$nm" --objdump "$objdump" \
    --backend "x86_64-avx2=$host_ir,$host_object" \
    --skip-backend "aarch64=typo" \
    --product "capi-cdylib=$capi" --product "web-native-twin=$web" >"$output" 2>&1; then
    printf 'native vectorization mutation unexpectedly passed: unknown-skip\n' >&2
    exit 1
fi
grep -q "unknown skipped backend" "$output" || {
    printf 'native vectorization mutation unknown-skip failed for the wrong reason\n' >&2
    head -c 800 "$output" >&2
    exit 1
}
printf '  red: unknown-skip\n'

printf 'native vectorization red mutations: ok (15 mutations)\n'
