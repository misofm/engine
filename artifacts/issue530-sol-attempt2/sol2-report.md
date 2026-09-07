# Issue #530 Sol attempt 2 evidence

## Source identity

The clean final gates ran at `09ef76f78932480c02b25574b81874bd66dd1ccc` with an empty porcelain status. The correction changes only the facade rustdoc and its existing Group 4/5 unit fixtures; runtime code is unchanged.

| Path | SHA-256 | Git blob |
|---|---|---|
| `crates/protocol/src/controller.rs` | `beb027014e4e55cb8f6005142c9839d361dd6d01d135893c8d7d092e133a0edf` | `63341f8bd62944ec4bcb38fcd0eade9a53a414b8` |
| `crates/protocol/src/controller_delivery.rs` | `efde0b8a7763519d593e81238dc7b2607064ba12ad93fbb04127962d88d9454c` | `80520ed6c56730379ef4dfa68d57bfcad9ff9894` |
| `crates/protocol/src/delivery.rs` | `0c711032042aad905fc2111319488a7680ff6e2d1063bd0a8760d2e805e6cd8d` | `bd66a50ab4c47633e8c9849bed1baee122873fad` |
| `crates/protocol/src/lib.rs` | `6d53a61a0aaa07196c35d3b07af1ab0ac33945e8aad28efd824136fccde862d7` | `e20e85380a5196a26b3978ee68566a2c6699e0c0` |
| `crates/protocol/tests/delivery_ownership.rs` | `a5fee48f9f6891aea721a3095c514b0c24db1ba3715bdf78c2b1de5f13ccae97` | `451c7870d3ac54ed8bf40b488390fc912e0a7699` |
| `Cargo.lock` | `ef85bfaac8b4df80b651fa89e5e9a67bfef141b58076bdaac513044161b0b853` | `1b74fb016a04a090d2d1d37ea15fd2917fb85491` |

## Sol2 checks

All raw records are `/tmp/issue530-sol2/<label>.{command.json,stdout,stderr,status}`. The capture helper is `/tmp/issue530-sol2-capture.py`; it records exact argv, cwd, HEAD, porcelain status, environment, and source hashes.

| Label | Command and result |
|---|---|
| `focused-debug-2` | `cargo test --locked -p protocol controller_delivery::tests -- --nocapture`; 5 passed, status 0; captured before checkpoint with the corrected file dirty at HEAD `9fd9c1f993313b6e2f483af4d40861c178daed54` |
| `sol2-focused-release` | `cargo test --locked -p protocol --release controller_delivery::tests`; 5 passed, status 0 |
| `sol2-delivery-ownership-debug` | `cargo test --locked -p protocol --features test-support --test delivery_ownership`; 3 passed, status 0, including Gate 6 allocation/realtime ownership evidence |
| `sol2-clippy` | `cargo clippy --locked -p protocol --all-targets --all-features -- -D warnings`; status 0 |
| `sol2-rustdoc` | `cargo doc --locked -p protocol --no-deps`; status 0 |
| `sol2-fmt` | `cargo fmt --all -- --check`; status 0 |
| `sol2-source-diff` | `git diff --check`; status 0 |
| `sol2-policy-workspace` | `bash scripts/check-workspace-policy.sh .`; status 0 |
| `sol2-policy-realtime` | `bash scripts/check-realtime-policy.sh .`; status 0, 42 regions in 12 files |
| `sol2-policy-protocol` | `bash scripts/check-protocol-control-policy.sh .`; status 0 |

The final proportional tranche contains 9 clean-checkpoint commands. Across the two final test commands, 8 tests passed (5 focused release and 3 ownership debug). Including the precommit corrected focused debug run, the correction has 13 passing test executions across 3 commands. There are 14 Sol2 raw command captures total: 5 implementation-stage captures and 9 final clean-checkpoint captures.

The five focused functions remain in `crates/protocol/src/controller_delivery.rs`. Group 4 now uses a supported two-record ticket with prefix one and checks a state-changing blocked transport request, before/after state, cached refusal, new-ID success, exact reliable events, released credit, and one provider cancellation observation. Group 5 uses a valid nonempty persistent edit and retains the pre-command canonical model, revision, decoded transport state, real admitted owner, and exact payload. Groups 1-3 and 6 remain unchanged from accepted Luna attempt 1 evidence.

## Preserved preliminary failures

- `format` has status 1 because the first `cargo fmt --all -- --check` correctly reported the new fixture/doc formatting diff. `format-apply` and `format-repair` each have status 0.
- `focused-debug` has status 101: Group 4 and Group 5 initially reused request IDs after advancing the replay cursor, so the fixture observed `ReplayExpired` instead of its expected `Unavailable`. The raw output is retained. Request IDs were made monotonic; this exposed no runtime defect. `focused-debug-2` is the corrected 5/5 pass.

## Reused unchanged evidence

The correction changes only rustdoc and unit-fixture bodies, so the broad protocol behavior, Wasm target compilation, and policy self-test implementations are unchanged. Their Luna attempt 1 raw captures remain under `artifacts/issue530-luna-attempt1`:

| Evidence | Luna source identity and result |
|---|---|
| `protocol-default-gate`, `protocol-support-gate` | HEAD `03ae4c14f234a48e627f497d57f67280bdff207c`, controller-delivery SHA-256 `8425aef38d1fc90818a228fd7b30ed7fc9b74ac8dad3a75524ca3c5ece221e6e`; both status 0 |
| `wasm-scalar-protocol`, `wasm-simd-protocol` | HEAD `81b8221435374ec7afcb015ce5a2ee9b8c712daf`, controller-delivery SHA-256 `e1143c7c15d10ddff3179bc22caa03e9bf2059869c7cc92085f26a1849ebebb2`; both status 0 |
| `policy-workspace-test`, `policy-realtime-test`, `policy-protocol-test` | HEAD `03ae4c14f234a48e627f497d57f67280bdff207c`, controller-delivery SHA-256 `e1143c7c15d10ddff3179bc22caa03e9bf2059869c7cc92085f26a1849ebebb2`; all status 0 |

These reused commands are attributed to their actual Luna source identities rather than the Sol2 checkpoint. No full workspace, browser, benchmark, or repeated broad target gate was run in Sol2.
