# Charge the transition-planning transient in both admissions

Filed by root from the #2232 verify (2026-11-01). Follow-up; not launch-blocking.

## Problem
After #2232, a structural edit that takes the transition still allocates a transient in `plan_strip_transition` that no admission charges: 0.63 MB at 64 restarted strips, 2.53 MB at 256 and 10.1 MB at 1,024 (about 9.9 KB per strip; measured ceiling about 1.06 × `canonical_upper_bound_bytes` of the committed model). The replacement pre-check charges exact rows only (#2199 D3), so a session near its budget can pass the pre-check and then allocate past it while planning the transition.

## Scope
Charge the transition-planning transient as one row in both the replacement pre-check and the exact admission after preparation, with the same rule, so the pre-check never refuses what the exact admission admits (#2199 D3). Derive the row from counts known before preparation (restarted strips and the model's canonical bound), not from a measured constant.

## Gates
- A test with `bench_support`'s thread-scoped peak: the transition-planning peak on a bus add at 64 and 256 tracks stays under the new row (a ceiling). Red when the row is left out of the pre-check.
- The C ABI exact-boundary tests stay green unchanged.
- No rendered bit moves (one-time comparison, PR evidence only).

## Decisions (attempt 1)
- D1. The row is `host_core::strip_transition_ceiling(committed)`, from the compiled model of the plan the successor displaces (`base.committed`: the committed model, or the kept model across a withdrawn candidate). `plan_strip_transition` copies that model only; it reads the successor model and never copies it. So the row needs no restarted-strip count, and both admissions have the same value before preparation. The restarted set is known only after preparation (`prepared_runtime.inventory.restarted_strips()`); the row does not need it.
- D2. The rule is a sum, each term from the committed model: three model copies at `compiled_model_bytes - canonical_bytes`; `3 * canonical_bytes` for the classifier's two canonical documents (each copy differs only in mutes set to `true`, and one document grows by doubling to less than twice its length); the row diff's two owner buffers (twice the largest owner's row count plus four rows, with its address text, and every VCA row the same way), found by one owner walk; the changed rows (two mute lanes per strip and route); and the records and lists. The effective values, automation indexes and change set are held only after the documents and the masked copy are freed, and those bytes cover them. No measured constant.
- D3. Both admissions charge the row once, in the replacement-peak term (`maximum_replacement_peak_bytes`, #1290a D3), as `ProspectivePlan::transition_bytes`. It is not charged in the graph cap: it is a transient, not a retained graph or model row. The C ABI leaves the replacement peak unbounded, so its exact-boundary rows do not move. The browser sets it to the memory budget. The reprepare peak charges it once too: one transition is planned at a time. A declared discontinuity plans no transition and charges 0; the transition across a donor is admitted by the donor's submit, as before.
- D4. Before a replan, the first planning's result is freed (`drop(transition.take())`), so one planning is held at a time, as the row counts it.

## Evidence (attempt 1)
- Ceiling against the thread peak of `plan_strip_transition` on the console bus add, every strip restarted: 1 track 129,171 B under 178,680 B; 64 tracks 633,472 B under 1,080,084 B; 256 tracks 2,525,248 B under 3,827,220 B; 1,024 tracks (scratch) 10,092,664 B under 14,816,674 B. Measured, not proved: the peak is at most 0.84 of the row over the verifier's sweep of about 70 shapes (0.836 with 64 VCAs over 136 tracks, at a document-doubling step). The row counts a document by its capacity; a copying reallocation could hold up to one more document length for a moment, which the margin covers but nothing proves.
- Gate 1 (`crates/host-core/tests/warm_lead_peak.rs`, `the_transition_of_the_console_bus_add_stays_under_its_ceiling`, at 1, 64 and 256 tracks): red without the documents term (64 tracks: 633,472 B over 551,055 B) and without the owner-buffer term (1 track: 129,171 B over 23,574 B).
- The C ABI exact-boundary tests pass unchanged (`double_live_oracle_drives_exact_and_one_below_c_caps` and the single-plan rows).
- No rendered bit moved: the 34 digests over `fixtures/session/v1` (C ABI, 300 blocks, plain and live, scratch harness) are identical at base d201e3082 and at head.
- Browser: `--check-matrix` passes on Chromium 151 and WebKit 26.5, single and worker; `console-limits` replies `ok` on the rebuild path, render allocations 0.
- Checks: `cargo fmt --all --check`; workspace clippy `--all-targets --all-features -D warnings`; release tests of host-core, control-plane, capi and host-web with `--all-features` (1,149 passed, 0 failed); `check-workspace-policy.sh`; `realtime_policy`; `check-test-support-ci.py`; `audit capi` (0 violations); the web build and `check-web-audioworklet.sh`: all pass. The aarch64 `cargo check --release --tests --all-features` passes for the touched crates; the whole workspace stops on this host at `wasmtime`'s build script, which needs `aarch64-linux-gnu-gcc` (not installed).

## Review (attempt 1 verify: FAIL, one MAJOR)
- MAJOR: gate 2 stayed green with the row charged nowhere: it bisected the smallest admitting peak from whatever was charged. Fixed in attempt 2.
- MINOR: `warm_lead_peak.rs` did not compile without `control-provider`; the new test is now gated on it.
- MINOR: the "0.75 of the row" claim held for the fixtures only; restated as measured above.

## Evidence (attempt 2)
- Gate 2 (`crates/control-plane/tests/session_replace.rs`, `a_replace_whose_transition_cannot_fit_is_refused_before_it_is_prepared`): a replace that adds a bus to 64 tracks. A test-support hook (`control_plane::test_transition_admission`) charges no row or skips the admission before preparation. With the pre-check skipped, the exact admission's smallest admitting peak is 4,105,947 B with the row and 3,564,773 B without it: the difference is the row, 541,174 B, to the byte. With both admissions, half the row below the smallest admitting peak, the replace is refused `control.resource.replacement_peak` with no preparation.
- Red mutations: the row left out of the pre-check (prepared once, then refused); left out of the shared `replacement_peak_fits` (difference 0, not 541,174); left out of the exact admission (difference 0).
- Checks (attempt 2): `cargo fmt --all --check`; workspace clippy `--all-targets --all-features -D warnings`; host-core clippy `--all-targets` without features; release tests of host-core without features (293 passed) and of host-core, control-plane, capi and host-web with `--all-features` (1,149 passed, 0 failed); `realtime_policy`: all pass.
