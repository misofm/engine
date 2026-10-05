# Retire the automation span kinds and the sample rate that no producer uses

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.6 and A9 and README finding F6 as decided, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch P1.

## Product outcome

The effect contract states only what the engine delivers. An effect receives `Point` spans and
nothing else, and a parameter is automated at block rate or not at all. The span kinds `Step`,
`Linear` and `Exponential`, the function `automation_segment_value`, the descriptor value
`AutomationRate::Sample` and the protocol's `ParameterAutomationRate::Sample` (wire value 1) are
retired: each raw value is refused, never reallocated and never renumbered. No rendered bit moves.

## Context

- **The vocabulary.** `AutomationRate {Sample=1, Block=2, None=3}` and
  `AutomationSpanKind {Point=1, Step=2, Linear=3, Exponential=4}`
  (`crates/effect-contract/src/lib.rs:139`, `:145`). `valid_runtime_span` checks each kind
  (`:1420-1455`); `automation_segment_value` evaluates the three moving kinds (`:1632-1656`); the
  descriptor check accepts `Sample` like `Block` (`:198-206`). The contract doc calls the three
  kinds and `Sample` "descriptor and protocol vocabulary" whose render-path delivery "is a later
  protocol capability" (`docs/EFFECT_CONTRACT_V1.md:149-155`).
