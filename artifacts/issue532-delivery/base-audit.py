import json, subprocess

def git(*args): return subprocess.check_output(['git',*args],text=True).strip()
head=git('rev-parse','HEAD')
base=git('ls-remote','origin','refs/heads/main').split()[0]
assert not git('status','--porcelain')
assert base=='f39c9fa74016d4c0d460778a9c787668cbeb1e70',base
assert git('merge-base',base,head)==base
allowed={'crates/host-core/src/scalar_point_endpoint.rs','crates/host-core/src/lib.rs','crates/host-core/tests/scalar_point_endpoint.rs'}
changed=git('diff','--name-only',base,head).splitlines()
assert all(p in allowed or p.startswith(('artifacts/issue532-','.github/ISSUE_SPECS/532-')) for p in changed)
print(json.dumps({'head':head,'current_main':base,'source_changes':sorted(set(changed)&allowed),'evidence_file_count':len(changed)-len(set(changed)&allowed)},indent=2))
