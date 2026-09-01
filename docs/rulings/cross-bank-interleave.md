# Fusing independent same-kernel banks into one interleaved loop — null

**Candidate.** Issue #163 phase 3. A TPT SVF is a first-order recurrence, so a bank's block loop is
a serial dependency chain and the vector units idle for most of every frame. The phase-3 plan's
mechanism was to step **K independent banks** together in one loop: the graph executor schedules
several `RuntimeUnit::Bank`s of the same kernel at the same dependency level, they share no
dataflow edge, and fusing K of their frame loops would put K more recurrences in flight.

**Claim under test.** That fusing *banks* is what the mechanism needs — that the independent chains
have to come from separate banks, and that the win is worth reaching them.

## What was built instead

One bank already contains independent recurrences, and phase 3 ships those:

* its **two channels** carry independent state by the definition of dual-mono, and
* its **four cascade sections** are independent of each other — section `k`'s integrators depend on
  section `k`'s previous frame and never on section `k - 1`. Section `k - 1`'s *output* feeds
  section `k` inside the same frame, so cascade depth lengthens the forward chain in the frame body
  but not the loop-carried one.

`svf_cascade_interleaved` takes both axes: `S` streams by `D` sections, `S * D` live recurrences.
The parametric EQ runs it at `S = 2` (its channels) and `D = Lane::SVF_CASCADE_DEPTH`. Nothing in
the executor, the schedule, the bank windows of #169/#170, or the effect contract is touched.

## Measurement

`crates/lane/tests/b2_interleave.rs`, bench host (Zen 5, `x86-64-v3`), one warmup and
three measured rounds, minimum reported, round spread under 0.3%. The workload is one EQ bank-block:
a four-section cascade over two channels of 128 frames, with a **distinct coefficient set in each of
the eight slots** — sharing one set would let an arm win by holding in registers what the real EQ
keeps apart. The baseline is the pre-phase-3 shape, eight serial `svf_block` passes. Every arm does
the same arithmetic in the same per-chain order; only the loop nesting differs.

`S = 2` is what one bank offers. `S = 4` and `S = 8` are the **cross-bank arms**: they are what
fusing two or four independent same-kernel banks would buy.

| backend  | S=2 D=1 | S=2 D=2   | S=2 D=4   | S=4 D=1   | S=4 D=2 | S=8 D=1   | S=8 D=2 |
|----------|---------|-----------|-----------|-----------|---------|-----------|---------|
| `Scalar` | 1.622x  | 1.774x    | **2.092x**| 2.536x    | 3.274x  | 4.027x    | *4.167x*|
| `Simd4`  | 1.800x  | **2.653x**| 2.085x    | *2.932x*  | 2.676x  | 3.180x    | 2.640x  |
| `Simd8`  | 1.721x  | **2.453x**| 1.889x    | *2.694x*  | 2.400x  | 2.510x    | 2.444x  |

Bold is the best cell reachable inside one bank, which is what shipped. Italic is the best
cross-bank cell.

## Ruling: null at the production native backend

At `Simd8` — the production native width, and the backend every recorded console number in this
program is taken at — the best cross-bank cell is **2.694x against the 2.453x that shipped: a 1.10x
margin.** At `Simd4` it is 2.932x against 2.653x, 1.10x again. Both turn over past four to eight
live recurrences, because two streams times four sections is already eight live integrator pairs
plus coefficient words, and a sixteen-register file spills after that. The cross-bank axis is not
adding independence the intra-bank axes did not already supply; it is competing with them for the
same registers.

Against that 1.10x, the cost is structural, not incremental. A `BankChain` reaches its kernel
through two layers of `dyn`: `Box<dyn BankStage>` and then
`Box<dyn PreparedNativeEffectBank>`. Interleaving two banks' frame loops means running one loop body
over two concrete kernels, which no `dyn` call can express. It needs either a `process_bank_group`
method on the effect contract taking `&mut [&mut dyn PreparedNativeEffectBank]` plus an `Any`
downcast on the render path, or a banking change that puts K cohorts behind one prepared object —
which moves bank width, AoSoA layout, per-track state-payload indices and the #170 window colouring.
Both are class-A-risk changes to a versioned contract, for a tenth.

**Scalar is the one place the margin is large** (4.167x cross-bank against the 2.092x that shipped,
2.0x more), because a scalar integrator is one `f32` slot and eight live recurrences do not exhaust
the file. Scalar is the unbanked per-node path and the oracle, not the console's banked path, so the
headroom is real but not where the console's time is.

## What would reopen this

A backend with a materially larger vector register file (AVX-512's 32 `zmm`, SVE), where the
turnover point moves past eight live recurrences and the cross-bank cells stop competing with the
cascade depth for registers. On that host the table above should be re-measured before any of this
ruling is relied on: every number here is `x86-64-v3`, sixteen registers.
