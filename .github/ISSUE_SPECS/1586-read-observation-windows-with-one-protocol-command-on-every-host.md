# Read observation windows with one protocol command on every host

Slice S26b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Second part of the planner's S26, after #1585. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

An app reads the latest window of any set of observation taps, up to 256 in one call, by stable
address: strip ID, console slot or insert ID, and tap ID. Each row says whether the tap is armed,
the arm sample and arm tag, and, once a window that opened at or after the arm has closed, its span,
sequence and left and right values in the tap's native unit. The read never consumes anything. The
C ABI serves it as `OBSERVATION_READ` through `miso_engine_v1_submit_command`; the browser's
existing read export calls the same core function.

## Context

- **The browser's read.** `AudioWorkletEngineHost::read_observations` (`hosts/host-web/src/lib.rs:2785`)
  checks the whole batch first and refuses duplicates; `read_observation_addresses_into` (`:2908`)
  is the allocation-free form the export uses (`miso_engine_web_v1_observation_read`,
  `hosts/host-web/src/ffi.rs:4367`, staging `ObservationStaging` `:279`). The resolution is
  host-web code: `observation_selection_for_address` (`lib.rs:6107`), `resolve_observation`
  (`:6150`), `read_observation_values` (`:6207`). Selections are numeric (`WebObservationSelection`,
  `:726`; strip index, rack, effect index), resolved to stable IDs through the session map exports
  (`observation_count` `ffi.rs:4227` ... `observation_tap_id` `:4328`).
- **No C ABI read.** The C ABI has no observation path; #1584 keeps the handles in
  `PreparedRuntime` and #1585 puts the arm state in the slot.
- **One reader acknowledges.** `ObservationReader::acknowledge` (`crates/engine/src/realtime/observe.rs:195`)
  writes the reader's own word; the meter fold (#1582) is that writer. A second thread that reads
  must not write it.
- **Message IDs.** `MessageId` (`crates/protocol/src/wire.rs:98`); the frozen code for this command
  is `0x000f`. `CapabilityFlags` (`crates/protocol/src/message_wire.rs:51-61`, `KNOWN` `:60`) has
  one bit per served feature, checked against the command list at decode (`:3509-3522`) and set
  in `capability_registry` (`crates/protocol/src/controller.rs:3219`, the flags at `:3257-3281`). The capability registry's command loop (`crates/protocol/src/controller.rs:3225`,
  `for id in 1_u16..=11`) and `allocated_id` (`crates/protocol/src/message_wire.rs:3534-3540`)
  list allocated commands; #1551 widens them.

## Decisions frozen for this slice

