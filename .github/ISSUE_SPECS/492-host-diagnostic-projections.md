# 492: Reuse typed session and effect diagnostic projections during host preparation

Current delivered e2b77fd6 still contains meaningful repeated conversions in crates/host-core/src/prepare.rs (the actual package path is crates/, not hosts/). parse_host_session and compile_host_model duplicate exactly the session::DiagnosticSet projection; native-effect preparation, attach_effect_console and attach_effect_observation duplicate exactly EffectDiagnosticSet projection. Builtin and graph mappings are each single adapters for distinct types. diagnostics.rs already centralizes byte encoding and the64-line bound. Do not create a second encoder or a diagnostic trait framework merely to eliminate superficial similarity among distinct types.

Smallest closable product: two private typed adapters in prepare.rs returning PrepareDiagnostics, used at the existing two session and three effect error sites. Suggested signatures consume session::DiagnosticSet and effect_compiler::EffectDiagnosticSet respectively, matching current map_err ownership. Each retains the exact existing kind, iterator over original diagnostics and diagnostic_lines invocation. Use map_err(adapter) directly at those five call sites. Keep builtin and graph mappings unchanged; they are not repeated instances of these two types. This closes the actual same-type duplication, not an assertion that every heterogeneous projection must become identical. Root should record this precise CP7 disposition rather than silently count all seven call sites as five identical functions.

Frozen behavior: Session versus Effect category; original diagnostic iteration order; code.as_str for session versus existing effect code; unchanged Display path formatting; exact tab/newline bytes, empty result behavior and first64 truncation. The existing host surface projects ONLY code/path: original session spans/messages are not emitted in PrepareDiagnostics today. Preserve that intentional current omission and retain upstream diagnostic values/semantics; do not invent a new span/message wire format or claim those fields are encoded. No sorting, deduplication, filtering or new cap. Parse/compile/effect/console/observation ordering and short-circuit points remain unchanged. Keep error temporaries consumed/dropped at the same map_err boundary; no retained prepared ownership or moved graph failure resources. Do not move the graph failure adapter/drop order into this cleanup.

Allowed paths: crates/host-core/src/prepare.rs (two private functions and five substitutions, plus compact cfg(test) tests if appropriate); existing crates/host-core/tests/prepare.rs only if a public entry-point parity assertion is needed; numbered spec/evidence. diagnostics.rs production encoder, PrepareDiagnostics public API, builtin/graph crates and host ABI consumers are unchanged. No runtime, framework, dependency, allocator or source-control redesign.

Finite proof: direct typed-adapter tests with literal expected category and code-tab-path-newline bytes for representative ordered two-diagnostic inputs in EACH type. Construct real diagnostic values, not a parallel projection implementation. Include nontrivial structured session path and original span/message fields where present, proving output remains precisely the current code/path projection; effect string paths should differ so reorder is visible. Include empty and65-entry inputs to prove the existing first64 bound is neither bypassed nor reimplemented. Expected bytes must be literal/purpose-built fixed pattern, not computed using diagnostic_lines or the new adapter. No mutation campaign is necessary for this mechanical deduplication.

Retain existing host-core prepare integration suite, including prepare_reports_the_session_shape_and_feeds_sources_independently, shape_policy_pins_rate_and_quantum, session_validation_owns_the_launch_rate_set, console_attaches_bounded_control_and_meter_halves_in_canonical_track_order, no_console_request_attaches_nothing_and_charges_nothing and dense_refusal_diagnostics_are_count_bounded. Existing dense_refusal_diagnostics_finish_under_one_second_in_release is historical test behavior; this scope adds no timing benchmark or new time acceptance. Run `cargo test --locked -p host-core --test prepare` debug/release and the new private typed tests with a nonempty exact filter if placed inline. Run fmt/diff and affected strict host-core Clippy; retain actual commands/statuses and classify inherited unrelated failures before expanding scope.

