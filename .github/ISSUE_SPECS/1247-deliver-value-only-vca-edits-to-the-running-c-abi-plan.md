# Deliver value-only VCA edits to the running C ABI plan

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-6).
Code anchors verified on `main` at `6fb211594`.

Slice V8 of *VCA groups* (#1239). Rewritten 2026-10-05 for decision 15. Decision 14's follow-up F9
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, "F9, rule 1") asked for a reason to keep
VCA membership structural on the C ABI. Decision 15 answers it: membership is live on the C ABI
(D15-6). The C ABI's render plane holds no VCA state, so a membership change needs no new render
memory, and "reach drift" is not a rule-3 reason. The old D1 (membership structural), the
capi-owned `LiveVcaState` and the per-queue `BACKPRESSURE` gates are superseded by decision 15
(D15-2, D15-6).

## Product outcome

A C ABI host, such as a fan's personal mix on a phone, can make all of these VCA edits on the
running plan, with no rebuild:

- ride a VCA ("drums -3 dB"), or mute it;
- add or remove a VCA;
- change a VCA's members, including nested VCAs.

The plan renders the same bits as a plan prepared from the edited session, after the ramp. A
member's own fader edit is live too. A VCA mute silences its members' `follows_mute` sends in the
same commit.

## Context

- **Guard G3.** `classify_live_delta` refuses any delta while either model declares a VCA
  (`crates/host-core/src/live_delta.rs:216-218`, `LiveRebuild::Vca` at `:119-120`). The
  classifier stays in host-core; #1309 moves capi's live admission and commit path into
  `crates/control-plane`.
- **Preparation composes VCAs once.** `SessionModel::effective_strip_faders`
  (`crates/session/src/vca.rs:100`) gives each strip, in `strips()` order:
  - `db[l] = vca_effective_db(own_db[l], reach offsets[l])`, where `vca_effective_db` is at
    `:23` and the reach comes from `SessionModel::vca_reach` (`:52`);
  - `mute[l] = own || vca_mute`.

  The builtins compiler bakes these values into each strip's fader
  (`crates/builtins-compiler/src/lib.rs:4941-4943`). The graph compiler reads the effective mute
  for every following route (`crates/graph-compiler/src/compile.rs:323-333`). Render holds no VCA
  state.
