#!/usr/bin/env bash
# Hermetic fake-child lifecycle tests; never invokes the real reference aggregator.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT

make_case() {
    case_root="$temp/$1"
    mkdir -p "$case_root/scripts" "$case_root/fixtures/effect-interchange/v1" "$case_root/bin"
    cp "$root/scripts/run-effect-interchange-reference-processes.sh" "$case_root/scripts/"
    printf 'accepted baseline\n' >"$case_root/accepted.txt"
    (cd "$case_root" && sha256sum accepted.txt) \
        >"$case_root/fixtures/effect-interchange/v1/ACCEPTED.sha256"
    : >"$case_root/scripts/effect-interchange-v1-reference.py"
    cat >"$case_root/bin/python3" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
index="${!#}"
printf '%s\n' "$index" >>"$FAKE_LOG"
if [[ "${FAKE_MUTATE_INDEX:-x}" == "$index" ]]; then
    printf 'changed\n' >"$FAKE_ROOT/accepted.txt"
fi
if [[ "${FAKE_MISSING_INDEX:-x}" == "$index" ]]; then
    exit 0
fi
if [[ "${FAKE_MALFORMED_INDEX:-x}" == "$index" ]]; then
    printf '{bad json}\n'
elif [[ "${FAKE_WRONG_INDEX:-x}" == "$index" ]]; then
    printf '{"combined_sha256":"%064d","descriptor_manifest_sha256":"%064d","issue":81,"package_manifest_sha256":"%064d","process_index":0,"schema_version":1,"state_manifest_sha256":"%064d"}\n' 0 0 0 0
else
    printf '{"combined_sha256":"%064d","descriptor_manifest_sha256":"%064d","issue":81,"package_manifest_sha256":"%064d","process_index":%s,"schema_version":1,"state_manifest_sha256":"%064d"}\n' 0 0 0 "$index" 0
fi
if [[ "${FAKE_DUPLICATE_INDEX:-x}" == "$index" ]]; then
    printf 'duplicate\n'
fi
if [[ "${FAKE_FAIL_INDEX:-x}" == "$index" ]]; then
    exit 17
fi
FAKE
    chmod +x "$case_root/bin/python3"
    export FAKE_ROOT="$case_root" FAKE_LOG="$case_root/invocations.txt"
    export PATH="$case_root/bin:$ORIGINAL_PATH"
}

ORIGINAL_PATH="$PATH"

make_case success
"$case_root/scripts/run-effect-interchange-reference-processes.sh" >/dev/null
out="$case_root/target/issue081/reference-processes"
[[ $(wc -l <"$FAKE_LOG") -eq 100 && $(wc -l <"$out/accepted.jsonl") -eq 100 ]]
diff -u <(seq 0 99) "$FAKE_LOG"
before="$(wc -l <"$FAKE_LOG")"
if "$case_root/scripts/run-effect-interchange-reference-processes.sh" >/dev/null 2>&1; then
    printf 'reference runner fake test: accepted overwrite escaped\n' >&2
    exit 1
fi
[[ $(wc -l <"$FAKE_LOG") -eq "$before" ]]

for mode in fail malformed missing duplicate wrong; do
    make_case "$mode"
    case "$mode" in
        fail) export FAKE_FAIL_INDEX=7 ;;
        malformed) export FAKE_MALFORMED_INDEX=8 ;;
        missing) export FAKE_MISSING_INDEX=9 ;;
        duplicate) export FAKE_DUPLICATE_INDEX=10 ;;
        wrong) export FAKE_WRONG_INDEX=11 ;;
    esac
    if "$case_root/scripts/run-effect-interchange-reference-processes.sh" >/dev/null 2>&1; then
        printf 'reference runner fake test: %s escaped\n' "$mode" >&2
        exit 1
    fi
    [[ $(wc -l <"$FAKE_LOG") -eq 100 ]]
    out="$case_root/target/issue081/reference-processes"
    [[ -f "$out/raw.jsonl" && -f "$out/status.tsv" && ! -e "$out/accepted.jsonl" ]]
    unset FAKE_FAIL_INDEX FAKE_MALFORMED_INDEX FAKE_MISSING_INDEX FAKE_DUPLICATE_INDEX FAKE_WRONG_INDEX
done

make_case mutation
export FAKE_MUTATE_INDEX=13
if "$case_root/scripts/run-effect-interchange-reference-processes.sh" >/dev/null 2>&1; then
    printf 'reference runner fake test: baseline mutation escaped\n' >&2
    exit 1
fi
[[ $(wc -l <"$FAKE_LOG") -eq 100 ]]
unset FAKE_MUTATE_INDEX

for kind in regular symlink hardlink; do
    make_case "existing-$kind"
    out="$case_root/target/issue081/reference-processes"
    mkdir -p "$out"
    case "$kind" in
        regular) : >"$out/raw.jsonl" ;;
        symlink) ln -s "$case_root/accepted.txt" "$out/raw.jsonl" ;;
        hardlink) ln "$case_root/accepted.txt" "$out/raw.jsonl" ;;
    esac
    if "$case_root/scripts/run-effect-interchange-reference-processes.sh" >/dev/null 2>&1; then
        printf 'reference runner fake test: %s output escaped\n' "$kind" >&2
        exit 1
    fi
    [[ ! -e "$FAKE_LOG" ]]
done

printf 'effect interchange reference runner fake lifecycle: ok\n'
