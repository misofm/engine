# Remove the eight-lane wasm measurement build

**Blocked on an owner ruling, because it reverses part of an earlier one.** Scoping study:
`docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 7, R7.

**The conflict.**

- The #183 ruling (`docs/rulings/wasm-simd8-null.md`, 2026-08-27) closed the W8-on-wasm switch as a
  null, but kept "the `--issue183` bench arm and the `miso_wasm_simd8` opt-in cfg … in the tree"
  so a future engine could re-measure.
- The 2026-09-28 direction points the other way:
  - "no target-specific code, beyond the architectural lane widths";
  - "modes production never needs are removed entirely, including test-only entry points".

The ruling to record: "The wasm build has one width, `Simd4`. The eight-lane measurement cfg and
its arms are removed. A future re-measurement re-adds a cfg in its own issue."

## Context

- **`crates/lane/src/backend.rs:8`, `:40-58`.** The `miso_wasm_simd8` arm moves `Backend::current()`
  on `wasm32` from `Simd4` to `Simd8`.
- **`crates/target-smoke/src/lib.rs:62-73`.** Its W8 assertion. `R1-…` may delete the crate first.
- **`Cargo.toml:105-108`.** The `check-cfg` entry for `miso_wasm_simd8`, and its comment.
- **The W8 leg of the wasmtime console benchmark** (`tools/wasm-console`, about 32 mentions in 823
  lines), its validator branch, and `scripts/test-wasm-console-benchmark.sh:214-265`, which CI lint
  runs.
- **The `--issue183` arms** of `scripts/operator/run-wasm-console-benchmark.sh` (`:46`, `:333`)
  and its preflight (`:40`, `:216`). They cannot run: `artifacts/issue183/` exists, and the
  scripts' repository root is wrong (audit, section 5).
- **Shipped module: unaffected.** No default build sets the cfg, and the module is built without
  it.
- **Related, and a separate choice.** `tools/wasm-gate-corpus` digests every case at three widths
  on every target (`WIDTHS = 3`, `src/lib.rs:69-72`), including `Simd8` on `wasm32`.
  - That is a determinism check of the generic lane code, not the measurement build. It uses
    `lane::Simd8` directly, not `Backend::current()`.
  - Keep it unless you want every wasm `Simd8` path gone. It costs a little of the wasm-guests job.

## Smallest closable slice

1. Delete the `miso_wasm_simd8` arm from `lane/src/backend.rs` and its module doc, the
   `target-smoke` assertion (unless `R1-…` has removed the crate), and the `check-cfg` entry and
   its comment.
2. Delete the W8 leg of `tools/wasm-console`, its validator branch and its mutation cases.
   `R9-…` deletes the whole tool; if it lands first, this step is moot.
3. Delete the `--issue183` arms from the operator runner and its preflight, unless `R9-…` deletes
   both scripts.
4. Append a supersession note to `docs/rulings/wasm-simd8-null.md` (the rulings README asks for
   this). Say the measurement hooks were removed on this date, and why. Keep the null result.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p lane`
     passes.
   - `RUSTFLAGS='-C target-feature=+simd128 --cfg miso_wasm_simd8' cargo check --target wasm32-unknown-unknown -p lane`
     now warns about an unexpected cfg and still selects `Simd4`. Record the output.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes.
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes.
   - `lane/src/backend.rs` loses lines, so panic line numbers in `lane` may shift. Prove with
     `wasm-objdump -d` that nothing else changes, and re-pin with that reason.
   - Or keep the line count by leaving a one-line comment where the arm was.
4. **CI routing.** `bash scripts/test-wasm-console-benchmark.sh` passes, if the tool still exists.
   `check-ci-path-routing.py` and `test-ci-path-routing.py` pass. The `verdict` table is unchanged.
5. **No live claim lost.** The only tests removed are the W8-measurement ones. List them from the
   `-- --list` diff (audit section 11).
   `wasm-gate-corpus`'s three-width determinism digests stay.

## Dependencies

The owner ruling. It is simpler after `R1-…` and `R9-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-remove-wasm-w8`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F11. The recommendation is sound; the context is incomplete.

1. **The kept re-measurement path is already dead.** `docs/rulings/wasm-simd8-null.md:36-43` kept
   the cfg and the `--issue183` arm so the paired capture could be re-run, and requires "a fresh
   paired record on the then-current base" to reopen. That capture cannot run today:
   `artifacts/issue183/` exists and the runner refuses to overwrite it, and the operator script's
   repository root is wrong (finding F2). Removing the hooks forecloses nothing that works.
2. **The cfg had a live test use on 2026-09-28.** Spec #976 (closed that day, lines 466-469) ran a
   `--cfg miso_wasm_simd8` leg of its bit-exact differential harness (15,000 scenarios, Simd8 in
   wasm). State that this ad-hoc leg is lost; a harness can still run Simd8 on wasm through
   `lane::Simd8` directly, as `wasm-gate-corpus` does.
3. **It is a lane-width measurement hook, not target-specific code** in the sense of the owner's
   rule; the ruling rests on "modes production never needs are removed", which does apply.
4. **Mobile scope: no effect.** The cfg is `wasm32`-only.
