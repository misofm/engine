PASS

# #1460 attempt 1 -- adversarial verdict (Sol verifier, 2026-10-06)

Reviewed: `git diff 23a32823a 320baffce` on `codex/d15-stream-g` (fcb7151b7 spec, #1377
Amendment 2 and STREAMS.md; 0427e8647 implementation; d9441fb2b test; 320baffce record). I checked
it against the spec's D0-D4 and gates 1-5, AGENTS.md, decision 15 D15-4 and the owner's
no-shortcuts rule. All work was done on exports: `tree` = 320baffce, `base` = 23a32823a. For each
base run I copied the five changed base source files over the head tree and rebuilt in the same
target directory, then copied the head files back (checked with `cmp`). Nothing was edited, built
or committed in the worktree. Small evidence is in `/tmp/claude-1002/v1460/evidence/`.

**Verdict.** The implementation does what D1-D4 say, and all five gates are green. I ran each gate
again myself:

- **Render node.** It holds only `processor` and `quantum`, and render reads both inline.
- **Bind-time reads.** Every control-side reader reads the plan record at bind. The response row
  carries the prepared bypass in its padding: 72 B on x86-64 and 40 B on wasm32, before and after.
- **Bit identity.** Native and browser PCM are bit-identical, base against head.
- **New test.** It is the only test of 1443 that goes red on its named defect.

There is no BLOCKER and no MAJOR finding. There is one MINOR finding (an overbroad sentence in the
evidence record) and three NITs (stale or unenforced comments).

The most important result is not a defect of this attempt. It is a gap in the spec, and root must
decide on it before #1377 is applied again (Items for ROOT, item 1):

- The spec's product outcome says: "Adding a control-only field to the metadata (as #1377 does
  ...) then costs render-owned memory nothing". That statement is false.
- Every native effect processor stores its own `PreparedEffectMetadata` by value.
- Render owns that processor (`EffectNode::processor`).
- So #1377's `tail_every_peak` and `rest` will still go into render-owned memory, once in each
  processor.

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. Gate 4's evidence sentence is broader than the review behind it.**

- *Where.* `.github/ISSUE_SPECS/1460-...md:166-167`: "No render-owned struct holds
  `PreparedEffectMetadata`: `NodeKind` and `LiveControlEffect` hold `EffectNode`".
- *Why it is false.* Render-owned structs outside `runtime.rs` hold a `PreparedEffectMetadata` by
  value, for example:
  - `PreparedDelay` (`crates/delay/src/lib.rs:573-574`);
  - `PreparedParametricEq` (`crates/parametric-eq/src/lib.rs:2930-2931`);
  - the compressor's instance (`crates/compressor/src/lib.rs:405`, read at `:962-963`).

  Each one lives in the `Box<dyn PreparedNativeEffect>` that `EffectNode` and
  `LiveControlEffect` own. Gate 4 in the spec says "review of `runtime.rs`", so the code meets the
  gate. Only the record's sentence goes beyond the gate.
- *Fix.* Narrow the sentence to `runtime.rs`, for example "No struct in `graph::runtime` holds
  `PreparedEffectMetadata`". Also add a pointer to Items for ROOT, item 1. The same correction
  applies to the `GraphPreparedEffect` doc sentence "This record never enters render-owned memory"
  (`crates/graph/src/lib.rs:891`). Its `processor` field does enter render-owned memory (it moves
  into `EffectNode`), and that processor carries its own copy of the metadata.

## NIT

- **n1. Stale rationale in `ids.rs`.** `crates/graph-compiler/src/ids.rs:377-379` still says that
  the control channels are returned beside the effects "because
  `core::mem::size_of::<RuntimeOp>()` is a reported byte". After #1460 a field of
  `GraphPreparedEffect` does not reach `RuntimeOp`. `ids.rs` is not in the authorized paths, so
  this is for the next slice that edits it (stream A).
