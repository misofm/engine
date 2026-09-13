# Discover and query native response analyses through owner-provided descriptors

Parent: #763. Required source dependencies: #764 (prepared parametric-EQ response) and #767 (input HPF/LPF response). Begin implementation only after #767's coherent source slice has passed review and root has checkpointed it. This brief was prepared read-only against `/tmp/miso-engine-767` while #767 was being implemented; it does not prescribe or depend on unfinished private implementation details.

## Smallest closable product outcome

A native embedding discovers whether a registered effect supplies a requested-configuration magnitude response, obtains authoritative shape/units/section metadata, prepares that response through the effect's factory, and queries it into caller-owned buffers without naming a concrete effect crate. Builtin HPF/LPF supplies the same response interface through its own owner. A consumer can render the two kinds of already implemented response with one result/buffer contract and explicit unsupported capability handling.

This is a narrow native response-provider precursor, not a generic analysis service. It adds no session-bound handles, graph composition, generated/browser/SDK analysis entry point, applied-target snapshot, subscription, FFT/capture, transport, or join. Those remain required children of #763. The next session-binding child can use existing stable effect addresses and prepared requests; the next metadata child can enumerate these same owner declarations. This issue must not become either successor during implementation.

Astra (`gpt-6-astra`, xhigh) approves this scope and gates. Luna max implements; Astra medium supplies one adversarial verdict per coherent attempt; maximum five attempts. Root owns issue numbering/synchronization, checkpoint commits and delivery. No concurrent implementation tranche or repository edits by the briefing agent.

## Verified source seams and decisions

1. `crates/effect-contract/src/lib.rs::NativeEffectFactory` already owns effect preparation. `NativeEffectRegistry` stores factories in stable EffectId order, supplies `get/get_ascii`, and enumerates `descriptors()`. An additive default factory method is the correct native discovery seam.
2. **Do not add a field to `EffectDescriptor` or repurpose `ObservationDescriptor`.** The former participates in existing serialized descriptor/CID/C-ABI contracts; the latter currently describes scalar resident observations and explicitly refuses computed binding. Keep these bytes and validators unchanged. A companion native response descriptor belongs to the same effect owner through its factory, without pretending it is already in the old wire menu.
3. `crates/parametric-eq/src/response.rs::EqResponseConfiguration` already owns immutable rounded words and bypass/enables. Its preparation may allocate transiently through the existing validator; its query is allocation-free. Adapt this implementation rather than recomputing coefficients or transfer math.
4. #767's builtin query is owner-local and validates full `BuiltinParameters`, exact cutoff bits/rates and HPF<LPF ordering. Its common-interface adapter must call that accepted implementation and preserve its filter-only meaning.
5. `crates/effect-compiler/src/prepare.rs::EffectPreparedEntry` already retains `factory: Arc<dyn NativeEffectFactory>` and `bank_preparation: EffectBankPreparation`; `.request()` reconstructs the exact accepted request. A later session provider can prepare response snapshots from these control-side objects before runtime ownership transfer. It must not read the mutable render processor.
6. `crates/host-core` intentionally has no concrete-effect production dependencies. Its optional `control-provider` feature is protocol-specific and allowed only for capi. The later analysis provider should use the owner contract independently of that feature, not add parametric-EQ or protocol dependencies to browser hosts.
7. `tools/parameter-metadata/src/lib.rs` already iterates `registry.descriptors()`, and can later use `registry.get(descriptor.id).response_analysis()` to emit authoritative response rows. Builtin rows can read the builtin owner declaration. Do not emit a host-maintained list of analyzable effect IDs. Graph signal availability must later be declared by graph/tap owners, not inferred from effect response support; this child advertises no graph signal/spectrum capability.

## Concrete native contract

Add a small `crates/effect-contract/src/response.rs` module with response-specific types. Internal Rust names remain unversioned. The interface is native-only in this issue; reserve no wire layout, C symbol or schema tag, and do not advertise browser execution availability from these Rust descriptors.

Suggested API shape (equivalent spellings permitted; keep this scope and behavior):

