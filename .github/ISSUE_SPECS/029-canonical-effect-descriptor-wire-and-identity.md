# 029 Canonical effect descriptor wire and identity

## Outcome

Ship one canonical external byte representation for every semantic `EffectDescriptorV1` field,
one domain-separated 32-byte descriptor identity, and matching bounded Rust/C inspection surfaces.
This is the first launch-sized interchange product. Package/CID selection, persisted state, migration
and broad qualification are stateless successors rather than hidden work in this issue.

## Status and attempt budget

**FINAL FAIL / STOPPED / RESCOPED.** Dependencies 002, 004 and 011 are accepted. The pre-edit
lifetime stop and corrected implementation pass exhausted this issue's two-attempt budget. The
focused-green partial wire/diagnostic checkpoint `64900f2` is technical input only and is not an
accepted descriptor-wire product or PASS dependency. Issue 082, **Close canonical effect descriptor
wire, identity, and C inspection ABI**, owns the corrected validation taxonomy and remaining product
closure. Workload, benchmark and timed invocation counts are all zero.

The provisional `miso-engine-effect-package` descriptor stub is unaccepted input. Preserve the
semantic effect contract and compiler exactly: no runtime trait, descriptor meaning, registry,
preparation, processing, automation, graph or DSP change is permitted.

## Canonical wire V1

All integers are unsigned little-endian. All offsets are absolute byte offsets from byte zero.
Every table begins at an 8-byte boundary; padding/reserved bytes are zero. No trailing byte is legal.
The total length is at most the caller's nonzero `maximum_descriptor_bytes`, `u32::MAX`,
`usize::MAX` and `isize::MAX`.

The 96-byte header is:

| Offset | Width | Field/value |
| ---: | ---: | --- |
| 0 | 8 | ASCII `MISOEFD1` |
| 8 | 2 | wire version `1` |
| 10 | 2 | header bytes `96` |
| 12 | 4 | flags `0` |
| 16 | 4 | exact total bytes |
| 20 | 2 | `contract_major` |
| 22 | 2 | `contract_minor` |
| 24 | 4 | `state_layout_version` |
| 28 | 4 | `supported_link_modes.bits()` |
| 32,36 | 4+4 | effect-ID string offset/length |
| 40,44 | 4+4 | display-name string offset/length |
| 48,52 | 4+4 | parameter count/table offset |
| 56,60 | 4+4 | port count/table offset |
| 64,68 | 4+4 | quality count/table offset |
| 72,76 | 4+4 | enum-choice count/table offset |
| 80,84 | 4+4 | string-pool bytes/offset |
| 88 | 8 | reserved zero |

Tables are contiguous in header order: 80-byte parameter records, 24-byte port records, 64-byte
quality records, 16-byte enum-choice records, then the unpadded terminal string pool. Parameter
records contain, in order: `id`, unit, domain, mapping, automation rate, channel policy, smoothing,
smoothing samples, flags, minimum bits, maximum bits, default bits, enum start/count, display-name
offset/length, display-unit offset/length and two reserved zero `u32`s. Flags are readable bit 0,
automatable bit 1, minimum-present bit 2 and maximum-present bit 3; no other bit is legal. Absent
minimum/maximum slots contain positive-zero bits.

Port records contain ID offset/length, role, required (`0` or `1`), layout and one reserved zero
`u32`. Quality records contain quality, sample rate, latency `u64`, tail kind (`1` finite, `2`
infinite), reserved zero, tail samples `u64` (zero for infinite), common/left/right state bytes,
reserved zero, fixed scratch `u64` and scratch-per-frame `u64`. Enum-choice records contain exact
value bits, label offset/length and reserved zero.

Wire enum numbers are exactly the accepted `#[repr(u32)]` values from Issue 011. Link-mode bits are
DualMono `1`, Maximum `2`, Average `4`; DualMono is mandatory and all other bits reject. Legal
`f32` values retain their exact IEEE-754 bits. Nonfinite and negative-zero values reject; positive
zero is canonical. UTF-8 strings are nonempty, at most 255 bytes (effect/port IDs at most 127),
contain no scalar for which Rust `char::is_control()` is true, and receive no normalization. IDs
retain the accepted lowercase-ASCII grammar.

