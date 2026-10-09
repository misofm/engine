# Make the floating-point control-word writers sound by construction

Stream G follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-09 by root from the #1489 verdicts' open item 1
(`/home/bl/misofm/submix-verdicts/1489-attempt1.md`, "Open items for root", item 1;
`/home/bl/misofm/submix-verdicts/1489-attempt2.md`, "Open items for root") and the batch-misc2
verdict (`/home/bl/misofm/submix-verdicts/batch-misc2-batch-verdict.md`, "Not blockers", #1489's
open item 1).

Root's ruling (2026-10-09), verbatim:

> (3) File both successors: (a) make softfma::write_mxcsr and fpenv::write_fp_control_word sound
> by construction (unsafe fn with a stated contract, or a safe API that accepts only valid words);

Smallest slice: the control-word writers become `unsafe fn` with a `# Safety` contract; the guard
`CanonicalFpEnv` stays the only safe way to change the word; every existing caller states why its
call meets the contract. No change to what any render writes.

## Problem (verified on `codex/d15-batch-misc2` at `a998adf21`)

- **Two safe writers whose soundness depends on the caller.** `lane::softfma::write_mxcsr`
  (`crates/lane/src/softfma.rs:103`, a safe `pub fn` in the public module `softfma`,
  `crates/lane/src/lib.rs:130`) calls `_mm_setcsr(value)` (`:120`). Its `SAFETY` comment
  (`:105-119`) grounds soundness in what callers pass: "No caller writes a word with a reserved bit
  (16-31) set, which would raise #GP", and it says that a non-default word is "outside the language
  model". `lane::fpenv::write_fp_control_word` (`crates/lane/src/fpenv.rs:147`, x86_64: forwards to
  `write_mxcsr` at `:148`; `:175`, AArch64: `msr fpcr` with a `SAFETY` comment that says the value
  "is either `CANONICAL_FPCR` or a word previously read from this same thread's FPCR"; `:195`, every
  other target: a no-op) is a safe `pub fn` in the public module `fpenv` (`lib.rs:123`).
- **Any safe caller can break the premise.** Safe code can pass a word with a reserved bit (a #GP on
  x86_64) or a word with other exception masks, rounding or DAZ/FTZ bits. The `core::arch`
  documentation of `_mm_setcsr` makes such a change immediate undefined behaviour (the #1489
  attempt-1 verdict quotes `core_arch/src/x86/sse.rs:1542-1546`, nightly-2026-08-20 rust-src). The
  verdict: "a real soundness gap, not a nicety" (`1489-attempt1.md`, open item 1).
- **Who calls them today.** The one non-test caller is the guard: `CanonicalFpEnv::enter`
  (`fpenv.rs:336-338`, writes the canonical word) and its `Drop` (`:357-360`, writes the word read
  on entry). The test callers write other words on purpose (hostile caller words, FTZ/DAZ):
  `crates/lane/tests/fp_env.rs` (15 call sites), `crates/lane/tests/g6_ftz_inert.rs:115`, `:117`,
  `crates/capi/src/runtime/tests.rs` (7 call sites, `:3106`-`:3313`),
  `crates/host-core/tests/fp_environment.rs:194`, `:206`, `:278` and
  `tools/wasm-gates/tests/g6_full_corpus_ftz.rs:81`, `:93`. The #1489 attempt-1 verdict lists every
  word they write; none sets bits 16-31.
- **Why "a safe API that accepts only valid words" is the guard.** The only words that can be
  installed without leaving Rust's floating-point model are the default word and the caller's own
  word handed back after a block. A safe API that admits exactly those is `CanonicalFpEnv`, which
  exists. Every other caller writes a non-default word on purpose, so no safe signature can admit
  it.

## Decisions

