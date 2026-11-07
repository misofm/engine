# Control plane: a provider epoch's phase-only state is cleared by hand beside the container that names its phase

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item PR-6).

## Mission
Give `ProviderEpoch` (`crates/control-plane/src/control.rs`) an explicit phase that owns the
state valid only in that phase, so a promotion or a parking moves that state by construction and
nothing is cleared by hand. A running epoch must not be able to hold a candidate's spectrum
captures or a declared discontinuity, a pending epoch must not be able to keep a committed base or
wait on a pairing, and a parked epoch must not be able to keep a base.

## Verified defect (base `24ea03525`)
`crates/control-plane/src/control.rs`:
- An epoch's phase is the container that holds it: `SessionState::pending_providers` (pending),
  `providers` (running), `retired_providers` (parked) (1274-1276). Its phase-only state sits in
  loose fields beside the others (41-142): `kept: Option<KeptBase>` (126), `parked:
  Option<LiveEpochResources>` (133), `discontinuity: bool` (137), `spectrum: EpochSpectrum`
  (140, an enum of `None | Candidate | Displaced`, `crates/control-plane/src/spectrum.rs` 101)
  and `observations_pending: Option<u64>` (92).
- The one promotion site, `promote_pending_provider` (4000-4093), moves them by hand:
  `core::mem::take(&mut next.spectrum)` (4040), `next.observations_pending = Some(..)` (4074),
  then on the displaced epoch `previous.spectrum = displaced` (4086), `previous.kept = None`
  (4089) and `previous.parked = row` (4090). The candidate's `discontinuity` (6020) and
  `spectrum` (6028) are assigned after construction. A forgotten line at any of these sites
  leaves a field valid in the wrong phase, with nothing to say so.
- Audit corrections:
  - `meter_tables` (66) is not phase-only. A pending epoch holds its own tables, and a promoted
    one keeps them until the reclaim of the plan it displaced hands them to the delivery
    (`adopt_meters`, 3933; in the atomic-promotion path that is a later service step). The
    delivery's displaced tables then come back to the parked epoch. Every phase takes part in
    the exchange, so the slot stays a field. Its doc ("The running epoch holds empty tables",
    65) was wrong in that window and is corrected.
  - `observations_pending` is valid in two phases, running and parked
    (`settle_observation_pairings`, 3889-3917, settles parked successors too), never pending.
  - `kept` is valid only on the running epoch. `kept_holder` (7581) can name a pending epoch
    when two are pending, but two are never pending: a candidate is pushed only at 6634, after
    either the donor's pop (6448) or the `debug_assert_eq!(pending_providers.len(), 0)` of a
    rebuild that supersedes nothing (6523), which `withdraw_pending_candidate` (5601) makes
    true by promoting every taken candidate first.

**Reachability: no stale phase-only field is reachable today.** Each site that moves an epoch
between containers:
- *Promotion* (`promote_pending_provider`, the only `mem::replace` of `providers`, 4076). It
  takes the candidate's spectrum, so the running epoch's is `None`; it sets `kept = None` and
  `parked = row` on the displaced epoch; `next` never held `kept` or `parked`, because only the
  running epoch is given a base (6512) and only this site sets `parked`.
- *Supersession* (6448). The withdrawn donor is popped and dropped with its pending state, and
  the successor is pushed in the same commit.
- *Reclaim* (3837). The parked epoch drops whole.
- `discontinuity` stays `true` on a promoted epoch. The flag is read only through
  `pending_providers.last()` (5688), so the stale value is never read.
- `spectrum` is `Candidate` on every pushed epoch (6028, unconditional) and `Displaced` on every
  parked one (4086), so `EpochSpectrum::None` is reachable only on the running epoch.

So the defect is structural (severity M by the tracker's legend: a single forgotten site causes a
silent bug), and there is no behavioural regression test that is red on base.

## Decision / model
- `ProviderEpoch` gains a private `phase: EpochPhase`, and loses `kept`, `parked`,
  `discontinuity`, `spectrum` and `observations_pending`:
  - `Pending { discontinuity, spectrum: CandidateSpectrum }`;
  - `Running { kept: Option<KeptBase>, observations_pending: Option<u64> }`;
  - `Parked { resources: LiveEpochResources, spectrum: SpectrumDisplaced, observations_pending }`.
