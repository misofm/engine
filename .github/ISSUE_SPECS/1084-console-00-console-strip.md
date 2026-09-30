# Console strip: session-level console effects with per-track inserts

The owner stated this design on 2026-09-29. It emulates a hardware mixing console: the console's
built-in channel processing is the same on every channel, and each channel sets only its own
knobs. Outboard gear is patched into a channel's insert point and differs per channel.

**Provenance.**
- Sol verified the draft and returned SOUND WITH AMENDMENTS
  (`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, first committed at
  `.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md` in `03aceb94`; findings H1-H5, M1-M9, L1-L7;
  amendments 1-14).
- The owner answered on 2026-09-29, and **decision 12** in
  `docs/rulings/engine-footprint-2026-09-29.md` records the answers.
- R0 amended AGENTS.md's chain line, its tap list and its #163 sentence in the same commit.

This umbrella folds all of that in. It is the design of record, and its slices are separate issues.

## Problem

The engine does not implement this today.

- **Racks are per track.** `docs/SESSION_SCHEMA_V1.md` and `crates/session/src/model.rs` (`Track`)
  give every track its own `simd1`, `dynamic` and `simd2` racks. Nothing makes a slot
  session-level, and nothing stops tracks from diverging.
- **Banking is opportunistic, and bypass splits it.** Banking groups whichever tracks happen to
  have identical chains. `bypass` is part of the bank key (`EffectProgramKey`,
  `crates/effect-contract/src/lib.rs:984-999`), so a bypassed track leaves its cohort (H1).
- **Effect remainders never bank.** An effect bank binds only a full group: every effect factory
  refuses `requests.len() != lanes`, and the contract has no inactive-lane mask (#96 F7,
  `crates/graph-compiler/src/banks.rs`). Leftover tracks run each effect one lane at a time.
  Builtins already pad partial banks.
- **The app.** On `misofm/app` `origin/main` (`0757a84`), EQ -> compressor is compiled onto every
  track's dynamic rack, and unselected tracks are marked `bypass`
  (`src/lib/mixer/engine/session-document.ts:252-275, 371-383, 476-479, 548-561`; corrected by M8).
  Today a bypassed EQ track sits in its own cohort and skips the wet path.

## Design (decided)

### Session shape

```json
"console": {
  "pre_insert":  [ { "slot": "eq",   "identity": { "kind": "native", "effect_id": "miso.parametric-eq" },
                     "quality": ..., "link_mode": ... },
                   { "slot": "comp", "identity": { "kind": "native", "effect_id": "miso.compressor" },
                     "quality": ..., "link_mode": ... } ],
  "post_insert": [ { "slot": "limiter", "identity": { "kind": "native", "effect_id": "miso.true-peak-limiter" },
                     "quality": ..., "link_mode": ... } ]
},
"tracks": [
  { "id": "vox",
    ...,
    "console": [ { "slot": "eq",      "bypass": false, "params": [ ... ] },
                 { "slot": "comp",    "bypass": true,  "params": [ ... ] },
                 { "slot": "limiter", "bypass": false, "params": [ ... ] } ],
    "inserts": { "effects": [ <per-track effect declarations, as today's dynamic rack> ] },
    ... }
]
```

- **Session `console`.** Each slot is declared once:
  - `slot`, a stable ID unique across both sections, because the address `(track, console, slot)`
    does not name the section (L6);
  - the native effect `identity`;
  - `quality`;
  - `link_mode`.

  `pre_insert` runs before the insert point and `post_insert` after it. Either list may be empty,
  and an empty list costs nothing.
- **Per-track `console`.** An array of `{ slot, bypass, params }` in exactly the session's slot
  order (`pre_insert`, then `post_insert`). Every track has every slot. A track cannot add, remove
  or reorder slots, and an entry carries no effect fields. The array form is what canonical JSON
  and BTLV can express; a map keyed by slot names has no canonical order (H4).
- **Per-track `inserts`.** Replaces `dynamic`, with the same semantics. It may be empty.
- **Removed:** the per-track `simd1`, `dynamic` and `simd2`.
- **Eligibility** (L2). The parametric EQ, compressor, gate/expander, soft-clip, transient shaper
  and true-peak limiter. The delay never banks. The multiband compressor is excluded until #1069
  closes. Third-party code is never a console slot. A compile diagnostic enforces the list.
- **Sidechain** (H3). Console slots have none. A keyed effect is an insert. A banked sidechain port
  is a future contract issue (#96 F9).

### Chain and tap points

```
input -> input section (polarity, trim, HPF/LPF) -> console.pre_insert -> inserts
      -> console.post_insert -> fader/mute -> pan/matrix -> routes
```

The seven send taps are renamed. Their positions and wire codes are unchanged.

| Code | Today | New |
|---|---|---|
| 1 | `input` | `input` |
| 2 | `post_input_builtins` | `post_input` |
| 3 | `post_simd1` | `insert_send` |
| 4 | `post_dynamic` | `insert_return` |
| 5 | `post_simd2_pre_fader` | `pre_fader` |
| 6 | `post_fader` | `post_fader` |
| 7 | `post_matrix` | `post_pan` |

A send or leased meter at `insert_send`, `insert_return` or `pre_fader` adds a reader, so that
cohort's chain cannot fuse across it (L5).

### Addressing

- **Automation targets** keep the shape `{ entity_id, rack, effect_id, parameter_id, channel }`.
  - A console slot is `rack: "console"`, `effect_id: <slot>`.
  - An insert is `rack: "inserts"`.
  - The builtins chassis stays `rack: "builtins"`, `effect_id: "strip"`.
  - Stored automation is still inert (`docs/SESSION_SCHEMA_V1.md`), so this is syntax only.
- **Live controls** address a console slot by `(track, console, slot index)` in session slot order,
  and an insert by `(track, inserts, index)`.

### Banking

- **Console slots always bank**, on every target and for every track count, with no member
  threshold. The owner chose this knowing that a one- or two-member remainder costs more padded
  than per node at W=8 (H5).
- **Grouping** (H2, M3). There is one bank group per (slot, pool class, dependency level), each
  padded to W. Levels are longest-path, and a bank may not cross a level, so differing insert
  counts split `post_insert` into several groups, and misaligned cohorts can cost chain fusion. The
  number of groups per slot is the sum over (pool class, level) of `ceil(n / W)`. An ALAP alignment
  of `post_insert` banks is an optional successor, and only on measured need.
- **Padding contract** (M1, amendment 6). `PrepareEffectBankRequest` gains an explicit active mask.
  A padded lane:
  - carries a clone of an active member's prepared request, never zeros;
  - is fed `+0.0`;
  - has its output discarded.

  D7 recovery and reports are masked to active lanes. Console slots need only absent-member lanes,
  never per-lane identity slots, because every track has every slot.
- **Coupling rule** (M2): banking may couple lanes' cost, never their bits. Every whole-bank
  decision must be bit-neutral per lane: fast-path gates, D7 recovery and shunt selection.
- **Inserts** bank opportunistically, as today: full groups bank and remainders render per node.
- **Class A.** A track's output bits are identical whether its lane is banked, padded or per node,
  with NaNs folded to one value (decision 10).

### Bypass, latency and sidechain

- **Bypass is per lane**, through the existing `BypassShunt`, so a bypassed track stays in its bank
  (H1, amendment 1). A session `bypass` lowers to prepared `bypass = false` plus the lane's initial
  shunt state, and the shunt is built whenever any lane is bypassed. A bypassed lane runs the wet
  path; that cost is accepted.
- **Latency is always paid** (L1, L4). Latency comes from the (quality, rate) row, and per-track
  knobs never reach the key. A bypassed slot still carries its latency on every track. A
  `post_insert` limiter adds `rate/100 + 6` samples session-wide, even when every lane is bypassed.
  PDC is unchanged and needs no new delay lines, because the latency is uniform.

### Silence and cost (accepted, then measured)

- The silent fast path is bank-wide (`block_is_positive_zero` over `frames * lanes`), so one active
  track keeps its bank-mates processing (M4). The owner accepted this. B0's sparse-activity row
  measures it, and a per-lane silence skip is a later issue if S4 warrants it.
- A bank-wide skip when every lane is bypassed stays out of scope. It would still have to feed
  latency lines.
- Every track runs every console slot, so cost scales with the group count above, whatever the
  bypass states.

### Wire identity and naming

- **An in-place V1 amendment on the #1063 precedent** (amendment 5). Nothing is renumbered, and
  every retired code is refused, never reallocated.
  - Tap codes 1-7 are unchanged, with the new spellings.
  - `dynamic` -> `inserts` keeps rack code 2 and BTLV track field 7.
  - `simd1`/`simd2` rack codes 1 and 3 and BTLV track fields 6 and 8 are retired. `builtins` keeps
    code 4.
  - `console` is appended: a rack code, a session root field and a track field.
  - The browser's 48-byte command record keeps `1` for `inserts`, retires `0` and `2` and appends
    `3` for `console`. The observation and live-response encodings follow the same rule.
  - There is no `ABI_VERSION` bump. The app and SDK update in lockstep.
- **Naming** (M6, amendment 13).
  - **Owner's decision.** The session key is `console`, and the engine's internal live-console
    names become "live controls". That covers `HostConsoleRequest`/`HostConsoleHandles`,
    `ConsoleEffectBankStage`, the fader, mute and pan lanes' console names and the rest. It also
    covers the sealed `miso_engine_web_v1_console_track_*` exports, whose `_v1` stays and whose
    retired spellings are never exported again.
  - **Root's application of it.** The four `console*` boot words and the SDK's public live-console
    API are renamed with the exports.
  - **Unchanged.** The console benchmark and fixture names stay. `builtins`/`"strip"` is unchanged,
    which is R0's reading.

### Class A by lowering

Internally, `pre_insert` lowers to `RackId::Simd1`, `inserts` to `Dynamic` and `post_insert` to
`Simd2` (M9, amendment 14). Each console entry lowers to today's `Effect`:
- `id` = the slot;
- `identity`, `quality` and `link_mode` from the session;
- `bypass` and `params` from the track;
- no sidechain.

An equivalent session therefore produces the identical graph, including the sealed `MISO-GRAPH-V1`
canonical text, and the identical render. The internal names `RackId`, `TrackStage`, `MeterTap` and
`RackLocation` stay.

### Migration of existing documents (S1a)

The migration is mechanical and works rack by rack. A rack is *uniform* when every effect in it is
eligible and every track declares it identically, with no sidechain. A uniform rack becomes
console:
- `simd1` becomes `pre_insert`;
- `simd2` becomes `post_insert`;
- `dynamic` becomes console too, in a document with two or more tracks. It is appended to
  `pre_insert`, or, if `simd1` stayed an insert, prepended to `post_insert`.

Everything else folds into `inserts` in chain order. Two documents keep a uniform `dynamic` as
`inserts`, because they are the console-versus-insert placement witnesses:
`compressor-dynamic-bank-observation.json` and the legacy `console-sixty-four-track.json`.

The rule's `dynamic` clause is what moves B0's app-shape row, native and V8, into
`console.pre_insert` with its 2-mod-3 bypass pattern, so S4 measures the app on the console. A
`dynamic` rack moved into a console section lowers to `Simd1` or `Simd2`, so its graph text changes
by design and its bits do not (#163).

### #971 under padding

Once every console group binds, the stranded-mono demotion's "binds more banks" objective measures
nothing (M3). S2 decides between restating it (for example, minimise planes x banks) and retiring
it for console slots, and records why (amendment 10). The mono pool is worth about 36 % on the
standing console row, so the choice must not silently demote mono tracks.

## Slices

Each slice is its own issue with one adversarial verdict (the issue map is below). Docs travel with
the slice that changes the behaviour.

| Slice | File (`.github/ISSUE_SPECS/`) | Scope | Depends on |
|---|---|---|---|
| R0 | this commit | Decision 12, the AGENTS.md amendment, this umbrella and the slice specs | none |
| B0 | `1085-console-b0-add-the-console-strip-benchmark-rows.md` | Strip at N in {9, 10, 13, 16, 64}, app shape, sparse activity; layout-neutral `strip_layout`; two V8 documents | R0 |
| S0 | `1086-console-s0-record-the-console-strip-baseline.md` | One native and one V8 run on B0's commit | B0 |
| P1 | `1087-console-p1-keep-a-bypassed-lane-in-its-effect-bank.md` | Session bypass -> shunt state; mixed-bypass cohorts bind one bank | R0 |
| P1b | `1100-console-p1b-bound-and-charge-the-bypass-shunts.md` | P1's verdict conditions: bound and charge the staging windows and shunts; the multiband keeps its prepared bypass; allocation and `-0.0` witnesses | P1 |
| P2a | `1088-console-p2a-let-an-effect-bank-bind-a-partial-group.md` | Active mask, padding contract, planner support; every factory still declines | P1 (merge order: shared files) |
| P2b | `1089-console-p2b-pad-parametric-eq-banks.md` | EQ opts in (supersedes #888's absent-member half) | P2a |
| P2c | `1090-console-p2c-pad-compressor-banks.md` | Compressor opts in (supersedes #889's absent-member half) | P2a |
| P2d | `1091-console-p2d-pad-true-peak-limiter-banks.md` | Limiter opts in; the clone keeps the fast body | P2a |
| P2e | `1092-console-p2e-pad-gate-transient-shaper-and-soft-clip-banks.md` | Gate, transient shaper and soft-clip opt in; gate defaults fixed; soft-clip D7 masked | P2a |
| S1a | `1093-console-s1a-add-the-session-console-and-inserts.md` | Grammar, model, validation, canonical writer, BTLV, lowering; migrate 18 documents and the app-shape row; keep both benchmarks running; repin | S1r (merge order), C1 pushed |
| S1b | `1094-console-s1b-carry-the-console-and-inserts-in-the-control-protocol.md` | Console session edits, retired codes, registry docs, `COMPLETE_SCHEMA_HASH` | S1a |
| S1r | `1095-console-s1r-rename-the-live-console-to-live-controls.md` | Rename the live-control attachment's names, exports, boot words and SDK API | R0; merges before S1a |
| S1c | `1096-console-s1c-address-console-slots-and-inserts-in-live-control.md` | Live addressing, the browser record's rack byte, the observation, live-response and spectrum encodings, the V8 harness's rack codes | S1a, S1b (S1r already merged) |
| S1d | `1097-console-s1d-ship-the-session-console-and-inserts-in-the-sdk.md` | SDK builder, types, writer, live controls, CLI, author-session skill, app handoff | S1a, S1c |
| S2 | `1098-console-s2-bind-every-console-slot-banked.md` | Padding policy for console slots, the no-fallback diagnostic, #971 | P1, P2b-P2e, S1a |
| S4 | `1099-console-s4-measure-the-console-strip-against-its-baseline.md` | Rerun B0's rows; before/after report | S2, S1d, S0 |

The rename (S1r) is its own slice rather than part of S1c: it reaches about 80 code, script and SDK
files, including sealed exports, and would triple S1c while mixing a class-A rename into a
semantic verdict (decision 12).

### Batch plan

The work is delivered in four CI-conscious batches. Each batch is pushed once, and its issues close
after the push.

| Batch | Slices, in merge order | Why this boundary |
|---|---|---|
| C1 | R0, B0, then S0's records | The baseline must be recorded on the unchanged engine, so C1 is pushed before any engine slice lands |
| C2 | P1, P1b, P2a, then P2b-P2e | S2's banking prerequisites, with no schema change. Every shipped plan is unchanged except mixed-bypass cohorts (P1) |
| C3 | S1r, S1a, S1b, S1c, S1d | The schema change, pushed once. From S1a until S1d the SDK is out of step, so nothing is pushed in between |
| C4 | S2, then S4 | The guarantee, then the after-measurement |

Sequencing inside each batch follows shared files, not only functional dependencies:
- P1 and P2a both edit `effect-contract/src/lib.rs` and `graph-compiler/src/banks.rs`, so P1
  merges first.
- P2b-P2e edit disjoint effect crates and may land in any order after P2a.
- C2 is not pushed until P1b, P2b, P2c and P2e have closed (P1 verdict, conditions 1-4). If one of
  P2b, P2c or P2e cannot close, P1's lowering is withheld for that slice's effects before the
  push, so no pushed build couples a bypassed lane's D7 to its bank-mates.
- S1r, S1a and S1c all edit `effect-compiler/src/prepare.rs`, and S1r and P1 both edit
  `rack/src/lib.rs`. C2 lands before C3, and inside C3 the order is S1r, S1a, S1b, S1c, S1d, one at
  a time.
- S1r goes first so that its SDK and browser gates run against an unchanged schema.

C2 and C3 are independent in function. C3 could go first if the SDK work is more urgent, but S2 needs
both.

## Objective gates (whole design)

1. **Class A.** Equivalent sessions (today's identical per-track chains against the console form)
   produce identical render digests. Where no rack moved in the migration, they also produce
   identical canonical graph text, by construction through the lowering. This is PR evidence in S1a,
   and S4 checks it end to end through `output_sha256`.
2. **Always banked.** Every console slot binds banked at N in {1, 3, 5, 9, 10, 13}, with mixed
   bypass, mixed insert counts and both pool classes. Simd8 runs on x86-64, and Simd4 runs through
   `scripts/run-aarch64-tests.sh` or the wasm gates (S2). Banked, padded and per-node renders are
   bit-identical per track.
3. **Realtime.** Render stays allocation-, lock- and syscall-free, and the realtime audits and
   callgraph gates pass.
4. **Schema.** Strict parse, canonical round trip, and refusal of:
   - per-track console effect fields;
   - unknown, missing, duplicate or misordered slots;
   - a console sidechain;
   - an ineligible effect;
   - every retired token and code.
5. **Evidence.** S4's benchmarks are reported against S0's.

## Follow-ups (not in these slices)

- A per-lane silence skip, if S4's sparse-activity row warrants it.
- An all-lanes-bypassed skip for a bank, which must still feed latency lines.
- ALAP alignment of `post_insert` banks, on measured need.
- A banked sidechain port (#96 F9).
- Multiband padding and eligibility after #1069.
- Performance issues that matter more under this design: #887 (tiled partial gather and scatter,
  amended) and #892 (the dry line feed when nothing is bypassed).
- Identity-slot insert cohorts: #888 and #889, amended to that half.
- #1102: put a limiter bank back on its uniform body after a partial recovery (#1091 verdict L1).
- One shared per-lane D7 recovery: #1089-#1092 each recover per lane in their own crate (for
  example `compressor::finish_lanes`). The frame-zeroing half may belong beside
  `effect_runtime::bank::nonfinite_lane_mask`, in the spirit of #95's ratchet (#1089 verdict L4).
  File it when a fifth copy would otherwise appear.
- Multiband per-lane D7, which lifts its prepared session bypass (#1100) once #1069 closes.
- #1104: make the session parse-transient bound affine (#1093 verdict, low 4).

## App (misofm/app; out of this repo's scope)

After S1d, the app moves EQ -> compressor from every track's dynamic rack into two
`console.pre_insert` slots, and each track's entry keeps its bypass for "unselected". S1r's and
S1d's handoff notes carry the name maps.

## Disposition of Sol's amendments

| # | Amendment | Disposition |
|---|---|---|
| 1 | Bypass through the shunt | Accepted (owner). P1 |
| 2 | Slot declaration: `slot`, `identity`, `quality`, `link_mode`; no sidechain | Accepted (owner). S1a |
| 3 | Grouping per (slot, pool class, level), padded | Accepted (owner). S2 |
| 4 | Per-track console array in slot order | Accepted (owner). S1a |
| 5 | Wire identity, in-place V1 | Accepted (owner): no `ABI_VERSION` bump. S1a, S1b, S1c, S1r |
| 6 | Padding contract | Accepted. P2a-P2e |
| 7 | Coupling rule | Accepted (owner). AGENTS.md; P2b-P2e, S2 gates |
| 8 | Eligibility list and diagnostic | Accepted (owner). S1a |
| 9 | No-fallback rule | Owner: always banked, no threshold. S2 |
| 10 | #971 under padding | Left to S2, with the question stated |
| 11 | Silence | Accepted (owner): the trade-off stands; sparse row in B0; skip deferred |
| 12 | App statement | Corrected above; app shape in B0 |
| 13 | Vocabulary | Owner: session key `console`; the internal live-console names and the two exports renamed (S1r). Root's application: the boot words and the public SDK API. `_v1` stays. `builtins`/`"strip"` unchanged (R0's reading) |
| 14 | Class A by lowering | Accepted (owner). S1a |

## Issue map

| Slice | Issue |
|---|---|
| 00 | #1084 |
| B0 | #1085 |
| S0 | #1086 |
| P1 | #1087 |
| P1b | #1100 |
| P2a | #1088 |
| P2b | #1089 |
| P2c | #1090 |
| P2d | #1091 |
| P2e | #1092 |
| S1a | #1093 |
| S1b | #1094 |
| S1r | #1095 |
| S1c | #1096 |
| S1d | #1097 |
| S2 | #1098 |
| S4 | #1099 |

## Sol verdict, R0

**FAIL.** Sol checked R0 (`5c58f280`..`6fdf5db2` on `5a1421a3`) against the owner's decisions of
2026-09-29, `AGENTS.md`, and the code at `6fdf5db2`.

The ruling and the AGENTS.md amendment are sound. The slice plan is not: the final slice cannot run
the before/after measurement the owner asked for, and one allowed merge order leaves a slice with a
gate it can never pass. Both need only small spec edits, so R0 attempt 2 is a spec-only revision.

**What passed.**
- **Decision 12** states every owner decision listed in the brief:
  - the slot declaration and the per-track `console` array;
  - `inserts`, with no console sidechain;
  - always banked, padded, with no threshold;
  - per-lane bypass through the shunt;
  - the silence trade-off, accepted and measured;
  - the session key and the rename;
  - the in-place V1 amendment, with tap codes 1-7 renamed, `dynamic` -> `inserts` keeping code 2
    and field 7, `simd1`/`simd2` retired and `console` appended;
  - class A by lowering.
- **AGENTS.md** reads consistently.
  - The tap codes are unchanged, so "stable enum values" still holds.
  - The #163 sentence is now scoped to inserts.
  - The third-party paragraph says "insert only".
  - The coupling clause restates the existing rule.
  - Nothing claims a later generation.
- **The dependency graph** has no cycle.
  - B0 merging before P1, P2a and S1a, with S0 timed on B0's commit, measures the unchanged engine.
  - The no-push window from S1a to S1d is sound. The SDK writes `simd1`/`dynamic`/`simd2`
    (`sdk/src/internal/session-json.ts:113-117`) until S1d.
- **The #887, #888 and #889 amendments** are correct and leave all three closable.
- **S1r:** the two exports are sealed contract identity.
  - They are class 2 in `docs/rulings/de-versioning-inventory.md:84`.
  - They are pinned in `scripts/check-abi-layout-v1.py:116-117` and the layout JSON.
  - Renaming the stem while keeping `_v1` is lawful under the version-suffix rule, which governs
    only the suffix and forbids only a later generation. #1063 and #313 are the in-place
    precedents.
- **S1a's migration rule** was checked on all 18 sessions: only `reduced-nobus-from-970-verify.json`
  folds. Sol read every bank factory the eligibility list names. Each declines only on a
  program-key, width or sidechain mismatch, so S2's "unreachable for valid sessions" holds.
- **Gates.** All exited 0 at `6fdf5db2`:
  - `check-workspace-policy.sh` and `check-session-policy.sh`;
  - `check-script-reachability.py`;
  - `check-ci-path-routing.py` and `test-ci-path-routing.py`;
  - `check-env-vocabulary.sh`;
  - `check-dsp-research.sh` and `check-builtins-listening.sh`.

  The route is `full`, because AGENTS.md changed.

### Findings, ranked by severity

**High: S4 has no owner for the owner-required after-measurement.** The owner's draft asks for
native and V8 rows, rerun "on the new shape". Two changes that measurement needs belong to no
slice.

- **The app-shape row never moves into the console.**
  - S1a's migration sends every `dynamic` rack to `inserts` (S1a, checkpoint 2).
  - S4 changes no code, yet it says that the app shape "now puts EQ -> compressor in
    `console.pre_insert`".
- **The V8 harness breaks, and no gate would notice.**
  - What reads the old shape: `scripts/web-mixing-automation-benchmark.mjs:136-140` reads
    `track[["simd1","dynamic","simd2"][control.rack]]` from `console-sixty-four-track-mono.json`.
  - What writes the old codes: `:279` writes `control.rack` (0, 1 or 2, from
    `tools/console-workload/examples/mixing_automation_controls.rs`) into the record's rack byte.
  - What breaks it: S1a migrates that fixture, and S1c retires rack bytes 0 and 2.
  - Why it goes unnoticed: neither slice is authorized to touch these files, and no CI gate runs
    the real harness (`scripts/test-console-benchmark.sh` stubs it).
  - The consequence: S4's gate 1 fails at preflight.

*Fix:*
- S1a owns the app-shape builder's move into `console.pre_insert`, as a stated exception to the
  mechanical rule that keeps the 2-mod-3 bypass pattern, and it owns the V8 harness's fixture
  lookup.
- S1c owns the harness's and the controls example's rack codes.
- Both add these untimed gates:
  - `bash scripts/run-web-mixing-automation-benchmark.sh preflight WORKDIR`;
  - `bash scripts/operator/preflight-console-benchmark.sh --step <scratch>`.

**Medium: S1r's gate 3 is impossible if S1a merges first.** The umbrella lets S1a precede S1r:
"implemented in parallel", with "S1r merges before S1c and S1d". After S1a, the SDK suites,
`check-sdk-generated.sh` and the SDK-driven browser qualification are red until S1d.

*Fix:* state "S1r merges before S1a".

**Medium: a required gate breaks and no slice is authorized to fix it.**
- `scripts/check-console-benchmark-fixture.sh` is the required qualification step "Console
  benchmark fixture integrity". It reads `simd1`/`dynamic`/`simd2` and `tap == "post_matrix"`
  from `console-sixty-four-track.json`, which S1a migrates.
- S1a neither authorizes nor gates it.
- `scripts/check-console-fixtures.sh` is one of S1a's gates, but it is not an authorized path.

*Fix:* add both scripts to S1a's authorized paths and gates.

**Medium: attribution in decision 12's "Naming".** The ruling presents the rename of the sealed
exports, the four boot words and the SDK's public live-console API as the owner's ruling. The owner's
decision, as briefed, renames the *internal* live-console names.

*Fix:* either cite the owner's words, or mark S1r checkpoint 2 as R0's reading pending owner
confirmation, as the ruling already does for `builtins`/`"strip"`.

**Medium: no batch plan.** "The batch is not pushed until S1d's gates pass" implies one batch from R0
through S1d: twelve or more slices, over several days, with no closed issue. Name the boundaries:

1. R0 and B0, with S0's records;
2. P1 and P2a-P2e;
3. S1r, S1a, S1b, S1c and S1d, pushed once;
4. S2 and S4.

**Low.**
- **The parallelism note is wrong.** "P1, P2a, S1a and S1r touch different crates" is not true:
  - P1 and P2a both edit `effect-contract/src/lib.rs` and `graph-compiler/src/banks.rs`;
  - P1, S1a, S1r and S1c all edit `effect-compiler/src/prepare.rs`;
  - P1 and S1r both edit `rack/src/lib.rs`.

  Sequence them.
- **S2's gates conflict.** Gate 2 renders console slots per node, but the guarantee and gate 3 forbid
  a per-node console plan. Scope `console.slot.unbanked` to production backends and exempt the
  test-only `Scalar` oracle (#1059).
- **Unowned docs and specs.**
  - `docs/BUILTINS_AND_METERING_V1.md:4, 171-174` spells the old taps. S1a should own it.
  - The open specs #973 and #987 use the rack vocabulary.
- **Stale text.**
  - Decision 12 still says "until root numbers it".
  - The umbrella's Slices section still says "Root renumbers the files once the GitHub issues exist".
- **#887's dependency.** Its amendment says "Dependencies. Still none", but its new gate 4 reads B0's
  N = 10 and 13 rows.
- **S1c's gate 4 does not say how to run it.** Run the qualification without `--sdk-root`
  (`hosts/host-web/qualification/run.mjs:837-858`) for the raw-export subset.
- **GitHub bodies lag.** #1084 has the old `CONSOLE-*` filenames and no issue map, and #887-#889
  have no amendments. Sync them after the batch push.
- **The draft verification sits in the wrong folder.** `DRAFT-console-strip-VERIFY.md` is in
  `.github/ISSUE_SPECS/`, which the README keeps for open-issue specs only (#1040). Move it when
  convenient.

## Attempt 2 (R0)

Terra, 2026-09-29, a spec-only revision answering Sol's R0 verdict above.

- **High (S4 had no owner).**
  - S1a's migration rule now takes a uniform `dynamic` rack into the console (two or more tracks;
    two named placement witnesses excepted), which moves the app-shape row, native and V8, into
    `console.pre_insert` with its bypass pattern.
  - S1a owns the V8 harness's fixture lookup and the benchmark builders. S1c owns the harness's and
    the controls example's rack codes.
  - Both S1a and S1c now gate on the untimed console preflight and the V8 `prepare` plus
    `preflight`.
  - S4 lists every row with the slice that moved it, and gates on `output_sha256` equal to S0's.
- **Medium.**
  - S1r merges before S1a (S1a, S1c, S1r and the table).
  - S1a is authorized for `check-console-benchmark-fixture.sh`, `check-console-fixtures.sh`,
    `test-console-benchmark.sh` and any other gate input the schema breaks.
  - Decision 12's naming is re-attributed: the internal names and exports are the owner's decision;
    the boot words and public SDK API are root's application, and `_v1` stays.
  - The batch plan (C1-C4) is above.
- **Low.**
  - The parallelism note is replaced by file-based sequencing.
  - S2's `console.slot.unbanked` covers production backends only, and the test-only `Scalar` oracle
    (#1059) is exempt.
  - S1a owns `docs/BUILTINS_AND_METERING_V1.md` and the open specs #973 and #987.
  - The stale "until root numbers it" and "Root renumbers" text is gone.
  - #887 depends on #1085.
  - S1c's gate 4 says to run without `--sdk-root`.
  - The design verification moved to `docs/handoffs/console-strip-2026-09-29/VERIFY.md`.
  - The GitHub bodies (#1084-#1099, and the #887-#889 amendments) are root's to sync after the C1
    push.

## Sol verdict, R0 attempt 2

**PASS.** Sol checked `11ea359b` against the attempt-1 verdict, the owner's decisions and the code
at that commit.

**Attempt-1 findings, each closed.**
- **High (S4 had no owner).**
  - S1a's `dynamic` clause moves B0's app-shape row, native and V8, into `console.pre_insert`.
    Bypass is not part of the "uniform" test, so the row qualifies, and the 2-mod-3 pattern
    survives.
  - Chain order is preserved in every case of the append, prepend and stay rule.
  - S1a owns the V8 fixture lookup and the builders. S1c owns the rack codes. Both gate on the
    native console preflight and on the V8 harness's `prepare` and `preflight` steps.
  - S4 maps every row to the slice that owns it.
- **Medium, merge order.** S1r now merges before S1a.
- **Medium, authorization.** S1a is authorized for, and gated on, `check-console-benchmark-fixture.sh`,
  `check-console-fixtures.sh` and `test-console-benchmark.sh`.
- **Medium, attribution.** The owner's decision and root's application are now separated, and
  `_v1` stays. Sol cannot see the owner's question text in the repo, so this is accepted as root
  attests it.
- **Medium, batch plan.** C1-C4 works under CI-conscious delivery:
  - each batch is pushed once and its issues close after the push;
  - C1 is pushed before any engine slice lands;
  - C3's single push is forced by the SDK lockstep;
  - the file-based sequencing inside each batch is correct.
- **Low.** Every low finding is closed.

**The deviation is sound.** Sol reclassified all 18 documents with a script. The only multi-track
documents with a uniform, non-empty `dynamic` rack are the two named witnesses. Neither blocks S4:
- `console-sixty-four-track.json` feeds only `sixty_four_track_console_legacy` and its twin,
  `sixty_four_track_eq_comp_simd1`.
- The headline `sixty_four_track_console` row and B0's strip and sparse-activity rows use the
  `-intended` fixture (`tools/console-workload/src/lib.rs:582-593`). That fixture becomes fully
  console, so S4 measures the console model at N = 64.
- Keeping the witnesses preserves the console-against-insert pairs that AGENTS.md's placement rule
  needs.

**Gates.** All exited 0 at `11ea359b`, on the `full` route:
- `check-workspace-policy.sh` and `check-session-policy.sh`;
- `check-script-reachability.py`;
- `check-ci-path-routing.py` and `test-ci-path-routing.py`;
- `check-env-vocabulary.sh`;
- `check-dsp-research.sh` and `check-builtins-listening.sh`.

**Low findings.** None blocks R0; fix them in passing.
1. **S1c's harness authorization is too narrow.** It says "the rack byte only" (`:279`), but after
   S1c's codes change, the lookup at `:136-140` also keys on `control.rack`. Authorize S1c for the
   harness's rack-code handling, or have S1a key the lookup on `slot_id`.
2. **S4's gate 5 names the wrong V8 fields.** The V8 records carry `quiet_output_sha256`,
   `restated_output_sha256`, `automated_output_sha256` and `preflight_output_sha256`, not
   `output_sha256`. Name them.
3. **S4's gate 5 has no allowance for intervening bit changes.** Allow a difference that is
   attributed to a named non-console commit that declared a bit change between C1 and C4.
   Otherwise an unrelated class-B fix makes the gate unpassable.
4. **Typo.** The P2b-P2e dependency lines read "(P2a), #1088)".
5. **Optional.** S1r's gates pass on the old schema, so it could close C2 instead and shorten C3's
   unpushed window.

## C2 boundary evidence

**PASS, nothing fixed.** Terra checked the merged batch once, as `qualification.yml` would. The
batch is `codex/batch-console-2` at `51a1514a` on `origin/main` `223330bc`: P1 #1087, P1b #1100,
P2a #1088, P2b #1089, P2c #1090, P2d #1091 and P2e #1092, plus root's doc, test and ratchet
commits.
- The router gives `route=full`, `math_closure=true`, `release_inputs=true` and `self_tests=[]`
  (`ci-path-router.py --flags`, base `223330bc`).
- 95 steps ran on an x86-64-v3 host (AMD EPYC 7313P) with rustc 1.97.1, `CARGO_INCREMENTAL=0` and
  the worktree's `target/`. Every step exited 0.
- No merge-interaction defect was found and no commit was needed beyond this record.

| Job | Gate (command) | Result |
|---|---|---|
| route | `check-ci-path-routing.py`, `test-ci-path-routing.py` | ok |
| docs-gates | `check-dsp-research.sh`, `check-builtins-listening.sh` | ok |
| lint | `cargo fmt --all -- --check` | ok |
| lint | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| lint | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| lint | every `check-*`/`test-*` step of the job: workspace, session, env-vocabulary, bench policy + `test-bench-policy.sh`, test-support CI, scalar-oracle self-test, script reachability, console-benchmark fixture, builtins-fixture mutations, bench preconditions, host-core, protocol-control, realtime + audit-leak + artifact-evidence-leak, trace validator, lane, unfused seal, rack/builtins/graph, effect-runtime policy + fixtures, conformance boundaries, EQ render contract, release-shape self-test, npm publish modes, stem store, stem-identity corpus | all ok |
| lint | the three sub-v3 probes (scalar and AVX2-without-FMA refused with "requires x86-64-v3"; AVX2+FMA compiles) | ok |
| test-debug-a | `cargo test --locked --workspace --all-targets --exclude …` with the job's features | 93 binaries, 1109 passed, 0 failed |
| test-debug-b | `cargo test --locked --all-targets -p lane … -p conformance` with the job's features; `conformance_fixtures -- --check` | 151 binaries, 834 passed, 0 failed; fixtures ok |
| test-release | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | 109 passed, `g5_native_digests_match_pins` ok |
| test-release | M3 FMA cfg; loom `spsc_loom`; M1 `m1_exhaustive` and F1 `f1_fast_db_bounds` `--ignored` (math closure) | ok |
| audit-native | release build; `cargo test --release -p audit -p bench -p console-workload` | 114 passed, 0 failed |
| audit-native | `audit capi` and its record validator; `audit delay`, `compressor` and `parametric-eq` at 100,000 blocks; `audit gate-expander` (bank width 8, `bank_available`) | every counter 0, `total_violations` 0 |
| audit-native | builtins, builtins-graph, graph, realtime and effect-contract traces at 1,000,000 blocks; protocol allocation audit; realtime, builtins and builtins-graph probe mutations | PASS |
| audit-native | `check-capi-abi.sh` and `--self-test`; scalar oracle absent from `libcapi.so`; graph determinism 100/100; builtins fixtures (50 files); console fixtures; `check-effect-contract.sh` (8 factories, 0 failed gates) | ok |
| release tests, affected crates | `cargo test --locked --release` over compressor, gate-expander, transient-shaper, soft-clip, true-peak-limiter, parametric-eq, multiband-compressor, effect-runtime, effect-contract, delay, graph-compiler, graph, rack, effect-compiler, host-core, conformance and bench-support, with the test-support features | 172 binaries, 1166 passed, 0 failed |
| console digests | `cargo test --release -p console-workload --test gain_pan_profile digests -- --ignored` at the head and at `223330bc` (a `git archive` in scratch, with its own target directory) | **22 of 22 rows identical** |
| console benchmark | `scripts/test-console-benchmark.sh` | PASS |
| wasm-guests | runner build; simd128 probe; evidence crates; `check-protocol-wasm-parity.sh`; `run-wasm-gates.sh --without-v8-spill --without-native` | ok |
| cross-target | `scripts/check-cross-targets.sh` | PASS; the iOS `memset_pattern16` counts equal every ceiling (parametric-eq 146) |
| release-shape | `check-release-shape.py`; `CARGO_PROFILE_RELEASE_PANIC=unwind cargo check --locked --release --workspace --all-targets` | ok |
| artifact | `build-web-audioworklet.sh` | module `f767076a…09e8`, closure `2e10e721…4edf` |
| artifact-identity | a twin `--module-only` build from another path and `CARGO_HOME`; `web-audioworklet-identity.py --self-test` and `report --event push --before 223330bc` | reproducible; **ARTIFACT CHANGED** |
| artifact-gates | `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, scalar oracle absent from the module, `test-web-audioworklet.sh`, V8 spill self-test and gate (Node 22.23.2) | ok |
| sdk | `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh` (284 passed), `sdk-package.sh check` | ok |
| browser | `npm run qualify -- … --check-matrix --self-test-mutations` for Chromium 151, Firefox 153 and WebKit 26.5 | all qualification gates passed |
| aarch64-debug/-release | see below | resolved; not run (no arm64 host) |

**Artifact.** ARTIFACT CHANGED:

| | main `223330bc` | batch `51a1514a` |
|---|---|---|
| Module digest | `885aa117ed0133f325a08992e0df0265e2984b1b63a163b8a8708722322c6545` | `f767076a03548350a35dc6d758716c328f108de59a40cb31442ace59435a09e8` |
| Size | 3,254,230 B | 3,270,838 B |

- The batch module is 16,608 B (0.51 %) larger. P1 and P2 change the effect kernels, so a change is
  expected.
- The main digest is the `audioworklet-sha256` status that main's own run recorded. A local
  `--module-only` build of `223330bc` reproduces it byte for byte.
- The committed pin (`6c952a2c…`) was not touched. Under #1061 it is checked only at release.

**AArch64 legs.** They cannot run on this host. Resolved:
- **Debug leg.** The 25 product crates (`capi`'s closure, from `scripts/lib/product-crates.sh`)
  plus `dsp-reference`, `conformance` and `target-smoke`. It has no expected-failure rows.
- **Release leg.** `lane` and `math` with `math/lane`, then `console-workload`, then the capi,
  delay, compressor, parametric-eq and gate-expander audits. It has two #1019 rows:
  `m2_exp2_lane_identity` and `m2_log2_lane_identity`, both in `math`'s `m2_lane_identity`.
- **Checks run here.**
  - The no-silent-skip scan (rg exit 1, no match).
  - The known-defect self-test.
  - `judge-skips` over each leg's `--list`: debug 1783 tests, release 114 tests. The listings come
    from x86 builds of the same packages and features. Every row names exactly one test.

**Local deviations, none a gate change.**
- The artifact directory is in scratch rather than `target/ci/qualification-artifacts`.
- The twin build's second checkout is a `git archive` of `HEAD`.
- The identity report ran as the main push will: `--event push --before 223330bc`.
- The browser legs' pulse socket and Chromium's `TMPDIR` needed short `/tmp` paths. The scratchpad
  path is longer than the 108-byte Unix-socket limit. The first Chromium attempt aborted on that
  ("Socket path too long") before any gate ran.
