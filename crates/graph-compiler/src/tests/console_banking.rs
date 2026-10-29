//! Issue #1098 (console strip S2): every console slot binds banked, for every track count, and the
//! banked render is the per-node render, track for track.
//!
//! * Gate 1, binding: [`every_console_slot_binds_banked_at_every_track_count`].
//! * Gate 2, class A: [`randomized_console_sessions_render_the_per_node_bits_on_every_track`], and
//!   the true-peak limiter as a padded `post_insert` slot at both widths,
//!   [`a_hot_padded_post_insert_limiter_renders_the_per_node_bits_at_both_widths`].
//! * Gate 3, the diagnostic: [`a_console_group_a_factory_declines_fails_the_compile`].
//! * Gate 4, #971 under padding: [`a_console_track_is_never_pooled_as_stereo_to_fill_a_bank`].
//!
//! Gate 6, the padded bank's member charge, is `bank_padding`'s. The per-node reference is always
//! the test-only `Scalar` oracle (#1059): the one backend on which a console slot may render per
//! node, because it is the reference.

use super::*;
use rack_compiler::CohortPoolClass;

/// The track counts decision 12's gate names: a lone track, remainders of one to five at eight
/// lanes, and full groups beside partial ones.
const TRACK_COUNTS: [usize; 6] = [1, 3, 5, 9, 10, 13];

/// The console eligibility list (`effect_compiler::CONSOLE_ELIGIBLE_EFFECTS`), as kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Eq,
    Comp,
    Gate,
    SoftClip,
    Transient,
    Limiter,
    Multiband,
    DeEsser,
    /// Not console-eligible: an insert that banks nowhere and so renders per node (issue #1899).
    Delay,
}

const ELIGIBLE: [Kind; 7] = [
    Kind::Eq,
    Kind::Comp,
    Kind::Gate,
    Kind::SoftClip,
    Kind::Transient,
    Kind::Limiter,
    Kind::Multiband,
];

/// Every launch effect that can pad an insert cohort (#888): the console eligibility list and the
/// de-esser, which is registered but not console-eligible (#1920).
const INSERT_KINDS: [Kind; 8] = [
    Kind::Eq,
    Kind::Comp,
    Kind::Gate,
    Kind::SoftClip,
    Kind::Transient,
    Kind::Limiter,
    Kind::Multiband,
    Kind::DeEsser,
];

impl Kind {
    const fn native_id(self) -> &'static str {
        match self {
            Self::Eq => "miso.parametric-eq",
            Self::Comp => "miso.compressor",
            Self::Gate => "miso.gate-expander",
            Self::SoftClip => "miso.soft-clip",
            Self::Transient => "miso.transient-shaper",
            Self::Limiter => "miso.true-peak-limiter",
            Self::Multiband => "miso.multiband-compressor",
            Self::DeEsser => "miso.de-esser",
            Self::Delay => "miso.delay",
        }
    }
    const fn name(self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::Comp => "comp",
            Self::Gate => "gate",
            Self::SoftClip => "soft-clip",
            Self::Transient => "transient",
            Self::Limiter => "limiter",
            Self::Multiband => "multiband",
            Self::DeEsser => "de-esser",
            Self::Delay => "delay",
        }
    }
}

/// `kind` with ID `id`: the mono fixture's own EQ, compressor and limiter with their real
/// parameters, and every other eligible effect at its declared defaults. Keyless and unbypassed.
fn effect(kind: Kind, id: &str) -> session::Effect {
    /// The fixture's strip, parsed once: `[eq, comp, limiter]`.
    static STRIP: std::sync::OnceLock<[session::Effect; 3]> = std::sync::OnceLock::new();
    let [eq, comp, limiter] = STRIP.get_or_init(|| {
        let fixture =
            parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_MONO_FIXTURE).expect("mono fixture");
        let lowered = fixture.lower_track(&fixture.tracks[0]);
        [
            lowered.pre_insert[0].clone(),
            lowered.pre_insert[1].clone(),
            lowered.post_insert[0].clone(),
        ]
    });
    let mut effect = match kind {
        Kind::Eq => eq.clone(),
        Kind::Comp => comp.clone(),
        Kind::Limiter => limiter.clone(),
        _ => {
            let mut effect = comp.clone();
            effect.params.clear();
            effect
        }
    };
    effect.identity = EffectIdentity::Native {
        effect_id: StableId::parse(kind.native_id()).expect("native id"),
    };
    effect.id = StableId::parse(id).expect("effect id");
    effect.sidechain = SidechainDeclaration::None;
    effect.bypass = false;
    effect
}

/// How one track is fed: its peak gain, and whether its two lanes read one stream.
#[derive(Clone, Copy, Debug)]
struct Feed {
    gain: f32,
    stereo: bool,
    /// Whether one input sample, in the third block, is a NaN, which the input section sanitizes on
    /// its lane alone (D7); a bank that let it reach a bank-mate would move that track's bits.
    poison: bool,
}

/// One track of a generated console session.
#[derive(Clone, Debug)]
struct TrackShape {
    feed: Feed,
    /// The track's inserts, in chain order.
    inserts: Vec<Kind>,
    /// One per console slot, `pre_insert` then `post_insert`.
    bypass: Vec<bool>,
    /// Which of [`VARIANTS`]' parameter values the track's EQ, compressor and limiter take.
    variant: usize,
}

/// Per-track parameter values, so that bank-mates differ and a lane bound with another member's
/// request moves bits: `(EQ gain dB, compressor threshold dB, limiter ceiling dB)`. Both channels
/// of an EQ take the same gain, which keeps a mono track collapse-eligible.
const VARIANTS: [(f32, f32, f32); 4] = [
    (-7.5, -6.0, -0.5),
    (-3.0, -12.0, -1.0),
    (3.0, -18.0, -3.0),
    (6.0, -24.0, -6.0),
];

/// `effect` with [`VARIANTS`]`[variant]`'s value on its one varied parameter, if it has one.
fn vary(mut effect: session::Effect, kind: Kind, variant: usize) -> session::Effect {
    let (eq_gain, threshold, ceiling) = VARIANTS[variant];
    let (parameter, value) = match kind {
        Kind::Eq => (4, eq_gain),
        Kind::Comp => (1, threshold),
        Kind::Limiter => (1, ceiling),
        _ => return effect,
    };
    for param in effect
        .params
        .iter_mut()
        .filter(|param| param.parameter_id == parameter)
    {
        param.value = value;
    }
    effect
}

/// The console both sections declare, in slot order.
#[derive(Clone, Debug)]
struct Console {
    pre: Vec<Kind>,
    post: Vec<Kind>,
}

impl Console {
    fn slots(&self) -> usize {
        self.pre.len() + self.post.len()
    }
}

/// The session `tracks` and `console` describe, on the mono fixture's tracks (`ch00`...): a track
/// is mono-mapped unless its feed is stereo, every track carries every console slot with its own
/// bypass, and its own inserts after `pre_insert`. Slot IDs carry their section and insert IDs
/// their position, so every ID is unique where it must be.
fn console_session(console: &Console, tracks: &[TrackShape]) -> session::SessionModel {
    let mut model = mono_fixture_with_tracks(tracks.len());
    let chains = tracks
        .iter()
        .map(|shape| {
            let section = |kinds: &[Kind], prefix: &str| -> Vec<session::Effect> {
                kinds
                    .iter()
                    .map(|kind| {
                        let id = format!("{prefix}-{}", kind.name());
                        vary(effect(*kind, &id), *kind, shape.variant)
                    })
                    .collect()
            };
            let mut pre = section(&console.pre, "pre");
            let mut post = section(&console.post, "post");
            for (effect, bypass) in pre.iter_mut().chain(post.iter_mut()).zip(&shape.bypass) {
                effect.bypass = *bypass;
            }
            let inserts = shape
                .inserts
                .iter()
                .enumerate()
                .map(|(index, kind)| {
                    let id = format!("insert{index}-{}", kind.name());
                    vary(effect(*kind, &id), *kind, shape.variant)
                })
                .collect();
            [pre, inserts, post]
        })
        .collect();
    for (track, shape) in model.tracks.iter_mut().zip(tracks) {
        if shape.feed.stereo {
            track.right_source_channel = 1;
        }
    }
    place(&mut model, chains);
    assert_eq!(
        (
            model.console.pre_insert.len(),
            model.console.post_insert.len()
        ),
        (console.pre.len(), console.post.len()),
        "the strip settles into the console"
    );
    model
}

