#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: the grid law is restated with the platform's ln/exp on purpose
//! Issue #1051: host-core's randomized console differential.
//!
//! One seeded generator of realistic consoles, after graph-compiler's #966 probe: a common strip
//! per rack with dropped and inserted slots or free per-track racks, mono-mapped and stereo tracks,
//! parameters drawn from every launch effect's descriptor (symmetric or channel-asymmetric),
//! bypassed effects, input delays, trims and cuts, compressor and gate sidechains from earlier
//! tracks, and sends from every tap into submixes. Each console is prepared through host-core's
//! own entry points with live controls attached, fed one deterministic source, and rendered with
//! random live-control records at random blocks: input trim and polarity, fader and mute, matrix,
//! effect parameter points (bit-equal `Left`/`Right` twins, or one channel) and live bypass.
//!
//! The oracles, every one by bits:
//!
//! * **armed against forced dual.** host-core arms the mono collapse at prepare
//!   (`PreparedRenderPlan::arm_mono_collapse`); `force_mono_collapse_off(true)` renders every chain
//!   dual. The collapse is a bit-exact optimisation, so the two must agree -- the class #970 was
//!   (the collapse armed on a chain that does not gather the track input). The armed collapse must
//!   fire on some seed, or the comparison proves nothing.
//! * **serialized against concurrent lowering.** `prepare_host_runtime_between_render_calls`
//!   lowers the builtins for a host that admits records only between render calls; it renders the
//!   same bits.
//!
//! And the invariant a differential cannot see: legal input never leaves the D7 block bound.
//!
//! A failing seed prints the command that replays it (`dsp_reference::randomized`).

use core::num::{NonZeroU32, NonZeroUsize};

use builtins::{BuiltinLaneSelector, Matrix2x2, MeterTap};
use builtins_compiler::{TrackControlRecord, TrackFaderRecord, TrackInputRecord};
use dsp_reference::randomized::{Draw, first_difference, run_seeds};
use effect_compiler::{CONSOLE_ELIGIBLE_EFFECTS, launch_native_effect_registry};
use effect_contract::{
    AutomationRate, EffectControlRecord, EffectDescriptor, NativeEffectRegistry,
    ParameterChannelPolicy, ParameterDomain,
};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, PreparedHost,
    ResponseParameterOverride, ResponsePreviewError, ResponsePreviewGrid, ResponsePreviewLimits,
    ResponsePreviewOutput, ResponsePreviewRequest, ResponsePreviewTarget,
    ResponseSnapshotCollector, ResponseSnapshotOutput, ResponseSnapshotQueryError,
    SourceSubmission, compile_host_session, prepare_host_runtime_between_render_calls,
    prepare_host_runtime_with_live_controls, query_response_snapshot_into,
};
use session::{
    ChannelMatrix, ConsoleEntry, ConsoleSlot, Effect, EffectIdentity, EffectParam, LinkMode,
    ParameterChannel, ParameterUnit, Route, RouteDestination, RouteSource, SendTap, SessionModel,
    Sidechain, SidechainDeclaration, StableId, Submix, canonical_session_json, parse_session_json,
};

/// Eight tracks, one banked parametric EQ each as a `console.pre_insert` slot: the strip every
/// generated track starts from, and the source and output every console uses.
const BANK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const QUANTUM: usize = 128;
const BLOCKS: usize = 8;
const TEST: &str = "randomized_consoles_render_the_same_bits_armed_dual_and_serialized";
const REPLAY: &str = "cargo test -p host-core --features host-core/test-support --test randomized \
                      -- --exact randomized_consoles_render_the_same_bits_armed_dual_and_serialized";

const KINDS: [&str; 8] = [
    "miso.parametric-eq",
    "miso.compressor",
    "miso.gate-expander",
    "miso.multiband-compressor",
    "miso.true-peak-limiter",
    "miso.soft-clip",
    "miso.transient-shaper",
    "miso.delay",
];

/// Common strips per rack, as indices into [`KINDS`].
const SIMD1: &[&[usize]] = &[&[], &[0], &[0, 1], &[1], &[2, 0, 1], &[0, 1, 6], &[0, 5]];
const DYNAMIC: &[&[usize]] = &[&[], &[], &[5], &[1], &[0, 5], &[7], &[1, 7]];
const SIMD2: &[&[usize]] = &[&[], &[4], &[4], &[5, 4], &[3, 4], &[0, 4]];

const TAPS: [SendTap; 7] = [
    SendTap::Input,
    SendTap::PostInput,
    SendTap::InsertSend,
    SendTap::InsertReturn,
    SendTap::PreFader,
    SendTap::PostFader,
    SendTap::PostPan,
];

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 400,
        maximum_effects: 400,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 256,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 26,
    }
}

fn live_controls(tap: MeterTap) -> HostLiveControlRequest {
    HostLiveControlRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: tap,
        observation_taps: 0,
        master_track: None,
    }
}

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn unit(unit: effect_contract::ParameterUnit) -> ParameterUnit {
    use effect_contract::ParameterUnit as Contract;
    match unit {
        Contract::Db => ParameterUnit::Db,
        Contract::Hz => ParameterUnit::Hz,
        Contract::Milliseconds => ParameterUnit::Milliseconds,
        Contract::Samples => ParameterUnit::Samples,
        Contract::Linear => ParameterUnit::Linear,
        Contract::Ratio => ParameterUnit::Ratio,
    }
}

/// A legal value of `parameter`, favouring the domain edges.
fn value(draw: &mut Draw, parameter: &effect_contract::ParameterDescriptor) -> f32 {
    let value = match parameter.domain {
        ParameterDomain::Boolean => draw.pick(&[0.0_f32, 1.0]),
        ParameterDomain::Enumeration => draw.pick(parameter.enum_choices).value,
        ParameterDomain::Continuous => draw.in_domain(
            parameter.minimum.unwrap_or(parameter.default_value),
            parameter.maximum.unwrap_or(parameter.default_value),
        ),
    };
    if effect_contract::parameter_value_valid(parameter, value) {
        value
    } else {
        parameter.default_value
    }
}

