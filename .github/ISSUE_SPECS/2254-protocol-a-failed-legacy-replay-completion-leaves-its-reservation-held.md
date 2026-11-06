# Protocol: a failed legacy replay completion leaves its reservation held, wedging every later request at Backpressure

Found by the implicit-state-machine audit of 2026-11-06. A defect on main at `8f46ab035`.

## Mission
`ReplayCache.reservation: Option<ReplayReservation>` (`crates/protocol/src/controller.rs`) was an implicit Idle -> Reserved -> Idle machine whose only exit was `complete`. On the legacy frame path (`process_command_frame_into_legacy`), an error after `ReplayDecision::Execute` -- the encode `?` returning `OutputTooSmall`, or `complete` refusing a reply over `max_response_bytes` -- returned with the reservation still `Some`. Every later new request then answered `BACKPRESSURE`, and structural planning routed to Legacy, for the endpoint's lifetime. The production C ABI and browser hosts reach this path through `prepare_command_frame_*` for every non-structural command (capacity `min(caller, max_response_bytes)`), so one command whose reply exceeds the bound (a large `TELEMETRY_CONFIGURE` echo, for example) wedged the endpoint.

`ReliableEventReservation(s)` (`crates/protocol/src/queue.rs`) checked the issuing queue, the generation and a multi-hold's remaining count only with `debug_assert`. Every queue carries the same generation (`QueueGeneration(4)`), so a hold committed to another queue passed even the debug checks whenever that queue held its own reservation: it took a unit from a count that never had it and left its own queue's count permanently raised.

## Invariants
- The replay cache holds no state between deciding a request and recording it. A request is new until its entry is recorded; nothing a caller drops or forgets can refuse later requests.
- A request the endpoint executed always gets a recorded reply, so a replay of its ID answers that reply and never executes it again (exactly-once execution per request ID).
- No reply exceeds the advertised `maximum_cached_response_bytes`.
- A reliable-event hold is committed or released only by the queue that issued it, and a multi-hold commits at most what it reserved, checked in every build before anything changes. The acked-batch question: an ack never precedes a drop -- no event is pushed into unreserved room, and no shared count wraps.

