PASS

# #1279 attempt 2 -- adversarial slice verdict

Range: attempt 2 is `341b58d8f` (parent `fab031fdb`). I also reviewed the whole #1279 diff `1080ccbc0..341b58d8f`.
I reviewed an export (`git archive 341b58d8f`) under `/tmp/claude-1002/v1279/`. I did not edit, build or commit in
`/home/bl/misofm/wt-d15-a2`. Small evidence is in `/home/bl/misofm/submix-verdicts/1279-attempt2-evidence/`
(gate summaries, the mutation and probe diff, the mutation summary, the oracle probe, the capi audit record).

No BLOCKER and no MAJOR. Attempt 1's MAJOR 1 is fixed, and I measured the fix. One new NIT (a line that the NIT 1
rewrap made wider) and the items that are with root.

## Attempt 1 findings

| Finding | Status |
| --- | --- |
| MAJOR 1 (gate 1 blind to the gate/expander) | **Fixed.** Measured below. |
| MINOR 1 (inserts not in `restarted_strips()`) | Done. The rustdoc (`host-core/src/prepare.rs:574-576`) and the record state it. Root confirms the reading. |
| MINOR 2 (`capi/tests/resource_lifecycle.rs`) | With root (named exception). Not changed. |
| NIT 1 (lines over 100 columns) | The five lines are fixed, but the rewrap made one new line of 121 columns. See NIT 1 below. |
| NIT 2 (`payload_sections_agree`) | Done in `rack/src/lib.rs:972`. The `symmetry.rs` doc edit is with root. |
| NIT 3 (`cfg_attr` ignore) | Done for all nine four-lane cases. |
| NIT 4 (no production reader of `carry_refusal_count`) | Left to root. The record does not name the successor. |
| NIT 5 (stale `DESIGNED` premise) | With root (spec premise). |

## MAJOR 1: how I checked the fix

**Mutation used.** In `resolve_effect_carry`, a slot of one effect (selected by an environment variable) writes no
move and does not set `restart`. So the strip is not listed, and only the bits can show the defect. This is the
mutation the task names. I built one binary and ran it once per effect.

| Slot not carried | `console_effect_state_survives_a_swap_at_eight_lanes` | `every_console_effect_does_work_at_eight_lanes` | Other red tests |
| --- | --- | --- | --- |
| parametric EQ | red on bits, block 9 | red, block 6 | collapse, prepared change, added insert |
| compressor | red on bits, block 9 | red, block 6 | collapse, prepared change, added insert |
| gate/expander | **red on bits, block 10** | **red, block 7** | prepared change, removed slot, added insert |
| transient shaper | red on bits, block 9 | red, block 6 | prepared change, added insert |
| soft-clip | red on bits, block 8 | red, block 6 | prepared change, added insert |
| true-peak limiter | red on bits, block 6 | red, block 6 | collapse, prepared change, added insert |

These blocks agree with the attempt 2 record. With no mutation, `successor_swap` passes all tests (65 passed, 9
ignored).

**Extra mutation: wrong lane.** Attempt 1 could not measure this case. I rotated the predecessor locations among
one effect's moves, so each lane gets the state of another strip's lane:

| Rotated effect | Gate 1 | `every_console_effect_does_work` |
| --- | --- | --- |
| gate/expander | red, block 10 | red, block 7 |
| parametric EQ | red, block 9 | red, block 6 |
| limiter | red, block 6 | red, block 6 |

With the attempt 1 feed, gate 1 stays **green** when the gate's lanes are rotated. So the old feed hid wrong-lane
restores of the gate too, as attempt 1 suspected. The new feed closes that gap.

**The spliced-feed oracle is sound: not vacuous and not self-referential.** I added a probe test, which is not
committed. For each slot alone, it compares three runs: the swapped run, the reference (a fresh B from frame 0) and
`at_rest` (a fresh B on the spliced feed). See `oracle-probe.txt`.

- **No mutation.** For all six slots, the swapped run equals the reference. `at_rest` differs from the reference
  from the swap block: block 6 for five slots, and block 7 for the gate, which is in its release.
