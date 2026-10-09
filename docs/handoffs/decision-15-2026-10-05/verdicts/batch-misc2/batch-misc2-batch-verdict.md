blockers

# Decision-15 batch-misc2 verdict: `codex/d15-batch-misc2` at 8bd74b1ef

Base: main `1d294c700` (11 commits; slices #1488, #1490, #1491, #1492, #1489).
`git ls-remote origin refs/heads/main` = `1d294c700`. The branch is not on origin yet.

**Summary:**
- Every gate that an x86_64 runner can run passes: **111 steps, 0 non-zero exits**. The gates
  include the three browser legs, the nightly vectorization report, and its red mutations.
- The merge is a fast-forward: `1d294c700` is an ancestor of `8bd74b1ef`, and the synthetic PR
  merge commit has `8bd74b1ef`'s tree (`6064c7bf`).
- No workflow file changed (`git diff 1d294c700 8bd74b1ef -- .github/workflows` is empty). The
  router script and the path-routing policy did not change. `scripts/check-ci-path-routing.py`
  passes. The required status checks on `main` are `["qualification"]` only. **No merge-gating
  check moved.**
- The four documents agree with each other and with the code: the `ffi.rs` header, the
  `render_lock.rs` header, `docs/REALTIME_DEPENDENCY_POLICY.md` "Unsafe-code ownership" and
  `hosts/host-web/MUTATIONS.md`. I checked each one export by export and line by line (see below).
- The cross-slice mutations reproduce on the merged tree. The most important one is MUTATIONS row
  `:563`, which is 6 after #1488 + #1492.
- There is **one blocker, in the records only**: #1492's two edits outside its paths are still
  "pending root's ratification". No code change is needed, and no gate needs to run again.

## Blocker (record only; fix before the push, no re-gate)

### B1. #1492: root has not ratified two edits outside its authorized paths

- **What changed:**
  - `hosts/host-web/MUTATIONS.md:563`: A #1479's phase-2 read-mutation row. The count changed
    from 4 to 6.
  - `hosts/host-web/src/render_lock.rs:5-11`: the module header. It no longer names the two
    selects as exceptions.
- **What the records say:**
  - The #1492 spec, Authorized paths (`.github/ISSUE_SPECS/1492-*.md:84`), lists `render_lock.rs`
    as a "coordinator amendment after attempt 1, pending root's ratification".
  - The coordinator note (`:86-96`) says "Root is asked to ratify both".
  - STREAMS row 101 gives row `:563` to "A #1479 (row `:563` only)". #1492 may only add or edit
    "its own rows".
  - The #1492 verdict (`verdicts/batch-misc2/1492-attempt1.md`, MINOR 1 and NIT 1) recommends
    ratification.
- **Status:** root's follow-up commit `8bd74b1ef` handled the #1491 and #1489 NITs. It did not
  record this ratification. The diff and the record do not agree.
- **The content is correct, so do not revert it.** I re-measured row `:563` on the merged tree:
  `render_locked_staging.rs:249`, "a collection capture's spectrum read allocated", left **6**.
  #1492's fourth select leg did not change the count. The `render_lock.rs` header is true; see the
  coherence section.
- **Fix:** in the #1492 spec, record root's ratification of both edits. Make the matching change
  in STREAMS row 101: #1492 edits row `:563`, or the row moves to #1492. This is the same kind of
  fix as the B1 and B2 of batch-misc.

## Checks asked for

| Check | Result |
|---|---|
| Export | I ran `git archive 8bd74b1ef` into a one-commit repository. Its tree hash is `6064c7bf...`, which equals the commit's tree. `docs/evidence/metering-519-520/timing-failed.log` is tracked but ignored, so I added it with `-f`. |
| Merge onto main | **Fast-forward.** `git merge-base --is-ancestor 1d294c700 8bd74b1ef` is true. I made a synthetic `--no-ff` merge in a `--shared` clone (parents `1d294c700`, `8bd74b1ef`). Its tree is `6064c7bf`. |
| Router (`pull_request` and `push`, `--base 1d294c700 --head 8bd74b1ef`) | `route=full`, `math_closure=true`, `release_inputs=false`, `self_tests=[]`. |
| `check-ci-path-routing.py` and `test-ci-path-routing.py` | Pass. Output: "ci path-routing workflow contract passed" and every mutation set passed. |
| Merge-gating checks | The workflows did not change. `gh api .../branches/main/protection/required_status_checks` (read only) returns contexts `["qualification"]`. `nightly.yml` did not change either: #1490 changes the subject of its report (`tools/audit`), not the workflow. |
| GitHub (read only) | #1488, #1489, #1490, #1491 and #1492 are all **OPEN**, and the titles equal the local H1s. Every body is behind its local spec, because the attempt records are not synced yet. This is expected in batch mode; sync after the push. |

### Path scope (each slice's hunks against its Authorized paths)

| Slice | Files | Result |
|---|---|---|
| #1488 | `ffi.rs` (the 11 exports of D1, the header, the `native_staging` helpers), the `render_lock.rs` header, new `tests/render_locked_source.rs`, its spec | yes |
| #1490 | `tools/audit/src/vectorization.rs` (the import of `UnarmedRest`, `ACTIVE_REGISTRY`, the two probes, `execute_probes`); one allowlist row | yes. NIT: the STREAMS stream-A cell (`:371`) does not name the import, which the spec allows. |
| #1491 | `scripts/test-web-audioworklet.mjs`: all 12 hunks are inside `testQualificationBoot` (`:2500-2966`); new MUTATIONS rows | yes |
| #1492 | `host-core` `spectrum.rs`, `lib.rs`, `tests/spectrum.rs`; `host-web` `lib.rs`, `tests.rs`, `ffi.rs` (the select path and the header), `render_locked_staging.rs`; MUTATIONS | yes, except the **two edits in B1** |
| #1489 | Policy doc: every hunk is inside "Unsafe-code ownership" (old `:45-137`). `softfma.rs`: comment lines only (no line that is not a comment changed). `check-realtime-policy.sh`: header comment lines `:20-23` (root's Amendment) | yes |
| root `8bd74b1ef` | the #1489 and #1491 specs, `softfma.rs` comments, the policy doc rewrap, 7 verdict copies | yes |

## Coherence across slices (ffi.rs, render_lock.rs, policy doc, MUTATIONS.md, code)

- **The wrapped set, from code.** I parsed every `miso_engine_web_v1_*` export body in `ffi.rs`.
  **42** call `render_locked(`. The policy doc's set 2 is "set 1 minus `dispose` and
  `render_allocation_count`, plus the 11 boot-only staging accessors". That is 42 names, and they
  are exactly the code's 42.
- **The post-boot calls, from the worklets.** The engine worklet calls 33 exports after
  `initialize`/`bindLiveControls` (lines > 861), and the PCM-feed worklet calls 2. Together they
  are exactly the doc's set 1 (33). The post-boot calls that are not wrapped are exactly
  `{dispose, render_allocation_count}`. That is the "two named exceptions" of the `ffi.rs` header
  (`:35-39`), of `render_lock.rs:7-8` and of the policy doc (`:213-220`).
- **The selects.** `spectrum_select` and `spectrum_stream_select` are wrapped (`ffi.rs:2974`,
  `:2990`). The `ffi.rs` header lists them in the set (issue #1492). No document still calls them
  exceptions. #1488's D2 text that names four exceptions is historical; its Amendment says that
  #1492 wraps the selects.
- **`spectrum_selection_epoch`:** it is not wrapped, and neither worklet calls it. The doc says
  the same (`:219`).
- **Line citations in the policy doc** match the code at `8bd74b1ef`, including the files that
  #1488 and #1492 edited after #1489 was written:
  - `ffi.rs`: `:878` (`response_header_bytes`), `:894` (`copy_live_record`), `:1805`
    (`read_live_record`), `:3929`/`:3936`/`:3939` (`source_submit`), `:3982` (`source_seek`),
    `:4881` (`live_response_ffi_tests`), `:617-626` (`reserve_stagings`), `:14-39` (header).
  - `render_lock.rs`: `:5-11` and `:88-91`.
  - The feed worklet: `:306-328`, `:386`, `:445`, `:539`.
  - `run.mjs:266-270`.
  - Root `Cargo.toml:90`/`:91`/`:102`.
  - `fpenv.rs`: `:141`, `:148`, `:166`, `:181`, `:322-329`.
  - `spsc.rs`: `:295-459`, `:381`, `:449`.
  - `disjoint.rs`: `:106-347`, `:208`, `:303`, `:317-345`, `:334`.
  - `capi/src/ffi.rs:807`.
  - `allocation_tracker.rs:802-809`.
- **The allowlist.** The doc lists the 19 entries of the `unsafe source exclusions` line of
  `scripts/check-realtime-policy.sh`, one per path, and adds no entry. `realtime policy: ok (89
  marked regions in 25 files)`.
- **The `softfma.rs` quotes** in the doc are verbatim in the `write_mxcsr` `SAFETY` comment
  (`softfma.rs:105-119`) at `8bd74b1ef`. The MXCSR helpers are called by `fpenv.rs`,
  `lane/tests/fp_env.rs` and `capi/src/runtime/tests.rs` only, as the doc says.
- **MUTATIONS.md rows re-run on the merged tree:**

  | Row | Result |
  |---|---|
  | `:563` | red, **6**, at `render_locked_staging.rs:249` |
  | #1492 select legs: `select` clones its entry | red, "the first select allocated" **2** |
  | #1492 select legs: the bridge builds an owned target | red, **2** |
  | Control: first mutation kept, both selects unwrapped | **green** |
  | #1492 smoothing-only leg | red, "the smoothing-only stream select allocated" **2** |
  | #1491 p7 | red: "staging-read staged collection differs from its options" |

  Every one was reverted, and the suite is green after the revert.
- **#1488's test still bites after #1492 restructured `ffi.rs`.** An injected `Box` in
  `spectrum_stream_start` is red ("spectrum_stream_start (refused: NaN smoothing) allocated", 2).
  An injected `Box` in `source_submit` is red ("source_submit (full quantum) allocated", 2). An
  injected `Box` in the `spectrum_select` export is red. All are green when reverted.
- **The module.** The shipped module is `9ea229e2...` (3,129,171 B), which is the digest that
  #1492's attempt record gives. The later #1489 and #1491 commits changed no module byte.

## Test value (new tests of the batch, confirmed on the merged tree)

- `post_boot_control_exports_are_render_locked_and_allocation_free` (`render_locked_source.rs`,
  #1488): red when any of the 11 D1 exports allocates inside its render-locked window. It stays
  red after #1492's `ffi.rs` restructuring (the `spectrum_stream_start` and `source_submit`
  injections above).
- `render_locked_staging.rs` phase 2 select legs and the smoothing-only leg (#1492): red when
  `select` or the bridge's select path allocates. The control with the selects unwrapped is green,
  so the wrap is what makes the count see it.
- The boot contract's staged-collection witness (`test-web-audioworklet.mjs`, #1491): red when the
  worklet skips collection staging (p7).
- The `recursive-svf-unarmed` allowlist row (#1490): the nightly report is green with
  `kernel_rules:4`, and its red-mutation suite passes. I did not re-run #1490's slice mutation,
  because no other slice touches the lane kernels or the probe. The only lane edit is #1489's
  comment-only `softfma.rs`.

## The gates run on 8bd74b1ef (all pass)

**How the run was made:**
- **Copies and targets:** each job ran in its own copy of the export, with
  `CARGO_TARGET_DIR=/tmp/claude-1002/vmisc2-batch/target/<job>`, `CARGO_INCREMENTAL=0`,
  `RUSTUP_TOOLCHAIN=1.97.1`, `GITHUB_SHA=8bd74b1ef...` and `GITHUB_EVENT_NAME=pull_request`.
- **Steps:** I took the step texts from this commit's `qualification.yml` with a YAML parser. They
  are byte-identical to batch-misc's extraction, because the workflow has not changed since then.
  Each step ran as `bash -e`.
- **Disk:** the watchdog was set at 25.5 GiB and never fired. The lowest free space was
  **31.9 GiB**.

| Job | Result |
|---|---|
| route | Both routing scripts pass. |
| docs-gates | "dsp research corpus: ok". The listening preregistrations are ok. |
| gate-self-tests (5) | All pass. CI skips this job here (`self_tests=[]`); I ran it anyway. |
| lint (32 steps) | All pass: fmt; clippy `-D warnings`; `cargo doc -D warnings`; workspace policy and its mutations; realtime policy (89 regions in 25 files) and its mutations; and the other policy, seal and probe steps, including the three sub-v3 and AVX2+FMA probes. |
| test-debug-a | builtins-compiler compile: pass. Workspace debug tests: **1,471 passed, 0 failed, 10 ignored** (126 binaries). That is +1 test and +1 binary against batch-misc: `render_locked_source`. `render_locked_staging` also passes. Doctests: 22 passed. |
| test-debug-b | **910 passed, 0 failed, 26 ignored** (163 binaries). Doctests: 3. Research-fixture completeness: pass. |
| test-release (9) | G-gates: 129 passed, 11 ignored. `filter_liveness`: 14. `tail_contract`: 22. `ramp_endpoint`: 16. `designer_total`: 1. M3 with FMA: pass. Loom SPSC: 1. M1 exhaustive: 2. F1 exhaustive: 5. |
| artifact | Module `9ea229e2f9c3158a...` (3,129,171 B). Named twin `3d6c006829ce1a39...`. Closure `7cc204a0296b9450...`. rustc 1.97.1. |
| artifact-identity | Self-test: pass. The twin, built from another path with another `CARGO_HOME` (`git archive`, not `git worktree add`), equals the built bytes. The report, run against the synthetic PR merge in the shared clone, reads main's status read only and gives **ARTIFACT CHANGED** `7c6ee735...` -> `9ea229e2...` against `1d294c700`'s recorded run 37864895035. It is reproducible. The pin `6c952a2c...` did not change, so this is not a release change, and the step exits 0. This change is expected: #1488 and #1492 changed `ffi.rs`. |
| artifact-gates (6) | Digest and twin checks: pass. Artifact gates and native parity: pass (32 red mutations). Scalar oracle absent: pass (2,845 symbols, none of 17). Hermetic host and worklet tests, with the leftover-TMPDIR check: pass. The boot contract has `callers=6 real-ready=6 real-disposed=6`. V8 spill gate: ok (Node 22.23.2). |
| sdk (3) | Pass. "SDK publishable-tarball gate passed". |
| browser: chromium 151.0.7922.34, firefox 153.0, webkit 26.5 | In each copy, `sdk/dist` was absent ("no sdk/dist"), and the step removes it anyway. So each run was in SDK source-bundle mode ("sdk bundle: the source ... (CI's mode)"). Each ran `npm run qualify -- --artifacts ... --sdk-root ... --browser B --check-matrix --self-test-mutations` with a private PulseAudio null sink. Each printed "**all qualification gates passed**". `render-allocations`: all 10 rows are 0. `sdk-render-allocations`: all 17 rows are 0 in each browser. These rows include `spectrum-collection=0`, which selects through the selects that are now render-locked, and `live-control=0`, which is #1488's wrapped live-control exports. |
| audit-native (20) | All pass. C ABI caller audit: 100,000 calls, 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations, `pcm_digest` **cb10fbface44a3a4** (unchanged). Graph determinism: 100/100. Effect conformance: 0 failed gates. The other audits, the trace, the probes, linkage, the fixtures and the bench also pass. |
| wasm-guests (5) | All pass. 146 cases, 258 comparisons, 0 mismatches. |
| cross-target | "cross-target matrix: PASS". The #1018 expected failures are unchanged (builtins 4, lane 1). |
| release-shape (2) | Pass. CI skips this job here (`release_inputs=false`); I ran it anyway. |
| Extra: nightly `native-vectorization-report`, "Generate report and run red mutations" (#1490) | Run verbatim in a fresh copy and target. `"status":"pass"`, `"kernel_rules":4`, `"failures":[]`, backend `x86_64-avx2`. "native vectorization red mutations: ok". |

## Jobs and steps not run, and why

- **`aarch64-debug`, `aarch64-release`:** they run only on arm64 runners. They are not emulated,
  as instructed. The cross-target job's native AArch64 iOS and Android `cargo check`/clippy rows
  ran and pass.
- **`artifact-record`:** it runs only on a push to main and writes a commit status.
- **`verdict`:** it aggregates the GitHub job results and posts telemetry. On its expectation
  table, `release-shape` and `gate-self-tests` would be skipped and every other x86 job would
  succeed.
- **Install, cache, upload and download steps:** the tools are installed on this machine, and the
  artifact files were copied in place of the downloads. The browser "Install pinned browser" step
  ran `npm ci` and `npx playwright install B` without its `sudo apt` lines.

## Not blockers (for root)

- **NIT. "boot" in the two headers.** The `ffi.rs` and `render_lock.rs` headers say "after boot".
  The worklet calls 28 unwrapped binding exports (`buffer_*`, `status_ptr`,
  `live_control_*`, the observation and source descriptors, `meter_header_ptr`,
  `prepared_companion_*`) *after* the `boot` export returns, inside `initialize`, before
  `miso.ready`. The sentence is true only when "boot" means the worklet's whole construction path.
  #1488's record and the policy doc's set 1 ("`process()` and its port handlers") use it that
  way. The headers do not define it. Consider one clause, "after boot (once `initialize` has
  posted `miso.ready`)", the next time someone edits `ffi.rs`.
- **NIT. `softfma.rs:116-117`** (root's `8bd74b1ef` text): "as every DAW callback does with FTZ
  and DAZ set" is a universal claim that has no evidence. The soundness argument does not need
  it. "as a DAW callback often does" would be accurate.
- **#1489's open items have no issue yet:**
  1. `softfma::write_mxcsr` and `fpenv::write_fp_control_word` are safe `pub fn`s. Their `SAFETY`
     ground is about what the callers pass. The #1489 verdict calls this a real soundness gap.
  2. `fpenv.rs:73-75` ("no call at all", "two register writes") is false on the shipped x86_64
     cdylib.

  Only the #1489 spec records these items. Under "no shortcuts", file a bounded successor for
  each.
- **GitHub sync after the push.** Sync all five bodies from the local specs. #1489's Amendment
  record already says its body predates the Amendment.
- **Spec and issue drift that comes from main (not this batch).** #1432 is open on GitHub and has
  no local spec. 74 specs in `.github/ISSUE_SPECS/` belong to closed issues, for example
  #1476-#1481, #1484, #1485 and many older ones. `main` has the same list. AGENTS.md says that a
  closed issue's spec leaves at the batch after it closes.

## Files

- **Logs:** `/tmp/claude-1002/vmisc2-batch/logs/` has every step's script and log,
  `results.tsv` (111 rows), `progress.log`, `router.txt`, `disk-min-kb`,
  `identreport/` (the identity report), `vecrep/evidence/report.json`, `mut/` (the mutation logs
  and `summary.txt`) and `gh/` (the GitHub bodies read for the comparison).
- **Harness:** the scripts (`lib.sh`, `job-*.sh`, `run-all.sh`, `mut.py`) and the extracted steps
  (`steps/`) are in `/tmp/claude-1002/vmisc2-batch/`.
- **Deleted:** the export, the per-job copies, the mutation copy, all targets, the runner temp
  (which included the twin) and the shared clone with its synthetic merge.
- **The worktree:** `/home/bl/misofm/wt-d15-misc2` was not changed. It is clean at `8bd74b1ef`.
