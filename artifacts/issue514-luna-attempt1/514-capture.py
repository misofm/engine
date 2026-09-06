from pathlib import Path
import os,sys,subprocess,json,hashlib
label=sys.argv[1];argv=sys.argv[2:];assert label and all(c.isalnum() or c=='-' for c in label);assert argv
root=Path('/home/bl/misofm/engine-idle-admission-clear');prefix=Path('/tmp/514-luna1-'+label)
paths=['hosts/host-web/src/lib.rs','hosts/host-web/src/tests.rs'];env=os.environ.copy();env['PATH']='/home/bl/.cargo/bin:'+env['PATH']
meta={'argv':argv,'cwd':str(root),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain'],cwd=root,text=True),'env':{k:env.get(k) for k in ['PATH','RUSTFLAGS','CARGO_TARGET_DIR']},'source':{p:{'sha256':hashlib.sha256((root/p).read_bytes()).hexdigest(),'git_blob':subprocess.check_output(['git','hash-object',p],cwd=root,text=True).strip()} for p in paths}}
with prefix.with_suffix('.command.json').open('x') as f:json.dump(meta,f,indent=2);f.write('\n')
with prefix.with_suffix('.stdout').open('xb') as out,prefix.with_suffix('.stderr').open('xb') as err:r=subprocess.run(argv,cwd=root,env=env,stdout=out,stderr=err)
with prefix.with_suffix('.status').open('x') as f:f.write(str(r.returncode)+'\n')
print(label,r.returncode,flush=True);sys.exit(r.returncode)
