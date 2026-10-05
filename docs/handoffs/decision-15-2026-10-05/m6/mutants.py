import model
from model import *

orig_adopt = model.adopt

def adopt_zeros_first(succ, pred, carried, raw, P):
    # #1283 D4 order on claim lines: P zeros, then pending
    orig_adopt(succ, pred, carried, raw, P)
    for ck, (s, line) in succ.claims.items():
        if ck in pred.claims and line.n > pred.claims[ck][1].n:
            pend = pred.claims[ck][1].pending()
            fill = ([0.0] * (line.n - len(pend)) + pend)
            succ.claims[ck] = (s, Line(line.n, fill))

def adopt_claims_at_rest(succ, pred, carried, raw, P):
    orig_adopt(succ, pred, carried, raw, P)
    for ck, (s, line) in succ.claims.items():
        succ.claims[ck] = (s, Line(line.n))

def adopt_sc_zero(succ, pred, carried, raw, P):
    orig_adopt(succ, pred, carried, {}, P)

for name, fn in [('zeros-first claim fill', adopt_zeros_first),
                 ('claim lines not carried', adopt_claims_at_rest),
                 ('Input-tap sidechain zero-filled', adopt_sc_zero)]:
    model.adopt = fn
    ra, sa = scenario_a(); rb, sb = scenario_b()
    print(f'{name:35s} A first diff {first_diff(ra, sa)}  B first diff {first_diff(rb, sb)}')
model.adopt = orig_adopt
