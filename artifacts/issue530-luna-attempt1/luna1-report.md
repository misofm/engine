# Issue 530 Luna attempt 1 report

Current worktree: `/home/bl/misofm/engine-controller-delivery`

Current clean `HEAD`: `81b8221435374ec7afcb015ce5a2ee9b8c712daf`

Capture helper: `/tmp/issue530-luna1-capture.py`. Every listed command has matching
`.command.json`, `.stdout`, `.stderr`, and `.status` files under `/tmp/issue530-luna1/`.

## Source identities

The final captured source set (SHA-256; the command metadata also records Git blob IDs) is:

| Path | SHA-256 |
| --- | --- |
| `crates/protocol/src/controller.rs` | `beb027014e4e55cb8f6005142c9839d361dd6d01d135893c8d7d092e133a0edf` |
| `crates/protocol/src/controller_delivery.rs` | `e1143c7c15d10ddff3179bc22caa03e9bf2059869c7cc92085f26a1849ebebb2` |
| `crates/protocol/src/delivery.rs` | `0c711032042aad905fc2111319488a7680ff6e2d1063bd0a8760d2e805e6cd8d` |
| `crates/protocol/src/lib.rs` | `6d53a61a0aaa07196c35d3b07af1ab0ac33945e8aad28efd824136fccde862d7` |
| `crates/protocol/tests/delivery_ownership.rs` | `a5fee48f9f6891aea721a3095c514b0c24db1ba3715bdf78c2b1de5f13ccae97` |
| `Cargo.lock` | `ef85bfaac8b4df80b651fa89e5e9a67bfef141b58076bdaac513044161b0b853` |

## Successful gates

| Capture label | Command/result |
| --- | --- |
| `first-compile` | `cargo check -p protocol`; status 0 (initial compile capture, native `/tmp/issue530-target`) |
| `final-functional-debug-4` | `cargo test --locked -p protocol controller_delivery::tests`; 5 passed, status 0 |
| `focused-debug-gate` | same focused tests; 5 passed, status 0 |
| `focused-release-gate` | `cargo test --locked -p protocol --release controller_delivery::tests`; 5 passed, status 0 |
| `delivery-debug-gate` | `cargo test --locked -p protocol --features test-support --test delivery_ownership`; 3 passed, status 0 |
| `delivery-release-gate` | same with `--release`; 3 passed, status 0 |
| `protocol-default-gate` | `cargo test --locked -p protocol`; 144 library tests plus integration groups 3, 1, 2, 1, 3; all status 0 |
| `protocol-support-gate` | `cargo test --locked -p protocol --features test-support`; 144 library tests plus integration groups 3, 1, 3, 1, 3; all status 0 |
| `clippy-protocol-2` | `cargo clippy --locked -p protocol --all-targets --all-features -- -D warnings`; status 0 |
| `rustdoc-protocol` | `cargo doc --locked -p protocol --no-deps`; status 0 |
| `fmt-check` | `cargo fmt --all -- --check`; status 0 |
| `policy-workspace-check`, `policy-workspace-test` | workspace policy check and mutation tests; status 0 |
| `policy-realtime-check`, `policy-realtime-test` | realtime policy check and mutation tests; status 0 |
| `policy-protocol-check`, `policy-protocol-test` | protocol-control policy check and mutation tests; status 0 |
| `policy-diff-check` | `git diff --check`; status 0 |
| `wasm-scalar-protocol` | `RUSTFLAGS='-C target-feature=-simd128' CARGO_TARGET_DIR=/tmp/issue530-target-wasm-scalar cargo check --locked --target wasm32-unknown-unknown -p protocol`; status 0 |
| `wasm-simd-protocol` | `RUSTFLAGS='-C target-feature=+simd128' CARGO_TARGET_DIR=/tmp/issue530-target-wasm-simd cargo check --locked --target wasm32-unknown-unknown -p protocol`; status 0 |
| `root-lint-focused` | root corrected-source focused debug run; 5 passed, status 0 |
| `root-lint-ownership` | root corrected-source ownership run; 3 passed, status 0 |

Gate6 is feature-gated because `MockProvider` is exported only under `test-support`. It is run
by the 3-test `delivery_ownership` support commands. The allocator assertion measures fresh A
facade preparation, B ordinary controller preparation, C standalone delivery preparation, and D
queue preparation with equivalent owned inputs outside measured closures, and asserts allocation
count/requested bytes `A = B + C - D`, zero preparation frees, and exact queue-and-delivery report
composition. No second allocator or dependency was added.

## Failure history and attribution

One additional direct, uncaptured `cargo check -p protocol` was run before the capture helper was
created; it exited 0. The unlocked `cargo test -p protocol --test delivery_ownership` is captured
as `focused-debug.*` with status 0. No failed uncaptured compile or test command is known. All
later Cargo commands use `--locked`.

Functional fixture failures are preserved in `functional-debug-1` through `functional-debug-4`:
the first two were test-import/lifetime/array-shape errors, `functional-debug-3` was a mutable
render-buffer borrow error, and `functional-debug-4` exposed one incorrect fixture expectation
(the second batch had not yet been admitted). They were corrected within the focused fixture;
`functional-debug-5` and later focused labels passed. `gate6-debug-1` recorded an incorrect
`session::SessionStore` path; `gate6-debug-2` recorded preparation frees caused by constructing
session/provider inputs inside the measured closure. Both were corrected in the existing fixture;
`gate6-debug-3` passed.

The first strict Clippy run, `clippy-protocol`, failed only on `needless_option_as_deref` and
`too_many_arguments` for the frozen 8-argument constructor. The bounded correction changed the
optional borrow to `as_mut()` and added a source-local justified allow for the frozen signature;
`clippy-protocol-2` passed. The root corrected-source captures `root-lint-focused` and
`root-lint-ownership` independently confirm the post-correction focused behavior. No later source
changes occurred before the isolated scalar/SIMD target checks.

The facade tests provide finite evidence for all six numbered groups: actual typed admission and
decoded fields/replay; durable ownership credit and terminal collection; whole mixed unsupported
FIFO cancellation; one queue/cursor with real boundary acknowledgment, event sequence/effective
sample/counts, provider cancellation accounting, stale poll and short-output retry; fixed-revision
refusals with usable metadata/snapshot/transport reads; and one allocation authority with the
existing allocator fixture. These are focused fixtures and do not claim DSP/host/CAPI acceptance.

## Exact checkpoint attribution

Focused debug/release, ownership debug/release and full default/support protocol gates ran on clean `03ae4c14`, before the two-line Clippy correction. The full default suite totals 154 passed; test-support totals 155 passed; both have zero failures/ignored. Corrected Clippy, documentation/policies and root focused/ownership reruns captured the dirty correction above `03ae4c14`; their source hashes match committed `81b82214`. Scalar/SIMD checks ran on clean `81b82214`. Reused pre-lint release/full-suite results keep their actual identity; no claim is made that they reran at `81b82214`. The additional uncaptured initial successful compile is not acceptance evidence.
