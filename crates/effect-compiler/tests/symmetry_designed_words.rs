//! The `DESIGNED` term, per launch effect, against the real prepared objects.
//!
//! # What this proves and what it deliberately does not
//!
//! Each effect's `channel_symmetry` / `lane_channel_symmetry` compares the designed per-lane words
//! **its own kernel reads** -- the lists are enumerated in the doc comment above each
//! implementation, derived from the kernel's load sites rather than from the parameter table. This
//! file proves the comparison is wired to something real in both directions: an instance prepared
//! with equal per-channel initial values says yes, and the *same* instance prepared with one
//! parameter differing on one channel says no.
//!
//! It does not attempt to prove the word lists are complete -- no test can, since completeness is
//! a claim about which loads exist. The list is defended by citation in each implementation's doc
//! comment and by the conservative default: an effect that has not derived a witness declines, so
//! the failure mode of an incomplete derivation is a missed collapse, not a wrong render.

use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    EffectDescriptor, EffectQuality, InitialParameterValue, LinkMode, ParameterChannel, PortRole,
    PrepareEffectLimits, PrepareEffectRequest, PreparedPorts, PreparedSidechainPort,
    default_initial_values,
};

/// The descriptor's own conforming initial-value slice, with one channel of one parameter moved.
///
/// A prepared instance requires **every** declared slot, so a witness test cannot hand over two
/// entries; it starts from the defaults and moves exactly one word, which is also what makes the
/// symmetric and asymmetric arms differ in exactly one place.
fn initial_values(
    descriptor: &'static EffectDescriptor,
    prelude: &[(u32, f32)],
    parameter_index: u32,
    left: f32,
    right: f32,
) -> Vec<InitialParameterValue> {
    let mut values: Vec<_> = default_initial_values(descriptor).collect();
    // Applied to both channels, so it cannot itself be the asymmetry: it exists only to put the
    // effect in a state where the addressed parameter reaches a designed word at all -- a disabled
    // EQ band designs to the identity words whatever its gain is.
    for value in &mut values {
        if let Some((_, setting)) = prelude
            .iter()
            .find(|(index, _)| *index == value.parameter_index)
        {
            value.value = *setting;
        }
    }
    let mut matched = 0_usize;
    for value in &mut values {
        if value.parameter_index != parameter_index {
            continue;
        }
        match value.channel {
            ParameterChannel::Left => {
                value.value = left;
                matched += 1;
            }
            ParameterChannel::Right => {
                value.value = right;
                matched += 1;
            }
            // A `Shared` parameter has one slot for both channels and can never be asymmetric, so
            // naming one in a case would make that case vacuous.
            ParameterChannel::Both => panic!("the addressed parameter is not per-lane"),
        }
    }
    assert_eq!(matched, 2, "one left slot and one right slot");
    values
}

const SAMPLE_RATE: u32 = 48_000;
const QUANTUM: u32 = 128;

/// The unconnected form of whatever sidechain port the descriptor declares.
///
/// Preparation refuses a request that omits a declared port, and it is not this file's business
/// which effects have one; `Unconnected` is the internal-detector shape every launch effect here
/// runs in its default session.
fn ports(descriptor: &'static EffectDescriptor) -> PreparedPorts {
    let sidechain = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
        .map_or(PreparedSidechainPort::None, |port| {
            PreparedSidechainPort::Unconnected {
                id: port.id,
                required: port.required,
            }
        });
    PreparedPorts { sidechain }
}

fn request<'a>(
    descriptor: &'static EffectDescriptor,
    initial: &'a [InitialParameterValue],
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: SAMPLE_RATE,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: ports(descriptor),
        initial_values: initial,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 22,
            maximum_automation_spans_per_block: 128,
        },
    }
}

/// One effect, the parameter index whose per-channel disagreement must be visible, and two values
/// that design to different words.
struct Case {
    effect: &'static str,
    /// Both-channel settings applied first, so the addressed parameter reaches a designed word.
    prelude: &'static [(u32, f32)],
    parameter_index: u32,
    left: f32,
    right: f32,
}

