# Pad compressor banks with inactive lanes

Slice P2c of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M1, M2 and amendment 6 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

A console compressor slot must bank at every track count (decision 12, "Banking"). After P2a, the
compressor's `bind_homogeneous_bank` (`crates/compressor/src/lib.rs:756`) still declines a padded
request.

- It declines a sidechain (`:786-795`), which console slots never carry (decision 12,
  "Sidechain"). It refuses non-native widths (`:800-802`).
- Its whole-bank D7 recovery resets every lane.
- Its whole-bank fast paths couple cost, not bits:
  - the idle-lane guard (`docs/rulings/compressor-idle-lane-guard-console-under-resolved.md`: "one
    automated track drags every other lane of its bank");
  - silent admission (`block_is_positive_zero` over `frames * lanes`,
    `crates/effect-runtime/src/bank.rs:137`).

## Smallest closable slice

Opt the compressor into padded requests under P2a's contract:
- a padded lane carries a clone of an active member's request, is fed `+0.0`, and its output is
  discarded;
- D7 recovery and reports attribute active lanes only.

A padded lane's detector and gain smoother stay at their rest state on `+0.0` input, so a silent
padded lane never keeps an otherwise silent bank out of silent admission.

Authorized paths:
- `crates/compressor/src/lib.rs` (bank binding, the D7 path and lane bookkeeping only);
- `crates/compressor/src/kernel.rs`, only if a padded lane needs a guard that costs nothing on
  active lanes;
- their tests, and this spec.

## Relation to #889

- This slice **supersedes #889's absent-member half**: a cohort of fewer than W compressor tracks
  binding as one bank.
- **#889 is amended** to keep only its identity-slot half: an insert cohort whose member lacks the
  compressor slot. It is insert-only and optional, and no console slice depends on it. See the
  amendment appended to #889's spec.

## Dependencies

- *Let an effect bank bind a partial group with inactive lanes* (P2a, #1088).

This is batch C2. P2b-P2e edit disjoint effect crates, so after P2a merges they may land in
any order, one merge each.
- It reuses the test shape of *Pad parametric EQ banks with inactive lanes* (P2b) but does not
  depend on it.

## Objective gates

1. For every active count 1..W-1, a padded compressor bank's active lanes are bit-identical to the
   same tracks rendered per node. This holds on random input, on the compressor's fixtures, and on
   blocks where active lanes ramp or are cut mid-ramp (the #1069 shape), at Simd8 on x86-64 and at
   Simd4 through `scripts/run-aarch64-tests.sh` or the wasm gates.
2. Coupling rule: active lanes' bits do not depend on the clone source, and every link mode
   combines L and R of one lane only.
3. A bank whose active lanes are all silent still takes silent admission. A padded lane that
   defeated it would turn this test red. A padded lane fed `+0.0` produces exactly `+0.0` out (not
   `-0.0`, not a denormal) and keeps its state finite and at rest, block after block (P2a verdict,
   L4).
4. D7: a planted non-finite state in one active lane recovers and reports that lane alone.
   A bypassed lane counts as active: a bypassed lane fed a tripping value (for example `1e30`
   behind enough legal gain) leaves every enabled bank-mate's bits unchanged (P1 verdict, M2).
5. `cargo test -p compressor -p graph-compiler -p graph` pass, as do
   `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` and the realtime
   audits. PR evidence: console digests are unchanged.

## Non-goals

- No kernel arithmetic change on active lanes.
- No ramp-path change.
- No planner policy change.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.

## Attempt 1 evidence

Terra, 2026-09-30. Branch `codex/1090-pad-compressor-banks` from `2320454c` (P2a on batch C2), with
P2a's verdict commit `b2027254` merged (`a656d599`). Commits `b11f237a`, `b6c568b3`, `ed4795df`,
`67b95c6a`, `e6746dee` and this record. Only `crates/compressor` and this spec changed.

### What landed

- **Binding** (`lib.rs`). The #1088 guard is gone. `BankParts::validate` is the old member loop,
  unchanged in order: the shape, then every lane's request, padded lanes included (a clone passes
  exactly when its member does). Then the three `Ok(None)` fallbacks: a heterogeneous program, a
  connected sidechain, a width this build does not run. `BankParts::bank::<L>` builds either width.
  A padded lane runs the request it was given; nothing in the kernel knows it is padded.
- **D7 per lane.** `finish_lanes` replaces the whole-channel `kernel::finish_channel` in `render`
  and `render_mono`. A clean block costs what it did (one `check_block` scan). A rejected block
  zeroes (`andnot`, so exactly `+0.0`) and resets the envelope of the failing lanes only. At
  `L = f32` that is the old policy, so a lane is recovered exactly as its per-node instance is.
- **Lane bookkeeping.** `PreparedCompressorBank::active` (a lane bitmask). Only an active lane's
  failure is reported. A span addressed to a padded lane is neither applied nor counted. A padded
  lane is not a track: snapshot and restore answer `effect.state.track`.
- **Why nothing else is needed.** On `+0.0` input every legal threshold and knee leave the silent
  detector level (`-160` dB) below the knee, so a padded lane's target and envelope stay `+0.0` and
  every output word is `+0.0`. The whole-bank decisions that remain (silent fast path, ramping
  prefix length, all-wet arm, link) couple cost, not bits; the bank's doc says why for each.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. Padded bank = per node | `src/padding_tests.rs::a_padded_bank_renders_each_member_as_its_own_instance`: 21 seeds at **Simd4 and Simd8 on every host** (a bank built by the factory's own `BankParts`, minus only the host-width gate, driven through the production trait bodies). Seeds reach every active count `1..W` under every link mode; random launch rate, in-domain values (edges included), every `Profile` (hostile NaN, infinities, `>= 1e30`) and whole-bank silence; automation points in blocks from 1 frame up, so points cut ramps in flight (the #1069 shape); a third of cases collapsed (`process_bank_mono`). Each member's output words (class-A), per-block reports and final payload equal its per-node instance. Reach, asserted: 253 blocks where one member was rejected and another not, 139 cut ramps, 14 collapsed cases. `tests/padding.rs::the_fixtures_render_through_a_padded_bank_as_they_do_per_node`: the 7 conformance PCM fixtures at their own launch rates, through the factory at the native width, members from the standing console's compressors, every active count and link mode, blocks of 128/100/37/64/1. | green (Simd8 native, Simd4 and Simd8 unit) |
| 2. Coupling rule | The differential renders every case once per clone source (every member in turn); each must equal the per-node oracle. Heterogeneous neighbours under all three link modes. | green |
| 3. Silent admission, and `+0.0` out at rest | `a_silent_padded_bank_takes_the_silent_fast_path` (test-only `SILENT_ADMISSIONS` counter): every count `1..=W`, dual and collapsed, both widths; the fast path skips exactly every silent block after the first, as the full bank does. The differential also asserts, every block, that each padded lane writes `+0.0` bits exactly (not `-0.0`, not a subnormal) on both channels, binds at rest (`+0.0` envelope, no ramp in flight, finite words) and keeps that state bit for bit, stray spans included. | green |
| 4. D7, one lane alone | `a_planted_envelope_recovers_its_own_lane_alone`, both widths, every count and lane and channel: an `f32::MAX` dB envelope planted on a member is rejected once, reported on that lane and channel only, and recovers; every member equals its per-node instance (the planted one's carries the same word). A NaN `mix` planted on a padded lane is rejected every block, recovered to `+0.0`, reported against nobody. **Bypassed lane:** `tests/padding.rs::a_bypassed_lane_fed_a_tripping_value_moves_no_bank_mates_bit` drives the real `BypassShunt` as `ConsoleEffectBankStage` does. The bypassed lane (ratio 1, makeup +24 dB) is fed `±1e29` on 3 of 8 blocks, counts `2..=W`, first and last member. Its wet block is rejected (asserted), its output is its dry input, and every enabled member's bits and reports equal both the render where it was fed noise and its per-node instance. | green |
| 5. Tests, policies, audits | `cargo test -p compressor -p graph-compiler -p graph` dev (in CI's debug-a and debug-b sets below) and release. `check-effect-runtime-policy.sh` (+ its test), `check-realtime-policy.sh` (54 regions) and `test-realtime-policy.sh`. Audits below. | green |
| 5. Console digests (PR evidence) | `console-workload`'s ignored `digests` harness, 64 blocks of all 22 native session rows, release, at head and with `crates/compressor/src` reverted to `b2027254`: **identical**. | identical |

**Red mutations** (scratch runner, each reverted; the tests named went red):

| Mutation | Red in |
|---|---|
| whole-bank D7 (the old `finish_channel`) | differential, planted, bypassed-lane |
| whole-bank D7 in `render_mono` only | differential |
| recovery keeps the envelope | planted |
| recovery does not zero the lane | planted |
| mask built one lane off (`2 << lane`) | differential, planted, bypassed-lane |
| padded lane's failure reported | planted |
| spans applied/counted on a padded lane | differential |
| padded lane counted as a track | differential, `a_padded_lane_is_not_a_track` |
| padded lane bound off rest (`-1` dB envelope) | differential, silent admission |
| padded lane bound from zeroed parameters | differential (binds off rest: non-finite words) |
| #1088 decline restored, before or after the member loop | binding, fixtures, bypassed-lane, payload tests |
| width gate hoisted above the member loop | `a_padded_request_binds_after_every_lane_is_validated`, `bank_fallback_never_hides_...` |
| sidechain gate hoisted above the member loop | the same two |
| padded lanes' requests not validated | the binding test |

P2a's verdict follow-ups are in: masks are members-first (the binding test pins
`effect.bank.mask_not_prefix`), gate 3 gained its L4 clause above, and the binding test malforms
lane 1, a non-first member, under a padded mask at the native width, at an unavailable width and
with a connected sidechain (L1).

**Also run, all green:**

- `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D
  warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`, clean except
  the known `tools/console-workload/src/lib.rs:374`.
- CI's debug-a set: 92 binaries, 1,103 passed, 10 ignored. Debug-b DSP set: 150 binaries, 804
  passed, 28 ignored. Release (`compressor`, `graph-compiler`, `graph`, `effect-contract`, `audit`,
  `bench`, `console-workload`, pinned console digests included): 56 binaries, 511 passed.
- `scripts/check-cross-targets.sh` (AArch64 iOS and Android product crates checked and linted,
  wasm `simd128`); `cargo check` and `clippy -D warnings` of `compressor --all-targets` for
  `aarch64-linux-android` and `aarch64-apple-ios`; `host-web` for wasm `simd128`;
  `run-aarch64-tests.sh`'s no-silent-skip scan over `crates/compressor`: no match. No AArch64 runner
  here: CI's `aarch64-debug` runs the integration tests at native `Simd4`.
- `scripts/run-wasm-gates.sh`: native, wasm `simd128`, V8 EQ loops.
- Audits (release): `audit capi`, `delay`, `compressor`, `parametric-eq` (100,000 blocks each) and
  `gate-expander`, 0 violations; the builtins, builtins-graph and graph traces; the protocol audit;
  the realtime probes and 1,000,000-block trace; the builtins probe mutation tests; the
  effect-contract 1,000,000-call trace.
- Policies: workspace (+ test), session, env vocabulary, bench, test-support CI, script
  reachability, host-core, protocol control, realtime audit leak, artifact evidence leak, lane,
  unfused seal, rack, builtins, graph, effect-runtime fixtures, conformance boundaries, EQ render
  contract, CI path routing; `check-effect-contract.sh target/release/bench` (8 factories);
  `check-graph-determinism.sh` (100/100); console and builtins fixtures; `check-capi-abi.sh`;
  `check-scalar-oracle-absent --native`.
- Artifact gates on a fresh build: `check-web-audioworklet.sh --without-metadata-regeneration`
  (render-export callgraph closure included), `check-browser-expected-resources.py --artifacts`,
  `check-scalar-oracle-absent --wasm`, `test-web-audioworklet.sh`, the V8 spill gate and its
  self-test.

**Scratch probes (PR evidence, deleted):**

- **Through the real planner, graph and rack.** A temporary `graph-compiler` unit test: the
  intended console at every track count 1-11, compressor bypass none / every third / track 0 / all
  but track 0, under the test-only `EveryGroup` policy with the **real** compressor factory (EQ and
  limiter behind P2a's full-mask double). Every partial group bound a padded compressor bank, and
  all 44 renders (24 blocks) equal the bank-free per-node oracle's bits.
- **Allocation.** Under `bench_support::alloc` and the realtime audit scope: 20,000 blocks of 8
  padded banks (every member count 1-8), dual and collapsed, a `1e29`-behind-+24 dB trip every 7th
  block (22,856 rejected lane-blocks): 0 allocations, 0 deallocations, 0 syscalls.

**The AudioWorklet artifact changes, and this slice does not re-pin it.**

| | Module digest | Size |
|---|---|---|
| Base (`b2027254`) | `b2eeb2d98e2013bffc1271828291bbf3f59d1ba233491bd66703e6f5fc53175d` | 3,262,117 B |
| Head | `d899416ccbf23e9512776e49984d351edb44d404884f832e583e83efbd135151` | 3,262,562 B |

+445 B: code +594 B, names -147 B, two fewer functions. `run-wasm-gates.sh` built the same head
module. `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` still holds `6c952a2c…`,
stale since before P2a.

### Path deviations, for Sol

- `kernel.rs`, not for a padded-lane guard: `finish_channel` (the whole-channel form) is now
  `#[cfg(test)]`, because production calls `finish_lanes` and the kernel tests' pinned #1006 digest
  folds the old form; and the all-wet arm's doc now names the per-lane check. No arithmetic moved.
- `src/padding_tests.rs`, a unit-test module declared from `lib.rs`: it needs both widths on one
  host and the internal witnesses.
- A `#[cfg(test)]` counter at the two silent fast-path returns in `Instance::render`/`render_mono`.

### Residuals

- **A NaN envelope is never recovered** (pre-existing, not this slice's). `fast_gain_from_db`
  clamps a NaN to `2^-126`, so a lane whose envelope turns NaN writes finite output and D7 never
  sees it; the NaN is sticky. Identical per node and in a bank. A planted `+inf` envelope reaches
  it in one frame (`c * (t - inf) + inf`). No legal input reaches it; it may deserve its own issue.
- A block whose only spans address padded lanes still withdraws the whole-bank silent claim
  (`!block.automation.is_empty()`). Cost only; no planner sends such a span.
- `effect-runtime/src/bank.rs`'s `finish_channel` doc still calls the compressor its first caller;
  only the compressor's kernel tests call it now. Outside this slice's paths.
- The differential reaches the silent fast path once; gate 3's own test is what covers it.
