//! Off-render preparation and bounded validation of parametric-EQ targets.
//!
//! The companion consumes the complete, canonical 60-entry candidate used by the existing EQ
//! preparation contract. It owns the mapping from descriptor rows to physical cascade sections,
//! designs each final lane configuration once, and emits only touched section targets. The
//! numerical validator deliberately does not redesign supplied words: it proves shape, canonical
//! semantic contents, finiteness, mix bounds, and stability, while the Rust preparer remains the
//! authority for semantic-to-coefficient equivalence.

use effect_contract::{
    EffectTargetError, EffectTargetRequest, InitialParameterValue, NativeEffectTargetPreparation,
    ParameterChannel, PreparedEffectTarget, canonical_bits, is_negative_zero,
    parameter_value_valid,
};
use engine::{SampleRateHz, is_launch_sample_rate};

use super::{
    BandTarget, EQ_BAND_COUNT, EQ_SECTION_COUNT, EqBandKind, EqDesignError, EqSvfWords,
    IDENTITY_WORD_BITS, PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory, validate_rounded_svf,
};

const PARAMETER_COUNT: usize = EQ_BAND_COUNT * 6 + 6;
const VALUE_COUNT: usize = PARAMETER_COUNT * 2;
const MAXIMUM_TARGETS: usize = EQ_SECTION_COUNT * 2;
const SEMANTIC_WORDS: usize = 6;

/// Decodes and validates one prepared EQ target at its render boundary.
///
/// The factory's preparation path proves the semantic-to-coefficient relationship off the render
/// thread. The render-side hook repeats only bounded record-shape, canonical and numerical checks
/// it can establish from the self-contained record; it never invokes descriptor validation or the
/// trigonometric designer.
pub(crate) fn decode_prepared_target(
    target: &PreparedEffectTarget,
) -> Result<(usize, ParameterChannel, BandTarget, EqSvfWords), EffectTargetError> {
    let slot = target.slot as usize;
    if slot >= EQ_SECTION_COUNT {
        return Err(EffectTargetError::Shape);
    }
    let words = &target.words;
    let enabled = match words[0] {
        0 => false,
        1 => true,
        _ => return Err(EffectTargetError::Domain),
    };
    let kind = match words[1] {
        1 => EqBandKind::Bell,
        2 => EqBandKind::LowShelf,
        3 => EqBandKind::HighShelf,
        4 => EqBandKind::LowPass,
        5 => EqBandKind::HighPass,
        6 => EqBandKind::Notch,
        _ => return Err(EffectTargetError::Domain),
    };
    let frequency = f32::from_bits(words[2]);
    let gain = f32::from_bits(words[3]);
    let q = f32::from_bits(words[4]);
    let slope = f32::from_bits(words[5]);
    let target_band = BandTarget {
        enabled,
        kind,
        frequency,
        gain,
        q,
        slope,
    };
    if slot == 0 && kind != EqBandKind::HighPass {
        return Err(EffectTargetError::Domain);
    }
    if slot == 5 && kind != EqBandKind::LowPass {
        return Err(EffectTargetError::Domain);
    }
    if (slot == 0 || slot == 5)
        && (gain.to_bits() != 0.0_f32.to_bits() || slope.to_bits() != 1.0_f32.to_bits())
    {
        return Err(EffectTargetError::Domain);
    }
    if is_negative_zero(frequency)
        || is_negative_zero(gain)
        || is_negative_zero(q)
        || is_negative_zero(slope)
        || !frequency.is_finite()
        || !gain.is_finite()
        || !q.is_finite()
        || !slope.is_finite()
    {
        return Err(EffectTargetError::Domain);
    }
    let coefficient_words = EqSvfWords::from_array(core::array::from_fn(|index| {
        f32::from_bits(words[SEMANTIC_WORDS + index])
    }));
    validate_target_coefficients(slot, target_band, &words[SEMANTIC_WORDS..])?;
    Ok((slot, target.channel, target_band, coefficient_words))
}

