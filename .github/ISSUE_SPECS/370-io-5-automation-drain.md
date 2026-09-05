# 370 — Wire a render-side drain for the accepted-automation queue or retire it

Split out of the #349 efficiency-audit tracker, **Wave 0** (correctness and gate integrity). Rows: **IO-5**.
This issue is self-contained: implement from this body alone. Do not re-audit #349.

## Assignment

| | |
|---|---|
| Implementer | Qwen 27B |
| Verifier | Fable 5.1 (reads the diff, reruns every gate below, posts the tracker note on #349) |
| Branch | `qwen/370-io-5-automation-docs` off `main` |
| Land after | none (rescoped 2026-09-04; the drain feature is briefed separately and lands after #369) |
| PR | one PR, squash-merged, title prefixed with the row id(s) |

## Operating procedure (complete; no other instructions exist)

Repo: `/Users/bl/Documents/GitHub/misofm/engine` (GitHub `misofm/engine`). `N` is this issue's number.

**0. Claim.** `gh issue view N --json labels` must not contain `claimed`; if it does, stop, another agent owns it. Every issue in the "Land after" row must be `CLOSED` (`gh issue view <dep> --json state`); if not, stop and say which one is open. Then `gh issue edit N --add-label claimed`.

**1. Worktree.** Never work in the main checkout.
```sh
git -C /Users/bl/Documents/GitHub/misofm/engine fetch origin
git -C /Users/bl/Documents/GitHub/misofm/engine worktree add ../engine-N -b <branch from the Assignment table> origin/main
cd /Users/bl/Documents/GitHub/misofm/engine-N && export CARGO_TARGET_DIR=$PWD/target
```
Read `AGENTS.md` and `docs/REALTIME_DEPENDENCY_POLICY.md` once before editing.

**2. Scope.** Implement only the finding(s) in this issue, following any "Reconciled scope" / "Notes for the implementer" section first and the finding's "Proposed change" second. Locate code by the named functions; line numbers are from `main` `4797a544`. Anything else you notice goes in the PR under "Seen, not done" and stays untouched. No reformatting, renames, dependency bumps or new tooling. Where the finding lists ordered options, try them in that order; an option marked class B or owner ruling is never implemented: if you reach it, post the evidence on this issue and stop.

**3. Null result.** If the premise no longer holds on current `main` (code already matches the proposal, or the named code is gone), do not invent a change. Comment on this issue with a one-paragraph null note, remove the `claimed` label, and stop.

**4. Evidence, all required.**
- The exact "Verification gate" commands from the finding, output saved.
- `cargo test --workspace` in the worktree and on `origin/main`; pass counts must match unless this issue adds tests (state the delta).
- Any script you edited: also run its `scripts/test-<name>.sh` mutation harness.
- Render-path rows only: `scripts/check-realtime-policy.sh`, the bit-identity fixture suite named in the finding, and one run of `scripts/run-console-benchmark.sh` on the named row. Report before/after once; never tune.
- Codegen claims (an instruction that should disappear): attach before/after `llvm-objdump -d` (or `cargo asm`) for the named function.

**5. Deliver.**
- Commit: subject `<row-id>: <what changed>`, body listing the commands run, last line `Refs #N`.
- `gh pr create --base main` with sections: Summary; Rows closed; Gate output (fenced); Before/after; Seen, not done; Skipped (any step not done and why). Add `Closes #N` only when every row in this issue is done.
- Do not post on #349; the verifier does that. Do not `git push --force`, do not touch `main`, do not edit outside the worktree.
- Reply with the PR URL and the Skipped list.

## Decision and rescoped wave 0 work (2026-09-04)

The owner decision is posted as the `Decision:` comment on this issue: **the queue stays; the render-side drain is a separate feature, not #349 work.** The finding below is kept verbatim for the record. Do **not** implement its "Proposed change". Do **not** delete anything. This issue's remaining scope is to make the tree stop claiming something it does not do. Three edits, exact text:

1. `docs/REALTIME_MEMORY.md`, the paragraph beginning "Parameter and event delivery is owned by the protocol crate's accepted-automation queue (#102);". Replace its first sentence with:
   > Live parameter delivery is owned by the per-effect `EffectControlLane` (#140), fed by the `EffectControlProducer`s prepared with the plan; the protocol crate's accepted-automation queue (#102) admits, retains and cancels sample-timed batches but has no render-side consumer yet (see `CONTROL_PROTOCOL_SEMANTICS.md`, "Delivery status"). The plan itself carries no parameter store.
   Keep the rest of the paragraph unchanged.
2. `docs/CONTROL_PROTOCOL_SEMANTICS.md`, immediately after the paragraph beginning "Automation requires exact revision". Insert a new paragraph:
   > **Delivery status (v1).** Accepted automation is retained in the queue and consumed today only by cancellation (`AUTOMATION_CANCELED`). No render-side drain exists: sample-accurate application of point, step, linear and exponential records is the later protocol capability named in `EFFECT_CONTRACT_V1.md` ("V1 runtime automation is `Point` spans …"). The catch-up rule and the late counter in the paragraph above are specified but not implemented; `LATE_AUTOMATION` is registered and nothing increments it.
   Before inserting, run `rg -n LateAutomation crates/` and confirm the only hits are the enum definition and its decoder; if something increments it, stop and report instead of inserting the last sentence.
3. `crates/protocol/src/queue.rs`, the doc comment on `pub fn try_dequeue_automation`. Replace `/// Pop one fixed batch without decoding or allocation.` with:
   ```rust
   /// Pop one fixed batch without decoding or allocation.
   ///
   /// The only production consumer is `ProtocolController::cancel_queued_automation_reserved`;
   /// render-side delivery is deferred (see `docs/CONTROL_PROTOCOL_SEMANTICS.md`, "Delivery status").
   ```

Do not touch `AGENTS.md`, `docs/EFFECT_CONTRACT_V1.md`, any `.github/ISSUE_SPECS/00*` file, any registry, hash or fixture.

**Verification gate for the rescoped work:** `cargo test -p protocol`, `cargo doc -p protocol --no-deps` (no new warnings), and the workspace pass count unchanged.

## Finding (verbatim from #349)

#### IO-5: the accepted-automation queue has no render-side drain; two disconnected automation models exist
- **Category:** architecture
- **Class:** N/A (non-render)
- **Severity:** high
- **Location:** producer/validator `crates/protocol/src/queue.rs:196-283` + `:804-836` + `:1098-1211`; the only consumer `crates/protocol/src/controller.rs:3643-3675` (cancellation). The *actual* render automation path is `crates/effect-contract/src/live.rs:44-90` (`EffectControlRecord`) and `:184-262` (`EffectControlLane::stage`), fed only from `hosts/host-web/src/lib.rs:1878-1916`.
- **Current code:**
```rust
// crates/protocol/src/controller.rs:3663-3667 — the ONLY dequeue outside tests
let batch = match self.queues.try_dequeue_automation() {
    Ok(batch) => batch,
    Err(_) => unreachable!("reservation and control-side cancellation are exclusive"),
};
```
```rust
// crates/effect-contract/src/live.rs:19-22 — the frozen rule of the path that DOES render
// A drained record takes effect at the **first sample of the next rendered block** …
// emits `AutomationSpanKind::Point` spans whose `start_sample` and `end_sample` are that
// block's `first_sample`.
```
- **Problem:** Grepping `AutomationBatchSlot` / `AutomationRecord` across `crates/engine`, `crates/graph`, `crates/host-core`, `hosts/` returns nothing outside `crates/capi/{tests,src/runtime/tests}.rs`. The 32-byte record carries `start`, `end`, `Step`/`Linear`/`Exponential` kinds and a per-block density budget (`docs/CONTROL_PROTOCOL_SIZING.md`, "For the mandated 10,000-record fixture, `ceil(10000/256) = 40`") — all of it validated, stored, density-accounted, and then only ever cancelled. Meanwhile the path that reaches an effect (`EffectControlRecord::Parameter`) carries **no sample time at all** and is produced only by the wasm host. So the engine has two automation models: a sample-accurate one nothing renders, and a block-boundary one only the browser can drive. This is why `capi` needed IO-4's mock: there is no real endpoint for the queue to serve.
- **Proposed change:** Pick one and delete the other's dead half.
  - **Preferred:** make the protocol queue the source of truth. Add a render-side drain in `crates/host-core`: `fn drain_automation(&mut self, queues: &mut ProtocolQueues, first_sample: u64, frames: u32)` that pops batches whose `start` falls in `[first_sample, first_sample + frames)` and lowers each `AutomationRecord` to a `PreparedAutomationSpan` on the addressed instance's `EffectControlLane`, keeping the existing `stage()` staging window. The queue is already sorted by `(start, handle)` and density-bounded per block (`queue.rs:1116-1128`), so the drain is a bounded head-scan with no sort and no search: pop while `head.start < block_end`. Handle→(effect slot, parameter_index, channel) resolution is plan-invariant and belongs in the `PreparedRenderPlan` as a dense `Box<[(u32,u32,ParameterChannel)]>` built at prepare.
  - **Or:** if sample-accurate automation is out of scope for launch, delete `AutomationKind::{Step,Linear,Exponential}`, the density/interval tables (`queue.rs:648-663`, `:1143-1211`) and `AUTOMATION_ENQUEUE` entirely, and say so in `docs/CONTROL_PROTOCOL_SIZING.md`. That removes ~600 lines and the whole of IO-6.
- **Expected effect:** Either the C ABI gains sample-accurate automation, or ~600 lines of validated-but-unreachable queue machinery leaves the tree. Today neither is true.
- **Rulings / constraints checked:** The `#139/#140` "acked batch" invariant is intact today (see *Things I checked that are fine*), and the preferred design preserves it: the ack is still emitted only after `try_push` returns `Ok`. `AGENTS.md` render rules are respected — the drain is bounded by the per-block density budget, allocates nothing, and reuses the existing preallocated staging window.
- **Verification gate:** `cargo test -p protocol`, `cargo test -p host-core`, `scripts/check-realtime-policy.sh`, `scripts/run-protocol-allocation-audit.sh`, plus a new eval booting the mandated 10,000-record fixture and asserting the rendered PCM changes at the declared samples.



## Constraints (from #349, binding)

- Unfused `(a*b)+c` everywhere; bit identity across scalar/W4/W8/wasm. A class A row moves **no rendered bit**. If the row turns out to need one, stop, re-class it B on #349, and do not implement.
- No new crate names, no renames for their own sake, no runtime SIMD dispatch, no `unsafe` outside the existing allowlist without a ruling.
- Render path is zero alloc/free/lock/syscall/I/O/log/panic (`scripts/check-realtime-policy.sh`, `docs/REALTIME_DEPENDENCY_POLICY.md`).
- One measurement per row where the finding names one, recorded, not chased. No tuning loops.
- Line numbers above are from `main` `4797a544` and may have drifted; locate by the named functions. If the premise no longer holds on current `main`, close this issue with a one-paragraph null note and append the same note to #349.

## Done when

1. The finding's **Verification gate** above is green, with the command output attached to the PR.
2. `cargo test --workspace` passes with the same pass count as `main`, plus the named policy scripts.
3. A one-paragraph note on #349: row id, commit hash, what changed (or the null result).

Spec mirror: `.github/ISSUE_SPECS/370-io-5-automation-drain.md`.


---

## Reconciled execution brief (2026-09-04; supersedes conflicting assignment/procedure text above)

# Astra brief: #370 / IO-5 — correct automation delivery documentation

APPROVED smallest closable slice: exactly three documentation edits from the issue's 2026-09-04 owner rescope. Preserve the accepted-automation queue; no render drain, deletion, feature or protocol change. Reviewed main 87926988. Live #370 is OPEN and unclaimed at this inspection (audit/wave-0/model:qwen labels only). Root must recheck before claiming. No dependency: the doc correction is explicitly independent of #369; the separate future drain feature lands after #369. Current user assignment replaces Qwen/Fable with Astra brief/review, Luna first implementation, Sol retry on failure.

## Premise verified; recheck before editing

`rg -n LateAutomation crates/` currently has exactly two hits: crates/protocol/src/message_wire.rs:654 enum declaration and :2073 decoder. No increment. `try_dequeue_automation` production calls remain in ProtocolController::cancel_queued_automation_reserved; other protocol hits are tests, and tools/audit is qualification rather than delivery. REALTIME_MEMORY still attributes delivery to the accepted queue. CONTROL_PROTOCOL_SEMANTICS still describes late catch-up with no delivery-status qualification. queue.rs still has the single-line pop comment. This is not a null result.

## Exact changes

1. In docs/REALTIME_MEMORY.md replace the first complete sentence starting `Parameter and event delivery is owned by ...` and ending `the plan itself carries no parameter store.` with:

Live parameter delivery is owned by the per-effect `EffectControlLane` (#140), fed by the `EffectControlProducer`s prepared with the plan; the protocol crate's accepted-automation queue (#102) admits, retains and cancels sample-timed batches but has no render-side consumer yet (see `CONTROL_PROTOCOL_SEMANTICS.md`, "Delivery status"). The plan itself carries no parameter store.

Keep the subsequent parenthetical about #84 phase C and PlanEpoch unchanged. The original semicolon joins the two physical lines into ONE sentence; do not retain a duplicate `the plan itself carries no parameter store` fragment.

2. In docs/CONTROL_PROTOCOL_SEMANTICS.md insert immediately after the `Automation requires exact revision` paragraph:

**Delivery status (v1).** Accepted automation is retained in the queue and consumed today only by cancellation (`AUTOMATION_CANCELED`). No render-side drain exists: sample-accurate application of point, step, linear and exponential records is the later protocol capability named in `EFFECT_CONTRACT_V1.md` ("V1 runtime automation is `Point` spans …"). The catch-up rule and the late counter in the paragraph above are specified but not implemented; `LATE_AUTOMATION` is registered and nothing increments it.

The LateAutomation precheck is mandatory. If new code increments it, or a production render drain has appeared, stop and request a scope correction from root rather than publishing a false statement.

3. Expand the doc comment immediately above crates/protocol/src/queue.rs::try_dequeue_automation to:

```rust
    /// Pop one fixed batch without decoding or allocation.
    ///
    /// The only production consumer is `ProtocolController::cancel_queued_automation_reserved`;
    /// render-side delivery is deferred (see `docs/CONTROL_PROTOCOL_SEMANTICS.md`, "Delivery status").
```

No executable Rust changes. Preserve prose exactly as the owner supplied it; normal line wrapping is acceptable. No other doc correction or modernization.

## Exclusions and spec synchronization

Do not edit AGENTS.md, EFFECT_CONTRACT_V1.md, any ISSUE_SPECS/00* file, registry, hashes, fixtures, controller, wire declarations, benchmark scripts or tests. No allocation audit, benchmark, sample-accurate 10,000-record PCM eval or runtime gate is required by the rescoped verification contract; those belonged to the expressly superseded feature proposal.

Root must create the missing `.github/ISSUE_SPECS/370-io-5-automation-drain.md` mirror from the existing remote issue (do not create another numbered GitHub issue), append this reconciled scope, and synchronize the remote body/roles before implementation. The remote title and legacy mirror filename still describe a drain; preserve number identity and explain prominently that this is docs-only. Root may align the GitHub title/local spec heading to `Document the accepted-automation queue's delivery status` in the same scope checkpoint, retaining the known mirror path to avoid gratuitous filename churn. This is coordination metadata alongside the three product-doc edits, not permission for broader local-spec repair.

## Verification and delivery

- Read AGENTS.md and REALTIME_DEPENDENCY_POLICY.md once. New isolated worktree off synchronized origin/main, branch `codex/370-io-5-automation-docs`; root owns claim/worktree/commits/pushes/GitHub. Do not modify existing active worktrees.
- Save the pre-edit LateAutomation and dequeue call-site search, then rerun after edits to ensure claims remain true. Inspect every Rust diff hunk: doc comments only.
- Required commands: `cargo test -p protocol`; `cargo doc -p protocol --no-deps` with no new warnings; `cargo test --workspace` candidate and baseline with unchanged pass counts; `git diff --check`. PATH=/home/bl/.cargo/bin:$PATH. An already completed baseline workspace run may be reused ONLY if its exact commit remains this issue's base and its output is available. If main moves, root accounts for/rechecks the new base. Compare baseline rustdoc warnings if any rather than asserting their absence without evidence.
- No tests need adding for this reversible prose correction. Do not pin exact prose bytes in tests. Standard required CI must pass; unrelated failure stays explicit.
- Luna gets one coherent pass, then pauses at exact-path checkpoint notification. Root commits that result promptly; no second tranche while it waits. Astra reviews the actual PR before merge. If Luna fails, Sol gets at most two implementation attempts with one Astra verdict each. Three failed attempts triggers a rescope, never weakened gates.
- PR title starts IO-5 and says docs. State plainly that sample-timed protocol batches still do not render and that the actual live effect lane uses block-boundary point delivery. Do not claim this closes the drain feature. After upstream evidence and Astra PASS, root merges, verifies #370 CLOSED, and synchronizes #349 with row, merge hash and docs-only outcome. Remove claim on a recorded null if the premise was already corrected by another agent.


## Execution evidence — Luna attempt 1 (2026-09-04)

Implementation checkpoint `f2dcfc15` changes only the three specified documentation locations. Pre/post searches confirm exactly two `LateAutomation` references (enum and decoder) and no production render-side automation drain. No executable Rust or protocol shape changed.

- `cargo test -p protocol`: PASS, 132 passed, 0 failed.
- `cargo doc -p protocol --no-deps`: PASS, no warnings.
- `cargo test --workspace`: baseline `87926988` and candidate `f2dcfc15` both PASS, 1545 passed, 0 failed, 24 ignored; test-count delta 0.
- `cargo fmt --all -- --check` and `git diff --check`: PASS.
- Logs retained at `/tmp/engine-370-{precheck,postcheck,cargo-test,cargo-doc,workspace,fmt}.log`; baseline `/tmp/engine-368-baseline.log`.

Astra PR review and required remote qualification remain pending. This evidence proves documentation accuracy only; the separate drain feature remains deferred.
