# Issue #575 attempt 1 consolidated source qualification

Qualification was run at pushed HEAD `4307c0dc746b1ad4faa9fcd2acf2d846581e7ee7` in `/home/bl/misofm/engine-io5-btlv-scalar-ingress`, with production source frozen at implementation checkpoint `7d50c93615cc553a9e57198261c370fc925397ed`. Each retained gate has a `.command.txt`, `.context.txt`, `.stdout.gz`, `.stderr.gz`, and `.exit.txt`; context records UTC, cwd, HEAD, worktree status, and source/Cargo.lock hashes captured before that command. Gzip members were written with `gzip -n`.

## Verdict

All proportional source gates below passed with numeric exit `0`. No product source, test, spec, manifest, lockfile, or script was edited during qualification. The only untracked path is this evidence directory. No benchmark, browser qualification, artifact repin, commit, push, or GitHub operation was performed.

| Gate | Exact command | Result |
|---|---|---|
| Protocol full debug | `cargo test --locked -p protocol --features test-support` | PASS; 154 unit + 1 response API + 11 ownership + 3 session tests passed, 0 failed |
| Protocol full release | `cargo test --locked --release -p protocol --features test-support` | PASS; 154 unit + 1 response API + 11 ownership + 3 session tests passed, 0 failed |
| Host scalar endpoint debug | `cargo test --locked -p host-core --features control-provider --test scalar_point_endpoint` | PASS; 9 passed, 0 failed |
| Host scalar endpoint release | same with `--release` | PASS; 9 passed, 0 failed |
| Host default debug/release | `cargo test --locked [--release] -p host-core` | PASS; 72 passed, 2 ignored per profile |
| Host feature full debug | `cargo test --locked -p host-core --features control-provider` | PASS; 88 passed, 2 ignored |
| Host feature full release | `cargo test --locked --release -p host-core --features control-provider` | PASS; 88 passed, 2 ignored |
| Protocol default debug/release | `cargo test --locked [--release] -p protocol` | PASS; 169 passed per profile |
| #572 ownership | `cargo test --locked -p protocol --test delivery_ownership` | PASS; 10 passed, 0 failed |
| Strict Clippy | protocol and host-core `cargo clippy --locked -p ... --all-targets --all-features -- -D warnings` | PASS; exit 0 each |
| Rustdoc | protocol and host-core `RUSTDOCFLAGS=-Dwarnings cargo doc --locked -p ... --all-features --no-deps` | PASS; exit 0 each |
| Formatting/diff | `cargo fmt --all -- --check`; `git diff --check` | PASS |
| Policies | workspace, protocol-control, host-core check scripts | PASS |
| Policy mutation suites | protocol-control and host-core test scripts | PASS |
| Conformance boundary | checker and full fixture/mutation suite | PASS |
| Wasm parity | `bash scripts/check-protocol-wasm-parity.sh` | PASS; scalar + simd128 |
| Wasm parity mutations | `bash scripts/check-protocol-wasm-parity.sh --self-test` | PASS; 1 inert invocation + 3 red rebuilds |
| Direct Wasm protocol | scalar and `+simd128` `cargo check --locked --target wasm32-unknown-unknown -p protocol --features test-support` | PASS |
| Direct Wasm host-core | scalar and `+simd128` `cargo check --locked --target wasm32-unknown-unknown -p host-core --features control-provider` | PASS |
| Final status/upstream | `git diff --check origin/main...HEAD` plus status/upstream query | PASS; clean, tracking origin |

The protocol Clippy output contains warnings emitted while checking dependency packages, but the strict invocations themselves exited zero and produced no lint failure.

## Source census

The exact committed source delta from `7d50c936` is:

- `crates/host-core/tests/scalar_point_endpoint.rs`
- `crates/protocol/src/controller/tests.rs`
- `crates/protocol/src/controller_delivery.rs`

`path-census` verifies those three paths exactly, no working-tree changes outside `artifacts/issue575-attempt1/`, clean diff-check, and unchanged Cargo.lock SHA-256 `92db9698cc062bd5ccc0a90f0ee43f051d77ce7e52f785ce9718fae3f400b753`. Per-gate contexts retain the contemporaneous hashes for the affected source files and lockfile.

## Capture note

`clippy-protocol-command-mistake.*` is a preserved non-credit diagnostic from a capture-helper invocation typo: the helper was given a nested directory as its output argument and consequently attempted to execute `clippy`, returning 127 before Cargo ran. The corrected `clippy-protocol.*` record is the sole credited protocol Clippy gate. No source or product command was affected.

## Payload manifest

`sha256sums.txt` covers every retained command, context, compressed stdout/stderr, exit record, this README, and the non-credit diagnostic, excluding only the manifest itself. Verify with:

```text
sha256sum -c artifacts/issue575-attempt1/sha256sums.txt
```
