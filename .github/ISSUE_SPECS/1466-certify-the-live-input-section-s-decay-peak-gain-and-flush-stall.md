# Certify the live input section's decay, peak gain and flush stall

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by root order: slice B1 of *Define how node tails compose through gain in the graph
extent* (#1379), Amendment 1 (H1, H3's live row; root ruling 4 split the live slice in two). Code
anchors verified on `main` at `7e8379523` and on `codex/d15-stream-g2` at `f3956e63c`; re-verify
every anchor at start.

## Product outcome

For an input section with a live input lane, the engine computes three certified composition
values over every history of live trim, polarity and filter targets: the decay `D`, the peak gain
`G_p` and the flush stall `sigma`. They are exposed through test-support accessors and checked on
the real kernel. The live bound keeps `CompositionBound::Unstated` until *Derive the live input
section's tail gain and state its composition* adds the fourth value (`G_t`), because the four
values are stated together or not at all. No reported tail and no rendered bit moves.

## Context

- **The carrier** is `NodeTailBound { tail, tail_every_peak, rest, composition }` (slice A1, #1464);
  the fixed-design values and the construction of `D` come from slice A2 (#1465).
- **`math::tail`'s live cascade** (#1433, `live_cascade`, `LiveCascade`, `live_zones`) computes the
  live `T_decay`, stall, `P*` and rests from the zone state bounds (`Phi`), the LPF's state bound,
  the 65-frame window and every settled group; its relative share is `eps / 2` and the stall's
  `eps / 2` (`crates/math/src/tail.rs:1494-1496`, `:1675`). The live bound costs about 250 us and
  is cached per rate by #1457.
- **`builtins::input_section_live_bound`** (`crates/builtins/src/tail.rs:211`) returns it as
  `NodeTailBound` with `composition: Unstated` (A1).
- **`live_bound_carries_every_term_an_independent_recomputation_requires`** in
  `crates/builtins/tests/tail_contract.rs` recomputes the live bound's terms in plain `f64`.
- **The live words.** A live retarget moves the recursion only through designs and their mixtures,
  within 64 half-ulps plus `u D` per word of their convex hull, and completes by `N + 64` (#1407).
  The trim ramp stays inside its endpoints, at most +24 dB (#1408).
- **Retarget transient (`f64` probe, #1379 Amendment 1, H9).** An HPF at the 48 kHz maximum driven
  by an alternating `+-1` input builds `ic1 = -2.70e4` with `|y| <= 1.04`; a 64-frame ramp to
  10 Hz with zero input then peaks at `|y| = 2.17e3` (+67 dB before the trim). The settled designs'
  `l1` (10 Hz HPF into the top LPF: 3.51) is far below that.

## The contract (#1379 Amendment 1, H1)

`eps = 10^(-144/20)`; `u = 2^-24`; `N` the first sample of silence; no control event at or after
`N`; `g_p = 10^(G_p/2000)`.

- **(N1) Peak.** For every input with `|x[n]| <= X` for all `n`, under any admitted history:
  `|y[n]| <= g_p X + sigma` for every `n`.
- **(N2) Tail at every decade.** If `|x[n]| <= X` for all `n` and `|x[n]| <= epsilon` for every
  `n >= M` (some `M >= N`), then for every integer `k >= 0` and every `n >= M + L + T + k D`:
  `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma`. (The live bound's own split is
  `eps / 2` relative and `eps / 2` stall; (N2) at `3/4` stays valid for it.)
- **`D` by the binding construction** of slice A2 (F-D2): the certified crossings `T(k)` for
  `k = 1..16` from one extended pass, a contraction certificate (`M v <= lambda v`, `lambda < 1`)
  for `k > 16`, and `D = max(max_{k <= 16} ceil((T(k) - T) / k), D_inf + ceil((T_lambda - T) / 17))`.

## Decisions frozen for this slice

- **L-D1. `D`** by the construction above, the maximum over the settled groups; the accessors as
  A2 (`T(k)`, `D_inf`, `lambda`, `T_lambda`).
- **L-D2. `G_p`:** the supremum over every admitted history, through the output row and the
  feedthrough, over the 65-frame window and every settled group, with the trim word of +24 dB,
  plus the live loud-input deviation; rounded up to millibels with `math::log`.
- **L-D3. `sigma`:** the live stall rounded up to millibels.
- **L-D4. No partial statement.** `input_section_live_bound` keeps `CompositionBound::Unstated`;
  the three values are reachable only through the accessors until the tail-gain slice states all
  four.
- **L-D5. Cost** within #1457's D1 budget (the live bound depends only on the rate and is cached per
  rate).

## DSP evidence (AGENTS.md)

- **Equations:** the TPT SVF cascade under #1407's live words (#1433's zone analysis); L-D1-L-D3.
- **Coefficient and update rules:** #1407's retarget through designs and their mixtures; #1408's
  trim ramp.
- **Numerical limits:** `f64` with #1433's certified rounding allowances; every millibel rounding
  upward.
- **Latency and tail:** latency 0; `T`, `T_rest` and both rests unchanged.
- **Units and smoothing:** samples at the plan's rate; gains in millibels.
- **Denormal/NaN:** unchanged (sanitized input; per-block reset of a non-finite state).
- **Citations:** as #1329 and #1433 ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP],
  [SMITH-SASP]; Higham for the rounding model).
- **Fixtures and objective tests:** L1, L3-L6. **Benchmarks:** #1457's gates (L6).
  **Listening:** none; no rendered bit moves.

## Deliverables

1. `math::tail`'s live cascade: `D`, `G_p` and `sigma`, with test-support accessors.
2. The derivation note's live part for `D`, `G_p` and `sigma`
   (`docs/derivations/1379-graph-tail-composition.md`).
3. Gates L1, L3-L6, and the mutation table.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs` (the accessors only), `crates/builtins/tests/tail_contract.rs`
  (named exceptions, stream A's)
- `docs/derivations/1379-graph-tail-composition.md` (the live part)
- The browser's memory fixture (the exact `memoryBytes` pin #1433 names), only if the live bound's
  peak allocation moves, with its reason
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Non-goals

- The live tail gain `G_t` and the switch to `Stated` (*Derive the live input section's tail gain
  and state its composition*).
- Fixed designs (A2). Any graph or report change (#1379).

## Hazards

- **#1433's values must not move**: live `T_decay`, `T_rest`, both rests and `P*` (L5).
- **`G_p` is not the settled designs' `l1`.** The retarget transient after a Nyquist drive exceeds
  it by tens of dB; L1 exists for that.
- **Cost.** The extended horizon over every settled group lengthens the live computation; a miss of
  #1457's budget stops the slice.

## Objective gates

`crates/builtins/tests/tail_contract.rs`, release, every launch rate.

- **L1. Live peak gain on the real kernel.** A real `InputBuiltins` with a live input lane, trim
  +24 dB, both sections designed at the worst-case pair (`input_section_worst_case_pair(rate)`),
  driven by an alternating `+-1` input for 1,000,000 frames, then the HPF target moved to 10 Hz with
  the input still running: the largest `|y|` over the run is at most `g_p + sigma` (accessors). The
  measured ratio is recorded.
- **L3. Independent recomputation.** The existing
  `live_bound_carries_every_term_an_independent_recomputation_requires` gains `D`, `G_p` and
  `sigma`: each lies between a plain-`f64` recomputation of the derivation in the test and that
  value plus 0.01 % and 64 frames (`D`) or 1 mB (gain, stall).
- **L4. Live decade identity.** The module's directly searched live crossing `T(k)` is at most
  `T + k D` for `k = 0..64`, and `D_inf` is at least the live floor recomputed as in A2's F1(d).
- **L5. Nothing certified moves.** Every existing `tail_contract` assertion passes unchanged; the
  live bound still states `Unstated`; no rendered bit moves (`audit capi`'s `pcm_digest`, the wasm
  G5 digests, the builtins PCM fixtures).
- **L6. Budget.** #1457's gates 2 and 8 pass after this change, within #1457's D1 budget.
- **Commands:** as slice A2 (`cargo test --locked --all-targets -p lane -p math -p builtins -p
  dsp-reference --features math/lane,builtins/test-support,lane/test-support`; `cargo test --locked
  --release -p builtins --features builtins/test-support --test tail_contract`; #1457's gate 2 and
  gate 8 commands (#1457 Amendment 3: gate 2 is `cargo build --locked --release -p builtins
  --features test-support --example input_bound_budget` and one invocation of `taskset -c <core>
  target/release/examples/input_bound_budget`, descriptive; gate 8 is `cargo test --locked -p
  builtins-compiler --features test-support --lib
  every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`); `audit capi`;
  `bash scripts/check-builtins-fixtures.sh . target/release/audit`; `bash
  scripts/run-wasm-gates.sh`; `bash scripts/check-workspace-policy.sh`; clippy with `-D warnings`;
  `cargo fmt --all -- --check`).

The attempt record carries a mutation table with at least the mutants below; each is red.

## Test value

- L1: a live `G_p` taken from the settled designs' `l1` (about +35 dB with the trim: the 10 Hz HPF
  into the top LPF has `l1` 3.51) is beaten by the retarget transient by tens of dB (one HPF alone:
  +67 dB before the trim); no other test drives a retarget after a Nyquist drive.
- L3: a live value that drops a ramp-window or zone term falls below the recomputation; the real
  kernel's slack (about 8.5 % in `R`) cannot see it.
- L4: a live `D` with no transient term falls below the module's own crossing at small `k`; a
  certificate rate that omits the state rounding falls below the floor (as A2's F1(c) and F1(d)).
- L6 (corrected 2026-10-08, root order): preparation reads `input_section_live_bound_table` and
  never computes the live bound (`crates/builtins/src/tail.rs`, module doc and the table's doc;
  `crates/builtins-compiler/src/lib.rs` and `crates/host-core/src/prepare.rs` read only the table),
  so a slow live construction cannot turn #1457's gates 2 and 8 red; the earlier sentence ("a live
  construction that runs one pass per `k` is red there") was wrong. What L6 defends is narrower:
  this slice's edits to the shared `crates/math/src/tail.rs` (the fixed walk lives there too) do not
  move the fixed walk's cost past #1457's budget or change gate 8's exact bounds. The live
  construction's own cost is recorded descriptively (one invocation, ms per rate). The attempt
  restates L6's test value with that measured evidence, or names the defect it catches; if none,
  L6 is reduced to a regression run of #1457's gates and claims no unique catch.

## Dependencies

- *State a fixed input section's decay, gains and flush stall* (slice A2, #1465): the construction
  of `D`, the accessors' shape and the shared files.
- *Carry every node's tail bound in one node-neutral struct* (slice A1, #1464).
- *Split a node's flush stall into a peak stall and a tail stall* (#1484, root ruling of
  2026-10-08): the two stalls this slice certifies (L-D3 as the ruling reads it, Attempt record).
- *Tighten the cascade exact-rest bound with a frequency-aware cascade analysis* (#1433, passed),
  *Retarget a live input filter only through its designs and their mixtures* (#1407), *Keep every
  trim, fader and matrix ramp inside its endpoints* (#1408).
- *Cache design bounds across preparations within a stated preparation budget* (#1457): the gated
  budget.
- Named exceptions: `STREAMS.md` (#1379 Amendment 1, H8).

## Attempt record

### Attempt 1 (2026-10-08): stopped before implementation, one spec problem (L-D3 against (N1))

Anchors re-verified on `codex/d15-stream-g3` at `583fd8607`. They moved with #1474 and #1465; their
content is as the spec states: the live relative share `eps / 2` is `LiveBound::tail`'s `limit`
(`crates/math/src/tail.rs:1639`), the stall's share is `live_cascade`'s `P*`
(`bound.stall(tail) / (eps / 2)`, `:1818`); `LiveCascade` `:829`, `live_zones` `:1035`,
`live_cascade` `:1805`; `builtins::input_section_live_bound` is `crates/builtins/src/tail.rs:483`;
`live_bound_carries_every_term_an_independent_recomputation_requires` is
`crates/builtins/tests/tail_contract.rs:1568`. The construction of `D` read as #1465's amended F-D2
(the certificate anchored at `T`, no extended walk); the live bound has no frame walk at all (its
settled phase is carried by powers of each zone group's 4 x 4 system), so the anchor costs one
power per group.

**Prototype (exploration only; not committed).** A prototype in `math::tail`
(`live_cascade_composition`, `LiveComposition`; the diff is kept outside the tree) computed the
three values at the +24 dB trim word, per unit of the input's peak:

- `D`: one certificate per settled zone group (#1465's `HalfSystem` on each group's relative
  system `(tau, H, X)`, order `H, tau, X`), anchored at `T` by powers of the step. Every group's
  transient `a` is negative at `T` (the group that sets `T` has `a = -0.91`), so
  `D = max_g (floor(b_g) + 1)`: **46,678 / 46,421 / 46,678 / 46,421** (44.1 / 48 / 88.2 / 96 kHz),
  `crossing(1)` 46,241 / 45,986 / 46,241 / 45,985, `lambda` 0.999950672 / 0.999950398, 60-63
  groups, about 0.26-0.34 ms per rate. (When `T` lies in the 65-frame window, each group anchors at
  the window's end with offset `65 - T`, so `D >= 66 - T` and `T + k D` is past every window frame;
  not reached at the launch rates, where `T` is about 700,000.)
- `G_p`: the window's first frame is the output bound at any frame of any admitted history with the
  current input zero (the pre-`N` analysis holds at every frame: `Phi` and the second section's
  Abel sum are stated for any history), and the current input adds `delta^2 g` through the two
  feedthroughs; so `L = outputs[0] + delta^2 g` bounds every frame, the window's and every settled
  group's included. **`L` = 1.629e7 (+144.24 dB) / 1.613e7 / 1.632e7 / 1.615e7.** The real kernel
  under L1's history (the worst-case pair, +24 dB trim, alternating `+-1` for 1,000,000 frames,
  then the HPF target moved to 10 Hz with the input running) peaks at **3.446e4 (+90.75 dB)** /
  3.428e4 / 3.446e4 / 3.429e4 (before the retarget: 3.19). So `G_p` is sound and about **53.5 dB**
  above the measured transient (the pre-`N` second-section state bound `sup Psi Phi` of #1433 is
  what sets it; the spec has no tightness line for `G_p`, so this is recorded, not a stop).
- `sigma`: see the problem below.

**The problem: L-D3's "live stall" does not satisfy (N1).** (N1) needs `|y[n]| <= g_p X + sigma`
at **every** frame, for every input, `X -> 0` included, so `sigma` must bound the flush's part of
the output bound at every frame. #1433's live stall (`LiveBound::stall(T)`, the stall `live_cascade`
reads for `P*`) is that part only **from `T` on**: the window's outputs from frame `T` and each
group's fixed point plus its decaying start from `T`. Before `T` the module's own flush part is far
larger, because the pre-`N` and window analysis (no settled-phase output-change trick, words still
moving) amplifies `F` through `sup Psi Phi_F`. Measured with the same machinery from frame 0
(`stall(0)`, which also covers a loud frame: the current input adds nothing to the flush part):

| rate | stall from `T` (`P* eps / 2`) | stall at every frame | ratio |
|---|---|---|---|
| 44.1 kHz | 3.794e-15 | 2.042e-12 | 538 (+54.6 dB) |
| 48 kHz | 3.773e-15 | 2.208e-12 | 585 (+55.3 dB) |
| 88.2 kHz | 3.798e-15 | 4.142e-12 | 1,091 (+60.8 dB) |
| 96 kHz | 3.853e-15 | 4.487e-12 | 1,164 (+61.3 dB) |

So a `sigma` read as the live stall (from `T`) is not a certified (N1) value: at early frames the
module's bound exceeds `g_p X + sigma` for small `X`. (N2) needs the stall only from `M + T` on,
where `stall(T)` suffices. The fixed slice (#1465) had no such split: its `F a` holds at every frame.

The two readings, for root:

1. **`sigma` = the every-frame flush bound** (`stall(0)`, rounded up). (N1) and (N2) hold with H1's
   single `sigma`. Cost: `P^ = 4 sigma / eps` is about 1.3e-4 to 2.8e-4 (-77.8 to -71 dBFS) instead
   of about `2 P*` (2.4e-7); H1's remark that the live `P^ < 2 P*` holds "to within `SLACK`" is then
   false (`P^` is about 1,076-2,328 `P*`), and every composed graph floor that reads the live input
   section's `sigma` rises by 55-61 dB.
2. **`sigma` = the stall from `T`** with a contract change: (N1)'s additive floor stated separately
   (a peak stall) from (N2)'s (a tail stall). That changes H1 (one `sigma`) and #1379's composition,
   which only root can amend.

No code, test or document changed besides this record. The prototype's `D` construction and `G_p`
are ready for the next attempt once `sigma` is ruled; the derivation note's live part will be
written first, as ruled.

### Root's ruling (2026-10-08), option (2)

Root ruled the conflict above with option (2): the contract changes.

- `CompositionBound`'s single stall becomes two fields: `sigma_p` (the peak stall; it bounds the
  flush part of the output at every frame and is (N1)'s term) and `sigma_t` (the tail stall; it
  bounds it from the node's tail on and is (N2)'s term). H1 and #1379's composition read `sigma_p`
  for (N1) and `sigma_t` for (N2). The fixed input section states `sigma_p = sigma_t = F a`; every
  other node still states `Unstated`.
- Reason: the live section's stall from `T` is 55-61 dB below its every-frame stall (the table
  above); one `sigma` would permanently cost every live strip that much exact-rest reach.
- The change is filed as *Split a node's flush stall into a peak stall and a tail stall* (#1484),
  which lands before this slice. It blocks this slice, #1467 and #1379.
- For this slice, L-D3 reads: `sigma_p` is the every-frame flush bound (`stall(0)` of the
  prototype) rounded up to millibels, and `sigma_t` the live stall from `T` (`P* eps / 2`) rounded
  up; `sigma_t <= sigma_p`. L1 checks `max |y| <= g_p + sigma_p`; L3 recomputes both stalls. The
  accessors expose both.
- Root also filed the live `G_p` tightening successor, *Tighten the live input section's peak gain
  toward the real-kernel history peak* (#1485): the prototype's `G_p` (+144.24 dB at 44.1 kHz) is
  sound and about 53.5 dB above the real-kernel history peak (+90.75 dB); this slice keeps it.
- L6's test value is corrected in place (Test value, L6).

The next attempt starts after #1484 has landed.

### Attempt 1, restarted under root's ruling (2026-10-08, implementer)

Anchors re-verified on `codex/d15-stream-g3` at `3954dce33` (after #1474, #1464, #1465 and its
follow-ups, and #1484): `LiveCascade` (`crates/math/src/tail.rs:829` then, `:904` after this slice),
`live_zones`, `LiveBound::window`, `settled_system`, `settled_start`, `tail`, `stall`,
`live_cascade`; `builtins::input_section_live_bound` and its table; the recomputation test.
`CompositionBound::Stated` carries `peak_stall` and `tail_stall` with the readers `peak_clause` and
`tail_clause` and rule (g) (#1484).

**Proof first.** The derivation's live part is written in
`docs/derivations/1379-graph-tail-composition.md`, "The live input section (issue #1466, slice
B1)": (P1) the split of #1433's bound into a relative part and a flush part by separate majorant
runs (needed: at every launch rate 21 of the 60-63 groups take the window's state bound in one part
and `Phi` in the other, so the split is not arithmetic of the minimum; a capped window refuses the
composition); (P2) the window's first frame bounds the states at every frame of every admitted
history (it reads only the frames before `N`); (N1) `G_p = ceil_mB((W_0(g, 0) + delta^2 g) SLACK)`
and `sigma_p = ceil_mB(stall(0))` with `stall(0) >= W_0(0, F)`; (N2) `sigma_t =
ceil_mB(min(stall(T), stall(0)))` (so rule (g) holds by construction) and `D` by the fixed design's
certificate per settled group on its relative `3 x 3` system, anchored at `T`, with the carry's
rounding argument redone for `n = 3` (off-diagonal at most 8, start at most `2^132`, both checked;
carried error below `2^-807` against `tau = 2^-600`).

**Changed.**
- `crates/math/src/tail.rs`: `LiveComposition` (`decay`, `groups`, `peak_gain`, `peak_stall`,
  `tail_stall`; `crossing`, `asymptotic_decay`, `lambda`, `floor_crossing`),
  `LiveBound::composition`, the public accessors `live_cascade_composition` and
  `live_cascade_crossing` (the directly searched crossing of `(eps / 2) 10^-k`, L4's evidence);
  `LiveBound::tail` takes its threshold (`T_decay` at `TAIL_FLOOR / 2`, unchanged); `LiveWindow`
  records whether a state bound took the cap (no value changes); `LIVE_STATE_LIMIT`.
- `crates/builtins/tests/tail_contract.rs`: L1
  (`live_peak_gain_bounds_a_retarget_after_a_nyquist_drive_on_the_real_kernel`), L3 (the
  recomputation test and `live_oracle` gain `D`, `G_p`, `sigma_p`, `sigma_t`), L4
  (`live_decay_covers_the_module_crossings_and_its_floor`), the helpers `live_composition` and
  `settled_contraction` (the latter factored out of the recomputation test, unchanged).
- `docs/derivations/1379-graph-tail-composition.md`: the live part, the title, the introduction and
  the gates.
- No change to `crates/builtins/src/tail.rs`: the accessors are `math::tail`'s (Deliverable 1); a
  `builtins` accessor would need a re-export in `crates/builtins/src/lib.rs`, which this slice is
  not authorized to edit. The millibel rounding at statement is B2's (`stated_composition`'s rule);
  the gates read the raw values and the record gives `ceil(2000 log10)`.
- `STREAMS.md` has no status column for this row; unchanged.

**Values** (release, `live_cascade_composition` at the +24 dB trim word; mB = `ceil(2000 log10)`):

| rate | `T` | `D` (= `D_inf`) | `lambda` | groups | `G_p` raw / mB | `sigma_p` raw / mB | `sigma_t` raw / mB |
|---|---|---|---|---|---|---|---|
| 44.1 kHz | 704,010 | 46,678 | 0.999950671973 | 60 | 1.629239e7 / 14,424 | 2.041952e-12 / -23,379 | 3.793538e-15 / -28,841 |
| 48 kHz | 699,952 | 46,421 | 0.999950397971 | 61 | 1.613073e7 / 14,416 | 2.207801e-12 / -23,312 | 3.772735e-15 / -28,846 |
| 88.2 kHz | 704,018 | 46,678 | 0.999950671973 | 62 | 1.631644e7 / 14,426 | 4.142272e-12 / -22,765 | 3.797574e-15 / -28,840 |
| 96 kHz | 699,960 | 46,421 | 0.999950397971 | 63 | 1.615350e7 / 14,417 | 4.486749e-12 / -22,696 | 3.852977e-15 / -28,828 |

`sigma_t = stall(T)` at every rate (the `min` with `stall(0)` never binds); `sigma_t <= sigma_p`
by 54.6-61.3 dB. Every group's transient `a` is negative at `T` (largest -0.91 / -0.54 / -0.39 /
-0.60), `T_lambda - T = 0`. `stall(0)` is the flush window's first frame plus 0.08-0.16 % (at most
1.4 mB). Cost (descriptive, one probe run, release): `live_cascade_composition` 0.24-0.38 ms per
rate, the 65 direct searches of L4 about 9 ms per rate; preparation never runs either.

**Gates.**
- L1: the real-kernel peak over the run is 3.4456e4 / 3.4281e4 / 3.4460e4 / 3.4285e4 (+90.75 /
  +90.70 / +90.75 / +90.70 dB; 3.19 before the retarget) against `g_p + sigma_p`: ratio 472.8 /
  470.5 / 473.5 / 471.2 (53.49 / 53.45 / 53.51 / 53.46 dB). Pass; the gap is #1485's.
- L3: module against the plain-`f64` recomputation: `D` equal at every rate; `G_p`, `sigma_p`,
  `sigma_t` above it by 3e-8, 1e-6 and 2e-6 relative at most (1 mB is 1.15e-4). The test also
  asserts `sigma_t <= sigma_p`, raw and in mB (#1484 verdict note: rule (g) runs only in the
  registry, and the live section is no registry row). Pass.
- L4: the directly searched `T(k)` is `T` at `k = 0` and at most `T + k D` for `k = 1..64`;
  `max (T(k) - T) / k = 46,099 / 45,846` (at `k = 1`), against `D` 46,678 / 46,421. `D_inf` is
  above the floor 44,466 / 44,224. Pass.
- L5: the release `tail_contract` (19 passed: every existing assertion unchanged; the live table
  test pins `Unstated`); `audit capi` `pcm_digest` `cb10fbface44a3a4`, 0 allocations, 0
  syscalls; `check-builtins-fixtures.sh` ok (50 files); `run-wasm-gates.sh` ok.
- L6: gate 8 passed. Gate 2: one invocation, `taskset -c 7`, after the 1-minute load fell below 2
  (`/proc/loadavg` before 1.76 3.20 3.40, after 1.59 3.10 3.36; the 5- and 15-minute loads were
  still above 3 from other work): exit 0, 17.0 ns needed (committed 17 ns), worst median design
  work 25.330 ms, 98.7 % of 25.67 ms. Not the figure of record (the batch verifier's is).
- The debug gate command (`lane`, `math`, `builtins`, `dsp-reference`, all targets): exit 0;
  `check-workspace-policy.sh` ok; `check-cross-targets.sh` PASS; workspace clippy `-D warnings`
  clean; `cargo fmt --check` clean.

**Mutation table** (each defect applied, the release `tail_contract -- live_` run (the ten live
tests; nothing else reads the composition), the files restored; "mirrored" means the same defect
also applied to the recomputation, a defect of the derivation itself):

| mutant | red | green |
|---|---|---|
| M1 `G_p` = trim x settled `l1` (3.51), mirrored | L1 only (`3.4456e4 > 55.63`) | L3 (mirrored), L4 |
| M2 `sigma_p` = `stall(T)` (attempt 1's reading) | L3 only | L1, L4 |
| M3 `sigma_t` = `stall(0)` | L3 only | L1, L4 |
| M4 `D` anchored at the settled start, not at `T` | L3 only (`D` 922,826) | L1, L4 |
| M5 `D` = `ceil(ln 10 / -ln rho_max)` per group (no transient), mirrored | L4 only (`T(1) = T + 46,099 > T + 45,220`) | L3 (mirrored) |
| M6 certificate's `tau` diagonal without the settled state rounding, mirrored | L4's crossing check only (`D` 45,264) | L3 (mirrored), L4's floor |
| M7 `G_p` without the current input's feedthrough `delta^2 g` | L3 only (module 9.4e-7 below) | L1, L4 |
| M8 `G_p` over the settled groups only, no window frame | L3 only (2.6 % below) | L1, L4 |
| M9 the stalls swapped (`sigma_p = stall(T)`, `sigma_t = stall(0)`) | L3 at its `sigma_t <= sigma_p` assertion | L1, L4 |

**Test value (restated with the measured evidence).**
- L1: as the spec states (M1): a live `G_p` from the settled designs' `l1` is beaten by the
  retarget transient by 55.8 dB; no other test drives a retarget after a Nyquist drive.
- L3: a `sigma_p` read from `T` (M2, attempt 1's defect), a `sigma_t` read at every frame (M3), a
  `D` anchored at the settled start (M4, sound but 20 times loose), a `G_p` without the current
  input (M7) or without the window frame (M8) each leaves the recomputation's band; nothing else
  reads these values, and the real kernel's 53.5 dB slack cannot see M7 or M8. Its
  `sigma_t <= sigma_p` assertion is the live section's rule (g) (M9); M9 is also outside the
  band, so the assertion claims no catch beyond L3's. The equality boundary: no live stall pair is
  equal (61 dB apart), so a strict `<` mutant cannot be told apart here; `<=` is asserted
  because equal stalls are admissible (#1484's registry tests pin that boundary).
- L4: the crossing check catches a derivation that takes `D` from the settled contraction alone
  (M5) or drops the state rounding from the certificate (M6), each mirrored, so L3 agrees: the
  coupled `H -> tau` transient makes the first decade 46,099 frames against the 45,220 the rate
  alone gives. The spec's sentence "a certificate rate that omits the state rounding falls below
  the floor" is **false** for the live section: M6 leaves `D_inf` 45,264 above the floor 44,466
  (the floor sits 5 % below `D`, set by the transient); M6 is red only at the crossing check. The
  floor assertion is kept as the spec's L4 invariant and claims no unique catch.
- L6 (reduced to a regression run, as the corrected text allows): this slice's `math::tail` edits
  touch only the live code (`LiveBound::tail`'s threshold, the window's cap flag, the new
  composition); the fixed walk is unchanged, and gate 2's figures stay at the committed 17 ns and
  gate 8 exact. L6 claims no unique catch.

**Open items.**
- `G_p` is 53.5 dB above the real-kernel peak (#1485).
- The live millibel statement (`ceil_mB` with the module's margins) and `G_t` are B2's (#1467).
- A `builtins` accessor for the live composition needs a re-export in `crates/builtins/src/lib.rs`
  (not authorized here); B2's statement makes it unnecessary.
