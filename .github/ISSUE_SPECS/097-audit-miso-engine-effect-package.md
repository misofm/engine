# 097 Audit: miso-engine-effect-package

One-line summary: Audit the effect package crate: canonical CID bytes, resolution off the render thread, and the third-party Wasm boundary.

**Authority: GitHub issue #97 and its plan comment.** This file is a stateless pointer, not a
second copy of the brief. The issue body carries the findings with `path:line` evidence; the plan
comment on the issue carries the numbered steps, evals, acceptance checklist and hazards; and the
master plan (the first comment on issue #83) decides everything cross-cutting -- the numeric
contract (D1-D12), the `Lane` trait and its per-operation semantics, the block-kernel contract, the
`miso-engine-math` and `miso-engine-effect-runtime` boundaries, the fixture re-pin policy of §8, the
workstream waves of §9 and the evals of §10. Where this file and those comments disagree, they win
and this file is corrected in the same checkpoint.

Read, in order: `AGENTS.md`; issue #125 (standing instructions for the audit workstream); issue #83
body, master-plan comment and execution-plan comment; then `gh issue view 97` and its plan
comment.

Do not re-decide anything the master plan decides, do not loosen a gate, and do not pin a fixture
from production output: fixtures are regenerated only from an independent `f64` oracle or from the
scalar `Lane` instantiation, with the old-to-new deviation and the audit finding cited in the
commit message.

## Evidence — 2026-08-24, branch `audit-097-package`

Delivered on `origin/main` @ `3ead104`. Fixtures untouched: `git diff origin/main -- fixtures` is
empty, and both package vectors, the CIDv1 text and the descriptor wire are byte-identical
throughout. Production lines in the crate 4,801 -> 4,744 (counted to the first `#[cfg(test)]` per
file).

### Findings acted on

| finding | verdict | commit |
|---|---|---|
| F1 O(n^2) canonical order | fixed — bounded `[u16; 4096]` stack index, `sort_unstable_by((key, caller index))`, `maximum_artifacts` clamped like `HARD_DESCRIPTOR_CAP` | `perf(effect-package): canonical order via bounded stack index` + its gates |
| F2 dead `compile.rs` stub | fixed — `compile.rs`, `tests/session_validation.rs`, the `miso-engine-session` dependency and the dead `PackageError` deleted | `refactor(effect-package): delete the compile stub and session dependency` |
| F3 state verifier / placeholder id | **already resolved by #079**, no production change; negatives cited and the one missing property added as a test | `test(effect-package): the state envelope binds the identity it names` |
| F4 stale fuzz target | **already repaired upstream**; `fuzz/` builds under `cargo metadata --locked` and `cargo check --locked --bins`, and the package gate script now runs that check too | step 2 commit |
| F5 double descriptor verification in the C `inspect` entry | fixed — `VerifiedEffectDescriptorWireV1::identity()` hashes already-verified bytes; 16 -> 8 allocations at the boundary | `perf(effect-package): one descriptor pass in the C inspect entry` |

### Measurements (this host, release, 4,096 one-byte source artifacts, 366,227 package bytes)

| operation | before | after |
|---|---|---|
| `effect_package_v1_required_size` + `encode_effect_package_v1` at the cap | 6.93 s (4.38 s measured for the encode half alone under the M-04 mutation) | 3.17-3.44 ms |
| package-native allocations at n = 4,096 | one nested Issue-082 descriptor pass | unchanged: 8 allocations, 1,000 bytes, 0 live |
| `miso_engine_effect_descriptor_v1_inspect` allocations | 16 | 8 |

### Gates

* `encode_at_the_frozen_artifact_cap_has_one_nested_descriptor_pass_and_no_native_allocation`,
  `c_inspect_performs_exactly_one_nested_descriptor_pass` and
  `c_inspect_reports_a_wire_diagnostic_for_an_empty_null_wire` in
  `crates/miso-engine-effect-package/tests/package_allocation.rs`.
* `the_state_envelope_binds_the_effect_identity_it_names` in `tests/state_vectors.rs`.
* `descriptor_wire_diagnostic_codes_and_strings_are_frozen` in `src/diagnostic.rs`.
* `scripts/check-effect-package-v1.sh` additionally rejects `.sort(`/`.sort_by(`/`.sort_by_key(`/
  `.sort_by_cached_key(` in the package-native surface and `cargo check --locked`s `fuzz/`.
* Ten red mutations, all proven and reverted, in `crates/miso-engine-effect-package/tests/MUTATIONS.md`.

### Spec amendments

`.github/ISSUE_SPECS/078-…md` "Exact limits and resource behavior" now states the fixed stack index,
the total `(key, caller index)` comparator, the ban on allocating stable sorts and the
`maximum_artifacts` clamp, with an amendment record carrying the before/after measurement.

### Sweep

`cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
-- -D warnings`; `cargo test --locked --workspace` (219 suites green); `RUSTDOCFLAGS='-D warnings'
cargo doc --locked --workspace --no-deps`; `cargo metadata --locked` and `cargo check --locked
--bins` on `fuzz/`; `scripts/check-effect-package-v1.sh`; `scripts/check-effect-descriptor-v1.sh`
(C smoke); `scripts/run-wasm-gates.sh` (133 cases, 331 comparisons, 0 mismatches — this crate adds
no corpus cases); every other `scripts/check-*.sh` and `scripts/test-*.sh` that is green on
`origin/main`.

### Blocked / reported, not worked around

`fixtures/effect-interchange/v1/ACCEPTED.sha256` (Issue 081) pins the SHA-256 of the accepted
*source* files, including `crates/miso-engine-effect-package/src/{package,ffi,wire,lib,diagnostic}.rs`,
and the manifest's own hash is pinned inside `scripts/check-effect-interchange-qualification.sh`, so
it cannot be refreshed by design ("any accepted-baseline hash change is a STOP, not a fixture
refresh"). That gate is **already red on `origin/main`**: `crates/miso-engine-effect-compiler/src/
prepare.rs` drifted in `a28474d`. Any audit fix to an accepted source file necessarily adds to that
set; this job takes it from 1 mismatch to 6 and does not re-pin the manifest.
`scripts/test-capi-qualification-v1-policy.sh` is likewise red on `origin/main` (12 mismatches,
identical on this branch — untouched by this work). Both need an owner decision on when the audit
wave's accepted baselines are re-pinned.

### Deferred, with owners

| finding | deferred to |
|---|---|
| F6 seven verifier passes for diagnostic phase order (0.3 ms) | owner question 1; #081 qualification or a "package verifier single pass" successor — diagnostics must stay byte-identical |
| F7 descriptor semantic pass computes every error with heap collections | successor on `wire.rs`; **that owner must replace the F5 gate's oracle** — it asserts an allocation count and becomes `0 == 0` once `wire.rs` is allocation-free |
| F8 copy-pasted LE helpers and hex decoders | #104 (tools consolidation) / #105 (shared test support) |
| F9 `ptr.write` alignment in the safety contract | #081 (needs a Miri or misaligned-buffer gate) |
| F10 content hashed up to three times on the CID/select path | owner question 3; the select rehash is spec-mandated |
| F11 test/reference volume, JSON-driven descriptor variants | #081 |

Owner questions 1-4 from the issue body: 1 and 3 stay open (F6, F10). 2 is answered — the
"no heap in the package layer" rule is kept, and the 8 KiB stack index satisfies it at 1,700x the
speed. 4 is answered by #079: `state.rs` ships as V1 and now verifies the identity it binds.
