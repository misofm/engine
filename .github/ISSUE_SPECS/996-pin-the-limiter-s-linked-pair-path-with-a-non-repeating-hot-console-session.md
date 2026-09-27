# Pin the limiter's linked-pair path with a non-repeating hot console session

## Product outcome

#990 computes a linked stereo pair's gain path once in the true-peak limiter (merged on the optimisation batch; exact in a 36,600-scenario differential). Its kernel tests catch every mutation, but no session-level test can catch its M1: the console source is one 128-frame block repeated, so each limiter's gain settles to a constant, and sessions expose no state snapshots. Found by the #990 verification (medium, non-blocking).

## Smallest closable slice

Add a console session test driven by non-repeating hot noise (a deterministic seeded generator, levels that keep every limiter reducing) with mid-run parameter retargets that link and unlink pairs, rendering 64+ blocks, with its PCM and gain-reduction digest pinned on the pre-#990 kernel (`bbcf8ce1`) and asserted equal after.

## Objective gates

- The test is green on the batch and red under #990's M1 and under a mutation that keeps the shared path after the pair unlinks.
- Every standing console digest unchanged.

Also record (low): once unlinked, a pair relinks only after reset, restore or `desymmetrize`; say in the test whether that is intended.

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, branch `codex/996-limiter-linked-session-test` from `1341162b`.
Commit: `8a4ba1b5` (the test and the mutation record). Test code only: `git diff 1341162b 8a4ba1b5
--stat` touches `crates/host-core/tests/limiter_linked_session.rs` (new) and
`crates/true-peak-limiter/tests/MUTATIONS.md`.

### The test

`crates/host-core/tests/limiter_linked_session.rs`,
`the_hot_console_renders_the_pre_990_words_through_link_and_unlink`. It lives in `host-core`
rather than `console-workload` because only the host facade takes per-block source content from a
test (`SourceControlSet::submit`). `console-workload` plays one frozen block per track, and making
it play more is a tooling change.

* **Session.** `console-sixty-four-track-intended.json`, prepared with
  `prepare_host_session_with_console`, with two in-test edits:
  * the one source is widened to 128 channels, with track `k` reading `2k` and `2k + 1`;
  * every compressor is bypassed from block 0 through its live control channel.
  Both edits are measured (below). Banks bind at W8 on this host.
* **Source.** Seeded SplitMix64 noise, one independent stream per source channel. Each 96-frame
  segment has its own level, +6 to +18 dBFS peak. The segment divides neither the quantum nor the
  241-frame window, so nothing repeats.
* **Retargets.** Fourteen, through the limiters' live control channels, over 96 blocks:
  * two-channel ceiling retargets in one block (`ch18`, blocks 12 and 60), which stay linked;
  * one-sided ceiling or release retargets that unlink six W8 banks, at blocks 17, 21, 49, 51, 53
    and 81, after up to 81 linked blocks, plus one more (`ch05`, block 83) that unlinks only at W4;
  * five re-equalling retargets (blocks 34, 36, 66, 68, 90).

  Every unlink lands where the van Herk phase `128 * block mod 241` is at most 37, so a stale
  right ring answers 203 to 235 frames wrongly. Each event lands in a different bank at W8 and at
  W4.
* **Digest.** One SHA-256 over, after every block, the master's 256 output words and every
  limiter's published gain-reduction window: sequence, left word and right word, with the tap armed
  at a one-block window.
* **Checks ahead of the pin:**
  * every limiter limits: from block 3, a reduction word above 0.5 (more than 6 dB) on both
    channels at every block end. The measured minimum is 0.7556 (12.2 dB), and the deepest is
    about 0.956;
  * each left reading rises in at least four blocks (measured 8 to 26), so no limiter is only
    releasing;
  * a pair's two readings stay bit-equal until a one-sided retarget reaches it;
  * every one-sided retarget parts them.
* **Runtime.** 2.3 s in dev, 0.05 s in release.
* **Relinking (low item).** The doc comment records that relinking only after `reset`, a restore
  or `desymmetrize` is **intended**: it is #990's invariant, because equal designed words do not
  prove equal gain words. The scenario shows why at `ch03`, block 34. The cost is liveness only.
  A session cannot observe a relink, so the test asserts the words through every re-equalled
  stretch. The relinking mutation K2 moves exactly the five re-equalled pairs. The decision itself
  stays asserted by #990 gate 2.

### How the pin was recorded on the pre-#990 kernel

