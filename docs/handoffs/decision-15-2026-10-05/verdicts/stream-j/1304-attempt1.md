VERDICT: PASS

# #1304 attempt 1 — adversarial verdict (decision-15 stream J)

Commit `5304737c1` on base `e3375bc1d`. I reviewed an export of the commit
(`/tmp/claude-1002/v1304/src`) and did not touch the worktree. Logs are in
`/tmp/claude-1002/v1304/logs/`.

The diff touches exactly the three authorized paths: the spec, the
`REFERENCE_BUDGETS` doc comment plus three `four_lanes` values in
`crates/capi/tests/resource_lifecycle.rs`, and one debug-mode step in
`scripts/run-aarch64-tests.sh`. It changes no production code, no eight-lane
ceiling, no width-independent ceiling, `REFERENCE_MODEL_BUDGET`, the 10 % rule or
the assertions.

## Findings

No BLOCKER and no MAJOR.

### NIT 1 — one doc-comment line is not reflowed (123 columns)
`crates/capi/tests/resource_lifecycle.rs:1054` reads `/// 30,160), against 128,984 at
eight lanes. capi retained moved 258,231 -> 273,640 (+15,409) inside its budget: the effect`.
The new sentence was joined onto the old #1263 tail without reflowing it. The base had
one 101-column line in this comment; the change removes it and adds this 123-column
line. rustfmt does not wrap comments (`max_width = 100`), so `fmt --check` is green.
Fix: reflow lines 1052-1056 to 100 columns.

### NIT 2 — the doc comment's qemu command leaves out the linker
`resource_lifecycle.rs:1046-1049` names `CARGO_BUILD_TARGET` and the `_RUNNER`. It
does not name `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER`, which a cross build
from x86 also needs, so the command as written does not reproduce the rows. The spec's
attempt record has the full command. Fix: add "and that target's linker" (the script
header already says this), or point to the spec record.

### NIT 3 (informational, no change required) — the D4 "why" goes one level deeper
The implementer's diagnosis is correct: the move prediction was exact and the baseline
was not. 247,298 and 56,909 were never the four-lane rows. I measured further back
under qemu (lean build, see below):

