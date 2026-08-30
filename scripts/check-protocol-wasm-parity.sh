#!/usr/bin/env bash
# Execute the same BTLV golden assertions in scalar and simd128 Wasm test binaries.
# The temporary `main` export is a wasm-interp entry point only; the protocol crate exports no C ABI.
#
# #274 -- why this gate reads a RETURNED VALUE and never trusts an exit status.
#
# Until this issue the gate ran `wasm-interp --run-all-exports "$artifact"` under `set -e`. It
# could not fail, for two independent reasons, and both are still true of wabt 1.0.34:
#
#   1. `--run-all-exports` silently SKIPS every export that takes parameters. The only function
#      export here is `main`, whose Wasm signature is the C `(argc: i32, argv: i32) -> i32`, so
#      nothing ran at all: the interpreter type-checked the module, printed nothing, exited 0.
#   2. Even when `main` IS invoked, a guest trap is reported as the *text* `=> error: ...` on
#      stdout and `wasm-interp` still exits 0. A panicking guest never reaches the shell.
#
# Cost of the inertness: the Wasm arm's own copy of the corpus digest sat at the pre-`b454b230`
# value while the native pin was re-pinned twice (`b454b230`, then #241's `04d291dd`), and both
# arms reported green. The digest now lives once, in `COMPLETE_SCHEMA_HASH`, asserted by the
# native `conformance_corpus` test and computed independently by the guest.
#
# So: invoke the export by name with its two arguments, and judge the interpreter's printed result
# line. The guest returns 0 when its computed digest equals the pin and 1 when it does not; any
# `error:` line (a decode refusal, a corpus-length change, any other trap) is refused as well, and
# so is silence. `--self-test` proves that discrimination for real -- it rebuilds the guest from
# mutated copies of the pin in a scratch tree and requires every one of them to come back red.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
readonly TARGET="wasm32-unknown-unknown"
readonly BINARY="miso_engine_protocol_wasm_golden"
# The one accepted interpreter result: `main` invoked with an empty argv, returning success.
readonly EXPECTED_RESULT="main(i32:0, i32:0) => i32:0"

self_test=0
case "${1-}" in
    "") ;;
    --self-test) self_test=1 ;;
    *) printf 'usage: %s [--self-test]\n' "$0" >&2; exit 2 ;;
esac
[[ "$#" -le 1 ]] || { printf 'usage: %s [--self-test]\n' "$0" >&2; exit 2; }

cd "$repository_root"
command -v wasm-interp >/dev/null 2>&1 || {
    printf 'wasm-interp is required for issue-005 Wasm parity\n' >&2
    exit 1
}

# The whole verdict, as one pure function of the interpreter's output and status. Everything the
# gate can conclude is concluded here, which is what `--self-test` drives.
judge() {
    local label="$1" status="$2" output="$3"
    if [[ "$status" != 0 ]]; then
        printf '%s: wasm-interp exited %s\n%s\n' "$label" "$status" "$output" >&2
        return 1
    fi
    if [[ -z "${output//[[:space:]]/}" ]]; then
        printf '%s: the interpreter ran nothing -- no export was invoked (issue #274)\n' "$label" >&2
        return 1
    fi
    if [[ "$output" != "$EXPECTED_RESULT" ]]; then
        printf '%s: expected exactly `%s`, got:\n%s\n' "$label" "$EXPECTED_RESULT" "$output" >&2
        printf '%s: a `=> i32:1` result means the guest computed a digest other than the pinned\n' \
            "$label" >&2
        printf '%s: COMPLETE_SCHEMA_HASH; an `error:` line means the guest trapped.\n' "$label" >&2
        return 1
    fi
    return 0
}

verify_artifact() {
    local label="$1" artifact="$2" output status=0
    output="$(wasm-interp "$artifact" -r main -a i32:0 -a i32:0 2>&1)" || status=$?
    judge "$label" "$status" "$output"
}

run_variant() {
    local name="$1"
    local feature="$2"
    local target_directory="target/ci/issue005-wasm-$name"
    local artifact="$target_directory/$TARGET/release/$BINARY.wasm"

    CARGO_TARGET_DIR="$target_directory" \
        RUSTFLAGS="-C target-feature=$feature -C link-arg=--export=main" \
        cargo build --locked --release --target "$TARGET" \
            -p miso-engine-protocol --bin "$BINARY"
  wasm-objdump -x "$artifact" | rg -- '-> "main"'
    verify_artifact "issue-005 Wasm golden parity ($name)" "$artifact"
}

# ---- red-mutation self-test ------------------------------------------------------------------
# Each row rebuilds the guest from a scratch copy of the tree with one edit applied, and requires
# the verdict to be RED. A mutation whose search text matches nothing is itself a failure: that is
# how a renamed constant would otherwise quietly retire a row.
PIN_FILE="crates/miso-engine-protocol/src/conformance.rs"
GUEST_FILE="crates/miso-engine-protocol/src/bin/$BINARY.rs"

