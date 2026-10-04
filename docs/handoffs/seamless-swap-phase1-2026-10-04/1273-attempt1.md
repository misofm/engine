# #1273 attempt 1 verdict: PASS

PASS with minors. There is no BLOCKER and no MAJOR. The slice does what D1-D6 freeze, every gate
holds when re-run, and the key tests go red under the defects they name.

There are three MINORs, worth fixing in the same pass:

1. Nothing pins D1's "committed model **before** the transaction" at the C ABI. If the arm passes
   the prospective model instead, every capi test stays green, and a source whose declaration
   changed carries its old ring. One extra assertion catches it; I proved that with a probe.
2. Nothing pins D3's seek routing. If `seek` goes back to the current epoch, every capi test stays
   green.
3. The header and docs do not say what happens to a source whose declaration changed.

The answer to "can an ack ever precede a drop?" is no. Details are in the next section.

## Commit reviewed

- `git diff 2a6f2c10c 1338b063c`: commits `448baae85` (attempt 1) and `1338b063c` (the
  coordinator-authorized race move).
  - Exported with `git archive 1338b063c` to `/tmp/claude-1002/v1273/attempt1/`. I built and tested
    only there.
  - I ran mutations in a second export, `/tmp/claude-1002/v1273/mut/`, and reverted each one.
  - `tar --compare` shows the first export is still byte-identical to the commit.
  - Logs: `/tmp/claude-1002/v1273/logs/` and `/tmp/claude-1002/v1273/mutlogs/`. Mutation driver:
    `/tmp/claude-1002/v1273/{mutate.py,runmut.sh}`.
  - I did not build in or write to `/home/bl/misofm/wt-swap`. I did not judge #1274's uncommitted
    work there.
- Files changed:
  - `crates/capi/src/runtime/{control,compile,mod,tests}.rs`
  - `crates/capi/src/ffi.rs`
  - `crates/capi/include/miso_engine_v1.h` (comment only)
  - `crates/capi/tests/{resource_lifecycle,plan_swap_race}.rs`
  - `crates/capi/Cargo.toml`, `Cargo.lock` (one line)
  - `scripts/check-realtime-policy.sh` (one allowlist alternative)
  - `tools/audit/src/capi.rs`
  - `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`
  - the slice spec
- Every path is in the authorized set or the coordinator amendment. `plan.rs` is untouched.
- Both commit messages end with the required `Co-Authored-By` line. No `target/` or artifacts were
  committed.

## Gates re-run (all in the export, all green)

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (56 regions in 15 files) / ok |
| `check-realtime-audit-leak.sh`, `check-bench-policy.sh` | ok, ok |
| `cargo test --locked -p capi` | lib 38, `plan_swap_race` 2, `resource_lifecycle` 9: all pass |
| `cargo test -p capi --all-targets` with CI's `host-core/test-support,protocol/test-support,engine/realtime-audit` | 38 + 2 + 9 pass |
| `plan_swap_race`, repeated | 10/10 debug runs and 1 release run green, about 3.2 s each, no hang |
| `cargo test --locked -p audit` | 33 pass |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 100,000 calls; allocations, deallocations, locks, feature detection, logs, file and network I/O, syscalls, unwinds and render errors all 0; carry counts `(1, 0)` asserted; `pcm_digest` `7281b6c931e05dcc`, the implementer's value |
| `check-capi-abi.sh` / `--self-test` | ok (shared and static) / ok. `nm -D libcapi.so` exports 14 `miso_engine_*` symbols and no carry symbol |
| `check-cross-targets.sh` | PASS. Only the expected #1018 iOS `memset_pattern16` rows fail |
| `cargo test --locked -p host-core --features test-support` (not touched; consumer check) | all green |
| double-live oracle with `--nocapture` | Matches the attempt record exactly. Carried ring 10,608 bytes (2,416 overhead), carry program 8 bytes, inventory 62 bytes. Requirements: graph 549,774; source-total 13,660; source-overhead 5,468; capi 167,770 |

- **Worklet chain: not run, and not needed.** No crate compiled into the browser Wasm module
  changed. The slice touches only capi, tools/audit, scripts and docs, so the module is unchanged.
- **bench-support stays out of shipped builds.**
  - `cargo tree -p capi -e normal,build` has 0 `bench-support` and 0 `realtime-audit` entries.
    That holds on the host, `aarch64-apple-ios`, `aarch64-linux-android` and `--target all`.
  - The `-e features,no-dev --target all` graph has 0 `realtime-audit` entries.
  - `scripts/lib/product-crates.sh` uses `-e normal`, so bench-support does not join the product
    crates.
  - The workspace uses resolver 3, so the dev-dependency's features do not leak into the
    cdylib or staticlib.
