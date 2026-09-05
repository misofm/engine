# Prepare current full-chain benchmark identity and a fresh #431 capture adapter

Ready-to-number TOOLING preparation draft; no implementation or timing authority from this report. Parent #431 retains the ONE future controlled capture. Root's adopted fresh-current20-row ruling at431-plan45a84ec8 supersedes historical matched-input comparison: current preparation uses canonical JSON, and older descriptor/PCM pins are not recertified as equal outputs. Preserve all historical035/072 scripts, validators and records.

Read deliveredbb5ed498; its relevant benchmark source, builtins fixtures, canonical session and historical validators/wrappers are unchanged fromaba905c0. The existing dispatcher re-execs the builtins subject without the subject argument, so its internal args-count1 check is compatible with `bench builtins`; do not remove it.

## Closable outcome and exact allowed paths

Deliver a statically validated current workload identity plus a dedicated, hermetically tested zero-launch capture adapter, ready for #431's separately authorized invocation. No runtime/DSP change or measurement in this child.

Allowed: tools/bench/src/builtins.rs (only the stale manifest constant/comment and a bounded untimed current-input regression); NEW scripts/builtins-current-benchmark-record-validator.jq; NEW scripts/builtins-current-benchmark-validator.jq; NEW scripts/preflight-builtins-current-benchmark.sh; NEW scripts/run-builtins-current-benchmark.sh; NEW scripts/test-builtins-current-benchmark.sh; one new suite call in the existing benchmark-mutation step of .github/workflows/qualification.yml; numbered evidence/reciprocal431 record. No existing072 wrapper/test/validator edits, fixtures, Cargo/dependency/profile, general helper API or generic runner framework. The dedicated filename means current #431 authority only, not arbitrary issue/output arguments.

## Exact current identity ruling

Current manifest: b244da45d88d670951205098b7516af20387a141eccb3bf60edb61e8ba57a919. Correct only the current workload guard from ad034... to that actual hash. No seal regeneration.

Current descriptor SHA256 pairs in48k/96k order:
- full_chain_filters:6a1633442678cfdecb2872deacd053e727c47f0bc94039a84b4e950949e195d0 / ac9e825b5051a161ca731b04bd9b9b825bad6484c3a3f911551051e316224fa0 (616bytes each).
- identity_chain:15dfc8b6d918d01a5d6e46417e37a10023d31a85391e8fb2371af0cdc055dd95 / 962bc24d4104cb5a30e3a5aa158a5ca1075cae01f08433d2c7cbe8c1271cd99a (596).
- matrix_ramp:f0d94928bed16804a26befde5eaabd3a8c233afa194a5cdcb259141af78c831b / ef5bf8c4e954c1e497eea997bffeb85fabad69ac6966f2798bd34ce2fa5ced6f (700).
- meter_success_full:ded3579ee8ffbf79d920648a33a7e2f35fa9c9b386e98ef469d583830ef992de / aa1c4d8835753ce290d7abcf1cbf3ffdb98b79a58f0ec6cd0cce6614f5befef9 (768).
- prepare_256_tracks:a1dec8525c20505a9b440e6cf93fa6ffa1144896c889fa3abd94f76224f3e210 / 880faace46cfa2e9f454d625e54206aa752a9947292057a6b58f64224ea13f30 (963).

All ten actual file bytes/lengths match these current manifest rows. Nested references also checked directly frombb5ed498: filters-asymmetric PCM e53eead1da91f80b8c93a730bd1a45629f4efdddf90c3642d38498d29952d1ff; identity-signed-zero PCM6bf5968e626491089468fe9289c4891116c9c5e3f159238cb2bbb3f37fdf6572; matrix-ramp-128 PCM4b302238e21a45301a1faca72b292d92feacde0dd17df7ddc8f9c271bc693fb8; graph-taps PCM508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19; canonical.json a240547d7e57f76a087c7c43cffc2c54944f96e7ac88a1a19158f65a4a0bc77b. Both-rate descriptors reference these same assets. No timing-derived pins.

