# Ship the browser module with one imported shared memory at every instantiation site

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10).
Code anchors verified on `main` at `6fb211594`; re-anchored on `02630e950` (2026-10-27, S0 of the #1332 plan) in the decision record below, which supersedes the Context list's line numbers.

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

- **D1. Build.** Rust target features `+simd128,+atomics,+bulk-memory,+mutable-globals` (amended
  2026-10-27: `+mutable-globals` is required with `--shared-memory`, since each instance's
  `__stack_pointer` and `__tls_base` are mutable globals and a second instance must be able to set
  them; #1331a's recipe built with it). Link arguments:
  `--shared-memory --import-memory --export-memory --initial-memory=<I> --max-memory=<M>`.
  - `M` is the maximum that *Prove two Wasm instances on one shared memory in three browser
    engines and on iOS* (#1331) recorded as reservable on the real iOS device.
  - `I` is the smallest initial size that boots every qualification fixture. Recorded (2026-10-27):
    `I` = 20 pages, the link minimum, leaving about 28 KB of static-data headroom over the 1 MiB
    stack, data and TLS; growth past it fails the link loudly, and the fix is to raise the
    constant (and the build script's copy).
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
    `scripts/test-web-audioworklet.mjs` (the instantiation call only);
  - `tools/wasm-gates/` and `scripts/run-wasm-gates.sh` (G5's runner: the atomic guest imports
    `env.memory` as shared, so wasmtime needs `Config::wasm_threads(true)`,
    `Config::shared_memory(true)`, its `threads` feature and a `SharedMemory` for the import;
    #1331a-2 gate 4, shim diff in `engine-work-evidence/1331/a2/gates/runner-threads-shim.diff`).

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

## Decision record

- 2026-10-25 (root housekeeping, from #1331a-2): authorized paths add G5's runner
  (`tools/wasm-gates/`, `scripts/run-wasm-gates.sh`). Without it the committed runner refuses the
  atomic guest with `threads must be enabled for shared memories`; under the shim the atomic guest
  has 0 mismatches over 270 comparisons. The atomic build prints `unstable feature specified for
  -Ctarget-feature: atomics` once per build; it is harmless: it does not fail under `-D warnings`,
  and `-A unstable_features` does not silence it (#1331 spec, "#1331a-2 result").

- 2026-10-27 (owner decisions): desktop Chromium and WebKit only (no Firefox anywhere, so gate 1
  and gate 4's Firefox legs are dropped); `M` = 512 MiB = 8192 pages, what #1331a reserved, so D2's
  default stays 512 MiB; #1331's iPhone check (#1331b) is closed as not planned, so iOS Safari is
  "not qualified" in the browser matrix and the dependency on #1331 is met by #1331a; the Worker
  mode ships later in one 0.9.0, so this slice is not released (`docs/RELEASE.md`: nothing
  re-pins).
- 2026-10-27 (S0 re-anchor on `02630e950`): build line `scripts/build-web-audioworklet.sh:177-178`
  (`+simd128` only); shape gate `scripts/check-web-audioworklet.sh:362-379` and its opcode atomics
  ban `:76-110`; budget `DEFAULT_MAXIMUM_MEMORY_BYTES` `hosts/host-web/src/lib.rs:92`, option rules
  in `validate_options`. Instantiation sites (14, the plan's 13 plus the render-lock probe):
  worklet `hosts/host-web/web/miso-engine-v1-audio-worklet.js:261`; `scripts/check-web-boot-budget.mjs:47`,
  `:70`; `sdk/src/browser/response-worker.ts:55`; `hosts/host-web/qualification/rebuild-cost.mjs:97`;
  `hosts/host-web/qualification/structural-edit-reference.mjs:94`;
  `hosts/host-web/tests/browser-v1/direct-oracle.mjs:189`, `:297`, `:348`, `:454`;
  `scripts/web-mixing-automation-benchmark.mjs:276`, `:858`; `sdk/src/core/asset.ts:121`;
  `sdk/test/browser-defaults-evals.mjs:284`, `:359`; the hermetic mock
  `scripts/test-web-audioworklet.mjs:2754`; and `scripts/test-render-lock-probe.mjs:18` (the probe
  builds on the module's cargo line, so it imports the memory too). `prepared-control.js` is gone.
  `hosts/blake3-web` and `stem-store/incremental-blake3.js` stay plain. Authorized paths add
  `sdk/src/core/memory.ts` (the SDK/Node helper), `sdk/src/core/boundary.ts` (two decode copies),
  `scripts/check-abi-layout-v1.py` and its fixture, `scripts/test-web-audioworklet.sh`,
  `scripts/test-render-lock-probe.mjs`, `scripts/test-sdk-artifact-builder-output-contract.sh`,
  the two `.d.ts` copies and `hosts/host-web/qualification/generate-matrix.mjs` (the matrix is
  generated). G5's runner shim (`tools/wasm-gates/`) is not needed: the G5 guest has its own plain
  cargo line, and the V8 spill gate only compiles the module.

## Evidence (2026-10-27, branch `codex/issue-1380-shared-memory` from `02630e950`)

**S1** (`c9171d907`). `WASM_INITIAL_MEMORY_PAGES = 20`, `WASM_MAXIMUM_MEMORY_PAGES = 8192`,
`WASM_PAGE_BYTES` (`hosts/host-web/src/lib.rs:104-122` at `c9171d907`, with const asserts that the default budget
fits the maximum); `validate_options` refuses `maximum_memory_bytes > 8192 * 65536` with
`RESULT_REFUSED_OPTIONS` and `web.boot.memory_maximum`. `I` = 20 is the link minimum of today's
module (1 MiB stack + 233 KB data + TLS); a module whose data outgrows it fails to link, loudly.
Every boot grows past it, and dlmalloc uses only pages it grew itself, so a larger `I` would only
waste pages. ABI layout: top-level `memory: { initialPages, maximumPages }`
(`tools/parameter-metadata/src/abi_layout.rs`), `sdk/assets/miso-engine-v1-abi-layout.json` and
`sdk/src/generated/abi.ts` regenerated; `check-abi-layout-v1.py` checks the rows (maximum is 8192,
0 < initial <= maximum, default budget fits) with three red mutations (44 caught).
Gate 2: `each_boot_option_rule_has_its_own_typed_refusal` (`hosts/host-web/src/tests.rs`) refuses
M+1 bytes with `web.boot.memory_maximum\t$` and boots at exactly M.

**S2.** Build: `module_flags` in `scripts/build-web-audioworklet.sh` (`+simd128,+atomics,+bulk-memory,+mutable-globals`,
`--shared-memory --import-memory --export-memory --initial-memory=1310720 --max-memory=536870912`)
on the host-web line (shipped module and render-lock probe); blake3-web keeps `+simd128`. The page
counts are spelled again in the script (it must build without running cargo first: the mocked
builder contract and the identity twin's bare RUSTUP_HOME); the shape gate holds the module's
import limits equal to the generator's layout rows. Module: `Import[1]: memory[0] pages:
initial=20 max=8192 shared <- env.memory`, no own memory, `memory` exported, start function
`__wasm_init_memory`; the atomics warning prints once per build. Shipped module
`ecc05633131eb4a84ee84b12d07440a31e4382cc390c7ae941ff3518bd9d5e80` (6,120,735 B; main's
`98634c73...`, 6,119,208 B): ARTIFACT CHANGED (not re-pinned: no release).
Shape gate (gate 3): `check_module_shape` in `scripts/check-web-audioworklet.sh`; the opcode
policy no longer bans atomics; `--self-test-module-shape` (run by `test-web-audioworklet.sh`) is
red on six mutations: a second import, an unshared memory, other maximum, other initial, a defined
instead of imported memory, the `memory` export gone. Render closures unchanged against main's
module: `render` closure=35, traps=5, indirect_sites=11, members=4, destructor_registration=0,
atomic_waits=0; `meter_poll` closure=14, indirect_sites=2, atomic_waits=0.
Sites: one helper per realm. Worklet `sharedMemoryImports()` (constants `MEMORY_INITIAL_PAGES`/
`MEMORY_MAXIMUM_PAGES`); SDK and every Node script `sdk/src/core/memory.ts` (reads `ABI_LAYOUT.memory`);
the rebuild-cost processor inline from `layout.memoryPages`. The hermetic mock stubs
`WebAssembly.Memory` and requires every worklet instance to import exactly `{ env: { memory } }`
with the layout's `{ initial, maximum, shared: true }`; red when the worklet passes `{}` or a
4096-page maximum (both run). The SDK copies the two views it hands to `TextDecoder`
(`boundary.ts`), since a decoder need not accept a shared view; Node accepts one, so no SDK test
could see it.

**Gate 1 (same bits).** `direct-oracle.mjs` with `MISO_ENGINE_WEB_ORACLE_PRINT=1` over main's
closure and this one: the two outputs are byte-identical (every digest, status and memory word,
`memoryBytes` 1,900,544 in both). `check-browser-expected-resources.py --artifacts`: pass. The
benchmark preflight digests of all seven arms/documents are equal for both modules, and the
browser legs' structural-edit digest (`c51d28da...`) and control-allocation counts are equal.

**Browser legs** (local, private null PulseAudio sink, Playwright 1.62.1, `--check-matrix
--self-test-mutations`, source SDK bundle): chromium 151.0.7922.34 and webkit 26.5 "all
qualification gates passed" on the new closure; the main page is non-isolated and the PCM-feed
row runs on the isolated server origin, so the shared-memory module is exercised on both. Every
render-allocation row is 0. Main's closure through the same harness: same results; the only
differing words are the real-time rows' timing (elapsedMs, fedBlocks).

**Headroom (descriptive, Node 22.23.2, `--no-liftoff`, pinned to one core, one warmup, two
rounds, alternating modules).** `web-mixing-automation-benchmark.mjs run` p50 ns per block, main /
atomic: quiet 154865 / 154023 and 156899 / 158632; automated 177047 / 175454 and 177368 / 178760;
64-track console 242311 / 237853 and 245938 / 236490; app shape 127683 / 128425 and 128725 /
129296; sends 341459 / 336380 and 346610 / 335288. No headroom change beyond round-to-round noise
(within about 3 %).

**Other gates run:** see the final report of this attempt (lint job, host-web and
parameter-metadata debug tests, `realtime_policy`, `test-web-audioworklet.sh`,
`check-scalar-oracle-absent.py`, V8 spill gate, render-lock probe `[0,2,2]`, SDK job:
`check-sdk-generated`, `check-sdk-types` (686 pass), `check-sdk-headless`, `sdk-package.sh check`).
Not changed: `.github/workflows/qualification.yml:400`'s comment still says "no imports, no shared
memory, no atomics"; a workflow edit is outside this worker's scope.

**After merging `origin/main` at `1f0934b7e` (#2176, #1332 S5; no conflict).** Merged module
`2daf4fb375264991009b6c4b560297f45038e8df10b3b90f7a7a6e2209bd9239`; that main's own module
`e80d96fe...`. `direct-oracle.mjs` print output byte-identical between the two. Render closures
equal to that main's: `render` closure=38, indirect_sites=11, members=5, atomic_waits=0;
`meter_poll` closure=14, indirect_sites=2; no call-graph pin moved. Re-run on the merged tree, all
pass: the lint job's 51 steps (fmt, clippy `--all-features -D warnings`, `cargo doc -D warnings`,
every policy script incl. `check-lane-policy.sh`, the builder contract), host-web +
parameter-metadata debug tests (196 pass), `realtime_policy`, `check-web-audioworklet.sh`,
`test-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, scalar-oracle,
V8 spill, render-lock probe, the SDK job, and both browser legs (chromium 151.0.7922.34, webkit 26.5).
