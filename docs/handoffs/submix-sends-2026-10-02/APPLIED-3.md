# APPLIED-3: how VERIFY-3's findings were applied

All edits are to `DESIGN.md` and `issues/*.md` in this plan directory. Nothing in the repository or on
GitHub was touched. Anchors that an edit relies on were re-read on `main` (`fe8ac679`) first.

## BLOCKER

| ID | Applied | File(s) |
|---|---|---|
| B1 | Applied. Slice 13 now has a Context bullet with the evidence, which was checked: `observation.ts:195-249`, the callers `boundary.ts:622/641` and `engine.ts:605/612`, and `observation_binding` `lib.rs:2462-2482`. It authorizes `sdk/src/core/observation.ts` and adds **D5. Observation on strips**, which passes the strip list `[...tracks, ...submixes]` at all four callers. Deliverables now cover D1-D5. New gate 4 is in `capability-evals.mjs` (a bus and a track compressor; the binding names the bus; the track-tap read succeeds) and has a test-value sentence. Later gates were renumbered 5-7, and nothing referenced them. | 13 |

## MAJOR

| ID | Applied | File(s) |
|---|---|---|
| N-A | Applied. Each brace-expanded policy line is now an explicit `for x in …; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh \|\| exit 1; done` loop. The policy scripts take no top-level arguments (checked). 18b's loop includes builtins and host-core. DESIGN 7 gets the note "This is a list, not a command". | 03, 04, 05, 06, 18b, DESIGN |
| N-B | Applied as written. D9 now reads "the qualifying count on the base plus one (13 if the base still carries twelve)" and the ratchet comment is updated. The path line and the gate line no longer use the invalid `--kernel-min 12` argument, and the gate records the counts before and after. | 22 |
| N-C | Applied. In gate 1, the contributors carry every entry with `bypass: true`, and session B's reference source is delayed by `L = rate/100 + 6`. | 05 |
| N-D | Applied. Slice 00 D5 names each batch's closing slice. Slice 08 gets deliverable 6 and the path `AGENTS.md` (those qualifiers only), covering the K1 sentences. Slice 26 gets deliverable 5 and the same path, covering the route-mute and follow-mute sentences. | 00, 08, 26 |
| N-E | Applied. 19 gate 4 now uses the relative growth over the unmuted compile, plus a test-only "no `RouteActivity`" accessor, which was added to D7. The base-commit equality is now PR evidence. 23 gate 4's fallback now stops for a Sol ruling, and 23 gate 6's base total is PR evidence. | 19, 23 |
| N-F | Applied. Writer parity moves to `console-evals.mjs` against `engineCanonical()` (`:470-479`, verified). `enginectl-cli.mjs` is authorized and named in the gate. Slice 20 inherits this through 18b's paths. V1's path list and gate are fixed the same way. | 18b, V1 |
| N-G | Applied. Slice 06 adds a Context line for the sites slices 03-05 added, a re-grep at the K1 head, and a `hosts/host-web/src/tests.rs` path. 18b adds the `Route {` and `"gain_db"` re-greps at the K3 base and authorizes their hits for the added key only. Slice 20 inherits this through its paths. V1 takes the same step at filing: its Context has a `"submixes"` re-grep and its paths authorize the hits. | 06, 18b, V1 |
| N-H | Applied. 02 no longer has a direct submix estimate charge. 03 D0 says "charged once (…uncharged between 02 and 03)". 03 gate 1 adds a hand-count assertion, and its test value now names double charging. | 02, 03 |
| N-I | Applied. `headless-path-evals.mjs` is removed from the paths and from gates 3 and 4, and `console-evals.mjs` takes its place. The file's #330 header was verified. | 08 |
| N-J | Applied. The `live-controls-evals.mjs` stubs (`:139`, `:253-261`, `:367`, verified) are added to the list and the paths. `SHAPE` is now cited at `:7`. Gate 2 is split into (a) headless shipped ordering and (b) the `measurement-evals.mjs` projection and the `sdk.meter.submix_count` refusal. Gate 3's unobservable first bullet is removed; its test value now points to slice 17 gate 4. A Context note says the measurement module is browser-only. | 13 |
| N-K | Applied. Slice 19 authorizes `compile.rs` (the `resource_estimate` call at `:485` only, verified as the sole caller), and D6 and deliverable 3 now say the gates are passed there. 22 D8 now charges the `RouteActivity` table when the compile-time estimate did not. | 19, 22 |

