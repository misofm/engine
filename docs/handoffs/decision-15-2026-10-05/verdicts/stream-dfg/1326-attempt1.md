PASS

# #1326 attempt 1 -- adversarial verdict

Issue: Give every browser plan live strip fader and mute lanes (stream D, D15-9).
Commits reviewed: 6fe4b4d80 (code), 6bf2fcdd6 (attempt record, simd128 ceiling re-pin); parent 2a589d360.
Reviewed with `git diff 2a589d360 6bf2fcdd6`, exported with `git archive` to /tmp/claude-1002/v1326/tree and
built with CARGO_TARGET_DIR=/tmp/claude-1002/v1326/target (mutations in a second export, own target dir).
No edits, builds or checkouts in the worktree. 7d5362dc2 (#1471 spec, STREAMS.md) ignored as instructed.

## Requirement check

- D1: `live_control_request` (hosts/host-web/src/lib.rs:7283-7307) always returns `Some(depth)`:
  the option when nonzero, else `host_core::STRIP_LIVE_QUEUE_DEPTH`. Met.
- D2: `pub const STRIP_LIVE_QUEUE_DEPTH` = 16 in crates/host-core/src/prepare.rs:306, re-exported
  in lib.rs; `LIVE_QUEUE_DEPTH` in crates/control-plane/src/compile.rs:18 is an alias of it. Met.
- D3: `live_lanes(options)` (lib.rs:7316-7326) is `ALL` for a nonzero option, else
  `FADER_AND_MATRIX`, passed to all three branches of `compile_ready` (lib.rs:6690-6732). The three
  host-core entries gained a trailing `lanes: HostLiveLanes`; every other caller passes `ALL`. Met.
- D4: `ReadyOwnership.commands_enabled` replaces the producers as the gate in
  `live_controls_attached`, both VCA/route refusals, the input-filter shadows and the live VCA
  state. I checked every other path that reaches `ready.controls` (`queue_available`, `push`,
  the kind 0-4 admission arms, `prepared_queue_address`, `copy_input_filter_config`, the ID copy
  functions): all are reached only after the `submit_commands_inner` gate (lib.rs:3362), or read
  state that stays empty for option 0, or read `ready.tracks`, which did not change. No command
  reaches the lanes with option 0. Met.
- D5: the measurements are correct (see "Ceilings" below). Met.
- D6: no render-path code changed. The lanes are created and settled, and no record is pushed
  to them. Met.
- Acked-batch question: no queue or admission path changed. The refusal with option 0 happens
  before any staging or push, so no ack can come before a drop.
- Public API: no parallel variant. No new public entry was added. Three existing entries gained
  the argument that D3 requires. The `selected_meters` parameter that was removed belonged to the
  private `prepare_host_runtime_with_live_controls_policy`. After the selected-meters entry started
  to call `..._policy_and_spectrum` directly (the way the other lane entries already do), every
  caller passed `None` for it, so it was dead. The public signature of
  `prepare_host_runtime_with_selected_meters_between_render_calls` only gained `lanes`. The public
  meters selection is still there.
- Authorized paths: every file in the diff is on the spec's list. The edit to stream B's
  `compile.rs` is the constant line plus one doc line (see NIT-4).
- Render-owned memory: the render side only gets the lanes' consumer rings and ramp state, which
  is render data. The producers stay in `ReadyOwnership` on the control side.
- Decision 15: this matches D15-9's mechanism bullet ("#1326 gives them to every browser plan").

## Ceilings (root ruling: ceilings follow measured values)

- Native `retained_ceilings.rs`: I measured `builtin_retained_bytes` = 3,383 (x86-64, `--nocapture`).
  3,383 x 1.1 = 3,721.3, rounded up to 64 bytes = 3,776. This matches the pin and the file's
  stated headroom rule (value + 10 %, rounded up to 64). No other row moved.
- simd128 `expected.json` `builtinRetainedBytes`: measured 3,219 of 3,584 (365 bytes free) by
  `check-browser-expected-resources.py --artifacts`. 3,219 x 1.1 = 3,540.9, rounded up = 3,584.
  This is the script's documented rule. The reason is in the 6bf2fcdd6 commit message. No other
  row moved.

## Findings

No BLOCKER. No MAJOR.

- **MINOR-1 (root follow-up, not counted against this slice).** Stale app-facing wording says
  option 0 attaches "no control channel":
  hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:44 and :791,
  sdk/src/browser/shipped-host.d.ts:44 and :791 (a generated copy of the same declaration),
  hosts/host-web/web/miso-engine-v1-audio-worklet.js:1549. These are stream H files, outside the
  slice's authorized paths, and the spec's non-goals forbid SDK edits. The implementer reported
  them. From the app's side the statement is still nearly true: no command path and no staging
  buffer. But the plan now has the lanes. Root should give one stream-H line edit:
  "accepts no live command and has no staging buffer". The Rust `WebBootOptions` doc
  (lib.rs:1226-1231) already says this correctly.
- **NIT-1.** hosts/host-web/src/lib.rs:1792-1793: "the browser always prepares with it, so this
  never fires" is now false. An option-0 plan has no input lane. The branch still never fires,
  but the reason is that option 0 refuses every command. The file is authorized. Fix the comment
  in the follow-ups commit.
- **NIT-2.** crates/host-core/src/prepare.rs:469: a 130-column doc line ("lane (#1053 D5, #1256,
  #1263) and get none; their route lanes arrive ... A settled"). rustfmt does not wrap comments,
  so fmt passes, but the 100-column style is broken. Rewrap it.
- **NIT-3 (root, sequencing).** STREAMS.md's hot-file row for `crates/host-core/src/prepare.rs`
  (line 75) lists D #1288, #1324 and #1325 but not D #1326, which edits that file. This slice also
  removes the private policy wrapper's `None` arguments. G #1457 (already landed) and G #1471 (the
  C ABI entries' `bound_cache`, pending) work in the same area, so the later slice must rebase.
  Root should add #1326 to the row.
- **NIT-4.** crates/control-plane/src/compile.rs:14: the spec allows "the one constant line" in
  stream B's file. The slice also adds one doc line above it. It is harmless and adjacent, but it
  is a second line for root to sequence with stream B.
- **NIT-5.** hosts/host-web/tests/retained_ceilings.rs:69-74: `four_lanes` was re-pinned to 3,776
  with no AArch64 measurement, because AArch64 runs only in CI. The per-strip figures in the
  comment are x86-64 values. The rings are records, not lanes, and AArch64 is 64-bit like x86-64,
  so the value should match and the 10 % headroom covers it. The comment should say that the
  four-lane value is inferred, not measured, until CI reports it.

## Test value

New test: `every_browser_plan_carries_the_strip_lanes_and_renders_the_lane_free_bits`
(hosts/host-web/src/tests.rs:9736).

Sentence: it goes red if any browser preparation branch, the spectrum collection boot alone
included, prepares an option-0 plan without the strip fader/mute and matrix/pan lanes. No other
test in host-web catches that (M1, M2b below). It also checks that option 0 renders bit-identical
to the lane-free plan the browser prepared before this change, under the browser's
`BetweenRenderCalls` delivery. host-core's `live_lanes.rs` prepares only with `Concurrent`
delivery (verified at crates/host-core/tests/live_lanes.rs:198-231), so the test's claim about
that is true. Its gate-2 bit comparison is not shown to be a unique catch. The implementer's
settled-fader arithmetic mutation is also caught by three existing fader tests, and the attempt
record says so honestly. No test-value claim in the doc comment or the record is false.

Gate 3 adds no new test. The existing `a_live_control_free_host_refuses_every_live_kind_as_unsupported`
now boots a lane-carrying session, so it defends the option-keyed gate (M5).

## Mutations re-run (my export; `cargo test --no-fail-fast -p host-web --features host-web/test-support` unless noted)

| Mutation | Red | Other tests red |
| --- | --- | --- |
| M1: option 0 prepares with `control_queue_depth: None` | new test | none (all host-web lib and integration targets) |
| M2: collection branch prepares with `None` for every option | new test | `render_locked_staging::render_locked_reads_never_allocate_after_boot` (its opted-in observation boot loses its collection) |
| M2b: collection branch drops the lanes for option 0 only | new test | none |
| M3: `live_lanes` returns `ALL` for option 0 | new test | `live_route_ids_enumerate_in_send_index_order_through_staging_sized_for_them`; `retained_ceilings` (builtin row over its ceiling) |
| M5: `live_controls_attached` reads `!controls.is_empty()` | new test | `a_live_control_free_host_refuses_every_live_kind_as_unsupported` |

Each mutation was reverted by copying the file back from the clean export.

## Gates run (x86-64, this box, from the export)

- `cargo test --locked -p host-core -p host-web -p capi -p control-plane --features host-core/test-support,host-web/test-support`: pass (host-web lib 189 passed, 2 ignored).
- `cargo test --locked --release -p console-workload`: pass.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: pass.
- `check-host-core-policy.sh`, `check-workspace-policy.sh`, `check-realtime-policy.sh`: pass.
- `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` (builtinRetainedBytes 3,219 of 3,584), `check-sdk-headless.sh`,
  `test-web-audioworklet.sh` (private TMPDIR, no leftovers): pass.
- `npm run qualify -- --check-matrix --self-test-mutations` (pulseaudio null sink as in CI): chromium 151.0.7922.34, firefox 153.0, webkit 26.5 -- all qualification gates passed.
- Not run: `check-capi-abi.sh` and `audit capi`. No ABI or capi source changed. The only control-plane edit is
  the alias, with the same value. `check-cross-targets.sh` was not re-run. No engine crate changed,
  and the implementer reported PASS.
