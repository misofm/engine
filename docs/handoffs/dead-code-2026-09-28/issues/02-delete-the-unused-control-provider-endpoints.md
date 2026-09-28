# Delete the unused control-provider endpoints

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 3, row A4. The
code is dead today whatever you rule on the C ABI and the protocol, because nothing calls it, not
even `capi`.

## Context

`crates/host-core` carries two native control endpoints behind the `control-provider` feature
(`src/lib.rs:92-93`, `:108-109`, `:225-228`):

| file | lines |
|---|---:|
| `src/builtin_batch_endpoint.rs` (#576, #579, #580, #587, #594) | 2,529 |
| `src/scalar_point_endpoint.rs` (#528, #536, #563, #575, #605, #608) | 1,060 |
| `tests/builtin_batch_endpoint.rs` | 1,727 |
| `tests/scalar_point_endpoint.rs` | 2,928 |
| **total** | **8,244** |

- **No callers.** `rg 'BuiltinBatchEndpoint|ScalarPointEndpoint|builtin_batch_endpoint|scalar_point_endpoint' crates hosts tools`
  finds only these four files and `host-core/src/lib.rs`. `capi` enables `control-provider` but
  never names either endpoint.
- **Compile proof.** A scratch copy with the modules, the re-exports and the two test files
  removed passed `cargo check --workspace --all-targets --all-features`.
- **Test cost.** The tests run in `test-debug-a` (feature unification through `capi`). There are
  30 integration tests (20 + 10), plus the in-source unit tests of `builtin_batch_endpoint.rs`,
  and all of them together take under a second. The cost is compile time.
- **Not in the browser.** `host-web` never enables `control-provider`, and
  `scripts/check-host-core-policy.sh` forbids it. So the shipped module cannot change.
- **History.** These were partial steps toward open #140 (deliver admitted **protocol** automation
  to the running plan). #140 is about the native protocol path. The browser's command admission is
  host-web's own (`admit_commands`).

## Smallest closable slice

1. Delete the four files, their `mod` and `pub use` lines, and any `control-provider`-only items
   that become unused. Prove the latter by compile, and list them.
2. Delete what exists only for these endpoints:
   - their `MUTATIONS.md` rows;
   - their entries in `scripts/check-host-core-policy.sh` and `scripts/test-host-core-policy.sh`,
     if any;
   - their mentions in `docs/`, outside `docs/handoffs/`.
3. Post a comment on #140 saying the partial endpoints were removed as unused, and pointing at this
   issue. If the protocol ruling (`R3-…`) keeps the protocol, #140's successor re-derives what it
   needs.
4. Optional, and only if the protocol is kept: protocol's `delivery.rs`, `controller_delivery.rs`
   and `tests/delivery_ownership.rs` (about 4,930 lines) have these endpoints as their only callers
   outside `protocol`. `controller.rs` also depends on them, so removing them needs surgery. File
   that separately. If `R3-…` removes the protocol, skip it.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p host-core`
     passes.
2. **Console digests.** The `gain_pan_profile digests` output
   (`cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`)
   is byte-identical on base and change.
3. **Shipped artifact: byte-identical.**
   - Build base and change with `scripts/build-web-audioworklet.sh --module-only EMPTY_DIR` on one machine.
   - `host-web` does not compile these modules, and `lib.rs` changes only inside `cfg`'d lines.
   - If the hash moves, stop and explain.
4. **Tests (no live claim lost).** `cargo test` over the CI feature sets (after `00-…`) passes.
   The `-- --list` diff against base contains only tests defined in the four deleted files. List
   them in the evidence.
   - Their subject is the deleted endpoints themselves.
   - `forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses`
     (`builtin_batch_endpoint.rs:1497`) compares each backend's endpoint with its own reference
     (`:1729-1743`), not Scalar with banked. So no "banking moves no bit" claim is lost. That claim
     lives in `graph-compiler/tests/bank_levels.rs`.
   - Some deleted tests also assert allocation-free render and off-render reclamation for the
     endpoints (`tests/builtin_batch_endpoint.rs:202`, `:374`; `src/builtin_batch_endpoint.rs:2138`;
     `tests/scalar_point_endpoint.rs:1469`, `:2736`). Name the surviving equivalents for the live
     paths in the evidence:
     - host-core `tests/spectrum.rs:439`, `:649` and `tests/observation_demand.rs:2007`;
     - `scripts/check-web-audioworklet.sh:349-449`;
     - host-web `src/tests.rs:3537`.
   - After this change, the `#[cfg(test)]` seam `prepare_host_runtime_with_console_backend`
     (`prepare.rs:596`) keeps one user (`limiter_linked_session.rs:333`). Keep it.
5. **CI routing.** No workflow references these files. `check-ci-path-routing.py` and
   `test-ci-path-routing.py` pass. `check-host-core-policy.sh` and its mutation test pass.

## Dependencies

`00-…`, then `01-…`, which deletes three members of these files.

## Standing rules for the implementer

- No product behaviour change.
- Commit on `codex/<issue>-delete-unused-endpoints`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F3. **Hold this draft for an owner ruling. "Dead whatever you
rule on the C ABI and the protocol" is wrong under the corrected scope.**

1. **The compile facts hold.** With both source files, both test files and their `mod`/`pub use`
   lines removed: native `--all-targets --all-features` passes with no new warning, `wasm32`
   `simd128` passes, iOS and Android `--lib` pass, and `-p capi -p host-core --all-targets` passes
   on iOS. No `control-provider` item becomes unused.
2. **But they are unwired, not dead.** Mobile playback is live scope and the C ABI stays (R2
   amendment). The open #140 spec targets delivery of admitted protocol automation "through the
   actual C ABI command and render calls"; per its own text (line 13) the only production consumer
   of that queue today cancels it. These endpoints (#528-#608) and protocol's `delivery.rs` /
   `controller_delivery.rs` are the partial implementation of exactly that. Deleting them is a
   #140 design decision.
3. **Failure scenario:** the draft lands as "no ruling needed"; the owner then schedules mobile live
   control (a fan's fader or mute during playback), and #140's successor must re-derive about 8,200
   lines of qualified, tested endpoint code that git history holds but no current test exercises.
4. **Revised recommendation:** move this draft to the "needs your ruling" set, decided together
   with #140: either (a) #140's plan uses these endpoints, so keep them and wire them; or (b) #140
   re-plans from scratch, so delete them as drafted. Step 4 (protocol delivery files) follows the
   same decision.
