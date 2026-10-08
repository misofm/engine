# Split a node's flush stall into a peak stall and a tail stall

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-08 by root order: root's ruling of 2026-10-08, option (2), on the stop of *Certify
the live input section's decay, peak gain and flush stall* (#1466) attempt 1. It is a contract
change to #1379 Amendment 1 H1 and to slice A1's carrier (#1464), and it lands before #1466. Code
anchors verified on `codex/d15-stream-g3` at `d12fd3940`; re-verify every anchor at start.

## Product outcome

Every node's composition statement carries two flush stalls instead of one: a **peak stall**
`sigma_p`, which bounds the flush part of the output at every frame and is the additive term of
(N1), and a **tail stall** `sigma_t`, which bounds it from the node's tail on and is the additive
term of (N2). The fixed input section states `sigma_p = sigma_t = F a` (today's value); every other
node still states `Unstated`. No reported tail, no certified value and no rendered bit moves. A
live input section can then state its small tail stall for (N2), and keep its large every-frame
stall for (N1), instead of paying the large one in both.

## Context

- **Why (root ruling of 2026-10-08, option (2)).** #1466 attempt 1 (its Attempt record, at
  `d12fd3940`) found that the live input section's stall from `T` (#1433's `LiveBound::stall(T)`,
  the stall `live_cascade` reads for `P*`) bounds the flush part only from `T` on. Before `T` the
  module's own flush part is much larger, because the pre-`N` and window analysis amplifies `F`
  through `sup Psi Phi_F`. Measured with the same machinery:

  | rate | stall from `T` (`P* eps / 2`) | stall at every frame (`stall(0)`) | ratio |
  |---|---|---|---|
  | 44.1 kHz | 3.794e-15 | 2.042e-12 | 538 (+54.6 dB) |
  | 48 kHz | 3.773e-15 | 2.208e-12 | 585 (+55.3 dB) |
  | 88.2 kHz | 3.798e-15 | 4.142e-12 | 1,091 (+60.8 dB) |
  | 96 kHz | 3.853e-15 | 4.487e-12 | 1,164 (+61.3 dB) |

  (N1) needs the every-frame value; (N2) needs only the value from `T` on. With one `sigma`, every
  live strip would carry the every-frame value into (N2), and its share of `P*_graph` (#1379 H4
  step 6) would rise by 55-61 dB for good: `P^ = 4 sigma / eps` would be about 1.3e-4 to 2.8e-4
  (-77.8 to -71 dBFS) instead of about 2.4e-7, which costs every live strip that much exact-rest
  reach.
