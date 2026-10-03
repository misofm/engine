# Count and cap submix strips in host preparation and the C ABI

Slice 09 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2, pushed once with slices 09-17.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

After batch K1 a submix is a full strip that every host renders, but host preparation still counts
and caps only tracks. After this slice:

- host-core counts submix strips, reports the count, and refuses a session that exceeds a configured
  `maximum_submixes` with the existing count refusal;
- a C ABI caller gives submixes their own bound through a word of the compile limits that is reserved
  today, with no layout, size or symbol change; zero keeps today's behaviour.

The bound is a configured resource, never a compiled maximum (`AGENTS.md`; DESIGN P12). The
live-control handles do not change here (*List every strip in the live-control handles and file bus
effects in the browser*, #1207).

## Context (verified on `fe8ac679`)

Batch K1 moved no anchor this slice cites, except that `count_effects` (`prepare.rs:1313-1324`) now
counts bus console entries and inserts too (*Render a submix strip on its summed input* (#1200) and *Carry
every console slot on every submix strip*, #1202).

- **Caps.** `HostPrepareCaps` (`crates/host-core/src/prepare.rs:99-137`) has `maximum_tracks`,
  `maximum_sources`, `maximum_routes`, `maximum_effects` and byte budgets. It derives only
  `Clone, Copy, Debug, Eq, PartialEq`; **there is no `Default`**, so a new field is a compile break at
  every full struct literal. There are exactly **17** full literals:
  - `crates/capi/src/runtime/compile.rs:367` (`prepare_caps`);
  - `crates/capi/tests/resource_lifecycle.rs:466`;
  - `crates/host-core/src/limiter_linked_session.rs:255`;
  - `crates/host-core/src/response.rs:997`;
  - `crates/host-core/tests/`: `collapse_arming.rs:55`, `effect_live_controls.rs:31`,
    `effect_observation.rs:40`, `fp_environment.rs:32`, `input_liveness_live_controls.rs:70`,
    `live_addressing.rs:58`, `prepare.rs:22`, `randomized.rs:90`, `source_in_place.rs:25`,
    `spectrum.rs:19`, `symmetry_witness.rs:71`, `track_delay.rs:41`;
  - `hosts/host-web/src/lib.rs:5675`.

  The `..caps()` functional-update literals in `crates/host-core/tests/prepare.rs` (`:519`, `:528`,
  `:541`, `:556`, `:567`, `:637`, `:647`) survive a new field unchanged. Re-run
  `git grep -n "HostPrepareCaps {" -- '*.rs'` before editing.
- **The count check** is at `prepare.rs:765-777`: it computes `track_count`, `source_count`,
  `route_count` and `effect_count` from the normalized model and refuses with
  `resource("host.resource.count")` if any exceeds its cap.
- **Report.** `HostPrepareReport` (`prepare.rs:198-212`) carries `track_count`, `route_count`,
  `effect_count` and others, but no submix count. It is built once, at `prepare.rs:1236`. Its readers
  copy fields by name (capi at `crates/capi/src/runtime/compile.rs:425-440`), so a new field breaks
  no reader.
- **host-web caps.** host-web builds its caps at `hosts/host-web/src/lib.rs:5675-5697`, setting every
  count cap to `u64::MAX` (`:5683` is `maximum_tracks`) and bounding the session by its memory
  budget.
- **The C ABI limits.** `miso_engine_v1_compile_limits` (`crates/capi/include/miso_engine_v1.h:107-133`)
  ends with `uint64_t reserved[4];` at `:132`.
  - The Rust mirror is `CompileLimits` (`crates/capi/src/abi.rs:84-134`); its `reserved` field
    (`:133-134`) is documented "Must be zero in ABI V1". The header carries no such comment.
  - The size is pinned at 208 bytes (`COMPILE_LIMITS_SIZE`, `abi.rs:296`; asserted at `:432`).
  - The offset `reserved == 176` is pinned at `abi.rs:454` and in the C smoke test
    `crates/capi/tests/c/abi_smoke.c:45`. `crates/capi/tests/c/header_smoke.cpp:9` pins only the size.
  - `limits_are_valid` (`crates/capi/src/runtime/compile.rs:354-359`) requires
    `reserved0 == 0 && reserved == [0; 4]`, so every existing caller passes zero.
  - `prepare_caps` (`compile.rs:366-390`) maps the limits field for field into `HostPrepareCaps`.
  - `CompileLimits` literals: `crates/capi/tests/resource_lifecycle.rs:160-161`,
    `crates/capi/src/runtime/tests.rs:17-18`, `crates/capi/src/ffi.rs:1163-1164` and
    **`tools/audit/src/capi.rs:238-265`** (`reserved: [0; 4]` at `:264`; VERIFY-2 M8: without it
    `cargo build -p audit` and clippy break). `crates/capi/src/ffi.rs:1353` sets
    `nonzero_reserved.reserved[2]` on an **`EngineConfig`**, whose `reserved` stays `[u64; 4]`; it is
    not a `CompileLimits` and does not change.
- **`scripts/check-capi-abi.sh`.** Its layout-drift self-test mutates the **first**
  `uint64_t reserved[4];` in the header (`:65`). That is `:104`, the engine-config struct before the
  compile limits, so this slice does not disturb it. The 14-symbol export list (`:193`) is frozen.
- **Today's probe.** A host that sizes `maximum_tracks` to its track count cannot express a separate
  submix bound. Reusing `maximum_tracks` for submixes would silently change that field's meaning
  (VERIFY-1 MINOR-6).

## Decisions frozen for this slice

- **D1. host-core cap, count and report.**
  - Add `HostPrepareCaps.maximum_submixes: u64`, checked beside the track check
    (`prepare.rs:765-777`) with the same refusal, `host.resource.count`.
  - Add `HostPrepareReport.submix_count: u64`, the normalized model's submix count.
  - It is a configured resource, never a compiled constant.
- **D2. The C ABI bound (DESIGN P12).**
  - In both the header and the Rust mirror, `uint64_t reserved[4]` becomes
    `uint64_t maximum_submixes; uint64_t reserved[3];`. The offsets are 176 and 184..207, so the
    struct stays 208 bytes.
  - `maximum_submixes == 0` means "use `maximum_tracks`". That is today's meaning for every existing
    caller, because they all pass zero.
  - `limits_are_valid` accepts any `maximum_submixes` and requires `reserved == [0; 3]`.
    `all_limits_nonzero` does not include it (zero is legal).
  - `prepare_caps` maps it: `maximum_submixes: if limits.maximum_submixes == 0 { limits.maximum_tracks } else { limits.maximum_submixes }`.
  - This is an in-place V1 amendment: no symbol, size or `ABI_VERSION` change. The header gains a
    comment on the new field stating the zero rule.
- **D3. host-web** sets `maximum_submixes: u64::MAX`, exactly as it sets its other count caps, so its
  memory budget remains the bound. Its behaviour does not change.

## Deliverables

1. host-core D1, with doc comments.
2. capi D2: header, Rust mirror, `limits_are_valid`, `prepare_caps`, the offset pins
   (`abi.rs:454`, `abi_smoke.c:45`, plus new assertions for `maximum_submixes`), and every
   `CompileLimits` literal including `tools/audit/src/capi.rs`.
3. host-web D3 (compile fix only).
4. Every `HostPrepareCaps` literal updated (17).
5. `docs/C_ABI_V1_QUALIFICATION.md`: one paragraph on D2, covering the field, its zero meaning and the
   unchanged layout.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs` (re-exports only, if needed),
  `crates/host-core/src/response.rs` (the literal at `:997`),
  `crates/host-core/src/limiter_linked_session.rs` (the literal at `:255`)
- `crates/host-core/tests/`: the twelve files above, plus one new test file (for example
  `submix_caps.rs`)
- `crates/capi/include/miso_engine_v1.h`, `crates/capi/src/abi.rs`,
  `crates/capi/src/runtime/compile.rs`, `crates/capi/src/runtime/tests.rs`, `crates/capi/src/ffi.rs`
  (the `CompileLimits` literal at `:1163-1190` only), `crates/capi/tests/resource_lifecycle.rs`,
  `crates/capi/tests/c/abi_smoke.c`, and one new capi test file (for example
  `crates/capi/tests/submix_limits.rs`)
- `tools/audit/src/capi.rs` (the `CompileLimits` literal only)
- `scripts/check-capi-abi.sh`, only if a pinned spelling of the compile-limits tail must follow D2
- `hosts/host-web/src/lib.rs` (the caps literal at `:5675-5697` only)
- `docs/C_ABI_V1_QUALIFICATION.md`
- this spec

## Non-goals

- No change to `HostLiveControlHandles`, meters, the master designation or live controls (the next
  slices).
- No change to the 240-byte plan resource report or to the export list.
- No VCA cap (the VCA umbrella takes `reserved[1]` the same way).

## Hazards

- **A nonzero reserved word.** `reserved[0..3]` must still be refused when nonzero. Only the renamed
  word is freed.
- **The zero rule.** `maximum_submixes == 0` must mean `maximum_tracks`, not "no submix allowed" and
  not "unbounded". Every existing caller passes zero.
- **The capi resource oracles** (`resource_lifecycle`) must not move for a session without submixes.

## Objective gates

1. **host-core cap at the boundary.** New test in `crates/host-core/tests/`:
   - a session with `maximum_submixes + 1` submixes refuses with `host.resource.count`;
   - one at the cap prepares, and `report.submix_count` equals the session's submix count.

   *Test value: it turns red if submixes are uncounted (and so bounded only by byte budgets), counted
   against `maximum_tracks`, or missing from the report.*
2. **C ABI bound.** New capi test through `miso_engine_v1_compile_session`:
   - with `maximum_submixes = 0`, a 2-track, 3-submix session refuses when `maximum_tracks = 2` and
     prepares when `maximum_tracks = 3`;
   - with `maximum_submixes = 3` and `maximum_tracks = 2`, the same session prepares;
   - with `maximum_submixes = 2`, it refuses;
   - a nonzero `reserved[0]` (of the shrunken array) still refuses with `RESULT_INVALID_ARGUMENT`.

   *Test value: it turns red if the C bound ignores the new word, treats zero as "no submixes", or
   stops refusing the remaining reserved words.*
3. **Layout unchanged.**
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test` pass.
   - `crates/capi/tests/c/abi_smoke.c` and `crates/capi/src/abi.rs` assert
     `offsetof(..., maximum_submixes) == 176`, `offsetof(..., reserved) == 184`, and size 208.

   *Test value: it turns red if `maximum_submixes` lands at another offset, moves `reserved`, or
   changes the struct size, which a C host compiled against the frozen layout would misread.*
4. **Unchanged where no submix exists.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi` reports zero allocations, locks and syscalls.
   - The `resource_lifecycle` oracles do not move.
5. **Workspace and policy.**
   - The workspace test command (DESIGN.md section 7, CI's `test-debug-a`).
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
6. **4-lane.** `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug`
   job at the K2 push (recorded "at batch push"). `capi` and `host-core` are in its package list.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The list of updated `HostPrepareCaps` and `CompileLimits` literals.
- Confirmation that no digest or oracle moved.

### Attempt 1 record (Terra)

- **D1** (`prepare.rs`): `HostPrepareCaps.maximum_submixes`, checked in the existing count `if`
  beside `maximum_tracks` (same `host.resource.count`); `HostPrepareReport.submix_count` from
  `model.submixes.len()`. **D2**: header and `CompileLimits` replace `reserved[4]` with
  `maximum_submixes; reserved[3]` (header comment states the zero rule); `limits_are_valid` requires
  `reserved == [0; 3]` and ignores the word; `prepare_caps` maps zero to `maximum_tracks`;
  `all_limits_nonzero` unchanged. **D3**: host-web `u64::MAX`. Qualification doc paragraph added.
- **Literals.** `HostPrepareCaps` (18, not 17: K1 added `host-core/tests/submix_strip.rs:80`, outside
  the authorized list, a one-line compile fix): capi `prepare_caps` and `resource_lifecycle::host_caps`
  (both mirror the zero rule), `limiter_linked_session.rs`, `response.rs`, host-web, and the 13 test
  files `collapse_arming`, `effect_live_controls`, `effect_observation`, `fp_environment`,
  `input_liveness_live_controls`, `live_addressing`, `prepare`, `randomized`, `source_in_place`,
  `spectrum`, `submix_strip`, `symmetry_witness`, `track_delay` (each `maximum_submixes` = its
  `maximum_tracks`). `CompileLimits` (`maximum_submixes: 0, reserved: [0; 3]`): `ffi.rs` test
  `limits()`, `runtime/tests.rs`, `resource_lifecycle.rs` `limits()`, `tools/audit/src/capi.rs`.
- **Deviation: the C ABI test lives in `crates/capi/tests/resource_lifecycle.rs`**, not a new file.
  A new file calling the exported functions needs `unsafe`, and `check-realtime-policy.sh` refuses
  `unsafe` outside its approved list, which names `resource_lifecycle.rs` and which this slice may
  not edit.
- **Tests and test value** (mutation each, reverted after):
  - `host-core/tests/submix_caps.rs::submixes_are_counted_capped_and_reported_apart_from_tracks`
    (3 tracks with `maximum_tracks = 3`; caps 1 and 4 prepare at the cap and refuse one over with
    `host.resource.count\t$\n`; cap 0 refuses one submix): red if submixes go uncounted, are
    counted against `maximum_tracks`, or are missing from the report. Mutations: drop the submix
    clause -> red; compare against `maximum_tracks` -> red; `submix_count: 0` in the report -> red.
  - `capi/tests/resource_lifecycle.rs::maximum_submixes_bounds_submixes_and_zero_defers_to_maximum_tracks`
    (2 tracks, 3 unrouted unity submixes through `miso_engine_v1_compile_session`; word 0 refuses at
    2 tracks, prepares at 3; word 3 prepares at 2 tracks; word 2 refuses at 2 and at 3 tracks;
    each of `reserved[0..3]` nonzero -> `RESULT_INVALID_ARGUMENT`): red if the C bound ignores the
    word, treats zero as "no submixes" or "unbounded", or stops refusing a remaining reserved word.
    Mutations: map the word to `maximum_tracks` -> red; zero -> 0 -> red; zero -> `u64::MAX` ->
    red; drop the reserved check -> red; check only `reserved[1..]` -> red; drop the host-core
    clause -> red.
  - Layout pins (`abi.rs` test, `abi_smoke.c`): `maximum_submixes == 176`, `reserved == 184`, size
    208. Mutations: header with the two fields swapped -> `check-capi-abi.sh` red (`abi_smoke.c`
    static asserts at 176/184); Rust mirror swapped ->
    `frozen_sizes_alignments_and_representative_offsets_match` red (`left: 200`).
- **Gates** (x86-64 AVX2 host):
  - 1, 2: green (above).
  - 3: `check-capi-abi.sh` ok (shared and static); `--self-test` ok.
  - 4: release build of audit/bench/capi/session-validator ok; `audit capi`: allocations 0, locks 0,
    syscalls 0, total violations 0, `pcm_digest` `ff6cdcb96cdcdad5`, identical to the same command
    on the pre-change tree (one-time comparison). `resource_lifecycle` passes with no oracle or
    budget edited.
  - 5: test-debug-a workspace command rc 0 (100 binaries, 1150 passed, 0 failed);
    `cargo test --release -p audit -p bench -p console-workload` ok (110 passed); host-core,
    realtime and workspace `check-*`/`test-*` pairs ok; `cargo fmt --check` ok; workspace clippy
    `-D warnings` clean; `cargo doc` `-D warnings` clean.
  - 6: `run-aarch64-tests.sh debug`: at batch push (no arm64 host).
- No test superseded; no digest or oracle moved.

## Decision record (K2 follow-ups)

- **Zero tightens the bound for an existing caller** (verdict MINOR-1). The Product outcome's "zero
  keeps today's behaviour" and D2's "today's meaning" are not literally true: from K1 (merged
  `258e1008c`) until #1206 a C caller's submixes were not counted at all, only bounded by the byte
  budgets. With a zero word, #1206 newly refuses a session with more submixes than
  `maximum_tracks`, with `host.resource.count` -- a typed compile-time refusal, never a silent
  drop or a misread, landing within days of K1 and before launch. `C_ABI_V1_QUALIFICATION.md`
  now says so, and that a caller cannot probe whether a library honours the word (NIT-3).
- **The `check-capi-abi.sh --self-test` header legs are vacuous** (verdict MINOR-2, pre-existing):
  filed as the stateless tooling issue #1232, *Make the C ABI checker's header mutation legs reach
  the compiler*. Gate 3's "`--self-test` ok" above proves nothing about header drift; the
  `abi_smoke.c` offset pins are the real protection (the verdict's M4).
- **NITs:** `prepare_caps`' doc says "field for field, except `maximum_submixes`" (NIT-1); the
  header's `reserved[3]` carries `/* Must be zero in ABI V1. */`, a comment only (NIT-2).

## Verdict

- **Attempt 1** (`8f8c013e6`): Sol PASS, no BLOCKER or MAJOR; two MINOR, three NIT, applied in the
  K2 follow-up commit as above. `docs/handoffs/submix-sends-2026-10-02/verdicts/1206-attempt1.md`.

## Dependencies

- *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205, batch K1 closed and
  pushed)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- No new exported C symbol and no C struct size or offset change other than the renamed word.
- Render stays allocation-, lock- and syscall-free.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
