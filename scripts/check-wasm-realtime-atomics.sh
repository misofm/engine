#!/usr/bin/env bash
# Build and inspect the browser-local queue monomorphization; it must require no atomic opcode.
set -euo pipefail

target_directory="${1:-target/ci/wasm-realtime-local}"
for tool in rustc cargo ar wasm-objdump rg find sort realpath mktemp; do
    command -v "$tool" >/dev/null 2>&1 || { printf '%s is required for the Wasm realtime atomic inspection\n' "$tool" >&2; exit 1; }
done
mkdir -p "$target_directory"
inspection_target="$(mktemp -d "$target_directory/.wasm-inspection.XXXXXX")"
scratch="$(mktemp -d)"
keep_target=1
cleanup() { rm -rf -- "$scratch"; if [[ "$keep_target" == 0 ]]; then rm -rf -- "$inspection_target"; else printf 'inspection target retained for diagnostics: %s\n' "$inspection_target" >&2; fi; }
trap cleanup EXIT

cfg_file="$scratch/cfg"
if ! rustc --print cfg --target wasm32-unknown-unknown -C target-feature=-simd128 >"$cfg_file" 2>"$scratch/cfg.stderr"; then printf 'rustc cfg production failed\n' >&2; exit 1; fi
if rg -q '^target_has_atomic="ptr"$' "$cfg_file"; then :; else cfg_status=$?; if [[ "$cfg_status" == 1 ]]; then printf 'wasm target does not advertise pointer-width atomic support\n' >&2; else printf 'cfg atomic-support search failed (status %s)\n' "$cfg_status" >&2; fi; exit 1; fi
if rg -q '^target_feature="atomics"$' "$cfg_file"; then printf 'browser-local fallback artifact unexpectedly enables Wasm atomics\n' >&2; exit 1; else cfg_status=$?; [[ "$cfg_status" == 1 ]] || { printf 'cfg atomics-feature search failed (status %s)\n' "$cfg_status" >&2; exit 1; }; fi

if ! CARGO_TARGET_DIR="$inspection_target" CARGO_PROFILE_RELEASE_LTO=false RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p engine -p source -p target-smoke; then printf 'scalar NON-LTO inspection cargo build failed\n' >&2; exit 1; fi
deps="$inspection_target/wasm32-unknown-unknown/release/deps"
declare -a families=(engine source target_smoke)
declare -a archives=() objects=()
for family in "${families[@]}"; do
    archive_list="$scratch/$family.archives"
    if ! find "$deps" -maxdepth 1 -type f -name "lib${family//_/-}-*.rlib" -print0 >"$archive_list"; then printf '%s archive discovery failed\n' "$family" >&2; exit 1; fi
    if ! sort -z -o "$archive_list" "$archive_list"; then printf '%s archive sort failed\n' "$family" >&2; exit 1; fi
    mapfile -d '' -t found_archives <"$archive_list"
    [[ "${#found_archives[@]}" == 1 ]] || { printf '%s archive population is incomplete (found %s, expected 1)\n' "$family" "${#found_archives[@]}" >&2; exit 1; }
    archives+=("$(realpath -- "${found_archives[0]}")")
done

for index in "${!families[@]}"; do
    family="${families[$index]}"; archive="${archives[$index]}"; family_dir="$scratch/$family"; mkdir -p "$family_dir"; members="$family_dir/members"
    if ! ar t "$archive" >"$members" 2>"$family_dir/ar-list.stderr"; then printf '%s archive member listing failed\n' "$family" >&2; exit 1; fi
    if ! sort "$members" >"$family_dir/members.sorted"; then printf '%s archive member sorting failed\n' "$family" >&2; exit 1; fi
    if [[ "$(wc -l <"$family_dir/members.sorted")" != "$(sort -u "$family_dir/members.sorted" | wc -l)" ]]; then printf '%s archive contains duplicate member names\n' "$family" >&2; exit 1; fi
    if ! (cd "$family_dir" && ar x "$archive") 2>"$family_dir/ar-extract.stderr"; then printf '%s archive extraction failed\n' "$family" >&2; exit 1; fi
    if ! find "$family_dir" -type f -name '*.o' -print0 >"$family_dir/objects"; then printf '%s object discovery failed\n' "$family" >&2; exit 1; fi
    if ! sort -z -o "$family_dir/objects" "$family_dir/objects"; then printf '%s object sort failed\n' "$family" >&2; exit 1; fi
    mapfile -d '' -t family_objects <"$family_dir/objects"
    [[ "${#family_objects[@]}" -gt 0 ]] || { printf '%s archive has no object members\n' "$family" >&2; exit 1; }
    while IFS= read -r member; do [[ "$member" == *.o ]] || continue; [[ -f "$family_dir/$member" ]] || { printf '%s object member was not extracted: %s\n' "$family" "$member" >&2; exit 1; }; done <"$family_dir/members.sorted"
    for object in "${family_objects[@]}"; do objects+=("$object"); done
done
[[ "${#objects[@]}" -gt 0 ]] || { printf 'no Wasm objects were available for atomic inspection\n' >&2; exit 1; }

observation_match=0
object_index=0
for object in "${objects[@]}"; do
    object_index=$((object_index + 1))
    decoded="$scratch/object-$object_index.decoded"; decoded_stderr="$scratch/object-$object_index.objdump.stderr"
    if ! wasm-objdump -d "$object" >"$decoded" 2>"$decoded_stderr"; then printf 'wasm-objdump failed for object: %s\n' "$object" >&2; exit 1; fi
    if rg -n 'atomic\.' "$decoded"; then printf 'browser-local fallback contains an atomic opcode: %s\n' "$object" >&2; exit 1; else scan_status=$?; [[ "$scan_status" == 1 ]] || { printf 'opcode scan failed for object %s (status %s)\n' "$object" "$scan_status" >&2; exit 1; }; fi
    if rg -l --binary 'observe' "$object" >/dev/null; then observation_match=1; else observation_status=$?; [[ "$observation_status" == 1 ]] || { printf 'observation object search failed for %s (status %s)\n' "$object" "$observation_status" >&2; exit 1; }; fi
done
if [[ "$observation_match" == 0 ]]; then
    if rg -q 'ObservationSlot' crates/engine/src/realtime/observe.rs; then :; else source_status=$?; if [[ "$source_status" == 1 ]]; then printf 'observation symbol absent from objects and source fallback\n' >&2; else printf 'source ObservationSlot search failed (status %s)\n' "$source_status" >&2; fi; exit 1; fi
fi
keep_target=0
printf 'wasm realtime atomics: ok (%s objects, scalar NON-LTO engine/source/target_smoke)\n' "${#objects[@]}"