- **The C ABI keeps no live VCA state.** See the module comment at
  `crates/host-core/src/vca.rs:59-61`. `LiveVcaState` (`vca.rs:84`) is the browser's composition.
  Its reach is fixed at `try_new` (`:116`), which is why the browser keeps membership structural
  (decision 14, rule 1, the browser's boot-sized admission scratch).
- **Count caps.** Preparation refuses `vca_count > caps.maximum_vcas` with `host.resource.count`
  (`crates/host-core/src/prepare.rs:1162-1170`). The live path today never adds a VCA, so it never
  runs this check.
- **Opcodes** (#1241): `UpsertVca` `0x0700`, `RemoveVca` `0x0701` and `SetVcaFader` `0x0702`
  (`crates/protocol/src/model.rs:112-116`).
- **After #1225 and #1226:**
  - strip records cover tracks and submixes;
  - route records come from `route_target`, which reads the effective mute;
  - follow-source mutes and `follows_mute` are live;
  - every live value is a latest-target cell (#1312; route lanes #1347).
- **Superseded tests.** These tests assert today's G3 behaviour:
  - `any_vca_needs_a_rebuild` (`crates/host-core/tests/live_delta.rs:460`);
  - the G3 case ("a VCA") of
    `deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing`
    (`crates/capi/src/runtime/live_tests.rs:1015`).

## Decisions frozen for this slice

- **D1. Remove guard G3, and mask the VCAs.** Delete `LiveRebuild::Vca`. The classifier copies
  `current.vcas` into the masked model, so every VCA field is out of the structural comparison:
  the set, IDs, faders and members. VCAs are control-only, and their whole effect reaches render
  through strip and route records (D2, D3).
- **D2. Strip records diff effective faders.** For each strip, the classifier compares
  `effective_strip_faders()` of `current` and `next`, lane by lane. It emits:
  - one `FaderDb` carrying the effective dB for each lane whose effective gain bits change;
  - one `Mute` carrying the effective mute for each lane whose effective mute changes.

  The `Both` merging and the push order are today's. Raw `left_db`, `right_db` and mute values are
  never read for records. With no VCA, `vca_effective_db` returns the member's own dB bit for bit
  (`vca.rs:19-27`), so VCA-free sessions emit exactly what they emit today. The domain check is
  `checked_fader_gain` on the effective dB, the value the plan bakes.
- **D3. Route records.** #1225's `route_target` already reads the effective mute. A VCA mute or
  unmute, or a membership change that alters a source's effective mute, emits the follow records
  in the same delta.
- **D4. Count caps on the live path.** Move the count check at `prepare.rs:1162-1170` into one
  function that both preparation and the live admission call. A live delta whose `next` exceeds a
  cap is refused with the same `host.resource.count` diagnostic and changes nothing.
  `compiled_model_admission` (`crates/capi/src/runtime/compile.rs:76`, in `crates/control-plane/src/compile.rs` after #1309) already charges model growth.
- **D5. Ramps (D15-1).** A `FaderDb` record uses the session's fader length, and a `Mute` record
  uses its mute length. Both come from `LiveRamps::for_session(next)` (#1054). One VCA move ramps
  every member it moves over that length.
- **D6. No mirror.** The C ABI keeps no `LiveVcaState`. The classifier reads both committed models
  (#1053 D9: the committed model is the authority), so membership drift cannot occur. Remove the
  "keeps no live VCA state until #1247" clause from `vca.rs:59-61`. In its place, say that the C ABI
  composes VCAs by diffing committed models.
- **D7. Cost.** Each classification computes `effective_strip_faders()` twice. That is
  `O(strips x reach)` on the control thread, under the caller's `maximum_vcas` and
  `maximum_tracks`. It never runs on render.

## Deliverables

- D1-D6 in the classifier, in `crates/control-plane`'s live admission, and in `prepare.rs`'s count
  check.
- `docs/C_ABI_V1_QUALIFICATION.md`: VCA fader, mute, membership and VCA add or remove edits are
  value-only, and a live edit that exceeds `maximum_vcas` is refused like a compile.
- The superseded tests in Context, inverted in the same PR.
- In `.github/ISSUE_SPECS/1053-*.md`, if that spec is still present: mark guard G3 as superseded.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the classifier).
- `crates/control-plane/src/control.rs`: `live_admission` and the live commit path (moved there by
  #1309).
- `crates/host-core/src/prepare.rs`: the count check at `:1162-1170` only (D4).
- `crates/host-core/src/vca.rs`: the module comment at `:59-61` only (D6).
- `crates/host-core/tests/live_delta.rs`, `crates/capi/src/runtime/live_tests.rs` and
  `crates/capi/tests/resource_lifecycle.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`; `.github/ISSUE_SPECS/1053-*.md` (guard G3's bullet only); this
  spec.

## Non-goals

- No browser change. The browser keeps membership structural under decision 14 rule 1, and
  `LiveVcaState` is not changed.
- No VCA solo, no send trim and no stored automation.
- No new symbol, opcode or field.

## Hazards

- **Raw values.** Any path that still reads `left_db`, `right_db`, `left_mute` or `right_mute` for
  a record reopens a VCA-muted member's following send, or drops an offset. Gates 2 and 3 catch it.
- **Clamping.** A VCA move that leaves every member clamped at +24 dB changes no effective bit, so
  it emits nothing.
- **The iOS memset rule.** `scripts/check-cross-targets.sh` counts `bl _memset_pattern16` per
  product crate, and the new crate may have no row. Store no splatted non-zero constant to memory.

## Objective gates

1. **PCM through the C ABI.** Each edit below keeps the same plan, with no new epoch and no
   re-seek. After `latency_samples`, plus the ramp, plus a send's compensation delay where it has
   one, the output is bit-identical to a plan compiled from the committed model. The downstream is
   stateless. The edits:
   - `0x0702` on a nested VCA;
   - `0x0702` that mutes that VCA, where one member has a `follows_mute` pre-fader send;
   - a member's own `0x020f`;
   - `0x0700` adding a member;
   - `0x0700` adding a new VCA;
   - `0x0701` removing a VCA.

   Run with 1 and 10 tracks at 44.1, 48, 88.2 and 96 kHz.
2. **Effective, not raw** (classifier unit test). One member is VCA-muted and has a `follows_mute`
   send. Toggling the member's own mute while the VCA mute holds yields no strip record and no
   route record.
3. **Effective mute in send edits.** A member is VCA-muted. A live `0x0505` on its following send
   renders bit-identically to a fresh plan of the committed model: the column stays zeroed.
4. **Count cap.** With `maximum_vcas` equal to the current count, a `0x0700` adding a VCA returns
   `COMPILE_REJECTED` with `host.resource.count`. The model, revision and cells are unchanged.
5. **Redundant records.** An exact replay writes nothing. A VCA move that changes no effective
   value, because every member is clamped, yields no record.
6. **Realtime.** The race test
   `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`
   (`crates/capi/tests/resource_lifecycle.rs:2900`) races VCA fader, mute and membership edits
   against render and a structural swap, over 20 runs. It checks:
   - `allocations == 0` and `frees == 0` around each render call after warm-up;
   - zero `INTERNAL` results;
   - a final block bit-identical to a fresh plan of the final committed model.
7. **Unchanged behaviour and policy.** The commands of gate 6 of #1225 (`audit capi`,
   `check-capi-abi.sh` and its self-test, the workspace test step, fmt, clippy, the host-core,
   realtime and workspace policy scripts, `check-cross-targets.sh`, and `run-aarch64-tests.sh
   debug`).

## Test value

- Gate 1 turns red in any of these cases:
  - a VCA, member or membership edit is still structural;
  - the live composition differs from preparation's;
  - a VCA mute leaves a member's follow send open.
- Gate 2 turns red if records diff raw mutes instead of effective ones. That emits a redundant
  record, which retargets a settled lane.
- Gate 3 turns red if a route record reads the raw committed mute instead of `own || vca_mute`.
- Gate 4 turns red if a live edit commits a model that its own rebuild would refuse.
- Gate 5 turns red if the composition re-emits unchanged targets.
- Gate 6 turns red if the VCA path allocates on render, or if a racing VCA edit reaches the
  retiring plan.

## Dependencies

- *Let C ABI sends follow their source strip's mute live* (#1226), and through it *Deliver
  value-only send and submix-strip edits to the running C ABI plan* (#1225)
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)
- *Hold live values in latest-target cells on both hosts* (#1312)
- *Hold route-lane values in latest-target cells* (#1347)
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054)
