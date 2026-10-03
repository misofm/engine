//! Issue #1215: every route's coefficients come from one function.
//!
//! A plan binds `graph::gated_route_coefficients(&route.transform, route.gate)` for each prepared
//! route, and a live producer will push `graph_compiler::route_coefficients` of the values it is
//! handed. The two layers agree only because the second calls the first; this file holds them to
//! it, pins which column each source lane's gate zeroes, and refuses a fold that overflows.
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
use session::{CompileCaps, SessionModel, compile_session, parse_session_json};

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
    /// A matrix coefficient: mostly a signed level up to +12 dB, sometimes an exact signed zero,
    /// so a gated `+0.0` is told apart from a product that was `-0.0` already.
    fn coefficient(&mut self) -> f32 {
        match self.next() % 8 {
            0 => 0.0,
            1 => -0.0,
            _ => self.uniform(-4.0, 4.0),
        }
    }
}

#[test]
fn every_route_coefficient_comes_from_the_one_gated_function() {
    let base = parse_session_json(SESSION).expect("fixture parses");
    let mut draw = Draw(0x1215_6761_7465_u64);
    let gates: Vec<(bool, [bool; 2])> = (0..8_u8)
        .map(|g| (g & 1 != 0, [g & 2 != 0, g & 4 != 0]))
        .collect();
    for round in 0..24 {
        let mut model = base.clone();
        let mut drawn = Vec::new();
        for route in &mut model.routes {
            route.gain_db = draw.uniform(-120.0, 24.0);
            let matrix = &mut route.channel_matrix;
            [matrix.ll, matrix.lr, matrix.rl, matrix.rr] = [(); 4].map(|()| draw.coefficient());
            // #1216 gate 2: the session's own switch, drawn per route.
            route.mute = draw.next().is_multiple_of(2);
            drawn.push((
                route.id.as_str().to_owned(),
                route.gain_db,
                [matrix.ll, matrix.lr, matrix.rl, matrix.rr],
                route.mute,
            ));
        }
        let artifact = compile(&model).expect("finite routes compile");
        let routes: &[PreparedRoute] = artifact.graph().routes();
        assert_eq!(routes.len(), drawn.len());
        for (id, gain_db, matrix, mute) in drawn {
            let context =
                format!("round {round}, route {id}: {gain_db} dB, {matrix:?}, mute {mute}");
            let prepared = routes
                .iter()
                .find(|route| {
                    matches!(&route.node, graph::GraphNodeId::Route { route_id }
                        if route_id.as_str() == id)
                })
                .unwrap_or_else(|| panic!("{context}: no prepared route"));
            // The plan's gate is the session's switch (#1216 D2); nothing else sets it yet.
            assert_eq!(
                prepared.gate,
                RouteGate {
                    mute,
                    follow_zeroed: [false; 2]
                },
                "{context}"
            );
            let checked = route_coefficients(gain_db, matrix, mute, [false; 2])
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

    // A finite gain times a finite coefficient can overflow. The overflow is refused whatever the
    // gate, so muting a route never admits values that would be infinite once it opens.
    let overflow = [1.0e10, 0.0, 0.0, 1.0];
    for &(mute, source_lane_muted) in &gates {
        assert_eq!(
            route_coefficients(700.0, overflow, mute, source_lane_muted),
            Err(RouteValueError::Domain),
            "mute {mute}, source lanes muted {source_lane_muted:?}"
        );
    }
    // The same gain with a unit matrix folds to a finite 3.2e34 and is not refused.
    assert!(route_coefficients(700.0, [1.0, 0.0, 0.0, 1.0], false, [false; 2]).is_ok());
    // A muted route's values are refused exactly as an open one's (#1216 D2).
    for mute in [false, true] {
        let mut model = base.clone();
        model.routes[0].gain_db = 700.0;
        model.routes[0].channel_matrix.ll = 1.0e10;
        model.routes[0].mute = mute;
        let id = model.routes[0].id.as_str().to_owned();
        assert_eq!(
            compile(&model).err(),
            Some(vec![(
                "graph.gain.non_finite".to_owned(),
                format!("$.routes[id={id}].gain_db")
            )]),
            "mute {mute}"
        );
        model.routes[0].channel_matrix.ll = 1.0;
        assert!(
            compile(&model).is_ok(),
            "a finite 700 dB fold compiles, mute {mute}"
        );
    }
}

/// #1216 gate 5: a route's mute is in the sealed graph text. The same session compiles twice, one
/// route muted and then open: the texts differ by exactly one `route-mute\t<node>` row, right after
/// that route's `route-transform` row, and an open route writes no such row.
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
        assert_eq!(
            muted_lines, open_lines,
            "the gate row is the only difference ({node})"
        );
    }
}
