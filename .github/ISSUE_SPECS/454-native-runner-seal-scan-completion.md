# Complete native PCM runner static seal scans

Parent #403, audit #349 TOOL-11 and #306. Queued scope; no implementation authority until numbered current-base Astra approval and root assignment. Planning base: delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`; its gate source is identical to the inspected `5a4a7d2071194cf6118241e24d073824668e3387`.

## Frozen bounded outcome

Complete both native PCM runner static seals

Files: check-native-pcm-runner.sh and its existing v1/portability suites. Preserve `[root] [v1|portability|all]`, exact prefixes and both mode outcomes. No runner execution or publication adapter redesign.

V1 producers: independent `generate.py --check` (verification only), find RIFF files -> wc exact4, required dependency/ABI matches, forbidden dependency matches, source bypass scan -> exact ABI exclusion, and four-root reachability scan -> own-package and doc-comment exclusions. Retain rf64 requirement and exact fixture identities. Source scans must finish before exception filtering; final empty allowed. Check find and wc individually even if failed find emits all four valid paths. Required roots are crates/hosts/tools/sidecars for reachability; an empty individual root is legal. Existing fixture must include them all. No new nonempty reachability requirement: this is a forbidden population.

Portability producers: required fixed boundaries/contract literals; forbidden impossible claims/hard-link/replacement/cleanup patterns; Python Unix-import predicate; counted identity checks>=2 and O_NOFOLLOW exactly4; late ownership checks. Preserve all current numeric/regex/Python behavior. Python status/read failure is fatal, not a new portability ruling. Required source/contract deletion is separate from selective later scan errors with intact inputs.

Finite controls: retain all fixture corruption, ABI bypass and portability mutations; add otherwise-valid missing required root/file, partial/error discovery/count, each positive/forbidden/filter class, late contract/source read. Two actual status-loss mutants: allowed complete source scan before exclusions, and late forbidden publication scan. Same targeted unexpected-success assertions, not generic nonzero. Use actual fixture generator --check; no Rust build or native runner invocation.


## Shared execution and evidence rules


All six real checkers are non-timed. Real audit-leak invokes Cargo tree offline; native invokes the fixture verifier, and interchange invokes static Python validators, not the100-process reference campaign or benchmark binary. These distinctions must stay explicit in commands and logs.

Existing `test-effect-interchange-benchmark.sh` declares hermetic lifecycle and routes to scratch fake cargo/git and fake-benchmark.py; its phase name timed_started is synthetic metadata, not audio timing. Existing test-rack-benchmark.sh likewise uses scratch fake cargo/git/rustc and a synthetic record emitter. Preserve those selected paths and prove the command targets remain the fakes before invocation. Do not invoke real run-*.sh directly or call runner main to qualify a static scan change. The108 suite currently sources only two functions; extend it to exercise the whole checker/optional directory and conditional-call propagation, while retaining source mutations. Copied script fixtures must source the physical intended helper, not accidentally a fixture-owned alternate implementation.

Use the existing checked helpers where flags/output contracts fit; filename grep discovery, bespoke parsers, hash/count and command-status captures can remain small local wrappers. No demonstrated need for a shared helper API expansion was found. Helpers retain their existing negative controls; child proof should target its new actual call sites. All new red assertions require operation, returned tool status and sentinel/output witness as appropriate, with a separate distinguished unexpected-success branch. Two controls per proposed bounded outcome are sufficient representative causal proof here; the finite directed table covers the remaining actual sites without a mutation campaign.

Each child: correct its existing valid fixture, preserve old mutations, run real non-timed gate(s) plus applicable hermetic suites and proportional syntax/policy checks; source PASS then root's unchanged-count workspace and actual-head PR/required CI. No artifact regeneration or benchmark publication. If a runner defect outside these scanner contracts survives one bounded correction, preserve its evidence and assign it separately, not as a reason to widen #403.

Root should number these four outcomes and reciprocal parent accounting before assignment, then freeze the exact merged source for the first one. Each follows Astra brief/review, Luna1, Sol2/3 after FAIL, hard stop. Parent403/306/349 remains open until delivery accounting is complete. This report neither closes a program nor authorizes implementation while another feature/tooling tranche owns overlapping work.


## Standing parent contract


