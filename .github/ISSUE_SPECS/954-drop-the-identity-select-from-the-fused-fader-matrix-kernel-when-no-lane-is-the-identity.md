# Drop the identity select from the fused fader-matrix kernel when no lane is the identity

## Amendments (adversarial verification, 2026-09-27; these override any conflicting text below)

The verification (evidence: `docs/handoffs/gain-pan-2026-09-26/VERIFY-954.md`) found the x86
premise false and the browser case real.

- **A1. What the select costs.** The release `bench` binary's fused loop
  (`FaderMatrixBankProcessor::process`) has no `vmaskmovps`: its select's first arm is
  `load * g andnot mute`, not the loaded word, so LLVM emits two `vblendvps` feeding plain `vmovups`.
  Wasm emits two `v128.bitselect` per frame. Measured, one runtime, interleaved, digests identical:
  native fused gain/pan -170 ns of 10.41 us per block (builtins -150 ns, console -200 ns, the
  metered row unresolvable); **wasm under V8, in place: gain/pan -1.13 us of 21.14 us, builtins
  -1.07 us, console -0.98 us**. Today on wasm the product's fused gain/pan plan (21.14 us) is slower
  than the split plan (20.57 us); this issue brings it to 19.99 us. The case for the issue is the
  browser. Codegen evidence becomes: the select-free loops contain no `vblendvps` and no `bitselect`,
  and the select arm keeps them.
- **A2. Call sites.** The kernel is at `crates/lane/src/kernels/builtins.rs:346`. Its four callers:
  `crates/builtins/src/lib.rs:3833` and `:3845` (the banked product path), `:4179` (per-track
  `process_fader_matrix`) and `:3119` (`BuiltinChain`, tools only). All four take the new dispatch.
- **A3. Where the guard runs.** Evaluate `L::mask_any(matrix.coef.identity)` inside each function,
  after its settled check, which runs after the control drains
  (`crates/builtins-compiler/src/lib.rs:1070-1071`). Never cache it. The fused path has no ramp tail
  (a ramping block falls back to the split stages), so there is no tail site and no M4.
