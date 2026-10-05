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

- The successor entry points take a `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641-647`). `PlanStateInventory` (`:558-564`) is the
  predecessor's control-side record; every successor is prepared from it
  (`prepare_host_runtime_successor`, `:909`).
- Each source ring's capacity is the host's `HostPrepareCaps::source_ring_frames` (`:106`), set in
  the ring config at `:1207-1212`. A carried ring keeps its inventory row's configuration.
  `default_source_ring_frames` (`:65-77`) is today's default: 100 ms (`:57`) plus two quanta. The
  C ABI uses it when the host passes `source_ring_frames == 0`
  (`crates/capi/src/runtime/compile.rs:705-707`), and so does the browser
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
  - It also gains a constructor, `SuccessorBase::new(inventory, committed)`, with `warm: None`.
    Every literal construction site moves to it in this change:
    `crates/host-core/tests/successor_swap.rs:988`,
    `crates/host-core/tests/support/successor.rs:101`,
    `crates/capi/tests/resource_lifecycle.rs:1591`, and the control plane's `prepare_runtime` call
    (`crates/capi/src/runtime/control.rs:910` today, under `crates/control-plane/src/` after
    #1309 D2). A warm successor sets its field on the value `new` returns
    (`SuccessorBase { warm: Some(&lead), ..SuccessorBase::new(inventory, committed) }`). A later
    field (#1397's `forced_restart`) then changes `new` and no construction site.
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
   D4's move of every `SuccessorBase` literal site to `SuccessorBase::new`.
2. `crates/host-core/tests/warm_successor.rs` (new) with gates 1-9.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs` (re-exports only)
- `crates/host-core/tests/warm_successor.rs` (new)
- The `SuccessorBase` construction sites only (D4): `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`, `crates/capi/tests/resource_lifecycle.rs`, and
  the control plane's `prepare_runtime` call in `crates/control-plane/src/` (package
  `control-plane`, created by #1309)
- `.github/ISSUE_SPECS/1354-prepare-a-warm-successor-whose-carried-nodes-lead-the-predecessor-by-p.md`

## Non-goals

- No snapshot, no copy at a block and no return path. There is no copy mode.
- No readiness check, no prime and no adoption (#1320, #1355). No `Primed` publication (#1311,
  #1360).
- No source-read offset change (#1396, #1355). No duck or fade (#1324, #1397).
- No control-plane or C ABI wiring (#1360), beyond D4's move of one construction site.

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
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
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
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): D4 edits the
  control plane's `SuccessorBase` construction site in the crate it creates.
- *Carry source-claim lines across a plan swap and fill a grown line for a prime* (#1402): D4 keeps
  a `restart_whole` strip's claim lines carried (#1402 D1).
