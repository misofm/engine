# #966 adversarial verification: the fix brief

**Tree.** Detached worktree of `codex/batch-plumbing-floor-2` at `caa055fc`. `prototype.patch` and
`evidence-harness.patch` both apply cleanly there; the study's `c04027b0` differs only in test-support
profiling code (`crates/graph/src/lib.rs`, `crates/rack`). Scratch additions (never for commit):
`crates/graph-compiler/tests/zz_verify_966.rs`, a wasm host test `zz_verify_966_host.rs`, and extra
exports in the harness guest. Copies of these, the mutation variants and the raw logs are in
`scratchpad/verify-966-evidence/`. Everything ran with `CARGO_INCREMENTAL=0`. Nothing was timed.

**Verdict: implement with amendments.** The ten-line binder check is correct, minimal and on the
right layer. The amendments below are test and brief changes. None of them changes the code fix.

---

## Findings, by severity

### 1. Medium, adjacent and pre-existing: the committed probe never arms the mono collapse, which the host does, and #970 bites there

`host-core::prepare` arms the collapse on every mono-mapped track (`crates/host-core/src/prepare.rs:1641-1646`).
`bank_levels.rs` never calls `arm_mono_collapse`, so it checks a narrower plan than production binds.
With the collapse armed exactly as the host arms it (my `zz_verify_966.rs`, `ZZV_FLAGS=36`: no meters,
no controls), the result diverges from `Scalar`:

- Plans #966 does not touch (no misaligned slot, bound today): 13 of 183 seeds (`20000..20300`),
  native, at `Simd8` and/or `Simd4`.
  - Seed 20194 on the **base-tree binary** differs in 2,086 of 4,096 samples at `Simd4` and at `Simd8`.
  - With the collapse unarmed, it is identical.
- Plans the fix rescues: 11 of 117 seeds, a similar rate.
- The intended 64-track fixture with every track mono-mapped and its own asymmetric input/EQ/comp:
  - at x86 `Simd4`, 3,124 of 4,096 samples differ;
  - with meters at the internal taps, `Simd8` and wasm `Simd4` differ too.

**Mechanism.** In the intended fixture with meters, the limiter chains (units 80-87) report every lane
eligible. Their upstream input builtins, EQ and compressor are asymmetric but sit in other chains.
That is exactly open issue **#970** ("arm the mono collapse only on chains that gather the track
input"). It is not caused by #966.

**Consequence.** Rescued ragged sessions load into the same #970 condition. For a mono-mapped track
with an asymmetric strip, "refused" becomes "loads with a wrong right channel" until #970 lands.

**Amendment.**
- State in the brief and in `bank_levels.rs` that the probe deliberately leaves the collapse unarmed,
  because of #970.
- Tell the implementer not to chase collapse-armed mismatches in #966.
- Flag the #966/#970 ordering to the owner. The recommendation is #970 first or together.
- After #970 lands, a host-armed leg of the probe is a follow-up.

### 2. Low-medium: M7 (`continue` becomes `break`) passes all four gates

With M7, every slot after the first misaligned slot of a group is dropped, including slots where the
lanes realign. All four committed tests pass (`v966-mut-M7.log`). In every reproducer, the slots after
the misaligned one are misaligned too.

**Amendment.** Add an over-reach guard for realignment and record M7 in `MUTATIONS.md`. The guard is
16 intended-strip tracks with these `simd1` programs:

- `ch00..ch07`: `[gate, eq, comp, transient, soft-clip]`;
- `ch08..ch15` alternating between:
  - A: `[gate, comp, transient, soft-clip]`;
  - B: `[gate, eq, comp, soft-clip]`.

In the A/B group:

- `gate` binds;
- `eq` and `transient` are skipped;
- `comp` is misaligned (A at rank 1, B at rank 2);
- `soft-clip` realigns at rank 3 and must bind.

Pins:

- Native `Simd8`: 9 banks. M7 gives 8 and M2 gives 8. The base tree refuses it.
- Wasm `Simd4`: 18 banks.
- Renders match `Scalar` bit for bit, native and wasm.

This is the one shape where the runtime runs a bound slot after a per-node misaligned one. It works
because `cohort_runs` merges only on proven dataflow (`crates/graph/src/runtime.rs:7200`).

### 3. Low: every reproducer puts the ragged lane at lane 0

**Mutation M8.** Skip lane 1 in the comparison. Only one probe line catches it: seed 58 at x86 `Simd4`.

**Amendment.** Parametrise gate 1 over `ch00`, `ch60` (middle lane) and `ch63` (last lane):

- all three bind 21 banks at native `Simd8` and 45 at wasm `Simd4`;
- all three are bit-identical to `Scalar`.

### 4. Low: the committed tests bind no console facilities

The committed tests bind no meters, no controls and no observation. I checked the rescued plans with
all of these on:

