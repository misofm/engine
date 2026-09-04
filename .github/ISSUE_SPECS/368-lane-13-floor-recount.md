# 368 — Re-derive the effect floor inventories against current lane lowerings

Split out of the #349 efficiency-audit tracker, **Wave 0** (correctness and gate integrity). Rows: **LANE-13**.
This issue is self-contained: implement from this body alone. Do not re-audit #349.

## Assignment

| | |
|---|---|
| Implementer | Codex |
| Verifier | Fable 5.1 (reads the diff, reruns every gate below, posts the tracker note on #349) |
| Branch | `codex/368-lane-13-floor-recount` off `main` |
| Land after | LANE-4 (#367) — the finding says one recount must cover both |
| PR | one PR, squash-merged, title prefixed with the row id(s) |

## Operating procedure (complete; no other instructions exist)

Repo: `/Users/bl/Documents/GitHub/misofm/engine` (GitHub `misofm/engine`). `N` is this issue's number.

**0. Claim.** `gh issue view N --json labels` must not contain `claimed`; if it does, stop, another agent owns it. Every issue in the "Land after" row must be `CLOSED` (`gh issue view <dep> --json state`); if not, stop and say which one is open. Then `gh issue edit N --add-label claimed`.

**1. Worktree.** Never work in the main checkout.
```sh
git -C /Users/bl/Documents/GitHub/misofm/engine fetch origin
git -C /Users/bl/Documents/GitHub/misofm/engine worktree add ../engine-N -b <branch from the Assignment table> origin/main
cd /Users/bl/Documents/GitHub/misofm/engine-N && export CARGO_TARGET_DIR=$PWD/target
```
Read `AGENTS.md` and `docs/REALTIME_DEPENDENCY_POLICY.md` once before editing.

**2. Scope.** Implement only the finding(s) in this issue, following any "Reconciled scope" / "Notes for the implementer" section first and the finding's "Proposed change" second. Locate code by the named functions; line numbers are from `main` `4797a544`. Anything else you notice goes in the PR under "Seen, not done" and stays untouched. No reformatting, renames, dependency bumps or new tooling. Where the finding lists ordered options, try them in that order; an option marked class B or owner ruling is never implemented: if you reach it, post the evidence on this issue and stop.

**3. Null result.** If the premise no longer holds on current `main` (code already matches the proposal, or the named code is gone), do not invent a change. Comment on this issue with a one-paragraph null note, remove the `claimed` label, and stop.

**4. Evidence, all required.**
- The exact "Verification gate" commands from the finding, output saved.
- `cargo test --workspace` in the worktree and on `origin/main`; pass counts must match unless this issue adds tests (state the delta).
- Any script you edited: also run its `scripts/test-<name>.sh` mutation harness.
- Render-path rows only: `scripts/check-realtime-policy.sh`, the bit-identity fixture suite named in the finding, and one run of `scripts/run-console-benchmark.sh` on the named row. Report before/after once; never tune.
- Codegen claims (an instruction that should disappear): attach before/after `llvm-objdump -d` (or `cargo asm`) for the named function.

**5. Deliver.**
- Commit: subject `<row-id>: <what changed>`, body listing the commands run, last line `Refs #N`.
- `gh pr create --base main` with sections: Summary; Rows closed; Gate output (fenced); Before/after; Seen, not done; Skipped (any step not done and why). Add `Closes #N` only when every row in this issue is done.
- Do not post on #349; the verifier does that. Do not `git push --force`, do not touch `main`, do not edit outside the worktree.
- Reply with the PR URL and the Skipped list.

## Finding (verbatim from #349)

#### LANE-13: floor-accounting gap term (a) is already closed in code but the ruling's tables still price it as open
- **Category:** architecture
- **Class:** N/A (accounting)
- **Severity:** low
- **Location:** `crates/lane/src/wide_impl.rs:39-64` and `:263-271`;
  `docs/rulings/effect-floor-accounting.md:128-129,134-136,183-186`; `tools/bench/src/floor.rs`;
  `scripts/console-benchmark-record-lib.jq`
- **Current code:**
```rust
// crates/lane/src/wide_impl.rs:264
fn select(m: Self::Mask, a: Self, b: Self) -> Self {
    m.select(a, b)   // wide 1.6.1 -> blend_varying_m256 == vblendvps on avx
}
```
- **Problem:** the brief's gap term (a) — "`bitselect` lowering to 3 bitwise ops instead of `vblendvps`" —
  **no longer exists**. Verified in `wide-1.6.1/src/f32x8_.rs:148-158`: `select` is
  `blend_varying_m256(if_false, if_true, mask)` under `avx`, i.e. one `vblendvps`, and
  `f32x4_.rs:236-248` is `blend_varying_m128` / `v128_bitselect` / `vbslq_f32`. The semantics argument is
  sound (`blendv` reads only the sign bit; every mask the trait can build is all-ones or all-zeros, and
  the four mask constructors in `kernels::builtins` are themselves ordered compares). But
  `effect-floor-accounting.md:134-136` still says "On the emitted code it is three — LLVM builds
  `(a & m) | (b & andnot m)`" and the compressor gap table at `:183-186` still attributes **+24
  instructions per channel-frame** to it. An implementer reading the ruling will re-do work that is done.
  Separately, `wide_impl.rs:108-117` records that `max`/`min` are now one instruction on x86 and wasm
  while the floors still price them at two — an owed recount that has not happened.
- **Proposed change:** re-derive the compressor/EQ/limiter/builtins inventories in
  `effect-floor-accounting.md` against the current lowerings (select = 1 emitted, `max`/`min` = 1 on
  x86/wasm and 2 on NEON, `exp2_int` = 2 after LANE-4), and re-issue the floor constants in
  `tools/bench/src/floor.rs` and `scripts/console-benchmark-record-lib.jq` in the same change, as that
  ruling requires. Do it **after** LANE-4, so one recount covers both.
- **Expected effect:** the standing authority every sealed console record is read against stops being
  conservative by an unquantified margin; gap term (a) is struck from the open list.
- **Rulings / constraints checked:** `effect-floor-accounting.md:108-117` explicitly says this recount is
  owed and must be an owner-visible change; this finding is the trigger, not an unauthorised re-pricing.
- **Verification gate:** `bash scripts/run-console-benchmark.sh` against the re-issued floors; the
  record validator (`scripts/console-benchmark-record-lib.jq`) must accept the sealed records unchanged
  apart from the re-priced floor fields.



## Constraints (from #349, binding)

- Unfused `(a*b)+c` everywhere; bit identity across scalar/W4/W8/wasm. A class A row moves **no rendered bit**. If the row turns out to need one, stop, re-class it B on #349, and do not implement.
- No new crate names, no renames for their own sake, no runtime SIMD dispatch, no `unsafe` outside the existing allowlist without a ruling.
- Render path is zero alloc/free/lock/syscall/I/O/log/panic (`scripts/check-realtime-policy.sh`, `docs/REALTIME_DEPENDENCY_POLICY.md`).
- One measurement per row where the finding names one, recorded, not chased. No tuning loops.
- Line numbers above are from `main` `4797a544` and may have drifted; locate by the named functions. If the premise no longer holds on current `main`, close this issue with a one-paragraph null note and append the same note to #349.

## Done when

1. The finding's **Verification gate** above is green, with the command output attached to the PR.
2. `cargo test --workspace` passes with the same pass count as `main`, plus the named policy scripts.
3. A one-paragraph note on #349: row id, commit hash, what changed (or the null result).

Spec mirror: `.github/ISSUE_SPECS/368-lane-13-floor-recount.md`.


---

## Reconciled execution brief (2026-09-04; supersedes conflicting assignment/procedure text above)

# Astra brief: #368 / LANE-13 — current-lowering floor accounting

Decision: APPROVED scope, root mirrors this brief into the local #368 spec and synchronizes the GitHub issue before implementation. Reviewed main 87926988. #367 is CLOSED; #378's owner ruling explicitly defers native AArch64 and unblocks #368. #368 is claimed by root for this workflow. This is the next eligible Wave 0 slice; no dependency exception is needed. #375/#389/#390 own unrelated live changes and must not be touched.

## Smallest closable outcome

One accounting-only PR makes the standing four kernel inventories and the Rust/jq floor composition agree with current lane operations, removes already-closed select lowering from the live gap list, proves the repriced records change only floor-derived data, and captures one honest descriptive console invocation. No DSP implementation or optimization is part of this issue.

## Frozen scope and authority

Read #368's existing body, AGENTS.md, docs/REALTIME_DEPENDENCY_POLICY.md, docs/rulings/effect-floor-accounting.md, docs/rulings/unfused-multiply-add-audit.md, crates/lane/src/wide_impl.rs, relevant compressor/EQ/limiter/builtin kernels, and tools/bench/src/floor.rs. The user's current assignment overrides the old model table: Astra briefs and reviews, Luna gets exactly one implementation attempt, Sol retries only if that attempt fails. Root owns Git checkpoints, pushes, PR creation, merge and issue synchronization. Implement in a new isolated worktree off synchronized origin/main; never use another active worktree or edit the shared main checkout.

Allowed files: docs/rulings/effect-floor-accounting.md; tools/bench/src/floor.rs; scripts/console-benchmark-record-lib.jq; focused additions/adjustments in scripts/test-console-benchmark.sh and existing floor.rs tests; a comment-only resolution of the owed recount in crates/lane/src/wide_impl.rs if needed; the #368 spec and brief/evidence record; new artifacts/issue368-floor-recount evidence. A minimal --issue368-floor-recount arm and usage text in scripts/run-console-benchmark.sh is authorized because all existing namespaces are historical one-shot authorities. Do not refactor that runner. Any other file needs a brief amendment first.

Explicit exclusions: rendered arithmetic, operation order, SIMD implementations, effect parameters, buffers, graph/compiler/protocol/host dependencies; class B changes; throughput-probe reruns; new benchmark framework; floor machine calibration; mono-collapse pricing decision; re-pricing the intentionally rounded reduction amortization; generic Rust/jq key-set tooling (TOOL-8); native AArch64 qualification. Keep BANK_WIDTH=8 and OPS_PER_CYCLE=3.7; state their historical host origin and do not imply they were calibrated on the current machine.

## Derivation contract

1. Recount expressions, not prose constants. For the current x86/Wasm lowerings max/min cost 1; select already had a 1-op floor, so closing the emitted select gap does NOT subtract more floor ops. `exp2_int_in_range` is the post-#367 two-operation synthesis on the admitted domain; callers retain their clamp. Do not label public clamped `exp2_int` a two-op method. NEON is unsupported/deferred; describe its source-level intended compare/select shape only with that qualification, never as verified emitted behavior.
2. Re-derive each line for compressor, EQ, limiter, builtins; identify exact called function for each changed line. Fractional per-channel counts are valid: a single shared stereo max saves 0.5 op per lane-sample. Do not round shared work up to an integer. Preliminary delta-only cross-checks, NOT acceptance oracles: compressor 94 - 12.5 = 81.5, limiter 138 - 8.5 = 129.5, EQ 51 and builtins 69 unchanged. Independently trace the source; report any discrepancy before broadening scope. Whole strip and derived row compositions must follow the justified totals.
3. Distinguish current authority from dated evidence. Keep historical measured cycles, timing, captures, and sealed artifacts untouched. The original +24 emitted-select gap and old disassembly counts must be explicitly historical, not asserted of current main. Add a clear current recount/closed-gap statement wherever a reader could mistake a historical optimization suggestion for open work. Recalculate current derived tables; historical tables may retain original pricing when clearly dated and labeled. Resolve the stale max/min recount debt, while preserving the separate unresolved mono-collapse ruling.
4. Do not claim instructions disappeared in this PR: it changes accounting only. Cite current source/lowering and existing #367 evidence for prior codegen changes. If making a fresh emitted-code claim, capture and identify the exact current function/artifact disassembly; never re-label old captures as current or transplant their measured instruction totals into the new table.
5. Rust and jq must independently express the same justified inventory and compositions. No widened tolerances, guessed clocks, renamed schema, changed output digests or weakened validators.

## Evidence gates (before PR)

- Root records baseline `cargo test --workspace` on origin/main and implementation worktree (PATH=/home/bl/.cargo/bin:$PATH). Candidate must pass with identical baseline count plus any explained focused tests. Save summaries/logs. Baseline infrastructure failures are candid blockers, not silently waived gates.
- `cargo test -p bench` for inventory/composition tests; `bash scripts/test-console-benchmark.sh` for validator acceptance and red mutations; `bash scripts/check-realtime-policy.sh`; formatting and proportional required policy gates. If the runner changes, run its existing console mutation suite and operator preflight, not an invented timed rehearsal.
- Add a focused counterexample to catch stale old floors in Rust/jq tests. Include a fractional stereo-link assertion and at least representative compressor+limiter compositions so a mistaken integer recount cannot pass. These should discriminate the accounting claim, not pin documentation prose.
- Historical-record comparison: copy representative sealed console records containing real cycle/floor columns to a new temporary/evidence namespace (never rewrite their originals). Update ONLY floor_cycles_per_lane_sample, percent_of_floor, and isolated_percent_of_floor according to current compositions and existing measured clock/timing/control rows; any other changed field must be explained and authorized. Compare JSON after removing precisely those fields for exact equality. Run the actual record and aggregate validators on the repriced set. Include standalone/ragged/unpriced rows when present. Prove stale old floor fields fail for affected rows and unrelated malformed fields still fail. Existing no-cycle records continue validating unchanged. This establishes the finding's 'sealed records unchanged apart from re-priced floor fields' without falsifying provenance.
- Freeze validator/workload and finish all non-timed gates FIRST. Preflight arguments, unique output namespace, output persistence, overwrite refusal, exit semantics and available counters without starting the timed workload. Then root authorizes the single benchmark invocation: `bash scripts/run-console-benchmark.sh --issue368-floor-recount`, exactly one warmup and two measured rounds under the existing runner contract. Use MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 only when host controls are unavailable and disclose the exact reason. Do not invent a core clock if perf is unavailable; omitted cycle fields are the supported representation, and the repriced historical-record proof above still exercises actual floor fields.
- If post-workload tooling fails: preserve raw/stdout/stderr/disposition, do not rerun or tune, report the failure immediately. One bounded tooling repair may be separately scoped; the issue cannot claim the original benchmark gate green while it is red. Root must rescope/split the qualification remainder rather than allowing this accounting feature to become a runner project.
- This is not an effect optimization loop: do not claim runtime speedups from floor repricing or describe historical isolated cycles as a current measurement. Where reporting residual gaps, give actual record-derived values, the new derived floor and a named reason; use 'not investigated' if necessary.

## Attempt/checkpoint/delivery rules

Luna executes one coherent implementation pass and focused verification, then pauses and sends exact paths plus evidence to root for an immediate compiling checkpoint. No second implementation tranche while that checkpoint is waiting. Root commits/pushes according to active delivery mode. Astra reviews the resulting PR against this brief and named gates before merge. A failed Luna attempt goes to Sol for a bounded revision (at most two Sol implementation attempts total); each gets one adversarial Astra verdict. After three failed attempts, stop and rescope; no disguised retry. Do not weaken any gate.

PR title starts LANE-13 and describes accounting. Include Summary, Rows closed, Gate output, Before/after (old/new inventories; no speed claim), Seen/not done, Skipped. Root closes #368 only after PASS and its evidence commit is upstream; then verifies CLOSED and posts #349's row/commit/outcome note. #349 remains an audit tracker, not authority for a cross-cutting cleanup branch.

## Attempt 2 decision/evidence record (2026-09-04)

Astra's attempt-1 verdict was **FAIL** at 57a3f86c: inconsistent derived cells,
historical/current contradictions, and missing namespace, immutable repricing proof, validator
negatives and source derivation. Sol attempt 2 retains 81.5/129.5/51/69 and corrects those findings.
The minimal operator-preflight namespace arm is an explicit scope amendment needed to preflight the
new runner namespace; it changes no preflight behavior. Durable evidence is under
artifacts/issue368-floor-recount/. Full workspace and timed gates remain root-owned and unrun.

Focused outcomes: historical repricing/equality/negative proof PASS (46 records; source SHA-256
9bb03dbbfa33e502fe05c0724bbfe6bf45d704236e0c85177b3ef1d02961ebcb); cargo test -p bench PASS
(30 passed); console validator mutation suite PASS with 0 workload launches; operator preflight
PASS with 0 workload launches, one warmup configured and two measured rounds configured. The
preflight also exposed and this tranche corrected its pre-existing repository-root resolution
(scripts/operator needs ../..). Logs: /tmp/engine-368-repricing.log,
/tmp/engine-368-bench-test.log, /tmp/engine-368-validator.log, /tmp/engine-368-preflight.out and
/tmp/engine-368-preflight.err.

## Spec-state finding

The local numbered-spec scan found no files for #366–#394, including #368's promised `.github/ISSUE_SPECS/368-lane-13-floor-recount.md`. Root reports all 172 existing numbered specs have matching remote issues. Before implementing #368, mirror its current remote body and this reconciled brief locally with exact issue number/title; synchronize the remote scope/role amendment. Do not manufacture new numbered issues for existing remote issues, edit other agents' active issue claims, or bundle unrelated historical spec restoration into this PR. Record remaining mirror debt as a bounded synchronization task.
