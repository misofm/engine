import pathlib,subprocess,os,json,hashlib,sys
root=pathlib.Path(sys.argv[1]); label=sys.argv[2]; out=pathlib.Path('/tmp/issue478-root-evidence')
env=os.environ.copy();env['PATH']='/home/bl/.cargo/bin:'+env['PATH'];env['RUSTC_BOOTSTRAP']='1';env['RUSTFLAGS']='-C target-feature=+avx2,+fma -Zprint-type-sizes';env['CARGO_TARGET_DIR']=f'/tmp/issue478-layout-{label}'
argv=['cargo','build','--locked','-p','graph','--features','test-support','--message-format=json']
paths=['crates/rack/src/lib.rs','crates/graph/src/runtime.rs','Cargo.lock','.cargo/config.toml','rust-toolchain.toml']
meta=dict(argv=argv,cwd=str(root),head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),status=subprocess.check_output(['git','status','--porcelain'],cwd=root,text=True),env={k:env[k] for k in ['RUSTC_BOOTSTRAP','RUSTFLAGS','CARGO_TARGET_DIR']},compiler=subprocess.check_output(['rustc','-Vv'],env=env,text=True),source={p:hashlib.sha256((root/p).read_bytes()).hexdigest() for p in paths})
(out/f'{label}.command.json').write_text(json.dumps(meta,indent=2)+'\n')
with (out/f'{label}.stdout').open('xb') as stdout,(out/f'{label}.stderr').open('xb') as stderr:
 p=subprocess.run(argv,cwd=root,env=env,stdout=stdout,stderr=stderr)
(out/f'{label}.status').write_text(str(p.returncode)+'\n')
artifacts=[]
for line in (out/f'{label}.stdout').read_text().splitlines():
 if not line.startswith('{'): continue
 try: event=json.loads(line)
 except ValueError: continue
 if event.get('reason')=='compiler-artifact' and event['target']['name'] in ['rack','graph']:
  artifacts += [dict(path=f,sha256=hashlib.sha256(pathlib.Path(f).read_bytes()).hexdigest()) for f in event['filenames']]
(out/f'{label}.artifacts.json').write_text(json.dumps(artifacts,indent=2)+'\n')
print(label,p.returncode,len(artifacts));sys.exit(p.returncode)
