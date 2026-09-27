# Run every pass of an admitted stationary EQ plan select-free


EQ optimisation, slice 2 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, sections 1, 2 and 5 (EQ-2). A tested prototype
of EQ-1 plus this change is `docs/handoffs/effects-2026-09-27/eq-variant-no-padding-select-free.patch`.

## Product outcome

Every stationary pass runs `svf_cascade_interleaved_with_dry_masks` (`crates/parametric-eq/src/lib.rs:1780`
in `interleave`, and its mono twin), and builds its masks per pass per block through `dry_mask`
(`:1136`), which extracts each lane's six coefficient words through the stack. A general band is
never dry, so for any session with two live general bands every select is a no-op. They still cost
10 % of the pass natively (`vblendvps`) and 13 % in the browser, where V8 rebuilds each mask inside
the frame loop (`vbroadcastss`, three `vinsertps`, `vcmpps`, then a three-instruction bitselect: 32
instructions per frame for four no-op selects).

On an **admitted** block (the list `cascade_sections` returned is shorter than six), every select is
a no-op, including a dedicated cut's dry lanes. Run those passes select-free and build no mask. A
refused block (all six sections) keeps today's masked kernel.

## Why this is exact

Admission (leg (a), `block_admits_elision`, `:997`) means both planes are finite, within
`BLOCK_LIMIT`, and free of `-0.0`. A dry lane is a dedicated cut holding the exact identity words
(`c1 = a2 = a3 = m1 = m2 = +0.0`, `m0 = 1.0`) with no ramp. For any finite state its step gives
`v1 = ic1 + (+-0)` and `v2 = ic2 + (+-0)`, and its wet output is
`(+0 * v2) + ((+0 * v1) + 1.0 * x)`. For a finite `x != -0.0` that is `x`, bit for bit: a nonzero
`x` absorbs the signed zeros, and `x = +0.0` gives `+0.0` for either sign of the zero terms. The
state update never reads the mask. So `select(dry, x, wet)` returns `wet`'s bits on every lane.

The prototype passed `cargo test -p parametric-eq` in dev and release (mixed cuts, signed-zero
refusals, disabled-cut oracles, mono collapse, partition invariance); the only failures were EQ-1's
kept-count pins.

## Lessons carried from #944

Dev and release; NaN words compared as "both NaN, or equal bits"; the scenario test written and
pinned on the base first; every mutation recorded red in `crates/parametric-eq/tests/MUTATIONS.md`.

## Invariants

- **Class A.** Every rendered word and every integrator word is unchanged.
- A refused or all-live block (the full six-section list) keeps the masked kernel: its input may
  carry `-0.0` or a non-finite word, and there the select is not a no-op.
- The non-stationary path (`process_block`/`process_section`) is untouched.
- Allocation-free; no new field; kernels stay `#[inline(always)]` (wasm `KERNEL_ROSTER` rule 1).

## Interface contract

1. `interleave` and `interleave_mono`: `let admitted = length < EQ_SECTION_COUNT;`. When
   `admitted`, every pair calls `svf_cascade_interleaved::<L, S, 2>` and the depth-1 section (EQ-1)
   calls `svf_cascade_interleaved::<L, S, 1>`; no `dry_mask` is evaluated. Otherwise today's code.
2. `cascade_sections`/`cascade_sections_mono` are unchanged: "shorter than six" is exactly
   "the gate admitted and at least one section was elided".

## Smallest closable slice

Authorized paths: `crates/parametric-eq/src/lib.rs` (`interleave`, `interleave_mono`, their docs,
one `#[cfg(test)]` counter of masked passes); `crates/parametric-eq/tests/bank.rs` (gate 1);
`crates/parametric-eq/tests/MUTATIONS.md`; the ruling's EQ inventory paragraph on selects
(`docs/rulings/effect-floor-accounting.md`, "EQ inventory"); this spec.

Steps: (1) gate 1 on the base, digest pinned; (2) contract 1; (3) gates.

## Non-goals

The skewed kernel (EQ-3), the refused path's masks, `dry_mask` itself (it stays for refused
blocks and ramps), any `crates/lane` change.

## Objective gates

