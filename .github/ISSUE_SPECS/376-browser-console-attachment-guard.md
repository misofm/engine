# Browser host lacks the headless boundary's "no console attached" guard; refusal reason is misleading

## Current approved scope — 2026-09-11

### 3. #376 — explain missing browser console attachment immediately

Product outcome: requesting browser console controls without attaching a console gives an actionable usage error instead of a later misleading `unsupportedKind` refusal.

Current evidence: `sdk/src/browser/engine.ts` lazily constructs the console without checking attachment; `sdk/src/browser/console.ts` requests the session map unconditionally. `hosts/host-web/src/lib.rs::submit_commands` maps missing console to `COMMAND_REASON_UNSUPPORTED_KIND`. The headless boundary already has the useful message.

Smallest scope: check the effective browser boot console policy at the public console access boundary, reject its existing Promise with MisoUsageError directing callers to policy.console.commandQueueRecords, and document the raw lower-level `unsupportedKind` no-console case. The issue explicitly permits documentation instead of a new wire reason. Keep audio-only boot legal and explicit console opt-out intact. Do not auto-attach controls, redesign SDK defaults, or change ABI reason values.

Acceptance: omitted, empty and explicit-zero console settings reject before sessionMap or command transport; later caller mutation cannot change the captured boot configuration; an attached console still admits commands; independent boot-policy handling and audio-only operation remain valid. Test browser/headless consistency and the packaged public SDK surface. Existing artifact pin stays fixed; required PR/main CI and synchronized closure.

Risk/size: small, mostly SDK behavior and documentation. Coordinate overlap with broad #379 without adopting its entire API redesign.


Astra XHIGH scoped this issue and user authorized delivery. Astra LOW implements, Astra XHIGH verifies; five-attempt ceiling. Root owns checkpoint commits/pushes, artifact identity and GitHub synchronization. Start only after a slot is freed by #387/#211 delivery; at most two active issues. Own sdk/src/browser/engine.ts, necessary focused SDK tests and user-facing SDK documentation. No Rust/ABI/default-console policy or adapter changes; preserve async console API and captured boot-policy semantics.

Run focused tests/type/package gates against the already qualified artifact, then independent review and required PR/main qualification. No artifact repin, new benchmark or compiler-IR captures. Preserve failed attempts. Close only after evidence is upstream, main qualification passes and remote issue state is synchronized; remove clean delivered worktree.

## Historical issue body

## Summary

Two related SDK-level gaps make "I forgot to attach a console" a hard bug to diagnose in the browser, even though the SDK already has a good error message for this exact situation — it just doesn't fire on the path most browser integrations actually use.

## 1. The headless boundary's clear guard doesn't exist on the browser/worklet host path

Booting an engine session in-process (headless, via `@misofm/engine/headless`) without `console.commandQueueRecords` set and then touching `session.console` throws immediately with a clear, actionable message:

```
MisoUsageError: this engine booted with no console attached; set console.commandQueueRecords
```

Booting the same document the same way (no console configured) through the **browser** path (`@misofm/engine/browser`'s `createEngine`, as used by `@misofm/engine-web-adapter`) has no equivalent guard. Instead, the session opens successfully (`state: "ready"`, plays audio fine), and the absence of a console only becomes visible later, indirectly, as every `console.submit(...)` call resolving with:

```
{ ok: false, code: "unsupported", reasonName: "unsupportedKind", rejectedIndex: 0, admitted: 0 }
```

Traced this to `hosts/host-web/src/lib.rs`: `console_request()` maps `console_command_queue_records == 0` to `control_queue_depth: None`; `console_attached()` (checking `!ready.controls.is_empty()`) is then `false`; `submit_commands` unconditionally returns that refusal without decoding anything when `!self.console_attached()`.

**Suggested fix:** give the browser host the same fail-fast behavior the headless boundary already has — either refuse to construct/return a working `session.console` object when no console was configured, or throw the same `MisoUsageError` message at the first `session.console` access, rather than requiring a caller to get all the way to a per-`submit()` refusal to find out.

## 2. The Rust refusal reason for "no console attached" is indistinguishable from "unrecognized command kind"

`submit_commands` reports the "no console attached" case using `COMMAND_REASON_UNSUPPORTED_KIND` — the same reason a genuinely malformed or unrecognized command kind would produce. The doc comment on that constant (around line 236) does name this specific case ("a host with no console attached at all"), so the distinction is understood internally, but it isn't exposed as a distinct value a caller can branch on or use to produce a clear error message.

This reads very misleadingly from the caller's side: `faderDb`, `mute`, and `solo` are all long-established, correctly-encoded command kinds (verified: the JS-side `wireCommandKinds` table encodes them fine, no exception is thrown building the record) — "unsupported kind" strongly suggests an encoding bug on the caller's side, not "you never attached a console."

**Suggested fix:** give this case its own reason (e.g. `COMMAND_REASON_NO_CONSOLE_ATTACHED`), or at least document `unsupportedKind` explicitly as covering this case so SDK consumers know to check console attachment first when they see it.

## Why this matters

Both gaps compounded to turn a one-line missing boot option into a debugging session that needed a full headless-Chrome CDP reproduction to pin down — the engine looked entirely healthy (correct session shape, correct track list, real audio playing) right up until every single real-time control silently failed. Root cause and full repro trace are in the companion issue misofm/engine-web-adapter#9, which is what actually caused this for us (it doesn't default or surface the missing `policy.console` option) — this issue is specifically about the two SDK/engine-side gaps that made the resulting failure hard to diagnose once it happened.

## Astra LOW attempt 1 checkpoint

BrowserEngine.console now returns a cached rejected Promise with MisoUsageError
and the browser policy path when captured commandQueueRecords is absent/zero.
Attached-console behavior, async contract and console-free boot/close remain.
Focused tests include mutation during boot, no transport on refusal, positive
command admission, and packaged public browser/headless error behavior.
Typecheck, 61 focused SDK tests and sdk-package check against the unchanged
qualified80abeec2 artifact passed. Actual commands/env/exits/logs:
`/tmp/issue376-attempt1`. No physical-browser opt-in run is claimed; required
CI/browser delivery remains. Root checkpoints; independent XHIGH review pending.