New record validator is the historical complete validator with precisely the manifest and SIX stale descriptor hashes replaced by the above current values; all four matrix/meter pins stay identical. New aggregate differs only in its included module name. Require a machine-readable exact allowed-delta comparison against historical validators. Preserve schema1, issue35 and issue035 workload/fixture names: they identify the existing workload, while disposition authority is431. No output digest pin is added: existing complete paired output-hash equality remains mandatory, not equality to an unrelated historical capture.

## Workload and adapter

One `bench builtins` invocation, not three executable launches: existing internal warmup and two rounds yield20 records, five workloads × two rates × two rounds. Preserve all constants, operations, parameters and numerical/audit predicates. Eight full-chain/identity rows measure the public seam; matrix/meter/preparation rows have their existing scopes. No live-bank mechanism or causal speedup claim.

Dedicated durable namespace artifacts/issue431-full-chain; reserved files builtins-benchmark.preflight.json, .raw.jsonl, .jsonl, .stderr, .disposition.json and README/manifest evidence. Store the sealed executable outside tracked artifacts under a dedicated target/issue431-prepared directory. No target/issue35 or issue72 dependency, write, relabel or restoration. Preflight and runner accept no workload/output/round overrides; explicit usage errors launch nothing.

Untimed preparation uses an isolated CARGO_TARGET_DIR and `cargo build --locked --release -p bench` with actual repository fat-LTO/codegen-units1/opt-level3 profile and pinned target features, recording full effective environment. Preflight never launches the resulting binary. Bind actual candidate commit/tree, current input/manifest bytes, source/lock, runner/preflight/validator/suite hashes, profile and executable SHA in the prepared record. Runner never rebuilds and refuses all identity drift/old outputs/symlink or overwrite hazards.

Use existing controlled benchmark preconditions unchanged: ceiling0.50, cooldown60seconds, affinity and sibling limit5%/0.2second sample. Reject uncontrolled override. Readiness is not benchmark authority. Runner reserves/persists its sole invocation disposition before any workload launch, including prelaunch refusal; no retry/resume/automatic successor chain. Stream raw stdout/stderr durably, preserve partial output and failure, validate each record and complete20-record aggregate before promotion. Verify exact phase counts workload_started1/warmup_complete1/timed_started1/round1+2 complete1each and actual process return. Record no current capture as executed in this tooling child.

## Finite zero-launch acceptance

The new suite uses a scratch repository and fake cargo/git plus synthetic executable/records, following the existing072 hermetic structure. Prove command resolution stays fake before calling the adapter. All tests must leave real workload launches0, including preflight, malformed args, existing/preexisting partial outputs, binary/source/validator drift, no-overwrite, failing warmup/round, partial stdout, validator/aggregate rejection, interruption and successful synthetic promotion. Original failure and raw preservation must be distinguishable from setup failures. Exercise both complete production validators with all20 synthetic records and focused shape/duplicate/missing/paired-output/audit mutation refusals; no new large corpus or production mutation campaign.

Add an untimed bench unit regression that hashes the actual included manifest and all ten descriptor/nested inputs via the existing parsing helpers without calling main, Instant measurement loops, warmup or preparing the benchmark workload. This catches the exact stale guard defect that prior synthetic suites missed. Verify current descriptor fixed parameters remain unchanged. It must not execute an effect or generate expected pins.

After implementation: shell syntax over the three new scripts; `cargo test --locked -p bench` and strict bench Clippy as applicable; new hermetic suite; exact current/historical validator allowed-delta check; source/prose diff checks. Root records commands/statuses and immutable source. No real preflight/runner invocation or timing during this preparation issue. Full proportional delivery gates, actual pushed-PR Astra review and required qualification follow accepted source; no automatic artifact repin for tools-only changes.

## Closure and sequencing

Astra scopes/reviews; Luna1, Sol2/3 then hard stop; root numbers/synchronizes/checkpoints. Queue this independent preparation after the active tooling delivery boundary; it cannot interfere with #460 qualification or any reserved quiet window. Close only this readiness capability when delivered. #431 remains open with its ONE invocation unspent until separately frozen and root-authorized; #443/#444 and broadRT4 remain open. If a concrete adapter defect requires broader machinery, preserve evidence and rebrief rather than widening this scope or launching to discover failures.

