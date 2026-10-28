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
printf 'RUSTUP_TOOLCHAIN=%s RUSTFLAGS=%s %s\n' "${RUSTUP_TOOLCHAIN-}" "${RUSTFLAGS-}" "$*" \
  >>"$(dirname "$0")/cargo.log"
if [[ $1 == build ]]; then
  case " $* " in
    *" -p host-web "*) artifact=host_web ;;
    *" -p blake3-web "*) artifact=blake3_web ;;
    *) echo "unexpected cargo build invocation" >&2; exit 1 ;;
  esac
  output="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/$artifact.wasm"
  mkdir -p "$(dirname "$output")"
  # The fixture is a WebAssembly module, since the build strips its `name` section (#1109). It
  # carries the flags it was built with in another custom section, so a changed flag is a changed
  # module.
  python3 -B - "$output" "$artifact fixture" "${RUSTFLAGS-}" <<'PY'
import sys
def custom(name: bytes, body: bytes) -> bytes:
    payload = bytes([len(name)]) + name + body
    size = bytearray()
    value = len(payload)
    while True:
        size.append((value & 0x7F) | (0x80 if value >> 7 else 0))
        value >>= 7
        if not value:
            break
    return b"\x00" + bytes(size) + payload
fixture = f"\n{sys.argv[2]}\n{sys.argv[3]}\n".encode()
with open(sys.argv[1], "wb") as output:
    output.write(b"\0asm\x01\x00\x00\x00" + custom(b"fixture", fixture)
                 + custom(b"name", b"\x00\x09\x08host_web"))
PY
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
  */miso-engine-v1-audio-worklet.simd128.wasm) [[ \${MOCK_UNPINNED:-0} == 1 ]] && { printf '%064d  %s\\n' 0 "\$1"; exit 0; }
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
  run_builder "$builder" "$output" "$log" >/dev/null
  [[ -s $log ]] || {
    echo "$builder did not run its mocked build" >&2
    exit 1
  }
  case "$builder" in
    build-web-audioworklet.sh)
      grep -aqx 'host_web fixture' "$output/miso-engine-v1-audio-worklet.simd128.wasm"
      grep -aqx 'blake3_web fixture' "$output/miso-engine-v1-blake3.simd128.wasm"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet.js" \
        "$output/miso-engine-v1-audio-worklet.js"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.js" \
        "$output/miso-engine-v1-audio-worklet-host.js"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts" \
        "$output/miso-engine-v1-audio-worklet-host.d.ts"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-control-worker.js" \
        "$output/miso-engine-v1-control-worker.js"
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
# It must run exactly the delivery build's cargo line, write the module alone, refuse what the
# delivery build refuses, and follow a flag changed in the script.
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
run_builder build-web-audioworklet.sh "$full" "$scratch/module-full.log" >/dev/null
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
  cmp -s "$scratch/module-only.log" <(grep ' build .* -p host-web$' "$scratch/module-full.log") || {
  echo "--module-only did not run exactly the delivery build's cargo line, and only it" >&2
  exit 1
}
# Issue #1061: no mode but --check-pin holds the module to the committed pin. The delivery build and
# --module-only write a module that differs from the pin and print its digest; --check-pin, the
# release fingerprint check, refuses it and writes nothing, and accepts the pinned module.
pin=$(tr -d '\n' <"$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256")
unpinned_digest=$(printf '%064d' 0)
unpinned_full="$scratch/unpinned-full"
unpinned_module="$scratch/unpinned-module"
unpinned_checked="$scratch/unpinned-checked"
pinned_checked="$scratch/pinned-checked"
mkdir "$unpinned_full" "$unpinned_module" "$unpinned_checked" "$pinned_checked"
printed=$(MOCK_UNPINNED=1 PATH="$mock_bin:$PATH" \
  bash "$repo_root/scripts/build-web-audioworklet.sh" "$unpinned_full") || {
  echo "the delivery build refused a module that does not match the pin" >&2
  exit 1
}
[[ $printed == "AudioWorklet module $unpinned_digest" ]] &&
  cmp -s "$full/$module_name" "$unpinned_full/$module_name" || {
  echo "the delivery build did not write an unpinned module and print its digest: $printed" >&2
  exit 1
}
printed=$(MOCK_UNPINNED=1 PATH="$mock_bin:$PATH" \
  bash "$repo_root/scripts/build-web-audioworklet.sh" --module-only "$unpinned_module") || {
  echo "--module-only refused a module that does not match the pin" >&2
  exit 1
}
[[ $printed == "AudioWorklet module $unpinned_digest" ]] || {
  echo "--module-only did not print the module's digest: $printed" >&2
  exit 1
}
status=0
MOCK_UNPINNED=1 PATH="$mock_bin:$PATH" \
  bash "$repo_root/scripts/build-web-audioworklet.sh" --check-pin "$unpinned_checked" \
  >/dev/null 2>"$scratch/unpinned-checked.err" || status=$?
