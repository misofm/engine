# Meter any boundary of a submix strip and designate a master strip in host-core

Slice 11 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A host can attach a meter to any of a submix strip's seven boundaries, exactly as on a track:
`input`, `post_input`, `insert_send`, `insert_return`, `pre_fader`, `post_fader` and `post_pan`. No
meter has ever observed a bus.

A host can also designate a bus as the master whose gain reduction it reports. A mix bus with a
limiter insert is the natural master once submixes carry effects. The designation is still an index,
not a discovery: this **widens** the "designation, not discovery" stopgap to strips; it does not
retire it (VERIFY-1 MINOR-12). The browser reaches a bus master once *Address submix strips in
browser live commands* (#1213) lets observation commands address a bus.

## Context (verified on `fe8ac679`)

After *List every strip in the live-control handles and file bus effects in the browser* (#1207):
`HostLiveControlHandles.strips` lists tracks then submixes, `track_count` marks the track prefix,
`meters` and `track_controls` stay one per track, and `canonical_index` (`prepare.rs:1186-1220`) is
built over every strip. After batch K1 the builtins compiler's `known_tracks` set
(`crates/builtins-compiler/src/lib.rs:3327-3332`) is a strip set.

- **The request.** `HostMeterRequest { track_id, tap, metrics }`
  (`crates/host-core/src/prepare.rs:326-336`), re-exported from `crates/host-core/src/lib.rs`.
  Constructed at `crates/host-core/tests/prepare.rs:62`, `:67`, `:90` and
  `hosts/host-web/src/lib.rs:5720` (one `PostMatrix` `SAMPLE_PEAK` meter per **track**).
- **Building the requests** (`prepare.rs:929-974`). Selected meters map `meter.track_id`; without a
  selection, one meter per track is built at the request's `meter_tap`. Handles are `index + 1` in
  that order. Each becomes a builtins-compiler `MeterRequest` (`crates/builtins-compiler/src/lib.rs:67-72`),
  whose `track_id: String` is an internal name and keeps it.
- **The builtins compiler** refuses a meter on an unknown ID with `builtin.meter.unknown_track`
  (`lib.rs:3332-3336`). Since K1 that set includes submix IDs.
- **The graph** accepts observers only on a `TrackStage` node or the Output
  (`crates/graph/src/lib.rs:1591-1596`). A submix strip's seven boundaries are `TrackStage` nodes
  keyed by the submix's ID since K1.
- **Ordering after binding** (`prepare.rs:1177-1220`), read from the code:
  1. **Unknown owner.** A bound meter whose ID is not in `canonical_index` refuses with
     `host.meter.order`.
  2. **Caller-selected meters keep the caller's order.** The bound consumers are compared position by
     position with the requests on handle, ID and tap; a mismatch refuses with `host.meter.order`.
  3. **Default meters** are sorted into canonical order.

  So the code fires only when a bound consumer names an ID outside the canonical order, or when a
  selected meter comes back at a different position (VERIFY-1 MINOR-15).
- **The master designation.**
  - `HostLiveControlRequest.master_track: Option<u32>` (`prepare.rs:305-310`). Its doc says "V1 has
    no structural master bus — submixes and outputs carry no effect racks — so the master reading is
    a designation rather than a discovery. The successor is effect racks on submixes": stale since
    K1 (VERIFY-2 MINOR 12).
  - `HostLiveControlHandles.master_track` (`:364-365`) echoes it.
  - Validation (`:1222-1228`) accepts an index below the track count and otherwise refuses with
    `shape("host.observation.master_track")`.
  - host-core has **no master reading**: it validates and echoes the index. The reading lives in
    host-web's meter poll (`hosts/host-web/src/lib.rs:3431-3446`). So a host-core test can prove only
    acceptance, echo and refusal (VERIFY-2 M6).
- **Spellings are kept (DESIGN P17, VERIFY-2 M14).** The field `master_track`, the code
  `host.observation.master_track`, the browser boot word `live_control_master_track_plus_one` and the
  SDK option `masterTrackPlusOne` keep their names; their docs say "strip index, tracks first". The
  browser docs change in *Address submix strips in browser live commands*.

## Decisions frozen for this slice

- **D1. Meters on strips.** `HostMeterRequest.track_id` becomes `strip_id`, naming any strip. The
  builtins-compiler `MeterRequest.track_id` keeps its internal name (decision 12 kept internal names)
  and receives the strip ID. Rule 1 uses the strip order (already true after slice 10; this slice
  gates it). Rules 2 and 3 are unchanged. Default meters stay one per **track**, so a host that does
  not select meters sees exactly today's set.
- **D2. Codes unchanged.** `host.meter.order` and `builtin.meter.unknown_track` keep their spellings.
- **D3. Master designation on strips.** Validation accepts any index below `strips.len()` (tracks
  first, then submixes) and refuses one at or past it with the unchanged `host.observation.master_track`.
  The field keeps the name `master_track`. Its doc (`prepare.rs:305-309`) and the handle's echo doc
  (`:364`) say: a strip index, tracks first; it widens the "designation, not discovery" stopgap to
  strips; a bus whose strip carries a limiter can now be designated.
- **D4. host-web** compiles against D1 (the field rename at `lib.rs:5720`) and changes no behaviour.
  *Carry submix strips in the browser meter frame* (#1209) requests bus meters.

## Deliverables

1. host-core D1 and D3, with doc comments.
2. The builtins-compiler meter validation accepts strip IDs, if K1 left it track-only (it should not
   have).
3. host-web D4.
4. One new host-core test file for the gates below.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs`
- `crates/host-core/tests/prepare.rs` (the field rename) and one new test file (for example
  `strip_meters.rs`)
- `crates/builtins-compiler/src/lib.rs` (`:3322-3345`), only if the meter `known_tracks` set is still
  track-only
- `hosts/host-web/src/lib.rs` (the `HostMeterRequest` construction at `:5715-5726` only)
- this spec

## Non-goals

- No browser meter-frame change and no browser master change.
- No meter on a route.
- No C ABI per-strip meters: the C ABI keeps its single output peak.
- No spectrum target on submixes (`crates/host-core/src/spectrum.rs` stays track and output only).
- No rename of `master_track`, its code, the boot word or the SDK option (P17).

## Hazards

- **Index drift.** Default meters stay parallel to `strips[..track_count]`. A session without
  submixes must produce byte-identical meter handles and snapshots.
- **Rule 2 is position-sensitive.** A selected bus meter must come back at its requested position. Do
  not sort selected meters.

## Objective gates

1. **Bus meters equal track meters.** New test.
   - Session A: three tracks with unity strips, fed **distinct, non-constant** samples per track and
     per lane, route into a submix `bus` with strip S, and `bus` routes at unity to the output.
   - Session B: one track `ref` with the same strip S. Its stereo source is the `f32` sum of the three
     routed contributions, computed in the test by the D3 expression (`l' = (lr * r) + (ll * l)`, two
     roundings) and summed left to right in route-ID order, as in *Render a submix strip on its summed
     input* (#1200) gate 1.
   - Strip S has trim, an EQ insert, a compressor insert with `link_mode: maximum`, a -6 dB fader and
     a non-identity pan, so that every tap differs. There are no console slots.
   - A selected meter at each of `bus`'s seven boundaries reports the same snapshot words as the same
     meter on `ref`: peak, energy, held peak, clip counts and window. Bit-identical over 16 seeds.

   *Test value: it turns red if a bus meter observes the wrong stage or lane, is refused, or comes
   back at a different position. No meter has ever observed a bus.*
2. **Order rules on strips.** A selection of `[bus post_pan, track_0 pre_fader]` binds and comes back
   in that order. A request naming an ID that is neither a track nor a submix refuses with
   `builtin.meter.unknown_track` and binds nothing.

   *Test value: it turns red if the canonical order used by rule 1 is track-only (refusing every bus
   meter with `host.meter.order`), or if selected meters are re-sorted.*
3. **The master designation on strips.** For a session with `T` tracks and `S >= 1` submixes,
   prepared with live controls and observation taps:
   - `master_track = Some(T + S - 1)` (the last submix) prepares, and `handles.master_track` echoes
     `Some(T + S - 1)`;
   - `master_track = Some(T + S)` refuses with `host.observation.master_track` and prepares nothing.

   *Test value: it turns red if the designation is still validated against tracks only (refusing
   every bus), or against something larger than the strip list.*
4. **Render allocates nothing** with seven bus meters attached: after warm-up, `allocations == 0` on
   the render thread, measured with `bench_support::alloc`'s thread-scoped counters (a host-core
   dev-dependency already).

   *Test value: it turns red if a bus meter's observer allocates per block, which no track meter
   test exercises on a reduction-fed `Input` stage.*
5. **Unchanged where no submix exists.** Every existing host-core and host-web meter and
   master-designation test passes with only the field rename. The default meter set and handles are
   identical.
6. **Workspace and policy.**
   - The workspace test command (DESIGN.md section 7).
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `bash scripts/check-builtins-policy.sh` and `bash scripts/test-builtins-policy.sh`, if the
     builtins compiler was touched
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
7. **4-lane.** `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at
   the K2 push (`host-core` and `builtins-compiler` are in it).

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- Any digest or canonical-text re-pin, listed with its reason (none expected).

## Dependencies

- *List every strip in the live-control handles and file bus effects in the browser* (#1207)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" gates are hard stops. NaNs are folded to one value (decision 10).
- Render stays allocation-, lock- and syscall-free.
- Keep wire and SDK spellings (P17): `master_track`, `host.observation.master_track` and
  `builtin.meter.unknown_track` are not renamed.
- A test that greps source or prose is refused.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