/// SplitMix64's finaliser.
const fn mix(counter: u64) -> u64 {
    let mut z = counter.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Uniform noise in `[-1, 1)` on a 2^-23 grid, for `stream` at `frame`: exact on every target.
fn noise(stream: u64, frame: u64) -> f32 {
    (mix((stream << 40) ^ frame) >> 40) as f32 * (1.0 / 8_388_608.0) - 1.0
}

/// The input frame a poisoned feed replaces with a NaN: in the third block, past every onset.
const POISONED_FRAME: u64 = 300;

/// A track input: seeded noise at the track's gain, the same stream on both lanes of a mono track.
struct NoiseSource {
    stream: u64,
    feed: Feed,
}
impl GraphRuntimeProcessor for NoiseSource {
    fn retained_bytes(&self) -> u64 {
        0
    }

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
            let frame = block.first_sample + index as u64;
            *left = if self.feed.poison && frame == POISONED_FRAME {
                f32::NAN
            } else {
                self.feed.gain * noise(2 * self.stream, frame)
            };
            *right = if self.feed.stereo {
                self.feed.gain * noise(2 * self.stream + 1, frame)
            } else {
                *left
            };
        }
        Ok(())
    }
}

/// A sample's bits with every NaN folded to one quiet NaN (decision 10).
fn folded(sample: f32) -> u32 {
    if sample.is_nan() {
        0x7fc0_0000
    } else {
        sample.to_bits()
    }
}

/// What one render saw: the session output and each track's post-pan stage, NaNs folded.
struct Rendered {
    output: Vec<u32>,
    tracks: Vec<Vec<u32>>,
    /// `bank_collapse_counters()`: `[blocks rendered collapsed, chains armed with a prefix]`.
    collapse: [u64; 2],
    units: Vec<engine::realtime::PlanUnitEligibility>,
    /// Blocks rendered.
    blocks: u64,
}

/// Bind `artifact` with a [`NoiseSource`] per track and a `BitRecorder` at each track's post-pan
/// stage, arm the mono collapse on the mono-mapped tracks as `host-core` arms it, and render
/// `blocks` blocks, or four past the plan's output latency if that is more.
fn render(artifact: PreparedGraphBuiltinsArtifact, tracks: &[TrackShape], blocks: u64) -> Rendered {
    render_arm(artifact, tracks, blocks, false)
}

/// [`render`], with every chain's collapse forced off when `forced_off`: the dual oracle.
fn render_arm(
    artifact: PreparedGraphBuiltinsArtifact,
    tracks: &[TrackShape],
    blocks: u64,
    forced_off: bool,
) -> Rendered {
    let frames = artifact.envelope().quantum.0 as usize;
    let blocks = blocks.max(artifact.report().output_latency.0.div_ceil(frames as u64) + 4);
    let (mut plan, sinks) = bind_arm(artifact, tracks, forced_off);
    let mut output = Vec::new();
    let mut pcm = vec![0.0_f32; frames * 2];
    for block in 0..blocks {
        plan.render(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime {
                absolute_sample: block * frames as u64,
            },
        )
        .expect("render");
        output.extend(pcm.iter().map(|sample| folded(*sample)));
    }
    let collapse = plan.bank_collapse_counters();
    let units = plan.unit_eligibility();
    drop(plan);
    Rendered {
        collapse,
        units,
        blocks,
        output,
        tracks: sinks
            .iter()
            .map(|sink| {
                sink.lock()
                    .expect("recorder")
                    .iter()
                    .flat_map(|(left, right)| [left, right])
                    .map(|bits| folded(f32::from_bits(*bits)))
                    .collect()
            })
            .collect(),
    }
}

/// Bind `artifact` as [`render_arm`] does, armed (or forced off) and not yet rendered: the plan and
/// each track's post-matrix recorder.
fn bind_arm(
    artifact: PreparedGraphBuiltinsArtifact,
    tracks: &[TrackShape],
    forced_off: bool,
) -> (PreparedRenderPlan, Vec<BitSink>) {
    let envelope = artifact.envelope();
    let index_of = |track_id: &str| -> usize {
        track_id
            .strip_prefix("ch")
            .and_then(|index| index.parse().ok())
            .expect("chNN")
    };
    let nodes = artifact
        .external_binding_nodes()
        .map(|node| {
            let processor: Box<dyn GraphRuntimeProcessor> = match node {
                GraphNodeId::TrackStage {
                    track_id,
                    stage: TrackStage::Input,
                } => {
                    let index = index_of(track_id.as_str());
                    Box::new(NoiseSource {
                        stream: index as u64 + 1,
                        feed: tracks[index].feed,
                    })
                }
                _ => Box::new(IdentityBinding),
            };
            GraphNodeBinding::new(node.clone(), processor)
        })
        .collect();
    let sinks: Vec<_> = tracks.iter().map(|_| BitSink::default()).collect();
    let observers = sinks
        .iter()
        .enumerate()
        .map(|(index, sink)| {
            GraphNodeObserverBinding::new(
                track_node(&format!("ch{index:02}"), TrackStage::PostMatrix),
                index as u64 + 1,
                Box::new(BitRecorder(Arc::clone(sink))),
            )
        })
        .collect();
    let mut plan = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers,
        })
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
        .plan;
    plan.arm_mono_collapse(&|track: &str| !tracks[index_of(track)].feed.stereo);
    plan.force_mono_collapse_off(forced_off);
    (plan, sinks)
}

/// Every track's bits, and the output's, are the oracle's; `what` names the case.
fn assert_renders_the_oracle(banked: &Rendered, oracle: &Rendered, what: &str) {
    for (index, (banked, oracle)) in banked.tracks.iter().zip(&oracle.tracks).enumerate() {
        assert!(!oracle.is_empty(), "{what}: ch{index:02} was observed");
        assert!(
            banked == oracle,
            "{what}: ch{index:02}'s bits differ from the per-node oracle's"
        );
    }
    assert!(
        banked.output == oracle.output,
        "{what}: the session output differs from the per-node oracle's"
    );
}

/// `model` at `dispatch` against the launch registry, or the compile's diagnostics.
fn compile_at(
    model: &session::SessionModel,
    dispatch: Backend,
) -> Result<PreparedGraphBuiltinsArtifact, GraphDiagnosticSet> {
    try_compile_console_model_at(
        model,
        1_098,
        &[],
        dispatch,
        launch_native_effect_registry().expect("launch registry"),
    )
}

/// The dependency level of every node of `artifact`'s graph.
fn node_levels(artifact: &PreparedGraphBuiltinsArtifact) -> BTreeMap<GraphNodeId, u64> {
    artifact
        .graph()
        .dependency_levels
        .iter()
        .flat_map(|level| {
            level
                .nodes
                .iter()
                .cloned()
                .map(move |node| (node, level.level))
        })
        .collect()
}

