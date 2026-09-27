# Dual-mono sources: what the engine does, what it misses, and a router design

Research handoff, 2026-09-27. Base: `codex/batch-plumbing-floor-2` at `6ca203f8`, on the local
branch `dual-mono-research`. Research and prototypes only: every prototype was reverted and is
recorded in [`dual-mono-prototypes.patch`](dual-mono-prototypes.patch) (applies cleanly to the
base with `git apply`). Nothing was pushed and no GitHub issue was created or edited; the draft
issue bodies are under [`issues/`](issues/).

## The answer in five lines

1. **A stereo file whose two channels are bit-identical is processed on both channels today.** The
   engine never looks at the audio. It collapses a track to one channel only when the *session*
   says both channels read one source channel, and then only on stages that are able to run one
   plane.
2. **The machinery to process one channel already exists and works** (the "mono collapse"), and it
   saves 43% of the strip on the all-mono benchmark row at both 8 and 4 lanes. What is missing is
   (a) detecting dual-mono content and turning it into a mono source, which the release CLI
   already does but nothing else does, and (b) a planner that does not throw the saving away.
3. **On the realistic session it currently throws it away.** Folding the 18 dual-mono dogfood stems
   into mono sources makes the 81-track mix with an EQ/compressor/limiter strip **33% slower** at
   8 lanes and 1% slower at 4 lanes, because splitting mono from stereo tracks leaves partial
   effect cohorts that fall back to per-node scalar code, and because interleaved mono tracks
   disable the route fold. With a remainder-aware pooling prototype the same fold is **5.3% faster
   at 8 lanes and 7.6% faster at 4 lanes**; the ceiling for these 18 stems is about 8%.
4. **The existing collapse has a wrong-audio defect** (found while designing the router, proven by
   two failing tests): a bank chain that sits *after* another chain or a per-node effect of the
   same track collapses on the track's source mapping alone, even when an upstream stage made the
   two channels differ. The fix is small and changes no measured row.
5. The dogfood stems: **18 of 81 are bit-identical dual mono, 18 more are mono content with one
   channel entirely silent**, 3 are near-mono (not bit-identical), 42 are genuine stereo. Only the
   first group can be folded exactly.

---

## 1. What the engine does today

### 1.1 The mechanism: the mono collapse (M0 to M3, #210)

A bank chain whose every active lane renders a track doing bit-identical work on both channels
runs its **upstream** slots (input builtins, then effects) over the left plane only, copies the
left plane into the right one at the **seam**, and runs the **seam-side** slots (fader, matrix)
and everything after them dual. The chain doc is `crates/rack/src/lib.rs:1651-1701`; the
dispatch is `BankChain::run_with_input` at `crates/rack/src/lib.rs:2258`.

The decision is a five-term witness per track (`crates/effect-contract/src/symmetry.rs:134-143`):

| term | meaning | decided | where |
|---|---|---|---|
| `SOURCE` | both lanes read one source channel | prepare, from the **session declaration** | `track_mono_source`, `crates/builtins-compiler/src/lib.rs:3893-3896`: `left_source_channel == right_source_channel` |
| `DESIGNED` | every per-channel word the kernel reads is bit-equal L vs R (and the input delays are equal) | prepare; builtins and console effects keep it current on live writes | input builtins `crates/builtins/src/lib.rs:2337` (26 words); effects `lane_channel_symmetry` (EQ, compressor, limiter); delay `crates/builtins-compiler/src/lib.rs:3917` |
| `LIVE` | no admitted record wrote one channel's upstream word | per block, at the queue drain | `ChannelSymmetryWitness::admit`, `symmetry.rs:262`; `ParameterChannel::Both` preserves, `Left`/`Right` clears |
| `UNBYPASSED` | no upstream stage is live-bypassed | per block, reversible | same |
| `RESTORED` | restored state payloads have byte-equal L/R sections | at restore | `payload_sections_agree` |

On top of the witness there are structural and planning conditions:

* **The chain must be able to run one plane** (`BankChain::collapse_prefix_of`,
  `crates/rack/src/lib.rs:1919`): the seam-side slots are a suffix, the upstream prefix is
  non-empty, **every** upstream slot supports a one-plane body, and every slot runs on the chain's
  exact lanes. One incompatible slot anywhere upstream makes the prefix `0` and the whole chain
  never collapses.
