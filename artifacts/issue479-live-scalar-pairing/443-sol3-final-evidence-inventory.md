# #443 final Sol3 evidence inventory

Reviewed clean pushed source head `205fa3b5ebb51a876b64c3729bab3c3be5fec01a` (root confirms later policy-only main integration did not change runtime/build/fixtures).

## Original product groups

### A — actual compiled scalar command/state/mechanism

Satisfied by new `actual_scalar_graph_queues_fuse_and_fall_back_against_separate_owners`: real `Backend::Scalar` bound plan; asserted adjacent PostFader/PostMatrix schedule; selected private mechanism; independent Concurrent preparation as old two-owner execution; exact post-matrix PCM and exact private fader/matrix state every call; FIFO same-sample immediate retargets; short ramp stays fallback for its whole call and next call fuses; long ramp and mid-ramp retarget; mute, remembered fader gain and unmute. `scalar_pair_reset_matches_reset_original_owners` applies the existing private scalar stage resets to the retained composite and independently reset owners, then compares state and next-call PCM.

The SAME selected assertion was mutation-tested by temporarily appending `&& false` to `scalar_pair_is_in_place`: `/tmp/443-sol3-selection-mutant-fails.{command,log,status}` status 101. Exact PCM equality runs before and passed; the failure is the `(process_calls, process_members) == (1,1)` assertion. Source was restored and `/tmp/443-sol3-selection-mutant-restored.*` is status 0. No mutant remains in Git diff.

`scalar_invalid_envelope_leaves_the_later_matrix_queue_untouched` now immediately observes fader drain 1 and matrix drain 0 after invalid lane envelope. `actual_scalar_graph_preserves_scheduled_matrix_prefix_error_and_queue_tail` proves actual same-buffer scheduled execution: equal first error, completed fader state, valid matrix-prefix state, two matrix records consumed through the invalid record, trailing record retained and consumed on retry, and exact retry PCM/state against old owners.

Asymmetric/recovery/mono behavior remains covered by `actual_graph_mono_collapse_disengages_on_input_command_and_recovers_nonfinite_input`; Concurrent reference is used throughout actual scalar fixtures. This older test includes vector-bank input processing but its selected fader/matrix member witness remains regression evidence rather than the new Scalar proof above.

### B — host applicability, observations and declines

The adopted `/tmp/astra-443-final-proof-seams-ruling.md` establishes native/shipped-SIMD host scalar inapplicability: `Backend::current()` plus all-bankable builtin groups never emits a scalar residual. The existing real host `acknowledged_pair_render_records_the_same_live_dispatch` remains green and proves actual ack/application sample/PCM/shared TrackControlProducer contract on its bank owner. New actual Scalar fixtures drive the same TrackControlProducer/consumer record types and prove record drain/state/PCM. No native scalar end-to-end execution is claimed and no backend override was added.

New actual Scalar graph proofs:
- post-fader observed selected track declines, publishes a real meter window and matches old-owner PCM;
- `staggered_observed_scalar_track_stays_separate_while_eligible_peer_pairs`: observed track stays separate with nonempty meter data and nonzero asymmetric crossfeeding-matrix PCM matching reference while its adjacent unobserved peer selects;
- `actual_scalar_extra_reader_declines_and_retains_separate_owner_pcm`: observed alias/extra-reader decline with no factory/dispatch and exact old-owner PCM;
- `actual_scalar_nonunity_send_reader_declines_with_reference_output`: actual 0.5-gain send reader stays separate while the peer pairs; route output and post-matrix PCM match reference;
- post-matrix capture observers remain permitted on every positive actual Scalar fixture; selected witness proves they do not block pairing;
- explicit `RouteSource`/`EffectSidechain` crossing-reader guard and explicit retired-op guard are now at admission. Redirects apply only to bank runs while scalar admission requires empty membership, which is the source invariant excluding redirected scalar ops.

Factory/type/policy/default-hook/wrong-order preservation is covered by `scalar_factory_checks_both_exact_owners_and_policies_without_consuming_state`, `late_factory_guards_return_live_policy_width_and_population_owners`, and `declined_factory_owners_keep_queued_state_and_render_in_original_order`. Same-buffer/in-place/delay defensive checks and destination/error effects remain covered by `synthetic_distinct_matrix_destination_is_the_scalar_pair_identity_decline` (correctly synthetic; #476 retains applicability). Stage order is directly pattern-matched as exact PostFader then PostMatrix after consecutive actual ops. Nonadjacency remains #470.

No separate actual EffectSidechain graph fixture was added. The admission guard explicitly rejects `EffectSidechain`, and full graph/debug/release suites cover graph sidechain lowering generally. This is source-invariant proof rather than an actual #443 sidechain fixture.

### C — resource and allocation

Satisfied:
- independent live scalar field layout/padding and exact two-pointer outer assertions;
- serialized/Concurrent/no-console/vector-bank resource controls;
- production `graph_scalar_owner_resource` plus `checked_add_scalar_owners` now augments the manual actual Scalar fixture before artifact binding, matching graph-compiler production accounting instead of deriving allowance from measurements;
- graph-compiler `live_scalar_owner_bytes_are_published_and_capped_before_binding`: graph/plan/largest exact and one-below caps and ownership return;
- `scalar_owner_resource_overflow_leaves_the_graph_estimate_unchanged`: checked overflow with no partial mutation;
- `actual_scalar_prepare_and_bind_retain_the_charged_owner_layouts`: bounded existing allocator separately observes exact fader/matrix owner size/alignment/count at materialization and exact outer at bind, records no release of original-owner layouts during bind, bounds outer by saved admitted largest, and proves positive off-render frees with no allocation;
- `actual_queued_scalar_graph_allocates_and_frees_nothing`: independent allocation/free liveness and repeated selected settled, long-ramp fallback, resettled and observed-decline renders with zero allocation and zero frees.

The adopted seam ruling replaces unavailable post-bind report retrieval with this direct retained-ownership observation. No runtime metadata/public getter was added.

### D — gates

All status files below contain 0:
- Debug full: `/tmp/443-sol3-final-debug-builtins.*`, `debug-compiler.*`, `debug-graph.*`, `debug-host.*`. Compiler includes 37 unit + allocation 6 + integrations; graph 55 unit + allocation integration; host 63 passed/1 ignored + 2 integration.
- Release full: `/tmp/443-sol3-final-release-builtins.*`, `release-compiler.*`, `release-graph.*`, `release-host.*`, `release-allocation.*` (6/6 including both new scalar tests).
- Approved clean-target graph-compiler release lib: `/tmp/443-sol3-final-release-graphcompiler-lib.*`, 61/61, ordinary release profile, fresh `CARGO_TARGET_DIR=/tmp/engine-443-final-graphcompiler-release-lib`.
- Strict affected all-target Clippy: `/tmp/443-sol3-final-clippy.*`.
- Format and diff: `/tmp/443-sol3-final-fmt.*`, `/tmp/443-sol3-final-diffcheck.*`.
- Frozen policies: `/tmp/443-sol3-final-policy-{realtime-2,lane-2,unfused-2,workspace-2,audit-leak-2,wasm-atomics-2}.*`.

Preserved failures include initial compilation/assertion iterations, the intended selection mutant failure, the known unfiltered release collision evidence from Sol2, and an initial direct script permission failure (`policy-realtime`); corrected `bash` policy invocations all pass.
