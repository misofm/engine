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
  - `PeakGain(u32)`: a bound on `sup |y| / P` over every input of peak `P` (ℓ1 norm of the response
    for a linear node), in millibels, rounded up. A summing node's bound is the sum over its inputs.
  - `TailDecay(u64)`: samples per further 20 dB. For `n >= N + latency + T + k * D`, the output is
    below `P * 10^((-144 - 20k)/20)`. A node at exact rest beyond `T` (gain-only nodes) reports `0`.
- **D2. Carriers.** Effects return them from `tail_and_rest` (#1377 D1, extended to a struct
  `TailContract { tail, decay, gain, rest }`). Builtins compute them with #1329's tail (input
  section: decay from the certified rate `rho_f`; fader up to +24 dB; matrix and route gains from
  their bounded domains). Graph-internal nodes (sums, compensation delays, routes) state them in
  `graph-compiler`.
- **D3. The rule.** Along each path from a graph input to the output, the extent is computed so that
  every upstream residual, amplified by the gains after it and by the number of summed terms, is
  below the -144 dB floor at the output. Concretely, node `i` contributes
  `T_i + ceil(A_i / 20 dB) * D_i`, where `A_i` is the dB sum of the gains downstream of `i` on the
  path plus `20 log10(number of nodes on the path)`. The extent is the maximum over paths of the sum
  of contributions plus latency shifts, as today. The implementer writes the proof in
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
- `crates/graph/src/lib.rs` (`GraphNode`), `crates/graph-compiler/src/pdc.rs`,
  `crates/graph-compiler/src/compile.rs`, `crates/graph-compiler/src/canonical.rs`,
  `crates/graph-compiler/tests/` (new `tail_composition.rs`, re-pinned digests), `fixtures/graph/v1/`
- The `tail_and_rest` functions of the eight effect crates (returning today's values, gain from their
  parameter domains)
- `docs/EFFECT_CONTRACT_V1.md`, `docs/BUILTINS_AND_METERING_V1.md`,
  `docs/derivations/1379-graph-tail-composition.md` (new), this spec

## Non-goals

- New per-effect tails (#1372-#1376). Removing `Infinite` (#1378). Any C ABI field.

## Objective gates

1. **Gain after a tail** (`crates/graph-compiler/tests/tail_composition.rs`): a session with one track,
   input HPF at the worst-case pair of #1329 and fader +24 dB, compiled at 48 kHz. Render an impulse
   then zeros through the real host-core runtime; every output sample from
   `latency + output_tail` on is below `10^(-144/20)`. Red today (residual about -120 dB at the
   reported tail).
2. **Recompute**: for the same session, `output_tail` equals D3's formula evaluated by hand in the test
   from the nodes' stated values (never a digest).
3. **Summing**: two tracks into one bus at unity: the extent includes the `+6 dB` of two terms.
4. Commands: the `test-debug-a` workspace command from `.github/workflows/qualification.yml`;
   `bash scripts/check-graph-determinism.sh`; `bash scripts/check-effect-contract.sh`;
   `cargo build --locked --release -p audit && ./target/release/audit capi`;
   `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

## Test value

- Gate 1: a composition that ignores downstream gain (today's rule) reports a tail the rendered
  output exceeds; no test renders past a reported plan tail.
- Gate 2: a rule that drops a path or a decay term disagrees with the hand evaluation.
- Gate 3: a rule that ignores fan-in under-reports by the summed terms.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329)
- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
