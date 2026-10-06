FAIL

# #1328 attempt 1: adversarial verdict (Sol)

- **Reviewed:** `8439972be` on `codex/d15-stream-g` (parent `0e3e21b68`). Exported by `git archive` to
  `/tmp/claude-1002/v1328/tree` (and the parent to `/tmp/claude-1002/v1328/base`); nothing was run in
  `/home/bl/misofm/wt-d15-g`.
- **Why it fails:** gate 4 is red. `scripts/run-wasm-gates.sh`'s V8 spill gate fails. The spec's
  Hazards line forbids weakening that gate, so the fix needs a spec-owner decision; an implementer
  cannot make it within the authorized paths. Two other MAJORs are spec defects (D6's evidence
  premises, and required edits that fall outside the authorized paths). The kernel, the twin, the
  EQ predicates, gates 1-3 and the D7 re-pin are correct as built.

## BLOCKER

**B1. Gate 4: the V8 spill gate is red** (`scripts/check-web-audioworklet-v8-spill.py`, run by
`scripts/run-wasm-gates.sh`). I reproduced it on this host: Node v22.23.2, V8 12.4.254.21, AMD EPYC
7313P. The native and wasm-simd128 G5 legs are green (142 cases, 0 mismatches). The parent's gate is
green: 3 held rows ok, dual pair 10 carried.

- *Cause 1, loop identification.* The gate counts a loop as select-free only when it has no
  `vpor`/`vorps` (`SELECT_OPS`, :179; :378). The two `mask_or` in `flush_pair` lower to two `vpor`
  per SVF step, so no SVF loop counts as select-free any more. All three held rows match zero loops
  and fail closed. A scratch copy of the D1 encoding with two chained `andnot` still gets 2 `vpor`
  per step (LLVM refolds it), which confirms the implementer's claim.
- *Cause 2, a real carried slot.* I changed one classifier rule in a scratch copy (select-free means
  no blend and exactly 2 OR per SVF step) and left the carried-slot rule alone. With that copy:
  - The mono depth-2 pair is clean.
  - The mono depth-1 tail is clean.
  - The **dual depth-1 tail carries `[rbp-0xc8]`**.
- **New fact.** `[rbp-0xc8]` is a **general-purpose integer**, not an SVF integrator. It is the
  per-frame output boundary-scan "bad" accumulator for the two channels (bits `0x1` and `0x100`,
  built by `vcmpps lt` against BLOCK_LIMIT, then `vptest`, `setnz` and `or`). The loop stores it at
  the top (`movq [rbp-0xc8],r14`) and reloads it mid-iteration. In the parent build the same
  accumulator stays in `r8`. All SVF integrators stay in xmm registers.
- **Evidence outside the held rows (not reported by the implementer):**
  - The masked mono depth-2 pair now carries `[rbp-0x220]`. That is a vector SVF `ic1`, stored at the
    loop top and reloaded as the `nc1*ic1` operand: the #977 store-to-load recurrence mechanism. The
    parent carried nothing there.
  - The dual masked tail now also carries `[rbp-0xc8]`.
  - Carried slots in the dual select-free pair rose from 10 to 13, and in the dual masked pair from 11
    to 12.
  - With the chained-`andnot` encoding the dual tail is clean, but the **held mono depth-2 pair**
    carries `[rbp-0x220]` instead. The spill moves when the encoding changes, so no encoding I tried
    is clean.
- **Decisions needed from the spec owner:**
  - (a) Authorize an identification change to `scripts/check-web-audioworklet-v8-spill.py`: either
    require no blend and exactly 2 OR per SVF step, or recognise the bitselect triple. Add self-test
    cases. Follow the gate's own re-pin protocol: rebuild the #1000 red arms and show they are still
    red. This does not loosen the carried-slot rule, but it is a gate edit outside this slice's paths.
  - (b) Decide the dual-tail integer slot. The choices:
    - Eliminate it. This is a code change in the EQ's interleaved tail or boundary scan, inside
      `crates/parametric-eq/src/lib.rs` payload code, which is Stream A's file, so it needs
      sequencing.
    - Accept it by an explicit owner ruling backed by a descriptive browser EQ timing. Exempting a
      held row weakens the gate.
    - Amend D1 to allow another bit-identical `flush_pair` encoding. This is not recommended: the
      probe shows the spill just moves.
  - (c) Decide whether the new masked-mono integrator spill, which the gate does not hold, is
    acceptable. A descriptive base-versus-commit browser timing of the standing one-band EQ and an
    all-live mono EQ is the evidence for that.

