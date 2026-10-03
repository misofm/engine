# #1204 *Address submix strips in session edits*: Sol verdict, attempt 1

- Reviewed: `git diff 6a4d729d9 7f767ae0c` (branch `codex/batch-submix-k1`, worktree
  `/home/bl/misofm/wt-submix-k1`). 5 files, +721/-48: `crates/protocol/src/model.rs`, the new
  `crates/protocol/tests/submix_strip_edits.rs`, `docs/CONTROL_PROTOCOL_REGISTRY.md`,
  `docs/C_ABI_V1_QUALIFICATION.md` and the spec. Every path is on the spec's authorized list.
- Binding: `AGENTS.md` (including the acked-batch question) and
  `.github/ISSUE_SPECS/1204-address-submix-strips-in-session-edits.md` with its Attempt 1 record.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1204` shows it OPEN, and
    its title matches the spec. That is correct in batch mode.
  - I exported `7f767ae0c` with `git archive` into `/tmp/claude-1002/v1204/src` for the gates, and
    into a second tree for mutations and scratch tests. Each tree had its own `CARGO_TARGET_DIR`.
    The gate tree was never edited.
  - After every mutation, `model.rs` was restored, and I checked it with `cmp`.
  - My scratch tests are saved in `submix-verdicts/1204-attempt1-verifier-scratch.rs`. They are
    evidence only, and I am not asking anyone to adopt them.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There is one MINOR finding (record accuracy) and three NITs. None
of them needs another attempt.

- **D1-D4 are implemented at the spec's sites.**
  - `strip_mut` searches tracks first, then submixes. Any other ID is `NotFound`.
  - `0203`, `020f`, `0210` and `0211` call it directly.
  - `rack_mut`'s `Inserts` arm and `knobs_mut`'s console branch call it, so `0204`-`020e` follow.
  - `rack_mut` still refuses `Console` (`ConsoleSlotFixed`) and `Builtins` (`NotFound`) before any
    lookup.
  - `track_mut` is left only for `0202`.
- **No wire identity changed.** No opcode, field or code changed. `complete_all_opcode_fixture` and
  `crates/conformance` are untouched, and the hash is unchanged.
- **Every gate passes when I re-run it, and the counts match Terra's record exactly.**

## Gates (re-run by me on `7f767ae0c`, x86-64-v3)

| Gate | Command | Result |
|---|---|---|
| 1-3 | `cargo test --locked -p protocol --features protocol/test-support` | exit 0. 141 passed (128 lib, 6, 1, 3, and 3 in the new file) |
| 4 | the test-debug-b command (with conformance), then `conformance_fixtures -- --check` | exit 0, 787 passed; then exit 0 |
| 4 | `bash scripts/check-protocol-wasm-parity.sh` | `issue-005 Wasm golden parity: ok (simd128)`, with the existing hash |
| 4 | `check-protocol-control-policy.sh`; `test-protocol-control-policy.sh` | ok; mutation tests ok |
| 4 | the test-debug-a workspace command (the exact excludes and features from `qualification.yml`) | exit 0. 99 binaries, 1143 passed, 0 failed |
| 4 | `cargo fmt --all -- --check` | exit 0 |
| 4 | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0, no warnings |
| extra | `check-workspace-policy.sh` | ok (in a non-git export, the script falls back to `find`) |
| extra | the new test file against `model.rs` at `6a4d729d9` | gates 1 and 3 are red; gate 2 is green, as Terra declared |

## Adversarial checks

1. **Atomicity of mixed track and submix batches (the acked-batch question).**
   - `prepare_transaction` applies the edits to a cloned candidate and returns on the first edit
     error or the final compile error. Commit is a move that cannot fail. #1204 does not touch
     this path; it only widens resolution.
   - Store-level scratch test, `sol_mixed_batch_failure_commits_nothing`:
     - The batch is `020f bus`, `020a bus console`, `0205 a inserts`, then a failing fourth edit.
     - The failing edit is one of: `020f` on an unknown ID; `0211 bus []`, which fails final
       validation with `console.entry_missing` at `$.submixes[0].console`; or `020e` naming a
       console parameter that does not exist.
     - Each is refused with the correct operation index. The revision and the snapshot are
       unchanged.
   - C ABI scratch test, `sol_mixed_failing_submix_batch_through_the_c_abi_acks_nothing`:
     - The batch is `020f bus`, `020f eq0`, then `0202 bus`.
     - The response is `NonOk` with `session.edit.not_found` at `edits[2]` (`operation_index` 2),
       and the header is still revision 42.
     - The reliable lane is empty: no `SessionCommitted`.
     - The valid two-edit prefix then commits at the same `Exact(42)`.
   - **No ack precedes a drop or a partial application.**
2. **The ID namespace, and whether gate 3 means anything.**
   - Scratch test `sol_track_and_submix_cannot_share_an_id_at_commit`:
     - `0300` upserting submix `a` beside track `a` is refused at final validation with
       `DuplicateId` (plus `MissingEntityReference` on the route that names `a`).
     - Upserting a submix named `main-out` is also refused with `DuplicateId`.
   - So the committed model never holds a shared ID. Inside a transaction a shared ID does occur,
     because `0300` and `0200` are plain upserts.
   - Gate 3's two orders observe exactly that window:
     - Tracks first gives submix `x` at 0 dB.
     - The mirror order gives -6 dB.
   - Gate 3 is meaningful. My mutation MC (refuse an ID that matches both lists) turns it red, and
     only it.
3. **`0202` stays track-only.** This is right per D2: a submix has no source assignment. `0202 bus`
   answers `NotFound`, in the store test and through the C ABI.
4. **Output IDs.** In my scratch test `sol_output_id_refuses_every_strip_opcode`, all 15 opcodes
   `0203`-`0211` addressed to `main-out` answer `Edit { 0, NotFound }`. That includes `020a` and
   `020d` on rack `console`, through `knobs_mut`. (See NIT-2 for the two exceptions the registry
   wording omits.)
5. **The console and builtins refusals.**
   - `0205` with rack `console` on `bus` answers `ConsoleSlotFixed`.
   - The refuse-before-lookup order (the spec's hazard) still holds. My mutation MA moves the
     lookup ahead of the check, and the existing
     `console_structural_and_declaration_edits_refuse_with_a_typed_status` (unknown track ID) turns
     red.
   - Rack `builtins` stays `NotFound` before any lookup.
6. **Snapshot and canonical JSON round trip.**
   - Gate 1 compares the full canonical snapshot with the base model, edited directly on `bus`
     only.
   - My scratch test `sol_snapshot_round_trips_after_every_submix_edit` checks each of the 15
     commits:
     - `parse_session_json(snapshot)` equals the normalized model;
     - re-canonicalizing that parse returns the identical bytes.
   - All pass.
7. **Do host-web and capi reach the same store logic?**
   - capi does. `command` calls `ProtocolController::prepare_command_frame`, which calls
     `SessionStore::prepare_transaction`, which calls `apply_session_edit`. The plan is then
     compiled from `prospective_session().compiled()`.
   - host-web does not reach it, and that is correct. It has no `SESSION_TRANSACTION_APPLY` or
     `SessionEdit` path at all: there is no `protocol` dependency, and only capi enables
     `host-core/control-provider`. Its live control records remain track-addressed, which is out of
     scope here (#1213, #1225).
   - I reproduced the C ABI round trip independently with scratch test
     `sol_submix_fader_edit_through_the_c_abi`:
     - Session: eq0 -> `bus` (`Submix::unity`) -> `main-out`, at 48 kHz.
     - The command is `020f bus -6 dB`. It returns `Success`, and `SessionCommitted` arrives at 43.
     - Blocks 0-7 are **bit-identical** to a plan compiled directly from the edited session, after
       pre-rolling that plan the same way to match `ResetAtReplacementBoundary`.
     - From block 3 on, the C output against the unedited plan is -6.000 dB (±0.003).
     - This confirms Terra's evidence.
8. **The deviation.** Terra changed nothing in `controller.rs`, because the error strings are
   unchanged. That is acceptable, since the authorized path was permissive, not mandatory.

## Test value (one sentence each)

- `every_strip_opcode_edits_a_submix`: red if any strip opcode still resolves a submix through the
  track lookup, or lands on the identical track strip. Terra's M2-M4 and M8 catch this, and so does
  my MD (`0210` reverted to `track_mut`). No other test addresses a submix.
- `strip_refusals_are_the_tracks`: red if `020f` silently skips an unresolved strip (M5), if `0202`
  silently skips an unknown track (M7), or if `strip_mut` falls back to a submix for an unmatched ID
  (my MB). I ran the protocol, conformance and capi suites with `--no-fail-fast`, and no other
  committed test catches any of the three.
- `a_strip_id_resolves_tracks_before_submixes`: red if `strip_mut` searches submixes first (M1),
  resolves tracks only (M2), or refuses or picks arbitrarily when both lists match (my MC). It is
  the only test that catches these.

## Findings

### MINOR-1: the gate-2 test-value line overclaims the console row

The record lists M6 (`rack_mut`'s `Console` arm returns the strip's inserts) as one of the
mutations "defending" gate 2.

M6 is real, but it is not unique to gate 2. With `--no-fail-fast` it also turns two existing tests
red:

- `console_structural_and_declaration_edits_refuse_with_a_typed_status`;
- `controller::tests::retired_and_console_refused_codes_meet_their_conformance_rows`.

The same applies to MA. The `0205`-console row defends only a submix-only special case. It stays
because the spec's gate 2 names it.

Terra's mutation runs used `--test submix_strip_edits` only. That shows a test is red, not that its
catch is unique.

**Fix (record only):** in the Attempt 1 record, say that gate 2's unique catches are M5, M7 and MB
(strip fallback for an unmatched ID), and that the console row duplicates existing coverage and is
kept by the spec.

### NIT-1: stale doc comments in `model.rs`

- `SessionEditError::NotFound`: "A targeted source, track, effect, route, or automation was absent".
  It should name a strip, track or submix.
- `ConsoleSlotFixed`: "A track cannot add, remove or reorder a console slot", and "a track's knobs
  through ...". Both should say "a strip".
- The opcode doc for `SetTrackConsole`: "appended to the track family".

**Fix:** reword these to "strip", in the same way as the variant docs.

### NIT-2: the registry's D2 wording is broader than the code

The registry says: "An output ID is `session.edit.not_found` for every strip opcode." There are two
exceptions, both by design:

- With rack `console`, the structural edits answer `console_slot_fixed` first (D3 states this).
- Inside one transaction, a submix upserted under an output's ID is resolved by a strip edit. The
  transaction is then refused at final validation with `DuplicateId`, so it is refused, but not
  with `not_found`.

**Fix:** add "(in the base model; rack `console` structural edits answer `console_slot_fixed` per
D3)".

### NIT-3: the Gate 1 and Gate 2 output-ID coverage is the minimum

Gate 2 covers an output ID only on `020f`. My scratch test runs all 15 opcodes, and they behave
correctly. I found no unique catch for the extra rows, so I am not asking for them to be added.
This is recorded only so nobody needs to re-check it.

## Scratch cleanup

I deleted `/tmp/claude-1002/v1204` (both exports and both target dirs) after writing this verdict.
The scratch tests are preserved in `submix-verdicts/1204-attempt1-verifier-scratch.rs`.
