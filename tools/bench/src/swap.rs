//! `bench swap`: the swap block's cost on the 64-track app-shape console (issue #1286).
//!
//! One run, two phases, each timing one render call per observation (D2), on session A, the
//! 64-track app-shape fixture at 96 kHz and quantum 128 (D1):
//!
//! 1. **Move.** A → B → A …, one carrying swap every [`INTERVAL_BLOCKS`] blocks. B is A plus one
//!    muted track whose ID sorts first ([`MUTED_TRACK`]), on A's source, so every bank shifts a
//!    lane: the worst carry shape for lane copies. Each swap block is a move observation; every
//!    other block of the phase is an ordinary observation.
//! 2. **Prime.** [`PRIME_CYCLES`] cycles. A cycle starts from a freshly prepared A on a fresh plan
//!    exchange and grows A → `C_1` → … → `C_4`, one growth every [`INTERVAL_BLOCKS`] blocks. Growth
//!    `j` adds one true-peak limiter and [`SOFT_CLIPPERS_PER_GROWTH`] soft clippers on the first
//!    track (D1's growth chain at 96 kHz), so every growth is a warm growth of
//!    `P = ceil_q(L_max)` and `C_4`'s `ΣP` is `p_max_samples(96 kHz, 128)`. Each `C_j` is
//!    prepared warm through host-core (`warm_lead`, then `SuccessorBase { warm }`) and published
//!    `Primed` with `not_before` at the next block (the render clock), after the bench sets the
//!    running rings' `prime_required` flags itself (scope note R2: the bench owns its rings); the
//!    timed observation is the render call that adopts it.
//!
//! Every plan is prepared as the C ABI prepares it (`capi`'s preparer: the live lanes the session's
//! rows choose, effect lanes for the observation taps, the meter queue depth), with every source
//! ring set explicitly to `stall_ring_frames + p_max_samples + q = 14,080` frames
//! ([`SOURCE_RING_FRAMES`]). The bench feeds the one source's ring with the tone outside the timed
//! call, [`FEED_AHEAD_FRAMES`] past the read position, so every prime is ready at its
//! `not_before` block. Successors are prepared, and displaced plans reclaimed, off the timed path.
//!
//! The record reports every distribution, each growth's `prime_bytes()` and adoption lateness,
//! the carry refusal count (root ruling 2026-10-12: read and reported, never asserted zero), the
//! smallest output peak of the timed blocks, and D3's derivation over this round's numbers.
//! `scripts/operator/swap-benchmark-validator.jq` is the frozen validator; the runner is
//! `scripts/operator/run-console-benchmark.sh --swap --step NAME`.
//!
//! `bench swap --preflight` runs [`self_test`], gate 1: the same code over one move swap and one
//! growth cycle, untimed, asserting each premise the timed run depends on.

use core::num::NonZeroUsize;

use bench_support::metadata::Metadata;
use bench_support::stats::{Percentiles, format_f64, nearest_rank};
use bench_support::{alloc as bench_alloc, timing};
use engine::realtime::{
    CarryOutcome, PlanAdoption, PlanExchangeConfig, PlanPublisher, PlanRetirer, PlanarBufferMut,
    RealtimePlanOwner, RenderIo, SwapOutcome, plan_exchange,
};
use graph_compiler::Backend;
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, HostShapePolicy, LiveValuesSuperseded, PlanStateInventory, PreparedHost,
    SourceControlSet, SourceSubmission, SuccessorBase, WarmConfig, WarmDecision,
    compile_host_model, default_source_ring_frames, p_max_samples, stall_ring_frames, warm_lead,
};
use session::{
    Effect, EffectIdentity, EffectQuality, LinkMode, SessionModel, SidechainDeclaration, StableId,
    parse_session_json,
};

/// Session A's fixture, as the record names it.
const FIXTURE_PATH: &str = "fixtures/session/v1/console-sixty-four-track-app.json";
const FIXTURE: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-app.json");
/// The fixture's one source, which every track reads.
const SOURCE: &str = "fixture-source";
/// D1: the highest launch rate.
const RATE: u32 = 96_000;
/// D1: Web Audio's quantum, the fixture's.
const QUANTUM: u32 = 128;
const QUANTUM_FRAMES: usize = QUANTUM as usize;
/// `p_max_samples(96 kHz, 128)`: `ΣP` after the fourth growth.
const P_MAX: u32 = p_max_samples(RATE, QUANTUM);
/// D1: every source ring, explicitly, so each growth has #1354 D2 step 7's headroom whatever the
/// default ring is.
const SOURCE_RING_FRAMES: u32 = stall_ring_frames(RATE, QUANTUM) + P_MAX + QUANTUM;
/// How far past the running plan's read position the bench keeps the ring fed: the largest
/// prime's `P + q` window, so readiness never waits on the feed.
const FEED_AHEAD_FRAMES: u64 = P_MAX as u64 + QUANTUM as u64;
/// The source length the bench declares, in frames: two seconds, longer than either phase reads
/// (the fixture's one second at 48 kHz is half a second here). The source is fed, never decoded.
const SOURCE_FRAMES: u64 = 2 * RATE as u64;
/// B's added track: muted, on A's source, and sorting before every track of A (`ch00`..`ch63`).
const MUTED_TRACK: &str = "aa-muted";
/// The track the growth chain grows: A's first.
const GROWN_TRACK: &str = "ch00";
/// D1: `c(96 kHz)`, the soft clippers (31 samples each) that pad one limiter so that
/// `ceil_q(j · u) = j · K` for `j = 1..4`.
const SOFT_CLIPPERS_PER_GROWTH: usize = 1;
/// D1: the growths of one cycle.
const GROWTHS: usize = 4;
/// D2: blocks between two swaps or two growths.
const INTERVAL_BLOCKS: usize = 32;
/// D2: the move phase's carrying swaps (frozen with the validator).
const MOVE_SWAPS: usize = 32;
/// D2: the prime phase's cycles.
const PRIME_CYCLES: usize = 32;
/// D3: the adoption block may use at most this share of its quantum (#26's callback ceiling).
const MARGIN: f64 = 0.7;
/// D3: #1331 D7's browser headroom `h` of the slowest supported engine (WebKit; owner ruling
/// 2026-10-25: Chrome and Safari only), measured on the 64-track app session at [`BROWSER_RATE`].
const BROWSER_HEADROOM: f64 = 13.7;
const BROWSER_HEADROOM_ENGINE: &str = "webkit";
const BROWSER_RATE: u32 = 48_000;
/// The successor revision the first published plan commits as: the first plan is at revision 1.
const FIRST_REVISION: u64 = 1;

