# Builtins corpus: standing identity and mixed dispatch coverage

## Approved current scope — 2026-09-11

All eight current builtin corpus cases leave their filters enabled. Append exactly
two cases exercising production identity-section and mixed-section dispatch. Reuse
the existing corpus, native width checks and Wasm corpus gate; preserve every old
case/index/input/output/pin. Derive only new scalar digests from disclosed deterministic
inputs and independent expected-output reasoning. Do not copy an observed SIMD digest
into a supposed scalar oracle. Prove actual identity_chain_block/mixed_chain_block
selection through the smallest existing seam or bounded test-only observation, plus
a discriminating control for each new path. No production algorithm change.

Own crates/builtins/src/corpus.rs, directly relevant builtin determinism/dispatch tests,
and existing tools/wasm-gates corpus dispatch/fixtures only if needed for appended
cases; scoped evidence/spec. Freeze the exact list before edits; report any new
subsystem needed. Gates: exact old eight pins unchanged; new independent scalar
expectation/control; scalar/four/eight-lane native comparison; current scalar and
simd128 Wasm corpus execution through wasmtime; no regression of nonfinite/tail/
allocation semantics where existing gates apply. No new corpus framework, benchmark,
listening campaign, fixture regeneration or compiler captures. This is standing
cross-target regression coverage, not a DSP optimization or sound-quality claim.

Astra LOW implements; Astra XHIGH reviews. Five attempts maximum. Root checkpoints
coherent compiling/focused-green exact paths and pushes before another tranche.
At most two active issues (#360/#213 now #228 closed); #522 queued. Isolated tree,
no nightly-workflow or host-web timing edits. Required reviewed-head PR/main CI,
upstream evidence and verified closure precede clean delivered-worktree removal.
Root owns shipped artifact qualification/pinning; no unilateral pin changes.
Preserve actual argv/env/source/exit/output evidence externally. Ordinary compile
feedback remains within unfinished pass; substantive failure gets bounded review.

## Historical issue body

Verifier finding F2 from strip Job 1 (#212): every builtins case in the frozen gate corpus (`crates/miso-engine-builtins/src/corpus.rs::lane_parameters`) carries non-zero cutoffs on every lane, so the wasm G5 gates never execute `identity_chain_block`/`mixed_chain_block`. Present class-A evidence for wasm is the sealed benchmark digest identity (all three legs, both arms) — sound today, but the elided path has no standing cross-target regression gate.

Fix: add an identity-section and a mixed-section corpus case and extend the oracle-generated pins so G5 exercises the elided path permanently. Re-pin ceremony per convention (the corpus digests are sealed — derivation-based re-pin, byte-accounted). Standard protocol: Opus implements, Fable verifies pre-PR.