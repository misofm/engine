# Pin the limiter's linked-pair path with a non-repeating hot console session

## Product outcome

#990 computes a linked stereo pair's gain path once in the true-peak limiter (merged on the optimisation batch; exact in a 36,600-scenario differential). Its kernel tests catch every mutation, but no session-level test can catch its M1: the console source is one 128-frame block repeated, so each limiter's gain settles to a constant, and sessions expose no state snapshots. Found by the #990 verification (medium, non-blocking).

## Smallest closable slice

Add a console session test driven by non-repeating hot noise (a deterministic seeded generator, levels that keep every limiter reducing) with mid-run parameter retargets that link and unlink pairs, rendering 64+ blocks, with its PCM and gain-reduction digest pinned on the pre-#990 kernel (`bbcf8ce1`) and asserted equal after.

## Objective gates

- The test is green on the batch and red under #990's M1 and under a mutation that keeps the shared path after the pair unlinks.
- Every standing console digest unchanged.

Also record (low): once unlinked, a pair relinks only after reset, restore or `desymmetrize`; say in the test whether that is intended.
