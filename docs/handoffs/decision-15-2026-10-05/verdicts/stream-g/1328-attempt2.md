FAIL

# #1328 attempt 2: adversarial verdict

- **Reviewed:** `git diff 3b0fa85ef 90444c537` on `codex/d15-stream-g-1328`: 691f4c1be (A1),
  917d06da2, 902fc972a (A6) and 90444c537. The commit was exported with `git archive` to
  `/tmp/claude-1002/v1328/tree`, and 3b0fa85ef to `.../base`. Nothing was built or edited in
  `/home/bl/misofm/wt-d15-g-1328`.
- **Host:** AMD EPYC 7313P, Node v22.23.2 (V8 12.4.254.21-node.56).
- **Why it fails:** one MAJOR finding (M1). The class-B acceptance that lets this slice move bits
  rests on a change bound, and that bound is false:
  - A4 states it as `< 2.2e-13` (-253 dBFS).
  - This attempt's record states it as `1.5e-12` (-236 dBFS).
  - The measured worst change is `4.94e-10` (**-186.1 dBFS**).
  - The mechanism is a level-dependent dead zone. Neither A4 nor the implementation note
    (`dsp-research/filters.md`) names it.
- **What passes:** the code, A1, A2, A3, A5 and every gate. The fix for M1 needs no code: the root
  re-affirms A4 on the corrected text below, and then a doc-only change corrects the record.

## MAJOR

**M1. The A4 change bound and the Listening line are false. Bits move by up to 4.9e-10 (-186 dBFS),
through a dead zone that nothing documents.**

