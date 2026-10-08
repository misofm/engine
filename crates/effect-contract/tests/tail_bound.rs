#![allow(missing_docs)]

//! #1377 D3 through #1462's table: the prepared metadata carries the registry's tail-bound entry
//! for the request's own rate and quality, and refuses another row's or another effect's entry.
//! #1464 K4: `NodeTailBound::max` keeps the componentwise rule for the composition, each stall by
//! its own maximum (#1484 S3). #1484 S1: the composition's readers pair each stall with its clause.

mod support;

use effect_contract::*;
use engine::LAUNCH_SAMPLE_RATES;

/// Distinct values per rate in every field, so a copy from the wrong field or the wrong rate
/// shows. The composition is stated (#1464), consistently: a finite tail, the tail gain below the
/// peak gain, the tail stall below the peak stall.
fn tail_and_rest(sample_rate: u32, _: EffectQuality) -> NodeTailBound {
    let rate = u64::from(sample_rate);
    let millibels = |value: u64| i32::try_from(value).expect("a test level fits i32");
    NodeTailBound {
        tail: TailSamples::Finite(rate / 1000),
        tail_every_peak: TailSamples::Finite(rate / 100),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: rate / 10,
            any_sanitized_input: rate,
        }),
        composition: CompositionBound::Stated {
            decay: TailDecay(rate / 20),
            peak_gain: PeakGain::Millibels(millibels(rate / 50)),
            tail_gain: PeakGain::Millibels(millibels(rate / 200)),
            peak_stall: FlushStall::Level(-millibels(rate / 40)),
            tail_stall: FlushStall::Level(-millibels(rate / 30)),
        },
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(id) => id,
        Err(_) => panic!("valid test port ID"),
    }
}

const fn quality(quality: EffectQuality, sample_rate: u32) -> QualityDescriptor {
    QualityDescriptor {
        quality,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: 0,
            right_bytes: 0,
        },
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}

static PORTS: [PortDescriptor; 2] = [
    PortDescriptor {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];

static QUALITIES: [QualityDescriptor; 8] = [
    quality(EffectQuality::Normal, 44_100),
    quality(EffectQuality::Normal, 48_000),
    quality(EffectQuality::Normal, 88_200),
    quality(EffectQuality::Normal, 96_000),
    quality(EffectQuality::High, 44_100),
    quality(EffectQuality::High, 48_000),
    quality(EffectQuality::High, 88_200),
    quality(EffectQuality::High, 96_000),
];

const fn descriptor(id: &'static str) -> EffectDescriptor {
    EffectDescriptor {
        id: match EffectId::new(id) {
            Ok(id) => id,
            Err(_) => panic!("valid test effect ID"),
        },
        display_name: "Tail Bound Test",
        contract_major: 1,
        contract_minor: 0,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::DUAL_MONO,
        parameters: &[],
        ports: &PORTS,
        qualities: &QUALITIES,
        tail_and_rest,
        observations: &[],
    }
}

static DESCRIPTOR: EffectDescriptor = descriptor("tail-bound-test");
static OTHER: EffectDescriptor = descriptor("tail-bound-other");

fn registry() -> NativeEffectRegistry {
    NativeEffectRegistry::new([
        Box::new(support::Factory(&DESCRIPTOR)) as Box<dyn NativeEffectFactory>,
        Box::new(support::Factory(&OTHER)),
    ])
    .expect("valid test descriptors")
}

fn request(sample_rate: u32, tail_bound: RegisteredTailBound) -> PrepareEffectRequest<'static> {
    PrepareEffectRequest {
        sample_rate,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: &[],
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1024,
            maximum_scratch_bytes: 1024,
            maximum_automation_spans_per_block: 8,
        },
        tail_bound,
    }
}