## Numbered preparation scope

GitHub #473 matches this title and body. This is queued tooling preparation; Astra must approve the numbered integrated base before Luna implementation. The sole #431 controlled capture remains unspent and unauthorized by this issue. Latest delivered main bb5ed498 is integrated; runtime/benchmark inputs are unchanged by this planning checkpoint.

## Astra numbered scope approval

# Astra #473 numbered scope/base review

**PASS for queued numbered scope at `b235dc9cb8c22e031be559dc2db7a7e363400330`, engine-473-plan.** The complete adopted draft `/tmp/astra-431-tooling-preparation-brief.md` is retained verbatim, followed only by numbered status. Compared to deliveredbb5ed498, the planning delta is solely431 retention and the new473 spec. No runtime/benchmark/input drift or implementation is present. Root reports exact remote title/body/number synchronization; no independent GitHub mutation/query is claimed here.

The readiness outcome and files are bounded as approved: stale current manifest guard/untimed input regression in bench; dedicated current record/aggregate validators, runner, preflight and fake-only suite; one existing CI-step suite call; numbered evidence. Historical035/072 files remain untouched. The current manifest and six changed descriptor pins are justified by the retained current-input ruling, with four unchanged matrix/meter pins and all original numeric/audit/paired-output acceptance preserved. No timing-derived input/output pin or historical matched-input speedup claim is authorized.

The existing20-row workload, dispatcher/internal argument semantics, one internal warmup/two rounds and release profile remain frozen. Zero-launch preflight/lifecycle proof is synthetic and isolated; no real runner, benchmark workload, actual capture or authority consumption belongs to473. Dedicated431 namespace is not a generic issue adapter. Future431 capture remains a separately frozen and root-authorized ONE controlled invocation, still unspent.

Existing fake executable verification, persistence/status/overwrite/refusal controls, exact validator allowed-delta check and untimed ten-input regression address the actual identified readiness failures. No further scope amendment is needed before root assignment at the approved tooling boundary. If intervening delivered source changes those exact inputs, repeat only the focused base comparison before coding.

Root may assign Luna1 when the current tooling boundary permits; this review itself does not start implementation. #431/#443/#444 and broadRT4 remain open. Actual-head Astra review and required CI remain delivery gates for473. No tests, builds, timing, edits or Git/GitHub mutations were performed.

Root retains this approved issue in the queue until the active tooling delivery boundary. No implementation or capture invocation has started.

## Delivered-base confirmation and Luna1 assignment

PR477 delivered the prior #456 tooling slice at main `fa3485c6bb1a69e6dd01df734a1ad9c945964715`; #456 is closed and unclaimed. Root integrated that delivered base and verified the approved bench source, fixtures, Cargo/configuration, historical validator/runner inputs and qualification workflow byte-identical to the approved473 base. The Astra numbered scope approval therefore remains applicable.

Luna attempt1 is now assigned only this numbered scope in engine-473-plan. Prepare current identity, validators, dedicated adapter/preflight and fake-only tests; do not execute the real preflight, runner, benchmark or timed workload. The sole future #431 capture remains unspent and unauthorized. Pause at each coherent focused-green tranche for root exact-path commit/push before layering more work. Root owns all Git/GitHub operations. Preserve historical035/072 behavior and the exact frozen validator delta.

## Luna1 first coherent checkpoint

The approved bench manifest constant now identifies the current sealed manifest. The existing untimed input regression verifies that constant, every measured descriptor and the nested PCM references through the existing parser. No benchmark preparation, execution, clock loop or timed workload is invoked. `cargo test --locked -p bench` completed with 31 tests passed and status0, retained at `/tmp/473-luna1-tranche1-bench-2.log` with `.log.command`/`.log.status`; the initial failing invocation is preserved separately. This is the first checkpoint within Luna attempt1, not full issue acceptance. Dedicated validators/preflight/runner/fake-only lifecycle and final gates remain.

## Luna1 adapter draft checkpoint — incomplete, not qualification

