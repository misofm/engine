PASS

# #1494 attempt 1 -- adversarial verdict

Branch `codex/d15-batch-fp` (worktree `/home/bl/misofm/wt-d15-fp`), base `b81eb1fb0`. Commits
`966938497` (slice and evidence) and `1c838fba9` (`scripts/test-bench-policy.sh` only). I reviewed
`git diff b81eb1fb0 1c838fba9`. I exported the head with `git archive` to
`/tmp/claude-1002/v1494/tree` and the base to `/tmp/claude-1002/v1494/base`. I ran the mutations in
a third export (`/tmp/claude-1002/v1494/mut`). I did not build anything in the worktree.

The slice does what D1-D5 ask:
- `write_mxcsr` and all three `write_fp_control_word` variants are `pub unsafe fn`, each with a
  `# Safety` section.
- `read_mxcsr` and `read_fp_control_word` stay safe.
- `CanonicalFpEnv` is the only safe path that writes the word. A grep of `crates`, `hosts` and
  `tools` (tests excluded) finds writes only at `fpenv.rs:187`, `:255`, `:449` and `:473`, and at
  `softfma.rs:140`. `attest_fp_environment` uses the guard. No parallel safe wrapper exists.
- All 29 test call sites (5 files) are in `unsafe` blocks with `SAFETY` comments.
- Each writer has a `compile_fail` fence (no error code) with a passing twin, per #1422 D2 and
  Amendment 1.
- The codegen did not change.

I found no BLOCKER and no MAJOR. Most of the MINOR findings are about the contract text that D1
dictated, so root decides on them.

## MINOR

**m1. `# Safety` bullet 2 implies that a word handed back stays inside Rust's floating-point
model. The base text said the opposite, and the attempt deleted that text.**

Where: `fpenv.rs:158-161`, `:221-224`, `:279-282`, `softfma.rs:106-110`, and the quote at
`docs/REALTIME_DEPENDENCY_POLICY.md:91-96`.

The bullet says: "A word other than [canonical], or other than a word read from this thread that
is being handed back, leaves the floating-point environment Rust assumes".

- **Handed-back words.** The wording puts the handed-back word in the same class as the canonical
  word. A handed-back caller word that has FTZ/DAZ set also leaves Rust's model. That is the
  case `fpenv.rs:16-17` gives as the reason for the module. `core::arch` (`x86/sse.rs:1542-1546`,
  nightly-2026-08-20 rust-src) says that a change to DAZ is UB "even when the register is altered
  and later reset to its original value".
- **What the base said.** The base `write_mxcsr` `SAFETY` comment (`softfma.rs:116-118` at
  `b81eb1fb0`) gave the real ground: "the exit write itself installs a non-default word whenever
  the caller had one ... but after it the entry runs no floating-point code, so the caller's word
  reaches no engine computation". That text is the #1489 attempt-2 n1 fix, in root's `8bd74b1ef`
  wording. D1's rule "SAFETY says only what the body relies on" removed it, and no new text
  restates it.
- **Status-flag words.** The literal claim is also too wide. `0x1F80 | 0x003F` (`fp_env.rs:181-184`)
  is neither the canonical word nor a handed-back word, but it stays in Rust's model: `core::arch`
  lists masks, rounding and DAZ, not the status flags. This error is on the safe side.
- **Fix.** Add one clause: a handed-back word can itself be outside Rust's model (a host that
  runs with FTZ/DAZ set). Writing it back returns the caller to the environment it already ran
  under, and the guard runs no floating-point code after that write.

**m2. Bullet 3 is stated for every write, so its literal text does not match the hand-back writes
or a leaked guard.**

- **Hand-back writes.** "The caller restores the previous word before it returns" (`fpenv.rs:162-163`
  and copies, `softfma.rs:111-112`). Each hand-back write (`Drop` `:473`, each `Restore::drop`,
  each explicit write-back) does not restore the word it replaces, because the hand-back *is* the
  restore. The `SAFETY` comments are correct only under the intended reading: "restore after a
  write that is not a hand-back".
- **Leaked guard.** `enter`'s discharge (`fpenv.rs:447-448`, "this guard's `Drop` writes `saved`
  back") depends on `Drop`. Safe code can skip `Drop` with
  `core::mem::forget(CanonicalFpEnv::enter())`. This is sound only because the canonical word needs
  no restore. Neither the contract nor the comment says so.
- **Fix.** Limit bullet 3 to a word outside Rust's model, or add the leaked-guard reason to
  `enter`'s comment.

**m3. A policy sentence is false for one of the five files.**

`docs/REALTIME_DEPENDENCY_POLICY.md:149-151` says that each control-word test writer installs a
host's word "and measure[s] the guard under it". `crates/lane/tests/g6_ftz_inert.rs` has no guard
(no `CanonicalFpEnv` in the file). It measures the D7 flush arms under FTZ|DAZ. The file's own
entry (`:157-158`) is correct.

