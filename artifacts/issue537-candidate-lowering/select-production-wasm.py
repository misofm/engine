from pathlib import Path
import hashlib,json,re
root=Path('/tmp/issue537-root-lowering')
records=[]
for backend,ids in [('scalar',[761]),('simd128',[811,813])]:
    source=root/f'decode-{backend}.stdout'
    lines=source.read_bytes().splitlines(keepends=True)
    headers=[]
    for i,line in enumerate(lines):
        m=re.match(rb'^([0-9a-f]+) func\[(\d+)\] <(.+)>:',line)
        if m: headers.append((i,int(m.group(2)),m.group(1).decode(),m.group(3).decode()))
    for ident in ids:
        positions=[k for k,v in enumerate(headers) if v[1]==ident];assert len(positions)==1
        k=positions[0];start,_,address,name=headers[k];end=headers[k+1][0] if k+1<len(headers) else len(lines)
        assert 'PreparedMultibandCompressor' in name
        data=b''.join(lines[start:end]);out=root/f'{backend}-prepared-multiband-func-{ident}.wat.txt'
        with out.open('xb') as f:f.write(data)
        records.append({'backend':backend,'function_index':ident,'start_address':address,'symbol':name,'original':str(source),'lines':[start+1,end],'selected':str(out),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
originals={}
paths=[*Path('/tmp/issue537-candidate-target/release/deps').glob('multiband_compressor-*.ll'),*Path('/tmp/issue537-candidate-target/release/deps').glob('multiband_compressor-*.s')]
for backend in ['scalar','simd128']:
    paths += [Path(f'/tmp/issue537-web-{backend}/wasm32-unknown-unknown/release/host_web.wasm'),root/f'decode-{backend}.stdout',root/f'metadata-{backend}.stdout']
for p in paths:
    b=p.read_bytes();originals[str(p)]={'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
(root/'production-wasm-identity.json').write_text(json.dumps({'source_head':'13d0580b66c287edc412090dfa7174a2800e05b1','selection':records,'limits':'Linked release inspection modules, not published artifact identities. Scalar Prepared effect and SIMD128 scalar/SIMD4 bank selected. Emitted W8 Wasm/mono wrappers not credited as supported dispatch.'},indent=2)+'\n')
(root/'original-output-identities.json').write_text(json.dumps(originals,indent=2)+'\n')
for r in records: print(r['backend'],r['function_index'],r['lines'],r['bytes'],r['sha256'])