/// Gate 1: every console slot binds banked, at every track count the gate names, and forms
/// exactly one padded group per (pool class, placement level) (issue #2136; per dependency level
/// before the windows planner).
///
/// Two consoles, each on every track with its own mix of bypassed slots:
///
/// * all six eligible effects -- `pre_insert` the EQ, compressor, gate/expander and transient
///   shaper, `post_insert` the soft-clip and the limiter. The gate/expander, soft-clip and
///   transient shaper declare no channel symmetry (the contract's default), so every track that
///   carries them pools stereo;
/// * the collapse-capable strip, EQ and compressor then the limiter, on which the even tracks,
///   mono-mapped, pool mono and the odd ones stereo.
///
/// The tracks carry zero, one or two inserts in turn, so `post_insert` sits at up to three
/// dependency levels (H2). At the host's width (eight lanes here, four on the AArch64 leg):
///
/// * no console node renders per node;
/// * each console rack forms one group per (pool class, placement level of its first slot),
///   `ceil(n / W)` of them for `n` tracks there, and binds every slot of every group;
/// * `post_insert` is placed at one level per pool, whatever the insert count (#2136);
/// * each track pools as its own class, mono-symmetric exactly when its feed is mono (nothing is
///   demoted, #971 under padding), and bypassed lanes share banks with unbypassed ones. Since
///   #1873 every console-eligible effect carries a channel-symmetry witness, so this holds on both
///   consoles; a declining witness is
///   [`a_mono_track_whose_witness_declines_pools_as_stereo`]'s.
///
/// Red if console groups stop padding (a remainder renders per node), if a padded group is bound
/// for fewer slots than it plans, if the planner merged groups across a placement level or a
/// class, if `post_insert` stays split by insert count (a level bucket), or if a bypassed lane
/// left its bank.
#[test]
fn every_console_slot_binds_banked_at_every_track_count() {
    let lanes = BankWidth::for_backend(host_dispatch())
        .expect("a vector host")
        .lanes() as usize;
    let every_effect = Console {
        pre: vec![Kind::Eq, Kind::Comp, Kind::Gate, Kind::Transient],
        post: vec![Kind::SoftClip, Kind::Multiband, Kind::Limiter],
    };
    assert_eq!(
        ELIGIBLE.map(Kind::native_id),
        effect_compiler::CONSOLE_ELIGIBLE_EFFECTS,
        "the kinds are the console eligibility list"
    );
    assert!(
        ELIGIBLE
            .iter()
            .all(|kind| every_effect.pre.contains(kind) || every_effect.post.contains(kind)),
        "every eligible effect is a slot"
    );
    let strip = Console {
        pre: vec![Kind::Eq, Kind::Comp],
        post: vec![Kind::Limiter],
    };
    let mut pools = BTreeSet::new();
    for console in [every_effect, strip] {
        for n in TRACK_COUNTS {
            let tracks: Vec<TrackShape> = (0..n)
                .map(|index| TrackShape {
                    feed: Feed {
                        gain: 0.5,
                        stereo: index % 2 == 1,
                        poison: false,
                    },
                    inserts: [vec![], vec![Kind::Eq], vec![Kind::Eq, Kind::Comp]][index % 3]
                        .clone(),
                    bypass: (0..console.slots())
                        .map(|slot| (index + slot) % 3 == 0)
                        .collect(),
                    variant: index % VARIANTS.len(),
                })
                .collect();
            let model = console_session(&console, &tracks);
            let artifact = compile_at(&model, host_dispatch())
                .unwrap_or_else(|diagnostics| panic!("{n} tracks: {diagnostics:?}"));
            let report = &artifact.report().rack_cohorts;
            let levels = node_levels(&artifact);
            assert!(
                report.scalar_in(RackLocation::Simd1).is_empty()
                    && report.scalar_in(RackLocation::Simd2).is_empty(),
                "{n} tracks: a console slot renders per node"
            );
            for (rack, location, slots) in [
                (RackId::Simd1, RackLocation::Simd1, console.pre.len()),
                (RackId::Simd2, RackLocation::Simd2, console.post.len()),
            ] {
                // Each chain's pool class, from the group that holds it, and the level of its first
                // slot, from the graph.
                let mut buckets: BTreeMap<(CohortPoolClass, u64), usize> = BTreeMap::new();
                for (chain, nodes) in report.chains.iter().filter(|(chain, _)| chain.rack == rack) {
                    let group = report
                        .plan
                        .groups
                        .iter()
                        .find(|group| group.members.contains(&Some(chain.clone())))
                        .unwrap_or_else(|| panic!("{n} tracks: {chain:?} is in a group"));
                    let index: usize = chain.track_id[2..].parse().expect("chNN");
                    let expected_class = if !tracks[index].feed.stereo {
                        CohortPoolClass::MonoSymmetricAtPrepare
                    } else {
                        CohortPoolClass::Stereo
                    };
                    assert_eq!(
                        group.class, expected_class,
                        "{n} tracks: {chain:?} pools as its own class"
                    );
                    pools.insert(group.class);
                    // Issue #2136: the chain's placement level, which the planner pooled it at;
                    // a chain is never placed below its first slot's dependency level.
                    assert!(
                        group.level >= levels[&GraphNodeId::Effect(nodes[0].clone())],
                        "{n} tracks: {chain:?} placed below its level"
                    );
                    *buckets.entry((group.class, group.level)).or_default() += 1;
                }
                let groups: usize = buckets.values().map(|count| count.div_ceil(lanes)).sum();
                assert_eq!(
                    report.groups_in(location).count(),
                    groups,
                    "{n} tracks, {location:?}: one group per ceil(n / W) of each (class, level): \
                 {buckets:?}"
                );
                assert_eq!(
                    report.bound_slots_in(location).count(),
                    groups * slots,
                    "{n} tracks, {location:?}: every slot of every group binds"
                );
                if location == RackLocation::Simd2 {
                    // Issue #2136: the insert counts put `post_insert` at up to three dependency
                    // levels, and the tracks' level windows share the deepest, so each pool
                    // places it at one level.
                    let pools_here: BTreeSet<CohortPoolClass> =
                        buckets.keys().map(|(class, _)| *class).collect();
                    assert_eq!(
                        buckets.len(),
                        pools_here.len(),
                        "{n} tracks: post_insert is placed at one level per pool, whatever the \
                         insert count: {buckets:?}"
                    );
                }
            }
            // A bypassed lane shares its bank with an unbypassed one somewhere in the plan.
            let bypassed = |node: &EffectNodeId| {
                let index: usize = node.track_id.as_str()[2..].parse().expect("chNN");
                let slot = console
                    .pre
                    .iter()
                    .map(|kind| format!("pre-{}", kind.name()))
                    .chain(
                        console
                            .post
                            .iter()
                            .map(|kind| format!("post-{}", kind.name())),
                    )
                    .position(|slot| slot == node.effect_id.as_str())
                    .expect("a console slot");
                tracks[index].bypass[slot]
            };
            if n > 1 {
                assert!(
                    report.bound_slots.iter().any(|bound| {
                        bound.members.iter().any(&bypassed)
                            && bound.members.iter().any(|member| !bypassed(member))
                    }),
                    "{n} tracks: a bank holds bypassed and unbypassed lanes"
                );
            }
        }
    }
    assert_eq!(pools.len(), 2, "both pool classes bind console banks");
}

/// A mono-fed track whose console EQ has different left and right gains pools as stereo, beside a
/// symmetric mono track that pools mono-symmetric: the pool class reads each prepared effect's
/// `DESIGNED` witness, not the feed alone. Red if the class ignored the effects' witnesses.
#[test]
fn a_mono_track_whose_witness_declines_pools_as_stereo() {
    let console = Console {
        pre: vec![Kind::Eq],
        post: vec![],
    };
    let tracks: Vec<TrackShape> = (0..2)
        .map(|_| TrackShape {
            feed: Feed {
                gain: 0.5,
                stereo: false,
                poison: false,
            },
            inserts: vec![],
            bypass: vec![false],
            variant: 0,
        })
        .collect();
    let mut model = console_session(&console, &tracks);
    for param in model.tracks[0].console[0]
        .params
        .iter_mut()
        .filter(|param| param.parameter_id == 4 && param.channel == ParameterChannel::Right)
    {
        param.value += 1.0;
    }
    let artifact = compile_at(&model, host_dispatch()).expect("compile");
    let report = &artifact.report().rack_cohorts;
    for (track, expected) in [
        ("ch00", CohortPoolClass::Stereo),
        ("ch01", CohortPoolClass::MonoSymmetricAtPrepare),
    ] {
        let group = report
            .plan
            .groups
            .iter()
            .find(|group| {
                group
                    .members
                    .iter()
                    .flatten()
                    .any(|chain| chain.track_id == track)
            })
            .unwrap_or_else(|| panic!("{track} is in a group"));
        assert_eq!(group.class, expected, "{track}");
    }
}

/// Seeds the randomized differential renders per PR; `PROBE_1098_SEEDS` widens a manual sweep.
const SEEDS: u64 = 48;

/// xorshift64*: deterministic, dependency-free, and the same on every target.
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
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