- meters at all seven taps, under `Concurrent` delivery and under `BetweenCalls` delivery with
  `MeterMetricSet::ALL` (the banked meter passes, #943 and #950);
- live control channels with bypass toggles at blocks 5 and 11;
- every observation tap armed.

With the collapse off, every width matches `Scalar` in PCM, every meter snapshot and every
observation window:

- 10 models, native: `ch00`/`ch60`/`ch63` less EQ, nine EQs, three EQs, ragged soft-clip, both
  realign sizes, and the mono fixture less `ch00`/`ch60` EQ with the collapse engaged `[356, 22]`;
- the same 10 models in wasm;
- 148 native rescued seeds;
- 531 wasm rescued seeds (`20000..21500`).

**Optional amendment.** One committed facility-armed reproducer, which costs about 5 s in debug.

### 5. Low, process: gate 5 sends the implementer to the scratch harness

`evidence-harness.patch` also edits three files that must never be committed:

- `crates/graph/src/lib.rs`: an `X966_RELAX` environment read inside `has_valid_structural_layout`;
- the root `Cargo.toml`;
- `Cargo.lock`.

**Amendment.** Apply only `tools/console-workload/tests/zz_probe_966.rs`, or revert those three
files, and commit nothing from the harness.

### 6. Low: anchor drift

The anchors are at `c04027b0`. At the branch head `caa055fc`:

| Anchor | At `c04027b0` | At `caa055fc` |
|---|---|---|
| `has_valid_structural_layout` | `graph/src/lib.rs:1158` | `:1179` |
| Level clause | `:1235` | `:1256` |
| Refusal | `:1712` | `:1734` and `:1737` |

The other anchors hold.

### 7. Low: the release-flake advice is wrong

"Rerun" failed three times in a row in a fresh target directory, after a `wasm-gates` release build
had shared that directory.

**Amendment.** Use debug or a dedicated `CARGO_TARGET_DIR`.

### 8. Info: the study's wasm "PCM moved 0" was mostly silence

The study ran `ZZ966_BLOCKS`, which defaults to 2. I re-ran at 16 blocks: 351 of 351 rescued seeds
were identical, and one seed (20780) is silent even at 16 blocks.

**Amendment.** Scope "every plan compile accepts binds" to the effect-bank `graph.scheduler.layout`
refusal.

---

## Confirmed claims

- **Root cause.**
  - Every base refusal is exactly one or more cross-level effect banks:
    - native: 556 of 556 lines, seeds `40000..41500`;
    - wasm: 342 of 342, seeds `40000..41000`, which is 34% against the study's 32%.
  - The relax table reproduces exactly:
    - `level` relaxed: `ch00` refused, `ch63` binds with 0 samples differing;
    - `both` relaxed: `ch00` binds with 2,100 of 3,072 samples differing.
  - No other source of cross-level banks exists:
    - chains are bucketed by first-slot level, and the path arithmetic is asserted (`banks.rs:148-204`);
    - sidechained chains are never candidates;
    - builtin banks are refused at compile attach (`graph/src/lib.rs:1384-1388`);
    - PDC and track delays are not graph nodes;
    - the dynamic rack goes through the same binder;
    - mono and stereo classes only split pools.
  - Equal level and equal rank are the same condition.
- **Fix.**
  - M1: all 4 tests fail, 28 lines over 22 seeds (15 at `Simd4`, 13 at `Simd8`).
  - M2: only gate 3 fails (19 ≠ 20).
  - M3 (first versus last member only) is caught.
  - Wasm reproducers: the base refuses 3 of 3; the fix binds 45, 6 and 43 banks.
  - `prepare_host_session` on the reproducer returns OK.
- **Class A.**
  - Fresh seeds `40000..41500`: 3,944 of 3,944 bound lines are byte-identical. The compared fields
    are:
    - SHA and levels;
    - effect and builtin bank members;
    - estimate and unit census;
    - shape, transposes, folds and redirects;
    - the PCM hash.
  - SHA and levels are unchanged on all 4,500 lines.
  - All 99 console rows are identical.
  - `graph_fixture --check` passes, and 10 of 10 determinism runs match.
- **Hygiene.**
  - clippy with `-D warnings` and `--all-features` (lib plus `bank_levels`) is clean, and rustfmt is
    clean.
  - The `graph-compiler` and `rack-compiler` suites pass.
  - The probe takes 54 s in debug. `scale.rs` already takes 60 s, and `test-debug-a` runs about
    4-5 of its 15 minutes.
  - The `Simd4` pins run in no CI job; I verified them only in the wasm guest.
- **Layer.** (a) is right:
  - (a′) moves lane membership in plans that bind today, for example x86 `Simd4`, or a misaligned
    delay slot;
  - (b) changes graph identity;
  - (c) is unsound.
