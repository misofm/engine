PASS

# #1300 attempt 1 -- adversarial verdict

Under review: `c288358b4` (parent `8be19c86e`) on `codex/d15-stream-a`, and the merge `13302fceb`
that brings `origin/main` `0a1176b3b` (stream J) on top of it. I reviewed
`git diff 8be19c86e c288358b4` against the spec body, AGENTS.md, decision 15 (D15-7, D15-8) and the
no-shortcuts principle. Every gate ran on `13302fceb` in a private clone
(`/tmp/claude-1002/v1300/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1300/target`). The red check
ran in a clone at `8be19c86e`; mutations and probes ran in a third clone at `13302fceb`. I did not
touch `/home/bl/misofm/wt-d15-a`.

No BLOCKER or MAJOR finding. Option C is implemented as the spec says. The diff touches only the
three authorized paths. No rendered bit moved: fixtures `--check` and the console digests are
green, and the real worklet roster is the same as on main. Every gate is green. Every new test
turns red on its named defect, and in each case it is the only red test. Neither Stop condition is
met. The merge does not conflict with #1300.

## Merge check (stream J against #1300)

- `git diff 0a1176b3b 13302fceb` touches only the three #1300 files. `git diff c288358b4
  13302fceb` does not touch `crates/soft-clip` or the spec. Main changed nothing in `soft-clip`,
  `lane` or `effect-runtime` after `8be19c86e`. In `effect-contract/src` the only change is #1330:
  `validate_prepare_request` now only debug-asserts the descriptor, and the registry validates it.
  That does not touch restore.
- **#1301** (`crates/conformance/src/randomized.rs`, `edge_ramp_restore_violations` /
  `edge_restore_positions`) probes in-flight **ramp currents** near domain edges. It plants no
  history words, and its input is finite and small. #1300 only widens the history rules, so the
  probe can lose a refusal but cannot gain one. `crates/effect-contract/tests/registry.rs` is
  #1330's descriptor test and does not use soft-clip.
- I searched the merged tree for any test that assumes the old rules (`effect.state.history`, a
  soft-clip restore with a NaN or inf word). The only hits are the #1300 rows themselves.
  - `conformance/src/effect.rs` state probes expect no NaN refusal.
  - The randomized harness's `TERMINAL_WORDS` (NaN, ±inf) are drawn only right before the
    hostile block, and the scenario ends there. Before #1300, scalar and bank refused those words
    with the same code. Now both accept them, and the D7 bound assertion still holds.
  - `soft-clip/tests/randomized.rs` passes.
- The ARTIFACT CHANGED delta reproduces on the merged base. Byte counts are
  `miso-engine-v1-audio-worklet.simd128.wasm` sizes:

  | Build | Module sha256 | Bytes | Named twin sha256 |
  |---|---|---|---|
  | `8be19c86e` | `a9a51862...` (matches the record) | 2894202 | `41003984...` (matches) |
  | main `0a1176b3b` | `09622a43...` | 2895462 | `6c3260a2...` |
  | merged `13302fceb` | `9275bcce...` | 2896157 | `d7d1472d...` |

  On both bases #1300 adds exactly 695 B. The digest change belongs to #1300 and is not
  re-pinned.

## Findings

### MINOR

1. **The soundness wording is not exact** (`crates/soft-clip/src/lib.rs:723-728`; the spec
   carries the same text at `.github/ISSUE_SPECS/1300-...md:66-71` and `:114-116`). I confirm the
   earlier verifier's note (a). The comment says every non-finite word "either reaches the output
   ... or ages out unread within 31 samples", and that "a crafted payload gains nothing beyond
   what the effect's own input can cause". A scratch probe (`logs/zz_probe.rs`, not committed)
   shows three ways this is wrong:
   - **Self-produced `X` ±inf is read, not unread.** It does not reach the output either: `cubic`
     clamps it to a finite ±2/3. Example: mix 1, `+1e37` at 100 and `-1e37` at 120. The `X`
     infinities stay 31 samples, D7 never fires and the output stays finite.
   - **On the identity or bypass path, an `e` NaN is also read.** The decimator reads it and the
     identity select discards the result.
   - **A crafted `X` NaN with a finite dry word keeps creating `e` NaNs.** This is a state the
     effect cannot produce: a self-produced `X` NaN always has a NaN dry word at the same age. For
     payload words 12..39 on identity, bypass and low-drive identity, the restored lane held
     non-finite words for up to **59 samples** with no D7 firing.
   - Own input cannot do this. In every self-produced plant I tried, D7 cleared the lane, or the
     words aged out, within 31 samples.

   The conclusion still holds:
   - D7 checks every output, nothing feeds back, and no single word outlives 31 samples.
   - Across 216 accepted crafted words × 5 configurations, no output left the D7 bound.
   - Neither Stop condition is met. No self-produced word survived more than 31 samples (worst: 31
     for `X` written at the block's last sample). No restored non-finite word reached the output
     without D7.

   Fix: state the three outcomes.
   - It reaches the output and D7 recovers the lane.
   - It is clamped by `cubic` to a finite value (`X` ±inf).
   - It feeds only a path the identity select discards. A crafted `X` NaN can then keep the lane
     dirty for up to ~60 samples with no output effect. If the mix or output moves off identity
     during that time, D7 fires.

   Then reword "gains nothing" to "can cause no output beyond a D7 recovery". Amend the spec's
   argument the same way.

### NIT

1. **"Two opposite infinities" is the wrong condition** (`lib.rs:805`, `state_roundtrip.rs:373`;
   spec `:59`). The half-band's even taps alternate in sign. What makes `u` NaN is two infinite
   `X` words whose **tap products** have opposite signs. Probe results:
   - `+1e37` at 118 and 119 (same sign, adjacent taps): 8 NaN `e` words.
   - `+1e37` at 100 and 120 (straddling the centre): 7 NaN `e` words.
   - `+1e37` at 118 and 120: none.

   The predicate is unaffected: it admits NaN whatever the cause. R2 still holds because its
   pair, at 118 and 120 with opposite signs, gives opposite-sign products.
2. **"Exactly the words the kernel can write" overstates** (`lib.rs:718`). `X` and `e` still admit
   normals below `FLUSH_EPS` (`1e-20`), and `X` admits `-0.0`. `flush` never writes either. The
   old comment said "the check is the looser zero-or-normal". Tightening this is a spec non-goal
   (#1071 NIT-1), but the comment should say "every word the kernel can write" and not "exactly".
3. **`history_words` says "The classes of"** (`state_roundtrip.rs:357`). It returns the values,
   not their classes.
4. **Note (b) was not a regression.** The `roster FAIL` lines come from `test-web-audioworklet.sh`,
   in the call-graph analyser's self-test. The synthetic rosters (`partly`, `absent`, `ambiguous`,
   a synthetic `soft-clip`, the limiter rows, `eight lanes`) fail there on purpose, and the run
   ends "web AudioWorklet call-graph analyser self-test passed". `check-web-audioworklet.sh`
   itself printed no FAIL line on main or on the merged tree. Its real roster rows are the same on
   both, including `roster ok vector=25 scalar=0 budget=8.0 ceiling=0.1 soft-clip f32x4`. The
   self-test FAIL lines are also the same on main. The committed attempt record does not mention
   them, so nothing needs to change.

## Spec conformance

- **Deliverable 1:** `x_history_word_valid` = `!is_subnormal()`; `e_history_word_valid` =
  `normal_or_zero || is_nan`; `dry_history_word_valid` = `true`. Each is a plain `fn` documented
  with its kernel line. They are still held in a `[fn(f32) -> bool; 3]`, which is allocation-free
  and the same shape as before.
- **Kernel and words:** I checked the kernel (`kernel.rs:186-213`, `halfband.rs:155-199`).
  - `X` ages 0..30, `e` ages 0..29 and dry age 30 are the only reads.
  - `cubic(±inf)` = ±2/3 and `cubic(NaN)` = NaN, so `e` is never infinite.
  - The drive is at least -24 dB, so there is no `0 * inf` in `X`.
  - The three sets are a superset of every self-produced word. Every self-produced word restores.
- **Deliverable 2:** the history bullets are rewritten. "A NaN is never accepted" and "the dry
  history must be finite" are gone. See MINOR-1 for the wording.
- **Gate 1:** the old test is deleted. `a_snapshot_holding_non_finite_history_restores_and_continues_bit_for_bit`
  has rows R1-R4 exactly as specified. For each row it checks:
  - no D7 in the snapshot block, a finite output and the word classes;
  - the restore returns `Ok` and the restored snapshot equals the payload;
  - three continuation blocks with equal bits and equal `ProcessReport`s;
  - equal final snapshots.

  It pins no NaN bits (`is_nan`/`is_infinite`/`contains(&INFINITY)` only). `restores_and_continues`
  gained a `blocks` argument with per-block report equality, and the existing callers pass `1`.
- **Gate 2:** `bad(12,NaN)`, `bad(73,NaN)` and `bad(103,inf)` are removed.
  `bad(12,1)`, `bad(43,1)` and `bad(43,inf)` are kept, and `bad(43,-inf)` is added.
- **Hazards:**
  - The predicates are inside `REALTIME_POLICY_BEGIN/END`, with no allocation, formatting or panic
    path.
  - No test pins a byte count or digest.
  - No version suffix or naming change.
  - No test greps source.
  - There are no queues, so the acked-batch question does not apply. Restore is a control-plane
    decode, and a refused restore writes nothing (decode, then apply).
- **Decision 15:** D15-7's carry now meets no own-snapshot refusal for soft-clip. #1279 D6 stays
  the generic fallback. Nothing touches D15-8.

## Red before the fix (`8be19c86e` + the new test file only, one row at a time)

| Row | Result |
|---|---|
| R1 | ok |
| R2 | panicked `state_roundtrip.rs:197`: `StatePayloadError { code: "effect.state.history" }` |
| R3 | same |
| R4 | same |

This matches the attempt record.

## Mutations (merged tree; each applied, the full `cargo test -p soft-clip --no-fail-fast` run, then reverted)

| Mutation | Red (and the only red test binary) | Rows red |
|---|---|---|
| M1: the old `:764` rules | new gate-1 test | R2, R3, R4 (R1 green) |
| M1a: `X` refuses NaN | new gate-1 test | R4 |
| M1b: `e` refuses NaN | new gate-1 test | R2, R4 |
| M1c: dry finite only | new gate-1 test | R3, R4 |
| M1d: dry refuses inf only | new gate-1 test | R3 |
| M1e: `X` zero-or-normal only | new gate-1 test | R1, R2, R3, R4 |
| M2: `e` admits ±inf | rejection test, `word 43 = 0x7f800000` | -- |
| M3: `X` admits subnormals | rejection test, `word 12 = 0x00000001` | -- |
| M4: `e` admits subnormals | rejection test, `word 43 = 0x00000001` | -- |
| M5: `e` admits `-inf` only | rejection test, `word 43 = 0xff800000` | -- |
| M5 with the `bad(43,-inf)` row removed | **all green** | -- |

## Test value

- `a_snapshot_holding_non_finite_history_restores_and_continues_bit_for_bit`: it turns red on a
  restore that refuses a non-finite history word the effect produced itself. The cases are a NaN
  in `X` (R4), a NaN in `e` (R2, R4), a non-finite dry word (R3 inf, R4 NaN) and an overflowed
  ±inf in `X` (R1-R3). Under every such mutation (M1, M1a-e), no other test in the soft-clip
  suite went red.
- `bad(43, f32::NEG_INFINITY)`: it turns red on an `e` rule that refuses only `+inf` (M5). With
  the row removed, the whole soft-clip suite is green under M5, so the existing `bad(43, inf)`
  does not catch it.

## Gates run on `13302fceb` (all exit 0)

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- `bash scripts/check-workspace-policy.sh`, `bash scripts/test-workspace-policy.sh`
- `bash scripts/check-realtime-policy.sh` (89 marked regions in 25 files),
  `bash scripts/test-realtime-policy.sh`
- `cargo run --locked -p conformance --example conformance_fixtures -- --check` (nothing re-pinned)
- `cargo test --locked --release -p console-workload` (console digests unchanged)
- `cargo test --locked -p soft-clip -p conformance -p effect-compiler -p effect-contract`
  - It includes soft-clip `allocation.rs` (3), `randomized.rs` (2 + 1 ignored) and
    `state_roundtrip.rs` (11), and effect-contract `registry.rs`.
- `bash scripts/check-cross-targets.sh`: `cross-target matrix: PASS`. The only failures are the 10
  expected `ios-asm-memset-pattern16` (#1018) rows.
- Worklet chain:
  - `build-web-audioworklet.sh --named-twin`
  - `check-web-audioworklet.sh --without-metadata-regeneration`
  - `check-browser-expected-resources.py --artifacts`
  - `check-scalar-oracle-absent.py --wasm`
  - `test-web-audioworklet.sh`

  The same chain also ran on main `0a1176b3b` for comparison.
- `git diff --stat 8be19c86e c288358b4`: only `crates/soft-clip/src/lib.rs`,
  `crates/soft-clip/tests/state_roundtrip.rs` and the spec.

Evidence kept in `/tmp/claude-1002/v1300/logs/`: the gate logs, the mutation logs, `zz_probe.rs`
and the probe output, and the digests.
