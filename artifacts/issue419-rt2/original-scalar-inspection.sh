#!/usr/bin/env bash
set -euo pipefail
repo=$1 target=$2 evidence=$3
export PATH=/home/bl/.rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/bin:$PATH
: >"$evidence"
cd "$repo"
CARGO_TARGET_DIR="$target" CARGO_PROFILE_RELEASE_LTO=false RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p engine -p source -p target-smoke >>"$evidence" 2>&1
cfg=$(rustc --print cfg --target wasm32-unknown-unknown -C target-feature=-simd128); cfg_rc=$?; printf 'cfg_status=%s\n' "$cfg_rc" >>"$evidence"; [[ $cfg_rc == 0 ]]; if printf '%s\n' "$cfg" | rg -q '^target_feature="simd128"$'; then printf 'scalar_cfg_simd128=present ERROR\n' >>"$evidence"; exit 1; else rc=$?; [[ $rc == 1 ]]; printf 'scalar_cfg_simd128=absent scan_status=%s\n' "$rc" >>"$evidence"; fi
scratch=$(mktemp -d /tmp/sol419-objects.XXXXXX); printf 'extraction_root=%s\n' "$scratch" >>"$evidence"
for family in engine source target_smoke; do
  mapfile -t archives < <(find "$target/wasm32-unknown-unknown/release/deps" -maxdepth 1 -type f -name "lib${family}-*.rlib" -print | sort); find_rc=$?; printf 'archive_family=%s enumeration_status=%s count=%s\n' "$family" "$find_rc" "${#archives[@]}" >>"$evidence"; [[ $find_rc == 0 && ${#archives[@]} == 1 ]]
  archive=${archives[0]}; printf 'archive=%s\n' "$archive" >>"$evidence"; members=$(ar t "$archive" 2>>"$evidence"); ar_t_rc=$?; printf 'archive_list_status=%s members=%s\n' "$ar_t_rc" "$(printf '%s\n' "$members" | wc -l)" >>"$evidence"; [[ $ar_t_rc == 0 && -n $members ]]
  dir="$scratch/$family"; mkdir "$dir"; (cd "$dir" && ar x "$archive") >>"$evidence" 2>&1; extract_rc=$?; printf 'archive_extract_status=%s family=%s\n' "$extract_rc" "$family" >>"$evidence"; [[ $extract_rc == 0 ]]
done
objects_file=/tmp/sol-419-nonlto-objects.txt
find "$scratch" -type f -name '*.o' -print | sort >"$objects_file"; object_find_rc=$?; count=$(wc -l <"$objects_file"); printf 'object_enumeration_status=%s object_count=%s identities=%s\n' "$object_find_rc" "$count" "$objects_file" >>"$evidence"; [[ $object_find_rc == 0 && $count -gt 0 ]]
while IFS= read -r object; do
  decoded=$(mktemp /tmp/sol419-decoded.XXXXXX); diagnostic=$(mktemp /tmp/sol419-diagnostic.XXXXXX)
  set +e; wasm-objdump -d "$object" >"$decoded" 2>"$diagnostic"; decode_rc=$?; set -e
  printf 'object=%s decoder_status=%s diagnostic_bytes=%s\n' "$object" "$decode_rc" "$(wc -c <"$diagnostic")" >>"$evidence"; [[ $decode_rc == 0 ]] || { cat "$diagnostic" >>"$evidence"; exit 1; }
  set +e; rg -n 'atomic\.' "$decoded" >>"$evidence" 2>&1; scan_rc=$?; set -e
  printf 'object=%s atomic_scan_status=%s\n' "$object" "$scan_rc" >>"$evidence"; case $scan_rc in 0) exit 1;; 1) :;; *) exit 1;; esac
done <"$objects_file"
printf 'complete opcode inspection PASS: named scalar non-LTO engine/source/target_smoke object set, objects=%s\n' "$count" >>"$evidence"