```rust
pub struct ResponseAnalysisDescriptor {
    pub id: u32,                         // nonzero, owner-local; 1 for each present owner
    pub name: &'static str,
    pub sections: &'static [ResponseSectionDescriptor],
    pub unit: ParameterUnit,             // Db
    pub cost: ObservationCost,           // Computed
    pub floor_db: f32,
    // Typed declarations: frequency axis in Hz; amplitude relative to unity;
    // independent L/R; requested-configuration query; explicit query cadence;
    // optional section curves; total meaning and bypass behavior below.
}

pub struct ResponseSectionDescriptor {
    pub id: u32,                         // nonzero owner-local stable section identity
    pub name: &'static str,
}

pub struct ResponsePrepareLimits {
    pub maximum_prepared_bytes: usize,
}

pub trait NativeEffectResponseFactory: Send + Sync {
    fn analysis_descriptor(&self) -> &'static ResponseAnalysisDescriptor;
    fn prepare_response(
        &self,
        request: PrepareEffectRequest<'_>,
        limits: ResponsePrepareLimits,
    ) -> Result<Box<dyn PreparedResponseAnalysis>, ResponseAnalysisError>;
}

// Add this default to NativeEffectFactory; existing non-provider effects need no edits.
fn response_analysis(&self) -> Option<&dyn NativeEffectResponseFactory> { None }

pub trait PreparedResponseAnalysis: Send + Sync {
    fn analysis_descriptor(&self) -> &'static ResponseAnalysisDescriptor;
    fn configuration(&self) -> ResponseConfigurationView<'_>;
    fn retained_bytes(&self) -> usize;
    fn query_into(
        &self,
        query: ResponseQuery<'_>,
        output: ResponseOutput<'_>,
    ) -> Result<ResponseSummary, ResponseAnalysisError>;
}
```

`ResponseQuery` holds the opaque `u64 configuration_id`, immutable frequency slice and caller `maximum_points`. `ResponseOutput` has the existing four caller slices: L/R totals plus independently optional L/R section-major curves. `ResponseSummary` carries configuration id, requested-configuration mode, validated sample rate, point count and floor; static descriptor and immutable configuration information remain available from the same provider. `ResponseConfigurationView` borrows per-section enabled slices and contains sample rate and `bypass: Option<bool>` (`Some` for EQ, `None` for builtin filters). These are views bounded by the immutable provider lifetime, not owned result arrays; document that explicitly. No mutable fields/setters, raw DSP state, borrowed caller-parameter storage, timestamp or plan identity is exposed.

The descriptor must declare, through small typed vocabularies rather than opaque prose:

- frequency axis unit Hertz and output amplitude-decibel reference to unity;
- independent L/R and computed cost;
- only `RequestedConfiguration` mode and on-explicit-query cadence (no stream/cadence-rate claim);
- total and optional section outputs, section order/membership and finite output floor;
- total scope: complete parametric-EQ cascade versus builtin HPF/LPF subtotal;
- bypass semantics: EQ bypass makes total identity while configured section curves remain available; builtin has no effect-wide bypass.

Do not predesign spectrum, phase, dynamics transfer, arbitrary channel modes, remote messages or subscriptions into this descriptor. Future generic discovery can discriminate response versus signal analyses, each with its own descriptor authority. Empty `response_analysis()` means unsupported response capability, never a silent/flat response.

Use one small typed error vocabulary for unsupported mode/capability where relevant, invalid grid, capacity, output shape, numerical failure, and preparation configuration rejection preserving the original stable error code. Do not convert an error into empty vectors or zero dB. The factory method returning `None` is the capability absence; supported preparation failure is `Err`, not another silent `None`.

Validate response descriptors during native registry construction when a factory supplies one: nonzero IDs, strictly increasing unique section IDs, nonempty bounded static declarations, finite negative floor, and the actual implemented mode/unit/cost/shape semantics. Use a small allocation-free validator; do not change existing `validate_descriptor`, factory descriptor bytes or diagnostic sets. A bad companion declaration must not enter an otherwise valid registry. Builtin declaration validation is covered by its owner test. Do not add a second registry or a global mutable cache.

## Owner adapters and resource behavior

Parametric EQ implements `NativeEffectResponseFactory` on its existing factory and exposes it through the additive method. Its descriptor declares four ordered bands (section IDs 1–4) and uses the same owner floor constant as its evaluator. Wrap or implement `PreparedResponseAnalysis` for the existing immutable configuration inside its owner module, where private fields are available. The common query delegates to existing `query_response_into` and translates only input/output/summary/error shapes. No second numerical implementation or changed calculation order.

