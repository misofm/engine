# Make the session parse-transient bound affine

Follow-up to *Add the session console and inserts* (#1093; Sol's attempt-1 verdict, low 4, and
C3 rebase verdict, low 2).

## Problem

Boot admission projects a session document's parse transient as
`PARSE_TRANSIENT_MULTIPLIER x document bytes`, and refuses a document whose projection exceeds
the budget.
- The projection is host-web's typed boot refusal (`hosts/host-web/src/lib.rs:1796-1808`).
- `scripts/check-web-boot-budget.mjs:9,54,64` and `boot_transient_budget.rs` mirror it.

#1093 raised the multiplier from 17 to 20, because the minimal document's fixed parse overhead
grew:
- The raw parse peak went from 6,780 to 8,944 bytes for a 511-byte document.
- Almost all of that is `json-syntax`'s containers and code map for the empty `console` object,
  not the model.

A linear bound charges that fixed overhead at every size:
- dense documents stay below 13.0 bytes per input byte, so their slack grew from 30% to 54%;
- a 1 MiB document's projection grew from 17 to 20 MiB.

## Smallest closable slice

Replace the multiplier with an affine bound, `a + k x bytes`, fitted to measured parse peaks:
- `a` covers the fixed per-document overhead;
- `k` covers the per-byte growth.

Update the host-web refusal, the boot-budget script and `boot_transient_budget.rs` together.

## Objective gates

1. Every case in `boot_transient_budget.rs` stays at or below the new bound, and the test asserts
   it. A dense 1 MiB document's projection falls well below today's 20 MiB; record the number.
2. A test fails if either `a` or `k` is set below the measured worst case.
3. `check-web-boot-budget.mjs` re-measures in the three browsers and agrees with the native bound.
4. The typed boot refusal keeps its code; only its threshold arithmetic changes.

## Non-goals

- No change to the parser or the frontend.