Parameters remain strictly increasing by numeric ID. Enum choices remain strictly increasing by
numeric value. Qualities remain strictly increasing by `(quality, sample_rate)`. Ports are encoded
in canonical `(role_number, id_UTF8_bytes)` order regardless of their semantic slice order. The
string pool has no deduplication and is the exact first-use concatenation: effect ID, display name,
each parameter's display name/display unit/enum labels in parameter order, then port IDs in canonical
port order. Each offset must equal that traversal cursor; overlap, gaps, aliases and unused bytes
reject.

The two validation boundaries are deliberately different:

- Size/encode accepts an actual `&'static EffectDescriptorV1` and must call the unchanged
  `validate_descriptor_v1` before sizing, canonicalization or publication.
- Verification of arbitrary caller bytes must not reconstruct an `EffectDescriptorV1`. The parser
  builds a private effect-package-owned `BorrowedEffectDescriptorViewV1<'a>` (the name is
  descriptive, not a new public contract type) and a private validator independently enforces every
  frozen Issue-011 semantic rule plus this issue's stricter canonical-wire rules. The public
  `VerifiedEffectDescriptorWireV1<'a>` may expose bounded borrowed wire accessors, but it is not an
  effect-contract descriptor and cannot enter the runtime/compiler.

The implementation must not use `unsafe` lifetime extension, `transmute`, `Box::leak`, interning,
a global cache or any equivalent lifetime laundering. It must not change or add a public seam to
`miso-engine-effect-contract`, its validator, runtime traits or compiler.

## Identity and bounded Rust API

`EffectDescriptorIdentityV1` is exactly:

`SHA-256(ASCII "miso.engine.effect-descriptor.identity.v1\0" || LE-u64(wire_length) || canonical_wire)`.

It is descriptor identity only, not package identity, trust or executable selection. Public Rust APIs
are named `effect_descriptor_wire_v1_required_size`, `encode_effect_descriptor_wire_v1`,
`verify_effect_descriptor_wire_v1` and `effect_descriptor_identity_v1`; they provide exact-size query,
encode into `&mut [u8]`, canonical borrowed verification/inspection and identity calculation. They
take explicit nonzero limits, use checked arithmetic and do not require a `Vec`. Insufficient output
returns the exact required byte count and leaves the complete caller buffer unchanged. All failure
paths are allocation-bounded control-plane work and never render-reachable.

## Fixed C ABI and diagnostics

The sole header path is
`crates/miso-engine-effect-package/include/miso_engine_effect_descriptor_v1.h`. It defines the wire
enums plus records matching the table fields, a 64-byte summary (ABI version, total bytes, four table
counts, state-layout version, link bits and 32-byte identity), and a 16-byte diagnostic (`code`, byte
offset, record index, required bytes), all with compile-time C11 size/alignment/offset assertions.
No Rust layout is transmuted as wire.

The exact C typedef sizes/alignments are: `miso_engine_effect_parameter_record_v1` 80/4,
`miso_engine_effect_port_record_v1` 24/4, `miso_engine_effect_quality_record_v1` 64/8,
`miso_engine_effect_enum_choice_record_v1` 16/4,
`miso_engine_effect_descriptor_summary_v1` 64/4 and
`miso_engine_effect_descriptor_diagnostic_v1` 16/4. Field offsets equal their wire-record offsets;
the summary's eight `u32` fields occupy bytes 0–31 and identity bytes 32–63.

The sole symbol is `miso_engine_effect_descriptor_v1_inspect`. In order, it accepts
`(const uint8_t *wire, size_t wire_len, uint32_t maximum_wire_bytes, summary*, parameter*,
uint32_t parameter_capacity, port*, uint32_t port_capacity, quality*, uint32_t quality_capacity,
enum_choice*, uint32_t enum_choice_capacity, uint32_t *required_parameters,
uint32_t *required_ports, uint32_t *required_qualities, uint32_t *required_enum_choices,
diagnostic*)` and returns the diagnostic/status `uint32_t`. Output-array pointers may be null only
when their capacity is zero; input may be null only at length zero. Summary, all required-count
pointers and diagnostic are mandatory. Validation is complete before publication. If any array is
short, return buffer-too-small, publish all required counts and required bytes, and leave summary/
record arrays byte-for-byte unchanged. Any other failure writes only the diagnostic and zero
required counts. For buffer-too-small, diagnostic `required_bytes` is the checked sum of required
parameter/port/quality/choice record storage; for every other inspect result it is zero. A null
diagnostic returns code 1 with no writes; any other mandatory-null argument writes code 1 when the
diagnostic is available. Success atomically fills every requested output.

