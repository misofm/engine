# #1241 attempt 1 verdict: Edit VCA groups through session transactions

**Verdict: PASS.** There is no BLOCKER and no MAJOR. The wire, the three opcodes, the apply arms,
the count and hash re-pins, and the docs all match D1 and D2 exactly. Every spec gate re-ran green
on the commit. A probe shows that a VCA transaction on the C ABI commits through the same
`SessionStore`, and that the replacement plan carries the result. There are two MINORs: two legs
of D2's final validation (removing a **submix** member, and the fader **range**) are stated in the
docs but no test covers them. Both have one-case fixes. There are also three NITs.

- **Implementation:** `210bd252f` on parent `250b72e94`, branch `codex/batch-vca`.
- **Review copies:** `git archive` exports of `210bd252f` (gates, plus a separate mutation copy)
  and `250b72e94` under `/tmp/claude-1002/v1241/`, deleted after review. I never touched the
  worktree, which the #1243 implementer is editing concurrently.
- **Host:** x86-64-v3 (AVX2).
- **Probes:** `1241-attempt1-verifier-scratch.rs`, next to this file.

## Gates re-run on `210bd252f` (all exit 0)

- **Gate 3, DSP crates:** the spec's `cargo test --all-targets` command passes 791 tests over 146
  binaries, with 0 failed and 24 ignored. It includes `conformance_corpus.rs` (46 frames, the
  re-pinned hash) and the 10,000-mutation closed-dispatch row.
- **Gate 3, fixtures:** `cargo run -p conformance --example conformance_fixtures -- --check`.
- **Gate 3, Wasm parity:** `check-protocol-wasm-parity.sh` reports "ok (simd128)". `--self-test`
  passed, with 1 inert-invocation row and 3 red rebuilds. Deviation 2 (the script has no scalar
  arm, because scalar Wasm is refused) is accepted.
- **Gate 4, workspace:** the spec's `cargo test` command passes 1,263 tests over 113 binaries, with
  0 failed and 9 ignored. All five new or rewritten tests ran and passed.
