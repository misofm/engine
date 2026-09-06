# Issue #528 Sol attempt 2 evidence report

## Reviewed source identity

Final checkpoint: `d605499a092f95e86533c41aeb4d2a0607a9bc28` (clean worktree).

- `crates/host-core/src/scalar_point_endpoint.rs`: SHA-256
  `263650e63384ecf898c965cde26745d015a1acec5da7002fa4c4a417dbf06fcf`, Git blob
  `65646667f57d328aa343ce98ab402cc6a5438c42`.
- `crates/host-core/tests/scalar_point_endpoint.rs`: SHA-256
  `ae97645cbf621257854ebae7be9029281c446ec25cd58e60b84fdf96c325d100`, Git blob
  `a24a006edee75845e57479fd04d8c7d74c23357e`.

The focused debug/release, full host-core and protocol regression captures were made at source
checkpoint `eb4247534f36bde561a56fbf28582a88bc91a358`. The only later test-source change is checkpoint
`d605499a`: `drop(reject)` was replaced by an equivalent lexical scope after strict Clippy reported
`drop_non_drop`. `final-clippy-repair-focused` reran all six endpoint tests on that corrected test
blob, and `final-clippy-fixed` compiled every host-core target with `-D warnings` on the final clean
checkpoint. Runtime assertions are unchanged, so the earlier release/full-package/protocol results
remain applicable with this explicit source distinction.

## Six frozen product gates

| Gate | Test and evidence |
| --- | --- |
| 1 | `points_slice_real_pcm_and_readback`: equally warmed asymmetric compressors; independently split existing native span calls at offsets 3/7 and simultaneous Left/Right; bitwise equality for both PCM planes; aggregate native report, native parameter readback, resident observation and complete Left/Right payload equality after borrow release; warmed no-event PCM differs. |
| 2 | `claim_state_is_truthful_until_terminal_collection`: queued/resident, handed-off-unclaimed, future-first-at-block-end, partial prefix/next sample, complete-but-uncollected and collected states; native target changes only on application; outstanding credit survives terminal render until collection; a following zero-admission block retains stable progress. |
| 3 | `late_points_apply_in_order_and_second_ticket_waits`: two originally distinct same-binding Points apply late at boundary 16 with zero samples between; second ticket waits for boundary 32; applied/late counts and last-application samples do not replay; direct ordered native retarget reference matches both PCM planes, final parameter state and full payload through a continuation block. |
| 4 | `real_cancellation_preserves_applied_prefix`: real partial and future-ticket cancellation; exact effective samples, applied prefixes, canceled remainders, reliable events, cleared claims and released credit; no post-barrier target application; mixed Point/Step/Linear batch remains whole at `PendingUnsupported` and FIFO-blocks a supported follower; real prior cancellation events fill reliable capacity and make the next cancellation refusal transactional; B+1 capacity remains Full after handoff and terminal publication until control collection. |
| 5 | `malformed_admission_and_render_envelopes_are_noops`: malformed empty/oversize/order/Point equality/revision/handle/domain/valid-first-invalid-last/past batches preserve ownership and state; invalid PCM shape/time preserves output and native payload. A forwarding real compressor faults Apply only after seven real frames and one successful Point, preserving native/service prefix 1/1, whole-block silence, first-fault stickiness and fault-only cancellation progress/event. A second wrapper faults snapshot Read only after a real 16-frame render with a claimed future ticket, then proves no further DSP. |
| 6 | `preparation_resources_and_success_path_are_bounded`: isolated self-child installs the bench allocator and proves positive allocation/free controls; endpoint preparation matches direct unchanged `PreparedAutomationDelivery::prepare` for allocation count and requested bytes with zero reallocations/preparation frees while owners remain live; reported delivery values and actual inline owner sizes match. Four real Point/render/read cycles and a real fault-only cancellation boundary run inside realtime audit scopes with zero allocation, free or other forbidden operation and with actual progress and nonsilent PCM. |

## Final proportional gates

Every command has exact argv, environment, checkpoint, dirty status, source hashes, stdout, stderr
and status in `/tmp/issue528-sol2/<label>.*`.

