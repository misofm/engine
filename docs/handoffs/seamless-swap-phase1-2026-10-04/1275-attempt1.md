# #1275 attempt 1 verdict: PASS

PASS with one MINOR and four NITs. There is no BLOCKER and no MAJOR.

- The export does what D1-D4 freeze. Every gate passes when I re-run it.
- Each new or amended test goes red under the defect it names. My probes also cover the past
  anchor, a seek_at while a successor is pending, a source the successor removes, and a held seek
  that crosses the swap. All of them are bit-exact or refused as documented.
- The #1273 MINORs are closed. The behavioural check replacing `carried_count() == 1` does defend
  D1: under V10 it is the only capi test that goes red. On the candidate, `carried_count()` really
  is 2, so the suggested assertion would have been wrong.
- The MINOR is in the checker's self-test. The new `seek-at-undefined-reference` case does not
  model an undefined reference. A checker that stops filtering out undefined references keeps the
  self-test green.

## Commit reviewed

- `git diff f41388939 55690373d`. The parent is `f41388939`, as expected. 11 files, +525/-12.
- I exported with `git archive 55690373d` to `/tmp/claude-1002/v1275/attempt1/` and built and
  tested only there.
  - `tar --compare` shows the export is still byte-identical to the commit (only uid, gid and
    mtime differ).
  - Gate logs are in `/tmp/claude-1002/v1275/attempt1/logs/`.
- I ran probes and mutations in a second export, `/tmp/claude-1002/v1275/mut/`, and reverted each
  mutation.
  - Mutation driver: `/tmp/claude-1002/v1275/mutate.py`. Logs: `/tmp/claude-1002/v1275/mutlogs/`.
  - The probes are appended to that copy's `runtime/tests.rs`, plus a probe-only `last_error`
    helper in its `ffi.rs`.
- I built no code in, and wrote nothing to, `/home/bl/misofm/wt-swap`. Its HEAD is still
  `55690373d`.
- **Paths.** Every path is in the slice's authorized set, plus two the coordinator assigned for the
  #1273 MINORs: the 1273 spec and `docs/CONTROL_PROTOCOL_SEMANTICS.md`. `lib.rs` needed no change
  (it re-exports `ffi::*` and `abi::*`).
- **Commit message.** It ends with the required `Co-Authored-By` line. No `target/` or artifacts
  were committed.
- **Shared touch point.** In `crates/capi/src/runtime/control.rs`, the only change is the new
  `SessionState::seek_at` (`:1030-1045`, +17 lines; the method starts at `:1032`). It mirrors `seek`: it synchronizes epochs,
  then calls `newest_providers_mut()`. `compile.rs`, the `Structural` arm,
  `synchronize_plan_epochs` and `ProviderEpoch` are untouched.

## Gates re-run (in the export)

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (58 regions in 16 files) / ok |
| `cargo test --locked -p capi` | lib 40, `plan_swap_race` 2, `resource_lifecycle` 9: all pass |
| `cargo test -p capi --all-targets` with CI test-debug-a's `host-core/test-support,protocol/test-support,engine/realtime-audit` | 40 + 2 + 9 pass |
| `cargo test --locked -p audit` | pass |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 100,000 calls. Allocations, deallocations, locks, feature detection, logs, file and network I/O, syscalls, unwinds and render errors are all 0. `pcm_digest` is `c60671f6593fa603` |
| `check-capi-abi.sh` | ok (x86_64 Linux, shared and static linkage) |
| `check-capi-abi.sh --self-test` | ok |
| `check-cross-targets.sh` | PASS. Only the expected #1018 iOS `memset_pattern16` rows fail |

- **Static library.** `nm target/release/libcapi.a` lists `T miso_engine_v1_source_seek_at`.
- **Worklet chain: not run, and not needed.** The diff touches only capi, scripts and docs, and
  none of these is compiled into the browser Wasm module.
- **Width.** capi cannot choose its width, so the tests ran at `Backend::current()` (`Simd8`). The
  slice adds no bank code. I did not verify `Simd4`: that needs the CI aarch64 legs.

## Contract identity

- **The new export, `miso_engine_v1_source_seek_at`, keeps the prelaunch `_v1` identity.**
  AGENTS.md's version rule requires that: no live surface may claim a later generation before
  launch.
- **Signature, header and Rust agree.** The parameter order is session, id, id bytes, generation,
  source_frame, anchor_sample in the header (`miso_engine_v1.h:269-274`), in Rust
  (`ffi.rs:553-560`) and in `abi_smoke.c`'s signature pointer.
- **Capability bit.** `FEATURE_SOURCE_SEEK_AT = 1 << 5` and the mask is `0x3f`. Both are pinned in
  `abi.rs` and `ffi.rs` tests and by `abi_smoke.c`'s static asserts.
