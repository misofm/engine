# Host core: a warm candidate's prime flags depend on whether its producers have moved yet

Item HC-4 of the implicit-state-machine tracker #2258 (audit of 2026-11-06, agent-reported,
re-verified here on `de0878e23`).

## Problem
`publish_primed` (`crates/host-core/src/warm.rs`) writes the `prime_required` flag of every ring a
warm candidate W's readiness reads. It decided which rings W carries with
`candidate_sources.holds_vacant(id)`, which reads whether W's entry still has no producer
(`crates/host-core/src/source.rs`, `holds_vacant`). That is a fact about *when* it is asked, not
about W: `SourceControlSet::adopt_persisting` fills exactly those entries. So the function was
correct only if it ran before the adoption, a rule stated in its doc comment and nowhere else.
Called after it:
- `holds_vacant` reads `false` for every carried ring;
- `running_sources.set_prime_flags` skips the carried entries, which are now vacant there;
- `candidate_sources.set_prime_flags(|_| false)` reaches the carried rings through W's set and
  writes `false`.

Every carried ring then reads `false`, so render's readiness check (`source` crate,
`consumer.prime_required()`) ignores them and adopts W without the lead buffered on the rings it
replays. `restore_prime_flags` (`crates/control-plane/src/control.rs`), which re-sets the flags of
a republished `Primed` candidate after its producers moved, needed the opposite reading: it
inferred "carried" from the *predecessor's* entry being vacant, one of the meanings HC-5 records
for `provider: None`.

The one production caller (`rebuild`, `control.rs`) calls `publish_primed` before the adoption, so
this was not reachable in production; it was a call-order rule nothing checked, and the gate rig in
`crates/host-core/tests/warm_successor.rs` set the flag by hand instead of calling the function.

Not held: the tracker's second point, `PrimedCandidate::hold_until`'s epoch match being only a
`debug_assert_eq!`. A reservation borrows its `PlanPublisher` exclusively (`reserve_replacement`
takes `&mut self`), the control plane holds one publisher, and the record and the hold are made
from the same `reservation.inner` in one function, so no second reservation can be passed. Binding
the record to its candidate beyond that is PR-7's model (the record moves onto the pending epoch).

## Contract
- Whether a set's plan carries a source's ring is recorded once, when the set is prepared, and
  never changes; `SourceControlSet::carries(id)` reads it.
- `publish_primed` writes every flag from `candidate.carries(id)`, on the running, withdrawn and
  candidate sets alike, so its result does not depend on whether `adopt_persisting` or a donation
  ran first. The "call before adoption" rule is deleted, not enforced.
- `restore_prime_flags` uses the same rule, so it no longer reads producer vacancy.
- `holds_vacant` and `test_only_prime_flag`, which nothing then needs, are removed.
- Values written in today's production order are unchanged (at publication, before any producer
  moves, `holds_vacant(id)` equals `carries(id)` for every entry), so no flag and no rendered bit
  moves.

Model chosen: the tracker suggested a typestate `Reserved -> Primed -> Committed` with the adoption
consuming the `Primed` token. That enforces the order; recording the carry at preparation removes
it. A token would also have to pass through every `adopt_persisting` caller (about 30, almost all
non-warm) and the control plane's panic-republish path. The ordering is entirely on the control
thread (the flags' cross-thread publication, the commit's `Release` and render's `Acquire`, is
unchanged), so no loom model applies.

## Non-goals
HC-5 (`ControlSource.provider: Option` for awaiting/donated/projected): separate outcome,
follow-up. PR-7 (the `primed` record stored apart from its candidate).
`crates/host-core/src/meters.rs`, `spectrum.rs`. No render-path code changes.

## Gates
- A regression test red on base: the warm-successor gate rig publishes through `publish_primed`
  after the producers moved, and its readiness gates go red on base.
- `cargo test --locked -p host-core --features test-support,control-provider`, `-p control-plane`;
  clippy `-D warnings`; `cargo fmt --check`; `scripts/check-workspace-policy.sh`.
- Mutation: revert the candidate's flag rule to `|_| false`; the gate goes red.

