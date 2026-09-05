# Complete benchmark ownership and production dependency scans

Parent #403, audit #349 TOOL-11 and #306. Queued scope; no implementation authority until numbered current-base Astra approval and root assignment. Planning base: delivered main `3faf89adea25e32e85a27d744c643a79cd80ce31`; its gate source is identical to the inspected `5a4a7d2071194cf6118241e24d073824668e3387`.

## Frozen bounded outcome

Complete benchmark-harness ownership and production dependency scans

Files: check-bench-policy.sh, check-realtime-audit-leak.sh; existing test-bench-policy.sh and test-realtime-audit-leak.sh. This owns EXACTLY the three original #306 loops assigned to #403: bench's one manifest loop and audit-leak's structural and resolution loops. Their shared population/production-harness boundary makes this one useful outcome.

Bench's original scans are grep, not rg: sole_owner, shared escaper presence, escaper candidate discovery plus per-candidate awk, forbidden private SHA patterns, each timed subject's required timer and forbidden clock/digest, exact unsafe-owner set, exact metadata-reader set; then find/sort/awk manifests and final count formatting. Preserve ERE/include semantics, the exact escaper indentation/40-line/comment/backslash algorithm, exact owner sets and exemptions. Grep0 means match,1 clean absence, other statuses failure; no quiet positive scan that hides completion failure. Capture every producer before sort/filter/comparison; check awk even when its valid result is empty/delegate. A failed delegate parser currently can be interpreted as no offender. Exact-owner scans must contain their owner; forbidden scans may be empty; dependency violation output may be empty. Preserve legal duplicate-free set semantics without adding a new Rust parser.

Manifest discovery retains `find crates hosts sidecars -mindepth2 -maxdepth2 -name Cargo.toml` and sort. All three roots must exist; aggregate required nonempty, individually empty roots allowed. Each parser uses its existing section grammar. bench permits crate dev dependencies but bans host/sidecar occurrences as before. Audit-leak permits dev sections plus the existing engine/conformance feature declarations; preserve exact exceptions. Its second loop extracts each package name (required nonempty) and executes EXACT `cargo tree --locked --offline -p NAME -e features,no-dev --target all`. Capture Cargo stdout/stderr/status separately before complete grep. Cargo failure with a perfectly clean-looking graph must fail; grep1 is clean,0 violation,other failure. Cargo tree is non-timed metadata resolution, not a build or benchmark. Do not hide its stderr or accept an empty completed graph as proving a named package resolved; require nonempty Cargo graph output.

Existing bench fixture copies tools but creates only empty crates/hosts and omits sidecars: correct the fixture with all roots and one actual valid package manifest. Do not relax production roots. Audit suite currently copies the workspace and runs real offline Cargo tree; retain that real non-timed positive/old mutations. For directed failures, add a tiny valid manifest fixture and a selective Cargo shim returning faithful named-package output, with earlier operations real/delegated. No replacement resolver.

Finite cases: clean existing tree; valid delegate and empty violations; each missing root, empty aggregate, named late manifest read; grep empty/error and real matching output/error, sort complete-list/error, awk empty/error and delegate/error, clean Cargo output/error, and grep error after successful Cargo. Selective targeting must reach later subjects/packages. Two actual uniquely verified production status-loss controls: complete failed manifest discovery and clean-looking failed Cargo graph. Run the SAME original error assertion on each mutant, require distinguished unexpected-success outcome; unrelated setup/error diagnostic failures must not satisfy it.


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
