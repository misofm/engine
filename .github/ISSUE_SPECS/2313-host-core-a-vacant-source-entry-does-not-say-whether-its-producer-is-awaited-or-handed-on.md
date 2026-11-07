# Host core: a vacant source entry does not say whether its producer is awaited or handed on

Item HC-5 of the implicit-state-machine tracker #2258 (audit of 2026-11-06, agent-reported,
re-verified here on `d441cd408`). Follows #2273, which recorded each entry's immutable `carried`
flag.

## Problem
`ControlSource.provider: Option<HostChunkProvider>` and `SourceControlSet.timeline:
Option<TimelineProducer>` (`crates/host-core/src/source.rs`) use `None` for four states:
- *awaiting*: the plan carries the ring of the plan before it and the producer has not arrived
  (`PreparedProducer::Carried`);
- *handed on*: the producer moved to a successor's set by its `adopt_persisting`;
- *donated*: the producer moved into a successor's fresh entry by `donate_producers`;
- *projected*: a projection (#2199) has no ring and no producer (`PreparedProducer::Projected`).

Every per-source call refuses all four alike with `SourceControlError::Vacated`, which is right:
none holds a producer. The states differ in what may fill the entry, and the code did not say so.
`adopt_persisting` filled any `None` entry from the other set's entry with the same ID, and it is
called in two directions (`crates/control-plane/src/control.rs`, `rebuild`):
- *forward*: a successor B takes the producers of the rings it carries, from the newest epoch,
  or, across a withdrawn candidate A, from A and then P0 (`#1325 D6` restores);
- *give-back* (#1310 D5 step 5): P0 takes back from the withdrawn A the producers it handed A that
  B did not take, because P0 renders their rings until B is adopted.

By vacancy alone, the give-back would also fill a P0 entry that is vacant for another reason (one
awaiting its own predecessor's producer, or one whose producer went to another set) with A's
*fresh* producer of the same ID, whose ring drops with A. P0's set would then ack PCM into a ring
nobody renders. The forward call would likewise refill a handed-on entry.

**Not reachable today (proof).** All producer moves are in `source.rs` (`push`,
`adopt_persisting`, `donate_producers`); the field is crate-private.
- A projection's set is built and dropped inside `prepare` (`prepare.rs`, `if projecting { return
  Ok(PrepareOutcome::Projected(..)) }`; the set is read only for `retained_bytes`), so no
  adoption ever sees a projected entry.
- A forward call's successor is freshly prepared: its vacant entries are exactly its `Carried`
  ones (donations fill only its fresh entries, which are already occupied).
- Donated entries exist only in the withdrawn donor A, which is popped and dropped in the same
  `rebuild`; the donated IDs are B's fresh entries, so B awaits none of them.
- In the give-back, a vacant P0 entry `x` was handed either to A (A carries `x`, so A's entry
  holds P0's producer or has handed it to B) or to B directly as a restore (`x` is in P0's duck
  overlay, and `DuckOverlay::joined` keeps an overlay source only when A's transaction does not
  hold it, so A has no `x`). A P0 entry that awaits its own predecessor's producer cannot arise:
  a successor is prepared against the newest epoch, which holds every producer of its own.

So, as with #2257 and #2273, this is hardening: the states become explicit, each move names the
state it fills, and the give-back can no longer be a forward call by another name. The tracker's
"set lifecycle (Prepared -> Adopted -> Drained)" is not a separate state: it is the union of the
entries' states, and a set-level copy would be a shadow of them (tracker pattern 2). It is not
modelled.

**Not read on render.** `SourceControlSet` is control-plane state: render owns the consumers,
inside the plan, and reads only the rings and the producer's prime flag, neither of which this
changes. No loom model applies, and no render-path code changes.

## Contract
- `Producer<P> { Awaiting, Owned(P), Handed, Donated, Projected }` replaces the `Option` for every
  source entry and the timeline (`Projected` is used for sources only; a projection's timeline is
  never prepared and stays `Awaiting`, inside the preparation that drops it).
- `adopt_persisting` (forward) fills only `Awaiting` entries, from an `Owned` entry of the
  predecessor, which it leaves `Handed`. This is the #2273 follow-up ("move a producer only into
  `carried && provider.is_none()`"), stated as a state: `Awaiting` is made only from `Carried`.
- `reclaim_from_withdrawn` (new; the give-back) fills only `Handed` entries, from an `Owned`
  entry of the withdrawn set that `carried` this set's ring. The control plane's #1310 D5 step 5
  calls it instead of `adopt_persisting`.
- `donate_producers` leaves the donor entry `Donated`.
- Every vacant state is still refused `Vacated`; no diagnostic, wire or report changes.
- In today's production order the new rules move exactly the producers the vacancy rule moved.

The #2273 follow-up's guard as written (`carried && provider.is_none()`) would have broken the
give-back: P0's handed-on entries need not be `carried` (a first plan carries nothing). The
explicit `Handed` state is what the give-back fills.

## Non-goals
HC-9 (session seek publishing before reporting failure, `source.rs`): separate, later.
`crates/host-core/src/spectrum.rs`, `meters.rs`. `SourceControlError` and its diagnostics. Render.

## Gates
- A regression test red on base: `source::tests::a_give_back_returns_only_the_producers_the_withdrawn_set_took`.
- Mutations of the new fill rules turn it red.
- A one-time check (not committed) that the base and new rules decide every move alike over the
  host-core, control-plane, capi and bench suites.
- `cargo test --locked` for host-core, control-plane, capi and bench; clippy `-D warnings`;
  `cargo fmt --check`; `cargo doc -D warnings`; `scripts/check-workspace-policy.sh`.

## Evidence (attempt 1)

**Verified on `d441cd408`.** The four meanings of `None` hold as listed. No reachable confusion
was found (proof under Problem). The defect is in the API: one vacancy rule serves two opposite
moves, and the give-back is the forward call run backwards. Nothing here is read on render, so
there is no loom model and no render-path change.

**Fix.** `Producer<P>` (`crates/host-core/src/source.rs`) holds every entry's producer and the
timeline's. `adopt_persisting` fills `Awaiting` only. `reclaim_from_withdrawn` fills `Handed`
only, and only from a `carried` entry; the control plane's #1310 D5 step 5 (`control.rs`,
`rebuild`) and the one test that does the give-back (`crates/host-core/tests/withdrawn_successor.rs`,
`reverted_owner_carries_from_p0`) call it. `donate_producers` leaves `Donated`. The builder maps
`PreparedProducer::{Carried, Projected, Fresh}` to `Awaiting`, `Projected` and `Owned`. Every
read goes through `Producer::owned`/`owned_mut`, so every vacant state is still refused `Vacated`.
`a_vacant_entry_refuses_the_whole_seek` now vacates its entries as `Handed` and `Awaiting`
instead of `Option::take`.

**Regression test.** `source::tests::a_give_back_returns_only_the_producers_the_withdrawn_set_took`
uses hand-built sets in three cases:
1. P0 awaits `y` and A redeclared `y`. The give-back returns only `x`.
2. P0 handed `x` to another set and A redeclared `x`. Nothing returns, and a forward adoption
   does not refill the handed-on `x`.
3. P0 awaits `y` and A carries a `y` whose producer came from elsewhere. Nothing returns.

Red on base, with `reclaim_from_withdrawn` spelled `adopt_persisting` (the base's give-back call)
on the `d441cd408` source:
- case 1: `assertion left == right failed: only x returns, left: 2, right: 1`;
- case 2 alone: `A's x stays, left: 1, right: 0`.

Green at head.

**Test value.** It goes red if a give-back fills a running entry by vacancy alone. An entry that
never handed its producer to the withdrawn plan would then take a producer of that plan's own,
whose ring drops with it, and the running set would ack PCM into a ring nobody renders. It also
goes red if a forward adoption refills a handed-on entry. No existing test builds a vacant
running entry that the give-back must leave alone.

**Mutations (each reverted).**
- `reclaim_from_withdrawn` also fills `Awaiting`: case 3 goes red (`P0 handed no y, left: 1,
  right: 0`).
- `reclaim_from_withdrawn` drops the `carried` check: case 2 goes red (`A's x stays`).
- `adopt_persisting` also fills `Handed`: case 2 goes red (`x is not awaited`).
- `control.rs` step 5 calls `adopt_persisting` instead of `reclaim_from_withdrawn` (it now fills
  `Awaiting` only, so no give-back happens):
  `control::tests::warm_successor::edits_while_the_warm_candidate_waits` goes red
  (`aux-source at 1024: Control(Vacated)`). The explicit states make the two directions
  non-interchangeable, and an existing gate notices.

**No move changed (one-time, not committed).** A temporary check at the top of
`adopt_persisting` and `reclaim_from_withdrawn` compared, for every entry the other set holds and
for the timeline, the base decision (fill if vacant and the other entry holds a producer) with
the new one, asserting equality. It passed over:
- `cargo test -p host-core --features test-support,control-provider`, excluding the new test,
  which builds diverging cases on purpose: about 1,030 adoptions and 4 give-backs;
- `cargo test -p control-plane -p bench` with test-support: 470 adoptions and 70 give-backs;
- `cargo test -p capi` with the test-support features: 1,306 adoptions and 115 give-backs.

The counts are approximate, because parallel test threads interleaved some log lines. The check
was then reverted. Coverage is thinner than the give-back count suggests: the reviewer's own
equivalence check (3,011 adoptions, 192 give-backs) found that only 3 give-backs actually move a
producer; the rest have nothing to return. The timeline branch of `reclaim_from_withdrawn` moves
nothing in any production flow (every successor takes the timeline), so only the unit test
reaches it. Render reads none of this state, and no producer moves differently in any
existing flow, so no rendered bit moves.

**Gates (head).** Listed in the issue's evidence comment. Summary:
- `cargo test --locked` for host-core (test-support, control-provider), control-plane + bench,
  and capi (test-support features): all pass;
- clippy `-D warnings` (host-core with no features, with `test-support`, and with
  `test-support,control-provider` together with control-plane and bench): clean;
- `cargo fmt --check`: clean;
- `cargo doc -D warnings` for host-core and control-plane with
  `host-core/test-support,host-core/control-provider`: clean. host-core with no features fails
  on the pre-existing `prepare.rs:262` link to the feature-gated `crate::warm::production_config`,
  which also fails on base and is not touched here;
- `scripts/check-workspace-policy.sh`, `check-host-core-policy.sh`, `test-host-core-policy.sh`:
  ok;
- realtime policy tool: not run, because no render-path code changed.

**Follow-ups (not in this issue).**
- `SourceControlError::Vacated` still names all four vacant states with one diagnostic. Splitting
  it would be a host-visible change and needs its own decision.
- host-core `cargo doc` with no features fails on the `prepare.rs:262` intra-doc link.

## Verdict (attempt 1)
**PASS** (Sol's adversarial review). The reviewer re-derived the unreachability proof, ran their
own base-vs-new equivalence check (3,011 adoptions, 192 give-backs, all equal) and agreed that the
#2273 guard as worded would have broken the give-back. Four minors, folded on the same branch
after merging `origin/main`:
1. The `rebuild` doc in `control.rs` no longer says "every producer A still holds that P0
   holds vacant"; it says "every producer P0 handed A that A still holds" and names
   `reclaim_from_withdrawn`.
2. The timeline branches are pinned: the regression test gains a timeline case (A takes it; a
   forward adoption does not refill a handed-on timeline; a give-back does not fill an awaited
   one; the give-back returns it). Mutations: `adopt_persisting`'s timeline rule also filling
   `Handed` goes red ("a handed-on timeline is not awaited"); `reclaim_from_withdrawn`'s timeline
   rule also filling `Awaiting` goes red ("an awaited timeline was handed nothing"); its timeline
   branch removed goes red ("the timeline returns").
3. `Producer::Donated`'s doc says no fill reads it, so it behaves as `Handed` does in a set dropped
   before any reclaim, and is kept only to document how the producer left.
4. The evidence states that only 3 of the give-backs move a producer.
