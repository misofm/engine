# Multiband compressor: a ramp's cut moves a lane's bits in a bank

Found by #1051's randomized differential (`crates/multiband-compressor/tests/randomized.rs`, ignored reproducer "#1051 defect 1", seed 1).

## Problem

When a parameter ramp is cut (a new automation segment replaces an in-flight ramp), one lane of a banked multiband compressor renders different bits from the same track rendered alone. That breaks the class-A rule that banking regroups lanes and never changes per-lane arithmetic, and partition invariance: moving a track into or out of a bank must not move a rendered bit.

## Smallest closable slice

Find where the cut applies per bank rather than per lane (or reads a neighbour's ramp state), fix it so each lane's ramp state is independent, and un-ignore the reproducer.

## Gates

1. The reproducer passes at seed 1 and across the randomized differential's seeds, at Simd4 and Simd8 and against the scalar oracle.
2. A planted cross-lane ramp read turns it red.
3. Console digests unchanged unless the defect affects a console workload; if so, explain every moved digest.
4. Render stays allocation-, lock- and syscall-free.
