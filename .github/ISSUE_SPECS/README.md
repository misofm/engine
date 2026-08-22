# Engine V2 issue specifications

These Markdown files are source-of-truth bodies for later GitHub issue creation.  They are not GitHub templates and do not create issues by themselves.  Each body is intentionally stateless: it contains its mission, applicable invariants, interface contract, dependencies, deliverables, non-goals, hazards, gates, target matrix, and evidence requirements.  “Declared tolerance,” “configured budget,” or similar language is valid only when the issue requires the value and its research/measurement rationale to be frozen in the Sol-approved brief before production code starts.

## Use

1. Create the GitHub issue title from the body H1 after removing its three-digit ordering prefix, then copy the complete body without replacing substantive sections.
2. Sol approves the brief and objective gates before implementation.
3. Terra implements attempt 1 and appends evidence to the issue.
4. Sol conducts adversarial review and may make two further implementation/revision attempts unless
   the issue's authoritative brief freezes a smaller attempt budget.
5. After the frozen attempt budget fails, stop and create a stateless rescope/rebrief issue; do not
   lower acceptance gates or silently begin another attempt.

Files/H1s are ordered by numeric prefix and use lowercase kebab-case filenames.  The prefix is planning metadata, not part of the published GitHub title; dependency entries therefore name the exact published title and remain portable outside this repository.

Issue numbers preserve creation order, not dependency order. The issue-011 rescope moved external
descriptor/package/state bytes into issue 029 without renumbering existing specs: launch work uses
**Native effect runtime contract and conformance**, while issue 027 and future repository work use
**Canonical effect interchange, state migration, and CID package identity**. Therefore the
extensibility sequence is 029 -> 027 -> 028.

The issue-006 three-attempt workflow remains recorded as failed at runner artifact promotion. A
fresh Sol rescope accepted its launch-critical graph compiler/runtime/PDC outcome from complete
functional evidence and the already measured validator-valid raw descriptive benchmark. Issue 030
owns only benchmark-wrapper operational hardening and promotion of those exact bytes. It is not a
dependency of issues 007–010 and does not authorize another issue-006 benchmark run.

Issue 032 corrects the repository-wide launch sample-rate policy after issue 007 accepted its
launch filter recurrence: 44,100/48,000/88,200/96,000 Hz are the exact launch session/render set;
176,400/192,000/352,800/384,000 Hz remain extended compatibility evidence only. Dependency order
places the accepted 007 slice -> 032 -> 034 corrections -> 036 -> 008, while the preserved
Issue-010/040 streaming checkpoints and Issue-043 product closure follow 032; downstream effects, hosts
and release qualification follow their exact listed dependencies.
Historical “required/all-eight” evidence is preserved as period evidence and does not override
issue 032.

Issue 007 stopped after three failed attempts. Its post-stop Sol rescope accepts only the proven
conditioned scalar TPT/builtin runtime slice as reusable technical foundation; it does not claim
machine qualification and preserves every failure. Issue 034, **Issue-007 launch-critical builtin
contract closure**, stopped after two failed attempts; its landed metadata, sealed-only graph,
checked resource-accounting and composite-matrix corrections remain bounded input, not PASS.
Issue 036, **Representable TPT cutoff domain and builtin contract acceptance**, owns the sole
remaining numerical boundary and final nonbenchmark acceptance. Issue 035, **Issue-007 builtin
qualification tooling, audits, and benchmark**, stopped after two attempts at accepted typed
fixture-contract checkpoint `0edc51c`; it has no overall PASS and ran no benchmark. Issue 056,
**Complete independent builtin corpus and corruption proof**, also stopped after its fixed response
candidate exposed unresolved repeated cascade recovery. Issue 059, **Builtin cascade decay and
recovery contract**, accepted the corrected recovery rule. Issue 060, **Complete independent
builtin corpus after recovery acceptance**, then stopped after its partial `10f0235` checker
checkpoint and interrupted typed-JSONL attempt without sealing the corpus. Issue 061, **Complete
builtin response cases and scalar PCM semantics**, and Issue 063, **Complete builtin meter,
diagnostic, and resource corpus semantics**, close two bounded surfaces. Issue 062, **Complete
builtin graph-tap and PDC fixture semantics**, stopped after correcting its graph model at
checkpoint `2bbed6a` because accepted benchmark-input TOMLs pin the displaced graph PCM hash.
Issue 067, **Reconcile builtin graph fixture and dependent benchmark input identities**, owns that
exact graph-payload/input-identity transaction. Issue 064, **Seal independent builtin corpus
corruption and read-only qualification**, joins 061/063/067 and owns the final 24/24 seal.
Issue 057, **Builtin direct and graph realtime audit and target qualification**, follows 064. Issue
058, **Builtin benchmark preflight and exactly-once qualification**, follows 057 and alone owns the
eventual one-invocation/two-round builtin benchmark. Issues 057–059, 061, 063–064 and 067 each permit
one Terra attempt plus one Sol correction. The successor timed invocation count is currently zero.

