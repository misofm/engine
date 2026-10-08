#![allow(missing_docs)]

//! `NativeEffectRegistry::new` is the one release-build descriptor validation point (issue #1330):
//! `validate_prepare_request` only debug-asserts it, so a registry that admitted an invalid main
//! descriptor would let it prepare in release. It is also where each effect's tail bound is
//! evaluated once per launch rate and quality and checked for consistency (issue #1462 D2).

mod support;

use effect_contract::*;
use support::{EFFECT_ID, descriptor, registry, unstated};

static VALID: EffectDescriptor = descriptor(1, unstated);
static WRONG_CONTRACT_MAJOR: EffectDescriptor = descriptor(2, unstated);

#[test]
fn registry_refuses_an_invalid_main_descriptor() {
    // The fixture differs from an admissible one only in `contract_major`, so the refusal below is
    // the registry's main-descriptor check and not some unrelated fixture defect.
    assert_eq!(validate_descriptor(&VALID), Ok(()));
    assert!(validate_descriptor(&WRONG_CONTRACT_MAJOR).is_err());
    let registry = registry(&VALID).expect("the valid twin enters the registry");
    assert!(registry.get(EFFECT_ID).is_some());

    let error = match support::registry(&WRONG_CONTRACT_MAJOR) {
        Ok(_) => panic!("a descriptor with contract_major 2 must not enter the registry"),
        Err(error) => error,
    };
    assert_eq!(
        error,
        RegistryError {
            code: "effect.descriptor.invalid",
            id: Some(EFFECT_ID),
        }
    );
}

/// The one launch rate each inconsistent fixture below breaks its rule at; every other rate states
/// [`consistent`]'s values, so a registry that checked only some rows would admit it.
const BROKEN_RATE: u32 = 96_000;

const REST: RestBound = RestBound::Bounded(RestSamples {
    peak_plus_24_dbfs: 20,
    any_sanitized_input: 40,
});

/// A consistent bounded statement: `tail <= tail_every_peak`, both finite, the rest bounded.
fn consistent(_: u32, _: EffectQuality) -> NodeTailBound {
    NodeTailBound {
        tail: TailSamples::Finite(5),
        tail_every_peak: TailSamples::Finite(10),
        rest: REST,
        composition: CompositionBound::Unstated,
    }
}

fn at_broken_rate(sample_rate: u32, broken: NodeTailBound) -> NodeTailBound {
    if sample_rate == BROKEN_RATE {
        broken
    } else {
        consistent(sample_rate, EffectQuality::Normal)
    }
}

/// Breaks only (a): a finite `tail_every_peak` shorter than the finite `tail`.
fn finite_peak_below_tail(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            tail: TailSamples::Finite(10),
            tail_every_peak: TailSamples::Finite(5),
            rest: REST,
            composition: CompositionBound::Unstated,
        },
    )
}

/// Breaks only (a): a finite `tail_every_peak` under an infinite `tail` (`Infinite` is largest).
fn finite_peak_below_infinite_tail(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            tail: TailSamples::Infinite,
            tail_every_peak: TailSamples::Finite(10),
            rest: REST,
            composition: CompositionBound::Unstated,
        },
    )
}

/// Breaks only (b): an unstated rest beside a finite `tail_every_peak`.
fn unstated_rest_with_finite_peak(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            tail: TailSamples::Finite(5),
            tail_every_peak: TailSamples::Finite(10),
            rest: RestBound::Unstated,
            composition: CompositionBound::Unstated,
        },
    )
}

/// Breaks only (c): a bounded rest beside an infinite `tail_every_peak`.
fn bounded_rest_with_infinite_peak(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            tail: TailSamples::Finite(5),
            tail_every_peak: TailSamples::Infinite,
            rest: REST,
            composition: CompositionBound::Unstated,
        },
    )
}

/// Breaks only (d): a bounded rest whose bound for a +24 dBFS peak exceeds its bound for every
/// sanitized input.
fn peak_rest_above_any_input_rest(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            tail: TailSamples::Finite(5),
            tail_every_peak: TailSamples::Finite(10),
            rest: RestBound::Bounded(RestSamples {
                peak_plus_24_dbfs: 41,
                any_sanitized_input: 40,
            }),
            composition: CompositionBound::Unstated,
        },
    )
}

/// A consistent composition (#1464): the tail gain below the peak gain.
const COMPOSITION: CompositionBound = CompositionBound::Stated {
    decay: TailDecay(30),
    peak_gain: PeakGain::Millibels(-100),
    tail_gain: PeakGain::Millibels(-200),
    stall: FlushStall::Level(-14_000),
};

/// [`consistent`] with a consistent stated composition, at every rate.
fn consistent_stated(_: u32, _: EffectQuality) -> NodeTailBound {
    NodeTailBound {
        composition: COMPOSITION,
        ..consistent(0, EffectQuality::Normal)
    }
}

/// Breaks only (e): a stated composition beside an infinite tail (the other rules hold: both
/// tails infinite, the rest unstated).
fn stated_composition_with_infinite_tail(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            tail: TailSamples::Infinite,
            tail_every_peak: TailSamples::Infinite,
            rest: RestBound::Unstated,
            composition: COMPOSITION,
        },
    )
}

