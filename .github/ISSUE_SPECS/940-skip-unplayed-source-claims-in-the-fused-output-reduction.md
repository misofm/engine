# Skip unplayed source claims in the fused Output reduction

Silence masks, slice 1: skip work for a track whose block is exactly zero. This file holds the
research record for the whole topic in `## Research findings` and the first, smallest closable
slice. Every other outcome is a named successor (`## Successors`). Base: `main` at `14f2917b`
(after #925-#928). Every `file:line` below was checked on that tree; after #936 and #937 land,
re-read the cited functions before editing.

## Product outcome

On a plan whose Output op carries the fused route reduction (`route_reduce`, issues #926/#927),
an input read in place from a source claim that **played no block this quantum** (underrun, end
of region: `played_planes(claim) == None`) is still read and mixed today. The kernel reads the
arena silence buffer, runs the route's 2x2 over 128 frames of `+0.0` and adds the result into the
master. The source set already knows this block is silent, so no detection is needed.

After this issue, the reduction skips such an input entirely: it is not paired, loaded, mixed or
added. When a skipped input's silent mix would have been `+0.0` on a plane, that plane gets one
`x + (+0.0)` after the chain. This is the only arithmetic the skip changes, and it restores
today's bits exactly (proof below).

Class A: bit-identical host planes on every block. Measured in the engine on the driver-fed
plumbing row, same runtime, A/B:

* no absent claims: the skip itself costs nothing;
* each absent claim saves about 55 cycles;
* 48 of 64 claims absent: 6,860 to 4,300 cycles per block (-37 %).

## Rulings (coordinator, 2026-09-26)

1. **File this slice.** The owner's standing goal is the no-effects path at its floor, and this
   slice fixes the exact skip rule and its gates on the smallest surface, which S5 reuses. It
   moves only bankless plans: `output_route_fold` admits nothing else (`runtime.rs:7055`), and no
   product host compiles a builtins-less plan today. The production wins are S3-S5.
2. **Sequence after "Resolve Output inputs in groups of eight and tighten the pair kernel" (#937).**
   That issue lands first and owns the resolve-before-pair restructure. This slice then builds on
   its group table: an unplayed input is not entered into the group (live inputs are compacted in
   edge order), and the fix-up runs after the chain. The "restr" arm below is #937's effect, not
   this issue's. The register-pressure hazard below still applies: the pair body stays outlined.
3. **The sparse row** is the companion tooling issue #941, which lands first, defined as below
   (claims of `ch16..ch63` play no block).
4. **Successors.** S3 (vectorise `block_is_positive_zero`) is small, independent and moves the
   idle row; it is #942. S5 needs an amended architecture issue first.

## Amendments (adversarial verification, 2026-09-26; these override any conflicting text below)

The verification confirmed the skip theorem (2 million randomized cases, 0 mismatches, including
infinities, NaN, subnormals, exact cancellation, leading, single and all-skipped inputs, and the
fix-up at the skipped position; it breaks under FTZ exactly as stated) and that FTZ/DAZ are clear
at every native render entry. It found the brief written against `main`'s kernel rather than #937's.

1. **Build on #937's kernel (H3).** This issue leaves `route_group`, `route_run`, `route_tail` and
   `mix_chunk` untouched. The resolve loop enters only **live** inputs and their routes into #937's
   eight-entry table, in edge order, and flushes the table to `route_group` when it is full. The last
   partial table takes the odd `G = 1` step as #937 does. `initial_store` is true only on the first
   `route_group` call of the block. The "Kernel" steps of the interface contract that describe a
   pending slot and `route_pair_planes` are replaced by this paragraph; the no-live-input fill and
   the per-plane `+0.0` fix-up (steps 6 and 7) stand.
2. **A real worst-case gate (M1).** With no absent claims the skip and declined arms run the same
   code, so G4's stop 1 compares nothing. Keep #937's `route_reduce` as a test-support-only base arm
   and compare the skip build against it at k = 0: the skip build's p50 may not exceed the base arm's
   by more than the spread of the base arm's three repeats.
3. **What moves (M2).** The skip fires only where the Output reads a source claim in place, which
   happens only in builtins-less plans; every product host compiles with builtins
   (`crates/host-core/src/prepare.rs:1195`, the C ABI's `compile.rs:410`, host-web). This slice is
   therefore benchmark-only: `sixty_four_track_plumbing_ring` and `sixty_four_track_plumbing_only`
   must not move beyond noise, and only the sparse row (#941) moves. Its value is the exact skip rule
   that the banked successor S5 reuses.
4. **Mutation cases (L1).** G1's corpus must contain, and G2 must name, the case that turns each
   mutation red: mutation 2, every input unplayed with every route row all-negative; mutation 3,
   skipped rows all-negative in the same block as a live chain summing to `-0.0`; mutation 4, a swap
   inside a non-first pair (a swap in the first pair is invisible, since addition is commutative);
   mutation 5, host planes filled with a sentinel before each block. Add #941's sparse-row tests to G3.
5. **No global knob (L2).** Do not add a global static absent-claims knob on `FrozenSourceDriver`: it
   duplicates #941's per-driver mechanism and would sit on the dense ring row's measured path. Reuse
   #941's absent-claims driver through a `#[doc(hidden)]` constructor.

## Invariants

* **I1, class A.** The host planes' bits equal the bits today's `route_reduce` writes, for every
  block, every fan-in of at least 2, every quantum (including one that is not a multiple of the
  lane width), and every pattern of unplayed claims. The proof is in `## Research findings`, (2).
* **I2, order.** Live inputs keep their edge order. The first live input is the stored value, as
  the first input is today. No reassociation, reordering or grouping of live terms changes.
* **I3, worst case unchanged.** The all-live block runs the same chunk loop as today, with no
  per-chunk branch added. The skip decision is one predictable branch per input per block.
  Skipping never makes a block slower than the same block with the skip declined, so the
  all-live block stays the worst case.
* **I4, no render-thread detection.** Only `played_planes(claim) == None` marks an input silent.
  No input is scanned, arena (non-claim) inputs are never skipped, and a played block of
  digital silence is not skipped (that is S2).
* **I5, realtime.** No allocation, lock, syscall or new per-track table. Per-block state is a few
  stack scalars: a pending input, `initial` and two fix-up flags. `crates/graph` stays free of
  `unsafe` and does not name `wide`.
* **I6, rule 3.** No function instantiated at the four-lane type carries more scalar than vector
  `f32` arithmetic. The silent-mix computation and every tail loop live in non-generic
  `#[inline(never)]` functions.

## Interface contract

No public API or driver-trait change. Everything is `pub(crate)` or private to
`crates/graph/src/runtime.rs`.

* **Resolution.** `OutputSources` (`runtime.rs:610-663`) gains `resolve(self, lease, position,
  buffer) -> Option<(&[f32], &[f32])>`. It returns `None` iff the input's claim is read in place
  (`claims[position] != NO_SOURCE_CLAIM` and `planes` is `Some`) and `planes.played_planes(claim)`
  is `None`. Otherwise it returns `input`'s planes, and an arena input is always `Some`. It counts
  a `None` in `test_only_count_source_plane(2)`, the "served silence" slot, exactly as `input`
  counts it today (`:653-658`), so the `[copied, played, silence]` pins keep their meaning.
  `input` stays for any other caller, or is removed if none remains.
* **Silent mix.** `silent_route_mix(route: [f32; 4]) -> [f32; 2]` is non-generic and
  `#[inline(never)]`. It returns the words the kernel would add for an all-`+0.0` input:
  `[<f32 as Lane>::fma(lr, 0.0, <f32 as Lane>::mul(ll, 0.0)), <f32 as Lane>::fma(rr, 0.0,
  <f32 as Lane>::mul(rl, 0.0))]`, the operand order of `mix_chunk` (`:938-941`). An unplayed input
  is **skipped** iff both words are zeros (`== 0.0`, either sign). It sets `fix[plane]` iff that
  plane's word is `+0.0` (`to_bits() == 0`). If a word is not a zero (a non-finite folded
  coefficient; compile rejects non-finite route coefficients and no fixture produces one), the
  input is served `ARENA_SILENCE_BUFFER` and treated as live, as today. That fallback keeps the
  argument independent of any coefficient validation.
* **Kernel.** `route_reduce::<L>` (`:714-765`) keeps its signature and its shape checks. Its body
  becomes:
  1. For each position, in edge order, resolve once. A skipped input is `continue`d.
  2. A live input fills a pending slot, or completes a pair with it.
  3. A pair runs `route_pair_planes::<L, 2>`: today's `route_pair` (`:783-834`) with resolution
     removed, taking the already-resolved planes, and `#[inline(never)]` (see Hazards).
  4. The first pair executed is `initial_store = true`.
  5. A leftover pending input runs at `G = 1`.
  6. **If no input was live,** fill each host plane with `+0.0` if `fix[plane]`, else `-0.0`.
  7. **Otherwise,** for each plane with `fix[plane]`, one pass of `x = x.add(L::zero())`: the
     vector frames at `L`, and the tail frames in a non-generic `#[inline(never)]` helper at `f32`.

  `route_tail` (`:841-880`) and `route_run` (`:886-935`) are unchanged.
* **Declined oracle.** A test-support-only thread-local flag,
  `test_only_set_output_silence_skip_declined(bool)`, re-exported beside
  `test_only_set_source_in_place_declined` (`crates/graph/src/lib.rs:27-38`). It mirrors the
  render-time `TEST_ONLY_RESIDENT_DISABLED` read in `Runtime::execute` (`runtime.rs:2834`). While
  set, an unplayed input is served `ARENA_SILENCE_BUFFER` and treated as live: today's arithmetic
  through the same kernel. A production build compiles none of it.

## Deliverables (the smallest closable slice)

Authorized paths:

* `crates/graph/src/runtime.rs`: the contract above, and its tests module;
* `crates/graph/src/lib.rs`: the re-export only;
* `crates/graph/tests/MUTATIONS.md`: a new section;
* `tools/console-workload/tests/silence_sweep.rs`: new, `#[ignore]` harness;
* `tools/console-workload/src/lib.rs`: one `#[doc(hidden)]` knob on `FrozenSourceDriver`, below;
* this spec.

1. The contract above.
2. The knob. `#[doc(hidden)] pub fn test_only_set_absent_claims_from(first: usize)` is a
   `static AtomicUsize` that defaults to `usize::MAX`. When it is set:
   * `FrozenSourceDriver::played_planes` returns `None` for claim indices at or above `first`;
   * `copy_track_input` fills `+0.0` for those claims (the driver contract,
     `crates/graph/src/lib.rs:1818-1828`).
   It is about 13 lines. The prototype patch has the same logic as a public static,
   `PROTO_FIRST_ABSENT_CLAIM`.
3. Gates G1-G6 and the evidence record in this file.

## Non-goals

* No detection of digital silence in a played block (S2).
* No change to any banked plan, bank chain, builtin or effect (S3-S5). No row with builtins may
  move a bit or a unit, since `output_route_fold` admits bankless plans only (`runtime.rs:7055`).
* No change to arena (non-claim) inputs, `reduce_plane_into`, `reduce_many` or buses.
* No change to how `played_planes` is dispatched, and no change to #937's `route_group`, `route_run`,
  `route_tail` or `mix_chunk`.
* No new benchmark row here: that is the companion tooling issue below. No class-B variant.

## Objective gates

1. **G1, bit identity.** New `crates/graph` test
   `unplayed_claims_skip_the_fused_output_reduction_bit_for_bit`.
   * **Plan.** A bankless plan whose Output op takes the fused fold with every input read in
     place: the builtins-less compile, as in #925's gate-1 test, bound through a lending driver
     shaped like `crates/graph/tests/rt10_source_in_place_alloc.rs:38-110`'s.
   * **Fan-in:** 2, 3, 5, 8, 9, 17, 20 and 64.
   * **Quantum:** 128 and 100. At 100, the tail runs.
   * **Unplayed sets:**
     * none, first, last, first two, all but one, all, and alternating;
     * 12 seeded random subsets that change every block, over 16 blocks.
   * **Live data (hostile):**
     * signed zeros;
     * subnormals `0x0000_0001..0x007F_FFFF` of both signs;
     * magnitudes `2^-24 .. 2^25`;
     * an exact-cancellation partner (the same route, negated input);
     * whole live blocks of `-0.0`.
   * **Route coefficients (hostile):** `-0.0`, both coefficients of a row negative, and mixed
     signs.
   * **Oracles.** The host planes must be bit-identical to each of:
     * (a) an **independent scalar oracle in the test**: per frame and plane,
       `v = mix(in_0); v = v + mix(in_i)`, with an unplayed input taken as `+0.0` words;
     * (b) the same runtime with the skip declined.
   * **The fix-up must be exercised.** Assert that the corpus contains at least one block where
     the oracle differs from a no-fix-up oracle.
2. **G2, red mutations** in `crates/graph/tests/MUTATIONS.md`, each applied alone:
   * (1) drop the fix-up pass;
   * (2) fill an all-unplayed block with `+0.0` regardless of the silent mixes. This needs a case
     where every skipped input's row is all-negative;
   * (3) add the fix-up whenever anything was skipped, ignoring the signs;
   * (4) swap the two inputs within a pair;
   * (5) start the first executed pair with `initial_store = false`.

   Each mutation must turn G1 red.
3. **G3, standing gates** unchanged and green:
   * `the_driver_fed_plumbing_row_renders_the_bound_rows_bits` (the dense ring row's digest equals
     the bound row's, and the pin is `[0, 64 x blocks, 0]`);
   * `the_plumbing_row_is_input_route_output_and_renders_the_base_bits` (#925, digest
     `57535244ba953d82f6c9c19428dc83a8ac412018c66acc167818e1917283f800`);
   * `the_plumbing_rows_output_fold_is_the_route_ops_own_bits`;
   * `reduction_is_left_to_right_bit_identical_to_scalar_reference`;
   * `crates/graph/tests/rt10_source_in_place_alloc.rs`, which must stay allocation-free.
4. **G4, no cliff** (in process, with a stop rule).
   * **Harness.** `silence_sweep.rs` builds `SixtyFourTrackPlumbingRing` through the knob for
     absent counts k in {0, 1, 4, 8, 16, 32, 48, 56, 64}.
   * **Per k, on one runtime:** assert skip and declined render the same 64-block digest, then
     time p50 cycles over 4,000 blocks, with three interleaved repeats of declined/skip. Run
     pinned: `CARGO_INCREMENTAL=0 taskset -c <cpu> cargo test --release -p console-workload
     --test silence_sweep -- --ignored --nocapture --test-threads 1`.
   * **Stop 1.** At k = 0 the skip arm's p50 may not exceed the declined arm's by more than the
     spread of the declined arm's three repeats.
   * **Stop 2.** For every k > 0, skip p50 must be at most declined p50 at the same k, plus the
     same spread tolerance.
   * **Record.** Copy the table into this spec as descriptive evidence. It is not a benchmark
     number.
5. **G5, rule 3.** Run `scripts/check-web-audioworklet-callgraph.py` on the AudioWorklet
   artifact built from the branch; its `check_kernel_shape` requires vector > scalar per function
   matching `4wide6f32x[48]`. List each new function's vector and scalar counts in the evidence.
   The browser artifact pin is repinned once at the batch boundary, as usual.
6. **G6, workspace gates:**
   * `cargo fmt --all --check`;
   * `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   * `cargo test -p graph`, both with and without `--features test-support`;
   * `cargo test -p console-workload`;
   * `bash scripts/check-realtime-policy.sh`, `check-graph-policy.sh` and
     `check-graph-determinism.sh`.

## Console benchmark rows

* **`sixty_four_track_plumbing_ring` can move.** The restructure alone measured -1,000 cycles
  per block in the prototype at k = 0, which is DIAGNOSIS-2 change 2's effect in a smaller form.
  Its digest must not move.
* **`sixty_four_track_plumbing_only` can move slightly.** It has the same fused Output unit, and
  its inputs are arena buffers that are never skipped. Its digest must not move.
* **Every with-builtins row must not move a bit or a unit.**
* **Companion tooling issue (file separately, run with this slice; shaped like #928):**
  `sixty_four_track_plumbing_ring_sparse`.
  * **The row.** The ring row's session and feed, with claims of tracks `ch16..ch63` (48 of
    64, 75 %) playing no block. It is a new `Workload` variant in `DRIVER_FED_WORKLOADS` (the
    same pattern as #928) and a row registered in `tools/bench/src/console.rs`, with
    `input_signal: "tone+absent"`.
  * **Pins:**
    * its own digest;
    * audible output;
    * `[copied, played, silence] = [0, 16 x blocks, 48 x blocks]`;
    * a digest equal to a variant that serves the 48 claims as `Some` all-`+0.0` planes (the
      "`None` renders as zeros" contract).
  * **Excluded from `tools/bench/src/floor.rs`.** A data-dependent row has no fixed floor.
  * **Why 48 contiguous.**
    * It is close to the measured real-session silence (91 % of track-blocks; see (4)) while
      keeping 16 live tracks, so pairs still run.
    * Contiguity keeps the with-builtins twin S5 needs bank-aligned: 6 of 8 banks silent at
      width 8, and 12 of 16 at width 4.
  * **The dense ring row stays the budget authority** (see I3).

## Dependencies

After #937 (ruling 2), which rewrites the same function. Independent of DIAGNOSIS-2
change 1 (dispatch only non-inert units), which removes the absent claims' `SourceInput`
dispatch but not their reduction work.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes,
  loads, stores, copies or branches. Every gate that says "bit-identical" is a hard stop.
- The owner's copy rule applies: this issue adds no copy.
- Render paths stay allocation-free, lock-free and syscall-free
  (`scripts/check-realtime-policy.sh` is mandatory). New render code sits inside the existing
  `REALTIME_POLICY_BEGIN/END` region of `runtime.rs`.
- Run `cargo fmt --all --check`, the workspace clippy line and the focused tests before every
  checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`.
- Do not quote a projected saving. The paired console benchmark runs once at the batch boundary.
  G4's table is descriptive.
- The prototype named below is evidence, not a patch to apply blindly. Its names are not the
  contract's.

## Hazards (what the implementer will hit)

* **Register pressure regresses the all-live path unless the pair loop is outlined.** Measured in
  a verbatim replica of the kernel:
  * pairing live inputs through a pending slot, with the chunk loop inlined into the resolving
    loop, went from 1 to 3-4 stack reloads per chunk iteration and ran 16-34 % *slower* with
    nothing silent;
  * a compaction table plus an inlined pair loop ran 16 % slower;
  * outlining the pair body (`#[inline(never)]`, planes pre-resolved) restored today's loop.

  In the engine, the outlined form was 12 % *faster* than today at k = 0. Verify with G4, and
  check that the new pair function's chunk loops match today's `route_run` loops instruction for
  instruction (`objdump -d` of the release test binary). Per 8 frames, a pair's initial-store loop
  is 4 loads, 8 `vmulps`, 6 `vaddps` and 2 stores. Its accumulate loop adds the 2 running-sum
  loads and 2 `vaddps`. Any extra stack reload inside either loop is the regression above.
* **Rule 3.** A scalar `mul`/`add` inlined into a function that also carries `f32x4` arithmetic
  counts against it. Keep `silent_route_mix` and the fix-up tail non-generic and never inlined.
  #920 failed exactly this way (560 vector against 1,680 scalar); #926 fixed it by outlining.
* **The fix-up is applied once, after the chain, per plane.** That position is valid only
  because the canonical FP environment has gradual underflow (FTZ and DAZ clear: `CANONICAL_MXCSR
  = 0x1F80`, `crates/lane/src/fpenv.rs:84`, pinned at every native render entry; FPCR 0 on
  AArch64; mandated on wasm). Under FTZ, a nonzero sum can flush to `-0.0`, and the `+0.0` would
  then have to sit at the skipped input's position. Do not "optimise" the fix-up into a bulk rule
  that ignores the signs (G2 mutation 3).
* **A lone live input** is the `G = 1` initial store. **No live input** is a fill of the signed
  zero, not a skip of the Output op: the host planes must still be written every block
  (`HostMaster`, `runtime.rs:324-362`).
* **Counters.** `test_only_count_source_plane(2)` must still count one per unplayed input per
  block. The ring-row pins at `tools/console-workload/src/lib.rs:2034-2039` stay
  `[0, 64 x blocks, 0]` because nothing is absent there.
* **`FrozenSourceDriver` holds the same frozen block for every claim of a track.** The knob must
  leave `claims` untouched and only answer `None`, or the dense digest moves.
* **NaN.** A quiet NaN partial plus `+0.0` returns the same NaN on x86 and on AArch64 with
  `FPCR.DN = 0`. Host-submitted planes are not sanitised, so NaN can reach this sum. The fix-up
  adds one such add per word. The wasm core spec allows a nondeterministic NaN payload on any add,
  so today's kernel is already exposed to the same thing there. Keep NaN out of G1's
  bit-identity corpus, or assert it separately on native only.

## Research findings

Method and hosts:

* **Host and build.** AMD EPYC 7313P (Zen 3), Linux 6.8, rustc 1.97.1, release (fat LTO, one
  codegen unit), `+avx2,+fma`, pinned to one core with `taskset`, loadavg 0.8-5.9.
* **Cycles are derived, not counted.** Nanoseconds are multiplied by a core clock calibrated from
  a dependent `vaddss` chain: 3.697-3.703 GHz on every run. This is PLAN.md's method, and `perf`
  is unavailable.
* **Instruments:**
  * **(a) A standalone replica** of `route_reduce`'s pair kernel over the row's working set:
    64 stereo blocks, 64-byte aligned, verbatim `route_run`/`mix_chunk`, no tail. It includes the
    detection kernels and a randomized identity check.
  * **(b) An in-engine prototype.** A test-support toggle selects:
    * today's `route_reduce` (base);
    * the restructured kernel with skipping off (restr);
    * the same kernel with skipping on (skip).

    A knob on `FrozenSourceDriver` makes claims absent. All three arms run on the same runtime,
    interleaved, three repeats of 4,000 blocks each. The patch is
    `docs/handoffs/plumbing-floor-2026-09-26/silence-masks-prototype.patch`, which applies to
    `14f2917b`. Run
    `CARGO_INCREMENTAL=0 taskset -c 30 cargo test --release -p console-workload --test
    silence_proto -- --ignored --nocapture --test-threads 1`.
  * **(c) A read-only scan of the dogfood stem set,** `/home/bl/misofm/agents/stems`: 81 stereo
    24-bit 44.1 kHz WAV stems of one song, 141.9 s, 48,895 blocks of 128 frames.
    * A block is **digital silence** when every PCM byte of both channels is zero. That is
      exactly an all-`+0.0` block after integer conversion.
    * A block past a stem's last frame is **absent**. The session length is the longest stem.
    * Banks are consecutive groups of W tracks in byte order of the file names, the order
      `enginectl session build --stems` uses (#333).

  (a) and (c) were throwaway programs outside the repository. Their method is stated here so
  they can be re-derived.

### (1) Where a zero block can be detected, and what detection costs

**Free signals that already exist:**

* **Source absent.** `played_planes(claim) == None` means the whole quantum is `+0.0`. It is
  known without a scan (`crates/source/src/lib.rs:1887-1893`), and both readers already serve
  `ARENA_SILENCE_BUFFER` for it (`crates/graph/src/runtime.rs:657`, `:1905`). The copy path
  fills `+0.0` (`source/src/lib.rs:1401`).
  * **Causes:** underrun, including startup and a late or stale-generation block
    (`:1324-1348`, `:1517-1520`); end of region, which is sticky until a seek (`:1309`); and the
    quantum after a seek.
  * **No pre-roll.** Every region is `[0, frames)`
    (`crates/host-core/src/prepare.rs:960,967-968`). So in a real session, "absent" means a stem
    file shorter than the song, or a fault.
  * A short final block is `Some`, with its tail zeroed once in `play()` (`:1560-1565`).
* **A settled mute writes `+0.0` by clearing bits,** not by multiplying:
  `L::load(frame).mul(gain).andnot(mute)` (`crates/lane/src/kernels/builtins.rs:164`, ramp path
  `:221-224`).
  * So `is_muted() && is_settled()` (`crates/builtins/src/lib.rs:4114`, `:4142`) knows its
    output is `+0.0`.
  * `is_muted()` alone flips at once, while the ramp is still in flight.
  * The value is small: the mute is per lane inside a bank, and the stages upstream of it must
    keep running.
* **Stateful effects at rest.** EQ, compressor and limiter already hold a bank-wide
  "silent fixed point" claim:
  * compressor `crates/compressor/src/lib.rs:509-542`;
  * EQ `crates/parametric-eq/src/lib.rs:2087-2139`;
  * limiter `crates/true-peak-limiter/src/lib.rs:2203-2259`, with `is_at_silent_rest` at `:643`.

  Each claim is earned by watching a real block. None is exposed outside its effect.

**Not free today: digital silence inside a played block.**

* **Nothing records it.** A `Some` block carries no silence bit (`TransferBlock`,
  `source/src/lib.rs:489-496`).
* **Its writers already touch every word, off the render thread:**
  * the host path's `submit_planes` `copy_from_slice` (`:899`);
  * the native decoder's `convert_frames` (`native_wave.rs:771-880`).

  An OR of the words' bits there is nearly free and carries in the struct's padding. That is
  successor S2.
* **Integer PCM zero converts to exactly `+0.0`** (`native_wave.rs:789-838`).
* **Float sources can carry `-0.0`.** Native float WAV sanitisation accepts `-0.0`
  (`sanitize_f32`, `native_wave.rs:883-892`, admits `magnitude == 0`; pinned case at `:1213`),
  and host planes are not sanitised at all. The test must be bits
  (`== 0`), not `== 0.0`, which is what `block_is_positive_zero` already does
  (`crates/effect-runtime/src/bank.rs:96-141`).

Measured detection cost on the render thread, as cycles per test, min of 40 batches of 200:

| test | silent, L1 | silent, from L2 | live (exits on the first chunk) |
|---|---:|---:|---:|
| `block_is_positive_zero` (today's helper, `bank.rs:130`) on one stereo track block, 2 x 128 words | 68 | 71 | 17 |
| a 64-word chunked OR fold (vectorises to `vpor`/`vptest`) on the same block | 29 | 43 | 15 |
| today's helper on one 8-lane bank block, 2 x 1,024 words | 523 | -- | 10 |
| the chunked fold on the same bank block | 145 | -- | 12 |

**Reading:**

* **Today's helper is not vectorised.** It runs at about 4 words per cycle: the non-exact
  `chunks(32)` with a per-chunk exit defeats the loop vectoriser. A chunked fold is 2.3-3.6x
  faster. That is successor S3.
* **A render-thread scan does not pay in the plumbing reduction.** Scanning every input costs
  about 970 cycles per block even with nothing silent (64 early exits at about 15 cycles each).
  In the replica, scan plus skip measured 5,649 cycles against 3,741, a figure that also carries
  that variant's kernel-shape penalty; see Hazards. It broke even only near 48 of 64 silent. The
  per-input work saved is only about 50-55 cycles, and the scan of a silent block costs 29-43.
* **So the plumbing path must use producer flags only (I4).** For a bank chain the arithmetic is
  the other way round. One vectorised scan of a bank block costs 145 cycles, against the roughly
  15,000 cycles a silent, settled 8-lane bank still costs today (see (4)).

### (2) Which consumers can skip exactly: the `-0.0` rule

All arithmetic is IEEE-754 binary32, round-to-nearest-even, with subnormals. That is the
canonical environment the engine pins (`fpenv.rs:84`; wasm by specification).

* **Lemma 1.** `x + y` is `-0.0` iff both `x` and `y` are `-0.0`. An exact cancellation gives
  `+0.0`, `(+0) + (-0) = +0`, and a nonzero exact sum of two floats is a multiple of `2^-149`, so
  it never rounds to zero.
* **Lemma 2.** `x + (+-0) = x`, bit for bit, for every nonzero `x`, including infinities and quiet
  NaN on the targets named under Hazards.
* **Lemma 3.** Remove zero-valued terms from a left-to-right chain. At every aligned step, the two
  running sums are then either identical or both zeros:
  * a nonzero partial absorbs a zero term (Lemma 2);
  * `0 + m = m` exactly for nonzero `m`;
  * a zero plus a zero stays a zero.

**Theorem (the skip rule).**

* Let `F` be the chain over all terms, and `F'` the chain over the live terms only, in the same
  order, with the first live term stored.
* Let every skipped term be a known signed zero `z_i`.
* Then `F = F' + (+0.0)` if some `z_i = +0.0`, and `F = F'` otherwise.
* If every term is skipped, `F = -0.0` iff every `z_i = -0.0`, else `+0.0`.

**Proof.**

1. By Lemma 3, `F` and `F'` are identical or both zeros. Leading skipped terms are covered too:
   a zero plus the first live term `t` is `t` when `t` is nonzero, and a zero otherwise.
2. If they are both zeros, Lemma 1 gives `F = -0.0` iff every term, live and skipped, is `-0.0`.
3. **Some skipped `z_i` is `+0.0`.** Then `F` is never `-0.0`:
   * if `F'` is nonzero, `F = F'`;
   * if `F'` is a zero, `F = +0.0`.

   That is `F' + (+0.0)`: Lemma 2 for nonzero `F'`, and `(+-0) + (+0) = +0`.
4. **Every skipped `z_i` is `-0.0`.** Then `F = -0.0` iff `F' = -0.0`, so `F = F'`.
5. **Every term is skipped.** The chain is the constants alone, and Lemma 1 decides it.

Two remarks:

* The rule needs gradual underflow. Under FTZ, a subnormal sum flushes to `-0.0`, and the
  position of the `+0.0` would then matter.
* `-0.0` is the true additive identity, `x + (-0.0) = x` for all `x`. So a skipped term whose
  silent mix is `-0.0` needs no fix-up at all.

**The silent term's sign.** An unplayed input is all `+0.0`, so its route mix per plane is
`(c_r * +0) + (c_l * +0)`. That is `-0.0` iff both coefficients of the row have the sign bit set,
and `+0.0` otherwise. A default pan row (`[g, +0.0]`) therefore gives `+0.0`, and the fix-up is
the common case.

**Evidence (replica, instrument (a)):** 40,000 hostile randomized cases, all bit for bit equal
to the full chain and to a scalar oracle.

* Fan-in 1-20 and 64.
* 60 % of inputs skipped.
* Live data: subnormals, `+-0`, whole `-0.0` blocks and cancellation partners.
* Coefficients: `+-0`, negatives, and `1e-30` (products that underflow to signed zeros).

Dropping the fix-up changes at least one bit in 1,005 of 5,000 cases, so the class-B variant is a
real bit change.

**By consumer:**

| consumer | exact skip? | rule |
|---|---|---|
| sum into a bus or the Output (`route_reduce`, a future bus `reduce_many`, a send's destination sum) | yes | the theorem above: skip, then one `+ 0.0` per plane when any skipped term is `+0.0`; order of live terms unchanged |
| gain `g` (fader, trim) | yes, as a known constant | `+0 * g` is `+0` if `g` has sign bit clear, else `-0`. The fader gain is always `>= 0` (`builtins/src/lib.rs:4975-4981`). Polarity folds a negative trim (`:3168`), which gives `-0.0`. |
| mute (settled) | yes | the output is `+0.0` whatever the input (`andnot`) |
| builtin 2x2 matrix / pan | yes, as a known constant | `select(identity, l, ll*l + lr*r)` (`kernels/builtins.rs:272-273`): `-0.0` only when both row coefficients are negative. Explicit `-0.0` coefficients are normalised to `+0.0` (`builtins/src/lib.rs:93-107`). |
| route 2x2 (sends, the Output fold) | yes, as a known constant | route coefficients may be negative or `-0.0` (compile rejects only non-finite and subnormal values); `silent_route_mix` computes the sign exactly |
| meters | the scan only | the builtin meter's `settled_silence` early-out (`builtins/src/lib.rs:4626-4650`) already accepts `+-0`. The window, hold and emit state must still advance, so a mask can only replace its scan. |
| filters (input HPF/LPF, EQ), dynamics, limiter, delay | only at a proven rest state | the state decays and the tail produces output. Rest means the state words have reached the exact fixed point of the kernel on `+0.0` input, and the D7 flush at `1e-20` (`lane/src/lib.rs:98-123`) makes that reachable in finite time. The existing EQ, compressor and limiter claims are exactly this. The builtin input section has none, although its fixed point is `+0.0` in every shape (derived from `kernels/builtins.rs:479-498`, `:1197`; to be proven by S4's test). |
| edge (PDC) and track delay lines | only after draining | see (3) |

**The mask is a per-block value per stereo buffer:** `Live`, or `Zero(sign_l, sign_r)`. It
propagates by the rules above:

* a known gain or matrix maps `Zero` to `Zero` with the computed sign;
* a sum of all-`Zero` terms is `Zero(-)` iff every term is `-0.0`;
* a stateful stage passes `Zero` only while it holds its rest claim;
* anything else produces `Live`.

### (3) Banking and PDC

* **Banking.** A bank's lanes run in lockstep, 8 on AVX2 and 4 on wasm/NEON:
  * the gather transposes all lanes (`rack/src/lib.rs:2649-2685`);
  * every slot processes the whole AoSoA block (`:2353-2368`);
  * the effect claims are bank-wide.

  So a bank mask is the AND of its lanes' masks. A bank of 8 with 3 silent lanes saves nothing:
  * the silent lanes cost the same SIMD work;
  * their gather reads the silence buffer like any plane;
  * their fold contribution is not a known constant unless each lane's whole chain is at rest,
    and no stage exposes a per-lane rest state.

  The real-session data below price this. 90.9 % of track-blocks are silent, but only 58.5 % of
  8-lane bank-blocks are entirely silent. About a third of silent track-blocks sit in mixed banks
  and are unrecoverable without re-banking. Re-banking by activity would reorder the master sum
  (class B) and is a structural plan change, so it is not proposed.
* **PDC and track delays** are pure ring swaps with no arithmetic:
  * `CompensationDelay` (`runtime.rs:966-1021`);
  * `TrackDelayLine` (`:1047-1110`);
  * `pdc_delay_block` (`lane/src/kernels.rs:697-714`).

  `-0.0` is preserved. Output block `k` is entirely `+0.0` iff the input was `+0.0` over samples
  `[kQ - D, kQ + Q - D)`. A conservative per-line run-length counter qualifies it after
  `ceil(D/Q) + 1` consecutive `+0.0` input blocks. At that point the ring is all `+0.0`, so the
  swap can be skipped and only the cursor advanced. No console fixture has a delay today (every
  `delay_samples` is 0), so this is successor S6 and has no row to move.

### (4) Realistic saving

**This slice, in the engine** (instrument (b); p50 cycles per block, same runtime, two runs on
cores 29 and 30):

| absent claims k | base | restr (restructure, skip off) | skip | skip vs restr | skip vs base |
|---:|---:|---:|---:|---:|---:|
| 0 | 8,628-8,705 | 7,593-7,663 | 7,593-7,667 | 0 | -1,000 (restructure) |
| 1 | 8,631-8,712 | 7,667-7,824 | 7,593-7,750 | -70 | -1,000 |
| 8 | 8,295-8,379 | 7,589-7,639 | 7,186-7,269 | -400 | -1,100 |
| 16 | 8,184-8,272 | 7,589-7,676 | 6,632-6,747 | -950 | -1,550 |
| 32 | 7,667-7,750 | 7,371-7,417 | 5,519-5,563 | -1,850 | -2,150 |
| 48 | 6,816-6,862 | 6,850-6,899 | 4,295-4,338 | -2,560 | -2,560 (-37 %) |
| 64 | 6,336-6,377 | 6,628-6,710 | 3,183-3,227 | -3,450 | -3,150 (-50 %) |

* Digests were equal for all three arms at every k. At k = 0 the digest is the plumbing row's
  `57535244...`.
* **Rows are A/B within one runtime.** Absolute numbers across rows differ by heap placement
  (DIAGNOSIS-2, question 5), so compare within a row, not down the columns.
* **The skip is worth about 55 cycles (15 ns) per absent claim.** At k = 64, what remains
  (about 3,200 cycles) is the 64 inert `SourceInput` dispatches (DIAGNOSIS-2 change 1), the 64
  `played_planes` resolutions and the fill.
* **Replica, kernel only:** 3,553 to 1,198 cycles at 48 of 64 (-66 %), 49 cycles per skipped
  input. The `+0.0` fix-up pass costs 35 cycles per block for both planes.

**A real session** (instrument (c): the dogfood stem set, 81 stems, byte-order track list):

* **Track-blocks:**
  * 9.1 % live;
  * **62.4 % digital silence** (`Some`, all words `+0.0`);
  * **28.4 % absent** (the file has ended: the free `None` this slice uses).
* **Silent tracks per block:** mean 73.6 of 81 (p10 68, p90 79). Absent-only: mean 23.0.
* **All-silent bank-blocks.** A bank qualifies only once its lanes have been silent for the
  settle allowance:

  | allowance | width 8 | width 4 |
  |---|---:|---:|
  | 1 block | 58.5 % | 73.5 % |
  | 1 s | 45.3 % | 61.4 % |
  | 4.6 s | 32.2 % | 47.7 % |

  The allowance is the time the chain's filters and detectors need to reach exact rest, and
  depends on their settings.

**What each piece is worth on such a session:**

* **This slice** on a 64-track bankless plan at the session's absent share (about 18 absent per
  block): about 1,000 cycles (0.27 us) per block.
* **With S2** (digital silence flagged as well, about 58 silent per block): about 3,200 cycles
  (0.86 us), roughly 40 % of the ring row. **Small in absolute terms, because the plumbing path
  is already cheap.**
* **The large prize is banked production sessions (S3-S5),** estimated from recorded rows, not
  measured (`artifacts/plumbing-floor/console-benchmark.accepted.jsonl`, p50):
  * `sixty_four_track_idle` (full strip, all silent, settled) is 36.6 us;
  * `sixty_four_track_console` (the same strip with tone) is 138.3 us;
  * `sixty_four_track_plumbing_only` is 3.3 us.

  The existing effect admissions already remove about 100 us on an all-silent session. What
  remains is about 4.2 us per silent 8-lane bank:
  * builtin input filters running on silence (`builtins_only` minus `dispatch_only` is about
    13 us for 64 tracks);
  * the fader and matrix kernels, sanitise and boundary scans;
  * three unvectorised admission scans per bank;
  * gather, scatter and fold.

  A whole-chain bank latch (S5) could remove most of that on 45-58 % of the banks of a session
  like this one: about 15-20 us per block for 64 tracks. S5's brief must measure it.

### (5) Realtime safety and the worst case

* **The skip is a per-block decision.** It is one predictable branch per input, with O(fan-in)
  work and no per-sample branch. The fix-up is O(frames). All state is on the stack, and nothing
  is allocated (I5).
* **The worst case is the all-live block, and it does not get worse.**
  * In the engine, skip equals restr at k = 0 to the cycle: 7,639 against 7,639 on core 29.
  * At every k > 0, skip is cheaper than both the declined arm on the same runtime and every
    k = 0 block measured.
  * G4 turns this into stop conditions.
* **Hosts must keep budgeting the dense row.** A silence-to-dense transition, such as a chorus
  where 60 tracks enter on one downbeat, costs the full dense block in a single quantum. Time a
  sparse session saves can be given to non-realtime work, but never to more realtime tracks. The
  dense rows (`sixty_four_track_plumbing_ring`, `sixty_four_track_console`) remain the budget
  authority. A sparse row is descriptive only.

## Class A / class B per change

| change | class | note |
|---|---|---|
| this slice: skip unplayed claims in `route_reduce`, with the `+0.0` fix-up and the signed-zero fill | **A** | theorem in (2); G1 against an independent scalar oracle |
| the resolve-before-pair restructure it needs | **A** | store and reload move no bit; live order unchanged; measured -1,000 cycles on the dense ring row |
| skip *without* the fix-up | B (rejected) | flips the sign of output zeros where every live mix is `-0.0` and a skipped mix is `+0.0`; it saves 35 cycles per block that had a skip; not worth a re-baseline |
| S2-S6 below | **A** | each is a skip that is exact under the stated rest or sign conditions |
| re-banking by activity, or summing live banks first | B (not proposed) | reorders the master sum; a structural plan change, with no per-block benefit over S5 |

## Successors (file each as its own issue)

* **S2. Carry a writer-computed silence bit on the transfer block.** Class A.
  * **Write side.** `TransferBlock` gains `silent: bool`. It is computed by OR-ing the written
    words' bits in `submit_planes` (`source/src/lib.rs:886-899`) and in the native decoder's
    `convert_frames` (`native_wave.rs:771-880`), over the valid frames only. The zero tail keeps
    the bit true.
  * **Read side.** It is exposed through a provided driver method, so the graph treats a silent
    `Some` like `None`.
  * **Row it moves:** a variant of the sparse row whose silent claims play `Some` zero blocks.
  * **Coverage:** on the dogfood set, a further 62.4 % of track-blocks, which the free `None`
    misses.
* **S3. Vectorise `block_is_positive_zero`** (`crates/effect-runtime/src/bank.rs:130`). Class A:
  a predicate with the same truth value.
  * Measured 523 to 145 cycles per silent bank block, and 68 to 29 per stereo track block. The
    live early exit is unchanged.
  * Every existing effect admission (EQ, compressor, limiter, and #893/#894) pays this per bank
    per block on silence.
  * Rows it moves: `sixty_four_track_idle`.
* **S4. Silent-block admission for the builtin input bank, fader and matrix.** Class A, on the
  template of #893/#894.
  * **Rest state:** every SVF integrator word is `+0.0` bits, no trim or filter ramp is in flight,
    and fader and matrix are settled.
  * **Output-sign condition:** the matrix admits only when no lane has an all-negative row, or it
    writes the constant.
  * Rows it moves: `sixty_four_track_idle` and a with-builtins sparse row.
* **S5. Latch a silent bank chain.** Class A.
  * **Condition:** every lane's claim is unplayed or silent (S1/S2 bits, no scan), and every slot
    holds its rest claim.
  * **Then** skip the gather, every slot and the scatter.
  * **Fold:** the epilogue adds each lane's constant silent contribution by this issue's
    theorem, meaning a skip plus a per-plane `+0.0` when any is `+0.0`.
  * **Needs** a rest-claim method on the bank stage trait (`effect-contract`/`rack`), which is
    cross-cutting, so it needs an amended architecture issue per `AGENTS.md`.
  * **Proposed row:** `sixty_four_track_console_sparse`, the console strip with `ch16..ch63`
    silent and the idle row's 512-block warm-up.
* **S6. Silent rest for PDC and track delay lines.** Class A. A run-length counter per line; skip
  the swap after `ceil(D/Q) + 1` silent blocks and advance the cursor. It has no row until a
  fixture carries delays.
* **Considered, not briefed:**
  * a per-lane fold skip for a settled muted lane (value small; it helps only `fold_cohort`,
    because `fold_resident_tiles` processes all lanes in one pass);
  * replacing the meter's scan with the mask (tens of cycles).
