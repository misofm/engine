# Count and cap VCA groups in host preparation and the C ABI

Slice V4 of *VCA groups* (#1239). It is in the VCA batch, after *Declare VCA groups in the session*
(#1240). It is the VCA twin of *Count and cap submix strips in host preparation and the C ABI*
(#1206): read its attempt record (git history, `cfa086d4a`) for the literal list and gate shapes
this slice repeats. It needs only #1240's `SessionModel.vcas`, and is ordered after #1242 in the
batch only to keep one tranche in flight.

## Product outcome

- host-core counts VCA groups, reports the count, and refuses a session that exceeds a configured
  `maximum_vcas` with the existing count refusal.
- A C ABI caller bounds VCAs through a word of the compile limits that is reserved today, with no
  layout, size or symbol change; zero means "use `maximum_tracks`", which every existing caller
  passes.

The bound is a configured resource, never a compiled maximum (`AGENTS.md`; DESIGN P12).

## Context (verified on `8c6268967`)

- **Caps.** `HostPrepareCaps` (`crates/host-core/src/prepare.rs:101-141`) has `maximum_tracks`,
  `maximum_submixes` (`:114`, #1206), `maximum_sources`, `maximum_routes`, `maximum_effects` and
  byte budgets. It derives only `Clone, Copy, Debug, Eq, PartialEq`; **there is no `Default`**, so a
  new field is a compile break at every full struct literal. There are exactly **24** full literals
  (`git grep -n "HostPrepareCaps {" -- '*.rs' ':!docs'`, minus the struct, the `impl` and the
  functional updates `..caps()`/`..bad_ring` in `crates/host-core/tests/prepare.rs:520-648`):
  - `crates/capi/src/runtime/compile.rs:368` (`prepare_caps`) and
    `crates/capi/tests/resource_lifecycle.rs:467`;
  - `crates/host-core/src/limiter_linked_session.rs:255` and `crates/host-core/src/response.rs:997`;
  - `crates/host-core/tests/`: `collapse_arming.rs:55`, `effect_live_controls.rs:31`,
    `effect_observation.rs:40`, `fp_environment.rs:32`, `input_liveness_live_controls.rs:70`,
    `live_addressing.rs:58`, `live_routes.rs:50`, `prepare.rs:22`, `randomized.rs:90`,
    `route_mute.rs:58`, `source_in_place.rs:25`, `spectrum.rs:19`, `strip_controls.rs:33`,
    `strip_handles.rs:25`, `strip_meters.rs:54`, `submix_caps.rs:16`, `submix_strip.rs:73`,
    `symmetry_witness.rs:71`, `track_delay.rs:41`;
  - `hosts/host-web/src/lib.rs:6054`.

  Re-run the grep at the slice's base: #1242 may add a test file with its own literal.
- **The count check** (`prepare.rs:810-822`) computes `track_count`, `submix_count`, `source_count`,
  `route_count` and `effect_count` from the normalized model and refuses with
  `resource("host.resource.count")` if any exceeds its cap.
- **Report.** `HostPrepareReport` (`prepare.rs:202-`) has `submix_count` (`:212`); it is built once
  (`:1319`). Its readers copy fields by name, so a new field breaks no reader. host-core's
  `submix_caps.rs` is the test template.
- **host-web caps** (`hosts/host-web/src/lib.rs:6033-6077`) set every count cap to `u64::MAX`
  (`maximum_submixes` at `:6063`) and bound the session by its memory budget.
- **The C ABI limits.** `miso_engine_v1_compile_limits` (`crates/capi/include/miso_engine_v1.h:107-136`)
  ends with `uint64_t maximum_submixes;` (`:134`, offset 176, with its zero-rule comment at
  `:132-133`) and `uint64_t reserved[3]; /* Must be zero in ABI V1. */` (`:135`, offsets 184-207).
  - The Rust mirror is `CompileLimits` (`crates/capi/src/abi.rs:84-139`): `maximum_submixes`
    `:136`, `reserved: [u64; 3]` `:138`. The size is 208 (`COMPILE_LIMITS_SIZE`, `:300`, asserted
    at `:436`); the offsets are pinned at `:458-459` and in `crates/capi/tests/c/abi_smoke.c:45-46`;
    `crates/capi/tests/c/header_smoke.cpp:9` pins the size only.
  - `all_limits_nonzero` (`crates/capi/src/runtime/compile.rs:326-352`) excludes
    `maximum_submixes`; `limits_are_valid` (`:354-359`) requires `reserved == [0; 3]`;
    `prepare_caps` (`:367-392`) maps zero to `maximum_tracks` (`:375-379`).
  - **Five** `CompileLimits` literals, each `reserved: [0; 3]`: `crates/capi/src/ffi.rs:1164-1191`,
    `crates/capi/src/runtime/tests.rs:18-45`, `crates/capi/tests/resource_lifecycle.rs:161-188` and
    `:1806-1833` (`submix_limits`), and `tools/audit/src/capi.rs:239-266`. Other `reserved: [0; 3]`
    literals in `hosts/host-web` are different structs and stay.
  - `scripts/check-capi-abi.sh`'s layout self-test mutates the **first** `uint64_t reserved[4];` of
    the header (`:65`), the engine config at `:104`, so this slice does not disturb it.
  - The C ABI's submix bound is tested by
    `maximum_submixes_bounds_submixes_and_zero_defers_to_maximum_tracks`
    (`crates/capi/tests/resource_lifecycle.rs:1904`), the template for gate 2. Its last loop,
    "The three remaining reserved words still refuse" (`:1936-1942`, with the comment at `:1780`),
    indexes `reserved[0..3]` and panics once `reserved` has two words.
  - `docs/C_ABI_V1_QUALIFICATION.md:64-71` documents `maximum_submixes` and says "`reserved` shrinks
    to three words at 184..207 ... The three remaining reserved words must still be zero", which
    this slice makes false.

## Decisions frozen for this slice

- **D1. host-core.** `HostPrepareCaps.maximum_vcas: u64`, checked in the existing count `if` with the
  same refusal (`host.resource.count`), and `HostPrepareReport.vca_count: u64`, the normalized
  model's VCA count.
- **D2. The C ABI bound** (DESIGN P12).
  - In the header and the Rust mirror, `uint64_t reserved[3]` becomes
    `uint64_t maximum_vcas; uint64_t reserved[2];`: `maximum_vcas` at offset 184, `reserved` at
    192..207. The struct stays 208 bytes; no symbol, size or `ABI_VERSION` changes. The header
    comment states the zero rule.
  - `maximum_vcas == 0` means "use `maximum_tracks`" (never "no VCAs", never "unbounded").
  - `limits_are_valid` accepts any `maximum_vcas` and requires `reserved == [0; 2]`;
    `all_limits_nonzero` does not include it.
  - `prepare_caps` maps it as it maps `maximum_submixes`.
- **D3. host-web** sets `maximum_vcas: u64::MAX`, as for its other count caps.

## Deliverables

1. host-core D1, with doc comments.
2. capi D2: header, Rust mirror, `limits_are_valid`, `prepare_caps`, the offset pins (`abi.rs`,
   `abi_smoke.c`, plus new assertions for `maximum_vcas`), and the five `CompileLimits` literals.
3. host-web D3 (the caps literal only).
4. Every full `HostPrepareCaps` literal updated (24, plus any #1242 added).
5. The submix test's reserved-word loop becomes `0..2`, its comments "the two remaining reserved
   words".
6. `docs/C_ABI_V1_QUALIFICATION.md`: amend the `maximum_submixes` paragraph (`:64-71`) so the tail
   reads `maximum_vcas` at 184 and two reserved words at 192..207, and add the `maximum_vcas` field,
   its zero meaning and the unchanged size.

## Authorized paths

- `crates/host-core/src/{prepare.rs,lib.rs,response.rs,limiter_linked_session.rs}` (the field, the
  check, the report, the two literals)
- `crates/host-core/tests/`: the files in the Context (their literal only) and one new test file
  (for example `vca_caps.rs`)
- `crates/capi/include/miso_engine_v1.h`, `crates/capi/src/{abi.rs,runtime/compile.rs,runtime/tests.rs}`,
  `crates/capi/src/ffi.rs` (the `CompileLimits` literal only), `crates/capi/tests/resource_lifecycle.rs`,
  `crates/capi/tests/c/abi_smoke.c`
- `tools/audit/src/capi.rs` (the `CompileLimits` literal only)
- `hosts/host-web/src/lib.rs` (the caps literal only)
- `scripts/check-capi-abi.sh`, only if a pinned spelling of the compile-limits tail must follow D2
- `docs/C_ABI_V1_QUALIFICATION.md`
- this spec

## Non-goals

- No change to the 240-byte plan resource report, the exported symbols or live controls.
- No change to how VCAs render (#1242).

## Hazards

- **A nonzero reserved word** must still be refused: only the renamed word is freed.
- **The zero rule.** `maximum_vcas == 0` must mean `maximum_tracks`; every existing caller passes 0.
- **No oracle moves** for a session without VCAs: the `resource_lifecycle` oracles and `audit capi`
  stay as they are.

## Objective gates

1. **host-core cap at the boundary.** New host-core test: a session with `maximum_vcas + 1` VCAs
   refuses with `host.resource.count`; one at the cap prepares, and `report.vca_count` equals the
   session's VCA count; with `maximum_tracks` below the VCA count and `maximum_vcas` at it, the
   session prepares.
   *Test value: it turns red if VCAs are uncounted, counted against `maximum_tracks` or
   `maximum_submixes`, or missing from the report.*
2. **C ABI bound.** A new test beside
   `maximum_submixes_bounds_submixes_and_zero_defers_to_maximum_tracks`, through
   `miso_engine_v1_compile_session`, on a 2-track, 3-VCA session:
   - `maximum_vcas = 0` refuses when `maximum_tracks = 2` and prepares when `maximum_tracks = 3`;
   - `maximum_vcas = 3` with `maximum_tracks = 2` prepares; `maximum_vcas = 2` refuses;
   - a nonzero `reserved[0]` or `reserved[1]` (of the two-word array) refuses with
     `RESULT_INVALID_ARGUMENT`.

   *Test value: it turns red if the C bound ignores the new word, treats zero as "no VCAs", reuses
   `maximum_submixes`, or stops refusing the remaining reserved words.*
3. **Layout unchanged.** `abi.rs` and `crates/capi/tests/c/abi_smoke.c` assert
   `offsetof(.., maximum_submixes) == 176`, `offsetof(.., maximum_vcas) == 184`,
   `offsetof(.., reserved) == 192` and size 208;
   `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test` pass.
   *Test value: it turns red if `maximum_vcas` lands at another offset, moves `reserved`, or
   changes the struct size, which a C host compiled against the frozen layout would misread.*
4. **Unchanged where no VCA exists.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi` reports zero allocations, locks and syscalls.
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - The `resource_lifecycle` oracles do not move.
5. **Workspace and policy.**
   - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - `rm -rf <A> <B> && mkdir -p <A> <B> && bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
     and `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (host-web's caps literal changes)
6. **4-lane.** `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug`
   at the batch push (recorded "at batch push"). `capi` and `host-core` are in its package list.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer, and the mutation that turned it red.
- The list of updated `HostPrepareCaps` and `CompileLimits` literals.
- Confirmation that no digest or oracle moved.

### Attempt 1 record (Terra)

- **D1** (`prepare.rs`): `HostPrepareCaps.maximum_vcas`, checked in the existing count `if` beside
  `maximum_submixes` (same `host.resource.count`); `HostPrepareReport.vca_count` from
  `model.vcas.len()`. **D2**: header and `CompileLimits` replace `reserved[3]` with
  `maximum_vcas; reserved[2]` (header comment states the zero rule); `limits_are_valid` requires
  `reserved == [0; 2]` and ignores the word; `prepare_caps` maps zero to `maximum_tracks`;
  `all_limits_nonzero` unchanged; offset pins `maximum_submixes == 176`, `maximum_vcas == 184`,
  `reserved == 192` in `abi.rs` and `abi_smoke.c` (size 208 pinned already). **D3**: host-web
  `u64::MAX`. `check-capi-abi.sh` needed no edit (its self-test mutates the engine config's
  `reserved[4]`). Qualification doc: the #1206 paragraph now names `maximum_vcas` at 184 and two
  reserved words at 192..207; a new paragraph states the word, its zero rule and the unchanged size.
- **Literals.** `HostPrepareCaps`: 25 = the 24 listed plus `host-core/tests/vca.rs` (#1242's file,
  the anticipated addition). capi `prepare_caps` and `resource_lifecycle::host_caps` mirror the zero
  rule; `submix_caps.rs` sets 100; every other literal copies its `maximum_submixes` value (100,
  256 in `limiter_linked_session.rs`, `u64::MAX` in `response.rs` and host-web). `CompileLimits`
  (`maximum_vcas: 0, reserved: [0; 2]`): `ffi.rs` test `limits()`, `runtime/tests.rs`,
  `resource_lifecycle.rs` `limits()` and `submix_limits`, `tools/audit/src/capi.rs`.
- **Deliverable 5.** The submix test's reserved loop is `0..2`; its comment says "the two remaining
  reserved words". The VCA test repeats the reserved-word leg as gate 2 requires, with the named
  word nonzero (only the renamed word is freed); its catch overlaps the submix loop by design.
- **Tests and test value** (mutation each, reverted after; logs `/tmp/claude-1002/kv-1243/`):
  - `host-core/tests/vca_caps.rs::vcas_are_counted_capped_and_reported_apart_from_tracks_and_submixes`
    (3 tracks with `maximum_tracks = 3`, `maximum_submixes = 0`; caps 1 and 4 prepare at the cap,
    report `vca_count`, and refuse one over with `host.resource.count\t$\n`; cap 0 refuses one VCA):
    red if VCAs go uncounted, are counted against `maximum_tracks` or `maximum_submixes`, or are
    missing from the report. Mutations: drop the VCA clause -> red; compare against
    `maximum_tracks` -> red; against `maximum_submixes` -> red; `vca_count: 0` -> red.
  - `capi/tests/resource_lifecycle.rs::maximum_vcas_bounds_vcas_and_zero_defers_to_maximum_tracks`
    (2 tracks, 3 empty unity VCAs through `miso_engine_v1_compile_session`; word 0 refuses at 2
    tracks, prepares at 3; word 3 prepares at 2 tracks; word 2 refuses at 2 and at 3 tracks;
    `reserved[0]`/`[1]` nonzero -> `RESULT_INVALID_ARGUMENT`): red if the C bound ignores the
    word, treats zero as "no VCAs" or "unbounded", reads `maximum_submixes`, or stops refusing a
    remaining reserved word. Mutations: map to `maximum_tracks` -> red; zero -> 0 -> red; zero ->
    `u64::MAX` -> red; map from `maximum_submixes` -> red; drop the reserved check -> red; check
    only `reserved[0]` -> red.
  - Layout pins. Mutations: header with `maximum_vcas` and `reserved` swapped ->
    `check-capi-abi.sh` red (`abi_smoke.c` static assert at 184); Rust mirror swapped ->
    `frozen_sizes_alignments_and_representative_offsets_match` red (`left: 200`).
- **Gates** (x86-64 AVX2 host, tree = this commit):
  - 1, 2: green (above).
  - 3: `check-capi-abi.sh` ok (shared and static); `--self-test` ok (#1232: its header legs are not
    relied on; the swap mutation above is the layout evidence).
  - 4: release build of audit/bench/capi/session-validator ok; `audit capi`: allocations 0, locks 0,
    syscalls 0, total violations 0, `pcm_digest` `ff6cdcb96cdcdad5` (the value #1206 recorded);
    `cargo test --release -p audit -p bench -p console-workload` ok (110 passed);
    `resource_lifecycle` passes with no oracle or budget edited.
  - 5: workspace test command rc 0 (114 binaries, 1265 passed, 0 failed); `cargo fmt --check` ok;
    clippy `--all-features -D warnings` clean; `cargo doc` `-D warnings` clean; host-core, realtime
    and workspace `check-*`/`test-*` ok; `check-cross-targets.sh` PASS; web AudioWorklet
    `--named-twin` build and `check-web-audioworklet.sh` ok.
  - 6: `run-aarch64-tests.sh debug`: at batch push (no arm64 host).
- No test superseded; no digest or oracle moved.

## Dependencies

- *Declare VCA groups in the session* (#1240)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- In-place V1 amendment: no new exported C symbol and no C struct size change.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
