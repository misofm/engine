# Classify repeated soft-clip cubic materializations

GitHub: https://github.com/misofm/engine/issues/653

Parent: #651. Audit parent: #559 FX4. Coordination: #560. Base:
`4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447`.

## Smallest closable slice

#651 correctly decoded native odd clamp pools but failed its final review because
it omitted Wasm-scalar even clamp materialization and called target-specific clamp
folding the FX4 residual. This successor asks only which identical emitted values
are materialized more than once within a single production frame-loop iteration
across the even and odd `cubic` calls.

The result is one compact repetition matrix for native scalar, native AVX2 W8,
Wasm scalar and Wasm SIMD128 W4. One row per actual emitted value must state exact
even/odd sites and repeat count. One-time, reused-local, loop-entry and algebraically
different folded operands must be listed explicitly as exclusions. If Astra LOW
verifies a source-owned repeated value that can move across the two calls without
changing any arithmetic operation, open a separate Luna XHIGH implementation issue.
Otherwise close FX4's soft-clip slice as no-change applicability.

## Exact ownership

This issue owns only:

- `.github/ISSUE_SPECS/653-classify-repeated-soft-clip-cubic-materializations.md`;
- `artifacts/issue653-soft-clip-cubic-repetition/` for one compact input identity,
  repetition matrix, claim-specific excerpts and verdict;
- #559/#560 coordination records.

It owns no Rust/test source, manifest, dependency, lock, workflow, full compiler
output, timing, benchmark, generated resource, AudioWorklet artifact or pin. Lane B
retains exclusive artifact authority. #652 is the disjoint active lane-B issue and
owns only its prepared-effect allocation-audit paths.

## Frozen inputs and fail-closed start

The sole Luna HIGH analyst uses the six unchanged #649 payloads at their existing
paths and hashes recorded by #651:

```text
a7ff9346a898cdb04c7bd88bc715d142c0c7c330a984a96147afca3368a01e30  /tmp/issue649-softclip-native/release/deps/soft_clip-4402e1e642a34f0e.s
594ad47fec545981dc59d7c9da9797ca81def108b2a02d19de8a7c5bf653af6a  /tmp/issue649-softclip-native/release/deps/soft_clip-4402e1e642a34f0e.ll
efe9909785bcdd2049eb48600fc5eadffbcffe5f51fc60d134ba9c01b8cace47  /tmp/issue649-softclip-wasm-scalar/wasm32-unknown-unknown/release/deps/soft_clip-c6dc6570cab48744.s
abd8b091cdcf02a0de6a651a5e9ef454d1938d974eee6dc77be090a521276b1f  /tmp/issue649-softclip-wasm-scalar/wasm32-unknown-unknown/release/deps/soft_clip-c6dc6570cab48744.ll
d16176c43ade071630235a24972a2e35595c6cc420d1ad6beb764fa988024b8e  /tmp/issue649-softclip-wasm-simd128/wasm32-unknown-unknown/release/deps/soft_clip-22a3f78411ddffba.s
c35dd592367274ca00d96fdcf2c0289fa000e338249189b7b7ad984c1248a281  /tmp/issue649-softclip-wasm-simd128/wasm32-unknown-unknown/release/deps/soft_clip-22a3f78411ddffba.ll
```

Before interpretation, record exact cwd, branch, full HEAD/upstream/current-remote
equality and a clean tracked/untracked porcelain result directly in the owned
compact identity file. Run one literal `sha256sum -c` using the six lines above and
record its complete output/status. Any dirty identity, missing input, non-regular
file or hash mismatch stops. Do not create a bespoke `/tmp` preflight framework,
rerun a compiler, modify a payload or reconstruct prior mappings.

## Repetition and applicability gates

For each production shape, map the frame loop and both source calls at
`crates/soft-clip/src/kernel.rs:199` and `:201`, then classify actual emitted
values associated with `-1`, `+1`, divisor `3`, `-2/3`, `+2/3` and any compiler-
folded `-3`, `-1/3`, `+1/3`:

- Count a repetition only when the same actual value has separate materialization
  instructions/sites during one loop iteration. A pool entry alone is not work.
- A register/local loaded once and reused by both calls counts once. A value moved
  before the loop counts as loop-entry, not per-frame repetition.
- `±2/3` and folded `±1/3` are different emitted values. Their target-dependent
  relationship is an exclusion, not the repeated-splat result.
- For native values, include pool labels, raw IEEE-754 bits and exact load/broadcast
  lines. For Wasm, include exact `f32.const`/`v128.const` and local-use lines.
- Confirm each repeat is within the loop boundaries and is used by the named even
  or odd call. Preserve enough raw lines for Astra LOW to reproduce every count.
- Separately state whether the repeated values originate in the two invocations of
  the same source helper and could be supplied once to both calls without moving,
  deleting, reordering or reassociating arithmetic. This is applicability only,
  never a speedup or compiler-optimality claim.

