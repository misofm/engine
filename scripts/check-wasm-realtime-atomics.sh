#!/usr/bin/env bash
set -euo pipefail
target_directory="${1:-target/ci/wasm-realtime-local}"
for tool in rustc cargo ar wasm-objdump rg find sort mktemp sed; do command -v "$tool" >/dev/null 2>&1 || { printf '%s is required for the Wasm realtime atomic inspection\n' "$tool" >&2; exit 1; }; done
mkdir -p "$target_directory"
checker_root="$PWD"
set +e; cd "$target_directory"; target_status=$?; set -e
[[ "$target_status" == 0 ]] || { printf 'inspection target resolution failed (status %s): %s\n' "$target_status" "$target_directory" >&2; exit 1; }
target_directory="$PWD"
set +e; cd "$checker_root"; root_status=$?; set -e
[[ "$root_status" == 0 ]] || { printf 'checker root restoration failed (status %s): %s\n' "$root_status" "$checker_root" >&2; exit 1; }
inspection_target="$(mktemp -d "$target_directory/.wasm-inspection.XXXXXX")"
scratch="$(mktemp -d)"; keep_target=1
cleanup() { rm -rf -- "$scratch"; if [[ "$keep_target" == 0 ]]; then rm -rf -- "$inspection_target"; else printf 'inspection target retained for diagnostics: %s\n' "$inspection_target" >&2; fi; }
trap cleanup EXIT
failed() {
  local op="$1" status="$2" identity="$3" out="${4:-}" err="${5:-}"
  printf '%s failed (status %s): %s\n' "$op" "$status" "$identity" >&2
  [[ -z "$out" || ! -s "$out" ]] || { printf '%s partial stdout:\n' "$op" >&2; sed 's/^/  /' "$out" >&2; }
  [[ -z "$err" || ! -s "$err" ]] || { printf '%s stderr:\n' "$op" >&2; sed 's/^/  /' "$err" >&2; }
  exit 1
}
run_capture() {
  local out="$1" err="$2"; shift 2
  set +e; "$@" >"$out" 2>"$err"; capture_status=$?; set -e
}

cfg="$scratch/cfg"; run_capture "$cfg" "$scratch/cfg.err" rustc --print cfg --target wasm32-unknown-unknown -C target-feature=-simd128
[[ "$capture_status" == 0 ]] || failed 'rustc cfg production' "$capture_status" wasm32-unknown-unknown "$cfg" "$scratch/cfg.err"
run_capture "$scratch/pointer.matches" "$scratch/pointer.err" rg -n '^target_has_atomic="ptr"$' "$cfg"; status=$capture_status
if [[ "$status" == 1 ]]; then printf 'wasm target does not advertise pointer-width atomic support\n' >&2; exit 1; elif [[ "$status" != 0 ]]; then failed 'cfg atomic-support search' "$status" "$cfg" "$scratch/pointer.matches" "$scratch/pointer.err"; fi
run_capture "$scratch/feature.matches" "$scratch/feature.err" rg -n '^target_feature="atomics"$' "$cfg"; status=$capture_status
if [[ "$status" == 0 ]]; then printf 'browser-local fallback artifact unexpectedly enables Wasm atomics\n' >&2; exit 1; elif [[ "$status" != 1 ]]; then failed 'cfg atomics-feature search' "$status" "$cfg" "$scratch/feature.matches" "$scratch/feature.err"; fi

set +e
CARGO_TARGET_DIR="$inspection_target" CARGO_PROFILE_RELEASE_LTO=false RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p engine -p source -p target-smoke >"$scratch/cargo.out" 2>"$scratch/cargo.err"
status=$?; set -e
[[ "$status" == 0 ]] || failed 'scalar NON-LTO inspection cargo build' "$status" "$inspection_target" "$scratch/cargo.out" "$scratch/cargo.err"

deps="$inspection_target/wasm32-unknown-unknown/release/deps"
families=(engine source target_smoke); archives=(); objects=()
for family in "${families[@]}"; do
  list="$scratch/$family.archives"
  run_capture "$list" "$scratch/$family.find.err" find "$deps" -maxdepth 1 -type f -name "lib$family-*.rlib" -print0; status=$capture_status
  [[ "$status" == 0 ]] || failed "$family archive discovery" "$status" "$deps" "$list" "$scratch/$family.find.err"
  run_capture /dev/null "$scratch/$family.sort.err" sort -z -o "$list" "$list"; status=$capture_status
  [[ "$status" == 0 ]] || failed "$family archive sort" "$status" "$list" '' "$scratch/$family.sort.err"
  mapfile -d '' -t found <"$list"
  [[ "${#found[@]}" == 1 ]] || { printf '%s archive population is incomplete (found %s, expected 1)\n' "$family" "${#found[@]}" >&2; exit 1; }
  archives+=("${found[0]}")
