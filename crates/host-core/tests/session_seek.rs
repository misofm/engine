//! Issue #1642, gate 1b (B16-R2): a session seek is one committed act, so the timeline and every
//! source take it in the same render block, while the session thread seeks concurrently with the
//! render thread.
//!
//! The session thread pushes 10,000 plain and anchored session seeks, each one generation newer,
//! through [`SourceControlSet::seek_session`], while a render thread renders the plan back to back.
//! The render thread records every block whose timeline value is not the previous block's plus a
//! quantum; the session thread reads each source's seek report (the block its generation started
//! at and the frame it started from). Every seek must be one timeline step, in the block both
//! sources started it at, at the frames the seek names. No PCM is fed: a seek applies with or
//! without it.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use bench_support::alloc as bench_alloc;
use engine::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo};
use graph_compiler::Backend;
use host_core::{SourceControlSet, test_only_timeline_at};
use session::{RouteSource, SessionModel, Source, StableId, parse_session_json};
use successor::{QUANTUM, Session};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const LONG: &str = "fixture-source";
const SHORT: &str = "short-source";
const LONG_FRAMES: u64 = 10_000;
const SHORT_FRAMES: u64 = 4_000;
const Q: u64 = QUANTUM as u64;
const SEEKS: usize = 10_000;
/// The span after a block's start the session thread pushes within.
const PUSH_WINDOW_NS: u64 = 40_000;

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// Track `eq0` on [`LONG`] (10,000 frames) and track `eq1` on [`SHORT`] (4,000 frames), each with
/// its own route, no effect and no filter.
fn two_sources() -> SessionModel {
    let mut model: SessionModel = parse_session_json(FIXTURE).expect("fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.tracks.retain(|track| track.id.as_str() == "eq0");
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
    }
    model.routes.retain(|route| route.id.as_str() == "eq0-main");
    model.sources[0].frames = LONG_FRAMES;
    let template = model.sources[0].clone();
    model.sources.push(Source {
        id: id(SHORT),
        content: template.content.replace("7e94", "0a0a"),
        frames: SHORT_FRAMES,
        ..template
    });
    let mut short = model.tracks[0].clone();
    short.id = id("eq1");
    short.source_id = id(SHORT);
    model.tracks.push(short);
    let mut route = model.routes[0].clone();
    route.id = id("eq1-main");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route");
    };
    *track_id = id("eq1");
    model.routes.push(route);
    model
}

fn backends() -> Vec<Backend> {
    vec![
        #[cfg(target_feature = "avx2")]
        Backend::Simd8,
        Backend::Simd4,
    ]
}

/// One session seek as the session thread pushed it, and where each source's report says it
/// landed: `(first sample, source frame)`.
struct Seek {
    timeline_sample: u64,
    anchor: Option<u64>,
    long: (u64, u64),
    short: (u64, u64),
}

/// What the render thread saw: every timeline step `(block sample, timeline value)`, its
/// allocator counts over the measured blocks, and how many blocks it rendered.
struct Rendered {
    plan: PreparedRenderPlan,
    steps: Vec<(u64, i64)>,
    overflow: bool,
    allocator: (u64, u64, u64),
    blocks: u64,
}

fn render_block(plan: &mut PreparedRenderPlan, output: &mut [f32]) -> u64 {
    let sample = plan.next_absolute_sample();
    plan.render_contiguous(
        RenderIo {
            output: PlanarBufferMut::try_new(output, 2, QUANTUM, QUANTUM).expect("output"),
        },
        sample,
    )
    .unwrap_or_else(|error| panic!("render: {error:?}"));
    sample
}

/// Render back to back until `stop`, publishing the next block's sample in `position`.
fn render_thread(
    mut plan: PreparedRenderPlan,
    stop: Arc<AtomicBool>,
    position: Arc<AtomicU64>,
) -> Rendered {
    let mut output = vec![0.0_f32; QUANTUM * 2];
    engine::realtime::audit::warm_up();
    let mut expected = 0_i64;
    for _ in 0..4 {
        let sample = render_block(&mut plan, &mut output);
        expected = test_only_timeline_at(&mut plan, sample).expect("timeline") + Q as i64;
    }
    position.store(plan.next_absolute_sample(), Ordering::Release);
    let mut steps = Vec::with_capacity(SEEKS + 64);
    let mut overflow = false;
    let mut blocks = 0_u64;
    let mark = bench_alloc::current_thread_counters();
    while !stop.load(Ordering::Acquire) {
        let sample = render_block(&mut plan, &mut output);
        let value = test_only_timeline_at(&mut plan, sample).expect("timeline");
        if value != expected {
            if steps.len() < steps.capacity() {
                steps.push((sample, value));
            } else {
                overflow = true;
            }
        }
        expected = value + Q as i64;
        blocks += 1;
        position.store(sample + Q, Ordering::Release);
    }
    let delta = bench_alloc::current_thread_delta_since(mark);
    Rendered {
        plan,
        steps,
        overflow,
        allocator: (delta.allocations, delta.reallocations, delta.deallocations),
        blocks,
    }
}

/// The source's report once it names `generation`: `(first sample, source frame)`.
fn landed(sources: &SourceControlSet, id: &str, generation: u64) -> Option<(u64, u64)> {
    let report = sources.seek_report(id.as_bytes()).ok()?;
    assert!(report.generation <= generation, "{id}: a newer generation");
    (report.generation == generation).then_some((report.first_sample, report.source_frame))
}

