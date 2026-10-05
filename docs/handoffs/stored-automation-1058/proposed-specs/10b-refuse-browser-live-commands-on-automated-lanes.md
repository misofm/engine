# Refuse browser live commands on automated fader lanes

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A2, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

In the browser, a live fader command on a lane whose fader the session automates is refused with a
new typed reason, `automated`, and nothing from its batch is admitted. Without the refusal the
command would be acked and then overwritten by the next grid event of the stored curve. A mute,
solo or VCA mute on that lane is still admitted: the lane's mute is not automated. A VCA fader
move that reaches the lane is draft 11's, which lands in the same push and composes it with the
curve. The refusal
stays until *Admit browser live edits in the Worker through the committed model* (#1382) replaces
the browser's admission.

## Context

- **Browser admission.** `admit_commands` (`hosts/host-web/src/lib.rs:4596-4637`) stages a whole
  batch and rolls every shadow back on a refusal (`:4623-4636`). The kinds are
  `COMMAND_FADER_DB = 3`, `COMMAND_MUTE = 4`, `COMMAND_SOLO = 9`, `COMMAND_VCA_FADER_DB = 16` and
  `COMMAND_VCA_MUTE = 17` (`:831`, `:833`, `:869`, `:931`, `:939`). A VCA fader move stages
  `FaderDb` records on every reached member (`:5185-5214`). The reasons end at
  `COMMAND_REASON_UNKNOWN_VCA = 14` (`:1076`); `RESULT_UNSUPPORTED` is chosen for
  `COMMAND_REASON_UNSUPPORTED_KIND` in the batch's `refuse` mapping (`:4668-4680`).
- **The reason vocabulary** is spelled in six sources that
  `scripts/check-command-reason-vocabulary.py` compares (its docstring lists them), and in the
  generated SDK and layout files (`sdk/src/generated/abi.ts`, `sdk/src/generated/catalog.ts`,
  `sdk/src/core/live-controls.ts`, `sdk/assets/miso-engine-v1-parameter-metadata.json`,
  `sdk/assets/miso-engine-v1-abi-layout.json`, `tools/parameter-metadata/src/abi_layout.rs`,
  `scripts/check-abi-layout-v1.py`, and the self-test fixtures under `scripts/fixtures/`).
- **Producers.** `TrackControlProducer` (`crates/builtins-compiler/src/lib.rs:254-270`) is the one
  per-strip control object both hosts get.
- **The predicate.** Draft 10a D3's `automated_cell(model, address)` says which lanes the session
  automates, from the same enumeration draft 09a's preparation compiles cells from.
- **Fader events on a muted lane** only remember the gain (draft 08 D2), so a live mute and a stored
  fader curve compose.

## Decisions frozen for this slice

- **D1. The lane mask.** Preparation records on each `TrackControlProducer` an
  `automated_fader: [bool; 2]` lane mask, from draft 10a's predicate. Later slices add their rows'
  masks beside it.
- **D2. The refusal.** `admit_commands` refuses, at the record's index, with `RESULT_UNSUPPORTED`
  and a new `COMMAND_REASON_AUTOMATED` (camelCase `automated`), a `COMMAND_FADER_DB` whose channel
  covers an automated lane (a `Both` command on a strip with one automated lane included). Nothing
  from the batch is admitted (the existing rollback). The reason takes the next free reason value
  when this slice merges (15 on `6ee64f484`), never a reused one; code and tests name it by its
  symbol. `COMMAND_VCA_FADER_DB` is not refused here: draft 11 owns VCA behaviour on an automated
  member and lands in the same push.
- **D3. Still admitted.** `COMMAND_MUTE`, `COMMAND_SOLO` and `COMMAND_VCA_MUTE` on an automated
  fader lane: the lane's mute is not automated, and the C ABI takes the same mute as a live record.
- **D4. The reason.** `COMMAND_REASON_AUTOMATED` gets a doc comment beside `:1076`: the target is
  driven by stored automation, so a live value would be overwritten; the caller edits the
  automation instead. It is added to every file that spells the vocabulary, in the same order.
- **D5. The acked-batch question: can an ack ever precede a drop? No.** The refusal comes before
  any admission, and the batch is all or nothing.

## Deliverables

1. D1 in `builtins-compiler` and host-core preparation.
2. D2-D4 in host-web and every vocabulary file.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the producer's lane mask only), `crates/host-core/src/prepare.rs`
  (filling the mask only)
- `hosts/host-web/src/lib.rs` (admission and the reason constant only), `hosts/host-web/src/tests.rs`
- Every file that spells `COMMAND_REASON_*` or `unknownVca` today (the new entry only), and the
  generated SDK files the build regenerates

## Non-goals

- The C ABI classification and carry (draft 10a). A VCA fader move (kind 16) on an automated member
  (draft 11, same push).
- Refusals for other rows: drafts 13a, 14a, 15 and 16a add theirs with this reason.
- A browser transaction path (#1382).

## Hazards

- **Vocabulary drift.** A reason added to the Rust constants alone fails the vocabulary gate;
  regenerate the SDK files with the build, never by hand.

## Objective gates

1. **Refusal** (`hosts/host-web/src/tests.rs`, new). Fader ride on track 0's left lane. A batch
   `[mute track 1, fader track 0 left]` returns `RESULT_UNSUPPORTED` with
   `COMMAND_REASON_AUTOMATED` at index 1, and the mute is not heard. A `Both` fader command on
   track 0 is refused; a right-lane fader command on track 0 and a fader command on track 1 are
   admitted.
2. **Mute composes** (same file). A mute on track 0 is admitted, and after the ramp the output equals
   a plan prepared muted with the same ride.
3. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`
   - `python3 -B scripts/check-command-reason-vocabulary.py`, `bash scripts/check-web-audioworklet.sh`,
     `bash scripts/test-web-audioworklet.sh`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
4. **No rendered bit moves** for a session with no stored automation: the browser legs of the
   `browser` job in `.github/workflows/qualification.yml` pass with unchanged digests.

## Test value

- Gate 1: red if the browser admits a fader move that the next grid event would overwrite, misses a
  `Both` command that covers an automated lane, refuses a lane the session does not automate, or
  admits part of a refused batch.
- Gate 2: red if the refusal also blocks the mute, which the session does not automate.

## Dependencies

- Draft 10a *Classify fader automation edits as carried rebuilds* (the predicate; it brings drafts
  09a and 09b).
- *Refuse commands that would be acknowledged with no effect* (#1315), whose D4 is the model for an
  all-or-nothing typed refusal, and which may take reason values first.
- Batch: R1, in one push with draft 11 *Compose VCA offsets with stored fader automation*.
