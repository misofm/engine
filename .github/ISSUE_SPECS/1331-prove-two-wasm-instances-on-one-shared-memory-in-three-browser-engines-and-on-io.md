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
faster than real time a Worker can render (catch-up headroom), and the repo's parity gates on the
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
- D7. **Catch-up headroom (descriptive).** In each engine and on iOS, while the worklet renders the
  64-track app session in real time, the Worker instance renders a second copy of it as fast as it
  can for 10 s of audio. Report `headroom = rendered_audio_seconds / wall_seconds`. One warmup,
  two measured rounds, no retry (AGENTS.md benchmark rule). The numbers are inputs for sizing
  `k_max`, the timeout and `P_max` (D15-8) together with *Record the swap block's cost on the
  64-track console* (#1286); this spike sizes nothing.
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
- No sizing of `k_max`, the timeout or `P_max`.
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
6. AArch64 under the nightly: `RUSTUP_TOOLCHAIN=nightly-2026-08-20 bash scripts/run-aarch64-tests.sh
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

(Filled in by the spike.)
