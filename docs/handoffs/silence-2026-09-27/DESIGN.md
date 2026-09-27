# Silence propagation: measurements, design and plan (2026-09-27)

Owner direction, verbatim: *"How is idle still 31.34? I think we should actually prioritize that.
I can think of many sessions where only a few seconds of a track has non-silent audio... We should
ensure that no processing is done if silence is detected - I think this should be applied to every
step in the chain as well."* Owner steer (via the coordinator): rest detection should be a property
of the slot or track, not code every effect must implement; per-effect rest claims become optional
fast paths.

This record measures where the idle block goes, designs silence propagation for the whole chain,
prototypes the smallest version of it, and slices the work. Draft issue bodies are in `issues/`.
Nothing here is pushed or filed.

## 0. Summary

* **The idle row's 30 µs is almost all avoidable.** 56 % is the builtin input filters running on
  silence, 7 % is the benchmark's own bound source copies, and the rest is effect admission scans,
  transposes, the fader, the matrix and the master fold. Every word they write is `+0.0`.
* **A whole-bank latch, prototyped, takes the idle row from 29.4 to 4.97 µs** on native eight
  lanes, bit-identical on every block tested. The latch skips a bank when every lane's input is
  `+0.0` and every slot is at rest. Of the 4.97 µs left, about 2.1 µs are the bound copies that
  #965 removes (2.09 µs probed on the latched run, 2.16 on the declined one), 0.96 µs are input scans that a source silence bit removes, and 0.4 µs are the limiter's
  cursor bookkeeping, which can be deferred. The projected production idle cost is about
  **0.6-1.0 µs**.
* **Sparse real sessions gain less, for two measurable reasons.**
  * **Banks run in lockstep.** One live track keeps its whole bank running. In the session's real
    bank order, only 50 % of eight-lane bank-blocks are entirely silent, although 91 % of
    track-blocks are.
  * **Dynamics tails.** A compressor or limiter emits exact `+0.0` from the first silent sample,
    but its gain state takes 2-6 s to reach an exact fixed point.
  * **Measured over the whole dogfood song:** the console strip falls 98.1 → 92.5 µs mean (-6 %),
    and the builtins-only strip falls 24.5 → 16.8 µs (-31 %). With the owner's activity-aware
    banking the figures are 98.1 → 67.0 µs (-32 %) and 24.5 → 12.7 µs (-48 %).
  * **At four lanes** (the browser width, native proxy) the builtins-only strip falls 42.3 → 20.3 µs
    (-52 %), or 15.1 µs (-64 %) with regrouping.
* **The worst case does not move.** The first loud block after a long silence costs 164.0 µs
  latched against 163.1 µs declined (median). Steady dense blocks cost 129.3 µs against 128.4 µs.
  Both are within noise.
* **The generic slot latch, as specified by the owner's steer, is sound for every banked in-tree
  effect.** Each effect's state payload is complete for this purpose. Only the limiter keeps
  cursors in its payload and so never qualifies, and the delay never banks at all. It is expensive,
  though: two EQ-bank snapshots cost 2.0 µs, about the EQ kernel's own cost. And on today's rows it
  finds nothing the minimal latch does not already find, because every effect in the standing strip
  already has a rest claim.
* **Plan.**
  1. Build the minimal latch first: the source bit, the builtin rest claims, the whole-bank latch
     and a dogfood sparse row.
  2. Then add exact rest tails for the compressor and limiter.
  3. Then activity-aware banking.
  4. Then the generic payload latch, for effects that have no claim of their own.

## 1. Method and hosts

* **Host.** AMD EPYC 7313P (Zen 3), Linux 6.8, rustc 1.97.1, `x86-64-v3`, release build (fat LTO,
  one codegen unit), `CARGO_INCREMENTAL=0`, pinned with `taskset` to core 5 (its SMT sibling was
  idle).
* **Clock.** Nanoseconds come from `std::time::Instant`. Cycles are quoted only where they are
  derived from those nanoseconds at 3.7 GHz.
