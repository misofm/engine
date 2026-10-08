# Attribute each claim's revisions exactly in the watermark and share one seqlock

Stream B follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-17). Filed
2026-10-08 by root from the #1314 attempt-1 verdict, NIT N1 and NIT N4. The code is #1314's
applied-revision watermark, which is on stream B's branch and not yet on `main`; this issue starts
when #1314 is on `main`.

No rendered bit moves. No allocation, lock or syscall is added on the render path.

## Problem (verified on `codex/d15-stream-b` at `b8392df66`; not on `main` at `1e78d7820`)

- **N1: claims between two advances.** `WatermarkWriter::note_claim`
  (`crates/engine/src/realtime/watermark.rs:171-176`) keeps a claimed candidate's `superseded`
  count and outcome for the next `advance` (`:186-218`). Its doc says "claims between two advances
  accumulate". They are merged: the `superseded` counts add, and if any claim was a
  `TransitionFallback`, the merged outcome is `TransitionFallback`. The next `advance` then counts
  every non-superseded revision it covers as a fallback, including revisions of an earlier claim
  that completed exactly. So the merged form misattributes outcomes.
- **N1 is untested.** The verifier's mutation M17 (`note_claim` overwrites instead of adding) left
  every test green. The only caller is `PlanExchange`'s claim
  (`crates/engine/src/realtime/plan_exchange.rs:747`). Through the C ABI two claims between two
  advances cannot happen today: a second rebuild needs `pending_providers` to be empty, which needs
  a successful block after the first adoption. #1310 (supersession) and #1311 (scheduled adoption)
  change the claim paths, so "unreachable today" is not a contract.
- **N4: the seqlock is written twice.** `watermark.rs`'s record (`:91-101`, counter
  `sequence_lock: AtomicU64`), its store (`:220-244`) and its bounded read (`:251-273`) repeat
  `observe.rs`'s (`crates/engine/src/realtime/observe.rs:75-87`, counter `AtomicU32`; publish
  `:127-153`; read `:169-189`), including `MAXIMUM_READ_ATTEMPTS = 64` (`watermark.rs:49`,
  `observe.rs:71`). The odd/even protocol and its fences are the part that must be right; two copies
  can drift.

## Decisions

- **D1. Exact attribution, no queue.** `note_claim` also receives the claimed candidate's own
  revision (the revision its mailbox cell carries). When a claim arrives while an earlier claim is
  still pending, the writer first settles the pending claim into its local `current` counters: the
  revisions from `current.revision` up to the pending claim's revision are counted with that
  claim's `superseded` count and outcome, and `current.revision` moves up to it. Nothing is stored
  to the shared record by a settle. The next `advance` stores one record whose `outcome_flags`
  are the union of the flags every settled span and the advance itself set. Render-side cost stays
  bounded: one settle per claim, no loop. If the claimed revision is not available at the claim
  site, stop and report to root; do not approximate.
- **D2. The doc states the rule.** `note_claim`'s and `advance`'s docs state D1 in place of "claims
  between two advances accumulate".
- **D3. One seqlock.** A new `crates/engine/src/realtime/seqlock.rs` holds the counter protocol:
  the writer's open (counter to odd, release fence), close (counter to even, release store), and
  the reader's bounded consistent read (acquire load, words, acquire fence, compare), with one
  `MAXIMUM_READ_ATTEMPTS` and its reason (`observe.rs:61-70`). It is safe Rust with no `unsafe`.
  `observe.rs` and `watermark.rs` keep their own records and field words and use it. One counter
  width serves both; the choice and its reason (wrap and reader-stall arithmetic) are recorded. If
  a record's retained bytes move (`observation_slot_retained_bytes`, `watermark_retained_bytes`),
  every pinned count that moves is re-pinned with that reason.
- **D4. Regions.** The open and close functions are render-side and sit in a marked realtime region;
  the read is control-side and does not.

## Authorized paths

- `crates/engine/src/realtime/watermark.rs`, `crates/engine/src/realtime/observe.rs`, the new
  `crates/engine/src/realtime/seqlock.rs`, `crates/engine/src/realtime/mod.rs` (the module line)
- `crates/engine/src/realtime/plan_exchange.rs` (the `note_claim` call only)
- resource tests whose pinned bytes move under D3 (each listed with its reason in the record)
- the realtime-policy floors (standing exception), if D4 adds a region
- this spec

## Non-goals

- Any change to the watermark's C ABI record, header, feature bit or reader API.
- Any change to when render advances or claims.
- Any other seqlock-like code (meters, spectrum).

## Hazards

- `plan_exchange.rs` is a hot file (B #1343 → B (#1310, #1311, #1314) → C (#1396, #1355) → H
  #1381 → B #1349). This slice edits one call and lands after #1314; the later slices rebase.
- `observe.rs` and `watermark.rs` are in #1446's comment-lines row. Either order; the later slice
  rebases and keeps #1446's wording.
- The realtime-policy floors row: a new region raises the floor (the awk gate's, or
  `Policy::workspace()` after the J batch). Re-measure when the slice lands.
- The loom models of the watermark and observation slot (if any run under `--features
  engine/realtime-audit` or loom cfg) must still pass unchanged.

## Objective gates

1. **D1's test.** A unit test in `watermark.rs` drives `WatermarkWriter` directly: claim A (exact,
   revisions up to `r + 2`), claim B (transition fallback, revisions up to `r + 5`), then one
   `advance(r + 5, s)`. The read record has `exact == 2`, `transition_fallback == 3`, both outcome
   flags set, `revision == r + 5`, `first_sample == s`. A second case adds `superseded` counts to
   both claims and checks each lands on its own span.
   - Mutation: `note_claim` overwrites the pending claim (M17): red.
   - Mutation: the settle uses the newer claim's outcome for the older span (today's merge): red.
2. **D3's protocol is still exercised.** The existing torn-read tests of both modules
   (`watermark.rs` `a_watermark_read_is_one_whole_publication_or_busy`;
   `crates/engine/tests/observation_transport.rs` `a_million_windows_are_read_whole_and_in_order`,
   #143 E11) pass, and each goes red when the shared close stores the counter before the words (PR
   evidence).
3. `cargo test --locked -p engine --features engine/realtime-audit`, the C ABI tests
   (`cargo test --locked -p capi`), `audit capi` (`allocations 0`, `syscalls 0`, same
   `pcm_digest`), `bash scripts/check-realtime-policy.sh` (or the tool) and
   `bash scripts/check-workspace-policy.sh` pass.

*Test value.* Gate 1's test is red when claims between two advances lose or misattribute a
revision's outcome (M17 and the merge), which no test catches today; gate 2 shows the shared
seqlock is still guarded by both modules' existing torn-read tests.

## Evidence

- Gates 1-3 output, each mutation's red run, the counter-width reasoning, any re-pinned byte count
  with its reason.

## Dependencies

- After (same stream): #1314 on `main` (with stream B batch 1). Before or after #1310 and #1311 in
  either order; whichever lands later rebases on `plan_exchange.rs`.
- After (other streams): none.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: under half a day.
