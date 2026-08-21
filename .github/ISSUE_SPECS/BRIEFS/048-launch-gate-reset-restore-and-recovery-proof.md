# Sol implementation brief — issue 048 launch gate reset, restore, and recovery proof

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1 after remote synchronization.** Consume Issue 014's preserved green
gate/expander checkpoint without treating it as PASS. This issue permits one Terra attempt and at
most one bounded Sol correction. A second failure stops. No benchmark or timing command is
authorized; `timed_benchmark_invocations=0`.

## Frozen implementation boundary

Work only in `miso-engine-gate-expander` tests and, if a proof exposes a real defect, the directly
corresponding gate reset/restore/recovery production path. Do not change the frozen descriptor,
equations, coefficient/update graph, parameter table, latency/tail, layout-1 payload, resource
rows, scalar/bank interface, core gain kernel, registry, graph or PDC.

Use 48 kHz, quantum 128 and the existing exact payload helpers. Evidence must execute scalar and
the available x86 W8 backend; an early return for unavailable W8 does not close this issue.

## Exact reset proof

Prepare asymmetric L/R values and at least two distinguishable W8 tracks. Seed normal ring audio,
retarget at least one ID 1–4 ramp, and snapshot before reset.

- `DiscontinuityKeepParameters`: cursor and every ring word become `+0`; phase is Open, `G=+0`,
  and `hold_remaining=K`; IDs 5–8 retain their prepared values; each ramp current equals its
  pre-reset target and remaining is zero.
- `FullToDefaults`: cursor/rings become `+0`; phase/Open/K/+0 is rederived; IDs 5–8 and every ramp
  current/target return to the originally prepared values with remaining zero.
- Calling full reset after discontinuity proves the original defaults were not overwritten.

Assert the complete scalar L/R payloads and at least two W8 track payloads. No shared track/lane
state is permitted.

## Exact active restore continuation

Use `threshold=-20 dB`, `ratio=20`, `range=48 dB`, `hysteresis=6 dB`, `attack=1 ms`, `hold=0`,
`release=5 ms`, `lookahead=10 ms`. Render asymmetric finite-normal main audio below threshold for
at least 640 samples, start one legal 64-update ramp, render 17 updates, and snapshot with
`G != 0` and `remaining=47`.

Restore the snapshot into a freshly prepared peer. Feed the uninterrupted and restored instances
the same next audio, spans and partitions `1,63,64,128`; compare every PCM bit, each per-call report
and the complete payload after every partition. Extend the existing W8 restore test so one original
bank continues beside the restored bank and eight scalar peers; compare every track's PCM, payload
and report. Two peers restored from the same potentially incomplete payload are not an oracle.

## Exact identity and recovery proof

For scalar signed-zero identity, use a connected finite-high sidechain to keep the lane Open while
main carries `-0` left and `+0` right at sample zero. Render through sample 480 and require exact
zero sign bits, `G=+0`, and zero sanitation/recovery. For W8, default hold longer than latency may
keep the unconnected zero detector in identity; require the packed select to return the same bits.

Warm scalar and W8 delay rings with nonzero finite-normal audio. Through test-only private access,
set `gain_reduction_db=NaN` on exactly one left lane/track immediately before one frame. The scalar
and matching W8/scalar-peer cohort must:

- emit that lane's delayed dry sample exactly;
- report exactly one left recovery and zero right/other-track recoveries;
- set only the injected lane to Open/K/+0 while preserving its advanced finite rings; and
- match PCM, full payload and per-track reports exactly between W8 and eight scalar peers.

Do not make malformed state restorable or expose a public fault-injection API.

## Ordered gates and stop conditions

1. Run the four focused proof groups and existing gate/core tests.
2. Prove descriptor, metadata, state sizes and resource rows unchanged; inspect the production diff.
3. Run format, focused warning-denied Clippy, locked workspace check/tests, warning-denied workspace
   Clippy/rustdoc, and workspace/realtime/effect-runtime/rack/graph policies and mutations.
4. Record W8 execution, exact outputs/payloads/reports, candidate identity, attempt verdict and
   `timed_benchmark_invocations=0`.

FAIL for a DSP/API/layout/resource change, recovery affecting another lane, restore divergence,
signed-zero loss, skipped W8 evidence, Issue-047 qualification work, audit/target/object execution,
benchmark/timing/listening, weakened assertion or a third attempt. A directly exposed bounded
state-path repair is allowed; a required redesign stops for a new rebrief.
