# host-web: every shared memory declares 512 MiB, so an iPhone cannot boot a session whose budget is far smaller

Every shared `WebAssembly.Memory` the browser hosts and the SDK create declares the module's whole
maximum, 8192 pages (512 MiB), whatever boot budget the caller set. WebKit reserves a shared
memory's maximum up front, because a shared buffer can never move. On iOS that address space is
scarce, and a page that holds two or three such memories at once fails with
`RangeError: Out of memory` before the engine allocates anything.

## Evidence
- An app's 40-stem mixer (4 console slots per strip, a reverb return, a master limiter) fails at
  `createEngine` on an iPhone, Safari 27.2 (iOS 27), with `RangeError: Out of memory`, also after
  every other tab is closed. The PCM streams from OPFS.
- On desktop Chromium the same session's one worker-mode memory is 34.9 MB after boot, while
  playing, and with an EQ response preview open.
- The page then holds up to three 512 MiB reservations: the scratch boot Worker's, the worker-mode
  control memory, and the response preview Worker's.
- Five sites declare 8192 pages: the worker-mode control memory
  (`miso-engine-v1-audio-worklet-host.js`), the single-mode worklet memory
  (`miso-engine-v1-audio-worklet.js`), and `sdk/src/core/memory.ts`, which the scratch boot
  (`MisoEngineAsset.instantiate`) and the response preview Worker use.

## Contract
- Every site creates its memory with
  `maximum = min(memory.maximumPages, memory.initialPages + ceil(budget / 65536) + 34)` when a
  budget is given, and `memory.maximumPages` otherwise.
  - The budget charges growth past the initial pages, which hold the first instance's stack, the
    static data and the TLS block (`check-web-boot-budget.mjs` measures growth).
  - 34 pages are the bytes no budget charges: a worker-mode instance reservation (a 2 MiB stack,
    its 64-byte canary and its TLS block, 272 bytes in today's module; 33 pages) and the one page
    of growth past the budget the boot gates allow.
  - So the memory can grow as far as the engine admits.
- The budget is the boot policy's `maximumMemoryBytes`. Absent or `0n` keeps 8192 pages, so the
  default is unchanged. A budget above the module's maximum still clamps to 8192 pages, and boot
  still refuses it with `web.boot.memory_maximum`.
- Worker mode: the page realm's one memory follows the boot options. Single mode: the worklet's
  memory follows its boot options. The scratch boot follows `scratchBootOptions(policy)`, the same
  policy word.
- A headless reboot (`loadSession`, `WasmBoundary.reboot`) runs on the same instance, so on the
  same memory. A reboot whose budget needs more pages than that memory declares is refused,
  typed (`refusedOptions`, `sdk.boot.memory_maximum`), rather than admitted and trapped later in
  `memory.grow`.
- The response preview has no boot budget. `BrowserResponsePreviewOptions` gains
  `maximumMemoryBytes?: bigint`, which bounds its Worker's memory by the same rule. A query that
  outgrows it fails like any other trap in that Worker.
- The module is unchanged. It imports `env.memory` with maximum 8192, and an imported memory may
  declare a smaller maximum. No Rust source changes, so the shipped digest does not move.

## Gates
- SDK: `sharedMemoryImports(budget)` declares the rule's maximum at `0n`/absent, at one byte, at a
  page boundary, and above the module's maximum. A real module instantiates and boots on a memory
  capped at a 64 MiB budget, a 1-byte budget is the engine's typed `refusedBudget`, and a reboot
  at the default budget on that instance is `sdk.boot.memory_maximum`.
- host-web hermetic suite (`test-web-audioworklet.mjs`): a single-mode boot with a budget creates
  its memory at the rule's maximum, and the worker-mode boot's page-realm memory does too.
- SDK response evals: the response Worker's init carries the preview's budget.
- The PR's `artifact-identity` reads `ARTIFACT UNCHANGED`.

## Non-goals
- Changing the module's declared maximum or the default budget.
- A memory budget for the response preview's own limits.

## Evidence / attempt record

### Attempt 1 (2026-11-21)
- Five sites follow the budget: `miso-engine-v1-audio-worklet-host.js` (worker mode, the page
  realm's memory), `miso-engine-v1-audio-worklet.js` (single mode), and `sdk/src/core/memory.ts`
  (`sharedMemoryMaximumPages`), which `MisoEngineAsset.instantiate(budget)` (scratch boot, headless
  boot) and the response Worker use. `BrowserResponsePreviewOptions.maximumMemoryBytes` is new and
  rides the Worker's `response-init`; a value that is not a u64 `bigint` is a `MisoUsageError`.
- `WasmBoundary.reboot` refuses a budget whose pages exceed its instance's memory
  (`refusedOptions`, `sdk.boot.memory_maximum`), after disposing the old session, as an engine
  refusal of the options does.
- No Rust source changes. The module built from this tree with `nightly-2026-08-20` is
  `234130dd00d9bfe1c20eaa2117ba6205d84dc07c6e3002b99095d29b894e2839`, the digest main d568605c1's
  CI recorded. The module's TLS block is 272 bytes (`__tls_size`), inside the 33 reservation pages.
- Local gates: `check-sdk-headless.sh` (719 pass, 0 fail), `test-web-audioworklet.mjs`,
  `check-sdk-generated.sh`, `check-sdk-types.sh`, `check-workspace-policy.sh`, `tsc --noEmit`.
- Mutants, each red: the host's and the worklet's `memoryMaximumPages` returning the module's
  maximum for every budget (hermetic suite red on its worker-mode and single-mode rows).
- Test value:
  - `boot-evals` "#2431": a memory that declares 512 MiB whatever the budget, a cap that drops the
    initial or unbudgeted pages or rounds down, or a reboot that admits a budget its memory cannot
    reach.
  - `response-evals` "#2431": a preview that drops its budget on the way to the Worker, or a Worker
    whose memory ignores it.
  - Hermetic rows: a worker-mode or single-mode host whose memory ignores the boot budget.
