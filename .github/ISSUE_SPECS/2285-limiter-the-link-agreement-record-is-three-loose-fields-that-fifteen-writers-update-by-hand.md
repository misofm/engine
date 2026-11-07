# Limiter: the link-agreement record is three loose fields that fifteen writers update by hand

Tracker: #2258, item FX-2 (true-peak limiter). Found by the implicit-state-machine audit of
2026-11-06.

## Problem
`LimiterCore` (`crates/true-peak-limiter/src/lib.rs`) kept the #990 linked-agreement record and
its #1370 D5 re-derivation as three loose fields: `gain_linked`, `relink_frames` and
`relink_watch`. There were 30 hand-written writes to them, at 15 sites (two of the sites are
test-only oracles). A `true` record lets `limiter_block_uniform` compute a linked pair's gain path
once and mirror it onto the right channel. A wrong `true` would therefore move output bits on the
right channel. The invariant (`gain_linked` implies `gain_state_agrees`) was checked only by the
`LinkedPair` differential in `tests`.

The tracker says `gain_linked && relink_frames > 0` is an illegal combination, reachable through
the D7 recovery and a restore, and tolerated only because of the order of the checks.

## Contract
- Find out, and record, whether any sequence through the public API reaches a `true` record over
  disagreeing gain words. If one does, a regression test reaches it, red on main.
- Record which combinations of the three fields are reachable, and whether each one is legal.
- Model the record as a `LinkRecord` whose state is private, in its own module. Its states are
  Linked (with a countdown that may still run), Apart (with a countdown) and Watching. It moves
  only through named events: stated, restated, narrowed, decided, unlinked, armed, ticked,
  watched.
- `debug_assert!(linked => gain_state_agrees)` where the record is trusted: the linked decision in
  `process_block`.

## Non-goals
- The compressor, de-esser, dynamic EQ and multiband link records, if they have one.
- No rendered bit may move. No state word may move. No flag value may move, except where the old
  encoding held a combination the new one cannot represent; the evidence lists any such case.

