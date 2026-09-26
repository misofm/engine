//! Issue #918 gate 3: a banked source gather that reads the played transfer block in place
//! allocates and frees nothing on the render path, across 1000 blocks, and renders the bits the
//! same plan renders when its driver does not lend its planes (every claim copied).
//!
//! Issue #927: the same for a bankless plan, whose fused Output reduction reads each plain strip's
//! claim in place.

use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
use effect_contract::{BankWidth, LatencySamples, TailSamples};
use engine::{
    QuantumFrames,
    realtime::{self, PlanarBufferMut, RenderEnvelope, RenderError},
};
use graph::*;
use lane::Backend;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering::Relaxed},
};

const FRAMES: u32 = 13;
const LANES: usize = 4;
const BLOCKS: u64 = 1_000;
/// Every seventh block is an underrun: nothing is played, and the gather reads silence.
const UNDERRUN_EVERY: u64 = 7;

/// The allocator's mode is process-wide and both tests set and restore it, so they take turns
/// (the pattern of `rt9_resident_bank_input_alloc.rs`).
static ALLOCATOR_MODE_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct RestoreMode(Mode);
impl Drop for RestoreMode {
    fn drop(&mut self) {
        set_mode(self.0);
    }
}

/// `[copy_track_input calls, played_planes calls]`, shared with the test.
#[derive(Default)]
struct Calls([AtomicU64; 2]);

/// A four-claim source set over preallocated planes. `begin_block` refills them in place (a new
/// pattern per block, a negative zero in each, nothing on an underrun); the claim `lane` reads
/// channels `(lane, lane)` for lane 3 (a mono mapping) and `(lane, lane + 4)` otherwise.
struct Played {
    lend: bool,
    played: bool,
    planes: Vec<f32>,
    calls: Arc<Calls>,
}

impl Played {
    fn channels(lane: usize) -> (usize, usize) {
        if lane == 3 {
            (lane, lane)
        } else {
            (lane, lane + LANES)
        }
    }

    fn plane(&self, channel: usize) -> &[f32] {
        let frames = FRAMES as usize;
        &self.planes[channel * frames..(channel + 1) * frames]
    }
}

impl GraphPreparedSourceSetDriver for Played {
    fn claim_count(&self) -> usize {
        LANES
    }

    fn begin_block(&mut self, first_sample: u64, frames: u32) -> Result<(), RenderError> {
        let block = first_sample / u64::from(frames);
        self.played = block % UNDERRUN_EVERY != UNDERRUN_EVERY - 1;
        if self.played {
            for (channel, plane) in self.planes.chunks_exact_mut(FRAMES as usize).enumerate() {
                for (frame, word) in plane.iter_mut().enumerate() {
                    let value = (block as usize * 31 + channel * 7 + frame) % 97;
                    *word = (value as f32 - 48.0) * 0.0625;
                }
                plane[1] = -0.0;
            }
        }
        Ok(())
    }

    fn copy_track_input(
        &mut self,
        claim: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError> {
        self.calls.0[0].fetch_add(1, Relaxed);
        let (left_channel, right_channel) = Self::channels(claim);
        if self.played {
            left.copy_from_slice(self.plane(left_channel));
            right.copy_from_slice(self.plane(right_channel));
        } else {
            left.fill(0.0);
            right.fill(0.0);
        }
        Ok(())
    }

    fn provides_played_planes(&self) -> bool {
        self.lend
    }

    fn played_planes(&self, claim: usize) -> Option<(&[f32], &[f32])> {
        self.calls.0[1].fetch_add(1, Relaxed);
        if claim >= LANES || !self.played {
            return None;
        }
        let (left_channel, right_channel) = Self::channels(claim);
        Some((self.plane(left_channel), self.plane(right_channel)))
    }
}

/// A builtin bank that mixes its two planes, so a swapped or misread plane shows in the bits.
struct Tilt;
impl GraphPreparedBuiltinBankProcessor for Tilt {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        _frames: u32,
        _first_sample: u64,
    ) -> Result<(), RenderError> {
        for (left, right) in left.iter_mut().zip(right.iter_mut()) {
            let (a, b) = (*left, *right);
            *left = a * 0.75 - b * 0.125;
            *right = b * 1.25 + a * 0.5;
        }
        Ok(())
    }
}

