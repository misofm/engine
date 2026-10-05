# Anchor the worklet callgraph checker's C allocator names

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).

## Mission

`scripts/check-web-audioworklet-callgraph.py` refuses any function in a guarded closure whose
**name** contains the substring `free`, so an ordinary out-of-line Rust method named `free` reads as
the C allocator. Anchor the C allocator names so only a real allocator symbol matches, prove it with
self-test cases, and then drop the two `#[inline(always)]` attributes that exist only to dodge the
false positive.

## The defect (found by the #1222 attempt 1 verdict, deviation 1)

- `FORBIDDEN` (`scripts/check-web-audioworklet-callgraph.py:103-106`; every anchor here is verified on the batch K3 follow-up tree, branch `codex/batch-submix-k3`) is
  `dealloc|dlmalloc|free|malloc|drop_glue|drop_in_place|drop_slow|unlink_chunk|insert_large_chunk|memory_grow|__rust_alloc`,
  searched (`FORBIDDEN.search`, `:324`) anywhere in each closure member's symbol name.
- Two accessors are `#[inline(always)]` only for this reason, each with a comment saying so:
  `GraphRouteControlProducer::free` (`crates/graph/src/lib.rs:1005-1014`) and
  `RouteControlProducer::free` (`crates/host-core/src/route_controls.rs:71-80`, the attribute at
  `:77`). Both bodies are an atomic load. The #1222 verifier rebuilt the module without the two attributes: the
  `miso_engine_web_v1_command_submit` allocation gate then fails on exactly
  `RouteControlProducer4free` and `GraphRouteControlProducer4free`.
- On `wasm32-unknown-unknown` the Rust allocator's symbols are already matched by `dlmalloc`,
  `dealloc` and `__rust_alloc`. A bare `free`, `malloc`, `calloc` or `realloc` can only be a C
  allocator's unmangled symbol.
- The workaround fails safe (a lapsed inline turns the gate red, never green), so this is a
  false-positive hazard, not a missed allocation.

## Invariants

- The gate never gets weaker: every allocator, deallocator and drop-glue symbol it refuses today
  (the shipped module's, and every self-test case's) is still refused.
- No engine behaviour changes. Dropping the two attributes may change the shipped module's bytes;
  it must not change any render digest.
- A test that greps source or prose is refused. The self-test runs the analyser on synthetic
  disassembly, which is not source grepping.

## Deliverables

1. `FORBIDDEN` matches the C allocator names only as a whole symbol name:
   `^(free|malloc|calloc|realloc)$`. It keeps every crate and shim alternative it has today
   (`dealloc`, `dlmalloc`, `drop_glue`, `drop_in_place`, `drop_slow`, `unlink_chunk`,
   `insert_large_chunk`, `memory_grow`, `__rust_alloc`) and adds `__rust_realloc`, which no
   alternative matches today.
2. Self-test cases in the analyser's `self_test()`:
   - an out-of-line accessor whose mangled name ends in `4free` (for example
     `_RNvMs_NtCs0_5graphNtB4_25GraphRouteControlProducer4free`) in a closure **passes**;
   - a closure member named exactly `free` **fails**, and so do `malloc`, `calloc` and `realloc`;
   - `_ZN8dlmalloc4free17h0E` and a `__rust_realloc` member **fail**.
3. Remove `#[inline(always)]`, and the comment sentences that justify it, from the two `free()`
   accessors named above.
4. The module-docstring paragraph that describes `FORBIDDEN` says the C names are anchored and why.

## Authorized paths

- `scripts/check-web-audioworklet-callgraph.py` (`FORBIDDEN`, its docstring paragraph, `self_test()`).
- `crates/graph/src/lib.rs` (`GraphRouteControlProducer::free` attribute and doc only).
- `crates/host-core/src/route_controls.rs` (`RouteControlProducer::free` attribute and doc only).
- This spec's own record sections.

