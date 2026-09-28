# Exact rest tails for the compressor and the limiter

Draft, slice S6 of the silence architecture issue (A0). Class A. Evidence:
`docs/handoffs/silence-2026-09-27/DESIGN.md` sections 3 and 4.9.

## Product outcome

A compressor emits exact `+0.0` from the first silent input sample (`x * g` with `g` finite and
positive), and a limiter does once its lookahead line has drained (about 4 blocks at the fixture's
5 ms), but neither reaches its exact fixed point until its gain state has decayed to the D7 flush:
about 1.9 s for the fixture compressor's 40 ms release, 2.7-6.3 s for the limiter after gain
reduction. Until then the whole-bank latch (S4) cannot fire, and the bank runs every slot at full
cost. On the dogfood song the console strip latched 2.07 of 8 banks per block against 3.07 for the
builtins-only strip, which rests in about 0.4 s.

This slice adds the contract's **Tail** tier (A0 D3) to both effects: when the input is `+0.0`, no
ramp is in flight, the bypass flag is the one in force and the output is provably `+0.0` for the whole
block, `silent_rest()` is true and `advance_silent_rest(frames, mono)` runs only the decaying recursion
(on the left channel only when `mono`),
with the kernel's exact per-sample operations on a `+0.0` input, so every state word is bit-identical
to the full kernel's:

* **compressor**: the detector at its floor and the gain computer's zero-reduction target feed the
  release ballistic of `gain_reduction_db` (`crates/compressor/src/kernel.rs`, `ballistic`); the
  output is `+0.0` by `gain_mix_step` on a `+0.0` input for every finite gain and mix;
* **limiter**: once `history` and `main_ring` are all `+0.0` and `required_ring`, `box_ring`,
  `prefix` are all exactly `1.0` and `box_sum == Wb` (`is_at_silent_rest` without `reduction`), the
  output is `main_ring`'s `+0.0` times any gain, and the tail is `d`'s release recursion plus cursors
  and phase. Also reorder `is_at_silent_rest` to test the eight-word `reduction` before the 15.5 KB
  `main_ring` (an audit finding: every tail block pays the ring scan first today).

## Authorized paths

`crates/compressor/src/{lib.rs,kernel.rs}`, `crates/true-peak-limiter/src/lib.rs`, their tests and
`MUTATIONS.md`, this spec.

## Objective gates

1. **Tail = kernel, bit for bit.** For each effect, widths 4 and 8 and the mono path: seeded loud
   passages driving gain reduction from 0 to 30 dB, release and lookahead settings across their
   domains, then silence; the tail arm (tail path engaged) and the forced-kernel arm produce
   bit-identical output and bit-identical state payloads on every block, through the tail, the
   fixed point, and a loud re-entry at a random block.
2. **It engages.** The tail path engages on the first block that satisfies its preconditions (count
   it); the exact fixed point claim still engages afterwards.
3. **Red mutations:** a tail step with one operation reordered; engaging before the lookahead line
   drained; engaging during a ramp; skipping the phase advance.
4. S4's corpus stays bit-identical with S6 enabled; worst-case and realtime gates as S4; rule 3 on the
   tail recursions (they are `L`-generic vector code and must stay vector-dominant).

## Console benchmark rows

`sixty_four_track_console_sparse` (S2). No digest changes.

## Dependencies

S4.

## Rulings

The class-B alternative (snap dB-domain smoothers to rest at, say, 1e-3 dB instead of the 1e-20
flush: rest in about 9 time constants instead of about 48, fewer lines of code, a one-time re-baseline)
is not this issue; the owner chooses between them.

## Standing rules for the implementer

As S4.
