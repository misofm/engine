# Build the console benchmark without graph test-support and align its frozen buffers

**Ruled** (coordinator, 2026-09-26): claims (d) and (e) and the harness half of change 6 of
`docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md`, as confirmed by its adversarial
verification (`DIAGNOSIS-2-VERIFY.md`). Benchmark tooling only; no engine source changes.

## Product outcome

Two harness effects make the console benchmark measure something other than the shipped engine.

1. `tools/bench/Cargo.toml:31` builds `graph` with `features = ["test-support"]`, although nothing
   under `tools/bench/src` uses a test-only API. The benchmarked render path therefore carries test
   hooks (a probe test per unit, a thread-local counter per in-place Output input). The verification
   measured +40 to +110 cycles per block on the ring row.
2. The harness's frozen per-claim blocks (`FrozenGraphSource`, `tools/console-workload/src/lib.rs:1657`,
   two `[f32; QUANTUM]` arrays) and the session output planes (`lib.rs:1097`,
   `vec![0.0; QUANTUM * 2]`) land on whatever 16-byte boundary the allocator returns. At 16 or 48 mod 64
   half the 32-byte loads split cache lines. The verification measured about ±0.085 us of movement on
   the ring row from this alone.

Build the benchmark binary without `test-support` and give both buffers 64-byte alignment, so every
record measures the shipped engine at a fixed placement.

## Smallest closable slice

Authorized paths: `tools/bench/Cargo.toml`, `tools/console-workload/src/lib.rs`, their tests, and this
spec. `Cargo.lock` only if the feature change requires it.

1. In `tools/bench/Cargo.toml`, depend on `graph` without `test-support` (`graph.workspace = true`).
   If a bench **test** needs a test-only API, move that feature to `[dev-dependencies]` rather than
   keeping it on the binary.
2. Give `FrozenGraphSource` `#[repr(C, align(64))]`. Its two 512-byte arrays then both start on a
   64-byte boundary wherever the struct is boxed, including `FrozenSourceDriver::claims`
   (`lib.rs:1811`) and the bound row's `Box::new(frozen_track_source(..))` (`lib.rs:1786`).
3. Replace the output `Vec` (`lib.rs:1097`) with a 64-byte-aligned boxed buffer of the same length,
   for example a `#[repr(C, align(64))]` wrapper around `[f32; QUANTUM * 2]`. Every reader keeps
   seeing the same `&[f32]` / `&mut [f32]` slices.
4. No `unsafe`.

## Non-goals

No change to the engine's own allocations (the graph arena and `TransferBlock` alignment are deferred
until a live-producer row exists), to any workload's content, to the runner's arms, or to the record
schema.

## Objective gates

1. `cargo tree -p bench -e features -i graph` (with the target the runner builds) shows no
   `test-support`. `cargo build --release -p bench` and `cargo test -p bench` pass.
2. A console-workload test asserts `as_ptr() as usize % 64 == 0` for every frozen claim's left and
   right plane on the ring row, for the bound row's frozen blocks, and for both output planes.
3. Every console workload's 64-block digest is unchanged, including `BASE_DIGEST` in
   `tools/console-workload/tests/chain_shape.rs`.
4. The wasm console arm still builds: `tools/wasm-console` and `tools/wasm-console-guest` link
   `console-workload`. Run their build and tests as the repository's scripts do, and record whether
   any wasm console digest pin moved (a layout change may move the guest's bytes; a moved pin is
   repinned with the reason, never silently).
5. `scripts/operator/preflight-console-benchmark.sh` passes (zero-launch preflight). Do not run the
   timed runner; the coordinator runs it once after the batch.
6. fmt, clippy with `-D warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`.

## Console benchmark rows

Every row may move by a small amount in either direction; no digest may move. The coordinator's next
paired run records the new baseline.

## Dependencies

None. Land it first in the batch so the engine changes that follow are measured against the corrected
harness.

## Standing rules for the implementer

- Work only from this body. Do not survey the workspace.
- Commit on `codex/<issue>-<slug>` from synchronized `main`.
- Benchmarks are descriptive. Do not tune or retry a timing.

## Attempt 1 evidence

