# #807 assignment 6 layout and ownership amendment

Astra XHIGH read-only design, accepted by root after assignment 4. This refines the
numbered issue's assignment 6; it does not activate production EQ or implement
assignment 7 admission. Follow the numbered issue for product and trust requirements.

## One preparation owner

Add a narrow effect-compiler helper constructing only one EQ factory Arc, conditional
on its real target_preparation capability. Construct no full registry or effect roster.
Before assignment 10, production construction returns unavailable. host-core's
EqTargetPreparer retains the factory once and accepts explicit fixture factory injection.
Its stateless operation uses fixed stack candidate/dirty/target arrays, validates all
60 canonical seeds and every original edit, normalizes edit zero consistently with
assignment 5, designs through the companion, and copies output only after success.
It creates no EffectControlOwner, processor, graph, source rings, host or revision.

Host-core real-EQ component tests use a dev-only concrete EQ wrapper. Host-web tests
cover codecs, failure/output preservation and production Unsupported. Do not add a
host-web effect dependency or a feature-gated fixture facade. Assignment 10 must prove
the actual successful production workspace/host route.

## Explicit little-endian layouts

Rust implementation names are unversioned. Wire channel 0/1/2 maps explicitly to
Left/Right/Both; never transmute the internal enum. All fields are explicit; no
uninitialized padding is transmitted. Generate offsets with offset_of! and sizes with
size_of!, including native and Wasm assertions.

| Structure | Fields with byte offsets | Bytes |
| --- | --- | ---: |
| WebEqTargetRequest | struct_size:u32@0, abi_version:u32@4, sample_rate_hz:u32@8, seed_count:u32@12, edit_count:u32@16, reserved:[u32;3]@20 | 32 |
| WebEqTargetEdit | parameter_id:u32@0, channel:u32@4, value:f32@8 | 12 |
| WebPreparedEffectTarget | slot:u32@0, channel:u32@4, words:[u32;12]@8 | 56 |
| WebEqTargetResult | struct_size:u32@0, abi_version:u32@4, value_count:u32@8, target_count:u32@12, workspace_retained_bytes:u64@16, workspace_largest_allocation_bytes:u64@24 | 32 |
| WebPreparedEffectCompanionHeader | struct_size:u32@0, abi_version:u32@4, host_generation:u64@8, target_count:u32@16, reserved:u32@20 | 24 |
| WebPreparedEffectCompanionRecord | track_index:u32@0, rack:u32@4, effect_index:u32@8, reserved:u32@12, base_revision:u64@16, slot:u32@24, channel:u32@28, words:[u32;12]@32 | 80 |
| WebEqTargetConfig | struct_size:u32@0, abi_version:u32@4, sample_rate_hz:u32@8, value_count:u32@12, host_generation:u64@16, owner_revision:u64@24, values:[f32;60]@32 | 272 |

Input: request header + 60 seed floats + <=256 edit records. Seed offset32,
edit offset272, capacity3344, exact bytes272+edit_count*12.
Output: result header + 60 final floats + <=12 targets. Value offset32,
target offset272, capacity944, exact bytes272+target_count*56.
Companion capacity24+(2*MAXIMUM_COMMAND_RECORDS)*80 =40984 bytes. Config/companion
layouts are defined in6; their real operations and allocations belong to7.

## Additive preparation exports

Prefix `miso_engine_web_v1_eq_target_`: open()->u32, request_ptr()->u32,
request_capacity()->u32, prepare(request_bytes:u32)->u32, result_ptr()->u32,
result_bytes()->u32, result_capacity()->u32, close()->u32.

Explicit open prewarms/reuses one TLS Option<Box<EqTargetWorkspace>> off audio;
unavailable production capability returns existing RESULT_UNSUPPORTED. Closed
operations return WRONG_STATE and pointer accessors return zero. Close drops the
workspace/factory off audio and invalidates pointers. Accessors do not clear results.
Failed preparation preserves the entire output backing and result length; status is
authoritative. A zero-edit success still requires genuine capability/seed validation.
No caller pointer is accepted. Check supplied exact length, capacity, struct sizes,
ABI, seed/count bounds, reserved zeros and selectors before decoding with checked
little-endian slice access. Include unaligned-input tests.

The boxed workspace contains both fixed input/output arrays inline. Retained bytes
are actual sizeof(workspace) plus its single factory Arc allocation; largest is their
maximum. Report these exclusively in the preparation-result resource fields. The
embedding accounts this bounded preparation workspace alongside its live host. Do
not prewarm a designer/factory in audio admission. Assignment7 separately allocates
and charges actual config/companion backing to the bridge.

## Mechanical gate glue

Update exact export vocabulary in scripts/check-web-audioworklet.sh and
scripts/check-abi-layout-v1.py, plus scripts/fixtures/abi-layout-v1-self-test.json.
Generate SDK ABI assets through existing codegen (sdk/assets/miso-engine-v1-abi-layout.json
and sdk/src/generated/abi.ts); no handwritten offsets. Keep actual artifact rebuild
and production-success browser/headless qualification at assignment10.

Focused gates cover real facade preparation, asymmetric seeds/Both edits, invalid
original edits despite later overwrite, malformed sizes/counts/reserved/truncation,
12/512 bounds, failure output preservation, pointer/open-close behavior, exact
resource arithmetic and prewarmed zero allocations/frees. Native/Wasm, ABI validator
and self-test, and generated SDK checks must pass. No new benchmark/harness framework.

## Forward cutover glue found during preparation review

Assignment10 must update tools/console-workload/src/lib.rs::push_parameter for opted-in
owners using the real preparation/publication seam, still off the clock. Its current
raw Parameter route will correctly refuse EQ after cutover. Preserve other effects'
ordinary behavior and count real failures; update the Point-span-only documentation
and existing console-workload automation/chain fixtures proportionally. This is an
existing production-tool caller, not permission to add benchmark machinery or run
extra timed workloads. Assignment7 must also preserve original wire indexes through
coalescing: today's host capacity loop reports lowered indexes, which are insufficient
once prepared targets and mixed-command expansion change the mapping.
