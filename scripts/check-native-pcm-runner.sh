#!/usr/bin/env bash
set -euo pipefail
root="${1:-.}"; cd "$root"

check_native_pcm_runner_v1() {
    fail() { printf 'native PCM runner V1 check failure: %s\n' "$1" >&2; exit 1; }
    tool=tools/native-pcm-runner; fixture=fixtures/native-pcm-runner/v1
    [[ -f "$tool/Cargo.toml" && -f "$tool/src/lib.rs" && -f "$tool/src/main.rs" ]] || fail 'tool surface is incomplete'
    [[ -f "$fixture/MANIFEST.tsv" && -f "$fixture/generate.py" ]] || fail 'fixture surface is incomplete'
    if python3 -I -B "$fixture/generate.py" --check; then :; else status=$?; fail "independent fixture corpus or manifest drifted (python3 status $status)"; fi
    if scan_tmp=$(mktemp -d); then :; else status=$?; fail "scratch creation failed (mktemp status $status)"; fi
    trap 'rm -rf -- "$scan_tmp"' RETURN
    if find "$fixture" -maxdepth 1 -type f -name 'riff-*.wav' >"$scan_tmp/riff" 2>"$scan_tmp/riff.err"; then find_status=0; else find_status=$?; fi
    if wc -l <"$scan_tmp/riff" >"$scan_tmp/count" 2>"$scan_tmp/count.err"; then count_status=0; else count_status=$?; fi
    [[ "$find_status" == 0 ]] || fail "RIFF corpus discovery failed (find status $find_status): $(<"$scan_tmp/riff.err")"
    [[ "$count_status" == 0 ]] || fail "RIFF corpus count failed (wc status $count_status): $(<"$scan_tmp/count.err")"
    riff_count=$(<"$scan_tmp/count"); [[ "$riff_count" =~ ^[[:space:]]*4[[:space:]]*$ ]] || fail 'exact four-rate RIFF corpus missing'
    [[ -f "$fixture/rf64-48000.wav" ]] || fail 'RF64 corpus row missing'
    for dependency in capi session source sha2; do
        if rg -q "^$dependency\\.workspace = true$" "$tool/Cargo.toml"; then :; else status=$?; fail "missing exact direct dependency $dependency (rg status $status)"; fi
    done
    for forbidden in engine graph graph-compiler; do
        if rg -q "^$forbidden" "$tool/Cargo.toml"; then fail "forbidden product bypass dependency $forbidden"; else status=$?; [[ "$status" == 1 ]] || fail "forbidden dependency scan failed (rg status $status)"; fi
    done
    for operation in miso_engine_v1_engine_create miso_engine_v1_compile_session miso_engine_v1_source_submit_planar_f32 miso_engine_v1_render_f32_planar miso_engine_v1_plan_resources miso_engine_v1_plan_destroy miso_engine_v1_session_destroy miso_engine_v1_engine_destroy; do
        if rg -q "$operation" "$tool/src/lib.rs"; then :; else status=$?; fail "missing frozen ABI operation $operation (rg status $status)"; fi
    done
    if rg -n 'compile_session\(|GraphCompiler|PcmSourceRing|PreparedRenderPlan' "$tool/src" >"$scan_tmp/bypass-source" 2>"$scan_tmp/bypass-source.err"; then bypass_source_status=0; else bypass_source_status=$?; fi
    [[ "$bypass_source_status" == 0 || "$bypass_source_status" == 1 ]] || fail "Rust product bypass source scan failed (rg status $bypass_source_status): $(<"$scan_tmp/bypass-source.err")"
    if rg -v 'miso_engine_v1_compile_session' "$scan_tmp/bypass-source" >"$scan_tmp/bypass" 2>"$scan_tmp/bypass.err"; then bypass_filter_status=0; else bypass_filter_status=$?; fi
    [[ "$bypass_filter_status" == 0 || "$bypass_filter_status" == 1 ]] || fail "Rust product bypass exclusion failed (rg status $bypass_filter_status): $(<"$scan_tmp/bypass.err")"
    [[ ! -s "$scan_tmp/bypass" ]] || { cat "$scan_tmp/bypass" >&2; fail 'Rust product bypass is reachable from the tool'; }
    for required_root in crates hosts tools sidecars; do [[ -d "$required_root" ]] || fail "native runner reachability root is missing: $required_root"; done
    if rg -n 'native-pcm-runner|native_pcm_runner' crates hosts tools sidecars --glob Cargo.toml --glob '*.rs' >"$scan_tmp/reachable" 2>"$scan_tmp/reachable.err"; then source_status=0; else source_status=$?; fi
    [[ "$source_status" == 0 || "$source_status" == 1 ]] || fail "native runner reachability scan failed (rg status $source_status): $(<"$scan_tmp/reachable.err")"
    if rg -v '^tools/native-pcm-runner/' "$scan_tmp/reachable" >"$scan_tmp/owned"; then own_status=0; else own_status=$?; fi
    [[ "$own_status" == 0 || "$own_status" == 1 ]] || fail "native runner ownership filter failed (rg status $own_status): $(<"$scan_tmp/owned")"
    if rg -v ':[0-9]+:[[:space:]]*///?[[:space:]]' "$scan_tmp/owned" >"$scan_tmp/filtered"; then comment_status=0; else comment_status=$?; fi
    [[ "$comment_status" == 0 || "$comment_status" == 1 ]] || fail "native runner comment filter failed (rg status $comment_status): $(<"$scan_tmp/filtered")"
    reachable=$(<"$scan_tmp/filtered")
    [[ -z "$reachable" ]] || { printf '%s\n' "$reachable" >&2; fail 'native-only runner is reachable from another package or Wasm surface'; }
    printf 'native PCM runner V1 check: ok\n'
}