/// One randomized console session: a track count the gate names; a console of one to four
/// distinct eligible effects split over the two sections; per track zero to two inserts, a random
/// bypass on each console slot, its own parameter values ([`VARIANTS`]), a mono or stereo feed, a
/// gain that is silent, quiet, or hot (+6 to +18 dBFS peak, so every limiter and compressor works
/// its gain path), and now and then a NaN input sample, which the input section's bank sanitizes on
/// its lane alone. Every third seed then gives most tracks one shared two-slot insert chain,
/// cycling through [`INSERT_KINDS`] by seed, so partial insert cohorts at and above the padding
/// threshold form and bind padded for every kind that can pad one (#888). Those draws come last,
/// so every other seed's session is unchanged.
fn generate(seed: u64) -> (Console, Vec<TrackShape>) {
    let mut rng = Rng::new(seed);
    let n = TRACK_COUNTS[rng.below(TRACK_COUNTS.len() as u64) as usize];
    let mut kinds = ELIGIBLE.to_vec();
    for index in (1..kinds.len()).rev() {
        kinds.swap(index, rng.below(index as u64 + 1) as usize);
    }
    kinds.truncate(1 + rng.below(4) as usize);
    let split = rng.below(kinds.len() as u64 + 1) as usize;
    let console = Console {
        pre: kinds[..split].to_vec(),
        post: kinds[split..].to_vec(),
    };
    let mut tracks: Vec<TrackShape> = (0..n)
        .map(|_| TrackShape {
            feed: Feed {
                gain: match rng.below(8) {
                    0 => 0.0,
                    1 | 2 => 0.1,
                    _ => 2.0 + 6.0 * rng.below(1000) as f32 / 1000.0,
                },
                stereo: rng.chance(500),
                poison: rng.chance(100),
            },
            inserts: (0..rng.below(3))
                .map(|_| ELIGIBLE[rng.below(ELIGIBLE.len() as u64) as usize])
                .collect(),
            bypass: (0..console.slots()).map(|_| rng.chance(300)).collect(),
            variant: rng.below(VARIANTS.len() as u64) as usize,
        })
        .collect();
    // Every third seed only, and drawn last, so every other seed is the session main generates.
    if seed.is_multiple_of(3) {
        let cycle = seed / 3;
        let kind =
            |offset: u64| INSERT_KINDS[((cycle + offset) % INSERT_KINDS.len() as u64) as usize];
        let shared = [kind(0), kind(3)];
        for track in &mut tracks {
            if rng.chance(800) {
                track.inserts = shared.to_vec();
            }
        }
    }
    (console, tracks)
}

