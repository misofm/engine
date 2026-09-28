# Delete the items the compiler proves unused

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 2 ("Truly dead
code"). No owner ruling is needed: nothing in any build, test, tool or target uses these items.

## Context

The audit demoted every workspace item that rust-analyzer's cross-reference index (SCIP) showed with
zero references from `pub` to private in a scratch copy, then let rustc's `dead_code` lint judge
them. It ran `cargo check --workspace --lib --bins --examples --all-features` (non-test build) and
`cargo check --workspace --tests --all-features` (test build) on x86-64-v3, and
`cargo check --target wasm32-unknown-unknown` (simd128) over the browser closure. An item is listed
below only when rustc reported it unused in **both** the non-test and the test build, and no
integration test, tool or other crate failed to compile after the demotion (which would have meant
a real user). Items reachable only from other dead items (the "cascade") are marked.

The audit found 129 such items (about 910 lines including doc comments).

- The table below holds the 104 that this draft deletes: about 720 lines in 18 crates.
- The other 25 belong to other drafts and are listed under "Out of scope":
  - `protocol` has 20;
  - `host-core`'s endpoint files have 4 (`record_count`, the `report` field and method, and
    `fault`);
  - `host-mobile` has 1.

It is small because the Rust tree is otherwise very clean at the item level. `unreachable_pub`,
`missing_docs` and clippy `-D warnings` already keep private dead code out.

| crate | items | items (file:line at `a9414c0c`) |
|---|---|---|
| `builtins` | 8 | `lib.rs:67` `ChannelLinkMode` (cascade); `:394` `DISABLED_LATTICE_INDEX`; `:3186` `link_mode`; `:3379` `reset_lifetime_recovered_state`; `:3650` `trim_target`; `:3817` `target_gain`; `:3826` `is_muted`; `:4326` `balance_matrix` |
| `builtins-compiler` | 2 | `lib.rs:4071` `classes` (cascade); `:4079` `mono_track_count` |
| `conformance` | 1 | `block.rs:69` `channel` |
| `dsp-reference` | 23 | `block.rs:34` `from_samples`; `compressor.rs:104` `latency_samples`; `delay.rs:38` `delay_target_ms`, `:63` `set_delay`, `:117` `sample_rate_hz`, `:150` `set_delay`; `lr4.rs:86` `reset`; `parametric_eq.rs:198` `is_identity`, `:285` `coefficients`; `svf.rs:393` `reset` (cascade), `:400` `state`; `tpt.rs:70` `output` (field), `:144` `output`, `:284` `magnitude_db`; `transient_shaper.rs:90` `fast_envelope`, `:96` `slow_envelope`; `true_peak_limiter.rs:191-193` fields `sample_rate_hz`, `parameters`, `n`, `:283` `required_delay`, `:295` `parameters`, `:389` `gain`, `:394` `reset` |
| `effect-compiler` | 4 | `migration.rs:170` `new`, `:189` `step`, `:389` `replay`; `prepare.rs:1245` `empty` |
| `effect-contract` | 7 | `lib.rs:211` `new`, `:481` `as_str`, `:1459` `snap`, `:2147` `is_empty`; `live.rs:207` `capacity`, `:820` `retained_bytes`; `step.rs:23` `as_str` |
| `effect-package` | 4 | `cid.rs:60` `verify_raw_bytes`; `state.rs:847` `as_bytes`, `:884` `latency_samples`, `:887` `tail` |
| `engine` | 23 | `realtime/buffer.rs:73` `count`, `:78` `total_samples`, `:125` `clear`, `:133`/`:136` fields `storage`/`stride`, `:148` `try_new`, `:173` `plane`, `:271` `plane_range`; `realtime/disjoint.rs:104` `planes`, `:116` `frames`, `:519` `reserved`, `:535` `total_bytes`; `realtime/plan_exchange.rs:436` `copy_response_snapshot`, `:497` `copy_worker_audit_snapshots`, `:504` `dispatch_counters`; `realtime/plan.rs:466`/`:864` `copy_worker_audit_snapshots` and `:490`/`:656` `dispatch_counters` (cascade: auxiliary-worker remnants of the removed dependency-wave renderer); `realtime/spsc.rs:323` `overflow_count`, `:416` `generation`, `:431` `underrun_count`, `:456` `is_empty` |
| `graph` | 1 | `lib.rs:2905` `quantum_samples` |
| `host-core` | 6 | `control_preparation.rs:457` `factory`; `render_session.rs:104` `render_contiguous`, `:121` `render`, `:181` `copy_response_snapshot`; `source.rs:228` `is_empty`; `spectrum.rs:2953` `DEFAULT_SMOOTHING_MS` |
| `host-web` | 4 | `lib.rs:224` `SPECTRUM_WINDOW_BYTES` (cascade), `:226` `SPECTRUM_RESULT_BYTES`, `:232` `SPECTRUM_MAXIMUM_RESULT_BYTES`, `:495` `OBSERVATION_STATUS_BYTES` |
| `lane` | 2 | `kernels/builtins.rs:35` `all_lanes`, `:130` `nonfinite_lanes_block` |
| `parametric-eq` | 4 | `lib.rs:239` `reset_ramping_elided_blocks`, `:244` `ramping_elided_block_count`, `:250` `test_only_reset_ramping_elided_blocks`, `:258` `test_only_ramping_elided_blocks` (a `test-support` hook no test calls) |
| `rack` | 5 | `lib.rs:600`/`:3233` `disarm_observations` (one is cascade), `:906` `designed_lane_witness`, `:1575` `left_mut`, `:1579` `right_mut` |
| `rack-compiler` | 1 | `lib.rs:93` `name` |
| `session` | 3 | `compile.rs:57` field `graph_entity_indexes`, `:98` `graph_entity_index`; `diagnostic.rs:153` `id` |
| `source` | 5 | `lib.rs:398` and `:429` `diagnostic_code`, `:813` `channel_count`, `:819` `quantum_frames`, `:1268` `read_block_contiguous` |
| `stem-hasher` | 1 | `lib.rs:44` `token` |

