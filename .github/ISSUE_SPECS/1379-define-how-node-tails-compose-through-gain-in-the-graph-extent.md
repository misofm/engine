# Define how node tails compose through gain in the graph extent

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan's reported output tail (`output_tail`, the C ABI `tail_samples`) is a certified bound on the
whole graph: from that sample on, the output stays below -144 dB relative to the peak of the
graph's input. Today the graph adds node tails along a path and ignores gain between nodes, so a
+24 dB fader after an input filter can leave a residual near -120 dB past
the reported tail.

## Context

- **The composition.** `timing` in `crates/graph-compiler/src/pdc.rs` computes each node's extent:
  the maximum over incoming edges of the source extent shifted by its compensation delay
  (`:105-120`), then shifted by the node's latency, plus the node's own tail (`:121-131`), capped
  by `maximum_finite_tail_samples` (`:132-136`). `output_tail` is the output node's extent
  (`:150-159`), reported as `CompiledGraph::output_tail` (`crates/graph-compiler/src/lib.rs:122`).
  Helpers `shifted_tail` and `max_tail` (`pdc.rs:162-176`).
- **Node data.** `GraphNode { id, latency, tail }` (`crates/graph/src/lib.rs:313-318`). Effects
  supply `PreparedEffectMetadata::tail`; builtins supply their tail through `graph-compiler`
  (`crates/graph-compiler/src/compile.rs:141-150`); other nodes are `Finite(0)`.
- **The node contract** (#1329 D1) is relative to the node's own input peak at one floor, -144 dB.
  That is not enough to compose: a residual at -144 dB re a node's input, amplified by a later gain
  `G`, is at `-144 + G` dB at the output. To compose, a node must say how its tail continues below
  -144 dB, and each node must bound its gain.
- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377) gives effects one
  function where these values live.

## Decisions frozen for this slice

- **D1. Two more node quantities**, both certified upper bounds over the node's parameter domain at
  its rate, documented in `effect-contract` beside `TailSamples`:
  - `PeakGain(u32)`, in millibels, rounded up, bounding two things: (a) the peak gain
    `sup |y| / P` over every input of peak `P`, and (b) the *incremental gain after silence*: for
    any control history and any two inputs that agree before `N`,
    `sup_{n >= N} |y[n] - y'[n]| <= G * sup_{n >= N} |x[n] - x'[n]|`. (b) is what composition
    needs, because a downstream node sees an upstream residual on top of its own state. For a
    linear node both are the ℓ1 norm of its response. For a nonlinear node (b) is a Lipschitz bound:
    a time-varying gain `y = g[n] * x` gives `sup |g|` (the dynamics effects after silence); a
    memoryless curve gives its derivative bound times the surrounding filters' ℓ1 norms (soft
    clip). A summing node's bound is the sum over its inputs.
  - `TailDecay(u64)`: samples per further 20 dB. For `n >= N + latency + T + k * D`, the output is
    below `P * 10^((-144 - 20k)/20)`. A node at exact rest beyond `T` (gain-only nodes) reports `0`.
