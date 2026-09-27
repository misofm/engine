# Run the limiter linked-pair session test at Simd4 and Scalar, with lane-edge and linked-ramp coverage

## Product outcome

#996 pins the true-peak limiter's linked-pair path (#990) with a hot, non-repeating 64-track console session (`crates/host-core/tests/limiter_linked_session.rs`). It runs only at the host's native bank width (W8). The browser runs 4-lane banks, so the product's width is untested at session level. The #996 verification found three gaps, all test-only:

1. The test's claim that W4 needs engine or tooling changes is wrong. Host-core's existing test-only seam `prepare_host_runtime_with_console_backend` (`crates/host-core/src/prepare.rs`) renders the same pin at `Simd4` and `Scalar`, and all six recorded mutation rows are red there.
2. #990's M9 (the right ramp leaves the left stale) stays green: there is one linked-ramp trial, and it is red at only 3 of 8 seeds.
3. No one-sided retarget lands on W8 lanes 0, 6 or 7, so a mutation confined to a bank's edge lanes passes at W8.

## Smallest closable slice

Test code only, in `crates/host-core/tests/limiter_linked_session.rs` (and `crates/true-peak-limiter/tests/MUTATIONS.md` for the record):

- run the #996 scenario at `Simd8`, `Simd4` and `Scalar` through the existing seam, each against the same pinned digest (the pin is width-independent because banking never changes per-lane arithmetic);
- add linked-ramp retargets (both channels of a linked pair moving a ramped control together) so that M9 is red at every seed tried;
- place one-sided retargets so each of W8 lanes 0 and 7, and W4 lanes 0 and 3, receives at least one.

## Objective gates

- The pin (`b22ef17c...`) is unchanged, re-recorded on the pre-#990 limiter source (`bbcf8ce1`) at all three widths in dev and release, or a new pin recorded there if the scenario must change.
- Red at every width under the six #996 mutation rows (M1, K1, K2, M2, M6, M7), and red under M9 at 8 of 8 seeds.
- A mutation that corrupts only lane 0 and one that corrupts only the last lane of a bank are red at W8 and W4.
- Dev runtime of the test stays under 10 s; every standing console digest is unchanged.

## Out of scope

Wasm session coverage of the same scenario needs tooling (a guest that takes per-block source content). That is an owner ruling, recorded in the #996 verdict; do not build it here.

## Attempt 1 evidence

Implementer: Terra, attempt 1, 2026-09-27, branch `codex/997-limiter-session-widths` from
`f12d1466`. Commit `6be46caa` carries the test and the mutation record. Test code only: `git diff
f12d1466 6be46caa --stat` touches `crates/host-core/src/lib.rs` (one `#[cfg(test)]` module line and
its comment), `crates/host-core/{tests => src}/limiter_linked_session.rs` and
`crates/true-peak-limiter/tests/MUTATIONS.md`. No engine source changed.

### The test

`crates/host-core/src/limiter_linked_session.rs`, three tests:
`the_hot_console_renders_the_pre_990_words_at_simd8`, `_at_simd4` and `_at_scalar`. Each compiles
the session with `compile_host_session` and prepares it through
`prepare_host_runtime_with_console_backend` at its backend, then runs #996's render loop and checks
against one `PIN`.

* **Where it lives (deviation).** The spec names `crates/host-core/tests/limiter_linked_session.rs`,
  but the seam is `#[cfg(test)] pub(crate)`, which a `tests/` file cannot see. The file moved into
  the crate as a `#[cfg(test)]` module (`git mv`, so history follows it). This is how #990 placed
  its gates 1 and 2 for the same reason. A non-test build of `host-core` is unchanged.
