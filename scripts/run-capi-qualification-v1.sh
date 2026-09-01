#!/usr/bin/env bash
# Execute the non-timed Issue-114 Linux qualification matrix exactly once in fresh staging.
set -euo pipefail

root="$(cd "${1:-.}" && pwd)"
cd "$root"
fixture=fixtures/capi-qualification/v1
stage=target/capi-qualification/v1
build="$stage/build"
logs="$stage/logs"
installed="$stage/installed"
consumer="$fixture/runtime_consumer.c"
header=crates/capi/include/miso_engine_v2.h

fail() {
    printf 'CAPI qualification V1 runner failure: %s\n' "$1" >&2
    exit 1
}

[[ ! -e "$stage" ]] || fail 'qualification staging already exists; stale artifacts are forbidden'
bash scripts/check-capi-qualification-v1.sh "$root" preflight

for tool in cc c++ ar nm readelf objdump python3 bash strace jq; do
    command -v "$tool" >/dev/null 2>&1 || fail "preflight-frozen host tool disappeared: $tool"
done
for tool in x86_64-w64-mingw32-gcc x86_64-w64-mingw32-g++ cl dumpbin xcrun otool lipo \
    aarch64-linux-android21-clang llvm-nm llvm-readobj llvm-objdump; do
    ! command -v "$tool" >/dev/null 2>&1 || fail "preflight-frozen absent tool appeared: $tool"
done
installed_targets="$(rustup target list --installed)"
for target in x86_64-unknown-linux-gnu x86_64-pc-windows-gnu x86_64-apple-darwin \
    aarch64-apple-darwin aarch64-apple-ios aarch64-linux-android; do
    rg -qx "$target" <<<"$installed_targets" || fail "preflight-frozen Rust target disappeared: $target"
done
for target in x86_64-pc-windows-msvc aarch64-apple-ios-sim; do
    ! rg -qx "$target" <<<"$installed_targets" || fail "preflight-frozen absent Rust target appeared: $target"
done

mkdir -p "$build" "$logs" "$installed/include" "$installed/lib" "$stage/bin"
cp "$header" "$installed/include/"

# This is the sole static/shared product build. Both crate types are emitted by one Cargo command
# into fresh qualification-owned staging; no pre-existing target artifact is consumed.
CARGO_TARGET_DIR="$build" cargo build --locked --release -p capi \
    >"$logs/capi-build.log" 2>&1
static_library="$build/release/libcapi.a"
shared_library="$build/release/libcapi.so"
[[ -f "$static_library" && -f "$shared_library" ]] || fail 'staged CAPI libraries are missing'
cp "$static_library" "$installed/lib/"
cp "$shared_library" "$installed/lib/"
sha256sum "$installed/include/miso_engine_v2.h" "$installed/lib/libcapi.a" \
    "$installed/lib/libcapi.so" >"$stage/ARTIFACTS.generated.sha256"

nm -g --defined-only "$installed/lib/libcapi.a" \
    | awk '{print $NF}' | rg '^miso_engine_v2_' | LC_ALL=C sort -u \
    >"$stage/static-symbols.txt"
nm -D --defined-only "$installed/lib/libcapi.so" \
    | awk '{print $NF}' | rg '^miso_engine_v2_' | LC_ALL=C sort -u \
    >"$stage/shared-symbols.txt"
nm -g "$installed/lib/libcapi.a" >"$stage/static-nm.txt"
nm -D "$installed/lib/libcapi.so" >"$stage/shared-nm.txt"
python3 -I -B scripts/check-capi-object-symbols-v1.py "$fixture/EXPECTED_SYMBOLS.txt" \
    "$stage/static-nm.txt" "$stage/shared-nm.txt"
diff -u "$fixture/EXPECTED_SYMBOLS.txt" "$stage/static-symbols.txt"
diff -u "$fixture/EXPECTED_SYMBOLS.txt" "$stage/shared-symbols.txt"
readelf --dyn-syms --wide "$installed/lib/libcapi.so" >"$stage/shared-readelf.txt"
objdump -p "$installed/lib/libcapi.so" >"$stage/shared-objdump.txt"