- A search result has THREE outcomes: matches, clean no-match, execution failure. A required path/read/parse error must never be interpreted as clean. Preserve stdout/stderr needed to distinguish them.
- Explicit conditionals capture command status; do not rely on set-e inside functions invoked from conditionals, pipelines/process substitutions, or standalone `! command`. Helpers must not toggle caller shell options or install caller traps/change cwd.
- Resolve sourced library by the script's own physical location before cd into a fixture root. A fixture-root argument selects data to inspect, never a different helper implementation. Preserve existing script CLI/environment and diagnostic prefixes.
- Preserve regex/glob/allowlist semantics. Filtering legitimate exceptions happens AFTER a successful checked source scan; an empty filtered result is allowed, failed source traversal is not. Do not use `--glob '*'` as a blind replacement for “no glob,” because it can alter ignored-file traversal.
- Known required roots remain required; no blanket filter-to-existing-directories or mkdir workaround. If an optional root is currently legitimate, document that specific policy and retain missing-required-root red cases.
- Expected discovery must be non-vacuous. Capture producer failures before a consumer loop and assert nonempty output when the policy requires at least one input. Record any legitimate empty-set case explicitly. All original #306 nine-loop debt must be assigned in the frozen per-child call-site inventory; if a remaining original site lies outside the 21 roster, record a stateless bounded successor before parent closure rather than silently omitting it.
- Every migrated gate has a clean positive control, retained old violation mutations and a new missing-root red case. Prove the changed helper is actually reached, not only that an unrelated earlier manifest check fails. Where deletion is intercepted by prior checks, additionally inject a controlled rg failure at the relevant scan while all required metadata remains valid.
- Red helpers explicitly reject unexpected success and distinguish intended predicate failure from missing tools/syntax errors. Each new helper-level failure class gets at least one counter-mutation demonstrating the assertion is live.
- No Cargo tests for prose/shell implementation mirrors; existing full workspace unchanged-count requirement is retained at coherent child boundary. Run all existing affected shell suites and applicable current required CI. No artifact byte regeneration, benchmark launches or publication solely for gate extraction.



## Delivery boundary

This issue owns only its explicitly named checker(s), affected existing suites and this decision record. The shared six-gate context above does not authorize edits to sibling outcomes. No helper API change, runner repair, runtime change, artifact regeneration or timed workload is authorized. #403/#306/#349 remain open until all original obligations are delivered. Root owns all Git/GitHub mutations and checkpoints; Astra briefs/reviews, Luna attempt 1 then Sol attempts 2/3, followed by hard stop and explicit rescope after a third failure.

## Astra numbered scope approval — queued

## Luna attempt 1 implementation record

The native runner checker now observes discovery, count, required and forbidden search statuses
before evaluating payloads or applying the two reachability filters. It retains the exact
`python3 -I -B .../generate.py --check` verifier invocation, requires `crates`, `hosts`, `tools`,
and `sidecars` while permitting each root to be empty, and preserves the V1, portability, and all
CLI modes. The affected suites cover missing roots, valid-output discovery failure, and the two
approved status-loss counter-controls; the latter both reach the distinguished unexpected-success
assertion with status 97.

Focused evidence (2026-09-05): `bash -n` passed; checker `v1`, `portability`, and `all` modes
passed; `scripts/test-native-pcm-runner-v1-policy.sh` and
`scripts/test-native-pcm-runner-portability-v1-policy.sh` passed. Logs are retained at
`/tmp/454-v1-check.log`, `/tmp/454-portability-check.log`, `/tmp/454-all-check.log`,
`/tmp/454-v1-suite.log`, and `/tmp/454-port-suite.log`.

# Astra #454 numbered scope review — PASS, queued

Exact clean planning checkpoint `a63e302bab98b5db3f4c26f1bb0af6ecf53844d6`, `/home/bl/misofm/engine-454-plan`. Read the full numbered brief, actual combined checker, both existing suites and fixture generator's check/generate branches. Assigned checker/suites/fixture source is unchanged from inspected `5a4a7d20` and delivered `60519995`; the stale earlier planning-base sentence does not change this verified current-source premise.

PASS for queued readiness. No further amendment or general design work is required. Root must wait for #453 delivery and compare the actual merged checker/helper/fixture base before Luna assignment. This review approves scope; the current checker still contains the intentionally assigned unchecked scans.

Roster is exact: scripts/check-native-pcm-runner.sh, test-native-pcm-runner-v1-policy.sh and test-native-pcm-runner-portability-v1-policy.sh, plus numbered evidence. Shared copied six-gate prose does not authorize sibling runner/validator/test changes or helper expansion. Preserve `[root] [v1|portability|all]`, root default `.`, subject default `all`, both current failure prefixes and invalid-subject exit2. Portability-only remains valid with just its source/contract; do not impose V1's fixture/four-root prerequisites on that independent mode. Default/all must propagate either mode's failure.

The producer inventory matches source:

