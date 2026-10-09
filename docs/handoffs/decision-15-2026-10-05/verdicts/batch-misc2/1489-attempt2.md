PASS

# #1489 attempt 2 -- adversarial verdict

Commit `3b10ae68c` (parent `bb84a30fc`) on `codex/d15-batch-misc2`. Attempt 1 is `30cc36ef9`
(parent `b0256b89d`). The combined change of both commits was reviewed against the spec. The
commit was exported with `git archive` to `/tmp/claude-1002/v1489/tree`, and nothing was built in
the worktree.

Scope is clean:
- Attempt 2 touches 3 paths: the spec (attempt record only), `crates/lane/src/softfma.rs` and
  `docs/REALTIME_DEPENDENCY_POLICY.md`.
- In the combined diff (`b0256b89d..3b10ae68c`, limited to the slice's paths),
  `git diff -U0` of `softfma.rs` and `scripts/check-realtime-policy.sh` shows only `//` and `#`
  lines.
- Every policy hunk is inside "Unsafe-code ownership" (`:45-334`).
- Between `fb303f836` and `bb84a30fc`, only `scripts/test-web-audioworklet.mjs`,
  `hosts/host-web/MUTATIONS.md` and the #1491 spec changed. `ffi.rs`, `render_lock.rs` and both
  worklets are byte-equal to the verdict-1 head.

## Attempt-1 findings: all fixed

- **M1 (fixed).** `softfma.rs:105-116` and its quote at `docs:89-97` now say:
  - "The entry write installs only `CANONICAL_MXCSR` (0x1F80, the architectural default), which
    is the environment Rust assumes, whatever word the caller had."
  - "The exit write restores the caller's own word, which `fpenv` read from this thread on entry."

  Both are true. `enter` (`fpenv.rs:336-339`) reads the word, writes `canonical_fp_control_word()`
  (`:258` = `CANONICAL_MXCSR` = `0x1F80`, `:85`) and runs the barrier. `Drop` (`:357-360`) runs the
  barrier and writes `self.saved`. The guard is `!Send` (`:300`), so "this thread" holds.
  "On the render path `fpenv` makes two writes" is true. The `core::arch` doc (nightly-2026-08-20
  rust-src `x86/sse.rs:1542-1546`) names masks, rounding and DAZ, as stated. The policy quotes
  each restated clause word for word.
- **m1 (fixed).** `softfma.rs:98-100` and `docs:97-98` say that `write_mxcsr` does not restore the
  previous word, and that `fpenv` restores it in the guard's `Drop` and each test before it
  returns. This is true:
  - `write_fp_control_word` (`fpenv.rs:147-149`) writes and returns.
  - Every test writer restores, through `Restore` (`fp_env.rs:59-65`, `capi runtime/tests.rs:3102-3108`,
    `host-core tests/fp_environment.rs:190-196`), `WordGuard` (`g6_full_corpus_ftz.rs`) or an
    explicit write-back (`g6_ftz_inert.rs:117`, `runtime/tests.rs:3167`, `:3207`, `:3313`).
- **m2 (fixed).** `docs:301-304` names `resource_lifecycle.rs`'s `LifecycleAllocator`. This is
  true: the file's `#[global_allocator]` is at `:24-25` and its `unsafe impl` at `:55-88`, and the
  render calls are at `:368` and `:371`.
- **n1 (fixed).** Every claim of `docs:290-300` is true at the commit:
  - Counting rule: `count_if_locked` (`render_lock.rs:51-55`) is the only writer of
    `RENDER_ALLOCATIONS` (one `fetch_add`). Its other use is the `load` at `:107`. Nothing in
    `hosts/host-web/src` resets it, so "from instantiation, never reset" holds.
  - Boot-time render-locked calls: a script over the engine worklet assigns every
    `miso_engine_web_v1_*` reference to its class method. The calls inside `constructor`,
    `initialize`, `stageSpectrumRequest`, `stageSpectrumCollectionRequest`, `bindLiveControls`,
    `writeBootOptions` and `failInitialization` that are also in the 42-export `render_locked` set
    are exactly the eleven named accessors (`:339`, `:341-346`, `:702-703`, `:749-751`, all after
    `boot` at `:330`) and `spectrum_target_id_ptr`/`_capacity` (`:455-456`, before `boot`).
    Every other boot call is unwrapped: the boot exports, `buffer_*`, `status_ptr`,
    `resource_ptr`, `live_control_*`, `source_*`, `observation_*` metadata, `spectrum_request_*`,
    `spectrum_collection_*`, `meter_header_ptr`, `prepared_companion_*` and `command_report_ptr`.
  - Staging origin:
    - `reserve_stagings` is at `ffi.rs:617-626` (exact). Its only call is at `:3820` in
      `boot_staged`, which both boot exports share (`:3840`, `:3852`).
    - The eleven bodies (`:3428-3505`, `:4235-4363`) only borrow a staging and return a pointer
      or a constant.
    - `spectrum_request_ptr` (`:2512`, unwrapped) reaches `leaked_staging`
      (`:579-591`, `Box::leak` on first touch) before `spectrum_target_id_ptr`. That accessor
      (`:2676`, wrapped) only borrows `target_id`, and `_capacity` (`:2689`) returns a constant.
  - Qualification:
    - Each raw workload calls `renderAllocationCount(host)` before `host.dispose()`
      (`qualification.js:168-169`, `:363-364`, `:470-471`, `:579-580`, `:708-709`).
    - `run.mjs:266-270` asserts 10 rows, all zero.
    - SDK instances are read before close (`sdk-response-entry.ts:154`), and
      `run.mjs:308-320` gates them.
- **n2 (fixed).** "Gate G6 shows that the D7 state flush does not depend on FTZ or DAZ, but the
  full render does, so `fpenv` installs a word with both clear" is true:
  - `g6_ftz_inert.rs:115` sets `FTZ|DAZ`.
  - `fpenv.rs:10-13` records the #144 full-corpus divergence.
  - `0x1F80` has bit 15 and bit 6 clear.
- **n3 (fixed).** `render_lock.rs:88-91` is the `cfg` line through the `static`. `:5-11` is the
  sentence "This module is the runtime proof ..." through "exactly zero".

## Gate 2 re-checks at `3b10ae68c` (beyond the fixes)

- **D-M2 sets, recomputed.** The engine worklet's post-boot methods (`receive*`, `process`,
  `postMeterFrame`, `readSpectrumStreamMetadata`) plus the feed worklet give 33 exports, the same
  as `docs:192-201`. Set 1 minus `render_locked` = {`dispose`, `render_allocation_count`}. The
  `render_locked` set minus set 1 = the eleven accessors.
