VERDICT: PASS

# #1234 attempt 1: adversarial verdict

- **Reviewed:** `codex/d15-stream-j` at `492d0153d`, which is the implementation `2e5bcc31f` plus the record `492d0153d`, on base `3bf212cad`.
- **Where it ran:** a `git archive` export of `492d0153d` at `/tmp/claude-1002/v1234/src`, and a second export of the base `3bf212cad` at `/tmp/claude-1002/v1234/base-src` for the comparison build. Both used `CARGO_TARGET_DIR=/tmp/claude-1002/v1234/target` and `TMPDIR=/tmp/claude-1002/v1234/tmp`. The worktree was not touched.

D1-D4 are implemented as specified, and the edits stay inside the authorized paths:
- `scripts/check-web-audioworklet-callgraph.py`;
- the attribute and doc lines of the two `free()` accessors;
- the spec's record.

All six objective gates pass in my export. Every factual claim in the record reproduces exactly:
- the digests and sizes;
- the two admitted names;
- the one newly refused name;
- that both accessors are outlined into the `command_submit` closure.

There is no BLOCKER and no MAJOR. There is one MINOR, about a class of names the anchored form gives up. Today's module contains none of them. There are three NITs.

## Independent confirmation on the rebuilt named twin

- **Build.** I ran `build-web-audioworklet.sh --named-twin` on the head and `--module-only --named-twin` on the base.
  - **Shipped module:** base `b723ff9f…` (2,894,096 B), head `5cfb8572…` (2,894,071 B).
  - **Named twin:** base `2cdfa78c…` (3,313,317 B), head `41c89241…` (3,313,442 B).
  - All four values match the record.
- **Name forms.** The head twin has 2,739 defined functions, and every one is named. The names come in four groups:
  - Rust v0-mangled names (`_R…`).
  - The `miso_engine_web_v1_*` exports.
  - `memcmp` and `__multi3`.
  - There are no legacy `_ZN…` names.
  - The module has **no Import section**, and `check-web-audioworklet.sh:363` refuses any import. The only exports are `memory` and `miso_engine_web_v1_*`.
- **Real allocator names.** The real dlmalloc entries are v0 names that end in an instantiating-crate suffix, for example `…8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE4freeCsdl5sGgnNXvY_3std`. They are therefore never the whole name `free`. The `dlmalloc` alternative catches all of them, both before and after this change.
- **Shim names.** The allocator shims are `___rustc12___rust_alloc`, `14___rust_dealloc`, `19___rust_alloc_zeroed` and `14___rust_realloc`.
- **Old against new regex, over every function in the head twin.** I wrote my own enumerator, `/tmp/claude-1002/v1234/diffre.py`.
  - The old regex refuses 237 functions and the new one refuses 236.
  - **Admitted only by the new regex:** exactly `host_core…RouteControlProducer4free` and `graph…GraphRouteControlProducer4free`.
  - **Refused only by the new regex:** exactly `_RNvCs9wFQrvczXsK_7___rustc14___rust_realloc`.
  - No function is named exactly `free`, `malloc`, `calloc` or `realloc`.
  - On the base twin, the new regex admits nothing that the old one refused.
- **Per-export verdicts.**
  - **Base analyser on the head twin:** `command_submit` fails on exactly the two accessors (closure 66). This reproduces #1222's finding.
  - **Head analyser on the head twin:** `command_submit` passes. `render` (closure 8) and `meter_poll` (closure 9) are unchanged.
  - **Either analyser on the base twin:** all three exports pass. The base `command_submit` closure is 64 members; the two extra members at head are the outlined accessors.
- **`__rdl_realloc`.** `___rustc13___rdl_realloc` matches neither regex. It directly calls the dlmalloc functions `malloc`, `free`, `memalign`, `dispose_chunk` and `unlink_chunk`, so the closure walk still catches it transitively. This gap predates the change, and the non-goals keep it out of scope.

## Findings

### MINOR 1: whole-name anchoring admits prefixed C or third-party allocator spellings that the substring form refused

- **Where:** `scripts/check-web-audioworklet-callgraph.py:112` (`^(free|malloc|calloc|realloc)$`) and the docstring at `:19-25`.
- **What happens:** under the base regex, any name containing `free` or `malloc` was refused. Under the head regex, all of these are admitted:
  - `dlfree`;
  - `__libc_free` and `__libc_malloc`;
  - `mi_free` and `mi_malloc_aligned`;
  - `je_free` and `tlsf_free`;
  - `emscripten_builtin_free`.

  I verified this with the regexes themselves.
- **Effect today:** nothing. The shipped module has no imports and no unmangled allocator, and the spec's invariant is scoped to today's module plus the self-test cases. That invariant holds.
- **The docstring's reasoning only runs one way.** It argues that a bare `free` can only be a C allocator. It never shows that a C allocator can only be spelled bare. The second claim is false for every prefixed allocator above. A future `#[global_allocator]` backed by C, where LTO inlines the `__rust_*` shim, would reach `mi_free` and the like with nothing else in the list matching.
- **Fix (a follow-up or spec amendment, for Sol):** refuse the C names as a substring of *unmangled* names only:

  ```
  ^(?!_R|_ZN).*(?:free|malloc|calloc|realloc)
  ```

  On today's head twin this gives exactly the same verdicts as the shipped regex; I checked this. It still admits every v0 or legacy Rust method named `free`, and it keeps every prefixed C spelling above refused.
  - Add one self-test row for a prefixed C name such as `dlfree`.
  - The bare `__rust_realloc` row would then also match the new alternative. Use its v0 form `_RNvCs0_7___rustc14___rust_realloc` so that row still discriminates the `__rust_realloc` alternative.

### NIT 1: the pass-side anchor is only tested for `free`

