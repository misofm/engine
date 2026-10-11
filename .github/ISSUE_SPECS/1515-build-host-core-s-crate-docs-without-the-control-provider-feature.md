# Build host-core's crate docs without the control-provider feature

## Product outcome

`cargo doc -p host-core --no-deps` with `RUSTDOCFLAGS="-D warnings"` passes with the default feature set
and with `--features control-provider`, so the crate's own doc command is a usable gate.

## Context

- `crates/host-core/src/lib.rs:19`, in the crate-level doc, links `[`SessionControlProvider`]`. That type
  exists only under the `control-provider` feature (`crates/host-core/Cargo.toml:15`,
  `control-provider = ["dep:protocol"]`). Without the feature, rustdoc reports a broken intra-doc link, so
  `cargo doc -p host-core --no-deps -D warnings` fails on `main`.
- The workspace form (`cargo doc --workspace`) enables the feature through feature unification, so it
  passes; #1498's gate 3 had to use that form for this reason (reported by the #1498 verdict).

## Decisions

- D1. The crate-level doc names `SessionControlProvider` as a link only when the feature is enabled:
  use `#[cfg_attr(feature = "control-provider", doc = "...")]` for the linked sentence and a plain code
  span otherwise, or move the sentence onto the feature-gated module's own doc. Either way the wording
  stays the same; no code changes.

## Authorized paths

`crates/host-core/src/lib.rs` (the crate-level doc only), this spec.

## Gates

- G1. `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p host-core --no-deps` passes, and so does the same
  command with `--features control-provider` (test value: a feature-gated item linked from always-built
  docs, which the workspace doc form cannot see because feature unification enables it).
- G2. `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --no-deps` still passes.