/// One final section configuration and the words designed from it.
#[derive(Clone, Copy)]
struct PreparedSection {
    target: BandTarget,
    words: EqSvfWords,
}

const fn empty_band() -> BandTarget {
    BandTarget {
        enabled: false,
        kind: EqBandKind::Bell,
        frequency: 0.0,
        gain: 0.0,
        q: 0.0,
        slope: 0.0,
    }
}

const fn empty_target() -> PreparedEffectTarget {
    PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Left,
        words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
    }
}

fn design_error(error: EqDesignError) -> EffectTargetError {
    match error {
        EqDesignError::InvalidInput => EffectTargetError::Domain,
        EqDesignError::Coefficients => EffectTargetError::Coefficients,
    }
}

fn validate_request(request: EffectTargetRequest<'_>) -> Result<(), EffectTargetError> {
    if request.values.len() != VALUE_COUNT || request.changed.len() != VALUE_COUNT {
        return Err(EffectTargetError::Shape);
    }
    if !is_launch_sample_rate(SampleRateHz(request.sample_rate)) {
        return Err(EffectTargetError::Domain);
    }
    for (index, value) in request.values.iter().enumerate() {
        let parameter_index = index / 2;
        let channel = if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        };
        if value.parameter_index != parameter_index as u32 || value.channel != channel {
            return Err(EffectTargetError::Shape);
        }
        let parameter = PARAMETRIC_EQ_DESCRIPTOR
            .parameters
            .get(parameter_index)
            .ok_or(EffectTargetError::Shape)?;
        if is_negative_zero(value.value) || !parameter_value_valid(parameter, value.value) {
            return Err(EffectTargetError::Domain);
        }
    }
    Ok(())
}

fn row(values: &[InitialParameterValue], parameter: usize, lane: usize) -> f32 {
    values[parameter * 2 + lane].value
}

/// Decodes one lane's complete semantic section table without designing coefficients.
fn semantic_sections(
    values: &[InitialParameterValue],
    lane: usize,
) -> Result<[BandTarget; EQ_SECTION_COUNT], EffectTargetError> {
    let mut sections = [empty_band(); EQ_SECTION_COUNT];
    for (slot, section) in sections.iter_mut().enumerate() {
        let (base, kind, general) = match slot {
            0 => (EQ_BAND_COUNT * 6, EqBandKind::HighPass, false),
            5 => (EQ_BAND_COUNT * 6 + 3, EqBandKind::LowPass, false),
            _ => ((slot - 1) * 6, EqBandKind::Bell, true),
        };
        if general {
            *section = BandTarget {
                enabled: canonical_bits(row(values, base, lane)) == canonical_bits(1.0),
                kind: EqBandKind::from_value(row(values, base + 1, lane))
                    .ok_or(EffectTargetError::Domain)?,
                frequency: row(values, base + 2, lane),
                gain: row(values, base + 3, lane),
                q: row(values, base + 4, lane),
                slope: row(values, base + 5, lane),
            };
        } else {
            *section = BandTarget {
                enabled: canonical_bits(row(values, base, lane)) == canonical_bits(1.0),
                kind,
                frequency: row(values, base + 1, lane),
                gain: 0.0,
                q: row(values, base + 2, lane),
                slope: 1.0,
            };
        }
    }
    Ok(sections)
}

/// Decodes and designs every final section exactly once per lane.
fn prepared_sections(
    values: &[InitialParameterValue],
    sample_rate: SampleRateHz,
    touched: &[[bool; 2]; EQ_SECTION_COUNT],
) -> Result<[[PreparedSection; EQ_SECTION_COUNT]; 2], EffectTargetError> {
    let mut sections = [[PreparedSection {
        target: empty_band(),
        words: EqSvfWords::IDENTITY,
    }; EQ_SECTION_COUNT]; 2];
    for (lane, destination) in sections.iter_mut().enumerate() {
        let semantic = semantic_sections(values, lane)?;
        for (slot, section) in destination.iter_mut().enumerate() {
            let target = semantic[slot];
            // Complete semantic candidates are decoded and checked above, while the designer is
            // invoked only for touched final section/lane pairs. Untouched sections never
            // produce a target and therefore do not pay a redundant off-thread design.
            let words = if touched[slot][lane] {
                target.words(sample_rate).map_err(design_error)?
            } else {
                EqSvfWords::IDENTITY
            };
            *section = PreparedSection { target, words };
        }
    }
    Ok(sections)
}

