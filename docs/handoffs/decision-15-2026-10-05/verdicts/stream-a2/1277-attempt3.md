PASS

# #1277 attempt 3 -- adversarial slice verdict

Small evidence (step summaries, the capi audit record, mutation logs): `/home/bl/misofm/submix-verdicts/1277-attempt3-evidence/`.

Range: attempt 3 = `9994001ad..14d15fed6` (4beb54f3e, 45d26841b, da0ddfa66, 14d15fed6), reviewed together with
the whole slice `8c0deee2c..14d15fed6` (18 files). I reviewed exports (git archive) under `/tmp/claude-1002/v1277/`.
I did not edit, build or commit in `/home/bl/misofm/wt-d15-a2`.

No BLOCKER and no MAJOR. Two MINOR and two NIT findings follow. Fix them in the closing record or in a follow-up
commit before the batch merges. They do not need another attempt.

## Attempt 2 findings: status

| Attempt 2 finding | Status |
| --- | --- |
| BLOCKER 1, `cargo doc` broken links | **Resolved.** The links are path-qualified (`live_delta.rs:146`). `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` passes. |
| MAJOR 2, followed mute treated as live | **Resolved.** `prepare.rs` has one predicate, `followed_mute_changes` (`:2354`). The classifier (`live_delta.rs:267`) and `join_strips` (`prepare.rs:2560-2562`) both call it, and `follows_mute_from` is deleted. A followed-mute change makes the fader family `Prepared`, so the strip is in `restarted_strips()` and gets no mute retarget. Two gates defend it (mutations below). |
| MINOR 3, realtime floors | **Resolved.** The floor is 111 regions in 27 files (`check-realtime-policy.sh:82`). I measured exactly 111 regions in 27 files on the export, so the floor is exact. The self-test pad (`1 x 29 + 13 x 3`) and the floor message match. The edit is named in the spec and falls within the STREAMS standing exception. Both scripts pass. |
| MINOR 4, `engine/src/realtime/mod.rs` tests | **Open, as instructed.** Root is asked for a named exception. I do not fail the attempt on it. |
| NIT 5, matrix smoothing window | **Resolved.** The `join_strips` doc explains it (`prepare.rs:2517-2519`). |
| NIT 6, vacuous in release | Unchanged and accepted. CI runs host-core tests in debug only. |

## The coordinator ruling (per family, per stage): consistent with the spec

- **D4 is per stage.** It says "a stage carries when ... no prepared value *of it* differs".
- **D3 is per value.** It cannot mean "classify the whole delta" on a successor. `classify_live_delta` refuses every
  delta whose track set differs (`LiveRebuild::Structure`), and every structural successor has such a delta. A
  whole-delta reading would make every value prepared, and D4 and gate 4 could then never carry anything.
- **So a refusal of one family's value makes only that family prepared.** A followed mute changes only the
  fader/mute stage's value. The matrix stage has no prepared difference, so it carries and is retargeted.
- **The fused stage restarts whole.** It is one stage with one key (D1). A prepared value of it differs, so it
  restarts whole (D4; D9 makes the fused stage one unit). The clamp at `prepare.rs:2619-2626` does this.
- **Audio consequence, for #1324.** In this edge case, a pan that changes together with a followed mute ramps in the
  split form (native) and steps in the fused form (browser). This is a direct result of D9 and is not a defect.
  #1324 should know about it when it duck-swaps strips in the restart set.

## Findings

### MINOR 1 -- the fused-form clamp is now reachable, and no test covers it

- **Where:** `crates/host-core/src/prepare.rs:2619-2626` (`carries = [both, both]`).
- **Why it matters now:** before attempt 3, the two families could differ in preparedness only through a `Domain`
  error, and a compiled session cannot reach that. The per-family ruling makes "fader prepared, matrix live" reachable
  (a followed mute plus a pan change), and the clamp is the only code that keeps the fused stage whole.
- **No gate exercises it:**
  - `a_followed_mute_change_restarts_the_stage_*` runs in the fused form, but X's pan does not change. Without the
    clamp, the matrix half carries the same settled value and the output does not change.
  - `a_followed_mute_change_leaves_the_matrix_stage_carrying_*` runs in the split form only.
- **Mutation D (clamp removed): every successor_swap test stays green (54/54).**
- **Probe (scratch copy only, then deleted):** fused form, X's left mute follows and changes, and X's pan changes to
  `[0.8, 0.6]` / 300.
  - With the clamp: `restarted == [X]`, and every block from `SWAP_BLOCK` on equals a fresh B.
  - With the clamp removed: the probe goes red. The matrix half carries and ramps instead of restarting at rest.
- **Fix:** add the probe as a fused-form case of the second gate. Test value: it catches a carry that moves only the
  matrix half of a fused stage whose fader half restarts.

### MINOR 2 -- the attempt-3 record contradicts the implementation

- **Where:** `.github/ISSUE_SPECS/1277-carry-fader-mute-and-pan-ramps-across-a-plan-swap.md:353-354`.
- **What it says:** "A strip it refuses carries neither stage and is in `restarted_strips()`, with no retarget
  written."
- **Why it is wrong:**
  - This is the earlier draft's behaviour. The record says itself (lines 365-371) that the earlier draft was
    replaced.
  - In the split form the matrix stage carries and gets a matrix retarget. The second gate tests exactly that.
- **Fix:** change the sentence to the per-family behaviour: the fader/mute stage restarts with no mute retarget, the
  matrix stage carries and is retargeted, and the fused stage restarts whole.

### NIT 3 -- two added doc lines exceed `max_width = 100`

`live_delta.rs:146` (127 characters) and `prepare.rs:2519` (107 characters). rustfmt does not wrap comments, so
`fmt --check` passes. Rewrap them.

