# Sol implementation brief — issue 034 launch-critical builtin contract closure

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1.** Issue 007 stopped after three attempts. Preserve all of its
evidence. This new workflow has exactly two available implementation attempts: Terra attempt 1
and, only if bounded, one Sol correction attempt. A second failure stops; do not rebrief in place
or weaken a gate.

The retained conditioned incremental all-`f32` TPT recurrence, response thresholds, scalar
builtin chain, matrix/pan math and meter calculations are not reopened. Never inspect V1/legacy.
Issue 034 owns parameter metadata, sealed-only graph integration, exact checked resource
accounting and compiler mutations only. Fixtures, million-render audits, target qualification and
the timed workload belong to issue 035. Timed benchmark invocation count is **0**; issue 034 must
not invoke or authorize `scripts/run-builtins-benchmark.sh` or the benchmark binary.

## Frozen metadata contract

Retain numeric IDs and names exactly:

| ID | Name | Scope | Mapping/domain | Default | Update/reset |
| --- | --- | --- | --- | --- | --- |
| 1 | `polarity_invert` | `PerLane` | Boolean false/true | false | prepared-only/restore |
| 2 | `trim_db` | `PerLane` | decibel amplitude, `[-144,24]` | 0 dB | prepared-only/restore |
| 3 | `hpf_hz` | `PerLane` | 0 disabled, else `[10,Fs/2)` | 0 Hz | prepared-only/restore |
| 4 | `lpf_hz` | `PerLane` | 0 disabled, else `[10,Fs/2)` | 0 Hz | prepared-only/restore |
| 5 | `fader_db` | `PerLane` | decibel amplitude, `[-144,24]` | 0 dB | prepared-only/restore |
| 6 | `mute` | `PerLane` | Boolean false/true | false | prepared-only/restore |
| 7 | `matrix_ll` | `MatrixShared` | finite linear `[-1,1]` | 1 | block-target/keep target |
| 8 | `matrix_lr` | `MatrixShared` | finite linear `[-1,1]` | 0 | block-target/keep target |
| 9 | `matrix_rl` | `MatrixShared` | finite linear `[-1,1]` | 0 | block-target/keep target |
| 10 | `matrix_rr` | `MatrixShared` | finite linear `[-1,1]` | 1 | block-target/keep target |

Add stable public enums for scope and mapping. Encode the filter condition as a domain variant
whose validation takes the prepared rate; do not emulate it with `maximum=INFINITY`. Matrix
descriptors state exact linear-N-update smoothing. Decibel mapping explicitly means amplitude
`10^(dB/20)`. Exhaustively compare the descriptor table and validation at 44,100/48,000/88,200/
96,000 Hz, including zero, 10, just below Nyquist, exact Nyquist, infinities and NaN. Do not
renumber stable IDs or change defaults.

## Frozen sealed-integration architecture

Keep `PreparedBuiltinsSession` opaque, non-`Clone` and consuming. Its private seal retains:

1. SHA-256 of canonical session TOML, sample rate and quantum;
2. sorted exact track IDs;
3. exactly three `(track ID, stage)` processor identities per track;
4. exact recomputable `(track ID, BuiltinTail)` values;
5. sorted meter tuples `(handle, track, tap, reset generation, period, hold, decay bits,
   logical queue capacity)`;
6. exact observer `(node, handle)` and consumer `(handle, track, tap)` identities; and
7. the resource report described below.

Remove `PreparedGraphPlan::attach_internal_bindings` as a public capability. Use the existing
dependency direction without a fake friend/token: `PreparedGraphBuiltinsArtifact` privately owns
the unbound graph plus the consumed sealed builtin processor/observer parts and meter consumers.
Its consuming bind method accepts only the genuine external bindings, verifies their exact node
set/envelope and disjointness, privately appends the compiler-owned builtin parts, then delegates
to graph binding. A normal issue-006 graph may still accept ordinary external bindings and normal
observation APIs, but those values cannot construct, convert to, extract from or be accepted as a
`PreparedGraphBuiltinsArtifact`. Do not use an unsafe convention, public forgeable token, Cargo
feature, doc-hidden constructor or runtime secret as a substitute for type/field privacy.

