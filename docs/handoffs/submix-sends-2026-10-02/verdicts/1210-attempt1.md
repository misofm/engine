# #1210 *Name submix strips in the browser session map and the SDK measurement*: Sol verdict, attempt 1

- **Reviewed:** `git diff 7f6e148f6 66b2c7dd7` on branch `codex/batch-submix-k2` (worktree
  `/home/bl/misofm/wt-submix-k2`). 30 files, +681/-49.
- **Binding:** `AGENTS.md` and
  `.github/ISSUE_SPECS/1210-name-submix-strips-in-the-browser-session-map-and-the-sdk-measurement.md`
  with its Attempt 1 record. I also read the #1213 spec (it owns the open problem) and the #1214
  spec (its gate 4 owns the browser live-controls map).
- **Paths:** every changed path is on the authorized list.
- **How I ran it:**
  - I did not modify the worktree, commit, push or touch GitHub.
  - I exported `66b2c7dd7` with `git archive` into `/tmp/claude-1002/v1210/src`, with its own
    `CARGO_TARGET_DIR`, and copied `sdk/node_modules` in untracked.
  - I put the export under a throwaway local git repo so that I could restore it with
    `git checkout -- .` after every mutation and probe.
  - I deleted the scratch directory when I finished.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There are two MINOR findings: one is a test gap, and one is a
gate missing from the #1213 spec. Neither is a code defect in this slice. There are also four NITs.

D1 to D5 are implemented as frozen:

- **D1, the exports.**
  - `live_control_submix_count` and `live_control_submix_id` sit beside the track pair.
  - They read `ReadyOwnership.submixes` and copy through `copy_id_into_staging`.
  - An out-of-range index, an invalid handle or a disposed handle answers 0.
- **D2, the staging size.**
  - `HostSessionShape.longest_submix_id_bytes` is added.
  - The staging projection is the max of the source, track and submix lengths. That max is spelled
    in `lib.rs` and in all three test spellings.
  - `HostSessionShape` is built in one place only, so the new field breaks no other crate.
- **D3, the session map.**
  - The worklet reads the submix IDs at construction, under the track IDs' rules: nonempty, within
    `sourceIdCapacity`, ASCII.
  - The reply posts `submixes`.
  - The host's exact-field list and the element rule include it.
  - The `.d.ts` declares it, and the SDK mirror is a byte copy (`check-sdk-types` pins it).
  - `shape()` and `sessionMap()` carry the list, and the browser `createBrowserLiveControls` map
    copies it.
- **D4, the measurement.**
  - `createMeasurementFeeds` takes `submixIds`, and `MeterUpdate.submixes` is keyed by ID in order.
  - `meterProjection` refuses with `sdk.meter.submix_count` when the count or either array length
    differs.
- **D5, observation on strips.**
  - The headless `observationMap` and `readObservations` pass `[...tracks, ...submixes]` to the
    enrich, resolve and decode steps.
  - So do the browser `observationMap` and `readObservations`.
  - Submix and track IDs share the session's graph namespace (`validate.rs`, one `graph` map), so
    `strips.indexOf` cannot be ambiguous.

### The #1207 hazard is closed on both paths

- **Headless, on the shipped module.**
  - With a bus compressor tap present, `observationMap()` names the bus binding `bus`.
  - A selected read of the track tap succeeds.
  - Mutations A and E below turn the gate red.
- **Browser.**
  - The worklet's observation-map builder (`miso-engine-v1-audio-worklet.js:683-711`) checks only
    `u32(trackIndex)`, and the host validator checks only `validU32`. Neither bounds the index by
    the track count, so the real worklet passes a bus binding's strip index `T + j` through.
  - The SDK fake-host test then enriches the bus binding and resolves the bus index. Mutation C
    turns it red.
- **Managed subscriptions** look a binding up only for the selection they were given. A bus
  binding in the map no longer breaks a track subscription. Subscribing to a bus tap is #1214's
  `edit.submix()`.

### The open problem is described correctly and breaks no K2 gate

I probed the shipped module headless, with a throwaway eval in the export.

- A selected read of the bus tap fails with typed `invalidArgument` (result 1):
  `ObservationReadError::InvalidSelection` maps to `RESULT_INVALID_ARGUMENT` at `ffi.rs:693`.
- The engine stays `ready`.
- A later track-tap read succeeds.
- A mixed batch that includes the bus tap fails whole, with the same typed error.

This matches the record. #1210 gate 4 reads only the track tap, and `check-sdk-headless` is
339/339 green. The #1213 spec's D1 already lists `observation_selection_for_address` as per strip.
See MINOR-2 for the gate it lacks.

### The deviations

- **Export position (accepted).**
  - The export lists are a sorted set, not a positional sequence.
  - `abi_layout.rs:126` says "sorted", and the `check-abi-layout-v1.py` self-test carries an "export
    set is unsorted" mutation.
  - The shell gate pipes its list through `sort`.
  - Appending at the end would fail the gate. "Append, never reorder" governs positional wire
    fields, and this list is not one.
