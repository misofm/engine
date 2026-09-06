#!/usr/bin/env bash
# Zero-workload preparation for the sole future Issue-431 current full-chain capture.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repository_root=$(cd "$script_directory/.." && pwd)
artifact_directory="$repository_root/artifacts/issue431-full-chain"
prepared_directory="$repository_root/target/issue431-prepared"
build_directory="$prepared_directory/build"
binary="$prepared_directory/bench"
seal="$artifact_directory/builtins-benchmark.preflight.json"
manifest_evidence="$artifact_directory/builtins-benchmark.manifest.json"
readme="$artifact_directory/README.md"
runner="$script_directory/run-builtins-current-benchmark.sh"
record_validator="$script_directory/builtins-current-benchmark-record-validator.jq"
aggregate_validator="$script_directory/builtins-current-benchmark-validator.jq"
lifecycle="$script_directory/test-builtins-current-benchmark.sh"
source_file="$repository_root/tools/bench/src/builtins.rs"
lock_file="$repository_root/Cargo.lock"
workspace_manifest="$repository_root/Cargo.toml"
config_file="$repository_root/.cargo/config.toml"
preconditions="$script_directory/check-bench-preconditions.sh"
fixture_manifest="$repository_root/fixtures/builtins/v1/MANIFEST.tsv"

fail() { printf 'Issue-431 current benchmark preflight failure: %s\n' "$1" >&2; exit 1; }
hash_file() {
    local output status
    if output=$(sha256sum "$1"); then status=0; else status=$?; fi
    ((status == 0)) || fail "sha256sum failed for $1 (status $status)"
    printf '%s' "${output%% *}"
}
one_link_file() {
    [[ -f "$1" && ! -L "$1" && "$(stat -c %h "$1")" == 1 ]]
}
for tool in awk cargo chmod cp git jq mkdir mktemp mv rm rustc sha256sum stat; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool unavailable: $tool"
done
for override in RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS CARGO_BUILD_TARGET \
    CARGO_PROFILE_RELEASE_OPT_LEVEL CARGO_PROFILE_RELEASE_LTO \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS \
    RUSTC RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER; do
    [[ ! -v "$override" ]] || fail "incompatible build environment: $override"
done
if cargo_version=$(cargo -V); then :; else fail 'cargo version query failed'; fi
if rustc_verbose=$(rustc -vV); then :; else fail 'rustc provenance query failed'; fi
rust_version=$(printf '%s\n' "$rustc_verbose" | awk 'NR==1 {print; found=1} END {if (!found) exit 1}') ||
    fail 'rustc version is unavailable'
target_triple=$(printf '%s\n' "$rustc_verbose" | awk '$1=="host:" {print $2; found=1} END {if (!found) exit 1}') ||
    fail 'rustc host target is unavailable'
llvm_version=$(printf '%s\n' "$rustc_verbose" | awk '$1=="LLVM" && $2=="version:" {print $3; found=1} END {if (!found) exit 1}') ||
    fail 'LLVM version is unavailable'
cargo_executable=$(command -v cargo)
rustc_executable=$(command -v rustc)
[[ -x "$cargo_executable" && -x "$rustc_executable" ]] || fail 'build tool executable unavailable'
for directory in "$repository_root/artifacts" "$repository_root/target"; do
    [[ ! -L "$directory" ]] || fail "parent directory is a symlink: $directory"
done
[[ ! -e "$artifact_directory" && ! -L "$artifact_directory" ]] ||
    fail "refusing existing prepared namespace: $artifact_directory"
[[ ! -e "$prepared_directory" && ! -L "$prepared_directory" ]] ||
    fail "refusing existing prepared namespace: $prepared_directory"
mkdir -p "$artifact_directory" "$prepared_directory"
[[ -d "$artifact_directory" && ! -L "$artifact_directory" &&
   -d "$prepared_directory" && ! -L "$prepared_directory" ]] ||
    fail 'prepared namespace is not a physical directory'
for path in "$binary" "$seal" "$manifest_evidence" "$readme"     "$artifact_directory/builtins-benchmark.raw.jsonl"     "$artifact_directory/builtins-benchmark.jsonl"     "$artifact_directory/builtins-benchmark.stderr"     "$artifact_directory/builtins-benchmark.disposition.json"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "refusing existing reserved path: $path"
done
for path in "$runner" "$record_validator" "$aggregate_validator" "$lifecycle"     "$source_file" "$lock_file" "$workspace_manifest" "$config_file" "$fixture_manifest" "$preconditions"; do
    one_link_file "$path" || fail "required one-link regular file unavailable: $path"
