# PCM feed: a live seek leaves a short gap while the shared ring holds the old generation

GitHub #2185. Found while building #1387 (2026-10-28).

## Observed

A live plain ring seek during playback can underrun a few quanta in both instance modes: worker
mode 0 to 512 frames per seek in the Chromium/WebKit smoke after #1387's headroom fix; single mode
about 896 frames on Chromium in the same smoke. A suspended seek (`prepareSeek`, refill, resume) is
already gap-free in both modes.

## Root cause

The writer seeks while the shared MSB1 ring is full of the old generation. It cannot publish
new-generation PCM until the drain frees those stale slots, and the drain applies the seek in the
same pass, so render pops the seek before any new chunk exists.

## Candidate fix (not decided)

Defer applying a live plain seek until its first new-generation chunk is published: the drain
frees the stale shared slots at once, the old generation keeps playing from the engine ring
meanwhile, and a bound covers a writer that never publishes. The fix applies to both consumers (the
SDK prelude and the control Worker), through the shared MSB1 case table.

## Gate (one test per claim)

One case in the shared MSB1 consumer case table: a live seek while the shared ring is full of the
old generation applies no seek before a new-generation chunk is published (or the bound expires).
