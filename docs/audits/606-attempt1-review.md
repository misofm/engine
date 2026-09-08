# Issue 606 attempt 1 adversarial review

Reviewer: Astra LOW

Reviewed head: `b148ffc079ce0ed51e205fc188e6dd0648fa3aa3`

Verdict: **FAIL**. Issue 606 stops and must split because its brief authorizes no runner
correction. Final preflight and capture remain unauthorized.

The decisive blocker is the exact preflight build recipe. It applies `-C lto=fat` globally through
`CARGO_ENCODED_RUSTFLAGS`, including to Cargo build dependencies and build scripts. A compile-only
review using those exact flags and `cargo build --locked --release -p bench --bin bench` exited 101:
Rust rejected the combination of Cargo's `-C embed-bitcode=no` with `-C lto` while compiling
`version_check`, `proc-macro2`, and other build dependencies. No prepared executable or workload ran.

The local and upstream heads matched, the tree was clean, current main was contained, and issue 605's
paths were disjoint. Rust sources, the fixture, and `Cargo.lock` remained unchanged. The guarded stub
publication lifecycle passed, including real collision behavior and retained recovery bytes. All 19
repository-root checksum-manifest entries verified, and the preserved mutation/restoration evidence
matched. Ordinary debug/release checks do not exercise the failing preflight flags and therefore do
not establish capture readiness.

The strict fixed workload makes some later record/seal comparisons redundant because each value must
already equal the same constant. The mutation matrix proves rejection of changed fields rather than
execution of every redundant branch; the reviewer found no acceptance hole from that limitation.

No final preflight, real `input-symmetry-capture` invocation, or timer ran. A narrow successor must
repair and compile-check the release build recipe before it may promote the preserved issue-606
tooling and frozen issue-602 Rust capture contract.