## Evidence (attempt 1)

**Verified on `de0878e23`.** The order hazard holds as described (see Problem); it is not
reachable through the one production caller. The `hold_until` point does not hold (see Problem).
The ordering is control-thread only, so no loom model.

**Fix.** `ControlSource` gains `carried`, set once by the builder from a new
`PreparedProducer { Carried, Projected, Fresh(provider) }` argument (a projection's vacant entry
was a third, unnamed meaning of `provider: None`; it is not carried). `SourceControlSet::carries`
reads it. `publish_primed` and `restore_prime_flags` write every flag as
`candidate.carries(id)` on every set that holds a producer. `holds_vacant` and the test-only
`test_only_prime_flag` lose their last callers and are deleted; `tools/bench`'s swap stream reads
`carries`.

**Regression test.** The `Stream` rig of `crates/host-core/tests/warm_successor.rs` no longer sets
the carried ring's flag by hand: it publishes a warm successor through `host_core::publish_primed`,
after `adopt_persisting` has moved the producers. The file now needs `control-provider`, as
`withdrawn_successor.rs` does (CI's workspace step enables it through `control-plane`). Red on base
(this rig on the `de0878e23` source):
- `a_warm_successor_waits_for_every_carried_ring_and_a_held_seek`: panics at
  `warm_successor.rs:2041` `not adopted while withheld`;
- `a_warm_successor_displaced_before_its_first_block_keeps_its_prime` and
  `the_growth_chain_primes_without_allocating`: the process aborts in render (for the first, read
  with `--nocapture`: `crates/source/src/lib.rs:2770` `a prime after a true readiness neither
  underruns nor holds`).

Green at head (22 of 22).

**Test value.** The rig's readiness gates turn red if a warm publication's flags depend on where
the producers sit, so that a host publishing after the move gets every carried ring `false` and W
is adopted before the lead is buffered. No existing test caught this: the control-plane gates all
publish before the move, and the rig bypassed `publish_primed`.

**Mutations (each reverted).**
- `publish_primed`: candidate set back to `|_| false` (the base rule). The same three gates go red
  as on base.
- `restore_prime_flags`: candidate set to `|_| false`.
  `control::tests::warm_successor::a_republished_warm_candidate_keeps_its_own_prime_flags` goes
  red (abort at the same `source` readiness assertion).

**No flag moved (one-time, not committed).** Temporary assertions in `publish_primed` and
`restore_prime_flags` compared the base rule with the new one for every flag written, and checked
at each publication that every candidate entry's vacancy equals its `carries`. They passed over
`cargo test -p control-plane --features test-support,host-core/test-support` (46 publications,
98 flags compared; 7 restores, 18 flags) and `cargo test -p capi` with the test-support features
(3 publications, 3 flags), and were then reverted. No render-path code changed, so this is the
no-bit-moved evidence: render reads only these flags, which are unchanged.

**Gates (head).**
- `cargo test --locked -p host-core --features test-support,control-provider`: 58 binaries, 620
  tests, all pass.
- `cargo test --locked -p control-plane -p bench --features control-plane/test-support,host-core/test-support`:
  118 tests, all pass.
- `cargo test --locked -p capi --features control-plane/test-support,host-core/test-support,protocol/test-support,engine/test-support,session/test-support,graph/test-support`:
  234 tests, all pass.
- `cargo clippy --locked -p host-core -p control-plane -p bench --all-targets` (with
  `host-core/test-support,host-core/control-provider,control-plane/test-support`, and `-p
  host-core` with no features and with `test-support`) `-- -D warnings`: clean.
- `cargo fmt --check`: clean. `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
  `scripts/check-host-core-policy.sh` and `scripts/test-host-core-policy.sh`: ok.
- Realtime policy tool: not run; no render-path code changed.

**Follow-ups (not in this issue).**
- HC-5: `provider: None` still means both "awaiting the predecessor's producer" and "producer
  given to a successor" (and, before this change, "projected"). With `carried` recorded, an enum
  over the mutable producer state is a smaller change.
- PR-7: the `primed` record still lives apart from its candidate in `SessionState`.
