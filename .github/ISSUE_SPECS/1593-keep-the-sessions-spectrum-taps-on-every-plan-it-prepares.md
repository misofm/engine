# Keep the session's spectrum taps on every plan it prepares

Slice S30 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C9 and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

Split from the planner's S30: this slice makes the spectrum request core monitor state that rides
every plan; #1594 adds the `SPECTRUM_CONFIGURE` message that changes it at run time.

## Product outcome

A session's spectrum taps are the core's monitor state, not a host's boot option. The control
plane gives them to every plan it prepares, the first plan and every successor, on both hosts. A
structural edit keeps every tap whose target is still there, with its open window and stream
history (D15-14: no restart at a swap), and drops a tap whose strip or output the edit removes; it
never refuses the edit for it. On the C ABI the spectrum readers live on the control thread, so
the next slices can read them with a command.

## Context

- **Preparation today.** The C ABI prepares through `prepare_runtime`
  (`crates/control-plane/src/compile.rs:598`), which calls `prepare_host_runtime_with_live_lanes`
  or `prepare_host_runtime_with_live_lanes_successor` (`:622`, `:631`;
  `crates/host-core/src/prepare.rs:961`, `:990`). The successor entry passes no spectrum
  (`prepare.rs:1006`). The browser's spectrum entries (`prepare.rs:1022`, `:1056`) take no design
  bound cache (`None` at `:1040`), which the C ABI entries take. All entries funnel into one policy
  function (`prepare.rs:1387`); it builds the spectrum observers into the graph bindings
  (`:1979-2023`) and keeps a private capture enum (`prepare.rs:58`). An unknown target is refused
  `host.spectrum.target` (`prepare.rs:3210`).
- **The session.** `SessionState` (`crates/control-plane/src/control.rs:356`) is built by
  `compile_children` (`compile.rs:751`; first preparation `:779`). The structural arm
  (`control.rs:1120-1323`) prepares the successor (`:1149`), refuses while a candidate is pending
  (`:1216`) or while the active epoch lags (`:1127`), commits the protocol token (`:1289`), moves the
  persisting source producers at once (`adopt_persisting`, `:1298-1300`) and pushes the candidate
  provider (`:1314`). `synchronize_plan_epochs` (`:974`) promotes a candidate after render adopted
  it (`promote_pending_provider`, `:1019`). `ProviderEpoch` is `control.rs:29`.
