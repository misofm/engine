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
### Attempt 1 (Terra, 2026-11-08)
Base `9c391cb46`, branch `codex/issue-2373-handoff-orphan-and-adopted-status`.

**Both claims hold on base.**
1. `adopt_render_half` (`ffi.rs`) offers a token to a booted host through `host.adopt_render`, which takes back only its own half (`Arc::ptr_eq` on the lifecycle word), or refuses with `RESULT_WRONG_STATE` while the host's half is here. So an orphan released after the Worker booted a new host is re-deposited and refused. Nothing else ever frees a registry entry: only `take`'s confirm empties one. Each such orphan also holds one of the registry's four entries, so enough of them make a later `host_release` return `0`.
2. `adopt_render` copied the render failure's diagnostic and nothing else. `record` mirrors the lifecycle state into the shell only at a control call, and the half learns of a control failure only at its next block. So after a control failure, the status read through the adopted half was `READY`, with the half's own last result and its last block still in the output buffer.

**Model.**
- **Item 1: the control instance sweeps the registry.** A new `handoff::free_disposed` runs at every boot (after the lifecycle refusal, before the preparation) and at every dispose (after the session is gone) on an unbound instance. A bound instance never boots a host, and its dispose of an adopted half is refused before it gets that far. For each entry, the sweep claims the token exactly as an adoption does (`claim`: the same `AcqRel` compare-exchange). It frees the half when the half's session is disposed (`RenderHost::session_disposed`: the lifecycle word is `STATE_DISPOSED`, or there is no session). Otherwise it cancels, and the half goes back under its token, unconsumed. `take` now uses the same two helpers, `claim` and `decide`, so their confirm and cancel steps are one piece of code. The sweep frees, so it sits outside the registry's realtime-policy region and is never called from a render-locked export.
  - "Disposed" is the lifecycle word, not the handle. Handles are per instance, and two native instances (two test threads) issue the same ones. The word is the one fact that says no host will take the half back.
  - A dispose also frees its own half if that half was released and never taken back. Its session is disposed by then.
- **Item 2: the adoption publishes the control fields.** `adopt_render` copies the shell's `last_result` into the half's status. When the lifecycle word is not ready, it silences the half (`RenderHalf::silence`: state `FAILED` and a zero output block), as `end_session` does with the half here. The state therefore comes from the lifecycle word, not from the shell's mirror, which can be stale (a render failure after the last control call). `blocking_rebuilds` needs no copy: a rebuild counts only while the half is here (`run_control`'s `render_here`), so the count the half took away is still the shell's. The render clock, counters and watermark stay the half's own.
- **Loom: none added. Reason:** the sweep adds no new atomic step. Its only cross-thread operation is `claim`, which is `take`'s compare-exchange moved into a helper, and its confirm and cancel are `take`'s stores, moved into `decide`. To any other thread, a sweep is indistinguishable from an adoption of that token whose `accept` decided by the lifecycle word. Two concurrent takers of one token were already part of the registry's protocol: the loser finds no entry and is refused, and an adopter never waits. The token load before the claim is `Relaxed` and only a hint. The lifecycle read happens under the claim, after the claim's `Acquire`. A stale `READY`/`FAILED` read only cancels, which is safe. A `DISPOSED` read means the control half is past `lifecycle.dispose()`, and dropping a `Send`, owned `RenderHost` there is sound. On the page, the Worker both disposes and sweeps, so the read is in program order. The lifecycle word's own transitions keep `lifecycle_loom`.
- **One hazard, by the protocol's existing rule.** An adoption of a live session's token during a sweep's claim of that entry is refused (`RESULT_INVALID_ARGUMENT`, token unconsumed), as against any other claim. The shipped pages never reach it: the Worker sweeps only while it holds no live host (boot), or as it disposes its only one, and the worklet adopts only the token the Worker's boot released after the sweep.

**Red on base.** Each new test was run on base with only the test-support accessor `native_staging::handoff_occupied` added.

Item 1, `an_orphan_released_after_a_re_boot_is_freed_by_the_next_dispose_or_boot` (`tests/worker_handoff.rs`):
```
panicked at hosts/host-web/tests/worker_handoff.rs:951:9:
assertion `left == right` failed: the dispose freed the orphan
  left: 1
 right: 0
```
Item 2, `an_adopted_half_publishes_the_control_fields_the_shell_showed` (`src/tests.rs`). The tuple is (state, last result, blocking rebuilds):
```
panicked at hosts/host-web/src/tests.rs:1891:5:
assertion `left == right` failed: the control fields are the shell's
  left: (2, 0, 0)
 right: (3, 1, 0)
```