/// One effect of `kind`, with a third of its parameters drawn: once for both channels, or once
/// per channel -- equal on a symmetric desk, drawn apart on an asymmetric one.
fn effect(
    draw: &mut Draw,
    registry: &NativeEffectRegistry,
    template: &Effect,
    (kind, link): (usize, u64),
    id: &str,
    symmetric: bool,
) -> Effect {
    let descriptor = registry
        .get_ascii(KINDS[kind])
        .expect("a launch effect")
        .descriptor();
    let mut effect = template.clone();
    effect.id = sid(id);
    effect.identity = EffectIdentity::Native {
        effect_id: sid(KINDS[kind]),
    };
    effect.sidechain = SidechainDeclaration::None;
    effect.bypass = link < 3 && draw.chance(1, 32);
    let links: Vec<LinkMode> = [
        (effect_contract::LinkMode::DualMono, LinkMode::DualMono),
        (effect_contract::LinkMode::Maximum, LinkMode::Maximum),
        (effect_contract::LinkMode::Average, LinkMode::Average),
    ]
    .into_iter()
    .filter(|(contract, _)| descriptor.supported_link_modes.contains(*contract))
    .map(|(_, link)| link)
    .collect();
    // The desk draws one link mode per kind, so a strip's slots can share a program and bank.
    effect.link_mode = if link < 3 {
        links[link as usize % links.len()]
    } else {
        LinkMode::DualMono
    };
    effect.params.clear();
    // A hazard desk's asymmetric stage sets every parameter, each channel apart, so that it makes
    // the planes differ audibly rather than by a draw that happened to agree.
    let apart = link == 3 && !symmetric;
    for parameter in descriptor.parameters {
        if !apart && !draw.chance(1, 3) {
            continue;
        }
        let per_channel = parameter.channel_policy == ParameterChannelPolicy::PerLane
            && (!symmetric || draw.chance(1, 2));
        let left = value(draw, parameter);
        if per_channel {
            let mut right = if symmetric {
                left
            } else {
                value(draw, parameter)
            };
            for _ in 0..8 {
                if !apart || right.to_bits() != left.to_bits() {
                    break;
                }
                right = value(draw, parameter);
            }
            for (channel, value) in [
                (ParameterChannel::Left, left),
                (ParameterChannel::Right, right),
            ] {
                effect.params.push(EffectParam {
                    parameter_id: parameter.id.0,
                    channel,
                    unit: unit(parameter.unit),
                    value,
                });
            }
        } else {
            effect.params.push(EffectParam {
                parameter_id: parameter.id.0,
                channel: ParameterChannel::Both,
                unit: unit(parameter.unit),
                value: left,
            });
        }
    }
    effect
}

/// Place generated per-track chains in decision 12's shape, by the rule #1093 migrated the
/// checked-in documents with: a first or third chain that every track declares identically (IDs,
/// identity, quality, link mode), keyless and console-eligible, becomes `console.pre_insert` or
/// `console.post_insert`, with each track's bypass and params in its entries; every other chain
/// folds into the track's inserts in chain order. Chain order is never changed.
fn place(model: &mut SessionModel, chains: Vec<[Vec<Effect>; 3]>) {
    let console = |rack: usize| -> Option<Vec<ConsoleSlot>> {
        let declaration = |effect: &Effect| {
            (
                effect.id.clone(),
                effect.identity.clone(),
                effect.quality,
                effect.link_mode,
            )
        };
        let first: Vec<_> = chains.first()?[rack].iter().map(declaration).collect();
        let uniform = chains.iter().all(|track| {
            track[rack]
                .iter()
                .map(declaration)
                .eq(first.iter().cloned())
                && track[rack].iter().all(|effect| {
                    effect.sidechain == SidechainDeclaration::None
                        && matches!(&effect.identity, EffectIdentity::Native { effect_id }
                            if CONSOLE_ELIGIBLE_EFFECTS.contains(&effect_id.as_str()))
                })
        });
        uniform.then(|| {
            first
                .into_iter()
                .map(|(slot, identity, quality, link_mode)| ConsoleSlot {
                    slot,
                    identity,
                    quality,
                    link_mode,
                })
                .collect()
        })
    };
    let (pre, post) = (console(0), console(2));
    model.console.pre_insert = pre.clone().unwrap_or_default();
    model.console.post_insert = post.clone().unwrap_or_default();
    let entry = |effect: Effect| ConsoleEntry {
        slot: effect.id,
        bypass: effect.bypass,
        params: effect.params,
    };
    for (track, [first, second, third]) in model.tracks.iter_mut().zip(chains) {
        let mut entries = Vec::new();
        let mut inserts = Vec::new();
        if pre.is_some() {
            entries.extend(first.into_iter().map(entry));
        } else {
            inserts.extend(first);
        }
        inserts.extend(second);
        if post.is_some() {
            entries.extend(third.into_iter().map(entry));
        } else {
            inserts.extend(third);
        }
        track.console = entries;
        track.inserts.effects = inserts;
    }
}

fn route(id: &str, source: RouteSource, destination: RouteDestination, gain_db: f32) -> Route {
    Route {
        id: sid(id),
        source,
        destination,
        channel_matrix: ChannelMatrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        },
        gain_db,
    }
}

