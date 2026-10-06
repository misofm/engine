# Define how node tails compose through gain in the graph extent

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Rewritten 2026-10-06 by Amendment 1 (root rulings, below). This issue is slice C of Amendment 1's
split: slices A1 (#1464), A2 (#1465), B1 (#1466) and B2 (#1467) state the node values it composes,
and #1468 tightens the fixed input section's peak gain. Slice C is about one working day, an
explicit exception to the half-day rule (Hazards; Amendment 1, H8).

## Product outcome

A plan's reported output tail is a certified bound on the whole graph, through every gain between
its nodes, for every input. The plan reports two values and a floor. For every graph input peak `P`
at or above the plan's flush floor `P*_graph`, the output stays below `P * 10^(-144/20)` from
`N + output_tail` on. For every peak, from `N + output_tail_every_peak` on, it is below that (at or
above the floor) or exactly zero (below it). Hosts report `output_tail_every_peak` (host-core's
report and the C ABI's `tail_samples`): `output_tail` certifies nothing for real signals whenever
`P*_graph` sits near or above full scale, which a live graph with a submix can reach (estimated up to
about +23 dBFS with every gain at its maximum). Today the
graph adds node tails along a path and ignores gain. With the top input-filter pair, a +24 dB fader
leaves the worst-case output 4.7 dB above the floor at today's reported tail, and with the matrix and
a +24 dB route at their maxima, about 41 dB above (`f64` probe, Amendment 1, H9). Today the extent
also omits a strip's delay line (D2a). This issue is slice C of Amendment 1's split; slices A1
(#1464), A2 (#1465), B1 (#1466) and B2 (#1467) state the node values it composes.

## Context

Code anchors verified on `main` at `7e8379523` and on `codex/d15-stream-g2` at `f3956e63c` (#1461);
after #1454 the lane anchors shift (Amendment 1, H4). Re-verify every anchor at start.

- **The composition.** `timings` in `crates/graph-compiler/src/pdc.rs` (`:38`) computes each node's
  extent: the maximum over incoming edges (sidechain edges included) of the source's extent plus
  its compensation delay (`:105-120`), shifted by the node's latency (`:121`), plus the node's tail
  (`:122-131`), checked against `maximum_finite_tail_samples` (`:132-136`). `output_tail` is the
  output node's extent (`:150-153`), reported as `GraphCompileReport::output_tail`
  (`crates/graph-compiler/src/lib.rs:184`); it includes every latency on the path. Helpers
  `shifted_tail` and `max_tail` (`pdc.rs:162-176`).
- **Node data.** `GraphNode { id, latency, tail }` (`crates/graph/src/lib.rs:313-317`), charged in
  `graph_metadata_bytes` as `size_of::<GraphNode>()` per node
  (`crates/graph-compiler/src/estimate.rs:368`), printed by `canonical.rs` in the `node` and `tail`
  rows. Effects supply `metadata.tail` (`crates/graph-compiler/src/compile.rs:254-261`); the builtins
  supply each strip's `T_decay` through `PreparedBuiltinsSession::tails` (`compile.rs:140`,
  `:216-230`; `crates/builtins-compiler/src/lib.rs:2158`); every other node is `Finite(0)`. A strip's
  delay line is `PreparedTrackDelay` on its `Input` node (`compile.rs:493-503`).
- **The node values** come from slices A1, A2, B1 and B2 (H1-H3): every node's `NodeTailBound` carries
  `composition: CompositionBound`; fixed input sections and live input sections state it; effects
  state `Unstated`.
- **Gain domains.** Trim and fader `[-144, 24]` dB (`crates/builtins/src/lib.rs`, `trim_db`,
  `fader_db`), VCA offsets clamped to the same domain (`crates/session/src/vca.rs:23-34`), matrix and
  pan coefficients in `[-1, 1]`, route gain `[-144, 24]` dB and coefficients `[-1, 1]` (#1237,
  `session::ROUTE_GAIN_DB_MAXIMUM`, `session::ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM`, read by
  `graph_compiler::route_values`, `ids.rs:322`). Every ramp stays inside its endpoints (#1408; the
  route ramp, `crates/graph/src/runtime.rs:854-927`).
- **Which gain lanes are live.** `HostLiveLanes` (`crates/host-core/src/prepare.rs:371-397`): with a
  control queue, every strip's fader/mute and matrix/pan lanes are attached; `strip_input` adds the
  input lane (`:1268`); `routes` adds one lane per route into a submix, attached after the graph
  compile (`:1580-1597`). Routes into the output have no lane.
- **Reports.** host-core copies `output_tail` into its prepared report (`prepare.rs:1599`, field
  `:226`), which the C ABI reports as `tail_kind` and `tail_samples`
  (`crates/capi/src/runtime/compile.rs:648`; doc comments `crates/capi/src/abi.rs:36-38`, `:262`).
  The browser host reports no tail today. `docs/C_ABI_V1_QUALIFICATION.md:253` states that attaching
  fader and matrix lanes does not change `tail_samples`, and `crates/host-core/tests/live_lanes.rs`
  asserts it (`:244`, whose doc `:196-199` says it stands for "no input lane was attached") and pins
  the `HostLiveLanes::ALL` tail (`:274`). Hosts set `maximum_finite_tail_samples` to `u64::MAX`;
  `graph_fixture`, the audit tools and `console-workload` set 10,000,000.

## Decisions frozen for this slice

Amendment 1's H1-H4 and H10 are the decisions; this section states them as slice C implements them.

- **D1. The node contract.** Amendment 1, H1, stated by slices A1, A2, B1 and B2. Slice C reads it.
- **D2. The graph carriers.** Amendment 1, H2: `GraphNode` carries the node's `NodeTailBound`;
  `GraphCompileReport` gains `output_tail_every_peak` and `output_flush_floor`; the canonical `tail`
  and `extent` rows.
- **D2a. Strip delay lines** (unchanged in substance). A strip's `builtins.*.delay_samples` (#210
  phase 2) is a pure delay that PDC deliberately does not count as latency
  (`crates/graph-compiler/src/pdc.rs:5-23`), lowered as `PreparedTrackDelay` on the strip's `Input`
  node. After the input stops the delayed strip keeps playing for `delay_samples`, so the extent
  counts it: the `Input` node states `T = max(left, right) delay_samples`, `D = 0` (exact zero after
  it), `G_p = G_t = 0` mB, `sigma` `Zero`, and `RestSamples` with that same value for both peaks (the
  ring holds only input samples). Today `output_tail` omits it.
- **D2b. Gain-only node values.** Amendment 1, H3: prepared values for fixed gain lanes, domain
  maxima for live ones, `Zero` for a fixed mute where the kernel gives exact zero. The graph compile
  receives the selection from its caller as an explicit input (no default); host-core passes it from
  `HostLiveLanes` and the control queue; tools pass their own (the audit tools, `graph_fixture` and
  `console-workload` prepare without a control queue, so every gain is fixed there).
- **D3. The rule.** Amendment 1, H4: the two passes, `k(v)`, the decay extent, `P*_graph` as the
  sum of effective stalls carried by downstream tail gain, the rest extent at `P*_graph`, the range
  hypothesis, and the inequality chain. The implementer writes the proof in
  `docs/derivations/1379-graph-tail-composition.md` and keeps the rule unless the proof needs a
  tighter term; any change is recorded in the note and in this spec.
- **D4. `Infinite` wins.** A node with `tail` `Infinite` or `composition` `Unstated` from which the
  output is reachable makes `output_tail` `Infinite` (H4 step 8); a node whose `rest` is `Unstated`
  makes `output_tail_every_peak` `Infinite` unless `P*_graph` is `Zero`; both until *Retire the
  Infinite tail* (#1378).
- **D5. What hosts report.** host-core's prepared report carries `output_tail_every_peak` (its field
  is renamed so; the C ABI's `tail_samples` reads it; no ABI field changes). The cap
  `maximum_finite_tail_samples` refuses on `output_tail_every_peak` (Q7). Slice C applies H10 to
  decision 15.
- **D6. Control thread only.** `i64` millibel arithmetic plus one `f64` stall sum at compile time,
  with `math::log`/`math::exp`. Nothing runs or allocates on render.

## Deliverables

1. `GraphNode`'s bound; the gain-liveness compile input and its callers; the graph-internal rows of
   H3 and D2a in `graph-compiler`; H4 in `pdc.rs` with the overflow and cap diagnostics;
   `GraphCompileReport`'s two fields; the canonical rows.
2. host-core's report field renamed to `output_tail_every_peak` and filled from the graph's value;
   the C ABI's read of it; `crates/capi/src/abi.rs`'s two doc comments; `docs/C_ABI_V1_QUALIFICATION.md`
   (`:253` and the tail's meaning).
3. The derivation note's graph part; `docs/EFFECT_CONTRACT_V1.md` and
   `docs/BUILTINS_AND_METERING_V1.md` updated (the graph extent's meaning, the floor, the two values,
   which one hosts report); H10 applied to decision 15.
4. Gates C1-C13; each moved canonical digest and resource count re-pinned one at a time with its
   reason; the `test-release` step for host-core's `tail_composition`.

## Authorized paths

- `crates/graph/src/lib.rs` (`GraphNode`'s field and the test literals that build one; named
  exception)
- `crates/graph-compiler/src/pdc.rs` (the rule; the module doc's track-delay paragraph `:5-23` gains
  the tail term), `compile.rs` (node values, D2a, D2b), `canonical.rs` (the `tail` and `extent`
  rows), `lib.rs` (`GraphCompileReport`'s fields, the compile input; unit tests whose extent or
  estimate moves), `ids.rs` (only a route-gain helper, if one is needed; J #1415's constants are not
  touched)
- `crates/graph-compiler/tests/tail_composition.rs` (new), `crates/graph-compiler/tests/track_delay.rs`
  (the digest), `crates/graph-compiler/tests/MUTATIONS.md` (rows that name tails), and other
  `graph-compiler` tests only where an asserted extent or estimate moves, each with its reason
- `crates/graph-compiler/src/bin/graph_fixture.rs` (the compile input; the report's fields if the
  fixture prints them), `fixtures/graph/v1/direct-route.*` (regenerated), `fixtures/graph/MANIFEST.tsv`
- `crates/host-core/src/prepare.rs` (the report field `:226`, the copy `:1599`, the selection passed
  to the graph compile; named exception, hot-file slot)
- `crates/capi/src/runtime/compile.rs` (the one read at `:648`), `crates/capi/src/abi.rs` (the doc
  comments at `:36-38` and `:262` only), `crates/capi/src/runtime/tests.rs` (the reads of the renamed
  field); stream B's files, named exception
- `crates/capi/tests/tail_every_peak.rs` (new, gate C12); stream F's `crates/capi` tests, named
  exception
- Every `GraphBuiltinsCompileRequest {` literal (the new field only; about 48 in 21 files, re-grep at
  start and list each file in the attempt record), including the `crates/graph-compiler/tests/*`
  files whose asserted values do not move (`bank_levels`, `bypass_cohorts`, `bypass_resources`,
  `compile_shapes`, `live_routes`, `route_activity`, `route_coefficients`, `scale`, `vca_follow`),
  `crates/host-core/tests/{live_routes,route_mute,submix_strip}.rs` and
  `tools/audit/src/fixture_builtins.rs`; named exception
- `crates/host-core/tests/tail_composition.rs` (new, release scale), `crates/host-core/tests/live_lanes.rs`
  (`:244` and `:274`, restated by C8), and other `crates/host-core/tests/*`, `crates/capi/tests/*`
  and `hosts/host-web/src/tests.rs` only where the renamed field or an asserted tail or graph resource
  count moves, each with its reason
- `.github/workflows/qualification.yml` (one `test-release` step for host-core's `tail_composition`),
  `scripts/check-ci-path-routing.py` (only if that step needs it)
- `tools/audit/src/graph.rs`, `tools/audit/src/builtins_graph.rs`, `tools/console-workload` (the
  compile input; fixtures only where a recorded graph identity or byte count moves)
- `docs/EFFECT_CONTRACT_V1.md`, `docs/BUILTINS_AND_METERING_V1.md`, `docs/C_ABI_V1_QUALIFICATION.md`,
  `docs/derivations/1379-graph-tail-composition.md` (the graph part),
  `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md` (H10's text only),
  this spec, its `STREAMS.md` row

## Non-goals

- Any node's values: slices A1, A2, B1 and B2 state the builtins'; #1372-#1376 state the effects'.
  Removing `Infinite` and the `Unstated` variants (#1378).
- Any C ABI field or browser wire change: `tail_samples` keeps its field and changes its source.
- A graph-level `RestSamples`, and any render-side consumer (#1107 reads node `RestSamples`).
- Sidechain magnitude composition and feedback.
- An absolute output floor in any report (#1329 Amendment 3, G2).

## Hazards

- **Every canonical digest moves** (the `tail` rows' new fields, the `extent` row, and the estimate
  row through `size_of::<GraphNode>()`), and with it every pinned graph identity:
  `ZERO_DELAY_CANONICAL_SHA256`, `fixtures/graph/v1/direct-route.*`, the joined-corpus manifest
  identity, `graph_metadata_bytes` and the totals that include it in every resource report and
  fixture. Re-pin one at a time; the only expected changes are those three; any other moved byte
  stops the slice.
- **Reported tails get longer, and some become `Infinite`.** Live gain lanes state domain maxima
  (+24 dB fader, +6 dB matrix row, +30 dB route into a submix), so each input section on a live path
  gains up to three or four decades (about 150k-200k samples at the top of the cutoff domain). Every
  graph with an effect reports `Infinite` (D4), including graphs with the gate, transient shaper or
  soft clip, which are finite today. Hosts now report `output_tail_every_peak`, which for a live
  graph with a submix takes `any_sanitized_input` rests (about 2.39M samples per live section at
  44.1 kHz) where `P*_graph` puts `X*` above +24 dBFS. Estimate for `live_lanes.rs`'s
  `HostLiveLanes::ALL` fixture: about 4.8M samples, against today's `2 T_decay`, 1,408,020 at
  44.1 kHz; the derived live `G_t` (slice B2) can lower it. The 10,000,000 cap of the audit tools,
  `graph_fixture` and `console-workload` can refuse a plan it accepted (it now checks the every-peak
  value); gate C8 records the headroom.
- **`P*_graph` can sit near or above full scale.** A stall is absolute: about `2e-19` for the top
  fixed pair, about `1.5e-16` for a 10 Hz HPF into the top LPF (`P*` `4.9e-9`), about `4e-15` for the
  live bound (`P*` about `1.2e-7`). Carried by about +120 to +130 dB of downstream gain (a fixed track
  into a fixed submix with every gain at its maximum), `P*_graph` lands between about -90 and
  -30 dBFS; through a live submix it can sit near or above full scale (estimated up to about
  +23 dBFS with every gain at its maximum, including the route into the output; about -1 dBFS with a
  0 dB output route). These are estimates; C8 records the computed values. A graph of about 102
  nested submix levels at domain maxima overflows the `f64` stall sum; it reports the floor
  `AboveRange` and the rest branch (H4 step 6, gate C13), not a refusal.
- **The C ABI's `tail_samples` changes meaning and value** for every plan (H10). The C ABI
  qualification text and its tests move with it; a plan whose faders are fixed and below their
  maxima reports a shorter tail without a control queue than with one.
- **Order.** C needs A1, A2, B1 and B2, and its `graph-compiler` turn; its `prepare.rs` and
  `graph/src/lib.rs` edits take the slots in H8.
- **Size: about one working day, an explicit exception to the half-day rule** (root ruling m7,
  2026-10-06). The carriers, the rule and the report are about half a day; the real-kernel release
  gates (C1, C2, C6, C11, C12) and the re-pins are the other half. They are not split: the gates are
  this issue's product evidence (ruling (d) requires C6), and host-core copies the graph's value
  into the C ABI's `tail_samples`, so a first half without them would move the reported tail on
  recomputation evidence alone, and neither half would be independently verifiable. If the slice
  runs past about one and a half working days, the coordinator stops and reports to root rather
  than cutting a gate.

## Objective gates

Each gate names the defect it turns red on in "Test value". Release-scale gates live in
`crates/host-core/tests/tail_composition.rs` (ignored in debug, run by the new `test-release` step);
recomputation gates live in `crates/graph-compiler/tests/tail_composition.rs`.

- **C1. Gain after a tail** (host-core, release). One track at 48 kHz: input HPF/LPF at
  `input_section_worst_case_pair(48_000)`, trim 0 dB, fader +24 dB, all four track matrix
  coefficients 1, one route to the output at +24 dB with all four coefficients 1, a source with
  identical channels (so every row gain of 2 is reached). Input of peak `P = 1`:
  `x[N - 1 - i] = sign(h[tau + 1 + i])` for `i < 1,000,000`, `h` the designed cascade's `f64` impulse
  response and `tau` the compiled `output_tail`, then zeros, rendered through the real host-core
  runtime. Assert `P >= P*_graph`, and every output sample of both channels from `N + output_tail` on
  is below `P eps`. Predicted (H9): today's rule +40.6 dB over the floor (red); H4's rule
  (`A = 6007` mB, `k = 4`) about 45 dB under (green).
- **C2. Gain before a tail** (host-core, release). A track with both filters disabled, trim +24 dB,
  fader +24 dB, matrix ones and a route at +24 dB with ones into a submix whose input section is the
  worst-case pair at 0 dB trim, then the submix's fader +24 dB, matrix ones and a route at +24 dB with
  ones to the output; worst-sign input for the submix's cascade, the same check. Predicted: a rule
  that counts only gain after the tail node (`k = 4`) +39 dB over the floor (red); H4's rule
  (`A = 14415` mB, `k = 8`) about 53 dB under (green).
- **C3. Recompute** (graph-compiler). For the sessions of C1, C2, C4, C5, C6, C7 and C11 (both
  selections), and one session whose track and submix both carry a live input lane (a live track
  into a live submix, so `G_p` and `G_t` differ on the path), `output_tail`, `output_tail_every_peak`
  and `P*_graph` equal H4's rule evaluated in the test by explicit enumeration of every
  source-to-output signal path, from the plan's per-node values, with no code shared with `pdc.rs`.
  Never a digest. If `graph-compiler`'s tests cannot prepare a live input lane, the live session's
  enumeration runs in host-core's `tail_composition.rs` instead.
- **C4. Fan-in** (graph-compiler). Two tracks, each with the worst-case pair, every fader, matrix and
  route at its domain maximum, into one submix with both filters disabled and trim +15 dB, so the
  submix sum's +6 dB moves each track section's `A` across a decade boundary (`m = 1`: 13,515 mB,
  `k = 7`; `m = 2`: 14,118 mB, `k = 8`; the implementer confirms the numbers on the compiled values):
  the two-track `output_tail` exceeds the one-track value by exactly one `D`.
- **C5. Strip delay.** C1's session with `delay_samples = 1_000` on both lanes reports
  `output_tail` and `output_tail_every_peak` exactly 1,000 samples longer and the same `P*_graph`
  (graph-compiler); an impulse at `N - 1` is still non-zero at the output at `N + 999` (host-core).
- **C6. Below the floor** (ruling (d); host-core, release, every launch rate). Per rate, the first
  design of Amendment 1 H5's list that meets (ii) (first: the 1 kHz input LPF, HPF disabled, trim
  0 dB, every downstream gain at 0 dB; then the same LPF in C1's gain shape; then a design that holds
  a sub-`REST_EPS` state until the joint flush arms). DC, worst-sign, random and alternating inputs
  of peak `P = P*_graph / 2` for 100,000 frames, then zeros. Assert: (i) for every pattern, every
  output sample from `N + output_tail_every_peak` on is exactly `+-0.0`; (ii) for DC or the
  worst-sign pattern, the last non-zero output sample falls at or after `N + output_tail`;
  (iii) `output_tail_every_peak` is C3's `max(output_tail, rest extent)`. Record, per rate, the
  design used and the last non-zero sample per pattern. If no listed design meets (ii) at a rate, the
  slice stops and reports to root; the gate is not weakened.
- **C7. `Infinite` wins** (graph-compiler). C1's session with one insert built from a test-only
  effect descriptor (graph-compiler's test factories; `tail` `Finite(0)`, `composition` `Unstated`)
  reports `Infinite` for both values and no `P*_graph`; C1 without it reports finite values.
- **C8. Live and headroom.** `live_lanes.rs:274` asserts the `HostLiveLanes::ALL` fixture's reported
  value (now `output_tail_every_peak`) equals C3's enumeration for that plan. `live_lanes.rs:244` is
  restated: the `FADER_AND_MATRIX` plan's reported value equals C3's enumeration with fader and matrix
  at their domain maxima and every input section at its prepared design (no input lane). Record
  `output_tail`, `output_tail_every_peak` and `P*_graph` per launch rate for that fixture and for C1,
  C2 and C6, and the headroom of each to 10,000,000.
- **C9. Re-pins, one at a time:** `fixtures/graph/v1/direct-route.*` (regenerated by
  `graph_fixture`), `ZERO_DELAY_CANONICAL_SHA256`, every resource count that includes
  `graph_metadata_bytes`, and the manifest identities. The only expected changes are the `tail`
  rows' new fields, the new `extent` row and `size_of::<GraphNode>()` times the node count; any other
  moved byte stops the slice.
- **C10. No rendered bit moves** (`audit capi`'s `pcm_digest`, wasm G5, builtins PCM fixtures).
- **C11. A live gain lane states its domain maximum** (host-core, release, 48 kHz). C1's session
  with the fader prepared at -24 dB. (a) Without a control queue the plan states the prepared fader
  gain, and C3 confirms the fixed selection's values. (b) With a control queue, the fader is moved
  live to +24 dB, the ramp completes before the worst-sign input starts, and every output sample from
  `N + output_tail` on is below `P eps`. Predicted: a rule that states the prepared -24 dB for the live
  lane (`A = 1207` mB, `k = 1`) leaves about +10 to +21 dB over the floor by H9's figures (+40.6 dB at
  `k = 0`, less one decade and `D`'s slack over it; red); H3's rule (`k = 4`) green. The slice records
  the measured excess of the mutant.
- **C12. Hosts report the every-peak value** (`crates/capi/tests/tail_every_peak.rs`, new). For
  C6's session at 48 kHz, prepared through the C ABI, `tail_kind` is finite and `tail_samples`
  equals the graph's `output_tail_every_peak`, and the session is chosen so that it differs from
  `output_tail` (the test asserts that it differs).
- **C13. The cap checks the reported value, and an overflowing floor stays sound**
  (graph-compiler). (a) A plan whose `output_tail` is within `maximum_finite_tail_samples` and whose
  `output_tail_every_peak` exceeds it is refused with `graph.tail.limit`; with the cap at
  `output_tail_every_peak` it prepares. (b) A track with the worst-case pair into a chain of nested
  submixes (filters disabled, every gain at its domain maximum) deep enough that the `f64` stall sum
  is not finite (about 102 levels; the test computes the least depth from the stated gains and uses
  it) reports `output_flush_floor` `AboveRange` and an `output_tail_every_peak` equal to C3's
  enumeration with `R_v = any_sanitized_input` at every stateful node; one level fewer reports a
  finite `Millibels` floor. (c) A unit test of the millibel sum helper: two `i64` operands whose sum
  overflows give `graph.tail.arithmetic_overflow` (a session cannot reach it: `i32` node gains need
  about `2^32` nodes on one path).
- **Commands:** the `test-debug-a` workspace command from `.github/workflows/qualification.yml`;
  `cargo test --locked --release -p host-core --test tail_composition -- --include-ignored` (also the
  new `test-release` step); `cargo test --locked -p capi --test tail_every_peak`; `python3 scripts/check-ci-path-routing.py`;
  `bash scripts/check-graph-determinism.sh`; `bash scripts/check-effect-contract.sh`;
  `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`;
  `cargo build --locked --release -p audit && ./target/release/audit capi`;
  `bash scripts/check-builtins-fixtures.sh . target/release/audit`;
  `bash scripts/check-workspace-policy.sh`;
  `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

The slice's record carries a mutation table (each defect applied, the named test run, the file
restored), as #1329's attempts did, with at least the mutants named in "Test value"; each mutant is
red.

## Test value

- C1: a composition that ignores gain between nodes (today's) reports a tail that the worst-case
  rendered output exceeds by about 40 dB; no test renders past a plan's reported tail.
- C2: a rule that counts only gain after the tail node under-reports by about 39 dB when the gain
  sits upstream.
- C3: a rule that drops a path, a decay term, the `n_p` term, the clamp `k >= 0`, the stall's cross
  term or the downstream gain on a stall, uses peak gains downstream or tail gains upstream (visible
  because the live session's `G_t` is below its `G_p`), rounds a millibel down, or takes the maximum
  of the stalls instead of their sum disagrees with the enumeration; the real kernel's slack (about
  45 dB in C1) cannot see any of them.
- C4: a sum stated at 0 dB (fan-in ignored) loses exactly one decade of the two-track extent.
- C5: an extent that omits the strip's delay line (today's) is short by the delay.
- C6: an every-peak value that drops the rest branch (reports `output_tail` for every peak) is
  beaten by the real kernel below the floor; no other test drives a graph below its floor.
- C7: a rule that composes an `Unstated` node as a gain-free, decay-free node reports a finite
  extent with no bound behind it.
- C8: a fader-and-matrix request that attaches the input lane reports the live bound's composition
  instead of the prepared design's; the `ALL` pin catches a selection that drops a live lane's domain
  maximum.
- C11: a rule that states a live lane's prepared value under-reports by about 10 to 21 dB once the
  lane moves; C1 and C2 set every gain at its maximum and cannot see it.
- C12: a host copy that still reads `output_tail` (the decay value) reports a number valid only above
  `P*_graph`; no other test reads the C ABI's tail against the graph's every-peak value.
- C13: a cap that checks only `output_tail` accepts a plan whose reported tail exceeds the cap; a
  stall sum whose `inf` is converted to a millibel value (saturated to `i32::MAX`, or `NaN` cast to
  0) states a finite floor with no bound behind it and loses the rest branch; an unchecked millibel
  sum wraps silently.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329, passed), *Carry each
  effect's tail and exact-rest bound in its prepared metadata* (#1377, passed), *Tighten the cascade
  exact-rest bound with a frequency-aware cascade analysis* (#1433, passed), *Keep only render-read
  effect fields in the render node table* (#1460, passed).
- Slices A1 (*Carry every node's tail bound in one node-neutral struct*, #1464), A2 (*State a fixed
  input section's decay, gains and flush stall*, #1465), B1 (*Certify the live input section's
  decay, peak gain and flush stall*, #1466) and B2 (*Derive the live input section's tail gain and
  state its composition*, #1467): the node values this slice composes.
- *Bound route gain and matrix values* (#1237, landed): a route's `PeakGain` is finite only under
  its bounds. *Retarget a live input filter only through its designs and their mixtures* (#1407) and
  *Keep every trim, fader and matrix ramp inside its endpoints* (#1408): the live words and the gain
  bounds during ramps.
- Its `graph-compiler/src/*` turn (after A #1285, J #1384 and C #1287's first slice) and the H8 slots
  in `graph/src/lib.rs`, `host-core/src/prepare.rs` and the workflows.
- #1375 and #1376 land in the same batch (Q3, third round), after A2, each with its D4 restated to
  H1 (H7).
- Not a dependency: *Tighten the fixed input section's peak gain past the cascade triangle
  inequality* (#1468) may land before or after this slice (H8).

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

**Design note.** The full amendment, with every derivation sketch, the inequality chain, the probe
evidence and the hot-file and exception lists, is kept verbatim in
`docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md` (root ruling, third round:
GitHub's body limit). This body keeps the decisions, the rule, the gates, the paths and the order;
where the two differ, this body governs.

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
- **MINORs and NITs.** All folded (listed in the design note).

### Root rulings applied, second round (2026-10-06, after the confirmation review)

The confirmation review (NOT CONFIRMED: three MAJORs, nine MINORs, nine NITs) found H4 and the new
mathematics sound. Root ruled, with no further review round (full text: the design note):
**MJ1** F2(b)'s second row is the HPF at 10 Hz into the LPF one `f32` above 10 Hz, margins include
`dev_loud` (#1465); **MJ2** F1(b)'s mutant is a certificate rate halfway to 1 (#1465); **MJ3** one
disabled-filter gain rule, `|trim| (1 + u)` or `|trim|` where the product is exact (H3, #1465);
**m1** F2(d) checks `G_p`'s use; **m2** a fixed route states its prepared ungated gain, the source's
mute decides only `Zero` (H3); **m3** C's paths add the `GraphBuiltinsCompileRequest {` literals and
C12's file; **m4** an overflowing stall sum gives the floor `AboveRange`, not a refusal (H4, C13);
**m5** `P*_graph` figures are estimates; **m6** the cascade triangle inequality is sound, no
correctness gap is deferred, tightenings are successors (#1468); **m7** C is about one working day,
an explicit exception (Hazards); **ruling 4** B split into B1 and B2 (H8); **m8** #1376's D4 restated
to H1 (H7); **m9** C6's design list per rate (H5); **NITs 1-9** folded.

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
  #1376 land on `main` in the batch with C, never before it (confirmed by root, 2026-10-06): `main` must never report a
  plan tail that is not certified.
- **Size.** #1379's body must fit GitHub's 65,536-character limit. The full amendment (H1-H10 with
  the derivation sketches, the inequality chain, the evidence and the hot-file and exception lists)
  is kept verbatim in the design note
  `docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md`; this spec keeps the decisions,
  the rule, the gates, the paths and the order, and points to the note.
- **#1468's factor from evidence.** #1468's first step measures `g_meas` and the achievable
  certified bound on the real kernel; its gate factor is then set from that measurement with a
  stated margin, at most 1.5, as #1433 set 1.15.

### H1. The node contract in additive form (ruling (a))

Summary; the reasons for each shape, the share split and the proof of the `D` construction are in
the design note, H1. `eps = 10^(-144/20)`; a node with latency `L`, input `x`, output `y`;
`u = 2^-24`; `N` the first sample of silence. Conditions (C): every control history the node admits
before `N`, and no control event at or after `N` (a filter retarget in flight completes by
`N + 64`, #1407; every trim, fader, matrix, pan and route ramp stays inside its endpoints, #1408 and
`crates/graph/src/runtime.rs:854-927`). A node with a sidechain states every value for every
sidechain input. A stereo node states the maximum over its two channels.

| symbol | name | unit | meaning |
|---|---|---|---|
| `D` | decay | samples per further 20 dB | (N2) |
| `G_p` | peak gain | millibels, rounded up, or `Zero` | (N1) |
| `G_t` | tail gain | millibels, rounded up, or `Zero`; `G_t <= G_p` | gain to an input that arrives at or after `N`, (N2) |
| `sigma` | flush stall | millibels re 1.0, rounded up, or `Zero` | absolute, at the output |

- **(N1) Peak.** If `|x[n]| <= X` for all `n`, under any admitted history: `|y[n]| <= g_p X + sigma`.
- **(N2) Tail at every decade.** Under (C), if `|x[n]| <= X` for all `n` and `|x[n]| <= epsilon`
  for every `n >= M` (some `M >= N`), then for every integer `k >= 0` and every
  `n >= M + L + T + k D`: `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma`. `G_t` bounds the
  part of the output that the input from `M` on produces; for a fixed design `G_t = G_p`.
- **(N3) Rest.** #1329 D2's `RestSamples`, read with "zero from `M`" (`M >= N`).
- `D = 0` means the `X` part is exactly zero from `M + L + T` on (gain-only nodes, sums, delays).
  `sigma` is internal to the composition: no report states an absolute output floor (#1329
  Amendment 3, G2). `G_p` uses the loud-input deviation `dev_loud`, never #1329's `dev_core`.
- **`D`'s construction (binding):** the certified crossings `T(k)` for `k = 1..16` from one extended
  pass, a contraction certificate (`M v <= lambda v`, `lambda < 1`) for `k > 16`, and
  `D = max(max_{k <= 16} ceil((T(k) - T) / k), D_inf + ceil((T_lambda - T) / 17))`, with
  `D_inf = ceil(ln 10 / -ln lambda)` and `T_lambda >= T` (#1465 F-D2).

### H2. The carriers (ruling (c), Q4)

`NodeTailBound { tail, tail_every_peak, rest, composition }` with `CompositionBound { Stated {
decay, peak_gain, tail_gain, stall }, Unstated }`, `TailDecay(u64)`, `PeakGain { Zero,
Millibels(i32) }` and `FlushStall { Zero, Level(i32) }` are slice A1's (#1464, one rename of
`EffectTailBound`, no parallel struct; `InputSectionBound` deleted). `PeakGain::Zero` composes as
`-infinity` (absorbing in a sum, neutral in a maximum); `Millibels` is signed. The parts slice C
adds:

- `graph::GraphNode`'s `tail: TailSamples` becomes the node's `NodeTailBound`. It carries
  `tail_every_peak` although composition never reads it (ruling (c) forbids a narrower struct).
  `GraphNode` is control-side plan data; the implementer verifies that no render-owned structure
  holds one, and if one does, the slice stops and reports.
- `GraphCompileReport` gains `output_tail_every_peak: TailSamples` and `output_flush_floor`
  (`P*_graph`: `Zero`, `Millibels(i32)` re 1.0 rounded up, `AboveRange` when the stall sum is not
  finite, or unstated when `output_tail` is `Infinite`) beside `output_tail`.
- The canonical plan text: each node's `tail` row carries the whole bound (`tail`,
  `tail_every_peak`, the two rest values, decay, both gains, stall; `unstated`, `zero` and
  `above_range` as tokens); a new `extent` row carries `output_tail`, `output_tail_every_peak` and
  `P*_graph`. The `node` rows and the DOT text keep `tail` as today.

### H3. Each node's values

Every gain is computed from an upper bound of the linear gain, rounded up to the next millibel, with
the `f32` rounding of the gain word and of each product included. "Underflow" is `2^-126` per
rounded operation on the output's dependency chain (flush-to-zero or gradual underflow, as
`math::tail`'s `UNDERFLOW`), rounded up into a `FlushStall::Level`.

| node | `T` | `D` | `G_p` | `G_t` | `sigma` | rest |
|---|---|---|---|---|---|---|
| track `Input` (source; delay line, D2a) | `max(left, right) delay_samples` | 0 | 0 mB | 0 mB | `Zero` | `T` for both peaks |
| submix `Input` (sum of `m >= 1` route edges; delay line if any) | max delay | 0 | `m (1 + u)^(m-1)` | same | `(m - 1)` underflows | `T` for both peaks |
| submix `Input`, `m = 0` | max delay | 0 | `Zero` | `Zero` | `Zero` | `T` for both peaks |
| `PostInputBuiltins`, fixed design (#1329 D4) | #1329's | H1 certificate | `|trim| (1 + u) (O + dev_loud)` | `= G_p` | the module's stall | #1329's |
| `PostInputBuiltins`, filters disabled | 0 | 0 | `|trim| (1 + u)`, or `|trim|` where the product is exact (a power-of-two trim, 0 dB included) | same | underflow | `ZERO` |
| `PostInputBuiltins`, live input lane | #1433's | H1 certificate, the maximum over the settled groups (slice B1) | supremum over every admitted history (slice B1) | derived (slice B2) | the live stall (slice B1) | #1433's |
| `PostSimd1`, `PostDynamic`, `PostSimd2PreFader` (identity boundaries) | 0 | 0 | 0 mB | 0 mB | `Zero` | `ZERO` |
| effect | its prepared metadata | | | | | |
| `PostFader` (fader, mute, VCA) | 0 | 0 | live: the `f32` word of +24 dB; fixed: the prepared effective gain, or `Zero` if muted | same | underflow (`Zero` if `G_p` is `Zero`) | `ZERO` |
| `PostMatrix` (2x2 matrix, pan) | 0 | 0 | live: row `l1` bound 2; fixed: the largest prepared row `l1` | same | 2 underflows | `ZERO` |
| `Route` | 0 | 0 | live: `2 * 10^(ROUTE_GAIN_DB_MAXIMUM / 20) * ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM`; fixed (its own gain and matrix fixed): the prepared ungated gain times the largest prepared ungated row `l1`, or `Zero` where the mute rule below allows it; each with the fold's rounding | same | 2 underflows (`Zero` if `G_p` is `Zero`) | `ZERO` |
| `Output` (sum of `m >= 1` routes; `m = 0` as the submix row) | 0 | 0 | `m (1 + u)^(m-1)` | same | `(m - 1)` underflows | `ZERO` |

- **Fixed and live input sections.** Their values, derivations and gates are slices A2 (#1465:
  `G_p = G_t = ceil_mB(|trim| (1 + u) (O + dev_loud))`, `O` the module's certified output majorant
  sum), B1 (#1466: the live `D`, `G_p`, `sigma`) and B2 (#1467: the live `G_t`, derived, never
  `G_p`). The fixed `G_p`'s triangle inequality is sound but about 14 dB loose at the top pair;
  #1468 tightens it (m6). Details: the design note, H3.
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

**Range hypothesis.** The node statements apply without overflow: every input section sanitizes
`|x| >= 1e30` and non-finite samples to `+0.0` (`NONFINITE_LIMIT`, `crates/lane/src/kernels/builtins.rs`),
zeroes a lane block and resets its state on a bad output (`crates/builtins/src/lib.rs:1861-1876`),
and no effect receives such a sample in a compiled plan (`crates/effect-contract/src/live.rs:860-862`).
Each replacement only lowers magnitudes, so every bound holds a fortiori (design note, H4).

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
6. *Floor:* `S_out(v) = g_p(v) max_u S_out(u) + sigma_v` (`sigma_v` at a source; `0` after a `Zero`
   gain), `S_in(v) = max_u S_out(u)` over signal predecessors; the effective stall
   `sigma'_v = sigma_v + (3/4) eps 10^(-k(v)) S_in(v)` if `D_v > 0`, else `sigma_v`;
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

**The inequality chain** (induction in schedule order; path weights by a uniform backward walk,
`sum_p w_p <= 1`; the decay branch below `eps P` for `P >= P*_graph`; exact zero propagating below
it), the fan-in and delay rules and where the ruling's sketch changes are written out in the design
note, H4; the derivation note `docs/derivations/1379-graph-tail-composition.md` proves them in
full.

### H5. The exact-zero gate (ruling (d))

Gate C6, with its precondition (for DC or the worst-sign pattern, the last non-zero output sample
falls at or after `N + output_tail`) and its design list per rate: (1) the 1 kHz input LPF, every
downstream gain at 0 dB; (2) the same LPF in C1's gain shape; (3) a design that holds a
sub-`REST_EPS` state until the joint flush arms. The estimates behind the list are in the design
note, H5 and H9. If no listed design meets the precondition at a rate, the slice stops and reports.

### H6. What is not claimed

Nothing for a control event at or after `N`; nothing about a meter tap, a send tap or any node
output other than the graph output; below `P*_graph`, only exact zero from
`output_tail_every_peak`; tightness beyond the gated lines (the looseness is sound; #1468 tightens
the fixed `G_p`); feedback and a sidechain's magnitude; a graph-level `RestSamples` and any
render-side consumer (#1107 reads node `RestSamples`); a kernel other than the one the values were
derived from. The full list is in the design note, H6.

### H7. Interactions

#1107 is unchanged (it reads node `RestSamples`; nothing here feeds render). A2, B1 and B2 each rerun
#1457's gates 2 and 4. #1461 and #1462 land before A1. #1375 and #1376 follow A2, each with its D4
restated to H1 (with the sidechain quantifier), and land in the batch with C (confirmed by root, 2026-10-06); #1378 retires
`CompositionBound::Unstated`; root records these in #1372-#1376 and #1378. #1237, #1407 and #1408
bound the route, fader, trim and matrix values. #1277 D4 and #1276 D1 carry a gain only when it is
live or equal to the successor's prepared value (H3). Stream F (#1261, #1262) reports the
every-peak value through host-core's report (H10). The full text is in the design note, H7.

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

#1375 and #1376 follow A2 and land in the batch with C (Q3, third round; confirmed by root, 2026-10-06). A1, A2, B1 and B2 share
`crates/builtins/src/tail.rs` and `tail_contract.rs`, so they are sequential; A2, B1, B2 and the
`G_p` successor also share `crates/math/src/tail.rs`. The `G_p` successor may land before or after
C; if after, it re-pins the graph digests its tighter value moves, one at a time.

**Sizes.** B was split (root ruling 4) because `G_t` became a full derivation: B1 certifies `D`,
`G_p` and `sigma` behind accessors with the bound still `Unstated` (H2 forbids a partial
statement), B2 derives `G_t` and states all four; each is half a day and verifiable alone. C is
about one working day, an explicit exception to the half-day rule (root ruling m7; Hazards).

**Hot-file slots and named exceptions** for every slice are listed in the design note, H8, and in
`docs/handoffs/decision-15-2026-10-05/STREAMS.md` (stream G's "Owns" line and the hot-file table).
Slice C's own exceptions are in "Authorized paths" above.

### H9. Evidence

The `f64` probes, the first reviewer's reproduction and the confirmation review's probes (F2(b)'s
excess, `dev_loud`, the margins, F1(b)'s branches, C6's estimates, the `P*_graph` estimate) are in
the design note, H9.

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

## Attempt record

### Attempt 1 (2026-10-06): stopped before code, not counted

The implementer stopped on a spec conflict before writing code: D1-D3 (2026-10-05) predated #1329
Amendment 3's two tails, and three faults followed (Amendment 1's opening paragraph). Root ruled
Amendment 1; under its Q9 this attempt is not counted, and the next verified run of slice C is its
attempt 1.
