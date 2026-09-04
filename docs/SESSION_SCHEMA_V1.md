# Session schema V1

`session` accepts strict RFC 8259 JSON through exact-pinned `json-syntax 0.12.5`, after a
contract-owned duplicate-key, nesting-depth, and empty-object preflight. Comments, trailing
commas, multiple top-level values, BOMs, invalid escapes, unpaired surrogates and non-JSON numeric
tokens refuse. A duplicate member refuses before its value is parsed or retained, at the decoded
member path, with a byte span over the second key. The root object is depth one; opening any
object or array at depth 129 refuses before that subtree is built.

An empty JSON object `{}` anywhere in the document refuses with `json.syntax` at that value's
path, with a byte span over its `{...}` bytes, before the typed walk runs (issue #387). This is a
workaround for a `json-syntax 0.12.5` defect, not a schema rule stated for its own sake: the
dependency never finishes the reserved `CodeMap` entry for an empty object (it never calls
`end_fragment` for that branch, unlike the empty-array branch), which otherwise misreads every
sibling member declared after it and can panic. No V1 schema position accepts `{}` regardless --
every object rejects unknown keys and requires every field explicit -- so the refusal costs no
legal document; empty arrays remain legal and are unaffected.

Canonical output is defined by the schema walk, not generic map order, RFC 8785/JCS, or a serde
serializer. It is UTF-8 without BOM, uses LF and two-space indentation, has no tabs or trailing
whitespace, and ends in exactly one LF. Object fields use `": "` and schema-declared order.
Order-insensitive entity arrays sort by stable ID and effect parameters by `(parameter_id,
channel)`; rack effects and automation segments retain declared order. Strings emit `\"`, `\\`,
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
`output_profile`, `sources`, `tracks`, `submixes`, `outputs`, `routes`, and `automation`. Every
object rejects unknown keys and every field is explicit, including empty arrays and
`"sidechain": { "kind": "none" }`. `quantum_frames` must be nonzero. Queue depth, source-ring size,
and memory budget are host policy and are not session-document fields.

Stable IDs use `[a-z][a-z0-9._-]{0,126}`. Sources have their own unique ID namespace. Tracks,
submixes, and outputs share the graph-entity namespace; routes, automations, rack-local effects,
and `(parameter_id, channel)` pairs are unique in their corresponding scopes. Canonical entity
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

`render_profile.mode` is a launch engine setting. Both V1 tokens still parse -- `single_thread`
and `dependency_waves` -- because the closed token set, the protocol wire encoding and the
canonical writer are all lossless by doctrine, and canonical round-trip forbids normalizing one
token into another. Only `single_thread` launches. `dependency_waves` rejects with
`render_mode.unsupported_at_launch` at `$.render_profile.mode` from parsing, typed compilation and
canonical serialization alike, so no caller reaches a prepared plan through an entry point that
skipped the check. The token named a native dependency-wave executor that was removed as
production-unreachable; a rejection is the honest answer, where silently rendering single-threaded
would let a session claim parallelism it never had.

`sample_rate_hz` is a launch engine setting and is exactly one of 44100, 48000, 88200, or
96000 Hz. Other values, including extended compatibility corpus rates, reject with
`sample_rate.unsupported_at_launch` at `$.sample_rate_hz`; parsing, typed compilation, and
canonical serialization never turn such a model into an engine session. It is the only sample
rate in a document; V1 has no per-source rate and no implicit sample-rate conversion.

Each source is exactly `{ id, content, channels, bit_depth, frames }`. `content` must match
`sha256:[0-9a-f]{64}` exactly. `channels` and `frames` are nonzero; `frames` is the full canonical
content length beginning at frame zero. `bit_depth` is integer `16`, integer `24`, or the string
`"32f"`; the canonical writer preserves those spellings. Locator, mapping, region, and per-source
rate are not part of the schema. Host resolver policy maps the content identity to bytes, then
must prove rate/channels/depth/frames against the declaration before publication. The canonical
PCM preimage and content identity contract is [STEM_IDENTITY_V1.md](STEM_IDENTITY_V1.md).

V1 output is exactly two planar `f32` channels, matching its explicit 2x2 matrices. A track maps
independent left and right source channels and declares independent builtins, fader/mute values,
ordered `simd1`/`dynamic`/`simd2` racks, and either a smoothed pan pair or smoothed 2x2 matrix.

An automation target's `rack` is one of four tokens: the three effect racks `simd1`, `dynamic`,
`simd2`, and -- since issue #178, ruled by #210's D2 -- `builtins`, the strip's own fixed section.
The strip is a chassis rather than a rack of instances, so it has no `effect_id` to identify; the
key is required all the same (V1 has no optional fields) and carries the fixed validated literal
`"strip"`. Its `parameter_id` is a builtin parameter ABI id, restricted to the rows that declare
`blockTarget`: `polarity_invert` (1), `trim_db` (2), `fader_db` (5), `mute` (6), the four
`matrix_*` coefficients (7-10) and -- since #242, under #239 ruling 5461507633 B4 -- `pan` (12).
That is **nine** rows, and `BUILTIN_AUTOMATION_TARGETS` in
`crates/session/src/validate.rs` is the list. The prepared-only rows -- `hpf_hz` (3),
`lpf_hz` (4), `delay_samples` (11) -- are **refused**, because a span addressed at a parameter with
no post-preparation write path could only ever be inert. `channel` follows the row's scope: the
five per-lane rows accept `left`, `right` or `both`, the four shared matrix coefficients only
`both`.

**The automation table is consumed by nothing today.** No lowering reads it, for the strip or for
any of the three effect racks: a valid target is valid-and-inert syntax that authors, round-trips
and renders nothing. Extending the vocabulary unblocks authoring and the SDK's builder; builtin
automation *rendering* is gated on issue #140's span feed.
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

`delay_samples` is deliberately **not** plugin latency and PDC never compensates it: it is a time
shift the session asked for, so it contributes zero to any node's declared latency and does not
appear in the compiled plan's route timings or inserted delays. Its rings are charged to the
plan's existing `graph_delay_bytes` row. It is prepared-only, changed through the ordinary
transactional session edit, as the cutoffs are -- but **no longer as `trim_db` is**: issue #210
phase 3 made `trim_db` and `polarity_invert` live (command kinds 10 and 11) and automatable, under
the ruling in `docs/rulings/builtins-input-liveness-d2.md`. `hpf_hz`, `lpf_hz` and `delay_samples`
are the three lane keys that remain prepared-only. Effect identity is tagged `native` with a stable `effect_id`, or `cid` with
opaque nonempty text. Native availability/descriptor domains/latency/tail are downstream issue-011
work; CID/package validity is downstream issue-029 work.

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
`{ kind = "track", track_id, tap }` or `{ kind = "submix_output", submix_id }`; a destination is
either `{ kind = "submix_input", submix_id }` or `{ kind = "output_input", output_id }`. This
makes output sources and track destinations unrepresentable. Routed sidechains reuse the tagged
source shape and require a nonempty stable `port_id`. Port *existence* is still not an issue-004
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
`post_input_builtins`, `post_simd1`, `post_dynamic`, `post_simd2_pre_fader`, `post_fader`, and
`post_matrix`.

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
| Third-party CID/package validity | 029 |

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