The five new script/validator paths are preserved as an incomplete attempt for review. Reported shell/JQ syntax statuses and repeated bench31 regression do not demonstrate the adapter product. Root inspection finds the fake-only lifecycle script merely checks syntax and prints a placeholder, the new validators do not follow the frozen exact-copy/hash-delta shape, and the short runner/preflight omit the required full provenance/preflight, controlled single-invocation disposition, durable failure persistence and phase validation. No real runner/preflight/workload was invoked. Root does not credit these stubs as a focused-green product tranche or source acceptance.

Raw draft evidence remains `/tmp/473-luna1-tranche2-*`; jq on null outputting false is syntax-only evidence, not validator acceptance. The useful current-input fix from tranche1 is retained. Astra will give one consolidated Luna1 verdict against the full existing spec before Sol retries; no scope requirement is weakened to accept this draft.

## Astra Luna1 verdict and Sol2 assignment

# Astra #473 Luna attempt1 — FAIL

Exact reviewed head7ffe1e97113603f169c522fe76d8656219b9d0c4 in engine-473-plan. Read full frozen473 spec and cumulative implementation from deliveredfa3485c6. This is one consolidated end-of-attempt verdict. The root record correctly labels the adapter incomplete; syntax checks and31 bench tests do not qualify a capture adapter. No preflight, runner, benchmark, build, test or Git/GitHub mutation executed during this review.

Preserve the useful first tranche: the actual included manifest constant is corrected to b244da45..., and the untimed regression now verifies that constant, all ten descriptor hashes and render PCM nested references using existing parsing. Historical scripts/validators/fixtures and workload arithmetic remain unchanged. The following FIVE groups are the finite original-contract Sol2 correction list, not a new framework or expanded matrix.

## 1. Complete the existing untimed input regression

The new regression explicitly skips prepare inputs after descriptor hashing. It therefore never checks their nested canonical.json bytes against session_template_sha256 without entering the actual preparation path. That hash check currently exists only inside prepare_256_tracks. Complete the original untimed input requirement by reading those existing descriptor fields and hashing SESSION directly, without preparing the workload or rendering. Preserve/check existing fixed descriptor parameters through the current field helpers (including common workload/rate/quantum identity), rather than relying on descriptor self-consistency alone. Retain the current manifest and ten frozen descriptor identities; no generated pins, new corpus or benchmark execution.

## 2. Deliver the exact frozen complete validator copies

The new record module wraps the historical validator and rewrites incoming identity fields to old hashes. This is explicitly not the approved exact-copy-plus-seven-hash-delta contract. The aggregate is also rewritten to call current_record_valid instead of differing only by included module name. Replace both with the prescribed complete historical copies: record changes ONLY current manifest plus SIX stale full-chain/identity/preparation descriptor hashes; all four matrix/meter pins and every numeric/schema/audit predicate unchanged. Aggregate changes ONLY module include. Preserve schema1/issue35/workload names and full paired output identity. Add the required machine-readable exact allowed-delta comparison; no relabelled historical identity validation.

## 3. Implement sealed preparation and identity refusal in the fixed431 namespace

Preflight currently writes a minimal READY record under target/issue431-prepared, not the required durable artifacts/issue431-full-chain namespace. It binds only commit and binary hash, builds relative to the caller CWD, and does not seal tree, current input bytes, source/lock, all adapter/validator/suite hashes or effective release environment. Its clean-tree command substitution can treat a failed Git status with empty stdout as clean. Checking only an existing binary does not protect the directory, parents or all reserved evidence paths from symlink/overwrite hazards.

Use the approved isolated release build and dedicated prepared executable location, anchor commands to the actual repository, explicitly check producer statuses, and bind every frozen identity/profile input with a zero-workload preparation record in the prescribed namespace. No overrides/general issue arguments, existing072 dependencies or historical-file writes. Runner must validate these seals against actual current bytes and refuse binary/source/validator/profile or output drift before launch; presently it only checks that an executable and a seal file exist and never reads the seal.

## 4. Implement controlled one-invocation lifecycle and durable validation/promotion