- `final-focused-debug`: status 0; endpoint outer suite 6 passed, isolated gate-6 child 1 passed.
- `final-focused-release`: status 0; endpoint outer suite 6 passed, isolated gate-6 child 1 passed.
- `final-host-feature-tests`: status 0; 84 tests passed including the isolated gate-6 child, 2 ignored.
- `final-host-default-tests`: status 0; 72 passed, 2 ignored.
- `final-protocol-delivery-unit`: status 0; 15 passed, 124 unit tests filtered; unrelated integration binaries selected zero tests.
- `final-protocol-delivery-ownership`: status 0; 2 passed.
- `docs-check`: status 0; locked host-core feature check.
- `docs-tests`: status 0; 2 ordinary doctests and 5 compile-fail doctests passed, including prepared-owner `Send` and started-owner `!Send`/`!Sync` proofs.
- `final-clippy-repair-focused`: status 0; final test blob, endpoint outer suite 6 passed and isolated child passed.
- `final-clippy-fixed`: status 0; locked host-core all-targets feature Clippy with `-D warnings`.
- `final-fmt`: status 0; workspace formatting check.
- `final-source-diff-check`: status 0; clean-checkpoint source whitespace/error-marker check.
- `final-workspace-policy`, `final-host-core-policy`, `final-realtime-policy`: status 0.
- `final-wasm-scalar`: status 0; locked feature-enabled host-core check for
  `wasm32-unknown-unknown`, `RUSTFLAGS=-Ctarget-feature=-simd128`, isolated target directory
  `/tmp/issue528-target-wasm-scalar`.
- `final-wasm-simd`: status 0; the same feature-enabled target check with
  `RUSTFLAGS=-Ctarget-feature=+simd128`, isolated target directory
  `/tmp/issue528-target-wasm-simd`.

No workspace-wide native ABI or browser suite was run in this proportional phase; root reserved
immutable delivery qualification until after the consolidated adversarial verdict.

## Evidence limits

`ScalarPointRenderError::SampleOverflow` and `ScalarPointCancelBoundaryError::SampleOverflow` are
defensive public results but cannot be reached by a bounded public-API fixture: endpoint time starts
at zero, the quantum is a validated nonzero `u32`, and only successful contiguous boundaries advance
it. Tests do not fabricate private state or use unsafe mutation.

The endpoint forwards `DeliveryResourceReport::largest_allocation_bytes`; it is not measured by
`bench_support::alloc::Counters`, which has no maximum-request field. Largest-request evidence is
inherited from unchanged #460 source proof
`protocol::delivery::tests::logical_record_bounds_and_resource_report_are_exactly_composed`, which
enumerates the ring and owner layouts and computes their maximum. The endpoint preparation delta is
independently supported by equal allocation count/requested bytes versus direct delivery preparation;
requested bytes are not mislabeled as retained payload or an allocation-size distribution.

## Preserved preliminary and failure evidence

All preliminary files remain under `/tmp/issue528-sol2`; checkpointed copies through the completed
fixture/doc tranches are under `artifacts/issue528-sol-attempt2`. Attempt-1 evidence remains under
`artifacts/issue528-luna-attempt1`.

- `focused-prelim.log`: initial locked run refused a mechanically stale lockfile before the approved
  test-only dependency/lock checkpoint.
- `check-source.log`: early compressed scaffold missed a required semicolon and failed type checking.
- `check-source-2.log`: strict missing-docs check enumerated the then-undocumented public API.
- The first gate-5 local compile exposed `usize`-to-`u64` conversion error E0277 and heterogeneous
  predicate-closure inference error E0282. Its raw tool output was observed in-session but was not
  redirected or retained on disk; it must not be represented as a captured log. The subsequent
  focused gate-5 capture records the corrected source.
- `focused-debug`, `focused-debug-2`: early compiling/focused-green recovery captures that did not
  establish the later complete six contracts.
- `pcm-fixture-debug` and archived `pre-final-pcm-fixture/pcm-fixture-debug.*`: pre-final gate-1
  captures; `pcm-fixture-debug-fp` supersedes them by adding the canonical FP guard.
- `gate5-debug`, `gate6-debug`, `remaining-gates-debug`, `docs-check`, `docs-tests`: bounded tranche
  captures tied to their recorded dirty source hashes/checkpoints.
- `final-clippy`: status 101, preserved exact `drop_non_drop` diagnostic on `drop(reject)`.
  `final-clippy-repair-focused` and final clean-checkpoint `final-clippy-fixed` establish the lexical
  scope correction without erasing the failure.

No benchmark, mutation campaign, fabricated delivery error, new generic harness, or broad
workspace/browser qualification was added.
