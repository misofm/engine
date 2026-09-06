from pathlib import Path
import subprocess,json,hashlib,os
root=Path('/home/bl/misofm/engine-idle-admission-clear');p=root/'hosts/host-web/src/tests.rs';original=p.read_bytes()
probe=b'''\n#[test]\nfn issue514_native_layout_probe() {\n    println!("ReadyOwnership size={} align={}", size_of::<ReadyOwnership>(), core::mem::align_of::<ReadyOwnership>());\n    println!("OptionReadyOwnership size={} align={}", size_of::<Option<ReadyOwnership>>(), core::mem::align_of::<Option<ReadyOwnership>>());\n    println!("AudioWorkletEngineHost size={} align={}", size_of::<AudioWorkletEngineHost>(), core::mem::align_of::<AudioWorkletEngineHost>());\n    let host = prepared_host(128);\n    let r = host.resources();\n    println!("bridge_metadata_bytes={} bridge_retained_bytes={} largest_bridge_allocation_bytes={} largest_named_allocation_bytes={}", r.bridge_metadata_bytes, r.bridge_retained_bytes, r.largest_bridge_allocation_bytes, r.largest_named_allocation_bytes);\n}\n'''
assert subprocess.check_output(['git','status','--porcelain'],cwd=root,text=True)==''
env=os.environ.copy();env['PATH']='/home/bl/.cargo/bin:'+env['PATH'];argv=['cargo','test','--locked','-p','host-web','--lib','tests::issue514_native_layout_probe','--','--exact','--nocapture']
p.write_bytes(original+probe)
meta={'argv':argv,'cwd':str(root),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'env':{k:env.get(k) for k in ['PATH','RUSTFLAGS','CARGO_TARGET_DIR']},'original_test_sha256':hashlib.sha256(original).hexdigest(),'temporary_test_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'production_lib_sha256':hashlib.sha256((root/'hosts/host-web/src/lib.rs').read_bytes()).hexdigest(),'note':'Temporary test-only native layout probe; no production change. Not shipped-Wasm layout evidence.'}
Path('/tmp/514-baseline-layout.command.json').write_text(json.dumps(meta,indent=2)+'\n');Path('/tmp/514-baseline-layout.probe.rs').write_bytes(probe)
try:
 with open('/tmp/514-baseline-layout.stdout','wb') as out,open('/tmp/514-baseline-layout.stderr','wb') as err:r=subprocess.run(argv,cwd=root,env=env,stdout=out,stderr=err)
 Path('/tmp/514-baseline-layout.status').write_text(str(r.returncode)+'\n')
finally:
 assert p.read_bytes()==original+probe,'unexpected concurrent source mutation'
 p.write_bytes(original)
 Path('/tmp/514-baseline-layout.restoration.json').write_text(json.dumps({'restored_test_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'status':subprocess.check_output(['git','status','--porcelain'],cwd=root,text=True)},indent=2)+'\n')
print('layout probe exit',r.returncode)
