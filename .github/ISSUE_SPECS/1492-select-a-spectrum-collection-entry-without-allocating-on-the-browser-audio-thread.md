# Select a spectrum collection entry without allocating on the browser audio thread

Stream H follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10). Filed
2026-10-09 by root from the #1479 attempt-1 verdict, NIT 1 and its observation
(`/home/bl/misofm/submix-verdicts/1479-attempt1.md`).

Root's ruling (2026-10-09), verbatim:

> (3) #1479: select's target clone allocates on the browser audio thread in the worklet message
> handler: file a stream H issue to remove that allocation on the audio thread (or state that H
> #1332 removes it by moving message handling to the Worker, as a dependency, and gate it there).

**Choice: remove the allocation here; #1332 is not a dependency.** #1332 (*Run the browser control
plane in a Worker and keep the AudioWorklet render-only*) does not move spectrum selection off the
worklet: its Non-goals keep "commands, sources, meters, observations or spectrum" on the worklet
thread and require them to be allocation-free there, and its `single` mode (no cross-origin
isolation) keeps all control work in the worklet's message handler permanently. #1332 D3.5 also
puts every post-boot worklet export inside `render_locked`, so after #1332 this allocation would be
a render-locked allocation in `worker` mode. Removing it is needed in both modes; #1332 cannot gate
it.

## Problem (verified on `codex/d15-batch-misc` at `e9798393e`, which carries #1479)

- **Where select runs.** The worklet's `port.onmessage` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:266`)
  calls `miso_engine_web_v1_spectrum_stream_select` or `miso_engine_web_v1_spectrum_select`
  (`:1309-1315`). That handler runs on the AudioWorklet (audio rendering) thread, between quanta,
  outside `process()`.
- **Two allocations on that path.**
  1. The bridge builds an owned target from the staged identity: `select_spectrum_with_smoothing`
     (`hosts/host-web/src/ffi.rs:2813` ff.) calls `spectrum_target` (`:920`), which converts the
     `&str` into a `Box<str>` (`:924`) for `SpectrumTarget` (`crates/host-core/src/spectrum.rs:168-175`),
     then drops it.
  2. `SpectrumCaptureCollection::select` (`spectrum.rs:950` ff.) returns an owned
     `SpectrumCaptureCollectionEntry` built by `owned_entry` (`:871-877`), which clones the
     target's `Box<str>`. `AudioWorkletEngineHost::select_spectrum`
     (`hosts/host-web/src/lib.rs:2576-2595`) discards the `Ok` value.
- **Not counted.** Neither export wraps its body in `render_locked` (`ffi.rs:2939-2946`,
  `:2954-2969`), so `miso_engine_web_v1_render_allocation_count` does not see these. The ffi
  header (`ffi.rs:14-23`) does not list them. `tests/render_locked_staging.rs` phase 2 selects
  before it reads its `before` count (`:205-216`), so it never measures a select.
- **Callers.** `select` and `selection_would_change` (`spectrum.rs:1001`) are called only by
  host-web (`lib.rs:2589`, `:2615`) and tests (`crates/host-core/tests/spectrum.rs:749`, `:756`,
  `:761`, `:833`, `:839`, `:854`; `spectrum.rs` unit tests `:3359`, `:3381`, `:3671`, `:3687`;
  `hosts/host-web/src/tests.rs:8805`). The C ABI does not call them.

## Decisions

- **D1. Select by a borrowed target.** host-core gains `SpectrumTargetRef<'a>` (the three
  `SpectrumTarget` kinds over `&'a str`) and `SpectrumTarget::as_ref()`. `select` and
  `selection_would_change` take `SpectrumTargetRef<'_>` in place of `&SpectrumTarget`; the old
  signatures are replaced, not kept beside (no parallel API variants). Matching compares kind and
  identity string, as `==` on `SpectrumTarget` does today.
- **D2. Select returns the index.** `select` returns `Result<usize, SpectrumCaptureCollectionSelectionError>`,
  the selected entry's index in preparation order. `owned_entry` is removed. A caller that wants
  the entry reads `entry(index)` (borrowed). The `entry` doc's "builds it on its own thread"
  sentence and the `select` doc name the browser audio thread as a caller.
- **D3. The bridge builds no owned target on the select path.** `select_spectrum_with_smoothing`
  builds a `SpectrumTargetRef` from the staged bytes; `AudioWorkletEngineHost::select_spectrum` and
  `spectrum_selection_would_change` take `SpectrumTargetRef<'_>`. `spectrum_target` stays for its
  other (boot-time) callers.
