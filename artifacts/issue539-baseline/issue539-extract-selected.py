from pathlib import Path
import hashlib
import json
import re

base = Path('/tmp/issue539-baseline-target/release/deps')
ll_path = next(base.glob('true_peak_limiter-*.ll'))
s_path = next(base.glob('true_peak_limiter-*.s'))
out = Path('/tmp/issue539-baseline/selected')
out.mkdir(parents=True, exist_ok=True)

# Actual native production callers and the core outlined dual callee.  The W8 mono
# implementation is inlined into its process_bank_mono caller in this release.
# The W8 dual uniform/per-lane bodies are inlined into LimiterCore::process_block.
names = {
    'process-scalar': '_RNvXsd_CsdvPQf9CMsz3_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process',
    'process-core-scalar': '_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_',
    'process-bank-w8': '_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_',
    'process-bank-mono-w8': '_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank17process_bank_monoB5_',
    'process-core-w8': '_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_',
}

ll = ll_path.read_text().splitlines()
asm = s_path.read_text().splitlines()
manifest=[]
for label,name in names.items():
    starts=[i for i,line in enumerate(ll) if line.startswith('define ') and re.search(r'@'+re.escape(name)+r'\(', line)]
    assert len(starts)==1,(label,starts)
    i=starts[0]; depth=ll[i].count('{')-ll[i].count('}')
    j=i
    while depth:
        j+=1; depth += ll[j].count('{')-ll[j].count('}')
    lp=out/(label+'.ll'); lp.write_text('\n'.join(ll[i:j+1])+'\n')
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