fn estimate() -> GraphResourceEstimate {
    GraphResourceEstimate {
        logical_nodes: 0,
        materialized_nodes: 0,
        edges: 0,
        schedule_items: 0,
        dependency_levels: 0,
        reductions: 0,
        routes: 0,
        effects: 0,
        audio_buffer_samples: 0,
        total_delay_samples: 0,
        delay_bytes: 0,
        graph_metadata_bytes: 0,
        declared_effect_bytes: 0,
        effect_bank_count: 0,
        effect_bank_scratch_bytes: 0,
        effect_bank_runtime_buffer_bytes: 0,
        effect_bank_metadata_bytes: 0,
        builtin_bank_bytes: 0,
        builtin_bank_scratch_bytes: 0,
        builtin_bank_count: 0,
        largest_allocation_bytes: 0,
        incremental_plan_bytes: 0,
        session_plus_plan_bytes: 0,
    }
}

/// `Input (claimed) -> PostInputBuiltins (one W4 builtin bank) -> Route -> Output` per lane, or,
/// `bankless`, `Input (claimed) -> Route -> Output`: the plain strip whose route the Output fold
/// retires (issue #926) and whose claim the fused reduction reads in place (issue #927).
fn prepared_plan(
    lend: bool,
    bankless: bool,
    calls: &Arc<Calls>,
) -> engine::realtime::PreparedRenderPlan {
    let envelope = RenderEnvelope {
        sample_rate: engine::SampleRateHz(48_000),
        quantum: QuantumFrames(FRAMES),
        input_channels: None,
        output_channels: core::num::NonZeroUsize::new(2).expect("stereo"),
    };
    let stage = |lane: usize, stage| GraphNodeId::TrackStage {
        track_id: StableGraphId::parse(&format!("track{lane}")).expect("track id"),
        stage,
    };
    let inputs: Vec<_> = (0..LANES)
        .map(|lane| stage(lane, TrackStage::Input))
        .collect();
    let members: Vec<_> = (0..LANES)
        .map(|lane| stage(lane, TrackStage::PostInputBuiltins))
        .collect();
    let routes: Vec<_> = (0..LANES)
        .map(|lane| GraphNodeId::Route {
            route_id: StableGraphId::parse(&format!("route{lane}")).expect("route id"),
        })
        .collect();
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("output id"),
    };
    let port = |node: &GraphNodeId, kind| GraphPortId {
        node: node.clone(),
        kind,
        effect_port: None,
    };
    let edge = |id, source: &GraphNodeId, destination: &GraphNodeId| GraphEdge {
        id,
        source: port(source, GraphPortKind::MainOutput),
        destination: port(destination, GraphPortKind::MainInput),
        path: "$.rt10".to_owned(),
    };
    let mut edges = Vec::new();
    for lane in 0..LANES {
        let route_id = StableGraphId::parse(&format!("route{lane}")).expect("route id");
        let route_source = if bankless {
            &inputs[lane]
        } else {
            edges.push(edge(
                GraphEdgeId::TrackMain {
                    target: members[lane].clone(),
                },
                &inputs[lane],
                &members[lane],
            ));
            &members[lane]
        };
        edges.push(edge(
            GraphEdgeId::RouteSource {
                route_id: route_id.clone(),
            },
            route_source,
            &routes[lane],
        ));
        edges.push(edge(
            GraphEdgeId::RouteDestination { route_id },
            &routes[lane],
            &output,
        ));
    }
    edges.sort_by(|left, right| left.id.cmp(&right.id));
    let levels: Vec<Vec<GraphNodeId>> = if bankless {
        vec![inputs.clone(), routes.clone(), vec![output.clone()]]
    } else {
        vec![
            inputs.clone(),
            members.clone(),
            routes.clone(),
            vec![output.clone()],
        ]
    };
    let schedule: Vec<_> = levels.iter().flatten().cloned().collect();
    let mut nodes: Vec<_> = schedule
        .iter()
        .cloned()
        .map(|id| GraphNode {
            id,
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
        })
        .collect();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    let mut required = inputs.clone();
    if !bankless {
        required.extend(members.iter().cloned());
    }
    required.push(output.clone());
    let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 918,
        spec: GraphSpec {
            nodes,
            ports: Vec::new(),
            edges,
        },
        sequential_schedule: schedule,
        dependency_levels: levels
            .iter()
            .enumerate()
            .map(|(level, nodes)| DependencyLevel {
                level: level as u64,
                nodes: nodes.clone(),
            })
            .collect(),
        route_timings: Vec::new(),
        inserted_delays: Vec::new(),
        buffer_assignments: Vec::new(),
        estimate: estimate(),
        envelope,
        required_bindings: required,
        routes: routes
            .iter()
            .enumerate()
            .map(|(lane, node)| PreparedRoute {
                node: node.clone(),
                transform: RouteTransform {
                    gain: 0.5 + 0.125 * lane as f32,
                    ll: 0.875,
                    lr: -0.25,
                    rl: 0.375,
                    rr: 1.125,
                },
            })
            .collect(),
        track_delays: Vec::new(),
        effects: Vec::new(),
        effect_controls: Vec::new(),
        effect_observations: Vec::new(),
        banks: Vec::new(),
        builtin_banks: if bankless {
            Vec::new()
        } else {
            vec![GraphPreparedBuiltinBank {
                backend: Backend::Simd4,
                members: members.clone().into_boxed_slice(),
                processor: Box::new(Tilt),
                scratch: rack::AoSoaScratch::new(BankWidth::Four, FRAMES).expect("scratch"),
            }]
        },
        observers: Vec::new(),
    });
    let source_set = GraphPreparedSourceSet::new(
        envelope,
        inputs
            .iter()
            .map(|node| GraphSourceInputClaim { node: node.clone() })
            .collect(),
        GraphSourceSetResourceReport {
            pcm_payload_already_charged_bytes: 0,
            overhead_bytes: 0,
            total_engine_owned_bytes: 0,
            largest_allocation_bytes: 0,
        },
        Box::new(Played {
            lend,
            played: false,
            planes: vec![0.0; 2 * LANES * FRAMES as usize],
            calls: Arc::clone(calls),
        }),
    );
    graph
        .bind_with_source_set(
            GraphRuntimeBindings {
                envelope,
                nodes: vec![GraphNodeBinding::identity(output)],
                observers: Vec::new(),
            },
            source_set,
        )
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
}

