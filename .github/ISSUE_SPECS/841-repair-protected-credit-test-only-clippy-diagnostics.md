# Repair protected-credit test-only Clippy diagnostics

## Problem and smallest outcome

Protected credit-refusal source passed independent issue #839 review at upstream `9131ada54f143d8176fee7e0f0aec1e9db37cb75`, but PR #840 required qualification run `35046415805` failed strict workspace Clippy on exactly three **test-only** diagnostics. The frozen PR merge tree matches the accepted head. Repair the named fixtures so required lint passes without changing #839 production or the discrimination of its allocation/free, PCM, receipt and credit gates. Artifact promotion is a separate successor. This issue is a CI integration repair, not another #839 implementation attempt, browser qualification or app adoption.

## Frozen boundary

Baseline is `9131ada5`; work on dedicated dependency branch `codex/ci-unblock-840` from that exact head. Allowed edits only inside `hosts/host-web/src/ffi.rs` test modules and `hosts/host-web/src/tests.rs` test fixtures, plus this numbered decision/evidence record:

- `ffi.rs` alias matrix `let calls: [(&str, fn(u32)->u32);8]`: use a named test-only fixture type preserving all eight calls, 48 measured invocations and full-state assertions.
- The alias is local and uses `(&'static str, fn(u32) -> u32)`; names are born unversioned.
- `ffi.rs` Pending receipt equality `receipts.iter().any(|receipt| *receipt == first_receipt)`: use `receipts.contains(&first_receipt)` with identical equality contract.
- `tests.rs` `ProtectedNativeState.input_filter_shadows` tuple: use a named test-only type preserving all committed/candidate bit fields, dirty bits and revision.

Do not add lint allow, weaken/rewrite the snapshots, change production Rust/ABI/JS/SDK/artifacts/build flags, or broaden the test corpus. If more than these three diagnostics remain after one bounded correction, attribute them before expanding the issue.

## Objective gates

The authentic baseline strict Clippy command exits101 with these three findings (`/tmp/miso-839-ci-policy.log`). In that PR run, documentation/later policy steps inside lint, SDK/artifact-gate/browser jobs were skipped, so they are not claimed green. Final exact pinned-toolchain command must pass: `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`. Focused alias/Pending/native state fixtures and `cargo test --locked -p host-web --features test-support` must pass, preserving issue #839 baseline 195 unit+2 integration, 2 unit ignored, including measured zero allocation/free and nonzero bit-identical PCM. `cargo fmt --all -- --check`, `git diff --check` pass. No artifact/browser qualification claim.

## Workflow and evidence

Fresh Astra XHIGH scope `/tmp/miso-839-ci-successor-scope.md` and separate fresh Astra XHIGH adversarial plan review `/tmp/miso-839-ci-successor-adversary.md` returned GO for this smallest test-only outcome. At issue boundary, audit local numbered specs vs `gh issue list --state all`, create matching GH issue in same pushed brief checkpoint, confirm actual number/title/body and open state. First Luna MAX handoff repairs the three findings in one two-file tranche; two **issue-wide** fresh bounded Luna MAX attempts at most, then one Sol HIGH and one Astra XHIGH if necessary; every submitted candidate gets one fresh independent Astra MEDIUM adversarial verdict. Hard four active hours incl review. Root commits exact-path coherent green checkpoints, pushes promptly to dependency branch, refreshes GH evidence, closes only after PASS evidence upstream/body exact/state completed, even while PR#840 remains blocked; body must say lint repair accepted and PR/main qualification pending. Keep PR #840 head frozen while successors accumulate; no new PR or premature CI event. After this lint repair is accepted, artifact promotion may start atop its frozen build-source checkpoint. Remote PR/main qualification and deployability remain pending.
