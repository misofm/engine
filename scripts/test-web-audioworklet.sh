#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
# Issue #1421: a shell has one EXIT trap, so one cleanup removes every directory this script makes,
# each only if it was made (the bun branch never makes `host_test_dir`).
host_test_dir=
mutation_dir=
cleanup() {
  if [[ -n "${host_test_dir:-}" ]]; then rm -rf -- "$host_test_dir"; fi
  if [[ -n "${mutation_dir:-}" ]]; then rm -rf -- "$mutation_dir"; fi
}
trap cleanup EXIT
if command -v node >/dev/null; then
node "$repo_root/scripts/test-web-audioworklet.mjs"

# Issue #393: exercise the private allocator boundary through a transformed test module. The
# production host has no counter accessor or test hook; this copy starts at MAX_SAFE_INTEGER - 1.
host_test_dir=$(mktemp -d "${TMPDIR:-/tmp}/miso-engine-host.XXXXXX")
safe_host="$host_test_dir/safe-host.mjs"
unchecked_host="$host_test_dir/unchecked-host.mjs"
sed 's/#lastRequestId = 0;/#lastRequestId = Number.MAX_SAFE_INTEGER - 1;/' \
  "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.js" >"$safe_host"
MISO_ENGINE_WEB_HOST_TEST_MODULE="$safe_host" MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST=1 \
  node "$repo_root/scripts/test-web-audioworklet.mjs"
sed 's/if (this.#lastRequestId >= Number.MAX_SAFE_INTEGER) return null;//' "$safe_host" >"$unchecked_host"
if MISO_ENGINE_WEB_HOST_TEST_MODULE="$unchecked_host" MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST=1 \
  node "$repo_root/scripts/test-web-audioworklet.mjs" >/dev/null 2>&1; then
  echo "unchecked safe-integer allocator mutation escaped the red test" >&2
  exit 1
fi
echo "safe-integer allocator boundary and red mutation passed"
elif command -v bun >/dev/null; then
  bun "$repo_root/scripts/test-web-audioworklet.mjs"
else
  echo "Node.js-compatible runtime required" >&2
  exit 2
fi

worklet="$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet.js"
"$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$worklet"
"$repo_root/scripts/check-web-audioworklet.sh" --self-test-opcodes
"$repo_root/scripts/check-web-audioworklet.sh" --self-test-module-shape
"$repo_root/scripts/check-web-audioworklet.sh" --self-test-timers
"$repo_root/scripts/check-web-audioworklet.sh" --self-test-capabilities
python3 -B "$repo_root/scripts/check-web-audioworklet-callgraph.py" --self-test
mutation_dir=$(mktemp -d "${TMPDIR:-/tmp}/miso-engine-mutation.XXXXXX")

# Issue #288: mutate the real qualification caller module and run the same hermetic boot check.
# Keep the helper import's relative layout under the temporary directory so a missing module or
# an import-path mistake cannot masquerade as a contract failure. Each transform is scoped to its
# intended source site and must replace exactly one matching line.
qualification_source="$repo_root/hosts/host-web/qualification/qualification.js"
qualification_module_dir="$mutation_dir/qualification-module"
mkdir -p "$qualification_module_dir/hosts/host-web/qualification" \
  "$qualification_module_dir/hosts/host-web/web"
qualification_helper="$repo_root/hosts/host-web/web/hex-lower.js"
cp "$qualification_helper" "$qualification_module_dir/hosts/host-web/web/hex-lower.js"

