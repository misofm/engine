# #1244 attempt 1 verdict: Compose live VCA moves in host-core

**Verdict: PASS.** No BLOCKER, no MAJOR. One MINOR to fix before close: the size/cost INFO in the
Attempt 1 record (and the module doc) understates the browser's 1 MiB worst case by about 3.6x.
That needs correcting, and #1245's brief must carry a bound on per-command work. Three NITs and two
INFOs for #1245.

- **Implementation:** `1db0aacc3` on parent `d5dce7b05`, branch `codex/batch-vca`.
- **Review copy:** `git archive` exports of `1db0aacc3` under `/tmp/claude-1002/v1244/` (`src` for the
  gates, `mut` for probes and mutations), with their own target directories. All deleted after the
  review. I did not touch the worktree, the branch or GitHub.
- **Host:** x86-64-v3 AVX2.
- **Probes and mutation driver:** `1244-attempt1-verifier-scratch.rs`, next to this file.

## Gates re-run on `1db0aacc3` (all exit 0)

| Gate | Result |
|---|---|
| CI's `cargo test --locked -p builtins-compiler --no-run`, then the spec's test-debug-a command (`--no-fail-fast`) | rc 0: 115 binaries, 1,275 passed, 0 failed, 9 ignored (same as the record) |
| `vca_live` at `MISO_ENGINE_RANDOMIZED_SCALE=100`, release | 8/8 green: 3,200 gate-1 seeds, 1,600 gate-2 seeds, **1,200 render-differential seeds** x 8 blocks (71 s) |
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | clean |
| `check-`/`test-` pairs for `host-core-policy`, `realtime-policy`, `workspace-policy` | all six rc 0 |
| `check-cross-targets.sh` | PASS. host-core is at 4 `memset_pattern16` calls, its ceiling; no new row |
| `build-web-audioworklet.sh --named-twin B A`, `check-web-audioworklet.sh A B/...named.wasm` | rc 0, callgraph included. Shipped module `d54586b9...` |
| `check-browser-expected-resources.py --artifacts A` | rc 0, no re-pin |
| `run-aarch64-tests.sh debug` | not run (no arm64 host). It stays "at batch push", as recorded |

## What holds up under probing

- **One composition (D1).** `effective_db` calls `session::vca_effective_db(own, reach offsets)`
  with the reach table flattened from `vca_reach()`, in its ID-sorted order. That is the same
  function and the same order preparation uses, so a live value can only differ from a fresh
  plan's through the table, and gate 1 checks the table.
  - Probe `sol_probe_deep_chain_and_lattice_equal_preparation`: chains 20 to 137 VCAs deep, and
    lattices where `v{i}` also holds `v{i-2}` (every strip reaches every VCA along many paths).
    Tracks and submixes, 200 random edits each, with values across the domain including
    `+-1e-30`, `-0.0` and both edges. Result: 80,000 lane checks, all bit-identical to
    `effective_strip_faders()`.
- **Own value versus effective value.** `own_db` is only ever written by `set_member_db`;
  `record_emitted_db` writes only `emitted_db`. Clamp-and-restore works (gate 3; H4 is red).
- **`!=` and signed zero.** Probe `sol_probe_signed_zero_effective_flip_renders_as_fresh`:
  - setup: own `-0.0`, VCA `+0.0` (bakes `+0.0`); the VCA then moves to `-0.0` (live effective
    `-0.0`);
  - `fader_delta` owes nothing, and the live render equals, bit for bit, a fresh plan that bakes
    `-0.0` dB, on negative input.
  - So the frozen `!=` is correct, and a bit-comparing delta would be a redundant record. My S5
    turns gate 2 red.
- **Mute precedence (D2).** `set_vca_mute` sets the per-lane term under the shadow, and
  `effective_mute`/`strip_delta` are unchanged.
  - One-lane VCA mutes stay per lane all the way through: the VCA state, the owner's term,
    `strip_delta` and the follow delta (gate 5 `b-bx [false, true]`). This is the #1242
    pitfall, and it does not recur here.
  - Solo never clears the VCA mute, and solo-safe is not VCA-safe (gate 5; H8 red).
- **Transactions.** The VCA state's shadow covers all four mutable arrays. The owner's
  `vca_mute_shadow` is reserved in `try_new`, copied in `shadow()` and restored in `rollback()`.
  The render differential rolls back 19 batches (12 seeds) across all three mirrors together.
- **Allocation.** Gate 6 measures 63 command passes after warm-up at
  `allocations == deallocations == 0`. H12 (collecting offsets into a `Vec`) is red when I re-ran
  it. A VCA-free `try_new` allocates nothing, and `retained_bytes` equals `requested - released`
  around `try_new`.
- **No `free` in any new name** (the callgraph gate passed). No splatted non-zero store
  (cross-targets: still 4).