/// #1377 D3, through #1462's table: the prepared metadata, and the program key a cohort is
/// grouped by, carry the four values the descriptor's `tail_and_rest` states for the request's
/// own rate (#1464 K2: `composition` among them).
///
/// Red mutations: the registry evaluates the function at a fixed rate, or
/// `expected_prepared_metadata` copies one field into another, or writes
/// `Unstated`/`Infinite` in place of the stated value (a dropped `composition` among them);
/// `program_key` drops or crosses `tail_every_peak`, `rest` or `composition`.
#[test]
fn prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate() {
    let registry = registry();
    for rate in LAUNCH_SAMPLE_RATES {
        let rate = rate.0;
        let entry = registry
            .tail_bound(DESCRIPTOR.id, rate, EffectQuality::Normal)
            .expect("a declared row");
        let metadata =
            expected_prepared_metadata(&DESCRIPTOR, request(rate, entry)).expect("valid");
        let stated = tail_and_rest(rate, EffectQuality::Normal);
        assert_eq!(metadata.tail, stated.tail, "{rate}");
        assert_eq!(metadata.tail_every_peak, stated.tail_every_peak, "{rate}");
        assert_eq!(metadata.rest, stated.rest, "{rate}");
        assert_eq!(metadata.composition, stated.composition, "{rate}");
        let key = metadata.program_key();
        assert_eq!(key.tail, stated.tail, "{rate}");
        assert_eq!(key.tail_every_peak, stated.tail_every_peak, "{rate}");
        assert_eq!(key.rest, stated.rest, "{rate}");
        assert_eq!(key.composition, stated.composition, "{rate}");
    }
}

/// #1462 D1: a request carrying the table entry of another rate, another quality or another
/// effect is refused, never prepared with that entry's bound; a rate outside the launch set has
/// no entry and is a typed lookup error.
///
/// Red mutations: `expected_prepared_metadata` stops comparing the entry's effect, rate or
/// quality with the request's (each one turns one assertion red); `tail_bound` falls back to
/// some row for an undeclared rate, or ignores the quality key (the High lookup returns the
/// Normal row's entry).
#[test]
fn a_request_with_another_rows_entry_is_refused() {
    let registry = registry();
    let entry = |descriptor: &EffectDescriptor, rate, quality| {
        registry
            .tail_bound(descriptor.id, rate, quality)
            .expect("a declared row")
    };
    let normal_96k = entry(&DESCRIPTOR, 96_000, EffectQuality::Normal);
    let mismatch = Err(EffectPrepareError {
        code: "effect.tail_bound.mismatch",
    });
    // Another rate.
    assert_eq!(
        expected_prepared_metadata(&DESCRIPTOR, request(48_000, normal_96k)).map(|_| ()),
        mismatch
    );
    // Another quality.
    let mut high = request(96_000, normal_96k);
    high.quality = EffectQuality::High;
    assert_eq!(
        expected_prepared_metadata(&DESCRIPTOR, high).map(|_| ()),
        mismatch
    );
    // Another effect.
    let other = entry(&OTHER, 96_000, EffectQuality::Normal);
    assert_eq!(
        expected_prepared_metadata(&DESCRIPTOR, request(96_000, other)).map(|_| ()),
        mismatch
    );
    // The matching entry prepares.
    assert!(expected_prepared_metadata(&DESCRIPTOR, request(96_000, normal_96k)).is_ok());
    // The lookup is keyed by quality too: the High row's entry is the High row's, and a High
    // request with it prepares.
    let high_96k = entry(&DESCRIPTOR, 96_000, EffectQuality::High);
    assert_eq!(high_96k.quality(), EffectQuality::High);
    let mut high = request(96_000, high_96k);
    high.quality = EffectQuality::High;
    assert!(expected_prepared_metadata(&DESCRIPTOR, high).is_ok());
    assert_eq!(
        registry
            .tail_bound(DESCRIPTOR.id, 192_000, EffectQuality::Normal)
            .map_err(|error| error.code),
        Err("effect.quality.unsupported")
    );
}

fn stated(
    decay: u64,
    peak_gain: PeakGain,
    tail_gain: PeakGain,
    [peak_stall, tail_stall]: [FlushStall; 2],
) -> NodeTailBound {
    NodeTailBound {
        composition: CompositionBound::Stated {
            decay: TailDecay(decay),
            peak_gain,
            tail_gain,
            peak_stall,
            tail_stall,
        },
        ..NodeTailBound::ZERO
    }
}