/// Gate 2, randomized: on generated console sessions every track renders, at the host's width,
/// the bits the per-node `Scalar` oracle renders, NaNs folded (decision 10).
///
/// Banking may couple lanes' cost, never their bits (decision 12, "Coupling rule"). A padded bank
/// runs clone lanes on `+0.0` beside its members, a bank-wide fast path and D7 recovery see every
/// lane, and a bypassed lane runs the wet path behind the shunt. Each generated session mixes, in
/// one bank, hot, quiet and silent lanes, mono and stereo pools, bypassed and unbypassed lanes,
/// and padded remainders at every count the gate names, so a whole-bank decision that is not
/// bit-neutral per lane moves some track's bits here. Each track is read at its post-pan stage,
/// so a defect cannot hide in the session output's sum. Every seed must bind every console slot
/// (the compile refuses otherwise), and the sweep must reach a padded bank with a bypassed lane,
/// a padded mono bank and a padded insert bank (#888).
///
/// Red (measured, #1098 evidence): a limiter bank that skips the gain path for every lane when its
/// first lane is quiet; a padded lane fed anything but `+0.0`; the rack scattering a padded lane.
/// The padded insert banks (#888) put the same clauses in the dynamic rack, where a clone lane
/// leaking into an active one moves an insert's bits.
#[test]
fn randomized_console_sessions_render_the_per_node_bits_on_every_track() {
    const BLOCKS: u64 = 12;
    let seeds = std::env::var("PROBE_1098_SEEDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(SEEDS);
    let lanes = BankWidth::for_backend(host_dispatch())
        .expect("a vector host")
        .lanes() as usize;
    let (mut padded_with_bypass, mut padded_mono, mut poisoned_beside_a_mate) = (0, 0, 0);
    // Padded insert banks per kind (#888), keyed by the kind's name in the insert's id.
    let mut padded_inserts = [0_usize; INSERT_KINDS.len()];
    for seed in 0..seeds {
        let (console, tracks) = generate(seed);
        let model = console_session(&console, &tracks);
        let what = format!(
            "seed {seed}: {} tracks, console {:?} / {:?}",
            tracks.len(),
            console.pre,
            console.post
        );
        let banked = compile_at(&model, host_dispatch())
            .unwrap_or_else(|diagnostics| panic!("{what}: {diagnostics:?}"));
        let report = &banked.report().rack_cohorts;
        assert!(
            report.scalar_in(RackLocation::Simd1).is_empty()
                && report.scalar_in(RackLocation::Simd2).is_empty(),
            "{what}: a console slot renders per node"
        );
        for bound in report.bound_slots.iter().filter(|bound| {
            matches!(
                report.plan.groups[bound.group].rack,
                RackLocation::Simd1 | RackLocation::Simd2
            ) && bound.members.len() < lanes
        }) {
            let group = &report.plan.groups[bound.group];
            padded_mono += usize::from(group.class == CohortPoolClass::MonoSymmetricAtPrepare);
            let bypassed = bound.members.iter().any(|member| {
                let index: usize = member.track_id.as_str()[2..].parse().expect("chNN");
                let lowered = model.lower_track(&model.tracks[index]);
                lowered
                    .pre_insert
                    .iter()
                    .chain(&lowered.post_insert)
                    .any(|effect| effect.id.as_str() == member.effect_id.as_str() && effect.bypass)
            });
            padded_with_bypass += usize::from(bypassed);
        }
        for bound in report.bound_slots.iter().filter(|bound| {
            report.plan.groups[bound.group].rack == RackLocation::Dynamic
                && bound.members.len() < lanes
        }) {
            let id = bound.members[0].effect_id.as_str();
            let (_, name) = id.split_once('-').expect("insertN-kind");
            let kind = INSERT_KINDS
                .iter()
                .position(|kind| kind.name() == name)
                .unwrap_or_else(|| panic!("{what}: an insert of no known kind: {id}"));
            padded_inserts[kind] += 1;
        }
        let poisoned = |member: &EffectNodeId| {
            tracks[member.track_id.as_str()[2..]
                .parse::<usize>()
                .expect("chNN")]
            .feed
            .poison
        };
        poisoned_beside_a_mate += report
            .bound_slots
            .iter()
            .filter(|bound| bound.members.len() > 1 && bound.members.iter().any(poisoned))
            .count();
        let banked = render(banked, &tracks, BLOCKS);
        let oracle = render(
            compile_at(&model, Backend::Scalar).expect("the Scalar oracle compiles"),
            &tracks,
            BLOCKS,
        );
        assert_renders_the_oracle(&banked, &oracle, &what);
    }
    println!(
        "#1098 randomized console differential: {seeds} seeds, padded banks with a bypassed lane \
         {padded_with_bypass}, padded mono banks {padded_mono}, padded insert banks per kind \
         {padded_inserts:?}, effect banks holding a poisoned track beside a bank-mate \
         {poisoned_beside_a_mate}"
    );
    assert!(
        padded_with_bypass > 0 && padded_mono > 0 && poisoned_beside_a_mate > 0,
        "the sweep reaches a padded bank with a bypassed lane, a padded mono bank and a poisoned \
         track beside a bank-mate"
    );
    for (kind, count) in INSERT_KINDS.iter().zip(padded_inserts) {
        assert!(
            count > 0,
            "the sweep reaches a padded insert bank of the {} (#888)",
            kind.name()
        );
    }
}

/// Gate 2, named (#1091 verdict L3): the true-peak limiter as a padded `post_insert` slot, with
/// mixed bypass, fed hot enough that its gain path runs, renders the per-node bits on every track
/// at eight lanes and at four.
///
/// The console is the limiter alone, in `post_insert`; the tracks carry zero, one or two EQ
/// inserts in turn, so the limiter sits at up to three dependency levels; since #2136 their level
/// windows place it at one level per pool. Tracks alternate mono and stereo, each with its own ceiling ([`VARIANTS`]), and tracks
/// `2..=3`, `6..=7` and so on bypass the limiter, a pattern that mixes bypassed and unbypassed
/// lanes inside the groups. Every track is fed seeded noise at +6
/// to +18 dBFS peak, far above the limiter's ceiling, and the gain path is shown to run: the
/// limiter moves every unbypassed track's bits against the same session with every lane bypassed.
///
/// Every vector width of the build runs: an `x86-64-v3` build binds the limiter at four lanes as
/// well as eight, and a four-lane build has no wider width to name.
///
/// Red if a limiter bank's whole-bank decision (its uniform-body gate, its bank-wide silence and
/// screen tests, the rack's shunt selection) moves a lane's bits with its bank-mates, padded lanes
/// included: restoring the dry signal on every lane of a bank that has one bypassed lane turns it
/// red (#1098 evidence).
#[test]
fn a_hot_padded_post_insert_limiter_renders_the_per_node_bits_at_both_widths() {
    const BLOCKS: u64 = 12;
    let console = Console {
        pre: Vec::new(),
        post: vec![Kind::Limiter],
    };
    let mut mixed_padded = BTreeSet::new();
    for n in TRACK_COUNTS {
        let tracks: Vec<TrackShape> = (0..n)
            .map(|index| TrackShape {
                feed: Feed {
                    gain: 2.0 + index as f32 * 0.5,
                    stereo: index % 2 == 0,
                    poison: false,
                },
                inserts: [vec![], vec![Kind::Eq], vec![Kind::Eq, Kind::Eq]][index % 3].clone(),
                bypass: vec![(index / 2) % 2 == 1],
                variant: index % VARIANTS.len(),
            })
            .collect();
        let model = console_session(&console, &tracks);
        let oracle = render(
            compile_at(&model, Backend::Scalar).expect("the Scalar oracle compiles"),
            &tracks,
            BLOCKS,
        );
        // The gain path runs: against every lane bypassed, every unbypassed track moves.
        let mut all_bypassed = tracks.clone();
        for track in &mut all_bypassed {
            track.bypass = vec![true];
        }
        let dry = render(
            compile_at(&console_session(&console, &all_bypassed), Backend::Scalar)
                .expect("the Scalar oracle compiles"),
            &all_bypassed,
            BLOCKS,
        );
        for (index, track) in tracks.iter().enumerate() {
            assert_eq!(
                oracle.tracks[index] != dry.tracks[index],
                !track.bypass[0],
                "{n} tracks: ch{index:02}'s limiter works its gain path unless bypassed"
            );
        }
        for &dispatch in Backend::VECTOR.iter().rev() {
            let width = BankWidth::for_backend(dispatch).expect("a vector width");
            let what = format!("{n} tracks at {dispatch:?}");
            let banked = compile_at(&model, dispatch)
                .unwrap_or_else(|diagnostics| panic!("{what}: {diagnostics:?}"));
            let report = &banked.report().rack_cohorts;
            assert!(
                report.scalar_in(RackLocation::Simd2).is_empty(),
                "{what}: the limiter never renders per node"
            );
            let lanes = width.lanes() as usize;
            let padded_bypassed = report.bound_slots_in(RackLocation::Simd2).any(|bound| {
                bound.members.len() < lanes
                    && bound.members.iter().any(|member| {
                        let index: usize = member.track_id.as_str()[2..].parse().expect("chNN");
                        tracks[index].bypass[0]
                    })
            });
            if padded_bypassed {
                mixed_padded.insert(width.lanes());
            }
            assert_renders_the_oracle(&render(banked, &tracks, BLOCKS), &oracle, &what);
        }
    }
    // Which track counts leave a padded remainder holding a bypassed lane depends on the width
    // and, since #2136, on the placement; every width meets one.
    assert_eq!(
        mixed_padded.len(),
        Backend::VECTOR.len(),
        "at every width some padded limiter bank carries a bypassed lane"
    );
}

/// A limiter whose bank bind declines every padded request, and binds full ones as the launch
/// limiter does.
struct DecliningPaddedLimiter {
    delegate: Arc<dyn NativeEffectFactory>,
}
impl NativeEffectFactory for DecliningPaddedLimiter {
    fn descriptor(&self) -> &'static effect_contract::EffectDescriptor {
        self.delegate.descriptor()
    }
    fn prepared_inline_bytes(&self) -> u64 {
        self.delegate.prepared_inline_bytes()
    }
    fn prepared_undeclared_heap_bytes(&self, quality: &effect_contract::QualityDescriptor) -> u64 {
        self.delegate.prepared_undeclared_heap_bytes(quality)
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<effect_contract::PreparedEffect, EffectPrepareError> {
        self.delegate.prepare(request)
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<effect_contract::PreparedEffectBank>, EffectPrepareError> {
        request.validate_shape()?;
        if request.is_padded() {
            return Ok(None);
        }
        self.delegate.bind_homogeneous_bank(request)
    }
}

/// Gate 3: a console group a factory declines fails the compile with `console.slot.unbanked`, on
/// a vector backend, and never yields a per-node plan; the `Scalar` oracle's exemption reaches no
/// vector backend.
///
/// `W + 1` mono tracks carry the limiter as their one console slot, against a limiter that
/// declines every padded bank. The full group binds and the padded one does not. At the host's
/// width, and at `Simd8` whatever the host, the compile is refused, and at the host's width the
/// diagnostic names the slot, the pool class and the dependency level of the group that did not
/// bind. The same session and registry compile at `Scalar`, where nothing banks. The same strip as
/// every track's insert compiles at the host's width, and its remainder renders per node: the
/// guarantee is the console's alone.
///
/// Red if the guarantee is dropped (the per-node plan compiles), if it reaches the `Scalar`
/// oracle, if it reaches inserts, or if the diagnostic stops naming the group.
#[test]
fn a_console_group_a_factory_declines_fails_the_compile() {
    let lanes = BankWidth::for_backend(host_dispatch())
        .expect("a vector host")
        .lanes() as usize;
    let console = Console {
        pre: Vec::new(),
        post: vec![Kind::Limiter],
    };
    let tracks: Vec<TrackShape> = (0..=lanes)
        .map(|_| TrackShape {
            feed: Feed {
                gain: 0.5,
                stereo: false,
                poison: false,
            },
            inserts: Vec::new(),
            bypass: vec![false],
            variant: 0,
        })
        .collect();
    let model = console_session(&console, &tracks);
    let registry = NativeEffectRegistry::new([Box::new(DecliningPaddedLimiter {
        delegate: launch_native_effect_registry()
            .expect("launch registry")
            .get_shared_ascii(Kind::Limiter.native_id())
            .expect("launch limiter"),
    }) as Box<dyn NativeEffectFactory>])
    .expect("declining registry");

    let scalar = try_compile_console_model_at(&model, 1_098, &[], Backend::Scalar, &registry)
        .unwrap_or_else(|diagnostics| panic!("the Scalar oracle is exempt: {diagnostics:?}"));
    assert_eq!(scalar.graph().prepared_bank_count(), 0);
    let remainder = format!("ch{lanes:02}");
    let level = node_levels(&scalar)[&GraphNodeId::Effect(EffectNodeId {
        track_id: gid(&remainder),
        rack: RackId::Simd2,
        effect_id: gid("post-limiter"),
    })];

    // The host's width and the widest this build has: both eight lanes on x86-64-v3 (#1112).
    let widest = *Backend::VECTOR.last().expect("a vector backend");
    for dispatch in [host_dispatch(), widest] {
        let Err(diagnostics) =
            try_compile_console_model_at(&model, 1_098, &[], dispatch, &registry)
        else {
            panic!("{dispatch:?}: a declined console group rendered per node");
        };
        let diagnostics = diagnostics.diagnostics();
        assert_eq!(
            diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code)
                .collect::<Vec<_>>(),
            ["console.slot.unbanked"],
            "{dispatch:?}"
        );
        if dispatch == host_dispatch() {
            assert_eq!(
                diagnostics[0].path,
                format!("$.console.post_insert[slot=post-limiter].bank[pool=mono,level={level}]"),
                "the diagnostic names the slot, the pool and the level of the group"
            );
        }
    }

    let mut inserts = model.clone();
    fold_console_into_inserts(&mut inserts);
    let folded = try_compile_console_model_at(&inserts, 1_098, &[], host_dispatch(), &registry)
        .unwrap_or_else(|diagnostics| {
            panic!("an insert remainder renders per node: {diagnostics:?}")
        });
    assert_eq!(
        folded
            .report()
            .rack_cohorts
            .scalar_in(RackLocation::Dynamic)
            .len(),
        1,
        "the insert remainder renders per node"
    );
}

/// Gate 4, #971 under padding: a mono track that carries a console slot is never pooled as stereo,
/// even where moving it would fill a stereo insert group.
///
/// `ch00..=ch{W}` are mono and `ch{W+1}..` stereo, `k - 1` of them, where `k` is the insert padding
/// threshold (#888), so the stereo insert remainder binds nothing. Every track carries the limiter
/// as its console slot and the same two inserts, EQ then compressor. Under #971's rule as it stood,
/// `ch{W}` strands -- every group it sits in is a partial mono one -- and moving it lifts the
/// stereo insert group to `k` members, which pads, two banks, at the price of one padded limiter
/// bank, so the move bound more banks and was kept, and the mono track lost its collapse. Since
/// #1098 a console group always binds, so a track that carries a console slot banks somewhere and
/// is never stranded (`banks::stranded_mono_tracks`): `ch{W}` stays mono in a padded limiter bank,
/// the insert remainders render per node, and the session binds five banks at any width.
///
/// The console-free counterpart keeps #971: the same chain as inserts alone, where the stranded
/// track banks nowhere, moves `ch{W}` to the stereo pool and binds both insert groups (the stereo
/// one padded), six banks.
///
/// Red if a padded console group strands its members again (`ch{W}` moves and six banks bind), or
/// if the demotion is retired for console-free sessions too (`ch{W}` stays mono there).
#[test]
fn a_console_track_is_never_pooled_as_stereo_to_fill_a_bank() {
    const BLOCKS: u64 = 12;
    let width = BankWidth::for_backend(host_dispatch()).expect("a vector host");
    let lanes = width.lanes() as usize;
    let k = crate::banks::min_padded_insert_members(width);
    let console = Console {
        pre: Vec::new(),
        post: vec![Kind::Limiter],
    };
    let tracks: Vec<TrackShape> = (0..lanes + k)
        .map(|index| TrackShape {
            feed: Feed {
                gain: 2.0,
                stereo: index > lanes,
                poison: false,
            },
            inserts: vec![Kind::Eq, Kind::Comp],
            bypass: vec![false],
            variant: 0,
        })
        .collect();
    let mono: BTreeSet<String> = (0..=lanes).map(|index| format!("ch{index:02}")).collect();
    let stranded = format!("ch{lanes:02}");

    let model = console_session(&console, &tracks);
    let artifact = compile_at(&model, host_dispatch()).expect("the console session compiles");
    assert_eq!(
        pooled_tracks(&artifact)[0],
        mono,
        "every mono track stays in the mono pool"
    );
    assert_eq!(
        artifact.graph().prepared_bank_count(),
        5,
        "three limiter banks (mono full, mono padded, stereo padded) and the mono insert group's two"
    );
    let report = &artifact.report().rack_cohorts;
    let limiter_group = report
        .bound_groups_in(RackLocation::Simd2)
        .find(|group| {
            group
                .members
                .iter()
                .flatten()
                .any(|chain| chain.track_id == stranded)
        })
        .expect("the stranded track's limiter banks");
    assert_eq!(
        limiter_group.class,
        CohortPoolClass::MonoSymmetricAtPrepare,
        "in a padded mono bank"
    );
    assert_renders_the_oracle(
        &render(artifact, &tracks, BLOCKS),
        &render(
            compile_at(&model, Backend::Scalar).expect("the Scalar oracle compiles"),
            &tracks,
            BLOCKS,
        ),
        "the console session",
    );

    let mut inserts = model.clone();
    fold_console_into_inserts(&mut inserts);
    let folded = compile_at(&inserts, host_dispatch()).expect("the insert session compiles");
    let mut moved = mono.clone();
    moved.remove(&stranded);
    assert_eq!(
        pooled_tracks(&folded)[0],
        moved,
        "with no console, the stranded track still moves to the stereo pool"
    );
    assert_eq!(
        folded.graph().prepared_bank_count(),
        6,
        "and completes the stereo insert group: three banks per pool"
    );
}

/// Issue #1368 gate 2: inserts of the lane-link effects at mixed detector link modes share one
/// bank per slot per group, and every track renders its per-node bits.
///
/// `2 W` stereo tracks carry the compressor and the gate/expander as console slots and again as
/// inserts; track `i`'s two inserts take link mode `i mod 3`, so every group of `W` lanes mixes
/// DualMono, Maximum and Average. The inserts form one stereo group per `W` tracks at one level,
/// and each of its two slots binds as one bank, so no insert renders per node.
///
/// Red if the program key still carries the real link mode (the mixed cohort splits and inserts
/// render per node), or if a bank applies one lane's mode to its bank-mates (the bits differ from
/// the per-node oracle's).
#[test]
fn inserts_at_mixed_link_modes_share_one_bank_per_slot() {
    const BLOCKS: u64 = 12;
    use session::LinkMode;
    const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];
    let lanes = BankWidth::for_backend(host_dispatch())
        .expect("a vector host")
        .lanes() as usize;
    let console = Console {
        pre: vec![Kind::Comp, Kind::Gate],
        post: Vec::new(),
    };
    let tracks: Vec<TrackShape> = (0..2 * lanes)
        .map(|index| TrackShape {
            feed: Feed {
                gain: 2.0,
                stereo: true,
                poison: false,
            },
            inserts: vec![Kind::Comp, Kind::Gate],
            bypass: vec![false; 2],
            variant: index % VARIANTS.len(),
        })
        .collect();
    let mut model = console_session(&console, &tracks);
    for (index, track) in model.tracks.iter_mut().enumerate() {
        for effect in &mut track.inserts.effects {
            effect.link_mode = LINKS[index % LINKS.len()];
        }
    }
    let artifact = compile_at(&model, host_dispatch()).expect("the session compiles");
    let report = &artifact.report().rack_cohorts;
    assert!(
        report.scalar_in(RackLocation::Dynamic).is_empty(),
        "an insert at a mixed link mode renders per node"
    );
    assert_eq!(
        report.bound_slots_in(RackLocation::Dynamic).count(),
        2 * 2,
        "one bank per insert slot per group of {lanes} tracks"
    );
    assert_renders_the_oracle(
        &render(artifact, &tracks, BLOCKS),
        &render(
            compile_at(&model, Backend::Scalar).expect("the Scalar oracle compiles"),
            &tracks,
            BLOCKS,
        ),
        "mixed-link inserts",
    );
}