* **All lanes or nothing** (`BankChain::all_lanes_symmetric`, `crates/rack/src/lib.rs:2060`): a SIMD
  op runs every lane, so a mixed cohort saves nothing. The planner therefore pools tracks by class
  (`CohortPoolClass`, `crates/rack-compiler/src/lib.rs:48-102`): `SOURCE && DESIGNED` at prepare is
  `MonoSymmetricAtPrepare`, anything else `Stereo`. Derived once per compile
  (`crates/graph-compiler/src/compile.rs:433-444`) and handed to both planners.
* **The structural join** (`crates/host-core/src/prepare.rs:1641-1646`, the only production call)
  arms each chain whose every lane's track has `SOURCE` (and symmetric input delay) —
  `Runtime::arm_mono_collapse`, `crates/graph/src/runtime.rs:2304-2316`. An unarmed chain never
  collapses.
* **Per block** (`crates/rack/src/lib.rs:2276-2359`): drain every slot's live queue first, then
  read the witness, then decide `collapse = armed && witness && collapse_channels_agree`. Leaving the
  collapse copies each collapsed stage's left state onto its right channel (`disengage_collapse`,
  `:2438`), so the first dual block is bit-identical to a never-collapsed run. Coming back needs the
  M3 agreement invariant, re-earned only by a proof (`BankStage::channels_agree`).

**So, to the owner's question "what drives it":** all of these, as a conjunction. The *gate* is
the session's source mapping (static, decided at prepare and armed at bind); effect compatibility is
static (bind); parameter symmetry is static at prepare and maintained per block for live writes;
bypass is per block. Nothing inspects samples. Detection is by declaration, not by content.

### 1.2 The stereo-producing stage and after

The seam is fixed at the first seam-side slot: the fader (`FaderBankProcessor::seam_side`,
`crates/builtins-compiler/src/lib.rs:656`) and the 2x2 matrix/pan (`:713`), both declared
`SEAM_SIDE_WITNESS` (`:369`), i.e. their L/R words may differ freely and never gate the collapse.
At the seam the chain copies the resident left plane into the right one
(`crates/rack/src/lib.rs:2400-2401`); from there the fader, matrix, scatter, route fold and master
accumulation run exactly the dual arithmetic in the dual operation order. The matrix is not
simplified: `ll*l + lr*r` is not rewritten as `(ll + lr)*l`, because the two round differently.
A pan move or an L-only fader move therefore never disengages anything.

### 1.3 Which effects keep a collapsed track collapsed

