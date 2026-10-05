"""Standalone model of prime adoption (M6). Not engine code.

A plan: source-read offset O; claim lines (per claiming node); nodes in topological order.
Each node: inputs = list of (edge_key, upstream, comp_len); stateful one-pole + latency line.
Upstream is ('claim', claim_key) or ('node', node_key). Output of a node at render r.
Consumer: per source, next_frame; plays q frames per block at source-read r+O.
Adoption at S (move mode): carried node state moved; equal-length lines swapped;
grown/shifted lines on raw content (claims, Input-tap edges) filled from pending ++ prime;
other changed lines head-aligned with zeros (they carry ducked zeros).
"""
import random
from collections import deque

q = 128

class Line:
    def __init__(self, n, init=None):
        self.n = n
        self.buf = deque(init if init is not None else [0.0] * n, maxlen=None)
        assert len(self.buf) == n
    def push_pop(self, x):
        if self.n == 0:
            return x
        self.buf.append(x)
        return self.buf.popleft()
    def pending(self):
        return list(self.buf)  # oldest first

class Node:
    def __init__(self, key, inputs, lat, coef, gain=1.0):
        self.key, self.inputs, self.lat, self.coef, self.gain = key, inputs, lat, coef, gain
        self.state = 0.0
        self.latline = Line(lat)
        self.lines = {e: Line(c) for (e, _, c) in inputs}

class Plan:
    def __init__(self, O, claims, nodes, out):
        self.O = O
        self.claims = claims          # claim_key -> (source, Line)
        self.nodes = nodes            # ordered list
        self.out = out
        self.next_frame = {}          # source -> consumer next frame (only render-owned)

def src_val(source, frame):
    rnd = random.Random(hash((source, frame)) & 0xffffffff)
    return rnd.uniform(-1, 1)

def render_block(plan, r0, outbuf):
    # consumer: each source plays q frames, contiguous
    played = {}
    for s in plan.next_frame:
        f = plan.next_frame[s]
        played[s] = [src_val(s, f + i) for i in range(q)]
        plan.next_frame[s] = f + q
    for i in range(q):
        vals = {}
        for ck, (s, line) in plan.claims.items():
            vals[('claim', ck)] = line.push_pop(played[s][i])
        for n in plan.nodes:
            acc = 0.0
            for (e, up, _) in n.inputs:
                acc += n.lines[e].push_pop(vals[up])
            n.state = n.coef * n.state + acc
            vals[('node', n.key)] = n.latline.push_pop(n.state * n.gain)
        outbuf.append(vals[('node', plan.out)])

def adopt(succ, pred, carried_nodes, raw_edges, P):
    """Move-mode adoption at S with a prime of P frames.
    raw_edges: edge keys whose upstream content is raw source (Input-tap) -> (source, upstream claim line len in succ)
    """
    succ.next_frame = dict(pred.next_frame)
    # prime: read P frames per source from consumer position
    prime = {}
    for s, f in succ.next_frame.items():
        prime[s] = [src_val(s, f + i) for i in range(P)]
        succ.next_frame[s] = f + P
    # claim lines: successor line holds last Lc' of [pending ++ prime]
    for ck, (s, line) in succ.claims.items():
        pend = pred.claims[ck][1].pending() if ck in pred.claims else []
        pool = pend + prime[s]
        n = line.n
        if n > len(pool):
            fill = [0.0] * (n - len(pool)) + pool   # not enough history: zeros (added)
        else:
            fill = pool[len(pool) - n:] if n else []
        succ.claims[ck] = (s, Line(n, fill))
    pnodes = {n.key: n for n in pred.nodes}
    for n in succ.nodes:
        if n.key in carried_nodes and n.key in pnodes:
            p = pnodes[n.key]
            n.state = p.state
            assert n.lat == p.lat
            n.latline = Line(n.lat, p.latline.pending())
        for (e, up, c) in n.inputs:
            if n.key in pnodes and e in pnodes[n.key].lines:
                pl = pnodes[n.key].lines[e]
                if pl.n == c:
                    n.lines[e] = Line(c, pl.pending())
                elif e in raw_edges:
                    s, claim_key = raw_edges[e]
                    # content on this edge is raw source through the upstream (pred) claim line;
                    # pred's upstream node emits claim content; line must emit pred-timeline content.
                    # pred content at render S..: pending(pl) then pred claim pending then prime.
                    pc = pred.claims[claim_key][1].pending() if claim_key in pred.claims else []
                    pool = pl.pending() + pc + prime[s]
                    n.lines[e] = Line(c, pool[:c] if c <= len(pool) else pool + [0.0]*(c-len(pool)))
                else:
                    pend = pl.pending()
                    # head-aligned (zeros after): only used on edges that carry ducked zeros
                    fill = (pend + [0.0] * c)[:c]
                    n.lines[e] = Line(c, fill)

def track(name, claim, lat_insert=None, gain=1.0, ins_coef=0.7, sc=None, sc_len=0, main_lines=None):
    """Return node list for a track: In -> [Ins] -> F (fader) -> M (matrix)."""
    ml = main_lines or {}
    nodes = [Node(name + 'in', [('e_' + name + 'c', ('claim', claim), 0)], 0, 0.0)]
    up = name + 'in'
    if lat_insert is not None:
        nodes.append(Node(name + 'ins', [('e_' + name + 'ins', ('node', up), 0)], lat_insert, ins_coef))
        up = name + 'ins'
        fe = 'e_' + name + 'f_from_ins'
    else:
        fe = 'e_' + name + 'f'
    finputs = [(fe, ('node', up), ml.get('f', 0))]
    if sc is not None:
        finputs.append(('sc_' + name, ('node', sc), sc_len))
    nodes.append(Node(name + 'f', finputs, 0, 0.5, gain))
    nodes.append(Node(name + 'm', [('e_' + name + 'm', ('node', name + 'f'), ml.get('m', 0))], 0, 0.25))
    return nodes

