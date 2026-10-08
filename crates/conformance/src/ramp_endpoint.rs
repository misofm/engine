//! The effect ramp-endpoint harness (issue #1409 gate 2; one shared copy since issue #1458).
//!
//! Each launch effect's `tests/ramp_endpoint.rs` names its ramped payload words ([`RampWord`]) and
//! at least one in-domain move per word ([`Move`]), found by #1409's (or #1458's) scan over the
//! domain edges (for a designed word, over parameter pairs whose words are 33 to 4096 ulps apart),
//! whose word the unclamped D11 law (`current + step` before the snap) took past its target. This
//! module runs them; no effect is named here.
//!
//! * **[`check_every_move`].** Each move is rendered in one-frame blocks; after each, the effect's
//!   own state snapshot must hold every ramp word between its value at rest and its target. After
//!   the first frame, the unclamped law run from the moved word's rest value with the step the
//!   snapshot holds must leave that interval, so the move reaches the clamp. From the second frame
//!   on, every ramp word must be one clamped D11 sample of the word the snapshot held one frame
//!   before, bit for bit, or its target on the sample that ends the ramp (issue #1458 root
//!   ruling): a word that holds or stops short while its ramp is in flight, or snaps to its target
//!   while the ramp still counts, is red, which the endpoint check alone cannot see. The same
//!   window rendered as one block must give the same ramp words and, for a [`Move::whole`] move,
//!   the same output and final snapshot.
//! * **[`check_every_bank_move`]** (#1409 D4). In a bank of the native width, the last lane makes
//!   the move while every other lane rests, so each lane's target differs. After every one-frame
//!   block the moving lane's output and ramp words must be bit-identical to a scalar instance
//!   making the same move, which a site that clamps toward another lane's target fails.
//!
//! Every instance is prepared at the 48 kHz quality row (`qualities[1]`, the rate the moves were
//! scanned at), quantum 128, the dual-mono link mode and the tail-bound entry of a registry that
//! admitted the factory (issue #1462 D1).

use effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectId, EffectProcessBlock,
    InitialParameterValue, LinkMode, NativeEffectFactory, NativeEffectRegistry, ParameterChannel,
    ParameterChannelPolicy, PortRole, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedEffect, PreparedNativeEffect,
    PreparedPorts, PreparedSidechainPort, RegisteredTailBound, StatePayloadOutput,
    default_initial_values,
};

use crate::randomized::admit;

/// The payload section that holds a ramp.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Section {
    /// The common section (a shared parameter).
    Common,
    /// Both channel sections, the same word in each (a per-lane parameter).
    Channels,
}

/// One ramped word: the parameter that moves it, and where its four payload words
/// `(current, target, step, remaining)` start.
#[derive(Clone, Copy, Debug)]
pub struct RampWord {
    /// The word's name, printed in a failure.
    pub name: &'static str,
    /// The parameter index whose change starts this ramp.
    pub parameter: u32,
    /// The payload section that holds the ramp.
    pub section: Section,
    /// The index of the ramp's first `u32` word in its section.
    pub word: usize,
}

/// One in-domain move: `word`'s parameter from `start` to `target`, whose word the unclamped law
/// took past its target before the snap.
#[derive(Clone, Copy, Debug)]
pub struct Move {
    /// The moved word.
    pub word: RampWord,
    /// The parameter's value at preparation.
    pub start: f32,
    /// The value a `Point` span at sample 0 moves the parameter to.
    pub target: f32,
    /// Whether the move's output and whole snapshot are partition-invariant. A word that feeds a
    /// coefficient the effect refreshes once per segment is not always, by that effect's frozen
    /// design (the multiband compressor's high ratio and high attack moves); there only the ramp
    /// words are compared.
    pub whole: bool,
}

/// One effect's ramp-endpoint data: everything that differs between effects.
#[derive(Clone, Copy, Debug)]
pub struct RampEndpoints<'a> {
    /// Every ramp word of the effect's payload: each is checked on every move.
    pub words: &'a [RampWord],
    /// The moves, each of a word in `words`.
    pub moves: &'a [Move],
    /// Parameters every instance starts at instead of its descriptor default, as
    /// `(parameter index, value)` on every channel; a move's own `start` overrides an entry for its
    /// parameter. The rest values of every lane, the bank's resting lanes included.
    pub rest: &'a [(u32, f32)],
}