- **No producer.** Live records stage only `Point` spans (`crates/effect-contract/src/live.rs:21`,
  `:456`), and every launch effect accepts only a `Point` (`crates/compressor/src/lib.rs:434`,
  `crates/gate-expander/src/lib.rs:563`, `crates/soft-clip/src/lib.rs:622`,
  `crates/true-peak-limiter/src/lib.rs:3847`; the graph's check `crates/graph/src/lib.rs:6641`).
  Stored automation stages only `Point` spans (#1058 A1.6). The "later protocol capability" was
  `AUTOMATION_ENQUEUE`, which drafts 21a-21c retire (#1058 A9). No launch descriptor declares
  `Sample`: its uses are test fixtures (`crates/effect-contract/src/lib.rs:2608-2624`, `:2711`;
  `crates/conformance/src/effect.rs:51`) and the mappings in
  `crates/host-core/src/control_provider.rs:656` and `tools/parameter-metadata/src/lib.rs:830`.
  `sdk/assets/miso-engine-v1-parameter-metadata.json` has no `"sample"` value.
- **The reference mock and the generator.** The conformance reference delay evaluates the moving
  kinds (`crates/conformance/src/effect.rs:345-362`), the randomized generator draws them
  (`crates/conformance/src/randomized.rs:1770-1780`), and
  `crates/conformance/tests/effect_contract.rs:25`, `:161-163` tests `automation_segment_value`.
  `crates/delay/src/lib.rs:2605-2625` builds a `Linear` span in a malformed-span test.
- **Other test fixtures that declare `Sample`:** `crates/effect-contract/src/step.rs:927` (in its
  `#[cfg(test)]` module, `:908`), `crates/effect-compiler/tests/native_session.rs:34`,
  `crates/protocol/src/controller/tests.rs:490` and
  `crates/conformance/tests/conformance_corpus.rs:81`.
- **A policy pin.** `scripts/check-effect-runtime-policy.sh:61` pins `fn automation_segment_value(`
  at count 1 in its duplicated-helper manifest.
- **Every user.** `git grep` on `c63f5f37d` for `AutomationRate::Sample`,
  `ParameterAutomationRate::Sample`, `AutomationSpanKind::{Step, Linear, Exponential}` and
  `automation_segment_value` (outside `docs/handoffs/` and `.github/`) finds them only in the files
  this Context names, in `crates/effect-contract/tests/response_analysis.rs`,
  `crates/protocol/src/message_wire/tests.rs` and in `docs/EFFECT_CONTRACT_V1.md:151-152`.
- **The protocol value.** `ParameterAutomationRate::Sample = 1`
  (`crates/protocol/src/message_wire.rs:244`, parsed at `:3039`); the registry lists "automation
  sample/block/none `1..3`" (`docs/CONTROL_PROTOCOL_REGISTRY.md:13`).
- **The removal ruling.** Code nothing uses is removed
  (`docs/rulings/engine-footprint-2026-09-28.md:20-21`).

## Decisions frozen for this slice

- **D1. Retired values.** `AutomationSpanKind` keeps `Point = 1`; `from_raw` returns `None` for 2,
  3 and 4, with a comment that names them retired and never reallocated. `AutomationRate` keeps
  `Block = 2` and `None = 3`; `from_raw` returns `None` for 1. The protocol's
  `ParameterAutomationRate` keeps 2 and 3, and its parse refuses 1 as an invalid field value.
- **D2. Code leaves.** `automation_segment_value` leaves; `valid_runtime_span` checks only a
  `Point`; the descriptor check accepts only `Block` for an automatable parameter; host-core's and
  the metadata tool's mappings lose `Sample`. The conformance reference mock and the generator
  keep only `Point` spans; a test that exists only for a retired kind is deleted, and the delay's
  malformed-span test uses a `Point` with a malformed field instead. Each test fixture that
  declares `Sample` (Context) declares `Block`. The policy manifest's `fn automation_segment_value(`
  row leaves with the function. The independent metadata gate's rate table
  (`scripts/check-parameter-metadata-v1.py:30`, `RATES = {1: "sample", 2: "block", 3: "none"}`,
  read at `:477`) loses its `1: "sample"` entry, so that gate also refuses the retired rate.
- **D3. Re-pins**, with the reason "draft 26 retires the span kinds and the sample rate": any
  conformance corpus row or digest that carries a retired value, and `COMPLETE_SCHEMA_HASH` only if
  a corpus frame carries `ParameterAutomationRate` value 1. `retired_code_rows` gains a row for
  value 1 in a parameter descriptor.
- **D4. Docs.** `docs/EFFECT_CONTRACT_V1.md:149-155` says: runtime automation is `Point` spans; the
  other kinds and the sample rate are retired by this slice. The registry line lists rate 1 as
  retired.
- **D5. The acked-batch question** has no subject here: nothing is queued or acknowledged.

## Deliverables

1. D1 and D2 in `crates/effect-contract`, `crates/protocol`, host-core's provider mapping, the
   metadata tool and `crates/conformance`.
2. D3 and D4.

## Authorized paths

- `crates/effect-contract/src/lib.rs`, `crates/effect-contract/src/step.rs` (its test module's
  fixture only), `crates/effect-contract/tests/`
- `crates/effect-compiler/tests/native_session.rs` (the fixture's rate only)
- `crates/protocol/src/controller/tests.rs` (the fixture's rate only)
- `crates/conformance/tests/conformance_corpus.rs`
- `scripts/check-effect-runtime-policy.sh` (the `automation_segment_value` manifest row only)
- `crates/protocol/src/message_wire.rs`, `crates/protocol/src/message_wire/tests.rs`
- `crates/host-core/src/control_provider.rs` (the rate mapping only)
- `tools/parameter-metadata/src/lib.rs` (the rate name only),
  `scripts/check-parameter-metadata-v1.py` (the `RATES` entry at `:30` only)
- `crates/conformance/src/{effect.rs,randomized.rs,protocol_corpus.rs}`,
  `crates/conformance/tests/effect_contract.rs`
- `crates/delay/src/lib.rs` (the malformed-span test only), and each effect test support module
  that builds a retired kind (`crates/*/tests/support/mod.rs`, `crates/*/tests/common/mod.rs`)
- `docs/EFFECT_CONTRACT_V1.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`

## Non-goals

- `Point` spans and every effect's span handling, which stay. Stored automation (A1).
- The protocol command (drafts 21a-21c).

## Hazards

- **Generated SDK files.** If the build regenerates a file that spells the rate names, regenerate
  it with the build, never by hand (`bash scripts/check-sdk-generated.sh`).

## Objective gates

1. **Retired values refused** (`crates/effect-contract` unit tests and `retired_code_rows`).
   `AutomationSpanKind::from_raw(2..=4)` and `AutomationRate::from_raw(1)` are `None`; a parameter
   descriptor frame with rate 1 fails decode; a descriptor that declares an automatable parameter
   with a non-`Block` rate fails the descriptor check.
2. **Effects unchanged.** Every effect crate's tests and the conformance suite pass, the
   randomized conformance run included; its generator now reaches only `Point` spans.
3. **No rendered bit moves.** `cargo build --locked --release -p audit -p bench -p capi -p
   session-validator`, then `./target/release/audit capi` shows the same `pcm_digest` as base (PR
   evidence) and zero allocations, locks and syscalls.
4. **Commands:**
   - `cargo test --locked -p effect-contract`, `cargo test --locked -p effect-compiler`,
     `cargo test --locked -p conformance`,
     `cargo test --locked -p protocol --features test-support`,
     `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p delay`
   - the workspace debug leg (`test-debug-a`) and the DSP leg (`test-debug-b`) in
     `.github/workflows/qualification.yml`
   - `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts` after
     `bash scripts/build-web-audioworklet.sh`, `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-effect-runtime-policy.sh`, `bash scripts/test-effect-runtime-policy.sh`
   - `python3 -B scripts/check-parameter-metadata-v1.py --self-test` (also run by
     `scripts/test-web-audioworklet.sh:205`)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if a retired kind or rate still parses, or if a descriptor can still declare a
  rate the engine never delivers.
- Gate 2 turns red if a launch effect depended on a retired kind.

## Dependencies

Batch P1. Direct dependency:

- Draft 21c *Delete the protocol automation queue, its records and counters*: its record codec
  carries the span kind (`AutomationKind`, `crates/protocol/src/queue.rs:30-44`), so the protocol's last producer of the moving kinds leaves first.
