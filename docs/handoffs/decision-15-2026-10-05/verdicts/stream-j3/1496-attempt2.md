FAIL

# #1496 attempt 2 verdict (last attempt, budget 2): prove that every export the worklets call after construction is render-locked

Range `d51654482..8fd895eb2` on `codex/d15-stream-j3`: `d8a1f734c` spec only, `8fd895eb2` code, tests
and the Attempt 2 record. I exported `8fd895eb2` to `/tmp/claude-1002/v1496b/tree` and built with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1496b/target`. For the attacks I used a second copy
(`/tmp/claude-1002/v1496b/atk`) and separate target directories. I did not touch the worktree.
The reproducers and scripts are in `/home/bl/misofm/submix-verdicts/evidence/1496-attempt2/`.

The attempt does what root's Amendment 1 says, letter by letter:
- The two attempt-1 reproducers are refused on the real `ffi.rs`.
- Rules (a) to (d) each have a case.
- All 18 recorded mutations go red, and only on the new tests.
- (d) has the scope root gave it.
- Every gate is green.

The attempt fails on F1. A new written form, with no macro, gets past every rule and every gate:
the listed export is made generic, so it has no symbol, and an unwrapped twin in a child module
takes its name. The tool passes, and the shipped module exports an unwrapped
`miso_engine_web_v1_command_submit`. Root's rule (b) states the invariant that this breaks
("every exported name comes from a listed fn's own identifier"), but the tool does not enforce it.
This is the last attempt, so the issue goes back to root for a rescope.

## MAJOR

- **F1. A listed export can lose its symbol to an unwrapped twin. The tool passes, and so does every
  gate.**
  - Where:
    - `tools/realtime-policy/src/render_wrap.rs:62-69` takes every top-level
      `#[unsafe(no_mangle)]` fn and does not look at its generics.
    - `web_host.rs` (`scan_export_names`) refuses only an `export_name` value with the prefix.
    - No rule looks at a `no_mangle` fn with a listed name outside the top level of `ffi.rs`.
  - Reproducer `generic_command_submit_twin.ffi.diff`. It has three parts:
    - The listed export becomes `pub extern "C" fn miso_engine_web_v1_command_submit(handle: impl
      Into<u32>, count: u32)` with `#[allow(no_mangle_generic_items)]`. The body is still the sole
      `render_locked(|| ..)` call, now with `handle.into()`.
    - Nothing instantiates the generic fn in the cdylib, so it emits no symbol, and there is no
      duplicate symbol.
    - A twin is added, `mod submit_twin { #[unsafe(no_mangle)] pub(crate) extern "C" fn
      miso_engine_web_v1_command_submit(handle: u32, count: u32) -> u32 { with_host_mut(..) } }`.
      It is an explicit-name `use` (no glob), and its body has no `render_locked`.
  - What I measured on that tree:
    - The release tool prints only `live.rs:366`.
    - `cargo clippy -p host-web --all-targets -D warnings` (native) and `--lib` on
      `wasm32-unknown-unknown`: both clean.
    - `cargo fmt --check` and `check-workspace-policy.sh`: both ok.
    - `cargo test -p host-web`: all pass.
    - `build-web-audioworklet.sh --named-twin` builds module `cc8133c7...`.
    - `check-web-audioworklet.sh` (the frozen export set and the call graph),
      `check-browser-expected-resources.py --artifacts` and `test-web-audioworklet.sh`: all pass.
    - `check-sdk-headless.sh`: 363/363.
    - In the named twin, the one exported `miso_engine_web_v1_command_submit` never writes the
      render-lock flag (`i32.store8 offset=1151400`). Each wrapped export, for example
      `prepared_command_submit`, writes it at entry.
  - The same shape on `source_seek` (`generic_original_twin.ffi.diff`) passes the tool and both
    clippy runs. `cargo test -p host-web` then refuses it with "symbol already defined", because
    `src/tests.rs` instantiates the generic. That is luck of test coverage, not a rule. No Rust test
    calls `command_submit`.
  - Second form, a tool-only false pass (`export_name_override_twin.ffi.diff`). On the listed fn,
    keep `#[unsafe(no_mangle)]` and add `#[allow(unused_attributes)]` and
    `#[unsafe(export_name = "retired_source_seek")]`. The unprefixed value is accepted by (b), and
    the test accepts the equivalent `export_name = "memory_probe"` on purpose. Then add a child-module
    twin with the listed name.
    - The tool passes, and both clippy runs are clean.
    - The shipped seek export is the unwrapped twin, and it calls `malloc` outside any window.
    - `check-web-audioworklet.sh` refuses the module on the extra export `+retired_source_seek`, so
      this form is caught by a merge gate. The generic form adds no export and is not caught.
  - Why this is a finding and not a limit:
    - Both forms are plain written items with no macro, so this is not #1438 limit 4.
    - Root's (b) says "every exported name comes from a listed fn's own identifier". In the generic
      form, the shipped name comes from a fn that is not listed.
    - The class message for `ExportNameOnWebExport` (`report.rs`) and the `web_host.rs` module doc
      claim that invariant, but the tool does not enforce it.
    - The attempt-1 fix scope (mine) assumed that "a second definition of the same symbol anywhere
      in the crate is a duplicate-symbol compile error". A generic listed export breaks that
      assumption, because it defines no symbol.
  - Fix scope (fail closed; the real tree has none of these, I checked):
    - Refuse generics on a listed export: any `sig.generics` parameter, or an `impl Trait`
      argument.
    - Refuse every `#[export_name]` under `hosts/host-web/src`, whatever its value.
    - Refuse a `#[unsafe(no_mangle)]` fn (in any module, `impl` block or file under
      `hosts/host-web/src`) whose name is a listed export, unless it is the top-level fn in
      `ffi.rs`.
    - With those rules, the listed fn always emits its own symbol, and a twin is a
      duplicate-symbol error. A `macro_rules!` twin stays limit 4.