1. **Scenario (public API, pinned on base).** `admitted_blocks_render_the_base_bits_without_selects`
   in `tests/bank.rs`, scalar and `native_bank()`: two live general bands (a depth-2 pass), plus the
   HPF live on half the lanes (dry lanes on the others) after an enable-then-disable of those
   lanes' HPF through prepared targets, so the dry lanes hold a frozen non-zero state. 32 blocks of
   hostile input (subnormals, `+0.0`, `2^-24..2^25`), some blocks with `-0.0` or a non-finite word
   (refused). SHA-256 of every output word and every lane snapshot, pinned on base.
2. **Counter.** The `#[cfg(test)]` masked-pass counter reads 0 across gate 1's admitted blocks and
   is non-zero on its refused blocks.
3. `cargo test -p parametric-eq`, dev and release; `chain_shape` and the before/after digests of
   every `WORKLOADS` row, as in EQ-1 gate 3.
4. **Mutations**, each alone, red:
   - M1: `admitted = true` always: gate 1 (a refused block with `-0.0` on a dry lane renders `+0.0`).
   - M2: `admitted = false` always: gate 2.
   - M3: skip the depth-1 arm's `admitted` branch only: gate 2.
5. **Browser artifact.** As EQ-1 gate 5. Record V8's loop for the admitted pass (`node --no-liftoff
   --print-wasm-code`): no `vinsertps`/`vcmpps eq` mask rebuild in it.
6. Toolchain and policy scripts as EQ-1 gate 7.

## Console benchmark rows

The standing fixtures have one live band, so after EQ-1 their pass is already select-free: expect no
row to move more than noise. The saving shows on sessions with two or more live bands (replica:
native W8 5,878 -> 5,301 cycles per two-section pass, V8 7,062 -> 6,125). A multi-band row is a
separate tooling issue.

## Dependencies

EQ-1 (the depth-1 arm this issue makes select-free on admitted blocks).

## Standing rules for the implementer

As EQ-1.

## What the implementer will hit

- **"Admitted" is a list length, not a flag.** A six-entry list is either refused or all-live, and
  neither scanned the input for `-0.0`; both must keep the masks.
- **`dry_mask` looks cheap in the source and is not**: per dedicated section it extracts `W x 6`
  lane words through a stack array, and on this path it runs per pass per block. Do not call it on
  the admitted path at all.
- **The mono body** (`interleave_mono`) takes the same rule; the collapsed test suite covers it.


## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, branch `codex/977-eq-elision-and-passes`, on #980 (`750fff95`;
#976 is in the base `1d8c4851`). The verification amendments on the GitHub issue are applied:
leg (c) also requires finite integrators (amendment 1, with its scenario in gate 1); the counter is
`cfg(any(test, feature = "test-support"))` with `test_only_*` accessors and the gate runs under
`--features test-support` (amendment 2). Host: AMD EPYC 7313P (Zen 3), `rustc 1.97.1`, `x86-64-v3`,
every build `CARGO_INCREMENTAL=0`.

### The change

- `interleave` and `interleave_mono`: `let admitted = length < EQ_SECTION_COUNT;`. When admitted,
  every pair runs `svf_cascade_interleaved::<L, S, 2>` and the depth-one tail
  `svf_cascade_interleaved::<L, S, 1>`; no `dry_mask` is built. A six-entry list (refused or all
  live) runs today's masked pairs. The tail's unadmitted arm keeps #976's mask-if-dry rule, written
  as `(!admitted).then(|| masks)` so the select-free call is not duplicated; a tail only exists on
  an odd, hence admitted, list, so that arm is never reached.
- Leg (c) (amendment 1): `section_state_has_no_negative_zero` became
  `section_state_is_finite_without_negative_zero` (every lane of `ic1` and `ic2`: not `-0.0`, and
  magnitude bits below `0x7f80_0000`), in `cascade_sections` and `cascade_sections_mono`.
- Docs: `cascade_sections` gains "Why an admitted plan runs select-free" (the dry lane's input has
  no `-0.0`; its state is finite by leg (c); for the HPF `v3` cannot overflow under leg (a); for the
  LPF an overflow makes both kernels fail §4.4 on the same lane; the state update never reads the
  mask; why the finiteness term is load-bearing). `interleave` and `interleave_mono` state the rule.
- Counter: `MASKED_PASSES` beside the #976 counter, incremented in every masked arm;
  `test_only_reset_masked_passes`/`test_only_masked_passes` under `feature = "test-support"`.
- Ruling, "EQ inventory": every pass of an admitted plan is select-free; the admitted floor is
  `24 * active + 3` (27, 51, 75, 99, 123 for 1-5 active), refused or all-live 153; a refusal now
  includes a non-finite state in a live section. `EQ_LANE_OPS` stays 27.

### Gate 1: `admitted_blocks_render_the_base_bits_without_selects` (`tests/bank.rs`)

Eight tracks, 32 hostile blocks (`+0.0` 1/16, subnormals of either sign 1/16, normals
`2^-24..2^26`; every fifth block 37 frames; block `8k + 3` one `-0.0` on track `2k`, plane `k mod 2`;
block `8k + 6` a non-finite word on tracks 0 and 4, plane `k mod 2`). Legs: scalar, `native_bank()`
dual, and the same bank through `process_bank_mono`. Shapes:

| shape | what it carries |
|---|---|
| `DryHpfInPair` | a bell and a high shelf (`set_initial`), the HPF switched on everywhere at block 0 and off at block 5 through prepared targets on the left of even tracks and the right of odd tracks: the HPF's pair has dry lanes with a frozen non-zero state |
| `DryLpfInTail` | the same with the LPF on the opposite lanes: the depth-one tail has dry lanes |
| `DryCutsTogether` | both cuts on everywhere, off on both channels of even tracks: a pair of two dry cuts |
| `PoisonedDryHpf` | amendment 1's case: HPF live on the right, a 10 Hz LPF on the left, `ic2 = -f32::MAX` restored into track 0's dry left HPF before blocks 0 and 16, and `1.1e31` on that lane at blocks 2 and 18. The test asserts the spike leaves both integrators `NaN` with no fault reported; later blocks are refused on that `NaN` until the block-6/22 fault resets the plane |

Every output word, report and state payload after every block, one SHA-256 per leg. Pinned on the
unmodified base (`750fff95`), identical in dev and release: scalar
`9316456b588c8e1d52e7c2a0070df003bb01de5b9ba890c91c802eff1853ce2e`, bank
`d4a1dc9db58fa425e74034f8c8fedf77fc5a6e1e3a3596053fe07ca1318e71bf`, bank-mono
`f442a0d3ea61ce2ca6af9d502b1fb0958ef5632a51554e0c95c350de9db5c162`. After the change: the same three
in dev and release, with and without `test-support`. Without the amendment (M4) the dual legs move.

### Gate 2: the counter

Under `--features test-support`, over the two switching shapes (whose admission the input alone
decides: stationary, and leg (a) on the unit's planes), masked passes `(on admitted, on refused)`:
scalar `(0, 36)` and `(0, 36)`, bank `(0, 24)` and `(0, 24)`, bank-mono `(0, 12)` and `(0, 12)`. The
refused counts are three masked pairs per refused block (12, 8 and 4 refused blocks). In-crate,
`elision::an_admitted_plan_runs_every_pass_select_free` asserts zero masked passes on admitted
blocks (dual and mono) with the dry lanes in two pairs, in the tail, in a pair ahead of a
general-band tail, and nowhere, at `f32`, `Simd4` and `Simd8`, the per-section path's bits, and
three masked passes on a `-0.0` block (six with its mono body). `elision::a_non_finite_state_in_a_live_section_refuses_elision`
covers the amended leg (NaN, `-NaN`, `+-inf` in either integrator of a dry lane refuse, dual and
mono; a finite `-f32::MAX` is admitted).

### Gate 3: suite and rows

`cargo test -p parametric-eq`: 106 passed, 3 ignored in dev and release, with and without
`test-support`. `chain_shape` (release): 23 passed. Row digests (scratch harness, clean
`git archive` copies, as in #980's record): all 15 rows x `Scalar`/`Simd4`/`Simd8`, one band and
the scratch-only two-band knob (90 lines), identical to the base; the wasm guest's 30 digests
identical to the base's and to the native ones. The two-band rows are the ones with a depth-two
admitted pass.

### Gate 4: mutations

Recorded in `crates/parametric-eq/tests/MUTATIONS.md` ("Issue #977"): M1, M1m, M2, M2m, M3, M3m and
M4 (the amendment dropped) all red. M2/M3 move no bit and are caught by the counter; M1 and M4 move
gate 1's digests (M1 scalar `cbbb3c83…`, bank `5f519f25…`; M1m bank-mono `ded71138…`; M4 scalar
`f14e7956…`, bank `c79e11df…`).

### Gate 5: browser artifact

Build script's cargo line, not repinned (artifact `7b975112…`). `--callgraph
miso_engine_web_v1_render` closure=8 traps=5, one trap owner; `--kernel-shape --kernel-pattern
'4wide6f32x[48]' --kernel-min 11` ok, kernels=14; `meter_poll` and `command_submit
--allocation-only` ok. The only lines that differ from the base: `parametric-eq f32x4 dual` vector
312 -> 384 and `collapsed` 156 -> 192 (scalar 0; one arithmetic-carrying kernel each), which is one
select-free depth-2 instantiation (2 sections x 2 streams x 18 counted ops = 72; mono 36).

V8 (`node --no-liftoff --print-wasm-code`, the console guest on the scratch two-band `eq_only`
row): in `PreparedParametricEq<f32x4, 4>::process_bank` the admitted depth-2 pass is a new
161-instruction loop with `vaddps` 36, `vmulps` 28, `vsubps` 8, and 8 `vcmpps`, all `(lt)` (the
flushes); no `vinsertps`, no `vbroadcastss`, no `vcmpps (eq)`, no `vpandn`/`vpand`/`vpor`
bitselect. The masked depth-2 loop (208 instructions: `vinsertps` 12, `vbroadcastss` 4, 4 `(eq)`
compares, 4 bitselects) remains, for refused and all-live blocks. It opens:

```text
4608 vmovdqu xmm0,[r12+rbx*1]
460e vsubps xmm11,xmm0,xmm14
4613 vmulps xmm8,xmm2,xmm13
4618 vmulps xmm2,xmm6,xmm11
461d vsubps xmm2,xmm2,xmm8
...
```

### Gate 6: toolchain

fmt, clippy (`--workspace --all-targets --all-features -D warnings`), doc (`-D warnings`),
`-p lane` dev and release (68), `-p effect-runtime` (86), `-p console-workload` (39),
`-p builtins-compiler --features test-support` (79), `-p wasm-gates` (9), `-p bench floor` (9),
lane policy, realtime policy (57 regions), EQ render contract, console benchmark validators: green.

### Descriptive A/B (not a gate)

Clean scratch builds of #980 (`750fff95`) and this change; one hold of `flock … timing.lock`,
`taskset -c 31`, load average 4.7-5.7; native harness alternated three times, both guests in one
Node process. Median / minimum of the per-round p50s, us:

| row | native before | native after | wasm before | wasm after |
|---|---:|---:|---:|---:|
| `eq_only` minus `builtins_only` | 8.23 / 8.53 | 8.50 / 8.86 | 19.40 / 19.46 | 19.58 / 19.51 |
| two-band `eq_only` minus `builtins_only` | 14.28 / 14.34 | 13.88 / 13.75 | 41.69 / 41.67 | 33.30 / 33.14 |
| two-band `console_mono` minus one-band | 1.53 / 1.29 | 1.48 / 1.76 | 6.66 / 6.61 | 3.93 / 4.12 |

The standing rows do not move beyond noise (their one live band was already select-free after
#976). The two-band row gains about 0.4-0.6 us natively and 8.4 us in wasm (the brief's replica
projected about 0.6 and 4.0).

### Deviations and notes for the verifier

1. **M1's red.** The brief expected a `-0.0` on a dry lane of a refused block to render `+0.0`.
   On a refused block the full cascade also executes the dead general bands, which rewrite `-0.0`
   to `+0.0` in both kernels, so gate 1 cannot show it. M1 is red on gate 1 through the poisoned dry
   lane, on gate 2, and in-crate; M1m additionally on the existing all-high-pass signed-zero oracle.
2. **The tail's masked arm** is kept (unreachable) to follow "otherwise today's code"; restructured
   so the select-free call exists once (a first version with a duplicated call added a second
   depth-1 instantiation: dual 420, collapsed 210).
3. **Leg (c)'s helper was renamed** (`section_state_is_finite_without_negative_zero`), and its lane
   helper replaced, per amendment 1, in `cascade_sections`/`cascade_sections_mono` outside the
   brief's original function list.
4. **Ruling:** besides the select rows, the EQ inventory's gate cost line now says four integer
   operations per lane-sample (#980's form) instead of "five integer comparisons".
5. The in-crate tests `an_admitted_plan_runs_every_pass_select_free` and
   `a_non_finite_state_in_a_live_section_refuses_elision` are additions beyond the brief's gates.