## MAJOR

**M1. D6's evidence premises do not reproduce** (spec :90-93, Listening :120-121). This is
independent of the implementer's harness: my own scratch harness ran 22 configs at 10 levels from 0
to -180 dBFS, with 4,864 samples of noise then 1.2M samples of silence, comparing the parent and the
commit bit for bit.

- (a) Bits move during non-silent input. The EQ low shelf 10 Hz +24 dB S 0.1 at 96 kHz, driven at
  -180 dBFS, moved 4,861 of the 4,864 input samples, starting at sample 3, with a largest change of
  -256.7 dBFS. Mechanism: the LF integrator `ic2` stays below `REST_EPS` and is zeroed each time
  `ic1` crosses the band, a signal-dependent perturbation at about -257 dBFS.
- (b) The first moved sample is not always at or below -204.8 dBFS. In the builtin HPF 100 Hz plus
  LPF 22 kHz chain at 44.1 kHz, the first moved output is at -169.4, -175.7, -191.2 and -192.0 dBFS
  for -20, -40, -60 and -80 dBFS input.
- (c) The change bound holds. The largest change across all my runs is -256.7 dBFS, below 2.2e-13
  (-253 dBFS).
- **Judgment.** This contradicts the spec's stated evidence and its Listening rationale ("every moved
  sample is below -204 dBFS"). It does not contradict the product contract (exact rest), and it does
  not contradict D15-4(a), which accepts class B with no restriction on where bits move. But D6 is the
  premise the class-B acceptance rests on, and the PR must "reproduce these numbers".
- **Decision:** the spec owner restates D6 and the Listening line on the measured invariant: the size
  of the change, about 1.5e-13 or -256 dBFS. The restatement names both moving cases above. The owner
  then re-affirms the acceptance, optionally requiring an analytic per-section bound on the change.

**M2. Derived floors are left stale, and the ruling now carries interim text.**

