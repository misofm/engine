# Copy a per-node effect's state into a same-layout instance in one pass

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A warm successor (D15-8 step 2) can take a per-node effect's exact state from the running plan in
one memory pass, while the running plan keeps its own instance and goes on rendering. A delay that
holds two seconds per channel is copied once into the successor's preallocated instance, not
twice through a payload scratch buffer. *Carry per-node effect instances across a plan swap*
(#1282) calls this in copy mode.

## Context

- `PreparedNativeEffect` (`crates/effect-contract/src/lib.rs:1822`) has render-safe, exact payload
  calls (`snapshot_state_payload`, `:1930`; `restore_state_payload`, `:1975`). Through a scratch
  buffer they copy every state byte twice, and the scratch must be sized for the largest state.
  For the delay at 96 kHz that is about 1.5 MiB.
- The trait has no `Any` access, so one instance cannot see another's concrete type.
  `StatePayloadError` is a code string (`:1108-1110`).
- There are eight launch native effects, each with one `PreparedNativeEffect` impl:
  - compressor: `crates/compressor/src/lib.rs:961`;
  - delay: `crates/delay/src/lib.rs:1226` (state struct `PreparedDelay`, `:572`);
  - gate/expander: `crates/gate-expander/src/lib.rs:879`;
  - multiband compressor: `crates/multiband-compressor/src/lib.rs:1626`;
  - parametric EQ: `crates/parametric-eq/src/lib.rs:3486`;
  - soft-clip: `crates/soft-clip/src/lib.rs:1137`;
  - transient shaper: `crates/transient-shaper/src/lib.rs:963`;
  - true-peak limiter: `crates/true-peak-limiter/src/lib.rs:4415`.
- Test doubles implement the trait too (`crates/graph/src/lib.rs:3619`, `:5031`, `:6621`;
  `crates/builtins-compiler/src/lib.rs:6035`; `crates/conformance/src/effect.rs:390`). They are not
  launch effects.
- The launch registry is `effect_compiler::launch_native_effect_registry`
  (`crates/effect-compiler/src/prepare.rs:202`).

## Decisions frozen for this slice

- **D1. Trait surface.** `PreparedNativeEffect` gains two methods with defaults:
  - `fn as_any(&self) -> Option<&dyn core::any::Any>` returns `None` by default;
  - `fn copy_state_from(&mut self, source: &dyn PreparedNativeEffect) -> Result<(),
    StatePayloadError>` refuses by default with `effect.state.copy.unsupported`, writing nothing.

  The defaults keep the test doubles compiling. Every launch effect overrides both (gate 1).
- **D2. Contract** (the method's rustdoc):
  - **Render-thread code.** No allocation, free, lock, syscall or log. Time is bounded by the
    prepared state size.
  - **What it copies.** Every state word a continuation reads: histories, ramps with their steps,
    envelopes, cursors and lines. After it, this instance renders, from then on, exactly what
    `source` renders by continuing.
  - **What it refuses, writing nothing:** a source of another concrete type
    (`effect.state.copy.type`), or one whose prepared configuration differs
    (`effect.state.copy.layout`). Prepared configuration means rate, quality, link mode, prepared
    bypass, latency, `state_sizes` and `state_layout_version`.
  - **What it never touches:** prepared, immutable words (coefficients designed at preparation).
    Rule 3 makes them equal anyway.
  - `source` is only read.
- **D3. Implementation.**
  - Each effect copies its state fields in place with `copy_from_slice`, or by plain assignment
    for scalars.
  - It never uses `clone`, `clone_from` or `Vec` operations that could reallocate.
  - Its layout check runs before the first write.

## Deliverables

1. D1-D2 in `crates/effect-contract/src/lib.rs`.
2. D3 in the eight effect crates.
3. One test per effect, and the registry gate.

## Authorized paths

- `crates/effect-contract/src/lib.rs`
- `crates/compressor/src/`, `crates/delay/src/`, `crates/gate-expander/src/`,
  `crates/multiband-compressor/src/`, `crates/parametric-eq/src/`, `crates/soft-clip/src/`,
  `crates/transient-shaper/src/`, `crates/true-peak-limiter/src/` (payload code and its tests only;
  stream G edits parameter code in the same crates, and root orders the merges)
- `crates/effect-compiler/tests/` (the registry gate)

## Non-goals

- No graph or carry change (#1282 calls it).
- No bank-lane copy: banks use the payload calls (#1279-#1281).
- No change to the payload layout.

## Objective gates

1. **Every launch effect copies exactly.** For each effect in `launch_native_effect_registry`:
   - prepare two instances with the same configuration;
   - drive A with a deterministic signal for several blocks, with a parameter ramp in flight at
     the copy and, for the delay, a ringing feedback tail;
   - call `B.copy_state_from(&A)`;
   - render A and B on with the same input for 16 blocks.

   Every output word is bit-identical. Then render A alone for 4 more blocks, and compare with a
   twin of A that was never copied from: bit-identical.
2. **Refusals write nothing.** A source of another effect type, and a same-type source prepared at
   another rate or quality, are refused with the D2 code. B then renders exactly as before the
   call.
3. **Realtime.** The copy makes zero allocations, reallocations and frees for every effect
   (`bench_support::alloc` thread counters, statics warmed).
4. Commands:
   - `cargo test --locked -p effect-contract -p compressor -p delay -p gate-expander -p multiband-compressor -p parametric-eq -p soft-clip -p transient-shaper -p true-peak-limiter -p effect-compiler`
   - `cargo build --locked --release -p audit && bash scripts/trace-effect-contract-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1:
  - a copy that misses a state word (a ramp's step, a cursor, the multiband's band envelopes, the
    limiter's look-ahead position) moves B's bits;
  - a launch effect left on the default refusal fails the registry walk;
  - a copy that writes into `source` breaks A's twin.

  Each turns it red, and no payload test sees a direct copy.
- Gate 2: a layout check that runs after the first write leaves B half-copied. It turns red.
- Gate 3: a copy through `clone_from` or a `Vec` resize that reallocates turns it red.

## Dependencies

- None open. The payload calls are render-safe and exact on `main` (#1278, and `e0e4e8a20` for the
  delay). Stream order: before *Carry per-node effect instances across a plan swap* (#1282).
