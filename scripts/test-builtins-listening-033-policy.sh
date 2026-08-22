#!/usr/bin/env bash
# Hermetic mutations for the Issue-033 static preparation boundary.
set -euo pipefail
script_directory="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template/crates/miso-engine-builtins/src" "$template/crates/miso-engine-builtins-compiler/src" \
    "$template/tools/miso-engine-builtins-fixture/src" "$template/fixtures/builtins/v1" \
    "$template/fixtures/conformance/v1" "$template/dsp-research/listening/issue033" \
    "$template/scripts" "$template/target/issue110"
cp "$root/Cargo.lock" "$template/"
cp "$root/crates/miso-engine-builtins/src/lib.rs" "$template/crates/miso-engine-builtins/src/"
cp "$root/crates/miso-engine-builtins-compiler/src/lib.rs" "$template/crates/miso-engine-builtins-compiler/src/"
cp "$root/tools/miso-engine-builtins-fixture/Cargo.toml" "$template/tools/miso-engine-builtins-fixture/"
cp "$root/tools/miso-engine-builtins-fixture/src/listening_main.rs" "$template/tools/miso-engine-builtins-fixture/src/"
cp "$root/fixtures/builtins/v1/MANIFEST.tsv" "$template/fixtures/builtins/v1/"
cp "$root/fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm" "$template/fixtures/conformance/v1/"
cp "$root/dsp-research/listening/issue007-filter-abx-preregistration.md" \
    "$root/dsp-research/listening/issue007-matrix-ramp-preregistration.md" \
    "$root/dsp-research/listening/TEMPLATE.md" "$template/dsp-research/listening/"
cp -a "$root/dsp-research/listening/issue033/." "$template/dsp-research/listening/issue033/"
cp "$root/scripts/check-builtins-listening.sh" "$root/scripts/check-builtins-listening-033.py" \
    "$root/scripts/check-builtins-listening-033.sh" "$root/scripts/test-builtins-listening-033.sh" \
    "$root/scripts/test-builtins-listening-033-policy.sh" "$root/scripts/preflight-builtins-listening-033.sh" \
    "$root/scripts/prepare-builtins-listening-033.sh" "$template/scripts/"
cp -a "$root/target/issue110/." "$template/target/issue110/"

bash "$template/scripts/check-builtins-listening-033.sh" "$template" >/dev/null
count=0
reject() {
    count=$((count + 1))
    case_root="$scratch/mutation-$count"
    cp -a "$template" "$case_root"
    operation=$1
    shift
    "$operation" "$case_root" "$@"
    if bash "$case_root/scripts/check-builtins-listening-033.sh" "$case_root" >/dev/null 2>&1; then
        printf 'Issue-033 policy mutation survived: %s\n' "$count" >&2
        exit 1
    fi
}
mutate_lock() { printf mutation >>"$1/Cargo.lock"; }
mutate_product() { printf mutation >>"$1/crates/miso-engine-builtins/src/lib.rs"; }
mutate_path() { printf mutation >>"$1/$2"; }
mutate_issue110() { printf mutation >>"$1/target/issue110/completion.seal.json"; }
add_issue110() { printf unexpected >"$1/target/issue110/unexpected"; }
remove_path() { rm "$1/$2"; }
remove_token() { sed -i "s/$2/REMOVED/g" "$1/$3"; }
add_forbidden() { printf '\nCommand::new("ffplay");\n' >>"$1/tools/miso-engine-builtins-fixture/src/listening_main.rs"; }
add_target() { mkdir -p "$1/target/issue33"; printf bad >"$1/target/issue33/unexpected"; }
weaken_attempt_order() {
    sed -i 's/or not trial_rows\[-1\]\["valid"\]/or False/' \
        "$1/scripts/check-builtins-listening-033.py"
}

reject mutate_lock
reject mutate_product
reject mutate_issue110
reject add_issue110
reject remove_path dsp-research/listening/issue033/preparation.schema.json
for schema in preparation qualification response reveal; do
    reject mutate_path "dsp-research/listening/issue033/$schema.schema.json"
done
reject mutate_path dsp-research/listening/issue033/FACILITATOR.md
for token in packet_member_sha256 validate_linked_qualification; do
    reject remove_token "$token" scripts/check-builtins-listening-033.py
done
reject weaken_attempt_order
reject remove_path scripts/prepare-builtins-listening-033.sh
reject remove_token 'name = "miso_engine_builtins_fixture_listening"' tools/miso-engine-builtins-fixture/Cargo.toml
for token in '480_000' 'QUANTUM: usize = 128' 'hpf_hz: 100.0' 'lpf_hz: 1_000.0' \
    'render_matrix(source, 64)' 'create_new(true)' 'filter_x_candidate' 'matrix_candidate_first' 'SplitMix64'; do
    reject remove_token "$token" tools/miso-engine-builtins-fixture/src/listening_main.rs
done
reject add_forbidden
reject add_target
printf 'Issue-033 policy mutations: PASS (%s rejected)\n' "$count"