The15-line runner has no controlled precondition check, no rejection of uncontrolled override, no persisted invocation reservation/disposition, no phase-count validation and no candidate/binary environment binding for the workload's required identities. It can launch repeatedly after failures. The EXIT trap deletes partial stdout; stderr is overwritten; accepted output is the only existing-path guard. mv -n can silently leave an old raw destination in place. These violate original failure preservation and sole-invocation authority, independently of validator correctness.

Implement only the frozen fixed431 lifecycle: reserve/persist the sole invocation BEFORE possible workload launch, retaining prelaunch refusal as consumed; unchanged ceiling0.50/cooldown60/affinity/sibling preconditions and no uncontrolled path; one bench builtins process with existing internal warmup/two rounds; durable raw stdout/stderr and failure/interruption disposition; exact one-of-each frozen phase and actual process status; no retry/resume. Validate every record and the complete20-row aggregate before promoting the original validated JSONL bytes.

Concrete present validation errors: jq is invoked without slurp against raw JSONL, while the aggregate requires an array of20. Its boolean filter stdout is redirected to the supposed accepted JSONL file, so even a successful validation would store a boolean rather than the20 records. Preserve the raw data and distinguish validator/process failure from successful promotion. All reserved paths and partial/preexisting outputs must refuse overwrite without destroying prior evidence.

## 5. Replace the placeholder with the complete existing-style hermetic proof and CI call

The new test script only runs bash -n and prints that an isolated harness is required. It exercises no production adapter path or validator. The required qualification.yml suite call is absent.

Implement the frozen scratch-repository/fake cargo+git/synthetic executable harness, verifying resolution stays fake before invoking adapters. Cover precisely the named cases: zero-launch preflight and bad args; existing/partial outputs, symlink/no-overwrite; binary/source/validator identity drift; controlled refusal; warmup/round failures, partial stdout, interruption, record/aggregate rejection and successful synthetic promotion. Exercise all20 valid synthetic records and the frozen shape/duplicate/missing/paired-output/audit refusals through complete production validators. Assert exact intended refusal/persistence/phase/process identities, not generic nonzero or printed labels. Confirm real workload launches remain0. Add the ONE allowed existing CI-step call. No extra production mutant campaign, helper framework, historical-suite modification or real runner execution.

After the coherent revision, retain exact commands/statuses for the prescribed bench unit/strict Clippy, three-script syntax, hermetic suite, allowed validator delta and source/prose checks. Source acceptance then permits root's remaining proportional delivery/actual-PR/required-CI work only. #431 remains open and its future real invocation unspent; this preparation issue grants no measurement authority. Luna1 is over; root may assign Sol attempt2 against this complete finite list, preserving the useful input checkpoint and candid draft history.

Root assigns one coherent Sol2 pass against these five original groups in the existing exact paths. The useful manifest/input regression is retained; incomplete adapter drafts are not accepted behavior. Pause at meaningful focused-green checkpoints for root commit/push before layering. No real preflight/runner/workload or timing, and no changes to future431 authority. Root owns Git/GitHub and later delivery qualification.

## Sol attempt 2 implementation evidence

The untimed bench regression now validates the common fixed descriptor fields for all ten inputs and directly hashes the canonical session referenced by both preparation descriptors, without preparing or rendering a workload. The two current validators are complete historical copies: the record differs only by the current manifest and six approved full-chain/identity/preparation descriptor hashes, while the aggregate differs only by its module include. The hermetic suite constructs and compares those exact expected copies before exercising the validators.

The dedicated preflight now anchors its isolated locked release build to the repository and publishes a one-link prepared executable under `target/issue431-prepared`. It publishes zero-workload README, input-manifest evidence and a READY seal under `artifacts/issue431-full-chain`. The seal binds commit/tree, Cargo lock and workspace manifest, source, Cargo target-feature configuration, controlled-precondition source, every adapter/validator/suite, all ten descriptors and their nested PCM/session inputs, the prepared executable, and the fixed release profile. Git status and identity producers are checked explicitly. Namespace parents, prepared directories and every reserved evidence path refuse symlink/reuse hazards.

