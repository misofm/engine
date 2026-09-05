import json,sys
from pathlib import Path
A,B=map(lambda p:[json.loads(x) for x in Path(p).read_text().splitlines() if x.strip()],sys.argv[1:3])
key=lambda r:(r['record'],r['workload_kind'],r['round'])
ma,mb={key(r):r for r in A},{key(r):r for r in B}
print(f'rowcounts: rt2={len(A)} rt1={len(B)}; unique keyed: rt2={len(ma)} rt1={len(mb)}')
print('matching key: (record, workload_kind, round)')
print('missing from rt2:',sorted(set(mb)-set(ma))); print('missing from rt1:',sorted(set(ma)-set(mb)))
allkeys=set().union(*(set(r) for r in A+B))
stable=['schema_version','record','workload_kind','tracks','synthetic_fixture','strip_content','strip_layout','input_signal','fixture_id','backend','sample_rate_hz','quantum_frames','observations','units','percentile_method','descriptive_only','statistical_method','target_triple','target_features','profile','measurement_control']
output_fields=sorted(f for f in allkeys if f.endswith('_output_sha256') or f=='output_sha256')
counter_fields=sorted(f for f in allkeys if f in {'render_errors','render_total_forbidden_operations'} or f.endswith('_transposes_per_block'))
layout_fields=sorted(f for f in allkeys if f.endswith('_layout'))
fields=stable+layout_fields+output_fields+counter_fields
print('stable identity fields compared:',','.join(stable+layout_fields))
print('emitted digest fields compared:',','.join(output_fields))
print('emitted error/counter fields compared:',','.join(counter_fields))
diffs=[]; presence=[]
for k in sorted(set(ma)&set(mb)):
 for f in fields:
  if (f in ma[k]) != (f in mb[k]): presence.append((k,f, f in ma[k],f in mb[k]))
  elif f in ma[k] and ma[k][f]!=mb[k][f]: diffs.append((k,f,ma[k][f],mb[k][f]))
print('stable/digest/counter value mismatches:',len(diffs))
print('stable/digest/counter presence mismatches:',len(presence))
for x in diffs: print('MISMATCH',x)
for x in presence: print('PRESENCE_MISMATCH',x)
print('field presence counts (rt2,rt1):')
for f in output_fields+counter_fields: print(f,sum(f in r for r in A),sum(f in r for r in B))
for wk in ('sixty_four_track_plumbing_only','sixty_four_track_gain_pan_only'):
 print(wk)
 for label,rows in (('rt2',A),('rt1',B)):
  print(label,[(r['round'],r['p50_ns_per_block'],r['p50_us_per_block'],r['p50_us_per_block_per_track'],r['output_sha256']) for r in sorted((x for x in rows if x['workload_kind']==wk),key=lambda x:x['round'])])

assert len(A)==len(B)==len(ma)==len(mb)==46
assert set(ma)==set(mb)
assert not diffs and not presence
print("PASS: complete keyed emitted digest/counter and named stable identity comparison")
