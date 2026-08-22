# Sol implementation brief — issue 078 canonical package, CID and artifact selection

## Decision

**READY FOR TERRA ATTEMPT 1.** Implement one cohesive control-plane product: canonical package
bytes, verified exact-byte CIDv1 and deterministic selection over that verified table. Existing
package/CID code is provisional technical input. Permit Terra once and one bounded Sol correction;
the second failure stops. Accepted Issue 082 is present at merge
`fb054bae41777585d12a48e71c99a2cfa9c3e3e4`. Benchmark and timed invocation counts remain zero.

## Frozen implementation order

1. Replace provisional owned/`Vec` verification with the exact 96-byte header, 72-byte variable
   record prefix and descriptor/table/content layout in the issue. Implement checked layout and the
   4 MiB/16 MiB/256 MiB/4,096/128 MiB default limits before encoding.
2. Define borrowed authoring records, `required_size`, atomic caller-slice encoding, borrowed
   verification/artifact iteration and the exact 32-byte diagnostic. Full preflight precedes every
   output write; parsing uses checked `u64` arithmetic and `usize`/`isize` fit.
3. Call the accepted Issue-082 verifier and identity for the embedded descriptor. Enforce exact
   record sorting, contiguous content, zero padding/reserved bytes, path/target/feature grammars and
   per-content SHA-256. Required-size, encode, verify and package-CID each use exactly one accepted
   validation-and-identity pass before publication; its temporary heap is permitted only under the
   4,194,304-byte descriptor cap and dies before return. Do not call verifier and identity as two
   passes, and do not change descriptor or effect-contract code.
4. Implement the strict 36-byte raw/SHA2-256 CID and 59-byte lowercase unpadded base32 codec.
   Package CID creation verifies first; text output is caller-buffer atomic.
5. Implement exact-kind/target feature-subset selection: greatest feature count, then smaller
   feature string, then smaller path. Validate sorted request capabilities and rehash selected
   content immediately before return.
6. Author two independent Python vectors first, freeze manifest/package/CID/content identities, then
   consume them from Rust. Add exact mutation, canary, limit, permutation, selection and CID codec
   tests without entering Issue-081 fuzz/100-process/target/benchmark scope.
7. Run focused package/native/scalar-Wasm/reference/policy gates, then the proportional locked
   nonbenchmark workspace seal and record a strict verdict.

## Non-negotiable review points

- The accepted Issue-082 validation-and-identity pass is the sole allocation exception. Exact
  allocator-dependent temporary bytes are deferred to Issue 081; Issue 078 proves one pass, the
  exact descriptor cap and no surviving allocation.
- Native package layout/sort/parse, artifact iteration/selection and CID binary/text remain
  allocation-free and caller-buffered/borrowed. The verified package and selected artifact borrow
  immutable input; there is no retained allocation, package-sized copy or hidden sort `Vec`.
- Header descriptor identity is Issue-082's domain-separated identity, while each table digest is
  plain SHA-256 over exact content and CID digest is plain SHA-256 over the exact whole package.
- Package verification error priority follows the frozen phases, not whichever check is easiest to
  execute first; within a phase use record index then field offset.
- Source target/features are empty, CoreWasm target is exact, native target grammar is explicit,
  features are sorted/unique, and selection has no implied feature or target fallback.
- Invalid or short caller-buffer operations publish nothing. All count/length/offset additions and
  conversions are checked before slicing.
- No package C ABI, runtime trait, state/migration, executable validation, resolver, trust or render
  integration is part of Issue 078.

## Stop conditions

STOP/rescope rather than changing Issue 082, adding lifetime laundering or unsafe typed values,
relaxing canonical bytes/diagnostics, adding any allocation beyond the one accepted nested
Issue-082 pass, retaining allocation or proportional hidden package state, introducing a
runtime/compiler seam, or pulling Issue-081 qualification machinery into this product issue.

## Downstream handoff

Accepted Issue 078 unblocks **Third-party WASM package and effect ABI conformance kit**. Broad
interchange fuzzing, 100-process/multitarget evidence, allocation audit and the sole later benchmark
remain in **Canonical effect interchange qualification, fuzzing, and benchmark** after Issue 080.
