#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
if command -v node >/dev/null; then
  node "$repo_root/scripts/test-web-audioworklet.mjs"
elif command -v bun >/dev/null; then
  bun "$repo_root/scripts/test-web-audioworklet.mjs"
else
  echo "Node.js-compatible runtime required" >&2
  exit 2
fi

worklet="$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js"
"$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$worklet"
"$repo_root/scripts/check-web-audioworklet.sh" --self-test-opcodes
python3 -B "$repo_root/scripts/check-web-audioworklet-callgraph.py" --self-test
mutation_dir=$(mktemp -d)
cleanup() {
  rm -rf -- "$mutation_dir"
}
trap cleanup EXIT
webdriver_runner="$repo_root/scripts/web-audioworklet-browser-correctness.py"
python3 -B "$webdriver_runner" --self-test-webdriver-responses
mutated_webdriver="$mutation_dir/web-audioworklet-browser-correctness.py"
sed '/^    value = response\["value"\]$/a\
    if value is None and method != "DELETE":\
        raise RuntimeError("generic null rejection mutation")
' "$webdriver_runner" >"$mutated_webdriver"
if python3 -B "$mutated_webdriver" --self-test-webdriver-responses >/dev/null 2>&1; then
  echo "generic WebDriver null-rejection mutation escaped response tests" >&2
  exit 1
fi
echo "web AudioWorklet WebDriver null-response mutation passed"
for mutation in \
  'new Array(1);' \
  'this.port.postMessage({});' \
  'BigInt(1);' \
  'this.exports.memory.grow(1);'
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
console_mutations=(
  # A pinned post is renamed: the occurrence count still matches, the pinned line does not.
  's/this\.port\.postMessage(this\.meterMessage);/this.port.postMessage(this.telemetryMessage);/'
  # The telemetry post is dropped from the window: the count no longer matches.
  's/this\.port\.postMessage(frame);/frame.sequence += 0;/'
  # The lease guard is removed from the meter call site: the frame is posted unconditionally.
  's/if (this\.meterLease) this\.postMeterFrame();/this.postMeterFrame();/'
  # The clock is read inside the frozen render-callback body.
  's/const started = this\.telemetryLease ? this\.clock\.read() : 0;/const started = Date.now();/'
)
for mutation in "${console_mutations[@]}"; do
  mutated="$mutation_dir/worklet-console.js"
  sed "$mutation" "$worklet" >"$mutated"
  if diff -q "$worklet" "$mutated" >/dev/null; then
    echo "console policy mutation matched nothing: $mutation" >&2
    exit 1
  fi
  if "$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$mutated" >/dev/null 2>&1; then
    echo "console process-policy mutation escaped: $mutation" >&2
    exit 1
  fi
done
# A clock read anywhere outside `renderClock()` fails the pinned-site rule even when it is not in
# the render-callback body.
mutated="$mutation_dir/worklet-clock.js"
sed 's/^  bindConsole(init) {/  bindConsole(init) {\n    this.boot = Date.now();/' "$worklet" >"$mutated"
if diff -q "$worklet" "$mutated" >/dev/null; then
  echo "clock-site mutation matched nothing" >&2
  exit 1
fi
if "$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$mutated" >/dev/null 2>&1; then
  echo "clock read outside renderClock() escaped the pinned-site rule" >&2
  exit 1
fi
echo "web AudioWorklet console policy mutations passed"

# Issue #137 D4/E7: the metadata schema validator is proved to discriminate before it is trusted,
# and the emitted document is validated against it.
python3 -B "$repo_root/scripts/check-parameter-metadata-v1.py" --self-test
metadata="$mutation_dir/parameter-metadata.json"
(cd "$repo_root" && cargo run --locked -q -p miso-engine-parameter-metadata -- --print) >"$metadata"
python3 -B "$repo_root/scripts/check-parameter-metadata-v1.py" "$metadata" >/dev/null
# `--check` is byte equality against a freshly generated document, so a hand edit is a failure.
mkdir -p "$mutation_dir/metadata"
(cd "$repo_root" && cargo run --locked -q -p miso-engine-parameter-metadata -- --write "$mutation_dir/metadata") >/dev/null
(cd "$repo_root" && cargo run --locked -q -p miso-engine-parameter-metadata -- --check "$mutation_dir/metadata") >/dev/null
sed -i 's/"liveUpdatable": true/"liveUpdatable": false/' \
  "$mutation_dir/metadata/miso-engine-v2-parameter-metadata.json"
