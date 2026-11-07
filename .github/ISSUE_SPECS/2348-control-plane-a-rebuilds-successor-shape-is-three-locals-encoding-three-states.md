# Control plane: a rebuild's successor shape is three locals encoding three states

Tracker #2258, item PR-9. Pattern: several bools/Options whose combinations encode a phase.

## Mission
`ControlSession::rebuild` (`crates/control-plane/src/control.rs`) decides, once, what kind of
successor it prepared: the ordinary one, a warm one (with or without a primed lead), or the
transition fallback. That decision should be one value whose variants are exactly the shapes that
exist, so a later edit cannot set a combination no shape has.

## Verified (base `900c3d24c`)
Three mutable locals carry the decision (`control.rs` 6169-6173): `warm_lead_blocks: Option<u32>`,
`transition_fallback: bool`, `replanned: bool`. They are written only in the one `if`/`else if`
chain that follows:

| Writer | `warm_lead_blocks` | `transition_fallback` | `replanned` |
|---|---|---|---|
| none (ordinary successor, or `WarmDecision::Ordinary`) | `None` | `false` | `false` |
| `fallback` (across a withdrawn donor, 6183-6185) | `None` | `true` | `true` |
| `WarmDecision::Warm` with no pending duck, `lead_samples == 0` (6217) | `None` | `false` | `true` |
| `WarmDecision::Warm` with no pending duck, `lead_samples > 0` (6217-6219) | `Some(blocks)` | `false` | `true` |
| `Warm` with a pending duck, or `Unavailable` (6223-6225) | `None` | `true` | `true` |

So four of the eight combinations are reachable, and they are three shapes (the warm shape has an
optional primed lead). They are read at 6232 (`replanned`: plan the transition again over the new
successor's restarted strips), 6325 (reprepare admission peak), 6404 (`CandidateOutcome`), 6647
(`publish_primed`), 6822 (`hold_primed_until` versus `NoEarlierThan`) and 6902 (the `Primed`
record).

The other four combinations (`transition_fallback` without `replanned`; a primed lead with
`transition_fallback`; a primed lead without `replanned`) cannot be constructed on base: every
writer sets the locals together in one arm. **No defect is observable on base.** The encoding is
one future edit away from an incoherent state, for example a fallback that forgets `replanned`
would publish a transition fallback whose duck set is the ordinary successor's.

## Decision / model
A local `enum SuccessorShape { Ordinary, Warm { primed_lead_blocks: Option<u32> }, Transition }`,
set by the same arms. `replanned` is `!matches!(shape, Ordinary)`; the outcome is a `match` with
`Transition => TransitionFallback`; the primed lead is projected once, from `Warm` only, into the
`warm_lead_blocks` the four later readers already use. Two locals and every combination no shape
has are gone. No behaviour changes.

## Non-goals
- The later readers keep reading the projected `Option<u32>`: it is derived once from the shape,
  never written, so it cannot disagree with it.
- No render path, no cross-thread state, no public API.

## Gates
- No red-on-base test: nothing is observable on base, and AGENTS.md refuses a structural/grep
  test. Instead, mutations of the new model must turn existing tests red, and a one-time
  no-bit-moved comparison against base is PR evidence (not a committed test).
- `cargo test --locked -p control-plane --features test-support`; the capi integration tests that
  drive transitions (`strip_transitions`, `latency_growth`).
- `cargo clippy --locked -p control-plane --all-targets --all-features -- -D warnings`,
  `cargo fmt --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p control-plane`,
  `bash scripts/check-workspace-policy.sh`.

## Evidence (attempt 1)
**Mutations.** Each mutant of the new model, run against
`cargo test -p control-plane --features test-support --lib`:

| Mutant | Red tests (`control::tests::warm_successor::`) |
|---|---|
| zero lead published primed (`Some(lead_blocks)` always) | `a_zero_lead_is_published_as_an_ordinary_duck_swap` |
| a lead never primed (`None` always) | 14 tests, e.g. `a_growth_is_published_primed_and_recorded`, `a_warm_candidate_is_admitted_with_the_reprepare_peak` |
| warm not planned again (replan only for `Transition`) | `a_zero_lead_is_published_as_an_ordinary_duck_swap` |
| transition not planned again (replan only for `Warm`) | 7 tests, e.g. `a_growth_past_the_bound_takes_the_transition_at_submit`, `a_sidechain_key_out_of_a_grown_strip_is_ducked` |
| `Transition` outcome `Exact` | 6 tests, e.g. `a_warm_candidate_not_ready_by_its_deadline_takes_the_transition` |
| `Warm` outcome `TransitionFallback` | 6 tests, e.g. `a_latent_insert_on_an_audible_strip_is_duck_swapped_warm` |
| donor fallback sets `Ordinary` | 4 tests, e.g. `a_refused_fallback_at_the_deadline_keeps_the_donor_pending` |
| submit fallback sets `Warm { None }` | 3 tests, e.g. `a_growth_past_the_bound_takes_the_transition_at_submit` |

All eight are killed, so the existing `warm_successor` tests cover every arm and reader of the
model; no new test is added.

**No bit moved.** A temporary (uncommitted) `Drop` probe on the `warm_successor` test rig hashed
every rendered block's output bits and the epoch render published after it (FNV-1a), for each of
the 66 rigs the 17 `warm_successor` tests build (ordinary, warm primed, warm zero-lead, transition
at submit and across a donor). Base `900c3d24c` and head print identical lines for all 66; the
only difference in the sorted logs is the suite's wall time.

**Gates.** `cargo test --locked -p control-plane --features test-support`: all pass (33 lib + 69
integration). `cargo test --locked -p capi --test strip_transitions --test latency_growth`: 3 + 12
pass. clippy (`--all-features`), fmt, doc and the workspace policy: clean.