Builtins exposes its owner-local descriptor (HPF then LPF, IDs 1–2) and an off-render preparation function returning a `Box<dyn PreparedResponseAnalysis>` for valid sample rate and `BuiltinParameters`. Its adapter may retain the accepted fixed-size configuration and enable flags and delegate to #767's public query; it must preserve all original validation and filter-only semantics. If the accepted #767 implementation already has an immutable prepared representation, reuse it. No artificial `NativeEffectFactory` registration for fixed builtins and no fabricated effect parameter schema.

Before allocating an adapter, compare its actual fixed concrete size with `maximum_prepared_bytes`, using checked/representable arithmetic. Reject insufficient capacity with no returned provider. `retained_bytes` reports the actual concrete heap payload; document that the caller's Box handle and later provider catalog overhead are separate. Existing configuration validation may allocate transiently off render; this does not permit an unbounded cache or retained request vectors. Provider object contains only fixed-size owned data and immutable references. Query and reading descriptor/configuration/retained bytes allocate/free zero heap bytes; last Box drop occurs off render. Query preserves the native owner function's exact frequency, shape, capacity and unchanged-output refusal contract.

Do not compose public floored curves. Future linear subtotal composition must request owner-provided unfloored magnitude/complex values or an owner-preserving accumulation primitive, then floor only once after the whole composition. This child exposes single-owner curves and explicit section membership only; it neither claims cross-owner composition nor locks the future composer into summing floored f32 output. That primitive, stable session membership/order and nonlinear-chain labeling belong to the required composition child.

## Allowed paths and focused gates

Allowed production paths:

- new `crates/effect-contract/src/response.rs`, module exports plus the default factory seam/registry validation in `src/lib.rs`;
- `crates/parametric-eq/src/response.rs` and its existing factory implementation in `src/lib.rs`;
- #767's response module and exports in `crates/builtins/src/lib.rs`.

Tests: focused response-descriptor tests in `crates/effect-contract/tests/response_analysis.rs`; owner adapter tests next to each existing response API test; one registry-dispatch integration test `crates/effect-compiler/tests/response_analysis.rs` using existing launch registry. A tiny Rust native usage example is optional. No Cargo production dependency changes, host-core/graph/protocol/render/kernel edits, generated JSON/TS/ABI changes, new unsafe code or new test framework. Root-owned numbered spec/evidence is allowed. Any artifact mismatch at delivery uses the already established bounded #766 workflow; no speculative artifact changes before source PASS.

Required discriminating tests:

1. Real launch registry finds EQ's owner descriptor/provider without a concrete-EQ import in the integration test; a nonprovider effect returns `None`. Invalid companion descriptors fail registry admission. Existing effect descriptor serialization/CID fixtures remain unchanged.
2. Common-interface EQ and builtin outputs are **bit-identical** to their accepted direct APIs for asymmetric L/R, mixed enabled/disabled sections, total-only and independently selected sections, bypass where applicable, DC/Nyquist and all four launch rates. Descriptor shape/floor/section order/configuration view matches actual output. This is adapter validation: reuse direct APIs as the adapter oracle and retain the existing independent DSP oracle tests unchanged.
3. Preparation errors keep original stable codes and no provider; resource bound just below/at actual prepared size discriminates. Query invalid grid/shape/budget errors preserve every output sentinel. Correlation identities above 2^53 and max u64 survive.
4. Mutating the caller's original preparation inputs after provider creation cannot affect later results. Generic trait-object queries allocate/free/reallocate zero bytes from their first call, measured through existing audited allocator after preparation. Config/descriptor views remain valid only for their immutable provider lifetime.
5. Native factory/render conformance remains unchanged, and no PreparedNativeEffect/Bank render trait or state layout is altered.

Proportional commands after the coherent implementation:

```
cargo test --locked -p effect-contract --test response_analysis
cargo test --locked -p effect-compiler --test response_analysis
cargo test --locked -p parametric-eq --test response --test conformance
cargo test --locked -p builtins --test filter_response --test determinism
cargo test --locked -p effect-contract --lib --tests
cargo check --locked --workspace --all-targets
cargo clippy --locked -p effect-contract -p parametric-eq -p builtins -p effect-compiler --all-targets -- -D warnings
cargo fmt --all -- --check
bash scripts/check-effect-runtime-policy.sh
bash scripts/check-builtins-policy.sh
bash scripts/check-realtime-policy.sh
CARGO_TARGET_DIR=target/response-provider-wasm-scalar RUSTFLAGS='-C target-feature=-simd128' cargo check --locked --release --target wasm32-unknown-unknown -p effect-contract -p parametric-eq -p builtins -p effect-compiler
CARGO_TARGET_DIR=target/response-provider-wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --release --target wasm32-unknown-unknown -p effect-contract -p parametric-eq -p builtins -p effect-compiler
```

