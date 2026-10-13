FAIL

# #1394 attempt 1 -- adversarial verdict (fresh verifier)

Range `adf234b19..5abf225f2` (7 commits) on `codex/d15-stream-e2`, exported to
`/tmp/claude-1002/v1394/{tree,mut}` and built with private target dirs. Spec:
`.github/ISSUE_SPECS/1394-carry-an-optional-per-edit-ramp-length-on-live-session-edits.md`.

Every gate is green. The attempt fails on one MAJOR finding: a stale ramp length. The fix is
small and bounded.

## MAJOR

**M1. A rack edit that rewrites an effect leaves the earlier edit's ramp length in place, against
D3's own rule. The Attempt record says this cannot happen, but it does.**
`crates/protocol/src/model.rs:563-631` (`record_edit_ramps`). D3's general rule is "the last edit in
the transaction that writes a `(target, row)` decides it". D3 also gives the reason upserts clear
their lengths: "because they rewrite those rows with no length". `PutTrackEffect`
(`model.rs:733-749`, which replaces an effect that has the same ID) and `SetTrackRack`
(`model.rs:726-732`, which replaces the whole rack) both rewrite the effect's `link_mode`, and
neither carries a length. `record_edit_ramps` sends both to `_ => {}`, so an earlier
`SetEffectLinkMode` length survives them. The same is true of `RemoveTrackEffect` followed by
`PutTrackEffect` under the same ID.

My probe (`/home/bl/misofm/submix-verdicts/1394-attempt1-probe.rs`) ran this transaction on the canonical
fixture: `SetEffectLinkMode{vocal, inserts, eq, Maximum, ramp_samples: Some(37)}`, then
`PutTrackEffect{vocal, inserts, eq, link_mode: DualMono}`. The put writes the effect last, with no
length. The prepared transaction still holds `(Effect{vocal, inserts, eq}, Link) = Some(37)`.

Attempt-record deviation 9 says "a stale entry for a removed target is unreachable: ... the upsert
that re-creates the ID clears it". That is false for an effect, because a rack edit re-creates an
effect without an upsert. For strips, routes and VCAs the claim holds: `UpsertTrack`, `UpsertSubmix`,
`UpsertRoute` and `UpsertVca` are the only ways to re-create those IDs, and each one clears.

No classifier path reads this today, because a link-mode change is `LiveRebuild::Structure`, so no
sound changes on this base. But the spec says the Link row's own slice "applies D6" through this
same lookup. That slice would then ramp a link change over a length that the last writer did not
give. That is a shortcut a later issue must find and fix, which the owner principle forbids.

Fix, in this issue:
- `PutTrackEffect` clears its `Effect{strip, rack, effect.id}` target.
- `SetTrackRack` clears every `Effect{strip, rack, _}` entry of that strip and rack.
- `RemoveTrackEffect` clears its target, so that removal and re-creation stay symmetrical.
- `SetEffectIdentity` can stay as it is, because it does not write `link_mode`.
- Extend gate 2, or the D3 table test, with a set-then-put case and a set-then-SetTrackRack case.
- Correct deviation 9.

`EditRamps::clear_target` already handles an `Effect` target (it has its own unit test, which has
no production caller today), so the code change is a few lines. If root prefers to amend D3
instead, the amendment must name these edits. The Link-row slice must not inherit the gap.

## MINOR

**m1. Nothing tests the wire diagnostic `session.edit.ramp_out_of_range`.**
`crates/protocol/src/controller.rs:3538`. Gate 3 says the transaction "refuses with
`session.edit.ramp_out_of_range` at that edit's index". The test (`tests/edit_ramps.rs`) checks only
the typed `SessionStoreError::Edit{operation_index, RampOutOfRange}`. I changed the mapping to
`"session.edit.not_found"`, ran the whole of `cargo test -p protocol --features test-support`, and
it stayed **GREEN**. The `capi` and `conformance` suites do not reach it either, because the code
string appears only in `controller.rs` and in prose. Add one controller-level or C ABI assertion of
the code and its `edits[i]` path. For example, add a row next to the conformance corpus's refused-code
rows (`protocol_corpus.rs:451`), or a `controller/tests.rs` case beside
`validation_diagnostics_preserve_operation_paths_and_exact_omission_count`.

## NIT

- **n1.** `crates/session/src/edit_ramp.rs`:
  `a_later_set_without_a_length_removes_the_entry_and_an_explicit_zero_is_an_entry` has no unique
  catch. Gate 2 already turns red if a later `None` does not remove an entry, and gates 2 and 4
  already turn red if an explicit 0 is not stored. The test lives in the crate that owns the type,
  so this is a judgment call, but it does not meet the test-value rule as written.
- **n2.** `edit_ramp.rs:68-80`: `from_code` maps every unknown code to `VcaMute`, and `entries()`
  (`filter_map` with `?`) silently drops an unknown rack code. Both are unreachable, because every
  code comes from `code()` or `RackName::wire()`. An exhaustive match, or an `unreachable!`, would
  state that.
