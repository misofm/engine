PASS

# #1329 attempt 5 -- adversarial verdict (Sol verifier, 2026-10-06)

Reviewed: `git diff 0f8add7a0 fadc45df0` on `codex/d15-stream-g` (590201615 Amendment 5 and
implementation, fadc45df0 record), and the whole #1329 change as it stands. I checked it against
the spec with Amendments 1-5, AGENTS.md, decision 15 D15-4 and the owner's no-shortcuts rule. All
work was done on exports: `tree` = fadc45df0, `base` = 0725a8949, and `var`, a scratch copy for
mutations and one measurement variant. Nothing was edited, built or committed in the worktree.
Small evidence is in `/tmp/claude-1002/v1329c/evidence/`.

**Verdict.** Every item in Amendment 5's scope is done. MJ1 is fixed: on the live-control browser
boot the cost is back to the pre-#1329 figures. The counted test is red on attempt 4's behaviour.
The seal check's live rule and the right channel are now tested. The `nu` and `STEP_UP` arguments
are sound; I re-derived both. No certified figure moved: release `tail_contract` prints the same
300 figure lines as attempt 4. No math constant changed: the `crates/math` diff is doc comments
only. Every gate is green. There is no BLOCKER and no MAJOR finding.

There are two MINOR findings, and both are false statements:
- **m1.** Root ruling A says that the browser computes no design bound. That is false for the
  browser's audio-only boot, which is the SDK default. I measured that boot at +81 % per
  64-track session. This needs a text correction before #1457's GitHub body is synchronized.
- **m2.** One test-value line says "either channel", but only the right channel is tested.

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. Root ruling A, #1457's amended context and one code comment say that the browser computes no
design bound. That is true only when the browser boots with live controls. An audio-only browser
boot is the SDK default. It bounds every distinct design on the AudioWorklet thread, at +15.5 ms
(+81 %) for each 64-track boot.**

- *Where.*
  - `.github/ISSUE_SPECS/1457-cache-design-bounds-across-preparations-within-a-stated-preparation-budget.md:42-49`
    says: "The browser prepares every strip with a live input lane (`HostLiveLanes::ALL`), so it
    computes no design bound at all; its boot cost is back to the pre-#1329 figures ... that is the
    platform this issue's budget binds". The amendment also deleted the earlier "after which the
    browser pays the cost on a Worker".
  - `crates/builtins-compiler/src/lib.rs:3573` says "the browser, whose every strip is live,
    computes none".
  - The #1329 spec, Amendment 5, ruling A (`:389`) says "The browser computes no design bound". The
    MJ1 summary (`:375`) says "where every strip is live".
- *Why each statement is false.*
  - `HostLiveLanes::ALL` attaches an input lane only to a strip that has a control request
    (`crates/host-core/src/prepare.rs:1436-1446`).
  - The browser makes control requests only when `live_control_command_queue_records != 0`
    (`hosts/host-web/src/lib.rs:7225-7228`). `WebBootOptions::explicit_defaults()` sets that word to
    0.
  - The SDK writes `liveControls?.commandQueueRecords ?? 0` (`sdk/src/core/abi.ts:189`). The SDK
    README says "Audio-only boot remains valid without it".
  - So the SDK default (`createEngine` without `policy.liveControls`) attaches no input lane. Every
    strip then reports its own design bound, which preparation computes. In the browser this runs
    on the AudioWorklet thread today.
