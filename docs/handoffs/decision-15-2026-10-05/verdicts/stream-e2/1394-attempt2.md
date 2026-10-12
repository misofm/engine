PASS

# #1394 attempt 2 -- adversarial verdict (fresh verifier)

Attempt 2 is the single commit `bd8a92e9d` (parent `6e4254588`, which is #1364 attempt 1 and is not
under review). I also re-checked the whole #1394 change (`adf234b19..5abf225f2` plus `bd8a92e9d`) as
it stands at `bd8a92e9d`. I exported `bd8a92e9d` to `/tmp/claude-1002/v1394/tree` (gates) and
`/tmp/claude-1002/v1394/mut` (spec gates and mutations), with private target dirs.

M1 is fixed for every `SessionEdit` variant. m1 is fixed. Every gate is green. There is no BLOCKER
and no MAJOR. Three MINOR findings and three NITs follow; two of them are items that root must rule
on (an out-of-path edit and an implementer edit to D3).

## MAJOR

None.

**M1 (attempt 1): fixed.** `record_edit_ramps` (`crates/protocol/src/model.rs:563-698`) now has no
`_ => {}`. Each of the 46 variants has an arm. I checked each no-op variant against
`apply_session_edit` (`model.rs:739-1000`):

- `SetEffectIdentity`, `SetEffectQuality` and `SetEffectSidechain` write `identity`, `quality` and
  `sidechain` through `effect_mut`. None writes `link_mode`.
- `SetEffectBypass`, `UpsertEffectParam` and `RemoveEffectParam` write `bypass` or `params` through
  `knobs_mut`. None writes `link_mode`.
- `SetTrackEffectOrder` only permutes the existing effects. Each effect keeps its `link_mode`, and the
  key is `(strip, rack, effect ID)`, which does not include the position.
- `SetRouteSource` and `SetRouteDestination` write only `source` or `destination`. Gain, mute,
  follows_mute and matrix stay as they were.
- `SetTrackSourceAssignment` writes the source ID and the two channels. The session scalars,
  sources, outputs and automation edits touch no strip, route, VCA or console row.

None of these edits writes a keyed row, so the stated reasons are correct. The keyed edits clear as
follows. `PutTrackEffect` and `RemoveTrackEffect` clear their `Effect` target. `SetTrackRack` calls
the new `EditRamps::clear_rack` (`crates/session/src/edit_ramp.rs:169`). `RemoveTrack` and
`RemoveSubmix` clear their `Strip` target, together with its effects. `RemoveRoute` and `RemoveVca`
clear their own targets.

`rack_mut` (`model.rs:1347`) refuses `Builtins` and `Console`, so an `Effect` entry can only exist in
the `Inserts` rack. For this reason, a `SetTrackRack` on another rack cannot leave a stale length.
`SetConsole` replaces only the session's slot declarations. If the slot set changes, every strip's
entries must be rewritten in the same transaction by `SetTrackConsole`, `UpsertTrack` or
`UpsertSubmix`, and each of those writes or clears the strip's `Link` row. Deviation 9 of the
attempt record is corrected.

## MINOR

**m-a. `SetVcaFader`'s D3 rows are pinned by no test.**
`model.rs:614-617` records `VcaFader` and `VcaMute` on `Vca(id)`. The mutation that records
`&[EditRampRow::VcaFader]` only stays **GREEN** in the whole `protocol --features test-support`
suite. The new "RemoveVca removes the VCA's rows" case expects `[]` after the clear, so it cannot see
which rows `SetVcaFader` wrote. `every_live_value_edit_records_its_rows_on_its_target`
(`crates/protocol/tests/edit_ramps.rs:160`) covers every other D3 row except the three that gate 2
covers, but it has no `SetVcaFader` entry. This gap came from attempt 1, where the base had no VCA,
and the attempt-1 verdict did not see it. The wrong-target mutation (`Route` in place of `Vca`) is
RED, so only the row set is open.

VCA rows are not live on this base (non-goal), so no sound changes today. But D3 is this issue's
deliverable, and the VCA-row slice will read these entries. The base now has the empty VCA `grp`, so
the fix is one table entry in `every_live_value_edit_records_its_rows_on_its_target` and one
`SetVcaFader` arm in its `bare` match. This does not fail the attempt. Fix it before the merge.

**m-b. Out-of-path edit: `crates/session/src/model.rs:998`.**
`RackName` gained `Ord, PartialOrd` so that `Key` can hold the typed rack (the n2 fix). The spec's
authorized paths in `crates/session` are `edit_ramp.rs` and `lib.rs` only. "Field additions only"
covers the `ramp_samples: None` constructions, not derives. The change adds two public trait impls
and changes no behaviour. The derived order is the declaration order, Inserts < Builtins < Console.
That is the same as the wire-code order (2 < 4 < 5), so the `entries()` order did not move. The
attempt record names the derive, but it does not say that the file is outside the authorized paths.

