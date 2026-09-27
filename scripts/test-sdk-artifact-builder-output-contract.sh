#!/usr/bin/env bash
# Focused qualification for the builders' caller-owned empty-directory contract.
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
scratch=$(mktemp -d)
cleanup() {
  rm -rf -- "$scratch"
}
trap cleanup EXIT

mock_bin="$scratch/mock-bin"
mkdir "$mock_bin"
cat >"$mock_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'RUSTFLAGS=%s %s\n' "${RUSTFLAGS-}" "$*" >>"$(dirname "$0")/cargo.log"
if [[ $1 == build ]]; then
  case " $* " in
    *" -p host-web "*) artifact=host_web ;;
    *) echo "unexpected cargo build invocation" >&2; exit 1 ;;
  esac
  output="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/$artifact.wasm"
  mkdir -p "$(dirname "$output")"
  # The fixture carries the flags it was built with, so a changed flag is a changed module.
  printf '%s fixture\n%s\n' "$artifact" "${RUSTFLAGS-}" >"$output"
  exit 0
fi
if [[ $1 == run && " $* " == *" -p parameter-metadata "* ]]; then
  output=${@: -1}
  printf 'parameter metadata fixture\n' >"$output/parameter-metadata.json"
  exit 0
fi
echo "unexpected cargo invocation" >&2
exit 1
EOF
chmod +x "$mock_bin/cargo"

cat >"$mock_bin/sha256sum" <<EOF
#!/usr/bin/env bash
set -euo pipefail
case "\$1" in
  */host_web.wasm) [[ \${MOCK_UNPINNED:-0} == 1 ]] && { printf '%064d  %s\\n' 0 "\$1"; exit 0; }
    printf '%s  %s\\n' "$(tr -d '\n' <"$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256")" "\$1" ;;
  *) echo "unexpected sha256sum input" >&2; exit 1 ;;
esac
EOF
chmod +x "$mock_bin/sha256sum"

run_builder() {
  local builder=$1
  local output=$2
  local log=$3
  local mock_log="$mock_bin/cargo.log"
  rm -f "$mock_log" "$log"
  local status=0
  PATH="$mock_bin:$PATH" bash "$repo_root/scripts/$builder" "$output" || status=$?
  if [[ -e $mock_log ]]; then
    cp "$mock_log" "$log"
  fi
  return "$status"
}

assert_refuses_without_build() {
  local builder=$1
  local output=$2
  local expected_sentinel=$3
  local log="$scratch/$builder-refusal.log"
  local status=0
  run_builder "$builder" "$output" "$log" >/dev/null 2>&1 || status=$?
  [[ $status == 2 ]] || {
    echo "$builder accepted an invalid output directory" >&2
    exit 1
  }
  cmp -s "$expected_sentinel" "$output/sentinel" || {
    echo "$builder changed a refused output directory" >&2
    exit 1
  }
  [[ ! -e $log ]] || {
    echo "$builder built after refusing output" >&2
    exit 1
  }
}

assert_refuses_path() {
  local builder=$1
  local output=$2
  local log="$scratch/$builder-path-refusal.log"
  local status=0
  run_builder "$builder" "$output" "$log" >/dev/null 2>&1 || status=$?
  [[ $status == 2 ]] || {
    echo "$builder accepted invalid output path $output" >&2
    exit 1
  }
  [[ ! -e $log ]] || {
    echo "$builder built after refusing output path" >&2
    exit 1
  }
}

check_builder() {
  local builder=$1
  local prefix=$2
  local output="$scratch/$prefix-empty"
  local log="$scratch/$prefix-success.log"
  mkdir "$output"
  run_builder "$builder" "$output" "$log"
  [[ -s $log ]] || {
    echo "$builder did not run its mocked build" >&2
    exit 1
  }
  case "$builder" in
    build-web-audioworklet.sh)
      [[ $(head -n 1 "$output/miso-engine-v1-audio-worklet.simd128.wasm") == 'host_web fixture' ]]
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet.js" \
        "$output/miso-engine-v1-audio-worklet.js"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.js" \
        "$output/miso-engine-v1-audio-worklet-host.js"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts" \
        "$output/miso-engine-v1-audio-worklet-host.d.ts"
      cmp -s <(printf 'parameter metadata fixture\n') "$output/parameter-metadata.json"
      ;;
  esac

  local non_empty="$scratch/$prefix-non-empty"
  local expected_sentinel="$scratch/$prefix-non-empty-sentinel"
  mkdir "$non_empty"
  printf 'do not overwrite\n' >"$non_empty/sentinel"
  cp "$non_empty/sentinel" "$expected_sentinel"
  assert_refuses_without_build "$builder" "$non_empty" "$expected_sentinel"

  local symlink_target="$scratch/$prefix-symlink-target"
  local symlink_output="$scratch/$prefix-symlink-output"
  mkdir "$symlink_target"
  printf 'do not overwrite\n' >"$symlink_target/sentinel"
  ln -s "$symlink_target" "$symlink_output"
  local symlink_log="$scratch/$prefix-symlink.log"
  local status=0
  run_builder "$builder" "$symlink_output" "$symlink_log" >/dev/null 2>&1 || status=$?
  [[ $status == 2 ]] || {
    echo "$builder accepted a symlink output directory" >&2
    exit 1
  }
  cmp -s <(printf 'do not overwrite\n') "$symlink_target/sentinel" || {
    echo "$builder changed a symlink target" >&2
    exit 1
  }
  [[ ! -e $symlink_log ]] || {
    echo "$builder built after refusing symlink output" >&2
    exit 1
  }

  assert_refuses_path "$builder" "$scratch/$prefix-missing"
  local regular_file="$scratch/$prefix-regular-file"
  printf 'not a directory\n' >"$regular_file"
  assert_refuses_path "$builder" "$regular_file"
}