Also in scope:

- **`crates/rack-compiler/Cargo.toml`: the `engine` dependency is unused.** `cargo machete` flags it,
  `crates/rack-compiler/src/lib.rs` never names `engine`, and the audit built
  `cargo check -p rack-compiler --all-targets` with the line removed.

- **47 `#[allow(dead_code)]` attributes that suppress nothing.** The audit stripped all 59
  non-test ones and ran `cargo check --lib`, natively and on `wasm32`. Only 12 warnings appeared:
  - Layout mirrors in `graph` (`lib.rs:2459`, `:2471`; `runtime.rs:988`, `:1035`, `:2109`,
    `:2262`, `:2287`), which keep theirs.
  - `capi/src/runtime/compile.rs:72`, which keeps its own.
  - Members used only by tests: `graph-compiler/src/canonical.rs:106` `edge_text_len`;
    `host-core/src/spectrum.rs:1643` `cadence`, `:1650` `observer_handle`, `:1780` `stage`;
    `host-web/src/lib.rs:2550` `boot_with_observation_demand`. These move behind `#[cfg(test)]`
    (or a `cfg_attr(not(test), expect(dead_code))`) instead of a blanket `allow`.

  The other 47 attributes go, including 17 in `host-web/src/observation_ingress.rs` and 16 in
  `host-web/src/lib.rs`.

Out of scope (each belongs to another draft, so it is not deleted twice):

- `protocol` (20 items) and `host-core`'s `builtin_batch_endpoint.rs`/`scalar_point_endpoint.rs`
  items (`record_count`, `report`, `fault`): see `02-...` and the protocol ruling (`R3-...`). If the
  owner keeps `protocol`, delete its 20 items here in a follow-up.
- `host-mobile::mobile_target_smoke`: the whole crate is `R1-...`.
- `session::CompileCaps`'s four fields documented as inert since #241
  (`crates/session/src/compile.rs:17-43`): public fields set by callers, so removing them edits every
  caller. File separately if wanted.

## Smallest closable slice

1. Delete every item in the table and the `engine` dependency of `rack-compiler`. Do not add
   `#[allow(dead_code)]` anywhere. If deleting an item makes another item unused, delete that too
   and list it in the evidence.
2. Remove the 47 no-op `#[allow(dead_code)]` attributes. Move the five test-only members listed
   above behind `#[cfg(test)]`.
3. If rustc reports that an item **is** used on some target or feature set the audit did not
   build, keep it and record where it is used.
4. Update doc comments that describe a deleted item. Do not touch `docs/handoffs/`.

## Objective gates

1. **Native build:** `cargo check --locked --workspace --all-targets --all-features`,
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` and
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` pass.
2. **Wasm build:** `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target
   wasm32-unknown-unknown -p host-web -p host-core` passes, and so does the wasm-guests job's
   scalar build line in `.github/workflows/qualification.yml` while that job exists.
3. **Console digests unchanged:** the output of
   `cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`
   is byte-identical on the base commit and on the change (all standing 64-block console digests),
   and `bash scripts/run-wasm-gates.sh` passes.
