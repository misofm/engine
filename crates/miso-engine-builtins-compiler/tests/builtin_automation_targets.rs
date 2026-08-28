//! The cross-check that keeps `miso-engine-session`'s automation-target table honest.
//!
//! # Why the table is duplicated at all
//!
//! `miso-engine-session` depends on `miso-engine-core` and nothing else, and that is a policy
//! rather than an accident: `scripts/check-session-policy.sh` pins the crate's whole dependency
//! list. So the session crate cannot read `BUILTIN_PARAMETER_DESCRIPTORS_V1`, and its
//! `BUILTIN_AUTOMATION_TARGETS_V1` is a deliberate second spelling of it -- the same shape as
//! `scripts/check-parameter-metadata-v1.py`'s second spelling of the command-kind list.
//!
//! This crate can see both, and this is where the two are held together. It is not a restatement
//! of the rule; it *derives* the expected table from the ABI, so a descriptor row whose
//! `update_rate` or `scope` moves without the session table moving is red here on the next run.
//!
//! The rule being derived: an automation target names something the render plane can be told to
//! change, which is exactly a row declaring `BuiltinParameterUpdateRate::BlockTarget`.
//! `PreparedOnly` rows -- `hpf_hz`, `lpf_hz`, `delay_samples` -- have no post-preparation write
//! path at all, so a span addressed at one could only ever be inert syntax, and the schema refuses
//! them rather than accepting them and doing nothing.

use miso_engine_builtins::{
    BUILTIN_PARAMETER_DESCRIPTORS_V1, BuiltinParameterScope, BuiltinParameterUpdateRate,
};
use miso_engine_session::{BUILTIN_AUTOMATION_EFFECT_ID_V1, BUILTIN_AUTOMATION_TARGETS_V1};

/// The session crate's automation-target table is exactly the block-target rows of the builtin
/// parameter ABI, with each row's `per_lane` flag taken from its declared scope.
#[test]
fn builtin_automation_targets_match_the_parameter_abi() {
    let expected: Vec<(u32, bool)> = BUILTIN_PARAMETER_DESCRIPTORS_V1
        .iter()
        .filter(|descriptor| descriptor.update_rate == BuiltinParameterUpdateRate::BlockTarget)
        .map(|descriptor| {
            (
                descriptor.id,
                descriptor.scope == BuiltinParameterScope::PerLane,
            )
        })
        .collect();
    assert_eq!(
        BUILTIN_AUTOMATION_TARGETS_V1.to_vec(),
        expected,
        "the session crate's second spelling of the builtin automation targets has drifted from \
         the parameter ABI it restates"
    );
}

/// Every parameter the schema refuses is refused for the one stated reason, and every parameter it
/// admits is admitted for the mirror of it.
///
/// The negative half matters more than the positive one: a row silently promoted to `BlockTarget`
/// would be admitted by the schema without anyone deciding it should be, and a row silently
/// demoted would leave sessions in the field naming a target that no longer exists.
#[test]
fn refused_targets_are_exactly_the_prepared_only_rows() {
    let admitted: Vec<u32> = BUILTIN_AUTOMATION_TARGETS_V1
        .iter()
        .map(|(id, _)| *id)
        .collect();
    for descriptor in &BUILTIN_PARAMETER_DESCRIPTORS_V1 {
        let live = descriptor.update_rate == BuiltinParameterUpdateRate::BlockTarget;
        assert_eq!(
            admitted.contains(&descriptor.id),
            live,
            "`{}` (id {}) declares {:?} but the schema {} it as an automation target",
            descriptor.name,
            descriptor.id,
            descriptor.update_rate,
            if admitted.contains(&descriptor.id) {
                "admits"
            } else {
                "refuses"
            }
        );
    }
    // The deferred tier, named so that reopening it is a deliberate edit here as well as in the
    // ABI: `hpf_hz`, `lpf_hz` and `delay_samples`.
    for id in [3_u32, 4, 11] {
        assert!(
            !admitted.contains(&id),
            "builtin parameter {id} is deferred and must not be an automation target"
        );
    }
    // Issue #210 phase 3 admitted these two. The assertion is here so that a phase that reverted
    // the liveness would have to revert this line too.
    for id in [1_u32, 2] {
        assert!(
            admitted.contains(&id),
            "builtin parameter {id} is live since #210 phase 3 and must be an automation target"
        );
    }
}

/// The fixed `effect_id` literal is spelled once and is the strip's vocabulary word.
#[test]
fn the_builtin_automation_effect_id_is_the_strip() {
    assert_eq!(BUILTIN_AUTOMATION_EFFECT_ID_V1, "strip");
}