/// Whether one section has any touched descriptor row on one lane.
fn section_touched(changed: &[bool], slot: usize, lane: usize) -> bool {
    let (base, fields) = match slot {
        0 => (EQ_BAND_COUNT * 6, 3),
        5 => (EQ_BAND_COUNT * 6 + 3, 3),
        _ => ((slot - 1) * 6, 6),
    };
    (0..fields).any(|field| changed[(base + field) * 2 + lane])
}

fn touched_sections(changed: &[bool]) -> [[bool; 2]; EQ_SECTION_COUNT] {
    core::array::from_fn(|slot| core::array::from_fn(|lane| section_touched(changed, slot, lane)))
}

fn words_same_bits(left: EqSvfWords, right: EqSvfWords) -> bool {
    left.to_array()
        .into_iter()
        .zip(right.to_array())
        .all(|(a, b)| a.to_bits() == b.to_bits())
}

fn sections_can_be_both(left: PreparedSection, right: PreparedSection) -> bool {
    left.target.same_bits(&right.target) && words_same_bits(left.words, right.words)
}

fn semantic_words(target: BandTarget) -> [u32; SEMANTIC_WORDS] {
    [
        u32::from(target.enabled),
        target.kind as u32,
        target.frequency.to_bits(),
        target.gain.to_bits(),
        target.q.to_bits(),
        target.slope.to_bits(),
    ]
}

fn target_words(section: PreparedSection) -> [u32; effect_contract::PREPARED_EFFECT_TARGET_WORDS] {
    let semantic = semantic_words(section.target);
    let coefficients = section.words.to_array();
    let mut words = [0_u32; effect_contract::PREPARED_EFFECT_TARGET_WORDS];
    words[..SEMANTIC_WORDS].copy_from_slice(&semantic);
    for (destination, value) in words[SEMANTIC_WORDS..].iter_mut().zip(coefficients) {
        *destination = value.to_bits();
    }
    words
}

fn push_target(
    staged: &mut [PreparedEffectTarget; MAXIMUM_TARGETS],
    count: &mut usize,
    slot: usize,
    channel: ParameterChannel,
    section: PreparedSection,
) {
    staged[*count] = PreparedEffectTarget {
        slot: slot as u32,
        channel,
        words: target_words(section),
    };
    *count += 1;
}

fn fill_targets(
    sections: &[[PreparedSection; EQ_SECTION_COUNT]; 2],
    touched: &[[bool; 2]; EQ_SECTION_COUNT],
) -> ([PreparedEffectTarget; MAXIMUM_TARGETS], usize) {
    let mut staged = [empty_target(); MAXIMUM_TARGETS];
    let mut count = 0;
    for slot in 0..EQ_SECTION_COUNT {
        let left = touched[slot][0];
        let right = touched[slot][1];
        if left && right && sections_can_be_both(sections[0][slot], sections[1][slot]) {
            push_target(
                &mut staged,
                &mut count,
                slot,
                ParameterChannel::Both,
                sections[0][slot],
            );
        } else {
            if left {
                push_target(
                    &mut staged,
                    &mut count,
                    slot,
                    ParameterChannel::Left,
                    sections[0][slot],
                );
            }
            if right {
                push_target(
                    &mut staged,
                    &mut count,
                    slot,
                    ParameterChannel::Right,
                    sections[1][slot],
                );
            }
        }
    }
    (staged, count)
}

