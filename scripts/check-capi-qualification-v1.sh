#!/usr/bin/env bash
# Static authority, availability, symbol, and result checker for Issue 114.
set -euo pipefail

root="$(cd "${1:-.}" && pwd)"
mode="${2:-final}"
cd "$root"

fail() {
    printf 'CAPI qualification V1 check failure: %s\n' "$1" >&2
    exit 1
}

fixture=fixtures/capi-qualification/v1
authorities="$fixture/AUTHORITIES.sha256"
symbols="$fixture/EXPECTED_SYMBOLS.txt"
toolchains="$fixture/TOOLCHAINS.tsv"
consumer="$fixture/runtime_consumer.c"
object_checker=scripts/check-capi-object-symbols-v1.py
evidence_checker=scripts/check-capi-qualification-evidence-v1.py

for path in "$authorities" "$symbols" "$toolchains" "$consumer" "$object_checker" \
    "$evidence_checker"; do
    [[ -f "$path" && ! -L "$path" ]] || fail "missing regular qualification input $path"
done
rg -Fq 'UNDEFINED_TYPES' "$object_checker" || fail 'object parser lacks import classification'
[[ "$mode" == preflight || "$mode" == final || "$mode" == preserved ]] ||
    fail "unknown checker mode $mode"

LC_ALL=C sort -c -k2,2 "$authorities" || fail 'authority manifest is not path-sorted'
# #313 rebuilt the complete Linux qualification against the prelaunch V1 ABI and refreshed all 26
# authority rows after the prefix-strip move. The manifest is live evidence again: every subject
# path must resolve and match the bytes that produced the accepted artifacts.
[[ $(wc -l <"$authorities" | tr -d ' ') -eq 26 ]] || fail 'authority membership changed'
if rg -v '^[0-9a-f]{64}  [^[:space:]]+$' "$authorities"; then
    fail 'authority manifest row shape'
fi
sha256sum --check --strict "$authorities" >/dev/null || fail 'qualification authority drift'

LC_ALL=C sort -c "$symbols" || fail 'expected symbols are not sorted'
[[ $(wc -l <"$symbols" | tr -d ' ') -eq 14 ]] || fail 'expected symbol count is not 14'
[[ $(sort -u "$symbols" | wc -l | tr -d ' ') -eq 14 ]] || fail 'duplicate expected symbol'
if rg -v '^miso_engine_v1_[a-z0-9_]+$' "$symbols"; then
    fail 'invalid expected symbol spelling'
fi

for operation in \
    miso_engine_v1_abi_version miso_engine_v1_query_capabilities \
    miso_engine_v1_engine_create miso_engine_v1_compile_session \
    miso_engine_v1_source_seek miso_engine_v1_source_submit_planar_f32 \
    miso_engine_v1_render_f32_planar miso_engine_v1_plan_resources \
    miso_engine_v1_submit_command miso_engine_v1_dequeue_event \
    miso_engine_v1_session_destroy miso_engine_v1_plan_destroy; do
    rg -q "$operation" "$consumer" || fail "consumer misses $operation"
done
for contract in 'std=c11' 'std=c++17' static shared; do
    rg -Fq "$contract" scripts/run-capi-qualification-v1.sh ||
        fail "qualification runner misses $contract"
done
if rg -n 'std::time|Instant::|SystemTime::|elapsed\(|criterion|cargo bench|run-.*benchmark' \
    "$fixture" scripts/run-capi-qualification-v1.sh 2>/dev/null; then
    fail 'timing or benchmark surface entered qualification'
fi

python3 -I -B - "$toolchains" <<'PY' || fail 'toolchain inventory is malformed'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
rows = [line.split("\t") for line in path.read_text(encoding="utf-8").splitlines()]
if rows[0] != ["schema_version", "1"]:
    raise SystemExit(1)
