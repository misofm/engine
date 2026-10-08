# Give the spectrum capture collection no cloning accessor

Stream A follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10). Filed
2026-10-08 by root from the #1333 attempt-1 verdict, NIT 1. #1333 found a real render-thread
allocation: the browser's spectrum read took the selected entry's channel mask through
`SpectrumCaptureCollection::selected_entry()`, which clones the selected target. #1333 Amendment 1
(A1) moved the read to the new non-cloning `selected_channels()`; the cloning accessor stayed
public beside it with no caller.

No rendered bit moves. No allocation is added.

## Problem (verified on `main` at `1e78d7820`)

- **The accessors** (`crates/host-core/src/spectrum.rs`):
  - `entry(index) -> Option<SpectrumCaptureCollectionEntry>` (`:858-867`) clones the entry's
    `SpectrumTarget` (it holds a `String`).
  - `selected_entry()` (`:869-873`) is `self.selected.and_then(|index| self.entry(index))`, so it
    clones too.
  - `selected_target()` (`:875-879`) and `selected_channels()` (`:881-888`) borrow and copy; the
    latter's doc says the browser's audio thread reads it on every spectrum read.
- **Callers.** `selected_entry()` has no caller in the workspace. `entry()` has two, both in an
  integration test (`crates/host-core/tests/spectrum.rs:737-738`, comparing against cloned
  `SpectrumCaptureCollectionEntry` values). The browser's `PreparedSpectrumCapture::channels` and
  `::target` (`hosts/host-web/src/lib.rs:1493-1505`) use the borrowing accessors.
- **Why it matters.** A cloning accessor beside the borrowing ones is the shape of the #1333 defect:
  a later render-path edit that reaches for "the selected entry" allocates on the audio thread.
  The runtime proof (`hosts/host-web/tests/render_locked_staging.rs` phase 2, and the browser
  `render-allocations` gate) catches it only on the paths those tests drive.