Diagnostic numbers/strings are frozen: `0/ok`, `1/null`, `2/limit`, `3/buffer_too_small`,
`4/header`, `5/length`, `6/reserved`, `7/enum`, `8/flags`, `9/order`, `10/offset`, `11/text`,
`12/float`, `13/semantic`, `14/overflow`; strings are `effect.descriptor.wire.<name>`. The first
error is selected by phase order: arguments, limit/host fit, header/version, total/section lengths,
reserved/flags, offsets/canonical order, enums, text, floats, semantic validation. Byte offset is the
first offending byte and record index is the offending table index; unavailable values are
`UINT32_MAX`. Within each phase Rust, C and Python use header/table traversal order, then increasing
record index, then increasing field byte offset. When borrowed semantic validation returns multiple
sorted errors, map them to their corresponding wire fields/records and select the earliest by that
same order rather than by validator message order. Rust and C expose the same record.

## Product gates and evidence

- Two checked semantic vectors cover every enum, continuous/Boolean/enumeration parameters,
  per-lane/shared policy, optional sidechain, finite/infinite tails and all four launch rates.
  Golden wire/identity bytes have a sorted SHA-256 manifest.
- The Python-standard-library-only `scripts/effect-descriptor-v1-reference.py` encoder/verifier/
  identity tool independently reproduces both vectors from exact hexadecimal `f32` bits. Rust and
  reference bytes are identical. Vectors and their manifest live only under
  `fixtures/effect-descriptor/v1/`; the C smoke lives under
  `crates/miso-engine-effect-package/tests/c/`.
- Decode/re-encode is byte-identical. Every semantic field class changes identity; permutation of
  port input order does not. Truncation, trailing data, bad counts/offsets/order, aliases/gaps,
  unknown enums/flags, reserved/padding, malformed text and illegal floats reject with exact frozen
  diagnostics.
- Semantic validation parity is exhaustive and table-driven. Every launch-registry static
  descriptor and both comprehensive static vectors are accepted by the unchanged Issue-011
  validator and by encode/parse/private-borrowed validation. Differential parity exhaustively covers
  invalid descriptors representable through safe public constructors. Compile-time static fixtures
  cover contract/state versions; display text; parameter zero/duplicate/order; finite/negative-zero/
  default/domain rules; continuous bounds/mapping/log positivity; Boolean shape; enumeration
  length/order/value/label/duplicate and default rules; automation/automatable/smoothing consistency;
  port duplicate/role/required/layout/main-pair/sidechain cardinality; quality order/rate/Normal/
  accepted-rate coverage; and equal lane-state sizes. Rate fixtures cover launch rates
  44100/48000/88200/96000 and accepted optional extended rates
  176400/192000/352800/384000. Text fixtures use exact `char::is_control()` semantics. For every
  safely representable invalid fixture, the private validator returns the exact same sorted,
  deduplicated `(path, DescriptorDiagnosticCode)` set as `validate_descriptor_v1`.
- Invalid `EffectId`/`PortId` grammar and `LinkModeSet` values with unknown bits or missing DualMono
  are constructor-sealed and cannot be placed in a safe static `EffectDescriptorV1`. Exhaustive
  frozen grammar/boundary/bit mutations must prove `EffectId::new`, `PortId::new` and
  `LinkModeSet::new` reject them, then prove Rust, C and Python reject their raw-wire encodings with
  the exact frozen wire diagnostic. These cases must not claim or attempt a
  `validate_descriptor_v1` call.
- Semantic parity must not invent rules absent from Issue 011: `contract_minor` is not constrained;
  `readable` need not be true; the parameter list need not be nonempty; and no additional maximum
  for common state, scratch bytes or scratch-per-frame is imposed beyond representability and the
  accepted validator's existing rules.
- Wire-only corruptions (header/length, offsets, gaps/aliases, padding/reserved, unknown enum/flags,
  string-pool ownership and trailing bytes) remain a separate stricter matrix and may reject before
  semantic validation with the frozen wire diagnostic. Differential fixtures are `static`/`const`;
  tests must not manufacture `'static` data by leaking allocations.