/// One randomized console, and whether it was drawn in the #970 shape.
///
/// Half the desks are drawn in that shape on purpose: every track mono-mapped, an
/// asymmetric first rack, symmetric later racks, and (in [`probe`]) a meter tap inside the strip,
/// which splits it into chains. The collapse must arm only the chain that gathers the track input;
/// armed on a later one it copies a left plane the asymmetric stage made differ over the right.
#[allow(clippy::too_many_lines)]
fn generate(draw: &mut Draw, registry: &NativeEffectRegistry) -> (SessionModel, bool) {
    let mut model = parse_session_json(BANK).expect("the bank fixture");
    model.sample_rate_hz = draw.pick(&[44_100, 48_000, 48_000, 88_200, 96_000]);
    let mut strip = model.tracks[0].clone();
    let template = model.lower_track(&strip).pre_insert[0].clone();
    strip.console.clear();
    strip.inserts.effects.clear();
    let mut chains = Vec::new();
    model.tracks.clear();
    model.routes.clear();
    model.submixes.clear();
    model.automation.clear();
    let tracks = match draw.below(8) {
        0 => 1,
        1..=5 => 2 + draw.below(9),
        _ => 11 + draw.below(14),
    };
    let hazard = draw.chance(1, 2);
    let tracks = if hazard {
        draw.pick(&[8, 8, 9, 16])
    } else {
        tracks
    };
    let free = !hazard && draw.chance(1, 4);
    let drop = draw.pick(&[0_u64, 30, 80, 200, 400]);
    let mono = if hazard {
        1_000
    } else {
        draw.pick(&[0_u64, 300, 1_000])
    };
    let symmetric = !hazard && draw.chance(1, 2);
    // The hazard strips: banking stages in both SIMD racks, split by nothing, by a per-node delay,
    // or by a banked stage in the dynamic rack.
    let racks = if hazard {
        [
            draw.pick(&[&[0_usize][..], &[0, 1], &[1]]),
            draw.pick(&[&[][..], &[7], &[1]]),
            draw.pick(&[&[0_usize][..], &[1], &[0, 1], &[0, 4]]),
        ]
    } else {
        [draw.pick(SIMD1), draw.pick(DYNAMIC), draw.pick(SIMD2)]
    };
    let links: [u64; KINDS.len()] = core::array::from_fn(|_| draw.below(3) as u64);
    let submixes = draw.below(3);
    let main_out = || RouteDestination::OutputInput {
        output_id: sid("main-out"),
    };
    for submix in 0..submixes {
        model
            .submixes
            .push(Submix::unity(sid(&format!("bus{submix}"))));
        model.routes.push(route(
            &format!("bus{submix}-main"),
            RouteSource::SubmixOutput {
                submix_id: sid(&format!("bus{submix}")),
            },
            main_out(),
            0.0,
        ));
    }
    for index in 0..tracks {
        let id = format!("ch{index:02}");
        let mut track = strip.clone();
        track.id = sid(&id);
        if draw.chance(mono, 1_000) {
            track.left_source_channel = 0;
            track.right_source_channel = 0;
        }
        // A hazard desk keeps its builtins symmetric: an asymmetric input delay alone declines a
        // track's collapse at prepare, and one declined lane unarms its whole bank.
        for (side, builtins) in [&mut track.builtins.left, &mut track.builtins.right]
            .into_iter()
            .enumerate()
        {
            if hazard || (side == 1 && symmetric) {
                break;
            }
            if draw.chance(1, 12) {
                builtins.delay_samples = 1 + draw.below(64) as u32;
            }
            if draw.chance(1, 6) {
                builtins.trim_db = draw.pick(&[-144.0_f32, -12.0, -6.0, 0.0, 3.0, 24.0]);
            }
            if draw.chance(1, 8) {
                builtins.polarity_invert = true;
            }
            if draw.chance(1, 8) {
                builtins.hpf_hz = draw.pick(&[20.0_f32, 80.0, 1_000.0]);
            }
        }
        if symmetric {
            track.builtins.right = track.builtins.left.clone();
        }
        if draw.chance(1, 6) {
            track.fader.left_db = draw.pick(&[-144.0_f32, -24.0, -6.0, 0.0, 6.0]);
            track.fader.right_db = if symmetric {
                track.fader.left_db
            } else {
                draw.pick(&[-144.0_f32, -24.0, -6.0, 0.0, 6.0])
            };
        }
        let mut track_chains: [Vec<Effect>; 3] = Default::default();
        for (rack, strip_kinds) in racks.into_iter().enumerate() {
            let mut kinds: Vec<usize> = if free {
                (0..draw.below(4))
                    .map(|_| draw.below(KINDS.len()))
                    .collect()
            } else {
                let mut kinds: Vec<usize> = strip_kinds
                    .iter()
                    .copied()
                    .filter(|_| hazard || !draw.chance(drop, 1_000))
                    .collect();
                if !hazard && draw.chance(drop / 4, 1_000) {
                    let at = draw.below(kinds.len() + 1);
                    kinds.insert(at, draw.below(KINDS.len()));
                }
                kinds
            };
            kinds.truncate(4);
            let effects: Vec<Effect> = kinds
                .iter()
                .enumerate()
                .map(|(slot, kind)| {
                    // On an asymmetric desk the first rack leans asymmetric and the later racks
                    // symmetric, so a strip often holds a symmetric stage after an asymmetric one:
                    // the later chain the collapse must not arm on a mono track (#970).
                    let symmetric = symmetric
                        || if rack == 0 {
                            !hazard && draw.chance(1, 3)
                        } else {
                            hazard || draw.chance(2, 3)
                        };
                    let mut effect = effect(
                        draw,
                        registry,
                        &template,
                        (*kind, if hazard { 3 } else { links[*kind] }),
                        &format!("fx{rack}{slot}"),
                        symmetric,
                    );
                    if matches!(*kind, 1 | 2) && index > 0 && draw.chance(1, 12) {
                        effect.sidechain = SidechainDeclaration::Routed(Sidechain {
                            source: RouteSource::Track {
                                track_id: sid(&format!("ch{:02}", draw.below(index))),
                                tap: draw.pick(&TAPS),
                            },
                            port_id: sid("sidechain-in"),
                        });
                    }
                    effect
                })
                .collect();
            track_chains[rack] = effects;
        }
        chains.push(track_chains);
        model.tracks.push(track);
        let to_bus = submixes > 0 && draw.chance(1, 4);
        let source = |tap| RouteSource::Track {
            track_id: sid(&id),
            tap,
        };
        if !to_bus || draw.chance(1, 2) {
            model.routes.push(route(
                &format!("{id}-main"),
                source(SendTap::PostPan),
                main_out(),
                0.0,
            ));
        }
        if to_bus {
            model.routes.push(route(
                &format!("{id}-bus"),
                source(draw.pick(&TAPS)),
                RouteDestination::SubmixInput {
                    submix_id: sid(&format!("bus{}", draw.below(submixes))),
                },
                -6.0,
            ));
        }
    }
    place(&mut model, chains);
    (model, hazard)
}

