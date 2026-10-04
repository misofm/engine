# Apply value-only parametric EQ parameter edits to the running C ABI plan through prepared targets

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is part of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259): each EQ band's frequency, gain,
Q and shelf slope, and the HPF and LPF `enabled`, frequency and Q, are live through prepared
targets. Anchors verified on `main` at `54b0a1bf8`; re-verify them after #1264 lands.

## Product outcome

A C ABI transaction that changes a live parametric EQ parameter, on a console slot's entry or an
insert of a track, is a live update:

- the control thread designs the EQ's prepared targets off the render thread;
- the EQ's owner transaction publishes them;
- the readback reports the new value.

A band's `enabled` and `kind` stay prepared (decision 14, follow-up F2) and still rebuild.

## Context (verified at `54b0a1bf8`)

- **The owner.** A target-capable effect (the EQ) gets an `EffectControlOwner` when its lane is
  attached (`crates/effect-compiler/src/prepare.rs:1497-1523`). Its producer then refuses plain
  `Parameter` records (`preflight`, `:802-809`).
- **The owner's transaction API** (`prepare.rs:864-940`):
  - `begin_owner(base_revision)`, `edit_owner(index, channel, value)`;
  - `preflight_candidate_targets(base, targets)`, which checks the revision and the complete queue
    room, and changes nothing;
  - `publish_candidate_targets(base, targets)`, `commit_owner()`, `discard_owner()`;
  - the owner's `committed_revision()` (`crates/effect-compiler/src/control.rs:145`).
- **The target designer.** host-core's stateless `EqTargetPreparer::prepare(sample_rate, seeds,
  edits, out)` (`crates/host-core/src/control_preparation.rs:305-457`). It takes 60 seed values in
  the canonical row order (`default_initial_values`), up to 256 edits, and returns at most 12
  targets. `EqTargetPreparer::new(parametric_eq_target_preparation_factory()?)` builds it.
- **The browser's sequence.**
  - `begin_owner`, then `edit_owner` for each EQ record (`hosts/host-web/src/lib.rs:5079-5145`);
  - the targets come through the prepared companion (`hosts/host-web/src/control_targets.rs`);
  - `publish_candidate_targets` in the publication pass (`lib.rs:5639`);
  - `commit_owner` after every queue is published (`:5722`).
- **The classifier.** #1264's D2 gives `Prepared` for every EQ parameter. Its D1 gives
  `resolve_initial_values`, the one source of an instance's 60 values.
- **The queue can be smaller than one target set.** An effect's queue capacity is
  `min(depth, automation_capacity)` (`crates/effect-compiler/src/prepare.rs:1471-1490`), and
  `automation_capacity` is the caller's `maximum_automation_spans_per_block`
  (`crates/effect-contract/src/lib.rs:2535`). On the C ABI that is `min(16, the limit)`, and any
  limit from 1 to 11 is legal, while `EqTargetPreparer` can return up to 12 targets.

## Decisions

- **D1. The classifier.** For a target-capable instance, `classify_live_delta` no longer gives
  `Prepared` for a parameter whose `automation_rate` is `Block`.
  - It seeds `EqTargetPreparer` with the instance's pre-commit values (from
    `resolve_initial_values`, in the canonical row order), passes the changed
    `(parameter_id, channel, value)` edits, and designs the targets on the control thread. A
    refusal gives `Domain`.
  - It reports both the edits and the targets on `LiveEffectRecords`: capi needs the edits for
    `edit_owner` and the targets for the preflight and the publication.
  - `enabled` and `kind` still give `Prepared`.
- **D2. capi, all or nothing.**
  0. **Capacity.** If an EQ instance's target count is greater than its queue's `capacity()`, the
     whole transaction takes the structural path, as #1264's D3 does for parameter records: it
     could never fit.
  1. For each EQ instance, in the newest epoch: `begin_owner(committed_revision())`, then
     `edit_owner` for each edit, then `preflight_candidate_targets`. A preflight refusal for lack
     of room is `LiveBackpressure` (#1257 D8); any other refusal is `Internal`.
  2. Then run every other step of #1053 D6 up to the protocol check.
  3. On any refusal before the first push, call `discard_owner` on every owner begun, and leave
     everything as it was.
  4. Then push the other records, `publish_candidate_targets`, commit the protocol, and
     `commit_owner`. None of the three can fail after the preflights; treat a failure as
     unreachable (`expect` with a message).
- **D3. The readback.** Set every changed EQ value, as #1264's D4 does.
- **D4. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` lists the live EQ parameters and the
  prepared ones.

## Authorized paths

- `crates/host-core/src/live_delta.rs` and `crates/host-core/tests/live_delta.rs`.
- `crates/capi/src/runtime/control.rs` and `live_tests.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No change to the EQ, its owner, `EqTargetPreparer` or the browser's companion protocol.
- No live `enabled` or `kind` (decision 14, F2).

## Hazards

- **Owner state is control-plane state.** A begun owner that is not discarded on a refusal poisons
  the next transaction. The rollback must cover every refusal path.
- **The EQ keeps state.** Gate 2 compares against the browser's path, never against a fresh plan.

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`:
   - a band gain change on one lane gives the edits and the targets `EqTargetPreparer` designs;
   - an HPF `enabled` change gives targets;
   - a band `kind` change gives `Prepared`;
   - an out-of-domain Q gives `Domain`.

   *Test value: it turns red if the classifier seeds the designer from values other than the ones
   preparation used, or lets a prepared-only EQ parameter go live.*
