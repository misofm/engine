# Issue #579 attempt 2 adversarial review

Reviewed exact pushed head: `62b034c83251c3ea2e37092a81a26f9659b9e948`

Reviewer: Astra LOW

Verdict: **FAIL**

Cached-token checking, nonempty cancellation gating, repeated nonempty lifecycle reuse,
bitwise PostFader comparison, and the mutation against actual fault cancellation are
accepted corrections. Full host-core all-target tests with `control-provider`, 13
endpoint integration tests, strict Clippy, formatting/diff, workspace and host policy,
and Wasm scalar/simd128 compilation passed independently. The reviewed worktree was
clean and upstream.

The final attempt must address four findings:

1. Immediate empty or already-collected cancellation returns the same completion
   twice and leaves publication blocked. Route immediate and cached acknowledgements
   through one exact-once finalization path and test both populations.
2. Native host preparation structurally banks every strip, including padded tails.
   Add only a crate-private `cfg(test)` backend-selection preparation seam in
   `crates/host-core/src/prepare.rs`; production remains pinned to
   `Backend::current()`. Use it from endpoint unit tests to compare separate native-bank
   and forced-scalar endpoints with matching ordinary-console references and read the
   existing pairing/state witnesses.
   `crates/host-core/Cargo.toml` may enable `builtins-compiler/test-support` only on
   its existing dev-dependency because dependency `cfg(test)` is not propagated; the
   normal dependency remains unchanged.
3. Revert mandatory production PostFader meter preparation and the public `meters()`
   expansion. Any extra observers needed solely for evidence belong in private test
   preparation so previously accepted low-meter-cap constructors keep working.
4. Factor actual endpoint queue preparation into the production helper, measure
   current-thread requested bytes/count with owners retained and zero frees/reallocs,
   compare with independent layouts, then prove matching off-render reclamation.
   Largest allocation remains an independent per-layout oracle because the allocator
   does not measure it directly.

No graph, engine, builtins, protocol, public backend override, artifact, lifecycle,
pairing, or lane-B change is authorized. Attempt 3 is the final implementation attempt;
a FAIL triggers the hard stop without weakening gates.
