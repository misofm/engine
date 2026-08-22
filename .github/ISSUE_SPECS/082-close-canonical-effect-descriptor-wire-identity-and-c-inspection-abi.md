# 082 Close canonical effect descriptor wire, identity, and C inspection ABI

## Outcome

Accept the canonical V1 effect-descriptor wire, domain-separated identity and bounded C inspection
ABI begun at stopped Issue 029, using a test taxonomy that never constructs an invalid Rust enum or
launders a lifetime.

## Status and attempt budget

**SOL PASS / COMPLETE / READY TO CLOSE.** Terra attempt 1 plus the single bounded Sol correction
completed the product contract at clean candidate
`178753c1168e38da9c032e311cfb11a6ce9f4a66` on `codex/batch-feature-082`. Workload,
benchmark, timed, audit and browser invocation counts are each exactly zero.

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

## Final Sol evidence — 2026-08-22

**PASS.** Stopped Issue 029 checkpoint `64900f2` remains historical focused-green technical input,
not an accepted dependency or retroactive PASS. Terra completed the product surface through
`1291b0b`; Sol's bounded correction at `178753c` changed only these four qualification/reference
paths (541 insertions, 11 deletions), with no wire, FFI, header or golden-byte change:

- `crates/miso-engine-effect-package/src/wire.rs` (test module only);
- `crates/miso-engine-effect-package/tests/descriptor_v1_qualification.rs`;
- `crates/miso-engine-effect-package/tests/c/descriptor_smoke.c`;
- `scripts/effect-descriptor-v1-reference.py`.

The two checked vectors are exact: comprehensive A is 1,587 wire bytes with identity
`7d2f1ee79aa5833c546ea06548cb29e13b37f4ab690e9024f1480d2fdfade298`; comprehensive B is
712 wire bytes with identity
`9bbf09878bca3228ad67687bc492bcc84894181884cf4e3ab387231fb318148f`. Their sorted six-row
manifest SHA-256 is `43bf0eb6b69d0756e8e12323bd54704f1781537ba4c7e4a4b31f6aa578345010`.

Executed evidence proves both validation domains without invalid typed construction or lifetime
laundering: exhaustive safely representable validator/borrowed-verifier parity; constructor and
raw-wire ID grammar boundaries; bounded missing/unknown link bits; closed enum, Boolean and flag
diagnostics; canonical extended-rate and `char::is_control()` behavior; exact phase/tie-break field
offsets; decode/re-encode byte identity; semantic-field identity changes; port permutation
invariance; and deterministic malformed header/length/offset/order/alias-gap/reserved/text/float
rows. Exact-size and one-short Rust/C canaries, all-or-none publication, required-count/null/capacity
permutations and every C record field/reserved word passed. The C11 smoke linked the sole inspection
symbol and matched the 96-byte wire header, 80/24/64/16-byte records, 64-byte summary and 16-byte
diagnostic. The independent Python standard-library verifier, native C inspection and scalar Wasm
export/object/no-SIMD checks all passed.

The clean candidate passed the final nonbenchmark seal:

- `cargo fmt --all -- --check`;
- locked workspace all-target/all-feature check and tests;
- warning-denied locked workspace all-target/all-feature Clippy and rustdoc;
- `bash scripts/check-effect-descriptor-v1.sh`;
- workspace, realtime and effect-runtime policy checks plus their mutation suites;
- dependency, unsafe/lifetime-laundering, generated-artifact and `git diff --check` scans.

Final counters: `workload_invocations=0`, `benchmark_invocations=0`, `timed_invocations=0`,
`audit_invocations=0`, `browser_invocations=0`. Issue 082 is complete and ready for its evidence
commit to be pushed, GitHub synchronization and closure.

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
