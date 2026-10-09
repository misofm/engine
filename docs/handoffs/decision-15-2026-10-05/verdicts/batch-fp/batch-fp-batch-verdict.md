ready to push

# Decision-15 batch-fp verdict: `codex/d15-batch-fp` at 4d6f18625

Base: main `b81eb1fb0` (`git ls-remote origin refs/heads/main` = `b81eb1fb0`). Commits
`966938497` + `1c838fba9` (#1494, PASS), `f7a625d92` (#1495, PASS), `4d6f18625` (root's
follow-ups, not verified before this verdict). The branch is not on origin yet.

**Summary:**
- Job A: `4d6f18625` is comment/doc only (every changed `.rs` line is a `//` or `///` line). I
  found no BLOCKER and no MAJOR. One MINOR (a policy sentence that implies only G6 measures code
  with no guard) and some NITs. None needs a re-gate.
- Job B: every `qualification.yml` step that an x86_64 runner can run passes: **110 steps, 0
  non-zero exits**. This includes the three browser legs. The shipped module is
  `9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939`, and the C ABI audit gives
  `pcm_digest` `cb10fbface44a3a4`, 0 allocations and 0 syscalls.
- The merge is a fast-forward (`b81eb1fb0` is an ancestor of `4d6f18625`). The synthetic PR merge
  tree is `bb8b70ab`, which is the head tree. No workflow changed (`git diff b81eb1fb0 4d6f18625 --
  .github/workflows` is empty), and the steps I extracted are byte-identical to batch-misc2's.
- GitHub (read only): #1494 and #1495 are OPEN, and their titles equal the local H1s. Sync the
  bodies and close them after the push.

## Job A: adversarial review of 4d6f18625

**What I checked against which source:**
- The `_mm_setcsr` text in `core::arch` (nightly-2026-08-20 rust-src, `x86/sse.rs:1542-1546`).
  The 1.97.1 toolchain has no rust-src.
- `fpenv.rs`, `softfma.rs`, `builtins/src/tail.rs`, the five test-writer files and the policy doc,
  all at the head.
- The shipped `libcapi.so`. I built it with `-p capi` alone, as `check-capi-abi.sh` does, in this
  run's audit-native job, and disassembled it.

### Sentence by sentence (the writers' `# Safety` text, `fpenv.rs:154-171` and copies `:225-242`, `:292-309`; `softfma.rs:102-118`)

| Claim | Verdict | Evidence |
|---|---|---|
| Rust assumes the canonical environment; the `core::arch` doc "names the default exception masks, rounding and DAZ" | True | `sse.rs:1542-1543`: changing "the masking flags, rounding mode, or denormals-are-zero mode flags leads to immediate Undefined Behavior: Rust assumes that these are always in their default state". |
| Only canonical, or (x86_64) canonical differing only in status bits 0-5, is inside | True for x86_64 | Bits 6 (DAZ), 7-12 (masks) and 13-14 (RC) are named by the doc. Bit 15 (FTZ) is not named by the UB sentence, but it changes results that LLVM assumes are IEEE (NIT n2). Status flags are not in the doc's list. |
| A handed-back word can be outside, because a host can run with FTZ/DAZ (`FZ`) | True | This fixes the attempt-1 verdict's m1. |
| A hand-back "meets this" when it returns the thread to the word its caller already ran under and the writer runs no floating-point code after it | True as the engine's rule | Any code after a hand-back runs under a word that it would have run under with no guard. This includes `tail.rs` preparation and a register-only value that the optimizer sinks past `Drop` (`fpenv.rs:446-451`). So the guard never puts code under a word that the code did not already run under. The text no longer claims that the handed-back word is inside Rust's model. |
| Bullet 3: no restore for a word inside the environment or for a hand-back; a hand-back is itself the restore | True | `CanonicalFpEnv`'s own two writes are canonical (inside) and a hand-back. Every test writer that installs a hostile word restores it through `Restore`, `WordGuard` or an explicit write-back. |
| "this is why `CanonicalFpEnv::enter` stays sound when safe code passes the guard to `mem::forget`" | True | The canonical word stays installed. That is inside Rust's model, and bullet 3 asks for no restore. |
| `enter` SAFETY (`fpenv.rs:472-476`) | True | Same ground. "loses the caller's word but leaves no code outside Rust's model" is exact. |
| `Drop` SAFETY (`fpenv.rs:498-501`) | True (see NIT n4) | `!Send`, so the same thread. A word read by STMXCSR has bits 16-31 clear. The only statement after the write is the return, and the barrier comes before the write (`:497`). |
| Body SAFETY comments (`fpenv.rs:265-270`, `softfma.rs:142-146`): "restores the previous word where the contract requires it" | True | -- |
| #1494 record: builtins preparation "keeps computing on its thread after the guard drops, under the host's word" | True for `fixed_input_walk` (NIT n3 for the other two names) | `tail.rs:262`. Its callers (`builtins/src/lib.rs:3505`, `:3583`) keep working after `fixed_input_bound` returns. In the cdylib, the builtins-preparation iterator calls `fixed_input_bound` at `0x16482f` through GOT `0x43c020`, which resolves to `0x3cda80`. |