/// The counts one run takes.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Shape {
    interval_blocks: usize,
    move_swaps: usize,
    prime_cycles: usize,
}

/// The runner's run (D2).
const FROZEN: Shape = Shape {
    interval_blocks: INTERVAL_BLOCKS,
    move_swaps: MOVE_SWAPS,
    prime_cycles: PRIME_CYCLES,
};

/// Gate 1's self-test: one move swap and one growth cycle.
const SELF_TEST: Shape = Shape {
    interval_blocks: 4,
    move_swaps: 1,
    prime_cycles: 1,
};

pub(crate) fn main() {
    bench_alloc::assert_installed();
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    match arguments.as_slice() {
        [] => {}
        // Gate 1, untimed and alone. It takes no round marker because it produces no record.
        [flag] if flag == "--preflight" => {
            let measured = self_test();
            eprintln!(
                "swap self-test: PASS ({} move swaps, {} primed adoptions, ΣP {}, smallest peak {})",
                measured.moves.len(),
                measured
                    .growths
                    .iter()
                    .map(|g| g.adoptions.len())
                    .sum::<usize>(),
                measured.sum_lead_samples,
                measured.min_peak
            );
            return;
        }
        _ => panic!("the swap benchmark accepts no arguments but --preflight"),
    }
    let round = round_from_runner();
    let backend = Backend::current();
    let metadata = Metadata::gather();
    // The runner pins the process; the render thread goes to its core before any plan is built.
    let _placement = crate::console::affinity::Placement::pin_render_thread(metadata);
    let measured = Measurement::run(FROZEN, true);
    println!("{}", measured.record(round, backend, metadata));
}

fn round_from_runner() -> u32 {
    match Metadata::gather().var("MISO_ENGINE_BENCH_ROUND").as_deref() {
        Ok("warmup") => 0,
        Ok("1") => 1,
        Ok("2") => 2,
        _ => panic!("the swap benchmark must be launched by its fixed runner"),
    }
}

/// Gate 1: the run's own code over [`SELF_TEST`], untimed, with every premise asserted: each move
/// swap is applied at the block after its publication and carries; each adoption happens at its
/// `not_before` block and carries, with `P == ceil_q(L_max)`; `C_4`'s `ΣP` is
/// `p_max_samples(96 kHz, 128)`; and every rendered block has a nonzero output peak.
///
/// # Panics
///
/// Panics on the first premise that fails.
pub(crate) fn self_test() -> Measurement {
    let measured = Measurement::run(SELF_TEST, false);
    for (index, swap) in measured.moves.iter().enumerate() {
        assert_eq!(
            swap.carry,
            CarryOutcome::Carried,
            "move swap {index} carries"
        );
    }
    let lead = u64::from(P_MAX) / GROWTHS as u64;
    for (step, growth) in measured.growths.iter().enumerate() {
        assert_eq!(
            growth.lead_samples,
            lead,
            "growth {}: P is ceil_q(L_max)",
            step + 1
        );
        for adoption in &growth.adoptions {
            assert_eq!(
                adoption.late_blocks,
                0,
                "growth {}: adopted at its not_before block",
                step + 1
            );
            assert_eq!(
                adoption.carry,
                CarryOutcome::Carried,
                "growth {}: the adoption carries",
                step + 1
            );
        }
    }
    assert_eq!(
        measured.sum_lead_samples,
        u64::from(P_MAX),
        "C_4's ΣP is p_max_samples(96 kHz, 128)"
    );
    assert_eq!(
        measured.silent_blocks, 0,
        "every block has a nonzero output peak"
    );
    measured
}

// ---------------------------------------------------------------------------------------------
// Sessions
// ---------------------------------------------------------------------------------------------

/// One session: its compilation, and the normalized model the C ABI commits.
struct Session {
    model: SessionModel,
    compiled: CompiledSession,
}

impl Session {
    fn compile(model: &SessionModel) -> Self {
        let compile_caps = caps()
            .compile_caps(model.sources.len())
            .unwrap_or_else(|failure| panic!("compile caps: {failure:?}"));
        let compiled = compile_host_model(model, compile_caps)
            .unwrap_or_else(|failure| panic!("compile: {failure:?}"));
        Self {
            model: compiled.normalized_model().clone(),
            compiled,
        }
    }
}

/// A, B and `C_1..C_4` (D1).
struct Sessions {
    a: Session,
    b: Session,
    growth: Vec<Session>,
}

fn sid(text: &str) -> StableId {
    StableId::parse(text).unwrap_or_else(|| panic!("stable ID {text}"))
}

