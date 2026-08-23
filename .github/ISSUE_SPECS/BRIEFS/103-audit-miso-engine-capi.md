# Sol implementation brief — issue 103 F2/F3 C ABI soundness

## Decision

**OWNER-DIRECTED DURABLE-CAPTURE RESCOPE-3 / READY FOR SYNTHETIC PREFLIGHT — SOL XHIGH PASS.** The
fresh workflow starts at candid stopped checkpoint `55bd47c` and adopts exact `ffi.rs` blob
`d09e3f289e85770a41335fdd0bfdb58a771173da` without edit. All three historical qualification
stops, three named invocations, two exact-name selections, zero valid Miri evidence and zero
implementation attempts remain counted. The owner's explicit retry direction is implemented as a
materially new evidence-delivery workflow, not a reclassification or fourth launch under the
stopped brief.

READY means synthetic detached-capture preflight only. Miri remains HOLD until root, Sol High and
Sol XHigh pass the PRE-MIRI artifact/scaffold audit. No direct interactive Miri command is allowed.

Issue 103 remains the final Issue-125 Step-0 gate. Step 1 does not begin while Miri is held or this
slice is incomplete; F2/F3 still requires pushed implementation, Sol XHigh PASS and green
synchronized evidence.

## Qualification scaffold

The adopted scaffold already adds only
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render` inside `ffi.rs`'s test module. It
carries the original `*mut Plan` in a test-local opaque `SendPlanPtr`; only that wrapper has the
audited `unsafe impl Send`, with no `Copy`, `Clone` or `Sync`. It recovers the pointer through a
method called inside `std::thread::scope`, keeps render on the parent thread, explicitly joins the
query thread inside the scope, and destroys the handles afterward.

The preserved candidate is `crates/miso-engine-capi/src/ffi.rs` blob
`d09e3f289e85770a41335fdd0bfdb58a771173da` at stopped-checkpoint HEAD `55bd47c`. It passed the
prior PRE-MIRI review and remains byte-for-byte unchanged. No `.addr()`, pointer/integer round-trip,
`expose_provenance`, `with_exposed_provenance`,
`without_provenance` or equivalent reconstruction is permitted. Sol must inspect the test-only
diff, format/Clippy-compile it, and prove the exact test name appears once before Miri. The current
whole-Plan/`RefCell` production defect must remain unchanged for the red run.

Use this exact pinned-nightly scaffold lint command:

```sh
cargo +nightly-2026-08-20 clippy --locked --no-deps \
  -p miso-engine-capi --lib --tests -- \
  -D warnings -A clippy::chunks_exact_to_as_chunks
