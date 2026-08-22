# Sol implementation brief — issue 069 builtin prepared-chain and graph realtime audit proof closure

## Decision and attempt budget

**READY.** Consume the stopped Issue-057 checkpoint `376774f` only as technical input and the
Issue-064 corpus as immutable accepted input. Permit one Terra attempt and one bounded Sol
correction/review; a second failure stops. This issue executes non-timed correctness/realtime
audits only. Benchmark, preflight, workload, timing, target and instruction invocations remain
zero.

## Smallest legal seam

Add no Cargo feature or exported test-support API. Inside the existing `miso-engine-builtins`
`#[cfg(test)]` module only, define a V1 test snapshot with exactly:

1. eight filter-state `u32` words: L-HPF s1/s2, L-LPF s1/s2, R-HPF s1/s2, R-LPF s1/s2;
2. four current-matrix and four target-matrix `u32` words in ll/lr/rl/rr order;
3. one `remaining_updates: u32`; and
4. two lifetime recovery `u64` counters, left then right.

Define one test helper selecting exactly one of the four lane/filter sections and setting only its
s1/s2 bits. Tests use the ordinary private chain and public production methods and compare the
snapshot and unchanged `BuiltinProcessReport`; do not duplicate DSP, bypass validation, expose
arbitrary private mutation, add restore/migration or change normal layout. A non-test symbol/static
scan proves the helper is absent from every production/audit binary.

Put the seven true tap/two-meter-set proof in a graph-compiler integration test modeled on the
accepted Issue-067 graph. Preserve `builtin.meter.duplicate`: prepare two independent graph
instances, each with exactly one genuine `MeterRequest` for each of the seven ordered taps. Drain
the success instance normally; allow the capacity-one saturation instance to reach the exact
full/drop outcome. Before that intentional queue divergence, require identical tap IDs/order and
first-window tuple bytes. In both instances the accepted seven tuples remain distinct, compiler
and runtime PDC are exactly 9 with the first early-route contribution at frame 9, and an identical
post-drain/full continuation digest proves processing did not diverge. Do not invent rack boundaries
in the scalar chain or change duplicate-meter preparation semantics.

## Separate deterministic evidence

Create only `tools/miso-engine-builtins-audit/fixtures/v1/{direct-schedule.pcm.f32le,
prepared-chain-state-report.jsonl,graph-meter-sets.jsonl,direct-result.json,MANIFEST.tsv}`. The author uses an
independent retained-f32/reference recurrence and independent meter/state/report projection, writes
only a unique scratch directory, and refuses overwrite. The checker is read-only and rejects a
payload-byte mutation, stale manifest row, noncanonical token/order and a manifest-valid semantic
mutation in each represented format. Review and commit the accepted bytes once; record all five
hashes before audit execution. Never call the author from a checker, audit, test of accepted bytes,
benchmark or policy script. Assert the Issue-064 manifest/graph PCM/graph meter hashes and all
accepted benchmark-input bytes are unchanged.

The internal prepared-chain expected schedule is exactly the Issue-069 body: 48 kHz/q128; filters
L=100/1000 Hz, R=200/2000 Hz; 257-update identity-to-swap target on call 1; retarget to
`[0.9,0.1;-0.1,0.9]` before call 2; atomic rejected nonfinite target and two nonfinite input
samples on call 3; injected L-HPF/R-LPF recovery on call 4; discontinuity/full reset before calls
5/6; finite `(0.25,-0.5)` input otherwise. The external direct audit mirrors only public target,
input and reset operations, uses ordinary finite call 4 with zero recovery, and then continues to
one million calls. Freeze exact public early PCM, internal post-event state/report rows, the
deterministic million-call result and 14 graph-meter rows. Do not derive expected values from the
candidate under test.

## Direct record

