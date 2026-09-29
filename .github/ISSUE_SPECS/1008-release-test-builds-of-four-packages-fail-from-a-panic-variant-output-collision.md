# Release test builds of four packages fail from a panic-variant output collision

Found by the #1001 implementer and reproduced by its Sol verification ([#1001's spec](https://github.com/misofm/engine/blob/80c4119b9e6814cb450e87568243d6df9b6be7bc/.github/ISSUE_SPECS/1001-keep-the-unmoved-plan-when-the-whole-mono-cohort-re-plan-fails-to-bind.md), "Release-test clobber"). Tooling only; no engine behaviour changes.


## Problem

`[profile.release] panic = "abort"` plus Cargo's unwind-only test harnesses mean
that one `cargo test --release` invocation can build two panic variants of a lib. `effect-package`
(`rlib`+`cdylib`), `capi` (`rlib`+`staticlib`+`cdylib`) and `host-web` (`rlib`+`cdylib`) get
un-hashed output names, so the two variants write the same `target/release/deps/lib*.{rlib,so}`.
Cargo warns `output filename collision … (rust-lang/cargo#6313)`, and a dependent then fails.

## Reproduced

Reproduced on batch head `53024efc`, toolchain 1.97.1, in a fresh target each time:

* `cargo test --locked --release -p graph-compiler`: ``error[E0463]: can't find crate for
  `effect_compiler` `` in the lib test and the `graph_fixture` test. A second fresh run failed in
  the abort `graph_fixture` bin instead, with E0463 for `graph_compiler` and `effect_compiler`.
* `-p session-validator`: ``error[E0460]: found possibly newer version of crate
  `effect_package` ``.
* `-p native-pcm-runner` (collides on `capi` and `effect-package`) and `-p parameter-metadata`
  (collides on `effect-package` and `host-web`): E0463 inside `host-core`.

## Scope

A `--unit-graph` check of `cargo test --release -p <pkg>` for all 45 workspace
packages finds exactly these four with both panic variants of a cdylib or staticlib lib. The
other 41 have one variant.

## Doc contradiction

`docs/REALTIME_DEPENDENCY_POLICY.md:280` says per-package invocations
"never put two panic variants of a clobbering lib unit in the same run". That is false for these
four packages.

## CI

No release `cargo test` command in `qualification.yml` hits it. That covers
`test-release`'s lane/math/wasm-gates, `m3_determinism`, the loom leg and m1/f1, and
`audit-native`'s `-p audit -p bench -p console-workload`. Nightly's four `--ignored` release
tests do not hit it either: each builds one variant. So CI is green, but nothing runs these four
packages' tests in release. A later separate invocation heals itself: Cargo marks the unit dirty
("the profile configuration changed"), which I checked on a toy workspace. The failure is
therefore confined to one invocation.

## Workaround

`CARGO_PROFILE_RELEASE_PANIC=unwind`.

## Objective gates

Either the four packages' `cargo test --locked --release --no-run` builds
in a fresh target, or the policy doc names them and the supported invocation, and
`check-release-shape.py` pins that set. The owner's deferred `dist`-profile decision is the
structural option.