- V1 surface checks and independent fixture verification; immediate regular riff-*.wav find/count exactly4 and required rf64 row; four exact direct dependency queries; three forbidden bypass dependency queries; eight ABI-operation presence queries; numbered tool-source bypass query then the exact miso_engine_v1_compile_session exclusion; four-root reachability query with original Cargo.toml/*.rs globs, followed by own-package and comment-line exclusions.
- Portability fixed source/contract presence queries; forbidden concurrency/publication/cleanup patterns; Python Unix-import window predicate; FileIdentity count>=2, late post-publication/owned-cleanup presence checks and O_NOFOLLOW count exactly4.

All original flags/patterns/counts remain binding. In particular the existing reachability comment exclusion `:[0-9]+:[[:space:]]*///?[[:space:]]` matches the accepted two-or-three-slash line form; do not narrow it to only Rust doc comments or broaden it into an arbitrary comment parser. Preserve raw numbered path/line output until both filters consume it. Source scan success is checked before exclusions; each invoked filter distinguishes0 retained rows,1 legitimate empty, execution error. No blind `--glob '*'` substitution or false nonempty requirement belongs on forbidden/reachability populations. Required scans/counts cannot accept clean absence. Search/count statuses must be observed before numeric comparison, even if failed producers emit exactly4 or otherwise valid output.

The current V1 fixture creates tools/crates/hosts but omits sidecars. The approved brief correctly requires fixing the fixture and testing missing sidecars with all earlier tool/corpus metadata valid; production must require all four reachability roots. Individually empty roots remain legal. Portability fixtures intentionally do not have that population and need no such expansion. Existing fixture drift, dependency/reverse-dependency and all nine portability mutations remain required; generic existing red helpers do not replace new precise error assertions.

Read-only generator inspection confirms the non-build seam: CHECK is true only for exact `--check`; publish compares read_bytes instead of write_bytes, and session_payload reads/validates stored JSON. The subprocess Cargo session canonicalizer is reachable through session() only when CHECK is false. Therefore the checker MUST retain exact `python3 -I -B .../generate.py --check`; do not invoke generation or omit/change that argument. It computes independent expected fixture bytes but performs no file regeneration, Rust compilation, native PCM runner execution or benchmark. Python read/parse/verification errors remain fatal. No generator source/fixture identity changes are authorized.

The finite directed cases correspond to actual mechanisms: V1 required and forbidden scans, both exclusion stages, find and count separately, late portability fixed/count/forbidden reads, Python verifier/read failure, missing required roots/files, and valid empty forbidden/filter positives. Inject errors selectively after earlier valid operations; include otherwise-valid output/error for required reads and discovery. For a forbidden scan, clean-empty/error is a meaningful failure-control payload; matching prohibited text independently violates policy and cannot establish a swallowed-error unexpected-success proof.

Freeze the two representative actual production counter-controls as follows, consistent with the numbered brief:

1. Four-root reachability source-status loss BEFORE own-package/comment exclusions. Delegate the real valid source query, retaining only allowed tool/comment matches, then inject nonzero and a sentinel. Original checker must reject that operation/status. The uniquely verified status-loss edit must preserve payload and allow the unchanged filters/policy to succeed, causing the SAME assertion's distinguished unexpected-success outcome.
2. Late portability forbidden publication/cleanup scan status loss, for example the existing FakeEntry owned/wrong-published alternative query after earlier boundary checks. Use the original clean source with empty match output plus injected error/sentinel. Original rejects at that scan; actual unique mutation permits later unchanged Python/count/ownership checks to finish and the SAME assertion rejects unexpected success. No arbitrary failed setup or syntax/panic exit qualifies.

These are a concrete selection of the already-required two controls, not an additional mutation campaign. Preserve real-mode positives, exact diagnostic/operation/status witnesses, restored-source proof, proportional syntax/policies and source-review ordering. Full unchanged-count workspace and actual-head PR/required CI remain later delivery gates. No new semantic bans, publication adapter repair, timed workload or generic framework is needed.

#454 remains queued after #453; #455/#456 and parent #403/#306/#349 retain their independent obligations. No tests, builds, timing, source/spec edits or Git/GitHub mutations were performed. Only this /tmp review was written.


## Delivered base PASS and Luna attempt 1

# Astra #454 delivered-base review — PASS

Exact reviewed integration head `f233e38293af9233924541d5f55097ccc368b45e`, `/home/bl/misofm/engine-454-plan`. The worktree is clean and the complete tree is byte-identical to delivered main `b6836835` (checked Git tree diff exits0). The retained #453/#462 delivery is therefore present; no implementation tranche is hidden in the integration.

Compared with approved checkpoint `d7e728ff`, the #454 numbered spec, assigned native-PCM checker, both native-runner suites and shared gate-lib are unchanged. The intervening script changes belong only to delivered #453/#462's two dependency checkers and their two suites. The parent-spec conflict resolution introduces no source drift or new #454 semantics.

