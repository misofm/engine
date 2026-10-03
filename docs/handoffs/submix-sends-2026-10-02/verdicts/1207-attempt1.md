# #1207 *List every strip in the live-control handles and file bus effects in the browser*: Sol verdict, attempt 1

- Reviewed: `git diff 8f8c013e6 7bdb45187` (branch `codex/batch-submix-k2`, worktree
  `/home/bl/misofm/wt-submix-k2`). 15 files, +579/-261.
- Binding: `AGENTS.md` and
  `.github/ISSUE_SPECS/1207-list-every-strip-in-the-live-control-handles-and-file-bus-effects-in-the-browser.md`
  with its Attempt 1 record. I also read #1208, #1210 and #1213 to judge what this slice may leave
  to them.
- Every changed path is on the authorized list. D6 did not apply: #1053 has not landed, and capi
  reads no `HostLiveControlHandles` (checked with grep: the only construction is `prepare.rs:1335`,
  and the only non-test readers are host-web and `limiter_linked_session.rs`).
- How I ran it:
  - I did not modify the worktree, the branch or GitHub.
  - I exported `7bdb45187` with `git archive` into `/tmp/claude-1002/v1207/src`, with its own
    `CARGO_TARGET_DIR`.
  - After every mutation and probe I restored the file and checked it with `cmp` against
    `git show 7bdb45187:<path>`.
  - The probe code is in `1207-attempt1-verifier-scratch.rs` next to this file.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There are two MINOR findings, both about how strong the tests
are, three INFO notes and some NITs. Nothing needs another attempt.

- D1 to D5 are implemented as frozen.
- No search over the unsorted joined list is left, and no index arithmetic sends a bus effect to
  the wrong slot.
- The per-track guard still refuses every command at a bus index.
- Every gate I re-ran passes.
- The browser module digest matches the record: shipped `0a6e44c9…`.

## Gates (re-run by me on `7bdb45187`, x86-64 AVX2)

