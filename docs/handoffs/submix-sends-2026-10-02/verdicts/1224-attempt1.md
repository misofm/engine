# #1224 attempt 1 verdict: Let a send follow its source strip's mute live in the browser

**Verdict: PASS.** There is no BLOCKER and no MAJOR. Four MINORs must be fixed before close:

- MINOR-1: an audible click in an edge case. A probe for it is red at head, and a verified patch is below.
- MINOR-2: an avoidable quadratic scan on the worklet thread. The same patch fixes it.
- MINOR-3: test gaps. Four plausible defects pass every committed test, and probes that turn them red are attached.
- MINOR-4: doc precision.

The NITs and INFO notes are optional.

- **Implementation:** `eff44271d` on parent `932f348a8`, branch `codex/batch-submix-k3`.
- **Review copy:** a `git archive` export of `eff44271d`. I never touched the worktree.
- **Host:** x86-64-v3 AVX2, AMD EPYC 7313P.
- **Probes:** `1224-attempt1-verifier-scratch.rs` in this directory. Append it to `hosts/host-web/src/tests.rs`. At head, every probe is green except `sol_p3_…`, which is red until MINOR-1 is fixed. `sol_cost_probe` is `#[ignore]`; run it with `--release --ignored --nocapture`.

## What holds up under probing

- **Semantics: settled follow-zeroed columns match a fresh plan.**
  - Probe C is a randomized differential: 12 seeds x 25 random batches of 1-3 records.
    - The records are kind 4 on any strip (tracks and buses) and any channel (L, R or both), and kind 9 on any track, at windows 0, 64 and 480.
    - The sends are pre-fader, post-fader, track to bus and bus to bus, plus a non-following input-tap send.
  - After every batch, two checks hold:
    - every mirror's `source_lane_muted` equals `[effective_mute(src,0), effective_mute(src,1)]`;
    - once settled, the output is bit-identical to a freshly booted host whose session mutes are those effective mutes.
  - 218 of the 300 steps had asymmetric lane mutes. The `malformed` guard never fired.
- **Named scenarios** (probe B), each settled state checked bit-identical against a booted host:
  - unmuting a strip while another track is soloed: no record, and the send stays silenced;
  - a solo switch in one batch (vocal off, drums on);
  - a send muted by `routeMute` and by follow, then `routeMute` cleared (still silenced), then un-soloed (reopens).
