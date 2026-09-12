# Reject ambiguous or misidentified vectorization probe bodies

Bug-fix child of #377. The broader decoder and platform-policy questions remain in the parent.

# Proposed #377 child: Reject ambiguous or misidentified vectorization probe bodies

## Scope ruling

APPROVED FOR ROOT TO FILE AND SYNCHRONIZE NOW as an independent scope-only checkpoint while #755 implementation runs. Revalidated against clean main `644756ffb59a2950fc3fa7b2c91b7f9f7435b485` after #307 delivery: all five inspected audit source, mutation, documentation, and allowlist files are unchanged from the original reviewed baseline. The source-traced findings and bounded correction remain valid. This approval permits issue/spec filing, not overlapping implementation: this child's implementation must wait until root commits #755's current implementation tranche, and verification must integrate latest main first.

Smallest closable reliability slice: the existing native vectorization audit must attribute each active probe rule to exactly one correctly identified disassembly body. A similarly named symbol must not stand in for a missing probe, and multiple bodies must not combine their instructions to satisfy one probe's required groups. This repairs existing certification behavior; it introduces no product feature.

Read-only scope review used baseline `c2c1368dcf4b8984fa7bdf8add5807e4336492ae`, GitHub #377 body and all comments available on 2026-09-12, current source/checkers/tests/docs, and AGENTS.md. No tests, builds, benchmarks, or reproducer execution were performed. The current false passes below are source-traced, executable regression designs, not claimed observed test runs. Implementation must establish red-before-green before claiming a confirmed fix. No library API research was needed because this slice requires no new dependency.

## Current evidence

- `tools/audit/src/vectorization.rs:215`: `symbol_bodies` recognizes headers ending in `>:` and containing `<`, then chooses the first registry symbol for which the entire header `contains(symbol)`. It does not compare symbol identity or count matches.
- At lines 227–236, `bodies.entry(symbol.clone()).or_default()` appends every matching body's text to the same string. Repeated matches are neither rejected nor kept separate.
- At lines 262–286, `certify` only checks whether that combined entry exists, then tests required groups against its combined text. Thus a missing real probe with a lookalike passes, and two incomplete bodies can jointly pass. `parse_allowlist`'s duplicate backend/symbol check concerns policy rows, not disassembly matches, and does not prevent either case.
- The current mutation script only changes required families, injects scalar/FMA/call instructions, or deletes a registry row. The current unit tests have no symbol-identity or ambiguous-body controls.
- The historical `tbl` / `*_block` substring-call bugs and eight-hex-word AArch64 encoding bug from #377 already have fixes and regression coverage at lines 518–585. They are not new findings.
- Current registry is three x86 rows only. `tools/audit/VECTORIZATION.md` explicitly retires native AArch64 under #378. #377's proposed six-row cross-target decoder rewrite is stale with respect to this ruling.

## Red-before-green controls

Use the existing `certify` test seam and actual checked-in x86 allowlist where practical; populate other active probes with valid synthetic instruction bodies so each mutation isolates one failure. Tests must assert the specific ownership/absence failure, not merely any nonempty failure list.

1. **Missing exact probe, suffix impostor:** replace the gain header `0000 <audit::vectorization::probe_gain_simd8>:` with `0000 <audit::vectorization::probe_gain_simd8_impostor>:` and retain a body containing `vmulps %ymm0, %ymm1, %ymm2`. Current code accepts the impostor as the gain probe. Fixed code must report the actual probe missing. Include a prefix impostor and a probe name embedded in an unrelated path as table cases.
2. **Split requirements across repeated bodies:** replace the one SVF body with two headers for `audit::vectorization::probe_svf_simd8`, at different addresses. First body has only `vmulps %ymm0, %ymm1, %ymm2`; second has only `vaddps %ymm0, %ymm1, %ymm2`. Both avoid forbidden instructions. Current concatenation satisfies all SVF required groups and passes. Fixed code must reject ambiguous probe-body attribution before certification.
3. **Duplicate independently valid bodies:** each repeated SVF body contains both required vector families. Still reject ambiguity; do not choose the first/last match or rely on a missing-family failure. Repeat at the same address as a duplicate header, defining repeated body headers as ambiguous for this existing disassembler interface.
4. **Green controls:** a unique exact probe per row remains green; unrelated headers containing similar probe names do not contaminate a valid exact probe's body; bodies in different listing order remain green. Include real supported demangled header spelling captured during implementation (GNU and LLVM if both installed), with the precise optional Rust demangler suffix spelling actually needed. Preserve rejection of genuinely absent probes.
5. **End-to-end mutations:** extend the existing objdump wrapper harness to rename the exact gain header into a near-name and to duplicate/split the SVF body. Each must emit JSON `status:fail` and the intended `probe symbol ... absent` or new explicit ambiguity failure class. Include a wrapper green control passing unmodified disassembly to demonstrate the wrapper itself is sound.