- **n2. Moot clause in `GraphEffectControlBinding`.** In the "Why beside" text
  (`crates/graph/src/lib.rs:916-919`), the implementer edited the first sentence but kept "so
  `NodeKind`'s largest variant stays unchanged". That reason no longer holds: `EffectNode::new`
  takes only `processor` and `quantum`, so a channel field on the record could not reach
  `NodeKind`. The other reason stays true: a live-control-free plan retains no channel payload.
- **n3. Unchecked layout claim.** `crates/graph/src/runtime.rs:2485-2487` says that
  `prepared_bypass` "sits in the row's padding, so the row's size is unchanged". I measured it:
  true on x86-64 (72 B) and wasm32 (40 B). But nothing checks it. `UnitIdentity` has a
  `const _: () = assert!` for the same kind of claim (`runtime.rs:2468`). If the row grows, the
  accounting stays correct, because `response_binding_table_bytes` uses `size_of`. Only the
  comment would become false without a warning. Either add the `size_of` assertion against a
  mirror with no `prepared_bypass` field (the `UnitIdentityWithoutFlags` pattern), or delete the
  sentence.

## Review answers (the caller's questions)

- **Does render read every field that is left in the render node?** Yes.
  - `EffectNode` holds `processor` and `quantum`, and nothing else (`runtime.rs:1187-1191`).
  - `execute_op` reads `effect.quantum` and `effect.processor` (`runtime.rs:3510-3538`,
    `:3570-3605`). `quantum` goes to `EffectProcessBlock::new` as the upper bound of the block.
  - `channel_symmetry` reads the processor (`:1850-1852`).
  - The other fields of `LiveControlEffect` (`control`, `spans`, `shunt`, `observation`) are
    unchanged, and render reads all of them.
- **Is anything that render reads now behind an extra pointer step or a lookup?** No.
  - `NodeKind::Effect(EffectNode)` is inline in the op, as `GraphPreparedEffect` was before.
  - `effect.quantum` replaces `effect.metadata.quantum`, at the same depth. The processor is the
    same single `Box<dyn>`.
  - `LiveControlEffect` was already boxed.
  - Render does no map or hash lookup. The `response_metadata` `BTreeMap` is read only in
    `response_owner_bindings` (`runtime.rs:6455-6520`). Its one call site (`:5979`) is in
    `build_sequential`, at bind.
- **Does any control-side reader still read control-only data from the render node?** No.
  - The response snapshot's per-node arm now reads `binding.prepared_bypass` (`runtime.rs:1536`).
  - `LiveControlEffect::new` reads `automation_capacity`, `latency` and the window check from the
    `GraphPreparedEffect` before it moves the processor (`:1226-1272`, at bind).
  - A search of the workspace finds no other reader of `NodeKind::Effect`, `live.effect` or
    `EffectNode`.
- **Is the binding row's bypass the same value the node had before?** Yes.
  - The key and priority match. `RuntimeParts::new` inserts the effect facts first. Bank,
    builtin-bank and binding facts are inserted after them and win (`:4558-4606`).
  - `node_kind` checks in the same order: membership, then `Some(processor)` bindings, then
    effects (`:4670-4706`).
  - So a node whose op is `NodeKind::Effect` always gets its own effect's facts.
  - A `Some(None)` binding is removed by `node_kind` and never inserted into the facts. It falls
    through to the effect in both places.
- **Is the effects-table lookup done only at bind or preparation?** Yes.
  - `bind` consumes `plan.effects` into `RuntimeParts`. `node_kind` removes each record by node.
    `EffectNode::new` drops the rest of the record on the bind thread.
  - Nothing keeps the metadata after bind. This agrees with the spec's non-goal.
