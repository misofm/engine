//! Issue #2194 gate B (root ruling R1): a prepared launch effect retains at most what its
//! descriptor and factory declare.
//!
//! A session's effect bytes are projected before any instance is prepared
//! (`effect_compiler::project_session_effects`): each instance's declared `maximum_state` and
//! scratch, its factory's `prepared_inline_bytes()` and its `prepared_undeclared_heap_bytes()`. A
//! plan charges a prepared processor its box plus its own heap (`size_of_val + retained_bytes()`).
//! The projection is a ceiling only if every processor fits under it, so this checks every launch
//! effect at every declared quality and launch rate, at several quanta (scratch scales with the
//! quantum), with each sidechain variant it declares and every lattice point of each prepared-only
//! parameter (a prepared parameter may size a buffer).

use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    EffectDescriptor, EffectLaneShape, InitialParameterValue, LinkMode, PortRole,
    PrepareEffectLimits, PrepareEffectRequest, PreparedPorts, PreparedSidechainPort,
    decimal_to_f32, default_initial_values, parameter_lattice_points,
};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTA: [u32; 4] = [1, 128, 1_024, 4_096];

/// The sidechain variants a descriptor admits: none, or an unconnected optional port and a
/// connected one.
fn port_variants(descriptor: &EffectDescriptor) -> Vec<PreparedPorts> {
    let Some(port) = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
    else {
        return vec![PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        }];
    };
    let connected = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: port.id,
            required: port.required,
        },
    };
    if port.required {
        vec![connected]
    } else {
        vec![
            PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required: false,
                },
            },
            connected,
        ]
    }
}

/// The defaults, then for each prepared-only parameter one value set per lattice point, every other
/// value at its default.
fn value_sets(descriptor: &'static EffectDescriptor) -> Vec<Vec<InitialParameterValue>> {
    let defaults: Vec<_> = default_initial_values(descriptor).collect();
    let mut sets = vec![defaults.clone()];
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        if parameter.prepared_reason.is_none() {
            continue;
        }
        for point in parameter_lattice_points(parameter).expect("a lawful lattice") {
            let value = decimal_to_f32(&point.canonical).expect("a convertible point");
            let mut set = defaults.clone();
            for slot in set
                .iter_mut()
                .filter(|slot| slot.parameter_index as usize == index)
            {
                slot.value = value;
            }
            sets.push(set);
        }
    }
    sets
}

/// Test value: red when a launch effect retains more than its declared state and scratch plus its
/// factory's `prepared_inline_bytes()` and `prepared_undeclared_heap_bytes()` (a buffer sized past
/// the declaration, or an inline size naming the wrong type), which would let a session the
/// projection admits allocate past the budget. No other test compares a prepared processor's
/// charge with its declaration.
#[test]
fn every_prepared_launch_effect_fits_its_projected_bytes() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut checked = 0_u32;
    let mut over = Vec::new();
    for descriptor in registry.descriptors() {
        let id = descriptor.id.as_str();
        let factory = registry.get_shared_ascii(id).expect("a registered factory");
        let inline = factory.prepared_inline_bytes();
        let link_mode = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
            .into_iter()
            .find(|mode| descriptor.supported_link_modes.contains(*mode))
            .expect("a supported link mode");
        let automation = u32::try_from(EffectLaneShape::live_parameter_cells(descriptor))
            .expect("a bounded cell count")
            .max(1);
        for quality in descriptor
            .qualities
            .iter()
            .filter(|quality| LAUNCH_RATES.contains(&quality.sample_rate))
        {
            let state = quality.maximum_state.total().expect("a bounded state");
            let undeclared = factory.prepared_undeclared_heap_bytes(quality);
            let tail_bound = registry
                .tail_bound(descriptor.id, quality.sample_rate, quality.quality)
                .expect("a declared row");
            for quantum in QUANTA {
                let scratch = quality.scratch_fixed_bytes
                    + quality.scratch_bytes_per_frame * u64::from(quantum);
                let projected = state + scratch + inline + undeclared;
                for ports in port_variants(descriptor) {
                    for values in value_sets(descriptor) {
                        let prepared = factory
                            .prepare(PrepareEffectRequest {
                                sample_rate: quality.sample_rate,
                                quantum,
                                quality: quality.quality,
                                bypass: false,
                                link_mode,
                                ports,
                                initial_values: &values,
                                limits: PrepareEffectLimits {
                                    maximum_total_state_bytes: state.max(1),
                                    maximum_scratch_bytes: scratch.max(1),
                                    maximum_automation_spans_per_block: automation,
                                },
                                tail_bound,
                            })
                            .unwrap_or_else(|error| {
                                panic!(
                                    "{id} {:?} {} Hz: {}",
                                    quality.quality, quality.sample_rate, error.code
                                )
                            });
                        let processor = &*prepared.processor;
                        let boxed = size_of_val(processor) as u64;
                        let charged = boxed + processor.retained_bytes();
                        if boxed > inline || charged > projected {
                            over.push(format!(
                                "{id} {:?} {} Hz, quantum {quantum}, {:?}: box {boxed} of {inline}, \
                                 charged {charged} of {projected}",
                                quality.quality, quality.sample_rate, ports.sidechain
                            ));
                        }
                        checked += 1;
                    }
                }
            }
        }
    }
    assert!(over.is_empty(), "over the projection:\n{}", over.join("\n"));
    assert!(
        checked >= 11 * 4 * 4,
        "every launch effect at every launch rate and quantum ({checked})"
    );
}