/// One prepared console: the host, its live handles, and the rendered output so far.
struct Arm {
    label: &'static str,
    host: PreparedHost,
    handles: HostLiveControlHandles,
}

/// One block's live-control records, applied to every arm.
#[derive(Clone, Copy, Debug)]
enum Live {
    Input(usize, TrackInputRecord),
    Fader(usize, TrackFaderRecord),
    Matrix(usize, TrackControlRecord),
    Effect(usize, EffectControlRecord),
}

fn lanes(draw: &mut Draw) -> BuiltinLaneSelector {
    draw.pick(&[
        BuiltinLaneSelector::Both,
        BuiltinLaneSelector::Both,
        BuiltinLaneSelector::Left,
        BuiltinLaneSelector::Right,
    ])
}

fn smoothing(draw: &mut Draw) -> u32 {
    draw.pick(&[0_u32, 1, 17, 64, 480])
}

/// Up to three live-control records for this block.
fn live_records(draw: &mut Draw, handles: &HostLiveControlHandles) -> Vec<Live> {
    let mut records = Vec::new();
    let tracks = handles.track_controls.len();
    for _ in 0..draw.below(4) {
        match draw.below(6) {
            0 if tracks > 0 => {
                let record = if draw.chance(1, 3) {
                    TrackInputRecord::PolarityInvert {
                        lanes: lanes(draw),
                        inverted: draw.chance(1, 2),
                        smoothing_samples: smoothing(draw),
                    }
                } else {
                    TrackInputRecord::TrimDb {
                        lanes: lanes(draw),
                        db: draw.pick(&[-144.0_f32, -12.0, -0.5, 0.0, 6.0, 24.0]),
                        smoothing_samples: smoothing(draw),
                    }
                };
                records.push(Live::Input(draw.below(tracks), record));
            }
            1 if tracks > 0 => {
                let record = if draw.chance(1, 3) {
                    TrackFaderRecord::Mute {
                        lanes: lanes(draw),
                        muted: draw.chance(1, 2),
                        smoothing_samples: smoothing(draw),
                    }
                } else {
                    TrackFaderRecord::FaderDb {
                        lanes: lanes(draw),
                        db: draw.pick(&[-144.0_f32, -24.0, -6.0, 0.0, 6.0]),
                        smoothing_samples: smoothing(draw),
                    }
                };
                records.push(Live::Fader(draw.below(tracks), record));
            }
            2 if tracks > 0 => {
                let mut coefficient = || draw.pick(&[-1.0_f32, -0.5, 0.0, 0.5, 0.707_106_77, 1.0]);
                let matrix = Matrix2x2 {
                    ll: coefficient(),
                    lr: coefficient(),
                    rl: coefficient(),
                    rr: coefficient(),
                };
                let record = TrackControlRecord {
                    matrix,
                    smoothing_samples: smoothing(draw),
                };
                records.push(Live::Matrix(draw.below(tracks), record));
            }
            _ if !handles.effect_controls.is_empty() => {
                let index = draw.below(handles.effect_controls.len());
                let descriptor: &EffectDescriptor = handles.effect_controls[index].descriptor;
                if draw.chance(1, 5) {
                    records.push(Live::Effect(
                        index,
                        EffectControlRecord::Bypass(draw.chance(1, 2)),
                    ));
                    continue;
                }
                let automatable: Vec<usize> = descriptor
                    .parameters
                    .iter()
                    .enumerate()
                    .filter(|(_, parameter)| parameter.automation_rate != AutomationRate::None)
                    .map(|(index, _)| index)
                    .collect();
                if automatable.is_empty() {
                    continue;
                }
                let parameter_index = draw.pick(&automatable);
                let parameter = &descriptor.parameters[parameter_index];
                let value = value(draw, parameter);
                let record = |channel| EffectControlRecord::Parameter {
                    parameter_index: parameter_index as u32,
                    channel,
                    value,
                };
                if parameter.channel_policy == ParameterChannelPolicy::Shared {
                    records.push(Live::Effect(
                        index,
                        record(effect_contract::ParameterChannel::Both),
                    ));
                } else if draw.chance(2, 3) {
                    // A both-channel write arrives as a bit-equal twin pair (issue #1004).
                    records.push(Live::Effect(
                        index,
                        record(effect_contract::ParameterChannel::Left),
                    ));
                    records.push(Live::Effect(
                        index,
                        record(effect_contract::ParameterChannel::Right),
                    ));
                } else {
                    let channel = draw.pick(&[
                        effect_contract::ParameterChannel::Left,
                        effect_contract::ParameterChannel::Right,
                    ]);
                    records.push(Live::Effect(index, record(channel)));
                }
            }
            _ => {}
        }
    }
    records
}