2. **Equal to the browser's lane.** A live EQ band edit through the C ABI, and targets designed in
   the test by calling `EqTargetPreparer::prepare` directly (never taken from the classifier),
   published through the owner of a host-core plan prepared with `HostLiveLanes::ALL` at the same
   block, render bit-identically on the nine-track EQ fixture, at the four launch rates. Follow the
   pattern of `hosts/host-web/src/tests.rs:2351`.
   *Test value: it turns red if the C ABI designs, addresses or publishes a target differently from
   the browser's lane.*
3. **All or nothing.** A transaction with an EQ edit and a fader edit on a track whose fader queue
   is full returns `BACKPRESSURE`. The EQ owner's `committed_revision` and its candidate state are
   unchanged, and a later retry succeeds.
   *Test value: it turns red if a refused transaction leaves an EQ owner begun or published.*
4. **A small queue.** With `maximum_automation_spans_per_block` set to 4, an edit that designs
   more than 4 targets takes the structural path (a new epoch) and succeeds; an edit that designs
   at most 4 stays live.
   *Test value: it turns red if an EQ edit that can never fit returns `BACKPRESSURE` forever.*
5. **Readback.** As #1264's gate 3, for an EQ parameter.
6. **Nothing else changes.** #1257's gates 6 and 7. 4-lane (NEON) is CI-only here.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.

## Dependencies

- *Apply value-only effect parameter edits to the running C ABI plan* (#1264)

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, on `fbb311dfe`)

