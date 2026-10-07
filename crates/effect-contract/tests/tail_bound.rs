#![allow(missing_docs)]

//! #1377 D3 through #1462's table: the prepared metadata carries the registry's tail-bound entry
//! for the request's own rate and quality, and refuses another row's or another effect's entry.

mod support;

use effect_contract::*;
use engine::LAUNCH_SAMPLE_RATES;

/// Distinct values per rate in every field, so a copy from the wrong field or the wrong rate
/// shows.
fn tail_and_rest(sample_rate: u32, _: EffectQuality) -> EffectTailBound {
    let rate = u64::from(sample_rate);
    EffectTailBound {
        tail: TailSamples::Finite(rate / 1000),
        tail_every_peak: TailSamples::Finite(rate / 100),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: rate / 10,
            any_sanitized_input: rate,
        }),
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
/// grouped by, carry the three values the descriptor's `tail_and_rest` states for the request's
/// own rate.
///
/// Red mutations: the registry evaluates the function at a fixed rate, or
/// `expected_prepared_metadata` copies one field into another, or writes `Unstated`/`Infinite` in
/// place of the stated value; `program_key` drops or crosses `tail_every_peak` or `rest`.
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
        let key = metadata.program_key();
        assert_eq!(key.tail, stated.tail, "{rate}");
        assert_eq!(key.tail_every_peak, stated.tail_every_peak, "{rate}");
        assert_eq!(key.rest, stated.rest, "{rate}");
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