- **The carrier today** (slice A1, #1464): `effect_contract::CompositionBound` (`crates/effect-contract/src/lib.rs:324`)
  is `Stated { decay: TailDecay, peak_gain: PeakGain, tail_gain: PeakGain, stall: FlushStall } |
  Unstated`; `FlushStall { Zero, Level(i32) }` (`:358`) is `sigma` in millibels re 1.0, rounded up.
  `NodeTailBound::max` (`:255`) takes each value by maximum. The registry's consistency check
  `tail_bound_consistent` (`:2645`) has rules (a)-(f); (e) and (f) are A1's composition rules.
- **The fixed input section** (slice A2, #1465): `builtins::tail`'s `stated_composition`
  (`crates/builtins/src/tail.rs:56`) states `stall: Level(ceil_mB(F a))` from
  `math::tail::CascadeComposition::stall` (one raw `f64`, `crates/math/src/tail.rs:520`), and
  `memoryless_channel` (`:101`) states one underflow (`2^-126`) for a channel with both filters
  disabled. `F a` holds at every frame, so it is both stalls; `math::tail` does not change.
- **Who reads `stall` today:** `crates/builtins/tests/tail_contract.rs` (`composition_values`,
  `:2084`; F3, `fixed_design_stall_is_the_module_stall_rounded_up_and_covers_the_flush_floor`,
  `:2403`, which reads `P^ = 4 sigma / eps`, an (N2) use; the disabled-filter test `:2442`);
  `crates/effect-contract/tests/{tail_bound,registry}.rs` (fixtures and the K3/K4 tests); and the
  forgery row `composition` of `crates/effect-compiler/src/prepare.rs` (`:1196`, a literal). No
  graph code reads it yet (#1379 is not implemented).
- **Where H1 and the composition state `sigma`:** #1379's spec (H1 table and (N1)/(N2), H3's
  `sigma` column, H4 step 6 and the floor), the design note
  `docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md` (H1, H3, H4 step 6 and the
  inequality chain (i)-(v)), `docs/EFFECT_CONTRACT_V1.md` (*Tail and exact rest*, `:104-133`),
  `docs/derivations/1379-graph-tail-composition.md` (*The contract (H1)*, `:11-31`; the fixed
  design's (N1) and (N2), `:61-96`; *The stall `sigma`*, `:252-261`; *A channel with its filters
  disabled*, `:262-270`).

## The contract after this slice (#1379 Amendment 1 H1, amended)

Notation as H1: `eps = 10^(-144/20)`; latency `L`; `N` the first sample of silence; `g_p`, `g_t`
the linear gains; `sigma_p`, `sigma_t` the two stalls' linear levels (`0` for `Zero`).

| field | symbol | unit | meaning |
|---|---|---|---|
| `peak_stall` | `sigma_p` | millibels re 1.0, rounded up, or `Zero` | the flush part of the output at every frame, (N1) |
| `tail_stall` | `sigma_t` | millibels re 1.0, rounded up, or `Zero`; `sigma_t <= sigma_p` | the flush part from the node's tail on, (N2) |

- **(N1) Peak.** If `|x[n]| <= X` for all `n`, under any admitted history:
  `|y[n]| <= g_p X + sigma_p` for every `n`.
- **(N2) Tail at every decade.** Under H1's conditions (C), if `|x[n]| <= X` for all `n` and
  `|x[n]| <= epsilon` for every `n >= M` (some `M >= N`), then for every integer `k >= 0` and every
  `n >= M + L + T + k D`: `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma_t`.
- **(N3) Rest.** Unchanged.
- **The share `3/4` and `P^`.** (N2) at `k = 0`, `epsilon = 0` is #1329 D1 for every peak
  `P >= P^ := 4 sigma_t / eps`.
- **`sigma_t <= sigma_p`.** (N2)'s frames are a subset of (N1)'s, so a node that states a tail
  stall above its peak stall states a value no derivation needs; the registry refuses it.
- **The composition (#1379 H4).** Every use of a node's stall that bounds a signal at every frame
  reads `sigma_p`; every use from the node's tail on reads `sigma_t`. In H4 step 6:
  `S_out(v) = g_p(v) max_u S_out(u) + sigma_p,v` (it feeds `X_v` in chain (i), an (N1) use) and
  `sigma'_v = sigma_t,v + (3/4) eps 10^(-k(v)) S_in(v)` if `D_v > 0`, else `sigma_t,v` (it feeds
  `Sigma` in chain (iii)-(v), an (N2) use). `X*(v)` (step 7) reads `S_in(v)`, so `sigma_p`.

## Decisions frozen for this slice

- **S-D1. Two fields, no third.** `CompositionBound::Stated`'s `stall: FlushStall` becomes
  `peak_stall: FlushStall` and `tail_stall: FlushStall` (`FlushStall` unchanged). No alias, no
  `stall` left beside them. The implementer may refine the two names (no version suffix) and states
  the final names in `docs/EFFECT_CONTRACT_V1.md`.
- **S-D2. Readers by clause.** `CompositionBound` gains two `const fn` readers, the only way the
  later slices read the stalls: `peak_clause(self) -> Option<(PeakGain, FlushStall)>` (`(G_p,
  sigma_p)`, (N1)'s pair) and `tail_clause(self) -> Option<(TailDecay, PeakGain, FlushStall)>`
  (`(D, G_t, sigma_t)`, (N2)'s triple); `None` for `Unstated`. The implementer may refine the names.
  #1466's L1, F3 here, #1467 and #1379 read through them; that pairing is H1, and a test pins it.
- **S-D3. A third composition rule.** `tail_bound_consistent` gains rule (g): in a `Stated`
  composition, `tail_stall <= peak_stall`, with `FlushStall::Zero` below every `Level`; it refuses
  with the existing typed error `effect.tail_bound.inconsistent`, as (e) and (f) do. No other new
  rule.
- **S-D4. `max`.** `NodeTailBound::max` takes each stall by its own maximum (`Zero` below every
  `Level`); it never takes one stall from the other.
- **S-D5. The fixed section states both.** `stated_composition` states
  `peak_stall = tail_stall = Level(ceil_mB(F a))` from the same raw value; `memoryless_channel`
  states both as one underflow. Every effect, the conformance mock, the live bound and every other
  node keep `Unstated`.
- **S-D6. H1 and #1379's composition updated in text.** The contract above replaces H1's `sigma`
  row and its (N1), (N2) and `P^` sentences in #1379's spec and the design note; H3's `sigma`
  column becomes `sigma_p` and `sigma_t` (equal on every row except the live input lane:
  `sigma_p` the every-frame flush bound, `sigma_t` the live stall from `T`, both slice B1's); H4
  step 6 and the chain read as the contract's last bullet. `math::tail` is unchanged.

## DSP evidence (AGENTS.md)

- **Equations:** H1 as amended above; the fixed section's `F a` (#1465 F-D4) holds at every frame,
  so it bounds both clauses.
- **Coefficient and update rules, latency and tail, smoothing, denormal/NaN:** unchanged (no kernel
  and no value moves).
- **Numerical limits:** the same `ceil_mB` rounding upward for both fields.
- **Citations:** as #1465 and #1329.
- **Fixtures and objective tests:** S1-S5. **Benchmarks:** none (no computation is added).
  **Listening:** none; no rendered bit moves.

## Deliverables

1. The two fields, the two readers, rule (g) and `max`'s rule in `crates/effect-contract/src/lib.rs`,
   with their docs.
2. `builtins::tail`'s two writers; every `stall:` literal renamed in the tests and the forgery row.
3. H1 amended in `docs/EFFECT_CONTRACT_V1.md`, `docs/derivations/1379-graph-tail-composition.md`,
   #1379's spec (H1, H3's column, H4 step 6) and the design note (H1, H3, H4 step 6, chain (i) and
   (iii)-(v)).
4. Gates S1-S5 and the mutation table; each moved resource count re-pinned with its reason.

## Objective gates

- **S1. The readers pair as H1 states.** `crates/effect-contract/tests/tail_bound.rs`: on a
  `Stated` composition with every value distinct (`sigma_t` strictly below `sigma_p`), the (N1)
  reader returns `(G_p, sigma_p)` and the (N2) reader `(D, G_t, sigma_t)`; both return `None` for
  `Unstated`.
- **S2. The registry refuses a tail stall above the peak stall.** `crates/effect-contract/tests/registry.rs`:
  a descriptor with `tail_stall` one millibel above `peak_stall`, and one with a `Level` tail stall
  beside a `Zero` peak stall, are each refused with `effect.tail_bound.inconsistent` at one rate
  only; a descriptor with `tail_stall` below `peak_stall`, and one with a `Zero` tail stall under a
  `Level` peak stall, are admitted.
- **S3. `max` keeps the stalls apart.** `max_states_a_composition_only_when_both_channels_state_one`
  gives its operands distinct stalls, crossed so that each side supplies one larger stall, and
  asserts both in both operand orders.
- **S4. Nothing certified moves.** For every row F3 and the disabled-filter test check, both
  stalls equal `ceil_mB` of the module's raw stall (or one underflow), which is today's `stall`;
  every other `tail_contract`, `effect-contract` and `builtins-compiler` tail assertion passes with
  only the field rename; F3's `P^` reads the (N2) reader; no rendered bit moves (`audit capi`'s
  `pcm_digest`, the wasm G5 digests, the builtins PCM fixtures). A resource count that names the
  tail entry moves only by the size of the added field (a `FlushStall`, about 8 bytes per bound;
  the builtins' tail entry 104 -> about 112 bytes), each re-pin audited with its reason. The
  browser `builtinRetainedBytes` row had 31 bytes of headroom after #1464 (2,017 of 2,048); if it
  exceeds its ceiling, the slice stops and reports (no ceiling is raised here).
- **S5. Commands:** the `test-debug-a` workspace command from `.github/workflows/qualification.yml`;
  `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`;
  `bash scripts/check-effect-contract.sh`; `cargo build --locked --release -p audit &&
  ./target/release/audit capi`; `bash scripts/check-builtins-fixtures.sh . target/release/audit`;
  `cargo test --locked -p audit`; `bash scripts/check-workspace-policy.sh`; `cargo run --locked -p
  conformance --example conformance_fixtures -- --check`; `bash scripts/run-wasm-gates.sh`; the
  worklet chain #1464 ran (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh`); `cargo clippy --locked --workspace --all-targets -- -D warnings`;
  `cargo fmt --all -- --check`.

The attempt record carries a mutation table (each defect applied, the named tests run, the file
restored) with at least the mutants in "Test value"; each is red.

## Test value

- S1: a reader that returns `sigma_t` in (N1)'s pair (the defect that would let #1379's `S_out`
  and #1466's L1 use the small stall at every frame) is red; so is one that returns `sigma_p` in
  (N2)'s triple (the defect that would put the every-frame stall back into `Sigma`). On the fixed
  section the two are equal, so only S1's distinct fixture can tell them apart; no other test can.
- S2: a registry without rule (g), or one that orders `Zero` above a `Level`, admits a tail stall
  above the peak stall, which #1379 would compose as a contradiction; nothing else validates it.
- S3: a `max` that takes a stall from the other field, or from one side only, states a stall no
  channel derived; S3's crossed operands see both.
- S4: a writer that states the two fixed stalls from different values (for example the tail stall
  left at one underflow) moves a certified value and is red at F3's equality; reuses the existing
  pins otherwise.

## Non-goals

- Any live value (#1466 states the live `sigma_p` and `sigma_t` behind accessors; #1467 states the
  live composition). Any change to `math::tail`, to `D`, `G_p`, `G_t`, `T`, the rests or `P*`.
- Any graph or report change (#1379 implements the amended composition).
- Raising any resource ceiling.

## Authorized paths (named exceptions are marked)

- `crates/effect-contract/src/lib.rs` (`CompositionBound`, its readers and docs,
  `NodeTailBound::max`, `tail_bound_consistent` and its doc; hot file),
  `crates/effect-contract/tests/tail_bound.rs`, `crates/effect-contract/tests/registry.rs`
- `crates/effect-compiler/src/prepare.rs` (the forgery row's literal, the field rename only; hot
  file)
- `crates/builtins/src/tail.rs` (`stated_composition`, `memoryless_channel`),
  `crates/builtins/tests/tail_contract.rs` (`composition_values`, F3 and the disabled-filter test:
  the field rename and the readers) (named exceptions, stream A's)
- Only where a recorded byte count moves: `tools/audit/src/fixture_builtins.rs`
  (`BOXED_TAIL_ENTRY_BYTES` and its note), `fixtures/builtins/v1/resources.jsonl`,
  `fixtures/builtins/v1/MANIFEST.tsv`, `tools/audit/src/builtins_graph.rs` (the manifest identity),
  `hosts/host-web/tests/browser-v1/expected.json`
- `docs/EFFECT_CONTRACT_V1.md` (*Tail and exact rest*: the composition table, (N1), (N2) and the
  rule list), `docs/derivations/1379-graph-tail-composition.md` (*The contract (H1)*, the fixed
  design's (N1) and (N2), *The stall `sigma`*, *A channel with its filters disabled*)
- `.github/ISSUE_SPECS/1379-define-how-node-tails-compose-through-gain-in-the-graph-extent.md`
  (H1, H3's `sigma` column, H4 step 6) and
  `docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md` (H1, H3, H4 step 6 and the
  chain), as S-D6 states
- Any other `stall:` literal of a `CompositionBound::Stated` that the re-grep finds, those lines
  only; the attempt record lists every file
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Hazards

- **Hot files.** `crates/effect-contract/src/lib.rs`, `crates/effect-compiler/src/prepare.rs`,
  `crates/builtins/src/tail.rs` and `tail_contract.rs`: this slice lands after #1465 and before
  #1466 (`STREAMS.md`); a slice in flight on them rebases.
- **Resource ceiling.** The browser's retained-bytes ceiling had 31 bytes of headroom after #1464;
  the added field costs about 16 bytes on wasm32 for one track's two tail entries. Measure it; if it
  does not fit, stop.

## Dependencies

- *Carry every node's tail bound in one node-neutral struct* (#1464): the carrier this slice
  changes.
- *State a fixed input section's decay, gains and flush stall* (#1465): the only node that states a
  stall today.
- It blocks *Certify the live input section's decay, peak gain and flush stall* (#1466), *Derive
  the live input section's tail gain and state its composition* (#1467) and *Define how node tails
  compose through gain in the graph extent* (#1379).
- Hot-file slots: `STREAMS.md`.

## Attempt record

### Attempt 1 (2026-10-08, implementer)

Anchors re-verified on `codex/d15-stream-g3` at `06915dc7c`: `CompositionBound` (`lib.rs:324`),
`FlushStall` (`:358`), `NodeTailBound::max` (`:255`), `tail_bound_consistent` (`:2645`),
`stated_composition` (`tail.rs:56`), `memoryless_channel` (`:101`), `math::tail`'s `stall`
(`:520`), `composition_values` (`tail_contract.rs:2084`), F3 (`:2403`), the disabled-filter test
(`:2442`), the forgery row (`prepare.rs:1196`). The re-grep for `stall:` found no other
`CompositionBound::Stated` literal.

**Names.** The proposed names are kept: `peak_stall`, `tail_stall`, `peak_clause`,
`tail_clause` (each names its clause, carries no version suffix, and matches `peak_gain` and
`tail_gain`). No `stall` field or alias is left.

**Changed.** `crates/effect-contract/src/lib.rs`: the two fields, the two `const fn` readers,
rule (g) in `tail_bound_consistent`, `max` by each stall's own maximum, and the docs (five
values, H1 as amended). `crates/builtins/src/tail.rs`: both writers state both stalls from one
value. `crates/effect-compiler/src/prepare.rs`: the forgery row's rename. Tests:
`tail_bound.rs` (S1 `readers_pair_each_stall_with_its_clause`; S3 crossed stalls in both orders;
the per-rate fixture states distinct stalls), `registry.rs` (S2
`a_tail_stall_above_the_peak_stall_is_refused`, fixtures `RULE_G`, `RULE_G_ZERO`;
`COMPOSITION` now has `sigma_t < sigma_p`; `launch_shape_stated` a `Zero` tail stall under a
`Level` peak stall), `tail_contract.rs` (`composition_values` reads through the two readers and
returns both stalls; F3 asserts both equal `ceil_mB(raw)` and reads `P^` from `sigma_t`; the
disabled-filter test asserts both stalls are one underflow). Docs: `EFFECT_CONTRACT_V1.md`,
`docs/derivations/1379-graph-tail-composition.md`, #1379's spec (H1, H2's carrier line, H3's
column split, H4 step 6) and the design note (H1, H2's code block, H3, H4 step 6, chain (i),
(iii), (v)). `STREAMS.md` has no status column for this row, so it is unchanged.

**Resource re-pins (each with its reason).** `CompositionBound` 32 -> 40 bytes, `NodeTailBound`
88 -> 96, `(Box<str>, NodeTailBound)` 104 -> 112 on 64-bit:
1. `BOXED_TAIL_ENTRY_BYTES` 104 -> 112 (`tools/audit/src/fixture_builtins.rs`, the layout fact,
   checked by `verify_pinned_native_resource_abi`).
2. `fixtures/builtins/v1/resources.jsonl`: both processor payload counts +16 per track (two tail
   entries per track, the payload's and the seal's, +8 each): 2,133 -> 2,149 (1 track), 8,604 ->
   8,668 (4), 143,229,429 -> 144,278,021 (65,537); the retained counts move by the same amount;
   the largest allocation and the allocation counts do not move. Length unchanged (2,370 B).
3. `fixtures/builtins/v1/MANIFEST.tsv`: `resources.jsonl`'s digest, from 2. alone.
4. The joined manifest identity `d0bf6198...` in `tools/audit/src/builtins_graph.rs`
   (`ACCEPTED_MANIFEST_SHA256`) and in `fixture_builtins.rs`'s
   `issue064_checked_corpus_is_read_only_and_complete` (with its re-pin note), from 3. alone.
5. Browser: `builtinRetainedBytes` measured **2,033 of 2,048** (+16, two wasm32 tail entries of
   +8; 15 bytes left). No ceiling is raised; `expected.json` is unchanged.
No PCM, meter, response or benchmark fixture moved; `audit capi`'s `pcm_digest` is
`cb10fbface44a3a4` (unchanged), 0 allocations, 0 syscalls; the wasm gates' digests pass unchanged.

**Mutation table** (each defect applied, the named tests run, the file restored; `EC` =
`cargo test -p effect-contract --all-features --test tail_bound --test registry`, `TC` = the
release `tail_contract` run):

| mutant | run | red |
|---|---|---|
| M1 `peak_clause` returns `sigma_t` | EC, TC | S1 only (`tail_contract` green: the fixed stalls are equal) |
| M2 `tail_clause` returns `sigma_p` | EC, TC | S1 only (`tail_contract` green) |
| M3 rule (g) dropped | EC | S2 only |
| M4 `Zero` ordered above a `Level` in rule (g) | EC | S2, plus the (e) and (f) tests (their `LAUNCH_SHAPE_STATED` twin, a `Zero` tail stall under a `Level` peak stall, is refused) |
| M5 `max` takes `tail_stall` from the peak stalls | EC | S3 |
| M6 `max` takes `tail_stall` from one side | EC | S3 (`crossed.max(left)`) |
| M7 `max` takes `peak_stall` from one side | EC | S3 (`left.max(crossed)`) |
| M8 fixed `tail_stall` left at one underflow | TC | F3 only |
| M9 disabled channel `peak_stall` at 0 mB | TC | disabled-filter test only |
| M10 rule (g) compares the stalls the wrong way round | EC | S2, plus the (e) and (f) tests (as M4) |
| M11 `peak_clause` returns `G_t` | EC | S1 |

M1 and M2 green on `tail_contract` confirm the spec's claim that only S1's distinct fixture
tells the readers' stalls apart; the readers have no other caller.

**Gates.** All green: the `test-debug-a` workspace command; the release `tail_contract` (17
passed); `check-effect-contract.sh`; `audit capi`; `check-builtins-fixtures.sh` (50 files);
`cargo test -p audit` (33 passed, after re-pin 4's second site; the first run was red only on it);
`check-workspace-policy.sh`; `check-realtime-policy.sh`; conformance fixtures `--check`;
`run-wasm-gates.sh`; the worklet chain (`build-web-audioworklet.sh --named-twin`,
`check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py
--artifacts`, `test-web-audioworklet.sh` with a private TMPDIR, nothing left in it);
`check-cross-targets.sh`; clippy `-D warnings`; `cargo fmt --check`.

**Open items (outside this slice's named sections, not edited).** Stale single-`sigma` text
remains in #1379's spec (D2a's `sigma` `Zero`; H2's canonical-text "both gains, stall";
H8's "B1 certifies `D`, `G_p` and `sigma` ... B2 ... states all four") and in the design note
(H2's canonical-text line, H7's gate `sigma`, H8's B1 line). #1379 (the canonical
plan text: one stall or two) and #1466 (B1) settle them.