## Gates
- A red-on-main regression test, or a recorded proof that no wrong `true` is reachable.
- A one-time no-bit-moved comparison against base, as PR evidence.
- `cargo test --locked -p true-peak-limiter` and its in-workspace consumers, clippy
  `-D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, the realtime policy
  tool.

## Evidence (attempt 1)

Base: `main` @ `24ea03525`. Line numbers below are base's.

**Reachability: no wrong `true` is reachable today.** Every write that can make the record
`true` reads the words or makes them equal:
- construction (~4162), `reset` (~4370) and the whole §4.4 recovery (~4557) state it from
  `lane_shapes_agree`, which is exact after `clear_runtime`;
- `restore_track` (~5389), the per-lane §4.4 recovery (~4566), the watch (~4302) and the end of
  the countdown (~4280) compare the gain words;
- `desymmetrize` (~4757) copies the left channel over the right.

Every other write lowers it: `reset_lane` (`&=`, ~4419), the linked decision of a rendered block
(~4501) and of a skipped one (~4904), the glide block (~4484), the collapsed block (~4667) and
`arm_relink` (~4260).

A record that is `true` over agreeing words is safe for the next block, because the decision also
requires every lane at `Maximum` and the designed words to agree. Under those legs both channels
compute their next gain words from equal operands. A block with a glide in flight unlinks the
record before it renders (~4484).

**The tracker's "illegal" combination is legal, and required.** `gain_linked && relink_frames > 0`
is reachable through all four establishing events that can land inside a countdown. A scratch
probe drove a uniform, symmetric bank at `Maximum` on twin noise through the public bank trait.
After lane 1 switched to `DualMono` and back (over 0 samples, or over 3,000 samples for a glide),
the record was:

| event inside the countdown | `[gain_linked, relink_frames, relink_watch]` after it |
|---|---|
| `restore_track_state_payload` of lane 0's own payload | `[1, 449, 0]` |
| `desymmetrize_channels` after one collapsed block, inside the glide | `[1, 2937, 0]` |
| one lane's §4.4 recovery, inside the glide | `[1, 2937, 0]` |
| the whole §4.4 recovery, inside the glide | `[1, 2937, 0]` |

The glide's next block unlinks the record and the countdown keeps running (`[0, 2425, 0]`).
The countdown must survive the establishing event: it is what brings the record back after the
glide. Gate 4 (`tests::glide::the_linked_pair_returns_once_the_words_agree`) already reaches the
whole-recovery case on purpose. At W1, W4 and W8 it requires that every block equals the reference
kernel (which never links) and that the pair links again after the glide.

Each path was rendered twice: once shipped, and once with the reference kernel
(`REFERENCE_KERNEL`). The outputs and every state word were digested after every block. All 10
path digests (5 paths, at W4 and W8) were equal between the two arms. So the combination moves no
bit.

The order of the checks is load-bearing in one place: the glide block's clear (~4484). With that
clear removed from base, gate 4 goes red at W1, block 41: "the linked-agreement record is set
over disagreeing gain words". The order is therefore already guarded by a test, and the new
model keeps it as `LinkRecord::unlink`.

The other combinations of the three fields:
- `relink_watch && relink_frames > 0`: unreachable. `arm_relink` ends the watch, and the watch
  starts only when the countdown reaches zero.
- `relink_watch && gain_linked`: unreachable through a chain. It needs a `desymmetrize` while
  the watch stands, so a collapse episode must begin during the watch. (The episode need not run
  a collapsed block: `rack`'s `skip_block`, ~4458, can take every block of it.) But `Watching`
  means the two channels' reduction words differ, so the chain's `collapse_channels_agree`
  premise does not hold and the chain cannot collapse.
  - Were it reached, base and head would differ in timing only. Base keeps the stale watch beside
    a linked record. A skipped block whose `decide` then narrows the record leaves base still
    watching, so base can re-link at a later dual block, while head stays `Apart { countdown: 0 }`.
    Every re-link is a full `gain_state_agrees`, so no bit moves either way.
  - It is the one combination the new model cannot represent. The no-bit-moved harness counted
    it and never reached it (0 steps).

**What a wrong `true` would do.** On base, a scratch test let a fixture-shaped bank at `Maximum`
link on hot material. It then moved one word of lane 0's right box ring without restating the
record, and rendered four more blocks shipped and with the reference kernel. 302 right-channel
words moved, at W1, W4 and W8, all on lane 0. That is the defect the model and the assertion
defend against: a future writer of gain words that forgets to restate the record.

So no test can be red on main for a reachable wrong `true`. The record was hardened instead,
below.

**Model chosen.** `LinkRecord` lives in its own module (`mod link`), as #2264's `SilentClaim`
does. Its one word is a private `enum State`:
- `Linked { countdown }`: the gain words agree, and a D5 countdown may still be running;
- `Apart { countdown }`: no proof; the record is re-derived when a nonzero countdown reaches zero;
- `Watching`: the D5 watch of the reduction words. It is never linked and carries no countdown.

It moves only through these methods:
- `stated(agrees)`: construction and a full reset;
- `restate(agrees)`: a restore, a §4.4 recovery and the disengage copy (`agrees = true`). It
  keeps the countdown and ends the watch;
- `narrow(holds)`: a lane reset;
- `decide(licence)`: the linked decision of a rendered and of a skipped block;
- `unlink()`: a glide's block and a collapsed block. It ends the watch;
- `arm(frames)`: a link switch;
- `tick(frames, dual, ..)`: the countdown, re-deriving from the words at zero on a dual path;
- `watch(keeps_rings, ..)`: the watch, at a dual block's end.

The restatement is exact. Base's mapping is `relink_watch` to `Watching`, else `gain_linked` to
`Linked { relink_frames }`, else `Apart { relink_frames }`. Each old site's writes are the new
method's transition under that mapping, with two changes:
- The glide block now ends a watch before it renders. Base ended it at the block's end, in
  `watch_relink(glided = true)`. No read lies between the two points, so the state after the
  block is the same. `watch_relink` lost its `glided` argument for that reason.
- `desymmetrize` ends a watch, where base left `relink_watch` set beside `gain_linked`. That is
  the one combination above that the new model cannot represent.

- `debug_assert!(!linked || gain_state_agrees(..))` right after the linked decision in
  `process_block`. That is the one place the record is trusted. It is a full ring comparison per
  linked block, in debug builds only: the release build is unchanged, under the realtime rules.
- `rest_tests::bank_state_words` now carries all three words of the record (`LinkRecord::words`,
  test-only), not only `gain_linked`. So every fast-path-versus-slow-path gate that compares
  state words now compares the countdown and the watch too. All of them pass.

**Test.** `tests::a_stale_linked_record_is_refused_where_it_is_trusted` (debug builds). At every
width (W1, W4, W8) a fixture-shaped bank at `Maximum` renders four hot blocks linked. Then one
right box-ring word of lane 0 moves without the record being restated. The next block must panic
with the record-implies-agreement assertion.
- On base `24ea03525` (the test added, reading `core.gain_linked`): `scalar: a stale linked
  record was believed` (FAILED).
- At head: passes.
- Test value: deleting or weakening the assertion turns it red. Every writer today restates the
  record, so no other test reaches a stale one. Without the assertion, such a writer would render
  the right channel from the left's gain path (302 words moved in the base experiment above).

**Mutations (head).** Each run is the whole `true-peak-limiter` suite in the dev profile.
Recorded in `tests/MUTATIONS.md`, rows 2285-1 to 2285-10. "Gate 4" is
`the_linked_pair_returns_once_the_words_agree`.
- The trust-point assertion made vacuous: red, `a_stale_linked_record_is_refused_where_it_is_trusted`.
- A glide's block no longer unlinks: red, gate 4 (the record over disagreeing gain words).
- `restate` drops the countdown, which is the tracker's model taken literally, so that a linked
  record never carries one: red, gate 4 ("the linked pair never returned").
- A collapsed block no longer unlinks: red, the M4 gate and four more.
- `decide` ignores its licence: red, 16 tests in 4 binaries, the new assertion among them.
- A restore restates the record linked whatever the words: red, three tests.
- Four rows turn nothing red, and none of them can move a bit. The argument for each is in
  `tests/MUTATIONS.md`:
  - `arm` keeps a linked record linked;
  - a lane reset never narrows the record;
  - the countdown re-derives on a collapsed bank too (equivalent under the contract, as 990-M4
    is);
  - the watch never ends on a block that breaks the rings.

**No bit moved.** A scratch harness, never committed, was adapted from #2264's. It drove 48
random cases per vector width (W4, W8), of 700 steps each, through the public bank trait. Half the
cases were uniform, symmetric banks, so that the #990 pair links. The steps covered:
- loud bursts (half the cases on twin `L == R` material), long `+0.0` stretches, `-0.0` and tiny
  single words, short and full blocks;
- NaN blocks, one lane or the whole bank, so that both §4.4 recoveries run, dual and collapsed;
- collapsed segments with their disengage copy;
- automation, both resets and both lane resets;
- link retargets, snapped and gliding over 64, 700 and 3,000 samples;
- restores from the bank's own payloads and from a loud donor's;
- in-bank and cross-bank lane swaps, and whole-bank skips whenever the bank reported rest.

It hashed (FNV-1a), after every step, every output bit, every state word (now with the record's
three words), the rest mask and the fast-path engagement count. Each case also ran a second arm
with the reference kernel, which never links, and the two arms' digests had to be equal.
- Base `24ea03525` and head, release profile: all 96 case digests identical, and all 10 path
  digests identical. In each run the shipped arm equalled the never-linking arm on every case.
  - Per run, at W4 / W8: 1,725 / 1,012 blocks rendered the linked pair, and 525 / 521 NaN
    blocks were recovered.
  - The record was linked with a countdown running on 7 / 11 steps, linked with a glide in flight
    on 4 / 8, and watching on 64 / 54.
  - Base's `relink_watch` is hashed as `relink_watch && !gain_linked`. The two are equal on every
    step the harness reached, since `linked && watching` was reached 0 times.
- Head with debug assertions on (`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true`): the new
  trust-point assertion and #2264's three ran over all 67,200 steps, and none tripped.
  - That run skips lane swaps for 8 steps after a NaN block. Otherwise `effect-contract`'s
    `swap_bank_lanes` trips its own `"a bank never refuses its own lane's payload"` assertion:
    a NaN still in a lane's delay line makes its own payload unrestorable. That is unrelated to
    this issue; see the follow-ups.

**Gates (head).**
- `cargo test --locked -p true-peak-limiter`: all pass (lib 59, every integration target).
- `cargo test --locked -p effect-compiler -p rack -p graph -p loudness`: all pass.
- `cargo test --locked -p host-core --features test-support`: all pass, with the new debug
  assertion active over every limiter session render.
- `cargo clippy --locked -p true-peak-limiter --all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p true-peak-limiter --no-deps`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo build --locked --release -p realtime-policy` then `./target/release/realtime_policy`:
  `realtime policy: ok (293 marked regions in 46 files)`.
  - The new region covers `LinkRecord`'s methods, whose bodies moved out of the #1370 region.
  - The render change adds no allocation, lock or syscall: `LinkRecord` is one `Copy` enum of at
    most a `u32` payload, and its methods are bounded by the lane and ring counts, as the code
    they replace was. The new assertion is debug-only.