The existing `/tmp/astra-454-numbered-scope-review.md` remains binding and sufficient. No rebrief or scope amendment is needed before Luna attempt1. Preserve its exact three-script roster, V1/portability/all mode separation, fixture check-only invocation, four-root versus valid-empty distinction, checked producer/filter/count table and the two precisely selected SAME-assertion controls. The current unchecked operations are the assigned implementation work, not evidence that the base is already accepted.

Luna gets one coherent pass, followed by Astra adversarial review; Sol2/3 only following FAIL. Full workspace and actual-head PR/required CI remain delivery gates after source acceptance. Parent #403/#306/#349 and the other children are not closed or waived by this base approval.

Review used read-only Git/source inspection; no tests, builds, timing, repository/spec changes or GitHub mutations were performed.


Root assigns Luna attempt1 on this byte-identical delivered base. Exactly the native PCM checker and two existing suites plus this record are authorized. No generator/fixture byte changes, builds, runner execution, benchmarks, sibling scripts or shared helper changes. Stop compiling/focused-green for root checkpoint and one Astra verdict; full workspace/PR/CI follow source PASS.


## Luna attempt 1 verdict and Sol attempt 2 assignment

# Astra #454 Luna attempt 1 — FAIL

Exact reviewed head `7c58ac16ae651869357364d47aedd506d0312565`, `/home/bl/misofm/engine-454-plan`; clean four-path checkpoint. One consolidated verdict against the complete numbered scope and delivered-base approval. Preserve this useful checkpoint; assign one coherent Sol attempt2 addressing the four finite groups below. No workspace/delivery qualification is authorized from this submission.

## Accepted progress

The modes and default/all dispatch, two diagnostic prefixes, generator's exact `python3 -I -B .../generate.py --check`, RIFF immediate regular-file predicate/exact4 and RF64 row, dependency/ABI spellings, reachability globs and both exclusion regexes, and portability source/Python predicates are retained. Portability remains usable without V1 fixture/root population. The V1 fixture now contains sidecars. Reachability source/own/comment stages are separately captured, required roots are explicit, and legitimate empty filtered output is allowed. Find and wc now have separate captured statuses. Existing corruption/dependency and nine portability mutations remain present.

The retained six logs show real v1/portability/all success, invalid-mode usage, and both suite successes; they do not supply independent status files. Credit the root/author terminal reports as such, not invented status artifacts. The two new suites print reached97 outcomes, but their completeness and assertion causality require corrections below.

## 1. Finish checked source/filter/count execution before predicates

Two actual false passes were independently reproduced on this exact source with disposable PATH shims, leaving repository files unchanged:

- The late `rg -c O_NOFOLLOW` producer prints `4` and emits a sentinel, then exits7. The checker exits0 and prints portability ok. Record: `/tmp/astra-454-attempt1-count-probe.log`.
- The tool-source bypass producer emits only a numbered allowed `miso_engine_v1_compile_session` row plus a sentinel and exits7. The exclusion legitimately removes that row and exits1; the pipeline's rightmost nonzero status is1, which the checker treats as clean. V1 exits0. Record: `/tmp/astra-454-attempt1-bypass-probe.log`.

These are the originally assigned producer/consumer obligations, not new policies. Split the tool-source scan and exact ABI exclusion into checked stages just like reachability, allowing clean empty output only after checked execution. Capture both FileIdentity and O_NOFOLLOW count producers before applying the unchanged >=2/exact4 predicates; valid numeric output cannot excuse producer failure. Preserve required-match/no-match/error distinctions at required dependency/ABI/source literals, both contract literals and late ownership literals. Their current `|| fail` branches reject failures but several erase returned status and operation identity, so they do not yet meet the frozen diagnostics/evidence contract. Preserve explicit Python verifier and import-predicate failures with their actual status and witness, and scratch creation must fail explicitly rather than depending on errexit under a conditional caller.

Do not change include/exclude grammar, count thresholds, mode prerequisites, or introduce a shared helper API/framework. Check operation completion and preserve stdout/stderr needed by directed assertions; do not rely on a pipeline or count substitution's final status. Existing quiet flags may only remain where their completion semantics satisfy the actual single-input operation; do not use quiet success to justify an unchecked multi-input scan.

## 2. Complete the finite V1 fault table with otherwise-valid fixtures

The new V1 suite adds missing-root cases, one synthetic find error, and one reachability source error. It does not exercise the assigned wc error, required/forbidden dependency reads, required ABI read, source-bypass producer and its exclusion consumer, own-package filter, comment filter, or verifier error/read-failure classes. An early global rg failure cannot cover these later independent operations.