- **Open streams this slice builds on** (all expected on `main` before the C13 phase):
  - *Prepare through an adapter-supplied preparer in the control-plane crate* (#1400): the
    `RuntimePreparer` trait (D1), capi's `CapiPreparer` (D3) and host-web's `WebRuntimePreparer`,
    which holds the browser's spectrum request and puts its capture in `RenderCompanions` (D5).
  - *Prepare every browser preparation branch concurrently, as a successor too, in host-core*
    (#1401): spectrum successor entries; its D4 refuses a successor whose target is missing.
  - *Carry spectrum capture state across a plan swap* (#1395): the carry key (D1), the move of
    the observer at the swap block (D2) and the pairing calls `pair_carried` (D3). Its D5 says the
    C ABI has nothing to carry.
  - *Carry meter and effect observation state across a plan swap* (#1327): `ObserverCarry` and
    `ObserverKind` (D3).
  - *Swap and retire browser plans through the Worker's service loop* (#1381): the browser's
    `SessionState` in the Worker (D1).
- **Monitor class** (ruling C9): spectrum configuration is not in the session model, is never
  saved and never advances the revision.
- **#1591 and #1592**: `SpectrumPreparation`, `SpectrumService`, `SpectrumAnalysis`.

## Decisions frozen for this slice

- **D1. The monitor state.**
  - host-core: `pub struct SpectrumConfiguration { pub preparation: SpectrumPreparation, pub hop: Option<SpectrumHop> }`.
  - control-plane, new file `crates/control-plane/src/spectrum.rs`: `SpectrumMonitor` with
    `configuration: Option<SpectrumConfiguration>`, `configured_epoch: u64` (0 with no
    configuration; +1 at every change), `adopted_epoch: u64` (the configuration epoch of the
    captures the service reads), `service: Option<SpectrumService>` and one `SpectrumAnalysis`.
    `SessionState` owns one. It is not in the session model or a snapshot, and no change of it
    advances the revision.
- **D2. Construction.** `compile_children` gains `spectrum: Option<SpectrumConfiguration>`. The
  first plan is prepared with it; `configured_epoch` and `adopted_epoch` start at 1 with a
  configuration, 0 without. capi passes `None` (the C ABI has no boot input for it; #1594 adds the
  message). The browser's Worker boot (#1381 D1) passes the request and hop of its boot options.
  This parameter is forced by order, not a placeholder: #1596 moves the browser to
  `SPECTRUM_CONFIGURE` and deletes it.
- **D3. One preparation input.**
  - `RuntimePreparer::prepare` gains `spectrum: Option<&SpectrumPreparation>` and returns
    `Option<PreparedSpectrumCaptures>` beside the host and the handles. The session passes its
    current preparation to every call: the first plan and every successor.
  - host-core gains one entry, `prepare_host_runtime_with_live_lanes_and_spectrum(compiled, caps,
    live_controls, lanes, spectrum: Option<&SpectrumPreparation>, successor:
    Option<SuccessorBase<'_>>, bound_cache: Option<&mut InputBoundCache>, superseded:
    &LiveValuesSuperseded)`, returning the host, the handles and the captures. It calls the policy
    function once and duplicates no preparation logic. `CapiPreparer` calls it in place of its two
    present calls; its lanes and live request are whatever #1554 leaves. The private enum at
    `prepare.rs:58` is replaced by #1591's `PreparedSpectrumCaptures`.
  - `WebRuntimePreparer` takes the request from the argument instead of holding it, and keeps its
    capture in `RenderCompanions` (it returns `None`) until #1596 moves the browser's readers.
  - The hop is not a preparation input. The monitor gives it to the service.
- **D4. The drop rule.** Before a structural successor is prepared, the session computes
  `SpectrumConfiguration::retain_present(&self, model: &SessionModel) -> Option<SpectrumConfiguration>`
  over the prospective model:
  - a `TrackPostInputBuiltins` or `TrackPostMatrix` entry stays when its ID names a track or a
    submix of the model; an `Output` entry stays when its ID names an output;
  - the order of the kept entries, the budget, the form and the hop stay; a configuration with no
    kept entry becomes `None`;
  - the successor is prepared with the result. The session stores it, and advances
    `configured_epoch` when it differs, only after the protocol commit, beside the candidate
    provider push. A refused or abandoned transaction leaves the configuration unchanged.
  - A structural edit is never refused for a spectrum entry. host-core's `host.spectrum.target`
    refusal stays as an invariant check; if it fires, the command answers `CommandError::Internal`.
- **D5. Readers on the control thread (C ABI).** The candidate `ProviderEpoch` holds the
  successor's captures, its configuration epoch and a pairing list made at preparation: one
  `ObserverCarry { kind: ObserverKind::Spectrum, successor, predecessor }` for each successor entry
  whose `(SpectrumTarget, SpectrumChannels)` equals an entry of the service's captures (#1395 D1).
  When `promote_pending_provider` makes the candidate current, the monitor calls #1395 D3's
  `pair_carried` with that list, makes the successor's captures the service's, and sets
  `adopted_epoch`. This runs on the control thread after render adopted the plan, so a superseded
  candidate is never paired (#1327 D3's reason). It relies on the adoption carrying, as the source
  producers already do (`control.rs:1298-1300`), because the session publishes a candidate only
  against the plan render runs (`control.rs:1127`, `:1216`).
- **D6. Service state across a swap.** The snapshot token is kept. When the selected entry
  carried, the selection, `selection_epoch`, an active stream and its analysis history are kept.
  When it did not: the selection is empty, `selection_epoch` advances by 1, an active stream stops
  (its next read answers `Inactive`) and the history resets. A caller's arm on an entry that did
  not carry is dropped, and the next one-shot read answers once with a new error `Dropped`
  (`spectrum.capture.dropped`; host-web maps it to `RESULT_WRONG_STATE`). A `None` configuration
  removes the service; every read answers `Unprepared`.
- **D7. Reads before adoption.** Until promotion the service keeps the captures of the plan that
  render runs; reads use them and report the old `adopted_epoch` (#1595). An entry that the new
  configuration adds is not readable until promotion.
- **D8. Realtime.** Nothing new runs on render. The observer move at the swap block is #1395's.
  The pairing and the service swap run on the control thread.

## Deliverables

1. host-core: D1's type, `retain_present`, D3's entry and enum swap, the `Dropped` variant, the
   service's adoption of captures with a pairing list.
2. control-plane: `crates/control-plane/src/spectrum.rs`, the fields on `SessionState` and
   `ProviderEpoch`, D2 and D3 in `compile.rs`, D4 in the structural arm, D5 in
   `promote_pending_provider`, and test-support accessors `test_spectrum(&self)` and
   `test_spectrum_mut(&mut self)` that return the monitor's service and analysis.
3. D3's trait change and its two implementations; the browser Worker boot passes its request.
4. The tests below; `crates/capi/src/runtime/spectrum_tests.rs` is new.

## Authorized paths

- `crates/host-core/src/{spectrum_service.rs,prepare.rs,lib.rs}`, `crates/host-core/tests/spectrum_service.rs`.
- `crates/control-plane/src/**`.
- `crates/capi/src/` (the preparer and `runtime/mod.rs`), `crates/capi/src/runtime/spectrum_tests.rs` (new).
- `hosts/host-web/src/` (the preparer and the Worker boot call only), `hosts/host-web/tests/`.

## Non-goals

- No wire message (#1594, #1595). No browser read change (#1596). No edit of `spectrum.rs`.
- No change of `CompileLimits` or any C symbol.

## Objective gates

Commands: `cargo test --locked -p host-core --features host-core/test-support`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --features host-web/test-support`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`,
`bash scripts/check-host-core-policy.sh`, `cargo run --locked --release -q -p realtime-policy`,
`bash scripts/check-capi-abi.sh`, `bash scripts/check-workspace-policy.sh`.

Fixture: tracks `t1`, `t2`, output `main`, 48 kHz, quantum 128, a sine on each track's source. The
C ABI tests build the session with a test constructor that passes a configuration, and read
through `test_spectrum_mut()`.

1. **The first plan.** Single `(t1 post-pan, Stereo)`: arm, render 2,048 frames, read. The window
   equals, field by field, the window of `prepare_host_runtime_with_spectrum` on the same document
   fed the same PCM from frame 0.
2. **A structural edit carries.** Collection `(t1 post-pan, Stereo)`, `(t2 post-input, Left)`,
   `t1` selected, stream started at 100 ms. A transaction adds a muted track `t0` (path
   `rebuild`). Reading after every block: no read after the first window is `Failed`, `Gap` or
   `Warming`; `capture_epoch` and `analysis_epoch` do not change; `sequence` has no gap; every
   window equals a reference session that had `t0` from the start, fed the same PCM, started at
   the same block.
3. **The drop rule.** Removing `t2` is accepted; the configuration has one entry;
   `configured_epoch` advances by 1; `t1`'s stream goes on as in gate 2. A caller's arm on `t2`
   before its removal (with `t2` selected for one-shot) reads `Dropped` once after promotion.
   Removing `t1` then is accepted; the configuration is `None`; every read is `Unprepared`.
4. **All or nothing.** A transaction that removes `t2` and holds an out-of-domain edit is refused;
   the configuration, both epochs, the service and its captures are unchanged, and the next window
   still reads.
5. **Monitor class.** Gates 2 and 3 advance the revision only by their transactions;
   `SESSION_SNAPSHOT_GET` bytes hold no spectrum.
6. **Realtime.** Around every render call of gates 2 and 3, swap blocks included, the render
   thread's `bench_support::alloc` counters read `allocations == 0 && frees == 0`.
7. **No spectrum, no change.** With `None`, every existing capi test passes and `audit capi`
   reports the merge base's `pcm_digest` (PR evidence).
8. **The browser preparer.** A host-web test boots the Worker session with a spectrum boot
   option: `WebRuntimePreparer` receives the request through the trait, and the existing browser
   spectrum read returns a window. A transaction through #1562's entry that removes the target's
   strip is accepted, and the successor has no spectrum observer.

## Test value

- Gate 1 turns red if the session's configuration does not reach the first plan's graph.
- Gate 2 turns red if a successor is prepared without the taps, if the readers are not paired at
  promotion, or if service state resets at a swap.
- Gate 3 turns red if a removed target refuses the edit, survives in the configuration, or drops a
  caller's arm silently.
- Gate 4 turns red if the drop is stored before the commit.
- Gate 5 turns red if spectrum state leaks into the model, the revision or the snapshot.
- Gate 6 turns red if the carry or the pairing allocates on render.
- Gate 7 turns red if the new preparation entry changes a session with no spectrum.
- Gate 8 turns red if the browser preparer keeps its own copy of the request or refuses the edit.

## Dependencies

- #1591, #1592.
- *Prepare through an adapter-supplied preparer in the control-plane crate* (#1400).
- *Prepare every browser preparation branch concurrently, as a successor too, in host-core* (#1401).
- *Carry spectrum capture state across a plan swap* (#1395), and through it *Carry meter and
  effect observation state across a plan swap* (#1327). Without #1395 every rebuild would restart
  the capture, which D15-14 forbids.
- *Swap and retire browser plans through the Worker's service loop* (#1381) and #1562 (the browser
  path of gate 8).
- `crates/capi` is stream B's: rebase after stream B's open edits.
- #1609 (make the spectrum one-shot handoff safe across threads), root 2026-10-14.

## Supersedes and shrinks

- Amends #1401 D4, #1400 D1 and D5, and #1395 D5 (see the amendment entries).

## Decision record

- 2026-10-24, root: split into T1 (host-core types, the one entry, `Dropped`), T2a (monitor,
  trait, first plan, `CapiPreparer`), T2b (drop rule, promotion pairing) and T3 (browser passes
  its configuration; the four old spectrum entries are deleted). Gates 1-6 live in
  `crates/control-plane/tests/spectrum_monitor.rs`; no capi spectrum test until #1595.
- T1: the entry takes no `bound_cache`: `InputBoundCache` no longer exists on `main`, so its
  signature is D3's without it. The four `*_with_live_controls_and_spectrum*` entries and
  `prepare_host_runtime_with_spectrum*` clone their borrowed request into a
  `SpectrumPreparation` until T3 deletes or folds them (control thread only).
- T1, D6: `Dropped` answers any caller arm a swap did not carry, single or collection, once, on
  the next one-shot read; the arm is released at promotion. It replaces #1618's
  `CaptureInvalid` (single) and `NotArmed` (collection) answers there, so their tests in
  `src/spectrum.rs` and `tests/successor_swap.rs` change their expectation, and host-web maps it
  to `RESULT_WRONG_STATE` (R3; the match is exhaustive). A single capture's generation still
  carries over, so the next arm is a later generation.
