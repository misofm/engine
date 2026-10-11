PASS

# #1490 attempt 1: adversarial verdict

Commit `d1b17d216` (parent `cca903769`), branch `codex/d15-batch-misc2`. I exported the commit with `git archive` to `/tmp/claude-1002/v1490/tree` and built it with `CARGO_TARGET_DIR=/tmp/claude-1002/v1490/target`. I did not change the worktree. Disk before the builds: 55 GB free.

Paths touched: the spec, `tools/audit/src/vectorization.rs` and `tools/audit/vectorization-allowlist.tsv`. All three are authorized. `crates/lane` has no change (D5). No workflow file has a change.

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **Spec `:132`, attempt record: "8 frames per iteration, 32 iterations" is false.**
   - The loop advances `%rcx` by 8 words (`addq $0x8`) and stops at 256 words (`cmpq $0x100`). One iteration is one frame of 8 lanes, and 32 iterations are the 32 frames of `PROBE_FRAMES`.
   - The gate and the excerpt are correct. Only the sentence is wrong.
   - Action for root: change it to "one 8-lane frame per iteration, 32 iterations".

## NIT

1. **`tools/audit/src/vectorization.rs:102` and `:157`, and spec `:26` and `:98`: "every non-stationary block" / "every live ramping block" says too much.**
   - In production, `svf_block::<L, UnarmedRest>` runs at `crates/parametric-eq/src/lib.rs:1663` only for a segment in which this section does not ramp and the section is not HPF/LPF. A ramping section runs `svf_block_ramped`, and HPF/LPF run `svf_block_ramped_with_dry_mask`.
   - Thus a ramping block whose live sections all ramp for the full block (for example, one band moves and HPF/LPF are dead) does not run this loop.
   - The part of the test-value sentence that carries the claim is true: the row goes red on the unarmed loop, and no other probe instantiates `svf_block_form::<L, false>`. The "every" clause is a description, and it comes from root's spec.
   - Better words: "the loop that the settled, non-HPF/LPF sections of a ramping block run when no lane can arm the joint flush".

## The checks the task asked for

- **The probe is the production monomorphisation.**
  - Production path: `render` / `render_mono` pass `UnarmedRest` (`lib.rs:3191`, `:3261`). Then `process_channels(_mono)` (non-stationary branch) calls `process_section::<UnarmedRest>`, which calls `svf_block::<L, R>` (`:1663`). The bank's `L` is `lane::Native`, which is `Simd8` under `avx2` (`crates/lane/src/lib.rs:600`).
  - The probe is `svf_block::<lane::Simd8, UnarmedRest>`, which dispatches to `svf_block_form::<Simd8, false>` (`kernels.rs:285-290`). It is the same generic body.
  - I disassembled the release `audit`. It links `<parametric_eq::PreparedParametricEq<f32x8, 8>>::process_bank` and `process_bank_mono`. These contain inlined unarmed loops of the same per-frame form as the probe:
    - one `vmovups` io load and no threshold load;
    - `vsubps`, 7 `vmulps` and 10 `vaddps`;
    - two `vandps`/`vcmplt_oqps`/`vandnps` per-word flushes;
    - no call and no `vfmadd*`.
  - The differences are only register allocation (the coefficients are memory operands) and a runtime trip count (`cmpq %r10, %rsi`).
- **`black_box` does not let the compiler remove the real shape.**
  - The probe body loads all six coefficient words from the opaque pointer (`(%rcx)` to `0xa0(%rcx)`), flips the `c1` sign with `vxorps`, and loads and stores the state through the opaque `&mut`.
  - `m0`/`m1` (0.0 in `execute_probes`) are not folded: all three output `vmulps` are present.
  - The loop is not unrolled. The static length only fixes the trip count, as in the armed probe. `UnarmedRest` slices no plane, so a dynamic length would add no check (`chunks_exact_mut` has no panic path). The production loop above confirms that it has no call.
- **The allowlist row copies the columns.**
  - The backend, the required groups, the forbidden-scalar groups and the forbidden-call groups of `recursive-svf-unarmed` are byte-identical to those of `recursive-svf` (`awk` compare). The row has 6 fields, and the file ends in a newline.
