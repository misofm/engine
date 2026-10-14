# Shrink the peak heap of a live-only commit

Successor of #1613 and #1562, decision 17. Code anchors verified on the flip branch after
`be71c6097`.
Issue number: assigned at filing (written here as #1615). Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review.

## Product outcome

A live-only transaction on a small session needs a smaller peak of heap at once, so the browser's
first live transaction stops growing the wasm memory in the AudioWorklet's message handler (a
growth that costs a `memory.grow` on the audio thread). The growth is no fault today: the worklet
re-binds its views and the control allocation count reports it (#1562, root ruling A). This slice
removes the cause, it does not change what a transaction does.

## Context

- **The measurement** (flip CI fold, native release, `native_staging::control_command`, a counting
  allocator, the qualification's nine-edit live transaction on `observation-session.json`; recorded
  in #1613's evidence): 754 allocations, 282,694 requested bytes and 167,303 bytes live at the peak,
  steady state; 846, 281,634 and 172,790 on the first call. The browser grew 46 to 48 pages on it.
- **The sites** (steady state):
  1. the strip's catalog rows in one scratch `Vec<ControlRow>` (232 bytes a row) that grows by
     doubling, 7,424 to 59,392 bytes (`crates/controls/src/registry.rs:39` `push_control`, called from
     `effect_rows.rs:336` and `strip.rs:269`, `:414`; the scratch is `fresh` in
     `ControlCatalog::stage_values`, `crates/host-core/src/control_provider.rs:1413`), about 111 KB
     requested and 89 KB live while the last doubling copies;
  2. the replay-cache clone for the prospective response, 65,536 bytes of storage
     (`crates/protocol/src/controller.rs:262`, `try_clone_eager`, from `:1869`), and the response
     frame staging of `max_response_bytes`, 16,384 bytes (`controller.rs:1851`);
  3. the derived-row scratch of the staging comparison, 4,656 and 9,296 bytes
     (`control_provider.rs:1590`);
  4. the canonical text of the one edited owner, old and new, for the estimate arithmetic: 4,096 and
     8,192 bytes twice (`crates/session/src/canonical.rs:194`, `:54`).

## Decisions to take (the implementer proposes, root rules)

- **S1. The row scratch.** Reserve the owner's row count before pushing (the catalog already holds
  the owner's run, `run`), or keep one scratch for the commit. Either removes the doubling.
- **S2. The replay clone.** The prospective replay cache is cloned eagerly (storage and entries) to
  complete one entry. A copy of the one entry's bytes, or completing the entry in the committed cache
  at the commit after the preflight, would drop 80 KB. This is the protocol's constant and changes
  the "acked-batch" argument of the cache, so it needs its own review; it is the decision to bring to
  root before any code.
- **S3. The text of an edited owner.** Writing the old and new text of an owner twice for the
  length arithmetic could use the length alone (a counting writer), with no buffer.

## Objective gates

- Thread-scoped allocator gate (as #1612 and #1613): the live-only commit's peak live bytes and
  requested bytes are ceilings, equal at 64 and 256 tracks, and the nine-edit transaction's peak is
  at most one wasm page above what the boot leaves free; the number is measured on this head and the
  ceiling set with the slack the repo's allowance asks for.
- The differential of #1613 (full path as oracle) stays green; the catalog, events, replay entry and
  revision are unchanged for every transaction.
- Render-locked allocations stay exactly 0.

## Non-goals

No change to a classification, a commit, the response bytes or the replay semantics. Growth stays
legal: the worklet re-binds its views after it.

## Amendment (controls coordinator, batch 9, 2026-10-14)

Checked against `main` at `7feb27800`.

- **Anchors moved.** `push_control` is `crates/controls/src/registry.rs:30`; it is called from
  `effect_rows.rs:327`, `strip.rs:269` and `:311`, `route.rs:244`, `vca.rs:79` and
  `session_rows.rs:83`. `ControlCatalog::stage_values` is `control_provider.rs:1412-1495`: the scratch
  `fresh` (`:1465`) is already one `Vec` for the whole commit, cleared per owner and reused for the
  derived rows (`:1488-1490`); it grows by doubling inside one owner. Site 3 is not a separate
  derived-row scratch: it is the `memo` map (`:1466`), which grows by `try_reserve(1)` in `stage_run`
  (`:1642`). `try_clone_eager` is `controller.rs:281`, called in the structural prepare at
  `:1874-1876`; the frame staging is `:1858`; the commit installs the clone at `:2318-2340`. Site 4 is
  `owner_bytes` (`session/src/canonical.rs:205`), called through `session::canonical_owner_bytes`
  (`in_place.rs:100`) from `protocol/src/model.rs:1448-1449`, once for the old owners and once for the
  new.
- **Proposed decisions (root rules; the implementer can start on S1 and S3 at once).**
  - **S1, the row scratch.** (A) Before the loops, reserve `fresh` once with `try_reserve_exact` to
    the largest of the changed owners' run lengths and the derived-row counts (both are known before
    any row is made), and reserve `memo` to the largest run. (B) Keep a scratch in the catalog across
    commits (needs `&mut self` or a cell). (C) Compare rows as `owner_rows_in` makes them, with no
    scratch (a new visitor API in `controls`). **Recommendation: A.** One exact allocation, no
    doubling copy, no change outside host-core; it also removes site 3's growth.
  - **S2, the replay clone.** (A) Keep it. (B) At prepare, call the read-only
    `ReplayCache::plan_preflight` on the committed cache and keep its `ReplayPreflightPlan`
    (`evicted`, `removed_bytes`) in the token instead of a cloned cache; at commit, call
    `apply_preflight_plan`, `complete` and the final-frame write on the committed cache. While a token
    is outstanding the structural generation is odd and every other entry point refuses
    (`controller.rs:1545`, `:1706`, `:1743`, `:1775`), so the cache cannot change between the plan
    and the commit, and the commit cannot fail (it asserts, as `overwrite_completed_response` does
    today). A dropped token has nothing to undo. (C) Reserve in the committed cache at prepare and
    release on drop: refused, because an eviction cannot be undone on abort. **Recommendation: B**,
    with the frame staging sized to the measured frame length (`plan.response_measure`) instead of
    `max_response_bytes`, if the measure gives the exact length (the final frame has the same length;
    the commit asserts it). The acked-batch answer does not change: the reply exists only after the
    replay entry is written, in the same commit call. The token's layout changes; keep
    `size_of::<PreparedStructuralCommand>()` with `_resource_layout_reservation`, or re-pin the C ABI
    resource row with its record.
  - **S3, the owner text.** (A) A counting sink: `JsonWriter` writes through a small trait with a
    `String` sink and a byte-count sink, and `owner_bytes` counts. (B) Reuse one `String` for the old
    and the new owner (half the bytes). (C) Keep per-owner byte counts in the compiled estimate (more
    state to keep exact). **Recommendation: A.** No buffer; the canonical writer corpus test holds
    the bytes unchanged.
- **Edit order.** In `protocol/src/controller.rs`: after #1567. In `host-core/src/control_provider.rs`:
  after #1341a1, #1567 and #1574. In `session/src/canonical.rs`: after #1566.

## Ruling (root, 2026-10-14)

- **S1 (row scratch): option A.** Reserve `fresh` and `memo` once, exactly, to the largest run
  before the loops; no `controls` change.
- **S2 (replay clone): option B.** At prepare, run the read-only `plan_preflight` and keep its plan
  in the token; at commit, apply the plan on the committed cache, `complete` the entry and write the
  final frame; size the frame staging to the measured length. While a token is outstanding every
  other entry point refuses (odd generation), so the commit cannot fail. Prepare and commit of one
  transaction run in one synchronous submit, so no real second request meets the refusal; the
  decision record still states what a concurrent entry gets (a typed refusal, nothing queued, no
  ack) and one test shows it. The token keeps its size; any re-pin of the C ABI token row comes
  with measured numbers.
- **S3 (owner text): option A.** A counting writer for `owner_bytes`; no buffer.

## Decision record: #1615 (batch 9)

Root's rulings held: S1 option A, S2 option B, S3 option A. Measured natively with
`hosts/host-web/tests/control_command_heap.rs` (new, committed as the gate): a thread-scoped counting
allocator around one `native_staging::control_command`, the qualification's nine-edit live
transaction (seven parametric-EQ edits on console slot `eq-simd1`, two left-lane input-filter
cutoffs) on `observation-session.json`, at 1, 64 and 256 tracks. The path is `live`, the response 80
bytes. "Requested" counts a `realloc` as its whole new size.

| steady call, one track | allocations | requested bytes | peak live bytes |
|---|---|---|---|
| before (`7feb27800`) | 786 | 293,477 | 168,031 |
| after S1 | 776 | 215,809 | 153,284 |
| after S1 and S3 | 754 | 183,057 | 153,284 |
| after S1, S3 and S2 | 753 | 101,025 | 71,252 |

The first call (one track) went 844 / 280,733 / 172,790 to 833 / 121,033 / 76,011. The steady call is
the same number at 1, 64 and 256 tracks. Debug and release builds give the same numbers once the
debug-only stage check (`verify_stages`, which builds a full catalog as an oracle) is off for the
test's thread. The browser's pages grown (46 to 48 before) were not measured: this box has no
browser. The peak fell by 96,779 bytes, 1.48 wasm pages, so the first transaction needs about 1.1
pages instead of 2.6.

**What remains at the peak (71,252).** The row scratch `fresh`, 39,672 bytes (171 rows of 232
bytes, S1's one exact allocation: the strip's rows must all exist to be compared), the `memo` map
18,576 bytes, and about 13 KB of the rest (writes, frame, request copy, events). The scratch is the
S1 option A; removing it needs option C (compare rows as they are made), which root did not rule.

**S1, the row scratch** (`host-core/src/control_provider.rs`, `stage_values`). Before the loops the
stage finds the longest changed-owner run and the largest derived-row count (both known before any
row is made) and reserves `fresh` once with `try_reserve_exact`, and `memo` to
`min(longest, descriptors.len())` (distinct descriptors cannot exceed the catalog's). The try-reserve
in `stage_run` stays and no longer grows. No `controls` change.

**S2, the replay clone** (`protocol/src/controller.rs`). `prepare_structural_plan` no longer clones the
replay cache. `plan_structural_command` already ran the read-only `plan_preflight` on the committed
cache and put the plan in `StructuralCommandPlan`; the token now keeps that plan (`replay_plan`,
`evicted` and `removed_bytes`) and a copy of the request bytes (`request`, `Box<[u8]>`, request
length). The commit applies the plan to the committed cache (`apply_preflight_plan`), `complete`s the
entry with the final frame and returns the reply, in one call. The frame staging is sized to
`OUTER_HEADER_BYTES + response_measure.length` (the measured length; prepare refuses with the same
`OutputTooSmall` the old max-sized buffer gave when that exceeds `max_response_bytes`) instead of
`max_response_bytes`; the final frame has the same length and the commit asserts it.
`overwrite_completed_response` and the prepare-time `preflight`/`complete` on the clone are gone.

*Why the commit cannot fail.* The plan is made against the committed cache in
`plan_structural_command`; between it and the commit nothing writes that cache: the token sets the
controller's structural generation odd and `process`, `process_command_frame_into`,
`prepare_command_frame` and `prepare_command_frame_in_place` refuse while it is odd. So at the commit
the cache is the one the plan saw, the eviction is the planned prefix, and `complete` finds its
reservation, an entry slot and the bytes (the plan checked `request + max_response_bytes` fits). The
commit `expect`s those, as `overwrite_completed_response` did.

*Acked-batch rule, and the cache's guarantees.* A reply exists only after its replay entry is
written: `CommittedCommandFrame` is built after `complete` returns, in the same call that applies the
session commit and the event. Nothing is written to the committed cache before the commit, so a
token dropped or refused (`WrongController`, `StaleGeneration`) leaves the cache as it was, which is
what dropping the clone did. The entry holds the request bytes and the committed frame (the path the
host reported), so an exact replay returns the same bytes, a changed request under the same ID is
`RequestIdReuse`, an evicted one is `ReplayExpired`, and the eviction happens once, at the commit.
New test `a_structural_commit_writes_its_replay_entry_and_evicts_at_the_commit` (one-entry cache:
nothing before the commit, the dropped token changes nothing, replay is exact, eviction at the
commit) and the existing `structural_token_cancel_owner_generation_and_serial_rules_are_affine`.

*What a concurrent entry gets.* While a token is outstanding every entry point refuses: the frame
entries return `Err(CommandFrameProcessError::PreparedCommandOutstanding)`, the compatibility
`process` answers a `Backpressure` frame, event egress returns `ReliableQueueFull`, and
`queues_mut` panics by design. Nothing is queued, no replay entry is made and no ack is sent; a
replay of a cached entry is refused the same way and answered once the token is committed or dropped.
In the hosts, prepare and commit of one transaction run in one synchronous submit, so no real second
request meets it. The new test shows the replay refusal; the existing one shows the rest.

*Token size.* The cache handle (88 bytes) became the plan (16) and the request box (16);
`_resource_layout_reservation` grew from 24 to 80, so `size_of::<PreparedStructuralCommand>()` is
960 before and after (new test `the_prepared_token_keeps_the_size_the_resource_row_counts`).

*Departure: the resource row.* The C ABI and browser resource report charged the prepared-protocol
term as response frame + token + the replay cache (the clone) + catalog
(`control-plane/src/compile.rs`, mirrored in `capi/tests/resource_lifecycle.rs`). With no clone that
term overcharged by the cache. It now charges frame + token + a control-frame-sized request copy +
catalog (the request is bounded by the control frame). For the browser (16,384-byte frame, 64 KiB /
32 entries cache) the term fell from 16,384 + 960 + 67,328 + catalog to 16,384 + 960 + 16,384 +
catalog, 50,944 bytes less. The largest single allocation is unchanged (the cache's 65,536-byte
arena is still the cache's own row). The live-peak gate takes this term; the `capi` and `control-plane`
tests are green at the new number.

**S3, the owner text** (`session/src/canonical.rs`, `value.rs`). `JsonWriter` is generic over a
`Sink` (`String`, or `ByteCount`, which keeps a length), and `owner_bytes` counts. A number is
spelled in a fixed 128-byte stack buffer (`Spelling`) by the same `spell_f32` the `String` path
uses, so the length is the document writer's by construction; the unit test
`the_counting_sink_counts_the_bytes_of_the_string_sink` walks every exponent and the extremes. The
canonical writer corpus test holds the document bytes unchanged. The change saves 22 allocations and
32,752 requested bytes per call (776 to 754, 215,809 to 183,057); the peak number did not move, so
the owner text was not live at the peak.

**Gates.** `cargo test`: `-p protocol --features test-support`, `-p control-plane --features
test-support`, `-p host-core --all-features`, `-p session`, `-p capi`, `-p host-web --features
host-web/test-support` pass. The heap gate is
`a_live_only_nine_edit_transaction_has_a_small_peak_at_every_size`: steady peak 71,252 against a
ceiling of 73,000, requested 101,025 against 104,000, first call (one track) 76,011 against 78,000,
the steady call equal at 64 and 256 tracks, at most 256 bytes left live. The #1613 differential
(`verify_stages`, the full path as oracle) stays on in every other test and is green. `cargo clippy
-D warnings` on the six touched crates, `scripts/run-protocol-allocation-audit.sh`, `audit protocol`
and the four policy scripts pass. Render-locked allocations: no render-path file changed.

**Successor found.** The first live commit on a larger session still grows with the session (64
tracks: peak 196,593, 256 tracks: 663,179, against 76,011 on one track), because it builds what the
later calls keep. It is a one-time cost, outside this slice's gate (the steady call), and worth its
own look if a browser session of hundreds of tracks must not grow the heap on its first edit.

**Browser fixture pin (batch 9 fold).** The browser identity fixture
(`hosts/host-web/tests/browser-v1/expected.json`, `directOracle.setControlTimeline.beforeDisposeStatus.memoryBytes`)
is the one exact field this slice moves: 3,080,192 -> 2,949,120 bytes (47 pages -> 45 pages, 2 pages
fewer), measured with the built `simd128` module through `direct-oracle.mjs`. Bisected by building and
printing the oracle at each first-parent commit of batch 9: `1d6f91246` (#1341a, #1602), `d7fe0470d`
(#1611) and `47676de09` (#1566, #1568, #1569) print 3,080,192; `af8bcad9b` (this slice) prints
2,949,120, and the later merges (#1574, docs, folds) keep it. The other two `memoryBytes` fields (the `simd128` initial and before-dispose statuses) stay 2,883,584,
every PCM digest, status
snapshot and result transcript is unchanged, and only the `resourceCeilings`-budgeted rows move (all
down). This is the live-transaction peak of the SET_CONTROL timeline's nine-edit commits falling by
about 97 kB, as this record measures natively.