Add selective cases for these original sites/classes, with earlier fixture generation/metadata and earlier scans succeeding: verifier error; find and wc separately; a required direct dependency and a late ABI required match; forbidden dependency scan; tool-source bypass scan and exact ABI-exclusion filter separately; four-root reachability source and each of its two filters separately. Required reads/count/discovery need both error-only and faithful otherwise-valid-output/error shapes; valid empty forbidden/filter output plus an error must also fail. Where an operation normally has no output, preserve that legitimate empty output rather than manufacture a prohibited match that already fails policy. Delegate/capture actual real output and status before injecting the failure. `one two three four` is not the claimed complete valid four-path discovery, and a single fabricated own-package row is not the frozen complete reachability source payload.

Retain and explicitly exercise valid empty individual reachability roots and allowed own-package/comment-only populations. Keep otherwise-valid missing required surface/fixture file/root cases exact; deleting tools is correctly intercepted by the earlier tool-surface prerequisite and must not be called proof of later tools traversal. Cover the later traversal by selective injection with the tool intact. Preserve exact two/three-slash comment exclusion and raw numbered paths. No fixture generation, runner execution or Rust build.

## 3. Complete the finite portability fault table and mode/error discrimination

Only the late FakeEntry forbidden scan has new directed coverage. Complete the original classes: required fixed source boundary, both required contract literals and contract prohibition; source forbidden publication scans with a genuinely late target; Python Unix-import read/parse/exit failure; FileIdentity and O_NOFOLLOW counts independently with valid numeric output/error; post-publication/partial/final ownership presence checks with selective later targets; required source/contract deletion in otherwise-valid portability-only fixtures. Required match status1 remains a policy absence, status>1 an execution error; valid empty forbidden status1 remains legal. Do not make portability depend on the V1 corpus or four roots.

Keep all existing nine semantic mutations and real v1/portability/all/invalid2 checks. New red assertions must identify exact operation, returned status and sentinel/output as appropriate, with a separate setup/wrong-diagnostic outcome and distinguished unexpected-success outcome. Check the count/source failures above through normal and conditional invocation so set-e cannot serve as accidental error propagation. A compact local assertion/selector is sufficient; no generic campaign or new policy.

## 4. Use the SAME original directed assertion for the two actual counter-controls

Current baseline errors are checked by inline expressions, while mutants are checked by different `assert_counter_mutant` functions that accept success as97. That demonstrates a status-loss edit can make the checker succeed, but does not prove the ORIGINAL operation/status/sentinel assertion rejects that success. The numbered brief explicitly freezes SAME-assertion causality.

Refactor the two selected directed assertions so their only variable is the checker path. Run the original, uniquely verified actual source-status mutation, and restored checker through that same function with the same input/shim. Original/restored directed case must pass its expected-failure assertion; mutant must reach exactly97 plus the named unexpected-success message. Wrong operation/status/missing sentinel/setup/syntax must use a distinct non97 outcome. Verify the mutation really changed exactly its intended guard and retain the diff or equivalent checked substitution evidence. Do not add further production mutants.

For reachability, delegate the actual complete otherwise-valid source query before injecting status/sentinel; keep only real allowed own-package/comment payload so unchanged downstream exclusions finish cleanly. For the late publication scan, retain actual clean no-match output plus injected error, all earlier and later untouched checks valid. The same assertion must reject swallowed status, not merely any nonzero or a copied checker that cannot find its fixture. Restore and recheck normal execution. Keep tool executable delegation portable and shell-quoted rather than introducing author-specific paths.

## Revision/delivery boundary

These four groups are the frozen producer table and two existing counter-controls, not expanded implementation scope. Exactly the checker, its two suites and issue evidence remain allowed; no helper/workflow/runner/generator/fixture-byte changes, runtime/artifact work or timing. Sol2 should deliver one coherent complete pass with explicit retained commands/statuses and source/restoration evidence. Source review follows before root's unchanged-count workspace and actual PR/required CI. Parent403/306/349 and sibling scopes remain open.

Review used read-only Git/source/log inspection plus two tiny disposable shell probes; no Cargo/builds, native runner, benchmark, repository or GitHub mutation was performed.

## Sol attempt 2 implementation record

The checker now captures completion status before consuming every assigned source, exclusion, and
count result. In particular, the tool bypass source scan is separated from its exact ABI exclusion,
and both portability counts reject valid-looking numeric output from a failed producer. Required
dependency, ABI, contract, and ownership queries retain their operation identity and returned
status. Python verifier/import failures and scratch creation also report their actual status.

