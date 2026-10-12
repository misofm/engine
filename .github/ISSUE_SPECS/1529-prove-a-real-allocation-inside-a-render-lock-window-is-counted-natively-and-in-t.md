# Prove a real allocation inside a render-lock window is counted, natively and in the shipped wasm module

Stream J3 of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10).
Successor of #1496 (attempt 2 FAIL, `/home/bl/misofm/submix-verdicts/1496-attempt2.md`: SP-1). Reproducers: `/home/bl/misofm/submix-verdicts/evidence/1496-attempt2/`
(`render_lock_cfg_split.diff`). Sibling: the issue "Close the remaining render-lock proof gaps in the realtime-policy tool" (call it A) makes the tool refuse the written
forms; this issue adds the runtime proof that catches any form the tool cannot see. Neither blocks the other.

## Product outcome

A gate shows that an allocation inside a real render-lock window moves the count, in the native build and in the browser module built from the same sources, so a
"count stays zero" gate cannot pass with nothing counted. The shipped module is byte-for-byte unchanged.

## Problem (verified on `79d61373b`, branch `codex/d15-stream-j3`, which contains #1496 attempt 2 at `8fd895eb2`)

1. **Every render-allocation gate is a zero gate with no positive control.** `render_locked_staging.rs` and `render_locked_source.rs` (and the browser legs that read
   `miso_engine_web_v1_render_allocation_count`) assert the count stays 0. `render_locked_cfg_split.diff` (`#[cfg(test)]` on the real `render_locked`, `render_lock.rs:126`, plus a
   `#[cfg(not(test))]` pass-through) leaves all of them green, because the only test that sees the real fn runs under `cfg(test)`. The same holds for the browser module's allocator:
   `#[cfg(all(target_family = "wasm", not(test)))] #[global_allocator] static ALLOCATOR: RenderLockedAllocator<std::alloc::System>` (`render_lock.rs:117-120`); changing its cfg
   unregisters the counter and every zero gate passes with nothing counted.
2. **The native integration binaries cannot see the wasm allocator.** They register their own `RenderLockedAllocator<System>` (`render_locked_source.rs`: `#[global_allocator]`), because the
   native rlib must not register a second one. So a native leg proves `render_locked`, `locked`, `count_if_locked` and the `GlobalAlloc` impl compiled without `cfg(test)`, and says nothing
   about the cfg on the wasm registration. Only a leg on the shipped module's build reads that.
3. **Measured on `79d61373b`.** `render_lock.rs` has `cfg` only at :117 and :139 (`#[cfg(test)] mod tests`); `mod render_lock;` is private in `lib.rs:7444` (only `RenderLockedAllocator` is re-exported, :7447), so
   no integration test reaches a function of that file. `hosts/host-web/Cargo.toml`: `[features]` has `default = []` and `test-support` only; no `[[test]]`. In `qualification.yml` the job that runs
   `scripts/check-web-audioworklet.sh` is **`artifact-gates`** ("AudioWorklet artifact gates", `timeout-minutes: 10`, shared-key `artifact-gates`): it installs toolchain `1.97.1` minimal and `wabt`, and does
   not add the `wasm32-unknown-unknown` target (the `artifact` job does, :114). The shipped build is `cargo build --locked --release --target wasm32-unknown-unknown -p host-web`, stable, `RUSTFLAGS="-C target-feature=+simd128 ..."`
   (`scripts/build-web-audioworklet.sh:113`); the module has no imports. `--features ...host-web/test-support,...` appears in `test-debug-a` at :632 and :645.

## Decisions