## MINOR

- **F2. One test-value sentence and two descriptions say more than the code proves.**
  - The record's test value for
    `an_export_name_with_the_table_prefix_is_refused_and_another_is_not` is "an exported name that
    does not come from a listed function's identifier, in any attribute spelling and any file of
    the crate". The test defends only the prefixed `export_name`.
  - The `ExportNameOnWebExport` display text and the `web_host.rs` doc make the same claim (see F1).
  - This goes with the F1 fix.

## Spec problems for root

- **SP-1. Rule (a) does not cover `render_locked`'s own definition. This is a compiling false pass
  outside root's text, so I do not charge it to the implementer.**
  - Reproducer `render_lock_cfg_split.diff`:
    - Put `#[cfg(test)]` on the real `pub(crate) fn render_locked` in `render_lock.rs`.
    - Add `#[cfg(not(test))] pub(crate) fn render_locked<R>(operation: impl FnOnce() -> R) -> R {
      operation() }`.
  - Measured:
    - The tool reports nothing new.
    - Native and wasm clippy are clean.
    - The `render_lock.rs` unit test still passes, because it runs only under `cfg(test)` and sees
      the real fn.
    - `render_locked_staging` and `render_locked_source` both pass with the pass-through, because
      they assert that the count stays at zero and have no positive control.
    - I did not run the browser chain on this tree.
  - Related point. The real `#[cfg(all(target_family = "wasm", not(test)))] #[global_allocator]`
    at `render_lock.rs:117` is a legitimate cfg. A change to that cfg would unregister the counting
    allocator, and every zero-allocation gate would then pass with nothing counted.
  - Root should decide one of two things:
    - (a) also refuses cfg on the `render_locked` definition, and the global-allocator cfg is pinned
      to its exact predicate; or
    - a positive control is added, where an allocation inside a real window must move the count in
      the wasm module and the integration binaries.
- **SP-2. Rule (b)'s first clause, as written, makes the second-form bypass in F1 legal.**
  The first clause refuses only a prefixed `export_name`. The rule needs the second clause turned
  into checks: a non-generic listed fn, no `export_name`, and no listed-name twin elsewhere.
- **SP-3 (observation).** Rules (a) and (c) and `WebModuleNotDeclared` assume that `src/lib.rs` is
  the crate root. A `[lib] path = ...` in `hosts/host-web/Cargo.toml` is outside
  "Under hosts/host-web/src". Today the manifest has no `path`. Root may want the tool to refuse a
  `path` key there.

## The implementer's additions beyond root's text (for root to ratify)

All three are correct fail-closed readings, and all three are true on the real tree, which has none
of them.
- **An inner `#![cfg]` or `#![cfg_attr]` in `ffi.rs` or `render_lock.rs` is refused.** An inner
  cfg removes the module, as cfg on `mod ffi;` does. The check also refuses a harmless
  `#![cfg_attr(all(), allow(dead_code))]` (I measured one finding). Over-refusal is acceptable.
- **`WebModuleNotDeclared`.** An inline `mod ffi { .. }`, or no top-level declaration at all,
  means that the build does not compile the file the rule reads. The class is correct, and
  mutation M12 shows that the test defends it.
- **`MissingWebSource`** (no `lib.rs` or no `render_lock.rs`). This is correct and harmless,
  because the build also fails without either file. Mutation M18 turns
  `walk::a_scan_root_that_is_a_file_is_a_finding` and the new case red.

## What I checked

- **The spec commit.** `d8a1f734c` contains root's Amendment 1. After whitespace normalisation,
  its text is the same as `r1496-d2-amend1.md`, and I checked this by script. It also has root's
  scope ruling for (d) and the named exceptions. `8fd895eb2` does not change those blocks.
