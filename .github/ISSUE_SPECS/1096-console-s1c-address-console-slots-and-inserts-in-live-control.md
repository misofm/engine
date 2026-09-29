# Address console slots and inserts in live control

Slice S1c of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M5 and amendment 5 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

Live controls address an effect by `(track_id, rack, effect_index)`, with the index taken in session
declaration order within the rack:
- `crates/host-core/src/prepare.rs:337-352`;
- `declared_effect_indices` in `crates/effect-compiler/src/prepare.rs:1450-1473`;
- `miso.command.v1`.

The racks carry six encodings: `RackName` 1-4, `ParameterRack` 1-4, `RackId` 1-3, `RackLocation`
1-3 in another order, `EffectRack` and two browser encodings.
- The browser's frozen 48-byte command record encodes rack `0` simd1, `1` dynamic, `2` simd2 and
  `255` not applicable (`hosts/host-web/src/lib.rs:3760-3770`).
- The live-response owner record encodes `0` input filters and `1..3` as `RackId`.
- The browser spectrum targets are `trackPostInputBuiltins` and `trackPostMatrix`
  (`scripts/check-abi-layout-v1.py:89`).

After S1a, the session addresses `console` slots and `inserts`, and none of these encodings can say
"console".

## Smallest closable slice

1. **Live address.**
   - A console slot is `(track_id, console, slot_index)`. `slot_index` is the slot's position in the
     session's slot order: `pre_insert`, then `post_insert`, which equals the index into the
     track's `console` array.
   - An insert is `(track_id, inserts, index)`.
   - Internally the address maps through the lowering: `pre_insert[k]` is `(Simd1, k)`,
     `post_insert[j]` is `(Simd2, j)` and `inserts[i]` is `(Dynamic, i)`.
   - Whether `EffectRack` becomes the session-facing `{Console, Inserts}` or stays internal behind a
     boundary translation is this slice's choice; record why. `RackId`, `TrackStage`, `MeterTap` and
     `RackLocation` stay (decision 12).
2. **The browser's 48-byte command record.** The rack byte keeps `1` for `inserts`. `0` and `2` are
   retired and refused with `unknownRack`. `3` is appended for `console`, and `255` stays "not
   applicable". The layout, size and `ABI_VERSION` are unchanged.
3. **The observation and live-response encodings** follow the same rule: `inserts` keeps its code,
   the simd1/simd2 codes are retired and refused, and `console` is appended. In the owner record,
   `2` stays `inserts`, `1` and `3` are retired, and `4` is `console`.
4. **The spectrum targets** follow the tap rename with codes unchanged: `trackPostInput` (1),
   `trackPostPan` (2) and `output` (3).
5. Update the host's `.d.ts` and refresh its SDK mirror (`sdk/src/browser/shipped-host.d.ts`,
   compared by `scripts/check-sdk-generated.sh`). Regenerate the ABI layout JSON and bindings.
6. **The V8 benchmark harness.** It writes the record's rack byte from the controls table
   (`scripts/web-mixing-automation-benchmark.mjs:279`), and it resolves rack names in its lookup at
   `:136-140`; this slice owns both. The table's codes come from
   `tools/console-workload/src/mixing_automation.rs`, through
   `examples/mixing_automation_controls.rs`, and today they are `0`/`1`/`2` with an index within
   the section (S1a's interim). This slice moves them to `3` plus the slot index for console slots
   and `1` plus the index for inserts. Without it, S4's V8 run refuses at preflight.

Authorized paths:
- `crates/effect-compiler/src/prepare.rs` (addressing);
- `crates/host-core/src/prepare.rs` and the live-control attach path;
- `hosts/host-web/src/**` and `hosts/host-web/web/**`;
- `tools/parameter-metadata/src/abi_layout.rs`;
- the generated `sdk/assets` layout, `sdk/src/generated/abi.ts` and the mirror;
- `scripts/check-abi-layout-v1.py` and its self-test fixture;
- `docs` pages that document the record;
- `scripts/web-mixing-automation-benchmark.mjs` (the rack byte only),
  `tools/console-workload/src/mixing_automation.rs` and
  `tools/console-workload/examples/mixing_automation_controls.rs`;
- this spec.

The SDK's builder and types are S1d's.

## Owner decisions that bind this slice

Decision 12's "Wire identity": nothing is renumbered, every retired code is refused, there is no
`ABI_VERSION` bump, and the app updates in lockstep.

## Dependencies

- *Add the session console and per-track inserts to the session schema* (S1a, #1093).
- *Carry the session console and inserts in the control protocol* (S1b, #1094).
- *Rename the live console to live controls* (S1r, #1095), which has merged before S1a in the same
  batch (C3).

## Objective gates

1. A live parameter change and a live bypass reach the right lane:
   - on a console slot in `pre_insert` and in `post_insert`, addressed by slot index;
   - on an insert;
   - in both the native host-core path and the browser record path.

   A planted off-by-one in the section split (post_insert slot addressed as pre_insert) turns it red.
2. Records carrying rack `0` or `2`, and owner or observation records carrying a retired code, are
   refused with their typed reasons.
3. `python3 -B scripts/check-abi-layout-v1.py` (layout and self-test) and
   `bash scripts/check-capi-abi.sh` pass. The C ABI carries no rack addressing today, so it should
   not move; if it does, explain why.
4. The host-web qualification that drives the raw exports passes in the three browsers in CI mode.
   Run it without `--sdk-root` (`hosts/host-web/qualification/run.mjs:837-858`), which leaves out
   the SDK bundle and runs only the raw-export subset. SDK-driven qualification is S1d's gate.
5. The benchmarks still run. Both checks are untimed, on a committed clean tree:
   - `bash scripts/operator/preflight-console-benchmark.sh --step <an unused scratch name>`;
   - `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then `preflight
     WORKDIR`, in an empty scratch directory.

   The V8 preflight exercises the new rack codes through the shipped module.
6. PR evidence: console digests are unchanged.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.
