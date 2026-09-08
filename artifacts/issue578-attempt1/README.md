# Issue #578 attempt 1 source qualification

Qualification was run at pushed HEAD `5065515af4f44c860e40687513b419e3c808c9de` in `/home/bl/misofm/engine-io5-delivery-frame-into`, tracking `origin/codex/io5-delivery-frame-into`. The implementation source is the committed checkpoint `0367244b`; the final HEAD adds only the decision-record update. No source, test, manifest, lockfile, policy, workflow, artifact, browser, benchmark, or GitHub state was changed during qualification. The only working-tree addition is this evidence directory.

Every gate has separate `.command.txt`, `.context.txt`, `.stdout.gz`, `.stderr.gz`, and `.exit.txt` files. Context records UTC, cwd, HEAD, upstream, status, affected-source hashes, and Cargo.lock hash before the command. Gzip members were created with `gzip -n` from the captured raw streams.

## Verdict

All credited qualification gates exited `0`:

| Gate | Command/result |
|---|---|
| Protocol support debug/release | `cargo test --locked -p protocol --features test-support`; release counterpart. Each: unit 158, response API 1, ownership 11, session 3 passed. |
| Protocol default debug/release | `cargo test --locked [--release] -p protocol`. Each: unit 158, response API 1, ownership 10, session 3 passed. |
| #572 ownership | `cargo test --locked -p protocol --test delivery_ownership`: 10 passed. |
| Host default debug/release | `cargo test --locked [--release] -p host-core`: 72 passed, 2 ignored per profile. |
| Host control-provider debug/release | `cargo test --locked [--release] -p host-core --features control-provider`: 88 passed, 2 ignored per profile. |
| Scalar endpoint debug/release | `cargo test --locked [--release] -p host-core --features control-provider --test scalar_point_endpoint`: 9 passed per profile. |
| Strict Clippy | Protocol and host-core all-target/all-feature `-D warnings`: exit 0 each. |
| Rustdoc | Protocol and host-core all-feature `RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps`: exit 0 each. |
| Style | `cargo fmt --all -- --check`; `git diff --check`: exit 0. |
| Policy | Workspace, protocol-control and host-core checks: exit 0 each. |
| Policy mutation suites | Protocol-control and host-core self-test scripts: exit 0 each. |
| Conformance boundary | Checker and full fixture/mutation suite: exit 0 each. |
| Protocol Wasm parity | Scalar + simd128 parity and `--self-test`: exit 0 each; self-test reports 1 inert invocation and 3 red rebuilds. |
| Direct Wasm | Corrected scalar/simd128 `cargo check` for protocol `test-support` and host-core `control-provider`: exit 0 each. |
| Source path census/final status | Corrected exact four-path census and final status: exit 0; HEAD tracks origin and only evidence is untracked. |

## Preserved non-credit diagnostics

`wasm-protocol-scalar.*` records one command serialization mistake: `env RUSTFLAGS=-C target-feature=-simd128 ...` split the assignment and rustc reported `unknown codegen option: --target` (exit 101) before target compilation. The corrected quoted assignment is retained separately for all four direct Wasm checks and passed.

`path-census.*` records one shell assertion mistake using literal `\\n` in the expected string (exit 1). `path-census-corrected.*` uses newline output and passed. Neither diagnostic is a product or target failure; neither was relabeled or overwritten.

## Source identity and census

Final context hashes are stable across the captures:

- `crates/protocol/src/controller.rs`: `9763f44b80866d0f6252d8982c8ecf9b4906f3de8f4b3ebc59016e028139553d`
- `crates/protocol/src/controller_delivery.rs`: `f002cf28d0e8e03bb2b77c7fef910fa4c61dc64dc98be680ea13942c9860d06f`
- `crates/host-core/tests/scalar_point_endpoint.rs`: `609c48ab83fe13c573bcf66a43189f36e1decc13ac421cc09bca88265e4d448b`
- `Cargo.lock`: `92db9698cc062bd5ccc0a90f0ee43f051d77ce7e52f785ce9718fae3f400b753`

The corrected census proves the base-to-HEAD tracked path set is exactly `.github/ISSUE_SPECS/578-controller-delivery-frame-into.md`, `crates/host-core/tests/scalar_point_endpoint.rs`, `crates/protocol/src/controller.rs`, and `crates/protocol/src/controller_delivery.rs`.

## Manifest

`sha256sums.txt` covers every retained command, context, compressed stdout/stderr, exit record, README, and preserved non-credit diagnostic except the manifest itself. Verify from the repository root:

```text
sha256sum -c artifacts/issue578-attempt1/sha256sums.txt
```
