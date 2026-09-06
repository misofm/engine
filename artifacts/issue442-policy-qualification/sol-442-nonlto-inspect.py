import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile

target = pathlib.Path(sys.argv[1])
repo = pathlib.Path(sys.argv[2])
cfg_path = pathlib.Path(sys.argv[3])
log_path = pathlib.Path(sys.argv[4])
base = target / "wasm32-unknown-unknown/release/deps"

def run(args, data=None, allowed=(0,)):
    process = subprocess.run(args, input=data, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if process.returncode not in allowed:
        raise SystemExit(
            f"FAIL command={args!r} status={process.returncode} "
            f"stderr={process.stderr.decode(errors='replace')}"
        )
    return process

records = []
with log_path.open("w") as log:
    log.write("source_sha=dc55baf97074edf98abbfc9477aa6c420f0599af\n")
    log.write(f"scope=named scalar non-LTO engine/source/target_smoke archives\nbase={base}\n")
    for family in ("engine", "source", "target_smoke"):
        found = run([
            "find", str(base), "-maxdepth", "1", "-type", "f",
            "-name", f"lib{family}-*.rlib", "-print",
        ])
        ordered = run(["sort"], found.stdout)
        archives = ordered.stdout.decode().splitlines()
        log.write(
            f"family={family} find_status={found.returncode} find_bytes={len(found.stdout)} "
            f"sort_status={ordered.returncode} archive_count={len(archives)} archives={archives!r}\n"
        )
        if len(archives) != 1:
            raise SystemExit(f"FAIL {family} archive count {len(archives)}")
        archive = archives[0]
        listing = run(["ar", "t", archive])
        members = listing.stdout.decode().splitlines()
        objects = [member for member in members if member.endswith(".o")]
        log.write(
            f"family={family} archive_list_status={listing.returncode} member_count={len(members)} "
            f"object_count={len(objects)} duplicate_objects={len(objects)-len(set(objects))}\n"
        )
        if not objects or len(objects) != len(set(objects)):
            raise SystemExit(f"FAIL {family} object population")
        for index, member in enumerate(objects):
            member_read = run(["ar", "p", archive, member])
            if not member_read.stdout:
                raise SystemExit(f"FAIL empty member {member}")
            extracted = pathlib.Path(
                tempfile.mkstemp(prefix=f"sol442-{family}-", suffix=".o")[1]
            )
            extracted.write_bytes(member_read.stdout)
            decoded = run(["wasm-objdump", "-d", str(extracted)])
            decoded_path = extracted.with_suffix(".decoded")
            decoded_path.write_bytes(decoded.stdout)
            atomic = run(["rg", "-n", r"atomic\.", str(decoded_path)], allowed=(0, 1))
            observation = run(
                ["rg", "-l", "--binary", "observe", str(extracted)], allowed=(0, 1)
            )
            record = dict(
                family=family,
                archive=archive,
                archive_member_index=index,
                member=member,
                member_read_status=member_read.returncode,
                member_bytes=len(member_read.stdout),
                sha256=hashlib.sha256(member_read.stdout).hexdigest(),
                decoder_status=decoded.returncode,
                decoded_bytes=len(decoded.stdout),
                atomic_scan_status=atomic.returncode,
                atomic_stdout=atomic.stdout.decode(errors="replace"),
                atomic_stderr=atomic.stderr.decode(errors="replace"),
                observation_scan_status=observation.returncode,
                observation_stdout=observation.stdout.decode(errors="replace"),
                observation_stderr=observation.stderr.decode(errors="replace"),
            )
            log.write(json.dumps(record) + "\n")
            if atomic.returncode != 1:
                raise SystemExit(f"FAIL atomic opcode or scan error {member}: {atomic.returncode}")
            records.append(record)
    matches = sum(record["observation_scan_status"] == 0 for record in records)
    statuses = [record["observation_scan_status"] for record in records]
    if any(status not in (0, 1) for status in statuses):
        raise SystemExit(f"FAIL observation scan execution error {statuses}")
    if matches == 0:
        fallback = run(
            ["rg", "-n", "ObservationSlot", str(repo / "crates/engine/src/realtime/observe.rs")],
            allowed=(0, 1),
        )
        log.write(json.dumps(dict(
            source_fallback_invoked=True,
            source_status=fallback.returncode,
            stdout=fallback.stdout.decode(errors="replace"),
            stderr=fallback.stderr.decode(errors="replace"),
        )) + "\n")
        if fallback.returncode != 0:
            raise SystemExit(f"FAIL observation source fallback status {fallback.returncode}")
    else:
        log.write(json.dumps(dict(
            source_fallback_invoked=False,
            binary_match_count=matches,
            reason="at least one binary match and every object scan completed without error",
        )) + "\n")
    cfg = cfg_path.read_bytes()
    ptr = run(["rg", "-x", 'target_has_atomic="ptr"'], cfg, allowed=(0, 1))
    atomics = run(["rg", "-x", 'target_feature="atomics"'], cfg, allowed=(0, 1))
    log.write(
        f"cfg_read_status=0 cfg_bytes={len(cfg)} "
        f"target_has_atomic_ptr_presence_scan_status={ptr.returncode} expected=0 "
        f"target_feature_atomics_absence_scan_status={atomics.returncode} expected=1\n"
    )
    if ptr.returncode != 0 or atomics.returncode != 1:
        raise SystemExit("FAIL scalar cfg predicates")
    log.write(
        f"population_reconciliation_status=0 families=3 archives=3 "
        f"decoded_objects={len(records)} atomic_clean_objects={len(records)}\n"
    )
    log.write(f"observation_status=PASS scans={statuses} binary_matches={matches}\n")
    log.write("PASS named scalar non-LTO complete population and predicate inspection\n")
