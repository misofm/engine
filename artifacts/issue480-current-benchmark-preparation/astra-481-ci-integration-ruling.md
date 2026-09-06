# PR481 qualification integration ruling — APPROVED

At PR head6a828682, the original473 workflow invokes the scratch lifecycle suite directly at qualification.yml:326. The CI log records CARGO_INCREMENTAL=0; the retained local reproduction exits1 with the intended incompatible-build-environment diagnostic. Separately, CI's formatting diff at builtins.rs:2014 is exactly the multiline wrapping of one assert_eq. The current working Rust diff matches that formatting-only correction.

Approve the narrow delivery amendment before checkpoint:

- Replace ONLY the original473 suite invocation with `env -u CARGO_INCREMENTAL bash scripts/test-builtins-current-benchmark.sh` in the existing workflow block. This gives the fake-only suite its intended baseline environment; its directed CARGO_INCREMENTAL refusal cases still explicitly supply the variable. Do not unset it globally, weaken the real preflight/runner refusal, bypass any case, or change another CI step.
- Retain the exact rustfmt wrapping of the one parent benchmark regression assertion. Verify token/whitespace-normalized equivalence and no other Rust delta. This changes neither the benchmark input guard nor the workload.

These are newly observed qualification integration corrections in original473-owned paths, not a fourth480 source attempt or new feature. Record the initial failed CI job and raw diagnostic plus the failed local reproduction honestly. Refresh applicability/provenance records to distinguish the formatting change from prior exact-byte source identity; do not continue claiming literal unchanged tools/bench bytes.

Proportional validation: syntax/static workflow-diff inspection, fmt, the full existing fake suite through the new invocation WITH inherited CARGO_INCREMENTAL=0, its original negative override cases and twenty-record lifecycle proof, existing validator-delta and packaged environment checks. All fake checks must still report zero real workload launches. No full workspace, real benchmark preparation/build/runner/timing is justified by these mechanical changes alone; retained parent benchmark test/Clippy results remain applicable after verified formatting equivalence.

The previous PR head is not merge-approved after failed CI. Root must checkpoint/package the corrected source and retained evidence, obtain a new exact-head actual-PR Astra review and required CI SUCCESS before merge. No edits or tests performed by this ruling.
