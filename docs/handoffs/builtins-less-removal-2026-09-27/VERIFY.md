# Verification: builtins-less removal issues #956-#961, #938 (option (b))

Tree: detached worktree of `codex/batch-plumbing-floor-2` at `4e98dbb2` (study anchors were derived on
`e2b5b9c8`; the only later merge, #954, touches no file the issues cite, and every spot-checked
anchor below matches `4e98dbb2`). All experiments ran in a scratch copy with its own target dir,
`CARGO_INCREMENTAL=0`. Nothing was pushed; no issue was edited; the console runner was not run.

## Method

A throw-away probe (`scratchpad/verify-removal/probe-instrumentation.patch`, 97 lines) logged, per
libtest thread, every time:

- `output_route_fold` admitted (`FOLD`, with fan-in and whether any builtin stage was bound);
- `source_plane_table` clause (b')/(e) admitted a claim (`SRC_OUT`, #927);
- `program::lower_with` elided an **unlisted builtin stage** (`ALIAS925`, #925).

Env switches simulated the deletions without editing code paths: `PROBE_NO_FOLD=1` forces the fold
to `None` (which also empties `output_producers`, so (b')/(e) cannot fire = #957);
`PROBE_REVERT_925=1` makes the builtins-less compile list all three builtin stages and stops the
#925 elision (= #958).

Suites per arm (CI's own commands): the debug workspace job
(`--workspace --all-targets`, CI's excludes, `--features builtins-compiler/test-support,
source/test-support,graph/test-support,engine/realtime-audit`), `--release -p audit -p bench -p
console-workload`, and `-p host-core --all-features`.

## 1. Who reaches the code #957/#958 delete (baseline census)

`FOLD` hits outside `crates/graph`'s own tests:

| binary | test | fan-in | builtins bound |
|---|---|---|---|
| host_core | `builtin_batch_endpoint::tests::forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses` | 9 | yes (Scalar) |
| host_core | `builtin_batch_endpoint::tests::endpoint_selects_existing_pair_factories_without_observer_barriers` | 3 | yes (Scalar) |
| console_workload / chain_shape / bench | plumbing rows only (`fanin=64`, no builtins) | 64 | no |
| rt10 | `an_in_place_output_read_renders_the_copy_bits_and_allocates_nothing` | 4 | no |

- No `FOLD`, `SRC_OUT` or `ALIAS925` hit in capi, host-web, builtins-compiler, source, or any
  with-builtins console row (metered/between-render-calls included) at Simd8, nor in the
  builtins-compiler `Backend::Simd4` tests.
- `SRC_OUT` (#927) fires only in graph tests, rt10's bankless arm, and the plumbing ring row.
- `ALIAS925` fires only for builtins-less compiles (graph-compiler tests, track_delay, scale,
  graph_fixture, the #650 audit's `banks64…` test, the #006 bench test, plumbing rows) and
  hand-built graph tests.

Structural reason (confirmed): `output_route_fold` returns `None` if `parts.membership()` is
non-empty (`runtime.rs:7367`), and membership includes builtin banks (`:4439-4461`). One bank
anywhere in the plan is enough, so padded short banks, mono collapse, split pairs and partial banks
do not matter. With ≥1 track at vector width `planned_strip_banks` always emits groups
(`builtins-compiler/src/lib.rs:1300-1342`). Zero tracks: fan-in 0 fails `inputs.len() < 2`.

## 2. Simulated #957 (`PROBE_NO_FOLD=1`)

- `host-core --all-features`: **all green**, both forced-Scalar tests included. Their pins
  (scalar == native bank bits, state and post-fader witnesses) hold under route ops + reduction.
- Workspace failures are exactly the tests #957 deletes or ports: graph
  `an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit`,
  `a_plain_strip_source_is_read_in_place…`, `a_claim_with_another_reader_keeps_the_copy…`, the
  four #936 gates, and rt10's bankless arm.
- Tools failures are exactly #956's rows: console-workload
  `the_driver_fed_plumbing_row_{dispatches_only_its_output_unit,renders_the_bound_rows_bits}`,
  `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables`; chain_shape
  `the_plumbing_row_is_input_route_output_and_renders_the_base_bits`,
  `the_plumbing_rows_output_fold_is_the_route_ops_own_bits`.

## 3. Simulated #958 (`PROBE_REVERT_925=1`)

Failures:

- graph: `program::tests::unlisted_builtin_stages_lower_as_aliases` and
  `tests::identity_bound_builtin_stages_alias_without_moving_a_bit` (the #925 tests, delete);
  **`runtime::tests::an_observed_source_input_stays_dispatched_and_meters_the_base_values`** (#936
  gate 2: its `ObservedAlias` shape meters an unlisted `PostFader`, which stops being an alias).
- graph-compiler: `accepted_session_compiles_binds_and_renders_direct_route` (B),
  `builtins_replace_only_the_three_internal_track_bindings` and
  `the_merged_span_hold_costs_the_input_slots_with_and_without_builtins` (2 of the 4 P tests;
  the other two stay green).
- tools: plumbing rows only. host-core, capi: green.
- `graph_fixture --manifest` is byte-identical with and without the revert, so #958 moves no
  `fixtures/graph/v1` byte.

## 4. Resource sensitivity of #957 (+32-byte executor pad)

A `[u64; 4]` added to `GraphExecutor` and both of its layout witnesses (the same parity #957 must
keep when it drops two `Box<[T]>` fields from `Runtime` and its two mirrors):

- Every suite is green: graph, capi `resource_lifecycle`, host-core all-features,
  console-workload, bench and audit.
- `graph_fixture --manifest` and `direct-route.resources.json` are unchanged.

So no native pin should move. The wasm32 shrink is 16 bytes; `hosts/host-web/tests/browser-v1/expected.json`
was not run. Run `scripts/check-browser-expected-resources.py`.

`graph_fixture --check` **already fails on the batch head** ("graph fixture manifest mismatch").
The checked-in `direct-route.resources.json` says `graph_metadata_bytes` 5336, and the head
generates 6048. So "#947's fixture test" cannot be #957's gate unless #947 lands first. The usable
proxy is `graph_fixture --manifest` equality before and after.

## 5. Option (b): the with-builtins compile is quadratic in track count

`PreparedGraphPlan::with_builtin_banks` (`crates/graph/src/lib.rs:1339-1342`) checks
`self.required_bindings.contains(member)`, a linear scan of a `Vec` of about 4 × tracks entries, for
every bank member (3 × tracks). Scratch test `crates/graph-compiler/tests/scale_builtins_probe.rs`
(since deleted) ran scale.rs's session through `compile_with_builtins`:

| tracks | release, Simd8 | with a `BTreeSet` lookup instead |
|---:|---:|---:|
| 8,192 | 3.94 s | – |
| 16,384 | 9.94 s | 2.26 s |
| 32,768 | 37.4 s | – |
| 65,537 | **158.6 s** | **11.2 s** |

- Debug at 65,537 tracks: 564.7 s at Simd8, 23.2 s at Scalar, with 1.13 GB peak RSS. Today's
  builtins-less `scale.rs` takes 30.6 s in debug.
- The quadratic is on every host's compile path. It stayed hidden because the only scale gate uses
  the builtins-less compile at Scalar, where no bank attaches.
- A faithful option-(b) port of `tests/scale.rs` and of the #006 bench's
  `graph_validate_65537_tracks` row needs this fixed first.

## 6. Other completeness gaps

- `tools/audit/src/graph.rs:256` (`audit graph`) and `tools/audit/src/source.rs:374`
  (`audit source`) hand-build builtins-less plans (`PreparedGraphPlan::new`). CI runs both at
  `qualification.yml:693-694`, and no issue covers them. `audit builtins-graph`
  (`tools/audit/src/builtins_graph.rs:566`, `Backend::current()`) already audits the
  with-builtins graph.
- `graph::PreparedGraphPlan::new` plus `bind` stays a public builtins-less constructor. It is a
  residual back door for any Rust embedder, and the builtins and graph compilers need it.
- Hand-built builtins-less plans also live in tests: `crates/graph` (all of its tests, rt1, rt9,
  rt10), `crates/source` (`lib.rs:3334`, `native_source.rs:3261`, `:3657`), and builtins-compiler
  (`lib.rs:5138`, `:6384`). `graph` cannot dev-depend on graph-compiler without a crate cycle,
  and those tests inspect `pub(crate)` internals. Scope them out explicitly, because they are not
  compiles.
- #959's gate 1 regex, `fn compile\(`, also matches six unrelated functions:
  `host-core/src/control_provider.rs:620`, `wasm-console/src/main.rs:299`,
  `native-pcm-runner/src/lib.rs:177`, `:1195` and `:1589`, and `graph-compiler/tests/track_delay.rs:134`.
- **The `Backend::Scalar` back door (out of scope, flagged).** With-builtins plans at Scalar are
  bankless, and they are the only with-builtins plans the fold ever reached. They are still built
  by:
  - host-core's two tests (`builtin_batch_endpoint.rs:1575`, `:2122`);
  - graph-compiler tests (17 `Backend::Scalar` uses) and builtins-compiler tests (48);
  - the #650 audit's `Zero64` and `CrossedSmall` corpora (`prepared_effect_allocations.rs:229`),
    until #959;
  - `graph_fixture.rs:89`;
  - the CI scalar wasm compile of host-core and host-web (`qualification.yml:755-757`).
  Scalar-only product code includes the split-pair runtime (`builtins-compiler/src/lib.rs:2313`).
  `docs/TARGET_MATRIX.md:8` and `:13` are stale.

## 7. Test preservation (#936 → #918 fixture)

- `SourceShape` (`runtime.rs:13863-13886`) has `delayed` (TrackDelayed), `observed`
  (ObservedInput) and the default banked claim (Plain).
- It has **no unbanked alias stage**. #936's ObservedAlias needs an elided alias whose buffer
  *is the claimed Input's*: in `ring_output_parts` the chain is Input → unlisted `PostFader` →
  Route (`:15609-15617`). In a with-builtins-ordered chain, `PostSimd1` follows `PostInputBuiltins`
  and aliases the bank member's buffer. The input unit is then inert, and the shape degenerates to
  Plain.
- The port needs a new `SourceShape` option: a `PostSimd1` alias between the Input and its first
  consumer. The `routed` track (Input → alias → Route) expresses it.
- `INERT_PRE_CHANGE` was recorded on `64b155d0`, whose loop dispatches every unit, and that tree
  already has #918's fixture. Recording the ported digests on #957's base, which already has
  #936, would make "moves no bit" circular.
- #927's `DeadClaim` shape is the only test of clause (b)'s zero-reader arm with a recoloured slot,
  and that arm stays live. Port it or retire it explicitly.

## 8. Targets

- Every row but `plumbing_ring` is bound-feed. `SourceFeed::Bound` means "one dispatched unit per
  track per block, which copies the track's frozen block into its arena buffer"
  (`console-workload/src/lib.rs:426-427`), and no host pays that.
- The C ABI, the native host and the browser all feed through source sets, where #918 gathers in
  place and #936 skips the input unit.
- Production shapes:
  - native: `gain_pan_ring` (concurrent delivery plus played planes);
  - browser: between-render-calls fused, driver-fed and, as the default boot selects, metered.
    #954 measured the fused plan *slower* under V8: 21.14 µs against 20.57 µs.