`bbcf8ce1..1341162b` touches `crates/true-peak-limiter/src` in exactly one commit, `fdc295db`
(#990). The base tree was `git archive 1341162b` in the scratch directory, with
`crates/true-peak-limiter/src/lib.rs` replaced by `git show bbcf8ce1:crates/true-peak-limiter/src/lib.rs`
(SHA-256 `37967da3…`, 1,654 changed lines against the head). It was built in its own target
directory. The test's repin mode printed `b22ef17ce523cc4bfa79ef3948469983636ffabffedad01d9de34203934f54d5`,
and the head printed the same digest. With the pin written in, the test passes on the base in
dev (2.39 s) and release, and on the head in dev (2.33 s) and release. The base tree, its target
directory and the worktree's `target/` were deleted afterwards.

### Mutations

Each mutation was applied alone to `crates/true-peak-limiter/src/lib.rs`, the test was run, and
the file was restored with `git checkout` and checked clean. All six were red. They are recorded in
`crates/true-peak-limiter/tests/MUTATIONS.md`, "Issue #996".

| row | mutation | fired | digest |
|---|---|---|---|
| 996-M1 | #990 M1, the mirrored suffix store skipped | channels-equal check, `ch00` at block 17 | `881bf62b…` |
| 996-K1 | the shared path kept after the pair unlinks (`designed_gain_agree` dropped from the decision) | `ch03`'s retarget parts nothing | `4c664808…` |
| 996-K2 | the record never cleared by a dual block (#990 M12), so the pair relinks on designed agreement | the pin | `198af660…` |
| 996-M2 | #990 M2, the right box store skipped | channels-equal check | `a6cf49f7…` |
| 996-M6 | #990 M6, the right required store skipped | channels-equal check | `2abf0380…` |
| 996-M7 | #990 M7, the right box sum not copied at block end | reduction floor | `c25993b2…` |

The digests come from a temporary print placed ahead of the assertions while the scenario was
being built. The scenario did not change afterwards.

Robustness of M1:
* M1 is red at 12 of 12 noise seeds.
* With a temporary reading dump, it moves five to seven lanes' published words in each of the six
  W8 banks that unlink, from the unlink block on. The two banks that never unlink do not move.

Two earlier shapes stayed green under M1, and are recorded so they are not retried:
* **The fixture's one shared stereo source** (compressors bypassed, three unlinks). Every lane
  hears the same noise, so an unlink is one trial.
* **Compressors in circuit.** On the shared source, 46 of 64 limiters only released after block 32.
  With per-track streams, M1 went red on only one or two lanes in two banks.

A bank-API scratch test (deleted) confirmed first that M1 is output-visible on independent
per-lane noise.

### Gates

| gate | command | result |
|---|---|---|
| format | `cargo fmt --all --check` | clean |
| lint | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |
| test crate, dev | `cargo test --locked -p host-core --all-targets` | all pass (73 unit tests and 15 test files, the new one included) |
| test crate, release | the same with `--release` | all pass |
| standing console digests | `cargo test --locked --release -p console-workload` | 39 pass, `chain_shape` 23 included. No engine code changed. |
| policy | `scripts/check-host-core-policy.sh`, `scripts/check-workspace-policy.sh` | ok |

All builds used `CARGO_INCREMENTAL=0`. There was no timed benchmark.

### Deviations and notes for the verifier

* **Width.** The host facade prepares at `Backend::current()`, so this test renders only W8 here,
  and only W8 in CI. The #990 verification also asked for Simd4 and the wasm guest at session
  level. That needs `console-workload` to take a per-block source (tooling code) or a host-core
  backend override (engine code), and neither is test code. The doc comment states the W4
  expectation. The kernel's `tests/linked.rs` keeps W4 and scalar.
* **The two in-test session edits** (per-track streams, compressors bypassed) mean this is the
  standing strip's session under a live console, not the benchmark row verbatim. Each edit is
  measured above.
* **Mutations beyond the brief.** K2, M2, M6 and M7 were added. Five of the six rows fire on a check
  ahead of the pin rather than on the pin, and all six move the digest.
* **Pre-existing, not touched.** `scripts/check-env-vocabulary.sh` fails on the batch head without
  this change. `docs/handoffs/dual-mono-2026-09-27/*` carries `MISO_RESEARCH_*` and
  `MISO_VERIFY_*` names outside the `MISO_ENGINE_` prefix. The new
  `MISO_ENGINE_REPIN_LIMITER_LINKED_SESSION` is crate-local and prefixed, like
  `MISO_ENGINE_REPIN_TRUE_PEAK_LIMITER_LINKED`.

## Sol attempt 1 verdict: PASS

Reviewer: Sol, 2026-09-27, on `53fa26bb`, alone and merged onto the batch head `077fb31e` (which
also carries #980's EQ change and the env-vocabulary fix). Branch code untouched. Every mutation
was applied to a scratch copy, and `src/lib.rs` was checked byte-identical by SHA-256 after each
row. Scratch trees, harnesses and target directories were deleted.

### What was reproduced

* **The pin is honest.** `b22ef17c…` renders in dev and in release from five trees:
  * the branch;
  * the branch with `src/lib.rs` from `bbcf8ce1`;
  * the branch with the whole `crates/true-peak-limiter` from `bbcf8ce1`;
  * the complete `bbcf8ce1` tree plus the new test;
  * the branch merged onto `077fb31e`.

  It also passes under CI's `test-debug-a` features (`engine/realtime-audit` included).
* **Widths.** The scenario was run as a scratch host-core unit test through the existing
  `#[cfg(test)]` seam `prepare_host_runtime_with_console_backend` (`crates/host-core/src/prepare.rs:596`).
  `Simd4` and `Scalar` both render the pin on this host, so the doc comment's W4 claim holds.
* **The claimed rows.** M1, M2, M6, M7, K1 and K2 are red, with the recorded digests, in dev and
  in release. A readings dump confirms the recorded effects:
  * M1 moves the right channel of 38 tracks: 5 to 7 lanes in each of the six W8 banks that
    unlink, from the unlink block on. Banks 2 and 6 do not move.
  * K2 moves exactly `ch03`, `ch12`, `ch37`, `ch42` and `ch57`, each from the block after it is
    made equal again.
  * M1, M2 and K2 are red at 8 of 8 noise seeds.

  Through the seam, all six rows are also red at `Simd4` and at `Scalar`.
* **Other #990 rows.** M5, M8 and M10 are red. M9 is green (finding 2).
* **Invented mutations**, each red unless stated:
  * X1: the shared gain fed by the left detector alone.
  * X2 and K1d: the landing block of a one-sided retarget renders linked (the decision one block
    late, or the kernel handed the previous record).
  * K1c: the linked body runs while the record is cleared (`link_max` passed to the kernel).
  * X5b: one mid-block suffix store not mirrored.
  * X9: the right box term stored at the expiring slot.
  * X6: the right `prefix` written back from its stale register.
  * X7: agreement ignores the release ramps.
  * X8: the right output read from the left delay ring.
  * X3 and X4, agreement skipping the last lane or lane 0: **green at W8** (finding 3). They are
    red at `Simd4` and `Scalar` through the seam, and in the kernel's own tests.
  * X5, the mirror store to the backward pass's oldest slot skipped: green, and equivalent. That
    slot is `start = cursor + 1`, which the next frame's required store overwrites before any read.
* **The session edits are sound.** The widened source and the live bypasses go through the real
  grammar, compile and host-prepare path. Bypass is the rack's runtime shunt, so the plan and the
  bank formation are unchanged. #990's decision is internal to the limiter, and the compressor only
  shapes its input, so the bypass hides no interaction #990 could have.
  * With the compressors in circuit, M1 is still widely visible (finding 4).
  * The bypass is justified by the non-vacuity floor: with the compressors in, track 14 falls to
    0.49 at block 16.
* **Gates.**
  * On the merged tree:
    * format clean;
    * clippy clean (workspace, all targets, all features, `-D warnings`);
    * host-core, all targets, pass in dev (CI features) and release;
    * `console-workload` release 39 of 39.
  * Policies ok: host-core, workspace, env-vocabulary (merged tree), realtime, realtime-audit-leak,
    artifact-evidence-leak, dsp-research, session and lane.
  * The non-vacuity figures reproduce: shallowest 0.7556, deepest 0.9566, rises 8 to 26.
  * Runtime is 2.3 s in dev and 0.05 s in release. Every run was bit-deterministic.

### Findings (severity-ranked; none fails a gate)

1. **Medium: the width deviation is wrong, and width coverage is cheap.**
   * *The claim:* `Simd4` at session level needs tooling or engine code.
   * *The fact:* the seam above is test-only and already used by host-core unit tests. W8-only
     meets #996's gates, which name no width.
   * *Failure scenario:* a defect that shows only at four-lane bank edges (X4) passes CI here and
     reaches AArch64 and wasm hosts.
   * *Successor (bounded, test code only):* a host-core unit test that runs this scenario at
     `Simd4` and `Scalar` through the seam against the same `PIN`, with findings 2 and 3 folded in.
2. **Low: M9 is green.** #990's M9 strands the right ramps in the linked ramping dispatch.
   * The scenario has one linked ramping trial (`ch18` at block 12, one lane), and M9 is red at
     only 3 of 8 seeds. The kernel's gates 1-3 catch it.
   * *Failure scenario:* that regression passes this test on the pinned seed.
   * *Fix:* retarget both channels on every lane of a linked bank.
3. **Low: no one-sided retarget lands on W8 lanes 0, 6 or 7.** So X3 and X4 pass at W8. The
   successor should add one on lanes 0 and 7.
4. **Low: the recorded measurement does not reproduce.** The spec and `MUTATIONS.md` say that with
   the compressors in circuit and per-track streams, M1 moved only one or two lanes in two banks.
   * On the committed schedule it moves 26 tracks at +6 to +18 dBFS and 27 tracks at +16 to
     +22 dBFS, in all six unlinking banks.
   * *Failure scenario:* the note is cited as a reason never to test the standing strip.
   * *Fix:* re-date or correct it.
5. **Info.** "Each event lands in a different bank at both launch widths" is false at W8 for
   `ch03` and `ch05`, which share bank 0. The table row says so.

### For the owner

* Rule on wasm session coverage. It needs tooling: a per-block source for the wasm console guest,
  or a wasm32 runner for host-core tests. The #990 verification checked kernel identity under
  Node.
* File finding 1's successor, with findings 2-4 folded in.
