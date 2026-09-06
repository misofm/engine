# #514 Luna attempt 1 evidence

Branch: `codex/idle-admission-clear` at root checkpoint `022e2f2b34a4c4d577d10d3f92caec9c398db583`.

## Candidate

The permitted production/test changes are confined to `hosts/host-web/src/lib.rs` and
`hosts/host-web/src/tests.rs`. The private boolean is initialized false, set after each successful
push/count increment, and used to guard the existing dense `in_flight.fill(0)` plus reset on a
successful render. Render rejection leaves the counters and flag untouched. The cfg(test)
thread-local pair counts actual physical fill calls and elements.

## Frozen gates

All gate commands were run through `/tmp/514-capture.py`; each has contemporaneous argv, cwd, head,
status, whitelisted environment and both output streams. The candidate layout probe used the
provided `/tmp/514-candidate-layout.py` wrapper, which records the same relevant identities and
restoration separately.

- Root-captured focused debug: `/tmp/514-luna1-root-focused-debug.*`, PASS, 1 test passed, 63 filtered.
- Focused release: `/tmp/514-luna1-focused-release.*`, PASS, 1 passed, 63 filtered.
- Full host-web debug: `/tmp/514-luna1-host-lib-debug.*`, PASS, 63 passed, 1 ignored.
- Full host-web release: `/tmp/514-luna1-host-lib-release.*`, PASS, 63 passed, 1 ignored.
- Corrected strict host-web Clippy: `/tmp/514-luna1-clippy-host-web-correction.*`, PASS.
- Corrected focused debug after the cast fix: `/tmp/514-luna1-focused-debug-correction.*`, PASS, 1 passed, 63 filtered.
- Formatting: `/tmp/514-luna1-fmt-check.*`, PASS.
- Diff check: `/tmp/514-luna1-diff-check.*`, PASS.
- Realtime policy and tests: `/tmp/514-luna1-realtime-policy.*`, `/tmp/514-luna1-realtime-policy-tests.*`, PASS.
- Workspace policy and tests: `/tmp/514-luna1-workspace-policy.*`, `/tmp/514-luna1-workspace-policy-tests.*`, PASS.

An initial focused debug run before capture used a non-frozen command and retained no raw record;
it is excluded from this evidence. The preserved initial Clippy failure is
`/tmp/514-luna1-clippy-host-web.*` (status 101), with two redundant casts. Only those two casts
were removed, then the corrected Clippy and focused debug records above were captured.

## Mutation

The temporary mutant replaced the production predicate with an unconditional block while retaining
the same physical-fill instrumentation and exact test assertion. The pre-test diff is
`/tmp/514-luna1-unconditional-fill-mutant.diff` (SHA-256
`b64a81a27df37fe917e7ee2e1e5f9426673678b90a073efcc656eb7f51f2260c`). The real failed run is
`/tmp/514-luna1-unconditional-fill-mutant.*`, status 101. It failed the intended idle assertion in
`tests.rs:1725`: observed `(3, 9)`, expected `(0, 0)`. The first automated restore attempt was
captured as `/tmp/514-luna1-unconditional-fill-restored.*`, status 101: the edit matched the wrong
brace in the `Observe` arm, leaving a temporary syntax error and a non-checkpoint lib blob. That
raw failed restore record is preserved and is not evidence about the candidate. The source was
then corrected exactly; the final restored exact test is
`/tmp/514-luna1-unconditional-fill-restored2.*`, status 0, 1 passed, 63 filtered, with
checkpoint-matching source bytes before the candidate probe.

## Native layout observation

The temporary candidate probe was `/tmp/514-candidate-layout.py`; its output is
`/tmp/514-candidate-layout.*`, status 0, 1 passed, 64 filtered, and its restoration record reports
clean status and the original test SHA-256. Native cfg(test) observations:

| value | baseline | candidate | delta |
|---|---:|---:|---:|
| `ReadyOwnership` size | 1336 | 1344 | +8 |
| `Option<ReadyOwnership>` size | 1336 | 1344 | +8 |
| `AudioWorkletEngineHost` size | 1864 | 1872 | +8 |
| `bridge_metadata_bytes` | 6859 | 6867 | +8 |
| `bridge_retained_bytes` | 29087 | 29095 | +8 |
| `largest_bridge_allocation_bytes` | 19238 | 19238 | 0 |
| `largest_named_allocation_bytes` | 19238 | 19238 | 0 |

Baseline artifacts are preserved under `artifacts/issue514-native-layout-baseline`; the old
baseline was not rerun or overwritten. These are native layout observations only. No shipped-Wasm
resource pin or numeric expectation was changed; target-specific consumer qualification remains
deferred to root after #511 integration.

No artifact, timing, benchmark, listening, Git, GitHub, dependency, Cargo, graph, builtins or JS
work was performed by this attempt. No source remains dirty after the root checkpoint.