* **Scenario changes.** Every #996 row stays, except that `ch37`'s two move to `ch38` and
  `ch18`'s two linked ramps widen to its whole W8 bank. Six rows are added:

  | block | tracks | retarget | why |
  |---|---|---|---|
  | 12, 60 | `ch16`-`ch23` | ceiling -3 dB, then -1.5 dB, both channels (widened from `ch18`) | linked ramps over every lane (M9) |
  | 26, 44 | `ch48`-`ch55` | release 250 ms, then ceiling -3.5 dB, both channels | linked ramps over every lane (M9) |
  | 66 | `ch23` | ceiling -4 dB, left | unlinks a bank from W8 lane 7 and W4 lane 3 |
  | 85 | `ch48` | release 180 ms, right | unlinks a bank from lane 0 at W8 and W4 |
  | 90, 92 | `ch23`, `ch48` | the other channel equal again | relink rows, like #996's |

  Moving `ch37` to `ch38` makes the eight retargets that unlink the eight W8 banks land on eight
  different lanes (it was lane 5 twice; `ch38` is lane 6). At W4 the nine unlinking retargets
  cover all four lanes. Every unlink still lands at van Herk phase 37 or less (66 is 13, 85 is 35).
* **New checks, ahead of the pin.**
  * Each track's bank and lane are read from the prepared plan (`bank_shape`,
    `unit_eligibility`), not assumed. Every unit that renders a track must agree on its bank and
    lane, and every bank must be full. At `Scalar`, nothing may bank.
  * Every lane of the width must carry the retarget that unlinks some bank. So a planner change
    that moved the edge retargets off the edge lanes fails the test instead of quietly dropping
    the coverage.
  * The digest is printed before the checks, so a failing run shows it.
* **The #996 checks are unchanged:** the limiting floor and rises, channels bit-equal until a
  one-sided retarget, and every one-sided retarget parts its pair. The measured figures are the
  same at every width: the shallowest reduction word is 0.7556 and the fewest rises is 8.
* **Runtime, dev profile:** `Simd8` 2.33 s, `Simd4` 3.15 s, `Scalar` 1.95 s. That is 3.2 s for the
  three under the default parallel harness and 8.8 s on one thread. In release the three take
  0.10 s together.

### The pin, recorded on the pre-#990 limiter

The scenario changed, so the pin is new: `6d87267b7502a4b4cb663315629d777350ce6ecc62a2d14e0eaebca9b88d9ff9`.

* **Base tree.** A scratch worktree of `f12d1466` plus this test, with
  `git checkout bbcf8ce1 -- crates/true-peak-limiter` and the later `tests/linked.rs` removed, so
  `git diff bbcf8ce1 -- crates/true-peak-limiter` is empty (`src/lib.rs` SHA-256 `37967da3…`).
  It was built in its own target directory.
* **Recording.** In repin mode the base printed `6d87267b…` at `Simd8`, `Simd4` and `Scalar`, in
  dev and in release.
* **Seeds.** Through a temporary seed hook (never committed), the base and the head also rendered
  identical digests at each of eight seeds (`0x0996` to `0x099D`), at all three widths, in dev and
  in release (base) and in dev (head). Every seed passed every check.
* **With the pin written in**, the committed test passes on the base in dev and release, and on
  the head in dev and release.

### Mutations

Eleven rows, recorded in `crates/true-peak-limiter/tests/MUTATIONS.md`, "Issue #997", with the
check that fired and the digest per width.

* **Driver.** Each mutation was applied alone to `crates/true-peak-limiter/src/lib.rs` by a
  scratch script that asserts a single match. The dev test was run. The file was restored with
  `git checkout`, and its SHA-256 was checked equal to `HEAD`'s blob (`5ca9ba88…`) after every row.
* **Seeds.** Each row ran twice: once at eight seeds through the seed hook, each compared with that
  seed's own unmutated digest, and once more against the committed file at the pinned seed.