Fix: "and measure the engine under it", or name G6 as the exception.

**m4. #1495 now has a premise that is not true, and the attempt record does not say so.**

- **What #1495 depends on.** D1 deleted the `softfma.rs` `SAFETY` citation "(`fpenv.rs:16-17`)"
  (base `softfma.rs:117`). Two parts of #1495 assume that citation exists:
  - D3: "`softfma.rs:116-117` keeps citing `fpenv.rs:16-17`; if the line numbers move, update that
    citation";
  - its authorized path: "`crates/lane/src/softfma.rs` (the `fpenv.rs:16-17` citation in
    `write_mxcsr`'s `SAFETY` comment ...)".
- **The record.** It says "#1495's lines ... are unchanged". For `fpenv.rs` this is true (see
  below), but the record does not say that the `softfma.rs` copy is gone.
- **What root must do.** Amend #1495: drop D3's `softfma.rs` clause and that authorized path. Also
  refresh #1495's citations:

  | #1495 cites | Now at |
  |---|---|
  | `fpenv.rs:73-76` | `:77-80` |
  | `fpenv.rs:141`, `:148`, `:166`, `:181`, `:322-329` | `:145`, `:187`, `:205`, `:254`, `:431-438` |
  | `asm!` sites `:167`, `:182`, `:327` | `:206`, `:255`, `:436` |
  | `softfma.rs:103` | `:134` |

  #1494's Dependencies section says the later slice rebases, so the refresh itself is expected.
- **Update after the review.** The worktree moved to `f7a625d92` (#1495 attempt 1, on top of
  `1c838fba9`). That attempt record notes that #1494 removed the `softfma.rs` copy and that D3's
  citation no longer exists. So the #1495 implementer has reconciled it in the record, but D3 in
  the #1495 spec body still states it. This verdict covers `1c838fba9` only.

**m5. The spec's test-value sentence is true as worded, but it does not name the defect that only
the doctest catches.**

- **Why it is true.** Gate 1 defines "that change" as the `x86_64` variant and `write_mxcsr` made
  safe again. The x86 test jobs do not set `-D warnings`. With `write_mxcsr` made safe and the
  callers unchanged, `cargo test -p lane --tests` and the capi `fp_environment` tests pass, with 9
  `unused_unsafe` warnings (`fpenv.rs:187` and the 8 x86 sites in `fp_env.rs`). So no existing test
  fails, and every caller still compiles.
- **What else catches that change.**
  - The lint job: `cargo clippy -D warnings` fails on lane's own forwarder block (verified).
  - The AArch64 legs: for the AArch64 pair, the same partial mutation makes lane's own
    `enter`/`Drop` blocks unused. The legs build with `RUSTFLAGS: -D warnings`
    (`qualification.yml:989-990`, `:1016-1017`), so the leg fails before any test runs. For that
    pair, "because every existing caller still compiles" is not true in CI.
- **What only the doctest catches: the full revert.** That is, the writer made safe *and* the
  callers' `unsafe` blocks removed. I verified this: with `write_mxcsr` safe and every
  `unsafe { write_mxcsr(..) }` in `fpenv.rs`, `fp_env.rs` and the capi tests unwrapped,
  `cargo clippy -p lane -p capi --all-targets -- -D warnings` exits 0, and only the `write_mxcsr`
  fence goes red.
- **Proposed wording for root:** "red when its writer is made safe again and its callers' `unsafe`
  blocks are removed; no test and no lint catches that".
- The attempt record already says this candidly. The change is to the spec body, which is root's
  text.

## NIT