/// Pushes one record and says whether the queue took it; a full queue answers the same on every
/// arm, because every arm has seen the same records.
fn push(handles: &mut HostLiveControlHandles, record: Live) -> bool {
    match record {
        Live::Input(track, record) => handles.track_controls[track].input.try_push(record).is_ok(),
        Live::Fader(track, record) => handles.track_controls[track].fader.try_push(record).is_ok(),
        Live::Matrix(track, record) => handles.track_controls[track]
            .producer
            .try_push(record)
            .is_ok(),
        Live::Effect(index, record) => handles.effect_controls[index].try_push(record).is_ok(),
    }
}

/// What the probe reached, over every seed.
#[derive(Debug, Default)]
struct Reach {
    consoles: u64,
    refused: u64,
    armed_collapse_blocks: u64,
    live_records: u64,
}

#[allow(clippy::too_many_lines)]
fn probe(seed: u64, registry: &NativeEffectRegistry, reach: &mut Reach) {
    let mut draw = Draw::new(seed);
    let (model, hazard) = generate(&mut draw, registry);
    let document = canonical_session_json(&model).expect("the generated console canonicalizes");
    let caps = caps();
    // An internal meter tap splits every strip into chains at that boundary.
    let tap = if hazard {
        draw.pick(&[MeterTap::PostSimd1, MeterTap::PostDynamic])
    } else {
        draw.pick(&[
            MeterTap::PostMatrix,
            MeterTap::PostSimd1,
            MeterTap::PostDynamic,
            MeterTap::PostFader,
            MeterTap::PostInputBuiltins,
        ])
    };
    let request = live_controls(tap);
    let compiled = match compile_host_session(&document, &caps) {
        Ok(compiled) => compiled,
        Err(failure) => {
            // A generated console the compiler refuses is a generator gap, counted and asserted
            // rare below; it is never silently a pass.
            eprintln!(
                "seed {seed}: refused {}",
                String::from_utf8_lossy(failure.as_bytes())
            );
            reach.refused += 1;
            return;
        }
    };
    let prepared = |serialized: bool| {
        let result = if serialized {
            prepare_host_runtime_between_render_calls(&compiled, &caps, &request)
        } else {
            prepare_host_runtime_with_live_controls(&compiled, &caps, &request)
        };
        result.unwrap_or_else(|failure| {
            panic!(
                "seed {seed}: prepare refused a compiled console: {}",
                String::from_utf8_lossy(failure.as_bytes())
            )
        })
    };
    let mut arms: Vec<Arm> = [
        ("armed", false, false),
        ("dual", false, true),
        ("serialized", true, false),
    ]
    .into_iter()
    .map(|(label, serialized, forced)| {
        let (mut host, handles) = prepared(serialized);
        host.plan.force_mono_collapse_off(forced);
        Arm {
            label,
            host,
            handles,
        }
    })
    .collect();
    reach.consoles += 1;
    let rate = model.sample_rate_hz;
    let source_channels = 2;
    for block in 0..BLOCKS {
        let context = format!("seed {seed} block {block}");
        let records = live_records(&mut draw, &arms[0].handles);
        for record in &records {
            let taken: Vec<bool> = arms
                .iter_mut()
                .map(|arm| push(&mut arm.handles, *record))
                .collect();
            assert!(
                taken.iter().all(|took| *took == taken[0]),
                "{context}: the arms' queues disagree on {record:?}"
            );
            reach.live_records += u64::from(taken[0]);
        }
        let base = (block * QUANTUM) as u64;
        let planes: Vec<Vec<f32>> = (0..source_channels)
            .map(|_| {
                let profile = loop {
                    let profile = draw.profile();
                    if profile.is_finite_legal() {
                        break profile;
                    }
                };
                (0..QUANTUM).map(|_| draw.sample(profile)).collect()
            })
            .collect();
        let borrowed: Vec<&[f32]> = planes.iter().map(Vec::as_slice).collect();
        let mut outputs = Vec::with_capacity(arms.len());
        for arm in &mut arms {
            arm.host
                .sources
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: base,
                        sample_rate_hz: rate,
                        planes: &borrowed,
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("a source block");
            let mut samples = vec![0.0_f32; QUANTUM * 2];
            let output =
                PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
            arm.host
                .plan
                .render(
                    RenderIo { output },
                    RenderTime {
                        absolute_sample: base,
                    },
                )
                .expect("render");
            for meter in &mut arm.handles.meters {
                while meter.consumer.try_pop().is_ok() {}
            }
            outputs.push(samples);
        }
        for (arm, output) in arms.iter().zip(&outputs).skip(1) {
            if let Some(word) = first_difference(&outputs[0], output) {
                panic!(
                    "{context}: the {} arm rendered {} frame {} as {:#010x}, the armed arm as \
                     {:#010x} (armed collapse counters {:?}, live records {records:?})",
                    arm.label,
                    if word < QUANTUM { "left" } else { "right" },
                    word % QUANTUM,
                    output[word].to_bits(),
                    outputs[0][word].to_bits(),
                    arms[0].host.plan.bank_collapse_counters(),
                );
            }
        }
        assert!(
            outputs[0].iter().all(|word| word.abs() < 1.0e30),
            "{context}: legal input left the D7 block bound"
        );
    }
    reach.armed_collapse_blocks += arms[0].host.plan.bank_collapse_counters()[0];
    assert_eq!(
        arms[1].host.plan.bank_collapse_counters()[0],
        0,
        "seed {seed}: the forced-dual arm never collapses"
    );
}

