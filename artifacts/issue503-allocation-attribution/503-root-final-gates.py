import os,subprocess,json,sys
from pathlib import Path
w=Path('/home/bl/misofm/engine-compressor-allocation-proof');env=os.environ.copy();env['PATH']='/home/bl/.cargo/bin:'+env['PATH'];env['CARGO_TARGET_DIR']='/tmp/engine-503-final-gates'
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=w,text=True).strip();assert not subprocess.check_output(['git','status','--porcelain'],cwd=w,text=True)
steps=[]
for profile in ['dev','release']:
 base=['cargo','test','--locked','-p','compressor','--test','conformance','--profile',profile]
 for name in ['scoped_allocator_attribution_controls_are_live_and_isolated','uniform_and_ragged_render_paths_allocate_and_free_nothing']:
  steps.append((profile+'-'+('control' if name.startswith('scoped') else 'render'),base+['--','--exact',name,'--nocapture']))
 steps.append((profile+'-conformance',base+['--','--nocapture']))
steps += [('clippy',['cargo','clippy','--locked','-p','compressor','--all-targets','--all-features','--','-D','warnings']),('fmt',['cargo','fmt','--all','--','--check']),('diff',['git','diff','--check']),('realtime',['bash','scripts/check-realtime-policy.sh']),('realtime-mutations',['bash','scripts/test-realtime-policy.sh'])]
for name,args in steps:
 p=Path('/tmp/503-root-final-'+name);assert not p.with_suffix('.log').exists()
 p.with_suffix('.command.json').write_text(json.dumps({'argv':args,'cwd':str(w),'source':head,'env_overrides':{k:env[k] for k in ['PATH','CARGO_TARGET_DIR']}},indent=2)+'\n')
 with p.with_suffix('.log').open('xb') as f:r=subprocess.run(args,cwd=w,env=env,stdout=f,stderr=subprocess.STDOUT)
 p.with_suffix('.status').write_text(str(r.returncode)+'\n');print(name,r.returncode,flush=True)
 if r.returncode:sys.exit(r.returncode)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=w,text=True).strip()==head
assert not subprocess.check_output(['git','status','--porcelain'],cwd=w,text=True)
