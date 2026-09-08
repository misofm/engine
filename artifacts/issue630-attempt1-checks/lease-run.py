from pathlib import Path
import hashlib, os, sys
lease=Path("/tmp/misofm-engine-issue630-attempt1-execution.lock/owner.txt")
expected=(b"issue=630\n" b"attempt=1\n" b"owner=/root/issue610_luna_xhigh\n" b"authorized_head=6b59637a70340105f5c11b9a6a335e4aed1bd129\n" b"scope=installed-state-preflight,sdk-check,matrix-check,cargo-fmt,diff-hygiene,workspace-policy,effect-runtime-policy\n")
if len(sys.argv)<4 or sys.argv[1] != "--tag" or sys.argv[3] != "--": raise SystemExit(2)
tag=sys.argv[2]; argv=sys.argv[4:]
raw=lease.read_bytes()
if raw != expected: raise SystemExit("canonical lease mismatch")
ev=Path("artifacts/issue630-attempt1-checks")
(ev/f"{tag}.lease.owner.txt").write_bytes(raw)
(ev/f"{tag}.lease.sha256").write_text(f"{hashlib.sha256(raw).hexdigest()}  {lease}\n")
os.execvp(argv[0], argv)