| commit | four-lane graph session+plan | four-lane graph metadata |
|---|---|---|
| `a509b681` table (as documented) | 230,845 | 56,840 |
| `01f1bfa5d^` and `01f1bfa5d` (#1098) | 239,090 | 56,893 |
| `31b53b62c` (= `986301820^`) | 231,794 | 56,893 |
| `986301820` (#1263) | 341,210 | 166,309 |
| `5304737c1` (this commit) | 341,210 | 166,309 |

So #1098's four-lane graph move was +8,245, not +16,453. A later commit between
`01f1bfa5d` and `31b53b62c` then lowered four-lane session+plan by 7,296 and left
metadata alone. The old derivation was wrong twice over. The attempt record's
wording "#1098's eight-lane graph move (+16,453)" repeats the old comment's label for
what is really the cumulative eight-lane move up to #1263's parent. No edit is
needed; this is recorded so the next reader does not search for it.

## Objective gates (run by me)

| gate | command / method | result |
|---|---|---|
| Rows (D2) at the reviewed commit | D1's exact cargo form under qemu-user 8.2.2 (implementer's linker and sysroot, read-only), `RUSTFLAGS=-D warnings` as CI's `aarch64-debug` sets, full debug profile, fresh target dir | `graph_session_plus_plan_bytes: 341210 of 375360`, `graph_incremental_plan_bytes: 341210 of 375360`, `graph_metadata_bytes: 166309 of 182976`, `effect_bank_scratch_bytes: 12288 of 13568`, `effect_bank_runtime_buffer_bytes: 12288 of 13568`, `effect_bank_metadata_bytes: 921 of 1024`, test `ok`. Identical to the implementer's `final.log`. |
| D3 arithmetic | exact rational `ceil(row*11/10/64)*64` | 341,210 → 375,360; 166,309 → 182,976; 12,288 → 13,568; 921 → 1,024. All six match the committed values. |
| D4 parent rows | the same command on exports of `986301820^` and `986301820` (lean profile: `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`; that profile gives 341,210/166,309 on the reviewed commit, so it does not change the rows) | parent 231,794 / 56,893; #1263 341,210 / 166,309; move 109,416 = 9 × 2,104 + 3 × 30,160 on both rows. Confirmed. |
| Gate 2 (one-time mutation) | `graph_metadata_bytes` `four_lanes` set to 166,272 (the 64-multiple below 166,309) under qemu | red: `retained rows over their budget ... ["graph_metadata_bytes 166309 > 166272"]`. Reverted → green. (The implementer's `mut.log` shows the same on session+plan at 341,146.) |
| D1 "nothing rebuilds" | `RUSTC_BOOTSTRAP=1 cargo test -Z unstable-options --unit-graph` for the main run (`--all-targets`, 419 units) and for the D1 command (88 units), both for `aarch64-unknown-linux-gnu` | 0 of D1's 88 units differ from a main-run unit in package, target, mode, platform, features or profile, so the D1 step reuses the main run's artifacts. This is the same claim the record makes, which no saved log showed. |
| Narrow-build question | `-p capi --test resource_lifecycle` with the script's features | Not possible: cargo refuses (`capi` does not contain `builtins-compiler/test-support, builtins/test-support, effect-compiler/test-support, math/lane, parametric-eq/test-support`). With only the four features capi can name, 12 units resolve differently (`lane` loses `test-support`, `builtins` and `builtins-compiler` lose `test-support`, and some registry features differ), so it is not the CI feature set and could in principle move a row. The D1 command itself is already the narrowest equivalent build: 88 units, 1.4 GB with debuginfo, about 20 s. |
| Gate 3 x86 | `cargo test --locked -p capi --test resource_lifecycle` | 11 passed. Eight-lane rows unchanged: 382,918 of 421,248 (×2), 185,121 of 203,648, 16,384 of 18,048 (×2), 805 of 896. |
| Gate 3 routing | `python3 -B scripts/test-ci-path-routing.py`; `python3 -B scripts/check-ci-path-routing.py`; `scripts/ci-path-router.py --flags --event pull_request` on the diff's name-status | pass; pass; `route=full`. Each changed file alone also routes `full`, so the AArch64 jobs run. |
| Gate 4 | `cargo fmt --all -- --check`; `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`; `bash scripts/check-workspace-policy.sh`; `bash -n scripts/run-aarch64-tests.sh` | all pass |
| Hazard: known-defect rows | `scripts/lib/aarch64-known-defects.py rows debug` | empty. The D1 step names no expected-failure row and is not a skip. |
| Not run | the full AArch64 debug and release legs (native arm64, PR CI) | Spec D2 assigns these to the PR's own qualification run. Disk was too tight for a whole-workspace `--all-targets` qemu build. Gate 1's "PR body" table and Gate 2's CI-log reading are PR-time items; the spec's attempt record already has the table and the log lines. |

## Test value

No test is new or rewritten. For the three tightened four-lane ceilings: a structural
four-lane regression in graph session+plan, incremental or metadata of more than about
10 % now turns `reference_session_retained_rows_stay_within_their_budgets` red on
AArch64. The old ceilings (413,952 and 204,544) passed a regression of up to +21.3 %
(session+plan and incremental) or +23.0 % (metadata) over the measured rows. The x86
eight-lane ceilings never see a four-lane-only per-lane term. Reproduced: metadata ceiling one
64-byte step below the row → red; restored → green.

## Pre-existing (not a #1304 finding)

The implementer saw 1 uncaptured failure in 15 x86 runs of `-p capi --test
resource_lifecycle`. An earlier slice saw
`live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`
fail once under heavy load with the raced plan's final block `[0, 0]` on both lanes.
I could not reproduce either:

- 30 runs of the full test binary, 6 concurrent at a time, on a host already at
  load average ~34 on 32 cores: 30/30 `ok`.
- 30 runs of the racing test alone (`--exact`), 3 streams in parallel, while 24
  busy-loop hogs ran for 200 s: 30/30 `ok`.

#1304 cannot cause it. The racing test reads `limits()`, not `REFERENCE_BUDGETS`, and
the commit changes only constants and comments that test does not read. A `[0, 0]`
final block passed the reference's "carries signal" check but would fail the bitwise
comparison against a fresh plan of the final snapshot. That means the raced plan
emitted silence after settling, which may be a real lost-edit or lost-source defect
rather than harness timing. Recommendation: a stateless issue with a capture loop that
keeps the failing log and the run index, so the next occurrence is diagnosable.

## Disk

Every build dir I created has been deleted (`/tmp/claude-1002/v1304/target*`); only
the 47 MB source export and the logs remain. Free space was never below 4.7 GB during
the review.
