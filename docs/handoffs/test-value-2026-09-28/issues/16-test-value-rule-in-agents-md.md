# Add a three-sentence test-value rule to AGENTS.md, and make Sol's verdict ask it

Draft, not a GitHub issue. **Enforcement**, so the cleanup sticks. From the 2026-09-28 test-value
audit ([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §8-§9). Base `a9414c0c`. AGENTS.md is the
owner's document, so the wording is for the owner to accept.

## Problem

The 2026-09-04 test-usefulness audit (`docs/audits/test-usefulness-2026-09-04/`) produced a ledger
of about 400 verdicts. They needed per-item acceptance, and nothing enforced its rules afterwards.

**Result:** about 95 of its ~106 DELETE rows are still present, and none of the 20 sampled TRIM rows
was applied. The suite then grew by about 1,000 tests (1,553 → 2,553 `#[test]` attributes).

**New tests repeat the patterns the rubric named:**
- source scrapes: `graph/src/runtime.rs:7770`, `:7937`; `lane/tests/input_chain_elision.rs:939`;
  `source/src/lib.rs:2882`;
- re-pins in place of deletes: `graph-compiler/tests/track_delay.rs:253`,
  `math/tests/f1_fast_db_bounds.rs:1168`;
- a one-time class-A comparison frozen as a permanent digest, beside the differential that proves it
  (`tools/console-workload/tests/paired_spans.rs` gate 3);
- four tests for one #994 predicate that no recorded mutation distinguishes
  (`compressor/tests/knee_overflow.rs`).

**What it costs:** pins caused 78 of the 132 red CI jobs in September, and none caught a defect.

A rule is only worth adding if it is cheap and it discriminates (AGENTS.md "The ceremony boundary").

## Outcome

**1. AGENTS.md.** Under "Issue-first execution and review workflow", after the adversarial-review
steps, add this rule. It is proposed text; the owner edits it.

> **Test value.** A new test names the defect it catches that no existing test catches, and the
> `MUTATIONS.md` row recording that mutation red is the evidence. A regression reproducer of a real
> bug is exempt, and it is permanent. A change that supersedes a test deletes the superseded test in
> the same PR. An exact byte, count or digest equality is allowed only for a wire, ABI or on-disk
> format, or for the single owner of a cross-target corpus; budgets are ceilings, and a test never
> reads source or prose text.

**2. Sol's adversarial verdict** carries one question per new test: "which mutation turns this red
that no existing test catches?" A verdict is not a PASS while a new test's answer is missing.

**3. The mechanical half** is issue 07's one-line lint in `scripts/check-workspace-policy.sh`: no
`include_str!`/`read_to_string` of source or prose in tests. It gets no mutation suite.

**A duplicate-pin lint was prototyped and rejected.** The rule was: fail when a hex literal of 16 or
more digits appears in two tracked files. On today's tree it finds 106 such literals. Most are
legitimate:
- vendored libm constants shared between `log.rs`, `log2.rs` and `log10.rs`;
- the SplitMix and golden-ratio PRNG constants in about 20 files;
- the pinned GitHub Action SHAs in the workflows;
- `NOTICE` hashes.

It would need an allow-list, which is ceremony. Duplicate pins are left to the review rule.

**Not proposed:**
- a per-PR mutation gate: too slow and too noisy;
- a test-count budget: it discriminates nothing.

A weekly, non-blocking `cargo mutants --in-diff` report is optional; see the audit §9.

## Scope

Authorized paths: `AGENTS.md` (the paragraph) and this issue's spec. The lint itself lands with
issue 07.

## Gates

1. **The lint discriminates.** On a scratch branch, `include_str!("../src/lib.rs")` in any test
   fails `check-workspace-policy.sh`, and the unmodified tree passes. **Precondition:** issue 07 has
   removed today's 33 hits. The one non-test hit, `tools/bench/src/builtins.rs:279-283`, reads
   `fixtures/`, which the rule excludes.
2. **The lint is cheap.** It adds under 1 s to the lint job.
3. **The review rule is used.** The next three PRs that add tests carry the answer in their Sol
   verdicts; the root agent checks this at its checkpoint audit.

## Saving and risk

- **Saving:** none directly. It prevents regrowth, which the 2026-09-04 audit shows is the real cost.
- **Risk:** a legitimate duplicated literal, such as a wire vector quoted in two decoders' tests.
  Those belong in one shared fixture file, which the rule then allows.
