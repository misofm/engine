# #1222 attempt 1 verdict: Admit live send commands in the browser

**Verdict: PASS.** There is no BLOCKER and no MAJOR. Five MINORs must be fixed before close. MINOR-1 to MINOR-4 are test gaps in host-web's wiring, and drop-in probes for them are attached. MINOR-5 is a misplaced doc comment. The NITs are optional.

- **Implementation:** commit `466f0ab63` on parent `1c5ce5d02`, branch `codex/batch-submix-k3`.
- **Review copy:** an exported copy (`git archive`). I never touched the worktree.
- **Host:** x86-64-v3 AVX2, AMD EPYC 7313P.
- **Probes:** `1222-attempt1-verifier-scratch.rs` in this directory. Append it to `hosts/host-web/src/tests.rs`. All four probes are green at head.

What holds up under probing:

- **Kinds 13-15 and reason 13 are in every spelling.** That covers:
  - the Rust constants and the decode whitelist;
  - the host JS set and table;
  - both `.d.ts` copies, which are byte-identical;
  - both generators, both schema-gate lists and both self-test fixtures;
  - `sdk/assets` (`cmp`-identical to the built artifact's JSON) and `sdk/src/generated`.

  No retired value is reused: I searched the history of `lib.rs`, and no `COMMAND_*` constant ever had the value 13, 14 or 15.
- **The self-test re-anchors still test what they claim.** I traced the rule that turns each mutation red:

  | Mutation | Rule that turns it red |
  |---|---|
  | `COMMAND_SOLO_MODE = 16` | `.d.ts MisoCommandKind disagrees` (the threading rule, not contiguity) |
  | JS set `…, 14, 16` | `host JS COMMAND_KINDS set disagrees` |
  | literal `<= 15` | the derived-bound rule |
  | whitelist drops `routeMute` | the whitelist/constant set rule |
  | `FUTURE_TAP = 14` | `host JS table disagrees` |
  | my extra: `UNKNOWN_ROUTE` renumbered to 12 | contiguity |
- **Admission validates first and then admits atomically.**
  - Pass one decodes every record, builds every send record with `RouteControlProducer::record`, and moves the mirror only under its shadow.
  - The room check then covers every staged entry. That includes the strip, input, effect and send bands, and the send band is checked with `free()`.
  - Only after that does anything push.
  - `admit_commands` commits the send mirror exactly where it commits solo, and rolls it back on every refusal path.
- **The bounds check is per kind.**
  - Strip kinds are checked against the strip count before dispatch.
  - Send kinds are checked against the live routes in their own arm.
  - I found no index confusion.
- **DOMAIN and MALFORMED are split correctly.**
  - MALFORMED: a non-finite value (at decode), and any wrong `rack`, `channel`, `effect_index`, `parameter_id` or extra value word.
  - DOMAIN: a non-boolean `routeMute`, a value `route_coefficients` refuses, or a ramp longer than `1 << 22`.
- **Follow-mute seeding has the prepared semantics.** The mirror seeds `source_lane_muted` from `solo.effective_mute`. My probe 1 covers:
  - following pre-fader sends from sources with lane mutes `[F,F]`, `[T,F]`, `[F,T]` and `[T,T]`;
  - a following send from a bus with its right lane muted;
  - two non-following controls.

  Random kind 13/14/15 edits at smoothing 0 and 480 land bit-for-bit on a fresh plan, in 12 cases. Probe 2 runs the same edits with an insert on every track.
- **The acked-batch question: no ack precedes a drop.**
  - The room check runs before every push.
  - Control and render share the worklet thread, so nothing moves between the check and the push.
  - The browser never swaps a plan. `route_controls` lives in `ReadyOwnership` beside the plan, so a queued, acked record cannot outlive its consumer.

## Gates re-run (head `466f0ab63`)

| Gate | Result |
|---|---|
| kind vocabulary `--self-test` / plain / `--artifacts <A>` | rc 0 (32 red) / rc 0 / rc 0 |
| reason vocabulary `--self-test` / plain | rc 0 (20 red) / rc 0 |
| parameter metadata `--self-test` / `<A>/…parameter-metadata.json` | rc 0 / rc 0 |
| ABI layout `--self-test` / `<A>/…abi-layout.json` | rc 0 (22 caught) / rc 0 |
| `test-web-audioworklet.sh` | rc 0 |
| `build-web-audioworklet.sh --named-twin <B> <A>` | rc 0. Shipped module `2daf0f83…`, 2,762,489 B, as recorded. |
| `check-web-audioworklet.sh <A> <B>/…named.wasm` | rc 0. Kernels 13, `f32x4_arith=9396`. Render `closure=8 traps=5`, with the sole trap owner `render_inner` (unchanged). `command_submit` `closure=48`, allocation-free. |
| `check-browser-expected-resources.py --artifacts <A>` | rc 0. Digests and exact rows agree; the self-test shows 32 red. |
| `check-scalar-oracle-absent.py --wasm` | rc 0 |
| `check-sdk-generated.sh <A>` / `check-sdk-types.sh` / `check-sdk-headless.sh <A>` | rc 0 / rc 0 / rc 0 (352/352) |
| host-core, realtime and workspace policy, with their `test-*` twins | All ok. Realtime reports 54 regions in 15 files. |
| test-debug-a (DESIGN 7, `--no-fail-fast`) | rc 0. **1,220 passed, 0 failed, 9 ignored**, over 108 binaries. That is the record's 1,221 minus the folded test. |
| `cargo fmt --all -- --check` | rc 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | rc 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` (extra) | rc 101, only #1221's BATCH-1 (`graph-compiler` `route_transform`). With `--exclude graph-compiler` it is rc 0, so #1222's docs are clean. |
| `run-aarch64-tests.sh debug` | Not run: there is no arm64 host. host-web is not in that script, and `LiveRouteState` has no width dependence. |

## Mutations

The implementer's rows I re-ran:

- **I1** (the generic bound restored ahead of dispatch) is RED on gate 3, gate 1 and gate 7.
- **I2** (the mirror committed as each send record is staged) is RED on gates 2 and 3.

My own mutations, each applied alone across every host-web target and then reverted:

| # | Mutation | Existing tests | Probes |
|---|---|---|---|
| V1 | `compile_ready` seeds the mirror with `&\|_, _\| false` instead of `solo.effective_mute` | **all GREEN** | probes 1 and 2 RED |
| V8 | the record is built with `[false; 2]` instead of `next.source_lane_muted` | **all GREEN** | probes 1 and 2 RED |
| V2 | `route_slot` omits `+ self.effect_controls.len()` | **all GREEN** | probes 2 and 3 RED |
| V6 | `ready.routes.commit()` is dropped on success | **all GREEN** | probe 4 RED |
| V3 | the bridge rows charge `route_control_resources.queue_bytes` instead of `total_bytes` | **all GREEN** | none |
| V4 | the mirror and shadow bytes are dropped from the bridge rows | **all GREEN** | none |

## Findings

### MINOR-1: no host-web test defends a following send's seeded source lanes

V1 and V8 are green on every test. Gate 1's session has no following send and no muted source. The host-core unit test defends only that `try_new` uses the closure it is handed. Nothing defends:

- that host-web hands it the right closure (`lib.rs:6293`);
- that the record carries `next.source_lane_muted`.

The regression this misses is audible. A live gain, matrix or unmute edit on a pre-fader send that follows a muted strip would build `mute = false` with full columns. The muted channel would then leak into the aux bus while the fresh plan keeps it silent. Gate 1's test-value sentence claims "a record built from a stale mirror field", so this is an overclaim for this field. *Follow-mute* (#1224) builds directly on the seed.

**Fix:** add probe 1 (`verifier_follow_mute_seed_lane_combos`), or fold its session into gate 1. It needs following pre-fader sends from sources with all four lane-mute combinations, plus a bus source with one muted lane, compared bit-for-bit with a fresh plan. Probe 1 is red under V1 and V8.

### MINOR-2: the send band's offset `3S + E` is never tested with `E > 0`

V2 is green on every test:

- Gate 1's session and the budget session have no effects (`E = 0`).
- Gate 3's second session has `E = 1`, but it asserts only `RESULT_OK`, never which queue received the record.

In production almost every session has console slots or inserts. Under V2:

- a send record lands on another send's queue, so the wrong send moves;
- for `r < E` the room check reads an effect queue and the push fails with `RESULT_INTERNAL` after earlier records in the batch were pushed. That is a partial batch, with the mirror rolled back under records that were applied.

The D4 layout is the slice's core claim, and gate 1's "addresses the wrong send" covers it only at `E = 0`.

**Fix:**

- Add probe 2 (`verifier_send_band_after_effects`): an insert on every track, random edits, and an assertion that each send's queue room drops by exactly its own records, then fresh-plan bits.
- Add probe 3 (`verifier_effect_queue_full_blocks_send`) to gate 2: a send record plus an effect record with that effect queue full push neither.

### MINOR-3: host-web's commit of the send mirror on success is not defended

V6 is green on every test:

- `assert_send_refusal` checks only refusals on fresh hosts.
- The host-core test checks `LiveRouteState`'s own commit, not host-web's call.

Under V6, a refused batch after an admitted one rolls the mirror back past the admitted values. The render plane has applied them. Every later record of that send is then built from stale values: an audible jump, and exactly the "refused batch leaks" class gate 2 is meant to catch.

**Fix:** add probe 4 (`verifier_a_refusal_after_an_admission_keeps_the_admitted_mirror`) to gate 2. It runs an admitted gain edit, then a refused batch, and asserts:

- the mirror holds the admitted gain;
- a further edit settles on a fresh plan with the admitted values.

### MINOR-4: the exact-retained test defends only the depth-dependent queue bytes

**What is correct.** The accounting itself (deviation 3) is right, and I found no double counting:

- `graph_session_plus_plan_bytes` is the graph estimate without the route total.
- The producer `Vec` is built with `with_capacity(len)` and collected in place into the `#[repr(transparent)]` wrapper, so its table bytes are inside `total_bytes` once.
- The mirror and its shadow are exact boxed slices.

**What the test misses.** `the_exact_retained_budget_charges_the_send_lanes` compares only the change between depths 8 and 64. Only the queue part of `total_bytes` changes with depth. So these mutations are green:

- V3: owners, producer table, IDs and the activity table all go uncharged.
- V4: the mirror and its shadow go uncharged.

The "one byte below is refused" check is self-consistent with the report and cannot see an undercount. The test-value sentence ("red if the browser leaves the route lanes (queues, owners, producer table, IDs, activity table) out") overclaims.

**Fix: one of the two.**

- **(a) Add an absolute check.** Boot `send_document` and a twin with send `send-g` removed, at the same depth. Assert that `bridge_retained` differs by exactly:
  - the twin pair's document-dependent bridge rows;
  - plus the independent preparations' `session_model_bytes` difference;
  - plus the `route_control_resources.total_bytes` difference;
  - plus `2 * size_of::<LiveRoute>()`.
- **(b) Narrow the claim.** Narrow the sentence to the queue bytes, and add a check that the bridge rows include `2 * R * size_of::<LiveRoute>()`.

(a) is preferred.

### MINOR-5: `into_route_edit` was inserted inside `into_solo_request`'s doc comment

At `hosts/host-web/src/lib.rs:4207-4246`, the paragraph "Read one `solo` record's requested bit…" now heads `into_route_edit`'s docs, and `into_solo_request` has none.

**Fix:** move the solo paragraph back above `const fn into_solo_request`.

### NIT-1: gate 3 tests one index twice

`for route in [3_u32, STRIP_BUS]` (`tests.rs:10981`) runs index 3 twice, because `STRIP_BUS == 3`. With 3 sends and 4 strips, index 3 is the only index below the strip count that is not a send. The record's "index 3 and the bus's strip index" are one case.

**Fix:** reword the record, or loop over `[3]`.

### NIT-2: the producer-order check short-circuits on an empty producer list

`live_routes_in_order` (`lib.rs:6278`) passes on `route_controls.is_empty()` even when live controls exist and the model has live routes. A host-core regression that attaches no lanes would then make every send `unknownRoute`, not fail at boot.

**Fix:** allow emptiness only when `handles.strip_controls` is empty.

### NIT-3: `UnknownSource` gets the wrong diagnostic

`LiveRouteStateError::UnknownSource` maps to `web.resource.allocation`.

**Fix:** use `web.live_controls.routes`.

### INFO: two per-queue counters are uncharged, and the send band grows them

`command_wanted` and `in_flight` are `queue_count` `u32`s each. I found no charge for either in any bridge row, and the gap predates #1222. #1222 grows them by 8 B per live send, so the code comment "every byte the route lanes add" is 8 B per send short.

**Route:** a follow-up issue on host-web's per-queue arrays, not this slice.

## Deviations

1. **`#[inline(always)]` on the two `free()` accessors. Accepted, with a checker follow-up.**
   - **Verified necessary.** I rebuilt the module without the two attributes. The `command_submit` allocation gate then fails on exactly `RouteControlProducer4free` and `GraphRouteControlProducer4free`. Both bodies are atomic loads.
   - **The rule is a false-positive hazard, not a real one here.**
     - `FORBIDDEN` matches the substring `free` anywhere in a mangled name.
     - On `wasm32-unknown-unknown`, the Rust allocator's symbols are already matched by `dlmalloc`, `dealloc` and `__rust_alloc`.
     - A bare `free` can only legitimately be a C allocator's unmangled symbol.
   - **The workaround fails safe.** If inlining ever lapses, the gate goes red (a false alarm). It never misses an allocation.
   - **Follow-up issue:**
     - anchor the C names (`^(free|malloc|calloc|realloc)$`) and keep the crate and shim names;
     - add self-test cases: an out-of-line accessor named `free` passes, while `<free>` and `dlmalloc…4free` fail;
     - then drop both attributes.
   - **Root must ratify:** the two edited files (`crates/graph/src/lib.rs` and `crates/host-core/src/route_controls.rs`) are outside the authorized paths.
2. **`kindsAwaitingSdk` in `sdk/test/live-controls-evals.mjs`. Accepted.**
   - It is outside the authorized paths, and it is needed to keep `check-sdk-headless` green.
   - It partly enforces its own removal. Once #1223 adds the three names to `kindNames`, the duplicates fail `deepEqual`. But if #1223 never adds them, the exception lives on silently.
   - #1223's spec does not mention it today. **Root amends #1223's brief:** "delete `kindsAwaitingSdk` and add `routeGainDb`, `routeMute` and `routeMatrix` to `kindNames`" becomes a named deliverable.
3. **#1221 MINOR-1 accounting.** Correct, with no double counting. The test is weaker than claimed (MINOR-4).
4. **Gate 2's finite overflow (`3e38` at +2 dB). Accepted.** Decode refuses any non-finite value as MALFORMED before dispatch, which is a pre-existing and correct rule. So DOMAIN can only be reached through the fold.
5. **The constructor takes `&SessionModel`. Accepted.** The live-route selection and the source-strip resolution are written once. `effective_mute` is the frozen `&dyn Fn(usize, usize) -> bool`, and #1226 can seed it from capi's model.
6. **`unsupportedKind` without live controls. Accepted.** It matches the strip kinds.

## Test value (one sentence per new test)

- **`construction_seeds_every_field_from_the_session_and_the_effective_mute`:** red if the mirror keeps an output route, seeds following lanes from the session instead of the closure, or resolves a submix source without the track offset (H1, H2, H4).
- **`a_rollback_restores_every_field_and_a_commit_keeps_them`:** red if the shadow is not retaken per transaction (H3).
- **`a_live_send_edit_lands_on_a_fresh_plans_bits`:**
  - red if the matrix words are reordered, a record reaches another send's queue at `E = 0`, or the mirror's gain, matrix or mute is stale (M1, M1b, M2, M3, I1);
  - not red for a following send's lanes (MINOR-1) or for the band offset with `E > 0` (MINOR-2).
- **`a_refused_send_batch_pushes_nothing_and_keeps_the_mirror`:**
  - red if a send record is pushed or the mirror committed before every record and queue is checked (M6, M7, M8, I2);
  - not red if host-web never commits on success (MINOR-3).
- **`every_kind_is_bounded_by_the_count_it_addresses`:** red if the bound runs before dispatch, a send reuses `unknownTrack`, or the send arm is bounded by the strip count (M4, M5, M12, I1).
- **`send_records_are_shape_checked`:** red if a send kind accepts a lane, rack, effect or parameter word, an extra value, a non-boolean mute, or an over-long ramp (M10).
- **`send_edits_admit_and_render_without_allocating`:** red if send admission or the mirror allocates on the render-call path (M9).
- **`the_exact_retained_budget_charges_the_send_lanes`:**
  - red if the route queues go uncharged (M11);
  - not red for the owners, table, IDs, activity table or mirror (MINOR-4).

## Required before close

1. MINOR-1, MINOR-2 and MINOR-3: add probes 1-4 (or equivalents) to the host-web tests, with `MUTATIONS.md` rows V1/V8, V2 and V6.
2. MINOR-4: strengthen or narrow the exact-retained test (fix (a) is preferred).
3. MINOR-5: restore `into_solo_request`'s doc comment.
4. Root: ratify deviation 1's two out-of-path edits and file the callgraph-checker follow-up. Amend #1223's brief to remove `kindsAwaitingSdk`.
5. Carried from #1221: BATCH-1 (the `graph-compiler` doc link) still fails the rustdoc lint step and must be fixed before the K3 push.
