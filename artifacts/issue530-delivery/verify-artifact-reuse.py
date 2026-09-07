from pathlib import Path
import hashlib, json, subprocess
old=Path('/tmp/issue524-delivery/resumed-artifact')
new=Path('/tmp/issue530-delivery/artifact')
old_names=sorted(p.name for p in old.iterdir())
new_names=sorted(p.name for p in new.iterdir())
assert len(old_names)==6 and old_names==new_names, (old_names,new_names)
rows=[]
for name in new_names:
 a,b=old/name,new/name
 assert a.read_bytes()==b.read_bytes(), name
 rows.append({'name':name,'sha256':hashlib.sha256(b.read_bytes()).hexdigest(),'bytes':b.stat().st_size})
candidate='0df067cbc8366607d2dcec7912364aa437c7f573'
paths=['hosts/host-web','tests/browser-v1','scripts','crates/capi','tools/parameter-metadata','crates/effect-contract','.cargo','rust-toolchain.toml','.github/workflows/qualification.yml']
diff=subprocess.check_output(['git','diff','--name-only',candidate,'HEAD','--',*paths],text=True)
# The browser run itself updated only these recorded results; all execution inputs must be identical.
allowed={'hosts/host-web/qualification/results.json','hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md'}
changed=set(diff.splitlines())
assert not changed-allowed, changed
print(json.dumps({'actual_browser_candidate':candidate,'current_head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'six_files':rows,'input_diff':sorted(changed),'evidence':'Reuses prior actual three-browser run; this command runs no browser.'},indent=2))