def output(inputs):
    return Node('out', inputs, 0, 0.1)

def run(plans_at, total_blocks):
    """plans_at: list of (S_block, builder) ; first at block 0 (no adoption). builder(pred) -> (succ, carried, raw, P)."""
    out = []
    plan = None
    for b in range(total_blocks):
        for (sb, build) in plans_at:
            if sb == b:
                if plan is None:
                    plan = build(None)[0]
                else:
                    succ, carried, raw, P = build(plan)
                    adopt(succ, plan, carried, raw, P)
                    plan = succ
        render_block(plan, b * q, out)
    return out

def scenario_a():
    # pred: T1, T2 -> out, O=0; T1 ducked (gain 0); T2 has a sidechain from T1's Input tap.
    def pred(_):
        claims = {'c1': ('s1', Line(0)), 'c2': ('s2', Line(0))}
        nodes = track('T1', 'c1', gain=0.0) + track('T2', 'c2', sc='T1in', sc_len=0) + \
            [output([('o1', ('node', 'T1m'), 0), ('o2', ('node', 'T2m'), 0)])]
        p = Plan(0, claims, nodes, 'out'); p.next_frame = {'s1': 0, 's2': 0}
        return p, None, None, 0
    def succ(_):
        P = 512
        claims = {'c1': ('s1', Line(0)), 'c2': ('s2', Line(512))}   # restarted c1: no lead
        t1 = track('T1', 'c1', lat_insert=486, gain=0.0, main_lines={'m': 26})  # 486 + 26 = 512
        t2 = track('T2', 'c2', sc='T1in', sc_len=512)                # sidechain line grows by P
        nodes = t1 + t2 + [output([('o1', ('node', 'T1m'), 0), ('o2', ('node', 'T2m'), 0)])]
        p = Plan(512, claims, nodes, 'out')
        carried = {'T1m', 'T2in', 'T2f', 'T2m', 'out'}               # T1 pre-fader + fader restarted
        raw = {'sc_T2': ('s1', 'c1')}
        return p, carried, raw, P
    ref = run([(0, pred)], 60)
    swp = run([(0, pred), (7, succ)], 60)
    return ref, swp

def scenario_b():
    # M1: first growth by an added latent track, ordinary rebuild, second growth.
    def p0(_):
        claims = {'c1': ('s1', Line(0)), 'c2': ('s2', Line(0))}
        nodes = track('T1', 'c1') + track('T2', 'c2') + \
            [output([('o1', ('node', 'T1m'), 0), ('o2', ('node', 'T2m'), 0)])]
        p = Plan(0, claims, nodes, 'out'); p.next_frame = {'s1': 0, 's2': 0}
        return p, None, None, 0
    def mk(O, L1, L2, extra):
        claims = {'c1': ('s1', Line(L1)), 'c2': ('s2', Line(L2))}
        nodes = track('T1', 'c1') + track('T2', 'c2')
        outs = [('o1', ('node', 'T1m'), 0), ('o2', ('node', 'T2m'), 0)]
        for (name, src, lat, oline) in extra:          # added tracks, muted (armed), claim line 0
            claims['c' + name] = (src, Line(0))
            nodes += track(name, 'c' + name, lat_insert=lat, gain=0.0)
            outs.append(('o' + name, ('node', name + 'm'), oline))
        nodes.append(output(outs))
        return Plan(O, claims, nodes, 'out')
    carried_base = {'T1in', 'T1f', 'T1m', 'T2in', 'T2f', 'T2m', 'out'}
    def g1(_):   # add T3 (latency 486) -> P1 = 512; out floor 512; T3 line into out 512-486
        return mk(512, 512, 512, [('T3', 's1', 486, 26)]), carried_base, {}, 512
    def r2(_):   # ordinary rebuild: add muted T4, no growth; offset inherited; claims equal
        return mk(512, 512, 512, [('T3', 's1', 486, 26), ('T4', 's2', None, 512)]), \
            carried_base | {'T3in', 'T3ins', 'T3f', 'T3m'}, {}, 0
    def g2(_):   # second growth: add T5 with latency 700 -> natural out 700 > 512 -> grows 188 -> P2 = 256
        # carried claims 512 -> 768; offset 768; out floor 512+256 = 768
        return mk(768, 768, 768, [('T3', 's1', 486, 26 + 256), ('T4', 's2', None, 768),
                                  ('T5', 's2', 700, 68)]), \
            carried_base, {}, 256
    ref = run([(0, p0)], 80)
    swp = run([(0, p0), (6, g1), (20, r2), (35, g2)], 80)
    return ref, swp

def first_diff(a, b):
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            return i
    return None

if __name__ == '__main__':
    ref, swp = scenario_a()
    print('A (#1397 g1 shape + Input-tap sidechain from restarted strip): first diff', first_diff(ref, swp), 'of', len(ref))
    ref, swp = scenario_b()
    print('B (M1: growth, ordinary rebuild, second growth): first diff', first_diff(ref, swp), 'of', len(ref))
