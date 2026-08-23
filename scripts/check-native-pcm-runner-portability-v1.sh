#!/usr/bin/env bash
# Static portability boundary for the Issue-116 native PCM publication adapters.
set -euo pipefail

root="${1:-.}"
cd "$root"

fail() {
    printf 'native PCM runner portability check failure: %s\n' "$1" >&2
    exit 1
}

source=tools/miso-engine-native-pcm-runner/src/lib.rs
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