- **CI reach.**
  - `test-debug-a` runs `--workspace --all-targets` with capi included, so it runs
    `plan_swap_race`.
  - The `aarch64-debug` leg runs capi's tests at `Simd4` (NEON).
  - The `aarch64-release` leg runs `audit capi`. Its validator checks counters and keys only.
- **Width.** Gates 1-4 run at `Backend::current()` (`Simd8`) locally. capi cannot choose its width,
  so the `Simd4` run happens only in CI's aarch64 legs. I did not verify `Simd4` locally. The slice
  adds no bank code: the carry is source-only and width-agnostic.
- **`pcm_digest` is not pinned anywhere live.**
  - `qualification.yml` checks only its format, `[0-9a-f]{16}`.
  - `run-aarch64-tests.sh` does not read it.
  - `ff6cdcb96cdcdad5` appears only in historical verdicts under
    `docs/handoffs/submix-sends-2026-10-02/verdicts/` and in this spec's attempt record.
  - No update is needed. The PR description must say that the value moved, as deliverable 3
    requires.

## The key questions

- **Is the producer move infallible, and can an ack precede a drop? No, it cannot.**
  - `adopt_persisting` (`host-core/src/source.rs:247`) only does a binary search and
    `Option::take`. It does not allocate, and it has no `?`.
  - It runs at `control.rs:861-870`, right after `prepared.commit(..)?` returns `Ok`.
  - Everything after it is infallible:
    - `replace_session_catalog`;
    - `pending_providers.push` and `reports.push`, whose capacity was checked at `:843-847`;
    - `reservation.commit()`;
    - the response write, whose size was admitted at `:749`.
  - Every refusal path returns before the move, including a protocol commit that fails. Mutation V2
    proves the order matters.
  - `ProtocolCommit` refusals leave nothing behind. A probe of mine showed the refusal disposes the
    token, the candidate plan and the provider and cancels the reservation. A retry is then
    admitted and carries (`carried_count` 1).
- **Is any submitted PCM lost between the commit and the swap? No.**
  - From the commit on, `submit` and `seek` go through `newest_providers_mut()`, which is the
    pending epoch. Its moved producer writes the very ring the running plan is still reading.
  - The successor takes that ring's consumer at the swap block.
  - A deferred swap keeps the predecessor rendering from the same ring. PCM is never stranded in a
    ring nobody reads, except for an added source, which only backpressures until the swap.
  - Gate 1's swap block plays block 6, which was queued before the commit. The race feeds every
    iteration under true concurrency, and only OK or ring-full backpressure is accepted.
- **Removed source. Correct.** After the commit, a submit returns `INVALID_ARGUMENT` with
  `source.id.unknown` (gate 2). Mutation V11 (fall back to the current set) turns it red.
- **Double-live counts each carried ring once.** In `validate_replacement_peak`, the source rows are
  `current + (prospective - carried)`. The candidate's own report counts allocated plus carried
  rings, because the plan owns them once it is active. The graph row includes the carry program.
  V6 and V12 turn the oracle red.
- **Wrong predecessor.**
  - The base is always the newest epoch.
  - Pending candidates are refused at `:810` before reservation, so at commit "newest" is
    `self.providers`, and that is the plan the candidate will displace.
  - Gate 4 and the race also defend the inventory chain: mutation V14 kept the predecessor's
    inventory in the candidate epoch, and swap 2 then failed to carry. Gates 1 and 2 stayed green
    under it.

## Edits to the shared #1053 touch points (exact list)

All of them are needed for D1-D5, and each is as small as it can be. `synchronize_plan_epochs`
(`control.rs:645-686`) is **unchanged**.

- **`control.rs` `ProviderEpoch` (`:12-37`).**
  - New field `inventory: PlanStateInventory`.
  - `current(sources, inventory)` and `candidate(sources, inventory)` each gained a parameter.
- **`control.rs` `Structural` arm (`:759-779`, `:790-793`, `:861-870`).**
  - `prepare_runtime(.., Some(SuccessorBase { inventory: &self.newest_providers().inventory,
    committed: self.controller.session().compiled().normalized_model() }))` replaces the
    `STRUCTURAL_SOURCE_STATE_POLICY` match.
  - The destructuring gained `carried` and `inventory`.
  - The call becomes `ProviderEpoch::candidate(sources, inventory)`.
  - `validate_replacement_peak` gained `carried`.
  - The `adopt_persisting` block was inserted after `prepared.commit(..)?`.
