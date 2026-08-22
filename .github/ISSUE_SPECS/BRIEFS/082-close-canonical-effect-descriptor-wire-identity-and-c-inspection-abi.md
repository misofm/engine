# Sol implementation brief — issue 082 descriptor wire, identity, and C inspection closure

## Decision

**READY FOR TERRA ATTEMPT 1.** Consume stopped Issue-029 checkpoint `64900f2` only as focused-green
technical input. Complete the already-frozen descriptor wire/identity/C-inspection product without
changing accepted effect contracts. Permit one Terra attempt and one bounded Sol correction;
benchmark and timed invocations remain zero.

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
