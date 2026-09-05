import hashlib, json, os, subprocess, tempfile
from pathlib import Path
root=Path('/home/bl/misofm/engine-rt1/target/issue399-wasm-scalar-inspect/wasm32-unknown-unknown/release/deps')
work=Path(tempfile.mkdtemp(prefix='engine-399-scalar-objects.',dir='/tmp'))
report={'profile':'release; CARGO_PROFILE_RELEASE_LTO=false; -simd128','directory':str(work),'archives':[],'objects':[],'complete':False}
out=Path('/tmp/engine-399-scalar-object-report.json')
def save(): out.write_text(json.dumps(report,indent=2)+'\n')
def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()
try:
    names=[Path(e.path) for e in os.scandir(root) if e.is_file()]
    for family in ('engine','source','target_smoke'):
        archives=sorted(p for p in names if p.name.startswith('lib'+family+'-') and p.suffix=='.rlib')
        assert archives, 'missing archive family '+family
        for archive in archives:
            listing=subprocess.run(['ar','t',str(archive)],capture_output=True,text=True)
            assert listing.returncode==0, listing.stderr
            members=listing.stdout.splitlines()
            dest=work/archive.name; dest.mkdir()
            extraction=subprocess.run(['ar','x',str(archive)],cwd=dest,capture_output=True,text=True)
            assert extraction.returncode==0, extraction.stderr
            objects=sorted(p for p in dest.iterdir() if p.suffix=='.o')
            expected=sorted(m for m in members if m.endswith('.o'))
            assert sorted(p.name for p in objects)==expected and objects, 'incomplete object extraction'
            report['archives'].append({'path':str(archive),'sha256':digest(archive),'members':members,'objects':len(objects),'list_status':listing.returncode,'extract_status':extraction.returncode})
            for obj in objects:
                decoded=subprocess.run(['wasm-objdump','-d',str(obj)],capture_output=True)
                obj.with_suffix('.disassembly').write_bytes(decoded.stdout)
                obj.with_suffix('.stderr').write_bytes(decoded.stderr)
                entry={'path':str(obj),'sha256':digest(obj),'decode_status':decoded.returncode}
                report['objects'].append(entry); save()
                assert decoded.returncode==0, decoded.stderr.decode(errors='replace')
                scan=subprocess.run(['rg','-n',r'atomic\.'],input=decoded.stdout,capture_output=True)
                entry['atomic_scan_status']=scan.returncode
                entry['atomic_scan_stdout']=scan.stdout.decode(errors='replace')
                entry['atomic_scan_stderr']=scan.stderr.decode(errors='replace')
                save()
                assert scan.returncode==1, 'atomic match or scan error '+str(entry)
    assert report['objects']
    report['complete']=True; save()
    print('PASS:',len(report['archives']),'archives,',len(report['objects']),'non-LTO scalar objects; every decoder exit0 and opcode scan exit1')
    print(out)
except BaseException:
    save()
    raise
