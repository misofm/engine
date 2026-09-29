# Remove the eight-lane wasm measurement build

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Approved (reverses the #183 ruling).

**Blocked on an owner ruling, because it reverses part of an earlier one.** Scoping study:
`docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 7, R7.

**The conflict.**

- The #183 ruling (`docs/rulings/wasm-simd8-null.md`, 2026-08-27) closed the W8-on-wasm switch as a
  null, but kept "the `--issue183` bench arm and the `miso_wasm_simd8` opt-in cfg … in the tree"
  so a future engine could re-measure.
- The 2026-09-28 direction points the other way:
  - "no target-specific code, beyond the architectural lane widths";
  - "modes production never needs are removed entirely, including test-only entry points".

The ruling to record: "The wasm build has one width, `Simd4`. The eight-lane measurement cfg and
its arms are removed. A future re-measurement re-adds a cfg in its own issue."

## Context

- **`crates/lane/src/backend.rs:8`, `:40-58`.** The `miso_wasm_simd8` arm moves `Backend::current()`
  on `wasm32` from `Simd4` to `Simd8`.
- **`crates/target-smoke/src/lib.rs:62-73`.** Its W8 assertion. `R1-…` may delete the crate first.
- **`Cargo.toml:105-108`.** The `check-cfg` entry for `miso_wasm_simd8`, and its comment.
- **The W8 leg of the wasmtime console benchmark** (`tools/wasm-console`, about 32 mentions in 823
  lines), its validator branch, and `scripts/test-wasm-console-benchmark.sh:214-265`, which CI lint
  runs.
- **The `--issue183` arms** of `scripts/operator/run-wasm-console-benchmark.sh` (`:46`, `:333`)
  and its preflight (`:40`, `:216`). They cannot run: `artifacts/issue183/` exists, and the
  scripts' repository root is wrong (audit, section 5).
- **Shipped module: unaffected.** No default build sets the cfg, and the module is built without
  it.
- **Related, and a separate choice.** `tools/wasm-gate-corpus` digests every case at three widths
  on every target (`WIDTHS = 3`, `src/lib.rs:69-72`), including `Simd8` on `wasm32`.
  - That is a determinism check of the generic lane code, not the measurement build. It uses
    `lane::Simd8` directly, not `Backend::current()`.
  - Keep it unless you want every wasm `Simd8` path gone. It costs a little of the wasm-guests job.

## Smallest closable slice

1. Delete the `miso_wasm_simd8` arm from `lane/src/backend.rs` and its module doc, the
   `target-smoke` assertion (unless `R1-…` has removed the crate), and the `check-cfg` entry and
   its comment.
2. Delete the W8 leg of `tools/wasm-console`, its validator branch and its mutation cases.
   `R9-…` deletes the whole tool; if it lands first, this step is moot.
3. Delete the `--issue183` arms from the operator runner and its preflight, unless `R9-…` deletes
   both scripts.
4. Append a supersession note to `docs/rulings/wasm-simd8-null.md` (the rulings README asks for
   this). Say the measurement hooks were removed on this date, and why. Keep the null result.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p lane`
     passes.
   - `RUSTFLAGS='-C target-feature=+simd128 --cfg miso_wasm_simd8' cargo check --target wasm32-unknown-unknown -p lane`
     now warns about an unexpected cfg and still selects `Simd4`. Record the output.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes.
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes.
   - `lane/src/backend.rs` loses lines, so panic line numbers in `lane` may shift. Prove with
     `wasm-objdump -d` that nothing else changes, and re-pin with that reason.
   - Or keep the line count by leaving a one-line comment where the arm was.
4. **CI routing.** `bash scripts/test-wasm-console-benchmark.sh` passes, if the tool still exists.
   `check-ci-path-routing.py` and `test-ci-path-routing.py` pass. The `verdict` table is unchanged.
5. **No live claim lost.** The only tests removed are the W8-measurement ones. List them from the
   `-- --list` diff (audit section 11).
   `wasm-gate-corpus`'s three-width determinism digests stay.

## Dependencies

The owner ruling. It is simpler after `R1-…` and `R9-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-remove-wasm-w8`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F11. The recommendation is sound; the context is incomplete.

1. **The kept re-measurement path is already dead.** `docs/rulings/wasm-simd8-null.md:36-43` kept
   the cfg and the `--issue183` arm so the paired capture could be re-run, and requires "a fresh
   paired record on the then-current base" to reopen. That capture cannot run today:
   `artifacts/issue183/` exists and the runner refuses to overwrite it, and the operator script's
   repository root is wrong (finding F2). Removing the hooks forecloses nothing that works.
2. **The cfg had a live test use on 2026-09-28.** Spec #976 (closed that day, lines 466-469) ran a
   `--cfg miso_wasm_simd8` leg of its bit-exact differential harness (15,000 scenarios, Simd8 in
   wasm). State that this ad-hoc leg is lost; a harness can still run Simd8 on wasm through
   `lane::Simd8` directly, as `wasm-gate-corpus` does.
3. **It is a lane-width measurement hook, not target-specific code** in the sense of the owner's
   rule; the ruling rests on "modes production never needs are removed", which does apply.
4. **Mobile scope: no effect.** The cfg is `wasm32`-only.

## Attempt 1 evidence

Terra, attempt 1, on `codex/1038-remove-eight-lane-wasm-build` (the root's branch name, not the
body's `codex/<issue>-remove-wasm-w8`), cut from the batch `codex/batch-slim-4` at `fcfb76b9`.
Implementation commit `281f07d6`. No timed benchmark was run.

**Ruling recorded.** The wasm build has one width, `Simd4`. The eight-lane measurement cfg
(`miso_wasm_simd8`) and its arms are removed. A future re-measurement re-adds a cfg in its own
issue. `docs/rulings/wasm-simd8-null.md` has a supersession note: what was removed, why, what is
lost (Amendment 2: #976's ad-hoc `--cfg` leg) and what is kept. `wasm-simd8-survey.md` has a
two-line pointer note, in the form #1027's note there used.

**Recount on `fcfb76b9`.**

| Slice step | State on this base | Done here |
|---|---|---|
| 1. `lane` arm and module doc; `target-smoke` W8 assertion; `check-cfg` entry and comment | all present (#1032 kept `target-smoke`) | removed |
| 2. `tools/wasm-console` W8 leg, validator branch, mutation cases | moot: #1039 deleted the tool and `test-wasm-console-benchmark.sh` | nothing |
| 3. `--issue183` operator arms and preflight | moot: #1039 deleted both scripts | nothing |
| 4. supersession note | the ruling already said #1039 retired the `--issue183` arm | note added |

**Not in the list, but the same class.** `wasm-gates` accepted `--expect-backend simd8`. Only the
wasm leg reads that argument, and only the measurement cfg could make a wasm guest report backend 2.
It is removed, as #1062 removed `--expect-backend scalar`. `ExpectedBackend` keeps one variant, and
`run-wasm-gates.sh` still passes `--expect-backend simd4`. The guest's `miso_gate_backend` mapping
(`8 => 2`) is width-generic and stays; only its doc line changed. `wasm-gate-corpus` is untouched,
so its three widths stay.

**Lines.** Code: +21/-55 in 6 files: `Cargo.toml` +1/-4, `lane/src/backend.rs` +4/-24,
`target-smoke` +4/-15, `wasm-gates` +6/-10, `wasm-gate-guest` +2/-2. Rulings: +31. No live
source outside the rulings, specs and handoffs names `miso_wasm_simd8`. `lane` now picks a width
only in `Backend::current()`. It still has ISA arms elsewhere, all older than this issue and out of
its scope: `wide_impl.rs` max/min operand order, `fpenv.rs` control words, `softfma.rs` and
`attest_host` on x86.

**Gates.**

| Gate | Result |
|---|---|
| 1. `cargo check --locked --workspace --all-targets --all-features` | exit 0 |
| 1. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| 1. `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p lane` (plus `target-smoke`) | exit 0 |
| 1. The same with `--cfg miso_wasm_simd8`, `-p lane -p target-smoke` | exit 0, **no warning** (below) |
| 2. `gain_pan_profile digests`, release, base and change | 17 digest lines byte-identical (sha256 of the lines `dd0e7194…`); only the harness's `finished in` time differs |
| 2. `bash scripts/run-wasm-gates.sh` (full: native, wasm `simd128`, V8 spill under Node 22.23.2) | ok. Both legs: 142 cases, 358 comparisons, 0 mismatches; wasm backend 1, native 2 |
| 3. `build-web-audioworklet.sh --module-only`, base and change, one machine | byte-identical (`cmp`): `9aca423b3e36ce6ab1c6d0dd21123d6eab81111ad0b3f47bdf5c8e09c613b2ee`, 3,415,176 bytes. No `wasm-objdump` diff and no re-pin needed; `backend.rs` has no panic site. The committed pin `6c952a2c…` differs from both local builds, as audit section 9 predicts; CI's `artifact-identity` job is the authority |
| 3. `check-scalar-oracle-absent.py --wasm` on the change module | ok |
| 4. `test-wasm-console-benchmark.sh` | moot: #1039 deleted it |
| 4. `check-ci-path-routing.py`, `test-ci-path-routing.py` | pass. No workflow edit, so the `verdict` table is unchanged. The router gives `route=full`, `self_tests=[]` for `fcfb76b9..HEAD` |
| 5. `cargo test --locked --workspace --all-targets --all-features -- --list`, base and change | **empty diff**: 2,212 tests in 201 binaries on both sides. The W8 assertion was a wasm-only `cfg` arm inside `smoke_values_are_canonical`, which stays, so no test name goes |
| `scripts/check-cross-targets.sh` | PASS: x86-64-v3; aarch64 iOS and Android product crates checked and linted; wasm `simd128`; armv7 and scalar wasm refused. The #1018 memset rows are expected failures |
| clippy `-D warnings`, `-p target-smoke -p lane --all-targets`, on `aarch64-apple-ios` and `aarch64-linux-android` | exit 0 |
| `cargo fmt --all -- --check`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | pass |
| `cargo test -p target-smoke`; release `cargo test -p lane -p math -p wasm-gates --features math/lane` | pass (111 passed, 16 ignored) |
| Every lint, docs-gates and gate-self-tests script, Python under `python3 -B` (58 entries, including `check-lane-policy.sh` and `test-lane-policy.sh`), and both sub-v3 refusal probes | all pass |

**Gate 1: a stale flag compiles silently, not with a warning.** The brief expected an
unexpected-cfg warning. `unexpected_cfgs` fires on a `cfg` a source condition names. Since no
source names `miso_wasm_simd8` any more, rustc has nothing to check, and a `--cfg` on the command
line alone raises nothing. Full output of the command:

```text
    Checking bytemuck v1.25.2
    Checking engine v0.1.0 (…/crates/engine)
    Checking wide v1.6.1
    Checking lane v0.1.0 (…/crates/lane)
    Checking target-smoke v0.1.0 (…/crates/target-smoke)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.39s
```

It still selects `Simd4`, by two measurements:

1. The release `wasm-gate-guest` built with `--cfg miso_wasm_simd8` is byte-identical to the one
   built without it (`5826c600…`).
2. Under `wasm_gates <guest> --expect-backend simd4`, it reports `"backend":1` with 358
   comparisons and 0 mismatches. `--expect-backend simd8` now exits 2 with `unknown backend
   'simd8'`.

**Pre-existing, not changed here.** Clippy on `wasm32` + `simd128` over `-p lane --all-targets`
fails with 5 errors in `crates/lane/tests/fp_env.rs` (`let_unit_value`, `unit_cmp`). The failure
is identical on `fcfb76b9`. CI does not lint `lane`'s tests for wasm; the `lane` lib and
`target-smoke --all-targets` are clean there. That belongs in a small follow-up, not in this issue.