| stage | one-plane body | link modes | notes |
|---|---|---|---|
| input builtins (trim, polarity, HPF, LPF) | yes, `crates/builtins/src/lib.rs:3520` | n/a | only if both channels elide the same sections (`mono_collapse_gate`, `:2152`) |
| parametric EQ | yes, `crates/parametric-eq/src/lib.rs:2720` | dual mono | |
| compressor | yes, `crates/compressor/src/lib.rs:1011` | all | a linked detector is computed on the one plane read twice (`:685-689`), so a link does **not** break the collapse |
| true-peak limiter | yes, `crates/true-peak-limiter/src/lib.rs:3071` | dual mono, maximum | same; the mono fixture keeps its `maximum` link |
| gate/expander, multiband compressor, soft clip, transient shaper | **no** (trait default `false`, `crates/effect-contract/src/lib.rs:1863`) | | banked, but no `process_bank_mono` and no `channel_symmetry` (default `false`, `:1743`), so a track carrying one pools as stereo and its chain never collapses |
| delay | **no bank at all** | dual mono (+ shared cross feedback) | always per node, always dual |
| any effect with a connected sidechain | no | | never banks (`blocks_banking`, `crates/rack/src/lib.rs:72`), per node, dual |
| any effect in a partial cohort | no | | effect banks bind only full groups (#96 F7), so the remainder renders per node, dual |
| fader, matrix/pan | n/a | | seam-side: run dual on the duplicated plane by design |

What *breaks* the collapse for a track: a stereo source mapping, unequal L/R designed words
anywhere upstream (EQ gain, trim, polarity, filter, input delay), a one-channel live write, a live
bypass (reversible), a restore whose sections differ, or one incompatible upstream slot. A linked
detector does not break it; different fader or pan values do not break it.

### 1.4 Banking: pools, partial banks, the half-mono row

`sixty_four_track_console_half_mono` (`tools/console-workload/src/lib.rs:304-323, 753-758`) makes
every odd track stereo again. Since M1 its eight cohorts pool into four all-mono and four
all-stereo (`the_half_mono_cohort_banks_like_a_uniform_one`, `tools/console-workload/tests/chain_shape.rs:417`),
and exactly half collapse (`:1028`). Two costs of pooling are already documented in the tree but
were only ever shown on synthetic rows:

* **Stranded remainders.** Builtin banks pad a partial cohort; effect banks do not, so a pool
  whose size is not a multiple of the width renders its tail's effects per node
  (`a_single_odd_track_strands_both_pools_remainders`, `crates/graph-compiler/src/lib.rs:8307-8331`,
  whose doc says the pooling predicate "is a measurement, and this is the measurement").
* **The forfeited route fold.** Issue #218's fold of routes and the master sum into the chain
  epilogue requires the chains' lanes, in render order, to equal the master reduction order
  (track order); pooling an **interleaved** session breaks that and the whole fold declines
  (`tools/console-workload/tests/chain_shape.rs:343-373`;
  `class_pooling_forfeits_the_route_fold_only_on_an_interleaved_session`,
  `crates/graph-compiler/src/lib.rs:8405`). That test calls contiguous mono/stereo "the shape a
  real session takes". **The dogfood session is interleaved**: its 18 dual-mono tracks sit at
  positions 0, 4, 10, 18-20, 22, 35, 36, 38, 40, 42-44, 53, 58, 60, 66 of 81 (the track order is
  the sorted stable id, which for imported stems is effectively random).

§4 measures both costs on the real session; they dominate the result.

### 1.5 Where dual-mono content enters today

* **Release preparation folds it.** Engine #744 added `session_validator fold-mono`
  (`tools/session-validator/src/lib.rs`), which rewrites a canonical session so a given stereo
  source becomes a one-channel source and every track reading it maps `(0, 0)`, with bit-exact
  PCM proven by A/B render tests (`tools/session-validator/tests/fold_mono.rs`). The CLI's
  `prepare-sparse` (misofm/cli #27, closed) proves whole-source `L == R` while hashing, computes
  the mono identity (`src/sparse-preparation.ts:651-744`) and calls `fold-mono`
  (`:883`). A release prepared that way arrives in the browser already mono.
* **Nothing else does.** The dogfood first-listen session
  (`/home/bl/misofm/agents/stems/mixes/first-listen-v1/render-001/session.json`) declares all 81
  stems as `(0, 1)`, so its 18 dual-mono tracks render dual. The engine's own stem tooling
  (`tools/stem-hasher`) does not report channel identity.
* The app writes effect parameters with `channel: both` (`app/src/lib/mixer/engine/console-writer.ts:215`)
  and taps only `post_matrix` (`session-document.ts:597`), so in today's app a collapsed track
  stays collapsed through normal editing.

---

## 2. Gaps

### 2.1 Defect: a chain after another unit collapses on the wrong premise (wrong audio)

`SOURCE` says the **track input** carries identical planes. It is the right premise for the chain
that gathers the track input, and the wrong one for any later chain of the same track: that
chain's input was produced by earlier stages that are in neither its witness nor the join.
`Runtime::arm_mono_collapse` (`crates/graph/src/runtime.rs:2304-2316`) nevertheless arms every
chain whose lanes' tracks have `SOURCE`, and the M3 comment that "`SOURCE` ... cannot be an
episode" (`crates/rack/src/lib.rs:2342-2350`) only holds for the first chain. A strip splits into
several chains whenever a stage meter or a send taps an internal boundary, or a per-node effect
(delay, sidechained effect, partial-cohort effect) sits between banked racks.

Two probes (in the patch, `crates/host-core/tests/dualmono_probe.rs`, eight mono-mapped tracks,
collapse armed vs forced off, bits compared):

| case | collapsed blocks | result on the base |
|---|---|---|
| asymmetric EQ in `simd1` (+6/-6 dB), symmetric EQ in `simd2`, meter at `PostSimd1` (splits the chain) | 16 | **right channel wrong from sample 0** (`-3.9395` vs `-3.6950`) |
| asymmetric per-node delay (5 ms / 7 ms) between two banked EQs | 32 | **right channel wrong from sample 240** (the left delay's echo copied into the right) |
| same, symmetric delay (control) | 32 | bit-identical |
| asymmetric EQ, one chain (control) | 0 | bit-identical (the chain declines as designed) |

Reachability: not from today's app UI (it writes both-channel parameters and taps only
post-matrix), but from the SDK/agent control surface, any session JSON, and any future send or
meter at an internal tap. **Class A fix** (prototyped, in the patch): arm only chains whose every
lane's first slot is the track's input builtins stage. With it all four probes pass and every
standing gate is unchanged (`chain_shape` 23/23, `host-core` `symmetry_witness`,
`input_liveness_console`, `track_delay`, `rack` `mono_reengage`, `graph` `rt9` all pass). The
prototype adds a flag to `UnitIdentity`; that type's wasm32 padding has room for exactly four flag
bytes (`runtime.rs:2047-2062`), so the real fix must carry the bit elsewhere. Draft:
[issues/01](issues/01-arm-the-collapse-only-on-chains-that-gather-the-track-input.md).

### 2.2 Bit-identical stereo files are not detected

The engine has no content detection. Could the decoder do it? Yes, cheaply, **per file, off the
render thread**, and that is the only granularity worth having:

* The comparison is a byte compare of the two channels' sample words, early-exit on the first
  difference. Measured (native, file in page cache, naive per-frame compare): **3.0 ns per frame**
  over the 18 identical files, 19 ms for a 142-second stem, 0.56 s for all 81 stems (2.3 GB). It
  can ride the pass that already computes the stem's BLAKE3 identity (`tools/stem-hasher`, which
  streams 48 KiB chunks) and emit the mono identity in the same pass, as the CLI already does.
* Per block is not worth it. In the 63 stems that are not dual mono, only **1.06%** of non-silent
  128-frame blocks have `L == R` (3 133 of 294 791). A per-block flag would also have to feed a render-time
  decision, turning `SOURCE` into an episode the M3 invariant excludes on purpose.
* For float sources the test must be bitwise (`-0.0 != +0.0`, NaN payloads), which a byte compare
  of the canonical PCM gives for free.

### 2.3 Incompatible effects

Four banked effects have no one-plane body (gate/expander, multiband, soft clip, transient
shaper). One of them anywhere upstream disables the collapse for the whole chain, including the
builtins before it. Two independent fixes: give each effect its mono trio (draft
[issues/04](issues/04-give-the-gate-expander-a-mono-collapse-body.md) is the template), and
collapse up to the first incompatible slot instead of declining the chain (draft
[issues/05](issues/05-collapse-up-to-the-first-incompatible-upstream-slot.md)).

### 2.4 The planner can make folding a loss (measured, §4.3)

Stranded remainders and the forfeited route fold cost more than the collapse saves on the
realistic session unless the planner changes. Draft
[issues/02](issues/02-keep-the-mono-pool-a-whole-number-of-cohorts.md) (remainder) is class A;
the fold conflict needs a ruling ([issues/07](issues/07-ruling-pooling-versus-the-route-fold.md)).

### 2.5 Smaller items

* **The seam copy** is one block copy per collapsed chain per block. Upper bound measured by
  skipping it: 0.65 µs of 73.5 µs on the mono row at 8 lanes (0.9%), 0.7 µs of 132 µs at 4 lanes
  (0.5%). It can be fused into the fader (read the left plane as the right input), which the
  standing no-unnecessary-copies rule asks for; low value. Draft
  [issues/06](issues/06-fuse-the-seam-copy-into-the-first-seam-side-slot.md).
* **One-sided stems** (18 of 81: one channel entirely digital silence, the other mono content) are
  mono material but not dual mono. Folding them is not exact (an average-linked detector sees
  `|x|/2` on `(x, 0)` but `|x|` on `(x, x)`; the silent channel's `-0.0`/`+0.0` bits are not
  guaranteed). Needs a ruling ([issues/08](issues/08-ruling-one-sided-and-near-mono-stems.md)).
* **Near-mono stems** (3 of 81, side/mid -73 to -103 dB): folding changes audio. Product decision.
* **Silence** dominates the stems: 87% of all 128-frame blocks in the 81 files are digital silence
  on both channels. That is a different, larger lever (silence masks, #940-#942) and out of scope.

---

## 3. Design: dual-mono detection and routing

The router the owner describes already exists as the witness, the pool class, the join and the
chain dispatch. The design is therefore: **detect at import, fold to a real mono source, and fix
and extend the existing router**, not a second router in front of the banks.

### 3.1 Detect at import and fold (engine unchanged at render)

1. **Where.** In the import worker, in the same streamed pass that computes the stem's canonical
   identity: compare each frame's two channel words; keep a running "identical" bit; at end of
   file, if identical, also hold the BLAKE3 identity of channel 0's canonical bytes (the mono
   identity). The engine's reference is `tools/stem-hasher` (draft
   [issues/03](issues/03-report-dual-mono-stems-in-the-stem-hasher.md)); the release CLI already
   does it in TypeScript; a browser import path does the same in its decode worker.