## MINOR

| ID | Applied / skipped | File(s) |
|---|---|---|
| 4.1 | **Applied (split).** Slice 18 is now two slices. **18a** is a new file, `18a-gate-every-routes-coefficients-through-one-function.md`, H1 *Gate every route's coefficients through one function*. It covers `RouteGate`, `PreparedRoute.gate` (always `OPEN`), `gated_route_coefficients`, `route_coefficients`, the 17 literals and `PlanningMetadata`; its gates are the determinism diff plus "one coefficient function". **18b** is the renamed file `18b-mute-a-route-in-the-session.md` and keeps the H1 *Mute a route in the session*. It covers the field, wire, opcode, SDK, migration, the `route-mute` row and the fold decline. Gate 2 extends 18a's test, and gate numbers 1-10 are kept, so slice 20's "gate 8" and "gates 9 and 10" still resolve. 18b depends on 18a; 19's dependency is unchanged. Cross-references were updated in 00, 19, 20, 21, 22, 23, 27 and V1. In DESIGN: 9.1 rows, the DAG (realigned), 9.5 K3 = 18a-26, the Size line, the table rows for 5.x and section 8, and R4. | 18a, 18b, 00, 19-23, 27, V1, DESIGN |
| 4.2 | Applied. DESIGN 7's browser build line is now prefixed `rm -rf <A> <B> && mkdir -p <A> <B> &&`, with its reason. | DESIGN |
| 4.3 | Applied. `crate::ffi::live_response_ffi_tests::measured`, following the pattern at `tests.rs:3638-3647` (verified). | 24, 26 |
| 4.4 | Applied. Gate 8 now requires equality with the D8 formula and at least the measured bytes. It runs for both the gated and the ungated case. | 22 |
| 4.5 | Applied. DESIGN 7 now says `aarch64-release` also runs `console-workload`. | DESIGN |
| 4.6 | Applied. Deliverable 4 deletes or folds the host-web K1 boot test. | 10 |
| 4.7 | Applied. The `LiveRouteState` constructor is frozen on `effective_mute: &dyn Fn(usize, usize) -> bool`. | 24 |
| 4.8 | Applied. D4 has a fallback: BM3's own spec plus an umbrella comment. | BM3 |
| 4.9 | Applied. DESIGN 9.5 now says "drafted during K2 review, implemented on the K2 head", with the reason. | DESIGN |
| 4.10 | Applied. Slice 23 gate 8 has a conditional (builtins-only request, or re-pins). Slice 00's #1053 annotation gets one sentence. | 23, 00 |
| 4.11 | Applied. 02:35 anchors (`schema.rs:950-970`, `session_wire.rs:1628-1629`, `visit.rs:252-253`); `Submix` loses `#[derive(Eq)]` (`model.rs:576`); 13:62 `SHAPE` `:7`; 18b/20 `SKILL.md:70` wording; 20:61 "A2 D1 amendment". | 02, 13, 18b, 20 |
| 4.12 | Applied. `SumDelay { line, channels_agree }`. `trace-graph-audit.sh` is added to gate 5. | 04 |
| 4.13 | Applied. Gate 2 counts only at `Backend::current()`. The `plan_bank_groups` arm is dropped in favour of accept-or-`FOREIGN_CONSOLE_REFUSAL`, and DESIGN 7 is reworded to match. Gate 6 gets a test-value sentence. | 05, DESIGN |
| 4.14 | Applied. D1 `params: []`; gate 2 now reads "`console()` after a spec'd submix refuses"; gate 9 adds fmt, clippy and doc. | 08 |
| 4.15 | Applied. Gate 7 adds `check-capi-abi.sh --self-test`. `enginectl-cli.mjs` is added to the non-goal red list. | 02 |
| 4.16 | Applied. Slice 15 authorizes the `test-web-audioworklet.mjs` reasons-loop row (`:1820-1827`, verified). D3 states `RESULT_INVALID_ARGUMENT` (`lib.rs:4301-4310`, verified). Gate 1 gets a test value. Slice 16 gate 2 asserts the result code. | 15, 16 |
| 4.17 | Applied. New D5a classifies by the strip count. `boundary.ts` is authorized (the classifier only); the worklet was already under `web/**`. | 16 |
| 4.18 | Applied. Gate 6 adds the `effect-runtime` and `workspace` pairs. | 10 |
| 4.19 | Applied. Test-value sentences are added to 09 gate 3, 14 gate 3 and 05 gate 6. | 09, 14, 05 |
| 4.20 | Applied. A deliverable updates the doc comment at `solo.rs:86` (verified). | 14 |
| 4.21 | Applied. 12 D1 adds a two-view `peaks` note. 16 gate 5 now says the native test reads the frame word. | 12, 16 |
| 4.22 | Applied. 18a keeps the signature of `route_folds_over_program`, so `program/tests.rs` is untouched. 18b adds `check-browser-expected-resources.py --self-test`. | 18a, 18b |
| 4.23 | Applied. 19 D5 is frozen as `Option<&mut RouteActivity>`. | 19 |
| 4.24 | Applied. 20 gate 6 names `console_session_edits.rs`; the `InvalidEnum` doc-comment update is added to the deliverables; gate 5 extends 18a's test. Slice 21: the saturation case is merged into gate 3 and removed from gate 1. | 20, 21 |
| 4.25 | Applied. 22 D7 adds `QueueGeneration(0)` (`spsc.rs:236-238`, verified). D3 now says "each from the previous record's `coefficients_at(0)`". | 22 |
| 4.26 | Applied. 18b gate 4 and 22 gate 7 state `bank_route_folds()` == 0 plan-wide and record the unmuted nonzero count. Slice 04 gate 3 already states a plan-wide relative count, so it is left as is. | 18b, 22 |
| 4.27 | Applied. 23 D1 now says the attach precedes the cap check, so the charge joins `admitted_graph_and_model` (`prepare.rs:1077-1093`, verified). | 23 |
| 4.28 | Applied. Gate 7 adds `graph_fixture -- --check` and `trace-graph-audit.sh`. | 17 |
| 4.29 | Applied. Slice 00 files its own spec as the umbrella's first child (D2, deliverable 4, paths). | 00 |
| 4.30 | Applied. In gate 1, the contributors carry every console entry with `bypass: true`. | 06 |

