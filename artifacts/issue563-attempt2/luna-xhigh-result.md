# Issue 563 attempt 2 Luna XHIGH result

Outcome: **PASS**

Starting head: `b87c9c0accf4447492c749a2493a9ebf7adebcb9`

Source commit: `6edf0992d580340e9e1da8402f1ce2ec01cfcc88`

Only `crates/host-core/tests/scalar_point_endpoint.rs` changed. The host foreign-thread probe now asserts exact equality between `foreign_current_thread` and `bench_support::alloc::Counters::default()` beside the existing realtime-audit zero assertions.

The complete nine-test scalar endpoint debug suite, strict affected-target Clippy, `cargo fmt --all --check`, `git diff --check`, and exact one-source-path census each returned status zero. The source SHA-256 is `9f3c4f9fc628bf4a90a5e5ee1413be8982281c52eedf8b3b677c14681c2228cf`.
