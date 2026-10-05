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
  (`hosts/host-web/src/lib.rs:2100-2101`). *Fall back to the transition when a warm successor is
  not ready by its deadline* (#1358) D2 grows that default by the warm headroom.
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
    outcome* (#1360) builds it from `P_MAX_SAMPLES` (written by *Fall back to the transition when
    a warm successor is not ready by its deadline*, #1358) and `PRIME_BYTES_MAX` (written by
    #1360), both derived by *Record the swap block's cost on the 64-track console* (#1286) D3. No
    formula is restated here. Gates set it directly.
  - `pub const fn stall_ring_frames(sample_rate_hz: u32, quantum_frames: u32) -> u32`: the ring
    that hides one producer stall, with exactly today's body of `default_source_ring_frames`
    (`:65-77`). `default_source_ring_frames` returns it unchanged in this slice; #1358 D2 later
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
     the duck's end to the fire. A restarted submix's `input` tap keeps the consumer in C only if
     that submix's `Input` stage arrives at exactly `a(n) + P`; step 6 checks it (with `P = 0` in
     step 4).
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
       through a sidechain, that submix's `Input` stage must arrive at exactly `a(n) + P`. If it
       arrives at `a(n) + g` with `g ≠ P`, each carried line into that stage and the sidechain
       line out of it change length by `P - g` samples, and #1283 D4's head-aligned copy breaks
       the detector's input at S. The consuming strip joins R (its nodes before the fader and its
       fader). Go to step 2.
     - **A late node.** Take a late C node `n` that no other late C node feeds. If its late input
       comes through an R or N strip whose `Input` stage arrives later than its floor because
       carried feeders reach it at their lead, restart those feeders whole, and, through every
       feeder that is a submix, that submix's own carried feeders (recursively): that `Input`
       stage then falls back to `a(n)`. Otherwise restart `n`'s own strip (track or submix)
       whole. Every node of a restarted strip joins R and the strip joins `restart_whole`
       (restarted and ducked as #1324 D2 states for such a strip). Go to step 2.
     - **Misaligned.** Return `Unavailable(Misaligned)` only when every strip that reaches the
       output would restart, or when `n` is the output and no feeder can be restarted. That is
       the transition's audio. So a latent insert added on a submix ("a limiter on a bus")
       restarts the submix's carried feeders and stays warm; every path it does not restart,
       the output included, stays exact.

     Each pass adds a strip, so the loop ends within the strip count.
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
2. `crates/host-core/tests/warm_successor.rs` (new) with gates 1-7.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs` (re-exports only)
- `crates/host-core/tests/warm_successor.rs` (new)
- `.github/ISSUE_SPECS/1354-prepare-a-warm-successor-whose-carried-nodes-lead-the-predecessor-by-p.md`

## Non-goals

- No snapshot, no copy at a block and no return path. There is no copy mode.
- No readiness check, no prime and no adoption (#1320, #1355). No `Primed` publication (#1311,
  #1360).
- No source-read offset change (#1396, #1355). No duck or fade (#1324, #1397).
- No control-plane or C ABI wiring (#1360).

## Objective gates

All at 48 kHz, quantum 128, unless stated. `L` is the true-peak limiter's latency at 48 kHz.
Unless a gate states its own, `WarmConfig` is `{ p_max: 4096, prime_bytes_max: u64::MAX }` and
every plan is prepared with an explicit `source_ring_frames = stall_ring_frames(48_000, 128) +
4096 + 128`, so no gate depends on `default_source_ring_frames` before or after #1358 D2.

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
2. **A growth confined to a restarted bus, and Misaligned.** Track X has a limiter insert and
   routes to the output; track T routes to submix S, which routes to the output. The edit adds a
   post-fader send from X into S.
   - With S unchanged otherwise, S's growth does not reach the output: X's new send makes S's
     `Input` stage late, S restarts whole, and `Δ` over C is 0. `warm_lead` returns
     `Warm(WarmLead { lead_samples: 0, restart_whole: [S] })`, not a transition.
   - With a limiter also on S in A, S's growth reaches the output. The first pass restarts S whole;
     the output is then late through S, whose `Input` stage X and T hold late, so X and T restart
     whole. Every strip that reaches the output would restart, so `warm_lead` returns
     `Unavailable(Misaligned)`.
3. **LeadBound at the exact thresholds.** From a predecessor whose offset is `384`
   (`test_only_set_source_read_offset`, #1396), gate 1's first edit:
   - `p_max = 384 + P` returns `Warm`; `p_max = 384 + P - 1` returns `Unavailable(LeadBound)`.
   - With `stall = stall_ring_frames(48_000, 128)`: `source_ring_frames = stall + P + 128` returns
     `Warm`; `stall + P` (one quantum less) returns `Unavailable(LeadBound)`; `stall` (no
     headroom) returns `Unavailable(LeadBound)`.
   - `stall_ring_frames(48_000, 128) == 5120` and `stall_ring_frames(44_100, 128) == 4736`
     (today's default body), and they stay so after #1358 D2 grows the default.
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
   - `g < P`: T1 already has a true-peak limiter in A, so the edit makes it two. B's `Input` stage
     arrives at `a + 486`. `warm_lead` returns `Warm(WarmLead { lead_samples: 512, restart_whole:
     [] })`, and preparing it gives `restarted_strips() == [B, T1, T3]`.
7. **A limiter on a bus.** In A, tracks T1 and T2 route to submix B, B and T3 route to the output,
   and nothing has latency. The edit adds a true-peak limiter insert to B. `warm_lead` returns
   `Warm(WarmLead { lead_samples: ceil(L / 128) * 128, restart_whole: [T1, T2] })`. Preparing it
   gives `restarted_strips() == [B, T1, T2]`, every C node (B's nodes after its fader, T3's nodes
   and the output) at exactly `a(n) + P`, and T3's claim line exactly `P` samples.
8. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a lead floor on every surviving node (round-4 B1) moves track 2 and the output off
  `a + P`; a lead not rounded to whole quanta gives the wrong `P`. Red.
- Gate 2: `Δ` taken over R as well gives `P > 0` for the bus case; no C1 check prepares a submix
  that arrives `L` late; a loop that returns `Warm` while the output is still late prepares a
  misaligned output. Red.
- Gate 3: a `<` for `<=`, or a ring headroom check that ignores the prime's extra block or a
  zero-headroom ring, fails the exact cases. A baseline that grows with the default (so after
  #1358 D2 every default ring reads as zero headroom) fails the exact `stall_ring_frames` values.
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
  output's path restarts a strip that stays exact. Red.

## Dependencies

- *Grow latency during playback by adopting a primed warm successor* (#1287), first slice: lead
  floors as claim lines.
- *Keep every node's latency from dropping during playback* (#1285): the floors `warm_lead`
  compiles with.
- *Carry fader, mute and pan ramps across a plan swap* (#1277): `restarted_strips()`.
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324): the restart set R.
- *Give a plan a source-read clock that leads its render clock* (#1396): `ΣP` in the inventory.
- *Carry source-claim lines across a plan swap and fill a grown line for a prime* (#1402): D4 keeps
  a `restart_whole` strip's claim lines carried (#1402 D1).
