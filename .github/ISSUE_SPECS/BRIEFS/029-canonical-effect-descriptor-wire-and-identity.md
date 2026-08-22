# Sol implementation brief — issue 029 canonical effect descriptor wire and identity

## Decision

**READY FOR TERRA ATTEMPT 1.** Implement only the complete descriptor-wire/C-identity product frozen
in Issue 029. Use one Terra attempt plus one bounded Sol correction. Do not edit accepted effect
runtime/compiler semantics or begin package, state, migration or qualification successors.

## Literal implementation order

1. Replace provisional `wire.rs` with checked size/encode/borrowed-verify/identity APIs using the
   exact 96-byte header and 80/24/64/16-byte tables. Canonicalize only port ordering; retain legal
   float bits and exact first-use UTF-8 strings.
2. Implement the frozen diagnostic phase order and all-or-none caller-buffer behavior before adding
   the C adapter. Locally isolate the minimum FFI `unsafe`; workspace unsafe deny remains intact.
3. Add the exact C header/records and native C11 link smoke. Do not infer C layout from Rust wire
   layout or expose callable plugin entrypoints.
4. Check in two comprehensive semantic vectors, canonical bytes/identities and a sorted manifest.
   The Python stdlib reference consumes exact hexadecimal f32 bits and must not call Rust.
5. Seal representative malformed rows, every semantic field class, port permutation invariance,
   output canaries, native and Wasm compile, policies and focused workspace gates.

Stop if complete semantic reconstruction cannot call the unchanged Issue-011 validator, if any wire
field remains implicit, or if C support requires a runtime-trait change. Benchmark/timed invocations
remain zero.

## Successor boundary

Issue 078 owns canonical packages/artifacts/CID/selection; 079 owns current-layout state envelope and
transactional restore; 080 owns migration registry/chains; 081 owns joined qualification, fuzz,
100-process/multitarget evidence and the sole future rough benchmark.
