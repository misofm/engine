# Astra #412 Luna attempt 1 review

**FAIL at `6e6e6eaa4f2fcae11b06506b1c004b272f66cb2f`.** One bounded Sol revision is required against the already frozen producer/grammar/fixture contract. Reviewed complete nine-path delta, three checkers, actual new tests and approved brief. Ran only three tiny disposable environment probes; no Cargo/builds/timing or repository/GitHub mutations.

Useful progress: source file enumeration stays in a NUL-containing temporary file until conversion, per-file grep now distinguishes match/no-match/error, migration uses the shared required/forbidden helpers, research whole-note extraction is no longer hidden behind process substitution, and the one allowed research CI invocation is present. These do not complete acceptance.

## 1. Environment source still falsely passes errors and changes grammar

Independently reproduced against a tiny fixture under `/tmp/astra412-probe-1vszjtkd`:

- Existing required tools/scripts directories with no names and vocabulary `table` exit0: `env vocabulary: ok (0 names, one MISO_ENGINE_ prefix)`. Frozen used/documented populations are required nonempty.
- A vocabulary row missing its closing backtick is accepted with one name. New documented grep dropped the closing backtick from the original exact row regex. Restore the exact original grammar.
- A selective Git shim emits `fatal: simulated repository error` and exits128; the checker falls back to find and exits0. Status128 alone cannot identify genuine non-Git input, and stderr was discarded. Preserve deliberate non-Git fixture support without treating every fatal repository/process error as that case.

Additionally, `stray_names="$(sort ... | grep ... || true)"` still swallows both producer/consumer failures outright. Other grouped pipelines accept aggregate status1 even when an earlier transform failed (e.g. first exclusion grep error followed by clean no-match second filter). Used/vocabulary grouped transforms and final display count likewise do not implement the explicitly frozen per-producer contract. Check each retained operation before consumers, use real successful no-match only where permitted, require tools/scripts roots/nonempty used and documented sets, preserve diagnostics, and do not depend on global pipefail/errexit. Reuse captured populations; no repeated discovery/framework.

## 2. Research still conflates producer errors with allowed emptiness

The Primary-key and whole-note chains inspect only a combined pipeline status and treat status1 as clean no-match. A sort/tr execution failure returning1 is not a valid empty key population. Whole-note such failure can bypass bibliography checks after valid Primary counts. Counts also rely on implicit command-substitution errexit. Separately capture/check extraction, conversion, uniqueness/count and lookup stages as frozen; shell builtins may replace unnecessary external transforms. Preserve the current exact bracket grammar, literal bibliography lookup, sixteen headings and nonempty rule rather than broadening research policy. All late console/template/listening input errors remain in the directed table.

## 3. The purported counter-controls do not exercise the claimed mechanism

Migration mutant is copied to `$temp/check-effect-state-migration-mutant.sh`; physical sourcing resolves `$temp/lib/gate.sh`, which does not exist in the fixture (the helper is `$temp/scripts/lib/gate.sh`). It fails before the intended serialization call. Its assertion accepts any nonzero exit and discards output. It neither proves a replacement occurred (grep only checks the existing line label) nor reruns the SAME intended fault assertion expecting unexpected success from the swallowed-error mutant.

Research has the same misplaced-mutant/helper failure. Its shim exempts `-o` key extraction and injects on earlier filters.md searches, not the named whole-note producer. The sed replacement touches both sort pipelines rather than uniquely targeting the whole-note handling. The earlier removed listening literal remains missing in the actual `$temp/dsp-research`; `$temp/mutant-research` is unused. Thus independent early failures certify the printed counter result. No intended late consumer counter exists.

The new env “counter-control” is a Git listing fault injection only: it makes no actual gate mutation and accepts arbitrary failure without the stage diagnostic. Required environment discovery and late-consumer mutation groups are absent.

Replace these with the exact frozen actual-counter groups using valid otherwise-passing fixtures and physical script/helper layout. Assert the replacement count/changed callsite, run the same selective negative assertion against normal and mutated code, and require the mutant to be rejected by THAT assertion for unexpected checker success. Restore originals and prove positive green. No permanent mutants, generic runner, broad-any-failure or expected-panic substitute.

## 4. Directed fixtures omit most of the frozen callsite contract

Only Git listing and migration final-ban injections were added. Missing directed mechanisms include Git classification/find/path transformations/source versus vocabulary reads/both comm consumers; migration required doc/API and other two bans; research section/Primary/whole-note extraction and late bibliography/template/listening reads with otherwise-valid partial output. Cover the existing finite table from the final brief, grouping identical repeated helper predicates by semantic class but not substituting an early failure for distinct late consumers. Each needs error-only and otherwise-valid-output-then-error, exact status/diagnostic assertion and successful surrounding metadata. chmod-only unreadability remains unsuitable.

The research suite copies the real corpus instead of making the approved small synthetic structural fixture. Its “empty-section” mutation deletes the Fixtures heading itself, so only missing-heading is tested. Its “duplicate-primary” appends a key outside Primary, so it cannot establish fewer than two distinct Primary keys; the unchanged section still controls that count. Build the compact synthetic two-key fixture and directly establish missing note/support, genuinely empty existing section, insufficient/duplicate Primary, unresolved outside-Primary key and final literal cases. Preserve all existing env and #406 effect-runtime semantic controls while adding the scoped ones. No research content additions.

Shared helper `--` change: it is generally sensible option termination and the root helper suite passed, but no #412 helper call requires a leading-dash pattern (bibliography already uses its own `rg -F --` helper). The frozen conditional helper allowance does not justify unrelated API hardening. Prefer restore the unchanged helper for this slice; alternatively demonstrate a concrete necessary callsite before retaining it and test that exact behavior. No new helper framework/modes needed.

## Bounded Sol revision

Complete these four groups in the already allowed scripts/suites/spec and one existing CI call. Preserve useful checked migration/source scanning and all established grammar/empty-set rules. Record an explicit final callsite-to-selective-test mapping plus actual counter assertion evidence. Run three real checkers, three affected suites, shared helper suite only if changed, shell syntax/diff. No broad workspace/PR qualification before focused source acceptance; no runtime/artifact/benchmark scope.

Luna attempt1 is consumed; Sol has attempts2 and, only if needed, final3. A third FAIL requires a preserved checkpoint and explicit rebrief, not another repair. This verdict adds no obligations beyond the frozen #412 contract.
