//! Issue #966: bind never refuses a compiled plan for an effect bank that spans dependency levels.
//!
//! A bank runs as one unit at its first member's position in the level-major schedule
//! (`runtime::units_of`), so bind refuses one whose members sit at different dependency levels
//! (`PreparedGraphPlan::has_valid_structural_layout`, `graph.scheduler.layout`). The rack-bank
//! binder used to emit exactly that bank whenever a cohort lane skipped an earlier slot of its
//! leader's program -- one track without the EQ its neighbours carry -- because the identity slot
//! it runs there has no graph node, so its later slots sit one level early. The binder now leaves
//! such a slot unbound and its members render per node.
//!
//! The gates:
//!
//! * the reported reproducers: the 64-track console less one EQ, with the ragged lane first, in
//!   the middle and last in its group, and again on the mono desk, where the armed collapse
//!   fires; the #962 seed-412 shape; and the #970 probe's reduced mono console;
//! * two over-reach guards: lanes that all skip the same slot still bank the next one, and a slot
//!   after a misaligned one, where the lanes realign, still banks;
//! * a randomized compile-then-bind probe over realistic consoles at every width.
//!
//! Every rendered plan is compared with the same session compiled at `Backend::Scalar`, where
//! nothing banks, twice: with the mono collapse unarmed, and armed the way `host-core`'s prepare
//! arms it. Banking regroups lanes and never changes per-lane arithmetic (AGENTS.md), so every
//! width must render the scalar plan's bits.
//!
//! Scope: the claim is that bind never refuses a compiled plan for a cross-level *effect* bank.
//! Builtin banks are pooled by each node's own level, so they cannot form one.

use builtins_compiler::{
    BuiltinCompileCaps, prepare_session_builtins, session_structural_symmetry,
};
use effect_compiler::{
    CONSOLE_ELIGIBLE_EFFECTS, EffectCompileCaps, launch_native_effect_registry,
    prepare_native_session_effects,
};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    ChannelMatrix, CompileCaps, ConsoleEntry, ConsoleSlot, Effect, EffectIdentity, Route,
    RouteDestination, RouteSource, SendTap, SessionModel, Sidechain, SidechainDeclaration,
    StableId, Submix, compile_session, parse_session_json,
};
use std::collections::{BTreeMap, BTreeSet};

const INTENDED: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");

/// The same desk with every track mono-mapped and every strip symmetric. The armed mono collapse
/// fires on it, so the armed leg can see a collapse that moves a bit. On a mono-mapped desk with
/// the intended fixture's asymmetric strips, the arming must instead decline every chain that does
/// not gather the track input (#970); the probe keeps both desks for that reason.
const MONO: &str = include_str!("../../../fixtures/session/v1/console-sixty-four-track-mono.json");

/// The #970 verification probe's reduced reproducer: eight mono-mapped tracks, `t0` running
/// `[eq, comp]` beside seven running `[comp]`. Its per-track racks differ, so #1093's migration
/// folded them into each track's inserts (placement invariance, #163, keeps the bits).
const REDUCED_MONO: &str = include_str!("data/reduced-nobus-from-970-verify.json");

/// Every width a host compiles for: the scalar oracle, the browser and mobile width, and native.
pub const WIDTHS: [Backend; 3] = [Backend::Scalar, Backend::Simd4, Backend::Simd8];

/// The blocks a render compares. PDC delays the output of a limiter or multiband strip by more
/// than a thousand samples, so two blocks is mostly silence and compares nothing.
pub const BLOCKS: u64 = 16;

fn graph_caps() -> GraphCompileCaps {
    GraphCompileCaps {
        maximum_nodes: u64::MAX,
        maximum_edges: u64::MAX,
        maximum_schedule_items: u64::MAX,
        maximum_dependency_levels: u64::MAX,
        maximum_audio_buffer_samples: u64::MAX,
        maximum_delay_samples_per_edge: u64::MAX,
        maximum_total_delay_samples: u64::MAX,
        maximum_graph_bytes: u64::MAX,
        maximum_plan_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_finite_tail_samples: u64::MAX,
    }
}

fn builtin_caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

/// One source channel: channel 0 is an impulse plus a slow sine, every other channel a cosine.
fn source_channel(channel: u8, t: f32) -> f32 {
    if channel == 0 {
        math::sinf(t * 0.013) + if t == 0.0 { 1.0 } else { 0.0 }
    } else {
        math::cosf(t * 0.021)
    }
}

/// A track's input: each side reads the source channel the session maps it to, scaled per track.
///
/// A mono-mapped track therefore really is symmetric at its input, as a host's source ring keeps
/// it. That is the premise the armed mono collapse relies on, so a source that fed such a track
/// two different sides would report a divergence no host can produce.
struct TrackSource {
    gain: f32,
    left_channel: u8,
    right_channel: u8,
}
impl GraphRuntimeProcessor for TrackSource {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), engine::realtime::RenderError> {
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            let t = (block.first_sample + index as u64) as f32;
            *left = self.gain * source_channel(self.left_channel, t);
            *right = self.gain * source_channel(self.right_channel, t);
        }
        Ok(())
    }
}

struct Identity;
impl GraphRuntimeProcessor for Identity {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), engine::realtime::RenderError> {
        Ok(())
    }
}

fn input_binding(
    node: &GraphNodeId,
    channels: &BTreeMap<&str, (u8, u8)>,
) -> Box<dyn GraphRuntimeProcessor> {
    let GraphNodeId::TrackStage {
        track_id,
        stage: TrackStage::Input,
    } = node
    else {
        return Box::new(Identity);
    };
    let hash = track_id.as_str().bytes().fold(7_u32, |hash, byte| {
        hash.wrapping_mul(31).wrapping_add(u32::from(byte))
    });
    let (left_channel, right_channel) = channels[track_id.as_str()];
    Box::new(TrackSource {
        gain: 0.05 + (hash % 13) as f32 * 0.01,
        left_channel,
        right_channel,
    })
}

