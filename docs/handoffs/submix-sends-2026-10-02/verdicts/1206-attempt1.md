# #1206 *Count and cap submix strips in host preparation and the C ABI*: Sol verdict, attempt 1

- Reviewed: `git diff b6b1bdf4b 8f8c013e6` (branch `codex/batch-submix-k2`, worktree
  `/home/bl/misofm/wt-submix-k2`). 28 files, +365/-9.
- Binding: `AGENTS.md` and `.github/ISSUE_SPECS/1206-count-and-cap-submix-strips-in-host-preparation-and-the-c-abi.md`
  with its Attempt 1 record, plus DESIGN P12 (`docs/handoffs/submix-sends-2026-10-02/DESIGN.md:238`).
- Every path is on the authorized list except `crates/host-core/tests/submix_strip.rs`, which is
  deviation 2 below.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. #1206 is OPEN, which is correct in batch
    mode.
  - I exported `8f8c013e6` with `git archive` into `/tmp/claude-1002/v1206/src`, with its own
    `CARGO_TARGET_DIR`.
  - After every mutation I restored the file and checked it with `diff`.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There are two MINOR findings. One is a candor gap in the record.
The other is a **pre-existing** defect in a gate's self-test that this slice did not cause. There
are three NITs. Neither MINOR needs another attempt.

- D1, D2 and D3 are implemented exactly as frozen.
- The header, the Rust mirror, the qualification doc and the spec agree on the field, its offset,
  its zero rule and the unchanged size.
- Every gate I re-ran passes. Four mutations of my own each turn a test red.

## Gates (re-run by me on `8f8c013e6`, x86-64 AVX2)

| Gate | Command | Result |
|---|---|---|
| 1 | `cargo test --locked -p host-core --test submix_caps` | 1 passed |
| 1, 5 | `cargo test --locked -p host-core` (every target) | all green, 0 failed |
| 2, 4 | `cargo test --locked -p capi` (unit tests and `resource_lifecycle`) | 35 + 10 passed. No `resource_lifecycle` oracle or budget was edited |
| 3 | `bash scripts/check-capi-abi.sh` | ok, shared and static |
| 3 | `bash scripts/check-capi-abi.sh --self-test` | ok, but see MINOR-2: its three header legs are vacuous |
| 4 | `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then `./target/release/audit capi` | allocations 0, locks 0, syscalls 0, total_violations 0. `pcm_digest` is `ff6cdcb96cdcdad5`, the same as the record |
| 5 | `check-`/`test-` pairs for `host-core-policy`, `realtime-policy` and `workspace-policy` | all six ok |
| 5 | `cargo fmt --all -- --check` | ok |
| 5 | `cargo clippy --locked -p host-core -p capi -p audit -p host-web --all-targets --all-features -- -D warnings` | clean |
| 5 | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p host-core -p capi` | clean |

I did not re-run these:
- the full workspace `test-debug-a` command;
- `cargo test --release -p audit -p bench -p console-workload`. Disk is tight. The only audit
  change is a struct literal, and clippy `--all-targets` compiles it;
- gate 6, aarch64. There is no arm64 host, so it stays "at batch push", as the record says.

## C ABI review

- **Layout.**
  - `maximum_submixes` is at offset 176, and `reserved[3]` is at 184..207. The size is still 208.
  - Both `abi.rs` (`frozen_sizes_alignments_and_representative_offsets_match`) and `abi_smoke.c:45-46`
    pin these offsets. The old `reserved == 176` pin is replaced in the same change, not left beside
    the new one.
  - `header_smoke.cpp` pins the size only, and that is unchanged.
- **Old caller, new library.** An old caller zero-fills `reserved[4]`.
  - The set of valid inputs is unchanged for that caller. Old `reserved[1..3]` are new
    `reserved[0..2]`, still required to be zero (`compile.rs:357`). The freed word is not in
    `all_limits_nonzero`.
  - A zero word maps to `maximum_tracks`. `all_limits_nonzero` already forces `maximum_tracks` to
    be at least 1. So zero can never mean "no submixes", and never "unbounded" unless the caller
    already chose `maximum_tracks = u64::MAX`.
- **New caller, old library.** A pre-#1206 library refuses a nonzero word with
  `RESULT_INVALID_ARGUMENT`. That is the safe direction: the caller gets a typed refusal, and the
  word is never misread.
- **`ABI_VERSION` unchanged: justified.**
  - No symbol, size or offset moves.
  - Every input that was valid stays valid.
  - AGENTS.md's prelaunch identity rule keeps live boundaries at V1.
  - `check-capi-abi.sh --self-test` treats `0x00010001` as drift.
- **The structural-edit path.** A replacement plan goes through
  `prepare_runtime(.., self.limits)` (`runtime/control.rs:749`, `:782`) and so through the same
  `prepare_caps`. A structural edit that adds submixes past the bound gets the same count refusal.
- **host-web** sets `u64::MAX`, so the count check can never refuse there, and the memory budget
  is still the bound. That is the same as its behaviour before #1206.

## Cap semantics

- `submix_count` comes from `model.submixes.len()` on the normalized model.
- It is checked in the same `if` as `maximum_tracks` and refuses with `host.resource.count`.
- It is reported once, in `HostPrepareReport` (`prepare.rs:1254`).
- It is a configured resource, never a compiled constant.

## Deviations

1. **The C ABI test lives in `resource_lifecycle.rs`: accepted.**
   - `scripts/check-realtime-policy.sh:29` lets `unsafe` appear in only one capi test file, and
     that file is `crates/capi/tests/resource_lifecycle.rs`.
   - Calling the exported `unsafe extern "C"` functions needs `unsafe`. The policy script is not
     an authorized path for this slice.
   - The file's allocator counters are thread-local (`:28-32`). So the new test cannot disturb its
     neighbours' allocation oracles.
