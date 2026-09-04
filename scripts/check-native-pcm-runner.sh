#!/usr/bin/env bash
# The Issue-073/Issue-116 native PCM C-ABI runner's static seal, in two parts.
#
# Static and fixture seal for the native-only Issue-073 C-ABI runner.
# Static portability boundary for the Issue-116 native PCM publication adapters.
#
# One script, two subjects (formerly `check-native-pcm-runner-v1.sh` and
# `check-native-pcm-runner-portability-v1.sh`): one tool's contract split across two files for no
# reason the tool itself gives. Both gates now run from CI, not from the retired local sweep.
set -euo pipefail
root="${1:-.}"
cd "$root"

check_native_pcm_runner_v1() {
    fail() {
        printf 'native PCM runner V1 check failure: %s\n' "$1" >&2
        exit 1
    }

    tool=tools/native-pcm-runner
    fixture=fixtures/native-pcm-runner/v1
    [[ -f "$tool/Cargo.toml" && -f "$tool/src/lib.rs" && -f "$tool/src/main.rs" ]] \
        || fail 'tool surface is incomplete'
    [[ -f "$fixture/MANIFEST.tsv" && -f "$fixture/generate.py" ]] \
        || fail 'fixture surface is incomplete'

    python3 -I -B "$fixture/generate.py" --check \
        || fail 'independent fixture corpus or manifest drifted'

    [[ $(find "$fixture" -maxdepth 1 -type f -name 'riff-*.wav' | wc -l) -eq 4 ]] \
        || fail 'exact four-rate RIFF corpus missing'
    [[ -f "$fixture/rf64-48000.wav" ]] || fail 'RF64 corpus row missing'

    for dependency in capi session source sha2; do
        rg -q "^$dependency\\.workspace = true$" "$tool/Cargo.toml" \
            || fail "missing exact direct dependency $dependency"
    done

    for forbidden in engine graph graph-compiler; do
        ! rg -q "^$forbidden" "$tool/Cargo.toml" \
            || fail "forbidden product bypass dependency $forbidden"
    done

    for operation in \
        miso_engine_v1_engine_create miso_engine_v1_compile_session \
        miso_engine_v1_source_submit_planar_f32 miso_engine_v1_render_f32_planar \
        miso_engine_v1_plan_resources miso_engine_v1_plan_destroy \
        miso_engine_v1_session_destroy miso_engine_v1_engine_destroy; do
        rg -q "$operation" "$tool/src/lib.rs" || fail "missing frozen ABI operation $operation"
    done

    ! rg -n 'compile_session\(|GraphCompiler|PcmSourceRing|PreparedRenderPlan' "$tool/src" \
        | rg -v 'miso_engine_v1_compile_session' \
        || fail 'Rust product bypass is reachable from the tool'

    # The native decoder is cfg-excluded on Wasm, so no crate may make this tool a dependency. The
    # workspace membership row is intentionally the sole reference outside the package and lockfile.
    #
    # The inner `rg` is wrapped in its own `|| true`: without it, `rg` exiting 2 on a missing search
    # root (e.g. a hermetic test fixture with no sidecars/) would make the whole `if rg | rg; then`
    # pipeline read as "no violation" under `pipefail`, even when a real match was printed to stdout
    # by the roots that do exist.
    # A doc-comment mention of the tool by name (e.g. a fixture crate citing which tool configures
    # its fixtures) is documentation, not reachability -- the same distinction
    # check-conformance-boundaries.sh draws for the f64 oracle. This only started mattering once the
    # rename made the tool's real name (`native-pcm-runner`) the same bare spelling doc prose already
    # used informally.
    reachable="$({
        rg -n 'native-pcm-runner|native_pcm_runner' crates hosts tools sidecars \
            --glob Cargo.toml --glob '*.rs' || true
    } | rg -v '^tools/native-pcm-runner/' | rg -v ':[0-9]+:[[:space:]]*///?[[:space:]]' || true)"
    [[ -z "$reachable" ]] || {
        printf '%s\n' "$reachable" >&2
        fail 'native-only runner is reachable from another package or Wasm surface'
    }

    printf 'native PCM runner V1 check: ok\n'
}

