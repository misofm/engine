# Capture RT5 sustained input-trim render cost once

GitHub: https://github.com/misofm/engine/issues/602

Bounded tooling/capture successor to #600 under audit lane A #559, based on delivered main
`ccafe150bb8b129d85d30601cda1f6f68176127c`. #238 and #496 delivered the RT5 runtime corrections;
#600 delivered the reviewed entirely untimed W8 workload and its connected correctness evidence.
The failed #600 runner survived one correction and was removed under its circuit breaker. This issue
owns one fresh, narrow runner implementation and the sole retained descriptive capture. It does not
reopen runtime optimization or any #600 implementation attempt.

Sol HIGH coordinates the issue and owns documentation, exact-path checkpoints, the single capture
authorization, GitHub synchronization and delivery. Luna HIGH/XHIGH implements. Every scope,
source/harness, capture and exact-head verification uses Astra LOW. Lane B #598 is the only other
active issue and owns disjoint lane-policy paths. Lane B retains authority over shipped-artifact
qualification/pinning; this native tools-only issue must not change or pin the AudioWorklet artifact.

## Smallest closable outcome

Add a separate native `input-symmetry-capture` entry that reuses #600's prepared-owner workload while
preserving `bench input-symmetry` as an entirely untimed qualification command. One process prepares
two independent native W8 owners. Each owner performs 512 untimed preparation renders, then each of
two measured rounds performs exactly 4,096 timed render calls per owner. Each round therefore contains
8,192 plan renders; the capture contains 16,384 timed renders total. Publication of eight immediate
`TrimDb { lanes: BuiltinLaneSelector::Both, ... }` records, PCM generation, output hashing and record
assembly remain outside every timed interval. Only `PreparedRenderPlan::render` is inside the timing
helper.

Targets continue alternating per owner between -6 dB and -12 dB with 256-sample smoothing at 48 kHz,
q128, eight populated tracks and native W8 dispatch. Each owner's absolute-sample and alternation
state stays continuous from preparation through both rounds. Preparation traffic/output is excluded
from measured counts and digests. The timed entry must reuse the same workload primitives and frozen
PCM/digest assertions; it may not fork a second arithmetic implementation.

Report each round's summed elapsed nanoseconds, exact 8,192-render denominator and finite positive
`nanoseconds_per_plan_render`. Record the two values only as the current native x86-64-v3 prepared-plan
render cost for this frozen eight-record-per-block W8 trim ride. The measurement combines queue drain,
trim-ramp arithmetic, settle/report reads, graph execution, remaining required refresh work and the
per-render timing-observation overhead. Make no baseline, speedup, isolated helper/spill-cycle,
universal saving, percentage, release-budget, effect-floor or 64-track claim. A valid result passes
regardless of magnitude.

Successful publication and validation of the one accepted capture closes RT5's retained measurement
and accounting obligation. It does not authorize tuning.

## Exact ownership

Allowed paths are:

- `tools/bench/src/input_symmetry.rs`, only to expose shared #600 workload primitives while keeping its
  existing command and tests untimed;
- `tools/bench/src/input_symmetry_capture.rs`, new timed entry;
- `tools/bench/src/main.rs`, capture-entry registration only;
- `scripts/input-symmetry-capture-validator.py`, new strict validator;
- `scripts/preflight-input-symmetry-capture.sh`, new final preflight;
- `scripts/run-input-symmetry-capture.sh`, new one-shot runner;
- `scripts/test-input-symmetry-capture.sh`, new subprocess-free or stub-only lifecycle tests;
- this numbered spec and focused `docs/audits/` review records;
- `artifacts/issue-602-input-symmetry-capture/` for protected qualification, seal, raw and accepted
  capture records;
- concise #559/#560 handoff status under root ownership.

Reuse `bench-support` timing, metadata, digest and allocation helpers, the delivered #600 owner and
fixture, and existing repository runner conventions. The tools/bench package already has the needed
dependencies. No runtime crate, manifest, lockfile, fixture, existing benchmark other than the narrow
#600 sharing seam, existing runner/validator, consumed #431 capture, effect-floor table, policy,
workflow, browser/SDK/ABI source, shipped artifact or artifact pin may change. Do not create a generic
benchmark framework. If the outcome needs any such expansion, stop and amend this issue before work.

