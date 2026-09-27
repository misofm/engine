# Skip provably silent work: the silent-skippable stage contract and the whole-bank latch

**Architecture issue (draft, not filed).** `AGENTS.md` requires one for a cross-cutting change: this
adds render-path semantics to the bank stage seam (`rack::BankStage`), the builtin bank seam
(`graph::GraphPreparedBuiltinBankProcessor`), the effect bank contract
(`effect_contract::PreparedNativeEffectBank`) and the source driver seam
(`graph::GraphPreparedSourceSetDriver`). Evidence and measurements:
`docs/handoffs/silence-2026-09-27/DESIGN.md`. Implementation is the slices S1-S12 listed below; this
issue owns only the decisions.

## Product outcome

An all-silent 64-track console stops paying for silence: a throwaway prototype of D5 measured the
idle row at 29.4 → 4.97 µs per block (native, eight lanes, bound feed), and attributed the rest to the
bench's bound copies (removed by #965), input scans (removed by S1) and bookkeeping (S5). A real
session renders its silent banks for nearly nothing, with every rendered bit,
every state word and every observable value unchanged (class A). Today no render path skips
silence except three effects' own admission checks: the builtin input filters, the fader, the
matrix, the transposes and the master fold run over `+0.0` at full cost (DESIGN.md section 2).

## Decisions (proposed; each is ruled by the coordinator/owner before S1 starts)

**D1. What silent means.** A block is silent when every word of both planes of every active lane has
bit pattern 0 (`+0.0`). `-0.0`, subnormals and NaN are not silent. (#942's
`block_is_positive_zero`, `crates/effect-runtime/src/bank.rs:137`.)