/// Frames rendered per move: the 64-sample window and a few settled frames after it.
const FRAMES: usize = 72;

/// The quantum every instance is prepared at and every block declares.
const QUANTUM: u32 = 128;

/// The factory under test, admitted into a registry, with the tail-bound entry of its 48 kHz row.
struct Admitted {
    registry: NativeEffectRegistry,
    effect: EffectId,
    tail_bound: RegisteredTailBound,
}

impl Admitted {
    fn new(factory: Box<dyn NativeEffectFactory>) -> Self {
        let (registry, effect) = admit(factory);
        let quality = registry
            .get(effect)
            .expect("the registry admitted the effect under test")
            .descriptor()
            .qualities[1];
        assert_eq!(quality.sample_rate, 48_000, "the moves are 48 kHz moves");
        let tail_bound = registry
            .tail_bound(effect, quality.sample_rate, quality.quality)
            .expect("the registry has an entry for every declared quality row");
        Self {
            registry,
            effect,
            tail_bound,
        }
    }

    fn factory(&self) -> &dyn NativeEffectFactory {
        self.registry
            .get(self.effect)
            .expect("the registry admitted the effect under test")
    }

    /// The preparation request with `values`, the sidechain connected when `connected`.
    ///
    /// # Panics
    ///
    /// When `connected` and the effect has no sidechain port: a connected run must connect.
    fn request<'v>(
        &self,
        values: &'v [InitialParameterValue],
        connected: bool,
    ) -> PrepareEffectRequest<'v> {
        let descriptor = self.factory().descriptor();
        let quality = descriptor.qualities[1];
        let port = descriptor
            .ports
            .iter()
            .find(|port| port.role == PortRole::SidechainInput);
        assert!(
            port.is_some() || !connected,
            "a connected run of an effect without a sidechain port"
        );
        let sidechain = port.map_or(PreparedSidechainPort::None, |port| {
            if connected {
                PreparedSidechainPort::Connected {
                    id: port.id,
                    required: port.required,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required: port.required,
                }
            }
        });
        let link = LinkMode::DualMono;
        assert!(descriptor.supported_link_modes.contains(link));
        PrepareEffectRequest {
            sample_rate: quality.sample_rate,
            quantum: QUANTUM,
            quality: quality.quality,
            bypass: false,
            link_mode: link,
            ports: PreparedPorts { sidechain },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: quality.maximum_state.total().unwrap_or(u64::MAX),
                maximum_scratch_bytes: (quality.scratch_bytes_per_frame * u64::from(QUANTUM)
                    + quality.scratch_fixed_bytes)
                    .max(1),
                maximum_automation_spans_per_block: 4,
            },
            tail_bound: self.tail_bound,
        }
    }

    fn prepare(&self, values: &[InitialParameterValue], connected: bool) -> PreparedEffect {
        self.factory()
            .prepare(self.request(values, connected))
            .expect("prepares")
    }

    /// The descriptor defaults with `rest` applied, then `parameter` at `value`, on every channel.
    fn values(&self, rest: &[(u32, f32)], moved: Option<(u32, f32)>) -> Vec<InitialParameterValue> {
        default_initial_values(self.factory().descriptor())
            .map(|mut initial| {
                for &(parameter, value) in rest.iter().chain(&moved) {
                    if initial.parameter_index == parameter {
                        initial.value = value;
                    }
                }
                initial
            })
            .collect()
    }

    /// `Point` spans moving `parameter` to `value` at sample 0, on every channel it has.
    fn move_spans(&self, parameter: u32, value: f32) -> Vec<PreparedAutomationSpan> {
        let channels: &[ParameterChannel] = match self.factory().descriptor().parameters
            [parameter as usize]
            .channel_policy
        {
            ParameterChannelPolicy::Shared => &[ParameterChannel::Both],
            ParameterChannelPolicy::PerLane => &[ParameterChannel::Left, ParameterChannel::Right],
        };
        channels
            .iter()
            .map(|&channel| PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel,
                parameter_index: parameter,
                start_sample: 0,
                end_sample: 0,
                start_value: value,
                end_value: value,
            })
            .collect()
    }
}

