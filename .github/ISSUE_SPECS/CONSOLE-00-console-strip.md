# Console strip: session-level console effects with per-track inserts

Owner design, stated 2026-09-29. It emulates a hardware mixing console: the console's built-in
channel processing is the same on every channel, and each channel only sets its own knobs.
Outboard gear is patched into a channel's insert point and differs per channel.

## Problem

The engine does not implement this today.

- **Racks are per track.** `docs/SESSION_SCHEMA_V1.md` and `crates/session/src/model.rs` (`Track`)
  give every track its own `simd1`, `dynamic` and `simd2` racks. Nothing makes the SIMD racks
  session-level, and nothing stops tracks from diverging.
- **Banking is opportunistic.** It groups whichever tracks happen to have identical chains.
- **AGENTS.md treats racks as interchangeable.** The #163 phase 1 sentence says bank eligibility
  is "never by which rack the session placed it in".
- **The app uses the dynamic rack for everything.** `misofm/app` puts EQ → compressor in each
  selected track's dynamic rack and leaves the SIMD racks empty.
- **Effect remainders never bank.** An effect bank binds only a full group: every effect factory
  refuses `requests.len() != lanes`, and the contract has no inactive-lane mask (#96 F7,
  `crates/graph-compiler/src/banks.rs`). Leftover tracks run each effect one lane at a time.
  Builtins already pad partial banks.

## Design

### Session shape

```json
"console": {
  "pre_insert":  [ { "slot": "eq",   "effect": <native effect id>, "quality": ... },
                   { "slot": "comp", "effect": <native effect id>, "quality": ... } ],
  "post_insert": [ { "slot": "limiter", "effect": <native effect id>, "quality": ... } ]
},
"tracks": [
  { "id": "vox",
    "console": { "eq": { <per-channel parameters>, "bypass": false }, "comp": { ... },
                 "limiter": { ... } },
    "inserts": [ <per-track effect declarations, as today's dynamic rack> ],
    ... }
]
```

- **`console` (session level).** It declares each slot's effect type, order and quality once.
  Every track has every slot.
  - `pre_insert` runs before the insert point and `post_insert` after it.
  - Either list may be empty, and an empty list costs nothing.
  - Only native effects that carry the homogeneous bank kernel contract are allowed.
- **Per-track `console` block.** It holds one entry per console slot: that track's parameters
  (per dual-mono channel, as effect parameters are today) and its `bypass`. A track cannot add,
  remove or reorder console slots. Switching a slot off is `bypass`, so the track stays in the
  bank.
- **`inserts` (per track).** Replaces today's `dynamic` rack, with the same semantics. It may be
  empty.
- **Removed:** the per-track `simd1`, `dynamic` and `simd2`.

### Chain and tap points

```
input → input section (trim, polarity, HPF/LPF) → console.pre_insert → inserts
      → console.post_insert → fader/mute → pan/matrix → routes
```

The seven send taps are renamed; their positions are unchanged.

| Today | New |
|---|---|
| `input` | `input` |
| `post_input_builtins` | `post_input` |
| `post_simd1` | `insert_send` |
| `post_dynamic` | `insert_return` |
| `post_simd2_pre_fader` | `pre_fader` |
| `post_fader` | `post_fader` |
| `post_matrix` | `post_pan` |

Automation targets and live control address console slots by `(track, "console", slot,
parameter)` and inserts by `(track, "inserts", effect_id, parameter)`. The builtins token is
revisited alongside the rename.

### Banking guarantee

- Every console slot has one bank signature across all tracks of a pool class. Dual-mono gives
  mono and stereo pool classes (#971).
- The compiler must bind every console slot banked, with no per-node fallback, on every target.
- Partial groups are padded with inactive lanes. That needs an effect-contract extension:
  - bank factories accept `members ≤ lanes`;
  - inactive lanes get a neutral configuration and silent (+0.0) input, and their output is
    discarded;
  - no link may cross from a dummy lane to a real lane.

  Class A must hold: a track's output bits are identical whether its lane is banked, padded or
  per node.
- `inserts` keep today's behaviour: they bank where tracks happen to match, otherwise per node.

### Cost trade-off (to be measured, not assumed)

In the console model every track runs every console slot, and a bypassed slot still occupies its
lane. Cost therefore scales with `ceil(tracks / lanes)` banks per slot, whatever the bypass
states. Two things need measuring:

- today's app shape, with effects only on selected tracks, per node or partially banked;
- a bank-wide skip when every lane in a bank is bypassed, as a possible later optimisation.
  It is not in scope unless measurement demands it.

## Slices (each is its own issue: implement, then adversarially verify)

- **S0, baseline benchmarks.** Freeze console-strip workloads before any change: EQ → compressor →
  limiter on N ∈ {10, 13, 16, 64} tracks, plus an app-shaped workload with effects on a subset of
  tracks. Record native `--step` rows and V8 rows on the shipped artifact, under the timing lock.
  Follow AGENTS.md's benchmark rules: one warmup, two measured rounds, no tuning.
- **S1, schema, model and lowering.**
  - Session-level `console`, per-track `console` knobs and bypass, and `inserts`.
  - The parser, canonical writer, validation, lowering, SDK types, capi session paths, fixtures
    and docs.
  - Tap and automation renames happen here too, so the schema changes once.
  - Every console slot must bind banked, full groups at least.
- **S2, effect-bank padding.** Partial groups bind with inactive lanes, so every console slot banks
  for every track count.
- **S3, AGENTS.md and docs.** The chain line, the #163 sentence's scope (inserts only), the tap
  enum and the console vocabulary.
- **S4, benchmarks.** Rerun S0's workloads on the new shape and report before and after.
- **App (misofm/app, separate repo, out of this repo's scope).** Move EQ → compressor from
  per-track dynamic racks into `console.pre_insert`. Hand off with the SDK change notes.

## Objective gates (whole design)

1. Class A: console digests and every class-A differential are unchanged for equivalent sessions,
   meaning today's identical per-track chains against the new console form.
2. Every console slot binds banked for every track count once S2 lands, proved by a test at
   N ∈ {1, 3, 5, 10, 13} on Simd4 and Simd8.
3. Render stays allocation-, lock- and syscall-free; the realtime audits and callgraph gates pass.
4. Schema: strict parse, canonical round trip, and refusal of per-track console effect declarations
   and of unknown slots.
5. S4 benchmarks are reported against S0.