- **n3.** The spec's gate-7 command `cargo test -p host-core --features test-support --test
  live_delta` runs 0 tests without `control-provider`. The implementer recorded this, and it was
  already true under #1054. CI's `test-debug-a` leg runs all 34 tests through feature unification
  (I confirmed this in its log). Fix the spec command at the next spec edit.
- **n4.** Informational. `EditRamps`'s heap (B-tree nodes and the cloned `StableId`s) lives in the
  prepared token, and the C ABI resource rows (`compile.rs:280-283`) do not charge it. The heap lasts
  only for one submit call, which is the same class as the classifier's uncharged control-thread
  clones (#369 precedent). It is not a contract breach. Only the inline 24 bytes are counted.

## The questions the caller raised

- **(a) D6 scope: confirmed.** On this base, `classify_live_delta` returns `LiveRebuild::Vca`
  whenever either model has a VCA. It compares routes and every submix field through the masked
  canonical JSON, so any change to them is `Structure`. It produces only `FaderDb`, `Mute` and the
  strip matrix record, plus `EffectControlRecord::Bypass` and `Parameter`, which carry no length:
  the bypass is a #1341 row and parameter ramps come from the descriptor, both non-goals. No send,
  follow or VCA record exists. All three record kinds that carry a length read it through
  `LiveRamps::resolve`. No direct `LiveRamps` field read is left (checked with grep; `for_row` is
  called only inside `resolve`).
- **(b) D4 bound: correct.** `prepare_transaction` (`model.rs:1100-1135`) applies every edit,
  compiles, then checks every edit against the compiled model's `sample_rate_hz`, before it builds
  `PreparedSessionTransaction`. On `Err`, `controller.rs:1766-1775` returns
  `StructuralCommandDisposition::Immediate`, before anything is reserved, pushed or committed:
  nothing is reserved, pushed to a queue, written to a cell or committed. `apply_transaction`
  commits only on `Ok`. Gate 3 shows that the revision and the snapshot are unchanged. Edit errors
  and compile diagnostics come before the ramp refusal; the spec does not fix that order, and the
  order is reasonable.
- **(c) Accepted.** Gate 1 adds the three corpus-missing edits next to the 64-opcode corpus and
  asserts that there are 11. Gates 2 and 3 are in the authorized `crates/protocol/tests/`.
- **(d) Re-pin: confirmed.** `ab357b6c432f9755` survives only in the three history lines.
  `5429f052592101ef` is the same in `protocol_corpus.rs:754`,
  `scripts/check-protocol-wasm-parity.sh:172-173`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and
  `fuzz/corpus/complete-schema-manifest.md:10`. The fuzz seed for session transactions is
  `set_session_id.hex`, which does not change. `conformance_corpus` and the Wasm parity script
  (simd128) both pass. The corpus moves only through `SetTrackFader` `Some(480)`. Every other edit
  is `None`, which encodes to bytes identical to the old encoding (the encoder emits the field only
  for `Some`).
- **(e) Not unreachable.** See M1. Effects re-created by `PutTrackEffect` or `SetTrackRack` keep
  the stale length. Strips, routes and VCAs are safe.
- **(f) Accepted.** The out-of-path files (`launch_registry_once.rs`, `capi/src/runtime/tests.rs`,
  `capi/tests/{design_bound_cache,plan_swap_race,resource_lifecycle}.rs`, `tools/audit/src/capi.rs`,
  and the four `protocol/tests/` files) change only by `ramp_samples: None` and two
  `&EditRamps::new()` arguments. A diff count shows that no other line changed.
- **(g) Confirmed: 816 -> 840 bytes (+24).** I built and measured both commits.
  `size_of::<EditRamps>() == 24`. No test pins the value: `resource_lifecycle.rs:762` computes it
  with `size_of`. The record says "the baseline was not built separately"; the baseline is 816.
- **Acked-batch question: no.** No queue, cell or admission path changes. Records carry only a
  different length, and the render setters accept any `u32` window. The refusal happens in
  preparation, before any push or ack. No ack can come before a drop.
- **Does the length leak into model state or canonical JSON? No.** `apply_session_edit` ignores the
  field (`..` on every arm). `EditRamps` lives only in `PreparedSessionTransaction` and is dropped
  by `commit_prepared` (`model.rs:1150`). `classify_live_delta` has one caller, `commit_live`, which
  reads `transaction.edit_ramps()` and nothing else. Gate 6 shows that the next transaction without
  a length settles over 480 frames, not 96.
- No new producer, fan or listener role wording, and nothing that depends on decision 16 (grep of
  the added lines).

## Test value, one sentence per new test

- `session_wire/tests.rs` `live_value_edits_round_trip_their_optional_ramp_length` (gate 1): red if
  the encoder drops an explicit 0 or disagrees with the decoder on the field, a defect that no
  existing round-trip catches because they all carry `None`. I applied "encoder emits nothing for
  `Some(0)`": RED here only. "Absent decodes as `Some(0)`" is also caught by the updated existing
  round-trips.
- `protocol/tests/edit_ramps.rs` `the_last_edit_to_write_a_row_decides_its_length` (gate 2): red if
  an upsert leaves a stale strip entry, a later `None` keeps an earlier length, or a fader edit
  records too few rows. I applied "`UpsertTrack` does not clear": RED.
- same file, `every_live_value_edit_records_its_rows_on_its_target`: red if an edit records the
  wrong row or target. This is the only place the D3 table is pinned. I applied
  "`SetRouteFollowsMute` writes `RouteGain`": RED.
- same file, `a_length_above_one_second_of_the_committed_rate_refuses_the_whole_transaction`
  (gate 3): red on an off-by-one, a bound taken from the base rate, a last-edit-only check, or a
  refused transaction that commits. I applied `>` -> `>=`: RED. The diagnostic code is not covered
  (m1).
- `session/src/edit_ramp.rs` `clearing_a_strip_drops_its_effects_and_nothing_else`: red if a strip
  clear leaves its effect entries, or takes another strip's or a same-ID route's entries. No
  protocol test sets effect entries before an upsert.
- `session/src/edit_ramp.rs` `clearing_one_effect_leaves_its_siblings_and_its_strip`: red if an
  effect clear takes its siblings or its strip. It defends the API that the M1 fix will use.
- `session/src/edit_ramp.rs` `a_later_set_without_a_length_...`: no unique catch (n1).
- `host-core/tests/live_delta.rs` `a_records_length_is_its_edits_own_when_the_edit_carried_one`
  (gate 4): red if the model's pan window beats the edit's length, or if `resolve` reads an
  explicit 0 as the default. I applied both: RED each.
- `capi/src/runtime/live_tests.rs`
  `a_live_edits_own_ramp_length_overrides_the_session_and_does_not_outlive_its_transaction`
  (gate 6): red if `commit_live` passes no edit ramps. Gate 4 cannot see this because it calls the
  classifier directly. I applied "`commit_live` passes `&EditRamps::new()`": RED.
- The rewrite of the existing `ramps_of_a_move` helper does not weaken it. Its callers assert exact
  `[fader, mute, matrix]` values.

## Gates run (all on `5abf225f2`, private `CARGO_TARGET_DIR`)

- **The whole `lint` job set of `qualification.yml`: all PASS.** This covers fmt, clippy
  `-D warnings`, rustdoc `-D warnings`, the workspace, session, bench, host-core, protocol-control,
  realtime, realtime-audit-leak, artifact-evidence-leak, lane, rack, builtins, graph and
  effect-runtime policy checks with their mutation tests, `check-test-support-ci` and its
  mutations, env vocabulary, the scalar-oracle self-test, the console-benchmark fixture, the
  builtins-fixture mutations, bench preconditions, the trace validator, the unfused seal,
  conformance boundaries, the PEQ seal, the release-shape self-test, npm publish modes, the stem
  store, stem identity, and the three x86 lane-guard probes. Script reachability needs git, so I ran
  it in a git-initialized copy of the same export: ok, 120 files and 20 mutation cases.
- **The whole `test-release` job set: all PASS.** lane/math/wasm-gates (133), filter_liveness (15),
  tail_contract (25), ramp_endpoint (16), designer_total (1), M3 FMA cfg, and the loom leg (13). M1
  and F1 are gated by `math_closure`, and this range does not touch `math`.
- **Spec gates: all PASS.**
  - `-p session` (96); `-p protocol --features test-support` (151, of which lib 132 and
    edit_ramps 3); `-p host-core --features test-support,control-provider --test live_delta` (34);
    `-p capi` (105); `-p conformance` (24).
  - `test-debug-a` (1568 passed, 133 suites, with live_delta 34 and edit_ramps 3 inside it) and its
    doctests (24).
  - `audit capi`: 0 allocations, 0 syscalls, `pcm_digest a9738fa800afcab3`.
  - `run-protocol-allocation-audit.sh`; `check-builtins-fixtures.sh`; `check-capi-abi.sh` (ok,
    shared and static; it hardcodes `target/release`, so I ran it through a link to my target);
    `check-protocol-wasm-parity.sh` (ok, simd128); `check-cross-targets.sh`.
- **Worklet chain: all PASS.** `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` (32 red mutations, every budget met),
  `check-scalar-oracle-absent.py --wasm`, and `test-web-audioworklet.sh` under a private TMPDIR
  (nothing left behind).
- Not run: AArch64 legs (CI only), the three-browser matrix, the V8 spill gate (it needs exact
  Node 22.23.2).
