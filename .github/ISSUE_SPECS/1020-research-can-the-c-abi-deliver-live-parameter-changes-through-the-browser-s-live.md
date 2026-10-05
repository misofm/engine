# Research: can the C ABI deliver live parameter changes through the browser's live-control lane?

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

## Question

The owner wants fans to change a mix live in the mobile app ("they should be able to change it live"). Today a live fader or mute change through the C ABI reaches audio only by structural plan replacement, which resets source rings at the block boundary (`capi/src/runtime/control.rs:46-53`). The browser host already delivers live parameter changes without a plan swap (the effect control lane that #1004 and #1012 hardened). An unfinished second route exists: about 8,200 lines of control-provider endpoints (#528-#608, protocol `delivery.rs` and `controller_delivery.rs`) that nothing calls, the partial implementation of #140. Evidence: `docs/handoffs/dead-code-2026-09-28/VERIFY-DEAD-CODE.md`, finding F8; draft `issues/02-…` is held on this answer.

Establish, with code references and a prototype if cheap:

1. Can the C ABI reuse the browser's live-control lane (the path from a command to an effect's control lane and the fader/matrix) instead of plan replacement, and what would that take?
2. If yes, which of the 8,200 unwired endpoint lines and which parts of #140 become unnecessary, so the engine footprint shrinks rather than grows?
3. If no, what is the smallest engine change that gives mobile live control without resetting source rings?

Deliver a short findings note under `docs/handoffs/` and a recommendation the owner can rule on in one line. No production code change.