fn native(id: &str, effect_id: &str, link_mode: LinkMode) -> Effect {
    Effect {
        id: sid(id),
        identity: EffectIdentity::Native {
            effect_id: sid(effect_id),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

impl Sessions {
    fn build() -> Self {
        let mut a = parse_session_json(FIXTURE).expect("the app-shape fixture parses");
        a.sample_rate_hz = RATE;
        assert_eq!(a.quantum_frames, QUANTUM, "the fixture's quantum");
        assert_eq!(a.sources.len(), 1, "the fixture's one source");
        assert_eq!(a.tracks.len(), 64, "the 64-track console");
        a.sources[0].frames = SOURCE_FRAMES;

        let mut b = a.clone();
        let mut muted = b.tracks[0].clone();
        muted.id = sid(MUTED_TRACK);
        muted.fader.left_mute = true;
        muted.fader.right_mute = true;
        b.tracks.push(muted);
        let mut route = b.routes[0].clone();
        route.id = sid(&format!("{MUTED_TRACK}-main"));
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the fixture's first route leaves a track");
        };
        *track_id = sid(MUTED_TRACK);
        b.routes.push(route);

        let growth = (1..=GROWTHS)
            .map(|growths| {
                let mut model = a.clone();
                let track = model
                    .tracks
                    .iter_mut()
                    .find(|track| track.id.as_str() == GROWN_TRACK)
                    .expect("the grown track");
                for growth in 0..growths {
                    track.inserts.effects.push(native(
                        &format!("limiter-{growth}"),
                        "miso.true-peak-limiter",
                        LinkMode::Maximum,
                    ));
                    for clipper in 0..SOFT_CLIPPERS_PER_GROWTH {
                        track.inserts.effects.push(native(
                            &format!("clip-{growth}-{clipper}"),
                            "miso.soft-clip",
                            LinkMode::DualMono,
                        ));
                    }
                }
                Session::compile(&model)
            })
            .collect();
        Self {
            a: Session::compile(&a),
            b: Session::compile(&b),
            growth,
        }
    }
}

/// The host caps of every preparation: generous bounds, and D1's explicit ring.
fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: SOURCE_RING_FRAMES,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 1_024,
        maximum_submixes: 1_024,
        maximum_vcas: 256,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 1_024,
        maximum_routes: 4_096,
        maximum_effects: 8_192,
        maximum_graph_session_plus_plan_bytes: 1 << 34,
        maximum_source_total_bytes: 1 << 32,
        maximum_source_overhead_bytes: 1 << 32,
        maximum_effect_state_bytes: 1 << 34,
        maximum_effect_scratch_bytes: 1 << 34,
        maximum_builtin_retained_bytes: 1 << 34,
        maximum_named_allocation_bytes: 1 << 34,
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 28,
    }
}

/// Prepare `session` as the C ABI's preparer does (`crates/capi/src/runtime/mod.rs`): the live
/// lanes the session's rows choose (decision 17, C6), effect lanes for an observation slot per
/// declared tap, and the strip meters' queue depth; as the successor `base` describes, if any.
fn prepare(
    session: &Session,
    base: Option<SuccessorBase<'_>>,
    superseded: &LiveValuesSuperseded,
) -> (PreparedHost, HostLiveControlHandles) {
    let live = HostLiveControlRequest {
        meter_queue_depth: NonZeroUsize::new(host_core::METER_QUEUE_DEPTH)
            .unwrap_or(NonZeroUsize::MIN),
        observation_taps: host_core::launch_maximum_observation_taps(),
        ..HostLiveControlRequest::default()
    };
    let lanes: HostLiveLanes = controls::live_lanes(session.compiled.normalized_model()).into();
    let lanes = HostLiveLanes {
        effects: lanes.effects || live.observation_taps != 0,
        ..lanes
    };
    let (host, handles, _) = host_core::prepare_host_runtime_with_live_lanes_and_spectrum(
        &session.compiled,
        &caps(),
        &live,
        lanes,
        None,
        base,
        superseded,
    )
    .unwrap_or_else(|failure| panic!("prepare: {failure:?}"));
    (host, handles)
}

// ---------------------------------------------------------------------------------------------
// The stream: one plan exchange, fed and rendered as a host drives it
// ---------------------------------------------------------------------------------------------

/// One rendered block.
struct Block {
    nanoseconds: u64,
    /// The absolute sample the block started at.
    start: u64,
    swap: SwapOutcome,
    carry: CarryOutcome,
    peak: f32,
}

struct Stream {
    publisher: PlanPublisher,
    owner: RealtimePlanOwner,
    retirer: PlanRetirer,
    sources: SourceControlSet,
    /// The newest published plan's handles, held as the C ABI holds them.
    _handles: HostLiveControlHandles,
    /// The newest published plan's inventory and committed model: the next edit's base.
    inventory: PlanStateInventory,
    committed: SessionModel,
    /// The running plan's source-read offset, and the pending candidate's.
    offset: u64,
    pending: Option<u64>,
    revision: u64,
    /// The next source frame the feed submits.
    fed: u64,
    tone: [Vec<f32>; 2],
    output: Vec<f32>,
    superseded: LiveValuesSuperseded,
    /// The carry refusals (#1279 D6) of every plan reclaimed so far.
    carry_refusals: u64,
}