- **The Simd4 twin compiles for aarch64.**
  - `cargo check --locked -p audit --target aarch64-unknown-linux-gnu` exits 0. `rustc --print cfg` for that target sets `target_feature="neon"`.
  - Negative control: I changed the twin's `UnarmedRest` argument to `1u8`, and the check failed with E0308 at `vectorization.rs:171`. Thus the check type-checks the twin. I then restored the file and confirmed it with `diff`.
  - I did not emulate AArch64.
- **The report is still not merge-gating, and nothing moved.** The report runs only in `nightly.yml` (`native-vectorization-report`, "evidence, not a merge gate" since #144). It fails its job and then `failure-notice` (#1481). It is not in `qualification.yml`, before or after this commit. The diff touches no workflow. The audit unit tests stay in `qualification.yml`'s `audit-native` job, and that job is unchanged. No check moved.

## Mutation run (redone)

The mutation is the recorded one. After `y.store(frame)` in `svf_block_form`'s frame loop, I added `if !ARMED { mutation_1490_hook(); }`, with `#[inline(never)] fn mutation_1490_hook() { core::hint::black_box(()); }`.

- **With the slice:** exit 1, `"status":"fail"`, `"kernel_rules":4`, `"failures":["recursive-svf-unarmed / probe_svf_unarmed_simd8: forbidden call 'call|callq' is present"]`. `probe_svf_simd8` is not named.
- **The same mutated kernel with the pre-slice audit tool** (`vectorization.rs` and the allowlist from `cca903769`): exit 0, `"status":"pass"`, `"kernel_rules":3`, `"failures":[]`.
  - In that binary, `call mutation_1490_hook` is present in the production `PreparedParametricEq<f32x8, 8>::process_bank` (2 sites) and `process_bank_mono` (1 site). Thus the production regression was real, and the old report did not see it.
- **Revert:** the sources are byte-identical to `d1b17d216` (`diff -r`). The report gives `"status":"pass"`, `"kernel_rules":4`, and the same `artifact_sha256` as gate 1.

## Test value

- **New row `recursive-svf-unarmed / probe_svf_unarmed_simd8` (and the probe that feeds it).** It goes red when the unarmed SVF frame loop gets a call, a scalar `*ss` fallback or a fused multiply-add in the AVX2 release build. Example: a `Simd8` regression in the per-word `flush` that only `svf_step_when(false, ...)` uses in this kernel. No existing probe instantiates `svf_block_form::<L, false>`: the pre-slice report stayed green with a call in the production `process_bank` loop.
- **`probe_svf_unarmed_simd4`** has no allowlist row (D4 adds no `aarch64-neon` rows), so it gates nothing yet. It only has to compile, which gate 4 shows.

## Gates run

1. `bash scripts/run-native-vectorization-report.sh`: `"status":"pass"`, `"kernel_rules":4`, `"failures":[]`.
   - The new probe's loop is 8-lane `ymm` (`vsubps`/`vmulps`/`vaddps`, flush `vandps`/`vcmplt_oqps`/`vandnps`) with 0 `call`, 0 `vfmadd*` and 0 scalar `mulss`/`addss`/`subss`/`divss`.
   - The only `*ss` are 3 `vbroadcastss` before the loop. The excerpt in the attempt record agrees with my disassembly.
2. Mutation: see above. Red only on the new row, green on the pre-slice tool, green on revert.
3. `bash scripts/test-native-vectorization-report.sh <target>/release/audit`: `native vectorization red mutations: ok`.
   - `cargo test --locked -p audit vectorization`: 13 passed.
   - `cargo test --locked -p audit` (the full crate): 33 passed.
   - `cargo clippy --locked -p audit --all-targets -- -D warnings`: exit 0.
   - `cargo fmt --all -- --check`: exit 0.
   - `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
4. `cargo check --locked -p audit --target aarch64-unknown-linux-gnu`: exit 0, plus the negative control above.

Gates 1 and 3 use the exact invocations from `.github/workflows/nightly.yml:68-70`.

Evidence kept in `/tmp/claude-1002/v1490/`:
- `unarmed.txt` (the probe disassembly);
- `prod-unarmed-loop.txt` (the inlined production loop);
- the `gate1.log`, `gate2-slice.log`, `gate2-pre.log` and `revert.log` logs;
- the `report-out`, `mut-out`, `mut-pre-out` and `revert-out` report JSONs.
