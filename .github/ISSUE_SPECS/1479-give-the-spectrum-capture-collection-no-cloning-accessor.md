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

- `crates/host-core/src/spectrum.rs` (`entry`, `selected_entry` and their docs only)
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
4. The workspace builds with `--all-targets` (no other caller of either accessor exists).

*Test value.* No new test. The removal takes away the API a regression would use; phase 2 and the
browser `render-allocations` gate remain the runtime guards, and gate 3 shows phase 2 still bites.

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
