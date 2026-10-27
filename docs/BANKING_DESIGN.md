# Ideal SIMD banking: design

Date: 2026-10-26, amended the same day after the bounded review (a bounded opus-high review, verdict FAIL; re-check PASS-WITH-FIXES).
Tree read: `/home/bl/misofm/engine` at `80532ee4a` (main), the #1981 worktree at `a1a2bc128`,
the #2129 worktree at `bd18f6c28`. Read-only; nothing was built.

## Changelog (amendment 3, 2026-10-26, after #2132's review)

- **Split strips.** C1's post-fader sends split every track strip into two bank units linked by
  position (resident pairing, producer rows), which `pack_member_eligible` excluded, so no C1
  track bank was in a pack group. New slice 5a (#2142): a pack group is over **lane chains** and one
  permutation moves every unit of a strip; it files before #2135. Section 1.1's "pack group"
  term and the slice table are amended.
- **W=4 counters.** New slice 0a (#2143) reads #2132's counters at four lanes on the wasmtime guest; V8
  keeps the timing. Slice table amended.

## Changelog (amendment 2, 2026-10-26, after the re-check: PASS-WITH-FIXES)

- **N1.** "A strip's dwell expires" is a packing trigger (per-strip countdown at the drain);
  slice 5 (#2135) gate 3 shows a toggled strip packed `R_b` blocks later. Section 3.3.
- **N2.** The step's target is the full pattern sort, a bounded counting sort over the group's
  `2^S` patterns. The per-group bound is restated as `ceil(A/W) + 2^k - 2` per ranked slot
  `k`, derived for the sorted order (the first run is aligned at position 0) and checked against
  the W=4, k=2 counterexample (3 = 3). Convergence over several blocks under `B_total` is
  defined; the bound holds at convergence; between blocks the transient is bounded, not
  monotone, and the reason is stated. Section 3.3, finish line.
- **M2 text.** Every segment, banked or not, gets an `L`. Section 2.2.
- **Minor.** The 6a key is the canonical schedule index (a cohort's is its first member's), so
  today's order is reproduced exactly; a global cap on swap attempts per block; the worst case of
  a full `reset_lane` burst; a bank's window is the intersection over its members using `asap'`.
- **Filing.** #1981, #2131 (`reset_lane`, S1a) and #2129 (S1b) exist; their slice files are
  removed and cited. The remaining slices are self-contained specs.

## Changelog (amendment 1, 2026-10-26)

- **B1 (root accepted).** Staleness is a per-lane, per-slot fact of the strip, never of the bank.
  Every un-bypass from settled bypass resets that lane's slot state (`reset_lane(lane,
  DiscontinuityKeepParameters)`) and holds dry for `W`, whether or not its bank ran. Sections 3.4
  and 4.1 rewritten. #2129 S1 is split: **S1a** is `reset_lane` on the bank trait for every
  banking effect; **S1b** is the skip with per-lane staleness. The former slices 4a/4b are
  superseded by S1a. Every slice that changes a lane's bank membership or skip state (5, 6b, 7, 8)
  now depends on S1a and S1b.
- **M1 (accepted).** The stale bit and the hold counter are per-lane state and travel with the
  lane in every `swap_lanes` and in the plan carry. #1981 merges first; S1b lands second and owns
  the travelling bits (slice 3).
- **M2 (accepted).** The sink rule is stated: the executor's unit order is a topological order of
  the op graph contracted by cohort, so every non-member consumer of a member sinks with it; a
  cohort is never declined for this reason. Section 2.2, slice 6a (#2134).
- **M3 (accepted).** Static numbers recomputed with aux A's delay splitting its chain and its
  drive banking alone (section 7, finish line). The audit's "-17 % at W=4/16" is corrected to
  0 %; W=8/16 is -19 %, not -24 %. The static criterion is restated as a checkable
  no-mergeable-pair property, not a `ceil(n/W)` bound.
- **M4 (accepted).** The dynamic bound is per pack group, with the slack term derived and the
  greedy's termination argued. Section 3.3, finish line.
- **M5 (accepted).** A bypass-class dwell `R_b`, a pin for automated bypass, and a global move
  budget per block, with the worst-case bound. Sections 3.3 and 3.5.
- **M6 (not accepted by root).** The owner ruled "1981 - accept". The owner question on #1981 is
  removed; the gather-OR attribution stays only as a measurement note in slice 4 (#2133).
- **Minor.** The sweep fills by earliest deadline; lane order inside a bank stays id-ascending
  (#206) and the route fold is matched, not forced; slice 4 (#2133) uses a monotone last-non-zero frame
  counter; the carry carries the hold and stale bits; slice 5 (#2135) checks `can_swap_lanes` on lanes
  with staged spans.

Owner direction (2026-10-26): "the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## 0. Summary

The static cohort design stays. What changes is the two places where lanes are lost today:

1. **Compile time: cohorts are formed over a feasible level window, not a level.** A chain
   segment's window is `[ASAP, ALAP]` on the strip DAG. Any two segments with the same program
   key whose windows intersect may share a bank. On C1 the four groups, aux B and aux A's pre
   segment then share banks; `post_insert` no longer splits by insert count. The graph's canonical
   levels and schedule (sealed `MISO-GRAPH-V1`) are untouched: placement is a lowering decision.
2. **Render time: a bank-slot runs only for lanes that need it, and lanes are regrouped by activity
   through exact lane swaps.** The activity of a strip is a pattern: silent-at-rest, plus one
   settled-bypass bit per slot. The bank-slot skip (#2129) and the silence skip (#1107) fire on a
   whole bank-slot; the packer (#1981's mechanism) sorts strips by pattern so bank-slots become
   homogeneous and skip. Silence is a propagated fact, not a scanned one.
3. **The un-bypass law is a property of the lane.** A lane un-bypassed from settled bypass is reset
   and held for its effect's warm-up in every bank. That is what makes the regrouping class A.

No class-B summation change is proposed. The fold order into every submix and the master stays
the proven canonical order (`route_fold`'s association list, or #1782a's deferred home-order
fold), whatever bank a strip renders in. The un-bypass law itself moves the bits of the
un-bypass transition against today's E5/#1341 crossfade from warm state; root accepted it under
the owner's 2026-10-26 bypass decision (section 8).

## 1. The general model

### 1.1 Terms

- **Strip**: a track or submix. Dual-mono: two planes, independent state. A mono stem reads one
  channel on both planes. A strip is one lane of one bank chain.
- **Chain segment**: a maximal fused run of a strip's stages: `post-input builtins -> pre_insert
  slots -> [inserts] -> post_insert slots -> fader -> matrix -> routes`. A per-node insert (the
  delay, a sidechained effect, a third-party effect) splits the strip into two segments
  (`banks.rs:174-225`, `graph-compiler/src/lib.rs:7708`, `:11685`).
- **Cohort**: segments that share one chain program key and one bank. **Bank-slot run (BSR)**:
  one bank executing one slot for one block. Cost is per BSR, not per lane.
- **Lane chain**: the ordered bank units that render the same strips at the same positions,
  linked by resident pairing or a bank producer row (a strip split by a send, a stage meter or a
  sidechain tap). **Pack group**: a run of peer lane chains of one program, one width, one fold
  target and one collapse arming (`LaneMap::discover`; slice 5a (#2142)). A lane moves
  only inside its pack group, and a move trades a strip in every unit of its lane chain at once.
  A group also needs a **layout** that keeps every permutation exact (`group_layout_holds`): the
  group's written buffers are distinct, no first-row input is among them, and a plain unit inside
  the group's span sits between two whole rows. The colouring may recycle an earlier chain's
  inputs into a later chain's outputs (C1 with its sends at `post_input` does at W=8); such chains
  form no group.
- **Lane use**: `strip-slots that need to run / (BSR x W)`.
- **Activity pattern** of a strip: `silent_at_rest: bool` and `bypassed[slot]: bool` (settled
  bypass), plus `automated: bool` (a bind-time bit).

### 1.2 Division of work

| when | what | bound |
|---|---|---|
| compile (control thread) | windows, cohort formation, padding, pool choice, placement, bank binding, lane-map discovery, every allocation | `O(N log N + N W + N x slots)` (2.7) |
| render, per block | the facts (flags, no scan), the pack step (bounded swaps), the skips | 3.5 |

Render never allocates and never changes a bank's program, width or slot set. The reachable
groupings at render are the permutations of strips within one pack group; every one is a lane
permutation of preallocated AoSoA columns, so nothing is preallocated per grouping beyond the one
swap scratch and the one staging tile the lane map already holds (`runtime.rs:9412-9440`).

### 1.3 What a bank is at render

Unchanged: one `BankChain` per cohort, `W` lanes, planar gather in `W`-frame tiles, the slots in
chain order, scatter or fold. `LaneMap` (#1782a) is the one indirection, and every reader keyed by
`(unit, lane)` translates through it.

## 2. Compile-time planner

### 2.1 The cohort key: per chain, with per-slot subsequence membership

The key stays the chain program: the ordered `EffectProgramKey`s (`effect-contract/src/
lib.rs:1577-1598`), the rack location and the pool class. Session bypass is not in the key
(#1087). A member whose program is a subsequence of the leader's joins with identity slots
(`RackProgram::subsequence_mask`, `rack/src/lib.rs:124`).

**Why not a per-slot key.** A strip owns one lane index across every slot of its chain; that is
what makes the chain one gather and one scatter (G5, H2). A per-slot cohort needs a planar/AoSoA
round trip between every pair of slots, and the kernel-mixing form of it was ruled null at 1.10x
(`cross-bank-interleave.md`). Per-slot *membership* exists through the subsequence mask; per-slot
*placement* is section 2.2; the nodes the chain pass leaves unbanked get a second pass (2.4).

**A member whose segment ends early.** Aux A's pre segment (`gate, eq, comp`, then the per-node
delay) is a prefix of the groups' program. It may join the groups' bank; the cohort's chain fusion
then stops at the comp slot for the whole bank, because fusion is proved lane-wise on the lowered
dataflow (`chains_into`) and one lane's output leaves the chain there. The bank pays one planar
round trip between comp and drive (the H2 cost) instead of one padded three-slot bank (H5: about
9.3 us at W=8), which is the cheaper of the two. Planner decision; slice 0 (#2132)'s rows confirm it.

### 2.2 Level windows (ASAP/ALAP)

Today a chain is bucketed by the longest-path level of its first slot and a bank never crosses a
level (#96 F12, `rack-compiler/src/lib.rs:101-102`, `banks.rs:250-301`).

**The strip DAG.** Nodes are chain segments. There is an edge `a -> b` wherever any op path leads
from a stage of `a` to a stage of `b`: routes, sends, sidechain taps, PDC staging, a per-node
insert between two segments of one strip. The DAG is the contraction of the op graph by segment,
so it carries every dependency the executor must respect.

**Definition.** `ASAP(c) = 1 + max ASAP(producers)` (the existing longest-path level) and
`ALAP(c) = min ALAP(consumers) - 1`, with `ALAP(sink) = ASAP(sink)` for the session output's
segment. The window is `[ASAP(c), ALAP(c)]`.

**Feasibility.** A path `a -> ... -> b` implies `ALAP(a) < ASAP(b)`, so intersecting windows
guarantee no path between members. Pairwise is not enough for a set of cohorts, so every cohort is
assigned **one placement level `L`** with `max ASAP <= L <= min ALAP`, and every strip-DAG edge
goes strictly upward in `L`. **Every segment gets an `L`**, banked or not: a per-node segment (a
connected sidechain, the delay, an insert remainder) is a cohort of one and is placed by the same
sweep, so the contracted graph of 2.2's placement rule has a level for every node. A bank's
**window** after placement is the intersection over its members of `[asap'_m, ALAP_m]`.

**The sweep.** Process candidates in topological order of the strip DAG:

1. Ready set: segments whose producers all have a placement. Each carries
   `asap' = max(ASAP, 1 + max L(producers))`; `asap' <= ALAP` always holds, because a producer
   sits at `L_p <= ALAP_p <= ALAP_c - 1`.
2. Group the ready set by key class `(rack, pool class, program compatibility)`. Within a class,
   sort by `(ALAP asc, active_count desc, id asc)`: **earliest deadline first**; the F5.2 rule
   (full programs fill first) is the secondary key among equal deadlines. Take the first
   candidate; its `ALAP` is the stab point; every candidate with `asap' <= ALAP_first` is
   compatible; fill the bank with up to `W` of them in sort order; the bank's `L` is its members'
   `max asap'`. Repeat until the class is empty.
3. Commit the banks' `L`; their consumers may become ready.

Earliest-deadline grouping is the standard interval greedy; deterministic in the multiset of
`(window, key, id)`. It is never worse than today's level bucketing, which is the special case
`L = ASAP` for everybody, and it is optimal for the nested windows a mixing graph produces
(tracks `[0, M-1]`, submixes `[k, M-1]`).

**Placement at the lowering, not in the graph.** The canonical graph keeps its longest-path
levels and its level-major schedule (`canonical.rs:316-321` writes both into `MISO-GRAPH-V1`;
`schedule.rs:1-4`). The executor's unit order is a lowering artefact already (a bank is one unit
at its first member's position, `program.rs:532-536`). The new rule (**M2**):

> The executor order is a topological order of the op graph **contracted by cohort**: each
> cohort is one node, each other op is its own node. Kahn's algorithm with a `BTreeSet` ready set
> keyed by the **canonical schedule index** (a cohort's key is its first member's index). Kahn
> guarantees soundness whatever the key; this key reproduces today's unit order exactly on every
> plan whose cohorts are single-level, because today's order is the canonical order with each
> bank at its first member's position. Deterministic; `O(ops + edges)`.

Why this and not "decline the cohort when a member has an external consumer before the placed
position": a member's external consumers (routes, sends, meters, PDC staging, taps) are exactly
the ops that connect it to later segments; under the decline rule every merge in which one
member's consumers sit canonically before another member's producers is lost, which on any graph
with two submix depths is most merges. Under the contraction rule those consumers sink with the
unit. Acyclicity: an edge between cohorts goes upward in `L` by the sweep; an edge through plain
ops is a strip-DAG edge, which the sweep also ordered; so the contracted graph is acyclic.

**Colouring (#169).** `program::lower` colours buffers over the **executed** unit order, not over
the canonical schedule plus a window permutation. Within a cohort unit the member ops are gathered
before the kernel runs and scattered after, so the only intra-unit permutation is the one
`render_bounds` already handles (`program.rs:590-600`); across units nothing is permuted relative
to the order that was coloured. The #169 sentence, "no physical slot may be recycled inside a bank
window except to an op proven to render after the run that last touched it", holds with the
window being the unit. `cohort_chain_merging_preserves_dataflow_on_random_graphs` gains
cross-level cohorts and sunk consumers as constructed shapes (slice 6a (#2134)).

**Where the fold goes.** `route_fold` folds a chain's routes into the master only when the chains'
lane order realises the master op's ordered contributor list exactly, else it declines and the
master op's planar reduction runs: same bits, more cost (`runtime.rs:12030-12089`). Lane order
inside a bank is id-ascending (#206, a structural validation requirement) and is not changed for
the fold. A merged submix cohort folds when its id order happens to be the master's contributor
order, which on C1 it is when route ids sort as strip ids; otherwise the fold declines and the
report counts it. Bits never move.

### 2.3 Cross-section banking

- Same effect at different console slot positions on different strips: not banked together
  (console programs are uniform across strips, decision 12; across sections it would break one
  strip's fused chain). A console cohort is a chain cohort.
- Same effect as an insert at different positions, or on a track and a submix: banked by the
  leftover pass (2.4) when the chain pass cannot place the node, and by windows when it can.
- Different kernels in one loop: never (`cross-bank-interleave.md`).

### 2.4 The insert leftover pass

Pass 1 is the chain pass over `Dynamic` chains, with windows. Pass 2 takes every insert node pass
1 left unbanked (a remainder below `min_padded_insert_members`, an order that matches no leader)
and re-offers it as a one-slot candidate with its own window, grouped by `EffectProgramKey`,
padded from the same threshold, bound as a one-slot chain. A node still unbanked renders per node.
Eligibility stays #163 phase 1's (the kernel contract); only placement widens.

As built (#2138, `leftover_pass` in `crates/graph-compiler/src/banks.rs`): the candidates are
placed by the chain pass's own sweep and merge counter (`place_over_windows`, keyed by program key
and pool class) over a sub-problem of the candidates and their graph neighbours, each neighbour
fixed where the placement put it. A move therefore keeps every edge strictly upward and the graph
contracted by bank acyclic, and a merge across levels is taken only when it saves a bank.

### 2.5 Mono and stereo pooling

The pool class stays a class, decided once per compile. The planner gains a cost rule for the
**remainders only**: per `(rack, program, L)`, when the mono pool's last group has `m` members,
the stereo pool's last group has `s`, and `m + s <= W`, the mono members join the stereo group.
One bank fewer, 0.6 of a bank's cost saved; never breaks a full group; the class map is updated
before the builtin planner runs (the two-planner obligation). A mono-symmetric strip rendered
dual-mono emits the same words on both planes as its collapsed render, so this is class A.

### 2.6 Padding and the tie-break

Padding is unchanged (decision 12). Within a bank the lane order is strictly ascending by id
(#206). Across banks the order is `(L asc, rack, class, program, first member id)`. The plan is a
function of the multiset of `(window, key, class, id)` and `W`.

### 2.7 Complexity

`N` segments, `E` strip-DAG edges, `P` ops. ASAP and ALAP: `O(N + E)`. The sweep: `O(N log N)`
sorts, `O(N)` fills, `O(N x slots)` subsequence masks. The contracted topological order:
`O(P + edges)`. No `MAX_TRACKS`.

## 3. Render-time packing

### 3.1 Activity classes and the two skips

Per strip, per block: `silent` (input exact `+0.0` on both planes), `at_rest` (every slot's lane
at rest; #2129 D6: a settled-bypassed lane whose latency line holds only `+0.0` is at rest),
`bypassed[slot]` settled, `automated`. Two whole-bank skips: #1107 (every active lane silent and
at rest) and #2129 (every populated lane settled-bypassed on the slot, no span, no target, no
armed `None` tap). Per bank by design: a SIMD op runs every lane (decision 12 M4). The packer's
job is to make banks homogeneous so the skips fire.

### 3.2 Silence as a propagated fact

- **Sources.** The submit path copies the host's chunk into the ring on the control thread. It
  keeps a monotone **last non-zero frame** index per source (ring frame coordinates), advanced
  while copying whenever a word's bits are not `+0.0` (a `-0.0` word counts as non-zero:
  conservative, cost only). A block `[a, b)` is silent iff `last_nonzero < a` and every frame it
  reads past the written end is a `+0.0` the ring emits itself. One compare per block. No ABI
  change; no render work.
- **Chains.** A chain that skipped emitted `+0.0` on every lane (#1107 D7's signed-zero rule); its
  output flag is silent. A chain that ran is not.
- **Routes and folds.** A route is silent iff its source is silent or the route is muted and
  settled. A submix input is silent iff every route into it is silent: one AND per route.
- Mid-chain silence (a gate closing) is effect-specific and a later slice.

### 3.3 The predicate, the trigger, the move, the residency

**Pattern and demand.** `P = (silent_at_rest, bypassed[0..S])`. Bank-slot `(b, k)` skips iff
every populated lane of `b` has `silent_at_rest || bypassed[k]`. A bank's **demand** is the number
of its slots that cannot skip; the pack group's cost is the sum of demands. Slots are ranked once
at bind by the effect's measured isolated cycles per lane-sample (`effect-floor-accounting.md`),
tie by chain order; the pattern is compared as an integer with the costliest slot most
significant.

**Trigger.** `PackGroup::dirty` is set when a bypass switch settles on any lane of the group,
when a lane's idle count crosses `R`, **when a strip's dwell expires** (a per-strip countdown
`dwell: u16` set to `R_b` at the settle and decremented at the drain; reaching zero sets the
group dirty, N1), on a move, at bind. A scene recall dirties every group at once; the global
budget below serves them over several blocks.

**The step: the target is the full pattern sort** (N2). On a dirty group:

1. *Rank and histogram.* Patterns are read over the ranked slots (the costliest `min(S, 8)`
   slots; `2^8 = 256` counters per group, preallocated `u16`). One pass over the group's movable
   positions (not `Fixed`: padded, pinned, refused this episode, inside its dwell) counts each
   pattern; prefix sums give each pattern's first target position among the movable positions in
   descending pattern order; a second pass assigns every movable strip its `target` (a
   preallocated `u16` per position). `O(span + 2^S)`.
2. *Realise.* Walk positions upward; where the resident strip is not the target's, swap the
   target strip in from its current position (one cycle-walk swap places one strip finally). A
   refused swap (automation fit, collapse arming, a staged cell the check will not carry) marks
   that strip `Fixed` for the episode and the target is recomputed on the next block. At most
   `span - 1` swaps realise the permutation.
3. *Budget.* The walk stops at `B_total = W` swaps or `A_total = 4W` attempts per block across
   all groups (round-robin cursor over dirty groups) and resumes next block from where it stopped;
   the group stays dirty until the walk reaches the end with no swap pending.

**Convergence and the bound.** Facts fixed, a group converges in at most
`ceil((span - 1) / B_total)` blocks of its share of the budget; with `G` dirty groups, in at most
`G` times that. At convergence the layout is the sorted order and the per-group bound of the
finish line holds. **Between blocks the cost is bounded, not monotone**: a cycle-walk swap touches
two banks and changes the group's cost by at most `2S`, so a block moves the cost by at most
`2 x W x S` in either direction, for at most `ceil((span - 1)/W)` blocks. A monotone realisation
cannot be promised together with convergence to the sort, because the sorted order is not always
the layout of least cost (the demand sum is an OR per slot, not a function the lexicographic
order minimises); the sort is chosen because its cost is *provably bounded*, and the transient is
a cost of at most a few banks for a few blocks after a toggle, never a bit.

**The move over lane chains (#2142).** A position names a lane chain and a lane, the same in every
row. `swap_bank_lanes(a, b)` checks every row's lanes and chains first
(`BankChain::lane_swap_check`), then trades the two strips row by row (chain state, member ops,
fold entries, source bits, observation rows), then records the move once. Resident pairing and
bank producer rows need no translation: lane `l` of row `j + 1` still gathers lane `l` of row `j`.
The one translated read is a per-node op's producer row (#1899): the op asks the unit that renders
its strip now (`LaneMap::producer_unit`). Only a chain's first row states the packer's idle fact;
a later row's input silence is stated only when its predecessor skipped, so its fact would hide
every strip whose bank mates run, and it costs cost, never bits.

**Homing.** Within the sorted target, strips of one pattern are placed in home order, so when
every strip has one pattern (an all-active episode) the target is the identity map and the fused
epilogue returns; no separate homing rule is needed.

**Residency (M5).** Three rules, each with a reason:

1. Silent class: `R = PACK_IDLE_BLOCKS = 16` (above every smoother ramp, below every effect rest
   bound; #1981 R4).
2. Bypass class: a strip whose settled-bypass pattern changed within the last `R_b = 16` blocks
   is `Running` on the slots that changed (it is demanding and it does not move). A toggle at the
   ramp period never settles for `R_b` blocks, so it never moves. Reason: `R_b = R`, the same
   debounce, so a bypass ride and a fader ride are bounded alike; 43 ms at 48 kHz is below any
   user-visible latency of the saving.
3. Automated bypass: a strip with a stored automation lane on any slot's bypass is **pinned**
   (`Fixed`) for the plan's life: it neither moves nor is counted as packable. Bind-time bit,
   beside the `automated` bit #1981 reads. Reason: its pattern is data-driven and unbounded in
   rate, and pinning costs at most one unskipped bank-slot per automated strip.

**Global budget (M5).** At most `B_total = W` moves and `A_total = 4W` swap attempts per block
across all groups (one bank's state per block, the worst swap cost #1981 D5 accepted; four
attempts per move leave room for refusals without letting attempts scale with the plan), served
round-robin over dirty groups by a cursor that survives blocks. A scene recall that dirties `G`
groups converges in at most `G x ceil((span_g - 1) / W)` blocks, during which every group still
renders exactly (an unsorted bank costs cost, not bits).

### 3.4 Staleness and the un-bypass law: per lane, per slot (B1)

> A lane's slot state is **stale** from the block its bypass settles until the block it is next
> un-bypassed, whatever its bank does in between. Un-bypass from settled bypass performs
> `reset_lane(lane, DiscontinuityKeepParameters)` on that slot, holds dry for the effect's
> warm-up `W` (#2129 D3), then crossfades. A lane never un-bypassed from settled bypass (a ramp
> reversed mid-way) is not stale and takes today's ramp.

Consequences:

- A lane's un-bypass bits are a function of the lane alone: never of whether its bank skipped,
  never of its bank mates, never of which bank it rendered in. That is what makes compile
  regrouping (6b, 7, 8) and render packing (5) class A on sessions that toggle bypass.
- #2129 S1b's skip then needs no bank-level `stale`: a skipped bank-slot's lanes are all stale by
  the rule above; a running bank's bypassed lane is stale too and its wet words are discarded.
- The hold counts as bypassed for observation (#2129 D5's bypass value) until the crossfade's
  first frame with `m(k) > 0`.
- Against today: E5/#1341's un-bypass in a running bank crossfaded from warm wet state; now it
  resets and holds. The transition's bits move. Root accepted this (B1) under the owner's
  2026-10-26 decision that bypass is real passthrough; recorded as class B for that transition
  only, confirmed by the owner in section 8.
- The stale bit and the hold counter are per-lane state: they travel in `swap_lanes` (every
  slot) and in the plan carry, keyed by strip (#2129, M1). The carry snapshots a stale lane
  without resetting it; the successor resets it at its un-bypass.

### 3.5 Worst-case cost per block

- Facts: `O(units)` flag reads, one compare per source; no scan of audio words.
- Clean groups: one load each.
- A full `reset_lane` burst (every strip un-bypassed on every slot in one block): one reset per
  lane per slot, bounded by the plan's total per-lane effect state (C1 at 64 tracks and W=8, on
  the order of 3 MB written once), on the block of the un-bypass and never again until the next
  such scene; exact, so not spread.
- Dirty groups: `O(span + 2^S)` for the histogram and targets once per episode, `O(span)` reads
  per visited block, at most `A_total = 4W` swap attempts and `B_total = W` swaps in all per
  block. One swap moves, per slot, the two lanes' per-lane state
  (`left_bytes + right_bytes` of `StatePayloadSizes`) through the scratch: three `memcpy`s. Bound:
  `3 x W x S_lane` bytes, `S_lane` the largest per-lane chain state in the plan (C1 at W=8: EQ
  5216 bytes per channel, limiter `3 Fs/100 + 39` words per channel, on the order of 200 KB in
  the worst block), once per dirty episode per group, spread by the global budget.
- A displaced folded group pays #1782a's deferred fold: one re-materialised tile per displaced
  home unit, measured +0.5 % all-active native (#1981 attempt 3).
- Allocation: zero, by `bench_support::alloc` counters.

## 4. Correctness and invariants

1. **Placement independence of the bits.** A lane's arithmetic depends on its own input, state,
   parameters and switches only; its un-bypass law is its own (3.4); a bank couples cost, never
   bits (decision 12 M2). Compile placement and render placement are permutations. The one
   cross-lane operation is the fold into a submix or the master, and its order is fixed:
   `route_fold`'s association proof, or the deferred home-order fold when displaced. A
   data-dependent fold order is never an option (#1981 D6). No class-B summation ruling is
   invoked.
2. **PDC and latency.** Latency is per prepared slot and fixed; bypass and the hold preserve it;
   PDC is computed on the graph (`pdc.rs`), not on unit order. Placement changes when a unit runs
   within a block, never the sample alignment of its output. Packing moves state with the lane,
   latency lines and hold counters included.
3. **Sidechains.** A chain with a connected sidechain renders per node (`blocks_banking`,
   `rack/src/lib.rs:79`); its segment's window includes the key's tap edge. A banked sidechain
   port (#96 F9) is slice 9 (#2139).
4. **Observation taps.** Readers keyed by `(unit, lane)` translate through the lane map
   (#1782a). A bypassed or held lane publishes its tap's declared bypass value (#2129 D5).
5. **Automation and D15-2.** The pack step runs after the drains and before the unit loop (#1981
   R1), so a live value drained at block `B` is applied at `B` whichever lane renders the strip;
   the watermark is unchanged. A lane with a staged span moved before the unit loop carries the
   staged cells with its control lane (slice 5 (#2135) gates it). Automated strips only enter banks with
   automation slots; strips with automated bypass are pinned.
6. **Plan swaps.** The carry takes both plans to the identity map first (`restore_home_lanes`),
   carries per home strip, and carries the stale bits and hold counters with the lane's state
   (#2129). A rebuild re-runs the planner; a strip's state travels by strip id, so a different
   placement in the successor is just a different home.
7. **Collapse.** A mono-pool bank collapses only when every lane is symmetric at dispatch; the
   remainder rule of 2.5 puts a mono strip in a stereo bank, where it renders dual; same words.

## 5. Flexibility

The planner never refuses a session shape. Shapes that do not bank, and their fallback:

| shape | today | after | fallback |
|---|---|---|---|
| connected sidechain on a slot | per node | per node until slice 9 (#2139) | per-node path, same bits |
| the delay | per node | per node | per node |
| third-party `cid` | refused at prepare | unchanged | n/a |
| insert remainder below threshold | per node | pass 2, else per node | per node |
| insert order that matches no leader | per node | pass 2 per slot | per node |
| a segment that ends early (a per-node insert after it) | own padded bank | joins by prefix; fusion stops there | round trip, same bits |
| foreign vector width | compile refused | unchanged | n/a |
| console remainder of one strip | padded bank | padded bank (section 8, item 2) | n/a |
| feedback | refused (acyclic) | an explicit positive-latency edge is a strip-DAG edge like any other | n/a |

The per-node path is the oracle the banked path is compared against.

## 6. Evidence plan

Per slice: one test per claim, red first on a named mutant, in the owning crate; a one-time
`to_bits` comparison of the console, C1 and irregular fixtures' rendered output before and after,
**with a bypass-toggling run included** (the verifier's point: a fixture that never toggles cannot
see an un-bypass regression); `bench_support::alloc` counters exact zero on the render path;
descriptive CPU rows, one invocation, one warmup, two rounds.

Rows (slice 0 (#2132)): the C1 shape at 16, 32 and 64 tracks in four states (load, typical by a fixed
seed, all active, sparse) plus a `Toggle` state (one strip's gate toggled every 64 blocks), W=8
native and W=4 V8; one irregular graph. Per row: banks bound, mergeable partial pairs (must be
0 at the finish line), static lane use, BSR executed per block, the per-group ideal, p50 us.

## 7. Slices

All class A except #2131's un-bypass law (class B, root-accepted, owner to confirm). Each is
closable in about half a day; file sets are in the slice specs.

Three steps are existing issues and have no slice file: **#1981** (swap-to-pack on silence,
owner-accepted, merges first), **#2131** (`reset_lane` on the bank trait for every banking effect
and the per-lane un-bypass law; this design's S1a) and **#2129** (the bank-slot skip with
per-lane staleness; the stale bit and hold counter travel in swaps and the carry; S1b, which lands
after #1981 and #2131).

| file | slice | depends on | unblocks | supersedes or amends |
|---|---|---|---|---|
| slice-0 | bank utilisation report and the C1 rows | nothing | the finish-line measurement | none |
| slice-0a | W=4 bank counters on the wasmtime guest | #2132 | W=4 finish-line evidence | none |
| slice-4 | silence as a propagated fact | #1981 | slice 5 (#2135)'s V8 baseline | #1981's gather OR (measurement note) |
| slice-5a | pack groups across split strips: lane chains, one permutation per strip | #1981, #2131, #2132 | #2135 (blocks it) | #1981's `pack_member_eligible` |
| slice-5 | pattern packing: the pattern sort, dwell, pins, global budget | #1981, #2131, #2129 (slice 4 (#2133) recommended) | the dynamic finish line | #1981's predicate; #1782 D5 |
| slice-6a | executor order as a contracted topological order; colouring over it | nothing | slice 6b (#2136) | #96 F12 restated; #169 argument re-made |
| slice-6b | ASAP/ALAP windows in the planner | #2131, slice 6a (#2134) | the static finish line | #96 F12; #966; #1098 H2 |
| slice-7 | cost-weighted mono/stereo remainders | #2131, slice 6b (#2136) | nothing | #971, #1098 amendment 10 |
| slice-8 | insert leftover pass | #2131, slice 6b (#2136) | nothing | #163 phase 1 placement |
| slice-9 | banked sidechain port (after the finish line) | slice 6b (#2136) | sidechained inserts bank | #96 F9 |

Filing and merge order: slice-0 (#2132), slice-0a, #1981, #2131, #2129, slice-4 (#2133),
slice-6a (#2134), **slice-5a**, slice-5 (#2135), slice-6b (#2136), slice-7 (#2137), slice-8
(#2138), slice-9 (#2139). #2131, slice-4 and slice-6a are mutually independent (effects and
effect-contract; source, capi, host-web and graph flags; graph program and runtime order) and may
run in parallel worktrees.

### The finish line

**Static (slices 6a, 6b, 7, 8).** The plan has **no mergeable pair**: no two partial banks of one
key class (same rack, same leader program or a subsequence of it, same pool class after the
remainder rule, same fold target) whose **windows intersect** and whose union fits `W`, where a
bank's window is the intersection over its members of `[asap'_m, ALAP_m]` after placement (so the
criterion never reports a pair the sweep could not have merged), **and whose merge still binds
every slot either bank bound** under #96 F7 (every member lane runs it; the binder's predicate
`every_member_runs`). This is checkable on the plan in
`O(banks^2)` and reported as a count; the finish line is zero on every slice-0 row.

*Definition change, owner decision 2026-10-27 (#2138):* the binding clause is new. The old count
took the subsequence test alone and so counted pairs F7 forbids: on the irregular graph at W=4 a
`[eq, deess, shape]` group whose lanes run only `eq` beside the leftover `[shape]` bank of two
lanes, whose merge would unbind the `eq` bank and bind nothing. It over-counted; no plan and no
bit changes with the definition. Lane use is reported beside it. Recomputed numbers for C1 (aux A's delay splits its
chain: its pre segment joins the submix pre bank by prefix, its drive banks alone; the master's
limiter splits the master, which banks alone in any case):

| tracks | W | BSR today | BSR after | lane use today | lane use after |
|---|---|---|---|---|---|
| 16 | 8 | 21 | 17 | 55 % | 68 % |
| 16 | 4 | 29 | 29 | 79 % | 79 % |
| 32 | 8 | 29 | 25 | 67 % | 78 % |
| 32 | 4 | 45 | 45 | 87 % | 87 % |
| 64 | 8 | 45 | 41 | 79 % | 87 % |
| 64 | 4 | 77 | 77 | 92 % | 92 % |

Submix BSR after: pre `{4 groups, aux B, aux A pre}` 6 lanes = 1 bank at W=8 (3 BSR), 2 banks at
W=4 (6 BSR); drive `{4 groups, aux B}` 1 bank at W=8, 2 at W=4; aux A drive 1; master 4. So 9 at
W=8 (13 today) and 13 at W=4 (13 today). **At W=4 the window merge gains nothing on C1**: six
submixes need two banks either way. The audit's "-17 % at W=4/16" was wrong; it is 0 %. W=8/16
is -19 % of BSR, not -24 %; W=8/64 is -9 %, not -11 %. If aux A's pre segment cannot join (the
round-trip trade-off of 2.1 measures the wrong way), it is a padded bank of its own: 20 BSR at
W=8/16 (58 %). The browser's gain on C1 comes from the render-time slices; the window merge is a
W=8 gain on C1 and a W=4 gain on graphs whose submix counts do not align to `W` (three groups and
one aux: two banks today, one after).

**Dynamic (slices 1, 3, 5).** Per pack group `g` with banks `n_g`, slots ranked `k = 1..S` and
`A_{g,k}` the strips of `g` with slot `k` enabled and not silent-at-rest, on a block where `g` is
clean:

`BSR_g <= sum over k of min(n_g, ceil(A_{g,k} / W) + 2^k - 2 + e_g)`

where `e_g` is the number of `Fixed` strips of `g` (pinned, refused, or inside their dwell; each
can keep at most one extra bank per slot alive). Derivation, for the **sorted** layout (which the
step converges to): the movable positions are ordered by pattern descending with slot 1 most
significant, so slot `k`'s enabled strips form at most `r = 2^(k-1)` runs, one per combination of
the higher-ranked bits, and each run sits at the *start* of its block. The first run starts at
position 0, so it covers exactly `ceil(a_1/W)` banks; every later run may start and end mid-bank,
covering at most `ceil(a_i/W) + 1`. Summing, `sum ceil(a_i/W) <= ceil(A/W) + r - 1`, plus `r - 1`
boundaries: `ceil(A/W) + 2r - 2 = ceil(A/W) + 2^k - 2`. Check against the review's W=4, k=2
counterexample (pattern `11` at position 0, `10` at 1-6, `01` at 7-8; slot 2 enabled on 0 and 7-8,
`A = 3`, banks 0, 1, 2 touched): the bound gives `1 + 2 = 3`, equal to the actual 3; the earlier
`2^(k-1) - 1` form gave 2 and was wrong. For `k = 1` the bound is `ceil(A/W)` exactly. On C1's four
slots the lowest-ranked slot can waste at most fourteen banks, which `min(n_g, ...)` caps at the
group's size; on the 16-track row that cap is 2 (W=8) or 4 (W=4), so the bound is tight there.
The finish line asserts the bound per group on every slice-0 row, measured at convergence (a
block where every group is clean), and reports the achieved ratio
`BSR_g / sum ceil(A_{g,k}/W)`. Current: the load state executes every BSR (0 % skip); a typical
mix skips `(1-p)^W` per bank-slot (audit 4).

**No regression.** All-active within 2 % of the slice-0 baseline on both widths; `Toggle` row
recorded; allocations exactly zero.

## 8. Decisions the owner must make

1. **The un-bypass law (B1).** Every un-bypass from settled bypass resets the lane's slot state
   and holds dry for the effect's warm-up, in every bank. This changes the un-bypass transition's
   bits against E5/#1341 (crossfade from warm wet state). Root accepted it on 2026-10-26 as the
   consequence of the owner's bypass decision; it is recorded as class B for that transition and
   needs the owner's confirmation in #2131.
2. **A lone console strip (the master) as a padded bank** costs 4 BSR at 1/W use; per node is
   about 2x cheaper at W=8 and even at W=4 (H5). Decision 12 says console slots always bank.
   Recommendation: no change; the browser is W=4.
3. **No class-B summation change is proposed.** The fold order stays canonical.
4. **#2129's open points** (D3's reading of D15-2; D5 taps publish the bypass value) stand as
   root recorded them; this design relies on D5 and D6.
