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
        &launch_native_effect_registry().expect("launch registry"),
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 24,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("the native effects prepare");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
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
/// must also compile, so the lowering (`route_values` too) cannot refuse what the session accepts. This replaces #1215's +700 dB overflow case: inside the domain no fold can
/// overflow (the largest product is about `15.85`).
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

/// #1216 gate 5: a route's mute is in the sealed graph text. The same session compiles twice, one
/// route muted and then open: outside the `estimate` row (which charges #1217's route-activity
/// table), the texts differ by exactly one `route-mute\t<node>` row, right after that route's
/// `route-transform` row, and an open route writes no such row.
///
/// Red if the gate is invisible to the canonical text (two plans binding different constants
/// would share one digest), or if the row lands on another route or elsewhere in the text.
#[test]
fn a_muted_route_seals_one_route_mute_row_after_its_transform() {
    let text = |model: &SessionModel| {
        let artifact = compile(model).expect("the session compiles");
        String::from_utf8(
            GraphCompiler::evidence(artifact.graph(), artifact.report()).canonical_bytes,
        )
        .expect("canonical text is UTF-8")
    };
    let open_model = parse_session_json(SESSION).expect("fixture parses");
    let open = text(&open_model);
    let open_lines: Vec<&str> = open.lines().collect();
    assert!(
        open_lines
            .iter()
            .all(|line| !line.starts_with("route-mute")),
        "an open route writes no gate row"
    );
    // Three choices, so the row is placed by the route and not by the text's first or last route.
    for index in [0, 4, open_model.routes.len() - 1] {
        let mut muted_model = open_model.clone();
        muted_model.routes[index].mute = true;
        let node = format!("route:{}", muted_model.routes[index].id.as_str());
        let muted = text(&muted_model);
        let mut muted_lines: Vec<&str> = muted.lines().collect();
        let row = format!("route-mute\t{node}");
        let at = muted_lines
            .iter()
            .position(|line| *line == row)
            .unwrap_or_else(|| panic!("no `{row}` row"));
        assert!(
            muted_lines[at - 1].starts_with(&format!("route-transform\t{node}\t")),
            "`{row}` follows `{}`",
            muted_lines[at - 1]
        );
        muted_lines.remove(at);
        // A muted route also makes bind build the route-activity table, which the sealed
        // `estimate` row charges (#1217 D6; `route_activity.rs` holds the charge to what bind
        // retains). That row may differ only by exactly that charge, in the fields it enters.
        let estimate_at = |lines: &[&str]| {
            lines
                .iter()
                .position(|line| line.starts_with("estimate\t"))
                .expect("an estimate row")
        };
        let (open_at, muted_at) = (estimate_at(&open_lines), estimate_at(&muted_lines));
        assert_estimate_moves_by_the_route_activity_charge(
            open_lines[open_at],
            muted_lines[muted_at],
            open_model.routes.len(),
            &node,
        );
        muted_lines.remove(muted_at);
        let mut open_rest = open_lines.clone();
        open_rest.remove(open_at);
        assert_eq!(
            muted_lines, open_rest,
            "the gate row is the only difference outside the estimate ({node})"
        );
    }
}

/// The muted compile's `estimate` row equals the open one's but for #1217 D6's charge,
/// `graph::route_activity_bound_bytes(routes, logical_nodes)`: `graph_metadata_bytes`,
/// `incremental_plan_bytes` and `session_plus_plan_bytes` each grow by exactly the charge, and
/// `largest_allocation_bytes` is the larger of the open one and the muted metadata bytes.
fn assert_estimate_moves_by_the_route_activity_charge(
    open: &str,
    muted: &str,
    routes: usize,
    node: &str,
) {
    let fields = |row: &str| -> Vec<u64> {
        row.split('\t')
            .skip(1)
            .map(|field| field.parse().expect("a numeric estimate field"))
            .collect()
    };
    let (open, muted) = (fields(open), fields(muted));
    assert_eq!(open.len(), 20, "the estimate row's field count");
    // Field positions in the row (`canonical.rs`'s `estimate` writer).
    const LOGICAL_NODES: usize = 0;
    const GRAPH_METADATA: usize = 11;
    const LARGEST_ALLOCATION: usize = 17;
    const CHARGED: [usize; 3] = [GRAPH_METADATA, 18, 19];
    let charge = graph::route_activity_bound_bytes(routes as u64, open[LOGICAL_NODES])
        .expect("the charge fits u64");
    let mut expected = open.clone();
    for field in CHARGED {
        expected[field] += charge;
    }
    expected[LARGEST_ALLOCATION] = open[LARGEST_ALLOCATION].max(expected[GRAPH_METADATA]);
    assert_eq!(
        muted, expected,
        "the estimate moves by the route-activity charge ({charge} bytes) and nothing else ({node})"
    );
}