/// Whether a bound plan's mono collapse is armed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Collapse {
    /// Nothing armed: the plan exactly as `GraphCompiler` binds it.
    Unarmed,
    /// Armed on every track whose structural witness admits it, exactly as
    /// `host-core::prepare_host_session` arms a plan before handing it to a host.
    Armed,
}

/// What one compile-then-bind of one session at one width did.
#[derive(Debug, Default)]
pub struct Outcome {
    /// Compile's first refusal code, if it refused the session.
    pub compile: Option<String>,
    /// Bind's refusal code, if bind refused a plan compile accepted.
    pub bind: Option<&'static str>,
    /// Bound effect banks.
    pub effect_banks: usize,
    /// Bound effect banks whose members span dependency levels. Bind refuses every one.
    pub cross_level_banks: usize,
    /// Planned full-group slots every lane runs whose members span levels: the shape the fix
    /// leaves unbound. It depends only on the plan, so it also counts shapes a width this build's
    /// factories decline would meet on a build that banks it.
    pub misaligned_slots: usize,
    /// For each misaligned slot, in plan order, the lanes that reach it before the slot's latest
    /// level: the ragged lanes, whose position in the group the reproducers pin.
    pub early_lanes: Vec<Vec<usize>>,
    /// Rendered output bits, when rendered.
    pub pcm: Option<Vec<u32>>,
    /// `[collapsed blocks, collapsible cohorts]` after the render.
    pub collapse: [u64; 2],
}

/// Compile `model` the way every host does at `dispatch`, bind it, arm it as `collapse` says, and
/// render `blocks` blocks.
pub fn compile_bind_render(
    model: &SessionModel,
    dispatch: Backend,
    blocks: u64,
    collapse: Collapse,
) -> Outcome {
    let mut outcome = Outcome::default();
    let session = match compile_session(model, compile_caps()) {
        Ok(session) => session,
        Err(error) => {
            outcome.compile = Some(format!("session {:?}", error.diagnostics()[0].code));
            return outcome;
        }
    };
    let registry = launch_native_effect_registry().expect("launch registry");
    let effects = match prepare_native_session_effects(
        &session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 32,
        },
    ) {
        Ok(effects) => effects,
        Err(error) => {
            outcome.compile = Some(format!("effects {}", error.0[0].code));
            return outcome;
        }
    };
    let builtins =
        prepare_session_builtins(&session, &[], builtin_caps()).expect("the session's builtins");
    let artifact = match GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 966,
        effects,
        builtins,
        caps: graph_caps(),
        dispatch,
    }) {
        Ok(artifact) => artifact,
        Err(failure) => {
            outcome.compile = Some(format!(
                "graph {}",
                failure.diagnostics.diagnostics()[0].code
            ));
            return outcome;
        }
    };

    let level_of: BTreeMap<&GraphNodeId, u64> = artifact
        .graph()
        .dependency_levels
        .iter()
        .flat_map(|level| level.nodes.iter().map(move |node| (node, level.level)))
        .collect();
    let effect_level = |node: &graph::EffectNodeId| level_of[&GraphNodeId::Effect(node.clone())];
    for bank in artifact.graph().effect_bank_members() {
        outcome.effect_banks += 1;
        if bank
            .iter()
            .any(|member| effect_level(member) != effect_level(&bank[0]))
        {
            outcome.cross_level_banks += 1;
        }
    }
    let report = &artifact.report().rack_cohorts;
    for group in report.plan.groups.iter().filter(|group| group.is_full()) {
        for slot in 0..group.program.len() {
            if !group.active_slots.iter().all(|lane| lane[slot]) {
                continue;
            }
            let levels: Vec<u64> = group
                .members
                .iter()
                .zip(group.active_slots.iter())
                .map(|(id, active)| {
                    let rank = active[..slot].iter().filter(|active| **active).count();
                    effect_level(&report.chains[id.as_ref().expect("full group")][rank])
                })
                .collect();
            let latest = levels.iter().copied().max().unwrap_or_default();
            if levels.iter().any(|level| *level != latest) {
                outcome.misaligned_slots += 1;
                outcome.early_lanes.push(
                    (0..levels.len())
                        .filter(|lane| levels[*lane] != latest)
                        .collect(),
                );
            }
        }
    }

    let channels: BTreeMap<&str, (u8, u8)> = model
        .tracks
        .iter()
        .map(|track| {
            (
                track.id.as_str(),
                (track.left_source_channel, track.right_source_channel),
            )
        })
        .collect();
    let envelope = artifact.envelope();
    let frames = envelope.quantum.0 as usize;
    let nodes = artifact
        .external_binding_nodes()
        .map(|node| GraphNodeBinding::new(node.clone(), input_binding(node, &channels)))
        .collect();
    let bound = match artifact.into_bound(GraphRuntimeBindings {
        envelope,
        nodes,
        observers: Vec::new(),
    }) {
        Ok(bound) => bound,
        Err(failure) => {
            outcome.bind = Some(failure.code);
            return outcome;
        }
    };
    let mut plan = bound.plan;
    if collapse == Collapse::Armed {
        let mono_source: BTreeSet<Box<str>> = session_structural_symmetry(&session)
            .into_iter()
            .filter(|(_, witness)| witness.eligible())
            .map(|(track, _)| track)
            .collect();
        plan.arm_mono_collapse(&|track: &str| mono_source.contains(track));
    }
    if blocks > 0 {
        let mut bits = Vec::with_capacity(blocks as usize * frames * 2);
        let mut pcm = vec![0.0_f32; frames * 2];
        for block in 0..blocks {
            pcm.fill(0.0);
            plan.render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                        .expect("output planes"),
                },
                RenderTime {
                    absolute_sample: block * frames as u64,
                },
            )
            .expect("render");
            bits.extend(pcm.iter().map(|sample| sample.to_bits()));
        }
        outcome.pcm = Some(bits);
        outcome.collapse = plan.bank_collapse_counters();
    }
    outcome
}

