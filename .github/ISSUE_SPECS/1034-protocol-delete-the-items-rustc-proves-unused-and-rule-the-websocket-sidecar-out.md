# Protocol: delete the items rustc proves unused, and rule the WebSocket sidecar out

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Keep the protocol: the C ABI's command path is the protocol. Delete only the items rustc proves unused, and remove the WebSocket sidecar and local-sidecar transport from AGENTS.md's scope text (closes #25).

**Blocked on an owner ruling, and on `R2-…`.** Scoping study:
`docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 7, R3. The ruling to record:
"The engine has one control surface, the browser host's command ABI (`miso.command.v1`). The
transport-neutral binary protocol, the local sidecar and the WebSocket transport are out of scope."

## Context

- **`crates/protocol`: 29,957 lines.** 18,049 production, 11,094 in-source tests and 829 in
  `tests/`; 175 tests, about 10 s. It contains BTLV, queues, the controller, delivery and the
  session and message wire formats.
- **Around it:**
  - `crates/conformance`'s protocol part, about 953 lines: `src/protocol_corpus.rs`, the wasm
    golden runner `src/main.rs`, and two tests. The DSP conformance part stays.
  - `tools/bench/src/protocol.rs` and its `.fbs` schema (1,690 lines). This is the only reason
    `bench` depends on `flatbuffers`.
  - `tools/audit/src/protocol.rs` (512 lines).
  - Four fuzz targets (`protocol_command`, `protocol_session_transaction`, `protocol_event`,
    `protocol_response`) and their seed corpora.
  - Nine scripts (668 lines), including `run-protocol-fuzz.sh`, `check-protocol-wasm-parity.sh`,
    `run-protocol-allocation-audit.sh`, `run-protocol-benchmark.sh`, `test-protocol-benchmark.sh`,
    `check-protocol-control-policy.sh` and `test-protocol-control-policy.sh`.
  - Six docs: `docs/CONTROL_BTLV_V1.md`, `CONTROL_PROTOCOL_CONFORMANCE.md`,
    `CONTROL_PROTOCOL_REGISTRY.md`, `CONTROL_PROTOCOL_SEMANTICS.md`, `CONTROL_PROTOCOL_SIZING.md`
    and `CONTROL_PROVIDER_BOUNDARY.md`.
- **Code in product crates used only by the protocol** (SCIP; re-prove by compile):
  - `engine/src/realtime/spsc.rs:308` `generation` and `:421` `success_count`;
  - in `session/src/model.rs`: `RenderMode` `wire`/`from_wire`, the `F32Planar` variant, and the
    `Step`/`Linear` automation variants. Check whether the session JSON parser still produces
    them before deleting.
- **Browser dependency: none.**
  - `protocol` is not in host-web's wasm closure.
  - host-web's command ABI has its own vocabularies (`hosts/host-web/src/lib.rs:1391-1491`).
  - The "acked-batch" admission AGENTS.md cites (#139/#140) is host-web's `admit_commands`
    (`lib.rs:6246`, with `admit_commands_staged` at `:6287`; the `WebCommandReport` it fills is at
    `:1497-1523`), not the protocol's.
  - No constant, golden file or wire format is shared.
- **Sidecar and WebSocket:** no code. `Cargo.lock` has no WebSocket or socket crate. The only
  WebSocket mentions are scans that check the worklet and stem store do *not* use it
  (`check-web-audioworklet.sh:462`, `:466`; `check-stem-store-v1.mjs:132`). Those scans stay.
- **CI.**
  - Golden parity: 19 s.
  - The protocol share of "Conformance boundaries": 38 s total.
  - SIMD128 probe: 1 s.
  - Benchmark argument checks: 4 s.
  - The protocol allocation audit.
  - `fuzz.yml`'s `protocol-fuzz` job: 1.6 minutes on each push or PR touching seven crates.
  - About 12 of the roughly 25 nightly deep-fuzz minutes.
- **Compile proof** (audit, section 7). With `protocol`, `capi`, native-pcm-runner and the
  protocol parts of conformance, bench and audit deleted:
  - `cargo check --workspace --all-targets --all-features` passed;
  - the wasm32 checks of host-web, host-core and conformance passed;
  - `cargo test -p conformance -p bench` passed.
- **Issues:** open #25 (WebSocket sidecar) and #140 (protocol automation delivery) close as
  descoped.

## Smallest closable slice

1. Delete `crates/protocol`, the conformance protocol corpus and its wasm golden runner, the
   `bench` `protocol` subject and the `flatbuffers` dependency, the `audit` `protocol` subject, the
   four fuzz targets and their corpora, the nine scripts and the six docs.
2. **CI.**
   - Delete the protocol golden-parity, allocation-audit and benchmark-argument steps, and the
     protocol half of "Conformance boundaries".
   - Replace `-p protocol` in the SIMD128 compile probe.
   - Delete `fuzz.yml`'s `protocol-fuzz` job, and remove `crates/protocol/**` and
     `scripts/run-protocol-fuzz.sh` from its `paths:`.
   - Remove the four targets from `nightly.yml`'s deep-fuzz loop.
3. **Policies.** Update `check-host-core-policy.sh`, `check-conformance-boundaries.sh`,
   `check-realtime-policy.sh`, `check-bench-policy.sh`, `check-release-shape.py` and
   `docs/ENGINE_ENV_VOCABULARY.md` (for example `MISO_ENGINE_BENCH_WASM_SCALAR_BYTES`), with their
   mutation tests.
4. Delete the product-crate items listed above once nothing uses them. Prove it by compile.
5. **AGENTS.md.** Rewrite "Interfaces and transports", the "Protocol mutations update the same
   typed session model" sentence, and the RFC 6455 citation to match the ruling. Keep the
   acked-batch question: it applies to host-web's admission.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p conformance`
     passes.
   - `cargo check --locked --manifest-path fuzz/Cargo.toml --bins` passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes.
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes. It is byte-identical, unless step 4
   touches `session` or `engine` lines in the module's closure; then prove with `wasm-objdump -d`
   that only panic line numbers moved, and re-pin with that reason.
4. **CI routing.**
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - `fuzz.yml`'s path list equals the new fuzz crate's workspace closure, as its header comment
     requires.
   - The `verdict` table is unchanged.
   - `nightly.yml`'s `failure-notice` still lists every job.
5. **No live claim lost.**
   - The removed tests are the protocol's, its fuzz targets, the conformance protocol corpus and
     the protocol subjects.
   - Every host-web command-admission test (`hosts/host-web/src/tests.rs`, including the
     test-support ones `00-…` turned on) still runs and passes.

## Dependencies

The owner ruling, then `R2-…` (capi carries protocol frames: 179 `protocol::` uses).

## Standing rules for the implementer

- Commit on `codex/<issue>-remove-protocol`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F3. With mobile playback in scope and the C ABI kept (R2
amendment), **the protocol cannot be removed.**

1. **The C ABI's command path is the protocol.** `miso_engine_v1_submit_command` decodes a binary
   protocol frame through the protocol controller (`capi/src/runtime/control.rs:675-690`,
   `prepare_command_frame`), and events leave as protocol frames. capi names `protocol::` 55 times
   outside its tests (`ffi.rs` 21, `runtime/control.rs` 29, `runtime/compile.rs` 4,
   `runtime/mod.rs` 1) and 115 times in `runtime/tests.rs`.
2. **The browser facts hold.** `protocol` is not in host-web's closure on `wasm32` or on
   `aarch64`, and neither `sdk/` nor `hosts/host-web/web/` mentions it. host-web's command ABI
   cannot simply be reused natively: its exports pass pointers as 32-bit values
   (`hosts/host-web/src/ffi.rs:2725`), so a native version would need a new wrapper around
   `admit_commands`.
3. **What is still genuinely unneeded:** the WebSocket sidecar (no code; open #25) and the local
   sidecar transport text in AGENTS.md; the 20 protocol items rustc proves unused (draft 01, "Out of
   scope"), which can now be deleted in a protocol-only follow-up of draft 01.
4. **Open question the owner must rule on:** whether mobile live control (a fan's fader, mute or
   effect change during playback) goes through protocol automation (#140, still open; the partial
   endpoints of `02-…` and protocol's `delivery.rs`/`controller_delivery.rs` are its unwired
   pieces) or through structural plan replacement only.
5. **Revised recommendation:** retire this draft. Replace it with (a) the draft-01 follow-up for
   the 20 dead protocol items, and (b) a ruling that the sidecar/WebSocket transport is out of
   scope, which edits only AGENTS.md and closes #25.