### NIT 4 -- MINOR 4 of attempt 2 stays open

It waits for root's named exception, which goes in the spec and in STREAMS. This is not a reason to fail.

## The acked-batch question: can an ack ever precede a drop?

**No.** Attempt 3 does not change the order. The join only changes which family is `Live` or `Prepared`. A strip that
the predicate refuses gets no mute retarget, so there is no write to drop.

- **Retarget writes come after every fallible step.** They remain preparation's last step (`prepare.rs:2157-2176`),
  after every fallible step: the strip-move cap check, `install_builtin_strip_carry`, the inventory identity and the
  report.
- **The stamp is refused before any write.** `stamp_held_gate` is refused (D10 line) before the first write.
- **A failure inside the write loop drops the plan.** `host.control.order` drops the plan with its open stamp, which
  `GatePin` permits.
- **The control-plane order is unchanged from attempt 2.** Every refusal after preparation drops the candidate. The
  handle re-stamps `r` as the last fallible step. `publish(r)` precedes `reservation.commit()`, and `RESULT_OK`
  follows the commit.
- **Gate 8 passes** in `cargo test -p capi`.

## Test value (new tests in attempt 3)

- **`a_followed_mute_change_restarts_the_stage_at_{four,eight}_lanes`** (both forms): catches a join that treats a
  followed mute as live, carries X's fader/mute stage, writes a mute retarget and leaves X out of the restart set.
  Through its control half, it also catches a predicate that ignores the `follows_mute` flag and restarts every
  mute-changed track.
- **`a_followed_mute_change_leaves_the_matrix_stage_carrying_at_{four,eight}_lanes`** (split form): catches a join
  that lets a followed mute make the matrix family prepared. That join restarts the pan stage at its new value and
  loses the ramp the carry continues.

## Mutations re-done (scratch copy `/tmp/claude-1002/v1277/tree3`, each reverted)

| Mutation | Red |
| --- | --- |
| A: join guard removed (`fader_live` ignores `followed_mute_changes`) | both `restarts_the_stage_*` and both `leaves_the_matrix_stage_carrying_*` (4/54) |
| B: `matrix_live = fader_live && ..` | both `leaves_the_matrix_stage_carrying_*` only |
| C: predicate ignores `route.follows_mute` | both `restarts_the_stage_*` only (the control half) |
| D: fused clamp removed | **none** (54/54 green). MINOR 1; the probe goes red |
| E: fader `import_lane` zeroes `remaining` (from attempt 2) | `a_ramp_in_flight_finishes_after_the_swap_{four,eight}` |
| F: retargets never written (from attempt 2) | `a_live_difference_retargets_*` (2), the followed-mute gates (4) |
| G: classifier no longer calls the shared predicate | existing `live_delta::a_followed_mute_change_needs_a_rebuild` |

## Gates run

All gates ran from the exports with toolchain 1.97.1 on x86-64-v3, and all pass.

**`lint` job, all 32 steps, in workflow order:**

- fetch, fmt, clippy `--all-targets --all-features -D warnings`, and Documentation
  (`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`).
- Workspace policy and its mutations, session policy, env vocabulary, and bench policy.
- Test-support CI policy and its mutations, the scalar-oracle self-test, and script reachability (120 files, 20
  mutation cases).
- The console benchmark fixture, the builtins fixture-manifest mutation tests, and the bench preconditions.
- Host-core, protocol and realtime policies (111 regions in 27 files), audit-leak and artifact-evidence-leak.
- The trace validator, lane policy, the unfused seal, rack/builtins/graph policy (including `check-rack-policy`), and
  effect-runtime policy and fixtures.
- Conformance boundaries, the PEQ render contract, the release-shape self-test, npm publish modes, the stem store and
  stem identity.
- The two sub-v3 refusals and the AVX2+FMA probe.

**`test-release` job, all steps the router selects:**

- lane, math and wasm-gates in release, `filter_liveness`, `tail_contract`, `ramp_endpoint` (8 crates),
  `designer_total --ignored`, and M3's FMA cfg.
- The loom leg: `RUSTFLAGS='--cfg loom' cargo test --release -p engine --lib spsc_loom`.
- M1 and F1 do not run, because `math_closure` is false (no math path changed).

**Gate 7:**

- `cargo test --locked -p builtins -p builtins-compiler -p graph -p host-core` with the test-support features: 623
  passed, 0 failed.
- `cargo test --locked -p capi`: 103 passed.
- `cargo test --locked -p console-workload`.
- The release build of audit, bench, capi and session-validator, with `trace-builtins-audit.sh` and
  `trace-builtins-graph-audit.sh`: PASS.
- `cargo test --release -p audit -p bench -p console-workload`.
- The builtins manifest digest: `check-builtins-fixtures.sh . target/release/audit` (50 files) and
  `check-console-fixtures.sh`.
- `check-cross-targets.sh`, plus the policy, fmt and clippy steps above.

**Also run:**

- `check-capi-abi.sh` and `--self-test`.
- `audit capi`: 100000 calls, with 0 allocations, deallocations, locks, syscalls and violations.
- `cargo test -p engine --lib`, and `cargo test -p control-plane --features test-support`.

**Worklet chain:**

- `build-web-audioworklet.sh --named-twin` with the strip-names check.
- `check-web-audioworklet.sh --without-metadata-regeneration`, and `check-browser-expected-resources.py --artifacts`.
- `check-scalar-oracle-absent.py --wasm`.
- `test-web-audioworklet.sh` with a private TMPDIR, which was empty afterwards.
- The V8 spill gate and its self-test (Node 22.23.2).

**Not run:** AArch64. It runs in CI only, as the verifier rules require.

**Person-role wording:** the slice adds none. "producer" appears only as existing type and field names.