- **Gate 4, hygiene:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets
  --all-features -D warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`.
- **Gate 4, policy:** the protocol-control and workspace policy checks and their self-tests.
- **Gate 4, cross-targets:** `check-cross-targets.sh` reports "cross-target matrix: PASS".
  `protocol` has no `ios-asm-memset-pattern16` row, so the new code stores no splatted constant.
  The expected-failure rows (#1018) are unchanged, and host-core is still at 4 calls.
- **Gate 4, aarch64:** `run-aarch64-tests.sh debug` waits for CI's `aarch64-debug` at the batch
  push, because this host is x86-64. That is as the spec allows.

## Adversarial checks

**Wire.** `schema::session::vca` is `ID` 1 (required `Utf8`), `FADER` 2 (the `fader` message) and
`MEMBERS` 3. `MEMBERS` is the same hand-built `FieldSpec` as `set_track_effect_order::EFFECT_ID`
(`mandatory: true, repeated: true`).

- `upsert_vca` is `VALUE` 1, `remove_vca` is `ID` 1, and `set_vca_fader` is `VCA_ID` 1 plus
  `VALUE` 2. They mirror `upsert_submix`, `remove_submix` and `set_track_fader` field for field.
- `payload_spec` has the three rows.
- `tx_vca` counts members through `field_count`. `parse_vca` decodes through `values_spec!` and
  then `stable_id`, so zero occurrences decode to `[]`.
- The corpus's `UpsertVca` spells its members out of order (`[vocal, drums]`), and the codec keeps
  that order.

**Opcodes.** `0x0700`-`0x0702` are new in the enum, `from_raw`, `SessionEdit`, `opcode()` and
`apply_session_edit`. No existing code moved, and `0006`, `0102` and `0104` still return `None`.
No other crate matches `SessionEdit` exhaustively or with a wildcard. The SDK has no opcode table,
and the spec's non-goal (the SDK writes whole sessions) holds.

**Re-pins are legitimate.** I dumped every frame's label, length and FNV-1a-64 on both the parent
and the commit:

- exactly **one** frame differs: `command.session_transaction_apply`, from 7,744 to 8,168 bytes;
- the first 43 fixture edits are identical;
- the only change is the three appended edits (`0700`, `0701`, `0702`).

The hash moved from `0x95c1_ceb6_8e44_f6e2` to `0xab35_7b6c_432f_9755`, and it is spelled the same
at all four sites. The count of 46 and the edit limit of 45 are consistent at every pin, and the
frame count stays 46. No stale "43 edits" or old-hash spelling survives outside the history
sentences.

**Atomicity.** I read `prepare_transaction`. It clones the normalized model, applies every edit,
and then runs `compile_session` once on the final candidate. A failure returns before
`commit_prepared`. The implementer's refusal table covers 10 cases, each opened by a `0702` that
would commit on its own. Each case asserts that the revision and the snapshot did not change:

- `0701` and `0702` at an unknown ID;
- `0203`, `020f`, `0210` and `0211` at a VCA ID;
- a dangling track member;
- removing a nested VCA that its parent still lists;
- a cycle (`vca.cycle` at both paths);
- a `0700` under a track's ID (`id.duplicate`).

The positive control (removing `x` while rewriting `grp`) commits. **There is no cascade**:
`RemoveVca` uses the generic `remove`, and the removal of a member that is still listed is refused,
as D2 requires.

**Canonical snapshot.** After each committed step, the snapshot equals `canonical_session_json` of
the model written directly, and it round-trips. The VCAs are sorted by ID and their members by ID.
The forward reference (the parent is upserted before its nested child) commits, which meets the
Hazard.

**C ABI (probe 1).** I fed `0700`, then `020f` at the VCA ID, then `0702`, through the C ABI's
`SessionState::command` (the C ABI's own `ProtocolController`/`SessionStore`). Every commit is
structural, as the attempt record says; #1053 has not landed.

- `0700` commits at revision 43, and the controller's snapshot carries the VCA.
- `020f` at the VCA ID answers `session.edit.not_found`, and the revision and the snapshot are
  unchanged.
- `0702` commits at revision 44, and the re-prepared plan renders it:
  - at -6 dB, the peak goes from 0.42117 to 0.21109 (×0.5012);
  - with a mute, the output is exactly zero.

**The acked-batch question.** An ack cannot precede a drop here. A VCA edit travels only inside the
transaction, and the success response is encoded after `prepare_transaction` succeeds. On the C
ABI, it is published only after `commit_prepared_structural`. This slice adds no queue.

**A1 (the #1240 sort test).** `compile_session_normalizes_vcas_and_members_by_id` bites. When I
remove the `vcas` sort from `compile_session`, it turns red, and all protocol tests stay green. So
it is the only test that catches that defect.

## Mutations (mine, each in the mutation copy, then restored)

| # | Mutation | Result |
| --- | --- | --- |
| MA | `parse_vca` sorts `members` on decode | RED: the round-trip test, and `vca_edits_commit_and_snapshot_canonically` through `through_the_wire` |
| MB | `0702` at an unknown ID creates a VCA with no members instead of `NotFound` | RED: `vca_refusals_commit_nothing` (`expect_err`) |
| MC | `0702` writes only the left lane's dB and mute | RED: `vca_edits_commit_and_snapshot_canonically` |
| A1 | `compile_session` stops sorting `vcas` | RED: `compile_session_normalizes_vcas_and_members_by_id` only |
| ME | `RemoveSubmix` prunes the removed submix from every VCA's `members` (a cascade) | **survives**: all `protocol`, `capi` and `conformance` tests pass (MINOR-1) |
| MF | `0702` clamps finite dB to `[-144, 24]` instead of leaving it to final validation | **survives**: all `protocol` tests pass (MINOR-2) |

## Test value (one sentence each)

- `vca_edits_round_trip_and_members_decode_empty`: red if the decoder drops, re-tags, refuses when
  empty, or reorders VCA members (MA). The fixture corpus checks only one fixed VCA, through a
  hash.
- `vca_edits_commit_and_snapshot_canonically`: red if `0702` writes the wrong VCA or lane (MC), or
  if `0700` appends instead of replacing. No codec test applies an edit.
- `vca_refusals_commit_nothing`: red if `0702` at an unknown ID quietly creates or skips (MB), if a
  strip edit resolves a VCA, or if a removal cascades into its parents.
- `opcode_registry_appends_the_console_edits_and_reallocates_nothing` (rewritten): red if `0x0700`
  to `0x0702` are missing from `from_raw` or land on a retired code.
- `compile_session_normalizes_vcas_and_members_by_id` (A1): red if the `vcas` or `members` sort
  goes, which no other test catches (see above).

## MINOR-1: no test covers removing a submix that a VCA lists

D2 and the new registry row both say that removing "a VCA, track or submix that a VCA still lists
refuses the whole transaction with `reference.missing_entity`". The track leg and the VCA leg are
tested; the submix leg is not. A plausible cascade in `RemoveSubmix` (mutation ME) passes every
test. Removing a submix is exactly where an implementer might add one, by analogy with cleaning up
routes.

**Fix.** Add one row to `vca_refusals_commit_nothing`. It upserts `grp` over `["a", "bus", "x"]`
(or lists `bus` in the base VCA), then `RemoveRoute a-bus`, `RemoveRoute bus-out` and
`RemoveSubmix bus`. It expects `Validation(&[("reference.missing_entity",
"$.vcas[0].members[1]")])`, with the revision and the snapshot unchanged.

## MINOR-2: no test covers the range leg of "validated with the whole transaction"

The product outcome says each edit is validated for "namespace, references, acyclicity, range",
and `0702` is the edit that carries the range. No protocol test sends an out-of-range VCA fader. If
the apply arm clamped instead (mutation MF), an agent's +30 dB would silently commit as +24 dB with
an OK ack. The validation itself works: probe 2 shows that 30 dB and -200 dB give
`numeric.out_of_schema_range` at `$.vcas[0].fader.{left,right}_db`, NaN gives
`numeric.non_finite`, and the revision and the snapshot are unchanged.

**Fix.** Add one row: `0702` on `grp` with `left_db: 30.0`, expecting `Validation(&[(
"numeric.out_of_schema_range", "$.vcas[0].fader.left_db")])`.

## NITs

1. **No coverage test for the corpus.** No test asserts that `complete_all_opcode_fixture` holds
   exactly one edit per allocated opcode, in opcode order. Swapping its `SetVcaFader` for a second
   `RemoveVca` keeps the count at 46, and only the hash pin notices. This predates the slice. A
   successor could assert `fixture.iter().map(opcode) == allocated codes`.
2. **A cap on `0700` members.** One `0700` can carry at most about 1,022 members, because of
   `ProtocolLimits::max_tlv_count` (1,024 by default). That is a configured resource and the same
   bound as `SetTrackEffectOrder`, so it does not breach AGENTS.md. It is worth one sentence in the
   registry row, though, since membership has no incremental opcode.
3. **Strip-edit coverage.** The strip-edit refusal rows cover `0203`, `020f`, `0210` and `0211`,
   but not the rack-addressed `0204`-`020e`. They all resolve through the same `strip_mut`, so this
   is coverage breadth, not a gap in the mechanism.
