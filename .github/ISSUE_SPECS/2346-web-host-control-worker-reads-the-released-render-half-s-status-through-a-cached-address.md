# Web host: the control Worker reads the released render half's status through an address it cached before the release

Found by #2332's adversarial review, under the implicit-state-machine tracker #2258. The defect predates #2332. Re-verified on `900c3d24c`.

## Problem
The control Worker (`hosts/host-web/web/miso-engine-v1-control-worker.js`) caches the status address once, in `readReadyFields` (`this.statusPointer = statusPointer`, line 406 on base). It does this before `miso_engine_web_v1_host_release` (line 342). `readStatus` (line 471) and so `answerStatus` (line 1578) keep reading through that cached address for the life of the session.

`miso_engine_web_v1_status_ptr` (`hosts/host-web/src/ffi.rs:1583`) answers `host.status()`, which is `RenderSlot::status()` (`hosts/host-web/src/lib.rs`):
- while the half is here, it is the half's `WebStatus`, inside `Box<RenderHalf>`;
- after `release_render`, it is the `RenderSlot::Away(WebStatus)` shell, inline in the host's own `Box<AudioWorkletEngineHost>`.

So after the release, the cached address is the released half's status. That status lives in a box the worklet's render thread now owns and writes on every block. In worker mode the Worker therefore:
- reads render-thread memory without atomics: the 64-bit clock words can tear, and the Worker reports the half's `state`, `last_result` and `blocking_rebuilds` as its control fields;
- never sees its own control results or the lifecycle state that `record` and `end_session` mirror into the Away shell.

A Worker status read after a control failure therefore reports what the render half last wrote, not the control half's failure. A mutation that stops `record` mirroring the lifecycle state into the shell stays green: no Rust test reads the shell after a render-side failure followed by a control call, and the JS rows run on a fake module that has one fixed status address.

## Model
The status address belongs to the slot, not the boot: the Worker reads `status_ptr` on every status read and never caches it. After a release that address is the shell, which is stable because the host is boxed. After the adoption back (in `finishDispose`) it is the half's own status again, which is also current, so a status answered there carries the half's clock.

Other cached addresses checked:
- `sourceIdPointer` and `sourcePcmPointer` are control-half staging (`buffer_pointer` -> `source_id_mut`/`source_pcm_mut`), so the release does not move them.
- `renderStatusPointer` is the worklet's own half status, which the worklet names and the Worker reads only with `Atomics.load`.

## Non-goals
- No change to the status layout, the ABI or the Rust slot model (#2332).
- The worklet's own status binding: it binds after it adopts the half, so it is the half's.

## Gates
- JS row (`scripts/test-web-audioworklet.mjs`, `testControlWorker`): a fake whose `status_ptr` follows the slot (half before the release, shell after it, half after the adoption). After a release, the shell holds a failed state and result while the half's status still says ready. A Worker status read must report the shell's. Red on base.
- Rust row (`hosts/host-web/src/tests.rs`): release, fail the released half, then make a control call on the host; the host's status (the shell `status_ptr` names) must read `STATE_FAILED`. Red if `record` stops mirroring the lifecycle state.
- `bash scripts/test-web-audioworklet.sh`, `bash scripts/check-workspace-policy.sh`, and the host-web Rust gates if Rust changes.

## Evidence / attempt record
Attempt 1 (Terra), branch `codex/issue-2346-worker-status-follows-slot`, on base `900c3d24c`.

**Fix.** `readStatus` calls `miso_engine_web_v1_status_ptr(this.handle)` on every read and refuses a zero address. `this.statusPointer` is gone. `readReadyFields` still checks the boot's address. The Rust change is test-only, so the shipped wasm is unchanged and needed no nightly rebuild.

**Red on base** (the new JS row, `node scripts/test-web-audioworklet.mjs`, before the fix):
```
AssertionError [ERR_ASSERTION]: the Worker reports the shell's lifecycle state and result
  actual: [ 'miso.status.v1', 2, 0 ],
  expected: [ 'miso.status.v1', 3, 7 ],
```

**Mutations** (each run by hand, then reverted):
- The base Worker, which caches the boot's address: the JS row goes red (above).
- Caching the shell's address at the first read after the release (`this.statusAt ??= status_ptr(...)`, loaded through `MISO_ENGINE_WEB_CONTROL_WORKER_TEST_MODULE`): the JS row goes red, with `after the adoption back, the status read is the half's own` (`renderedQuanta` 0n, expected 11n).
- In `AudioWorkletEngineHost::record`, dropping `shell.state = control.lifecycle.state()`: `cargo test --locked -p host-web --all-features --no-fail-fast` goes red on exactly one test, the new `the_released_status_shell_shows_a_render_side_failure_at_the_next_control_call` (110 passed, 1 failed in the lib suite; every integration binary green). Without the new row, the mutation survives the whole host-web suite.

**Gates** (all green): `cargo test --locked -p host-web --all-features`; `cargo clippy --locked -p host-web --all-features --all-targets -- -D warnings`; `cargo fmt --check -p host-web`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-web --all-features`; `bash scripts/test-web-audioworklet.sh`; `bash scripts/check-workspace-policy.sh`.

**Limits.** The JS rows drive the Worker on a fake module, so the JS row proves the Worker follows the address `status_ptr` names. The Rust row proves the shell that address names carries the mirrored lifecycle state. No hermetic test runs the real wasm in the Worker; the browser qualification does.