- **Is the estimate still correct?** Yes. The live-control owner charge is
  `EFFECT_RENDER_NODE_BYTES + Box<EffectControlLane> + Box<[span]> + BypassShunt + Option<Box<ObservationLane>>`:
  - x86-64: 24 + 8 + 16 + 72 + 8 = 128 B, equal to `size_of::<LiveControlEffect>()` = 128 B.
  - wasm32: 12 + 4 + 8 + 36 + 4 = 64 B, equal to `size_of::<LiveControlEffect>()` = 64 B.
  - At base, wasm32 under-charged by 4 B: 136 + 4 + 8 + 36 + 4 = 188 B against a 192 B box,
    because `GraphPreparedEffect` is 8-aligned on wasm32. Head is exact, so this change also
    fixes that small gap.
  - `bypass_resources::a_bypass_or_live_controls_are_charged_at_least_what_they_retain` passes.
- **How does the runtime-metadata estimate move?**
  - The split-pair *unit* delta is now 0. `RuntimeUnit` and `RuntimeUnitWithoutSplitPairSlot` are
    both 248 B on x86-64 and both 160 B on wasm32. Before, the delta was 16 B on x86-64 and 8 B on
    wasm32.
  - The op delta is unchanged (16 B and 8 B). `emitted_op_layout_delta_bytes` takes the larger
    delta, so `total_bytes` does not change.
  - Only `runtime_op_containing_bytes`, `runtime_unit_containing_bytes` and possibly
    `largest_allocation_bytes` get smaller. They are calculated from `size_of`, so they stay
    correct.
  - The browser fixture's 24 resource rows are byte-identical, base against head (see below).
- **Is the use of the response row's padding sound on wasm32 and 64-bit?** Yes, measured:
  `ResponseOwnerBinding` is 72 B before and after on x86-64, and 40 B before and after on wasm32.
  (AArch64 uses the same 64-bit layout as x86-64.) The field is a plain `bool` with no unsafe
  code and no `repr`, so the layout choice is the compiler's. See n3 for the missing check.

## Measured sizes (gate 2)

I printed each value at compile time with a mismatched-array-length probe in a scratch copy of
`runtime.rs`, then removed the probe. Each probe ran with the base files and then the head files.

| Type | x86-64 base | x86-64 head | wasm32 base | wasm32 head |
|---|---|---|---|---|
| `NodeKind` | 200 | 32 | 136 | 20 |
| `RuntimeOp` | 280 | 112 | 184 | 64 |
| `RuntimeUnit` | 280 | 248 | 184 | 160 |
| `RuntimeOpWithoutSplitPairSlot` | 264 | 96 | 176 | 56 |
| `RuntimeUnitWithoutSplitPairSlot` | 264 | 248 | 176 | 160 |
| `LiveControlEffect` | 304 | 128 | 192 | 64 |
| `ResponseOwnerBinding` | 72 | 72 | 40 | 40 |
| `GraphPreparedEffect` (control side) | 200 | 200 | 136 | 136 |
| `PreparedEffectMetadata` | 104 | 104 | 88 | 88 |
| `EffectNode` (new) | -- | 24 | -- | 12 |

The implementer's x86-64 figures agree with mine. The implementer did not measure wasm32; the
wasm32 columns above are new.

## Bit identity (gate 1): base against head, done again myself

- **`audit capi` (release).** `pcm_digest` is `cb10fbface44a3a4` on both, and the whole JSON
  record is byte-identical. Both report 0 allocations, 0 deallocations, 0 locks, 0 syscalls and
  0 violations. The base build recompiled `graph` (I checked the build log). The audit session has
  a live-edited compressor insert, so it goes through `LiveControlEffect`'s `EffectNode`.
- **`graph_fixture`.** The fingerprint, the `--manifest` output and `--check` are byte-identical,
  base against head. `--check` passes on both.
