# Verification of the dead-code and scope audit

Date: 2026-09-28. Verifier: Sol (adversarial). Subject: `DEAD-CODE-AUDIT.md` and the drafts in
`issues/`, as committed in `9e05c068` on top of `main` at `a9414c0c`. Every fact below was
reproduced on that tree by a command, a build or a grep; where a fact was taken from a parallel
verification pass and spot-checked rather than re-run, it says so. No product code, test, script or
workflow was changed, nothing was pushed, and no GitHub issue was created or edited (reads only).
Scratch copies and build directories were deleted afterwards.

Each draft now ends with an **Amendments** section that applies the findings below. One new draft
was added: `issues/00b-repair-the-operator-script-roots.md`.

## 0. Scope correction received during verification

After the audit was written, the owner corrected the product scope:

> "We're still going to have the engine cross-platform especially on mobile apps. The
> production/mixer UI will be primarily in a browser, but fans still need to use the engine to open
> sessions and that will happen on web app or mobile app."

So there are two uses. **Producers mix in the browser** (wasm `simd128` AudioWorklet). **Fans open
and play sessions in the web app and in native iOS and Android apps.** Mobile embedding is live
scope. The audit judged every native surface against a browser-only product, so its section 7
recommendations for R1, R2 and R3 are reversed below, R4 and R8 change, and draft 02 moves to the
"needs a ruling" set. `AGENTS.md`'s mission line ("Build for native/cloud embedding, iOS, Android,
and browser WebAssembly") already names mobile; the audit's framing, not AGENTS.md, was out of date.

## 1. Which claims hold

| audit claim | verdict |
|---|---|
| Tests behind `test-support` never run in CI, and one fails on `main` | **Holds.** No workflow enables `host-web`, `host-core`, `effect-compiler` or `parametric-eq` `test-support` for a `cargo test`, and no manifest forwards it. The failure reproduces deterministically, alone as well. But it is not a regression (F1), and fewer tests are affected than "588" suggests. |
| Six of seven operator shell scripts cannot run (wrong root) since 2026-09-01 | **Holds.** All six fail exactly as described (F2). No single draft repairs them. |
| Draft 00 fixes the first | **Partly.** Its CI change is right; its step 1 was unresolved (now bisected), and it has no recurrence guard. |
| 129 items proved unused | **Mostly holds.** 45 of 47 sampled items deleted outright pass every build, `aarch64` included; 2 are used by a `cfg(test)` module (F17). |
| 8,244 endpoint lines nothing calls | **The line count and "no caller" hold; "dead whatever you rule" does not** under mobile scope (F8). |
| Used-up benchmark machinery | **Holds for "cannot run again".** Removing it breaks a required lint ratchet the drafts do not mention (F14), and two "unreached" test files do run in CI (F19). |
| 62 MB of `artifacts/` that nothing reads | **Holds for code, tests, scripts and CI** (docs gates pass on a pruned copy). Open GitHub issues cite some folders (F15). |
| "About 83,000 lines of wider-scope features ... none of it touches browser output" | **Browser half holds; the scope half does not.** The C ABI, the protocol, `control-provider`, block-boundary plan replacement and the AArch64 arms (about 41,000 of those lines: capi 9,802, protocol 29,957, `control_provider.rs` 874) are the mobile playback surface (F3). |
| Keep native x86 as tooling | **Holds**, and native AArch64 is now also a product target. |

## 2. Findings, severity-ranked

### F3 (critical): R1, R2 and R3 would remove the mobile playback surface

- **Facts.**
  - `crates/capi` builds as `rlib`, `staticlib` and `cdylib` (`Cargo.toml:11`). Its header
    (`include/miso_engine_v1.h:213-249`) exposes session compile from JSON, planar PCM submission
    and seek, render at an absolute sample, command submission and event readout. That is what a
    fan's app needs to open and play a session.
  - The C ABI's command path *is* the protocol: `miso_engine_v1_submit_command` decodes a binary
    frame through the protocol controller (`capi/src/runtime/control.rs:675-690`). capi names
    `protocol::` 55 times outside its tests.
  - capi alone uses `host-core`'s `control-provider`, the engine's block-boundary plan replacement
    (`control.rs:757`, `:803-810`; `runtime/plan.rs:201`) and `lane::fpenv::in_canonical_fp_environment`
    (its render-entry guard, `ffi.rs:775`, with an AArch64 branch).
  - `lane::Backend::current()` gives `Simd4` (NEON) on `aarch64`; `lane/src/fpenv.rs` holds the
    FPCR flush-to-zero code.
  - Every product crate, capi and protocol included, passes `cargo check --all-targets
    --all-features` for `aarch64-apple-ios` and `aarch64-linux-android` (Rust 1.98.1; compile
    only, see section 6).
