Verdict: PASS

# #1257 attempt 1 -- Sol adversarial verdict

Commit under review: `ef5ac335c` (parent `f02d09b24`), worktree `/home/bl/misofm/wt-1053`, reviewed
from the export `/tmp/claude-1002/v1257-a1` (built with `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1053`).
Held against the slice spec (D1-D8, gates 1-7, hazards), umbrella #1053 D1-D10, AGENTS.md realtime
rules and the acked-batch question.

No BLOCKER and no MAJOR. Three MINOR and six NIT findings, below. The implementation is correct by
inspection and by the gates. Every MINOR is a gap in test or documentation coverage, not a defect.

## The acked-batch question: can an ack precede a drop?

No. `commit_live` (control.rs:949-1043) runs these steps in #1053 D6's order: classify, admission,
resolve, room on every touched queue, `check_prepared_structural`, push, commit, respond. Nothing
is mutated before the last fallible check:

- `Producer::available_capacity` reads the real consumer index (engine `spsc.rs:328-337`), so the
  room it reports can only grow. A `try_push` after the check cannot fail.
- The structural generation has one outstanding affine token. Under the same `&mut self`, the
  commit cannot fail after the predicate passes.
- The records go only to the newest epoch: the pending candidate, else the current provider. That
  is the plan that is rendering, or the one the next render swaps in. I traced every interleaving
  of the swap, retirement push, reclaim and atomic publication against `synchronize_plan_epochs`.
  The live arm never targets a plan that will not render.
- A plan that retires with records still queued is replaced by one prepared from the committed
  model, which already holds those values (D9).
- The classifier refuses every value the setters refuse, so render applies every record it pops.

The inverse hazard (a record pushed before a refused commit) is also prevented by the code, but
only half of its guard is tested (MINOR 1).

## Judgments on the implementer's notes and the caller's checks

- **`CommandError::LiveBackpressure` and D10.** It is not a new result code or event. It maps to
  the existing `RESULT_BACKPRESSURE` (ffi.rs:632-638). `control.live.backpressure` is a last-error
  diagnostic string, as `control.plan.backpressure` is, and the slice's D8 authorizes it. No
  registry of these strings exists. `check-capi-abi.sh` passes, so there is no new symbol.
- **The epoch-lag reorder.** For a rebuild during the lag, `BUFFER_TOO_SMALL` now precedes
  `BACKPRESSURE`. The slice's D1 mandates this order (classify after the response-size check, lag
  check after classify). I found no other change to the rebuild path:
  - Classification is pure control-thread work and changes no state.
  - `Err` hands back the untouched token.
  - The fault phases, admission, pending check, reservation, commit, catalog replacement and
    publication are byte-for-byte as before.
  - No existing rebuild test was silently moved to the live arm. Every existing structural test
    uses `SetSessionId`, `SetSourceContent` or remove/put edits, which are non-live.
  - Value-only edits now succeed while a candidate is pending or during the lag, and an
    identical-value transaction is live with zero records. D1 and D7 intend both.
- **Events.** The live arm commits through the same `commit_prepared_structural`
  (controller.rs:1974-2025), so `SESSION_COMMITTED` and `AUTOMATION_CANCELED` (reason
  `RevisionChanged`, at `provider.current_sample()`) are identical to a rebuild's. No test asserts
  them positively (NIT 3).