- **Exported `meterProjection` (accepted).**
  - `sdk/package.json` `exports` exposes only `.`, `./headless`, `./browser`, `./assets` and
    `./package.json`, and the browser barrel does not re-export it.
  - The public API is therefore unchanged, and `sdk-package.sh check` is green.
- **Lazy `strips()` in the browser engine (accepted, see NIT-2).**
  - It is called three times per `readObservations` and once per `observationMap`.
  - That work is on the main thread, behind two `postMessage` round trips that cost far more. It is
    on no per-block, audio-thread or render path.
  - The justification holds: the `live-response-evals.mjs` stubs return
    `{ sampleRateHz, quantumFrames }` with no `tracks`, so an eager spread would throw inside
    `createEngine`.
- **Worklet `submix_count` consistency refusal (accepted).**
  - It mirrors the existing `track_count` header check.
  - It runs only when meters are attached, and only at construction.
  - A mutation turns it red.

## Gates re-run on `66b2c7dd7`

All were run on x86_64. A is `/tmp/claude-1002/v1210/A` and B is `/tmp/claude-1002/v1210/B`.

| Gate | Result |
|---|---|
| `check-session-map-shape.py`, and `--self-test` | ok; 17 mutations caught |
| `check-abi-layout-v1.py --self-test` | ok; 22 caught |
| `build-web-audioworklet.sh --named-twin B A` | ok. Shipped module `c43ee96079ad...0d0b82af59` (2 694 139 B), the same bytes as in the record |
| `check-abi-layout-v1.py A/...layout.json` | ok. A's layout is byte-identical to `sdk/assets/...layout.json` and to `scripts/fixtures/abi-layout-v1-self-test.json` |
| `check-web-audioworklet.sh A B/...named.wasm` | ok |
| `check-browser-expected-resources.py --artifacts A` | ok; 32 red mutations |
| `test-web-audioworklet.sh` | ok |
| `check-sdk-generated.sh A` and `check-sdk-types.sh` | ok |
| `check-sdk-headless.sh A` | 339 pass, 0 fail |
| `sdk-package.sh check A` | ok |
| `cargo test -p host-web --lib` | 129 pass, 1 ignored |
| `cargo test -p host-core -p parameter-metadata` | all pass |
| `check-host-core-policy.sh` and `test-host-core-policy.sh` | ok |
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |

**Not run:** the full workspace test command (DESIGN section 7). The disk was at 19 GB free with a
12 GB floor. The diff reaches only `host-core` (one computed field), `host-web` and
`parameter-metadata`. I ran all three crates' tests, and workspace clippy compiled every target.

## Mutations

Each was applied to the export, run, and restored.

| # | Mutation | Result |
|---|---|---|
| R1 | Staging projection without `.max(longest_submix_id_bytes)` (`lib.rs` only) | red: `id_staging_bytes` 14 ≠ 63 |
| R2 | `_submix_id` reads `submixes` reversed | red: submix 0 has length 63, not 5 |
| R3 | `live_control_submixes()` truncated to 1 entry | red: count 1 ≠ 2 |
| A | Headless `enrichObservationMap(shape.tracks, …)` | red: capability gate 4 |
| B | Headless `shape().submixes` reversed | red: capability gate 2(a) |
| C | Browser `enrichObservationMap(shape.tracks, …)` | red: "the browser engine meters and observes a bus by its submix ID" |
| D | `meterProjection` keys submix `index` by `submixIds[S-1-index]` | red: gate 2(b) projection |
| E | Headless strips ordered `[...submixes, ...tracks]` | red: capability gate 4 |
| W1 | Worklet drops `length > sourceIdCapacity` for submix IDs | red: "a submix ID longer than staging must fail initialization" |
| W2 | Worklet posts `submixes` reversed | **green** (MINOR-1) |
| S1-S3 | Shape gate with `submixes` dropped from the `.d.ts`, renamed in the worklet reply, or dropped from the host list | each red (rc 1) |

## Test value

Each new test answers one question: which plausible defect turns it red that no other test
catches?

- **`host-web submix_ids_enumerate_in_canonical_order_through_staging_sized_for_them`.**
  - It catches staging sized without the submix IDs, where the long ID would trap the copy.
  - It also catches an export that orders submixes away from `ReadyOwnership`'s canonical order.
  - R1 to R3 confirm it.
- **capability-evals: "the session map names every submix in the frame's bus order".**
  - It catches a shipped enumeration whose order disagrees with the frame's bus sections, or a
    `shape()` that skips the new exports. Mutation B confirms it.
  - The fader check is structural: -40 dB on two tracks against unity on one, both from the same
    source, so the loud bus should read about 50 times the quiet one.
- **capability-evals: "a bus effect's observation names its submix and a track tap still reads".**
  - It catches a headless enrich step that uses the track list alone or puts the strips in the wrong
    order, which is the #1207 hazard. Mutations A and E confirm it.