Implementer: Claude Opus 5.5 (attempt 1), branch `codex/935-bench-hygiene`, code commit `1ce76329` on
base `14f2917b`. No engine source changed; no `unsafe`; `Cargo.lock` unchanged.

### What changed

- `tools/bench/Cargo.toml`: `graph = { workspace = true, features = ["test-support"] }` became
  `graph.workspace = true`. No bench test uses a graph test-only API (`tools/bench` has no
  `test_only` reference and no `tests/` directory), so no `[dev-dependencies]` entry was needed.
- `tools/console-workload/src/lib.rs`:
  - `FrozenGraphSource` is `#[repr(C, align(64))]`. `left` sits at offset 0 and `right` at 512;
    the size stays 1024 bytes with no padding, so `FrozenSourceDriver::resource_report` is
    unchanged.
  - New `OutputPlanes([f32; QUANTUM * 2])`, `#[repr(C, align(64))]`. `SessionRuntime::output` is
    now `Box<OutputPlanes>` rather than `Vec<f32>`. `render` lends `&mut self.output.0` and
    `hash_output` iterates `&self.output.0`, which are the same `&mut [f32]` / `&[f32]` of 256
    words the `Vec` gave.
  - `const _: () = assert!((QUANTUM * size_of::<f32>()).is_multiple_of(64))` makes "a
    64-byte-aligned left plane means a 64-byte-aligned right plane" a compile-time fact.
  - `source_binding` now boxes the bound feed's block through a new `bound_track_source`, which
    returns `Box<FrozenGraphSource>`. The alignment test builds the bound blocks through that same
    function.
  - New unit test `tests::the_frozen_blocks_and_the_output_planes_start_a_cache_line` (gate 2).

### Gates

| # | Command | Result |
|---|---|---|
| 1 | `cargo tree --locked -p bench -e features -i graph --target x86_64-unknown-linux-gnu` (the host triple `run-console-benchmark.sh` builds; `--target all` gives the same answer) | PASS: `test-support` appears 0 times. On base `14f2917b` the same command shows `graph feature "test-support"` (parent `bench`). |
| 1 | `CARGO_INCREMENTAL=0 cargo build --locked --release -p bench` | PASS. `nm -C target/release/bench \| grep -c test_only` gives 0. |
| 1 | `CARGO_INCREMENTAL=0 cargo test --locked -p bench` | PASS: 64 passed, 0 failed |
| 2 | `CARGO_INCREMENTAL=0 cargo test --locked -p console-workload` | PASS: unit 4/4 (includes the new alignment test and `the_driver_fed_plumbing_row_renders_the_bound_rows_bits`); automation 4/4; chain_shape 24/24 (includes `BASE_DIGEST`); placement 3/3; plumbing_profile 0 run, 2 ignored measurement harnesses |
| 3 | Scratch digest sweep, described below | PASS: 21/21 digests identical to base |
| 3 | `BASE_DIGEST` in `tools/console-workload/tests/chain_shape.rs` | PASS: unchanged (`57535244…f800`) |
| 4 | `RUSTFLAGS='-C target-feature=+simd128' cargo build --locked --release --target wasm32-unknown-unknown -p wasm-console-guest` | PASS |
| 4 | `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --release --target wasm32-unknown-unknown -p wasm-console-guest -p console-workload` (the `nightly.yml` form) | PASS |
| 4 | `cargo build --locked --release -p wasm-console` | PASS |
| 4 | Host zero-launch refusals from `preflight-wasm-console-benchmark.sh` (empty round marker, missing guest, surplus argument, nonexistent module) | PASS: all four refused |
| 4 | `bash scripts/test-wasm-console-benchmark.sh` (the CI form) | PASS (0/0 runtime and timing invocations) |
| 5 | `bash scripts/operator/preflight-console-benchmark.sh` | PASS with `workload_launches: 0`, after setting the occupied default artifact directory aside (see Deviations). Preflight record: `binary_sha256` `a6d1944b…1b1d`, candidate `1ce76329`. |
| 6 | `cargo fmt --all --check` | PASS |
| 6 | `CARGO_INCREMENTAL=0 cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS |
| 6 | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| extra | `scripts/check-realtime-policy.sh`, `scripts/check-bench-policy.sh`, `scripts/check-workspace-policy.sh` | PASS (all three) |

`cargo tree` after the change:

```text
graph v0.1.0 (crates/graph)
└── graph feature "default"
    ├── bench v0.1.0 (tools/bench)
    │   └── bench feature "default" (command-line)
    ├── builtins-compiler v0.1.0 (crates/builtins-compiler)
    │   └── builtins-compiler feature "default"
    │       ├── bench v0.1.0 (tools/bench) (*)
    │       ├── console-workload v0.1.0 (tools/console-workload)
    │       │   └── console-workload feature "default"
    │       │       └── bench v0.1.0 (tools/bench) (*)
    │       └── graph-compiler v0.1.0 (crates/graph-compiler)
    │           └── graph-compiler feature "default"
    │               ├── bench v0.1.0 (tools/bench) (*)
    │               └── console-workload v0.1.0 (tools/console-workload) (*)
    ├── console-workload v0.1.0 (tools/console-workload) (*)
    └── graph-compiler v0.1.0 (crates/graph-compiler) (*)