```

`--no-deps` excludes six pre-existing protocol diagnostics outside the fence. The sole named
allowance covers only the pre-existing CAPI runtime `chunks_exact(2)` occurrence; all other CAPI
warnings remain denied, including scaffold unsafe-documentation failures. This qualification-only
allowance does not alter the final stable all-targets `-D warnings` gate, which retains no allowance.

## Durable evidence delivery

Use only the exact external namespace
`/tmp/engine-v2-103-retry.o8pyzA/evidence/issue-103-f2-f3-rescope-3/` with separate never-reused
children `capture-preflight-v1`, `pre-fix-red-v1` and `corrected-green-v1`. Each child owns its
read-only exact command, strict Bash launcher/argv runner, hashes, repository/environment record,
live combined `transcript.partial`, authoritative runner identity, atomic final transcript/digest/
command-status/capture-status files and final `COMPLETE`. No helper or artifact may appear
untracked in the repository.

Each launch is armed exactly once by atomically creating `LAUNCH_ONCE`, then uses `nohup setsid`
and a recoverable identity. The launcher uses `set -euo pipefail` plus explicit
`if ! mkdir -- .../LAUNCH_ONCE; then exit 124; fi`; every later launcher step fails closed. The
runner uses `set -euo pipefail`, disables `-e` only around its command/`tee` pipeline, immediately
captures both `PIPESTATUS` entries, and atomically publishes `exit.status` and `capture.status`.
`capture.status` must be `0` for every phase. Final files use same-directory `mv`, with `COMPLETE`
last, then the child is sealed read-only. No cleanup, overwrite, reuse or direct command invocation
is allowed.

`runner.pid` plus `process.identity` are authoritative and must match live PID, PGID, SID, start
time and absolute runner argv; PPID is excluded. `launcher.pid` is advisory. A missing `START` may
be recovered by root whenever that authoritative live runner tuple matches, even when
`launcher.pid` is absent; this continues the consumed launch and is not a rerun.

First prove the mechanism once with the spec's harmless Bash command: combined stdout/stderr tokens,
a 15-second live partial-capture interval, final exit `23` and capture status `0`. It invokes
neither Cargo nor Miri.
Then root, Sol High and Sol XHigh must independently pass the synthetic artifacts, unarmed red
launcher/runner/command hashes, exact adopted `ffi.rs` blob, test-only/provenance/barrier/join/
destruction laws, retained production defect and non-Miri preflights. Only root may arm the
intended-red child.
After valid red, implementation and green non-Miri gates, repeat the three-party artifact audit on
the separately generated corrected child before root arms it.

The exact Miri argv in both Miri children is the single named pinned command in the spec. If only a
partial transcript survives, the authoritative stable runner identity controls recovery while
live; a dead/mismatched runner without atomically complete transcript, digest and both status files
is terminal incomplete evidence. Partial text never proves red or green, and nonzero
`capture.status` invalidates every phase. `LAUNCH_ONCE` always consumes that phase's slot.

## Exact implementation

Split `Plan` into a raw-projectable `PlanResourceView` over an `Arc<SharedPlanState>`, one
`AtomicU32` diagnostic code, and render-exclusive `PlanState`. Every plan FFI entry projects only
the needed field; no whole `&Plan`/`&mut Plan` is formed. Resource queries keep the current-to-
replacement report transition and do not clear diagnostics. Replace the plan `RefCell<FixedBytes>`
with the spec's fixed code/text table and rederive the resource delta independently.

Before any slice construction, apply semantic cap, `isize::MAX`, checked conversion/extent and
alignment checks to TOML, source IDs, control frames, plane arrays, planes and render output exactly
as the issue spec states. Preserve diagnostic precedence and allow aligned retry after every
rejection.

Update the header's ownership table for all 14 exports and Issue-022's decision record. Do not
claim the synchronized report query can never block.

## Proof

E1 is the 2,000-render concurrent query test plus one detached rescope-3 pinned-Miri red at 16
iterations that must reach the retained production whole-Plan alias/data-race defect, followed by
one separately captured identical corrected green. A wrapper/header/barrier/setup/toolchain/
capture failure or unexpected pass consumes the fresh pre-fix slot without satisfying E1 and is
STOP. A failure of the sole corrected run is also STOP; no retry, alternate filter, substitute
toolchain, interactive invocation or tuning is authorized.
E2 proves resource queries preserve the last render diagnostic. E3/E4 prove oversized and
misaligned dangling inputs reject before reads and aligned retries succeed. E5 forbids whole-plan
references. E6 proves handles are `Send` while Session/Plan are not `Sync`. Execute and revert every
named red mutation, then run the complete command list in the issue spec.

Starting counters are: three named invocations total; one historical zero-test invocation; one
historical unrelated scaffold failure; one exact fresh invocation with externally interrupted
result delivery; two exact-name selections total; zero valid pre-fix red, corrected green or valid
Miri evidence; zero valid-workload retries; three preimplementation qualification stops; and zero
implementation attempts started or failed. The stopped rescope-2 command launch consumed its `1/1`
fresh pre-fix slot. `running 1 test` does not establish completion or the intended whole-Plan
diagnostic.

The successor authorizes one synthetic capture preflight, one intended-red launch and, only after
valid red plus implementation, one corrected launch. Full-success cumulative counters are five
named invocations, four exact-name selections, one valid red, one valid green, two valid evidence
invocations, zero valid-workload retries, one implementation attempt started, zero failed and the
three historical preimplementation stops unchanged.

## Fence and stop conditions

This rebrief changes exactly the Issue-103 spec and brief while preserving the existing test-only
`ffi.rs` candidate byte-for-byte. Qualification creates only the expressly fenced external
evidence artifacts. No production fence opens before valid intended-red evidence.

After valid intended-red evidence, this successor opens only the spec's existing CAPI
implementation/header fence, bounded resource-lifecycle evidence, Issue-022's single decision
amendment, and Issue-103 spec/brief. Cargo, symbols, scripts, fixtures, protocol, core, graph and
hosts remain outside scope. No benchmark or timing run.

Stop on a pinned-Miri environment failure, lost active-report transition, whole-Plan reference,
unexplained resource-owner delta, or any cap/alignment check that cannot precede slice creation.
Also stop if either fresh Miri invocation misses its required outcome. The three-attempt rule does
not create extra Miri slots or permit stale Miri evidence after an ownership/projection revision.
Do not weaken a gate or land a partial F2/F3 checkpoint.

The prior workflow remains stopped. This successor supplies its required material rescope, but Miri
may not start until the synthetic proof and root/High/XHigh PRE-MIRI audits pass. Stop on an armed
child with incomplete/corrupt artifacts, nonzero capture status, dead or mismatched authoritative
runner identity, an unrelated red, unexpected red pass or any green failure. Preserve and seal the
evidence; do not clean or rerun.

## Terminal verdict

The first named Miri command selected zero tests. The one authorized replacement ran the exact E1
test but stopped on pointer provenance erased by the scaffold's integer round-trip before reaching
the intended whole-Plan alias/data-race red. Sol XHigh returned terminal pre-implementation STOP:
two named Miri invocations, zero valid red/green evidence, zero retries of a valid workload, and
zero implementation attempts. Production is unchanged and the flawed scaffold is not retained.

The owner subsequently supplied that explicit fresh authorization. The terminal facts and counters
remain historical evidence. The fresh workflow then consumed its one pre-fix slot: pinned Miri
reported `running 1 test`, but an external reporting interruption left no persisted completion,
status or diagnostic and no surviving process. No rerun or edit occurred. This is the third failed
qualification shape, qualification remains unproven, and implementation attempts remain zero.
Preserve exact scaffold blob `d09e3f2` in the candid stopped checkpoint. The owner's subsequent
explicit retry direction permits preparation and synchronization of a materially respecified
durable-capture successor; it does not permit a direct fourth qualification retry under this brief.

This rebrief is that owner-directed successor. It preserves the terminal record and exact scaffold,
adds the detached one-shot evidence law and grants only the staged budget above. It does not itself
run Miri or begin implementation.