#[test]
fn randomized_consoles_render_the_same_bits_armed_dual_and_serialized() {
    let registry = launch_native_effect_registry().expect("the launch registry");
    let mut reach = Reach::default();
    let seeds = run_seeds(TEST, REPLAY, 12, |seed| probe(seed, &registry, &mut reach));
    println!("{seeds} seeds: {reach:?}");
    if dsp_reference::randomized::replaying() {
        return;
    }
    assert!(
        reach.refused * 4 <= seeds,
        "the generator must emit consoles the compiler accepts: {reach:?}"
    );
    assert!(
        reach.armed_collapse_blocks > 0,
        "the armed collapse must fire on some console, or armed against dual compares nothing: \
         {reach:?}"
    );
    assert!(reach.live_records > 0, "{reach:?}");
}

// ---------------------------------------------------------------------------------------------
// The response queries: the stopped preview against the live snapshot, and the grid against its
// law.
// ---------------------------------------------------------------------------------------------

const RESPONSE_TEST: &str = "randomized_response_queries_agree_across_paths_and_with_the_grid_law";
const RESPONSE_REPLAY: &str = "cargo test -p host-core --features host-core/test-support --test \
                               randomized -- --exact \
                               randomized_response_queries_agree_across_paths_and_with_the_grid_law";

/// `generate_response_grid`'s law, restated in `f64` from its documentation: `points >= 2`, finite
/// endpoints with `0 <= minimum < maximum <= rate / 2` (and a positive minimum on a logarithmic
/// axis), linear or logarithmic interpolation, both endpoints written exactly, and a strictly
/// increasing finite result.
fn grid_law(grid: ResponsePreviewGrid, rate: u32) -> Option<Vec<f32>> {
    let (points, minimum, maximum, logarithmic) = match grid {
        ResponsePreviewGrid::Linear {
            points,
            minimum_hz,
            maximum_hz,
        } => (points, minimum_hz, maximum_hz, false),
        ResponsePreviewGrid::Logarithmic {
            points,
            minimum_hz,
            maximum_hz,
        } => (points, minimum_hz, maximum_hz, true),
    };
    if points < 2
        || !minimum.is_finite()
        || !maximum.is_finite()
        || minimum < 0.0
        || minimum >= maximum
        || maximum > rate as f32 * 0.5
        || (logarithmic && minimum <= 0.0)
    {
        return None;
    }
    let (low, high) = (f64::from(minimum), f64::from(maximum));
    let mut values = Vec::with_capacity(points);
    for index in 0..points {
        let t = index as f64 / (points - 1) as f64;
        let value = if logarithmic {
            (low.ln() + (high.ln() - low.ln()) * t).exp()
        } else {
            low + (high - low) * t
        } as f32;
        if !value.is_finite() || values.last().is_some_and(|last: &f32| value <= *last) {
            return None;
        }
        values.push(value);
    }
    values[0] = minimum;
    values[points - 1] = maximum;
    Some(values)
}

/// A grid at the edges of its law: two points, the endpoints at `0`, Nyquist and one ulp either
/// side, an empty or inverted range, a non-finite endpoint.
fn draw_grid(draw: &mut Draw, rate: u32) -> ResponsePreviewGrid {
    let nyquist = rate as f32 * 0.5;
    let points = draw.pick(&[2_usize, 2, 3, 4, 17, 64, 256]);
    let logarithmic = draw.chance(1, 2);
    let minimum = draw.pick(&[
        0.0_f32,
        f32::from_bits(1),
        1.0e-3,
        20.0,
        20.0,
        1_000.0,
        nyquist * 0.5,
        nyquist,
        -1.0,
        f32::NAN,
    ]);
    let maximum = draw.pick(&[
        nyquist,
        nyquist,
        nyquist.next_down(),
        nyquist.next_up(),
        20_000.0_f32.min(nyquist),
        minimum,
        minimum.next_up(),
        1_000.0,
        f32::INFINITY,
    ]);
    if logarithmic {
        ResponsePreviewGrid::Logarithmic {
            points,
            minimum_hz: minimum,
            maximum_hz: maximum,
        }
    } else {
        ResponsePreviewGrid::Linear {
            points,
            minimum_hz: minimum,
            maximum_hz: maximum,
        }
    }
}

/// `f32` words at most one ulp apart: the grid law is restated with the platform's `ln`/`exp`,
/// the engine's with its own `math` crate.
fn within_one_ulp(a: f32, b: f32) -> bool {
    a.to_bits().abs_diff(b.to_bits()) <= 1
}

/// A response override list for `effect`'s parameters: a `Both` session value becomes one
/// override per channel for a per-lane parameter, which is the preview's spelling of it.
fn overrides(effect: &Effect, descriptor: &EffectDescriptor) -> Vec<ResponseParameterOverride> {
    let mut rows = Vec::new();
    for param in &effect.params {
        let parameter = descriptor
            .parameters
            .iter()
            .find(|parameter| parameter.id.0 == param.parameter_id)
            .expect("a declared parameter");
        let channels: &[effect_contract::ParameterChannel] =
            match (param.channel, parameter.channel_policy) {
                (_, ParameterChannelPolicy::Shared) => &[effect_contract::ParameterChannel::Both],
                (ParameterChannel::Both, _) => &[
                    effect_contract::ParameterChannel::Left,
                    effect_contract::ParameterChannel::Right,
                ],
                (ParameterChannel::Left, _) => &[effect_contract::ParameterChannel::Left],
                (ParameterChannel::Right, _) => &[effect_contract::ParameterChannel::Right],
            };
        for channel in channels {
            rows.push(ResponseParameterOverride {
                parameter_id: param.parameter_id,
                channel: *channel,
                value: param.value,
            });
        }
    }
    rows
}

