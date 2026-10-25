# Build the expected-resources check's own module with the shipped builder

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #1334, its "Stated limits" (branch
`codex/batch-b13-1334` at `6eefa8d29`). Depends on #1334 merging first.

## Problem
`scripts/check-browser-expected-resources.py` without `--artifacts` builds its own module with its own cargo
line, on the stable pin. After #1334 the shipped module is built on the nightly channel with `-Zbuild-std`
by `scripts/build-web-audioworklet.sh`, so a local run without `--artifacts` checks a module that is not the
shipped one. CI always passes `--artifacts`, so CI is not affected.

## Evidence (`6eefa8d29`)
- `scripts/check-browser-expected-resources.py:453-480`: `build_module` runs `cargo build --locked --release ...`
  itself.
- `scripts/build-web-audioworklet.sh:25`, `:43-45`: `--module-only`, the one home of the shipped cargo line.
- #1334 spec, "Stated limits": "check-browser-expected-resources.py without --artifacts builds its own module on stable".

## Smallest slice
Make `build_module` call `bash scripts/build-web-audioworklet.sh --module-only <dir>` and read the module from
there; delete its own cargo line. No change to any expected digest or budget.

## Gates
- `python3 -B scripts/check-browser-expected-resources.py` (no `--artifacts`) passes and reads a module whose
  digest equals the one `build-web-audioworklet.sh --module-only` builds.
- Its `--self-test` passes; `lint-checks.sh --self-tests` (the artifact-evidence-leak check finds one shipped
  cargo invocation, in the builder).

## Paths
`scripts/check-browser-expected-resources.py`, this spec.
