# Boot an empty session on both hosts

Owner answer (2026-10-15): an app opens an empty mixer and adds stems through structural edits.
Decision 17 programme. Code anchors verified on `4be10cd67` (PR #1680). Issue number: assigned at filing (written here as TBD30). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A session with no source and no track boots on both hosts (it renders silence at the output), and
a structural edit can then add a stereo stem with its track; both hosts render it identically.

## Context

- A session with no source or no track fails to boot today: host-core's source graph preparation
  refuses it (`graph_failure("host.source.graph.prepare")`, `crates/host-core/src/prepare.rs:2570`),
  at least on the browser; #1562b's gates had to boot with one mono source.
- Since #1562b both hosts accept rebuilds through the one core path, so the first stem can be added
  by a structural transaction.

## Decisions frozen for this slice

- **D1.** An empty session (no source, no track; submixes, VCAs and console slots may be declared)
  is valid and prepares a plan that renders silence at the output, on both hosts, with no special
  path in a host adapter.
- **D2.** Every boot-time size that depends on the session (strip counts, meter frames, staging)
  handles zero, and grows by the existing successor paths when the first strip arrives.

## Objective gates

1. Boot empty on both hosts: the plan renders silence, status, meters (an empty frame) and the
   control list are consistent; zero render allocations.
2. **Parity row:** boot empty, add a stereo stem and a track by one structural transaction, feed the
   stem and hear it, bit-identical on the browser and the C ABI at the four launch rates.
3. The browser qualification gains an empty-boot leg (chromium, firefox, webkit).
4. `cargo test --locked -p host-core --all-features -p control-plane --features test-support -p
   capi -p host-web --features host-web/test-support`; boot budget.

## Authorized paths

- `crates/host-core/src/prepare.rs` (and `shape.rs`), `crates/graph-compiler` if an empty graph
  needs it, `crates/control-plane`, `crates/capi` tests, `hosts/host-web/**` (empty-boot sizes),
  the worklet JS and SDK only where an empty shape is refused today, `hosts/host-web/qualification`,
  this spec's record.

## Dependencies

- #1562 (rebuilds on the browser).

## Size

Half a day to a day.

## Decision record: core and C ABI part (batch 8)

What shipped:

- The one refusal on the path was `prepare_graph_source_set` (`crates/source/src/lib.rs`), which
  returned `SourceGraphSourceSetError::EmptySources` for a session with no source; the variant and
  its check are deleted. The session validator, the graph compiler, the shape report, the meter
  and control-list builders, the plan swap and the C ABI staging already handled zero strips: the
  empty boot needed no other change, and no host adapter has a special path (D1). Every
  boot-time size that depends on the strip count was exercised at zero by the gates below and
  needed no fix (D2); the successor path grows from empty to one strip and shrinks back.
- From TBD29: `HostPrepareCaps::maximum_source_channels` and its `host.source.channels` check are
  deleted (the validator rule is the one limit), with its test `source_channel_cap_is_optional`,
  its `MUTATIONS.md` row M-06 and the stale doc line; callers dropped the field.

Gates:

- `crates/host-core/tests/empty_boot.rs`: `an_empty_session_prepares_and_renders_silence` (the
  console-slot-only mixer and `canonical-minimal.json`), `declared_submixes_and_vcas_do_not_make_an_empty_session_noisy`
  (an unfed bus and a VCA over it). Each renders four blocks of `+0.0` with zero render
  allocations.
- `crates/capi/tests/empty_boot.rs` (its own binary, `bench_support`'s allocator):
  `an_empty_session_boots_and_renders_silence_without_allocating` at 44.1, 48, 88.2 and 96 kHz:
  resource row (0 sources, 0 tracks), boot watermark, silence with zero render allocations, control
  list (the session header only, no strip), counters, snapshot, and a render-peak meter that reads
  `0.0`; `a_structural_transaction_adds_the_first_stem_to_an_empty_session` at the same rates: one
  `SESSION_TRANSACTION_APPLY` (source, track, route) takes the rebuild path, the swap block
  allocates nothing, the structure epoch and the control list grow, the fed stem is heard, and a
  second transaction removes it again (silence, counts back to zero). The browser parity row
  mirrors this file.

Departures: `crates/capi/tests/empty_boot.rs` joins the realtime policy's unsafe allowlist
(`tools/realtime-policy/src/policy.rs`, `docs/REALTIME_DEPENDENCY_POLICY.md`) as every capi test
binary that calls the C entries does. Otherwise none. Browser part (empty-boot sizes, parity row, qualification leg) stays with lane W;
the stale remark "A session with no source or track does not prepare" in
`hosts/host-web/tests/control_parity.rs` (`mono_model`) is theirs to drop.

## Decision record: browser part (batch 13, lane W)

What shipped:

- **Two browser boot-time sizes were fixed at zero** (D2); everything else the spec lists (strip
  order, meter frame and header, source map, observation tables, staging) already handled zero and
  needed no change:
  1. `AudioWorkletEngineHost::meters_attached` (the meter lease's gate) read the plan's strip
     meters, so an empty plan refused the lease with `RESULT_UNSUPPORTED` while the worklet and the
     SDK already derive `metersAttached` from the boot option (`liveControlMeterBlocks != 0`):
     the SDK's preparation rehearsal (`prepareBrowserSessionInWorker`) would have thrown "Preparation
     meter lease refused" on an empty session. It now reads the configured window.
  2. The master-window ring was sized from the first strip meter's queue (`master_capacity`), so a
     session that booted empty kept a zero-length ring for its whole life and the first strip's
     master peaks were all counted lost. It is `METER_QUEUE_DEPTH` when a window is configured and
     no strip meter exists.
- `poll_meters` with no strip meter publishes the master's own windows (a frame of the master's
  two peaks, header `trackCount` 0), so a page that leases the meters on an empty mixer gets a
  continuous stream; the first strip's adoption restarts delivery as any non-carried swap does.
  No allocation: the branch reads the preallocated master ring and writes the preallocated frame.
- A boot with a designated master strip (`liveControlMasterTrackPlusOne != 0`) over an empty
  session is still refused (`host.observation.master_track`: the strip does not exist). That is the
  honest answer for a strip index; a live master designation is successor work (see the report).

Gates:

- Parity row: `hosts/host-web/tests/control_parity.rs`
  `both_hosts_boot_an_empty_session_and_add_the_first_stem_alike` (four launch rates): empty boot
  (a console slot declared), identical silence, identical control list / state / snapshot bytes,
  one structural transaction (source, track, route) with the same verdict on both legs, the stem
  fed after the swap and heard bit for bit alike. The stale "does not prepare" remark at
  `mono_model` is dropped.
- Browser module: `hosts/host-web/tests/empty_boot.rs`
  `an_empty_session_boots_and_grows_by_a_structural_transaction` (own binary, the render-locked
  allocator, through the exports): status, boot watermark, control list (session rows only), the
  master-only meter frame, zero render-locked allocations on the empty plan, across the first
  strip's swap and while the stem plays; the meter frame grows to the strip and the control list's
  structure epoch moves.
- Fixtures: `hosts/host-web/tests/empty_boot_fixture.rs` writes `qualification/empty-session.json`
  (the live-control session with no source, track or route) and
  `qualification/empty-boot-frame.json` (the codec's frame that adds them back), byte-compared.
- Qualification leg (`runEmptyBootQualification`; gates `empty-boot-silence`, `empty-boot-shape`,
  `empty-boot-meter`, `empty-boot-first-stem`; the render-allocation list gains `empty-boot` and
  `empty-boot-first-stem`): chromium, firefox, webkit with `--check-matrix --self-test-mutations`.
  The SDK has no structural-edit builder, so the page sends the codec's frame through the SDK's
  endpoint (`sendStructuralFrameThroughSdk`, which stamps the allocator's next request ID and the
  committed revision).
- Wasm build: boot high-water growth 20,447,232 bytes (before: 20,447,232).
