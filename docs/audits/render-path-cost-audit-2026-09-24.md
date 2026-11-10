# Render-path cost audit — 2026-09-24

**Purpose.** Trace how audio moves through the engine from source bytes to host output, one block
at a time, and record every place the per-block cost is higher than the contract requires:
redundant copies, per-sample work that could be per-block, passes that could fuse, scalar paths
that could bank, work done for nobody (unobserved meters, bypassed slots, silent blocks), and
memory laid out against the access pattern. The goal is processing time per block. This is a
read-only audit: no benchmark was run, no rendered bit was moved, and every claim below cites the
line it was read at.

**Baseline.** `main` at `1dca662a` (2026-09-24). Five parallel read-only traces, one per stage
(orchestration and buffers; sources and input; builtins and lane; racks and effect DSP; output,
observation and hosts), each required to cite `file:line`, to distinguish *confirmed* (read in
source) from *suspected* (inferred, not measured), and to state bit-exactness risk per finding.
The builtins trace additionally instantiated the production kernels and read the emitted code at
`Simd8` (`+avx2,+fma`) and `Simd4` (`+simd128`).

**Relation to the standing record.** Issue #349 (2026-09-03, 123 findings) is the prior
efficiency audit and its lane handoffs #559/#560 record what has been delivered since. This audit
is a fresh trace, not a re-read of #349; where a fresh finding coincides with a #349 finding the
row says so and gives that finding's current state, so nothing already delivered, already ruled
null (`docs/rulings/`), or already flagged class B is proposed again. Findings are classified as
the floor ruling (`docs/rulings/effect-floor-accounting.md`) classifies them: **class A** moves no
rendered bit (fewer passes, loads, stores, copies, branches; same arithmetic in the same order);
**class B** changes arithmetic or its order and needs an owner ruling, a derived tolerance and
listening evidence before a benchmark; **class C** is a contract question (what bypass or silence
is allowed to freeze) that needs a ruling before either.

**How to read the cost estimates.** Q is the render quantum in frames (128 in every standing
record), W the bank width (8 on x86-64-v3, 4 on wasm/NEON). A *pass* is one read or write sweep
over one plane of one block. Costs are per block unless stated; the standing 64-track console
record (`artifacts/strip1/console-benchmark.accepted.jsonl`) puts the whole builtins arithmetic
for 64 tracks at roughly 17 µs and rows between 10 and 78 µs per block, which is the scale to
hold each estimate against.

## 1. The path a sample takes

### 1.1 Control plane, once per plan

Sessions compile to a `PreparedRenderPlan` (`crates/engine/src/realtime/plan.rs`) whose graph
executor (`crates/graph/src/lib.rs`, `runtime.rs`) owns a `DisjointArena` of quantum-sized planar
`f32` buffers (`crates/engine/src/realtime/disjoint.rs`), a level-major unit schedule, and one
`BankChain` per bank cohort (`crates/rack/src/lib.rs`) with an `AoSoaScratch` of `2·Q·W` words.
Everything below is per block and allocation-free; the traces found no allocation, lock, syscall
or log on any render path (see §4).

### 1.2 Per block

1. **Enter.** `RealtimePlanOwner::render_contiguous` → `enter_block`
   (`plan_exchange.rs:361-475`): one SPSC `try_pop` with an Acquire load; atomics beyond that only
   on an actual plan swap. `StartedRenderSession` pins the FP environment once per render call
   (`host-core/src/render_session.rs:104-153`), never per bank.
2. **Sources → arena.** For every track input, `SourceSet::copy_track_input`
   (`crates/source/src/lib.rs:1613-1650`) pops one `Box<TransferBlock>` from the source's SPSC
   ring and `copy_from_slice`s both planes into the track's arena input buffer. This is the only
   render-thread copy on the source side; the ring carries pointers, not samples, and the consumer
   zero-fills only the tail a short block did not reach. Upstream of the ring a PCM sample is
   written three times as `f32` (decoder staging → `TransferBlock` → arena) on the native worker
   and web paths alike (§2.2).
3. **Unit loop.** `GraphExecutor::render` (`lib.rs:2247`) runs every unit of the schedule every
   block. There is no silence, mute or activity gate at this level (§2.1 F4).
   - An **op** unit (`execute_op`, `runtime.rs:2183-2386`) optionally stages a PDC delay, reduces
     its inputs into its output buffer (`reduce_plane`, `:275-289`: a single input is a memcpy
     unless already in place; N inputs go through the lane-vectorised `reduce_many`), then does the
     node's work in place: a route is `mix2x2_block`, an effect a `dyn` call, an identity nothing.
   - A **bank** unit hands the members' input buffers straight to the chain
     (`bank_gather_source`, `:2069`) and runs `BankChain::run_with_input`
     (`rack/src/lib.rs:2125-2302`):
     a. `begin_block` on every slot (one `dyn` call each; drains the live-console queue);
     b. if mono-collapse is armed, `lane_symmetry_bank` per slot, which fans out to W
        `lane_symmetry` calls (§2.4 F20);
     c. **gather** planar → AoSoA scratch: full banks go through W×W register transposes
        (`tile_gather`, `:252-279`), partial banks (track count not a multiple of W, including one
        track) through a scalar strided `gather_lane` (`:180-199`) (§2.1 F2);
     d. every slot processes the scratch **in place**: the input builtins are one fused pass
        over both planes (sanitise, trim, HPF, LPF, boundary scan:
        `lane/src/kernels/builtins.rs:448-507`, 122 instructions per frame for 8 tracks × 2
        channels at `Simd8`); each effect slot is two `dyn` hops, a shape validation, a silent
        admission scan, the kernel, and a separate `check_block` read pass; the settled output
        builtins are one fused fader+mute+matrix pass (`fader_matrix_block`, `:253-278`,
        14 instructions per frame at `Simd8`);
     e. a collapsed (mono) block copies the left plane over the right (`:2233`);
     f. **scatter** AoSoA → planar: full bank with no fold writes straight into the member (or
        redirected consumer) buffers (`tile_scatter_direct_plane`); full bank *with* the route fold
        writes into `staging_*` and then `fold_plane`/`fold_cohort` (`runtime.rs:1218-1305`) runs
        `mix2x2_block` over the staging tile and copies or accumulates it into the master; partial
        banks scatter scalar-strided per lane, then fold the same way.