- *Measured.* I ran the repository's rebuild-cost proxy (`rebuild-round`) once, with two rounds:
  Node v22.23.2, `--no-liftoff`, `taskset -c 7`, 25 boots per document, no failed boot, every boot
  audible. For the audio-only boot I used a copy of the script that changes one line:
  `COMMAND_QUEUE_RECORDS = 0` (`evidence/web-rebuild-audio-only.mjs`). Boot p50 in ms, round 1 /
  round 2:

  | boot | module | 9-track EQ | 64-track console | 64-track app shape | 64-track sends |
  |---|---|---|---|---|---|
  | live controls | pre-#1329 `10a3c816…` | 2.15 / 2.16 | 20.31 / 20.36 | 19.75 / 19.92 | 33.86 / 34.04 |
  | live controls | attempt 5 `ec1d2f66…` | 2.21 / 2.25 | 20.20 / 20.48 | 19.69 / 19.88 | 33.60 / 34.18 |
  | live controls | attempt 5 with the 48 kHz live bound as a constant (`ad09489c…`, measurement only) | 2.15 / 2.13 | 20.11 / 20.19 | 19.62 / 19.71 | 34.14 / 34.00 |
  | **audio-only** | pre-#1329 `10a3c816…` | 2.01 / 2.00 | 19.13 / 19.13 | 18.63 / 18.57 | 32.60 / 31.26 |
  | **audio-only** | attempt 5 `ec1d2f66…` | **2.24 / 2.24** | **34.66 / 34.68** | **34.17 / 34.23** | **46.61 / 46.61** |

- *Why this is MINOR and not MAJOR.*
  - The code is correct.
  - In an audio-only boot the design bounds are not waste. They are the bounds that the non-live
    strips report and that graph lowering reads, as D4 and D7 require.
  - R7 accepted this cost and gave it to #1457, together with #1332 (stream H). It was there in
    attempt 4, too.
  - What is wrong is the record. The statements tell #1457 that its budget binds only the C ABI,
    and they say that the browser's boot cost is back to the pre-#1329 figures. On the SDK default
    path, the browser can still hold the AudioWorklet thread for the full #1457 worst case until
    #1332 lands. About 0.7 s is estimated for 64 distinct near-top designs.
  - The source of the error is the attempt-4 verdict's premise ("in the browser every strip is
    live"). Root ruling A took that premise over, and the implementer followed the ruling exactly.
- *Fix (root's documents, text only).*
  - Restate #1457's "Where preparation runs" as follows. A browser boot with live controls bounds no
    design: each strip reports the live bound. An audio-only browser boot, which is the SDK
    default, bounds every distinct design on the AudioWorklet thread until #1332, and on a Worker
    after #1332. The C ABI bounds every distinct design on its control thread. #1457's budget binds
    both hosts.
  - Correct ruling A and the comment at `builtins-compiler/src/lib.rs:3573`. A suitable form for
    the comment: "a browser boot with live controls computes none".
  - Do this before #1457's GitHub body is synchronized. The GitHub body does not yet have the
    amendment (`gh issue view 1457`).

**m2. The m2 test-value line says "including either channel", but a key that drops the left
channel is not caught.**
- *Where.* Spec `:647`; the test is at `crates/builtins/tests/tail_contract.rs:302-333`.
- *Why the left channel is not caught.* The new sixth strip has HPF 1000 Hz on the left and HPF
  10 Hz on the right. Its bound equals the bound of strip 1 (HPF 10 Hz on both channels), because
  the 10 Hz channel dominates. So a key that drops the left channel (`[lanes[1], lanes[1]]`, M5)
  gives the sixth strip strip 1's bound. That bound is the same value, so the test stays green.
- *Measured.* M5 is green in release `tail_contract`, in every builtins-compiler lib test and in
  the builtins lib tests. M5 is the mirror of C3. A real session whose left channel dominates gets
  an unsound bound under it.
- *Status.* The test delivers exactly what Amendment 5's m2 asked for: the right channel, which
  M4 (C3) turns red. Only the claim is too broad.
- *Fix.* Add one strip that differs from the first only in its left HPF (10 Hz), and add
  `assert_ne!(own[6], own[0])`. I probed this change (`evidence/left-only-probe.diff`). It is green
  on the shipped code, red on M5 and still red on M4. The other fix is to restrict the line to the
  right channel. Under the no-shortcuts rule, adding the strip is the better fix.

## NIT

- **The 9-track residual (+0.07 ms) is the live bound, not noise.**
  - The variant module differs from the head module only in that it returns the 48 kHz live bound
    as a constant. Its size is 3,053,689 B against 3,053,802 B, so module size is not a factor.
  - The variant boots the 9-track document in 2.15 / 2.13 ms, which equals pre-#1329 (2.15 /
    2.16). Head takes 2.21 / 2.25 ms.
  - So the record's open candidate is confirmed: the residual is the once-per-boot live-bound
    computation, about 0.07 ms in V8. Every live-control boot pays it, also the 64-track ones,
    where it is inside the noise.
  - The work is not waste, because every live strip reports that bound. It depends only on the
    rate, and there are four launch rates. #1457's cache, or a per-rate table checked by a test,
    removes it. It does not matter for render, and it is about 3 % of a 2 ms control-plane boot.
    Record it in #1457.
