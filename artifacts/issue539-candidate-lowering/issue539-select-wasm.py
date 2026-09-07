from pathlib import Path
import hashlib,json,re
root=Path('/tmp/issue539-root-lowering')
records=[]
for backend,ids in [('scalar',[716,724]),('simd128',[755,756,757,767,769,770])]:
    source=root/f'{backend}-disassemble.stdout'
    lines=source.read_bytes().splitlines(keepends=True)
    headers=[]
    for i,line in enumerate(lines):
        m=re.match(rb'^([0-9a-f]+) func\[(\d+)\] <(.+)>:',line)
        if m: headers.append((i,int(m.group(2)),m.group(1).decode(),m.group(3).decode()))
    for ident in ids:
        positions=[k for k,v in enumerate(headers) if v[1]==ident];assert len(positions)==1
        k=positions[0];start,_,address,name=headers[k];end=headers[k+1][0] if k+1<len(headers) else len(lines)
        assert 'true_peak_limiter' in name
        data=b''.join(lines[start:end]);out=root/f'{backend}-limiter-production-func-{ident}.wat.txt'
        with out.open('xb') as f:f.write(data)
        records.append({'backend':backend,'function_index':ident,'start_address':address,'symbol':name,'original':str(source),'lines':[start+1,end],'selected':str(out),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
originals={}
paths=[*Path('/tmp/issue539-candidate-target/release/deps').glob('true_peak_limiter-*.ll'),*Path('/tmp/issue539-candidate-target/release/deps').glob('true_peak_limiter-*.s')]
for backend in ['scalar','simd128']:
    paths += [Path(f'/tmp/issue539-web-{backend}/wasm32-unknown-unknown/release/host_web.wasm'),root/f'{backend}-disassemble.stdout',root/f'{backend}-metadata.stdout']
for p in paths:
    b=p.read_bytes();originals[str(p)]={'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
(root/'production-wasm-identity.json').write_text(json.dumps({'source_head':'615787e92a209d7146110f7920bb9c09d7f93a3d','selection':records,'limits':'Linked release inspection modules, not published artifact identities. Actual scalar process/core and SIMD128 scalar plus SIMD4 dual/mono process/core selected. Root/callee identity follows decoder symbols; no W8 Wasm dispatch claim.'},indent=2)+'\n')
(root/'original-output-identities.json').write_text(json.dumps(originals,indent=2)+'\n')
for r in records: print(r['backend'],r['function_index'],r['lines'],r['bytes'],r['sha256'])
