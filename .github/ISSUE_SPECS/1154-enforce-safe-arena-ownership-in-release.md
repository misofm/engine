# Use a safe single-owner arena for sequential rendering

## Owner decision and source finding

On 2026-10-02 the owner ruled that there is no identified need for multi-lease arena support, after root explained #1154's safe-versus-unsafe API choice. Root takes this as authorization to remove multi-lease functionality and keep a safe exclusive arena matching the production single-render-thread model. No multicore executor or deliberately unsafe public multi-lease API is authorized.

The original #1115 finding remains source evidence: checked_write and write_read/write_read2/write_read_stereo use debug-only ownership/alias checks; generic plane offsets are not completely validated before raw slice formation; public Arc-backed leases can allow a foreign writer/read overlap that builder wave numbers do not synchronize. Production currently creates one sequential lease and follows the intended ordering. No current production fault, runtime UB reproducer or completed Miri proof is claimed by that discovery.

## Smallest authorized product slice

Remove the general multi-lease builder/access model and shared-arena ownership. Construct exactly one non-cloneable owner of the preallocated planar storage, with no public route to foreign mutable/shared leases. Preserve zero-copy producer/consumer buffer access, reserved silence buffer 0, existing buffer IDs/plane layout, SIMD lane order, graph execution order and every valid render word. Migrate the actual graph constructor and its test helpers; update the engine reexports and current realtime-policy descriptions. Retire tests/error branches that defend multi-lease/wave behavior made unexpressible by this correction; keep their mutation records explicitly historical rather than rewriting old evidence.

Audit every remaining safe access method. Invalid plane/buffer/shape, silence writes and output/read or output/output overlap must be rejected before arithmetic can overflow or references can be formed, in both debug and release. Existing Option multi-borrow refusals remain transactional; convenience methods may retain programmer-error panic behavior with actual release guards. No ordinary test executes undefined behavior on the old implementation. Exclusive Rust borrowing must provide access timing; documentation alone is insufficient.

Preparation owns allocation and validation. Render stays allocation/free/lock/I/O/syscall-free, bounded and zero-copy; no new per-sample validation loop, broad capability framework or extra retained tables. Preserve retained-byte accounting accuracy and caps; deleting ownership bookkeeping may change Rust structural size, but never PCM capacity or a sealed resource-format fixture silently. This issue does not absorb #1074 or add a generic memory walker.

## Brief and execution

The user-requested two GPT-6.1 Sol xhigh agents continue: A supplies the implementation proposal and then owns the single approved product tranche; B independently audits safety, resource consumers and test value, then reviews the frozen implementation. Root Sol owns the final concrete brief approval, exact-path checkpoint commits/pushes, one adversarial verdict per coherent attempt (maximum five), GitHub synchronization and required qualification/main delivery. No implementation starts before root records and synchronizes the exact proposed API/files. Read-only proposal work is authorized now.

Scope is engine's arena implementation/reexports, graph's constructor/test helper migration and directly superseded current documentation. Any additional host, ABI, compiler-resource subsystem or benchmark framework requires a bounded successor rather than expanding this correction. Stop at the first compiling/focused-green product tranche for root checkpoint before further edits or evidence.

## Objective gates and test value

- Existing arena zero-copy address/word, silence, alias/shape/refusal and four/eight-lane scatter owners pass after migration; same-API invalid access regressions run only on the corrected code unless Miri can safely diagnose the old implementation.
- Focused engine and graph debug/release tests, strict package clippy, formatting and existing realtime/workspace/graph policies. Representative downstream graph-compiler/resource/fixture and actual installed allocator/render gates qualify the changed ownership boundary without new harness/corpus expansion. Freeze the final concrete subset before running it.
- Compile-only wasm simd128, AArch64 iOS and Android product checks. Use available Miri for a bounded relevant safe-access subset if supported; report unavailable/toolchain limits accurately and record wider qualification separately rather than blocking a proven usable product slice.
- Every new/rewritten test's verdict names the plausible unique defect; remove superseded multi-lease tests in the same implementation. No prose/source grep tests, permanent old-output digest or private layout/byte pin. One-time old/new valid-output evidence stays in PR evidence, not a committed oracle.
- No descriptive timing loop or projected speedup. Changes to generated code may be reported only for artifacts actually inspected. Required qualification succeeds before merge; synchronize and close #1154 only once root PASS and its evidence are upstream.

## Evidence and delivery state

Original finding confirmed by worker A and root under #1115; owner direction now settled. Current production caller is crates/graph/src/runtime.rs; a repository-wide source search finds no other non-test arena consumer. Root started codex/safe-single-owner-arena-1154 from synchronized main 7345ecb9. Exact API proposal and implementation approval are pending the two independent reads. No implementation or validation improvement is claimed yet.