| Gate | Command | Result |
|---|---|---|
| 1, 3, 4 | `cargo test --locked -p host-core -p host-web -p effect-compiler --all-targets --features host-web/test-support,host-core/test-support,effect-compiler/test-support,engine/realtime-audit,builtins-compiler/test-support,graph/test-support` | all green: host-web lib 129 passed, 1 ignored; host-core every target; effect-compiler every target |
| 2 | the same run, `--test strip_handles` | 1 passed |
| — | `cargo test … -p graph-compiler --lib` (same features), because D5 changes what `attach_*` returns | 93 passed |
| 5 | `build-web-audioworklet.sh --named-twin B A` | ok. Shipped `0a6e44c997ca7ac3…` (2693096 B), the same as the record. Named twin `268499c2…` |
| 5 | `check-web-audioworklet.sh A B/…named.wasm` | ok |
| 5 | `check-browser-expected-resources.py --artifacts A` | ok, and the self-test is green (32 red mutations) |
| 5 | `check-sdk-headless.sh A` | ok |
| 6 | `check-`/`test-` pairs for `host-core`, `realtime`, `effect-runtime` and `workspace` policy | all eight exit 0. The workspace pair ran in a throwaway `git init` of the export |
| 6 | `cargo fmt --all -- --check` | ok |
| 6 | `cargo clippy --locked -p host-core -p host-web -p effect-compiler --all-targets --all-features -- -D warnings` | clean |
| 6 | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p host-core -p host-web -p effect-compiler --features host-core/control-provider` | clean. Without the feature, `lib.rs:19`'s link to `SessionControlProvider` fails. That line is unchanged and is feature-unification only |

I did not re-run these, because the disk had about 25 GB free:
- the full DESIGN §7 workspace test command (the record reports 1151 passed);
- clippy and doc across the whole workspace. Every reader of the renamed field is in the three
  packages I linted;
- `run-aarch64-tests.sh`. There is no arm64 host, so this stays "CI at batch push", as the record
  says.

## Review

### Strip order (D1)

- `live_control_strips` is `model.tracks` chained with `model.submixes`.
  - Both come from `compile.rs:136-142`, sorted by `StableId`'s derived `Ord` on `String`. That is
    byte order, the same `cmp` that `strip_index` uses.
  - Validation keeps IDs unique across tracks and submixes (`validate.rs:76-92`, one `graph`
    map), so a strip ID cannot be found in both segments.
- `control_requests` and the default meters iterate `live_control_tracks = &strips[..T]`.
- `canonical_index` covers every strip, as D1 orders. #1208 depends on this.
  - Side effect: a host-core caller that selects a meter on a submix now passes `host.meter.order`.
  - No product path can do that: host-web selects tracks only, and capi selects none.
  - #1208 owns that widening.
- The master is still validated against `T`.

### The lookup hunt

I grepped every `binary_search`, `.tracks`, `tracks.len()`, `rack_effects`, `effect_base`,
`observation_tracks`, `observation_present` and `.submixes` in host-web.

- **Searches.**
  - The three track-list searches are now `strip_index`: producer filing, observation filing and
    `resolve_observation`.
  - The only other `binary_search` is over sources (`:2922`).
- **Queue bands.**
  - `queue_available`, `push`, `preflight_effect` and `prepared_queue_address` use `3 * tracks.len()`,
    and `tracks` is the track prefix.
  - The effect band is `3T + dense_slot` over all T+S effects.
  - `queue_count = 3T + total_effects`. The probe measured `in_flight.len() == command_wanted.len() == 18`
    for T=2 with 12 effects.
- **Command guard.**
  - `track >= track_count` at `:4351` runs before every kind.
  - The later `rack_effects.get(track)` at `:4483` is therefore dead for a bus.
  - Probe: bypass and observe at indices 2 and 3 both return `RESULT_INVALID_ARGUMENT` with
    `UNKNOWN_TRACK`.
- **Read by address.** `observation_selection_for_address` still indexes `ready.tracks`, so a
  bus index is `InvalidSelection`. #1213 makes it per strip.
- **Meter frame.** `poll_meters` uses `tracks = ready.tracks.len()`, so the frame is still
  `3T + 3` words. That matches the non-goal.
- **GR fold (D4).**
  - The fold now skips `track == u32::MAX || track >= T` before it reads `armed`.
  - With D4 removed (my M2) and a bus tap force-armed, a bus writes `gain_base + T`, which is the
    master word: 8.689951 against 0.0. So the skip is what protects the frame.
  - `observation_present` has T+S entries and is read only at `master_track < T`.
  - The poll allocates nothing with bus lanes present: 0/0 on 8 polls in the probe.
- **The accepted hazard.**
  - The observation map now reports bus bindings at `track_index` 2 and 3 (probe).
  - `copy_eq_target_config(2, 3, 0)` returns `RESULT_OK` with the bus EQ.
  - Both are as the spec accepts, and commands at those indices are still refused.
- **Prepared companion.**
  - A companion target that names a bus index now resolves through `effect_slot`.
  - It is still refused, at wire index 0. INFO-5 covers the change in reason.
- **Defensive boot refusal.**
  - `track_count > strips.len()` and a model/handles count mismatch both refuse with
    `web.live_controls.effects`.
  - Both are unreachable by construction, and both are cheap.
  - They compare counts, not IDs. See MINOR-1.

### D5 and the render side

D5 attaches a live-control lane to every bus effect. Until now the render had never run a bus lane
with a control consumer, including a lane of a banked console slot. I checked this with probes
beyond the gates:
- A live-controlled bus session, with two console slots per strip, a bypassed bus limiter and a
  compressor insert per bus, renders the same bits as plain `prepare_host_runtime` for 24 blocks.
- A live `Bypass(true)` on aaa-bus's insert and on zzz-bus's console EQ renders the
  session-bypassed oracle bit-exactly.
- A live `Bypass(true)` on aaa-bus's **banked** console limiter renders the session-bypassed
  oracle bit-exactly. It is audible: 4200 samples differ from plain.

So each filed bus producer drives its own bus lane, and attaching lanes moves no bit. The
randomized differential (`randomized.rs:644`) now also sends records to bus producers, because it
draws over `effect_controls.len()`.

### Realtime

- **On a per-block path:** the D4 comparison in `poll_meters` and the existing control-queue
  drain, which now also runs over bus lanes.
- **Off it:** `strip_index` runs only at boot and in the control-plane `read_observations`.
- No allocation, lock or syscall is added. Gate 4 reports 0/0, and my `poll_meters` probe 0/0.
- The worklet call-graph gate (`check-web-audioworklet.sh`) passes.

### Deleted tests

Both deleted tests are superseded:
- `submix_strip.rs::no_bus_effect_gets_a_live_channel_or_an_observation_handle` asserted the P16
  rule, which D5 reverses.
  - `strip_handles` and host-web gate 1 assert the opposite.
  - Its claim about track console slots is covered by gate 1, which has console slots on tracks,
    and by `live_addressing.rs`.
- `live_controlled_boot_of_a_bus_with_an_effect_renders`: gate 1's host repeats that boot with
  more in it.
  - It has two buses, two console slots and meters on, and renders 8 blocks.
  - Its observation cap is 1 where the old test's was 4. Every effect has at most one tap, so the
    boot is the same.

## Mutations (mine; each restored and checked with `cmp`)

| # | Mutation | Committed tests | My probe |
|---|---|---|---|
| M1 | `strip_index` drops the `T +` offset for submixes | **red**: all three bus tests panic at boot (duplicate filing refuses) | red |
| M4 | `observation_present` sized `T` | **red**: gate 1 (`2 != 4`) | — |
| M5 | bus `rack_effects` count no inserts | **red**: all three bus tests refuse at boot | red |
| M6 | host-core strip prefix reversed | **red**: `limiter_linked_session` ×3 (`ch63 != ch00`). Cargo stopped before `strip_handles` | — |
| M8 | P16 put back in `attach_effect_observation` only | **red**: `strip_handles` and gate 1 | — |
| M2 | D4 skip removed | green | red (master word 8.69) |
| M3 | command guard widened to `rack_effects.len()` | green | red (bus 2 admitted) |
| M7 | `queue_count` excludes bus effects | green | red (12 ≠ 18) |
| M9 | host-web iterates `model.submixes` reversed when filling `rack_effects` | **green** | — |

M2, M3 and M7 surviving is expected: no command can reach a bus queue or arm a bus tap until #1213
(INFO-3). M9 surviving is MINOR-1.

## Test value

- **`tests::a_bus_session_boots_live_controlled_and_files_every_bus_effect`** (gate 1).
  - It turns red if a bus producer or observer is refused or misfiled:
    - a search of the unsorted list (implementer's mutation);
    - a strip index without its `T +` offset (M1);
    - bus inserts missing from the tables (M5);
    - presence sized per track (M4);
    - P16 left in either half (M8).
  - No other test files a bus effect in host-web.
- **`tests::a_bus_session_admits_and_renders_without_allocating`** (gate 4).
  - It turns red if a live-controlled render of a session with bus effect lanes allocates.
  - That render is new with D5. `a_processed_bus_renders_without_allocating` renders without live
    controls, and every other measured host has no submix.
- **`strip_handles::handles_list_tracks_then_submixes_and_file_bus_effects`** (gate 2).
  - It turns red if a submix enters the track prefix or the two segments are merged by sorting.
  - It turns red if the control requests or default meters start to cover buses.
  - It turns red if a bus effect has no channel or no observer (M8).
  - No other test checks the strip order of the handles.

## Findings

### MINOR-1: gate 1 cannot see a strip-order mismatch in host-web's tables

- **The gap.**
  - Both buses in `bus_effect_host` have the same shape, `[1, 1, 1]`.
  - So if host-web filled `rack_effects` and `effect_base` for the submixes in a different order
    from `handles.strips` (M9: `model.submixes.iter().rev()`), every dense slot is the same and
    gate 1 stays green.
  - The boot consistency check compares only the counts:
    `model.tracks.len() != track_count || model.submixes.len() != submixes.len()`.
  - Today both orders come from the same normalized model, so this is no defect now.
  - But with buses of different shapes the mismatch misfiles producers, and nothing would report
    it.
- **Fix** (either part is enough; both are cheap):
  1. Give the buses in `bus_effect_host` different insert counts. For example, `aaa-bus` gets a
     compressor and then an EQ insert, and `zzz-bus` only the compressor. Extend the `inserts`
     table in gate 1. Then M9 misfiles, and gate 1 turns red on the owner and address asserts.
  2. In `compile_ready`, refuse with `web.live_controls.effects` unless
     `model.submixes[j].id.as_str() == &*submixes[j]` for every `j`, and the same for the tracks.

### MINOR-2: gate 4 does not measure the one per-block function this slice changed

- **The gap.**
  - The measured window is `submit_commands` plus `render_next`.
  - The only per-block host-web code #1207 edits is the gain-reduction fold in `poll_meters`.
    The worklet calls it from `process()`.
  - That fold now walks T+S presence slots and the bus observation entries.
  - My probe measured it at 0/0 on 8 polls, so this is no defect.
  - But no committed test measures `poll_meters` allocation at all.
- **Fix.**
  - In `a_bus_session_admits_and_renders_without_allocating`, call `set_meter_lease(true)` and
    arm `t0`'s compressor tap (`observe(&mut host, 0, 1, 0, 1, 2, true)`) before the measured
    window.
  - Render enough blocks to close a window, then include `host.poll_meters()` in the `measured`
    closure.
  - This is optional, because the spec's gate-4 wording names only the submission and
    `render_next`. It could be folded into #1209, which edits the same fold.

### INFO-3: D4, the per-track guard and the `queue_count` growth are pinned only by #1213

- M2, M3 and M7 survive every committed test.
- None can be reached through the public path while the guard holds.
- #1213 gates 1, 4, 5 and 7 address bus queues and bus taps, so they will pin all three.
- No test is owed here. A test added now would be superseded inside the same batch.

### INFO-4: until #1210, SDK observation fails for a bus session with a tapped bus effect

- From this commit until #1210 D5 lands, `enrichObservationMap(shape.tracks, …)` is called at
  `sdk/src/core/observation.ts:200` (browser, `engine.ts:605`; headless, `boundary.ts`).
- It throws `sdk.observation.map` for any binding with `trackIndex >= tracks.length`.
- So a bus carrying a compressor, limiter or gate, booted with observation taps, makes every SDK
  `observationMap()` and `readObservations()` fail, **including reads of track taps**. Under K1
  those reads worked.
- The probe shows those bindings at `track_index` 2 and 3.
- #1210 (VERIFY-3 B1) owns the fix inside K2, and K2 is pushed once after #1214, so nothing ships
  broken.
- #1207's Hazards section does not name this.
- **Recommendation.**
  - Add one line to #1207's Hazards or Evidence: "the SDK observation map throws for a bus with a
    tapped effect until #1210 D5; K2 must not be pushed or split between #1207 and #1210".
  - The root should hold the batch to that.

### INFO-5: the refusal reason for a companion target at a bus index has changed

- `prepared_queue_address` now resolves a bus index because the tables cover strips.
- A companion target at a **non-EQ** bus effect is now refused with `UNSUPPORTED_KIND` (the
  `has_owner` check, `:4690`) instead of `MALFORMED`.
- An EQ target reaches the orphan check and stays `MALFORMED`.
- It is still refused, at wire index 0, and only in sessions with submixes. Sessions without
  submixes refuse exactly as before.
- #1213 redefines strip-indexed admission. No action needed.

### NITs

- `crates/host-core/src/prepare.rs:933-934`: the comment still says
  "`HostLiveControlHandles::tracks` and the requested channels cannot disagree". It should say
  `strips[..track_count]`.
- `crates/host-core/src/prepare.rs:363`: the `effect_controls` doc still opens "One control producer
  per prepared **track** effect instance". The next sentence corrects it.
- `crates/host-core/src/solo.rs:86` cites `HostLiveControlHandles::tracks`. This file is outside
  #1207's authorized paths; leave it to #1211, which edits it.
- effect-compiler: the `address` docs on `EffectControlProducer` and `EffectObservationHandle`
  say "within its track", but the owner may now be a submix.
- host-web `ffi.rs:4034`: "observation effect's **track** index" can now be a strip index `>= T`.
  #1210 and #1213 own that surface.
- `ReadyOwnership.tracks` keeps S slots of spare capacity after `drain(track_count..)`. This is
  harmless.

## What I could not verify

- The full workspace test command and the workspace-wide clippy and doc runs. I kept to the three
  changed packages plus graph-compiler's lib tests, because of disk space.
- aarch64.
- Gate 3's comparison of the tables against the base commit. I did not rebuild the base. Instead
  I checked it from the code:
  - With no submixes, `strips == tracks`, `submixes` is empty and `strip_index` reduces to the
    old search.
  - `rack_effects`, `effect_base` and `queue_count` are built from the same loops, and
    `observation_present` has `strip_count == T` entries.
  - The D4 skip never fires for an index `< T`.
  - No existing host-web test was changed, and all of them pass.