Evidence: my own harness on the real kernels. Both binaries were built from the tree. The
per-word build differs only in `flush_pair`'s body, which becomes `(andnot(|n1|<FLUSH_EPS),
andnot(|n2|<FLUSH_EPS))`. The left outputs were compared bit for bit.

- **Configurations:** 12 single-section EQs (`ParametricEqFactory`) and 6 `BuiltinChain`s.
- **Signals:** 5, namely DC, sines at 10 Hz, 100 Hz and 1 kHz, and LCG noise. Each runs for 1 s at
  the rate, then 1.2 M silent samples.
- **Levels:** 51, from 0 to -320 dBFS, with a 1 dB grid from -201 to -230 dBFS.
- **Runs:** 4,590, plus a 0.05 dB sweep from -209 to -212 dBFS for both 10 Hz +24 dB shelves.

Results:

| input level (peak) | worst change, absolute | where | worst change, relative to input peak |
|---|---|---|---|
| 0 to -140 dBFS | `3.1e-13` (-250.2 dBFS) | high shelf 10 Hz +24 dB, mostly in tails | -114 dB or less |
| -160 dBFS | `4.8e-13` (-246.4) | bell 10 Hz Q 0.1 +24 dB, 10 Hz sine | -86 dB |
| -180 / -200 dBFS | `4.4e-12` / `3.9e-11` | the same | -47 / -8 dB |
| **-210.35 dBFS** | **`4.94e-10` (-186.1 dBFS)** | low shelf 10 Hz +24 dB S 1, 96 kHz, 5 Hz square, while input is live | **+24.2 dB** |
| -211 to -320 dBFS | falls with the input level | low shelf, then high shelf | up to **+24.5 dB** (the section's whole response) |

Other measured worst cases:

- Gate-2 shelf (S 0.1), DC at -210.35 dBFS: `4.49e-10` (-187.0 dBFS).
- Bell 10 Hz Q 0.1 +24 dB, 10 Hz sine: `2.23e-10` (-193.0 dBFS).
- Builtin HPF 10 Hz at 96 kHz: `1.7e-11` (-215.2 dBFS).
- EQ HP/LP/notch 10 Hz: about `1.5e-11`. The change is about equal to the input: the high-pass
  passes the DC that it should block, and the low-pass blocks the DC that it should pass.

**Mechanism (analytic).** From rest, one step writes `n1 = 2*a2*x` and `n2 = 2*a3*x`. If every
input sample satisfies `|x| < L* = REST_EPS / (2*max(a2, a3))`, the joint rule zeroes both words on
every sample. The section then never leaves rest, and its output is only its direct term
`(m0 + m1*a2 + m2*a3)*x`. The per-word law instead runs the full filter. The change is therefore
`(h - g_dead*delta) * x`, and it is bounded by `||h - g_dead*delta||_1 * L*`.

- `L*` is largest at 10 Hz and 96 kHz.
  - For the +24 dB low shelf, `g` is divided by `sqrt(A)`, which gives `a2 = 1.638e-4` and
    `L* = 3.05e-11` (-210.3 dBFS). The measured peak sits at -210.35 dBFS.
  - For the builtin HPF/LPF at 10 Hz, `L* = 1.5e-11` (-216.3 dBFS).
  - For LR4 at 80 Hz (not measured), `L*` is about `1.9e-12` (-234 dBFS). The band makeup gains
    of ±24 dB scale this to about -210 dBFS, which is below the EQ case.
- `||h - g||_1` is about 16.3 for the shelf (`A^2 - 1 = 14.85` plus overshoot). The bound is
  therefore about `5.0e-10` (-186 dBFS), which agrees with the measurement.
- The per-word law had the same kind of dead zone, but at about -330 dBFS. The joint flush raises
  it by about 120 dB.

**Where bits move.** "Below about -160 dBFS input" (A4) is not a real onset:

- A signal that starts from zero moves bits on its first samples at any level, for example a
  10 Hz sine at -160 dBFS from sample 2.
- In DC-blocking builtin chains, input-time moves happen at 0 dBFS. These changes are at most
  `3.8e-15`.
- Noise onsets depend on the generator. My generator moves bits at -180 dBFS; attempt 2's moved
  nothing before -220 dBFS.
- In tails, the first moved sample can be loud: -165 dBFS, in the tail of a -170 dBFS 10 Hz sine
  through the gate-2 shelf.
- The quantity to hold is the change size, not the onset.

**What is wrong in the record:**

- Spec :447-460: the attempt-2 harness used white noise only. Its bound (`1.5e-12`, -236 dBFS) and
  its proposed Listening line are 50 dB short.
- A4 (:129-137) is 66 dB short.
- The DSP-evidence Listening line (:184-185, "every moved sample is below -204 dBFS") was never
  restated as A4 required. It is false.
- `dsp-research/filters.md` ("Adopted law" and "Why `1e-14`", :41-43) states no dead zone.
  AGENTS.md requires the implementation note to state its numerical limits.

**Correctness beyond class B?** Not audibly:

- -186 dBFS is about one LSB of 32-bit integer PCM (2^-31 is -186.6 dBFS).
- It is about 48 dB below a 24-bit LSB.
- A path to a 24-bit noise floor needs more than 50 dB of gain after the section, applied to a
  signal that is itself at about -210 dBFS.

It is still a qualitative change that the class-B wording hides: below `L*` a section stops
filtering. A +24 dB band gives no boost, and the HPF passes DC up to about -216 dBFS. The owner
should accept that by name.

**Fix:**

1. The root records A4 restated (text below) and re-affirms it.
2. A doc-only attempt 3 then:
   - replaces the A4 evidence paragraph (:442-460) and the open item (:501);
   - makes the Listening line (:184-185) the restated one;
   - adds the dead zone to `dsp-research/filters.md` under the numerical limit.

**Recommended A4 text (for the root to re-affirm):**

> **A4 (restated).** The joint flush moves output bits by a bounded amount, at any input level, not
> only in tails.
>
> **Mechanism.** From rest, a step writes `n1 = 2·a2·x` and `n2 = 2·a3·x`. While every input
> sample satisfies `|x| < L* = REST_EPS/(2·max(a2, a3))`, the section stays at rest and outputs
> only its direct term `(m0 + m1·a2 + m2·a3)·x`, where the per-word law runs the full filter. The
> change at the section's output is at most `‖h − g_direct·δ‖₁·L*`.
>
> **Domain.** `L*` is at most `3.05e-11` (-210.3 dBFS), for the EQ low shelf at 10 Hz, +24 dB and
> 96 kHz. It is `1.5e-11` (-216 dBFS) for a 10 Hz builtin HPF/LPF and about `1.9e-12` for LR4 at
> 80 Hz.
>
> **Bound.** The worst change is about `5.0e-10` (-186 dBFS), for that shelf. The measured worst is
> `4.94e-10` (-186.1 dBFS), with a 5 Hz square wave at -210.35 dBFS. For input at or above -140 dBFS,
> every change measured is below `3.2e-13` (-250 dBFS).
>
> **Relative to the input.** A section whose input stays below `L*` loses its whole response: up
> to +24.5 dB for ±24 dB bands, and the HPF passes DC below about -216 dBFS. The per-word law
> already did this below about -330 dBFS.
>
> **Accepted under D15-4(a).** The worst change is about one 32-bit-integer LSB and about 48 dB
> below a 24-bit LSB, and the slice fixes two never-resting states. The listening line reads
> "every change is below -186 dBFS at the section's output; no listening run". The PR evidence
> reproduces the shelf's dead-zone peak and attempt 2's builtin chain case.

## MINOR

**m1. The spill gate's mutation record is stale, and its "one-token" red arm has been green since
#999.** I bisected it. With today's gate script, the one-token tail edit is:

- red at `6f4c0379e` (#1009, `[rbp-0xc0]`, the recorded listing);
- red at `d09d50248`, the first parent of the #999 merge;
- **green at `27cf24132`**, the #999 merge ("fold the 4.4 boundary scan into the depth-one tail").
  That merge replaced the tail with the bounded-verdict kernel (84 → 112 instructions);
- green at `a9414c0c6`, the first `main` commit that carries #1009, so it has never been red on
  `main`;
- green at `0e3e21b68` and at this tree.

The gate has not lost discrimination:

- The rule is unchanged.
- #977 attempt 1 (rebuilt, module `0db9b2f5`) is still red, at `[rbp-0xa0]`.
- #1328 attempt 1's encoding is red on today's code. With A3 reverted, the dual tail fails at
  `[rbp-0xc8]` (a general-purpose slot) and the masked mono pair at `[rbp-0x220]`.

The protocol only requires saying so, which the implementer did. But two records still name the
one-token edit as a red arm:

- `tools/wasm-gates/MUTATIONS.md:221-270`, which also still says the gate holds three select-free
  rows;
- the gate script's re-pin paragraph (:124-126).

**Fix:** name the #1328 attempt-1 arm as the current witness, and record the one-token arm as green
since `27cf24132`. `MUTATIONS.md` is outside the authorized paths, so the root must authorize it.

**m2. The `dry` copy in `desymmetrize` is untested, and so is the existing `identity` copy.**

- Dropping `self.right.dry = self.left.dry` (lib.rs:3090) turns no `parametric-eq` test red. I
  verified this.
- Dropping `self.right.identity = …` (:3089) also turns none red. That gap predates this slice.
- The scenario: a both-channel ramp ends while the bank is collapsed, then the bank desymmetrizes.
- The consequences differ:
  - A stale `dry` can only miss a dry lane, so at most a `-0.0` → `+0.0` bit changes on a
    dedicated cut at the identity.
  - A stale `identity` can elide a live section in release builds.
- The debug assertion `identity_flags_agree` would catch both, but no test reaches that path.

The gap is documented, and it is the same kind as the existing one, so this is not MAJOR. Under
the no-shortcuts principle, one test (collapse → ramp ends in mono → desymmetrize → stationary
dual block, compared with an uncollapsed run) covers both copies.

**m3. The stated cause of the dual-tail slot is imprecise.**

- Reverting only the dual tail's own mask build (lib.rs:2587) brings back `[rbp-0xc8]`.
- Reverting only the dual masked pair's masks (:2555-2556) leaves the tail clean.
- So the slot follows the tail site's in-place mask construction, which also feeds the masked
  tail loop. It does not follow the masked pair.
- The prose in lib.rs:2507-2512 and in the gate's docstring should say so.

The honest "not structural" statement stands.

## NIT

- **n1.** The self-test does not pin `selects`' memory-operand arm. Dropping `not from_memory`
  keeps all 26 cases green, yet the docstring states that memory operands count as selects.
- **n2.** The spec says "now five held rows" at :479 and "all four held rows" at :489. The gate
  has four held rows and one reported row.
- **n3.** Builtins M2: after #1328, T2 no longer exercises the twin's per-word arm. It is recorded
  honestly; it is a small loss of coverage for the oracle twin.

## Judgments on the points asked

- **A2/A6: satisfied in substance.** The dual depth-1 tail carries no slot at 90444c537.
  - The held dual-tail row goes red when the `[rbp-0xc8]`-class spill returns. I reproduced this:
    attempt 1's code (A3 reverted) gives `FAIL … [rbp-0xc8]`, `movq [rbp-0xc8],r14`.
  - The evidence states honestly that the allocation is not structural (see m3 for the precise
    site).
  - The check, its semantics and its bits are untouched.
  - I did not reproduce the joint-fold measurement (`[rbp-0xf0]`).
  - The root should ratify that A2's "restructure the check" was met by A3 instead.
- **A3: correct.** Every coefficient or `remaining` change site refreshes `dry`, through
  `refresh_identity`:
  - `settle` and `start_ramp`;
  - the snap in `process_section`;
  - `discontinuity_reset`;
  - `from_prepared`, for full resets;
  - `commit_track`, for restore and swap import.

  `desymmetrize` copies it (but see m2), and snapshot export is unaffected. `Lane::Mask: Send`
  adds no `unsafe`; the compiler checks every implementation, and the cross-target builds pass.
  It is needed because `Channel` lives in a `Send` bank, and it is in scope (`lane/src/lib.rs`).

  **No rendered bit moved:**
  - the pinned corpora (G5 at every width and on wasm, BUILTINS/E9/multiband,
    `conformance_fixtures`, builtins fixtures, graph determinism) all pass;
  - the 817 debug tests run `identity_flags_agree` on every stationary block;
  - my direct comparison of 3b0fa85ef with 90444c537 over all 3,060 EQ probe runs moved 0 samples.
- **A5: correct.**
  - The floors are 79 / 32 / 322.
  - The g2 and audit oracles reach the joint band, and their reach is asserted.
  - The stale prose is fixed, apart from m1 and m2.
  - The rest-sample NIT is fixed: 773,888 and 421,888 are the block ends.
  - The iOS memset counts are 186 and 122. `check-cross-targets.sh` passes and requests no further
    lowering, so 122 is the exact count (A5 said 128).

## Test value

- **A1 classifier (26 self-test cases).** I re-ran the implementer's rule mutations on a copy of the
  script, and each one turns the self-test red:

  | mutation | red cases |
  |---|---|
  | every `or` is a select | `flush pair` |
  | no `or` is a select | 4 cases and the verdicts |
  | one mask input suffices | 3 |
  | an outside register is taken as a mask | 3 |
  | every logic op makes a mask | 2 and the verdicts |
  | a row ignores `masked` | the verdicts |

  The defect each one defends against is a classifier that reads the joint flush's mask `or` as a
  select (every held row fails closed), or reads a real dry-mask bitselect as select-free (a masked
  loop is held as select-free). The old and new classifiers give identical verdicts on the base
  (0e3e21b68) module.
- **New held row "mono depth-2 pair, masked".** A masked mono pair that carries an integrator
  through a stack slot turns it red, and only it: with `channel.dry_mask(at[k])` read in place I got
  `FAIL [rbp-0x180]`, with every other row green. No other row looks at a masked loop.
- **Dual-tail row (rewritten docs).** The block-limit fold's general-purpose slot coming back
  (`[rbp-0xc8]`) turns it red, which I reproduced. No other row holds that loop.
- **g2 `g2_svf_step_yields_both_taps_of_one_state` (rewritten).**
  - With `svf_step` flushing per word, this test is red in the silent half (band tap, frame 2376);
    the old oracle is green with that kernel.
  - The same defect is also red in the G5 lane digests and in gates 1 and 2, so the edit adds no
    unique catch. It is M3's correction that keeps this independent oracle truthful through the
    band, and it asserts that it reaches the band.
  - Its unique catches (swapped taps, swapped `a2`/`a3`) remain.
- **Audit SVF conformance (rewritten).**
  - The audit's model losing the joint rule makes the audit panic, which I reproduced: 439
    mismatches, NEITHER arm.
  - No other check holds the audit's model to the kernel through the band.

## Gates run (export at 90444c537)

| gate | result |
|---|---|
| gate-4 `cargo test` set | 817 passed, 0 failed |
| release `lane`/`math`/`wasm-gates` | 107 passed |
| `run-wasm-gates.sh` | ok: native and simd128 legs. V8 spill gate: self-test 26 ok, and the four held rows clean (dual tail 123 instructions, mono pair, mono tail, masked mono pair 103); dual pair reported, 13 slots |
| `conformance_fixtures --check` | ok |
| audit build and `check-builtins-fixtures.sh` | ok, 50 files |
| `audit unfused-fma conformance` | SVF unfused, 0 mismatches |
| `check-unfused-seal.sh` | ok |
| `check-graph-determinism.sh` | 100/100 |
| `check-cross-targets.sh` | PASS; no lowering requested |
| `check-lane-policy`, `check-dsp-research`, `check-workspace-policy` | ok |
| `cargo fmt --check` | ok |
| clippy `-D warnings` | ok |
| worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` | all ok |
| `test-console-benchmark.sh` | PASS |

Spill-gate mutations:

- A3 reverted: `[rbp-0xc8]` and `[rbp-0x220]`.
- Mono masked read reverted: `[rbp-0x180]`.
- Tail-only revert: `[rbp-0xc8]`.
- Pair-only revert: clean.
- One-token edit: green.
- #977 attempt 1: `[rbp-0xa0]`.