check_builder build-web-audioworklet.sh web

# Issue #1009: `build-web-audioworklet.sh --module-only` is the one home of the shipped module's
# cargo line for every reader that is not the delivery build (`run-wasm-gates.sh`'s V8 spill gate).
# It must run exactly the delivery build's cargo line, write the module alone, not hold it to the
# pin, refuse what the delivery build refuses, and follow a flag changed in the script.
run_module_only() {
  local root=$1 output=$2 log=$3
  local mock_log="$mock_bin/cargo.log"
  rm -f "$mock_log" "$log"
  local status=0
  PATH="$mock_bin:$PATH" bash "$root/scripts/build-web-audioworklet.sh" --module-only "$output" \
    >/dev/null || status=$?
  if [[ -e $mock_log ]]; then
    cp "$mock_log" "$log"
  fi
  return "$status"
}
module_name=miso-engine-v1-audio-worklet.simd128.wasm
full="$scratch/module-full"
module="$scratch/module-only"
mkdir "$full" "$module"
run_builder build-web-audioworklet.sh "$full" "$scratch/module-full.log"
run_module_only "$repo_root" "$module" "$scratch/module-only.log"
[[ $(find "$module" -mindepth 1 -printf '%f\n') == "$module_name" ]] || {
  echo "--module-only wrote more or less than the module" >&2
  exit 1
}
cmp -s "$full/$module_name" "$module/$module_name" || {
  echo "--module-only built a different module from the delivery build" >&2
  exit 1
}
[[ $(grep -c . "$scratch/module-only.log") == 1 ]] &&
  cmp -s "$scratch/module-only.log" <(grep ' build ' "$scratch/module-full.log") || {
  echo "--module-only did not run exactly the delivery build's cargo line, and only it" >&2
  exit 1
}
unpinned_full="$scratch/unpinned-full"
unpinned_module="$scratch/unpinned-module"
mkdir "$unpinned_full" "$unpinned_module"
status=0
MOCK_UNPINNED=1 run_builder build-web-audioworklet.sh "$unpinned_full" "$scratch/unpinned.log" \
  >/dev/null 2>&1 || status=$?
[[ $status == 1 && -z $(find "$unpinned_full" -mindepth 1 -print -quit) ]] || {
  echo "the delivery build accepted a module that does not match its pin" >&2
  exit 1
}
MOCK_UNPINNED=1 run_module_only "$repo_root" "$unpinned_module" "$scratch/unpinned.log" || {
  echo "--module-only refused a module that does not match the pin" >&2
  exit 1
}
refused="$scratch/module-refused"
mkdir "$refused"
printf 'do not overwrite\n' >"$refused/sentinel"
status=0
run_module_only "$repo_root" "$refused" "$scratch/module-refused.log" 2>/dev/null || status=$?
[[ $status == 2 && ! -e $scratch/module-refused.log ]] || {
  echo "--module-only accepted a non-empty output directory or built after refusing it" >&2
  exit 1
}
# A flag changed in the script reaches the module. The copy keeps one root, so the path remap in
# RUSTFLAGS is the same for both builds and the flag is the only difference.
copy="$scratch/flag-copy"
mkdir -p "$copy/scripts" "$copy/hosts/host-web/web" "$scratch/flag-before" "$scratch/flag-after"
cp "$repo_root/scripts/build-web-audioworklet.sh" "$copy/scripts/"
cp "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256" \
  "$copy/hosts/host-web/web/"
run_module_only "$copy" "$scratch/flag-before" "$scratch/flag-before.log"
sed -i 's/-C target-feature=+simd128 /-C target-feature=+simd128 -C opt-level=s /' \
  "$copy/scripts/build-web-audioworklet.sh"
run_module_only "$copy" "$scratch/flag-after" "$scratch/flag-after.log"
! cmp -s "$scratch/flag-before/$module_name" "$scratch/flag-after/$module_name" &&
  grep -q -- '-C opt-level=s' "$scratch/flag-after/$module_name" || {
  echo "a flag changed in build-web-audioworklet.sh did not reach the --module-only module" >&2
  exit 1
}
# And the V8 spill leg of run-wasm-gates.sh takes its module from that mode and builds none itself.
wasm_gates="$repo_root/scripts/run-wasm-gates.sh"
grep -q 'bash scripts/build-web-audioworklet.sh --module-only' "$wasm_gates" &&
  ! grep -q -- '-p host-web' "$wasm_gates" || {
  echo "run-wasm-gates.sh builds the shipped module other than through --module-only" >&2
  exit 1
}
if bash "$repo_root/scripts/sdk-package.sh" build first second >/dev/null 2>&1; then
  echo "sdk package builder accepted a retired second codec artifact directory" >&2
  exit 1
fi
if node "$repo_root/sdk/codegen/stage-package.mjs" first second >/dev/null 2>&1; then
  echo "package staging accepted a retired second codec artifact directory" >&2
  exit 1
fi
echo "SDK artifact builder output-directory, single-directory and --module-only contract passed"
