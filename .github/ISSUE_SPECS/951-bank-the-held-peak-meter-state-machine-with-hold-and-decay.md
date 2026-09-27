# Bank the held-peak meter state machine with hold and decay

## Product outcome

#950 banks the full meter pass for meters with hold 0 and decay off, the only configuration any host binds today. A meter with a peak hold time or a dB-per-second decay still runs its held-peak state machine one lane at a time. Under the owner's rule ("We shouldn't leave scalar arithmetic where vector arithmetic is possible"), that is a scheduled defect, not a permanent path: the held-peak update is a per-lane, sample-serial state machine (hold countdown, then decay) that can run across a bank's lanes exactly like #950's seeded energy sum, with each meter's own state as the seed.

## Smallest closable slice

To be briefed after #950 lands, reusing its seed-and-accept design: a lane-parallel, sample-serial held-peak pass for hold > 0 and decay on, bit-identical to the scalar state machine on every published word (`held_peak`, and every word it feeds), with the scalar path as the oracle and a wasm differential.

## Dependencies

#949, #950.

Filed from #950's coordinator ruling 2.