- **Non-test unsafe sites of web `ffi.rs`.** Before the test module at `:4881` they are exactly
  `:878`, `:894`, `:1805`, `:3929`, `:3936`, `:3939` and `:3982`. `catch_unwind` appears only in
  the header doc (`:3`). The `miso_engine_web_v1_render` body (`:3990-3999`) holds no unsafe.
- **Citations.** Every file:line citation in the section is exact at the commit (each was read):
  - root `Cargo.toml:90`, `:91`, `:102`;
  - `disjoint.rs:208`, `:303`, `:334`, `:317-345` and region `:106-347`;
  - `spsc.rs:381`, `:449` and region `:295-459`;
  - `fpenv.rs:141`, `:148`, `:166`, `:181`, `:322-329`;
  - capi `ffi.rs:807`;
  - web `ffi.rs:14-39`, `:617-626`, `:4881`;
  - feed worklet `:306-328`, `:386`, `:445`, `:539`;
  - `run.mjs:266-270`;
  - `allocation_tracker.rs:802-809`.
- **No reserved MXCSR bit.** Every `write_mxcsr`/`write_fp_control_word` caller writes one of:
  - `0x1F80`;
  - a word read from the register;
  - such a word OR'd or masked with `0x003F`, `0x0020`, `0x0040`, `0x4000`, `0x6000` or `0x8000`
    (`fp_env.rs`, capi `runtime/tests.rs`, `g6_ftz_inert.rs`, host-core `word::{flushing, clear,
    hostile}`, `g6_full_corpus_ftz.rs`).

  None sets bits 16-31.
- **Gate 1.** The allowlist at `scripts/check-realtime-policy.sh:31` has 19 paths, all named in the
  section. The section's only extra `.rs` paths are the three cited tests and short names
  (`softfma.rs`, `fpenv.rs`, `fpenv_extra.rs`, `allocation_tracker.rs`).

## NIT

- n1. `softfma.rs:113-116`: "the other words are a test-only exposure" is true when "exposure" means
  engine computation under a non-default word. In production, the barriers keep the render under
  `0x1F80`, and after the exit write the C entry runs no floating-point code. But the sentence does
  not say that the exit write itself installs a non-default word whenever the caller's word is
  non-default. `fpenv.rs:16-17` says that is every DAW callback. The `core::arch` doc covers that
  case by its letter ("even when the register is altered and later reset to its original value").
  A clearer clause would be: "after the exit write the entry runs no floating-point code, so the
  caller's word reaches no engine computation".
- n2. `docs:292-293` lists the uncounted calls as "the boot exports themselves, the unwrapped boot
  accessors and `dispose`". The list leaves out `render_allocation_count`, which is also post-boot
  and unwrapped. It is not claimed to be exhaustive, and that export makes no allocator call.
- n3. `docs:98` is 103 characters; the rest of the section wraps at 100.

## Open items for root (unchanged, outside this slice)

- Attempt 1's open items stand. (1) `softfma::write_mxcsr` and `fpenv::write_fp_control_word` are
  safe `pub fn`s whose `SAFETY` ground is about what their callers write. (2) The claims at
  `fpenv.rs:73-75` ("no call at all", "two register writes") are false on the shipped `x86_64`
  cdylib. The attempt-2 record files (2) again.

## Test value

No test is added (D-test), which is correct, so there is no mutation run. No queue is touched, so
the acked-batch question does not apply.

## Gates run (exported `3b10ae68c`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1489/target`)

- `cargo build --locked -p lane`: ok (lane recompiled)
- `cargo fmt --all -- --check`: exit 0
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`
- `bash scripts/check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)`
- `bash scripts/check-lane-policy.sh`: `lane policy: ok`
- `bash scripts/check-dsp-research.sh`: `dsp research corpus: ok`
- `bash scripts/check-builtins-listening.sh`: `issue-007 listening preregistrations: ok (human evidence pending)`
- Also run: `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p lane --no-deps` ok;
  `cargo clippy --locked -p lane --all-targets -- -D warnings` ok; `bash scripts/test-realtime-policy.sh` ok.

Evidence: `/tmp/claude-1002/v1489/{gate2.out,allow2.txt,doc2.txt}` plus attempt 1's files.