targets = {row[1]: row[2] for row in rows if row[0] == "target"}
expected = {
    "x86_64-unknown-linux-gnu": "PRESENT",
    "x86_64-pc-windows-gnu": "PRESENT",
    "x86_64-pc-windows-msvc": "ABSENT",
    "x86_64-apple-darwin": "PRESENT",
    "aarch64-apple-darwin": "PRESENT",
    "aarch64-apple-ios": "PRESENT",
    "aarch64-apple-ios-sim": "ABSENT",
    "aarch64-linux-android": "PRESENT",
}
if targets != expected:
    raise SystemExit(1)
tools = {(row[1], row[2]) for row in rows if row[0] == "tool"}
for name in ["cc", "c++", "ar", "nm", "readelf", "objdump", "python3", "bash", "strace", "jq"]:
    if (name, "PRESENT") not in tools:
        raise SystemExit(1)
for name in ["x86_64-w64-mingw32-gcc", "x86_64-w64-mingw32-g++", "cl", "dumpbin", "xcrun", "otool", "lipo", "aarch64-linux-android21-clang", "llvm-nm", "llvm-readobj", "llvm-objdump"]:
    if (name, "ABSENT") not in tools:
        raise SystemExit(1)
PY

if [[ "$mode" == preflight ]]; then
    printf 'CAPI qualification V1 preflight: ok\n'
    exit 0
fi

matrix="$fixture/MATRIX.tsv"
evidence="$fixture/EVIDENCE.sha256"
[[ -f "$matrix" && -f "$evidence" ]] || fail 'final matrix or evidence manifest missing'
LC_ALL=C sort -c -k2,2 "$evidence" || fail 'evidence manifest is not path-sorted'
semantic_mode=committed
[[ "$mode" == preserved ]] && semantic_mode=preserved
python3 -I -B "$evidence_checker" "$root" "$semantic_mode" ||
    fail 'qualification semantic evidence is invalid'
python3 -I -B - "$matrix" "$root" <<'PY' || fail 'qualification matrix is incomplete or dishonest'
import hashlib, pathlib, re, sys
root = pathlib.Path(sys.argv[2])
rows = [line.split("\t") for line in pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()]
if rows[0] != ["schema_version", "1"] or rows[1] != ["benchmark_invocations", "0"] or rows[2] != ["timing_invocations", "0"]:
    raise SystemExit(1)
data = {row[0]: row[1:] for row in rows[3:]}
expected = {
    "linux-c11-static": "PASS", "linux-c11-shared": "PASS",
    "linux-cpp17-static": "PASS", "linux-cpp17-shared": "PASS",
    "linux-static-symbols": "PASS", "linux-shared-symbols": "PASS",
    "linux-capi-regressions": "PASS", "linux-runner-corpus": "PASS",
    "linux-capi-render-audit": "PASS", "linux-million-swap-syscall-audit": "PASS",
    "windows-gnu": "UNAVAILABLE", "windows-msvc": "UNAVAILABLE",
    "macos-x86_64": "UNAVAILABLE", "macos-aarch64": "UNAVAILABLE",
    "ios-aarch64-device": "UNAVAILABLE", "ios-aarch64-simulator": "UNAVAILABLE",
    "android-aarch64": "UNAVAILABLE",
}
if set(data) != set(expected):
    raise SystemExit(1)
for name, status in expected.items():
    row = data[name]
    if len(row) != 4 or row[0] != status or not row[1] or not re.fullmatch(r"[0-9a-f]{64}", row[3]):
        raise SystemExit(1)
    if status == "UNAVAILABLE" and "absent" not in row[1].lower():
        raise SystemExit(1)
    evidence = root / row[2]
    if not evidence.is_file() or hashlib.sha256(evidence.read_bytes()).hexdigest() != row[3]:
        raise SystemExit(1)
if any(row[0] == "FAIL" for row in data.values()):
    raise SystemExit(1)
PY

if find . -path './target' -prune -o -type f \
    \( -name '*.o' -o -name '*.a' -o -name '*.so' -o -name '*.dylib' -o -name '*.dll' \
       -o -name '*.exe' -o -name '*.profraw' -o -name '*.profdata' \) -print | grep -q .; then
    fail 'generated qualification artifact exists under a source path'
fi

printf 'CAPI qualification V1 check: ok\n'