4. **Observation.** `observe_unit` (permanent bindings) or `observe_active_unit` (controlled
   bindings, cursor walk over active entries only) per unit. Meters read the resident AoSoA block
   per lane with stride W (§2.3 F1).
5. **Output.** The Output node is an in-place identity on the master buffer; the executor then
   `copy_from_slice`s both planes into the host's `PlanarBufferMut` (`lib.rs:2283-2285`). The C
   ABI passes the caller's planes straight through; the AudioWorklet copies wasm memory into the
   browser-owned output arrays once more (`worklet.js:1761-1762`), which Web Audio makes
   unavoidable.
6. **Control application.** Per effect instance and per builtin queue one Acquire load, then a
   bounded insertion into a preallocated span window (`effect-contract/src/live.rs:264-372`,
   `builtins-compiler/src/lib.rs:438-475`). No maps, no per-event allocation, no per-sample
   transcendental. Filter coefficients are designed in `f64` at control time and arrive
   pre-designed; live targets ramp linearly over 64 samples.

### 1.3 Copy count, one track to master

| stage | passes over the block | notes |
|---|---|---|
| ring → arena | 1 copy (2 planes) | `copy_channel` ×2 |
| gather planar → AoSoA | 1 transpose | scalar strided when the bank is partial |
| input builtins | 1 fused pass | at the class-A floor; recurrence-latency-bound |
| each effect slot | ≥ 2 (kernel) + 1 (`check_block`) | `check_block` fusion measured null (§3) |
| output builtins | 1 fused pass settled; 3 passes while any lane ramps | §2.3 F2 |
| scatter + fold | 1 transpose to staging + `mix2x2` pass + accumulate pass | fold declines under metering (§2.1 F1) |
| master → host | 1 copy | unavoidable under the arena ownership model |
| host → device (browser) | 1 copy | Web Audio owns the output arrays |

A sample is therefore moved five to six times per block beyond the DSP itself in the fold case,
and the fold case is the *good* case: with a meter on every track (a DAW-shaped session) the fold
and the scatter redirect both decline and the path becomes scatter → member buffer → `mix2x2` in
place → `reduce_many` into the master → host copy.

## 2. Findings

Each finding carries: an id (stage letter + number), confirmation status, class (A/B/C as defined
above), the #349 row it coincides with and that row's state, the per-block cost as read, the fix
that moves no bit, and the risk. Rows are ordered by expected per-block saving within each stage;
§5 gives the cross-stage order. Where a fresh finding turned out to be already delivered, already
null-ruled or already class-B-flagged, it is listed in §3 instead so it is not proposed twice.

### 2.1 Orchestration and buffers (`crates/engine/src/realtime`, `crates/graph`, `crates/rack` plumbing)

