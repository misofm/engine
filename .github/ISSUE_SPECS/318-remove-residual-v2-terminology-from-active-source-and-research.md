# Remove residual V2 terminology from active source and research

## Owner ruling

No active prelaunch source, test, or research guidance may call the current engine or a current
contract V2. Use unversioned wording internally and V1 only where a boundary identity requires it.

## Smallest closable product slice

- remove stale V2 product prose from active DSP research and audit notes;
- rename generic migration test operands away from V1/V2/V3 decorative identifiers; and
- change rejected session/format fixtures from 2 to invalid sentinel 0.

Mathematical variables, external standard/product/ISA versions, licenses, URLs, and frozen
historical issue/evidence records are not product-version matches.

## Decision record

1. Research headings describe adopted engine decisions without a generation label.
2. Migration tests may exercise multiple abstract numeric revisions, but their internal Rust names
   describe source/intermediate/current roles rather than product generations.
3. Rejection tests use 0 or a maximum-value sentinel, never a hypothetical next product version.
4. Historical records are not rewritten when doing so would falsify their recorded context.

## Objective gates

- The active research checker and research corpus pass with unversioned decision headings.
- Session, session-validator, web worklet, audit, effect-runtime, and effect-compiler focused tests
  pass.
- A live-tree audit finds no V2 product terminology outside explicitly classified
  external/mathematical/historical records.

## Workflow

Sol briefs and approves this stateless scope. Terra performs the bounded cleanup. Sol reviews each
remaining match classification before PASS.

## Evidence record

- Sol brief approved; GitHub issue #318 matches this stateless local spec.
- Terra replaces the research corpus/template/checker heading with `Adopted decisions`, removes
  current-engine generation prose, and renames migration fixtures to source/intermediate/current
  roles. Session, validator, console, and worklet rejection fixtures now use invalid sentinel 0.
- The DSP research gate, session suite, eight session-validator tests, nine state-payload tests, six
  migration tests, 29 audit tests, hermetic worklet/mutation suite, formatting, workspace all-target
  check, and diff hygiene pass.
- The migration rename initially collided with the existing current-factory variable; separating
  `current_descriptor` from `current_factory` restored type clarity and is covered by the compiler
  and all six migration tests.