- **Elsewhere in `control.rs`.**
  - Removed: `StructuralSourceStatePolicy` and its constant (`:46-53` on the parent).
  - Added: the `#[cfg(test)]` `ProtocolCommit` phase (`:65-66`) and its branch in
    `ObservedPreparedToken::commit` (`:296-306`).
  - Added: `newest_providers` and `newest_providers_mut` (`:988-998`).
  - `submit` and `seek` now route through them (`:1010`, `:1024`).
- **`compile.rs`.**
  - `prepare_runtime` (`:448-532`) gained `successor: Option<SuccessorBase>`, and branches between
    `prepare_host_runtime_successor` and `prepare_host_runtime`.
  - `prepare_runtime` now passes the inventory into `prepared_capi_resources`, adds the carried
    rings to the three source rows and the carry program to the graph row, and returns `carried`
    and `inventory`.
  - `PreparedRuntime` gained two fields (`:15-18`).
  - New `CarriedSourceBytes` type (`:44-53`).
  - `capi_resources` gained an inventory epoch row (`:130`, `:168-169`), and
    `prepared_capi_resources` takes the inventory (`:237`, `:262`).
  - `validate_replacement_peak` gained `prospective_carried` (`:281`, `:300-319`).
  - `compile_children` passes `None` and keeps the inventory (`:557`, `:589`, `:655`).
- **#1256 and #1263 can extend this.** Their `prepare_host_runtime_with_live_controls_successor`
  call slots into the same `match successor`.

## Test value (one sentence per new or rewritten test; mutations run by me)

- **`a_c_abi_structural_transaction_keeps_the_source_playing`** (`tests.rs:1030`, gate 1).
  - Red when the arm prepares with no successor base (V5, fresh ring), when the producer is not
    handed over (V1), or when a post-commit submit goes to the current set (V3). Each was red here.
  - Before this slice, the only test of the swap block pinned the opposite: a silent block.
- **`removing_a_track_and_its_source_keeps_the_other_source_playing`** (`:1115`, gate 2).
  - Red when the removed source's submit still reaches the old set's producer (V11). Only this test
    catches that.
  - Also red under V1, V3 and V5.
  - The implementer reports it red under a host-core join by index (M5). I did not re-run M5.
- **`a_refused_structural_transaction_moves_no_producer`** (`:1196`, gate 3).
  - Red when the hand-over runs before the protocol commit (V2: `source.ring.vacated` at block 3,
    in the `ProtocolCommit` phase).
  - Gate 1 and `every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits` both
    stayed green under V2, so this is the only catch.
- **`control_calls_inside_a_plan_swapping_render_call_keep_replacement_live`** (`:1276`, rewritten
  for gate 4).
  - Red when the candidate epoch keeps a stale inventory, so that swap 2 cannot carry (V14:
    `carried_count` 1, expected 2). Gates 1 and 2 stay green under V14, so they do not catch it.
  - Also red under V1, V3 and V5 (pending-candidate feed).
- **`plan_swap_race.rs`, `control_calls_racing_plan_swapping_renders_never_wedge_replacement` and
  `resource_queries_racing_plan_swaps_always_find_the_published_row`** (moved and rewritten).
  - Red when any render call through the exported entry allocates or frees in a carrying swap
    block under real thread concurrency. My V15 boxed one value on `CarryOutcome::Carried` in
    `PlanState::render`, and both tests went red at block 2-3 in 0.04 s.
  - Also red under V1, V3, V5 (`source.ring.vacated`) and V14. Under V14 the failure is the generic
    10 s "nothing was queued" deadline, not a hang.
- **`structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic`** (amended).
  - The silent-block pin was deleted in the same change, which is correct (D4).
  - Its post-commit submit now asserts newest routing, and it is red under V3 and V5.
- **`capi_retained_bytes_charge_every_byte_the_compile_retains`,
  `double_live_oracle_drives_exact_and_one_below_c_caps` and `tiny_control_frame…`** (rows moved).
  - Red when the inventory is not charged (V7, all three).
  - The double-live oracle is also red when a carried ring is counted twice (V6, exact cap refused)
    and when the carry program is left out of the graph row (V12).
  - The rows come from live values, not literals.
- **`audit capi`.** Exits 134 with `(0, 0) != (1, 0)` when no transaction is applied (A1).

