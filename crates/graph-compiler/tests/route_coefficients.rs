//! Issue #1215: every route's coefficients come from one function.
//!
//! A plan binds `graph::gated_route_coefficients(&route.transform, route.gate)` for each prepared
//! route, and a live producer will push `graph_compiler::route_coefficients` of the values it is
//! handed. The two layers agree only because the second calls the first; this file holds them to
//! it and pins which column each source lane's gate zeroes. Issue #1237 bounds the values the
//! live path accepts to the session's route domain.
//! *Mute a route in the session* (#1216) and *Let a route into a submix follow its source strip's
//! mute in the session* (#1218) extend it with the gates a session can set; #1216 also holds the
//! session's mute to the sealed graph text.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use graph::{GraphCompileCaps, PreparedRoute, RouteGate, gated_route_coefficients};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
    RouteValueError, route_coefficients,
};
use session::{
    CompileCaps, Route, RouteDestination, RouteSource, SendTap, SessionModel, StableId, Submix,
    compile_session, parse_session_json,
};

/// Nine tracks, each routed to the session output by its own route.
const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

/// The production compile at this build's width; `Err` carries each diagnostic's code and path.
fn compile(model: &SessionModel) -> Result<PreparedGraphBuiltinsArtifact, Vec<(String, String)>> {
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
    .expect("the session compiles");
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
    .expect("the builtins prepare");
    let effects = prepare_native_session_effects(
        &session,
        launch_native_effect_registry().expect("launch registry"),
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 24,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("the native effects prepare");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        dispatch: Backend::current(),
        plan_id: 1,
        effects,
        builtins,
        caps: GraphCompileCaps {
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
        },
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .map_err(|failure| {
        failure
            .diagnostics
            .diagnostics()
            .iter()
            .map(|diagnostic| (diagnostic.code.to_owned(), diagnostic.path.clone()))
            .collect()
    })
}

fn bits(coefficients: [f32; 4]) -> [u32; 4] {
    coefficients.map(f32::to_bits)
}

/// xorshift64: a fixed, dependency-free stream, so a failure names a reproducible draw.
struct Draw(u64);

impl Draw {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    /// Uniform in `[lo, hi)`.
    fn uniform(&mut self, lo: f32, hi: f32) -> f32 {
        let unit = (self.next() >> 40) as f32 / (1_u64 << 24) as f32;
        lo + (hi - lo) * unit
    }
    /// A matrix coefficient: mostly a signed level in the route domain `[-1, 1]` (issue #1237;
    /// this drew up to +12 dB before the domain existed), sometimes an exact signed zero, so a
    /// gated `+0.0` is told apart from a product that was `-0.0` already.
    fn coefficient(&mut self) -> f32 {
        match self.next() % 8 {
            0 => 0.0,
            1 => -0.0,
            _ => self.uniform(-1.0, 1.0),
        }
    }
}

fn sid(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

/// The nine-track fixture plus #1218's sends: every track sends from `pre_fader` into bus `bus`,
/// which sends from `pre_fader` into bus `aux`, and both buses feed the output. Each send's
/// `follows_mute` is set later; the fixture's own routes go to the output and never follow.
fn with_sends(base: &SessionModel) -> SessionModel {
    let mut model = base.clone();
    let template = model.routes[0].clone();
    let route = |id: String, source: RouteSource, destination: RouteDestination| Route {
        id: sid(&id),
        source,
        destination,
        ..template.clone()
    };
    let output = template.destination.clone();
    let into = |submix: &str| RouteDestination::SubmixInput {
        submix_id: sid(submix),
    };
    let tap = SendTap::PreFader;
    for name in ["aux", "bus"] {
        let submix = Submix::unity(sid(name), &model.console);
        model.submixes.push(submix);
    }
    let mut sends: Vec<Route> = model
        .tracks
        .iter()
        .map(|track| {
            let source = RouteSource::Track {
                track_id: track.id.clone(),
                tap,
            };
            route(format!("send-{}", track.id.as_str()), source, into("bus"))
        })
        .collect();
    let bus = |tap| RouteSource::Submix {
        submix_id: sid("bus"),
        tap,
    };
    sends.push(route("send-bus".to_owned(), bus(tap), into("aux")));
    sends.push(route(
        "zz-bus".to_owned(),
        bus(SendTap::PostPan),
        output.clone(),
    ));
    let aux = RouteSource::Submix {
        submix_id: sid("aux"),
        tap: SendTap::PostPan,
    };
    sends.push(route("zz-aux".to_owned(), aux, output));
    model.routes.extend(sends);
    model
}

#[test]
fn every_route_coefficient_comes_from_the_one_gated_function() {
    let base = parse_session_json(SESSION).expect("fixture parses");
    // #1218 gate 5: sends into a submix, from tracks and from a submix, may follow.
    let base = with_sends(&base);
    let mut draw = Draw(0x1215_6761_7465_u64);
    let gates: Vec<(bool, [bool; 2])> = (0..8_u8)
        .map(|g| (g & 1 != 0, [g & 2 != 0, g & 4 != 0]))
        .collect();
    let mut shapes = std::collections::BTreeSet::new();
    for round in 0..24 {
        let mut model = base.clone();
        // #1218 gate 5: every strip's lane mutes, drawn, so a follow reads the right strip.
        let mut strip_mutes = std::collections::BTreeMap::new();
        for fader in model
            .tracks
            .iter_mut()
            .map(|track| (&track.id, &mut track.fader))
            .chain(
                model
                    .submixes
                    .iter_mut()
                    .map(|submix| (&submix.id, &mut submix.fader)),
            )
        {
            let (id, fader) = fader;
            fader.left_mute = draw.next().is_multiple_of(2);
            fader.right_mute = draw.next().is_multiple_of(2);
            strip_mutes.insert(id.as_str().to_owned(), [fader.left_mute, fader.right_mute]);
        }
        let mut drawn = Vec::new();
        for route in &mut model.routes {
            route.gain_db = draw.uniform(-120.0, 24.0);
            let matrix = &mut route.channel_matrix;
            [matrix.ll, matrix.lr, matrix.rl, matrix.rr] = [(); 4].map(|()| draw.coefficient());
            // #1216 gate 2: the session's own switch, drawn per route.
            route.mute = draw.next().is_multiple_of(2);
            // #1218 gate 5: only a route into a submix may follow.
            let into_submix = matches!(route.destination, RouteDestination::SubmixInput { .. });
            route.follows_mute = into_submix && draw.next().is_multiple_of(2);
            let source = match &route.source {
                RouteSource::Track { track_id, .. } => track_id.as_str(),
                RouteSource::Submix { submix_id, .. } => submix_id.as_str(),
            };
            let source_lane_muted = if route.follows_mute {
                strip_mutes[source]
            } else {
                [false; 2]
            };
            drawn.push((
                route.id.as_str().to_owned(),
                route.gain_db,
                [matrix.ll, matrix.lr, matrix.rl, matrix.rr],
                route.mute,
                source_lane_muted,
            ));
        }
        shapes.extend(drawn.iter().map(|row| row.4));
        let artifact = compile(&model).expect("finite routes compile");
        let routes: &[PreparedRoute] = artifact.graph().routes();
        assert_eq!(routes.len(), drawn.len());
        for (id, gain_db, matrix, mute, source_lane_muted) in drawn {
            let context = format!(
                "round {round}, route {id}: {gain_db} dB, {matrix:?}, mute {mute}, \
                 source lanes muted {source_lane_muted:?}"
            );
            let prepared = routes
                .iter()
                .find(|route| {
                    matches!(&route.node, graph::GraphNodeId::Route { route_id }
                        if route_id.as_str() == id)
                })
                .unwrap_or_else(|| panic!("{context}: no prepared route"));
            // The plan's gate is the session's switch (#1216 D2) and, for a following send, its
            // source strip's lane mutes (#1218 D2).
            assert_eq!(
                prepared.gate,
                RouteGate {
                    mute,
                    follow_zeroed: source_lane_muted
                },
                "{context}"
            );
            let checked = route_coefficients(gain_db, matrix, mute, source_lane_muted)
                .unwrap_or_else(|error| panic!("{context}: {error:?}"));
            // The constant the runtime binds for this route is the domain-checked one, bit for bit.
            assert_eq!(
                bits(checked),
                bits(gated_route_coefficients(&prepared.transform, prepared.gate)),
                "{context}"
            );
            let open = route_coefficients(gain_db, matrix, false, [false; 2])
                .unwrap_or_else(|error| panic!("{context}: {error:?}"));
            for &(mute, source_lane_muted) in &gates {
                let gate = RouteGate {
                    mute,
                    follow_zeroed: source_lane_muted,
                };
                let gated = route_coefficients(gain_db, matrix, mute, source_lane_muted)
                    .unwrap_or_else(|error| panic!("{context}, {gate:?}: {error:?}"));
                assert_eq!(
                    bits(gated),
                    bits(gated_route_coefficients(&prepared.transform, gate)),
                    "{context}, {gate:?}"
                );
                // `[ll, lr, rl, rr]`: the left source lane feeds `ll` and `rl`, the right `lr`
                // and `rr`. A gated coefficient is `+0.0`, never the product's own signed zero.
                let silent = mute || source_lane_muted == [true; 2];
                assert_eq!(gate.silences(), silent, "{gate:?}");
                let expected = [0, 1, 2, 3].map(|index| {
                    if silent || source_lane_muted[index % 2] {
                        0.0_f32.to_bits()
                    } else {
                        open[index].to_bits()
                    }
                });
                assert_eq!(bits(gated), expected, "{context}, {gate:?}");
            }
        }
    }

    assert_eq!(shapes.len(), 4, "the rounds draw every follow-zeroed shape");
}

/// The compile request's session half: `compile_session`'s refusal codes at `path`, if any.
fn session_refusal(model: &SessionModel, path: &str) -> Option<String> {
    compile_session(
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
    .err()
    .map(|set| {
        set.diagnostics()
            .iter()
            .filter(|item| item.path.to_string() == path)
            .map(|item| item.code.to_string())
            .collect::<Vec<_>>()
            .join(",")
    })
}

/// Issue #1237 gate 2: a route value is in domain on the live path (`route_coefficients`, which
/// host-core's send producer and host-web's admission call) exactly when the session accepts it.
/// The values are gate 1's boundaries, each one `f32` step either side of them, and the
/// non-finite values, and subnormal values of both signs with the smallest normal; each is set on
/// the gain or on one coefficient of an otherwise unity route, under every gate. An accepted value
/// must also compile, so the lowering (`route_values` too) cannot refuse what the session accepts.
/// This replaces #1215's +700 dB overflow case: inside the domain no fold can overflow (the
/// largest product is about `15.85`).
///
/// Test value: red if the two paths' domains differ -- `route_values` left unbounded, bounded by
/// other limits than `validate_routes`, exclusive where the session is inclusive, refusing a
/// subnormal coefficient the session accepts, or checking only the open gate -- so a value the
/// session refuses could still be pushed live, or one it accepts refused live or at boot.
#[test]
fn a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it() {
    let base = parse_session_json(SESSION).expect("fixture parses");
    let id = base.routes[0].id.as_str().to_owned();
    let steps = |value: f32| [value.next_down(), value, value.next_up()];
    let finite = |bounds: [f32; 2]| {
        let mut values: Vec<f32> = bounds.into_iter().flat_map(steps).collect();
        values.extend([bounds[0] - 0.5, bounds[1] + 0.5, 0.0, -0.0]);
        values.extend([f32::NAN, f32::INFINITY, f32::NEG_INFINITY]);
        // Subnormal values of both signs, and the smallest normal (#1237 attempt 1 verdict J1-1):
        // in the session's domain, so the live path and the lowering must accept them too.
        let half_normal = f32::MIN_POSITIVE / 2.0;
        values.extend([1.0e-45, -1.0e-45, half_normal, -half_normal]);
        values.extend([f32::MIN_POSITIVE, -f32::MIN_POSITIVE]);
        values
    };
    let gates: Vec<(bool, [bool; 2])> = (0..8_u8)
        .map(|g| (g & 1 != 0, [g & 2 != 0, g & 4 != 0]))
        .collect();
    let unity = [1.0, 0.0, 0.0, 1.0];
    let mut refused = 0;
    for position in 0..5 {
        let (field, values) = if position == 0 {
            ("gain_db".to_owned(), finite([-144.0, 24.0]))
        } else {
            let key = ["ll", "lr", "rl", "rr"][position - 1];
            (format!("channel_matrix.{key}"), finite([-1.0, 1.0]))
        };
        let path = format!("$.routes[0].{field}");
        for value in values {
            let (mut gain_db, mut matrix) = (0.0, unity);
            if position == 0 {
                gain_db = value;
            } else {
                matrix[position - 1] = value;
            }
            let mut model = base.clone();
            model.routes[0].gain_db = gain_db;
            let channel = &mut model.routes[0].channel_matrix;
            [channel.ll, channel.lr, channel.rl, channel.rr] = matrix;
            let session = session_refusal(&model, &path);
            let expected = if value.is_finite() {
                "numeric.out_of_schema_range"
            } else {
                "numeric.non_finite"
            };
            if let Some(codes) = &session {
                assert_eq!(codes, expected, "{path} = {value}");
                refused += 1;
            }
            for &(mute, source_lane_muted) in &gates {
                let live = route_coefficients(gain_db, matrix, mute, source_lane_muted);
                assert_eq!(
                    live.is_err(),
                    session.is_some(),
                    "{path} = {value}, mute {mute}, lanes {source_lane_muted:?}: live {live:?}, \
                     session {session:?}"
                );
                if let Err(error) = live {
                    assert_eq!(error, RouteValueError::Domain);
                }
            }
            if session.is_none() {
                // An accepted value compiles: the lowering's `route_values` is the live check.
                compile(&model).unwrap_or_else(|codes| panic!("{id}: {path} = {value}: {codes:?}"));
            }
        }
    }
    // Each field refuses its two outer steps, its two half-steps and the three non-finite values.
    assert_eq!(refused, 5 * 7);
}

/// Issue #1237 gate 4 (the coefficients): a folded product that is subnormal, of either sign, is
/// `+0.0` in that position, from the plan's bound constants and from the live path alike. At
/// -144 dB (`6.31e-8`) a coefficient of `±1.0e-35` folds to about `±6.3e-43`; at 0 dB a subnormal
/// coefficient (`±1.0e-45`, `±f32::MIN_POSITIVE / 2`) is itself the product.
///
/// Test value: red if `gated_route_coefficients` returns the subnormal product, or flushes a
/// negative one to `-0.0`, or flushes the wrong position, or a normal product with it; and red if
/// a subnormal coefficient is refused at boot instead of compiling and binding `+0.0` (#1237
/// attempt 1 verdict J1-1).
#[test]
fn a_subnormal_folded_coefficient_is_positive_zero() {
    let base = parse_session_json(SESSION).expect("fixture parses");
    let tiny = 1.0e-35_f32;
    let half_normal = f32::MIN_POSITIVE / 2.0;
    let cases = [
        (-144.0_f32, [tiny, -tiny, 0.5, -tiny]),
        (0.0, [1.0e-45, -1.0e-45, 0.5, -half_normal]),
    ];
    for (gain_db, matrix) in cases {
        let gain = math::db_to_gain_f32(gain_db);
        assert!(
            [0, 1, 3]
                .into_iter()
                .all(|index| (gain * matrix[index]).is_subnormal()),
            "{gain_db} dB: the draw folds to a subnormal of each sign"
        );
        let expected = [0.0_f32, 0.0, gain * 0.5, 0.0];
        assert!(expected[2].is_normal());
        let mut model = base.clone();
        model.routes[0].gain_db = gain_db;
        let channel = &mut model.routes[0].channel_matrix;
        [channel.ll, channel.lr, channel.rl, channel.rr] = matrix;
        let artifact = compile(&model)
            .unwrap_or_else(|codes| panic!("{gain_db} dB: an in-domain route compiles: {codes:?}"));
        let id = model.routes[0].id.as_str().to_owned();
        let prepared = artifact
            .graph()
            .routes()
            .iter()
            .find(|route| {
                matches!(&route.node, graph::GraphNodeId::Route { route_id }
                    if route_id.as_str() == id)
            })
            .expect("the route is prepared");
        assert_eq!(
            bits(gated_route_coefficients(&prepared.transform, prepared.gate)),
            bits(expected),
            "{gain_db} dB: the plan's bound constants"
        );
        assert_eq!(
            bits(route_coefficients(gain_db, matrix, false, [false; 2]).expect("in domain")),
            bits(expected),
            "{gain_db} dB: the live path"
        );
    }
}
