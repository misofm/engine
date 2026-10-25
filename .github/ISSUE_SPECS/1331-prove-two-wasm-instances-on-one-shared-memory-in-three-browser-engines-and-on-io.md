# Prove two Wasm instances on one shared memory in three browser engines and on iOS

Stream H(a) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-8).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The owner and stream H get one recorded, reproducible answer before any browser control-plane code
is written: does the decision-15 browser shape work on every browser the product ships to? The
shape is one `host-web` module, built with a pinned dated nightly and `-Zbuild-std`, instantiated
twice on one shared `WebAssembly.Memory`: a Worker instance and an AudioWorklet instance. The
answer covers Chromium, Firefox and WebKit on desktop and Safari on a real iOS device, the
non-isolated fallback, memory growth, the largest shared-memory maximum iOS reserves, how much
faster than real time a Worker can render (render headroom), and the repo's parity gates on the
nightly compiler. This is a spike: it changes no shipped file. Its result is the decision record
below, and it gates *Run the browser control plane in a Worker and keep the AudioWorklet
render-only* (#1332).

## Context

- Today the whole engine runs in one instance inside the AudioWorklet, on unshared memory, built
  with stable 1.97.1 (`rust-toolchain.toml:2`). `scripts/check-web-audioworklet.sh` bans imports
  (`:363-366`), shared memory (`:372-375`), atomics (`:377-380`) and `Worker`/`Atomics`/
  `SharedArrayBuffer`/`memory.grow` in the worklet JS (`:519-522`).
- The shipped build is one cargo line in `scripts/build-web-audioworklet.sh:113-114`
  (`RUSTFLAGS="-C target-feature=+simd128 ..."`, `cargo build --locked --release --target
  wasm32-unknown-unknown -p host-web`). Release profile: `lto = "fat"`, `panic = "abort"`
  (`Cargo.toml:118-122`).
- `nightly-2026-08-20` is already pinned for fuzzing (`.github/workflows/fuzz.yml:99`,
  `nightly.yml:89`). Verified locally: it is `rustc 1.100.0-nightly (f7d782a3b 2026-08-19)`, LLVM
  23.1.0, with `rust-src` installable.
- Per-instance state that a second instance must not share: the six `thread_local!` statics in
  `hosts/host-web/src/ffi.rs:503-513` (`LIVE_HOST`, `NEXT_HANDLE`, `BOOT_STAGING`,
  `RESPONSE_STAGING`, `SPECTRUM_STAGING`, `OBSERVATION_STAGING`).
- The worklet compares `this.exports.memory.buffer !== this.memoryBuffer` in seven places
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1005, 1231, 1308, 1358, 1422, 1595, 1823`)
  and treats a change as fatal. With shared memory, another instance's growth changes it.
- Host memory default: `DEFAULT_MAXIMUM_MEMORY_BYTES = 512 << 20` (`hosts/host-web/src/lib.rs:115`).
- The first-party app sends `Cross-Origin-Embedder-Policy: require-corp` and
  `Cross-Origin-Opener-Policy: same-origin` (app repo `public/_headers:18-19`). Its isolation audit
  says iOS Safari was never checked (app repo `docs/mixer/isolation-audit.md:295-297`).
- Round-1 C4 and round-2 Q-C4 of the decision-15 plan review
  (`docs/handoffs/decision-15-2026-10-05/PLAN-2026-10-05-adversary-round1.md:296-302` and
  `PLAN-2026-10-05-adversary-round2.md:75-81`) record scratch probes in Chromium 151, Firefox 153
  and WebKit 26.5. They are not reproducible
  from the repo and did not run on iOS or run the repo's parity gates.
- Parity gates in the repo: G5 (`scripts/run-wasm-gates.sh`, `cargo test --locked --release -p
  wasm-gates --features math/lane`), the native/simd128 session digest
  (`hosts/host-web/src/tests.rs:1632`, `native_identity_session_digest_pins_the_wasm_parity`, paired
  with `hosts/host-web/tests/browser-v1/direct-oracle.mjs`), the browser legs' `native-corpus-digest`
  gate (`hosts/host-web/qualification/run.mjs:149-152`), and the AArch64 legs
  (`scripts/run-aarch64-tests.sh release`, `qualification.yml` jobs `aarch64-release`).

## Decisions frozen for this slice

- D1. **Rejected alternative, recorded, not re-run.** Stable `wasm32-wasip1-threads` was built and
  rendered correctly. It is rejected because each instance must drive wasi-libc internals by hand
  (the internal `__wasi_init_tp`, without which an instance spins in `__pthread_key_delete`), a
  second instance's first drop-carrying `thread_local` wrote through a null pthread-specific
  pointer into the first instance's stack, render's direct closure reaches
  `__wasilibc_futex_wait` and Chromium traps any wait in an AudioWorklet, and the target's platform
  support does not cover browsers. Time-slicing preparation in the worklet and transferring a
  prepared artifact between instances were measured and fail (round-1 C4).
- D2. **Build.** Toolchain `nightly-2026-08-20` with component `rust-src`. Command, in a scratch
  `CARGO_TARGET_DIR`: `cargo +nightly-2026-08-20 build --locked --release --target
  wasm32-unknown-unknown -Zbuild-std=std,panic_abort -p host-web` with
  `RUSTFLAGS="-C target-feature=+simd128,+atomics,+bulk-memory,+mutable-globals"` plus the link
  arguments the shape needs (`--shared-memory`, `--import-memory`, `--max-memory=<bytes>`, and the
  exports a second instance needs for its own stack and TLS, e.g. `__wasm_init_tls`, `__tls_size`,
  `__tls_align`). Record the exact final flag set; #1334 and #1332 adopt it verbatim. Also build the
  same command without `+atomics` and the shared-memory link arguments (the shape #1334 ships
  first) and record its digest.
- D3. **Instances.** The page creates one shared memory and passes it to a Worker and to the
  AudioWorkletProcessor (`processorOptions`). The worklet instance boots a session and renders it.
  The Worker instance, on the same memory, boots and disposes 64-track sessions in a loop
  (`fixtures/session/v1/console-sixty-four-track-app.json`) while the worklet renders. Each instance
  gets its own stack and TLS block. Exported Rust functions are used unchanged; the spike adds no
  Rust code. If an unchanged export cannot run in a second instance, that is a finding, not a fix.
- D4. **Pass criteria per engine** (all must hold):
  1. `crossOriginIsolated === true` on the page (the worklet cannot read it; the page posts it).
  2. The worklet's PCM digest over 350 blocks of the 64-track app session equals the same module's
     single-instance digest and the frozen native digest the browser legs use.
  3. No trap, no hang, no glitch counter increase in the worklet during the Worker loop (at least
     20 boot/dispose cycles).
  4. Growth: the Worker grows memory while the worklet renders. Record per engine whether old
     views stay valid and whether `memory.buffer` changes identity; the worklet re-derives views
     and keeps rendering with the same digest.
  5. The nightly module's render export closure (`check-web-audioworklet-callgraph.py --callgraph
     miso_engine_web_v1_render` on the named twin) contains no `memory.atomic.wait32/64`.
- D5. **Non-isolated page.** Same module, no COOP/COEP: the worklet creates its own shared memory
  and runs one instance. Record in each engine and on iOS whether creating a shared
  `WebAssembly.Memory` in the worklet succeeds and the digest matches. This is the decision-15
  fallback (same API, same artifact, blocking rebuild).
- D6. **iOS leg.** A real iPhone on the current iOS release, Safari, page served over HTTPS with
  COOP `same-origin` and COEP `require-corp` (for example the scratch server behind a Cloudflare
  quick tunnel). The page shows its result as one JSON object. Record: iOS and Safari versions,
  device model, D4 1-5, D5, and the largest of 512 MiB, 1 GiB, 2 GiB, 4 GiB maxima for which
  `new WebAssembly.Memory({initial, maximum, shared: true})` succeeds, each tried in a fresh page
  load and after opening three other tabs. This leg needs a person with the device.
- D7. **Render headroom (descriptive).** In each engine and on iOS, while the worklet renders the
  64-track app session in real time, the Worker instance renders a second copy of it as fast as it
  can for 10 s of audio. Report `headroom = rendered_audio_seconds / wall_seconds`. One warmup,
  two measured rounds, no retry (AGENTS.md benchmark rule). The numbers are the browser row's input
  to `PRIME_BYTES_MAX` (D15-8) in *Record the swap block's cost on the 64-track console* (#1286)
  D3; this spike sizes nothing.
- D8. **Parity gates on the nightly compiler.** Run, with `RUSTUP_TOOLCHAIN=nightly-2026-08-20`,
  the native and AArch64 gates listed under Objective gates, and the browser digest gates against
  the nightly modules. A digest difference is a finding; nothing is re-pinned.
- D9. **Outcome rule.** If every engine and iOS pass D4, #1332, #1333 and #1334 proceed as briefed.
  If iOS fails D4.1, D4.2 or cannot reserve 512 MiB shared, stop stream H(b) and report to root for
  a D15-10 re-ruling with the evidence. A desktop-engine failure is handled the same way.
- D10. **Where evidence lives.** The harness (page, Worker, processor, server) and raw logs go to
  `/home/bl/misofm/engine-work-evidence/1331/`, outside the repo. The decision record below is the
  committed result. No repo file other than this spec changes.

## Deliverables

1. The harness under `/home/bl/misofm/engine-work-evidence/1331/` with a one-line run command
   per leg.
2. This spec's "Decision record" section filled in: the exact build flags and digests (D2), the
   per-engine table for D4/D5, the iOS table (D6), the headroom table (D7), the parity gate results
   (D8), and the outcome under D9.

## Authorized paths

- This spec file only. Scratch builds and the harness stay outside the repo.

## Non-goals

- No change to `hosts/host-web`, `sdk/`, scripts or workflows (that is #1332, #1333, #1334).
- No handoff of a prepared plan between instances (that is #1332).
- No sizing of `PRIME_BYTES_MAX` or `P_MAX`.
- No Android device run (iOS is the named risk; Android Chrome shares Chromium's behaviour).

## Hazards

- Fat LTO plus build-std is slow; build once per flag set and reuse.
- build-std embeds the `rust-src` path in panic locations. Record whether two builds from different
  `RUSTUP_HOME` paths give the same digest; #1334 needs the answer.
- Chromium traps `Atomics.wait` in a worklet; any wait in the worklet's path is a D4 failure.

## Objective gates

1. Desktop legs: the harness passes D4 1-5 and D5 in Chromium, Firefox and WebKit at the Playwright
   versions pinned by `hosts/host-web/qualification/package-lock.json`.
2. iOS leg: D6 recorded from a real device.
3. Native gates under the nightly: `RUSTUP_TOOLCHAIN=nightly-2026-08-20 cargo test --locked
   --release -p lane -p math -p wasm-gates --features math/lane` and `RUSTUP_TOOLCHAIN=nightly-2026-08-20
   cargo test --locked -p host-web --lib tests::native_identity_session_digest_pins_the_wasm_parity`.
4. G5 under the nightly: a scratch copy of `scripts/run-wasm-gates.sh` whose guest build uses the D2
   command shape (both with and without `+atomics`) passes against the committed pins.
5. Browser bit identity: `python3 -B scripts/check-browser-expected-resources.py --artifacts <dir>`
   and the browser legs' `npm run qualify -- --artifacts <dir> --sdk-root <repo>/sdk --browser
   <b>` for each of chromium, firefox, webkit, against a closure built from the non-atomic D2
   module (copy `scripts/build-web-audioworklet.sh` to scratch and change only its cargo line).
6. **Dropped (root, 2026-10-24; see the decision record).** AArch64 under the nightly: `RUSTUP_TOOLCHAIN=nightly-2026-08-20 bash scripts/run-aarch64-tests.sh
   release` and `cargo test --locked --release -p wasm-gates --features math/lane` on arm64
   (hardware, or the emulated route the script header documents); record which.
7. Stable gates unaffected: `git status` shows only this spec changed.

## Test value

No test is added. The spike's value is the decision record; each gate above names the behaviour it
checks.

## Dependencies

- none.
- It blocks *Run the browser control plane in a Worker and keep the AudioWorklet render-only*
  (#1332) and *Build the browser artifact on a pinned nightly toolchain* (#1334).

## Decision record

### Rulings and scope (root, 2026-10-24)

- **Slices.** #1331a splits in two: **#1331a-1** (this record: D2 builds, the harness, D3, D4 and D5
  in Chromium, Firefox and WebKit; gates 1 and 7) and **#1331a-2** (D7 headroom, gate 3, gate 4 and
  gate 5 on the nightly). #1331b (iPhone: D6, D5 and D7 on iOS, gate 2) is owner-gated and not in
  this sprint.
- **Gate 6 and D8's AArch64 clause are dropped.** The nightly builds only the Wasm module, so no
  AArch64 artifact is built with it; #1334 gate 7 runs the stable AArch64 legs against the same pins.
  **Gate 3 stays** (the one native check of the nightly's LLVM 23 against the committed lane pins).
- **D9 is judged on the desktop record now.** A desktop pass lets #1334 start; #1380 and #1332 still
  wait for #1331b.
- Stale facts in Context, not fixed here: the stable pin is `1.99.1`; `hosts/host-web/src/ffi.rs`
  now has eight `thread_local!`s, all `const`-initialised; the worklet's buffer-identity checks
  became "re-bind in handlers, silence-and-count in `process()`" (#1381a).

### #1331a-1 result (2026-10-24, base `56fbe4617`): desktop PASS

Evidence: `/home/bl/misofm/engine-work-evidence/1331/` (README there has the one-line command per
leg). Harness v2, each browser leg run once. Comparator: no frozen native pin exists for this
session and transcript, so the spike computes one with a scratch native (x86-64-v3, stable `1.99.1`)
crate on `host-web`'s public API: `44ecef3380b86e8de00d553ca8dc8ad70f7e15911d974c13128760173581781f`
(sha256 over the first 350 output blocks, 256 LE `f32` words each, of the 64-track app session fed
a fixed integer-ramp stereo source).

**D2 builds.** `rustc 1.100.0-nightly (f7d782a3b 2026-08-19)`, LLVM 23.1.0, `rust-src`, installed
with `rustup toolchain install nightly-2026-08-20 --profile minimal --component rust-src --target
wasm32-unknown-unknown`. Command: `cargo +nightly-2026-08-20 build --locked --release --target
wasm32-unknown-unknown -Zbuild-std=std,panic_abort -p host-web` (about 50 s each), with:

- atomic (Worker shape): `RUSTFLAGS="-C target-feature=+simd128,+atomics,+bulk-memory,+mutable-globals
  -C link-arg=--shared-memory -C link-arg=--import-memory -C link-arg=--max-memory=536870912
  -C link-arg=--export=__stack_pointer -C link-arg=--export=__wasm_init_tls
  -C link-arg=--export=__tls_size -C link-arg=--export=__tls_align -C link-arg=--export=__tls_base
  -C strip=debuginfo <remap>"`. Shipped-shape (name section stripped) `b4bf75aa1843f83f13f9f64c25bcf9fe7b13c84fa026c730b9c0963e65b9040c`,
  5,090,925 B; named twin `730413304ccabf6cd7e3e1e81ec8dbab70db218721a942b1360a9835aae865e3`. One
  import, `env.memory` shared, 20..8192 pages; a start function `__wasm_init_memory`.
- plain (the shape #1334 ships first): `RUSTFLAGS="-C target-feature=+simd128,+bulk-memory,+mutable-globals
  -C strip=debuginfo <remap>"`. `0d32902855362282241053a901f7a552cb466334934aa42168c55217aad32e35`,
  5,090,257 B; named twin `e074b641729444a892032f98bdcbb98d3de8565f6e5de17e28c50208a80468f7`. No import,
  exports `memory`: today's ABI surface. Today's stable module is 5,093,283 B, so build-std costs no size.
- `<remap>` = the shipped script's two remaps (`$CARGO_HOME=/cargo`, repo `=/repo`) **plus**
  `--remap-path-prefix=$(rustc --print sysroot)/lib/rustlib/src/rust=/rust-src`. **RUSTUP_HOME
  hazard, answered for #1334:** without the third remap the digest depends on the toolchain's path
  (the same toolchain at two paths gave `c97fed27...` and `ae69c8c7...`); with it the copy at another
  path gives the same bytes as above. A rebuild is bit-reproducible.

**D3/D4/D5 per engine** (Playwright 1.62.1; `two` = isolated page, worklet + Worker on one shared
memory; `single-plain` = D5; `single-iso` = the same module's single-instance reference):

| Engine | D4.1 COI | D4.2 digest (two, single-iso, single-plain) | D4.3 Worker cycles: total / inside window | D4.3 glitches | D4.4 growth | D5 shared memory in worklet, non-isolated |
|---|---|---|---|---|---|---|
| Chromium 151.0.7922.34 | true | = native, all three | 32 / 31 | no trap, hang, render error, non-finite or render allocation; one 5-quantum skip at the 2nd `process()` call (before the window) and one 23 ms first rendered block: the same pair appears in both single legs | 2 explicit grows of 16 MiB inside the window plus allocator growth (6.5 -> 45.5 MiB); worklet saw 14 buffer changes | succeeds (`SharedArrayBuffer` undefined on page and in worklet), digest = native |
| Firefox 153.0 | true | = native, all three | 20 / 5 (boot ~150 ms; the worklet kept rendering, 1197 blocks, until the 20th) | none of the above; 0 skipped quanta; about 44 % of calls inside the window (155/350) read 3-5 ms by `Date.now()` (1 ms resolution), 36 % over the whole run (436/1216), and 47-48 % in both single legs: the session's own cost, not the Worker's | as Chromium, 14 changes | succeeds, digest = native |
| WebKit 26.5 | true | = native, all three | 57 / 56 | none; one 6 ms first rendered block (single legs: 5 ms + one 3 ms call, and 4 ms) | as Chromium, 13 changes | succeeds, digest = native |

D4.3 is judged as no increase over the same engine's single-instance legs; the glitch proxy has 1 ms
resolution. Late `process()` calls (over 2 ms) inside the window, `two` against `single-iso` /
`single-plain`: Chromium one 23 ms first block against one 16 ms / one 14 ms; Firefox 155 against
163 / 168; WebKit one 6 ms first block against two (5 ms, 3 ms) / one 4 ms.

D4.4 in all three engines: `memory.buffer` changes identity on every growth, on the growing side and
in the worklet; old views stay valid (an old `SharedArrayBuffer` keeps its length and reads the same
word at a probe index as the new view); the worklet re-derives its views and the digest holds. D4.5: the CI form of the render
call-graph gate (`--trap-owner ...enter_block --trap-owner ...observe=1 --render-thread`) passes on
both nightly twins, `atomic_waits=0`; the direct-call closure does not cover the plan executor's
`call_indirect` targets: these targets are not checked for atomic waits; a zero render-allocation
count only excludes the allocator lock on the paths that ran.

**Recipe rules found (#1332 adopts them).**

1. Instance order. lld's `__wasm_init_memory` (`--shared-memory`) initialises the data segments once
   with a `cmpxchg` flag and `memory.atomic.wait32`s when another instance is initialising, and
   std's allocator lock (`Mutex::lock_contended`), `Once::call` and `RwLock::read_contended` are the
   module's other wait sites. Chromium traps a wait in a worklet (not exercised here), so the
   worklet instantiates either first or after the other instance's instantiation returned, and
   finishes its boot before the Worker loop starts; after boot it calls only render-locked exports,
   which never take the lock.
2. Second instance's stack and TLS. Its `__stack_pointer` starts at the first instance's stack top
   (1 MiB) and its `__tls_base` at 0, so before any other export runs the Worker sets both: the
   harness grows the memory by 17 pages from JS (dlmalloc never hands out pages it did not grow
   itself), calls `__wasm_init_tls(base)` and sets `__stack_pointer` to the end. Without it (Node
   negative control, `--skip-reserve`) the Worker's first boot traps `unreachable`. This replaces the
   scope note's `document_ptr` idea: any export called before the stack is set runs on the
   worklet's stack, concurrently with its render. No Rust code is added.
3. `CONTROL_ALLOCATIONS` and `RENDER_ALLOCATIONS` (`hosts/host-web/src/render_lock.rs`) are plain
   statics, so on shared memory both instances count into one copy (the Worker read 6-17 million
   control allocations, its own plus the worklet's boot). Only the render flag is per instance. The
   module doc's "one instance owns one copy" holds only on unshared memory; #1332 must report the
   counters per instance or say they are shared.
4. In every engine a non-isolated worklet can create a shared `WebAssembly.Memory` although
   `SharedArrayBuffer` is not exposed, so D5's fallback runs the same atomic artifact.

**Harness history (candid).** The first Chromium run (harness v1, kept as `results/harness-v1/`)
passed every digest but started the Worker loop and its explicit growth before the worklet's window,
so D4.4 was not exercised while rendering; v2 makes the Worker wait for the window, grow twice inside
it and record glitches by phase, and all nine legs ran once on v2. The first v2 Chromium `two`
launch died on `EADDRINUSE` in the runner's own server before a browser started; the runner now
takes a kernel-chosen port and that leg ran once.

**Outcome under D9 (desktop part).** Every desktop engine passes D4 1-5 and D5: #1334 may start.
Open for #1331a-2: D7 headroom (Firefox's ~3 ms readings per 2.67 ms quantum on this 64-track
session make it the engine to watch) and gates 3-5. #1380/#1332 still wait for #1331b.

### #1331a-2 result (2026-10-25, base `b0bc367af`): gates 3, 5 PASS; gate 4 plain PASS, atomic not passed (runner); D7 recorded

Evidence: `/home/bl/misofm/engine-work-evidence/1331/a2/` (README there). Toolchain for every
nightly step: `nightly-2026-08-20` = `rustc 1.100.0-nightly (f7d782a3b 2026-08-19)`, the same
install as #1331a-1. The host runner, the native comparator and `parameter-metadata` stay on the
repo's stable pin.

**Rebuild at this base.** #1331a-1's modules refuse today's fixture (`boot_result` 1): #1369a moved
the session grammar to `link_mode: "slot"` after `56fbe4617`. The D2 commands, unchanged, rebuilt
both modules: atomic `ff266dfc456573263cdcea57c99e83985a9d2f5bf9eaffcb94e9f18ffe6ed022` (5,626,039 B; named twin
`db6e0aff...`), plain `4a7dc7eedea7f022fa71a74acf0a3a94db4afa53eef86673efaf60e9320dd964` (5,624,549 B; named twin
`cf17b597...`). The gate-5 script copy built the same plain bytes in another target dir. Note for
#1380: the atomic shape prints `unstable feature specified for -Ctarget-feature: atomics` once per
build; it does not fail under `-D warnings`, and `-A unstable_features` does not silence it
(`a2/logs/atomics-warning.log`: a trivial crate, all three forms print it once and exit 0).

**D7 headroom (descriptive).** Workload frozen before timing (`a2/workload/FROZEN.txt`): the atomic
module, and the 64-track app session with one change, `fixture-source` declared 1,441,280 frames
(30 s) instead of 48,000. Reason: a design probe in Node (`a2/probe/silence-probe.mjs`, untimed)
showed that render after the 1 s region ends costs less (silence skipping): the logged run
(`a2/logs/silence-probe.json`) reads 0.259 ms per block inside the region and 0.095 ms per block
after it, about 2.7 times less (a first, unlogged run read about 3.4). So a 10 s span on the
fixture as is would measure silence. The isolated `two` page: the worklet renders the session in real time. The Worker
instance (own stack and TLS, recipe rule 2) boots a second copy and renders it as fast as it can:
one warmup and two measured spans of 3750 blocks (10 s), back to back on one boot. A block is the
worklet's own transcript step (feed one quantum through `source_submit`, render, copy 256 words
out). `headroom = 10 s / span wall time`, `performance.now()` in the Worker. One invocation per
browser, no retry, under the exclusive bench lock.

Headroom at fs_h = 48 kHz, q = 128 frames:

| Engine | warmup | round 1 | round 2 | Worker boot | worklet during the spans: calls, late (> 2 ms), missed quanta, max |
|---|---|---|---|---|---|
| Chromium 151.0.7922.34 | 14.75 | 16.13 | 16.36 | 191 ms | 712, 0, 0, 1 ms |
| Firefox 153.0 | 1.10 | 1.10 | 1.10 | 150 ms | 10219, 4381, 0, 4 ms |
| WebKit 26.5 | 13.13 | 13.67 | 13.69 | 38 ms | 834, 0, 0, 1 ms |

All spans: 0 render errors, 0 non-finite words, 0 silent blocks; worklet render allocations 0, no
trap. In all three engines, the worklet's first 350 blocks gave the #1331a-1 native digest
`44ecef33...` (the longer source does not change them). In the Worker, Firefox renders this
session about 15 times slower than Chromium. The Firefox Worker uses about 0.91 of real time per
block (1/h). About 43 % of the worklet's calls during the spans read over 2 ms (4381/10219), and
#1331a-1 saw 44 % in `two` and 47-48 % in both single legs on unshared memory, so the slowdown is
not specific to the Worker or to the second instance. The cause is not investigated, and the slowdown
can be a property of the Playwright Firefox build only. Firefox h = 1.10 gives m·h = 0.77 ≤ 1,
which triggers #1286 D3's stop: #1286 D3 derives no browser value and its slice stops for an
owner ruling. These
rows are the browser input to #1286 D3; this spike sizes nothing.

**Gate 3 (native under the nightly): PASS.** `RUSTUP_TOOLCHAIN=nightly-2026-08-20 cargo test
--locked --release -p lane -p math -p wasm-gates --features math/lane`: 35 test binaries, 128
passed, 0 failed, 7 ignored, no warning. `RUSTUP_TOOLCHAIN=nightly-2026-08-20 cargo test --locked
-p host-web --lib tests::native_identity_session_digest_pins_the_wasm_parity`: 1 passed.

**Gate 4 (G5 under the nightly): plain PASS; atomic not passed as written (runner).** A scratch copy of
`scripts/run-wasm-gates.sh` (diff in `a2/gates/`) builds the guest with the D2 shape
(`cargo +nightly-2026-08-20 ... -Zbuild-std=std,panic_abort -p wasm-gate-guest`, the D2 flags and
the three remaps; `--without-v8-spill`).

- plain: native leg (the stable host runner) and wasm leg 0 mismatches (155 cases; 385 and 270 comparisons), detector
  residency 21 limiter functions, both f64 lane censuses vectorised: the script prints `ok`.
- atomic: NOT PASSED as written. The committed runner cannot load the guest: wasmtime refuses it
  with `threads must be enabled for shared memories`, because the guest imports `env.memory` as
  shared, 17..8192 pages.
- Under the preserved shim (`a2/gates/runner-threads-shim.diff`: `Config::wasm_threads(true)`,
  `Config::shared_memory(true)`, wasmtime feature `threads`, a `SharedMemory` for the import; run in
  a detached scratch worktree, now removed), the atomic guest has 0 mismatches over 270
  comparisons, residency 21, and both censuses green. The plain guest also passes under the shim.
  The runner change (threads plus the `SharedMemory` import) goes to #1380, the first slice that
  ships `+atomics`; root amends #1380's authorized paths.

**Gate 5 (browser bit identity on the plain nightly module): PASS.** A scratch copy of
`scripts/build-web-audioworklet.sh` built the closure. Only its cargo line changed, plus its
`repo_root` line, because the copy is outside the repo. `check-browser-expected-resources.py
--artifacts`: every expected.json digest and exact row agrees, budgets hold, self-test 43 red
mutations. `npm run qualify -- --artifacts <closure> --sdk-root <worktree>/sdk --browser <b>`:
`all qualification gates passed` in Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5. WebKit
runs on this Linux box (Playwright's WebKit 26.5 build), so WebKit has no stated limit here. The
first Chromium launch stopped before any page loaded (`Socket path too long`: the harness had set
`TMPDIR` to a long scratch path). It ran once more with the default `TMPDIR` and passed. Also run,
but not gates of this spec: the shape gate `check-web-audioworklet.sh
--without-metadata-regeneration` and the V8 spill gate pass on the nightly plain closure and its
named twin.

**Gate 7: PASS.** Only this spec changed in the repo.

**Outcome.** Gates 3 and 5 pass; gate 4 passes for the plain shape and is not passed for the
atomic shape as written (runner; green under the shim). The nightly keeps every committed native,
G5 and browser pin on the plain shape, so #1334 may adopt the plain D2 command as recorded. Open:
(1) the cause of Firefox's slow render (every mode, unshared memory too; maybe the Playwright
build only). Owner: root routes it as an owner question and a successor diagnosis issue; until
then #1286 D3 derives no browser value and its slice stops for an owner ruling. (2) G5 for the atomic shape needs the runner change, in #1380.
#1331b (iPhone) stays owner-gated, and D7 on iOS is part of it.

## Amendment (decision 17, root 2026-10-13)

**Split (root, 2026-10-13).** Both pieces stay in this issue.

- **#1331a, the desktop legs.** D1-D5, D7 and D8 in Chromium, Firefox and WebKit, and the
  decision record (the spike's evidence: build flags and digests, the D4/D5 table, the desktop
  headroom table, the parity results). Gates 1 and 3-7. It runs when a lane is free; it is off the
  critical path, because the browser ships `single` mode first and the Worker deployment (#1331a
  -> #1334 -> #1380 -> #1332 -> #1387) lands after #1565A.
- **#1331b, the iOS leg.** D6, D5's iOS record and D7 on iOS. Gate 2. It is an owner-scheduled
  human step: it needs a person with a real iPhone. It is not in the post-sprint heavy-gate
  successor (#1605). D9's iOS clause is judged on its record: an iOS failure stops the Worker
  deployment (#1332) for a decision 15 re-ruling; `single` mode is not affected.

The issue closes when both pieces have their records.