```

### The alignment test (gate 2)

`the_frozen_blocks_and_the_output_planes_start_a_cache_line` checks four things.

1. **Layout.** `align_of::<FrozenGraphSource>() == 64`, `offset_of!(FrozenGraphSource, right) ==
   512`, `size_of::<FrozenGraphSource>() == 1024` (no padding), and `align_of::<OutputPlanes>() ==
   64`.
2. **Ring row.** `FrozenSourceDriver::new` builds the claims over `SixtyFourTrackPlumbingRing`'s own
   64 track inputs, sorted. This is the constructor `build_full` calls. Every claim's
   `played_planes` left and right slices satisfy `as_ptr() as usize` is a multiple of 64.
   `played_planes` is the accessor the graph reads a claim through in place.
3. **Bound row.** `bound_track_source` builds all 64 of `SixtyFourTrackPlumbingOnly`'s blocks,
   and they are held at once, so each is a separate allocation. Every `left` and `right` is
   64-aligned.
4. **Output planes.** For real `SessionRuntime::new` runtimes of both plumbing rows, `output[..128]`
   and `output[128..]` are 64-aligned.

The check uses `.is_multiple_of(64)` rather than `% 64 == 0`; see Deviations.

### Digest sweep (gate 3)

A scratch integration test, `tools/bench/tests/zz_scratch_digests.rs`, was never committed and was
deleted after use. It builds with **bench's own feature resolution**. For every row in `WORKLOADS`
and `DRIVER_FED_WORKLOADS` it takes three 64-block digests over `hash_output`:

- **current:** `SessionRuntime::new` with no warmup. This is `chain_shape.rs`'s `render` shape.
- **warm:** the row's `warmup_blocks()` first, then 64 hashed blocks. This is `SessionMeasurement::run_for`'s shape.
- **simd4:** `build_with_dispatch(.., Backend::Simd4)`, the wasm arm's native-at-four leg.

It also digests the four facility arms (meters, control, unarmed, armed) on `sixty_four_track_console`.

The sweep ran twice:

- **Before:** base `14f2917b`, where bench still carried `graph/test-support` and the old layout.
- **After:** this change, with no `test-support` and the aligned layout.

`diff` of the two outputs is empty, so 21 of 21 lines are identical. The "same" column records that
current = warm = simd4 on that row.

| Row | 64-block digest (before = after) | same |
|---|---|---|
| `nine_track_baseline` | `e7c6ef01770ab7da98d4b793a6a817bd50a4dfe682321ecfc023ddc275285a80` | yes |
| `nine_track_ragged_strip` | `17613a3ab693d3f0dfc457b41f77669f880581fa19c433941a1ef2437684198a` | yes |
| `sixty_four_track_console` | `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de` | yes |
| `one_twenty_eight_track_stretch` | `cba2c94f81544caad0945f0720480b568b1a47808d25fd95911f61bd37f5f9b1` | yes |
| `sixty_four_track_eq_only` | `9b2c56a1da62ebda8aef595973870ea077477d209aacdcd6321637e949d5c828` | yes |
| `sixty_four_track_compressor_only` | `95c9375429fbca3449bf9c5134508a6220f17fc9adda0b3267c061b75bb14175` | yes |
| `sixty_four_track_builtins_only` | `b63eccd09c19eb7a6e0608144024ac5b14c7d5f7d1c56012cbbd49d6aad8f7f0` | yes |
| `sixty_four_track_dispatch_only` | `15688888612d161e507bc400b9eed356fc1776797c8c66ca52d1e7c9114d3a2d` | yes |
| `sixty_four_track_idle` | `de2f256064a0af797747c2b97505dc0b9f3df0de4f489eac731c23ae9ca9cc31` | yes |
| `sixty_four_track_console_legacy` | `f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177` | yes |
| `sixty_four_track_eq_comp_simd1` | `f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177` | yes |
| `sixty_four_track_plumbing_only` | `57535244ba953d82f6c9c19428dc83a8ac412018c66acc167818e1917283f800` | yes |
| `sixty_four_track_gain_pan_only` | `01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4` | yes |
| `sixty_four_track_console_mono` | `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7` | yes |
| `sixty_four_track_console_mono_dual` | `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7` | yes |
| `sixty_four_track_console_half_mono` | `4a656cdf63882999b720b7dd765c1b4f7bbcb95e2ab38c10445f71d269665180` | yes |
| `sixty_four_track_plumbing_ring` | `57535244ba953d82f6c9c19428dc83a8ac412018c66acc167818e1917283f800` | yes |
| `sixty_four_track_console`, meters arm, 4 warmup blocks | `6a70dbcc7adfb8d436948b4373104559e993737707a34d388697747b4d1c1466` | n/a |
| `sixty_four_track_console`, control arm, 4 warmup blocks | `6a70dbcc7adfb8d436948b4373104559e993737707a34d388697747b4d1c1466` | n/a |
| `sixty_four_track_console`, unarmed arm, 4 warmup blocks | `6a70dbcc7adfb8d436948b4373104559e993737707a34d388697747b4d1c1466` | n/a |
| `sixty_four_track_console`, armed arm, 4 warmup blocks | `6a70dbcc7adfb8d436948b4373104559e993737707a34d388697747b4d1c1466` | n/a |

### Wasm console arm (gate 4)

- The repository has no committed wasm console digest pin. No test or script pins a
  `guest_module_sha256` or a wasm leg's `output_sha256`. The only such values are the synthetic
  `"1" * 64` in the validator suite and the historical records under `artifacts/`, which
  are records, not pins. **No pin moved, and there was nothing to repin.**
- The guest's bytes did move, as the brief anticipated. Both builds used the simd128 build above in
  this worktree's default target directory:
  - base `14f2917b`: `328fa9c22d11c22ce51ed60a436b3b93b1c7aa0e5209e801e3b618db930cd859` (20 972 184 bytes)
  - this change: `41288cd235b55a9385aa1b41d0ef2258a930ee5be6297b976f15ebc2ed72ade2` (20 983 392 bytes)

  Rebuilding the change reproduced the same hash. The reason is the new `OutputPlanes` type and the
  aligned allocations, which change the guest's code and data. Every wasm record carries the
  `guest_module_sha256` it measured, so the next wasm run records the new module under its own
  hash.
- The wasm legs' `output_sha256` cannot be taken without running the timed wasm host, which this
  brief forbids. The native Simd4 digests above are unchanged, and alignment does not enter any
  arithmetic. The next wasm run's `digest_identity` check (`all_legs_identical`) is the confirmation.

### Deviations

1. **Preflight artifact directory.** Every arm `preflight-console-benchmark.sh` accepts already has
   records under `artifacts/`, so the script as checked in refuses at its overwrite check for any
   arm. The default arm fails with
   `issue-149 artifact already exists: artifacts/issue149/console-benchmark.raw.jsonl`. Its arm
   list also lags the runner's: there are no `--pure-path`, `--copy-removal*` or
   `--plumbing-floor*` cases. The script is outside this brief's authorized paths, so it was left
   unchanged. To run it, `artifacts/issue149` was moved to the scratchpad, the unmodified default
   arm was run (PASS, 0 workload launches), and the directory was moved back. `git status` was
   clean afterwards. **Coordinator:** add the batch's arm to the preflight before the timed run.
2. **`% 64 == 0` became `.is_multiple_of(64)`** in the test and the const assertion. The two are
   equivalent. Clippy's `manual_is_multiple_of` lint, under `-D warnings`, rejects the `%` form,
   and the repository already uses `is_multiple_of`.
3. **`bound_track_source` was added**, a four-line helper that `source_binding` now calls. It lets
   the test build the bound blocks through the product path instead of a transcription.
4. **The ring-row check does not read the plan's instances.** It builds a driver through the same
   constructor over the same claims. The plan's driver is owned inside the graph, and reaching it
   would need a new test hook in `graph`, which is out of scope. The allocation's alignment is a
   property of the element type's layout, which the allocator must honour, so the result holds for
   every instance. The output planes, by contrast, are checked on the real runtimes.
5. **Wasm builds used this worktree's default target directory** rather than the preflight's
   `target/ci/issue163-phase2-guest` and `-scalar` directories, because disk is short. For the same
   reason the scalar-guest refusal check was not run. It tests the host's module-feature check, and
   this change does not touch the host. The runner's frozen `CARGO_PROFILE_RELEASE_*` environment
   was not applied to these builds, so the guest hashes above are informational before/after
   evidence, not runner hashes.

## Sol attempt 1 verdict: PASS

Reviewer: Sol (Claude Opus 5.5), adversarial review of `git diff 14f2917b..e2569f58`. I re-ran
every gate myself in this worktree. I did not edit the implementation and ran no timed benchmark.

### Gates, re-run independently

| # | What was re-run | Result |
|---|---|---|
| 1 | `cargo tree --locked -p bench -e features -i graph` with `--target x86_64-unknown-linux-gnu` and `--target all` | 0 `test-support` (base `14f2917b`: 1, parent `bench`). No crate in bench's whole feature tree enables any `test-support` (0 over `--target all`) |
| 1 | The runner's exact build (`scripts/run-console-benchmark.sh:428`: `cargo build --locked --release -p bench` alone, under its frozen `CARGO_PROFILE_RELEASE_*` env) | Builds. Every `target/release` graph fingerprint records `features: []`. No `test_only`/`TEST_ONLY` symbol or string in the binary. The runner selects `bench` only, so no other workspace member's features unify in. The `test-support` that `console-workload` needs stays under its `[dev-dependencies]` (`tools/console-workload/Cargo.toml:34`), which Cargo never resolves for a dependent |
| 1 | `cargo test --locked -p bench` | 64/64. Nothing loses a test-only API: `tools/bench` names none and has no `tests/` directory, and console-workload's 4 unit and 35 integration tests (2 ignored) still build with `graph/test-support` through the dev-dependency |
| 2 | `cargo test -p console-workload` | Passes. **Mutation:** with `align(64)` dropped from both types *and* the two `align_of` asserts removed, the pointer checks alone fail (`ring claim 0` at 32 mod 64). The test catches the defect, not just the layout |
| 3 | My own sweep: a scratch `tools/bench/tests` test, deleted afterwards, built with bench's own feature resolution. It covers all 17 rows (no warmup, row warmup, `Simd4`) and the four facility arms. It ran against the base files (graph `test-support` on, old layout) and against HEAD | 21/21 lines identical. The values match the attempt 1 table. `BASE_DIGEST` passes in `chain_shape.rs` |
| 4 | simd128 guest release build | Module hash reproduces `41288cd2…72ade2` |
| 4 | Other wasm builds: the `nightly.yml` check form, a scalar-guest check, the `--cfg miso_wasm_simd8` check, and the `wasm-console` release build | All pass. `wasm-console` has 0 tests |
| 4 | Wasm host refusals and `scripts/test-wasm-console-benchmark.sh` | All four refusals exit 1. The validator suite passes with 0/0 invocations |
| 4 | Search for a committed wasm digest pin | None in the scripts, validators or sources. No pin moved |
| 5 | Preflight run exactly as checked in | Refuses at `scripts/operator/preflight-console-benchmark.sh:102`. This is not caused by this change: all 41 of its arms have occupied artifact directories |
| 5 | Preflight with `artifacts/issue149` set aside, then restored | PASS, `workload_launches: 0`. `binary_sha256` `a6d1944b…1b1d` reproduces the attempt 1 value. Afterwards the tree is clean, and the directory's 3 tracked files match HEAD (`git diff --quiet HEAD -- artifacts`) |
| 6 | `cargo fmt --all --check`, workspace clippy (`--all-targets --all-features -D warnings`), `-p bench -p console-workload --all-targets` clippy, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`, and the realtime, bench and workspace policy scripts | All pass |

