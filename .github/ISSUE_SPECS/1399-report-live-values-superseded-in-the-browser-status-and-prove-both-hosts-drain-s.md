# Report live_values_superseded in the browser status and prove both hosts drain strip cells alike

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Split from *Hold live values in latest-target cells on both hosts* (#1312), which it follows.
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A browser host reads `live_values_superseded` from its status record, with the same value the C
ABI reports through `COUNTERS_GET`. One script of strip edits renders bit-identically through both
hosts, so the two admissions and drains cannot drift apart.

## Context

- `WebStatus` (`hosts/host-web/src/lib.rs:1276-1302`) is 80 bytes; its last field is
  `reserved: [u64; 4]` at offsets 48, 56, 64, 72, required zero. The status is built at
  `:2186-2197` (`reserved: [0; 4]`). Layout asserts: `hosts/host-web/src/tests.rs:286`
  (`size_of::<WebStatus>() == 80`) and `:413` (`offset_of!(WebStatus, reserved) == 48`).
- Mirrors of the layout:
  - `status_fields` in `tools/parameter-metadata/src/abi_layout.rs:330-345` (`"reserved"`,
    `"u64[4]"`), which generates `sdk/src/generated/abi.ts`
    (`bash scripts/check-sdk-generated.sh` compares it);
  - the worklet's status reader, `readStatus()`
    (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:944-960`), which throws unless offsets
    48, 56, 64 and 72 are zero (`:946-949`); its result is spread into the `miso.status.v1` reply
    (`:1070`);
  - the main-realm host's status reply check, an exact field list and value validation
    (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1013-1017`, `:1121-1128`);
  - the status type `MisoStatus`, `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:846-857`,
    and its shipped copy `sdk/src/browser/shipped-host.d.ts:846-857`;
  - the hermetic harness's fake status reply (`scripts/test-web-audioworklet.mjs:159-164`).
- #1312 adds the cells, the session counter (one `Arc<AtomicU64>`, D2's per-cell unit: a `Both`
  record counts in each channel cell, so 40 `Both` edits before one block add 78), the C ABI's
  `CounterId::LiveValuesSuperseded` (D8) and the browser admission on cells (D7).
- `hosts/host-web/tests/boot_transient_budget.rs` is the model for a native integration test of
  host-web.

## Decisions frozen for this slice

- **D1. Status layout.** `WebStatus` gains `live_values_superseded: u64` at offset 48, and
  `reserved` becomes `[u64; 3]` at offsets 56, 64, 72. The size stays 80. Every mirror in Context
  follows: `status_fields` lists the new field and `u64[3]`; the worklet reader checks only 56, 64
  and 72 and returns `liveValuesSuperseded` (a `bigint`); the host's status field list gains it
  and validates it as a `u64`; both `.d.ts` copies gain `readonly liveValuesSuperseded: bigint`;
  and the harness's fake status reply carries it. *Publish the applied-revision watermark in the browser
  status* (#1349) then takes the three remaining words.
- **D2. Value.** Written at each status refresh from the session counter (a load), so it is never
  behind the last rendered block. No feature bit: an in-place V1 status field (decision 15,
  D15-3).

## Deliverables

1. D1-D2 in `hosts/host-web/src/lib.rs` and the mirrors; regenerated SDK files.
2. Gate 1 in `hosts/host-web/src/tests.rs`; gate 3 in `scripts/test-web-audioworklet.mjs`; gate 2
   in a new `hosts/host-web/tests/strip_cells_cross_host.rs`.

## Authorized paths

- `hosts/host-web/src/lib.rs` (`WebStatus` and its refresh only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/strip_cells_cross_host.rs` (new), `hosts/host-web/Cargo.toml`
  (`control-plane` as a dev-dependency).
- `tools/parameter-metadata/src/abi_layout.rs` (`status_fields` only),
  `sdk/src/generated/abi.ts` (regenerated), `hosts/host-web/web/miso-engine-v1-audio-worklet.js`
  (the status reader only), `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` (the status
  reply's field list and validation only), `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
  and `sdk/src/browser/shipped-host.d.ts` (`MisoStatus` only).
- `scripts/test-web-audioworklet.mjs` (the fake status reply and gate 3's case only).

## Non-goals

- The cells, the counter and the admissions (#1312). The watermark words (#1349).
- Effect, input and route cells (#1345, #1346, #1347): they reach the same counter with no change
  here.

## Objective gates

1. **Status (new host-web test).** 40 `Both` fader edits on one track before one render: the
   status reports `live_values_superseded == 78`, equal to the session counter, and offsets 56,
   64 and 72 read zero. The layout asserts move to the new field (`offset_of!(WebStatus,
   live_values_superseded) == 48`, `offset_of!(WebStatus, reserved) == 56`).
2. **Both hosts agree (new integration test).** One session and one script: two fader edits and
   two pan edits on the same track between two blocks, once through the control plane's
   `SESSION_TRANSACTION_APPLY` and once through host-web's command admission, each followed by 8
   blocks. The PCM is bit-identical and the two counters are equal (3: each fader channel cell and
   the matrix cell replaced once).
3. **The status reply carries the word.** In `scripts/test-web-audioworklet.mjs`, a worklet whose
   fake status block holds a nonzero word at 48 answers a status request with that
   `liveValuesSuperseded`, and the host accepts the reply; a reply without the field is refused.
4. Commands:
   - `cargo test --locked -p host-web --features test-support`
   - the SDK job's `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts` and
     `bash scripts/check-sdk-types.sh`, after building the artifacts as `qualification.yml` does
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`, and the
     browser legs of `qualification.yml`'s `browser` job
   - `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a status word at the wrong offset or never written.
- Gate 2: the two hosts draining differently (order, `Both` folding), which breaks #1054's
  cross-host bit-identity for two edits in one block.
- Gate 3: a worklet reader that still rejects a nonzero word 48 (every status read would throw
  after the first superseded value), or a host check that refuses or drops the new field.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).