fn validate_target_coefficients(
    slot: usize,
    section: BandTarget,
    bits: &[u32],
) -> Result<(), EffectTargetError> {
    let words = EqSvfWords::from_array(core::array::from_fn(|index| f32::from_bits(bits[index])));
    if !section.enabled {
        if words
            .to_array()
            .into_iter()
            .map(f32::to_bits)
            .eq(IDENTITY_WORD_BITS)
        {
            return Ok(());
        }
        return Err(EffectTargetError::Coefficients);
    }
    if slot == 0 {
        let expected_m1 = {
            let rounded = (-1.0 / f64::from(section.q)) as f32;
            if rounded == 0.0 { 0.0 } else { rounded }
        };
        if words.m0.to_bits() != 1.0_f32.to_bits()
            || words.m1.to_bits() != expected_m1.to_bits()
            || words.m2.to_bits() != (-1.0_f32).to_bits()
        {
            return Err(EffectTargetError::Coefficients);
        }
    } else if slot == 5
        && (words.m0.to_bits() != 0.0_f32.to_bits()
            || words.m1.to_bits() != 0.0_f32.to_bits()
            || words.m2.to_bits() != 1.0_f32.to_bits())
    {
        return Err(EffectTargetError::Coefficients);
    }
    validate_rounded_svf(words).map_err(|_| EffectTargetError::Coefficients)
}

fn expected_section(
    sections: &[[BandTarget; EQ_SECTION_COUNT]; 2],
    slot: usize,
    channel: ParameterChannel,
) -> Result<&BandTarget, EffectTargetError> {
    match channel {
        ParameterChannel::Left => Ok(&sections[0][slot]),
        ParameterChannel::Right => Ok(&sections[1][slot]),
        ParameterChannel::Both => {
            if sections[0][slot].same_bits(&sections[1][slot]) {
                Ok(&sections[0][slot])
            } else {
                Err(EffectTargetError::Domain)
            }
        }
    }
}

fn validate_target_semantics(
    target: &PreparedEffectTarget,
    slot: usize,
    section: BandTarget,
) -> Result<(), EffectTargetError> {
    if target.words[..SEMANTIC_WORDS] != semantic_words(section) {
        return Err(EffectTargetError::Domain);
    }
    validate_target_coefficients(slot, section, &target.words[SEMANTIC_WORDS..])
}

/// Validates already-prepared targets without calling the trigonometric designer.
fn validate_prepared_targets(
    request: EffectTargetRequest<'_>,
    targets: &[PreparedEffectTarget],
) -> Result<(), EffectTargetError> {
    validate_request(request)?;
    if targets.len() > MAXIMUM_TARGETS {
        return Err(EffectTargetError::Capacity);
    }
    let sections = [
        semantic_sections(request.values, 0)?,
        semantic_sections(request.values, 1)?,
    ];
    let touched = touched_sections(request.changed);
    let mut seen = [[false; 2]; EQ_SECTION_COUNT];
    let mut prior = None;
    for target in targets {
        let slot = target.slot as usize;
        if slot >= EQ_SECTION_COUNT {
            return Err(EffectTargetError::Shape);
        }
        let channel_order = match target.channel {
            ParameterChannel::Left => 1_u32,
            ParameterChannel::Right => 2_u32,
            ParameterChannel::Both => 3_u32,
        };
        let key = (target.slot, channel_order);
        if prior.is_some_and(|previous| key <= previous) {
            return Err(EffectTargetError::Shape);
        }
        prior = Some(key);
        let section = *expected_section(&sections, slot, target.channel)?;
        validate_target_semantics(target, slot, section)?;
        match target.channel {
            ParameterChannel::Left => {
                if !touched[slot][0] || seen[slot][0] {
                    return Err(EffectTargetError::Shape);
                }
                seen[slot][0] = true;
            }
            ParameterChannel::Right => {
                if !touched[slot][1] || seen[slot][1] {
                    return Err(EffectTargetError::Shape);
                }
                seen[slot][1] = true;
            }
            ParameterChannel::Both => {
                if !touched[slot][0] || !touched[slot][1] || seen[slot][0] || seen[slot][1] {
                    return Err(EffectTargetError::Shape);
                }
                seen[slot] = [true; 2];
            }
        }
    }
    for slot in 0..EQ_SECTION_COUNT {
        for lane in 0..2 {
            if touched[slot][lane] != seen[slot][lane] {
                return Err(EffectTargetError::Shape);
            }
        }
    }
    Ok(())
}