**A test reordered.** `an_orphaned_render_half_is_freed_by_the_worker_that_adopts_it_back` had the worklet release the orphan "late" with no barrier after the Worker's dispose, so the release could land before the dispose. With the fix, that dispose frees a half that is already in the registry, and the Worker's adoption then finds nothing. One more barrier now orders the release after the dispose, as the test's comment intended.

**Mutations of the fix.** Each was applied, run and restored.

| Mutation | Result |
|---|---|
| dispose without `free_disposed` | red: "the dispose freed the orphan" (`1` vs `0`) |
| boot without `free_disposed` | red: "the boot freed the orphan and kept the live half" (`2` vs `1`) |
| the sweep frees every half (`session_disposed() \|\| true`) | red: same assertion (`0` vs `1`): another instance's live half was freed |
| adoption sets the state to `FAILED` without silencing | red: "the adopted half of a failed session is silent" |
| adoption copies the shell's (stale) state instead of the lifecycle word's | red: "the lifecycle word's state, not the stale shell's" (`2` vs `3`) |
| adoption copies no `last_result` | red: "the control fields are the shell's" (`(3, 0, 0)` vs `(3, 1, 0)`) |
| adoption copies `blocking_rebuilds` too (the first draft did) | the copy's removal stayed **green**: equal by construction (see Model), so the copy was dropped |

**No bit moved.** A temporary probe, not committed, used the exports on the nine-track fixture (48 kHz, 128-frame blocks, the handoff helpers' fed ramp). It ran four scenarios: release and adopt back on the same instance, or via a second instance that renders two blocks, each with and without a `session_fail` while the half is away. After each step it wrote the output-bit hash, the return code and the status (state, last result, blocking rebuilds, clock, quanta, watermark). The transcripts of base and fix differ in exactly two lines: the status read right after an adoption that followed a control failure, which this issue changes. Base reads `(READY, 0)` with the half's last block. The fix reads `(FAILED, 255)` and silence, the state the base reached only at the next block. With those lines removed, the transcripts are byte-identical (SHA-256 `d8d8cf039e0d48ea31da8065d5384e1ae9d6521dbc1ebd5d82a422c87dc6fc09`, 40 lines, 24 rendered blocks).

**Test value.**
- `an_orphan_released_after_a_re_boot_is_freed_by_the_next_dispose_or_boot` is red if a disposed session's half released after a re-boot stays in the registry past the next dispose or boot, or if the sweep frees a live session's released half. No existing test releases an orphan while the Worker holds a host.
- `an_adopted_half_publishes_the_control_fields_the_shell_showed` is red if the adoption back leaves the half's status as the worklet left it: a control failure reads as ready, with the half's own last result and a non-silent block. It is also red if the adoption takes the shell's stale state instead of the lifecycle word's. No existing test reads the status after `adopt_render` once the control half has moved on.

**Gates run (all green):**
- `cargo test --locked -p host-web --all-features` and `cargo test --locked -p host-web`. `worker_handoff` was also run eight more times on its own, green each time.
- `cargo clippy --locked -p host-web --all-targets -- -D warnings`, with and without `--all-features`.
- `cargo fmt --check`, and `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-web`, with and without `--all-features`.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (298 marked regions in 48 files)`.
- The loom leg, `lifecycle_loom`.
- `bash scripts/test-web-audioworklet.sh`, exit 0.
- The wasm artifact, built with `bash scripts/build-web-audioworklet.sh --named-twin` on the pinned `nightly-2026-08-20`, with wabt 1.0.34 from the noble deb, unpacked locally. On it:
  - `scripts/check-web-audioworklet.sh --without-metadata-regeneration` exited 0. `host_adopt` and `host_release` keep closures of 1 and 2, with no allocator, free, drop glue or atomic wait.
  - `scripts/check-browser-expected-resources.py --artifacts` exited 0, and its direct oracle matched `expected.json`'s digests.
  - `scripts/check-scalar-oracle-absent.py` exited 0.
- The committed module pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`) is the release fingerprint, re-pinned only by a release (`scripts/build-web-audioworklet.sh` header, docs/RELEASE.md). It is not changed here.

**Not run locally:** the Playwright browser matrix (Chromium and WebKit, `single` and `worker`). This machine has no PulseAudio, and the job's null sink needs it. CI's `browser` job runs the matrix on the merge. The JavaScript is unchanged. Its Worker paths (`finishDispose`, `reclaimOrphan`) call the same exports. `reclaimOrphan`'s adoption can now be refused, but only for a disposed session's half that a sweep already freed. The Worker then disposes nothing, and nothing is left to free.