- **D4. Both select exports are render-locked.** `miso_engine_web_v1_spectrum_select` and
  `miso_engine_web_v1_spectrum_stream_select` wrap their whole bodies in `render_locked`, and the
  `ffi.rs` header's set (`:14-23`, as H #1488 leaves it) names them. Behaviour and result codes are
  unchanged.
- **D5. Zero on the real code is the claim.** If the gate reads non-zero once D1-D4 are in (another
  allocation on the select path, for example in a continuous restart), the slice stops and reports
  the allocating call path to root; it does not loosen the gate or remove the wrap.

## Authorized paths

- `crates/host-core/src/spectrum.rs` (`SpectrumTargetRef`, `SpectrumTarget::as_ref`, `select`,
  `selection_would_change`, `owned_entry`'s removal, the `entry` doc, and the unit-test call sites;
  all outside every `REALTIME_POLICY` marked region) and `crates/host-core/src/lib.rs` (the
  re-export only)
- `crates/host-core/tests/spectrum.rs` (the `select` call sites only)
- `hosts/host-web/src/lib.rs` (`select_spectrum`, `spectrum_selection_would_change`)
- `hosts/host-web/src/ffi.rs` (`select_spectrum_with_smoothing`, the two select exports, the
  header's set)
- `hosts/host-web/src/tests.rs` (the `select_spectrum` call site only)
- `hosts/host-web/tests/render_locked_staging.rs` (phase 2 only)
- `hosts/host-web/MUTATIONS.md` (the new rows)
- `hosts/host-web/src/render_lock.rs` (the module header's exception list only; coordinator
  amendment after attempt 1, pending root's ratification)
- this spec

**Coordinator note (after attempt 1's verdict, MINOR 1 and NIT 1).** Attempt 1 made two edits
outside the original list. Both were needed to keep existing text true, and neither is reverted.
Root is asked to ratify both:

- `hosts/host-web/MUTATIONS.md` row `:563` (A #1479's phase 2 read mutation): the count changed
  from 4 to 6. #1488's wrap of `spectrum_stream_start` made 4 false, because that body also reads
  the channel mask. The verifier re-measured it: 4 at `1d294c700`, 6 at `40a1ead6f` and at
  `b0256b89d`, with the same failing assertion.
- `hosts/host-web/src/render_lock.rs` module header: without the edit it would still name the two
  selects as exceptions, which D4 made false. It is added above as a coordinator amendment.

## Non-goals

- The other post-boot spectrum control exports (`spectrum_arm`, `spectrum_cancel`,
  `spectrum_stream_start`, `spectrum_stream_stop`, `spectrum_selection_epoch`), which are also not
  render-locked today; H #1488's D2 export-by-export check records them.
- Moving selection to the Worker (#1332, #1382).
- Any change to the worklet JavaScript, the SDK or the wasm export set.

## Hazards

- `hosts/host-web/src/ffi.rs` header and `crates/host-core/src/spectrum.rs` are hot files
  (STREAMS hot-file rows); the later slice rebases.
- The `wasm32` release module changes; the artifact identity job reports it CHANGED, as expected.

## Objective gates

1. **Zero on the real code.** `tests/render_locked_staging.rs` phase 2 reads `before` before its
   first select, then: selects `output` (a change), selects `output` again (no change), renders,
   reads, starts the stream, renders, then calls `spectrum_stream_select` to the `trackPostPan`
   entry with a changed `smoothing_ms` (a change with a stream restart), renders and reads the
   stream. The count is unchanged at the end, and each select returns `RESULT_OK`.
   `cargo test --locked -p host-web --test render_locked_staging` passes.
2. **Red on each allocation (mutations, recorded; PR evidence).** Each applied alone, then
   reverted:
   - `select` builds and drops an owned entry (`let _ = self.entry(index).map(|(t, c)| (t.clone(), c));`): gate 1 red;
   - the bridge's select path builds a `SpectrumTarget` with `spectrum_target` and drops it: gate 1 red;
   - D4 reverted with the first mutation kept: gate 1 green (the wrap is what makes the count see it).
3. **Existing gates.** `cargo test --locked -p host-core`, `cargo test --locked -p host-web`
   (all targets), `cargo clippy --locked --workspace --all-targets -- -D warnings`,
   `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`,
   `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh` and the worklet
   chain (`scripts/build-web-audioworklet.sh --named-twin`, `scripts/check-web-audioworklet.sh`,
   `check-browser-expected-resources --artifacts`, `scripts/test-web-audioworklet.sh`; exact
   invocations from `.github/workflows/qualification.yml`) exit 0. The browser qualification's
   render-allocation counts stay zero.

*Test value.* Phase 2's select legs are red when `select` or the bridge's select path allocates
(a target clone or an owned target built from the staged identity) on the audio thread; today
phase 2 selects before its `before` read and neither select export is render-locked, so no test
counts a select.

## Evidence

- Gate 1 output; gate 2's three runs; gate 3 outputs.

## Dependencies

- After (other streams): A #1479 (`owned_entry` and `entry` as this spec cites them).
- After (same stream): H #1488 (the `ffi.rs` header's render-locked set; D4 adds to it). #1332 is
  not a dependency (see the choice above); if this slice lands first, #1332 D3.5 finds both
  select exports already wrapped.

## Standing rules for the implementer

- Work only from this body. Read the cited lines and #1479's verdict first.
- A test that greps source or prose is refused. Allocation counts use the module's own counting
  allocator in the existing one-test integration binary.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: half a day.

## Attempt record

### Attempt 1 (2026-10-09, worktree `wt-d15-misc2`, branch `codex/d15-batch-misc2`, on #1479 and #1488)

**Changes.**

- D1/D2 (`crates/host-core/src/spectrum.rs`, `lib.rs`): new `SpectrumTargetRef<'a>` (`Clone, Copy,
  Debug, Eq, PartialEq`; the three kinds over `&'a str`), re-exported, and `SpectrumTarget::as_ref()`.
  `select` and `selection_would_change` take `SpectrumTargetRef<'_>` (old signatures replaced).
  Both find the entry through one private `position` helper that compares
  `capture.target.as_ref() == target` and the mask. `select` returns `Result<usize, _>` (the
  index in preparation order). `owned_entry` is removed. The `entry` doc and the `select` doc
  name the browser audio thread. Unit-test and `tests/spectrum.rs` call sites pass
  `target.as_ref()`.
- D3 (`hosts/host-web/src/ffi.rs`, `lib.rs`): new `spectrum_target_ref(raw, id)` does the
  validation that `spectrum_target` did and borrows the identity. `spectrum_target` (boot-time
  callers only) now builds its owned target from it, so the validation stays in one place.
  `select_spectrum_with_smoothing` calls `spectrum_target_ref`. `select_spectrum` and
  `spectrum_selection_would_change` take `SpectrumTargetRef<'_>`. `src/tests.rs` passes
  `target.as_ref()`.
- D4: `miso_engine_web_v1_spectrum_select` and `miso_engine_web_v1_spectrum_stream_select` wrap
  their whole bodies in `render_locked`. The `ffi.rs` header adds them to the spectrum-observer
  bullet and has two named exceptions now (`dispose`, `render_allocation_count`). The
  `render_lock.rs` module header said "four named exceptions" and named the two selects; it now
  names the two exceptions only.
- Gate 1 (`tests/render_locked_staging.rs` phase 2): `before` is read right after boot, before
  the first select. The phase selects `output` twice (a change, then a repeat) and checks the
  count after each. It then renders, reads, starts the stream, renders and reads the stream, as
  before. Then it stream-selects the `trackPostPan` entry with `smoothing_ms` 50 (the stream
  started at 100), so the entry and the smoothing both change and the continuous cadence
  restarts. It checks the count, renders 64 quanta, reads the stream (`RESULT_OK`) and checks
  the count again. The module doc says why.
- `hosts/host-web/MUTATIONS.md`: three new rows (below). Row `:563` (A #1479's phase 2 read
  mutation) re-measured: it now reads 6, not 4. The reason: #1488 wrapped
  `spectrum_stream_start`, which also calls `AudioWorkletEngineHost::spectrum_channels`
  (`ffi.rs`, the stream-start body), so the mutated clone's two allocator calls there now count
  too. The failing assertion is unchanged. This edit is to A's row (STREAMS row 101 gives A row
  `:563`), and its only purpose is to keep the row's number true. Root can move it.

**D5.** Gate 1 reads zero with D1-D4 in, including the stream select's continuous restart. No
other allocation is on the select path.

**Gate 1.** `cargo test --locked -p host-web --test render_locked_staging`: 1 passed.

**Gate 2 (mutations; each applied alone and then reverted, with gate 1 green again after each):**

| Mutation | Result |
|---|---|
| `select` runs `let _ = self.entry(index).map(\|(t, c)\| (t.clone(), c));` after it finds the entry | red: `the first select allocated`, left 2, right 0 |
| `select_spectrum_with_smoothing` runs `let _ = spectrum_target(target, id);` before it borrows the target | red: `the first select allocated`, left 2, right 0 |
| the first mutation kept, and both exports unwrapped from `render_locked` (D4 reverted) | green (1 passed): the count sees the select's allocation only through the wrap |
| (re-check of row `:563`) `PreparedSpectrumCapture::channels`'s collection arm clones the selected target | red: `a collection capture's spectrum read allocated`, left 6, right 0 |

**Gate 3.**

- `cargo fmt --all -- --check`: exit 0.
- `cargo test --locked -p host-core`: all binaries ok, 0 failed.
- `cargo test --locked -p host-web --all-targets`: lib 185 passed (2 ignored); every integration
  binary (`render_locked_staging`, `render_locked_source`, `boot_transient_budget`,
  `retained_ceilings`) passed.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: exit 0.
- `bash scripts/check-workspace-policy.sh`: exit 0. `bash scripts/check-realtime-policy.sh`:
  `realtime policy: ok (89 marked regions in 25 files)`.
- `bash scripts/check-cross-targets.sh`: exit 0, `cross-target matrix: PASS` (the #1018 expected
  failures only).
- Worklet chain, qualification.yml's exact invocations:
  `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
  exit 0 (shipped module `9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939`,
  3129171 B; named twin `3d6c0068...`; CHANGED against the pin, as the Hazards expect);
  `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
  exit 0; `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
  exit 0; `bash scripts/test-web-audioworklet.sh` with a fresh private `TMPDIR` (the CI step's
  shape) exit 0 and nothing left in the `TMPDIR`.
- Browser qualification, Chromium only, run locally in the CI step's shape (private pulseaudio
  null sink; `npm run qualify -- --artifacts ... --sdk-root sdk --browser chromium --check-matrix --self-test-mutations`):
  exit 0, `all qualification gates passed (151.0.7922.34)`. Every `render-allocations` and
  `sdk-render-allocations` row reads 0, `spectrum-collection=0` included (that workload selects
  through the now render-locked exports). Firefox and WebKit run in CI only.
- `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps -p host-core -p host-web --features host-core/control-provider`: exit 0.

*Test value.* Phase 2's select legs are red when `select` or the bridge's select path allocates
(a cloned owned entry, or an owned target built from the staged identity) on the audio thread.
Before this slice, phase 2 read `before` after its select and neither select export was
render-locked, so no test counted a select. The two red mutations and the green control above
show this.

### Attempt 1 follow-ups (2026-10-09, after the PASS verdict)

**Changes.**

- NIT 4: `tests/render_locked_staging.rs` phase 2 has a fourth select leg. After the last stream
  read it stream-selects the same `trackPostPan` entry with `smoothing_ms` 25 (was 50), so only
  the smoothing changes and the bridge takes its `restart_spectrum_stream` branch. It checks the
  count, renders 64 quanta, reads the stream (`RESULT_OK`) and checks the count again. The module
  doc names the leg. `hosts/host-web/MUTATIONS.md` has a row for it, next to #1492's rows.
- NITs 2 and 3: the `render_lock.rs` header paragraph and the `ffi.rs` header's "named exceptions"
  paragraph are reflowed to 99 columns. The text is unchanged.
- MINOR 1 and NIT 1: the coordinator note and the `render_lock.rs` amendment under Authorized
  paths. Nothing is reverted.

**Mutation (applied alone, then reverted and the file touched before the rebuild).**

| Mutation | Result |
|---|---|
| the smoothing-only restart branch of `select_spectrum_with_smoothing` (before `restart_spectrum_stream`) runs `std::hint::black_box(vec![0_u8; 16]);` | red: `the smoothing-only stream select allocated`, left 2, right 0; reverted: green (1 passed) |

*Test value.* The new leg is red when the bridge allocates only in its smoothing-only restart
branch; attempt 1's legs never reach that branch, so no other test catches it (verdict NIT 4,
mutation `mrestart`: green before this leg).
