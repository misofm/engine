# #1379 Amendment 1: design note

Design record for *Define how node tails compose through gain in the graph extent* (#1379), stream G
of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
D15-4(b)). It holds Amendment 1 in full: the root rulings, the node contract and its reasons (H1),
the carriers (H2), each node's values (H3), the composition rule with its inequality chain (H4), the
exact-zero gate (H5), what is not claimed (H6), the interactions (H7), the split with every hot-file
slot and named exception (H8), the probe evidence (H9) and the amended decision text (H10). The
issue body (`.github/ISSUE_SPECS/1379-define-how-node-tails-compose-through-gain-in-the-graph-extent.md`)
keeps the decisions, the rule, the gates, the paths and the order, and points here; where the two
differ, the issue body governs. The proofs themselves are written by the slices in
`docs/derivations/1379-graph-tail-composition.md`.

## Amendment 1 (root rulings, 2026-10-06, after attempt 1's stop)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(b) and #1329 Amendment 3.
Attempt 1 stopped before code. D1-D3 were written on 2026-10-05, before #1329 Amendment 3 gave each
node two tails, `T_decay` for every peak at or above its flush floor `P*` and
`T_rest = max(T_decay, R(P*))`. The implementer found three faults: (1) `tail_every_peak` is
missing from D2's struct, from `GraphNode` and from the graph's report; (2) D1's decade claim (below
`P * 10^((-144 - 20k)/20)` from `N + latency + T + k D`) holds only for `P >= P* * 10^k`, because
below that the `f32` output sits at the per-word flush's absolute stall, so D3's per-node
`-144 - A_i` dB holds only above `P*_i * 10^(ceil(A_i / 20))`; (3) a sound graph bound needs two
branches and a graph-level floor.

Root ruled (2026-10-06, binding): (a) state `TailDecay` with its peak range, each node exporting its
flush floor or the stall it implies; (b) compose `tail_every_peak` with a graph-level `P*_graph`,
reported beside `output_tail` in the compiled report and the canonical text, the sketched formula to
be verified or corrected; (c) D2 extends the landed `EffectTailBound { tail, tail_every_peak, rest }`
with decay and gain, with no parallel struct; (d) a gate below the raised floor on the real kernel at
`P = P*_graph / 2` that checks the exact-zero branch. D2a and D4 stand.

This amendment derives every rule (H1-H4). Two parts of the ruling's sketch change because the
derivation requires it, and root confirmed both (rulings applied, Q1): the floor is a **sum of
stalls carried by downstream gain** (not `max_i P*_i 10^((A_i + G_down_i)/20)`), and the rest branch
is **graph-level** (rests add along a path from the upstream rests; a per-node
`max(T_i + k_i D_i, R_i)` is not the rule). The work exceeds half a working day: H8 splits it into
five slices, A1, A2, B1, B2 and C. A1 (#1464), A2 (#1465), B1 (#1466) and B2 (#1467) are new
issues; #1379 keeps slice C, whose sections replace this spec's. A sixth new issue, #1468, tightens
the fixed input section's peak gain (ruling m6). Attempt 1 is not counted (rulings applied, Q9).

### Root rulings applied (2026-10-06)

- **Q1.** Both corrections to the sketch stand: `P*_graph` is the sum of effective stalls carried by
  downstream tail gain, and the rest branch is graph-level (H4).
- **Q2, M2.** Hosts report `output_tail_every_peak`, the bound valid for every input. `output_tail`
  holds only for `P >= P*_graph`, and `P*_graph` can sit near or above full scale for a live graph
  with a submix (estimated up to about +23 dBFS with every gain at its maximum, including the route
  into the output; about -1 dBFS with a 0 dB output route), so a host reporting it would certify
  nothing for real signals. Decision 15 D15-4(b) is
  amended (H10, applied by slice C). Slice C changes host-core's copy
  (`crates/host-core/src/prepare.rs:1599`) and so the C ABI's `tail_samples`; the graph's decay value
  may be reported beside it only where a consumer needs it, named as the decay value.
- **M1.** `CascadeBound` returns the loud-input deviation term (the output deviation fixed point of
  `Deviation::at_end`, `crates/math/src/tail.rs:1947-1961`), and a recomputation gate checks it (F2(d)).
- **M3, Q5, Q6.** Gain-only nodes state prepared values (with every ramp endpoint the plan can reach)
  for lanes that cannot change after preparation, and the domain maximum only for live lanes (H3);
  slice B2 derives the live `G_t` (no `G_t = G_p` option). No correctness gap is deferred;
  tightenings are successors (m6 below).
- **M4, Q8.** Slice A is split into A1 (the carrier, every composition value `Unstated`) and A2 (the
  fixed-design values). A1 follows #1461, #1462 and #1457; A2 follows A1; B1 follows A2; B2 follows
  B1; C follows B2, with new hot-file slots and stream-A named exceptions (H8).
- **Q3.** Effects state `CompositionBound::Unstated` and graphs with an effect report `Infinite`;
  #1376 is sequenced right after slice A2, in the batch with slice C, and its D4 is restated to H1
  (H7).
- **Q4.** `EffectTailBound` is renamed once to `NodeTailBound` in A1, after #1462: a rename, not a
  parallel struct, so ruling (c) stands.
- **Q7.** `maximum_finite_tail_samples` checks the value the hosts report, `output_tail_every_peak`
  (which is at least `output_tail`), and refuses on it.
- **Q9.** Attempt 1 stopped on a spec conflict before code, as #1377's first run did (#1377
  Amendment 2, ruling 3), so it is not counted: the next verified run of each slice is its attempt 1,
  each on its own counter.
- **MINORs and NITs.** All folded: the range hypothesis and the sanitizer (H4), the sidechain
  quantifier and `down` for a node with no path (H1, H4), condition (C) (H1), the decade gates F1(b)
  and F1(d) (objective, construction-independent), F2(b)'s margin and rows, C6's per-pattern
  precondition, C3's mutant list, C7's test-only effect, #1457's gated budget (F5, L6), ownership and
  hot-file slots (H8), `docs/BUILTINS_AND_METERING_V1.md:55` (K paths), `i64` millibel sums (H4),
  channel combination (H3), strictness, the `T_lambda >= T` clamp, the live share, `Sigma = 0`,
  `m = 0`, F1(a)'s +24 dB threshold, `GraphNode`'s fields, `rest: None`, `estimate.rs:368`.

### Root rulings applied, second round (2026-10-06, after the confirmation review)

The confirmation review (NOT CONFIRMED: three MAJORs, nine MINORs, nine NITs) found the
composition (H4) and the new mathematics sound. Root ruled; no further review round.

- **MJ1.** F2(b)'s second named row is the HPF at 10 Hz into the LPF one `f32` above 10 Hz (the HPF
  at its maximum into the LPF at 10 Hz is refused with `FilterOrder`); the margins include
  `dev_loud`; H9 and F2(d)'s test value are corrected (`dev_loud` about 59 mB at the top pair).
- **MJ2.** F1(b)'s mutant is a certificate rate halfway between the floor rate and 1, which is red
  for every construction; the probe evidence is in H9.
- **MJ3.** One disabled-filter gain rule in H3, F-D3 and F2(c): `|trim| (1 + u)`, or `|trim|` alone
  where the product is exact (a power-of-two trim, 0 dB included), rounded up.
- **m1.** F2(d) also checks the use: `G_p` equals the rounded-up formula from the accessors; the
  recomputation names `input_sup[1]`'s source.
- **m2.** A route's `G_p` is its prepared ungated value whenever its own gain and matrix are fixed;
  the source strip's mute decides only whether `Zero` may be stated (H3).
- **m3.** C's authorized paths add every `GraphBuiltinsCompileRequest {` literal (the new field
  only) and a named file for C12.
- **m4.** An `f64` stall sum that overflows gives the floor `AboveRange` (the rest branch for every
  peak), not a refusal (H4 step 6); C13 gates it; the `i64` check is a unit test of the sum helper.
- **m5.** The `P*_graph` figure is an estimate everywhere it appears, H10 included.
- **m6.** The cascade triangle inequality in the fixed `G_p` is sound (about 14 dB loose at the top
  pair), so removing it is a tightening, not a shortcut: no correctness gap is deferred. A stream G
  successor, *Tighten the fixed input section's peak gain past the cascade triangle inequality*
  (#1468), owns it, with a gate like #1433's (certified within a stated factor of measured, never
  below it).