The V1 suite exercises the complete frozen fault classes with otherwise-valid fixtures: verifier,
find and wc, required and forbidden dependencies, required ABI, bypass source/exclusion, four-root
reachability source and both filters, missing surfaces/roots, valid empty roots, and allowed
own-package/comment-only populations. The portability suite retains its nine mutations and adds
required source/contract deletion plus selective required, forbidden, Python, count, and three late
ownership failures. Delegating shims use the discovered real executable and preserve real payloads
before injecting a sentinel and nonzero status.

Exactly two actual source mutants remain. Each is verified by a retained unified diff and is passed
to the same directed assertion as the original and byte-restored checker. Original/restored reject
the injected operation failure; the unique mutant reaches the distinguished unexpected-success
branch with status 97, while setup or diagnostic mismatch returns 96.

Focused evidence (2026-09-05): syntax, real `v1`, `portability`, and `all`, invalid-subject exit 2,
and both policy suites passed. Exact command logs and independent status files are retained under
`/tmp/454-sol2-*`, including the two mutation diffs. No Cargo build, native runner, fixture
generation, benchmark, timing, Git, or GitHub operation was run.

Root assigns Sol attempt 2 for one coherent pass over these four frozen groups. Preserve the accepted checkpoint and all existing gates; pause when the exact three scripts and this evidence record are ready for root checkpoint and Astra review. No workspace build, timing, shared helper, or sibling changes are authorized.


## Sol attempt 2 verdict and final Sol attempt 3 assignment

# Astra #454 Sol attempt 2 — FAIL

Exact reviewed clean head `fcb434e1233c50d9dac7c1c1e2553197423f46fa`, `/home/bl/misofm/engine-454-plan`. One consolidated verdict against the full frozen issue and four attempt-1 correction groups. Production progress and the two corrected SAME-assertion controls are accepted; the original finite proof contract is still incomplete. Assign ONE final coherent Sol3 pass below. A subsequent FAIL is a hard stop/rescope, not a fourth repair.

## Accepted source and controls

The actual false passes are closed: bypass source and exact ABI exclusion now capture/check separate statuses before policy, and both portability counts capture/check the producer status before numeric comparison. Scratch creation, Python exits, required contract/ownership reads now fail explicitly and retain status. Existing mode separation, verifier exact --check, root population, include/exclude/Python grammar and thresholds are retained. No runtime, runner, helper or workflow change is present.

The suites now selectively reach all named search/filter/count classes, rather than relying on an early global rg failure. Valid empty roots and an allowed comment population are explicit. Missing V1 roots/surfaces and portability source/contract cases exist. Preserve the old semantic mutations.

Both actual production controls now pass ORIGINAL, mutant and byte-restored checker paths through the SAME assertion. The retained diffs change the reachability status guard and late forbidden status guard respectively. Mutants reach97/named unexpected success, while original/restored calls return0 after finding the expected injected diagnostic. Do not add a third mutant or replace these accepted mechanisms. The status files independently record syntax/v1/portability/all/suites0 and invalid mode2. Those green runs cover the implemented cases, not the missing cases below.

## Final group 1 — faithful paired producer modes and actual delegate outcomes

The newly introduced shims all run the real producer then unconditionally replace its status at the selected call. `install_rg_fault` saves status but does not verify its expected value before injecting7; find/wc delegates do not capture/check their real status at all. Therefore a failed delegate or wrong input can masquerade as the assigned otherwise-valid payload. The assertions usually check only diagnostic/sentinel and never establish that a count/discovery/source payload was complete and otherwise acceptable.

Complete exactly these six previously named NONEMPTY producer operations with BOTH modes: (a) suppress output, injected execution error; (b) complete actual otherwise-valid output, then injected error. The operations are RIFF find, RIFF wc, bypass source, four-root reachability source, FileIdentity count and O_NOFOLLOW count. No additional operation or mutant is being added.

For full mode, capture/check the real producer status BEFORE injection and retain its complete payload. Assert real find status0 and exactly the four valid immediate fixture paths (order independent, no invented one/two/three/four); real wc status0 and count4 with actual four-row input; source scans status0 with their actual complete numbered allowed payload; identity count status0 with its actual valid >=2 value and O_NOFOLLOW status0/count4. Do not hardcode filesystem traversal order or treat a nonzero delegate as a successful source. For count/source payloads, preserve enough side evidence to prove the operation input/output, rather than infer it from the new checker's failure message.

For the other already-present selective cases, check the real delegate's expected status too: required quiet matches0 (their valid stdout is already empty), forbidden clean searches1, and filters0 or1 according to the concrete valid fixture population. Legitimately empty stdout does not need duplicate identical modes; do not manufacture prohibited matches as the full payload. The existing two representative counter-controls retain their exact same assertions and actual source edits, now with checked underlying source status/payload. No new framework, corpus or tool dependency.

## Final group 2 — finish the named read/absence/conditional evidence