Before implementation root numbers/synchronizes and confirms actual source base/ownership; no competing host-core work may be overwritten. One Luna pass, Sol2/3 after adversarial FAIL and hardstop after3. Source identity and actual-head PR/requiredCI remain delivery gates. Normal artifact verification may be needed because source-equivalent refactors can change shipped bytes; no unsolicited repin/browser expansion without an actual mismatch/ruling. No measured speed claim: this is control-plane maintenance, not render-path optimization. #463 remains sole runtime feature.

Read-only source inspection and existing CP reconciliation; no tests/builds/timing, source/spec edits or Git/GitHub mutations performed.

## Numbered baseline

GitHub492 number/title matches this spec. Base is delivered main e2b77fd65e142b19d032c688b71d3ef810d7ddb2. Root boundary audit found no missing local numbered issue identity. This is independent host-core maintenance; #463 remains sole runtime feature. Pending Astra actual numbered-base approval before fresh Luna1.

## Numbered approval and Luna attempt 1

# Astra #492 numbered scope/base review — PASS

Exact head1c4576c3b49689156794ba9952869c90ca37a3de in engine-cp7-diagnostics, basee2b77fd65e142b19d032c688b71d3ef810d7ddb2. Independently checked only numbered spec differs from delivered base; complete approved draft body is retained with numbered title/baseline. Root reports GitHub492 matching identity/body and boundary missing0; no independent remote query was requested for this source-identity review.

Approve fresh Luna1 within the exact two private typed adapters/five substitutions and compact typed parity proof. Preserve original category/order/code/path encoding,64-line truncation and omission of spans/messages from the existing host byte surface. Builtin/graph one-off projections, diagnostics.rs encoder/API and graph-failure ownership/drop order remain untouched. No shared trait framework, default-feature/dependency or runtime change.

The frozen direct literal expected bytes for both typed diagnostic sets, empty/65 cases and existing host preparation/Clippy/fmt gates remain mandatory. Retain original upstream diagnostic semantics; do not claim the helper restores fields the host format never emitted. Source base and current ownership permit this independent maintenance slice while463 remains sole runtime feature. Root owns checkpoints, remote synchronization and eventual exact-head PR/requiredCI.

Read-only source/Git inspection. No tests/builds/timing or source/spec/Git/GitHub mutations performed.

Root adopts PASS and assigns fresh Luna1. Pause at the first coherent compiling/focused-green tranche for exact-path root checkpoint before more implementation. Retain actual command/cwd/source/log/numeric exit records; root owns Git/GitHub.

## Luna attempt 1 evidence

Implementation514deb8b extracts two adapters and five callsites;42264ee4 restores original borrowed effect iteration and adds initial typed coverage;d9674f98 expands session literal projection and bound coverage. Focused typed test1, host prepare debug/release12passed+1ignored each, affected strictClippy/fmt/diff return0 in retained command/cwd/source/log/status records. Earlier test compile/assertion failures remain recorded. These progress checkpoints are one Luna implementation attempt, not independent verdicts.

Current submitted test asserts literal ordered session/effect bytes, session category and upstream span/message presence, empty effect input and line-count bounds for both types. Root sends the exact submitted test against the full frozen proof for one consolidated Astra verdict; no narrower coverage is substituted for the original gate. Source remains confined to prepare.rs.

## Luna attempt 1 verdict and Sol attempt 2

# Astra #492 Luna attempt 1 — FAIL

Exact head1377ed958c86a074afb469b263c34071d34139c3, source d9674f98, engine-cp7-diagnostics. Read frozen scope, cumulative source/test and retained evidence. No tests/builds/timing or repository/Git/GitHub mutations.

Production is accepted: exactly two private typed adapters replace the two session and three effect closures. The original borrowed diagnostic iteration, categories, code/path encoder,64 bound and map_err ownership/short-circuit locations are unchanged. Builtin/graph projections and shared encoder remain untouched. Session ordered literal bytes and upstream span/message presence, plus effect ordered literal bytes, are useful accepted tests.

