# Wasm gates: fail when V8 spills the EQ's stationary cascade loops in the shipped AudioWorklet artifact

## Product outcome

#977 attempt 1 made the standing one-band browser EQ +21 % slower (19.9 to 24.1 us per 64 tracks) because V8 kept one integrator of the depth-1 tail loop in a stack slot instead of a register. Every committed gate stayed green: the roster, the callgraph rules, the digests and the in-crate timings, which ran in a different wasm guest where the loop compiled cleanly. The #977 attempt-2 verification then showed that a one-token edit to the tail's surroundings brings the spill back, and that a rustc or V8 upgrade could too. Only a scratch V8-listing scan (`v8loops.py` in the #977 verifier's harness) sees it.

## Smallest closable slice

A committed, deterministic check in the wasm gate family (beside `scripts/check-web-audioworklet-callgraph.py`) that loads the shipped `host_web.wasm` in the pinned Node version, has V8 optimise the EQ's stationary cascade functions (depth-1 tail and depth-2 pair, dual and mono), and fails if any of those loops carries a stack slot across its back edge. It prints the offending listing. It must not time anything.

## Objective gates

1. Green on the batch head. Red on `codex/977-eq-elision-and-passes-attempt1` (the spilling build), and red under the one-token tail edit the #977 verifier recorded.
2. Deterministic: the same verdict on ten consecutive runs, with the Node version and V8 flags pinned in the script and checked at start (a different Node version is a clear error, not a pass).
3. It locates the loops by stable symbol names, not by listing offsets, and fails closed if it cannot find one.
4. Wired into `scripts/run-wasm-gates.sh`, with its runtime reported (target under 60 s).
5. `docs` note: what the check proves (no carried stack slot in those loops), and what it does not (timing).