### Policy doc (`docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership")

- **Citations are exact at the head:**
  - `fpenv.rs:145` (`read_mxcsr()`) and `:195` (the forwarder's `write_mxcsr`).
  - `:213` and `:271`: the `unsafe {` lines of `mrs` and `msr`. This is the same convention as the
    old `:205`/`:254` and the base `:166`/`:181`.
  - `:456-463`: `fn scheduling_barrier` through its closing brace.
  - `:477` (`enter`'s write) and `:502` (`Drop`'s write).
  - The #1494 record's doctest lines are also exact: `softfma.rs:124`/`:132`, and `fpenv.rs`
    `:177`/`:185`, `:248`/`:256` and `:317`/`:322`. The doctest run lists `softfma.rs` line 124
    and `fpenv.rs` line 177 as compile_fail ok.
- **"pub unsafe fn is not a leak" (`:51-53`):**
  - It is coherent. AGENTS.md has no rule about unsafe code. The policy still requires a named,
    allowlisted file, because `unsafe_code = "deny"` fires on any `unsafe fn` declaration.
  - Inside the workspace, edition 2024 together with `unsafe_op_in_unsafe_fn = "deny"`
    (`Cargo.toml:45`, `:91`) makes every caller use an `unsafe` block, even a caller inside an
    `unsafe fn`.
  - The existing `pub unsafe fn`s (capi `plan_carry_counts` and `plan_replacement_count`) have
    `# Safety` sections. See NIT n5 for the C-caller scope.
- **Doctest sentences (`:103-104`, `:111-115`):**
  - They are true.
  - `scripts/run-aarch64-tests.sh:145-147` runs `cargo test --doc` on the product crates, and
    lane is one of them.
  - The no-op pair runs nowhere.
- **The allowlist is unchanged:** 24 paths in the `check-realtime-policy.sh:31` exclusions, and
  each one is listed in the section.

### MINOR

**m1. `docs/REALTIME_DEPENDENCY_POLICY.md:157-160`: the corrected G6 sentence now suggests that
only `g6_ftz_inert.rs` runs measured code with no guard. Four of the other files do so too.**

The text is "...and measure the engine under it: the guard, or in `g6_ftz_inert.rs` the D7 flush
arms, which run with no guard". Each of these files also has an unguarded control arm under the
hostile word:
- `fp_env.rs:153` (the `flushed` product, outside the guard; also `:330` on AArch64);
- `capi/src/runtime/tests.rs:3182` (`render_behind_the_c_entry`);
- `host-core/tests/fp_environment.rs:88` (`render_unguarded`);
- `g6_full_corpus_ftz.rs:112` (`unguarded_report`, the whole corpus under FTZ+DAZ with no guard).

So the "or" clause marks the wrong exception. This is the same sentence as the attempt-1
verdict's m3, so the fix is still not exact.

Suggested fix: "and measure the engine under it: through the guard, and with no guard as a control
(`g6_ftz_inert.rs` measures only the D7 flush arms, with no guard)". This is a doc change only and
needs no re-gate.

### NIT

- **n1. Line wrap.** These lines pass the doc's 100-column wrap:
  - Policy doc `:104` (125 columns) and `:118` (105).
  - #1495 record `:193` (126) and `:228` (121). These are the lines that were corrected in place.
- **n2. FTZ has no cited source.** Bullet 2 cites the `core::arch` doc for masks, rounding and DAZ,
  and then also puts FTZ (bit 15) outside the environment. The doc's UB sentence does not name FTZ.
  The classification is correct (FTZ flushes results that the compiler assumes are IEEE
  subnormals), but it rests on that reason, not on the cited sentence.
  The same applies on AArch64. "Only the canonical word ... is inside" is stricter than Rust's
  model: `FPCR.DN` only selects the default NaN, and Rust's NaN rules allow that result. The
  clause "the caller treats every other word as outside it" already makes the rule safe.
- **n3. #1494 record, m1 bullet.** It names `live_bound` and `live_composition` as "builtins
  preparation". They are reached only from `input_section_live_bound` and
  `input_section_live_cascade`, which run in the gate and the tests. Preparation reads
  `input_section_live_bound_table`, a `const fn` (`tail.rs:556`). Only `fixed_input_walk` is on the
  preparation path. The claim about computing after `Drop` is still true for all three.
- **n4. `Drop` SAFETY, "the word the caller already ran under".** This is exact when the guard is
  dropped in the scope that entered it, and every engine guard is a scope-local `let` (13 sites).
  But the guard is `'static` and only `!Send`, so safe code could move it, for example into a
  `thread_local!`, and drop it in a later host call that arrived with a different word. This needs
  a host that first entered with an outside word, so it adds no unsoundness. "The word this thread
  ran under when `enter` read it" would be exact.
- **n5. Scope of `:51-53`.** "the compiler makes every caller take on that contract" holds for
  Rust callers. The capi exports are also `pub unsafe extern "C" fn`, and their C callers have no
  compiler check. That boundary has its own entry (`:179-184`), so this is wording only. "A Rust
  `pub unsafe fn`" would be exact.
- **n6 (pre-existing, for root; not new in `4d6f18625`).** The `core::arch` doc calls a change to
  masks, rounding or DAZ immediate UB "even when the register is altered and later reset to its
  original value" (`sse.rs:1544-1546`). So the contract's "except the code it deliberately
  measures" exception, and a hand-back of a host's FTZ/DAZ word, are UB by the letter of
  `core::arch`. The contract is the engine's rule for running tests, not a Rust-level soundness
  guarantee. The new text no longer claims otherwise. Root may want one sentence that says this
  plainly.

### #1495 record corrections (n2-n4), checked against this run's shipped `libcapi.so`

The function addresses are identical to the record: render `0x80880`, `fixed_input_bound`
`0x3cda80`, `read_mxcsr` `0x3e6bb0` and `write_mxcsr` `0x3e6bd0`.
- **Exits.** These branches go to `0x808eb`/`0x808f1`: `0x808ad`, `0x808c2`, `0x808c9`,
  `0x808d3`, `0x808d8`, `0x808df`, `0x80915`, `0x8091c`, `0x80958`, `0x809de`, `0x809f1`,
  `0x80a04` and `0x80e94`. That is exactly the record's list. The function has one `ret`
  (`0x8090d`), and no jump leaves the function.
- **GOT slots.** The slots `0x43b740` and `0x43b748` are called at exactly 7 sites: `0x80897`,
  `0x808a4`, `0x808f6`, `0x80928`, `0x3cda9d`, `0x3cdaaa` and `0x3cdf69`.
- **Builtins preparation.** `0x16482f` calls GOT `0x43c020`, which resolves to `0x3cda80`. The
  call is inside the `input_section_bounds_within` iterator of
  `prepare_session_builtins_with_live_controls_and_policy`.

The copied verdicts `verdicts/batch-fp/1494-attempt1.md` and `1495-attempt1.md` are byte-identical
to the originals in `submix-verdicts/`.

## Job B: the gates run on 4d6f18625 (all pass)

**Method:** I used batch-misc2's harness, re-pointed to this commit.
- **Export:** `git archive 4d6f18625` into a one-commit repository. Its tree is `bb8b70ab`, which
  equals the commit's tree. The tracked-but-ignored `docs/evidence/metering-519-520/timing-failed.log`
  was added with `-f`.
- **Copies and environment:** each job ran in its own copy, with
  `CARGO_TARGET_DIR=/tmp/claude-1002/vbatch-fp/target/<job>`, `CARGO_INCREMENTAL=0`,
  `RUSTUP_TOOLCHAIN=1.97.1`, `GITHUB_SHA=4d6f186...` and `GITHUB_EVENT_NAME=pull_request`.
- **Steps:** I extracted the step texts from this commit's `qualification.yml` with a YAML parser.
  Each step ran as `bash -e`.
- **Disk:** the watchdog was set at 25.5 GiB and never fired. The lowest free space was
  **35.2 GiB**.

| Job | Result |
|---|---|
| route | Router (`pull_request` and `push`, `--base b81eb1fb0 --head 4d6f18625`, in a `--shared` clone): `route=full`, `math_closure=true`, `release_inputs=false`, `self_tests=[]`. `check-ci-path-routing.py` and `test-ci-path-routing.py`: pass. |
| docs-gates | "dsp research corpus" ok, and the listening preregistrations are ok. |
| gate-self-tests (5) | All pass. CI skips this job here (`self_tests=[]`); I ran it anyway. |
| lint (32 steps) | All pass: fmt; clippy `--all-features -D warnings`; `cargo doc -D warnings`; workspace policy and its mutations; bench policy ("4 unsafe owners") and its mutations; realtime policy (89 marked regions in 25 files) and its mutations; lane policy and its mutations; the unfused seal; the three sub-v3 and AVX2+FMA probes; and the remaining policy steps. |
| test-debug-a (3) | The builtins-compiler compile passes. Debug tests: **1,471 passed, 0 failed, 10 ignored** (126 binaries). Workspace doctests: 22 passed. |
| test-debug-b (3) | **910 passed, 0 failed, 26 ignored** (163 binaries). DSP doctests: **7** (lane: 3 plain, plus 4 `compile_fail`: `softfma.rs` line 124, `fpenv.rs` lines 177, 414 and 419). Research-fixture completeness: pass. |
| test-release (9) | G-gates: 133 passed, 11 ignored. This is +4 against batch-misc2: the new lane doctests run in this step too. `filter_liveness`: 14. `tail_contract`: 22. `ramp_endpoint`: 16. `designer_total`: 1. M3 is built with FMA (`target_feature="fma"`). Loom SPSC: 1. M1 exhaustive: 2. F1 exhaustive: 5. |
| artifact | Module `9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939`, named twin `3d6c006829ce1a39...`, closure `7cc204a0296b9450...`, rustc 1.97.1. |
| artifact-identity | Self-test: pass. The twin, built from another path with another `CARGO_HOME`, is byte-identical. The report against the synthetic PR merge reads main's status read only: **ARTIFACT UNCHANGED** against `b81eb1fb0`'s recorded run 37879401215. The pin `6c952a2c...` did not change, so this is not a release change. |
| artifact-gates (6) | The digest and named-twin checks pass ("9 non-custom and 2 other custom sections byte-identical"). `check-web-audioworklet.sh` passes ("static/object checks passed"), and `check-browser-expected-resources.py --artifacts` passes (32 red mutations). Scalar oracle absent: 2,845 symbols, none of 17. `test-web-audioworklet.sh` passes with the private-TMPDIR leftover check; the boot contract gives `callers=6 real-ready=6 real-disposed=6`. V8 spill gate: ok (Node 22.23.2). |
| sdk (3) | Pass: "SDK publishable-tarball gate passed". |
| browser: chromium 151.0.7922.34, firefox 153.0, webkit 26.5 | Before each run, `sdk/dist` was absent ("no sdk/dist"), and the step removes it anyway. Each run printed "sdk bundle: the source ... (CI's mode)". Each ran `npm run qualify -- --artifacts ... --sdk-root ... --browser B --check-matrix --self-test-mutations` with a private PulseAudio null sink. Each printed "artifact set: the exact 7-file shipped set is pinned" and "**all qualification gates passed**". All 27 render-allocation and SDK render-allocation rows are 0 in each browser. |
| audit-native (20) | All pass. **C ABI caller audit:** 100,000 calls, `pcm_digest` **cb10fbface44a3a4**, 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations; the record validator passes. **`check-capi-abi.sh`:** "C ABI check: ok (shared and static linkage)", and its mutations are ok. **Release unit tests:** 113 passed, 2 ignored. The delay, compressor, EQ, gate, builtins (1M), graph (1M), protocol, realtime trace (1M), probe mutations and effect (1M) audits pass. Scalar oracle absent from `libcapi.so`. Graph determinism: 100/100. Builtins fixtures: 50 files. Console fixtures and effect conformance (8 factories) pass. |
| wasm-guests (5) | All pass. wasmtime 47.0.3: 146 cases, 258 comparisons, 0 mismatches. |
| cross-target | "cross-target matrix: PASS". The #1018 expected failures are unchanged (builtins 4, lane 1). |
| release-shape (2) | Pass. CI skips this job here (`release_inputs=false`); I ran it anyway. |

## Jobs and steps not run, and why

- **`aarch64-debug`, `aarch64-release`:** they run only on arm64 runners, and I did not emulate
  them, as instructed. This means the AArch64 `write_fp_control_word` doctest pair and the
  AArch64 arms of `fp_env.rs`, `g6_ftz_inert.rs` and `fp_environment.rs` did not run here. The
  cross-target job's AArch64 iOS and Android `cargo check` and clippy rows ran and pass.
- **`artifact-record`:** it runs only on a push to main, and it writes a commit status.
- **`verdict`:** it aggregates GitHub job results and posts telemetry. On its expectation table,
  `release-shape` and `gate-self-tests` would be skipped, and every other x86 job would succeed.
- **Install, cache, upload and download steps:** the tools are installed on this machine. The
  artifact job's files were copied in place of the downloads. The browser "Install pinned browser"
  step ran `npm ci` and `npx playwright install B` without its `sudo apt` lines.
- **`artifact-identity` twin:** CI's step uses `git worktree add`, which I was told not to use. The
  twin was built from a second `git archive` of the same commit at another path, with another
  `CARGO_HOME`. The claim it tests is the same.

## Files

- **Logs:** `/tmp/claude-1002/vbatch-fp/logs/` has each step's script and log, `results.tsv`
  (110 rows), `progress.log`, `router.txt` and `disk-min-kb`.
- **Evidence:** `/tmp/claude-1002/vbatch-fp/evidence/` has `render.dis`, `got-slots.txt`,
  `mxcsr-helpers.dis`, `allow.txt`, `doc-paths.txt` and `section.md`.
- **Harness:** `lib.sh`, `job-*.sh`, `run-all.sh`, `extract.py` and `steps/` are in
  `/tmp/claude-1002/vbatch-fp/`.
- **Deleted:** the export, the per-job copies, all targets, the runner temp (which included the
  twin), the shared clone with its synthetic merge, and the 38 MB `libcapi.so` with its full
  disassembly.
- **The worktree:** `/home/bl/misofm/wt-d15-fp` was not changed. It is clean at `4d6f18625`.

## Root's rulings (2026-10-09) and recorded items

Added by root to this copy only; the original verdict is unchanged.

**Rulings.**

1. **Accepted: `1c838fba9`** (`scripts/test-bench-policy.sh`; #1494 verdict n1). Recorded in
   #1494's spec ("Root's rulings after attempt 1") and added to its Authorized paths with
   "(root, 2026-10-09)".
2. **Accepted: #1494's extra doc edits**: the `fpenv.rs` module-doc paragraph in "Why this file
   carries `unsafe`", `g6_ftz_inert.rs:10-12` and `fp_env.rs:12-14` (#1494 record's out-of-scope
   list, verdict n2). Recorded in #1494's spec the same way.
3. **#1495 D3 amended** (#1494 verdict m4): `softfma.rs`'s citation of `fpenv.rs:16-17` went with
   #1494 D1, so D3 and #1495's `softfma.rs` authorized-path line are amended (root, 2026-10-09).
4. **#1494's *Test value* sentence reworded** (#1494 verdict m5): the D4 `compile_fail` doctest
   alone catches the full revert (the writer made safe again and the callers' `unsafe` blocks
   removed); a signature-only revert is also caught by clippy's `unused_unsafe` under
   `-D warnings` in the callers, including the AArch64 `-D warnings` legs.
5. **Filed #1498** *Narrow the universal DAW-callback FTZ/DAZ claim in host-core and the C ABI
   tests* (stream G lead; named exceptions: `crates/host-core/src/render_session.rs:8-9` and
   `crates/host-core/src/lib.rs:78`, which no stream lists, and
   `crates/capi/src/runtime/tests.rs:3080-3081`, B's), from the #1495 verdict's m1, with #1495
   D3's narrowing of `fpenv.rs:16-17` as its model.
6. **Filed #1499** *Build the shipped mobile C ABI libraries with the release profile's fat LTO,
   and gate it* (stream B: `crates/capi` and its crate-type list are B's, and no stream owns mobile
   build tooling), from the #1495 verdict's open item 1. Root's re-check:
   `cargo build --locked --release -p capi -v` passes no `-C lto` to `capi` (crate types `rlib`,
   `staticlib`, `cdylib`); `cargo rustc --locked --release -p capi --lib --crate-type staticlib -v`
   passes `-C lto=fat`. The non-LTO `libcapi.a` has 386 members (per crate); the fat-LTO one has
   309 (one `capi` module, plus `compiler_builtins` and C builtins objects). #1472's ratchet and
   #1495's `x86_64` sentence in `fpenv.rs` ("Realtime properties") depend on it.

**Recorded, no change.**

- `scripts/check-realtime-policy.sh:21-23` lists an incomplete set of `fpenv.rs` unsafe sites
  (#1494 verdict n3); #1446 deletes that script.
- `g6_ftz_inert.rs`'s write-back is not panic-safe (#1494 verdict n6); unchanged from the base.
- Stale `fpenv.rs` line citations in older specs (#1321 `:288`, #1422 `:278`/`:360`, #1478,
  #1489) are records of their own commits and stay as they are.
- This verdict's NITs n2-n6: FTZ (and, on AArch64, "only the canonical word") is placed outside the
  environment without a cited source (n2; the classification is right, and "the caller treats
  every other word as outside it" keeps the rule safe); n3 is fixed in #1494's record
  (`afea51c57`); the `Drop` comment's "the word the caller already ran under" is exact only when
  the guard is dropped in the scope that entered it (n4); "the compiler makes every caller take on
  that contract" holds for Rust callers only, not C callers of the `capi` exports (n5); and
  `core::arch` calls a changed control word immediate UB even when it is restored, so the
  measured-test exception is the engine's own rule, not a Rust-level guarantee (n6).
- `afea51c57` (doc only) landed after this batch run (which ran on `4d6f18625`); `cargo fmt`, the
  policy gates and the docs-gates steps were re-run on it.

**GitHub bodies to sync** (not edited by root's workers): #1494 (Authorized paths, *Test value*,
attempt record) and #1495 (D3, Authorized paths).
