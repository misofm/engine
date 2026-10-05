# Carry spectrum capture state across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-14, D15-7).
Code anchors verified on `main` at `6fb211594`.

The spectrum half of the former *Carry meter, observation and spectrum state across a plan swap*,
split for size in the decision-15 fix round. The meter and observation half is *Carry meter and
effect observation state across a plan swap* (#1327); this slice reuses its rules, its observer
location table and its program section.

## Product outcome

A browser page that shows a strip's or the output's spectrum sees no reset when a track is added
or another strip is edited. A spectrum capture whose target and channels are the same in both
plans keeps its analysis history, its open window, its stream epoch, its sequence and its mode
through a plan swap. The host's existing reader keeps receiving, with no `Failed` epoch, no `Gap`,
no `Warming` restart, and no window relabelled. This replaces #1269's P10 ("restart at the
swap"), which decision 15 rejects (D15-14). Only a changed or added capture starts fresh.

## Context

- **Observer.** `SpectrumCaptureObserver` (`crates/host-core/src/spectrum.rs:1441-1449`) holds the
  `Producer<SpectrumCapturedRecord>`, two `SPECTRUM_WINDOW_FRAMES` planes (2048 frames, `:24`), its
  `SpectrumChannels`, the shared `mode` word, the one-shot state (`SpectrumOneShotState`, `:1301`,
  with its shared `state` word) and the continuous state (`SpectrumContinuousState`, `:1311`, with
  its history ring, `expected_block_sample`, `next_window_start`, `sequence` and the shared
  `SpectrumContinuousShared`, `:1260`). It is a graph observer (`impl GraphRuntimeObserver`,
  `:2216`), bound by `GraphNodeObserverBinding { node, handle, observer: Box<dyn ...> }`
  (`crates/graph/src/lib.rs:2653-2657`).
- **Control side.** `SpectrumCapture` (`spectrum.rs:457-472`) holds the consumer, the same three
  shared words (`state`, `mode`, `shared`), its read counters, its `target` and `channels`.
  `SpectrumCaptureCollection` (`:803-808`) holds several captures, the `selected` index and a
  `selection_epoch`; `select` (`:885`) finds an entry by `(target, channels)`, and preparation
  refuses duplicate entries (`SpectrumPrepareError::DuplicateEntry`).
- **Keys.** `SpectrumTarget` (`:166-174`) names a track's post-input or post-matrix boundary, or an
  output, by ID. Observer handles are positional (`SPECTRUM_OBSERVER_HANDLE - index`,
  `:1420-1424`), so they are not keys.
- **A fresh observer restarts the stream.** A discontinuity resets the history and starts a new
  epoch (`SpectrumContinuousReadError::Failed`, `:1242-1246`). That is what a swap to a freshly
  prepared observer shows today.
- **Preparation.** `prepare_host_runtime_with_live_controls_and_spectrum` (`crates/host-core/src/prepare.rs:843`)
  returns a `SpectrumCapture`, and `..._and_spectrum_collection` (`:870`) a
  `SpectrumCaptureCollection`. Both build their observers at `:1676-1700`. Neither has a successor
  form on `main`: *Prepare every browser preparation branch concurrently, as a successor too,
  in host-core* (#1401) adds them.
- **The browser** holds the capture in its own `Single`/`Collection` wrapper
  (`hosts/host-web/src/lib.rs:1482-1487` reads `stream_epoch` through it).
- **The C ABI** has no spectrum capture, so it has nothing to carry.

## Decisions frozen for this slice

- **D1. Key and rule (P1).** A spectrum observer carries when its `SpectrumTarget` and
  `SpectrumChannels` are equal in both plans, and both plans have the same sample rate and render
  quantum. A collection entry is matched by `(target, channels)`, never by index or handle.
  Nothing in it is a live value, so there is no retarget. A non-carried observer starts fresh. It
  changes no audio and is not a strip restart.
- **D2. Move mode.** Register spectrum observers in #1327's observer location table under D1's key,
  with the new `ObserverKind::Spectrum`. A single capture is entry 0; a collection's entries are
  in the collection's entry order, so an entry's index is its capture's index. At the swap block,
  swap each carried observer between the plans: the whole `SpectrumCaptureObserver`, its producer
  and its three shared words included. Swapping the boxed observers is enough and
  allocation-free. A reader that already holds the carried capture therefore stays connected to
  the observer that renders on, and its mode word still controls it. The successor's fresh
  observer goes to the retiring predecessor. The section writes a `Spectrum` record in #1327
  D3's carry record for each carried observer.
- **D3. Reader pairing, at the swap block.** No capture is swapped on the control side, for
  #1327 D3's reason: a commit precedes adoption and can be superseded. host-core gives two
  pairing calls, allocation-free, dropping nothing, callable on the render thread:
  - `SpectrumCapture::pair_carried(&mut self, predecessor: &mut SpectrumCapture, carries:
    &[ObserverCarry])`, for a single capture: when the record has a `Spectrum` entry, the
    successor's capture and the predecessor's swap.
  - `SpectrumCaptureCollection::pair_carried(&mut self, predecessor: &mut Self, carries)`: each
    carried entry's `SpectrumCapture` swaps into the successor's collection at the successor's
    index for its key. When the predecessor's selected entry carried, the successor selects that
    entry's new index and keeps `selection_epoch`. When it did not carry, the successor has no
    selection and its `selection_epoch` is the predecessor's plus one; the host's next `select`
    arms an entry as it does today.

  A host whose captures live inside the plan calls them in the swap block from its attachment
  hook. The browser does: *Swap and retire browser plans through the Worker's service loop*
  (#1381) D3.
- **D4. Copy mode belongs to #1287 and #1354.** In a warm successor the observer renders the
  catch-up, which overlaps windows the predecessor publishes. *Snapshot a running plan into a
  returned successor at a block* (#1354, its D4) copies each carried spectrum observer's state
  under D1's key, and *Pre-roll a successor whose latency grows* (#1287, its W6) publishes from it
  only windows whose sequence the predecessor did not publish, exactly as #1327 D4 says for
  meters. This slice's program section is move-mode only.
- **D5. Both hosts.** Everything is in host-core and in #1327's graph table. The browser's capture
  lives in its plan's host attachment, and #1381 D3's hook calls D3's pairing in the swap block,
  so the browser has no other carry code. The C ABI has nothing to carry.

## Deliverables

1. D1 and D2: spectrum observers in #1327's location table and move section.
2. D3 in `crates/host-core/src/spectrum.rs` (the two `pair_carried` calls) and
   `crates/host-core/src/prepare.rs` (the key table). The `Spectrum` variant of `ObserverKind`
   in `crates/engine/src/realtime/plan.rs`.
3. Gates 1-4 in `crates/host-core/tests/successor_swap.rs`, through #1401's successor entry points.

## Authorized paths

- `crates/host-core/src/spectrum.rs` (observer registration, the two pairing calls and their unit
  tests only; no kernel change)
- `crates/host-core/src/prepare.rs` (the spectrum key table only)
- `crates/engine/src/realtime/plan.rs` (the `ObserverKind::Spectrum` variant only; outside stream
  A's ownership, root sequences it after #1327)
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (only if #1327's location table needs a
  spectrum owner kind)
- `crates/host-core/tests/successor_swap.rs`, `crates/host-core/tests/support/successor.rs`

## Non-goals

- No copy mode (D4). No browser code (#1381's hook, #1290's gate). No change to the spectrum kernels, cadence rules or
  read API.
- No meter or observation tap carry (#1327).

## Objective gates

1. **Continuous capture carries.** A has a single continuous capture on track `t`'s post-matrix
   boundary, both channels, started so that a window is open at the swap block. B adds a muted
   track whose ID sorts first. Read through A's `SpectrumCapture` after every block until the swap
   block; there, call `pair_carried` on B's and A's captures with B's `observer_carries()`, then
   read through B's capture. No read returns `Failed`, `Gap` or `Warming` after the stream first produces;
   `stream_epoch` is unchanged; `sequence` has no gap; and every window equals the reference's
   field by field (a fresh B with the same capture, fed from frame 0, started at the same block).
2. **Collection and mode.** A has a collection with entries `(t post-matrix, Stereo)` and
   `(u post-input, Left)`, the first selected and continuous. B lists them in the other order.
   After the swap and `pair_carried`, B's collection reads windows as in gate 1, with the first
   entry selected at its new index. `stop_continuous` then stops the rendering observer: no window
   is published after the next block. `select` on the second entry arms the successor's observer
   for it, and its first window equals the reference's.
3. **A changed key starts fresh.** B changes the channels of `(u post-input, Left)` to `Right` in
   its request. That entry's reader starts fresh, and `(t post-matrix, Stereo)` carries as in
   gate 1. A single one-shot capture armed before the swap and carried completes with a window
   whose `first_sample` precedes the swap block and equals the reference's.
4. **Realtime.** The swap block and the pairing calls make zero allocations and frees, measured with
   `bench_support::alloc`'s thread-scoped counters after warm-up.
5. Commands:
   - `cargo test --locked -p engine -p graph -p host-core --features graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the carry swaps the state but keeps the successor's producer (the host
  reads a dead queue), if the history is lost (a `Failed` epoch), or if the open window is
  dropped or relabelled (a sequence gap).
- Gate 2 turns red if the shared mode word stays with the retired observer, so the host's stop or
  select no longer reaches the rendering one, or if entries are paired by position (B reverses
  the order).
- Gate 3 turns red if the rule ignores the channels and carries a window of the wrong planes, or
  if a pending one-shot is lost at the swap.
- Gate 4 turns red if the swap allocates, frees or drops a boxed observer on the render thread.

## Dependencies

- *Carry meter and effect observation state across a plan swap* (#1327), for the observer
  location table, the move section and the carry record.
- *Prepare every browser preparation branch concurrently, as a successor too, in host-core* (#1401),
  for the successor entry points the gates use.