- **Where:** `scripts/check-web-audioworklet-callgraph.py:628-635`.
- **Surviving mutations:** two mutations keep the self-test green.
  - **No trailing anchor:** `^(free|malloc|calloc|realloc)`, with the `$` dropped.
  - **Ungrouped:** `^free|malloc|calloc|realloc$`, which refuses `malloc` and `calloc` anywhere in a name again.
- **Why it is only a NIT:** both mutations fail safe, by refusing more names.
- **Fix:** loop the pass case over out-of-line `…4free`, `…6malloc`, `…6calloc` and `…7realloc` accessors. That kills the ungrouped mutant.

### NIT 2: the `(a1) Rust allocator _ZN8dlmalloc4free17h0E` row duplicates case (a)

- **Where:** `scripts/check-web-audioworklet-callgraph.py:644`.
- **What happens:** `reaching("_ZN8dlmalloc4free17h0E")` rebuilds case (a)'s fixture byte for byte. Mutation M-no-dlmalloc turns both rows red together, so this row has no unique catch. The implementer's record says so too.
- **Context:** the spec required this row, so it is not an attempt defect.
- **Fix:** drop the row, or replace it with the real v0 form from the module, `_RNvMs0_NtCs0_8dlmalloc8dlmallocINtB5_8DlmallocNtNtB7_3sys6SystemE4freeCs1_3std`. That form at least pins the mangling the module actually emits.

### NIT 3: the docstring's appeal to how wasm-objdump prints imports is moot, and the spec names the wrong wiring script

- **Docstring:** `scripts/check-web-audioworklet-callgraph.py:23-24`.
  - An import has no body, so `wasm-objdump -d` emits no `func[N] <…>:` header for it.
  - `closure()` (`:319`) silently skips a callee index it has no body for. The analyser can never see an imported `free`.
  - What actually excludes imports is `check-web-audioworklet.sh:363`.
  - **Fix:** say that in place of the sentence about printing.
- **Spec:** the non-goal at spec `:67` says `check-web-audioworklet.sh` runs the self-test. It is `scripts/test-web-audioworklet.sh:41`, which qualification.yml wires in at `:349`. Coverage is intact; only the spec's text is wrong.

## Gates (rerun in the export)

1. **Self-test.** `python3 -B scripts/check-web-audioworklet-callgraph.py --self-test` exits 0.
2. **Red on revert.**
   - The exact base `FORBIDDEN` exits 1 on four cases: `(a1) out-of-line accessor named free passes`, `bare C allocator calloc`, `bare C allocator realloc` and `Rust allocator __rust_realloc`.
   - The unanchored `(free|malloc|calloc|realloc)` exits 1 on exactly the out-of-line-accessor case. So does the end-anchored-only form.
   - Restored, the self-test is green.
3. **Mutation.**
   - Dropping `^(free|malloc|calloc|realloc)$` exits 1 on `bare C allocator` free, malloc, calloc and realloc.
   - `^free$` alone exits 1 on malloc, calloc and realloc.
   - Dropping `calloc` exits 1 on calloc.
   - Dropping `|__rust_realloc` exits 1 on exactly `Rust allocator __rust_realloc`.
   - Dropping `dlmalloc` exits 1 on (a), (a1)-dlmalloc, (b1) and (b1b).
   - The survivors are the two in NIT 1.
4. **Web build and check.** `bash scripts/build-web-audioworklet.sh --named-twin N A` exits 0. `bash scripts/check-web-audioworklet.sh A N/miso-engine-v1-audio-worklet.simd128.named.wasm` exits 0 (`web AudioWorklet static/object checks passed`).
5. **Resources and digests.** `python3 -B scripts/check-browser-expected-resources.py --artifacts A` exits 0. Its digests and exact rows agree with the built module and every row is within budget, so no render digest moved.
6. **Formatting, lint and policy.**
   - `cargo fmt --all -- --check` exits 0.
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` exits 0 with no warnings; graph, host-core and host-web were all checked.
   - `bash scripts/check-workspace-policy.sh` reports `workspace policy: ok`.
7. **Extra: unit and integration tests.** `cargo test --locked -p graph -p host-core` passes all 35 test binaries with 0 failures.

**Realtime and architecture.** All callers of the two `free()` accessors are on the control path:
- `hosts/host-web/src/lib.rs:1785`;
- `route_controls.rs:107`;
- tests.

Outlining an atomic load adds no allocation, lock or syscall, and touches no render path.

**Pin.** The committed release pin, `6c952a2c…`, differs from both the base and the head module. Under #1061 the pin is the release fingerprint and is not held per PR, so this change does not touch it.

## Test value (one sentence per new case; every mutation reproduced red, then green on revert)

- **(a1) out-of-line accessor named `free` passes:** it turns red if the C names are unanchored again, or anchored only at the end, so that an ordinary outlined Rust method named `free` reads as the allocator. That is #1222's false positive, and no prior case catches it. Reproduced with the base, unanchored and end-anchored-only mutants.
- **(a1) bare C allocator `free` / `malloc` / `calloc` / `realloc`:** these turn red if the anchored C alternative is deleted or narrowed, which would admit a real C allocator entry by its exact symbol. `calloc` and `realloc` were never refused by the base regex, so those two rows are new coverage. Reproduced with the drop-all, `^free$`-only and drop-calloc mutants.
- **(a1) Rust allocator `__rust_realloc`:** it turns red if `__rust_realloc` is missing from the list; nothing else matches that name. Reproduced with the drop-`|__rust_realloc` mutant, which turns exactly this row red.
- **(a1) Rust allocator `_ZN8dlmalloc4free17h0E`:** no unique catch, because it is the same fixture as case (a); see NIT 2.
