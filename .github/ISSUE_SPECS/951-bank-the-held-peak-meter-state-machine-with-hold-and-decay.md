# Bank the held-peak meter state machine with hold and decay

## Product outcome

#950 banks the full meter pass for meters with hold 0 and decay off, the only configuration any host binds today. A meter with a peak hold time or a dB-per-second decay still runs its held-peak state machine one lane at a time. Under the owner's rule ("We shouldn't leave scalar arithmetic where vector arithmetic is possible"), that is a scheduled defect, not a permanent path: the held-peak update is a per-lane, sample-serial state machine (hold countdown, then decay) that can run across a bank's lanes exactly like #950's seeded energy sum, with each meter's own state as the seed.

## Smallest closable slice

To be briefed after #950 lands, reusing its seed-and-accept design: a lane-parallel, sample-serial held-peak pass for hold > 0 and decay on, bit-identical to the scalar state machine on every published word (`held_peak`, and every word it feeds), with the scalar path as the oracle and a wasm differential.

## Dependencies

#949, #950.

Filed from #950's coordinator ruling 2.

## Brief — 2026-10-27 (Sol implementation, attempt 1)

The owner selected three Sol implementation streams and CI-conscious batching. This stream owns
only #951; no local build/test/formatter runs before root approves this brief. Every subsequent
check uses the shared serial wrapper at
`/home/bl/misofm/engine-work-evidence/easy3-20261027/run-serial.sh`, in this worktree,
with the wrapper's own target directory, jobs=2 and test threads=1. No push, PR or GitHub mutation
is authorized in this stream; root synchronizes the issue and delivers the batch.

Smallest closable product slice: bank every held-peak sample update for a final resident meter
with a positive hold and/or enabled decay, using each lane's own post-preamble state as the seed.
Keep the existing hold=0/decay=0 maximum merge and existing energy seed search unchanged. A separate
held seed query and held pass let a held-only meter seed without stealing another meter's energy
seed. The first compatible held state per lane is computed once; every consumer verifies its own
state and configuration before accepting and otherwise uses its correct scalar fallback. Blocks
crossing a window still take the existing scalar split; discontinuities seed from cleared state.

The scalar recurrence remains the definition: sanitize the sample, take its magnitude; if it is
at least the held value, replace held and reload the u32 hold counter; otherwise decrement a
positive counter; otherwise multiply by the prepared per-sample decay and flush non-normal
results to +0 when decay is enabled. Off means an exact identity multiplier. The counter is the
full u32 domain: two f32 vectors carry its high/low 16-bit digits, with exact borrow/decrement;
casting the whole counter to f32 would lose countdown frames above 2^24. Every lane has independent
state, hold length and decay. No cross-lane reductions, allocations, new latency, signal change,
host/protocol change, effect change or lane-trait expansion is needed. Snapshot/cumulative count
accounting remains the owning meter's existing commit. Meter state/config/result metadata is inline.

Exact planned paths:

- this numbered spec;
- `crates/lane/src/kernels/builtins.rs` (held state/config vocabulary and bank kernel);
- `crates/builtins/src/lib.rs` (held seed, validated accept, recurrence state commit);
- `crates/graph/src/lib.rs` and `crates/graph/src/runtime.rs` (necessary production seed/result
  transport and resident pass wiring, narrowly within the existing meter mechanism);
- `crates/builtins-compiler/src/lib.rs` (observer wiring and existing integration test);
- `crates/builtins/tests/meter.rs` (extend the existing owning scalar/banked differential);
- `tools/wasm-gate-corpus/src/lib.rs` (extend existing meter differential with independent held
  oracle, including high counter limbs and per-lane parameters);
- `tools/wasm-gate-guest/src/lib.rs` and `scripts/run-wasm-gates.sh` only if needed to extend the
  existing lowering probe/check to the held recurrence. No new report field or pinned digest.

