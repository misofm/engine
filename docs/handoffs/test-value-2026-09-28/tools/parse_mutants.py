#!/usr/bin/env python3
"""Build a per-test catch matrix from a cargo-mutants output dir run with --no-fail-fast.

usage: parse_mutants.py <out-dir> [<out-dir2> ...] <crate>
  Extra dirs are --iterate passes over the same mutants; their catches are merged.
Writes <out-dir>/matrix.json: {mutants: {key: {desc, summary, failing, file, line, genre}},
catches: {test: n}, unique: {test: n}, zero: [[file, line, name], ...]} and prints a summary.
Test names are matched to ../data/test-classes.tsv by crate and function name.
"""
import collections, csv, json, os, re, sys

ANSI = re.compile(r'\x1b\[[0-9;]*m')
HERE = os.path.dirname(os.path.abspath(__file__))
CLASSES = os.path.join(HERE, '..', 'data', 'test-classes.tsv')


def load(outdir):
    base = os.path.join(outdir, 'mutants.out')
    oc = json.load(open(os.path.join(base, 'outcomes.json')))
    res = {}
    for o in oc['outcomes']:
        sc = o['scenario']
        if sc == 'Baseline' or (isinstance(sc, dict) and 'Baseline' in sc):
            continue
        m = sc['Mutant']
        start = m['span']['start']
        key = '%s:%s:%s %s' % (m['file'], start['line'], start['column'], m.get('name') or m.get('genre', ''))
        fn = m.get('function')
        fname = fn.get('function_name', '') if isinstance(fn, dict) else ''
        desc = '%s:%d %s -> %s (%s)' % (m['file'], start['line'], fname, m.get('replacement', ''), m.get('genre', ''))
        failing = []
        logp = os.path.join(base, o['log_path'])
        if os.path.exists(logp):
            cur = None
            for line in open(logp, errors='replace'):
                line = ANSI.sub('', line)
                mm = re.search(r'Running (\S+)(?: \((\S+)\))?', line)
                if mm:
                    cur = mm.group(1)
                mm = re.match(r'test (\S+) \.\.\. FAILED', line.strip())
                if mm:
                    failing.append((cur, mm.group(1)))
        res[key] = dict(desc=desc, summary=o['summary'], failing=sorted(set(failing)), file=m['file'],
                        line=start['line'], genre=m.get('genre', ''))
    return res


def main():
    dirs, crate = sys.argv[1:-1], sys.argv[-1]
    merged = {}
    for d in dirs:
        for k, v in load(d).items():
            prev = merged.get(k)
            if prev is None or v['summary'] == 'CaughtMutant':
                if prev is not None:
                    v['failing'] = sorted(set(map(tuple, prev['failing'])) | set(map(tuple, v['failing'])))
                merged[k] = v
    counts = collections.Counter(v['summary'] for v in merged.values())
    print(crate, dict(counts))
    tests = [r for r in csv.DictReader(open(CLASSES), delimiter='\t') if r['crate'] == crate]
    catches, unique = collections.Counter(), collections.Counter()
    for v in merged.values():
        if v['summary'] != 'CaughtMutant':
            continue
        names = set(n.split('::')[-1] for _, n in v['failing'])
        for n in names:
            catches[n] += 1
        if len(names) == 1:
            unique[next(iter(names))] += 1
    zero = [(t['file'], t['line'], t['name']) for t in tests if catches[t['name']] == 0]
    print('tests', len(tests), 'with >=1 catch', sum(1 for t in tests if catches[t['name']]),
          'with a unique catch', sum(1 for t in tests if unique[t['name']]), 'zero catches', len(zero))
    json.dump(dict(mutants=merged, catches=catches, unique=unique, zero=zero),
              open(os.path.join(dirs[0], 'matrix.json'), 'w'), indent=1)


main()
