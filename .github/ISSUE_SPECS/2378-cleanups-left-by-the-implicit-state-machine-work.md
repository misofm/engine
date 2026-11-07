# Cleanups left by the implicit-state-machine work

Successor bundle filed when the implicit-state-machine tracker #2258 closed. These are small removals and pairings that the reviews recorded. None is a reachable defect today, except possibly C9, which is verified first. Each row is independent; split freely, and drop a row that turns out to be already done.

| # | Area | Cleanup | Origin |
|---|---|---|---|
| C1 | effects | Merge the limiter's private `SilentClaim` into `effect_runtime::silent`. | #2320 |
| C2 | effects | Pair `lane_default_bytes` with its validation at the type level, and make the limiter validator tie `history[k]` to its main ring. | #2324 |
| C3 | effects | `parse_lane` in dynamic-eq and de-esser still admits `-0.0` loosely, and the compressor and transient-shaper param-ramp restore sites lack the at-target clause. Tighten them to the reachable words. | #2316 |
| C4 | graph / rack | A trait-level `Drained` token, so every effect that must hold a drain carries it. | #2326 |
| C5 | engine | Remove `read_into` (with the tap's `Option` and its `Default`). | #2307 |
| C6 | control-plane | Remove the dead multi-hold API (`ReliableEventReservations` and friends in `queue.rs`). Make `compile_children_warm` reachable, or gate it with `test-support`. | #2330, #2348 |
| C7 | control-plane | The solo path clones the committed compilation only so `compiled()` can lend it, and the controller's `if !monitor_only` is a second commit decision. Fold both into the transaction form. | #2286 |
| C8 | graph-compiler | The banked member processor kept on the `compile_with_builtins` path is dead weight. | #2303 |
| C9 | control-plane | Verify first: `materialize_prepared` may leave a sessionless token on a store error. If it does, this is a defect: give it a red test and its own commit. | #2330 |
| C10 | host-core | The `prepare.rs` doc link to the feature-gated `crate::warm` breaks `cargo doc -p host-core` without `control-provider`. | pre-existing |
| C11 | tooling | The call-graph gate's Ms2 and Ms3 impl-index pins couple source order to the gate. Key them by name. | #2332 |
| C12 | web/sdk | One generated source for both MSB1 seek-case consumers (a packaging change). | #2282 |
| C13 | engine | Derive `MeterDelivery`'s `gate_from`, and fix the `OutputPeakGate::activate` doc, when `meters.rs` is next touched. | HC-7 |

## Gates
- A removal or refactor needs no new test and moves no bit: the no-bit-moved comparison and the downstream crates (graph-compiler fixtures, host-core) are PR evidence.
- C9, if confirmed, follows the defect rule: a test that is red on base.
- `scripts/check-workspace-policy.sh` and the call-graph gate are green.

## Evidence / attempt record
(empty)
