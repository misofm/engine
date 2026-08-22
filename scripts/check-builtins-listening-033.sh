#!/usr/bin/env bash
# Static, zero-render Issue-033 preparation boundary checker.
set -euo pipefail
[[ $# -le 1 ]] || { printf 'usage: %s [ROOT]\n' "$0" >&2; exit 2; }
root="$(cd "${1:-.}" && pwd)"
cd "$root"
fail() { printf 'Issue-033 policy failure: %s\n' "$1" >&2; exit 1; }
hash_file() { sha256sum "$1" | awk '{print $1}'; }
require_hash() {
    [[ -f "$1" && ! -L "$1" ]] || fail "missing regular authority: $1"
    [[ "$(hash_file "$1")" == "$2" ]] || fail "authority hash: $1"
}

require_hash Cargo.lock 4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a
require_hash crates/miso-engine-builtins/src/lib.rs 9095eb93a0c04b1eabf95d95a94c4490dd20887ed602f2ce4ed3d90724980f79
require_hash crates/miso-engine-builtins-compiler/src/lib.rs 1b7fad8a72fc76ffcb97e31c11bfa386168b9c9c688083069c24623f3e2afc75
require_hash fixtures/builtins/v1/MANIFEST.tsv bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff
require_hash fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm \
    69968e143ce9b3920825d3c3a308eb7f6d042ebd44c5cbeea836118184241795
require_hash dsp-research/listening/issue007-filter-abx-preregistration.md \
    be3a9c192f2e73783af6f16e6268f065fc5862b1688cf02e791faf2ba666060c
require_hash dsp-research/listening/issue007-matrix-ramp-preregistration.md \
    66316d8fd6a2a25fb904373e8a88c6802fbc1c60fa2f95ed02f549b423d014a9
require_hash dsp-research/listening/TEMPLATE.md \
    18866f68bcb416732fb43b9a6e27cf9d042e12f68e30d9169b6cc60f1f9d18c2
require_hash scripts/check-builtins-listening.sh \
    6a3628687f31007b402e706fc09b0bb4d5eb85303fb9abd3b0a1921e859c8cd8
require_hash dsp-research/listening/issue033/FACILITATOR.md \
    5f1029e337c245b36d0535c3c0e9bbc9ff933429d242ec41596fbd69dcbdd2b4
require_hash dsp-research/listening/issue033/preparation.schema.json \
    1b1d07ffeb00dfc3145848ae241d763e37fff9339aa510e7c3a597217aaf68d9
require_hash dsp-research/listening/issue033/qualification.schema.json \
    52c33c96ea4e450294f11270e4af3273971b9e4c7e6d4639a227464b7eeda634
require_hash dsp-research/listening/issue033/response.schema.json \
    6a84c9f469bd705a10947c746e1c9bc9c7dd49d32e04786a79881c9a930fa392
require_hash dsp-research/listening/issue033/reveal.schema.json \
    2a2a3745f4e6734108326eb4ee5ba365d26e6613d1d3f65f44e0b4b8e5f5bfdc
require_hash dsp-research/listening/issue033/provenance.template.json \
    942d20f743ee7c396101809e31340da56f61120298bdb92ac07f750ae2d688d9
require_hash dsp-research/listening/issue033/response-form.jsonl \
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

while read -r name bytes digest; do
    path="target/issue110/$name"
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] ||
        fail "Issue-110 regular one-link artifact: $name"
    [[ "$(wc -c <"$path")" == "$bytes" && "$(hash_file "$path")" == "$digest" ]] ||
        fail "Issue-110 artifact identity: $name"
done <<'EOF'
completion.seal.json 2988 3ce39b2653d6b912b6ede083fe8479e46bcbce665095190bd94d15fe82ca238d
miso_engine_builtins_bench 3200296 a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912
builtins-benchmark.preflight.json 1893 9a7a78748b32d8a7cdee1bf7e886e38e6a358f6dfd093d93bbd51bdac2eddaa0
builtins-benchmark.raw.jsonl 38477 8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc
builtins-benchmark.jsonl 38477 8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc
builtins-benchmark.validator.stderr 211 7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396
builtins-benchmark.disposition.json 1075 361f3a4f612e88dcc8a6dcb9f810528b175a64fbf3eea07122024df7971f274f
EOF
[[ ! -e target/issue110/builtins-benchmark.prelaunch.disposition.json &&
   ! -L target/issue110/builtins-benchmark.prelaunch.disposition.json ]] ||
    fail 'Issue-110 prelaunch disposition appeared'
[[ "$(find target/issue110 -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)" == $'builtins-benchmark.disposition.json\nbuiltins-benchmark.jsonl\nbuiltins-benchmark.preflight.json\nbuiltins-benchmark.raw.jsonl\nbuiltins-benchmark.validator.stderr\ncompletion.seal.json\nmiso_engine_builtins_bench' ]] ||
    fail 'Issue-110 exact membership'
[[ "$(stat -c %i target/issue110/builtins-benchmark.raw.jsonl)" != \
   "$(stat -c %i target/issue110/builtins-benchmark.jsonl)" ]] || fail 'Issue-110 inode alias'

required=(
    tools/miso-engine-builtins-fixture/src/listening_main.rs
    dsp-research/listening/issue033/preparation.schema.json
    dsp-research/listening/issue033/response.schema.json
    dsp-research/listening/issue033/reveal.schema.json
    dsp-research/listening/issue033/qualification.schema.json
    dsp-research/listening/issue033/provenance.template.json
    dsp-research/listening/issue033/response-form.jsonl
    dsp-research/listening/issue033/FACILITATOR.md
    scripts/check-builtins-listening-033.py
    scripts/test-builtins-listening-033.sh
    scripts/test-builtins-listening-033-policy.sh
    scripts/preflight-builtins-listening-033.sh
    scripts/prepare-builtins-listening-033.sh
)
for path in "${required[@]}"; do [[ -f "$path" && ! -L "$path" ]] || fail "required path: $path"; done
[[ "$(rg -n '^name = "miso_engine_builtins_fixture_listening"$' tools/miso-engine-builtins-fixture/Cargo.toml | wc -l)" == 1 &&
   "$(rg -n '^path = "src/listening_main.rs"$' tools/miso-engine-builtins-fixture/Cargo.toml | wc -l)" == 1 ]] ||
    fail 'listening binary registration'
for token in '480_000' 'QUANTUM: usize = 128' 'hpf_hz: 100.0' 'lpf_hz: 1_000.0' \
    'render_matrix(source, 64)' '48_000, 96_000' 'create_new(true)' '0o600' \
    'filter_x_candidate' 'matrix_candidate_first' 'SplitMix64'; do
    rg -Fq "$token" tools/miso-engine-builtins-fixture/src/listening_main.rs || fail "renderer token: $token"
done
for token in 'packet_member_sha256' 'COPIED_PACKET_INPUTS' 'validate_linked_qualification' \
    'not trial_rows[-1]["valid"]' 'actual_hashes != preparation["packet_member_sha256"]'; do
    rg -Fq "$token" scripts/check-builtins-listening-033.py || fail "validator token: $token"
done
for forbidden in 'std::net' 'TcpStream' 'UdpSocket' 'Command::new' 'target/issue110' \
    'miso_engine_builtins_bench' 'Instant::now' 'SystemTime::now'; do
    ! rg -Fq "$forbidden" tools/miso-engine-builtins-fixture/src/listening_main.rs ||
        fail "renderer forbidden reachability: $forbidden"
done
for script in scripts/preflight-builtins-listening-033.sh scripts/prepare-builtins-listening-033.sh; do
    ! rg -n '(^|[[:space:]])(aplay|ffplay|paplay|play|open|xdg-open|curl|wget|nc)([[:space:]]|$)' "$script" ||
        fail "playback/network command: $script"
    ! rg -n 'target/issue110/.*(>|>>|mv|cp|ln|rm|truncate)' "$script" ||
        fail "Issue-110 write: $script"
done
python3 -I -B scripts/check-builtins-listening-033.py --self-test >/dev/null
python3 -I -B - <<'PY' || fail 'canonical schemas/answer-free fixtures'
import json
from pathlib import Path
root = Path("dsp-research/listening/issue033")
for path in sorted(root.glob("*.schema.json")):
    raw = path.read_bytes()
    value = json.loads(raw.decode("utf-8", "strict"))
    expected = (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()
    assert raw == expected
assert (root / "response-form.jsonl").read_bytes() == b""
template = json.loads((root / "provenance.template.json").read_text(encoding="utf-8"))
assert template["permission_confirmed"] is False
assert all("answer" not in path.name for path in root.iterdir())
PY
if [[ -e target/issue33 || -L target/issue33 ]]; then
    allowed=$'inbox\npreparation.seal.json'
    members="$(find target/issue33 -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)"
    [[ "$members" == "$allowed" || "$members" == $'inbox\nmiso_engine_builtins_fixture_listening\npreflight.seal.json\npreparation.seal.json' ]] ||
        fail 'Issue-033 unauthorized artifact stage'
fi
printf 'Issue-033 preparation policy: PASS (real preflight/render/playback/session/trial/response/reveal/result=0/0/0/0/0/0/0/0)\n'
