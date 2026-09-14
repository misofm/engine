# Assignment 4 resume amendment — Astra XHIGH findings

Status: final read-only recommendation received at account-switch pause. No source edits. Root must freeze the staging choice and amend the numbered issue before implementing; do not treat the alternative below as already approved.

## Queue and graph accounting

Use `engine::realtime::bounded_spsc_retained_payload::<EffectControlRecord>(actual_capacity)` as exact authority: sentinel slot, enum layout, shared header and Arc counts are included. `attach_effect_console` in effect-compiler/prepare.rs uses min(requested_depth, automation_capacity). Existing queue storage is currently uncharged; enlargement affects every console effect, even unsupported/non-EQ factories.

Add a checked helper in graph-compiler/estimate.rs, invoked in compile.rs while entries retain their control bindings, **after semantic_estimate is cloned and before capped_estimate/cap checks**. This preserves target-neutral semantic graph identity. Charge once into graph_metadata_bytes, incremental_plan_bytes, session_plus_plan_bytes; update largest_allocation_bytes for each actual backing allocation/header. Existing host-core propagation suffices. Do not repurpose HostPrepareReport.control_retained_bytes, which means SourceControlSet table/ID arena only.

## Host-web decoded staging

ReadyOwnership.command_decoded is Box<[(u32, AdmittedCommand)]>, and AdmittedCommand embeds the enlarged enum. boxed_command_staging allocates 2*MAXIMUM_COMMAND_RECORDS + 2*track_count entries. Its backing array currently appears uncharged; public 48-byte wire staging is a separate allocation.

Charge the entire actual typed allocation once into bridge_metadata_bytes and bridge_retained_bytes, and include it in largest_bridge_allocation_bytes/largest_named_allocation_bytes before final budget validation. Preserve the public wire-staging row. No new ABI report field or broad accounting rewrite is needed.

## Producer safety

There is no existing checked target preflight: EffectControlProducer.producer exposes a raw Producer<EffectControlRecord>. Recommended bounded change: retain field name with a checked wrapper whose underlying producer is private; preserve capacity/try_push usage, distinguish unsupported delivery from full, and return the refused record. At assignment 4 production target publication must be refused before queue mutation because factory capability stays None. Component fixtures can explicitly construct low-level queues under exclusive ownership to exercise the completed EQ hooks. No public raw escape hatch or caller-controlled capability setter.

Later owner/admission assignments enable target publication only through validated transactions. Once an owner requires prepared targets, ordinary unlowered Parameter records must also be refused before publication. Host-web delivery preflight must run on each original command before any publish/observation mutation, preserving refusal index and mixed-batch atomicity. A push failure after successful preflight remains an invariant error, not ordinary refusal.

## Staging choice to freeze

Original approved brief: caller-supplied separate span/target slices. Scalar owns target staging; bank needs per-lane staging, packed targets, and **separate target offsets** from semantic offsets. Charge all actual allocations once.

Astra's simpler proposed amendment: optional target staging owned per EffectControlLane, only for opted-in owners and sized to actual queue capacity. Stage FIFO and expose retained prefix; scalar applies immediately, bank applies only after desymmetrization. This avoids a second packed target allocation and target offsets. It changes the caller-supplied API requirement and must be explicitly accepted in the spec before coding. Unsupported owners must allocate no target staging. Do not claim exact accounting before choosing layout.

In either design, snapshot available_at_entry once, drain at most that number, sort/dedupe only semantic spans, never reorder/collapse targets across batches. Fold target channel selectors into symmetry (Left/Right clears LIVE, Both preserves). Apply every target in FIFO after bank state restoration and before render. Example Left X, Both Y, Left Z must produce left Z/right Y. Unsupported application cannot silently succeed.

## Bounded files and gates

Primary: effect-contract/live.rs, symmetry.rs; graph/runtime.rs; rack/lib.rs. Necessary glue: effect-compiler/prepare.rs (and exports only if needed), graph-compiler/estimate.rs and compile.rs, hosts/host-web/src/lib.rs. Direct tests in effect-contract, graph/rack, graph-compiler, host-core and host-web. Leave console-free EffectBankStage and unrelated effect implementations unchanged.

Gates: <=64-byte queue record, frozen drain/no producer chasing, real target application, FIFO overlap, desymmetrization ordering, zero dropped ACKed targets, native/Wasm. Additional resource/preflight gates: console-off charge zero; non-EQ and pre-cutover EQ queues charged; actual capped depth and sentinel proven; unsupported-owner target staging absent; scalar/bank storage charged once; exact graph/host-web budget accepted and one byte below refused; independent largest allocation; unsupported target leaves queue/counters/observation unchanged; mixed late refusal atomic; ordinary semantic commands operational.
