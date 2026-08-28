# D2: which strip input parameters are live, and what liveness costs

**Issue**: #210 (owner ruling, adopted by the coordinator 2026-08-27), implementing #178's schema
half. **Landed**: phase 3.

**Class**: a *design* ruling -- what is live, at what price, and what the decision drags with it --
rather than the null optimization measurement this directory's README describes. It is filed here
for the same reason `fast-db-tier-boundaries.md` and `multiband-ramping-split-boundary.md` are: it
records a boundary and the exact evidence that would reopen it, and a later phase that wants to
move the boundary should have to argue with a file rather than with a memory.

## The unified principle the ruling rests on

*A strip parameter is live iff its declick story is the existing linear-gain law or cheaper; it is
session-automatable iff it is live; the session automation table's reachability is decided for the
container, not per parameter.*

## The tiering, per parameter

| Parameter | Before | After | Mechanism, or the price of not doing it |
|---|---|---|---|
| `trim_db` (id 2) | `PreparedOnly` | **live** (`BlockTarget` + `LinearNUpdates`), command kind 10 | A banked input drain (`BuiltinBankProcessor::begin_block`) and a ramping variant of the input kernel (`input_chain_ramp_block`) in which the trim coefficient steps per sample under the D11 linear-N law. The settled path dispatches the existing `input_chain_block_elided` bit-identically. |
| `polarity_invert` (id 1) | `PreparedOnly` | **live** (`BlockTarget` + `LinearNUpdates`), command kind 11 | Zero new DSP. Polarity is the *sign* of the same `trim_signed` coefficient the trim rides, so a flip is a retarget of that coefficient to its own negation and the linear ramp carries it through zero. That is a console-grade declicked flip for the cost of the ramp already built for trim. |
| `hpf_hz` (id 3), `lpf_hz` (id 4) | `PreparedOnly` | **deferred**, with the slot kept | The honest price, and it is not small: a live filter move needs a control-plane `f64` redesign per event, a per-word coefficient ramp, a ramped/settled block split, and redesign-elision -- the parametric EQ's whole ramp machinery (`crates/miso-engine-parametric-eq/src/lib.rs`), which is ramp state of 6 words x 2 sections x 2 lanes per track plus a second input-chain path. Nothing in this round needs it. Revisit **together** with #191's variable-slope question, because a slope option is a section-count change and a floor recount trigger and the two should be designed once. |
| `delay_samples` (id 11) | `PreparedOnly` | **stays prepared** | Its own ruling (phase 2): changing a delay length mid-render re-times the ring and glitches unavoidably. |
| `fader_db` (5), `mute` (6), `matrix_*` (7-10) | live since #140 B / #137 D1 | **now also automatable** | Schema only; see below. |

## The blast radius the ruling carries (the verifier's additions)

### 1. The Job-1 elision justification, rewritten

Job 1 decides `InputChainPlan` **at bank construction** and the recorded justification was
"`hpf_hz`, `lpf_hz`, `trim_db` and `polarity_invert` all declare `PreparedOnly` … so no live
surface can move them". Phase 3 removes that premise for two of the four.

**The decision stays sound, and the reason is the predicate's read surface rather than a
re-derivation.** `section_is_identity` reads exactly eight words per section: the six `SvfCoef`
words and the two `SvfState` integrators. `trim` is not among them -- it is consumed one step
earlier, at `input_chain_block`'s step 3 -- and a section is the arithmetic identity or it is not,
whatever the chain multiplies its input by. The only parameters that design those eight words are
`hpf_hz` and `lpf_hz`, and both remain `PreparedOnly`, so the coefficients are still written
exactly once.

The rationale is rewritten to say exactly that in all three places it lives:
`crates/miso-engine-lane/src/kernels/builtins.rs` (`InputChainPlan`),
`crates/miso-engine-builtins/src/lib.rs` (`InputStage::plan`) and
`docs/rulings/effect-floor-accounting.md`.

**The settled path phase 3 dispatches is the elision-planned kernel variant Job 1 introduced** --
`input_chain_block_elided` over the prepared words, not the unelided `input_chain_block`. That is
what the class-A OFF claim is about: a lane no command has ever retargeted runs the exact call, on
the exact words, in the exact order it ran before the feature existed, behind one `bool` test.

### 2. The HPF/LPF liveness proviso (binding on the deferred tier)

**Any future `hpf_hz`/`lpf_hz` liveness REQUIRES a command-driven elision-plan invalidation or
recompute hook.** That is precisely where "cannot go stale" breaks: a live filter retarget writes
the six coefficient words the predicate reads, so a plan decided before it is a plan decided about
different words. The hook already exists in shape -- `InputStage::set_lane_state_words` and
`InputStage::reset` both re-decide the plan after writing state -- and a filter retarget must join
them. This is a hook, not a comment: a phase that adds filter liveness without it ships wrong bits
on any bank whose sections were elided at prepare.

### 3. Mono-collapse

Live trim and polarity are **de-symmetrizing live commands upstream of the seam**, and the
collapse machinery had to be taught about them in three places:

* the gate compares the **live** trim words, not the prepared ones. `InputStage::coef.trim` is
  republished from the ramp's `current` after every retarget and after every ramping block, and
  `lane_channel_symmetry` compares it -- plus `target`, `step` and the countdown, because at the
  block an asymmetric retarget is admitted `current` has not moved yet and a witness that compared
  only `current` would let that block collapse;
