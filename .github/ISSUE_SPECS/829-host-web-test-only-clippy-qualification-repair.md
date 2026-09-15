# Repair host-web test-only Clippy diagnostics blocking required qualification

At protected-read source checkpoint `c968c791`, the required `qualification.yml` lint step, `CARGO_TARGET_DIR=/home/bl/misofm/engine/target cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, exits 101 on exactly four unchanged foundation test-only diagnostics: `hosts/host-web/src/ffi.rs` private preparation-case tuple (`type_complexity`), `wire_command` fixture helper (`too_many_arguments`), `command_state` fixture snapshot (`type_complexity`), and `hosts/host-web/src/tests.rs` literal-boolean assertion (`bool_assert_comparison`). Sol HIGH read-only brief confirmed the four findings, four `wire_command` call sites and two `command_state` uses. #826's committed-output source contract passed, but that issue allowed `ffi.rs` only and did not claim this broader qualification gate.

## Scope and decision

Allowed edits only within existing `#[cfg(test)]` fixture modules of `hosts/host-web/src/ffi.rs`, plus the named literal-boolean assertion in `hosts/host-web/src/tests.rs`. Give the preparation-case function tuple and command-state snapshot named test-only types; group the eight `wire_command` inputs into a named fixture record and update its four call sites; replace the boolean `assert_eq!` with `assert!`. Preserve fixture input values, raw command bytes, comparison fields and expected results. No production behavior, ABI/layout/API, workflow, feature implementation, lint allow, new harness or unrelated cleanup.

## Objective gates

- The exact required workspace all-targets/all-features strict Clippy command passes under the pinned toolchain. Record baseline/final output and any newly exposed diagnostic.
- Focused locked host-web lib `test-support` fixtures for private protected boot nested headers, mixed observation-batch command rejection, and protected response admission remain green; full locked host-web lib `test-support` remains green.
- `cargo fmt --all -- --check` and `git diff --check` pass. No benchmark or artifact promotion.

## Delivery/evidence

Smallest closable outcome: one test-only qualification repair that allows the required `main` lint job to evaluate the protected-observation branch; it does not itself ship protected EQ or change source contracts. At most four issue-wide coherent attempts: two fresh Luna MAX rounds, then one Sol HIGH round if needed, then Astra XHIGH. Each candidate gets one independent fresh Astra MEDIUM adversarial verdict; fixture-only corrections count. Root commits exact-path green or candid useful checkpoints, pushes promptly, synchronizes issue evidence and closes only after PASS/upstream. Record checkpoint hash, exact gate results, reviewer verdict and remote state here.

Decision/evidence: GitHub issue #829 was created with this exact title and matched to the local numbered spec at pushed `8277e1b1` before implementation; local/remote bodies and open state were verified. Sol HIGH read-only brief approved the bounded scope and required gate. A boundary audit found all current #825–#829 specs/states matched; 17 older GitHub-only tracker/postlaunch bodies were inventoried but left remote to avoid adding 9,021 lines of unrelated prose to this feature branch.

Round 1, bounded Luna MAX tranche A at pushed `9aeaf839`: only test fixtures changed. Six malformed protected-boot preparation cases now use a named record with identical mutations/results; one response assertion uses `assert!(bool)`. Three focused locked host-web fixtures and full locked lib `test-support` (166 passed, 2 ignored), workspace format and diff checks passed. Strict host-web all-targets Clippy then reported only the two next-tranche `wire_command`/`command_state` diagnostics; GitHub #829 body/comment refreshed.

Round 1, bounded Luna MAX tranche B at pushed `8a0446aa`: `wire_command` uses a named test-only input record at all four call sites; `command_state` uses named snapshots and field comparisons at its two uses. Raw command bytes, input values and expected fields remain identical. Exact required workspace all-targets/all-features strict Clippy passed; five focused command fixtures, full locked host-web lib `test-support` (166 passed, 2 ignored), workspace format and diff checks passed; GitHub #829 body/comment refreshed.

Independent fresh Astra MEDIUM round-1 verdict: **PASS** at `8a0446aa`. Reviewer verified all six preparation mutations/expected results, four encoded command call sites, every original six-control/four-filter snapshot field and report comparison, permitted test-only paths, and absence of lint suppression/production/API/layout changes. It independently ran the exact mandated workspace all-targets/all-features strict Clippy, three focused fixtures, full locked host-web lib `test-support` (166 passed, 2 ignored), format and diff checks; all passed. PASS evidence checkpoint hash, GitHub close and remote verification: pending.