2. **What it reports.** Per file: `channels_identical: bool` and, when true, the mono identity and
   frame count. Nothing per block.
3. **The fold.** Replace the source declaration by a one-channel source with the mono identity and
   remap every track that reads it to `(0, 0)`: the existing `fold-mono` transform, or the same two
   session edits through the control protocol (`SetSource`, `SetTrackSourceAssignment`,
   `crates/protocol/src/model.rs:190-200, 470-490`), which recompile off render.
4. **Why a fold and not an "identical channels" attestation on a stereo source.** The collapse
   gathers only the left plane; a false attestation would silently drop the right channel. A fold
   is exact by construction (proved by #744's A/B tests), cannot lie, and also halves the stem's
   storage, transfer, decode and source-ring bytes.
5. **Exactness.** Integer PCM with identical words converts to identical `f32` bits, so the folded
   session renders the same bits as the stereo one; measured again here on the dogfood session:
   the as-is, folded-collapsed and folded-forced-off arms produce one digest at both widths (§4.3).

### 3.2 Routing: fix and extend the existing planner and join

| slice | change | class |
|---|---|---|
| S1 | Arm the collapse only on chains that gather their track's input (§2.1) | A (bug fix) |
| S2 | Keep the mono pool a whole number of cohorts; the remainder joins the stereo pool and renders dual | A (regroup only) |
| S3 | Decide what to do when pooling interleaved tracks forfeits the route fold | **ruling** (§3.7) |
| S4 | Report dual-mono stems at import (`stem-hasher`), fold with `fold-mono` | A |
| S5 | One-plane bodies for gate/expander, multiband, soft clip, transient shaper (one issue each) | A |
| S6 | Collapse up to the first incompatible upstream slot (seam after the last compatible leading slot) | A |
| S7 | Fuse the seam copy into the first seam-side slot | A |
| S8 | Let a later chain collapse when its predecessor collapsed this block (recovers what S1 gives up on split strips) | A, needs design |

**Collapsed tracks packing more tracks per bank** (for example two mono tracks in one bank's L and
R planes) is **not recommended**: the collapse already skips the right plane, so packing saves no
arithmetic, only per-chain overhead; it would couple two tracks through every linked detector
(`max(|a|, |b|)`), and the seam would have to unpack two tracks into four planes.

### 3.3 Where to re-expand, and what "exactly" means

* **Today** the expansion point is fixed at the fader, the first stage that is *allowed* to make L
  and R differ in a symmetric strip. That is already the right point for the common case.
* **S6** moves it earlier, statically, when an upstream slot has no one-plane body: run the leading
  compatible slots on one plane, copy, run the rest dual. The witness is then taken over the
  collapsed slots only. **A dynamic seam** (move it per block to the first slot whose witness
  declines, so an L-only EQ change re-expands at that EQ and keeps the builtins collapsed) is the
  natural next step: per-slot agreement flags instead of one per chain, and a per-slot
  `desymmetrize` when the seam moves earlier. It is class A by the same induction but is more
  mechanism than the measured gain justifies today, because the app writes both-channel values.
* **Across chains (S8)** the expansion point may lie in an earlier chain; the later chain may then
  collapse only in blocks where its input planes are proven identical, i.e. its predecessor
  collapsed in that block. A per-node op between chains ends the collapse for the rest of the strip.
* **"Exactly" means**: every output sample, every meter reading, every send, every state snapshot
  and every per-channel report is bit-identical (`to_bits`, so `-0.0` and NaN payloads count) to
  the dual render of the same session, on every block, including the block where the collapse
  stops (the disengage copy) and every block after it. The duplication is a bit copy; nothing
  downstream of the seam may be algebraically simplified on the strength of `l == r`.

### 3.4 Automation and parameter changes mid-song

* **Both-channel writes** (what the app sends) keep the collapse: both ramps get one target at one
  boundary and advance through identical values.
* **An L-only or R-only write upstream** (for example an L-only EQ change) clears `LIVE` at the
  drain, before the witness is read, so the block it lands on renders dual, after the disengage copy
  made the right state equal to the counterfactual dual state. It stays dual until a proof shows
  the two channels' state is bit-equal again; re-equal *words* alone never re-engage
  (`re_equal_designed_words_after_a_one_channel_retarget_never_re_engage`, `chain_shape.rs:1493`).
* **A pan, matrix, fader or mute move** is seam-side and never disengages.
* **A live bypass** disengages while on and re-engages when lifted (`chain_shape.rs:1392`).
* **Session automation spans** have no admission point yet; the `LiveConsoleRecord` trait
  (`symmetry.rs:292`) makes the future span type declare its effect on the witness or fail to
  compile.

### 3.5 Sends, meters, PDC

* Taps at or after the fader read the post-seam planes, which are the dual planes bit for bit.
* A tap at an internal boundary splits the chain; today that is exactly the §2.1 defect. After S1
  the second chain renders dual (correct, slower); S8 would recover it.
* PDC is unaffected: the collapse changes no latency and no delay line. Asymmetric input delay is a
  prepare-time decline (`track_input_delay_symmetric`); effect lookahead lines are per-channel
  state that the disengage copy carries.

### 3.6 Already done, class A, needs a ruling

* **Done:** the collapse itself (M0-M3, #210 phases 2-3), per-track witness, planner pooling, the
  join, disengage/re-engage, the release-path fold (#744 + cli #27).
* **Class A:** S1, S2, S4, S5, S6, S7, S8.
* **Rulings:** S3 (pooling vs the route fold, §3.7), one-sided and near-mono stems (§2.5), and the
  already-open question in `docs/rulings/effect-floor-accounting.md` "The mono rows" of whether a
  collapsed row's floor halves.

### 3.7 The ruling S3 needs

The master sum must run in track order to keep its bits, and the route fold needs the chains to
render in that order; pooling interleaved mono tracks breaks both at once. The options:

1. **Keep today's rule** (pool by class, lose the fold on interleaved sessions): with S2 this is
   still a net win whenever a mono cohort carries a real strip (-5.3% at 8 lanes, -7.6% at 4 on the
   dogfood strip) but a loss or a wash on a builtins-only mix (+6% at 8 lanes, -0.4% at 4).
2. **Class A policy:** pool by class only when the mono cohorts' saving beats the fold (a simple
   rule on upstream bank slots per mono cohort; builtins-only cohorts would not pool).
3. **Order the tracks at import** so mono-sourced tracks are contiguous in stable-id order. Keeps
   the fold and the saving (the ~8% ceiling), but changes the master's summation order relative to
   a session imported without it (last-bit differences in the mix), so it needs an owner decision.

---

## 4. Measurements

Method: a research example driving the production compile, prepare, bind and join
(`SessionRuntime::research_build` and `examples/dualmono_research.rs` in the patch). One invocation
per width, one warmup (500 blocks per arm), two measured rounds of 3 000 observations; arms
alternate observation by observation with rotating order; p50 of per-block `render` time in µs.
Every run under the shared timing lock, pinned `taskset -c 31`, AMD EPYC 7313P, rustc 1.97.1,
release profile (fat LTO), host shared with other agents (load about 3.7): read the relative
numbers. The **8-lane** leg is the production native build. The **4-lane** leg is a native build
with a research cfg that makes `Backend::current()` four lanes (SSE), because in an 8-lane build the
effect factories refuse four-lane banks and render every effect per node, which is not what the
browser does. For the wasm target itself, the sealed mono2 capture (`artifacts/mono2/README.md`)
recorded the mono row at -38.4% under `wasm_simd128`. Every "same audio" arm printed the same
digest; raw records are the `timing-*.tsv` files beside this document.

### 4.1 The dogfood stems (`/home/bl/misofm/agents/stems`, read only)

All 81 are 2-channel 24-bit 44.1 kHz WAV. Scan: [`stem-scan.tsv`](stem-scan.tsv), tool source
[`stem-scan.rs.txt`](stem-scan.rs.txt).

| class | files | share of frames | share of non-silent blocks |
|---|---:|---:|---:|
| bit-identical dual mono (`L == R` every frame) | **18** | 17.3% | 18.5% |
| one channel entirely silent (mono content, hard-sided) | 18 | 29.2% | 31.4% |
| near mono (side/mid below -60 dB, not identical) | 3 | | |
| stereo | 42 | | |

The 18 dual-mono stems: BASS_2, BASS_3, BV_S_3, BV_S_9, BV_S_13, BV_S_14, BV_S_16, BV_S_17,
BV_S_19, BV_S_29, BV_S_37, DRUMS_10, DRUMS_14, FX_7, FX_9, GUITAR_1, INTRO FX_1, LEAD VOX_1. No
source file is natively mono. 87.2% of all 128-frame blocks are digital silence on both channels.

### 4.2 The existing mono rows (64 tracks, whole intended strip)

| row | 8 lanes, µs | vs console | 4 lanes, µs | vs console |
|---|---:|---:|---:|---:|
| `console` | 129.2 | | 234.7 | |
| `console_mono` (collapse) | **73.5** | **-43.1%** | **132.2** | **-43.7%** |
| `console_mono_dual` (same fixture, forced off) | 129.9 | +0.5% | 235.3 | +0.3% |
| `console_half_mono` | 104.2 | -19.3% | 185.9 | -20.8% |
| mono, seam copy skipped (timing bound, wrong bits) | 72.9 | | 131.5 | |

Round 2 agrees within 0.2%. Shapes: `[8, 48]` at 8 lanes and `[16, 96]` at 4 lanes (one chain per
cohort). The half-mono row sits a little above half the mono saving because its interleaving also
forfeits the route fold. The recorded `artifacts/steps/after-945` capture reads 79.2 / 135.6 µs
for the same pair.

### 4.3 The realistic session: dogfood first-listen, 81 tracks, 18 dual mono

Two strips: **as mixed** (input builtins with HPF, faders, pan; no effects) and **with a mixing
strip** (the standing EQ + compressor on `simd1` and true-peak limiter on `simd2`, channel-symmetric
parameters as the app writes them, on every track). Input is the benchmark's frozen tone, fed
identically to both planes of the 18 dual-mono tracks; so this measures active-audio cost, not the
stems' silence. Arms:

* `as is`: the 18 declared stereo `(0, 1)`, identical planes: what renders today.
* `folded`: the 18 mapped `(0, 0)`, today's planner (what `fold-mono` produces).
* `folded + S2`: the same with the remainder-aware pooling prototype.

p50 µs (round 1 / round 2 of the main run; digest identical across all three arms in each group):

| session | width | as is | folded (today) | folded + S2 |
|---|---|---:|---:|---:|
| as mixed | 8 | 31.0 / 31.1 | 33.5 / 33.6 (**+8%**) | 32.9 / 33.0 (+6%) |
| as mixed | 4 | 48.6 / 48.6 | 47.7 / 47.7 (-2%) | 48.4 / 48.4 (-0.4%) |
| mixing strip | 8 | 169.2 / 169.3 | 224.5 / 224.5 (**+33%**) | **160.2 / 160.3 (-5.3%)** |
| mixing strip | 4 | 297.4 / 297.5 | 300.8 / 300.9 (+1%) | **274.7 / 274.7 (-7.6%)** |

Shapes: the mixing strip is `[12, 63]` as is and with S2, `[13, 60]` folded today at 8 lanes
(`[22, 123]` / `[23, 120]` at 4 lanes): the extra partial cohorts render their effects per node.

### 4.4 Where the planner cost comes from (diagnosis run)

A second invocation added arms that fold but never arm the collapse, separating the regroup from
the saving (same method; within-run numbers):

| mixing strip | 8 lanes | 4 lanes |
|---|---:|---:|
| as is | 168.9 | 294.9 |
| folded, today's pooling, collapse never armed | 237.4 (**+68.5 stranded + fold**) | 324.6 (+29.7) |
| folded + S2, never armed (regroup cost = lost route fold) | 172.7 (+3.9) | 297.6 (+2.7) |
| folded + S2, collapse (saving on 16 tracks) | 159.3 (-13.4) | 274.5 (-23.1) |
| **estimate with the fold kept (as is minus the saving)** | **155.4 (-8.0%)** | **271.8 (-7.8%)** |

| as mixed (builtins only) | 8 lanes | 4 lanes |
|---|---:|---:|
| as is | 30.2 | 48.3 |
| folded + S2, never armed | 33.5 (+3.3) | 50.6 (+2.3) |
| folded + S2, collapse | 33.2 (-0.3 to -0.7) | 48.5 (-2.0) |
| estimate with the fold kept | ~29.7 (-1.7%) | ~46.3 (-4.2%) |

Reading: the collapse saves about **0.84 µs per mono track at 8 lanes and 1.44 µs at 4 lanes** on
a real strip (about 40% of that track's cost), and 0.02 to 0.13 µs on a builtins-only strip.
Stranding costs about 65 µs at 8 lanes (27 at 4), and the lost fold 2.3 to 4 µs whatever the
strip. The ceiling for these 18 stems is about 8% on a mixing strip; if the one-sided stems were
ruled foldable too (36 of 81 tracks), roughly 18%.

One nuance for S2: on the builtins-only mix at 4 lanes today's pooling beats S2 (47.7 vs 48.4 µs),
because builtin banks pad a partial cohort and so collapse it, while S2 demotes those two tracks to
dual. The remainder rule should apply only to cohorts whose program has effect slots.

### 4.5 Detection cost

3.0 ns per frame natively for a full compare of an identical file (the worst case); non-identical
files exit at the first differing frame, which is often after a long silent intro. All 81 files:
0.56 s.

---

## 5. Sliced plan

In order; each is a separate, closable issue (drafts in [`issues/`](issues/)):

1. **S1** [Arm the collapse only on chains that gather the track input](issues/01-arm-the-collapse-only-on-chains-that-gather-the-track-input.md)
   (correctness; first).
2. **S2** [Keep the mono pool a whole number of cohorts](issues/02-keep-the-mono-pool-a-whole-number-of-cohorts.md)
   (turns the measured +33% into -5.3% at 8 lanes; +1% into -7.6% at 4 lanes).
3. **S3** [Ruling: pooling versus the route fold](issues/07-ruling-pooling-versus-the-route-fold.md),
   then its implementation issue.
4. **S4** [Report dual-mono stems in the stem hasher](issues/03-report-dual-mono-stems-in-the-stem-hasher.md)
   (engine-side detection for every path that does not go through the release CLI).
5. **S5** [Give the gate/expander a mono-collapse body](issues/04-give-the-gate-expander-a-mono-collapse-body.md),
   then the same for multiband, soft clip and transient shaper.
6. **S6** [Collapse up to the first incompatible upstream slot](issues/05-collapse-up-to-the-first-incompatible-upstream-slot.md).
7. **S7** [Fuse the seam copy into the first seam-side slot](issues/06-fuse-the-seam-copy-into-the-first-seam-side-slot.md).
8. **S8** and the dynamic seam: design issue when a measured session wants it.
9. [Ruling: one-sided and near-mono stems](issues/08-ruling-one-sided-and-near-mono-stems.md).

## 6. Decisions for the owner

1. **Pooling versus the route fold** (§3.7): accept the fold loss, let the planner decide by a
   simple rule, or order mono tracks contiguously at import (last-bit change in the master sum).
2. **One-sided stems** (18 of 81 dogfood stems): treat "mono content on one side, silence on the
   other" as a mono source panned hard to that side? Not bit-exact with some detector links, so it
   is a product choice, not an optimisation.
3. **Near-mono stems**: offer a "treat as mono" action? It changes audio.
4. Whether a collapsed row's efficiency floor halves (already open in the floor ruling).

## Appendix: reproduce

```
git apply docs/handoffs/dual-mono-2026-09-27/dual-mono-prototypes.patch
# probes: green with the patch (S1 arm change included);
# MISO_ENGINE_RESEARCH_ARM_ANY_CHAIN=1 restores the shipped arming and turns the two asymmetric probes red
cargo test --locked -p host-core --test dualmono_probe -- --test-threads 1
MISO_ENGINE_RESEARCH_ARM_ANY_CHAIN=1 cargo test --locked -p host-core --test dualmono_probe -- --test-threads 1
# 8-lane bench
cargo build --release --locked -p console-workload --example dualmono_research
# 4-lane bench (research cfg; RUSTFLAGS replaces the config's target flags, so restate them)
RUSTFLAGS="-C target-feature=+avx2,+fma --cfg miso_native_simd4" \
  cargo build --release --locked -p console-workload --example dualmono_research
# run (identical-sha list: blake3/sha256 identities of the 18 dual-mono stems, one per line)
flock -w 7200 <timing.lock> taskset -c 31 target/release/examples/dualmono_research <list>
# DUALMONO_PREFLIGHT=1 renders 8 observations (harness check); DUALMONO_UNARMED=1 is the §4.4 run
```

The first-listen session predates the BLAKE3 stem identity; the driver relabels its `sha256:`
identities as `blake3:` (they are only labels here). The patch's prototypes: the probe test, the
research build and example, `MISO_ENGINE_RESEARCH_SKIP_SEAM_COPY` (rack, timing bound only),
`MISO_ENGINE_RESEARCH_MONO_REMAINDER` (S2, global rather than per strip program), the S1 arm change
(`MISO_ENGINE_RESEARCH_ARM_ANY_CHAIN` restores the shipped behaviour) and the `miso_native_simd4` cfg.