impl Stream {
    fn start(session: &Session) -> Self {
        let superseded = LiveValuesSuperseded::new();
        let (host, handles) = prepare(session, None, &superseded);
        let PreparedHost {
            plan,
            sources,
            inventory,
            ..
        } = host;
        let offset = inventory.source_read_offset();
        let (publisher, owner, retirer) = plan_exchange(
            plan,
            PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::MIN,
            },
        )
        .unwrap_or_else(|_| panic!("plan exchange"));
        let block = console_workload::source_block(0, false);
        let tone = [
            block[..QUANTUM_FRAMES].to_vec(),
            block[QUANTUM_FRAMES..].to_vec(),
        ];
        Self {
            publisher,
            owner,
            retirer,
            sources,
            _handles: handles,
            inventory,
            committed: session.model.clone(),
            offset,
            pending: None,
            revision: FIRST_REVISION,
            fed: 0,
            tone,
            output: vec![0.0; QUANTUM_FRAMES * 2],
            superseded,
            carry_refusals: 0,
        }
    }

    fn base(&self) -> SuccessorBase<'_> {
        SuccessorBase::new(&self.inventory, &self.committed, self.revision + 1)
    }

    /// Feed the tone until the source holds [`FEED_AHEAD_FRAMES`] past the frame the next block
    /// reads.
    fn feed(&mut self) {
        let until = self.owner.next_absolute_sample() + self.offset + FEED_AHEAD_FRAMES;
        while self.fed < until {
            self.sources
                .submit(
                    SOURCE.as_bytes(),
                    SourceSubmission {
                        generation: 1,
                        start_frame: self.fed,
                        sample_rate_hz: RATE,
                        planes: &[&self.tone[0], &self.tone[1]],
                        frames: QUANTUM,
                        end_of_region: false,
                    },
                )
                .unwrap_or_else(|error| panic!("submit at {}: {error:?}", self.fed));
            self.fed += u64::from(QUANTUM);
        }
    }

    /// Feed, render one block (timed when `timed`), and reclaim what it displaced.
    fn block(&mut self, timed: bool) -> Block {
        self.feed();
        let start = self.owner.next_absolute_sample();
        let owner = &mut self.owner;
        let output = &mut self.output;
        let render = || {
            owner.render_contiguous(
                RenderIo {
                    output: PlanarBufferMut::try_new(output, 2, QUANTUM_FRAMES, QUANTUM_FRAMES)
                        .expect("output"),
                },
                start,
            )
        };
        let (nanoseconds, report) = if timed {
            timing::timed(render)
        } else {
            (0, timing::untimed(render))
        };
        let report = report.unwrap_or_else(|error| panic!("render at {start}: {error:?}"));
        if report.swap == SwapOutcome::Applied {
            self.offset = self.pending.take().expect("a pending candidate");
        }
        // The displaced plan is reclaimed off the render call, and its carry refusals are read
        // there, by the holder of the plan (#1279 D6).
        while let Ok((_, mut plan)) = self.retirer.try_reclaim() {
            self.carry_refusals += graph::carry_refusal_count(&mut plan).unwrap_or(0);
        }
        let peak = self
            .output
            .iter()
            .fold(0.0_f32, |peak, sample| peak.max(sample.abs()));
        Block {
            nanoseconds,
            start,
            swap: report.swap,
            carry: report.carry,
            peak,
        }
    }

    /// Publish `host`, prepared from [`Self::base`] for `session`: adopted at the next block, or,
    /// with `lead_blocks`, `Primed` with `not_before` at the render clock (the next block's
    /// start). Returns `not_before` for a primed candidate.
    ///
    /// A primed candidate's flags are written first (#1320 D4, the order #1403 D3 states): every
    /// ring whose producer the running set holds reads `true` exactly when the candidate carries
    /// it (holds it vacant), and every ring the candidate created reads `false`. Only then does the
    /// candidate's set take the carried producers, and the commit's `Release` orders the stores
    /// before render's load of the candidate.
    fn publish(
        &mut self,
        host: PreparedHost,
        handles: HostLiveControlHandles,
        session: &Session,
        lead_blocks: Option<u32>,
    ) -> Option<u64> {
        assert!(self.pending.is_none(), "one candidate at a time");
        self.revision += 1;
        let PreparedHost {
            mut plan,
            mut sources,
            inventory,
            ..
        } = host;
        // The successor's carry retargets are stamped with this revision; it is published on the
        // successor's own gate before the successor is (#1277 D5).
        plan.take_revision_gate()
            .expect("the successor's gate")
            .publish(self.revision);
        let not_before = lead_blocks.map(|_| self.publisher.render_clock());
        let adoption = match (not_before, lead_blocks) {
            (Some(not_before), Some(lead_blocks)) => {
                self.sources.set_prime_flags(|id| sources.holds_vacant(id));
                sources.set_prime_flags(|_| false);
                PlanAdoption::Primed {
                    not_before,
                    lead_blocks,
                }
            }
            _ => PlanAdoption::Next,
        };
        let reservation = self
            .publisher
            .reserve_replacement(plan, adoption)
            .unwrap_or_else(|_| panic!("reserve the successor"));
        // After the flags: W's set takes the producers of the rings it carries.
        sources.adopt_persisting(&mut self.sources);
        self.pending = Some(inventory.source_read_offset());
        reservation.commit(self.revision);
        self.sources = sources;
        self._handles = handles;
        self.inventory = inventory;
        self.committed = session.model.clone();
        not_before
    }

    /// Displace the running plan with an ordinary successor of the same `session`, untimed, so
    /// every plan the stream measured is reclaimed and its carry refusals are read; returns the
    /// stream's refusal count.
    fn close(mut self, session: &Session) -> u64 {
        let (host, handles) = prepare(session, Some(self.base()), &self.superseded);
        self.publish(host, handles, session, None);
        let block = self.block(false);
        assert_eq!(
            block.swap,
            SwapOutcome::Applied,
            "the closing swap is applied"
        );
        assert!(self.pending.is_none(), "every candidate adopted");
        self.carry_refusals
    }
}

// ---------------------------------------------------------------------------------------------
// The measurement
// ---------------------------------------------------------------------------------------------

/// One move swap's block.
#[derive(Clone, Copy)]
struct MoveSwap {
    nanoseconds: u64,
    carry: CarryOutcome,
    restarted_strips: usize,
}

/// One growth's adoption block.
#[derive(Clone, Copy)]
struct Adoption {
    nanoseconds: u64,
    carry: CarryOutcome,
    /// Blocks between `not_before` and the adopting block.
    late_blocks: u64,
}

/// One growth step over every cycle.
struct Growth {
    lead_samples: u64,
    sum_lead_samples: u64,
    prime_bytes: u64,
    adoptions: Vec<Adoption>,
}

