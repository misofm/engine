<!--
Provenance: copied from misofm/engine-v2-old docs/research/09-console-domain-model.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Console domain model

The future vocabulary is deliberately precise. Tracks, audio groups, auxes, submixes, matrices, and buses carry audio. DCA/VCA groups are control relationships only. Every edge owns a source tap, destination, gain, on state, pan, tap point, mute policy, and latency; those properties belong to the route, not guessed defaults.

PFL/AFL route selected sources to a listen bus and differ in tap semantics. The distinction is informed by the [Yamaha cue guide](https://manual.yamaha.com/pa/mixers/dm3/rm/en-US/6296246283.html). DiGiCo’s SD/Quantum documentation is evidence for the operational vocabulary and workflow, not an implementation template ([manual](https://support.digico.biz/hc/en-gb/article_attachments/37137993915665)).

Scenes are sparse, scoped patches with explicit safes and timing; they are not full-session blind replacements. Talkback, cue, mute groups, sidechains, and routing require explicit graph and priority semantics before capability is enabled. V0.1 advertises these terms only as known vocabulary with capability false and `not_implemented` refusal; it must not freeze imaginary behavior.