Objective gates: existing meter differential must publish bit-identical full snapshots for all
metric selections, hold-only/decay-only/combined ballistics, window splits, resets/discontinuities,
negative zero/non-finites and lifetime counters. Update its existing commit-count assertions so
held configurations must actually bank; wrong held state/config seeds must decline. The existing
compiler integration test must prove production bank commits with positive hold/decay at padded
track counts and no render allocations/frees. Extend the existing cross-target kernel differential
for per-lane state/config and countdown boundaries (including u32::MAX), check wasm simd128 and
scalar-oracle identity, and inspect vector lowering using the existing guest probe. Red evidence
must show that the pre-change implementation fails the new bank-commit assertion and that a held
recurrence mutation fails the owner differential; restore before the green checkpoint. Reuse
existing native allocation, source sanitization and resource ownership gates instead of adding
redundant tests. Run focused owning tests, proportional lint/format, and wasm differential under
the shared serial wrapper. Broad merged-tree gates and independent Astra review belong to root.

No benchmark or listening exercise is needed: this is a bit-preserving meter-only arithmetic
change, not audio DSP or a claimed performance result. Extended target matrices or new harness
work are qualification successors if the representative existing gates cannot run locally.
After the coherent green tranche, commit exact paths locally and pause for root's status/commit
audit before further implementation. Maximum five attempts, each with one adversarial verdict.

## Attempt 1 — native product checkpoint (2026-10-27)

Implemented the lane-parallel held recurrence with exact two-digit u32 countdown, independent
energy/held seed searches, both-channel bitwise state/configuration checks and inline results.
No heap field was added to an accumulator/observer; temporary bank metadata is stack-only.
The hold=0/decay=0 maximum merge remains the existing path.

Evidence directory: `/home/bl/misofm/engine-work-evidence/easy3-20261027/951/`.
`red-prechange.log` records the amended production owner failing the baseline: Simd4, hold=7,
decay=0 made zero banked commits when one was required. `production-green.log` passes the same
owner with all four hold/decay combinations, forced/separate mono, nine tracks with padded banks,
bit-identical PCM/snapshots and exactly zero render-thread allocations and frees.
`meter-green.log` passes all 12 meter tests: the existing full differential compares 19,504,032
snapshots bitwise and records 423,288 commits over all selections, ballistics, window splits,
resets and discontinuities; both channels reject each independently corrupted held seed word.
The existing multi-binding owner now proves a held-only binding does not steal the ALL meter's
energy seed and both seeded meters commit every contained block.

Test value (production owner rewrite): a final resident positive-hold/decay meter that silently
runs scalar, loses state while banking or allocates/frees during render now fails, where the
previous row bound only hold=0/decay=0.
Test value (full meter differential rewrite): accepting a mismatched held state/configuration
word on either channel, losing per-sample countdown/decay order or failing to bank advanced held
metric selections now fails; the old owner explicitly required those selections to stay scalar.
Test value (multi-binding owner rewrite): allowing a held-only meter's unused energy to stop the
seed search before an ALL meter now fails; the previous row covered only SAMPLE_PEAK first.

The existing wasm corpus function has been extended locally with an independent held/countdown
oracle, but that extension is not yet compiled or executed at this checkpoint. Remaining gates:
compile/run the extended corpus natively and on wasm simd128, reuse/extend the existing lowering
probe as needed, capture one held-recurrence mutation red then restore, run proportional formatting,
lints and graph/compiler owning regressions. Root's independent Astra review and merged-tree gates
remain outstanding. This checkpoint claims native product evidence only, not issue completion.

## Qualification decisions — attempt 1

Root approved the existing guest meter probe and its existing script census as additional narrow
paths. The probe now executes the real held recurrence with dynamic full-u32 countdowns and folds
both final held values and counters; its census requires packed f32 multiply/subtract and refuses
scalar f32 arithmetic within that owned probe. Integer boundary conversions remain permitted.
The existing `meter_block_mismatches` field owns the added held oracle; no report schema, pins,
exported symbol names or numeric boundary identity changes.

