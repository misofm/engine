//! Issue #1051: the multiband compressor's randomized bank differential.
//!
//! `conformance::run_effect_differential` renders every bank this build binds against its scalar
//! instances -- whole and chunked, dual and collapsed, continued and restored with hostile state
//! words -- and probes `bind_homogeneous_bank`'s three outcomes, all through the contract calls. A
//! failing seed prints the command that replays it.
//!
//! # Ramp cuts (#1069)
//!
//! The first run of this differential found that a multiband lane's rendered bits depended on
//! where its in-flight ratio, attack or release ramp was cut: by a neighbour lane's (or the other
//! channel's) ramp arrival, or by a block boundary. Each segment sampled those three ramps once,
//! at its first frame, and segment boundaries are shared by the whole bank and by the host's
//! blocks. Since #1069 the kernel derives each lane's coefficients every frame from that frame's
//! ramp words: the ratio's `1/R - 1` lane-wide, and attack and release from coefficient ramps that
//! walk to the exact design (the compressor's rule). This test runs at full strength: automation
//! on any lane, chunked blocks, and restores that start a ramp on any lane. It was the ignored
//! reproducer until then, and is red on that fix's revert.

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    multiband_compressor::MultibandCompressorFactory,
    seeds: 24,
    blocks: 24,
    craft: None,
    known: &[],
    banks_natively: true,
    witness: false,
);

/// The D7 recovery's report against the contract, on fixed input (no seed): one block, on the
/// failing lane and channel alone (#1073).
#[test]
fn the_d7_recovery_reports_one_block_on_the_failing_lane() {
    conformance::assert_d7_reports(Box::new(multiband_compressor::MultibandCompressorFactory));
}

/// #1278: the plan-swap carry restores every lane it carries, so a restore must accept every state
/// the effect itself reaches -- here each sampled state of a smoothed ramp to a domain edge, at
/// every launch rate.
/// Since #1409 every ramp word stays between its start and its target, so the strict restore
/// (#1411) admits each snapshot. Red when the render site whose word the snapshot holds leaves a
/// ramp word outside its endpoints at render (a missing or reverted #1409 clamp), or when the
/// restore refuses a valid in-range snapshot. The fix is that clamp or that validation, never a
/// restore slack.
#[test]
fn the_effects_own_edge_ramp_snapshots_restore() {
    conformance::EffectDifferential::assert_edge_ramps_restore(Box::new(
        multiband_compressor::MultibandCompressorFactory,
    ));
}
