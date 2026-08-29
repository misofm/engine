# 264 Logic Pro sharing comparison: adopt hard asset closure, not platform packages

One-line summary: Logic's consolidate-before-transfer posture is useful, but its packages are native
state for compatible Apple environments rather than portable runtime closure; retain V2's shared
content store and make any flattening explicit.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #264.** This local file mirrors its official-document comparison.

## Authority and evidence limit

Official Apple documentation retrieved 2026-08-29:

- [Sharing overview](https://support.apple.com/guide/logicpro/sharing-overview-lgcp5a70f0fc/mac)
- [Save projects](https://support.apple.com/guide/logicpro/lgcpce128e82/mac)
- [Consolidate project assets](https://support.apple.com/guide/logicpro/lgcpce09b9d8/mac)
- [Share to Logic Pro for iPad](https://support.apple.com/guide/logicpro/lgcp1e18b8f7/mac)

Logic Pro's source, project serialization, integrity rules and media runtime are proprietary. No
unpublished architecture is inferred. Engine comparison uses
[`90c3b9a`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).

## Findings

Logic can save a package or folder and optionally copy external audio, sampler data, impulse
responses, video and Sound Library content into project media. Sharing guidance tells artists to make
all assets available and select all asset classes. The macOS package is platform-specific and should
be compressed for Internet/non-Apple transfer.

Plugin/channel-strip parameters remain project state, but absent Audio Units require freeze/bounce.
Mac/iPad and version compatibility are conditional; current-version saves may not reopen in older
Logic. A package is therefore native runnable state only inside a sufficiently compatible Logic and
asset/plugin environment. A bounced song is playable but is no longer the editable session.

V2's platform-neutral canonical TOML, exact engine/backend, launch effect set, closed launch-rate set,
content-addressed stems and native/browser digest gates are the appropriate portable closure.

## Decision

- **Adopt at product level:** verify/consolidate-before-transfer as a hard deterministic sharing
  preflight over V2 identities and effect availability.
- **Adopt conditionally:** explicit artist freeze/bounce with canonical-PCM identity and visible loss
  of editability.
- **Preserve:** V2's platform-neutral session, shared store, launch-rate refusal, effect identity and
  cross-target render evidence.
- **Reject:** macOS package semantics, per-session media duplication, optional external references,
  opaque version migration and saved parameters as executable closure.
- **Existing ownership:** #241/#244/#245 already provide the stronger verify-before-play identity,
  shared store and deterministic transport path. A Logic-shaped package would regress the requirement
  that many fan mixes share stems without re-download.

## Gates for downstream sharing UX

1. Session-open/share remains closed until all artifacts are decoded, verified and stored; missing
   content is typed failure, never silence.
2. Identical canonical input produces the same complete dependency inventory.
3. Missing effects require explicit flatten or refusal; flattened results render without the AU and
   remain visibly non-editable.
4. One content object serves the same stem across sessions; unsupported rate/features refuse without
   implicit conversion.
5. Native/browser boots of the accepted document and PCM retain existing exact digest parity.
6. Any uploaded archive is only a hostile transport envelope dismantled into canonical TOML and
   verified shared-store objects before boot—never a new runtime package format.

## Limitation

Official documentation supports workflow conclusions only. It does not reveal Logic's graph,
serialization or numeric implementation and cannot establish exact cross-platform replay.