- **The gate 3 strike-through also strikes the part that stands.** Spec `:612-615`: the whole line
  is struck through, but the note says "the upper side's claim stands". Strike only the lower-side
  sentence, or repeat the upper-side claim outside the strike.
- **The new citation in `docs/rulings/effect-floor-accounting.md:473-476` is not complete.** It
  names `fixed_input_bound` and `filter_response.rs`. `enabled` is also read in the
  response-snapshot builder at `crates/builtins/src/lib.rs:2395`, which is also control plane. The
  ruling's point (control plane only) is correct.

## Attempt-4 findings: status

| # | finding | status | evidence |
|---|---|---|---|
| MJ1 | design bounds computed and discarded for live strips | **Fixed** | `.filter` over live strips (`builtins-compiler/src/lib.rs:3576-3585`), consumed in strip order (`:3631-3639`). Counted test: 6 / 0 / 4. M1 (attempt 4's behaviour): 6 against 0, the only red. Live-control boot within noise of pre-#1329 (table above). Restating #1457: done as ruled, but the premise is false for audio-only boots (m1). |
| m1 | seal live rule untested | **Fixed** | Forged payload and seal (`:12034-12046`). M3 (E2) is red with `BuiltinDiagnosticSet([])`; the other 54 builtins-compiler lib tests stay green. |
| m2 | right channel unseen | **Fixed for the right channel** | M4 (C3) red in `each_strip_…` (strip 5 gets strip 0's bound) and in the counted test (5 against 6). The left mirror M5 is green everywhere: see the new m2. |
| m3 | false live-identity test-value line | **Fixed** | The restated line (spec `:634-640`) agrees with the attempt-4 measurements (B6 red on both tests; the identity's own catch is below the oracle's tolerance). |
| NIT | `nu` proof text | **Fixed** | Re-derived. `A = [[1-2c1, -2a2],[2a2, 1-2a3]]` (`math/src/tail.rs:153-158`). For `k >= 0`, `c1, a3` lie in `[0,1)` and `2 a2 <= 2g/(1+g^2) <= 1`, also after the `f32` cast (monotone rounding; `1.0` is representable; Butterworth `2 a2 <= 0.586`). Per word the error is `((1+u)^3 - 1) = 3.0000000000000003 u`. `sqrt(2) 3.01 = 4.2568`, so `8/4.26 = 1.878` (`8 / (3 sqrt(2) (1+2^-24)) = 1.886`). `R_NORM` = 1.3065629648763772 >= exact `sqrt(1 + r)` = 1.30656296487637653. |
| NIT | `STEP_UP` argument | **Fixed, sound** | I counted every `STEP_UP` site (`:952`, `:957`, `:963`, `:971`). `out.input[1]`: `|h|` and the output rounding pass through two additions, and `gamma e` through a product and one addition. The inner roundings of `|h|` belong to the output-rounding term, which covers its own evaluation (`8u` against `6.01u`; at most 6 roundings per term, counted on `c`, `d` and the output sum). Later inputs and states: a product and one addition. `(1-u)^3(1+4u) - 1 = 0.999999999999999 u > 0`. The radius growth term: a sum, a product, an addition and `STEP_UP` give `(1-u)^4(1+4u) = 1 - 9.99999999999999 u^2 + ...`, which `nu`'s factor of 1.88 absorbs. `1 + 4u = 1 + 2^-51` is exact. |
| NIT | wrong module digest in the record | **Fixed** | The attempt-4 line is marked as `feefbe166`'s. I rebuilt fadc45df0: `ec1d2f66…a58cddfb`, 3,053,802 B, as recorded. The record commit moves no source line. |
| NIT | superseded lines unmarked | **Fixed** | Struck through with pointers (gate 3 formatting: NIT above). |
| NIT | stale host-core doc | **Fixed** | `prepare.rs:372-375` is accurate. Doc-only diff. |
| ROOT-A | amend #1457 | **Done as ruled; premise false** | See m1. |
| ROOT-B | host-core doc | **Done** | |
| ROOT-C | mark superseded lines | **Done** | |
| ROOT-D | digest | **Done** | The final digest is recorded at the batch boundary. |
| ROOT-E | effect-floor citation | **Done** | Marked as a citation update. Not complete: NIT above. |
| ROOT-F | scope | **Respected** | Only the authorized paths changed. The `crates/math` and `crates/host-core` diffs are comments only. No fixture, artifact, script or workflow changed. |
| STREAMS row | #1457 row reference | **Fixed** | `STREAMS.md:285` is row 21. |

## Implementer claims checked

- **The counter.**
  - It is `#[cfg(any(test, feature = "test-support"))]` on the `thread_local!`, on its increment in
    `fixed_input_bound` (`builtins/src/tail.rs:151-152, 186-192`) and on the accessor
    (`builtins/src/lib.rs:5604-5609`).
  - No shipped build enables `builtins/test-support`. Every enabling edge is a dev-dependency or a
    test-support feature. `host-web` and `capi` build without it.
  - The counter is absent from the shipped wasm twin (0 matches for `FIXED_INPUT_BOUNDS` and
    `fixed_input_bounds_computed`; `fixed_input_bound` is present). It is absent from
    `libcapi.so` and `libcapi.a` (`nm`: 0).
  - As a control, I found it present in the test-support test binary.
  - `fixed_input_bound` runs only at preparation (control plane), never in render.
    `check-realtime-policy.sh` is green, and `audit capi` reports 0 allocations, deallocations and
    syscalls.
- **Counts 6 / 0 / 4.**
  - The partition is right: eq0-eq2, eq3, eq4, eq5, eq6 and eq7-eq8 give 6 designs. With eq3 and
    eq4 live, 4 designs remain.
  - The test reads the counter after `validate_for_session`, so it also proves that the seal check
    computes nothing.
  - The counter is thread-local and test threads are separate. The exact 6 also shows that
    preparation stays on the calling thread.
- **Mutants.**
  - M1 (attempt 4's behaviour): 6 against 0.
  - M2 (per strip): 9 against 6; `tail_contract` stays green, so this is the only catch.
  - M3 (E2): red.
  - M4 (C3): red.
  - All as claimed (`evidence/mutations-summary.txt`, `evidence/mut/`).
- **`STEP_UP` and `nu`.** Verified above. The argument is rigorous. Its precondition is that every
  summand reaches the result through at most two roundings before the product with `1 + 4u`. That
  holds at all four sites, and the one exception is stated.
- **Boot timing.** The 64-track documents are within noise, as claimed. The 9-track residual is the
  live bound (NIT). The claim about the browser in general is false for audio-only boots (m1).

## Test-value sentences (new and rewritten tests)

- builtins-compiler `design_bounds_are_computed_only_for_strips_without_a_live_input_lane` (new):
  - A preparation that computes a live strip's design bound is red, and nothing else catches it:
    M1, attempt 4's browser waste, counts 6 where 0 is required, and every other builtins-compiler
    test is green.
  - A preparation that bounds each strip rather than each distinct design is red, and nothing else
    catches it: M2 counts 9 against 6, and `tail_contract` is green.
  - It also catches C3 (5 against 6).
- builtins-compiler `live_input_lane_reports_the_live_bound_and_plain_input_its_own` (rewritten): a
  seal check that takes a live strip's bound from its sealed entry, not the sealed live bound,
  accepts a payload and a seal forged alike (M3, E2). The test is red on that defect, and the other
  54 lib tests are green.
- `each_strip_is_bounded_by_its_own_design_when_designs_are_shared` (rewritten):
  - A design key that drops the right channel hands the right-only strip the first strip's bound
    (M4, C3, red).
  - It is blind to the mirror defect, a key that drops the left channel (M5, green everywhere; m2).

## Gates run (all green)

- **Release `tail_contract --include-ignored`.** 9 passed, 7.2 s. With `--nocapture`, the 300
  figure lines are identical to attempt 4's log. Examples: 1(a) HPF 10 into LPF 22049.48, +24 dB:
  416,117. Live 44.1 kHz: 904,785 / 1,081,764. Live 48 kHz: 899,524 / 1,075,575 /
  `peak_plus_24_dbfs` 1,257,840 / `any_sanitized_input` 2,569,016. 1(a): 112 rows.
- **Lint and docs.** `cargo fmt --all -- --check`. `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings` (CI's flags, including `--all-features`).
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- **Debug suites.** `cargo test --locked -p builtins-compiler --no-run`. `test-debug-a` with the
  exact `qualification.yml` command: 1,442 passed, 0 failed, 10 ignored; the new test runs here.
  `test-debug-b`: 888 passed, 0 failed, 25 ignored. `conformance_fixtures --check`.
- **Policy scripts.** `check-workspace-policy.sh`, `check-realtime-policy.sh`,
  `check-lane-policy.sh`, `check-builtins-policy.sh`, `check-dsp-research.sh`,
  `check-ci-path-routing.py`, `test-ci-path-routing.py`, `check-test-support-ci.py`.
- **Release native.**
  - `cargo build --release -p audit -p bench -p capi -p session-validator`.
  - `audit capi`: 0 allocations, 0 deallocations, 0 syscalls, 0 violations, `pcm_digest`
    `cb10fbface44a3a4`. This is identical to attempt 4.
  - `check-effect-contract.sh target/release/bench`.
  - `cargo test --release -p audit -p bench -p console-workload`: 113 passed, 0 failed.
  - `check-capi-abi.sh` and `--self-test`. `check-scalar-oracle-absent.py --native`.
  - `graph_fixture --check`. `check-graph-determinism.sh` (100/100).
  - `check-builtins-fixtures.sh` (50 files). `check-console-fixtures.sh`.
- **Cross targets.** `check-cross-targets.sh`: PASS. builtins `memset_pattern16` 5 calls, the
  expected #1018 row.
- **Worklet chain.**
  - `build-web-audioworklet.sh --named-twin` into empty directories. The module is `ec1d2f66…`,
    3,053,802 B.
  - `strip-wasm-names.py --self-test` and `check`.
  - `check-web-audioworklet.sh --without-metadata-regeneration`.
  - `check-browser-expected-resources.py --artifacts`.
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh`.
- **Wasm gates.** `run-wasm-gates.sh --without-v8-spill --without-native`: 143 cases, 252
  comparisons, 0 mismatches.
- **Extra.**
  - Boot timing: live-control and audio-only, pre / head / variant (table above).
  - Pre-#1329 module rebuilt: `10a3c816…`, as recorded.
  - Five mutations plus the left-only probe.
  - The `STEP_UP` and `nu` arithmetic, in 50-digit Decimal.
- **Not run.** AArch64 execution (CI only). The `artifact-identity` CI comparison.

## Items for ROOT

- **ROOT-1 (m1, before the #1457 GitHub sync).**
  - Restate ruling A and #1457's "Where preparation runs" with the measured audio-only figures
    above.
  - The facts to state: a live-control browser boot bounds no design. An audio-only browser boot
    (the SDK default) bounds every distinct design on the AudioWorklet thread, +15.5 ms (+81 %) per
    64-track boot today, and on a Worker after #1332. The C ABI bounds every distinct design on its
    control thread.
  - #1457's D1 budget then names both hosts.
  - Correct the comment at `crates/builtins-compiler/src/lib.rs:3573` in the batch follow-ups.
- **ROOT-2 (m2).** Fold in the left-only strip (`evidence/left-only-probe.diff`), or narrow the
  test-value line to the right channel, in the batch follow-ups.
- **ROOT-3 (NIT).** Record in #1457 that the live bound costs about 0.07 ms per live-control browser
  boot, and that it depends only on the rate (four launch rates). A cache or a per-rate table,
  checked by a test, removes it.
- **ROOT-4.** The batch boundary records the final module digest (ruling D) and synchronizes the
  bodies of #1329 and #1457. The #1457 GitHub body does not yet have the Amendment 5 paragraph.