Synthetic fixture text is permitted evidence about this parser. It is not proof that the real compiler currently emits duplicate probe symbols. The defect is the checker accepting an invalid subject, not a demonstrated shipped DSP regression.

## Implementation contract and exact paths

Allowed implementation path: `tools/audit/src/vectorization.rs` (header/body extraction, certification error propagation, focused unit tests). Recognize the existing disassembler symbol-header envelope and compare the complete intended probe path rather than searching arbitrary text. Actual supported demangler suffixes may be normalized deliberately, with positive and negative tests; do not invent a permissive arbitrary-suffix grammar. Fail absent/ambiguous matching explicitly. A matching body is never silently merged with another.

Allowed integration/evidence paths: `scripts/test-native-vectorization-report.sh`, `tools/audit/VECTORIZATION.md`, `tools/audit/VECTORIZATION_MUTATIONS.md`, and the matching new `.github/ISSUE_SPECS/<number>-<slug>.md` / index if required by the current repository index convention. Root creates the matching GitHub issue and local spec in the same checkpoint, verifies number/title, and owns all commits and remote synchronization.

No edits to production kernels, lane arithmetic, manifests, Cargo.lock, allowlist content/shape, report JSON schema, artifact/disassembly hash meanings, CLI flags, target policy, or CI workflow. No new object container / typed decoder abstraction, dependency survey, workspace audit, AArch64 row resurrection, tail-call analysis, instruction-token overhaul, benchmark machinery, or byte-patching framework. These are separable #377 parent decisions, not necessary to close this proven attribution defect.

## Objective gates

1. Establish and record both current source-traced false passes with focused tests before implementation; if they do not reproduce after revalidation, stop and rebrief instead of substituting historical bugs.
2. Focused `cargo test --locked -p audit vectorization` passes after the correction; preserve existing missing-family/scalar/FMA/call/registry tests. Run formatting for touched Rust.
3. Build the native release audit once and run its ordinary vectorization report and extended mutation script against that exact binary. The ordinary report remains green with three x86 rules and the same schema/hash definitions; new invalid subjects reject for the intended failure classes. These commands execute probe certification, not timed performance benchmarks.
4. Inspect/report available GNU/LLVM compatibility using the same release artifact; both if installed, otherwise disclose the tested disassembler/version and do not install a target matrix to close this slice.
5. Astra medium adversarial review verifies exact identity, duplicate rejection, boundary termination, realistic demangled paths, no accidental AArch64 claim, and scope containment. No workspace-wide test/build matrix or listening/benchmark work is necessary for a tooling-only attribution correction.

## Workflow and attempt bound

Honor the user's requested Astra medium scope -> Luna max implementation -> Astra medium review roles, overriding generic Sol/Terra role labels. Maximum **two coherent implementation attempts**, one adversarial verdict per attempt. On second FAIL, retain evidence and stop for a newly bounded rebrief; no hidden third retry. If correcting symbol attribution unexpectedly needs object parsing/dependencies or a larger policy redesign, stop immediately and rebrief under #377 rather than growing this child.

Root may now create and synchronize the matching child issue/spec as a scope-only checkpoint. Before implementation, root must confirm #755's current implementation tranche is checkpointed, verify clean isolated ownership and issue/spec state, and record the starting SHA. #755's separately owned workflow/test paths do not overlap this child's allowed audit paths; do not touch its worktree or broaden either issue. Integrate latest main before verification and revalidate any intervening changes to the named functions/fixtures. Checkpoint as soon as the focused implementation tranche is green; follow the session's current batching/push mode. Close only this child after review PASS, evidence upstream, and GitHub synchronization. The broader #377 remains open.

## Attempt 1 implementation evidence

Luna max reproduced both false passes before modifying production parsing: the near-name and repeated-body regression tests both failed with an empty certification failure list (test exit 101). The correction matches the full probe module path, permits only the documented narrow Rust disambiguator, stores bodies separately, and rejects multiple matching bodies explicitly before instruction requirements.

