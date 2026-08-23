#!/usr/bin/env bash
# Hermetic mutations for the Issue-115 platform publication boundary.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT

copy_case() {
    case_root="$temp/$1"
    mkdir -p "$case_root/tools/miso-engine-native-pcm-runner/src" "$case_root/scripts"
    cp "$root/tools/miso-engine-native-pcm-runner/src/lib.rs" \
        "$case_root/tools/miso-engine-native-pcm-runner/src/lib.rs"
    cp "$root/scripts/check-native-pcm-runner-portability-v1.sh" "$case_root/scripts/"
}

reject() {
    if "$case_root/scripts/check-native-pcm-runner-portability-v1.sh" "$case_root" \
        >/dev/null 2>&1; then
        printf 'native PCM portability mutation escaped: %s\n' "$1" >&2
        exit 1
    fi
}

copy_case baseline
"$case_root/scripts/check-native-pcm-runner-portability-v1.sh" "$case_root" >/dev/null

copy_case pathname-fallback
printf '\n// mutation\nfn escaped_fallback(&self) { fs::hard_link(&self.partial_path, &self.final_path); }\n' \
    >>"$case_root/tools/miso-engine-native-pcm-runner/src/lib.rs"
reject 'generic pathname fallback'

copy_case unix-import
sed -i '/#\[cfg(unix)\]/{N;s/#\[cfg(unix)\]\n//;}' \
    "$case_root/tools/miso-engine-native-pcm-runner/src/lib.rs"
reject 'unguarded Unix import'

copy_case replace-enabled
sed -i '0,/replace_if_exists = 0/s//replace_if_exists = 1/' \
    "$case_root/tools/miso-engine-native-pcm-runner/src/lib.rs"
reject 'replace-enabled Windows publication'

copy_case missing-post-identity
sed -i '0,/if !adapter.partial_is_owned() || !adapter.final_is_owned()/s//if false/' \
    "$case_root/tools/miso-engine-native-pcm-runner/src/lib.rs"
reject 'unchecked published identity'

copy_case followed-path
sed -i '0,/O_NOFOLLOW/s//O_FOLLOW/' \
    "$case_root/tools/miso-engine-native-pcm-runner/src/lib.rs"
reject 'followed Unix pathname identity'

copy_case unowned-cleanup
printf '\n// unchecked-unowned-cleanup\n' \
    >>"$case_root/tools/miso-engine-native-pcm-runner/src/lib.rs"
reject 'unowned cleanup'

printf 'native PCM runner portability policy mutations: ok\n'