check_native_pcm_runner_portability_v1() {
    fail() { printf 'native PCM runner portability check failure: %s\n' "$1" >&2; exit 1; }
    source=tools/native-pcm-runner/src/lib.rs; contract=docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md
    [[ -f "$source" && -f "$contract" ]] || fail 'runner library or contract is missing'
    if portability_tmp=$(mktemp -d); then :; else status=$?; fail "scratch creation failed (mktemp status $status)"; fi
    trap 'rm -rf -- "$portability_tmp"' RETURN
    for required in 'cfg(any(target_os = "linux", target_os = "android"))' renameat2 RENAME_NOREPLACE 'cfg(target_vendor = "apple")' renameatx_np RENAME_EXCL 'cfg(windows)' GetFileInformationByHandle SetFileInformationByHandle FileRenameInfo FILE_RENAME_INFO_CLASS 'replace_if_exists = 0' O_NOFOLLOW FILE_FLAG_OPEN_REPARSE_POINT platform.unsupported 'trait PublicationAdapter' complete_publication; do
        if rg -Fq "$required" "$source"; then :; else status=$?; fail "missing required boundary: $required (rg status $status)"; fi
    done
    if rg -Fq 'exclusively owned for the complete runner' "$contract"; then :; else status=$?; fail "exclusive output-directory precondition is missing (rg status $status)"; fi
    if rg -Fq 'does not claim safety against a' "$contract"; then :; else status=$?; fail "concurrent same-privilege mutation limitation is missing (rg status $status)"; fi
    if rg -n 'identity-conditionally unlink|safe against (a )?concurrent same-privilege' "$contract"; then fail 'impossible concurrent directory-mutation guarantee was restored'; else status=$?; [[ "$status" == 1 ]] || fail "contract prohibition scan failed (rg status $status)"; fi
    for pattern in 'fs::hard_link\(&self\.partial_path|std::fs::hard_link\(&self\.partial_path' 'AT_EMPTY_PATH|/dev/fd/|FileLinkInfo|FILE_LINK_INFO_CLASS' 'replace_if_exists\s*=\s*(1|true)|REPLACE_IF_EXISTS|MOVEFILE_REPLACE_EXISTING' 'RENAME_REPLACE' 'FakeEntry::Owned\s*\|\s*FakeEntry::WrongPublished|WrongPublished\s*\|\s*FakeEntry::Owned'; do
        if rg -n "$pattern" "$source"; then fail 'forbidden publication or cleanup strategy was restored'; else status=$?; [[ "$status" == 1 ]] || fail "portability forbidden scan failed (rg status $status)"; fi
    done
    if python3 -I -B - "$source" <<'PY'
import pathlib, sys
lines = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
for index, line in enumerate(lines):
    if "use std::os::unix" in line:
        window = "\n".join(lines[max(0, index - 2):index])
        if "#[cfg(unix)]" not in window and "target_os" not in window and "target_vendor" not in window: raise SystemExit(1)
PY
    then :; else status=$?; fail "Unix imports are not explicitly guarded (python3 status $status)"; fi
    if rg -c 'FileIdentity::from_file\(&file\)' "$source" >"$portability_tmp/identity-count" 2>"$portability_tmp/identity-count.err"; then status=0; else status=$?; fi
    identity_count=$(<"$portability_tmp/identity-count")
    [[ "$status" == 0 ]] || fail "FileIdentity count scan failed (rg status $status): $(<"$portability_tmp/identity-count.err")"
    [[ "$identity_count" =~ ^[0-9]+$ && "$identity_count" -ge 2 ]] || fail 'held and post-publication handle identities are not both checked'
    for ownership in 'if !adapter.partial_is_absent() || !adapter.final_is_owned()' 'if self.path_is_owned(&self.partial_path)' 'if self.path_is_owned(&self.final_path)'; do
        if rg -Fq "$ownership" "$source"; then :; else status=$?; fail "missing required ownership boundary: $ownership (rg status $status)"; fi
    done
    if rg -c 'O_NOFOLLOW' "$source" >"$portability_tmp/nofollow-count" 2>"$portability_tmp/nofollow-count.err"; then status=0; else status=$?; fi
    nofollow_count=$(<"$portability_tmp/nofollow-count")
    [[ "$status" == 0 ]] || fail "O_NOFOLLOW count scan failed (rg status $status): $(<"$portability_tmp/nofollow-count.err")"
    [[ "$nofollow_count" =~ ^[0-9]+$ && "$nofollow_count" -eq 4 ]] || fail 'both Linux/Android and Apple path identity checks must be no-follow'
    printf 'native PCM runner portability check: ok\n'
}
subject="${2:-all}"
case "$subject" in v1) check_native_pcm_runner_v1 ;; portability) check_native_pcm_runner_portability_v1 ;; all) check_native_pcm_runner_v1; check_native_pcm_runner_portability_v1 ;; *) printf 'usage: %s [root] [v1|portability|all]\n' "$0" >&2; exit 2 ;; esac