pub(crate) struct Measurement {
    shape: Shape,
    ordinary: Vec<u64>,
    moves: Vec<MoveSwap>,
    growths: Vec<Growth>,
    /// `ΣP` after the fourth growth of every cycle (equal in every cycle, asserted).
    sum_lead_samples: u64,
    move_carry_refusals: u64,
    prime_carry_refusals: u64,
    /// Blocks rendered with a zero output peak, and the smallest peak, over the timed
    /// observations (every rendered block in the self-test, which times none).
    silent_blocks: usize,
    min_peak: f32,
    observed_blocks: usize,
}

impl Measurement {
    /// Run `shape`; with `timed`, every observation is one [`timing::timed`] render call.
    fn run(shape: Shape, timed: bool) -> Self {
        let sessions = Sessions::build();
        let mut measured = Self {
            shape,
            ordinary: Vec::new(),
            moves: Vec::new(),
            growths: (0..GROWTHS)
                .map(|_| Growth {
                    lead_samples: 0,
                    sum_lead_samples: 0,
                    prime_bytes: 0,
                    adoptions: Vec::new(),
                })
                .collect(),
            sum_lead_samples: 0,
            move_carry_refusals: 0,
            prime_carry_refusals: 0,
            silent_blocks: 0,
            min_peak: f32::INFINITY,
            observed_blocks: 0,
        };
        measured.run_moves(&sessions, timed);
        for _ in 0..shape.prime_cycles {
            measured.run_cycle(&sessions, timed);
        }
        measured
    }

    /// Count `block`'s peak: every timed block in a timed run, every block in an untimed one.
    fn observe(&mut self, block: &Block, timed_run: bool, observation: bool) {
        if observation || !timed_run {
            self.observed_blocks += 1;
            self.min_peak = self.min_peak.min(block.peak);
            if block.peak == 0.0 {
                self.silent_blocks += 1;
            }
        }
    }

    /// Phase 1: A ⇄ B, one carrying swap every interval, after one untimed lead-in interval.
    fn run_moves(&mut self, sessions: &Sessions, timed: bool) {
        let interval = self.shape.interval_blocks;
        let mut stream = Stream::start(&sessions.a);
        for _ in 0..interval {
            let block = stream.block(false);
            self.observe(&block, timed, false);
        }
        for swap in 0..self.shape.move_swaps {
            let next = if swap % 2 == 0 {
                &sessions.b
            } else {
                &sessions.a
            };
            let (host, handles) = prepare(next, Some(stream.base()), &stream.superseded);
            let restarted_strips = host.restarted_strips().len();
            stream.publish(host, handles, next, None);
            let block = stream.block(timed);
            self.observe(&block, timed, true);
            assert_eq!(
                block.swap,
                SwapOutcome::Applied,
                "move swap {swap} is applied at the block after its publication"
            );
            self.moves.push(MoveSwap {
                nanoseconds: block.nanoseconds,
                carry: block.carry,
                restarted_strips,
            });
            for _ in 1..interval {
                let block = stream.block(timed);
                self.observe(&block, timed, true);
                self.ordinary.push(block.nanoseconds);
            }
        }
        let last = if self.shape.move_swaps % 2 == 1 {
            &sessions.b
        } else {
            &sessions.a
        };
        self.move_carry_refusals += stream.close(last);
    }

    /// Phase 2, one cycle: a fresh A on a fresh exchange, grown to `C_4` warm.
    fn run_cycle(&mut self, sessions: &Sessions, timed: bool) {
        let interval = self.shape.interval_blocks;
        let config = WarmConfig {
            p_max: u64::from(P_MAX),
            // This run is what sets the bound.
            prime_bytes_max: u64::MAX,
        };
        let mut stream = Stream::start(&sessions.a);
        let mut sum_lead = 0;
        for (step, session) in sessions.growth.iter().enumerate() {
            for _ in 0..interval {
                let block = stream.block(false);
                self.observe(&block, timed, false);
            }
            // The production order (#1354a ruling 1): the ordinary successor, whose inventory
            // `warm_lead` reads, then the warm successor.
            let (ordinary, _) = prepare(session, Some(stream.base()), &stream.superseded);
            let decision = warm_lead(
                &session.compiled,
                &caps(),
                stream.base(),
                &ordinary.inventory,
                &config,
            )
            .unwrap_or_else(|failure| panic!("warm_lead: {failure:?}"));
            drop(ordinary);
            let WarmDecision::Warm(lead) = decision else {
                panic!("growth {}: no warm successor: {decision:?}", step + 1);
            };
            let (host, handles) = prepare(
                session,
                Some(SuccessorBase {
                    warm: Some(&lead),
                    ..stream.base()
                }),
                &stream.superseded,
            );
            assert_eq!(
                host.prime_bytes(),
                lead.prime_bytes(),
                "the warm successor counts the lead's prime"
            );
            let lead_blocks =
                u32::try_from(lead.lead_samples / u64::from(QUANTUM)).expect("the lead in blocks");
            sum_lead += lead.lead_samples;
            let growth = &mut self.growths[step];
            if growth.adoptions.is_empty() {
                growth.lead_samples = lead.lead_samples;
                growth.sum_lead_samples = sum_lead;
                growth.prime_bytes = lead.prime_bytes();
            } else {
                assert_eq!(
                    (
                        growth.lead_samples,
                        growth.sum_lead_samples,
                        growth.prime_bytes
                    ),
                    (lead.lead_samples, sum_lead, lead.prime_bytes()),
                    "growth {}: every cycle grows alike",
                    step + 1
                );
            }
            let not_before = stream
                .publish(host, handles, session, Some(lead_blocks))
                .expect("a primed candidate");
            // Render until the candidate is adopted; a late adoption is recorded, never hidden.
            let mut rendered = 0;
            let adoption = loop {
                let block = stream.block(timed);
                rendered += 1;
                if block.swap == SwapOutcome::Applied {
                    self.observe(&block, timed, true);
                    break Adoption {
                        nanoseconds: block.nanoseconds,
                        carry: block.carry,
                        late_blocks: (block.start - not_before) / u64::from(QUANTUM),
                    };
                }
                self.observe(&block, timed, false);
                assert!(
                    rendered < interval,
                    "growth {}: not adopted within {interval} blocks",
                    step + 1
                );
            };
            self.growths[step].adoptions.push(adoption);
            for _ in rendered..interval {
                let block = stream.block(false);
                self.observe(&block, timed, false);
            }
        }
        assert_eq!(sum_lead, stream.offset, "the running plan reads ΣP ahead");
        self.sum_lead_samples = sum_lead;
        let last = sessions.growth.last().expect("four growths");
        self.prime_carry_refusals += stream.close(last);
    }

