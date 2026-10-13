# Close the remaining render-lock proof gaps in the realtime-policy tool

Stream J3 of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10).
Successor of #1496 (attempt 2 FAIL, `/home/bl/misofm/submix-verdicts/1496-attempt2.md`: F1/F2, SP-1, SP-3). A slice of the realtime-policy tool
program (#1438 lists the slices). Reproducers: `/home/bl/misofm/submix-verdicts/evidence/1496-attempt2/` (`probes.py`, `*.diff`).
Sibling: the issue "Prove a real allocation inside a render-lock window is counted, natively and in the shipped wasm module" (call it B) adds the
runtime proof; this issue is the static tool and manifest side. Neither blocks the other.

## Product outcome

For every export the worklets call after construction, the shipped module's exported symbol is the listed, wrapped function and nothing else; and
`render_locked`, `locked`, `count_if_locked` and the counting allocator cannot be swapped for a pass-through by any written `cfg` form.

## Problem (verified on `79d61373b`, branch `codex/d15-stream-j3`, which contains #1496 attempt 2 at `8fd895eb2`)

1. **F1/SP-2: a listed export can lose its symbol to an unwrapped twin; tool and all gates pass.** Two compiling forms, no macro: (i) the listed fn made
   generic (`impl Into<u32>` argument, `#[allow(no_mangle_generic_items)]`) emits no symbol, and a child-module `#[unsafe(no_mangle)]` fn with the listed
   name takes it (`generic_command_submit_twin.ffi.diff`, `generic_original_twin.ffi.diff`); (ii) `#[unsafe(export_name = "retired_source_seek")]` on the
   listed fn (a value without the prefix passes rule (b)) plus a child-module twin (`export_name_override_twin.ffi.diff`). The shipped module then exports an
   unwrapped `command_submit`; the release tool, clippy (native and wasm), `check-web-audioworklet.sh`, `test-web-audioworklet.sh` and SDK headless
   (363/363) all passed (form (ii) is caught only by the frozen export-set check, by its extra name).
2. **SP-1: rule (a) does not reach `render_locked`'s own definition.** `render_lock_cfg_split.diff`: `#[cfg(test)]` on the real `pub(crate) fn render_locked`
   (`render_lock.rs:126`) plus a `#[cfg(not(test))]` pass-through. Tool silent, clippy clean, the unit test (runs under `cfg(test)`) sees the real fn, and
   `render_locked_staging` and `render_locked_source` pass because they assert the count stays 0. The `#[global_allocator]` is
   `#[cfg(all(target_family = "wasm", not(test)))]` (`render_lock.rs:117`, static :118-120); changing it unregisters the counter and every zero gate passes.
   Existing rule (a) reads only the `mod` lines in `lib.rs` and inner attributes of `render_lock.rs` (`web_host.rs` `scan_modules`).
3. **SP-3: the tool assumes `src/lib.rs` is the crate root.** `hosts/host-web/Cargo.toml` `[lib]` holds `name = "host_web"` and
   `crate-type = ["rlib", "cdylib"]`, no `path`. A `path = "src/other.rs"` passes the #1519 manifest rule (relative, inside the package) and moves the root
   away from the `ffi.rs` and `render_lock.rs` the tool reads.
4. **Measured on `79d61373b` (0 real refusals for each rule below).** 33 listed exports (`render-thread-exports.json`), each defined once as a top-level
   `pub extern "C" fn` of `ffi.rs` with `#[unsafe(no_mangle)]`, none generic; `export_name`: 0 and `cfg_attr`: 0 under `hosts/host-web/src` (5 files);
   `no_mangle`: 123, all in `ffi.rs`; `fn render_locked`: 1 (`render_lock.rs:126`); `cfg!`: 0 in `render_lock.rs`; `#[global_allocator]`: 2 (`render_lock.rs:118`;
   `ffi.rs:4896` inside `#[cfg(test)] mod live_response_ffi_tests`); `cfg` in `render_lock.rs`: exactly :117 (the allocator) and :139 (`#[cfg(test)] mod tests`).

## Decisions

- **D1 (F1/SP-2). Three checks in `tools/realtime-policy`** (`render_wrap.rs`, `web_host.rs`), each a new finding class:
  - (a) a listed export with any generic parameter (type, const, lifetime), a `where` clause, or `impl Trait` in an argument or the return type (nested included) is refused.
  - (b) every `export_name` attribute under `hosts/host-web/src`, whatever its value or spelling (`unsafe(..)`, `cfg_attr(.., ..)`, nested). It replaces the prefix test of
    `ExportNameOnWebExport`; the accepted `export_name = "memory_probe"` case becomes refused.
  - (c) a fn named like a listed export (prefixed) that carries `no_mangle` in any spelling (`#[no_mangle]`, `#[unsafe(no_mangle)]`, inside `cfg_attr`), in any module, `impl` or trait block or
    fn body of any file under `hosts/host-web/src`, is refused unless it is the top-level item of `ffi.rs`. A second top-level definition stays `ExportDefinedTwice`.
  - Fix the F2 wording: the `ExportNameOnWebExport` message and the `web_host.rs` module doc (which says "not enforced") claim only what (a) to (c) enforce; the attempt-2 test-value sentence for the export-name test is replaced.
- **D2 (SP-1). The tool refuses `cfg` and `cfg_attr` on every top-level item of `hosts/host-web/src/render_lock.rs`** (fn, static, const, struct, `impl`, `use`, `mod`, macro item such as `thread_local!`), so
  `render_locked`, `locked`, `count_if_locked`, the `GlobalAlloc` impl and the counters are covered. Allowed, compared on the parsed attribute and predicate, never on text:
  1. **The allocator.** The one `#[global_allocator]` item: its attributes, apart from `doc`, are exactly `#[cfg(all(target_family = "wasm", not(test)))]` and `#[global_allocator]`; its type is
     `RenderLockedAllocator<std::alloc::System>`; it is a top-level item of `render_lock.rs`. This is the only `cfg` allowed on that item. A `#[global_allocator]` elsewhere is refused unless it sits in a module
     marked `#[cfg(test)]` (the one in `ffi.rs`).
  2. **The unit-test module (root decision, 2026-10-12: two exact allowed forms: the probe item and `#[cfg(test)] mod tests`).** An inline `mod tests` whose attribute is exactly `#[cfg(test)]`. It is allowed by its exact form: `cfg(test)` removes code from the shipped build and the module defines nothing the build uses.
     Any other item with `cfg(test)`, and `#[cfg(not(test))]` on anything, is refused.
  3. **The other exact allowed form: the probe item** (added by B, so it may be absent; absence is fine and nothing requires it). A top-level `pub extern "C" fn miso_engine_web_v1_render_lock_probe` whose attributes, apart from `doc`, are exactly
     `#[cfg(feature = "render-lock-probe")]` and `#[unsafe(no_mangle)]`. A different name, kind, predicate, extra attribute or a second such item is refused. B's item must match this spelling; the spelling is the contract between the two issues.
  - Beyond the top level, in `render_lock.rs` only: a `cfg` or `cfg_attr` attribute at any nested site (statement, `impl` item, field, argument) and a `cfg!` invocation are refused, outside the allowed `mod tests`. Measured: 0 today. (Root's ruling names top-level items; this closes `#[cfg(..)]` on the `set(true)` statement inside `render_locked`.)
  - `render_locked`: a top-level fn of `render_lock.rs`; no other item named `render_locked` (fn, macro, const, static, `impl` method, nested fn, in a test module too) exists in any file under `hosts/host-web/src`. An `ffi.rs` use of the name stays judged by the shadow scan.
- **D4 (SP-3). `[lib] path` is absent or exactly `src/lib.rs` in `hosts/host-web/Cargo.toml`.** Home: the `manifest_target_checker` (`python3` `tomllib`) in `scripts/check-workspace-policy.sh` (#1519): one added condition, one violation status, one self-test case in
  `scripts/test-workspace-policy.sh` per spelling (`[lib] path`, `lib = { path = .. }`); `./src/lib.rs` is refused (exact string). Today: absent, 0 refusals. The tool cannot read it: its Rust walk reads no manifests, and the policy script already parses every tracked one.

## Cases (each refused unless marked accepted), test value and mutation

Cases live in `tools/realtime-policy/tests/policy/{web_host,render_wrap}.rs`; each mutation is applied alone, red only on its case, then reverted.
- D1(a): `generic_command_submit_twin`, `generic_original_twin`; `impl Into<u32>` alone, a lifetime parameter, a `where`. Test value: a listed export that defines no symbol while an unwrapped twin takes the name (the attempt-2 tool passed it). Mutation: ignore `sig.generics`; ignore `impl Trait`.
- D1(b): `export_name_override_twin`; an unprefixed value in `unsafe(..)`, in `cfg_attr(.., ..)`, in a nested module of `control_targets.rs`. Test value: an exported name that does not come from a listed fn's own ident. Mutation: restore the prefix test (the unprefixed cases turn red).
- D1(c): the child-module twin of each reproducer; an `impl`-block twin (`probes.py` `impl_block_twin_*`); `#[no_mangle]` in `cfg_attr`; accepted: the real top-level exports, and a non-`no_mangle` fn with a listed name in a child module. Test value: a second definition that the generic or renamed original no longer collides with. Mutation: scan `ffi.rs` top level only; ignore `cfg_attr`; refuse every listed-name fn.
- D2 refused: `render_lock_cfg_split` (cfg on the fn); `cfg_attr` on it; `cfg` on `locked`, on `count_if_locked`, on the `GlobalAlloc` impl, on `RENDER_ALLOCATIONS`; `#[cfg(any())]` on the `set(true)` statement; `cfg!` in a body; `#[cfg(test)]` on a fn; `#[cfg(not(test))] mod tests`; a second `fn render_locked` in `lib.rs`, in `control_targets.rs`, inside an `impl`; the allocator under `all(target_family = "wasm", test)`, under `target_family = "wasm"`, with a second `cfg`, typed `System`, in `lib.rs`; the probe with predicate `feature = "other"`, with another name, as a `static`, with an extra `cfg`.
  Accepted: the real `render_lock.rs` (allocator, `mod tests`); the same file with the exact probe item added; the same file without it. Test value: a pass-through or an unregistered counter while every count-stays-zero gate is green. Mutation: drop each sub-rule alone (top-level cfg, nested cfg and `cfg!`, second definition, allocator predicate, allocator type and placement, test-module form, probe spelling).
- D4: `path = "src/other.rs"` (red), `path = "src/lib.rs"` and absent (accepted), the inline spelling, `./src/lib.rs` (red). Test value: a moved crate root that makes the tool read a file that does not ship.

## Authorized paths

`tools/realtime-policy/**`; `scripts/check-workspace-policy.sh` and `scripts/test-workspace-policy.sh` (D4 only); this spec. No change to `hosts/host-web`, a worklet, a header or a workflow.

## Non-goals

No runtime proof that the allocator counts (B). No export, worklet or header change. Not macro-assembled exports, `cfg` or names (#1438 limit 4); not `RUSTFLAGS --cfg` (#1527); not a cfg split in any other file (`ffi.rs` keeps #1496 rule (a)).

## Hazards

- #1438 threat model: fail closed, over-refusal acceptable. The tool tests presence of `cfg`, never evaluates it. Limit 4 (names or attributes an expansion assembles) and limit 1 stay open; a verifier finding inside them is not a FAIL.
- The `cfg(test)` module and the nested-site extension were decisions for root. Root ruled 2026-10-12: two exact allowed forms (the probe item and `#[cfg(test)] mod tests`); the nested `cfg`, `cfg_attr` and `cfg!` refusals stay.
- Landing order with B: the probe exception is for a named item that may be absent. A first: the tool tests use a synthetic tree with and without the item, the real-tree gate passes without it. B first: the real tree carries the item and A's tool accepts it only by the exact spelling above, so B's CI is green only if the spellings agree. Either order, no step waits for the other.

## Objective gates

1. Real tree: the tool prints only what it prints at `79d61373b` (`crates/effect-contract/src/live.rs:366` until #1345 D8); `cargo test --locked -p realtime-policy`.
2. Each refused case red on its reproducer, with the verifier's three tree-level diffs applied to the real tree; revert green.
3. Mutations of "Cases", each alone, recorded as PR evidence.
4. `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`, `python3 scripts/check-ci-path-routing.py`.

## Dependencies

After: the J batch (J3 with #1496 attempt 2 and #1519 attempt 3, the `manifest_target_checker`) on `main`. Before: nothing. Independent of B.

## Standing rules

Work from this body. A test that greps source is refused; each new test answers which plausible defect it turns red. Attempt budget: three, one adversarial verdict each. Size: one day.

