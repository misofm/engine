import subprocess, pathlib, hashlib, json
base=pathlib.Path('/tmp/sol419-nonlto-target.ORPqmw/wasm32-unknown-unknown/release/deps')
listed=[pathlib.Path(x) for x in pathlib.Path('/tmp/sol-419-nonlto-objects.txt').read_text().splitlines()]
observed=[]
def run(args,data=None):
 p=subprocess.run(args,input=data,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 assert p.returncode==0,(args,p.returncode,p.stderr)
 return p.stdout
for family in ['engine','source','target_smoke']:
 found=run(['find',str(base),'-maxdepth','1','-type','f','-name',f'lib{family}-*.rlib','-print'])
 archives=run(['sort'],found).decode().splitlines()
 assert len(archives)==1,archives
 archive=archives[0]; members=run(['ar','t',archive]).decode().splitlines()
 objects=[m for m in members if m.endswith('.o')]
 assert objects and len(objects)==len(set(objects))
 for member in objects:
  p=pathlib.Path('/tmp/sol419-objects.0jjST0')/family/member
  archived=run(['ar','p',archive,member]);assert archived==p.read_bytes(),p
  observed.append(p)
  print(json.dumps(dict(family=family,archive=archive,find_status=0,sort_status=0,list_status=0,member_read_status=0,member=member,sha256=hashlib.sha256(archived).hexdigest(),matches_previously_decoded_object=True)))
assert sorted(observed)==sorted(listed),(observed,listed)
print('PASS: checked complete archive population matches all three previously decoded/scanned objects; no rebuild')
