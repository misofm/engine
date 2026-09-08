# Issue 603 attempt 1 adversarial review

Reviewer: Astra LOW

Reviewed head: `475ac5c249a85ab7e72074adf4010f5b5be9c399`

Verdict: **FAIL**. One bounded runner correction may proceed. Final preflight and capture remain
unauthorized.

The local and upstream heads matched, the tree was clean, `Cargo.lock` was unchanged, and the frozen
Rust sources were untouched. Ordered markers now gate accepted publication; runner disposition
publication checks failure; and the validator adds u64/nonzero bounds, unique missing metadata, and
the expanded seal workload fields.

Blocking findings:

1. The frozen Rust entry emits record issue 602 while the validator requires record issue 603. The
   issue-603 tooling seal remains issue 603, but validation and stubs must retain the frozen issue-602
   record identity.
2. Preflight rejects the committed `qualification/` directory and therefore cannot prepare this
   checkpoint. It must preserve qualification while reserving only new prepared/capture outputs.
3. Preflight seal publication still uses fallthrough-prone `ln && rm` before READY rather than
   explicitly failing the publication operation.
4. Build isolation omits release debug-assertion, overflow-check, and Cargo build-rustflags inputs,
   and no stub test proves refusal before build.
5. Copied-root identity cases retain an unrelated argv mismatch. Marker cases use records with
   identities unrelated to each copied seal, so validator rejection can mask marker failure.
6. The validator tests lack a missing-record case and a genuine rounds 2-then-1 reorder. The child
   prefix matrix omits the 8,192-call state, and failure cases need explicit status assertions and
   faults at the actual publication operations.

Safe checks passed for the existing assertions: the current stub lifecycle script, two untimed
capture arithmetic tests, all eleven qualification checksums and restored validator identity, plus
diff and clean-tree checks. No final preflight, real capture entry, or timing ran.

If a substantive runner defect remains after the authorized correction and Astra LOW rereview, this
issue must stop and split again.
