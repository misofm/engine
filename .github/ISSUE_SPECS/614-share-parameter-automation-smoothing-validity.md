# Share parameter automation and smoothing validity

GitHub: https://github.com/misofm/engine/issues/614

Parent: #560 (lane B, CP8)

CP8 currently duplicates the same four-field parameter validity law in `effect-contract`'s typed descriptor validator and `effect-package`'s borrowed wire validator. This issue extracts only that law into one allocation-free parts predicate owned by `effect-contract` and makes both production validators call it. It is the smallest closable CP8 product slice; the rest of the roughly 400-line descriptor-law duplication remains open under #560.

Sol HIGH coordinates the issue, documentation, checkpoints, GitHub synchronization, and artifact applicability. Luna HIGH or XHIGH implements. Per the current user routing, Astra LOW performs every scope, implementation, exact-head, integration, artifact-applicability, and delivery verification that would otherwise use Sol HIGH/XHIGH. Lane A's active #539 owns only the limiter implementation and its bounded tests; these paths are disjoint. This issue is lane B's second active slot across the two lanes.

## Exact ownership and outcome

Production ownership is limited to:

- `crates/effect-contract/src/lib.rs`
- `crates/effect-package/src/wire.rs`

Inline tests in those files and this numbered spec/evidence are also allowed. Introduce one plainly named public `const` or ordinary pure predicate in `effect-contract` over exactly:

- `AutomationRate`
- `bool automatable`
- `SmoothingRule`
- `u32 smoothing_samples`

The predicate must allocate nothing, perform no I/O, expose no wire vocabulary, and return the exact existing law:

- `AutomationRate::None` is valid only with `automatable == false`, `SmoothingRule::None`, and zero samples.
- `AutomationRate::Sample` and `AutomationRate::Block` require `automatable == true`.
- `SmoothingRule::None` requires zero samples.
- `SmoothingRule::Linear` and `SmoothingRule::OnePole99` require nonzero samples.

Replace only the duplicated condition in `parameter_valid` and `parameter_semantics_valid` with calls to that predicate. Preserve the validation statements before and after it in their existing order.

Do not share any other descriptor predicate. Do not change enum definitions, enum-choice storage, wire layout/schema/version, offsets, diagnostics, canonicalization, dependencies, public descriptor structs, effect-runtime smoothing, DSP, compiler behavior, artifacts, pins, workflows, or policies. If the predicate requires another abstraction, changes an accepted/rejected input, or broadens ownership, stop and return for rescope.

## Objective gates

1. Before implementation, push and synchronize this brief and #560, then obtain Astra LOW exact clean scope PASS.
2. Prove the shared predicate against an independent expected truth table covering all 36 combinations of the three automation-rate variants, both automatable values, three smoothing variants, and representative zero/nonzero sample lengths. The expected oracle must state the law independently rather than invoke or restate the production predicate through another helper.
3. Exercise both production callers. Retain the existing typed-versus-borrowed semantic differential tests and demonstrate unchanged acceptance/rejection for automation and smoothing mutations.
4. Retain the wire diagnostic phase, diagnostic code, record index, byte offset, and canonical encoded bytes. No parser/rejection reordering may occur.
5. Run focused debug and release tests for the affected crates, their full existing test suites, strict affected Clippy, workspace formatting/diff checks, and proportional descriptor/wire policy gates. No benchmark or timing invocation is authorized; this issue makes no performance claim.
6. Root checkpoints each coherent green tranche before more implementation. Astra LOW reviews the exact pushed source/evidence head and again after any current-main integration.
7. After source PASS, root decides ordinary native ABI and published AudioWorklet artifact applicability. Qualification or pinning is serialized with #539 and remains lane-B/root-owned. Byte-identical retained qualification may be attributed only with recorded source applicability; drift requires the existing static, resource, hermetic, and three-browser gates.
8. Open one PR only after the reviewed branch is ready. Require the repository's `qualification` check to succeed, verify live main immediately before guarded exact-head merge, verify merge parents and post-main qualification, synchronize #560 and this issue, then remove the clean delivered worktree.

One Luna implementation pass is initially authorized. A substantive finding receives at most the remaining attempts under the repository's three-attempt rule. No gate may be weakened and no fourth disguised retry is allowed.

## Astra LOW scope review

Astra LOW returned **PASS** at exact clean pushed brief
`a2679c6312bb3389355868f70f5f683e1d16523f`, based on current main/merge-base
`9e113be98cf31c1eaf4297b0a031518244b71c33` and synchronized tracker
`aace5842720b91cfb85331b7b414933019bbad90`. The preceding review failed only for one extra EOF
blank line; that byte was removed, `git diff --check` passes, and the local body now matches GitHub
#614 byte-for-byte. The reviewer confirmed the exact duplicate law, complete 36-case domain, both
production callers, frozen wire gates, bounded ownership and disjoint open #539. Luna HIGH/XHIGH
attempt 1 is authorized within the two named source files, their inline tests and issue evidence.
