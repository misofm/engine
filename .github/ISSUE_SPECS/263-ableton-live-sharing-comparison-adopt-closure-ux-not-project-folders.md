# 263 Ableton Live sharing comparison: adopt closure UX, not copied-project identity

One-line summary: Ableton's Collect All and Save provides a clear dependency-closure workflow, but a
collected project remains tied to compatible Live, Packs, plugins and licenses; adopt the one-action
preflight experience over V2 hashes/shared storage, not per-project media copying.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #263.** This local file mirrors its official-document comparison.

## Authority and evidence limit

Official primary documentation retrieved 2026-08-29:

- [Collect All and Save](https://help.ableton.com/hc/en-us/articles/209775645-Collect-All-and-Save)
- [Transferring Projects to another computer](https://help.ableton.com/hc/en-us/articles/209071909-Transferring-Projects-to-another-computer)
- [Live 12: Managing Files and Sets](https://www.ableton.com/en/manual/managing-files-and-sets/)

Ableton Live and `.als` serialization are proprietary. No source-architecture claim or
reverse-engineered assumption is made. Engine comparison uses
[`90c3b9a`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).

## Findings

Ordinary Save retains references; Collect All and Save copies audio/video and Max for Live devices
into a Project folder. Third-party plugins cannot be collected and must exist separately. Transfer
requires compatible Live editions, Packs, Max, plugins and licenses; incompatible processing may
need freeze/bounce. Related Sets in one Project can share one copied sample, but official docs do not
claim content-addressed deduplication across Projects.

Ableton's useful architectural outcomes are dependency preflight and disk streaming: large
uncompressed media need not be retained in memory. V2 already has the stronger runtime form—small
canonical documents, cross-session content identity and bounded source rings. A copied Project is
runnable only inside a compatible authorized Live environment; freeze/bounce provides audio
portability by sacrificing editability.

## Decision

- **Adopt at product/adapter level:** one “prepare for fan sharing” action which inventories every
  source/effect dependency, resolves only missing hashes and refuses green completion until closure.
- **Adopt conditionally:** artist-authorized flattening for effects that cannot be represented or
  distributed. Ingest the result as canonical PCM and visibly mark it non-editable.
- **Preserve:** V2's semantic document, shared content store, JIT source rings and engine-owned native
  effects.
- **Reject:** media copies per shared session, normal path-relink repair, presets as executable plugin
  closure and installed editions/licenses as portable state.
- **Existing ownership:** #244's hard gate, progress, store, LRU/pins and A/B mix fetch-count test are
  a stronger hash-based Collect All; #245 owns decode; no duplicate core collection issue is needed.

## Gates for downstream sharing UX

1. Mix A cold with eight stems fetches eight; mix B sharing six fetches two; repeated pinned A/B
   switching fetches zero—reuse #244's gate.
2. Sharing completion follows full source/effect closure; no acknowledgement may precede a later
   missing dependency or drop.
3. Unsupported effects produce typed refusal or explicit flatten, never silent bypass.
4. A flattened source renders without the plugin and is marked non-editable; session size remains
   independent of source duration and inside the web document cap.
5. Warm playback reads verified content through bounded windows with no whole-stem allocation.

## Limitation

Official behavior establishes user workflow and portability constraints, not Ableton's private
serializer, DSP graph or numeric semantics. The lesson is UX over already-owned V2 primitives.