/// Push [`SEEKS`] session seeks, each after the one before it has landed on both sources.
fn session_thread(sources: &mut SourceControlSet, position: &AtomicU64) -> Vec<Seek> {
    let mut seeks = Vec::with_capacity(SEEKS);
    let mut state = 0x2545_f491_4f6c_dd1d_u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for index in 0..SEEKS {
        let generation = index as u64 + 2;
        // Residue `generation mod Q`, which the previous seek's never shares, so every seek is a
        // visible timeline step; below 10,000, so the long source never clamps.
        let timeline_sample = (next() % 78) * Q + generation % Q;
        let anchor = (next() % 2 == 0).then(|| {
            let blocks_ahead = next() % 4;
            (position.load(Ordering::Acquire) / Q + blocks_ahead) * Q
        });
        // Push at a random instant shortly after a block begins, where the render's source set
        // pops the timeline and then each source: a push there lands between two of the pops.
        let seen = position.load(Ordering::Acquire);
        while position.load(Ordering::Acquire) == seen {
            core::hint::spin_loop();
        }
        let wait = Duration::from_nanos(next() % PUSH_WINDOW_NS);
        let start = Instant::now();
        while start.elapsed() < wait {
            core::hint::spin_loop();
        }
        loop {
            match sources.seek_session(generation, timeline_sample, anchor) {
                Ok(()) => break,
                Err(error) if error.is_backpressure() => thread::yield_now(),
                Err(error) => panic!("seek {generation}: {error:?}"),
            }
        }
        let (long, short) = loop {
            if let (Some(long), Some(short)) = (
                landed(sources, LONG, generation),
                landed(sources, SHORT, generation),
            ) {
                break (long, short);
            }
            thread::yield_now();
        };
        seeks.push(Seek {
            timeline_sample,
            anchor,
            long,
            short,
        });
    }
    seeks
}

fn run(backend: Backend) {
    let session = Session::compile(two_sources());
    let host = session.prepare(backend);
    let mut sources = host.sources;
    let stop = Arc::new(AtomicBool::new(false));
    let position = Arc::new(AtomicU64::new(0));
    let renderer = {
        let (stop, position) = (Arc::clone(&stop), Arc::clone(&position));
        let plan = host.plan;
        thread::spawn(move || render_thread(plan, stop, position))
    };
    while position.load(Ordering::Acquire) == 0 {
        thread::yield_now();
    }
    let seeks = session_thread(&mut sources, &position);
    stop.store(true, Ordering::Release);
    let rendered = renderer.join().expect("the render thread");

    assert_eq!(
        rendered.allocator,
        (0, 0, 0),
        "{backend:?}: the render thread allocates and frees nothing over {} blocks",
        rendered.blocks
    );
    assert!(
        !rendered.overflow,
        "{backend:?}: more timeline steps than seeks"
    );
    assert_eq!(
        rendered.steps.len(),
        seeks.len(),
        "{backend:?}: one timeline step per seek"
    );
    let mut anchored = 0;
    let mut late = 0;
    for (index, (seek, &(sample, value))) in seeks.iter().zip(&rendered.steps).enumerate() {
        let generation = index as u64 + 2;
        assert_eq!(
            (seek.long.0, seek.short.0),
            (sample, sample),
            "{backend:?}: seek {generation} split across blocks: timeline at {sample}, long at {}, \
             short at {}",
            seek.long.0,
            seek.short.0
        );
        let lateness = match seek.anchor {
            None => 0,
            Some(anchor) => {
                anchored += 1;
                assert!(
                    sample >= anchor,
                    "{backend:?}: seek {generation} before its anchor"
                );
                if sample > anchor {
                    late += 1;
                }
                sample - anchor
            }
        };
        assert_eq!(
            value,
            (seek.timeline_sample + lateness) as i64,
            "{backend:?}: seek {generation}'s timeline value"
        );
        assert_eq!(
            (seek.long.1, seek.short.1),
            (
                seek.timeline_sample.min(LONG_FRAMES) + lateness,
                seek.timeline_sample.min(SHORT_FRAMES) + lateness
            ),
            "{backend:?}: seek {generation}'s source frames"
        );
    }
    // Not vacuous: both forms ran, on time and late, and the short source clamped.
    assert!(
        anchored > SEEKS / 4 && anchored < SEEKS * 3 / 4,
        "{anchored} anchored"
    );
    assert!(late > 0, "an anchored seek observed late");
    assert!(
        seeks
            .iter()
            .any(|seek| seek.anchor.is_none() && seek.timeline_sample > SHORT_FRAMES),
        "a plain seek past the short source's end"
    );
    drop(rendered.plan);
}

/// Gate 1b.
///
/// Test value: red if a consumer observes a session seek's command before the session publishes
/// it (the timeline and the sources pop at different instants while the session thread pushes, so
/// a seek pushed during a block splits across two blocks), if the render thread allocates or
/// frees, or if a source is not clamped or the anchor not kept.
#[test]
fn concurrent_session_seeks_reach_the_timeline_and_every_source_in_one_block() {
    use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
    struct RestoreMode(Mode);
    impl Drop for RestoreMode {
        fn drop(&mut self) {
            set_mode(self.0);
        }
    }
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);
    for backend in backends() {
        run(backend);
    }
}