done

for index in "${!families[@]}"; do
  family="${families[$index]}"; archive="${archives[$index]}"; dir="$scratch/$family"; mkdir -p "$dir"
  run_capture "$dir/members" "$dir/list.err" ar t "$archive"; status=$capture_status
  [[ "$status" == 0 ]] || failed "$family archive member listing" "$status" "$archive" "$dir/members" "$dir/list.err"
  run_capture "$dir/members.sorted" "$dir/member-sort.err" sort "$dir/members"; status=$capture_status
  [[ "$status" == 0 ]] || failed "$family archive member sort" "$status" "$archive" "$dir/members.sorted" "$dir/member-sort.err"
  expected=(); declare -A seen=()
  while IFS= read -r member; do
    [[ "$member" == *.o ]] || continue
    [[ ! -v "seen[$member]" ]] || { printf '%s archive contains duplicate object member: %s\n' "$family" "$member" >&2; exit 1; }
    seen["$member"]=1; expected+=("$member")
  done <"$dir/members.sorted"
  [[ "${#expected[@]}" -gt 0 ]] || { printf '%s archive has no object members\n' "$family" >&2; exit 1; }
  set +e; (cd "$dir" && ar x "$archive") >"$dir/extract.out" 2>"$dir/extract.err"; status=$?; set -e
  [[ "$status" == 0 ]] || failed "$family archive extraction" "$status" "$archive" "$dir/extract.out" "$dir/extract.err"
  run_capture "$dir/objects" "$dir/object-find.err" find "$dir" -maxdepth 1 -type f -name '*.o' -printf '%f\0'; status=$capture_status
  [[ "$status" == 0 ]] || failed "$family object discovery" "$status" "$dir" "$dir/objects" "$dir/object-find.err"
  run_capture /dev/null "$dir/object-sort.err" sort -z -o "$dir/objects" "$dir/objects"; status=$capture_status
  [[ "$status" == 0 ]] || failed "$family object sort" "$status" "$dir/objects" '' "$dir/object-sort.err"
  mapfile -d '' -t discovered <"$dir/objects"
  [[ "${#expected[@]}" == "${#discovered[@]}" ]] || { printf '%s object reconciliation failed (expected %s, discovered %s)\n' "$family" "${#expected[@]}" "${#discovered[@]}" >&2; exit 1; }
  for i in "${!expected[@]}"; do
    [[ "${expected[$i]}" == "${discovered[$i]}" ]] || { printf '%s object reconciliation failed (expected %s, discovered %s)\n' "$family" "${expected[$i]}" "${discovered[$i]}" >&2; exit 1; }
    objects+=("$dir/${discovered[$i]}")
  done
  unset seen
done
[[ "${#objects[@]}" -gt 0 ]] || { printf 'no Wasm objects were available for atomic inspection\n' >&2; exit 1; }

observation_match=0; n=0
for object in "${objects[@]}"; do
  n=$((n + 1)); decoded="$scratch/$n.decoded"
  run_capture "$decoded" "$scratch/$n.decode.err" wasm-objdump -d "$object"; status=$capture_status
  [[ "$status" == 0 ]] || failed wasm-objdump "$status" "$object" "$decoded" "$scratch/$n.decode.err"
  run_capture "$scratch/$n.atomic.matches" "$scratch/$n.atomic.err" rg -n 'atomic\.' "$decoded"; status=$capture_status
  if [[ "$status" == 0 ]]; then cat "$scratch/$n.atomic.matches" >&2; printf 'browser-local fallback contains an atomic opcode: %s\n' "$object" >&2; exit 1; elif [[ "$status" != 1 ]]; then failed 'opcode scan' "$status" "$object" "$scratch/$n.atomic.matches" "$scratch/$n.atomic.err"; fi
  run_capture "$scratch/$n.observe.matches" "$scratch/$n.observe.err" rg -l --binary observe "$object"; status=$capture_status
  if [[ "$status" == 0 ]]; then observation_match=1; elif [[ "$status" != 1 ]]; then failed 'observation object search' "$status" "$object" "$scratch/$n.observe.matches" "$scratch/$n.observe.err"; fi
done
if [[ "$observation_match" == 0 ]]; then
  run_capture "$scratch/source.matches" "$scratch/source.err" rg -n ObservationSlot crates/engine/src/realtime/observe.rs; status=$capture_status
  if [[ "$status" == 1 ]]; then printf 'observation symbol absent from objects and source fallback\n' >&2; exit 1; elif [[ "$status" != 0 ]]; then failed 'source ObservationSlot search' "$status" crates/engine/src/realtime/observe.rs "$scratch/source.matches" "$scratch/source.err"; fi
fi
keep_target=0
printf 'wasm realtime atomics: ok (%s objects, scalar NON-LTO engine/source/target_smoke)\n' "${#objects[@]}"