The V1 verifier shim demonstrates successful real verification followed by injected status6, but there is no actual verifier read-failure case. Keep MANIFEST.tsv/generate.py and tool surface valid; remove one required referenced fixture/session input reached by the real --check verifier, and require its read diagnostic plus the checker's python status. This must not invoke generation or Cargo. The portability Python case currently exits8 before executing the supplied script; retain that controlled exit and add the originally required late read failure with the source/contract intact through earlier scans, then make the real Python read fail at that operation. Do not confuse early source deletion with the later Python consumer. These are original read-failure obligations, not new semantic bans.

Portability missing-source and missing-contract currently use the old broad `reject` helper, which accepts any nonzero and discards diagnostics. Make these NEW cases require the actual source/contract-surface refusal on an otherwise-valid portability fixture. Legacy nine semantic mutations may remain legacy helpers; no general rewriting campaign is requested.

Exercise the corrected bypass and count failures under conditional invocation of the checker/functions as specified, in addition to normal execution, so their propagation demonstrably does not depend on errexit. Retain mode positives and invalid2. Required-count clean absence must remain a policy failure distinct from execution error: the current FileIdentity/O_NOFOLLOW producer status1 is labelled “scan failed”; distinguish clean no-match from status>1 without changing >=2/exact4 semantics. Check absence/failure at these two count sites specifically; do not expand this into all possible count values.

## Final group 3 — explicit unexpected-success versus wrong/setup outcome

Only the two counter-control assertion functions currently distinguish unexpected checker success97 from wrong diagnostic/setup96. `assert_rg_fault` instead returns96 for BOTH success and misclassification, and the new find/wc/verifier and missing-file assertions use generic exit1. The previous review explicitly required the new directed assertions to distinguish these outcomes.

Use the existing local suite assertion style consistently: checker success -> named unexpected-success97; actual nonzero missing/wrong operation/status/sentinel/payload ->96; correct targeted rejection ->0. Preserve actual operation identifiers and underlying producer status evidence. No extra production mutant is needed: retain the accepted two SAME-assertion controls and ensure their unchanged97 checks continue to discriminate. Show the final original/mutant/restored outcomes and retain exact commands/statuses/diffs. Do not call a wrong diagnostic or missing executable proof of rejection.

## Frozen boundary

Exactly the checker, two existing suites and issue evidence remain allowed. Most remaining work is test/evidence completion; production needs only the narrow count-status1 diagnostic separation if needed. Preserve all accepted scanner predicates and two actual controls. No helper API, workflow, publication adapter, generator, fixture-byte source change, runner execution, native/Wasm build, benchmark, new framework or mutation campaign. Full workspace and actual PR/required CI follow source PASS only; parents403/306/349 retain their obligations.

Review used read-only source/Git/log/status inspection. No tests, builds, timing, repository or GitHub mutations were performed. This is the complete finite final revision list; no further unenumerated matrix is requested.

Root assigns the final Sol attempt 3 against precisely these three groups. Retain accepted source and both counter-controls. Pause focused-green for exact-path checkpoint and one final Astra verdict; no fourth repair is authorized after a failed verdict.

## Sol attempt 3 implementation record

The accepted scanner corrections and exactly two source mutants are preserved. Production changes
are limited to distinguishing a clean count absence (`rg` status 1) from a count execution failure
(status greater than 1), while retaining the original `>=2` and exact-four predicates and captured
stderr.

The six named nonempty producers now have paired empty/error and faithful full-output/error cases:
RIFF discovery, RIFF count, bypass source, four-root reachability source, FileIdentity count, and
O_NOFOLLOW count. Before injection, each shim checks the real executable's expected status. Full
cases additionally validate the actual four fixture paths, four-row wc input/count, complete
numbered allowed source populations, FileIdentity value `>=2`, or O_NOFOLLOW value `4`, and report
the checked payload evidence with the sentinel. Other selective required, forbidden, and filter
cases check the real delegate's expected status before injecting their error.

The real fixture verifier read-failure case removes only a referenced session JSON while preserving
the verifier, manifest, and tool surface. The late portability read case leaves source and contract
intact for all earlier scans and passes the actual Python predicate a missing source path. Missing
source/contract diagnostics, count clean absence, execution failures, and conditional checker
invocations are distinguished. New directed assertions map unexpected checker success to 97 and
wrong operation/status/sentinel/payload or setup to 96.

Focused evidence (2026-09-05) is retained under `/tmp/454-sol3-*`: syntax; actual `v1`,
`portability`, `all`, and invalid-subject status 2; both policy suites; source diff checks; and the
two original/mutant/restored mutation diffs. No generator write, fixture-byte change, Cargo/native
runner/build, benchmark, timing, Git, or GitHub operation was performed.