**Scope.** Three paths changed: `tools/bench/Cargo.toml`, `tools/console-workload/src/lib.rs`, and
this spec. `Cargo.lock` is unchanged. No engine source changed, and the diff adds no `unsafe`. The
new names carry no version suffix. The `REALTIME_POLICY` region is untouched.

**Alignment.** The alignment is real and safe on every path.
- Bound feed: `bound_track_source` (`lib.rs:1799`) returns a `Box<FrozenGraphSource>`. It
  unsize-coerces into the `Box<dyn GraphRuntimeProcessor>` that `GraphNodeBinding::new` stores,
  so the allocation stays the same one.
- Driver claims: `Box<[FrozenGraphSource]>` (`lib.rs:1843`) is collected through a `Vec` whose
  layout carries align 64. `Box::new(driver)` moves only the fat pointer.
- Render path: the harness never copies the frozen words into another container. The bound
  processor copies them into the engine's arena, and the driver lends them in place. The only
  unaligned `Vec<f32>` left, `SourceSignal::block`, is build-time only.
- Sizes: `size_of` stays 1024 bytes, so `resource_report` is unchanged.
- Output: `Box<OutputPlanes>` (`lib.rs:1113`) lends the same 256-word slice in the same order.

### Findings, most severe first

None blocks the verdict.