Do not propose moving the half-scale operation, reciprocal substitution or any
class-B arithmetic. Do not infer one target from another or inherit #647/#649/#651
tables. No timing, cycle, projected-savings or artifact claim is permitted.

## Workflow and stop rules

1. Astra LOW reviews the clean pushed brief, six live hashes, disjoint ownership
   and exact repetition definition.
2. After PASS, one named Luna HIGH analyst performs the literal identity/hash check
   once and writes one compact repetition matrix/excerpt tranche. Stop on failure.
3. Root checkpoints and pushes that exact-path tranche, then Astra LOW independently
   verifies every count and the applicability conclusion.
4. Up to three total attempts are permitted. Each correction is documentation-only,
   separately scoped and re-reviewed; no compiler recapture or source change may be
   used to repair evidence. Attempt 3 failure is a hard stop.
5. PASS closes this evidence issue and either records no-change applicability or
   opens one separately numbered Luna XHIGH source issue for only the verified
   repeats. Any evidence-only merge still requires exact-head/current-main review,
   required PR CI, guarded exact-parent merge, post-main qualification, tracker and
   GitHub synchronization, and clean delivered-worktree removal.

No source implementation or lane-B artifact work is authorized by #653.

## Astra LOW scope verdict and analyst

Astra LOW returned **PASS** at exact clean pushed brief
`004d2951733a780e5184a500253a03d042c80786`, live main
`4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447` and synchronized tracker
`f53fcba3cfc9ba4a4072026580ba8be32cdbf989`. GitHub #653 matches, #651 is
closed and all six payload hashes match. The identical-value/within-iteration rule
separates actual repetition from reuse and different folded operands without
moving arithmetic; #652 ownership is disjoint.

Nash, Luna HIGH agent `issue638_luna_high`, is the sole analyst. Nash must capture
clean porcelain before creating any owned evidence file, then run the literal hash
check and one read-only repetition matrix/excerpt tranche. Preserve predecessor
failures. No compiler, source, timing or artifact work is authorized.

## Attempt 1 FAIL and bounded attempt 2

Luna's identity and literal six-file hash check ran and passed at clean pushed
authorization `67d243d8594f4b49fde97154e88df521bb0f855a`; the proposed evidence
was subsequently checkpointed at `d95f63235ee48397a32f250b22acdea5a67cb193`,
and the identical threshold/divisor repeats remained a plausible hypothesis. Astra
LOW returned **FAIL** because the matrix again omitted Wasm scalar even-clamp sites:
line 4614 materializes `+2/3` as `f32.const 0x1.555556p-1`, and line 4615
materializes `-2/3` as `f32.const -0x1.555556p-1` within `.LBB11_22`.

Attempt 2 is documentation-only and preserves the accepted identity/hash evidence
and unchanged payloads. It may edit only the existing #653 matrix, excerpts and
verdict plus this issue record. Add the exact 4614-4615 sites and sufficient
surrounding stack/loop evidence, keep each at count one, and remove the incorrect
claim that surrounding assembly has no separate clamp constant. Explicitly label
loop-entry count separately from per-iteration count and give every folded odd
`±1/3` exclusion its actual bits and site. Do not change the repeat definition or
extend the applicability conclusion beyond supplying unchanged values once to both
helper calls. No new hash run, compiler, source, timing, payload or artifact work
is authorized. Fresh Astra LOW scope PASS is required before Luna edits.

The first attempt-2 scope review returned **FAIL** at clean pushed
`0a80a6c81f3c3239c08807fca6210f7190250bbc` because it incorrectly said the
identity/hash check ran at the later evidence checkpoint rather than authorization
`67d243d8594f4b49fde97154e88df521bb0f855a`. This final attempt-3 brief
corrects only that provenance distinction. The original identity evidence stays
unchanged, no check reruns, and the bounded matrix/excerpts/verdict edits above
remain the only executable work after a fresh Astra LOW PASS. Any failure exhausts
#653; no fourth attempt.

## Astra LOW final-attempt scope verdict

Astra LOW returned **PASS** at exact clean pushed final brief
`73dd0e882eb85fddaf88293e1fa14b485e2a9016`, live main
`4acfa4a1c25248e47bdd6bc14e34c9cb6ac43447` and synchronized tracker
`ef0032871cfceedc0b34a8071a5aa21daef210a8`. Execution at `67d243d8` and
evidence checkpoint `d95f6323` are correctly distinguished; the original identity
record is unchanged.

Nash alone may make the bounded matrix/excerpts/verdict corrections: exact Wasm
scalar clamp sites and stack support, count-one classification, loop-entry versus
per-iteration labels, and folded-operand bits/sites. No rerun, compiler, source,
timing, payload or artifact work is authorized. Any failure exhausts #653.
