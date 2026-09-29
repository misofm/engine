# Hold document parse, the protocol session store, capi prepare and host-web boot above 65,535 tracks

Successor B of #1045 (Sol verdict, attempts 3-5). Engine rule: arbitrary track counts, never a compiled `MAX_TRACKS`. Nothing parses a session document of this size today (#1046 deleted `session/tests/descriptive_scale.rs`, which was ignored and never scheduled). A 65,537-track document is 93.7 MB and takes 19.3 s to parse in debug, so this is nightly work.

## Smallest closable slice

In a release-mode nightly gate, parse one 65,537-track document once and drive each entry point from that parse: the session parser, the protocol session store edits, capi prepare and host-web boot. Assert each accepts it (or refuses with the documented configured-resource diagnostic, never a hard cap).

## Gates

1. A planted track cap at each of the four entry points turns the nightly gate red; green when reverted.
2. The job's time and peak memory are stated and fit the nightly runner; routing and actionlint pass.
