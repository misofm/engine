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
