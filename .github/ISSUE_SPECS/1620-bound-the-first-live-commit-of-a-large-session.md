# Bound the first live commit of a large session

Successor of #1615, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code
anchors verified on the batch 9 branch `codex/controls-batch-9k2`. Issue number: assigned at filing
(written here as #1620). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a
batch review.

## Product outcome

The first live-only transaction after boot on a large session needs about the same peak of heap as
a later one, so a browser page's first fader move does not grow the wasm memory in the
AudioWorklet's message handler on a 64- or 256-track session.

## Context

- #1615 cut the steady live commit of the nine-edit transaction to a 71,252-byte peak, the same at
  1, 64 and 256 tracks (`hosts/host-web/tests/control_command_heap.rs`).
- The first call is not: its peak is 76,011 bytes at 1 track, 196,593 at 64 and 663,179 at 256
  (#1615's decision record). It builds state that later calls reuse.
- On a non-isolated page this runs in the worklet's message handler, where a growth is a
  `memory.grow` on the audio thread (counted and reported; #1562, root ruling A).

## Decisions to take (the implementer proposes, the coordinator rules)

- **Option A (boot).** Build the reused state during boot or preparation, off the audio-thread
  handler's first edit, and charge it in the boot resource report.
- **Option B (lazy, bounded).** Keep it lazy but size it from the session at preparation so the
  first call makes one exact allocation.
- First measure which structures make up the first-call excess at 64 and 256 tracks.

## Objective gates

1. The heap test gains a first-call row at 64 and 256 tracks with a ceiling near the steady peak
   (measured numbers and the reason in the decision record).
2. The boot budget gate (`scripts/check-web-boot-budget.mjs`) still passes at its current pin.
3. `cargo test --locked -p host-web --features host-web/test-support`, `-p control-plane
   --features test-support`, `-p host-core --all-features`.

## Test value

- Gate 1 turns red if the first live commit grows with the session again.

## Authorized paths

- `crates/host-core/src/control_provider.rs`, `crates/control-plane/src/*` (the first-commit state
  only), `hosts/host-web/tests/control_command_heap.rs`, this spec's decision record.

## Dependencies

- #1615.

## Size

Half a day, one sonnet-high implementer.

## Decision record: #1620 (batch 14b) -- measurement, stopped before coding

Anchor correction: the heap gate lives at `tools/parameter-metadata/tests/control_command_heap.rs`
(the batch 9 CI fold moved it to use `bench_support::alloc`), not under `hosts/host-web/tests/`.

Measured on `8b3ae13ec` (debug, `verify_stages(false)`), the nine-edit transaction of that test:

| tracks | first call: allocations / requested / peak / left | steady call peak | first-call excess | session canonical text |
|---|---|---|---|---|
| 1 | 834 / 129,249 / 77,347 / 1,078 | 72,588 | 4,759 | 5,474 |
| 64 | 1,848 / 549,392 / 197,929 / 1,078 | 72,588 | 125,341 | 96,887 |
| 256 | 4,922 / 1,866,704 / 663,179 / 1,078 | 72,588 | 590,591 | 375,479 |

**What the excess is.** Not lazily built state that later calls reuse. Traced with a temporary
backtrace hook on every allocation of 4 KiB or more (reverted): the first call leaves the in-place
path (#1613) and takes the full preparation. `SessionStore::prepare_in_place`
(`crates/protocol/src/model.rs`) returns `None` because `session::owners_counts_equal` is false: the
fixture's track holds a sparse `eq-simd1` parameter list (IDs 1 to 6, ID 4 per lane), and the edits
write IDs 65, 66, 67, 81, 82, 83 and ID 4 on `both`, which adds parameter entries. A count change is
not a change of values to the in-place path, so the store clones the whole model, runs
`compile_session` (a second model clone, the validation maps, the stable sorts) and writes the
whole canonical text. At 256 tracks the peak is that text's `String` doubling to 524,288 bytes
(final 376,405) plus about 66 KB of model clones and sort buffers; at 64 tracks the text reaches
131,072. The 1,078 bytes left are the committed model's added parameter entries. From the second
call on, the entries exist, the counts are equal and every call is in place (72,588 at every size).

**Why neither option fits.** Option A (build the reused state at boot) and Option B (lazy, one exact
allocation) both assume reusable state; there is none. The cost is the full compile path, taken by
any live edit that first writes a parameter the sparse model does not hold, on any revision, not
only the first. The candidate fixes are outside this spec's authorized paths and need a ruling:
(C1) let the in-place path accept a `SetControl` that adds a parameter entry (adjust
`owners_counts_equal` and the in-place resource estimate in `protocol`/`session` so an added entry
is charged exactly); or (C2) densify every effect's parameter list at preparation (changes the
committed model and its canonical snapshot, a contract change). Per the coordinator's ruling the
work stopped here; no code changed.

## Root ruling (2026-10-15): C1

- **Design.** The in-place path admits a `SetControl` that adds a parameter entry, and charges the
  added entry exactly. The committed model and the canonical snapshot equal the full path's for
  the same transaction. The rejected alternative (C2, fill every parameter list at preparation)
  would change the committed model and its canonical snapshot.
- **Authorized paths** gain `crates/session/src/in_place.rs` and `crates/protocol/src/model.rs`
  (the in-place admission and its resource estimate), and the heap gate's real home,
  `tools/parameter-metadata/tests/control_command_heap.rs`.
- **Gates.** (1) A differential gate: for edits that add parameter entries, the in-place commit's
  model and canonical snapshot equal the full path's, bit for bit. (2) The first-call peak at 64
  and 256 tracks is measured and recorded, with a ceiling near the steady peak. (3) The first
  call's allocation count does not depend on the track count.

### Implementation (batch 14b)

**What shipped (C1).** `session::owners_counts_equal` is replaced by `session::owners_growth`
(`crates/session/src/in_place.rs`): `Some(OwnersGrowth)` when the edited owners hold as many
console entries, inserts and VCA members as the committed ones and no effect's parameter list is
shorter, with the entries added and the longest grown list; `None` for any other change of a count
(a shorter list included, so every item the estimate takes a maximum over only grows). The new
`ResourceEstimate::after_owners_edit` (`crates/session/src/estimate.rs`) charges the growth exactly
as `estimate_session_resources` and `with_canonical_bytes` do: per added entry one
`size_of::<EffectParam>()` in the model and one structural item (1,024 bytes, now the named
constant `CANONICAL_BYTES_PER_ITEM`) in the canonical bound; the single largest allocation is the
largest of the committed one, the new canonical bound and the grown lists. It runs the compile's
own `check_caps` (now `pub(crate)`, its unused model argument dropped) and the platform and
canonical-bound checks on the result. `SessionStore::prepare_in_place`
(`crates/protocol/src/model.rs`) uses both and passes the store's caps: a growth that does not fit
returns `None` and the full path gives its typed refusal before any commit (no ack before a drop).
The entries land where the full path puts them: `controls`' parameter write already keeps the list
in canonical `(ID, channel)` order, the order `compile_session` sorts into.

Outside the listed paths (mechanical): the in-place admission comment in
`crates/control-plane/src/control.rs` (the in-place transaction may now add parameter entries, never
a count `check_counts` reads); `crates/session/tests/in_place.rs` moved to the new function; the
differential gate lives in `crates/host-core/tests/in_place_commit.rs`, the home of the #1613 `Pair`
harness that compares the two paths.

**Gates.**

1. Differential: `in_place_commit::edits_that_add_parameter_entries_commit_in_place_as_the_full_path_does`
   (console EQ above IDs 1 to 6 on one lane and both; an empty insert EQ list gaining an entry, then
   one below and one between; a split of a `both` entry; one transaction over seven owners of tracks
   and the submix, console and inserts; an abandoned one). Every transaction is prepared and
   classified in place and the stores (model, canonical snapshot, estimate, revision) equal the
   full path's after each. Refusal: `an_added_entry_over_the_caps_is_refused_before_any_commit`
   (a store whose compiled-model or single-allocation cap is the committed figure: a fader commits
   in place, an added entry is refused by both entries with the same `Validation` error and the
   store is unchanged). Session unit: `owners_with_values_the_compiler_refuses_are_not_valid_and_counts_are_compared`.
2. `tools/parameter-metadata/tests/control_command_heap.rs` (debug): the first call is gated at
   every size, `FIRST_PEAK_CEILING` 74,000 and `FIRST_REQUESTED_CEILING` 105,000 (was 78,000 peak
   at one track only); `first.left <= 256` (was 4,096). Re-pinned for #1620 from these
   measurements, on this head before and after the change:

   | tracks | first call before: allocations / requested / peak / left | first call after | steady (unchanged) |
   |---|---|---|---|
   | 1 | 839 / 130,294 / 77,600 / 1,078 | 757 / 103,033 / 72,865 / 80 | 758 / 103,918 / 72,841 / 80 |
   | 64 | 1,853 / 550,437 / 198,182 / 1,078 | 757 / 103,033 / 72,865 / 80 | the same |
   | 256 | 4,927 / 1,867,749 / 663,179 / 1,078 | 757 / 103,033 / 72,865 / 80 | the same |

   The steady ceilings (73,000 peak, 104,000 requested) are kept.
3. The same test asserts the first call's allocation count equal at 1, 64 and 256 tracks.
