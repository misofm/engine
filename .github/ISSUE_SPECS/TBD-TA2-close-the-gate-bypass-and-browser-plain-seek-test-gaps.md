# Close the gate bypass and browser plain-seek test gaps

Owner-approved, relayed by root on 2026-10-18, from the test audit (mutation pilot, stage 1).
Filed by agent G. Test-only: no product code changes. Code anchors at `81a3074fc`.

## Product outcome

Two tests that the mutation pilot found weak now assert what their names claim:

1. The gate/expander bypass test renders an input the gate acts on, so a bypass that does not
   reach the kernel turns it red.
2. A host-web test checks the host's bookkeeping of the successor a plain session seek publishes
   (root ruling B16-R9), which audio alone cannot show.

## Context

- Gap 1 (pilot rows s1o-m1, s1o-m2). `contract::bypass_is_exact_identity_at_all_rates`
  (`crates/gate-expander/tests/contract.rs`) rendered 3 samples at default values. Preparation
  seeds every lane open with the full hold (100 ms at defaults), so the gain stays 0 dB and the
  output is dry through the `gain_db == 0` branch whatever the bypass word says. Mutants: the
  kernel ignores the bypass word (`kernel.rs:255`, s1o-m1) and the prepared gate drops the bypass
  flag (`lib.rs:473`, s1o-m2) left it green. s1o-m3 (bypass mutes) was already red.
- Gap 2 (pilot row s1e-m2). `hosts/host-web/src/lib.rs:1721`: the plain form of
  `ReadyHost::session_seek` runs through `run_control`. A mutant that sends it straight to the
  control plane leaves the successor with no `PendingShape`, does not count it in
  `blocking_rebuilds`, skips the table-room check and does not name the successor's meter header.
  The first command submission before the swap then fails the host
  (`web.internal.pending_shape`). Audio and latency are unchanged, so no host-web test saw it
  (`control_parity::a_plain_session_seek_declares_and_an_anchored_one_does_not_alike_on_both_hosts`
  checks result verdicts, bits and latency, not the host's table bookkeeping).

## Decisions

- D1. Gap 1 rewrites the old test in place (it supersedes it): the active values (`T = -20`,
  ratio 20, range 48 dB, hold 0 ms) over a 4,800-frame -60 dBFS plateau with a signed zero at
  frame 0, both channels. It asserts the unbypassed render is not dry (no vacuous pass) and the
  bypassed render is bit-exact dry, at 44.1, 48, 88.2 and 96 kHz.
- D2. Gap 2 adds `tests::structural_apply::a_plain_session_seek_keeps_and_counts_its_successor_on_the_browser`
  beside `a_declared_discontinuity_resets_the_floors_on_the_browser`, from the same #1285 floors.
  It mirrors the C ABI's `runtime::session_seek_declares::an_accepted_plain_session_seek_publishes_one_successor_and_applies`
  (`crates/capi/src/runtime/tests.rs`): both the swapped and the still-waiting removal.
  Assertions: one successor epoch; `blocking_rebuilds` up by one; exactly one unadopted
  `PendingShape`, at the successor's epoch (no superseded candidate's left); the rendering plan
  names that entry's meter header; `submit_commands(0)` is `RESULT_OK` and the host stays ready;
  the next block adopts the successor.
- D3. The table-room backpressure branch is not tested. `PENDING_SHAPES` is 4, `run_control`
  reclaims adopted entries first and drops superseded ones, and the core holds at most one
  waiting candidate, so no public call sequence fills the table before a plain seek. The branch
  is defensive; no test is added for it.
- D4. host-core has no host-side table bookkeeping to mirror. The C ABI has the equivalent
  publication test (D2); it keeps no tables, so it needs nothing more.

## Authorized paths

- `crates/gate-expander/tests/contract.rs`
- `hosts/host-web/src/tests.rs`
- this spec

## Objective gates

- `cargo fmt --all --check`
- `cargo clippy --locked -p gate-expander --all-targets --all-features -- -D warnings`, and the same
  for `-p host-web`
- `cargo test --locked --all-features -p gate-expander`, and the same for `-p host-web`
- `scripts/check-workspace-policy.sh`

## Evidence (agent G, 2026-10-18)

Mutants are the pilot's exact old/new texts, applied one at a time with a revert-on-exit driver;
the tree was clean after each.

| Test | Branch | s1o-m1 | s1o-m2 | s1o-m3 | s1e-m2 |
|---|---|---|---|---|---|
| gate-expander `contract::bypass_is_exact_identity_at_all_rates` | green | red | red | red | - |
| host-web `tests::structural_apply::a_plain_session_seek_keeps_and_counts_its_successor_on_the_browser` | green | - | - | - | red |

- Gap 1: all three mutants fail at the bypassed-left bit comparison.
- Gap 2: the mutant fails first at the `blocking_rebuilds` assertion. With that assertion and the
  table checks removed in a scratch probe, the mutant fails at the `submit_commands(0)`
  assertion, so the submission check is red on its own.

## Test value

- Gap 1: red if bypass does not reach the gate kernel (the kernel ignores the bypass word, or
  preparation drops the flag) while the gate is acting, which no other gate-expander test caught
  in the pilot (only rack's `bypass_shunt_identity` tests caught the dropped flag).
- Gap 2: red if the browser's plain session seek publishes its successor without the host's plan
  bookkeeping, so the successor is uncounted, unnamed and has no tables and the next submission
  before its swap fails the host, which no host-web test catches because the audio is unchanged.

## Dependencies

None.
