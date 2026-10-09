PASS

# #1481 attempt 1: adversarial verdict

Commit `fb5898857` (parent `ae8b09318`), branch `codex/d15-batch-misc`. I exported both commits with `git archive` and built them at `/tmp/claude-1002/v1481` (`CARGO_TARGET_DIR=/tmp/claude-1002/v1481/target`). I did not change the worktree.

Paths touched: the spec, `.github/workflows/nightly.yml`, `scripts/test-test-support-ci.py` and `tools/audit/src/vectorization.rs`. All four files are authorized. Only the `nightly.yml` header goes past the authorized lines (NIT 1).

## BLOCKER

None.

## MAJOR

None.

## MINOR

None.

## NIT

1. **`nightly.yml:6-8` header rewrite: outside the authorized lines, but D3 makes it necessary. It is accepted.**
   - The spec allows only the step's `continue-on-error` line and its comment (`:63-64`).
   - The old header said the job "carries `continue-on-error: true` on its only assertive step ... it cannot fail a build by construction". After D3, both parts of that sentence are false. Without the rewrite, the file would contradict itself: line 7 would say "cannot fail by construction", while line 64 would say "fails this job".
   - This is the same fact as the step comment, in the same file. The other three header lines and the "(non-blocking)" job name are unchanged. It is not unauthorized scope.
   - The spec author missed one consequence of D3. The implementer found it and flagged it in the attempt record (spec `:129-131`).
   - Action for root, not for the implementer:
     - amend the spec's Authorized paths entry to name the header bullet;
     - widen the J #1481 cell of the STREAMS hot-file row (`docs/handoffs/decision-15-2026-10-05/STREAMS.md:104`) to match, so that the record agrees with the diff.
   - Wording (optional): "cannot fail a build" (`:7`) would be more exact as "cannot fail a merge". The nightly run itself can now fail, and the next clause says so.
2. **The objdump description in the attempt record has the wrong count (spec `:160`).** The record says the frame loop has "two `vcmplt_oqps`". The loop has three: `dc3b2` compares against the rest plane in memory, and `dc3b8` and `dc3c7` compare against the per-word threshold in `%ymm9`. My build has the same loop addresses (`dc360..dc3f4`), so this is the same code. This error is in evidence prose only. No claim depends on it.

## Item (a): ruling

The header rewrite is a necessary consequence of D3, not an unauthorized edit. NIT 1 has the details. Keep it, and amend the spec's Authorized paths so that the record matches the diff.

## Item (b), D4: ruling

**D4 is satisfied as written.**
- `svf_block_form::<L, false>` is a production instantiation. `parametric-eq`'s two `UnarmedRest` calls (`crates/parametric-eq/src/lib.rs:3191`, `:3261`) reach `svf_block::<L, R>` at `:1663` through `process_section`. This happens on non-stationary (ramping) blocks, for sections that are not HPF/LPF.
- The record says correctly that the report does not cover the unarmed form.
- The fixed probe still reaches the armed loop. Its loop has the per-frame rest-plane load `vcmplt_oqps (%rax,%rdx,4)` and the joint `vpmaxud`. Thus the condition that would put an unarmed probe in scope is not met.

**Standing ruling ("codegen decides where no benchmark document reaches a site").** I applied it.
- I added a temporary `probe_svf_unarmed_simd8` (`svf_block::<Simd8, UnarmedRest>`) in my scratch tree and disassembled it.
- The result: one 8-lane `ymm` frame loop with `vmovups`, `vsubps`, `vmulps`, `vaddps`, the per-word `vandps`/`vcmplt_oqps`/`vandnps` flush, and `vmovups`.
- It has zero calls, zero scalar `*ss` ops and zero `vfmadd*` instructions. It would pass the existing `recursive-svf` row today.
- The codegen at this site is therefore clean. No defect needs a fix.

**A follow-up issue is recommended, but it does not block this issue.**
- Of the two forms of this kernel, the report holds only the less frequent one. The armed form runs only on blocks near silence. Every live-audio ramping block runs the unarmed form.
- The per-word step, `svf_step_when(false, ..)`, is also the step that the builtins chain kernels run on live blocks (`crates/lane/src/kernels/builtins.rs:669` ff.).
- My objdump is a one-time observation, not a guard. If a regression is unique to the unarmed branch, the nightly will not see it.
- Smallest slice: one AVX2 probe for `svf_block::<Simd8, UnarmedRest>`, one allowlist row with the same columns as `recursive-svf`, and the `Simd4` twin. Do not change the kernel.

## Test value

