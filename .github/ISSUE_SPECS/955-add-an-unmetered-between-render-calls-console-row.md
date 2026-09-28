# Add an unmetered between-render-calls console row

## Product outcome

`sixty_four_track_console_metered` (#881) is the default web boot's shape: between-render-calls delivery, which fuses each cohort's fader and matrix into one stage (40 chain stages against the standing row's 48), plus a `SAMPLE_PEAK` meter on every track. The only unmetered row, `sixty_four_track_console`, uses concurrent delivery, so metered minus standing mixes the meters' cost with the fusion difference. Add the unmetered twin with the same delivery, so the meters' own cost, and the fused path's cost, can each be read directly. Recommended by the #881 attempt 2 verification.

## Smallest closable slice

One row, `sixty_four_track_console_fused` (name to taste): the metered row's plan and entry point with no meter selected. Same files as #881 (the row, `floor.rs` pin, record count everywhere, validator, self-test, preflight). Gates: its 64-block digest equals `sixty_four_track_console`'s; its stage count is pinned at 40; every existing row unchanged; preflight passes; no timed run by the implementer.

## Dependencies

#881.
