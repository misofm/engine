# Narrow the universal DAW-callback FTZ/DAZ claim in host-core and the C ABI tests

Stream G follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-09 by root from the #1495 attempt-1 verdict's m1
(`docs/handoffs/decision-15-2026-10-05/verdicts/batch-fp/1495-attempt1.md`, "Open items for root",
item 2) and #1495's attempt record ("Corrections after the attempt-1 verdict").

Root's ruling (2026-10-09), in substance: correct the universal DAW-callback FTZ/DAZ claim in the
three live copies outside #1495's authorized paths, with the same narrowing #1495 D3 applied to
`crates/lane/src/fpenv.rs:16-17`. Comment lines only; a small slice; gates like #1495's.

Smallest slice: three comment passages stop claiming that every DAW audio callback arrives with
FTZ and DAZ set. No code, test logic or emitted byte changes.

## Problem (verified on `codex/d15-batch-fp` at `abbbc65b0`)

- **#1495 narrowed the claim in `fpenv.rs`.** "every DAW audio callback arrives with FTZ and DAZ
  already set" had no primary source, so #1495 D3 narrowed it (`fpenv.rs:16-17`, commit
  `f7a625d92`): "the native hosts, whose audio thread may run with FTZ and DAZ set (a host can set
  them to avoid denormal stalls; the C ABI contract does not forbid it)". The C ABI header
  (`crates/capi/include/miso_engine_v1.h`) states no floating-point environment requirement, and
  `miso_engine_v1_render_f32_planar`'s doc (`crates/capi/src/ffi.rs:795-799`) says the caller's
  environment is borrowed and restored (#1495's attempt record).
- **Three live copies still make the universal claim** (the #1495 verdict's m1; line numbers
  re-read at `abbbc65b0`):
  - `crates/host-core/src/render_session.rs:8-9` (module doc): "and every DAW audio callback
    arrives with FTZ+DAZ set."
  - `crates/host-core/src/lib.rs:78` (crate doc, the #146 bullet): "Every DAW audio callback
    arrives with hardware FTZ and DAZ set, and issue #144 measured what that does".
  - `crates/capi/src/runtime/tests.rs:3080` (doc of the #146 C ABI test): "Every DAW audio
    callback arrives with FTZ and DAZ already set, so this is the shape of the real exposure."
- **Not in question.** `fpenv.rs:33` ("Refusing a DAW's callback thread") and `ffi.rs:798` ("A DAW
  audio callback that arrives with FTZ and DAZ set") are conditional, not universal (#1495's
  attempt record, "Open items"). The #144 measurement (69-70 of 331 corpus rows off-pin under
  FTZ+DAZ) is not in question; only the "every DAW" premise is.

## Decisions

- **D1. Narrow, as #1495 D3 did.** Each of the three passages keeps a universal claim only with a
  primary source for it. Otherwise it says what the engine needs: a native host's audio thread may
  run with FTZ and DAZ set (for example, to avoid denormal stalls), and the C ABI contract does not
  forbid it. Each passage may cite `crates/lane/src/fpenv.rs:16-17` (or the line it is on at the
  slice's head) instead of repeating the reason.
- **D2. Comment lines only; keep line counts.** Each edited passage keeps its line count, so no
  line citation of these files elsewhere moves. If a passage cannot keep its count, the slice
  re-reads and refreshes every citation of the moved lines in the same commit.
- **D3. The test's sense stays.** `runtime/tests.rs:3080` still says why the FTZ+DAZ arm is the
  exposure that matters; only the "every DAW" premise changes.

## Authorized paths

- `crates/host-core/src/render_session.rs` (the module doc lines `:8-9` only; no stream lists this
  file; by named exception)
- `crates/host-core/src/lib.rs` (the crate doc line `:78` and its sentence only; no stream lists
  this file; by named exception; it is on #1446's comment-lines hot-file row)
- `crates/capi/src/runtime/tests.rs` (B's; the doc comment `:3080-3081` only)
- this spec

## Non-goals

- Any code, test logic or assertion change.
- `fpenv.rs` and `ffi.rs` (done by #1495, or conditional and true).
- Rewording the #144 or #146 measurements.

## Hazards

- `crates/host-core/src/lib.rs` carries #1446's comment lines (STREAMS hot-file row): keep
  #1446's wording when rebasing over it. H #1492 also edits `lib.rs` (the re-export only).
- `crates/capi/src/runtime/tests.rs` is also edited by #1494 (the write call sites): the later
  slice rebases.

## Objective gates

1. **Every edited sentence is true (verifier).** The verifier reads each edited passage at the
   slice's head and checks each sentence against `fpenv.rs:10-17`, the header and `ffi.rs:795-799`,
   or against a cited primary source. A universal claim without a primary source fails the
   attempt.
2. **No codegen change.** The C ABI caller audit (`./target/release/audit capi`, after
   `cargo build --locked --release -p audit -p bench -p capi -p session-validator`) reports
   the same `pcm_digest` as on the base commit, 0 allocations, 0 syscalls; the browser
   module digest equals the base's (`scripts/build-web-audioworklet.sh --named-twin`).
3. **Existing gates.** `cargo fmt --all -- --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked
   -p host-core --no-deps`, `cargo test --locked -p capi` (the doc comment is on a test; it must
   still build), `bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`
   (or the realtime-policy tool after #1446) exit 0.

*Test value.* No test is added: the defect is false prose, and a test that greps prose is refused;
gate 1's verifier read is the check, so there is no mutation run.

## Evidence

- The three passages before and after; gate 2's audit line and module digest.

## Dependencies

- After (same stream): #1495 (its `fpenv.rs:16-17` wording is the one these passages cite).
- After (other streams): none. In any order with J #1446, H #1492 and G #1494 on the shared files;
  the later slice rebases.

## Standing rules for the implementer

- Work only from this body. Read the cited lines and #1495's spec and verdict first.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: one hour.

## Attempt record

### Attempt 1 (implementer, 2026-10-11, head `7d5362dc2`+`b3ddc8d86` base, branch `codex/d15-batch-dfg`)

Anchors re-read on the branch. `render_session.rs:8-9` and `lib.rs:78` were where the spec says. The
test's doc comment had moved to `crates/capi/src/runtime/tests.rs:3091-3092` (the spec's `:3080-3081`
is stale; same text). `ffi.rs` moved the conditional sentence to `:843`; still conditional, untouched.
A repository search for other "every DAW"/"DAW audio callback" claims found no further copy.

Before and after (each passage keeps its line count; every line is at most 99 columns):

- `render_session.rs:8-9`: "... and every DAW audio callback arrives / with FTZ+DAZ set." became
  "... and a native host's audio thread may / run with FTZ+DAZ set (see `crates/lane/src/fpenv.rs`)."
- `lib.rs:78`: "Every DAW audio callback arrives with hardware FTZ and DAZ set, and issue #144" became
  "A native audio thread may run with hardware FTZ and DAZ set, and issue #144" (`lib.rs:79` and #1446's
  wording untouched).
- `runtime/tests.rs:3091`: "Every DAW audio callback arrives with FTZ and DAZ already set, so this is
  the shape of the real" became "A native host's audio thread may run with FTZ and DAZ set, so this is the
  shape of the real" (line 3092 and the test's sense unchanged, D3).

Truth check: each new sentence is the `fpenv.rs:16-17` claim ("native hosts, whose audio thread may run
with FTZ and DAZ set"); no universal claim remains.

Gates (all at the edited tree):

- `cargo fmt --all -- --check`: ok.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --no-deps` (the CI form): ok. The spec's
  `-p host-core` form fails identically on the base without my edit, at `lib.rs:19`: the intra-doc
  link `SessionControlProvider` is behind the `control-provider` feature, which only a workspace build
  enables. Pre-existing; the workspace form is the gate CI runs (`qualification.yml:475`).
- `cargo test --locked -p capi`: all suites pass (81, 3, 11).
- `cargo clippy --locked -p host-core -p capi --all-targets -- -D warnings`: clean.
- `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok (93 marked regions).
- No codegen change: `./target/release/audit capi` gives the same `pcm_digest` as on the base commit, 0 allocations,
  0 deallocations, 0 syscalls, 0 violations. Browser: `build-web-audioworklet.sh --module-only` before
  and after the edit gives the same shipped module `429788e6...3be3` (3129218 B) and the same named
  twin `e44d96c7...50b8`.
- No test added or changed in logic, so no mutation run (spec test-value note).