The dedicated runner validates the complete seal and current bytes before launch, rejects the uncontrolled override, enforces the fixed 0.50 load ceiling, 60-second cooldown, affinity and 5%/0.2-second sibling controls, and reserves its sole invocation disposition before any refusal or possible launch. It invokes `bench builtins` once with candidate/binary identities, preserves raw stdout and stderr plus partial failures/interruption, requires exactly one of every frozen phase and the actual process status, validates every record and the slurped 20-record aggregate, then promotes an independent copy of the original JSONL bytes. Existing, partial and symlink outputs refuse overwrite, and every outcome consumes the fixed authority without retry or resume.

The new suite uses a scratch repository with resolved fake cargo/git/taskset/sleep/cat and a synthetic executable. It proves zero-launch preflight and refusals; existing/symlink output preservation; binary/source/validator drift; controlled and uncontrolled refusal; warmup/round failures with partial raw output; interruption; record, aggregate and phase rejection; complete 20-record validation/promotion; byte-identical raw/accepted output; and consumed-authority refusal. It retains focused shape, missing, duplicate, paired-output and audit validator mutations. The qualification benchmark-mutation step calls the suite once. All synthetic lifecycle cases report zero real workload launches; no real preflight, runner, benchmark, audio, or timed workload was executed. The sole future #431 capture remains UNSPENT and UNAUTHORIZED.

Focused gate evidence from this coherent checkpoint:

- `/tmp/473-sol2-bench-test.{command,log,status}`: status 0, 31 tests passed.
- `/tmp/473-sol2-bench-clippy.{command,log,status}`: status 0 with `-D warnings`.
- `/tmp/473-sol2-shell-syntax.{command,log,status}`: status 0 for all three adapter scripts.
- `/tmp/473-sol2-hermetic.{command,log,status}`: status 0 and `current builtins benchmark validators/lifecycle: PASS (real workload launches: 0)`.
- `/tmp/473-sol2-validator-delta.{command,log,status}`: status 0 for the exact seven-hash record copy and one-include aggregate copy comparison.
- `/tmp/473-sol2-source-prose.{command,log,status}`: status 0 for diff hygiene, exact authorized paths, historical 035/072 and input/config byte preservation, and explicit #431 `UNSPENT`/`UNAUTHORIZED` state.

## Astra Sol2 verdict and final Sol3 assignment

# Astra #473 Sol attempt2 — FAIL

Exact reviewed a7aea9229b84f5356b4bb09d8bb50beb895aebb7, engine-473-plan. Full473 contract and prior five-group review examined. No real/synthetic preflight, runner, workload, benchmark, tests or builds executed during review; findings below are source-based. This is one consolidated verdict, preserving successful portions and limiting final Sol3 to the following four original-contract groups.

Accepted progress: actual manifest/nested canonical input/common-field regression now covers the missing preparation seam without rendering. Independently compared validator text: exactly seven hash occurrences change, all non-hash text identical; aggregate differs ONLY in included module. Preserve these copies and historical files. The fixed namespace, raw JSONL preservation/promotion, checked Git operations, input hashes, unchanged controlled predicates, phase counts and real fake-only harness are substantial implementations. Reported six focused statuses0 and114-name environment check do not settle the remaining defects.

## 1. Seal the ACTUAL build environment and attribution, not profile literals

Preflight runs cargo with arbitrary inherited RUSTFLAGS/CARGO_ENCODED_RUSTFLAGS, Cargo profile/target overrides and wrapper/toolchain environment. It then unconditionally writes target_features="+avx2,+fma", opt_level="3", lto="fat", codegen_units=1. These values are not observed or enforced. For example CARGO_PROFILE_RELEASE_LTO=false changes the build while the seal still asserts fat; RUSTFLAGS can override repository target configuration. Runner only compares these constants with the seal. This fails the explicit full effective-environment/profile and drift contract.

