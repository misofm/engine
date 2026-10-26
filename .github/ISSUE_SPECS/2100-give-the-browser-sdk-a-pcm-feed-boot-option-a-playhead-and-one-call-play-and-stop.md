# Give the browser SDK a PCM-feed boot option, a playhead and one-call play and stop

Filed by root from the console sprint (2026-10-26). Source: the console C0 slice (misofm/studio#6) and the scope note `scratchpad/opt/scope/sdk-console-gaps.md`, items 1, 5, 6 and 9.

## Product outcome
A browser app boots the engine with a PCM feed, reads the playhead and plays or stops with one SDK call each. It needs no wrapper around `createHost` and no position derived from meters.

## Smallest slice
- `CreateEngineOptions.pcmFeed?: boolean | { moduleUrl?: string | URL }`. When it is set, `createEngine` awaits `prepareEngineFeed(context, moduleUrl)` after the context is verified and before `createHost` (sdk/src/browser/engine.ts ~709-722). The default is off, so this is not breaking.
- `engine.position(): Promise<{ frame, generation, atSample }>` from the host's `seekReport("$timeline")` and `nextAbsoluteSample`: frame = sourceFrame + (nextAbsoluteSample - firstSample). It goes beside `seek` (engine.ts ~1296).
- `engine.play()` and `engine.stop()` set the transport state and resume or suspend the context in one call. The README says that `setTransport` alone gates nothing (TRANSPORT_SET is a stored label, #1635).
- README: the feed order, the playhead, the transport, and one sentence that the master is an ordinary submix designated after boot (`submix(id).master(true)`, #1683), whose meter carries gain reduction only (#1690 D3).

## Gates (one eval each, sdk/test/browser-evals.mjs)
- With `pcmFeed: true`, a fake context records `addModule` order [feed prelude, engine worklet]. This catches the prelude loading second.
- After `seek(f)` and N fake quanta, `position().frame` is f + N*quantum. This catches a wrong anchor.
- `stop()` leaves the context suspended and the transport stopped. This catches the two halves drifting apart.

## Paths
sdk/src/browser/engine.ts, sdk/README.md, sdk/test/browser-evals.mjs, this spec.
