# Session schema V1

`miso-engine-session` accepts strict TOML 1.0 and requires the root keys, in canonical order,
`schema_version`, `session_id`, `revision`, `sample_rate_hz`, `quantum_frames`, `render_profile`,
`output_profile`, `limits`, `sources`, `tracks`, `submixes`, `outputs`, `routes`, and `automation`.
Every table rejects unknown keys and every field is explicit, including empty arrays and
`sidechain = { kind = "none" }`.

Stable IDs use `[a-z][a-z0-9._-]{0,126}`. Sources have their own unique ID namespace. Tracks,
submixes, and outputs share the graph-entity namespace; routes, automations, rack-local effects,
and `(parameter_id, channel)` pairs are unique in their corresponding scopes. Canonical entity
sets sort by ID, effect parameters sort by `(parameter_id, channel)`, and rack effects plus
automation segments preserve declared order. Canonical text uses LF, exactly one final newline,
canonical string escapes, and finite `f32` spellings that preserve exact bits through both direct
`f32` parsing and the parser's `f64`-then-`f32` conversion. Normal values use shortest `f32`
`Display`; the two double-rounding values use exact `f64` `Display`; integral spellings gain `.0`
to remain TOML floats; and negative zero is preserved exactly as `-0.0`.

`sample_rate_hz` is a launch engine setting and is exactly one of 44100, 48000, 88200, or
96000 Hz. Other values, including extended compatibility corpus rates, reject with
`sample_rate.unsupported_at_launch` at `$.sample_rate_hz`; parsing, typed compilation, and
canonical serialization never turn such a model into an engine session. A source's nonzero
`sample_rate_hz` remains lossless asset metadata and does not itself claim engine support.

V1 output is exactly two planar `f32` channels, matching its explicit 2x2 matrices. A track maps
independent left and right source channels and declares independent builtins, fader/mute values,
ordered `simd1`/`dynamic`/`simd2` racks, and either a smoothed pan pair or smoothed 2x2 matrix.
Builtin cutoffs are finite nonnegative hertz values, but their DSP/Nyquist relationships are not
issue-004 validation. Effect identity is tagged `native` with a stable `effect_id`, or `cid` with
opaque nonempty text. Native availability/descriptor domains/latency/tail are downstream issue-011
work; CID/package validity is downstream issue-029 work.

Routes use a tagged source and destination port shape. A source is either
`{ kind = "track", track_id, tap }` or `{ kind = "submix_output", submix_id }`; a destination is
either `{ kind = "submix_input", submix_id }` or `{ kind = "output_input", output_id }`. This
makes output sources and track destinations unrepresentable. Routed sidechains reuse the tagged
source shape and require a nonempty stable `port_id`; port existence remains downstream work. The
only track taps are `input`, `post_input_builtins`, `post_simd1`, `post_dynamic`,
`post_simd2_pre_fader`, `post_fader`, and `post_matrix`.

Issue 004 owns structural validity, ID syntax/uniqueness, references whose declaration role is
already represented by this schema, finite/`f32`/unit-local ranges, source channel and region
bounds, ordered automation representation, and checked resource estimates. Ownership continues as
follows:

| Deferred validation | Owning issue |
| --- | --- |
| Graph cycles, scheduling, port existence, PDC | 006 |
| Builtin/effect DSP domains and Nyquist relationships | 007 |
| SIMD-bank/cohort compatibility | 008 |
| Source asset resolution and declared-rate matching | 010 |
| Native descriptor/effect validity | 011 |
| Third-party CID/package validity | 029 |

Consequently, a declared source rate may differ from the engine rate in this IR; there is still no
implicit SRC, and issue 010 must resolve or reject it before plan publication. Issue 004 does not
claim cycle freedom, valid downstream ports, effect availability, or a publishable render plan.

`compile_session` first computes checked retained-string, vector/index, canonical upper-bound,
queue, source-ring, largest-allocation, `usize`/`isize`, and total byte estimates. `CompileCaps`
then bounds compiled model bytes, requested runtime bytes, the largest single allocation, queue
items, source-ring frames, and source-ring bytes before canonical allocation, cloning, sorting, or
index construction. The session's own `limits.memory_bytes` independently bounds requested runtime
bytes. There is no track-count cap.

Diagnostics use one stable dotted code registry and a structured `DiagnosticPath` of field, index,
or stable-ID segments. A rejected parse or compile returns a nonempty `DiagnosticSet` and no partial
artifact. A successful `CompiledSession` is immutable and non-publishable; it has no graph schedule,
DSP state, `PlanPublisher`, or `PreparedRenderPlan` capability.

The manifest requests `serde = 1.0.228` and `toml = 0.9.9`; the latter resolves to package version
`0.9.9+spec-1.0.0`. `spec-1.0.0` is not a Cargo feature. TOML defaults are disabled and only
`parse` and `serde` are enabled. The schema-specific canonical writer intentionally does not enable
the dependency's `display` feature.