One finite original proof group remains for Sol2: finish exact typed category/first64 parity. Effect tests never assert PrepareRejection::Effect, so a category regression passes them. Assert it for the nonempty representative (and preserve empty behavior). Both65-entry tests merely count newline bytes; the effect test also excludes index64, but either still permits duplication/reordering/substitution among the retained lines. Compare complete expected first64 bytes, built from a fixed known pattern independent of diagnostic_lines/the new helper, and assert the actual input diagnostic population is65 before projection. Use distinct ordered paths so changed first64 selection is observable. The current session test's malformed top-level document does not establish a clean65-diagnostic population; construct the actual public parser input with otherwise-valid required fields and exactly65 intended errors, assert their known code/path population, then compare its first64 expected encoded bytes. Do not derive the expected result by calling the adapter or encoder, and do not add a general fixture framework or mutation campaign.

Explicit applicability correction to the original empty-input wording: session::DiagnosticSet has a private Vec and only pub(crate) from_vec, which asserts nonempty. An empty session diagnostic set is not constructible through the supported API. Do NOT add a session constructor, unsafe/transmute or production helper overload merely to manufacture it. Root should record that the empty-input gate applies to publicly constructible EffectDiagnosticSet only; session's nonempty type invariant is the precise reason for the exemption. This is not a waiver of any reachable behavior. Likewise public parser-produced session diagnostics are the correct test seam rather than a forbidden direct private constructor.

Retained package56 payload hashes/sizes checked without errors. Reported focused1 and host prepare12+1ignored in both profiles, strict Clippy/fmt/diff0 remain useful; earlier compile/assertion failures are candid. Passing counts do not establish the missing category/full-prefix assertions. After the small test-only correction run the frozen affected tests/gates on the final candidate and retain actual commands/statuses. No production repair is requested, no further Luna revision; root may synchronize this finite FAIL/applicability ruling and assign Sol2. #463 runtime remains independent.

Root adopts FAIL and the exact empty-session applicability ruling. Sol2 owns only the missing category and complete known first64 byte assertions using a parser-produced exactly65 diagnostic population. Production adapters and accepted previous proof remain frozen. Pause at first coherent focused-green checkpoint; root owns commits/pushes/GitHub and consolidated verdict.

## Public session diagnostic cap applicability

# Astra #492 session diagnostic-cap applicability — APPROVE precise amendment

Read current test-only checkpoint1ed69049 and source cap: session/diagnostic.rs defines MAXIMUM_SESSION_DIAGNOSTICS=64; parse.rs:88 and validate.rs:767 stop appending at that bound. Combined with the private/nonempty DiagnosticSet construction, the normal public parser/compiler cannot supply65 diagnostics to this adapter. The earlier demand for an actual65-entry SESSION input was inapplicable and must be corrected explicitly, not met through a new constructor/unsafe/API.

Accept the candidate's65 intended unknown fields on an otherwise-valid canonical document, with an asserted actual64-entry pre-adapter set of exact ordered UnknownField/code/path values, followed by complete expected64-line byte equality. This proves preservation of the upstream-capped session population. It does NOT independently prove session-adapter truncation from65; no such claim should be recorded. The separate publicly constructible EffectDiagnosticSet65 case proves the shared host truncation behavior, with category and full-prefix equality. Keep the prior empty-session invariant ruling.

No production change, cap change, broader test interface or additional gate is needed. Root should synchronize this narrow applicability correction and allow Sol2 to finish the existing gates. This is not the consolidated Sol2 verdict; final source/evidence review remains pending. No tests/builds or repository/Git/GitHub mutation performed.

Root adopts this correction. Focused-green test-only checkpoint1ed69049 is pushed; Sol2 may finish existing gates with production adapters unchanged.

## Sol attempt 2 final evidence

Finalsourceb5ac4dbe retains the accepted production adapters; tests assert effect category and actual65 ordered diagnostic entries with exact first64 bytes. Otherwise-valid session input supplies65 intended errors; the approved parser cap ruling establishes actual64 known ordered entries and exact64 projected lines. No claim of reachable65 session diagnostics or empty session set is made.