The commit adds no new test. The spec's test-value claim is: "Gates 2 and 3 show the existing rule is red for the probe's own call and for a real call in the body, and green only with the fix." I re-ran both gates and the claim is true (below). The committed `test-native-vectorization-report.sh` mutation `call-inside-kernel` already holds the synthetic form of gate 3. Thus the gate-3 evidence correctly stays PR evidence.

## Gates run (by the verifier, on the exported trees)

1. **Gate 1: PASS.**
   - `bash scripts/run-native-vectorization-report.sh` on `fb5898857` gave `"status":"pass","kernel_rules":3,"failures":[]` (artifact `b0171dde...`; it is different from the implementer's digest because the build paths are different).
   - `bash scripts/test-native-vectorization-report.sh <target>/release/audit` gave `native vectorization red mutations: ok`.
   - The fixed `probe_svf_simd8` body is call-free: no `cmp` against the plane length and no `slice_index_fail` before the loop. The loop is `dc360..dc3f4`, and it ends with `addq $0x8; cmpq $0x100; jne`.
   - The rest pointer, coefficients and state go through `black_box` (stack store and reload). The rest words are loaded from memory on each frame, so no value is folded. `#[inline(never)]` stays. The spec's hazard is respected.
2. **Gate 2 (revert D1 only): RED.** I put the parent's `vectorization.rs` into the commit tree. The result was `"status":"fail"` with `["recursive-svf / probe_svf_simd8: forbidden call 'call|callq' is present"]`. The body has `cmpq $0x100,%rdx` and then `callq core::slice::index::slice_index_fail`. When I restored D1, the report passed.
3. **Gate 3 (rule still bites): RED in two forms.**
   - (a) The implementer's form: an `#[inline(never)]` out-of-line call in the probe body before the kernel. The forbidden-call message appeared.
   - (b) A stronger form: an `#[inline(never)]` call inside `svf_block_form`'s frame loop (a temporary edit to `crates/lane` in the scratch tree). `callq ... verifier_out_of_line_frame` appeared inside the loop, before `jne`, and the forbidden-call message appeared.
   - After both runs, the tree is identical to the commit again (checked with `diff`).
4. **Gate 4: all green.**
   - `cargo clippy --locked -p audit --all-targets -- -D warnings`: rc 0.
   - `cargo fmt --all -- --check`: rc 0.
   - `python3 scripts/check-ci-path-routing.py`: "contract passed".
   - `python3 -B scripts/test-ci-path-routing.py`: rc 0.
   - `python3 -B scripts/test-test-support-ci.py`: rc 0.
   - `bash scripts/check-workspace-policy.sh`: ok.
   - `cargo test --locked -p audit vectorization`: 13 passed.
   - D5: `scripts/test-test-support-ci.py` has no line over 100 columns now. The rewrap of `:149` moves one element to `:150`, which is necessary. The set contents are the same.
5. **Gate 5: PENDING.** It needs the first nightly after the batch lands on `main`. Record the run ID in the issue.

**Extra check (not a gate): NEON.**
- I cross-compiled `audit` to an `aarch64-unknown-linux-gnu` object (release, fat LTO). In the scratch tree only, `capi` was built as rlib-only to skip the cdylib link.
- `probe_svf_simd4` is call-free, with no `bl`. Its loop is `.4s` `fmul`/`fadd`/`fsub`/`fcmgt`/`umax`/`bic` with `cmp x10,#0x200; b.ne`, and the rest plane is loaded on each frame (`ldr q18,[x8,x10]`).

**Other checks.**
- `nightly.yml`: the upload step keeps `if: always()`, so the report is uploaded when the job is red. `failure-notice` has `native-vectorization-report` in its `needs`, and its body has the result row. `check-ci-path-routing.py` does not examine this job.
- Standing rulings:
  - No merge-gating check moved. The non-blocking nightly job only becomes able to fail.
  - The rule is not weakened. D2's `call|callq` column stays, and the fix changes the probed subject in the same way as #372.
  - No retry, longer deadline or weaker predicate.
  - No queue, render-owned memory or public API is touched. The acked-batch question does not apply.
- `crates/lane` is not changed (non-goal respected). GitHub issue #1481 is OPEN, and its title matches the spec.

Evidence kept at `/tmp/claude-1002/v1481/`:
- report JSONs: `gate1-report.out`, `gate2-revert.out`, `gate3a.out`, `gate3b.out`, `d4.out`
- body excerpts: `svf-fixed.txt`, `svf-reverted.txt`, `svf-unarmed.txt`, `svf-neon-fixed.txt`
