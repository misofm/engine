# Rubric — how "useful" was decided

This is the rule every verdict in `01-foundation.md`, `02-dsp-effects.md`, `03-compilers-hosts-tools.md` and `04-non-rust.md` was measured against. It is recorded here so a reader can disagree with a verdict on the rule rather than on taste.

## The test of usefulness

A test is useful only if **there is a plausible current code change that would make it fail, and no cheaper or stronger surviving test would also catch that change.**

Both halves are load-bearing. A test that nothing could break is dead weight regardless of what it is named. A test that something could break is still dead weight if a cheaper test in the same run, or a stronger test in a neighbouring layer, goes red on the same change; in that case the cheaper or stronger one is the owner and the other is a copy.

"Cheaper" and "stronger" are compared across the whole run, not within a crate. A script gate, a fuzz target, a release-profile leg or a sibling crate's test all count as surviving protection if they are actually invoked by CI. A gate that exists but is wired into no workflow does not count, and several were found in that state.

## What is not a reason to keep a test

- **"It is referenced by a closed issue."** A closed issue records why the test was written, not whether the claim is still protected somewhere cheaper. Retired requirements were treated as retired.
- **"It has a strong name."** Names were checked against assertion bodies. Where a name promises more than the body delivers, the verdict is on the body, and the mismatch is flagged.
- **"It has a recorded mutation."** A recorded red mutation is strong evidence of usefulness, and it was weighed as such, but a mutation that also turns another surviving test red does not make this copy the owner.
- **"It is fast."** Cheap duplicates still cost a binary link, a maintenance edit in lockstep with the code they transcribe, and a reader's time deciding which of two similar tests is authoritative.

## Standing verdicts

- **Parameter sweeps** are trimmed to **boundary values plus one interior representative**, unless the domain is genuinely discontinuous (a different code path per case, not a different number). Partition sweeps of the form `{1, 7, 64, 128, 512}` were repeatedly reduced on this basis: block size 1 crosses every boundary, and the ramp-window and quantum constants supply the rest.
- **Tautologies** are DELETE. A tautology is any test that re-derives a constant from the same code that defines it, compares a value to itself through a different spelling, or asserts a property that holds by construction of the type or the arithmetic.
- **Print-only and documentary tests** are DELETE. If a test's failure mode is "the number in the log changed", it belongs in `tools/bench` or in a recorded nightly, not in a blocking run.
- **Wall-clock assertions in a debug build are NIGHTLY** (release, with the number recorded). A debug-build timing bound on a shared runner measures the runner, not the product. The deterministic half of such a test (counts, allocation totals, diagnostic lines) stays blocking; only the timing half moves.
- **Source-text scrapes are brittle.** A test that `include_str!`s another file and greps it fails on a rename or a comment strip, and passes on the semantic change it was meant to catch. Every such test carries a note saying what it should assert instead (a `clippy::disallowed_methods` entry, a `(name, fn-pointer)` table, an argv actually executed, a value read from the shipped artifact).
- **Digest pins need exactly one owner.** Where the same corpus is rendered against the same constants in more than one place, one place is the owner and the rest are copies; the finiteness and non-vacuity checks that accompany a pin are a separate claim and survive independently.
- **MOVE is a verdict.** Several tests assert a neighbouring crate's laws; the verdict is to move them to that crate rather than to delete or keep them where they are.

## Verdict vocabulary

`KEEP` · `MERGE (into X)` · `TRIM (N cases to M, naming the representatives)` · `NIGHTLY` · `RELEASE-ONLY` · `MOVE (to crate X)` · `REWRITE (assert Y instead)` · `DELETE`.

Every DELETE and every TRIM names either the surviving protection or the retired requirement. A verdict with neither is not a verdict.

## Method and its limits

Every file in scope was read in full, including assertion bodies and loop bounds; nothing was modified. **`cargo` was not run**, so every cost figure is an estimate derived from the cited loop bound or corpus size, and figures the auditor could not corroborate are marked "unverified" inline. The measured CI context used to calibrate the estimates (debug, 4-vCPU) was: `builtins-compiler/tests/scale.rs` 22–42 s, `builtins-compiler/tests/allocation_tracker.rs` 18–41 s, `graph-compiler/tests/scale.rs` 22–42 s, `builtins/tests/response.rs` 18–37 s, `tools/console-workload/tests/chain_shape.rs` 35–62 s, `tools/wasm-gates` `g6_full_corpus_ftz` 29–61 s and `g5_native_corpus` 9–20 s.

Each file was additionally flagged for hidden global state, environment-variable reads, names that promise more than the assertion delivers, architecture-coupled skips that pass green without asserting, tests that are dead in CI behind an unenabled feature, and duplication of a script gate.

Where two audit streams reached different conclusions about the same test, both positions are recorded rather than silently reconciled; the consolidated comment lists those conflicts under "Uncertainties".