- **Ramp length.**
  - Gate 6 (delayed, 486 samples) and my P2 (two strips with different windows in one batch) both match explicit `routeMute` at the same window, block for block.
  - My P4 (a solo-driven follow at the solo's window) does too.
  - Settled follows leave no residue (gates 1, 4, 5 and probe C).
- **Atomicity: the acked-batch question.** No ack precedes a drop.
  - The follow records are staged into `command_decoded` and counted in `command_wanted`.
  - The room check covers every staged entry before any push. The ack follows the pushes.
  - The route consumer drains every record available at block entry, and the browser never swaps a plan.
  - `admit_commands` rolls back both mirrors on every `Err`, and commits them only on `Ok`.
  - I re-ran the implementer's I3 (commit right after the follow pass): red on gate 3.
- **Staging capacity.** `delta` yields each live route at most once, so `+ route_count` is the exact worst case: all tracks x all buses, every send following. The bridge accounting charges it, and the expected-resources budgets pass.
- **AGENTS.md.** The edit matches ruling D5 ("Decided, not landed": #1224 removes the qualifiers on the route-mute and follow-mute sentences).
  - Only the "Planned under decision 13 … a route has neither." sentence is gone, and the VCA qualifier stays.
  - #1216 (`f48fe7c74`) and #1218 (`0da034dba`) are ancestors of `eff44271d`, so the remaining sentence is true. It claims no live C ABI follow.
- **The three reviewable decisions.**
  1. **The `malformed` guard (a follow with no staged strip mute):** accept.
     - It is unreachable on this tree. After every admitted batch, the solo state's emitted mirror equals the effective mute, and so does the route mirror, so every lane change has a staged strip record. Probe C never tripped it.
     - `MALFORMED` is also the right result, not `RESULT_INTERNAL`: `admit_commands` skips `discard_owner` for `RESULT_INTERNAL`, which would leak a begun EQ owner on this pre-push refusal.
  2. **Refusing a window past 2^22 as `domain` instead of clamping:** accept.
     - D3 ties the follow ramp to the strip's window, and the design already chose refusal over clamping for route lengths (REVISION-1 row 1, P5).
     - 2^22 samples is 87 s at 48 kHz, so no real gesture hits it.
     - The refusal depends on state: it applies only when the change moves a following send. That has to be documented (MINOR-4).
  3. **The O(sends x staged) scan:** bounded and allocation-free, but needlessly quadratic in sends (MINOR-2).

## Findings

### MINOR-1: a no-op record on the other lane sets the follow ramp, so the send clicks

D3 takes "the last strip-mute record staged for that source strip". Kind 4 always stages a record, even one that changes nothing.

- **Trigger.** A batch carries `mute drums L @480` (changes) and `mute drums R=false @0` (a no-op on an unmuted lane).
- **Effect.** The follow record takes ramp 0. The send's left column steps to zero while the strip's left lane fades over 480 samples, which is an audible step on the return.
- **Measured.**
  - Probe P3 compares three hosts: X (the no-op at 0), Y (the no-op at 480) and Z (no no-op). The strip is bit-identical in all three, because Y equals Z.
  - X differs from Y at block 0, sample 0: 0.468 against 0.754.
  - P3 is red at head.
- **Fix.** Match only a record whose lanes cover a lane whose follow value changed, and amend the D3 wording to "the last strip-mute record staged for that source strip that covers a lane whose effective mute changed". The patch below makes P3 green, and every #1224 test and probe stays green.
  - The invariant still holds: every changed lane has a covering record, from kind 4 or from the coalescing pass.
  - When both lanes change through records with different windows, the last one still wins. One route ramp cannot do better.

```diff
             let fader_slot = (strip_count + entry.source_strip) as u32;
-            let Some((smoothing_samples, wire_index)) = ready.command_decoded[..lowered]
+            let changed = [
+                source_lane_muted[0] != entry.source_lane_muted[0],
+                source_lane_muted[1] != entry.source_lane_muted[1],
+            ];
+            let Some((smoothing_samples, wire_index)) = ready.command_decoded[..follow_start]
                 .iter()
                 .rev()
                 .find_map(|staged| match staged.kind {
                     StagedCommandKind::Command(AdmittedCommand::Fader(
                         TrackFaderRecord::Mute {
+                            lanes,
                             smoothing_samples, ..
                         },
-                    )) if staged.queue_slot == fader_slot => {
+                    )) if staged.queue_slot == fader_slot
+                        && match lanes {
+                            BuiltinLaneSelector::Left => changed[0],
+                            BuiltinLaneSelector::Right => changed[1],
+                            BuiltinLaneSelector::Both => changed[0] || changed[1],
+                        } => {
```

### MINOR-2: the ramp lookup rescans the follow records already appended

The scan runs over `command_decoded[..lowered]`, and `lowered` grows inside the follow loop. Each lookup therefore walks back over every follow record staged so far, none of which is a strip mute record. The cost is quadratic in the number of yielded sends, on the AudioWorklet thread (the worklet calls `command_submit` there).

| Session (all sends following) | Best solo submit at head | With `[..follow_start]` |
|---|---|---|
| 16 x 8 (128 sends) | 10 us | 5 us |
| 32 x 16 (512 sends) | 92 us | 26 us |
| 64 x 16 (1,024 sends) | **347 us** | **66 us** |

The table is `sol_cost_probe`, native release, best of 20. Wasm is slower again, against a 2.67 ms quantum.

- **Fix.** `[..follow_start]`, as in the MINOR-1 patch. It has no semantic effect, because follow records are never `Fader::Mute`, and every test stays green.
- **Record.** Correct decision 3's cost sentence. "At most 572 entries" describes the staging test, not a bound. The real bound is `sends x (2*256 + 2*strips + sends)` at head.

### MINOR-3: plausible defects that no committed test turns red

Each row was applied to the export, run against the #1224 tests and my probes, and reverted.

| Mutation | Committed #1224 tests | Probe that turns it red |
|---|---|---|
| M2: the ramp lookup ignores the source strip (any strip's last mute record) | all green | **P2** |
| M5: the follow record drops the send's own `mute` (`entry.mute` -> `false`) | all green | **P1**: a user-muted send reopens its right column on a one-lane source mute |
| M9: the follow record uses 0 dB, not the mirror's `gain_db` | all green | **P1** |
| The follow pass before coalescing, with the guard replaced by a ramp-0 fallback | all green | **P4** |

- **Adopt** P1, P2 and P4, and P3 once MINOR-1 is fixed. P1 also pins the matrix, because the edited matrix is in the booted reference. Without them, D1's "the mirror's gain_db, matrix and mute" and D3's per-strip and solo-window ramp have no audio gate.
- **Gate 1's test-value sentence overclaims.** It says gate 1 is red if the pass "runs before the solo coalescing pass and so reads the previous batch's mutes".
  - The solo state is already final when the batch loop ends. Ordering changes only the ramp lookup.
  - Gate 1 checks settled blocks only, so that row is red only through the `malformed` guard.
  - Reword the sentence (P4 is the audio gate for the ordering), and say in the deliverable-3(b) MUTATIONS row that the red is the guard's.

### MINOR-4: doc precision for a behaviour change (`docs/BUILTINS_AND_METERING_V1.md`, "Sends follow mute")

- **The 2^22 sentence.** "A strip mute whose window exceeds a send's longest ramp (`2^22` samples) is refused `domain`" reads as unconditional. State the condition:
  - It applies only when the change moves a following send.
  - A solo's window counts the same way, and is reported at the batch's first solo record.
  - The same kind 4 is admitted when no following send moves, as it was before this slice.
- **The backpressure sentence.** "At the wire index of the strip mute that asked for the record" is not always true. The room check names the first staged entry on the full queue, which can be an earlier send command (kind 13-15) on the same send. Say "the first record staged on that send's queue".

## NITs

1. **The overlong-solo refusal index** (`sol_d_…`). Take `[solo bass @100, solo vocal @2^22+1]`: it is refused at index 0, although the overlong window is at index 1. Coalesced records answer to `solo_first_wire_index`, which is the existing convention. Either document it or track the last solo record's index for the domain case.
2. **`route_base = strip_count * 3 + ready.effect_controls.len()`** respells `ReadyOwnership::route_slot`. Use `route_slot(route)` so the band arithmetic has one owner.
3. **"Metering and observation while soloed"** now repeats itself: "…which only a route into a submix may set (#1218). … and only a route into a submix can follow." Drop the second clause.

## INFO

- **A follow record on a send muted by its own switch changes no coefficient.** It goes from silenced to silenced, yet it reactivates an undelayed route for the ramp window and uses a queue slot. A full queue on such a send can therefore refuse a strip mute.
  - It is bit-neutral at the output (probe A: follow, a non-follow twin and a booted host are identical, and no `-0.0` appears).
  - It is compliant, because D2 defines "redundant" by `source_lane_muted`.
  - Consider "move the mirror lanes, stage nothing, while `entry.mute`" in #1226 or a later slice. That needs a D2 amendment.
- The ARTIFACT CHANGED digests reproduce exactly: shipped `e323e4b17ff9a76bb38ae94ec3a18657ff43b68aab4eb8deae0f9f2006552e45` (2,765,144 B), named twin `90d7d9cb…` (3,155,519 B). `command_submit` is now `closure=50` (was 48), with `LiveRouteState::follow` as a new trap owner on the control path. Render is unchanged (`closure=8 traps=5`, sole owner `render_inner`).

## Test value (one sentence each)

- `delta_yields_every_following_send_…`: red if `delta` stops after a strip's first following send, or yields an unchanged send or another strip's (H1, H3 red).
- `delta_follows_one_lane_at_a_time`: red if a lane maps to the other column, or a bus source loses the track offset (my M1, left lane only: red).
- `a_send_without_follow_never_follows_…`: red if a non-following send is yielded or followed, or `follow` escapes the shadow (H2, H4, H5).
- Gate 1 `soloing_a_track_silences_…`: red on the measured leak, when solo leaves followed pre-fader sends open (I1 red), but not on pass ordering (MINOR-3).
- Gate 2 `a_follow_never_pushes_a_redundant_send_record`: red if a settled send gets a redundant record or the mirror is not advanced.
- Gate 3 `a_full_send_queue_refuses_…`: red if strip mutes commit without their follows, or the window is clamped (I2 and I3 red).
- Gate 4 `a_one_lane_mute_follows_…`: red on a swapped or duplicated lane (M1 red).
- Gate 5 `muting_a_bus_silences_its_followed_send`: red if follow reads only tracks' mutes, or loses the bus offset (M1 red).
- Gate 6 `a_delayed_send_follows_like_an_explicit_send_mute`: red if a delayed send is deactivated or its ramp is the first record's, 0, or stale (I1 red).
- Gate 9 `follow_records_admit_and_render_without_allocating`: red if the follow pass allocates on the worklet path.
- D4 `the_decode_staging_holds_…`: red if staging is not grown by the live-send count.

## Mutations

| Mutation | Result |
|---|---|
| Mine. M1: `delta` compares lane 0 only | RED: gates 4 and 5, my C, and all three host-core unit tests |
| Mine. M2: the ramp lookup ignores the source strip | green on every committed test; RED on P2 |
| Mine. M5: the follow record drops the send's own mute | green on every committed test; RED on P1 |
| Mine. M9: the follow record uses gain 0 dB | green on every committed test; RED on P1 |
| Mine. Follow pass before coalescing, plus a ramp-0 fallback | green on every committed test; RED on P4 |
| The implementer's I1: no follow pass | RED on 12 tests (gates 1-6 and 9, staging, and my B, C, P1, P2) |
| The implementer's: clamp the ramp to 2^22 | RED: gate 3 |
| The implementer's: `routes.commit()` after the follow pass | RED: gate 3 |
| Proposed fix (MINOR-1 and MINOR-2 patch) | all #1224 tests and probes green, P3 included |

## Gates re-run (export of `eff44271d`)

| Gate | Result |
|---|---|
| `build-web-audioworklet.sh --named-twin <B> <A>` | rc 0. Digests as recorded. |
| `check-web-audioworklet.sh <A> <B>/…named.wasm` | rc 0. Kernels 13, `f32x4_arith=9396`. Render `closure=8 traps=5`. |
| `check-browser-expected-resources.py --artifacts <A>` | rc 0. Rows agree, 32 red self-test mutations. |
| `check-sdk-headless.sh <A>` / `test-web-audioworklet.sh` | rc 0 / rc 0 |
| `check-web-audioworklet-v8-spill.py` on the named twin | rc 0 |
| CI browser legs: `npm run qualify -- --artifacts <A> --sdk-root sdk --browser {chromium,firefox,webkit} --check-matrix --self-test-mutations`, private PulseAudio null sink, **SDK source bundle (CI's mode)** | rc 0 on all three. Chromium 151.0.7922.34, Firefox 153.0, WebKit 26.5. |
| `check-sdk-generated.sh <A>` / `check-sdk-types.sh` / `sdk-package.sh check <A>` | rc 0 / rc 0 / rc 0 |
| test-debug-a (DESIGN 7, `--no-fail-fast`) | rc 0. **1,233 passed, 0 failed, 9 ignored** over 108 binaries, as recorded. |
| host-core policy check and test; realtime policy check and test; workspace policy check and test | all ok. Realtime reports 54 regions in 15 files. |
| `cargo fmt --all -- --check` | rc 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | rc 0. Re-run on the pristine tree. |
| `run-aarch64-tests.sh debug` | Not run: there is no arm64 host. Left to CI's `aarch64-debug` at the K3 push. host-web is not in it. |
