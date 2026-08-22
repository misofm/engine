#!/usr/bin/env bash
# Static Issue-111 successor boundary; never renders or reads a private source.
set -euo pipefail
[[ $# -le 1 ]] || { printf 'usage: %s [ROOT]\n' "$0" >&2; exit 2; }
root="$(cd "${1:-.}" && pwd)"
cd "$root"
fail() { printf 'Issue-111 policy failure: %s\n' "$1" >&2; exit 1; }
hash_file() { sha256sum "$1" | awk '{print $1}'; }
require_hash() {
    [[ -f "$1" && ! -L "$1" ]] || fail "missing frozen path: $1"
    [[ "$(hash_file "$1")" == "$2" ]] || fail "frozen path drift: $1"
}

require_hash Cargo.lock 4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a
while read -r digest path; do require_hash "$path" "$digest"; done <<'EOF'
ee0542febb4b2dbde53f22dab0ae55d73483784fefb5e36b31bcd7b0786726e6 tools/miso-engine-builtins-fixture/Cargo.toml
992297bc85b12655d11f090c50d48681ad0ba60b8c33e7c77864089d5227100a tools/miso-engine-builtins-fixture/src/listening_main.rs
5f1029e337c245b36d0535c3c0e9bbc9ff933429d242ec41596fbd69dcbdd2b4 dsp-research/listening/issue033/FACILITATOR.md
b4971c4805086819f6a7393d7932d5ebd2a9c16ab1d331cde3e2c21c189c0704 dsp-research/listening/issue033/README.md
1b1d07ffeb00dfc3145848ae241d763e37fff9339aa510e7c3a597217aaf68d9 dsp-research/listening/issue033/preparation.schema.json
942d20f743ee7c396101809e31340da56f61120298bdb92ac07f750ae2d688d9 dsp-research/listening/issue033/provenance.template.json
52c33c96ea4e450294f11270e4af3273971b9e4c7e6d4639a227464b7eeda634 dsp-research/listening/issue033/qualification.schema.json
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 dsp-research/listening/issue033/response-form.jsonl
6a84c9f469bd705a10947c746e1c9bc9c7dd49d32e04786a79881c9a930fa392 dsp-research/listening/issue033/response.schema.json
2a2a3745f4e6734108326eb4ee5ba365d26e6613d1d3f65f44e0b4b8e5f5bfdc dsp-research/listening/issue033/reveal.schema.json
3f5c348e3a0e1d93560e347968a8647f65d2ed3672a28e1f79f4bcf8e83eb163 scripts/check-builtins-listening-033.py
dcabbb2b3e04cd630a6056464002614a30a4ee6888010c74fab822bb6554edfc scripts/check-builtins-listening-033.sh
bf3ba048566343a82f55525730714188ab021cf9859618f454043ea143233f0e scripts/preflight-builtins-listening-033.sh
57cb7164ec7c7cc8dcefd9e8d56795dcaa1ed140391b094bf306084e2f2e824c scripts/prepare-builtins-listening-033.sh
9c997ce9c81e7f3c1858582f5952aa0ac8a1fd987fc787745db869f6410c175b scripts/test-builtins-listening-033-policy.sh
37cbaff0fe73904fc4d8820c935789b9e7d1bab031b3626dde896813059b903f scripts/test-builtins-listening-033.sh
EOF

[[ ! -e target/issue33 && ! -L target/issue33 ]] || fail 'stopped Issue-033 namespace appeared'
while read -r name bytes digest; do
    path="target/issue110/$name"
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] ||
        fail "Issue-110 file shape: $name"
    [[ "$(wc -c <"$path")" == "$bytes" && "$(hash_file "$path")" == "$digest" ]] ||
        fail "Issue-110 file identity: $name"
done <<'EOF'
completion.seal.json 2988 3ce39b2653d6b912b6ede083fe8479e46bcbce665095190bd94d15fe82ca238d
miso_engine_builtins_bench 3200296 a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912
builtins-benchmark.preflight.json 1893 9a7a78748b32d8a7cdee1bf7e886e38e6a358f6dfd093d93bbd51bdac2eddaa0
builtins-benchmark.raw.jsonl 38477 8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc
builtins-benchmark.jsonl 38477 8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc
builtins-benchmark.validator.stderr 211 7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396
builtins-benchmark.disposition.json 1075 361f3a4f612e88dcc8a6dcb9f810528b175a64fbf3eea07122024df7971f274f
EOF
[[ "$(find target/issue110 -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)" == \
   $'builtins-benchmark.disposition.json\nbuiltins-benchmark.jsonl\nbuiltins-benchmark.preflight.json\nbuiltins-benchmark.raw.jsonl\nbuiltins-benchmark.validator.stderr\ncompletion.seal.json\nmiso_engine_builtins_bench' ]] ||
    fail 'Issue-110 exact membership'
[[ "$(stat -c %i target/issue110/builtins-benchmark.raw.jsonl)" != \
   "$(stat -c %i target/issue110/builtins-benchmark.jsonl)" ]] || fail 'Issue-110 inode alias'

required=(
    scripts/check-builtins-listening-111.py
    scripts/check-builtins-listening-111.sh
    scripts/test-builtins-listening-111-policy.sh
    scripts/test-builtins-listening-111.sh
    scripts/preflight-builtins-listening-111.sh
    scripts/prepare-builtins-listening-111.sh
)
for path in "${required[@]}"; do [[ -f "$path" && ! -L "$path" ]] || fail "successor path: $path"; done
for token in \
    'packet / "private/assignment-key.json"' \
    'preparation["assignment_key_sha256"] != key_digest' \
    'preparation["packet_member_sha256"]["private/assignment-key.json"] != key_digest' \
    'old.validate_linked_qualification' \
    'stat.S_IMODE(metadata.st_mode) != mode'; do
    rg -Fq "$token" scripts/check-builtins-listening-111.py || fail "successor validator token: $token"
done
for script in scripts/preflight-builtins-listening-111.sh scripts/prepare-builtins-listening-111.sh; do
    rg -Fq 'target/issue111' "$script" || fail "successor namespace: $script"
    ! rg -Fq 'target/issue33' "$script" || fail "predecessor namespace write: $script"
    ! rg -n '(^|[[:space:]])(aplay|ffplay|paplay|play|open|xdg-open|curl|wget|nc)([[:space:]]|$)' "$script" ||
        fail "playback/network command: $script"
done
python3 -I -B scripts/check-builtins-listening-111.py --self-test >/dev/null
if [[ -e target/issue111 || -L target/issue111 ]]; then
    members="$(find target/issue111 -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)"
    [[ "$members" == $'inbox\npreparation.seal.json' ||
       "$members" == $'inbox\nmiso_engine_builtins_fixture_listening\npreflight.seal.json\npreparation.seal.json' ]] ||
        fail 'Issue-111 unauthorized artifact stage'
fi
printf 'Issue-111 listening authority policy: PASS (real preflight/render/playback/session/trial/response/reveal/result=0/0/0/0/0/0/0/0)\n'