- **One slot's carry suppressed.** For every one of the six slots, the swapped run equals `at_rest` *exactly* in
  every block from the swap (`run_vs_atrest = None`). So `at_rest` is the exact render of "this slot restarted at
  rest". The `assert_ne` therefore checks a real counterfactual and not an approximation.
  - It is not vacuous. A slot whose state at the swap equals rest makes the `assert_ne` fail.
  - It is not self-referential. `at_rest` is a fresh plan on independent PCM and uses no carry code.
- **The test defends the feed (its stated defect).** I put back the attempt 1 envelope (`500..1000`) with an
  environment switch:
  - With no carry mutation, the only red test in the binary is `every_console_effect_does_work_at_eight_lanes`,
    with "gate at the swap holds state its rest does not" (65 passed, 1 failed).
  - With the old feed and the gate's carry suppressed, gate 1 is green and the new test is red.
  - So a future feed change that hides one effect from gate 1 now turns a test red by name.
- **Limiter note.** The limiter's `at_rest` first block is all zeros (look-ahead latency). The mutated run's swap
  block is the same zeros, so the oracle is still exact.

**Four lanes.**
- The rewritten test runs at `Backend::Simd4` on AArch64 CI. The test exists at each width because the feed was
  tuned at eight lanes.
- `cargo check --tests --target aarch64-unknown-linux-gnu -p host-core -p rack -p graph` with `RUSTFLAGS=-D warnings`
  passed (rc 0).
- I did not run AArch64 tests, as the rules require.

## NIT 2 and NIT 3

- **NIT 2.** `payload_sections_agree` is `left.len() == right.len() && left == right`
  (`effect-contract/src/symmetry.rs:385-387`).
  - It is a pure slice comparison: no allocation, no panic, bounded by the payload size.
  - It means the same as the old `left == right`.
  - The call stays inside the `rack` realtime region, and `check-realtime-policy.sh` passes (112 regions in 27
    files).
  - Its doc still says "Off the render thread, at restore time". The one-line edit is outside the authorized paths
    and is with root.
- **NIT 3.** All nine four-lane cases are `#[test] #[cfg_attr(not(any(target_feature = "neon", target_feature =
  "simd128")), ignore = "...")]`. On x86-64-v3 they compile and are listed as ignored (9 ignored in
  `successor_swap`).
  - On AArch64 `neon` is on, so they are not ignored.
  - The `aarch64-debug` job runs `scripts/run-aarch64-tests.sh debug`. That script runs `cargo test --all-targets`
    over the product crates (host-core is one) with `host-core/test-support`, and no skip row names these tests.
    So the cases run natively there.
  - The eight-lane cases keep `#[cfg(target_feature = "avx2")]`, which was the file's convention before this slice.

## Findings

### NIT 1 -- the NIT 1 rewrap made a new 121-column doc line

- **Where:** `crates/effect-contract/src/lib.rs:2594`, which reads "/// effects -- both kinds clear the *running*
  state to constants but re-seed the *designed* words from each channel's".
- **What is wrong:** the rewrap moved the over-wide text down one line. It did not wrap it.
- **The record is wrong about it:** it says "The only other lines over 100 columns in the range are two shell lines".
  For the whole range `1080ccbc0..341b58d8f`, the over-wide added lines are this one and the two realtime-policy
  shell lines (104 and 109 columns).
- rustfmt does not wrap comments, and no gate checks this.

### Items with root (not counted against the attempt)

- The named exception for `crates/capi/tests/resource_lifecycle.rs` (`7852f0a25`), and for #1277's two tests in
  `crates/engine/src/realtime/mod.rs`.
- Gate 2's premise error. The gate/expander, soft-clip and transient shaper never collapse, so gate 2 uses EQ,
  compressor and limiter.
- The reading that inserts start at rest until #1281/#1282 without going into `restarted_strips()`.
- The stale-`DESIGNED` test-value premise (attempt 1 NIT 5).
- The `symmetry.rs` one-line doc edit (NIT 2).
- NIT 4: the record still names no successor that surfaces `carry_refusal_count` in production.

## Other checks

- **Production change in attempt 2:** only `rack/src/lib.rs:972`, which means the same as before. Everything else is
  docs, the test feed and tests.
