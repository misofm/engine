from pathlib import Path
import os,subprocess,json,sys
w=Path('/home/bl/misofm/engine-compressor-allocation-proof')
assert not subprocess.check_output(['git','status','--porcelain'],cwd=w,text=True)
env=os.environ.copy();env['PATH']='/home/bl/.cargo/bin:'+env['PATH'];env['CARGO_TARGET_DIR']='/tmp/engine-503-qualified';env['CARGO_INCREMENTAL']='0'
args=['cargo','test','--locked','--all-targets','-p','lane','-p','math','-p','effect-runtime','-p','delay','-p','compressor','-p','multiband-compressor','-p','gate-expander','-p','true-peak-limiter','-p','transient-shaper','-p','soft-clip','-p','parametric-eq','-p','builtins','-p','dsp-reference','-p','conformance','--features','math/lane']
p=Path('/tmp/503-root-dsp-integration')
assert not p.with_suffix('.status').exists()
p.with_suffix('.command.json').write_text(json.dumps({'argv':args,'cwd':str(w),'source':subprocess.check_output(['git','rev-parse','HEAD'],cwd=w,text=True).strip(),'env_overrides':{k:env[k] for k in ['PATH','CARGO_TARGET_DIR','CARGO_INCREMENTAL']}},indent=2)+'\n')
with p.with_suffix('.log').open('xb') as f:r=subprocess.run(args,cwd=w,env=env,stdout=f,stderr=subprocess.STDOUT)
p.with_suffix('.status').write_text(str(r.returncode)+'\n');print('original DSP integration command',r.returncode,flush=True);sys.exit(r.returncode)
