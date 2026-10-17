# Session schema V1

`session` accepts strict RFC 8259 JSON through exact-pinned `json-syntax 0.12.5`, after a
contract-owned duplicate-key, nesting-depth and empty-object preflight. Comments, trailing commas, multiple
top-level values, BOMs, invalid escapes, unpaired surrogates and non-JSON numeric tokens refuse.
A duplicate member refuses before its value is parsed or retained, at the decoded member path,
with a byte span over the second key. The root object is depth one; opening any object or array at
depth 129 refuses before that subtree is built.

An empty object (including whitespace-only `{ }`) refuses with `json.syntax` at its decoded
field/index path and a byte span covering both braces. Empty objects are valid JSON but no V1
schema object admits one: records require explicit fields and tagged unions require their tag.
This is a temporary workaround for json-syntax 0.12.5 leaving empty-object CodeMap entries
unfinished, which can corrupt diagnostics or panic during the typed walk (#387; dependency
repair is #391). Empty arrays and braces inside strings retain their existing semantics.

Canonical output is defined by the schema walk, not generic map order, RFC 8785/JCS, or a serde
serializer. It is UTF-8 without BOM, uses LF and two-space indentation, has no tabs or trailing
whitespace, and ends in exactly one LF. Object fields use `": "` and schema-declared order.
Order-insensitive entity arrays sort by stable ID and effect parameters by `(parameter_id,
channel)`; console slots, a track's console entries, rack effects and automation segments retain
declared order. Strings emit `\"`, `\\`,
`\b`, `\t`, `\n`, `\f`, and `\r`; other C0/C1 controls use uppercase four-digit `\uXXXX`.
Solidus and all other Unicode scalars, including U+2028, U+2029 and non-BMP scalars, emit directly.

Booleans and `u8`/`u32` fields are JSON booleans/numbers. Integer-number fields reject fractions,
exponents and any leading minus, including `-0`. Every typed `u64` leaf (source `frames`, and
automation `start_sample`/`end_sample`) is a canonical unsigned decimal JSON string matching
`^(0|[1-9][0-9]*)$`, bounded through `18446744073709551615`; an automation segment's
`start_sample` and `end_sample` are further bounded below `2^53` (`9007199254740992`,
`automation::TIME_LIMIT`: node times are exact in `f64` below it, and the stored-automation program
builder refuses a segment at or past it), so the validator refuses a larger one with
`numeric.out_of_schema_range` at that sample's path. `revision` is the same canonical
decimal string, bounded through `9223372036854775807` (2^63 - 1, `session::MAX_SESSION_REVISION`):
a plan's revision gate cannot hold a larger revision, so the validator refuses one with
`numeric.out_of_schema_range` at `$.revision`. Finite `f32` fields accept
semantically valid integer, fractional and exponent spellings and emit the proven shortest
non-exponent spelling that round-trips to identical bits through both direct-f32 and
f64-then-f32 readers. Integral floats retain `.0`; negative zero emits `-0.0`; NaN and infinities
refuse.

The schema requires the root keys, in canonical order,
`schema_version`, `session_id`, `revision`, `sample_rate_hz`, `quantum_frames`, `render_profile`,
`output_profile`, `sources`, `console`, `tracks`, `submixes`, `vcas`, `control_smoothing`,
`outputs`, `routes`, and `automation`. Every
object rejects unknown keys and every field is explicit, including empty arrays and
`"sidechain": { "kind": "none" }`. `quantum_frames` must be nonzero. Queue depth, source-ring size,
and memory budget are host policy and are not session-document fields.

A submix is a strip (decision 13, #1199). Its keys, in canonical order, are `id`, `builtins`,
`console`, `inserts`, `fader`, and exactly one of `pan` or `matrix` (#1202 adds `console`). Each
value's grammar, validation, diagnostic code and canonical spelling are the
track's, verbatim: neither `pan` nor `matrix` is `schema.missing_field`, both is
`schema.wrong_type`, and an unknown key is `schema.unknown_field`. Diagnostics use index paths, for
example `$.submixes[0].fader.left_db`. `Submix::unity(id, console)` is the transparent strip:
identity input section, one `{ slot, bypass: true, params: [] }` entry per declared console slot in
slot order, no inserts, an unmuted 0 dB fader and the identity matrix with no smoothing; a bypassed
entry is transparent and still pays its slot's latency. A submix sums the routes that target it,
left to right in route-ID order, and then runs its strip on that sum exactly as a track runs its
strip on its source: input section, console slots and inserts, fader and pan or matrix (#1200,
#1202). A bus is never mono-collapsed, and its insert latency joins plugin-delay
compensation. Its `delay_samples` delays the summed input, per lane, before the input section
runs (#1201): like a track's delay it is a musical time shift, and plugin-delay compensation never
compensates it. On the wire the submix message carries `id` 1,
`builtins` 2, the repeated `console` entry 3 (the track's field 11 entry message, #1202),
`inserts` 4, `fader` 5 and the tagged pan-or-matrix 6 (pan 1, matrix 2, as the track's field 10).

A VCA group (#1240, umbrella #1239) is `{ id, fader, members }`, in that canonical key order:
a control-only fader with no audio path. `fader` is the strip fader's record read as a per-lane
offset; each `left_db`/`right_db` is in `[-144, 24]` dB (`numeric.out_of_schema_range` outside,
`numeric.non_finite` for NaN or an infinity). `members` is the schema's one array of stable-ID
strings: a non-array is `schema.wrong_type` at `$.vcas[<i>].members`, and a non-string item
`schema.wrong_type`, a malformed one `id.invalid`, at `$.vcas[<i>].members[<j>]`. It may be empty.
A VCA's ID joins the graph-entity namespace; a collision with a track, submix, output or another
VCA is `id.duplicate` at `$.vcas[<i>].id`, and a VCA ID never satisfies a lookup that wants a strip
or an output (a route source or destination, a sidechain source, a `strip` automation target's
`owner_id`; a VCA is an `owner: "vca"` target): each is `reference.missing_entity`, as an unknown ID is. Every member names a declared
track, submix or VCA (`reference.missing_entity` at `$.vcas[<i>].members[<j>]` otherwise, an output
included), and appears once in its VCA (`id.duplicate` at the later occurrence); one strip may sit in
several VCAs. Membership is acyclic: a VCA on a membership cycle among VCAs, a self-member included,
is refused with `vca.cycle` at `$.vcas[<i>]`, once per VCA on the cycle in ascending declared index,
while a diamond (one VCA reachable along two paths) is legal. Canonically `vcas` sorts by ID and each
`members` list sorts by ID. The canonical session stores each VCA's own offset and each member's
own fader, never an effective value. VCAs apply at preparation on every host (#1242): a strip's
reach is the set of VCAs it is reachable from through membership, directly or through nested
VCAs, each once even along several paths; its effective lane gain is
`clamp(own_db + sum(offsets), -144, 24)`, summed in `f64` (the reaching VCAs' offsets first, in
ascending VCA ID, then its own value added to that sum), clamped in `f64` and rounded once to `f32`, and exactly its own
value with an empty reach; its effective lane mute is its own mute or any reaching VCA's mute. The
prepared fader section bakes the effective values, so post-fader and post-pan sends follow a VCA,
and a `follows_mute` route reads its source's effective mute. A member's own fader is still checked
against its domain, unclamped (`builtin.gain.domain` at its own `fader.<lane>_db`). On the wire
the VCA message carries `id` 1, `fader` 2 and the repeated `members` 3.

`control_smoothing` (#1054, decision 15 D15-1) is the session's live-ramp setting: one required
tagged object, in the schema's "nothing set is spelled out" shape, like `"sidechain": { "kind":
"none" }`. Its `kind` is one of:

- `{ "kind": "default" }`: the session uses the default table below. No other key is allowed
  (`schema.unknown_field` at the key).
- `{ "kind": "explicit", "mute_ms": .., "fader_ms": .., "pan_ms": .. }`: all three required, in
  that order, each a finite `f32` with `0 <= ms <= 1000` (`numeric.out_of_schema_range` at
  `$.control_smoothing.<key>` outside it). `0` and `-0.0` are legal and mean the hard switch that
  sample-exact edits and bit-comparison tests use; `-0.0` keeps its canonical spelling.

A missing `control_smoothing` key is `schema.missing_field`, a non-object `schema.wrong_type`, an
unknown `kind` `schema.invalid_enum`, and an unknown key `schema.unknown_field`. The model field is
`SessionModel::control_smoothing: Option<ControlSmoothing>` (`None` is `default`).

*Default table.* `session::CONTROL_SMOOTHING_DEFAULT` holds the researched defaults of #1055
(`docs/handoffs/control-smoothing-defaults/FINDINGS.md`, sections 1 and 9): `mute_ms` 10, `fader_ms`
20, `pan_ms` 20. A blinded listening result (#1388) can change these values and nothing else. They
keep `pan_ms >= 2 x mute_ms`, so a matrix coefficient that crosses zero (below) clicks no more than a
mute. *Rounding.* `SessionModel::control_smoothing_samples()` is the one conversion, and every
host calls it: a length becomes `floor(f64(ms) * f64(sample_rate_hz) / 1000 + 1/2)` sample frames,
evaluated in `f64` from the `f32` value (round half up; 5 ms at 44.1 kHz is 221). At the four launch
rates 10 ms is 441 / 480 / 882 / 960 samples and 20 ms is 882 / 960 / 1764 / 1920.

*Which key a live row uses.* One table; a live row ramps over its key's length, and a row not yet
live on a host reads the same table when it lands:

| key | rows |
|---|---|
| `fader_ms` | fader dB, input trim, send gain, VCA offset, detector link glide |
| `mute_ms` | mute, solo, polarity invert (twice `mute_ms`), send mute, send `follows_mute` toggle, VCA mute, effect bypass crossfade |
| `pan_ms` | pan, matrix, send matrix |

*Polarity invert* is a derived rule on the `mute_ms` key, not a fourth key: it ramps over twice the
mute length (20 ms at the defaults). A flip from `+1` to `-1` over the mute's own length clicks
about 6 dB more than the mute, and over twice that length it clicks as the mute does (#1055,
`FINDINGS.md` 9.2). *Matrix coefficients through zero.* A raw matrix coefficient (in `[-1, 1]`) or a
send matrix coefficient (any finite value) that crosses zero is a polarity flip of that path on the
`pan_ms` ramp; at the defaults `pan_ms` is twice `mute_ms`, so it clicks as the mute does, and a
session whose explicit `pan_ms` is below twice its `mute_ms` gets a louder crossing. The same row
lengths are the jump ramps of stored automation (#1058 README A3): every plan holds the three lengths
in one cell, and a live change of them is written there when the session has an automation on a row
of this table (#1639).

On the wire the `control_smoothing` record carries `kind` 1 (`default` 1, `explicit` 2), then, for
`explicit`, `mute_ms` 2, `fader_ms` 3 and `pan_ms` 4.

The root field registry is `schema_version` 1, `session_id` 2, `revision` 3, `sample_rate_hz` 4,
`quantum_frames` 5, `render_profile` 6, `output_profile` 7, `sources` 9, `tracks` 10, `submixes`
11, `outputs` 12, `routes` 13, `automation` 14, `console` 15 and `vcas` 16 and
`control_smoothing` 17. Root field 8 was `limits` (removed in `04d291dd`); it is retired and never
reallocated.

Stable IDs use `[a-z][a-z0-9._-]{0,126}`. Sources have their own unique ID namespace. Tracks,
submixes, VCAs and outputs share the graph-entity namespace; routes, automations, console slots (across
both sections), a strip's (track's or submix's) inserts, and `(parameter_id, channel)` pairs are
unique in their corresponding scopes. Canonical entity
sets sort by ID, effect parameters sort by `(parameter_id, channel)`, and rack effects plus
automation segments preserve declared order. Canonical text uses LF, exactly one final newline,
canonical string escapes, and finite `f32` spellings that preserve exact bits through both direct
`f32` parsing and `f64`-then-`f32` conversion by external readers. Normal values use shortest `f32`
`Display`; the two double-rounding values use exact `f64` `Display`; integral spellings gain `.0`
to remain floats; and negative zero is preserved exactly as `-0.0`.

The minimal and full exact-byte examples are
[`canonical-minimal.json`](../fixtures/session/v1/canonical-minimal.json) and
[`canonical.json`](../fixtures/session/v1/canonical.json). They freeze indentation, key order,
numeric/string spelling, and the final newline.

`render_profile.mode` has exactly one V1 token, `single_thread`. Every other spelling, including
the retired `dependency_waves`, is an unknown value and rejects with `schema.invalid_enum` at
`$.render_profile.mode`. `dependency_waves` named a native dependency-wave executor that was
removed as production-unreachable, and issue #1063 then removed the token itself from the model,
the parser, the canonical writer and the protocol encoding. Its protocol wire code `2` is retired,
not reallocated: a peer that spells it is refused, as the registry requires. Parallel render, if it
returns, needs a new issue that re-earns it.

`sample_rate_hz` is a launch engine setting and is exactly one of 44100, 48000, 88200, or
96000 Hz. Other values, including 176400, 192000, 352800 and 384000 Hz, reject with
`sample_rate.unsupported_at_launch` at `$.sample_rate_hz`; parsing, typed compilation, and
canonical serialization never turn such a model into an engine session. It is the only sample
rate in a document; V1 has no per-source rate and no implicit sample-rate conversion.

Each source is exactly `{ id, content, channels, bit_depth, frames }`. `content` must match
`blake3:[0-9a-f]{64}` exactly. `channels` is 1 or 2: a source is mono or stereo (owner answer of
2026-10-15), and any other value, zero or more than `SOURCE_CHANNELS_LAUNCH_MAXIMUM`, rejects with
`source.channels.unsupported` at `$.sources[i].channels` at load and in a structural edit's
validation, on every host. `frames` is nonzero; `frames` is the full canonical
content length beginning at frame zero. `bit_depth` is integer `16`, integer `24`, or the string
`"32f"`; the canonical writer preserves those spellings. Locator, mapping, region, and per-source
rate are not part of the schema. Host resolver policy maps the content identity to bytes, then
must prove rate/channels/depth/frames against the declaration before publication. The canonical
PCM preimage and content identity contract is [STEM_IDENTITY_V1.md](STEM_IDENTITY_V1.md).

V1 output is exactly two planar `f32` channels, matching its explicit 2x2 matrices. A track maps
independent left and right source channels and declares, in canonical order, independent
`builtins`, its `console` entries, its ordered `inserts`, fader/mute values, and either a smoothed
pan pair or smoothed 2x2 matrix.

## Session console and inserts (owner decision 12)

The session declares its console once, at the root, between `sources` and `tracks`:
`"console": { "pre_insert": [...], "post_insert": [...] }`. Each slot is exactly
`{ slot, identity, quality, link_mode }`: a stable `slot` ID unique across **both** sections
(a console address names the slot, not its section), a native `identity`, and the quality and
link mode every strip runs it at. A console slot takes no sidechain -- a keyed effect is an insert
-- and carries no per-track `bypass` or `params`; either key refuses there as
`schema.unknown_field`. A third-party (`cid`) identity refuses as `console.slot_not_native` at the
slot's `identity`. Either section may be empty, and an empty section costs nothing.

Every strip carries every slot: every track's and every submix's (#1202) `console` array holds
exactly one `{ slot, bypass, params }` entry per slot, in the session's slot order (`pre_insert`,
then `post_insert`), never canonical ID order. The entry carries only that strip's knobs; `id`,
`identity`, `quality`, `link_mode` and `sidechain` refuse there as `schema.unknown_field`. An entry
naming an undeclared slot refuses as `reference.missing_entity`, a repeated one as `id.duplicate`,
one out of slot order as `console.entry_order` (each at the entry's `slot`), and a strip without an
entry for a declared slot as `console.entry_missing` (at the strip's `console`), at the strip's
index path (`$.tracks[<i>]` or `$.submixes[<i>]`). Entry `params` follow the effect-parameter
rules and canonicalize by `(parameter_id, channel)`.

**Latency grows with bus depth.** A console slot's latency is paid on every strip that carries it,
bypassed or not, so a signal that passes through a track and then `n` nested buses pays a latent
slot `n + 1` times: one true-peak limiter (486 samples at 48 kHz) on every strip puts a first-order
bus's output at 972 samples, and plugin-delay compensation aligns every parallel path to that.

A track's `inserts` is `{ "effects": [...] }`, the ordered per-track effects between the two
console sections, with exactly the retired `dynamic` rack's semantics: full effect declarations,
sidechains included. It may be empty.

The chain is `input -> input section -> console.pre_insert -> inserts -> console.post_insert ->
fader/mute -> pan/matrix -> routes`.

**Eligibility.** A console slot must be one of the six effects that always bank:
`miso.parametric-eq`, `miso.compressor`, `miso.gate-expander`, `miso.soft-clip`,
`miso.transient-shaper` and `miso.true-peak-limiter`. The delay never banks, and the multiband
compressor is excluded until #1069 closes. The schema checks the identity's syntax only; the list
is enforced where native identities resolve, by effect preparation, which refuses any other slot
with `console.slot.ineligible_effect` at `$.console.<section>[slot=<id>].identity`.

**Banking.** A console slot always banks, on every vector width and for every strip count: the
graph compiler forms one bank group per (slot, pool class, dependency level) and pads a partial
group with inactive lanes (decision 12). A submix's console lanes join the group their slot, pool
class (a submix is always `Stereo`) and level select, beside other buses and any track lane there;
a bus sits after every contributor's chain, so a bus at a level no track occupies forms its own
padded group per slot. A console group that does not bind fails the compile with
`console.slot.unbanked` at `$.console.<section>[slot=<id>].bank[pool=<class>,level=<level>]`;
there is no per-node fallback. Inserts bank opportunistically: a full group banks and a remainder
renders per node.

**Class A by lowering.** Internally, `pre_insert` lowers to the graph's first rack
(`RackId::Simd1`), a track's `inserts` to the second (`Dynamic`) and `post_insert` to the third
(`Simd2`). Each console entry lowers to an ordinary effect whose ID is the slot, whose identity,
quality and link mode are the slot's, whose bypass and params are the strip's, and whose sidechain
is `none` (`SessionModel::lower_strip`). A session equivalent to a pre-decision-12 one therefore
compiles to the identical graph, including the sealed `MISO-GRAPH-V1` canonical text, and renders
the identical bits. The internal names `RackId`, `TrackStage`, `MeterTap` and `RackLocation` are
unchanged.

**Retired, on the #1063 precedent.** The per-track `simd1`, `dynamic` and `simd2` keys are gone:
each refuses as `schema.unknown_field` at the track, never read as its successor. The session
model's field registry keeps every ID: track field 7 is now `inserts` (was `dynamic`), fields 6
(`simd1`) and 8 (`simd2`) are retired and never reallocated, and `console` is appended as root
field 15 and track field 11, with registries of its own for the slot declaration (`slot` 1,
`identity` 2, `quality` 3, `link_mode` 4) and the track entry (`slot` 1, `bypass` 2, `params` 3).

An automation target is a control address (#1566, decision 17 C2): the plain address the control
list uses (fields 1 to 6), as a record tagged by `section`, with the keys `owner` (`strip`, `route` or `vca`;
`session` is not a token, because no session row is automatable), `owner_id` (the strip, route or VCA
ID), `section` (`own`, `builtins`, `console` or `inserts`), `slot_id` (the console slot or the insert),
`control` (a `u32`, the row ID inside the section) and `lane` (`left`, `right` or `both`). `slot_id`
is required in a `console` or `inserts` section and refused (`schema.unknown_field`) in `own` and
`builtins`; every other key is required. The old keys (`entity_id`, `rack`, `effect_id`,
`parameter_id`, `channel`) are unknown keys. The wire message is the control list's address
message, `ControlAddress` (fields `owner` 1, `owner_id` 2, `section` 3, `slot_id` 4, `control` 5 and
`lane` 6; owner kind strip 2, route 3, VCA 4 and section inserts 2, builtins 4, console 5, own 6, the
retired rack codes 1 and 3 never allocated), and the session wire refuses owner kind session. A
target carries no expected effect: field 7 belongs to `SET_CONTROL`'s own address message,
`SetControlAddress` (#1678), and a stored target is refused with it.

Session validation checks the structure of a target, and nothing about its row: the `strip` owner is
a declared track or submix (`reference.missing_entity` at `target.owner_id` otherwise; a VCA ID is
not a strip), a `route` owner is a declared route and a `vca` owner a declared VCA, a route or VCA
owner uses the `own` section (`schema.invalid_enum` at `target.section` otherwise), a `console`
`slot_id` is a console slot the strip carries and an `inserts` `slot_id` one of that strip's inserts
(`reference.missing_entity` at `target.slot_id`). Whether the row exists, is automatable, takes the
lane and takes the segment's unit and values is the control core's rule
(`controls::automation_diagnostics`), reported by host preparation and `session-validator` stage 5
beside the effect diagnostics: `control.address.unknown` (no row), `control.automation.target`
(the row is rebuild-only, derived, a monitor row, a setting, or one of the two live rows that a ruling keeps out of automation: a send's `follows_mute`, a routing setting, and a strip's `matrix_mode`, a mode; #1632), `control.inactive` (the owner's shape
does not use the row), `control.lane` (`left` or `right` on a shared row), `control.domain` (a
segment whose unit is not the row's, or a value the row refuses), `control.automation.shape` and
`control.automation.order` (#1645, below), and `control.automation.offset` (#1650, below). A console target on a submix
addresses that submix's entry for the slot (#1202). A session does not have to state a parameter
value for the row it automates: the row exists whether or not the session states it.

**The fader, mute and VCA rows render; every other row is inert today.** An entry on a strip's fader
(`builtins` row 5) is compiled into the plan as a table (#1646) and rendered on the C ABI and in
the browser alike (#1647): the lane follows its curve in node time, reaching the curve's value at
the end of each 64-sample retarget on a fixed grid, taking each discontinuity at its exact sample
over the session's fader ramp length (`control_smoothing.fader_ms`), lined up with the audio it
acts on through every latent insert, and set exactly, with no ramp, where a session seek reaches
the fader. One exception: while a mute or unmute ramp holds the lane's gain, the lane only
remembers each value it is given, a seek's included, and where the hold ends with the gain free it
ramps over the fader ramp length to the curve at that ramp's end (a seek that reaches the fader
exactly where the hold ends still sets it exactly). The output does not depend on the render
quantum or on where a seek landed. An entry on a strip's mute (`builtins` row 6, `step` only,
values `0` and `1`) is compiled into the same table after every fader entry (#1651): the lane mutes
and unmutes at each step's exact sample over the session's mute ramp length
(`control_smoothing.mute_ms`), and the mute the stage renders is the curve's OR the lane's other
mute terms (its own static mute, any reaching VCA's mute, and solo), so a hand mute closes an open
curve and a hand unmute leaves a closed curve closed. Only a change of that OR moves the lane. The
control list marks a lane's `mute` row `automated` (`ControlEntry` field 35) while a mute entry
drives the lane (#1651, root ruling R18-2); the row's value stays the lane's static (hand) mute. A
session seek sets it exactly where the seek changes it, and a ramp toward a state the seek leaves
unchanged finishes. An entry on a VCA's own rows (#1655: row 1
`offset_db` and row 2 `mute`, per lane) is compiled into the same table after every mute entry, one
table per automated VCA lane, and rendered on the C ABI and in the browser alike through the VCA's
members, since a VCA has no audio: each member lane the VCA reaches, directly or through nested
VCAs, follows the VCA curves in its own fader node time, so a member behind a latent insert hears
the VCA move with its own audio. A member lane's fader dB is `vca_compose_db(own, S + c_1 + ... +
c_m)`: `own` is the lane's own curve when its fader is automated, else its static fader dB; the
`c_k` are the reaching automated VCA curves in ascending VCA ID; the sum is in `f64`, `S` first, and
`S` holds the lane's own hand offset, then the static offsets of the reaching VCAs no entry drives,
then the hand offsets of the reaching automated VCAs, each group in ascending VCA ID. Static terms
come first, then the automated curves: a class B summation order under the standing
summation-order ruling (with F16), equal to the static path's ascending-ID order except in F16's
degenerate cases, so a flat VCA ride renders the bits of the same static VCA offset. Events of
several curves at one sample take one jump over the longest of their rows' lengths. A member
lane's mute is its own curve OR each reaching VCA `mute` curve OR its other mute terms, and a
following send of a member follows the VCA mute curves in its tap's node time. A hand move of an
automated VCA `offset_db` writes the entry's offset (live), a hand VCA mute ORs with the curve
(live), and a hand fader move of a member with no fader entry of its own stays live. An edit of a
VCA entry, and a change of which member lanes an automated VCA reaches (membership, nesting, or an
automated VCA added or removed), is a rebuild; a VCA no entry drives keeps its live edits. A
session without VCA automation renders the bits it rendered before. Send rows do not render: an
entry on a send's own rows (a route into a submix, rows 1-6) is valid and inert, as below (#1654
renders them later). A route into the output stays prepared: its rows are rebuild-only, so an
entry on one is refused `control.automation.target`. Stored automation has no queue: render
generates its events from the plan. An entry on any other row,
for the strip, a console slot or an insert, is valid and inert: it authors and round-trips and
renders nothing until its row's slice lands. Rendering the whole table identically on every
platform is owned by issue #1058; #140, which once gated it, was descoped
(`rulings/engine-footprint-2026-09-28.md`).

**What the table means: the hold rule (#1626, #1058 A3).** One entry is one curve of one lane over
absolute sample time. Before its first segment the lane holds the first `start_value`. Inside a
segment `[start_sample, end_sample)` it follows the segment: `linear` is `v0 + (v1 - v0)·x`,
`exponential` is `v0·(v1/v0)^x`, with `x = (t - t0)/(t1 - t0)`, and a `step` segment holds
`start_value` on the whole of `[start_sample, end_sample)`. After a segment, and in a gap between
two, the lane holds that segment's `end_value`; a `step` takes its `end_value` at `end_sample`. The
value after the last segment holds for ever.

**What a row takes (#1645).** Each segment is reported once, at
`$.automation[id=<id>].segments[<k>]`: `control.domain` first, else `control.automation.shape`.
The shape rule is one rule for every row: a toggle row (polarity, mute) or an enumeration takes only
`step`; `exponential` only on a continuous row whose minimum is above `0` and whose unit is not dB
(of the builtins, only HPF and LPF; never a fader, trim, pan or matrix row); and on a row with a
disabled value (a filter's `0`, "off") a `linear` or `exponential` segment has neither value equal
to it, so "off" is entered and left only by a `step`.

**The hand-move offset (#1650).** Every automation entry carries a required `offset` record, key
4 after `segments` (`id` 1, `target` 2, `segments` 3), with two required finite `f32` keys,
`left_db` 1 and `right_db` 2: the dB offset a hand move adds to the curve of each lane. On the wire
it is field 4 of `Automation`, a required nested `AutomationOffset` message with two required `F32`
fields 1 and 2. A missing record or lane is `schema.missing_field` and a third key
`schema.unknown_field`, at their paths. The row rule (`controls::automation_diagnostics`) judges
each lane at `$.automation[id=<id>].offset.left_db` or `.right_db`, after the segments:

- `fader_db` (builtins control 5 of a strip) and a VCA's `offset_db` (own control 1, per lane,
  #1655) take an offset (`takes_offset`). The input trim and the send gain do not yet: the slices
  that render them (#1658, #1654) add them.
- A lane that takes no offset holds exactly `+0.0`, else `control.automation.offset`. That is a
  lane the entry does not drive (`right_db` of a `left` entry, `left_db` of a `right` entry) and
  every lane of an entry whose row takes no offset. `-0.0` is refused there too, so "no offset"
  has one spelling.
- A lane a `fader_db` or VCA `offset_db` entry drives holds an offset in the fader's own domain,
  `[-144, 24]` dB (the VCA offset domain), else `control.domain`. `-0.0` is a valid zero there.

The offset of a lane belongs to the entry that drives the lane, so a lane-granular hand move on a
`both` entry needs no entry split. `RemoveAutomation` removes the offset with the entry;
`SetAutomationSegments` and `SetAutomationTarget` keep it, and the row rule judges it against the
new target. The offset is stored, checked, round-tripped (canonical JSON, wire, SDK) and heard
(#1650b): render composes each fader event of the lane as `vca_compose_db(curve, S)`, where `S`
is the `f64` sum of the lane's own offset first and then the reaching VCA offsets in ascending VCA
ID (with VCA automation, the order of the VCA rows above). On a lane a rendered `fader_db` entry drives, the `fader_db` row reads and writes the offset:
`SET_CONTROL fader_db` sets it (live, one offsets-cell write; a `both` write sets both lanes),
`CONTROL_STATE_GET` and the control list report it as the row's value, and the list marks the row
`automated` (`ControlEntry` field 35). The static fader value of an automated lane is its fallback,
not a control value: the static fallback changes only through a rebuild (remove the automation,
set the fader, upsert the automation, in one transaction) or a document replacement, and any edit
that changes it on an automated lane is a change of structure. An `UpsertAutomation` that changes
only an entry's offset is live, as a hand move is.

**The input filter order, on curves (#1645).** On each strip lane where HPF (3) or LPF (4) is
automated (a `both` entry drives both lanes), each side is its entry under the hold rule, or the
constant of its static value. The time axis is cut at every `start_sample` and `end_sample` of
either curve; on each piece a side that is a constant `0` leaves it unchecked, and otherwise:
two constants need `H < L` (the static rule); two `linear` or two `exponential` pieces need
`L > H·(1 + 2^-22)` at both ends; any other pair needs `min(L) > max(H)·(1 + 2^-22)` over the two
ends, which is sufficient, not necessary, and refuses some pairs that never cross. The margin keeps
the rendered `f32` values strictly ordered. Render can lead a stored filter value by 64 samples
(the grid lookahead) and lag it by 64 (the filter's jump ramp), so the HPF at each sample is held
below the LPF at every sample within 128 samples of it: stored automation the render would carry
across the order is refused here, never accepted and then refused in render.
A failure is `control.automation.order`, once per lane
at the first failing piece, at the segment of the LPF entry (else the HPF entry) that defines its
value there. The rule reads only the model, so it also judges a static filter value under an
automated curve.

Session validation refuses what no rendering could honour, structurally and before any row is read:

- **One entry per lane** (`automation.duplicate_lane`, at `$.automation[<i>].target.lane`). Two
  entries share a lane when their `owner`, `owner_id`, `section`, `slot_id` and `control` are equal
  and their lanes overlap: `both` overlaps `left` and `right`, `left` overlaps `left`, `right`
  overlaps `right`. A `left` entry and a `right` entry on one control are two lanes. Of an
  overlapping pair the entry with the greater `id` is refused, once however many entries it
  overlaps, so the refused entry does not depend on declared order. An entry whose target is
  already refused, and an entry whose `id` repeats an earlier one (`id.duplicate`), is not
  reported again.
- **Discontinuities at least 64 samples apart** (`automation.discontinuity_spacing`, at the later
  one's `$.automation[<i>].segments[<k>].start_sample` or `.end_sample`, once per close pair). A
  *discontinuity* is a sample where the lane's value differs, by `f32` bits (so `0.0` and `-0.0`
  differ), from its value just before it. Only segment boundaries can be one: a segment's
  `start_sample` whose `start_value` differs from the value held before it (never the first
  segment's), and the `end_sample` of a `step` segment whose `end_value` differs from its
  `start_value`. Two boundaries at one sample are one discontinuity. 64 is the engine's fixed grid
  period for a moving curve, not a session value. The rule reads only an entry whose segments
  passed the order and overlap checks.

These rules read no row, so they say nothing of unit, domain, shape or filter order against the
target's row: that is the preparation rules of the control core
(`controls::automation_diagnostics`, above). A pan target on a matrix strip is that core's
`control.inactive`.
Builtin cutoffs are finite nonnegative hertz values, but their DSP/Nyquist relationships are not
issue-004 validation.

Each lane's builtins table carries exactly `polarity_invert`, `trim_db`, `hpf_hz`, `lpf_hz` and
`delay_samples`, all required. `delay_samples` (issue #210 phase 2) is the track's input-side time
alignment for multi-mic work, an integer count of samples in the inclusive range `0..=48000`
validated by issue-004 -- a flat schema domain rather than a DSP one, because what it bounds is the
ring allocation a session can demand. It is expressed in **samples**, not milliseconds: alignment is
sample-exact, the engine is sample-domain throughout, and #147's unit-in-name rule makes the unit
part of the key. A host converts from milliseconds; the session never does. The two lanes are
independent under the dual-mono law, and a track whose lanes declare different delays is genuinely
asymmetric upstream of the mono-collapse seam, so it declines that track's collapse.

On a submix, `delay_samples` delays the summed input instead (#1201). The next paragraph's
`delay_samples` rules apply to it unchanged: it is not latency, its rings are charged to
`graph_delay_bytes`, and it is prepared-only. The live trim, polarity and HPF/LPF commands that
paragraph also describes address tracks only, in canonical track order; a submix strip's input
section is not live-addressable until #1213.

`delay_samples` is deliberately **not** plugin latency and PDC never compensates it: it is a time
shift the session asked for, so it contributes zero to any node's declared latency and does not
appear in the compiled plan's route timings or inserted delays. Its rings are charged to the
plan's existing `graph_delay_bytes` row. It is prepared-only, changed through the ordinary
transactional session edit. Issue #210 phase 3 made trim and polarity live, and issue #808
added live HPF/LPF targets (first through command kinds 10, 11 and 12, deleted by #1565;
today the strip's builtin rows, set by `SET_CONTROL`, with an atomic pair edit as one
transaction). Filter coefficients use fixed 64 current-then-advance
updates after off-render preparation; trim retains its existing smoothing law.
`delay_samples` is the only lane key that remains prepared-only. The liveness ruling
is `docs/rulings/builtins-input-liveness-d2.md`; these commands do not add a session
automation render feed. Effect identity is tagged `native` with a stable `effect_id`, or `cid` with
opaque nonempty text. Native availability/descriptor domains/latency/tail are downstream issue-011
work. A `cid` identity parses and validates here and is refused at effect preparation with
`effect.third_party.unavailable_at_launch`: third-party effects are out of scope until a new issue
reopens them (owner ruling R6a, #1037, which removed the issue-029 package crate). Removing the
`cid` arm from this grammar would move that refusal to parse time; it is a Session V1 grammar
change that needs its own approval.

A `native` `effect_id` is therefore a *stable ID*, not a registry lookup: this schema checks its
syntax and never its membership. `fixtures/session/v1/canonical.json` exercises exactly that
boundary. It names `effect_id = "parametric-eq"` without the `miso.` prefix the launch registry
carries, and it is accepted, compiled and round-tripped all the same; the launch registry would
refuse it at preparation, which is the point. That spelling is load-bearing rather than a typo.
The prepare-side tests that consume the fixture
(`crates/effect-compiler/tests/native_session.rs`) inject a test-local factory whose
descriptor id is the same unprefixed `parametric-eq`, and the fixture's SHA-256 is pinned three
levels deep: `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml` carry it as
`session_template_sha256` and `tools/bench` re-derives and compares it at benchmark
time; `fixtures/builtins/v1/MANIFEST.tsv` digests those two documents; and
`tools/audit/src/fixture_builtins.rs` pins both the field literal and the manifest's
own digest. Re-spelling the fixture would move all of them for no behavioural gain. Author new
sessions from the metadata's registry ids -- `miso.parametric-eq` and the rest -- and do not copy
this fixture's `effect_id`.

Routes use a tagged source and destination port shape. A source is either
`{ kind = "track", track_id, tap }` or `{ kind = "submix", submix_id, tap }`; a destination is
either `{ kind = "submix_input", submix_id }` or `{ kind = "output_input", output_id }`. This
makes output sources and track destinations unrepresentable. A submix's input is the sum of the
routes that target it, summed in route-ID order. Every strip, track or submix, offers the same
seven taps at the same points of its chain (`input`, `post_input`, `insert_send`,
`insert_return`, `pre_fader`, `post_fader`, `post_pan`; #1203), and `tap` is required on both
kinds; a pre-fader tap is not gated by the fader mute. The retired `submix_output` (which left the
strip after its pan or matrix, now `{ kind = "submix", submix_id, tap = "post_pan" }`) is an unknown
`kind` and refuses with `schema.invalid_enum`. Every route carries a required boolean `mute`, written
after `gain_db` (#1216): the send's on/off switch. A muted route stays in the graph, with its edge,
its latency compensation and its gain and matrix kept, and contributes silence; muting or unmuting
a route never changes the plan's structure or latency. Every route also carries a required boolean
`follows_mute`, written after `mute` (#1218). With `follows_mute: true` the send follows its source
strip's lane mutes (the source's fader `left_mute` and `right_mute`, on a track or a submix): a
muted source lane's column of the route matrix (`ll` and `rl` for the left lane, `lr` and `rr` for
the right) contributes `+0.0`, never by way of the gain, and a send whose two source lanes are both
muted is silenced exactly as a muted route is (below). It is what makes a `pre_fader` (or earlier)
send go quiet with its muted strip; from a `post_fader` or `post_pan` tap the fader mute already
silences the lane, so the follow changes no audible sample. A send whose two source lanes are muted
is skipped, and its silent contribution becomes `+0.0` instead of a signed zero. Only a route into a submix
may follow: a route into the output is never live, so `follows_mute: true` there refuses with
`schema.invalid_enum` at `$.routes[<i>].follows_mute` ("follows_mute applies only to a route into
a submix"). The refusal is a validation, not a parse, so a transaction that sets the flag and
re-points the route into a submix commits. The prepared follow is part of the sealed graph text (a
`route-follow-zeroed` row). A muted route whose edge into its
destination carries no latency compensation contributes nothing: it is neither mixed nor read
(#1217). A muted route whose edge is compensated keeps running and contributes its mix with `+0.0`
coefficients through its delay, so its compensation line never holds stale audio. Either way the
first route in route-ID order owns the destination's sum -- its contribution when it contributes,
else `+0.0` -- and each later contributing route is added to it in order. A lone contributing
`-0.0` therefore keeps its sign only as the first route, and a destination whose every route is
muted and uncompensated sums to exact `+0.0`. A zero coefficient times a non-finite `input`-tap
sample is NaN, so a muted compensated route can carry NaN into a bus input; the bus's input section
sanitizes it, but a meter at the bus's `input` boundary sees it. A route's `gain_db` lies in
`[-144, 24]` dB and each `channel_matrix` coefficient in `[-1, 1]`, both inclusive (#1237): a
finite value outside refuses `numeric.out_of_schema_range` at `$.routes[<i>].gain_db` or
`$.routes[<i>].channel_matrix.<ll|lr|rl|rr>`, and a non-finite one keeps `numeric.non_finite`. The
compiler's lowering and every live send record check the same domain (a live value outside it is
refused `domain`). A route applies `gain * coefficient` per matrix position; a product that is
subnormal (a coefficient below about `1.9e-31` at -144 dB) is flushed to `+0.0` before it is
bound or pushed. Routed sidechains reuse the tagged source shape,
taps included, and require a nonempty stable `port_id`. Port *existence* is still not an issue-004
concern -- the schema layer never sees a descriptor -- but it is no longer downstream work either:
`prepare_native_session_effects` refuses an unknown port at boot with
`effect.sidechain.unknown_port` (`crates/effect-compiler/src/prepare.rs:1113`), beside
`effect.sidechain.missing` for a required declared port the session left unconnected and
`effect.sidechain.unexpected` for a routed sidechain the descriptor does not declare at all. A
session naming a port no descriptor declares therefore parses, validates and compiles, and then
fails preparation. Those three refusals are the authority and are unmoved. What changed is what
stands in front of them: issue #275 recorded that the generated SDK catalog published each
effect's parameters and observations but no port table, so `portId` was the one session field an
SDK builder could not check before boot. Issue #278 closed that gap by publishing the declared
`ports` per effect -- id, role, `required` and lane layout, for all eight launch effects, of which
exactly `miso.compressor` and `miso.gate-expander` declare an optional `sidechain-in`. `effect()`
now resolves `portId` against that table and refuses a misspelling, a non-sidechain port and a
sidechain on an effect that declares none, each naming the legal ports, while the boot-time
refusal remains what a hand-written document meets. The only track taps are `input`,
`post_input`, `insert_send` (after `console.pre_insert`), `insert_return` (after the inserts),
`pre_fader` (after `console.post_insert`), `post_fader`, and `post_pan`, with wire codes 1-7 in
that order. Decision 12 renamed them in place and moved no position or code; the retired spellings
`post_input_builtins`, `post_simd1`, `post_dynamic`, `post_simd2_pre_fader` and `post_matrix` are
unknown tokens and refuse with `schema.invalid_enum`.

Issue 004 owns structural validity, ID syntax/uniqueness, references whose declaration role is
already represented by this schema, finite/`f32`/unit-local ranges, source identity/shape bounds,
ordered automation representation, and checked resource estimates. Ownership continues as
follows:

| Deferred validation | Owning issue |
| --- | --- |
| Graph cycles, scheduling, port existence, PDC | 006 |
| Builtin/effect DSP domains and Nyquist relationships | 007 |
| SIMD-bank/cohort compatibility | 008 |
| Source asset resolution and declared-shape matching | 010 |
| Native descriptor/effect validity | 011 |
| Third-party CID/package validity | none: out of scope until reopened (R6a, #1037) |

Issue 010 must resolve content and reject any decoded rate/channels/depth/frames mismatch before
plan publication. Issue 004 does not claim cycle freedom, valid downstream ports, effect
availability, or a publishable render plan.

`compile_session` first computes checked retained-string, vector/index, canonical upper-bound,
largest-allocation, `usize`/`isize`, and total model-byte estimates. `CompileCaps` bounds compiled
model bytes and the largest single allocation before canonical allocation, cloning, sorting, or
index construction. Its legacy queue/runtime/ring fields report zero for a session because those
allocations are host policy; host preparation applies the chosen queue, ring, and aggregate memory
caps separately. There is no track-count cap.

Diagnostics use one stable dotted code registry and a structured `DiagnosticPath` of field, index,
or stable-ID segments. A rejected parse or compile returns a nonempty `DiagnosticSet` and no partial
artifact. A successful `CompiledSession` is immutable and non-publishable; it has no graph schedule,
DSP state, `PlanPublisher`, or `PreparedRenderPlan` capability.

Every diagnostic returned from parsing has a source span, including diagnostics produced by domain
validation after the model shape has been read. Typed canonicalization and compilation have no source
text, so their otherwise matching code/path diagnostics have `span = None`. Every model `u64`
field serializes as a canonical decimal string and supports the full unsigned domain, except
`revision` (bounded as above).

The runtime manifest exact-pins `json-syntax = 0.12.5` without default features and has no runtime
serde or runtime `serde_json` dependency. `serde_json` is dev-only for order and unknown-key
mutations; it is neither the acceptance parser nor the canonical writer.