done

if candidate_commit=$(git -C "$repository_root" rev-parse --verify HEAD); then :; else
    fail 'candidate commit query failed'
fi
if candidate_tree=$(git -C "$repository_root" rev-parse 'HEAD^{tree}'); then :; else
    fail 'candidate tree query failed'
fi
if clean_output=$(git -C "$repository_root" status --porcelain=v1 --untracked-files=normal); then :; else
    fail 'candidate status query failed'
fi
[[ -z "$clean_output" ]] || fail 'candidate is not clean'

input_list="$prepared_directory/input-paths"
cat >"$input_list" <<'INPUTS'
fixtures/builtins/v1/MANIFEST.tsv
fixtures/builtins/v1/benchmark/full_chain_filters-48000.toml
fixtures/builtins/v1/benchmark/full_chain_filters-96000.toml
fixtures/builtins/v1/benchmark/identity_chain-48000.toml
fixtures/builtins/v1/benchmark/identity_chain-96000.toml
fixtures/builtins/v1/benchmark/matrix_ramp-48000.toml
fixtures/builtins/v1/benchmark/matrix_ramp-96000.toml
fixtures/builtins/v1/benchmark/meter_success_full-48000.toml
fixtures/builtins/v1/benchmark/meter_success_full-96000.toml
fixtures/builtins/v1/benchmark/prepare_256_tracks-48000.toml
fixtures/builtins/v1/benchmark/prepare_256_tracks-96000.toml
fixtures/builtins/v1/pcm/filters-asymmetric.f32le
fixtures/builtins/v1/pcm/identity-signed-zero.f32le
fixtures/builtins/v1/pcm/matrix-ramp-128.f32le
fixtures/builtins/v1/pcm/graph-taps.f32le
fixtures/session/v1/canonical.json
INPUTS
input_rows="$prepared_directory/input-identities"
: >"$input_rows"
while IFS= read -r relative; do
    path="$repository_root/$relative"
    one_link_file "$path" || fail "current input unavailable: $relative"
    printf '%s  %s\n' "$(hash_file "$path")" "$relative" >>"$input_rows"
done <"$input_list"
input_tree_sha256=$(hash_file "$input_rows")
[[ "$(hash_file "$fixture_manifest")" == b244da45d88d670951205098b7516af20387a141eccb3bf60edb61e8ba57a919 ]] ||
    fail 'current fixture manifest identity mismatch'

rm -rf -- "$build_directory"
if (cd "$repository_root" && \
    CARGO_TARGET_DIR="$build_directory" \
    CARGO_PROFILE_RELEASE_OPT_LEVEL=3 CARGO_PROFILE_RELEASE_LTO=fat \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS='-Ctarget-feature=+avx2,+fma' \
    cargo build --locked --release -p bench); then :; else
    fail 'isolated release build failed'
fi
built_binary="$build_directory/release/bench"
[[ -x "$built_binary" && -f "$built_binary" && ! -L "$built_binary" ]] ||
    fail 'isolated release binary unavailable'
temporary_binary=$(mktemp "$prepared_directory/.bench.XXXXXX")
cp -- "$built_binary" "$temporary_binary"
chmod 0755 "$temporary_binary"
mv -n -- "$temporary_binary" "$binary"
[[ ! -e "$temporary_binary" && -x "$binary" && ! -L "$binary" &&
   "$(stat -c %h "$binary")" == 1 ]] || fail 'prepared binary publication failed'

if final_commit=$(git -C "$repository_root" rev-parse --verify HEAD); then :; else fail 'final commit query failed'; fi
if final_tree=$(git -C "$repository_root" rev-parse 'HEAD^{tree}'); then :; else fail 'final tree query failed'; fi
if final_clean=$(git -C "$repository_root" status --porcelain=v1 --untracked-files=normal); then :; else fail 'final status query failed'; fi
[[ "$final_commit" == "$candidate_commit" && "$final_tree" == "$candidate_tree" && -z "$final_clean" ]] ||
    fail 'candidate drifted during preparation'

