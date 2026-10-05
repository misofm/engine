# Root draft: decisions and plan for the open items (2026-10-05)

Owner principle (binding, 2026-10-05): "Please make the decisions based on maximizing long-term reliability and correctness. We shouldn't take any shortcuts that need to be fixed or worked around in the future. We are building this out with agents so the usual pre-AI benefit of taking shortcuts now is not worth it. We can just spend resources now to do things correctly from the beginning."

Under this principle every earlier "accept for now" recommendation is withdrawn. Root's revised position per item:

## A. Seamless swap phase 2
A1. Do all of #1269 phase 2 (#1277, #1279–#1288) and the browser track (#1290–#1297), with the corrected decisions below.

## B. Mobile live updates (#1053 follow-ons)
B1. No step changes anywhere. Every live value change ramps. Implement #1054 (configurable ramp lengths) and #1055 (research default ramp lengths, cited and measured) now; non-zero researched defaults in the engine, the C ABI and the SDK (also fixes ruling finding F7's SDK half).
B2. No BACKPRESSURE for value edits while paused. Proposal: per-lane latest-target slots (last writer wins, with its ramp) for value edits, so a value edit can never be refused for capacity. Open: is coalescing superseded value targets compatible with AGENTS.md "commands are never silently lost"? (Argument: the committed model holds the final value; a superseded intermediate target is never audible while paused; the ack reports the applied target.) Adversary must rule.
B3. Report the outcome of every edit: the response says whether the engine applied it live or by plan replacement, and the render sample at which it takes effect. Protocol change now, not later.
B4. Do not report an infinite tail for live input filters. Compute the true bounded worst-case tail over the parameter domain the plan can reach. Then do #1261 and #1262.
B5. #1306: size live effect windows by lane depth.
B6. Also now unblocked: #1225, #1226, #1247 (mobile live buses, sends, VCAs). Do them.

## C. Seamless swap decisions
C1. Carry state whenever the prepared layout is compatible, also when the same transaction changes the node's values (values are parameters, not layout). No "restart at rest".
C2. Latency growth during playback: no gap and no permanent extra latency. Candidate: pre-roll / history fill — the successor's new delay lines are filled from carried history so the swap is seamless and aligned. Adversary to evaluate feasibility versus a fixed reserve.
C3. A strip added during playback starts at its exact sample and ramps in from silence.
C4. Browser plan preparation must not block the audio thread at all. Candidate: prepare on a worker (Wasm threads + SharedArrayBuffer, or a separate Wasm instance that produces a transferable prepared plan). Time-slicing on the audio thread still steals render budget. Adversary to evaluate the host requirements (COOP/COEP) and the alternatives.
C5. A removed strip must not click: fade it out (two-phase: mute ramp, then structural swap), not a hard stop.
C6. One public edit API on every host now (decision 14). No separate public replaceSession; the browser gets the unified edit call that routes internally to live update or plan rebuild.

## D. New public C ABI function
D1. Review `miso_engine_v1_source_seek_at` and feature bit 32 properly (naming, semantics, error codes, interaction with generations, documentation, tests). Fix whatever the review finds before launch.

## E. Decision-14 findings
E1 (F1) refuse automation targets on non-live parameters (validator + SDK).
E2 (F2) for each prepared-only effect parameter: make it live or record a correctness/glitch/optimisation reason.
E3 (F3) make `follows_mute` live.
E4 (F4) refuse (typed) commands that would be acked but have no effect; extend rule 3 to accept correctness reasons and record them.
E5 (F7) measure bypass and step clicks; crossfade the bypass switch.
E6 also F9 (#1247 VCA membership reason) — record or make live.

## F. Reliability follow-ups already filed
#1232, #1234, #1235, #1236 (owner Q1 answer: per-strip link mode), #1237 (route bounds), #1248, #1251, #1300–#1305. Do them.

## Questions for the adversary
1. Is any root position wrong, unsafe, or itself a shortcut? Is any infeasible in this engine's realtime rules?
2. What is missing (items, dependencies, risks)?
3. Propose the order: workstreams, dependencies, what can run in parallel without file conflicts.
