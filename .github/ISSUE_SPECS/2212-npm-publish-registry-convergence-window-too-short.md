# npm-publish: the publish job's registry convergence window is too short (red on every release)

Filed by root on 2026-10-30 (#2212).

## Problem

The `publish` job of `.github/workflows/npm-publish.yml` fails its step "Verify registry version,
integrity, public access, and latest converge together" on every recent release, although the
package was published correctly. The step polled 12 times with 5 s between attempts (about one
minute). The npm registry takes longer than that to serve a new version's metadata, so a correct
publish reports red and needs a manual `verify` run every time.

## Scope

- Lengthen the publish job's convergence window for normal registry propagation, with margin.
- Keep the exact-integrity, public-access and `latest` checks, the "never publish again" rule, and
  `verify` as the recheck for a truly ambiguous publish.
- Keep a sane poll interval (no tight loop) and stay within the job's timeout (the job sets no
  `timeout-minutes`, so the GitHub default of 360 minutes applies).
- Update `scripts/test-npm-publish-modes.py` where it pins the attempt count.
- No Rust, SDK, release-pin or version-literal change.

## Gate

The next release's `publish` run is green without a `verify` rerun, or the workflow self-test covers
the longer window.

## Evidence and decision record (attempt 1, 2026-10-30)

### Measured lag

Times are from the job logs (`gh api repos/misofm/engine/actions/jobs/<job>/logs`) and from the
registry's own per-version `time` field (`npm view @misofm/engine time --json`). "Publish returned"
is the `+ @misofm/engine@<v>` line of the real (non-dry-run) `npm publish`. The lower bound is the
last failing poll of the publish job's window; the upper bound is the converging poll of the
following `verify` run.

| Version | Publish run / job | Publish returned (UTC) | Last 404 (lower bound) | Registry `time` | `verify` converged (upper bound) | Lag by registry `time` |
|---|---|---|---|---|---|---|
| 0.8.1  | 40493560614 / 122153225290 | 2026-10-27 11:06:35.6 | > 58.8 s | 11:10:48.2 | <= 280.5 s (40494175961) | 252.6 s |
| 0.9.0  | 40659565351 / 122718904812 | 2026-10-28 08:50:02.8 | > 58.7 s | 08:55:13.5 | <= 325.4 s (40660139867) | 310.8 s |
| 0.10.0 | 40808648580 / 123233497476 | 2026-10-29 02:50:29.3 | > 58.9 s | 02:53:37.7 | <= 248.1 s (40809069608) | 188.5 s |
| 0.10.1 | 40859255383 / 123397994379 | 2026-10-29 10:39:52.5 | > 58.5 s | 10:44:00.9 | <= 279.1 s (40859875956) | 248.5 s |
| 0.10.2 | 40994282477 / 123863670643 | 2026-10-30 02:54:08.7 | > 58.2 s | 02:55:46.9 | <= 160.3 s (40994530461) | 98.2 s |
| 0.10.3 | 41055803933 / 124062917586 | 2026-10-30 12:30:44.0 | > 58.9 s | 12:32:20.7 | <= 158.5 s (41056170959) | 96.8 s |

The registry `time` lies inside every bound, and the 0.9.0 `verify` run polled for 49 s before it
converged (step start 08:54:39.3, next step 08:55:28.1), which brackets the first-served moment
tightly around that `time`. The registry `time` is therefore a good estimate of when `npm view`
first served the version. Measured lag: 97 to 311 s (median about 220 s). Every failure was the
same `E404 No match found for version` on `npm view <pkg>@<v>`.

### Decision

- Window: 60 attempts, 15 s apart: 59 sleeps of 15 s = 885 s, plus the registry calls of each
  attempt (about 0.3 s for a 404 poll and 1.1 s for a converging one in the logs), so about 15 minutes. That is about 2.8 times the worst
  measured lag (311 s) and well inside the job's 360-minute default timeout. A 15 s interval keeps
  the load to four rounds of three registry calls per minute; the normal case still converges at
  the first poll after propagation.
- Unchanged: the exact `version`/`shasum`/`integrity`, `public` access and `latest` comparison; the
  failure message ("rerun only in verify mode after propagation, never publish again"); `verify`
  runs the same step with the same window, so it stays the recheck for a publish that does not
  converge in 15 minutes.
- `scripts/test-npm-publish-modes.py`: the normalized-workflow hash gate accepts exactly this window
  edit (mapped back to the baseline text, so any other edit still fails); the registry-mismatch
  fixture now expects 60 bounded attempts and 59 sleeps of exactly 15 s. Test value: a window that
  silently shrinks, or a poll loop that loses its sleep (a tight loop against the registry), turns
  it red; checked by a local mutation to `interval_seconds=1`, which failed with "registry polling
  did not wait 15 s between its 60 attempts".