- **D2. Carriers.** Effects return them from `tail_and_rest` (#1377 D1, extended to a struct
  `TailContract { tail, decay, gain, rest }`). Builtins compute them with #1329's tail (input
  section: decay from the certified rate `rho_f`; trim and fader up to +24 dB,
  `crates/builtins/src/lib.rs:474-479`, `:535-540`; the 2x2 matrix/pan at most its row ℓ1 norm).
  Graph-internal nodes (sums, compensation delays, routes) state them in `graph-compiler`. A route's
  gain is `gain_db` times its matrix's row ℓ1 norm, finite only because *Bound route gain and matrix
  values* (#1237) bounds `gain_db` to `[-144, 24]` dB and each coefficient to `[-1, 1]`; without it
  a route's `PeakGain` has no bound.
- **D2a. Strip delay lines.** A strip's `builtins.*.delay_samples` (#210 phase 2) is a pure delay
  that PDC deliberately does not count as latency (`crates/graph-compiler/src/pdc.rs:5-23`), lowered
  as `PreparedTrackDelay` on the strip's `Input` node (`crates/graph-compiler/src/compile.rs:505-514`).
  After the input stops, the delayed strip keeps playing for `delay_samples`, so the extent must
  count it. The strip's `Input` node states `T = max(left, right) delay_samples`, `TailDecay(0)`
  (exact zero after it), `PeakGain` 0 dB and `RestSamples` = that same value for both peaks (the
  ring holds only input samples). Today `output_tail` omits it entirely.
- **D3. The rule.** Along each path from a graph input to the output, the extent is computed so that
  every node's residual is below the -144 dB floor at the output relative to the graph input's
  peak `P`. Node `i`'s input peak is at most `P` times the gains *before* it, and its residual is
  amplified by the gains *after* it, so both count. Concretely, node `i` contributes
  `T_i + ceil(A_i / 20 dB) * D_i`, where `A_i` is the dB sum of the `PeakGain` of every *other*
  node on the path (upstream and downstream) plus `20 log10(number of nodes on the path)`. The
  extent is the maximum over paths of the sum of contributions plus latency shifts, as today. The implementer writes the proof in
  `docs/derivations/1379-graph-tail-composition.md` and keeps this formula unless the proof needs
  a tighter term; any change is recorded in the note and the spec.
- **D4. `Infinite` still wins**, and a node with `TailDecay` unstated (an effect still `Infinite`)
  keeps the path `Infinite`, until *Retire the Infinite tail* (#1378).
- **D5. Control thread only.** Pure arithmetic on `u64` and `u32` at compile time; nothing on render.

## Deliverables

1. `PeakGain`, `TailDecay`, the `TailContract` struct; the builtins' values; the graph-internal
   nodes' values.
2. The rule in `pdc.rs`, with overflow diagnostics as today.
3. The derivation note; `docs/EFFECT_CONTRACT_V1.md` and `docs/BUILTINS_AND_METERING_V1.md` updated.
4. Canonical plan text carries the two quantities; re-pin each moved canonical digest one by one.

## Authorized paths

- `crates/effect-contract/src/lib.rs` (tail types), `crates/builtins/src/tail.rs`,
  `crates/builtins/src/lib.rs` (fader, matrix gain bounds)
- `crates/graph/src/lib.rs` (`GraphNode`), `crates/graph-compiler/src/pdc.rs` (the rule, and the
  module doc's track-delay paragraph `:5-23`, which gains the tail term),
  `crates/graph-compiler/src/compile.rs` (node values, D2a), `crates/graph-compiler/src/canonical.rs`,
  `crates/graph-compiler/src/lib.rs` (unit tests whose extent moves),
  `crates/graph-compiler/tests/` (new `tail_composition.rs`, re-pinned digests), `fixtures/graph/v1/`
- `crates/host-core/tests/live_lanes.rs` and `crates/capi/src/runtime/tests.rs`, only where an
  asserted finite `output_tail` value moves, each with its reason
- The `tail_and_rest` functions of the eight effect crates (returning today's values, gain from their
  parameter domains)
- `docs/EFFECT_CONTRACT_V1.md`, `docs/BUILTINS_AND_METERING_V1.md`,
  `docs/derivations/1379-graph-tail-composition.md` (new), this spec

## Non-goals

- New per-effect tails (#1372-#1376). Removing `Infinite` (#1378). Any C ABI field.

## Objective gates

1. **Gain after a tail** (`crates/graph-compiler/tests/tail_composition.rs`): a session with one
   track, input HPF/LPF at the worst-case pair of #1329 (`input_section_worst_case_pair(48_000)`)
   and fader +24 dB, compiled at 48 kHz. Input of peak `P = 1`: the worst-sign sequence
   `sign(h[T + i])` reversed over the last 1,000,000 samples before `N` (`h` the section cascade's
   impulse response; an impulse excites the residual far below its bound and cannot catch an
   under-reported tail), then zeros, rendered through the real host-core runtime. Every output
   sample from `N + latency + output_tail` on is below `10^(-144/20)`. Red today (residual about
   -120 dB at the reported tail).
2. **Gain before a tail**: a track at unity routed with route `gain_db = +24` into a submix whose
   input HPF/LPF is at the same worst-case pair, same worst-sign input and check. Red under a rule
   that counts only downstream gain (the +24 dB sits upstream of the tail node).
3. **Recompute**: for the sessions of gates 1 and 2, `output_tail` equals D3's formula evaluated by
   hand in the test from the nodes' stated values (never a digest).
4. **Summing**: two tracks into one bus at unity: the extent includes the `+6 dB` of two terms.
5. **Strip delay**: the gate 1 session with `delay_samples = 1_000` on both lanes reports an
   `output_tail` exactly 1,000 samples longer than without it (D2a: the delay is a term of the
   existing `Input` node with `PeakGain` 0 dB, so no node count and no `A_i` changes), and an
   impulse at `N - 1` is still non-zero at the output at `N + 999`.
4. Commands: the `test-debug-a` workspace command from `.github/workflows/qualification.yml`;
   `bash scripts/check-graph-determinism.sh`; `bash scripts/check-effect-contract.sh`;
   `cargo build --locked --release -p audit && ./target/release/audit capi`;
   `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

## Test value

- Gate 1: a composition that ignores downstream gain (today's rule) reports a tail the rendered
  worst-case output exceeds; no test renders past a reported plan tail.
- Gate 2: a rule that counts only gain after the tail node (the first draft of D3) under-reports
  when the gain sits upstream.
- Gate 3: a rule that drops a path or a decay term disagrees with the hand evaluation.
- Gate 4: a rule that ignores fan-in under-reports by the summed terms.
- Gate 5: an extent that omits the strip's delay line (today's) is short by the delay.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329)
- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Bound route gain and matrix values* (#1237): a route's `PeakGain` is finite only under its bounds