Graph compilation recomputes the expected seal from the same effect-prepared session before
consumption. On failure it returns both complete prepared inputs and one sorted diagnostic set.
Test an external crate with compile-fail cases for artifact construction, private-field mutation,
parts extraction, clone/back-conversion and the removed generic attachment method. A runtime test
also proves arbitrary external bindings cannot overlap the internally owned builtin nodes.

The test-only corruption feature is absent from production builds and independently corrupts:
`SessionIdentity`, `Tracks`, `Processors`, `Tails`, `Requests`, `Observers`, `Consumers` and
`Resources`. Each case produces its exact diagnostic before input consumption. Include duplicate,
missing, extra and mismatch subcases so set equality is not inferred from counts.

## Frozen resource formulas

Rename/document the report as engine-owned retained payload, not RSS. Phase 1 performs parameter,
count, `Layout` and cap validation without issue-owned processor, meter queue or artifact payload
allocation. Phase 2 allocates only accepted reported layouts and remains transactional.

Count one allocation and the exact requested `Layout` for every concrete input/fader/matrix
processor box; actual-capacity processor/observer/consumer/tail/seal vectors; each stable
ID/`Box<str>` payload; meter observer box; producer and consumer endpoint; SPSC logical header;
and its `logical_capacity + 1` slots. A checked SPSC resource helper in `miso-engine-core` is shared
by preflight and queue construction. Include engine layout padding. Exclude allocator headers,
pages, unrelated session/effect artifacts, the transient unsplit chain and duplicate trait-object
pointer accounting.

Every add, multiply, capacity and `usize/u64/isize` conversion is checked. Do not use `as`,
saturating operations or fallback maxima on the accounting path. Overflow returns the exact code
`builtin.resource.arithmetic_overflow`. Cap failures allocate zero issue payloads.

Extend the test allocator snapshot from total/largest to an ordered multiset of `(size,align)`
layouts and counts. For each tracks `{1,4,65537}` x meter sets `{0,1,7}` with logical capacity
four, compare the full multiset, allocation count, summed retained bytes and largest request to
the report. Exercise equal cap and one byte below each independently applicable total/largest
boundary. Retain 65,537-track zero-meter success plus configured-resource rejection.

## Frozen 10,000-case compiler mutation

Use one checked-in deterministic seed and exactly 10,000 cases. Generate complete preparation
requests, not direct scalar samples. Cover valid/invalid lane booleans, gain bounds/nonfinite,
zero/enabled/Nyquist filter values and order; all matrix coefficients/targets and smoothing
`0,1,2,127,128,u32::MAX`; every meter tap plus duplicate/unknown tracks/handles, period, hold,
decay, reset generation and queue boundaries; quanta `1,127,128,255,1024`, empty/mismatched block
shapes, discontinuity/time overflow; and exact/equal/one-below/overflow resource caps.

Hash the generated case descriptions so accidental coverage drift is visible. Every case yields a
complete artifact or exact sorted typed diagnostics and both inputs; no partial success, panic,
timeout or accepted-report over-allocation is allowed. Assert that every required equivalence
class occurred at least once rather than relying on the PRNG alone.

## Ordered implementation and verification

1. Land the descriptor/domain API and exhaustive tests without changing DSP bits.
2. Refactor the builtin graph artifact to the private wrapper-owned bind path; add external
   compile-fail and eight-category corruption tests.
3. Replace accounting casts/formulas, add shared SPSC layout helper and two-phase allocation
   multiset tracker/grid.
4. Add the exact compiler mutation generator and coverage assertions.
5. Run focused debug and pinned scalar release tests; 65,537-track tests; native, Android, iOS and
   Wasm `-simd128/+simd128` compile checks; locked workspace tests; warning-denied all-target
   Clippy/rustdoc; formatting; and workspace/realtime/graph/builtin policies/mutations.
6. Run only a zero-launch check and record `timed_benchmark_invocations=0`.

If a change requires altering the retained DSP, fixture/benchmark ownership, issue-006 topology or
issue-008 SIMD semantics, stop and report scope failure. Completion means only issue-034 contract
closure. It authorizes issue 008 and issue 035 to proceed; it is not machine qualification,
listening evidence or launch approval.
