# Build host-core's crate docs without the control-provider feature

## Product outcome

`cargo doc -p host-core --no-deps` with `RUSTDOCFLAGS="-D warnings"` passes with the default feature set
and with `--features control-provider`, so the crate's own doc command is a usable gate.

## Context

- `crates/host-core/src/lib.rs:19`, in the crate-level doc, links `[`SessionControlProvider`]`. That type
  exists only under the `control-provider` feature (`crates/host-core/Cargo.toml:15`,
  `control-provider = ["dep:protocol"]`). Without the feature, rustdoc reports a broken intra-doc link, so
  `cargo doc -p host-core --no-deps -D warnings` fails on `main`.
- `crates/host-core/src/prepare.rs:73`, in the doc of `session_jump_lengths` (#1639 D1), links
  `[`crate::LiveDelta::jump_lengths`]`; `LiveDelta` is behind the same feature. Found by running G1 on
  82f987296 (exit 101, two errors: `lib.rs:19:49` and `prepare.rs:73:64`).
- The workspace form (`cargo doc --workspace`) enables the feature through feature unification, so it
  passes; #1498's gate 3 had to use that form for this reason (reported by the #1498 verdict).

## Decisions

- D1. The crate-level doc names `SessionControlProvider` as a link only when the feature is enabled:
  use `#[cfg_attr(feature = "control-provider", doc = "...")]` for the linked sentence and a plain code
  span otherwise, or move the sentence onto the feature-gated module's own doc. Either way the wording
  stays the same; no code changes. Applied as an inner `#![cfg_attr(..., doc = ...)]` pair in the
  crate doc (the link stays under the feature); `prepare.rs` uses a plain code span with
  "(feature `control-provider`)".
- D2. No per-crate `cargo doc -p host-core` CI job (root): the fix makes the crate's own command a
  usable gate; a new job is a workflow change outside this issue.

## Authorized paths

`crates/host-core/src/lib.rs` (the crate-level doc only), `crates/host-core/src/prepare.rs` (the
`session_jump_lengths` doc only), this spec.

## Gates

- G1. `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p host-core --no-deps` passes, and so does the same
  command with `--features control-provider` (test value: a feature-gated item linked from always-built
  docs, which the workspace doc form cannot see because feature unification enables it).
- G2. `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --no-deps` still passes.

## Evidence

- After the fix: G1 passes without and with `--features control-provider`; G2 passes. The rendered
  crate doc keeps the `SessionControlProvider` link with the feature.
