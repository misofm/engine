# Scoping study: removing the builtins-less path

Date: 2026-09-27. Read-only study. No code was changed, no benchmark was run, and nothing was
pushed or filed.

**Trees read.**

- `M:` is `main` at `252622b6`.
- `B:` is the local optimisation batch `codex/batch-plumbing-floor-2` at `e2b5b9c8`. It merges
  #935, #936, #937, #942, #943, #944, #945, #949 and #881 on top of `main`'s `65671c21`. The
  batch moved from `c941b9c7` to `e2b5b9c8` (#943, #949) while this study ran, and every `B:`
  anchor was re-derived on `e2b5b9c8`.

A `path:line` without a prefix is a `B:` anchor. Files the batch does not touch have the same
lines on `M:`.

**Owner's question.** Should the engine carry a "builtins-less" plan at all, when no real host
compiles one? The real hosts are the browser boot (`hosts/host-web`), the C ABI (`crates/capi`)
and the native host (`crates/host-core`). "Builtins-less" means a plan built by
`GraphCompiler::compile` rather than `compile_with_builtins`, so its tracks have no input section,
fader or pan matrix.

---

## Verdict

1. **The premise holds.**
   - Every shipped host compiles with builtins (`crates/host-core/src/prepare.rs:1195`) at the
     build's vector width:
     - `Simd8` natively.
     - `Simd4` in the browser, whose only shipped artifact is `simd128`
       (`scripts/build-web-audioworklet.sh:26-30`).
   - Nothing current or planned produces a builtins-less plan. Its only non-test callers are:
     - the two plumbing benchmark rows;
     - the graph determinism fixture;
     - two manually run compile tools (the #650 audit and the #006 compile benchmark).
2. **One qualification.** The optimisations in question need a plan with *no bank at all*. They do
   not strictly need a plan without builtins.
   - A with-builtins plan is bankless only at `Backend::Scalar`. No shipped build uses that
     backend.
   - Two host-core tests force it, and #926 recorded the fused Output fold running there (fan-in 3
     and 9). The fold is class A, so removing it changes no test result.
   - #927's in-place Output source read is stricter still. It needs an Input read only by a
     retired route, which no plan with builtins has at any backend.
3. **Code only builtins-less plans reach.** About 900 lines of render and bind code in
   `crates/graph`:
   - the fused Output route reduction: #926's fold, the #926/#937 kernels and #927's in-place
     Output source reads with `played_planes_group`;
   - #925's alias lowering of unlisted builtin stages.

   None of it can run in a shipped host. The browser module still carries two of its kernels.
4. **Recommendation: option (a).**
   - Remove the builtins-less compile from every product and tool path.
   - Keep a private, test-only builtins-less entry for the graph compiler's own unit tests. There
     builtins are noise, not the subject.
   - Delete the render-path code in (3).
   - Retire both plumbing rows.
   - Make `sixty_four_track_gain_pan_only` the pure-audio-path target. It has builtins, they are
     settled, and it has no effects.
   - Re-base the driver-fed row and #938's live-producer row onto that session.
5. **Size.**
   - About −900 product lines.
   - About −2,600 lines of graph-crate tests, plus about 300 ported (#936's gates).
   - About −2,100 lines of benchmark tooling, plus about 300 for the re-based driver-fed row.
   - It is split into one tooling issue, one product slice and four successors (Appendix).

---

## 1. Is the premise true?

### 1.1 Every production compile has builtins and a vector backend

| host | how it reaches the graph compiler | dispatch |
|---|---|---|
| browser (`hosts/host-web`) | Every boot branch (`hosts/host-web/src/lib.rs:7749-7800`) calls a host-core `prepare_*` entry. Every one of those passes `Backend::current()` (`crates/host-core/src/prepare.rs:536, 557, 583, 620, 654, 673, 696, 719, 742, 786`). The one compile is `compile_with_builtins { dispatch: backend }` at `prepare.rs:1195-1196`. | `Simd4`. The shipped module is `simd128` only (`scripts/build-web-audioworklet.sh:26-30, 60, 79`, owner decision W4-D1), and `crates/lane/src/backend.rs:31-70` makes that `Simd4`. |
| C ABI (`crates/capi`) | `crates/capi/src/runtime/compile.rs:410` calls `prepare_host_runtime`, which reaches `prepare.rs:1195` through `:508-516`. | `Simd8` on every x86 target (`backend.rs:32-35`); the workspace pins `x86-64-v3`. |
| native PCM runner | Only through the C ABI (`tools/native-pcm-runner/src/lib.rs:1250`). | `Simd8` |
| builtin batch endpoint | `crates/host-core/src/builtin_batch_endpoint.rs:1077-1091` passes `Backend::current()`. Only the `#[cfg(test)]` seams (`:1093`, `:1111`) pass another backend. | `Simd8` / `Simd4` |
| `hosts/host-native`, `hosts/host-mobile` | Boot attestation and target smoke only. They build no graph. | n/a |

Native AArch64 is "unsupported, no claim" (`docs/TARGET_MATRIX.md:11-12, 88`). If it is revived it
would be `Simd4` (`backend.rs:36-38`).

### 1.2 Every caller of the builtins-less compile

`GraphCompiler::compile` is `crates/graph-compiler/src/compile.rs:139-143`. It is the shared
pipeline with no builtin tails and `prepared_builtins = None`.

| caller | what it is | dispatch | runs where |
|---|---|---|---|
| `tools/console-workload/src/lib.rs:1120` | The two plumbing rows (branch `:1108-1191`) | `current()` | Native console bench. Also the wasm console arm, where the row is `WORKLOADS[11]` (`:463`). |
| `tools/console-workload/src/lib.rs:2316` | Test `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables` (`:2309`), which recompiles the ring row | `current()` | CI (`M:.github/workflows/qualification.yml:564`) |
| `crates/graph-compiler/src/bin/graph_fixture.rs:88` | The graph fixture and fingerprint | `Scalar` | The CI determinism gate: `scripts/check-graph-determinism.sh` runs at `qualification.yml:717` |
| `tools/audit/src/prepared_effect_allocations.rs:212, 258, 506` | #650 compile-only allocation records | `Scalar` for two corpora, `current()` for `Banks64` (`:226-231`) | The subcommand is manual (no script or workflow runs it). Its unit test runs in CI (`qualification.yml:564`). |
| `tools/bench/src/graph.rs:286, 632` | #006 compile benchmark | `current()` | Manual (`scripts/run-graph-compiler-benchmark.sh:74`). Its unit test runs in CI. |
| tests | `crates/graph-compiler/src/lib.rs` (61 call sites in 38 functions), `tests/track_delay.rs` (8 functions), `tests/scale.rs` (1) | mixed | CI |

Two tools also build builtins-less plans by hand:

- `tools/audit/src/graph.rs:256`: an Input → Output lifecycle audit, in CI at `qualification.yml:694`.
- `tools/audit/src/source.rs:374`: the source audit, in CI at `:693`.

Both have fan-in one, so neither reaches any code in section 2.

Nothing else builds a builtins-less plan:

- The fuzz targets depend on no graph crate (`fuzz/Cargo.toml:11-15`).
- `crates/conformance` compiles no graph.
- The wasm-gate guest depends only on `lane` and the DSP corpus.

### 1.3 Can a with-builtins plan have bankless track strips?

**Only at `Backend::Scalar`.**

- `planned_builtin_bank_members` returns no groups when the backend has no bank width
  (`crates/builtins-compiler/src/lib.rs:1300-1309`). It also skips a stage node that has no
  dependency level (`:1320-1327`). When every group is empty, `into_graph_artifact_with_banks`
  falls back to per-node bindings (`:2349-2363`).
- On a vector backend every strip node **of every track that exists** is a bank member. The doc
  says so at `:1290-1294`, and `debug_assert!(plan.scalar.is_empty())` at `:1342` checks it.
- The only other with-builtins plan without a bank is a session with zero tracks. Its Output has
  fan-in 0, so it fails the fold's `inputs.len() < 2` clause (`crates/graph/src/runtime.rs:7383`)
  and reaches none of section 2. This study did not check whether the session grammar admits zero
  tracks.

**Fewer tracks than the bank width.**

- Each stage gets one short bank per dependency level, padded with identity lanes ("Partial groups
  are padded rather than dropped (#96 F6)", `crates/rack-compiler/src/lib.rs:243-245`; see also
  `builtins-compiler/src/lib.rs:1290-1293`).
- So a one-track session at `Simd8` has three one-member builtin banks, each with seven identity
  lanes. There is no scalar remainder.

**Effects that do not bank.** This covers a slot without the bank-kernel contract, a sidechained
slot, and future opaque third-party Wasm, which is dynamic-rack only and never banks (`AGENTS.md`;
spec 028; `008-…md:35`).

- Such an effect renders per node.
- The track's builtins still bank, so the plan still has banks.

**Where `Backend::Scalar` exists at all.**

- `Backend::current()` is `Scalar` only on a target other than x86, AArch64 or wasm with `simd128`
  (`backend.rs:61-69`).
- Shipped: never.
- CI compiles `host-web` and `host-core` with `-simd128` (`qualification.yml:755-757`). That build
  is compiled only; it is not run and not shipped.
- The scalar wasm-gate guest does run (`scripts/run-wasm-gates.sh:8`), but it builds no graph.
- Tests run with-builtins plans at `Scalar` in two host-core tests:
  - `forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses`
    (`builtin_batch_endpoint.rs:1497`, scalar at `:1575`, `:1595`);
  - `endpoint_selects_existing_pair_factories_without_observer_barriers` (`:1752`,
    `run(Backend::Scalar)` at `:2122`).
- There are also graph-compiler and builtins-compiler unit tests at `Scalar`.

Side note: `docs/TARGET_MATRIX.md:13` still says the baseline and `+simd128` wasm builds are
"distinct artifacts". W4-D1 made that stale.

### 1.4 Is any builtins-less host planned?

No. No spec plans a builtins-less, plain-mixer, effects-only or mixer-only host. The specs that
touch the question agree:

- #925's verdict: "no host, C-ABI or web path compiles builtins-less"
  (`.github/ISSUE_SPECS/925-…md:381-384`).
- #927: host-core "always compiles with builtins", and a bind probe over
  `cargo test -p host-core --all-features` found no Output in-place read
  (`927-…md:366-370`).
- #940's ruling 1: "It moves only bankless plans … no product host compiles a builtins-less plan
  today. The production wins are S3-S5" (`M:.github/ISSUE_SPECS/940-…md:31-35`). #940 and #941
  are already closed on GitHub.

**Answer.** The premise is true for every shipped build. The one qualification is that the class
that matters is *bankless* plans, which includes with-builtins plans at `Backend::Scalar`. Those
exist only in tests and in an unshipped CI compile check.

---

## 2. Code reachable only from builtins-less or fully bankless plans

**How to read the table.** "Shipped" means a with-builtins plan at `Simd8` or `Simd4`. "Scalar"
means a with-builtins plan at `Backend::Scalar`, which only tests produce.

| # | code (`B:`) | issue | deciding predicate | shipped | Scalar (tests) | builtins-less |
|---|---|---|---|---|---|---|
| 1 | `output_route_fold` and `OutputRouteFold` (`crates/graph/src/runtime.rs:7287-7439`) | #920/#926 | **`parts.membership().is_empty()` (`:7367`)**. The membership holds every effect-bank *and* builtin-bank member (`bank_membership`, `:4439-4461`). The remaining clauses are Output identity, fan-in ≥ 2, and every producer a plain, in-place, unobserved route read only by the Output. | never | **yes**: #926 recorded the two scalar `builtin_batch_endpoint` tests taking it, at fan-in 3 and 9 (`926-…md:328`) | yes |
| 2 | The fused kernels: `OUTPUT_GROUP`, `route_reduce`, `route_group`, `route_pair_vectors`, `route_lone_vectors`, `route_tail`, `route_run`, `mix_chunk`, `add_mixed_chunks` (`runtime.rs:688-1087`) | #926/#937 | Called only from `execute_op`'s third host arm (`:3676-3684`). That arm is taken only when `routes` is non-empty, which means row 1 admitted. | never | yes | yes |
| 3 | `Runtime.output_routes` and its plumbing: the field (`:2569-2573`), both layout mirrors (`:2633`, `:2658`), the accessor (`:2754-2756`), the constructor parameter and assert (`:2806-2828`, `:2873`), the `execute` routing (`:3068`, `:3093`) and `execute_op`'s parameters and doc (`:3602-3624`) | #926 | same as row 2 | never. The table is empty, but every plain unit still pays the `output` selects (`:3091-3103`) and the Output unit one `is_empty` per block. | yes | yes |
| 4 | `OutputSources` and `resolve_group` (`:622-686`); `Runtime.output_sources` (`:2574-2579`, `:2634`, `:2659`, `:2874`, `:3069`, `:3094-3103`, `:6103`); `SourcePlanes.output` (`:6126-6135`); `source_plane_table` clauses (b′) and (e) (doc `:6160-6187`, code `:6219`, `:6237`, `:6280-6290`) | #927/#937 | An Input claim whose **only reader is a route the Output fold retired** (`:6280-6284`). | never | **never**. With builtins the Input's reader is `PostInputBuiltins`, a bank member or a bound op, never a route. Structurally, `compile.rs:236-243` always adds the Input → `PostInputBuiltins` edge, and `compile_with_builtins` always lists `PostInputBuiltins` (`:816`, `:826-832`), so that stage keeps its op and is the Input's reader. #927's bind probe agrees (`927-…md:366-370`). | yes, driver-fed only (`sixty_four_track_plumbing_ring`) |
| 5 | `GraphSourcePlanes::played_planes_group` (`crates/graph/src/lib.rs:1863-1899`, a realtime-marked region) | #937 | Its only caller is `OutputSources::resolve_group` (`runtime.rs:684`). | never | never | yes (ring) |
| 6 | The Output-fold arms of `validate_fold_installation` (`runtime.rs:5492-5500`, `:5507-5508`) and `build_sequential` (`:5730-5734`) | #926 | `output_fold.is_some()` | never | yes | yes |
| 7 | Test seams: `test_only_set_output_route_fold_declined` (`runtime.rs:5273`, `:5285-5287`, `:5451-5452`; export `lib.rs:36`) and `GraphExecutor::output_route_folds` (`lib.rs:2572`) | #926 | test-support only | n/a | n/a | n/a |
| 8 | #925's alias arm: `is_builtin_stage` (`crates/graph/src/program.rs:197-217`), the `listed` interning and the predicate (`:649-664`), the `bindable` parameter through `lower`, `lower_with` and the test-only `lower_with_per_bank_windows` (`:544-581`), `lower_from_current_fields` (`lib.rs:1373-1388`), and the `required_bindings` conditional (`crates/graph-compiler/src/compile.rs:803-835`) | #925 | A builtin stage missing from `required_bindings` (`program.rs:662`). | **never**. `compile_with_builtins` lists all three stages of every track (`compile.rs:816`, `:826-832`), and attaching a builtin bank refuses a member that is not listed (`lib.rs:1341`). With builtins the predicate reduces to `is_alias_candidate`. | never | yes, plus hand-built graph tests |
| 9 | The `GraphCompiler::compile` entry (`compile.rs:136-143`) and its three `None` arms (`:449`, `:654`, `:679`) | — | — | — | — | the entry itself |

**What the shipped browser module carries today.**

- It contains `route_group<f32x4>` (38 vector operations) and `route_tail` (132 scalar
  operations). Neither can run in a browser (`937-…md:236-245`).
- Removing them lowers the AudioWorklet gate's kernel census from 15 to 14. The ratchet minimum is
  11 (`scripts/check-web-audioworklet.sh:428`).

**Code that looks related but is reachable with builtins. Keep all of it.**

- The bank-chain route fold and its epilogues (#218, #915, #945). They share `plain_route_gains`
  (`:6902`), `input_producers` (`:6995`), `op_names_buffer` (`:6697`) and `observed`.
- `source_plane_table` clauses (a)–(d), `gathers_only` (`:6118`) and `ArenaMembers::plane` (#918).
  In every source-fed host session the `PostInputBuiltins` bank gathers the Input in place
  (`crates/host-core/tests/source_in_place.rs:185`).
  - `GraphSourcePlanes::played_planes` and `GraphPreparedSourceSetDriver::provides_played_planes`
    stay.
- #936's inert source-input skip. Every host claim's `SourceInput` op is inert unless it is metered
  or delayed.
- #916's `HostMaster`, and `reduce_plane_into` / `reduce_many_into` (#898). The Output op of every
  plan uses them, and they run whenever a lane does not fold.
- #932 (open). It concerns #918's bank gather as much as #927's reader.

---

## 3. Non-product code that depends on the builtins-less mode

### 3.1 Graph crate tests: builtins-agnostic by nature (keep), and optimisation-specific (go)

`crates/graph` does not depend on `builtins` (`crates/graph/Cargo.toml`), so every graph test
builds its plan by hand without builtin processors. That is the natural unit-test shape for the
graph layer, and it stays. The tests below exist only to exercise the section 2 code:

| feature | tests (`B:`) | ≈ lines | fate |
|---|---|---:|---|
| #926 fold and kernel | `runtime.rs`: `assert_route_reduce_is_the_route_ops_and_the_reduction` (`:14515`), `a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit` (`:14642`), `a_route_tail_refuses_a_run_as_long_as_the_widest_lane` (`:14653`), `an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit` (`:15170`), and their fixtures | 770 | delete |
| #927 in-place Output read | `a_plain_strip_source_is_read_in_place_by_the_fused_output_with_the_copy_bits` (`:16207`), `a_claim_with_another_reader_keeps_the_copy_and_the_copy_bits` (`:16242`), `RingSource` (`:15324`), `ring_output_parts` (`:15520`) | 980 | delete, after porting #936 (below) |
| #927/#937 kernel read | `a_route_reduction_reads_each_lent_input_as_the_copys_words` (`:16681`), `a_source_sets_group_call_is_its_per_claim_call` (`:16696`) | 240 | delete |
| #927 allocation | `tests/rt10_source_in_place_alloc.rs:441` and the `bankless` arm of its plan builder | 75 | delete. Keep the #918 arm at `:378`. |
| #925 | `lib.rs`: `identity_bound_builtin_stages_alias_without_moving_a_bit` (`:6233`) and its corpus helpers (about 360 lines). `program/tests.rs`: `unlisted_builtin_stages_lower_as_aliases` (`:230`), the random-graph arm, and the `builtins_bound` argument at 28 call sites. | 530 | delete; the call-site edits are mechanical |
| **#936, a general feature: port, do not delete** | `an_unobserved_source_input_is_not_dispatched_and_moves_no_bit` (`:16433`), `an_observed_source_input_stays_dispatched_and_meters_the_base_values` (`:16452`), `a_delayed_claim_stays_dispatched_and_renders_the_base_bits` (`:16484`), `the_metadata_charge_covers_the_ring_plans_executor_tables` (`:16515`). All four are built on #927's bankless `ring_output_parts`. The `Plain` shape's claim is read by the fused Output, and `ObservedAlias` meters an elided `PostFader`, which is a #925 alias. | 295 | port onto #918's banked source fixture |
| ledger | `crates/graph/tests/MUTATIONS.md`, the rows for #925, #926, #927 and #937 | — | mark retired |

**This is the one real hazard of the removal.** Deleting #927's fixture without first moving #936's
four gates would silently drop the tests of a feature every host uses.

### 3.2 Graph-compiler tests

47 test functions compile without builtins:

- 38 in `crates/graph-compiler/src/lib.rs`. 23 call it directly. The other 15 go through the
  helpers `compile_bank_and_per_node`, `compile_bank_only`, `compile_fixture`,
  `compile_reverse_route_submix_fixture`, `compile_chain_fixture`, `compile_queued_eq` and
  `compile_console_model`.
- 8 in `tests/track_delay.rs`.
- 1 in `tests/scale.rs`.

19 of them bind and render. None uses `GraphNodeBinding::identity`: binds iterate
`required_bindings`.

**G (42): graph-layer semantics, where builtins are noise.**

- What they cover:
  - determinism and the canonical SHA;
  - levels and colouring;
  - dispatch and resources;
  - effect bank planning and bank-vs-per-node bit identity;
  - PDC alignment;
  - the eight launch-effect fixtures;
  - track delay (8);
  - the 65,537-track scale test.
- Some pin values that hold only without builtins. Moving them onto builtins means re-pinning, not
  re-testing:
  - canonical SHA `14d73…` in `issue122_reverse_route_ids_emit_sorted_levels_and_bind`
    (`:2139`);
  - the Output tail `Finite(0)` in `direct_graph_report_exposes_zero_output_latency_and_tail…`
    (`:2374`). With builtins it is `Infinite`.
  - chain and slot counts in the console, intended-placement and add-a-track tests;
  - SHA `eb3ca776…` in `tests/track_delay.rs:225`.

**B (1): pins the builtins-less shape.**

- `accepted_session_compiles_binds_and_renders_direct_route` (`:13670`) asserts that
  `required_bindings.len() == 2`. That is #925's shape.

**P (4): pair builtins-less against with-builtins.**

- `runtime_bank_slot_reservation_is_published_and_capped_transactionally` (`:3000`).
- `the_merged_span_hold_costs_the_input_slots_with_and_without_builtins` (`:7349`), which pins
  192/129 builtins-less against 256/193 with builtins.
- `builtins_replace_only_the_three_internal_track_bindings` (`:12420`), which pins
  `[Input, Route, Output]`.
- `post_bank_graph_cap_rejects_transactionally_with_both_prepared_inputs` (`:12738`).

In each P test only the builtins-less arm is optimisation-specific.

### 3.3 Benchmarks, gates and tools

| item | anchors | what depends on the mode |
|---|---|---|
| console-workload | Variants `:238-286`. `SourceFeed` `:410-444`. `DRIVER_FED_WORKLOADS` `:482`. `Strip::PlumbingOnly` `:537-546`. The builtins-less branch `:1108-1191`. `FrozenSourceDriver` `:2007-2112`. Unit tests `:2120-2600`. | About 815 lines exist only for the two rows. |
| `tools/console-workload/tests/chain_shape.rs` | `:329` (special arm `:340-356`), `:834`, `:890` (pins `BASE_DIGEST` and census `[1, 65]`), `:995` (skips the row at `:1015`), `:1084` (the #926 fold gate) | About 205 lines. |
| `tools/console-workload/tests/plumbing_profile.rs` | The whole file, 592 lines, `#[ignore]` | It is the only consumer of `graph::test_only_phase_profile` (`crates/graph/src/lib.rs:46-197`, plus probes in the render loop). |
| `tools/bench/src/console.rs` | `PLUMBING_FEED_PAIR` `:163-168`, the in-run digest assert `:212-221`, the test at `:2105` | About 110 lines. |
| `tools/bench/src/floor.rs` | Constants `:87-111`. The floor arm `:288-293`. Tests `:496` (parity), `:566`, `:592`, `:622`. | About 155 lines. |
| jq validators | `scripts/console-benchmark-validator.jq:7` (`length == 50`), `:12` (36 session records), `:33-36` (the plumbing pair shares one digest). `scripts/console-benchmark-record-lib.jq:20`, `:71`, `:133-140`, `:198`, `:208-211`, `:336-345`, `:451`. `scripts/wasm-console-benchmark-validator.jq:70-71`, `:207` (`== 32`), `:212-213` (16 kinds). | The rows are **required**: a record without them is refused. |
| scripts | `scripts/test-console-benchmark.sh` (about 125 lines), `scripts/test-wasm-console-benchmark.sh:61`, `:165-176` (index mutations `.[11]`–`.[15]`), `scripts/run-console-benchmark.sh:126-139`, `:329-330`, `scripts/operator/preflight-console-benchmark.sh:160` (`records_required: 50`) | Counts and pins. |
| wasm console arm | `WORKLOADS` is append-only because the guest prepares a row by index (`tools/console-workload/src/lib.rs:446-450`; `tools/wasm-console-guest/src/lib.rs:140-160`) | Removing index 11 re-indexes rows 12–15. |
| determinism gate | `scripts/check-graph-determinism.sh` runs `graph_fixture` (builtins-less, `Scalar`) 100 times and compares each run with the first. It stores nothing but `target/issue6/fresh-process-determinism.json`. | The gate is self-relative, so porting `graph_fixture` onto builtins keeps it meaningful. The checked-in `M:fixtures/graph/v1/*` and `M:fixtures/graph/MANIFEST.tsv` come from the same binary. Only #947's byte-for-byte test gates them (`947-…md:10`), and they are regenerated if the binary moves onto builtins. |
| #650 audit | `tools/audit/src/prepared_effect_allocations.rs` | It measures a compile no host runs. It is run manually. |
| #006 compile benchmark | `tools/bench/src/graph.rs:286` | Same. Its validators pin nothing absolute (`scripts/graph-benchmark-record-validator.jq:33`, `scripts/graph-benchmark-validator.jq:16`). |
| floor ruling | `docs/rulings/effect-floor-accounting.md:499-584` (the "Plumbing inventory"), `:12`, `:621` | Rewrite (section 5). |
| history | `docs/handoffs/plumbing-floor-2026-09-26/` (3,106 lines); 44 sealed record files across 16 capture directories under `M:artifacts/`; about 20 specs | Keep as history. |

**No dependence.** These do not depend on the mode:

- fuzz, `crates/conformance`, `crates/capi`, `crates/source`;
- every host-core product path.

The two host-core scalar-seam tests take the fold only incidentally. They compare bits, and the
fold is class A, so they stay green without it.

---

## 4. Options

| | (a) Remove from product and tools; private test entry (**recommended**) | (b) Remove entirely; builtins mandatory in the compiler | (c) Keep behind a feature for tests and tools; delete the dead optimisations and rows | (d) Retire the rows only |
|---|---|---|---|---|
| **Removes** | Section 2 rows 1–9. `compile` becomes a `#[cfg(test)]` entry inside graph-compiler. Both plumbing rows. | Everything in (a), plus the `Option` arms (`compile.rs:449`, `:654`, `:679`) and the private entry | Section 2 rows 1–8, and both rows | The two rows and their pins |
| **Product lines** | about −900 in `crates/graph` and graph-compiler: kernels 466, fold 153, scattered state and seams about 150, `played_planes_group` about 40, #925 about 70, compile entry about 20 | about −915 | about −880 | 0 |
| **Tests** | graph: about −2,600, with about 300 ported (#936). graph-compiler: 1 B deleted, the 4 P tests lose their builtins-less arm, the 42 G unchanged but calling a renamed private entry. `track_delay.rs` and `scale.rs` move in-crate, because an integration test cannot see a `cfg(test)` item. | graph as in (a). graph-compiler: all 47 functions ported to `compile_with_builtins`, re-pinning SHAs, tails and chain counts. About ±1,000 lines of churn. | As (a) for graph. graph-compiler unchanged. | none outside the tooling row |
| **Tooling** | about −2,100, plus about 300 for the re-based ring row. `graph_fixture`, the #650 audit and the #006 bench ported to builtins. `fixtures/graph` regenerated. | Same as (a) | about −2,100, plus 300. The tools keep calling `compile` with the feature on. | about −2,100, plus about 300 if the ring row is re-based |
| **Public API** | `GraphCompiler::compile`, `GraphCompileRequest`, `GraphCompileFailure` and `PreparedGraphArtifact` become crate-private. `graph::test_only_set_output_route_fold_declined` goes (it is doc-hidden, test-support). `graph::program::lower` loses `bindable` (no caller outside the crate). The C ABI, SDK and wire are untouched. | Same as (a) | `compile` stays public behind a feature | none |
| **Realtime risk** | Low. Render-path code is only deleted. No shipped plan changes, by the section 2 predicates. One realtime-marked region goes (`lib.rs:1863-1899`). The AudioWorklet kernel census drops 15 → 14 (minimum 11). | Low | Low | none |
| **Determinism risk** | The product slice changes none of it. The fresh-process gate is self-relative. The compile-entry successor regenerates `fixtures/graph/v1/*` and its manifest; coordinate with #947, which regenerates and gates those fixtures anyway. | Same, plus many re-pinned SHAs | none | none |
| **Other risk** | #936's gates must be ported, not deleted (§3.1). `size_of::<GraphExecutor>()` shrinks by 32 bytes (two boxed slices). Totals charge only layout deltas, and removing the fields from `Runtime` and both of its witnesses keeps those deltas. The absolute size still reaches every plan's runtime-metadata estimate: it is reported as `runtime_owner_allocation_bytes` and is one candidate for `largest_allocation_bytes` (`scalar_split_runtime_owner_layout`, `crates/graph/src/lib.rs:2451`; used at `:484-536`). Run the `capi` `resource_lifecycle` test, the host-core resource tests and #947's resource-report fixtures, and re-pin whatever moves, as #936 did in `80f7ec80`. | Test weakening during a large port. Graph-layer failures become entangled with builtin bank planning. | Release binaries (`graph_fixture`, bench, audit) must enable a test-only feature, which is against #935 (`tools/bench/Cargo.toml:31`). The determinism gate, #650 and #006 keep measuring a compile no host runs. | The dead code stays. The browser module keeps shipping two kernels that cannot run. |
| **What the owner gets** | No product or tool path can build a builtins-less plan. About 900 fewer render and bind lines. A smaller browser module. Benchmarks and tool gates measure only real host shapes. Graph-layer unit tests stay cheap and focused. | The same as (a), plus about 15 lines of `None` arms gone, at a large test cost | Most of (a)'s engine win, but the mode survives as an API and a measurement target | The benchmarking complaint fixed, nothing else |

**Recommendation: (a).** It meets the owner's rule, "no code exists that no real host uses", for
both product and tooling. The graph compiler keeps one private way to test graph semantics without
builtin noise. (b) pays about 1,000 lines of test churn to remove about 15 lines of product code.
(c) keeps the mode alive as an API, a determinism target and a benchmark target.

**Order.**

1. **T1 (tooling, lands first).** Retire the two rows and re-base the driver-fed row.
2. **P1 (product slice).** Delete section 2 rows 1–7 and port #936's gates.
3. **S1.** Revert #925's alias arm (row 8).
4. **S2.** Make `compile` private and port the tools.
5. **S3–S5.** Benchmark successors.

Full bodies are in the Appendix.

---

## 5. Benchmarks

**Rows that go.**

- `sixty_four_track_plumbing_only` and `sixty_four_track_plumbing_ring`.
- #941's sparse plumbing row is already closed. #940's silence-mask successors S3–S5 are
  with-builtins and unaffected. Their sparse row should be the with-builtins one #940 already
  proposes, `sixty_four_track_console_sparse`.

**The driver-fed row becomes `sixty_four_track_gain_pan_ring`.**

- It is the gain/pan session, compiled with builtins. Its inputs are claimed by the frozen source
  set, bound through `into_bound_with_source_set`
  (`crates/builtins-compiler/src/lib.rs:2974`), with `FrozenSourceDriver` unchanged.
- The `PostInputBuiltins` bank gathers each claim in place: #918, `source_plane_table` clause (b).
  That is the production feed, the one `crates/host-core/tests/source_in_place.rs` pins.
- Today no row measures that feed. The plumbing ring measures #927's builtins-less Output read
  instead.
- Its digest must equal `sixty_four_track_gain_pan_only`'s.

**The pure-audio-path target is `sixty_four_track_gain_pan_only`.**

- Why this row:
  - It is the real no-effects session: racks empty, input sections identity-elided, and fader and
    pan as declared.
  - Its builtins are present and settled.
  - It is the C ABI's and the native host's delivery shape: concurrent, through
    `prepare_host_runtime`.
- Its floor is 22 lane-ops, 0.743 cycles per lane-sample (`effect-floor-accounting.md:442-495`,
  `:623`).
- Measured in process on one uncontrolled host: 14.6 µs/block before #944 and #945, and 10.6 after
  both. The plumbing row measured 3.2 µs/block
  (`docs/handoffs/gain-pan-2026-09-26/GAIN-PAN-VERIFY.md`, section 2).
- The gap between 10.6 µs and the plumbing row's 3.2 µs is the builtins, and the bank chains they
  run in, that the plumbing row never had. That is the gap the owner's "pure path at its floor"
  goal has to close.

**The browser's pure path is a second shape.**

- The default web boot uses between-render-calls delivery, which fuses each cohort's fader and
  matrix into one stage (#881 attempt 2).
- Every `WORKLOADS` row is concurrent (#954 amendment A6).
- #954's verification measured the product's fused gain/pan plan at 21.14 µs under V8, against
  20.57 µs for the split plan.
- Successor S4 adds `sixty_four_track_gain_pan_fused`, the no-effects twin of #955's
  `sixty_four_track_console_fused`. It becomes the browser's pure-path target.

**#938 is re-based, not dropped.**

- The cross-core producer cost it measures is real and belongs to every host.
- The row becomes `sixty_four_track_gain_pan_live`: the same session as `gain_pan_ring`, fed
  through the production source path (`prepare_graph_source_set` plus a host chunk provider), with
  a live producer on the same CCD.
- Its digest must equal `sixty_four_track_gain_pan_ring`'s.
- With builtins, the cross-core read lands in the `PostInputBuiltins` bank gather. That is where it
  lands in production.

**#936's console gates re-home to `gain_pan_ring`.**

- `the_driver_fed_plumbing_row_dispatches_only_its_output_unit` (`tools/console-workload/src/lib.rs:2268`,
  1 unit against 65) becomes: at the native width, `gain_pan_ring` dispatches exactly `units − 64` units per block, and
  `gain_pan_only` dispatches all of its units.
- The metadata-charge test (`:2309`, with the builtins-less recompile at `:2316`) recomputes on
  `gain_pan_ring`'s with-builtins graph.

**Floor accounting.**

- `tools/bench/src/floor.rs`:
  - Delete the plumbing arm (`:288-293`) and constants (`:87-111`).
  - Delete `:566`.
  - Reduce `:592` to its identity-pair half.
  - Re-base `:622` onto `gain_pan_ring` at 22 lane-ops with no control.
- `scripts/console-benchmark-record-lib.jq` restates the same.
- `docs/rulings/effect-floor-accounting.md:499-584` becomes a short section titled "Routing
  component" that says:
  - The route `mix2x2` and the master reduction (4 lane-ops) remain lines of the identity and
    builtins inventories.
  - No row is costed at them alone.
  - The floor of the table is the identity inventory (22), shared by `dispatch_only`,
    `gain_pan_only` and `gain_pan_ring`.
  - The row was retired on 2026-09-27, because no host compiles a builtins-less plan.
- The derived-floor row at `:621` becomes a component line.
- The clean control the ruling already names at `:562-568` stays available:
  `builtins_only − gain_pan_only` = 47 lane-ops.

**Record counts.**

- Native: 50 → 48 (`console-benchmark-validator.jq:7`, preflight `:160`). Console-session records:
  36 → 34 (`:12`).
- The pair-digest clause (`:33-36`) becomes the gain/pan pair.
- Wasm arm: 32 → 30 records and 16 → 15 kinds.

---

## 6. What the owner must decide

1. **Option.** (a) as recommended, or (b) or (c).
2. **The batch branch.**
   - #937 is merged on `codex/batch-plumbing-floor-2`, but its code is builtins-less only.
   - Recommendation: land the batch unchanged and delete #937's code in P1. Rewriting the batch
     would invalidate its recorded benchmark steps. #937's cost to the browser module is small and
     lasts only until P1.
   - The alternative is to revert #937 from the batch before it lands, and close #937 as
     superseded.
3. **The wasm arm's append-only index.**
   - Recommendation: accept the re-index. Records carry kind names, and the next wasm capture is a
     fresh one anyway.
   - The alternative is a placeholder row that keeps index 11.
4. **Pure-path target.** `gain_pan_only` now, and `gain_pan_fused` for the browser once S4 lands.
5. **#938.** Re-base it (recommended) or close it.
6. **The phase-profile harness** (`plumbing_profile.rs` and `graph::test_only_phase_profile`).
   Retarget it to `gain_pan_only`, or delete the hooks (S5).
7. **Backend of the ported tools.** S2 ports `graph_fixture`, the #650 audit and the #006
   benchmark to `compile_with_builtins`. `graph_fixture` and two #650 corpora compile at
   `Backend::Scalar` (`graph_fixture.rs:89`; `prepared_effect_allocations.rs:226-231`), which
   would give a bankless with-builtins plan that no host builds either. Recommendation: move them
   to `Backend::current()` in S2. That also moves the fixture bytes, so it goes in the same
   regeneration.
8. **Out of scope, flagged.** `Backend::Scalar` is itself compile-only in shipped products. The
   same "no real host" test applies to the scalar split-pair and scalar-owner lowering. It is also
   the lane oracle, though, so it needs its own study. `docs/TARGET_MATRIX.md:13` is stale.

---

## Appendix: draft issue body

```markdown
# Remove the builtins-less render path

**Proposed** (scoping study `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md`, option (a)).
Class A for every host: no plan a host can compile changes a bit or a unit.

## Product outcome

No host compiles a plan without builtins: the browser boot, the C ABI and the native host all reach
`GraphCompiler::compile_with_builtins` (`crates/host-core/src/prepare.rs:1195`) at the build's
vector width. Yet `crates/graph` carries about 900 lines of render and bind code that only a plan
with **no bank at all** can reach, and on a vector backend every with-builtins track is a bank
member:

- the fused Output route reduction (#926), its kernels (#926/#937) and its in-place source reads
  (#927, `played_planes_group`);
- the alias lowering of unlisted builtin stages (#925).

The shipped AudioWorklet module carries two of those kernels, and they can never run there. Two
benchmark rows measure this path, one of them in the wasm arm. Delete the code, retire the rows,
and point the benchmark at the real no-effects session.

## Root evidence

- Why the fold is bankless-only: `output_route_fold` declines any plan with a bank member
  (`crates/graph/src/runtime.rs:7367`; the membership includes builtin banks, `:4439-4461`).
- Why every with-builtins track banks: on a vector backend every builtin stage is a bank member,
  and short banks are padded (`crates/builtins-compiler/src/lib.rs:1290-1294, 1342`;
  `crates/rack-compiler/src/lib.rs:243-245`).
- Why the in-place Output read needs a builtins-less plan: it needs an Input whose only reader is a
  retired route (`runtime.rs:6280-6284`). With builtins, the reader is `PostInputBuiltins`.
- Why the #925 arm needs a builtins-less plan: it needs a builtin stage outside
  `required_bindings` (`crates/graph/src/program.rs:662`). `compile_with_builtins` lists all
  three stages (`crates/graph-compiler/src/compile.rs:816-832`).
- Shipped builds are `Simd8` natively and `Simd4` in the browser, which ships `simd128` only
  (`crates/lane/src/backend.rs:31-70`; `scripts/build-web-audioworklet.sh:26-30`).
  `Backend::Scalar` is reached only by tests and an unshipped CI compile.

## Companion tooling issue (file separately; lands first): retire the builtins-less console rows

Authorized paths:
- `tools/console-workload/src/lib.rs` and `tools/console-workload/tests/`;
- `tools/bench/src/console.rs` and `tools/bench/src/floor.rs`;
- `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-lib.jq` and
  `scripts/wasm-console-benchmark-validator.jq`;
- `scripts/test-console-benchmark.sh` and `scripts/test-wasm-console-benchmark.sh`;
- `scripts/run-console-benchmark.sh` and `scripts/operator/preflight-console-benchmark.sh`
  (counts only), and `scripts/operator/run-wasm-console-benchmark.sh:96` (a comment);
- `docs/rulings/effect-floor-accounting.md` (the "Plumbing inventory" section and lines 12
  and 621);
- that issue's spec.

1. Delete `Workload::SixtyFourTrackPlumbingOnly`, `Strip::PlumbingOnly`, the builtins-less branch
   of `build_full` (`lib.rs:1108-1191`) and its guard (`:1056-1066`).
2. Turn `SixtyFourTrackPlumbingRing` into `SixtyFourTrackGainPanRing`
   (`sixty_four_track_gain_pan_ring`).
   - Strip `GainPan`.
   - Prepare with `prepare_session_builtins` and compile with `compile_with_builtins`.
   - Bind with `into_bound_with_source_set` (`crates/builtins-compiler/src/lib.rs:2974`).
   - `FrozenSourceDriver` is unchanged.
3. Re-home the row's tests onto the new row, and delete the plumbing-only ones:
   - `tools/console-workload/src/lib.rs`: `:2133`, `:2186`, `:2268`, `:2309`, `:2398`, `:2538`;
   - `tests/chain_shape.rs`: `:329`, `:834`, `:890`, `:995`, `:1084`.
   Delete `tests/plumbing_profile.rs`, or retarget it (successor S5).
4. Bench, floor and jq: the gain/pan feed pair replaces `PLUMBING_FEED_PAIR`.
   - In `floor.rs`, `gain_pan_ring` takes 22 lane-ops with no control. The plumbing arm, its
     constants and its tests go.
   - Record counts go from 50 to 48, and session records from 36 to 34.
   - The pair-digest clause names `gain_pan_only` and `gain_pan_ring`.
5. Wasm arm: remove `WORKLOADS[11]`. Update the wasm validator (32 to 30 records, 16 to 15 kinds)
   and the index mutations in `test-wasm-console-benchmark.sh:165-176`.
6. Rewrite the ruling's "Plumbing inventory" as "Routing component". The 4 lane-ops stay lines of
   the identity and builtins inventories, and the identity inventory (22) becomes the floor of the
   table. Record the retirement and its reason.

Gates:
1. `gain_pan_ring`'s 64-block digest equals `gain_pan_only`'s. Its source-plane counters show
   every claim read in place by its bank gather, and none copied.
2. At the native width, `gain_pan_ring` dispatches exactly `units − 64` units per block, and `gain_pan_only` dispatches
   all of its units. This is #936's dispatch gate, re-homed.
3. The metadata-charge test passes on `gain_pan_ring`'s with-builtins graph.
4. Every other row's digest and unit census is unchanged, and
   `rg 'GraphCompiler::compile\(' tools/console-workload` is empty.
5. `floor.rs` and the jq restatement agree. `test-console-benchmark.sh`,
   `test-wasm-console-benchmark.sh` and the preflight pass. Do not run the timed runner.
6. fmt; clippy with `-D warnings`; `cargo doc` with `-D warnings`; `cargo test -p console-workload
   -p bench`.

## Smallest closable slice (this issue): delete the Output route fold family

Authorized paths:
- `crates/graph/src/runtime.rs` and `crates/graph/src/lib.rs`;
- `crates/graph/tests/rt10_source_in_place_alloc.rs` and `crates/graph/tests/MUTATIONS.md`;
- this spec.

1. **The fold.** Delete `output_route_fold` and `OutputRouteFold` (`runtime.rs:7287-7439`).
2. **The kernels.** Delete `OutputSources`, `OUTPUT_GROUP`, `route_reduce`, `route_group`,
   `route_pair_vectors`, `route_lone_vectors`, `route_tail`, `route_run`, `mix_chunk` and
   `add_mixed_chunks` (`:622-1087`).
3. **The runtime state.** Delete:
   - the `output_routes` and `output_sources` fields and their two layout mirrors;
   - the constructor parameter and its assert;
   - the `execute` routing;
   - `execute_op`'s `routes` and `sources` parameters and its third host arm, leaving two arms
     (`:3676-3684`);
   - the Output-fold arms of `validate_fold_installation` and `build_sequential`;
   - the seam and the `output_route_folds` accessor.
4. **`source_plane_table`.** Delete clauses (b′) and (e), the `output_producers` parameter and
   `SourcePlanes.output`. Rewrite the doc: a claim is bound in place only by a bank gather, or when nothing reads it
   (clause (b), `runtime.rs:6159`).
5. **`lib.rs`.** Delete `GraphSourcePlanes::played_planes_group` and its realtime region
   (`:1863-1899`), the seam export (`:36`) and the accessor (`:2572`).
6. **Port #936's four graph gates** (`runtime.rs:16433`, `:16452`, `:16484`, `:16515`) onto #918's
   banked source-fed fixture, keeping all four shapes:
   - `Plain`: an inert claim.
   - `ObservedInput`.
   - An observed elided alias, now at a rack boundary (`PostSimd1`) instead of `PostFader`.
   - `TrackDelayed`.
   Record the pre-change digests on the base commit, as `INERT_PRE_CHANGE` does.
7. **Delete the other dedicated tests** (listed in section 3.1 of the study) and the bankless arm
   of `rt10`. Mark the #926, #927 and #937 rows in `MUTATIONS.md` retired.

## Non-goals

- No change to the bank-chain route fold (#218/#915/#945), to #918's in-place bank gathers, to
  #936's skip, to #916's host planes, or to any with-builtins lowering.
- #925's alias arm is successor S1. The compile entry is successor S2.

## Objective gates

1. **Digests.** Every console workload's 64-block digest and unit census is unchanged. After the
   companion issue, every row has builtins.
2. **Host-core.** `cargo test -p host-core --all-features` is green, including the two forced-scalar
   tests. They took the fold. Now they run the route ops and the reduction, which #926 proved
   bit-identical.
3. **#936.** Its four ported gates are green on the banked fixture, with digests equal to base.
4. **#918.** `rt10`'s #918 arm is green, with zero allocations over 1,000 blocks.
5. **Nothing left behind.**
   `rg -n 'output_route|route_reduce|route_group|route_tail|OutputSources|played_planes_group|output_sources' crates/`
   finds nothing outside history text.
6. **AudioWorklet artifact.** The build script's cargo line and `scripts/check-web-audioworklet.sh`
   exit 0. The kernel census is the previous one minus one (at least 11), and the render closure is
   unchanged. The pin is repinned at the batch boundary.
7. **Resources.** The `capi` `resource_lifecycle` test and the host-core resource tests pass.
   `size_of::<GraphExecutor>()` shrinks by 32 bytes. Totals charge only layout deltas, which do
   not move, but `runtime_owner_allocation_bytes` does, and so may a `largest_allocation_bytes`
   it dominates (`crates/graph/src/lib.rs:484-536`). Re-pin only what moves, and state why.
8. **Red mutations** in `MUTATIONS.md`. Each must turn a ported #936 gate red:
   - dispatch every `SourceInput` unit;
   - skip an observed Input unit;
   - skip a `TrackDelay` unit.
9. fmt; clippy with `-D warnings`; `cargo test -p graph` with and without `test-support`;
   `-p graph-compiler`, `-p console-workload`, `-p host-core --all-features`, `-p capi`,
   `-p source --all-features`; `scripts/check-graph-determinism.sh` and #947's fixture test (the slice must not move
   `fixtures/graph/v1/*`),
   `check-graph-policy.sh`, `check-realtime-policy.sh`, `check-lane-policy.sh`.

## Console benchmark rows

No row may move a bit. After the companion issue no row reaches the deleted code, so no row may
move a unit either.

## Successors (file each as its own issue)

- **S1. Revert #925's alias arm.**
  - `compile.rs:803-835` lists the three builtin stages unconditionally.
  - `program::lower` loses `bindable` (`program.rs:544-581`, `:649-664`) and `is_builtin_stage`.
  - Delete the #925 tests (study §3.2 and §3.1), and drop the `builtins_bound` argument at its 28
    call sites.
  - The graph-compiler B test and the P tests' builtins-less arms go back to the identity-bound
    shape or are deleted.
  - Gate: every with-builtins program is unchanged. The predicate already reduces to
    `is_alias_candidate` there.
- **S2. Make the builtins-less compile a private test entry.**
  - `GraphCompiler::compile` becomes `#[cfg(test)]`, and its request, failure and artifact types
    become crate-private.
  - Move `tests/track_delay.rs` and `tests/scale.rs` in-crate.
  - Port `graph_fixture` (and regenerate `fixtures/graph/v1/*` with #947), the #650 audit and the
    #006 compile benchmark to `compile_with_builtins`, at `Backend::current()` if the owner agrees
    (study §6, decision 7).
- **S3. Amend #938** to `sixty_four_track_gain_pan_live`, on the gain/pan session. Its digest must
  equal `gain_pan_ring`'s.
- **S4. Add `sixty_four_track_gain_pan_fused`.** It is the web boot's between-render-calls
  no-effects plan, the gain/pan twin of #955, and the browser's pure-path target.
- **S5. Retarget or delete the phase-profile harness.** It covers `graph::test_only_phase_profile`
  and the deleted `plumbing_profile.rs`.

## Dependencies

- The companion issue lands first.
- Land after the plumbing-floor-2 batch (#936 and #937 merged), so the four #936 gates exist to be
  ported.

## Standing rules for the implementer

- Work only from this body. Class A: every "unchanged" gate is a hard stop.
- The render path stays allocation-, lock- and syscall-free, and `crates/graph` stays free of
  `unsafe`.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not run timed benchmarks.
```
