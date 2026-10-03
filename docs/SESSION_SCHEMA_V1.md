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
exponents and any leading minus, including `-0`. Every typed `u64` leaf (`revision`, source
`frames`, and automation `start_sample`/`end_sample`) is a canonical unsigned decimal JSON string
matching `^(0|[1-9][0-9]*)$`, bounded through `18446744073709551615`. Finite `f32` fields accept
semantically valid integer, fractional and exponent spellings and emit the proven shortest
non-exponent spelling that round-trips to identical bits through both direct-f32 and
f64-then-f32 readers. Integral floats retain `.0`; negative zero emits `-0.0`; NaN and infinities
refuse.

The schema requires the root keys, in canonical order,
`schema_version`, `session_id`, `revision`, `sample_rate_hz`, `quantum_frames`, `render_profile`,
`output_profile`, `sources`, `console`, `tracks`, `submixes`, `outputs`, `routes`, and `automation`. Every
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

Stable IDs use `[a-z][a-z0-9._-]{0,126}`. Sources have their own unique ID namespace. Tracks,
submixes, and outputs share the graph-entity namespace; routes, automations, console slots (across
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
`blake3:[0-9a-f]{64}` exactly. `channels` and `frames` are nonzero; `frames` is the full canonical
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

An automation target's `rack` is one of three tokens, with explicit wire codes: `inserts` (2,
the retired `dynamic` rack's code), `builtins` (4) and `console` (5). Codes 1 (`simd1`) and 3
(`simd2`) are retired and refused, never reallocated, and the retired spellings are unknown
tokens. `console` addresses a slot by `effect_id: <slot>` in either section; `inserts` addresses a
strip's insert by its ID. An automation target's `entity_id` names a track or a submix (#1199),
and a `console` target may name a submix: it addresses that submix's entry for the slot (#1202).
A submix's target is as inert as a track's. `builtins`, since issue #178 (ruled by #210's D2), is
the strip's own fixed section. The strip is a chassis rather than a rack of instances, so it has
no `effect_id` to identify; the key is required all the same (V1 has no optional fields) and
carries the fixed validated literal `"strip"`. Its `parameter_id` is a builtin parameter ABI id,
restricted to the rows that declare `blockTarget`: `polarity_invert` (1), `trim_db` (2), `hpf_hz` (3), `lpf_hz` (4),
`fader_db` (5), `mute` (6), the four `matrix_*` coefficients (7-10), and `pan` (12).
That is **eleven** rows; `BUILTIN_AUTOMATION_TARGETS` in
`crates/session/src/validate.rs` is the authority. Issue #808 adds the two filter
rows. The prepared-only `delay_samples` (11) remains refused. The seven per-lane
rows accept `left`, `right` or `both`; the four shared matrix coefficients accept
only `both`.

**The automation table is consumed by nothing today.** No lowering reads it, for the strip, a
console slot or an insert: a valid target is valid-and-inert syntax that authors, round-trips
and renders nothing. Extending the vocabulary unblocks authoring and the SDK's builder. Rendering
the stored automation table, builtin targets included, identically on every platform is owned by
issue #1058 (research first); #140, which once gated it, was descoped
(`rulings/engine-footprint-2026-09-28.md`).
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
transactional session edit. Issue #210 phase 3 made trim and polarity live through
command kinds 10 and 11. Issue #808 adds live HPF/LPF targets through command kind12,
including an atomic pair edit. Filter coefficients use fixed 64 current-then-advance
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
a route never changes the plan's structure or latency. Routed sidechains reuse the tagged source shape,
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
field supports the full unsigned domain and serializes as a canonical decimal string.

The runtime manifest exact-pins `json-syntax = 0.12.5` without default features and has no runtime
serde or runtime `serde_json` dependency. `serde_json` is dev-only for order and unknown-key
mutations; it is neither the acceptance parser nor the canonical writer.
