PASS

# #1498 attempt 1 verdict (adversarial verifier, 2026-10-11)

Commit `c393bf892` (parent `b3ddc8d86`), branch `codex/d15-batch-dfg`. Reviewed `git diff b3ddc8d86 c393bf892`
on an exported tree (`git archive`), never in the worktree.

## Scope check

- The diff touches exactly four files: the three authorized comment passages and this spec (attempt record
  appended). Every changed source line is a `//!` or `///` doc line. No code, test logic or assertion moved.
- Line counts kept: `render_session.rs:8-9` 2 -> 2, `lib.rs:78` 1 -> 1, `runtime/tests.rs:3091` 1 -> 1
  (the doc comment's second line `:3092` is unchanged). No line citation of these files moves.
- The test doc comment is at `crates/capi/src/runtime/tests.rs:3091-3092` at the base, not the spec's `:3080-3081`
  (same text; the file grew). The implementer edited the right passage and recorded the drift.
- #1446 interaction (STREAMS.md hot-file row, `:116`): #1446 is not on this branch (no J branch touches the three
  files), and #1446's own cited line is `lib.rs:75` (the `scripts/check-realtime-policy.sh` line), which this commit
  does not touch; line counts are kept, so #1446's cites stay valid in either landing order. The capi
  `runtime/tests.rs` row (`:117`, #1494 vs #1498) is respected: only the doc comment changed.

## Truth check (gate 1)

Against `crates/lane/src/fpenv.rs:16-17` at the commit ("the native hosts, whose audio thread may run with FTZ and
DAZ set (a host can set them to avoid denormal stalls; the C ABI contract does not forbid it)"):

- `render_session.rs:8-9`: "... 69-70 of 331 corpus rows render off-pin, and a native host's audio thread may run
  with FTZ+DAZ set (see `crates/lane/src/fpenv.rs`)." True; the #144 measurement is unchanged; the cite resolves to
  the module doc that carries the reason.
- `lib.rs:78`: "A native audio thread may run with hardware FTZ and DAZ set, and issue #144 measured what that
  does". True (permissive, not universal). Browser Wasm is excluded by the bullet's own last sentence.
- `runtime/tests.rs:3091`: "A native host's audio thread may run with FTZ and DAZ set, so this is the shape of the
  real exposure." True, and D3 holds: the sentence still says why the FTZ+DAZ arm is the exposure that matters.
- Repository search at the commit (`git grep` over every tracked file for "every DAW/host/native ... FTZ/DAZ",
  "DAW ... arrive/callback ... FTZ/DAZ", unconditional "host/callback/thread sets FTZ") finds no remaining universal
  claim outside quotations in issue specs and handoff verdicts (historical records). The two remaining DAW
  mentions are conditional and true: `crates/capi/src/ffi.rs:843` ("A DAW audio callback that arrives with FTZ and
  DAZ set ...") and `crates/lane/src/fpenv.rs:33` ("Refusing a DAW's callback thread is not available").

## Findings

No BLOCKER, no MAJOR.

- MINOR `crates/host-core/src/lib.rs:78`, `crates/capi/src/runtime/tests.rs:3091`: D1 reads "it says ... a native
  host's audio thread may run with FTZ and DAZ set (for example, ...), and the C ABI contract does not forbid it.
  Each passage may cite `fpenv.rs:16-17` instead of repeating the reason." `render_session.rs:9` cites fpenv; these
  two carry neither the reason nor the cite. Nothing in them is false, and the root ruling's test (every claim
  true) is met, so this does not fail the attempt; a strict reading of D1 wants one of the two. Optional to fix.
- NIT (out of this slice's paths, for root): the spec's `:3080-3081` anchor is stale (`:3091-3092` at the base),
  and STREAMS.md still cites `:3080-3081` (`docs/handoffs/decision-15-2026-10-05/STREAMS.md:117` and the stream G
  "Owns" list at `:401`). S0's file; refresh at the next STREAMS edit.
- NIT (pre-existing, not this slice, for root): `crates/host-core/src/lib.rs:19` has an intra-doc link to
  `SessionControlProvider`, which exists only under the `control-provider` feature. The spec's gate 3 form
  `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p host-core --no-deps` fails with
  `unresolved link to SessionControlProvider` at `lib.rs:19:49` on both the base `b3ddc8d86` and the commit
  (reproduced on both exported trees). It passes with `--features control-provider` and in the workspace form,
  where feature unification enables it. Worth a small follow-up (cfg_attr the link or name it in backticks), since
  a default-feature per-crate doc build is broken.

## Open item from the implementer: the doc gate form

Confirmed. The right gate is the workspace form CI runs, `RUSTDOCFLAGS='-D warnings' cargo doc --locked
--workspace --no-deps` (`.github/workflows/qualification.yml:475`, the required `qualification` check). It passes at
the commit. The spec's `-p host-core` form is a spec defect (it fails on the untouched base for an unrelated
reason), not an attempt defect; `-p host-core --features control-provider` also passes at the commit if a
per-crate form is wanted.

## Test value

No test added or changed (comment-only; a prose-grep test is refused by AGENTS.md). No mutation run, as the spec
says: gate 1 is this verifier read.

## Gates run (exported tree of `c393bf892`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1498/target`)

- `cargo fmt --all -- --check`: ok.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: exit 0.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p host-core --no-deps`: exit 101 at commit and at base, same
  error (`lib.rs:19:49`); with `--features control-provider`: exit 0.
- `cargo clippy --locked -p host-core -p capi --all-targets --all-features -- -D warnings`: exit 0.
- `cargo test --locked -p capi`: 81 + 3 + 11 + 0 passed, exit 0.
- `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-realtime-policy.sh`: ok (93 marked regions in
  26 files).
- Gate 2 (C ABI audit digest and browser module digest) not re-run: every changed line is a doc comment, which
  does not reach machine code; the implementer's record gives `pcm_digest cb10fbface44a3a4`, 0 allocations,
  0 syscalls and identical browser module digests before and after.
- Edited lines are at most 99 columns (`max_width = 100`).
