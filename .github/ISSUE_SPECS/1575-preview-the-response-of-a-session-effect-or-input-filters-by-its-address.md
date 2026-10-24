# Preview the response of a session effect or input filters by its address

Slice S18 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C2 and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

An app asks for the frequency response of an EQ in the running session (an insert or a console
slot of one strip), or of that strip's input filters, by the same stable address it uses to set
their controls. The engine computes it from the committed values, with optional "what if" values
for some controls, so an app can draw the curve while a hand drags a band before it commits.
Both hosts get it through `RESPONSE_GET`; nothing is prepared or kept between requests.

## Context

- **The wire is frozen.** #1573 D2: target 2, `address` (`control_address` of #1551 D3, with
  `control` and `lane` absent), plus `override` fields `{control, lane, value}`; #1573 D7 refuses it
  today with `response.target.unavailable`; the provider receives the committed model.
- **The address.** #1546 D4: `ControlAddress { owner, section, control, lane }`; sections own 6,
  builtins 4, console 5, inserts 2. Builtin row IDs: `hpf_hz` 3 and `lpf_hz` 4
  (`BUILTIN_PARAMETER_DESCRIPTORS`, `crates/builtins/src/lib.rs:462`). #1549 D2-D5: an effect
  section lists the bypass (control 0), one row per parameter and lane, and on an insert whose
  effect has more than one link mode, the link-mode row `0x8000_0000`; `controls::get` returns a
  parameter lane's resolved value. #1546 D6: `controls::list`, `controls::check`.
- **The model.** `ConsoleSlot { slot, identity, quality, link_mode }` and `ConsoleEntry { slot,
  bypass, params }` (`crates/session/src/model.rs:343-363`); `Effect { id, identity, quality,
  bypass, link_mode, params, sidechain }` (`:590-605`); `EffectIdentity::{Native, ThirdPartyCid}`
  (`:609`).
- **One resolver.** `effect_compiler::resolve_initial_values(descriptor, params)`
  (`crates/effect-compiler/src/prepare.rs:1970`) turns a session entry's `params` into one value
  per descriptor parameter and lane, in descriptor order: the same values a plan rebuild prepares.
- **Today's preparation path** (`prepare_response_preview`, `crates/host-core/src/response.rs:502`)
  starts from the descriptor defaults and applies overrides (`apply_overrides`, `:588`).
- **#774 planned a retained catalog** of prepared bindings with handles and generations
  (`f4e1f01c0:.github/ISSUE_SPECS/774-bind-requested-session-responses-by-stable-target-ids-with-engine-owned-grids.md`).
  It is paused and not on `main`.

## Decisions frozen for this slice

- **D1. What an address may name.** `owner` must be a strip; `section` must be builtins, console
  or inserts. Anything else is `INVALID_FIELD` `response.address`. An unknown strip, slot or
  insert is `NOT_FOUND` `control.address.unknown`.
- **D2. Resolution against the committed model** (a new `host_core::resolve_response_target(model:
  &SessionModel, address: &ControlAddress, overrides: &[ResponseOverride]) ->
  Result<OwnedResponseTarget, ResponseQueryError>`):
  - **builtins:** the four cutoffs are `controls::get` of rows 3 and 4 on `left` and `right`;
    the target is `ResponsePreviewTarget::InputFilters`.
  - **console or inserts:** the effect is the slot's or the insert's native identity; a
    `ThirdPartyCid` is `UNAVAILABLE` `effect.third_party.unavailable_at_launch`. The values are
    `resolve_initial_values(descriptor, params)` of the strip's `ConsoleEntry` or `Effect`;
    quality and link mode are the `ConsoleSlot`'s or the `Effect`'s; bypass is the entry's.
- **D3. Overrides are checked as controls.** Each override names `{this owner, this section,
  control, lane}`. The row is found among `controls::list` rows of that section (`both` on a
  per-lane row names its `left` and `right` rows). No row is `INVALID_FIELD`
  `control.address.unknown`. Then `controls::check(&row.descriptor, lane, value, None)` runs; its
  error is `INVALID_FIELD` with the `ControlError` code (`control.lane`, `control.domain`,
  `control.read_only`, `control.inactive`). In the builtins section only controls 3 and 4 are
  accepted (`response.override.control` otherwise). A (control, lane) pair given twice is
  `response.override.duplicate`. Every check runs before any owner is prepared. Then the override
  values replace the resolved values (control 0: bypass; the link-mode row: link mode).
- **D4. A resolved target in the core.** `ResponsePreviewTarget` gains `ResolvedEffect {
  effect_id: &str, values: &[InitialParameterValue], quality, bypass, link_mode }`.
  `prepare_response_preview` accepts it without `apply_overrides`; `values` must hold exactly one
  value per descriptor parameter and lane in `default_initial_values` order
  (`InvalidParameter` otherwise). #1572's 256-override cap counts only the request's `override`
  fields, never the resolved values.