fn audible(pcm: Option<&Vec<u32>>) -> bool {
    pcm.is_some_and(|pcm| pcm.iter().any(|bits| f32::from_bits(*bits) != 0.0))
}

// ---------------------------------------------------------------------------------------------
// The randomized console generator.

/// xorshift64*: deterministic, dependency-free, and the same on every target.
pub struct Rng(u64);
impl Rng {
    /// A generator for `seed`, warmed past xorshift's weak first outputs.
    pub fn new(seed: u64) -> Self {
        let mut rng = Self(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        for _ in 0..4 {
            rng.next();
        }
        rng
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    fn chance(&mut self, per_mille: u64) -> bool {
        self.below(1000) < per_mille
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Eq,
    Comp,
    Gate,
    Multiband,
    Limiter,
    SoftClip,
    Transient,
    Delay,
}

const KINDS: [Kind; 8] = [
    Kind::Eq,
    Kind::Comp,
    Kind::Gate,
    Kind::Multiband,
    Kind::Limiter,
    Kind::SoftClip,
    Kind::Transient,
    Kind::Delay,
];

impl Kind {
    const fn native_id(self) -> &'static str {
        match self {
            Self::Eq => "miso.parametric-eq",
            Self::Comp => "miso.compressor",
            Self::Gate => "miso.gate-expander",
            Self::Multiband => "miso.multiband-compressor",
            Self::Limiter => "miso.true-peak-limiter",
            Self::SoftClip => "miso.soft-clip",
            Self::Transient => "miso.transient-shaper",
            Self::Delay => "miso.delay",
        }
    }
    const fn slot_name(self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::Comp => "comp",
            Self::Gate => "gate",
            Self::Multiband => "multiband",
            Self::Limiter => "limiter",
            Self::SoftClip => "soft-clip",
            Self::Transient => "transient",
            Self::Delay => "delay",
        }
    }
}

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

/// The fixture's own EQ, compressor and limiter (its real parameters), and every other launch
/// effect at its declared defaults.
struct Templates {
    eq: Effect,
    comp: Effect,
    limiter: Effect,
}

impl Templates {
    fn of(model: &SessionModel) -> Self {
        let lowered = model.lower_track(&model.tracks[0]);
        Self {
            eq: lowered.pre_insert[0].clone(),
            comp: lowered.pre_insert[1].clone(),
            limiter: lowered.post_insert[0].clone(),
        }
    }
    fn effect(&self, kind: Kind, id: &str) -> Effect {
        let mut effect = match kind {
            Kind::Eq => self.eq.clone(),
            Kind::Comp => self.comp.clone(),
            Kind::Limiter => self.limiter.clone(),
            _ => {
                let mut effect = self.comp.clone();
                effect.params.clear();
                effect
            }
        };
        effect.identity = EffectIdentity::Native {
            effect_id: sid(kind.native_id()),
        };
        effect.id = sid(id);
        effect.sidechain = SidechainDeclaration::None;
        effect
    }
}

/// Per-rack strips a generated console draws its common strip from.
const SIMD1: &[&[Kind]] = &[
    &[],
    &[Kind::Eq],
    &[Kind::Eq, Kind::Comp],
    &[Kind::Comp],
    &[Kind::Gate, Kind::Eq, Kind::Comp],
    &[Kind::Eq, Kind::Comp, Kind::Transient],
    &[Kind::Eq, Kind::SoftClip],
];
const DYNAMIC: &[&[Kind]] = &[
    &[],
    &[],
    &[Kind::SoftClip],
    &[Kind::Comp],
    &[Kind::Eq, Kind::SoftClip],
    &[Kind::Delay],
    &[Kind::Comp, Kind::Delay],
];
const SIMD2: &[&[Kind]] = &[
    &[],
    &[Kind::Limiter],
    &[Kind::Limiter],
    &[Kind::SoftClip, Kind::Limiter],
    &[Kind::Multiband, Kind::Limiter],
    &[Kind::Eq, Kind::Limiter],
];

const TAPS: [SendTap; 7] = [
    SendTap::Input,
    SendTap::PostInput,
    SendTap::InsertSend,
    SendTap::InsertReturn,
    SendTap::PreFader,
    SendTap::PostFader,
    SendTap::PostPan,
];

/// Place generated per-track chains in decision 12's shape, by the rule #1093 migrated the
/// checked-in documents with: a first or third chain that every track declares identically (IDs,
/// identity, quality, link mode), keyless and console-eligible, becomes `console.pre_insert` or
/// `console.post_insert`, each track's bypass and params in its entries; every other chain folds
/// into the track's inserts in chain order. Chain order, and so the rendered bits, never change.
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

/// Fold `console.pre_insert` into every track's inserts, ahead of its own: the placement a
/// per-track edit of that strip needs, since a console slot runs on every track (decision 12).
fn fold_pre_insert(model: &mut SessionModel) {
    let slots = core::mem::take(&mut model.console.pre_insert);
    let lowered: Vec<Vec<Effect>> = model
        .tracks
        .iter()
        .map(|track| {
            slots
                .iter()
                .zip(&track.console)
                .map(|(slot, entry)| Effect {
                    id: slot.slot.clone(),
                    identity: slot.identity.clone(),
                    quality: slot.quality,
                    bypass: entry.bypass,
                    link_mode: slot.link_mode,
                    params: entry.params.clone(),
                    sidechain: SidechainDeclaration::None,
                })
                .collect()
        })
        .collect();
    for (track, mut pre) in model.tracks.iter_mut().zip(lowered) {
        track.console.drain(..slots.len());
        pre.append(&mut track.inserts.effects);
        track.inserts.effects = pre;
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

/// The shape a generated console took, for the probe's per-shape tally.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Shape {
    /// Every track draws from one strip per rack, each slot dropped with probability `drop` per
    /// mille and, at a quarter of that rate, one extra effect inserted anywhere.
    Template {
        /// Per-mille probability that a track drops each slot of the strip.
        drop: u64,
    },
    /// Every track carries up to three random effects per rack in random order.
    Free,
}

/// One randomized, realistic console session: 1 to 40 tracks or a 64-track desk; a common strip
/// per rack with dropped and inserted slots, or free per-track racks; mono tracks, and all-mono
/// desks with asymmetric or symmetric strips; per-side input delays; bypassed effects; compressor
/// and gate sidechains from earlier tracks at every tap; and sends from every tap into up to two
/// submixes.
pub fn generate(seed: u64) -> (SessionModel, Shape) {
    let mut rng = Rng::new(seed);
    let tracks = match rng.below(20) {
        0..=11 => 1 + rng.below(24),
        12..=16 => 25 + rng.below(16),
        _ => 64,
    } as usize;
    let shape = if rng.chance(250) {
        Shape::Free
    } else {
        Shape::Template {
            drop: [0, 30, 80, 200, 400][rng.below(5) as usize],
        }
    };
    let mono = [0, 0, 300, 1000][rng.below(4) as usize];
    // An all-mono desk on an odd seed takes the symmetric mono strip, and every other desk the
    // intended fixture's asymmetric one. The armed leg needs both. On the symmetric strip the
    // collapse fires, so a wrong collapse is visible. On the asymmetric strip only chains that
    // gather the track input may arm (#970), and a later chain the arming wrongly admitted would
    // copy a left plane over a right one that an asymmetric stage upstream made differ. The
    // parity draws nothing from `rng`, so the choice moves no other draw.
    let symmetric = mono == 1000 && seed % 2 == 1;
    let base =
        parse_session_json(if symmetric { MONO } else { INTENDED }).expect("console fixture");
    let templates = Templates::of(&base);
    let strip = base.tracks[0].clone();
    let mut model = base;
    let mut chains = Vec::new();
    model.tracks.clear();
    model.routes.clear();
    model.submixes.clear();
    model.automation.clear();
    let racks = [
        SIMD1[rng.below(SIMD1.len() as u64) as usize],
        DYNAMIC[rng.below(DYNAMIC.len() as u64) as usize],
        SIMD2[rng.below(SIMD2.len() as u64) as usize],
    ];
    let submixes = rng.below(3) as usize;
    let main_out = || RouteDestination::OutputInput {
        output_id: sid("main-out"),
    };
    for submix in 0..submixes {
        model.submixes.push(Submix {
            id: sid(&format!("bus{submix}")),
        });
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
        if rng.chance(mono) {
            track.left_source_channel = 0;
            track.right_source_channel = 0;
        }
        if rng.chance(80) {
            track.builtins.left.delay_samples = 1 + rng.below(64) as u32;
        }
        if rng.chance(80) {
            track.builtins.right.delay_samples = 1 + rng.below(64) as u32;
        }
        let mut track_chains: [Vec<Effect>; 3] = Default::default();
        for (rack, template) in racks.into_iter().enumerate() {
            let mut kinds: Vec<Kind> = match shape {
                Shape::Free => (0..rng.below(4))
                    .map(|_| KINDS[rng.below(8) as usize])
                    .collect(),
                Shape::Template { drop } => {
                    let mut kinds: Vec<Kind> = template
                        .iter()
                        .copied()
                        .filter(|_| !rng.chance(drop))
                        .collect();
                    if rng.chance(drop / 4) {
                        let at = rng.below(kinds.len() as u64 + 1) as usize;
                        kinds.insert(at, KINDS[rng.below(8) as usize]);
                    }
                    kinds
                }
            };
            kinds.truncate(4);
            let effects: Vec<Effect> = kinds
                .iter()
                .enumerate()
                .map(|(slot, kind)| {
                    // Rack-qualified, so a chain folded into the inserts beside another rack's
                    // keeps unique effect IDs, and console slots stay unique across sections.
                    let mut effect =
                        templates.effect(*kind, &format!("{}{rack}{slot}", kind.slot_name()));
                    effect.bypass = rng.chance(60);
                    if matches!(kind, Kind::Comp | Kind::Gate) && index > 0 && rng.chance(70) {
                        effect.sidechain = SidechainDeclaration::Routed(Sidechain {
                            source: RouteSource::Track {
                                track_id: sid(&format!("ch{:02}", rng.below(index as u64))),
                                tap: TAPS[rng.below(7) as usize],
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
        let to_bus = submixes > 0 && rng.chance(250);
        let track_source = |tap| RouteSource::Track {
            track_id: sid(&id),
            tap,
        };
        if !to_bus || rng.chance(500) {
            model.routes.push(route(
                &format!("{id}-main"),
                track_source(SendTap::PostPan),
                main_out(),
                0.0,
            ));
        }
        if to_bus {
            let tap = TAPS[rng.below(7) as usize];
            model.routes.push(route(
                &format!("{id}-bus"),
                track_source(tap),
                RouteDestination::SubmixInput {
                    submix_id: sid(&format!("bus{}", rng.below(submixes as u64))),
                },
                -6.0,
            ));
        }
    }
    place(&mut model, chains);
    (model, shape)
}

// ---------------------------------------------------------------------------------------------
// The reproducers and the over-reach guards.

/// The standard 64-track console with the EQs of `tracks` removed: each runs `[comp]` beside
/// tracks running `[eq, comp]`, so its compressor sits one level before theirs. A track cannot drop
/// a console slot (decision 12), so the strip is folded into every track's inserts first.
pub fn sixty_four_track_console_less_eqs(tracks: &[usize]) -> SessionModel {
    console_less_eqs(INTENDED, tracks)
}

/// [`sixty_four_track_console_less_eqs`] on the mono desk, where the armed collapse fires.
pub fn mono_console_less_eqs(tracks: &[usize]) -> SessionModel {
    console_less_eqs(MONO, tracks)
}

fn console_less_eqs(fixture: &str, tracks: &[usize]) -> SessionModel {
    let mut model = parse_session_json(fixture).expect("console fixture");
    fold_pre_insert(&mut model);
    for index in tracks {
        let removed = model.tracks[*index].inserts.effects.remove(0);
        assert_eq!(
            removed.id.as_str(),
            "eq",
            "the fixture's first pre_insert slot is its EQ"
        );
    }
    model
}

/// The intended fixture cut to its first `tracks` tracks, with the routes of the rest removed.
fn intended_prefix(tracks: usize) -> SessionModel {
    let mut model = parse_session_json(INTENDED).expect("intended fixture");
    model.tracks.truncate(tracks);
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| {
        matches!(&route.source, RouteSource::Track { track_id, .. } if kept.contains(track_id))
    });
    model
}

/// The #962 probe's seed-412 shape: nine tracks of the intended strip, seven carrying `soft-clip`
/// at insert slot 0, one (`ch03`) carrying it at slot 1 behind an EQ, and one with no inserts.
pub fn ragged_dynamic_soft_clip() -> SessionModel {
    let mut model = intended_prefix(9);
    let templates = Templates::of(&model);
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.inserts.effects = match index {
            3 => vec![
                templates.effect(Kind::Eq, "eq"),
                templates.effect(Kind::SoftClip, "soft-clip"),
            ],
            8 => Vec::new(),
            _ => vec![templates.effect(Kind::SoftClip, "soft-clip")],
        };
    }
    model
}

/// Sixteen intended-strip tracks whose first-rack programs realign after a misaligned slot. The
/// programs differ per track, so they replace the `pre_insert` strip as inserts.
///
/// `ch00..=ch07` run `[gate, eq, comp, transient, soft-clip]` and fill one eight-lane group (two
/// four-lane groups) by themselves. `ch08..=ch15` alternate `A = [gate, comp, transient,
/// soft-clip]` and `B = [gate, eq, comp, soft-clip]`. In each A/B group the gate binds, the EQ and
/// the transient shaper are skipped by some lane, the compressor is misaligned (A reaches it at
/// rank 1, B at rank 2), and the soft-clip realigns (rank 3 on every lane) and must bind.
pub fn realigning_simd1_strips() -> SessionModel {
    let mut model = intended_prefix(16);
    let templates = Templates::of(&model);
    fold_pre_insert(&mut model);
    for (index, track) in model.tracks.iter_mut().enumerate() {
        let program: &[Kind] = match index {
            0..=7 => &[
                Kind::Gate,
                Kind::Eq,
                Kind::Comp,
                Kind::Transient,
                Kind::SoftClip,
            ],
            _ if index % 2 == 0 => &[Kind::Gate, Kind::Comp, Kind::Transient, Kind::SoftClip],
            _ => &[Kind::Gate, Kind::Eq, Kind::Comp, Kind::SoftClip],
        };
        track.inserts.effects = program
            .iter()
            .map(|kind| templates.effect(*kind, kind.slot_name()))
            .collect();
    }
    model
}

/// The #970 verification probe's reduced reproducer (see [`REDUCED_MONO`]).
pub fn reduced_mono_console() -> SessionModel {
    parse_session_json(REDUCED_MONO).expect("the reduced #970 reproducer parses")
}

/// Compile, bind and render `model` at every width, unarmed and armed, and require the scalar
/// plan's bits everywhere. Returns the unarmed outcomes in `WIDTHS` order, and the armed ones at
/// `[Simd4, Simd8]`.
fn assert_binds_and_renders_the_scalar_bits(
    name: &str,
    model: &SessionModel,
) -> ([Outcome; 3], [Outcome; 2]) {
    let outcomes =
        WIDTHS.map(|dispatch| compile_bind_render(model, dispatch, BLOCKS, Collapse::Unarmed));
    for (dispatch, outcome) in WIDTHS.iter().zip(&outcomes) {
        assert_eq!(outcome.compile, None, "{name} at {dispatch:?}: compile");
        assert_eq!(outcome.bind, None, "{name} at {dispatch:?}: bind");
        assert_eq!(
            outcome.cross_level_banks, 0,
            "{name} at {dispatch:?}: a bound effect bank spans levels"
        );
        assert!(
            outcome.pcm == outcomes[0].pcm,
            "{name} at {dispatch:?}: banking moved a rendered bit"
        );
    }
    let armed = [Backend::Simd4, Backend::Simd8]
        .map(|dispatch| compile_bind_render(model, dispatch, BLOCKS, Collapse::Armed));
    for (dispatch, armed) in [Backend::Simd4, Backend::Simd8].iter().zip(&armed) {
        assert_eq!(armed.bind, None, "{name} at {dispatch:?}, armed: bind");
        assert!(
            armed.pcm == outcomes[0].pcm,
            "{name} at {dispatch:?}: the armed mono collapse moved a rendered bit"
        );
    }
    assert!(
        audible(outcomes[0].pcm.as_ref()),
        "{name}: the scalar plan rendered audio"
    );
    (outcomes, armed)
}

/// The planned misaligned slots at `[Simd4, Simd8]`. They are a property of the plan, not of this
/// build's factories, so both widths are asserted on every host.
fn misaligned_slots(name: &str, outcomes: &[Outcome; 3], expected: [usize; 2]) {
    assert_eq!(
        [outcomes[1].misaligned_slots, outcomes[2].misaligned_slots],
        expected,
        "{name}: misaligned planned slots at [Simd4, Simd8], each left unbound"
    );
}

/// The ragged lanes of each misaligned slot at `[Simd4, Simd8]`: a property of the plan, so both
/// widths are asserted on every host.
fn ragged_lanes(name: &str, outcomes: &[Outcome; 3], simd4: &[&[usize]], simd8: &[&[usize]]) {
    let expected: [Vec<Vec<usize>>; 2] =
        [simd4, simd8].map(|slots| slots.iter().map(|lanes| lanes.to_vec()).collect());
    assert_eq!(
        [
            outcomes[1].early_lanes.clone(),
            outcomes[2].early_lanes.clone()
        ],
        expected,
        "{name}: the lanes that reach each misaligned slot early, at [Simd4, Simd8]"
    );
}

/// Effect banks the plan binds at this build's own width, where every launch factory in these
/// reproducers banks: `simd4` on the browser and AArch64 builds, `simd8` on `x86-64-v3`. A factory
/// declines a width its build was not compiled for (D4), which is why the other width's count is
/// not pinned. The `simd4` pins were read in a `wasm32` + `simd128` guest, never off an x86 run.
fn native_bank_count(name: &str, outcomes: &[Outcome; 3], simd4: usize, simd8: usize) {
    let (index, expected) = match Backend::current() {
        Backend::Simd4 => (1, simd4),
        Backend::Simd8 => (2, simd8),
        Backend::Scalar => return,
    };
    assert_eq!(
        outcomes[index].effect_banks,
        expected,
        "{name}: effect banks bound at {:?}",
        Backend::current()
    );
}

/// The ordinary trigger: one track of the standard console without its EQ, `ch00` (lane 0 of its
/// group).
///
/// Before #966 this refused at `Simd8` on a native host and at `Simd4` in the browser. The fix
/// unbinds exactly one bank: the compressor slot of `ch00`'s mixed group (22 planned at eight
/// lanes, 21 bound; 46 and 45 at four).
#[test]
fn the_sixty_four_track_console_less_one_eq_binds_at_every_width() {
    let name = "64-track console less ch00's EQ";
    let (outcomes, _) =
        assert_binds_and_renders_the_scalar_bits(name, &sixty_four_track_console_less_eqs(&[0]));
    // The cohort of 64 fills every group, so `ch00` shares a full group with seven `[eq, comp]`
    // tracks at eight lanes and with three at four, as lane 0 of both.
    misaligned_slots(name, &outcomes, [1, 1]);
    ragged_lanes(name, &outcomes, &[&[0]], &[&[0]]);
    native_bank_count(name, &outcomes, 45, 21);
}

/// Gate 1 with the ragged lane in the middle of its group. Members sort by active slot count and
/// then chunk, and each group lists its lanes by track id, so `ch60` lands at lane 4 of
/// `ch56..=ch63` at eight lanes and at lane 0 of `ch60..=ch63` at four.
#[test]
fn the_console_less_a_middle_lanes_eq_binds_at_every_width() {
    let name = "64-track console less ch60's EQ";
    let (outcomes, _) =
        assert_binds_and_renders_the_scalar_bits(name, &sixty_four_track_console_less_eqs(&[60]));
    misaligned_slots(name, &outcomes, [1, 1]);
    ragged_lanes(name, &outcomes, &[&[0]], &[&[4]]);
    native_bank_count(name, &outcomes, 45, 21);
}

/// Gate 1 with the ragged lane last in its group at both widths (`ch63`: lane 7 of `ch56..=ch63`,
/// lane 3 of `ch60..=ch63`).
#[test]
fn the_console_less_a_last_lanes_eq_binds_at_every_width() {
    let name = "64-track console less ch63's EQ";
    let (outcomes, _) =
        assert_binds_and_renders_the_scalar_bits(name, &sixty_four_track_console_less_eqs(&[63]));
    misaligned_slots(name, &outcomes, [1, 1]);
    ragged_lanes(name, &outcomes, &[&[3]], &[&[7]]);
    native_bank_count(name, &outcomes, 45, 21);
}

/// The #962 probe's seed-412 shape. Before #966 it refused at `Simd8` natively; the report that it
/// bound at `Simd4` came from an `x86-64-v3` host, whose soft-clip factory declines four lanes.
#[test]
fn a_cohort_lane_behind_an_extra_dynamic_eq_binds_at_every_width() {
    let name = "nine tracks, one soft-clip behind an EQ";
    let (outcomes, _) = assert_binds_and_renders_the_scalar_bits(name, &ragged_dynamic_soft_clip());
    // Eight soft-clip lanes: one full group at eight lanes, two at four, and `ch03`'s group is the
    // one whose soft-clip slot is misaligned either way.
    misaligned_slots(name, &outcomes, [1, 1]);
    native_bank_count(name, &outcomes, 6, 2);
}

/// Gate 1 on the mono desk: every track mono-mapped and symmetric, so the rescued plan is one the
/// host's collapse join arms, and the armed leg must fire the collapse at this build's own width
/// and still render the scalar bits.
#[test]
fn the_mono_console_less_one_eq_binds_and_collapses_at_every_width() {
    let name = "64-track mono console less ch00's EQ";
    let (outcomes, armed) =
        assert_binds_and_renders_the_scalar_bits(name, &mono_console_less_eqs(&[0]));
    misaligned_slots(name, &outcomes, [1, 1]);
    native_bank_count(name, &outcomes, 45, 21);
    let native = match Backend::current() {
        Backend::Simd4 => &armed[0],
        Backend::Simd8 => &armed[1],
        Backend::Scalar => return,
    };
    assert!(
        native.collapse[0] > 0,
        "{name}: the armed collapse fired at {:?}: {:?}",
        Backend::current(),
        native.collapse
    );
}

/// Effect banks the reduced mono console binds at four lanes: one compressor bank, for the full
/// group `t13, t14, t18, t6`, whose compressor every lane runs at rank 0. #1093 derived it from
/// the four-lane plan on `x86-64-v3` (where the factories decline four lanes); the AArch64 leg
/// runs it. At eight lanes no group is full and nothing banks.
const REDUCED_MONO_SIMD4_BANKS: usize = 1;

/// The #970 verification probe's reduced mono console, refused on the base tree as well: `t0`
/// runs `[eq, comp]` and the seven others `[comp]`, `t11` and `t13` with a soft-clip after it,
/// every track mono-mapped, so the armed leg performs the host's collapse join.
///
/// #1093 folded the document's divergent racks into each track's inserts, so `t11`'s and
/// `t13`'s soft-clip now extends their compressor chain instead of forming a later one, and the
/// plan shape moved: `t0` and `t11` each lead a cohort of their own and no planned slot is
/// misaligned. The #966 misaligned-compressor case is the four `console_less_eqs` tests' and the
/// #970 later-chain case `host-core`'s `collapse_arming.rs`'s; this one stays a bind-and-render
/// regression over the reduced mono desk.
#[test]
fn the_reduced_mono_console_from_the_970_probe_binds_at_every_width() {
    let name = "reduced mono console from the #970 probe";
    let (outcomes, _) = assert_binds_and_renders_the_scalar_bits(name, &reduced_mono_console());
    misaligned_slots(name, &outcomes, [0, 0]);
    native_bank_count(name, &outcomes, REDUCED_MONO_SIMD4_BANKS, 0);
}

/// The fix must not over-reach: lanes that all skip the same slot reach the next one together.
///
/// Nine tracks lack the EQ. `ch56..=ch63` fill one group by themselves, every lane runs the
/// compressor at rank 0, the compressors share a level, and that bank still binds. Only the
/// mixed group holding `ch00` loses its compressor bank. A binder that refused every slot
/// behind an identity slot, rather than every misaligned one, binds one bank fewer here.
#[test]
fn lanes_that_skip_the_same_slot_still_bank_it() {
    let name = "64-track console less nine EQs";
    let (outcomes, _) = assert_binds_and_renders_the_scalar_bits(
        name,
        &sixty_four_track_console_less_eqs(&[0, 56, 57, 58, 59, 60, 61, 62, 63]),
    );
    misaligned_slots(name, &outcomes, [1, 1]);
    native_bank_count(name, &outcomes, 43, 20);
}

/// The fix must not over-reach past the misaligned slot either: lanes that realign bank again.
///
/// In each A/B group of [`realigning_simd1_strips`] the compressor is misaligned and unbound, and
/// the soft-clip after it sits at one level on every lane and binds. Eight lanes: five banks for
/// the full group, gate and soft-clip for the A/B group, and a limiter bank at each of the two
/// levels the two chain lengths put `simd2` at, so 9. Four lanes: 10, 4 and 4, so 18. A binder
/// that stopped at the first misaligned slot of a group, or refused every slot behind an identity
/// slot, loses the soft-clip bank of every A/B group: 8 at eight lanes.
#[test]
fn a_slot_after_a_misaligned_one_realigns_and_still_banks() {
    let name = "sixteen tracks realigning after a misaligned compressor";
    let (outcomes, _) = assert_binds_and_renders_the_scalar_bits(name, &realigning_simd1_strips());
    misaligned_slots(name, &outcomes, [2, 1]);
    native_bank_count(name, &outcomes, 18, 9);
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

/// What the probe found for one seed.
#[derive(Debug, Default)]
struct SeedReport {
    shape: Option<Shape>,
    compile_refused: u64,
    /// Seeds with a misaligned planned slot, per `WIDTHS` entry.
    misaligned: [bool; 3],
    refused: Vec<(Backend, &'static str)>,
    rendered: bool,
    silent: bool,
    moved: Vec<(Backend, Collapse)>,
    collapsed: bool,
}

fn probe_seed(seed: u64, blocks: u64, render_all: bool) -> SeedReport {
    let (model, shape) = generate(seed);
    let mut report = SeedReport {
        shape: Some(shape),
        ..SeedReport::default()
    };
    for (index, dispatch) in WIDTHS.into_iter().enumerate() {
        let outcome = compile_bind_render(&model, dispatch, 0, Collapse::Unarmed);
        if outcome.compile.is_some() {
            report.compile_refused += 1;
            continue;
        }
        report.misaligned[index] = outcome.misaligned_slots > 0;
        if let Some(code) = outcome.bind {
            report.refused.push((dispatch, code));
        }
    }
    if !(report.misaligned.contains(&true) || render_all) || blocks == 0 {
        return report;
    }
    report.rendered = true;
    let scalar = compile_bind_render(&model, Backend::Scalar, blocks, Collapse::Unarmed).pcm;
    report.silent = !audible(scalar.as_ref());
    // The host arms only mono-mapped tracks, so on a desk without one the armed plan is the
    // unarmed plan and a second render of it would compare nothing new.
    let armable = model
        .tracks
        .iter()
        .any(|track| track.left_source_channel == track.right_source_channel);
    for dispatch in [Backend::Simd4, Backend::Simd8] {
        for collapse in [Collapse::Unarmed, Collapse::Armed] {
            if collapse == Collapse::Armed && !armable {
                continue;
            }
            let outcome = compile_bind_render(&model, dispatch, blocks, collapse);
            report.collapsed |= outcome.collapse[0] > 0;
            if outcome.pcm.is_some() && outcome.pcm != scalar {
                report.moved.push((dispatch, collapse));
            }
        }
    }
    report
}

/// Every plan compile accepts binds at every width, and every plan the fix touches renders the
/// scalar plan's bits with the mono collapse unarmed and armed.
///
/// The fixed range is seeds `0..64`; `PROBE_966_START`, `PROBE_966_COUNT` and `PROBE_966_BLOCKS`
/// widen it for a manual sweep, and `PROBE_966_RENDER_ALL=1` also renders the seeds the fix does
/// not touch. Every seed is compiled and bound at all three widths. A seed whose plan has a
/// misaligned slot at either SIMD width -- the only plans the fix changes -- is also rendered at
/// all three, unarmed, and at both SIMD widths armed as a host arms it, and compared bit for bit
/// with the scalar render. The audible count is asserted, because a comparison of two silent
/// renders proves nothing. The armed collapse must also fire on some rendered seed. The generator
/// keeps symmetric all-mono desks, where it fires and a wrong collapse would move a bit, and
/// asymmetric ones, where it must decline every chain that does not gather the track input (#970).
///
/// The probe must also *reach* the defect, or it proves nothing: the count of seeds with a
/// misaligned planned slot is asserted to be nonzero at both SIMD widths. That count is a property
/// of the plan, so it holds on a host whose factories decline the other width -- an `x86-64-v3`
/// build binds four-lane banks only for the limiter and the multiband compressor, and the browser
/// build binds them for every launch effect.
///
/// Seeds are spread over the host's cores and reported in seed order; each seed is independent.
#[test]
fn randomized_consoles_compile_bind_and_render_the_scalar_bits() {
    let start = env_u64("PROBE_966_START", 0);
    let count = env_u64("PROBE_966_COUNT", 64);
    let blocks = env_u64("PROBE_966_BLOCKS", BLOCKS);
    let render_all = env_u64("PROBE_966_RENDER_ALL", 0) != 0;
    let workers = std::thread::available_parallelism().map_or(1, |cores| cores.get().min(8));
    let mut reports: Vec<(u64, SeedReport)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers as u64)
            .map(|worker| {
                scope.spawn(move || {
                    (start + worker..start + count)
                        .step_by(workers)
                        .map(|seed| (seed, probe_seed(seed, blocks, render_all)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("probe worker"))
            .collect()
    });
    reports.sort_by_key(|(seed, _)| *seed);

    let mut refused = Vec::new();
    let mut moved = Vec::new();
    let mut silent = Vec::new();
    let mut compile_refused = 0;
    let mut misaligned = [0_u64; 3];
    let mut rendered = 0_u64;
    let mut collapsed = 0_u64;
    let mut by_shape: BTreeMap<Shape, [u64; 2]> = BTreeMap::new();
    for (seed, report) in reports {
        compile_refused += report.compile_refused;
        for (tally, hit) in misaligned.iter_mut().zip(report.misaligned) {
            *tally += u64::from(hit);
        }
        refused.extend(
            report
                .refused
                .iter()
                .map(|(dispatch, code)| (seed, *dispatch, *code)),
        );
        moved.extend(
            report
                .moved
                .iter()
                .map(|(dispatch, collapse)| (seed, *dispatch, *collapse)),
        );
        if report.silent {
            silent.push(seed);
        }
        rendered += u64::from(report.rendered);
        collapsed += u64::from(report.collapsed);
        let tally = by_shape
            .entry(report.shape.expect("every report names its shape"))
            .or_default();
        tally[0] += 1;
        tally[1] += u64::from(report.misaligned.contains(&true));
    }
    eprintln!(
        "seeds {start}..{}: compile refusals {compile_refused}, seeds with a misaligned slot \
         [scalar, simd4, simd8] {misaligned:?}, rendered {rendered}, of which the armed collapse \
         fired in {collapsed}, [seeds, seeds with a misaligned slot] by shape {by_shape:?}",
        start + count
    );
    assert!(
        refused.is_empty(),
        "bind refused compiled plans: {refused:?}"
    );
    assert!(moved.is_empty(), "banking moved rendered bits: {moved:?}");
    assert!(
        silent.is_empty(),
        "rendered silence, so compared nothing: {silent:?}"
    );
    assert_eq!(
        compile_refused, 0,
        "the generator emits sessions compile accepts"
    );
    assert!(
        misaligned[1] > 0 && misaligned[2] > 0,
        "the probe reaches a misaligned cohort slot at both SIMD widths: {misaligned:?}"
    );
    assert!(
        blocks == 0 || collapsed > 0,
        "the armed leg fires the mono collapse on some rendered seed"
    );
}
