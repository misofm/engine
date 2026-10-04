# Prepare C ABI plans with live track fader and matrix lanes

Core slice 4 of the umbrella *Deliver value-only fader, mute and pan transactions to the running
C ABI plan through the live console lanes* (#1053), its decisions D4 and D5. Anchors verified on
`main` at `54b0a1bf8`; re-verify those in host-core after #1254 lands.

## Product outcome

Every C ABI plan, at compile and at every structural replacement, carries one live fader/mute lane
and one live matrix/pan lane per strip. capi keeps their producers with the plan's provider epoch,
and charges them to the byte. Nothing pushes to them yet (#1257 does).

Rendering, `latency_samples`, `tail_kind` and `tail_samples` do not change. The resource report
grows by the rings (the builtin rows) and by the producer table (the capi row).

## Context (verified at `54b0a1bf8`)

All paths are in `crates/capi/` unless named.

- **Preparation.**
  - `prepare_runtime` (`src/runtime/compile.rs:412-471`) calls `prepare_host_runtime(compiled,
    &caps)` (`:421`). That requests no live controls (`crates/host-core/src/prepare.rs:563-574`).
  - `PreparedRuntime` (`compile.rs:11-17`) carries the sources, the plan, the report, the catalog
    and the capi resources.
  - The tail is projected at `:430-433`.
- **The provider epoch.**
  - `ProviderEpoch { epoch, sources }` (`src/runtime/control.rs:9-12`), with `::current` and
    `::candidate` (`:15-31`).
  - It is built at compile (`compile.rs:519-525`, `:588`) and in the structural arm
    (`control.rs:755-762`).
  - `synchronize_plan_epochs` drops a reclaimed plan before it removes that plan's provider
    (`control.rs:640-655`). So producers are dropped after their plan, as the source producers
    are, and the last owner of a shared ring frees it on the control thread.
- **capi's own charges.**
  - `capi_resources` (`compile.rs:109-214`) charges the epoch rows (`:147-152`) through host-core
    mirrors: `control_table_bytes` and `source_id_arena_bytes`
    (`crates/host-core/src/source.rs:269`, `:276`).
  - `checked_layout::<ProviderEpoch>(2)` (`:171`) charges the struct's own growth.
  - `prepared_capi_resources` (`:216-244`) computes them from the compiled session.
- **The oracles** (`tests/resource_lifecycle.rs`).
  - `capi_retained_bytes_charge_every_byte_the_compile_retains` (`:730`). Its host half replays
    `host_core::prepare_host_runtime` (`host_half`, `:511`). `observe_compile` (`:587`) drops the
    owners one by one (`HostOwners`, `:568`). The source producers go first, so a ring they share
    with the plan is counted with the plan (`:555-575`).
  - `double_live_oracle_drives_exact_and_one_below_c_caps` (`:1263`) derives its caps from the two
    live reports.
- **builtins' own charges.** builtins already charges each control's rings, its producer vector and
  its seal to its processor accumulator (`crates/builtins-compiler/src/lib.rs:3628-3680`). They land
  in `builtin_retained_payload_bytes`, which is checked against `maximum_builtin_retained_bytes`.
- **Dependencies.** capi depends on host-core, not on builtins-compiler (`Cargo.toml`). Every type
  it names must come through host-core.
- **The selection (#1254).** `host_core::prepare_host_runtime_with_live_lanes(compiled, caps,
  live_controls, HostLiveLanes::FADER_AND_MATRIX)` returns `(PreparedHost,
  HostLiveControlHandles)`, with `Concurrent` delivery. The C ABI renders with that delivery today.
- **The realtime audit.** `audit capi` (`tools/audit/src/capi.rs`) renders 100,000 calls of the
  nine-track EQ fixture through the C entry point.

## Decisions

- **D1. The request.** `prepare_runtime` calls `prepare_host_runtime_with_live_lanes` with
  `HostLiveControlRequest { control_queue_depth: Some(LIVE_QUEUE_DEPTH),
  ..HostLiveControlRequest::default() }` and `HostLiveLanes::FADER_AND_MATRIX`.
  `pub(crate) const LIVE_QUEUE_DEPTH: NonZeroUsize` is 16 (#1053 D4) and lives in `compile.rs`.
- **D2. The epoch keeps the producers.**
  - `PreparedRuntime` and `ProviderEpoch` gain `strips: StripLanes`, a new struct in `control.rs`.
    It holds `controls: Box<[TrackControlProducer]>`, in `HostLiveControlHandles::strips` order
    (the tracks, then the submixes), and `track_count: usize`.
  - capi drops the rest of the handles at preparation, on the control thread: the strip ID list
    `HostLiveControlHandles::strips` (one `Box<str>` per strip, `prepare.rs:368-371`; each producer
    keeps its own `track_id`) and the vectors that are empty with this selection.
  - The producer vector becomes the boxed slice with `into_boxed_slice()`. If that reallocates
    (spare capacity), the oracle sees it; size the vector exactly or accept the shrink in the
    window.
  - `ProviderEpoch::current` and `::candidate` take the new field.
- **D3. host-core.**
  - Re-export `TrackControlProducer`.
  - Add the mirror `pub fn strip_control_table_bytes(strip_count: usize, strip_id_bytes: usize) ->
    Option<u64>`: the boxed producer slice plus each producer's `track_id` bytes.
  - capi adds that mirror to `epoch_rows`. `prepared_capi_resources` computes its arguments from
    the compiled session's strips, tracks and submixes alike.
- **D4. The oracle.**
  - `host_half` replays the same `prepare_host_runtime_with_live_lanes` call and, inside its
    observed window, does what capi does: it drops the strip ID list and the empty vectors and
    keeps only `strip_controls` as the boxed slice. Otherwise `host_live` counts the ID list, which
    capi does not keep, and the exactness assertion is off by its size.
  - `HostOwners` gains `strips`, dropped first, like the sources. `observe_compile`'s owner sum
    (`resource_lifecycle.rs:616-620`, "the four owners") becomes five.
  - `assert_capi_retained_bytes_are_complete` adds `strips` to its left side.
  - `assert_host_owners_are_charged` asserts that `owners.strips` equals
    `strip_control_table_bytes(..)`.
  - builtins' processor accumulator also charges the producer vector. That is a conservative
    double charge across two different caps, and it predates this slice. Record it; do not remove
    it here.
- **D5. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` gains a paragraph:
  - every plan carries two 16-record rings per strip, charged in `builtin_retained_payload_bytes`,
    and the producer table, charged in `capi_retained_bytes`;
  - a caller whose caps were exact before this slice must raise `maximum_builtin_retained_bytes`
    and `maximum_capi_retained_bytes`;
  - the tail and the latency do not change.

## Authorized paths

- `crates/capi/src/runtime/compile.rs`, `control.rs`, `mod.rs` and `tests.rs`.
- `crates/capi/tests/resource_lifecycle.rs`.
- `crates/host-core/src/lib.rs`, plus `prepare.rs` or `source.rs` for the mirror only.
- `crates/builtins-compiler/src/lib.rs`: `TrackControlProducer`'s documentation only.
- `tools/bench/src/console.rs`: the one sentence of module documentation only.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No push and no classifier: #1257 does that.
- No input, effect or route lanes (#1053 D5).
- No benchmark row. The console benchmark's C-ABI-shaped rows prepare without controls. The real
  C ABI path now checks two empty queues per strip per block: two atomic loads. A row for it is a
  separate bench issue, if the owner wants one.

## Hazards

- **Two stale comments.** `TrackControlProducer`'s documentation says "a producer must be dropped
  before the plan that owns its consumer" (`crates/builtins-compiler/src/lib.rs:242-244`), and so
  does `EffectControlProducer`'s (`crates/effect-compiler/src/prepare.rs:631-633`). Both contradict
  `PreparedHost`'s field order and capi's order: the rings are shared `Arc`s, so either order is
  sound, and the last owner frees the ring on the control thread. Correct the builtins comment in
  this slice. Leave the effect one to #1263.
- **The benchmark's description.** `tools/bench/src/console.rs:168` says the C ABI prepares "with no
  live controls". After this slice it attaches fader and matrix lanes (routes stay static). Correct
  the sentence. A row that attaches the lanes is a separate bench issue, if the owner wants one.

- **Tight caps in existing tests.** The builtin and capi rows grow. A test with a hand-picked
  `maximum_builtin_retained_bytes` or `maximum_capi_retained_bytes` may need a higher value. Raise
  it with a comment that gives the reason; never loosen an exactness assertion.
- **The iOS memset rule.** capi has no row in `scripts/lib/aarch64-known-defects.py`, so one
  `bl _memset_pattern16` in capi fails `scripts/check-cross-targets.sh`. Do not fill memory with a
  repeated non-zero constant.

## Objective gates

Run every command from the repository root.

1. **Same rendering, same tail.** Add a new test in `crates/capi/src/runtime/tests.rs`,
   `c_abi_plans_with_live_lanes_render_like_lanes_free_plans`. Run it for 1 and 10 tracks, at
   44.1, 48, 88.2 and 96 kHz, on `generated_parity_session` (`tests.rs:110`). Run it also on a
   session with an empty console, no inserts and no input filter, whose lanes-free tail must first
   be asserted `TAIL_FINITE`: the parity session's EQ and input filters make both tails infinite,
   so only this second session can tell them apart.
   - Prepare the C ABI plan and a `host_core::prepare_host_runtime` plan of the same compiled
     session. Feed both the same source blocks.
   - Their 8 rendered blocks are bit-identical.
   - The C ABI report's `latency_samples`, `tail_kind` and `tail_samples` equal those of the
     lanes-free host report.
   - `providers.strips.controls` holds one producer per strip, in canonical order, each with
     `input == None`.

   *Test value: it turns red if capi attaches the input lane (`TAIL_INFINITE`), changes a rendered
   bit, or drops the producers.*
2. **Exact charges.** `cargo test --locked -p capi --test resource_lifecycle`. Both oracles pass
   with D3 and D4.
   *Test value of the oracle change: it turns red if capi keeps the producer table but does not
   charge it, or charges more than it keeps.*
3. **Nothing else moves.**
   - `cargo test --locked -p capi`: every existing test passes.
   - The workspace test command:
     `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
4. **The C ABI artifacts.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `./target/release/audit capi` exits 0 and reports zero allocations, frees, locks and
     syscalls.
   - `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`
   - `python3 -B scripts/check-scalar-oracle-absent.py --native target/release/libcapi.so`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
5. **Workspace and policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh debug` (capi and host-core are in its crate
     list) is CI-only here (the `aarch64-debug` job); record it as not run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new or changed test's name, with its one-sentence test value.
- The resource rows before and after, on the nine-track EQ fixture: `builtin_retained_payload_bytes`
  and `capi_retained_bytes`, each with its reason.

## Dependencies

- *Let host-core attach a strip's fader and matrix lanes without its input, effect or route lanes*
  (#1254).

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- "Bit-identical" gates are hard stops.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, base `a2a417c9a`)

**What landed.**
- `crates/capi/src/runtime/compile.rs`: `LIVE_QUEUE_DEPTH` (16); `prepare_runtime` calls
  `prepare_host_runtime_with_live_lanes` with D1's request and `HostLiveLanes::FADER_AND_MATRIX`,
  destructures the handles keeping only `strip_controls` (boxed slice) and `track_count`, and drops
  the rest on the control thread. `PreparedRuntime` gains `strips`. `capi_resources` takes
  `strip_count`/`strip_id_bytes` and adds `host_core::strip_control_table_bytes` to `epoch_rows`;
  `prepared_capi_resources` computes them over `normalized_model().strips()` (tracks and submixes).
- `crates/capi/src/runtime/control.rs`: `StripLanes { controls, track_count }`; `ProviderEpoch`
  gains `strips`, and `::current`/`::candidate` take it; the structural arm passes the candidate's.
  `StripLanes` and the `strips` field carry `#[allow(dead_code)]` (held for ownership; only tests
  read them until #1257).
- `crates/capi/src/runtime/mod.rs`: imports only.
- `crates/host-core/src/prepare.rs`: `pub fn strip_control_table_bytes(strip_count,
  strip_id_bytes)`; `lib.rs` re-exports it and `TrackControlProducer`.
- `crates/builtins-compiler/src/lib.rs`: `TrackControlProducer`'s stale drop-order sentences
  corrected (documentation only). `EffectControlProducer`'s is left to #1263.
- `tools/bench/src/console.rs`: the one module-doc sentence. `docs/C_ABI_V1_QUALIFICATION.md`: the
  D5 paragraph.
- `crates/capi/tests/resource_lifecycle.rs`: D4 (below), and the two builtin payload budgets
  raised with their reason.
- Nothing outside the authorized paths. No push, no classifier, no input/effect/route lane.

**The producer vector does not reallocate.** builtins-compiler builds it with
`Vec::with_capacity(controls.len())` and pushes exactly one per request, so `into_boxed_slice()`
is a no-op; the oracle's window includes it and is exact.

**New or changed tests and their test values.**
- `c_abi_plans_with_live_lanes_render_like_lanes_free_plans` (`crates/capi/src/runtime/tests.rs`,
  gate 1): 1 and 10 tracks x 44.1/48/88.2/96 kHz, on `generated_parity_session` and on
  `bare_parity_session` (empty console, no inserts, both filters off; its lanes-free tail is
  asserted `TAIL_FINITE`). 8 blocks bit-identical against `host_core::prepare_host_runtime` of the
  same session (and at least one non-zero sample), equal `latency_samples`/`tail_kind`/
  `tail_samples`, one producer per strip in canonical order, `input == None`, `track_count`.
  *Red if capi attaches the input lane, changes a rendered bit, or drops the producers.*
- `resource_lifecycle` oracle (D4): `host_half` replays the same live-lane preparation and keeps
  only the boxed producer slice; `HostOwners.strips` is dropped first; the owner sum is five;
  `assert_capi_retained_bytes_are_complete` adds `owners.strips`; `assert_host_owners_are_charged`
  asserts `owners.strips == strip_control_table_bytes(..)` over the compiled session's strips. The
  double-live oracle adds the prospective strip table, measured from the kept slice
  (`size_of_val` + ID bytes), to its capi epoch term. *Red if capi keeps the producer table but
  does not charge it, or charges more than it keeps.*

**Mutation runs** (each reverted; tree restored from a saved copy):
- M1 `strip_input: true` in capi's lane selection: gate 1 red ("no strip carries an input lane").
- M1b the same, with the test's `input.is_none()` assertion disabled: red on
  "bare 1 tracks at 44100 Hz: tail kind" (left 1, right 0); the parity session passed the tail
  check, so only the bare session discriminates, as the spec says.
- M2 producers dropped (`controls` empty): red ("one producer per strip").
- M5 a fader `Mute` record pushed onto strip 0 at preparation: red, "parity 1 tracks at 44100 Hz:
  block 0" (bits differ).
- M3 `routes: true` and M4 `effects: true`: gate 1 stays green. Candid: on these sessions a live
  route or effect lane moves no bit and no tail by construction, so gate 1 does not catch them.
  (Corrected after the verdict, MINOR 1: the earlier sentence said both "show only as larger
  resource rows, which the budget test bounds", which is wrong both ways.) M4 is caught exactly by
  `capi_retained_bytes_charge_every_byte_the_compile_retains` (left 387,119, right 258,135), by
  `tiny_control_frame_still_accounts_three_provider_counters_exactly` and by the budget test,
  because the oracle's `host_half` spells `FADER_AND_MATRIX` independently. M3 moves no row on any
  capi fixture (none has a route into a submix), so no test bounds it; the submix-routed oracle
  session is #1256 MINOR 1, folded into #1258.
- O1 strip row charged as zero: `capi_retained_bytes_charge_every_byte_the_compile_retains` red
  (left 258,135 observed, right 256,884).
- O2 strip row charged for one extra strip: red (left 258,135, right 258,271).

**Resource rows, nine-track EQ fixture, x86-64** (before at `a2a417c9a` -> after):
- `builtin_retained_payload_bytes` (= `builtin_processor_payload_bytes`): 17,451 -> 28,521
  (+11,070, 1,230 per strip): two 16-record rings per strip plus builtins' producer-vector entry and
  control-seal row. Budget raised 19,200 -> 31,424 (value + 10 %, rounded to 64) in
  `REFERENCE_BUDGETS`, with the reason in its doc.
- `capi_retained_bytes`: 256,812 -> 258,135 (+1,323): the strip table, nine 136-byte producers
  plus 27 ID bytes (1,251), and `ProviderEpoch` growing 40 -> 64 bytes in its three charged slots
  (current inside the session, pending, retired: 72). Inside its 282,432 budget.
- Double-live requirements: builtin 34,902 -> 57,042; capi 167,558 -> 170,132. Both oracles exact.
- Recorded, not removed (D4): builtins' processor accumulator also charges the producer vector
  (`add_vector_layout::<TrackControlProducer>`), so the table is charged once to the builtin cap
  and once to the capi cap. Conservative; it predates this slice.

**Gates** (all on the implementation tree):
- `cargo test --locked -p capi`: 36 + 11 passed. `--test resource_lifecycle`: 11 passed.
- `cargo test --locked -p host-core --all-features`: pass.
- The workspace test command (gate 3): pass.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`: pass.
  `./target/release/audit capi`: 100,000 calls, allocations 0, deallocations 0, locks 0,
  syscalls 0, total_violations 0.
- `check-capi-abi.sh` and `--self-test`: pass. `check-scalar-oracle-absent.py --native
  target/release/libcapi.so`: pass. `cargo test --locked --release -p audit -p bench -p
  console-workload`: pass.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- host-core, realtime and workspace policy check + self-test: pass. `check-cross-targets.sh`:
  pass (no new `memset_pattern16` in capi). `check-ci-path-routing.py`: pass.
- Worklet chain (host-core is in the browser module): `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`: pass.
- 4-lane NEON (`run-aarch64-tests.sh debug`): CI only, not run locally.

### Follow-ups applied (after the attempt 1 PASS, verdict NIT 1 and NIT 4)

- NIT 1: `largest` in `crates/capi/src/runtime/compile.rs` keeps the whole strip row and now says
  it is conservative (the slice and the `track_id`s are separate allocations, so it can only refuse
  earlier). Feeding `Layout::array::<TrackControlProducer>(n)` instead was declined: it would move
  `largest_allocation_bytes` for no behavioural gain, and the safe-direction comment is the
  verdict's second option.
- NIT 4: the charge-site comment in `compile.rs` names D4's double charge (builtins' processor
  accumulator also charges the producer vector, against the builtin cap).
- The M3/M4 sentence in the mutation runs above is corrected.
- MINOR 1 (a submix-routed session in the charge oracle) is folded into #1258; NIT 2 and NIT 3 are
  not in this follow-up's scope.
- Comment-only code changes: no new test, so no mutation run.
- Gates on the follow-up tree (`978463341`, x86-64-v3, one run for the whole #1253-#1257
  follow-up set): `cargo fmt --all -- --check`; `cargo test --locked` for `host-core
  --all-features`, `builtins`, `builtins-compiler`, `capi` and `host-web`; workspace clippy
  `--all-targets --all-features -D warnings`; `cargo doc` with `-D warnings`; release build and
  `./target/release/audit capi` (100,000 calls, allocations 0, syscalls 0, total_violations 0);
  `check-capi-abi.sh`; realtime, workspace and host-core policy (check, plus the realtime and
  workspace self-tests); `check-cross-targets.sh` (iOS `memset_pattern16` expected failures
  unchanged): all pass. Worklet chain not run: no line compiled into the browser module changed
  (host-core, host-web, builtins and builtins-compiler changed only in docs, comments and
  tests).

### Verdict

**Verdict.** Sol attempt 1: PASS (verdict file `docs/handoffs/live-updates-1053/1256-attempt1.md`). MINOR 1 (an exact resource oracle with submix strips) folded into #1258 (`0fdb4f886`); NIT 1 and NIT 4 applied in `b6a91ef63`; NIT 2 (the `allow(dead_code)`) is gone at head, since #1257 and #1264 read every field, and NIT 3's wording ("outlive whichever ... drops first, and the last owner frees") is in `control.rs`.