/// Issue #987: a later chain of a mono strip is armed on its producers' rendering, and collapses.
///
/// Sixteen tracks on the collapse-capable console (EQ and compressor, then the limiter), every
/// fourth one stereo; the even tracks carry a banked EQ insert, so their strip is three chains
/// (the `pre_insert` chain, the insert chain and the `post_insert` chain), and the odd tracks a
/// delay insert, which banks nowhere, so theirs is two chains around a per-node op (issue #1899).
/// Armed and forced off, every track's post-matrix bits and the session output agree, the
/// forced-off arm never collapses, and every collapsible chain whose lanes are all mono -- the
/// later ones included, behind a bank or behind the symmetric delay -- is armed and collapses on
/// every block; no stereo chain is armed.
///
/// Red if a later chain stays unarmed (the #970 rule alone arms only the chains that gather the
/// input; the #987 table alone gives the chain behind the delay no row), or if the producer table
/// or the per-block term drops a lane whose producer collapsed.
#[test]
fn a_later_mono_chain_arms_on_its_producers_and_renders_the_dual_bits() {
    let console = Console {
        pre: vec![Kind::Eq, Kind::Comp],
        post: vec![Kind::Limiter],
    };
    let tracks: Vec<TrackShape> = (0..16)
        .map(|index| TrackShape {
            feed: Feed {
                gain: 0.5,
                stereo: index % 4 == 3,
                poison: false,
            },
            inserts: if index % 2 == 0 {
                vec![Kind::Eq]
            } else {
                vec![Kind::Delay]
            },
            bypass: vec![false; console.slots()],
            variant: index % VARIANTS.len(),
        })
        .collect();
    let model = console_session(&console, &tracks);
    let compile = || compile_at(&model, host_dispatch()).expect("compile");
    assert_eq!(
        compile()
            .report()
            .rack_cohorts
            .scalar_in(RackLocation::Dynamic)
            .len(),
        8,
        "every odd track's delay renders per node"
    );
    let armed = render_arm(compile(), &tracks, 12, false);
    let dual = render_arm(compile(), &tracks, 12, true);
    assert_eq!(dual.collapse[0], 0, "the forced-off arm never collapses");
    assert_renders_the_oracle(&armed, &dual, "later mono chains, armed against forced off");
    // Every collapsible chain whose lanes all render mono tracks.
    let mono: Vec<_> = armed
        .units
        .iter()
        .filter(|unit| {
            unit.banked
                && unit.upstream_of_seam_stages > 0
                && !unit.lane_tracks.is_empty()
                && unit.lane_tracks.iter().all(|track| {
                    let index: usize = track[2..].parse().expect("chNN");
                    !tracks[index].feed.stereo
                })
        })
        .collect();
    let mono_chains = mono.len() as u64;
    let gathering = mono.iter().filter(|unit| unit.gathers_track_input).count() as u64;
    assert!(
        mono.iter()
            .all(|unit| unit.gathers_track_input != unit.armed_on_input),
        "every later mono chain reports itself armed on its input, and no gathering chain does"
    );
    assert!(
        !armed
            .units
            .iter()
            .any(|unit| unit.armed_on_input && !mono.contains(&unit)),
        "no chain outside the mono chains is armed on its input"
    );
    assert!(
        mono_chains > gathering && gathering > 0,
        "the strip splits into later mono chains ({mono_chains} mono chains, {gathering} gather)"
    );
    assert_eq!(
        armed.collapse,
        [mono_chains * armed.blocks, mono_chains],
        "every mono chain, the later ones too, is armed and collapses on every block"
    );
}