Execute exactly 1,000,000 ordinary public `BuiltinChain::process_dual_mono` calls. Arm six
individual early intervals so target and reset operations remain outside markers, then one interval
for calls 7..=1,000,000. The internal builtins test, not the external binary, owns retained-state
injection and exact private-state comparison. The external record compares the frozen public PCM,
process-report and deterministic result rows for its public-only mirror schedule and proves stable
storage and all nine counters plus checked total zero. Serialize address stability as booleans, not
raw addresses.

## Graph and ownership record

Keep exactly three independently prepared plans. Use accepted handles 1..7 and reset generation 0
on Plan A so its sample-0 output and first snapshots compare byte-for-byte with the immutable graph
PCM/meter fixtures; explicitly prove the seven snapshot tuples are distinct and the early route is
delayed by nine samples at runtime as well as in compiler metadata.

Render A once, publish/apply B once, publish C, then require
`DeferredRetirementFull` for every remaining call. Check exact values:

- renders A/B/C = 1/999999/0;
- applied/deferred/prior-B-on-defer = 1/999998/999998;
- active epoch remains B and `owner.deferred_count()` is 999998; and
- C never applies, renders or enters the retirement queue.

Use four marker pairs: A, applying B, first deferred B and remaining deferred B. After all markers,
the retirement thread reclaims/destroys A. Then disarm and destroy the owner on the declared
control thread, destroying active B and pending C. Assert exactly one `(plan, role)` row for each,
zero render role and no early drop. Canonical JSON records role names/booleans/counts; raw thread
IDs remain auxiliary evidence.

## Trace and detector seal

Retain exactly nine detector variants and serialize every field. Both binaries need exactly nine
terminating probes. Replace the two stale trace parsers with one shared timestamp-aware validator:
pair markers on the render TID, project each interval over every `strace -ff -ttt` TID file, allow
only the boundary marker writes, and reject malformed/nested/unclosed markers or any overlapping
syscall. Hermetic trace fixtures must pass clean input and reject otherwise-identical render-thread
and auxiliary-thread injections. Hash the validator, canonical validator output and raw trace-file
set. The direct script takes no unsupported `--blocks`; both scripts require the exact million
count and complete deterministic JSON schema.

## Ordered implementation and gates

1. Land the internal test seam and focused layout/reset/recovery tests; prove non-test absence.
   Land the two-instance graph-compiler success/saturation seven-tap proof separately from the
   scalar chain and retain duplicate-request rejection.
2. Land the independent scratch author, read-only checker and four accepted payloads; freeze hashes
   and unchanged Issue-064/benchmark-input identities.
3. Correct direct functional evidence and both probe suites. Stop on the first fixture mismatch.
4. Correct graph fixture/lifecycle/destruction evidence. Stop unless all 999,998 deferrals and
   exact roles are observed.
5. Complete synthetic trace mutations, then run each real all-thread trace once through a yielded
   PTY. A trace or million-call retry is not authorized inside the same attempt.
6. Run focused audit/builtin/core/graph tests; format; warning-denied audit/builtin/core/graph
   Clippy; applicable realtime/graph policy and mutation scripts; shell syntax; diff/static
   no-artifact/no-workload checks. Re-seal candidate and all fixture hashes.

Stop on expected-byte drift, a default-enabled seam, production/benchmark dependency, recovery or
tap ambiguity, any nonzero detector/trace event, incorrect lifecycle/destruction, author reachability
from read-only paths, corpus/benchmark-input change or second failed attempt. Do not move target or
instruction work here.

## PASS record

Record the exact candidate; attempt count; feature/dependency proof; separate fixture hashes;
unchanged Issue-064 hashes; direct and graph canonical record hashes; exact lifecycle/owner rows;
nine probes per binary; real/synthetic trace hashes; focused/policy outcomes;
`workload_invocations=0`; `timed_benchmark_invocations=0`; and strict Terra/Sol verdict. PASS alone
unblocks **Builtin native, AArch64, and Wasm runtime-selection and instruction qualification**.