**O1. Metering disables the route fold and the scatter redirect.** Confirmed. Class A. New angle
on delivered RT-1/RT-2 (#399, #419/#422). `foldable_lane` (`crates/graph/src/runtime.rs:4826-4855`)
declines when the chain's last slot *or* the route is observed; `scatter_target` (`:4494-4548`)
declines when the producer or any later tap has an observer; and the resident-observe path
requires an empty fold (`:1930-1943`, `:2117-2133`). The fold (#218) and the direct scatter were
each delivered and measured on the unmetered console fixture, and neither is reachable once a
track carries a meter, which is every track of a DAW-shaped session. Cost: roughly three extra
`2·Q`-word passes per track per block (scatter to own buffer, separate `mix2x2` pass, one
`reduce_many` contributor read). Fix: let observers read the resident AoSoA lane
(`BankChain::final_output_lane`, `crates/rack/src/lib.rs:2061`) before the fold consumes the
staging tile; the fold mutates only `staging_*`, so the observer's words are identical. Risk: no
bit change; moderate code risk in the eligibility predicates. The `observed()` body (`:4764`) was
not read; the conclusion rests on `foldable_lane`'s call sites.

**O2. Partial-bank gather and scatter are scalar and strided.** Confirmed. Class A. The retained
RT-1 residual (ragged tail deliberately kept per lane). Whenever `full_bank` is false — any
cohort whose member count is not a multiple of W, including one track — `gather`
(`crates/rack/src/lib.rs:2534-2547`) and `scatter` (`:2599-2612`) use `gather_lane`/`scatter_lane`
(`:180-248`): one 4-byte load plus one strided 4-byte store per lane-sample per plane, twice per
chain per block, while the kernel still processes all W lanes. Cost: `4·Q` strided scalar moves
per active lane per chain per block against `2·Q/W` vector tiles. Fix: run the tiled path for
partial banks, feeding inactive gather lanes from `ARENA_SILENCE_BUFFER`
(`crates/engine/src/realtime/disjoint.rs:34`) and scattering inactive lanes into a chain-owned
dump buffer reserved at bind (`write_stereo_many`, `disjoint.rs:262-307`, needs distinct writable
buffers). Transposes are exact permutations: no bit change.

**O3. The fold epilogue is two passes over every tile.** Confirmed. Class A. New angle on
delivered RT-2. On the tiled path `tile_scatter` (`crates/rack/src/lib.rs:281-300`) writes the
transposed tile to `staging_*`, `fold_plane` (`runtime.rs:1218-1229`) runs `mix2x2_block` over it
and then copies or `sum_into_block`s it into the master; `fold_cohort` (`:1231-1305`) does W
`mix2x2` passes then `ordered_accumulate_block` re-reads all W tiles. Cost: two redundant
`2·Q`-word store+load round trips per lane per chain per block. Fix: a fused kernel
`out = (first ? 0 : out) + ((ll·l) + (lr·r))` consumed straight from the register tile
`tile_scatter` already holds (`:288-296`), keeping the per-element operation order; the existing
test `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` (`runtime.rs:7040`) is the
bit-identity gate. Depends on O1 for the metered case.

**O4. There is no cohort-level silence or activity gate.** Confirmed absence. Class C (needs its
own issue and a ruling). New. `GraphExecutor::render` runs `0..units.len()` (`lib.rs:2247`);
`run_with_input` processes every slot with active lanes (`rack:2201-2216`); a source underrun
writes zeros and the whole chain still runs; muted tracks, silent stems and stopped sources cost
the full chain. Per-effect silent fast paths exist for the EQ, compressor and limiter (E2), but
nothing skips gather, scatter, or a slot whose input is `+0.0` and whose tail has expired. Savings
are potentially most of the DSP on sparse sessions. Risk is high if done naively (tails,
envelopes, meters that must decay, live-console records that must drain); gate per cohort on
"input all `+0.0` AND every slot's tail expired AND no live console record", with a per-lane bit,
and prove that an audible track is never gated.

**O5. `reduce_many` re-derives arena slices inside the vector loop.** Confirmed. Class A. The
recorded RT-3 residual ("retaining checked-view access overhead"). `runtime.rs:291-327` calls
`lease.read(plane, input)` per input per vector iteration (`disjoint.rs:198-213`: bounds check,
offset multiply, `from_raw_parts`) and `lease.write(...)` for the output; `write` takes
`&mut self`, so LLVM cannot hoist the reads, and `&source[index..]` adds a bounds check per load.
Cost: about `(N+1)·Q/W` redundant scalar operations per plane per reduction; matters for the
un-folded masters and submixes of O1's metered case. Fix: a bounded `write_read_many::<N>` on
`ArenaLease` that returns the output and up to eight input slices once, then `chunks_exact`
iteration exactly as `ordered_accumulate_block` (`crates/lane/src/kernels.rs:644-695`) already
does. Bit-identical.

**O6. `observe_unit` walks every member of every bank every block on the permanent path.**
Confirmed. Class A. Open RT-12/CP-13; #816 (PR 817) delivered the controlled cursor walk, so this
now applies to permanent bindings only — which is what the web host boots (H2). `lib.rs:2254-2258`
→ `runtime.rs:1915-1968`: an O(W) eligibility predicate, then every member, then
`member.observers` (empty). Cost: O(slots × lanes) branches per bank per block, no audio work.
Fix: a per-unit `has_observers` bit at bind, or route permanent observers through the activation
snapshot so the cursor walk is the only dispatch. No risk.

**O7. PDC staging copies then swaps.** Confirmed. Class A. New. `execute_op`
(`runtime.rs:2207-2218`) `copy_from_slice`s the producer into staging for both planes, then
`CompensationDelay::process` swaps staging with the ring (`pdc_delay_block`,
`crates/lane/src/kernels.rs:697-720`). Cost: one extra `2·Q` pass per delayed edge per block. Fix:
a three-way move (`staging[i] = ring[c]; ring[c] = source[i]`) that reads the producer directly.
Pure moves; only sessions with inserted PDC delays.

**O8. The mono-collapse witness is recomputed every block.** Confirmed, minor. Class A. Matches the
"nested forwarding" residual recorded when RT-6 (#709, PR 723) delivered stage-wide witnesses;
verify against #709 before opening. `run_with_input` (`rack:2152-2156`) calls
`lane_symmetry_bank()` (`:1909-1930`) when armed: one `dyn` call per slot and, through the
default body (`:447-458`), W more `lane_symmetry` calls each. Only the drain (`begin_block`)
mutates the witness, so it can be cached and refreshed only when a drain admitted a record. Cost:
O(slots × W) indirect calls per chain per block. No risk.

**O9. The Output node's arena buffer is copied into the host planes.** Confirmed, small. Class A.
Flagged inside RT-12's comment but never indexed; RT-13 (`BufferArena` unused) is open.
`lib.rs:2283-2285` copies both planes; with exactly one input the Output node's `reduce_plane`
(`runtime.rs:279-283`) also copies the route into the arena first, so single-track and
single-submix sessions pay two copies. Cost: `2·Q` words per copy (about 1 KB at Q=128). Fix:
let the executor pass the host `PlanarBufferMut` planes as the Output node's write target
(`reduce_many` already takes `out` as an arena index), honouring `plane_stride ≠ frames`. Not
worth chasing alone; bundle with O3.

**O10. Smaller items.** `acquire_resident_input`'s partial-bank path has a loop-invariant
`if !mono` inside its per-word loop (`rack:2354-2367`; introduced by #713). Bypass restore is a
strided scalar loop per bypassed lane (`rack:1236-1250`) and `accumulate_aux` is per-frame scalar
(`:2500-2523`); both dormant today. `write_stereo_many` does an O(W²) duplicate scan per block
(`disjoint.rs:271-279`) that could be validated at bind. All class A, all negligible.

**O11. Buffer alignment (suspected, unmeasured).** `DisjointArena.cells: Box<[UnsafeCell<f32>]>`
(`disjoint.rs:72`) and `AoSoaScratch.left/right: Box<[f32]>` carry 4-byte alignment guarantees;
buffer bases are `frames·4`-aligned (`disjoint.rs:127-129`), so 32-byte AVX2 loads can split cache
lines depending on offset. `REALTIME_MEMORY.md:11` defers alignment; only SPSC cursors are
`align(64)`. Fix: over-allocate and align both to 64 bytes at bind. No bit change. Measure before
claiming a win.

### 2.2 Sources and input (`crates/source`, `crates/engine/src/realtime/spsc.rs`, host feeds)

The render side is clean: `begin_block`/`copy_channel`/`end_block` (`crates/source/src/lib.rs:1125-1240`)
are integer math, `copy_from_slice`, a tail `fill`, and SPSC atomics with the minimum ordering
(one Release store per push/pop, Acquire only when the cached cursor says full/empty,
`spsc.rs:307-313, 435-441`; cursors on separate 64-byte lines). Findings are on the feed side.

**S1. The web feed is one `postMessage` round trip per quantum per source.** Confirmed structure,
suspected cost. Class A. New; the honest-null ruling `bulk-source-submit-null.md` rejected *N
sources per call* and explicitly kept *multi-quantum submit* (#101) available, so this is not a
re-proposal. `validate_submission_metadata` (`lib.rs:1000-1008`) rejects `frames > quantum_frames`,
so `worklet.js:1396-1430` and `host.js:1209, 1234` exchange one message in and one ack out per 128
frames per source: 375 messages per second per source each way, received on the AudioWorklet
thread between `process()` calls. Note the app's shared-memory feed (misofm/app#45) removed the
message path for the app; this stands for the repository's own worklet host and for any host that
still posts. Fix: let `HostPlanarChunk.frames` be `k·quantum`; in `submit_planes` check
`available_capacity() >= k` (`spsc.rs:278-287`) and pop `k` recycled blocks *before* copying so
admission stays all-or-nothing (the acked-batch rule holds). The pinned test
`producer_submission_has_one_stamping_and_copy_body` moves with it.

**S2. The native worker decodes into a private staging block and then copies it into the recycled
`TransferBlock`.** Confirmed. Class A. Open IO-12. `native_source.rs:1010-1014, 1676-1681,
1637-1644` → `submit_planes` (`lib.rs:849-855`). Cost: `channels·Q·4` bytes per block per source
on the decode thread, pure waste. Fix: `take_recycled_block` first, decode into `block.samples`,
then publish; keep validate-before-mutate order. The same fix removes one copy on the web path if
an FFI exposes "next recycled block, commit".

**S3. Reference runner: a 512-byte `read_exact` per source per block.** Confirmed. Class A
(tooling). New. `tools/native-pcm-runner/src/lib.rs:525-538` prepares the decoder with
`max_frames_per_decode = quantum`, so the scratch is `Q·block_align` bytes and `native_wave.rs:416`
issues one `read(2)` per quantum on a raw `File` (no `BufReader`, `:476`). Fix: prepare with the
worker's shape (`worker_decode_buffer_shape`, 64 KiB, `native_source.rs:1459-1490`). Bit-exact.

**S4. Reference runner: dead `planar.fill(0.0)` before a full overwrite.** Confirmed. Class A.
New. `tools/native-pcm-runner/src/lib.rs:583`; `decode_into` writes every decoded frame of every
plane and `submit` passes `decoded_frames`. Delete it.

**S5. Decoder conversion is per-channel multi-pass with a runtime stride.** Confirmed structure.
Class A. New, low priority. `convert_frames` (`native_wave.rs:784-880`) re-reads the interleaved
bytes once per channel with `chunks_exact(block_align)` and bounds-checked
`frame[sample_offset + n]`; 24-bit sign extension branches on the data (`:812-818`). Reciprocals
are exact powers of two already. Fix: specialise on `(encoding, channels ∈ {1,2})` with a const
stride, `as_chunks`, and `(raw << 8) >> 8`. Bit-exact. Roughly 1% of a core at 64 sources.

**S6. Smaller items.** `synchronize_plan_epochs()` runs on every C-ABI submit
(`crates/capi/src/runtime/control.rs:927-929`) — measure before touching. `TransferBlock` is two
heap allocations (`Box<TransferBlock>` holding `Box<[f32]>`, `lib.rs:480-503`), a dependent
pointer chase per `copy_channel`; a per-ring slab is optional. The web FFI rebuilds the plane slot
table and calls `host.status()` twice per submit (`hosts/host-web/src/ffi.rs:5364-5417`); folds
into S1. The worker's 20 ms idle poll (`native_source.rs:533-534, 1561-1569, 1763`) belongs to
open #124 and must not be "fixed" with a render-thread unpark.

### 2.3 Builtins and lane (`crates/builtins`, `crates/builtins-compiler`, `crates/lane`)

The settled kernels are at the class-A floor and were verified in emitted code: the input chain
is 122 instructions per frame for 8 tracks × 2 channels at `Simd8` with no calls and no bounds
checks (recurrence-latency-bound, not op-bound); the settled output section is 14. Nothing below
touches those two loops.

**B1. Meters are scalar, branchy, `f64`-serial and strided per lane.** Confirmed shape; cost
estimated. Class A for peak/held/counts, class B for RMS bits. Open RT-10/IO-8; #722 delivered
resident-input meters and recorded across-track SIMD as the successor, which is exactly this.
`observe_segment` (`crates/builtins/src/lib.rs:4810-4854`) runs per lane over the resident AoSoA
block with stride W (`MeterInput::samples`, `:4394-4404`: `chunks_exact(stride).map(|f| f[lane])`),
about 30 instructions per sample per channel with four to five data-dependent branches
(`normal_or_zero`, the held-peak state machine, which the doc comment at `:4805-4809` calls
branch-free) and a serial `vcvtss2sd`+`vmulsd`+`vaddsd` energy chain. Estimate: 2-3 k cycles per
meter per block; with a meter on each of 64 tracks that is 30-45 µs per block, i.e. *more than
the whole builtins arithmetic* (≈17 µs). Fix: one `Lane`-generic bank-wide meter kernel over the
resident block, once per tap per bank: peak via `max`, counts via `1.0 & mask` accumulation as
`sanitize_gain_block` does, held peak via `select`, energy as W independent partial sums reduced
in a fixed order and promoted to `f64` once per block. Peak, held and counts can be bit-identical;
the summation-order change to RMS is the class-B half already flagged at #349 "RT-10 step 2(b)"
and needs a ruling (it moves meter words, never rendered audio). Sub-slice, bit-exact and
new: when `peak_hold_frames == 0 && !decay_enabled` (the web and native defaults,
`host-core/src/prepare.rs:1117-1118`) the held-peak state machine degenerates to a running max and
can be skipped outright.

**B2. Any ramping lane drops the whole bank to three passes.** Confirmed. Class A. New angle on
RT-4, which delivered the *settled* fusion and left the ramp fallback unchanged.
`try_process_settled_with_matrix` (`builtins:3796-3810`) returns false if either bank has a
nonzero countdown; the fallback (`builtins-compiler:1089-1090`) runs `FaderRampStage::process_plane`
on L, on R (`builtins:2731-2783`) and then `MatrixStage::process` (`:2979-3024`), each split into
a ramp segment and a settled tail: 6 loads + 6 stores per frame instead of 2 + 2, plus about
200-300 scalar ops of per-block bookkeeping (`:2748-2772`, `:2992-3015`, `sync_settled`
`:2902-2914`). A live console riding one fader per block keeps its whole eight-track cohort on this
path indefinitely. Fix: a fused `fader_matrix_ramp_block` (both channels' `GainMuteRamp` plus the
matrix ramp or settled coefficient in one traversal) that keeps the per-lane op order and the
"identity select is not applied while the matrix ramps" rule (`lane/src/kernels/builtins.rs:295-296`)
and the same segment/tail split. Bit-identical by the same argument as the settled fusion.

**B3. The filter-ramp body's codegen is poor.** Confirmed at both widths. Class A. New (body added
by #808; LANE-7 is adjacent). `input_chain_ramp_block_filter` (`lane/src/kernels/builtins.rs:711-805`)
materialises a `[left_frame, right_frame]` slice pair (`:738`) and indexes it per channel per
frame, leaving a reachable `slice_index_fail` in the frame loop, 83 stack references and 356
instructions per frame at `Simd8` against about 250 needed; two 768-byte `memcpy`s per call for
`let mut coefficients = c.section` (`:731`) and its write-back (`:797`); and per-frame
loop-invariant `current_target_identity(target)` (`:788-791`) and `coefficient.c1.neg()` (`:761`).
Runs only for the 64-frame filter ramp per event. Fix: precompute the identity masks and `-c1`
before the loop and write two explicit channel arms. No bit change.

**B4. A trim ramp with a mixed elision plan runs the elided sections anyway.** Confirmed. Class A.
Documented boundary (`effect-floor-accounting.md:376-382`), never scoped.
`input_chain_ramp_block_elided` (`kernels/builtins.rs:907-921`) short-circuits only the all-identity
plan; the common HPF-on/LPF-off bank runs both sections while any lane's trim or polarity ramps.
Cost: 24 wasted lane-ops per elided section per lane-sample while ramps are in flight. Fix:
ramp-aware const-generic arms of `mixed_channel_block` (`:1273-1314`) taking the per-frame trim
word; the elision proof (`:1041-1073`) is independent of the trim word, so bit-identical.

**B5. Post-ramp bookkeeping spills about 80 vectors per ramping block.** Confirmed. Class A.
RT-5 was delivered (#238, #496, #611) and then #808 added 39 filter word pairs on top.
`refresh_filter_channel_symmetry_post_ramp` (`builtins:1216-1289`: 39 pairs → 78 `lane_read`
spills + 8×39 scalar compares), `settle` (`:1587-1618`), `load_countdown` (`:1570-1578`),
`settle_filter` (`:1351-1387`), called from `:1693-1697, :1730, :1745-1748`. About 1.5-2 k scalar
ops per bank per ramping block, roughly 10% of the kernel. Fix: skip the 24 filter word pairs when
`!filter_ramping` and compare words vectorially (`L::eq` then one `store_bits`). Control-plane
words only.

**B6. Smaller items.** `members_sum` and the recovery check run unconditionally
(`builtins:1782-1785`: two spills + 16 `f32→u64` conversions per bank per block for a counter that
is almost always zero; gate on one `mask_any`). The scalar strip path — six plane copies, four
passes and three revalidations per track (`runtime.rs:2231-2264`, `builtins-compiler:4051-4099`,
`builtins:2529-2532`) — is reachable only on a backend with no bank width; `BuiltinChain::process_dual_mono`
(`builtins:3086-3116`) already fuses it and is never bound (#476 closed this as unsupported for
distinct output buffers). A leased meter at `PostSimd1`, `PostDynamic` or `PostSimd2PreFader`
splits the chain and adds a full AoSoA round trip (`runtime.rs:5213-5215`), a documented cliff
worth stating in the SDK.

### 2.4 Racks and effects (`crates/rack`, `crates/effect-runtime`, `crates/effect-contract`, the effect crates)

Dispatch for a bank of eight tracks with two slots is about ten passes per block and six `dyn`
calls (twenty-four with collapse armed); there is no per-sample virtual dispatch, no copy between
slots, and the 8 KiB scratch is L1-resident.

**E1. Only full, all-lanes-active cohorts are ever banked; everything else renders scalar per
node.** Confirmed, structural. Class A by contract (banking never moves a bit) but a contract
change. New; `banks.rs` cites the closed #96 F7 "no per-lane bypass mask" as the reason.
`crates/graph-compiler/src/banks.rs:214-222`: `if !group.is_full() { continue; }` and `if
!group.active_slots.iter().all(..) { continue; }`. A cohort with fewer than W members, or a slot
only some members carry, runs each node through `NodeKind::Effect` on the scalar lane
(`runtime.rs:2263-2296`). A five-track AVX2 session banks nothing; seven of eight tracks with a
compressor run seven scalar compressors. The rack's `active_lanes` mask and the identity-coefficient
`BankKernel` (`effect-runtime/src/bank.rs:270-283`) exist, but no effect implements identity
lanes and `HomogeneousBank` (`bank.rs:245-371`) has no users. Fix: let a bank bind with a per-lane
identity/bypass request, or pad the cohort with an exact identity lane and skip its scatter (O2
makes the tail cheap). Needs each effect's `bind_homogeneous_bank` to accept an identity lane and
a per-effect proof that the identity lane is exact.

**E2. Five effects have no silent fast path.** Confirmed. Class A once the fixed point is proven.
New. `multiband-compressor`, `soft-clip`, `gate-expander`, `transient-shaper` and `delay` never
check `block_is_positive_zero`; the compressor (`:513-527`), EQ (`:2079-2083`) and limiter
(`:2204-2230`, `is_at_silent_rest` `:643-660` is the template) skip settled all-`+0.0` blocks.
Muted and empty regions pay full oversampled soft-clip, two-band multiband, gate and transient DSP.
The `-0.0` argument in `bank.rs:96-128` must be repeated per effect.

**E3. Bypass semantics differ per effect, and six of seven banked effects run the full kernel
under bypass.** Confirmed. Class C (one documented rule first), then class A per effect. New. The
EQ returns before touching the planes (`parametric-eq:2850-2852`); compressor, gate, transient,
multiband, soft-clip and limiter run everything and `select` at the end. Concretely: soft-clip runs
the whole 2× oversampled chain (3 ring pushes, two 30-tap FIRs, two `cubic`s with a divide,
`soft-clip/src/kernel.rs:104-136`) to feed a `select` at `:133-134`; the limiter runs the annex-2
detector, van Herk minimum, box average and release (`:1422`, `:1666`); the multiband `BYPASS=true`
instantiation is a whole-block load/store identity loop (`:877-881`) that could early-return in the
non-`RAMPING` arm; the gate recomputes its link/bypass masks from `f32` flags every frame
(`gate-expander/src/kernel.rs:205-211, 250`; open DYN-6) before the full detector. Zero-latency
effects could early-return; latency-carrying ones need only their delay ring fed. The blocker is a
rule on whether bypass freezes state (envelopes, oversampling history) or keeps evolving it; the
contract today only requires latency preservation. Multiband early-return and the gate mask hoist
(the compressor's `Invariants` pattern, `compressor/src/kernel.rs:204-236`, which measured a native
win) are bit-neutral regardless of that rule.

**E4. A console-controlled slot with latency pays four whole-block passes per block for the dry
line, bypassed or not.** Confirmed. Class A. New (the #349 sweep judged the shunt fine for
un-bypassed cohorts and missed that `feeds_line()` charges on latency alone). `rack:1152-1158`
calls `shunt.capture` when `any_bypassed || shunt.feeds_line()`; `BypassShunt::capture`
(`effect-contract/src/live.rs:750-781`) copies both planes into `dry_*` and swaps them through the
ring with `pdc_delay_block`. With latency (limiter, soft-clip) the copy and the swap run every
block and the swapped-out samples are never read unless a bypass lands. Fix: when not bypassed,
write the input straight into the ring (one write per plane, no swap) and materialise `dry_*`
only on bypassed blocks; the restore loop (`:1232-1250`) becomes a masked select per frame. Also
applies to the scalar `ConsoleEffect` path (`runtime.rs:2296-2340`).

**E5. The delay never banks and its inner loop branches on invariants.** Confirmed. Class A. Open
FX-10/FX-11 plus a new banking angle. `bind_homogeneous_bank` returns `Ok(None)`
(`delay/src/lib.rs:592-608`, by design per the crate doc); `delay_chunk` (`:1023-1110`) branches
per sample on `damping_off`, `matrix_through`, `matrix_swap` (`:1075, 1090-1094`) and `tap_sample`
(`:1139-1150`) on `lane.fading`; `fill_windows` (`:534-547`) copies the old (and during crossfade
the new) tap window per chunk per channel and `TapWindows::new` zero-fills 2 KB per block
(`:1266`). Fix: const-generic variants as the multiband does, read taps from the ring directly
(a chunk never wraps, `:851`), and a per-lane-offset AoSoA ring to bank it (its own gate).

**E6. The limiter's per-lane fallback is scalar per lane when lookahead differs across a cohort.**
Confirmed. Class A for the kernel, but the clean fix is a planner change needing a ruling. New.
`limiter_block` (`true-peak-limiter:1680-1718`) chooses `limiter_block_per_lane` when any lane's
`LaneShape` differs; that body runs `sliding_minimum` (`:1253-1303`) and the expiry gather
(`:1400-1407`) as `for lane in 0..width` per frame per channel with `MAXIMUM_WIDTH` stack
round-trips. Making lookahead part of the program key would make cohorts uniform by construction,
the same class of change as the retired compressor-D candidate
(`effect-floor-accounting.md:909-913`), so flag rather than build. The 32×W stack staging of peaks
in `detector_chunk` (`:1750-1764`, reloaded `:1822-1823`) is a separate, small fusion (#621 kept
it by design).

**E7. The compressor's ramp path redesigns coefficients per lane per frame with scalar `expf` and
divides.** Confirmed. Class B as proposed. New (the per-lane ramp early-out is recorded
under-resolved in `compressor-idle-lane-guard-console-under-resolved.md`). `advance_ramps`
(`compressor/src/kernel.rs:141-164`) calls `design_lane` (`design.rs:154-180`) for every moving
lane every frame: `rate_coefficient` → `math::expf` twice, two divides in `GainComputerCoef::new`
(`dynamics.rs:20-33`), then `Coef::load`. Designing at segment boundaries and ramping the
designed words vectorially (the multiband/soft-clip pattern) would change bits during ramps, which
is the effect's own smoothing contract; flag for ruling with the ramp gate re-derived.

**E8. EQ elision gate.** Confirmed. Class A. Open DYN-8. `cascade_sections`
(`parametric-eq:1683-1740`) needs an extra scalar full-block scan per plane
(`block_admits_elision`, `:997-1005`) and drops sections only in multiples of the depth; identity
sections otherwise still run the SVF plus `select` (`lane/src/kernels.rs:306-322`). Vectorise the
scan or fold it into the previous slot's boundary check. The cascade depth itself is settled
(§3).

**E9. Smaller items.** Multiband ramping adds all twenty ramp vectors every frame even when one
parameter moves (`:866-872`; a per-segment moving mask, keeping the `-0.0` guard; DYN-10 adjacent).
Two shape validations per slot per block (`effect-contract:1154-1190` then each effect's own
guard; open FX-13). The compressor mono path evaluates the link on identical inputs
(`compressor/src/kernel.rs:474`; only the `max` and second `abs` are removable, `Average` must stay
`0.5a+0.5a`). `AoSoaScratch` is allocated per bound slot and all but the first dropped at prepare
(`banks.rs:262`, `runtime.rs:3110-3114`; prepare-time only). `HomogeneousBank` is dead code
(`bank.rs:245-371`) and is also the abstraction E1 needs.

### 2.5 Output, observation and hosts (`crates/host-core`, `crates/capi`, `hosts/*`, `tools/native-pcm-runner`)

**H1. The reference runner writes one sample per syscall and hashes four bytes at a time.**
Confirmed. Class A (tooling). New. `tools/native-pcm-runner/src/lib.rs:1108-1117` (`file:
Option<File>`, no `BufWriter`, `:725-735`): per sample `write_all(&4 bytes)`, `digest.update`,
`checked_add`. At Q=128 that is 256 `write(2)` calls and 256 four-byte SHA updates per block,
roughly 150-250 µs — fifty to a hundred times the engine's render time for a small session and
the throughput ceiling of every runner-based measurement. Fix: `to_le_bytes` the block into a
preallocated buffer, one `write_all`, one `digest.update`. Bit-exact; offline host.

**H2. The web host boots permanent per-track meters that run every block whether or not anyone
holds the lease.** Confirmed. Class A. New angle: #816/#818/#820 (PRs 817/819/821) delivered
`MeterBindingPolicy::Controlled` and the native selected-meter owner, and the web host never
adopted it. `host-core/src/prepare.rs:1085-1125` builds one `SelectedMeterRequest` per track when
`meter_period_frames` is set; `:1155-1180` routes the web's path to
`prepare_selected_session_builtins_between_render_calls`, which binds `Permanent`
(`builtins-compiler:3211-3253`, `:3580-3585`); `hosts/host-web/src/lib.rs:7726-7736` requests
`SAMPLE_PEAK` at `PostMatrix` for every track. The lease gates only the master-peak scan
(`:4903-4923`) and `poll_meters`; the per-track `MeterObserver` loops run regardless and their
snapshots are dropped from the depth-8 SPSC. Cost: the B1 loop, per track, per block, for nobody:
roughly 1-2% of the 2.67 ms budget at 64 tracks before B1 lands, and it also keeps O1's fold
disabled. Fix: bind web console meters `Controlled` and drive activation from the meter lease.
No bit change.

**H3. Spectrum capture is a per-sample loop with per-sample checked arithmetic and window
tests.** Confirmed. Class A. New (feature post-dates #349). `continuous_capture`
(`host-core/src/spectrum.rs:2685-2727`) → `continuous_append_sample` (`~:2560-2620`): per sample a
`checked_add`, a ring wrap branch, two `includes_left/right` branches, a validity byte write, a
`history_filled` branch and a window-end recompute, preceded by a separate `selected_is_finite`
pass (`:2740-2748`); the resident variant adds `checked_mul`/`checked_add` per frame per plane
(`:2728-2790`). About 2-2.5 k cycles per active observer per block, five to ten times a copy. Fix:
compute frames-to-take once per block, then at most two `copy_from_slice` per plane, one `fill` of
the validity bytes, one publish check.

**H4. Spectrum publish triple-copies a 16 KB window on the render thread.** Confirmed. Class A.
New. `continuous_publish` (`spectrum.rs:2615-2660`): a scalar ring→buffer reconstruction loop over
2048 frames with a per-frame wrap branch, a by-value `SpectrumWindow` stack temporary
(`[f32; 2048] × 2`, `:438-448`), then `try_push` moves the ~16 KB record (`:465-471`) into the
SPSC slot. About 48 KB of memmove per hop plus a 16 KB stack frame inside the audio callback
(the worklet's stack). Fix: reserve/commit into the queue slot (the retirement queue in
`plan_exchange.rs` already uses that pattern) and reconstruct with two `copy_from_slice` per plane.

**H5. Scalar peak scans.** Confirmed. Class A. Open IO-8. The web master-peak scan
(`hosts/host-web/src/lib.rs:4907-4923`) is scalar with two branches per sample
(`meter_sample_magnitude`, `:7239-7245`); the C-ABI scan (`crates/capi/src/ffi.rs:822-833`) is a
dependent `f32::max` chain LLVM will not vectorise and whose NaN semantics the engine forbids on
pinned paths (`builtins:4807-4809`). Both become a select-form lane max with a final reduce;
reproduce the `-0.0`/subnormal rule lane-wise.

**H6. Smaller items.** `BuiltinBatch` (`[Option<BuiltinBatchRecord>; 256]`, about 8 KB,
`builtin_batch_endpoint.rs:33, 59-66`) is copied by value twice on the block that applies it
(`:832-853`; borrow instead). Web `poll_meters` rescans `meter_pending` several times when a window
closes (`hosts/host-web/src/lib.rs:5107-5260`; #521 delivered the O(1) idle return, not this; keep a
`pending_count`). The Output-node copy is O9.

## 3. Already settled — do not re-propose

These came up in the traces and are listed so the next reader does not spend a round on them.

| candidate | why it is closed | record |
|---|---|---|
| fuse `check_block` into each kernel's final pass | measured as an upper bound at < 0.3% of the console block, under the drift floor (not ruled for delay or wasm) | `docs/rulings/d7-check-block-fusion.md` |
| EQ cascade depth 3 or 6 | depth 2 is the measured 16-`ymm` ceiling; reopen only on AVX-512/SVE | `docs/rulings/cross-bank-interleave.md:50-81` |
| bank-wide identity skip for fader/matrix | deliberate: the `andnot` makes muted `-1.0` exactly `+0.0`; both select arms are the contract | `docs/rulings/effect-floor-accounting.md:475-481` |
| dropping the D7 sanitise + boundary scan (≈40% of the input chain's lane-ops) | contract, class B | same, and the D7 ruling |
| limiter reciprocal of `hot.window`; any `1/x` replacing a divide; folding the palindromic halfband taps; fast-dB tier for the transient shaper; fused multiply-add anywhere | class B (moves rendered bits); flagged, never chased | `effect-floor-accounting.md:905-913`, `fast-db-tier-boundaries.md:46-50`, `unfused-multiply-add-audit.md` |
| limiter divides (FX-3) | disposed class B via #632 / PR 634 | #559 |
| bulk N-sources-per-call source submit | null on the shared-memory feed | `docs/rulings/bulk-source-submit-null.md` (multi-quantum submit, S1, explicitly kept open) |
| per-lane compressor ramp early-out | under-resolved at console level, not null | `docs/rulings/compressor-idle-lane-guard-console-under-resolved.md` |
| wasm `Simd8` | closed null by owner ruling | `docs/rulings/wasm-simd8-null.md` |

## 4. Verified clean

Stated so the next audit does not re-read them: no allocation, lock, syscall, log or panic path on
any `REALTIME_POLICY` region read (audit hooks compile to nothing without `realtime-audit`,
`engine/src/realtime/audit.rs:205-226`); atomics are Acquire/Release/Relaxed with one `AcqRel`
`fetch_add` on plan swap; observation publish is Relaxed stores plus a Release fence
(`observe.rs:121-153`); every arena and scratch buffer is exactly quantum-sized; delay and PDC
lines are slice swaps with no `%`; route gains are folded at bind and no orchestration loop
contains a divide or transcendental; `Lane::load`/`store` lower to `vmovups`/`v128.load`; the
denormal flush is one `andnot` (`lane/src/lib.rs:121-124`) and the NaN check is once per block;
dynamics gain computers use the vectorised polynomial `log2`/`exp2` tier; the limiter's true-peak
detector is a register-resident 12-tap polyphase with an amortised van Herk minimum; filter
design is `f64` at control time only; the FP environment is pinned once per render call; meters
take `sqrt` once per emitted window; the worklet zero-fills output only on failure paths; the
`in_flight.fill(0)` after admission is already gated (IO-7). Wasm `Simd4` kernels are shuffle-free
with two loads and two stores per frame when `simd128` is applied to the whole dependency graph —
an earlier probe that showed shuffle-heavy loops was a build error, not a codegen fact.

## 5. Suggested order

Ranked by expected per-block saving on a DAW-shaped session (many tracks, a meter on each, live
fader moves, some tracks silent), with the smallest closable slice named. Every row is class A
unless marked; class B and C rows need a ruling before an issue.

1. **H2 + O1** — bind web meters `Controlled`, then let observers read the resident lane so the
   delivered fold and direct scatter fire on metered sessions. Two issues, H2 first (one host
   change, no engine risk), O1 second (eligibility predicates; existing bit-for-bit fold test).
2. **B1** — the bank-wide meter kernel. Peak/held/counts bit-identical (one issue); the RMS
   partial-sum half is class B and goes to the owner with B1's measurement attached. Include the
   hold=0/decay-off sub-slice.
3. **E1 + O2** — identity lanes so partial cohorts bank, and tiled gather/scatter for partial
   banks. O2 first (rack only, pure permutation), then E1 per effect starting with the EQ and
   compressor (the two on every console track).
4. **B2** — the fused ramping fader+matrix pass, with B5's word-pair skip in the same issue.
5. **E4** — feed the dry line without the copy+swap when nothing is bypassed.
6. **O3 (+O9)** — the fused fold epilogue from the register tile, after O1.
7. **E2** — silent fast paths for the five effects that lack one, one issue each with its
   fixed-point proof.
8. **H3 + H4** — spectrum capture and publish as block copies with a reserve/commit push.
9. **S1 + S2** — multi-quantum host submit and decode-into-recycled-block (also drops one copy on
   the web path).
10. **O4** (class C) — cohort silence gating: brief the rule first, because it changes what a
    muted track's tail and meters do.
11. **E3** (class C) — one bypass rule, then the per-effect early returns; the multiband
    early-return and the gate mask hoist can go first since they are bit-neutral under any rule.
12. **H1, S3, S4** — three tooling one-liners that make every runner-based number honest.
13. Small class-A cleanups in one same-crate batch each: O5, O6, O7, O8, B3, B4, B6, E5, E8, E9,
    H5, H6, S5, S6.

Flagged for owner ruling, not chased: E6 (lookahead in the program key), E7 (segment-boundary
compressor design), B1's RMS reassociation, and the class-B rows in §3.

## 6. Still open in #349 and not rediscovered here

Listed so they are not lost between the two audits (state per #559 at the time of writing):
RT-11 (tile transpose crosses a by-value `[[f32; 8]; 8]` boundary), RT-13 (`BufferArena`
allocated, unused), RT-15/17 (organisational), DYN-4 (multiband `band_amplitude` re-splats),
DYN-5 (compressor link/bypass masks const-generic), DYN-7 (three branching one-poles), DYN-9 (EQ
512 B stack scratch zeroed per bank per block), DYN-10 (multiband track-major ramp storage), DYN-11
(mooted by #737 — needs a null note, not work), DYN-12/13/14/15/16, FX-4 (limiter hot-step
constant re-splat), FX-5 (limiter mono-collapse duplicate), FX-6 (soft-clip stationary split), FX-7
(soft-clip dry history double write), FX-8 (`halfband2x_decim_even` per-tap counter compare),
FX-9 (transient-shaper identity mask invariants), FX-12..19 (DRY), LANE-1/3 (Apple
`memset_pattern16` splat; AArch64 max/min fold — unsupported-target defects), LANE-5 (`ramp_block`
per-frame monotone select), LANE-6 (`svf_block_ramped` recomputes `-c1` per frame; B3 finds the
same pattern in the newer filter-ramp body), LANE-7 (six input-chain body copies; B3's body is a
seventh), LANE-8/10/11/12.

## 7. Method notes and limits

No benchmark was run and no number above is a measurement except where a cited artifact or ruling
supplies one; estimates are labelled as such and should be replaced by a frozen-workload
measurement before an issue quotes them. The traces read the x86-64-v3 and wasm `simd128`
lowerings; AArch64 was not inspected. The `observed()` body behind O1 and the exact reach of
`synchronize_plan_epochs()` (S6) were not read. Scratch evidence (probe crates and emitted
assembly) was kept outside the repository and is not part of this record.