- **measurement-evals: "a bus frame projects each submix's meter keyed by ID in canonical order".**
  - It catches a bus meter keyed by the wrong index, or read out of the track or master section.
    Mutation D confirms it.
- **measurement-evals: "a meter frame for another submix list is refused with sdk.meter.submix_count".**
  - It catches a projection that accepts a frame whose bus count differs from the known list. The
    `submixCount: 3` case isolates the count clause.
- **measurement-evals: "the browser engine meters and observes a bus by its submix ID".**
  - It catches `engine.ts` handing the track list alone to the feeds, the map or the resolver.
    Mutation C confirms it.
  - This is the only test of the resolver against strips; see MINOR-2.
- **test-web-audioworklet.mjs, the main-realm session map.**
  - It catches a host that accepts a reply without `submixes`, or with a non-array, a non-string or
    an empty entry.
- **test-web-audioworklet.mjs, the processor.**
  - It catches a worklet that does not post the enumerated list, or that accepts an empty or
    oversized submix ID. W1 confirms the oversized case.
  - It also catches a meter header whose bus count disagrees with the enumeration.
  - Order is not covered; see MINOR-1.
- **check-session-map-shape.py `--self-test`, two new mutations.**
  - They catch a host list or a worklet reply that drifts from the declared field set.

## Findings

### MINOR-1: the worklet's submix order is not pinned

The processor fake in `createFakeExports` has exactly one submix (`submixIds = ["drums-bus"]`). A
worklet that posts its submixes reversed or otherwise reordered therefore stays green (W2).

- **Why it matters:** the browser `EngineLiveControls` map is built from this reply, not from the
  scratch shape, and #1214 will index bus commands by it.
- **Why it is only MINOR:** the bus meters take their labels from the scratch boot's headless
  `shape()`, which gate 2(a) pins. The worklet loop is also trivially indexed, so the defect is
  unlikely.
- **Fix:**
  - Give `createFakeExports` a `submixIds` parameter, so the frame layout follows it. This can fold
    into #1209's MINOR-2 `submixCount` parameter.
  - Run the processor's session-map assertion once with two IDs, for example
    `["aa-bus", "zz-bus"]`.
  - Assert `map.submixes` deep-equals that list in order.

### MINOR-2: the #1213 spec needs a gate for a selected bus-tap read

The #1213 spec's D1 makes `observation_selection_for_address` per strip. None of its objective
gates performs a selected read of a bus tap, though: gate 5 arms a tap and reads the frame's word.
`K2-followups.md` asks for that read, but the spec body does not carry it.

Until that gate exists, nothing headless exercises the SDK resolver against strips. Mutating
`boundary.ts`'s `resolveObservationAddressesWithTracks(map, shape.tracks, …)` stays green today,
because both spellings fail on the bus read.

- **Fix:** root amends #1213's gates with a headless `readObservations` of the bus tap through the
  shipped module, in `capability-evals.mjs`. The read returns one row with `trackId: "bus"`, and the
  host does not refuse it.
- **Optional:** add a native host-web `read_observation_addresses` at index `T + j`.

### NIT-1: headless `pollMeters()` enumerates every ID on every poll

Headless `pollMeters()` calls `this.shape()` on every poll only to read `tracks.length`
(`boundary.ts:1096`). Since #1210, each poll also makes S more export calls and S more `TextDecoder`
decodes.

- This is not the render path, and the pattern was already there for sources and tracks.
- **Suggested follow-up:** read `live_control_track_count` directly, or cache the shape, which does
  not change after boot.
- **Optional:** cross-check the header's `submixCount` against the enumerated count, as the worklet
  now does.

### NIT-2: the browser `strips()` list is rebuilt on every call

The browser `strips()` builds a new array on each call, three times per read. A lazy memo
(`let strips; const getStrips = () => strips ??= [...]`) keeps the stub workaround and removes the
copies.

Better: in a later batch, give the `live-response-evals.mjs` stubs `tracks: []` and `submixes: []`,
and build the list eagerly. Production code then no longer takes its shape from a stub that breaks
the `SessionShape` type.

### NIT-3: `decodeObservationRows`'s `strips` parameter does nothing

- **Why:** each address comes from `strips.indexOf(selection.trackId)` over the same list, so
  `strips[address.trackIndex] === selection.trackId` always holds.
- **Effect:** the tracks-only spelling that stays green is therefore not a defect, and no test can
  or should be required for it.
- **Optional cleanup:** return `selection.trackId` and drop the parameter.

### NIT-4: the native test pins an exact byte count

`assert_eq!(resources.id_staging_bytes, 63)` pins an exact value, when the claim is "at least the
longest ID".

- It is acceptable, because the capacity is defined as exactly the max of the three lengths.
- `>=` would match AGENTS' ceilings-not-exact guidance more closely.
- Without this assertion the byte-for-byte copy still panics natively, so either spelling catches R1.
