---
name: author-session
description: Author, extend, or repair a strict Session V1 canonical JSON document and prove it with the real grammar, typed-model, compile, builtins, and native-effect preparation pipeline.
---

# Authoring a Session V1 document

Run commands from the repository root. A session is one strict JSON document; JSON is the sole
live Session V1 format. There are no aliases, comments, trailing commas, duplicate keys, format
sniffing, or TOML translation. Unknown keys reject.

## Read the authorities

1. Read `docs/SESSION_SCHEMA_V1.md` end to end, especially "Session console and inserts".
2. Start from the worked session beside this file, `.claude/skills/author-session/worked-session.json`:
   a two-track strip with both console sections, a per-track insert, a keyed insert, a submix and a
   console automation target. It passes all five validator stages and is canonical. For more,
   copy structure from `fixtures/session/v1/`: `canonical-minimal.json` (empty console),
   `console-sixty-four-track-intended.json` (the production strip: EQ -> compressor, then a
   limiter) and `console-sixty-four-track-app.json` (the app's EQ -> compressor with unselected
   tracks bypassed). `fixtures/session/v1/canonical.json` is a schema example with an unregistered
   effect ID and deliberately fails effect preparation.
3. Generate parameter metadata rather than guessing effect IDs, parameter IDs, units, domains, or
   defaults:

   ```sh
   cargo run -q -p parameter-metadata -- --print
   ```

The fourteen root keys are `schema_version`, `session_id`, `revision`, `sample_rate_hz`,
`quantum_frames`, `render_profile`, `output_profile`, `sources`, `console`, `tracks`, `submixes`,
`outputs`, `routes`, and `automation`. Every field and empty array is explicit. Durable unsigned
64-bit values (`revision`, source `frames`, automation `start_sample`/`end_sample`) are canonical
decimal JSON strings: no sign, whitespace, leading zero except `"0"`, or value above
`18446744073709551615`.

## The console strip and inserts

The chain is `input -> input section (polarity, trim, HPF/LPF) -> console.pre_insert -> inserts ->
console.post_insert -> fader/mute -> pan/matrix -> routes`.

- **The session console** is declared once, at the root, between `sources` and `tracks`:
  `"console": { "pre_insert": [...], "post_insert": [...] }`. Each slot is exactly
  `{ "slot", "identity", "quality", "link_mode" }`. `slot` is a stable ID unique across **both**
  sections. A slot has no `sidechain`, `bypass` or `params`. Either section may be empty.
- **Eligibility.** A console slot is one of `miso.parametric-eq`, `miso.compressor`,
  `miso.gate-expander`, `miso.soft-clip`, `miso.transient-shaper`, `miso.true-peak-limiter`.
  The delay, the multiband compressor, third-party (`cid`) effects and anything keyed by a
  sidechain are inserts.
- **Every track carries every slot**: its `console` array holds exactly one
  `{ "slot", "bypass", "params" }` entry per slot, in slot order (`pre_insert`, then
  `post_insert`). An entry carries only the track's knobs; a bypassed entry still runs (and still
  pays its latency). A track cannot add, drop or reorder a slot.
- **`inserts`** is `{ "effects": [...] }`: the track's own full effect declarations, in chain
  order, with `sidechain` (normally `{"kind":"none"}`; a keyed effect uses
  `{"kind":"routed","source":{...},"port_id":"sidechain-in"}`). It may be empty. An insert ID may
  equal a slot ID; they live in different racks.
- **Retired**: the per-track `simd1`, `dynamic` and `simd2` keys. Each refuses as
  `schema.unknown_field`; never write them.

## Core shapes and vocabularies

- A source has exactly `id`, `content`, `channels`, `bit_depth`, and `frames`. `content` is
  `blake3:` plus 64 lowercase hex digits; `frames` is a nonzero decimal string; `bit_depth` is
  `16`, `24`, or `"32f"`.
- A submix and output are respectively `{"id":"buss"}` and `{"id":"main-out"}`.
- A track is `id`, `source_id`, `left_source_channel`, `right_source_channel`, `builtins`,
  `console`, `inserts`, `fader`, then either `pan` with `left`, `right`, `smoothing_samples`, or
  `matrix` with `ll`, `lr`, `rl`, `rr`, `smoothing_samples`; never both.
- A route `channel_matrix` has `ll`, `lr`, `rl`, `rr` and no smoothing field.
- `fader` contains `left_db`, `right_db`, `left_mute`, `right_mute`. Solo is live monitoring state
  and never appears in a session document.

Closed tokens:

- render mode: `single_thread` (the only token; anything else is `schema.invalid_enum`)
- sample format: `f32_planar`
- quality: `draft`, `normal`, `high` (launch native effects publish only `normal`)
- link mode: `dual_mono`, `maximum`, `average`; each effect supports a subset, on a slot or an
  insert alike: the EQ, soft-clip and delay take only `dual_mono`, the limiter has no `average`,
  and any other pairing is `effect.link_mode.unsupported` at prepare-effects
