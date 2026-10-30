#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib/gate.sh"

process_policy_re='BigInt|new[[:space:]]|subarray|memory\.grow|memory\.buffer|WebAssembly|fetch\(|Promise|console\.|JSON\.|performance\.now|Date\.now'
# Issue #1332 S3: `memory\.buffer` is banned in the body. The module's memory is shared, so a growth
# gives `memory.buffer` a new object while every view over the older one stays valid: `process()`
# renders over the views it holds and never reads the buffer (reading it there once silenced a
# quantum per growth).
#
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
# Issue #1290d1: the meter body is the active meter set's, preallocated with its views in the
# message handler that committed the plan (a replacement plan's frame is sized for its strips), so
# the pinned spelling names the set.
process_policy_posts=(
  'this.port.postMessage(set.message);'
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

# Issue #1381a D6: the timers. The worklet never holds a timer, and the main-realm host holds exactly
# one `setInterval(` site, between the markers `SERVICE_TICK_BEGIN` and `SERVICE_TICK_END`.
# `setTimeout` stays banned in both files. Control work of the `single` mode runs in the worklet's
# message handler, so the worklet needs a message to run on; the single-mode service tick is that
# message, posted once per quantum. The same site runs the one other main-realm timer, the dispose
# deadline of both modes (#1332 S8), which fires once and is cleared. A second site, a site
# outside the markers or a timer in the worklet is red.
check_timer_policy() {
  local main=$1 worklet=$2
  local sites pinned
  if rg -q 'setTimeout|setInterval' "$worklet"; then return 1; fi
  if rg -q 'setTimeout' "$main"; then return 1; fi
  sites=$(rg -c 'setInterval' "$main" || true)
  pinned=$(sed -n '/SERVICE_TICK_BEGIN/,/SERVICE_TICK_END/p' "$main" | rg -c 'setInterval\(' || true)
  [[ "${sites:-0}" == 1 && "${pinned:-0}" == 1 ]]
}

# Issue #1332 S7 (D8): the browser capabilities of the three JavaScript files. The main-realm host
# holds exactly one `Worker(` site, `new Worker(workerUrl)`, which starts the control Worker, and
# still no `Atomics`, `SharedArrayBuffer` construction, hand-made growth, `WebSocket` or clock. The
# worklet holds no `Worker(`, `Atomics` (so no `Atomics.wait`), `SharedArrayBuffer` construction,
# hand-made growth or `WebSocket`. The control Worker holds no `fetch(`, `WebSocket`, hand-made
# growth, `Atomics.wait` or `Worker(`, no `setTimeout`, and exactly one `setInterval(` site, its
# service tick between `SERVICE_TICK_BEGIN` and `SERVICE_TICK_END`. `--self-test-capabilities`
# proves each rule can fire.
check_capability_policy() {
  local main=$1 worklet=$2 worker=$3
  local sites pinned
  if rg -q 'Atomics|new[[:space:]]+SharedArrayBuffer|memory\.grow|WebSocket|performance\.now|Date\.now' "$main"; then
    return 1
  fi
  [[ $(rg -c '\bWorker\(' "$main" || true) == 1 ]] || return 1
  [[ $(rg -c 'new Worker\(workerUrl\);' "$main" || true) == 1 ]] || return 1
  if rg -q 'Atomics|new[[:space:]]+SharedArrayBuffer|memory\.grow|WebSocket|\bWorker\(' "$worklet"; then
    return 1
  fi
  if rg -q 'fetch\(|WebSocket|memory\.grow|Atomics\.wait|\bWorker\(|setTimeout|importScripts' "$worker"; then
    return 1
  fi
  sites=$(rg -c 'setInterval' "$worker" || true)
  pinned=$(sed -n '/SERVICE_TICK_BEGIN/,/SERVICE_TICK_END/p' "$worker" | rg -c 'setInterval\(' || true)
  [[ "${sites:-0}" == 1 && "${pinned:-0}" == 1 ]]
}

if (($# == 1)) && [[ $1 == --self-test-capabilities ]]; then
  capability_dir=$(mktemp -d)
  trap 'rm -rf -- "$capability_dir"' EXIT
  main_ok="$capability_dir/main.js"
  worklet_ok="$capability_dir/worklet.js"
  worker_ok="$capability_dir/worker.js"
  printf '%s\n' 'function start(workerUrl) {' '  const worker = new Worker(workerUrl);' '}' >"$main_ok"
  printf '%s\n' 'class Processor { process() { return true; } }' >"$worklet_ok"
  printf '%s\n' 'class ControlWorker {' '  // SERVICE_TICK_BEGIN' \
    '  start() { this.tick = setInterval(() => this.run(), 3); }' '  // SERVICE_TICK_END' '}' >"$worker_ok"
  check_capability_policy "$main_ok" "$worklet_ok" "$worker_ok" || {
    echo "the valid capability fixtures were refused" >&2
    exit 1
  }
  mutate() {
    local name=$1 main=$2 worklet=$3 worker=$4
    if check_capability_policy "$main" "$worklet" "$worker"; then
      echo "capability mutation escaped policy: $name" >&2
      exit 1
    fi
  }
  with_line() {
    local source=$1 line=$2 target=$3
    { cat "$source"; printf '%s\n' "$line"; } >"$target"
  }
  with_line "$main_ok" 'function again(url) { return new Worker(url); }' "$capability_dir/m1.js"
  mutate "a second Worker site in the main realm" "$capability_dir/m1.js" "$worklet_ok" "$worker_ok"
  printf '%s\n' 'function none() {}' >"$capability_dir/m2.js"
  mutate "no Worker site in the main realm" "$capability_dir/m2.js" "$worklet_ok" "$worker_ok"
  with_line "$worklet_ok" 'function hold(word) { Atomics.wait(word, 0, 0); }' "$capability_dir/l1.js"
  mutate "Atomics.wait in the worklet" "$main_ok" "$capability_dir/l1.js" "$worker_ok"
  with_line "$worklet_ok" 'const worker = new Worker("x.js");' "$capability_dir/l2.js"
  mutate "a Worker in the worklet" "$main_ok" "$capability_dir/l2.js" "$worker_ok"
  for line in 'fetch("/x");' 'new WebSocket("ws://x");' 'memory.grow(1);' 'Atomics.wait(word, 0, 0);' \
    'setTimeout(() => {}, 1);' 'setInterval(() => {}, 9);' 'importScripts("x.js");'; do
    with_line "$worker_ok" "$line" "$capability_dir/w.js"
    mutate "control Worker: $line" "$main_ok" "$worklet_ok" "$capability_dir/w.js"
  done
  echo "web AudioWorklet capability-policy mutations passed"
  exit 0
fi

# Owner decision W4-D1 (#83): exactly one artifact ships and it is built with `+simd128`. The
# scalar artifact and the dual-artifact selection are gone, so this file no longer has a
# "scalar must contain no vector opcode" leg. No scalar wasm build is left at all: `lane` refuses
# wasm32 without `simd128` at compile time (issue #1062). Issue #1380 D4: atomic opcodes are
# allowed in the module (it is built with `+atomics` on a shared memory); the one atomic rule that
# matters on the render thread, no `memory.atomic.wait` in a render export's closure, is the
# call-graph gate's `--render-thread` (#1333 D4), below.
check_opcode_policy() {
  local simd_text=$1
  grep -q 'f32x4.mul' <<<"$simd_text" || return 1
  grep -q 'f32x4.add' <<<"$simd_text" || return 1
  grep -q 'f32x4.sub' <<<"$simd_text" || return 1
  ! grep -Eqi 'relaxed' <<<"$simd_text" || return 1
}

# Issue #1380 D4: the module's memory shape, read from `wasm-objdump -x`. Exactly one import, and it
# is `env.memory`: a shared memory whose limits are the ABI layout document's `memory.initialPages`
# and `memory.maximumPages`, the pages every instantiation site creates. The module defines no
# memory of its own and still exports the imported one as `memory`. A second import, an unshared
# memory or limits other than the layout's fail; `--self-test-module-shape` proves each one does.
check_module_shape() {
  local metadata=$1 initial=$2 maximum=$3
  local imports
  imports=$(awk '
    /^Import\[/ { in_imports = 1; next }
    in_imports && /^[A-Z][A-Za-z]+\[/ { in_imports = 0 }
    in_imports { print }
  ' <<<"$metadata")
  grep -qx 'Import\[1\]:' <<<"$metadata" || return 1
  [[ $(grep -c . <<<"$imports") == 1 ]] || return 1
  [[ $imports == " - memory[0] pages: initial=$initial max=$maximum shared <- env.memory" ]] ||
    return 1
  ! grep -q '^Memory\[' <<<"$metadata" || return 1
  grep -qx ' - memory\[0\] -> "memory"' <<<"$metadata"
}

if (($# == 1)) && [[ $1 == --self-test-module-shape ]]; then
  valid=$'Type[1]:\n - type[0] () -> nil\nImport[1]:\n - memory[0] pages: initial=20 max=8192 shared <- env.memory\nFunction[1]:\n - func[0] sig=0\nExport[1]:\n - memory[0] -> "memory"'
  check_module_shape "$valid" 20 8192 || {
    echo "the valid module shape was refused" >&2
    exit 1
  }
  mutate() {
    local name=$1 metadata=$2
    if check_module_shape "$metadata" 20 8192; then
      echo "module-shape mutation escaped policy: $name" >&2
      exit 1
    fi
  }
  mutate "a second import" "${valid/Import\[1\]:$'\n'/Import[2]:$'\n' - func[0] sig=0 <- env.now$'\n'}"
  mutate "an unshared memory" "${valid/ shared <- / <- }"
  mutate "limits other than the layout's" "${valid/max=8192/max=16384}"
  mutate "an initial size other than the layout's" "${valid/initial=20/initial=17}"
  # The import stays exactly as valid: only the "defines no memory of its own" rule can see this.
  mutate "a memory defined beside the import" "${valid/$'\n'Function\[1\]:/$'\n'Memory[1]:$'\n' - memory[1] pages: initial=1$'\n'Function[1]:}"
  mutate "the memory no longer exported" "${valid/ - memory\[0\] -> \"memory\"/ - func[0] -> \"start\"}"
  echo "web AudioWorklet module-shape mutations passed"
  exit 0
fi

if (($# == 1)) && [[ $1 == --self-test-opcodes ]]; then
  valid_simd=$'f32x4.mul\nf32x4.add\nf32x4.sub'
  check_opcode_policy "$valid_simd"
  for mutation in \
    $'f32x4.add\nf32x4.sub' \
    $'f32x4.mul\nf32x4.sub' \
    $'f32x4.mul\nf32x4.add' \
    $'f32x4.mul\nf32x4.add\nf32x4.sub\ni8x16.relaxed_swizzle'
  do
    if check_opcode_policy "$mutation"; then
      echo "missing/forbidden SIMD opcode mutation escaped policy" >&2
      exit 1
    fi
  done
  echo "web AudioWorklet opcode-policy mutations passed"
  exit 0
fi

if (($# == 1)) && [[ $1 == --self-test-timers ]]; then
  timer_dir=$(mktemp -d)
  trap 'rm -rf -- "$timer_dir"' EXIT
  main_ok="$timer_dir/main-ok.js"
  worklet_ok="$timer_dir/worklet-ok.js"
  printf '%s\n' \
    'class Host {' \
    '  // SERVICE_TICK_BEGIN' \
    '  #start() { this.tick = setInterval(() => this.post(), 3); }' \
    '  // SERVICE_TICK_END' \
    '  #stop() { clearInterval(this.tick); }' \
    '}' >"$main_ok"
  printf '%s\n' 'class Processor { process() { return true; } }' >"$worklet_ok"
  check_timer_policy "$main_ok" "$worklet_ok"
  mutate() {
    local name=$1 main=$2 worklet=$3
    if check_timer_policy "$main" "$worklet"; then
      echo "timer mutation escaped policy: $name" >&2
      exit 1
    fi
  }
  # A second site beside the pinned one.
  main_two="$timer_dir/main-two.js"
  { cat "$main_ok"; printf '%s\n' 'function poll() { setInterval(() => {}, 9); }'; } >"$main_two"
  mutate "a second main-realm site" "$main_two" "$worklet_ok"
  # The only site, but outside the markers.
  main_outside="$timer_dir/main-outside.js"
  printf '%s\n' \
    'class Host {' \
    '  // SERVICE_TICK_BEGIN' \
    '  // SERVICE_TICK_END' \
    '  #start() { this.tick = setInterval(() => this.post(), 3); }' \
    '}' >"$main_outside"
  mutate "a site outside the markers" "$main_outside" "$worklet_ok"
  # No site at all: the tick is gone.
  main_none="$timer_dir/main-none.js"
  printf '%s\n' 'class Host {}' >"$main_none"
  mutate "no service tick" "$main_none" "$worklet_ok"
  # A timeout in the main realm.
  main_timeout="$timer_dir/main-timeout.js"
  { cat "$main_ok"; printf '%s\n' 'function later() { setTimeout(() => {}, 9); }'; } >"$main_timeout"
  mutate "a main-realm timeout" "$main_timeout" "$worklet_ok"
  # A timer in the worklet.
  worklet_interval="$timer_dir/worklet-interval.js"
  printf '%s\n' 'class Processor { constructor() { setInterval(() => {}, 9); } }' >"$worklet_interval"
  mutate "a worklet interval" "$main_ok" "$worklet_interval"
  worklet_timeout="$timer_dir/worklet-timeout.js"
  printf '%s\n' 'class Processor { constructor() { setTimeout(() => {}, 9); } }' >"$worklet_timeout"
  mutate "a worklet timeout" "$main_ok" "$worklet_timeout"
  echo "web AudioWorklet timer-policy mutations passed"
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

# Issue #1044: `--without-metadata-regeneration` leaves out the one step below that re-runs the
# metadata generator, `parameter-metadata --check`. Only a caller whose directory was written by
# `build-web-audioworklet.sh` (which runs the same generator's `--write`) at this commit may pass
# it: there the check compares the generator with itself. CI's `artifact-gates` passes it for the
# `artifact` job's download, verified against that job's closure digest first, and
# scripts/check-ci-path-routing.py refuses the flag in any other job or workflow. It needs an
# explicit directory; every other form, the local no-argument one included, keeps the check.
regenerate_metadata=1
if (($# == 3)) && [[ $1 == --without-metadata-regeneration ]]; then
  regenerate_metadata=0
  shift
fi

# With no argument the gate builds the artifact it checks, so `bash scripts/check-web-audioworklet.sh`
# is runnable the same way every other `scripts/check-*.sh` is. CI keeps passing the directory it
# already built (#104 phase A: the no-argument form used to exit 2 and read as a red gate).
#
# Issue #1109: the shipped module carries no `name` section, and the call-graph gate below finds
# functions by name. So the gate takes the build's named twin beside the directory, reads the names
# from it, and first proves it is the shipped module plus its `name` section and nothing else
# (`strip-wasm-names.py check`), so what the call graph says of the twin's code holds for the
# shipped module's. Every other check here reads the shipped module itself.
if (($# == 0)); then
  self_built=$(mktemp -d)
  trap 'rm -rf -- "$self_built"' EXIT
  mkdir "$self_built/artifacts" "$self_built/named"
  bash "$(dirname "${BASH_SOURCE[0]}")/build-web-audioworklet.sh" \
    --named-twin "$self_built/named" "$self_built/artifacts" >&2
  set -- "$self_built/artifacts" "$self_built/named/miso-engine-v1-audio-worklet.simd128.named.wasm"
fi

if (($# != 2)) || [[ $1 == -* || $2 == -* ]]; then
  echo "usage: $0 [[--without-metadata-regeneration] ARTIFACT_DIRECTORY NAMED_TWIN]" >&2
  exit 2
fi

artifact_dir=$1
named=$2
[[ -d "$artifact_dir" && ! -L "$artifact_dir" ]] || {
  echo "artifact directory must be a non-symlink directory" >&2
  exit 2
}
[[ -f "$named" ]] || {
  echo "the named twin must be a file" >&2
  exit 2
}
command -v wasm-objdump >/dev/null || {
  echo "wasm-objdump is required" >&2
  exit 2
}

# Issue #243: the set was six files; #2111 added the stem identities' BLAKE3 module, a seventh, and
# #1332 S7 the control Worker of `worker` mode, an eighth. `miso-engine-v1-abi-layout.json` joined it because the boot
# ABI's bytes -- options offsets, result codes, the staging sequence -- were hand-written on the
# JavaScript side five times over and drifted; it is emitted by the same generator, from the same
# `offset_of!`s, and travels with the module it describes.
expected=$(printf '%s\n' \
  miso-engine-v1-abi-layout.json \
  miso-engine-v1-audio-worklet-host.d.ts \
  miso-engine-v1-audio-worklet-host.js \
  miso-engine-v1-audio-worklet.js \
  miso-engine-v1-audio-worklet.simd128.wasm \
  miso-engine-v1-blake3.simd128.wasm \
  miso-engine-v1-control-worker.js \
  miso-engine-v1-parameter-metadata.json)
actual=$(find "$artifact_dir" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)
[[ "$actual" == "$expected" ]] || {
  echo "artifact directory does not contain the exact eight frozen outputs" >&2
  diff -u <(printf '%s\n' "$expected") <(printf '%s\n' "$actual") >&2 || true
  exit 1
}

simd="$artifact_dir/miso-engine-v1-audio-worklet.simd128.wasm"
main_js="$artifact_dir/miso-engine-v1-audio-worklet-host.js"
worklet_js="$artifact_dir/miso-engine-v1-audio-worklet.js"
worker_js="$artifact_dir/miso-engine-v1-control-worker.js"
python3 -B "$(dirname "${BASH_SOURCE[0]}")/strip-wasm-names.py" check "$named" "$simd" || {
  echo "the named twin is not the shipped module plus its name section" >&2
  exit 1
}

expected_exports=$(printf '%s\n' \
  memory \
  __stack_pointer \
  __tls_align \
  __tls_base \
  __tls_size \
  __wasm_init_tls \
  miso_engine_web_v1_abi_version \
  miso_engine_web_v1_boot \
  miso_engine_web_v1_boot_command_ptr \
  miso_engine_web_v1_boot_diagnostic_bytes \
  miso_engine_web_v1_boot_diagnostic_ptr \
  miso_engine_web_v1_boot_options_ptr \
  miso_engine_web_v1_boot_result \
  miso_engine_web_v1_buffer_capacity \
  miso_engine_web_v1_buffer_ptr \
  miso_engine_web_v1_control_allocation_count \
  miso_engine_web_v1_control_command \
  miso_engine_web_v1_control_event \
  miso_engine_web_v1_control_frame_bytes \
  miso_engine_web_v1_control_request_ptr \
  miso_engine_web_v1_control_response_bytes \
  miso_engine_web_v1_control_response_ptr \
  miso_engine_web_v1_control_revision \
  miso_engine_web_v1_declare_discontinuity \
  miso_engine_web_v1_document_ptr \
  miso_engine_web_v1_dispose \
  miso_engine_web_v1_host_adopt \
  miso_engine_web_v1_host_release \
  miso_engine_web_v1_instance_bind \
  miso_engine_web_v1_instance_reserve \
  miso_engine_web_v1_meter_header_ptr \
  miso_engine_web_v1_meter_lease \
  miso_engine_web_v1_meter_poll \
  miso_engine_web_v1_plan_latency_ptr \
  miso_engine_web_v1_plan_latency_read \
  miso_engine_web_v1_render \
  miso_engine_web_v1_render_allocation_count \
  miso_engine_web_v1_render_diagnostic_bytes \
  miso_engine_web_v1_render_diagnostic_ptr \
  miso_engine_web_v1_replace_document_ptr \
  miso_engine_web_v1_resolved_source_ring_frames \
  miso_engine_web_v1_response_frame_close \
  miso_engine_web_v1_response_frame_ptr \
  miso_engine_web_v1_response_frame_reply_bytes \
  miso_engine_web_v1_response_frame_reply_ptr \
  miso_engine_web_v1_response_frame_submit \
  miso_engine_web_v1_resource_ptr \
  miso_engine_web_v1_service \
  miso_engine_web_v1_session_seek \
  miso_engine_web_v1_session_seek_at \
  miso_engine_web_v1_source_channels \
  miso_engine_web_v1_source_count \
  miso_engine_web_v1_source_fill \
  miso_engine_web_v1_source_frames \
  miso_engine_web_v1_source_id \
  miso_engine_web_v1_source_prepare_seek \
  miso_engine_web_v1_source_seek \
  miso_engine_web_v1_source_seek_at \
  miso_engine_web_v1_source_seek_report \
  miso_engine_web_v1_source_seek_report_ptr \
  miso_engine_web_v1_source_submit \
  miso_engine_web_v1_status_ptr \
  miso_engine_web_v1_successor_header_ptr \
  miso_engine_web_v1_watermark_ptr \
  miso_engine_web_v1_watermark_read | sort)

# Issue #1047: the shipped ABI layout document publishes exactly these exports, every one but
# `memory` (linear memory, not a call). It is read from the artifact, not from the generator's
# source, and the list above stays independent of the generator, so an export added to one of the
# module, this list or the generator without the others fails here or at the module check below.
layout_json="$artifact_dir/miso-engine-v1-abi-layout.json"
layout_exports=$(python3 -B -c '
import json, sys
exports = json.load(open(sys.argv[1], encoding="utf-8"))["exports"]
assert isinstance(exports, list) and all(isinstance(name, str) for name in exports)
print("\n".join(exports))
' "$layout_json" | sort) || {
  echo "cannot read the published export list from $layout_json" >&2
  exit 1
}
# #1332 S6: the linker's TLS and stack symbols (`__wasm_init_tls`, `__tls_size`, `__tls_align`,
# `__tls_base`, `__stack_pointer`) are exported for a second instance's start (#1331 recipe rule 2),
# and are no more the ABI's calls than `memory` is.
expected_functions=$(grep -vxE 'memory|__wasm_init_tls|__tls_size|__tls_align|__tls_base|__stack_pointer' <<<"$expected_exports")
[[ "$layout_exports" == "$expected_functions" ]] || {
  echo "the ABI layout document's published exports are not the frozen export set" >&2
  diff -u <(printf '%s\n' "$expected_functions") <(printf '%s\n' "$layout_exports") >&2 || true
  exit 1
}

layout_memory=$(python3 -B -c '
import json, sys
memory = json.load(open(sys.argv[1], encoding="utf-8"))["memory"]
print(memory["initialPages"], memory["maximumPages"])
' "$layout_json") || {
  echo "cannot read the shared memory's page counts from $layout_json" >&2
  exit 1
}
read -r initial_pages maximum_pages <<<"$layout_memory"
metadata=$(wasm-objdump -x "$simd")
exports=$(awk '
  /^Export\[/ { in_exports = 1; next }
  in_exports && /^[A-Z][A-Za-z]+\[/ { in_exports = 0 }
  in_exports && /-> "/ {
    sub(/^.*-> "/, ""); sub(/".*$/, ""); print
  }
' <<<"$metadata" | sort)
[[ "$exports" == "$expected_exports" ]] || {
  echo "unexpected Wasm exports in $simd" >&2
  diff -u <(printf '%s\n' "$expected_exports") <(printf '%s\n' "$exports") >&2 || true
  exit 1
}
check_module_shape "$metadata" "$initial_pages" "$maximum_pages" || {
  echo "the module must import exactly one shared env.memory of $initial_pages..$maximum_pages pages (the ABI layout's) and export it as memory: $simd" >&2
  awk '/^(Import|Memory)\[/ { p = 1; print; next } /^[A-Z][A-Za-z]+\[/ { p = 0 } p' <<<"$metadata" >&2
  exit 1
}

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
# #106 E5: the shipped artifact must still compute in the vector family. The kernel count is a
# ratchet, measured on this artifact. Raise it when a wave adds kernels; a count that drops below
# it is a regression to report, never a floor to lower.
#
# ## Issue #163 phase 0e: the raw f32x4 total became a per-kernel shape gate
#
# What this line used to say was `--simd-floor 3450`: at least 3450 `f32x4.{mul,add,sub}`
# instructions in the whole module. That number had already been re-derived once -- 4500 -> 3450,
# by issue #149's fast dB tier (#144 item 5), which replaced the exact-tier Cephes `log2`/`exp2`
# (degree 9 and 6, each with a range-reduction fold) with refitted minimax polynomials (degree 5
# and 4, no fold). Fewer polynomial terms is fewer vector instructions, by construction and on
# purpose, and the floor's own comment says a count below it is "a regression to report, never a
# floor to lower". A gate whose documented response to correct work is the one move it forbids is
# measuring the wrong thing.
#
# It is measuring the wrong thing in a specific way. A raw total conflates two events that a floor
# pass has to be able to tell apart:
#
#   * a kernel de-vectorises -- the failure #106 E5 exists to catch -- and the total falls;
#   * a kernel does the same work with fewer operations, and the total falls.
#
# The property actually wanted is per kernel and scale free: *this kernel still does its arithmetic
# in the vector family*. The analyser now asserts exactly that, over a roster of the named
# `process_bank`/`process_section`/`process_block` bodies the old comment enumerated. Each roster
# kernel must match exactly one arithmetic-carrying function, and its scalar `f32.{mul,add,sub,div}`
# count must stay inside `max(ceiling * vector, 8)`. The ceilings, their derivation from the counts
# measured on this artifact, and why four times the measured ratio is the right multiple are
# documented in `KERNEL_ROSTER` in `check-web-audioworklet-callgraph.py`.
#
# The arithmetic roster retains the compressor and parametric-EQ collapsed bodies as separate
# kernels. LLVM may outline the collapsed true-peak limiter's arithmetic into a distinct
# `LimiterCore<f32x4>::process_block_mono`, leaving `PreparedTruePeakLimiterBank<f32x4>::process_bank_mono`
# as a zero-arithmetic forwarding entry. The Python gate requires exactly one of each, a direct call,
# and zero counted vector/scalar arithmetic in the entry before applying the unchanged kernel shape
# budget. This keeps the outlined form held to the same vector contract without treating the wrapper
# as an arithmetic kernel or accepting indirect reachability.
#
# The *separation* of the two bodies is a requirement and not an accident, and it was measured: M2
# first wrote them as one function behind a `bool`, and the shipped dual path got slower on a
# console row that never collapses. `KERNEL_ROSTER`'s derivation note carries the numbers.
#
# Why the reshape is strictly stronger where it matters, and weaker only where it should be:
#
#   * **Stronger.** The old total was one number for the whole module, so one kernel could
#     scalarise completely while another grew and the gate stayed green. The roster is per kernel
#     and requires presence, so scalarising any single named kernel is red on its own. The
#     roster budget also catches *partial* scalarisation that the pre-existing "vector strictly
#     dominates scalar" rule waves through -- a kernel at 100 vector / 40 scalar dominates and is
#     still a third scalarised; self-test case (c1) is exactly that shape.
#   * **Weaker.** It no longer asserts any absolute instruction count. That is the point: phases 1
#     to 4 of #163 exist to lower those counts.
#
# Re-measured on the named twin at issue #1970 (built from `15f37828f`), for the record (vector /
# scalar; the M2-era count in brackets where it moved). These are the derivation's *input* and are
# deliberately not asserted. (The eight-lane multiband body, 2560/20, left the browser build with
# #1110.)
#
#   multiband dual          3993/0     multiband mono           2131/0   [3464/0, 1862/0 before #1367b]
#   transient-shaper dual    808/0     transient-shaper mono     407/0   [786/72 at M2]
#   gate-expander dual       265/0     gate-expander mono        134/0   [180/8 at M2]
#   true-peak-limiter dual  1139/0     true-peak-limiter mono    537/0   [448/0, 224/0]
#   compressor dual         1359/0     compressor mono           733/0   [267/0, 138/0]
#   parametric-eq dual      1352/1     parametric-eq mono        748/1   [168/0, 84/0]
#   dynamic-eq dual         1014/0     dynamic-eq mono           511/0   [626/0, 316/0 before #1969]
#   de-esser dual            893/0     de-esser mono             450/0   [561/0, 283/0 before #1969]
#   soft-clip f32x4           55/0     route-mix-ramp f32x4       22/0   [25/0 soft-clip]
#
# The dynamic-eq and de-esser rows were re-measured after #1969 (its detector link glide), on the
# named twin built from `e3e7cdab3`, and the multiband rows re-measured after #1367b (its detector
# link glide); the other rows are the #1970 build's.
#
# The parametric EQ's one scalar operation per body is a once-per-block `f32.sub` of two splats in
# `lane::silence_armable_holding`, inside the 0.10 ceiling's budget; the derivation table in the
# analyser names it. Each collapsed body sits at about half its dual sibling's vector count with
# no more scalar arithmetic than its sibling, which is what a correct one-plane variant looks like
# from here.
#
# The `--kernel-min` half of the ratchet -- the part that actually counts kernels -- rises from 8
# to 11, by the three this wave added. It never drops. The artifact carried thirteen then; since
# #1110 removed the eight-lane multiband body it carried twelve, a one-kernel slack. Issue #1220
# links the live route's indexed-ramp mix, `lane::kernels::route_mix_ramp_block<f32x4>` (22 vector
# / 0 scalar since #1452's undo 2; 21 between the #1220 amendment and that undo), into the render
# path: the artifact carries thirteen and the floor rises to that count, so the slack is spent and
# losing any one kernel is red.
#
# The pattern counts four-lane kernels only (`4wide6f32x4`; it was `4wide6f32x[48]`), so an
# eight-lane kernel can never pad the count. Eight lanes are refused outright instead: the
# `--kernel-shape` run also fails on any function name that spells an eight-lane instantiation
# (`EIGHT_LANE` in the analyser). The browser runs four lanes only and #1110 took eight out of the
# wasm build, so such a name is code no browser can execute coming back; the module at `7d030945`
# carried 29 and fails this rule.
#
# Issue #1333 D4: `--render-thread` adds three rules over the same closure of each of the three
# render-thread exports below -- no thread-local destructor registration, no `memory.atomic.wait`,
# and `call_indirect` sites exactly equal to the analyser's pinned `INDIRECT_SITES` table, each
# entry with its reason. The direct-call gate cannot see past an indirect call (the plan executor
# is a `Box<dyn PreparedPlanExecutor>`); the browser qualification's render-locked allocation count
# is the proof through it, and this table makes any new dynamic dispatch on the render thread a
# reviewed change.
callgraph="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)/check-web-audioworklet-callgraph.py"
named_disassembly=$(wasm-objdump -d "$named")
# Issue #2041: no render closure admits an unpinned trap owner. #1381a, #1577, #1581 and #1588b
# each pinned an engine SPSC ring's checked `slots[cursor]` index here (a `panic_bounds_check` that
# could not fire); `spsc.rs` now reads a slot without a bounds check under its unsafe charter, so a
# checked index inlined into the render closure again is red.
# Issue #2068: the one trap owner left is pinned at its site count, not admitted by name.
# `PreparedRenderPlan::render_inner` inlines `PlanarBufferMut::plane_mut`, whose `&mut
# self.storage[start..end]` is a checked slice index, in the executor-less silence branch: one
# `unreachable`. Every web plan carries an executor (both graph binding paths end in
# `prepare_with_executor`), so the branch is dead; it is core-owned (#84). A new checked index
# inlined into `render_inner` changes the count and is red.
# #1588b: `MeterFrameProducer::publish_frame`'s published-queue push refusal, unreachable because
# that queue has room for every frame the free queue does not hold, is forgotten rather than
# dropped, so no free joins the closure.
printf '%s
' "$named_disassembly" |
  python3 -B "$callgraph" --callgraph miso_engine_web_v1_render \
    --trap-owner '18PreparedRenderPlan12render_inner=1' --render-thread || exit 1
printf '%s
' "$named_disassembly" |
  python3 -B "$callgraph" --kernel-shape --kernel-pattern '4wide6f32x4' --kernel-min 13 ||
  exit 1

# #137 D2: the export `process()` calls beside the render export gets the same allocation gate.
# `miso_engine_web_v1_meter_poll` runs on the render thread after every rendered block; it may not
# reach an allocator or drop glue. Its closure admits no trap owner (#2041; `render_inner` is not
# in it). The allocation half of the gate is absolute: an allocator, a deallocator or drop glue in
# the closure fails. (#1589c deleted `command_submit`, the observation records' export, which this
# gate held too: an `Observe` edit is control work through `control_command`.)
printf '%s
' "$named_disassembly" |
  python3 -B "$callgraph" --callgraph miso_engine_web_v1_meter_poll \
    --render-thread || exit 1

# Issue #1332 S6: the worklet in `worker` mode adopts the render half at its start and releases it
# at teardown, and the Worker calls the same two exports to hand it out and take it back. Neither
# may reach an allocator, a deallocator or drop glue: the release and the adoption exchange
# preallocated boxes, and every refusal path forgets or redeposits what it holds. The worklet's
# bind and its render-diagnostic reads are held to the same rule. Issue #1332 S7 (root ruling
# R-S7-meters): so is `meter_header_ptr`, which the worker-mode `process()` calls after `render()`.
for export in miso_engine_web_v1_host_adopt miso_engine_web_v1_host_release \
  miso_engine_web_v1_instance_bind miso_engine_web_v1_render_diagnostic_ptr \
  miso_engine_web_v1_render_diagnostic_bytes miso_engine_web_v1_meter_header_ptr; do
  printf '%s
' "$named_disassembly" |
    python3 -B "$callgraph" --callgraph "$export" --render-thread || exit 1
done

# #137 D3 amends the browser-capability ban deliberately.
#
# Render telemetry needs a wall clock, and `WorkletGlobalScope` does not include `Performance` in
# the specification -- user agents disagree -- so the worklet probes once and falls back to
# `Date.now`, reporting the resolution it actually got. `currentTime` cannot substitute: it
# advances by exactly one quantum per block no matter how long the render took.
#
# The ban therefore becomes a pinned-site rule for the worklet's `renderClock()` helper, and stays
# a blanket ban for the main-realm host, for `Worker`/`Atomics` everywhere, and for every other
# line of the worklet. (`setTimeout` and `setInterval` are the pinned-site rule of
# `check_timer_policy`, issue #1381a D6.) The clock is never read inside the frozen
# `process()` policy body -- `process_policy_re` still bans both names there.
# Issue #1332 S7: the rules and their self-test are `check_capability_policy`'s.
check_capability_policy "$main_js" "$worklet_js" "$worker_js" || {
  echo "forbidden browser capability in the main-realm host, the worklet or the control Worker" >&2
  exit 1
}
# Issue #1381a D6: the timers are a pinned-site rule, not a blanket ban (see `check_timer_policy`).
check_timer_policy "$main_js" "$worklet_js" || {
  echo "a timer outside the pinned service tick, or no service tick" >&2
  exit 1
}
check_clock_policy "$worklet_js" || {
  echo "a clock is read outside the pinned renderClock() helper" >&2
  exit 1
}
if rg -n 'quantumFrames[[:space:]]*[:=][[:space:]]*128' "$main_js" "$worklet_js" "$worker_js"; then
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

# Issue #137 D4/E7: the shipped metadata is exactly what the registry produces right now, and it
# satisfies its own schema. `--check` regenerates and compares byte for byte, so a stale file, a
# hand edit, or an effect added to the registry without rebuilding all fail here. Issue #1044:
# left out under `--without-metadata-regeneration` (see the top of this file), where the directory's
# documents are that generator's output at this commit already; the schema gates below still run.
if ((regenerate_metadata)); then
  (
    cd "$(dirname "${BASH_SOURCE[0]}")/.."
    cargo run --locked --release -q -p parameter-metadata -- --check "$artifact_dir"
  ) >/dev/null || {
    echo "shipped parameter metadata is stale" >&2
    exit 1
  }
else
  echo "parameter-metadata --check left out: the caller's directory holds the generator's own output at this commit (#1044)"
fi
python3 -B "$(dirname "${BASH_SOURCE[0]}")/check-parameter-metadata-v1.py" \
  "$artifact_dir/miso-engine-v1-parameter-metadata.json" || exit 1

# Issue #243: the same `--check` above covers the ABI layout document (one generator, one
# transcription discipline, so an artifact directory can never hold a current metadata beside a
# stale layout). This is its independent schema gate.
python3 -B "$(dirname "${BASH_SOURCE[0]}")/check-abi-layout-v1.py" \
  "$artifact_dir/miso-engine-v1-abi-layout.json" || exit 1

# Issue #207: the session map's shape, across the five places that now spell it -- the Rust FFI's
# introspection exports (which decide each field's width), the frozen export list above, the
# worklet that calls them, the main-realm host's acknowledgement validator, and the `.d.ts` an SDK
# generates against. `--artifacts` additionally holds the SHIPPED js/.d.ts to the tree's, so a
# stale artifact fails here.
python3 -B "$(dirname "${BASH_SOURCE[0]}")/check-session-map-shape.py" \
  --artifacts "$artifact_dir" || exit 1

# Issue #338: the 17x JSON parse-transient pin is re-measured against the exact
# 1 MiB, 512-track x 4-effect accepted shape on the shipped wasm artifact. The refusal leg proves
# a one-byte-under budget dies before parsing without unbounded memory growth.
node "$(dirname "${BASH_SOURCE[0]}")/check-web-boot-budget.mjs" "$simd" || exit 1

echo "web AudioWorklet static/object checks passed"