- `spectrum.rs`'s three-state `EpochSpectrum` goes. Its `Candidate` arm becomes the struct
  `CandidateSpectrum`, and its `Displaced` arm is `Parked`'s non-optional `SpectrumDisplaced`. A
  pending epoch always has a candidate half, and a parked one always has what its successor's
  promotion displaced.
- The constructors set the phase: `ProviderEpoch::current` is `Running`, and
  `ProviderEpoch::candidate` takes the discontinuity and the spectrum half and is `Pending`.
  Two transitions change it, and both run only in `promote_pending_provider`:
  - `promote()` (pending to running) returns the candidate's spectrum half to install;
  - `park(resources, displaced)` (running to parked) drops the kept base with the running phase
    and carries a waiting pairing over.
  The read-before-move pass, which already refuses `Internal` before anything moves, also checks
  that every epoch it is about to promote is pending and that the running one is running. So
  each transition finds the phase it moves from, and the container and the phase cannot
  disagree after a promotion.
- Accessors read the phase: `is_pending`/`is_running`, `kept`/`kept_mut`, `keep`,
  `discontinuity`, `candidate_spectrum_mut`, `parked_resources`, `observations_pending`/
  `set_observations_pending`. A read in the wrong phase answers "none". That is what the old
  fields held there, except a promoted epoch's spent `discontinuity`, which was never read. A write in the wrong phase (`keep` on a non-running epoch, a pairing mark
  on a pending one) is a `debug_assert`, because the callers' container rules exclude it.
- **PR-7 is not folded in.** It is a different outcome: the `Primed` record's lifetime, its
  deadline step and the republish path (`restore_prime_flags`, which PR #2281 edits). It attaches
  later as `prime: Option<host_core::PrimedCandidate>` in `EpochPhase::Pending`.
  `SessionState::primed` then becomes `pending_providers.last()`'s field, so the stale-record
  clears at render's adoption and on a `Taken`/`Nothing` withdrawal (`synchronize_plan_epochs`,
  `withdraw_pending_candidate`) become `promote()` dropping the phase. The variant's doc records
  this.
- Control-thread only. Every epoch container is a `SessionState` field reached through
  `&mut self`, and the only cross-thread words nearby (`shared.active_epoch`, the report mutex)
  are not part of the phase. So no loom model is needed.