The earlier full-render allocation measurement linked the audited global allocator into the entire
compiler unit-test binary. Running the existing mixed custom Capture observer alongside it then
aborted, because that fixture grows a Vec in render. Root ruled to isolate this claim rather than
expand unrelated Capture cleanup: remove allocator linkage from the compiler unit owner; add one
positive-hold/decay full-render case to the already isolated
`crates/builtins-compiler/tests/lane_swap_alloc.rs`, using `bench_support::alloc` thread counters,
preallocated output, warmed statics and a non-vacuous bank-commit witness. The provisional
`allocation_tracker.rs` edit was reverted; that custom allocator owner is unchanged. Temporary
kernel-only allocation assertions were removed so the full render entry owns this claim once.

The final configured unit and allocation fixtures select the last track of nine (one padded tail
bank on both vector widths); the compatibility fixture still selects track zero. The earlier
`production-green.log` and initial compiler regression run observed track zero. Final tail-fixture
parity and allocation results remain pending until recorded below.

`native-corpus.log` passes the established release native G5 runner: 157 cases, 391 comparisons,
zero mismatches, including the extended held/countdown oracle. The completed native correctness
build overlapped an external wt-2166 build; no timing claim is drawn from it. Later offline
compilers are coordinated with the fresh-review supervisor and paused only on our side while
foreign validation runs; no running test is paused.

Test value (new isolated full-render allocation row): allocating or freeing in the graph's advanced
held seed/pass/accept branch now fails under the audited allocator, where the existing lane-swap
allocation owner never bound a held meter and the original compiler unit row used only hold=0/off.
Test value (existing cross-target meter differential extension): losing a high-digit borrow,
rounding a countdown above 2^24 or coupling per-lane hold/decay configuration now fails against an
independent full-u32 scalar recurrence, including the countdown word that snapshots do not expose.

## Qualification relocation checkpoint — 2026-10-27

Root moved remaining qualification to required final PR CI plus one dispatch-only evidence run,
because sustained unrelated local builds/tests prevent uncontended validation. This is the same
implementation attempt, not a new attempt or a lowered gate. All local queued/running #951 jobs
were canceled through verified own-PID ancestry/cwd checks; no foreign process was signaled. Fresh
Astra's `fresh-astra/cancel-local.log` preserves cancellation evidence. The active mutation's
restoration path ran: the lane kernel matches both its exact pre-mutation backup and commit
`fa560b658`; no compiler/linker/test or queued guest process from this stream remains.

The interrupted no-high-borrow mutant never executed. Its partial compiler output is preserved as
`mutation-no-borrow-interrupted.log`; it supplies **no mutation RED evidence**. The queued pinned
Wasm guest build never started. These gates are PENDING remote:

- final tail/asymmetric-channel production parity and bank-commit unit row;
- the isolated positive-held full-render allocation row in `lane_swap_alloc.rs`;
- the pinned nightly simd128 guest's existing differential and extended vector census;
- the directed high-digit-borrow mutation RED and restored native GREEN;
- final all-target Clippy, proportional policy checks and required merged qualification CI.

The earlier completed native product/owner evidence remains valid for its recorded configurations:
12 meter tests (19,504,032 full snapshot comparisons), both earlier resident compiler regressions,
the graph full-meter frame-count ceiling, release native G5 (157 cases/391 comparisons, all
mismatch fields zero), and the earlier all-target Clippy pass before the final allocation/tail
fixture edits. Formatter-only `cargo fmt --all` passed through the shared serial wrapper after
cancellation. No final tail/allocation/Wasm/mutation result is claimed at this checkpoint.

Test value (final production owner): wrong lane selection in a padded held bank or swapped L/R
held-state metadata now fails, where the earlier native row observed a full first bank with
symmetric channels. The combined hold/decay case keeps L/R distinct, and the other cases retain
collapsed/forced-dual execution coverage.