    /// The round's record: one JSON line.
    fn record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let move_ns: Vec<u64> = self.moves.iter().map(|swap| swap.nanoseconds).collect();
        let carried = |carry: CarryOutcome| carry == CarryOutcome::Carried;
        let growths: Vec<String> = self
            .growths
            .iter()
            .enumerate()
            .map(|(step, growth)| {
                let ns: Vec<u64> = growth.adoptions.iter().map(|a| a.nanoseconds).collect();
                format!(
                    concat!(
                        "{{\"step\":{step},\"lead_samples\":{lead},\"sum_lead_samples\":{sum},",
                        "\"prime_bytes\":{bytes},\"adoptions\":{adoptions},\"carried\":{carried},",
                        "\"late_adoptions\":{late},\"adoption_ns\":{ns}}}"
                    ),
                    step = step + 1,
                    lead = growth.lead_samples,
                    sum = growth.sum_lead_samples,
                    bytes = growth.prime_bytes,
                    adoptions = growth.adoptions.len(),
                    carried = growth.adoptions.iter().filter(|a| carried(a.carry)).count(),
                    late = growth
                        .adoptions
                        .iter()
                        .filter(|a| a.late_blocks != 0)
                        .count(),
                    ns = distribution(&ns),
                )
            })
            .collect();
        let last = self.growths.last().expect("four growths");
        let ordinary_p99 = rank(&self.ordinary, 99);
        let adoption_ns: Vec<u64> = last.adoptions.iter().map(|a| a.nanoseconds).collect();
        let d3 = Derivation::new(ordinary_p99, rank(&adoption_ns, 99), last.prime_bytes);
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":1286,\"record\":\"swap_carry\",\"round\":{round},",
                "\"backend\":\"{backend}\",\"workload_kind\":\"sixty_four_track_app_swap\",",
                "\"fixture\":\"{fixture}\",\"input\":\"tone\",\"sample_rate_hz\":{rate},",
                "\"quantum_frames\":{quantum},\"tracks\":64,\"muted_track\":\"{muted}\",",
                "\"source_ring_frames\":{ring},\"l_max_samples\":{l_max},",
                "\"soft_clippers_per_growth\":{clippers},\"p_max_samples\":{p_max},",
                "\"interval_blocks\":{interval},",
                "\"move\":{{\"swaps\":{swaps},\"carried\":{move_carried},",
                "\"restarted_strips\":{restarted},\"carry_refusal_count\":{move_refusals},",
                "\"ordinary_ns\":{ordinary},\"move_ns\":{move_ns}}},",
                "\"prime\":{{\"cycles\":{cycles},",
                "\"carry_refusal_count\":{prime_refusals},\"growths\":[{growths}]}},",
                "\"timed_blocks\":{timed_blocks},\"silent_timed_blocks\":{silent},",
                "\"min_timed_output_peak\":{peak:.6},",
                "\"d3\":{d3},",
                "{metadata}",
                "\"benchmark\":\"swap\"}}"
            ),
            round = round,
            backend = crate::console::backend_name(backend),
            fixture = FIXTURE_PATH,
            rate = RATE,
            quantum = QUANTUM,
            muted = MUTED_TRACK,
            ring = SOURCE_RING_FRAMES,
            l_max = l_max_samples(RATE),
            clippers = SOFT_CLIPPERS_PER_GROWTH,
            p_max = P_MAX,
            interval = self.shape.interval_blocks,
            swaps = self.moves.len(),
            move_carried = self.moves.iter().filter(|swap| carried(swap.carry)).count(),
            restarted = self
                .moves
                .iter()
                .map(|swap| swap.restarted_strips)
                .sum::<usize>(),
            move_refusals = self.move_carry_refusals,
            ordinary = distribution(&self.ordinary),
            move_ns = distribution(&move_ns),
            cycles = self.shape.prime_cycles,
            prime_refusals = self.prime_carry_refusals,
            growths = growths.join(","),
            timed_blocks = self.observed_blocks,
            silent = self.silent_blocks,
            peak = self.min_peak,
            d3 = d3.json(),
            metadata = metadata.record_fields(),
        )
    }
}

/// `L_max(fs)`: the true-peak limiter's `fs / 100 + 6` (`crates/true-peak-limiter`), the largest
/// latency a launch native effect declares (D3 item 1).
const fn l_max_samples(rate: u32) -> u32 {
    rate / 100 + 6
}

/// The nearest-rank `percent`th percentile of `samples`.
fn rank(samples: &[u64], percent: usize) -> u64 {
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    nearest_rank(&sorted, percent, 100)
}

/// One distribution: count, min, p50, p90, p99, max (D2).
fn distribution(samples: &[u64]) -> String {
    let summary = Percentiles::from_samples(samples);
    format!(
        "{{\"count\":{},\"min\":{},\"p50\":{},\"p90\":{},\"p99\":{},\"max\":{}}}",
        samples.len(),
        summary.min,
        summary.p50,
        rank(samples, 90),
        summary.p99,
        summary.max
    )
}