Focused tests: 12 passed. One native release build produced audit SHA-256 `f4187cec06b54a7e787419049a46049bbb6d4a18f86d0d179fdbdb9b57b3860d`; normal report passed with three x86 rules. LLVM 18.1.3 and GNU Binutils 2.42 both passed against that same artifact. The extended mutation harness passed, including the unmodified wrapper, near-name absence and split-body ambiguity. No benchmark workload was run. Logs are preserved as `/tmp/issue758-attempt1-*`, including baseline failure, focused tests, release build, ordinary/GNU reports, disassembler identity and mutations. Only the four allowed implementation/documentation paths changed; Astra medium verdict is pending.

## Attempt 1 adversarial verdict: FAIL

Astra medium FAIL at `90631be59d2aeb98a7116a51bfb64ff85e8885f3`. Independently replacing the real gain header with `Disassembly of section <audit::vectorization::probe_gain_simd8>:` still produced exit 0/status pass on the same release artifact. `symbol_header` accepts any nonempty prefix, so a non-symbol line can own the probe. Evidence: `/tmp/issue758-astra-invalid-header.log`, wrapper `/tmp/issue758-astra-invalid-header-objdump`, full review `/tmp/issue758-astra-verdict.md`.

The second and final attempt must require a nonempty hexadecimal address token followed by whitespace before the outer symbol opener, retain correct real-header boundaries, and add malformed/prose/instruction-prefix rejection with ordinary annotation controls. Correct the independently-valid duplicate test so a single body first passes its actual rule; add a true prefix impostor. All other reviewed scope and recorded gates were sound. This is a bounded correction within the existing allowed paths, not a decoder expansion. It waits for the current #391 implementation tranche checkpoint. No third attempt is authorized.

## Attempt 2 implementation evidence (final attempt)

Luna max tightened symbol headers to a complete hexadecimal address token followed by whitespace and the outer symbol opener. Malformed prose and instruction prefixes cannot own a body; unrelated real headers still terminate it. Added malformed-prefix/annotation-boundary controls, a true prefix impostor and a duplicate test that first proves its single body passes. The end-to-end wrapper includes Astra's section-heading-shaped invalid header.

All 14 focused tests, formatting and diff checks pass. One release rebuild produced audit SHA-256 `6c5632216ec11817d4a26da4c9a2417778f704293689bb3e652ea8944ad87492`. Ordinary report and mutations pass; LLVM 18.1.3 and GNU Binutils 2.42 pass on this same artifact. Astra's prior wrapper now fails explicitly for missing probe symbol (exit 1). No timing benchmarks were run. Evidence is `/tmp/issue758-attempt2-*`; prior attempt and failure evidence remain preserved. Only the same four allowed implementation paths changed. Final Astra verdict is pending; no third attempt is authorized.

## Final adversarial verdict: attempt 2 PASS

Astra medium PASS at clean integrated head `bf0bb679d325024486bf3c7657896eebbab39043`, including latest main `8bfbb5b6` with unchanged audit code. Independent execution of the original invalid-header wrapper against the same attempt-2 artifact now returns exit 1/status fail and exactly the missing gain-probe diagnostic. The strict address envelope, body boundaries, true prefix case and independently-valid duplicate fixture are sound. All recorded focused, normal, mutation and GNU/LLVM gates remain valid; no new scoped blocker was found.

Final review is `/tmp/issue758-astra-verdict-attempt2.md`, independent output `/tmp/issue758-astra-attempt2-independent.log`. Attempt-1 FAIL remains preserved. This completes the authorized two-attempt workflow successfully. Required delivery CI remains pending at this evidence checkpoint; only #758 may close after synchronized delivery, while broader parent #377 stays open.

## Delivery CI block and separately scoped successor #762

PR #761 qualification run 34701531963 failed Clippy at vectorization.rs:233 because its nested if-let blocks must collapse into a let chain. Captured log: `/tmp/issue758-ci-clippy-failure.log`. The prior semantic PASS remains recorded, but delivery is blocked. #758's two-attempt limit is not extended: #762 owns only the syntax-equivalent lint correction, actual full-workspace Clippy, existing focused tests and one new Astra verdict. No lint suppression or gate weakening is permitted. Prior disassembler evidence remains attached to its named attempt-2 artifact, not relabeled as a new-head run. Both issues remain open until updated-head CI and delivery pass.
