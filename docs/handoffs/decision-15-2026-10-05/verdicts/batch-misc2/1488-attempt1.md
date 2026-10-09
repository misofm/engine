PASS

# #1488 attempt 1 -- adversarial verdict

Commit `cca903769` (parent `1d294c700`), worktree `/home/bl/misofm/wt-d15-misc2`. I reviewed an exported tree
(`git archive cca903769`) and a second exported copy for mutations. I did not build in, edit or check out the
worktree. `git diff 1d294c700 cca903769` touches four files, all authorized: the spec, `hosts/host-web/src/ffi.rs`,
`hosts/host-web/src/render_lock.rs` (header only) and the new `hosts/host-web/tests/render_locked_source.rs`.
`docs/REALTIME_DEPENDENCY_POLICY.md` is correctly not edited (#1489 has not landed). The rebuilt shipped module is
`ccf7c664...f9`, the same as the implementer's.

Evidence (logs, mutation runners, browser runner): `/home/bl/misofm/submix-verdicts/evidence/1488-attempt1/`.

## Decisions

- **D1 holds.** All eleven exports start with `render_locked(|| ...)` around the whole body (script check over
  `ffi.rs`, `wrapcheck.py`). `git diff -w` shows only the wrap lines and one re-wrapped comment in
  `spectrum_stream_start`: no logic, result code or `SAFETY` text changed. Early `return`s that were at function
  level now return from the closure, which the function returns: same result. No wrapped export calls another
  render-locked export, so the `debug_assert!` "windows do not nest" cannot fire, and the release build cannot clear
  the flag early.
- **D2 holds (gate 4, checked independently).** I listed every `miso_engine_web_v1_*` call in both worklet files and
  mapped each to its method. Engine worklet: `initialize`, `stageSpectrumRequest`, `stageSpectrumCollectionRequest`,
  `bindLiveControls`, `writeBootOptions`, `readResources` and `failInitialization` run only from the constructor;
  every export they call is pre-boot. The post-boot calls (`receive()` and its handlers, `process()`,
  `postMeterFrame`, `readSpectrumStreamMetadata`) are: `render`, `command_submit`, `meter_poll`, the four staging
  reads, the post-boot staging accessors, `source_submit`, `source_seek`, `input_filters_config_copy`,
  `eq_target_config_copy`, `eq_target_config_ptr`, `prepared_command_submit`, `meter_lease`, `spectrum_arm`,
  `spectrum_cancel`, `spectrum_stream_start`, `spectrum_stream_stop`, plus `dispose`, `render_allocation_count`,
  `spectrum_select` and `spectrum_stream_select`. Every one is wrapped except those four, and each named reason is
  true: `dispose` frees the host and is also the boot-failure path (`:866`); `render_allocation_count` is the
  reader; the two selects allocate today, and #1492 D1-D4 removes the allocation and wraps them. Feed worklet:
  only `source_submit` (`:386`) and `source_seek` (`:445`), both wrapped. No export is called through a computed
  name or a stored function reference after boot. `spectrum_selection_epoch` has no call site in either worklet
  file, so the header sentence is true without it. The `ffi.rs` and `render_lock.rs` headers say exactly this.
- **D3 holds.** One test, own `RenderLockedAllocator<System>`, fresh thread, observation session. Each of the
  eleven exports is called through its export on its accepted path and a refused path, with an exact result code
  (one exception, NIT 2) and an unchanged count after each call. The five new `native_staging` helpers are inside
  the existing `#[cfg(not(target_family = "wasm"))]` module, they use only existing public host accessors, and
  nothing else calls them: the shipped wasm is not affected by them. The module changes only by the eleven wraps,
  as the Hazards expect.
- **D4 did not trigger.** Zero on the real code natively, and also in real browsers (below).

## Findings

No BLOCKER. No MAJOR.

**MINOR 1. The spec's Test-value sentence is not exactly true for `prepared_command_submit`** (spec lines
167-173; root's rule: every test-value claim in a spec must be true). "No existing test catches it for nine of
them: none calls them under a counting allocator" is true of the exports themselves, but not of the defect it
names. `prepared_command_submit` only forwards to `AudioWorkletEngineHost::submit_prepared_commands`
(`lib.rs:3032`). Two existing `test-support` lib tests measure that method on accepted batches with the lib's
counting allocator and assert zero: `tests::prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`
(`tests.rs:3625`) and `tests::prepared_mixed_eq_builtin_fader_commits_and_refuses_atomically` (`tests.rs:3957`).
CI runs them (`--features host-web/test-support`). Probe: one `Box::new(0u8)` at the top of
`submit_prepared_commands` turns both red, and `render_locked_source` red too (`mut3-summary.txt`,
`ts-host-submit_prepared_commands`). An allocation in the export's own body stays unique to the new test
(`ts-prepared_command_submit`: only `render_locked_source` red). The overlap with `render_locked_staging.rs` is
stated correctly: injections into `spectrum_arm` and `spectrum_stream_start` turn both binaries red, and an
injection on the NaN-refused path alone turns only `render_locked_source` red. Corrected sentence for root:

> *Test value.* `render_locked_source.rs` is red if any of the eleven exports of D1 allocates or frees inside its
> render-locked window, on its accepted or its tested refused path. For an allocation in an export's own body, no
> existing test catches it: none calls these exports under a counting allocator. Two existing tests overlap on
> accepted paths. `render_locked_staging.rs` calls `spectrum_arm` and `spectrum_stream_start` inside its measured
> span. The `test-support` lib tests `prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`
> and `prepared_mixed_eq_builtin_fader_commits_and_refuses_atomically` measure
> `AudioWorkletEngineHost::submit_prepared_commands`, which `prepared_command_submit` forwards to, on accepted
> batches. Only this binary catches an allocation on the tested refused paths (for example arm while a stream is
> active, start with NaN smoothing, a truncated companion).

**NIT 1. One `source_seek` call site is not in the lists.** The feed's attach node has a `prepare-seek` port
handler (`miso-engine-v1-pcm-feed-worklet.js:539-540`). It calls `prepareSharedSeeks` -> `applySharedSeek` ->
`source_seek` (`:445`) on the AudioWorklet thread. The `ffi.rs` header (`:27-28`, "the feed's `process()` and the
engine worklet's source and seek handlers") and the spec table (line 235, "F:445 `process()` via
`applySharedSeek`") do not name it. The export is wrapped, so the set is still true.

**NIT 2. One result check is a tautology.** `render_locked_source.rs:154` asserts
`unchanged(before, "eq_target_config_ptr", pointer, pointer)`. D3 asks for each exact result code. The comment
gives the reason (a native address may not fit `u32`), and the count check is the claim, so this is acceptable.

**NIT 3. "The only other mention" of `spectrum_selection_epoch` is not the only one** (spec Root's amendment
paragraph). It is also in the export-name lists in `scripts/check-web-audioworklet.sh:304`,
`scripts/check-abi-layout-v1.py:216` and `tools/parameter-metadata/src/abi_layout.rs:234`. None is a caller, so
the conclusion stands.

## Observations (no action in this issue)

- No gate defends the wrap itself. Gate 2's D1-reverted runs are green by design: if a later edit removes a wrap,
  nothing goes red until an allocation is also there, and then the browser count does not see it either.
- The coordinator's decision not to wrap `spectrum_selection_epoch` is correct (no worklet calls it). Root should
  see the note in the spec.

## Test value (one sentence per new test)

- `post_boot_control_exports_are_render_locked_and_allocation_free` (`render_locked_source.rs`): red when any of
  the eleven D1 exports allocates or frees inside its render-locked window on a tested path. That is unique for an
  allocation in the export's own body (all eleven) and on the tested refused paths. It overlaps with
  `render_locked_staging.rs` on the accepted paths of `spectrum_arm` and `spectrum_stream_start`, and with two
  `test-support` lib tests on `submit_prepared_commands`'s accepted path (MINOR 1).

No queue is touched, so the acked-batch question does not apply.

## Gates run (all on the exported tree, private target dir)

- **Gate 1.** `cargo test --locked -p host-web --test render_locked_source`: 1 passed.
- **Gate 2 (mutations, injection `let _keep = std::hint::black_box(Box::new(0u8));` first in the closure).**
  Injected, red with the named call, for all eleven: `source_submit` "(full quantum)", `source_seek`,
  `spectrum_arm`, `spectrum_cancel`, `spectrum_stream_start` "(refused: NaN smoothing)", `spectrum_stream_stop`,
  `prepared_command_submit` "(refused: truncated companion)", `meter_lease` "(take)", `eq_target_config_copy`
  "(refused: retired rack)", `eq_target_config_ptr`, `input_filters_config_copy`. These match the attempt record.
  D1 reverted (a block-local `render_locked` that only calls the body) with the injection kept: green for
  `source_submit`, `source_seek`, `spectrum_cancel` and `prepared_command_submit`. Reverted: green. Uniqueness
  (full `cargo test -p host-web`, with and without `test-support`): only `render_locked_source` red for
  `source_submit`, `source_seek`, `spectrum_cancel`, `spectrum_stream_stop`, `prepared_command_submit`,
  `meter_lease`, `eq_target_config_copy` and `input_filters_config_copy`; `render_locked_staging` red as well for
  `spectrum_arm` and `spectrum_stream_start` (as stated); NaN-refused-path-only injection: only
  `render_locked_source` red. Host-method probe: see MINOR 1.
- **Gate 3.** All exit 0: `cargo test --locked -p host-web` (lib 185 + 2 ignored, `boot_transient_budget` 2,
  `render_locked_source` 1, `render_locked_staging` 1, `retained_ceilings` 1); the same with
  `--features test-support` (lib 188); `cargo clippy --locked -p host-web --all-targets -- -D warnings` and with
  `--all-features`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p host-web --no-deps`;
  `cargo fmt --all -- --check`; `check-workspace-policy.sh` ("ok"); `check-realtime-policy.sh` ("ok (89 marked
  regions in 25 files)"); `check-cross-targets.sh` ("PASS"). Worklet chain with `qualification.yml`'s exact
  invocations: `build-web-audioworklet.sh --named-twin ...` (module `ccf7c664...f9`), `strip-wasm-names.py check`,
  `check-web-audioworklet.sh --without-metadata-regeneration ...`, `check-browser-expected-resources.py
  --artifacts ...`, `check-scalar-oracle-absent.py`, and `test-web-audioworklet.sh` under a fresh private TMPDIR
  (no leftovers).
- **Browser qualification (the implementer could not run it).** `npm run qualify -- --artifacts ... --sdk-root
  ... --browser <b> --check-matrix --self-test-mutations` with a private PulseAudio null sink, as CI does. Chromium
  151, Firefox 153 and WebKit 26.5: "all qualification gates passed". Every render-allocation row is 0 (corpus,
  live-control, observation, stall, staging reads) and every SDK row is 0 (spectrum queries, continuous streams,
  collection, live-bypass prepared commands). So the zero holds in real browsers with the eleven exports wrapped.
