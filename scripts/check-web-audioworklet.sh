#!/usr/bin/env bash
set -euo pipefail

process_policy_re='BigInt|new[[:space:]]|subarray|memory\.grow|WebAssembly|fetch\(|Promise|console\.|JSON\.|performance\.now|Date\.now'
# Issue #137 D2/D3 amend this gate deliberately.
#
# Before #137 the render callback posted nothing, so a blanket `postMessage` ban was exactly right.
# Meter frames and telemetry frames are posted from inside the callback by design -- one flat
# numeric payload per decimated window, which is the whole point of decimating -- so a blanket ban
# would now forbid the feature rather than a mistake.
#
# The ban is therefore replaced by a *pinned-occurrence* rule, which is strictly stronger than
# "postMessage is allowed here": the policy body must contain exactly these two calls, spelled
# exactly this way, each posting a body preallocated at construction, and nothing else. A third
# post, a post of a freshly built object, or a post that is not one of these two lines fails the
# gate. `new ` remains banned in the body, so the frames cannot be allocated here either.
process_policy_posts=(
  'this.port.postMessage(this.meterMessage);'
  'this.port.postMessage(frame);'
)
check_process_policy() {
  local source=$1
  local body code post count
  body=$(sed -n '/PROCESS_POLICY_BEGIN/,/PROCESS_POLICY_END/p' "$source")
  grep -q 'silence(outputs)' <<<"$body" || return 1
  grep -Eq "$process_policy_re" <<<"$body" && return 1
  # Comment lines are dropped before the occurrence count so prose about `postMessage` is not
  # mistaken for a call, and so a call cannot hide behind a comment marker either.
  code=$(grep -vE '^[[:space:]]*(//|/\*|\*)' <<<"$body")
  count=$(grep -c '\.postMessage(' <<<"$code" || true)
  [[ "$count" == "${#process_policy_posts[@]}" ]] || return 1
  for post in "${process_policy_posts[@]}"; do
    grep -qF "$post" <<<"$code" || return 1
  done
  # Both posts are lease-guarded at their one call site in `process()`.
  grep -qF 'if (this.meterLease) this.postMeterFrame();' <<<"$body" || return 1
  grep -qF 'if (this.telemetryLease) this.recordRenderTime(' <<<"$body" || return 1
  return 0
}