- **n1. `1c838fba9` (`scripts/test-bench-policy.sh`) is a necessary result of the authorized
  `check-bench-policy.sh` edit, and it is correct.**
  - The base self-test, run against the head check, exits 96 ("incomplete selected grep payload:
    unsafe-owner-grep-error").
  - With only the payload line fixed, it exits 96 at `count-error` (`output: 4`).
  - So both the payload path and the four `3`->`4` counts are needed. Nothing else changed.
  - The file is not in the spec's authorized paths, and it is not in stream G's named exception
    (`STREAMS.md:372` gives G "one path in `scripts/check-bench-policy.sh`'s `tools/` unsafe set
    (J's)"). Root should accept it on the same reasoning as the spec's conditional clause for
    `test-realtime-policy.sh`.
- **n2.** `crates/lane/tests/fp_env.rs:12-14` adds a module-doc paragraph. The spec allows "the
  write call sites and the `allow` attribute only", and the record's list of out-of-scope edits does
  not include this paragraph. It is true and harmless.
- **n3.** `scripts/check-realtime-policy.sh:21-23` says: "Its own unsafe sites are the AArch64
  `mrs`/`msr FPCR` pair ... and the empty `asm!` barrier". This is now incomplete: it leaves out the
  three `unsafe fn` declarations and the guard's and forwarder's `unsafe` blocks. The spec allowed
  only the exclusion line, and #1446 deletes this script, so the fix can wait.
- **n4.** `docs:98-99` and `:106-107` say that a doctest pair "proves" safe code cannot call the
  writer. For the no-op (wasm) variant, the pair is documentation only (`fpenv.rs:286-290`) and CI
  does not run it. My static probe (below) shows that it behaves as documented.
- **n5.** `docs:51` says: "Unsafe code must not leak through a public API". The policy now has two
  more `pub unsafe fn` in lane's public API. There is a precedent (capi `plan_carry_counts` and
  `plan_replacement_count`, `ffi.rs:1137`, `:1158`). The section could say that an explicit
  `pub unsafe fn` with a `# Safety` contract is not a leak.
- **n6.** The G6 write-back (`g6_ftz_inert.rs:125`) is not a guard. A panic in `all_arms()`
  (`:122`) skips it. The base had the same shape, and the spec does not allow a change to the
  scenario. libtest runs each test on its own thread, so the word ends with that thread. The
  `SAFETY` comment "the write-back below restores `saved`" is true on the normal path only.

## Judged as requested

- **`fpenv.rs:71-73`** (the "Why this file carries `unsafe`" paragraph). Accepted. Without it, the
  section's "one file, two reasons" does not cover the `unsafe fn` and the guard's `unsafe`
  blocks. The paragraph is true.
- **`g6_ftz_inert.rs:10-12`.** Accepted. The old "the workspace forbids it everywhere else" became
  false. The new text is true: two `unsafe` blocks are the file's only unsafe code, and the file is
  on the awk allowlist (`check-realtime-policy.sh:31`).
- **`check-bench-policy.sh:209-213`** (comment counts and sentence). Accepted. The edit keeps line
  `:210`, which #1446 cites, exact.
- **The #1495 passages are textually unchanged.**
  - `fpenv.rs:1-70` is byte-identical to the base, which includes the DAW text at `:16-17`.
  - "Realtime properties" (base `:71-76`) is byte-identical at head `:75-80`.
  - The `softfma.rs` copy of the citation is gone (m4).
- **capi `allow`.** It is an item-level `#[allow(unsafe_code)]` on `mod fp_environment`
  (`runtime/tests.rs:3092`, `cfg(x86_64)`). The file compiles only under `#[cfg(test)]`
  (`runtime/mod.rs:54-55`), so the `allow` does not reach non-test code. The file has no other
  `unsafe`.
- **The `SAFETY` comments are true.** I checked each word definition:
  - **x86 words.**
    - `hostile_word`: `(base & !0x6000) | FTZ | DAZ | 0x4000 | 0x0020`.
    - capi `hostile`: `| 0x6000 | 0x0020`.
    - host-core `word::{flushing, clear, hostile}`: bits 15, 6, 13-14 and 5.
    - `FLUSH_BITS`: FTZ|DAZ.
  - **AArch64 words.**
    - `hostile_word`: FZ bit 24, DN bit 25, RMode `0b01<<22`.
    - host-core: FZ and RMode.
    - `FLUSH_BITS`: FZ.
  - **No reserved bit.** None of these words sets MXCSR bits 16-31 or an FPCR `RES0` bit.
  - **Callers.** Every caller of the capi helpers, `WordGuard::set` and host-core `arm` passes a
    word read from the same thread, with only the stated bits changed.
  - **Restores.** Each comment names the correct restore: `Restore`, `_restore`, `WordGuard`'s
    `Drop`, an explicit write-back, or "is the write-back".
  - **Lane bodies.**
    - The forwarder (`:186`) cites `write_mxcsr`'s contract. On `x86_64` the two contracts are
      word-for-word the same.
    - `enter` (`:447-448`) and `Drop` (`:470-472`) are true, with m2's limits.
    - The AArch64 body (`:249-253`) and the `write_mxcsr` body (`:136-139`) say only what the
      body relies on.
- **Citations are exact at head.**
  - Policy: `fpenv.rs` `:145`, `:187` (the forwarder calls), `:205`, `:254` (the `unsafe {` lines
    of `mrs`/`msr`, the same convention as base `:166`/`:181`), `:431-438` (`scheduling_barrier`),
    `:449`, `:473`.
  - Record: every line in its D3 table, `:169`/`:177`, and `softfma` `:118`/`:126`.
  - Policy section paths equal the allowlist: 24 paths each, sorted diff empty.
  - #1446's `fpenv.rs:63`, `softfma.rs:16`, `:29` and `check-bench-policy.sh:210` are still exact.
    Its policy-doc citations were already stale at the base.

## Test value (one sentence per new test)

- **`softfma::write_mxcsr` fence** (`softfma.rs:118`): red when `write_mxcsr` is made safe again
  and its callers' `unsafe` blocks are removed. No test and no lint catches that (verified); the
  signature-only form is also caught by clippy's `unused_unsafe`.
- **`write_mxcsr` twin** (`:126`): red when a rename or typo in the shared code would keep the
  fence green for the wrong reason. Verified with `read_mxcsrr` and `write_mxcsrr`: in both cases
  the twin is FAILED and the fence stays ok.
- **`fpenv::write_fp_control_word` fence, x86_64** (`fpenv.rs:169`): the same as the
  `write_mxcsr` fence, for the `fpenv` writer. It is red on the signature-only mutation, and the
  `write_mxcsr` fence stays green under it.
- **`write_fp_control_word` twin, x86_64** (`:177`): red when `read_fp_control_wrd` or
  `write_fp_control_wrd` is in the shared code. The fence stays green in both cases.
- **AArch64 pair** (`:232`/`:240`): the same defects on AArch64. These run in CI's `aarch64-debug`
  doctest leg only, and I did not emulate them. A static compile (`cargo check --example` of each
  body for `aarch64-unknown-linux-gnu` and `aarch64-linux-android`): the fence body gives exactly
  E0133 and the twin body compiles.
- **No-op pair** (`:292`/`:297`): documentation only. It never runs, so it gets no test-value
  claim. A static wasm32 (`+simd128`) compile gives E0133 for the fence and ok for the twin.
- **Fence reasons.** I made each fence a plain doctest once. Each failed with exactly one error:
  `error[E0133]: call to unsafe function write_mxcsr / write_fp_control_word is unsafe and requires
  unsafe block`. This matches the comments.
- **Acked-batch question.** No queue is touched, so it does not apply.

## Gates run (exported `1c838fba9`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1494/target`)

1. **Gate 1, `cargo test --locked -p lane --doc`:** 3 plain and 4 `compile_fail`, all ok. Each
   mutation was reverted from a byte copy, and the final baseline is green again.

   | Mutation | Red | Other doctests |
   |---|---|---|
   | (a) x86_64 `write_fp_control_word` safe | `:169` compile fail FAILED, exit 101 | all ok |
   | (a) `write_mxcsr` safe | `:118` FAILED | all ok |
   | (b) typos in the shared code (4 cases) | the twin FAILED | the fence ok |
2. **Gate 2.**
   - `cargo test --locked -p lane`: exit 0.
   - `-p capi`: 71 + 2 + 11 tests, and 0 doctests.
   - `-p host-core --test fp_environment`: 3 passed.
   - `-p wasm-gates --test g6_full_corpus_ftz`: 2 passed (45.2 s).
3. **Gate 3.**
   - `check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)`.
   - `test-realtime-policy.sh`: ok, with no edit needed.
   - `check-bench-policy.sh`: `ok (... 4 unsafe owners ...)`.
   - `test-bench-policy.sh`: `bench policy mutations: ok`.
   - `check-lane-policy.sh`: ok.
   - `check-workspace-policy.sh`: ok (run in a `git init` copy).
   - Allowlist and section: equal.
4. **Gate 4.**
   - `build-web-audioworklet.sh --named-twin`: shipped `9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939`,
     named twin `3d6c0068...`.
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `audit capi`: `pcm_digest` `cb10fbface44a3a4`, `allocations` 0, `syscalls` 0, `locks` 0,
     `total_violations` 0.
5. **Gate 5.**
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`: exit 0.
   - The CI form with `--all-features`: exit 0.
   - `cargo fmt --all -- --check`: exit 0.
   - `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p lane --no-deps`: exit 0. The `--workspace`
     form: exit 0.
   - `check-cross-targets.sh`: `cross-target matrix: PASS`.
6. **`qualification.yml` doctest steps.**
   - "Workspace doctests": exit 0 (22 passed).
   - "DSP crates doctests": exit 0, with lane's 3 plain and 4 `compile_fail`.
7. **Extra (static, no emulation).** `RUSTFLAGS='-D warnings' cargo clippy --tests --target
   aarch64-unknown-linux-gnu -p lane -p host-core -p capi -- -D warnings`: exit 0. This lints the
   AArch64 `cfg` arms of `fp_env.rs`, `g6_ftz_inert.rs` and `fp_environment.rs`. For `wasm-gates`,
   wasmtime's build script needs an aarch64 C compiler, which this host does not have, so that
   crate's AArch64 arm was not checked.

Not verified: any AArch64 or wasm *run*. These are CI only.

Evidence: `/tmp/claude-1002/v1494/ev/` (logs, `gates*.out`, `mut*.out`, `allow.txt`, `doc.txt`,
`audit-capi.json`).