- **m7.** Slice C is about one working day, an explicit exception to the half-day rule: its
  real-kernel gates are the product evidence (ruling (d) requires C6), and any split along the
  report line would move the C ABI's reported tail on recomputation evidence alone, so halves would
  not be independently verifiable (H8).
- **Root ruling 4 (B's size).** With `G_t` a full derivation, B exceeds half a working day. It is
  split into B1 (the live decay, peak gain and stall, certified and exposed through accessors; the
  bound stays `Unstated`) and B2 (the live tail gain, and the switch to `Stated`), each closable
  and independently verifiable (H8).
- **m8.** #1376's D4 is restated to H1 (H7). **m9.** C6 names a design list per rate (H5).
- **NITs 1-9.** Folded: anchors after #1461 and #1454, K2's wording and test, the carry rule
  (H3), F1(c)'s reason, L2's trim and pair, H6's wording, "one pass" in F-D2, `G_p`'s summation
  factor, A1's payload-code literals.

### Root rulings applied, third round (2026-10-06, after filing)

- **Q3 restated with its evidence: #1375 needs only the carrier (case 1).** *Report a zero tail
  beyond latency for the compressor and the true-peak limiter* (#1375) uses from #1379 only the
  types and the node contract: its D1 states `TailSamples::Finite(0)` and `TailDecay(0)`, its D4
  states a gain "(#1379 D1: a peak gain and an incremental gain after silence)", and its home is
  `tail_and_rest` (#1377). Its gates 1-3 are per-effect real-kernel runs and a recomputation of its
  rest helper; no gate or deliverable reads a composed graph extent. Its one graph-compiler edit,
  `launch_true_peak_limiter_fixture_retains_banks_tails_latency_and_transactional_caps`
  (`crates/graph-compiler/src/lib.rs:12354`, the assertion at `:12383`), checks the effect entry's
  own `metadata.tail`, not `output_tail`. So #1375's dependency on #1379 narrows to #1464 and
  #1465, its D4 is restated to H1 (as #1376's), and #1376 can follow A2. **Batch condition:** a
  plan-level tail that #1375 or #1376 makes finite is sound only once C composes it through gain
  (before C the extent ignores the compressor's makeup, as it ignores a fader today), so #1375 and
  #1376 land on `main` in the batch with C, never before it.
- **Size.** #1379's body must fit GitHub's 65,536-character limit. The full amendment (H1-H10 with
  the derivation sketches, the inequality chain, the evidence and the hot-file and exception lists)
  is kept verbatim in the design note
  `docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md`; this spec keeps the decisions,
  the rule, the gates, the paths and the order, and points to the note.
- **#1468's factor from evidence.** #1468's first step measures `g_meas` and the achievable
  certified bound on the real kernel; its gate factor is then set from that measurement with a
  stated margin, at most 1.5, as #1433 set 1.15.

### H1. The node contract in additive form (ruling (a))

Notation: `eps = 10^(-144/20)`; a node with latency `L`, input `x`, output `y`; `u = 2^-24`.
Conditions (C): every control history the node admits before `N`, and no control event at or after
`N`. A live filter retarget in flight at `N` completes by `N + 64` (#1407); a trim, fader, matrix,
pan or route ramp in flight at `N` may run longer (`smoothing_samples: u32`,
`crates/session/src/visit.rs:106`), and every such ramp stays inside its endpoints (#1408 for trim,
fader and matrix; the route ramp's endpoint law, `crates/graph/src/runtime.rs:854-927`). A node with a
sidechain port states every value below for every sidechain input: a value that depends on the
sidechain's magnitude is not a valid statement.

Beside #1329's three values (`T` = `T_decay`, `tail_every_peak`, `RestSamples`), each node states
five more (four before #1484 split the stall). Each is a certified upper bound at the node's rate, over its parameter domain (#1377 D5)
or its prepared design (#1329 D4), computed on the control thread. A stereo node states each value as
the maximum over its two channels (as `InputSectionBound::max` does today,
`crates/builtins/src/tail.rs:68-90`); no node before the 2x2 matrix mixes channels.

| symbol | name | unit | meaning |
|---|---|---|---|
| `D` | decay | samples per further 20 dB | (N2) below |
| `G_p` | peak gain | millibels, rounded up, or `Zero` | (N1) below |
| `G_t` | tail gain | millibels, rounded up, or `Zero`; `G_t <= G_p` | gain to an input that arrives at or after `N`, (N2) |
| `sigma_p` | peak stall | millibels re 1.0, rounded up, or `Zero` | absolute flush part of the output at every frame, (N1) |
| `sigma_t` | tail stall | millibels re 1.0, rounded up, or `Zero`; `sigma_t <= sigma_p` | absolute flush part of the output from the node's tail on, (N2) |

`g_p = 10^(G_p/2000)` and `g_t = 10^(G_t/2000)` are the linear gains; a `Zero` gain is `0` (the node's
output is exactly `+-0.0` for every input).

- **(N1) Peak.** For every input with `|x[n]| <= X` for all `n`, under any admitted history:
  `|y[n]| <= g_p X + sigma_p` for every `n`.
- **(N2) Tail at every decade.** Under (C), if `|x[n]| <= X` for all `n` and `|x[n]| <= epsilon`
  for every `n >= M` (some `M >= N`), then for every integer `k >= 0` and every
  `n >= M + L + T + k D`:
  `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma_t`.
  `G_t` bounds, at every `n >= M`, the part of the output that the input from `M` on produces (the
  reference's response to that part, plus the rounding deviation it drives), for every admitted
  history before `N`; where the node's state is exactly zero at `M`, that part is the whole output.
- **(N3) Rest.** #1329 D2's `RestSamples`, read with "zero from `M`" (`M >= N`) for "zero from `N`".

Why these shapes:

- **Two stalls (#1484, root ruling of 2026-10-08 on #1466 attempt 1).** `sigma_p` bounds the flush
  part of the output at every frame and `sigma_t` only from the node's tail on; (N2)'s frames are a
  subset of (N1)'s, so `sigma_t <= sigma_p`. For a fixed design `F a` holds at every frame and
  `sigma_p = sigma_t`. For the live input section the pre-`N` and window analysis amplifies `F`
  through `sup Psi Phi_F`, so its every-frame flush bound is 538-1,164 times (+54.6 to +61.3 dB)
  its stall from `T`; one `sigma` would carry the every-frame value into (N2) and raise each live
  strip's share of `P*_graph` by that much. Every use that bounds a signal at every frame reads
  `sigma_p` (`CompositionBound::peak_clause`), every use from the node's tail on reads `sigma_t`
  (`CompositionBound::tail_clause`).

- **The share `3/4`.** For a fixed design it is #1329's split: `T_decay` certifies `eps / 2` for the
  exact reference and `eps / 4` for the relative rounding; the last quarter is the stall's. So (N2) at
  `k = 0`, `epsilon = 0` is #1329 D1 for every peak `P >= P^ := 4 sigma_t / eps`. #1329's own floor
  (`P* = F a / (eps/2 - g dev_core)`, `math::tail::CascadeBound::flush_floor`) satisfies
  `P* <= P^ < 2 P*`. The live bound splits `eps / 2` for the whole relative part and `eps / 2` for
  the stall (`crates/math/src/tail.rs:1494-1496`, `:1675`); (N2) at `3/4` is still valid for it, and
  there `P^ < 2 P*` holds only to within the bound's `SLACK` factor. Nothing #1329 certified changes.
- **Two deviation terms, two symbols.** `dev_core` is #1329's deviation supremum from `T_decay` on
  (`dev_sup_from_core`, `tail.rs:2245-2251`); it feeds `P*`. `dev_loud` is the output deviation while
  the input is loud: the fixed point `Deviation::at_end` reaches (the last section's `difference`,
  `tail.rs:1947-1961`; "before `N` the drives are at their suprema and the deviation is at most the
  fixed point", `:1929-1932`), per unit of `|trim| P` (`:1935`). After `N` the drives only fall, so
  `dev_loud` bounds the deviation at every frame. `G_p` uses `dev_loud`, never `dev_core`. Today the
  loop computes it and discards it.
- **The peak range (finding 2).** The bundled statement `|y| < P eps 10^(-k)` follows from (N2)
  exactly when `P >= P^ 10^k`, the raised floor. The node exports `sigma_t`; the range is derived from
  it. So the decade law is stated in its bundled form only where it holds, and in its additive form
  for every peak. Below the raised floor the node's output is not relative to `P`, but it is still
  bounded, which is what composition needs.
- **The tail gain `G_t`** is what amplifies an upstream residual. `G_p` also covers the state a
  pre-`N` history builds and a retarget exposes. They differ by tens of dB for a live input section:
  an `f64` probe (H9) drives a top-cutoff HPF at the Nyquist rate, which builds `ic1` near `-2.7e4`
  while the output stays near 1, and a 64-frame retarget toward 10 Hz then peaks at `2.2e3` (+67 dB
  before trim). For a fixed design `G_t = G_p`: its words never change, so an input that arrives after
  `N` meets the same operator as one before it.
- **`D = 0`** means the `X` part is exactly zero from `M + L + T` on, for every `k`: gain-only
  nodes, sums, pure delays.
- **Not an absolute output floor.** `sigma_p` and `sigma_t` are internal terms of the contract that
  composition carries. No node and no graph report states `|y| < max(P eps, floor)`: #1329 Amendment 3, G2
  stands, and the graph statement (H4) stays relative above its floor and exactly zero below it.
- **Certified for every `k`, never only at sampled `k`.** The construction (binding): compute the
  certified crossing `T(k)` of the floor `(3/4) eps 10^(-k)` directly, by the machinery that computes
  `T`, for `k = 1..K` with `K = 16` (the exact share `eps/2 10^(-k)`, the rounding share
  `eps/4 10^(-k)`, the horizon extended), all `K + 1` crossings from one extended pass (horizon
  `T(16)`, about 2.8 `T` at the top pair; one run per `k` would cost about 16-30 times); cover every `k > K` by a contraction certificate on #1329's
  non-negative majorant and deviation propagations: a positive `v` and a rate `lambda < 1` with
  `M v <= lambda v`, so the bound from any frame `t0 >= T` falls by `lambda` per frame. Let
  `D_inf = ceil(ln 10 / -ln lambda)` and `T_lambda >= T` the first frame at or after `T` from which
  the certificate's bound is below the `k = 0` floor. Then
  `D = max(max_{k <= K} ceil((T(k) - T) / k), D_inf + ceil((T_lambda - T) / (K + 1)))`.
  For `k > K`: `T + k D >= T + k D_inf + (T_lambda - T) = T_lambda + k D_inf >= T(k)`, because
  `k >= K + 1` and `T_lambda - T >= 0`. The exact suffix of the top pair does not fall by a constant
  number of samples per decade (36k-51k samples per decade at 44.1 kHz, H9), so a slope alone is not
  a certificate. The majorant is block lower-triangular with non-negative entries, so its spectral
  radius is `max_s (q_s + mu_s)` over the enabled sections and every valid `lambda` is at least it:
  `D_inf >= ceil(ln 10 / -ln(max_s (q_s + mu_s)))` (gate F1(d)).

### H2. The carriers (ruling (c), Q4)

One struct for every node. Slice A1 renames `effect_contract::EffectTailBound` to `NodeTailBound`
(once; no alias, no parallel struct) and adds one field. The implementer may refine the other names
(no version suffix) and states them in `docs/EFFECT_CONTRACT_V1.md`:

```rust
pub struct NodeTailBound {
    pub tail: TailSamples,
    pub tail_every_peak: TailSamples,
    pub rest: RestBound,
    /// H1's five values; `Unstated` until the node's slice derives them (#1378 retires it).
    pub composition: CompositionBound,
}
pub enum CompositionBound {
    Stated {
        decay: TailDecay,
        peak_gain: PeakGain,
        tail_gain: PeakGain,
        peak_stall: FlushStall,
        tail_stall: FlushStall,
    },
    Unstated,
}
pub struct TailDecay(pub u64);                // D: samples per further 20 dB
pub enum PeakGain { Zero, Millibels(i32) }    // rounded up; negative is an attenuation
pub enum FlushStall { Zero, Level(i32) }      // sigma_p, sigma_t: millibels re 1.0, rounded up
```

- The five values are stated together or not at all: there is no partial state.
  `CompositionBound::Unstated` is the counterpart of `RestBound::Unstated`.
- `PeakGain::Zero` is a node whose output is exactly `+-0.0` for every input (a sum with no inputs; a
  fader or route muted at preparation whose value cannot change, H3). In the composition it is
  `-infinity`: absorbing in a sum, neutral in a maximum.
- `PeakGain::Millibels` is signed: a fixed input section with its trim below 0 dB, or a node that only
  attenuates, states an attenuation; `k` is clamped at 0 (H4), so a negative sum never shortens a
  node's tail below `T`.
- `PreparedEffectMetadata` and `EffectProgramKey` carry `composition` as they carry `rest`;
  `expected_prepared_metadata` copies it; `effect-compiler`'s mismatch check compares it. This needs
  #1461 first: no control-only bytes in render-owned memory (#1329 R5).
- `NativeEffectRegistry::new` (#1462) refuses two inconsistent statements with a typed error:
  `Stated` with `tail` `Infinite`, and `tail_gain > peak_gain` (`Zero` below every `Millibels`).
- `builtins::InputSectionBound` is **deleted**. `input_section_bound`, `input_section_bounds`,
  `input_section_live_bound` and `PreparedBuiltinsSession::input_bounds` return `NodeTailBound`.
  The field `rest: None` becomes `rest: RestBound::Unstated`; a function that returns `Option` today
  (`None` at a rate off the launch set) keeps that return. `InputSectionBound::max` becomes a
  function on `NodeTailBound` with the same componentwise rule (the composition values by maximum;
  `Unstated` if either side is). The builtins state their tails in the same struct as the effects.
- `math::tail::CascadeBound` is a computation result, not a carrier: A2 adds the raw values (decay
  frames, the `dev_loud` term, the linear gains, the stall) that `builtins` rounds up into millibels.
- `graph::GraphNode`'s `tail: TailSamples` becomes the node's `NodeTailBound` (slice C). It carries
  `tail_every_peak` although composition never reads it, because ruling (c) forbids a second,
  narrower struct; the canonical text prints it so the plan text shows each node's whole bound.
  `GraphNode` is control-side plan data (`PreparedGraphPlan::spec`; the runtime reads node IDs from it
  while it binds); the implementer verifies that no render-owned structure holds a `GraphNode`, and if
  one does, the slice stops and reports.
- `GraphCompileReport` gains `output_tail_every_peak: TailSamples` and `output_flush_floor`
  (`P*_graph`: `Zero`, `Millibels(i32)` re 1.0 rounded up, `AboveRange` when the stall sum is not
  finite (H4 step 6), or unstated when `output_tail` is `Infinite`) beside `output_tail`.
- The canonical plan text: each node's `tail` row carries the node's whole bound (`tail`,
  `tail_every_peak`, the two rest values, decay, both gains, stall; `unstated`, `zero` and
  `above_range` spelled as tokens); a new `extent` row carries `output_tail`, `output_tail_every_peak` and `P*_graph`. The
  `node` rows and the DOT text keep `tail` as today.

### H3. Each node's values

Every gain is computed from an upper bound of the linear gain, rounded up to the next millibel, with
the `f32` rounding of the gain word and of each product included. "Underflow" is `2^-126` per
rounded operation on the output's dependency chain (flush-to-zero or gradual underflow, as
`math::tail`'s `UNDERFLOW`), rounded up into a `FlushStall::Level`.

| node | `T` | `D` | `G_p` | `G_t` | `sigma_p` | `sigma_t` | rest |
|---|---|---|---|---|---|---|---|
| track `Input` (source; delay line, D2a) | `max(left, right) delay_samples` | 0 | 0 mB | 0 mB | `Zero` | `Zero` | `T` for both peaks |
| submix `Input` (sum of `m >= 1` route edges; delay line if any) | max delay | 0 | `m (1 + u)^(m-1)` | same | `(m - 1)` underflows | `(m - 1)` underflows | `T` for both peaks |
| submix `Input`, `m = 0` | max delay | 0 | `Zero` | `Zero` | `Zero` | `Zero` | `T` for both peaks |
| `PostInputBuiltins`, fixed design (#1329 D4) | #1329's | H1 certificate | `|trim| (1 + u) (O + dev_loud)` | `= G_p` | the module's stall | the module's stall | #1329's |
| `PostInputBuiltins`, filters disabled | 0 | 0 | `|trim| (1 + u)`, or `|trim|` where the product is exact (a power-of-two trim, 0 dB included) | same | underflow | underflow | `ZERO` |
| `PostInputBuiltins`, live input lane | #1433's | H1 certificate, the maximum over the settled groups (slice B1) | supremum over every admitted history (slice B1) | derived (slice B2) | the every-frame flush bound (slice B1) | the live stall from `T` (slice B1) | #1433's |
| `PostSimd1`, `PostDynamic`, `PostSimd2PreFader` (identity boundaries) | 0 | 0 | 0 mB | 0 mB | `Zero` | `Zero` | `ZERO` |
| effect | its prepared metadata | | | | | | |
| `PostFader` (fader, mute, VCA) | 0 | 0 | live: the `f32` word of +24 dB; fixed: the prepared effective gain, or `Zero` if muted | same | underflow (`Zero` if `G_p` is `Zero`) | underflow (`Zero` if `G_p` is `Zero`) | `ZERO` |
| `PostMatrix` (2x2 matrix, pan) | 0 | 0 | live: row `l1` bound 2; fixed: the largest prepared row `l1` | same | 2 underflows | 2 underflows | `ZERO` |
| `Route` | 0 | 0 | live: `2 * 10^(ROUTE_GAIN_DB_MAXIMUM / 20) * ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM`; fixed (its own gain and matrix fixed): the prepared ungated gain times the largest prepared ungated row `l1`, or `Zero` where the mute rule below allows it; each with the fold's rounding | same | 2 underflows (`Zero` if `G_p` is `Zero`) | 2 underflows (`Zero` if `G_p` is `Zero`) | `ZERO` |
| `Output` (sum of `m >= 1` routes; `m = 0` as the submix row) | 0 | 0 | `m (1 + u)^(m-1)` | same | `(m - 1)` underflows | `(m - 1)` underflows | `ZERO` |

- **Fixed input section.** `O` is `math::tail`'s output majorant summed over all frames,
  `sum_t o(t)`, as the module accumulates it, with its `SLACK accumulation(n)` factor for the `f64`
  summation; it bounds the cascade's `l1` norm for every reset pattern (a reset only drops
  non-negative terms). `dev_loud` is H1's loud-input deviation, per unit of `|trim| P`; A2 makes
  `CascadeBound` return it (it is not `dev_core`). The stall is `F` times the absolute output's fixed
  point, which holds at every frame. `O` and the stall are computed inside `fixed_cascade` today;
  `dev_loud` is computed and discarded. The per-section triangle inequality inside `O` is sound but
  about 14 dB loose at the top pair (`O + dev_loud` = 3.50 against an exact cascade `l1` of 0.697);
  removing it is a tightening (m6), owned by *Tighten the fixed input section's peak gain past the
  cascade triangle inequality* (#1468).
- **Live input section** (slices B1 and B2). `G_p` (B1) comes from #1433's zone state bounds (`Phi`)
  and the LPF's state bound, through the output row and the feedthrough, over the 65-frame window
  and every settled group, with the trim word of +24 dB. `G_t` (B2) is derived, never set to `G_p`: by H1's
  definition, from a state that the input from `M >= N` on builds during at most 64 ramp frames
  (words in the convex hull of the designs plus #1407's rounding allowance), then the settled design's
  response, over every settled group, plus the deviation it drives, with the trim word of +24 dB.
- **Gain-only nodes: prepared values unless live** (M3). A gain value is *fixed* when nothing can
  change it after preparation: no live lane carries it and the plan carries no ramp into it. Then the
  node states the prepared value, as the maximum over the prepared word and every ramp endpoint the
  plan starts with. Otherwise the node states the domain maximum. Today's lanes
  (`crates/host-core/src/prepare.rs:371-397`, `:1268`, `:1580-1597`): the fader, mute, VCA, matrix and
  pan values are live whenever the plan has a control queue (`HostLiveLanes::FADER_AND_MATRIX` is
  the least selection with one); a route into a submix is live when `lanes.routes` and a control queue
  are both set; a route into the output has no lane. A route's `G_p` is its prepared ungated value
  (its gain times its largest ungated row `l1`) whenever its own gain and matrix are fixed:
  follow-mute only zeroes coefficient columns, never the gain
  (`gated_route_coefficients`, `crates/graph-compiler/src/compile.rs:327-333`), and a route without
  a lane cannot change its follow state (`crates/host-core/src/prepare.rs:1582-1584`). The source
  strip's mute decides only whether `Zero` may be stated: a fixed route states `Zero` only when it is
  muted at preparation, or follows a source strip whose mute is fixed and muted. **Carried values.**
  #1277 D4 carries a stage only when both plans attach the same control kind and no prepared value
  differs, and #1276 D1 carries an input section only when the live kind matches and the sections
  are bit-equal. So every carried gain is either on a live lane (it states the domain maximum) or
  equal to the successor's prepared value with no ramp, and the compile needs no carried endpoint.
  Slice C checks that #1277 D4 and #1276 D1 still read so on `main`, and stops and reports if either
  does not. The graph compile receives this selection explicitly from its caller (no default).
- **Domain maxima.** The fader's effective gain is clamped to +24 dB after VCA offsets
  (`session::vca_effective_db`, `crates/session/src/vca.rs:23-34`). The route's bound reads
  `session::ROUTE_GAIN_DB_MAXIMUM` and `session::ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM`, never copies of
  them: without *Bound route gain and matrix values* (#1237) a route's gain had no bound (about
  +770 dB), so its `PeakGain` is finite only under #1237's bounds.
- **A prepared mute is `Zero`** only where the kernel's output for a settled mute is exactly `+-0.0`
  for every finite input (a product by `0.0`, or a cleared buffer); slice C verifies this per kernel,
  and states the unmuted gain where it does not hold.
- **Effects** state `CompositionBound::Unstated`, all eight and the conformance mock. Each needs its
  own derivation (#1372-#1376); a gain "from the parameter domain" with no analysis behind it would
  be an uncertified number. Under D4 every graph with an effect therefore reports `Infinite`,
  including the gate, transient shaper and soft clip, whose tails are finite today; #1376 follows
  slice A2 (Q3).
- **Compensation delays** stay edge shifts in the extent (exact copies with no gain), not nodes.

### H4. The composition (ruling (b))

**The graph contract.** Every graph input (each source) has peak at most `P` and is zero from `N`;
no control event happens at or after `N`. The plan reports:

- `output_tail`: for every `P >= P*_graph`, `|y_out[n]| < P eps` for every
  `n >= N + output_tail`. `output_tail` includes the output latency, as
  `GraphCompileReport::output_tail` already does (its extent adds every latency on the path).
- `output_tail_every_peak >= output_tail`: for every `P > 0`, from `N + output_tail_every_peak` on
  the output is below `P eps` (`P >= P*_graph`) or exactly `+0.0` or `-0.0` (`P < P*_graph`). Hosts
  report this value (Q2).
- `P*_graph`: the graph's flush floor, in graph-input terms; `Zero` when `Sigma = 0` (step 6), and
  then the decay branch holds for every `P > 0`; `AboveRange` when the stall sum is not finite, and
  then the exact-zero branch holds for every `P > 0`.

**Range hypothesis.** The node statements are applied to the computation without overflow. At a
memoryless node (fader, matrix, route, sum) a product above `f32::MAX` becomes `inf`, and a sum of
`inf`s can become `NaN`; nested submixes at domain maxima can reach that for inputs near the
sanitizer limit. Every such value is replaced before it reaches a stateful node: every input section
sanitizes `|x| >= 1e30` and non-finite samples to `+0.0` (`NONFINITE_LIMIT`,
`crates/lane/src/kernels/builtins.rs:27`, `:588-591` on `main` `7e8379523`; about `:78` and
`:639-642` after #1454), zeroes a lane block and resets its state on a
bad output (`crates/builtins/src/lib.rs:1861-1876`), and no effect receives such a sample in a
compiled plan (`crates/effect-contract/src/live.rs:860-862`). A replacement by `+0.0`, a zeroed block
and a reset state each satisfy every bound the induction below carries (each bound is on a magnitude,
and a reset drops non-negative majorant terms), so every bound holds a fortiori. At the output, for
`n >= N + output_tail` every term is at most `eps P <= eps 1e30` times a finite path gain, so no
overflow is reachable there.

**The rule.** Two passes over the schedule, `O(nodes + edges)`, on the control thread, in `i64`
millibels (`Zero` as `-infinity`) except the stall sum:

1. *Signal edges* are main edges and route edges. A sidechain edge carries no magnitude (each node's
   values hold for every sidechain input, H1); it enters the two time recurrences (steps 5 and 7) as
   today (PDC aligns sidechains). `m_v` is `v`'s signal in-degree.
2. *Backward:* `down(v) = max over signal successors w of (G_t(w) + down(w))`, `down(output) = 0`,
   `down(v) = -infinity` if no signal path leads from `v` to the output; `nd(v)` is the same maximum
   of the count of nodes with `D > 0`, `v` excluded.
3. *Forward:* `up(v) = max over signal predecessors u of (up(u) + G_p(u))`, `0` at a source; `nu(v)`
   is the same maximum of the count of nodes with `D > 0`, `v` excluded.
4. `A(v) = up(v) + down(v) + ceil(2000 log10(nu(v) + nd(v) + 1))` millibels;
   `k(v) = max(0, ceil(A(v) / 2000))` if `D_v > 0` and `A(v) > -infinity`, else `0`. Every node with
   `D > 0` has exactly one signal input (an input section, an effect's main input); a node with
   `D > 0` and fan-in needs an amendment before it composes.
5. *Decay branch:* `extent(v) = max over incoming edges of (extent(u) + comp(u -> v)) + L_v + T_v
   + k(v) D_v` (today's recurrence plus `k D`); `output_tail = extent(output)`.
6. *Floor:* `S_out(v) = g_p(v) max_u S_out(u) + sigma_p,v` (`sigma_p,v` at a source; `0` after a
   `Zero` gain; it feeds `X_v` in chain (i), an (N1) use, so it reads the peak stall),
   `S_in(v) = max_u S_out(u)` over signal predecessors; the effective stall
   `sigma'_v = sigma_t,v + (3/4) eps 10^(-k(v)) S_in(v)` if `D_v > 0`, else `sigma_t,v` (it feeds
   `Sigma` in chain (iii)-(v), an (N2) use, so it reads the tail stall; #1484);
   `Sigma = sum over v of 10^(down(v)/2000) sigma'_v` (a term with `down(v) = -infinity` is `0`);
   `P*_graph = 4 Sigma / eps`, stated as `Zero` if `Sigma = 0`, else as
   `ceil(2000 log10 P*_graph)` millibels. Evaluated in `f64`, each operation inflated by
   `1 + 2^-30`, with `math::log` and `math::exp` (vendored: the same bits on every target) for every
   logarithm and exponential, so the canonical text stays target-independent and the stated
   `P*_graph` is strictly above `4 Sigma / eps` whenever `Sigma > 0`. *Overflow (m4):* if any value
   of this step is not finite (about 102 nested submix levels at domain maxima, about 6,007 mB each,
   take `10^(down/2000)` past `f64::MAX`; the session has no nesting cap), the floor is stated as
   `AboveRange`: every peak is below it, so the rest branch (step 7) is the bound for every input.
   This is not a refusal: the bound stays sound and finite. (A millibel value of a finite `f64`
   floor fits `i32`: `2000 log10(f64::MAX)` is about 616,500.)
7. *Rest branch:* if `P*_graph` is `Zero`, `output_tail_every_peak = output_tail`. If it is
   `AboveRange`, every `X*(v)` is taken as above +24 dBFS (so `R_v` is `any_sanitized_input` at
   every node) and `output_tail_every_peak = max(output_tail, rest_extent(output))`. Otherwise
   `X*(v) = 10^(P*_graph/2000) 10^(up(v)/2000) + S_in(v)` (linear, from the millibel values; `0` for
   the first term when `up(v) = -infinity`); `R_v` is `peak_plus_24_dbfs` if `X*(v)` is at most the
   `f32` word of +24 dB, else `any_sanitized_input`. `any_sanitized_input` covers every input a node
   can receive (range hypothesis), so no `X*(v)` leaves a node without a rest value.
   `rest_extent(v) = max over incoming edges of (rest_extent(u) + comp(u -> v)) + L_v + R_v`;
   `output_tail_every_peak = max(output_tail, rest_extent(output))`.
8. *`Infinite` wins (D4).* `output_tail` is `Infinite`, and `P*_graph` unstated, if any node from
   which the output is reachable (by any edge kind) has `tail` `Infinite` or `composition`
   `Unstated`. `output_tail_every_peak` is `Infinite` if `output_tail` is, or if `P*_graph` is not
   `Zero` and such a node's `rest` is `Unstated`. A node's own `tail_every_peak` is not read; its rest
   is.
9. *Diagnostics:* `graph.tail.arithmetic_overflow` for any `i64` overflow in a millibel sum or in
   either extent (with `i32` node gains, a millibel path sum overflows `i64` only past about `2^32`
   nodes on one path, so its check is a unit test of the sum helper, not a session gate); `graph.tail.limit` when `output_tail_every_peak` exceeds
   `maximum_finite_tail_samples` (Q7; it is at least `output_tail`, so the check covers both).

**The inequality chain** (the derivation note writes it in full):

- (i) *Induction in schedule order.* Let `M_v = max over incoming edges of (extent(u) + comp(u -> v))`.
  For `n >= N + M_v`, `|x_v[n]| <= epsilon_v`, where `epsilon_v` sums, over every node `j` upstream
  of `v`, `j`'s own relative residual `(3/4) eps 10^(-k_j) X_j` and its effective stall `sigma'_j`
  (from `j`'s tail stall `sigma_t,j`: it bounds `j`'s output from `j`'s tail on, (N2)),
  each carried to `v`'s input by the tail-gain path sum. A compensation delay is an exact ring, so a
  bound on `u`'s output from `N + extent(u)` holds for the delayed copy from
  `N + extent(u) + comp`; latency shifts by `L`; a sum adds its inputs with coefficient
  `(1 + u)^(m-1)`. For every `n`, `|x_v[n]| <= X_v <= P U_v + S_in(v)` by (N1) (so `S_in(v)` carries
  the peak stalls `sigma_p` upstream of `v`: it bounds every frame), with `U_v` the
  peak-gain path sum; `U_v <= 10^(up(v)/2000)` because a sum's `G_p` includes its fan-in `m`, so a
  path's product bounds `m` times the largest input.
- (ii) (N2) at `v` with `M = N + M_v` gives the step for `n >= N + extent(v)`.
- (iii) At the output: `|y_out| <= (3/4) eps P sum_j 10^(-k_j) U_j W_j + sum_j W_j sigma'_j`, with
  `W_j` the tail-gain path sum from `j` to the output. Every term holds from the nodes' tails on, so
  each `sigma'_j` reads `sigma_t,j` (N2), never `sigma_p,j`.
- (iv) *Path weights.* `U_j W_j = sum over source-to-output paths p through j of w_p Gamma_{p,j}`,
  where `Gamma_{p,j}` is the product of the other nodes' gains on `p` (peak gains before `j`, tail
  gains after it, a sum's gain including its fan-in) and `w_p` the product of `1 / m_s` over the
  nodes `s` on `p` that have a signal predecessor (`m_j = 1`, step 4). Steps 2-4 give
  `10^(-k_j) Gamma_{p,j} <= 1 / n_p` for every `p` through `j` (`n_p` the nodes with `D > 0` on
  `p`; the separate maxima only make `k_j` larger). So the first term is at most
  `(3/4) eps P sum_p w_p (sum over j on p with D_j > 0 of 1 / n_p) <= (3/4) eps P`:
  `sum_p w_p <= 1`, because `w_p` is the probability that a backward walk from the output, taking
  a uniformly chosen signal predecessor at each node, follows `p`. The same walk gives
  `W_j <= 10^(down(j)/2000)`.
- (v) For `P >= P*_graph`: `sum_j W_j sigma'_j <= Sigma` (step 6's `Sigma` sums the same
  tail-stall terms), and `Sigma < (eps/4) P*_graph <= (eps/4) P`
  when `Sigma > 0` (step 6's inflation makes the stated floor strictly larger), so
  `|y_out| < (3/4) eps P + (eps/4) P = eps P`. When `Sigma = 0`, `|y_out| <= (3/4) eps P < eps P` for
  every `P > 0`.
- (vi) For `P < P*_graph`: every node's input peak is at most `X*(v)`, or is replaced by `+0.0`
  (range hypothesis). Exact zero propagates: a node whose every input is exactly `+-0.0` from `M`
  rests by `M + L + R` (N3); a gain, a sum and a ring of `+-0.0` give `+-0.0`. So the output is
  `+-0.0` from `N + rest_extent(output)`.

**Fan-in, delay and the stall's absolute nature**, stated once: a sum's fan-in enters as its gain
(`20 log10 m` plus its rounding) on every path through it, and as a path weight in (iv); the extent
takes the latest input (a sum is quiet only when every input is); a stall is absolute, so it is not
scaled by the input, gains no benefit from upstream attenuation and is amplified by every downstream
tail gain, and stalls of different nodes add at the output. A node with `D = 0` takes no share of the
relative budget (`n_p` counts `D > 0` nodes only), which is why the strip delay (D2a) adds exactly
its delay.

**Where the ruling's sketch changes** (Q1, confirmed):

- *The floor.* The sketch's `max_i P*_i 10^((A_i + G_down_i)/20)` becomes
  `(4 / eps) sum_i W_i sigma'_i = sum_i W_i P^'_i`. A stall does not scale with the input, so
  upstream gain does not raise it (the upstream part of `A_i` does not belong); the decade factor
  `10^(k_i)` is not needed (below a node's raised floor (N2) still holds additively); and stalls of
  different nodes add (a sum with fan-in weights in `W_i`, not a maximum). For a one-node graph the
  rule gives `P^_1`, which is at least `P*_1`.
- *The rest branch.* The sketch's per-node `max(T_i + ceil(A_i/20) D_i, R_i at the raised floor)`
  becomes the two graph-level branches. A node rests only after its input is exactly zero, which
  happens only after every node upstream rests, so rests add along a path from the upstream rests,
  not from the upstream decay extents. The sketch's `R_i` at the node's own raised floor is unsound
  when upstream gain raises the node's input peak; taken at the node's input-peak bound at
  `P*_graph` it is sound but never shorter than the maximum of the two branches.

### H5. The exact-zero gate (ruling (d))

Gate C6: on the real kernel, an input of peak `P*_graph / 2` and then silence. Every output sample
from `N + output_tail_every_peak` on is exactly `+-0.0`. The gate also asserts its own precondition,
per rate, for at least one input pattern (DC or the worst-sign pattern): the last non-zero output
sample falls at or after `N + output_tail`, so that run needs the exact-zero branch. Other patterns
(random, alternating) are run for (i) and their last non-zero sample is recorded; an alternating input
through a 1 kHz LPF is attenuated so much that it can rest before `N + output_tail`.

**The design list (m9), the same at every launch rate.** The precondition is not shown in advance
for every rate, so the gate tries these designs in order, per rate, and uses the first that meets
the precondition for DC or the worst-sign pattern:

1. **The 1 kHz input LPF (HPF disabled), trim 0 dB, every downstream gain at 0 dB** (fader 0 dB,
   identity track matrix, one route to the output at 0 dB with an identity matrix, a source with
   identical channels). Only rounding remains in `A`, so `k` is 0 or 1 and `output_tail` is at
   most about `T + D` (about 200 samples at 44.1 kHz), while `P*_graph` is about the LPF's own `P^`, so the
   input peak `P*_graph / 2` lies between about `P*/2` and `P*`. #1329's gate 1(c) record: the real
   kernel rested by frame 384 at 44.1 kHz from peaks at and below its own `P*` (`8.8e-12`), against
   a `T_decay` of 175.
2. **The same LPF in C1's gain shape** (fader, matrix and route at their maxima, `k = 4`): its rest
   from `P*_graph / 2` and its `output_tail` differ by a few samples by estimate (about 280 against
   275 at 44.1 kHz, about 293 against 302 at 48 kHz).
3. **A design whose real kernel holds a sub-`REST_EPS` state until the joint flush arms** (rest near
   `N_SILENCE`: 3,764 / 4,096 / 7,527 / 8,192 frames at 44.1 / 48 / 88.2 / 96 kHz) with an
   `output_tail` below that. Its existence is not shown (the window is narrow by estimate); the
   implementer searches the cutoff domain for one and records the search.

The slice records, per rate, which design it used and the last non-zero sample. If no design on the
list meets the precondition at a rate, the slice stops and reports to root: the gate is not weakened
and the precondition is not dropped.

### H6. What is not claimed

- Nothing for a control event at or after `N`: a fader move, a route edit, a VCA change, a mute, a
  seek or a plan swap during the tail starts a new history (the swap slices state theirs, #1269).
- Nothing about a meter tap, a send tap or any node output other than the graph output.
- Below `P*_graph`, only exact zero from `output_tail_every_peak`; no relative bound before it.
- Tightness beyond the gated lines. The cascade's triangle inequality between sections and the live
  cascade's slack make the bound longer than the real kernel needs; gates F1(b) and F2(b) bound that
  looseness, L1 records its ratio, and every measured margin is recorded. The looseness is sound, so
  no correctness gap is deferred; the fixed `G_p`'s triangle inequality is tightened by the
  successor *Tighten the fixed input section's peak gain past the cascade triangle inequality*
  (#1468).
- Feedback (the graph stays acyclic) and a sidechain's magnitude (each node's values hold for every
  sidechain input; an effect whose bound depends on its sidechain stays `Unstated`).
- A graph-level `RestSamples` (exact rest for every input up to +24 dBFS) and any render-side
  consumer: silence skipping (#1107) reads each node's `RestSamples` per lane, not this composition
  and not `T_rest`.
- That the bound holds for an engine that skips the per-word flush or runs another kernel: it holds
  for the kernels whose words, roundings and flush laws the node values were derived from.

### H7. Interactions

- **#1107.** Unchanged: it uses node `RestSamples` (#1329 addendum A2). Nothing here feeds render.
- **#1457.** Its D1 budget is a stated, gated figure for the worst case (#1457 R1), binding both
  hosts (R2). A2 lengthens a fixed design's computation (`T(k)` for `k <= 16` from one extended pass
  over about 16 more decades: about 2.8x for the top pair, 8.8 ms to about 25 ms per design, by
  estimate; one run per `k` would cost about 16-30x) and B1 and B2 the live bound's (about 250 us
  since #1433, cached per rate by #1457). A2, B1 and B2 each rerun #1457's gates 2 and 4 and pass
  them; a miss stops the slice. #1457's cache key does not change; its value
  type becomes `NodeTailBound` in A1.
- **#1461.** Lands before A1 (the composition field grows `PreparedEffectMetadata`).
- **#1462.** Lands before A1; A1 adds H2's two registry rules in `NativeEffectRegistry::new`.
- **#1372-#1376 and #1378.** Each per-effect slice states its `CompositionBound`; #1375 and #1376
  follow A2 and land in the batch with C (Q3, third round); their dependency on #1379 becomes A1 and
  A2 (they need only the types and the contract). #1375's D4 is restated to H1 as #1376's below. #1376's D4 ("Gain (#1379 D1: a peak gain and an incremental gain after
  silence)") is restated to H1: the gate, transient shaper and soft clip state `D`, `G_p`, `G_t` and
  `sigma` under H1's contract, with H1's sidechain quantifier (the gate has a sidechain, and its gain
  bound `g <= 1` holds for every sidechain input). #1378 retires `CompositionBound::Unstated` with
  `Infinite` and `RestBound::Unstated`. Root records these dependencies and #1376's restated D4 in
  #1372-#1376 and #1378.
- **#1237, #1407, #1408.** The route, fader, trim and matrix bounds rest on them.
- **#1276, #1277, #1279 (stream A carry).** #1277 D4 and #1276 D1 carry a gain only when it is on a
  live lane or equal to the successor's prepared value with no ramp, so H3's fixed/live rule covers
  every carried value; slice C checks that both still read so.
- **Stream F (#1261, #1262).** They report the every-peak value through host-core's report (H10);
  slice C changes the copy, and no ABI or wire field changes.

### H8. The split and the order (smallest closable slice first)

The whole issue is about three working days (types, two derivations of which the live one has two
parts, the composition, real-kernel gates and re-pins). Five slices, each self-contained and
independently verifiable, in this order, and one tightening successor:

| slice | issue | title | after (same stream) | after (other streams) | estimate |
|---|---|---|---|---|---|
| A1 | #1464 | Carry every node's tail bound in one node-neutral struct | #1457, #1461, #1462 | — | half a day |
| A2 | #1465 | State a fixed input section's decay, gains and flush stall | A1 | — | half a day |
| B1 | #1466 | Certify the live input section's decay, peak gain and flush stall | A2 | — | half a day |
| B2 | #1467 | Derive the live input section's tail gain and state its composition | B1 | — | half a day |
| C | #1379 | Define how node tails compose through gain in the graph extent | B2 | #1237; its `graph-compiler` turn (after A #1285, J #1384, C #1287 first slice) | about one working day (exception, below) |
| G_p | #1468 | Tighten the fixed input section's peak gain past the cascade triangle inequality | A2, B2 (shared files) | — | half a day |

#1375 and #1376 follow A2 and land in the batch with C (Q3, third round). A1, A2, B1 and B2 share
`crates/builtins/src/tail.rs` and `tail_contract.rs`, so they are sequential; A2, B1, B2 and the
`G_p` successor also share `crates/math/src/tail.rs`. The `G_p` successor may land before or after
C; if after, it re-pins the graph digests its tighter value moves, one at a time.

**B's size (root ruling 4).** With `G_t` a full derivation (it was `G_t = G_p` in the first draft),
B is about one working day: the live `D` by H1's construction over every settled group, the live
`G_p` as a supremum over every admitted history, the stall, and the new `G_t` derivation with its
real-kernel gate. It is split at the line that keeps each half verifiable on its own. B1 certifies
`D`, `G_p` and `sigma` in `math::tail`'s live cascade and exposes them through test-support
accessors; the live bound keeps `CompositionBound::Unstated` (H2 forbids a partial statement), and
B1's real-kernel gate (L1) and recomputation (L3) check the accessors. B2 derives `G_t`, gates it on
the real kernel (L2), and switches the live bound to `Stated`. Nothing reads the live values before
C, so neither half moves a reported tail.

**C's size (m7, root ruling 3).** C is about one working day, an explicit exception to AGENTS.md's
half-day rule, recorded here and in C. The carriers, the rule and the report are about half a day,
and the real-kernel release gates (C1, C2, C6, C11, C12) with the re-pins are the other half. The
gates are #1379's product evidence (ruling (d) requires C6), and host-core copies the graph's value
into the C ABI's `tail_samples`, so a first half without them would move the reported tail on
recomputation evidence alone: the halves would not be independently verifiable. C is not split.

**Hot-file slots** (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`, root adds them):

- `crates/effect-contract/src/lib.rs` (`:78`): `... → G #1462 → G A1`.
- `crates/effect-compiler/src/prepare.rs` (`:75`): `G #1462 → G A1` (one comparison), in either order
  with B (#1315, #1345); the later slice rebases.
- `crates/builtins-compiler/src/lib.rs` (`:76`): `G (#1329) → G A1` (the bound type, the seal,
  `input_bounds`, the tail-entry charge), in either order with F (#1261, #1262) and J; the later
  slice rebases.
- `crates/parametric-eq/src/lib.rs` (`:90`): G A1 (the `tail_and_rest` field only) before G #1372, in
  either order with A's payload slices; the later slice rebases.
- `crates/transient-shaper/src/corpus.rs` and the other `PreparedEffectMetadata {` literals in the
  effect crates' payload code and test helpers: G A1 (the field only), in either order with A's
  payload slices; the later slice rebases.
- `crates/graph/src/{lib,runtime}.rs` (`:88`): G A1 (the rename only, `lib.rs`) and G #1379
  (`GraphNode`'s field), each after G #1461, in either order with A, B, C and D; the later slice
  rebases.
- `crates/graph-compiler/src/*` (`:89`): G #1379 as listed.
- `crates/host-core/src/prepare.rs` (`:70`): G #1379 (the tail copy and the gain-liveness selection
  passed to the graph compile, a few lines), in either order with the others; the later slice
  rebases.
- `.github/workflows/*.yml` (`:100`): G #1379 (one `test-release` step), in any order; the later
  slice rebases.

**Named exceptions** (stream A owns `crates/builtins*`, `crates/graph`, `crates/graph-compiler`,
`crates/host-core/src/prepare.rs` and the effect crates' payload code, `STREAMS.md:167`; stream B
owns `crates/capi`, `:190`; stream F owns the `crates/capi` tests, `:286`; stream S0 owns
`docs/rulings/`, `:156`; root adds these to stream G's row, `:305`):

- A1: `crates/builtins/src/tail.rs` (the bound type), `crates/builtins/src/lib.rs` (the bound entry
  points and re-exports), `crates/builtins/tests/tail_contract.rs`; `crates/builtins-compiler/src/lib.rs`
  (the bound type, the seal, `input_bounds`, the tail-entry charge, their unit tests);
  `crates/graph/src/lib.rs` (the rename only); the effect crates' `tail_and_rest` functions (the
  rename and the field only); the `PreparedEffectMetadata {` literals in the effect crates' payload
  code (`crates/transient-shaper/src/corpus.rs:190`) and test helpers (the field only).
- A2, B1, B2 and the `G_p` successor: `crates/builtins/src/tail.rs`,
  `crates/builtins/tests/tail_contract.rs`.
- C: `crates/graph/src/lib.rs` (`GraphNode`'s field and the test literals that build one);
  `crates/graph-compiler` (the extent, as #1379 already has, and every `GraphBuiltinsCompileRequest {`
  literal in its tests, the new field only); `crates/host-core/src/prepare.rs` (the tail copy and the
  selection); the `GraphBuiltinsCompileRequest {` literals in `crates/host-core/tests/*` and
  `tools/audit/src/fixture_builtins.rs` (the new field only); `crates/capi/src/runtime/compile.rs`
  (the one read of the renamed field) and `crates/capi/src/abi.rs` (two doc comments), stream B's;
  `crates/capi/tests/tail_every_peak.rs` (new, C12), stream F's;
  `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md` (stream S0's: H10's
  text only).
- The `G_p` successor, only if it lands after C: `crates/graph-compiler/tests/track_delay.rs` (the
  digest), `fixtures/graph/v1/direct-route.*`, `fixtures/graph/MANIFEST.tsv` and any other
  canonical pin its tighter value moves, each with its reason.

### H9. Evidence for this amendment (`f64` probe, exploration only)

A standalone C program (kept outside the tree in the session scratchpad, not committed) computes
the `f64` impulse response of the designed `f32` words (`SvfSection::design`'s formulas) and its
suffix sums `S(j)`:

- Top pair (HPF one `f32` below the maximum into the LPF at the maximum), 0 dB trim. `T_b(eps/2)`
  and the steps to each further decade, `k = 1..12`: 44.1 kHz 393,996 (equal to #1329's record);
  49,729, 43,132, 38,891, 50,823, 49,677, 44,694, 36,270, 50,520, 49,551, 44,764, 35,731, 50,361.
  48 kHz 391,854; 49,394, 42,202, 39,447, 50,558, 49,329, 43,967, 36,624, 50,273, 49,184, 43,952,
  36,184, 50,119. `l1` 0.6967 / 0.6959. The decade length varies by about 30 %.
- 10 Hz HPF into the LPF at the maximum, 48 kHz: `T_b(eps/2)` 335,140, steps 32k-50k, `l1` 3.51.
  Single sections at 48 kHz: HPF one `f32` below the maximum 1.09, LPF at the maximum 2.43, 10 Hz
  HPF 2.43, 1 kHz HPF 2.26, 1 kHz LPF 1.09 (so the top pair's cascade, 0.696, is 11.6 dB below the
  product of its sections' norms).
- 1 kHz LPF, 48 kHz: `T_b(eps/2)` 190, steps 18-28 samples.
- Worst-sign residual `S(tau + 1)` relative to `eps` at unity gain, top pair, 0 dB trim: at
  44.1 kHz and `tau` = 420,148 (#1329's `T_decay`), -19.3 dB; at 48 kHz, -19.4 dB at 417,700 (an
  estimate of `T_decay` by the same ratio to `T_b`), -104.6 dB at 620,000, -197.2 dB at 820,000.
  With the fader at +24 dB alone today's tail leaves +4.7 dB over the floor; with the fader, the
  matrix (row 2) and a route at +24 dB with row 2 (about +60 dB), +40.6 dB.
- Retarget transient: an HPF at the 48 kHz maximum driven by an alternating `+-1` input builds
  `ic1 = -2.70e4` with `|y| <= 1.04`; a 64-frame ramp to 10 Hz (or to 1 kHz) with zero input then
  peaks at `|y| = 2.17e3` (`2.13e3`). The LPF under the same history peaks at 1.36 (68).
- The reviewer's independent reproduction (2026-10-06): the 44.1 kHz top-pair figures above; C1 6007
  mB (`k = 4`), C2 `up` 8408 + `down` 6007 = 14,415 mB (`k = 8`), C4 13,515 mB one track (`k = 7`)
  and 14,118 mB two (`k = 8`).
- The confirmation review's probes (2026-10-06, an export of `f3956e63c`, deleted after), which
  supersede the first reproduction's 7 x 7 grid (that grid included HPF-above-LPF rows, which
  preparation refuses with `FilterOrder`, `crates/builtins/src/lib.rs:3452-3454`):
  - *F2(b)'s excess.* `O / (||h_HPF||_1 ||h_LPF||_1)` depends only on the LPF section,
    `(gamma beta / (1 - q) + |d|) / ||h_LPF||_1`. Over a 61-point log grid of the whole LPF domain
    it is at most +5.267 dB at every launch rate (LPF at 10 Hz; the limit as `fc/fs -> 0` is about
    5.27 dB, so it does not grow at 88.2 or 96 kHz), against F2(b)'s +6.02 dB line.
  - *`dev_loud`* (`Deviation::at_end`'s output `difference`, from the module's own sums): top pair
    at 44.1 kHz 0.2310 against `O` = 3.2719, that is about 7 % or +0.59 dB (about 59 mB); HPF at the
    maximum alone +0.44 dB; low-cutoff rows at most +0.005 dB.
  - *Margins of a correct `G_p` under F2(b)'s line*, with `dev_loud`, `(1 + u)` and millibel
    rounding included: gate 1(a)'s worst row (HPF 10 Hz into LPF 1 kHz) +0.94 to +1.16 dB; top pair
    +3.6 dB; HPF 10 Hz into the LPF one `f32` above 10 Hz +0.74 to +0.75 dB at every rate.
  - *The state-norm mutant* (each section's `||c|| ||b|| / (1 - q) + |d|`) on legal rows: top pair
    +1.07 dB over the line (red); HPF 10 Hz into the LPF one `f32` above 10 Hz +1.06 dB over (red);
    every row whose LPF is at the maximum about 2.3-2.4 dB under (green).
  - *F1(b)'s first branch*, `max_{k<=16} ceil((T(k) - T)/k)` with `T(k)` from the module's own
    `fixed_cascade`: between 1.00x and 1.12x `max(D_mean, D_floor)` on all 14 enabled rows at all
    four rates (trim 0 dB); worst the 1 kHz LPF at 48 kHz (28 against 25.0); top pair 1.067x
    (`T(1) - T` = 48,531 against `D_mean` 45,467); `T` reproduces #1329's 420,148. The module's
    majorant takes its longest step at `k = 1` (the polynomial factor of near-equal poles; steps
    48,531, 48,136, 47,816, ...).
  - *F1(b)'s certificate branch*, `D_inf + ceil((T_lambda - T)/17)` at the best `lambda`, five rows
    at 44.1 and 96 kHz: with the least valid certificate (the fixed point of
    `v = max(m(T), M v / lambda)`, checked `M v <= lambda v`) at most 1.056 `D_floor`, with
    `T_lambda - T` at most 0.42 decades; with a resolvent-aligned certificate
    `v = lambda (lambda I - M)^-1 m(T)` up to 1.42 `D_floor`. Charging the whole transient per
    decade (`D_inf + (T_lambda - T)`) gives about 1.45x under the least valid certificate (green
    under 1.5), so it is not a usable mutant; a certificate rate halfway between the floor rate and
    1 gives `D_inf` about 2 `D_floor`, above 1.5x on every row (MJ2).
  - *C6's 1 kHz LPF in C1's gain shape*, by estimate: rest from `P*_graph / 2` about 280 against
    `output_tail` 275 at 44.1 kHz, about 293 against 302 at 48 kHz (H5's list).
  - *The `P*_graph` estimate under M3's rule:* the route into the output has no lane, so it states
    its prepared value; with a 0 dB output route a live graph with a submix gives about -1 dBFS
    (against about +23 dBFS with every gain at its maximum).

### H10. Decision 15 D15-4(b) and (c), amended (applied by slice C)

Slice C replaces, in `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
the sentence "`T_decay` is the bound above for every peak at or above the node's flush floor `P*`
(below it the `f32` output near the 1e-20 per-word flush is no longer relative to `P`); tail
reporting uses it (#1261, #1262, PDC)." with:

> `T_decay` is the bound above for every peak at or above the node's flush floor `P*` (below it the
> `f32` output near the 1e-20 per-word flush is no longer relative to `P`); the graph extent
> composes it, with each node's decay, peak and tail gains and flush stall (#1379). The graph states
> two values (root, 2026-10-06; #1379 Amendment 1): `output_tail`, valid only for graph input peaks
> at or above the graph's flush floor `P*_graph`, and `output_tail_every_peak >= output_tail`, from
> which the output is below `P·10^(−144/20)` for `P ≥ P*_graph` and exactly zero for
> `P < P*_graph`. Tail reporting uses `output_tail_every_peak`, the only value that holds for every
> input: `P*_graph` can sit near or above full scale for a live graph with a submix (estimated up
> to about +23 dBFS with every gain at its maximum), so a reported `output_tail` would certify
> nothing for real signals there. A host may report the decay value
> beside it where a consumer needs it, named as the decay value with its floor, never in its place.

and replaces "(c) #1261 and #1262 then report the bounded tail, never `Infinite`." with:

> (c) #1261 and #1262 then report `output_tail_every_peak`, through host-core's prepared report and
> the C ABI's `tail_samples` (no field changes); it is finite for every graph once *Retire the
> Infinite tail* (#1378) lands.

### Slice issues (filed 2026-10-06 by root order; each body is its local spec)

| slice | issue | title | spec |
|---|---|---|---|
| A1 | #1464 | Carry every node's tail bound in one node-neutral struct | `.github/ISSUE_SPECS/1464-carry-every-node-s-tail-bound-in-one-node-neutral-struct.md` |
| A2 | #1465 | State a fixed input section's decay, gains and flush stall | `.github/ISSUE_SPECS/1465-state-a-fixed-input-section-s-decay-gains-and-flush-stall.md` |
| B1 | #1466 | Certify the live input section's decay, peak gain and flush stall | `.github/ISSUE_SPECS/1466-certify-the-live-input-section-s-decay-peak-gain-and-flush-stall.md` |
| B2 | #1467 | Derive the live input section's tail gain and state its composition | `.github/ISSUE_SPECS/1467-derive-the-live-input-section-s-tail-gain-and-state-its-composition.md` |
| C | #1379 | Define how node tails compose through gain in the graph extent | this spec |
| G_p | #1468 | Tighten the fixed input section's peak gain past the cascade triangle inequality | `.github/ISSUE_SPECS/1468-tighten-the-fixed-input-section-s-peak-gain-past-the-cascade-triangle-inequality.md` |