common=(-Wall -Wextra -Werror -pedantic -I"$installed/include")
native=(-ldl -lpthread -lm -lrt -lutil)
cc -std=c11 "${common[@]}" "$consumer" "$installed/lib/libcapi.a" \
    "${native[@]}" -o "$stage/bin/c11-static"
cc -std=c11 "${common[@]}" "$consumer" -L"$installed/lib" -lcapi \
    -Wl,-rpath,"$installed/lib" -o "$stage/bin/c11-shared"
c++ -x c++ -std=c++17 "${common[@]}" "$consumer" -x none \
    "$installed/lib/libcapi.a" \
    "${native[@]}" -o "$stage/bin/cpp17-static"
c++ -x c++ -std=c++17 "${common[@]}" "$consumer" -x none \
    -L"$installed/lib" -lcapi \
    -Wl,-rpath,"$installed/lib" -o "$stage/bin/cpp17-shared"
for binary in c11-static c11-shared cpp17-static cpp17-shared; do
    "$stage/bin/$binary" fixtures/session/v1/parametric-eq-nine-track.toml \
        >"$logs/$binary.log" 2>&1
done
sha256sum "$stage/bin/c11-static" "$stage/bin/c11-shared" "$stage/bin/cpp17-static" \
    "$stage/bin/cpp17-shared" >>"$stage/ARTIFACTS.generated.sha256"

CARGO_TARGET_DIR="$build" cargo test --locked -p capi -p protocol \
    >"$logs/capi-regressions.log" 2>&1
# The frozen runner corpus is executed once by its accepted test matrix. Do not rerun this command.
CARGO_TARGET_DIR="$build" cargo test --locked -p native-pcm-runner \
    >"$logs/runner-corpus.log" 2>&1
CARGO_TARGET_DIR="$build" cargo run --locked --release -p audit -- capi \
    >"$stage/capi-audit.json" 2>"$logs/capi-audit.stderr"
jq -e '.kind == "issue022_capi_render_audit" and .calls == 100000 and
    .render_errors == 0 and .stable_output_address == true and .total_violations == 0' \
    "$stage/capi-audit.json" >/dev/null

CARGO_TARGET_DIR="$build" cargo build --locked --release -p audit \
    >"$logs/realtime-build.log" 2>&1
trace_prefix="$stage/realtime-trace"
strace -ff -qq -o "$trace_prefix" "$build/release/audit" \
    realtime --blocks 1000000 --audit --trace-markers >"$stage/realtime-audit.json"
marker_file=""
while IFS= read -r candidate; do
    if rg -q 'MISO_ENGINE_RT_BEGIN' "$candidate" && rg -q 'MISO_ENGINE_RT_END' "$candidate"; then
        [[ -z "$marker_file" ]] || fail 'multiple realtime marker traces'
        marker_file="$candidate"
    fi
done < <(find "$stage" -maxdepth 1 -type f -name 'realtime-trace.*' | sort)
[[ -n "$marker_file" ]] || fail 'realtime marker trace missing'
unexpected="$(awk '
    /MISO_ENGINE_RT_BEGIN/ { inside = 1; next }
    /MISO_ENGINE_RT_END/ { inside = 0; found = 1; next }
    inside { print }
    END { if (!found) exit 2 }
' "$marker_file")"
[[ -z "$unexpected" ]] || fail 'syscall observed inside armed realtime interval'
jq -e '.kind == "realtime_audit" and .blocks == 1000000 and .swaps_accepted > 0 and
    .swaps_deferred > 0 and .total_violations == 0' "$stage/realtime-audit.json" >/dev/null

sha256sum "$logs/capi-regressions.log" "$logs/runner-corpus.log" \
    "$stage/capi-audit.json" "$stage/realtime-audit.json" >"$stage/QUALIFICATION.generated.sha256"
printf 'CAPI qualification V1 runner: ok (%s)\n' "$stage"
