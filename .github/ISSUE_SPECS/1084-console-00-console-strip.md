# Console strip: session-level console effects with per-track inserts

The owner stated this design on 2026-09-29. It emulates a hardware mixing console: the console's
built-in channel processing is the same on every channel, and each channel sets only its own
knobs. Outboard gear is patched into a channel's insert point and differs per channel.

**Provenance.**
- Sol verified the draft and returned SOUND WITH AMENDMENTS
  (`.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`, commit `03aceb94`; findings H1-H5, M1-M9,
  L1-L7; amendments 1-14).
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
- **Naming** (M6, amendment 13). The session key is `console`. The existing "console" names for the
  live-control attachment become "live controls":
  - `HostConsoleRequest`/`HostConsoleHandles`, `ConsoleEffectBankStage` and the rest;
  - the sealed `miso_engine_web_v1_console_track_*` exports, whose `_v1` stays and whose retired
    spellings are never exported again;
  - the four `console*` boot words;
  - the SDK's live-console API.

  The console benchmark and fixture names stay. `builtins`/`"strip"` is unchanged.

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

### #971 under padding

Once every console group binds, the stranded-mono demotion's "binds more banks" objective measures
nothing (M3). S2 decides between restating it (for example, minimise planes x banks) and retiring
it for console slots, and records why (amendment 10). The mono pool is worth about 36 % on the
standing console row, so the choice must not silently demote mono tracks.

## Slices

Each slice is its own issue with one adversarial verdict. Docs travel with the slice that changes
the behaviour. Root renumbers the files once the GitHub issues exist.

| Slice | File (`.github/ISSUE_SPECS/`) | Scope | Depends on |
|---|---|---|---|
| R0 | this commit | Decision 12, the AGENTS.md amendment, this umbrella and the slice specs | none |
| B0 | `1085-console-b0-add-the-console-strip-benchmark-rows.md` | Strip at N in {9, 10, 13, 16, 64}, app shape, sparse activity; layout-neutral `strip_layout`; two V8 documents | R0 |
| S0 | `1086-console-s0-record-the-console-strip-baseline.md` | One native and one V8 run on B0's commit | B0 |
| P1 | `1087-console-p1-keep-a-bypassed-lane-in-its-effect-bank.md` | Session bypass -> shunt state; mixed-bypass cohorts bind one bank | R0 |
| P2a | `1088-console-p2a-let-an-effect-bank-bind-a-partial-group.md` | Active mask, padding contract, planner support; every factory still declines | R0 |
| P2b | `1089-console-p2b-pad-parametric-eq-banks.md` | EQ opts in (supersedes #888's absent-member half) | P2a |
| P2c | `1090-console-p2c-pad-compressor-banks.md` | Compressor opts in (supersedes #889's absent-member half) | P2a |
| P2d | `1091-console-p2d-pad-true-peak-limiter-banks.md` | Limiter opts in; the clone keeps the fast body | P2a |
| P2e | `1092-console-p2e-pad-gate-transient-shaper-and-soft-clip-banks.md` | Gate, transient shaper and soft-clip opt in; gate defaults fixed; soft-clip D7 masked | P2a |
| S1a | `1093-console-s1a-add-the-session-console-and-inserts.md` | Grammar, model, validation, canonical writer, BTLV, lowering; migrate 18 documents; repin | R0 |
| S1b | `1094-console-s1b-carry-the-console-and-inserts-in-the-control-protocol.md` | Console session edits, retired codes, registry docs, `COMPLETE_SCHEMA_HASH` | S1a |
| S1r | `1095-console-s1r-rename-the-live-console-to-live-controls.md` | Rename the live-control attachment's names, exports, boot words and SDK API | R0 |
| S1c | `1096-console-s1c-address-console-slots-and-inserts-in-live-control.md` | Live addressing, the browser record's rack byte, the observation, live-response and spectrum encodings | S1a, S1b, S1r |
| S1d | `1097-console-s1d-ship-the-session-console-and-inserts-in-the-sdk.md` | SDK builder, types, writer, live controls, CLI, author-session skill, app handoff | S1a, S1c |
| S2 | `1098-console-s2-bind-every-console-slot-banked.md` | Padding policy for console slots, the no-fallback diagnostic, #971 | P1, P2b-P2e, S1a |
| S4 | `1099-console-s4-measure-the-console-strip-against-its-baseline.md` | Rerun B0's rows; before/after report | S2, S1d, S0 |

The rename (S1r) is its own slice rather than part of S1c: it reaches about 80 code, script and SDK
files, including sealed exports, and would triple S1c while mixing a class-A rename into a
semantic verdict (decision 12).

**Merge order and batch notes.**
- B0 merges before any engine slice (P1, P2a, S1a), so that S0 times the engine unchanged. S0 may
  run later on a clean detached worktree of B0's commit.
- P1, P2a, S1a and S1r touch different crates and may be implemented in parallel, subject to
  AGENTS.md's one-feature WIP limit.
- S1r merges before S1c and S1d.
- Between S1a and S1d, the SDK and the SDK-driven browser qualification are knowingly out of step
  with the engine. The batch is not pushed until S1d's gates pass.

## Objective gates (whole design)

1. **Class A.** Equivalent sessions (today's identical per-track chains against the console form)
   produce identical canonical graph text and render digests. This is PR evidence in S1a, and by
   construction through the lowering.
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
| 13 | Vocabulary | Owner: session key `console`; live-control names renamed (S1r); `builtins`/`"strip"` unchanged (R0's reading) |
| 14 | Class A by lowering | Accepted (owner). S1a |

## Issue map

| Slice | Issue |
|---|---|
| 00 | #1084 |
| B0 | #1085 |
| S0 | #1086 |
| P1 | #1087 |
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
