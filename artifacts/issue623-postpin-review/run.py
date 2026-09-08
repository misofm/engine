import os,subprocess,json,pathlib,hashlib,datetime
p=pathlib.Path('/home/bl/misofm/engine-issue623-host-rate-artifact/artifacts/issue623-postpin-review'); w='/home/bl/misofm/engine-issue623-host-rate-artifact'; env=os.environ.copy();env['CARGO_TARGET_DIR']='/tmp/issue623-postpin-review-target'
commands=[['bash','scripts/check-web-audioworklet.sh','/tmp/issue623-postpin-output'],['python3','-B','scripts/check-browser-expected-resources.py','--artifacts','/tmp/issue623-postpin-output'],['bash','scripts/test-web-audioworklet.sh'],['npm','--prefix','sdk','ci','--ignore-scripts'],['bash','scripts/sdk-package.sh','check','/tmp/issue623-postpin-output'],['node','hosts/host-web/qualification/generate-matrix.mjs','--check'],['cargo','fmt','--all','--check'],['git','diff','--check','origin/main...HEAD'],['bash','scripts/check-workspace-policy.sh'],['bash','scripts/check-effect-runtime-policy.sh'],['bash','scripts/check-host-core-policy.sh']]
records=[]
try:
 for i,cmd in enumerate(commands):
  name=f'{i:02d}'; rec=dict(argv=cmd,cwd=w,head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=w,text=True).strip(),start=datetime.datetime.now(datetime.timezone.utc).isoformat(),env={k:v for k,v in env.items() if k.startswith(('CARGO','RUST','MISO','PATH','NODE'))});(p/(name+'.command.json')).write_text(json.dumps(rec,indent=2)+'\n')
  with (p/(name+'.stdout')).open('w') as out,(p/(name+'.stderr')).open('w') as err:r=subprocess.run(cmd,cwd=w,env=env,stdout=out,stderr=err)
  rec['status']=r.returncode;records.append(rec);(p/(name+'.status')).write_text(str(r.returncode)+'\n');print(name,r.returncode,flush=True)
  if r.returncode:break
finally:
 (p/'commands.json').write_text(json.dumps(records,indent=2)+'\n')
 (p/'sha256sums.txt').write_text(''.join(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f.relative_to(p))+'\n' for f in sorted(p.iterdir()) if f.is_file() and f.name!='sha256sums.txt'))
