# #1240 attempt 1 verdict: Declare VCA groups in the session

**Verdict: PASS.** There is no BLOCKER and no MAJOR. One MINOR should be fixed before close
(a test gap with a ready probe); the NITs are optional.

- **Implementation:** `7f106a6de` on parent `963ba68ae`, branch `codex/batch-vca`.
- **Review copy:** `git archive` exports of `7f106a6de` and `963ba68ae` under
  `/tmp/claude-1002/v1240/` (deleted after review). I never touched the worktree, which #1242 is
  editing concurrently.
- **Host:** x86-64-v3 AVX2, AMD EPYC 7313P.

## Gates re-run on `7f106a6de` (all exit 0)

- **Gates 1-2:** `cargo test --locked -p session`: every target green, including the new
  `tests/vca.rs` (7) and `tests/visit_model.rs` (3), and the regenerated writer corpus
  (`canonical_writer_corpus_is_rust_generated_and_current`, not ignored).
- **Gate 3:**
  - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
  - `check-graph-determinism.sh` PASS 100/100, and `target/issue6/fresh-process-determinism.json`
    is byte-identical (`cmp`) to the one I produced from the base `963ba68ae` export.
  - `graph_fixture -- --check`, `check-console-fixtures.sh`, `check-builtins-fixtures.sh`.
  - `cargo test --locked --release -p audit -p bench -p console-workload` (only the listed pins
    moved; `ACCEPTED_GRAPH_PCM_SHA256` and the meter digests are untouched).
  - `audit capi`: `pcm_digest ff6cdcb96cdcdad5`, zero violations, identical to the base's
    `audit capi` (built from the base export).
  - The DSP-crate `--all-targets` command, `conformance_fixtures -- --check`,
    `check-protocol-wasm-parity.sh` (`COMPLETE_SCHEMA_HASH` `0x95c1_ceb6_8e44_f6e2` untouched;
    `crates/protocol/src` and `crates/conformance` have no diff).
- **Gate 4:** `check-sdk-types.sh`; fresh `build-web-audioworklet.sh --named-twin B A`;
  `check-web-audioworklet.sh`; `check-browser-expected-resources.py --self-test` and
  `--artifacts A`; `check-sdk-headless.sh A` (357 pass, 0 fail); `sdk-package.sh check A`
  (enginectl 18 pass); the browser legs under a private PulseAudio null sink with `sdk/dist`
  absent: chromium 151.0.7922.34, firefox 153.0 and webkit 26.5 "all qualification gates passed".
- **Gate 5:** CI's `test-debug-a` command (`session-validator/tests/skill.rs` included);
  `cargo fmt --check`; clippy `-D warnings`; `cargo doc -D warnings`; the session,
  protocol-control and workspace policy checks and self-tests; `check-cross-targets.sh`.
  `run-aarch64-tests.sh debug` is deferred to CI's `aarch64-debug` at the batch push, as the
  record says (this host is x86-64).

## What holds up under probing

- **Grammar.** Root key `vcas` is required (`schema.missing_field` at `$.vcas`), root field 16,
  walked between `submixes` and `outputs`; `vca` keys `id` 1, `fader` 2, `members` 3; a VCA record
  declares `2 + members.len()` fields; `members` is `array_begin`, `n` x `id_item`, `array_end`.
  `parse_id_list` refuses a non-array at `$.vcas[i].members`, a non-string item and a malformed ID
  at `$.vcas[i].members[j]`. Root field 8 stays retired (the field-key test's exact root list
  would catch a walked 8).
- **Validation.**
  - Namespace: VCAs are indexed after tracks, submixes and outputs with `Entry`, so every collision
    reports at `$.vcas[i].id` and never displaces the shadowed strip. Route source/destination,
    sidechain source and automation target lookups match concrete variants (`matches!` /
    `_ => missing`), so `GraphEntity::Vca` never satisfies them.
  - Members resolve to a track, submix or VCA; outputs and unknown IDs are
    `reference.missing_entity`; a repeat is `id.duplicate` at the later index and is skipped
    before resolution, so it adds no edge; one strip in several VCAs is legal.
  - Cycles: the iterative Tarjan is correct as written (low-link propagated to the parent before the
    root check, which is order-safe; component root found by `rposition` over exactly the
    component, so linear overall). Probes: a 12-VCA ring reports `$.vcas[0]` .. `$.vcas[11]` in
    numeric index order (`DiagnosticSet` sorts structured paths, not strings); a 20,000-deep VCA
    chain validates with no stack growth and canonicalises (4.2 MB) in 112 ms debug; closing it
    into a ring yields the 64-diagnostic cap, `$.vcas[0]`..`$.vcas[63]`. Diamonds pass.
  - Spans: every VCA diagnostic carries a source span on the text path (`"ghost"`, `30.0`,
    `"vocal"`, and the VCA object for `vca.cycle`).
  - Offsets: `validate_finite_range(-144, 24)` on both lanes; the ends are accepted.
