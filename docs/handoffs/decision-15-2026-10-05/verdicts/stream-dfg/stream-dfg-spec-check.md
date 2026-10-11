FAIL

# Spec-only check: 233ee9071 + daf47d2d0 on codex/d15-batch-dfg (parent 622aba3b1)

Fresh verifier. I did not edit, build or check out anything in /home/bl/misofm/wt-d15-dfg, and its
`git status` was clean before and after. I ran one kernel probe in a `git archive daf47d2d0` export at
/tmp/claude-1002/vspec/tree, with its own CARGO_TARGET_DIR, and then deleted that export. The probe
source is kept at
/tmp/claude-1002/-home-bl-misofm-engine/258223fb-6f51-4ebb-b550-04d18c009589/scratchpad/vspec/zz_probe.rs.

## Checks

1. Scope: PASS. `git diff --stat 622aba3b1 daf47d2d0` touches four files, all in
   `.github/ISSUE_SPECS/` (1268, 1471, 1515, 1516). `git diff --quiet 622aba3b1 daf47d2d0 -- crates`
   is clean, so no code changed.
2. #1471: PASS. The root note is one added line (1471 spec :180) directly after gate 5, and the gate
   text is unchanged. It matches `a_refused_rebuild_leaves_the_seed_as_it_was`
   (crates/capi/tests/design_bound_cache.rs:519-583). The cap is fixed at compile (`compile_at(..,
   capped(peak - 1))`). Edit A is refused one byte below its bisected peak with `refused.designs == 1`.
   Edit B (strips 7 and 8 take A's 55 Hz design through `builtins_like`) then commits with
   `committed.designs == 1`.
3. #1268 Amendment 1:
   - (a) The three passages are consistent with each other. They are not true in general. See MAJOR-1.
   - (b) Gate 1 matches the test. Every leg checks out against filter_liveness.rs:1286-1383 (both
     bodies; plain and `neighbour`; `6..14`; sawtooth `((n % p)/p - 0.5) * 0.8` with `p = 61 + 12*lane`;
     six -0.0 blocks; `[0; 8]` and the plan from block 2, with the neighbour's left HPF live and its
     right HPF set by `matches!(body, Dual)`; lane 1 output from block 3; output and state of lanes
     0, 2 and 3 in every block). Gate 1 is not weaker than the original gate 1 for any defect it
     defends. The original's "negative samples" history protected only the signed-zero output claim,
     and in the HPF scenario no output leg can see that claim. The state legs check exact `+0.0`
     bits. One small overstatement: NIT-2.
   - (c) Anchors: PASS. All of these match the code on the head: `section_is_identity` :1590-1599;
     the clears :1166-1167 and :1410-1411; the `_body` functions :1071 and :1328, with their callers
     :1017 and :1280; `svf_step` kernels.rs:1076-1077 and `svf_step_when` :1093;
     `refresh_filter_plan` lib.rs:1313-1335, called at :1799 (stage `process` :1762) and :1944
     (stage `process_mono` :1908); readback lib.rs:5822-5830; bank `process` :3909 and
     `process_mono` :3925; filter_liveness.rs:223. 11f59fa42 is an ancestor of the head, and it
     deletes `input_tail_is_infinite_while_a_filter_target_is_ramping`.
   - (d) PASS. The diff adds no "producer" or "fan" wording.
4. #1515 and #1516: PASS. Each spec is byte-identical (`cmp`) to its scratchpad source
   (hcdoc/spec.md, lanecomment/spec.md). The filenames match the title slugs. Neither is implemented:
   host-core lib.rs:19 still has the unconditional `[`SessionControlProvider`]` link, and no code
   changed.
   Root's added check: both committed specs are byte-identical to their GitHub issue bodies. I
   compared the raw JSON `body` field (1515: 1707 bytes, 1516: 2142 bytes, both ending in "\n"), and
   both issues are OPEN with matching titles. `gh issue view N --json body -q .body` prints one extra
   newline (1708 and 2142+1 bytes). That newline comes from gh's `-q` output, not from the bodies.
5. `scripts/check-workspace-policy.sh` writes only to `mktemp -d /tmp/workspace-policy.*` (and
   deletes it on exit), and it reads git with `rev-parse` and `ls-files`. I ran it in the worktree:
   `workspace policy: ok`, exit 0, tree still clean.

## Findings

**MAJOR-1: the general signed-zero claim is false for the LPF section.** Locations: 1268 spec :14-19
(Product outcome), :55-59 (Context), :76-81 (D4), :159-160 (Test value "none exists (D4)") and
:290-294 (Amendment).

The spec now says that a stale integrator never changes an output bit, the sign of an exact zero
included, "with or without the clear", because "the stage that follows ends in `v + 0.0`". That is
true only for the HPF (section 0) followed by an identity or elided LPF, which is the scenario
gate 1 tests. It is false for a live-disabled **LPF** (section 1): nothing follows it in the chain,
so nothing adds `+0.0` after it.

Counterexample:
- The HPF is real and rested (`m0, m1, m2 = 1, -k, -1`; integrators `+0.0`; HPF design at
  lib.rs:754-757). A `-0.0` input gives `v0 = -0.0`, `v1 = v2 = +0.0`, and its mix gives `-0.0`.
- A stale identity LPF with negative integrators then gives `fma(+0, ic2<0, fma(+0, ic1<0, -0.0))`,
  which is `-0.0`.
- The never-enabled twin elides the LPF, so `mixed_chain_block` adds `+0.0` after the HPF
  (`mixed_channel_block`, builtins.rs:1946-1947) and gives `+0.0`.

Measured on the real kernel (`input_chain_block_elided`, f32 lane, 120 Hz HPF at 48 kHz, LPF
integrators (-0.1, -0.05), x = -0.0):
- bank output `0x80000000`, twin output `0x0`;
- the stale-HPF case (B): `0x0` and `0x0`.

So without the clear, a stale LPF integrator does change the sign of an exact zero at the chain
output. The clear is observable at the output for the LPF, and the original D4 ("no output sample
except the sign of an exact zero") was the true statement. Production is not affected: main has the
clear, and gate 1 and its test are correct for the HPF.

Fix: limit the "no output bit" statement to the HPF that is followed by an identity or elided LPF,
which is what the test and the attempt-1 verdict measured. Also say that, without the clear, a stale
LPF after a rested real HPF can emit `-0.0` where the never-enabled form emits `+0.0`. Then restore
D4's caveat for that case.

**MINOR-1: the #1516 Context repeats the false general claim.** 1516 spec :15-17 says that #1268's
evidence "shows that at the strip output a stale integrator never changes an output bit, the sign of
an exact zero included". It shows this only for the HPF case (MAJOR-1). The same probe gives these
results:
- `-0.0` integrators on either section give `+0.0` out for an input of `±0.0` (cases C and D). So
  the comment's literal "identity coefficients over a `-0.0` integrator emit `-0.0`" is false.
- A negative *nonzero* stale integrator on the LPF does emit `-0.0` (case A). So an observable
  divergence exists, but for a different reason than the comment gives.

Correct the Context before #1516 is briefed. Otherwise the implementer starts from a false premise.

**NIT-1: the output formula names the integrators where the step outputs belong.** 1268 spec :57,
:78, :204 and :292 write the settled output as `fma(0, ic2, fma(0, ic1, 1 * x))`. The kernel computes
`fma(m2, v2, fma(m1, v1, m0 * v0))` with `v1 = ic1 + d1` and `v2 = ic2 + d2`, where `d1` and `d2` are
signed zeros. These are equal bit for bit except when an integrator is itself a zero. The conclusion
does not change.

**NIT-2: gate 1 asserts slightly more than the test checks.** 1268 spec :118 says "lane 1's first two
state words are not zero". The test asserts `assert_ne!(&live[..2], &[0, 0])` (filter_liveness.rs:1318),
which only proves that the two words are not *both* zero. The neighbour's "live state" (:1325) uses
the same check. Change the gate to "not both zero" so that it states exactly what the test asserts.
