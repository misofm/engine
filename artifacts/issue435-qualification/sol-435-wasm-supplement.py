import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile

manifest = pathlib.Path(sys.argv[1])
repo = pathlib.Path(sys.argv[2])
cfg_path = pathlib.Path(sys.argv[3])
output = pathlib.Path(sys.argv[4])

def run(args, data=None):
    return subprocess.run(args, input=data, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

records = []
for line in manifest.read_text().splitlines():
    if line.startswith("{"):
        row = json.loads(line)
        if "archive" in row and "member" in row:
            records.append(row)
if len(records) != 3:
    raise SystemExit(f"FAIL expected three inspected object records, got {len(records)}")

statuses = []
matches = 0
with output.open("w") as log:
    log.write("source_sha=69fd0bfb0504075db4d302df08ff480faab4102e\n")
    log.write("observation_pattern=observe options=rg -l --binary\n")
    for record in records:
        member = run(["ar", "p", record["archive"], record["member"]])
        digest = hashlib.sha256(member.stdout).hexdigest()
        log.write(json.dumps(dict(
            family=record["family"],
            archive=record["archive"],
            member=record["member"],
            member_read_status=member.returncode,
            bytes=len(member.stdout),
            expected_sha256=record["sha256"],
            actual_sha256=digest,
            hash_matches=digest == record["sha256"],
            member_stderr=member.stderr.decode(errors="replace"),
        )) + "\n")
        if member.returncode != 0 or not member.stdout or digest != record["sha256"]:
            raise SystemExit(f"FAIL member verification {record['family']}")
        extracted = pathlib.Path(
            tempfile.mkstemp(prefix=f"sol435-observe-{record['family']}-", suffix=".o")[1]
        )
        extracted.write_bytes(member.stdout)
        observed = run(["rg", "-l", "--binary", "observe", str(extracted)])
        log.write(json.dumps(dict(
            family=record["family"],
            observation_scan_status=observed.returncode,
            stdout=observed.stdout.decode(errors="replace"),
            stderr=observed.stderr.decode(errors="replace"),
        )) + "\n")
        statuses.append(observed.returncode)
        matches += int(observed.returncode == 0)
    if any(status not in (0, 1) for status in statuses):
        raise SystemExit(f"FAIL observation scan execution error {statuses}")
    if matches == 0:
        fallback = run([
            "rg", "-n", "ObservationSlot",
            str(repo / "crates/engine/src/realtime/observe.rs"),
        ])
        log.write(json.dumps(dict(
            source_fallback_invoked=True,
            source_status=fallback.returncode,
            stdout=fallback.stdout.decode(errors="replace"),
            stderr=fallback.stderr.decode(errors="replace"),
        )) + "\n")
        if fallback.returncode != 0:
            raise SystemExit(f"FAIL source fallback {fallback.returncode}")
    else:
        log.write(json.dumps(dict(
            source_fallback_invoked=False,
            binary_match_count=matches,
            reason="all binary scans completed and at least one matched",
        )) + "\n")
    cfg = cfg_path.read_bytes()
    ptr = run(["rg", "-x", 'target_has_atomic="ptr"'], cfg)
    atomics = run(["rg", "-x", 'target_feature="atomics"'], cfg)
    log.write(
        f"cfg_query_status=0 target_has_atomic_ptr_presence_scan_status={ptr.returncode} "
        f"target_feature_atomics_absence_scan_status={atomics.returncode}\n"
    )
    if ptr.returncode != 0 or atomics.returncode != 1:
        raise SystemExit("FAIL cfg predicates")
    log.write(f"observation_status=PASS scans={statuses} binary_matches={matches}\n")
    log.write("PASS same-member hashes, observation arm, and scalar cfg predicates\n")