check_native_pcm_runner_portability_v1() {
    fail() {
        printf 'native PCM runner portability check failure: %s\n' "$1" >&2
        exit 1
    }

    source=tools/native-pcm-runner/src/lib.rs
    contract=docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md
    [[ -f "$source" && -f "$contract" ]] || fail 'runner library or contract is missing'

    for required in \
        'cfg(any(target_os = "linux", target_os = "android"))' \
        'renameat2' \
        'RENAME_NOREPLACE' \
        'cfg(target_vendor = "apple")' \
        'renameatx_np' \
        'RENAME_EXCL' \
        'cfg(windows)' \
        'GetFileInformationByHandle' \
        'SetFileInformationByHandle' \
        'FileRenameInfo' \
        'FILE_RENAME_INFO_CLASS' \
        'replace_if_exists = 0' \
        'O_NOFOLLOW' \
        'FILE_FLAG_OPEN_REPARSE_POINT' \
        'platform.unsupported' \
        'trait PublicationAdapter' \
        'complete_publication'; do
        rg -Fq "$required" "$source" || fail "missing required boundary: $required"
    done

    rg -Fq 'exclusively owned for the complete runner' "$contract" \
        || fail 'exclusive output-directory precondition is missing'
    rg -Fq 'does not claim safety against a' "$contract" \
        || fail 'concurrent same-privilege mutation limitation is missing'
    ! rg -n 'identity-conditionally unlink|safe against (a )?concurrent same-privilege' "$contract" \
        || fail 'impossible concurrent directory-mutation guarantee was restored'

    ! rg -n 'fs::hard_link\(&self\.partial_path|std::fs::hard_link\(&self\.partial_path' "$source" \
        || fail 'generic pathname hard-link publication is forbidden'
    ! rg -n 'AT_EMPTY_PATH|/dev/fd/|FileLinkInfo|FILE_LINK_INFO_CLASS' "$source" \
        || fail 'stopped hard-link publication strategy was restored'
    ! rg -n 'replace_if_exists\s*=\s*(1|true)|REPLACE_IF_EXISTS|MOVEFILE_REPLACE_EXISTING' "$source" \
        || fail 'replace-enabled publication is forbidden'
    ! rg -n 'RENAME_REPLACE' "$source" \
        || fail 'replace-enabled rename publication is forbidden'
    ! rg -n 'FakeEntry::Owned\s*\|\s*FakeEntry::WrongPublished|WrongPublished\s*\|\s*FakeEntry::Owned' "$source" \
        || fail 'known-unowned fake publication cleanup is forbidden'

    python3 -I -B - "$source" <<'PY' || fail 'Unix imports are not explicitly guarded'
import pathlib
import sys

lines = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
for index, line in enumerate(lines):
    if "use std::os::unix" not in line:
        continue
    window = "\n".join(lines[max(0, index - 2):index])
    if "#[cfg(unix)]" not in window and "target_os" not in window and "target_vendor" not in window:
        raise SystemExit(1)
PY

    [[ $(rg -c 'FileIdentity::from_file\(&file\)' "$source") -ge 2 ]] \
        || fail 'held and post-publication handle identities are not both checked'
    rg -Fq 'if !adapter.partial_is_absent() || !adapter.final_is_owned()' "$source" \
        || fail 'post-publication identity check is missing'
    rg -Fq 'if self.path_is_owned(&self.partial_path)' "$source" \
        || fail 'owned-only partial cleanup under the exclusive contract is missing'
    rg -Fq 'if self.path_is_owned(&self.final_path)' "$source" \
        || fail 'owned-only final cleanup under the exclusive contract is missing'
    [[ $(rg -c 'O_NOFOLLOW' "$source") -eq 4 ]] \
        || fail 'both Linux/Android and Apple path identity checks must be no-follow'

    printf 'native PCM runner portability check: ok\n'
}

subject="${2:-all}"
case "$subject" in
    v1) check_native_pcm_runner_v1 ;;
    portability) check_native_pcm_runner_portability_v1 ;;
    all) check_native_pcm_runner_v1; check_native_pcm_runner_portability_v1 ;;
    *) printf 'usage: %s [root] [v1|portability|all]\n' "$0" >&2; exit 2 ;;
esac