4. **Shipped artifact:** build it with `bash scripts/build-web-audioworklet.sh --module-only EMPTY_DIR`
   on base and change, **on the same machine**. The digest depends on whether the toolchain's
   `rust-src` component is installed (audit section 9), so do not compare a local build against
   the committed pin.
   - The two modules must be byte-identical, or differ only in panic-location line numbers in the
     data section. Prove the second case by showing that `wasm-objdump -d` is identical function
     by function and the export list is identical.
   - Re-pin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` only in that second
     case, and state the reason in the evidence. The CI `artifact` job is the authority.
   - Run `python3 -B scripts/check-browser-expected-resources.py`, and re-pin only what moves,
     with the reason.
5. **Tests (no live claim lost):** `cargo test --locked --workspace --all-targets` passes with the CI feature sets of
   the `test-debug-a` and `test-debug-b` jobs, plus the `test-support` features `00-...` adds. The
   `-- --list` output is identical on base and change: no test is deleted, because no test
   references these items.
6. **CI routing:** no workflow, script or router change is needed; `python3 -B scripts/check-ci-path-routing.py`
   and `python3 -B scripts/test-ci-path-routing.py` pass.

## Dependencies

`00-...` first, so gate 5 covers the `test-support` tests as well. Land this before `02-...` and
the ruling-dependent removals so their diffs stay small.

## Standing rules for the implementer

- No product behaviour change; a moved console digest is a hard stop.
- Commit on `codex/<issue>-delete-unused-items`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, findings F8 and F3. A scratch copy with **47 of the 129 items deleted
outright** (18 crates, every host-web constant, the engine plan/exchange wrappers, lane, source,
rack, dsp-reference, builtins, session, effect-contract, effect-package, graph, rack-compiler with
its `engine` dependency, conformance, effect-compiler, builtins-compiler, host-core and stem-hasher)
was built natively (`--workspace --all-targets --all-features`), for `wasm32` `simd128`
(`-p host-web -p host-core`), for the scalar-wasm package list, for the fuzz bins, and with Rust
1.98.1 for `aarch64-apple-ios` and `aarch64-linux-android` (`--workspace --lib --all-features`,
excluding the two tools whose `blake3` build script needs a cross C compiler) and
`aarch64-unknown-linux-gnu` `--all-targets` for lane, soft-clip, graph, engine, capi and host-core.
45 of the 47 are dead on every one of those builds. Two are not:

1. **`parametric-eq` `lib.rs:239` `reset_ramping_elided_blocks` and `:244`
   `ramping_elided_block_count` are used.** The `#[cfg(test)] mod ramping_elision` (`lib.rs:6379`)
   calls them at `:7601`, `:7604` and `:7677`; deleting them gives E0425 in the `parametric-eq` lib
   test build. Delete only the two `test_only_*` hooks (`:250`, `:258`) and narrow the two helpers
   from `cfg(any(test, feature = "test-support"))` to `#[cfg(test)]`, or clippy `--all-features`
   warns on them. So the table's parametric-eq row is 2 deletions plus 2 narrowings, and the
   "unused in both builds" statement is wrong for these two.
2. **`Cargo.lock` changes.** Removing `rack-compiler`'s `engine` dependency rewrites the lockfile.
   Commit it in the same change, or every `--locked` gate fails.
3. **Expected knock-on warnings** (clippy `-D warnings` fails until removed): unused imports
   `ResponseSnapshot*` (`engine/src/realtime/plan_exchange.rs:15`), `QuantumFrames`
   (`graph/src/lib.rs:230`), `RenderTime` (`host-core/src/render_session.rs:35`). Step 1 already
   covers these ("delete that too and list it").
4. **The no-op `allow(dead_code)` count is at most 45, not 47.** Stripping all 56 non-test
   attributes (55 outer, plus the inner `#![allow(dead_code)]` at `engine/src/realtime/plan.rs:3`,
   which suppresses nothing) leaves 13 warnings natively, 11 on `wasm32` and 12 on iOS. The draft's
   keep-list misses `tools/native-pcm-runner/src/lib.rs:733` (`fault`, read only by test fault
   injection at `:1004-1159`). No extra warning appears on aarch64.
5. **Mobile scope (the owner's correction): no item here is used by AArch64-only code**, and all
   of them compile out of the capi closure cleanly. But the draft's "Out of scope" list changes:
   - the 20 `protocol` items: R3 is now retired (the C ABI needs the protocol), so delete them in a
     protocol-only follow-up of this draft, with the same gates;
   - `host-mobile::mobile_target_smoke`: goes only if R1's reduced stub removal is ruled.
6. **Non-Rust consumers checked:** the four host-web constants have none. The SDK reads those sizes
   through `miso_engine_web_v1_spectrum_result_bytes` (`hosts/host-web/src/ffi.rs:3127`) and
   `…_observation_status_bytes` (`:4832`), which compute their own values;
   `tools/parameter-metadata/src/abi_layout.rs` imports none of them; `node sdk/codegen/assets.mjs
   --check` passes on the scratch tree.
7. **Add gates:** an aarch64 `cargo check --lib` for `-p lane -p engine -p host-core -p capi -p
   source` (a toolchain with the aarch64 std; CI has none yet), and `cargo check --manifest-path
   fuzz/Cargo.toml --bins`.
