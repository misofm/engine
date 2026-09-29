# Engine footprint: owner rulings of 2026-09-29

Follow-ups to `engine-footprint-2026-09-28.md`, recorded from the owner's answers on 2026-09-29.

## Decision 9: browser observation paths (#1064)

Decision 8 assumed the browser adapter's "legacy" observation path was unused. #1064's step 1
found the reverse: the SDK, the shipped AudioWorklet and the app boot only the ordinary ("legacy")
path, which carries the track and master meters, resident gain reduction and the session's
spectrum; the "protected" path (#825/#828, built for a live EQ analyzer with its own bounded
ingress) has only test callers. Ruling: **keep the ordinary path and remove the unadopted
protected path** (its boot exports, `PreparedObservationStorage::Protected`, the observation
ingress, the protected dispatch and its tests). #824 (protected one-shot) closes as not planned.
A live EQ analyzer, if it returns, is designed on the ordinary path in its own issue.

## Decision 10: NaN encodings in class-A identity (#1065)

x86 and AArch64 produce different NaN bit patterns (and wasm leaves them unspecified), so raw
NaN bits cannot be identical across targets without canonicalizing every NaN at a per-sample
cost. Ruling: **class-A identity treats every NaN as one value.** Class-A comparisons, digests
and differentials fold NaNs to one canonical word before comparing; the promise is identical bits
on every target except a NaN's sign and payload. The existing NaN-safety rules stand: finite input
must not produce NaN, and each effect's documented NaN behaviour still holds. The engine does not
canonicalize NaNs at render.
