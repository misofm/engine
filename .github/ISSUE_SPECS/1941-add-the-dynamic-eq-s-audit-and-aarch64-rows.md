# Add the dynamic EQ's audit and AArch64 rows

Filed by root as #17's slice S3 (backlog sprint, 2026-10-24). The web kernel rows land with #17 S2 (the #1889 table enforces them).

## Smallest closable slice

1. An `audit dynamic-eq` subcommand row and its CI and AArch64 rows, as the shipped effects have (see #1920 for the de-esser's).
2. Console eligibility (decision 12's list) only if the owner says yes; the question is on root's owner list (launch: insert only).

## Paths

`tools/audit/src/**`, `.github/workflows/qualification.yml` (rows only), this spec. After #17 S2 merges.