1. **Low, coordinator action.** Gate 5 cannot pass for anyone as the preflight is checked in.
   - All 41 arms refuse at `preflight-console-benchmark.sh:102` because their artifact
     directories are occupied.
   - The preflight also lacks 8 of the runner's arms: `--pure-path` and `--pure-path-baseline`,
     `--copy-removal`, `--copy-removal-baseline` and `--copy-removal-without-920`,
     `--plumbing-floor` and `--plumbing-floor-baseline`, and `--issue388-lane4-evidence`.

   The gate is met in substance, and I reproduced it. Before the timed run, the coordinator must
   register the batch's arm in the preflight and run it there.
2. **Low, accepted.** The gate 2 checks for the ring row (`lib.rs:2253`) and the bound row
   (`lib.rs:2272`) inspect instances rebuilt through the product constructors, not the plan's own
   instances. The output planes, by contrast, are the real runtimes'.

   This satisfies gate 2, for three reasons:
   - The asserted `align_of`, `offset_of` and `size_of` facts, together with the allocator
     contract, fix every heap instance of the type.
   - The plan's instances are exactly such instances, built by the same two functions
     (`lib.rs:1019` and `lib.rs:1821`).
   - Reaching the plan's own instances would need a `graph` test hook, which is outside the
     authorized paths.

   The remaining gap is regression coverage only: a later `build_full` or `source_binding` that
   stops using those constructors would escape the test.
3. **Low, coordinator action at merge.** Merging into `origin/main` gives an add/add conflict on
   this spec. `git merge-tree` shows it. `65671c21` (#939) added the same brief about 2 minutes
   after the branch was cut. The branch's copy is a strict superset, so take the branch version.
4. **Informational.** Two small departures, neither needing action:
   - The test uses `.is_multiple_of(64)` rather than the literal `% 64 == 0`. The two are
     equivalent, and clippy under `-D warnings` requires the method.
   - Three other bench commands, `rack.rs:577`, `builtins.rs:541` and `input_symmetry.rs:217`,
     still render into unaligned `Vec`s. They are not the console benchmark and are outside this
     brief.
