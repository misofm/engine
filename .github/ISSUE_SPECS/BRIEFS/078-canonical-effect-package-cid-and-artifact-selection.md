# Sol implementation brief — issue 078 canonical package, CID and artifact selection

## Decision

**SOL PASS / COMPLETE / READY TO CLOSE.** The allocation-boundary correction at `f295734`, Terra
checkpoints `6af336c`, `d375db3`, `05cfabd` and `ae02d2a`, and the bounded three-path Sol correction
produced clean candidate `40fbcff97f82210b81db112c42dc162cf156a6b1` on
`codex/batch-feature-078`. The full locked nonbenchmark workspace/policy seal passed. Workload,
benchmark, timed, audit, browser and general-target invocation counts are each zero.

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

## Final Sol verdict

**PASS.** The three-path correction sealed partial-table diagnostic priority, all five one-below
encode canaries, full Python diagnostic identity/overflow/artifact-limit behavior and exhaustive
ASCII CID prefix/alphabet rejection without changing frozen package bytes. The independent packages
are 2,547 and 1,327 bytes with SHA-256 values
`af7b5d38afd3191c33d9d40d95d933ff9b83fe949cb95c3d80bd7bbf916daa52` and
`6a5934e1222a8601c0aca2194da10f00cc5357596b6355f6c5d64baf748f532c`; manifest SHA-256 is
`74cb06877960c1675e24742b65df373254ce71341522e2196038053c9d571bf3`.

Executed evidence passed exact 96/72 package layout and canonicalization, checked diagnostics and
atomicity, exactly-one nested Issue-082 allocation with zero native/retained package allocation,
strict 36/59-byte CID coding, deterministic selection plus rehash, independent Python vectors,
native and scalar-Wasm checks, full locked workspace check/tests, warning-denied Clippy/rustdoc,
formatting, policies/mutations and static/diff/artifact scans. Issue 078 is complete and ready to
close; Issues 027 and 081 are unblocked.