## Final source PASS and delivered-base qualification freeze

# Astra #454 FINAL Sol attempt 3 — PASS

Exact reviewed clean head `7724a581e320a3e080e4d0276efb928382065e83`, `/home/bl/misofm/engine-454-plan`. One consolidated source verdict against the complete numbered contract and the three finite attempt-2 groups. PASS permits root to integrate delivered #466, freeze the actual candidate and perform retained parent workspace/PR/required-CI qualification. It is not delivery approval or a claim that the older runtime base has already been qualified against current main.

## Final finite corrections

1. The six nonempty producer sites now cover both suppressed-output/error and full-real-output/error: RIFF find, RIFF wc, bypass source, four-root reachability source, FileIdentity count and O_NOFOLLOW count. The full reachability leg is the retained SAME-assertion control, while its empty leg is independently directed. Delegates check expected real status before injecting failure. Find verifies all four actual filenames as an order-independent exact set; wc verifies actual four existing path rows and real count4; bypass and reachability retain complete real numbered allowed output; both count full legs validate actual numeric values before publication. Quiet required reads and genuinely empty forbidden/filtered populations keep their appropriate real0/1 outcomes rather than inventing prohibited output. No extra producer or corpus was added.

2. Actual verifier and late Python read failures are now discriminating. V1 retains valid initial surfaces then removes referenced riff-96000.json, requiring the real FileNotFound diagnostic and python status1. Portability leaves earlier source/contract scans intact and changes only the real Python read argument to a missing path, requiring that actual read diagnostic/status; its controlled exit8 remains separate. Newly introduced missing source/contract cases now require the precise surface failure. Count clean absence1 has a policy diagnostic distinct from execution failure, preserving >=2/exact4 thresholds.

   The committed suites invoke checker executables inside assertions but do not themselves source the functions under a conditional. To verify the explicitly assigned errexit-independent claim without changing source or extending the implementation pass, Astra independently executed the unchanged checker both normally and via `if source CHECKER ROOT MODE; then ...; fi` for the corrected bypass source and O_NOFOLLOW count faults. Each shim first ran the exact real producer, required status0/nonempty valid output, emitted that complete output plus ASTRA_VALID_PAYLOAD_ERROR and exited7. All four checks exited1 with the intended operation/status7 diagnostic and sentinel. Retain `/tmp/astra-454-attempt3-conditional-proof.log` as REVIEWER evidence in final packaging; do not attribute it to Sol or claim it is a committed suite case. No source edit or additional repair was involved.

3. New directed assertions distinguish unexpected checker success97 from wrong/setup outcome96 and correct rejection0. The two existing actual production mutations remain exactly the reachability source-status guard and the late forbidden-status guard; retained diffs confirm those changes. ORIGINAL, mutant and byte-restored checker paths run through the SAME respective assertion. Logs show the intended named unexpected-success branch, the suite requires97, and restored calls pass. The full/empty output controls and real delegate checks avoid attributing setup failure to the intended scan. No third mutation campaign is required or claimed.

## Retained scope and evidence

The only production delta after attempt2 is the two count-status1 diagnostic branches. Previously accepted source fixes remain: bypass source/exclusion checked separately, independent find/wc and count statuses, explicit scratch/Python/required-read failures, checked four-root source and each exclusion. Required paths, exact generator --check, RIFF/RF64 identity, dependency/ABI/regex/glob/comment/Python/count policies, portability-only isolation and both prefixes are preserved. Existing semantic mutation fixtures and valid empty/allowed populations remain intact. No helper, workflow, runner, generator, fixture-source bytes or runtime change.

Independent retained status files report syntax/v1/portability/all/v1-suite/port-suite/diff0 and invalid mode2. Reviewed the two mutant diffs and suite terminal logs. The worktree is clean. No full workspace, Cargo/build, native runner, benchmark or timing was performed by this review; only the four tiny unchanged-source normal/conditional probes above supplemented read-only Git/source/log inspection.

Root must preserve failed-attempt history and this reviewer supplement, integrate actual delivered main after source PASS, then run the retained immutable workspace and actual-head PR/required-CI gates. #403/#306/#349 and sibling outcomes remain open. No further implementation repair is requested.

Root integrated delivered main `6589c5185411d51bce7d0a0aafab4df63a5e47db`; all three accepted scripts remain byte-identical to reviewed `7724a581`. The upcoming immutable workspace command uses `/tmp/engine-454-workspace-qualified`, includes doctests, and compares its population with delivered #430 (275 result blocks, 1,591 passed, zero failed, 24 ignored). No source/evidence edits while it runs. Final actual PR review and required CI remain mandatory.