If #767's accepted focused test filename differs, root substitutes the verified exact filename before assigning implementation. No benchmark or browser-functionality claim follows. Preserve command results; broaden tests only for a concrete failure or invalidated assumption.

## Required immediate successor and parent retention

Next, brief the session-bound requested-configuration provider: stable `(track ID,rack,effect slot ID)` bindings, explicit input-filter target, transactional plan-scoped catalog/resource admission, engine-owned validated grids, and explicit requested-configuration identity. Use retained factory/request authorities, never array position or the render plan. A bounded composition child adds ordered declared EQ/filter membership and unfloored accumulation. The generated metadata/SDK child emits these same owner descriptors and exposes browser/headless query parity without client DSP.

Graph signal capabilities must be authored in graph/tap code when actual selected capture is implemented; response presence never implies a spectrum/pre/post tap. All live envelope/subscription/sharing/lifecycle, u64 sample correlation, applied snapshots, spectrum Worker/capture/transport, packed vectors, joins, and integrated #763 evidence remain required. This precursor closes none of those unfinished outcomes.

Root commits each coherent tranche promptly, records Astra medium PASS/FAIL against exact source and gates, completes current artifact/required-CI delivery as applicable, synchronizes this child remotely before closure, and leaves #763 open.

## Numbered source boundary

#767 source passed Astra medium attempt 2 at `dc070bcda506896fe05b4dfefb931841b6d6caa3`; #768 artifact qualification passed attempt 1 at `b92e72160b34329a354244a3d1edfa6441fc149b`. This child starts from their frozen delivery head `53f20959` while PR #769 completes required CI; the accepted response source stays fixed on that branch. #770 is the sole active feature implementation in its separate checkout. Actual builtin test filename is `filter_response.rs` as specified. Native EQ #764 and artifact #766 are already merged/closed.

## Attempt 1 focused checkpoint

Luna max implemented the additive owner response descriptor/provider interface, native factory discovery/registry companion validation, and thin EQ/builtin adapters. Existing descriptor wire contracts and numerical evaluators remain outside the change. Focused owner/contract/registry tests report EQ13, builtins11, effect-contract2, and effect-compiler1 passing. Targeted four-crate all-target Clippy and workspace formatting pass after implementation debugging; raw logs/exits are `/tmp/issue770-focused-{eq-response-final-2,builtins-filter-final,contract-final,compiler-final,clippy-final-3,fmt-final-2}.{log,exit}`. Root checked the exact allowed-path diff and `git diff --check`; no production dependency/ABI/generated asset changes are present. Remaining full/workspace/policy/target gates and the attempt's adversarial verdict are pending. This tranche pauses for root checkpoint before further work.

## Attempt 1 remaining gate evidence

After root checkpoint `c864ffa263a84be627945ff7290446080a18b6a4`, all remaining specified commands
exited 0. Raw stdout/stderr and exit status are preserved outside the repository:

- `cargo test --locked -p parametric-eq --test response --test conformance` — `/tmp/issue770-gate-eq-response-conformance.log` and `.exit` (`0`)
- `cargo test --locked -p builtins --test filter_response --test determinism` — `/tmp/issue770-gate-builtins-response-determinism.log` and `.exit` (`0`)
- `cargo test --locked -p effect-contract --lib --tests` — `/tmp/issue770-gate-effect-contract-full.log` and `.exit` (`0`)
- `cargo check --locked --workspace --all-targets` — `/tmp/issue770-gate-workspace-check.log` and `.exit` (`0`)
- `bash scripts/check-effect-runtime-policy.sh` — `/tmp/issue770-gate-effect-runtime-policy.log` and `.exit` (`0`)
- `bash scripts/check-builtins-policy.sh` — `/tmp/issue770-gate-builtins-policy.log` and `.exit` (`0`)
- `bash scripts/check-realtime-policy.sh` — `/tmp/issue770-gate-realtime-policy.log` and `.exit` (`0`)
- scalar Wasm check — `/tmp/issue770-gate-wasm-scalar.log` and `.exit` (`0`)
- SIMD Wasm check — `/tmp/issue770-gate-wasm-simd.log` and `.exit` (`0`)

