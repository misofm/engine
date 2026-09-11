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

## Attempt 1 exact-path freeze — 2026-09-11

- `crates/builtins/src/corpus.rs`: exactly two appended cases, independent scalar
  expected words using the existing `dsp-reference` recurrence, and test-only
  delegating Lane arithmetic observation to distinguish actual dispatch.
- `tools/wasm-gate-corpus/src/lib.rs`: retain the original eight-case builtin block
  and append the new builtin cases after the previous global last case. This
  preserves existing limiter/compressor indexes as well as all builtin indexes.
- `tools/wasm-gates/tests/g5_native_corpus.rs`: ownership and preserved-index checks
  for that existing corpus dispatch.
- This issue spec: scoped decisions and evidence.

No production DSP algorithm or public test-observation surface changes. The test
Lane wrapper delegates every operation to the selected real backend and counts
only `fma` operations: identity has no recurrences, mixed has two sections per
frame, and the full-chain control has four. Unlike a prepared-plan observation,
this detects actually executing the wrong kernel even when the output is equal.

## Attempt 1 focused checkpoint

Astra LOW reports PASS for `cargo test --locked -p builtins --lib --test determinism`:
11 unit and two integration tests cover independently derived scalar expectations,
actual dispatch at scalar/four/eight lanes, forced-full controls and all ten digests.
The original eight digest literals remain unchanged. Receipts with command, source
and output are preserved at `/tmp/issue213-astra1`. Global Wasm corpus compilation,
scalar/simd128 execution and independent Astra XHIGH review remain pending; this
checkpoint does not claim those gates have passed.

## Attempt 1 target qualification

At pushed source `ce5feaed`, Astra LOW reports 84 builtin tests passed (one
benchmark ignored), seven native G5 tests passed, and both scalar and simd128
Wasmtime 47.0.3 guests passed 141 cases / 355 comparisons with zero mismatches.
Affected-package all-target Clippy with warnings denied and builtin policy passed.
Twelve command receipts, toolchain identities and guest hashes are preserved at
`/tmp/issue213-astra1`. Existing eight digest byte arrays and case names are
unchanged, with later global indexes preserved. No benchmark, compiler capture,
fixture regeneration or shipped artifact pin change occurred. Independent Astra
XHIGH review and required reviewed-head PR/main CI remain pending delivery gates.

## Independent review and artifact qualification

Astra XHIGH records attempt 1 PASS at `1eda852f`: an external float32 oracle
independently reproduces both new digests and detects omitted identity addition,
bypassed real filters, and a reset at the split block. Independent focused tests
pass. One stale corpus-layout comment was corrected without executable changes.
Root's ordinary shipped artifact build passed; all six files are byte-identical
to the delivered artifact, including Wasm SHA-256
`a24d0cae46f097336c02268b946f92fbf4419f05b6e05d20a1476f19c76d7e60`.
The first invocation rejected a missing output directory before building; its
receipt is preserved, and the corrected ordinary build passed without repinning.
Review: `/tmp/issue213-astra-xhigh-review`; artifact and comparison receipts:
`/tmp/issue213-artifact-zjos1zzu`. Required PR/main CI remains pending.

## Historical issue body

Verifier finding F2 from strip Job 1 (#212): every builtins case in the frozen gate corpus (`crates/miso-engine-builtins/src/corpus.rs::lane_parameters`) carries non-zero cutoffs on every lane, so the wasm G5 gates never execute `identity_chain_block`/`mixed_chain_block`. Present class-A evidence for wasm is the sealed benchmark digest identity (all three legs, both arms) — sound today, but the elided path has no standing cross-target regression gate.

Fix: add an identity-section and a mixed-section corpus case and extend the oracle-generated pins so G5 exercises the elided path permanently. Re-pin ceremony per convention (the corpus digests are sealed — derivation-based re-pin, byte-accounted). Standard protocol: Opus implements, Fable verifies pre-PR.