# Grow latency during playback by adopting a primed warm successor

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-9, D15-12, D15-17).
Formerly slice 17 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`main` at `6fb211594`.

This issue is the umbrella of the **warm successor** and owns its proof obligation, the lemma
below. Its own deliverable is the **first slice** below; the later slices are listed at the end and
are filed separately. Owner question Q2 of #1269 (pre-roll, a reserve or a gap) is answered by
D15-8 (round-5 amendment): prime adoption. There is no off-thread catch-up, no render-thread
pre-roll and no reserve.

## Product outcome (whole issue)

Adding a latent effect that raises a node's latency while audio plays (the output's or a
submix's) neither gaps nor jumps the mix. Every carried path continues bit for bit. The mix
becomes later only in the source-read clock, which hosts anchor seeks on (D15-12). Render pays one
bounded raw-frame prime in the adoption block, at most `PRIME_BYTES_MAX`, never `k` blocks of DSP.
There is no permanent latency reserve. The only fallback is the transition, counted and reported
through the watermark's `transition_fallback` flag.

## Product outcome (this slice)

A floor on a node that reads a source is realised as a source-claim compensation line, in the
compiler and the graph runtime. A successor compiled with the lemma's lead floors (on carried
nodes only) puts every carried node at exactly its predecessor arrival plus `P`, and gives a
restarted strip's `Input` stage no claim line. A plan compiled with lead floors renders the
predecessor's mix delayed by exactly `P`. Gates 1-3 check these. The claim-line carry and the fill
rules are *Carry source-claim lines across a plan swap and fill a grown line for a prime* (#1402).

## Context

- **No history is needed.** The samples a grown claim line needs are future samples (round 1 C2,
  round 2 Q-C2). For a carried node they are raw source frames the host has already queued, not
  processed samples: under C1-C4 below, a carried node processes exactly what the predecessor's
  node processes (round-4 M6). So render fills them from the rings at adoption.
- **The swap today.** `RealtimePlanOwner::enter_block`
  (`crates/engine/src/realtime/plan_exchange.rs:375-437`) takes a candidate, adopts the clock
  (`:418`), and runs the carry in one block. `GraphExecutor::adopt_predecessor`
  (`crates/graph/src/lib.rs:3139`) moves the sources (`adopt_sources`, called at `:3169`;
  `crates/source/src/lib.rs:1851`) and copies the input lanes. `GraphCarryProgram`
  (`crates/graph/src/lib.rs:2757`) holds only the source pairs today.
- **Timing.** `pdc::timings` (`crates/graph-compiler/src/pdc.rs:37-120`) gives each node the
  maximum incoming arrival and compensates every incoming edge to it. A node with no incoming edge
  gets arrival 0 (`:61-65`). A track's input stage reads its source through a claim
  (`GraphSourceInputClaim`, `crates/graph/src/lib.rs:2131`), not through a graph edge. So a floor
  on it, under *Keep every node's latency from dropping during playback* (#1285) D1, would raise
  its arrival with no line to delay its content.
- **Sidechains.** An `EffectSidechain` edge (`crates/graph-compiler/src/compile.rs:393-416`) reads
  a strip's tap (`route_source_node` and `stage`, `crates/graph-compiler/src/ids.rs:187-220`) and
  has no gain lane, so a duck cannot silence it.
- **Clocks.** The graph driver reads every source at the block's absolute sample
  (`consumer.begin_block_at(first_sample)`, `crates/source/src/lib.rs:1721`).
  `PreparedRenderPlan` keeps one clock: `next_absolute_sample` (`crates/engine/src/realtime/plan.rs:609`),
  passed to the executor as `RenderTime` (`:943`). *Give a plan a source-read clock that leads its
  render clock* (#1396) adds the source-read offset `O`.
- **Ring headroom.** `default_source_ring_frames` (`crates/host-core/src/prepare.rs:65`) is 100 ms
  (`SOURCE_STALL_TOLERANCE_MS`, `:57`) plus two quanta. #1354 D1 names that body
  `stall_ring_frames`, the baseline a warm successor's ring headroom is measured above.

## The lemma (binding on every slice)

Every stream C slice implements this contract. A slice that cannot meet it stops and reports; it
does not weaken it.

**Setup.** Predecessor A has source-read offset `O`, recorded floored input arrivals `a(n)` (#1285
D2), and claim lines of length `λ`. Successor W's nodes split into:
- C, carried nodes;
- R, the nodes of `PreparedHost::restarted_strips()` (*Carry fader, mute and pan ramps across a
  plan swap*, #1277 D6; *Duck-swap a strip whose state cannot continue across a plan swap*, #1324
  D1): every node before the fader, and the fader (#1324 D2), plus every node of a strip that the
  C1 iteration restarts whole (#1324 D2, `restart_whole`);
- N, added nodes.

`Δ` is the largest arrival growth over **C only**, and `P = q · ceil(Δ / q)` for quantum `q`.
Floors are `a(n) + P` on C, `a(n)` on R, none on N. W's source-read offset is `O + P` (#1396).

**Conditions.** Each holds exactly, or the edit takes the counted transition.
- **C1 (alignment).** `a'(n) = a(n) + P` for every `n` in C, checked at preparation. Floors are
  lower bounds (#1285 D1), so a node can arrive later than its floor. If one does, preparation
  restarts strips whole (every node of a restarted strip joins R), recomputes `Δ` over the new C,
  and checks again (#1354 D2 step 6):
  - it takes the first late C node in schedule order and walks back from its late inputs through
    R and N nodes along every edge kind (route, added route, sidechain: PDC takes its maximum over
    sidechain edges too, `crates/graph-compiler/src/pdc.rs:61-65`), following the inputs that hold
    each arrival;
  - it restarts whole the strip of every C node that holds that path at its lead (a route node
    counts as its source strip), recursively through every one that is a submix;
  - only when the walk finds no such node does it restart the late node's own strip whole; the
    output belongs to no strip, so there the walk finding none returns
    `WarmUnavailable::Misaligned`.

  It iterates until C1 holds. Only if every strip that reaches the output is restarted whole does
  preparation return `WarmUnavailable::Misaligned`, and the edit takes the transition, which gives
  that same audio. So a latent insert added on a submix (a limiter on a bus) is warm-exact: its
  carried feeders restart with it and duck-swap (W9). A kick that keys the bass's compressor, when
  the edit adds a limiter to the bass, restarts the kick, whose key holds the bass's chain late.
  Every other path, the output included, stays exact. A
  growth that the restarts confine to R leaves `Δ = 0`: the edit is then an ordinary rebuild that
  duck-swaps those strips, not a transition.
- **C2 (isolation).** Every edge from R or N into C carries exact `+0.0` over `[S, fire)`: fader
  and post-fader paths, ducked routes (#1324, #1391, #1363), armed fades (#1288). An
  `EffectSidechain` edge has no gain lane, so the tap decides, exactly as #1324 D1's rule: a
  sidechain from an R strip's `post_input`, `insert_send`, `insert_return` or `pre_fader` tap puts
  the consuming strip in R (repeated until no strip joins). A track's `input` tap (its raw source,
  whose line L3 fills) and a `post_fader` or `post_pan` tap (exact `+0.0` from the duck's end to
  the fire) keep the consumer in C. A restarted submix's `input` tap (the sum of its incoming
  routes) keeps the consumer in C if every line into that submix's `Input` stage comes from an R
  or added strip (exact `+0.0` from the duck's end to the fire, with `S` counting the sidechain
  line plus the longest line into the stage, #1397 D1 step 3), and otherwise only if the stage
  arrives at exactly `a(n) + P`, checked with C1 (with `P = 0` for a rebuild that grows nothing,
  #1354 D2 step 4). If it arrives at `a(n) + g` with `g ≠ P`, every carried line into that stage
  and the sidechain line out of it change length by `P - g` samples, so the consuming strip joins
  R. Under a lead that needs a line into the stage that is not carried, such as a route from a
  restarted strip's tap before the fader: a carried route's floor holds the stage at `a(n) + P`.
- **C3 (bounds).** `ΣP + P <= P_MAX` and the prime's bytes `<= PRIME_BYTES_MAX`, else the
  transition (`WarmUnavailable::LeadBound`, `WarmUnavailable::PrimeBudget`).
- **C4 (readiness).** Render checks it on the **active** plan's consumers before it claims: `S >=
  not_before` (the duck has settled and its lines have drained); every source W carries has its
  next `k + 1 = P/q + 1` blocks queued and playable (active generation, contiguous from
  `next_frame`, or past the end of the region); and no command is queued and no held seek is
  anchored below `S + O + P + q` (*Let a source consumer check and replay its next blocks for a
  prime*, #1320 D2: that covers the window `[S + O, S + O + P + q)` and an anchor in
  `(S + O - q, S + O)`, which applies late at `S + O`).

**L1 (edges).** For a C->C edge, `c' = (a(n) + P) - (a(m) + P) - lat(m) = c`. Each such line moves
unchanged (*Carry compensation lines across a plan swap*, #1283 D2).

**L2 (claim lines).** A W claim line of length `λ'` emits source-read sample `r + O + P - λ'` at
render sample `r`. At S it is filled with the last `λ'` samples of `[A's λ pending] ++ [prime]`.
The prime is `k` consumer blocks at source-read `S + O + j·q`, `j = 0, ..., k - 1`, on A's own
schedule, read without observing new commands.
- C lines (`λ' = λ + P`): the emission equals A's. The order is A's pending at the read end, then
  the prime. #1283 D4's head-aligned copy would put `+0.0` where the prime goes.
- R lines (`λ' = λ`): the shifted fill.
- N lines: `+0.0` at the head only when `λ' > P`, behind the arm.
- With `P = 0` and equal lengths (every ordinary rebuild), L2 is the move: swap the rings and the
  cursor.

**L3 (`Input`-tap lines from R into C).** Such a line grows by `P`. Its fill is A's line pending
`++` A's claim pending `++` the prime, which yields A's content.

**L4 (induction).** Under C1-C4, for every `n` in C and every `r >= S` up to the first fire
upstream of `n`, W's state and input at `n` equal A's continued without a swap (same host commands,
as functions of the source-read clock). State moves at S by move-mode carry (#1277-#1284, #1327,
#1395). Inputs are equal by L1-L3 and C2. Banking never changes per-lane bits. Live edits committed
after the commit apply at S from W's cells (*Deliver value-only fader, mute and pan transactions to
the running C ABI plan through the live console lanes*, #1053 D7; D15-2).

Against an uninterrupted render of the new graph, C nodes not downstream of R or N are equal.
Downstream of R or N no mechanism can be equal, because the new chain has no history. There the
contract is the D15-9 fade-in or duck-swap, and the prime meets #1288's and #1324's references
exactly: restarted and added chains start at rest at S, as a fresh plan from S does.

**Corollary.** An off-thread catch-up would give a C node either A's state at S, which the move
gives directly, or a wrong one. D15-8 (round-5 amendment) retires it.

**Seeks and clock.** A held seek inside the prime window waits (C4), then applies exactly. After S
the source-read clock reads `O + P` ahead of render (#1396 D4 readers). A command that arrives
during the adoption callback is observed at block S on the new clock.

## Mechanism (binding on every slice)

- **W1. Preparation.** Submit prepares W with the lemma's floors, checks C1 with the strip-restart
  iteration, checks C2 and C3, and publishes `Primed { not_before, lead_blocks }` (#1311, #1403).
  It never waits for render.
- **W2. Prime adoption.** While a `Primed` candidate is pending, render runs C4 before it claims.
  It reads per-ring `prime_required` atomics, which control stores before the `Release` that
  publishes. Once ready, it claims and adopts in move mode in the same block. Then, in the source
  section after `adopt_sources`, each carried consumer replays `k` blocks (`prime_block_at`) and
  render fills the claim lines by L2 and L3. There is no copy at B, no return, and no in-flight
  state.
- **W3. Clocks.** Every successor inherits its predecessor's source-read offset; a warm successor
  adds `P`. An ordinary rebuild never moves the source-read clock back. Only a host-declared
  discontinuity resets the offset (and `ΣP`) to 0, and with it the floors (#1396, *Reset latency
  floors at a host-declared discontinuity*, #1323).
- **W4. Edits while pending.** A pending warm successor is an ordinary pending candidate (#1053
  D7): live edits go to the newest candidate's cells and apply at S. Retargets are written once, at
  preparation (#1277 D5, move mode). A structural edit supersedes it by compare-and-swap
  (*Supersede an unadopted candidate plan by compare-and-swap*, #1310, including its D5 step 5,
  `adopt_persisting`). A host-declared stop supersedes it with a plain rebuild through #1310
  (#1323). Its revision completes `exact` or `superseded`.
- **W5. Observers.** Meters, observation taps and spectrum captures carry by move (#1327, #1395):
  observers carry by #1327 D1's keys, in a warm adoption as at any swap.
- **W6. Fallback.** The transition is the only fallback, counted `TRANSITION_FALLBACK` and
  reported through the watermark's `transition_fallback` flag (#1314). It runs on
  `WarmUnavailable` at submit, or when C4 is still unmet `PRIME_DEADLINE_SAMPLES` of render after
  `not_before` (#1358 D1). Then the next control call withdraws the candidate (`Taken` means it
  was adopted exactly), re-prepares with the withdrawn candidate as donor, and publishes
  (`fall_back_to_transition`, #1397). While render waits, the predecessor keeps playing exactly. A
  host that never queues `P + q` frames ahead gets the transition: any exact mechanism needs those
  frames.
- **W7. Deadline.** It is counted in render samples, so a paused host never falls back (D15-17).
- **W8. Constants.** `P_MAX` and `PRIME_BYTES_MAX` are engine constants. *Record the swap block's
  cost on the 64-track console* (#1286) owns their single derivation (the prime adoption block
  measured at `ΣP = P_MAX`), with *Prove two Wasm instances on one shared memory in three browser
  engines and on iOS* (#1331) for the browser rows. *Grow the default source ring by the
  warm-prime headroom* (#1406) writes `P_MAX_SAMPLES(fs, q)` and the ring headroom (`P_MAX` plus
  one quantum) into code, and #1360 writes `PRIME_BYTES_MAX`. No other spec
  restates a formula.
- **W9. Edits that duck strips.** An edit that both grows latency and ducks strips, restarting
  them (#1324 D1, such as a latent insert added to an audible strip) or removing them (#1325),
  ducks those strips on the predecessor (#1324 D4's duck set), and `not_before` holds adoption until
  the duck has settled and its lines have drained (#1397 D1). The restarted strips fade in at
  `S + D` (#1324 D3, #1288 D4). Carried paths stay exact.

## Decisions frozen for this slice

- **D1. Source-claim line.** When the floor on a source-reading node exceeds its incoming arrival
  (0), the compiler inserts a source-claim compensation line of `floor` samples on each of that
  node's claims. It is reported beside `InsertedDelay` and runs on the existing PDC delay kernel
  (`pdc_delay_block`, `crates/lane/src/kernels.rs:990`). A claim read in place (#918) through such a
  line becomes a copy.
- **D2. No change without floors.** Without floors the program and its bytes do not change.
- **D3. Record.** Before implementing, check the lemma against the compiler and record any
  correction in this spec's decision record.

The claim-line carry, keyed by (claiming node, source), the L2 and L3 fill routine and the
host-core join are *Carry source-claim lines across a plan swap and fill a grown line for a prime*
(#1402).

## Deliverables

1. D3, then D1-D2 in the graph compiler and the graph runtime.
2. `crates/graph-compiler/tests/latency_growth.rs` (new) with gates 1-3.

## Authorized paths

- `crates/graph-compiler/src/pdc.rs`, `crates/graph-compiler/src/lib.rs`,
  `crates/graph-compiler/src/compile.rs` (the claim lines beside `inserted_delays`, `:870`),
  `crates/graph-compiler/src/estimate.rs` (their bytes, beside the `InsertedDelay` row at `:385`),
  `crates/graph-compiler/src/canonical.rs` (their evidence and `dot` rows, `:157`, `:343`),
  `crates/graph-compiler/tests/latency_growth.rs` (new)
- `crates/graph/src/lib.rs` and `crates/graph/src/runtime.rs` (the claim-line runtime only)
- `.github/ISSUE_SPECS/1287-grow-latency-during-playback-by-adopting-a-primed-warm-successor.md`

## Non-goals

- No claim-line carry, fill routine or host-core join (#1402).
- No `warm_lead`, no C1 iteration and no `WarmUnavailable` (#1354): gate 1 passes the lemma's floor
  map to the compiler directly.
- No readiness check, `prime_block_at`, `Primed` publication or adoption (#1320, #1311, #1403,
  #1355).
- No duck-swap or fallback (#1397, #1358).

## Hazards

- **Ownership.** `crates/graph` and `crates/graph-compiler` are stream A's files. This slice lands
  after #1285, in the merge order root sets.

## Objective gates

1. **B1 compile.** Predecessor A is two tracks routed to the output with no latency, built the
   way `crates/graph-compiler/tests/compile_shapes.rs` builds its sessions (the shape of #1397
   gate 1). Successor W adds a true-peak limiter insert to track 1. W is compiled with the lemma's
   floors: track 1's nodes before the fader and its fader at `a(n)` (R), the limiter with none (N),
   every other node at `a(n) + P` (C), with `P` from `Δ` over C at 48 kHz and a 128-frame quantum.
   Every C node arrives at exactly `a(n) + P`. Every C->C edge has A's compensation. Track 2's
   claim line is exactly `P` samples. Track 1's `Input` stage has no claim line. A second case
   adds a muted limiter track inside a submix whose arrival grows while the output's does not; the
   same checks hold with `P` from the submix's growth.
2. **Delay relation.** A plan compiled from A with every node floored at A plus `P` renders A's
   output delayed by exactly `P` samples, bit for bit, from frame 0, after `P` samples of `+0.0`.
   Both plans are bound with `bind_with_source_set` (`crates/graph/src/lib.rs:1882`) to a test
   `GraphPreparedSourceSetDriver` that plays frame-encoded samples, as
   `crates/graph/tests/rt10_source_in_place_alloc.rs:64` does. All four launch rates, both bank
   widths.
3. **No change without floors.** `cargo test --locked -p graph-compiler` and the graph fixture
   corpus pass unchanged.
4. Commands:
   - `cargo test --locked -p graph-compiler`
   - `cargo test --locked -p graph --features graph/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a lead floor on every surviving node (the round-4 B1 defect) floors track 1's `Input`
  stage, gives it a claim line, and moves the output off `a + P`; floors with no claim line, as
  #1285 D1 alone would give, show none on track 2. Red.
- Gate 2: a claim line padded at the wrong end, or a line on only one lane of a dual-mono track,
  breaks the exact delay. Red.
- Gate 3: a claim line or estimate row emitted with no floor changes an existing plan's bytes.
  Red.

## Dependencies

- *Keep every node's latency from dropping during playback* (#1285): floors.

Each later slice below names its own dependencies in its own body.

## Later slices (filed)

Every slice must also keep zero allocations, frees and syscalls on render.

1. *Carry source-claim lines across a plan swap and fill a grown line for a prime* (#1402): the
   claim-line carry keyed by (claiming node, source), the L2 and L3 fill routine, the host-core
   join and the carry-level M1 gate.
2. *Let a source consumer check and replay its next blocks for a prime* (#1320): the C4 readiness
   check, `prime_block_at` and `prime_required`.
3. *Give a plan a source-read clock that leads its render clock* (#1396): W3's offset inheritance,
   its reset at #1323's declaration, and the clock readers.
4. *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `warm_lead`
   over C, the C1 iteration, C2's sidechain restart, C3 and `WarmUnavailable`.
5. *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): W2, `Primed`,
   the source-read offset `O + P`, and the render-level gates (including the render-level M1).
6. *Classify a latency-growth edit and publish its warm successor from the control plane*
   (#1403): classification with an injected `WarmConfig`, `publish_primed` and the
   `PrimedCandidate` record.
7. *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
   successor cannot adopt* (#1397): W9 and W6's `fall_back_to_transition`.
8. *Grow the default source ring by the warm-prime headroom* (#1406): `P_MAX_SAMPLES` and the
   ring headroom, with every pin of the ring rule.
9. *Fall back to the transition when a warm successor is not ready by its deadline* (#1358): W7
   and `PRIME_DEADLINE_SAMPLES`.
10. *Check the warm-successor deadline in miso_engine_v1_service and report its outcome* (#1360).
11. *Check the warm-successor deadline in the browser Worker's service loop and report its outcome*
    (#1361).

`P_MAX` and `PRIME_BYTES_MAX` come from *Record the swap block's cost on the 64-track console*
(#1286). *Adopt a successor plan no earlier than a scheduled sample* (#1311) adds the `Primed`
publication kind.
