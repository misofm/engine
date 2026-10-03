# #1246 attempt 1 verdict: Enumerate VCA groups and drive them from the SDK

**Verdict: PASS.** No BLOCKER, no MAJOR. The export order is the admission order by construction,
the SDK encodes kinds 16 and 17 exactly as it encodes kinds 3 and 4, the unknown-VCA refusal carries
the engine's reason name, the session map is one shape across the worklet, the host validator, both
`.d.ts` copies and the SDK, and the D1 deviation is sound and budget-neutral. **The `AGENTS.md`
qualifier is removed** (D7), and nothing else in that file changed. One MINOR (a hermetic-stub
blind spot that lets a plausible copy-paste defect in the new worklet reader pass every gate, with a
verified one-line fix) and three NITs.

- **Implementation:** `5248f94c4` on parent `86c050075` (#1245), branch `codex/batch-vca`.
- **Review copies:** `git archive` exports of `5248f94c4` under `/tmp/claude-1002/v1246/` (`src` for
  the gates, `mut` for mutations, each with its own target directory), node_modules copied from the
  worktree. All deleted after review. The worktree, branch and GitHub were not touched.
- **Host:** x86-64-v3 AVX2, 32 cores.

## Gates re-run on `5248f94c4` (all exit 0)

| Gate | Result |
|---|---|
| test-debug-a (gate 6 `cargo test` command, `--no-fail-fast`) | 115 binaries: 1,288 passed, 0 failed, 9 ignored (matches the record); both new Rust tests ran green |
| `cargo fmt --all -- --check`; clippy `--workspace --all-targets --all-features -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | clean |
| `check-session-map-shape.py --self-test` and plain | 23 mutations caught; one shape |
| `check-abi-layout-v1.py --self-test` and on `A/…abi-layout.json` | 22 caught; ok. The artifact JSON's `exports` is 122 names, sorted |
| `build-web-audioworklet.sh --named-twin B A` | **ARTIFACT CHANGED**, reproducible: shipped `5d21f73e…2675` (2,849,413 B), named twin `386b0651…4743` (3,252,917 B), both byte-for-byte the record's |
| `check-web-audioworklet.sh A B/…named.wasm` | ok (exact export list, callgraph, boot high-water) |
| `check-browser-expected-resources.py --artifacts A` | ok, 32 red self-test mutations, no re-pin |
| `test-web-audioworklet.sh` | ok (hermetic suite, kind/reason vocab, session-map on-disk drift) |
| `check-sdk-generated.sh A`, `check-sdk-types.sh`, `check-sdk-headless.sh A` (360 pass, 0 fail), `sdk-package.sh check A` | ok |
| Browser legs `npm run qualify -- --artifacts A --sdk-root sdk --browser X --check-matrix --self-test-mutations`, private PulseAudio null sink, stray `sdk/dist` deleted first, SDK source-bundle mode (CI's) | chromium 151.0.7922.34, firefox 153.0, webkit 26.5: "all qualification gates passed" |
| `check-`/`test-` policy pairs for host-core, realtime, workspace | all rc 0 |
| `check-cross-targets.sh` | rc 0, "cross-target matrix: PASS"; only the known #1018 `memset_pattern16` expected failures (transient-shaper, true-peak-limiter) |
| `.d.ts` mirror | `cmp`-identical |

Gate 7 (batch boundary) was not re-run; the record lists it green and `aarch64-debug` "at batch push".

## What holds up

- **Export order equals the VCA index order (D1).** `_vca_count` is `ready.vcas.vca_count()`, the
  exact bound admission refuses kind 16/17 against (`lib.rs:4898`). `_vca_id(i)` copies
  `ready.session.normalized_model().vcas[i].id`. `LiveVcaState::try_new(model)` indexes VCAs by
  position in that same `model`, and `model` is `session.normalized_model()` of the very
  `CompiledSession` that is then moved into `ReadyOwnership.session` (`lib.rs:6816`, `:6979`, the
  `ReadyOwnership { .. session }` literal). `ReadyOwnership.session` is never reassigned. The
  normalized model's `vcas` is ID-sorted (`visit.rs` `sorted_array(f::VCAS, ..)`), and the SDK eval
  boots a document declaring `drums, fx, band` and reads `band, drums, fx`.
- **D1 deviation is sound and budget-neutral.** No second ID list is retained, so no bridge bytes
  are added and the #1245 exact retained-budget tests stay unchanged; it follows the
  `copy_session_source_id` precedent and cannot disagree with the live state's order. Without live
  controls `LiveVcaState::empty()` makes both exports answer 0, matching admission (every VCA kind
  refused).
- **D2 staging.** `longest_vca_id_bytes` joins the `.max` chain in `lib.rs` and the three `tests.rs`
  mirrors. My mutation M8 (drop the VCA term from `lib.rs` only) turns
  `live_vca_ids_enumerate_in_vca_index_order_through_staging_sized_for_them` red (the
  `id_staging_bytes` assertion; without it the copy's `copy_from_slice` would panic).
- **Encoding (D4).** `VcaEdits.faderDb`/`.mute` are `StripEdits.faderDb`/`.mute` with the kind name
  swapped: `lane(options)`, `smoothing(options)`, `builtinNumber("fader_db", db)` / `enabled ? 1 : 0`,
  `rack` 255 via `trackEdit`. The generated vocabulary maps `vcaFaderDb` 16 and `vcaMute` 17. Gate 1
  asserts the browser request words exactly (`kind`, `rack`, `channel`, `trackIndex`,
  `smoothingSamples`, `values`) for both-lane, left-lane and right-lane records.
- **Unknown VCA.** `MisoUsageError`, `diagnosticCode` `unknownVca` (the engine's reason-14 name),
  listing the enumerated VCAs or `none`; a member track ID and a submix ID are both refused.
- **kindNames (D6).** `vcaFaderDb`/`vcaMute` are in `kindNames`, `kindsAwaitingSdk` is gone, and the
  vocabulary eval compares the hand list to the generated kinds exactly.
- **Session map `vcas` consistency (D3).** Worklet reader (route rules: `u32`, non-zero, at most
  `sourceIdCapacity`, ASCII) and reply field; host JS exact field list and `Array.isArray` + non-empty
  string validation; `MisoSessionMap.vcas` in both `.d.ts`; SDK `SessionMap.vcas` (headless
  `#liveVcas()` through the exports, browser copy of the reply); every stub. The shape checker gains
  the spec's named mutation plus worklet and `.d.ts` ones.
- **AGENTS.md (D7).** The sentence "Approved by decision 13 (#1196) as owner-delegated answer (a),
  landing with the *VCA groups* umbrella, which is filed when batch K3 of #1196 closes: no VCA group
  exists yet." is gone; the diff is that one sentence and nothing else. No other "landing"/"not yet"
  VCA qualifier remains in `AGENTS.md`.
- **APP-LIVE.md** "Live VCA groups (#1246)" is accurate against the code: `session(..).vca(id,
  { members, fader? })` matches `VcaSpec`; `LaneOptions` is `channel` + `smoothingSamples`;
  `fader_db` is `[-144, 24]`; `OfflineEngine.sessionMap()` exists; the solo/mute/follow statements
  match the umbrella's rules 5 and 7; wire kinds 16/17 and reason 14 are right. It makes no C ABI
  claim, so #1247 not having landed does not make it wrong.
- **Authorized paths.** Every touched file is inside the spec's list (`crates/host-core/tests/MUTATIONS.md`
  is under the authorized `crates/host-core/tests/`).

