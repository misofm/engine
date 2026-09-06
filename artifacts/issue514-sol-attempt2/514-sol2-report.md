# #514 Sol attempt 2 source-qualification report

## Outcome

PASS for the bounded Sol attempt 2 source correction. The exact frozen test, its release build, both complete `host-web` library suites, strict affected Clippy, formatting, diff hygiene, and the existing realtime/workspace policies and their mutation controls all pass. No production, instrumentation, Cargo, graph, pin, fixture, or artifact source changed in this attempt.

The root checkpoint is clean and pushed at `b12fbc3a54660c4f447792445ede3383d3d9b517` on `codex/idle-admission-clear`; `origin/codex/idle-admission-clear` resolves to the same commit. The corrected test source is SHA-256 `d35a91c02b8b020997348583be0846d5270f70397250487c0614bba18650cf91`, Git blob `ea9b160972cb9be5bc2b169162ac857e00461abf`.

## Bounded corrections

Only the existing `idle_render_skips_admission_counter_clear_without_losing_queue_credit` test in `hosts/host-web/src/tests.rs` changed.

- The accepted-prefix witness now performs two successful submissions before one render: two repeated matrix-destination records, followed by one fader-destination record. It compares each complete `WebCommandReport` against the exact frozen fields, requires the shared application sample `3 * QUANTUM`, and observes exact accumulated counters `[2, 1, 0]` with the pending flag set.
- The full-queue witness stages the excess matrix record in slot zero, the slot `submit_commands(1)` actually reads. After exact BACKPRESSURE report comparison it requires `[DEFAULT_COMMAND_QUEUE_RECORDS, 0, 0]` and a still-set pending flag. Rendering that host records exactly one physical dense clear `(1, 3)`, resets the flag, and zeros every counter. A full-capacity refill is then accepted at the next exact application sample, its successful render again records `(1, 3)`, and the following command-free render records `(0, 0)`.
- The real pending-command `TimeOverflow` host now seeds output with `-1.0`. The rejected render requires `STATE_FAILED`, bitwise positive-zero output, and exact `web.render.rejected\t$\n`, while retaining the ready owner, exact pending counters/flag, and `(0, 0)` clear stats. WRONG_STATE re-entry preserves the failed state, diagnostic, pending ownership, and zero clear count.

## Captured gates

The capture wrapper retains its earlier `/tmp/514-luna1-` filename prefix; every new label begins `sol2-`, and this report identifies Sol attempt 2 as the owner. Each label has `.command.json`, `.stdout`, `.stderr`, and `.status` files at the listed prefix. Every recorded status is `0`.

- `sol2-focused-debug`: exact debug test, 1 passed, 63 filtered. Raw prefix `/tmp/514-luna1-sol2-focused-debug`. This was captured immediately before the required checkpoint at base HEAD `a4d16dd32f9d7118547e37b6fea58a55675bf7cd`, with only the eventual committed test blob modified; its recorded source SHA-256/blob exactly match `b12fbc3a`.
- `sol2-focused-release`: exact release test, 1 passed, 63 filtered. Raw prefix `/tmp/514-luna1-sol2-focused-release`.
- `sol2-host-lib-debug`: complete library suite, 63 passed, 1 ignored, 0 failed. Raw prefix `/tmp/514-luna1-sol2-host-lib-debug`.
- `sol2-host-lib-release`: complete release library suite, 63 passed, 1 ignored, 0 failed. Raw prefix `/tmp/514-luna1-sol2-host-lib-release`.
- `sol2-clippy-host-web`: `cargo clippy --locked -p host-web --lib --all-targets --all-features -- -D warnings` passed. Raw prefix `/tmp/514-luna1-sol2-clippy-host-web`.
- `sol2-fmt-check`: `cargo fmt --all -- --check` passed. Raw prefix `/tmp/514-luna1-sol2-fmt-check`.
- `sol2-diff-check`: `git diff --check` passed. Raw prefix `/tmp/514-luna1-sol2-diff-check`.
- `sol2-realtime-policy`: 42 marked regions in 12 files passed. Raw prefix `/tmp/514-luna1-sol2-realtime-policy`.
- `sol2-realtime-policy-tests`: realtime policy mutation tests passed. Raw prefix `/tmp/514-luna1-sol2-realtime-policy-tests`.
- `sol2-workspace-policy`: workspace policy passed. Raw prefix `/tmp/514-luna1-sol2-workspace-policy`.
- `sol2-workspace-policy-tests`: workspace policy mutation tests and gate-library counter-mutants passed. Its stderr intentionally contains the directed-fault/control diagnostics exercised by the passing script. Raw prefix `/tmp/514-luna1-sol2-workspace-policy-tests`.

The complete debug and release suite logs explicitly show the frozen new test and all retained named regressions passing: `command_ack_names_the_exact_application_sample`, `paired_fader_and_matrix_commands_share_the_acknowledged_application_sample`, `an_effect_parameter_command_names_the_exact_application_sample`, `command_flood_is_typed_backpressure_and_leaves_the_render_untouched`, `a_refused_solo_submission_leaves_the_console_untouched`, `a_solo_that_changes_nothing_emits_nothing`, and `render_failure_retains_ownership_and_silences`.

Except for the pre-checkpoint focused debug capture described above, every captured command records clean status, HEAD `b12fbc3a54660c4f447792445ede3383d3d9b517`, and the same final source SHA-256/blob.

## Prior-proof applicability and deferred delivery

The attempt-2 diff changes only later assertions and command staging within the existing test. `hosts/host-web/src/lib.rs` remains SHA-256 `aaea4b3c344e3f948fe965e1e8ed0c8394b5fb3843c2a50974890e7be9391924`, Git blob `0400e53fab15ba4066057432cba11928431efe2a`. The production flag, physical-fill instrumentation, and the initial repeated-idle assertion are unchanged. Therefore the accepted attempt-1 unconditional-fill mutation still addresses the same production fill site with the same instrumentation and still fails the unchanged initial `(0, 0)` assertion when it observes `(3, 9)`. No new mutation run is needed for these later test-only assertions.

The same exact diff leaves every production type and field unchanged, so the accepted candidate native layout/resource observation remains applicable: ReadyOwnership/Option 1,344 bytes, host 1,872 bytes, bridge metadata 6,867 bytes, retained 29,095 bytes, with largest bridge/named allocation 19,238 bytes. The baseline/layout runs were deliberately not repeated.

Integrated artifacts/current consumers, post-#511 scalar and SIMD Wasm builds, the shipped render-closure allocation/free gate and controls, drift review, and actual PR qualification remain deferred until #511 delivery and default-branch integration. This source PASS does not claim integrated delivery or issue closure. Root retains repository/GitHub/spec ownership and will package this report for the requested consolidated Astra verdict.

No unexpected failures occurred. No benchmark, timing run, artifact generation, Git mutation, or GitHub mutation was performed by Sol attempt 2.