# #137 D3: the wall clock the render telemetry needs may be read at exactly one site.
#
# `WorkletGlobalScope` does not include `Performance` in the specification and user agents
# disagree, so the worklet probes once in `renderClock()` and falls back to `Date.now`, reporting
# the resolution it actually got. `currentTime` cannot substitute: it advances by exactly one
# quantum per block no matter how long the render took. Everywhere else in the worklet, and
# everywhere in the main-realm host, both names stay banned -- and `process_policy_re` keeps them
# banned inside the frozen render-callback body as well.
check_clock_policy() {
  local source=$1
  local body sites pinned
  body=$(sed -n '/^function renderClock() {/,/^}/p' "$source")
  [[ -n "$body" ]] || return 1
  sites=$(grep -Ec 'performance\.now|Date\.now' "$source" || true)
  pinned=$(grep -Ec 'performance\.now|Date\.now' <<<"$body" || true)
  [[ "$sites" == "$pinned" ]]
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
  check_clock_policy "${1#--source-policy=}" || {
    echo "a clock is read outside the pinned renderClock() helper" >&2
    exit 1
  }
  exit 0
fi

# With no argument the gate builds the artifact it checks, so `bash scripts/check-web-audioworklet.sh`
# is runnable the same way every other `scripts/check-*.sh` is. CI keeps passing the directory it
# already built (#104 phase A: the no-argument form used to exit 2 and read as a red gate).
if (($# == 0)); then
  self_built_artifacts=$(mktemp -d)
  trap 'rm -rf -- "$self_built_artifacts"' EXIT
  bash "$(dirname "${BASH_SOURCE[0]}")/build-web-audioworklet.sh" "$self_built_artifacts" >&2
  set -- "$self_built_artifacts"
fi

if (($# != 1)); then
  echo "usage: $0 [ARTIFACT_DIRECTORY]" >&2
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
  miso_engine_web_v1_command_report_ptr \
  miso_engine_web_v1_command_submit \
  miso_engine_web_v1_compile \
  miso_engine_web_v1_config_bytes \
  miso_engine_web_v1_config_new \
  miso_engine_web_v1_config_ptr \
  miso_engine_web_v1_console_track_count \
  miso_engine_web_v1_console_track_id \
  miso_engine_web_v1_dispose \
  miso_engine_web_v1_meter_lease \
  miso_engine_web_v1_meter_poll \
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

# #106 E1/E2: the render export's direct-call closure must reach no allocator, deallocator or drop
# glue, and must own no trap outside the one documented core site. This is the allocation gate for
# the wasm render path: there is no native audited-allocator tool for the browser host, and the
# shipped binary is the only thing that can witness the property.
#
# #106 E5: the shipped artifact must still contain the vector kernels. The floor and the kernel
# count are ratchets, measured on this artifact. Raise them when a wave adds kernels; a count that
# drops below them is a regression to report, never a floor to lower.
callgraph="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/check-web-audioworklet-callgraph.py"
printf '%s
' "$simd_disassembly" |
  python3 -B "$callgraph" --callgraph miso_engine_web_v1_render || exit 1
printf '%s
' "$simd_disassembly" |
  python3 -B "$callgraph" --simd-floor 4500 --kernel-pattern '4wide6f32x[48]' --kernel-min 8 ||
  exit 1

# #137 D1/D2: the two exports `process()` calls beside the render export get the same allocation
# gate. `miso_engine_web_v1_meter_poll` runs on the render thread after every rendered block, and
# `miso_engine_web_v1_command_submit` runs in `port.onmessage`, which the user agent dispatches
# between quanta on that same thread; neither may reach an allocator or drop glue.
# Each export's own symbol is the only admitted trap owner: the bounded SPSC endpoints inline
# their checked slot indexing into the caller, and that index is unreachable by the queue's own
# cursor invariant. The allocation half of the gate is not relaxed at all -- an allocator, a
# deallocator or drop glue in either closure still fails.
printf '%s
' "$simd_disassembly" |
  python3 -B "$callgraph" --callgraph miso_engine_web_v1_meter_poll \
    --trap-owner 22AudioWorkletEngineHost11poll_meters || exit 1
# `command_submit` runs in `port.onmessage`, not in `process()`, and its pan-law conversion
# reaches `miso-engine-math`'s vendored argument reduction, which is full of checked indices. The
# engine's rule for the control path is "never allocate on the render thread", so the allocation
# half is what this export is held to -- and it is held to it absolutely.
printf '%s
' "$simd_disassembly" |
  python3 -B "$callgraph" --callgraph miso_engine_web_v1_command_submit --allocation-only || exit 1

# #137 D3 amends the browser-capability ban deliberately.
#
# Render telemetry needs a wall clock, and `WorkletGlobalScope` does not include `Performance` in
# the specification -- user agents disagree -- so the worklet probes once and falls back to
# `Date.now`, reporting the resolution it actually got. `currentTime` cannot substitute: it
# advances by exactly one quantum per block no matter how long the render took.
#
# The ban therefore becomes a pinned-site rule for the worklet's `renderClock()` helper, and stays
# a blanket ban for the main-realm host, for `setTimeout`/`setInterval`/`Worker`/`Atomics`
# everywhere, and for every other line of the worklet. The clock is never read inside the frozen
# `process()` policy body -- `process_policy_re` still bans both names there.
if rg -n 'Atomics|new[[:space:]]+SharedArrayBuffer|memory\.grow|WebSocket|Worker\(|setTimeout|setInterval|performance\.now|Date\.now' "$main_js"; then
  echo "forbidden browser capability found in the main-realm host" >&2
  exit 1
fi
if rg -n 'Atomics|new[[:space:]]+SharedArrayBuffer|memory\.grow|WebSocket|Worker\(|setTimeout|setInterval' "$worklet_js"; then
  echo "forbidden browser capability found in the worklet" >&2
  exit 1
fi
check_clock_policy "$worklet_js" || {
  echo "a clock is read outside the pinned renderClock() helper" >&2
  exit 1
}
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