/// What [`Double`] does to the launch factory it wraps (issue #1899).
#[derive(Clone, Copy)]
enum Role {
    /// Nothing: the launch factory.
    Forward,
    /// Prepares an [`EpisodeEffect`] around the launch instance and binds no bank.
    Episode,
    /// Binds a [`PassBank`] in place of the launch bank.
    PassBank,
}

/// A launch factory, as [`Role`] changes it.
struct Double {
    delegate: Arc<dyn NativeEffectFactory>,
    role: Role,
}
impl NativeEffectFactory for Double {
    fn descriptor(&self) -> &'static effect_contract::EffectDescriptor {
        self.delegate.descriptor()
    }
    fn prepared_inline_bytes(&self) -> u64 {
        self.delegate
            .prepared_inline_bytes()
            .max(size_of::<EpisodeEffect>() as u64)
    }
    fn prepared_undeclared_heap_bytes(&self, quality: &effect_contract::QualityDescriptor) -> u64 {
        self.delegate.prepared_undeclared_heap_bytes(quality)
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<effect_contract::PreparedEffect, EffectPrepareError> {
        let mut prepared = self.delegate.prepare(request)?;
        if matches!(self.role, Role::Episode) {
            prepared.processor = Box::new(EpisodeEffect {
                inner: prepared.processor,
                quantum: u64::from(prepared.metadata.quantum),
                asymmetric: false,
            });
        }
        Ok(prepared)
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<effect_contract::PreparedEffectBank>, EffectPrepareError> {
        match self.role {
            Role::Forward => self.delegate.bind_homogeneous_bank(request),
            Role::Episode => Ok(None),
            Role::PassBank => Ok(self.delegate.bind_homogeneous_bank(request)?.map(|bank| {
                effect_contract::PreparedEffectBank {
                    processor: Box::new(PassBank),
                    metadata: bank.metadata,
                }
            })),
        }
    }
}

/// The blocks over which an [`EpisodeEffect`] treats its two channels differently.
const EPISODE: core::ops::Range<u64> = 2..4;

/// A per-node effect whose two channels part for [`EPISODE`] and then work alike again: over
/// those blocks it inverts its left input before the wrapped instance runs, and its witness
/// declines, as an effect's does while one channel's designed words differ; outside them it is the
/// wrapped instance with the wrapped witness. What the episode left in the instance's state stays.
struct EpisodeEffect {
    inner: Box<dyn PreparedNativeEffect>,
    quantum: u64,
    /// Whether the last block was in the episode.
    asymmetric: bool,
}
impl PreparedNativeEffect for EpisodeEffect {
    fn retained_bytes(&self) -> u64 {
        core::mem::size_of_val(&*self.inner) as u64 + self.inner.retained_bytes()
    }
    fn reset(&mut self, kind: effect_contract::ResetKind) {
        self.inner.reset(kind);
    }
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        self.asymmetric = EPISODE.contains(&(block.first_sample / self.quantum));
        if self.asymmetric {
            for sample in block.left.iter_mut() {
                *sample = -*sample;
            }
        }
        self.inner.process(block)
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), effect_contract::StatePayloadError> {
        self.inner.snapshot_state_payload(output)
    }
    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: effect_contract::StatePayloadInput<'_>,
    ) -> Result<(), effect_contract::StatePayloadError> {
        self.inner
            .restore_state_payload(state_layout_version, input)
    }
    fn channel_symmetry(&self) -> bool {
        !self.asymmetric && self.inner.channel_symmetry()
    }
}

/// A stateless pass-through bank that collapses and, holding no state, proves its channels agree
/// (`BankStage::channels_agree`) on the first block its input agrees again. A stateful proving bank
/// would have seen its own state differ while its input differed, and its proof would hide the
/// op's.
struct PassBank;
impl PreparedNativeEffectBank for PassBank {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn reset(&mut self, _kind: effect_contract::ResetKind) {}
    fn reset_lane(&mut self, _lane: usize, _kind: effect_contract::ResetKind) {}
    fn process_bank(
        &mut self,
        block: effect_contract::EffectBankProcessBlock<'_>,
    ) -> effect_contract::BankProcessReport {
        effect_contract::BankProcessReport::empty(block.width)
    }
    fn snapshot_track_state_payload(
        &self,
        _track_index: u32,
        _output: StatePayloadOutput<'_>,
    ) -> Result<(), effect_contract::StatePayloadError> {
        Ok(())
    }
    fn restore_track_state_payload(
        &mut self,
        _track_index: u32,
        _state_layout_version: u32,
        _input: effect_contract::StatePayloadInput<'_>,
    ) -> Result<(), effect_contract::StatePayloadError> {
        Ok(())
    }
    fn lane_channel_symmetry(&self, _lane: usize) -> bool {
        true
    }
    fn supports_mono_collapse(&self) -> bool {
        true
    }
    fn channels_agree(&self) -> bool {
        true
    }
}

/// The launch EQ, compressor and delay, and the gate as `role` makes it.
fn double_registry(delay: Role) -> NativeEffectRegistry {
    let launch = launch_native_effect_registry().expect("launch registry");
    let double = |kind: Kind, role| {
        Box::new(Double {
            delegate: launch
                .get_shared_ascii(kind.native_id())
                .expect("launch factory"),
            role,
        }) as Box<dyn NativeEffectFactory>
    };
    NativeEffectRegistry::new([
        double(Kind::Eq, Role::Forward),
        double(Kind::Comp, Role::Forward),
        double(Kind::Delay, delay),
        double(Kind::Gate, Role::PassBank),
    ])
    .expect("double registry")
}

/// `count` mono tracks on the console `[EQ] / [gate]`, each with `insert` as its one insert, which
/// renders per node between the two chains.
fn split_strip_tracks(count: usize, insert: Kind) -> (Console, Vec<TrackShape>) {
    let console = Console {
        pre: vec![Kind::Eq],
        post: vec![Kind::Gate],
    };
    let tracks = (0..count)
        .map(|index| TrackShape {
            feed: Feed {
                gain: 0.5,
                stereo: false,
                poison: false,
            },
            inserts: vec![insert],
            bypass: vec![false; console.slots()],
            variant: index % VARIANTS.len(),
        })
        .collect();
    (console, tracks)
}

/// Issue #1899, the sticky state term: a later chain behind a per-node effect whose channels
/// parted once stays dual, though the effect's input and words are symmetric again.
///
/// Eight mono tracks, `[EQ] -> delay (5 ms, per node) -> [gate]`, where the delay is an
/// [`EpisodeEffect`] and the gate a [`PassBank`], so the later chain can prove its own agreement
/// back. Over [`EPISODE`] the delay inverts its left input and declines; after it, the first chain
/// still collapses (identical planes) and the delay's words agree, but its line holds the episode
/// and echoes it into the left plane only. Armed and forced off render the same bits.
///
/// Red if the delay's fact is this block's input and words alone (the sticky `&&` dropped in
/// `Runtime::execute`): the later chain re-engages after the episode and copies the left echo over
/// the right one. No host-core probe sees that mutation: none has an episode followed by
/// symmetric words, and a stateful proving bank behind the op still holds a state that differs.
#[test]
fn a_chain_behind_a_per_node_effect_whose_channels_parted_stays_dual() {
    let (console, tracks) = split_strip_tracks(8, Kind::Delay);
    let mut model = console_session(&console, &tracks);
    for track in &mut model.tracks {
        track.inserts.effects[0].params = vec![EffectParam {
            parameter_id: 1,
            channel: ParameterChannel::Both,
            unit: ParameterUnit::Milliseconds,
            value: 5.0,
        }];
    }
    let registry = double_registry(Role::Episode);
    let compile = || {
        try_compile_console_model_at(&model, 1_899, &[], host_dispatch(), &registry)
            .unwrap_or_else(|diagnostics| panic!("compile: {diagnostics:?}"))
    };
    let armed = render_arm(compile(), &tracks, 16, false);
    let dual = render_arm(compile(), &tracks, 16, true);
    assert_eq!(dual.collapse[0], 0, "the forced-off arm never collapses");
    let after = dual.tracks[0].len() / dual.blocks as usize * EPISODE.end as usize;
    assert!(
        dual.tracks[0][after..]
            .as_chunks::<2>()
            .0
            .iter()
            .any(|[left, right]| left != right),
        "the delay echoes the episode into one plane after it ends"
    );
    assert!(
        armed
            .units
            .iter()
            .any(|unit| unit.banked && unit.armed_on_input),
        "the chain behind the delay is armed on it"
    );
    assert!(armed.collapse[0] > 0, "the first chain collapses");
    assert_renders_the_oracle(
        &armed,
        &dual,
        "a chain behind an episode, armed against dual",
    );
}

