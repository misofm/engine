#!/usr/bin/env bash
set -euo pipefail
if (($#)); then echo "usage: $0" >&2; exit 2; fi
repo=$(git rev-parse --show-toplevel)
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
sentinel="$tmp/real-workload-sentinel"; : >"$sentinel"
ledger="$tmp/stub-invocations"; : >"$ledger"; export MISO_ENGINE_602_STUB_LEDGER="$ledger"
if python3 "$repo/scripts/input-symmetry-capture-validator.py" >/dev/null 2>&1; then exit 1; fi
if bash "$repo/scripts/preflight-input-symmetry-capture.sh" unexpected >/dev/null 2>&1; then exit 1; fi
if bash "$repo/scripts/run-input-symmetry-capture.sh" unexpected >/dev/null 2>&1; then exit 1; fi
python3 - "$tmp" "$repo/scripts/input-symmetry-capture-validator.py" <<'PY'
import json, pathlib, subprocess, sys
root=pathlib.Path(sys.argv[1]); validator=sys.argv[2]
digest="75eeffae6a0116867d6d6fbe589834df53f3dce87ffd09092f22e5968bc28f87"
seal={"schema_version":1,"issue":602,"kind":"input_symmetry_capture_seal","status":"READY","candidate_commit":"a"*40,"candidate_tree":"b"*40,"binary_sha256":"c"*64,"fixture_sha256":"d"*64,"source_sha256":"e"*64,"timed_source_sha256":"f"*64,"untimed_source_sha256":"0"*64,"dispatcher_sha256":"1"*64,"validator_sha256":"2"*64,"runner_sha256":"3"*64,"preflight_sha256":"4"*64,"cargo_lock_sha256":"5"*64,"argv":"/sealed/bench input-symmetry-capture","cwd":"/sealed","compiler":"rustc 1.0","target_triple":"x86_64-unknown-linux-gnu","effective_build_flags":"-C target-feature=+avx2,+fma -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort -C debug=1","sample_rate_hz":48000,"quantum_frames":128,"lane_width":8,"track_count":8,"records_per_block":8,"preparation_blocks_per_owner":512,"measured_blocks_per_owner":4096,"expected_records":2,"expected_attempted_records":65536,"expected_renders_per_round":8192,"expected_rounds":2,"expected_timed_render_calls":16384,"expected_workload_processes":1}
keys=("cpu_model","governor_or_power_mode","rust_version","llvm_version","target_features","profile","background_load_note","measurement_control","cpu_affinity")
metadata={k:None for k in keys}; metadata.update(target_triple=seal["target_triple"],candidate_commit=seal["candidate_commit"],rust_version=seal["compiler"]); missing=sorted(k for k,v in metadata.items() if v is None)
def rec(n):
 r={"schema_version":1,"issue":602,"kind":"input_symmetry_capture","round":n,"fixture_id":"fixtures/session/v1/parametric-eq-bank-console.json","fixture_sha256":seal["fixture_sha256"],"sample_rate_hz":48000,"quantum_frames":128,"lane_width":8,"track_count":8,"records_per_block":8,"target_pair_db":[-6.0,-12.0],"smoothing_samples":256,"preparation_blocks_per_owner":512,"measured_blocks_per_owner":4096,"owners":2,"render_denominator":8192,"attempted_records":65536,"accepted_records":65536,"successful_renders":8192,"render_errors":0,"output_words":2097152,"nonzero_samples":1,"owner_digests":[digest,digest],"elapsed_ns":8192,"nanoseconds_per_plan_render":1,"source_commit":seal["candidate_commit"],"source_tree":seal["candidate_tree"],"binary_sha256":seal["binary_sha256"],"source_sha256":seal["source_sha256"],"argv":seal["argv"],"cwd":seal["cwd"],"compiler":seal["compiler"],"target_triple":seal["target_triple"],"effective_build_flags":seal["effective_build_flags"],"backend":"Simd8","os":"linux",**metadata,"missing_metadata":missing,"descriptive_only":True}; return r
(root/"seal.json").write_text(json.dumps(seal,separators=(",",":"))+"\n"); (root/"records.jsonl").write_text("\n".join(json.dumps(rec(n),separators=(",",":")) for n in (1,2))+"\n")

def check(path, lines):
 path.write_text("\n".join(lines)+"\n"); assert subprocess.run([sys.executable,validator,str(root/"seal.json"),str(path)],capture_output=True).returncode != 0
def check_seal(obj, name):
 p=root/name; p.write_text(json.dumps(obj)+"\n"); assert subprocess.run([sys.executable,validator,"--seal-only",str(p)],capture_output=True).returncode != 0
lines=(root/"records.jsonl").read_text().splitlines(); check(root/"duplicate.jsonl",[lines[0][:-1]+',"round":1}',lines[1]])
a=json.loads(lines[0]); a["backend"]=True; check(root/"numeric.jsonl",[json.dumps(a),lines[1]])
a=json.loads(lines[0]); a["round"]=2; check(root/"reordered.jsonl",[json.dumps(a),lines[1]])
a=json.loads(lines[0]); a["target_pair_db"]= [-6,-12.0]; check(root/"pair-type.jsonl",[json.dumps(a),lines[1]])
bad=json.loads((root/"seal.json").read_text()); del bad["status"]; check_seal(bad,"seal-missing.json")
bad=json.loads((root/"seal.json").read_text()); bad["issue"]=True; check_seal(bad,"seal-type.json")
bad=json.loads((root/"seal.json").read_text()); bad["unexpected"]=1; check_seal(bad,"seal-extra.json")
PY
python3 scripts/input-symmetry-capture-validator.py --seal-only "$tmp/seal.json"
python3 scripts/input-symmetry-capture-validator.py "$tmp/seal.json" "$tmp/records.jsonl"
[[ ! -s "$sentinel" ]] || { echo "real workload/timing sentinel was touched" >&2; exit 1; }
# Run the one-shot lifecycle against a deliberately failing stub in an isolated artifact root.
runner_root="$tmp/runner"; mkdir -p "$runner_root/prepared-release"
cat >"$runner_root/prepared-release/bench" <<'EOF_STUB'
#!/usr/bin/env bash
echo child-failure >>"$MISO_ENGINE_602_STUB_LEDGER"
exit 7
EOF_STUB
chmod +x "$runner_root/prepared-release/bench"
python3 - "$runner_root/input-symmetry-capture.seal.json" "$runner_root/prepared-release/bench" "$repo" <<'PY2'
import hashlib, json, pathlib, subprocess, sys
out,binary,repo=sys.argv[1:]; root=pathlib.Path(repo); b=pathlib.Path(binary)
def h(path): return hashlib.sha256(path.read_bytes()).hexdigest()
commit=subprocess.check_output(["git","-C",repo,"rev-parse","HEAD"],text=True).strip(); tree=subprocess.check_output(["git","-C",repo,"rev-parse","HEAD^{tree}"],text=True).strip()
timed=root/"tools/bench/src/input_symmetry_capture.rs"; untimed=root/"tools/bench/src/input_symmetry.rs"; dispatch=root/"tools/bench/src/main.rs"
compiler=subprocess.check_output(["rustc","--version"],text=True).strip(); target=subprocess.check_output(["rustc","-vV"],text=True).split("host: ",1)[1].splitlines()[0]
seal={"schema_version":1,"issue":602,"kind":"input_symmetry_capture_seal","status":"READY","candidate_commit":commit,"candidate_tree":tree,"binary_sha256":h(b),"fixture_sha256":h(root/"fixtures/session/v1/parametric-eq-bank-console.json"),"source_sha256":hashlib.sha256(timed.read_bytes()+untimed.read_bytes()+dispatch.read_bytes()).hexdigest(),"timed_source_sha256":h(timed),"untimed_source_sha256":h(untimed),"dispatcher_sha256":h(dispatch),"validator_sha256":h(root/"scripts/input-symmetry-capture-validator.py"),"runner_sha256":h(root/"scripts/run-input-symmetry-capture.sh"),"preflight_sha256":h(root/"scripts/preflight-input-symmetry-capture.sh"),"cargo_lock_sha256":h(root/"Cargo.lock"),"argv":str(b)+" input-symmetry-capture","cwd":repo,"compiler":compiler,"target_triple":target,"effective_build_flags":"-C target-feature=+avx2,+fma -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort -C debug=1","sample_rate_hz":48000,"quantum_frames":128,"lane_width":8,"track_count":8,"records_per_block":8,"preparation_blocks_per_owner":512,"measured_blocks_per_owner":4096,"expected_records":2,"expected_attempted_records":65536,"expected_renders_per_round":8192,"expected_rounds":2,"expected_timed_render_calls":16384,"expected_workload_processes":1}
pathlib.Path(out).write_text(json.dumps(seal,separators=(",",":"))+"\n")
PY2
MISO_ENGINE_602_SELF_TEST=1 MISO_ENGINE_602_ROOT="$runner_root" bash scripts/run-input-symmetry-capture.sh >/dev/null 2>&1 || true
[[ -f "$runner_root/raw/stdout.jsonl" && -f "$runner_root/raw/stderr.log" && -f "$runner_root/disposition.json" ]] || { echo "runner did not retain child evidence" >&2; exit 1; }
[[ ! -e "$runner_root/input-symmetry-capture.jsonl" ]] || { echo "failed child was published" >&2; exit 1; }
grep -q '"child_status":7' "$runner_root/disposition.json"
[[ ! -s "$runner_root/raw/stdout.jsonl" && -f "$runner_root/raw/stderr.log" ]]
if MISO_ENGINE_602_SELF_TEST=1 MISO_ENGINE_602_ROOT="$runner_root" bash scripts/run-input-symmetry-capture.sh >/dev/null 2>&1; then exit 1; fi
# Dangling protected paths are refused before reservation or child launch.
dangling_root="$tmp/dangling-protected"; cp -a "$runner_root" "$dangling_root"; rm -rf "$dangling_root/raw" "$dangling_root/disposition.json" "$dangling_root/.capture-reservation"; ln -s "$tmp/no-such-output" "$dangling_root/input-symmetry-capture.jsonl"
if MISO_ENGINE_602_SELF_TEST=1 MISO_ENGINE_602_ROOT="$dangling_root" bash scripts/run-input-symmetry-capture.sh >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$dangling_root/raw" ]]
# Every sealed identity mismatch must refuse before the synthetic child is reached.
ledger_before=$(wc -l <"$ledger")
for identity in candidate_commit candidate_tree binary_sha256 fixture_sha256 source_sha256 timed_source_sha256 untimed_source_sha256 dispatcher_sha256 validator_sha256 runner_sha256 preflight_sha256 cargo_lock_sha256 argv cwd compiler target_triple effective_build_flags; do
  identity_root="$tmp/identity-$identity"; cp -a "$runner_root" "$identity_root"; rm -rf "$identity_root/raw" "$identity_root/disposition.json" "$identity_root/.capture-reservation"
  python3 - "$identity_root/input-symmetry-capture.seal.json" "$identity" <<'PYID'
import json, pathlib, sys
p=pathlib.Path(sys.argv[1]); key=sys.argv[2]; s=json.loads(p.read_text())
s[key]=(("0"*40) if key in ("candidate_commit","candidate_tree") else (("0"*64) if key.endswith("sha256") else "identity-mismatch"))
p.write_text(json.dumps(s,separators=(",",":"))+"\n")
PYID
  MISO_ENGINE_602_SELF_TEST=1 MISO_ENGINE_602_ROOT="$identity_root" bash scripts/run-input-symmetry-capture.sh >/dev/null 2>&1 || true
  grep -q '"child_status":"not_run"' "$identity_root/disposition.json"
done
[[ "$(wc -l <"$ledger")" == "$ledger_before" ]] || { echo "identity mismatch reached synthetic child" >&2; exit 1; }
# A successful child with malformed output exercises validator failure and raw validator logs.
validator_root="$tmp/validator-failure"; cp -a "$runner_root" "$validator_root"; rm -rf "$validator_root/raw" "$validator_root/disposition.json" "$validator_root/.capture-reservation"
cat >"$validator_root/prepared-release/bench" <<'EOF_BAD'
#!/usr/bin/env bash
echo validator-child >>"$MISO_ENGINE_602_STUB_LEDGER"
printf 'malformed\n'
exit 0
EOF_BAD
chmod +x "$validator_root/prepared-release/bench"
python3 - "$validator_root/input-symmetry-capture.seal.json" "$validator_root/prepared-release/bench" <<'PY3'
import hashlib, json, pathlib, sys
seal_path,binary=sys.argv[1:]; p=pathlib.Path(seal_path); seal=json.loads(p.read_text()); seal["binary_sha256"]=hashlib.sha256(pathlib.Path(binary).read_bytes()).hexdigest(); seal["argv"]=str(pathlib.Path(binary).resolve())+" input-symmetry-capture"; p.write_text(json.dumps(seal,separators=(",",":"))+"\n")
PY3
MISO_ENGINE_602_SELF_TEST=1 MISO_ENGINE_602_ROOT="$validator_root" bash scripts/run-input-symmetry-capture.sh >/dev/null 2>&1 || true
[[ -f "$validator_root/raw/validator.stdout" && -f "$validator_root/raw/validator.stderr" ]] || { echo "validator evidence was not retained" >&2; exit 1; }
grep -q '"validator_status":"1"' "$validator_root/disposition.json"
grep -q '^malformed$' "$validator_root/raw/stdout.jsonl"; grep -q '^FAIL:' "$validator_root/raw/validator.stderr"
# Valid records with a broken destination exercise transactional publication failure.
publication_root="$tmp/publication-failure"; mkdir -p "$publication_root/prepared-release"
cp "$runner_root/input-symmetry-capture.seal.json" "$publication_root/input-symmetry-capture.seal.json"
cp "$runner_root/prepared-release/bench" "$publication_root/prepared-release/bench"
cat >"$publication_root/prepared-release/bench" <<EOF_PUB
#!/usr/bin/env bash
echo publication-child >>"\$MISO_ENGINE_602_STUB_LEDGER"
cat "\$MISO_ENGINE_602_VALID_RECORDS"
EOF_PUB
chmod +x "$publication_root/prepared-release/bench"
python3 - "$publication_root/input-symmetry-capture.seal.json" "$publication_root/prepared-release/bench" <<'PY4'
import hashlib, json, pathlib, sys
p=pathlib.Path(sys.argv[1]); b=pathlib.Path(sys.argv[2]); s=json.loads(p.read_text()); s["binary_sha256"]=hashlib.sha256(b.read_bytes()).hexdigest(); s["argv"]=str(b.resolve())+" input-symmetry-capture"; p.write_text(json.dumps(s,separators=(",",":"))+"\n")
PY4
python3 - "$tmp/records.jsonl" "$publication_root/input-symmetry-capture.seal.json" "$tmp/pub-records.jsonl" <<'PY5'
import json, pathlib, sys
records=pathlib.Path(sys.argv[1]).read_text().splitlines(); seal=json.loads(pathlib.Path(sys.argv[2]).read_text()); pathlib.Path(sys.argv[3]).write_text("\n".join(json.dumps(dict(json.loads(line),source_commit=seal["candidate_commit"],source_tree=seal["candidate_tree"],binary_sha256=seal["binary_sha256"],fixture_sha256=seal["fixture_sha256"],source_sha256=seal["source_sha256"],argv=seal["argv"],cwd=seal["cwd"],compiler=seal["compiler"],rust_version=seal["compiler"],candidate_commit=seal["candidate_commit"])) for line in records)+"\n")
PY5
MISO_ENGINE_602_SELF_TEST=1 MISO_ENGINE_602_SELF_TEST_FAULT=publication MISO_ENGINE_602_ROOT="$publication_root" MISO_ENGINE_602_VALID_RECORDS="$tmp/pub-records.jsonl" bash scripts/run-input-symmetry-capture.sh >/dev/null 2>&1 || true
grep -q '"accepted_status":"publication_failed"' "$publication_root/disposition.json"
# A post-workload disposition persistence failure retains the accepted bytes and recovery files.
persistence_root="$tmp/persistence-failure"; mkdir -p "$persistence_root/prepared-release"
cp "$publication_root/input-symmetry-capture.seal.json" "$persistence_root/input-symmetry-capture.seal.json"
cp "$publication_root/prepared-release/bench" "$persistence_root/prepared-release/bench"
python3 - "$persistence_root/input-symmetry-capture.seal.json" "$persistence_root/prepared-release/bench" <<'PY6'
import json, pathlib, sys
p=pathlib.Path(sys.argv[1]); s=json.loads(p.read_text()); s["argv"]=str(pathlib.Path(sys.argv[2]).resolve())+" input-symmetry-capture"; p.write_text(json.dumps(s,separators=(",",":"))+"\n")
PY6
python3 - "$tmp/pub-records.jsonl" "$persistence_root/prepared-release/bench" "$tmp/persist-records.jsonl" <<'PY7'
import json, pathlib, sys
lines=pathlib.Path(sys.argv[1]).read_text().splitlines(); argv=str(pathlib.Path(sys.argv[2]).resolve())+" input-symmetry-capture"; pathlib.Path(sys.argv[3]).write_text("\n".join(json.dumps(dict(json.loads(line),argv=argv)) for line in lines)+"\n")
PY7
MISO_ENGINE_602_SELF_TEST=1 MISO_ENGINE_602_SELF_TEST_FAULT=persistence MISO_ENGINE_602_ROOT="$persistence_root" MISO_ENGINE_602_VALID_RECORDS="$tmp/persist-records.jsonl" bash scripts/run-input-symmetry-capture.sh >/dev/null 2>&1 || true
[[ -f "$persistence_root/input-symmetry-capture.jsonl" && -f "$persistence_root/disposition.json.tmp" ]]
[[ ! -e "$repo/artifacts/issue-602-input-symmetry-capture/input-symmetry-capture.seal.json" && ! -e "$repo/artifacts/issue-602-input-symmetry-capture/input-symmetry-capture.jsonl" ]] || { echo "self-test touched final namespace" >&2; exit 1; }
[[ "$(wc -l <"$ledger")" == 4 ]] || { echo "unexpected stub invocation count" >&2; exit 1; }
[[ ! -s "$sentinel" ]] || { echo "real workload/timing sentinel was touched" >&2; exit 1; }
echo "PASS: #602 validator mutations and durable zero-real-workload sentinel"