- **Failure scenario.** R2 then R3 land as recommended ("remove unless native embedding is
  planned"; "remove together"). The next mobile task finds no C ABI, no command path and no plan
  replacement, and R1 has added a `compile_error!` that refuses every target except x86-64-v3 and
  wasm, so the engine no longer builds for iOS or Android at all.
- **Correction.** R1: keep the AArch64 arms; only the two stub hosts are unneeded. R2: keep capi;
  native-pcm-runner is a separate optional call. R3: keep the protocol; only the WebSocket sidecar
  (no code) and 20 dead protocol items go. Drafts R1, R2, R3, 01, 02, 03 and 06 amended.

### F1 (high): the never-run failing test is an intended change the gap hid, not a regression

- **Reproduced.** `cargo test -p host-web -p host-core -p effect-compiler -p parametric-eq --features <the four test-support>`
  on `a9414c0c`: one failure, `tests::acknowledged_pair_render_records_the_same_live_dispatch`
  (`tests.rs:3220`, `process_calls` 2 vs 1). It also fails when run alone, so it is not a
  thread-local race.
- **Bisected** (first-parent, then into the merged batch): passes at `86be6792`, fails at
  `608f0379` (#916, 2026-09-25, "write the master straight into the host planes"), which reached
  `main` through `97435208` (#922). #916's own scope amendment `9c762d7c` explains it: the Output
  became dedicated storage, so "track 0 pairs", and it updated the builtins-compiler harness counts.
  The host-web test sits behind `host-web/test-support`, so nobody saw it.
- **Witness on `main`:** `process_calls 2, fused_calls 2, fallback_calls 0, process_members 9`,
  records drained 1 and 1. With only those three counts changed (2, 9, 2) the test passes,
  output values included (scratch copy).
- **Scope of the gap.** `-- --list` over the four crates gives 597 names with the features and 593
  without. The four never-run tests are that one plus
  `prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`,
  `prepared_mixed_eq_builtin_fader_commits_and_refuses_atomically` and host-core's
  `dormant_controlled_spectrum_does_no_capture_work_on_render`. Gated assertion blocks inside tests
  that do run are also skipped (host-web `tests.rs:9318-9324`, `:9467-9486`; host-core
  `observation_demand.rs:1511`, `:1970`, `:2040`; parametric-eq `tests/bank.rs:913-923`,
  `:1344-1354`). The other 585 tests already run in CI.
- **Failure scenario if draft 00 were followed as written.** The implementer bisects, then must
  decide "regression or intended" without the #916 context. If they "fix the code" back to one
  call, they undo #916's dedicated-Output pairing and move console digests.
- **Draft 00 amended:** test-only fix citing `608f0379`/`9c762d7c`; the `process_members` message
  must be re-worded, since the witness now aggregates the bank call and the tail call; add a
  recurrence guard that fails when any `test-support` feature is not enabled by some CI test step.

### F4 (high): the mobile target is entirely unqualified

- No CI job builds, lints or tests any AArch64 target. `scripts/check-cross-targets.sh:4-5`
  records that the Android and iOS rows were removed under #378 ("native AArch64 unsupported, no
  claim", closed); #023 (iOS/Android embedding examples) was closed 2026-08-22, and spec 001 still says "Mobile support is browser-based; native iOS and Android embedding targets are deferred" ([#1's spec, line 43](https://github.com/misofm/engine/blob/80c4119b9e6814cb450e87568243d6df9b6be7bc/.github/ISSUE_SPECS/001-bootstrap-rust-workspace-and-target-matrix.md#L43)).
- The two register defects in `docs/TARGET_MATRIX.md:95-101` are still present:
  - `parametric-eq`'s iOS release assembly holds **151 `memset_pattern16` calls** (reproduced), a
    realtime-rule breach inside render on Apple targets;
  - the compressor's Android release assembly holds `fmaxnm`/`fminnm` folds (LANE-3, #366, closed
    as deferred; count from the parallel pass, not re-run), so mobile output will not match the x86
    and browser oracles bit for bit.
- Seven test-only warnings on aarch64 (`host-core/tests/fp_environment.rs:25-102`,
  `lane/tests/fp_env.rs:14`) would fail `-D warnings` on an aarch64 leg.
- 32-bit `armeabi-v7a` Android falls silently to `Backend::Scalar` (`lane/src/backend.rs:60-68`;
  only x86 has a `compile_error!`, `lane/src/lib.rs:72-80`); x86-64 Android emulators and iOS
  simulators inherit the AVX2+FMA pin (`.cargo/config.toml`) and refuse to start on CPUs without it.
- Nothing measures NEON performance.
- **Failure scenario.** A mobile app ships on the current tree: its render thread calls libc inside
  the EQ kernel on iOS, and a fan's mix differs in the last bits from the producer's browser mix,
  with no test anywhere that would have said so.
- **Recommendation (owner ruling):** "native AArch64 (iOS, Android arm64-v8a) is a product target",
  then separate issues for an aarch64 CI leg, LANE-3 and the Darwin memset, the Android ABI list, and
  emulator/simulator builds. Recorded in the R1 amendment.

### F8 (high): draft 02's endpoints are unwired, not dead

- Compile facts hold: deleting both endpoint modules and their tests passes native, wasm, iOS and
  Android builds, `-p capi` included, with no new warning.
- But the open #140 spec requires admitted automation to reach PCM "through the actual C ABI command
  and render calls" ([#140's spec, line 75](https://github.com/misofm/engine/blob/80c4119b9e6814cb450e87568243d6df9b6be7bc/.github/ISSUE_SPECS/140-automation-span-feed.md#L75)), and says the only production consumer of
  that queue today cancels it (`:13`). These endpoints (#528-#608) and protocol's `delivery.rs` and
  `controller_delivery.rs` are the partial implementation. So today a fan's live fader or mute
  change through the C ABI reaches PCM only by structural plan replacement, which resets source
  rings at the boundary (`capi/src/runtime/control.rs:46-53`).
- **Failure scenario.** Draft 02 lands as "no ruling needed"; mobile live control is scheduled next
  and #140's successor re-derives about 8,200 lines of qualified endpoint code that nothing still
  exercises.
- **Draft 02 amended:** hold for an owner ruling decided with #140.

### F14 (high): drafts 04b and 05 break the required lint job

- `scripts/check-bench-policy.sh:184-185` keeps a `timed_subjects` ratchet that "never shrinks":
  `tools/bench/src/rack.rs`, `tools/audit/src/fp_env.rs`, `tools/wasm-console/src/main.rs`. On a
  scratch copy with 04b's subjects removed, lint fails with
  `converted subject is missing: tools/bench/src/rack.rs`; the base passes.
- `scripts/test-bench-policy.sh` mutates rack.rs, graph.rs, builtins.rs and effect_interchange.rs
  (`:113`, `:224-266`, `:356-394`); `test-realtime-policy.sh:281`,
  `test-effect-runtime-policy.sh:93-99` and `test-env-vocabulary.sh:39` write into rack.rs.
- R9 hits the same ratchet (`wasm-console`, and `audit fp-env` if the nightly benchmarks go).
- **Failure scenario.** The 04a-05 batch reaches its single push; `qualification` goes red in lint;
  the batch needs a second CI round, which CI-conscious batch mode exists to avoid.
- Drafts 04b, 05 and R9 amended.

### F6 (high): R8's "no shipped target produces Scalar" is an Android-ABI question

- With mobile in scope, an `armeabi-v7a` build (`target_arch = "arm"`) runs exactly the whole-plan
  Scalar path R8 deletes.
- **Failure scenario.** The owner ships 32-bit Android for older devices after R8 landed; that build
  no longer compiles, or has no execution mode.
- **R8 amended:** defer until the owner rules on Android ABIs; if only arm64-v8a ships, add a 32-bit
  ARM `compile_error!` in the same change.

### F2 (medium): the operator scripts have no single owner for their repair

- Reproduced from a clean tree with arguments that stop before any workload:
  - `preflight-rack-benchmark.sh` → `scripts/test-rack-benchmark.sh: No such file or directory`, exit 127;
  - `preflight-wasm-console-benchmark.sh --mono3` → `scripts/check-console-benchmark-fixture.sh: No such file`, exit 1;
  - `run-wasm-console-benchmark.sh --mono3` and `run-wasm-kernel-timing.sh` → source `scripts/scripts/check-bench-preconditions.sh`, exit 1;
  - `prepare-builtins-listening.sh INBOX OUT` → `scripts/check-builtins-listening.sh: No such file`, exit 1;
  - `seal-web-audioworklet-browser-correctness.sh SEAL` → `scripts/scripts/build-web-audioworklet.sh: No such file`, exit 127.
- The cause is the move in `f0509c3f` (#319, 2026-09-01); only `preflight-console-benchmark.sh` was
  later fixed.
- **Failure scenario.** R9 is ruled "keep" or waits; nothing repairs the wasm console scripts, and
  the next listening test (AGENTS.md requires blinded listening evidence) cannot be prepared until
  04c lands.
- **New draft `00b`:** repair every surviving operator root and add a static check with a mutation
  test.

### F7 (medium): R8's replacement oracle is strictly weaker

- Simd4-vs-Simd8 compares two banked plans. It cannot see a banking bug that is identical at both
  widths: a cohort key that ignores a per-track field, a non-identity absent slot, state or a
  sidechain bound to the wrong member, a bank-path PDC error.
- `dsp-reference` and lane-kernel comparisons are per effect and cannot see plan-level regrouping.
- **Failure scenario.** A cohort-key omission puts a Draft-quality track in a High cohort at both
  widths; both render the same wrong output; the new oracle passes; AGENTS.md's "a placement change
  must not move a rendered bit" is unguarded.
- Class-A proofs do **not** depend on the Scalar path: they rest on console `output_sha256` equality
  (`scripts/test-console-benchmark.sh:718`, `:823`) and on the one-lane scalar leg (W=1, #976),
  which R8 keeps.
- The atomics check is not a pure duplicate: `check-wasm-realtime-atomics.sh:36-41` inspects whole
  non-LTO rlibs before link-time dead-code removal. Re-target it rather than delete it.
- **The owner must choose:** keep an unbanked per-node plan as a stated test-only oracle exception,
  or accept the weaker guarantee in writing.

### F5 (medium): R4's native-only hooks are compiled into mobile builds

- The hooks are `cfg(not(target_arch = "wasm32"))`, so every AArch64 capi build carries the unused
  `retirement_worker` fields (`source/src/lib.rs:1636`, `:1707`), and capi's test mirrors them
  (`resource_lifecycle.rs:803-826`).
- The capi layout re-pin is therefore mandatory, not conditional on R2.
- The main recommendation holds for mobile: capi never names `native_source` or `native_wave`, and
  platform decoders (AVAudioFile, MediaCodec) submit planar PCM, as AGENTS.md says for mobile hosts.
- Whether a native desktop or cloud renderer is still wanted is an open owner question.

### F9 (medium): R5 is larger than "descriptor metadata, about 50 lines"

- `builtins::validate_builtin_filter_cutoff` (`builtins/src/lib.rs:343-370`, in the shipped module)
  also admits extended rates.
- The conformance mock effect declares four extended quality rows (`conformance/src/effect.rs:99-107`)
  and would fail a tightened `validate_descriptor`.
- 12 files outside `engine/src/lib.rs` name the extended set.
- **Failure scenario.** The implementer edits only the listed files and the `conformance` and
  `builtins` test binaries go red.
- No SDK, host-web, parameter-metadata or ABI-layout surface exposes extended rates.

### F12 (medium): R9 misses a ruling that depends on the wasmtime console

- The wasm floor rule (`docs/rulings/effect-floor-accounting.md:815-840`) derives its per-row
  residual from `artifacts/compressor-round1/wasm-console-benchmark.accepted.jsonl`, and
  `compressor-identity-mask-hoist-wasm-null.md:55-65` cites the same record.
- `isolated_cycles_per_lane_sample` is native-only (`tools/bench/src/floor.rs:315`), as the audit
  says.
- R9 must add history notes to both rulings, and must also edit the lint ratchet (F14).

### F13 (medium): BRIEFS, handoffs and closed-spec citations are load-bearing

- `.github/ISSUE_SPECS/BRIEFS/` is not read only by an allowlist line:
  - `BRIEFS/019` is the frozen source of the soft-clip graph and the half-band coefficients
    (`soft-clip/src/lib.rs:3`, `kernel.rs:3`, `tests/polyphase_identity.rs:5`, `:20`;
    `lane/src/kernels/halfband.rs:3`, `:58`);
  - `BRIEFS/016` is cited by `dsp-reference/src/true_peak_limiter.rs:57`;
  - `BRIEFS/013` is the compressor authority in `docs/README.md:19`.
- Handoff files draft 08 calls "history" are cited by open issues:
  - `effects-2026-09-27/LIMITER-DIAGNOSIS.md` and its `.patch` by #988, #989, #991 and #992;
  - `plumbing-floor-2026-09-26/DIAGNOSIS-2.md` by #938;
  - `builtins-less-removal-2026-09-27/VERIFY.md` by #965.
- Other load-bearing notes:
  - `docs/issue880-mb1.md` is the provenance of committed coefficients (`math/src/lane_math.rs:42`, `:85`);
  - `docs/issue880-mb2.md` records the tolerance behind `transient-shaper/tests/oracle.rs:30`;
  - `docs/research/legacy-v2old/02-…` is cited by `lane/src/fpenv.rs:30`.
- R10's gate 2 cannot pass as written: five code comments and three open specs (1010, 1008, 026)
  cite closed specs by path.
- **Failure scenario.** Draft 08 step 3 lands; a reviewer checking the half-band coefficients
  against their "character for character" source finds no in-tree source.
- Drafts 08 and R10 amended.

### F16 (medium): draft 04b deletes the only check of a live audit constant

- The "Builtins benchmark real-tree manifest consumers" step (`qualification.yml:355-356`) is the
  only thing tying `tools/audit/src/builtins_graph.rs:48` `ACCEPTED_MANIFEST_SHA256` to
  `fixtures/builtins/v1/MANIFEST.tsv`. The audit only prints it (`:297`).
- **Failure scenario.** The manifest changes and the constant silently goes stale.
- Keep a one-consumer check. Draft 04b amended.

### F15 (low): draft 07 misses GitHub-only citations

- Open #560 has no local spec, and its body points at `artifacts/issue470-*` (including the loose
  `.md`), `issue555-*`, `issue557-qualification`, `issue558-*`, `issue563-*` and `issue570-*` as
  "durable raw evidence".
- Draft 07's grep covers local files only.
- Editing `crates/lane` comments also turns on the `math_closure` exhaustive sweeps.

### F17 (low): draft 01's table has two live items and misses a lockfile

- `parametric-eq` `lib.rs:239` and `:244` are called from `#[cfg(test)] mod ramping_elision`
  (`:7601`, `:7604`, `:7677`); deleting them gives E0425. Draft step 3 would catch this at compile
  time.
- Removing rack-compiler's `engine` dependency rewrites `Cargo.lock`; commit it or `--locked`
  fails.
- The no-op `allow(dead_code)` count is at most 45, not 47, and the keep-list misses
  `native-pcm-runner/src/lib.rs:733`.

### F18 (low): draft 06 conflicts with the corrected scope and an open issue

- "Everything else unsupported" would write AArch64 out of `TARGET_MATRIX.md`.
- The open #26 spec (`026-…md:88-92`) cites `fixtures/capi-qualification/v1`. The fixture is still
  unread by code, so re-point the spec text in the same change.

### F19 (low): wording in audit section 4

- `compressor/tests/bench_ramp.rs` has two non-ignored tests (`:375`, `:541`) that `test-debug-b`
  runs.
- The four `tools/bench/src/input_symmetry.rs` tests run in audit-native.
- "Reached by no workflow" is true of the scripts only.

### F20 (low): R6, R7 context

- R6's optional grammar step must also edit `sdk/src/internal/session-json.ts:91`, which writes
  `cid`.
- Mobile playback needs no persisted effect state; sessions carry none.
- R7's kept re-measurement path is already dead: `artifacts/issue183/` exists and the root is
  wrong. Spec #976 used `--cfg miso_wasm_simd8` as a test leg on 2026-09-28.

### F21 (low): a method trap for base-versus-change comparisons

- `git archive` extracts files with the commit's mtime. Reusing a `CARGO_TARGET_DIR` across two
  extracted trees made cargo treat the newer tree's older-dated sources as fresh. In this
  verification that produced stale builds that failed to compile.
- The same trap could produce a false "digests identical".
- `build-web-audioworklet.sh` is immune, because it uses a fresh `mktemp` target. Console-digest
  and `-- --list` comparisons are not.
- Build base and change from separate `git worktree` checkouts, or give each tree its own target
  directory.

## 3. Corrected facts per ruling, for browser mixing and mobile playback

| ruling | browser (producer mixing) | mobile playback (fans) | revised recommendation |
|---|---|---|---|
| R1 AArch64, host shells, target-smoke | not used | **AArch64 arms live** (NEON `Simd4`, FPCR FTZ). The stub hosts are unused by any app. No aarch64 CI; two known defects; armv7 falls to Scalar; x86 emulators refused | Keep the arms. Rule "AArch64 is a product target" and file a CI leg, LANE-3, Darwin memset, Android ABI list. Removing the two stubs is optional |
| R2 C ABI, native PCM runner | not used | **capi is the mobile surface** (`staticlib`/`cdylib`, playback API complete, compiles for iOS and Android, never linked or run); `audit capi` is its only realtime audit | Retire the draft. Optionally remove `native-pcm-runner` alone (desktop WAV oracle, not a mobile deliverable) |
| R3 control protocol | not used (not in host-web's closure on wasm or aarch64) | **capi's command and event path**; live automation delivery is unfinished (#140) | Retire the draft. Delete the 20 dead items; rule the WebSocket sidecar out (AGENTS.md text only, closes #25) |
| R4 native WAV decode workers | not compiled | not used by capi; but the hooks' fields are compiled into mobile builds | Remove Part A with a mandatory capi layout re-pin; Part B with the runner decision; ask whether a desktop or cloud renderer is wanted |
| R5 extended rates | refused by `session::validate`; predicate in the module | same | Remove, with the larger file list (F9) |
| R6 effect packages, state | not executed | not needed; iOS forbids JIT for third-party Wasm | Remove; SDK edit if `cid` goes |
| R7 `miso_wasm_simd8` | measurement hook, re-measure path already dead | n/a | Remove; note #976's lost ad-hoc leg |
| R8 whole-plan Scalar | not executed at `Simd4` | **production path on armv7 Android** if that ABI ships | Defer; rule on Android ABIs and on the oracle exception first |
| R9 wasmtime console | Cranelift, not V8; broken since 2026-09-01 | measures nothing mobile | Remove, with ruling history notes and the lint ratchet edit |
| R10 closed specs | n/a | n/a | Possible, but keep cited BRIEFS and re-point path citations first |

**New ruling questions the correction creates:**

1. Is native AArch64 a product target now, and do #378 and #023 reopen?
2. Which Android ABIs ship?
3. Does mobile live control go through protocol automation (#140) or structural replacement only?
4. Is any native desktop or cloud renderer still wanted? That decides R4 Part B and the runner.

## 4. The drafts: bounded, and gated?

All drafts are small and self-contained in shape. The gate conventions (native and wasm builds,
console digests, same-machine artifact comparison, CI routing, `-- --list` diffs) are sound. The
gaps, now written into each draft's Amendments:

- **Every code draft that touches a crate in capi's closure** (01, 03, R4, R5 and the reduced R1)
  must add an aarch64 `cargo check` for iOS and Android, since CI has none.
- **03:** `enter_block` is capi's plan-swap path, so add `cargo test -p engine -p capi` and the capi
  runtime audit.
- **04b, 05, R9:** the bench-policy ratchet and mutation tests (F14).
- **04b:** also the manifest constant check (F16).
- **07, 08:** open-issue bodies and source comments in their link gates.
- **R10:** gate 2 as written cannot pass.

## 5. Safe order of work

1. **00 and 00b** (no ruling). Fix the one test (test-only, per F1). Enable the four
   `test-support` features in `test-debug-a`/`test-debug-b` with a recurrence guard. Repair the
   operator-script roots.
2. **01** (with F17) **and 03** (with capi and aarch64 gates).
3. **04a, 04b, 04c, 05** as one CI-conscious batch, with the lint ratchet edited in the same batch
   and 04b's manifest check kept.
4. **06** (target table deferred to the AArch64 ruling), then **07 and 08** as one batch, with
   their keep-lists for cited files.
5. **Owner rulings.**
   - First the new mobile questions in section 3: AArch64 target, Android ABIs, #140 live-control
     path, desktop/cloud renderer. These decide 02, R1-reduced, R4 Part B, R8, and the fate of
     native-pcm-runner.
   - Then the independent rulings: R5, R6, R7, R9 and R10 (after 04b).
   - R2 and R3 are retired; their small remnants (the runner; the 20 dead protocol items and the
     sidecar text) are separate drafts.
6. **Before any mobile feature work:** the aarch64 CI leg and the two register defects (F4).

## 6. What was not verified, and why

- **Nothing was linked or run on AArch64.** There is no NDK, Xcode, simulator or aarch64 runner
  here. The iOS and Android facts are `cargo check`/clippy and emitted assembly under Rust 1.98.1,
  because the pinned 1.97.1 has no aarch64 standard library. The 1.98.1 aarch64 standard libraries
  were installed for this verification and removed afterwards.
- **The armv7 Scalar fallback** is read from source; the `armv7-linux-androideabi` target was not
  installed.
- **The compressor `fmaxnm`/`fminnm` count and the Android clippy run** come from a parallel pass.
  Its logs were inspected; only the iOS `memset_pattern16` count was re-run.
- **The shipped artifact was not rebuilt.** No draft change was applied to the tree; the R6
  module-identity claim is plausible from the call graph but unproved here.
- **Not compile-proved:** draft 03 Part 2 (render input) and the R3/R6 whole-crate deletions.
  Their compile facts are the audit's.
- **Not checked:** the audit's CI-minute figures.

## 7. Commands (representative)

- Test gap: `cargo test --locked -p host-web -p host-core -p effect-compiler -p parametric-eq --features host-web/test-support,host-core/test-support,effect-compiler/test-support,parametric-eq/test-support --no-fail-fast`
  and the same with `-- --list`, with and without the features.
- Bisect: `git archive <sha> -- . ':(exclude)artifacts'` into a fresh directory, each with its own
  `CARGO_TARGET_DIR` (see F21). Command: `cargo test -p host-web --features host-web/test-support --lib -- --exact tests::acknowledged_pair_render_records_the_same_live_dispatch`.
  Commits tested: `024ad674`, `735197b1`, `f8d52f17`, `4cdc878f`, `55bd3ff9`, `f2a8ef4f`,
  `1ee6cede` (all pass); `97435208`, `fd83026e`, `252622b6` (fail); `86be6792` (pass);
  `47a11617`, `9c762d7c` (fail). `608f0379` is the only code change between its parent
  `e178a379` (an ancestor of the passing `86be6792`) and the failing `47a11617`: `10de67c9`
  changes comments only, and `47a11617` docs only.
- Operator scripts: each run from the clean worktree, with arguments that stop before any
  workload (section F2).
- Dead items: 47 items deleted in a scratch copy, then:
  - `cargo check --workspace --all-targets --all-features`;
  - `RUSTFLAGS='-C target-feature=+simd128' cargo check --target wasm32-unknown-unknown -p host-web -p host-core`;
  - the scalar-wasm package list;
  - `cargo check --manifest-path fuzz/Cargo.toml --bins`;
  - `cargo +1.98.1 check --workspace --lib --all-features --target aarch64-apple-ios` (and
    `aarch64-linux-android`) excluding `native-pcm-runner` and `stem-hasher`;
  - `cargo +1.98.1 check --all-targets --target aarch64-unknown-linux-gnu -p lane -p soft-clip -p graph -p engine -p capi -p host-core`.
- Artifacts: the draft 06-08 deletions applied to a scratch copy, then `check-dsp-research.sh`,
  `test-dsp-research.sh`, `check-builtins-listening.sh`, `check-session-policy.sh`,
  `test-session-policy.sh`, `check-artifact-evidence-leak.sh`, `check-bench-preconditions.sh`,
  `check-step-vocabulary.py`, `check-ci-path-routing.py` and `test-ci-path-routing.py`, all
  passing.
- iOS assembly: `cargo +1.98.1 rustc --release -p parametric-eq --target aarch64-apple-ios -- --emit asm`,
  then `grep -c memset_pattern16` gives 151.
