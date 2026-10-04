Verdict: PASS

# #1253 attempt 1: adversarial verdict (Sol)

- Commit under review: `b5489f00d` (parent `11b7c92e4`), branch `codex/1053-live-updates`.
- Reviewed from the export `/tmp/claude-1002/v1253-a1`, built in its own `target/`.
- Mutations ran in a second export, `/tmp/claude-1002/v1253-a1-mut`.
- Logs are in `/tmp/claude-1002/v1253-a1-logs/`.
- I did not edit, build in or check out `/home/bl/misofm/wt-1053`.

## Summary

The slice does what D1-D3 ask, within the authorized paths:

- The four drains are bounded with the `begin_block` shape.
- The two production drains are marked realtime regions.
- The floors rise by exactly one file and two regions, to 13 files and 43 regions.
- One line-based rule refuses `while let Ok(..) = ..try_pop()` inside marked bodies.
- One mutation case proves the rule.

Single-threaded behaviour is identical: every record present at entry is still popped, in order, in the same block. All the gates I re-ran are green, the module digests reproduce exactly, and all four mutations I ran went red and then green again.

There are no BLOCKER, MAJOR or MINOR findings, only two NITs.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR
None.

### NIT

**N1. The rule matches one spelling of an unbounded drain.**
- Where: `scripts/check-realtime-policy.sh:84-85`.
- The rule is a single-line regex. I put three unbounded variants into a marked fixture region, and the gate passed all three:
  - `while let Ok(record) =` with `control.try_pop()` wrapped onto the next line. rustfmt produces this shape when the line passes `max_width = 100`, for example with a long receiver inside a deep `impl`.
  - `loop { let Ok(record) = control.try_pop() else { break }; .. }`.
  - `while let Ok(record) = Consumer::try_pop(control) { .. }`.
- It does catch the canonical form and the `Ok(record)=control.try_pop()` form without spaces.
- This matches the slice's D3 exactly, so it is not a defect of this attempt. Umbrella D11's wording ("refuses an unbounded `try_pop` loop") is broader than what the gate can prove.
- Fix (optional, as a follow-up): a structural rule, for example any `try_pop(` in a marked body requires `available_at_entry` in the same region, plus mutation cases for the `loop`/`else break` form and the wrapped form. Otherwise, narrow D11's wording to the spelling the gate refuses.

**N2. Stale prose outside the authorized paths.**
- Where: `hosts/host-web/src/tests.rs:3097` and `:4352`, and `hosts/host-web/MUTATIONS.md:19`.
- All three still describe the red mutation as moving "the `while let Ok(record) = self.control.try_pop()` drain". That text no longer exists: the oracle drains are now `for _ in 0..available { .. }`.
- The mutation is still performable (move the drain call after `process`), so the tests keep their value. Only the wording is stale.
- The implementer was right not to touch those paths.
- Fix: reword to "the `drain_controls` call" in the next slice that is authorized to touch host-web prose. #1257 touches docs and is the natural home.

### Forward note (not a finding)

Under `Concurrent` delivery, each lane snapshots `available_at_entry()` when its own turn comes in the per-lane loop, not once per block. A record pushed mid-render can therefore land this block or the next, depending on whether its lane has been drained yet.
- This is bounded: at most capacity × lanes per drain.
- It never drops a record. A record not popped stays queued, so an ack cannot precede a drop through this code.
- #1257 must derive the ack's `applied_at_sample` with that one-block uncertainty in mind, or serialise pushes against the render. The fused `FaderMatrixBankProcessor` only pairs `BetweenRenderCalls` banks (`make_fader_matrix`), so the fader-then-matrix split noted in the handoff F-notes cannot arise in the fused path.

## Scope check

`git diff --stat 11b7c92e4 b5489f00d` changes exactly four files: the slice spec, `crates/builtins-compiler/src/lib.rs`, `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh`.

In `lib.rs`, the only edits are:
- the four drain loops;
- two explanatory comment lines inside each production drain;
- the two marker pairs.

Nothing else moved:
- No record type, queue, delivery mode, host-core, host-web or capi change.
- No new Rust test, as the non-goal requires.
- The fused processor's fader-then-matrix order is unchanged (`lib.rs:1103-1104`).
- The new region bodies pass the forbidden-body predicate as written. The `FADER_MATRIX_LIVE_WITNESS.with(..)` witness counters stay under `cfg(test/test-support)`.

Other `while let Ok(..) = ..try_pop()` loops left in the tree are not render-path code:
- `RealtimePlanOwner` and `PlanRetirer` `Drop` impls (`plan_exchange.rs:466/486`).
- Meter-snapshot consumers in tests (graph-compiler, builtins-compiler, console-workload, audit fixtures).

The commit message ends with the required `Co-Authored-By` line.

