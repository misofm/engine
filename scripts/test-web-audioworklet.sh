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
mutated_utf8="$mutation_dir/worklet-utf8.js"
sed 's/(codePoint >>> 18) | 0xf0/(codePoint >>> 18) | 0xe0/' "$worklet" >"$mutated_utf8"
if MISO_ENGINE_WEB_WORKLET_TEST_MODULE="$mutated_utf8" \
  node "$repo_root/scripts/test-web-audioworklet.mjs" >/dev/null 2>&1; then
  echo "worklet UTF-8 four-byte lead mutation escaped byte-parity test" >&2
  exit 1
fi
echo "web AudioWorklet UTF-8 byte-parity mutation passed"
