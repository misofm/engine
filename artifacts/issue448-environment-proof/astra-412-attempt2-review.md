# Astra #412 Sol attempt 2 review

**FAIL at `8e5f597557be44f717fee26d02b393dd71c4a6cb`.** The implementation improves source completion, but the frozen error-classification and directed/counter-proof contract is still incomplete. One final coherent Sol attempt 3 remains; another FAIL must hard-stop/rescope.

Reviewed the three complete checkers, affected suites, cumulative helper/workflow delta and full frozen brief/prior verdict. Shared helper is restored to the prerequisite API, and the workflow retains exactly the one authorized research-suite invocation. Environment paths remain in a NUL-containing file until intentional conversion; required roots/nonempty populations and closing-backtick grammar are restored. Research extraction/conversion/sort stages are now separately checked, and its finite synthetic fixture correctly tests an existing empty section, distinct Primary keys and outside-Primary bibliography resolution. Migration retains the exact tokens/APIs/bans and helper semantics. Keep these useful changes.

Independently ran two tiny disposable probes; results are `/tmp/astra-412-attempt2-probes.log`. No repository/Git/GitHub mutation, broad build or timing performed. The supplied combined focused run reports all real gates/suites green; the probes below show why that is not yet sufficient.

## 1. Git fatal classification still has a concrete false pass

The fallback condition accepts any status128 output containing `not a git repository`. In an actual initialized Git fixture, set GIT_DIR to a nonexistent repository: real git reports a fatal invalid-repository error with that substring, and the checker selects find and exits0 (`env vocabulary: ok (1 names, one MISO_ENGINE_ prefix)`). This is an execution/configured-repository failure, not the deliberately non-Git fixture case.

Make the nongit classification deliberately narrower, preserving genuine nongit positives while rejecting an invalid explicit repository/worktree configuration and other repository faults. Do not classify by status or a broad substring alone. Add that real initialized-repository failure and the original fatal-probe case alongside the nongit/Git positives, with diagnostic preservation. This corrects the original frozen classification requirement, not a new Git feature.

## 2. Actual counter wrappers still accept unrelated failure

All counter sites treat ANY nonzero return from assert_fault/assert_rg_fault/assert_migration_fault as a successful control. Those assertions return the same generic failure for checker success, missing diagnostics, missing partial output, syntax/setup failure or another policy rejection. The outer wrapper never requires the named unexpected-checker-success assertion and its distinguished status. The evidence's claim that losing the expected diagnostic suffices explicitly does not meet the frozen contract.

Independent reproduction replaced both research disposable mutants immediately before execution with a script containing only `exit 23`. The suite still exits0 and prints `dsp research mutations: ok`. Thus the physically corrected helper layout and source-replacement checks do not prevent the very unrelated-failure certification identified in attempt 1.

Use the same focused assertion for original and mutant, but distinguish checker unexpected success from every other assertion failure. Require only the exact intended assertion/status in the counter wrapper. Verify every actual edit affects the intended unique production callsite (including env edits after mutation), record executed original/mutant outcomes, and restore/prove positive fixtures. No generic runner is needed. Keep all five frozen groups: env discovery, env latest consumer, migration final ban, research whole-note producer and research late consumer.

## 3. The finite directed table and otherwise-valid payloads remain incomplete

The new tests do not implement the complete frozen per-operation table:

- Environment omits selective find, NUL conversion, normalization, both path exclusions, stray sort/prefix filter, tools/scripts used-name read, fragment filter, used sort, vocabulary delimiter conversion/sort, first comm difference and final count/format transforms. Current source/vocabulary/probe injections have only one partial shape, not their error-only/full-valid-output pair. The source shim's `source partial` is not the selected grep's valid output, and the vocabulary shim invents an undocumented PARTIAL row rather than the fixture's exact valid result. Missing scripts/empty used/documented semantic controls and fatal128 discrimination are not present in the affected suite.
- Research omits both Primary and whole-note tr/sort faults. Its two awk probes always emit `awk partial`; there is no error-only pair, and the Primary payload cannot satisfy the required two-key section if status is ignored. Primary/whole rg partials emit only `[DUMMY-A]` instead of the real otherwise-valid full extraction. The Primary result cannot pass the distinct-key rule; it is therefore not the promised full-looking producer proof. Select the actual operation/phase, delegate all other calls, and use its real valid output before injected failure. Preserve the distinction between Primary population and whole-note literal resolution.
- Migration now selects each required/banned semantic class in both modes, but its required-document/API payload is the same invented `intended partial row`, not that query's actual successful output. Replace it with delegated successful output plus an injected diagnostic/error. For negative predicates where correct output is empty, a delegated clean-empty/error control is valid; do not demand fabricated successful match data for an absence check.

Complete the already frozen table in these same suites. A command genuinely removed from production needs no resurrected test; commands still present do. Do not replace the missing cases with broad early faults, file permission changes or another corpus/framework. Each selective assertion must name the intended stage and injected error outcome, not merely any nonzero status.

## 4. Executable entry points regressed

The checkpoint changes `scripts/check-env-vocabulary.sh` and `scripts/test-dsp-research.sh` from mode100755 to100644. Bash-prefixed CI hides this, but the existing env checker can no longer be invoked directly as before. Restore the executable modes; no scope instruction authorized retiring that entry behavior.

## Final bounded revision

Complete these four existing-contract groups in one coherent pass, preserving the accepted source fixes, exact grammar, helper restoration, #406 controls and the single CI addition. Run the three real gates and affected suites plus syntax/diff; record the actual per-callsite mapping and distinguished counter outcomes. Root checkpoints before the final Astra verdict. Broader workspace/PR qualification waits for focused PASS. No runtime/artifact/benchmark change, speculative parser hardening, fourth attempt or gate weakening is authorized.