impl NativeEffectTargetPreparation for ParametricEqFactory {
    fn maximum_targets(&self) -> usize {
        MAXIMUM_TARGETS
    }

    fn prepare_targets(
        &self,
        request: EffectTargetRequest<'_>,
        out: &mut [PreparedEffectTarget],
    ) -> Result<usize, EffectTargetError> {
        validate_request(request)?;
        let touched = touched_sections(request.changed);
        let sample_rate = SampleRateHz(request.sample_rate);
        let sections = prepared_sections(request.values, sample_rate, &touched)?;
        let (staged, count) = fill_targets(&sections, &touched);
        if out.len() < count {
            return Err(EffectTargetError::Capacity);
        }
        // The generated records are fully validated before this one and only output mutation.
        validate_prepared_targets(request, &staged[..count])?;
        out[..count].copy_from_slice(&staged[..count]);
        Ok(count)
    }

    fn validate_targets(
        &self,
        request: EffectTargetRequest<'_>,
        targets: &[PreparedEffectTarget],
    ) -> Result<(), EffectTargetError> {
        validate_prepared_targets(request, targets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use effect_contract::{EffectTargetRequest, default_initial_values};

    fn defaults() -> Vec<InitialParameterValue> {
        default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect()
    }

    fn request<'a>(
        values: &'a [InitialParameterValue],
        changed: &'a [bool],
    ) -> EffectTargetRequest<'a> {
        EffectTargetRequest {
            sample_rate: 48_000,
            values,
            changed,
        }
    }

    fn prepare(
        values: &[InitialParameterValue],
        changed: &[bool],
        out: &mut [PreparedEffectTarget],
    ) -> Result<usize, EffectTargetError> {
        ParametricEqFactory.prepare_targets(request(values, changed), out)
    }

    #[test]
    fn untouched_candidate_emits_no_targets_at_every_launch_rate() {
        for sample_rate in [44_100, 48_000, 88_200, 96_000] {
            let values = defaults();
            let changed = vec![false; VALUE_COUNT];
            let mut out = [empty_target(); MAXIMUM_TARGETS];
            let result = ParametricEqFactory.prepare_targets(
                EffectTargetRequest {
                    sample_rate,
                    values: &values,
                    changed: &changed,
                },
                &mut out,
            );
            assert_eq!(result, Ok(0));
        }
    }

    #[test]
    fn one_numeric_edit_designs_one_left_physical_section() {
        let mut values = defaults();
        let mut changed = vec![false; VALUE_COUNT];
        values[4].value = 1_000.0;
        changed[4] = true;
        let mut out = [empty_target(); MAXIMUM_TARGETS];
        let count = prepare(&values, &changed, &mut out).expect("target");
        assert_eq!(count, 1);
        assert_eq!(out[0].slot, 1);
        assert_eq!(out[0].channel, ParameterChannel::Left);
        assert_eq!(out[0].words[0], 0);
        assert_eq!(out[0].words[1], EqBandKind::Bell as u32);
        assert_eq!(out[0].words[2], 1_000.0_f32.to_bits());
    }

    #[test]
    fn symmetric_both_edit_coalesces_and_asymmetric_state_stays_dual() {
        let mut values = defaults();
        let mut changed = vec![false; VALUE_COUNT];
        values[4].value = 1_000.0;
        values[5].value = 1_000.0;
        changed[4] = true;
        changed[5] = true;
        let mut out = [empty_target(); MAXIMUM_TARGETS];
        assert_eq!(prepare(&values, &changed, &mut out), Ok(1));
        assert_eq!(out[0].channel, ParameterChannel::Both);

        values[6].value = 6.0;
        changed[6] = true;
        changed[7] = true;
        assert_eq!(prepare(&values, &changed, &mut out), Ok(2));
        assert_eq!(out[0].channel, ParameterChannel::Left);
        assert_eq!(out[1].channel, ParameterChannel::Right);
    }