/// Issue #1899, admission: a sidechained per-node effect is no producer, because its words and its
/// main input say nothing about the key it reads.
///
/// `W` mono tracks, `[EQ] -> compressor (per node) -> [gate]`, each compressor keyed by the input of
/// one stereo track. The compressor's main input is identical on both planes and its words agree,
/// but the key is not, so it writes two different planes. Armed and forced off render the same
/// bits, and no chain behind a keyed compressor is armed on its input.
///
/// Red if admission drops the sidechain clause (`input_producer_table`): the later chain collapses
/// and copies the left plane over the right one.
#[test]
fn a_chain_behind_a_sidechained_per_node_effect_is_not_armed_on_it() {
    let lanes = BankWidth::for_backend(host_dispatch())
        .expect("a vector host")
        .lanes() as usize;
    let (console, mut tracks) = split_strip_tracks(lanes + 1, Kind::Comp);
    let key = tracks.len() - 1;
    tracks[key].feed.stereo = true;
    tracks[key].inserts.clear();
    let mut model = console_session(&console, &tracks);
    for track in &mut model.tracks[..key] {
        track.inserts.effects[0].sidechain = SidechainDeclaration::Routed(Sidechain {
            source: RouteSource::Track {
                track_id: StableId::parse(&format!("ch{key:02}")).expect("key track"),
                tap: session::SendTap::Input,
            },
            port_id: StableId::parse("sidechain-in").expect("port id"),
        });
    }
    let registry = double_registry(Role::Forward);
    let compile = || {
        try_compile_console_model_at(&model, 1_899, &[], host_dispatch(), &registry)
            .unwrap_or_else(|diagnostics| panic!("compile: {diagnostics:?}"))
    };
    let armed = render_arm(compile(), &tracks, 12, false);
    let dual = render_arm(compile(), &tracks, 12, true);
    assert_eq!(dual.collapse[0], 0, "the forced-off arm never collapses");
    assert!(
        dual.tracks[0]
            .as_chunks::<2>()
            .0
            .iter()
            .any(|[left, right]| left != right),
        "the keyed compressor writes two different planes"
    );
    assert!(
        !armed.units.iter().any(|unit| unit.armed_on_input),
        "no chain is armed on a keyed compressor"
    );
    assert!(armed.collapse[0] > 0, "the first chains collapse");
    assert_renders_the_oracle(
        &armed,
        &dual,
        "a chain behind a keyed compressor, armed against dual",
    );
}

/// [`try_compile_console_model_at`] at the host's width, with every delay insert given a live
/// control lane, so it binds as a live-controlled per-node instance a plan swap can move.
fn compile_with_live_delays(
    model: &session::SessionModel,
    registry: &NativeEffectRegistry,
) -> PreparedGraphBuiltinsArtifact {
    let session = compile_session(
        model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("compiled console model");
    let builtins = prepare_session_builtins(
        &session,
        &[],
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
        },
    )
    .expect("prepared console builtins");
    let mut effects = prepare_native_session_effects(
        &session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 22,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("prepared console effects");
    for entry in effects
        .entries
        .iter_mut()
        .filter(|entry| entry.factory.descriptor().id.as_str() == Kind::Delay.native_id())
    {
        let (_writer, lane) = effect_contract::effect_control_lane(
            effect_contract::EffectLaneSpec {
                shape: effect_contract::EffectLaneShape::from_descriptor(
                    entry.factory.descriptor(),
                    0,
                ),
                bypass: entry.initial_bypass,
                observation_depth: 0,
                link: None,
            },
            &engine::realtime::LiveValuesSuperseded::new(),
        )
        .expect("a lane");
        entry.control = Some(Box::new(lane));
    }
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        dispatch: host_dispatch(),
        plan_id: 1_899,
        effects,
        builtins,
        caps: integration_caps(),
        gain_liveness: crate::GainLiveness::FIXED,
    })
    .unwrap_or_else(|failure| panic!("compile: {:?}", failure.diagnostics))
}

/// Issue #1899 R3, the swap carry: a per-node instance's agreement fact travels with its state.
///
/// The sticky rig's session (`[EQ] -> live-controlled delay (5 ms, per node) -> [gate]`, the delay
/// an [`EpisodeEffect`], the gate a [`PassBank`]), rendered by a predecessor plan through
/// [`EPISODE`] and two blocks past it, then by a successor that takes every delay instance by
/// pointer (`LiveControlEffect::move_from`) and nothing else. The successor's later chains are
/// fresh, so they agree and would collapse at once; the delay's line holds the episode's echo, so
/// the successor must start from the predecessor's `false`. Armed and forced off render the same
/// bits.
///
/// Red if the carry hands the successor `true` (node to node, or lane to node): the fresh later
/// chain collapses on the swap block and copies the left echo over the right one.
#[test]
fn a_swap_carries_a_per_node_effects_agreement_with_its_state() {
    const SWAP: u64 = 6;
    const BLOCKS: u64 = 12;
    let (console, tracks) = split_strip_tracks(8, Kind::Delay);
    let mut model = console_session(&console, &tracks);
    for track in &mut model.tracks {
        track.inserts.effects[0].params = vec![EffectParam {
            parameter_id: 1,
            channel: ParameterChannel::Both,
            unit: ParameterUnit::Milliseconds,
            value: 5.0,
        }];
    }
    let registry = double_registry(Role::Episode);
    // Each track's post-matrix bits over the predecessor's blocks, then the successor's.
    let run = |forced_off: bool| -> Vec<Vec<u32>> {
        let (mut before, before_sinks) = bind_arm(
            compile_with_live_delays(&model, &registry),
            &tracks,
            forced_off,
        );
        let (mut after, after_sinks) = bind_arm(
            compile_with_live_delays(&model, &registry),
            &tracks,
            forced_off,
        );
        let lanes = |plan: &mut PreparedRenderPlan| {
            graph::bank_effect_lanes(plan)
                .expect("a graph plan")
                .into_iter()
                .filter(|(_, lane)| matches!(lane.location, graph::GraphEffectSite::Node(_)))
                .collect::<BTreeMap<_, _>>()
        };
        let predecessor = lanes(&mut before);
        let moves: Vec<_> = lanes(&mut after)
            .into_iter()
            .map(|(key, lane)| graph::GraphEffectMove {
                successor: lane.location,
                predecessor: predecessor[&key].location,
            })
            .collect();
        assert_eq!(moves.len(), tracks.len(), "every delay instance moves");
        graph::install_carry_program(
            &mut after,
            graph::GraphCarryProgram {
                predecessor: graph::plan_identity(&mut before).expect("a graph plan"),
                sources: Box::default(),
                timeline: false,
            },
        )
        .expect("carry program");
        graph::install_effect_carry(&mut after, moves, 0).expect("effect carry");
        let frames = after.envelope().quantum.0 as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        let mut block = |plan: &mut PreparedRenderPlan, index: u64| {
            plan.render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
                },
                RenderTime {
                    absolute_sample: index * frames as u64,
                },
            )
            .expect("render");
        };
        for index in 0..SWAP {
            block(&mut before, index);
        }
        assert_eq!(
            after.adopt_predecessor_plan(&mut before),
            engine::realtime::CarryOutcome::Carried
        );
        for index in SWAP..BLOCKS {
            block(&mut after, index);
        }
        drop((before, after));
        before_sinks
            .iter()
            .zip(&after_sinks)
            .map(|(before, after)| {
                let mut bits: Vec<u32> = Vec::new();
                for sink in [before, after] {
                    bits.extend(
                        sink.lock()
                            .expect("recorder")
                            .iter()
                            .flat_map(|(left, right)| [*left, *right]),
                    );
                }
                bits
            })
            .collect()
    };
    let (armed, dual) = (run(false), run(true));
    let frames = armed[0].len() / BLOCKS as usize;
    let swap = SWAP as usize * frames;
    assert!(
        dual[0][swap..swap + frames]
            .as_chunks::<2>()
            .0
            .iter()
            .any(|[left, right]| left != right),
        "the carried delay echoes the episode into one plane on the swap block"
    );
    assert!(
        armed == dual,
        "the swapped render, armed against forced off"
    );
}