Exact pending command/mutation handoff is preserved outside the worktree as
`/home/bl/misofm/engine-work-evidence/easy3-20261027/951/remote-pending.sh` (not executed locally).
The directed mutation replaces only
`let next_high = high.sub(L::select(borrow, one, zero));` with `let next_high = high;`, runs the
existing release native G5 runner, requires its `meter_block_mismatches` to be positive, restores
the exact source, and requires the same runner GREEN. The remote owner must retain the emitted
JSON and extended census, update this decision record, and obtain the independent Astra verdict
before the issue can pass or close.

## Fresh independent adversarial verdict — conditional PASS (2026-10-27)

Astra independently reviewed the lane recurrence, exact two-limb u32 borrow/reload, independent
energy/held seed searches, post-preamble both-channel state/configuration acceptance, bounded
window fallback, production graph transport and final padded-tail/asymmetric fixture. No new
allocation, lock, I/O, syscall, cross-lane arithmetic or unbounded render operation was found.
The final allocation row uses the existing `bench_support::alloc` thread-scoped owner, warms the
actual positive-hold/decay path, measures full `plan.render`, witnesses a real bank commit and
checks the published record outside the measured span.

[Evidence run 40471281487](https://github.com/misofm/engine/actions/runs/40471281487), source
`21c38051cba9abeb706fcaa1622485cb267da702`, proof
`7f14f6fbf88c1ecb438976d63a1599e6dddf6d50`, completed the previously pending targeted gates:

- Both final resident-meter tests passed, including the padded observed tail and distinct L/R
  combined hold/decay case; both allocation-owner tests passed, with zero allocations/frees and
  the actual held bank-commit assertion.
- Native G5 passed 157 cases/391 comparisons with every mismatch field zero. Omitting the high
  countdown digit's borrow then executed and failed with **1,409 meter mismatches**, not a compile
  failure. Exact source restoration and a clean-diff guard preceded the same native GREEN result.
- The pinned nightly/build-std SIMD128 guest passed 157 cases/274 comparisons with every mismatch
  field zero. The existing owned meter probe contains one packed f32 multiply and subtraction,
  while its scalar f32 arithmetic census is zero.

Test value, by changed owner:

- `a_banked_block_commit_publishes_the_scalar_meters_snapshots` now rejects a wrong hold/decay
  recurrence or either channel's mismatched held state/configuration seed, which previously
  always declined to scalar.
- `a_peak_meter_bound_first_leaves_the_seed_to_the_all_meter_behind_it` now rejects suppressing
  a held-only seed behind another observer or conflating it with the independent energy seed.
- `resident_meter_accepted_plan_avoids_planar_acquisition_and_matches_decline` rejects missing
  production held transport, wrong padded-lane selection or swapped L/R state, despite a correct
  standalone kernel.
- `a_banked_held_meter_render_allocates_nothing` rejects allocation/free in the real held
  seed/pass/accept render branch that the previous zero-ballistics and lane-swap cases never ran.
- The existing cross-target meter differential's added remaining-u32 comparison rejects lost
  high-digit borrow/reload above f32's integer range before a long hold can affect a snapshot;
  its existing compiled-code probe rejects scalar decay/countdown lowering on SIMD128.

**Conditional source-review PASS, attempt 1.** All #951 critical source and test paths are unchanged
between the proved source and reduced tree `da4f72f60d29bade17cd86b7a299b3d2f8734b9f`. The later
#988 timing failure is independent. Final workspace Clippy/policy checks and required regular
qualification on the reduced product tree remain mandatory before delivery/closure; no pending
CI result is represented as passed. Raw artifact and independent judgment are retained under
`engine-work-evidence/easy3-20261027/fresh-astra/remote/` outside Git.

## Root source co-review and final delivery gate (2026-10-27)

PASS for held-meter banking after fresh independent review `0a29fc46fddd4aaf126897b09c4e09ec1e18fcfe`. Per-lane exact u32 limbs, post-preamble state/configuration matching and independent energy/held selection preserve the scalar state machine. The final tail fixture and audited full-render owner exercise actual padded-bank commits. Executed high-borrow RED/restored GREEN and the actual nightly packed lowering defend the arithmetic and target path. No render allocation or resource/state-format expansion is introduced.

The limiter screen is excluded after its real performance failure. Its removal leaves this slice’s reviewed source unchanged. Final normal required qualification on the actual combined product tree is mandatory before merge, issue closure and completed-worktree cleanup. This record does not claim that pending workflow passed.

## Required-CI attempt 1 FAIL and bounded attempt 2 scope (2026-10-27)

[PR #2169 required run 40475574564](https://github.com/misofm/engine/actions/runs/40475574564) rejected attempt 1 on integration/portability gates that the earlier targeted proof did not establish. Source-level conditional approvals are not a final PASS:

- Cross-target job `122093153675`: actual post-LTO iOS staticlib assembly contains one new `_memset_pattern16` call, attributed by the existing owner to `graph::runtime::bank_meter_pass::plane<wide::f32x4>`. A lazily-bound libSystem call in render is forbidden; no defect-list allowance is authorized. The scratch decay array’s nonzero default is unnecessary because every loaded lane is overwritten from its seed. Attempt 2 uses zero initialization and retains the exact recurrence; removal must be proved by the actual corrected iOS assembly gate, not inferred from source.
- Artifact job `122094528084`: the actual named Wasm twin has a newly outlined bank-meter `plane<Simd4>` with five vector operations and zero scalar operations. It is real render code, so it belongs in `RENDER_KERNELS`, not `CONTROL_THREAD_EXEMPT`. The narrow reviewed normalized key names only this exact function; all existing arithmetic, presence, uniqueness, scalarization and roster rules stay in force. Corrected artifact qualification must confirm its final shape.
- Workspace debug job `122093153733`: two existing graph-compiler owners still require held ballistics never to bank. The fallback owner’s obsolete ballistics row and the mixed-period/metrics owner’s count derivation must reflect the newly valid held seed. Real no-commit, window and metric fallbacks, bitwise publications and non-energy guards remain. Expected counts are derived from eligible fixture rows/windows, never copied from the observed 87.

Root authorizes one coherent attempt 2 correction, bounded to the lane scratch initializer, the existing graph-compiler test module, `scripts/check-web-audioworklet-callgraph.py`’s exact render-kernel row, and this decision record. No algorithm, wire/ABI/state format, budget, corpus pin, libc exception, scalarization allowance, generic harness or additional test owner is added. The original Sol author supplies the source/test fix; root owns the precise roster bookkeeping, and fresh Astra independently verifies the complete revision before one corrected push. No local Cargo/test/benchmark workload is used while the host is busy; actual corrected required CI remains mandatory.

Attempt 2 source checkpoints: `a7f7e764d01ff9e413c80ea5be6ceb449dfe8ff7` changes only the decay scratch’s initial value to zero and documents the overwrite-before-load invariant. Every lane loaded by `L::load` is unconditionally assigned `seed.decay`; the unread suffix never participates. `745bef4603248984844e8b3d74308ea86d919e39` changes only existing graph-compiler test owners: removes the obsolete hold-never-banks row, and sums the existing window helper for every full-pass consumer except the separate SAMPLE_PEAK merge row. This includes held row 5; rows 3/7 contribute zero because their windows are shorter than a block. Snapshot/PCM, decline, non-energy, window and merge guards remain. The expected total is derived, not copied from a failing observation.

Root checkpoint `0c44fa4a3f2f2d9f018d5f10c4c94d38c63c3839` classifies the actual outlined meter plane with its exact normalized render key. It adds no control-thread exemption or arithmetic allowance. The existing callgraph analyzer self-test passed through the serial wrapper; final actual iOS assembly, named artifact, existing graph-compiler owners and full corrected qualification remain pending. Formatting/diff checks passed; no local Cargo/test binary or benchmark was launched for the revision. Fresh Astra independently reviews this bounded correction before its single corrected push.
