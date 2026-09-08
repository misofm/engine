import subprocess,os,json,hashlib,datetime,pathlib,sys
E=pathlib.Path('/home/bl/misofm/engine-issue623-host-rate-artifact/artifacts/issue623-scratch-qualification'); R=pathlib.Path('/home/bl/misofm/engine-issue623-artifact-scratch'); O=pathlib.Path('/tmp/issue623-qualified-output')
SOURCE='ca5a8b492a41ba85b3e90d8dedb2b49b787f2f00'; DIGEST='ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3'
env=dict(os.environ,CARGO_TARGET_DIR='/tmp/issue623-qualification-target'); env.pop('MISO_ENGINE_WEB_AUDIOWORKLET_REPIN',None);env.pop('MISO_ENGINE_WEB_STRIP',None)
commands=[]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=R,text=True)
def run(name,args,extra=None):
 en=dict(env);en.update(extra or {});rec={'name':name,'argv':args,'cwd':str(R),'head':git('rev-parse','HEAD').strip(),'start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'env':{k:v for k,v in en.items() if k.startswith(('CARGO','RUST','MISO_ENGINE_WEB','PLAYWRIGHT')) or k in ('PATH','NODE_OPTIONS')}}
 (E/(name+'.command.json')).write_text(json.dumps(rec,indent=2)+'\n')
 with (E/(name+'.stdout')).open('wb') as out,(E/(name+'.stderr')).open('wb') as err: result=subprocess.run(args,cwd=R,env=en,stdout=out,stderr=err)
 rec['status']=result.returncode;rec['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();commands.append(rec);(E/(name+'.status')).write_text(str(result.returncode)+'\n');(E/'commands.json').write_text(json.dumps(commands,indent=2)+'\n');print(name,result.returncode,flush=True)
 if result.returncode:raise RuntimeError(name+' failed; no retry')
try:
 assert git('rev-parse','HEAD').strip()==SOURCE
 status=git('status','--porcelain');(E/'pre-status.txt').write_text(status);assert not status
 (E/'inputs.sha256').write_text(''.join(sha(R/p)+'  '+p+'\n' for p in ['Cargo.lock','Cargo.toml','rust-toolchain.toml','.cargo/config.toml','scripts/build-web-audioworklet.sh','crates/effect-contract/src/lib.rs','crates/effect-package/src/wire.rs',*['hosts/host-web/web/'+x for x in ['miso-engine-v1-audio-worklet.js','miso-engine-v1-audio-worklet-host.js','miso-engine-v1-audio-worklet-host.d.ts','miso-engine-v1-audio-worklet-artifact.sha256']]]))
 run('00-toolchain',['rustc','-vV'])
 pin='hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256';(R/pin).write_text(DIGEST+'\n')
 assert git('diff','--name-only').splitlines()==[pin];(E/'first-overlay.diff').write_text(git('diff'))
 run('01-build',['bash','scripts/build-web-audioworklet.sh',str(O)])
 prior={pathlib.Path(l.split()[1]).name:l.split()[0] for l in (R/'artifacts/issue619-postpin/six-file.sha256').read_text().splitlines()}
 actual={p.name:sha(p) for p in O.iterdir()};assert set(actual)==set(prior) and len(actual)==6
 wasm='miso-engine-v1-audio-worklet.simd128.wasm';assert actual[wasm]==DIGEST
 assert all(actual[n]==v for n,v in prior.items() if n!=wasm)
 (E/'six-file.sha256').write_text(''.join(v+'  '+str(O/n)+'\n' for n,v in sorted(actual.items())))
 (E/'identity.json').write_text(json.dumps({'candidate':DIGEST,'non_wasm_identical':True,'files':actual},indent=2)+'\n')
 results='hosts/host-web/qualification/results.json';old=json.loads((R/results).read_text());new=dict(old,candidateCommit=SOURCE,wasmSha256=DIGEST);(R/results).write_text(json.dumps(new,indent=2)+'\n')
 run('02-matrix-generate',['node','hosts/host-web/qualification/generate-matrix.mjs'])
 assert set(git('diff','--name-only').splitlines())=={pin,results,'hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md'}
 check=json.loads((R/results).read_text());assert {k:v for k,v in check.items() if k not in ('candidateCommit','wasmSha256')}=={k:v for k,v in old.items() if k not in ('candidateCommit','wasmSha256')}
 (E/'second-overlay.diff').write_text(git('diff'))
 run('03-static',['bash','scripts/check-web-audioworklet.sh',str(O)])
 run('04-resources',['python3','-B','scripts/check-browser-expected-resources.py','--artifacts',str(O)])
 run('05-hermetic',['bash','scripts/test-web-audioworklet.sh'],{'CARGO_TARGET_DIR':'/tmp/issue623-hermetic-target'})
 run('06-sdk-install',['npm','--prefix','sdk','ci','--ignore-scripts'])
 run('07-sdk',['bash','scripts/sdk-package.sh','check',str(O)])
 run('08-browser-install',['npm','--prefix','hosts/host-web/qualification','ci','--ignore-scripts'])
 run('09-browsers',['npm','--prefix','hosts/host-web/qualification','run','qualify','--','--artifacts',str(O),'--browser','all','--check-matrix','--self-test-mutations'])
 run('10-matrix-check',['node','hosts/host-web/qualification/generate-matrix.mjs','--check'])
 assert set(git('status','--porcelain').splitlines())=={' M '+pin,' M '+results,' M hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md'}
 verdict='PASS'
except Exception as ex:
 verdict='FAIL: '+str(ex);print(verdict,flush=True)
finally:
 (E/'final-status.txt').write_text(git('status','--porcelain'));(E/'final-overlay.diff').write_text(git('diff'))
 (E/'README.md').write_text('# Issue623 scratch qualification\n\n'+verdict+'\n\nFrozen source '+SOURCE+'; no retries. Commands and raw statuses/streams are adjacent. Output and build targets are retained here; checksum manifest excludes build targets and itself.\n')
 files=[p for p in E.rglob('*') if p.is_file() and not any(x in ('target','hermetic-target') for x in p.relative_to(E).parts) and p.name!='sha256sums.txt']
 (E/'sha256sums.txt').write_text(''.join(sha(p)+'  '+str(p.relative_to(E))+'\n' for p in sorted(files)))
 sys.exit(0 if verdict=='PASS' else 1)