- **Canonical form and the new walker capability.** `JsonWriter::id_item` is an ordinary array item
  (comma, newline, indentation); `StringBytes` counts it; `id_array` mirrors `sorted_array`
  (allocates only on the canonical walk of an unsorted list, never on the estimate's declared
  walk, so the preflight stays allocation-free). Rust `StableId` ordering and the SDK's
  `asciiCompare` agree on the ID alphabet.
- **Wire.** No wire message carries a VCA in this slice; the snapshot is canonical JSON. That is
  the spec's non-goal (#1241 owns opcodes `0700`-`0702`), so an unmoved `COMPLETE_SCHEMA_HASH` is
  correct.
- **Migration.** All 21 session documents: a script parsed parent and head, required
  `vcas == []` immediately after `submixes`, removed it, and compared value and key order: all
  21 equal (+14 bytes each pretty-printed, +12 for the single-line graph-compiler document). No
  JSON document with `"submixes"` outside `artifacts/` lacks `vcas`; the inline sessions
  (`support.mjs`, both protocol tests, `abi_layout.rs`, both `round_trip.rs` literals) are
  migrated; the remaining `submixes` hits are session maps, caps fields or model mutations, not
  root literals; `test-debug-a` green confirms none was missed.
- **Re-pins**, each verified: `canonical.json` sha256 `dc19e091...` (computed); `session.json`
  1955 -> 1969 bytes (`wc -c`) and the self-test row 1956 -> 1970; the capi vector length word
  `0x3763` -> `0x3771` (+14 = `  "vcas": [],\n`); MANIFEST rows and digest consistent
  (`check-builtins-fixtures.sh` and the release audit tests green). Every changed path is in the
  authorized list.
- **No output bit moved:** determinism evidence `cmp`-identical to the base; `audit capi` PCM
  digest identical to the base; every pinned graph/PCM digest unchanged.
- **SDK.** `vca(id, { fader?, members })` refuses with the engine's codes; `#graphIds` includes
  VCAs; role-specific route/sidechain/automation lookups (tracks or submixes only) cannot accept a
  VCA; the range check reads the `fader_db` metadata row, not literals; `VcaSpec` is exported
  through `export type * from "./core/types.ts"`. `membersFirst` terminates on a request cycle and
  leaves it for the builder to refuse.
- **Deviations.** All six are acceptable. D1's `$` was the spec's slip: the parser reports a
  missing root key at the key, and `$.console` (`console_schema.rs:202`) is the precedent.
  `Vca(usize)` and the non-displacing `Entry` are improvements. Deviation 4 is harmless but
  over-conservative (NIT-1).

## Mutations (mine, each in a scratch copy, reverted)

