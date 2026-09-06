# Issue 496 Luna attempt 1 report

The implementation checkpoint is in `crates/builtins/src/lib.rs`. The private post ramp helper
extracts each compared word once and is called at exactly the dual and collapsed post ramp seams.
No production layout, DSP arithmetic, public API, or control behavior changed.

Proof results:

- W1/W4/W8, partial members, padding, signed `+0.0` versus `-0.0`, trim/target/step/section-word
  differences, countdown differences, and independent oracle comparisons pass.
- Actual dual and mono post ramp seams observe 30 lane word extractions on a symmetric W8 bank.
- Settled blocks do not invoke the helper.
- Old full refresh mutation at the dual seam fails the same exact assertion with 240 versus 30;
  source was restored and passing.
- Exact named tests pass in debug and release.
- Builtins library, builtins input liveness, builtins mono liveness, and host-core console liveness
  pass in debug and release.
- Builtins clippy, builtins/realtime/lane/workspace policies, and fmt check pass.

Raw subprocess evidence (each has `.stdout`, `.stderr`, and `.meta` captured during invocation):

- Pre edit optimized IR: `/tmp/496-luna1-preedit-ir-1788674856503609026.*` (the earlier failed
  PATH invocation is retained separately; this is the successful second inspection).
- Mutation failure: `/tmp/496-luna1-mutant-1788675232903650463.*`.
- Exact debug/release tests: `/tmp/496-luna1-exact-*` and `/tmp/496-luna1-adversarial-*`.
- Full library debug/release: `/tmp/496-luna1-libfull-*`.
- Integration debug/release: `/tmp/496-luna1-integration-*`.
- Policy and fmt checks: `/tmp/496-luna1-policy-*`.

The raw metadata records the current source HEAD, actual `git hash-object` blob, SHA256, cwd,
effective PATH/RUSTFLAGS/CARGO_TARGET_DIR, argv, and exit status. No benchmark or timing command
was run.