## Timed subject contract

The new timed entry must:

1. reject arguments and require native `Backend::Simd8`;
2. prepare two independent owners through #600's reviewed construction and execute 512 preparation
   blocks per owner without observing a clock;
3. emit one `capture_started` lifecycle marker for its sole process execution, then round-completion
   markers exactly once in order; do not call a round a separate benchmark invocation;
4. for each round, reset only measured counters/digest, then for 4,096 iterations publish to owner A,
   time only A's render, publish to owner B, and time only B's render, preserving per-owner continuity;
5. use checked arithmetic for all counts and elapsed accumulation, report 65,536 attempted and accepted
   records, 8,192 successful renders, zero render errors and 2,097,152 output words per round;
6. require both owners' output digests to equal each other and the reviewed #600 phase digest
   `75eeffae6a0116867d6d6fbe589834df53f3dce87ffd09092f22e5968bc28f87`, with nonzero PCM;
7. emit exactly two strict single-line JSON records, rounds 1 then 2, and no other stdout;
8. perform no allocation/free, lock, syscall, I/O or logging inside `PreparedRenderPlan::render`; timing
   observation surrounds the call and is acknowledged overhead rather than part of the realtime body.

The timed source receives no performance threshold and must not be run during implementation,
self-test, source review, CI, PR review or any command other than the sole root-authorized capture.
Compile and unit-test it without invoking its entry point. A test-only injected clock may validate
elapsed/count arithmetic without observing a real clock or rendering the full timed workload.

## Strict record and identity contract

The validator accepts exactly two UTF-8 JSON-lines records and rejects blank, malformed, duplicate,
missing, extra or reordered rounds; duplicate JSON keys; booleans in numeric fields; non-integral or
out-of-range counts; non-finite/nonpositive timing; unknown/missing keys; and backend values other
than exact `Simd8`.

Each record carries exact issue, schema/kind, round, fixture ID/hash, sample rate, quantum, W8 width,
track count, records per block, target pair, smoothing, preparation blocks per owner, measured blocks
per owner, owners, renders/attempted/accepted/errors/output words, both owner digests, elapsed
nanoseconds, finite positive nanoseconds per plan render, source commit/tree, binary and source hashes,
exact argv/cwd, Rust/compiler/target/effective build flags, CPU/OS and explicit metadata-missing list.
The validator recomputes all derived counts and timing division and requires the reviewed digest.

The final preflight seal has an exact schema and exact key set. Every string, integer, list and digest
has a strict type/domain; booleans never satisfy integer fields. It records the clean candidate commit
and tree, release binary, fixture, Cargo.lock, timed/untimed source, dispatcher, validator, runner and
preflight hashes; exact argv/cwd/toolchain/target/build flags; workload constants; expected two
records; one allowed workload process; 16,384 timed render calls; and `READY` status. The validator
must fully validate this seal before any workload launch. Partial field scraping is not validation.

## One-shot runner and lifecycle contract

The self-test uses temporary stubs and may not execute either real input-symmetry entry, a timing
helper, or the final preflight/capture namespace. It must exercise bad arguments; strict valid and
invalid record/seal schemas; duplicate keys; wrong backend and numeric types; missing/duplicate/extra/
reordered rounds; every identity mismatch; refusal before launch; child failure; raw stdout/stderr
preservation; validator failure; transactional accepted publication; post-workload persistence
failure with retained recovery location; exact status propagation; and overwrite refusal. A durable
sentinel proves zero real workload-process and zero timed-render invocations in all self-tests.

The final preflight runs only after Astra LOW source/harness PASS. It refuses a dirty or wrong-head
worktree, builds the release binary once into a dedicated prepared directory, verifies the timed entry
without running it, freezes every identity, fully self-validates the prospective seal, and publishes
the sole final seal without overwrite. Preflight must not create any capture output or launch the
workload. A preflight failure may be corrected before capture; no timing has occurred.

The runner first reserves every protected output without clobbering, fully validates the unchanged
seal, then rechecks every candidate and file identity before launch. It launches the prepared binary
exactly once in the sealed cwd/argv and never retries or resumes. It preserves raw stdout and stderr
before validation, propagates child failure, validates into scratch, publishes accepted JSONL
transactionally without overwrite, and writes an honest disposition containing separate runner,
child and validator statuses; one workload-process invocation; lifecycle-marker counts; 16,384 timed
render calls only when the child reaches them; hashes and byte counts for raw/stdout/stderr/validator/
accepted files; and any retained recovery path.