- **Concurrency.** A push against a rendering plan lands this block or the next: the drains are
  bounded at entry (#1253), and the fader and mute records of one strip can split across a block.
  The header (miso_engine_v1.h:34-49) and the qualification doc (C_ABI_V1_QUALIFICATION.md:213-255)
  both state the one-quantum skew. The doc says outright "There is no block-atomicity claim". No
  block-atomicity claim appears anywhere.
- **Plan retirement.** Records die with the retiring plan, and the next rebuild starts from the
  committed model. Gate 2 defends this (M9b).
- **The out-of-path `test_last_error` hook** (ffi.rs:1029-1036). It is justified: the realtime
  policy confines `unsafe` in capi src to ffi.rs (check-realtime-policy.sh:28-29), and no existing
  test reader of the last error existed. The attempt record records it (NIT 1).
- **The stateless bus in the submix test.** This is legitimate. With a stateful bus the bus filter
  memory differs forever between a live run and a fresh plan. M3b proves the test still catches
  the defect it targets.
- **Gate 3c.** I found and ran a capi-side mutation that turns it uniquely red (NIT 2).

## Findings

### MINOR 1 -- The room check's matrix term and its fader record count are untested

**Where.** The room check is control.rs:990-998. The tests are live_tests.rs:575-603 (gate 3a)
and :607-636 (gate 3b).

**What is untested.** Gates 3a and 3b fill only fader lanes, with single-record edits. I ran two
mutations, and the whole `cargo test -p capi` suite (46 + 11) stayed green under each:

- I dropped the matrix term (`|| producer.producer.available_capacity() < ...` became `|| false`).
- I made the fader term need one slot only (`count()` became `count().min(1)`).

Under either defect, a transaction that overflows a lane panics at `unreachable!` midway through
the push loop. Records already pushed would then play while the token stays uncommitted. That is
the hazard the slice names: "a record pushed before a refused commit". #1258's gates do not cover
it either.

**Fix.**
- In gate 3a, also fill one track's matrix lane with 16 pan edits. Assert that the 17th returns
  `BACKPRESSURE` / `control.live.backpressure` with the refusal state and the rooms unchanged.
- Add one two-record edit (`left_db != right_db`) against a fader lane with room 1. Assert
  `BACKPRESSURE` and that the room stays 1.

### MINOR 2 -- No test for the slice's first hazard (the live arm must skip the epoch-lag check)

**Where.** control.rs:803-814.

**What is untested.** I moved the lag check above `commit_live`, and every capi test stayed green.
The attempt record says the skipped lag check is #1258's. But #1258's race gate retries every
`control.plan.backpressure`, so it would not catch this either.

**Fix.** Add a deterministic in-window live edit modeled on
`control_calls_inside_a_plan_swapping_render_call_keep_replacement_live` (tests.rs:852-932). After
the swapping `render_contiguous` and before the atomic publishes, a value-only `SetTrackFader`
must:
- return `Ok`;
- prepare no candidate;
- take one record from the promoted provider's fader room.

Correct the attempt-record sentence.

### MINOR 3 -- The new `capi_retained_bytes` growth is not documented

**What changed.** `ProviderEpoch` gains `CapiResources` (32 bytes). Every session's
`capi_retained_bytes` grows by 96 bytes: the inline current epoch in `Session`, plus the two
reserved `ProviderEpoch` slots. On the nine-track reference I measured 258,231 bytes, against
#1256's documented 258,135. A caller whose `maximum_capi_retained_bytes` was exact is now refused
at compile.

**Where.** #1256's section of `docs/C_ABI_V1_QUALIFICATION.md` (:203-209) documents its own
movement and tells exact-cap callers to raise their caps. #1257's section (:213-255) says nothing.

**Fix.** Add one sentence to the #1257 section: +96 bytes of `capi_retained_bytes` per session,
258,135 -> 258,231 on the reference; exact-cap callers raise `maximum_capi_retained_bytes`. The
oracles needed no edit, which matches gate 6's "change only by `ProviderEpoch`'s new field".

### NIT 1 -- The `test_last_error` hook is outside the authorized paths

It is a justified minimum change (see above) and already recorded. No action is needed beyond
keeping the record.

### NIT 2 -- Gate 3c's test value and mutation should be recorded

The attempt record says no capi-side mutation reaches gate 3c. That is wrong for a live response
that diverges from the committed frame. I made the live arm flip the last response byte after
`committed.write_into`. Gate 3c went red (live_tests.rs:652), and nothing else did.

A protocol mutation that skips installing the prospective replay (controller.rs:2010) also turns
it red, but 2 protocol tests and 3 capi tests catch that too, so it is not unique.

Record the response-byte mutation as gate 3c's test value. It is a plausible defect: owner
question Q3 is about putting a live/rebuild flag in the response.

### NIT 3 -- No live test asserts the events a live edit emits

`Rig::drain_reliable` (live_tests.rs:166-174) drains the reliable lane without inspecting it. D10
holds by construction, because the live arm uses the same commit. Still, one positive assertion
would pin it against a future live-only commit path:
- exactly one `SESSION_COMMITTED` per live edit;
- one `AUTOMATION_CANCELED` when a batch is queued.

### NIT 4 -- Header wording

In miso_engine_v1.h:37-41, "Any other change replaces the plan exactly as before: ... a fader
outside its domain (refused as ...COMPILE_REJECTED ...)" reads as if a refused edit replaces the
plan. Reword it to "takes the replacement path exactly as before".

### NIT 5 -- Evidence hygiene

The implementer's surviving `gates-summary.log` (`/tmp/claude-1002/w1257-a1/`, 04:42) shows
`fmt exit=1` and `cross-targets exit=101`. Both came from a tree with a debug `eprintln!` loop in
live_tests.rs:943. No passing run on the final tree was preserved. I re-ran both on `ef5ac335c`,
and both pass (below), so the claims hold. Keep the final-tree logs next time.

### NIT 6 -- Gate 1(a)'s mute window does not prove its own non-vacuity

**Where.** live_tests.rs:388-410.

**What is missing.** The mute check asserts exact `+0.0` from E + K on. The spec says "In every
compared window, the reference output must be non-zero; assert it." Here `original` is stepped
through those same blocks (`while original.block < rig.block`, :408), but its output is discarded.
Signal is only asserted later, in (b)'s window.

**Fix.** Collect the reference blocks for the mute window and assert that each is non-zero. The
source is continuous and non-zero, and (b) shows signal right afterwards, so the risk is low.

## Test value, one sentence per new test

1. `live_fader_mute_and_pan_edits_change_the_running_plan_bit_exactly`: red if a value-only edit
   still rebuilds, a record lands on the wrong track or lane, or the live value differs from what
   preparation bakes (M1, M3).
2. `a_live_track_edit_beside_a_submix_strip_reaches_its_track`: red if strip resolution confuses
   the tracks-then-submixes producer table, for example index + 1 resolving eq9 to `bus` (M3b).
3. `a_live_mute_survives_a_later_rebuild`: red if a live edit is acked without reaching the
   committed model, so a later rebuild reverts it (M9b).
4. `a_full_live_lane_refuses_before_anything_changes`: red if the fader room check is missing or
   off by one, or if the refusal carries `control.plan.backpressure` (M5, M6, M7). It does not
   cover the count or matrix terms (MINOR 1).
5. `a_live_transaction_with_one_full_lane_pushes_to_no_lane`: red if a strip's records are pushed
   as soon as its own room checks, before the other strips' rooms (M4; I re-ran it: red at
   live_tests.rs:634).
