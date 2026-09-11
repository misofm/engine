# Qualify causal multiband-compressor CPU cost on an active-input workload

Queued measurement successor to #739, not a blocker for its causal product slice. No current qualified console workload measures active multiband processing, and #739 claims no CPU number or speedup.

After #739 delivery, freeze one smallest workload at 48 kHz with 128-frame blocks for scalar and native bank paths, refreshing bounded input every block and requiring active reduction/release in both bands plus zero faults. Reuse one existing benchmark entry point through a bounded adaptation; do not create a general framework or alter DSP to improve a number. Freeze stimulus, parameters, clock region, host/compiler/source provenance and validator before timing.

Preflight arguments/schema/units/persistence/overwrite refusal and activity without timed work. Run exactly one invocation with one warmup and two measured rounds; preserve both rounds and raw stdout/stderr/exit outside delivered worktrees. Report descriptive cost and limitations, with no unsupported speedup/floor/capacity/sound-quality claim. A post-measurement runner failure ends timing with raw evidence preserved.

Astra XHIGH scopes, Luna XHIGH implements any necessary bounded adaptation, Sol XHIGH verifies under the current owner pipeline. Root owns delivery and evidence. Freeze exact paths/gates before implementation; maximum five attempts. This queued issue authorizes no current source change or benchmark run.
