# Web host: the render-half handoff leaks an orphan released after a re-boot, and an adopted half's status starts without the control state

Successor of #2332 (WEB-8) and #2346, filed when the implicit-state-machine tracker #2258 closed. Both items are in the Rust slot model, `hosts/host-web/src/ffi.rs` and `hosts/host-web/src/lib.rs`.

## Problem
1. **Leaked orphan (#2332 non-goal).** When an orphaned render half (#1332 S8, `ffi.rs` around line 1614) is released after its Worker has booted a new host, it stays in the handoff registry and is never freed. Each such re-boot leaks one half for the life of the instance.
2. **Status at adoption (#2346 follow-up).** `AudioWorkletEngineHost::adopt_render` (`lib.rs` around line 2871) installs the half without copying the control lifecycle state or the control `last_result` into the half's `WebStatus`. A status read through the half after adoption can therefore show a lifecycle older than the shell showed before the release.

## Contract
- Every render half released into the registry is freed exactly once, by its dispose or by the next boot, whichever comes first. The registry is empty after a dispose of the last host.
- After `adopt_render`, the status read through the half equals the shell's status for every control-owned field. Render-owned fields are untouched.

## Non-goals
- Replacing `RenderHalf::ready`'s `Option` with a `Disposed` variant (#2332 non-goal). Do it only if it falls out of item 1.
- The JavaScript side: #2346 fixed the cached status address.

## Gates
- Red on base: one Rust test per item. (1) Re-boot, release the orphan, and assert the registry count returns to zero. (2) Change the control lifecycle, release and adopt, and assert the half's status matches.
- Loom, or a stated reason none is needed, if the registry change touches any atomic.
- `cargo test -p host-web` green, plus the browser matrix.

## Evidence / attempt record
(empty)