Freeze/enforce the already approved repository profile/target settings and record relevant effective environment/toolchain/build provenance. Refuse incompatible inherited overrides rather than silently certify them or change the workload profile; no new generic config framework. Bind runner execution metadata to recorded observations. Currently only candidate/binary variables are supplied to bench; the other Metadata::collect environment fields can be missing or caller-supplied independently of the asserted seal. Missing metadata must stay candid under the preserved validator, but no field may pretend to be a verified controlled/build fact. Add finite fake-only incompatible-profile/target-environment refusal and matching provenance checks to the current harness.

## 2. Enforce sole reservation ownership and physical namespace throughout the runner

The runner installs its EXIT trap BEFORE atomically creating its reservation. If two callers both pass the initial absence check, the losing noclobber write fails, but its EXIT trap still calls publish_disposition and mv -f over the winning caller's disposition. That corrupts the only-authority evidence even though noclobber prevents a second initial reservation. Track successful reservation ownership; a caller that did not acquire it must never finalize/replace another invocation's disposition. Preserve a successful reservation on all subsequent refusal paths.

Runner checks the final artifact directory but not its artifacts parent, nor the prepared directory/target parent. A previously sealed namespace can be relocated behind a parent symlink and still pass individual !-L file checks. Preflight's parent checks do not protect later runner use. Refuse these fixed parent/prepared namespace aliases before writes/launch; no recursive filesystem security framework required. Retain all existing output guards. Add narrow fake-only competing-reservation/loser-does-not-write proof and post-preparation parent/prepared-directory symlink refusal, preserving prior evidence bytes and zero launches.

## 3. Keep process termination and failed persistence truthful

process_status initially captures the actual child status but is overwritten to1 after successful workload execution when phase/record/aggregate/promotion checks fail. The disposition therefore falsely reports a workload process failure for a process that returned0. Preserve the actual workload return separately from validation/promotion/runner status, with null/not-started before launch. Interrupt handling hardcodes130 for both INT/TERM and exits without an explicit child termination/reap protocol. The synthetic interruption fixture kills the parent and immediately exits itself, so it cannot prove preservation while a workload remains active.

Use bounded explicit child lifecycle ownership for the ONE process: on interruption stop/reap that child before publishing final hashes/counts; retain actual wait status and signal/refusal reason. Do not permit a child to continue mutating raw/stderr after final disposition. Keep no-retry authority and durable partial output. Check disposition publication failures so an unavailable final write cannot turn into successful capture reporting; preserving RUNNING reservation on a failed final publication is safer than erasing it. No new runner framework.

## 4. Finish the original failure-path proof, not a larger matrix

Current run_failure_case only requires nonzero and disposition status/reason. Except the warmup case, it does not verify exact child status, retained raw/stderr bytes, absent accepted output or one actual synthetic launch. Round1/2 produce five/fifteen rows but those partial populations are never asserted. Interruption emits no partial payload and self-exits, as above. Several original preservation/phase requirements can therefore regress while the suite stays green.

For the existing warmup/round1/round2/record/aggregate/phase/interruption cases, assert exact emitted raw/stderr identities or complete expected bounded payload, phase counts, actual child return, no accepted promotion, one synthetic launch and a second invocation's refusal without changing those artifacts. Use one bounded blocking synthetic child for genuine parent interruption and verify termination/reap plus partial-output preservation. Keep successful20-row promotion byte equality and existing production-validator mutation table unchanged. Distinguish intended lifecycle failures from harness/setup failures. Exercise a final-disposition/promotion persistence refusal through the existing fake harness; do not introduce a production mutation campaign or unrelated cases.

The existing nine allowed paths remain sufficient. No validator/numeric/pin/workload changes, historical072 rewrite, new helper/allocator/framework or timing are authorized. Preserve all current successful evidence and candid failures. Root may assign final Sol3 once this finite list is synchronized; another FAIL hardstops/rescopes rather than a fourth correction. Source PASS and real #431 capture remain ungranted.

Root assigns the final Sol3 implementation pass against these four remaining original groups. Preserve accepted input/validator work and all raw failed/green evidence. Pause at meaningful coherent checkpoints for root exact-path commit/push. No real workload, preflight or capture execution and no timing; future431 remains unspent. A further final FAIL requires explicit rescope, not a fourth repair.