/// A state payload: common, left and right sections.
type Payload = (Vec<u8>, Vec<u8>, Vec<u8>);

/// A prepared instance's whole payload, sized by the metadata its preparation derived.
fn snapshot(prepared: &PreparedEffect) -> Payload {
    let sizes = prepared.metadata.state_sizes;
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    prepared
        .processor
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("a prepared instance snapshots");
    (common, left, right)
}

fn word_u32(bytes: &[u8], word: usize) -> u32 {
    u32::from_le_bytes(
        bytes[word * 4..word * 4 + 4]
            .try_into()
            .expect("four bytes"),
    )
}

fn word_f32(bytes: &[u8], word: usize) -> f32 {
    f32::from_bits(word_u32(bytes, word))
}

/// The `(current, target, step, remaining)` of `ramp` in each section that holds it.
fn ramps(payload: &Payload, ramp: RampWord) -> Vec<(f32, f32, f32, u32)> {
    let sections: Vec<&Vec<u8>> = match ramp.section {
        Section::Common => vec![&payload.0],
        Section::Channels => vec![&payload.1, &payload.2],
    };
    sections
        .into_iter()
        .map(|bytes| {
            (
                word_f32(bytes, ramp.word),
                word_f32(bytes, ramp.word + 1),
                word_f32(bytes, ramp.word + 2),
                word_u32(bytes, ramp.word + 3),
            )
        })
        .collect()
}

/// A deterministic test signal.
fn signal(frame: usize, channel: usize) -> f32 {
    ((frame * 37 + channel * 11) % 97) as f32 / 97.0 - 0.5
}

/// Renders `frames` frames from `first` with `spans`; the sidechain carries signal when
/// `connected`.
fn render(
    effect: &mut dyn PreparedNativeEffect,
    first: usize,
    frames: usize,
    spans: &[PreparedAutomationSpan],
    connected: bool,
) -> (Vec<f32>, Vec<f32>) {
    let mut left: Vec<f32> = (first..first + frames)
        .map(|frame| signal(frame, 0))
        .collect();
    let mut right: Vec<f32> = (first..first + frames)
        .map(|frame| signal(frame, 1))
        .collect();
    let side: Vec<f32> = (first..first + frames)
        .map(|frame| 0.7 * signal(frame, 2))
        .collect();
    let block = EffectProcessBlock::new(
        &mut left,
        &mut right,
        connected.then_some((&side[..], &side[..])),
        first as u64,
        spans,
        QUANTUM,
    )
    .expect("a well-shaped block");
    let _ = effect.process(block);
    (left, right)
}

/// The first frame (1-based) at which the unclamped law's word `w0 + step + ... + step` leaves
/// `[min(w0, target), max(w0, target)]` before the snap of a `samples`-sample ramp.
fn unclamped_first_out(w0: f32, target: f32, step: f32, samples: u32) -> Option<u32> {
    let (low, high) = (w0.min(target), w0.max(target));
    let mut word = w0;
    (1..samples).find(|_| {
        word += step;
        !(low..=high).contains(&word)
    })
}

/// Whether `now` is one sample of the clamped D11 law on the snapshot ramp `previous`, both as
/// `(current, target, step, remaining)`. At rest (`remaining` all zero bits, as either encoding an
/// effect uses, `u32` or `f32`, writes it) the word holds. In flight it either takes
/// `lane::kernels::ramp_toward(current, step, target)`, or snaps to its target on the sample that
/// ends the ramp (`remaining` is zero bits after it); the target never changes. So a word that
/// holds, or stops short, while its ramp is in flight fails, unless the step is too small to move
/// it, where `ramp_toward` holds it too; and a snap to the target while the ramp still counts
/// fails (issue #1458 batch follow-ups), unless `ramp_toward` itself reached the target.
fn follows_law(previous: (f32, f32, f32, u32), now: (f32, f32, f32, u32)) -> bool {
    let (current, target, step, remaining) = previous;
    let same = |a: f32, b: f32| a.to_bits() == b.to_bits();
    if !same(now.1, target) {
        return false;
    }
    if remaining == 0 {
        return same(now.0, current);
    }
    same(now.0, lane::kernels::ramp_toward(current, step, target))
        || (same(now.0, target) && now.3 == 0)
}

