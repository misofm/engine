import json, subprocess

def git(*args): return subprocess.check_output(['git',*args],text=True).strip()
head=git('rev-parse','HEAD')
base=git('ls-remote','origin','refs/heads/main').split()[0]
assert not git('status','--porcelain')
assert base=='301a57c6fb9982025b6fb0a49f3d35843c3e6f60',base
assert git('merge-base',base,head)==base
allowed={'crates/protocol/src/controller.rs','crates/protocol/src/controller_delivery.rs','crates/protocol/src/delivery.rs','crates/protocol/src/lib.rs','crates/protocol/tests/delivery_ownership.rs'}
changed=git('diff','--name-only',base,head).splitlines()
assert all(p in allowed or p.startswith(('artifacts/issue530-','.github/ISSUE_SPECS/530-')) for p in changed)
print(json.dumps({'head':head,'current_main':base,'source_changes':sorted(set(changed)&allowed),'evidence_file_count':len(changed)-len(set(changed)&allowed)},indent=2))
