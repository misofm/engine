from pathlib import Path
import hashlib
import json

base = Path('/tmp/issue537-baseline-target/release/deps')
ll_path = next(base.glob('multiband_compressor-*.ll'))
s_path = next(base.glob('multiband_compressor-*.s'))
out = Path('/tmp/issue537-baseline/selected')
out.mkdir(parents=True, exist_ok=True)

names = {
    'process-scalar': '_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process',
    'process-bank-w4': '_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_',
    'process-bank-w8': '_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_',
}

ll = ll_path.read_text().splitlines()
asm = s_path.read_text().splitlines()
manifest=[]
for label,name in names.items():
    # LLVM function starts at define line containing exact symbol and closes at matching brace.
    starts=[i for i,line in enumerate(ll) if line.startswith('define ') and name in line]
    assert len(starts)==1,(label,starts)
    i=starts[0]; depth=ll[i].count('{')-ll[i].count('}')
    j=i
    while depth:
        j+=1; depth += ll[j].count('{')-ll[j].count('}')
    lp=out/(label+'.ll'); lp.write_text('\n'.join(ll[i:j+1])+'\n')
    # ASM starts at symbol label; include through .size line.
    starts=[i for i,line in enumerate(asm) if line == name+':']
    assert len(starts)==1,(label,starts)
    a=starts[0]; b=next(i for i in range(a+1,len(asm)) if asm[i].startswith('\t.size\t'+name+','))
    sp=out/(label+'.s'); sp.write_text('\n'.join(asm[a:b+1])+'\n')
    for p, kind, first,last in [(lp,'llvm',i+1,j+1),(sp,'asm',a+1,b+1)]:
        manifest.append({'label':label,'kind':kind,'path':str(p),'original':str(ll_path if kind=='llvm' else s_path),'lines':[first,last],'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'symbol':name})
manifest_path=out/'manifest.json'
manifest_path.write_text(json.dumps({'source_llvm':str(ll_path),'source_asm':str(s_path),'functions':manifest},indent=2)+'\n')
print(manifest_path)
for m in manifest: print(m['label'],m['kind'],m['lines'],m['bytes'],m['sha256'])