- **Browser.** I built the shipped simd128 module from head and again from base (base module
  `21a6164e...`, head module `6e96405e...`). Then I ran `direct-oracle.mjs` in print mode on each.
  The two printed documents are byte-identical, including:
  - the simd128 PCM SHA-256;
  - the three native PCM SHA-256 values (`d8506beb...`, `fc760629...`, `4d1412f8...`);
  - every status, transcript and timeline;
  - all 24 resource rows, `graphMetadataBytes` among them.

  `check-browser-expected-resources.py --artifacts` says "digests and exact rows agree" for both
  modules.
- **At head, against the committed values.** All of these pass:
  - `conformance_fixtures --check`;
  - `check-graph-determinism.sh` (100/100);
  - `check-builtins-fixtures.sh` (50 files, also at base);
  - G5: `cargo test --release -p lane -p math -p wasm-gates --features math/lane` (122 passed)
    and `run-wasm-gates.sh --without-v8-spill --without-native`.

## Mutations, done again myself (each one on all of test-debug-a, `--no-fail-fast`)

| Mutant | Result |
|---|---|
| M1 `prepared_bypass: false` in place of `effect.metadata.bypass` (`runtime.rs:4565`) | 1442 pass, 1 fail: only `response::tests::a_per_node_effect_snapshot_reports_its_prepared_bypass` ("the delay's prepared bypass", left false, right true) |
| M2 `quantum: metadata.quantum / 2` (`runtime.rs:1199`) | 166 fail (graph live tests, host-core and capi render tests, among others) |
| M3 `quantum: metadata.quantum + 1` | 1443 pass. This mutant is equivalent: `EffectProcessBlock::new` uses `quantum` only as `left.len() > quantum` (`crates/effect-contract/src/lib.rs:1349`), and no block is longer than the plan quantum. |
| New test with base graph code (base `graph`/`graph-compiler`, head `response.rs`) | Passes. The test agrees with the behaviour before the change. |

## Test value

`response::tests::a_per_node_effect_snapshot_reports_its_prepared_bypass`: the test goes red when
a per-node effect's response snapshot reports a bypass that does not come from the effect's
prepared metadata. For example, the moved `prepared_bypass` is filled with `false`, or it is lost
while it goes from `ResponseOwnerFacts` to the binding row. M1 shows that none of the other 1442
test-debug-a tests catches this. The `bypass = true` iteration catches a constant-`false` mutant,
and the `bypass = false` iteration catches a constant-`true` or negated value.

The `unbounded_caps()` extraction is only a refactor of the test module. The existing EQ test keeps
its body.

## Gates run (all at head unless marked; all green)

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, and the same with
  `--all-features`. The diff adds no `allow`.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- test-debug-a, the exact `qualification.yml` command: 1443 passed, 0 failed.
- test-debug-b, the exact command: 888 passed, 0 failed. None of these crates depends on `graph`,
  `graph-compiler` or `host-core`.
- `conformance_fixtures --check`
- Policy scripts: `check-workspace-policy.sh`, `check-lane-policy.sh`, `check-realtime-policy.sh`
  (89 regions), `check-graph-policy.sh`, `check-host-core-policy.sh`, `check-rack-policy.sh`,
  `check-builtins-policy.sh`, `check-effect-runtime-policy.sh`, `check-session-policy.sh`,
  `check-env-vocabulary.sh`, `check-protocol-control-policy.sh`, `check-unfused-seal.sh`,
  `check-conformance-boundaries.sh`, `check-parametric-eq-render-contract.sh`,
  `check-bench-policy.sh`, `check-test-support-ci.py`
- `check-graph-determinism.sh` (100/100)
- Release builds of `audit`, `bench`, `capi` and `session-validator`. Then:
  - `audit capi`, at head and at base;
  - `check-builtins-fixtures.sh`, at head and at base;
  - `check-console-fixtures.sh`;
  - `check-effect-contract.sh`;
  - `check-capi-abi.sh` and its `--self-test`;
  - `check-scalar-oracle-absent.py --native`.
- G5 native leg (test-release `lane`/`math`/`wasm-gates`), and `run-wasm-gates.sh
  --without-v8-spill --without-native`.