## Mutations of my own (in the `mut` export; each restored after)

| # | Mutation | Result |
|---|---|---|
| M1 | worklet reads `vcaCount` from `miso_engine_web_v1_live_control_route_count` (a copy-paste of the route block it was cloned from) | **survives** `test-web-audioworklet.mjs` (stub has 2 routes and 2 VCAs), and the browser legs cannot see it (no qualification session declares routes or VCAs) -> finding m1 |
| M4 | `VcaEdits.faderDb` ignores the lane option (`channel: CHANNELS.both`) | red: gate 1, `a VCA edit encodes its kind at the engine's VCA index` |
| M6 | worklet VCA reader drops its `> 0x7f` ASCII check | survives; the track, submix and route readers have no non-ASCII stub mutation either (pre-existing, out of this slice's gate 4) -> NIT n3 |
| M7 | host validator accepts an empty VCA ID (drops `value.length > 0`) | red: `the host accepted a session map with an empty VCA ID` |
| M8 | `lib.rs` ID staging drops `.max(shape.longest_vca_id_bytes)` | red: `live_vca_ids_enumerate_…` (`id_staging_bytes` 73 expected) |

## Test value (one sentence each)

- `vca_caps::the_session_shape_measures_the_longest_vca_id`: red if the shape measures no VCA, or only
  the first or last one (the 46-byte ID sits between `a` and `zz`), which the host-web test cannot
  see because its long ID is also the canonical last.
- `live_vca_ids_enumerate_in_vca_index_order_through_staging_sized_for_them`: red if staging ignores
  VCA IDs (M8), or the exported index `i` is not the VCA a kind 16 at `i` composes (distinct reach sets).
- Gate 1 "a VCA edit encodes its kind at the engine's VCA index": red if the SDK indexes by any order
  but the engine's or writes a wrong kind/lane/value word (M4).
- "an unknown VCA ID refuses with unknownVca": red if an unplaceable ID (typo, member strip, submix)
  reaches an index or the code is not the engine's.
- Gate 2 "a live VCA fader and mute equal the session booted…": red if the shipped module's admission
  rides the wrong VCA (the record's E1), which gate 1 cannot see.
- Type tests: red if `vca()`'s surface grows (`solo`, `pan`, `effect`) or `vcas` becomes optional.
- Stub mutations (empty / too-long VCA ID, four malformed replies) and the three shape-checker
  mutations: red if the worklet accepts an unreadable VCA ID (or the host validator a malformed list)
  or the three session-map spellings drift (M7).

## Findings

### MINOR

**m1. The hermetic stub enumerates as many VCAs as routes, so a worklet that reads the VCA count from
the wrong export passes every gate.** `scripts/test-web-audioworklet.mjs:2176` has
`const vcaIds = ["zz-vca", "aa-vca"];` beside `routeIds = ["zz-send", "aa-send"]`. M1 (the worklet's
`vcaCount` read from `_route_count`, a plausible slip in a block copied from the route reader)
leaves the delivered map equal to `fake.vcaIds`, and no browser qualification session declares a
route or a VCA, so the real module never exercises the reader with a non-zero count either.
**Fix (verified):** enumerate three VCAs, e.g. `const vcaIds = ["zz-vca", "mm-vca", "aa-vca"];`
(still out of canonical order). The suite stays green on `5248f94c4` and turns M1 red at
`issue #1246: the enumerated VCA order`. Add an `hosts/host-web/MUTATIONS.md` row for it.

### NIT

**n1. The native enumeration test's doc overstates what it exercises.** It says the VCAs are
"declared out of canonical order" and "declaration order … differ[s] from canonical order", but
`vca_ride_host` and `test_boot` boot `canonical_session_json(&model)`, which sorts `vcas`, so the
engine never sees a non-canonical declaration there. The order-independence claim is carried by the
SDK eval's `vcaDocument()` (which really is non-canonical). Reword to "the model lists them out of
canonical order; the canonical document the host boots sorts them", or boot a hand-reordered JSON.

**n2. D1's text and the code differ.** The deviation is sound (above); when the root accepts it,
amend D1 in the spec ("`_vca_id` copies from the retained normalized model the live VCA state was
built from; no list is copied") so the frozen decision matches the code.

**n3. No non-ASCII stub mutation for any enumerated ID** (M6 survives for VCAs; tracks, submixes and
routes share the gap). Pre-existing and outside gate 4; a one-row follow-up (a stub `_vca_id` that
writes `0xe9`) would cover all four readers' pattern.

## Required before close

- m1 (one line in the stub plus a MUTATIONS row). n1-n3 at the root's discretion.
