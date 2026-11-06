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
- Legacy frame path: the reply is encoded into `output[..min(output.len(), max_response_bytes)]`. A reply that does not fit falls back to `INTERNAL` with diagnostic `protocol.encode` at the session's current revision, and that reply is recorded. An endpoint whose bound cannot hold that fallback refuses a new request before dispatch with the uncached replay `BACKPRESSURE` reply (unaccepted), so it never executes a command it cannot answer.
- `process` (the owned path) already fell back to `INTERNAL` `protocol.encode` for an over-bound reply. It takes the same admission flow and the same pre-dispatch check.
- The planned (non-legacy) paths keep their behaviour. They already planned read-only and recorded at completion; they now call the one-step `complete` in place of `apply_preflight_plan` and `complete`.
- Queue: `commit_reliable_event`, `commit_reserved_reliable_event`, `release_reliable_event(s)` panic, before any change, when the hold was issued by another queue (`Arc` identity plus generation) or a multi-hold has no unit left. The decrement of the shared count is a checked `try_update`.

## Decision: the outcome of the request whose reply failed
The command has run, so whatever it changed stays changed: a session commit with its `SESSION_COMMITTED` event, a `TRANSPORT_STATE` event carrying the origin request ID, or a stored telemetry configuration. The options were:
1. **Release the reservation without recording.** Refused: the ID stays new, so the client's retry executes the command a second time (a duplicate commit or event). This breaks exactly-once execution.
2. **Return the true reply uncached and retire the ID** (later replays get `REPLAY_EXPIRED`). Refused: the reply would exceed the advertised `maximum_cached_response_bytes`, which caller outputs are sized to. In the production prepared path the reply cannot fit anyway.
3. **Record a bounded fallback reply (chosen).** The reply is `INTERNAL` `protocol.encode`, which the registry defines as "failed after it was admitted". Its header revision is the revision after the command, so a commit is visible in the reply itself, and the reliable event stream carries the commit with its origin request ID. The reply does not claim `OK` and does not claim a refusal before execution. It is recorded, so a replay returns the same bytes and executes nothing. These are the bytes `process` already produced for an over-bound reply, so the two legacy paths now agree. No new diagnostic code is introduced, so the protocol registry and the wire stay unchanged.

The degenerate endpoint whose `max_response_bytes` cannot hold even that fallback cannot record any reply. It therefore refuses new requests before dispatch rather than executing them.

## Non-goals
- No wire, codec or corpus change. No change to the planned path's replies, ordering or replay entries.
- Not measuring legacy replies before execution. A `TELEMETRY_CONFIGURE` echo is known only after the provider answers (see Hazards).
- No change to `docs/` (out of this issue's file set).

## Hazards
- `docs/CONTROL_PROTOCOL_SEMANTICS.md` l.11 says "an unretainable response is rejected before state changes". The planned structural path does that. The legacy path cannot for replies sized only after execution (the `TELEMETRY_CONFIGURE` echo), so such a command now answers the recorded `INTERNAL` `protocol.encode` with its effect applied. Before this change it wedged the endpoint. A follow-up should either bound the echo before the provider applies it or restate the doc sentence. Left open.
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
