# Add one test-value question to Sol's verdict and one standing sentence to AGENTS.md

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. **Enforcement**, so the cleanup sticks. From the 2026-09-28 test-value
audit ([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §8-§9). Base `a9414c0c`. Paths starting
`../` are relative to the audit's handoff folder. AGENTS.md is the owner's document, so the wording
is for the owner to accept.

## Problem

The 2026-09-04 test-usefulness audit (`docs/audits/test-usefulness-2026-09-04/`) produced a ledger of
about 400 verdicts. They needed per-item acceptance, and nothing enforced its rules afterwards.

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
  (`compressor/tests/knee_overflow.rs`). One of them no longer goes red on the bug at all.

**What it costs:** pins caused 78 of the 132 red CI jobs in September, and none caught a defect.

A rule is only worth adding if it is cheap and it discriminates (AGENTS.md "The ceremony boundary").
**It must not become a new ledger.**

## Outcome

**1. Sol's adversarial verdict asks one question** for each new or changed test:

> Which plausible defect turns this red that no existing test catches?

- The answer is a sentence in the verdict. It is not a new file or row.
- `MUTATIONS.md` rows remain welcome evidence, but are not required per test.
- A verdict is not a PASS while a new test has no answer.
- **Exempt:** a regression reproducer of a real bug, which is also permanent.
- **Judged differently:** a randomized differential is judged by what its generator reaches (edges,
  restores, asymmetric words), not by unique catches. It overlaps other tests by design.

**2. AGENTS.md** gains one standing sentence, under "Issue-first execution and review workflow".
This is proposed text; the owner edits it.

> **Test value.** Every new test answers, in its adversarial verdict, which defect it catches that no
> existing test catches (real-bug reproducers are exempt and permanent); a change that supersedes a
> test deletes it in the same PR; and an exact fixture resource byte count, or a digest of a
> fixture's bytes, is used only for a wire, ABI or on-disk format or for the single owner of a
> cross-target corpus. Resource budgets are ceilings, while counts that *are* the claim, such as
> `allocations == 0`, stay exact.

**3. The mechanical half** is issue 07's one-line lint in `scripts/check-workspace-policy.sh`: no
literal-path `include_str!`/`read_to_string` of `.rs` or `.toml` in tests.

**A duplicate-pin lint was prototyped and rejected.** The rule was: fail when a hex literal of 16 or
more digits appears in two tracked files. On today's tree it finds 106 such literals. Most are
legitimate:
- vendored libm constants shared between `log.rs`, `log2.rs` and `log10.rs`;
- the SplitMix and golden-ratio PRNG constants in about 20 files;
- the pinned GitHub Action SHAs in the workflows;
- `NOTICE` hashes.

It would need an allow-list, which is ceremony. Duplicate pins are left to the review question.

**Not proposed:**
- a per-PR mutation gate: too slow and too noisy;
- a test-count budget: it discriminates nothing;
- a per-test ledger row.

A weekly, non-blocking `cargo mutants --in-diff` report is optional; see the audit §9.

## Scope

Authorized paths: `AGENTS.md` (the sentence) and this issue's spec. The lint lands with issue 07.

## Gates

1. **The rule is closable and objective.** The AGENTS.md sentence is present, and it names the
   verdict question. The Sol verdict template, wherever the review workflow keeps it, contains the
   question.
2. **It is applied to the audit's own cuts.** Each of issues 01-15 that adds or keeps a test records
   the answer for that test in its PR's verdict. For issue 14's first slice, that is the answer for
   the new randomized test.
3. **No new ledger.** The PR adds no file and no table whose rows grow with the number of tests.

## Saving and risk

- **Saving:** none directly. It prevents regrowth, which the 2026-09-04 audit shows is the real cost.
- **Risk:** reviewer fatigue. It is one question per test, with the answer already known to whoever
  wrote the test.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **As drafted, the rule would have prevented about 5 of the 10 counter-examples (finding F6).**

   | counter-example | prevented by the rule as drafted? |
   |---|---|
   | the four scrapes and `f1_fast_db_bounds.rs:1168` | **yes**, by the lint: each is a literal `include_str!` of a `.rs` file |
   | `track_delay.rs:253` | **no, and rightly.** It holds 2 unique product mutant catches, so it is not a counter-example |
   | `paired_spans.rs` gate 3 | **no.** It is a digest of rendered output, outside the pin clause |
   | the `knee_overflow.rs` quartet | **no.** They are exempt as #994 reproducers, although `:185` no longer goes red on its revert (re-confirmed) |

2. **Amend the AGENTS.md sentence:**
   - the pin clause reads "an exact resource byte count, **or a digest of fixture bytes, rendered
     output or a compiled artifact**, is used only for a wire, ABI or on-disk format or for the single
     owner of a cross-target corpus. A one-time 'no bit moved' comparison against a pre-change base
     is PR evidence, not a committed test";
   - the reproducer exemption reads "a regression reproducer is exempt while its PR records it red on
     the bug's revert; one found green on its revert is repaired or deleted".
3. **Gate 2 covers tests *added or rewritten* in issues 01-15, not "kept" ones.** Recording an answer
   for every kept test is a ledger.
4. **The lint** (issue 07) excludes literal paths containing `/fixtures/`. See issue 07's amendment 2.
5. **Allocation tests.** Add to the review question: a new allocation-count test measures with
   `bench_support::alloc`'s thread-scoped counters and warms process statics first. This replaces
   issue 13's part 2.
6. **Cost.** One question per new test, and a lint of about 1 s. It is cheap. The lint discriminates
   mechanically; the question discriminates only as far as Sol's verdict enforces "no PASS without
   an answer".
