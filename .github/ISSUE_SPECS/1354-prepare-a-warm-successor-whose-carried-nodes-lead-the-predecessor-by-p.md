# Prepare a warm successor whose carried nodes lead the predecessor by P

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment)).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): the lemma's
setup, C1 with its strip-restart iteration, C2's sidechain rule and C3. Code anchors verified on
`main` at `6fb211594`.

## Product outcome

For a structural edit that grows a node's arrival, host-core answers, off the render thread and
before the commit, exactly one of: an ordinary rebuild, a warm successor whose carried nodes all
arrive at exactly their predecessor arrival plus `P`, or a typed `WarmUnavailable` reason that sends
the edit to the transition. A valid edit is never refused for it. There is no snapshot, no copy at
a block and no return path: render adopts the warm successor in move mode (#1355).

## Context

- The successor entry points take a `SuccessorBase { inventory, committed, revision }`
  (`crates/host-core/src/prepare.rs:641-647`). `PlanStateInventory` (`:558-564`) is the
  predecessor's control-side record; every successor is prepared from it
  (`prepare_host_runtime_successor`, `:909`).
- Each source ring's capacity is the host's `HostPrepareCaps::source_ring_frames` (`:106`), set in
  the ring config at `:1207-1212`. A carried ring keeps its inventory row's configuration.
  `default_source_ring_frames` (`:65-77`) is today's default: 100 ms (`:57`) plus two quanta. The
  C ABI uses it when the host passes `source_ring_frames == 0`
  (`crates/control-plane/src/compile.rs:709-711`), and so does the browser
  (`hosts/host-web/src/lib.rs:2100-2101`). *Grow the default source ring by the warm-prime
  headroom* (#1406) D2 grows that default by the warm headroom.
- *Keep every node's latency from dropping during playback* (#1285) D1-D2: the compile request's
  floor map, and the inventory's recorded floored arrival `a(n)` per node.
- *Carry fader, mute and pan ramps across a plan swap* (#1277) D6: `PreparedHost::restarted_strips()`.
  *Duck-swap a strip whose state cannot continue across a plan swap* (#1324) D2 restarts every node
  before the fader, and the fader, of such a strip.
- #1287's first slice: lead floors become source-claim lines (#1287 D1). *Carry source-claim
  lines across a plan swap and fill a grown line for a prime* (#1402) carries them and fills them
  by L2 and L3.
- *Give a plan a source-read clock that leads its render clock* (#1396) D2: the inventory records
  the predecessor's source-read offset, `ΣP`.
- `EffectSidechain` edges (`crates/graph-compiler/src/compile.rs:393-416`) read a strip's tap
  (`crates/graph-compiler/src/ids.rs:187-220`) and have no gain lane.

## Decisions frozen for this slice

- **D1. Types.** In `crates/host-core/src/prepare.rs`:
  - `WarmUnavailable { Misaligned, LeadBound, PrimeBudget }`, a typed value distinct from
    `PrepareDiagnostics`.
  - `WarmLead { lead_samples: u64, restart_whole: Box<[String]> }` (strip IDs, sorted), with
    `prime_bytes()`.
  - `WarmDecision { Ordinary, Warm(WarmLead), Unavailable(WarmUnavailable) }`.
  - `WarmConfig { p_max: u64, prime_bytes_max: u64 }`. It has no default: every caller passes
    both bounds. *Check the warm-successor deadline in miso_engine_v1_service and report its
    outcome* (#1360) builds it from `p_max_samples(fs, q)` (written by #1406 D1) and
    `PRIME_BYTES_MAX` (written by #1360), both derived by *Record the swap block's cost on the
    64-track console* (#1286) D3. No formula is restated here. Gates set it directly.
  - `pub const fn stall_ring_frames(sample_rate_hz: u32, quantum_frames: u32) -> u32`: the ring
    that hides one producer stall, with exactly today's body of `default_source_ring_frames`
    (`:65-77`). `default_source_ring_frames` returns it unchanged in this slice; #1406 D2 later
    adds the warm headroom on top of it. It is the fixed baseline D2 step 7 measures headroom
    against, so the check does not move when the default grows.
- **D2. `warm_lead`, over C only.** `host_core::warm_lead(compiled, caps, base, &WarmConfig) ->
  Result<WarmDecision, PrepareDiagnostics>` is the only place `P` is computed; the control plane
  calls it at classification (*Classify a latency-growth edit and publish its warm successor from
  the control plane*, #1403 D2). It runs, in order:
  1. R is the nodes of the strips the ordinary preparation's carry join restarts
     (`restarted_strips()`, every node before the fader and the fader, #1324 D2). N is the nodes
     not in the predecessor. C is every other node.
  2. **C2's sidechain rule**, the same as #1324 D1's. An `EffectSidechain` edge from an R strip's
     `post_input`, `insert_send`, `insert_return` or `pre_fader` tap into a C node puts the
     consuming strip in R (its nodes before the fader and its fader); repeat until no strip joins.
     A track's `input` tap and a `post_fader` or `post_pan` tap keep the consumer in C: #1287's L3
     fills a track's `input`-tap line, and a `post_fader` or `post_pan` tap is exact `+0.0` from
     the duck's end to the fire. A restarted submix's `input` tap keeps the consumer in C if every
     line into that submix's `Input` stage comes from a strip in R or an added strip (those lines
     are exact `+0.0` from the duck's end to the fire, and #1324 D4's `C` counts the sidechain
     line plus the longest line into the stage), and otherwise only if the stage arrives
     at exactly `a(n) + P`; step 6 checks it (with `P = 0` in step 4).
  3. **`Δ` over C.** Compile with #1285's floors and no lead. `Δ` is the largest
     `arrival(n) - a(n)` over C. Nodes in R or N do not count, so a growth that the restarts
     confine to R is not a growth.
  4. If `Δ <= 0`, every C node arrives at exactly `a(n)`, so C1 holds with `P = 0`. Apply step 6's
     submix `input`-tap check with `P = 0`; if a strip joins R, go to step 2. Then return
     `Ordinary` when no strip was restarted whole, else
     `Warm(WarmLead { lead_samples: 0, restart_whole })`, an ordinary rebuild that duck-swaps those
     strips.
  5. `P = q · ceil(Δ / q)`. Compile with floors `a(n) + P` on C, `a(n)` on R, none on N.
  6. **C1.** Every C node must arrive at exactly `a(n) + P`.
     - **Submix `input` tap (M1).** For each C node that reads a restarted submix's `input` tap
       through a sidechain, unless step 2's exemption holds (every line into the stage comes from
       a strip in R or an added strip), that submix's `Input` stage must arrive at exactly
       `a(n) + P`. If it arrives at `a(n) + g` with `g ≠ P`, each carried line into that stage
       and the sidechain line out of it change length by `P - g` samples, and #1283 D4's head-aligned copy breaks
       the detector's input at S. The consuming strip joins R (its nodes before the fader and its
       fader). Go to step 2.
     - **A late node.** Take the first late C node `n` in W's schedule order; no other late C
       node feeds it. Walk back from each input of `n` that arrives after `a(n) + P`, through R
       and N nodes only, along every edge kind: a strip's chain, a route (carried or added) and an
       `EffectSidechain` edge (PDC takes its maximum over sidechain edges too,
       `crates/graph-compiler/src/pdc.rs:61-65`). From each R or N node the walk follows the inputs
       that hold its arrival (an input whose arrival plus latency equals it). Each C node the walk
       reaches that way holds the late path at its lead; a route node counts as its source strip.
       Restart whole the strip of every such C node, and no other strip: a restarted submix's
       other feeders stay carried, and a feeder that really holds the stage late is found by
       the next pass's walk. Every node of a restarted strip joins R and the strip joins
       `restart_whole` (restarted and ducked as #1324 D2 states for such a strip).
     - **Invariant: the walk always reaches a C node.** If it reaches none (whether `n` belongs
       to a strip or is the output, a C node of no strip), that is an invariant error: a debug
       assertion fires and `warm_lead` returns a `PrepareDiagnostics` invariant error. It never
       restarts `n`'s own strip and never guesses. Proof sketch: steps 3 and 5 compile one graph
       with the same R floors and no N floors, and arrivals are monotone in the floors. A path
       into a late C node `n` whose tight inputs reach no C node has the same value in both
       compiles, so it is at most `a(n) + Δ <= a(n) + P`, which contradicts `n` being late. This
       needs every carried node to keep its latency, which holds because a latency change is a
       prepared difference, so that node's strip is in R. No gate can reach this branch.
     - **Misaligned.** After a pass, return `Unavailable(Misaligned)` if every predecessor strip
       that reaches the output is in `restart_whole`; that is the transition's audio. (An added
       strip never joins `restart_whole`, so it is not counted.) Otherwise go to step 2.

     So a latent insert on a submix ("a limiter on a bus") restarts the submix's carried
     feeders (the walk reaches them through the submix's restarted nodes); a parallel bus with a
     latent insert, fed from a bus's `post_pan` tap, restarts that bus and only those of its
     feeders that hold it late (gate 9); a kick that keys a bass compressor, when the edit adds a
     limiter to the bass, restarts the kick (its key holds the bass's chain late); and every path
     neither restarts, the output included, stays exact. Each pass adds a strip to R (the
     submix `input`-tap rule, step 4 or 6) or to `restart_whole` (the walk), no strip leaves
     either, and a strip can enter each once, so the loop ends within twice the strip count. The
     result is deterministic on W's schedule: the first-node rule reads W's one schedule order.
     `P`, the ducked set and the decision kind do not depend on the schedule; `restart_whole`
     can differ under another schedule (the round-8 review's model: 11 of 17,786 random
     sessions, for example `[B1, T0, T1]` against `[T0, T1]` where B1's only carried node is a
     dead-end `post_pan` stage). In the round-7 review's structural model (design evidence, not
     a gate; 4,650 random sessions, with removals and added latent buses) this walk terminated,
     met C1, the sidechain closure and the submix rule, and never reached the invariant
     branch.

     **Known behaviour: the walk is exact but not minimal.** In that model about 5% of random
     sessions duck 1-4 more strips than the smallest restart set that meets C1 (a brute-force
     search). One cause: a partly restarted strip's dead-end `post_pan` stage counts in `Δ`. The
     result is still exact, the model found no `Misaligned` result where a warm answer existed,
     and the extra strips duck and fade in as #1324 states. The walk does not search for the
     minimum set; this is recorded design behaviour, not a gap in C1.
  7. **C3, lead.** Return `Unavailable(LeadBound)` if `ΣP + P > p_max`, or if any carried ring's
     capacity above `stall_ring_frames(fs, q)` for the session's rate and quantum is below
     `P + q` frames. A ring without that headroom (the round-4 "hold cap 0" case) cannot hold the
     prime's `k + 1` blocks without spending the producer's stall tolerance, so warm preparation
     refuses it whole; no ring is ever partly primed.
  8. **C3, prime.** Return `Unavailable(PrimeBudget)` if `prime_bytes > prime_bytes_max`.
  9. Otherwise return `Warm(WarmLead { lead_samples: P, restart_whole })`.
- **D3. Prime byte count.** `prime_bytes` is what render writes and reads in the adoption block:
  `4 · (Σ k · q · channels(s)` over the carried sources `s` the prime replays,
  `+ Σ 2 · λ'` over every claim line and every L3 line W fills at S`)`, with `k = P / q` and `λ'`
  the line's length in W. It is exact, not an estimate: #1286 measures the adoption block at
  `ΣP = P_MAX` against it.
- **D4. Preparation.** `SuccessorBase` gains `warm: Option<&WarmLead>`.
  - `SuccessorBase::new` (*Reset latency floors at a host-declared discontinuity*, #1323 D3) sets
    it to `None`, and no construction site changes. A warm successor sets its field on the value
    `new` returns (`SuccessorBase { warm: Some(&lead), ..SuccessorBase::new(inventory,
    committed, revision) }`).
  - `SuccessorBase::new` takes three arguments, `(inventory, committed, revision)`: `revision` is
    the `r` the successor commits as (#1277 D10; #1323 D3). Root ruling, 2026-10-12.
  - With `Some`, preparation compiles with D2's floors, marks every node of each `restart_whole`
    strip not carried (its source-claim lines still carry, #1402 D1), and adds those strips to
    `restarted_strips()`, so #1324 ducks them. It installs the carry program in move mode.
  - It re-checks C1. A mismatch is a `PrepareDiagnostics` invariant error, since `warm_lead` has
    checked it.
  - It refuses a `lead_samples` that is not a multiple of the quantum with a typed error.
  - The prepared host exposes `lead_samples()` and `prime_bytes()`. The warm successor's
    source-read offset `O + P` is #1355's, on #1396 D2's rule.
- **D5. Acked-batch question.** `warm_lead` runs before the commit and changes no queue. A
  `WarmUnavailable` is a value: the edit still commits and takes the transition. An ack can never
  precede a drop.

## Deliverables

1. D1-D4 in `crates/host-core/src/prepare.rs`, re-exported from `crates/host-core/src/lib.rs`.
2. `crates/host-core/tests/warm_successor.rs` (new) with gates 1-9.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs` (re-exports only)
- `crates/host-core/tests/warm_successor.rs` (new)
- `.github/ISSUE_SPECS/1354-prepare-a-warm-successor-whose-carried-nodes-lead-the-predecessor-by-p.md`

## Non-goals

- No snapshot, no copy at a block and no return path. There is no copy mode.
- No readiness check, no prime and no adoption (#1320, #1355). No `Primed` publication (#1311,
  #1360).
- No source-read offset change (#1396, #1355). No duck or fade (#1324, #1397).
- No control-plane or C ABI wiring (#1360), and no change to a `SuccessorBase` construction site
  (#1323 D3 moved them all to `SuccessorBase::new`).

## Objective gates

All at 48 kHz, quantum 128, unless stated. `L` is the true-peak limiter's latency at 48 kHz.
Unless a gate states its own, `WarmConfig` is `{ p_max: 4096, prime_bytes_max: u64::MAX }` and
every plan is prepared with an explicit `source_ring_frames = stall_ring_frames(48_000, 128) +
4096 + 128`, so no gate depends on `default_source_ring_frames` before or after #1406 D2.

1. **Lead over C.**
   - Two audible tracks into the output with no latency; the edit adds a limiter insert to track 1
     (the shape of #1397 gate 1). `warm_lead` returns `Warm` with `lead_samples = ceil(L / 128) *
     128` and empty `restart_whole`. Preparing it gives every C node at exactly `a(n) + P`,
     `restarted_strips() == [track 1]`, and `lead_samples()` equal to it.
   - An added muted limiter track gives the same lead with nothing restarted.
   - An added muted limiter track routed into a submix whose arrival grows while the output's
     does not gives the submix's growth, rounded up to whole quanta, with nothing restarted.
   - An edit that grows nothing returns `Ordinary`.
   - Preparation with a `lead_samples` one sample off a multiple of 128 returns the typed error.
2. **A send into a bus from a latent track.** Track X has a limiter insert and routes to the
   output; track T routes to submix S, which routes to the output. The edit adds a post-fader send
   from X into S.
   - With S unchanged otherwise: S's `Input` stage is the first late node, held late through the
     added send by X's carried fader at its lead, so X restarts whole. `warm_lead` returns
     `Warm(WarmLead { lead_samples: 512, restart_whole: [X] })`; preparing it gives S, T and the
     output at exactly `a(n) + 512`.
   - With a limiter also on S in A: the same, `Warm(WarmLead { lead_samples: 512, restart_whole:
     [X] })`, with S, T and the output at exactly `a(n) + 512`.
3. **LeadBound at the exact thresholds.** From a predecessor whose offset is `384`
   (`test_only_set_source_read_offset`, #1396), gate 1's first edit:
   - `p_max = 384 + P` returns `Warm`; `p_max = 384 + P - 1` returns `Unavailable(LeadBound)`.
   - With `stall = stall_ring_frames(48_000, 128)`: `source_ring_frames = stall + P + 128` returns
     `Warm`; `stall + P` (one quantum less) returns `Unavailable(LeadBound)`; `stall` (no
     headroom) returns `Unavailable(LeadBound)`.
   - `stall_ring_frames(48_000, 128) == 5120` and `stall_ring_frames(44_100, 128) == 4736`
     (today's default body), and they stay so after #1406 D2 grows the default.
4. **PrimeBudget at the exact threshold.** Gate 1's first edit reports `prime_bytes()` equal to
   D3 summed over its compiled lines and carried sources. `prime_bytes_max` at that value returns
   `Warm`; one byte less returns `Unavailable(PrimeBudget)`.
5. **Sidechain isolation.** Gate 1's first edit, plus a compressor on track 2 sidechained from
   track 1. From track 1's `pre_fader` tap, `warm_lead`'s preparation restarts track 2 as well
   (`restarted_strips() == [track 1, track 2]`). From track 1's `input` tap, and from its
   `post_fader` tap, track 2 stays carried.
6. **Sidechain from a restarted submix's `input` tap (M1).** Tracks T1 and T2 route to submix B;
   B and T3 route to the output. T3 holds a compressor whose routed sidechain reads B's `input`
   tap. The edit adds a true-peak limiter insert to T1 and a compressor insert (latency 0,
   `crates/compressor/src/lib.rs:295`) to B, so T1 and B restart. `P = 512` in both cases.
   - `g = P`: T1 has no insert in A. B's `Input` stage arrives at `a + 512`, held by carried T2.
     `warm_lead` returns `Warm(WarmLead { lead_samples: 512, restart_whole: [] })`; preparing it
     gives `restarted_strips() == [B, T1]`, and T3's sidechain line and T2's line into B keep A's
     lengths.
   - `g < P`: T1 already has a true-peak limiter in A, so the edit makes it two, and T1 routes into
     B from its `pre_fader` tap. That route is restarted with T1 (#1324 D2), so B's `Input` stage
     arrives at `a + 486`. (From T1's `post_fader` tap the route is carried, its floor `a + P` holds
     the stage there, and `g = P`.) `warm_lead` returns `Warm(WarmLead { lead_samples: 512,
     restart_whole: [] })`, and preparing it gives `restarted_strips() == [B, T1, T3]`.
7. **A limiter on a bus.** In A, tracks T1 and T2 route to submix B, B and T3 route to the output,
   and nothing has latency. The edit adds a true-peak limiter insert to B. `warm_lead` returns
   `Warm(WarmLead { lead_samples: ceil(L / 128) * 128, restart_whole: [T1, T2] })`. Preparing it
   gives `restarted_strips() == [B, T1, T2]`, every C node (B's nodes after its fader, T3's nodes
   and the output) at exactly `a(n) + P`, and T3's claim line exactly `P` samples. With T3 also
   holding a compressor whose routed sidechain reads B's `input` tap, T3 stays carried (every line
   into B's `Input` stage comes from
   a restarted strip, step 2's exemption): the same result, `restarted_strips() == [B, T1, T2]`.
8. **A carried key or feeder holds a restarted chain late.**
   - Tracks K, Bs and V route to the output, and nothing has latency. Bs holds a compressor whose
     routed sidechain reads a tap of K. The edit adds a true-peak limiter after that compressor on
     Bs. From K's `pre_fader` tap and from its `input` tap, `warm_lead` returns
     `Warm(WarmLead { lead_samples: 512, restart_whole: [K] })`; preparing it gives
     `restarted_strips() == [Bs, K]`, and V's nodes and the output at exactly `a(n) + 512`.
   - Track T4 routes to submix B2, B2 and track T5 route to submix B, and B, T3 and T6 route to the
     output. B2 holds a compressor whose routed sidechain reads T6's `pre_fader` tap. The edit adds
     a true-peak limiter to B. `warm_lead` returns `Warm(WarmLead { lead_samples: 512,
     restart_whole: [B2, T4, T5, T6] })`; preparing it gives `restarted_strips() == [B, B2, T4,
     T5, T6]`, and T3's nodes, B's nodes after its fader and the output at exactly `a(n) + 512`.
9. **A parallel bus on a bus restarts only the feeder that holds it late.** Tracks T0 (no
   insert) and T1 (a true-peak limiter insert) route from their `post_fader` taps to submix Bu,
   and Bu routes to the output. The edit adds submix NB, holding a true-peak limiter insert, fed
   by a route from Bu's `post_pan` tap, and routes NB to the output. `warm_lead` returns
   `Warm(WarmLead { lead_samples: 512, restart_whole: [Bu, T1] })`; preparing it gives
   `restarted_strips() == [Bu, T1]`, and T0's nodes (its route into Bu included) and the output
   at exactly `a(n) + 512`. `Δ = 486` and `P = 512`: T0's route, at its lead floor, holds
   restarted Bu 26 samples later than in A, so NB's path reaches the output at `Δ + 26`, and
   `P − Δ = 26` absorbs that.
10. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a lead floor on every surviving node (round-4 B1) moves track 2 and the output off
  `a + P`; a lead not rounded to whole quanta gives the wrong `P`. Red.
- Gate 2: an iteration that restarts the late node's own strip first restarts S, then X and T,
  and returns `Misaligned` for an edit that restarting X alone keeps exact; a loop that returns
  `Warm` while S or the output is still late prepares a misaligned output. Red.
- Gate 3: a `<` for `<=`, or a ring headroom check that ignores the prime's extra block or a
  zero-headroom ring, fails the exact cases. A baseline that grows with the default (so after
  #1406 D2 every default ring reads as zero headroom) fails the exact `stall_ring_frames` values.
  Red.
- Gate 4: a prime count that leaves out the L3 lines or one lane, or compares with `<`, fails the
  threshold. Red.
- Gate 5: a sidechain from a restarted strip's `pre_fader` tap left carried leaves a detector
  hole in track 2's compressor; restarting it for an `input` or `post_fader` tap (a rule that
  differs from #1324 D1's) restarts a strip that stays exact. Red.
- Gate 6: a C1 that checks only C nodes' arrivals keeps T3 carried when `g < P`, and its detector
  then reads a sidechain line 26 samples longer than A's from S (round-5 M1); a rule that restarts
  every consumer of a submix `input` tap restarts T3 when `g = P`, where it stays exact. Red.
- Gate 7: an iteration that restarts only the late node's own strip returns `Misaligned` for this
  common edit and sends it to a whole-mix duck (round-5 M2); one that also restarts T3 or the
  output's path restarts a strip that stays exact; a submix `input`-tap rule without step 2's
  exemption restarts T3 in the second case, and every strip ducks. Red.
- Gate 8: an iteration that restarts feeders only through a held-late `Input` stage restarts Bs
  whole, walks the lateness to the output and has no rule there (round-6 M1); a walk that ignores
  sidechain edges never finds K or T6. Red.
- Gate 9: a walk that also restarts a restarted submix's carried feeders restarts T0 as well,
  so every predecessor strip is restarted whole and it returns `Misaligned` for an edit with an
  exact warm answer (round-7 M1). Red.

## Dependencies

- *Grow latency during playback by adopting a primed warm successor* (#1287), first slice: lead
  floors as claim lines.
- *Keep every node's latency from dropping during playback* (#1285): the floors `warm_lead`
  compiles with.
- *Carry fader, mute and pan ramps across a plan swap* (#1277): `restarted_strips()`.
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324): the restart set R.
- *Give a plan a source-read clock that leads its render clock* (#1396): `ΣP` in the inventory.
- *Reset latency floors at a host-declared discontinuity* (#1323): `SuccessorBase::new` (D3).
- *Carry source-claim lines across a plan swap and fill a grown line for a prime* (#1402): D4 keeps
  a `restart_whole` strip's claim lines carried (#1402 D1).

## Decision record (1354a, attempt 1, 2026-10-24)

Split (scope R1): 1354a ships D1, D2 steps 1 and 3-9 (the walk over every edge kind, sidechains
included), D3 and D4, and gates 1-4 and 7-9. 1354b ships step 2's sidechain closure and step 6's
submix `input`-tap rule (M1), gates 5, 6 and the second case of gate 7, and the L3 term of D3.
**Known limit until 1354b:** a consumer keyed from a restarted strip's `post_input`,
`insert_send`, `insert_return` or `pre_fader` tap stays carried (the limit 1324a states too).

Root rulings (2026-10-24), each amending the frozen text above:

- **Amends D2 (signature) and scope R4.** `warm_lead(compiled, caps, base, ordinary:
  &PlanStateInventory, config)`. Reason: R is the ordinary successor's `restarted_strips()`, which
  only a full preparation's joins produce, after bind, and the joins depend on the host's entry
  (`resolve_strip_carry` carries a stage only when `before.live == live`). A probe through the plain
  entry would restart every strip of a live predecessor and answer `Misaligned` for every product
  edit. The caller prepares the ordinary successor anyway (the `Ordinary` answer); its
  `restarted_strips()` is R's join part and its `input_floors()` are step 3's arrivals, so
  `warm_lead` compiles once per pass (W, with the lead floors) through `compile_with_builtins`.
  A declared discontinuity answers `Ordinary`.
- **Amends D1 and D4 (`WarmLead`).** `WarmLead` keeps a private field with the ordinary restarted
  set; with `restart_whole` it is the R the floors were derived from. Reason: preparation compiles
  before bind, and the join's restarted set is known only after bind. Preparation builds its floors
  from that field, then re-checks C1 against its own compile and that the join restarts no strip
  outside R; either mismatch is `host.successor.warm_invariant`.
- **Amends #1402 R-A for warm mode.** With a lead, the strip delay join keeps a lane carried, and
  moves nothing, when its declared delay is equal, its source unchanged and its ring grew by
  exactly `P` (from no line, too). Reason: every carried track's claim line grows by `P`, and R-A's
  "a ring-length mismatch restarts" would restart every carried track (gate 1 could not hold). The
  fill and its program entry stay #1355's. **Guard:** no production path requests warm mode before
  #1403: `SuccessorBase::new` sets `warm: None`, every construction site outside host-core's tests
  (`crates/control-plane/src/control.rs`, `hosts/host-web/src/tests.rs`) uses `..SuccessorBase::new`,
  and nothing outside `crates/host-core/tests/warm_successor.rs` sets `warm`.
- **R3 / decision 2, the whole-strip restart filter.** Written once: for each strip of
  `restart_whole` it drops the input-section move, the effect moves (the chain join), the fader move
  (`resolve_strip_carry`), and the compensation lines on its chain before the matrix, into its
  effects' sidechains and on its routes from a tap before the fader; matrix and post-fader lines
  and claim lines still carry; the strip joins `restarted`. For the floors, a `restart_whole` strip
  is in R with every node (its matrix and all its routes), as D2 step 6 says; a join-restarted strip
  is in R with its nodes before the fader, its fader and its routes from a tap before the fader.
- **#1406 D1 is amended:** `p_max_samples` is written here, beside `stall_ring_frames`. Line for
  root: #1406's own record needs the matching amendment (not edited here).
- **Gates use the live-controls entry** (`HostLiveControlRequest::default()`), as the control plane
  prepares: a per-node effect (the true-peak limiter) carries only with a control lane (#1281), so
  through the plain entry the limiter's strip restarts on every edit.
- **Node membership.** N is read from the committed model (strips' seven stages and effects,
  routes, the output), the nodes the compiler emits; the inventory holds no node list.

Evidence (48 kHz, quantum 128, `Backend::current()`):
`crates/host-core/tests/warm_successor.rs`, 10 tests, and one unit test in `prepare.rs`, all
green. Red first on mutants of
`prepare.rs`, one run each:

| mutant | red tests |
|---|---|
| `P = Δ` (no quantum rounding) | gate 1 and 7 others |
| lead floor on R nodes too (round-4 B1) | gate 1 and every gate |
| no quantum check on a lead | `a_lead_off_the_quantum_is_refused` |
| walk restarts the late node's own strip | gates 2, 7, 8, 9 |
| `>=` for `>` on `ΣP + P` | gate 3 |
| ring headroom without the prime's block | gate 3 |
| prime without the claim lines | gate 4 |
| `>=` for `>` on the prime | gate 4 |
| walk ignores sidechain edges | gate 8 |
| walk follows every input, not the holding ones | gate 9 |
| grown claim line restarts its strip (R-A) | gates 1, 2, 4, 7, 8, 9 |
| whole strip's fader carries | gate 2 (R6) |
| whole strip's effects carry | gate 2 (R6) |
| no C1 re-check | `a_lead_that_misses_c1_is_the_invariant_error` |
| no join re-check | `a_join_outside_the_leads_restart_set_is_the_invariant_error` |
| `drops_line` always false | `prepare::tests::the_whole_strip_filter_drops_only_the_pre_fader_lines` |

Verifier fold (attempt 1 PASS, minor findings):
- The compensation-line part of the R3 filter is tested at the unit level
  (`the_whole_strip_filter_drops_only_the_pre_fader_lines`: chain, effect, sidechain and
  pre-fader route lines of a whole-restarted strip drop; its matrix and post-fader lines and other
  strips' lines stay). **Known gap:** the preparation's call of the filter has no integration
  count. `graph::test_only_carry_counts` exposes no delay-move count, and `crates/graph` is outside
  this slice's paths. A follow-up that adds the count there can extend gate 2's R6 assertion.
- The invariant test is split into one fixture per re-check, both reached through public API: a
  lead too short for the edit (`P = 384` with gate 1's restart set) fails only C1; gate 1's lead
  on an edit that also adds a latency-free insert to track 2 keeps C1 and fails only the join.
- The #1272 D3 carry rule is one predicate (`source_carries`, with `source_ring_config`), read by
  the preparation's source loop and by `warm_lead`'s ring check.

Notes for later slices:
- **1354b:** `warm_lead`'s pass bound is `strips + 1`, enough while each pass adds a strip to
  `restart_whole`. When 1354b's M1 rule adds strips to R in a pass, the bound must grow to about
  `2 · strips + 1` (D2: a strip can enter each set once).
- **#1355:** `prime_bytes` counts every source-claim line of W, including an added track's line
  whose source is not carried. #1355 must confirm which lines the fill writes and narrow D3's count
  if the fill skips such lines.
- **1354b (from #1355 attempt 2, 2026-10-25):** the L3 term of D3 is live debt, not empty by
  construction. #1355's fill writes the `Input`-tap sidechain lines (L3) from a restarted track
  into a carried node since that slice, and `prime_bytes_of` still leaves them out, so
  `PrimeBudget` under-counts a prime with such a line. 1354b adds the term; #1355 does not change
  the budget.

Test value (1354a):
- Gate 1: a lead floor on every surviving node (round-4 B1) moves track 2 and the output off
  `a + P`; a lead not rounded to whole quanta gives the wrong `P`; a grown claim line that restarts
  its strip lists track 2 as restarted. The last case: a lead off the quantum prepares a successor
  that misaligns by a sub-block offset.
- Gate 2: an iteration that restarts the late node's own strip first restarts S and does not
  return `[X]`; a restart filter that keeps X's fader or pre-fader owners carried imports a ducked
  fader without a retarget at #1397 (R6).
- Gate 3: a `<` for `<=`, or a ring headroom check that ignores the prime's extra block, fails the
  exact cases; a baseline that grows with the default fails the exact `stall_ring_frames` values.
- Gate 4: a prime count without the claim lines, or compared with `>=`, fails the threshold.
- Gate 7: a walk that restarts only the late node's own strip leaves B's input late and answers
  wrong.
- Gate 8: a walk that ignores sidechain edges never finds K or T6.
- Gate 9: a walk that also follows a restarted submix's inputs that do not hold it late restarts
  T0 as well.
- Invariant, C1: a preparation that trusts a lead's `P` prepares a successor whose output is late.
- Invariant, join: a preparation that trusts a lead's restart set floors a restarted strip at
  `a + P` and never reports it.
- Filter unit test: a filter that keeps a whole-restarted strip's pre-fader lines carries their
  pending audio into a strip that restarts at rest.

## Decision record (1354b, attempt 1, 2026-10-25)

Scope: D2 step 2 and step 6's M1 rule, gates 5, 6 and gate 7's second case, the L3 term of D3,
and the three debts recorded for 1354b. Class A: every change runs only under
`SuccessorBase::warm`; the strip delay join's refactor is behaviour-identical (host-core's suites
pass unchanged).

- **Step 2 and M1 are one rule, #1324 D1's.** Each pass of `warm_lead` with `P > 0` runs
  `close_over_sidechains` (1324b's helper, unchanged) over `R` (the join part, the strips restarted
  whole and the lead's delay restarts) with the lead compile's sidechain lines against A's. On that
  compile a submix `input`-tap line changes length exactly when the stage arrives at `a(n) + g`
  with `g ≠ P`, so the closure's submix rule is M1, with step 2's exemption (`submix_input_is_quiet`,
  the strips restarted whole counted as ducked). A strip that joins goes to `R`'s join part and the
  pass repeats (step 2). With `P = 0` (step 4) the closure runs on the ordinary successor's lines.
  The lead's lines come from the unbound compile's inserted delays (`compiled_sidechain_lines`);
  gate 5 checks that the count over them equals the warm successor's over its bound lines.
- **Debt (a), the re-pointed track.** The lead's strip delay join restarts a track whose claim line
  grows on a source that does not carry; `warm_lead` now runs that join on each pass's compile
  (`strip_delay_restarts`, one rule with `resolve_strip_delay_carry`) and adds its restarts to `R`.
  Row: `the_prime_counts_the_words_its_fill_writes`, last case (prepares, restarts `[t1, t2]`).
- **Debt (c), a consumer of a strip restarted whole.** Closed by the step-2 closure above. The warm
  preparation now restarts every strip of `R` (its join part from `WarmLead`, whatever its own
  join did) and refuses a restart outside `R` after the closure, not before it
  (`host.successor.warm_invariant`). So the "reverse" of the 1354a join re-check (a strip in `R`
  the join leaves carried at floor `a`) cannot occur.
- **Debt (b), D3's L3 term.** `sidechain_fill_words`: each channel of an L3 line of length `λ'`
  takes `min(λ', e + c + P)` words (the predecessor's pending words of the edge, the source
  track's claim pending on that lane, the prime), as #1355's `fill_head` writes them. `warm_lead`
  counts it over the lead compile's fills, preparation over its own (`sidechain_fills`).
- **Pass bound** `2 · strips + 1`: a strip enters the join part and `whole` at most once each.
- **Step 4 (amends D2).** `Ordinary` only when nothing is restarted whole and `R`'s join part is
  the ordinary successor's; a strip that joined on an earlier lead compile makes it `Warm` with
  `lead_samples = 0` (an ordinary rebuild that duck-swaps it). No gate reaches it: it needs a pass
  whose `Δ` falls to 0 only because such a strip joined, and every shape found grows a carried
  node after it too.
- **Gate 5 (amends the gate text).** Gate 1's shape restarts track 1 by the join, which the
  ordinary successor's closure (1324b) already covers. The gate uses gate 2's edit instead (step
  6 restarts X whole) with track Y keyed from X: from `pre_fader`, Y joins `R` and its nodes
  before the fader keep `a(n)`; from `input` and `post_fader`, Y leads by `P`; from `input` the
  prime counts Y's L3 line (`P` words a channel).
- **Gate 6.** In the `g < P` case T3's `Input` stage keeps `a(n)`; its compressor and the stages
  after it wait for B's `Input` stage, as in A, so they are not at `a(n)`.
- **Misaligned** has a gate now (1354a verify): gate 9's last case.
- **Known, unchanged:** `warm_lead` reads `base.committed` without the duck overlay (as 1354a).

Red on `main` (`66b5e0566`) before the change: gate 5 (`y` `Input` at 512, not 0), gate 6 `g < P`
(`t3` `Input` at 512), the re-pointed row (`host.successor.warm_invariant`). Mutants of
`prepare.rs`, one run each (`crates/host-core/tests/warm_successor.rs`):

| mutant | red tests |
|---|---|
| no closure on the lead compile | gates 5, 6, re-pointed row |
| closure does not count `whole` as restarted | gates 5, 7 (second case) |
| lead delay join adds nothing | re-pointed row |
| no L3 words; L3 on one channel | gate 5 |
| preparation omits L3 | gate 5 |
| no `Misaligned` | gate 9 |
| preparation ignores `R`'s join part | gate 6, re-pointed row |
| no restart-outside-`R` check | `a_join_outside_the_leads_restart_set_is_the_invariant_error` |
| shared rule: every submix `input` tap adds; `pre_fader` keeps | gates 6, 7; gate 5 |

Test value (1354b):
- Gate 5: a closure that ignores the strips step 6 restarts whole leaves Y at `a + P` while it
  restarts; a prime that leaves out L3 under-counts `PrimeBudget`.
- Gate 6: a C1 that checks only C nodes keeps T3 at `a + P` with a sidechain line 26 samples
  longer than A's; a rule that restarts every submix `input`-tap consumer restarts T3 when `g = P`.
- Gate 7, second case: an exemption that does not count the strips restarted whole restarts T3.
- Gate 9, last case: a loop with no `Misaligned` answers `Warm` with every reaching strip ducked.
- Re-pointed row: an `R` without the lead's delay restarts refuses a valid warm edit.
