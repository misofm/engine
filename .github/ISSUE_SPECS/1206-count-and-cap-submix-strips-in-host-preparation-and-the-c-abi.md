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
