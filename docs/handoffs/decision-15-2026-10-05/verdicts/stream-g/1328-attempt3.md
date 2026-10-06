FAIL

# #1328 attempt 3: adversarial verdict

- **Reviewed:** `git diff 1f333e98d ad96a6327` on `codex/d15-stream-g` (documentation, three tests,
  no DSP change). The commit was exported with `git archive` to `/tmp/claude-1002/v1328a3/tree`,
  and a second copy (`.../mut`) took the mutations. Nothing was built or changed in
  `/home/bl/misofm/wt-d15-g`.
- **Host:** AMD EPYC 7313P, Node v22.23.2 (V8 12.4.254.21-node.56).
- **Why it fails:** two MAJOR findings, both in the record, not in the code.
  - **M1** is not the implementer's fault: attempt 3 recorded A7 item 1 word for word, as told. The
    A7 bound is true for one section, but in a cascade the change at the effect's output is about
    70 dB larger (-116 dBFS, measured on the real EQ). The root must re-affirm on effect-level text.
  - **M2:** the new EQ test's doc and the attempt-3 record say that a stale `identity` flag "only
    chooses a schedule". That is false. A release build with a stale flag skips a live HPF.
- **What passes:** the code (no DSP change), the three new tests, which are good, the spill-gate
  records, the NIT folds, every gate I ran, and the GitHub sync.
- Scratch evidence: `/home/bl/misofm/submix-verdicts/1328-attempt3-verifier-scratch.rs`.

## MAJOR

**M1. The A4/A7 bound applies to one section only. In a cascade the change compounds, to about
1.5e-6 (-116 dBFS) at the EQ's output, which is 70 dB above the stated -186 dBFS.**
(A4 at spec :129-140; A7 item 1 at :176-191 (A7 is :172-207); the Listening line at :224-226;
`dsp-research/filters.md:19`.)

- **Mechanism (this follows from the A7 analysis itself):**
  - A section whose input stays below its `L*` passes only its direct term, with a gain of about 1.
  - So the next section gets the input without the first section's boost. If that input is below
    the next section's own `L*`, that section also stays at rest.
  - The per-word law runs each section's full response, so each following section gets the boosted
    signal.
  - At the chain's output the change is therefore `(H_chain - G_direct)·x`, not
    `(h_1 - g_1·δ)·x`. The bound becomes `‖h_chain − G_direct·δ‖₁·L*`.
- **Measured on the real EQ** (`ParametricEqFactory.prepare`, 96 kHz, input is a 3.84 Hz square
  wave, then silence). The per-word build changes only `flush_pair`'s `rest` mask in
  `crates/lane/src/lib.rs`.

  | four bands, low shelf +24 dB, S 1 | input | per-word output peak | joint output peak | change at the EQ output |
  |---|---|---|---|---|
  | 10 Hz | 3e-11 (-210.5 dBFS) | 1.509e-6 | 3.008e-11 | about **1.5e-6 (-116.4 dBFS)** |
  | 10 Hz | 1e-11 (-220 dBFS) | 5.03e-7 | 1.003e-11 | about 5.0e-7 (-126 dBFS) |
  | 100 Hz | 3e-12 (-230.5 dBFS) | 2.207e-7 | 5.27e-11 | about **2.2e-7 (-133 dBFS)**, in the bass band |

  An independent `f32` simulation of the recurrence gives the same numbers (-116.4 dBFS for four
  sections at 10 Hz) and also -162.7 dBFS for two sections. The same simulation reproduces the
  single-section point exactly (`4.9379e-10`, see the judgment on item 1 below).
