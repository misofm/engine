# Ship the browser module with one imported shared memory at every instantiation site

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10).
Code anchors verified on `main` at `6fb211594`.

This is the predecessor slice of *Run the browser control plane in a Worker and keep the
AudioWorklet render-only* (#1332).

## Product outcome

The shipped browser module is built with atomics. It imports one shared `WebAssembly.Memory`
with a declared maximum, and re-exports it as `memory`. Every place that instantiates the module
creates a local shared memory and runs one instance on it. Rendered PCM is bit-identical to
today's module in all three browsers and in Node. A page that is not cross-origin isolated runs
exactly this way for good: it is D15-10's `single` mode. A Worker joining the same memory is
#1332.

## Context

- **The module is built** with `RUSTFLAGS="-C target-feature=+simd128 ..."` and
  `cargo build --release --target wasm32-unknown-unknown -p host-web`
  (`scripts/build-web-audioworklet.sh:111-115`).
  - `+atomics` with the prebuilt std refuses to link (round-2 Q-C4).
  - The nightly `-Zbuild-std` build comes from *Build the browser artifact on a pinned nightly
    toolchain* (#1334).
- **The shape gates forbid this today.**
  - `scripts/check-web-audioworklet.sh:363-381` refuses any import, a shared memory, and any
    atomic opcode.
  - `check_opcode_policy` also refuses atomics (`:64-71`, through `check_no_atomic_opcodes`,
    `:76-81`).
  - The self-tests of these rules are at `:85-110`.
- **Every instantiation site passes empty imports:**
  - the worklet: `hosts/host-web/web/miso-engine-v1-audio-worklet.js:286`;
  - prepared controls: `hosts/host-web/web/prepared-control.js:343`;
  - the SDK asset: `sdk/src/core/asset.ts:121`;
  - the SDK response worker: `sdk/src/browser/response-worker.ts:71`, `:82`, `:93`;
  - the SDK evals: `sdk/test/browser-defaults-evals.mjs:324`, `:399`;
  - the qualification: `hosts/host-web/qualification/rebuild-cost.mjs:98`;
  - the parity oracle: `hosts/host-web/tests/browser-v1/direct-oracle.mjs:308`, `:374`, `:466`;
  - the boot-budget check: `scripts/check-web-boot-budget.mjs:40`, `:63`, run by
    `check-web-audioworklet.sh:591`;
  - the benchmark harness: `scripts/web-mixing-automation-benchmark.mjs:270`, `:764`;
  - the hermetic harness mock: `scripts/test-web-audioworklet.mjs:2722-2756`.

  `docs/handoffs/` patches are historical and are not changed.
- **Docs.** `hosts/host-web/DEPLOYMENT.md:15` and `BROWSER_DEPLOYMENT_MATRIX.md:14` say the
  matrix does not qualify shared-memory operation.
- **Budget.** The boot memory budget defaults to 512 MiB (`DEFAULT_MAXIMUM_MEMORY_BYTES`,
  `hosts/host-web/src/lib.rs:115`).

## Decisions frozen for this slice

- **D1. Build.** Rust target features `+simd128,+atomics,+bulk-memory`. Link arguments:
  `--shared-memory --import-memory --export-memory --initial-memory=<I> --max-memory=<M>`.
  - `M` is the maximum that *Prove two Wasm instances on one shared memory in three browser
    engines and on iOS* (#1331) recorded as reservable on the real iOS device.
  - `I` is the smallest initial size that boots every qualification fixture.
  - Both are named constants in `hosts/host-web/src/lib.rs`, `WASM_INITIAL_MEMORY_PAGES` and
    `WASM_MAXIMUM_MEMORY_PAGES`. The ABI layout generator emits them into
    `miso-engine-v1-abi-layout.json` as `memory.initialPages` and `memory.maximumPages`.
- **D2. Budget.** If `M` is below 512 MiB, `DEFAULT_MAXIMUM_MEMORY_BYTES` becomes `M`. A boot whose
  `maximum_memory_bytes` exceeds `M` is refused with `RESULT_REFUSED_OPTIONS` and the diagnostic
  `web.boot.memory_maximum`.
- **D3. One helper per realm.** Each site creates its memory as
  `new WebAssembly.Memory({ initial, maximum, shared: true })`, with the page counts read from the
  ABI layout. It instantiates with `{ env: { memory } }`. A memory is never shared between two
  instances in this slice.
- **D4. Shape gates, rewritten, not deleted.**
  - The module has exactly one import, `env.memory`. It is shared, and its limits equal the ABI
    layout's `initialPages` and `maximumPages`.
  - The module still exports `memory`.
  - Atomic opcodes are allowed in the module. `memory.atomic.wait` stays forbidden in every
    worklet export's closure: that is #1333's static check, now live.
  - Each rewritten rule keeps a red mutation in the self-test:
    - a second import;
    - an unshared memory;
    - limits that differ from the ABI layout.
- **D5. Docs.** `DEPLOYMENT.md` and `BROWSER_DEPLOYMENT_MATRIX.md` state that the module uses a
  shared memory, that it runs on non-isolated pages with one instance per memory, and that the
  isolated two-instance mode is #1332's.

## Deliverables

1. D1 in `scripts/build-web-audioworklet.sh` (the `RUSTFLAGS` line) and the two constants in
   `hosts/host-web/src/lib.rs`.
2. The generator rows in `tools/parameter-metadata/src/abi_layout.rs`, and the regenerated
   `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`.
3. D2 in boot.
4. D3 at every site listed under Context.
5. D4 in `scripts/check-web-audioworklet.sh`.
6. D5.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`, `hosts/host-web/web/`,
  `hosts/host-web/qualification/`, `hosts/host-web/tests/browser-v1/direct-oracle.mjs`,
  `hosts/host-web/DEPLOYMENT.md`, `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
- `sdk/src/core/asset.ts`, `sdk/src/browser/response-worker.ts`,
  `sdk/test/browser-defaults-evals.mjs`, `sdk/assets/`, `sdk/src/generated/`
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`
- These are outside stream H's ownership; root sequences them after #1334:
  - `scripts/build-web-audioworklet.sh` (the `RUSTFLAGS` line);
  - `tools/parameter-metadata/src/abi_layout.rs` (the memory rows);
  - `scripts/check-web-boot-budget.mjs`, `scripts/web-mixing-automation-benchmark.mjs` and
    `scripts/test-web-audioworklet.mjs` (the instantiation call only).

## Non-goals

- No Worker and no second instance (#1332).
- No change to boot, render or any export's behaviour.
- No toolchain pin (#1334). No new allocation gate (#1333).

## Objective gates

1. **Same bits.** The browser legs' native-digest gate and `check-browser-expected-resources.py
   --artifacts` (the `direct-oracle.mjs` parity) pass unchanged against the new module in
   Chromium, Firefox, WebKit and Node. No pinned digest of rendered PCM changes.
2. **Budget refusal.** A native host-web test boots with `maximum_memory_bytes` one byte above `M`
   and gets `RESULT_REFUSED_OPTIONS` with `web.boot.memory_maximum`. At exactly `M` the budget
   check passes.
3. **Shape.** `bash scripts/check-web-audioworklet.sh` passes on the new module. Its self-test
   turns red on each D4 mutation.
4. **Commands** (verified in `.github/workflows/qualification.yml`):
   - the artifact build, then `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (build as #1332 gate 7 spells it), and the self-building `bash scripts/check-web-audioworklet.sh`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   - `python3 -B scripts/check-scalar-oracle-absent.py --wasm target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-web-audioworklet-v8-spill.py target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm` (Node 22.23.2)
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix --self-test-mutations`
     in `hosts/host-web/qualification`, for chromium, firefox and webkit
   - `cargo test --locked -p host-web --features host-web/test-support`,
     `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`
   - The shipped artifact changes: report its digest (`artifact-identity`).

## Test value

- Gate 1 is not new. It turns red if atomics or the imported memory change any rendered bit,
  for example through a data segment initialized twice.
- Gate 2: turns red if a boot budget above the module's hard maximum is accepted. That boot
  would fail later, inside `memory.grow`, as an untyped trap.
- Gate 3's mutations: each proves that a rewritten shape rule can still fire.

## Dependencies

- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331), for
  `M`.
- *Build the browser artifact on a pinned nightly toolchain* (#1334).
- *Gate AudioWorklet render against allocation statically and at runtime* (#1333). Its static
  thread-local and atomic-wait checks must be in place before atomics ship.