No source, test, dependency, or build-input edits were made after the focused checkpoint.

# Issue #770 attempt 1 adversarial review

Verdict: **FAIL**.

Reviewed frozen head: `c864ffa263a84be627945ff7290446080a18b6a4` in `/tmp/miso-engine-770` (feature checkpoint `413794b6`, followed by ancestry-only main merge). Reviewer: Astra, medium. Read the full #770 stateless brief, allowed diff against delivered main, complete companion contract, registry seam, owner adapter implementations, focused tests, relevant existing builtin error authority and supplied gates. No source edits, agents, GitHub actions, commits, or repeated expensive gates were performed.

## Findings and minimal corrections

1. **Contradictory companion semantics are admitted.** `crates/effect-contract/src/response.rs::validate_response_analysis_descriptor` checks axis/unit/cost and the single-variant enums, but never checks the `total_scope`/`bypass` combination. An otherwise valid `BuiltinInputFilterSubtotal` declaration with `EffectWideIdentityWithSections`, or `ParametricEqCascade` with `NoEffectBypass`, returns `Ok(())` and enters the registry, although neither is the implemented contract frozen by this brief. Add validation for the two supported scope/bypass combinations. The contract tests currently exercise only `id == 0`; BAD_RESPONSE's disordered sections are masked by that earlier failure, and the test named allocation-free contains no measurement. Add independent valid-base mutations for section ID/order, bounds/names/floor, axis/unit/cost and the contradictory pairings, and verify malformed companion admission is rejected. Validate the actual builtin declaration in its owner test as explicitly required. This remains a small companion validator/test change; do not change legacy descriptor validation or its diagnostic set.

2. **The builtin adapter replaces established preparation diagnostic spellings.** `crates/builtins/src/filter_response.rs::builtin_configuration_error` invents strings such as `effect.builtin.filter_cutoff`, `effect.builtin.gain` and `effect.builtin.matrix`. The existing translation in `crates/builtins-compiler/src/lib.rs:4426`–4431 is `builtin.gain.domain`, `builtin.filter.cutoff`, `builtin.filter.order`, `builtin.filter.coefficients`, `builtin.matrix.coefficient` and `builtin.matrix.smoothing`. The native builtin owner itself returns the enum and has no pre-existing static-string method; the cited compiler mapping is the existing public diagnostic spelling authority, not another response API. Root confirmed these six existing spellings must be preserved within the new owner adapter under the brief's original-code requirement. Use them directly in the allowed adapter and assert representative exact gain/cutoff/matrix codes, plus exact forwarded EQ preparation codes instead of only matching `Configuration(_)`. No compiler dependency, generic error refactor or compiler edit is needed. Render-only enum variants are unreachable from response preparation; if retaining their arms, the existing compiler fallback is `builtin.resource.arithmetic_overflow`, rather than newly invented strings. Unsupported query rate is already explicitly mapped/tested as `effect.quality.unsupported` and is separate from these six mappings.

3. **Several explicit adapter-only evidence claims are not discriminated.** Correct these together in the current owner test files:

   - `common_eq_adapter_preserves_input_ownership_capacity_and_refusal_sentinels` mutates `values[2]`, then only makes refused queries; there is no successful before/after equality assertion. The builtin adapter has no source-mutation check. Query both immutable providers successfully before/after mutating valid caller configuration fields and compare the results and configuration views. The existing direct-owner mutation test cannot test a newly added adapter's ownership.
   - Both new adapter parity corpora enable every section. Add an asymmetric mixed enabled/disabled configuration at the four launch rates and assert exact enabled-mask contents against descriptor order, floor/scope/bypass metadata and output. Current EQ views mostly assert rate/bypass; builtin only checks one enabled-slice length. Include builtin right-only optional output (currently absent) and EQ's normal left-only case alongside its existing bypass-left-only case.
   - The brief requires bit identity, but parity assertions use float `assert_eq!`, which does not distinguish signed zeros. Compare `to_bits` (existing builtin `bits` helper is available), especially disabled-section/total identities and one-sided modes. Preserve the old independent DSP tests and thresholds.
   - Common-interface refusal tests should retain both optional output arrays when checking grid/budget errors, and cover malformed right-section output as well as left. Builtin common refusal tests currently supply no section output at all. Measure representative refused trait-object queries, and descriptor/configuration/retained-byte reads, with the existing allocator alongside the first-success query measurement. This proves the new wrapper paths, not merely the unchanged direct APIs. Exercise `2^53+1` as well as the already checked maximum correlation identity through the common interface.

