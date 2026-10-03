# #1245 attempt 1 verdict: Ride VCA groups live in the browser

**Verdict: PASS.** There is no BLOCKER and no MAJOR. The admission is correct in every batch shape
I could build: it is all or nothing, and no ack precedes a drop. A settled live session lands on a
fresh plan's bits. Ramps carry the gesture's window on every derived record. The bounds, the boot
count and the timing claim all reproduce. Two MINORs must be fixed before close: four plausible
ramp and refusal-index defects pass every committed test. Verified test patches close both. There
are also three NITs and two INFOs.

- **Implementation:** `86c050075` on parent `1db0aacc3` (#1244), branch `codex/batch-vca`.
- **Review copies:** `git archive` exports of `86c050075` under `/tmp/claude-1002/v1245/`: `src`
  for the gates and `mut` for probes and mutations, each with its own target directory. All
  deleted after the review. I never touched the worktree, the branch or GitHub.
- **Host:** x86-64-v3 AVX2, 32 cores. node v22.23.2.
- **Probes and mutation driver:** `1245-attempt1-verifier-scratch.rs`, next to this file.

## Gates re-run on `86c050075` (all exit 0)

| Gate | Result |
|---|---|
| test-debug-a (the spec's gate 10 `cargo test` command, `--no-fail-fast`) | 115 binaries: 1,286 passed, 0 failed, 9 ignored (same as the record) |
| `cargo fmt --all -- --check`; clippy `--workspace --all-targets --all-features -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | clean |
| Kind vocabulary `--self-test` and plain | 32 red mutations; agrees |
| Reason vocabulary `--self-test` and plain | 20 red mutations; agrees |
| `build-web-audioworklet.sh --named-twin B A` | **ARTIFACT CHANGED**, reproducible. Shipped module `731f65cb…f5e` (2,848,600 B) and named twin `77d7f7cc…a93`; both match the record byte for byte |
| `check-parameter-metadata-v1.py` and `check-abi-layout-v1.py`, `--self-test` and on the artifact JSON | ok |
| `check-web-audioworklet.sh A B/…named.wasm` | ok. command-submit closure 64, callgraph clean; the new `LiveVcaState` and `set_vca_mute` owners are in the trap-audited closure; render closure 8, 13 kernels |
| `check-browser-expected-resources.py --artifacts A` | ok, 32 red self-test mutations, no re-pin |
| `test-web-audioworklet.sh` | ok, including the on-disk `FUTURE_TAP = 15` and kind-18 drift mutations |
| `check-sdk-generated.sh A`, `check-sdk-types.sh`, `check-sdk-headless.sh A` (357 pass, 0 fail), `sdk-package.sh check A` | ok |
| Browser legs: `npm run qualify -- --check-matrix --self-test-mutations`, SDK source-bundle mode (CI's), private PulseAudio null sink; the stray `sdk/dist` deleted before and after | chromium 151.0.7922.34, firefox 153.0 and webkit 26.5: "all qualification gates passed" |
| `check-`/`test-` pairs for `host-core-policy`, `realtime-policy` and `workspace-policy` | all six rc 0. The two known "directed fault unexpectedly passed" lines (`mutant-population`, `isa-build-late`) also appear on the base |
| `check-cross-targets.sh` | rc 0. host-core is still at its 4-call `memset_pattern16` ceiling, and there is no new row |
| `.d.ts` mirror | `cmp`-identical to `sdk/src/browser/shipped-host.d.ts` |

## What holds up under probing

### Admission, transactions and the acked-batch question

- **The order is the spec's.** Room is checked for every staged entry before any push. The VCA
  fader pass adds to `command_wanted[strip_count + strip]` for each record it stages. So the room
  check sees every VCA-derived fader, mute and follow record, and the VCA, solo and route mirrors
  move only under their shadows.
- **`admit_commands` commits or rolls back `vcas` together with `solo` and `routes`.** No ack can
  precede a drop:
  - Records are pushed only after every destination's room is known.
  - Mirrors commit only on `Ok`.
  - A refusal rolls back all three mirrors.
- **The only push-after-check failure is the pre-existing `RESULT_INTERNAL` arm.** That arm is
  unreachable single-threaded.
- **Mutations S15 (no VCA rollback) and the record's "never committed" row are both red.**

### Semantics: settled identity

My differential is `sol_probe_randomized_settled_differential`. I ran it at 400 seeds in release,
twice: once with the user kinds at ramp 0, as the committed gate has them, and once with their ramps
drawn from {0, 64, 300}. Both runs are green.

- **Session:** 2 to 6 VCAs over 4 levels with multiple parents (169 sessions had diamonds), tracks
  and submixes as members, and offsets that include both domain edges.
- **Batches:** 1 to 10 records mixing kinds 16, 17, 3, 4 and 9 on every channel. VCA ramps are drawn
  from {0, 1, 64, 200, 300}. After a kind 17, a kind 4 on a member that VCA reaches is biased in.
- **Refusals:** one batch in five ends with a refused record placed after its VCA records:
  - an unknown VCA;
  - a mute value of 0.5;
  - a 30 dB fader.

  Each refusal must leave the whole snapshot unchanged: fader and send room, the VCA mirror, the
  owner's effective and emitted mutes, the send mirror and the follow lanes. It must also leave
  every transaction closed.
- **Checks after every admitted batch:**
  - every reached strip's `effective_db` equals the test's own top-down reference, bit for bit;
  - every strip's effective mute equals `own || vca || solo-term` and its emitted mute equals its
    effective mute;
  - the owner's VCA term equals `vcas.vca_mute` on every reached strip, which is the lazy refresh's
    invariant at commit;
  - `fader_delta` owes nothing;
  - the follow mirrors equal those of a host booted fresh from the edited session;
  - after the ramps settle, 2 blocks are bit-identical to that host.
- **Reach:**
  - 2,545 admitted batches;
  - 479 refusals after a kind 17;
  - 1,765 batches with a kind 4 after a kind 17;
  - 241 of those kind 4s split per lane;
  - 702 batches with a solo after a kind 17;
  - 458 split kind 3s;
  - 842 batches with both a kind 16 and a kind 3 on a reached strip;
  - 1,413 batches with both a ride and a mute;
  - 844 batches that moved a follow mirror.

### The lazy VCA-mute refresh (A1d) is equivalent to D3 for every reader

- **The only reader inside the batch loop is the kind 4 arm.** No other arm reads the strip-mute
  owner, and kind 4 refreshes its own strip before it composes:
  - `set_solo` and `solo_safe` read only the solo bits;
  - the route arms build from the route mirror, which only the follow pass moves;
  - kind 3 does not read the owner at all.
- **The readers after the loop come after the one full refresh.** That refresh sits at the head of
  the coalescing pass, which runs whenever `vca_mute_seen`. The coalescing pass and the follow pass
  are its only readers.
- **Every batch shape the brief named is covered by the differential above, with per-batch mirror
  checks:**
  - kind 17 then kind 4 on the same strip, both lanes, one lane and split (1,765 batches);
  - kind 17 then solo (702);
  - kind 17 then a follow derivation (the follow mirrors are compared against a fresh host);
  - a refusal mid-batch after a kind 17 (479).
- **The committed `a_member_mute_after_a_vca_mute_in_one_batch_composes_with_it` is the only test
  that sees a missing per-strip refresh.** I reproduced the record's M15 (R15 below). The settled
  state is the same without the per-strip refresh; only the kind 4 record's ramp differs, and that
  test checks it.

### Ramps: no click (`sol_probe_ramps_match_direct_member_moves`, 400 seeds, green)

- **Comparison:** the live VCA host against a VCA-free host. The reference's own faders and mutes
  are the live effective values. For every lane whose effective value changed, it is told directly a
  kind 3 or kind 4 with the same ramp.
- **Coverage:** every block from the batch through the ramp is compared bit for bit, not only
  settled blocks. Rides, mutes, and rides plus mutes in one batch use ramps of 64 to 500 samples.
  There were 1,749 batches with records.
- **Result:** the VCA fader pass, the coalesced mute records and the follow records all carry the
  gesture's window.
- **Further probe** (`sol_probe_split_member_move_and_last_ride_ramp`): a split kind 3 on a member
  ramps like two direct lane moves, and two rides in one batch take the last ride's ramp, as D3 says.

### D3's long-ramp refusal

`sol_probe_long_vca_mute_ramp_refuses_at_first_coalesced_index`:

- A kind 17 with ramp `INDEXED_RAMP_LENGTH_MAXIMUM + 1` that moves a following send is refused
  whole with `domain`.
- The refusal names the batch's first kind 9 or 17 record. I checked behind fader moves, behind a
  solo and behind an earlier kind 17.
- Nothing moves.

### Bounds, boot count and budget (amendment A1)

- **The boot count runs before any table that grows with the pairs.** `browser_vca_shape` runs
  after `compile_host_model` and before `compile_ready`. Session validation, normalization and
  `compiled_session_shape` build no per-pair table: validation is Tarjan over VCA edges, and the
  only `vca_reach()` and `effective_strip_faders()` callers are preparation, inside `compile_ready`,
  and `LiveVcaState::try_new`. The shape allocates per VCA (a 256-bit set and a counter) and per
  membership entry, never per pair.
- **The count equals the live reach.** It is asserted against the live state in the committed
  gate 1 and in the bound test, which the record's M14 turns red.
- **Boundaries:**
  - 16,384 pairs boot and 16,385 refuse with `web.vca.reach_pairs`;
  - 256 VCAs boot and 257 refuse with `web.vca.maximum_vcas`;
  - each refusal is `RESULT_REFUSED_BUDGET` with the exact fixed diagnostic.
  - My S8 (`>=` instead of `>`) is red.
- **Retained bytes:** `BrowserVcaShape::retained_bytes` equals `LiveVcaState::retained_bytes`
  exactly. The formula and the state both use the target's `size_of`, so the equality holds on
  wasm32 too.
- **Exact bridge rows:** the state, its shadow and `2 * reached * size_of::<StagedCommand>` are
  charged, as the retained-budget test checks.
- **Timing (A1b), re-measured:**
  - **Native release, the record's batch, 40 warm runs, three times:** median 150.0, 150.2 and 150.0
    us; maxima 165.4, 158.8 and 159.5 us. The record says 0.153 ms. The committed test's own
    `eprintln` reads 152.8 us.
  - **Shipped simd128 module in node 22.23.2, the record's script:** warm median 0.185 to 0.236 ms,
    p99 0.37 to 0.58 ms. These two runs shared the machine with the browser legs; the record says
    median 0.18 ms.
  - **Worst case:** the measured batch is the worst shape I can find at the bound. Per record, kind
    3 costs `2 * |reach|` and a kind 4 refresh `|reach|`, both at most 256. The two passes cost
    about three times the pair count per batch.

## MINOR-1: no committed test defends the ramp on a derived record (fix before close)

The product outcome is that every member follows "through its existing declicked fader ramp". The
implementation does this (see "Ramps" above). But every committed test either compares settled
blocks or uses ramp 0 on the path concerned:

- gate 1 compares only after its ramps settle;
- gate 3 stages nothing;
- gate 5 skips its ramp blocks.

So four plausible defects pass the whole committed host-web suite. Each one is an audible hard
switch or a wrong window:

| # | Mutation | Committed suite | My probes |
|---|---|---|---|
| S1 | the VCA fader pass stages `smoothing_samples: 0` (every VCA ride clicks on every member) | **green** | red: `sol_probe_ramps_match_direct_member_moves` |
| S2 | kind 17 does not set `coalesce_smoothing` (a VCA mute hard-switches members and their following sends) | **green** | red: the same probe |
| S3 | the kind 3 split records drop the window (a member move under a one-sided VCA clicks) | **green** | red: `sol_probe_split_member_move_and_last_ride_ramp` |
| S23 | the VCA fader pass takes the first ride's ramp, not the last's (D3) | **green** | red: the same probe |

**Fix:** commit the two probes as tests. Both are verified green on `86c050075` and red on these
mutations:

- `sol_probe_ramps_match_direct_member_moves`, renamed for example
  `a_vca_ride_and_mute_ramp_as_direct_member_moves`. 8 seeds keep it cheap: 40 seeds took about a
  second in debug.
- `sol_probe_split_member_move_and_last_ride_ramp`.

Add a `MUTATIONS.md` row for each. This is the same class as #1242 attempt 2's MINOR-1 (M3).

## MINOR-2: D3's long-ramp refusal and its wire index are untested (fix before close)

D3 freezes a rule: "a kind 17 whose smoothing exceeds `ROUTE_RAMP_LENGTH_MAXIMUM` and moves a
following send refuses the whole submission (`domain`) at the coalesced record's wire index (the
batch's first kind 9 or 17 record)". No committed test reaches it.

My S12 survives the committed suite: kind 17 never sets `coalesce_first_wire_index`, so a refusal
names index 0. Gate 4's mute refusal sits at index 0, where both forms agree.

**Fix:** commit `sol_probe_long_vca_mute_ramp_refuses_at_first_coalesced_index`. It is green on
`86c050075` and red on S12.

## NITs

- **NIT-1: the quantum assertion never runs in CI.**
  - `the_browser_bounds_vca_reach_and_a_batch_at_the_bound_fits_a_quantum` asserts
    `elapsed < quantum` only when `!cfg!(debug_assertions)`.
  - CI runs host-web's lib tests only in debug (test-debug-a, aarch64-debug). Nightly's release
    `--lib` step filters by other names.
  - So the test-value clause "red if a batch at the bound costs a quantum" is never enforced.
  - **Fix**, either of:
    - split the timing half into an
      `#[ignore = "release-mode budget; runs nightly"] ..._in_release` test, per
      `maximum_document_dense_invalid_boot_finishes_under_one_second_in_release`, and leave
      nightly's wiring to a follow-up (`nightly.yml` is outside the authorized paths);
    - drop the clause, and keep the number as record evidence.
- **NIT-2: the A1c projection check never binds, and its doc overstates the transient.**
  - **The check is never the binding refusal.** I bisected the smallest admitted budget for two
    sessions:
    - the bound chain (64 x 256): the boundary refusal is `host.budget.retained_exact`, at
      3,537,847 B;
    - a 256-VCA lattice where each VCA holds every earlier one (32,640 VCA edges, the most
      membership entries the bounds allow): it is preparation's `host.resource.limit`, at
      35,338,138 B.
  - **Consequence:** my S9 (the check removed) survives every test. That is acceptable for a
    defensive pre-check, but the record and the doc should say it is dominated at these bounds.
  - **The doc over-counts.** `transient_bytes`' doc counts "two `vca_reach()` results alive at
    once", but `try_new` runs `drop(reach_lists)` before `effective_strip_faders()`. The bound
    therefore over-counts one reach result. It stays conservative, but the wording should say so.
- **NIT-3: a clamped member's own move always stages (#1244 INFO-1).**
  - Kind 3 on a clamped member (own +20 dB in a +24 dB VCA, moved to +21 dB) stages an unchanged
    +24 dB target.
  - Probe `sol_probe_clamped_member_own_move` shows the 300-sample retarget renders bit-identically
    to a fresh plan through and after the ramp.
  - That matches kind 3 on an unreached strip, which always stages. D3's literal text allows it, and
    it moves no bit.
  - **Fix:** record the choice in the attempt record, as INFO-1 asked.

## INFO

- **INFO-1: the first wasm submission is over a quantum.** The first submission on a fresh wasm
  engine took 3.0 and 4.3 ms in my two runs. That is V8 lazily compiling the command path, for any
  kind; the record's deviation 3 already says so. It is the same class for every kind, not VCA
  work. A host that wants a clean first gesture can warm the path with a no-op batch.
- **INFO-2: the bounds still need the owner's ruling.** The A1 bounds (256 VCAs, 16,384 pairs) are
  a planner decision subject to owner review, and the code and doc say so. Nothing here depends on
  the exact numbers beyond the measured cost.

## Deviations: all accepted

- **A1d (the lazy refresh)** is equivalent to D3 for every reader, by the argument and the probes
  above.
- **The unreachable `unsupportedKind` arm** (no command staging without live controls) follows the
  route-kind precedent. `controls` is `handles.strip_controls`, the same predicate that leaves
  `vcas` empty.
- **The V8 first-call cost** is INFO-1.

## Vocabulary

- **No value is reused or renumbered.**
  - Kinds 16 `vcaFaderDb` and 17 `vcaMute`, and reason 14 `unknownVca`, are appended in every
    spelling.
  - The decode whitelist, the JS set and table, both `.d.ts` copies, both generators, both
    schema-gate lists, both self-test fixtures and the regenerated `sdk/assets` and
    `sdk/src/generated` all agree.
- **The SDK eval's exclusion is narrow.** `kindsAwaitingSdk` names exactly the two VCA kinds. Any
  other unmapped kind still fails, which is the record's row.
- **The self-test anchors moved correctly.** No mutation "matches nothing": 32 and 20 red, as
  recorded.

## Mutations

### Mine

Applied one at a time in the `mut` export over the whole host-web lib suite plus my probes, with the
file restored after each run.

| # | Mutation | Result |
|---|---|---|
| S1 | VCA fader pass ramp 0 | committed suite **green**; red only in my ramp probe (MINOR-1) |
| S2 | kind 17 leaves `coalesce_smoothing` | committed suite **green**; red only in my ramp probe (MINOR-1) |
| S3 | kind 3 split records ramp 0 | committed suite **green**; red only in my split probe (MINOR-1) |
| S4 | the coalescing refresh only touches strips with a muted reach (an un-mute never releases) | red: gates 1, 2, 5 and my probes |
| S5 | kind 3 non-split stages the left lane's value for a `Right` command | red: gate 1 and my differential |
| S6 | kind 3 non-split omits `record_emitted_db` | red: gate 1 and my differential |
| S8 | pair bound `>=` | red: the bound test |
| S9 | A1c projection check removed | **green** everywhere (NIT-2: never binding) |
| S12 | kind 17 leaves `coalesce_first_wire_index` | committed suite **green**; red only in my long-ramp probe (MINOR-2) |
| S15 | `ready.vcas.rollback()` removed | red: gates 4 and 6 and my differential |
| S19 | VCA fader pass skips the last strip | red: gates 1 and 7 and my probes |
| S20 | VCA fader pass skipped when the batch also mutes a VCA | red: gate 1 and my probes |
| S22 | a one-lane VCA mute applied to both lanes | red: gate 1 and my probes |
| S23 | VCA fader pass takes the first ride's ramp | committed suite **green**; red only in my split probe (MINOR-1) |
| S24 | the kind 3 split never happens (one record carrying the left lane's value) | red: gate 1 and my differential |

### The record's, re-run

| Row | Result |
|---|---|
| R15 (M15: a later kind 4 does not refresh its strip's VCA term) | red: `a_member_mute_after_a_vca_mute_in_one_batch_composes_with_it` only, as recorded |
| R10 (M10: the staging does not grow by the reached strips) | red: gate 7 and the retained-budget test, as recorded |

## Test value (one sentence each)

- **`a_live_vca_ride_lands_on_a_fresh_plans_bits`:** a randomized settled differential, judged by
  reach (its reach counters are asserted). It is the only committed test red on a kind 3 that
  stages the wrong lane's value, skips the split, or forgets its emitted mirror (S5, S6, S24).
- **`a_member_move_and_a_vca_move_compose`:** turns red if the clamp is stored as the member's own
  value, if a member move overwrites the VCA term, or if a VCA un-mute clears a member's own mute.
  It is the spec's named gate 2, deterministic and readable. It overlaps gate 1 (S4).
- **`a_vca_move_that_changes_no_effective_value_stages_nothing`:** turns red if composition
  re-emits an unchanged target (the record's M3, this test only).
- **`a_vca_batch_that_overfills_a_queue_is_refused_whole`:** turns red if a record is pushed, or a
  mirror commits, before every room check, or if a success never commits (the record's
  "never committed" row, this test only).
- **`a_vca_mute_silences_member_sends_and_survives_solo`:** turns red if the VCA term misses the
  follow composition or un-soloing clears it, at the spec's named gate 5 fixture. It overlaps gate 1
  (S4).
- **`vca_records_are_addressed_and_shape_checked`:** turns red if the index is checked against
  another table, misnamed `unknownRoute`, or the `effect_index`/`parameter_id` rule is dropped (the
  record's M8 and M9, this test only).
- **`the_decode_staging_holds_a_full_batch_and_its_vca_records`:** turns red if the staging does
  not grow by the reached strips (R10).
- **`vca_rides_and_mutes_admit_and_render_without_allocating`:** turns red if VCA admission or its
  passes allocate (`allocations == 0`, `deallocations == 0`; the record's M11).
- **`the_exact_retained_budget_charges_the_vca_state`:** turns red if the VCA state or the staging
  growth is left out of the exact retained rows (R10, the record's M12).
- **`the_browser_bounds_vca_reach_and_a_batch_at_the_bound_fits_a_quantum`:** turns red if either
  bound is unenforced or off by one, or if the boot count or projection disagrees with the live
  state (S8, the record's M13, M14 and M18). Its timing half is not enforced by CI (NIT-1).
- **`a_member_mute_after_a_vca_mute_in_one_batch_composes_with_it`:** turns red if a kind 4 after a
  kind 17 in one batch composes without the new VCA term (R15, this test only).

## For close

- Apply MINOR-1 and MINOR-2: commit the three probes as tests, with `MUTATIONS.md` rows for S1, S2,
  S3, S12 and S23.
- Take the NITs' doc and record lines.
- Nothing in the implementation needs to change.

## Scratch

I deleted `/tmp/claude-1002/v1245/` after the review: both exports, all target directories, the
web build outputs, the wasm timing copy and the logs. The probes and the mutation driver are kept
in `1245-attempt1-verifier-scratch.rs`.
