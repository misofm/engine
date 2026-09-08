# Issue #600 attempt 1 review

Reviewer: Astra LOW

Verdict: **FAIL**

Exact pushed head: `c735e5d5172b446fdc6c515784a24f70eeeed982`

Two implementation attempts remain. Timing is not authorized.

The source and paths are disjoint from #598, and the safe validator/lifecycle, strict Clippy,
rustdoc, formatting, diff, and unchanged #238/#496 builtins gates pass. The implementation is not
capture-ready for these reasons:

1. The real preflight encodes an empty Rust flag before `-Ctarget-feature`, so its claimed release
   build fails while the fake-Cargo test misses the defect.
2. The untimed tests call an owner method that always invokes the timing helper, and the timed
   closure also includes output-buffer construction rather than only `PreparedRenderPlan::render`.
3. Drain, no-pending-work and continuing trim/ramp-state witnesses are missing; the positive digest
   is self-derived; counters and timing use unchecked/saturating/clamped arithmetic.
4. A permanent `Mode::Suppress` branch plus caught assertion is not the required temporary restored
   source mutation.
5. The validator does not strictly freeze metadata/missing values, numeric JSON types, duplicate
   keys, or required seal keys and accepts arbitrary matching owner/output digests.
6. Post-child persistence failures can lose scratch evidence; reported runner/timing statuses can
   disagree with actual execution; and the runner reports a repository cwd without changing to it.
7. Lifecycle tests omit source/binary identity changes and post-workload persistence/publication
   failures, and their reported synthetic launch count is wrong. The preflight also permits
   inherited profile overrides while claiming fixed build flags.

The earlier suspected dispatcher-argument conflict was withdrawn after checking the dispatcher's
re-exec behavior and is not a finding. No real subject, final preflight, capture, or direct mutation
ran. Attempt 2 must correct the bounded subject/harness evidence above without widening paths.