- **D1. The raw writers are `unsafe fn`.** `softfma::write_mxcsr` and all three
  `fpenv::write_fp_control_word` variants (x86_64, AArch64, and the no-control-word target, so the
  API has one shape on every target) become `pub unsafe fn`. Each carries a `# Safety` section that
  states the contract: (1) on x86_64 the word sets no reserved bit (16-31); on AArch64 it sets no
  `RES0` bit of `FPCR`; (2) a word other than the target's canonical word, or other than a word read
  from this thread that is being handed back, leaves the floating-point environment Rust assumes, so
  the caller lets no compiled floating-point code run under it except the code it deliberately
  measures; (3) the caller restores the previous word before it returns, itself or through a guard
  that does (`CanonicalFpEnv`'s `Drop`). The `SAFETY` comments inside the bodies say only what the
  body itself relies on.
- **D2. The guard is the safe interface.** `CanonicalFpEnv::enter` and its `Drop` call the writers
  inside `unsafe` blocks whose `SAFETY` comments discharge D1: the entry writes the canonical word;
  the exit writes the word read from this thread on entry. `read_mxcsr` and
  `read_fp_control_word` stay safe.
- **D3. Every test caller states its contract.** Each test call site is wrapped in an `unsafe`
  block with a `SAFETY` comment that names the word, says it sets no reserved bit, and names the
  guard or write-back that restores the previous word. A file that gains `unsafe` gains
  `#![allow(unsafe_code)]` and its named entry in the realtime unsafe allowlist (the awk gate's
  exclusions, `scripts/check-realtime-policy.sh:31`, while it is on `main`; `Policy::workspace()`
  in `tools/realtime-policy` after J #1446), in the policy's "Unsafe-code ownership" section
  (`docs/REALTIME_DEPENDENCY_POLICY.md`, one entry per file, as #1489's gate 1 requires), and, for
  `tools/wasm-gates/tests/g6_full_corpus_ftz.rs`, in `scripts/check-bench-policy.sh`'s exact set of
  `#![allow(unsafe_code)]` files under `tools/` (`:220-234`).
- **D4. A compile-fail doctest per writer.** One `compile_fail,E0133` doctest on
  `fpenv::write_fp_control_word` (every target) and one on `softfma::write_mxcsr` (present only
  where the function exists, so the AArch64 and wasm doctest runs do not fail for a missing name):
  each calls the writer from safe code with a word read from this thread.
- **D5. No codegen change.** `unsafe fn` changes no emitted code. The shipped browser module
  (`9ea229e2...` at `a998adf21`) and the C ABI caller audit's `pcm_digest` (`cb10fbface44a3a4`) do
  not change.

## Authorized paths

- `crates/lane/src/softfma.rs` (`write_mxcsr`'s signature, doc, `SAFETY`, and the D4 doctest)
- `crates/lane/src/fpenv.rs` (the three `write_fp_control_word` variants, the `enter` and `Drop`
  call sites, the D4 doctest)
- `crates/lane/tests/fp_env.rs`, `crates/lane/tests/g6_ftz_inert.rs` (the write call sites and the
  `allow` attribute only)
- `crates/capi/src/runtime/tests.rs` (B's; the write call sites only)
- `crates/host-core/tests/fp_environment.rs` (the write call sites and the `allow` attribute only)
- `tools/wasm-gates/tests/g6_full_corpus_ftz.rs` (the write call sites and the `allow` attribute
  only)
- the realtime unsafe allowlist (D3: the awk gate's exclusion line,
  `scripts/check-realtime-policy.sh:31`, while the awk gate is on `main`, and
  `scripts/test-realtime-policy.sh` only if its self-test fails without a matching edit; else
  `Policy::workspace()`'s unsafe rows; J's, by named exception, one row per new file)
- `scripts/check-bench-policy.sh` (J's; the expected `tools/` unsafe set, one path)
- `docs/REALTIME_DEPENDENCY_POLICY.md` ("Unsafe-code ownership" only: the `softfma.rs` and
  `fpenv.rs` entries and one entry per new file)
- this spec

## Non-goals

- Any change to the written words, the guard's order, the barriers or the canonical words.
- Correcting `fpenv.rs:16-17` or `:73-75` (the sibling successor *Prove or correct fpenv's
  realtime-cost and DAW-callback claims against the shipped x86_64 codegen*).
- Removing or rewriting a test's hostile-word scenario.

## Hazards

- `crates/lane/src/fpenv.rs` and `softfma.rs` carry #1446's comment lines (STREAMS hot-file row
  108): keep #1446's wording when rebasing over it, or let #1446 rebase over this.
- The realtime unsafe allowlist moves from the awk gate to `tools/realtime-policy` with J's batch
  (#1446). Land the rows in whichever gate is on `main` when this lands; if both are, both.
- `crates/capi/src/runtime/tests.rs` is `#[cfg(test)]` inside `crates/capi/src`; its
  `#![allow(unsafe_code)]` scope must not reach non-test code (an item-level `allow` on the test
  module, or the existing module attribute if one is there).

## Objective gates

1. **Sound by construction (mutation, recorded; PR evidence).** `cargo test --locked -p lane --doc`
   passes with both D4 doctests. Make `write_fp_control_word` safe again (drop `unsafe` from the
   x86_64 variant's signature and wrap its body): the D4 doctest on it is red ("test compiled
   successfully"). Revert: green. The same for `write_mxcsr`.
2. **Every caller compiles under the contract.** `cargo test --locked -p lane`,
   `cargo test --locked -p capi`, `cargo test --locked -p host-core --test fp_environment` and
   `cargo test --locked -p wasm-gates --test g6_full_corpus_ftz` pass.
3. **Policy gates.** `bash scripts/check-realtime-policy.sh` and its self-test
   (`bash scripts/test-realtime-policy.sh`) or, after #1446, the realtime-policy tool and its
   tests; `bash scripts/check-bench-policy.sh`; `bash scripts/check-lane-policy.sh`;
   `bash scripts/check-workspace-policy.sh` exit 0. The policy section's paths equal the allowlist
   (#1489's gate 1, re-run).
4. **No codegen change.** `scripts/build-web-audioworklet.sh --named-twin` gives the module
   `9ea229e2...` (or, if another slice moved it first, the base's digest); the C ABI caller audit
   reports `pcm_digest` unchanged, 0 allocations and 0 syscalls.
5. **Existing gates.** `cargo clippy --locked --workspace --all-targets -- -D warnings`,
   `cargo fmt --all -- --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p lane --no-deps`,
   `bash scripts/check-cross-targets.sh` exit 0.

*Test value.* Each D4 doctest is red when its writer is made callable from safe code again (the
defect #1489's verdict names); no existing test fails on that change, because every existing
caller still compiles.

## Evidence

- Gate 1's two red/green runs; gate 2-5 outputs; the list of call sites with their `SAFETY` words.

## Dependencies

- After (other streams): none open. J #1446 changes where D3's allowlist rows go, not whether this
  can land.
- After (same stream): none. In any order with the sibling successor on `fpenv.rs` and
  `softfma.rs`; the later slice rebases.

## Standing rules for the implementer

- Work only from this body. Read the cited lines and both #1489 verdicts first.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: half a day.