Also fixed during the consistency pass: 18b's Context had a brace-expanded `derive-{…}` regeneration
command. It is now three explicit `python3 -I -B scripts/derive-*-console-fixture.py --validator … >
<fixture>` commands, in the order `check-console-fixtures.sh:33-37` uses.

## Consistency check (after the edits)

- **Titles.** There are 39 issue files. Every H1 equals its DESIGN 9.1-9.3 row (39 rows).
- **Dependencies.** Every Dependencies line names an existing H1, or the umbrella title, or #1053's
  exact title (the local spec's H1).
- **Italic references.** Every italic cross-reference resolves.
- **DESIGN agreement.** Body dependencies match DESIGN's "Depends on" column. 18a's are 08 and "merges
  after 17"; 18b's is 18a; 19's is 18b.
- **Scripts.** Every `scripts/…` named in a command exists. Each command uses its script's required
  argument form, checked against the script's own usage line: build `--named-twin <B> <A>`; `<A>` on
  the `sdk-*` and `headless` gates; `.` plus the audit path on the builtins fixtures; the audit path
  on `trace-graph-audit`; `debug` on `run-aarch64-tests`; the bare and `--self-test` forms.
- **Brace expansion.** No command line in a gate or a Context command is brace-expanded any more.
  Brace lists remain only in authorized-path lists, plus DESIGN 7's annotated descriptive list.

## Skipped

None.
