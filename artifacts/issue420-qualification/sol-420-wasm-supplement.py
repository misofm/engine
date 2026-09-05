import subprocess, pathlib, tempfile, hashlib, json, sys
records=[
('engine','/tmp/sol420-nonlto.m3INqF/wasm32-unknown-unknown/release/deps/libengine-43010e1ae45ff1b3.rlib','engine-43010e1ae45ff1b3.engine.3cdc5b765e4ae52f-cgu.0.rcgu.o','2f6ea4ab8bbb9b474f4d499fb70d029d2ae1ea5ddec86acbfe516a3ac24ab687'),
('source','/tmp/sol420-nonlto.m3INqF/wasm32-unknown-unknown/release/deps/libsource-0fdfa2234eb4bd89.rlib','source-0fdfa2234eb4bd89.source.7b5fcfd726247e1-cgu.0.rcgu.o','de8a478abc88d1ee7c379ee795e7c73f4d3b170e6df1e3dc5247a2edcc9c8353'),
('target_smoke','/tmp/sol420-nonlto.m3INqF/wasm32-unknown-unknown/release/deps/libtarget_smoke-f8895a03d6c8d18b.rlib','target_smoke-f8895a03d6c8d18b.target_smoke.d196ae6c3bbfe597-cgu.0.rcgu.o','c9c2a2d4b51f0ed1e186057f1726d123e263487de37b42b69d67a3383009a5d0')]
def run(args,data=None): return subprocess.run(args,input=data,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
log=pathlib.Path('/tmp/sol-420-wasm-supplement.log')
statuses=[]; matches=0
with log.open('w') as f:
 f.write('source_sha=51e2aed211b30523076e0e8dd07973b13b57dc11\nnonlto_target=/tmp/sol420-nonlto.m3INqF\nobservation_pattern=observe options=rg -l --binary\n')
 for family,archive,member,expected in records:
  p=run(['ar','p',archive,member]); actual=hashlib.sha256(p.stdout).hexdigest()
  f.write(json.dumps(dict(family=family,archive=archive,member=member,member_read_status=p.returncode,bytes=len(p.stdout),expected_sha256=expected,actual_sha256=actual,hash_matches=(actual==expected),member_stderr=p.stderr.decode(errors='replace')))+'\n')
  if p.returncode!=0 or not p.stdout or actual!=expected: raise SystemExit(f'member verification failed: {family}')
  path=pathlib.Path(tempfile.mkstemp(prefix=f'sol420-observe-{family}-',suffix='.o')[1]); path.write_bytes(p.stdout)
  q=run(['rg','-l','--binary','observe',str(path)])
  f.write(json.dumps(dict(family=family,observation_scan_status=q.returncode,stdout=q.stdout.decode(errors='replace'),stderr=q.stderr.decode(errors='replace')))+'\n')
  statuses.append(q.returncode)
  if q.returncode==0: matches+=1
 if any(s not in (0,1) for s in statuses): raise SystemExit(f'observation scan execution error: {statuses}')
 if all(s==1 for s in statuses):
  q=run(['rg','-q','ObservationSlot','crates/engine/src/realtime/observe.rs'])
  f.write(json.dumps(dict(source_fallback_invoked=True,source_pattern='ObservationSlot',options='rg -q',source_status=q.returncode,stdout=q.stdout.decode(errors='replace'),stderr=q.stderr.decode(errors='replace')))+'\n')
  if q.returncode!=0: raise SystemExit(f'source fallback failed/status error: {q.returncode}')
 else:
  f.write(json.dumps(dict(source_fallback_invoked=False,binary_match_count=matches,reason='at least one binary match and all three scans completed without error'))+'\n')
 f.write(f'observation_status=PASS scans={statuses} binary_matches={matches}\n')