Dependency order is 007 accepted slice -> 034 corrections -> 036 -> both 008 and stopped 035,
then 035 checkpoint -> stopped 056 -> 059 -> stopped 060 -> parallel 061/063 plus stopped 062 ->
067 -> 064 -> 057 -> 058. Issue 008 needs 036's accepted preparation/metadata and
sealed graph/resource contract but not the qualification successors. Issues 022, 023 and 024 wait
for 057's audited/target-qualified machine candidate. Issue 033 runs the real preregistered human
listening only after 058 seals that candidate and its accepted benchmark; issue 026 waits for both
058 and 033. This ordering forbids synthetic trials, audible-quality
claims and release bypasses while allowing independent SIMD work after its true contract
dependency is complete.

Issue 008 stopped after its two-attempt budget. Checkpoint `87783c5` preserves safe explicit
scalar/Wasm/NEON/AVX2/FMA kernels, the generic AoSoA/effect-bank substrate and direct builtin-bank
conformance as technical input, but Issue 008 is **not PASS**. Issue 037, **Production SIMD builtin
bank graph retention and reachability qualification**, owns the missing production post-input-
builtin graph retention, exact 100 seeded layouts and corrected real-TPT 100,000-render audit;
timing is forbidden there. Issue 038, **Issue-008 real audio benchmark workloads and exactly-once
qualification**, follows 037, replaces the placeholder byte-fold workload and alone may later
authorize one warmup plus two measured rounds. Issue 038 completed its one authorized invocation
without retry; its evidence remains descriptive rather than an optimization claim.

Issues 009, 022, 024, 031 and 026 wait for 037 because their completed product contracts require
the retained production builtin-SIMD graph. Issue 038 is a dependency only of release
qualification issue 026; it does not block scheduler issue 009, streaming issue 010, effects or
deployment feature work. Issue 042 and Issues 013, 014, 021 and 053 continue to consume Issue 008's
preserved generic architecture/effect-bank slice without treating its stopped issue as PASS.

Issue 009 stopped after its two-attempt budget without overall PASS. Upstream checkpoint `3236b9c`
preserves the real Linux x86-64 parallel graph scheduler, transactional fallbacks, audit/target
evidence and a zero-launch benchmark runner as technical input. Issue 039, **Native graph scheduler
qualification and benchmark**, owns only the missing q128 full production differential, exact 32
completion perturbations, exact 100 preparations/count matrix, injected ownership failures,
all-thread syscall trace, macOS/rustdoc/clean-candidate proof and sole one-warmup/two-round scheduler
benchmark. Issue 039 permits one Terra attempt plus one bounded Sol correction; its timed invocation
count starts at zero. It gates only consumers that claim qualified native parallel graph execution
and release issue 026. Sequential streaming, effects, control work and browser/mobile adapters
remain nonblocked.

