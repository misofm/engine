# Ruling request: one-sided and near-mono stems

Draft ruling request for the owner, not an implementation issue. Base `6ca203f8`.

## What the dogfood stems contain (81 stereo WAV files, `DUAL-MONO.md` §4.1)

* **18 bit-identical dual mono.** Foldable exactly (existing `fold-mono`); no ruling needed.
* **18 one-sided:** one channel is digital silence for the whole file, the other carries mono
  content (backing-vocal doubles printed hard left or hard right). 29% of all frames, 31% of the
  non-silent blocks. Today every such track is processed on both channels; the silent side runs
  the whole strip on zeros.
* **3 near mono** (side/mid between -73 and -103 dB): almost, but not bit, identical.

## Why one-sided stems are not an optimisation decision

Folding `(x, 0)` into a mono source panned hard to one side does not render the same bits in
general:

* an average-linked detector sees `0.5*|x| + 0.5*0` on the stereo file but `0.5*|x| + 0.5*|x|` on
  the fold, so the gain differs;
* the silent side's output is whatever the strip makes of zeros, including `-0.0` versus `+0.0`;
* later edits behave differently (an EQ on the "mono" track now shapes the audible side only if the
  pan law routes it there).

So it changes what the mix is, not how fast it renders.

## Questions

1. Should import offer "mono content on one side" as a mono source with a hard pan (halving its
   strip cost and its storage), accepting that it is a different session from the file as
   delivered?
2. Should near-mono stems be offered a "treat as mono" action (audibly identical, not bit-exact)?
3. Independently of both: is it worth an engine issue to skip strip work for a channel whose input
   and state are exactly zero (a silence-mask extension, related to #940-#942)? That would help
   the one-sided stems without changing the session, and it would also cover the 87% of blocks in
   these stems that are silent on both channels.
