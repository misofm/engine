# Effect interchange V1 qualification

Issue 081 qualifies the already accepted descriptor-wire, package/CID, state-envelope, and
migration boundaries as one portable, read-only interchange product. It adds no wire, public Rust
API, C export, runtime dependency, serialization format, or render operation. The accepted product
sources, references, fixtures, C header, and the Issue 002 benchmark remain immutable.

## Immutable baseline

`fixtures/effect-interchange/v1/ACCEPTED.sha256` is a sorted, repository-relative SHA-256 manifest
authored from clean base `8d78ea3d4ad42d36831b4d0267908a7b98f16167`. It covers every file in
the descriptor, package, and state V1 fixture directories; the three independent stdlib Python
references; the descriptor C header; and the exact package/compiler sources implementing
descriptor wire/FFI, package/CID/state, migration, and restore. Qualification checks it before and
after every runner. A mismatch is a STOP and is never repaired by refreshing the baseline.

## Independent references and process lifecycle

The import-safe stdlib aggregator loads all three accepted Python references in one process,
executes their complete checks without invoking Rust or another process, verifies exact fixture
membership, and emits one canonical address-free JSON row. Its sole public process runner takes no
arguments, refuses existing regular files, symlinks, and hardlinks, and attempts exactly 100 fresh
children in ascending index order `0..99`. Raw output and every exit status survive failure. Only
100 unique successful rows with identical manifest hashes are atomically published, without
overwrite. Hermetic lifecycle tests replace `python3` with an inert fake; they are not reference
process invocations.

## Deterministic parser mutation

The ignored qualification campaign performs exactly 10,000 independent trials each for descriptor,
package, and state parsing. Its SplitMix64 seeds are `0x081d_e5c0_0000_0001`,
`0x081d_e5c0_0000_0002`, and `0x081d_e5c0_0000_0003`. Every trial rebuilds a frozen input, consumes
one generator value, flips exactly one selected bit, parses twice under `catch_unwind`, compares the
complete normalized result, checks reserved/unavailable fields, verifies all borrowed ranges and
supported re-encoding, and proves fixture/guard canaries read-only. Accepted counts are descriptive;
panic, nondeterminism, mutation outside the exact input, or re-encode disagreement fails. Existing
libFuzzer targets compile only and are never executed by this campaign.

## Migration matrix

The ignored portable matrix contains exactly 48 successes:

`{44100,48000,88200,96000} × {zero,one,two steps} × {scalar,bank source} × {scalar,bank destination}`.

All rows use quantum 128, Normal quality, unequal nonempty common/lane payload sections, complete
ordered initial values, and exact saved request caps. Historical bank ownership selects bypass;
zero/one/two steps select DualMono/Maximum/Average. Each result is compared with independently
constructed canonical current-layout bytes and a second snapshot. Source bytes, workspace suffixes,
scalar initial suffixes, and unrelated width-four bank lanes remain unchanged. Zero-step required
migration prefixes have length zero; any supplied bytes are untouched oversized suffixes. The
accepted Issue 080 failure-priority and by-value-disposal tests remain the failure suite.

## Native ABI, allocation, and memory boundaries

The only C export remains `miso_engine_effect_descriptor_v1_inspect`. C11 and Rust independently
freeze every size, alignment, field offset, count, identity, and comprehensive-A projected value for
parameter `80/4`, port `24/4`, quality `64/8`, enum choice `16/4`, summary `64/4`, and descriptor
diagnostic `16/4`. Package, state, and migration diagnostics retain Rust `repr(C)` layout tests but
are not C exports. One-short capacities are atomic and prefix/suffix canaries remain unchanged.

Isolated native allocation tests record allocation/deallocation counts and bytes, peak live bytes,
and survivors. Descriptor temporary allocation is bounded and released. Package publication is
exactly one matched descriptor pass plus zero package-native allocation. Verified artifact/CID and
prebound state operations allocate nothing. Registry/resolution allocations are control-plane,
bounded by accepted caps, and fully released. Prebuilt bank migration/restore has zero framework
allocation. Scalar success retains exactly the required owned replay-initial slice and one mock
destination `Box`; both allocations are released on drop and live bytes return to baseline.
Absolute native counts are descriptive; zero deltas and no survivors are gates where specified.
Input SHA-256, one-byte/count-below cases, output canaries, and guard pages where supported prove
read-only input and atomic caller publication. No test invokes render or production DSP.

## Target and static matrix

The five exact rows are native x86-64 Linux execution; Android `aarch64-linux-android` compile only;
iOS `aarch64-apple-ios` compile only; and Wasm `wasm32-unknown-unknown` scalar `-simd128` and SIMD
`+simd128` compile/object only. Missing installed targets or inspection tools are HOLD, never skip.
Both Wasm objects expose only the descriptor inspector under the `miso_engine_` prefix; scalar has
no SIMD opcode. Neither mobile nor Wasm row claims execution or cross-CPU byte identity.

Static checks freeze the accepted manifest and dependency direction, forbid new unsafe production
surface and migration serialization, keep qualification dependencies out of production crates, and
keep descriptor/package/state/migration references out of core/session/graph/rack/builtins render
sources. They reject stale APIs, tracked generated output, artifacts, and new C exports. Workspace,
realtime, effect-runtime, rack, graph, and builtins policies and their available mutations remain
mandatory.

## Lifecycle barrier and counters

Checkpoint 1 builds and fake-tests qualification infrastructure only. Checkpoint 2 adds the
consolidated `miso-engine-bench` package and its one validator, zero-launch
preflight, public runner, and hermetic lifecycle. The no-argument binary owns exactly four workloads:
descriptor verify/identity, package verify/CID/AVX2+FMA selection, current state verify/re-encode,
and two-step width-four bank-member migration/restore. It freezes each output digest untimed,
including the complete canonical final migration snapshot rather than only its opaque payload,
performs one all-workload warmup, then emits exactly eight closed-schema records from two measured
rounds of 256 complete observations per workload. Percentiles are nearest-rank p50/p95/p99/p99.9;
units are `ns_per_operation`; results are descriptive only.

The preflight requires a clean exact candidate and a candidate-bound nonbenchmark seal, validates a
synthetic record and the fake lifecycle, warning-denied release-builds into a temporary target,
atomically publishes the sealed binary and a no-clobber seal, and never executes that binary. The
public runner revalidates every candidate/tool/source/fixture/lock identity, invokes that exact
binary once, preserves raw output on every postlaunch failure, strictly validates eight records,
and atomically publishes accepted output and disposition without overwrite. Regular files,
symlinks, and hardlinks at output paths are rejected. Prelaunch failures may be corrected;
their first append-only prelaunch disposition remains preserved in a separate path, while the sole
final disposition is reserved for an attempted binary launch. Accepted output is an fsynced copy
with an inode distinct from the preserved mutable raw output. Postlaunch evidence is final for the
attempt.

During both implementation checkpoints the real 100-process matrix, exact 30,000-trial campaign,
48-row matrix, five-target script, benchmark preflight, benchmark runner, benchmark workload, and
timed measurement remain unexecuted, so every real counter remains zero. The sole timed invocation
requires a later clean nonbenchmark seal and explicit Sol XHigh authorization. No benchmark result
is a performance threshold.