2. **The literal in the K1 test `submix_strip.rs:86`: accepted.**
   - K1 added that file after the spec counted 17 literals. `HostPrepareCaps` has no `Default`, so
     the new field breaks every full literal.
   - The fix is one line, using the file's own `maximum_tracks` value. The spec told the
     implementer to re-run the grep before editing.

## Mutations (mine)

| # | Mutation | Result |
|---|---|---|
| M1 | `prepare_caps`: `maximum_submixes: limits.maximum_submixes.max(limits.maximum_tracks)` | capi test red at "the word overrides a larger track cap" |
| M2 | `prepare.rs`: `submix_count >= caps.maximum_submixes` (off by one) | `submix_caps` red (`:58`), and the capi test red |
| M3 | `all_limits_nonzero` also requires `maximum_submixes` (zero refused) | capi test red: `left: 1` (`RESULT_INVALID_ARGUMENT`), `right: 5` |
| M4 | header with `reserved[3]` and `maximum_submixes` swapped, saved as `<dir>/miso_engine_v1.h` and passed through `MISO_ENGINE_CAPI_HEADER` | `check-capi-abi.sh` red: `abi_smoke.c:45` and `:46` static assertions fail |

## Test value

- `host-core/tests/submix_caps.rs::submixes_are_counted_capped_and_reported_apart_from_tracks`
  turns red if:
  - host preparation stops counting submixes;
  - it counts them against `maximum_tracks` (the track cap equals the track count);
  - it applies the bound off by one (M2);
  - it leaves `submix_count` out of the report.
- `capi/tests/resource_lifecycle.rs::maximum_submixes_bounds_submixes_and_zero_defers_to_maximum_tracks`
  turns red if the C bound:
  - ignores the word;
  - maps zero to anything but `maximum_tracks` (M3 is one such mapping);
  - merges the word with the track cap (M1);
  - stops refusing any of the three remaining reserved words.

  No existing test catches any of these, because before this change no test set a nonzero
  `CompileLimits.reserved` word.
- The re-pinned offsets in `abi.rs` and `abi_smoke.c` turn red if the field or `reserved` moves
  (M4).

## Findings

### MINOR-1: the record does not say that zero tightens the bound for existing callers

Where the claim appears:
- The spec's Product outcome says "zero keeps today's behaviour".
- D2 says zero "is today's meaning for every existing caller".

Neither is literally true:
- From K1 (merged `258e1008c`) until this change, a C caller's submixes were not counted at all.
  Only the byte budgets bounded them.
- After #1206, a zero word newly refuses any session with more submixes than `maximum_tracks`, with
  `host.resource.count`.

This is safe:
- The change is a typed compile-time refusal, never a silent drop or a misread.
- It lands within days of K1, before launch.
- The implementation's own text is accurate: the header, `abi.rs` and the qualification doc all say
  "the value every caller already passes", not "behaviour unchanged".

The record should still say it.

**Fix:** add one sentence to the Attempt 1 record, and optionally to the
`C_ABI_V1_QUALIFICATION.md` paragraph: "For a caller written before #1206, zero newly bounds
submixes by `maximum_tracks`. Between K1 and #1206 they were uncounted."

### MINOR-2 (pre-existing, outside this slice): the header legs of the `check-capi-abi.sh` self-test are vacuous

**The defect.**
- Three self-test legs mutate the header: `header-constant-drift`, `header-layout-drift` and
  `header-signature-drift`.
- Each copies the header to `$scratch/abi-version.h`, `layout.h` or `signature.h`, and passes that
  path as `MISO_ENGINE_CAPI_HEADER`.
- The fixtures `#include "miso_engine_v1.h"`. So every mutated run fails with
  `fatal error: miso_engine_v1.h: No such file or directory` before any static assertion runs.

**Proof.** I copied the *unmodified* header as `layout.h` and ran it the same way. It fails with
rc 1 and the same error.

**Consequences.**
- Gate 3's "`--self-test` ok" proves nothing about header drift, for this slice or for any other.
  The same applies to #1199 and #1205, and to the briefs for #1211, #1214, #1218 and #1224, which
  all cite it.
- The real protection is the `abi_smoke.c` pins, which M4 shows do work.

This is not caused by #1206. The script is authorized here only "if a pinned spelling must follow
D2". So this does not count against the attempt.

**Fix:** open a bounded tooling successor, and record it in the K2 follow-ups ledger.
- Write each mutated header as `miso_engine_v1.h` in its own scratch directory.
- Before the mutations, add a positive control: an unmutated header copied the same way must pass.

### NIT-1: the `prepare_caps` doc comment is out of date

`compile.rs:361-365` still says "field for field". Add "except `maximum_submixes`, whose zero means
`maximum_tracks`".

### NIT-2: the header does not say the reserved words must be zero

The header still does not state that `reserved` must be zero; the Rust mirror does. The spec noted
this gap. One comment, `/* Must be zero in ABI V1. */`, would close it.

### NIT-3: a caller cannot tell whether the library supports the word

- A caller cannot discover whether a library honours `maximum_submixes`. An older library answers
  a nonzero word with `RESULT_INVALID_ARGUMENT`.
- This is safe, and moot before launch with a single library generation.
- Optionally, the qualification paragraph could state it.
- `resource_lifecycle.rs::host_caps` also restates the zero rule. That follows the file's existing
  mirror pattern, which is acceptable.

## Scratch

`/tmp/claude-1002/v1206/` (the export, the target directory and the mutation copies) was deleted
after the run.
