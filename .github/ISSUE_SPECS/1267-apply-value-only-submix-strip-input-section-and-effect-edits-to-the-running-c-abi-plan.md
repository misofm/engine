# Apply value-only submix-strip input-section and effect edits to the running C ABI plan

Follow-up of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053) and of *Deliver value-only send and submix-strip edits
to the running C ABI plan* (#1225). It closes the submix half of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259). Filed on 2026-10-04 with anchors
at `54b0a1bf8`; **every anchor must be re-verified when it starts**, because each of its
dependencies changes the code it extends.

## Product outcome

On the C ABI, a transaction that changes a submix strip's input `trim_db`, `polarity_invert`,
`hpf_hz` or `lpf_hz`, a live parameter of one of its effects (console entry or insert), or one of
its effects' bypass, is a live update, exactly as the same edit on a track is after #1261-#1266.

Decision 14's rows apply to every strip, tracks and submixes alike (its Q5 answer: buses keep trim
and polarity). After #1225 a submix strip's fader and pan are live on the C ABI, but its input
section and effects are not: #1225 excludes them, and #1261-#1266 handle tracks only.

## Context (at `54b0a1bf8`; re-verify)

- **The model.** `Submix { id, builtins, console, inserts, fader, matrix_or_pan }`
  (`crates/session/src/model.rs:688-702`): the same fields as a track, without a source mapping.
- **The lanes.** host-core attaches one control producer per strip, submixes included
  (`HostLiveControlHandles::strip_controls`, `crates/host-core/src/prepare.rs:374-381`), and one
  effect producer per effect instance, submix effects included (`:382-388`; `track_id` then
  carries the submix ID).
- **The classifier.** After #1225, `classify_live_delta` masks a submix strip's `fader` and
  `matrix_or_pan`. After #1261-#1266, it masks a track's input section, effect `params` and
  `bypass`. This slice applies the track rules to submix strips, through the same code.
- **G1** (#1053's guards). Until this slice, any other submix-strip field keeps a delta
  structural.

## Decisions

- **D1.** Every rule of #1261 (trim, polarity), #1262 (HPF, LPF through `InputFilterPreparer`),
  #1264 (effect parameters, the `Prepared` and capacity fallbacks, the readback), #1265 (the EQ
  through its owner) and #1266 (bypass, the prepared-bypass exception) applies to a submix strip
  exactly as to a track. Records are addressed by the submix's strip ID, never by index.
- **D2.** No new lane, record, setting or ABI item. If a rule needs a submix-specific exception,
  stop and record it rather than invent one.
- **D3.** Mark G1 superseded in #1053's spec when its last part is lifted, if the spec is still in
  `.github/ISSUE_SPECS/`.

## Authorized paths

- `crates/host-core/src/live_delta.rs` and `crates/host-core/tests/live_delta.rs`.
- `crates/capi/src/runtime/control.rs` and the capi live test module.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- `.github/ISSUE_SPECS/1053-*.md` (G1's bullet only).
- This spec.

## Non-goals

- No submix `delay_samples` (rule 1, rebuild).
- No change to the browser.

## Objective gates

1. **The classifier.** For each track case of #1261, #1262, #1264, #1265 and #1266's classifier
   gates, the same edit on a submix strip gives the same records, addressed to the submix.
   *Test value: it turns red if a submix edit stays structural or is addressed by index.*
2. **PCM.** For trim, polarity and a compressor bypass on a submix strip that a send feeds, the
   live edit renders bit-identically to a plan compiled from the committed snapshot, from
   `latency_samples` plus one quantum after the edit (plus the send's compensation delay), on a
   session whose submix has no other stateful effect. For HPF, LPF, effect parameters and the EQ,
   compare with the same records pushed by hand into a host-core plan prepared with
   `HostLiveLanes::ALL`.
   *Test value: it turns red if a submix record lowers or lands differently from a track's.*
3. **Nothing else changes.** #1257's gates 6 and 7. 4-lane (NEON) is CI-only.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.

## Dependencies

- *Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225)
- *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262), which follows *Apply value-only input trim and polarity edits to the running C ABI
  plan* (#1261)
- *Apply value-only parametric EQ parameter edits to the running C ABI plan through prepared
  targets* (#1265)
- *Apply value-only effect bypass edits to the running C ABI plan* (#1266)

## Standing rules for the implementer

- Work from this body and the merged code of its dependencies. Extend them; do not fork them.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
