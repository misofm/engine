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
