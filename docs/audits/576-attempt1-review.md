# Issue 576 attempt 1 review

Reviewed head: `700fc8d0923d18a2df79b0c203a47a558eaf2316`

Reviewer: Astra LOW

Verdict: **FAIL**

The private matching constructor, fixed typed payload, ordinary singleton/future
handling, healthy cancellation-before-injection and exact-path isolation are usable
foundations. Focused debug/release tests, the complete host-core control-provider
suite, strict Clippy/rustdoc, formatting, whitespace, workspace/host policy and Wasm
scalar/simd128 compilation passed.

The endpoint releases generic ticket credit in `collect` before reading and validating
the matching outcome metadata. Missing or mismatched metadata can therefore return an
error after irreversible reuse. Cancellation polling reports generic completion
without endpoint outcome reconciliation. Attempt 2 must stage and validate metadata
before generic release and preserve all state on every mismatch.

Preparation computes the new largest allocation without enforcing
`maximum_named_allocation_bytes`, and allocates delivery/outcome storage before its
aggregate cap decision. The report labels a partial builtin/delivery sum as combined
host retention and describes only a preparatory render subobject as render inline
storage. Attempt 2 must preflight checked projections before allocation, enforce exact
and one-below caps, and report separated quantities without double counting.

The three sequential zero-input tests do not satisfy the frozen discriminators.
Attempt 2 must add actual threaded singleton/post-claim proof, late two-ticket FIFO,
invalid-final-record and saturation atomicity, bank/scalar PCM/state/observation and
pairing-decline proof, the private post-graph/pre-terminal fault seam, repeated
allocation/free and resource gates, FP/Send/Sync coverage, and all five named mutation
results. Existing general host tests do not replace these endpoint-specific gates.

The correction remains within the existing #576 ownership boundary. No protocol,
graph, engine, builtin, manifest, artifact or #575 path is authorized.
