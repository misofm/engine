# Pin the browser wasm memory as a ceiling

Follow-up of #1060 (resource budgets, not exact counts), found by the batch 14a review of #1683.
Code anchors verified on the batch 14a fold branch `codex/controls-b14a-fold`. Issue number:
assigned at filing (written here as #1691). Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review.

## Product outcome

A change that moves the browser module's linear memory by one page does not turn the gate red or
need a re-pin. Growth past a recorded budget still does.

## Context

- **The pin.** `hosts/host-web/tests/browser-v1/expected.json` pins the simd128 `memoryBytes`
  exactly, in three places: `directOracle.simd128.initialStatus`,
  `directOracle.simd128.beforeDisposeStatus` and the `beforeDisposeStatus` of the set-control
  timeline row. `scripts/check-browser-expected-resources.py` compares them with the rest of the
  printed document (`check_document`, the equality after the budgeted rows are removed), so one
  moved page is red.
- **It moves by single pages.** #1632 moved it down one 64 KiB page (2,883,584 -> 2,818,048) and
  #1683 moved it up one page (2,818,048 -> 2,883,584). Neither move was an unintended change; each
  was a re-pin of three rows.
- **The rule.** `AGENTS.md`: "Resource budgets are ceilings." #1060 already made the
  retained-memory rows ceilings (`CEILING_ROWS`, `resourceCeilings`, each with a printed
  headroom). `memoryBytes` is the one resource figure left pinned exactly. It is a size, not a
  claim. Linear memory grows by whole pages, so it moves in steps.
- **What stays exact.** The three PCM digests, the status fields other than `memoryBytes`, the
  result transcripts and the exact rows are claims. They stay exact.

## Decisions to take (the implementer proposes, the coordinator rules)

- Where the ceiling lives: a `memoryCeilingBytes` key beside `resourceCeilings` in the simd128 leg
  of `expected.json`, or one `memoryBytes` entry in `resourceCeilings` (which would need the
  partition to cover a status field).
- The margin. Proposed: the measured value plus 2 pages (128 KiB, about 4.5 % of 2.8 MB), rounded to
  a whole page, with the measurement and its date recorded beside the key in the commit message.
  Two pages cover one move in each direction seen so far, and a second move in one batch is a
  reason to look. The margin is raised only with its reason, as the other ceilings are.
- Whether the three rows share one ceiling (proposed: yes; they measure one module, and a
  WebAssembly memory never shrinks, so the last status is the peak).

## Objective gates

1. `memoryBytes` is a ceiling: the checker removes it from the exact comparison, checks that all
   three printed values are positive page multiples and at most the ceiling, and prints the headroom
   in pages with its other budget lines.
2. The self-test stays red on a mutation: each of the three printed values one page over the ceiling
   is caught, a zero is caught, and a missing or extra ceiling key is caught. The green case sits
   at the ceiling. The existing mutation `move_status` (which adds 128 to a status field) moves to
   a field that stays exact.
3. The current tree passes `scripts/check-browser-expected-resources.py` and its `--self-test`, with
   the digests and exact rows unchanged.

## Authorized paths

- `hosts/host-web/tests/browser-v1/expected.json`
- `scripts/check-browser-expected-resources.py` (the checker and its self-test)

## Dependencies

- None. It is independent of any open batch.

## Size

A third of a day.