[[ $status == 1 && -z $(find "$unpinned_checked" -mindepth 1 -print -quit) ]] &&
  grep -q "AudioWorklet artifact pin mismatch: expected=$pin observed=$unpinned_digest" \
    "$scratch/unpinned-checked.err" || {
  echo "--check-pin accepted a module that does not match the pin, or wrote output" >&2
  exit 1
}
printed=$(PATH="$mock_bin:$PATH" \
  bash "$repo_root/scripts/build-web-audioworklet.sh" --check-pin "$pinned_checked") || {
  echo "--check-pin refused the pinned module" >&2
  exit 1
}
[[ $printed == "AudioWorklet module $pin" ]] &&
  cmp -s "$full/$module_name" "$pinned_checked/$module_name" &&
  cmp -s <(printf 'parameter metadata fixture\n') "$pinned_checked/parameter-metadata.json" || {
  echo "--check-pin did not write the whole delivery closure for the pinned module" >&2
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
# RUSTFLAGS is the same for both builds and the flag is the only difference. The copy carries no
# pin file: --module-only never reads it. It carries the toolchain file the build reads (#1334).
copy="$scratch/flag-copy"
mkdir -p "$copy/scripts" "$scratch/flag-before" "$scratch/flag-after"
cp "$repo_root/scripts/build-web-audioworklet.sh" "$repo_root/scripts/strip-wasm-names.py" \
  "$copy/scripts/"
cp "$repo_root/rust-toolchain.toml" "$copy/"
run_module_only "$copy" "$scratch/flag-before" "$scratch/flag-before.log"
sed -i 's/+mutable-globals"$/+mutable-globals -C opt-level=s"/' \
  "$copy/scripts/build-web-audioworklet.sh"
run_module_only "$copy" "$scratch/flag-after" "$scratch/flag-after.log"
! cmp -s "$scratch/flag-before/$module_name" "$scratch/flag-after/$module_name" &&
  grep -q -- '-C opt-level=s' "$scratch/flag-after/$module_name" || {
  echo "a flag changed in build-web-audioworklet.sh did not reach the --module-only module" >&2
  exit 1
}
# Issue #1109: the build ships its module without the `name` section and, with `--named-twin`,
# writes the named twin -- cargo's output, names and all -- beside it, in any mode. The pair are
# twins by `strip-wasm-names.py check`. The twin's directory is held to the output's rules, and
# refused before anything is built.
named_output="$scratch/named-output"
named_twin="$scratch/named-twin"
mkdir "$named_output" "$named_twin"
PATH="$mock_bin:$PATH" bash "$repo_root/scripts/build-web-audioworklet.sh" --module-only \
  --named-twin "$named_twin" "$named_output" >/dev/null 2>&1
twin_name=miso-engine-v1-audio-worklet.simd128.named.wasm
[[ $(find "$named_twin" -mindepth 1 -printf '%f\n') == "$twin_name" ]] &&
  cmp -s "$named_output/$module_name" "$module/$module_name" &&
  python3 -B "$repo_root/scripts/strip-wasm-names.py" check "$named_twin/$twin_name" \
    "$named_output/$module_name" >/dev/null || {
  echo "--named-twin did not write the named twin of the shipped module" >&2
  exit 1
}
for refused_twin in non-empty same-as-output; do
  twin_dir="$scratch/twin-$refused_twin"
  output_dir="$scratch/twin-$refused_twin-output"
  mkdir "$output_dir"
  if [[ $refused_twin == non-empty ]]; then
    mkdir "$twin_dir"
    printf 'do not overwrite\n' >"$twin_dir/sentinel"
  else
    twin_dir=$output_dir
  fi
  rm -f "$mock_bin/cargo.log"
  status=0
  PATH="$mock_bin:$PATH" bash "$repo_root/scripts/build-web-audioworklet.sh" \
    --named-twin "$twin_dir" "$output_dir" >/dev/null 2>&1 || status=$?
  [[ $status == 2 && ! -e $mock_bin/cargo.log && -z $(find "$output_dir" -mindepth 1 -print -quit) ]] || {
    echo "--named-twin accepted a $refused_twin directory or built after refusing it" >&2
    exit 1
  }
done
# Issue #1334: the module builds on rust-toolchain.toml's `[browser-artifact] channel`, a dated
# nightly. --print-toolchain prints it (read here by a TOML parser, not by the script's own awk) and
# builds nothing; a missing table or a floating or undated channel is refused, by --print-toolchain
# and by a build, before anything is built or written.
channel=$(python3 -B -c 'import sys, tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["browser-artifact"]["channel"])' \
  "$repo_root/rust-toolchain.toml")
rm -f "$mock_bin/cargo.log"
[[ $channel =~ ^nightly-[0-9]{4}-[0-9]{2}-[0-9]{2}$ &&
  $(PATH="$mock_bin:$PATH" bash "$repo_root/scripts/build-web-audioworklet.sh" --print-toolchain) == "$channel" &&
  ! -e $mock_bin/cargo.log ]] || {
  echo "--print-toolchain did not print the [browser-artifact] dated nightly, or built" >&2
  exit 1
}
# The delivery build's cargo lines (logged by the mock above) -- the worklet module's and, since
# #2111, the BLAKE3 module's -- run on that channel, with std rebuilt and simd128 on. Issue #1380
# D1: the worklet module alone adds atomics and links one imported, re-exported shared memory; the
# BLAKE3 module stays plain.
module_features='+simd128,+atomics,+bulk-memory,+mutable-globals'
module_links='-C link-arg=--shared-memory -C link-arg=--import-memory -C link-arg=--export-memory -C link-arg=--initial-memory=[0-9]* -C link-arg=--max-memory=536870912'
[[ $(grep -c ' build ' "$scratch/module-full.log") == 2 &&
  $(grep ' build ' "$scratch/module-full.log" | grep -c -- \
    "^RUSTUP_TOOLCHAIN=$channel RUSTFLAGS=-C target-feature=$module_features $module_links .* build --locked --release -Zbuild-std=std,panic_abort .*--target wasm32-unknown-unknown -p host-web$") == 1 &&
  $(grep ' build ' "$scratch/module-full.log" | grep -c -- \
    "^RUSTUP_TOOLCHAIN=$channel RUSTFLAGS=-C target-feature=+simd128 -C strip.* build --locked --release -Zbuild-std=std,panic_abort .*--target wasm32-unknown-unknown -p blake3-web$") == 1 ]] || {
  echo "a shipped cargo line does not run on the [browser-artifact] channel with -Zbuild-std and its features (the worklet module: atomics on one shared imported memory; BLAKE3: simd128 alone)" >&2
  exit 1
}
toolchain_copy="$scratch/toolchain-copy"
mkdir -p "$toolchain_copy/scripts"
cp "$repo_root/scripts/build-web-audioworklet.sh" "$repo_root/scripts/strip-wasm-names.py" \
  "$toolchain_copy/scripts/"
stable_table=$(sed -n '1,/^\[browser-artifact\]/{/^\[browser-artifact\]/!p}' "$repo_root/rust-toolchain.toml")
for refused_toolchain in missing-table floating undated in-another-table; do
  case $refused_toolchain in
    missing-table) printf '%s\n' "$stable_table" ;;
    floating) printf '%s\n[browser-artifact]\nchannel = "nightly"\n' "$stable_table" ;;
    undated) printf '%s\n[browser-artifact]\nchannel = "nightly-2026-8-20"\n' "$stable_table" ;;
    in-another-table) printf '%s\n[browser-artifact]\n[other]\nchannel = "%s"\n' "$stable_table" "$channel" ;;
  esac >"$toolchain_copy/rust-toolchain.toml"
  output_dir="$scratch/toolchain-$refused_toolchain"
  mkdir "$output_dir"
  rm -f "$mock_bin/cargo.log"
  printed_status=0 build_status=0
  PATH="$mock_bin:$PATH" bash "$toolchain_copy/scripts/build-web-audioworklet.sh" --print-toolchain \
    >/dev/null 2>&1 || printed_status=$?
  PATH="$mock_bin:$PATH" bash "$toolchain_copy/scripts/build-web-audioworklet.sh" --module-only \
    "$output_dir" >/dev/null 2>&1 || build_status=$?
  [[ $printed_status != 0 && $build_status != 0 && ! -e $mock_bin/cargo.log &&
    -z $(find "$output_dir" -mindepth 1 -print -quit) ]] || {
    echo "the builder accepted a $refused_toolchain browser artifact toolchain, or built" >&2
    exit 1
  }
done
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
echo "SDK artifact builder output-directory, single-directory, --module-only, --check-pin and --print-toolchain contract passed"