/// #1218 D3: a following send's zeroed source lanes are in the sealed graph text. A send from a
/// muted track compiles with `follows_mute` on and off: outside the `estimate` row (a send whose
/// two source lanes are muted is silenced, so #1217's activity table is charged), the texts differ
/// by exactly one `route-follow-zeroed\t<node>\t<l>\t<r>` row, after that route's
/// `route-transform` row and, when the send is also muted, its `route-mute` row.
///
/// Red if the follow is invisible to the canonical text (a follow-zeroed plan and an open one
/// would share a digest while binding different constants), if the lane bits are swapped, or if
/// the row lands before the route's `route-mute` row or on another route. The case where only the
/// follow silences the send (both lanes muted, the send itself open) is the one where the estimate
/// row moves: red there if the activity charge is keyed on the route's `mute` alone rather than on
/// the gate's `silences()` (#1218 verdict NIT-3).
#[test]
fn a_following_send_seals_one_route_follow_zeroed_row() {
    let text = |model: &SessionModel| {
        let artifact = compile(model).expect("the session compiles");
        String::from_utf8(
            GraphCompiler::evidence(artifact.graph(), artifact.report()).canonical_bytes,
        )
        .expect("canonical text is UTF-8")
    };
    let without_estimate = |text: &str| -> Vec<String> {
        text.lines()
            .filter(|line| !line.starts_with("estimate\t"))
            .map(str::to_owned)
            .collect()
    };
    let base = with_sends(&parse_session_json(SESSION).expect("fixture parses"));
    let track = base.tracks[3].id.as_str().to_owned();
    let send = format!("send-{track}");
    let node = format!("route:{send}");
    for (left, right, mute) in [
        (true, false, false),
        (false, true, false),
        (true, true, false),
        (true, true, true),
    ] {
        let mut open = base.clone();
        let fader = &mut open.tracks[3].fader;
        (fader.left_mute, fader.right_mute) = (left, right);
        let index = open
            .routes
            .iter()
            .position(|route| route.id.as_str() == send)
            .expect("the send is declared");
        open.routes[index].mute = mute;
        let mut following = open.clone();
        following.routes[index].follows_mute = true;
        let open_text = text(&open);
        assert!(
            open_text
                .lines()
                .all(|line| !line.starts_with("route-follow-zeroed")),
            "a send that does not follow writes no follow row"
        );
        let following_text = text(&following);
        let mut lines = without_estimate(&following_text);
        let row = format!(
            "route-follow-zeroed\t{node}\t{}\t{}",
            u8::from(left),
            u8::from(right)
        );
        let at = lines
            .iter()
            .position(|line| *line == row)
            .unwrap_or_else(|| panic!("no `{row}` row"));
        let previous = if mute {
            format!("route-mute\t{node}")
        } else {
            format!("route-transform\t{node}\t")
        };
        assert!(
            lines[at - 1].starts_with(&previous),
            "`{row}` follows `{}`",
            lines[at - 1]
        );
        lines.remove(at);
        assert_eq!(
            lines,
            without_estimate(&open_text),
            "the follow row is the only difference outside the estimate ({left}, {right}, {mute})"
        );
        let estimate = |text: &str| -> String {
            text.lines()
                .find(|line| line.starts_with("estimate\t"))
                .expect("an estimate row")
                .to_owned()
        };
        // Only the follow silences the send when both lanes are muted and the send is open; then,
        // and only then, the follow charges the route-activity table.
        let follow_alone_silences = left && right && !mute;
        assert_eq!(
            estimate(&following_text) != estimate(&open_text),
            follow_alone_silences,
            "the estimate moves exactly when the follow alone silences the send \
             ({left}, {right}, {mute})"
        );
    }
}