The worktree currently has uncommitted edits by the next implementer (`builtins-compiler` lib and tests, `graph-compiler`, `host-core/prepare.rs`). They are not part of `b5489f00d` and were not reviewed.

## Test value

**New `marked-unbounded-try-pop-drain`** (`scripts/test-realtime-policy.sh`)
- It turns red if the gate's unbounded-drain rule is deleted, scoped away from the marked bodies, or its regex stops matching the canonical `while let Ok(record) = control.try_pop()` form.
- No existing case catches this: every other mutation case expects a different failure class.
- Proven by mutation B below.

**Rewritten `marked-file-count-floor` and `no-marked-files-uses-floor`** (expected message "thirteen")
- Red if the file floor is left at 12, so dropping the new builtins-compiler markers would not be noticed.
- Proven by mutation C.

**Rewritten `marked-region-count-floor`** (expected message "forty-three")
- Red if the region floor is left at 41.
- Proven by mutation D.

**The `create_fixture` and `empty_bodies` additions** are not tests on their own. They keep the synthetic tree at exactly the new floors. I measured the fixture at 43 regions in 13 files; at the parent it was 41 in 12.

## Mutation runs (re-run by Sol)

| Mutation | Change | Result | After revert |
|---|---|---|---|
| A | Production `drain_matrix_controls` reverted to `while let Ok(record) = control.try_pop()` | `check-realtime-policy.sh` red: "marked realtime unbounded try_pop drain (bound it with available_at_entry)" at `lib.rs:1025` | green (56 regions in 16 files) |
| B | New `gate_scan_forbidden` rule deleted | `test-realtime-policy.sh` red: "mutation unexpectedly passed: marked-unbounded-try-pop-drain" | green |
| C | File floor `-ge 13` set back to `-ge 12` | `test-realtime-policy.sh` red: `marked-file-count-floor` failed with the wrong class | green |
| D | Region floor `-ge 43` set back to `-ge 41` | `test-realtime-policy.sh` red: "mutation unexpectedly passed: marked-region-count-floor" | green |

Mutation A matches the implementer's A, B matches their B and C matches their C. D is mine.

## Gates re-run (from the export of `b5489f00d`)

### Gate 1: behaviour is unchanged
- `cargo test --locked -p builtins-compiler --features test-support`: PASS. 52 unit, 10 + 3 + 5 + 1 + 2 integration, 1 ignored (the nightly scale gate).
- The workspace test command, exactly as the spec gives it: PASS. 115 `test result: ok` lines, no failures.
- `cargo test --locked -p builtins-compiler --no-run`: PASS.

### Gate 2: the policy rule
- `bash scripts/check-realtime-policy.sh && bash scripts/test-realtime-policy.sh`: PASS.
  - `realtime policy: ok (56 marked regions in 16 files)`.
  - `realtime policy mutation tests: ok`.

### Gate 3: static rendering is unchanged
- `cargo test --locked --release -p audit -p bench -p console-workload`: PASS (10 `test result: ok` lines).

### Gate 4: the realtime audits
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`: PASS.
- `trace-builtins-audit.sh`: PASS.
- `trace-builtins-graph-audit.sh`: PASS.
- `./target/release/audit capi`, over 100000 calls: allocations 0, deallocations 0, locks 0, syscalls 0, `total_violations` 0.

### Gate 5: the shipped AudioWorklet module
- Branch: `6eb292980c2f31a7a2b86ef8388a17b1f0b5efa1b70544ec1a955d1bd207188f` (2850019 B).
- Base `54b0a1bf8`: `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4` (2849915 B).
- Both match the implementer's figures exactly. The digest changed, as the spec expects.
- `check-web-audioworklet.sh --without-metadata-regeneration`: PASS.
  - My first run reported a missing `abi-layout.json` and `parameter-metadata.json`. That was my own sequencing error: I started the check before the build had finished writing them. The rerun on the finished build passed.
- `check-browser-expected-resources.py --artifacts`: PASS.
- `test-web-audioworklet.sh`: PASS.

### Gate 6: workspace and policy
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: PASS.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: PASS.
- host-core, realtime and workspace check+test policy loop: PASS.
- `scripts/check-cross-targets.sh`: PASS. The iOS `memset_pattern16` expected failures are unchanged, including transient-shaper 268 and true-peak-limiter 104.
- `scripts/check-capi-abi.sh`: PASS.
- `run-aarch64-tests.sh debug`: not run. This is an x86-64 host, so it runs only in CI's `aarch64-debug` job.

## Acked-batch question

Can an ack ever precede a drop? Not through this change:
- The bounded drain never discards a record. Records beyond the entry snapshot stay queued for the next block.
- An apply error returns early and leaves the rest queued, exactly as before.
- For the browser (`BetweenRenderCalls`), every acked record is present at the next block's entry, so the ack sample is unchanged.