| Mutation | Result |
|---|---|
| M1: Tarjan drops the low-link propagation to the parent (the cycle's entry VCA is missed in a 3-cycle) | RED: `vca_cycles_refuse_once_per_vca_in_index_order` |
| M1b: a component counts as cyclic only at size 2 (a 3-cycle passes) | RED: same test |
| M2: `seen` persists across VCAs (a strip in two VCAs counted as a repeat) | RED: diamond/overlap, forest and cycle tests |
| M6: the repeated-member check never fires (a duplicate member counted twice) | RED: `vca_references_and_namespace_refuse_at_their_paths` |
| M7: an edge to any visited VCA marks a cycle (diamond refused) | RED: diamond, cycle and forest tests |
| S1 (SDK): the builder's repeated-member check removed | RED: builder-evals "VCAs" |
| S2 (SDK): VCA key order `id, members, fader` | RED: console-evals writer parity |
| S3 (SDK): `normalize` no longer sorts `vcas` | RED: console-evals writer parity |
| M3: `compile_session` no longer sorts each `members` | **GREEN: survives every `session` test** |
| M3b: `compile_session` no longer sorts `vcas` | **GREEN: survives every `session` test** |
| M4: VCA members leave the canonical bound's structural items | GREEN (see NIT-1) |
| M5: VCAs leave `entity_count` | GREEN (see NIT-1) |

## Test value (one sentence each)

- `vcas_are_required_and_members_are_stable_id_strings`: red if `vcas` defaults when absent or a
  member is read leniently (non-string or malformed) or refused at the wrong path.
- `random_vca_forests_round_trip_canonically`: red if the canonical writer loses, misplaces or
  fails to sort `vcas`/`members`, or the text does not re-parse to the same model.
- `vca_cycles_refuse_once_per_vca_in_index_order`: red if a multi-VCA cycle (or a self-member)
  passes, a cycle member is missed, or a VCA that only feeds a cycle is reported (M1, M1b).
- `a_diamond_and_overlapping_vcas_are_accepted`: red if a diamond or an overlap is refused (M2,
  M7). It overlaps the forest generator, which reaches diamonds too (NIT-3); the spec's gate 2
  names it, so it stays.
- `vca_references_and_namespace_refuse_at_their_paths`: red if a dangling, output, repeated or
  colliding reference is accepted or reported elsewhere (M6).
- `a_vca_is_not_a_route_endpoint_sidechain_or_automation_target`: red if a VCA ID satisfies a
  strip or output lookup (a `Some(_)` arm in any of the four).
- `vca_offsets_are_bounded_to_the_fader_domain`: red if an out-of-range or non-finite offset
  reaches preparation, or the domain's ends are refused.
- `visit_model::vca_root_field_is_sixteen_and_members_are_id_items`: red if field 8 is reused,
  `vcas` gets another ID or position, a VCA declares the wrong field count, or members are walked
  as records or unsorted on a canonical walk.
- console-evals writer parity: red if the SDK writer orders `vcas`, a VCA's keys or `members`
  differently from the engine (S2, S3).
- builder-evals VCA refusals: red if the builder emits a VCA the engine refuses or refuses with
  another code (S1).
- enginectl case: red if the CLI drops `vcas`, reads them before the strips, or declares a parent
  before its nested VCA (implementer's `membersFirst` mutation; not re-run by me, see INFO).

## Findings

### MINOR-1: `compile_session`'s VCA normalization is untested

D1 makes the canonical form "in the canonical writer and in normalization". The writer is
pinned; normalization is not: M3 and M3b (drop either sort in `compile_session`) leave every
`session` test green, because every VCA test either canonicalises (the writer re-sorts) or checks
only `parse_session_json`, which does not normalise. The normalized model is what #1242's
preparation will read (`vca_effective_db` sums "the VCAs in ascending VCA-ID order"), so a lost
sort would make preparation depend on declaration order with no test to say so.

**Fix:** add this test to `crates/session/tests/vca.rs` (it reuses the file's `with_vcas` and
`vca_json`; add `compile_session` and `CompileCaps` to the `use`). Applied to `vca.rs` in a
scratch copy it is green at head and red under M3 and under M3b; the snippet is
`rustfmt`-formatted.

```rust
/// Gate 1, normalization: `compile_session` keeps `vcas` and each `members` list sorted by ID,
/// as the canonical writer does, whatever the declared order (#1240 D1).
#[test]
fn compile_session_normalizes_vcas_and_members_by_id() {
    let caps = CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    let source = with_vcas(json!([
        vca_json("z", json!(["vocal", "a"])),
        vca_json("a", json!([]))
    ]));
    let compiled =
        compile_session(&parse_session_json(&source).expect("valid"), caps).expect("compiles");
    let vcas: Vec<(&str, Vec<&str>)> = compiled
        .normalized_model()
        .vcas
        .iter()
        .map(|vca| {
            (
                vca.id.as_str(),
                vca.members.iter().map(StableId::as_str).collect(),
            )
        })
        .collect();
    assert_eq!(vcas, [("a", vec![]), ("z", vec!["a", "vocal"])]);
}
```

*Test value: red if `compile_session` stops sorting `vcas` or a `members` list, which no
existing test catches.*

### NIT-1: the canonical bound charges a whole structural item (1 KiB) per VCA member

Deviation 4 adds every member to `structural_items` (x 1,024 bytes). Its rationale is true per
line (a one-character member's 13-byte line exceeds its 10-byte string allowance), but not in
aggregate: a VCA has at most 26 one-character members (deficit 3 bytes each, 78 bytes), and the
VCA's own 1,024-byte entity item already covers its record with room to spare, so the bound held
without the change. Measured: 300 tracks x 300 VCAs each listing every track gives canonical
1.71 MB against a bound of 96.1 MB. Harmless at real sizes (the cap check sees the bound before
the actual bytes replace it, and the defaults are 100 MB on the C ABI and 512 MiB in the browser),
and no test pins either way (M4, M5 green). Optional: charge members a tight per-line constant
(e.g. 16 bytes for indentation, quotes, comma and newline) instead of a structural item, or keep
it and say in the comment that it is deliberately loose.

### NIT-2: a cyclic `enginectl` request gets `reference.missing_entity`, not `vca.cycle`

Per D3 and the README this is the specified behaviour, but the message tells an agent that a
member it did declare "is not a declared track, submix or VCA". `membersFirst` already walks the
request; tracking an in-progress set would let it refuse a back edge with the engine's own code,
`vca.cycle`, at `$.vcas[<i>]`. Optional; would need a one-line spec/README amendment.

### NIT-3: the explicit diamond test overlaps the forest generator

Every mutation that turns `a_diamond_and_overlapping_vcas_are_accepted` red (M2, M7) also turns the
random-forest test red, because the level-based generator reaches diamonds and overlaps. The spec's
gate 2 names a diamond explicitly, so keeping it as the deterministic named case is reasonable.

### NIT-4: `SESSION_SCHEMA_V1.md` speaks of a VCA wire message that does not exist yet

"On the wire the VCA message carries `id` 1, `fader` 2 and the repeated `members` 3." No message
carries a VCA until #1241. Because the batch pushes #1240-#1246 together this is true at push;
until then, "#1241's VCA message carries ..." would be accurate.

### INFO

- The session fuzz targets seed only from `canonical.json`, whose `vcas` is empty; adding the
  writer corpus (which carries a VCA forest) as a seed would let libFuzzer start from members.
- I did not re-run the implementer's `enginectl` mutation (it would build `session-validator` in a
  second tree); S1-S3 cover the SDK writer and builder.
- `run-aarch64-tests.sh debug` is pending CI's `aarch64-debug` at the batch push.
