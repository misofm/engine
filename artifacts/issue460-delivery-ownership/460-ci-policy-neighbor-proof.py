from pathlib import Path
import subprocess,tempfile,json
repo=Path('/home/bl/misofm/engine-140-ownership')
s=(repo/'scripts/test-realtime-policy.sh').read_text()
fixture=s[s.index('create_fixture() {'):s.index('\nexpect_failure() {')]
with tempfile.TemporaryDirectory(prefix='engine460-policy-') as tmp:
 root=Path(tmp)
 subprocess.run(['bash','-euc',fixture+'\ncreate_fixture "$1"','fixture',str(root)],check=True)
 tests=root/'crates/protocol/tests';tests.mkdir(parents=True)
 result=[]
 for name,want in [('delivery_ownership.rs',0),('delivery_ownershipXrs.rs',1),('delivery_ownership_extra.rs',1)]:
  path=tests/name;path.write_text('unsafe fn unapproved_neighbor() {}\n')
  p=subprocess.run(['bash',str(repo/'scripts/check-realtime-policy.sh'),str(root)],text=True,capture_output=True)
  print(name,p.returncode,p.stdout,p.stderr,flush=True)
  assert p.returncode==want
  if want:assert 'unsafe code exists outside the issue-approved ownership/audit files' in p.stderr and name in p.stderr
  result.append({'name':name,'status':p.returncode});path.unlink()
 print(json.dumps(result))
