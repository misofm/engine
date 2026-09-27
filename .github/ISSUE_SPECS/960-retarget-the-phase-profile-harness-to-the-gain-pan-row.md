# Retarget the phase-profile harness to the gain/pan row

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (owner-directed removal, 2026-09-27; option (b) per the owner's second ruling below).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

1. **Authorized paths:** `tools/console-workload/tests/` (the new `gain_pan_profile.rs`),
   `crates/graph/src/lib.rs` and `crates/rack/src/lib.rs` (test-support probes only), and this spec.
2. Add sub-phase probes inside the bank chains (gather, each slot, fold, scatter), because the
   single `BANK` phase hides everything that matters on the gain/pan row.
3. Target the production-feed row `sixty_four_track_gain_pan_ring`. Land after #957.

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Smallest closable slice

- **S5. Retarget or delete the phase-profile harness.** It covers `graph::test_only_phase_profile`
  and the deleted `plumbing_profile.rs`.

Ruling: retarget, do not delete. `graph::test_only_phase_profile` (test-support only) stays, and a `gain_pan_profile` harness replaces the deleted `plumbing_profile.rs`, so the pure-path target can be diagnosed per phase.

## Dependencies

#956.


## Attempt 1 evidence

Implementer: Claude Opus 5.5, 2026-09-27, branch `codex/960-retarget-phase-profile` on #958's
branch head `403afba9` (the builtins-less path already gone below it). Implementation commit
`362b3683`: 5 files, +930 / -13.

### Probes added

All of them sit behind `test-support`; `cargo tree -p bench -e features,no-dev` names no
`test-support` feature anywhere in the bench's graph.

- **`crates/rack/src/lib.rs`: `test_only_bank_phase_profile`** (`#[cfg(feature = "test-support")]`,
  `#[doc(hidden)]`). Thread-local accumulators and an `Instant` clock, like the graph profile. A
  chain run is split into `PROLOGUE` (every slot's `begin_block` drain and the collapse decision),
  `GATHER` (gather, collapsed gather or resident-input copy), `SLOT + i` (each slot by chain
  position, eight positions), `SEAM` (the collapse copy), `SCATTER` (transposes and copies),
  `FOLD` (the members' `fold_resident`, `fold_cohort` and `fold_plane`, including the resident
  offer's checks) and `AUX` (`accumulate_aux`). Probe sites: `run_with_input` (start, gather, each
  prefix and seam-side slot, seam, scatter, aux, finish), `scatter` (per-lane `fold_plane`, the
  cohort), and `scatter_tiled` (the resident offer, per-lane `fold_plane`, the staged cohort). The
  snapshot carries nanoseconds, entries per sub-phase, chain runs and clock reads.
  `BankStage::test_only_stage_name` (default `type_name::<Self>()`, same feature) labels the slots.
- **`crates/graph/src/lib.rs`:** `test_only_phase_profile::bank` re-exports the rack module, and
  `enable` and `reset` drive both profiles. The module's docs now point at #960 and this harness,
  and the `SOURCE` and `BOUND` docs describe the ring row. `graph::test_only_phase_profile` is kept
  as it was otherwise.
- **`tools/console-workload/tests/gain_pan_profile.rs`**, replacing the deleted
  `plumbing_profile.rs`:
  - `phase_profile` (`#[ignore]`) renders `sixty_four_track_gain_pan_ring` (warm-up 512 blocks,
    then 3 repeats of: 4000 blocks timed per block with probes off, 4000 as one loop with probes
    off, 4000 as one loop with probes on). It prints the graph phases, the chain sub-phases and the
    "outside chain" residual (`BANK` less the sub-phases: the unit's dispatch, member setup and
    observation). Cycles are derived from a calibrated core clock. Each row is also given net of
    the probes: one in-situ probe cost per entry, where the cost is (probed loop mean - unprobed
    loop mean) / clock reads. It asserts nothing about time.
  - `digests` (`#[ignore]`) prints the 64-block digest of every native session row with the
    probes off, and asserts the probes-on digest equals it.
  - `probes_split_the_bank_units_and_move_no_bit` (standing, untimed, in `cargo test`) checks the
    ring row: one prologue, gather, scatter and fold per chain run, no seam, each slot named exactly
    when it ran, the sub-phases nesting inside `BANK`, and the digest with the probes on equal to
    the digest with them off.
- **rack unit test** `bank_phase_probes_split_every_scatter_path_and_move_no_bit`
  (`--features test-support`). At both widths it covers five scatter paths: direct, a declined
  resident offer followed by the staged cohort, per-lane folds, the partial bank, and the aux
  epilogue. For each it asserts exact `[SCATTER, FOLD, AUX]` entries, the slot names, and the
  profiled words bit-equal to the unprofiled ones. Red mutation: charging the staged cohort to
  `SCATTER` instead of `FOLD` fails it (`left: [3, 1, 0]`, `right: [2, 2, 0]`); restored.

### Production build carries none of it

`cargo build --locked --release -p bench` at the base `403afba9` and at `362b3683`:

| check | base | after |
|---|---|---|
| `.text` sha256 (`objcopy --only-section=.text`) | `8b4cc865…1748d8` | `8b4cc865…1748d8` (byte-identical) |
| `.rodata` sha256 | `68b9faf4…2ba103` | `68b9faf4…2ba103` (byte-identical) |
| `nm -S` name, size and type, 3450 symbols | — | 0 lines differ |
| probe symbols (`nm -C`: `test_only`, `phase_profile`, `stage_name`) | 0 | 0 |
| probe strings (`test_only_bank_phase_profile`, `test_only_phase_profile`, `test_only_stage_name`, `enter_slot`) | 0 | 0 |

The only allocated section that differs is `.data.rel.ro`: 102 `u32` words, every one a
panic-location line number moved by the inserted source lines (deltas 21 to 226). The debug line
tables differ too. The shipped instructions are the base's, byte for byte.

**Digests (hard stop): unchanged.** I rendered the 64-block digest of all 17 native session rows
(`native_session_rows()`: `WORKLOADS`, the ring row and the metered row) in release in two
states: at the base (crate changes stashed, a throwaway uncommitted test) and at the final tree
(`digests`). All 17 are identical. The probes-on digest equals the probes-off one for every row;
that covers the collapse seam on the mono rows. `chain_shape.rs`'s four pinned base digests also
pass.

| row | digest |
|---|---|
| `nine_track_baseline` | `e7c6ef01770ab7da98d4b793a6a817bd50a4dfe682321ecfc023ddc275285a80` |
| `nine_track_ragged_strip` | `17613a3ab693d3f0dfc457b41f77669f880581fa19c433941a1ef2437684198a` |
| `sixty_four_track_console` | `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de` |
| `one_twenty_eight_track_stretch` | `cba2c94f81544caad0945f0720480b568b1a47808d25fd95911f61bd37f5f9b1` |
| `sixty_four_track_eq_only` | `9b2c56a1da62ebda8aef595973870ea077477d209aacdcd6321637e949d5c828` |
| `sixty_four_track_compressor_only` | `95c9375429fbca3449bf9c5134508a6220f17fc9adda0b3267c061b75bb14175` |
| `sixty_four_track_builtins_only` | `b63eccd09c19eb7a6e0608144024ac5b14c7d5f7d1c56012cbbd49d6aad8f7f0` |
| `sixty_four_track_dispatch_only` | `15688888612d161e507bc400b9eed356fc1776797c8c66ca52d1e7c9114d3a2d` |
| `sixty_four_track_idle` | `de2f256064a0af797747c2b97505dc0b9f3df0de4f489eac731c23ae9ca9cc31` |
| `sixty_four_track_console_legacy` | `f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177` |
| `sixty_four_track_eq_comp_simd1` | `f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177` |
| `sixty_four_track_gain_pan_only` | `01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4` |
| `sixty_four_track_console_mono` | `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7` |
| `sixty_four_track_console_mono_dual` | `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7` |
| `sixty_four_track_console_half_mono` | `4a656cdf63882999b720b7dd765c1b4f7bbcb95e2ab38c10445f71d269665180` |
| `sixty_four_track_gain_pan_ring` | `01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4` |
| `sixty_four_track_console_metered` | `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de` |

### Profile of `sixty_four_track_gain_pan_ring` (descriptive, one run)

The single recorded run of the committed harness:

```text
CARGO_INCREMENTAL=0 cargo test --locked --release -p console-workload --test gain_pan_profile --no-run
taskset -c 31 target/release/deps/gain_pan_profile-f4d0bb42ce86a824 --ignored --nocapture \
    --test-threads 1 phase_profile
```

- **Host and time.** AMD EPYC 7313P (Zen 3), 32 logical CPUs, pinned to CPU 31, at
  2026-09-27T12:08:02Z.
- **Code.** Tree `362b3683`; test binary sha256 `3991ab71…bf27538`.
- **Load.** Another verifier was building on this host. I sampled the 1-minute load average every
  20 s from 11:57 (34 samples, from 3.90 down) and started the run at the first sample under 2.0.
  That was 1.98, the lowest observed; the load was `1.98 3.26 3.67` at both the start and the end
  of the run.
- **Row shape.** Backend `Simd8`, quantum 128, 64 tracks, 16384 lane-samples per block.
  `bank_shape` is `[8, 24]`, and all 64 routes are folded. The unit runs are `8 x bank, 1 x
  output`: the input units are not dispatched, and every claim is gathered in place.
- **Clock.** The core clock was derived at 3.697 GHz from a dependent `f32` add chain at 3 cycles.
  One clock read costs 20.0 ns.

```text
repeat 0: probes off  min 8716 ns  p50 8847 ns  p95 8947 ns  mean 8884 ns  (p50 = 32709 cycles/block, 1.996 cycles/lane-sample)
repeat 0: probes on   mean 10909 ns/block, 69.0 clock reads/block, 29.3 ns per probe in situ (clock read alone 20.0 ns)
repeat 1: probes off  min 8726 ns  p50 8847 ns  p95 8947 ns  mean 8893 ns  (p50 = 32709 cycles/block, 1.996 cycles/lane-sample)
repeat 1: probes on   mean 10905 ns/block, 69.0 clock reads/block, 29.2 ns per probe in situ (clock read alone 20.0 ns)
repeat 2: probes off  min 8726 ns  p50 8847 ns  p95 8957 ns  mean 8900 ns  (p50 = 32709 cycles/block, 1.996 cycles/lane-sample)
repeat 2: probes on   mean 10915 ns/block, 69.0 clock reads/block, 29.2 ns per probe in situ (clock read alone 20.0 ns)
```

Median of the three repeats, per block. The probes cost about 29 ns each in situ: 69 reads
lift the block from 8893 to about 10909 ns. "Net" subtracts one probe cost per entry. It is an
estimate: small nets, such as `enter`'s -1.4 ns, are inside its error.

| phase                          | entries | probed ns |    net ns |   net cyc | net c/ls |  share |
|--------------------------------|---------|-----------|-----------|-----------|---------|--------|
| enter                          |     1.0 |      27.8 |      -1.4 |        -5 |  -0.000 |  -0.0% |
| source                         |     1.0 |      36.5 |       7.3 |        27 |   0.002 |   0.1% |
| output                         |     1.0 |      56.1 |      26.9 |        99 |   0.006 |   0.3% |
| bank (whole unit)              |     1.0 |   10740.7 |    8844.3 |     32699 |   1.996 |  99.6% |
|   prologue                     |     8.0 |     388.0 |     153.8 |       569 |   0.035 |   1.7% |
|   gather                       |     8.0 |    2268.3 |    2034.1 |      7520 |   0.459 |  22.9% |
|   slot 0 (BuiltinStage)        |     8.0 |    2805.1 |    2571.5 |      9507 |   0.580 |  29.0% |
|   slot 1 (BuiltinStage)        |     8.0 |    1033.8 |     800.4 |      2959 |   0.181 |   9.0% |
|   slot 2 (BuiltinStage)        |     8.0 |    1328.6 |    1094.2 |      4046 |   0.247 |  12.3% |
|   scatter                      |     8.0 |     268.8 |      34.4 |       127 |   0.008 |   0.4% |
|   fold                         |     8.0 |    2086.8 |    1852.0 |      6847 |   0.418 |  20.9% |
|   outside chain                |     1.0 |     565.0 |     302.2 |      1117 |   0.068 |   3.4% |
| total (entries: clock reads)   |    69.0 |   10864.4 |    8877.9 |     32823 |   2.003 | 100.0% |

Slot 0 is the input section, slot 1 the fader, and slot 2 the pan matrix (see the notes below).

**Reading, descriptive only.** Unprobed, the row costs about 2.00 cycles per lane-sample (p50 8847
ns). Almost all of it (99.6 % net) is the eight bank units. Inside them the net cost is:

| part | share |
|---|---|
| input section: sanitise, trim, one `add(+0.0)` and the boundary scan (see below) | 29 % |
| gather: the planar-to-AoSoA transpose, reading the played planes in place | 23 % |
| route/master fold, from the resident block | 21 % |
| pan matrix | 12 % |
| fader | 9 % |
| per-unit dispatch, setup and observation outside the chain | 3.4 % |
| prologue drains | 1.7 % |
| scatter | about 0 |

The input section is that short because its identity filters are elided. The row's identity
input builtins let the chain reduce to `identity_chain_block` in
`crates/lane/src/kernels/builtins.rs`, per `sixty_four_track_dispatch_only`'s doc. The scatter is
about 0 because the resident fold takes the block before any transpose. These numbers are not the
console benchmark and gate nothing.

### Gates

All run with `CARGO_INCREMENTAL=0`, the worktree's own `target/`, and `set -o pipefail` where
output was piped, on the committed implementation tree (`362b3683`) with no scratch file present.

| gate | command | result |
|---|---|---|
| fmt | `cargo fmt --all --check` | pass |
| clippy | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| doc | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| graph + rack | `cargo test --locked -p graph -p rack` | pass: graph lib 107, rt10 1, rt1 1, rt9 1; rack lib 40, `console_bank` 10, `mono_reengage` 4 |
| graph + rack, test-support | `cargo test --locked -p graph -p rack --features test-support` | pass: graph lib 107, rt10 1, rt1 1, rt9 8; rack lib 41 (the new probe test), `console_bank` 10, `mono_reengage` 4 |
| console-workload | `cargo test --locked -p console-workload` | pass: lib 8, `automation` 4, `chain_shape` 23, `gain_pan_profile` 1 (+2 ignored), `placement` 3 |
| release bench | `cargo build --locked --release -p bench` | pass; the production-build check above |
| digests | release `gain_pan_profile -- --ignored digests`, and the base run | 17 of 17 identical to the base; probes on equal to probes off |
| policy | `scripts/check-realtime-policy.sh`, `check-graph-policy.sh`, `check-rack-policy.sh`, `check-workspace-policy.sh`, `check-realtime-audit-leak.sh` | ok (57 marked regions in 16 files); PASS; PASS; ok; OK |
| policy self-tests | `scripts/test-realtime-policy.sh`, `test-graph-policy.sh`, `test-rack-policy.sh` | ok; ok; ok |

The console benchmark was not run.

### Deviations and notes for the verifier

- **Two manifest lines outside the authorized paths.** `crates/rack/Cargo.toml` gains
  `[features] test-support = []`, and `crates/graph/Cargo.toml` changes `test-support = []` to
  `test-support = ["rack/test-support"]`. `rack` had no feature to gate a probe behind, and the
  amendment's "`crates/rack/src/lib.rs` (test-support probes only)" cannot be met without one.
  Nothing else was a workable gate:
  - `cfg(test)` does not reach the console harness;
  - a `--cfg` rustflag would override the pinned `+avx2,+fma` target rustflags;
  - an always-compiled generic hook would put the API in production.

  Both dependency lists are unchanged: the rack and graph policies pass, and `Cargo.lock` is
  unchanged.
- **One feature-gated trait method.** `BankStage::test_only_stage_name` has a default body, so no
  implementor changed. It is the slot label and exists only with the feature.
- **Builtin slots print the wrapper's name.** Every builtin slot prints
  `graph::runtime::BuiltinStage` because the concrete type sits behind the graph wrapper's
  `Box<dyn …>`. Forwarding the inner name would need a forwarding method in
  `crates/graph/src/runtime.rs`, which is not authorized.
  - The table reads slot 0, 1 and 2 as the input section, fader and pan matrix. That order comes
    from how the run is built: `runtime.rs` assembles the chain from the bank run in member order
    `PostInputBuiltins -> PostFader -> PostMatrix`, the strip's dataflow order (AGENTS.md chain).
  - The row has 3 slots per chain (`bank_shape` `[8, 24]`), so the fader and matrix are not paired.
- **The graph's side of a bank unit is not probed separately.** It has two parts:
  - the member reductions (none on this row) and the `ArenaMembers` setup, in
    `runtime.rs::execute`;
  - the render loop's per-unit dispatch and `observe_unit`, in `lib.rs`.

  Both are charged to `BANK` outside the chain run, and together they are the "outside chain"
  residual, `BANK` less the sub-phases.
- **The graph's `EXIT` phase is always zero.** `Probe::finish` enters `EXIT` and takes no later
  probe, so the tail after the last unit is charged to that unit's phase (here `output`). This
  predates this change and was left as is.
- **Probe cost and the net column.** A probed chain takes eight clock reads per run, and the
  probes add about 2 µs per block, about 22 % of the unprobed block. The raw column is what the
  probes charged. The net column subtracts one in-situ probe cost per entry. It is an estimate, and
  small net values are within its error.
  - Example: `scatter` is empty by construction on this row, because the resident fold takes the
    block before any transpose.
  - Its raw 8 entries x about 30 ns are almost all probe.
- **Harness runs.** During development, earlier versions of the harness were run twice with
  `taskset` but without waiting for low load, only to check the output format. They are not
  recorded. The table above is the single run of the committed harness, pinned with
  `taskset -c 31`. It was set to start once the 1-minute load average fell below 2.0, or after 40
  minutes, whichever came first; the load threshold triggered it.
- **Not the console benchmark.** The numbers are descriptive, from one host, one run, and gate
  nothing.