/// Runs every move of `endpoints` on `factory` once per entry of `sidechain` (the sidechain
/// connected when `true`): the one-frame endpoint check and the one-block partition comparison
/// (module doc).
///
/// # Panics
///
/// On any violation, naming the move, the word, the frame and the sidechain state; when a `true`
/// entry meets an effect without a sidechain port; and when a move's word is not in `words`.
pub fn check_every_move(
    factory: Box<dyn NativeEffectFactory>,
    endpoints: &RampEndpoints<'_>,
    sidechain: &[bool],
) {
    let admitted = Admitted::new(factory);
    for &connected in sidechain {
        for &mv in endpoints.moves {
            check_move(&admitted, endpoints, mv, connected);
        }
    }
}

fn check_move(admitted: &Admitted, endpoints: &RampEndpoints<'_>, mv: Move, connected: bool) {
    let words = endpoints.words;
    let context = format!(
        "{} from {:e} ({:#010x}) to {}, sidechain connected {connected}",
        mv.word.name,
        mv.start,
        mv.start.to_bits(),
        mv.target
    );
    let samples =
        admitted.factory().descriptor().parameters[mv.word.parameter as usize].smoothing_samples;
    let values = admitted.values(endpoints.rest, Some((mv.word.parameter, mv.start)));
    let spans = admitted.move_spans(mv.word.parameter, mv.target);
    let moved = words
        .iter()
        .position(|word| word.word == mv.word.word && word.name == mv.word.name)
        .expect("the moved word is in the table");
    let mut framewise = admitted.prepare(&values, connected);
    let rest: Vec<_> = words
        .iter()
        .map(|word| ramps(&snapshot(&framewise), *word))
        .collect();
    let (mut left, mut right) = (Vec::new(), Vec::new());
    let mut previous = snapshot(&framewise);
    for frame in 0..FRAMES {
        let first = if frame == 0 { &spans[..] } else { &[] };
        let (l, r) = render(framewise.processor.as_mut(), frame, 1, first, connected);
        left.extend(l);
        right.extend(r);
        let now = snapshot(&framewise);
        if frame > 0 {
            for word in words {
                let before = ramps(&previous, *word);
                for (section, (before, now)) in
                    before.into_iter().zip(ramps(&now, *word)).enumerate()
                {
                    assert!(
                        follows_law(before, now),
                        "{context}: frame {frame}, `{}` (section {section}) went from {before:?} \
                         to {now:?}, not one clamped D11 sample",
                        word.name
                    );
                }
            }
        }
        for (word, rest) in words.iter().zip(&rest) {
            for (section, (rest, now)) in rest.iter().zip(ramps(&now, *word)).enumerate() {
                let (low, high) = (rest.0.min(now.1), rest.0.max(now.1));
                assert!(
                    (low..=high).contains(&now.0),
                    "{context}: after frame {frame}, `{}` (section {section}) is {:e} \
                     ({:#010x}), outside [{low:e}, {high:e}]",
                    word.name,
                    now.0,
                    now.0.to_bits()
                );
            }
        }
        if frame == 0 {
            for (section, (rest, now)) in rest[moved].iter().zip(ramps(&now, mv.word)).enumerate() {
                assert!(
                    unclamped_first_out(rest.0, now.1, now.2, samples).is_some(),
                    "{context}: section {section}: the unclamped law stays inside, so this move \
                     does not reach the clamp"
                );
            }
        }
        previous = now;
    }
    let mut whole = admitted.prepare(&values, connected);
    let (whole_left, whole_right) = render(whole.processor.as_mut(), 0, FRAMES, &spans, connected);
    let (whole_state, framewise_state) = (snapshot(&whole), snapshot(&framewise));
    for word in words {
        assert_eq!(
            ramps(&whole_state, *word),
            ramps(&framewise_state, *word),
            "{context}: `{}` ends on a partition-dependent ramp",
            word.name
        );
    }
    if mv.whole {
        let bits = |samples: &[f32]| samples.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&whole_left), bits(&left), "{context}: left output");
        assert_eq!(bits(&whole_right), bits(&right), "{context}: right output");
        assert!(
            whole_state == framewise_state,
            "{context}: the final snapshot depends on the partition"
        );
    }
}