## Non-goals

- No other change to the callgraph rules, the trap allow-list or the kernel-shape gate.
- No renaming of the two accessors (renaming would also dodge the false positive, and is what this
  issue exists to make unnecessary).
- No change to CI wiring: `check-web-audioworklet.sh` already runs the analyser and its self-test.

## Hazards

- `wasm-objdump` prints imported and exported names unmangled. Confirm on the shipped named twin
  that no legitimate closure member is named exactly `free`, `malloc`, `calloc` or `realloc`; if one
  is, it is a real allocator reach and the gate must stay red.
- Without the attribute LLVM may still inline the accessors. The gate must pass either way; record
  whether `4free` appears in the `command_submit` closure on the rebuilt module.

## Objective gates

1. `python3 -B scripts/check-web-audioworklet-callgraph.py --self-test` exits 0 with the new cases.
2. Red on revert: with deliverable 1 reverted (the unanchored `free`), the self-test's
   out-of-line-`free` case fails. Record the run.
3. Mutation: drop `^(free|malloc|calloc|realloc)$` entirely from `FORBIDDEN`; the self-test's
   bare-`free` case fails. Record the run.
4. `bash scripts/build-web-audioworklet.sh --named-twin <N> <A>` then
   `bash scripts/check-web-audioworklet.sh <A>
   <N>/miso-engine-v1-audio-worklet.simd128.named.wasm` exit 0 with the attributes removed (the
   script takes `ARTIFACT_DIRECTORY NAMED_TWIN`).
5. `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` passes (no render
   digest moved).
6. `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features --
   -D warnings`, and `bash scripts/check-workspace-policy.sh` pass.

## Evidence

- Each gate's command and exit status from the PR head.
- The rebuilt module's size against the base's, and whether the accessors were outlined.
- One test-value sentence per new self-test case.

## Dependencies