- **D5. Serving.** `SessionControlProvider::response_get` serves target 2: D3's checks, D2, then
  the same path as a configuration target (#1573 D7: prepare, reply-length check, curves). The
  reply is #1573's `response_result`; `configuration_id` is echoed. The reply carries the committed
  revision in its header: the curve is the response of the committed values, which a live edit
  changes at once; render reaches them by the applied-revision watermark.
- **D6. Nothing is retained.** No catalog, handle or prepared owner outlives the request. A
  structural edit needs no invalidation.

## Deliverables

1. D2-D4 in `crates/host-core/src/response.rs`; D5 in `crates/host-core/src/control_provider.rs`.
2. The registry text of target 2 and its codes (`docs/CONTROL_PROTOCOL_REGISTRY.md`).
3. The tests below.

## Authorized paths

- `crates/host-core/src/{response.rs,control_provider.rs,lib.rs}`, `crates/host-core/Cargo.toml`
  (only if `effect-compiler` or `controls` is not yet a dependency), `crates/host-core/tests/`.
- New `crates/capi/tests/response_address.rs`.
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`.

## Non-goals

- No retained response catalog, no handles, no generations (#774's design is not built).
- No response for effects without a response provider (only the parametric EQ and the input
  filters have one).
- No applied (render) state: that is the `strip` target (#1578).

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`, `cargo test --locked -p capi`,
`cargo test --locked -p control-plane --features test-support`.

In `crates/capi/tests/response_address.rs`, on a 48 kHz session with two tracks, a parametric EQ
insert with asymmetric band values on track 1, a parametric EQ console slot (track 2 has its own
entry values) and a compressor insert:

1. **Committed values.** For the insert, the console entry of track 2, and track 1's builtins
   section with asymmetric HPF and LPF, the address reply's curves and masks are bit-equal to a
   configuration reply (#1573) that gives the same effect, quality, link mode and every value as
   overrides.
2. **Follows a live edit.** After a live `SET_CONTROL` (#1553) of one band gain, the next address
   reply equals the configuration reply with that gain, with no plan rebuild (the plan epoch is
   unchanged) and the revision advanced by one.
3. **What-if values.** An override of one band gain gives the curve of the committed values with
   that gain; the committed model, revision and snapshot do not change. `both` on a per-lane gain
   equals `left` plus `right`. Refusals: `left` on a shared parameter (`control.lane`), a gain
   outside its range (`control.domain`), an unknown parameter ID (`control.address.unknown`),
   builtins control 5 (`response.override.control`).
4. **Refusals.** An unknown strip or slot (`NOT_FOUND` `control.address.unknown`); a route owner
   or section own (`response.address`); the compressor insert (`UNAVAILABLE`
   `response.effect.unsupported`); a `cid` insert (`effect.third_party.unavailable_at_launch`).
5. **Nothing kept.** The provider's resource report is the same before and after 100 address
   requests.

## Test value

- Gate 1 turns red if the address resolves to other values than the ones a rebuild prepares.
- Gate 2 turns red if the address reads a stale copy (the prepared entry) instead of the
  committed model.
- Gate 3 turns red if an override bypasses the control validator or writes the model.
- Gate 4 turns red if an address that names no response owner is answered or misclassified.
- Gate 5 turns red if a request leaves a prepared owner or a catalog behind.

## Dependencies

- #1573 (the wire and the serving path), #1551 (`control_address`), #1546, #1547 and #1549 (the rows,
  `controls::get` and `controls::check`), #1553 (gate 2's `SET_CONTROL`).

## Supersedes and shrinks

- Supersedes *Bind requested session responses by stable target IDs with engine-owned grids*
  (#774): a session effect or a strip's input filters is addressed by its stable decision-17
  address, resolved on demand from the committed model, on both hosts, with no retained catalog.
  See the amendment record.

## Decision record (attempt 1, 2026-10-24)

Scope note `scope/1575.md` (root-accepted) amends the spec:

- **Resolver signature.** Protocol-free `host_core::resolve_response_target(model, strip: &str,
  section: ResponseAddressSection, overrides: &[ResponseParameterOverride], sample_rate_hz) ->
  ResolvedResponseTarget` (owned; `target()` borrows it as `ResponsePreviewTarget::ResolvedEffect`
  or `InputFilters`). The provider maps the wire address (`response_address`) and refuses a
  non-strip owner, the `own` section or a present `control`/`lane` as `response.address`.
- **Override rows (D3).** Read with `controls::owner_rows_in` for the addressed strip alone (the
  rows `controls::list` gives for that owner, without listing the session), then
  `controls::check`. Applied after resolution: control `0` bypass, `LINK_MODE_CONTROL` link mode,
  parameters through the existing `apply_overrides` lane walk over the resolved values
  (`normalize_zero`d, as `controls::set` writes them).
- **Serving (D5).** `response_preflight` returns a `CheckedTarget`; `serve_checked` computes the
  limits once, builds the configuration or address target, and `serve_target` shares preparation,
  the reply bound and the curves. `serve_response_at` takes `Option<&SessionModel>` in place of
  its `session: bool`; the session-less entry passes `None` and keeps refusing an address.
- **Codes.** New `ResponseQueryError` variants in the one `refusal()` table: `Address`
  (`INVALID_FIELD response.address`), `AddressUnknown` (`NOT_FOUND control.address.unknown`),
  `ThirdParty` (`UNAVAILABLE effect.third_party.unavailable_at_launch`), `OverrideControl`
  (`INVALID_FIELD` + the validator's code), `Committed` (`VALIDATION_FAILED` + the resolver's
  code; unreachable on a validated model).
- **Tests.** Gates 1, 3 and 4 are host-core unit tests in `control_provider.rs` (a model built in
  the test); gates 2 and 5 are one C ABI row in `crates/capi/tests/response_get.rs`
  (`EditPath::Live`, revision + 1, the same curves on each of 100 requests), not a new
  `response_address.rs`. Verifier MINOR: `miso_engine_v1_plan_resources` counts render-plan bytes
  only and cannot see a kept response owner, so gate 5 rests on the design (no field on
  `SessionControlProvider`), not on a resource assertion.
- **Console lookup.** Resolved through `SessionModel::lower_strip`, the plan lowering itself, not
  a separate copy of its slot/entry split. The capi refusal case "the address target" is deleted: it is
  served now.
