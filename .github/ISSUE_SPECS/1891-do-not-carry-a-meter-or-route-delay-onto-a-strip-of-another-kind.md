# Do not carry a meter or a route delay line onto a strip that reuses an ID with another kind

Filed by root from #1593 T2b's finding (backlog sprint, 2026-10-24).

## Product outcome

A transaction can remove track `x` and add submix `x` (the same stable ID, another kind). #1593 T2b fixed this for spectrum observers: `resolve_spectrum_carry` now pairs only when the ID names a strip of the same kind in the base and successor models. Two other carry resolvers in `crates/host-core/src/prepare.rs` match on the ID alone and may carry state across such a kind change:

- the meter carry (`resolve_meter_carry`, `meters_may_carry`, near line 3980) matches on strip ID, tap, metrics and config, so a track meter's window could carry onto the new submix;
- the route delay carry (`resolve_delay_carry`) matches the route edge ID plus its endpoints; if an endpoint is only the strip ID, a compensation line on a re-sourced route could carry across the kind change (not yet confirmed).

Fader and matrix lanes, effects, input sections and strip delays are not affected: their joins look strips up by kind.

## Smallest closable slice

Apply T2b's rule to both resolvers: carry only when the strip ID names a strip of the same kind in both models; otherwise start at rest. One red-first test per resolver in `crates/host-core/tests/successor_swap.rs` (remove track x and add submix x in one transaction; the meter or the delay line starts at rest, bit for bit against a fresh plan). Confirm or refute the delay case first, by test.

## Paths

`crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`, this spec.

## Decision record (attempt 1, 2026-10-24)

- D1 (root): one helper, `strip_kind_of(model, id) -> Option<StripKind>` in `prepare.rs`, with `same_strip_id_kind(before, after, id)` comparing the kind's discriminant (not `StripKind`'s `PartialEq`, which compares the strip contents). #1593 T2b's `same_strip_kind` is now a wrapper over it. The lookup binary-searches: both models are normalized (a successor refuses an unnormalized committed model).
- D2 (root): the rule is injected as a `same_strip: impl Fn(&str) -> bool` closure into `resolve_meter_carry`, `meters_may_carry` (so "may carry" stays exact) and `resolve_delay_carry`. Every call site passes `|id| same_strip_id_kind(base.committed, <successor's normalized model>, id)`.
- D3 (root, amended in attempt 2): the delay case is real. The graph compiler names a submix's stage as a track's (`GraphNodeId::TrackStage`, #1203 D3) and a submix's effect as a track's (`GraphNodeId::Effect`), so a route that keeps its ID and tap across a source kind change, or a sidechain key into an effect whose strip changed kind, has the same edge and endpoints. A line carries only if every `TrackStage` and `Effect` endpoint passes `same_strip`; other endpoints pass. A strip of another kind then starts its lines at rest, as an added strip does. The grammar reaches the sidechain case: submix inserts take a routed sidechain as track inserts do (one `validate_effect`). A route's `TrackStage` destination is a submix input on both sides (a route never targets a track), so a kind change cannot reach it; it stays checked, with a comment. Fixed in host-core, not in the graph compiler (its node IDs are sealed graph text).
- D4 (root): a fresh meter on a strip of another kind still joins the carried window grid (#1327 D6): a grid, not state.
- No rendered bit moves on any carry that is unchanged; the carry program only shrinks. No public API change.

## Evidence

- Red first (commit `bfb01511a`, pre-fix code): `meter_carry::a_track_replaced_by_a_submix_of_its_name_carries_no_meter` failed (21 meter carry records, expected 14: the submix's 7 readers were paired with the track's meters); `a_route_whose_source_changed_kind_does_not_carry_its_line` failed (the swapped run differs from fresh B at the swap block: `bus1` played the removed track's pending audio).
- Green after the fix: both pass at every width, with `a_track_replaced_by_a_submix_of_its_name_does_not_carry` and the meter and PDC suites (`cargo test -p host-core --all-features --lib --test successor_swap`: 97 and 120 pass).
- Attempt 2 (verifier findings): the meter reproducer also changes `eq1`'s trim, so a kind test that compares whole `StripKind` values (strip contents) goes red: mutant run, 7 carry records against 14, restored. New reproducer `a_sidechain_line_onto_a_strip_of_another_kind_starts_at_rest`, committed red at `2b594ed7d` (first difference 3 blocks after the swap: the submix's compressor read the track's pending key), green with the `Effect` arm. Its reference is the swap from A keyed from `eq0`'s input tap, whose line has another source and starts at rest; an A without `eq1` is no reference, because an added strip fades in and a restarted one does not.