/// Render `BLOCKS` blocks into one reused host buffer, keeping every block's master bits.
fn render(plan: &mut engine::realtime::PreparedRenderPlan, masters: &mut [u32]) {
    let frames = FRAMES as usize;
    let mut pcm = [f32::from_bits(0x7fc0_0918); 2 * FRAMES as usize];
    for block in 0..BLOCKS {
        let output = PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output");
        plan.render(
            realtime::RenderIo {
                input: None,
                output,
            },
            realtime::RenderTime {
                absolute_sample: block * u64::from(FRAMES),
            },
        )
        .expect("render");
        let start = block as usize * 2 * frames;
        for (slot, word) in masters[start..start + 2 * frames].iter_mut().zip(&pcm) {
            *slot = word.to_bits();
        }
    }
}

#[test]
fn an_in_place_source_gather_renders_the_copy_bits_and_allocates_nothing() {
    let _mode_guard = ALLOCATOR_MODE_GUARD
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);

    realtime::audit::warm_up();
    realtime::audit::reset();
    realtime::audit::in_render_scope(|| {
        let probe = Vec::<u8>::with_capacity(core::hint::black_box(64));
        core::hint::black_box(&probe);
        drop(probe);
    });
    let live = realtime::audit::snapshot();
    assert!(
        live.allocations > 0 && live.deallocations > 0,
        "the audit counts allocations in render scope"
    );

    let words = BLOCKS as usize * 2 * FRAMES as usize;
    let (lent_calls, copied_calls) = (Arc::new(Calls::default()), Arc::new(Calls::default()));
    let mut lent = prepared_plan(true, false, &lent_calls);
    let mut copied = prepared_plan(false, false, &copied_calls);
    let (mut lent_masters, mut copied_masters) = (vec![0_u32; words], vec![0_u32; words]);

    realtime::audit::reset();
    render(&mut lent, &mut lent_masters);
    let measured = realtime::audit::snapshot();
    assert_eq!(
        (measured.allocations, measured.deallocations),
        (0, 0),
        "{BLOCKS} in-place blocks allocate and free nothing"
    );

    render(&mut copied, &mut copied_masters);
    assert_eq!(
        lent_masters, copied_masters,
        "the in-place gather renders the copied plan's bits"
    );
    assert!(
        lent_masters.iter().any(|word| *word != 0),
        "the master carries audio"
    );
    let calls = |calls: &Calls| [calls.0[0].load(Relaxed), calls.0[1].load(Relaxed)];
    assert_eq!(
        calls(&lent_calls),
        [0, BLOCKS * LANES as u64],
        "lent: no claim copied, every lane's gather borrowed its planes every block"
    );
    assert_eq!(
        calls(&copied_calls),
        [BLOCKS * LANES as u64, 0],
        "not lent: every claim copied every block, no plane borrowed"
    );
}

