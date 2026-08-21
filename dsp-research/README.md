# DSP research corpus

This directory is the implementation evidence base for Engine V2. It documents decisions before production DSP is written; it is not a collection of claims that an algorithm sounds good. Each production effect issue links to the applicable notes and records any implementation-specific deviation in its decision record.

## Contents

- `BIBLIOGRAPHY.md` is the bounded source list and stable citation keys.
- `CITATION_POLICY.md` defines what counts as evidence and how it is cited.
- `NOTE_TEMPLATE.md` contains the mandatory headings for new topic notes.
- The ten `*.md` topic notes live directly in this directory.
- `../fixtures/conformance/` contains the deterministic `.mepcm` fixture corpus.
- `listening/TEMPLATE.md` records blinded listening evidence without treating it as a substitute for measurement.
- `listening/FORMAT_EXAMPLE.md` is a fully populated synthetic A/A record with no human or sound-quality claim.

## How to use this corpus

1. Select the topic note(s) that apply to an issue.
2. Copy the required headings from `NOTE_TEMPLATE.md` without renaming them.
3. Cite stable bibliography keys and concise paraphrases; record a primary or official source wherever one exists.
4. Add deterministic fixtures and independent-reference tests before accepting a production kernel.
5. Attach generated fixture manifests, test logs, benchmark JSON, and completed listening records to the implementing issue or release evidence.

## Normative engineering baseline

Realtime code receives finite planar `f32` blocks and must have prepared, bounded state. It cannot allocate, lock, perform I/O, log, mutate graph structure, or make data-dependent unbounded calls. Numerical validation uses an independent test-only `f64` model that imports no production kernel. Scalar repetition for the same input/state/event sequence must be byte-identical; cross-backend comparisons use the declared tolerance for that kernel.

The corpus launch gates apply exactly to 44,100, 48,000, 88,200, and 96,000 Hz. The 176,400,
192,000, 352,800, and 384,000 Hz observations are preserved extended compatibility/research
evidence only and do not establish engine, host, effect, or release support. Source-rate conversion
is out of scope unless a later issue explicitly adds it. Audio remains dual-mono unless an explicit
detector-link mode or smoothed 2x2 matrix says otherwise.

## Evidence status

These notes specify methods, limits, and gates. Issue 002 also supplies parser/reference fixtures and a
descriptive harness benchmark; neither is evidence for an effect's sound quality. Real listening results
remain the responsibility of the effect or release issue that owns the audible candidate.
