# #511 Sol attempt 2 report

## Result

Source correction and proportional local gates PASS. Final reviewed/pushed source head is
`ca8f34d5b1c50012f5e1a1001281c22b8ce3a005`, clean and equal to
`origin/codex/runtime-slot-reservation`. Root owns final artifact/native-Wasm delivery
qualification and the consolidated Astra verdict; those are not claimed here.

Final source SHA256:

- `crates/graph/src/lib.rs`: `9bdac963068795bc8a33f93b18ba5bbe5221ab628bdf739ec8694a919bbbd432`
- `crates/graph/src/runtime.rs`: `b92af988c6a8a602a7f93bf8cd11f4fb62e1f68831acca0eac8e39aaf75c23a2`
- `crates/graph-compiler/src/compile.rs`: `7f435cff8087b7e66dc74b57873cd343ff178fe8f849b5c9cf61e14c8513ff13`
- `crates/graph-compiler/src/lib.rs`: `fc4a43306b56b0ddb6d4f85ff86ee926fc14b98a9543b18e5e420b9218f444d8`
- `crates/builtins-compiler/src/lib.rs`: `6642d7fa25ca317e133b1705fe62fb6148f0e6894e28a51ba48353e152021ebc`
- `crates/builtins-compiler/tests/allocation_tracker.rs`: `840d020fc71ce11721a0009852b0f2b0a172510597f58f7750d57d2396bd3ae2`

## Corrections made

- The existing builtin estimate fold now runs immediately after successful builtin preflight,
  before new fallible combined-count/mask/slot arithmetic, preserving the earlier builtin
  arithmetic diagnostic priority.
- An empty builtin-bank kind reports `maximum_mask_bytes == 0`; maximum width remains aggregated
  only from populated kinds.
- The feature-gated runtime seam records fixed aggregate construction facts at the actual
  `RuntimeParts::chain_for`/`bank_chain` path. It is a thread-local fixed `Cell`, with no registry,
  callback, allocation, or production diagnostic surface.
- The existing builtins fixture exposes its pre-existing direct-console non-pairing variant. The
  physical test keeps this direct-attachment evidence distinct from real compiler admission.
- The physical allocator fixture now seeds and checks known pre-existing ownership, flags checked
  byte-counter underflow/overflow, removes the unused/incorrect peak claim, enumerates exact
  conversion and release layout counts, and tests whole-array largest and named coexistence.
- The compiler fixture independently tests all four fold fields, old-largest below/above L, zero,
  each additive overflow with whole-estimate rollback, empty width, mixed real effect+builtin
  counts, exact direct-arithmetic publication/attachment, cap/minus-one, returned live effects,
  sealed builtins, and dispatch-independent canonical semantics.
- The four authorized existing numeric fixtures derive C from target-native `BankStage` pointer,
  `BankSlot`, bool and selected-width layouts and add it only to their approved total equations.

No Cargo, rack production, benchmark, timing, generated fixture, pin, wire/schema, Git, or GitHub
mutation was performed by this agent. Root made the two required checkpoint commits/pushes.

## Actual construction and independent physical arithmetic

The final feature-gated fixture facts were read from the compiled final-source test-support rlibs
by `/tmp/511-sol2-facts.*` (status 0):

- paired delivery: charged/prepared N = 6, aggregate R = 6, aggregate S = 5,
  max per-chain R = 3, max per-chain S = 2; therefore S < R and aggregate R <= N;
- unpaired delivery: N = 6, aggregate R = 6, aggregate S = 6,
  max per-chain R = 3, max per-chain S = 3; therefore the actual unpaired chain has S = 3 > 1,
  S <= R, and aggregate R <= N.

On this native target `F=16`, `B=32`, `W=8`. The isolated three-slot production conversion
independently derives `C=3*(16+3*32+3*8)=408` and `L=max(48,96,8)=96`.
It observes exactly one 96-byte slot-array allocation, three 8-byte cloned-mask allocations, and
one 48-byte incoming-stage-vector release. The named conversion coexistence is
`48+96+24=168 <= 408`; each request is <= 96. Retained slot/mask ownership is
`96+4*8=128 <= 3*(32+2*8)=144`. Off-render destruction observes exactly one 96-byte slot array,
four 8-byte masks (three slot clones plus original chain mask), and two 32-byte pre-existing
scratch-plane releases, with seven frees, zero allocations, checked live balance returning to zero,
and no underflow/overflow flag.

