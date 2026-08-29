---
name: author-session
description: Author, extend, or repair a miso engine-v2 session TOML file and prove it valid with the real session pipeline (grammar, schema, compile, builtins preparation).
---

# Authoring an engine-v2 session

Run every command from the repository root. The validator writes no file of its own, produces no
artifact, and renders no audio — canonical output goes to stdout and you redirect it yourself.

## What you are producing

One strict, versioned TOML document. `schema_version = 1`, thirteen root keys in canonical order:

`schema_version`, `session_id`, `revision`, `sample_rate_hz`, `quantum_frames`, `render_profile`,
`output_profile`, `sources`, `tracks`, `submixes`, `outputs`, `routes`, `automation`.

Every field is explicit, empty arrays included. Every effect declares `sidechain` even when it has
none (`sidechain = { kind = "none" }`). There is no "extra keys are ignored": an unknown key is a
rejection.

## Authorities, in this order

1. **`docs/SESSION_SCHEMA_V1.md`** — normative and short enough to read end to end. Do that once
   before you start, and again whenever the validator returns a code you do not recognize.
2. **A fixture** — `fixtures/session/v1/`. **Copy the structure of the nearest one; never invent
   the shape from memory.** You will still write your own document, but every table in it should
   have a fixture table as its model.
   `canonical.toml` is the usual starting point and a complete instance of every table you will
   need — `sources`, `builtins`, `fader`, `render_profile`, `routes`, `automation`. Read
   it field by field rather than reconstructing a table from this document. Also useful:
   `canonical-minimal.toml` (smallest legal session), `observation-frame-shape.toml` (three tracks,
   three effects, fully declared parameter sets), `console-sixty-four-track.toml` (64 tracks), and
   six more. `toml-1.0-invalid-duplicate-key.toml` is **deliberately invalid** — never copy it.

   Two tables have **no fixture at all**: a populated `submixes` array, and a track `matrix`.
   (Grepping for `matrix = { ll` finds only route `channel_matrix` tables, a different shape.) Both
   are under "Table shapes" below — that is the only place to get them.
