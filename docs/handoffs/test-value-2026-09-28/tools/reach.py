"""Reachability of scripts/ files from .github/workflows, through non-comment mentions.

Mode strict: carriers are workflows, package.json npm scripts, and any tracked file that is
itself reached (followed transitively).
Mode broad: additionally every .rs/.ts/.mjs/.js/.cjs file outside scripts/ counts as a reached
carrier (cargo test / the SDK / host-web code may shell out to a script).
"""
import os, re, sys, subprocess, json, itertools

ROOT = sys.argv[1]
mode = sys.argv[2] if len(sys.argv) > 2 else 'strict'
os.chdir(ROOT)
tracked = [l for l in open(sys.argv[3]).read().splitlines()]
tracked_set = set(tracked)
scripts = [p for p in tracked if p.startswith('scripts/')]
TEXT_EXT = ('.sh', '.py', '.jq', '.mjs', '.cjs', '.js', '.ts', '.yml', '.yaml', '.toml', '.json',
            '.rs', '.md', '.txt', '.html')


def strip_comments(path, text):
    out = []
    hash_style = path.endswith(('.sh', '.py', '.yml', '.yaml', '.toml', '.jq', '.txt'))
    slash_style = path.endswith(('.rs', '.mjs', '.cjs', '.js', '.ts'))
    for line in text.splitlines():
        s = line.strip()
        if hash_style and s.startswith('#') and not s.startswith('#!'):
            continue
        if slash_style and (s.startswith('//') or s.startswith('*') or s.startswith('/*')):
            continue
        out.append(line)
    return '\n'.join(out)


def expand_braces(text):
    # expand simple a{b,c}d patterns (one level, repeated) so brace lists name files
    pat = re.compile(r'([A-Za-z0-9_./-]*)\{([A-Za-z0-9_.,-]+)\}([A-Za-z0-9_./-]*)')
    extra = []
    for m in pat.finditer(text):
        for alt in m.group(2).split(','):
            extra.append(m.group(1) + alt + m.group(3))
    return text + '\n' + '\n'.join(extra)


cache = {}


def body(path):
    if path not in cache:
        try:
            t = open(path, errors='replace').read()
        except (IsADirectoryError, FileNotFoundError):
            t = ''
        cache[path] = expand_braces(strip_comments(path, t))
    return cache[path]


def mentions(src_text, target):
    base = os.path.basename(target)
    if base in src_text:
        return True
    # jq modules are included by stem
    if target.endswith('.jq'):
        stem = base[:-3]
        if re.search(r'(include|import)\s+"' + re.escape(stem) + '"', src_text):
            return True
    return False


seeds = [p for p in tracked if p.startswith('.github/workflows/')]
reached = set(seeds)
# npm scripts: package.json files are reached when a workflow runs npm in that dir; treat both
# package.json files as carriers (their scripts entries are what `npm run` executes).
for pj in ('sdk/package.json', 'hosts/host-web/qualification/package.json'):
    if pj in tracked_set:
        reached.add(pj)
if mode == 'broad':
    for p in tracked:
        if not p.startswith('scripts/') and not p.startswith('artifacts/') and \
                not p.startswith('.github/ISSUE_SPECS/') and p.endswith(('.rs', '.ts', '.mjs', '.js', '.cjs')):
            reached.add(p)

# candidate carriers outside scripts/ that can be reached by mention (host-web / sdk node code)
others = [p for p in tracked if not p.startswith('scripts/') and p.endswith(('.mjs', '.cjs', '.js', '.ts', '.sh', '.py'))
          and not p.startswith('artifacts/') and not p.startswith('.github/ISSUE_SPECS/')]
frontier = list(reached)
via = {}
while frontier:
    src = frontier.pop()
    t = body(src)
    for tgt in itertools.chain(scripts, others):
        if tgt in reached:
            continue
        if mentions(t, tgt):
            reached.add(tgt)
            via[tgt] = src
            frontier.append(tgt)

unreached = [p for p in scripts if p not in reached]
total = 0
for p in sorted(unreached):
    n = sum(1 for _ in open(p, errors='replace'))
    total += n
    print(f'{n:6d} {p}')
print(f'UNREACHED {len(unreached)} files, {total} lines (mode={mode}); scripts total {len(scripts)}')
json.dump({'unreached': sorted(unreached), 'via': via}, open(sys.argv[4], 'w'), indent=1)