* the drain folds `ChannelSymmetryWitness::admit`, so a per-lane retarget clears `LIVE` before
  the collapse dispatch reads the witness. That ordering is why the drain is `begin_block` and not
  the first paragraph of `process`, and it is the difference between correct bits and a left-lane
  retarget published on both channels of the admitting block;
* a `channel = both` command is **one** record carrying `BuiltinLaneSelector::Both` and is admitted
  as `SymmetryEvent::Preserve`, so a symmetric ride keeps the collapse bit-identically. This is a
  deliberate departure from the effect-parameter lowering, where `both` on a `PerLane` parameter
  becomes two records and therefore two `Desymmetrize` events; `TrackInputRecord` carries the
  argument.

**The disengage boundary, and the correction it took.** The first cut of this phase copied the
input stage's *whole* per-channel state at `desymmetrize` -- integrators and trim ramp -- on the
"whole per-channel state is restored at the disengage boundary" reading. That reading is wrong for
the ramp, and adversarial verification found the window: `BankChain::run` drains every slot's
`begin_block` **before** it reads the collapse witness and runs `disengage_collapse` **after**, so
on the block a per-lane record ends a collapse the order is *mirror (block N-1) → drain and apply
(block N) → witness declines → boundary copy*. The two channels are apart at that boundary
legitimately, and the copy cloned the just-drained left record onto the right channel: a one-lane
retarget ramped both lanes, and because `LIVE` is a latch the chain never collapsed again and the
right channel never recovered.

The rule that replaced it is narrower and is the true one: **a stage restores at the disengage
boundary exactly the per-channel state its one-plane body froze.** `InputStage::process_mono`
freezes the integrators -- it advances channel `0`'s and leaves channel `1`'s -- so those need the
boundary. It does not freeze the ramp; it *mirrors* it, at the bottom of every collapsed block, so
the ramp's restore path is per block and the boundary must leave it alone. The collapse gate's own
premise -- that a one-plane block is dispatched only when the two channels' ramp records compare
bit-equal -- is now asserted where it holds, at the top of `process_mono`, rather than where it
does not.

**Re-engage.** `LIVE` is a latch -- cleared by the drain, never set again within a plan -- exactly
as it is for `EffectControlRecord`. So a track whose channels were driven apart by a per-lane
trim ride does not re-engage its collapse even after the two words are made equal again. That is
**stronger** than M3's rule, which is that re-equal parameter words alone must not re-engage; here
they cannot re-engage at all, and the M3 proof path (`channels_agree`, implemented for the input
bank over the integrators and the whole trim-ramp record) is reachable only for chains that lost
agreement some other way. Making `LIVE` recoverable would be a change to the M-series machinery --
the proof would have to be consulted *before* the witness rather than after it -- and is
deliberately not one this phase makes.

`delay_samples` is excluded from the input kernel's word list and the live trim words are included,
and the two rationales have to stay coherent side by side: the test is **what this kernel loads**.
The delay is a graph node at `TrackStage::Input`, upstream of the bank, and `input_chain_block`
never sees it -- its verdict is taken at prepare by `track_input_delay_symmetric` and conjoined
into the same `DESIGNED` term. The trim word *is* loaded, on every frame, so it is compared here.
Listing the delay would be claiming a load that does not happen; omitting the trim would be
skipping one that does.

### 4. The metadata gate's live-set literal

`scripts/check-parameter-metadata-v1.py` pins the live builtin names **exactly**, not merely by
count, and that literal moved in the same change that flipped the two descriptor rows. It carries
two red mutations of its own: promoting a deferred row into the live set and demoting the live trim
row out of it. Both are internally consistent -- `liveUpdatable` still follows `updateRate` -- so
only the literal refuses them.

## The #178 half: `rack = "builtins"`

`RackName` gains a fourth token, `builtins`, wire code 4 (appended, so no existing code moves). A
target with that rack carries `effect_id = "strip"` -- a fixed *validated* literal, because Session
V1 has no optional keys -- and a `parameter_id` that must be one of the builtin ABI's
block-target rows, with `channel` per the row's scope.

**The load-bearing caveat, and it is a ruling and not a caveat about the ruling: the session
automation table is consumed by nothing today.** No lowering reads it, for this rack or for any of
the other three. So a valid `builtins` target is valid-and-inert syntax: it authors, it
round-trips, it survives the canonical writer, and it renders nothing. What the extension buys is
that authoring and the #207 SDK's `automate()` builder are unblocked and the vocabulary is decided
once. **Builtin automation *rendering* is explicitly gated on #140's automation-span feed**, whose
natural destination is the three drains #137, #140 and #210 phase 3 built, because a span's
block-first-sample semantics already match the drain contract. Phase 3 builds no feed and nothing
in it should be read as having built one. The gate is restated at the validation arm
(`crates/miso-engine-session/src/validate.rs`) and in `docs/SESSION_SCHEMA_V1.md`.

## What would reopen each half

* **HPF/LPF liveness**: a workload that needs live filter moves, *plus* the elision-invalidation
  hook above, *plus* #191's slope decision, designed together.
* **`delay_samples` liveness**: a re-timing story that does not glitch.
* **Builtin automation rendering**: #140's span feed.
* **`LIVE` recoverability**: an M-series change that consults the state proof before the witness.