* **Base.**
  * The phase breakdown (section 2) and the prototype (section 6) were built on
    `codex/batch-plumbing-floor-2` at `3fc79b5a`.
  * The branch has since moved to `85019581` (#958, #960). #960 adds a committed bank sub-phase
    profiler (`rack::test_only_bank_phase_profile`, `tools/console-workload/tests/gain_pan_profile.rs`),
    which the slices below should use instead of this record's throwaway probes.
  * The prototype patch `whole-bank-latch-prototype.patch` applies to `3fc79b5a`. It does not
    apply to `85019581`, because #960 touches the same lines of `crates/rack/src/lib.rs`.
* **Timing lock.**
  * **Taken under the shared timing lock** (`flock` on `.../scratchpad/timing.lock`): every
    prototype A/B timing (section 6), the downbeat measurement and the snapshot-cost measurement.
  * **Taken before the lock existed** (about 12:03-12:08, load average 4.5-6): the phase breakdowns
    in section 2.
    * They may carry noise from other agents' builds.
    * Their p50 (30.1 µs) agrees with the later locked declined arm (29.4 µs) within 2.5 %.
    * It also agrees with the recorded `artifacts/steps/after-950` idle row (30.99 µs).
* **Status.** All figures are descriptive, in-process and throwaway. The console benchmark runner
  was not run. The owner's 31.34 µs is a runner number from a nearby step.
* **Instruments.**
  * **(a) Phase probes.** Graph `test_only_phase_profile` plus throwaway `Instant` laps inside
    `BankChain::run_with_input`, charged per slot position. Each lap costs about 30 ns, which is
    subtracted per chain in the tables.
  * **(b) The prototype.** `whole-bank-latch-prototype.patch`: harnesses
    `tools/console-workload/tests/silence_{profile,proto}.rs` and
    `crates/parametric-eq/tests/silence_snapshot_cost.rs`. Run with, for example,
    `CARGO_INCREMENTAL=0 taskset -c 5 cargo test --release -p console-workload --test silence_proto -- --ignored --nocapture --test-threads 1`.
  * **(c) A read-only scan of the dogfood stems.** `stem-silence-banking.rs.txt`, a standalone
    program with no dependencies.
  * **(d) Three read-only audits,** one per group of effect crates, of state payload completeness
    (summarised in section 4.3).

## 2. Where the idle block goes

`sixty_four_track_idle`: the intended 64-track console strip (input trim/HPF/LPF, EQ and compressor
on `simd1`, limiter on `simd2`, fader, pan, route fold into the master), eight 8-lane banks, one
chain of six slots per bank, bound feed writing exact `+0.0`, and 512 warm-up blocks. Probes off,
p50 **30.1 µs** (three repeats, 30.10/30.10/30.10). The probe laps below are corrected by 30 ns per
chain.

| where | µs/block | share | what it does on silence |
|---|---:|---:|---|
| builtin input section (slot 0: trim, HPF, LPF SVFs) | **16.8** | 56 % | runs both SVF recurrences over zeros; the kernel has no rest check |
| bound source feed (64 `FrozenGraphSource` copies of zeros) | 2.2 | 7 % | copies zeros into the arena; a benchmark artifact that #965 removes |
| limiter admission (slot 3) | 1.9 | 6 % | ramp checks, 2 x 1,024-word zero scan, 36 `u64` divisions to advance cursors and phase |
| scatter and master fold | 1.9 | 6 % | transposes and route-folds 8 x 128 frames of `+0.0` into the master |
| compressor admission (slot 2) | 1.8 | 6 % | ramp checks and the 2 x 1,024-word zero scan |
| gather | 1.7 | 6 % | transposes zeros into the resident block |
| matrix (slot 5) | 1.1 | 4 % | settled 2x2 over zeros |
| EQ admission (slot 1) | 1.0 | 3 % | ramp checks and the 2 x 1,024-word zero scan |
| fader (slot 4) | 0.8 | 3 % | gain and mute over zeros |
| drains, collapse decision, member ops, unit dispatch, Output, enter | 0.9 | 3 % | per-block bookkeeping |
| **total** | **30.1** | | output: `+0.0` in both host planes |

* **What is irreducible:** writing `+0.0` into the two host planes (256 words, about 20 ns), draining
  control queues so that a change is noticed, and a handful of flag reads. Everything else in the
  table is provably skippable once the chain's inputs are known to be `+0.0` and every stage is at
  rest.
* **The dense row has the same shape.** On `sixty_four_track_console` (tone, 127.2 µs) the input
  section costs the same 17.3 µs as on silence. EQ, compressor and limiter cost 15.4, 41.4 and
  44.5 µs. Gather and scatter cost 2.3 µs each. The existing effect claims are what make silence
  about 100 µs cheaper than tone today. Nothing makes the builtins or the plumbing cheaper.
* **At four lanes** (native `Simd4`, a proxy for the browser):
  * The builtins-only strip on silence costs 41.2 µs. The input section at four lanes is about
    2.1 µs per 4-lane bank, which is twice the per-track cost at eight lanes.
  * The full console strip cannot be measured this way. On x86 the effects bank only at the native
    width (`Backend::current()`, for example `crates/compressor/src/lib.rs` `bind_homogeneous_bank`),
    so at `Simd4` they fall back to 128 scalar per-node ops.
  * The last wasm console record (`artifacts/issue183-post-round2`, an older tree) has the idle row
    at 61.5 µs in wasm `simd128`, against 32.0 µs native at eight lanes. **The browser's idle cost
    is about twice native.**

## 3. How silent real sessions are (dogfood stems)

The dogfood set: `/home/bl/misofm/agents/stems`, 81 stereo 24-bit 44.1 kHz stems of one song,
141.9 s, 48,895 blocks of 128 frames. It reproduces #940 exactly:

* Track-blocks: **9.1 % live, 62.4 % digital silence** (a `Some` block, all `+0.0`), **28.4 %
  absent** (past the end of the file). Mean silent tracks per block: 73.6 of 81.
* The real session (`mixes/first-listen-v1/render-001/session.json`) carries **no rack effects on
  any track**. It is builtins only: HPF 25 Hz, LPF off, fader, pan. So a signature constraint on
  regrouping (option (c) in the coordinator's request) is vacuous for it: every track has one
  signature.
* **The session's real bank order is not file order.** The track ids are content hashes
  (`t-0124e3fd…`), and banks follow sorted ids. That order clusters co-silent stems less well than
  file names do.

**Fraction of bank-blocks that are entirely silent.** A bank qualifies at a block only if every
lane has been silent for the whole settle allowance.

| grouping | W | 1 block | 0.4 s | 1 s | 4.6 s |
|---|---:|---:|---:|---:|---:|
| session order (sorted track ids, today) | 8 | 50.2 % | 41.9 % | 36.2 % | 23.7 % |
| file-name order (#940's figure) | 8 | 58.5 % | 50.9 % | 45.3 % | 32.3 % |
| co-silence grouping, optimised at 1 block | 8 | 77.8 % | 72.3 % | 67.7 % | 55.6 % |
| co-silence grouping, optimised at 1 s | 8 | 74.4 % | 71.8 % | 69.1 % | 57.2 % |
| session order (sorted track ids, today) | 4 | 69.9 % | 62.4 % | 55.7 % | 39.5 % |
| file-name order | 4 | 73.5 % | 66.6 % | 61.4 % | 47.9 % |
| co-silence grouping, optimised at 1 block | 4 | 83.9 % | 79.2 % | 74.9 % | 62.7 % |
| co-silence grouping, optimised at 1 s | 4 | 83.6 % | 80.8 % | 77.8 % | 65.0 % |

* **Grouping method.** A greedy seed, then pairwise-swap hill climbing that maximises the silent
  bank-block count at the stated allowance.
  * Seed: open a bank with the unassigned stem that is silent most often, then add the stem that
    keeps the running intersection of silent blocks largest.
  * Climb: take the best strictly improving swap of two stems between banks until none remains.
  * The stems' per-block silence vectors are bitsets, and the objective is the intersection's
    popcount.
  * This is a heuristic, not an optimum. The groupings are listed in section 9.
* **Headroom of co-silence grouping over today's session order:** +27.6 points (1 block) and
  +32.9 points (1 s) at eight lanes; +14.0 and +22.1 points at four lanes.
* **Stranded silence.** In file order, 35.7 % of all track-blocks at eight lanes (18.3 % at four)
  are silent tracks inside a bank that is not entirely silent. Lockstep execution cannot recover
  them; only regrouping can.

**The settle allowance is set by the slowest stage of each bank's chain.** From the audits (section
4.3):

| stage | exact rest after the input goes silent |
|---|---|
| input HPF 25-35 Hz (D7 flush at 1e-20) | about 0.4 s |
| EQ | until its integrators flush (the same order) |
| compressor, fixture release 40 ms | about 1.9 s (gain reduction decays in dB toward the flush) |
| limiter, release 60-139 ms | about 5 blocks with no gain reduction; 2.7-6.3 s after gain reduction |
| multiband compressor, transient shaper | about 4.5-4.8 s at their defaults |
| gate | hold, then about 0.8 s |
| soft-clip | 2 blocks |
| delay (not banked) | about 13 s at defaults, up to 30 min at maximum feedback |

**The prototype confirms this on the real pattern.** On the first 64 stems, the console strip
latched 2.07 of 8 banks per block, which is 25.9 %. That is consistent with the 4.6 s column for those
stems (25.9 %). The builtins-only strip latched 3.07 of 8, which is 38.4 %, between the 0.4 s and
1 s columns.

## 4. Design

### 4.1 Principles

1. **Class A by default.** A skipped stage leaves exactly the bits the kernel would have written:
   the output, the state, and every word an observer can read. The one class-B item (co-silence
   regrouping) and the one class-B alternative (snapping tails to rest) are separate, and are
   flagged.
2. **Silence means `+0.0`, by bits.** A block is *silent* when every word of both planes of every
   active lane has bit pattern 0. `-0.0` is not silent. This is #942's `block_is_positive_zero` rule
   (`crates/effect-runtime/src/bank.rs:137`), and the reason is the same: an input of `-0.0`
   is not the input the rest claims were earned on.
3. **Per-block decisions only.** A handful of flag reads and at most one vectorised scan per plane
   per block. No per-sample branch, no allocation, no lock, and no new per-track table on the
   render path.
4. **The lockstep bank is the unit of skipping.** A bank's lanes share every instruction, so
   arithmetic can only be skipped when every lane allows it.
5. **The budget stays the dense block.** Time saved on sparse blocks is never lent to realtime
   work. Section 4.10 has the measured worst case.

### 4.2 Silence facts: where "exactly silent" comes from

A *silence fact* is a per-block `bool` attached to a block that some consumer is about to read.
Producers, from cheapest to most expensive:

1. **An unplayed source claim** (`played_planes(claim) == None`: underrun or end of region). This is
   free today. The graph already serves `ARENA_SILENCE_BUFFER` for it
   (`crates/graph/src/runtime.rs:1609-1632`). It covers 28.4 % of the dogfood track-blocks.
2. **A writer-computed silence bit on the transfer block** (#940's S2; slice S1).
   * Both writers already touch every word off the render thread:
     * the host submit path `submit_planes`, which does `copy_from_slice`
       (`crates/source/src/lib.rs:867-903`);
     * the native decoder's `convert_frames` (`crates/source/src/native_wave.rs:771`).
   * An OR of each channel's bits there gives `silent_channels: u64`, one bit per channel.
   * `play()` zero-fills a short block's tail (`source/src/lib.rs:1557-1575`), so the bit covers
     the whole quantum.
   * A provided method on the driver trait exposes it for a claim's two mapped channels.
   * It covers the remaining 62.4 %.
3. **A latched producer.** A bank chain that skipped this block wrote `+0.0` to every buffer it
   owns (section 4.5). Slice S10 lets it publish that fact so that the Output op or a bus
   reduction can skip the input without reading it.
4. **The #942 scan, as a fallback.** Used for any input with no producer fact: a bound processor
   (the benchmark's bound feed), a host input buffer, an unflagged arena buffer. Measured cost:
   * 8.9 ns per 256 silent words, 16 ns per 512, 28 ns per 1,024;
   * it exits after the first 32 words on live audio.
   * A bank chain scans its 2 x 8 planar inputs *before* gather: about 120 ns per 8-lane bank when
     silent, and 2-3 ns per lane when live.

A **silence table** carries facts across units (slice S10).
* It holds one byte per arena buffer, stamped with the block counter. The bind-time allocation is
  one byte per buffer.
* Each unit writes the facts for the buffers it produces, and consumers read them.
* Within a bank chain the fact travels in a local variable, from the chain input through each slot.

### 4.3 The per-stage rest contract (architecture issue A0)

This is cross-cutting. It touches `effect-contract`, `rack`, `graph`, `builtins`,
`builtins-compiler` and `source`, and it adds render-path semantics to the bank stage seam.
`AGENTS.md` requires an architecture issue for that, drafted as `issues/A0-...md`.

**Definition.** A stage is *silent-skippable* at block N when, having drained its control for N,
it can promise the following for an all-`+0.0` input block of `frames` frames:

* **(a) Output.** The kernel would write an output that depends on nothing that changes while the
  promise holds. So the output is the same block, every block.
* **(b) State.** The kernel would leave every state word bit-identical, except for words that
  `advance_silent(frames, mono)` updates exactly as the kernel would. `mono` is the chain's
  collapse mode for the block: a collapsed stage runs `process_mono`, which advances only the left
  channel's state (the limiter's latched mono body advances only `left`'s van Herk phase,
  `crates/true-peak-limiter/src/lib.rs:2302-2312`), so the skipped update must too.
* **(c) Time.** The kernel reads no absolute time and no other state outside (b). No automation
  span, prepared target, bypass change or live record touches block N.

Two tiers satisfy this:

* **Rest (a fixed point).** `advance_silent` is bookkeeping only. Examples: the limiter's cursors
  and phase, a console stage's observation window.
* **Tail.** The output is provably `+0.0` while a recursion still decays. `advance_silent` runs
  exactly that recursion and nothing else. For example, a compressor's gain smoother on a silent
  input, where the output is `x * g = +0.0` for every finite `g >= 0`.

**The seam,** on `rack::BankStage` (`crates/rack/src/lib.rs:559`), with defaults that decline:

```rust
/// Read after `begin_block`. False on any block whose `begin_block` admitted a record.
fn silent_skippable(&self) -> bool { false }
/// A skipped silent block's exact state update (cursor bookkeeping, or a tail recursion).
/// `mono`: the chain renders this block collapsed, so only the left channel's state moves.
fn advance_silent(&mut self, frames: u32, mono: bool) {}
```

These are mirrored on `graph::GraphPreparedBuiltinBankProcessor` (`crates/graph/src/lib.rs:1048`)
and, optionally, on `effect_contract::PreparedNativeEffectBank`.

**Who implements it.**

* **Builtins** (the engine's own code; slice S3): exact, state-only checks. No payload is involved.
  * **Input section.** At rest when:
    * no trim ramp or filter ramp is in flight (`ramping`, `filter_ramping`,
      `crates/builtins/src/lib.rs:967,1004`);
    * every integrator word of every lane is `+0.0` (`state`, `:969`; eight vectors).
    * **Why this is a fixed point.** `svf_step`'s state update `flush(ic + 2d)` maps every zero to
      `+0.0` (`crates/lane/src/kernels.rs:350-359`, `lane/src/lib.rs:133`). The output mix
      `m2*v2 + (m1*v1 + m0*v0)` of the last section adds a `+0.0` term in every shape:
      * an LPF last section;
      * an HPF last section fed `+0.0`;
      * an identity section, including the elided `v + 0.0`;
      * polarity inversion, which makes `v0 = -0.0` but only on the first section.
    * S3 proves this exhaustively by test, over every shape at both widths and on the mono path.
  * **Fader.** Settled (`remaining == 0` on every lane). The gain is at least `+0.0` and mute
    clears bits, so `+0.0` maps to `+0.0` and the stage is memoryless.
  * **Matrix.** Settled. Its rest output is a signed zero per lane, and the chain's seal (4.5)
    verifies it.
  * **Precondition:** the fader and matrix bank processors drain their queues inside `process`
    (`crates/builtins-compiler/src/lib.rs:634,693`). S3 moves those drains to `begin_block`, so a
    skipped block can never drop or delay a record. This is the acked-batch question: the ack
    must never precede a drop.
* **Effects with an existing claim.** The EQ, compressor and limiter expose it through
  `PreparedNativeEffectBank::silent_rest` in slice S4.
  * EQ: `silent_fixed_point && no ramp in flight`, or bypassed (`crates/parametric-eq/src/lib.rs:2071-2141`).
  * Compressor: `silent_fixed_point && max_remaining == 0 && bypass unchanged`
    (`crates/compressor/src/lib.rs:490-555`).
  * Limiter: `silent_fixed_point && ramps stationary && bypass unchanged`, with `advance_silent`
    advancing the cursors and both channels' phase on a dual block, and only the left channel's
    on a collapsed block, exactly as its two latched bodies do
    (`crates/true-peak-limiter/src/lib.rs:2208-2231`, `:2302-2312`). The prototype advanced both
    unconditionally, which is wrong on a collapsed block (it moves a frozen right-channel phase
    that `desymmetrize` later overwrites: inaudible, but a state-payload difference). The
    prototype corpus had no collapsing fixture, so it could not see this; S4's gates now include
    one.
  * The compressor's `render` has a fourth leg, `detector` is `Main` or `Silent`
    (`crates/compressor/src/lib.rs:508-514`). It is vacuous for banks, which refuse a sidechain
    (`:1089`), and S4 must state that rather than drop it silently.
  * Each claim is earned by observing a real block. The prototype uses exactly these for the
    effects. For the builtins it used a narrower claim than S3 specifies: it declined any fader or
    matrix bank whose lanes have a console channel at all, so it never exercised a live record
    admitted during silence. S4's gates add that pattern.
* **Everything else, generically.** The owner's slot-level rule is slice S8 below. Per-effect claims
  and tail paths become optional fast paths (slice S6 for the compressor and limiter tails).

**The generic slot latch (the owner's steer), and what the audit found.** The slot wrapping an
effect with no claim of its own detects rest itself:

* On a block whose input is silent and which admitted no change, it runs the kernel and tests the
  output with #942.
* If the output is silent, it snapshots the whole bank's state payload
  (`snapshot_track_state_payload` per lane, into two preallocated buffers).
* If the snapshot equals the previous block's, the effect is at a fixed point and the slot latches.
* **Release:** a non-silent input; any drained span, prepared target or bypass change; a
  `desymmetrize`; a change of `frames`; a plan swap, which starts with fresh slots. Resets are
  unreachable on a bound bank.
* The prepared-target entry point `apply_prepared_target_lane` matters here. It is how EQ
  parameters arrive (`crates/rack/src/lib.rs:1270`), and it must release the latch.

*Soundness per effect*, from three read-only audits of the render path against each payload:

| effect | payload complete for rest? | generic compare engages? | payload per 8-lane bank | notes |
|---|---|---|---|---|
| parametric EQ | yes. `target` and `identity` are outside the payload but only read while ramping, or they only pick a schedule | yes, when the integrators flush | 7,488 B | **measured: two snapshots + compare 2.03 µs**, against the settled `process_bank` at 0.16 µs and the dense EQ slot at 1.9 µs. The per-lane `lane_get` extraction is O(W²) |
| compressor | yes. Ramp `step`, `rate_ramps` and `words` are rebuilt on restore, but they only move while `remaining > 0`, and `remaining` is in the payload | yes, at the end of the ~1.9 s tail | 1,408 B | the output is `+0.0` from the first silent sample, so an unbounded compare would snapshot every tail block; needs backoff |
| gate/expander | yes. `ramp_frames_left` is outside the payload but only selects a schedule | yes | 1,472 B | never emits exact `+0.0` from live input: gain is at least -96 dB (`kernel.rs:240-251`) |
| multiband compressor | yes (`BandCache` is a memo) | yes, about 1,700-1,800 blocks after compression | 3,008 B | no claim today (#893) |
| transient shaper | yes (`step` is 0 at rest) | yes, about 1,600-1,700 blocks after signal | 704 B | output `+0.0` from the first silent sample; no claim today (#894) |
| soft-clip | yes (logical-order histories, no cursor) | yes, after 2 blocks | 6,720 B | |
| true-peak limiter | yes, but its **cursors and van Herk phase are in the payload** | **never** (they advance every block) | 94,464 B at 48 kHz | keeps its own claim. Excluding the cursors from the compare would be unsound away from rest |
| delay | yes, but the cursor is in the payload | **never** | 768,168 B per instance at 48 kHz | **does not bank** (`bind_homogeneous_bank` returns `None`), so it never reaches a bank slot. It needs its own run-length rest (S11) |

No in-tree effect reads absolute time: `first_sample` only validates automation spans. None keeps
render state outside its struct.

**The contract obligations S8 must gate:**

1. *Payload completeness at rest.* A conformance test drives the effect to rest, snapshots it,
   restores it into a fresh instance, and requires bit-identical output from both on the same
   random signal.
2. *The frames rule.* Equality over one block proves only that the state has a period dividing
   `frames`, so the latch records `frames` and releases when it changes.
3. *A byte cap.* A slot whose payload exceeds the cap does not compare. Proposed: 8 KiB per bank.
4. *Backoff.* After a failed compare, wait 1, 2, 4 … 64 blocks before the next. A compressor tail
   of about 700 blocks then costs about 16 compares instead of 700.

**Cost, and when to stop comparing.**

* Once latched, the slot never compares again: the kernel does not run, so the state cannot move.
* Only on release does it return to candidate mode.
* The comparison therefore runs only in the settling window, and only on blocks whose output is
  already silent:
  * about 2 per silence episode for the EQ and soft-clip;
  * bounded by backoff for tails.
* The measured EQ cost argues for an optional bulk state view (`state_words`, whole vectors with no
  per-lane extraction). An effect may override it, with the payload as the fallback; the EQ's own
  `state_bits` is 24 vector stores and a 192-word compare. That is an optimisation inside S8,
  not a requirement.

**Third-party Wasm (future scope).**

* These effects never bank, and they run on sandbox workers behind at least one quantum of pipeline
  latency (`.github/ISSUE_SPECS/027-…`, `028-…`; nothing exists in code yet).
* The render side could skip exchanging a block with the worker, and the worker could skip calling
  the plugin. Either needs one of:
  * a worker-side ABI export of the render state, or a digest of the declared bounded state region,
    so the worker can run the same before/after rule;
  * a plugin-declared rest export.
* A lying plugin can only corrupt its own output, which it controls anyway. The worker can audit a
  declared claim by running the kernel every Nth latched block and comparing.
* This is post-launch.

### 4.4 Meters and observers

* **A latched chain's resident block keeps the rest output** (the seal rule, 4.5). So a resident
  observer, such as the banked meter at the final lane (#943, #950), reads correct `+0.0` words on
  a skipped block. So does an arena-buffer observer of a filled output.
* **Meters must still advance their windows and emit on schedule.** They do, because observers run
  after the unit whether or not the chain skipped.
* A known-silent flag could replace the meter's scan with a constant peak of `0.0` (slice S12, tens
  of ns per bank; small).
* An armed effect observation tap still publishes on skipped blocks: `advance_silent` of a console
  stage accumulates the resident tap value, which is unchanged. Without that, the observation
  windows would shorten.

### 4.5 The whole-bank latch (slice S4; prototyped)

In `BankChain::run_with_input`, after the drains and before the collapse decision:

1. **Qualify:**
   * every slot with active lanes is `silent_skippable()`;
   * every active lane's input is silent: a source fact, else a scan of its gather source
     (`BankMembers::input_silent`);
   * no resident predecessor and no auxiliary destination (the prototype declines both).
2. **Skip, when qualified and sealed:**
   * call `BankMembers::skip_silent(active, frames, fold)`, which writes the chain's outputs as
     constants for the **active** lanes only (a partial bank's width exceeds its population; the
     prototype passed the width and would index past a partial bank's member list, so only full
     banks were exercised);
   * call `advance_silent(frames, collapsed)` on every active slot, where `collapsed` is the mode
     the chain rendered its sealing block in (a collapse transition releases the seal);
   * return. There is no gather, no slot `process`, no scatter, and no collapse transition. The
     collapse state is left as it was, which is correct because no stage ran.
3. **Seal, when qualified but not yet sealed:** run the block normally. After the last slot, seal if
   the whole resident block is `+0.0` (#942 over 2 x 1,024 words, 55 ns, once per episode).
   * The seal is what makes the resident block valid for observers and resident successors on the
     skipped blocks that follow.
   * The first run under qualified conditions already produces the rest output, because every
     stage was at rest at its start. So the resident block holds exactly what every later skipped
     block would have produced.
4. **Unseal** on any block that does not qualify.

**The fold constant** (`ArenaMembers::skip_silent`, prototype):

* Each folded lane contributes `route_word` of `(+0.0, +0.0)` per master plane: `lr*(+0) + ll*(+0)`,
  which is `-0.0` exactly when both of that row's coefficients carry the sign bit, and `+0.0`
  otherwise (`crates/graph/src/runtime.rs:1947-1950`).
* The chain's exact effect on a master plane follows #940's skip theorem (Research findings (2),
  independently verified on 2,000,000 hostile cases):
  * **From a storing lane on:** the master becomes a constant, the left-to-right `f32` sum of the
    zeros. The fold kernels admit only lane 0 as a storing lane (`fold_resident_tiles` declines
    otherwise, `crates/graph/src/runtime.rs:1843`; `fold_cohort` writes nothing), so the skip
    declines on exactly the same premise instead of emulating an association the kernels never
    run.
  * **On a master it only accumulates into:** `master + (+0.0)` if any contribution is `+0.0`, and
    no write at all if every contribution is `-0.0`, since `-0.0` is the exact additive identity.
* An unfolded lane's output buffer is filled with `+0.0`, which is its scatter's value.
* **No FP-mode dependence.** Every skipped contribution is a signed zero and the fix-up sits at
  the chain's own position in the sum, so this rule holds under FTZ and DAZ too. (It is #940's
  deferred fix-up over *live* partial sums that needs gradual underflow.) NaN is out of the bit
  corpus, as in #940: the only difference would be an sNaN master, which cannot occur because the
  first contributor always stores an arithmetic result.

**Validated (host output only).** `silence_proto_bits` compares every output block of a latched plan
against a declined twin, bit for bit. It does not compare resident blocks, state payloads or meter
snapshots; those are argued, not measured, and the limiter finding above (collapsed blocks) is a
counterexample the output comparison cannot see. S4's gate 1 adds per-block state-payload,
resident-block and meter comparisons.

* **Strips:** console, builtins-only, EQ-only, compressor-only, gain/pan.
* **Patterns:**
  * all silent;
  * 48 of 64 silent;
  * synthetic transitions, with random runs and common downbeats;
  * the first 8,000 blocks of the dogfood pattern.
* **Widths:** eight lanes, and four lanes for builtins-only.
* **Result: 0 mismatches.** Up to 53,393 chain skips per run, with seals and releases exercised.

### 4.6 Partially silent banks

* **Per-lane masks: rejected.**
  * A lockstep kernel spends the same instructions on a silent lane as on a live one.
  * A silent lane's gather reads the silence buffer like any plane.
  * Its fold contribution is part of a whole-bank tile pass. `fold_resident_tiles` handles all
    lanes at once, so skipping one lane saves about 29 ns.
* **Accept the cost within a bank, and recover it between banks by regrouping** (slice S7, class B,
  optional):
  * Keep the user's visual order, but let the compiler form cohorts from co-silence. The activity
    maps are computed at import, off the render thread, where the decoder already touches every
    word.
  * **Bank membership already does not follow entry order:** the planner keys on
    `(level, id, program)` (`crates/graph-compiler/src/lib.rs:4126`,
    `bank_membership_is_independent_of_entry_order`).
  * **Regrouping reorders the master sum,** so it is class B. `route_fold` declines when the chain
    order and the reduction order disagree (`route_ids_ordered_against_the_cohorts_decline_the_route_fold`,
    `crates/graph-compiler/src/lib.rs:10249`). So S7 must reorder the master accumulation to cohort
    order as well, or the fold is lost. The owner has ruled class B acceptable when it is
    measurably faster.
  * **Measured, whole dogfood song, console strip:** grouping alone takes 98.1 → 78.7 µs, because
    the effects' existing bank-wide claims engage more often. Grouping and the latch together
    reach 67.0 µs.

### 4.7 Mid-chain silence

* **Producers today.**
  * A settled mute writes `+0.0` by clearing bits (`crates/lane/src/kernels/builtins.rs:164`).
  * A latched slot passes its `+0.0` input through.
  * **A gate cannot produce exact `+0.0` from live input.** Its gain is at least -96 dB
    (`range <= 96`, `gate-expander/src/lib.rs:311`; `fast_exp2` is always positive), and the output
    keeps the sign of `x`.
  * So the owner's example, a gate turning a near-silent source into true silence, needs a gate
    **hard-close mode**:
    * once the smoothed gain reaches a floor (for example -144 dB), emit `andnot(x, closed)`, which
      is `+0.0`;
    * existing sessions keep today's bits (class A for them);
    * the new mode's closed output differs from `x * 1e-7.2` (class B for that mode only);
    * this is a product ruling.
* **Detection.**
  * From state: a latched slot, a settled full mute, a hard-closed gate.
  * Otherwise from #942 on the block the slot just wrote: about 3 ns per slot on a live block,
    because it exits early, and 55 ns per eight-lane bank on a silent one.
* **Propagation (slice S9):**
  * the chain carries an `input_silent` fact from slot to slot;
  * a slot whose input is silent and which is latched skips its kernel;
  * the block is left as `+0.0` in place;
  * when every slot downstream of the silent point is latched, the chain still runs its gather but
    skips the rest.
* **Value today: nil on the standing rows,** which carry no gate, and whose effects already skip on
  their own claims. It grows with gates and with effects that have no claim. Lockstep still applies:
  all lanes' gates must be closed together.

### 4.8 Master sum and buses

* In today's rows every track folds into the master inside its chain, so the master-sum skip *is*
  the fold constant in 4.5.
* Contributors that reach the Output op or a bus reduction unfolded (ragged tails, bus returns,
  sends into buses) are handled in slice S10. `route_reduce` and `reduce_many_into`
  (`crates/graph/src/runtime.rs:601`) skip every input whose silence fact is set, with #940's
  `+0.0` fix-up and signed-zero fill.
  * #940's slice itself is moot: the builtins-less plans it targeted were removed (#956-#964).
  * Its theorem, gates and mutation list carry over unchanged.
* **Rows it moves:** none of the standing rows. A bus row would be its evidence.

### 4.9 Delays, PDC and tails

* **Pure delay lines.** PDC `CompensationDelay`, `TrackDelayLine` (`crates/graph/src/runtime.rs:652,733`)
  and the rack's bypass shunt line (`effect-contract/src/live.rs:688-790`) are pure ring swaps.
  * Rule (slice S11): keep a per-line run-length count of consecutive silent input blocks.
  * Once `count * Q >= D + Q`, every ring word is `+0.0`, the output block is silent, and the swap
    can be skipped. The cursor advance is deferred.
  * No console fixture has a delay today (`delay_samples = 0`), so S11 needs a delay-bearing variant
    of the sparse row.
* **Lookahead.** The limiter's own claim already requires uniform rings (`is_at_silent_rest`,
  `crates/true-peak-limiter/src/lib.rs:643-655`). The audit found it scans the 15.5 KB `main_ring`
  before the 8-word `reduction`; checking `reduction` first saves the ring scan on every tail block
  (a small fix inside S6).
* **The delay effect** (feedback, damping): per-node and never banked. Its rest needs its own rule:
  * the tap reads have been `+0.0` for at least the delay length;
  * the input is `+0.0`;
  * the damping state is `+0.0`.
  * Then the active window of the ring is all `+0.0`. This is later scope (S11 notes it).
* **Dynamics tails, the measured limiter of the latch on the console strip.** Two options:
  * **(T-A) Exact tail paths, class A, slice S6.** While the input is `+0.0` and the output provably
    `+0.0`, run only the decaying recursion:
    * the compressor's gain smoother;
    * the limiter's `d` once its rings are uniform;
    * later the multiband, transient and gate followers.
    * Each is expressed as `silent_skippable() = true` in the Tail tier, with `advance_silent`
      running the recursion.
    * Cost: about 2 % of the kernel.
    * Estimated effect on the console dogfood pattern: skipped banks rise from 2.07 toward the
      builtins-only 3.07 per block, and each extra skipped bank-block saves about 13 µs, so about
      -12 µs per block mean (about -13 %).
  * **(T-B) Snap to rest, class B (owner ruling).** Flush dB-domain smoothers at, say, 1e-3 dB
    instead of 1e-20. Time to rest drops from about 48τ to about 9τ. It is simpler, but it moves
    bits.

### 4.10 Worst case and realtime

* **Silence-to-dense transition, measured** (`silence_proto_downbeat`: 4,000 silent blocks then 64
  loud, six cycles, arms alternated):

  | strip | arm | last quiet block | first loud block median / max | steady loud p50 |
  |---|---|---:|---:|---:|
  | console | declined | 35.0 µs | 163.1 / 211.6 µs | 130.5 µs |
  | console | latched | 13.0 µs | 164.0 / 192.5 µs | 129.1 µs |
  | builtins-only | declined | 24.2 µs | 33.9 / 62.1 µs | 25.1 µs |
  | builtins-only | latched | 4.0 µs | 34.8 / 39.9 µs | 25.3 µs |

  * **The latch adds nothing measurable** to the downbeat or to steady dense blocks.
  * Chunk-interleaved dense A/B over 8,000 blocks: 128.4 µs declined against 129.3 µs latched (p50).
  * **Pre-existing, outside this design:** in both arms the first loud block costs 25-35 % more
    than steady dense. The dense row's p50 therefore understates the downbeat block. This is left
    as an observation for the owner. A budget should be the downbeat, not steady p50.
* **Per-block work.**
  * The qualify check short-circuits on the first slot that is not at rest: 10-20 ns per chain on
    a dense block, and the input section is slot 0.
  * The input scan runs only when every slot is at rest.
  * There is no per-sample branch.
* **Settling cost.**
  * The whole-bank latch pays one 55 ns seal scan per episode.
  * The generic payload compare (S8) pays up to 2 µs per EQ-bank attempt. That is why it is gated
    by output silence, capped and backed off.
  * The cap bounds the worst settle-block overshoot over the dense block to one snapshot per slot.
* **Safety (the contract's claim; S4's gate 1 is what proves it).** A skipped block leaves every
  observable word as the kernel would have: outputs, resident block, state, observation windows
  and cursors. Any release trigger returns the chain to the normal path the same block. The
  prototype measured the output half of this only. Evidence counters are the exception: the
  bank's `transposes`, the collapse counters, the builtin processors' qualification counters and
  the source-plane gather counters do not advance on a skipped block (S4 lists them).

## 5. Options for the owner, side by side

"M" is the minimal-complexity option the coordinator asked to be priced separately:

* a decoder silence bit;
* the builtin rest checks;
* the existing effect claims;
* one whole-bank latch.

The prototype *is* M, except that it scans the inputs instead of reading a bit.

| option | what it adds | new code per effect? | idle row (µs) | console strip, dogfood song (µs mean) | builtins-only, dogfood song (µs mean) | 4-lane builtins-only, dogfood (µs mean) |
|---|---|---|---:|---:|---:|---:|
| today | none | none | 29.4 | 98.1 | 24.5 | 42.3 |
| **M** (S1+S3+S4) | source bit, builtin rest, whole-bank latch, fold constant | none (the EQ/comp/limiter claims exist) | **4.97 measured (bound feed); about 1.5 projected after #965 and S1** | **92.5 measured** | **16.8 measured** | **20.3 measured** |
| M + S5 trims | deferred limiter advance, cached rest bits, skip the member loop when latched | none | **about 0.6-1.0 projected** | ≈ M | ≈ M | ≈ M |
| M + S6 tails (class A) | compressor and limiter tail paths | two effects, optional | ≈ M | about 80 estimated | ≈ M | ≈ M |
| M + S7 regrouping (class B) | co-silence cohorts | none | ≈ M | **67.0 measured** | **12.7 measured** | **15.1 measured** |
| G (S8, owner's steer) | generic payload latch for effects with no claim | none (payload exists) | ≈ M | ≈ M on this strip (every effect has a claim) | ≈ M | ≈ M |
| G + S9 mid-chain | per-slot skip, gate hard close | the gate's hard-close mode | ≈ M | ≈ M (no gates) | ≈ M | ≈ M |

**Reading.**
* **M captures essentially all of the measurable win on today's rows.**
* The two levers that move sparse sessions further are **rest time** (S6 or the class-B snap) and
  **grouping** (S7).
* **G's value is coverage, not cycles today.** With G, effects without claims (multiband, transient
  shaper, gate, soft-clip, future effects) stop blocking the bank latch, and #893 and #894 become
  unnecessary. G costs payload snapshots during settling, and is sound only under the S8 gates.

## 6. Prototype measurements (all under the timing lock)

* **Idle row** (`silence_proto_timing`, three interleaved repeats, 4,000 blocks each, native eight
  lanes): declined 29.35-29.39 µs against latched **4.96-4.97 µs** (p50). Eight chain skips per
  block.
* **Decomposing the latched 4.97 µs** (`silence_proto_idle_parts`, timing-only knobs):

  | removed | p50 | saving | removed in production by |
  |---|---:|---:|---|
  | nothing | 4.97 µs | | |
  | limiter cursor and phase advance | 4.57 µs | 0.40 | S5, a deferred advance |
  | input scans | 4.01 µs | 0.96 | S1, the source bit |
  | both | 3.60 µs | 1.37 | |
  | the bound copies (not a knob; probed at 2.09 µs on the latched run) | about 1.5 | 2.1 | #965, the source-set feed |

  The remaining approximately 1.5 µs:
  * drains, about 53 ns per chain;
  * member ops, about 53 ns per chain;
  * rest checks and the fold constant;
  * unit dispatch;
  * Output and enter.
  S5 targets the drains (a bind-time mask of slots whose `begin_block` does anything), the member
  loop (not needed on a skipped block) and caching of the rest bits.

* **All-silent builtins-only strip:** 23.9 → 3.85 µs at eight lanes, 41.2 → 4.41 µs at four lanes.
* **Static sparse** (tracks 16-63 silent, 6 of 8 banks): 57.6 → 38.1 µs (p50).
* **Dogfood song** (48,895 blocks, first 64 stems, bound feed, chunk-interleaved A/B, mean µs):

  | strip | width | grouping | declined | latched | skipped banks per block |
  |---|---:|---|---:|---:|---:|
  | console | 8 | file order | 98.1 | 92.5 | 2.07 / 8 |
  | console | 8 | co-silence (1 s) | 78.7 | 67.0 | 3.67 / 8 |
  | builtins-only | 8 | file order | 24.5 | 16.8 | 3.07 / 8 |
  | builtins-only | 8 | co-silence (1 s) | 24.7 | 12.7 | 4.77 / 8 |
  | builtins-only | 4 | file order | 42.3 | 20.3 | 9.31 / 16 |
  | builtins-only | 4 | co-silence (1 s) | 42.0 | 15.1 | 11.60 / 16 |

  * **"Today" here is file-name order.** The real session banks in sorted track-id order (content
    hashes), which clusters co-silence worse: 50.2 % of eight-lane bank-blocks are silent in id
    order, against 58.5 % in file order (§3). So on the real session the latch alone gains a
    little less than the file-order rows show, and regrouping gains a little more.
  * **A trap:** a first run of these that was *not* interleaved showed the console file-order latched
    arm slower (119.7 against 98.4 µs). It was host drift between two back-to-back passes of about
    6 s. The chunk-interleaved rerun above is the valid comparison.
  * Future A/Bs over long patterns must interleave.
* **Browser estimate.**
  * The wasm idle cost is about twice native (§2), so about 60 µs today, and about 2-3 µs latched.
    This is an estimate.
  * The skip fraction at four lanes is higher (§3), so the sparse-session saving in the browser is
    proportionally larger than native: -52 % against -31 % on the builtins-only strip.
* **Snapshot cost** (`silence_snapshot_cost`, W8 EQ bank, 7,488 B): two payload snapshots and a
  compare take 2.03 µs. The settled `process_bank` takes 0.16 µs, and the dense EQ slot on the
  console row takes 1.9 µs.

## 7. Plan

**The architecture issue first:** `issues/A0-silence-propagation-architecture.md` (the contract in
4.3, the latch in 4.5, the rulings). The slices follow, in order. Each has a draft in `issues/`
with authorized paths, gates and the row it moves.

| # | slice | class | depends on | row it moves | size |
|---|---|---|---|---|---|
| A0 | architecture: silent-skippable contract, silence facts, whole-bank latch, rulings | – | – | – | brief |
| S1 | carry a writer-computed silence bit on the transfer block | A | A0 | none alone (removes the S4 scans on production feeds) | small |
| S2 | add the dogfood sparse rows (production-fed, with builtins) | tooling | #965 | adds rows | medium |
| S3 | builtin rest claims; fader and matrix drains to `begin_block` | A | A0 | none alone | small |
| S4 | latch a silent bank chain (whole-bank skip, fold constant, seal) | A | S1, S3 (S2 for evidence) | `sixty_four_track_idle`, the sparse rows | medium |
| S5 | trim the latched cost (deferred limiter advance, cached rest, skip member ops, drain mask) | A | S4 | idle row toward 1 µs | small |
| S6 | exact rest tails for the compressor and limiter | A | S4 | sparse console row | medium |
| S7 | group bank cohorts by co-silence (optional) | **B** | S4, S2 | sparse rows | medium |
| S8 | generic slot latch from the state payload (the owner's steer) | A | S4 | rows with effects that have no claim (new variant) | medium |
| S9 | mid-chain silence: per-slot skip; gate hard-close mode | A (the mode is B) | S8 | a gated variant | medium |
| S10 | skip silent contributors in master and bus sums (silence table) | A | S4 | a bus variant | medium |
| S11 | rest for delay lines, PDC and bypass shunts (run length) | A | S4 | a delay variant | small |
| S12 | meters read known silence | A | S4 | sparse metered row | small |

**Minimal path** (the owner's lower-complexity choice): A0 → S1 → S3 → S2 → S4 → S5. That is the
idle row at about 1 µs and a measured sparse win. Everything after S5 is optional and ordered by
measured value: S6 and S7 first, S8 for coverage, then S9-S12.

**What the owner must rule on:**

1. **Minimal first, or generic first?** Recommended: minimal first (S1-S5), then S8 as the coverage
   step, replacing #893 and #894. The measured EQ snapshot cost (2 µs per attempt) and the
   limiter's and delay's exclusion argue for keeping per-effect claims as the fast path, with the
   generic compare as the fallback.
2. **Class B: co-silence regrouping (S7).** It reorders the master sum. Measured -20 % to -32 % on
   the console strip over the dogfood song.
3. **Class B alternative to S6:** snap dynamics tails to rest (for example a 1e-3 dB flush), or the
   exact tail paths (class A, more code in two effects).
4. **The gate hard-close mode** (S9): a new product mode that makes a closed gate emit exact `+0.0`.
   Without it, a gate never produces silence.
5. **The idle row's feed.** Its 2.1 µs of bound copies is a benchmark artifact. #965 removes it, and
   the latch's idle number should be quoted on the source-set feed.
6. **The budget row.** The first loud block after silence costs 25-35 % more than steady dense in
   both arms. Consider adding a downbeat row as the budget authority.
7. **The sparse row's fixture.** Derived per-block activity masks of 64 dogfood stems, run-length
   encoded JSON of a few KB, no audio, with the derivation program kept beside it. Agree that
   derived masks (not the private stems) may be checked in.

## 8. Files in this handoff

* `DESIGN.md`: this record.
* `issues/*.md`: draft issue bodies (A0, S1-S12). Not filed.
* `whole-bank-latch-prototype.patch`: the throwaway prototype and harnesses, applying to
  `3fc79b5a`. It contains:
  * **the latch:** rack, graph, builtins, builtins-compiler, effect-contract, EQ, compressor,
    limiter;
  * **the pattern source:** console-workload;
  * **harnesses:** `silence_profile.rs` (phase and sub-phase probes), `silence_proto.rs`
    (bit identity, A/B, interleaved A/B, downbeat, idle decomposition), and
    `parametric-eq/tests/silence_snapshot_cost.rs`.
  * The probes are not the committed #960 profiler; slices should use #960's.
  * **Prototype limitations** (S4 must not inherit them):
    * `skip_silent` is called with the bank width and walks every lane, so a partial bank (fewer
      members than lanes) would index past its member list. Only full banks were exercised; S4
      must walk active lanes only, and its ragged-strip gate covers it.
    * There is no `frames` rule and no explicit unseal on an admitted record. It relies on
      `silent_rest()` turning false, and the fader and matrix processors simply decline when any
      lane has a control channel.
    * Console effect stages, resident predecessors and auxiliary destinations always decline.
    * Evidence counters (`transposes`, builtin qualification counters) do not advance on a
      skipped block. Any pin of them on a silent row must count skips.
* `stem-silence-banking.rs.txt`: the dogfood scan and grouping program. It is a standalone Cargo
  binary with no dependencies:
  * `EXPORT64=1` writes the 64-stem pattern and groupings the harness reads from its `SCRATCH`
    path;
  * `ORDER81=<file>` scores a given order.

## 9. Appendix: groupings found (all 81 stems)

These are illustrative, not prescriptive.

**Eight lanes, optimised at a 1 s allowance.** Every bank has eight members except the last, which
has one.

| bank | members |
|---:|---|
| 1 | FX_10, FX_12, FX_9, FX_11, FX_14, FX_7, BV_S_13, BV_S_14 |
| 2 | DRUMS_6, BV_S_16, BV_S_18, BV_S_12, DRUMS_22, VOCAL FX_1, BV_S_40, DRUMS_19 |
| 3 | SYNTH_1, FX_13, DRUMS_10, DRUMS_2, SYNTH, DRUMS_26, INTRO FX_1, BASS_2 |
| 4 | BV_S_45, BV_S_37, DRUMS_18, SYNTH_4, BV_S_27, BV_S_25, BV_S_22, BV_S_23 |
| 5 | BV_S_35, BV_S_3, LEAD VOX_1, SYNTH_5, SYNTH_2, FX_2, BV_S_17, FX_3 |
| 6 | BV_S_36, BV_S_38, BV_S_39, BV_S_4, BV_S_19, BV_S_21, BV_S_26, BV_S_43 |
| 7 | BV_S_29, BV_S_9, BASS_1, BV_S_15, GUITAR_1, DRUMS_12, SYNTH_8, SYNTH_6 |
| 8 | BV_S_7, DRUMS_14, DRUMS_25, DRUMS_16, BV_S_41, DRUMS_15, DRUMS_24, DRUMS_17 |
| 9 | FX_1, BV_S_30, BV_S_44, DRUMS, DRUMS_11, BV_S_31, BV_S_33, BV_S_28 |
| 10 | DRUMS_23, BV_S_1, BV_S_2, DRUMS_8, DRUMS_9, FX_8, DRUMS_20, BASS_3 |
| 11 | FX_17 |

**What the grouping does.** Transition effects end up with transition effects, and each backing
vocal section with itself. Drums are split by the passages they play in rather than kept as a kit.
This is the co-silence structure a musician's visual order does not express.
