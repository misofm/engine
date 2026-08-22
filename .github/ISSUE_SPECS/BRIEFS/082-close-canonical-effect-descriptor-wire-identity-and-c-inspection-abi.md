# Sol implementation brief — issue 082 descriptor wire, identity, and C inspection closure

## Decision

**SOL PASS / COMPLETE / READY TO CLOSE.** Stopped Issue-029 checkpoint `64900f2` remains
focused-green technical input only. Terra attempt 1 and the single bounded Sol correction completed
the frozen descriptor wire/identity/C-inspection product without changing accepted effect contracts.
Clean candidate `178753c1168e38da9c032e311cfb11a6ce9f4a66` on
`codex/batch-feature-082` passed the full locked nonbenchmark workspace/policy seal; workload,
benchmark, timed, audit and browser invocation counts are each zero.

## Literal implementation order

1. Preserve and re-audit the checkpoint's static encode validation, checked layout/encoding,
   private borrowed parser/semantic validator, exact identity and diagnostic phase/tie-break order.
   Repair production only if the frozen contract proves it necessary.
2. Split tests into exactly two domains. Safely constructible invalid static descriptors compare
   exact sorted/deduplicated Issue-011 validator errors with borrowed-validator errors.
   Constructor-sealed IDs/link sets and every invalid closed-enum/Boolean/flag raw value instead
   prove constructor/`from_raw` rejection where applicable and exact Rust/C/Python wire diagnostics;
   never claim an unchanged-validator call for an unrepresentable value.
3. Finish only the frozen C header/inspect adapter and its layout, null, capacity, atomic-publication
   and canary smoke. Keep FFI `unsafe` local and policy-covered; no Rust layout is wire layout.
4. Produce the two independent-reference golden vectors/manifest and seal byte, identity,
   permutation, malformed-row and first-error agreement across Rust, C and Python.
5. Run focused package/native-C/Wasm/policy gates, then the proportional locked nonbenchmark
   workspace seal. Record exact evidence and strict PASS/FAIL.

STOP on any effect-contract/compiler/runtime seam change, invalid typed enum construction, lifetime
laundering, relaxed wire/API/diagnostic contract or further testability exception. Do not enter
package/CID, state, migration or Issue-081 qualification work.

## Downstream boundary

Only accepted Issue 082 unblocks **Canonical effect package, CID, and artifact selection** and
**Prepared effect state envelope and transactional current-layout restore**. Issues 081, 027 and 026
remain transitively gated through their existing direct dependencies.

## Final Sol verdict

**PASS.** The bounded four-path Sol qualification correction proved exhaustive two-domain
validation, independent Python decode/re-encode identity, complete C record/null/canary semantics,
native C and scalar Wasm inspection, exact diagnostic ordering and semantic identity coverage. The
checked vectors are 1,587 and 712 bytes with identities
`7d2f1ee79aa5833c546ea06548cb29e13b37f4ab690e9024f1480d2fdfade298` and
`9bbf09878bca3228ad67687bc492bcc84894181884cf4e3ab387231fb318148f`; manifest SHA-256 is
`43bf0eb6b69d0756e8e12323bd54704f1781537ba4c7e4a4b31f6aa578345010`. Full locked workspace
check/tests, warning-denied Clippy/rustdoc, formatting, descriptor native/Wasm/Python checks,
workspace/realtime/effect-runtime policies and mutations, and static/diff/artifact scans passed.
Issue 082 is complete and ready to close after commit/push and remote synchronization.