| row | mutation | `Simd8` | `Simd4` | `Scalar` |
|---|---|---|---|---|
| M1 | #990 M1, the mirrored suffix store skipped | 8/8 | 8/8 | 8/8 |
| K1 | the decision drops `designed_gain_agree` | 8/8 | 8/8 | 8/8 |
| K2 | the decision not written back to `gain_linked` (#990 M12) | 8/8 | 8/8 | 8/8 |
| M2 | #990 M2, the right box store skipped | 8/8 | 8/8 | 8/8 |
| M6 | #990 M6, the right required store skipped | 8/8 | 8/8 | 8/8 |
| M7 | #990 M7, the right box sum not copied at block end | 8/8 | 8/8 | 8/8 |
| M9 | #990 M9, the right ramps not copied at block end | 8/8 | 8/8 | 8/8 |
| E0 | `designed_gain_agree` skips lane 0 | 8/8 | 8/8 | 8/8 |
| EL | `designed_gain_agree` skips the last lane | 8/8 | 8/8 | 8/8 |
| S0 | the mirrored suffix store keeps lane 0's old word | 8/8 | 8/8 | 8/8 |
| SL | the mirrored suffix store keeps the last lane's old word | 8/8 | 8/8 | 8/8 |

"8/8" means red at eight of eight seeds. Every row was red again against the committed file at
the pinned seed. By gate:

* **The six #996 rows** are red at every width.
* **M9** fires the channels-equal check at every seed and width. A temporary reading dump shows it
  parts three to five of `ch16`-`ch22` and six or seven of `ch49`-`ch55` at each seed, identically
  at every width.
* **Lane 0.** E0 is caught at W8 by `ch48` (lane 0) and at W4 by `ch12` (lane 0). S0 moves exactly
  lane 0 of six W8 banks and seven W4 banks.
* **Last lane.** EL is caught at W8 by `ch23` (lane 7) and at W4 by `ch03` (lane 3). SL moves
  exactly the last lane of six W8 banks and seven W4 banks.
* **Scalar.** S0 and SL are M1 at `Scalar` (one lane) and render M1's digest there.

### Gates

| gate | command | result |
|---|---|---|
| format | `cargo fmt --all --check` | clean |
| lint (brief) | `cargo clippy --locked --workspace --all-targets -- -D warnings` | clean |
| lint (CI) | the same with `--all-features` | clean |
| host-core, dev | `cargo test --locked -p host-core --all-targets` | all pass (76 unit tests, the three new ones included, and 15 test files) |
| host-core, release | the same with `--release` | all pass |
| host-core, CI features | dev, `--features host-core/control-provider,builtins-compiler/test-support,source/test-support,graph/test-support,engine/realtime-audit` (CI `test-debug-a`'s features) | all pass (89 unit tests) |
| standing console digests | `cargo test --locked --release -p console-workload` | 39 pass (`chain_shape` 23). No engine code changed. |
| env vocabulary | `scripts/check-env-vocabulary.sh` | `env vocabulary: ok (134 names, one MISO_ENGINE_ prefix)` |
| policy | `scripts/check-host-core-policy.sh`, `scripts/check-workspace-policy.sh` | ok |

All builds used `CARGO_INCREMENTAL=0`. There was no timed benchmark. The scratch tree, its target
directory and the worktree's `target/` were deleted afterwards.

### Notes for the verifier

* **The file moved** from `tests/` to `src/`. This is the one deviation from the slice's wording;
  see above.
* **One #996 row changed track** (`ch37` to `ch38`, both rows) to put an unlink on W8 lane 6. The
  spec asked only for lanes 0 and 7; with lane 6 covered, the test can assert every lane at every
  width.
* **`unit_eligibility` and `bank_shape`** are `#[doc(hidden)]` plan accessors, called off render
  before the first block. At W4 the plan reports two banked units per track, one of one stage and
  one of three. Both group the same four tracks in the same order, and the test requires that.
* **Scalar and M1.** At `Scalar`, M1, M2, M6, S0 and SL fire only on the pin. Each pair there is
  its own instance, so no untouched bank-mate parts.
* **Beyond the brief.** `MUTATIONS.md`'s #996 section now points to the #997 section. It also
  carries the #996 verification's finding 4: the compressors-in-circuit note did not reproduce.
  The note was annotated, not deleted.
* **Out of scope, as the spec says:** the wasm guest.
