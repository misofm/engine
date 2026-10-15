//! Decision 15, D15-2 and issue #1345 D4: the staging window holds every live cell.
//!
//! A block stages one span for every parameter cell it reads, in the block that reads it: there is
//! no deferral path and no drop. So a lane needs a window of at least its live parameter cells,
//! and an effect must accept that many spans. This file gates the descriptor-level count that
//! preparation raises each instance's `automation_capacity` to, the lane's refusal of a window
//! that is too short, and a block that applies every cell of a synthetic effect with far more
//! cells than a small capacity and than one dirty word.

#[path = "support/lane_rig.rs"]
mod lane_rig;

use effect_contract::{
    AutomationRate, CompositionBound, EffectControlRecord, EffectDescriptor, EffectId,
    EffectLaneShape, EffectQuality, EnumChoice, LatencySamples, LinkModeSet, NodeTailBound,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterId,
    ParameterLattice, ParameterMapping, ParameterUnit, PortDescriptor, PortId, PortLayout,
    PortRole, QualityDescriptor, RestBound, SmoothingRule, StatePayloadSizes, TailSamples,
};
use lane_rig::{Rig, parameter};

/// `PerLane` `Block` parameters.
const PER_LANE: usize = 40;
/// `Shared` `Block` parameters.
const SHARED: usize = 5;
/// Parameters that are not live: a `None`-rate one and a `Block`-rate one that is not
/// automatable.
const NOT_LIVE: usize = 2;
const TOTAL: usize = PER_LANE + SHARED + NOT_LIVE;
/// The least window the synthetic effect needs: two cells per `PerLane` parameter, one per
/// `Shared` one.
const CELLS: usize = 2 * PER_LANE + SHARED;

const fn parameter_descriptor(
    id: u32,
    rate: AutomationRate,
    policy: ParameterChannelPolicy,
    automatable: bool,
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: ParameterId(id),
        display_name: "Synthetic",
        name: "synthetic_db",
        display_unit: "dB",
        unit: ParameterUnit::Db,
        domain: ParameterDomain::Continuous,
        minimum: Some(-24.0),
        maximum: Some(24.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: rate,
        channel_policy: policy,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable,
        enum_choices: &[] as &[EnumChoice],
        lattice: ParameterLattice::arithmetic(0.1, 1),
        prepared_reason: None,
    }
}

const PARAMETERS: [ParameterDescriptor; TOTAL] = {
    let placeholder = parameter_descriptor(
        0,
        AutomationRate::Block,
        ParameterChannelPolicy::Shared,
        true,
    );
    let mut table = [placeholder; TOTAL];
    let mut index = 0;
    while index < TOTAL {
        // Interleave the three kinds so the live parameters are not one contiguous run: the
        // shape must skip the others and keep the live ones' order.
        let id = index as u32 + 1;
        table[index] = if index < NOT_LIVE {
            match index {
                0 => parameter_descriptor(
                    id,
                    AutomationRate::None,
                    ParameterChannelPolicy::Shared,
                    false,
                ),
                _ => parameter_descriptor(
                    id,
                    AutomationRate::Block,
                    ParameterChannelPolicy::Shared,
                    false,
                ),
            }
        } else if (index - NOT_LIVE).is_multiple_of(9) {
            parameter_descriptor(
                id,
                AutomationRate::Block,
                ParameterChannelPolicy::Shared,
                true,
            )
        } else {
            parameter_descriptor(
                id,
                AutomationRate::Block,
                ParameterChannelPolicy::PerLane,
                true,
            )
        };
        index += 1;
    }
    table
};

const MAIN_IN: PortId = match PortId::new("main-in") {
    Ok(value) => value,
    Err(_) => panic!("valid port ID"),
};
const MAIN_OUT: PortId = match PortId::new("main-out") {
    Ok(value) => value,
    Err(_) => panic!("valid port ID"),
};
const PORTS: [PortDescriptor; 2] = [
    PortDescriptor {
        id: MAIN_IN,
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: MAIN_OUT,
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];
const QUALITIES: [QualityDescriptor; 1] = [QualityDescriptor {
    quality: EffectQuality::Normal,
    sample_rate: 48_000,
    latency: LatencySamples(0),
    maximum_state: StatePayloadSizes {
        common_bytes: 0,
        left_bytes: 0,
        right_bytes: 0,
    },
    scratch_fixed_bytes: 0,
    scratch_bytes_per_frame: 0,
}];

fn tail(_: u32, _: EffectQuality) -> NodeTailBound {
    NodeTailBound {
        tail: TailSamples::Finite(0),
        tail_every_peak: TailSamples::Infinite,
        rest: RestBound::Unstated,
        composition: CompositionBound::Unstated,
    }
}

const ID: EffectId = match EffectId::new("synthetic-cells") {
    Ok(value) => value,
    Err(_) => panic!("valid effect ID"),
};

static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: ID,
    display_name: "Synthetic Cells",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest: tail,
    observations: &[],
};

