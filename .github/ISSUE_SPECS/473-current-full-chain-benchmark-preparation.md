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