**Verdict (attempt 1): PASS, with two minors folded.**
- The adversarial review found that every transition maps exactly onto base's three fields.
- It agreed that `Linked { countdown > 0 }` is required.
- It agreed that the four mutations that turn nothing red are bit-neutral.
- It reproduced the no-bit-moved harness exactly (108 of 108 lines).
- The two minors, folded:
  - the `relink_watch && gain_linked` bullet above now gives the right exclusion
    (`collapse_channels_agree`, not "no collapsed block") and the right consequence (a timing
    difference after a narrowing skipped block, not "the next dual block ends it");
  - `tests/MUTATIONS.md` row 2285-7 now says the lane reset's narrowing can fire. A restore
    compares the gain words, not the `LaneShape`s, so a linked record can stand with lane shapes
    apart. The next decision catches that case too.

**Follow-ups (not in this issue).**
- `effect_contract::swap_bank_lanes` (`crates/effect-contract/src/live.rs` ~3369): a NaN left in
  a limiter lane's history, with the block's output still finite, makes that lane's own payload
  unrestorable. When the swap's step 3 fails, its revert of the first lane then fails too
  (debug: `"a bank never refuses its own lane's payload"`). The swap returns `Restore` with the
  first lane changed. The reviewer confirmed this on base. The coordinator files it as its own
  issue; it is out of #2285's scope.
- The compressor, de-esser, dynamic EQ and multiband effects may carry a similar record (#2258
  FX-2 named the limiter only).
