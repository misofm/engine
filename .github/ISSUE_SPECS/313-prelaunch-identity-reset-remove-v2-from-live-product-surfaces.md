# Prelaunch identity reset: remove V2 from live product surfaces

## Owner ruling

The engine has not launched. No live product identity may claim to be V2. Internal implementation
names remain unversioned; where a schema, wire, artifact, or ABI identity structurally requires a
version, its first prelaunch identity is V1.

This ruling supersedes issue #215 only where that issue preserved a V2 spelling as sealed contract
identity. It does not turn mathematical names such as an SVF's `v2` integrator output, external
standards or dependency versions, or historical old/new migration operands into product versions.

## Smallest closable product slice

Reset every live Engine V2 identity to V1 in one transactional change:

- rename the native C header, exported `miso_engine_v2_*` symbols, C types, macros, consumers, and
  qualification pins to `miso_engine_v1_*` / `MISO_ENGINE_V1_*`;
- rename shipped `miso-engine-v2-*` browser and SDK artifacts to `miso-engine-v1-*`, including
  generators, manifests, provenance, checks, tests, and active documentation;
- reset live `.v2` browser qualification schema tags to `.v1`;
- replace active Engine V2 / boot-v2 product terminology with Engine V1 / boot v1, or unversioned
  wording where the version adds no information;
- amend `AGENTS.md` so prelaunch policy permits only unversioned internal names and V1 boundary
  identities;
- add a non-vacuous policy gate that rejects newly introduced live-product V2 spellings.

Historical issue specs, sealed measurement/listening evidence, and research archives are records of
what was asserted at the time. They are not live product surfaces and are not rewritten in this
slice. A follow-up inventory must classify their remaining V2 prose and decide whether removing it
would falsify evidence or merely clean stale terminology.

## Non-goals

- changing numeric ABI version `0x0001_0000` or Session V1 / protocol V1 values;
- renaming mathematical variables (`v0`, `v1`, `v2`, `v3`), Wasm `v128`, CIDv1, TOML 1.0,
  x86-64-v3, Git porcelain versions, dependency/action versions, or external product versions;
- deleting real effect-state migration coverage merely because its test operands are called old,
  current, V1, V2, or V3;
- rewriting Git history or renaming the GitHub repository in this slice.

## Decision record

1. Prelaunch contract stability does not justify publishing a second-generation identity.
2. Boundaries that already carry a version retain versioning but reset to V1.
3. Internal implementation names stay unversioned under issue #215's rule.
4. The native C ABI rename is atomic: no compatibility aliases retain `miso_engine_v2_*`.
5. Generated browser/SDK artifacts are regenerated or mechanically updated from their canonical
   producers; stale V2 filenames are removed rather than duplicated.
6. Historical records are preserved in this slice and explicitly inventoried afterward.

## Objective gates

- `git grep` over live trees (`AGENTS.md`, `crates/`, `hosts/`, `sdk/`, `tools/`, `sidecars/`,
  `scripts/`, `fuzz/`, and active `docs/`) finds no product V2 identity or terminology, with an
  explicit allowlist only for mathematical/external-version false positives.
- The C header compiles for C and C++, exports exactly the renamed V1 symbol set, and all existing
  ABI layout/linkage/lifecycle qualification remains green.
- Browser artifact build and reproducibility checks produce only `miso-engine-v1-*` artifacts.
- SDK generation, typecheck, headless tests, browser worklet tests, and qualification schema checks
  are green with V1 names/tags.
- `cargo check --workspace --all-targets` and proportional focused tests pass.
- The new policy gate proves itself with a red self-test mutation and rejects both textual Engine
  V2 branding and machine identities such as `miso_engine_v2_*`, `.v2`, and `miso-engine-v2-*`.

## Evidence record

Pending implementation and Sol review.