Root must ratify this edit, or the implementer must revert it. The other n2 shape is the old `u8`
rack code with an exhaustive `from_wire`, which needs no edit to `model.rs`.

**m-c. The implementer edited D3 and gate 7, which are root's text.** Root must ratify both. My
judgment:

- `PutTrackEffect` and `SetTrackRack` clearing is implied by D3's existing rule ("the last edit that
  writes a `(target, row)` decides it"). Both edits write the effect's `link_mode`, and neither
  carries a length. The attempt-1 verdict asked for exactly this.
- The clearing on `RemoveTrack`, `RemoveSubmix`, `RemoveRoute`, `RemoveVca` and
  `RemoveTrackEffect` is **not** literally implied, because a removal writes no row. It goes past the
  rule as written. But it has no observable effect on any record length:
  - A removed target can come back only through an edit that clears it itself: `UpsertTrack`,
    `UpsertSubmix`, `UpsertRoute`, `UpsertVca`, `PutTrackEffect` or `SetTrackRack`.
  - The classifier reads only the targets in `next`.

  So it is hygiene, and it changes no product scope. It changes only what `entries()` returns, and
  `entries()` has no production caller.
- The sentence "the match over `SessionEdit` is exhaustive" is an implementation note, not scope.
- The gate-7 command change (adding `control-provider`) fixes attempt-1 n3. It is correct: without
  the feature the command runs 0 tests, and with it the command runs 34.

Conclusion: the D3 change makes no change to the product contract. It adds one behaviour-neutral
rule (clear on removal). It was made by the implementer in a frozen-decision section, so I report
it to root.

## NIT

- **n-a. `SessionEdit::ramp_samples()` still ends in `_ => None` (`model.rs:554`).** There is a real
  but small latent risk. If a future live-value variant gets a `ramp_samples` field but no arm here,
  its length is dropped by the recorder (`model.rs:564`), by the D4 bound (`model.rs:1190`) and by
  the encoder (`session_wire.rs:343`, `:639`). The encoder's use reduces the risk: when I removed the
  `SetVcaFader` arm, `live_value_edits_round_trip_their_optional_ramp_length` went **RED**. So the
  normal gate-1 round-trip for a new edit catches the omission, but only if its author adds that
  test. An exhaustive match in a `const fn` costs nothing and would apply the same rule that
  attempt 2 applied to `record_edit_ramps`. Recommend it. It does not block.
- **n-b. `clearing_a_rack_takes_that_racks_effects_of_that_strip_only`
  (`edit_ramp.rs:283`): only its rack leg is a unique catch, and that leg defends a state that
  production cannot reach today.**
  - "`clear_rack` ignores the rack" turns this test RED and stays GREEN in all of protocol, because
    only `Inserts` can hold an `Effect` entry.
  - "Takes the strip's own rows" and "takes another strip's entries" are also caught by the protocol
    test.

  This is the same class as attempt 1's accepted `clear_target` effect test: it defends the public
  API contract. I accept it.
- **n-c. `entries()`'s `unreachable!` (`edit_ramp.rs:206`) cannot be reached from any input.**
  `Key` is private. The only key that is inserted is `Key::new` → `split`, and `split` gives an effect
  part only to `Owner::Strip`. The two range-start keys in `clear_target` and `clear_rack` are never
  inserted. `EditRamps` has no `Deserialize`, and `Default` or `Clone` cannot create a key. Also,
  `entries()` has no production caller (tests only), and the code runs on the control thread. The
  former `from_code` and `filter_map` paths are gone.

The other attempt-1 items:

- n1: the redundant unit test is deleted, which is correct.
- n4: informational and unchanged. The inline size is still 24 bytes, and the `capi` resource tests
  pass.

## The caller's questions

- **Does the new base weaken gate 2 or gate 3? No.** The base now has a submix `bus`, an empty VCA
  `grp` and no automation.
  - Gate 2 asserts exact entry vectors. A submix or a VCA that no edit names adds no entry.
  - Gate 3's cases refuse on `RampOutOfRange`, which comes after a successful compile, or they
    `expect` a prepare. The canonical automation only targets the `eq`, and no gate-2 or gate-3 case
    depended on it.
  - `model.routes[0]` is still `to-main`.
  - The base change is what lets the removal cases compile.
- **m1 mutation, re-run.** I mapped `RampOutOfRange` to `"session.edit.not_found"`: RED
  (`a_ramp_length_out_of_range_answers_its_own_code_at_the_edit_path`). I also made the path index a
  constant 0: RED in the same test. The existing `edit_rejections_use_typed_operation_diagnostics...`
  test uses index 0, so it cannot see the path mutation.