### Deviations: all accepted

- **Out-of-range indices.** `false` or no-op; `effective_db` is `0.0` for an unknown strip or lane
  and the prepared value for an unreached strip; `fader_delta` of an unreached strip is empty. This
  follows the solo/route precedent, and nothing in D3 reads an unreached strip. See NIT-2 for a doc
  sharpening.
- **"At least N seeds".** `run_seeds` multiplies by `MISO_ENGINE_RANDOMIZED_SCALE`, so `ran >= N` is
  the right assertion.
- **The render differential beyond the frozen gates.** It is a randomized differential with its
  reach asserted. It is the only test that catches my S1.

## MINOR-1: the 1 MiB worst case is about 6.3 M pairs, not 1.7 M (fix the record and module doc; carry a bound into #1245)

The record's "For #1245" bullet and `vca.rs`'s `# Size` doc both cite ~1.7 M (strip, VCA) pairs /
~14 MiB on wasm32. That figure came from my #1243 INFO-1, which measured **canonical
(indented) JSON**. The browser checks only the raw byte length (`MAXIMUM_DOCUMENT_BYTES`, 1 MiB) and
`parse_host_session` accepts whitespace-free JSON. Probe `sol_probe_worst_case_pairs_and_command_cost`
(release, this host) builds submixes plus a chain where `v0` holds every strip and `v{i}` holds
`v{i-1}`:

| Document (<= 1 MiB) | Pairs | `retained_bytes` native | wasm32 (computed) | `try_new` | One VCA dB move (D3) | One VCA mute move (D3) |
|---|---|---|---|---|---|---|
| canonical, 684 submixes x 2,527 VCAs | 1.73 M | 26.5 MiB | 13.3 MiB | 0.40 s | **2.9 ms** | 0.9 ms |
| whitespace-free, 1,292 submixes x 4,842 VCAs (1,045,371 B) | **6.26 M** | 95.6 MiB | **47.9 MiB** | 1.9 s | **10.6 ms** | 3.6 ms |

(With tracks instead of submixes, the whitespace-free case is 4.07 M pairs, 31.2 MiB on wasm32 and
6.8 ms per move.)

- **Bytes are reported honestly.** `retained_bytes` and `largest_allocation_bytes` match the
  tables, and a host that charges them charges the right thing. 48 MiB fits the 512 MiB default
  budget.
- **The try_new transient is not covered by the parse projection.** It holds three pair-sized
  tables at once: the `vca_reach()` lists plus both flattened tables, then the inner `vca_reach()`
  of `effective_strip_faders()`. That is about 75 MB on wasm32 at the whitespace-free worst case.
  `PARSE_TRANSIENT_MULTIPLIER = 20` (a 20 MiB projection) does not cover it.
- **The command cost is the real carry-forward.** D3's VCA move costs the sum over
  `reached_by(v)` of `|reach(s)|`, and this runs on the AudioWorklet thread (`port.onmessage` ->
  `miso_engine_web_v1_command_submit`).
  - Even the canonical-form worst case (2.9 ms native) exceeds one 128-frame quantum at 48 kHz
    (2.67 ms). The whitespace-free case is about 4 quanta natively, and wasm on a phone is slower.
  - The implementation is optimal for the frozen composition: an exact, ordered `f64` sum per
    reached strip cannot be updated incrementally. So this is not an attempt-2 code change.

**Fix:**
1. Correct the record's INFO and the `# Size` doc to the figures above. Say that the bound is the
   raw byte length, not canonical JSON.
2. Amend #1245's brief so the browser bounds per-command VCA work before the batch push. Options:
   - a boot-time cap on total reach pairs (`reach.len()`; consider exposing it as
     `reach_pair_count()`);
   - a cap on nesting depth;
   - a finite browser `maximum_vcas`, refused with a typed error;
   - an owner-accepted worst case stated in milliseconds.

   #1245 should also charge `retained_bytes` and the try_new transient against the boot budget.
   Left unbounded, this is a MAJOR in #1245, not here.
3. Optional, here or in #1245: the mute path can be O(`reached_by(v)`) with per-strip, per-lane
   counts of muting VCAs. The dB path cannot.

## NITs

- **NIT-1. `record_emitted_db` without its `shadow()` survives every test** (my S2, green across
  `vca_live` and the lib tests). It is unreachable through D3, where a setter always shadows
  first. If you want it defended: a gate-4 step whose first VCA-state mutation is
  `record_emitted_db`, then `rollback`, then `fader_delta` must owe the record again.