A post-workload tooling or persistence failure preserves all recoverable raw bytes and consumes the
sole timing authority. It blocks RT5 closure and moves repair/promotion to another numbered issue;
it never permits a rerun. The accepted records and disposition, rather than launcher exit alone,
determine PASS.

## Workflow and objective gates

1. Push this stateless brief and synchronize the matching numbered GitHub issue. Astra LOW must return
   exact-head scope/applicability PASS before Luna implementation.
2. Luna HIGH/XHIGH implements one coherent attempt. Root checkpoints every green exact-path tranche;
   Astra LOW adversarially reviews the exact pushed source and untimed harness. At most three attempts
   are allowed, and gates may not be weakened.
3. Before capture authorization, pass #600's existing untimed command/tests unchanged in debug and
   release; new capture unit tests without invoking its entry; lifecycle/self-tests with zero real
   workload/timing launches; strict validator mutations; strict bench Clippy/rustdoc; fmt/diff;
   workspace/realtime/builtins/graph/lane policies; and unchanged #238/#496 tests in debug/release.
   Preserve exact commands/statuses and a restored direct harness mutation that proves a central
   validator or prelaunch-refusal gate can fail.
4. After Astra LOW source/harness PASS, root may run final preflight once. Astra LOW verifies the
   published seal and exact candidate identity. Only then may root authorize exactly one
   `bash scripts/run-input-symmetry-capture.sh` execution. No tuning or retry.
5. Astra LOW validates raw/accepted bytes, strict schemas, identities, lifecycle counts, statuses,
   hashes, two finite timing records, reviewed output digests and the honest accounting statement.
   Capture PASS depends on validity, never magnitude.
6. Integrate current main, recheck #598 exact-path disjointness, obtain Astra LOW exact integrated-head
   PASS, push, run required PR qualification, merge, require successful post-main qualification,
   synchronize and close this issue, mark RT5 delivered in #559/#560, verify remote states, and remove
   all clean delivered worktrees. Lane B supplies any artifact qualification/pinning applicability
   decision; this issue expects no shipped artifact change.

The benchmark-runner correction limit starts fresh for this newly bounded successor but remains one
bounded correction. If a runner defect remains after that correction, stop runner work and split
repair/promotion. After three failed implementation attempts, stop and rescope. No original open
finding may start before this final partial barrier and lane B's remaining partials are delivered.

## Prior evidence and applicability

#600's Astra LOW attempt-3 PASS and merged evidence establish the workload, connected ramp oracle,
full capacity-16 drain, nonzero deterministic PCM, reviewed debug/release digest, direct suppressed-
push discriminator and zero representative render allocation/realtime violations. Reuse that evidence
rather than rebuilding it. #600 deliberately performed no timing, capture, preflight or runner
execution. #431's earlier controlled capture is consumed and unrelated. The retained RT5 descriptive
measurement remains applicable; only its tooling/capture boundary is open.

At this issue boundary, all 290 numbered specs on current main have matching GitHub issues. The only
recent remote issues without main-tree specs are the two handoff specs #559/#560 on their pushed
handoff branch and active lane-B #598 on its isolated branch. No stale or missing active entry blocks
this successor.

## Astra LOW scope review

Astra LOW returned **PASS** on exact pushed brief
`9a407e508d7f47c643e08e9c3c0250070f6f388a`. Current main is contained, the GitHub title/body and
local spec match, #598 paths are disjoint, and lane B retains shipped-artifact authority. The review
accepted the bounded successor, the shared #600 workload, exact 8,192-plan-render denominator per
round, strict seal and lifecycle contracts, one workload-process/16,384-timed-render successful
capture accounting, and one bounded runner correction.

Implementation must report only observed phase completion on partial child failure; one final
preflight means one protected published seal that corrections never overwrite; buffer construction,
publication, bookkeeping and hashing remain outside the timed closure; and every post-launch tooling
failure consumes capture authority and requires a successor. Luna HIGH/XHIGH implementation may
begin. Final preflight, the real timed entry and capture remain unauthorized pending Astra LOW
source/harness PASS.