/// Red mutation: in any one implementation, drop a word from the comparison (for example, stop
/// comparing a whole term such as the compressor's ramps or `delay`) -> that effect's asymmetric row fails while
/// every other row stays green, so the failure names the effect and the word.
#[test]
fn each_launch_effect_sees_its_own_designed_words_disagree() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let cases = [
        // `band-1-gain`, parameter id 4 -> index 3. A gain difference redesigns all six SVF words
        // of section 0.
        Case {
            effect: "miso.parametric-eq",
            // Band 1 enabled (index 0) as a peaking section (index 1) at 1 kHz (index 2). A
            // *disabled* band designs to the identity words whatever its gain is, which would have
            // made this case vacuous -- and is itself a small proof that the witness compares the
            // designed words rather than the parameter table.
            prelude: &[(0, 1.0), (1, 2.0), (2, 1_000.0)],
            parameter_index: 3,
            left: -6.0,
            right: 6.0,
        },
        // `threshold`, index 0: a passthrough coefficient word, so the disagreement is visible in
        // `words[COEF_THRESHOLD]` with no design arithmetic in between. It is *also* visible in
        // `ramps[0]`, because a settled ramp rests at the value it designed -- the two are
        // redundant here by the design law, not by accident, and the witness compares both because
        // both are in the kernel's read surface.
        Case {
            effect: "miso.compressor",
            prelude: &[],
            parameter_index: 0,
            left: -18.0,
            right: -6.0,
        },
        // `lookahead`, index 7: the one compressor parameter that is **not** ramped
        // (`apply_automation` refuses `parameter_index >= RAMP_COUNT`), so it reaches the kernel
        // only through `delay[lane]` and `lookahead_ms[lane]`. It is the case that proves those
        // two words are compared rather than covered by the ramp comparison.
        Case {
            effect: "miso.compressor",
            prelude: &[],
            parameter_index: 7,
            left: 0.0,
            right: 5.0,
        },
        // `ceiling`, index 0: retargets the limit ramp, which is one of the four per-lane words.
        Case {
            effect: "miso.true-peak-limiter",
            prelude: &[],
            parameter_index: 0,
            left: -3.0,
            right: -1.0,
        },
        // `lookahead`, index 2: the limiter's twin of the compressor case above, and the one that
        // covers the two words the ramp comparison cannot reach.
        //
        // It is `AutomationRate::None` and `SmoothingRule::None`, so it is not ramped at all: it
        // reaches the kernel only as `lane[l]` -- the van Herk window geometry
        // `LaneShape { window, end_offset, box_offset }`, leg one of the `lanes_uniform` gate that
        // chooses the uniform body over the general one -- and as `lookahead_ms[l]`, which
        // `commit_lane` and `reset_to_defaults` read. Neither is covered by `ceiling`'s ramp case,
        // so without this row an implementation that compared only the two `LinearRamp`s would
        // pass every limiter assertion in this file.
        //
        // Asymmetric lookahead is **legal and prepared**: the descriptor's channel policy is
        // `PerLane`, the declared latency is `lookahead_maximum + 6` regardless of the setting, and
        // preparation accepts the pair. Nothing but this comparison stands between such a track and
        // a collapse that would silently give one channel the other's window.
        Case {
            effect: "miso.true-peak-limiter",
            prelude: &[],
            parameter_index: 2,
            left: 0.0,
            right: 5.0,
        },
        // `delay-ms`, index 0: moves the tap geometry, which is what `fill_windows` reads.
        Case {
            effect: "miso.delay",
            prelude: &[],
            parameter_index: 0,
            left: 30.0,
            right: 120.0,
        },
    ];

    for case in cases {
        let factory = registry
            .get_ascii(case.effect)
            .unwrap_or_else(|| panic!("{} is a launch effect", case.effect));

        let descriptor = factory.descriptor();
        let symmetric = initial_values(
            descriptor,
            case.prelude,
            case.parameter_index,
            case.left,
            case.left,
        );
        let asymmetric = initial_values(
            descriptor,
            case.prelude,
            case.parameter_index,
            case.left,
            case.right,
        );

        let prepared = factory
            .prepare(request(descriptor, &symmetric))
            .unwrap_or_else(|failure| panic!("{}: {}", case.effect, failure.code));
        assert!(
            prepared.channel_symmetry(),
            "{}: equal per-channel initial values must design to equal words",
            case.effect
        );

        let prepared = factory
            .prepare(request(descriptor, &asymmetric))
            .unwrap_or_else(|failure| panic!("{}: {}", case.effect, failure.code));
        assert!(
            !prepared.channel_symmetry(),
            "{}: parameter {} differs between the channels and the witness did not see it",
            case.effect,
            case.parameter_index
        );
    }
}

/// The banked form of the same rule, lane by lane: exactly the asymmetric lane declines, and its
/// neighbours -- which share the vector -- do not. That is the no-cross-lane-coupling property the
/// witness must have, and a vector-wide comparison (an `all lanes agree` reduction, which the EQ's
/// own `identity` flags are) would fail it.
///
/// Red mutation: make `PreparedParametricEq::designed_channel_symmetry` compare whole vectors
/// instead of lane `lane` -> every lane declines and the `lane != 1` assertion fails.
#[test]
fn a_bank_declines_exactly_the_asymmetric_lane() {
    use effect_contract::{BankWidth, PrepareEffectBankRequest};
    use lane::Backend;

    let backend = Backend::current();
    let Some(width) = BankWidth::for_backend(backend) else {
        return;
    };
    let lanes = width.lanes() as usize;
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry
        .get_ascii("miso.parametric-eq")
        .expect("a launch effect");

    let descriptor = factory.descriptor();
    const EQ_PRELUDE: &[(u32, f32)] = &[(0, 1.0), (1, 2.0), (2, 1_000.0)];
    let symmetric = initial_values(descriptor, EQ_PRELUDE, 3, -6.0, -6.0);
    let asymmetric = initial_values(descriptor, EQ_PRELUDE, 3, -6.0, 6.0);

    let requests: Vec<_> = (0..lanes)
        .map(|lane| {
            request(
                descriptor,
                if lane == 1 {
                    &asymmetric[..]
                } else {
                    &symmetric[..]
                },
            )
        })
        .collect();
    let bank = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("bind")
        .expect("the EQ carries a homogeneous bank kernel");

    for lane in 0..lanes {
        assert_eq!(
            bank.lane_channel_symmetry(lane),
            lane != 1,
            "lane {lane} of {lanes}"
        );
    }
    assert!(
        !bank.lane_channel_symmetry(lanes),
        "a lane index the width does not have declines"
    );
}