    #[test]
    fn capacity_and_domain_errors_leave_output_unchanged() {
        let mut values = defaults();
        let mut changed = vec![false; VALUE_COUNT];
        changed[4] = true;
        changed[12] = true;
        let sentinel = PreparedEffectTarget {
            slot: 99,
            channel: ParameterChannel::Right,
            words: [0xdead_beef; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        };
        let mut out = [sentinel; 1];
        assert_eq!(
            prepare(&values, &changed, &mut out),
            Err(EffectTargetError::Capacity)
        );
        assert_eq!(out, [sentinel; 1]);

        values[4].value = 1.0;
        assert_eq!(
            prepare(&values, &changed, &mut [empty_target(); MAXIMUM_TARGETS]),
            Err(EffectTargetError::Domain)
        );
    }

    #[test]
    fn disabled_target_requires_exact_identity_and_hostile_finite_mix_is_rejected() {
        let mut values = defaults();
        let mut changed = vec![false; VALUE_COUNT];
        changed[0] = true;
        let mut out = [empty_target(); MAXIMUM_TARGETS];
        let count = prepare(&values, &changed, &mut out).expect("disabled target");
        assert_eq!(count, 1);
        assert_eq!(&out[0].words[6..], &IDENTITY_WORD_BITS);

        out[0].words[6 + 3] = 129.0_f32.to_bits();
        assert_eq!(
            ParametricEqFactory.validate_targets(request(&values, &changed), &out[..1]),
            Err(EffectTargetError::Coefficients)
        );

        // A disabled semantic target cannot carry a stable non-identity coefficient set either.
        values[0].value = 1.0;
        changed[0] = true;
        let count = prepare(&values, &changed, &mut out).expect("enabled target");
        assert_eq!(count, 1);
        out[0].words[6 + 3] = 129.0_f32.to_bits();
        assert_eq!(
            ParametricEqFactory.validate_targets(request(&values, &changed), &out[..1]),
            Err(EffectTargetError::Coefficients)
        );

        // The dedicated HPF's fixed output mix is part of its contract, even when the alternate
        // words would still satisfy the generic stability predicate.
        let mut cut_values: Vec<InitialParameterValue> =
            default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect();
        let mut cut_changed = vec![false; VALUE_COUNT];
        cut_values[48].value = 1.0;
        cut_changed[48] = true;
        let cut_count = prepare(&cut_values, &cut_changed, &mut out).expect("enabled HPF");
        assert_eq!(cut_count, 1);
        out[0].words[6 + 3] = 0.0_f32.to_bits();
        assert_eq!(
            ParametricEqFactory.validate_targets(request(&cut_values, &cut_changed), &out[..1]),
            Err(EffectTargetError::Coefficients)
        );
    }

    #[test]
    fn enabled_targets_validate_each_kind_at_each_launch_rate() {
        let kinds = [
            EqBandKind::Bell,
            EqBandKind::LowShelf,
            EqBandKind::HighShelf,
            EqBandKind::LowPass,
            EqBandKind::HighPass,
            EqBandKind::Notch,
        ];
        for sample_rate in [44_100, 48_000, 88_200, 96_000] {
            for kind in kinds {
                let mut values = defaults();
                let mut changed = vec![false; VALUE_COUNT];
                for lane in 0..2 {
                    values[lane].value = 1.0;
                    values[2 + lane].value = kind as u32 as f32;
                    values[4 + lane].value = 10.0;
                    values[6 + lane].value = 24.0;
                    values[8 + lane].value = 0.1;
                    values[10 + lane].value = 0.1;
                    for touched in changed.iter_mut().take(12) {
                        *touched = true;
                    }
                }
                let mut out = [empty_target(); MAXIMUM_TARGETS];
                let request = EffectTargetRequest {
                    sample_rate,
                    values: &values,
                    changed: &changed,
                };
                let count = ParametricEqFactory
                    .prepare_targets(request, &mut out)
                    .expect("enabled kind target");
                assert_eq!(count, 1);
                assert_eq!(out[0].slot, 1);
                assert_eq!(out[0].channel, ParameterChannel::Both);
                ParametricEqFactory
                    .validate_targets(request, &out[..count])
                    .expect("validated kind target");
            }
        }
    }

    #[test]
    fn all_sections_and_both_lanes_fill_the_twelve_target_capacity() {
        let mut values = defaults();
        let mut changed = vec![false; VALUE_COUNT];
        for lane in 0..2 {
            for slot in 0..EQ_SECTION_COUNT {
                let (base, fields) = match slot {
                    0 => (EQ_BAND_COUNT * 6, 3),
                    5 => (EQ_BAND_COUNT * 6 + 3, 3),
                    _ => ((slot - 1) * 6, 6),
                };
                values[base * 2 + lane].value = 1.0;
                changed[base * 2 + lane] = true;
                for field in 1..fields {
                    changed[(base + field) * 2 + lane] = true;
                }
            }
        }
        let mut out = [empty_target(); MAXIMUM_TARGETS];
        let count = prepare(&values, &changed, &mut out).expect("all sections");
        assert_eq!(count, EQ_SECTION_COUNT);
        for (slot, target) in out[..count].iter().enumerate() {
            assert_eq!(target.slot, slot as u32);
            assert_eq!(target.channel, ParameterChannel::Both);
        }
        ParametricEqFactory
            .validate_targets(request(&values, &changed), &out[..count])
            .expect("all section targets");

        // Distinct complete lane configurations require twelve records in section/channel order.
        for slot in 1..=4 {
            let gain_parameter = (slot - 1) * 6 + 3;
            values[gain_parameter * 2].value = 6.0;
            values[gain_parameter * 2 + 1].value = -6.0;
            changed[gain_parameter * 2] = true;
            changed[gain_parameter * 2 + 1] = true;
        }
        values[50].value = 20_000.0;
        values[51].value = 19_000.0;
        changed[50] = true;
        changed[51] = true;
        values[56].value = 20_000.0;
        values[57].value = 19_000.0;
        changed[56] = true;
        changed[57] = true;
        let count = prepare(&values, &changed, &mut out).expect("asymmetric sections");
        assert_eq!(count, MAXIMUM_TARGETS);
        for (index, target) in out[..count].iter().enumerate() {
            assert_eq!(target.slot, (index / 2) as u32);
            assert_eq!(
                target.channel,
                if index % 2 == 0 {
                    ParameterChannel::Left
                } else {
                    ParameterChannel::Right
                }
            );
        }
        ParametricEqFactory
            .validate_targets(request(&values, &changed), &out[..count])
            .expect("asymmetric section targets");
    }

    #[test]
    fn malformed_rows_selectors_and_masks_are_rejected_without_writes() {
        let mut values = defaults();
        let mut changed = vec![false; VALUE_COUNT];
        values[4].value = 1_000.0;
        changed[4] = true;
        let mut valid = [empty_target(); MAXIMUM_TARGETS];
        let count = prepare(&values, &changed, &mut valid).expect("valid target");
        assert_eq!(count, 1);

        let mut duplicate = [empty_target(); 2];
        duplicate[0] = valid[0];
        duplicate[1] = valid[0];
        assert_eq!(
            ParametricEqFactory.validate_targets(request(&values, &changed), &duplicate),
            Err(EffectTargetError::Shape)
        );

        let mut missing_values = defaults();
        let mut missing_changed = vec![false; VALUE_COUNT];
        missing_values[4].value = 1_000.0;
        missing_changed[4] = true;
        missing_changed[12] = true;
        assert_eq!(
            ParametricEqFactory
                .validate_targets(request(&missing_values, &missing_changed), &valid[..1]),
            Err(EffectTargetError::Shape)
        );

        let second_values = defaults();
        let mut second_changed = vec![false; VALUE_COUNT];
        second_changed[12] = true;
        let mut second = [empty_target(); MAXIMUM_TARGETS];
        let second_count = prepare(&second_values, &second_changed, &mut second).expect("second");
        assert_eq!(second_count, 1);
        let mut extra_changed = vec![false; VALUE_COUNT];
        extra_changed[4] = true;
        assert_eq!(
            ParametricEqFactory
                .validate_targets(request(&second_values, &extra_changed), &second[..1]),
            Err(EffectTargetError::Shape)
        );

        let mut both_changed = vec![false; VALUE_COUNT];
        both_changed[4] = true;
        both_changed[12] = true;
        let mut ordered = [empty_target(); MAXIMUM_TARGETS];
        let ordered_defaults = defaults();
        let ordered_count =
            prepare(&ordered_defaults, &both_changed, &mut ordered).expect("ordered");
        assert_eq!(ordered_count, 2);
        ordered.swap(0, 1);
        assert_eq!(
            ParametricEqFactory.validate_targets(
                request(&ordered_defaults, &both_changed),
                &ordered[..ordered_count]
            ),
            Err(EffectTargetError::Shape)
        );

        let mut channel_values = defaults();
        channel_values[4].value = 1_000.0;
        channel_values[5].value = 1_000.0;
        let mut channel_changed = vec![false; VALUE_COUNT];
        channel_changed[4] = true;
        let mut channel_target = [empty_target(); 1];
        prepare(&channel_values, &channel_changed, &mut channel_target).expect("channel target");
        let mut wrong_channel = channel_target[0];
        wrong_channel.channel = ParameterChannel::Right;
        assert_eq!(
            ParametricEqFactory
                .validate_targets(request(&channel_values, &channel_changed), &[wrong_channel]),
            Err(EffectTargetError::Shape)
        );

        let mut too_many = [empty_target(); MAXIMUM_TARGETS + 1];
        too_many[..MAXIMUM_TARGETS].copy_from_slice(&valid);
        assert_eq!(
            ParametricEqFactory.validate_targets(request(&values, &changed), &too_many),
            Err(EffectTargetError::Capacity)
        );

        let sentinel = PreparedEffectTarget {
            slot: 77,
            channel: ParameterChannel::Right,
            words: [0xfeed_face; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        };
        let mut output = [sentinel; MAXIMUM_TARGETS];
        let mut invalid_values = defaults();
        invalid_values[4].value = 1.0;
        assert_eq!(
            prepare(&invalid_values, &changed, &mut output),
            Err(EffectTargetError::Domain)
        );
        assert_eq!(output, [sentinel; MAXIMUM_TARGETS]);
        invalid_values = defaults();
        invalid_values[0].parameter_index = 1;
        assert_eq!(
            prepare(&invalid_values, &changed, &mut output),
            Err(EffectTargetError::Shape)
        );
        assert_eq!(output, [sentinel; MAXIMUM_TARGETS]);
        assert_eq!(
            ParametricEqFactory.prepare_targets(
                EffectTargetRequest {
                    sample_rate: 48_000,
                    values: &defaults()[..VALUE_COUNT - 1],
                    changed: &changed,
                },
                &mut output
            ),
            Err(EffectTargetError::Shape)
        );
    }

    #[test]
    fn stable_forgery_with_expected_header_is_accepted_by_design() {
        let mut values = defaults();
        let mut changed = vec![false; VALUE_COUNT];
        values[0].value = 1.0;
        values[6].value = 6.0;
        changed[0] = true;
        changed[6] = true;
        let mut out = [empty_target(); MAXIMUM_TARGETS];
        let count = prepare(&values, &changed, &mut out).expect("target");
        assert_eq!(count, 1);
        let forged = super::super::design_svf(
            EqBandKind::Bell,
            2_000.0,
            6.0,
            core::f32::consts::FRAC_1_SQRT_2,
            1.0,
            SampleRateHz(48_000),
        )
        .expect("stable alternate words");
        let forged_bits = forged.to_array().map(f32::to_bits);
        assert_ne!(&out[0].words[SEMANTIC_WORDS..], &forged_bits);
        for (destination, value) in out[0].words[6..].iter_mut().zip(forged.to_array()) {
            *destination = value.to_bits();
        }
        assert_eq!(
            ParametricEqFactory.validate_targets(request(&values, &changed), &out[..1]),
            Ok(())
        );
    }
}
