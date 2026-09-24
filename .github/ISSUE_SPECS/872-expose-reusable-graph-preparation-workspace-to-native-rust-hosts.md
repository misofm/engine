# Expose reusable graph preparation workspace to native Rust hosts

GitHub: https://github.com/misofm/engine/issues/872

## Outcome and dependencies

Depends on [#871](https://github.com/misofm/engine/issues/871). Make the workspace usable through the real shared host preparation pipeline, so native Rust embedders and the benchmark can prepare successive full hosts with one workspace.

## Exact implementation slice

Own `crates/host-core/src/{lib.rs,prepare.rs}` and one focused host-core integration test. Re-export `GraphCompileWorkspace` and add `prepare_host_runtime_with_workspace(compiled, caps, workspace)` with the ordinary preparation entry's output and diagnostics. Route it into the existing common preparation pipeline and `compile_with_builtins_and_workspace`; avoid a duplicate preparation pipeline. Existing stateless, console, spectrum and observation entry points preserve their contracts and use ephemeral workspaces.

The caller owns the workspace separately from `PreparedHost`, the render plan, source handles and plan exchange. Its retention cap is explicit at construction. Expose/report scratch capacity separately from existing plan/source/effect resource reports; do not silently add it to canonical plan estimates or imply existing caps already cover additional caller-owned scratch. Release/reuse is a control operation, never a render operation.

The initial deployable capability is opt-in native Rust preparation. CAPI `SessionState::command` currently invokes `runtime::compile::prepare_runtime` for structural replacements; this issue does not make that owner retain a workspace. Browser boot/reboot likewise remains stateless. CAPI adoption with correct retained-resource admission is a measured successor, not an unreported omission.

## Objective gates

- Prepare A, retain its complete host/plan, prepare B using the same workspace, then release/drop the workspace and render both hosts. Verify equal expected PCM, source ownership, reports and diagnostics against the existing entry point.
- Refuse a candidate after preparation has used compiler scratch; A remains usable and the next valid B succeeds. Separate early validation rejection from late rejection in tests.
- No scratch pointer or destructor reaches render; use the existing render allocation/free audit on the A/B ownership test. Preserve the existing plan-exchange contract without adding a separate queue-qualification project.
- Run focused host-core and graph-compiler tests, affected strict Clippy, release/target checks proportionate to the shared pipeline, and formatting/policy gates. Cross-target builds prove portability; they are not browser performance measurements.

No ABI, SDK, generated artifact, host owner, protocol admission or DSP edits belong here.

## Delegation and checkpoint contract

Assign one fresh `gpt-6-luna` agent with `reasoning_effort=max` to this bounded issue. Root briefs/reviews the issue and owns exact-path commits, integration and GitHub synchronization. This user-selected model workflow supersedes historical Terra defaults for this series. Each attempt is one coherent implementation pass plus one root adversarial verdict; maximum five attempts, then preserve evidence and split/rebrief. Root commits every compiling, focused-green tranche before any further implementation layers on it. No implementation slice should exceed half a working day; split before expanding beyond the named boundary. Independent tooling may run in its own worktree/target; overlapping source owners are sequential. Final independent verification remains the user's fresh Astra XHIGH review after implementation and capture. Status: scoped; implementation and official measurements have not started.