- **A4. Gate 1, two oracles.** Compare against the select form with an all-false mask and against
  `gain_mute_block` followed by `matrix2x2_block_without_identity`; cover mixed mutes and gains of
  0 and 1; NaN words compare as "both NaN"; run in dev and release. (Release saw 0 changed non-NaN
  words and 54 changed NaN payloads at `f32`, the #944 class statement.)
- **A5. Scenario test,** through `try_process_settled_with_matrix`, pinned on the unmodified base: a
  full bank with an exact `IDENTITY` lane and one frame with `l = -0.0, r = +0.0` (so M1 fails on the
  output words), an instant (0-sample) retarget to `IDENTITY`, and a partial bank. Add a dispatch
  witness for M2.
- **A6. Row gates.** Every `WORKLOADS` row uses concurrent delivery and makes zero fused calls, so
  "every console digest unchanged" witnesses nothing here. The row gates are #881's metered pair test
  and `composite_live_sequence_…`; M3 fails both (checked).
- **A7. Benchmark statement.** No native row can resolve the saving; say so. The wasm artifact is
  repinned at the batch boundary; the kernel count and render closure are unchanged (checked).

## Product outcome

#944 gave the settled, unpaired pan matrix a select-free arm when no lane of a bank is the exact identity, and the gain/pan row moved from 14.1 to 11.1 us per block (every digest unchanged). The #881 verification found that the default web boot does not take that path: between-render-calls delivery fuses each cohort's fader and matrix into one stage, `fader_matrix_block` (`crates/lane/src/kernels/builtins.rs`), which #944 left untouched as a non-goal. The product's fused stage therefore still evaluates the per-lane identity select every frame (on x86 folded into a `vmaskmovps` masked store, which #944 measured at about 10 extra cycles per frame on Zen 3; on wasm a `bitselect`). Give the fused kernel the same select-free arm.

## Smallest closable slice

Mirror #944 on the fused pair: a `fader_matrix_block_without_identity` kernel (the second arm verbatim, no `L::select`), chosen once per call when `!L::mask_any(identity)` over all lanes, padding included, at every settled call site of the fused pair; the existing kernel stays the oracle. Same class statement as #944 (every non-NaN word unchanged, a NaN stays NaN; LLVM may commute the final add), same gates adapted: dev and release differential against the oracle at `f32`, `Simd4` and `Simd8`; a scenario test pinned on base; every console digest unchanged; red mutations; x86 disassembly showing plain stores; wasm census via the build script's cargo line.

## Rows it can move

`sixty_four_track_console_metered` (#881) and any plan prepared with between-render-calls delivery. No digest may move.

## Dependencies

#944 (merged in the optimisation batch), #881.

Found by the #881 attempt 1 verification.

## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), 2026-09-27, branch `codex/954-fused-fader-matrix-without-select`
on `e0f25bb6` (the unmodified base of this attempt: the batch branch through #949 plus this brief).
Host AMD EPYC 7313P (Zen 3), rustc 1.97.1, node 22.23.2, `CARGO_INCREMENTAL=0`, the worktree's own
`target/` (deleted afterwards). `scripts/run-console-benchmark.sh` was not run.

Commits:

- `018a0008` test: the amendment-5 scenario gates, written and pinned on the unmodified base.
- `ab9bdbd9` the kernel, `MatrixStage::fused_settled_block`, all four call sites, the witness, gate 1.
- the evidence commit carrying this section and both `MUTATIONS.md` records.

### Design

- `lane::kernels::builtins::fader_matrix_block_without_identity` is `fader_matrix_block`'s second
  arm verbatim: two loads, `load * gain andnot mute` per plane, `ll*l + lr*r`, `rl*l + rr*r`, two
  stores, no `L::select` anywhere, and `debug_assert!(!L::mask_any(matrix.identity))`. Its doc
  gives the reason (amendment 1's blend/bitselect cost), the precondition and #944's class
  statement. `fader_matrix_block` is not edited and stays the oracle.
- `MatrixStage::fused_settled_block` is the one dispatch: `L::mask_any(self.coef.identity)` over
  all `L::WIDTH` lanes, padding included, once per call; the select form when true, otherwise the
  select-free kernel and a `#[cfg(test)]` increment of `FUSED_SELECT_FREE_BLOCKS` (a counter of its
  own, beside #944's `MATRIX_SELECT_FREE_BLOCKS`). It sits next to #944's `settled_block` inside
  the same `REALTIME_POLICY` region, takes the gains and mutes as arguments, and adds no field.
- Amendment 2: all four call sites take it -- `try_process_settled_with_matrix` at `Simd4` and
  `Simd8` (after its shape and `remaining_nonzero` checks), `FaderMuteRampBuiltins::process_fader_matrix`
  (after both `is_settled()`), and `BuiltinChain::process_dual_mono` (after the matrix
  `is_settled()`). Amendment 3: the mask is read there, after the settled check and so after the
  owner's control drain; it is never cached. The fused path has no ramp tail, so there is no tail
  site and no M4.

### Gates

| # | command | result |
| --- | --- | --- |
| 1 (A4) | `cargo test --locked -p lane --test fader_matrix`, dev and `--release` | green. The select-free kernel against oracle A (`fader_matrix_block`, all-false mask) and oracle B (`gain_mute_block` x2 then `matrix2x2_block_without_identity`), 5 families (4 hostile + overflow), 3 gain sets (mixed; unity and zero; up to 15.85), 3 mute sets (none, mixed, all), frames `[1, 3, 8, 9, 128]`, guard words, `f32`/`Simd4`/`Simd8`. NaN-payload differences `[oracle A, oracle B]` at `[f32, Simd4, Simd8]`: dev `[[0,0],[0,0],[0,0]]`, release `[[30,0],[456,0],[894,0]]`; 0 non-NaN differences and 0 NaN-vs-non-NaN in both profiles. Negative control (mixed identity mask) differs at every width |
| 2 (A5 witness) | `cargo test --locked -p builtins --lib fused_fader_matrix_takes`, dev and release | green. W8 and W4: 1 per fused call for a full non-identity bank; 0 for an identity member, a `W-1` bank, a 1-member bank; 0 after an instant (0-sample) retarget of lane 0 to `IDENTITY`, 1 per call after the instant retarget back; a matrix ramp and a fader ramp each decline the fused call and count 0; 1 once settled. Scalar: `process_dual_mono` 1 per block for a pan, 0 for `IDENTITY` and after an instant retarget to it; `process_fader_matrix` the same |
| 3 (A5 scenario) | `cargo test --locked -p builtins --test matrix`, dev and `--release` | on `e0f25bb6` (step 1, before any change) both profiles printed `fused_fader_matrix_shapes_render_the_base_bits` `46cc00962fac4916a0d5dfed9b197fb12c85082acd0cae806774876c3744abce` (368 fused blocks of 384) and `scalar_fused_fader_matrix_renders_the_base_bits` `c51310190d189b0432ae56f97e331ee79a9e2c1dd222b5cccc086b4abbcaaafe` (44 of 48); pinned in `018a0008`; identical after `ab9bdbd9` in dev and release. The banked gate drives `try_process_settled_with_matrix` at both widths with `{1, W-1, W}` members over a full non-identity bank, one exact `IDENTITY` member, an instant retarget of member 0 to `IDENTITY` and back, and smoothed matrix and fader retargets (split-stage fallback as the product does); every lane meets `l = -0.0, r = +0.0` at unity gain on lanes `0 mod 4`. The scalar gate covers the two `f32` sites |
| 4 (A6 rows) | `cargo test --locked -p builtins-compiler --features test-support --lib composite_live_sequence_matches_original_owners_and_discriminates_both_branches`; `cargo test --locked -p console-workload --lib the_metered_console_row_renders_the_console_bits_and_publishes_every_window`, dev and release | green; metered digest `fe5bed9b...` unchanged. M3 turns both red (below) |
| 5 | every console digest: `cargo test --locked -p console-workload`, dev and `--release` | green (lib 25, `chain_shape` and the other test binaries, all pins unmoved). Per amendment 6 these rows make zero fused calls, so only the metered row in gate 4 can see the change |
| 6 | mutations M1, M2, M3, M5 | all red, each on the gate the brief predicts; `crates/builtins/tests/MUTATIONS.md`, `crates/lane/tests/MUTATIONS.md` |
| 7 | `check-lane-policy.sh`, `check-builtins-policy.sh`, `check-realtime-policy.sh`, `check-builtins-fixtures.sh` | lane policy ok; builtins policy ok; `realtime policy: ok (56 marked regions in 16 files)`; `builtins fixtures: ok (50 files)` |
| 8 | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; `cargo test --locked` of `lane` (dev and `--release`), `builtins` (dev, `--release`, and `--features test-support`), `builtins-compiler --features test-support`, `console-workload` (dev and `--release`), `capi` | all green (builtins-compiler lib 58, capi 32 + 4) |

### Codegen, native (release `bench`, `objdump -d --no-show-raw-insn`)

`<FaderMatrixBankProcessor as GraphPreparedBuiltinBankProcessor>::process` now holds four fused
loops: the select form at `xmm` and `ymm` keeps its two `vblendvps` per frame, and the select-free
loops at `xmm` and `ymm` have none. The function has 4 `vblendvps` (all in the select loops), 0
`vmaskmovps`, and 2 `vtestps` (one guard per width). The select-free `f32x8` loop:

```text
vmulps (%rdi),%ymm10,%ymm6 ; vandps %ymm0,%ymm6,%ymm6       # l = load*gl andnot mute
vmulps (%rsi),%ymm11,%ymm7 ; vandps %ymm1,%ymm7,%ymm7       # r
vmulps %ymm6,%ymm2,%ymm8 ; vmulps %ymm7,%ymm3,%ymm9 ; vaddps %ymm9,%ymm8,%ymm8   # yl
vmulps %ymm6,%ymm4,%ymm6 ; vmulps %ymm7,%ymm5,%ymm7 ; vaddps %ymm7,%ymm6,%ymm6   # yr = rl*l + rr*r
vmovups %ymm8,(%rdi) ; vmovups %ymm6,(%rsi)
```

The select arm computes `yr` as `rr*r + rl*l` (`vaddps %ymm6,%ymm9,%ymm6`) where the select-free
loop computes `rl*l + rr*r`: the commuted add behind gate 1's release NaN-payload counts, as in
#944. `FaderMuteRampBuiltins::process_fader_matrix` (auto-vectorised `f32`) shows the same split:
its select loops blend with `and`/`or` (`vpand`/`vpor`), its select-free loops have no blend.

### Codegen, browser (A1, A7)

Built with `scripts/build-web-audioworklet.sh`'s own cargo line
(`RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo <both remaps>" cargo build --locked
--release --target wasm32-unknown-unknown -p host-web`) into one private target directory, at
`018a0008` (base) and at `ab9bdbd9`; `wasm-objdump -d` into `check-web-audioworklet-callgraph.py`.
The artifact was not repinned.

| | base | after |
| --- | --- | --- |
| artifact sha256 | `890705fa...` | `225812fe...` |
| `--callgraph miso_engine_web_v1_render` | closure 8, traps 5, owner `render_inner` | identical |
| `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` | 15 kernels, `f32x4_arith` 11,719 | 15 kernels, 11,751; roster lines identical |
| `meter_poll`, `command_submit` callgraphs | closure 9 / 39, traps 2 / 77 | identical |
| `BuiltinFaderBank::try_process_settled_with_matrix` loops | 2: `Simd4` 2 `v128.bitselect`, `Simd8` (2 x `f32x4`) 4 | 4: the same two select loops unchanged, plus `Simd4` and `Simd8` select-free loops with 0 `v128.bitselect` (`load, mul, and` x2, 4 `mul`, 2 `add`, 2 `store` per `f32x4`) |
| guard | -- | `Simd4`: one `v128.any_true` + `br_if`; `Simd8`: two `v128.any_true`, `i32.or`, `br_if` |

The base artifact's sha256 does not match the committed pin `8934cdd9...`: the pin is already stale
on this batch branch (earlier issues changed the artifact), and amendment 7 repins at the batch
boundary.

### Wasm A/B under V8 (throwaway, descriptive)

Harness kept out of the repository: `wasm-console-guest` built twice for `wasm32-unknown-unknown
+simd128` with two scratch exports (set a between-render-calls switch in `build_full`, as the
verification did; prepare with `SourceSignal::Local`), OFF = this tree with M2 (never select-free,
the base kernel choice), ON = this tree. Both modules in one node process, `taskset -c 14`,
Backend `Simd4`, 200 warm-up blocks each, 8 interleaved paired rounds x 400 blocks, per-round p50
ns/block; one invocation, no retry. Another agent was building on the host.

| row (wasm, V8) | delivery | OFF p50 | ON p50 | median paired diff | digests OFF = ON |
| --- | --- | ---: | ---: | ---: | --- |
| gain_pan_only | between render calls | 20,730 | 19,888 | **-816** (rounds 2-8: -762..-842; round 1 +13,807) | yes |
| gain_pan_only | concurrent (control) | 20,544 | 20,695 | +151 | yes |
| builtins_only | between render calls | 51,829 | 51,005 | **-825** | yes |
| builtins_only | concurrent (control) | 51,388 | 51,704 | +321 | yes |
| console | between render calls | 295,060 | 293,938 | **-1,666** | yes |
| console | concurrent (control) | 294,965 | 294,777 | -173 | yes |

The concurrent rows never enter the fused path, so they measure the two modules' layout bias
(+151 ns on gain/pan). Against that control the fused gain/pan plan gains about 1.0 us of 20.7 us
per 64-track block, consistent with amendment 1's -1.13 us; builtins about 1.1 us. The fused
gain/pan plan (19.89 us) is now faster than the split plan (20.54-20.70 us). Every digest was
identical ON and OFF under both deliveries, and each row's between-render-calls digest equals its
concurrent one.

### Benchmark statement (A7)

No native row can resolve the saving (the verification measured about 0.17 us of 10.4 us on the
native fused gain/pan plan and nothing resolvable on the 147 us metered row), so no native number
is claimed. The case is the browser: about 1 us per 64-track block on the fused plan under V8. The
AudioWorklet artifact changes and is repinned at the batch boundary; its kernel count (15) and
render closure (8 members, 5 traps) are unchanged.

### Deviations

- The amendment-5 scenario is two pinned tests: the banked one A5 names, and a second over the two
  scalar call sites, which no existing test pins by bits against M1.
- M5 (one call site bypassing the dispatch) was added to the briefed mutations; only the witness
  sees it, as intended.
- Gate 1's release NaN-payload counts are larger than the verification's (54 at `f32`) because its
  case set is larger (an overflow family and three gain x three mute sets) and now also counts
  `Simd4`/`Simd8`; every difference is a NaN-payload difference against oracle A, none against
  oracle B, so the class statement holds as written.
- The wasm A/B compares two modules instead of one toggle binary; the concurrent control row
  measures the resulting bias and is reported beside each result.