self_test_run() {
    local scratch failures=0 output status
    scratch="$(mktemp -d)"
    # shellcheck disable=SC2064
    trap "rm -rf -- '$scratch'" RETURN
    tar -c --exclude=./target --exclude=./.git -C "$repository_root" . | tar -x -C "$scratch"

    local selftest_target="$repository_root/target/ci/issue005-wasm-selftest"
    local scratch_artifact="$selftest_target/$TARGET/release/$BINARY.wasm"

    build_scratch() {
        CARGO_TARGET_DIR="$selftest_target" \
            RUSTFLAGS="-C target-feature=-simd128 -C link-arg=--export=main" \
            cargo build --locked --release --target "$TARGET" \
                --manifest-path "$scratch/Cargo.toml" -p miso-engine-protocol --bin "$BINARY" \
                >/dev/null
    }

    mutate() {
        local relative="$1" search="$2" replace="$3"
        cp "$repository_root/$relative" "$scratch/$relative"
        grep -qF -- "$search" "$scratch/$relative" || {
            printf 'self-test FAILED: mutation matched nothing -- %s: %s\n' "$relative" "$search" >&2
            return 1
        }
        python3 - "$scratch/$relative" "$search" "$replace" <<'PY'
import sys
path, search, replace = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(path).read()
assert text.count(search) == 1, f"{path}: {search!r} must occur exactly once"
open(path, "w").write(text.replace(search, replace))
PY
    }

    restore() {
        cp "$repository_root/$PIN_FILE" "$scratch/$PIN_FILE"
        cp "$repository_root/$GUEST_FILE" "$scratch/$GUEST_FILE"
    }

    # Row 0 -- the control. An unmutated scratch copy must be GREEN, so a red row below is the
    # mutation talking and not the scratch harness.
    restore
    build_scratch
    if verify_artifact 'self-test control (unmutated scratch tree)' "$scratch_artifact"; then
        printf 'self-test: control is green\n'
    else
        printf 'self-test FAILED: the unmutated scratch tree is red\n' >&2
        failures=$((failures + 1))
    fi

    # Row 1 -- the exact historical inertness. The pre-#274 invocation of the CORRECT artifact
    # runs no export and prints nothing; the verdict function must refuse that silence.
    output="$(wasm-interp "$scratch_artifact" --run-all-exports 2>&1)" || status=$?
    status=${status-0}
    if judge 'self-test row 1' "$status" "$output" 2>/dev/null; then
        printf 'self-test FAILED: mutation escaped -- `--run-all-exports` (issue #274) judged green\n' >&2
        failures=$((failures + 1))
    else
        printf 'self-test: `--run-all-exports` (the #274 inert invocation) is refused\n'
    fi

    # Rows 2..4 -- real rebuilds. Each must be refused.
    local -a labels=(
        'the pre-#241 digest the Wasm arm actually carried'
        'a garbage digest'
        'a guest-side panic (corpus length assertion)'
    )
    local -a files=("$PIN_FILE" "$PIN_FILE" "$GUEST_FILE")
    local -a searches=(
        'COMPLETE_SCHEMA_HASH: u64 = 0xbdeb_b0f8_1c38_ec42;'
        'COMPLETE_SCHEMA_HASH: u64 = 0xbdeb_b0f8_1c38_ec42;'
        'assert_eq!(corpus.len(), 46);'
    )
    local -a replacements=(
        'COMPLETE_SCHEMA_HASH: u64 = 0x88a8_ee6a_6d9e_4acc;'
        'COMPLETE_SCHEMA_HASH: u64 = 0x0bad_0bad_0bad_0bad;'
        'assert_eq!(corpus.len(), 45);'
    )
    local index
    for index in "${!labels[@]}"; do
        restore
        if ! mutate "${files[$index]}" "${searches[$index]}" "${replacements[$index]}"; then
            failures=$((failures + 1))
            continue
        fi
        build_scratch
        if verify_artifact "self-test row $((index + 2))" "$scratch_artifact" 2>/dev/null; then
            printf 'self-test FAILED: mutation escaped -- %s\n' "${labels[$index]}" >&2
            failures=$((failures + 1))
        else
            printf 'self-test: refused %s\n' "${labels[$index]}"
        fi
    done

    restore
    if [[ "$failures" != 0 ]]; then
        printf 'issue-005 Wasm golden parity self-test FAILED (%s)\n' "$failures" >&2
        return 1
    fi
    printf 'issue-005 Wasm golden parity self-test passed (1 inert-invocation row, 3 red rebuilds)\n'
}

run_variant scalar -simd128
run_variant simd128 +simd128
printf 'issue-005 Wasm golden parity: ok (scalar + simd128)\n'

if [[ "$self_test" == 1 ]]; then
    self_test_run
fi
