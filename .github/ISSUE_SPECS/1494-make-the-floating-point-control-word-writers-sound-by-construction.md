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

## Attempt record

### Attempt 1 (implementer, 2026-10-09)

Branch `codex/d15-batch-fp`, base `b81eb1fb0`.

**Reconciliations with root's binding additions.**

- **D4 fence.** Per #1422 D2 and its Amendment 1 (which supersede this body's
  `compile_fail,E0133`), each D4 fence is plain `compile_fail` without an error code, and each has
  a passing twin that is identical except that the call sits in an `unsafe { }` block with a
  `// SAFETY:` comment (the word was just read from this thread, so writing it back changes
  nothing). The comment beside each fence names the code rustc reports today and says stable
  rustdoc does not check it. Read by flipping both fences to plain doctests once: each failed with
  exactly one error, `error[E0133]: call to unsafe function `write_mxcsr` / `write_fp_control_word`
  is unsafe and requires unsafe block`, and no other.
- **Where the doctests exist.** `write_mxcsr` is `cfg(target_arch = "x86_64")`, and rustdoc
  collects no doctest from an item whose `cfg` is off, so its pair exists only where the function
  does. The `write_fp_control_word` pair is on each of the three variants. On this x86_64 host the
  doctest run lists only the x86_64 variant's pair (`fpenv.rs` lines 169/177), and neither the AArch64 variant's pair nor the wasm-only fences (the no-op variant's pair,
  the portable `CanonicalFpEnv`'s two) appear, which shows the cfg gating. The AArch64 pair runs
  in CI's `aarch64-debug` doctest leg; it is not emulated here. The no-op variant's pair carries the
  same "documentation only" note as the portable guard's fences.
- **No parallel API.** No safe wrapper was added; `CanonicalFpEnv` stays the only safe way to
  change the word.
- **Allowlist.** #1446 has not landed, so the five new rows are in the awk gate's exclusion line
  (`scripts/check-realtime-policy.sh:31`). `scripts/test-realtime-policy.sh` passed without an
  edit.
- **#1495's lines.** `fpenv.rs`'s "every DAW audio callback arrives with FTZ and DAZ already set"
  (`:16-17`) and the "Realtime properties" paragraph are unchanged. The paragraph moved from
  `:73-75` to `:77-79` because the "Why this file carries `unsafe`" section above it gained four
  lines (next bullet); #1495 rebases.

**Edits outside the listed scope, each needed to keep a statement or gate true (root to accept or
drop).**

- `crates/lane/src/fpenv.rs` module doc, "Why this file carries `unsafe`": one paragraph says the
  raw writer is now an `unsafe fn` and the guard calls it in `unsafe` blocks. Without it the
  section's "one file, two reasons" no longer covers all of the file's unsafe.
- `crates/lane/tests/g6_ftz_inert.rs:10-12` (module doc): it said the workspace "forbids" unsafe
  "everywhere else" than `lane`, which is false once this file calls the writer. Now it says the
  writer is an `unsafe fn`, these two calls are the file's only unsafe code, and the file is on the
  allowlist.
- `scripts/check-bench-policy.sh`: besides the one path, the comment's counts change ("these four
  files", "A fifth file") and one sentence names #1494's entry.
- `scripts/test-bench-policy.sh` (not listed in Authorized paths): its `unsafe-owner-grep-error`
  case pins the expected `tools/` set and its four `count-*` cases pin the count `3`. With
  `g6_full_corpus_ftz.rs` added to the set, the self-test exited 96 ("incomplete selected grep
  payload", the actual set having the fourth path). The edit appends the path to the expected set
  and changes `3` to `4` in those four diagnostics; nothing else. It is a separate commit so root
  can drop it.

**D1/D2 (the writers and the guard).**

- `crates/lane/src/softfma.rs`: `pub unsafe fn write_mxcsr`. `# Safety`: no reserved bit
  (16-31, #GP); a word other than `CANONICAL_MXCSR` or a word read from this thread being handed
  back leaves the environment Rust assumes (the `core::arch` doc of `_mm_setcsr`), so the caller
  runs no compiled floating-point code under it except the code it deliberately measures; the
  caller restores the previous word, itself or through `CanonicalFpEnv`'s `Drop`. Body `SAFETY`:
  SSE (every x86_64 host, and the crate's compile guard), and the caller upholds the contract.
  `read_mxcsr` stays safe.
- `crates/lane/src/fpenv.rs`: the three `write_fp_control_word` variants are `pub unsafe fn` with
  the same contract (AArch64: no `RES0` bit of FPCR). x86_64 body (`:187`): "this function's
  `# Safety` contract is `write_mxcsr`'s, and the caller upholds it". AArch64 body: `MSR FPCR`
  is unprivileged, thread-local and cannot trap at EL0, and the caller upholds the contract. The
  no-op variant is `unsafe` so the API has one shape on every target. `read_fp_control_word` stays
  safe.
- Guard call sites: `enter` (`:449`): "the canonical word sets no reserved bit and is the
  environment Rust assumes, so compiled code may run under it; this guard's `Drop` writes `saved`
  back". `Drop` (`:473`): "`enter` read `self.saved` from this thread (the guard is `!Send`, so
  this is the same thread), so it sets no reserved bit, and writing it hands the caller's own word
  back, which restores the word `enter` replaced".

**D3 (test call sites, 29 in 5 files; each `unsafe` block has a `SAFETY` comment that names the
word, says it sets no reserved or `RES0` bit, and names the restore).**

| File (allow) | Lines | Word named | Restore named |
|---|---|---|---|
| `crates/lane/tests/fp_env.rs` (`#![allow]`, `:16`) | x86: `:70` | `self.0`, read from this thread | is the write-back |
| | `:88`, `:118`, `:140`, `:221` | `hostile_word`: FTZ, DAZ, RC, a status flag, all below bit 16 | `_restore` |
| | `:184` | `0x1F80 \| 0x003F` | `_restore` |
| | `:199`, `:206` | `0x1F80 \| FTZ` (bit 15), `0x1F80 \| DAZ` (bit 6) | `_restore` |
| | AArch64: `:255` | `self.0`, read from this thread | is the write-back |
| | `:272`, `:302`, `:324`, `:372` | `hostile_word`: FZ, DN, RMode (defined fields) | `_restore` |
| | `:356`, `:361` | `CANONICAL_FPCR` (0); `0 \| bit` for FZ, DN, an RMode value | `_restore` |
| `crates/lane/tests/g6_ftz_inert.rs` (`#![allow]`, `:18`) | `:121`, `:125` | `saved \| FLUSH_BITS` (FTZ\|DAZ or FZ); `saved` | the write-back at `:125` |
| `crates/capi/src/runtime/tests.rs` (item `#[allow]` on the x86_64 `mod fp_environment`, `:3092`, inside the `#[cfg(test)]` module, so it reaches no non-test code) | `:3109` | `self.0` | is the write-back |
| | `:3140`, `:3190` | `caller`: a read word with only FTZ/DAZ set or cleared (every caller of the two helpers) | write-back of `saved` and `_restore` |
| | `:3175`, `:3220`, `:3331` | `saved`, read at the top of the function | is the write-back |
| | `:3288` | `hostile`: FTZ, DAZ, RC, a status flag | write-back and `_restore` |
| `crates/host-core/tests/fp_environment.rs` (`#![allow]`, `:21`) | `:198` | `self.0` | is the write-back |
| | `:214` | `word::flushing` / `word::clear` (FTZ, DAZ, RC; or FZ, RMode) | `_restore` |
| | `:290` | `word::hostile` | `_restore` |
| `tools/wasm-gates/tests/g6_full_corpus_ftz.rs` (`#![allow]`, `:31`) | `:87` | every caller passes a read word with only `FLUSH_BITS` set or cleared | `WordGuard`'s `Drop` |
| | `:101` | `self.saved`, read by `set` | is the write-back |

Allowlist rows (one per new file): the awk gate's exclusion line, `scripts/check-bench-policy.sh`'s
`tools/` set (`g6_full_corpus_ftz.rs`), and `docs/REALTIME_DEPENDENCY_POLICY.md`: the `softfma.rs`
and `fpenv.rs` entries are rewritten for #1494 (the old entry quoted the `write_mxcsr` `SAFETY`
comment, which D1 replaced, so the quote went with it; citations moved `:141`/`:148` ->
`:145`/`:187`, `:166`/`:181` -> `:205`/`:254`, `:322-329` -> `:431-438`), a new subsection
"Control-word test writers" has one entry per new file, and "Render-path reachability" says these
files write the word on a test thread between render calls, never inside one.

**Gate 1 (mutation, `cargo test --locked -p lane --doc`, run on the final tree; each reverted from
a byte copy, then the baseline re-run green: 3 + 4 doctests ok).**

| Mutation | Red | Others |
|---|---|---|
| (a) x86_64 `write_fp_control_word` safe again (`pub unsafe fn` -> `pub fn`; body already wrapped) | `fpenv::write_fp_control_word (line 169) - compile fail ... FAILED`, exit 101 | both twins and the `write_mxcsr` fence ok |
| (a) `write_mxcsr` safe again | `softfma::write_mxcsr (line 118) - compile fail ... FAILED`, exit 101 | others ok |
| (b) shared path typo `read_fp_control_wrd` in the x86_64 pair | twin `fpenv::write_fp_control_word (line 177) ... FAILED`, exit 101 | the fence stayed green (the wrong-reason defect the twin catches) |
| (b) shared path typo `read_mxcsrr` in the `write_mxcsr` pair | twin `softfma::write_mxcsr (line 126) ... FAILED`, exit 101 | the fence stayed green |

*Test value, checked.* Under mutation (a) on `write_mxcsr`, `cargo test --locked -p lane --tests`
passes: every caller still compiles, with nine `unused_unsafe` warnings. So the D4 doctest is the
only *test* that turns red, as the body says. The warnings mean that `cargo clippy -D warnings`
also fails on that mutation while callers keep their `unsafe` blocks; the doctest still turns red
when the callers' blocks are removed too, which no lint sees.

**Gate 2.** `cargo test --locked -p lane` (all targets and doctests) exit 0; `cargo test --locked
-p capi` exit 0 (71 + 11 + 2 + 0 tests); `cargo test --locked -p host-core --test fp_environment`
3 passed; `cargo test --locked -p wasm-gates --test g6_full_corpus_ftz` 2 passed (44.6 s, debug).

**Gate 3.** `check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)`;
`test-realtime-policy.sh`: `realtime policy mutation tests: ok` (no edit needed);
`check-bench-policy.sh`: `bench policy: ok (... 4 unsafe owners ...)`; `test-bench-policy.sh`:
`bench policy mutations: ok` (after the edit above; exit 96 before it); `check-lane-policy.sh`:
`lane policy: ok`; `check-workspace-policy.sh`: `workspace policy: ok`. The allowlist's 24 paths
equal the `.rs` paths named in "Unsafe-code ownership" before "Render-path reachability" (sorted
diff empty).

**Gate 4.** `scripts/build-web-audioworklet.sh --named-twin`: shipped module
`9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939` (unchanged), named twin
`3d6c0068...`. `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
then `./target/release/audit capi` (qualification.yml's invocation): `pcm_digest`
`cb10fbface44a3a4` (unchanged), `allocations` 0, `syscalls` 0, `total_violations` 0.

**Gate 5 and extras.** `cargo clippy --locked --workspace --all-targets -- -D warnings` exit 0;
`cargo fmt --all -- --check` exit 0; `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p lane
--no-deps` exit 0, and the lint job's `--workspace` form exit 0; `bash scripts/check-cross-targets.sh`
`cross-target matrix: PASS` (AArch64 iOS/Android crates checked and linted, wasm simd128);
qualification.yml's "DSP crates doctests" command exit 0 (lane: 3 plain + 4 `compile_fail`) and
"Workspace doctests" command exit 0. AArch64 tests and doctests run only in CI.

**Open items.** Root to accept or drop the `scripts/test-bench-policy.sh` commit (unlisted path).

### Follow-ups after the attempt-1 verdict (PASS)

Doc and comment lines only; no signature, code or doctest changed. Each change keeps D1's
three-point structure.

- **m1 (bullet 2).** The `# Safety` text of `softfma::write_mxcsr` and of the three
  `write_fp_control_word` variants now says: only the canonical word, or an `x86_64` word that
  differs from it only in the MXCSR status flags (bits 0-5, which the `core::arch` documentation of
  `_mm_setcsr` leaves out of Rust's assumption), is inside the environment Rust assumes, and the
  caller treats every other word as outside it; a word read from this thread and handed back can be
  outside it, because a host can run with FTZ/DAZ (AArch64: `FZ`) set; a hand-back write meets the
  confinement rule when it returns the thread to the word its caller already ran under and the
  writer runs no floating-point code after it. The stricter ground "no engine floating-point code
  runs under the handed-back word" is not true for every guard user: the guard users in
  `crates/builtins/src/tail.rs` (`fixed_input_walk`, on the builtins-preparation path, and
  `live_bound` and `live_composition`, which only the gate and the tests reach, through
  `input_section_live_bound` and `input_section_live_cascade`; preparation reads the `const fn`
  `input_section_live_bound_table`) keep computing on their thread after the guard drops, under the
  host's word, as they did before `enter`. So the text claims only what is true: the hand-back puts
  no code under a word the caller was not already under.
- **m2 (bullet 3).** The restore duty now applies only to a write that neither installs a word
  inside the environment nor hands back a read word; a hand-back write is itself the restore, and
  a word inside the environment needs no restore. `enter`'s `SAFETY` comment now says why the guard
  is sound when safe code passes it to `mem::forget`: the canonical word stays installed, which
  loses the caller's word but leaves no code outside Rust's model. `Drop`'s `SAFETY` comment now
  states the hand-back ground (the caller's own prior word, and no floating-point code in the guard
  after the write; the barrier is before it). The writers' body `SAFETY` comments say "restores
  the previous word where the contract requires it".
- **m3.** `docs/REALTIME_DEPENDENCY_POLICY.md`, "Control-word test writers": "measure the guard
  under it" became "measure the engine under it: the guard, or in `g6_ftz_inert.rs` the D7 flush
  arms, which run with no guard".
- **n4.** The policy's two doctest sentences no longer say "proves" for a target where the pair
  never runs: the `write_mxcsr` pair "checks, on each `x86_64` doctest run"; for
  `write_fp_control_word`, the `x86_64` pair runs in every `x86_64` doctest run, the AArch64 pair
  in CI's `aarch64-debug` leg, and the no-op variant's pair is documentation only.
- **n5.** "Unsafe code must not leak through a public API" is in the "Unsafe-code ownership"
  section, so it now adds: a `pub unsafe fn` whose `# Safety` section states the caller's contract
  is not a leak, because the compiler makes every caller take the contract on in an `unsafe` block
  of its own.
- **The policy's contract quote** (the `softfma.rs` entry) follows the new bullets 2 and 3.
- **Line citations refreshed** in the policy (the edit added lines above them): `fpenv.rs` `:187`
  -> `:195` (the forwarder's `write_mxcsr` call), `:205` -> `:213` and `:254` -> `:271` (the AArch64
  `unsafe` blocks), `:431-438` -> `:456-463` (`scheduling_barrier`), `:449` -> `:477` and `:473` ->
  `:502` (the guard's two writes); `:145` did not move. The attempt-1 record above cites the
  attempt-1 head; at this head the doctests are `softfma.rs` `:124`/`:132` and `fpenv.rs`
  `:177`/`:185` (x86_64), `:248`/`:256` (AArch64), `:317`/`:322` (no-op).

**Notes on the attempt-1 record (verdict findings recorded, body unchanged).**

- **n2.** One more edit outside the listed scope, missing from the list above: a module-doc
  paragraph in `crates/lane/tests/fp_env.rs` (`:12-14`). It is true; root to accept or drop it.
- **m4.** D1 removed `softfma.rs`'s citation of `fpenv.rs:16-17` (base `softfma.rs:117`, in the
  `write_mxcsr` `SAFETY` comment), because the new `SAFETY` comment says only what the body relies
  on. The line "#1495's lines ... are unchanged" above is true for `fpenv.rs` only. #1495's D3
  clause and authorized path for that `softfma.rs` citation now have nothing to act on (#1495's
  attempt-1 record says so); amending #1495's body is root's.
- **m5.** The body's *Test value* sentence is true as worded for the tests, but not the whole
  story: the signature-only revert (writer made safe, callers keep their `unsafe` blocks) is also
  caught by `cargo clippy -D warnings` (`unused_unsafe` on lane's own forwarder block) and, for
  the AArch64 pair, by the AArch64 legs' `RUSTFLAGS: -D warnings` build. The defect that only the
  doctest catches is the full revert: the writer made safe and the callers' `unsafe` blocks
  removed (the verifier ran it: `cargo clippy -p lane -p capi --all-targets -- -D warnings` exits
  0 and only the `write_mxcsr` fence turns red). The body sentence is root's text and is left as
  it is.

**Gates (this follow-up).** `cargo fmt --all -- --check` exit 0; `RUSTDOCFLAGS="-D warnings" cargo
doc --locked -p lane --no-deps` exit 0; `cargo test --locked -p lane --doc`: 3 plain and 4
`compile_fail`, all ok; `cargo clippy --locked -p lane --all-targets -- -D warnings` exit 0;
`check-workspace-policy.sh`, `check-realtime-policy.sh` (89 marked regions in 25 files),
`check-lane-policy.sh`, `check-bench-policy.sh` (4 unsafe owners) and `test-bench-policy.sh`: ok.

**Left for root.** n3 (`scripts/check-realtime-policy.sh:21-23` lists an incomplete set of
`fpenv.rs` unsafe sites), n6 (G6's write-back is not a guard), the `scripts/test-bench-policy.sh`
commit (n1), the n2 paragraph, the m4 amendment of #1495, and the m5 wording of the *Test value*
sentence.