**Changes.**
- D1 (`crates/host-core/src/live_delta.rs`): for an instance whose factory has a prepared-target
  capability, a changed `Block` value no longer gives `Prepared`. The changed values stay
  `EffectControlRecord::Parameter` records (the owner's edits, never pushed), and the classifier
  designs the targets with `EqTargetPreparer` at `next.sample_rate_hz`, seeded with the
  pre-commit values from `resolve_initial_values` (canonical row order) and the changed
  `(parameter_id, channel, value)` edits. `LiveEffectRecords` gains `targets:
  Option<Vec<PreparedEffectTarget>>` (`Some` only for the EQ). A capability `EqTargetPreparer`
  refuses at construction (any non-60-row one) is `Prepared`; a design refusal is `Domain`. A
  band's `enabled` and `kind` (`automation_rate` `None`) stay `Prepared`, by the existing test.
- D2 (`crates/capi/src/runtime/control.rs`, `commit_live`): after the producer resolution, an
  instance whose queued count (records, or an EQ's targets) exceeds its queue's `capacity()`
  hands the token back for the rebuild; an EQ instance without an owner is `Internal`. Then, for
  each EQ instance, `begin_owner(committed_revision())`, `edit_owner` per edit and
  `preflight_candidate_targets` (`Capacity` is `LiveBackpressure`, anything else `Internal`);
  then the strip and non-EQ effect room checks, the non-EQ preflights, every readback handle and
  the protocol predicate. The begun owners live in an `OpenOwners` guard whose `Drop` discards
  every owner still begun, so every refusal path after the first `begin_owner` (including the
  test fault before the push) discards them. Then the pushes, each owner's
  `publish_candidate_targets`, the protocol commit and each `commit_owner`, each `expect`ed.
- D3: the readback lookup and update cover the EQ's edits like any other record.
- D4 (`docs/C_ABI_V1_QUALIFICATION.md`): a new "Value-only parametric EQ edits" section; the
  #1263 and #1264 sections point at it.
- Test support: `TestTransactionSnapshot` gains `effect_owners` (epoch, strip, instance,
  committed revision, phase, committed and candidate value bits); test-only, in `control.rs`.
  `live_tests.rs`'s `Rig` gains `with_limits`.
- The browser module is unchanged: `live_delta` is compiled only with host-core's
  `control-provider` feature, which only capi enables, so the worklet chain was not run.

**Tests and their value.**
- Gate 1 (`crates/host-core/tests/live_delta.rs`, on #1264's model with the EQ insert's band 1
  shaped away from the defaults):
  - `an_eq_band_gain_change_carries_its_edits_and_designed_targets`: red if the classifier seeds
    the designer with values other than the ones preparation gave the owner (the seeds are read
    from the owner of a host-core plan prepared from the same model), designs at another rate, or
    passes other edits.
  - `an_eq_hpf_enable_change_carries_targets`: red if the HPF's live `enabled` is refused or
    carries no targets.
  - `prepared_parameter_changes_need_a_rebuild` (rewritten): the EQ band-gain case is replaced by
    band `enabled` and `kind` changes, each beside a live gain. Red if a prepared-only EQ
    parameter goes live.
  - `an_out_of_domain_eq_q_needs_a_rebuild`: red if an EQ Q above its domain is admitted live.
- Gate 2 (`live_eq_parameter_edits_render_like_the_browsers_lane`, capi): band 1 gain (left) on
  eq0's console EQ, band 1 Q (both) on eq4's, and HPF enabled + frequency on an EQ insert of the
  last track, through `miso_engine_v1_submit_command`, against a `HostLiveLanes::ALL` host-core
  plan whose EQ owners publish targets that the test designs with `EqTargetPreparer` from each
  owner's committed rows, at the same block; bit-identical for 12 blocks on the nine-track
  fixture at the four launch rates, with no rebuild, and audibly different from an unedited plan.
  Red if the C ABI designs, addresses or publishes a target differently from the browser's lane.
- Gate 3 (`a_refused_eq_transaction_leaves_its_owner_idle`): an EQ edit beside a fader edit on a
  track whose fader lane is full is `BACKPRESSURE` after the EQ owner was begun and preflighted;
  the model, revision, replay, reliable lane, every owner's revision, phase, committed and
  candidate rows, and the EQ lane's room are unchanged, and the retry after a render commits
  (owner revision 1, `Idle`). Then sixteen one-target edits fill the EQ lane and the seventeenth
  is `BACKPRESSURE` with `control.live.backpressure`, changing nothing. Red if a refused
  transaction leaves an EQ owner begun or published.
- Gate 4 (`an_eq_edit_designing_more_targets_than_its_queue_rebuilds`): with
  `maximum_automation_spans_per_block` 4 (EQ lane capacity 4), band gains on bands 1-3 design
  4 targets (checked against `EqTargetPreparer` directly) and stay live, filling the lane; bands
  1-4 design 5 and commit through the structural path, with the rendering plan's lane and owners
  untouched. Red if an EQ edit that can never fit returns `BACKPRESSURE` forever.
- Gate 5 (`the_eq_parameter_readback_after_a_live_edit_equals_a_rebuilds`): a console EQ set, an
  insert HPF frequency set and an insert gain removal back to the default, live on one rig and,
  with a source content edit, structural on another; every metadata row and state record is
  identical and differs from before. Red if the readback keeps an EQ value's old state.

**Mutation runs** (each introduced, run, reverted; logs `/tmp/claude-1002/w1265-a1/M*.log`):
- M1, the classifier seeds the designer with the defaults: gate 1(a) red; capi gates 2-5 red (the
  owner's preflight refuses the targets: `INTERNAL`).
- M2, a target-capable effect's parameters skip the `automatable`/`Block` check:
  `prepared_parameter_changes_need_a_rebuild` red (the designer then refuses band `enabled` as
  `Domain`).
- M3, the classifier designs at 48 kHz whatever the session's rate: gate 2 red.
- M4, `OpenOwners` discards nothing: gate 3 red (the owner state differs after the refusal).
- M5, capi counts an EQ's records, not its targets, against the capacity: gate 4 red.
- M6, an owner preflight's `Capacity` maps to `INTERNAL`: gate 3 red.
- M7, capi skips the readback update when an EQ is in the delta: gate 5 red.
- M8, capi skips `commit_owner`: gate 3 red (the owner stays `Published` at revision 0).

**Gates** (from the committed tree; logs `/tmp/claude-1002/w1265-a1/gate-*.log`):
- `cargo fmt --all -- --check`: pass.
- `cargo test --locked -p capi`: pass (58 lib, 13 `resource_lifecycle`, doc).
- `cargo test --locked -p host-core --features control-provider,test-support`: pass (`live_delta`
  25).
- `cargo test --locked -p protocol --features test-support`: pass.
- The workspace test command of #1257 gate 6: pass (1,349 tests, 0 failed).
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: pass.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
  `./target/release/audit capi`: allocations 0, deallocations 0, locks 0, syscalls 0,
  total_violations 0.
- `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`: pass.
- `python3 -B scripts/check-scalar-oracle-absent.py --native target/release/libcapi.so`: pass.
- `for x in host-core realtime workspace protocol-control; do ...check/test...; done`: pass.
- `bash scripts/check-cross-targets.sh`: PASS (iOS `memset_pattern16` expected failures as
  recorded by #1018).
- Worklet chain: not run; no line compiled into the browser module changed (see Changes).
- 4-lane (NEON), `bash scripts/run-aarch64-tests.sh debug`: not run locally (CI-only).

**Open items for review.**
- Order inside D2: the owner transactions run right after the producer resolution and before the
  other room checks, as D2 lists them, so a fader-lane refusal is exercised after an owner was
  begun (gate 3's first case).
- `LiveEffectRecords` is still reachable only as `host_core::live_delta::LiveEffectRecords`
  (`host-core/src/lib.rs` is outside the authorized paths).
- The C header's "Live edits" comment and `docs/CONTROL_PROTOCOL_SEMANTICS.md:15` (outside the
  authorized paths) still name only fader, mute and pan, as #1264 noted.