if (cd "$repo_root" && cargo run --locked -q -p miso-engine-parameter-metadata -- --check "$mutation_dir/metadata") >/dev/null 2>&1; then
  echo "a hand-edited metadata document escaped --check" >&2
  exit 1
fi
if python3 -B "$repo_root/scripts/check-parameter-metadata-v1.py" \
  "$mutation_dir/metadata/miso-engine-v2-parameter-metadata.json" >/dev/null 2>&1; then
  echo "a builtin that denies its own blockTarget update rate escaped the schema gate" >&2
  exit 1
fi
echo "web AudioWorklet parameter-metadata gates passed"

# Issues #143/#151: one command-reason vocabulary across six spellings, and the `.d.ts`'s
# `observe()` declaration held to the shipped implementation. The gate's own red mutations run
# first -- including a Rust reason bumped without the other five.
python3 -B "$repo_root/scripts/check-command-reason-vocabulary.py" --self-test
python3 -B "$repo_root/scripts/check-command-reason-vocabulary.py"
# The same mutation performed on disk rather than in memory, so the gate is proved against the
# real files it is pointed at in CI and not only against its own in-process copies.
vocabulary_dir="$mutation_dir/vocabulary"
mkdir -p "$vocabulary_dir/scripts" "$vocabulary_dir/hosts/miso-engine-host-web/src" \
  "$vocabulary_dir/hosts/miso-engine-host-web/web" \
  "$vocabulary_dir/tools/miso-engine-parameter-metadata/src"
cp "$repo_root/scripts/check-command-reason-vocabulary.py" \
  "$repo_root/scripts/check-parameter-metadata-v1.py" "$vocabulary_dir/scripts/"
cp "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js" \
  "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts" \
  "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js" \
  "$vocabulary_dir/hosts/miso-engine-host-web/web/"
cp "$repo_root/tools/miso-engine-parameter-metadata/src/lib.rs" \
  "$vocabulary_dir/tools/miso-engine-parameter-metadata/src/"
sed 's/^pub const COMMAND_REASON_OBSERVATION_UNBOUND: u32 = 11;/&\npub const COMMAND_REASON_FUTURE_TAP: u32 = 12;/' \
  "$repo_root/hosts/miso-engine-host-web/src/lib.rs" \
  >"$vocabulary_dir/hosts/miso-engine-host-web/src/lib.rs"
if diff -q "$repo_root/hosts/miso-engine-host-web/src/lib.rs" \
  "$vocabulary_dir/hosts/miso-engine-host-web/src/lib.rs" >/dev/null; then
  echo "the Rust reason-bump mutation matched nothing" >&2
  exit 1
fi
if python3 -B "$vocabulary_dir/scripts/check-command-reason-vocabulary.py" >/dev/null 2>&1; then
  echo "a Rust reason bumped without the other five spellings escaped the vocabulary gate" >&2
  exit 1
fi
echo "web AudioWorklet command-reason vocabulary gates passed"

# Issue #151: the shipped defect itself. Restoring the literal `<= 9` bound on the acknowledgement
# must take the whole hermetic suite red -- the refused subscription stops being a typed
# per-request rejection and becomes the host-wide sticky 255 that kept the app's GR meters dead.
mutated_host="$mutation_dir/host-reason-cap.js"
sed 's/validCommandReason(message\.reason)/validU32(message.reason) \&\& message.reason <= 9/' \
  "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js" \
  >"$mutated_host"
if diff -q "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js" \
  "$mutated_host" >/dev/null; then
  echo "the reason-cap mutation matched nothing" >&2
  exit 1
fi
if MISO_ENGINE_WEB_HOST_TEST_MODULE="$mutated_host" \
  node "$repo_root/scripts/test-web-audioworklet.mjs" >/dev/null 2>&1; then
  echo "the <= 9 command-reason cap escaped the observation refusal tests" >&2
  exit 1
fi
echo "web AudioWorklet reason-cap mutation passed"
mutated_utf8="$mutation_dir/worklet-utf8.js"
sed 's/(codePoint >>> 18) | 0xf0/(codePoint >>> 18) | 0xe0/' "$worklet" >"$mutated_utf8"
if MISO_ENGINE_WEB_WORKLET_TEST_MODULE="$mutated_utf8" \
  node "$repo_root/scripts/test-web-audioworklet.mjs" >/dev/null 2>&1; then
  echo "worklet UTF-8 four-byte lead mutation escaped byte-parity test" >&2
  exit 1
fi
echo "web AudioWorklet UTF-8 byte-parity mutation passed"
