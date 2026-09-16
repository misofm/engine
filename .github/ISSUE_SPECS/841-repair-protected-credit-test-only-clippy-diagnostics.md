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

## Decision/evidence

Issue boundary audit found **430 local numbered specs, 534 remote issues, no local-numbered spec missing remotely and no recent title mismatch**; #839 closed/completed with exact body, #824 remains deferred/open. GitHub #841 was allocated with this exact title/body, then local numbered spec and #839 failed-PR delivery record were committed together at `6be696f1`, pushed to non-PR dependency branch. Both GH #839/#841 bodies and states were verified exact; PR#840 head remained frozen `9131ada5`.

Fresh Luna MAX issue-wide attempt 1 changed only the three frozen test seams in `ffi.rs` and `tests.rs`; root audited/committed/pushed exact-path checkpoint `3d16ba6d` after agent paused. The named local FFI alias preserves all eight entries and 48 measured calls, the Pending receipt assertion now uses the same full equality via `.contains`, and the input-filter snapshot tuple has a test-only name with identical fields. The exact pinned strict `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` passed; focused alias/Pending/native-state fixtures passed; full locked Host-Web suite passed **195 unit and two integration, two unit ignored**; workspace rustfmt and diff checks passed. Raw Luna log `/tmp/miso-841-luna-max1.log`, final `/tmp/miso-841-luna-max1.md`. Fresh independent Astra MEDIUM attempt-1 verdict remains pending. This is lint-only source acceptance, **not** final PR/main qualification or app delivery.