/// Issue #927: the bankless plan's fused Output reduction reads every claim in place, allocates
/// and frees nothing across 1000 blocks (one in seven an underrun, read from the silence buffer),
/// and renders the bits of the same plan whose driver does not lend (every claim copied). The
/// driver's own call counts say which path each arm took.
#[test]
fn an_in_place_output_read_renders_the_copy_bits_and_allocates_nothing() {
    let _mode_guard = ALLOCATOR_MODE_GUARD
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);

    realtime::audit::warm_up();
    realtime::audit::reset();
    realtime::audit::in_render_scope(|| {
        let probe = Vec::<u8>::with_capacity(core::hint::black_box(64));
        core::hint::black_box(&probe);
        drop(probe);
    });
    let live = realtime::audit::snapshot();
    assert!(
        live.allocations > 0 && live.deallocations > 0,
        "the audit counts allocations in render scope"
    );

    let words = BLOCKS as usize * 2 * FRAMES as usize;
    let (lent_calls, copied_calls) = (Arc::new(Calls::default()), Arc::new(Calls::default()));
    let mut lent = prepared_plan(true, true, &lent_calls);
    let mut copied = prepared_plan(false, true, &copied_calls);
    let (mut lent_masters, mut copied_masters) = (vec![0_u32; words], vec![0_u32; words]);

    realtime::audit::reset();
    render(&mut lent, &mut lent_masters);
    let measured = realtime::audit::snapshot();
    assert_eq!(
        (measured.allocations, measured.deallocations),
        (0, 0),
        "{BLOCKS} in-place blocks allocate and free nothing"
    );

    render(&mut copied, &mut copied_masters);
    assert_eq!(
        lent_masters, copied_masters,
        "the in-place Output read renders the copied plan's bits"
    );
    assert!(
        lent_masters.iter().any(|word| *word != 0),
        "the master carries audio"
    );
    let calls = |calls: &Calls| [calls.0[0].load(Relaxed), calls.0[1].load(Relaxed)];
    assert_eq!(
        calls(&lent_calls),
        [0, BLOCKS * LANES as u64],
        "lent: no claim copied, every Output input borrowed its planes every block"
    );
    assert_eq!(
        calls(&copied_calls),
        [BLOCKS * LANES as u64, 0],
        "not lent: every claim copied every block, no plane borrowed"
    );
}