The source implementation already uses owned fixed data and correct delegation; finding 3 concerns the frozen adapter acceptance evidence, not a demonstrated current DSP or ownership defect. Reuse current small fixtures/helpers without expanding the DSP corpus or adding a framework.

## Verified positive source conclusions

- The additive default `NativeEffectFactory::response_analysis` seam and launch-registry integration expose EQ without concrete-EQ imports in the integration test; a compressor explicitly returns no capability. No field was added to EffectDescriptor or ObservationDescriptor, and no existing descriptor serialization/digest routine or render/state trait layout changed.
- Both adapters delegate directly to their accepted owner query implementations, translating only shapes and metadata/errors. No coefficient math, transfer arithmetic, flooring order, render path or PCM expectation changed.
- Concrete `size_of::<PreparedEqResponse>()`/`size_of::<PreparedInputFilterResponse>()` checks precede allocation (and preparation validation), and returned retained bytes are `size_of::<Self>()`. Fixed owned payloads have no retained caller slice or vector. The documented accounting excludes the caller's Box handle/catalog; at-size and one-byte-below tests pass.
- EQ wraps its existing immutable prepared words; builtin retains a copied fixed configuration and enable flags, using existing complete owner validation and query. Read-only views borrow their provider lifetime. No mutable state or cache is introduced, and no production dependency is added.
- Query refusal translations otherwise remain typed; first trait-object queries measure zero allocation/free/reallocation. Owners retain finite output, input/grid/shape admission and transactional preflight behavior.

## Gate evidence

Focused supplied results report EQ response 13, builtin filter response 11, companion contract 2, and registry integration 1 passing. The final targeted Clippy and format exit records are zero. I also read the remaining saved zero exit records and terminal results under `/tmp/issue770-gate-{eq-response-conformance,builtins-response-determinism,effect-contract-full,workspace-check,effect-runtime-policy,builtins-policy,realtime-policy,wasm-scalar,wasm-simd}.{log,exit}`: conformance/determinism/full-contract tests, workspace all-target check, policies and both Wasm builds pass. Earlier implementation-debugging failures remain separate preserved logs; they are not the frozen gate result. No reviewer rerun is claimed, and Wasm builds do not establish runtime response parity.

This is the one coherent attempt-1 FAIL verdict. The corrections stay within existing allowed companion/owner/test/spec paths, with unchanged DSP and no generic architecture expansion. Artifact delivery remains a separate successor after SOURCE PASS; parent #763 remains open.

Root authorizes attempt 2 for the three bounded findings, within the existing allowed paths. Preserve legacy descriptors, DSP arithmetic, dependencies and gates. Use the existing builtin diagnostic spellings directly in the owner adapter; no compiler refactor. Complete all adapter assertions in one coherent pass, then focused/Clippy/policy green checkpoint and a single adversarial verdict.

## Attempt 2 focused correction checkpoint

Luna max corrected companion scope/bypass validation and mapped builtin preparation errors to the established diagnostic spellings. Adapter tests now exercise independent descriptor mutations/admission, successful source-mutation stability, mixed enable masks/views, bitwise output identity/one-sided modes, and common-interface refusal/accessor allocation behavior. Production changes are limited to the companion validator and builtin error translation; DSP arithmetic, dependencies, legacy descriptors and generated assets are unchanged.

Focused results: effect-contract2, effect-compiler1, EQ13, builtins11 passed; targeted all-target Clippy, formatting, effect-runtime/builtins/realtime policies and diff checks exit0. Raw logs/exits: `/tmp/issue770-attempt2-{contract-final,compiler-final,eq-response-final,builtins-filter-final,clippy-3,fmt-check,effect-runtime-policy,builtins-policy,realtime-policy,diff-check}.{log,exit}`. Earlier debugging logs remain distinct. Source pauses for root checkpoint; broad changed-source gates and one Astra medium attempt-2 verdict remain pending.