- **D1. The message.** `MessageId::ObservationRead = 0x000f`. Request `observation_read_request`:
  1 `selection` `Message` repeated, 1 to 256 (`observation_selection`: 1 `address` `Message`, #1551's
  `control_address` with owner strip, section console or inserts, `slot_id` present, `control` and
  `lane` absent; 2 `tap_id` `U32`; 3 `channels` `U8`, left 1, right 2, both 3). Response
  `observation_read_page`: 1 `observed_sample` `U64` (the render sample at the read), 2 `row`
  `Message` repeated, one per selection in order (`observation_row`: 1 `status` `U8`, pending 1,
  unarmed 2, ready 3; 2 `armed` `Bool`; 3 `arm_sample` `U64`; 4 `arm_tag` `U64`; 5 `first_sample`
  `U64`, 6 `end_sample` `U64`, 7 `sequence` `U64`, 8 `blocks` `U32`, all four present exactly when
  ready; 9 `left` `F32` and 10 `right` `F32`, each present exactly when ready and asked; 11 `unit`
  `U8`, the tap's native `ParameterUnit` code). The codec checks every presence rule both ways.
- **D2. Status.** From one `read_state` (#1585 D2): unarmed when not armed; ready when armed and
  the window's `first_sample >= arm_sample` (#1534's predicate); pending otherwise, also when the
  bounded read gives up.
- **D3. One core read.** `host_core::observation::read_into(handles: &ObservationIndex, selections:
  &[ObservationSelection], rows: &mut [ObservationRow]) -> Result<(), ObservationReadError>`, moved
  from the three host-web functions above and `read_observations`' duplicate rule.
  `ObservationIndex` is built once per plan (on the control thread, at preparation or adoption): for
  each observed effect, its strip ID, section key and handle, so a read resolves a stable address
  without allocating. It never acknowledges.
- **D4. Refusals, all or nothing.** The whole batch is checked before any row is written:
  unknown strip, slot or insert, `NOT_FOUND`, `observation.address.unknown`; unknown tap,
  `NOT_FOUND`, `observation.tap.unknown`; a computed tap, `UNAVAILABLE`,
  `observation.tap.unsupported`; an effect with no observation lane, `UNAVAILABLE`,
  `observation.unbound`; the same address and tap twice, `INVALID_FIELD`,
  `observation.selection.duplicate`. Each names `selection[i]`.
- **D5. A read-only reader.** `ObservationReader::observer(&self) -> ObservationObserver`: a second
  handle to the same slot with `read` and `read_state` and no `acknowledge`. The index of D3 holds
  observers, so a read on any thread never writes the slot.
- **D6. Hosts.**
  - C ABI: the readers are state of `SessionState` (the adopted plan's `ObservationIndex`, installed
    in `promote_pending_provider`, `crates/control-plane/src/control.rs:976-978`), so the command is
    session-executed. It uses the one dispatch form for such commands, #1594 D9: the controller
    decodes and checks the frame and returns `PreparedCommandFrame::Session(PreparedSessionCommand)`;
    `SessionState::command` runs `read_into` and calls `complete_session_command(prepared, reply)`
    or `complete_session_command_refused(prepared, status, code)` for D4's refusals. If this slice
    lands before #1594, it adds the variant and the two functions exactly as #1594 D9 words them,
    and #1594 reuses them.
  - Capability: `CapabilityFlags` bit 15 (frozen by the spec coordinator) means "serves
    `OBSERVATION_READ` (`0x000f`) and the `Observe` edit (`0x0801`)". `ProviderFeatures::observation`
    (#1584 D7a) lists `0x000f` and sets it. `CapabilityFlags::KNOWN` gains bit 15 (bits 14 and 16
    are the response and spectrum slices'). The capability decode refuses bit 15 unless `0x000f` is
    among the supported commands, and the reverse (the rule shape of
    `crates/protocol/src/message_wire.rs:3509-3522`).
  - Browser: host-web builds the plan's `ObservationIndex` in `compile_ready`, beside its
    `effect_observations` (`hosts/host-web/src/lib.rs:7383-7425`), at boot, so the render-locked
    read export allocates nothing. `read_observation_addresses_into` resolves its numeric addresses
    to stable addresses through the existing map and calls `read_into`; `WebObservationResult` bytes are unchanged
    (the arm fields are not in it; #1590 deletes the record).
- **D7. Docs.** The protocol registry, semantics and sizing documents, the C ABI header prose and
  `docs/C_ABI_V1_QUALIFICATION.md`.

## Deliverables

1. D1 in `crates/protocol` (wire, schema, codec, typed frame, the session-executed dispatch of
   #1594 D9, capability, corpus).
2. D3 and D5 in `crates/host-core/src/observation.rs` (new) and `crates/engine/src/realtime/observe.rs`.
3. D6 in `crates/control-plane/src/control.rs` and `hosts/host-web/src/lib.rs`.
4. D7 and the tests below.

## Authorized paths

- `crates/protocol/src/{wire.rs,message_wire.rs,message_wire/tests.rs,schema.rs,typed_frame.rs,controller.rs,controller/tests.rs,lib.rs}`,
  `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`,
  `fuzz/corpus/complete-schema-manifest.md`.
- `crates/engine/src/realtime/observe.rs`, `crates/host-core/src/{observation.rs,lib.rs}`,
  `crates/host-core/tests/effect_observation.rs`.
- `crates/control-plane/src/{control.rs,compile.rs}`.
- `crates/capi/src/runtime/{tests.rs,live_tests.rs}`, `crates/capi/include/miso_engine_v1.h`
  (prose), `tools/audit/src/{capi.rs,protocol.rs}`.
- `hosts/host-web/src/{lib.rs,tests.rs}`.
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS,SIZING}.md`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No subscription by read: a read arms nothing.
- No browser frame path and no export deletion (#1589, #1590).

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --all-features`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`.

1. **C ABI read follows the arm.** Fixture of #1584 gate 1. Before any `Observe`: the row is
   unarmed. After the arm transaction and before a render: unarmed (render has not applied it).
   After one render: armed, pending, with the reply's tag. After the first window closes: ready,
   `first_sample == arm_sample`, values equal to the scalar reference, `unit` dB. After a disarm
   and one render: unarmed with the disarm's tag.
2. **Non-consuming.** Two reads in a row with no render return identical rows, and the meter
   fold's next frame still folds the same window (it was not acknowledged by the read).
3. **All or nothing.** A batch with one unknown tap, or a duplicate, returns the typed refusal and
   no row; 257 selections are refused by the codec.
4. **Browser bytes unchanged.** `selected_observation_reads_are_bounded_stable_and_non_consuming`
   (`hosts/host-web/src/tests.rs:6318`), `selected_observation_reads_keep_same_track_owner_windows_and_sequences`
   (`:6460`) and `selected_observation_numeric_read_refuses_failed_host_without_touching_output`
   (`:6550`) pass unchanged.
5. **Same rows on both hosts.** A native test boots the same session on both hosts, arms the same
   tap (C ABI by `Observe`, browser by kind 7), renders the same PCM, and gets equal status,
   window and values from `OBSERVATION_READ` and from the browser read for the same tap.
6. **Realtime and allocation.** A 256-row read after warm-up makes `allocations == 0` on the
   reading thread beyond the response frame's own buffer, and render stays allocation-free while
   reads run on another thread.

## Test value

- Gate 1 turns red if a row reports an arm before render applied it, or a window from before it.
- Gate 2 turns red if a read consumes a window or disturbs the meter fold.
- Gate 3 turns red if a partly bad batch returns partial rows.
- Gate 4 turns red if the move changes a browser byte.
- Gate 5 turns red if the two hosts read the same tap differently.
- Gate 6 turns red if the read path allocates per row or the second reader writes the slot.

## Dependencies

- #1585 (the arm state), #1584 (the handles on C ABI plans), #1551 (`control_address`, the widened
  command range).
- `crates/control-plane` and `crates/capi` are stream B's files; `hosts/host-web/src/lib.rs` is
  stream H's. This slice rebases after them.

## Supersedes and shrinks

- None.

## Decision record: slice #1586a (2026-10-25)

Split by root (scope note, R1 accepted): #1586a is the core read and the browser rewire; #1586b is
the `OBSERVATION_READ` command on the C ABI (D1, D6's C ABI and capability part, D7). No protocol
change and no hash re-pin in #1586a.

- **D3 amended (root ruling on finding 1).** The browser's carried swap exchanges reader sets
  between two plans' handles at fixed positions on the render thread
  (`host_core::pair_carried_observations`), so an index that held reader or observer copies taken
  at preparation would read the successor's fresh slot after a carry, which nothing publishes to.
  `ObservationIndex` therefore holds, per effect instance, its stable key (strip ID, section,
  effect ID), its handle's position in the plan's `effect_observations` (`None` for an instance
  with no tap, which reads as `Unbound`) and its descriptor, sorted for a binary search.
  `read_into` and `read_with` also take the active plan's `&[EffectObservationHandle]` and read only
  through `ObservationReader::read_state`. host-web builds the index in `build_plan_shape` (the boot
  plan's and every replacement's), where the positions are fixed: a carry swaps only `readers`,
  never a position, and the shape is adopted in the same swap block as the companions.
- **D5 amended.** `ObservationReader::observer()` is #1586b's, only if the C ABI's index wants
  observers; #1586a does not add it (nothing in it needs it).
- **D3 amended (root ruling on finding 2).** host-core cannot reach the control plane's issued-tag
  table (`control-plane` depends on `host-core`), so the read takes a per-selection lookup
  `FnMut(&ObservationKey) -> (u64, bool)`, filled by host-web from `SessionState::observation_issued`.
  The #1585 status contract is unchanged (Pending while the slot's tag is below the issued tag of an
  arm or re-arm, or no window since the arm, or every attempt tore; an accepted disarm reads
  Unarmed at once).
- **`read_with`.** The browser's numeric export maps each address to its stable selection through
  the session map in the same pass as the check, so the first refusal in address order is the one
  returned, as before; `read_into` is the slice form over stable selections. Both check the whole
  batch before any row is written and keep what each selection resolved to on the stack (no
  allocation per read).
- **Duplicate rule.** One rule for both browser paths: the same address and tap twice, whatever the
  lanes (D4). The numeric export used to compare whole addresses, lanes included, so a batch naming
  one tap twice with different lanes was read; it is now refused with the invalid-argument result.
  The SDK already refuses that batch (`validateObservationSelections`).
- **Moved and deleted.** `resolve_observation`, `read_observation_values` and
  `ReadyOwnership::effect_by_id` (its only callers were the read and a test helper) are gone;
  `observation_selection_for_address` stays as the numeric map. `ObservationReadChannels` and
  `ObservationReadStatus` are host-core's, re-exported by host-web under the same names.
- **Charge.** The index's table and the IDs it copies join the shape's instance-table row.
- **Tap bound.** The old `tap_index >= u32::BITS` refusal is replaced by `tap < readers.len()`
  (`Unbound` otherwise): one reader per declared tap, and the tap count per effect is capped at
  preparation.
- **Gate 2, host-core half: closed on purpose.** The existing browser test
  `selected_observation_reads_keep_same_track_owner_windows_and_sequences` asserts each reader's
  `consumed_sequence` is unchanged after a read that now goes through `read_with`, so a
  host-core copy would test the same claim twice.

### Evidence

- Bytes: a one-time scratch dump (not committed) of every numeric batch and single read, all
  channels, across arm, render, disarm and re-arm on `observation_host` and
  `same_track_observation_host`, on base `1c485261c` and on this branch, separate target dirs: 366
  lines identical except the one batch above (same tap, two lane sets), now refused.
  `WebObservationResult` is built from the unchanged export code over these values.
- Gate 4's three browser tests pass unchanged.
- Carry: `an_armed_tap_and_its_windows_follow_the_effect_through_a_swap` and two other
  structural tests are red on a mutant that reads handles copied at preparation; the existing test
  defends the claim, so no new test (its test-value sentence names it).
- Charge: `observation_staging_resource_is_exact_for_the_taps_off_and_on_modes` now boots one
  console EQ and adds its instance rows and its index entry to the exact bridge totals; red when the
  index charge is removed.
- `a_read_checks_the_whole_batch_before_it_writes_a_row` (host-core): red on a single-pass read
  that writes before the batch passed, on a duplicate rule that compares lanes, and on a refusal
  that names selection 0.
