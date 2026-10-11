PASS

# #1268 attempt 1 verdict (adversarial verifier)

Commit b3ddc8d86 (parent 7d5362dc2), branch codex/d15-batch-dfg. Exported with `git archive` to
/tmp/claude-1002/v1268/tree, built with CARGO_TARGET_DIR=/tmp/claude-1002/v1268/target.
Diff: `crates/builtins/tests/filter_liveness.rs` (+167, test only) and the spec (+93). No production
code changed, so every run on this commit is a run of the base's (7d5362dc2) production code.

## D2 applied correctly

- Gate 1 is green with no production change: the new test passes on the export (`filter_liveness`
  15 passed). Because the diff is test-only, this is the D1 measurement on the base.
- Both bodies are exercised: `Body::Dual` calls `BuiltinInputBank::process` with a `Left` target
  (dual ramp body, clear at `crates/lane/src/kernels/builtins.rs:1166-1167`); `Body::Mono` calls
  `process_mono` with a `Both` target (one-plane body, clear at `:1410-1411`). M1 (dual clear
  removed) and M2 (mono clear removed) each turn the test red at `filter_liveness.rs:1341` (lane 1
  state leg), so the test reaches the D3 defect on both bodies. Under M2 the dual leg runs green
  first and the mono leg fails, so the matching leg is red (gate 2).
- No production edit, no D3 path; D4 does not apply. Authorized paths respected.

## Spec corrections (all verified true)

- Anchors: the clears are at `kernels/builtins.rs:1166-1167` and `:1410-1411`,
  `section_is_identity` at `:1590`, `svf_step_when` at `crates/lane/src/kernels.rs:1093`. True.
- "No existing test reads the integrators or the plan after a disable" is false. True:
  `a_disable_re_sent_every_frame_completes_clears_and_elides_on_time`
  (`filter_liveness.rs:368`) reads `state_words`, `current_words` and the elision plan on the scalar
  chain and on lane 1 of a four-lane bank (dual body); `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths`
  (`crates/builtins/src/tests.rs:328`) reads the plan of a one-lane stage on both bodies.
- The signed-zero output claim does not hold. True, analytically and by measurement. An elided run
  of sections is replaced by `add(+0.0)` (`identity_chain_block`, `kernels/builtins.rs:1727`;
  `mixed_chain_block` doc), and a running identity section over `+0.0` integrators also maps `-0.0`
  to `+0.0`; so a stale identity HPF's `-0.0`/`+0.0` is normalized by the following LPF stage in
  both the bank and the twin. Measured: with the state and plan assertions removed (output legs
  only), the test is GREEN under M1 and under M2. The output leg is a lane-isolation guard, as the
  record says, not proof of the clear.

## Mutation runs (re-done by the verifier on the export; reverted; tree verified byte-equal after)

Suite = `cargo test --locked --no-fail-fast --all-targets -p builtins --features builtins/test-support`.

| Mutation | New test | Other builtins tests red |
|---|---|---|
| M1: dual clear lines removed | RED `:1341` (lane 1 state `[1015199171, 1016482006, 0...]`) | `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths`, `a_disable_in_the_enables_drain_still_completes_and_elides`, `a_disable_freezes_the_recursion_words_and_moves_only_the_mix`, `a_disable_re_sent_every_frame_completes_clears_and_elides_on_time`, `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs` |
| M1, output legs only | green | n/a |
| M2: mono clear lines removed | RED `:1341` | `tests::filter_prefix_ends_at_countdown_for_dual_and_mono_paths` only |
| M2, output legs only | green | n/a |
| M4b mono: `if L::mask_any(identity) { all lanes }` | RED `:1266` (neighbour lane output, 0 vs 1032851128) | **none** |
| M4b mono, output legs only | RED `:1266` | n/a |
| restored | green | none |

The implementer's table matches on every row I re-ran (M1, M2, M4b mono, restored). M4a and M4b
dual were not re-run (recorded as run on an earlier revision of the test).

## Test value

`a_settled_live_disable_is_re_elided_and_bit_identical_to_never_enabled`: a one-plane
(`process_mono`) ramp completion whose integrator clear is widened from the settled lane to every
lane of the bank (M4b mono, e.g. a scalar `mask_any` shortcut) wipes a neighbouring lane's live HPF
state, and this test is the only builtins test that turns red on it.

Is the largely duplicate remainder acceptable? Yes. The test has a real unique catch, so the
AGENTS.md test-value rule is met; the state and plan legs that older tests also cover are what D2
and gate 1 require, live inside the same test (no second test to supersede), and the record states
plainly that M1/M2 are not unique catches. Every test-value claim in the Attempt record is true as
far as re-run.

## Findings

No BLOCKER, no MAJOR.

- MINOR `crates/builtins/tests/filter_liveness.rs:1226-1229` (with the spec's Attempt record):
  gate 1's letter asks for "several blocks of negative samples" before the disable; the test uses a
  signed sawtooth (`history_input`, range about [-0.4, 0.4)). The deviation is harmless (the measured
  output-only runs show the history's sign cannot reach the output, and the state leg checks exact
  `+0.0` bits), but the Attempt record names only `neighbour` as a deviation from the body's letter.
  Record it.
- NIT `filter_liveness.rs:1331`: the assertion context is `format!("block {block}")` only; it omits
  the body, `neighbour` and `history_blocks`. M1 and M2 fail with the identical message "lane 1
  integrators are +0.0 after block 2", so the failing leg (which gate 2 and D3 ask about) is not
  readable from the failure. Add the body and scenario to `context`.
- NIT `filter_liveness.rs:1347`: in the `neighbour` scenario only `!plan[0][0]` is asserted; the
  right HPF's plan entry is not checked (dual: lane 3 is `Left`-addressed, so `plan[1][0]` should be
  elided). Small coverage gap, not required by the spec's letter.
- NIT spec Attempt record: the M4a/M4b-dual rows cite `:1265`, a line of the earlier revision; on
  the committed file the left-plane assertion is `:1266`. The record says those rows were run on the
  earlier revision, so this is cosmetic.
- Note for root (not a finding against the attempt): the spec's Context and Test value sections still
  carry the false "no existing test" and signed-zero statements; the implementer corrected them in
  the Decision record and left the body as filed, which is root's call.

## Gates run (on the export)

- `cargo test --locked --all-targets -p builtins --features builtins/test-support`: green (all
  binaries; `filter_liveness` 15 passed).
- `cargo test --locked -p builtins --features builtins/test-support --test filter_liveness a_settled_live`: green.
- `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`: green (33
  result lines ok, 0 failed).
- `bash scripts/check-workspace-policy.sh`, `check-builtins-policy.sh`, `check-lane-policy.sh`: exit 0.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: exit 0.
- `cargo fmt --all -- --check`: exit 0.
- Mutations M1, M2, M4b mono (+ output-only variants of each) as tabled above.
- D3-only gates not applicable (no production change). NEON is CI-only.
