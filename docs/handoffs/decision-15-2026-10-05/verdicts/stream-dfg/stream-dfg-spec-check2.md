PASS

# Spec-only check 2: b44fd2540 on codex/d15-batch-dfg (#1268 Root Amendment 1 correction)

Fresh verifier. I did not edit, build or check out anything in /home/bl/misofm/wt-d15-dfg. Its
`git status` was clean before and after. I ran the kernel probe in a `git archive b44fd2540` export at
/tmp/claude-1002/vspec2/tree, with CARGO_TARGET_DIR=/tmp/claude-1002/vspec2/target, and then deleted
the export.

Spec = `.github/ISSUE_SPECS/1268-elide-a-builtin-input-filter-section-again-after-a-live-disable-settles-it-to-identity.md`

## Checks

1. Scope: PASS. `git show --stat b44fd2540` shows one file, the 1268 spec (+86/-34).
   `git diff 233ee9071 b44fd2540 -- crates` is empty. (The range also includes daf47d2d0, the #1516
   spec, which is a different commit.)
2. Signed-zero and stale-integrator statements: PASS for every passage in the list (Product outcome
   :13-24, Context :29-38 and :63-77, D4 :93-106, gate 1 closing note :157-161, Test value :186-190,
   Root Amendment 1 :317-337 and the open item :369-372).
   - Kernel facts: `svf_step_when` (kernels.rs:1093) computes `v1 = ic1 + d1` and `v2 = ic2 + d2`.
     The section output is `m2.fma(v2, m1.fma(v1, m0.mul(v0)))` (builtins.rs:731-733, :1937-1939
     and :1953-1955). Section 0 is the HPF and section 1 is the LPF (`for section in 0..2`). The
     elided forms add `+0.0` (`identity_chain_block` :1735; `mixed_channel_block` :1926 and :1957).
     The designs are HPF `(1, -k, -1)` and LPF `(0, 0, 1)` (builtins lib.rs:754-758). The identity
     is m0 = `0x3F800000` and every other word `+0.0` (builtins.rs:1550-1552), so `nc1 = -0.0`.
   - Probe re-run (prior zz_probe.rs, unchanged, on b44fd2540): A (real rested HPF, stale identity
     LPF `(-0.1, -0.05)`, x = -0.0) gives bank `0x80000000` and twin `0x0`. B (stale identity HPF,
     LPF elided) gives `0x0` and `0x0`. The spec quotes both results, and they are exact.
   - Added cases: 180 combinations, 4096 frames each, with no difference in output or LPF state.
     - E: a stale identity HPF (six integrator pairs, signs mixed, `-0.0` included) followed by a
       **real** LPF (1, 8 and 20 kHz; five LPF states, zero and tiny states included), x = ±0.0.
       The twin has the HPF elided and the same LPF.
     - B2: a stale HPF followed by an identity LPF that runs (not elided) over `+0.0` integrators.
       The twin has both sections elided.
     So the HPF claim ("no output bit, the sign of an exact zero included") holds in general, not
     only in the gate-1 scenario. The LPF claim ("can change the sign of an exact zero", which needs
     negative integrators and an exact `-0.0` input) is true, and A shows it.
3. Gate 1: PASS. The word diff of the gate section from 233ee9071 to b44fd2540 has one change in
   the assertion text: "not zero" becomes "not both zero". The closing note gets the HPF and LPF
   scope. The rest is reflow. Each leg matches `settled_disable_scenario` and
   `a_settled_live_disable_is_re_elided_and_bit_identical_to_never_enabled`
   (crates/builtins/tests/filter_liveness.rs:1286-1395):
   - both bodies, plain and `neighbour`, `6..14` history blocks;
   - `assert_ne!(&live[..2], &[0, 0])`;
   - the HPF is not elided during the history;
   - the neighbour's twin lane 3 has a live state;
   - six `-0.0` blocks;
   - `[0; 8]` and the plan from block 2: the neighbour keeps `plan[0][0]` live, and `plan[1][0]`
     equals `matches!(body, Dual)`;
   - lane 1 output from block 3;
   - output and state of lanes 0, 2 and 3 in every block.
   Compared with 622aba3b1, no gate is weaker. Gate 1 adds the neighbour scenario, the history
   lengths and the other-lane state legs. Gates 2 and 3 are byte-identical.
4. NIT-1 formula: PASS. `fma(0, v2, fma(0, v1, 1 * v0))` matches the kernel operand for operand
   (`a.fma(b, c) = a*b + c`, as the `svf_step` doc order shows). With identity coefficients, `d1`
   and `d2` are signed zeros when `v0` and `ic2` are finite (spec: "finite integrators"). So `v1`
   and `v2` equal `ic1` and `ic2` bit for bit unless the integrator is itself a zero, and the result
   is `v0` when `v0` is not zero. All of these statements are exact.
5. Wording and scope: PASS. No added line contains "producer" or "fan". No other file changed.

## Findings

**MINOR-1: text that has no scope still says that a `-0.0` integrator is observable.** Spec
:231-236 (Decision record) says: "Only a `-0.0` integrator word is observable". The probe shows
that this is false on both sections. Cases C and D (`-0.0` integrators, x = ±0.0) give `+0.0`.
On the LPF, the observable state is a *negative non-zero* pair (case A).

Spec :337-338 tells the reader to read the Decision and Attempt records "with that scope". That
scope (HPF versus LPF) does not correct this sentence. Context :32-38 also refutes the filed
`-0.0` reason "for the HPF section" only. A reader can infer from this that the reason holds for
the LPF, but it does not (case C). The passages in the check list are true as written, so this
does not cause a FAIL. But #1516 is briefed from these premises. Fix: add one clause to the
:337 note or to Context :32-38. Say that a `-0.0` integrator changes no output bit on either
section, and that the LPF divergence comes from negative non-zero integrators.

**NIT-1: the HPF claim is general, but its reason covers only an identity LPF.** Product outcome
:16-19 and D4 :98-101 justify the HPF claim with "the LPF stage that follows, elided or running
over `+0.0` integrators, ends in `v + 0.0`". A real LPF on the same lane does not end in `v + 0.0`.
The claim still holds (probe E): the LPF's `(0, 0, 1)` output is `v2 + (0*v1 + 0*v0)`, and `v2` is
never `-0.0` under round-to-nearest. The text is not wrong, but it is incomplete. Optional: add
"or, for a real LPF, its output is `v2`, which no signed zero at its input moves".

**NIT-2: the "Changed passages" list still describes the earlier text.** :342-345 (Product outcome
"bit-identical with or without the clear") and :355-356 (D4 "no output bit, the sign of an exact
zero included") describe the first amendment, not the current scoped text. The paragraph at
:330-337 records the correction, so a careful reader is not misled.
