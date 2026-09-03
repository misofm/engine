---
name: author-session
description: Author, extend, or repair a strict Session V1 canonical JSON document and prove it with the real grammar, typed-model, compile, and builtins-preparation pipeline.
---

# Authoring a Session V1 document

Run commands from the repository root. A session is one strict JSON document; JSON is the sole
live Session V1 format. There are no aliases, comments, trailing commas, duplicate keys, format
sniffing, or TOML translation. Unknown keys reject.

## Read the authorities

1. Read `docs/SESSION_SCHEMA_V1.md` end to end.
2. Copy structure from the nearest `fixtures/session/v1/*.json` document. Start with
   `fixtures/session/v1/canonical.json` or `canonical-minimal.json`; use
   `observation-frame-shape.json` for populated
   effects and `console-sixty-four-track-intended.json` for the production rack layout.
3. Generate parameter metadata rather than guessing effect IDs, parameter IDs, units, domains, or
   defaults:

   ```sh
   cargo run -q -p parameter-metadata -- --print
   ```

The thirteen root keys are `schema_version`, `session_id`, `revision`, `sample_rate_hz`,
`quantum_frames`, `render_profile`, `output_profile`, `sources`, `tracks`, `submixes`, `outputs`,
`routes`, and `automation`. Every field and empty array is explicit. Durable unsigned 64-bit values
(`revision`, source `frames`, automation `start_sample`/`end_sample`) are canonical decimal JSON
strings: no sign, whitespace, leading zero except `"0"`, or value above `18446744073709551615`.

## Core shapes and vocabularies

- A source has exactly `id`, `content`, `channels`, `bit_depth`, and `frames`. `content` is
  `sha256:` plus 64 lowercase hex digits; `frames` is a nonzero decimal string; `bit_depth` is
  `16`, `24`, or `"32f"`.
- A submix and output are respectively `{"id":"buss"}` and `{"id":"main-out"}`.
- A track has either `pan` with `left`, `right`, `smoothing_samples`, or `matrix` with `ll`, `lr`,
  `rl`, `rr`, `smoothing_samples`; never both.
- A route `channel_matrix` has `ll`, `lr`, `rl`, `rr` and no smoothing field.
- Every effect has a `sidechain`, normally `{"kind":"none"}`. A connected sidechain uses
  `{"kind":"routed","source":{...},"port_id":"..."}`.
- Every track has `simd1`, `dynamic`, and `simd2`, with `effects: []` when unused. Effect order is
  semantic. Intended layout is EQ then compressor on `simd1`, limiter on `simd2`. Third-party CID
  effects are allowed only in `dynamic` and never bank.
- `fader` contains `left_db`, `right_db`, `left_mute`, `right_mute`. Solo is live monitoring state
  and never appears in a session document.

Closed tokens:

- render mode: `single_thread` (`dependency_waves` parses but is unsupported at launch)
- sample format: `f32_planar`
- quality: `draft`, `normal`, `high`
- link mode: `dual_mono`, `maximum`, `average`
- identity kind: `native`, `cid`
- channel: `left`, `right`, `both`
- unit: `db`, `hz`, `milliseconds`, `samples`, `linear`, `ratio`
- automation shape: `step`, `linear`, `exponential`
- rack: `simd1`, `dynamic`, `simd2`, `builtins`
- tap: `input`, `post_input_builtins`, `post_simd1`, `post_dynamic`,
  `post_simd2_pre_fader`, `post_fader`, `post_matrix`

Automation targets contain `entity_id`, `rack`, `effect_id`, `parameter_id`, `channel`. Rack-effect
targets must name a declared parameter/channel pair. For `rack: "builtins"`, `effect_id` is
`"strip"`; IDs 1 polarity, 2 trim, 5 fader, 6 mute accept left/right/both, while matrix IDs 7–10
accept `both` only. HPF, LPF, and delay are prepared-only and cannot be automated.

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
- A validator PASS establishes Session V1 validity and preparation, not graph acyclicity, external
  effect availability, package validity, or that declared automation currently renders.

## Validate and canonicalize

```sh
cargo run -q -p session-validator -- validate path/to/session.json
cargo run -q -p session-validator -- validate --canonical draft.json > session.json
cargo run -q -p session-validator -- validate session.json
```

The four stages are `json-grammar`, `typed-model`, `compile-session`, and `prepare-builtins`.
Grammar failures report `json.syntax`; duplicate keys report the decoded JSON path and the second
key's byte span. Typed failures use `schema.*`, `numeric.*`, `reference.*`, and the other codes in
`docs/SESSION_SCHEMA_V1.md`. Read the code and exact `$.json.path`, fix the named leaf, and rerun.

Canonical JSON goes to stdout, stage reports go to stderr, and a failure produces no canonical
document. Never hand-tune ordering, whitespace, float formatting, or escapes: ship the writer's
bytes, including its final LF.
