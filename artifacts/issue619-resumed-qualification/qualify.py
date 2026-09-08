import subprocess,os,json,hashlib,datetime,pathlib,sys
E=pathlib.Path('/tmp/issue619-resumed-qualification'); R=pathlib.Path('/home/bl/misofm/engine-issue619-artifact-scratch'); O=pathlib.Path('/tmp/issue617-scratch-qualification/output')
SOURCE='d63bc437e948d6284b1b6cbe459f0b46c4ed6566'; DIGEST='f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664'
env=dict(os.environ,CARGO_TARGET_DIR=str(E/'target')); env.pop('MISO_ENGINE_WEB_AUDIOWORKLET_REPIN',None);env.pop('MISO_ENGINE_WEB_STRIP',None)
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
 assert git('rev-parse','HEAD').strip()=='794d0dddf17370098f832607766819f73fc77d26'
 assert subprocess.run(['git','merge-base','--is-ancestor',SOURCE,'HEAD'],cwd=R).returncode==0
 pin='hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256';results='hosts/host-web/qualification/results.json';matrix='hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md'
 assert set(git('status','--porcelain').splitlines())=={' M '+pin,' M '+results,' M '+matrix}
 for name in [pin,results,matrix]:assert (R/name).read_bytes()==(pathlib.Path('/home/bl/misofm/engine-issue617-artifact-scratch')/name).read_bytes()
 evidence=R/'artifacts/issue617-scratch-qualification'
 rows=(evidence/'sha256sums.txt').read_text().splitlines();assert len(rows)==28
 for line in rows:
  h,n=line.split(maxsplit=1);assert sha(evidence/n)==h
 identity=json.loads((evidence/'identity.json').read_text());actual={f.name:sha(f) for f in O.iterdir()};assert actual==identity['files'] and len(actual)==6
 assert actual['miso-engine-v1-audio-worklet.simd128.wasm']==DIGEST
 prior={pathlib.Path(l.split()[1]).name:l.split()[0] for l in (R/'artifacts/issue539-cp8-artifact-decision/delivered-six.sha256').read_text().splitlines()}
 assert all(v==actual[n] for n,v in prior.items() if not n.endswith('.wasm'))
 old=json.loads(git('show','HEAD:'+results));new=json.loads((R/results).read_text());assert new==dict(old,candidateCommit=SOURCE,wasmSha256=DIGEST)
 (E/'identity.json').write_text(json.dumps({'head':git('rev-parse','HEAD').strip(),'source':SOURCE,'files':actual,'manifest_entries':28},indent=2)+'\n')
 (E/'pre-overlay.diff').write_text(git('diff'))
 run('03-static',['bash','scripts/check-web-audioworklet.sh',str(O)])
 run('04-resources',['python3','-B','scripts/check-browser-expected-resources.py','--artifacts',str(O)])
 run('05-hermetic',['bash','scripts/test-web-audioworklet.sh'],{'CARGO_TARGET_DIR':str(E/'hermetic-target')})
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
 (E/'README.md').write_text('# Issue619 scratch qualification\n\n'+verdict+'\n\nFrozen source '+SOURCE+'; no retries. Commands and raw statuses/streams are adjacent. Output and build targets are retained here; checksum manifest excludes build targets and itself.\n')
 files=[p for p in E.iterdir() if p.is_file() and p.name!='sha256sums.txt']
 (E/'sha256sums.txt').write_text(''.join(sha(p)+'  '+str(p.relative_to(E))+'\n' for p in sorted(files)))
 sys.exit(0 if verdict=='PASS' else 1)