- **The amendment is in place.** No struct, size or `ABI_VERSION` changed:
  `MISO_ENGINE_V1_CAPABILITIES_SIZE` is unchanged and `ABI_VERSION` is still `0x00010000`.
  - The thread table gained the symbol in the session group and changed no existing rule.
  - `docs/C_ABI_V1_QUALIFICATION.md:106-121` records the amendment the way it records the
    `maximum_submixes` and `maximum_vcas` amendments, with the count of 15.
- **One observation, not a finding.** The earlier in-place amendments were owner rulings. This one
  is the spec's D3, under the umbrella's P4, which is "subject to owner review". That review should
  cover D3.
- **The checker freezes the symbol.**
  - The frozen list contains it. Mutation S2 deletes it from the list, and the checker then fails
    with "exported symbol set differs".
  - The shared library's set is diffed exactly.
  - The static library is not diffed, as before this slice. But `abi_smoke.c` calls the symbol, so
    a `.a` without it fails the static link leg.
  - Mutation S3 (the header bit set to 64) fails `abi_smoke.c`'s static assert.
- **Thread rule and `last_error` parity.**
  - Both exports share one body (`source_seek_entry`, `ffi.rs:579-625`): the same handle-kind
    check, null and UTF-8 ID checks, and `last_error` clear or set.
  - Probe P3 compared the two results directly for generation 0 and for an out-of-region frame,
    and they match (`source.region.outside` for the frame).
  - Backpressure is shared too: a second queued seek returns `RESULT_BACKPRESSURE` (6) with
    `source.seek.backpressure`.
  - Unaligned anchors 1, 64, 127, 1025 and `u64::MAX` each return `INVALID_ARGUMENT` with
    `source.seek.anchor_unaligned`, at 96 kHz.

## `pcm_digest`

