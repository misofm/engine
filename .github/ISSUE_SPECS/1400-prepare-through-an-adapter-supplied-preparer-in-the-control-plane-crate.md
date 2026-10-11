# Prepare through an adapter-supplied preparer in the control-plane crate

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11).
Split from *Swap and retire browser plans through the Worker's service loop* (#1381): this is its
former D1. Code anchors verified on `main` at `6fb211594`.

## Product outcome

The portable control plane prepares every plan, at boot and for every structural transaction,
through a preparer that the adapter supplies. The C ABI's preparer is today's preparation,
unchanged, so no C ABI bit or byte moves. The browser has its own preparer: it prepares each of
its three boot shapes (no spectrum, one spectrum capture, a spectrum collection) with concurrent
control delivery, and puts the plan's render-side companions inside the prepared plan. The
browser's boot already prepares through it, and its rendered bits do not change.

## Context

- **The preparation is fixed to the C ABI today.** `prepare_runtime`
  (`crates/control-plane/src/compile.rs:578`) validates the shape (`:587`), builds a live request
  with `LIVE_QUEUE_DEPTH` (`:17`, `:594-597`) and calls
  `prepare_host_runtime_with_live_lanes` or `_with_live_lanes_successor` with `C_ABI_LIVE_LANES`
  (`:25-28`, `:598-608`). The rest of it (`:609-693`) is the control plane's own accounting:
  `StripLanes`, the effect producers, `prepared_capi_resources` (`:313`) and the report row.
- Its two callers are the boot compile (`compile_children`, `compile.rs:721`) and the structural
  arm (`crates/control-plane/src/control.rs:1043`), which passes a `SuccessorBase`
  (`crates/host-core/src/prepare.rs:641`).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309) moves this code
  into `crates/control-plane` (lib `control_plane`) as it is. Its non-goal names this slice's
  trait as the place where each adapter supplies its own preparation.
- **The browser prepares differently.** `compile_ready` (`hosts/host-web/src/lib.rs:6617`)
  builds its live request from its options (`live_control_request`, `:7221`), selects one meter per
  strip (`:6626-6643`), and prepares through one of three branches (`:6644-6684`):
  `prepare_host_runtime_with_live_controls_and_spectrum`,
  `prepare_host_runtime_with_live_controls_and_spectrum_collection`, or
  `prepare_host_runtime_with_selected_meters_between_render_calls`. The last one uses
  `BetweenRenderCalls` delivery (host-core routes every selected-meter request there,
  `crates/host-core/src/prepare.rs:1503-1509`).
- **Render-side companions.** Preparation returns render-side halves next to the plan: the meter
  consumers (`HostLiveControlHandles::meters`) and effect observation handles
  (`HostLiveControlHandles::effect_observations`), plus the browser's spectrum capture
  (`PreparedSpectrumCapture`, `hosts/host-web/src/lib.rs:1400`). The browser keeps them in
  `ReadyOwnership` (`:1509`; `spectrum_capture` `:1578`, `meters` `:1582`,
  `effect_observations` `:1586`). The control plane ignores both handle vectors today.
