# 082 Close canonical effect descriptor wire, identity, and C inspection ABI

## Outcome

Accept the canonical V1 effect-descriptor wire, domain-separated identity and bounded C inspection
ABI begun at stopped Issue 029, using a test taxonomy that never constructs an invalid Rust enum or
launders a lifetime.

## Status and attempt budget

**SOL-BRIEFED / READY FOR TERRA ATTEMPT 1.** Permit one Terra implementation attempt and one bounded
Sol correction; a second failure stops. Workload, benchmark and timed invocation counts are zero and
remain zero.

Checkpoint `64900f2` is focused-green technical input, not an accepted dependency or PASS. Preserve
its 96-byte header, 80/24/64/16-byte tables, canonical string/port ordering, exact diagnostic phase
and tie-break order, checked caller-buffer APIs, borrowed verification and identity:

`SHA-256("miso.engine.effect-descriptor.identity.v1\0" || LE-u64(wire_length) || wire)`.

The public Rust names remain `effect_descriptor_wire_v1_required_size`,
`encode_effect_descriptor_wire_v1`, `verify_effect_descriptor_wire_v1` and
`effect_descriptor_identity_v1`. Static size/encode must call the unchanged Issue-011
`validate_descriptor_v1`; arbitrary bytes remain a private effect-package borrowed semantic view,
never an `EffectDescriptorV1`.

## Frozen two-domain validation taxonomy

1. **Safely representable typed states.** Every accepted launch-registry descriptor and both static
   comprehensive vectors pass the unchanged validator and borrowed verifier. Every invalid
   descriptor state constructible through safe public Rust APIs is covered exhaustively by static
   fixtures; the borrowed validator must return the exact same sorted, deduplicated
   `(path, DescriptorDiagnosticCode)` set as `validate_descriptor_v1`. This includes reachable
   contract/state, display text, parameter, port-topology and quality/rate/state-size rules.
2. **Constructor-sealed or closed wire states.** Invalid `EffectId`/`PortId` grammar and
   unknown/missing `LinkModeSet` bits prove exhaustive constructor rejection over the frozen
   grammar/boundary/bit matrix. Unknown discriminants for closed enums—including parameter unit,
   domain, mapping, automation rate, channel policy and smoothing; port role and the one-variant
   `PortLayout`; quality and tail kind—plus noncanonical Boolean/flag values are raw-wire cases.
   Rust, C and Python must reject each with the exact frozen code, byte offset and record index.
   These cases do not call or claim parity with `validate_descriptor_v1`.

No test or implementation may use invalid-enum construction, `unsafe` lifetime extension,
`transmute`, `Box::leak`, interning, a global cache or equivalent laundering. Do not change
`miso-engine-effect-contract`, its validator, the compiler or any public runtime seam. Exact
`char::is_control()` text semantics and accepted rates 44100/48000/88200/96000 plus optional
176400/192000/352800/384000 remain frozen. Do not invent Issue-011 constraints on
`contract_minor`, readable parameters, nonempty parameter lists, common state or scratch maxima.

## Remaining product closure and gates

- Finish the fixed header
  `crates/miso-engine-effect-package/include/miso_engine_effect_descriptor_v1.h` and sole symbol
  `miso_engine_effect_descriptor_v1_inspect`. Retain the Issue-029 record sizes, mandatory/null
  rules, exact required counts, all-or-none short-buffer behavior, 16-byte diagnostic and 64-byte
  summary. A C11 compile/link/run smoke checks every size/alignment/offset and matches Rust records,
  identity and first-error diagnostics.
- Check in two comprehensive vectors, canonical wire/identity bytes and sorted SHA-256 manifest
  under `fixtures/effect-descriptor/v1/`. The Python-standard-library-only
  `scripts/effect-descriptor-v1-reference.py` independently encodes, verifies and hashes exact
  hexadecimal `f32` inputs. Rust, C and Python bytes/diagnostics agree.
- Prove exact-size and size-minus-one canaries for Rust and C; decode/re-encode identity; every
  semantic field changes identity; input port permutation does not; every malformed header, length,
  offset, order, alias/gap, reserved/padding, enum, flag, text and float row rejects deterministically.
- Pass focused package tests/check/Clippy/rustdoc, native C smoke, locked workspace nonbenchmark
  gates, `wasm32-unknown-unknown` compile, format, workspace/realtime policy and mutation checks,
  dependency/static scans and no-generated-artifact checks.

Evidence records checkpoint/source/tool hashes, exact commands, vector sizes/hashes, C layout,
diagnostic matrices and strict Terra/Sol verdicts. Any need for an effect-contract change, unsafe
invalid typed value, wire/API/diagnostic relaxation or another testability exception is STOP/rescope.

## Allowed files and non-goals

Allowed files are Issue-029's existing `miso-engine-effect-package` wire/diagnostic/lib/Cargo surface,
one descriptor-only FFI module/header/tests, `fixtures/effect-descriptor/v1/`, the independent Python
reference, one C smoke script and minimal direct workspace/realtime policy allowlist plus mutations.
The package/CID/state/compile provisional modules and effect-contract/compiler production remain
read-only.

No package/CID/artifact selection, state envelope, migration, runtime trait or DSP/graph change,
third-party execution, broad fuzz/100-process/target matrix, benchmark, timing or listening.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Versioned TOML schema and transactional session compiler
- Native effect runtime contract and conformance

## References

- [FIPS 180-4, Secure Hash Standard](https://csrc.nist.gov/pubs/fips/180-4/upd1/final)
- [WebAssembly core specification](https://webassembly.github.io/spec/core/)
