# Issue #578 attempt 2 source qualification

Qualification ran from clean pushed HEAD `8f5b2e4072d1aa0a7deb36f1628eae755bb83822` in `/home/bl/misofm/engine-io5-delivery-frame-into`, on branch `codex/io5-delivery-frame-into` tracking `origin/codex/io5-delivery-frame-into`. The source correction is checkpoint `b023eec6`; HEAD adds its synchronized decision record. Qualification added only this evidence directory.

Each gate has separate `.command.txt`, `.context.txt`, `.stdout.gz`, `.stderr.gz`, and `.exit.txt` files. Context was captured immediately before each command and records UTC, cwd, HEAD, branch, upstream, status, and SHA-256 identities for the issue spec, affected source files, and `Cargo.lock`. Captured streams were compressed losslessly with `gzip -n`.

## Verdict

All recorded gates exited `0`.

| Gate | Result |
|---|---|
| Protocol default debug/release | `cargo test --locked -p protocol` and release counterpart: each 160 unit, 1 response API, 10 ownership, 3 session tests passed. |
| Protocol `test-support` debug/release | Each: 160 unit, 1 response API, 11 ownership, 3 session tests passed. |
| Unchanged #572 ownership | `cargo test --locked -p protocol --test delivery_ownership`: 10 passed. |
| Host-core default debug/release | Each: 72 passed, 2 ignored across the workspace package test binaries. |
| Host-core `control-provider` debug/release | Each: 88 passed, 2 ignored across the workspace package test binaries. |
| Scalar endpoint debug/release | Each: 9 passed. |
| Strict Clippy | Protocol and host-core all-target/all-feature `-D warnings`: exit 0 each. Existing clippy.toml reachability warnings were emitted; neither command failed. |
| Rustdoc | Protocol and host-core all-feature `RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps`: exit 0 each. |
| Style | `cargo fmt --all -- --check` and `git diff --check`: exit 0. |
| Policy | Workspace, protocol-control, and host-core checker scripts: exit 0 each. |
| Policy mutations | Protocol-control and host-core self-test scripts: exit 0 each. |
| Conformance | Boundary checker and complete fixture/mutation suite: exit 0 each. |
| Protocol Wasm parity | Scalar and simd128 parity plus `--self-test`: exit 0; parity reports scalar+simd128 golden parity, and self-test reports 1 inert invocation plus 3 red rebuilds. |
| Direct Wasm | Correctly quoted scalar/simd128 `cargo check` for protocol `test-support` and host-core `control-provider`: exit 0 each. |
| Path census | Source tranche `bb7bdff1..b023eec6` is exactly the three authorized source/test paths; checkpoint delta `bb7bdff1..HEAD` is exactly those three plus the issue spec. Both exit 0. |
| Final status | `git diff --check origin/main...HEAD` and HEAD/upstream/status report exit 0; branch is synchronized and only this evidence directory is untracked. |

## Source identities

The pre-command contexts consistently record these SHA-256 values:

- `.github/ISSUE_SPECS/578-controller-delivery-frame-into.md`: `7a1d2ed7346947ad62b9381715dc959934c8ef191737a64ffaa508ad192d685e`
- `crates/protocol/src/controller.rs`: `541c03a82a2eedf770452f59fcda1acd4bce325c5563b16faff0bb0e43470e39`
- `crates/protocol/src/controller_delivery.rs`: `97af18a85e47049ccac77dc6121fba3aaf47a49872f0fa5b3e99dcad2fd0b0cb`
- `crates/host-core/tests/scalar_point_endpoint.rs`: `3d89ad8379657917dd9bb1c927359e8aeeef2aecb7e5dd166536a6feeef84908`
- `Cargo.lock`: `92db9698cc062bd5ccc0a90f0ee43f051d77ce7e52f785ce9718fae3f400b753`

## Manifest verification

`sha256sums.txt` covers every retained command, context, compressed stdout/stderr, exit record, and this README. From the repository root:

```text
sha256sum -c artifacts/issue578-attempt2/sha256sums.txt
```