/// What an override list must answer: the first unknown parameter, the first channel that
/// contradicts its parameter's policy, the first repeat of a parameter and channel -- in list
/// order -- or nothing.
fn override_law(
    rows: &[ResponseParameterOverride],
    descriptor: &EffectDescriptor,
) -> Option<ResponsePreviewError> {
    for (index, row) in rows.iter().enumerate() {
        let Some(parameter) = descriptor
            .parameters
            .iter()
            .find(|parameter| parameter.id.0 == row.parameter_id)
        else {
            return Some(ResponsePreviewError::InvalidParameter);
        };
        let both = row.channel == effect_contract::ParameterChannel::Both;
        if both != (parameter.channel_policy == ParameterChannelPolicy::Shared) {
            return Some(ResponsePreviewError::ConflictingChannel);
        }
        if rows[..index].iter().any(|earlier| {
            earlier.parameter_id == row.parameter_id && earlier.channel == row.channel
        }) {
            return Some(ResponsePreviewError::DuplicateParameter);
        }
    }
    None
}

/// What the response differential reached.
#[derive(Debug, Default)]
struct ResponseReach {
    compared_points: u64,
    invalid_grids: u64,
    shape_refusals: u64,
    override_refusals: u64,
    skipped: u64,
}

#[allow(clippy::too_many_lines)]
fn response_probe(seed: u64, registry: &NativeEffectRegistry, reach: &mut ResponseReach) {
    let mut draw = Draw::new(seed ^ 0x5245_5350);
    let rate = draw.pick(&[44_100_u32, 48_000, 88_200, 96_000]);
    let mut model = parse_session_json(BANK).expect("the bank fixture");
    model.sample_rate_hz = rate;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let symmetric = draw.chance(1, 3);
    let template = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    let mut eq = effect(&mut draw, registry, &template, (0, 3), "eq", symmetric);
    eq.bypass = draw.chance(1, 8);
    eq.link_mode = LinkMode::DualMono;
    place(&mut model, vec![[vec![eq.clone()], Vec::new(), Vec::new()]]);
    let track = &mut model.tracks[0];
    let nyquist = rate as f32 * 0.5;
    let mut filters = [0.0_f32; 4];
    for (index, builtins) in [&mut track.builtins.left, &mut track.builtins.right]
        .into_iter()
        .enumerate()
    {
        if draw.chance(1, 2) {
            builtins.hpf_hz = draw.pick(&[20.0_f32, 80.0, 1_000.0, 5_000.0]);
        }
        if draw.chance(1, 2) {
            builtins.lpf_hz = draw.pick(&[2_000.0_f32, 12_000.0, 18_000.0_f32.min(nyquist * 0.9)]);
        }
        filters[index * 2] = builtins.hpf_hz;
        filters[index * 2 + 1] = builtins.lpf_hz;
    }
    let document = canonical_session_json(&model).expect("the probe canonicalizes");
    let caps = caps();
    let Ok(compiled) = compile_host_session(&document, &caps) else {
        reach.skipped += 1;
        return;
    };
    let Ok(mut host) = host_core::prepare_host_runtime(&compiled, &caps) else {
        reach.skipped += 1;
        return;
    };
    let mut collector = ResponseSnapshotCollector::new(
        rate,
        4,
        effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS,
        64,
    );
    let capture = host
        .copy_response_snapshot(model.tracks[0].id.as_str(), &mut collector)
        .expect("a live snapshot copies");
    let snapshot = collector.finish(capture);

    let descriptor = registry.get_ascii(KINDS[0]).expect("the EQ").descriptor();
    let mut rows = overrides(&eq, descriptor);
    // Order does not matter to a valid list; a third of the lists carry one defect instead.
    for index in (1..rows.len()).rev() {
        rows.swap(index, draw.below(index + 1));
    }
    let broken = draw.chance(1, 3) && !rows.is_empty();
    if broken {
        let at = draw.below(rows.len());
        match draw.below(3) {
            0 => rows[at].parameter_id = u32::MAX - draw.below(3) as u32,
            1 => {
                rows[at].channel = if rows[at].channel == effect_contract::ParameterChannel::Both {
                    effect_contract::ParameterChannel::Left
                } else {
                    effect_contract::ParameterChannel::Both
                };
            }
            _ => {
                let copy = rows[at];
                rows.insert(draw.below(rows.len() + 1), copy);
            }
        }
    }
    let configuration_id = draw.next_u64() >> 11;
    let preview = host_core::prepare_response_preview(ResponsePreviewRequest {
        configuration_id,
        sample_rate_hz: rate,
        quantum_frames: 128,
        target: ResponsePreviewTarget::Effect {
            effect_id: KINDS[0],
            overrides: &rows,
            quality: effect_contract::EffectQuality::Normal,
            bypass: eq.bypass,
            link_mode: effect_contract::LinkMode::DualMono,
        },
        limits: ResponsePreviewLimits::default(),
    });
    if let Some(expected) = override_law(&rows, descriptor) {
        assert_eq!(
            preview.as_ref().err(),
            Some(&expected),
            "seed {seed}: the override list {rows:?}"
        );
        reach.override_refusals += 1;
        return;
    }
    let preview =
        preview.unwrap_or_else(|error| panic!("seed {seed}: the EQ preview refused ({error:?})"));
    let filter_preview = host_core::prepare_response_preview(ResponsePreviewRequest {
        configuration_id: 1,
        sample_rate_hz: rate,
        quantum_frames: 128,
        target: ResponsePreviewTarget::InputFilters {
            left_hpf_hz: filters[0],
            left_lpf_hz: filters[1],
            right_hpf_hz: filters[2],
            right_lpf_hz: filters[3],
        },
        limits: ResponsePreviewLimits::default(),
    })
    .unwrap_or_else(|error| panic!("seed {seed}: the input-filter preview refused ({error:?})"));

    for query in 0..4 {
        let context = format!("seed {seed} query {query} at {rate} Hz");
        let grid = draw_grid(&mut draw, rate);
        let points = grid.points();
        let law = grid_law(grid, rate);
        // The grid alone.
        let mut axis = vec![0.0_f32; points];
        let generated = host_core::generate_response_grid(grid, rate, &mut axis);
        match (&law, generated) {
            (Some(expected), Ok(())) => assert!(
                expected
                    .iter()
                    .zip(&axis)
                    .all(|(a, b)| within_one_ulp(*a, *b))
                    && expected[0] == axis[0]
                    && expected[points - 1] == axis[points - 1],
                "{context}: {grid:?} generated {axis:?}, the law {expected:?}"
            ),
            (None, Err(ResponsePreviewError::InvalidGrid)) => reach.invalid_grids += 1,
            (law, generated) => panic!(
                "{context}: {grid:?}: the law says {}, generate_response_grid {generated:?}",
                if law.is_some() { "valid" } else { "invalid" }
            ),
        }
        // A caller buffer one point short or long, a quarter of the time.
        let short = draw.chance(1, 4);
        let length = if short {
            draw.pick(&[points.saturating_sub(1), points + 1])
        } else {
            points
        };
        let mut frequencies = vec![0.0_f32; length];
        let mut snapshot_left = vec![0.0_f32; length];
        let mut snapshot_right = vec![0.0_f32; length];
        let composed = query_response_snapshot_into(
            &snapshot,
            grid,
            ResponseSnapshotOutput {
                frequencies_hz: &mut frequencies,
                total_left_db: &mut snapshot_left,
                total_right_db: &mut snapshot_right,
            },
        );
        let mut preview_frequencies = vec![0.0_f32; points];
        let mut totals = [
            vec![0.0_f32; length],
            vec![0.0_f32; length],
            vec![0.0_f32; length],
            vec![0.0_f32; length],
        ];
        let [eq_left, eq_right, filter_left, filter_right] = &mut totals;
        let eq_result = preview.query_into(
            grid,
            ResponsePreviewOutput {
                frequencies_hz: &mut preview_frequencies,
                total_left_db: eq_left,
                total_right_db: eq_right,
                sections_left_db: None,
                sections_right_db: None,
            },
        );
        let filter_result = filter_preview.query_into(
            grid,
            ResponsePreviewOutput {
                frequencies_hz: &mut preview_frequencies,
                total_left_db: filter_left,
                total_right_db: filter_right,
                sections_left_db: None,
                sections_right_db: None,
            },
        );
        if points < 2 || short {
            assert_eq!(
                composed.err(),
                Some(ResponseSnapshotQueryError::OutputShape),
                "{context}: a snapshot query into {length} words for {points} points"
            );
            if law.is_some() {
                assert_eq!(
                    eq_result.err(),
                    Some(ResponsePreviewError::OutputShape),
                    "{context}"
                );
                assert_eq!(
                    filter_result.err(),
                    Some(ResponsePreviewError::OutputShape),
                    "{context}"
                );
            }
            reach.shape_refusals += 1;
            continue;
        }
        let Some(expected_axis) = law else {
            assert_eq!(
                composed.err(),
                Some(ResponseSnapshotQueryError::InvalidGrid),
                "{context}"
            );
            assert_eq!(
                eq_result.err(),
                Some(ResponsePreviewError::InvalidGrid),
                "{context}"
            );
            assert_eq!(
                filter_result.err(),
                Some(ResponsePreviewError::InvalidGrid),
                "{context}"
            );
            continue;
        };
        let summary =
            composed.unwrap_or_else(|error| panic!("{context}: the snapshot query ({error:?})"));
        assert_eq!(summary.sample_rate_hz, rate, "{context}");
        let eq_summary =
            eq_result.unwrap_or_else(|error| panic!("{context}: the EQ query ({error:?})"));
        assert_eq!(eq_summary.configuration_id, configuration_id, "{context}");
        filter_result.unwrap_or_else(|error| panic!("{context}: the filter query ({error:?})"));
        assert!(
            frequencies
                .iter()
                .zip(&expected_axis)
                .all(|(a, b)| within_one_ulp(*a, *b)),
            "{context}: the snapshot's axis"
        );
        // The snapshot composes the track's owners in `f64` log magnitudes; the previews are the
        // same owners one at a time. Where neither sits on the -120 dB floor, the snapshot total
        // is the sum of the previews' totals.
        for index in 0..points {
            for (side, composed, eq, filter) in [
                (
                    "left",
                    snapshot_left[index],
                    totals[0][index],
                    totals[2][index],
                ),
                (
                    "right",
                    snapshot_right[index],
                    totals[1][index],
                    totals[3][index],
                ),
            ] {
                if eq <= -100.0 || filter <= -100.0 || composed <= -100.0 {
                    continue;
                }
                let sum = eq + filter;
                assert!(
                    (composed - sum).abs() <= 2.0e-3 + 1.0e-5 * sum.abs(),
                    "{context}: {side} point {index} at {} Hz: the snapshot composed {composed} dB, \
                     the previews {eq} + {filter} dB",
                    frequencies[index]
                );
                reach.compared_points += 1;
            }
        }
    }
}

#[test]
fn randomized_response_queries_agree_across_paths_and_with_the_grid_law() {
    let registry = launch_native_effect_registry().expect("the launch registry");
    let mut reach = ResponseReach::default();
    let seeds = run_seeds(RESPONSE_TEST, RESPONSE_REPLAY, 24, |seed| {
        response_probe(seed, &registry, &mut reach);
    });
    println!("{seeds} seeds: {reach:?}");
    if dsp_reference::randomized::replaying() {
        return;
    }
    assert!(
        reach.compared_points > 0
            && reach.invalid_grids > 0
            && reach.shape_refusals > 0
            && reach.override_refusals > 0,
        "{reach:?}"
    );
    assert!(reach.skipped * 4 <= seeds, "{reach:?}");
}
