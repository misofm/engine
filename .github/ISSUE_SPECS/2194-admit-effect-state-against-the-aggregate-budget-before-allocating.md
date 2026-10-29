# Admit effect state against the aggregate budget before allocating

GitHub #2194. Found by #2191 attempt 2: a 512-track session with four delay inserts per track
(about 700 KB of JSON) peaks at 1.46-1.58 GB during boot before `host.effect.resource.limit`
refuses it, three times the 512 MiB default budget. On wasm the linear memory never shrinks.

## Root cause

Nothing sums effect state across instances before each instance is allocated. The effect compiler
checks one instance against the effect cap (`validate_prepare_request`) and then calls
`factory.prepare`, which allocates (a delay at 48 kHz holds about 768 KB of rings). The sum is
first compared with `maximum_effect_state_bytes` in host-core after the instance loop, when every
ring exists. Host-web's boot projection (`projected_retained_bytes`) has no effect row, and the
control plane prepares a rebuild or replace candidate in full before it admits it. The replace
staging checks the document length only, not the live session's retained memory.

## Decision record (root rulings on the mapping, 2026-10-29)

- R1: every native factory reports its exact inline size, `prepared_inline_bytes()`
  (`size_of` of the prepared processor type, the largest when it prepares more than one). The
  aggregate projection charges declared state plus inline bytes per instance. A ceiling gate
  holds `boxed_dyn_bytes(processor) <= maximum_state.total() + prepared_inline_bytes()` for every
  launch effect, quality and launch rate.
- R2: one parse multiplier. The replace parse projection reuses `PARSE_TRANSIENT_MULTIPLIER` (20).
- R3: the dense 6,000-track residual (graph, builtins and control rows, bounded only by the parse
  projection) is out of scope. Named successor: #2199.
- R4: one shared walk of strips, racks and instances. The projection and the preparation loop are
  two passes over it.
- R5: a re-pinned drift bound is the newly measured value plus the documented headroom. No gate is
  loosened.
- R6: no C ABI change and no new limits word. The C ABI's fix is the core's pre-allocation check of
  effect state against its own cap.
- Class A only: no rendered bit moves and no render-path file changes. Refusals move earlier;
  nothing refused before is admitted now.

## Contract

1. `effect_compiler::project_session_effects` sums, without allocating effect state, every
   instance's declared state (`QualityDescriptor::maximum_state`), scratch and inline bytes from the
   descriptor's `(quality, rate)` row. `prepare_native_session_effects` runs it first and refuses
   `effect.resource.limit` at
   `$.effect_compile_caps[projected_bytes=P,limit_bytes=L,instances=N]` before the first
   `factory.prepare` when the declared state or scratch exceeds its cap.
2. Host-core runs the same projection right after `check_counts` and refuses
   `host.effect.resource.limit\t$.maximum_effect_state_bytes[projected_bytes=P,limit_bytes=L,instances=N]`
   (or `$.maximum_effect_scratch_bytes[...]`) before any preparation allocation. The post-loop sum
   still feeds the report rows; a difference from the projection is an internal fault.
3. Host-web's retained projection gains the effect row (declared state plus inline bytes), so
   `host.budget.retained_projection` refuses before the control plane compiles. Its diagnostic adds
   `effect_state_bytes=`.
4. The control plane admits a rebuild or replace candidate's projection before it prepares it: the
   held plans' graph and source rows, the held models and the candidate's model, plus the
   candidate's projected effect bytes, against `maximum_replacement_peak_bytes`
   (`control.resource.replacement_peak`); and the held effect state plus the candidate's against
   `maximum_effect_state_bytes` (`effect.resource.limit`). Same retryable or permanent split as the
   post-preparation admission, which stays as the exact check.
5. `replace_document_ptr` projects `document_bytes * PARSE_TRANSIENT_MULTIPLIER` plus the live
   session's retained bytes (the bridge's fixed rows and the control plane's held rows) against the
   budget before it allocates the staging. A refusal returns zero and writes
   `host.budget.parse_projection\t$.maximum_memory_bytes[projected_bytes=,budget_bytes=,document_bytes=,live_retained_bytes=]`
   to the diagnostic buffer.

## Non-goals

- Graph, builtins and control rows before allocation (#2199).
- A C ABI limits word or any ABI layout change.
- Splitting host-web's per-subsystem caps from the budget: a cap derived from the boot session would
  mis-bound a replacement session of another shape.

## Gates (one test per claim)

- A. `crates/host-core/tests/effect_state_projection.rs`: the 512-track x 4-delay session under
  `maximum_effect_state_bytes = 512 MiB` is refused `host.effect.resource.limit` naming projected
  and limit bytes, with the thread's peak under `20 x document`.
- B. `crates/effect-compiler/tests/prepared_state_ceiling.rs`: for every launch effect, quality and
  launch rate (and both sidechain variants where declared), the prepared processor's charged bytes
  are at most its declared state plus `prepared_inline_bytes()`.
- C. `hosts/host-web/tests/boot_transient_budget.rs`: the same document at the default budget is
  refused `host.budget.retained_projection` naming `effect_state_bytes`, peak under `20 x document`.
- D. `scripts/check-web-boot-budget.mjs` with the new example `effect_state_document`: the wasm boot
  is refused with linear-memory growth at most `20 x document + frame rows + one page`.
- E. `representative_retained_projection_tracks_the_post_prepare_exact_aggregate` stays green
  (R5), `retained_projection_budget_diagnostic_names_projected_bytes` names the new field.
- F. `crates/control-plane` test: a one-track live session, `SESSION_REPLACE` to the 512 x 4 document
  under a 512 MiB replacement peak is refused `control.resource.replacement_peak` before the
  candidate is prepared (thread peak under `20 x document`), the running plan untouched.
- G. `hosts/host-web/tests/boot_transient_budget.rs`: a live session that retains most of a small
  budget refuses `replace_document_ptr` for a document whose `20 x bytes` exceeds the remainder,
  naming `live_retained_bytes`, with almost no allocation.

## Evidence

(Filled during attempt 1.)