Final focusedtyped debug/release1each, prepareintegration12passed+1existingignored each, strictalltargets/allfeaturesClippy, correctedfmt and diff all return0. Raw records preserve initial zero-test filters, initial parser-cap mismatch and fmt1; final exact tests and correctedfmt supersede them. Awaiting one consolidated Astra Sol2 verdict.

## Astra Sol attempt 2 source acceptance

# Astra #492 Sol attempt 2 — PASS

Exact reviewed head: 72a4fc52eb72ea85254bdadf3ac02b0b4a4d5823, source b5ac4dbec3d3817aa4c99466e970777cafd7133e, engine-cp7-diagnostics. One consolidated source verdict against the complete numbered scope and adopted empty/session-cap applicability rulings. Read-only source, Git and retained-record inspection; no tests, builds, timing or repository mutations performed.

The single remaining proof group is satisfied. The representative effect projection now asserts Effect category and complete ordered literal bytes. Its independently known 65-entry input is checked for every code/path, then the complete expected first 64 lines are compared, so duplication, missing lines, reordering and substituted retained paths cannot pass on a newline count alone. Empty EffectDiagnosticSet still produces empty bytes. Session retains the two exact literal lines, Session category, and assertions that upstream span/message information exists while the host format intentionally omits it. The otherwise-valid canonical document with 65 intended unknown fields supplies exactly 64 parser-capped diagnostics; every actual code/path is checked before comparing the complete fixed expected 64-line byte pattern. Neither expected pattern calls the adapter or shared encoder.

The documented applicability limits are accurate: public session diagnostics cannot be empty or exceed the upstream 64 cap. The session case proves preservation of that actual population, not an independently exercised 65-to-64 session-adapter truncation. The constructible 65-entry effect case exercises the shared host truncation. No new constructor, unsafe access, cap, public API or parallel encoding framework was introduced.

Accepted production remains unchanged: two private typed adapters and the original two session/three effect map_err substitutions, preserving borrowed iteration, category, encoding, ownership and short-circuit positions. Builtin/graph projections and diagnostics.rs remain untouched. Changes since the reviewed applicability checkpoint 1ed69049 in prepare.rs are only rustfmt wrapping; the final evidence head has no crates/hosts/tools/Cargo/config delta from b5ac4dbe and the worktree is clean.

Retained final exact private-test commands execute one test in each profile, both status 0. Prepare integration executes all 12 passing tests plus the existing ignored test in each profile, including the named canonical, console, shape, rate and bounded-diagnostic gates. Strict host-core all-targets/all-features Clippy, corrected fmt and diff statuses are 0. Final command source records identify b5ac4dbe. Earlier zero-test filters, parser-cap mismatch and fmt failure remain historical evidence rather than successful coverage. Independently verified all 130 manifested payload hashes and byte sizes, with exact tracked coverage of 131 files including manifest; no omissions or extras.

Root may proceed with proportional delivery on integrated current main: freeze and verify source identity, run ordinary artifact verification, and retain any actual mismatch before requesting a bounded pin/current-consumer qualification amendment. This source PASS does not authorize an automatic repin, changed PCM/resource expectations, timing, or merge without the required actual-head PR review and successful required CI. No further product correction is requested.

Root adopts source PASS. Artifact delivery is deliberately sequenced after #463 merges, then current main will be integrated and the final combined source verified once. GitHub492 remains open pending actual PR review/qualification/merge; no delivery or performance claim yet.

## Current-main delivery integration

Root integrated deliveredmain571dfc5b (PR493) and its closure record without conflicts. Entire host-core crate remains byte-identical to Astra-accepted source. Freeze this combined source for ordinary shipped-worklet verification; prior standalone artifacts are not reused as combined evidence. Any actual mismatch requires the bounded delivery ruling before repin.
