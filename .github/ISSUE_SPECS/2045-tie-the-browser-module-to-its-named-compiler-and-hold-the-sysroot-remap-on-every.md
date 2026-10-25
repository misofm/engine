# Tie the browser module to its named compiler and hold the sysroot remap on every PR

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #1334, its "Stated limits" (branch
`codex/batch-b13-1334` at `6eefa8d29`). Depends on #1334 merging first.

## Problem
#1334 builds the browser module on a pinned nightly (`[browser-artifact]` channel) with `-Zbuild-std` and a
sysroot remap. Two of its properties have no committed gate:
1. Nothing ties the shipped module bytes to the named compiler. A build that silently runs on another
   toolchain is caught only if its digest moves.
2. The sysroot remap (`--remap-path-prefix=$sysroot=/rustc-sysroot`) is held only by PR evidence. The
   `artifact-identity` twin shares the runner's `RUSTUP_HOME`, so a dropped remap stays green on a PR; it
   fails closed later (the next bump or `npm-publish.yml`'s rebuild refuses the bytes).

## Evidence (`6eefa8d29`)
- `scripts/build-web-audioworklet.sh:49`, `:69`: the channel read from `rust-toolchain.toml`;
  `:152-174`: the three remaps, `:167` the sysroot.
- `.github/workflows/qualification.yml:162-190`: the `artifact-identity` twin (another checkout path and
  another `CARGO_HOME`, the same `RUSTUP_HOME`).
- #1334 spec, "Stated limits": "The sysroot remap has no committed gate" and its gate 4 (PR evidence only).

## Smallest slice
In the artifact job: (a) read the module's `producers` custom section (`processed-by` rustc version) and
compare it with `RUSTUP_TOOLCHAIN=<channel> rustc --version`; refuse on a mismatch. (b) Build the
`artifact-identity` twin from a toolchain copy at another path (a second `RUSTUP_HOME`), so its digest equals
the shipped one only if the remap holds. Both are CI steps plus a hermetic self-test of the producers reader.

## Gates
- The producers check's self-test: red on a module whose producers section names another rustc, and on a
  module with no producers section.
- PR evidence: with the sysroot remap dropped, the twin digest differs and the job fails.
- `lint-checks.sh --self-tests`, `check-ci-path-routing.py`, `test-ci-path-routing.py`.

## Paths
`.github/workflows/qualification.yml` (artifact and artifact-identity jobs), `scripts/build-web-audioworklet.sh`
(only if the reader needs a mode), a new script under `scripts/` for the producers check, this spec.

## Decision and evidence record (attempt 1, 2026-10-25)
- Scope note wins: `check-wasm-producers.py MODULE --toolchain CHANNEL` runs `rustup run CHANNEL
  rustc --version` itself and compares the whole version string (release, hash, date) with the one
  `processed-by` `rustc` entry; it loads `strip-wasm-names.py`'s walker by path (one parser).
  CI passes the build step's `toolchain` output. The step sits between build and upload.
- The twin installs the channel into a fresh `RUSTUP_HOME="$RUNNER_TEMP/twin-rustup"` (not a copy).
- `scripts/check-ci-path-routing.py` pins the identity twin step exactly, so its pin moved with it;
  it now also pins the producers step (between build and upload). `test-ci-path-routing.py` gains
  mutations for both. These two files are outside the scope note's paths: the pin forces them.
- Local evidence (one machine, two RUSTUP_HOMEs): fresh install 8 s, 676 MB. Twin with another
  checkout path, CARGO_HOME and RUSTUP_HOME: `478ca94c...` = the shipped module. With the `:169`
  sysroot remap dropped, only RUSTUP_HOME differing: `bc5d3eeb...` vs `87ed775a...` (red).
- Real module: `rustc 1.100.0-nightly (f7d782a3b 2026-08-19)` passes against `nightly-2026-08-20`
  and fails against `nightly` (1.101.0-nightly).