/// #1464 K4: `NodeTailBound::max`, the bound of a node whose two channels are bounded by its
/// operands, is `Unstated` in its composition when either channel's is, and otherwise takes each
/// of the five values from whichever channel's is larger, with `Zero` below every level. The
/// other three values keep #1329's componentwise rule. Every operand is a statement the registry
/// admits. #1484 S3: the crossed operands give each side one larger stall, so each stall is
/// checked by its own maximum in both operand orders.
///
/// Red mutations: `max` keeps one side's `Stated` composition beside an `Unstated` one; takes
/// any one of the composition's five values, a tail or a rest value from one fixed side, or by
/// minimum; takes one stall from the other stall field; or orders `Zero` above a `Millibels` or a
/// `Level`.
#[test]
fn max_states_a_composition_only_when_both_channels_state_one() {
    let left = stated(
        7,
        PeakGain::Millibels(-300),
        PeakGain::Millibels(-900),
        [FlushStall::Level(-14_000), FlushStall::Level(-14_500)],
    );
    let right = stated(
        3,
        PeakGain::Zero,
        PeakGain::Zero,
        [FlushStall::Zero, FlushStall::Zero],
    );
    let larger = stated(
        7,
        PeakGain::Millibels(-300),
        PeakGain::Millibels(-900),
        [FlushStall::Level(-14_000), FlushStall::Level(-14_500)],
    );
    assert_eq!(left.max(right).composition, larger.composition);
    assert_eq!(right.max(left).composition, larger.composition);
    // Each value from the side whose value is larger, mixed across the sides: `crossed` has the
    // larger peak stall and `left` the larger tail stall, so each side supplies one stall.
    let crossed = stated(
        2,
        PeakGain::Millibels(600),
        PeakGain::Millibels(-500),
        [FlushStall::Level(-13_000), FlushStall::Level(-15_000)],
    );
    let mixed = stated(
        7,
        PeakGain::Millibels(600),
        PeakGain::Millibels(-500),
        [FlushStall::Level(-13_000), FlushStall::Level(-14_500)],
    );
    assert_eq!(left.max(crossed).composition, mixed.composition);
    assert_eq!(crossed.max(left).composition, mixed.composition);
    // An unstated side makes the whole composition unstated, on either side.
    let unstated = NodeTailBound::ZERO;
    assert_eq!(unstated.composition, CompositionBound::Unstated);
    assert_eq!(left.max(unstated).composition, CompositionBound::Unstated);
    assert_eq!(unstated.max(left).composition, CompositionBound::Unstated);
    // The other three values keep #1329's componentwise rule: each finite tail and each rest
    // value from the side whose value is larger, mixed across the sides, in both orders.
    let left_channel = NodeTailBound {
        tail: TailSamples::Finite(12),
        tail_every_peak: TailSamples::Finite(40),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: 30,
            any_sanitized_input: 50,
        }),
        ..left
    };
    let right_channel = NodeTailBound {
        tail: TailSamples::Finite(20),
        tail_every_peak: TailSamples::Finite(35),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: 25,
            any_sanitized_input: 60,
        }),
        ..right
    };
    let componentwise = NodeTailBound {
        tail: TailSamples::Finite(20),
        tail_every_peak: TailSamples::Finite(40),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: 30,
            any_sanitized_input: 60,
        }),
        composition: larger.composition,
    };
    assert_eq!(left_channel.max(right_channel), componentwise);
    assert_eq!(right_channel.max(left_channel), componentwise);
    // An infinite tail or an unstated rest on either side absorbs.
    assert_eq!(left.max(NodeTailBound::UNBOUNDED), NodeTailBound::UNBOUNDED);
    assert_eq!(NodeTailBound::UNBOUNDED.max(left), NodeTailBound::UNBOUNDED);
}

/// #1484 S1: the composition's readers pair each stall with its clause, as H1 states: (N1)'s pair
/// is `(G_p, sigma_p)` and (N2)'s triple is `(D, G_t, sigma_t)`; both are `None` for `Unstated`.
/// Every value is distinct, and `sigma_t` lies strictly below `sigma_p`, so a reader that takes
/// any value from another field shows. On the fixed input section the two stalls are equal, so
/// only this fixture tells the readers' stalls apart.
///
/// Red mutations: `peak_clause` returns `sigma_t` (the small stall at every frame) or `G_t`;
/// `tail_clause` returns `sigma_p` (the every-frame stall in `Sigma`) or `G_p`; either returns a
/// value for `Unstated`.
#[test]
fn readers_pair_each_stall_with_its_clause() {
    let composition = stated(
        11,
        PeakGain::Millibels(-200),
        PeakGain::Millibels(-700),
        [FlushStall::Level(-12_000), FlushStall::Level(-14_000)],
    )
    .composition;
    assert_eq!(
        composition.peak_clause(),
        Some((PeakGain::Millibels(-200), FlushStall::Level(-12_000)))
    );
    assert_eq!(
        composition.tail_clause(),
        Some((
            TailDecay(11),
            PeakGain::Millibels(-700),
            FlushStall::Level(-14_000)
        ))
    );
    assert_eq!(CompositionBound::Unstated.peak_clause(), None);
    assert_eq!(CompositionBound::Unstated.tail_clause(), None);
}