/// D3 over one round's numbers: item 3's `PRIME_BYTES_MAX`, and items 1-2's values at every
/// launch rate.
struct Derivation {
    /// `o`, `a_P` (ns) and `b_P` (bytes).
    ordinary_ns: u64,
    adoption_ns: u64,
    prime_bytes: u64,
    native: Option<i64>,
    browser: Option<i64>,
    prime_bytes_max: Option<i64>,
    refusal: Option<&'static str>,
}

impl Derivation {
    /// `β = (a_P − o) / b_P`; `b_native = floor((m · D − o) / β)`; `b_browser =
    /// floor((m · h − 1) · o / β)`; `PRIME_BYTES_MAX = min(b_native, b_browser)`, derived only when
    /// it is at least `b_P` and `m · h > 1`. Each row is evaluated as `… · b_P / (a_P − o)` in this
    /// exact order, so the validator recomputes the same doubles.
    fn new(ordinary_ns: u64, adoption_ns: u64, prime_bytes: u64) -> Self {
        let (o, a, b) = (ordinary_ns as f64, adoption_ns as f64, prime_bytes as f64);
        let quantum_ns = f64::from(QUANTUM) * 1e9 / f64::from(RATE);
        let mut derivation = Self {
            ordinary_ns,
            adoption_ns,
            prime_bytes,
            native: None,
            browser: None,
            prime_bytes_max: None,
            refusal: None,
        };
        if adoption_ns <= ordinary_ns || prime_bytes == 0 {
            derivation.refusal = Some("adoption_not_slower_than_ordinary");
            return derivation;
        }
        let native = ((MARGIN * quantum_ns - o) * b / (a - o)).floor() as i64;
        let browser = ((MARGIN * BROWSER_HEADROOM - 1.0) * o * b / (a - o)).floor() as i64;
        derivation.native = Some(native);
        derivation.browser = Some(browser);
        let floor = i64::try_from(prime_bytes).unwrap_or(i64::MAX);
        derivation.refusal = if MARGIN * BROWSER_HEADROOM <= 1.0 {
            Some("browser_headroom_within_margin")
        } else if native < floor {
            Some("native_row_below_b_p")
        } else if browser < floor {
            Some("browser_row_below_b_p")
        } else {
            None
        };
        if derivation.refusal.is_none() {
            derivation.prime_bytes_max = Some(native.min(browser));
        }
        derivation
    }

    fn json(&self) -> String {
        let integer =
            |value: Option<i64>| value.map_or_else(|| "null".to_owned(), |v| v.to_string());
        let beta = if self.adoption_ns > self.ordinary_ns && self.prime_bytes != 0 {
            format_f64(
                (self.adoption_ns - self.ordinary_ns) as f64 * 1_000.0 / self.prime_bytes as f64,
            )
        } else {
            "null".to_owned()
        };
        let rates: Vec<String> = host_core::LAUNCH_SAMPLE_RATES
            .iter()
            .map(|rate| {
                let rate = rate.0;
                let before = stall_ring_frames(rate, QUANTUM);
                let after = default_source_ring_frames(rate, QUANTUM);
                // Session A's one stereo source, `f32` frames.
                let bytes = |frames: u32| u64::from(frames) * 2 * 4;
                format!(
                    concat!(
                        "{{\"sample_rate_hz\":{rate},\"l_max_samples\":{l_max},",
                        "\"p_max_samples\":{p_max},\"stall_ring_frames\":{before},",
                        "\"default_ring_frames\":{after},\"ring_pcm_bytes_before\":{bytes_before},",
                        "\"ring_pcm_bytes_after\":{bytes_after}}}"
                    ),
                    rate = rate,
                    l_max = l_max_samples(rate),
                    p_max = p_max_samples(rate, QUANTUM),
                    before = before,
                    after = after,
                    bytes_before = bytes(before),
                    bytes_after = bytes(after),
                )
            })
            .collect();
        format!(
            concat!(
                "{{\"o_ns\":{o},\"a_p_ns\":{a},\"b_p_bytes\":{b},\"beta_ps_per_byte\":{beta},",
                "\"margin\":{margin},\"browser_headroom\":{h},",
                "\"browser_headroom_engine\":\"{engine}\",\"browser_sample_rate_hz\":{fs_h},",
                "\"b_native_bytes\":{native},\"b_browser_bytes\":{browser},",
                "\"prime_bytes_max\":{max},\"prime_bytes_max_refusal\":{refusal},",
                "\"rates\":[{rates}]}}"
            ),
            o = self.ordinary_ns,
            a = self.adoption_ns,
            b = self.prime_bytes,
            beta = beta,
            margin = MARGIN,
            h = BROWSER_HEADROOM,
            engine = BROWSER_HEADROOM_ENGINE,
            fs_h = BROWSER_RATE,
            native = integer(self.native),
            browser = integer(self.browser),
            max = integer(self.prime_bytes_max),
            refusal = self
                .refusal
                .map_or_else(|| "null".to_owned(), |reason| format!("\"{reason}\"")),
            rates = rates.join(","),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Adoption, FROZEN, GROWTHS, Measurement, MoveSwap, self_test};
    use bench_support::metadata::Metadata;
    use graph_compiler::Backend;
    use std::path::Path;
    use std::sync::OnceLock;

    /// Gate 1's run, shared by both tests: it is the expensive part.
    fn measured() -> &'static Measurement {
        static MEASURED: OnceLock<Measurement> = OnceLock::new();
        MEASURED.get_or_init(self_test)
    }

    /// Gate 1: the self-test renders A and B through one move swap and one cycle A → `C_1` → … →
    /// `C_4` through four primed adoptions, with every premise asserted inside [`self_test`].
    #[test]
    fn the_self_test_carries_a_move_and_primes_four_growths_to_p_max() {
        let measured = measured();
        assert_eq!(measured.moves.len(), 1, "one move swap");
        assert_eq!(
            measured
                .growths
                .iter()
                .map(|growth| growth.adoptions.len())
                .collect::<Vec<_>>(),
            [1; GROWTHS],
            "one adoption per growth"
        );
    }