## Contract
- `ReplayCache::preflight(&self, ..)` is read-only and returns `ReplayDecision::Execute(ReplayAdmission)`. `ReplayAdmission` is an opaque plan: cache ID, the cache's completion count when planned, request ID, request length and digest, and the eviction to apply.
- `ReplayCache::complete(&mut self, admission, request, response)` checks every field before anything changes, then applies the eviction, stores request and response and advances the frontier in one step. It refuses an admission from another cache, one planned before a later completion (stale), or other request bytes (`ReservationMissing`), and a response over the bound (`ResponseTooLarge`).
- `TELEMETRY_CONFIGURE`, the one state-changing legacy command whose reply is sized by its request, measures its echo frame (the request with the provider's meter window, header included) after the `meter.strip.unknown` check and before the provider stores anything. An echo over `max_response_bytes` is refused `LIMIT_EXCEEDED` `telemetry.reply_too_large` at the unchanged revision through `telemetry_refusal_outcome`, and that refusal is recorded.
- Legacy frame path: the reply is encoded into `output[..min(output.len(), max_response_bytes)]`. A reply that does not fit falls back to `INTERNAL` with diagnostic `protocol.encode` at the session's current revision, and that reply is recorded. An endpoint whose bound cannot hold that fallback refuses a new request before dispatch with the uncached replay `BACKPRESSURE` reply (unaccepted), so it never executes a command it cannot answer.
- `process` (the owned path) already fell back to `INTERNAL` `protocol.encode` for an over-bound reply. It takes the same admission flow and the same pre-dispatch check.
- The planned (non-legacy) paths keep their behaviour. They already planned read-only and recorded at completion; they now call the one-step `complete` in place of `apply_preflight_plan` and `complete`.
- Queue: `commit_reliable_event`, `commit_reserved_reliable_event`, `release_reliable_event(s)` panic, before any change, when the hold was issued by another queue (`Arc` identity plus generation) or a multi-hold has no unit left. The decrement of the shared count is a checked `try_update`.

## Decision: the outcome of the request whose reply failed
The command has run, so whatever it changed stays changed: a session commit with its `SESSION_COMMITTED` event, a `TRANSPORT_STATE` event carrying the origin request ID, or a stored telemetry configuration. The options were:
1. **Release the reservation without recording.** Refused: the ID stays new, so the client's retry executes the command a second time (a duplicate commit or event). This breaks exactly-once execution.
2. **Return the true reply uncached and retire the ID** (later replays get `REPLAY_EXPIRED`). Refused: the reply would exceed the advertised `maximum_cached_response_bytes`, which caller outputs are sized to. In the production prepared path the reply cannot fit anyway.
3. **Record a bounded fallback reply (chosen as the catch-all).** The reply is `INTERNAL` `protocol.encode` (the code `encode_failure_outcome` already used; the registry does not list it). Its header revision is the revision after the command, so a commit is visible in the reply itself, and the reliable event stream carries the commit with its origin request ID. The reply does not claim `OK` and does not claim a refusal before execution. It is recorded, so a replay returns the same bytes and executes nothing. These are the bytes `process` already produced for an over-bound reply, so the two legacy paths now agree.

**Attempt 2: bound the reply before execution.** Option 3 alone left a conforming `TELEMETRY_CONFIGURE` with an oversized echo answering `INTERNAL` with its table stored, contrary to `docs/CONTROL_PROTOCOL_SEMANTICS.md` l.11 ("an unretainable response is rejected before state changes"). The echo is a conforming provider's persisted request plus its meter window, so its size is known before the provider runs; the controller refuses it there with the typed `telemetry.reply_too_large`. Option 3 stays as the catch-all, reachable only by a provider that breaks its persist-and-echo contract. Line 11 stays as written; the registry gains the controller's telemetry refusal rows and the semantics doc one clause in the refusal order.

The degenerate endpoint whose `max_response_bytes` cannot hold even that fallback cannot record any reply. It therefore refuses new requests before dispatch rather than executing them.

## Non-goals
- No wire, codec or corpus change. No change to the planned path's replies, ordering or replay entries.
- Docs change only to state the new refusal: two registry rows, one semantics clause, and the configure refusal lists in `crates/capi/include/miso_engine_v1.h` and `docs/C_ABI_V1_QUALIFICATION.md`.

## Hazards
- `docs/CONTROL_PROTOCOL_SEMANTICS.md` l.11 ("an unretainable response is rejected before state changes") now holds on the legacy path for conforming providers (attempt 2). A non-conforming provider whose echo differs from its request can still reach the recorded `INTERNAL` `protocol.encode` with its effect applied.
- An endpoint whose bound holds the `protocol.encode` fallback but not the longer `telemetry.reply_too_large` refusal answers an oversized configure with the recorded `INTERNAL` fallback, with nothing stored. Such a bound (under about 140 bytes) is degenerate; the refusal still changes nothing.
- `ReplayAdmission` is `Copy`, so it is not an affine type. Safety comes from the cache-side check: a second completion of the same admission is stale and refused.
- A queue misuse is a caller defect and now panics. On the C ABI that aborts (see `docs/REALTIME_DEPENDENCY_POLICY.md`). Every caller is in this crate, and each commits a hold it took from the same `self.queues`.

## Gates
- A regression test that is red at `8f46ab035` for the wedge and green after.
- No change to wire bytes or to the planned path's behaviour: the full `-p protocol` suite, including the byte-pinned corpus and the planned/one-call byte-identity tests, stays green.
- `cargo test --locked -p protocol` and `-p control-plane`; `cargo clippy --locked -p protocol --all-targets -- -D warnings`; `cargo fmt --check`; `scripts/check-workspace-policy.sh`.

## Evidence (attempt 1)
Base `8f46ab035`; branch `codex/issue-2254-replay-reservation`; implementation commit `cca676d30`.

**Red (tests written first, on base code):**
- `cargo test --locked -p protocol --lib an_executed_command_whose_reply` panicked: `the next new request is refused (Encode(OutputTooSmall { required: 240 })): wedged`. The second request is answered with a 240-byte `BACKPRESSURE` frame, the wedge, which does not even fit the 128-byte bound.
- `cargo test --locked -p protocol --lib a_reservation_committed_to_another_queue` panicked: `a foreign multi-event hold is refused`. The debug build accepted the foreign commit silently, because both queues carry generation 4 and the other queue's count was nonzero.

**Green (at `cca676d30`):** both tests pass. `cargo test --locked -p protocol`: 212 passed, 0 failed. `cargo test --locked -p control-plane`: 13 passed, 0 failed. `cargo clippy --locked -p protocol --all-targets -- -D warnings`: clean. `cargo fmt -p protocol -- --check`: clean. `scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**New tests and their value:**
- `controller::tests::an_executed_command_whose_reply_overflows_the_bound_is_recorded_and_wedges_nothing` (production `prepare_command_frame` path, bound 128, a 5-meter, 8-counter `TELEMETRY_CONFIGURE`). It turns red if any exit after execution leaves the cache refusing later requests (the wedge), or if a fix releases without recording so that the replay re-executes the configuration (`telemetry_generation` reaches 2). It also turns red if the reply exceeds the bound or claims `OK`. No existing test drives a legacy reply over the bound.
- `queue::tests::a_reservation_committed_to_another_queue_changes_neither_queue`. It turns red if commit stops checking the issuing queue (for example, a revert to the generation-only `debug_assert`, which every queue passes): a foreign commit then publishes into unreserved room and leaves the issuer's count raised for good. No existing test commits across queues.

**Rewritten tests (same claims, admission API):** `replay_prefix_derives_length_retained_bytes_and_highest_new_id` (the eviction now waits for completion), `fixed_replay_arena_returns_exact_hits_and_compacts_one_evicted_prefix`, `fixed_replay_arena_reports_response_and_reservation_bounds` (a completion with no reservation becomes another cache's admission), and `replay_admission_binds_exact_request_bytes_and_one_completion`. The last replaces `replay_reservation_binds_exact_request_bytes`: "a pending reservation cannot be overwritten" becomes "an admission planned before a later completion is refused", plus "an admission completes once". It turns red if `complete` skips the completion-count check and applies a stale eviction plan.

**Not covered by a debug-red test:** a multi-hold committed past its count was already caught by `debug_assert` in debug builds. The release-build `assert_ne!` that replaces it has no test that is red in debug.

## Evidence (attempt 2)
Base: attempt-1 HEAD `475db20c3`.

**Change:** `ProtocolController::telemetry_echo_refusal` (`crates/protocol/src/controller.rs`) measures the echo payload with `encoded_telemetry_configuration_len` (the success writer's own `write_telemetry_configuration`) against `response_payload_budget()` (the bound less the 48-byte outer header, as `fit_control_list` does) and is called in the `TelemetryConfigure` arm before `provider.telemetry_configure`. The post-execution fallback's comment says conforming providers cannot reach it.

**Tests (red on `475db20c3` with the attempt-2 tests applied):**
- `a_telemetry_echo_over_the_bound_is_refused_before_anything_changes` replaces `an_executed_command_whose_reply_overflows_the_bound_is_recorded_and_wedges_nothing`. Red at base: `left: Internal, right: LimitExceeded`. It asserts the refusal at revision 7, `telemetry_generation == 0`, the provider's stored table unchanged, a byte-equal replay that still applies nothing, and the next request served. It turns red if the bound check moves after the provider call (the table is stored) or is dropped (`INTERNAL`), or if the refusal is not recorded.
- `a_telemetry_echo_exactly_at_the_bound_executes`: the 392-byte echo at bound 392 is `OK`, generation 1; at 391 it is `LIMIT_EXCEEDED`, generation 0. Red at base (the 391 case answers `INTERNAL`). Mutation: comparing the payload against the whole bound (forgetting the header) turns it red; the over-bound test stays green under that mutation, so this test is the only one defending the header term.

**Verdict (attempt 2): FAIL.** The code is correct: the size check is exact, it runs on every path before any state changes, and the order matches the docs. The fail is test value. Attempt 2 deleted the only test that drove an executed legacy reply over the bound, so two mutations of the still-contracted `INTERNAL` catch-all survived the suite: the `OutputTooSmall` arm returning `Err(Internal)` unrecorded (M1), and dropping `.min(max_response_bytes)` from the reply slice (M5). The degenerate-bound pre-dispatch check that attempt 1 added was untested too (M3); the review first called it pre-existing, but it is not on `8f46ab035`. Minor: the C ABI header and qualification doc did not list the new refusal. Minor, accepted: restoring the request's meter window before the provider call has no test; the effect is invisible because the controller writes the echo's window after the provider returns.

## Evidence (attempt 3)
Base: attempt-2 HEAD `718467eb0`.

**Change:** test-only `MockProvider::echo_override` (a provider that breaks persist-and-echo by answering a different echo than it stored). The configure refusal lists in `crates/capi/include/miso_engine_v1.h` and `docs/C_ABI_V1_QUALIFICATION.md` gain `telemetry.reply_too_large`. No production code change.

**New tests, each checked against the reviewer's mutations (run, then restored):**
- `a_nonconforming_echo_over_the_bound_is_recorded_as_internal_and_executes_once` runs through both legacy entry points, the prepared path and the one-call path with a 4096-byte output. A small configure whose echo fits passes the guard; the provider answers the oversized echo. The test asserts the recorded `INTERNAL` `protocol.encode` at revision 7 within the bound, `telemetry_generation == 1` after a byte-equal replay, and that the next request is served. Red under M1 (`the executed command is answered: Internal`) and under M5 (one-call path, same message). It turns red if an over-bound reply after execution goes unrecorded, so a retry would re-execute.
  - A first draft padded the echo with duplicate meter entries. The codec refused that echo with a non-`OutputTooSmall` error, which takes the other fallback inside `encode_outcome_or_internal_into`, so M1 survived. The substitute echo is now a valid configuration.
- `a_bound_below_the_fallback_reply_refuses_before_dispatch`: bound 100, below the 120-byte fallback. A `TRANSPORT_SET`, whose own reply would fit, fails `OutputTooSmall` twice; the transport stays `Stopped` and no event is queued. Red under M3 (`attempt 0 is refused`). It turns red if the endpoint executes a command it could not record a reply for. The first draft used the oversized configure, which the new echo guard refused anyway, masking M3.

**Verdict (attempt 3): PASS.** M1, M3 and M5 turn red; M4 (the meter-window restore) survives, accepted as above. Two minors folded in the same attempt:
- The owned `process` path's pre-dispatch check (attempt 1) was untested (M3b survived). `a_bound_below_the_fallback_reply_refuses_before_dispatch` now also sends the `TRANSPORT_SET` through `process` twice. There the `BACKPRESSURE` refusal does not fit the bound either and falls back unrecorded, so the test asserts that the reply is not `OK`, the transport stays `Stopped` and no event is queued. With the check removed it is red (`owned attempt 0`, status `Ok`).
- Wording: the attempt-3 commit message calls the legacy pre-dispatch check "pre-existing". Attempt 1 introduced it.