Issue 010 stopped after strict Sol review without overall PASS. Checkpoint `5dbe1cb` preserves its
native parser/decoder, move-owned planar ring, host chunk boundary, coordinator source fan-out and
target/Wasm evidence only as technical input. Issue 040, **Issue-010 launch-critical source
ownership and accounting closure**, then stopped after two attempts: its plan-owned worker lifetime,
shape, representative correctness and non-`Arc` accounting checkpoint remain accepted technical
input, but its opaque telemetry `Arc` prevents exact retained accounting and therefore overall PASS.
Issue 043, **Exact lock-free native source sanitation telemetry handoff**, owns the remaining safe
move/SPSC/block-stamped correction and gates Issues 022–024. Issue 041, **Issue-010 source streaming
qualification tooling and adversarial evidence**, follows 043 and owns the expanded diagnostic
corpus, frozen seek races, real worker-delay audit and duration-independent allocation-layout/RSS
proof. Issue 041 is nonblocking for hosts/features and gates only release qualification Issue 026.
Issues 040, 041 and 043 forbid benchmarks; the timed invocation count remains zero.

Issue 012 stopped without overall PASS when its first independent-oracle gate disproved the frozen
five-`f32`-coefficient direct-form-I numerical contract. Issue 042 then selected an endpoint-
conditioned transfer and proved the complete analytic/search, scalar/bank/graph/audit and target
surfaces, but stopped after its first legal one-second impulse triggered runtime recovery in the
frozen direct-history delta recurrence. Those checkpoints remain technical input only. Issue 044,
**Conditioned time-domain launch parametric EQ recurrence**, then stopped after its corrected full
comparison found no passing direct/scaled/transposed/flush recurrence. Issue 045, **Launch
parametric EQ recurrence derivation and runtime proof**, is research-only and requires f64 mapping
equivalence before retained-f32 testing. Issues 015, 017 and 026 depend on Issue 045.

Issue 013, **Launch feed-forward peak compressor**, closes the bounded launch product at its
descriptor/scalar/W4/W8/registry/graph boundary. Issue 046, **Launch compressor qualification,
realtime audit, and benchmark**, owns only the deferred checked corpus/oracle matrices, exact
10,000 and million-sample rows, expanded cohort/determinism evidence, 100,000-render audit,
target/instruction proof, zero-launch preflight, sole eventual one-warmup/two-round descriptive
benchmark and audition/listening handoff. It permits one Terra attempt plus one Sol correction;
its timed invocation count starts at zero. Issue 046 gates only release qualification Issue 026
and does not block other effect implementations.

Issue 014, **Launch hysteretic peak gate/expander**, stopped after its two attempts with the
descriptor/scalar/W4/W8/registry/graph product and bounded correction preserved as technical input,
but without effect-local reset, uninterrupted restore, signed-zero identity and injected recovery
proofs. Issue 048, **Launch gate reset, restore, and recovery proof**, owns only that launch-product
closure and any directly exposed bounded repair. Issue 047, **Launch gate/expander qualification,
realtime audit, and benchmark**, follows 048 and owns expanded corpus/sequences, cohort/determinism,
100,000-render audit, targets/instructions, the sole eventual descriptive benchmark and audition/
listening handoff. Issues 048 and 047 each permit one Terra attempt plus one Sol correction; their
timed invocation counts start at zero. Issue 047 gates only Issue 026 and does not block unrelated
effect implementations.

Issue 016, **Launch fixed-4x true-peak safety limiter**, stopped after its two attempts without
overall PASS. Its corrected fixed-four-times BS.1770 Annex-2 scalar detector, guarded gain/hold law,
fixed latency and state checkpoint remain accepted technical input. Issue 050, **Launch true-peak
limiter bank and graph closure**, owns only the missing W4/W8 gain-apply banks, scalar tails,
registry/effect-compiler and representative graph/PDC product closure. Issue 049, **Launch true-peak
limiter qualification, realtime audit, and benchmark**, follows 050 and owns expanded standard/
corpus matrices, long sequences, the 100,000-render audit, targets/instructions, the sole eventual
one-warmup/two-round descriptive benchmark and audition/listening handoff. Issues 050 and 049 each
allow one Terra attempt plus one Sol correction. Their timed invocation counts start at zero;
Issue 049 gates only release qualification Issue 026.