3. **The parameter metadata** — generate it; never guess a parameter id, unit, domain, or default:
   ```
   cargo run -q -p miso-engine-parameter-metadata -- --print
   ```
   `effects[].id` is the string for `effect_id`. `effects[].parameters[]` gives `id`, `name`,
   `unitName`, `domainName`, `minimum`, `maximum`, `default`, and `enumChoices` where the domain is
   an enumeration. A session's `unit` token must equal that parameter's `unitName`.
   `builtins.parameters[]` describes the builtin strip in the same shape, but with its own domain
   vocabulary under `domain` — `booleanExact`, `finiteInclusive`, `disabledOrRateKeyedHertz` —
   rather than the effects' `domainName` of `boolean`, `continuous`, `enumeration`.

   Unit-in-name (#147, still open): builtin fields already carry the unit (`trim_db`, `hpf_hz`),
   effect parameter *names* do not yet (`threshold`, `attack`). Until they do, `unitName` is the
   authority, not the name. Named step sizes (#242) are a control-plane concept and never appear
   in a session file.

### Effect ids

The registry ids are `miso.parametric-eq`, `miso.compressor`, `miso.gate-expander`,
`miso.true-peak-limiter`, `miso.multiband-compressor`, `miso.soft-clip`, `miso.transient-shaper`,
`miso.delay`. Confirm against the metadata.

`fixtures/session/v1/canonical.toml` carries two traps of its own. It declares
`effect_id = "parametric-eq"` without the `miso.` prefix — that predates the registry, and V1 does
not check effect availability (issue 011), so the validator accepts it and later preparation would
not; take `effect_id` strings from the metadata. And it puts its EQ in the **`dynamic` rack with
`simd1` empty**, contradicting the #175 layout below. Follow the layout rule, not that fixture.

**The worked example of the intended layout is
`fixtures/session/v1/console-sixty-four-track-intended.toml`** (#175, 2026-08-26): the standing
64-track qualification session, and the only checked-in fixture that populates `simd2`. Copy its
rack structure — a two-slot `simd1` chain, `dynamic = { effects = [] }`, and a one-slot `simd2`
chain — when you need the production layout. It is *generated*, not authored:
`scripts/derive-intended-console-fixture.py` derives it from the retired
`console-sixty-four-track.toml` and takes its canonical spelling from this validator, so edit the
generator and regenerate rather than editing the file (`scripts/check-intended-console-fixture.sh`
compares the two byte for byte). Its header documents the limiter's parameter choices and their
provenance in the metadata, including why `lookahead` is uniform across all sixty-four tracks
while `ceiling` and `release` vary.

Parameter ids are not contiguous. `miso.parametric-eq` bands are **16 apart**: band 1 is 1–6,
band 2 starts at 17, band 3 at 33, band 4 at 49.

## Rack semantics

The track strip, in order:

`input builtins -> simd1 -> dynamic -> simd2 -> fader/mute -> 2x2 matrix or pan -> routes`

- All three rack keys are always present, empty if unused: `simd1 = { effects = [] }`.
- **Bank eligibility follows the effect's kernel contract, not rack placement** (#166). A
  bank-eligible native effect banks in the `dynamic` rack exactly as it does in a SIMD rack, and one
  without the contract renders per node in a SIMD rack exactly as it does in `dynamic`. Placement
  regroups lanes; it never moves a rendered bit, so it is a layout decision, not a correctness one.
- Intended production layout (#175): **EQ and compressor on `simd1`, limiter on `simd2`.** Follow it
  unless the user asks otherwise.
- Third-party wasm (`identity = { kind = "cid", cid = "..." }`) is permitted **only** in `dynamic`,
  and it never banks anywhere — opacity disqualifies it regardless of rack.
- Effect order inside a rack is semantic and preserved exactly as declared.

Sidechains: `sidechain = { kind = "none" }` is the bank-friendly form. A connected sidechain is
`sidechain = { kind = "routed", source = { ... }, port_id = "..." }` using the route-source shape
below with a nonempty stable `port_id`; that effect then falls back to per-node rendering.

## Closed vocabularies

Every one of these is a closed set. A token outside it is `schema.invalid_enum`, and none of them
appear in the parameter metadata — this list is the authority.

| Key | Accepted tokens |
| --- | --- |
| `render_profile.mode` | `single_thread`, `dependency_waves` (parses, but only `single_thread` launches -- see below) |
| `output_profile.sample_format` | `f32_planar` |
| effect `quality` | `draft`, `normal`, `high` |
| effect `link_mode` | `dual_mono`, `maximum`, `average` |
| effect `identity.kind` | `native` (with `effect_id`), `cid` (with `cid`) |
| effect `sidechain.kind` | `none`, `routed` |
| param / automation `channel` | `left`, `right`, `both` |
| param / automation `unit` | `db`, `hz`, `milliseconds`, `samples`, `linear`, `ratio` |
| automation `shape` | `step`, `linear`, `exponential` |
| automation `target.rack` | `simd1`, `dynamic`, `simd2`, `builtins` (the strip itself -- see below) |
| route source `tap` | `input`, `post_input_builtins`, `post_simd1`, `post_dynamic`, `post_simd2_pre_fader`, `post_fader`, `post_matrix` |

`link_mode` is the detector link, not a channel mode. `dual_mono` keeps the two lanes fully
independent and is the default choice; `maximum` links a compressor or limiter by the peak of both
lane detectors (the usual "stereo-linked" behaviour); `average` links by their mean. There is no
`stereo`, `linked`, or `stereo_linked` token.

## Table shapes

- A submix and an output are each nothing but an ID: `{ id = "buss" }`, `{ id = "main-out" }`.
  They carry no fader, matrix, or racks — a submix is a named mix point the graph compiler owns.
- A source is exactly `{ id, content, channels, bit_depth, frames }`. `content` is
  `"sha256:<64 lowercase hex>"` over canonical PCM, `channels` and `frames` are nonzero, and
  `bit_depth` is integer `16`, integer `24`, or string `"32f"`. The session root supplies the only
  sample rate. Locators, regions, per-source rates, rings, queues, and memory budgets are host or
  resolver policy and never appear in the document.
- A track declares **either** `pan = { left, right, smoothing_samples }` **or**
  `matrix = { ll, lr, rl, rr, smoothing_samples }`, never both.
- A route's `channel_matrix = { ll, lr, rl, rr }` has no `smoothing_samples`; a track's `matrix`
  does. They are different tables.
- An effect's `params` array may be **partial** — declare only the parameters you are setting, and
  the rest take their contract defaults. But an automation target can only name a
  `(parameter_id, channel)` pair that is present in `params`, so declare anything you intend to
  automate even if you set it to its default.
- `fader = { left_db, right_db, left_mute, right_mute }` — decibels and mutes, per lane.
- **There is no `solo` key, and asking for one is the wrong question.** Solo is monitoring
  state, not mix state (issue #210 ruling D1): the engine composes it live from the command
  plane as `left_mute/right_mute || (any solo engaged && this track not soloed)`, and no
  session document carries a solo bit. A session reloads with every solo clear, and an
  offline or stem render — which runs with no command stream at all — can never come out
  soloed. If you want a strip silent *in the document*, set its `left_mute`/`right_mute`.

### Automation targets: rack effects, and now the strip

An `automation` target is `{ entity_id, rack, effect_id, parameter_id, channel }`, all five
required, on a track (submixes and outputs carry no racks). `rack` is one of four tokens.

**The three effect racks.** `simd1`/`dynamic`/`simd2` name a rack, `effect_id` names an effect in
it, and `parameter_id`/`channel` must match a `(parameter_id, channel)` pair the effect actually
declares in its `params` — so declare anything you intend to automate even at its default, or the
target does not resolve.

**`rack = "builtins"`** (issue **#178**, ruled by #210's D2) names the strip itself. The strip is a
chassis, not a rack of instances, so there is nothing to identify — but V1 has no optional keys, so
`effect_id` is still required and carries the fixed literal `"strip"`. Anything else is
`reference.missing_entity` at `$.automation[N].target.effect_id`.

`parameter_id` is a builtin parameter ABI id, and only the ones the render plane can be *told* to
change are accepted:

| id | parameter | `channel` |
| --- | --- | --- |
| 1 | `polarity_invert` | `left`, `right` or `both` |
| 2 | `trim_db` | `left`, `right` or `both` |
| 5 | `fader_db` | `left`, `right` or `both` |
| 6 | `mute` | `left`, `right` or `both` |
| 7-10 | `matrix_ll`, `matrix_lr`, `matrix_rl`, `matrix_rr` | `both` only |

`hpf_hz` (3), `lpf_hz` (4) and `delay_samples` (11) are **refused** —
`reference.missing_entity` at `$.automation[N].target.parameter_id`. They are prepared-only: there
is no post-preparation write path, so a span addressed at one could only ever be inert, and the
schema says so rather than accepting it and doing nothing. A matrix coefficient addressed
`left` or `right` is `schema.invalid_enum` at `…target.channel`: the 2x2 is one shared object.

**What a valid target does today: nothing.** The automation table is consumed by nothing — for the
strip *or* for the three effect racks. No lowering reads it. A target authors, round-trips, and
renders nothing; builtin automation *rendering* is gated on issue #140's span feed. So write these
if you are authoring a document for a future feed or for the SDK's builder, and do not expect a
render to change.

The *command plane* is a different surface and is genuinely live: a host driving the engine over
the control protocol can move `trim_db` (kind 10), `polarity_invert` (11), `fader_db` (3), `mute`
(4), pan/matrix (1/2) and `solo` (9) on a running plan. That is not this file.

## Values a PASS does not vouch for

The validator checks representable and preparable, not *musically intended*. These four are the
ones that pass while meaning something other than what you assumed.

- **`pan.left` / `pan.right` are constant-power pan positions in `[-1.0, 1.0]`, not gains** — one
  position per dual-mono lane, `-1.0` hard left, `0.0` centre, `1.0` hard right. Conventional
  stereo is `{ left = -1.0, right = 1.0 }`. **`canonical.toml` declares `{ left = 1.0, right = 1.0 }`,
  which pans both lanes hard right.** Copy its structure; never copy its pan values. Out of range
  is `numeric.out_of_schema_range` at `$.tracks[N].pan.left`.
- **Builtin `hpf_hz` / `lpf_hz` of `0.0` means the filter is disabled**, not 0 Hz — that is the
  metadata's `disabledValue`. An enabled cutoff is at least `10.0` and below the rate-keyed maximum
  for your `sample_rate_hz`.
- **`delay_samples` is required on both lanes and is written in samples, never milliseconds.**
  Every lane's builtins table is exactly `polarity_invert`, `trim_db`, `hpf_hz`, `lpf_hz`,
  `delay_samples`; omit it and you get `schema.missing_field`. Write `delay_samples = 0` unless the
  session is doing multi-mic time alignment. The domain is `0..=48000` and out of range is
  `numeric.out_of_schema_range` at `$.tracks[N].builtins.left.delay_samples`. Convert from
  milliseconds yourself: `round(ms * sample_rate_hz / 1000)`. For the ordinary alignment workflow
  set **both lanes to the same value** — unequal lanes are a legitimate but unusual declaration
  that makes the track's two channels genuinely different and costs it the mono-collapse
  optimization. The delay is applied at the track input, ahead of trim/HPF/LPF, so every `input`
  send tap, sidechain and input meter sees aligned audio; it is not latency and PDC does not
  compensate it away.
- **`link_mode` is mandatory on every effect but inert without a detector.** EQ, soft clip,
  transient shaper and delay have nothing to link: write `dual_mono`. For compressors, limiters and
  gates pick deliberately — a true-peak limiter normally wants `maximum`, because independent
  per-lane gain reduction shifts the stereo image.
- **`boolean` and `enumeration` parameters are not free scalars.** Check the metadata's
  `domainName`: `boolean` takes exactly `0.0` or `1.0`, and `enumeration` takes a value listed in
  that parameter's `enumChoices`. Both carry `unitName: "linear"`, which is a carrier, not a licence.
- **An automation `target.entity_id` must name a track**, and the target must resolve — either to
  a `(parameter_id, channel)` pair a rack effect declares in `params`, or to one of the strip's
  eight automatable builtin ids under `rack = "builtins"`. Both resolve at validation and **neither
  renders**: the automation table is consumed by nothing today, so a fader or trim ride you "wrote"
  as automation is valid, canonical, and inert. See "Automation targets: rack effects, and now the
  strip" above, and `docs/rulings/builtins-input-liveness-d2.md`.

## The validation loop

```
cargo run -q -p miso-engine-session-validator -- validate path/to/session.toml
```

Four stages, the real pipeline in the real order, each `PASS` / `FAIL` / `SKIP`. Exit 0 all passed,
1 a stage failed, 2 usage or I/O error.

```
session: draft.toml
  PASS  stage 1  toml-grammar      TOML grammar (toml_parser)
  FAIL  stage 2  typed-model       strict V1 schema decode and validation
        schema.unknown_field  $.tracks[0].fader.left_gain  (line 13, column 608)  key is not part of SESSION_SCHEMA_VERSION_V1
  SKIP  stage 3  compile-session   not reached
  SKIP  stage 4  prepare-builtins  not reached
result: FAIL at stage 2 (typed-model), 1 diagnostic
```

The failing stage names the kind of repair:

| Stage | Rejects | Typical fix |
| --- | --- | --- |
| 1 `toml-grammar` | `toml.syntax` | a TOML typo: unbalanced brace, duplicate key |
| 2 `typed-model` | `schema.*`, `id.*`, `reference.*`, `numeric.*`, `unit.*`, `source.*`, `automation.*`, `capacity.zero`, `sample_rate.*`, `render_mode.*` | the document contradicts the schema |
| 3 `compile-session` (resource preflight, caps, canonical normalization) | `resource.*`, `capacity.*` | the declared shape overflows a checked compiler bound |
| 4 `prepare-builtins` (off-render builtins preparation) | `builtin.*` | a builtin value outside its DSP domain (e.g. a cutoff above the rate-keyed maximum) |

Read `code`, then the `$.json.path` — it points at the exact leaf. Stage 1 and 2 diagnostics also
carry `(line, column)`; stages 3 and 4 have no source text, so they carry none. Fix, rerun, repeat
until `result: PASS`.

Then normalize and ship the canonical form:

```
cargo run -q -p miso-engine-session-validator -- validate --canonical draft.toml > session.toml
```

Canonical TOML goes to stdout, the stage report to stderr, and nothing is written when a stage
fails. Re-run the plain `validate` on the result, and ship that file. Do not hand-tune spacing,
key order, or float spellings — regenerate instead.

## Constraints and pitfalls

- `render_profile.mode` must be `single_thread`. `dependency_waves` is a valid token that the
  parser knows, so it fails at stage 2 with `render_mode.unsupported_at_launch` rather than
  `schema.invalid_enum` -- the repair is to write `single_thread`, not to invent a new token. The
  parallel executor it named no longer exists.
- `sample_rate_hz` is exactly one of `44100`, `48000`, `88200`, `96000`; IDs match
  `[a-z][a-z0-9._-]{0,126}` (lowercase, leading letter); `output_profile` is exactly
  `{ id, channels = 2, sample_format = "f32_planar" }`.
- `quantum_frames`, every source's `channels`, and every source's `frames` must be nonzero
  (`capacity.zero` otherwise).
- Namespaces are dual: sources have their own; tracks, submixes and outputs share the graph-entity
  namespace. Route ids, automation ids, rack-local effect ids and `(parameter_id, channel)` pairs
  are unique in their own scopes.
- Ordering: entities sort by ID and effect params sort by `(parameter_id, channel)` — the canonical
  writer does that for you. Rack effect order and automation segment order are *preserved as
  declared*, because they are semantic.
- Route source is `{ kind = "track", track_id, tap }` or `{ kind = "submix_output", submix_id }`;
  destination is `{ kind = "submix_input", submix_id }` or `{ kind = "output_input", output_id }`,
  plus `channel_matrix` and `gain_db`. Outputs cannot be sources, tracks cannot be destinations.
- **Unknown keys reject.** A misspelled key yields two diagnostics — `schema.unknown_field` for the
  typo and `schema.missing_field` for the real key it displaced.
- **`pan` and `matrix` are mutually exclusive.** Both gives `schema.wrong_type` at `$.tracks[N]`;
  neither gives `schema.missing_field` at the same path.
- **Floats.** Values must be finite and `f32`-representable. An integer literal is accepted for a
  float field and canonicalizes to `0.0`, but `-0.0` is preserved exactly and is not `0.0`. Let
  `--canonical` decide every spelling.
- **Unit domains.** `hz`, `milliseconds`, `samples` and `ratio` must be `>= 0`, and `samples` must
  be integral. `db` and `linear` may be negative — subject to the parameter's own metadata domain.
- **Automation segments** must be ordered, non-overlapping, and `end_sample > start_sample`. An
  `exponential` segment additionally requires `start_value` **and** `end_value` to be strictly
  positive (`automation.invalid_range`, "exponential values must be positive") — which rules it out
  for the common case of a `db` ramp through zero or into negative gain. Use `linear` there.
- **Source channels.** `left_source_channel` and `right_source_channel` must be less than that
  source's `channels`.
- **A PASS is not a renderable graph.** V1 owns structure, ids, references, domains and resources.
  Graph cycles and ports (issue 006), Nyquist relationships (007), effect availability (011) and
  CID/package validity (029) are all checked elsewhere.
