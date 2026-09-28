#!/usr/bin/env python3
"""For each test, find other tests whose catch set contains its own (its dominators).

usage: dominance.py <crate> <matrix.json> [--exclude-file SUBSTR ...]
A test with a single dominator adds no mutant discrimination over that dominator *for these
mutants*. It is a removal candidate only when its claim is not about another crate's code or a
property no mutant exercises (allocation, timing); the issue gates say so case by case.
"""
import collections, csv, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
CLASSES = os.path.join(HERE, '..', 'data', 'test-classes.tsv')
crate, mpath = sys.argv[1], sys.argv[2]
excl = sys.argv[sys.argv.index('--exclude-file') + 1:] if '--exclude-file' in sys.argv else []
times, rand = {}, set()
for r in csv.DictReader(open(CLASSES), delimiter='\t'):
    if r['crate'] == crate:
        times[r['name']] = float(r['local_exec_s'] or 0)
        if 'randomized' in r['flags']:
            rand.add(r['name'])
m = json.load(open(mpath))
sets = collections.defaultdict(set)
for k, v in m['mutants'].items():
    if v['summary'] == 'CaughtMutant' and not any(e in v['file'] for e in excl):
        for _, n in v['failing']:
            sets[n.split('::')[-1]].add(k)
out = []
for t, s in sets.items():
    doms = [u for u, su in sets.items() if u != t and s <= su]
    cheapest = min(doms, key=lambda u: times.get(u, 0)) if doms else None
    out.append((times.get(t, 0), t, len(s), cheapest, [u for u in doms if u in rand][:2]))
out.sort(reverse=True)
dom = [o for o in out if o[3]]
print(f'{crate}: {len(sets)} catching tests; {len(dom)} have a single dominator; '
      f'{sum(o[0] for o in dom):.1f} of {sum(o[0] for o in out):.1f} local seconds are in dominated tests')
for o in out[:30]:
    print(f'   {o[0]:6.2f}s {o[1]} catches={o[2]} dominated_by={o[3]} '
          f'({times.get(o[3], 0):.2f}s) by_randomized={o[4]}')