- `tools/bench/src/floor.rs:71` (EQ_LANE_OPS 27), `:80` (BUILTINS 69) and the test at `:638-640`.
- `scripts/console-benchmark-record-lib.jq:67,:79`.
- `scripts/test-console-benchmark.sh:674-675,776-777` (307 in the synthetic records).
- `docs/rulings/effect-floor-accounting.md:275-278` and `:620-622` ("until their follow-up re-pins
  them").

As left, console records carry floors that contradict the ruling, which says its own table is
composed by `floor.rs`. Deliverable 6 asks for "every row derived from them", but the authorized
paths omit these files. That is a spec defect, and the interim note breaks the no-shortcuts
principle. **Fix:** the spec owner authorizes the three files; the implementer updates them and
deletes the interim sentences.

**M3. Two committed bit-exact SVF oracles still state the per-word law.**

- `crates/lane/tests/g2_kernel_identity.rs:606-607`. Its doc at :584 lists the red mutation "drop one
  of the two `flush` calls", which no longer exists.
- `tools/audit/src/unfused_fma.rs:193-194,206-207`, the `SvfF32` arms that `model_conformance` asserts
  production matches bit for bit.

Both stay green only because their noise inputs never reach the joint band. **Fix:** the spec owner
authorizes the paths; the implementer restates `flush_pair` in both.

## MINOR

- **m1. Four test edits outside the authorized paths, in `crates/parametric-eq/src/lib.rs` test
  modules** (around :4303, :4782, :4856, :5120-5150, :7877-7902). They are necessary: they encoded the
  per-word law that D5 replaces, so without them gate 4 is red. They are correct (mutation evidence
  below). They touch the test module only, no payload code. The spec owner should ratify them and
  tell Stream A.
- **m2. iOS memset ratchet** (`scripts/lib/aarch64-known-defects.py`). `check-cross-targets.sh` asks
  to lower **two** rows: parametric-eq 132 to 128 **and builtins 194 to 186**. The parent reports
  194/132, so this commit caused both. Put both in the path amendment.
- **m3. Stale prose of the per-word law:**
  - the ruling appendix, `effect-floor-accounting.md:1044-1045` (its conclusion still holds);
  - the ruling's "inert" definition at `:253`;
  - the `ramping_sections` description of leg (c) at `parametric-eq/src/lib.rs:2032-2036`;
  - `crates/builtins/tests/MUTATIONS.md:13`, whose M2 line spells the old twin line.

## NIT

- **n1.** The rest samples recorded in the spec (:234-235) and `dsp-research/filters.md:23`, 773,760
  and 421,760, are the *start* of the first block where `at_rest` holds (block 6045 and block 3295).
  The test's rest point is the end of that block: 773,888 and 421,888.

## Test value

- **Gate 1** `the_input_lpf_at_the_cutoff_maximum_reaches_exact_rest`. A flush law that leaves the
  maximum-cutoff input LPF in its period-2 cycle turns it red: the per-word kernel never rests (state
  `a4c99363`, verified), and neither does a joint threshold of 1e-17 or 1e-19 applied to *both* the
  kernel and the twin, where the kernel-twin differential `stage.rs` stays green. No other test drives
  a section to rest at the cutoff maximum. It stays green at 1e-16: D3 defends the exact threshold,
  not this test.
- **Gate 2** `the_low_shelf_fixed_point_reaches_exact_rest`. A kernel with no joint rule sticks at
  `ic2 = 6.0120676e-20` and turns it red (verified). It does not discriminate the threshold: it stays
  green down to 1e-19.
- **Gate 3** `g4_pair_law_holds_at_every_width` / `g4_pair_law_cases_at_every_width`. A rest test
  built from `Lane::max`, which drops a NaN, turns red **only** here. I verified that across lane,
  builtins, parametric-eq, dsp-reference and multiband. `mask_or` and `le` also turn it red; other
  tests catch those too.
- **Rewritten** `a_non_inert_state_in_a_dead_section_refuses_elision`. An inert predicate without the
  pair term elides a dead section whose lone word lies in [FLUSH_EPS, REST_EPS). Red, together with
  the randomized differential and the three `ramping_elision` tests.
- **Rewritten** `leg_c_refuses_below_flush_eps_admits_it_and_re_engages`. A flush-shaped predicate
  without the pair term is red **only** here. It defends D5's exactness, not bit identity, as the
  implementer says.
- **Elision seed (2e-14, -1.25e-20) and `interleave_identity` unit step.** Both keep the existing
  coverage and claim no new defect. An unreached HPF/LPF still leaves the output equal to the input
  and the state at +0.0.
- **Generator arm (D5), judged by reach.** With the original generator the inert-pair mutation stays
  green; with the arm it turns red (verified).

## Re-pin (D7)

From my scalar `lane_case_values` dump, parent versus commit: **exactly** cases 1, 5, 9, 13, 17 and 21
moved. Each moved 1,152 of 8,192 samples from index 331, with largest changes of -280.8 to -284.8 dBFS.
No other lane case moved.

G5 is green at Scalar, Simd4, Simd8 and wasm simd128, so the pins are the scalar oracle's. The
delegated pins, `MANIFEST.tsv` and `conformance --check` did not move. Case 21 equals case 1, as A2
requires (an idle ramp is the plain SVF).

This use is allowed: AGENTS.md permits a digest for the single owner of a cross-target corpus (#1048).
Each case is named in the commit with its reason. It is justified.

## Gates run (export at 8439972be)

- gate-4 `cargo test` set: 816 passed, 0 failed.
- release lane/math/wasm-gates: 107 passed.
- `run-wasm-gates.sh`: **FAIL** (V8 spill gate, 3 held rows fail closed). G5 native and wasm legs
  green.
- `conformance_fixtures --check`: ok.
- `check-builtins-fixtures.sh`: ok (50 files).
- `check-graph-determinism.sh`: 100/100. The script ignores `CARGO_TARGET_DIR`, so I re-ran it with
  the in-tree target.
- `check-cross-targets.sh`: PASS, with the two "lower its row" notes in m2.
- `check-lane-policy.sh`, `check-dsp-research.sh`, `check-workspace-policy.sh`: ok.
- `clippy -D warnings`: ok.
- `fmt --check`: ok.
- D3 input-filter radius recomputed independently: 6.65e-16, κ 2.416. Matches.
- The D3 worst-case EQ bell (10 Hz Q 18 +24 dB at 96 kHz) rests at 56 s for an impulse of 1.0 and
  62 s for 15.85, inside gate 2's 100 s rationale.
