import subprocess, pathlib, tempfile, hashlib, json, sys
base=pathlib.Path(sys.argv[1])/'wasm32-unknown-unknown/release/deps'
out=pathlib.Path(sys.argv[2])
rows=[]
def run(args, data=None, allowed=(0,)):
 p=subprocess.run(args,input=data,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 if p.returncode not in allowed:
  raise SystemExit(f'FAIL command={args!r} status={p.returncode} stderr={p.stderr.decode(errors="replace")}')
 return p
with out.open('w') as log:
 log.write(f'scope=named scalar non-LTO engine/source/target_smoke archives\nbase={base}\n')
 for family in ('engine','source','target_smoke'):
  f=run(['find',str(base),'-maxdepth','1','-type','f','-name',f'lib{family}-*.rlib','-print'])
  log.write(f'family={family} find_status={f.returncode} find_bytes={len(f.stdout)}\n')
  s=run(['sort'],f.stdout)
  archives=s.stdout.decode().splitlines()
  log.write(f'family={family} sort_status={s.returncode} archive_count={len(archives)} archives={archives!r}\n')
  if len(archives)!=1: raise SystemExit(f'FAIL {family} archive count {len(archives)}')
  archive=archives[0]
  a=run(['ar','t',archive]); members=a.stdout.decode().splitlines()
  objects=[m for m in members if m.endswith('.o')]
  log.write(f'family={family} archive_list_status={a.returncode} member_count={len(members)} object_count={len(objects)} duplicate_objects={len(objects)-len(set(objects))}\n')
  if not objects or len(objects)!=len(set(objects)): raise SystemExit(f'FAIL {family} object population')
  for index,member in enumerate(objects):
   m=run(['ar','p',archive,member]);
   if not m.stdout: raise SystemExit(f'FAIL empty member {member}')
   extracted=pathlib.Path(tempfile.mkstemp(prefix=f'sol420-{family}-',suffix='.o')[1]); extracted.write_bytes(m.stdout)
   d=run(['wasm-objdump','-d',str(extracted)])
   decoded=extracted.with_suffix('.decoded'); decoded.write_bytes(d.stdout)
   scan=run(['rg','-n','atomic\\.',str(decoded)],allowed=(0,1))
   log.write(json.dumps(dict(family=family,archive=archive,archive_member_index=index,member=member,member_read_status=m.returncode,member_bytes=len(m.stdout),sha256=hashlib.sha256(m.stdout).hexdigest(),decoder_status=d.returncode,decoded_bytes=len(d.stdout),atomic_scan_status=scan.returncode))+"\n")
   if scan.returncode!=1: raise SystemExit(f'FAIL atomic opcode or scan error {member}: {scan.returncode}')
   rows.append((family,archive,member))
 log.write(f'population_reconciliation_status=0 families=3 archives=3 decoded_objects={len(rows)} atomic_clean_objects={len(rows)}\n')
 log.write('PASS named scalar non-LTO complete archive/object population\n')
