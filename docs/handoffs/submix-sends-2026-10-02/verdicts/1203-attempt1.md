# #1203 *Tap a submix strip at any of the seven send points*: Sol verdict, attempt 1

- Reviewed: `git diff b0c7e29fb 6a4d729d9` (branch `codex/batch-submix-k1`, worktree
  `/home/bl/misofm/wt-submix-k1`): 29 files, +860/-120.
- Binding: `AGENTS.md`, the decision-13 ruling (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`,
  "Wire identity") and `.github/ISSUE_SPECS/1203-tap-a-submix-strip-at-any-of-the-seven-send-points.md`,
  including its Attempt 1 record and deviations.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1203` shows it OPEN, and
    its title matches the spec's H1, which is correct in batch mode.
  - I exported `6a4d729d9` with `git archive` to `/tmp/claude-1002/v1203/src`, under a throwaway
    local git so the policy scripts see tracked files. I built with my own `CARGO_TARGET_DIR` and
    `CARGO_PROFILE_{DEV,TEST,RELEASE}_DEBUG=0`.
  - I exported the parent `b0c7e29fb` to a second tree for the cross-commit migration proof.
  - Mutations and scratch tests ran in the export and were reverted. Before the final gates I
    `diff -r`'d the export against a fresh archive of `6a4d729d9`: IDENTICAL.
  - I deleted all scratch afterwards.
- My scratch tests are kept in `submix-verdicts/1203-attempt1-verifier-scratch.rs`, so the root can
  adopt them.

## Verdict: PASS

There is no BLOCKER and no MAJOR.

- D1-D5 are present, at the sites and in the shapes the spec freezes.
- Every gate I could run passes when I re-run it, and the counts match Terra's record exactly.
- The breaking grammar change is mandated:
  - The spec's D1/D2 and the ruling ("`submix_output` becomes an unknown token. Wire tag 2 ... is
    kept, and its tap becomes required. ... a retired code is refused, never reallocated") require
    it.
  - No compatibility spelling was owed.
  - The old JSON spelling and the old tapless tag 2 are both **refused, never reinterpreted**.
- The migration is proven to move no node and no bit (proof below).
- The `COMPLETE_SCHEMA_HASH` re-pin is attributable to D5 alone (proof below).

There are two MINOR findings and five NITs. None of them needs another attempt.

## Gates (re-run by me on `6a4d729d9`, x86-64-v3, native W8)

| Gate | Command | Result |
|---|---|---|
| 1, 2, 3, 6 | `host-core --test submix_strip` (inside test-debug-a) | all ok (15/15 in the file) |
| 4 | `session --test invalid_matrix submix_source_spellings_refuse_at_their_index_paths`; `protocol session_wire::tests::a_tapless_submix_source_is_refused_in_every_shape_that_carries_one` | ok; ok |
| 5 | `graph-compiler --test compile_shapes a_loop_through_a_bus_tap_is_a_graph_cycle` | ok |
| 7 | the test-debug-a workspace command (exact excludes and features from DESIGN section 7) | exit 0. 98 binaries: 1140 passed, 0 failed, 9 ignored |
| 7 | the test-debug-b command, then `conformance_fixtures -- --check` | exit 0 and exit 0. 145 binaries: 787 passed |
| 7 | `check-protocol-wasm-parity.sh` | `issue-005 Wasm golden parity: ok (simd128)`, with the new hash |
| 7 | `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | exit 0 |
| 7 | `check-graph-determinism.sh` | PASS (100/100). The script hard-codes `target/debug`, so I symlinked my target dir. |
| 7 | `graph_fixture -- --check` | exit 0 |
| 7 | `check-console-fixtures.sh <release>/session_validator`; `check-builtins-fixtures.sh` | ok; ok (50 files) |
| 7 | `cargo fmt --all -- --check`; workspace clippy `--all-targets --all-features -D warnings` | both exit 0 |
| 7 | the four `check-`/`test-` policy pairs (session, protocol-control, graph, workspace); `check-realtime-policy.sh` | all ok |
| 7 | `run-aarch64-tests.sh`'s no-silent-skip scan (its exact `rg --pcre2` pattern and roots) | no match (rg exit 1) |
| 7 | `run-aarch64-tests.sh debug` | **Not runnable here.** The host is x86_64 with no aarch64 runtime, so this runs in CI's `aarch64-debug` job at the batch push (NIT-5). |
| extra | `session_validator validate --canonical worked-session.json` | all five stages PASS. The canonical output is byte-identical to the file. |
| extra | head's `session_validator validate` on the **parent's** `worked-session.json` | stage 2 FAIL with exactly one diagnostic: `schema.invalid_enum $.routes[0].source.kind ... expected track or submix` |

- Every touched file is on the authorized-paths list.
- `CONTROL_PROTOCOL_CONFORMANCE.md` gained a reason clause next to the hash. #1199 and #1202 did
  the same, and deliverable 2 asks for the sentence.

## Adversarial checks

**The migration moves no node and no bit (spec D3, deliverable 4).**
- `stage(SendTap::PostPan)` is `TrackStage::PostMatrix`, the stage `SubmixOutput` mapped to on the
  parent, so the route and sidechain graph nodes are unchanged.
- No other consumer of `RouteSource` branches on the submix variant. I checked with `git grep` over
  every non-test source: `pdc.rs` and the runtime are generic, and `control_provider.rs` matches
  tracks only.
- Cross-commit proof: a scratch `v1203_migration_digest` printed the graph SHA-256
  (`GraphCompiler::sha256`) and an FNV-1a-64 of 16 rendered blocks of PCM bits. The feeds were
  deterministic noise.

  | Session | Parent `b0c7e29fb` (`submix_output`) | Head `6a4d729d9` (`submix`, `post_pan`) |
  |---|---|---|
  | `worked-session.json` (`band-main` route) | graph `3ec00106…0531`, pcm `9b209be7a96d2d34` | identical |
  | the same, plus a track keyed from the band's end (a **sidechain** migration) | graph `f7fbec76…22a9`, pcm `c5023ee13388c32a` | identical |

**`COMPLETE_SCHEMA_HASH` re-pin legitimacy.**
- I reverted D5 alone, so the corpus's `SetRouteSource` value is `route.source.clone()` again and
  the codec change stays in place.
- The conformance test then computes `0xc0f6ecedbf50920a`, the old pin, exactly.
- So the codec change alone moves nothing, as the spec predicted. The move to `0xa1dcc56f2e4a48f9`
  is caused solely by D5's tapped tag-2 source, which is now in the hash and in the wasm parity
  gate.
- The four sites agree. The parity script's `corpus.len() == 46` is unchanged.

**Retirement rules.**
- `submix_output` falls to the unknown-kind arm, `schema.invalid_enum` at `.kind`, with or without
  a `tap`. Gate 4 pins both shapes at both index paths.
- Wire tag 2 keeps its meaning (a submix source), and nothing is renumbered.
- An old tapless tag 2 fails `one_spec!` with `InvalidTlv` on all three shapes, so it is refused,
  not read as `post_pan`.
- An old peer could never have sent tag 2 *with* field 3 (the parent's `SUBMIX` spec refused it). So
  no previously valid message is reinterpreted.
- Tap codes 1-7 are shared and unchanged.

**Taps at all seven points: the right stage, the right PDC, bus-to-bus, sidechains and cycles.**
- Gate 1 (8 seeds, a launch rate drawn per seed, the oracle's seven renders asserted pairwise
  different) is a real equivalence, because the oracle has no submix.
- My scratch tests are all green on head, at native W8:
  - `v_bus_to_bus_taps_match_track_to_bus`, 6 seeds: three taps of one bus (`pre_fader`,
    `insert_return`, `post_input`) into a second processed bus are bit-identical to the same taps of
    a track fed the sum.
  - `v_sidechain_from_a_latent_bus_tap_is_compensated`, 6 seeds: a bus with a limiter insert keys
    track `x` from its `insert_return`. The output latency and the inserted delays match the track
    oracle (888 or 966 samples on `x`'s keyed-effect main edge), and the bits are identical.
  - `v_tap_cycles_through_sidechains` checks four shapes:
    - A bus keyed from its own `insert_return`, `pre_fader`, `post_fader` or `post_pan` is refused
      with `graph.cycle`. Keyed from its own `input`, `post_input` or `insert_send`, it prepares,
      which is correct because those taps are upstream of the insert.
    - A contributor keyed from the bus it feeds, at the bus's `input`, is refused with `graph.cycle`.
    - An `input`-tap loop between two buses is refused with `graph.cycle`.
    - A bus's `pre_fader` routed into its own input is refused with `graph.cycle`.
  - `v_delayed_bus_input_taps_match_a_delayed_track`, 6 seeds: with delays of 37/5 samples, the
    `input`, `post_input` and `post_pan` taps match a delayed track. The bus delay is at the `input`
    tap, as a track's is (#1201 D1).
  - `v_muted_bus_pre_fader_tap_is_ungated`, 6 seeds: on a muted bus, the `pre_fader` tap is
    non-silent and equals the muted track's; `post_fader` and `post_pan` are silent. D4 holds.

**Realtime and portability.**
- No runtime code changed. Gate 6 shows exact zero allocations after block 0 (render audit and
  thread-scoped counters), over all seven taps.
- No target-specific code was added. The tap mapping is graph-level and width-independent.
- The 4-lane run is CI's at the batch push.

**Deviations.**
- Two latency-free EQ slots in gate 1: accepted. With an empty `post_insert`, `insert_return` and
  `pre_fader` would carry the same audio, and the gate demands that every tap differ.
- Fixed strip parameters with drawn contributors: accepted. A drawn strip could make "every tap
  differs" fail by luck.
- The gate-4 cases sit in a new `invalid_matrix` test: accepted. The category tests pin their case
  counts by name.
- Gate 5's path is spelled `$.submixes`: see MINOR-1.
- Gate 6's mutation is non-specific: see NIT-2. The deviation is disclosed honestly.

## Mutations

The implementer's own mutations are recorded in the spec. These are mine, each applied to the export
and reverted:

| # | Mutation | Red |
|---|---|---|
| M1 | `route_source_node`: a submix `input` tap maps to `PostInputBuiltins` (submix only) | gate 1 (`tap Input`); `route_helpers_map_every_typed_variant_to_its_graph_node`; my S1 |
| M2 | sidechain edges only: a submix `pre_fader` source maps to `PostFader` | **gate 2 alone** |
| M3 | parser: drop the `submix` arm's `reject_key("track_id")` | **nothing** (NIT-3; the parent's equivalent check was untested too, I confirmed it at `b0c7e29fb`) |
| M4 | wire decoder, tag 2: copy-paste reads the tap from field `TAG` | gate 4 (wire), `every_send_tap_tag_is_typed_and_canonical`, both round-trip tests |
| M5 | canonical writer: every submix source is written `tap: "post_pan"` | gates 1, 2, 3 and my S1-S5, via `document()`. **No `session`-crate test** (NIT-4). |
| H1 | D5 reverted (codec kept) | `frozen_corpus_bytes...` computes the old `c0f6ecedbf50920a`, which is the re-pin proof |

## Test value (one sentence each)

- **Gate 1** turns red if any submix tap maps to the wrong stage, or if the canonical writer loses a
  submix tap (M1, M5). No other test renders a tapped, processed bus.
- **Gate 2** turns red if sidechain edges map a bus tap differently from routes (M2: unique).
- **Gate 3** turns red if a bus tap's arrival is taken from the strip's end. Every mapping defect I
  found that does this also reddens gates 1, 2 and 5 (NIT-2), so its standing value is the PDC pin
  for bus sends.
- **Gate 4 (session)** turns red if `submix_output` is read as an alias, or if a tapless submix
  source gets a default tap, on either the route or the sidechain path.
- **Gate 4 (wire)** turns red if any of the three decoders that share the spec accepts a tapless
  tag 2 (M4 and the implementer's default-tap mutation).
- **Gate 5** turns red if cycle detection skips a route edge that leaves a mid-strip stage.
- **Gate 6** turns red if a mid-strip bus tap allocates on render. The defect is not tap-specific
  (NIT-2).
- **The extended `every_send_tap_tag_is_typed_and_canonical`** turns red if the codec drops or
  rewrites a submix tap code (M4).
- **The rewritten `route_helpers_map...`** turns red if a submix tap maps to a stage other than the
  table's (M1).

## Findings

### MINOR-1: gate 5 names the route only in an internal field; hosts see `graph.cycle $.submixes`

- What happens:
  - Gate 5 asserts that the routes appear in `cycle_edge_paths`. But `cycle_edge_paths` reaches no
    host: `git grep` finds it only in the graph crates.
  - The host-visible failure (host-core, so also capi, host-web and `session_validator`) carries
    only `code` and `path`.
  - For gate 5's shape, and for a bus `pre_fader` routed into its own input, the text is
    `graph.cycle\t$.submixes`: no route and no strip ID. I saw this from `prepare_host_runtime` in
    `v_tap_cycles_through_sidechains`.
  - An `input`-tap loop happens to read `$.routes[id=ba].source`, and a sidechain loop reads
    `$.submixes[id=bus].sidechain` or `...simd2.effects[id=desk-tone]`.
- Why: this is #1200 verdict MINOR-1, still open in the K1 ledger. `compile.rs` takes
  `cycle.1.first()`, and the witness now often starts on a strip chain edge whose sealed path is
  the bare collection path. #1203 makes the failure easier to hit, because every tap opens a new
  loop shape, and its gate text says "naming the route".
- Fix (in-batch, diagnostics only, no digest moves):
  - Apply the ledger item. In `compile.rs`, take as `path` the first `cycle_edge_paths` entry that
    starts with `$.routes[` or ends in `.sidechain`, and fall back to the first entry.
  - Then tighten gate 5 to `assert_eq!(cycle.path, "$.routes[id=ab].source")`, or whichever route
    edge the rule picks, so the gate pins what a host shows.
  - The deviation was disclosed, and the graph-level diagnostic does name the route, so this is not
    a FAIL.

### MINOR-2: D4 ("a pre-fader tap is not gated by the fader mute") has no test

- `SESSION_SCHEMA_V1.md` now states it, and the spec freezes it as D4.
- But no gate mutes the strip. Gate 1's oracle would agree with a defect that silences the
  upstream of a muted strip for tracks and buses alike, and I found no existing test of a muted
  strip's pre-fader send, track or bus. I searched every test that sets a mute.
- The plausible defect is a skip-on-silence optimization that treats a muted fader as a silent
  strip.
- Fix: adopt `v_muted_bus_pre_fader_tap_is_ungated` from the scratch file. It is green on head and
  asserts both the bit equality with a muted track and that the `pre_fader` tap is non-silent.

### NIT-1: the bus delay at the `input` tap is untested

- Gate 1's strip has `delay_samples = 0`.
- The cheapest fix is to set `tap_strip()`'s lanes to delays of 37 and 5 samples. Gate 1 then
  covers the delay at all seven taps, and "every tap differs" still holds. Alternatively, adopt
  `v_delayed_bus_input_taps_match_a_delayed_track`.

### NIT-2: correct the gate-3 and gate-6 test-value sentences

- The record's gate-3 sentence implies a catch of its own. Every arrival-from-the-end mapping defect
  also reddens gate 1, and `pdc.rs` has no strip-specific code that a unique defect could live in.
- Say that gate 3 is the PDC pin for bus sends and that its mapping defect is shared with gates 1,
  2 and 5.
- Gate 6's non-specificity is already disclosed. Keep it, because the spec mandates it.

### NIT-3: a submix source carrying `track_id` is never tested

- M3 survives. The parent's `submix_output` cross-field rejection was also untested.
- Add one `parse_case` to gate 4's test: `{ kind = "submix", submix_id = "bus", tap = "pre_fader",
  track_id = "vocal" }` gives `UnknownField` at `<path>.track_id`.
- Optionally add the track arm's `submix_id` mirror.

### NIT-4: the session crate never round-trips a non-`post_pan` submix tap

- M5, a writer that flattens every submix tap to `post_pan`, is caught only through host-core's
  `document()` path.
- One sidechain or route with `SendTap::PreFader` in `canonical_schema.rs`'s full-surface model
  would pin it where it lives.
- The writer corpus is generated from `canonical.rs`, so leave that model at `post_pan` to keep the
  corpus unchanged.

### NIT-5: the 4-lane run is deferred to CI

- `run-aarch64-tests.sh debug` could not run here; this matches the record's "at batch push".
- Verify `aarch64-debug` at the K1 push.
- The tap mapping is width-independent, so I expect no surprise.

## Batch note (not a finding)

- The SDK still spells `submix_output` at the sites the spec lists. They stay red until #1205 inside
  K1, as the spec says.
- The K1 push must not happen before #1205 lands, or the SDK legs fail.
- The author-session skill's prose (`{"id":"buss"}` for a submix) is also #1205's to refresh.
