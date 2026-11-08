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