- **The attempt-1 reproducers on the real tree:**
  - `cfg_split_pair` gives `ConditionalWrapPath` on the export and `ExportNameOnWebExport` at
    `ffi.rs:3991`.
  - `cfg_import_plus_glob` gives `ConditionalWrapPath` on the import and `GlobInWrapModule` at
    `ffi.rs:46`.
- **The real tree:**
  - The release tool prints exactly `crates/effect-contract/src/live.rs:366`.
  - The five `use super::*;` lines are at `ffi.rs:4853`, `4883`, `5351`, `6443` and
    `render_lock.rs:141`, all in child test modules, and they are accepted.
  - There is no `export_name` and no `#[path]` under `hosts/host-web/src`.
- **Mutations** (`mutations.py`). I applied each one alone, and each turned red only the new
  `web_host` tests (M18 also turned `walk` red):
  - M1: cfg on an export ignored.
  - M2: `unsafe(..)` not read.
  - M3: the import site not checked.
  - M4: the call site not checked.
  - M5: the `mod` site not checked.
  - M6: the inner attribute not checked.
  - M7: `cfg_attr` not treated as conditional.
  - M8: `cfg_attr` arguments not read.
  - M9: only `ffi.rs` scanned.
  - M10: every `export_name` refused.
  - M11: `#[path]` not checked.
  - M12: an inline or missing module declaration accepted.
  - M13: groups not searched.
  - M14: nested paths not searched.
  - M15: `render_lock.rs` not searched.
  - M16: globs refused in every module (199 tests red).
  - M17: exports collected from child modules too.
  - M18: no `MissingWebSource`.
- **Other bypass probes** (`probes.py`):
  - `#[cfg_attr(all(), cfg(all()))]` on an export is refused.
  - `#[cfg_attr(all(), unsafe(export_name = "miso_engine_web_v1_probe"))]` and
    `#[unsafe(export_name = ..)]` in a nested module of `control_targets.rs` are refused.
  - An escaped prefix `"\x6diso_engine_web_v1_.."` is refused.
  - `export_name = concat!(..)` passes the tool. That is limit 4 (macro assembly), and here it
    also fails `unsafe_code`.
  - An `impl`-block twin beside the intact original passes the tool and native clippy, but the
    wasm build refuses it with "symbol already defined". With the export_name override, it is F1's
    second form.
  - A `#[no_mangle]` (old spelling) twin is an edition-2024 compile error.
  - An `extern "C"` block with `#[link_name = "miso_engine_web_v1_source_seek"]` is an import, not
    an export. It is a `dead_code` error and not a bypass.
  - A later `use other::render_locked as render_locked` gives `Shadowed` and 31 `Unresolved`
    (E0252 as well).
  - A `macro_rules!` export with the original renamed gives `ExportMissing`. A macro twin that
    keeps the original is a duplicate symbol. Limit 4 is recorded.
  - `#[test]` on an export does not compile, because every wrapped export takes arguments.
- **Gates on `8fd895eb2`:**
  - `cargo test --locked -p realtime-policy`: 203 integration and 2 unit tests pass.
  - `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`, `cargo fmt --all --
    --check` and `check-workspace-policy.sh`: all ok.
  - `build-web-audioworklet.sh --named-twin`: shipped module `9784d3509dba6bfd...`, the same as
    the base.
  - `check-web-audioworklet.sh --without-metadata-regeneration`,
    `check-browser-expected-resources.py --artifacts` and `test-web-audioworklet.sh` (private
    TMPDIR, left empty): all exit 0.
  - `check-sdk-headless.sh`: 363 pass, 0 fail.
- **F2 and the NITs from attempt 1:**
  - The record sentence is corrected.
  - "11 cases" is corrected to 10.
  - `src/lib.rs:2` is rewrapped.

## Test value

Each sentence names the defect that the test turns red and that no older test catches.

- `a_cfg_split_twin_with_an_export_name_is_refused`: the attempt-1 cfg-split twin, which the old
  tool passed.
- `conditional_compilation_on_a_guarded_item_is_refused`: a cfg on the import, an export, the wrap
  call, a `mod` declaration or an inner attribute, at each of its eight sites. This includes the
  `cfg(any())` import with a glob.
- `an_export_name_with_the_table_prefix_is_refused_and_another_is_not`: a prefixed `export_name`
  in any spelling or file. The record's wording overclaims (F2).
- `a_path_on_a_guarded_module_or_an_inline_module_is_refused`: `#[path]`, an inline module or an
  undeclared guarded module.
- `a_top_level_glob_use_is_refused_and_a_child_module_glob_is_not`: a top-level glob in either file,
  and over-refusal of the real test-module globs.
- `an_export_moved_into_a_child_module_is_missing`: a rule that collects exports from child modules.
- `a_missing_host_module_file_is_a_finding`: a deleted `render_lock.rs` that the tool treats as a
  pass.

All of these sentences are true for the defects named. None of the tests defends F1.

The target directories, the two tree copies and the build outputs under `/tmp/claude-1002/v1496b`
are deleted. The small evidence files are kept.
