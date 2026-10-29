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
   resolving instance's declared state (`QualityDescriptor::maximum_state`) and scratch from the
   descriptor's `(quality, rate)` row (`effect_contract::declared_instance_resources`, which
   `validate_prepare_request` also reads), plus each factory's `prepared_inline_bytes()` and
   `prepared_undeclared_heap_bytes(quality)`. It is the first pass of the one walk
   (`for_each_session_effect`) whose second pass is `prepare_native_session_effects`. The
   compiler's own `EffectCompileCaps` stay per instance.
2. Host-core runs the same projection right after `check_counts` and refuses
   `host.effect.resource.limit\t$.maximum_effect_state_bytes[projected_bytes=P,limit_bytes=L,instances=N]`
   (or `$.maximum_effect_scratch_bytes[...]`) before any preparation allocation. The post-loop sum
   still feeds the report rows; a difference from the projection is an internal fault.
3. Host-web's retained projection gains the effect row (`EffectStateProjection::retained_bytes`:
   declared state and scratch, inline and undeclared heap bytes), so
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

- A. `crates/host-core/tests/effect_state_projection.rs`,
  `effect_state_over_the_cap_is_refused_before_it_is_allocated`: the 512-track x 4-delay session under
  `maximum_effect_state_bytes = 512 MiB` is refused `host.effect.resource.limit` naming projected
  and limit bytes, with the thread's peak under `20 x document`.
- B. `crates/effect-compiler/tests/prepared_state_ceiling.rs`,
  `every_prepared_launch_effect_fits_its_projected_bytes`: for every launch effect, declared
  quality, launch rate, quanta 1 to 4,096, both sidechain variants where declared and every lattice
  point of a prepared-only parameter, the prepared processor's box is at most
  `prepared_inline_bytes()` and its box plus heap at most its declared state and scratch plus
  `prepared_inline_bytes()` and `prepared_undeclared_heap_bytes()`. (In the effect compiler, which
  owns the launch registry and the projection; the conformance crate does not depend on the launch
  effects.)
- C. `hosts/host-web/tests/boot_transient_budget.rs`,
  `effect_state_the_budget_cannot_hold_is_refused_before_preparation`: 128 tracks x 4 delays under
  a budget of exactly their declared state (so host-core's effect cap admits it) is refused
  `host.budget.retained_projection` naming `effect_state_bytes`, peak under `20 x document`.
- D. `scripts/check-web-boot-budget.mjs` with the new example `effect_state_document`: the wasm boot
  is refused with linear-memory growth at most `20 x document + frame rows + one page`.
- E. `representative_retained_projection_tracks_the_post_prepare_exact_aggregate` stays green
  (R5), `retained_projection_budget_diagnostic_names_projected_bytes` names the new field.
- F. `crates/control-plane/tests/session_replace.rs`,
  `a_replace_whose_effects_cannot_fit_is_refused_before_it_is_prepared` (effect cap unbounded): a one-track live session, `SESSION_REPLACE` to the 512 x 4 document
  under a 512 MiB replacement peak is refused `control.resource.replacement_peak` before the
  candidate is prepared (thread peak under `20 x document`), the running plan untouched.
- G. `hosts/host-web/src/tests.rs`, `a_replace_parse_is_projected_beside_the_live_session`: at the
  default budget, `replace_document_ptr` at the boot's derived document limit refuses beside the
  live session, naming `live_retained_bytes`, staging nothing and allocating at most its
  diagnostic; the longest document the remainder holds stages, one byte more does not.

## Evidence (2026-10-29, base `8822bf0a4`)

Red on the code before each slice, run once locally (the box has 385 GB free; nothing over 1.6 GB
runs in CI, since every gate's green run refuses before the large allocation):

- A (`effect_state_over_the_cap_is_refused_before_it_is_allocated`), on base: `the refusal
  peaked at 1578719945 bytes, over the 27715180-byte parse transient of a 1385759-byte document`.
- B (`every_prepared_launch_effect_fits_its_projected_bytes`), with the projection as the mapping
  wrote it (declared state plus inline bytes): red for three effects. The parametric EQ's heap is a
  scratch-sized buffer (8 bytes per frame, 1,024 bytes at quantum 128 against 1,008 declared state
  bytes); the soft-clip's three histories are double-written (heap 1,656 against 840 declared);
  the reverb keeps its prime sieve (heap 652,648 against 646,008 declared at 44.1 kHz, 1,400,373
  against 1,384,472 at 96 kHz). Hence the scratch term and `prepared_undeclared_heap_bytes`
  (correction 1 below).
- C (`effect_state_the_budget_cannot_hold_is_refused_before_preparation`), on the slice-1 commit
  (host-core's cap in place, no effect row): `the refusal peaked at 400289233 bytes, over the
  6940780-byte parse transient`.
- F (`a_replace_whose_effects_cannot_fit_is_refused_before_it_is_prepared`), on the slice-1
  commit: `the replace peaked at 1601245321 bytes, over the 27715180-byte parse transient`.
- G (`a_replace_parse_is_projected_beside_the_live_session`), with the new refusal skipped: `the
  boot limit's document beside the live session` (the base behaviour: the staging checks the
  length against the boot limit only).

Green: A to G. Gate D on the module built from this tree (`build-web-audioworklet.sh
--module-only`): `effect-state refusal growth 13762560 of bound 28352108`; the existing legs are
unchanged (`growth 19726336 of projection 21542912`). Gate D was not run red: at the default budget
host-core's effect cap alone already refuses this document before it allocates, so the leg is red
only on the tree before #2194, whose module was not rebuilt.

Gate E re-pins (R5, measured plus about 8%, each tighter):

| Representative | Measured gap / projection | Bound, old -> new (permille) |
| --- | --- | --- |
| identity one track | 355,867 / 762,308 = 0.467 | 513 -> 504 |
| parametric EQ nine tracks | 650,619 / 823,416 = 0.790 | 942 -> 853 |
| console sixty-four tracks | 3,073,188 / 1,872,862 = 1.641 | 2,901 -> 1,772 |

Not run: the browser `console-limits` qualification (no browser matrix run in this attempt); no
bridge row changed, so `expected.json` is untouched.

Corrections to the mapping:

1. Declared state is the state payload's size, not a heap bound. Three launch effects hold heap
   their payload does not carry, so R1's two terms are not a ceiling. The projection charges the
   declared scratch too, and each factory reports `prepared_undeclared_heap_bytes(quality)`: the
   soft-clip's histories beyond the payload (derived from its constants) and the reverb's sieve
   (`largest + 2 * PRIME_SEARCH + 1`). Every other launch effect reports zero. Gate B holds the
   sum as a ceiling at quanta 1, 128, 1,024 and 4,096.
2. `EffectCompileCaps::maximum_total_state_bytes` is a per-instance ceiling (it becomes each
   request's `PrepareEffectLimits`), and its callers (tools, tests) pass per-instance values. The
   aggregate check therefore sits in host-core, which owns the aggregate cap, not in the effect
   compiler. The compiler exposes the projection; the walk is still one (R4).
3. `prepare_caps` keeps every subsystem cap at the whole budget. A cap derived from the boot
   session's other rows would mis-bound a replacement of another shape (non-goal).
4. The replace staging refusal returns zero as before and writes its diagnostic to the live host's
   diagnostic buffer; it does not change `last_result`, which the page reads as the last control
   command's result.
5. The 512 x 4 document built from the nine-track fixture is 1,385,759 bytes, not about 700 KB.