Issue 018, **Launch two-band LR4 multiband compressor**, owns only the fixed Normal-quality
two-band LR4 product: one accepted conditioned-TPT crossover, two Issue-013-style compressors,
scalar/W4/W8 execution and representative registry/graph/PDC closure. Issue 051, **Launch
multiband compressor qualification, realtime audit, and benchmark**, follows 018 and owns expanded
corpus/matrices, nonlaunch three-to-eight-band/topology research, long rows, the 100,000-render
audit, targets/instructions, the sole eventual one-warmup/two-round descriptive benchmark and
audition/listening handoff. Both permit one Terra attempt plus one Sol correction. Their timed
invocation counts start at zero; Issue 051 gates only release qualification Issue 026.

Issue 019, **Launch fixed-2x cubic soft-clip saturator**, stopped after its Terra and Sol attempts
without overall PASS; accepted scalar checkpoint `e674d5e` remains technical input. Issue 053,
**Launch soft-clip bank and graph closure**, owns only the missing W4/W8 banks, scalar tails, single
representative alias claim, registry/effect-compiler and ten-track graph/PDC/cap product closure.
Issue 052, **Launch saturator/clipper qualification, realtime audit, and benchmark**, follows 053
and owns expanded mode/quality research, corpus/long rows, realtime audit, complete targets/
instructions, the sole eventual one-warmup/two-round descriptive benchmark and listening handoff.
Issues 053 and 052 each permit one Terra attempt plus one Sol correction and start with zero timed
invocations; Issue 052 gates only release qualification Issue 026.

Issue 020, **Launch dual-envelope transient shaper**, owns one fixed Normal-quality causal product:
fast/slow peak-envelope contrast, attack/sustain/mix controls, explicit detector links, zero
latency/tail, scalar/W4/W8 execution and representative registry/graph closure. Issue 054,
**Launch transient-shaper qualification, realtime audit, and benchmark**, follows 020 and alone owns
the expanded corpus, exact seeded/long rows, realtime audit, targets/instructions, sole eventual
one-warmup/two-round descriptive benchmark and listening handoff. Both allow one Terra attempt plus
one Sol correction and start with zero timed invocations; Issue 054 gates only release
qualification Issue 026.

Issue 021, **Launch integer-time dual-mono and ping-pong delay**, owns one fixed Normal-quality
scalar dynamic-rack product: nearest-sample 1–2000-ms taps, bounded 128-update dual-tap changes,
per-lane signed feedback/damping/mix, an explicit shared dual-mono-to-ping-pong feedback matrix,
fixed two-second prepared histories, zero latency, Infinite tail and representative registry/graph
closure. Issue 055, **Launch delay qualification, realtime audit, and benchmark**, follows 021 and
alone owns expanded corpus/long stress, realtime audit, scalar target/instruction evidence, the
sole eventual one-warmup/two-round descriptive benchmark and listening handoff. Both allow one
Terra attempt plus one Sol correction and start with zero timed invocations; Issue 055 gates only
release qualification Issue 026. Any future gathered W4/W8 delay bank is a separate stateless
product/optimization issue, not Issue 055 scope.

## Shared definition

Engine V2 is a greenfield, Rust, agent-first mixing/mastering engine.  It must not inspect/copy V1.  The render thread exclusively owns a prepared plan whose topology/capacities are immutable and whose preallocated DSP state is mutated during rendering.  The render path performs no allocation/free, lock, I/O, network, logging, syscall, structural plan mutation, or data-dependent unbounded work; displaced plans are reclaimed off-thread.  There is no compiled track limit.  Audio is planar `f32`; dual-mono channels remain independent unless an explicit contract links them.  Output is PCM.
