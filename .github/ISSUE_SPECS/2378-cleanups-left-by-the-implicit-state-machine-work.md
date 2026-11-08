# Cleanups left by the implicit-state-machine work

Successor bundle filed when the implicit-state-machine tracker #2258 closed. These are small removals and pairings that the reviews recorded. None is a reachable defect today, except possibly C9, which is verified first. Each row is independent; split freely, and drop a row that turns out to be already done.

| # | Area | Cleanup | Origin |
|---|---|---|---|
| C1 | effects | Merge the limiter's private `SilentClaim` into `effect_runtime::silent`. | #2320 |
| C2 | effects | Pair `lane_default_bytes` with its validation at the type level, and make the limiter validator tie `history[k]` to its main ring. | #2324 |
| C3 | effects | `parse_lane` in dynamic-eq and de-esser still admits `-0.0` loosely, and the compressor and transient-shaper param-ramp restore sites lack the at-target clause. Tighten them to the reachable words. | #2316 |
| C4 | graph / rack | A trait-level `Drained` token, so every effect that must hold a drain carries it. | #2326 |
| C5 | engine | Remove `read_into` (with the tap's `Option` and its `Default`). | #2307 |
| C6 | control-plane | Remove the dead multi-hold API (`ReliableEventReservations` and friends in `queue.rs`). Make `compile_children_warm` reachable, or gate it with `test-support`. | #2330, #2348 |
| C7 | control-plane | The solo path clones the committed compilation only so `compiled()` can lend it, and the controller's `if !monitor_only` is a second commit decision. Fold both into the transaction form. | #2286 |
| C8 | graph-compiler | The banked member processor kept on the `compile_with_builtins` path is dead weight. | #2303 |
| C9 | control-plane | Verify first: `materialize_prepared` may leave a sessionless token on a store error. If it does, this is a defect: give it a red test and its own commit. | #2330 |
| C10 | host-core | The `prepare.rs` doc link to the feature-gated `crate::warm` breaks `cargo doc -p host-core` without `control-provider`. | pre-existing |
| C11 | tooling | The call-graph gate's Ms2 and Ms3 impl-index pins couple source order to the gate. Key them by name. | #2332 |
| C12 | web/sdk | One generated source for both MSB1 seek-case consumers (a packaging change). | #2282 |
| C13 | engine | Derive `MeterDelivery`'s `gate_from`, and fix the `OutputPeakGate::activate` doc, when `meters.rs` is next touched. | HC-7 |
| C14 | host-web (JS) | Three web-host guards that #2377's T9 found unreachable: `snapshotDocument`'s offset/length check, `webError`'s `planes` parameter, and the boot `finish`'s `settled` (`miso-engine-v1-audio-worklet-host.js`). Delete each, or show that it is reachable and give it a row. | #2377 |
| C15 | builtins | `FaderRampStage::reset` still zeroes the dead f32 countdown word that #2377's T2 stopped writing on import. | #2377 |

## Gates
- A removal or refactor needs no new test and moves no bit: the no-bit-moved comparison and the downstream crates (graph-compiler fixtures, host-core) are PR evidence.
- C9, if confirmed, follows the defect rule: a test that is red on base.
- `scripts/check-workspace-policy.sh` and the call-graph gate are green.

## Evidence / attempt record

Attempt 1 (Terra), branch `codex/issue-2378-ism-cleanups`, one commit per row.

- **C9, confirmed defect.** `ProtocolController::materialize_prepared` took the token's
  `prospective_session` out and handed it by value to `SessionStore::materialize`; on a store
  refusal the session was gone, leaving a token whose `prospective_session()` panics ("live affine
  token owns prospective session") and whose edits are lost. The one production caller
  (`control-plane` `commit_live`) drops the token on that error, but the API is public and the
  refusal is reachable through it: a store at another revision (a foreign controller, which the
  call does not check) refuses with `RevisionConflict`. Fix: `SessionStore::materialize` takes
  `&mut PreparedSessionTransaction` and replaces it only on success, and `materialize_prepared`
  lends it. Test `controller::tests::a_refused_materialization_leaves_the_token_its_in_place_session`
  is red on base: `panicked at crates/protocol/src/controller.rs:1528:14: live affine token owns
  prospective session`. The base is the mutation (the `take()` restored turns it red).
- **C10, done.** On base `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p host-core` fails:
  `error: unresolved link to crate::warm::production_config --> crates/host-core/src/prepare.rs:262:20`.
  The `WarmConfig` doc now names the function in plain code and adds the link only under
  `cfg_attr(feature = "control-provider")`; the build is green with and without the feature, and
  the featured page still links `host_core::warm::production_config`. Doc only: no bit moves.
- **C15, done.** `FaderRampStage::reset` no longer writes `ramp[c].remaining`. The kernel's `f32`
  countdown word has one writer, `process_plane` (`crates/builtins/src/lib.rs:3708`), which
  rebuilds it from the `u32` countdown immediately before its one reader,
  `gain_mute_ramp_block`; `fader_bank_lane_words` already leaves it out. So the zero was
  overwritten before any read. No production render path calls this reset at all (its callers
  are `BuiltinFaderBank::reset` and `FaderMuteRampBuiltins::reset`, which `graph` never calls), so
  no rendered bit can move; the `builtins` suite (163 tests, `tests/randomized.rs` resetting banks
  against scalar references among them) is green.
- **C13, done.** `MeterDelivery::gate_from` is gone: `output_gate_from()` derives
  `if level { activation } else { INACTIVE }`, which is what `align` stored. The field had no
  other reader, and between `set_level`'s `self.level = on` and its `align` it could hold
  `level == false` with an open value (#2258 HC-7's "also"); now no state can disagree. `align`
  makes the same `activate`/`deactivate` calls with the same words, so the gate render reads, and
  every rendered or metered bit, is unchanged. `OutputPeakGate::activate`'s doc claimed "a new
  activation discards a window the meter has open"; the meter reads the gate once per block and
  compares it to the value its window was opened under, so `activate(X); deactivate();
  activate(X)` between two blocks, or `activate` at the value it holds, keeps the window. The doc
  now says that. Mutation: `output_gate_from` forced to `INACTIVE` turns
  `tests/meters.rs::a_restart_begins_a_new_generation_on_the_next_grid_boundary` red at line 517;
  forced to `activation` was green on every test (the level-off branch had no reader, nor did the
  base's `gate_from = INACTIVE` write), so that test now also asserts the gate reads shut once the
  level is off, and the mutant is red at line 525.
- **C5, done.** The row is `host_core::observation` (the issue's "engine" area is the origin's
  #2307 wording). `read_into` had no production caller; the C ABI and the browser read through
  `control-plane`'s `read_with`. It goes, with the `RowsTooShort` refusal only it returned (and
  its `None` arm in `control-plane`'s `observation_refusal`), `ObservationRow`'s `Default`,
  `ObservationReading`'s `Default` impl, and `tap: Option`: a row's `tap` is now
  `&'static ObservationDescriptor`, so `wire_row` reads its unit directly. Every row a read wrote
  carried `Some(tap)` before, so the `map_or(Db, ..)` fallback was unreachable and the wire bytes
  are unchanged. No render code is touched. The two `tests/effect_observation.rs` tests that
  called `read_into` read through `read_with` (a `read_rows` helper recording each write); the
  batch test now asserts a refused batch makes no `write` call, which replaces the sentinel-row
  comparison, and that a good batch writes positions 0 and 1 once each. Mutation: `read_with`
  writing each row inside its check loop turns `a_read_checks_the_whole_batch_before_it_writes_a_row`
  red at line 1550 ("a refused batch writes no row"). `control-plane` `observation_bindings` (22)
  and the C ABI's two observation-read tests are green.
- **C6, done.** `ReliableEventReservations`, its `Drop`, `reserve_reliable_events`,
  `commit_reserved_reliable_event` and `release_reliable_events` leave `crates/protocol/src/queue.rs`
  and the crate's re-exports; their only caller was the #2254 test. That test keeps its claim
  (a hold is committed only by its issuer, and a refused hold changes neither queue) over single
  holds: two reservations each on both queues show every unit came back. Mutation: commenting
  out `assert_issued_here` in `commit_reliable_event` turns it red ("a foreign hold is refused",
  `queue.rs:925`). `compile_children_warm` is now `#[cfg(all(test, feature = "test-support"))]`:
  on base `cargo clippy -p control-plane --all-targets -- -D warnings` fails with "function
  `compile_children_warm` is never used" (`compile.rs:2729`); it is green now, and so is the
  `--all-features` run. No render code is touched.
- **C8, done.** `EffectPlacement::Banked` loses `kept`: the `Vec` supply of
  `GraphCompiler::compile_with_builtins` drops a banked member's processor at placement, as
  `compile_projected` never prepares one, so both paths hand the plan the same effect table.
  Nothing read the kept processor: `graph`'s bind (`crates/graph/src/runtime.rs:9072`) takes a
  node's per-node effect only when the node has no bank membership, and a member's response
  facts are the bank's (they overwrite the effect's, `runtime.rs:8929`), so a kept processor was
  dropped unrun at bind. #2303's own mutation record already showed the `kept: None` mutant
  surviving the whole `graph-compiler` suite. Evidence: `graph_fixture --check` exits 0 (no plan
  fixture moves), `cargo test -p graph-compiler` (206 tests) and the `host-core` suites that
  compile through `compile_with_builtins` (`live_lanes`, `live_routes`, `route_mute`,
  `submix_strip`, `tail_composition`) are green, `audit` and `console-workload` build. Only the
  test/tool compile path changes; the production `compile_projected` path is untouched, so no
  rendered bit can move.
- **C14, done: all three guards deleted** from `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`.
  - `snapshotDocument`'s offset/length comparison: the intrinsic getters keep a `Uint8Array` view
    inside its buffer with `byteLength == length`, and a detached or out-of-bounds view reads
    `0`/`0`/`0`, so the comparison never refused anything. The refusals it seemed to make are made
    elsewhere: a `SharedArrayBuffer` backing by the intrinsic `ArrayBuffer` `byteLength` getter,
    which throws on it (that call stays, with a comment saying why), and a detached or
    out-of-bounds view by the copy's `set`, which throws on it (Node 24: both `TypeError`). The
    `byteOffset` and `byteLength` intrinsic captures only fed the comparison and go with it.
  - `webError`'s `planes` parameter: no caller passes a third argument. A peer's error that
    carries planes is forwarded as the peer's own frozen message, so `MisoError.planes` in the
    `.d.ts` stays, and the `.d.ts` is unchanged.
  - The boot `finish`'s `settled`: `finish` can run at most once, because its first call replaces
    (resolve) or clears (reject) every Worker handler that calls it, handlers are looked up at
    dispatch so a second queued event reaches the replacement, and the `postMessage` failure path
    runs inside the executor before any handler can. A comment now says so.
  - Evidence: `bash scripts/test-web-audioworklet.sh` (hermetic main-realm and worklet suites with
    their mutation rows) is green. Its rows that reach these sites, "SharedArrayBuffer document",
    "detached document" and "Int8Array document", stay green; deleting the kept `byteLength`
    call turns "SharedArrayBuffer document" red (`expected rejection`,
    `test-web-audioworklet.mjs:644`), so the row that pins the refusal still discriminates.
- **C11, done.** `scripts/check-web-audioworklet-callgraph.py` keys `INDIRECT_SITES` by the
  analyser's existing `kernel_key` (#1889: the unhashed name without the `M`/`X` impl
  disambiguator and with every back-reference as `B_`) instead of the unhashed name, so the
  `Ms2`/`Ms3` pins of `RenderReady::adopt_shape`, `RenderHalf::render_next` and
  `RenderHalf::poll_meters`, and the `Ms7`/`Ms4` pins of `RealtimePlanOwner` and
  `PreparedRenderPlan`, no longer move when an impl block is added or reordered (the `Ms6` to
  `Ms7` re-pin of 2026-10-16 is the cost this removes). Each key is the old pin through
  `kernel_key`, so the same artifact meets the same keys. `indirect_sites` also reports a key that
  members of different names share, and the export then fails, as the kernel roster already
  does. Self-test rows (h1e): a pin survives `Ms2_`/`B5_` to `Ms5_`/`B6_`, two members sharing a
  key fail, and the shipped table is in key form. Mutations: keying by `unhashed` again fails
  "(h1e) a pinned site matches its member after an impl-index move"; dropping the shared-key
  check fails "(h1e) two members sharing a pin key fail". `--self-test` passes.
- **C1, done.** The true-peak limiter's private `mod silent` goes; `LimiterCore::silent` is
  `effect_runtime::silent::SilentClaim`, the compressor's, EQ's and multiband's type, and the
  module's documentation moves to the field. Each restatement is exact:
  `settle(quiet, scan, every)` is `SilentClaim::settled(quiet, || scan, every)` (the scan, a pure
  read, now runs only for a quiet block, as the shared type promises); `quiet && proved()` beside
  its hand-written `debug_assert_eq!` against the scan is `quiet && trusted(|| scan, every)` on
  both the dual and the collapsed fast path (same value, same debug check). The one rule that
  differed is `settle_skipped` on a partial mask: the limiter's withdrew a standing claim, the
  shared type keeps it. The `advance_clocks` contract (`effect-contract` `lib.rs:3015`) says a
  skipped bank is skipped whole, so no caller passes a partial mask, and keeping is sound anyway:
  a standing claim says every lane rests, a lane outside the mask keeps its words, and
  `lanes_at_silent_rest` does not read the van Herk `phase` the mask's lanes advance. The
  `advance_clocks` doc says this, and `effect_runtime::silent`'s module and test docs no longer
  call the limiter a separate copy.
  - **No bit moved** (one-time, scratch, not committed): #2264's limiter harness (W4 and W8
    banks, 48 seeds each, 900 steps of loud bursts, `+0.0` stretches, `-0.0` and tiny words, both
    resets, lane resets, link retargets, restores from the bank's own and a donor's payloads,
    in-bank and cross-bank lane swaps, collapsed runs, automation and whole-bank skips; FNV-1a
    over every output bit, every state word including the claim, the rest mask and the
    fast-path engagement count after every step) gives 96 identical digests on base `839c1db7c`
    and on the change, in release, and again in a debug build, where every `trusted` check ran.
  - Mutation: deleting `self.silent.withdraw()` from `restore_track` turns three limiter tests
    red, `a_restore_withdraws_the_silence_claim` through the new `trusted` check on the dual fast
    path (`lib.rs:4411`). `cargo test -p true-peak-limiter -p effect-runtime` (165) is green.
- **C3, half done, half already done.**
  - *Compressor and transient shaper, the at-target clause: already done by #2368* (merged
    before this issue). Their restores now read every parameter ramp through
    `state_payload::read_ramp` / `LinearRamp::from_words`, which refuses a settled ramp off its
    target or with a nonzero step: compressor `crates/compressor/src/state.rs:144-146` (parameter),
    `:167` (rate) and `:183` (link); transient shaper `crates/transient-shaper/src/lib.rs:1588`
    (parameter) and `:1606` (link). #2368 has the red-on-base reproducers. Nothing to do.
  - *De-esser and dynamic EQ, `-0.0` countdowns: fixed.* Both crates' `integral_within` admitted
    `-0.0` (`(0.0..=limit).contains(&-0.0)` is true), so `parse_lane` accepted a `-0.0` in every
    parameter ramp's and link weight's `remaining` word and in both silence clocks. The engine
    writes a countdown as `u32 as f32` and a silence clock from `+0.0` upward
    (`lane::kernels::silence_advance`), never `-0.0`; such a lane would restore with a word the
    rest scan's exact `+0.0` test never reports at rest. `integral_within` now requires
    `is_sign_positive()`, so it admits exactly the reachable words. New test
    `glide_tests::a_negative_zero_countdown_is_refused` in each crate forges each countdown word
    of a rest payload (every ramp, both links, both clocks; each channel) to `-0.0`, expects the
    word's own refusal code and an unchanged snapshot. Red on base in both crates: `panicked at
    crates/de-esser/src/glide_tests.rs:956:22: a -0.0 countdown: ()` and `panicked at
    crates/dynamic-eq/src/glide_tests.rs:962:22: a -0.0 countdown: ()` (the restore succeeded).
    The base is the mutation (dropping `is_sign_positive()` restores it). The predicate changes
    only at `-0.0`, which no engine payload holds, so no carry or lane swap of an engine payload
    decides differently and no rendered bit moves; both crates' suites (39 tests, the bank and
    rest suites' restores and swaps among them) are green.

**Rows not landed: successors and drops** (corrected in the attempt 2 fold, after Sol's verdict).
- **C2(a), successor #SUCC-C2A.** Pair `lane_default_bytes` with `validate_track_state_payload`
  and `exchange_lane_defaults` at the type level (one `LaneSwap` hook). Attempt 1's reason was
  wrong: `swap_bank_lanes` already dispatches through `&mut dyn PreparedNativeEffectBank`, and the
  call-graph closure stops at `executor.render`, so no new pinned `call_indirect` site appears.
  The real cost is a trait change across the eight banking effects, their test banks and the
  callers.
- **C2(b), successor #SUCC-C2B.** Tie the limiter's `history[k]` bitwise to its main-ring word,
  once the relation is proven across every writer (resets, D7, the collapse disengage copy,
  restores, lookahead moves), so no live lane's own snapshot becomes unrestorable (the #2324
  failure shape).
- **C7, successor #SUCC-C7.** Fold the whole-path solo's `MonitorWhole(CompiledSession)` clone
  (`crates/protocol/src/model.rs:1769`) and the controller's second commit decision, the
  `(monitor_only, session_committed)` match (`controller.rs:3369`), into the token's form.
- **C15's follow-up, carried in #SUCC-C2A's spec.** With the reset's write gone, the fader's
  `ramp[c].remaining` is pure per-block scratch, rebuilt at the top of every ramping block in
  `process_plane` (`crates/builtins/src/lib.rs:3704-3710`); it should become a local there.
- **C4, dropped.** `rack`'s `run_with_input`, the one render driver of a bank chain, already
  requires `Drained` (`crates/rack/src/drain.rs`; `render_slots` and `skip_block` take it by
  value, `crates/rack/src/lib.rs:4609`, `:4714`), and no direct driver of a stage exists. A
  trait-level token would be minted by every `begin_block` override and prove nothing (#2326).
  Reopen if a second driver appears.
- **C12, dropped.** #2282's shared case table, `sdk/test/msb1-consumer-cases.mjs`, already runs
  every seek case against both consumers and so guards their drift, and the two copies differ on
  purpose (the prelude cannot allocate in `process()`). A generated shared source would buy no
  guard the table does not give, at the cost of a packaging change to the pinned artifact.

**Gates (attempt 1 head, one crate at a time, `-j 4`).**
- `cargo test --locked`: `protocol` (all targets), `control-plane --features test-support` (102),
  `host-core --features test-support,control-provider` (628), `capi` (238),
  `host-web --features test-support` (200), `graph-compiler` (206), `effect-compiler --features
  test-support` (59), `builtins` (163), `true-peak-limiter` + `effect-runtime` (165),
  `de-esser` + `dynamic-eq` (39): all green.
- `cargo clippy --locked --all-targets -- -D warnings`, with and without `--all-features`, over
  `protocol control-plane host-core builtins graph-compiler true-peak-limiter effect-runtime
  de-esser dynamic-eq`: clean. `cargo fmt --all --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` over the same crates: clean, and `host-core`
  without `control-provider` (C10) too.
- `bash scripts/check-workspace-policy.sh`: ok. `realtime_policy` (release): ok, 298 marked
  regions in 48 files. `graph_fixture --check`: exit 0.
- Web: `bash scripts/test-web-audioworklet.sh` green; `check-web-audioworklet-callgraph.py
  --self-test` passes; the shipped artifact rebuilt locally (`build-web-audioworklet.sh
  --named-twin`, nightly-2026-08-20) passes `check-web-audioworklet.sh
  --without-metadata-regeneration`, so the kernel-keyed `INDIRECT_SITES` (C11) match the real
  closure (`render` 11 sites over 5 members, `meter_poll` 2). Not run here: the Playwright
  Chromium and WebKit legs, which need a PulseAudio sink this host lacks; C14 changes only the
  main-realm host's input checks, which the hermetic suite drives.

### Verdict on attempt 1 (Sol): FAIL, one MAJOR

Every landed row except C9 was judged sound.
1. **MAJOR, C9.** `materialize_prepared` never checked the token's lease, though
   `check_prepared_structural` refuses `WrongController`. Sol's scratch test: two controllers at
   the same revision with different models; the foreign `materialize_prepared` returns `Ok`, the
   token now carries the foreign model, and `owner.commit_prepared_structural` commits it
   (`session_id` becomes `"foreign"`). Attempt 1's test reached the store refusal only through
   such a foreign controller, so its trigger becomes unreachable once the lease is checked.
2. **MINOR, C3.** The doc and test-value text were wrong about the two silence clocks: the rest
   scan (`channel_rest_lanes`) does not read them, and `silence_advance` would turn a `-0.0`
   positive. Refusing `-0.0` there only rejects a word the engine never writes.
3. **MINOR, C1.** `LimiterCore::lanes_at_rest` should say why it keeps its own debug check rather
   than `trusted`.
4. **Remaining rows.** C4 and C12 are dropped, C2(a)'s reason is corrected, C15 gains a
   follow-up, and C2(a), C2(b) and C7 get stateless successor specs.

### Attempt 2 (Terra): the fold, on the same branch

- **C9.** `materialize_prepared` now runs `check_prepared_structural` first (issued, current,
  revision) and returns the new typed `MaterializePreparedError::Token(PreparedCommandCommitError)`
  before touching the token; the store's refusal is `MaterializePreparedError::Store`. Its one
  production caller (`control-plane` `commit_live`) still maps any error to `Internal`. The
  attempt-1 test is replaced by
  `controller::tests::another_controllers_materialization_is_refused_and_the_owner_keeps_its_model`:
  a foreign controller at the same revision whose model has another `session_id` is refused with
  `Token(WrongController)`, the token stays in place, and the owner's materialization and commit
  install the owner's own model. Red on base's behaviour (the token check skipped, and the
  refusal assertion dropped so the commit runs): `assertion left == right failed: the owner kept
  its own session, left: StableId("foreign"), right: StableId("demo.session")` at
  `controller/tests.rs:1890`. With the check skipped but the refusal assertion kept, it is red at
  `:1882`. The store half of the claim (a refused materialization leaves the transaction as it
  was) is now structural, not tested: `SessionStore::materialize` takes the transaction `&mut`
  and assigns it only after `?` passed, and through the controller a current token can no longer
  meet a store refusal, so one test holds the row's one reachable claim.
- **C3.** `integral_within`'s doc (both crates) and both tests' test-value text now say that a
  `-0.0` countdown would never step and never read at rest, while a `-0.0` silence clock is
  refused only because the engine never writes one.
- **C1.** `LimiterCore::lanes_at_rest`'s doc says it keeps its own check because it believes
  `silent_claim_stands` (the claim with the ramp and glide legs), which `trusted` would not check.
- **Remaining rows.** Recorded above; the three successor specs are written for root to file.