**D2. Silence facts.** A consumer knows its input is silent from, in order of preference:
(1) an unplayed claim (`played_planes == None`); (2) a writer-computed silence bit on the played
transfer block (S1); (3) a latched producer (S10's silence table); (4) the #942 scan of the planar
input, 2-3 ns per live plane, about 5 ns per silent 128-word plane. No fact is ever inferred from a
magnitude test.

**D3. The stage contract.** A stage is *silent-skippable* at block N when, after its control for N
has been drained, it promises for an all-`+0.0` input block of `frames` frames that the kernel
would (a) write the same output it wrote on its last such block, (b) leave every state word
bit-identical except for words `advance_silent(frames, mono)` updates exactly as the kernel would
(`mono`: the chain renders the block collapsed, so only left-channel state moves, as in each
stage's `process_mono`), and (c)
read no absolute time and no state outside (b); and that no automation span, prepared target,
bypass change or live record was admitted for block N. Two tiers qualify: **Rest** (a fixed point;
`advance_silent` is bookkeeping such as cursors, phases, observation windows) and **Tail** (output
provably `+0.0` while a recursion decays; `advance_silent` runs exactly that recursion). The seam:

```rust
// rack::BankStage (crates/rack/src/lib.rs:559), mirrored on GraphPreparedBuiltinBankProcessor
// (crates/graph/src/lib.rs:1048) and, as provided methods, on PreparedNativeEffectBank.
fn silent_skippable(&self) -> bool { false }      // read after begin_block
fn advance_silent(&mut self, frames: u32, mono: bool) {} // a skipped block's exact state update
```

Every default declines. A stage must drain all of its control in `begin_block`, never in `process`,
so that a skipped block cannot drop or delay a record (the acked-batch question).

**D4. Who implements D3.** Builtins implement it with exact state checks (S3). The EQ, compressor
and limiter expose their existing observation-earned claims (S4). Every other banked effect gets
the owner's generic slot rule (S8): the slot compares the effect's complete state payload across two
consecutive silent-output blocks and latches on equality; per-effect claims and tail paths are
optional fast paths (S6). An effect whose payload moves every block (limiter cursors, delay cursor)
or exceeds a byte cap does not use the generic rule. Payload completeness at rest becomes a
contract obligation with a conformance gate (S8).

**D5. The whole-bank latch.** A bank chain qualifies at block N when every active slot is
silent-skippable and every active lane's input is silent (D2). It **seals** after one normal run in
which it qualified and whose resident block came out entirely `+0.0`. While qualified and sealed it
**skips**: no gather, no slot `process`, no scatter, no collapse transition; it calls
`advance_silent(frames, mono)` on every active slot (in the collapse mode it sealed in) and writes
the outputs of its **active** lanes as constants:
* an unfolded lane's output buffer: `+0.0`;
* a folded lane's master contribution per plane: `route_word(coefficients, +0.0, +0.0)`, a signed zero;
  when lane 0 stores, the master becomes the left-to-right `f32` sum of the zeros; a master the chain
  only accumulates into gets `x + (+0.0)` iff any contribution is `+0.0`, and no write if all are
  `-0.0` (#940's skip theorem, Research findings (2)). The skip declines on the premises the fold
  kernels decline on (a storing lane other than lane 0, an unwritable master).
Any block that does not qualify unseals; a change of `frames` or of collapse mode unseals. The resident block keeps the
rest output, so resident observers and successors read correct words on skipped blocks.

**D6. Budget.** Decisions are per block: flag reads and at most one early-exit scan per input plane;
no per-sample branch, allocation, lock or syscall. The dense block stays the budget; a skipped
block's saved time is never lent to realtime work. The first loud block after silence may not
exceed the declined arm's by more than the declined arm's own spread (prototype: 164.0 against
163.1 µs median, maxima 192.5 against 211.6 µs).

**D7. Class B items are separate and optional.** Co-silence cohort grouping (S7), snapping dynamics
tails to rest (alternative to S6), and a gate hard-close mode (S9) each need their own ruling.

**D8. Third-party Wasm (future).** Never banked; executed on sandbox workers. A future latch needs a
worker-side ABI export of the render state (or a digest of the declared bounded state region) or a
plugin-declared rest export audited by the worker. Out of scope until that executor exists.

## Invariants every slice keeps

* **I1 class A:** host planes, every arena buffer a consumer reads, every resident block an observer
  reads, every state payload and every published observation are bit-identical to the declined arm,
  on every block, including during a collapsed run. Evidence counters (`transposes`, collapse
  counters, builtin qualification counters, source-plane gather counters) are exempt and are listed
  by S4.
* **I2 release:** any admitted record, span, target, bypass change, `desymmetrize`, `frames` change or
  plan swap returns the affected chain to the normal path on that block.
* **I3 worst case:** no block is slower than the same block declined beyond the declined arm's own
  spread.
* **I4 realtime:** `scripts/check-realtime-policy.sh`; allocation-free render tests; wasm AudioWorklet
  callgraph rule 3 (scalar constant arithmetic lives in non-generic `#[inline(never)]` functions).
* **I5 FP environment:** the fold constant's rule holds in any rounding-to-nearest environment,
  FTZ/DAZ included (every skipped term is a signed zero and the fix-up sits at the chain's own
  position). S10's skip over live partial sums needs gradual underflow (the canonical MXCSR
  `0x1F80`, FPCR 0, wasm by specification). NaN stays out of bit corpora, as in #940.

## Slices (drafts in this directory)

S1 source silence bit; S2 dogfood sparse rows; S3 builtin rest claims and drains to `begin_block`;
S4 whole-bank latch; S5 latched-cost trims; S6 compressor/limiter exact tails; S7 co-silence cohorts
(class B, optional); S8 generic slot latch from the state payload; S9 mid-chain silence and gate
hard close; S10 master/bus sum skip with a silence table; S11 delay-line, PDC and bypass-shunt rest;
S12 meters read known silence. Minimal path: S1, S3, S2, S4, S5.

## Rulings requested

1. Adopt D1-D6 as the architecture (amend the "Approved audio architecture" section of
   `AGENTS.md` with one paragraph on silence skipping).
2. Minimal path first (S1-S5), generic payload latch (S8) second, replacing #893 and #894?
3. S7 (class B, reorders the master sum): authorised?
4. Tails: S6 (class A, per-effect code) or the class-B snap?
5. Gate hard-close mode (S9b): a product decision.
6. Check in derived activity masks of the dogfood stems (S2)?

## Standing rules

Issue-first; no implementation on this issue. Every slice is class A unless it says otherwise, keeps
its own gates, and does not quote projected savings.