- **Acked-batch question: no.** Attempt 2 changes no queue, cell or admission path. The refusal stays
  in `prepare_transaction`, before any reserve, push or commit. No ack can come before a drop.
- **The whole #1394 change, after #1364 moved `LiveRamps`.** I re-checked it at `bd8a92e9d`.
  `LiveRamps` is now in `crates/host-core/src/live_ramps.rs`. The classifier
  (`live_delta.rs:276-330`) still reads every length through `resolve`. No `LiveRamps` field is read
  directly. `commit_live` passes `transaction.edit_ramps()` (`control.rs:1352`).
- No new producer, fan or listener role wording, and nothing that depends on decision 16. I checked
  this with grep on the added lines of both ranges.

## Test value, one sentence per new test

- `edit_ramps.rs` `an_edit_that_rewrites_or_removes_a_target_clears_its_earlier_lengths`: it turns
  red if a rack edit, a put or a removal leaves an earlier edit's stale length for a row that it
  rewrote or ended. No other test catches this. These mutations each went **RED** in this test only:
  - `PutTrackEffect` no clear; `RemoveTrackEffect` no clear; `SetTrackRack` no clear;
  - `SetTrackRack` clears the whole strip; `PutTrackEffect` clears the whole strip;
  - `RemoveTrack`, `RemoveSubmix`, `RemoveRoute` and `RemoveVca` each with no clear.
- `controller/tests.rs` `a_ramp_length_out_of_range_answers_its_own_code_at_the_edit_path`: it turns
  red if the wire diagnostic code or its `edits[i]` index is wrong. Both mutations: **RED**.
- `edit_ramp.rs` `clearing_a_rack_takes_that_racks_effects_of_that_strip_only`: it turns red if
  `clear_rack` ignores the rack. That mutation is **RED** here and **GREEN** in protocol (n-b).

Mutation log: `/tmp/claude-1002/v1394/logs/mutations.out`. There are 16 runs, and I restored each
one with `git checkout`.

## Gates run (all on `bd8a92e9d`, private `CARGO_TARGET_DIR`)

- **The whole `lint` job set of `qualification.yml`: 32 of 32 steps PASS.**
  - fmt, clippy `-D warnings`, rustdoc `-D warnings`.
  - The workspace, session, bench, host-core, protocol-control, realtime, realtime-audit-leak,
    artifact-evidence-leak, lane, rack, builtins, graph and effect-runtime policy checks, with their
    mutation tests.
  - `check-test-support-ci` and its mutations, env vocabulary, the scalar-oracle self-test.
  - Script reachability (in a git-initialized export), the console-benchmark fixture, the
    builtins-fixture mutations, bench preconditions, the trace validator, the unfused seal.
  - Conformance boundaries, the PEQ seal, the release-shape self-test, npm publish modes, the stem
    store, stem identity.
  - The three x86 lane-guard probes, including the `rustc --print cfg` assertions.
- **The whole `test-release` job set: all PASS.**
  - lane/math/wasm-gates (133), filter_liveness (15), tail_contract (25), ramp_endpoint (16),
    designer_total (1), M3 FMA cfg, loom (13).
  - M1 exhaustive (2) and F1 (5). The router gives `math_closure=true` for the stream range, so I ran
    them.
- **Spec gates: all PASS.**
  - `-p session` (96); `-p protocol --features test-support` (153; `edit_ramps` 4).
  - `-p host-core --features test-support,control-provider --test live_delta` (34); `-p capi` (105);
    `-p conformance` (24).
  - `test-debug-a` (1576 passed, 134 suites) and its doctests (24).
  - `audit capi`: 0 allocations, 0 syscalls, 0 violations, `pcm_digest a9738fa800afcab3`.
  - `run-protocol-allocation-audit.sh`; `check-builtins-fixtures.sh` (builtins manifest digest);
    `check-capi-abi.sh` (through a `target` link to my target dir).
  - `check-protocol-wasm-parity.sh` (ok; the attempt record did not re-run it);
    `check-cross-targets.sh`.
- **Worklet chain: all PASS.**
  - `build-web-audioworklet.sh --named-twin`.
  - `check-web-audioworklet.sh --without-metadata-regeneration`.
  - `check-browser-expected-resources.py --artifacts` (32 red mutations, every budget met, source
    total 3358 of 3648 bytes).
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh` under a private TMPDIR (nothing left behind).
- Not run: the AArch64 legs (CI only), the three-browser matrix, and the V8 spill gate (it needs
  exact Node 22.23.2).
