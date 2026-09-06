from pathlib import Path
import os,subprocess,json,sys
w=Path('/home/bl/misofm/engine-rt5-ramp-plan');q=w/'hosts/host-web/qualification';out='/tmp/engine-496-qualified';head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=w,text=True).strip();env=os.environ.copy();env['PATH']='/home/bl/.cargo/bin:'+env['PATH'];env['CARGO_TARGET_DIR']='/tmp/engine-496-artifact-target'
steps=[('builder',w,['bash','scripts/build-web-audioworklet.sh',out]),('static',w,['bash','scripts/check-web-audioworklet.sh',out]),('resource',w,['python3','scripts/check-browser-expected-resources.py','--artifacts',out]),('hermetic',w,['bash','scripts/test-web-audioworklet.sh']),('npm-ci',q,['npm','ci','--ignore-scripts']),('browser',q,['npm','run','qualify','--','--artifacts',out,'--browser','all','--record-matrix','--candidate-commit',head,'--self-test-mutations']),('matrix',q,['npm','run','matrix','--','--check'])]
for name,cwd,args in steps:
 p=Path('/tmp/496-qualified-'+name);p.with_suffix('.command.json').write_text(json.dumps({'argv':args,'cwd':str(cwd),'source':head,'env_overrides':{'PATH_prefix':'/home/bl/.cargo/bin','CARGO_TARGET_DIR':env['CARGO_TARGET_DIR']}},indent=2)+'\n')
 with p.with_suffix('.log').open('xb') as f:r=subprocess.run(args,cwd=cwd,env=env,stdout=f,stderr=subprocess.STDOUT)
 p.with_suffix('.status').write_text(str(r.returncode)+'\n');print(name,r.returncode,flush=True)
 if r.returncode:sys.exit(r.returncode)
