import json, subprocess

def git(*args): return subprocess.check_output(['git',*args],text=True).strip()
head=git('rev-parse','HEAD')
base=git('ls-remote','origin','refs/heads/main').split()[0]
assert not git('status','--porcelain')
assert base=='fe58709c04c880b14dd72f9bd4a25ef5f7071202',base
assert git('merge-base',base,head)==base
allowed={'crates/gate-expander/src/kernel.rs','crates/gate-expander/src/lib.rs','crates/gate-expander/tests/identity.rs','crates/gate-expander/tests/state.rs'}
changed=git('diff','--name-only',base,head).splitlines()
assert all(p in allowed or p.startswith(('artifacts/issue534-','.github/ISSUE_SPECS/534-')) for p in changed)
print(json.dumps({'head':head,'current_main':base,'source_changes':sorted(set(changed)&allowed),'evidence_file_count':len(changed)-len(set(changed)&allowed)},indent=2))