- **This commit did not move it.** I built `audit capi` at three commits:
  - `0297efa8c` (#1274): `7281b6c931e05dcc`, the #1273 value.
  - `807b48547` (#1276): `c60671f6593fa603`.
  - `f41388939` (the parent): `c60671f6593fa603`.
- **#1276 (`807b48547`) moved it.** That is consistent with the swap block now carrying the strip
  input state. #1276's attempt record does not mention the move. That belongs to #1276's PR
  description, not to this slice.
- **Nothing live pins it.**
  - `qualification.yml:778` only checks the format, `[0-9a-f]{16}`.
  - `git grep` finds `c60671f6593fa603` only in this slice's attempt record, and `7281b6c931e05dcc`
    in no tracked file.

## Probes (verifier-only tests in the mutation export; all green on the candidate)

| Probe | What it checks | Result |
|---|---|---|
| P1 past anchor | The transaction is applied, blocks 6-9 render (the swap is at 6), then `seek_at(s2, 2, F=384, A=896)` with A already rendered. The host submits generation 2 from F, and block 10 observes the seek. | Bit-identical at 48 and 96 kHz to the fresh reference with s2 silent below block 10 and frame `F + (1280 - 896)` = 768 from block 10 on |
| P2 `F != A` | `seek_at(s2, 2, 0, 1024)`: stem frame 0 enters at block 8 | Bit-identical at 48 and 96 kHz |
| P3 refusal parity | Unaligned anchors, generation 0, out-of-region frame, backpressure | As listed under Contract identity |
| P4 removed by the pending successor | After a transaction that removes `aux-source`, its seek_at returns `INVALID_ARGUMENT` and `source.id.unknown`; a persisting source's seek_at while pending returns OK | ok |
| P5 held seek crosses the swap | `seek_at(fixture-source, 2, 0, 1024)` before the transaction; the predecessor renders blocks 4-5 and holds the seek; the transaction swaps at block 6; the carried consumer applies the seek at block 8 | Bit-identical to the reference over 16 blocks |

The acceptance test itself is the "seek_at issued while a successor is pending" case. Its seek comes
after the commit, and the next render is the swap.

**Can an ack ever precede a drop? No.**

- `seek_at` returns OK only after `try_push` into the command queue of the ring the newest session
  owns.
- That queue reaches the render in one of three ways:
  - for an added source, in the reserved successor, which the render thread never refuses;
  - for a persisting source, in the carried ring;
  - for a seek the predecessor has already popped, as `held_seek` inside the moved consumer (P5).
- A seek on a source the pending successor removes is refused, never acked (P4).

## Test value (one sentence per new or amended test; I ran every mutation listed)

- **`an_added_c_abi_source_starts_at_its_anchored_render_sample`** (`runtime/tests.rs:1086-1209`,
  gate 1).
  - Red when the export forwards to the plain seek (M1: "48000 Hz added stem block 8").
  - Red when `SessionState::seek_at` addresses the running plan's set (M5: the call at `:1122`
    returns 1).
  - It is the only committed test that catches M5.
- **`anchored_seek_refusals_reach_the_c_host_as_their_own_diagnostic`** (`ffi.rs:2264`, gate 3).
  Red under three mutations, and the only catch of M6 and M8:
  - the anchor is dropped (M1);
  - frame and anchor are transposed in `SessionState::seek_at` (M6), which gate 1 misses because
    it uses `F == A`;
  - the anchored path stops clearing `last_error` on success (M8).
- **`version_and_capabilities_are_exact` and `masks_and_result_codes_are_frozen`** (amended,
  gate 2). Both are red when the bit is left out of `FEATURE_MASK` (M2).
- **`structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic`** (amended,
  `:795-811`).
  - Red when the arm diffs against the prospective model instead of the committed one (V10:
    `NonContiguous { expected: 256, actual: 0 }`). It is the only capi test red under V10.
  - A probe shows `carried_count()` is 2 on the candidate, because the #1276 strip-input carry
    counts. So the behavioural check is the right replacement for the suggested `== 1`.
- **`removing_a_track_and_its_source_keeps_the_other_source_playing`** (amended, `:1348-1356`).
  Red when `seek` goes back to `self.providers` (V4). No other committed test catches V4.
- **`abi_smoke.c`** (bit check and one call).
  - Red when the header's bit or mask drifts (S3, static assert).
  - Red when either library form lacks the symbol (link failure).
  - Its static leg is the only check that the `.a` exports the symbol.
- **`check-capi-abi.sh --self-test` case `seek-at-undefined-reference`.** Its only unique catch is
  a frozen set that treats `seek_at` as optional. It does not defend the property it is named for
  (MINOR 1).

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **The self-test's "undefined reference" case does not model an undefined reference**
   (`scripts/check-capi-abi.sh:92-100`; the claim is repeated in
   `docs/C_ABI_V1_QUALIFICATION.md:120-121`).
   - **The problem.** The wrapper deletes the `miso_engine_v1_source_seek_at` line whatever
     arguments `nm` receives. That is the same shape as the existing `symbol-removal` case.
   - **Mutation S1.** I dropped `--defined-only` from the checker's `nm -D` call, so an undefined
     `U miso_engine_v1_source_seek_at` would count as exported. `check-capi-abi.sh --self-test`
     still printed "C ABI mutation tests: ok". This case does not catch the one defect its name
     and the spec's deliverable 2 point at.
   - **The product is fine today.** The checker does use `--defined-only`, so the defect is in the
     test only.
   - **Fix.** Make the wrapper honour the defined-only flag. When the arguments contain
     `--defined-only` (or `-gU`/`-U` on Apple), delete the line. Otherwise rewrite it to
     `                 U miso_engine_v1_source_seek_at`. For example:

     ```bash
     printf '%s\n' '#!/usr/bin/env bash' \
         'if [[ " $* " == *" --defined-only "* || " $* " == *" -gU "* ]]; then' \
         '    "${REAL_NM}" "$@" | sed "/miso_engine_v1_source_seek_at$/d"' \
         'else' \
         '    "${REAL_NM}" "$@" | sed "s/^.* \([A-Za-z]\) miso_engine_v1_source_seek_at$/                 U miso_engine_v1_source_seek_at/"' \
         'fi' >"$nm_undefined"
     ```

   - **Proof.** I ran this wrapper (without the `-gU` alternative). The self-test passes against
     the real checker, and under S1 it fails with "C ABI mutation unexpectedly passed:
     seek-at-undefined-reference".

### NIT

1. **Gate 1 uses `F == A`**, which is the spec's own shape (`runtime/tests.rs:1122`).
   - As a result, a transposition of frame and anchor (M6) leaves gate 1 green. Gate 3 catches it
     only through its anchor-129 case.
   - A second gate-1 call with `F != A` would make the acceptance test self-sufficient, for
     example stem frame 0 entering at block 8 (my P2, bit-exact).
2. **The header's "Until A the stem renders silence"** (`miso_engine_v1.h:77`) could add that those
   blocks count as source underruns, as `SourceControlSet::seek_at`'s doc says. A host that
   watches underrun telemetry should not be surprised by them.
3. **Line length.** `ffi.rs:543` is 101 columns. Neighbouring doc comments wrap at 100.
4. **Doc wrap and count in `docs/C_ABI_V1_QUALIFICATION.md`.**
   - `:73` is an orphaned line holding only `` (`runtime::tests`' ``. Rewrap the paragraph.
   - `:129` says "the 14 frozen `miso_engine_v1_*` definitions". It is historical, but it now
     reads as stale next to `:119`'s 15. Consider "(15 since #1275)".

## Not verified

- **`Simd4` execution of the gates and of `audit capi`.** This needs the CI `aarch64-debug` and
  `aarch64-release` legs, and the rules forbid running AArch64 locally. The slice adds no bank code.
- **Apple `nm -gU`.** The checker's Apple branch and the proposed `-gU` alternative in the fix
  need a macOS host.
