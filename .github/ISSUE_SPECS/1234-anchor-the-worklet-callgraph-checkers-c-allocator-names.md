# Anchor the worklet callgraph checker's C allocator names

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
  `RouteControlProducer::free` (`crates/host-core/src/route_controls.rs:59-68`). Both bodies are
  an atomic load. The #1222 verifier rebuilt the module without the two attributes: the
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
   `bash scripts/check-web-audioworklet.sh <A>` exit 0 with the attributes removed.
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