- `PreparedRenderPlan` (`crates/engine/src/realtime/plan.rs:544`) has no slot for host data.
- *Prepare every browser preparation branch concurrently, as a successor too, in host-core* (#1401)
  adds the concurrent entries for all three browser branches, at boot and as a successor.

## Decisions frozen for this slice

- **D1. The trait.** In `crates/control-plane`:

  ```rust
  pub trait RuntimePreparer: Send {
      fn prepare(
          &self,
          compiled: &CompiledSession,
          caps: &HostPrepareCaps,
          successor: Option<SuccessorBase<'_>>,
      ) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics>;
  }
  ```

  It covers only the adapter's choice: the live request, the lane selection, meters, spectrum and
  companions. Everything else stays in the crate's `prepare_runtime`, unchanged and in the same
  order: the shape check before the call, then `StripLanes`, the effect producers, the resource
  projection and the limits after it.
- **D2. The session prepares through nothing else.** The session constructor takes
  `preparer: Box<dyn RuntimePreparer>` and stores it in `SessionState`. The boot compile and the
  structural arm call `prepare_runtime(&*self.preparer, ..)`. No other call into host-core's
  preparation remains in the crate.
- **D3. capi's preparer.** `CapiPreparer` (unit struct, in capi) is today's live request and lane
  selection: `LIVE_QUEUE_DEPTH` and `C_ABI_LIVE_LANES` move from the crate to capi beside it, and
  it calls the same two host-core entries. capi passes `Box::new(CapiPreparer)` to the
  constructor.
  - Order with the two slices that edit `C_ABI_LIVE_LANES`: *Deliver value-only send edits to the
    running C ABI plan* (#1225, `routes: true`) and *Apply value-only input trim and polarity
    edits to the running C ABI plan* (#1261, the input lane). This slice moves the constant
    verbatim, with whatever fields it holds when this slice merges. A slice of the two that merges
    first edits it in `crates/control-plane/src/compile.rs`, and this slice carries that edit
    unchanged. A slice that merges after this one edits it beside `CapiPreparer` in capi. Root
    rebases whichever lands second; neither order changes a value.
- **D4. The plan carries host data.** `PreparedRenderPlan` gains one field,
  `host_attachment: Option<Box<dyn HostAttachment>>`, `None` by default. `HostAttachment` is a new
  engine trait, `pub trait HostAttachment: Send + 'static { fn as_any_mut(&mut self) -> &mut dyn
  Any; }`. The plan gains `set_host_attachment(&mut self, value: Box<dyn HostAttachment>)`
  (control side, before publication) and `host_attachment_mut::<T: 'static>(&mut self) ->
  Option<&mut T>` (a downcast through `as_any_mut`). In this slice the engine only stores it and
  drops it with the plan. Reading it from render through the plan owner, and the swap-block hook
  that pairs carried readers, are #1381's (its D3).
- **D5. The browser's preparer.** `WebRuntimePreparer` in host-web holds the boot options and the
  spectrum request.
  - One inner function, `prepare_web_host(compiled, caps, options, spectrum, successor)`, holds
    `compile_ready`'s live request, meter selection and three branches. It calls #1401's
    concurrent entries, the successor ones when `successor` is `Some`, and never a
    `_between_render_calls` entry. It returns the host, the handles and the spectrum capture.
  - `WebRuntimePreparer::prepare` calls it, then moves `handles.meters`,
    `handles.effect_observations` and the spectrum capture into one `RenderCompanions` struct
    (those three fields; #1381 adds its bookkeeping), implements `HostAttachment` for it, and
    sets it on the plan with `set_host_attachment`. It returns the host and the remaining handles.
  - `compile_ready` calls `prepare_web_host` with `successor: None` and keeps the companions in
    `ReadyOwnership` as today. The browser renders its plan directly until #1381.
- **D6. No behaviour change on either host.** capi keeps every result, diagnostic and byte. The
  browser keeps every result, diagnostic and resource row. The no-spectrum branch's change from
  `BetweenRenderCalls` to concurrent delivery must not move a rendered bit (#1401 gate 3 holds the
  host-core side; gate 2 here holds the browser).

## Deliverables

1. D1-D2 in `crates/control-plane/src/` (the trait, the constructor parameter, the stored box).
2. D3 in `crates/capi/src/` (the preparer, the two constants, the constructor call).
3. D4 in `crates/engine/src/realtime/plan.rs`.
4. D5 in `hosts/host-web/src/lib.rs`, and the `control-plane` dependency in
   `hosts/host-web/Cargo.toml`.
5. Tests below.

## Authorized paths

- `crates/control-plane/src/`, `crates/control-plane/tests/runtime_preparer.rs` (new)
- `crates/capi/src/` (D3 only)
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`, `hosts/host-web/Cargo.toml` (the
  `control-plane` dependency only), `Cargo.lock`
- Outside stream H's ownership; root sequences them: `crates/engine/src/realtime/plan.rs` (D4
  only), `crates/control-plane/` and `crates/capi/` after #1309.

## Non-goals

- No browser `SessionState`, no Worker, no render through `RealtimePlanOwner`, no
  `active_attachment_mut` on the owner, no service step: #1381.
- No new host-core entry: #1401.
- No structural edit on the browser: *Replace the running browser session in the Rust host*
  (#1290).
- No change to the browser's resource report or its caps.

## Hazards

- **Fused fader-matrix bank.** `BetweenRenderCalls` fuses each strip's fader and matrix banks
  (`crates/builtins-compiler/src/lib.rs:1152-1169`); concurrent delivery does not. AGENTS.md
  says regrouping lanes never changes a bit. If gate 2 goes red, stop and report; do not re-pin a
  digest.
- **Drop order.** `ReadyOwnership`'s field order is load-bearing (`lib.rs:1506-1508`). Keep it.

## Objective gates

1. **C ABI unchanged.** `cargo test --locked -p capi` passes with no test changed.
   `cargo build --locked --release -p audit -p capi && target/release/audit capi` reports 0
   allocations, deallocations, locks, syscalls and violations, with the same `pcm_digest` as the
   base (PR evidence, not a committed pin).
2. **Browser bits unchanged.** The browser legs' native-digest gate and the `direct-oracle.mjs`
   parity pass unchanged in Chromium, Firefox and WebKit.
3. **The session prepares through the preparer.** `crates/control-plane/tests/runtime_preparer.rs`
   (feature `test-support`) builds a session with a test preparer that counts its calls and
   delegates to `prepare_host_runtime_with_live_lanes[_successor]`. Boot calls it once with
   `None`. One structural transaction (add a track) calls it once with `Some(base)`, and the
   base's inventory is the running plan's. A refused shape never calls it.
4. **The browser preparer, three branches.** In `hosts/host-web/src/tests.rs`, for each of no
   spectrum, one capture and a collection, at boot and as a successor of a nine-track session:
   - the returned plan's `host_attachment_mut::<RenderCompanions>()` is `Some`, with one meter
     consumer per strip and the branch's capture (none, single, collection);
   - the returned handles hold no meter and no observation handle;
   - the successor prepares the unchanged source vacant (its report's carried source bytes are
     nonzero).
5. **Commands:**
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     and `bash scripts/test-web-audioworklet.sh`
   - in `hosts/host-web/qualification`: `npm ci && npm run qualify -- --artifacts ../../../target/ci/qualification-artifacts --sdk-root ../../../sdk --browser chromium --check-matrix --self-test-mutations` (and `firefox`, `webkit`)
   - `bash scripts/check-cross-targets.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 3: turns red if the structural arm still calls host-core directly with the C ABI's lanes,
  which would give every browser successor C ABI lanes, no meters and no spectrum; or if a
  successor is prepared without the running plan's base, which would reallocate every ring.
- Gate 4: turns red if one branch still prepares between render calls or drops its capture, if a
  meter consumer stays in the handles (the control plane would drop it on the control thread and
  its meter would go silent), or if a successor branch ignores the base.
- Gates 1 and 2 are not new. They hold both hosts' bits across the refactor.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Prepare every browser preparation branch concurrently, as a successor too, in host-core* (#1401).
- *Give every browser plan live strip fader and mute lanes* (#1326): the `lanes` argument of the
  three browser entries.

Dependent: *Swap and retire browser plans through the Worker's service loop* (#1381).