6. `a_replayed_live_edit_pushes_nothing`: red if the live arm answers with bytes other than the
   committed frame that the replay cache serves (my mutation: red at :652, unique).
7. `a_live_edit_while_a_candidate_is_pending_reaches_the_candidate`: red if a live edit goes to the
   current plan instead of the pending candidate (M2; I re-ran it: red at :691).
8. `deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing`: red if capi feeds the
   classifier the wrong pair of models, bypassing G1, G2 or G3 (M9), or if a domain failure
   reaches a queue.
9. `a_fault_before_the_live_push_leaves_every_queue_and_the_model_alone`: red if a push moves above
   the protocol predicate, the last fallible check (M8).
10. `the_live_admission_accepts_each_cap_and_refuses_one_byte_below`: red if any #1053 D8 term is
    missing or charged against the wrong plan (M10-M14).

No test greps source or prose. There are no digest or byte pins: gate 5's exact caps are synthetic
inputs to a pure function, and the PCM comparisons are run against live references.

## Gates I re-ran (export of `ef5ac335c`)

| Gate | Result |
|---|---|
| `cargo test --locked -p capi` | 46 + 11 + 0 passed |
| `cargo test --locked -p protocol --features test-support` | 130 + 7 + 1 + 3 + 3 + 2 = 146 passed |
| `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | pass |
| `audit capi` | 100,000 calls; 0 allocations, deallocations, locks and syscalls; `total_violations` 0 |
| `check-capi-abi.sh` (on the release set) and `--self-test` | pass |
| `check-scalar-oracle-absent.py --native target/release/libcapi.so` | pass (3,709 symbols) |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked -p capi -p protocol --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p capi -p protocol --no-deps` | pass |
| host-core, realtime, workspace and protocol-control policy checks + self-tests | pass |
| `check-cross-targets.sh` | PASS. Re-run because the implementer's log showed exit 101 from a debug tree; iOS `memset_pattern16` rows are the expected #1018 failures |
| `resource_lifecycle` budgets | `capi_retained_bytes` 258,231 of 282,432; `builtin_retained_payload_bytes` 28,521 of 31,424 |

Not re-run, by the lean instruction:
- The workspace test command. The implementer logged 1,324 passed and 0 failed. The protocol
  change is a pure extraction, so I had no doubt.
- Workspace-wide clippy and doc. Clippy and doc did run for capi and protocol.
- The worklet chain. capi and protocol are not in the browser module.
- AArch64. It runs in CI only.

## Mutations I ran (all reverted; the export matched `ef5ac335c` afterwards)

- **M2** (push to the current provider): gate 3d red. The implementer's claim is confirmed.
- **M4** (push as each strip checks): gate 3b red. The implementer's claim is confirmed.
- **Live response byte flipped**: gate 3c red, and nothing else. This is new test value for 3c.
- **Prospective replay not installed** (protocol): gate 3c red, plus 3 capi and 2 protocol tests.
  The catch is not unique.
- **Matrix room term dropped**: every test green. This is MINOR 1.
- **Fader room term needs one slot**: every test green. This is MINOR 1.
- **Epoch-lag check moved above the live arm**: every test green. This is MINOR 2.

## Paths

Every change sits inside the authorized paths, except `ffi.rs`'s `#[cfg(test)] test_last_error`
hook. That deviation is justified and recorded (NIT 1). The header diff is comment-only.
`tests.rs` changes visibility only. `controller.rs` changes `check_prepared_structural` only.
`resource_lifecycle.rs` is untouched.
