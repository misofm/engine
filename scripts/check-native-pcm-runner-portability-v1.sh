#!/usr/bin/env bash
# Static portability boundary for the Issue-115 native PCM publication adapters.
set -euo pipefail

root="${1:-.}"
cd "$root"

fail() {
    printf 'native PCM runner portability check failure: %s\n' "$1" >&2
    exit 1
}

source=tools/miso-engine-native-pcm-runner/src/lib.rs
[[ -f "$source" ]] || fail 'runner library is missing'

for required in \
    'cfg(any(target_os = "linux", target_os = "android"))' \
    'AT_EMPTY_PATH' \
    'cfg(target_vendor = "apple")' \
    '/dev/fd/' \
    'AT_SYMLINK_FOLLOW' \
    'cfg(windows)' \
    'GetFileInformationByHandle' \
    'SetFileInformationByHandle' \
    'replace_if_exists = 0' \
    'O_NOFOLLOW' \
    'FILE_FLAG_OPEN_REPARSE_POINT' \
    'platform.unsupported' \
    'trait PublicationAdapter' \
    'complete_publication'; do
    rg -Fq "$required" "$source" || fail "missing required boundary: $required"
done

! rg -n 'fs::hard_link\(&self\.partial_path|std::fs::hard_link\(&self\.partial_path' "$source" \
    || fail 'generic pathname hard-link publication is forbidden'
! rg -n 'replace_if_exists\s*=\s*(1|true)|REPLACE_IF_EXISTS|MOVEFILE_REPLACE_EXISTING' "$source" \
    || fail 'replace-enabled publication is forbidden'
! rg -n 'unchecked-unowned-cleanup' "$source" \
    || fail 'cleanup by an unowned pathname is forbidden'

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
rg -Fq 'if !adapter.partial_is_owned() || !adapter.final_is_owned()' "$source" \
    || fail 'post-publication identity check is missing'
rg -Fq 'if !self.sink.path_is_owned(&self.sink.partial_path)' "$source" \
    || fail 'owned-only partial cleanup check is missing'
[[ $(rg -c 'O_NOFOLLOW' "$source") -eq 4 ]] \
    || fail 'both Linux/Android and Apple path identity checks must be no-follow'

printf 'native PCM runner portability check: ok\n'
