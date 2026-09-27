# Recover the sidechained compressor's 3.5% V8 regression from the settled-body rewrite

## Product outcome

The compressor's settled-body rewrite (#981-#985) made every banked compressor shape faster (native Simd8 isolate 40.5 -> 24.5 us, V8 80.4 -> 60.1 us per 64-track block), except one: a compressor with a **connected sidechain** is 3.5% slower under V8 (373 -> 387 us over three runs), introduced by #983 (targets before recurrence); it is flat natively. The #981 verification recorded it; V8 now spills the retained Silent/Sidechain loop, which the new settled body does not cover.

## Smallest closable slice

Diagnose the V8 spill in the retained sidechain loop (register pressure from the shared frame-law helpers or the loop shape #983 left behind), and restore the sidechain shape to at least its pre-#983 V8 cost without moving a bit; optionally extend the settled body to sidechained compressors if that is the cleaner fix.

## Objective gates

- Sidechained compressor under V8: no slower than base `197db1c9` (A/B against a separately built base, under the timing lock), and native not slower.
- Bit-identical output and state against the base kernel (the #981 randomized differential).
- Every console digest unchanged.

Weekly-optimisation issue per AGENTS.md; not a release blocker.
