# 262 DAWproject architecture comparison: keep interchange at the adapter boundary

One-line summary: DAWproject is useful vendor-neutral exchange data, not standalone runnable state;
keep V2's canonical runtime session and content identities, and consider only a hostile-input,
loss-reporting adapter after the fan-delivery foundation lands.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #262.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- DAWproject: [`ee4dcdde75940f30e14e55401a26955a58b8322b`](https://github.com/bitwig/dawproject/tree/ee4dcdde75940f30e14e55401a26955a58b8322b), inspected 2026-08-29.
- No competitor execution, benchmark or legacy Miso inspection occurred.

## Findings

DAWproject 1.0 explicitly describes a stable vendor-neutral exchange format rather than a native DAW
format ([README](https://github.com/bitwig/dawproject/blob/ee4dcdde75940f30e14e55401a26955a58b8322b/README.md#L27-L52)).
It is a ZIP containing project/metadata XML, media and embedded plugin state
([README](https://github.com/bitwig/dawproject/blob/ee4dcdde75940f30e14e55401a26955a58b8322b/README.md#L31-L71)).
It represents tracks, routing, timelines, automation, plugins and generic devices, but media identity
is path-based. References may be archive-relative, external relative or absolute
([FileReference.java](https://github.com/bitwig/dawproject/blob/ee4dcdde75940f30e14e55401a26955a58b8322b/src/main/java/org/dawproject/file/FileReference.java#L5-L22)).
Audio declarations omit a canonical digest, frame count and bit depth. Plugin state is embedded while
the executable is only identified by descriptive format/vendor/ID/version metadata
([Device.java](https://github.com/bitwig/dawproject/blob/ee4dcdde75940f30e14e55401a26955a58b8322b/src/main/java/org/dawproject/device/Device.java#L18-L72)).
Generic device parameters do not define an algorithm capable of preserving PCM bits.

The public reference loader unmarshals project XML without its schema-validation path and can read a
whole ZIP entry with `readAllBytes`. No visible archive count, total expansion, content-hash or
external-path-containment policy was found
([DawProject.java](https://github.com/bitwig/dawproject/blob/ee4dcdde75940f30e14e55401a26955a58b8322b/src/main/java/org/dawproject/file/DawProject.java#L171-L176)).
That reference code is not suitable as an untrusted fan-upload boundary.

A recipient therefore gets rich exchange semantics, not renderer closure: an importer, compatible
plugins, mapping decisions and media resolution are still required. V2's strict canonical session,
fixed algorithms, exact source/effect identity and runtime remain the authoritative fan format.

## Decision

- **Adopt only as a future adapter:** import/export behind an explicit deterministic capability/loss
  report. Generic EQ/compressor/gate/limiter recognition may support best-effort mapping, never an
  exact-render claim without algorithm/parameter equivalence fixtures.
- **Preserve:** V2's runtime TOML, unknown-field rejection, content-addressed media, typed refusal,
  effect availability and separate native/browser implementations.
- **Reject:** DAWproject ZIP as the V2 delivery architecture, embedded stems/plugin blobs, external
  path locators, XML extension tolerance in the runtime schema and generic-device substitution as
  bit-equivalent execution.
- **Existing ownership:** #241 owns canonical-PCM source identity/schema; #244 owns verified shared
  storage/deduplication; #245 owns deterministic lossless delivery. A DAWproject adapter must consume
  them, not recreate them. Importing `FileReference` into session locators would regress #241.

## Gates for a future adapter

1. Accept a pinned format version and emit a deterministic capability/loss report before V2 output.
2. Cap XML/archive bytes, entries, expanded bytes and nesting; disable entities/DTD; reject duplicate
   names, unsafe paths and untrusted external references.
3. Stream media through #241/#244 identity and storage; never materialize whole entries.
4. Identical input yields byte-identical canonical V2 TOML; identical PCM across archives creates one
   store object.
5. Unsupported clips/warps/instruments/devices yield typed flatten-required/unsupported records,
   never omission. Exact device mapping requires a ruled algorithm/parameter fixture.

## Limitation

DAWproject deliberately optimizes interchange breadth, not deterministic execution. That is not a
defect in its mission, and it is why it should remain outside V2's runtime contract.