- None. Found by *Admit live send commands in the browser* (#1222).

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

Base `3bf212cad` (branch `codex/d15-stream-j`); implementation commit `2e5bcc31f`.

- **D1.** `FORBIDDEN` is now `^(free|malloc|calloc|realloc)$|dealloc|dlmalloc|drop_glue|drop_in_place|drop_slow|unlink_chunk|insert_large_chunk|memory_grow|__rust_alloc|__rust_realloc`.
- **D2.** Self-test case (a1) in `self_test()` swaps the member in case (a)'s closure for each name:
  - `_RNvMs_NtCs0_5graphNtB4_25GraphRouteControlProducer4free` **passes**. *Defect it catches:* an
    unanchored C name, which would refuse every ordinary outlined Rust method named `free`.
  - `free`, `malloc`, `calloc`, `realloc` each **fail**. *Defect it catches:* the C allocator
    alternative deleted or mis-anchored, for example `^free$` alone, which would admit a real C
    allocator reach. The base regex never refused bare `calloc` or `realloc` either.
  - `_ZN8dlmalloc4free17h0E` and `__rust_realloc` **fail**. *Defect it catches:* `__rust_realloc`
    missing from the list, since nothing else matches it. The dlmalloc row keeps the Rust allocator
    refused alongside the anchored names; case (a) already covers it, so it is not a unique catch.
- **D3.** `#[inline(always)]` and its justifying doc sentences are removed from
  `GraphRouteControlProducer::free` (`crates/graph/src/lib.rs`) and `RouteControlProducer::free`
  (`crates/host-core/src/route_controls.rs`). No other change.
- **D4.** A paragraph under `--callgraph` in the module docstring says the C names are anchored and why.
- **Gate 1.** `python3 -B scripts/check-web-audioworklet-callgraph.py --self-test`: exit 0.
- **Gate 2 (red on revert).** With the exact base `FORBIDDEN` restored, the self-test exits 1:
  `(a1) out-of-line accessor named free passes`, `(a1) bare C allocator calloc`,
  `(a1) bare C allocator realloc` and `(a1) Rust allocator __rust_realloc` all fail. With only the
  anchor removed (`free|malloc|calloc|realloc` unanchored), it exits 1 on exactly the out-of-line
  accessor case. Restored, it is green.
- **Gate 3 (mutation).** With the anchored alternative replaced by a never-matching `(?!)`, the self-test
  exits 1 on `(a1) bare C allocator free`, `malloc`, `calloc` and `realloc`. A further mutation
  dropping `|__rust_realloc` exits 1 on exactly `(a1) Rust allocator __rust_realloc`. Restored,
  it is green.
- **Gate 4.** `bash scripts/build-web-audioworklet.sh --named-twin N A` (exit 0), then
  `bash scripts/check-web-audioworklet.sh A N/miso-engine-v1-audio-worklet.simd128.named.wasm`: exit 0
  with the attributes removed.
- **Gate 5.** `python3 -B scripts/check-browser-expected-resources.py --artifacts A`: exit 0 (the
  digests and exact rows agree with the built module).
- **Gate 6.** `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings` and `bash scripts/check-workspace-policy.sh`: all pass.
- **Outlining (hazard 2).** Without the attribute LLVM keeps both accessors out of line.
  `RouteControlProducer4free` and `GraphRouteControlProducer4free` are both members of the
  `miso_engine_web_v1_command_submit` closure in the rebuilt named twin. The base regex would refuse
  exactly these two there, which reproduces the #1222 finding, and the anchored regex passes it.
- **No weakening (hazard 1).** Across every function in the rebuilt named twin, the only names the
  base regex refuses that the new one admits are those two accessors. No function is named exactly
  `free`, `malloc`, `calloc` or `realloc`. The new regex also refuses one name the base admitted:
  `_RNvCs9wFQrvczXsK_7___rustc14___rust_realloc`.
- **Size.** Shipped module: base `b723ff9f…` is 2,894,096 B, rebuilt `5cfb8572…` is 2,894,071 B
  (-25 B). Named twin: 3,313,317 B before and 3,313,442 B after.

### Attempt 1 verdict follow-ups (batch follow-ups, 2026-10-05)

The verifier passed attempt 1. Folded:
- NIT 1: `self_test()` now loops the out-of-line accessor pass case over all four names
  (`…4free`, `…6malloc`, `…6calloc`, `…7realloc`) and adds four pass cases for unmangled names
  that merely begin with a C allocator name (`free_count`, `malloc_count`, `calloc_count`,
  `realloc_count`).
  - Defect caught that no other case catches: a `FORBIDDEN` C alternative that loses its trailing
    `$` (prefix match) or its grouping (`^free|malloc|calloc|realloc$`), which refuses an ordinary
    function only *named* like an allocator.
  - Mutation run (each applied to a copy of the checker, `--self-test` run):
    - `$` dropped: attempt-1 self-test rc 0; fold rc 1 on the four `unmangled …_count` cases.
    - ungrouped: attempt-1 self-test rc 0; fold rc 1 on the `malloc`, `calloc`, `realloc`
      accessor cases and the `free_count`, `malloc_count`, `calloc_count` cases.
    - unmutated: fold self-test rc 0.
- NIT 3 (docstring): the import reasoning now says what holds: an import has no body, so
  `wasm-objdump -d` prints no function header for it and this checker never sees one, and
  `check-web-audioworklet.sh` refuses any import before this gate runs.

Not folded:
- NIT 2 (the `_ZN8dlmalloc4free17h0E` (a1) row duplicates case (a)): the spec requires that row;
  it is left as is.
- NIT 3 (spec text): the non-goal says `check-web-audioworklet.sh` runs the self-test; it is
  `scripts/test-web-audioworklet.sh`, wired in by qualification.yml. Coverage is intact. The
  non-goal is spec scope text, which the coordinator owns, so it is reported, not edited.
