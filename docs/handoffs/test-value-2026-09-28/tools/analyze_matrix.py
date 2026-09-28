#!/usr/bin/env python3
"""Summarise a catch matrix (from parse_mutants.py) against the claim classification.

usage: analyze_matrix.py <crate> <matrix.json> [--exclude-file SUBSTR ...]
Prints the mutation score, how many tests catch each caught mutant, a greedy minimal covering set,
catches by claim class and by the `randomized` flag, unique catchers, zero-catch tests, and the
slowest tests with their catches. --exclude-file drops mutants in fixture generators (the audit
excluded `src/corpus.rs` for compressor and `src/bin/` for graph-compiler).
"""
import collections, csv, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
CLASSES = os.path.join(HERE, '..', 'data', 'test-classes.tsv')
crate, mpath = sys.argv[1], sys.argv[2]
excl = sys.argv[sys.argv.index('--exclude-file') + 1:] if '--exclude-file' in sys.argv else []
cls, times = {}, {}
for r in csv.DictReader(open(CLASSES), delimiter='\t'):
    if r['crate'] == crate:
        cls[r['name']] = r
        times[r['name']] = float(r['local_exec_s'] or 0)
m = json.load(open(mpath))
mut = {k: v for k, v in m['mutants'].items() if not any(e in v['file'] for e in excl)}
caught = {k: set(n.split('::')[-1] for _, n in v['failing']) for k, v in mut.items() if v['summary'] == 'CaughtMutant'}
summ = collections.Counter(v['summary'] for v in mut.values())
viable = summ['CaughtMutant'] + summ['MissedMutant'] + summ['Timeout']
print(f'== {crate}: {dict(summ)}; score {(summ["CaughtMutant"] + summ["Timeout"]) / max(viable, 1):.1%}')
buckets = collections.Counter()
for s in caught.values():
    n = len(s)
    buckets['1' if n == 1 else '2-4' if n <= 4 else '5-19' if n <= 19 else '20+'] += 1
print('   catchers per caught mutant:', dict(sorted(buckets.items())))
remaining, cover = {k: s for k, s in caught.items() if s}, []
while remaining:
    cnt = collections.Counter(t for s in remaining.values() for t in s)
    best = max(cnt.items(), key=lambda kv: (kv[1], -times.get(kv[0], 0)))[0]
    cover.append(best)
    remaining = {k: s for k, s in remaining.items() if best not in s}
print(f'   greedy cover: {len(cover)} of {len(cls)} tests catch everything the suite catches: {cover}')
rnd = lambda t: t in cls and 'randomized' in cls[t]['flags']
print(f'   caught by >=1 randomized test {sum(1 for s in caught.values() if any(map(rnd, s)))}; '
      f'only by fixed-input tests {sum(1 for s in caught.values() if s and not any(map(rnd, s)))}')
byclass = collections.Counter(k for s in caught.values() for k in {cls[t]['class'] if t in cls else '?' for t in s})
print('   mutants caught by >=1 test of class:', dict(byclass))
uniq = collections.Counter(next(iter(s)) for s in caught.values() if len(s) == 1)
catches = collections.Counter(t for s in caught.values() for t in s)
print('   unique catchers:', sorted(((n, t) for t, n in uniq.items()), reverse=True))
print('   zero-catch tests:', sorted(t for t in cls if catches[t] == 0))
for sec, t in sorted(((times.get(t, 0), t) for t in cls), reverse=True)[:12]:
    print(f'      {sec:6.2f}s {t} catches={catches[t]} unique={uniq[t]} class={cls[t]["class"]}')