- The worklet chain, built into fresh output directories:
  - `build-web-audioworklet.sh --named-twin`;
  - `strip-wasm-names.py --self-test` and `check`;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py --artifacts`, at head and at base;
  - `check-scalar-oracle-absent.py --wasm`;
  - `test-web-audioworklet.sh`.
- `check-cross-targets.sh`: PASS. The only failures are the known, expected #1018 iOS memset rows.
- I did not run: AArch64 hardware tests (CI only), the V8 spill leg (`artifact-gates`), and the
  SDK jobs. None of them reads a changed path in a way that this review depends on.

## Scope

Every file in the diff is in the spec's authorized paths:

- the `response.rs` hunk is inside `mod tests`;
- the `graph-compiler/src/lib.rs` hunk is the test mirror inside `mod tests`;
- the docs changes are STREAMS.md and #1377 Amendment 2.

## Items for ROOT

1. **The spec's product outcome and #1377 Amendment 2 depend on a false statement. Root must
   decide before #1377 is applied again.**
   - *The statement.* The spec (line 13-14) says: "Adding a control-only field to the metadata
     (as #1377 does with `tail_every_peak` and `rest`) then costs render-owned memory nothing."
   - *Why it is false.* Each native processor holds the full `PreparedEffectMetadata` by value,
     and render owns each processor through `EffectNode::processor`. Examples:
     `PreparedDelay` (`crates/delay/src/lib.rs:573-574`), `PreparedParametricEq`
     (`crates/parametric-eq/src/lib.rs:2930-2931`) and the compressor's `instance.metadata`.
   - *The processor copy is the original.* `effect-compiler` makes the plan's copy from
     `processor.metadata()` (`crates/effect-compiler/src/prepare.rs:561`). The trait requires that
     accessor (`crates/effect-contract/src/lib.rs:1880`).
   - *Effect of #1377 (`d0af9d53d`).* It adds the two fields to that struct and fills them in each
     effect's prepare. So `tail_every_peak` and `rest` will be in every processor's render-owned
     box.
   - *What #1460 did.* It removes the second, inline copy from every render op (176 B on x86-64
     per op, effect op or not). That is what made clippy's `large_enum_variant` go red. But #1460
     does not satisfy #1329 R5 ("render-owned memory carries no control-only data") for
     processors.
   - *What I recommend under the no-shortcuts rule.*
     - Either rule a follow-up slice: each processor keeps only the metadata words it reads at
       render (today some of `quantum`, `bypass`, `sample_rate`, `link_mode`, `latency`,
       `state_sizes`, `width`, by crate), and `metadata()` is answered from a control-side
       source;
     - or say explicitly that processor-held metadata is exempt, and correct the spec's outcome
       sentence and #1377 Amendment 2 item 1.
   - *Before #1377 is applied again.* Decide whether the fields go into `PreparedEffectMetadata`
     (so into each processor) or into a control-side record next to it.
   - *Later.* D15-4(b) says that silence skipping (#1107) will use `rest`. When that happens,
     `rest` becomes a render-read field and has a legitimate place in render-owned memory.
2. **`RuntimeUnit` is now set by the `Bank` variant.**
   - Sizes: 248 B on x86-64 against 112 B for `RuntimeOp`; 160 B on wasm32 against 64 B.
   - So each plain-op unit in the render unit table carries about 136 B (x86-64) or 96 B (wasm32)
     of unused variant space. The unit table got only 11 % smaller (x86-64), but each op got 60 %
     smaller.
   - Banks are a non-goal of #1460, and boxing the bank chain would add a pointer step on the
     render path. This is information for a future layout issue, and it needs a measurement
     first. It is not a finding.
3. **Fold m1 and n1-n3 into the record and comments.** None of them changes behaviour.
4. **GitHub.** The record says that the #1460 GitHub body does not have the attempt record or the
   updates to the Authorized paths and Test value. Sync it before #1460 closes.