- **The mutation record names the accessor.** `hosts/host-web/MUTATIONS.md:563` records phase 2's
  catch as "`PreparedSpectrumCapture::channels` back to `selected_entry().map(|entry|
  entry.channels)`". That mutation cannot be written once the accessor is gone.

## Decisions

- **D1. Remove `selected_entry()`.** It has no caller and is the cloning form of two borrowing
  accessors.
- **D2. `entry()` borrows.** `entry(index)` returns `Option<(&SpectrumTarget, SpectrumChannels)>`
  and clones nothing. The collection then has no accessor that allocates. A caller that needs an
  owned `SpectrumCaptureCollectionEntry` builds it itself, on its own thread. The two test
  assertions compare the borrowed pair with the expected entry's fields.
- **D3. The mutation record.** `MUTATIONS.md:563` is rewritten to an equivalent allocating
  mutation that still compiles (for example, in `PreparedSpectrumCapture::channels`,
  `Self::Collection(capture) => { let _ = capture.selected_target().cloned();
  capture.selected_channels() }`), and the run is repeated: phase 2 must still be red with its
  message. The row records the new mutation and its output.

## Authorized paths

- `crates/host-core/src/spectrum.rs` (`entry`, `selected_entry` and their docs; and, by the
  Amendment, body-only edits at `select`'s two `self.entry(index)` sites through one private
  helper)
- `crates/host-core/tests/spectrum.rs` (the two `entry` assertions at `:737-738` only)
- `hosts/host-web/MUTATIONS.md` (row `:563` only; stream H's file, by named exception)
- this spec

## Non-goals

- Any change to `selected_target`, `selected_channels`, selection, `cancel_except` or the capture
  drains.
- Any change to `SpectrumCaptureCollectionEntry` or to the request path that builds entries
  (`hosts/host-web/src/ffi.rs:1030`).
- The spectrum-capture carry across a plan swap (#1395).

## Hazards

- `crates/host-core/src/spectrum.rs` is a hot file (H #1449 → A #1327 → A #1395; J #1443's
  markers). This slice edits two accessors outside every marked region; it lands in any order with
  the others and the later slice rebases (STREAMS hot-file row).
- A caller added on another branch between filing and implementation must be moved to a borrowing
  accessor in this slice; list it in the attempt record.

## Objective gates

1. `cargo test --locked -p host-core` and `cargo test --locked -p host-web --test
   render_locked_staging` pass.
2. `cargo clippy --locked -p host-core -p host-web --all-targets -- -D warnings`,
   `cargo fmt --all -- --check` and `bash scripts/check-workspace-policy.sh` pass.
3. D3's mutation, applied alone, turns `render_locked_staging` phase 2 red with "a collection
   capture's spectrum read allocated"; reverted, green (PR evidence).
4. The workspace builds with `--all-targets`. Apart from the two test assertions,
   `SpectrumCaptureCollection::select` is the remaining internal caller of `entry` (Amendment);
   `selected_entry` has no caller.

*Test value.* No new test. The removal takes away the API a regression would use; phase 2 and the
browser `render-allocations` gate remain the runtime guards, and gate 3 shows phase 2 still bites.

## Amendment (root, 2026-10-08)

Verbatim: "approved as proposed (Amendment, root): body-only edits at select's two sites (build the
owned entry from the borrowed pair, through one private helper if that keeps them identical);
select's signature, behaviour and existing control-path clone unchanged; no allocation added;
correct gate 4's wording to name select as the remaining internal caller. The borrowed-form
alternative is out of scope."

Context: the Problem section said `entry()`'s only callers are `tests/spectrum.rs:737-738`;
`SpectrumCaptureCollection::select` (`crates/host-core/src/spectrum.rs` `:959-961` and `:992-993`)
also calls `self.entry(index)`. Attempt 1 of the first run stopped on this defect.

## Evidence

- Gates 1-4 output; gate 3's red and green runs.

## Dependencies

- After: none open (#1333 is on `main`).
- Either order with A #1327, A #1395 and J #1443 (hot file).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: under two hours.

## Attempt record

### Attempt 1 (implementer, 2026-10-08, after the Amendment)

- **Change.** `crates/host-core/src/spectrum.rs`: `selected_entry()` removed (D1); `entry(index)`
  returns `Option<(&SpectrumTarget, SpectrumChannels)>` through the borrowing `target()` and
  `channels()` (D2); a new private `owned_entry(index)` builds the owned
  `SpectrumCaptureCollectionEntry` from `entry`'s pair, and both `select` sites call it. `select`'s
  signature and behaviour are unchanged; its one clone of the target is the same clone as before,
  on the control thread; no allocation is added.
  `crates/host-core/tests/spectrum.rs`: the two assertions compare `entry(i)` with
  `Some((&track_x.target, track_x.channels))`. `hosts/host-web/MUTATIONS.md`: the phase-2 row
  records D3's mutation (D3).
- **Callers added on other branches.** None: on this branch, `selected_entry` has no caller and
  `entry` has only the two test assertions and `select`.
- **Gate 1.** `cargo test --locked -p host-core`: every binary ok (one pre-existing ignored test in
  each of two binaries). `cargo test --locked -p host-web --test render_locked_staging`: 1 passed.
- **Gate 2.** `cargo clippy --locked -p host-core -p host-web --all-targets -- -D warnings`: clean.
  `cargo fmt --all -- --check`: clean. `bash scripts/check-workspace-policy.sh`: `workspace policy:
  ok`.
- **Gate 3 (D3 mutation).** In `PreparedSpectrumCapture::channels`
  (`hosts/host-web/src/lib.rs`), the collection arm became `{ let _ =
  capture.selected_target().cloned(); capture.selected_channels() }`. Red:
  `render_locked_reads_never_allocate_after_boot ... FAILED`, `assertion left == right failed: a
  collection capture's spectrum read allocated`, `left: 4`, `right: 0`. Reverted (file restored,
  `git diff` empty): green, 1 passed.
- **Gate 4.** `cargo build --locked --workspace --all-targets`: finished, no error.
- **Other checks.** `bash scripts/check-realtime-policy.sh`: ok (89 marked regions in 25 files).
  `bash scripts/check-cross-targets.sh`: PASS (the #1018 expected failures only). Worklet chain
  (host-core is compiled into the worklet), with the CI invocations from `qualification.yml`
  after emptying the output directories: `build-web-audioworklet.sh --named-twin` exit 0,
  `check-web-audioworklet.sh --without-metadata-regeneration` exit 0,
  `check-browser-expected-resources.py --artifacts` exit 0 (digests and exact rows agree),
  `test-web-audioworklet.sh` exit 0.
- **Test value.** No new test. The two rewritten assertions keep the value they had: they turn red
  if `entry` reports a wrong target or mask for a prepared index; the return-type change gives
  them no new catch. Mutation run: `entry` reads `self.captures.get(index ^ 1)`. Red:
  `cargo test --locked -p host-core --test spectrum` fails
  `prepared_collection_switches_exact_taps_without_audio_or_render_allocation` at
  `tests/spectrum.rs:737` (`left: Some((TrackPostInputBuiltins("eq0"), Left))`, `right:
  Some((TrackPostInputBuiltins("eq1"), Stereo))`); the lib unit test
  `explicit_collection_start_uses_the_checked_hop` also goes red, through `select`. Reverted: 9
  passed.