    /// The measured run's counts at the frozen shape: the self-test's swap, adoptions and prime
    /// bytes repeated to the frozen counts, with distinct positive timings, so `record` derives D3
    /// from numbers the validator can check.
    fn frozen_from(measured: &Measurement) -> Measurement {
        let swaps = FROZEN.move_swaps;
        let cycles = FROZEN.prime_cycles;
        let mut growths: Vec<_> = measured
            .growths
            .iter()
            .map(|growth| super::Growth {
                lead_samples: growth.lead_samples,
                sum_lead_samples: growth.sum_lead_samples,
                prime_bytes: growth.prime_bytes,
                adoptions: Vec::new(),
            })
            .collect();
        for (step, growth) in growths.iter_mut().enumerate() {
            growth.adoptions = (0..cycles as u64)
                .map(|cycle| Adoption {
                    nanoseconds: 30_000 + 1_000 * step as u64 + cycle,
                    ..measured.growths[step].adoptions[0]
                })
                .collect();
        }
        Measurement {
            shape: FROZEN,
            ordinary: (0..(swaps * (FROZEN.interval_blocks - 1)) as u64)
                .map(|block| 10_000 + block % 97)
                .collect(),
            moves: (0..swaps as u64)
                .map(|swap| MoveSwap {
                    nanoseconds: 20_000 + swap,
                    ..measured.moves[0]
                })
                .collect(),
            growths,
            sum_lead_samples: measured.sum_lead_samples,
            move_carry_refusals: measured.move_carry_refusals,
            prime_carry_refusals: measured.prime_carry_refusals,
            silent_blocks: measured.silent_blocks,
            min_peak: measured.min_peak,
            observed_blocks: swaps * FROZEN.interval_blocks + GROWTHS * cycles,
        }
    }

    fn jq(arguments: &[&str], input: &str) -> std::process::Output {
        let mut child = std::process::Command::new("jq")
            .args(arguments)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("jq runs");
        std::io::Write::write_all(&mut child.stdin.take().expect("jq stdin"), input.as_bytes())
            .expect("jq takes the records");
        child.wait_with_output().expect("jq finishes")
    }

    /// The runner's metadata, which a test process does not have.
    const RUNNER: &str = concat!(
        ".cpu_model = \"Test CPU\" | .governor_or_power_mode = \"performance\" | ",
        ".rust_version = \"rustc\" | .llvm_version = \"21\" | ",
        ".target_triple = \"x86_64-unknown-linux-gnu\" | .target_features = \"runtime-avx2\" | ",
        ".profile = \"release\" | .background_load_note = \"controlled\" | ",
        ".measurement_control = \"controlled\" | .cpu_affinity = \"15\" | ",
        ".candidate_commit = \"0000000000000000000000000000000000000000\" | ",
        ".missing_metadata = []"
    );

    /// Whether `scripts/operator/swap-benchmark-validator.jq` accepts `records` after `edit` is
    /// applied to each.
    fn validator_accepts(records: &[String], edit: &str) -> bool {
        let edited: Vec<String> = records
            .iter()
            .map(|record| {
                let output = jq(&["-c", edit], record);
                assert!(
                    output.status.success(),
                    "the edit {edit:?} did not apply: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                String::from_utf8(output.stdout).expect("jq writes UTF-8")
            })
            .collect();
        let validator = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scripts/operator/swap-benchmark-validator.jq");
        let verdict = jq(
            &[
                "-s",
                "-e",
                "-f",
                validator.to_str().expect("a UTF-8 validator path"),
            ],
            &edited.concat(),
        );
        match verdict.status.code() {
            Some(0) => true,
            Some(1) => false,
            status => panic!(
                "the validator returned no verdict ({status:?}): {}",
                String::from_utf8_lossy(&verdict.stderr)
            ),
        }
    }

    /// Gate 2: the validator accepts the subject's own record at the frozen counts and refuses the
    /// self-test's shortened record and every record gate 2 names.
    #[test]
    fn the_validator_refuses_every_record_gate_two_names() {
        let measured = measured();
        let backend = Backend::current();
        let metadata = Metadata::gather();
        let shortened = [1, 2].map(|round| measured.record(round, backend, metadata));
        // `cargo test -p bench the_validator_refuses -- --nocapture` prints the self-test's record.
        println!("{}", shortened[0]);
        assert!(
            !validator_accepts(&shortened, RUNNER),
            "the self-test's shortened, untimed run must not pass for the frozen one"
        );
        let frozen = frozen_from(measured);
        let records = [1, 2].map(|round| frozen.record(round, backend, metadata));
        assert!(
            validator_accepts(&records, RUNNER),
            "the record at the frozen counts: {}",
            records[0]
        );
        for (edit, why) in [
            ("del(.move.move_ns)", "a missing distribution"),
            ("del(.d3.prime_bytes_max)", "a missing D3 value"),
            (".move.swaps = 31", "a swap count other than the frozen one"),
            (
                ".prime.growths[0].adoptions = 31",
                "an adoption count other than the frozen one",
            ),
            (".move.carried = 31", "a swap that did not carry"),
            (
                ".prime.growths[2].prime_bytes = 0",
                "an adoption that primed nothing",
            ),
            (
                ".prime.growths[1].late_adoptions = 1",
                "an adoption after its not_before",
            ),
            (
                ".prime.growths[3].sum_lead_samples = 3968",
                "a chain short of p_max_samples",
            ),
            (".silent_timed_blocks = 1", "a silent timed block"),
            (".input = \"silence\"", "an input other than the tone"),
            (
                ".d3.b_native_bytes += 1",
                "a native row the numbers do not give",
            ),
            (".move.ordinary_ns.min = 0", "an untimed distribution"),
        ] {
            assert!(
                !validator_accepts(&records, &format!("{RUNNER} | {edit}")),
                "{why}"
            );
        }
    }
}
