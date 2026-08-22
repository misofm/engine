# Sol implementation brief — issue 029 canonical effect descriptor wire and identity

## Decision

**TERRA ATTEMPT 1 STOPPED PRE-EDIT / SOL BRIEF CORRECTION READY.** Terra exposed an impossible
briefing requirement: arbitrary borrowed bytes cannot become the accepted `'static`
`EffectDescriptorV1` without lifetime laundering. No code was edited. Implement only the corrected
descriptor-wire/C-identity product frozen in Issue 029. The corrected implementation pass is the
last permitted pass; do not edit accepted effect runtime/compiler semantics or begin package, state,
migration or qualification successors.

## Literal implementation order

1. Replace provisional `wire.rs` with checked size/encode/borrowed-verify/identity APIs using the
   exact 96-byte header and 80/24/64/16-byte tables. Size/encode takes the real static descriptor and
   calls unchanged `validate_descriptor_v1` first. Verification parses only into a private
   `BorrowedEffectDescriptorViewV1<'a>` and uses a private semantic validator; the public verified
   wire view is not an `EffectDescriptorV1`. Canonicalize only port ordering; retain legal float
   bits and exact first-use UTF-8 strings.
2. Implement the frozen diagnostic phase order and all-or-none caller-buffer behavior before adding
   the C adapter. Locally isolate the minimum FFI `unsafe`; workspace unsafe deny remains intact.
3. Add the exact C header/records and native C11 link smoke. Do not infer C layout from Rust wire
   layout or expose callable plugin entrypoints.
4. Check in two comprehensive semantic vectors, canonical bytes/identities and a sorted manifest.
   The Python stdlib reference consumes exact hexadecimal f32 bits and must not call Rust.
5. Differentially seal the private validator against the unchanged Issue-011 validator: all launch
   registry descriptors and both comprehensive static vectors accept, and compile-time static
   invalid fixtures covering every Issue-011 diagnostic/rule family return the exact same sorted,
   deduplicated `(path, DescriptorDiagnosticCode)` set. Cover 44100/48000/88200/96000 plus accepted
   optional 176400/192000/352800/384000 rates and exact `char::is_control()` text semantics. Do not
   add rules for `contract_minor`, `readable == true`, nonempty parameters, or common-state/scratch
   maxima that Issue 011 does not define. Separately seal every wire-only malformed class, every
   semantic field class, port permutation invariance, output canaries, native and Wasm compile,
   policies and focused workspace gates.

Rust, C and Python choose a failure by phase, then header/table traversal order, increasing record
index and increasing field byte offset. If semantic parity produces several sorted errors, map each
to its wire field/record and choose the earliest by this order, not validator message order.

Never reconstruct arbitrary wire as `EffectDescriptorV1`; never use `unsafe` lifetime extension,
`transmute`, `Box::leak`, interning, a global cache or equivalent lifetime laundering. Add no public
effect-contract seam. Stop if static encode cannot call the unchanged Issue-011 validator, borrowed
semantic validation cannot prove exhaustive exact diagnostic parity, any wire field remains
implicit, or C support requires a runtime-trait change. Benchmark/timed invocations remain zero.

## Successor boundary

Issue 078 owns canonical packages/artifacts/CID/selection; 079 owns current-layout state envelope and
transactional restore; 080 owns migration registry/chains; 081 owns joined qualification, fuzz,
100-process/multitarget evidence and the sole future rough benchmark.
