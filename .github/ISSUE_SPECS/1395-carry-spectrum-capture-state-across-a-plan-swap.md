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
- **D4. A warm successor carries by the same move.** A warm successor (*Grow latency during
  playback by adopting a primed warm successor*, #1287) renders no block before its adoption at
  `S` (D15-8 (round-5 amendment)). D2's section runs in that adoption block, as at any swap, so a
  carried observer's `expected_block_sample` is the adoption block's start and no window is
  published twice (#1327 D4). This slice adds no other mode.
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

- No browser code (#1381's hook, #1290's gate). No change to the spectrum kernels, cadence rules
  or read API.
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
   - `bash scripts/check-host-core-policy.sh`, `cargo run --locked --release -q -p realtime-policy`,
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

**Decision 17 amendment (2026-10-13).** D1 and D2 are unchanged. D5's "The C ABI has nothing to carry" ends with #1593: the C ABI gains spectrum captures, and its control plane pairs them with D3's `pair_carried` when it promotes the adopted candidate, on the control thread, from a pairing list built by D1's key at preparation. #1596 moves the browser to the same control-side pairing, so from then on no host pairs spectrum readers in the swap block.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Checked against `12d4a4f9d`.

- **Anchors moved.** In `crates/host-core/src/spectrum.rs`: `SPECTRUM_OBSERVER_HANDLE` `:31`,
  `SpectrumTarget` `:204`, `SpectrumCapture` `:525`, `SpectrumCaptureCollection` `:967`, `select`
  `:1088`, the observer's `impl GraphRuntimeObserver` `:2497` (its `block_end` `:2543`, from
  #1609). The successor entries exist since #1401: `prepare.rs:1201` (single) and `:1233`
  (collection).
- **Where the readers live now.** Since #1591 and #1592 the captures are held by host-core's
  `SpectrumService` (`crates/host-core/src/spectrum_service.rs:585`), on the control side. Since
  #1381a the browser moves its capture out of the plan's attachment into it at boot. So D3's "a host
  whose captures live inside the plan calls them in the swap block" fits no host, and D5's browser
  sentence is void. **D5 now reads:** both hosts pair on the control side when the adopted
  candidate is promoted, as #1593 D5 states for the C ABI. This slice adds that pairing to
  `SpectrumService` as one method (the successor's prepared captures in, D3's rules applied, the
  result installed), built from D3's two functions. The browser's call is #1290 (d); the C ABI's is
  #1593.
- **The pairing list.** The swap block writes the carry record into the plan, which the control
  side cannot read in Worker mode. The control side builds the same list at preparation from D1's
  key, and applies it at promotion when the adoption carried; when the hand-over was refused
  (`CarryOutcome::PredecessorMismatch`, counted by `RealtimePlanOwner::carry_mismatch_count`) it
  installs the successor's fresh captures. Gates 1-3 call the `SpectrumService` method after the
  adoption block instead of calling `pair_carried` in it.
- **One-shot conflict with #1618 (open).** #1618 recommends settling an in-flight one-shot capture
  to `Invalid` at a swap (its option A). Gate 3's last sentence carries it instead. Recommendation:
  a carried observer keeps its one-shot state (D2 moves the whole observer, so it completes and
  gate 3 stands); #1618's settle applies only to a capture that does not carry (its key changed or
  it was removed) and to a refused hand-over. #1618 lands after this slice, rescoped so; root
  rules.
- **Engine edit.** `ObserverKind::Spectrum` is declared by #1327a if root accepts the fold in
  #1327's batch 10 amendment; then this slice edits no engine file. Otherwise the variant is this
  slice's one `plan.rs` edit, under its own one-attempt rule.
- **Split (plan v2).** #1395a: the single capture (gates 1, 3's single part, 4); #1395b: the
  collection (gate 2, 3's collection part).
- **Dependencies.** #1327a (the table, the record and the section). #1401 is closed.

## Amendment (controls coordinator, batch 12, 2026-10-15)

Checked against `ea336c039`.

- **No engine edit.** #1327a declared `ObserverKind::Spectrum` (`crates/engine/src/realtime/plan.rs:548-555`).
  This slice edits nothing under `crates/engine/src/realtime`.
- **The observer table is generic.** `GraphRuntimeObserver::carry_kind` returns
  `Option<ObserverKind>` (default `None`, `crates/graph/src/lib.rs:2887`). `GraphObserverRow` and
  `GraphObserverMove` carry their kind (`:3519`, `:3533`), and the move section writes the kind of
  each move into the record. So D2 is: `SpectrumCaptureObserver` (`spectrum.rs:2497`) answers
  `Some(Spectrum)`, and host-core gives spectrum rows keyed by D1, as `MeterInventoryRow` and
  `resolve_meter_carry` do for meters (`crates/host-core/src/prepare.rs:3152`). A graph edit is still
  "only if needed". The spectrum observer's binding handle is positional
  (`SPECTRUM_OBSERVER_HANDLE - index`, `spectrum.rs:31`). So when a collection is reordered, the
  observer swapped in must take its new binding's handle (`adopt_binding_handle`), as the meter
  observer does.
- **D3 and D5 as amended in batch 10 still hold.** Pairing happens on the control side when the
  adopted candidate is promoted, as one `SpectrumService` method (`spectrum_service.rs:585`) that
  uses D3's two functions. No `pair_carried` exists yet. Add `crates/host-core/src/spectrum_service.rs`
  to the authorized paths (the batch 10 amendment requires it, but the list was not changed).
- **Where the browser's successor capture is.** The boot plan's capture is moved into
  `SpectrumService` (`take_spectrum_capture`, `hosts/host-web/src/lib.rs:5139`, called at `:5932`).
  A successor's capture stays in the successor's `RenderCompanions` (`:5055`). #1290 (d) takes it
  out at promotion, in the service step after the adoption block, and calls this slice's method.
  Between adoption and promotion, a carried entry is served by the capture that the service
  already holds, because its observer was swapped. An entry that is not carried reads as pending
  until promotion.
- **A refused hand-over.** `RealtimePlanOwner::carry_mismatch_count`
  (`crates/engine/src/realtime/plan_exchange.rs:719`) is readable at promotion only when the
  control and render halves are one instance (single mode, the only browser mode today). The C ABI's
  #1593 and the browser's Worker mode (#1332) must publish it to the control side. Record this as
  a rule for those slices. This slice does not change it.
- **Anchors.** The batch 10 anchors in `spectrum.rs` are unchanged. The successor entries are now at
  `prepare.rs:1339` (single) and `:1371` (collection).
- **#1618** lands after #1395b, as root ruled. Its settle is part of the same `SpectrumService`
  method: an entry that is not carried, or all entries after a refused hand-over. So
  `spectrum_service.rs` is edited in this order: #1395a, #1395b, #1618.
- **Order in lane K.** #1395a, then #1395b, after #1584 D6a. Both slices edit
  `crates/host-core/src/prepare.rs` and `crates/host-core/tests/successor_swap.rs`.

## Decision record: #1395a, the single capture (batch 8)

**Shipped.** A single spectrum capture carries across a plan swap, and the control side keeps
reading it.

- *Swap block (D1, D2).* `SpectrumCaptureObserver` answers `carry_kind() = Some(Spectrum)`
  (`spectrum.rs`). `prepare.rs` gives each single capture an inventory row keyed by
  `(target, channels)` (`SpectrumInventoryRow`), and `resolve_spectrum_carry` joins the rows with the
  predecessor's by key, never by entry or handle, and only at equal sample rate and quantum (both
  are now in `PlanStateInventory`). The moves go into the one observer section with the meters'
  (`install_observer_carry`), and a carry program is requested when a capture may carry
  (`spectrum_may_carry`). No engine, graph or `latest_cell` edit. The observer keeps no binding
  handle, so `adopt_binding_handle` has nothing to do (a collection's reorder in #1395b needs no
  handle work either).
- *Pairing at promotion (D3, D5, batch-10 amendment).* The pairing list is built at preparation
  from the same join (`PlanStateInventory::spectrum_pairing`, `SpectrumPair`), so it names exactly
  the observers the swap block moves. `SpectrumCapture::pair_carried(&mut self, predecessor,
  pairs)` swaps the two captures only when the list names entry 0 of both and their target and
  channels are equal. `SpectrumService::adopt_successor(successor, pairing, carried)` applies it
  when the adoption carried, installs the successor's fresh captures otherwise (a refused
  hand-over), and returns the displaced captures so the caller drops them off the audio thread.
  The service is allocation- and free-free here. No host pairs in the swap block.
- *Departures.* (1) `pair_carried` takes the control-side pairing list, not the swap block's
  `ObserverCarry` slice (the record exists and a test proves it equals the list). (2) The
  inventory carries only single captures' rows: a collection has none, so it never moves an
  observer; #1395b adds its rows and the collection arm of `adopt_successor`. (3) An in-flight
  arm or stream on a capture that does not carry is not settled by `adopt_successor`; that is
  #1618's, in this method. (4) `browser_successor.rs` gate 2 read the successor's own capture; that
  observer is now carried away, so the test pairs the captures first (a mechanical update).

**Gates -> tests** (`crates/host-core/tests/successor_swap.rs`, `mod spectrum_carry`):
gate 1 `a_continuous_capture_carries_and_its_reader_keeps_receiving` (status, window fields and
planes equal the reference's block by block, epoch 1, no gap; record == pairing list);
`a_refused_hand_over_installs_the_fresh_capture` (the oracle can fail);
gate 3 `an_armed_one_shot_completes_across_the_swap` (a window that spans the swap, equal to the
reference's) and `a_changed_key_starts_fresh`; gate 4
`the_swap_block_and_the_promotion_allocate_and_free_nothing` (zero allocator calls in every
block and in `adopt_successor`). Unit tests: `spectrum::tests::pair_carried_swaps_only_a_named_pair_of_one_key`,
`prepare::tests::a_spectrum_capture_carries_by_key_at_one_shape`. Gate 2 (collection) is #1395b.
