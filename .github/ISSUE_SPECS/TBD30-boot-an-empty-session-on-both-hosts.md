# Boot an empty session on both hosts

Owner answer (2026-10-15): an app opens an empty mixer and adds stems through structural edits.
Decision 17 programme. Code anchors verified on `4be10cd67` (PR #1680). Issue number: assigned at filing (written here as TBD30). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A session with no source and no track boots on both hosts (it renders silence at the output), and
a structural edit can then add a stereo stem with its track; both hosts render it identically.

## Context

- A session with no source or no track fails to boot today: host-core's source graph preparation
  refuses it (`graph_failure("host.source.graph.prepare")`, `crates/host-core/src/prepare.rs:2570`),
  at least on the browser; #1562b's gates had to boot with one mono source.
- Since #1562b both hosts accept rebuilds through the one core path, so the first stem can be added
  by a structural transaction.

## Decisions frozen for this slice

- **D1.** An empty session (no source, no track; submixes, VCAs and console slots may be declared)
  is valid and prepares a plan that renders silence at the output, on both hosts, with no special
  path in a host adapter.
- **D2.** Every boot-time size that depends on the session (strip counts, meter frames, staging)
  handles zero, and grows by the existing successor paths when the first strip arrives.

## Objective gates

1. Boot empty on both hosts: the plan renders silence, status, meters (an empty frame) and the
   control list are consistent; zero render allocations.
2. **Parity row:** boot empty, add a stereo stem and a track by one structural transaction, feed the
   stem and hear it, bit-identical on the browser and the C ABI at the four launch rates.
3. The browser qualification gains an empty-boot leg (chromium, firefox, webkit).
4. `cargo test --locked -p host-core --all-features -p control-plane --features test-support -p
   capi -p host-web --features host-web/test-support`; boot budget.

## Authorized paths

- `crates/host-core/src/prepare.rs` (and `shape.rs`), `crates/graph-compiler` if an empty graph
  needs it, `crates/control-plane`, `crates/capi` tests, `hosts/host-web/**` (empty-boot sizes),
  the worklet JS and SDK only where an empty shape is refused today, `hosts/host-web/qualification`,
  this spec's record.

## Dependencies

- #1562 (rebuilds on the browser).

## Size

Half a day to a day.
