# Let host-core attach a strip's fader and matrix lanes without its input, effect or route lanes

Core slice 2 of the umbrella *Deliver value-only fader, mute and pan transactions to the running
C ABI plan through the live console lanes* (#1053), its decision D5. Anchors verified on `main` at
`54b0a1bf8`.

## Product outcome

A host can ask preparation for live fader/mute and pan/matrix lanes on every strip, and for nothing
else. The C ABI needs exactly that (#1256).

Today one field, `HostLiveControlRequest::control_queue_depth`, attaches everything at once:

- each strip's matrix, fader **and input** queues;
- one queue per effect instance;
- one queue per route into a submix.

Two of those would change C ABI plans with nothing to gain:

- **The input lane makes every strip's builtin tail infinite.** Any strip with a control request
  gets `BuiltinTail::Infinite`, because a live filter target can enable a filter. Every C ABI
  resource report would then say `TAIL_INFINITE`. Its 40-byte records also make the largest of the
  three rings.
- **Effect and route lanes cost memory,** and a live route also loses the route fold (decision 14).
  The C ABI has nothing to push to them yet: effect lanes are decision 14's follow-up F5, and
  route lanes are #1225's.

The browser keeps every lane it has today, bit for bit.

## Context (verified at `54b0a1bf8`)

**builtins-compiler** (`crates/builtins-compiler/src/lib.rs`):

- `TrackControlRequest { track_id, queue_capacity }` (`:233-239`).
- `TrackControlProducer { track_id, producer, fader, input: Producer<TrackInputRecord> }`
  (`:246-262`).
- **Where the three queues are created** (`:3449-3483`). The consumers go into
  `StripControlConsumers { input: Some(..), fader: Some(..), matrix: Some(..) }`, whose fields are
  already `Option`. The bank input processor also keeps one `Option<Consumer<TrackInputRecord>>`
  per lane (`BuiltinBankProcessor::controls`, `:418-422`). So a strip with no input consumer can
  already be represented below the producer.
- **How the queues are charged.** Three rings per request, plus the producer vector and its seal,
  go to the processor accumulator (`:3628-3680`).
- **The tail rule.** A strip with a control request gets `BuiltinTail::Infinite`, both in the
  prepared tail (`:3435-3439`) and in the seal's `expected_tails` (`:3129-3150`, `:3143`).
- `control_capacity` (`:3414`) and `control_seal` (`:3476`) record the requests.
- There are 16 `TrackControlRequest {` literals in `crates`, `hosts` and `tools`.

**host-core** (`crates/host-core/src/prepare.rs`):

- `HostLiveControlRequest` (`:289-345`) and `HostLiveControlHandles` (`:359-430`).
- Every public entry runs through one function,
  `prepare_host_runtime_with_live_controls_policy_and_spectrum` (`:797-1408`). With
  `control_queue_depth` set, it attaches all three kinds of lane:
  - the effect lanes (`:942-947`);
  - a `TrackControlRequest` per strip (`:985-994`);
  - the route lanes (`:1127-1140`).

  Observation taps need the effect lanes (`:957-965`). `prepare_host_runtime_with_live_controls`
  (`:583-596`) prepares with `Concurrent` delivery (`between_render_calls = false`).

**Users of `TrackControlProducer::input`:**

- `hosts/host-web/src/lib.rs:1775` and `:1815`;
- `crates/host-core/tests/collapse_arming.rs:300`, `input_liveness_live_controls.rs:217` and
  `randomized.rs:709`;
- `crates/builtins-compiler/tests/input_drain.rs:221-266` and `allocation_tracker.rs:514`;
- `crates/builtins-compiler/src/lib.rs:9247`.

**The consumer side.** The test-only scalar oracle's `strip_bindings`
(`crates/builtins-compiler/src/lib.rs:2203-2240`) binds a controlled strip's three per-node
processors and calls `control.input.expect("a strip is banked on all three stages or on none")`.
With no input consumer it would panic under `Backend::Scalar`, the reference oracle the owner keeps.

## Decisions

- **D1. builtins-compiler.**
  - `TrackControlRequest` gains `pub input_lane: bool`, and `TrackControlProducer::input` becomes
    `Option<Producer<TrackInputRecord>>`.
  - **With `input_lane: false`:**
    - no input ring is created or charged;
    - the producer's `input` is `None`;
    - the strip's input bank lane gets no consumer;
    - the strip's builtin tail is the chain's own (`chain.tail()`), both in the prepared tail and
      in `expected_tails`;
    - the control seal records the flag, so a validated artifact cannot add or drop the lane.
  - **With `true`:** everything is as today. Every existing literal passes `true`.
  - **The scalar oracle.** In `strip_bindings`, a strip with no input consumer binds the plain
    `InputProcessor(input)`, the processor a strip without controls gets, instead of the live input
    processor. The fader and matrix stay live.
- **D2. host-core.**
  - Add `pub struct HostLiveLanes { pub strip_input: bool, pub effects: bool, pub routes: bool }`,
    with `HostLiveLanes::ALL` (all `true`) and `HostLiveLanes::FADER_AND_MATRIX` (all `false`).
  - Add `pub fn prepare_host_runtime_with_live_lanes(compiled: &CompiledSession, caps:
    &HostPrepareCaps, live_controls: &HostLiveControlRequest, lanes: HostLiveLanes) ->
    Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics>`, with `Concurrent`
    delivery.
  - Every existing entry passes `HostLiveLanes::ALL`.
  - The fader and matrix lanes are attached whenever `control_queue_depth` is `Some`.
    `strip_input`, `effects` and `routes` gate the other three.
  - A request with `observation_taps > 0` and `effects: false` is refused with
    `host.observation.live_controls`, the code `:957-965` already uses: a subscription rides the
    effect queue.
  - Re-export the type at the crate root next to `HostLiveControlRequest`.
- **D3. Documentation.** The docs of `HostLiveControlHandles::strip_controls` and
  `HostLiveControlRequest::control_queue_depth` say which lanes each selection attaches.
- **D4. host-web.** The two `.input` uses handle `Option`. host-web always prepares with the input
  lane, so for it `None` cannot occur; treat it as a full, unaddressable slot (no room, `Err` on
  push), never as a panic.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs`: the request, the producer, ring creation and charging,
  the tail rule, the seal and the bank wiring.
- `crates/builtins-compiler/tests/*.rs`: literal and `Option` updates, plus one allocation case
  (gate 3).
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs`.
- `crates/host-core/tests/*.rs`: literal and `Option` updates, and the new file
  `live_lanes.rs`.
- `hosts/host-web/src/lib.rs`: the two `.input` sites, and any `TrackControlRequest` literal.
- Any other `TrackControlRequest` literal under `crates`, `hosts` or `tools`.
- This spec.

## Non-goals

- No change to capi; #1256 uses the new entry.
- No change to the browser's lanes, its records, its delivery mode or its digests.
- No change to the input lane itself, or to what a live filter target does.

## Hazards

- **The seal.** `validate_for_session` compares the prepared tails with `expected_tails`, and the
  controls with `control_seal`. Change both with the tail rule, or a valid fader-and-matrix plan
  fails its own seal.
- **The iOS memset rule.** `scripts/check-cross-targets.sh` counts `bl _memset_pattern16` per
  product crate against `scripts/lib/aarch64-known-defects.py`. New code must not fill memory with
  a repeated non-zero constant.

## Objective gates

Run every command from the repository root.

1. **The new selection.** New file `crates/host-core/tests/live_lanes.rs`. Use two sessions: the
   nine-track EQ fixture (`fixtures/session/v1/parametric-eq-nine-track.json`), and a session with
   an empty console, no inserts and no input filter. First assert that the second session's
   lanes-free tail is finite: the EQ fixture's tail is infinite either way (its EQ declares an
   infinite tail, and its tracks enable input filters), so only the second session can show the
   tail rule. (The test-only backend seam, `prepare.rs:657-673`, is `#[cfg(test)]` and cannot be
   reached from `tests/`; the scalar oracle is covered by gate 1(d).)
   - **(a) What is attached.** `prepare_host_runtime_with_live_lanes(.., FADER_AND_MATRIX)` with
     depth 16 returns one producer per strip, with `input == None`, and no effect and no route
     producers. Its `report.output_tail` and `report.latency_samples` equal those of
     `prepare_host_runtime` on the same compiled session. Over 8 blocks fed the same source, its
     output is bit-identical to that plan's.
     *Test value: it turns red if a fader-and-matrix request still attaches the input lane (the
     tail becomes infinite), attaches effect or route lanes, or changes a rendered bit.*
   - **(b) The lanes are wired.** Push `TrackFaderRecord::Mute { lanes: Both, muted: true,
     smoothing_samples: 0 }` to one strip's `fader` producer, and a non-identity
     `TrackControlRecord` with smoothing 0 to another strip's `producer`, before the first block.
     Over 8 blocks, the output is bit-identical to that of a lanes-free plan prepared from the
     edited session (the mute and the matrix baked) and fed the same source from sample 0.
     *Test value: it turns red if the lanes are attached but no longer drained by the banks.*
   - **(c) Observation.** `observation_taps: 1` with `effects: false` is refused with
     `host.observation.live_controls`.
     *Test value: it turns red if an observation tap is attached without the effect queue it
     rides.*
   - **(d) The scalar oracle.** A unit test in `crates/builtins-compiler/src/lib.rs` prepares a
     two-track session with `input_lane: false` under `Backend::Scalar` and under the native
     backend. Both prepare; pushing one fader record and one matrix record before the first block
     gives bit-identical output from the two backends.
     *Test value: it turns red if the scalar oracle still expects an input consumer (it panics) or
     binds the strip differently from the bank.*
2. **Nothing else changes.**
   - Every existing host-core, builtins-compiler and host-web test passes. The existing tests run
     `HostLiveLanes::ALL` through the old entries.
   - The workspace test command:
     `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo test --locked -p builtins-compiler --no-run`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
3. **Exact charges.** `crates/builtins-compiler/tests/allocation_tracker.rs` gains one
   `input_lane: false` case. What the builtins report charges equals what preparation allocates,
   exactly as its existing cases require.
   *Test value: it turns red if an unrequested input ring is still allocated, or is still charged.*
4. **The shipped AudioWorklet module.**
   - Build it on this branch and on its base, and report both SHA-256 digests. The expected
     result is "changed", because the `Option` is on the push path:
     `rm -rf target/ci/qualification-artifacts target/ci/qualification-named-twin && mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts && sha256sum target/ci/qualification-artifacts/miso-engine-v1-audio-worklet.simd128.wasm`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   - `bash scripts/test-web-audioworklet.sh`
5. **Workspace and policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh debug` runs only on an arm64 host. It is
     CI-only here (the `aarch64-debug` job); record it as not run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The two module digests.

## Dependencies

None. This slice edits `crates/builtins-compiler/src/lib.rs`, and so do *Bound the builtin fader
and matrix drains to the records present at block entry* (#1253) and *Classify a committed
session delta as a live track fader, mute and pan update or a rebuild* (#1255), in other places.
Land them one at a time and rebase.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- "Bit-identical" gates are hard stops.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, base `b5489f00d`)

**Implementation.**

- builtins-compiler: `TrackControlRequest::input_lane`; `TrackControlProducer::input` is
  `Option`. With `false` no input ring is created or charged, the strip's input consumer is
  `None` (the bank lane already took `Option`), the tail is `chain.tail()` in both the prepared
  tails and `expected_tails`, and the control seal is `(track, capacity, input_lane)`, compared
  against the producers' `input.is_some()`. The scalar oracle's `strip_bindings` binds the plain
  `InputProcessor` for a strip with no input consumer. Every existing literal passes `true`.
- One change the gate forced: the request index (`control_capacity`, a `BTreeMap`) is built
  before `TestPhaseTwoAllocationGuard::begin` instead of after it. It is a transient lookup, not
  retained storage; with any control request its node was observed as an uncharged phase-two
  allocation, which the new exact-charge case (gate 3) caught. No existing case prepared controls
  under that observation. Production builds are unaffected (the guard is test-support only).
- host-core: `HostLiveLanes { strip_input, effects, routes }` with `ALL` and `FADER_AND_MATRIX`,
  re-exported at the crate root; `prepare_host_runtime_with_live_lanes` (Concurrent delivery).
  The private core function takes the selection; every other entry passes `ALL`. Effect lanes,
  the strip input lane and the route lanes are gated; `observation_taps > 0` with
  `effects: false` is refused with `host.observation.live_controls`. Docs of
  `control_queue_depth` and `strip_controls` say which lanes each selection attaches.
- host-web: the two `.input` sites treat `None` as an unaddressable slot (`queue_available`
  returns `None` -> `COMMAND_REASON_UNSUPPORTED_KIND`; `push` returns `Err`), never a panic.
- **Outside the authorized paths (minimum, forced by compilation):**
  `hosts/host-web/src/tests.rs` (one queue-counter closure reads `owner.input` through
  `as_ref().expect(..)`). `crates/graph-compiler/src/lib.rs` is covered by "any other
  `TrackControlRequest` literal" (4 literals get `input_lane: true`).

**New tests and their test value.**

- `crates/host-core/tests/live_lanes.rs` (the plain session is the EQ fixture with an empty
  console, no inserts, filters off, and its last track routed through a unity submix `bus`, so it
  has one route into a submix):
  - `only_the_plain_session_has_a_finite_lanes_free_tail` -- the precondition: plain is
    `Finite`, EQ is `Infinite`.
  - `fader_and_matrix_lanes_attach_nothing_else_and_render_the_lanes_free_bits` (1a) -- red if
    a fader-and-matrix request still attaches the input lane (tail becomes infinite), attaches
    effect or route lanes, or changes a rendered bit. It also checks `ALL` on the plain session
    gives `input: Some`, one route lane and an infinite tail.
  - `fader_and_matrix_lanes_render_as_the_session_with_their_values_baked` (1b) -- red if the
    lanes are attached but no longer drained by the banks. Mute on strip 2, matrix
    `[0.5 0.25; -0.5 0.75]` on strip 5, both smoothing 0, versus the baked session; it also
    asserts the baked edits change the output.
  - `observation_without_effect_lanes_is_refused` (1c) -- red if an observation tap is attached
    without the effect queue it rides.
- `builtins-compiler` unit test
  `input_free_live_strips_match_between_the_scalar_oracle_and_the_banks` (1d) -- red if the
  scalar oracle still expects an input consumer (it panics) or binds the strip differently from
  the bank. It also asserts the records moved the output.
- `crates/builtins-compiler/tests/allocation_tracker.rs`
  `live_control_rings_are_charged_exactly_as_allocated_with_and_without_the_input_lane` (gate 3)
  -- red if an unrequested input ring is still allocated, or is still charged; it also checks
  `input_lane: true` and that each prepared artifact passes its own seal.

**Mutation runs** (each applied, test run, reverted):

| # | Mutation | Result |
|---|---|---|
| M1 | host-core passes `input_lane: true` regardless of `lanes.strip_input` | red: 1a (`input.is_none()`) |
| M1b | builtins tail rule ignores `input_lane` (`.is_some()` / `.is_ok()` in both prepared tails and `expected_tails`) | red: 1a, `left: Infinite right: Finite(0)` |
| M2 | effect lanes attached regardless of `lanes.effects` | red: 1a (`effect_controls.is_empty()`) |
| M3 | route lanes attached regardless of `lanes.routes` | red: 1a (`route_controls.is_empty()`) |
| M4 | the fader bank stops taking its consumer (`.and_then(\|_\| None)`) | red: 1b |
| M5 | observation refusal drops `\|\| !lanes.effects` | red: 1c |
| M6 | scalar oracle panics on a missing input consumer (the pre-change `expect`) | red: 1d |
| M7 | input ring charged unconditionally | red: gate 3 (layouts differ) |
| M8 | input ring created unconditionally (`input_lane = true` at creation) | red: gate 3 (layouts differ) |

**Gates (all from the head of this attempt, x86-64-v3 host).**

- Gate 1: `cargo test -p host-core --features test-support --test live_lanes` 4 passed;
  1(d) and gate 3 pass.
- Gate 2: the workspace test command: exit 0, 116 result lines, 1300 passed, 0 failed.
  `cargo test --locked -p builtins-compiler --no-run`: ok.
  `cargo test --locked --release -p audit -p bench -p console-workload`: ok.
- Gate 4: AudioWorklet module SHA-256, base `b5489f00d`:
  `6eb292980c2f31a7a2b86ef8388a17b1f0b5efa1b70544ec1a955d1bd207188f`; this attempt:
  `04ce0d44483b5ebce30619c3abcf7991e9dd0d4d69719c3df30def529b3e6f52` -- changed, as expected.
  `check-web-audioworklet.sh --without-metadata-regeneration`: passed.
  `check-browser-expected-resources.py --artifacts`: digests and rows agree, within budget.
  `test-web-audioworklet.sh`: passed.
- Gate 5: `cargo fmt --all -- --check` ok; clippy `-D warnings` ok; `cargo doc` with
  `-D warnings` ok; host-core, realtime and workspace check/test policy scripts ok;
  `check-cross-targets.sh` PASS (iOS `memset_pattern16` expected failures unchanged, #1018).
  AArch64 (`run-aarch64-tests.sh debug`): not run locally, CI-only.
- Batch rules: `check-capi-abi.sh` ok; `audit capi`: 100,000 calls, 0 allocations, 0 syscalls,
  0 violations.
- Lesson (a): no session, request or wire field changed, so no browser qualification stub needs
  one. Lesson (c): no new CI command or test-file pattern.

### Follow-ups applied (after the attempt 1 PASS, verdict MINOR 1-3 and NIT 1-2)

- MINOR 1: gate 1(a) (`fader_and_matrix_lanes_attach_nothing_else_and_render_the_lanes_free_bits`)
  now asserts `Concurrent` delivery. For each fixture it prepares the same request through
  `prepare_host_runtime_between_render_calls` and asserts the fader-matrix witness saw at least one
  fused-bank factory call (non-vacuity), then prepares through `prepare_host_runtime_with_live_lanes`
  and asserts `factory_calls == 0`. Test value: red if the live-lanes entry prepares with
  `BetweenRenderCalls` delivery, which renders the same bits and so passed every earlier test.
  Mutation: `between_render_calls` `false` -> `true` in `prepare_host_runtime_with_live_lanes`:
  red (`left: 2, right: 0`, "Concurrent delivery forms no fused fader-matrix bank"); reverted, green.
- MINOR 2: `HostLiveControlRequest::control_queue_depth`'s doc names each lane's own flag.
- MINOR 3: the `effect_controls` and `route_controls` docs state the `HostLiveLanes` condition, and
  `route_controls` states #1053 D5 (C ABI plans prepare with `FADER_AND_MATRIX`; their route lanes
  arrive with #1225).
- NIT 1: both builtins-compiler comments now say "two or three" rings.
- NIT 2: the precondition (`only_the_plain_session_has_a_finite_lanes_free_tail`) is folded into
  the top of gate 1(a); the separate test is deleted.