## Non-goals
- No change to `pending_providers` being a `Vec` with room for two (`PENDING_PROVIDERS`, used in
  the admission row bounds), to `kept_holder`'s two-pending arm, or to `restore_prime_flags`
  (PR #2281).
- No move of the kept base onto the pending successor, and no change to `meter_tables`'s
  exchange.
- No render-path change. `ProviderEpoch` is control-plane state, and no rendered bit can move.

## Gates
- No stale state is reachable, so no test can be red on base. Recorded instead: the
  reachability proof above, and a mutation run that shows which new transitions and accessors
  the existing lifecycle tests defend. One new test covers the invariant that the phase does not
  encode (below).
- `cargo test --locked` for control-plane and the crates that depend on it (capi, host-core,
  host-web, effect-contract), with the CI test-support features; clippy `-D warnings`;
  `cargo fmt --check`; rustdoc `-D warnings`; `bash scripts/check-workspace-policy.sh`; the
  realtime policy tool, run as a check (no marked region changes); the control-plane `cfg(loom)`
  leg.

## Evidence (attempt 1)

**Model as built** (`crates/control-plane/src/control.rs`, `crates/control-plane/src/spectrum.rs`).
- `EpochPhase` is described under Decision / model. Only one `impl ProviderEpoch` block reads or
  writes the private `phase` field. It holds two predicates (`is_pending`, `is_running`), the
  accessors and the two transitions.
- `promote_pending_provider` now has three steps:
  1. Its read-before-move pass refuses `Internal` unless every epoch to promote `is_pending` and
     the running one `is_running`.
  2. `next.promote()` gives up the candidate's spectrum half, which goes to
     `SpectrumMonitor::adopt`.
  3. `previous.park(row, displaced)` replaces the three hand-written assignments.
- `held_epochs` reads `parked_resources()`. `settle_observation_pairings` reads and settles the
  mark through `observations_pending()` and `set_observations_pending`. Every `.kept` site goes
  through `kept()`, `kept_mut()` or `keep()`.
- `ProviderEpoch::candidate` takes `discontinuity` and the `CandidateSpectrum`, so a candidate
  cannot be pushed without its spectrum half.
- **The invariant the phase does not encode.** A running epoch keeps a base exactly while a
  successor is pending. That spans two containers (`providers.kept()` against
  `pending_providers`). `park` drops the base with the running phase, and `promote` starts the new
  running phase with none. But a running phase can still be given a stale base (mutation M8,
  below), and no existing test noticed.

**New test.** `a_kept_base_drops_when_render_adopts_its_successor`
(`crates/control-plane/tests/runtime_preparer.rs`). It runs a rebuild, renders until the
successor's revision applies, runs the service step, and then requires that nothing is pending and
that the running epoch keeps no base.
- Test value: it turns red if a promotion leaves a kept base on the epoch it makes running, whether
  the base moved with the epoch or was never dropped. Both admissions would then charge a model
  that no plan renders, because `held_models` reads the running epoch's base, until a later rebuild
  replaces it. No existing test catches this: M8 passes all 112 binaries.
- It is green on base `24ea03525` (base sources with this test, separate target dir: `1 passed`),
  because the defect is not present there. Under M8 it is red:
  `panicked at crates/control-plane/tests/runtime_preparer.rs:521:5: the newly running epoch keeps
  no base`.

**Mutation run.** Each mutation was applied to head's `control.rs`. Every test binary of
control-plane, capi, host-web, host-core and effect-contract (112 binaries, CI test-support
features) was then run, eight in parallel. The reference run at head failed none.
`alternating_configures_against_a_rendering_thread_allocate_nothing_on_render` was skipped
during the mutation runs, because it fails under load on base too (see below).

| # | Mutation | Failing tests (binaries) |
|---|---|---|
| M1 | `promote()`'s spectrum captures are dropped, not installed | 18 (spectrum_monitor, worker_handoff, protocol_cross_host, capi) |
| M2 | `park` drops a waiting observation pairing | 0, survives (see below) |
| M3 | `set_observations_pending` is a no-op | 1 (`a_promotion_before_the_carry_pairs_the_carried_tap_once_render_carries`) |
| M4 | `discontinuity()` always `false` | 1 (`an_edit_that_supersedes_a_discontinuity_successor_is_prepared_at_it`) |
| M5 | `keep` is a no-op | 58 (16 binaries) |
| M6 | `kept_mut` answers `None` | 15 (5 binaries) |
| M7 | `parked_resources` answers `None` | 5 (resource_lifecycle, plan_swap_race, capi) |
| M8 | promotion moves the displaced epoch's base onto the new running epoch | 0 before the new test; red with it |

M2 survives because nothing reachable can observe it. A mark (on an epoch B, waiting to pair B
with the epoch P0 it displaced) can still be waiting when B is parked, as C is promoted, in two
ways:
- Render claimed C without running B's swap block. This cannot happen: render runs that block,
  which writes B's carry record, before any later claim. Every promotion settles the pending
  pairings first, so B's mark is already cleared when C's promotion parks B.
- P0's plan has no host attachment. The swap block then never calls `adopt_predecessor`
  (`crates/engine/src/realtime/plan.rs` 1508, at `5b5450d9a`), so B's record is never written. B's
  mark waits until P0's plan is reclaimed, which settles it as nothing carried, and C's promotion
  can park B before that. Dropping the mark at `park` is equivalent here: nothing carried into
  B, so B's readers are its own whether B settles before C pairs with it or not. Pairing C with
  B early or late reaches the same state.

On base the mark stayed in the field. `park` keeps it, as defence in depth for
`settle_observation_pairings`'s parked-successor arm, rather than drop a mark whose loss no
reachable path can observe.

**Load-dependent failure, pre-existing.** `spectrum_monitor::alternating_configures_against_a_
rendering_thread_allocate_nothing_on_render` fails its `ready > 0` assert ("some reads analyzed a
window") when the machine is heavily loaded (load average 120 on 32 cores, from other agents). It
then hangs instead of failing: the assert panics inside `thread::scope` before `done` is stored,
so the render thread never stops. Base and head binaries were run interleaved, 16 copies each, at
the same time: base 2 passed and 14 timed out, head 3 passed and 13 timed out. Alone, head passes
6 of 6 runs (1.7-3.4 s), and it passes in the serial `cargo test` runs. This is not caused by
this change; it is listed as a follow-up.

**Gates (head).**
- `cargo test --locked -p control-plane -p capi -p host-web -p host-core -p effect-contract
  --all-targets --features <the CI list>`, at the final head: 112 binaries, 1,224 passed
  (1,223 plus the new test), 0 failed, 10 ignored.
- `cargo clippy --locked -p control-plane --all-targets --features test-support -- -D warnings`:
  clean. `cargo clippy --locked -p control-plane -- -D warnings`: clean. Without features, clippy
  `--all-targets` reports a dead `compile_children_warm` (`compile.rs`, untouched here). That is
  pre-existing, and CI runs clippy with `--all-features`.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p control-plane`, with and without
  `test-support`, and with `--document-private-items`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `./target/release/realtime_policy`: `realtime policy: ok (292 marked regions in 46 files)`.
- control-plane `cfg(loom)` leg (`loudness_arming_loom`, the qualification invocation, its own
  target dir): `1 passed`.
- No loom model is added. The phase is control-thread state (see Decision / model).
- No render-path change, so no no-bit-moved comparison is needed. `ProviderEpoch` never reaches
  render.

**Attempt-1 verdict: PASS** (adversarial review). The reviewer found every transition equivalent
to base. The branch merges cleanly with `main`, and the merged tree passed 1,229 tests. The new
test was accepted as a legitimate guard. Three minors are folded after merging `origin/main`
(`5b5450d9a`):
1. The M2 reasoning above now covers the second way a mark can still be waiting at `park`: a
   predecessor plan with no host attachment.
2. `promote_pending_provider` no longer drops a `None` row silently
   (`parked.into_iter().take(index + 1).flatten()`). Its pre-pass refuses `Internal` unless
   there is a row slot for every epoch it parks. The loop takes each row with
   `let Some(row) = row else { debug_assert!(false, ..); return Err(CommandError::Internal) }`,
   which that pre-pass makes unreachable.
3. The follow-ups below record what the `Option` refactor would encode, and how PR-7's
   non-promotion clears attach.

Re-run on the merged tree, with the folds:
- `cargo test --locked -p control-plane --all-targets --features test-support`: 10 binaries, 102 passed, 0 failed.
- clippy `-D warnings` (control-plane with test-support, and the lib alone): clean.
- `cargo fmt --check`: clean.
- rustdoc `-D warnings`, with `--document-private-items`: clean.

**Follow-ups (not in this issue).**
- PR-7: put the `Primed` record in `EpochPhase::Pending`, as described under Decision / model.
- `pending_providers` never holds more than one epoch (see the proof). It could become an
  `Option`, which would remove `kept_holder`'s and `restore_prime_flags`'s dead two-pending arms.
  With pending exclusive, the kept base could then move onto the pending successor. That would
  encode by type the invariant `a_kept_base_drops_when_render_adopts_its_successor` guards: a
  base exists only while its successor is pending, and drops with it.
- PR-7's record also has clears that are not promotions: the deadline step's `Stale` clear
  (`check_prime_deadline`) and `withdraw_pending_candidate`'s clear of a record whose epoch is
  not the `Withdrawn` candidate's. They need a `Pending`-phase accessor (for example
  `prime_mut()` on the pending epoch), not `promote()`, which handles only the render-adopted
  and `Taken`/`Nothing` clears. #2258 tracks this and the item above.
- `alternating_configures_against_a_rendering_thread_allocate_nothing_on_render` should not hang
  when it fails (store `done` on unwind, for example with a drop guard). Its `ready > 0` claim
  also depends on render keeping pace under load.