- **Paths:** attempt 2 changes no file outside the authorized paths. The record names the root items.
- **Acked-batch question: can an ack ever precede a drop?** No.
  - Neither attempt adds a queue or an ack.
  - The effect-carry install and its cap checks come before the stamp.
  - A refused restore at the swap block is D6's counted defect path, not a dropped command.
- **Person-role wording:** no added line in the range uses producer, fan, listener, musician, artist, engineer or
  user.
- **Version suffixes:** attempt 2 adds no new name.

## Test value

- **`every_console_effect_does_work_at_{eight,four}_lanes` (rewritten):** catches a test feed (or session) under which
  one console effect's state at the swap block renders the same as its state at rest, so that gate 1 cannot hear
  that effect's carry. With the attempt 1 feed, it is the only red test in `successor_swap`. It also catches a
  one-effect carry defect (not moved, or moved to the wrong lane) with that slot alone on the strip.
- **`console_effect_state_survives_a_swap_*` (gate 1, unchanged except for the feed):** now catches a not-moved or
  wrong-lane restore for all six effects, the gate/expander included (red at block 10 for both). Before this attempt
  it was green for the gate.

## Gates run (all from the export, toolchain 1.97.1, x86-64-v3; all pass)

- **`lint` job, every step in workflow order** (`lint-summary.txt`):
  - fetch, fmt, clippy `--all-targets --all-features -D warnings`, and Documentation
    (`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`).
  - Workspace, session, environment-vocabulary and bench policies.
  - The test-support CI policy and its mutations, and the scalar-oracle self-test.
  - Script reachability failed in the plain export because it has no `.git`. It passed in a git-initialised copy of
    the same export (120 files, 20 cases; `lint-reach-git.log`).
  - Console benchmark fixture, the builtins fixture-manifest mutation tests, and the bench preconditions.
  - Host-core and protocol policies.
  - Realtime policy (112 regions in 27 files) with its self-test, audit-leak and artifact-evidence-leak.
  - Trace validator, lane policy and the unfused seal.
  - Rack, builtins and graph policies (including `check-rack-policy`), with their mutation tests.
  - Effect-runtime policy and fixtures, conformance boundaries, and the PEQ render contract.
  - Release-shape self-test, npm publish modes, stem store and stem identity.
  - Both sub-v3 refusals and the AVX2+FMA probe.
- **`test-release` job, every step** (`rel-summary.txt`):
  - lane, math and wasm-gates in release, `filter_liveness`, `tail_contract`, `ramp_endpoint` (8 crates) and
    `designer_total --ignored`.
  - M3's FMA cfg.
  - The loom leg (`RUSTFLAGS='--cfg loom ...' cargo test --release -p engine --lib spsc_loom`).
  - M1 and F1 exhaustive.
- **Spec gate 5:**
  - `cargo test --locked -p rack -p graph -p host-core` with the test-support features: 459 passed, 0 failed,
    11 ignored. `successor_swap` alone: 65 passed, 9 ignored.
  - The release `audit` build with `trace-graph-audit.sh`: PASS, 1,000,000 blocks.
  - `cargo test -p console-workload`.
  - The policy, clippy and doc steps above.
- **C ABI:**
  - `cargo test --locked -p capi`: 103 passed.
  - `check-capi-abi.sh` and `--self-test`.
  - `audit capi`: 100000 calls, with 0 allocations, deallocations, locks, syscalls and violations
    (`capi-audit.json`).
- **Also run:**
  - `trace-builtins-audit.sh` and `trace-builtins-graph-audit.sh`.
  - The builtins manifest digest (`check-builtins-fixtures.sh . target/release/audit`, 50 files) and
    `check-console-fixtures.sh`.
  - `check-cross-targets.sh`.
  - The CI workspace debug test command with `engine/realtime-audit`: 1589 passed, 0 failed.
- **Worklet chain** (`wl-summary.txt`):
  - The named-twin build with the strip-names check.
  - `check-web-audioworklet.sh --without-metadata-regeneration` and `check-browser-expected-resources.py
    --artifacts`.
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh` with a private TMPDIR, which was empty afterwards.
  - The V8 spill gate and its self-test.
- **Not run:** AArch64 tests (CI only). The four-lane cases compile for `aarch64-unknown-linux-gnu` with
  `-D warnings`.
