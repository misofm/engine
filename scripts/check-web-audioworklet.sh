#!/usr/bin/env bash
set -euo pipefail

process_policy_re='postMessage|BigInt|new[[:space:]]|subarray|memory\.grow|WebAssembly|fetch\(|Promise|console\.|JSON\.'
check_process_policy() {
  local source=$1
  local body
  body=$(sed -n '/PROCESS_POLICY_BEGIN/,/PROCESS_POLICY_END/p' "$source")
  grep -q 'silence(outputs)' <<<"$body" || return 1
  ! grep -Eq "$process_policy_re" <<<"$body"
}

# Owner decision W4-D1 (#83): exactly one artifact ships and it is built with `+simd128`. The
# scalar artifact and the dual-artifact selection are gone, so this file no longer has a
# "scalar must contain no vector opcode" leg; the wasm-scalar *cargo check* stays in CI because
# `miso-engine-lane`'s scalar wasm path is still gated, it is just not shipped.
check_opcode_policy() {
  local simd_text=$1
  grep -q 'f32x4.mul' <<<"$simd_text" || return 1
  grep -q 'f32x4.add' <<<"$simd_text" || return 1
  grep -q 'f32x4.sub' <<<"$simd_text" || return 1
  ! grep -Eqi 'relaxed|atomic' <<<"$simd_text"
}

if (($# == 1)) && [[ $1 == --self-test-opcodes ]]; then
  valid_simd=$'f32x4.mul\nf32x4.add\nf32x4.sub'
  check_opcode_policy "$valid_simd"
  for mutation in \
    $'f32x4.add\nf32x4.sub' \
    $'f32x4.mul\nf32x4.sub' \
    $'f32x4.mul\nf32x4.add' \
    $'f32x4.mul\nf32x4.add\nf32x4.sub\ni8x16.relaxed_swizzle' \
    $'f32x4.mul\nf32x4.add\nf32x4.sub\ni32.atomic.load'
  do
    if check_opcode_policy "$mutation"; then
      echo "missing/forbidden SIMD opcode mutation escaped policy" >&2
      exit 1
    fi
  done
  echo "web AudioWorklet opcode-policy mutations passed"
  exit 0
fi

if (($# == 1)) && [[ $1 == --source-policy=* ]]; then
  check_process_policy "${1#--source-policy=}" || {
    echo "render callback or transitive helper violates the frozen static policy" >&2
    exit 1
  }
  exit 0
fi

if (($# != 1)); then
  echo "usage: $0 ARTIFACT_DIRECTORY" >&2
  exit 2
fi

artifact_dir=$1
[[ -d "$artifact_dir" && ! -L "$artifact_dir" ]] || {
  echo "artifact directory must be a non-symlink directory" >&2
  exit 2
}
command -v wasm-objdump >/dev/null || {
  echo "wasm-objdump is required" >&2
  exit 2
}

expected=$(printf '%s\n' \
  miso-engine-v2-audio-worklet-host.d.ts \
  miso-engine-v2-audio-worklet-host.js \
  miso-engine-v2-audio-worklet.js \
  miso-engine-v2-audio-worklet.simd128.wasm)
actual=$(find "$artifact_dir" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)
[[ "$actual" == "$expected" ]] || {
  echo "artifact directory does not contain the exact four frozen outputs" >&2
  diff -u <(printf '%s\n' "$expected") <(printf '%s\n' "$actual") >&2 || true
  exit 1
}

simd="$artifact_dir/miso-engine-v2-audio-worklet.simd128.wasm"
main_js="$artifact_dir/miso-engine-v2-audio-worklet-host.js"
worklet_js="$artifact_dir/miso-engine-v2-audio-worklet.js"

expected_exports=$(printf '%s\n' \
  memory \
  miso_engine_web_v1_abi_version \
  miso_engine_web_v1_buffer_capacity \
  miso_engine_web_v1_buffer_ptr \
  miso_engine_web_v1_compile \
  miso_engine_web_v1_config_bytes \
  miso_engine_web_v1_config_new \
  miso_engine_web_v1_config_ptr \
  miso_engine_web_v1_dispose \
  miso_engine_web_v1_prepare \
  miso_engine_web_v1_render \
  miso_engine_web_v1_resource_ptr \
  miso_engine_web_v1_source_seek \
  miso_engine_web_v1_source_submit \
  miso_engine_web_v1_status_ptr | sort)

for module in "$simd"; do
  metadata=$(wasm-objdump -x "$module")
  exports=$(awk '
    /^Export\[/ { in_exports = 1; next }
    in_exports && /^[A-Z][A-Za-z]+\[/ { in_exports = 0 }
    in_exports && /-> "/ {
      sub(/^.*-> "/, ""); sub(/".*$/, ""); print
    }
  ' <<<"$metadata" | sort)
  [[ "$exports" == "$expected_exports" ]] || {
    echo "unexpected Wasm exports in $module" >&2
    diff -u <(printf '%s\n' "$expected_exports") <(printf '%s\n' "$exports") >&2 || true
    exit 1
  }
  if grep -Eq '^Import\[[1-9]' <<<"$metadata"; then
    echo "Wasm imports are forbidden: $module" >&2
    exit 1
  fi
  memory_record=$(awk '
    /^Memory\[/ { in_memory = 1; next }
    in_memory && /^[A-Z][A-Za-z]+\[/ { in_memory = 0 }
    in_memory { print }
  ' <<<"$metadata")
  if grep -Eqi 'shared=yes|shared=true' <<<"$memory_record"; then
    echo "shared Wasm memory is forbidden: $module" >&2
    exit 1
  fi
  disassembly=$(wasm-objdump -d "$module")
  if grep -Eqi 'atomic' <<<"$disassembly"; then
    echo "atomics found: $module" >&2
    exit 1
  fi
done

simd_disassembly=$(wasm-objdump -d "$simd")
if ! check_opcode_policy "$simd_disassembly"; then
  echo "simd128 opcode contract failed" >&2
  exit 1
fi

if rg -n 'Atomics|new[[:space:]]+SharedArrayBuffer|memory\.grow|WebSocket|Worker\(|setTimeout|setInterval|performance\.now|Date\.now' "$main_js" "$worklet_js"; then
  echo "forbidden browser capability found" >&2
  exit 1
fi
if rg -n 'quantumFrames[[:space:]]*[:=][[:space:]]*128' "$main_js" "$worklet_js"; then
  echo "hardcoded 128-frame quantum found" >&2
  exit 1
fi
if ! check_process_policy "$worklet_js"; then
  echo "render callback violates the frozen static policy" >&2
  exit 1
fi
process_body=$(sed -n '/PROCESS_POLICY_BEGIN/,/PROCESS_POLICY_END/p' "$worklet_js")
grep -q 'miso_engine_web_v1_render(this.handle, actualFrames)' <<<"$process_body"
grep -q 'output\[0\]\.set(this.outputLeft)' <<<"$process_body"
grep -q 'output\[1\]\.set(this.outputRight)' <<<"$process_body"

echo "web AudioWorklet static/object checks passed"