/// The live parameter cell count is one per `Shared` `Block`-rate parameter and two per `PerLane`
/// one, whatever the others are: it is what preparation raises an instance's capacity to.
///
/// Test value: a count that includes a `None`-rate or non-automatable parameter,
/// or one channel of a `PerLane` pair (an effect would then refuse the twin of a staged span).
#[test]
fn the_live_cell_count_is_one_per_shared_and_two_per_per_lane_block_parameter() {
    assert_eq!(EffectLaneShape::live_parameter_cells(&DESCRIPTOR), CELLS);
    // A prepared-target owner keeps the descriptor's count (its capacity does not depend on
    // whether a lane is attached), and its lane holds no parameter cell.
    let owner = EffectLaneShape::from_descriptor(&DESCRIPTOR, 6);
    assert_eq!((owner.parameter_cells(), owner.target_slots()), (0, 6));
    let plain = EffectLaneShape::from_descriptor(&DESCRIPTOR, 0);
    assert_eq!((plain.parameter_cells(), plain.target_slots()), (CELLS, 0));
}

/// A window shorter than the lane's live cells is refused at bind, never discovered on the render
/// thread: the lane reports how many spans it needs.
///
/// Test value: red if `check_window` accepts a short window (a cell would then be dropped or
/// deferred in a block). That preparation raises each instance's window to its live cells is held
/// through preparation by effect-compiler's `native_session` test
/// `every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells`, which also writes every
/// cell of each prepared launch lane in one transaction and stages them in one block.
#[test]
fn a_window_shorter_than_the_live_cells_is_refused_with_the_count_it_needs() {
    let rig = Rig::new(EffectLaneShape::from_descriptor(&DESCRIPTOR, 0), 0);
    assert_eq!(rig.lane.parameter_cells(), CELLS);
    assert_eq!(rig.lane.check_window(4), Err(CELLS));
    assert_eq!(rig.lane.check_window(CELLS - 1), Err(CELLS));
    assert_eq!(rig.lane.check_window(CELLS), Ok(()));
    assert_eq!(rig.lane.check_window(CELLS + 40), Ok(()));
}

/// One transaction writes every live cell of an effect with far more cells than a small automation
/// capacity (and more than one dirty word of 64), and one block applies all of them, in canonical
/// order, with none deferred or dropped.
///
/// The lane here has the window `check_window` demands, its cell count, built directly: this test
/// does not go through preparation, so it does not hold the window's sizing (that is the
/// preparation-level test named above).
///
/// Test value: red if the lane's dirty bookkeeping aliases or loses a cell past the first 64, or
/// if a block stages the cells out of canonical order or applies one twice. No other test drives
/// more cells than one dirty word through a block.
#[test]
fn one_block_applies_every_live_cell_of_one_transaction() {
    let shape = EffectLaneShape::from_descriptor(&DESCRIPTOR, 0);
    // The synthetic effect spans more than one dirty word.
    const { assert!(CELLS > 64) };
    let mut rig = Rig::new(shape, 0);
    let mut records = Vec::new();
    for (index, descriptor) in PARAMETERS.iter().enumerate() {
        if !(descriptor.automatable && descriptor.automation_rate == AutomationRate::Block) {
            continue;
        }
        let value = index as f32 * 0.5;
        match descriptor.channel_policy {
            ParameterChannelPolicy::Shared => {
                records.push(parameter(index as u32, ParameterChannel::Both, value));
            }
            ParameterChannelPolicy::PerLane => {
                // The two channels differ, so no pair collapses them into fewer spans.
                records.push(parameter(
                    index as u32,
                    ParameterChannel::Right,
                    value + 0.25,
                ));
                records.push(parameter(index as u32, ParameterChannel::Left, value));
            }
        }
    }
    assert_eq!(records.len(), CELLS);
    rig.commit(&records).expect("every record has a cell");
    let staged = rig.stage();
    assert_eq!(
        staged.staged, CELLS,
        "every cell staged in the block that read it"
    );
    let keys: Vec<(u32, u32)> = rig.staging[..staged.staged]
        .iter()
        .map(|span| (span.parameter_index, span.channel as u32))
        .collect();
    let mut sorted = keys.clone();
    sorted.sort_unstable();
    assert_eq!(keys, sorted, "canonical (parameter_index, channel) order");
    sorted.dedup();
    assert_eq!(sorted.len(), CELLS, "no cell twice");
    for record in &records {
        let EffectControlRecord::Parameter {
            parameter_index,
            channel,
            value,
        } = *record
        else {
            unreachable!("only parameter records");
        };
        let span = rig.staging[..staged.staged]
            .iter()
            .find(|span| span.parameter_index == parameter_index && span.channel == channel)
            .expect("a span for every record");
        assert_eq!(span.start_value.to_bits(), value.to_bits());
    }
    assert_eq!(rig.superseded.get(), 0);
    assert_eq!(rig.stage().staged, 0, "and none is left for a later block");
    assert!(!rig.live(), "two differing channels per PerLane parameter");
}