- identity kind: `native`, `cid`
- channel: `left`, `right`, `both`
- unit: `db`, `hz`, `milliseconds`, `samples`, `linear`, `ratio`
- automation shape: `step`, `linear`, `exponential`
- automation rack: `console`, `inserts`, `builtins`
- tap, in signal order: `input`, `post_input`, `insert_send` (after `pre_insert`),
  `insert_return` (after the inserts), `pre_fader` (after `post_insert`), `post_fader`,
  `post_pan`. The retired spellings (`post_input_builtins`, `post_simd1`, `post_dynamic`,
  `post_simd2_pre_fader`, `post_matrix`) are `schema.invalid_enum`.

Automation targets contain `entity_id`, `rack`, `effect_id`, `parameter_id`, `channel`. For
`rack: "console"`, `effect_id` is the slot ID; for `rack: "inserts"`, the insert's ID; either must
name a parameter/channel pair already declared on that track's entry or insert. For
`rack: "builtins"`, `effect_id` is `"strip"`; IDs 1 polarity, 2 trim, 3 HPF, 4 LPF, 5 fader,
6 mute and 12 pan accept left/right/both, while matrix IDs 7-10 accept `both` only. Delay (11) is
prepared-only and cannot be automated. Stored automation is inert today: it authors and
round-trips, and renders nothing.

## Semantic cautions

- Launch rates are exactly 44100, 48000, 88200, and 96000 Hz. Render mode is `single_thread`.
- IDs match `[a-z][a-z0-9._-]{0,126}`. Sources have their own namespace; tracks, submixes, and
  outputs share the graph-entity namespace.
- Pan values are positions in `[-1.0,1.0]`, not gains. Conventional stereo is left `-1.0`, right
  `1.0`.
- Builtin HPF/LPF `0.0` disables the filter. `delay_samples` is required on both lanes and lies in
  `0..=48000`.
- Boolean effect values are exactly `0.0` or `1.0`; enumeration values must be listed by metadata.
- Floats must be finite and exactly representable as `f32`. Negative zero is preserved. Let the
  canonical writer choose spelling.
- Automation segments are ordered and nonoverlapping with `end_sample > start_sample`;
  exponential endpoints must both be positive.
- A validator PASS establishes Session V1 validity, builtin preparation, and launch native effect
  preparation, including console eligibility. It does not certify graph/PDC compilation, source
  availability, host resource budgets, or that declared automation currently renders.

## Console refusals, by code

| Defect | Code | Stage |
| --- | --- | --- |
| A track without an entry for a declared slot | `console.entry_missing` | typed-model |
| An entry out of slot order | `console.entry_order` | typed-model |
| A repeated entry, or a slot ID repeated across sections | `id.duplicate` | typed-model |
| An entry naming no declared slot | `reference.missing_entity` | typed-model |
| `identity`/`quality`/`link_mode`/`sidechain`/`id` on an entry; `sidechain`/`bypass`/`params` on a slot; `simd1`/`dynamic`/`simd2` on a track | `schema.unknown_field` | typed-model |
| A `cid` slot | `console.slot_not_native` | typed-model |
| A slot effect off the eligibility list | `console.slot.ineligible_effect` | prepare-effects |

## Author through the SDK instead

The TypeScript builder (`sdk/src/core/session.ts`, `session().console({ preInsert, postInsert })`
then `.track(id, { source, console: [...entries], inserts: [...] })`) and
`enginectl session build` write the canonical bytes and refuse, before boot and with the same
code, every defect above that they can express. They cannot express a `cid` slot, because a slot
names a native effect ID, so `console.slot_not_native` comes only from the engine. Their document
still goes through the validator below.

## Validate and canonicalize

```sh
cargo run -q -p session-validator -- validate .claude/skills/author-session/worked-session.json
cargo run -q -p session-validator -- validate path/to/session.json
cargo run -q -p session-validator -- validate --canonical draft.json > session.json
```

The five stages are `json-grammar`, `typed-model`, `compile-session`, `prepare-builtins`, and
`prepare-effects`. The effect stage uses the launch registry and reports its `effect.*` and
`console.slot.*` diagnostics for unavailable or ineligible effects and invalid parameters, ports,
link modes, or quality. Grammar failures report `json.syntax`; duplicate keys report the decoded
JSON path and the second key's byte span. Typed failures use `schema.*`, `numeric.*`,
`reference.*`, `console.*`, and the other codes in `docs/SESSION_SCHEMA_V1.md`. Read the code and
exact `$.json.path`, fix the named leaf, and rerun.

Canonical JSON goes to stdout, stage reports go to stderr, and a failure produces no canonical
document. Never hand-tune ordering, whitespace, float formatting, or escapes: ship the writer's
bytes, including its final LF. The worked session is its own canonical form:
`validate --canonical worked-session.json` reproduces it byte for byte.
