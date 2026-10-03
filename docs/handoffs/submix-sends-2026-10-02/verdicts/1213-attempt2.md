# #1213 *Address submix strips in browser live commands*: Sol verdict, attempt 2

- Reviewed: `git diff 4df439f2c 89ba1b233` (attempt 2: 9 files, +252/-34), judged together with
  attempt 1 (`e4a6269fe`, `c13ac5e1d`) and its FAIL verdict (`1213-attempt1.md`). Branch
  `codex/batch-submix-k2`, worktree `/home/bl/misofm/wt-submix-k2`.
- Binding: `AGENTS.md`, and
  `.github/ISSUE_SPECS/1213-address-submix-strips-in-browser-live-commands.md` with amendment A1,
  the A2 path addition and the Attempt 1 and 2 records.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1213`: OPEN, and the title
    matches the spec's H1.
  - I exported `89ba1b233` to `/tmp/claude-1002/v1213b/src`, copied `sdk/node_modules`, and used my
    own `CARGO_TARGET_DIR` and `TMPDIR`. I ran mutations in place, scripted, and restored them
    byte for byte. Afterwards, `diff -r` against a fresh archive differed only by the generated
    `sdk/dist`. Scratch deleted.
  - Attempt 2 changed only docs, tests and the JS harness; I found no engine change.

## Verdict: PASS

**No BLOCKER and no MAJOR.** The attempt 1 MAJOR and all four MINORs are resolved, and every
recorded red mutation reproduces. The shipped module is unchanged. Four non-blocking items are
listed below (one MINOR, three NITs).

## Gates (all run from `89ba1b233`, x86_64 AVX2, all rc 0)

| Gate | Result |
|---|---|
| `cargo test -p host-web --lib` | 141 passed, 0 failed, 1 ignored |
| `build-web-audioworklet.sh --named-twin` | Shipped module `7d6c0a8b...0f90b`, 2 695 834 B, the same as attempt 1. **ARTIFACT UNCHANGED** confirmed |
| `check-web-audioworklet.sh <A> <B>/...named.wasm` | rc 0 |
| `check-browser-expected-resources.py --artifacts` | rc 0 |
| `test-web-audioworklet.sh` | rc 0, including both new attempt 2 blocks |
| `check-sdk-generated.sh <A>`, `check-sdk-types.sh` | rc 0 |
| `check-sdk-headless.sh <A>` | 346 pass, 0 fail. The A1a, D5a and A1e evals pass |
| `sdk-package.sh check` | rc 0 |
| `cargo fmt --all -- --check` | rc 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | rc 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | rc 0 |
| `check-`/`test-` realtime, host-core and workspace policy | all rc 0 |

- The `.d.ts` and `sdk/src/browser/shipped-host.d.ts` are byte-identical (`cmp`).
- **Not run:**
  - the DESIGN section 7 workspace test command. This is the same disk-driven skip as attempt 2's
    record. The attempt 2 diff is host-web tests and docs only, and workspace clippy
    `--all-targets` compiled every test target;
  - `run-aarch64-tests.sh` (no arm64 host).

## Attempt 1 findings: resolution

- **MAJOR-1 (`frameSlot`): resolved.**
  - `MisoObservationBinding.frameSlot` is now defined as the strip index, equal to `trackIndex`:
    below `trackCount` it indexes `trackGrDb`, and from there it reads
    `submixGrDb[frameSlot - trackCount]`.
  - These sites are fixed in the `.d.ts` and its byte mirror, and they are accurate:
    - the addressing header;
    - `MisoCommandReason.UnknownTrack`;
    - `MisoCommand.trackIndex`, which also notes that `Solo` at a submix gives `NotSoloable`;
    - `MisoSessionMap.tracks`;
    - `MisoObservationSubscription.trackIndex`;
    - `sessionMap()`.
  - The host JS comment and `EFFECT_OBSERVATION_V1.md` match.
  - The harness assertion behaves as follows:
    - Host `frameSlot: 0` turns it red (`0` vs `2`).
    - My own mutation, which makes the bus slot submix-local (`trackIndex - T`, the plausible
      wrong "fix"), also turns it red.
    - Reading the mapping as a plain `trackGrDb` index (the old contract) turns it red
      (`undefined` vs `1.25`).
  - The old-contract mutation changes the test's own helper, not the product. The assertion that
    actually defends the product is `busBinding.frameSlot === map.tracks.length`. That is
    adequate, because the defect was a contract defect.
- **MINOR-1 (prepared EQ): resolved for the stated test value.**
  `a_prepared_eq_edit_on_a_bus_equals_the_same_edit_on_a_track` is red under P2
  (`prepared_queue_address` Eq base `tracks.len() * 3`) and under P3 (the owner marker
  `tracks.len() * 3 + effect`): the bus submission returns 1, not 0. See MINOR-A for a caveat.
- **MINOR-2 (lane): resolved.**
  `a_single_lane_bus_mute_equals_the_bus_booted_with_that_lane_muted` is red under these
  mutations:
  - P1 (`lane = 0`): block 2, sample 128, the right lane;
  - my own swap, `usize::from(!matches!(lanes, Right))`.
- **MINOR-3 (worklet D5a): resolved.** The `testProcessor()` block calls `receiveEqTargetConfig`
  directly. It is red under these mutations:
  - the `this.trackCount` revert (`2` vs `4`);
  - my own off-by-one mutation, `>` for `>=` (`T + S` gives `4` vs `2`).
- **MINOR-4: resolved.** `WebObservationSelection.track_index` and
  `WebObservationResult.track_index` now say strip index. `admit_commands` says per strip and
  `2 * strip_count`.
- **NITs 1, 2 and 4: done.** NIT 3 (the optional typed command builder) was declined, which is
  acceptable.

## Mutations run

| Mutation | Result |
|---|---|
| P2: `prepared_queue_address` Eq base `ready.tracks.len() * 3` | **Red**: prepared-EQ arm (1 vs 0) |
| P3: the admission EQ owner marker `ready.tracks.len() * 3 + effect` | **Red**: prepared-EQ arm (1 vs 0) |
| P1: kind 4 `let lane = 0_usize` | **Red**: single-lane bus mute |
| Mine: kind 4 lane swapped | **Red**: single-lane bus mute |
| Worklet classifier `>= this.trackCount` | **Red**: `2` vs `4` |
| Mine: worklet classifier `>` for `>=` | **Red**: `T + S` gives `4` vs `2` |
| Host `frameSlot: 0` | **Red** |
| Mine: host `frameSlot` made submix-local for a bus | **Red** (`0` vs `2`) |
| Harness mapping read as plain `trackGrDb[frameSlot]` | **Red** (`undefined` vs `1.25`) |
| Mine: B's EQ edit `-6` while A's stays `-12` (test-strength probe) | **Survives**. See MINOR-A |
| Mine: the push pass drops a bus's companion targets | **Red**: publishing zero targets is refused with 255, so the silent-drop class is closed |

## Findings (non-blocking)

### MINOR-A: the prepared-EQ arm's render comparison is vacuous

- **The cause.** The arm builds the EQ with `eq.params = Vec::new()`. `band-1-enabled` defaults
  to `0`, so the band-1 gain edit (parameter 4, -12 dB) changes no sample.
- **The proof.** Giving host B -6 dB while A gets -12 dB leaves the test **green**. Its
  `assert!(audible)` is also trivially true, because noise is never silent.
- **What still holds.** The test-value sentence names only the P2/P3 spellings. Those are
  refused at admission, and the `RESULT_OK` assertion catches them, so the stated claim is
  accurate. The doc line "renders the bits the same edit renders" is not.
- **Origin.** The flaw came from the attempt 1 verifier's own probe, which this attempt
  committed verbatim.
- **Fix (proven in scratch).** Declare `parameter_id: 1, Both, Linear, 1.0` in `eq.params`. The
  arm then stays green on `89ba1b233` and turns red under the -6/-12 probe. Do this when the file
  is next touched; it does not hold this issue.

### NIT-A: `lib.rs` line-count preservation

- **The claim is true.** I rewrapped the 132-column `admit_commands` doc line onto two lines and
  rebuilt `--module-only`:
  - 24 bytes moved (the `core::panic::Location` line words);
  - the digest changed from `7d6c0a8b...` to `f08c5433...`.
- **It is not necessary.**
  - Since #1061 (owner decision 5) no per-change module pin exists. The `artifact-identity` job
    only reports, and the committed `.sha256` is a release fingerprint, re-pinned at release.
  - So no gate breaks if a doc edit moves panic-location bytes. "ARTIFACT CHANGED" is then the
    honest report.
- **It is not a fragile pin, but it is a source contortion.** It left a 132-column doc line
  (`lib.rs:4306`; the base already had 5 overlong lines, so no gate enforces the width). It
  should not become a practice.
- **Where the expectation came from.** The attempt 1 verdict's line "the shipped Wasm module's
  bytes should not move" invited it. That expectation is withdrawn: a docs-only change may move
  module bytes through panic locations.
- **Fix.** Wrap the line at the next `lib.rs` touch.

### NIT-B: an overlong `.d.ts` line

- `.d.ts:51` (and the mirror) has a 158-column line, left over from the addressing-header rewrap.
- It is cosmetic. The `.d.ts` is not compiled into the module, so no line-count reason applies.

### NIT-C: leftover "track" wording on the master and observation surfaces

- `MisoMeterFrame.masterGrDb` (`.d.ts`) still says "the designated master track's own folded
  reading ... when no track was designated". The same sentence appears in
  `EFFECT_OBSERVATION_V1.md`.
- `MisoObservationAddress.trackIndex` and `MisoObservationMapBinding.trackIndex` have no doc.
  They are strip indices since A1a.
- None of these sites is in D6's list, and none is wrong about a wire value. Fold them into the
  next doc pass.

## Whole-issue assessment (attempts 1 and 2)

- **Engine (D1 to D5).** Attempt 1's verified state stands: correct indexing, all-or-nothing
  admission and solo composition. Attempt 2 changed no engine logic, and the identical module
  digest confirms it.
- **D5a.** Both classifiers are now pinned:
  - the SDK classifier by the headless eval;
  - the worklet classifier by the harness block.
- **D6.** Complete at every listed site. `frameSlot` and the addressing docs are also fixed now.
- **Hazards.**
  - Acked-batch: no ack precedes a drop. My bus-only target-drop mutation is refused (255), not
    silently acked.
  - `solo_safe` is pinned by gates 2b and 3.
  - `S = 0` is unchanged: no existing test was edited.
- **Test value.** Every new attempt 2 test has a reproduced red mutation, and none greps source
  or pins bytes. The one weakness is MINOR-A, which is non-blocking.