- Rust and C buffer canaries prove exact-size success and size-minus-one all-or-none behavior.
  A native C11 compile/link/run smoke agrees on records, identity and diagnostics.
- Locked native tests/check/Clippy/rustdoc, `wasm32-unknown-unknown` compile, format, workspace and
  realtime policy/mutations, dependency/static scans and no checked-in generated artifacts pass.

Evidence records exact commands, toolchain/target identities, fixture sizes/hashes, reference hash,
C layout report, diagnostic matrix and strict Terra/Sol verdicts. No benchmark, timing, 100-process
run, target matrix or fuzz campaign belongs here.

## Allowed files and non-goals

Allowed crate implementation is only `miso-engine-effect-package`'s `wire.rs`, `diagnostic.rs`,
`lib.rs`, `Cargo.toml`, a new descriptor-only FFI module, header and tests. `cid.rs`, `package.rs`,
`state.rs` and `compile.rs` remain untouched provisional successor input. Also allowed are
`fixtures/effect-descriptor/v1/`, `scripts/effect-descriptor-v1-reference.py`, one bounded native C
smoke script and minimal workspace/realtime policy allowlist plus mutation updates for the local FFI
boundary. `effect-contract` and `effect-compiler` production files are read-only accepted
dependencies.

STOP rather than implement if the static encoder cannot call the unchanged Issue-011 validator, if
the private borrowed validator cannot prove the exhaustive exact diagnostic parity above, or if the
solution requires lifetime laundering, a public effect-contract/runtime/compiler change, or any
canonical-wire relaxation. This is the final briefing clarification; any further contract, API or
testability blocker triggers STOP/rescope rather than another amendment.

No package/archive/artifact/CID encoding or selection; state envelope/restore/migration; third-party
ABI; signatures/trust/repository; runtime trait change; graph/DSP work; fuzz/100-process/target
matrix; benchmark or listening. Issues 078–081 own those separable outcomes.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Versioned TOML schema and transactional session compiler
- Native effect runtime contract and conformance

## References

- [FIPS 180-4, Secure Hash Standard](https://csrc.nist.gov/pubs/fips/180-4/upd1/final)
- [WebAssembly core specification](https://webassembly.github.io/spec/core/)

## Terra corrected-pass STOP evidence

**FINAL STOP / RESCOPE REQUIRED.** Terra's corrected implementation pass produced a focused-green,
package-local partial checkpoint in `diagnostic.rs` and `wire.rs`: the static encoder calls the
unchanged Issue-011 validator; the implementation has the frozen header/table shapes, checked
caller-buffer behavior, borrowed package-local parsing and semantic validation, canonical port
ordering, identity hashing and focused diagnostic/canary/parity tests. This partial implementation
is not accepted as Issue-029 completion.

The final clarification still requires an invalid safe static descriptor fixture for port layout.
That fixture cannot exist. `PortLayout` is a closed accepted `#[repr(u32)]` enum with only
`DualMonoPlanar` (`miso-engine-effect-contract/src/lib.rs:133`), while the unchanged validator's
port rules compare that typed value only with `DualMonoPlanar` (lines 446, 453 and 460). An unknown
layout can be represented only as raw wire and rejected in the wire enum phase; it cannot be placed
in a safe `PortDescriptorV1` for an exact `validate_descriptor_v1` differential call. Unsafe enum
construction or an effect-contract test/public seam would violate the frozen scope. Unlike
EffectId, PortId and LinkModeSet, the final clarification did not authorize constructor-sealed
raw-wire-only treatment for `PortLayout` while continuing to require invalid static layout parity.

The initial pre-edit lifetime STOP and this final corrected-pass testability STOP exhaust the
authorized clarification/pass budget. Focused evidence before STOP: package check passed; all eight
new wire tests plus four existing package tests passed; focused all-target Clippy with warnings
denied passed; format check passed. No C FFI/header, checked fixtures, Python reference, cross-target
or broad workspace work began. `workload_invocations=0`, `benchmark_invocations=0` and
`timed_invocations=0`.

## Successor record

Issue 082, **Close canonical effect descriptor wire, identity, and C inspection ABI**, consumes
checkpoint `64900f2` only as focused-green technical input. It distinguishes safely constructible
typed semantic parity from constructor-sealed and closed-enum raw-wire diagnostics, then completes
the original C inspection, independent-reference, golden-vector and nonbenchmark product seal.