binary_sha256=$(hash_file "$binary")
cargo_executable_sha256=$(hash_file "$cargo_executable")
rustc_executable_sha256=$(hash_file "$rustc_executable")
lock_sha256=$(hash_file "$lock_file")
workspace_manifest_sha256=$(hash_file "$workspace_manifest")
source_sha256=$(hash_file "$source_file")
config_sha256=$(hash_file "$config_file")
preconditions_sha256=$(hash_file "$preconditions")
runner_sha256=$(hash_file "$runner")
preflight_sha256=$(hash_file "$script_directory/preflight-builtins-current-benchmark.sh")
record_sha256=$(hash_file "$record_validator")
aggregate_sha256=$(hash_file "$aggregate_validator")
lifecycle_sha256=$(hash_file "$lifecycle")
temporary_manifest=$(mktemp "$artifact_directory/.manifest.XXXXXX")
jq -n -S --arg input_tree "$input_tree_sha256" --arg manifest "$(hash_file "$fixture_manifest")" \
    '{schema_version:1,issue:431,kind:"builtins_current_input_manifest",
      input_tree_sha256:$input_tree,fixture_manifest_sha256:$manifest,inputs:16}' >"$temporary_manifest"
mv -n -- "$temporary_manifest" "$manifest_evidence"
temporary_readme=$(mktemp "$artifact_directory/.README.XXXXXX")
printf '%s\n' '# Issue 431 current full-chain capture' '' \
    'Prepared without launching the benchmark. The sole capture remains separately authorized.' >"$temporary_readme"
mv -n -- "$temporary_readme" "$readme"
manifest_evidence_sha256=$(hash_file "$manifest_evidence")
readme_sha256=$(hash_file "$readme")
temporary_seal=$(mktemp "$artifact_directory/.preflight.XXXXXX")
jq -n -S \
  --arg commit "$candidate_commit" --arg tree "$candidate_tree" --arg binary "$binary_sha256" \
  --arg lock "$lock_sha256" --arg workspace "$workspace_manifest_sha256" \
  --arg source "$source_sha256" --arg config "$config_sha256" \
  --arg preconditions "$preconditions_sha256" \
  --arg runner "$runner_sha256" --arg preflight "$preflight_sha256" \
  --arg record "$record_sha256" --arg aggregate "$aggregate_sha256" \
  --arg lifecycle "$lifecycle_sha256" --arg inputs "$input_tree_sha256" \
  --arg manifest "$(hash_file "$fixture_manifest")" --arg evidence "$manifest_evidence_sha256" \
  --arg readme "$readme_sha256" \
  --arg cargo_version "$cargo_version" --arg rust_version "$rust_version" \
  --arg llvm_version "$llvm_version" --arg target_triple "$target_triple" \
  --arg cargo_executable "$cargo_executable" --arg rustc_executable "$rustc_executable" \
  --arg cargo_executable_sha "$cargo_executable_sha256" \
  --arg rustc_executable_sha "$rustc_executable_sha256" \
  '{schema_version:1,issue:431,kind:"builtins_current_benchmark_preflight",status:"READY",
    candidate_commit:$commit,candidate_tree:$tree,binary_sha256:$binary,
    cargo_lock_sha256:$lock,workspace_manifest_sha256:$workspace,
    tool_source_sha256:$source,cargo_config_sha256:$config,
    preconditions_sha256:$preconditions,
    runner_sha256:$runner,preflight_script_sha256:$preflight,
    record_validator_sha256:$record,aggregate_validator_sha256:$aggregate,
    lifecycle_sha256:$lifecycle,input_tree_sha256:$inputs,
    fixture_manifest_sha256:$manifest,manifest_evidence_sha256:$evidence,readme_sha256:$readme,
    cargo_version:$cargo_version,rust_version:$rust_version,llvm_version:$llvm_version,
    target_triple:$target_triple,cargo_executable:$cargo_executable,
    rustc_executable:$rustc_executable,cargo_executable_sha256:$cargo_executable_sha,
    rustc_executable_sha256:$rustc_executable_sha,
    target_features:"+avx2,+fma",profile:"release",opt_level:"3",lto:"fat",codegen_units:1,
    records_required:20,warmup_passes:1,measured_rounds:2,
    preflight_invocations:1,runner_invocations:0,workload_invocations:0,
    timed_benchmark_invocations:0}' >"$temporary_seal"
mv -n -- "$temporary_seal" "$seal"
one_link_file "$seal" || fail 'preflight seal publication failed'
printf 'Issue-431 current benchmark preflight: READY (preflight/runner/workload/timed=1/0/0/0)\n'
