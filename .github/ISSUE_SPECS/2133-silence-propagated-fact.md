# Silence as a propagated fact: no scan on the render thread

Draft slice 4 (#2133) of the banking design. Successor of #1981 (merged first, owner-accepted). Class A: no
rendered word changes; the facts the packer reads change source.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

#1981 learns which lanes' inputs were exact `+0.0` by ORing the bits of every word the gather
moves (`tile_gather`, `AoSoaScratch::gather_lane`). That is a per-word operation on every bank of
every block. The fact exists for free elsewhere: the host's chunk is copied into the source ring
on the control thread, a skipped chain is known to have emitted `+0.0`, and a submix's input is
the fold of routes whose silence is known.

**Measurement note (not a claim).** #1981 attempt 3 recorded V8 all-active at +3.0/+3.2 % and
native `dispatch_only` at +0.3 to +0.7 us against its twin; the twin ran at loadavg 3.4 and the
candidate at 6.8, so the attribution of that cost to the OR is a hypothesis this slice measures,
not a finding it rests on. The slice is justified by the design rule that silence is propagated,
whatever the number comes out as.

## Smallest closable slice

- **Monotone last-non-zero frame.** The submit path (C ABI source submit and the Worker
  instance's submit, both on the control thread) advances a per-source `last_nonzero: u64` (ring
  frame coordinates) while it copies: for every word whose bits are not `+0.0` (a `-0.0` word
  counts as non-zero, conservative, cost only) it sets `last_nonzero = frame`. A block `[a, b)` is
  silent iff `last_nonzero < a` and every frame past the written end is a `+0.0` the ring emits
  itself (underrun, held seek, zeroed tail: `source/src/lib.rs:423-427`, `:2426`). One compare per
  block, `O(1)`; a seek resets the counter with the generation. No ABI change.
- **Chain flag.** `RuntimeUnit::Bank` sets `output_silent` for every populated lane when the chain
  skipped (#1107 proves every lane emitted `+0.0`), and clears it otherwise. A per-node effect op
  sets it when it skipped and clears it otherwise.
- **Route and fold flag.** A route's output is silent iff its source is silent or the route is
  muted and settled. A submix `Input` is silent iff every route into it is silent: one AND per
  route in the fold. A fold whose inputs are all `-0.0` is not flagged (the flag is conservative).
- `BankChain::input_silent_lanes` is set from the lanes' input flags before the gather; the
  gather's OR is removed and `the_gather_reports_exactly_the_silent_lanes` is deleted in the same
  PR (its claim moves to gate 1).
- The silence skip's own head test (`block_is_positive_zero` on the gathered block) is unchanged.

## Gates (one test per claim, red first)

1. `crates/source` unit test: a block after a chunk of `+0.0` is silent; one `-0.0` word in the
   last frame of the right plane makes its block not silent; a block spanning a silent chunk and
   an underrun tail is silent; a seek resets the counter. Red on a counter that ignores the right
   plane or survives a seek. Test value: red when a block with a non-zero word is reported silent.
2. `crates/graph` unit test on a two-track-one-submix plan with the test double: the submix's
   input flag is set iff both routes' sources are silent or muted-settled; a ramping mute clears
   it. Red on a flag that ignores the mute's settle. Test value: red when a submix is reported
   silent while a route still carries signal.
3. #1981's packing gate unchanged and green with its skip count asserted unchanged (474 against
   335). Red on a flag that never sets.
4. Allocation: the existing render gate; the submit path's work is on the control thread (the
   realtime-policy tool's region list, count recorded).
5. CPU, descriptive: the V8 and native all-active console rows and `dispatch_only`, before and
   after, under one lock and comparable loadavg; recorded in the spec with the loadavg of each run.

## Paths

`crates/source/src/lib.rs`, `crates/capi/src/` (submit), `hosts/host-web/src/` (the Worker
instance's submit), `crates/graph/src/runtime.rs` (flags, route AND, chain flag),
`crates/rack/src/lib.rs` (the gather OR removed; `input_silent_lanes` set from outside),
`crates/graph-compiler/tests/silent_bank_skip.rs` (gate 3 unchanged), this spec.

## Dependencies

#1981 merged. Recommended before slice 5 (#2135) so slice 5 (#2135)'s V8 baseline is the propagated one.

## Decision and evidence record

### Attempt 1 (implementation)

- **Source fact** (`crates/source/src/lib.rs`). `PcmSourceProducer::submit` keeps `last_nonzero` (absolute source frame, per generation; a chunk of another generation resets it before its own words count) while it copies, and stamps it into each `TransferBlock`. `PcmSourceConsumer::played_block_silent` is `true` when no block played (underrun, held seek, end) or the stamp is below the block's first frame (`play` zeroes a short block's tail). `SourceGraphSourceSetDriver::played_block_silent` answers per claim; a vacant entry is silent. The truth is computed where the chunk is copied, so host-core, the C ABI and host-web need no change: they all submit through `HostChunkProvider::submit`.
- **Driver interface** (`crates/graph/src/lib.rs`). `GraphPreparedSourceSetDriver::played_block_silent`, default `false`; the executor's claim copy loop states the copied buffer's flag from it. The benchmark's `FrozenSourceDriver` (`tools/console-workload`) records its frozen blocks' silence at construction; its resource test counts the flag bytes.
- **Propagation** (`crates/graph/src/runtime.rs`). `Runtime::silent`, one flag per arena buffer, stated by every writer after it writes: a skipped chain sets its unfolded lanes' outputs, a chain that ran clears them, and a folding chain clears its master; a route is silent when it mixed, its input is silent and its coefficients keep `+0.0`; a plain sum (`Identity`, not the Output, not a fold master; a bank member's reduction) is the AND of the inputs it mixes, with an inactive route left out (`sum_silent`); a staged input, an effect, a delay line, a following route and a host processor with inputs state `false`. A bank's lane fact is the played block's record for a claim gathered in place, else its input buffer's flag (`lane_input_silence`), set with `BankChain::set_input_silent_lanes` before the run, only on a grouped unit (an ungrouped unit states 0).
- **Rack** (`crates/rack/src/lib.rs`). The gather's OR is removed (`tile_gather`, `gather_lane`, `gather_lane_left`, `silent_mask`); `the_gather_reports_exactly_the_silent_lanes` is deleted; the skip's own head test is unchanged.
- **Two points the spec did not state, decided conservatively** (each only ever withholds a flag):
  1. *Route sign.* "A route is silent iff its source is silent" is not exact: `fma(b, +0, a * +0)` is `-0.0` when both of a row's coefficients are sign-negative, so a route with a negative row turns silence into `-0.0`. The route flag also requires each output row to have one finite, sign-positive coefficient (static) or one constant one (a ramping live route: the indexed ramp's `fma(k, step, start)` may pass a `+0.0` target, so a moving column never counts).
  2. *Host processor inputs.* Gate 3's plan feeds its tracks through `GraphRuntimeProcessor`s, which no engine fact describes. A host processor **with no graph input** has its output read with the skip's test (`planes_are_positive_zero`); no shipped host binds one (#965), so no production path scans. Without it gate 3 cannot stay green unchanged.
- **Known losses against #1981's OR** (cost only, not measured on a shipped row): a per-node insert never reports silence (no effect reports a skip), so a post-insert bank after one never sees a silent input; a delayed track input (`TrackDelay`) and a fold master's sum never do either.

### Attempt 1 (evidence)

- **Gate 1**, `a_played_block_is_silent_iff_its_producer_copied_no_nonzero_word` (`crates/source`): green. Red first: the counter ignoring the right plane (block 1 reported silent), and the counter surviving the seek (both post-seek blocks not silent).
- **Gate 2**, `a_submix_input_is_silent_iff_every_route_into_it_is` (`crates/graph/tests/silence_facts.rs`, two tracks, two live routes, one submix): flags `[T, F, F, F, T, F]`. Red first: a route counted silent by its mute bit instead of its settled activity (blocks 2 and 3 set).
- **Gate 3**, `a_packing_plan_renders_the_bits_of_an_unpacked_one`, unchanged: skips 474 against 335, lane moves 38, displaced 0, bits equal. Red first: no lane fact ever set (`strips are displaced at the seek` fails).
- **Gate 4**: `a_skipping_block_allocates_nothing` and the graph `rt*_alloc` tests green. `realtime-policy`: ok, 234 marked regions in 43 files (233 on main; one new region: the fact helpers). The submit's scan is in `HostChunkProvider::submit`, on the control thread, outside every region.
- **Gate 5, descriptive** (`artifacts/steps/b13-2133/`, twin of main at `5c756b894` in `main/`). One invocation each, one warmup and two rounds, uncontrolled, sequential under one flock. Loadavg at the starts: native main 3.49, native candidate 2.61, V8 main 2.65, V8 candidate 2.51. Validator PASS for both runs.
  - Native, p50 us per block, rounds 1/2, main → candidate: all-active `sixty_four_track_console` 98.65/98.69 → 105.20/99.28 (+6.6 %/+0.6 %); `dispatch_only` 9.31/9.57 → 9.40/9.43 (+1.0 %/-1.5 %); sparse 59.34/59.29 → 62.25/59.59; one-per-bank 25.08/25.02 → 26.12/25.66; toggle 99.29/99.73 → 105.86/99.79; idle 5.87/6.17 → 6.52/6.49 (+11 %/+5 %); `gain_pan_live` 13.07/12.95 → 13.54/13.67. The candidate's round 1 is high on almost every row (+4 % to +35 %, `console_meters` and `console_mono` the most), and its round 2 is within about 1 % of main on most rows: the same noise pattern #1981 recorded. Not retried.
  - V8 p50 us per block, rounds 1/2, main → candidate: all-active console 244.9/246.3 → 241.3/240.6 (-1.5 %/-2.4 %). Main includes #1981's accepted +3.0/+3.2 %, so about 2 points of it come back here. app_shape -2.8/-4.0 %, sends -3.3/-4.4 %, sparse -0.8/-8.5 %, toggle -1.0/-2.6 %, one-per-bank -0.6/-12.3 %, quiet/restated/automated arms -0.4 % to -1.2 %.
- **Bit identity** (PR evidence, not committed): all 78 native output digests equal the twin's; every V8 document digest and the quiet, restated and automated digests equal.
- **Other gates**: `cargo test` for rack, source, graph (with and without `test-support`), graph-compiler and console-workload (release), host-core and capi (release), all green. Clippy `-D warnings` (all targets, all features) on the touched crates, `cargo doc -D warnings`, `cargo fmt --check`, `check-workspace-policy.sh`, `check-graph-policy.sh`, `check-rack-policy.sh`, `check-realtime-audit-leak.sh` and `check-test-support-ci.py` all pass.

### Root rulings (2026-10-26) on attempt 1's two points

- **Route sign (root ruling, 2026-10-26): accepted.** A route counts as silent only when it is active, its input is silent, and every output row has one finite coefficient with a positive sign that does not move during a live ramp. The counterexample to the spec's original rule: `fma(b, +0.0, a * +0.0)` is `-0.0` when `a` and `b` are both sign-negative, so a silent source through a row negative on both columns gives `-0.0`, which no skip's input test accepts as silence. The rule can hold a flag back; it can never set a wrong one.
- **The one scan (root ruling, 2026-10-26): accepted.** It applies only to a host `GraphRuntimeProcessor` with no graph input, and it reads that output with the skip's own `planes_are_positive_zero`. It is safe by construction: the engine reads the words itself, so a host cannot lie through it. No shipped host binds such a processor (#965: a source set feeds every track input), so no production path scans. **Reopening condition:** if a production host ever binds such a processor, either the processor must state its silence as a fact, or the scan must be costed on that path.