- **What is wrong in the record:**
  - The opening sentence of A4 ("moves output bits by a bounded amount, at any input level") and
    its number are scoped to "the section's output". No sentence says that the change compounds
    through a chain. Readers will take -186 dBFS as the change at the effect's output.
  - The Listening line ("every change is below −186 dBFS at the section's output") is true, but it
    is not the quantity that reaches the output bits. At -116 dBFS (and at -133 dBFS in the bass
    band), the change is above a 24-bit LSB (-138.5 dBFS). It is still inaudible: infrasonic or
    bass content, more than 100 dB down.
  - The acceptance reason ("5.0e-10 is below the f32 rounding error of any signal above about
    −30 dBFS … under the arithmetic noise the engine already accepts") does not hold at 1.5e-6.
  - The rejection of the `x == 0` gate ("buys nothing measurable at −186 dBFS") rests on the
    single-section number.
  - `filters.md` says "Below `L*` a section loses its whole response (up to 24.5 dB …)". A
    four-band EQ loses up to about 96 dB of response this way.
- **This is not an implementer fault.** A7 item 1 dictated this text, and attempt 2's verifier
  (whose recommended text A7 adopted) measured single-section EQs only.
- **Fix (root decision, then doc-only):** the root re-affirms A4 on text that states the chain
  bound and the measured effect-level numbers, or reconsiders the `x == 0` alternative. Suggested
  addition:

  > The bound holds at one section's output. A section in the dead zone passes only its direct
  > term, so a following section sees the input without the boost and stays at rest too if that
  > input is below its own `L*`. Through a chain, the change is at most
  > `‖h_chain − G_direct·δ‖₁·L*`. Measured at the EQ's output with four +24 dB low shelves at
  > 96 kHz: about `1.5e-6` (−116 dBFS) at 10 Hz and about `2.2e-7` (−133 dBFS) at 100 Hz.

  Attempt 4 then records the result in A4, the Listening line and `filters.md`.

**M2. The new EQ test's doc and the attempt-3 record say a stale `identity` "only chooses a
schedule". That is false. A release build with a stale flag skips a live section.**
(`crates/parametric-eq/tests/mono_collapse.rs:325-328`; spec :596-598, "Release stays green, as the
test's doc says: a stale `identity` only chooses a schedule (the crate's gates prove the schedules
render the same bits)".)

- **Reproduced:**
  - **Mutant:** `self.right.identity = self.left.identity;` dropped from `desymmetrize`, release
    build.
  - **Scenario:** the test's off->on HPF ramp ends while the bank is collapsed. Then
    `desymmetrize_channels` runs. Then a **left-only** retarget switches the left HPF off. The input
    has no `-0.0`.
  - **Result:** the dual elision gate (`left.identity && right.identity`, lib.rs:2395) skips
    section 0 on both channels, but the right channel's HPF is still live. The right plane moves at
    block 15: `-0.00031907234` against `-0.00058527436` for the never-collapsed bank. The unmutated
    tree is green on the same scenario.
- **Why the committed test stays green in release:**
  - Every block of its input carries `-0.0` (`sample()`, index % 5 == 0), so `block_admits_elision`
    refuses elision on every block. The stale flag never selects anything there.
  - Directly after `desymmetrize`, the AND with the left channel's correct flag also protects it.
  - The test's conclusion ("both mutants render the same bits here") is true. The reason it gives
    is false, and the spec record states that reason as a general fact.
- Attempt 2's m2 had already said "A stale `identity` can elide a live section in release
  builds". This attempt states the opposite without evidence. The new `desymmetrize` doc in
  lib.rs is correct; only the test doc and the record are wrong.
- **The test still meets A7 item 4:**
  - Both mutants are red in a debug build through `identity_flags_agree` (lib.rs:1815).
  - CI's required `test-debug-b` job runs
    `cargo test --all-targets -p parametric-eq … --features …parametric-eq/test-support…` with the
    dev profile, where debug assertions are on (the workspace overrides no `[profile.dev]`).
  - On the x86-64-v3 runner the bank width is 8 (avx2), so `native_bank()` returns `Some` and the
    test runs.
  - The debug-only catch is therefore a real CI gate.
- **Fix:** correct the test doc and the record. Say that a stale `identity` can skip a live
  section in release, and that release stays green here because every block carries `-0.0`.
  Recommended but not required: add the left-only phase on a `-0.0`-free input (scratch test (1)),
  so that the identity mutant is also red in release and the gate does not depend only on a debug
  assertion.

## MINOR

None.

## NIT

- **n1.** `filters.md:19` says the measurement was "with a 5 Hz square wave". Attempt 2's harness
  square has a half-period of 12,488 samples, which is **3.84 Hz** at 96 kHz. My reproduction gives
  `4.9379e-10` at 3.84 Hz (an exact match) and `4.598e-10` at 5 Hz. This error came from attempt 2's
  verdict.
- **n2.** A7 items 5 and 6 are near-verbatim, not verbatim: articles were added, the `:479`/`:489`
  references were dropped, and "Keep A7 in the #1328 spec" became "A7 stays in this spec". The
  meaning did not change.
- **n3.** The dual-tail cause text (`lib.rs` interleave doc and the gate docstring) adds "V8 sank
  their construction into it [the masked tail loop]". The slot is in the select-free tail loop. The
  two reverts that the text quotes are verified (see item 6), but the sinking mechanism is an
  inference, and attempt 3 did not re-measure it.
- **n4.** `crates/parametric-eq/tests/MUTATIONS.md` has no row for the new test, while
  `crates/builtins/tests/MUTATIONS.md` got one (M2b). The red/green result is in the spec, as A7
  item 4 asked.

## Judgments on the points asked

1. **A7 recorded faithfully: yes, apart from M1 (root text) and n1/n2.**
   - Items 1-4 are verbatim, with backticks only.
   - The A4 quote occurs word for word in A4 and in A7.
   - The A4 bullet and the DSP-evidence Listening line are replaced.
   - The attempt-2 bullets are kept and marked superseded.
   - The `filters.md` paragraph is under "Numerical and stability limits" and contains the `L*`
     formula, the direct-term output, the bound, the worst case, the builtin and LR4 values, the HPF
     DC note, the -140 dBFS bound, the per-word -330 dBFS zone and the rejected `x == 0` gate.
   - My independent `f32` simulation (words designed as `design_svf_words_f64`, rounded once)
     gives:

     | quantity | recomputed |
     |---|---|
     | `L*`, low shelf 10 Hz +24 dB, 96 kHz | `3.049e-11` at S 1, `3.052e-11` at S 0.1 (-210.3 dBFS) |
     | `‖h − g·δ‖₁`, S 1 | 16.26, so the bound is `4.96e-10` (-186.1 dBFS) |
     | measured point (3.84 Hz square, -210.35 dBFS) | `4.9379e-10` (exact match) |
     | builtin HPF 10 Hz, 96 kHz | `1.529e-11` (-216.3 dBFS) |
     | LR4 80 Hz, 96 kHz | `1.917e-12` (-234.3 dBFS) |
     | per-word dead zone | `3.05e-17` (-330.3 dBFS) |

   - The domain minima confirm that these are the worst points: EQ 10 Hz and ±24 dB with Q/S at
     least 0.1, builtin 10 Hz, multiband crossover 80 Hz.
   - A high shelf at -24 dB has the same `L*`, so "at most" holds.
   - Decision 15 is not edited.
2. **Spill-gate records: correct.**
   - `tools/wasm-gates/MUTATIONS.md` and the gate's re-pin paragraph name #977 attempt 1
     (`0db9b2f5`, `[rbp-0xa0]`) and #1328 attempt 1's code (`[rbp-0xc8]`, `[rbp-0x220]`).
   - They record the bisect (red at `6f4c0379e` and `d09d50248`, green from `27cf24132`, 84 to
     112 instructions, green at `a9414c0c6` and `0e3e21b68`), as attempt 2's verdict measured it.
   - I re-ran the #1328 arm on ad96a6327 with the A3 reads reverted at all four sites. It is still
     red: dual tail `[rbp-0xc8]` and masked mono pair `[rbp-0x220]`, gate exit 1.
   - I did not re-bisect.
3. **EQ test:** discussed in M2.
   - Both mutants are red in debug and green in release, as recorded.
   - The full parametric-eq suite stays green on both mutants (debug, `--all-targets`, new test
     skipped), so the new test is the only catch.
   - The stale-`dry` argument is correct: a stale copy can only be "no dry lane", and an identity
     section run wet changes at most `-0.0` to `+0.0` for finite state. Here every live band
     downstream absorbs the sign.
4. **T2b: good.**
   - Twin `s1` per-word arm dropped: red at `rate=44100, index=257`.
   - Twin `s2` per-word arm dropped: red at `rate=44100, index=276`.
   - Both are red on the state-word assertion, as M2b records.
   - All 160 other builtins and dsp-reference tests (`--all-targets`, `--no-fail-fast`) stay green
     on both mutants. The baseline is 161 green.
   - It asserts that both arms fire at every launch rate.
5. **Self-test `memory operand`: good.** With `not from_memory` dropped, the self-test exits 1 and
   only `memory operand` fails (`got (2, 2, [], True)`). Restored, 27 cases are ok.
6. **Doc corrections: correct (n3 aside).**
   - Tail-only revert on ad96a6327: dual tail `[rbp-0xc8]`, every other held row clean, which
     confirms the m3 correction.
   - "Four held rows and one reported" is right everywhere, and the gate output agrees.
7. **No DSP change: confirmed.**
   - In `crates/parametric-eq/src/lib.rs`, every changed line is a `///` line.
   - The gate script changes only in its docstring and one self-test case.
   - No other source file outside tests changed.
8. **Gates:** all green (table below).
9. **GitHub sync: yes.** `gh issue view 1328 --json body` equals the spec at ad96a6327, apart from
   one trailing newline. The issue is OPEN, which is correct.

## Test value

- **`a_desymmetrized_bank_carries_the_collapsed_channels_identity_flags_and_dry_masks`:** a
  `desymmetrize` that drops or stales the right channel's `identity` flags or `dry` masks, after a
  dedicated-cut ramp ends while the bank is collapsed, turns it red in the debug build that CI runs.
  No other parametric-eq test goes red on either mutant.
- **T2b `scalar_stage_is_the_reference_recurrence_where_the_per_word_flush_fires`:** a twin (or
  kernel) whose per-word flush arm is dropped on either word turns it red. T2 and every other
  builtins/dsp-reference test stay green on both.
- **Self-test case `memory operand`:** a classifier that reads an `or` with a memory operand as a
  mask combine turns it red. The other 26 cases stay green.

## Gates run (export at ad96a6327)

| gate | result |
|---|---|
| gate-4 debug `cargo test --all-targets` set (14 packages, spec features) | 832 passed, 0 failed |
| release `cargo test --all-targets -p parametric-eq -p builtins -p dsp-reference` (test-support) | 284 passed, 0 failed (both new tests ok) |
| release `cargo test -p lane -p math -p wasm-gates --features math/lane` | 109 passed, 0 failed |
| `check-web-audioworklet-v8-spill.py --check-toolchain` / `--self-test` | ok / 27 cases ok |
| `run-wasm-gates.sh` | exit 0. Native and simd128 legs ok. Four held rows clean (dual tail 123 instructions, mono pair 93, mono tail 60, masked mono pair 103); dual pair reported with 13 slots |
| `check-dsp-research.sh`, `check-workspace-policy.sh` | ok, ok |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | ok |
| `cargo fmt --all -- --check` | ok |

Not re-run: the worklet chain, conformance and builtins fixtures, graph determinism and
cross-targets. No code under them changed: the only source change is doc comments.

Mutation runs (each reverted, and the restore checked against `git show ad96a6327:<path>`):

- `desymmetrize` identity copy dropped: debug red, release green. Scratch scenario: release red.
- `desymmetrize` dry copy dropped: debug red, release green.
- Twin `s1` / `s2` per-word arm dropped: T2b red (257 / 276).
- `not from_memory` dropped: self-test red on `memory operand` only.
- Spill gate, A3 reads reverted: `[rbp-0xc8]` and `[rbp-0x220]`. Tail-only revert: `[rbp-0xc8]`.
- `flush_pair` made per-word (for M1): EQ cascade output peaks as in the M1 table.