The compiler oracle verifies `size_of::<bool>() == 1`, N=0 with no width, populated scalar
rejection, representative W4/W8 and multi-N C/L arithmetic, calculation overflow, direct checked
fold behavior for every affected field, and rollback for graph/plan/session addition overflow.
The real mixed fixture independently combines nonzero effect-bank and builtin-bank counts,
derives the combined C/L and delta from native layouts, and compares the complete published and
attached estimates against direct field arithmetic. Effect bank payload/scratch/metadata and
canonical semantics are checked unchanged. Exact whole-plan graph/plan/largest caps accept; each
minus-one cap rejects while returning eight live effect entries and valid sealed builtins.

## Gate results and raw records

- Frozen graph exact: debug 1 passed/0 failed at
  `/tmp/511-sol2-focused-graph-debug4.*`; release 1/0 at
  `/tmp/511-sol2-focused-graph-release.*`.
- Frozen physical exact: debug 1/0 at `/tmp/511-sol2-focused-physical-debug.*`; release 1/0 at
  `/tmp/511-sol2-focused-physical-release.*`. After the comment/loop-only lint correction, focused
  debug remained 1/0 at `/tmp/511-sol2-correction-physical-debug.*`.
- Full graph-compiler lib: debug 64/0 at `/tmp/511-sol2-full-graph-debug.*`; release 64/0 at
  `/tmp/511-sol2-full-graph-release.*`. All four previously failing numeric fixtures passed and
  their later assertions executed.
- Full builtins allocation tracker: debug 7/0 at `/tmp/511-sol2-full-physical-debug.*`; release 7/0
  at `/tmp/511-sol2-full-physical-release.*`.
- Strict all-targets Clippy status 0: graph normal
  `/tmp/511-sol2-clippy-graph-normal.*`; graph test-support
  `/tmp/511-sol2-clippy-graph-test-support.*`; graph-compiler normal
  `/tmp/511-sol2-clippy-graph-compiler-normal.*`; builtins-compiler normal
  `/tmp/511-sol2-clippy-builtins-normal.*`; builtins-compiler with
  `test-support,graph/test-support` `/tmp/511-sol2-clippy-test-support2.*`.
  Existing `clippy.toml` unreachable-function configuration warnings appeared in dependency output
  but did not violate `-D warnings`; every scoped command exited 0.
- Format check status 0: `/tmp/511-sol2-fmt-check.*`.
- Diff check from attempt-2 base `a6e6429b...` status 0: `/tmp/511-sol2-diff-check.*`.
- Graph policy PASS: `/tmp/511-sol2-policy-graph2.*`.
- Realtime policy PASS, 42 marked regions in 12 files: `/tmp/511-sol2-policy-realtime.*`.
- Workspace policy PASS: `/tmp/511-sol2-policy-workspace.*`.

Each successful gate record contains cwd, exact argv, immutable HEAD or dirty source SHA256,
`git status --short`, and only the effective PATH addition (`/home/bl/.cargo/bin:$PATH`). Cargo
commands use `--locked`. Raw stdout, stderr and numeric status are separate files.

## Preserved failures and evidence limits

- `/tmp/511-sol2-focused-graph-debug.*`: initial builtins adapter could not reference graph's
  feature-gated seam under the frozen graph command. Corrected without Cargo changes by making the
  builtins adapter select only the existing paired/unpaired variant and reading graph facts in the
  explicitly enabled physical test.
- `/tmp/511-sol2-focused-graph-debug2.*`: two `Default` uses and field/method confusion in the new
  test failed compilation; corrected locally.
- `/tmp/511-sol2-focused-graph-debug3.*`: an invalid full-canonical comparison between
  effect-only and builtin-attached binding surfaces failed. Replaced with the same mixed session
  compiled under vector/scalar dispatch, the established target-independent semantic comparison.
- `/tmp/511-sol2-clippy-test-support.*`: three missing controlled-unsafe safety comments and one
  single-element-loop lint; corrected mechanically in the test fixture and requalified.
- `/tmp/511-sol2-clippy-normal.*`: a combined multi-package command caused Cargo feature
  unification to enable `builtins-compiler/test-support` without `graph/test-support`; it is an
  operational command-composition failure. Separate normal package commands and the explicit
  enabled command all pass. It is not represented as a successful qualification command.
- `/tmp/511-sol2-policy-graph.*`: direct execution returned 126 because the checked-in script lacks
  an executable bit; `bash scripts/check-graph-policy.sh` passed and is the effective gate.
- One early `cargo fmt --all -- --check` preflight was invoked before applying the required PATH
  prefix and failed immediately with `cargo: command not found`. It produced no source change and
  was not captured into raw files. This limitation is stated rather than retroactively
  reconstructing raw evidence; the final correctly captured immutable-head format gate passed.

No source gate remains failing. Final native/Wasm immutable-delivery/artifact evidence and the
single consolidated Astra attempt-2 verdict remain with root.
