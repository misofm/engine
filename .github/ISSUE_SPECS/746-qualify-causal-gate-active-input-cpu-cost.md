# Qualify causal gate/expander CPU cost on an active-input workload

Queued measurement successor to #738, not a blocker for its causal zero-latency product slice. The current console workload selector has no gate/expander workload. No CPU number or speedup is claimed by #738.

Freeze one smallest active gate workload after #738 delivery: 48 kHz, 128-frame blocks, scalar and native bank paths, fresh bounded input every block, audible gate opening/closing with explicit per-channel activity and actual zero-fault checks. Reuse an existing benchmark entry point through one bounded adaptation; do not create a general runner framework or change DSP to improve a number. Scope the exact input, parameters, clock region, host/compiler/source identity and validator before timing.

Preflight CLI/schema/units/persistence/overwrite refusal and activity without timed work. Run exactly one invocation with one warmup and two measured rounds, retaining both rounds, raw stdout/stderr/exit and source/host provenance outside delivered worktrees. Report descriptive cost and limitations; no before/after speedup, floor, capacity or sound-quality claim without corresponding evidence. A runner failure after measurement preserves raw output and ends timing; no hidden retry.

Astra XHIGH scopes, Luna XHIGH implements a necessary bounded benchmark adaptation, Sol XHIGH verifies under the owner pipeline unless changed. Root owns checkpoints, GitHub synchronization and any artifact decision. At implementation boundary freeze exact paths and gates; maximum five attempts. This queued issue authorizes no current source change or benchmark run.