/// Runs every move of `endpoints` on the last lane of a bank of `factory` at the native width,
/// every other lane resting, against a scalar instance making the same move (module doc). The
/// sidechain is unconnected. A build without a native bank width returns at once; CI runs each
/// width.
///
/// # Panics
///
/// On any lane-against-scalar difference, naming the move, the word and the frame; and when the
/// effect does not bind a native bank.
pub fn check_every_bank_move(factory: Box<dyn NativeEffectFactory>, endpoints: &RampEndpoints<'_>) {
    let admitted = Admitted::new(factory);
    for &mv in endpoints.moves {
        check_bank_move(&admitted, endpoints, mv);
    }
}

fn check_bank_move(admitted: &Admitted, endpoints: &RampEndpoints<'_>, mv: Move) {
    let context = format!(
        "{} from {:e} to {} in a bank",
        mv.word.name, mv.start, mv.target
    );
    // A build executes one bank width, the native one; CI runs each.
    let Some(width) = BankWidth::for_backend(lane::Backend::current()) else {
        return;
    };
    let lanes = width.lanes() as usize;
    let moved = lanes - 1;
    let resting = admitted.values(endpoints.rest, None);
    let values = admitted.values(endpoints.rest, Some((mv.word.parameter, mv.start)));
    let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
        .map(|lane| admitted.request(if lane == moved { &values } else { &resting }, false))
        .collect();
    let mut bank = admitted
        .factory()
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: width.backend(),
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("binds")
        .expect("this effect banks natively")
        .processor;
    let mut scalar = admitted.prepare(&values, false);
    let spans = admitted.move_spans(mv.word.parameter, mv.target);
    let sizes = scalar.metadata.state_sizes;
    for frame in 0..FRAMES {
        let first: &[PreparedAutomationSpan] = if frame == 0 { &spans } else { &[] };
        let mut offsets = vec![0_u32; lanes + 1];
        offsets[lanes] = first.len() as u32;
        let mut left = vec![signal(frame, 0); lanes];
        let mut right = vec![signal(frame, 1); lanes];
        let _ = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                1,
                width,
                frame as u64,
                first,
                &offsets,
                QUANTUM,
            )
            .expect("a well-shaped bank block"),
        );
        let (l, r) = render(scalar.processor.as_mut(), frame, 1, first, false);
        assert_eq!(
            (left[moved].to_bits(), right[moved].to_bits()),
            (l[0].to_bits(), r[0].to_bits()),
            "{context}: output at frame {frame}"
        );
        let mut lane_payload: Payload = (
            vec![0_u8; sizes.common_bytes as usize],
            vec![0_u8; sizes.left_bytes as usize],
            vec![0_u8; sizes.right_bytes as usize],
        );
        bank.snapshot_track_state_payload(
            moved as u32,
            StatePayloadOutput::new(
                &mut lane_payload.0,
                &mut lane_payload.1,
                &mut lane_payload.2,
                sizes,
            )
            .expect("sizes"),
        )
        .expect("a bank lane snapshots");
        let scalar_payload = snapshot(&scalar);
        for word in endpoints.words {
            let bits = |payload: &Payload| {
                ramps(payload, *word)
                    .iter()
                    .map(|ramp| (ramp.0.to_bits(), ramp.1.to_bits(), ramp.2.to_bits(), ramp.3))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                bits(&lane_payload),
                bits(&scalar_payload),
                "{context}: `{}` after frame {frame}",
                word.name
            );
        }
    }
}
