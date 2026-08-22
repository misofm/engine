#!/usr/bin/env bash
# Hermetic static mutations for the Issue-111 successor authority edge.
set -euo pipefail
script_directory="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template/tools/miso-engine-builtins-fixture/src" \
    "$template/dsp-research/listening/issue033" "$template/scripts" "$template/target/issue110"
cp "$root/Cargo.lock" "$template/"
cp "$root/tools/miso-engine-builtins-fixture/Cargo.toml" "$template/tools/miso-engine-builtins-fixture/"
cp "$root/tools/miso-engine-builtins-fixture/src/listening_main.rs" "$template/tools/miso-engine-builtins-fixture/src/"
cp "$root/dsp-research/listening/issue007-filter-abx-preregistration.md" \
    "$root/dsp-research/listening/issue007-matrix-ramp-preregistration.md" \
    "$root/dsp-research/listening/TEMPLATE.md" "$template/dsp-research/listening/"
cp -a "$root/dsp-research/listening/issue033/." "$template/dsp-research/listening/issue033/"
cp "$root/scripts/check-builtins-listening.sh" "$root/scripts/"*builtins-listening-033* \
    "$root/scripts/"*builtins-listening-111* "$template/scripts/"
cp -a "$root/target/issue110/." "$template/target/issue110/"

bash "$template/scripts/check-builtins-listening-111.sh" "$template" >/dev/null
count=0
reject() {
    count=$((count + 1))
    case_root="$scratch/mutation-$count"
    cp -a "$template" "$case_root"
    "$1" "$case_root"
    if bash "$case_root/scripts/check-builtins-listening-111.sh" "$case_root" >/dev/null 2>&1; then
        printf 'Issue-111 policy mutation survived: %s\n' "$count" >&2
        exit 1
    fi
}
mutate_predecessor() { printf drift >>"$1/scripts/check-builtins-listening-033.py"; }
mutate_issue110() { printf drift >>"$1/target/issue110/completion.seal.json"; }
add_issue110() { printf extra >"$1/target/issue110/extra"; }
add_issue33() { mkdir "$1/target/issue33"; }
remove_successor() { rm "$1/scripts/prepare-builtins-listening-111.sh"; }
weaken_assignment() {
    sed -i 's/preparation\["assignment_key_sha256"\] != key_digest/False/' \
        "$1/scripts/check-builtins-listening-111.py"
}
weaken_member() {
    sed -i 's/preparation\["packet_member_sha256"\]\["private\/assignment-key.json"\] != key_digest/False/' \
        "$1/scripts/check-builtins-listening-111.py"
}
alternate_path() {
    sed -i 's@private/assignment-key.json@private/alternate-key.json@g' \
        "$1/scripts/check-builtins-listening-111.py"
}
remove_linked() {
    sed -i 's/old.validate_linked_qualification/old.validate_qualification/' \
        "$1/scripts/check-builtins-listening-111.py"
}
wrong_namespace() {
    sed -i 's/target\/issue111/target\/issue33/g' "$1/scripts/prepare-builtins-listening-111.sh"
}
add_player() { printf '\nffplay forbidden.wav\n' >>"$1/scripts/prepare-builtins-listening-111.sh"; }
add_issue111() { mkdir -p "$1/target/issue111"; printf unexpected >"$1/target/issue111/unexpected"; }

reject mutate_predecessor
reject mutate_issue110
reject add_issue110
reject add_issue33
reject remove_successor
reject weaken_assignment
reject weaken_member
reject alternate_path
reject remove_linked
reject wrong_namespace
reject add_player
reject add_issue111
printf 'Issue-111 policy mutations: PASS (%s rejected)\n' "$count"
