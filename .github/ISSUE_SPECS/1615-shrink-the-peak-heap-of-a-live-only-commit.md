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