/// Breaks only (f): a tail gain one millibel above the peak gain.
fn tail_gain_above_peak_gain(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            composition: CompositionBound::Stated {
                decay: TailDecay(30),
                peak_gain: PeakGain::Millibels(-100),
                tail_gain: PeakGain::Millibels(-99),
                stall: FlushStall::Zero,
            },
            ..consistent(sample_rate, EffectQuality::Normal)
        },
    )
}

/// Breaks only (f): a tail gain above a `Zero` peak gain (`Zero` is below every `Millibels`).
fn tail_gain_above_zero_peak_gain(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    at_broken_rate(
        sample_rate,
        NodeTailBound {
            composition: CompositionBound::Stated {
                decay: TailDecay(0),
                peak_gain: PeakGain::Zero,
                tail_gain: PeakGain::Millibels(i32::MIN),
                stall: FlushStall::Zero,
            },
            ..consistent(sample_rate, EffectQuality::Normal)
        },
    )
}

/// No bound stated at all: an infinite tail with an unstated composition.
fn unbounded(_: u32, _: EffectQuality) -> NodeTailBound {
    NodeTailBound::UNBOUNDED
}

static CONSISTENT: EffectDescriptor = descriptor(1, consistent);
static UNBOUNDED: EffectDescriptor = descriptor(1, unbounded);
static CONSISTENT_STATED: EffectDescriptor = descriptor(1, consistent_stated);
static RULE_E: EffectDescriptor = descriptor(1, stated_composition_with_infinite_tail);
static RULE_F: EffectDescriptor = descriptor(1, tail_gain_above_peak_gain);
static RULE_F_ZERO: EffectDescriptor = descriptor(1, tail_gain_above_zero_peak_gain);
static RULE_A: EffectDescriptor = descriptor(1, finite_peak_below_tail);
static RULE_A_INFINITE: EffectDescriptor = descriptor(1, finite_peak_below_infinite_tail);
static RULE_B: EffectDescriptor = descriptor(1, unstated_rest_with_finite_peak);
static RULE_C: EffectDescriptor = descriptor(1, bounded_rest_with_infinite_peak);
static RULE_D: EffectDescriptor = descriptor(1, peak_rest_above_any_input_rest);

/// `descriptor` is refused with the D2 code, while its consistent twin (every row as at the
/// other rates) is admitted, so the refusal is the broken row's.
fn assert_refused_as_inconsistent(descriptor: &'static EffectDescriptor) {
    assert!(
        registry(&CONSISTENT).is_ok(),
        "the consistent twin is admitted"
    );
    assert_eq!(
        registry(descriptor).map(|_| ()),
        Err(RegistryError {
            code: "effect.tail_bound.inconsistent",
            id: Some(EFFECT_ID),
        })
    );
}

/// Issue #1462 D2 (a): `tail_every_peak >= tail`, with `Infinite` the largest, at every rate.
///
/// Red mutation: the registry drops rule (a), or checks only the first quality row.
#[test]
fn a_tail_over_every_peak_below_the_tail_is_refused() {
    assert_refused_as_inconsistent(&RULE_A);
    assert_refused_as_inconsistent(&RULE_A_INFINITE);
}

/// Issue #1462 D2 (b): `rest` is `Unstated` only with an infinite `tail_every_peak`.
///
/// Red mutation: the registry drops rule (b), or checks only the first quality row.
#[test]
fn an_unstated_rest_with_a_finite_tail_over_every_peak_is_refused() {
    assert_refused_as_inconsistent(&RULE_B);
}

/// Issue #1462 D2 (c): `rest` is `Bounded` only with a finite `tail_every_peak`.
///
/// Red mutation: the registry drops rule (c), or checks only the first quality row.
#[test]
fn a_bounded_rest_with_an_infinite_tail_over_every_peak_is_refused() {
    assert_refused_as_inconsistent(&RULE_C);
}

/// Issue #1462 D2 (d): in a bounded rest, `peak_plus_24_dbfs <= any_sanitized_input`.
///
/// Red mutation: the registry drops rule (d), or checks only the first quality row.
#[test]
fn a_rest_for_a_limited_peak_above_the_rest_for_any_input_is_refused() {
    assert_refused_as_inconsistent(&RULE_D);
}

/// Issue #1464 K3 (e): a stated composition needs a finite tail. The same values beside a finite
/// tail, and an unstated composition beside an infinite tail, are admitted.
///
/// Red mutation: the registry drops rule (e), or checks only the first quality row.
#[test]
fn a_stated_composition_with_an_infinite_tail_is_refused() {
    assert!(
        registry(&CONSISTENT_STATED).is_ok(),
        "the finite-tail twin with the same composition is admitted"
    );
    assert!(
        registry(&UNBOUNDED).is_ok(),
        "an infinite tail with no composition is admitted"
    );
    assert_refused_as_inconsistent(&RULE_E);
}

/// Issue #1464 K3 (f): `tail_gain <= peak_gain`, with `Zero` below every `Millibels`.
///
/// Red mutation: the registry drops rule (f), compares the two gains the wrong way round, orders
/// `Zero` above a `Millibels`, or checks only the first quality row.
#[test]
fn a_tail_gain_above_the_peak_gain_is_refused() {
    assert!(
        registry(&CONSISTENT_STATED).is_ok(),
        "a tail gain below the peak gain is admitted"
    );
    assert_refused_as_inconsistent(&RULE_F);
    assert_refused_as_inconsistent(&RULE_F_ZERO);
}