qualification_mutations=(
  'outer document|renderCorpusSegment real host guard rejected (miso.error.v1 result=1)'
  'shared boot key|renderCorpusSegment real host guard rejected (miso.error.v1 result=1)'
  'diagnostic options|diagnoseReady initializer was refused (tag=miso.error.v1 result=1)'
)
for entry in "${qualification_mutations[@]}"; do
  name=${entry%%|*}
  expected=${entry#*|}
  mutated_qualification="$qualification_module_dir/hosts/host-web/qualification/qualification.js"
  qualification_log="$mutation_dir/qualification-${name// /-}.log"
  case "$name" in
    "outer document")
      match_count=$(awk '
        /^async function renderCorpusSegment\(/ { in_render = 1 }
        /^async function runCorpusQualification\(/ { in_render = 0 }
        in_render && $0 == "    document: sessionDocument," { count += 1 }
        END { print count + 0 }
      ' "$qualification_source")
      if [[ "$match_count" -ne 1 ]]; then
        echo "qualification mutation match count $match_count, wanted 1: $name" >&2
        exit 1
      fi
      awk '
        /^async function renderCorpusSegment\(/ { in_render = 1 }
        /^async function runCorpusQualification\(/ { in_render = 0 }
        in_render && $0 == "    document: sessionDocument," {
          $0 = "    sessionDocument: sessionDocument,"
          count += 1
        }
        { print }
        END { if (count != 1) exit 1 }
      ' "$qualification_source" >"$mutated_qualification"
      ;;
    "shared boot key")
      match_count=$(awk '$0 == "    sourceRingFrames," { count += 1 } END { print count + 0 }' \
        "$qualification_source")
      if [[ "$match_count" -ne 1 ]]; then
        echo "qualification mutation match count $match_count, wanted 1: $name" >&2
        exit 1
      fi
      awk '
        $0 == "    sourceRingFrames," {
          $0 = "    sourceRingFramez: sourceRingFrames,"
          count += 1
        }
        { print }
        END { if (count != 1) exit 1 }
      ' "$qualification_source" >"$mutated_qualification"
      ;;
    "diagnostic options")
      match_count=$(awk '$0 == "        options: bootOptions(QUANTUM_FRAMES)," { count += 1 } END { print count + 0 }' \
        "$qualification_source")
      if [[ "$match_count" -ne 1 ]]; then
        echo "qualification mutation match count $match_count, wanted 1: $name" >&2
        exit 1
      fi
      awk '
        $0 == "        options: bootOptions(QUANTUM_FRAMES)," {
          $0 = "        limits: bootOptions(QUANTUM_FRAMES),"
          count += 1
        }
        { print }
        END { if (count != 1) exit 1 }
      ' "$qualification_source" >"$mutated_qualification"
      ;;
    *)
      echo "unknown qualification mutation: $name" >&2
      exit 1
      ;;
  esac
  printf 'qualification mutation match count: %s=%s\n' "$name" "$match_count"
  if diff -q "$qualification_source" "$mutated_qualification" >/dev/null; then
    echo "qualification mutation matched nothing: $name" >&2
    exit 1
  fi
  if node "$repo_root/scripts/test-web-audioworklet.mjs" \
    --qualification-module "$mutated_qualification" >"$qualification_log" 2>&1; then
    echo "qualification mutation escaped the hermetic check: $name" >&2
    exit 1
  fi
  if ! grep -Fq "$expected" "$qualification_log"; then
    echo "qualification mutation failed for an unexpected reason: $name" >&2
    sed -n '1,12p' "$qualification_log" >&2
    exit 1
  fi
  echo "qualification source mutation passed: $name"
done
for mutation in \
  'new Array(1);' \
  'this.port.postMessage({});' \
  'BigInt(1);' \
  'this.exports.memory.grow(1);' \
  'if (this.exports.memory.buffer !== this.memoryBuffer) this.staleViewQuanta += 1;'
do
  mutated="$mutation_dir/worklet.js"
  awk -v mutation="$mutation" '
    { print }
    /silence\(outputs\) \{/ { print "    " mutation }
  ' "$worklet" >"$mutated"
  if "$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$mutated" >/dev/null 2>&1; then
    echo "transitive process-helper mutation escaped policy: $mutation" >&2
    exit 1
  fi
done
echo "web AudioWorklet transitive process-policy mutations passed"

# Issue #137 D2/D3: the two amended policy rules each get their own red mutation. The generic
# `this.port.postMessage({});` mutation above already covers "a third post appears"; these cover
# "a pinned post disappears" and "the clock leaves its one pinned site".
live_control_mutations=(
  # A pinned post is renamed: the occurrence count still matches, the pinned line does not.
  's/this\.port\.postMessage(set\.message);/this.port.postMessage(this.telemetryMessage);/'
  # The telemetry post is dropped from the window: the count no longer matches.
  's/this\.port\.postMessage(frame);/frame.sequence += 0;/'
  # The lease guard is removed from the meter call site: the frame is posted unconditionally.
  's/if (this\.meterLease) this\.postMeterFrame();/this.postMeterFrame();/'
  # The clock is read inside the frozen render-callback body.
  's/const started = this\.telemetryLease ? this\.clock\.read() : 0;/const started = Date.now();/'
)
for mutation in "${live_control_mutations[@]}"; do
  mutated="$mutation_dir/worklet-live-controls.js"
  sed "$mutation" "$worklet" >"$mutated"
  if diff -q "$worklet" "$mutated" >/dev/null; then
    echo "live-control policy mutation matched nothing: $mutation" >&2
    exit 1
  fi
  if "$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$mutated" >/dev/null 2>&1; then
    echo "live-control process-policy mutation escaped: $mutation" >&2
    exit 1
  fi
done
# A clock read anywhere outside `renderClock()` fails the pinned-site rule even when it is not in
# the render-callback body.
mutated="$mutation_dir/worklet-clock.js"
sed 's/^  bindLiveControls(init) {/  bindLiveControls(init) {\n    this.boot = Date.now();/' "$worklet" >"$mutated"
if diff -q "$worklet" "$mutated" >/dev/null; then
  echo "clock-site mutation matched nothing" >&2
  exit 1
fi
if "$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$mutated" >/dev/null 2>&1; then
  echo "clock read outside renderClock() escaped the pinned-site rule" >&2
  exit 1
fi
echo "web AudioWorklet live-control policy mutations passed"

# Issue #137 D4/E7: the metadata schema validator is proved to discriminate before it is trusted,
# and the emitted document is validated against it.
python3 -B "$repo_root/scripts/check-parameter-metadata-v1.py" --self-test
metadata="$mutation_dir/parameter-metadata.json"
(cd "$repo_root" && cargo run --locked -q -p parameter-metadata -- --print) >"$metadata"
python3 -B "$repo_root/scripts/check-parameter-metadata-v1.py" "$metadata" >/dev/null
# `--check` is byte equality against a freshly generated document, so a hand edit is a failure.
mkdir -p "$mutation_dir/metadata"
(cd "$repo_root" && cargo run --locked -q -p parameter-metadata -- --write "$mutation_dir/metadata") >/dev/null
(cd "$repo_root" && cargo run --locked -q -p parameter-metadata -- --check "$mutation_dir/metadata") >/dev/null
# The document is compact JSON (#382), so the edit does not depend on whitespace, and an edit that
# matches nothing fails here rather than reading as a caught mutation.
cp "$mutation_dir/metadata/miso-engine-v1-parameter-metadata.json" "$mutation_dir/metadata-before.json"
sed -i -E 's/"liveUpdatable": *true/"liveUpdatable":false/' \
  "$mutation_dir/metadata/miso-engine-v1-parameter-metadata.json"
if cmp -s "$mutation_dir/metadata-before.json" \
  "$mutation_dir/metadata/miso-engine-v1-parameter-metadata.json"; then
  echo "the liveUpdatable mutation changed nothing in the metadata document" >&2
  exit 1
fi
if (cd "$repo_root" && cargo run --locked -q -p parameter-metadata -- --check "$mutation_dir/metadata") >/dev/null 2>&1; then
  echo "a hand-edited metadata document escaped --check" >&2
  exit 1
fi
if python3 -B "$repo_root/scripts/check-parameter-metadata-v1.py" \
  "$mutation_dir/metadata/miso-engine-v1-parameter-metadata.json" >/dev/null 2>&1; then
  echo "a builtin that denies its own blockTarget update rate escaped the schema gate" >&2
  exit 1
fi
echo "web AudioWorklet parameter-metadata gates passed"

# Issue #207: one session-map shape across the Rust FFI, the frozen export list, the worklet, the
# host's acknowledgement validator and the `.d.ts`. Its own red mutations run first, then the two
# drift classes that matter are performed on disk: the host validator losing the source list (the
# #151 shape -- a map the validator does not expect fails the WHOLE host with a sticky 255) and the
# `.d.ts` narrowing a `u64` region to a JavaScript `number` (the shape an SDK generates against).
python3 -B "$repo_root/scripts/check-session-map-shape.py" --self-test
python3 -B "$repo_root/scripts/check-session-map-shape.py"
map_dir="$mutation_dir/session-map"
mkdir -p "$map_dir/scripts" "$map_dir/hosts/host-web/src" \
  "$map_dir/hosts/host-web/web"
cp "$repo_root/scripts/check-session-map-shape.py" "$repo_root/scripts/check-web-audioworklet.sh" \
  "$map_dir/scripts/"
cp "$repo_root/hosts/host-web/src/ffi.rs" "$map_dir/hosts/host-web/src/"
cp "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet.js" \
  "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts" \
  "$map_dir/hosts/host-web/web/"
session_map_mutations=(
  'miso-engine-v1-audio-worklet-host.js|s/"result", "sources", "metersAttached"/"result", "metersAttached"/'
  'miso-engine-v1-audio-worklet-host.d.ts|s/  readonly frames: bigint;/  readonly frames: number;/'
)
for entry in "${session_map_mutations[@]}"; do
  target=${entry%%|*}
  expression=${entry#*|}
  original="$repo_root/hosts/host-web/web/$target"
  mutated="$map_dir/hosts/host-web/web/$target"
  sed "$expression" "$original" >"$mutated"
  if diff -q "$original" "$mutated" >/dev/null; then
    echo "the session-map mutation matched nothing: $entry" >&2
    exit 1
  fi
  if python3 -B "$map_dir/scripts/check-session-map-shape.py" >/dev/null 2>&1; then
    echo "a session-map drift escaped the shape gate: $entry" >&2
    exit 1
  fi
  cp "$original" "$mutated"
done
echo "web AudioWorklet session-map shape gates passed"

mutated_utf8="$mutation_dir/worklet-utf8.js"
sed 's/(codePoint >>> 18) | 0xf0/(codePoint >>> 18) | 0xe0/' "$worklet" >"$mutated_utf8"
if MISO_ENGINE_WEB_WORKLET_TEST_MODULE="$mutated_utf8" \
  node "$repo_root/scripts/test-web-audioworklet.mjs" >/dev/null 2>&1; then
  echo "worklet UTF-8 four-byte lead mutation escaped byte-parity test" >&2
  exit 1
fi
echo "web AudioWorklet UTF-8 byte-parity mutation passed"
