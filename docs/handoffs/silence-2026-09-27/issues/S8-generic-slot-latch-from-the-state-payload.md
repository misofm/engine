# A generic slot rest latch from the effect's state payload

Draft, slice S8 of the silence architecture issue (A0). Class A. The owner's steer: rest detection is
a property of the slot, not code every effect must implement. Evidence:
`docs/handoffs/silence-2026-09-27/DESIGN.md` section 4.3 (the three payload audits and the measured
snapshot cost).

## Product outcome

Every banked effect without a claim of its own (multiband compressor, transient shaper,
gate/expander, soft-clip, and every future native effect) reaches rest in the whole-bank latch without
per-effect code, and #893 and #894 become unnecessary. The slot wrapping the effect detects rest from
the effect's complete state, which the contract already requires it to expose for save and restore.

## The rule

For an effect whose `rest_detection()` is `Generic` (below), the slot keeps two preallocated image
buffers of `lanes x payload bytes`, a `have_previous` flag, a backoff counter and a `latched` flag.

* On a block whose input is silent (the chain passes the fact) and whose drain admitted nothing:
  * latched → the slot is `silent_skippable`; the chain skips it (and, with every other slot, the
    bank);
  * not latched → run the kernel; if the output block is `+0.0` by bits (#942) and the backoff allows,
    snapshot every lane (`snapshot_track_state_payload`) into the current image; if `have_previous`
    and the two images are equal, latch; otherwise keep the image as previous. A non-silent output
    clears `have_previous`.
* Any other block: unlatch, clear `have_previous`.
* **Backoff:** after a failed comparison wait 1, 2, 4 … 64 blocks before the next attempt (an attempt
  is two consecutive snapshots). A compressor-like tail of about 700 blocks then costs about 16
  attempts, not 700.
* **Frames rule:** equality over one block proves only a period dividing `frames`; the latch records
  `frames` and releases on a different `frames`.
* **Collapse:** a collapsed block compares the left section only (the right section is frozen by
  design); `desymmetrize` releases.

`rest_detection()` is a provided method on `PreparedNativeEffectBank`: `Own` when the effect
implements `silent_rest` itself (EQ, compressor, limiter, and any tail path of S6), `Never`, or
`Generic`. The default is `Generic` when the bank's payload is at most 8 KiB per bank, otherwise
`Never`. The limiter is `Own` (its payload carries cursors that move every block, so a generic
comparison never matches); the delay does not bank.

## Contract obligations (effect-contract)

1. **Payload completeness at rest.** Every word that can influence future output is in the payload,
   or is a pure function of payload words (a memo), or only selects between schedules that render
   identical bits. Gate: a conformance test per effect drives it to rest on silence, snapshots it,
   restores into a fresh prepared instance, then feeds both the same seeded signal: bit-identical
   output and payloads for 256 blocks.
2. **Time invariance:** no read of `first_sample` or any time outside the payload except automation
   span validation.
3. **Snapshot on a bound bank:** permitted at a block boundary for this purpose (the trait docs today
   name only the unpublished-bank caller); allocation-free into caller buffers; failure (`Err`)
   unlatches, never panics.
4. An optional bulk `state_words` view may override the payload for speed (the EQ's 7,488-byte W8
   payload costs 2.03 µs for two snapshots and a compare, measured, against its dense slot's 1.9 µs;
   its own `state_bits` is 24 vector stores).

## Authorized paths

`crates/effect-contract/src/lib.rs`, `crates/rack/src/lib.rs` (the slot logic in `EffectBankStage`
and `ConsoleEffectBankStage`), the effect crates' conformance tests (and a `rest_detection` override
for the limiter), `crates/conformance/`, tests, `MUTATIONS.md`, this spec.

## Objective gates

1. Conformance (obligation 1) for multiband compressor, transient shaper, gate/expander, soft-clip,
   EQ and compressor, at widths 4 and 8.
2. Bit identity against the declined oracle for new test strips carrying each of the four
   claim-less effects, over S4's pattern corpus; skips counted.
3. Cost: per silence episode, the number of snapshots is recorded; with backoff it is at most
   `2 * (log2(64) + tail_blocks / 64 + 1)` per slot.
4. Red mutations: latch on one snapshot; compare a payload with the header only; ignore `frames`;
   keep the latch across an admitted span, target or bypass change.
5. Realtime: images allocated at bind; allocation-free render test with latching and releasing;
   realtime policy; rule 3.

## Console benchmark rows

None of the standing rows carries a claim-less effect. Add a sparse variant with a multiband compressor
or transient shaper in `dynamic`, or measure in-process only (the owner decides).

## Dependencies

S4. Replaces #893 and #894 if ruled.

## Standing rules for the implementer

As S4.