- **D3. A counted allocation in a real window, native and wasm.** Feature `render-lock-probe = []` (off by default) in `hosts/host-web/Cargo.toml`, and one item in `render_lock.rs` with exactly this spelling (the contract with A's tool exception):
  `#[cfg(feature = "render-lock-probe")]`, `#[unsafe(no_mangle)]`, `pub extern "C" fn miso_engine_web_v1_render_lock_probe(inside: u32) -> u32`. It allocates and frees a `Box` of a `black_box`ed value inside `render_locked` when `inside != 0`,
  outside any window otherwise, and returns `render_allocation_count()`. It is in no table, no worklet, no header and no shipped build. `lib.rs` gains one `#[cfg(feature = "render-lock-probe")] pub use render_lock::miso_engine_web_v1_render_lock_probe;`
  so the native test reaches it (decision for root: the alternative is an `extern "C"` declaration in the test, which depends on the linker keeping an unreferenced rlib symbol).
  - **Native leg:** `hosts/host-web/tests/render_locked_positive_control.rs` (`[[test]] required-features = ["render-lock-probe"]`; one test; registers `RenderLockedAllocator<System>` as `render_locked_source.rs` does): `probe(0)` leaves the count, `probe(1)` moves it by exactly 2
    (one alloc, one dealloc), `probe(0)` again leaves it. It compiles the lib without `cfg(test)`, so it sees the pass-through twin of SP-1. CI: add `host-web/render-lock-probe` to the two `--features` lists of `test-debug-a` (:632 workspace tests, :645 doctests).
  - **Wasm leg (the claim is about the shipped module):** `scripts/build-web-audioworklet.sh` gains `--render-lock-probe` (module only: the shipped cargo line plus `--features host-web/render-lock-probe`, its own target directory below the output directory,
    never pinned, never delivered, never uploaded; the cargo line keeps one home). `scripts/test-render-lock-probe.mjs` instantiates that module in Node (no imports) and asserts the same three calls on a fresh instance.
    It runs in the **`artifact-gates`** job, in a new step directly after "Hermetic browser host and worklet tests" and before `setup-node` (the runner's Node, as that step). The job also gains `rustup target add --toolchain 1.97.1 wasm32-unknown-unknown`.
  - **Stays out of the shipped module:** the item and the `pub use` exist only under the feature. Gate 5 measures the shipped digest; `check-web-audioworklet.sh` keeps its frozen export set, so a probe export in a shipped build is red there.

## Cases, test value and mutation

Each mutation is applied alone to the real tree, red only where stated, then reverted.
- Native `probe(1)` moves the count by exactly 2; `probe(0)` by 0. Test value: a counter that counts nothing (a pass-through `render_locked`, `locked` always false, `count_if_locked` empty), which every existing zero gate passes. Mutations: `render_lock_cfg_split.diff` (native red and wasm red); delete the `RENDER_LOCKED.set(true)` line (both red); empty `count_if_locked` (both red).
- Wasm `probe(1)` moves the count by exactly 2 on the module built with the feature. Test value: the browser module's `#[global_allocator]` is unregistered or differently cfg'd, which no native test can see. Mutations: change the allocator predicate to `all(target_family = "wasm", test)` (wasm red, native green: the native binary registers its own allocator); remove the `#[global_allocator]` line (wasm red).
- Shipped module has no `miso_engine_web_v1_render_lock_probe` export. Test value: the probe leaking into the product. Mutation: drop the `cfg(feature)` on the item (`check-web-audioworklet.sh` red on the export set, digest gate red).

## Authorized paths

`hosts/host-web/Cargo.toml` (feature, `[[test]]`); `hosts/host-web/src/render_lock.rs` (the probe item only); `hosts/host-web/src/lib.rs` (the one `pub use`); `hosts/host-web/tests/render_locked_positive_control.rs` (new);
`scripts/build-web-audioworklet.sh` (`--render-lock-probe` only); `scripts/test-render-lock-probe.mjs` (new); `.github/workflows/qualification.yml` (the two `--features` lists, and in `artifact-gates` the target install, the new step, and a `timeout-minutes` change only if the measured wall time needs it);
`scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` (only if they pin the edited steps and go red on this edit alone); this spec. No `tools/realtime-policy` change.

## Non-goals

No export, worklet or header change; no product change beyond the probe item under a default-off feature. No tool rule (A). Not a cfg or allocator rule for `RUSTFLAGS --cfg` (#1527). No test of the allocator's entry points (the unit test `render_lock_counts_each_entry_point_only_inside_its_own_window` covers them).

## Hazards

- #1438 threat model: this gate is the runtime backstop for what a static tool cannot see (macro-assembled names or attributes, limit 4). It proves the build it runs on, not a different `RUSTFLAGS`.
- The feature compiles extra code into a crate whose `ffi.rs` set is frozen. Gate 5 is the proof it does not leak. Check that no script enumerates exports from source (`scripts/check-web-audioworklet-callgraph.py` mentions `no_mangle`) in a way that sees the probe under default features.
- A second wasm build in `artifact-gates` (timeout 10 min, shared cache only saved on `main`). The attempt record states cold and warm wall time of the step; the timeout moves only by the measured need.
- The count is one process-wide `AtomicU32`; the native binary holds exactly one test, so nothing else moves it. Windows do not nest: the probe calls no export.
- Landing order with A: A's tool accepts this item only by the exact spelling in D3. If A has not landed, nothing checks it; if A has landed, a different spelling is a tool finding in this issue's gate 1, and the fix is here.

## Objective gates

1. `cargo test --locked -p host-web --features render-lock-probe --test render_locked_positive_control` green; the wasm leg (`build-web-audioworklet.sh --render-lock-probe` then `node scripts/test-render-lock-probe.mjs`) green, locally and in `artifact-gates`.
2. Both legs red on `render_lock_cfg_split.diff` applied to the real tree; the wasm leg red on the allocator-predicate mutation (native stays green, recorded); revert green.
3. The mutations of "Cases", each alone, recorded as PR evidence.
4. The release realtime-policy tool run (the command #1496 gate 1 names) prints only what it prints at `79d61373b` (`crates/effect-contract/src/live.rs:366` until #1345 D8).
5. Shipped module unchanged: `build-web-audioworklet.sh --module-only` digest equals the base commit's (the `audioworklet-sha256` status of the base; `9784d3509dba..` at `8fd895eb2`, re-measured at the attempt's base); `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` (private TMPDIR), `check-sdk-headless.sh` pass; the `artifact-identity` job says UNCHANGED.
6. `cargo clippy --locked -p host-web --all-targets --features render-lock-probe -- -D warnings` and without it, `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`, `python3 scripts/check-ci-path-routing.py`, `python3 scripts/check-test-support-ci.py` (unaffected: the feature is not `test-support`).

## Dependencies

After: the J batch (J3 with #1496 attempt 2 and #1519 attempt 3) on `main`. Before: nothing. Independent of A, in either order.

## Standing rules

Work from this body. A test that greps source is refused; each new test answers which plausible defect it turns red. Attempt budget: three, one adversarial verdict each. Size: one day.

