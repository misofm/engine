# Issue 563 attempt 3 Luna XHIGH result

Outcome: **PASS**

Starting head: `4171bf63f195f0cc855e047291d8c50038c11f46`

Source commit: `a0dcc74b106c8807fed1173cdf6d44b79180cb85`

Only `tools/bench-support/src/alloc.rs` changed. Six adjacent `SAFETY` comment lines now document the four unsafe allocator test operations. No executable semantics changed.

The exact CI Clippy invocation passed first: `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`. All 39 bench-support tests, the complete nine-test scalar endpoint debug suite, formatting, diff check, and exact one-source-path census also returned zero.

Final source SHA-256 identities:

- `tools/bench-support/src/alloc.rs`: `628ed65e2fa6c6dedd24aae6840d6b0b3d8366f1d22f44f217e21eb99bccc5cf`
- `crates/host-core/tests/scalar_point_endpoint.rs`: `9f3c4f9fc628bf4a90a5e5ee1413be8982281c52eedf8b3b677c14681c2228cf`
