# Issue 475 Sol attempt 2 evidence

Reviewed source is `60004323b0b8fb71b80864927c2c9e186dc6be49`. The production compressor
files are byte-identical between object-build source `81ed8e72` and reviewed source `60004323`;
the intervening change only isolates the integration test's process-global allocation counter.
`kernel.rs` SHA-256 is
`0659a83efa4ae4ea71ef6dcc752ddf0812a742b648c872be175a3ab49fc3b129`.

The access mutant log is contemporaneous: its diff was captured while the one-line mutant was
installed. All independent old-access word comparisons completed before the branch-local uniform
fill witness failed with Cargo status 101. The mutant was then restored and the coherent runtime
log records the unchanged test passing.

The allocator's first full-suite run is retained because it found genuine process-global counter
contamination from the concurrently running conformance test. The corrected test invokes the same
named test in an exact-filter child process. Its positive allocation and deallocation probes occur
outside the measured interval; 32 repeated production renders of uniform staged, uniform D=0
per-frame, and ragged bank access report zero allocations, deallocations, and reallocations. The
corrected locked debug/release suites and strict Clippy pass. `--all-targets` started the timing
example's test harness, which contained zero tests; no timing or benchmark body ran and that zero
test is not evidence.

The direct `compressor --emit=obj` files are LLVM bitcode; their failed bad-magic decode is retained
and receives no object-inspection credit. The successful evidence is the linked production
`wasm-gate-guest` module, which contains the real compressor corpus and factory path. Both scalar
`-simd128` and `+simd128` modules compile with `--locked` and decode with `wasm-objdump 1.0.34`.
The decode log records module hashes, byte and function populations, zero scalar v128 matches,
26,376 simd128 opcode lines, and named compressor `process_block`/`fill_taps` instantiations.
`fill_taps<Simd8>` in the simd128 object contains two `memory.copy` instructions followed by the
conditional loop structure, identifying the uniform two-copy arm and retained ragged path. This is
shape evidence, not an instruction-count or speed claim. `frames_loop` and `idle_frames_staged`
may be inline-only and are not claimed as separately emitted bodies.

Width coverage is bounded precisely. Local public scalar tests exercise W1; this x86-64-v3 host's
public bank tests exercise its native W8. The private old-access oracle instantiates W1/W4/W8 in
one test binary. The freshly decoded production Wasm modules contain named W4 and W8 compressor
instantiations, while the already accepted unchanged G5 records execute the scalar-Wasm and
simd128 backend corpus. No claim is made that every new public transition test executed at every
width.

The retained failed logs are evidence of the actual correction path and are not passing gates.
The immutable delivery qualification, full-workspace qualification, timing, PR and required CI
were outside this attempt.