- **NIT-2. Setter preconditions are undocumented.** `set_vca_db(NaN)` is accepted. Its delta
  then owes `NaN` on both lanes forever, even after `record_emitted_db` (`NaN != NaN`; probe
  `sol_probe_non_finite_offset_owes_forever`). `+500` and `-1000` are accepted and clamp
  silently. The route mirror has the same precedent (the host validates). **Fix:** one doc line
  on `set_vca_db` and `set_member_db`: "finite and in `[-144, 24]`; the host validates before
  calling". Also make `effective_db`'s doc say that an unreached strip returns the value
  preparation baked, **not** a later own move.
- **NIT-3.** `try_new` runs `vca_reach()` twice: once directly, once inside
  `effective_strip_faders()`. Seeding `emitted_db` with `vca_effective_db` over the lists already
  built would give the same bits for half the time and transient. D1 says "seeded from
  `effective_strip_faders()`", so this is optional and needs a brief note if taken.

## INFO for #1245

- **INFO-1. A member's own move.** D3 says "calls `set_member_db`, then stages `effective_db`".
  For a clamped member, that stages an unchanged target. Gate 2's named case uses `fader_delta`
  plus `record_emitted_db` instead, which owes nothing. #1245 should pick one deliberately: either
  `fader_delta` (handle a zero-record admission) or "an explicit own move always stages" (as kind
  3 does for a non-member). Then record the choice.
- **INFO-2. #1245 must stage a VCA move all-or-nothing.** A move can owe up to
  `2 x reached_strip_count` fader records. #1245's all-or-nothing staging must check every
  reached strip's queue room before any push, so the emitted mirror can never claim a record that
  a full queue dropped (the acked-batch question).

## Mutations

### Mine (applied in the `mut` export one at a time; file restored and checked after each)

| # | Mutation | Result |
|---|---|---|
| S1 | owner's `set_vca_mute` ORs the new term into the old (an un-muted VCA never releases) | red: `live_vca_moves_render_as_a_fresh_plan` only |
| S2 | `record_emitted_db` without `shadow()` | **green** (NIT-1; unreachable through D3) |
| S3 | VCA `shadow()` omits `own_db_shadow` | red: `rollback_restores_every_mirror_and_commit_keeps_them` |
| S4 | `vca_mute()` reads only the first reaching VCA | red: `a_live_recompute_equals_preparation`, render differential |
| S5 | `fader_delta` compares bits instead of `!=` | red: `a_composition_never_owes_a_redundant_record` |
| S6 | owner's `shadow()` omits `vca_mute_shadow` | red: `rollback_restores_every_mirror_and_commit_keeps_them` |

### The record's, re-run

| Row | Result |
|---|---|
| H12 | red, gate 6 |
| H13 | red, both gate-2 tests (as recorded; the render differential does not catch a redundant record at zero smoothing, and gate 2 owns that) |

## Test value (one sentence each)

- **`a_live_recompute_equals_preparation`:** turns red if the flattened reach, its order, a lane
  index or the own-versus-effective split differs from preparation, or if `reached_by` misses a
  nested member (H1-H3, H9, S4).
- **`a_composition_never_owes_a_redundant_record`:** turns red if a delta re-emits an unchanged
  lane, misses a moved one, mis-shapes `Both`/one-lane, or seeds the mirror from own values (H5,
  H13, H14, S5).
- **`an_unchanged_effective_value_owes_no_record_and_the_delta_shape_follows_the_lanes`:** turns
  red if a member clamped before and after, or a member-less VCA, owes a record, or a selector
  disagrees with the moved lanes (H5, H13, H14).
- **`a_clamped_member_returns_to_its_own_value`:** turns red if the clamped effective value is
  stored as the member's own (H4).
- **`rollback_restores_every_mirror_and_commit_keeps_them`:** turns red if a refused submission
  leaves any VCA, member, emitted or owner-VCA-term mirror changed, or a second transaction does
  not shadow afresh (H6, H7, H15, S3, S6).
- **`a_vca_mute_wins_over_solo_and_reaches_the_following_sends`:** turns red if solo clears a VCA
  mute, solo-safe exempts a submix, or the VCA mute misses the follow composition (H8, H9).
- **`the_command_path_allocates_nothing_and_the_retained_bytes_are_measured`:** turns red if a
  setter or delta allocates, the VCA-free path allocates, or `retained_bytes` omits a table or
  shadow (H10-H12).
- **`live_vca_moves_render_as_a_fresh_plan`:** a randomized differential judged by reach. Its
  reach counters are asserted, and it is the only test that catches S1, a VCA un-mute that never
  releases its members.

## For close

- Apply MINOR-1 steps 1 and 2: the record and doc correction, and #1245's brief amendment.
- NIT-2's doc lines are cheap; take them at the same time.
- Nothing else in the slice needs to change.

## Scratch

I deleted `/tmp/claude-1002/v1244/` after the run: both exports, all target directories, the web
build outputs and the logs. The probe code and mutation driver are kept in
`1244-attempt1-verifier-scratch.rs`.
