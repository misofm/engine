#!/usr/bin/env bash
set -euo pipefail

root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
mkdir -p "$temp/fixtures/builtins"
cp -R "$root/fixtures/builtins/v1" "$temp/fixtures/builtins/"
for mutation in content manifest unlisted missing coverage; do
    copy="$temp/$mutation-root"
    mkdir -p "$copy"
    cp -R "$temp/fixtures" "$copy/fixtures"
    case "$mutation" in
        content) printf 'mutation\n' >>"$copy/fixtures/builtins/v1/cases.toml" ;;
        manifest) printf 'broken\n' >>"$copy/fixtures/builtins/v1/MANIFEST.tsv" ;;
        unlisted) printf 'unlisted\n' >"$copy/fixtures/builtins/v1/unlisted.txt" ;;
        missing) rm "$copy/fixtures/builtins/v1/pcm/identity-signed-zero.f32le" ;;
        coverage) sed -i '/id = "response-cascade-44100-1-fixed-0"/,+8d' "$copy/fixtures/builtins/v1/cases.toml" ;;
    esac
    if bash "$root/scripts/check-builtins-fixtures.sh" "$copy" >/dev/null 2>&1; then
        printf 'builtins fixture corruption escaped: %s\n' "$mutation" >&2
        exit 1
    fi
done

# #1026 (F16): the audit's accepted manifest digest is the corpus's one hash consumer. A copy with
# the real constant passes the whole script, so the stale and missing cases fail for their own
# reason. The stub audit binary accepts the real corpus and refuses the script's corrupted canary,
# which keeps the positive control hermetic (no cargo build).
consumer_root="$temp/consumer-root"
mkdir -p "$consumer_root/tools/audit/src"
cp -R "$temp/fixtures" "$consumer_root/fixtures"
cp "$root/tools/audit/src/builtins_graph.rs" "$consumer_root/tools/audit/src/"
stub_audit="$temp/stub-audit"
printf '#!/usr/bin/env bash\n[[ "$1 $2" == "fixture-builtins --check" && "$3" == */fixtures/builtins/v1 ]]\n' \
    >"$stub_audit"
chmod +x "$stub_audit"
bash "$root/scripts/check-builtins-fixtures.sh" "$consumer_root" "$stub_audit" >/dev/null || {
    printf 'builtins fixture check rejected the real manifest consumer\n' >&2
    exit 1
}
consumer_source="$consumer_root/tools/audit/src/builtins_graph.rs"
for mutation in stale missing; do
    cp "$root/tools/audit/src/builtins_graph.rs" "$consumer_source"
    case "$mutation" in
        stale) sed -i '/const ACCEPTED_MANIFEST_SHA256: &str =/,/;/s/"[0-9a-f]\{64\}"/"'"$(printf '0%.0s' {1..64})"'"/' "$consumer_source" ;;
        missing) sed -i 's/const ACCEPTED_MANIFEST_SHA256: &str =/const RENAMED_MANIFEST_SHA256: \&str =/' "$consumer_source" ;;
    esac
    ! cmp -s "$root/tools/audit/src/builtins_graph.rs" "$consumer_source" ||
        { printf 'builtins manifest consumer mutation did not apply: %s\n' "$mutation" >&2; exit 1; }
    if output="$(bash "$root/scripts/check-builtins-fixtures.sh" "$consumer_root" "$stub_audit" 2>&1)"; then
        printf 'builtins manifest consumer mutation escaped: %s\n' "$mutation" >&2
        exit 1
    fi
    [[ "$output" == *'stale or missing builtins manifest consumer: tools/audit/src/builtins_graph.rs'* ]] || {
        printf 'builtins manifest consumer mutation failed for another reason: %s\n%s\n' "$mutation" "$output" >&2
        exit 1
    }
done
printf 'builtins fixture mutations: ok\n'