## Findings

### MINOR

1. **D1's pre-transaction committed model is not defended at the C ABI** (`control.rs:766`).
   - **Mutation V10.** Pass `prepared.get().prospective_session().compiled().normalized_model()`
     as `committed`. All 49 capi tests stay green.
   - **The defect it hides.** A source whose declaration changed then carries its old ring.
     Examples are `SetSourceContent` with new content or a new frame count. The old stem's queued
     PCM plays after the swap, and its generation and position carry over, so the host's fresh
     generation-1 feed is refused. That is state carried for an owner P1 says must restart.
   - **The test that already has the scenario.** `structural_command_keeps_protocol_…` makes exactly
     such a transaction (frames 512) and passes under V10, because it reseeks anyway.
   - **Why it matters now.** #1257 and #1260 are about to reshape this same arm.
   - **Fix.** After `.expect("second provider promotion and retirement")` (`tests.rs:771`), add:
     ```rust
     assert_eq!(children.plan.owner.carried_count(), 1, "a changed source does not carry");
     ```
   - **Probe result.** The assertion is green on `1338b063c`. It is red under V10 (`left: 2`,
     `right: 1`).
2. **D3's seek routing is not defended.**
   - **Mutation V4.** Revert `seek` to `self.providers` (`control.rs:1024`). All 49 capi tests stay
     green.
   - **The defect it hides.** Between commit and swap, a seek on a persisting source would return
     `source.ring.vacated`. A seek on an added source would return `source.id.unknown`. A seek on a
     **removed** source would be accepted (`Ok`) into a plan about to retire. That contradicts the
     new header text.
   - **Fix.** In gate 2, after the commit (`tests.rs:1171`), assert that
     `children.session.seek(AUX, 2, 0)` reports
     `(MISO_ENGINE_V1_INVALID_ARGUMENT, "source.id.unknown")`.
   - **Probe result.** Green on the commit. Red under V4 with `left: Ok(())`.
3. **The host contract does not cover a changed source.**
   - `miso_engine_v1.h:55-61`, `C_ABI_V1_QUALIFICATION.md:59-74` and
     `CONTROL_PROTOCOL_SEMANTICS.md:17` cover unchanged, removed and added sources.
   - They do not cover a source whose declaration the transaction changed. Such a source restarts in
     a new ring at generation 1, frame 0; PCM accepted for it before the commit is discarded with
     the old plan. A host needs to know that it must restart its feed.
   - D6 froze four statements, and the implementer followed them. Add one sentence to each of the
     three places. It is comment and doc only, with no size change.

### NIT

1. **The hidden accessor's shape.**
   - `plan_carry_counts` is `#[doc(hidden)] pub unsafe fn` (`ffi.rs:1053-1063`), while the spec says
     `pub fn`. Raw-pointer dereference makes `unsafe` the right shape.
   - The `#[cfg(test)] test_plan_carry_counts` wrapper (`:1065-1069`) goes slightly beyond "the
     accessor only". It is harmless.
2. **Gates 2 and 3 do not use the exported entries.** They go through `SessionState` and
   `PlanState`. The spec asks for exported entries only in gate 1, and gate 2's refusal check uses
   `SourceFailure::report()`, the exact mapping `miso_engine_v1_source_submit_planar_f32` returns
   (`ffi.rs:503-508`). Acceptable.
3. **`ProtocolCommit` is missing from the owner and credit matrix.** It is not in
   `every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits` (`PHASES` has 6).
   My probe shows it disposes and cancels everything and that a retry carries. Adding it to
   `PHASES` would pin that.
4. **The double-live oracle still uses `SetSessionId`** (`resource_lifecycle.rs:1273-1291`). #1260
   makes that trigger model-only, which breaks the oracle's swap premise. Whichever lands second
   should switch it to the muted-`UpsertTrack` trigger, as the spec's Coordination section says.
5. **Overlong lines.** `docs/C_ABI_V1_QUALIFICATION.md:163` is 128 columns. Doc comments at
   `resource_lifecycle.rs:634` and `:636` are 101 and 115 columns. These came from the edits; the
   neighbouring text wraps at 100.

## Not verified

- **`Simd4` execution of gates 1-4 and of `audit capi`.** This needs the CI `aarch64-debug` and
  `aarch64-release` legs. The rules forbid running AArch64 locally.
- **CI's full `test-debug-a` workspace sweep.** I ran capi under CI's features, plus audit and
  host-core. capi's only reverse dependency is `audit`.
