//! One runtime model, shared by both executors.
//!
//! The sequential executor and the native dependency-wave executor used to carry two copies of
//! every piece of node semantics: two reductions, two route loops, two effect-block constructions,
//! two node builders, two bank loops. They now share this module and differ only in *where the
//! audio lives* -- the sequential executor colours one arena and reads its producers in place,
//! the native one gives every parcel its own arena and stages every edge across partitions
//! (#98 F7).
//!
//! Everything here is derived from the lowered [`ExecutionProgram`](crate::program) (#99 F2), so
//! the ops, their input order, the identity aliases and the buffer colouring are a pure function
//! of the semantic graph rather than a second opinion about it.
//!
//! ## Arena
//!
//! Audio lives in one [`DisjointArena`](engine::realtime::DisjointArena) per prepared
//! plan: two planar `f32` planes of `buffers * frames` words, reached only through a checked
//! [`ArenaLease`]. The lease API is the one implementation of *where the audio is* as well as
//! of what happens to it.
//!
//! The sequential executor holds a single lease over the whole coloured arena. A delayed edge
//! copies, and that copy is made by the consuming op through the [`RuntimeOp::staged`] list.
//!
//! ## Frozen arithmetic
//!
//! * Reductions are master plan #83 D9: stable edge-ID order, left-to-right, block-wide, through
//!   [`sum2_block`] and [`sum_into_block`]. Fan-in 0 zero-fills, fan-in 1 copies (which preserves
//!   `-0.0`), fan-in `n >= 2` is `out = in0 + in1` then `out += in_k` for each remaining input.
//! * Routes are D3: the linear gain is folded into the 2x2 coefficients **once, at bind**, and
//!   render spends one multiply and one [`Lane::fma`](lane::Lane::fma) per output word
//!   through [`mix2x2_block`].
//! * Compensation delays are two-segment slice exchanges through [`pdc_delay_block`]; there is no
//!   per-sample work and no `%` on the render path.
//! * The graph performs **no** sanitisation (D7). Input sanitisation is the input stage's, output
//!   finiteness is the bank boundary check.

use std::collections::BTreeMap;

use core::num::NonZeroUsize;

use engine::realtime::{
    ARENA_SILENCE_BUFFER, ArenaLease, ArenaLeaseSetBuilder, ArenaStereoPair, RenderError,
    ResponseSnapshotAvailability, ResponseSnapshotError, ResponseSnapshotOwnerInfo,
    ResponseSnapshotSection, ResponseSnapshotSink,
};

/// The arena reserves buffer zero as the always-zero silence slot, so every executor buffer is
/// offset by one.
pub(crate) const ARENA_BASE: u32 = 1;

// Private qualification hooks count actual planar acquisition reads, not transpose accounting.
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static TEST_ONLY_RESIDENT_DISABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static TEST_ONLY_RESIDENT_COUNTS: std::cell::Cell<[u64; 2]> = const { std::cell::Cell::new([0; 2]) };
    static TEST_ONLY_METER_RESIDENT_DISABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static TEST_ONLY_METER_INPUT_COUNTS: std::cell::Cell<[u64; 3]> = const { std::cell::Cell::new([0; 3]) };
    static TEST_ONLY_OBSERVATION_DISPATCH_COUNTS: std::cell::Cell<[u64; 2]> =
        const { std::cell::Cell::new([0; 2]) };
}

/// Reset the test-only counts of prepared observer and bank-member object accesses.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_observation_dispatch_reset() {
    TEST_ONLY_OBSERVATION_DISPATCH_COUNTS.with(|value| value.set([0; 2]));
}

/// Return `[observer object accesses, bank member accesses]` from prepared dispatch.
#[cfg(any(test, feature = "test-support"))]
#[must_use]
#[doc(hidden)]
pub fn test_only_observation_dispatch_counts() -> [u64; 2] {
    TEST_ONLY_OBSERVATION_DISPATCH_COUNTS.with(std::cell::Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
#[inline]
fn test_only_observation_dispatch_observer_access() {
    TEST_ONLY_OBSERVATION_DISPATCH_COUNTS.with(|value| {
        let mut counts = value.get();
        counts[0] += 1;
        value.set(counts);
    });
}

#[cfg(any(test, feature = "test-support"))]
#[inline]
fn test_only_observation_dispatch_member_access() {
    TEST_ONLY_OBSERVATION_DISPATCH_COUNTS.with(|value| {
        let mut counts = value.get();
        counts[1] += 1;
        value.set(counts);
    });
}

// Issue #900: calls to the permanent path's per-op observer walk, and the switch that restores the
// unconditional unit walk so one test build can compare both paths. Unit tests only: neither the
// count nor the switch exists in a production or `test-support` build.
#[cfg(test)]
thread_local! {
    static TEST_ONLY_OBSERVE_CALLS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static TEST_ONLY_OBSERVER_SKIP_DISABLED: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}

/// Reset the `observe` call count; `skip_disabled` makes `observe_unit` walk every unit again.
#[cfg(test)]
pub(crate) fn test_only_observe_calls_reset(skip_disabled: bool) {
    TEST_ONLY_OBSERVER_SKIP_DISABLED.with(|value| value.set(skip_disabled));
    TEST_ONLY_OBSERVE_CALLS.with(|value| value.set(0));
}

/// Calls to the permanent path's per-op observer walk, `observe`, since the last reset.
#[cfg(test)]
pub(crate) fn test_only_observe_calls() -> u64 {
    TEST_ONLY_OBSERVE_CALLS.with(std::cell::Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
pub fn test_only_meter_input_reset(disabled: bool) {
    TEST_ONLY_METER_RESIDENT_DISABLED.with(|value| value.set(disabled));
    TEST_ONLY_METER_INPUT_COUNTS.with(|value| value.set([0; 3]));
}

/// Actual `[planar acquisitions, resident offers, resident accepts]`, including accepted errors.
#[cfg(any(test, feature = "test-support"))]
pub fn test_only_meter_input_counts() -> [u64; 3] {
    TEST_ONLY_METER_INPUT_COUNTS.with(std::cell::Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
pub fn test_only_resident_input_reset(disabled: bool) {
    TEST_ONLY_RESIDENT_DISABLED.with(|value| value.set(disabled));
    TEST_ONLY_RESIDENT_COUNTS.with(|value| value.set([0; 2]));
}

#[cfg(any(test, feature = "test-support"))]
pub fn test_only_resident_input_counts() -> [u64; 2] {
    TEST_ONLY_RESIDENT_COUNTS.with(std::cell::Cell::get)
}

/// A bounded, render-local witness for the private post-fader buffer at a failed render boundary.
///
/// This exists only for the split-owner qualification fixture. The capture is copied into fixed
/// storage on the render thread and read after the call returns; it is not a production diagnostic
/// or an observer path.
#[cfg(any(test, feature = "test-support"))]
const TEST_ONLY_FAILED_BUFFER_CAPACITY: usize = 128;

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestOnlyFailedBufferCapture {
    pub captured: bool,
    pub overflow: bool,
    pub frames: usize,
    pub left: [u32; TEST_ONLY_FAILED_BUFFER_CAPACITY],
    pub right: [u32; TEST_ONLY_FAILED_BUFFER_CAPACITY],
}

#[cfg(any(test, feature = "test-support"))]
impl Default for TestOnlyFailedBufferCapture {
    fn default() -> Self {
        Self {
            captured: false,
            overflow: false,
            frames: 0,
            left: [0; TEST_ONLY_FAILED_BUFFER_CAPACITY],
            right: [0; TEST_ONLY_FAILED_BUFFER_CAPACITY],
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static TEST_ONLY_FAILED_BUFFER_TARGET: std::cell::Cell<Option<u32>> = const { std::cell::Cell::new(None) };
    static TEST_ONLY_FAILED_BUFFER: std::cell::Cell<TestOnlyFailedBufferCapture> =
        const { std::cell::Cell::new(TestOnlyFailedBufferCapture {
            captured: false,
            overflow: false,
            frames: 0,
            left: [0; TEST_ONLY_FAILED_BUFFER_CAPACITY],
            right: [0; TEST_ONLY_FAILED_BUFFER_CAPACITY],
        }) };
    static TEST_ONLY_COMPLETION_DISABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_arm_failed_buffer_capture(buffer: u32) {
    TEST_ONLY_FAILED_BUFFER_TARGET.with(|target| target.set(Some(buffer)));
    TEST_ONLY_FAILED_BUFFER.with(|capture| capture.set(TestOnlyFailedBufferCapture::default()));
}

#[cfg(any(test, feature = "test-support"))]
#[must_use]
#[doc(hidden)]
pub fn test_only_failed_buffer_capture() -> TestOnlyFailedBufferCapture {
    TEST_ONLY_FAILED_BUFFER.with(std::cell::Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_set_completion_disabled(disabled: bool) {
    TEST_ONLY_COMPLETION_DISABLED.with(|value| value.set(disabled));
}

#[cfg(any(test, feature = "test-support"))]
#[must_use]
#[doc(hidden)]
pub fn test_only_completion_disabled() -> bool {
    TEST_ONLY_COMPLETION_DISABLED.with(std::cell::Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestOnlySelectedSplitFader {
    pub node: GraphNodeId,
    pub buffer: u32,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static TEST_ONLY_SELECTED_SPLIT_FADER:
        std::cell::RefCell<Option<TestOnlySelectedSplitFader>> = const { std::cell::RefCell::new(None) };
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_reset_selected_split_fader() {
    TEST_ONLY_SELECTED_SPLIT_FADER.with(|selected| *selected.borrow_mut() = None);
}

#[cfg(any(test, feature = "test-support"))]
#[must_use]
#[doc(hidden)]
pub fn test_only_selected_split_fader() -> Option<TestOnlySelectedSplitFader> {
    TEST_ONLY_SELECTED_SPLIT_FADER.with(|selected| selected.borrow().clone())
}

#[cfg(any(test, feature = "test-support"))]
fn test_only_record_selected_split_fader(node: GraphNodeId, buffer: u32) {
    TEST_ONLY_SELECTED_SPLIT_FADER.with(|selected| {
        *selected.borrow_mut() = Some(TestOnlySelectedSplitFader { node, buffer });
    });
}
use effect_contract::{
    BypassShunt, ChannelSymmetryWitness, EffectControlLane, EffectProcessBlock, ObservationLane,
    ObservationSample, PreparedAutomationSpan, PreparedNativeEffect, ResponseAnalysisError,
    ResponseSnapshotKind, ResponseSnapshotRequest as OwnerSnapshotRequest, ResponseSnapshotSummary,
    transpose_tile_4, transpose_tile_8,
};
use lane::Lane;
use lane::kernels::{mix2x2_block, ordered_accumulate_block, pdc_delay_block, sum_into_block};
use rack::{BankChain, BankMembers, BankPlaneViews, FoldCohort, ResidentFoldCohort};

use crate::observation_activation::{
    ActivationBinding, ActivationEntry, GraphObservationActivationConfig,
    GraphObservationAdmissionError, GraphObservationController, RealtimeObservationActivation,
    prepare_activation,
};
use crate::{
    GraphBindingBlock, GraphEdgeId, GraphNodeObserverBinding, GraphObservationBlock,
    GraphObservationValidity, GraphPreparedEffect, GraphRuntimeProcessor,
    GraphRuntimeSplitPairProcessor,
};

/// Lane type the block kernels are instantiated at to vectorise **over frames**.
///
/// Frames are independent, so this is purely a width choice: master plan #83 §4.2 pins every one
/// of these kernels to a width-independent result, and gate G2 proves it, so the rendered bits do
/// not depend on which arm of this `cfg` a target takes.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) type FrameLane = lane::Simd8;
/// See [`FrameLane`].
#[cfg(any(
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
pub(crate) type FrameLane = lane::Simd4;
/// See [`FrameLane`].
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
)))]
pub(crate) type FrameLane = f32;

// REALTIME_POLICY_BEGIN

/// The session Output op's storage for one block: the host's two output planes (issue #916).
///
/// Before this issue, the Output op wrote an arena buffer and `GraphExecutor::render` copied that
/// buffer into the host's planes after the last unit. Now the planes themselves are that storage.
/// `render` takes them once per block, [`HostMaster::new`] checks that each is exactly
/// `lease.frames()` words, and the executor threads this one value through every unit of the
/// block:
///
/// * [`Runtime::execute`] takes a reborrow of it. The unit that runs the Output op
///   ([`Runtime::output_unit`]) reduces and processes into these planes instead of its arena
///   buffer. A folded chain whose master op is the Output op ([`FoldTarget::Output`]) accumulates
///   its epilogue into them.
/// * [`Runtime::observe_unit`] and [`Runtime::observe_active_unit`] take a shared borrow. The
///   Output op's observers read these planes.
///
/// Both writers are picked by **node**, at bind, and never by comparing a buffer index with the
/// session output's. The colouring may give the Output op's physical slot to a buffer that
/// retired before it. The standing console workloads give it track zero's input slot (see
/// [`route_fold`]'s ledger). So `op.output == program.output` holds for those earlier ops too,
/// and would send their audio here.
///
/// A later parameter of `execute` (a source set's planes, say) goes after this one, so every call
/// site keeps one shape.
///
/// # Precondition: nothing reads the Output's arena slot
///
/// On this path the Output's arena slot is never written: the Output op, and any fold into it,
/// write these planes instead. So a plan in which some op read the Output's value out of the arena
/// would read stale words. `preflight_sequential` refuses such a plan with
/// `graph.scheduler.layout`, before any owner moves. The refused shapes are an Output op with a
/// reader, and an op after the Output op that names its slot (`output_value_is_read`). The graph
/// compiler never emits either.
pub(crate) struct HostMaster<'a> {
    left: &'a mut [f32],
    right: &'a mut [f32],
}

impl<'a> HostMaster<'a> {
    /// The host's planes, if both are exactly `frames` words: the arena block the Output op's
    /// buffer would have been, so every kernel that wrote that buffer writes these unchanged.
    pub(crate) fn new(left: &'a mut [f32], right: &'a mut [f32], frames: usize) -> Option<Self> {
        (left.len() == frames && right.len() == frames).then_some(Self { left, right })
    }
}

impl HostMaster<'_> {
    /// The same planes for one call, handed back when it returns.
    pub(crate) fn reborrow(&mut self) -> HostMaster<'_> {
        HostMaster {
            left: &mut *self.left,
            right: &mut *self.right,
        }
    }

    /// Both planes, shared: what the Output op's observers read.
    fn planes(&self) -> (&[f32], &[f32]) {
        (&*self.left, &*self.right)
    }

    /// Both planes, exclusively: what the Output op and a folded Output master write.
    fn planes_mut(&mut self) -> (&mut [f32], &mut [f32]) {
        (&mut *self.left, &mut *self.right)
    }

    /// `+0.0` over both planes. It runs on the executor's failure paths only, so a host that
    /// ignores the error plays silence rather than a partly written master.
    pub(crate) fn silence(&mut self) {
        self.left.fill(0.0);
        self.right.fill(0.0);
    }
}

/// D9 reduction of one plane: stable edge order, left-to-right, block-wide.
///
/// `inputs` are buffer indices in `spec.edges` order, which the compiler sorts by `GraphEdgeId`.
/// A single input is a copy -- or nothing at all when the op reads its producer in place, which is
/// how the lowering removes a pass-through's copy -- so `-0.0` survives; `a + b` then accumulates
/// left to right, exactly as the scalar reference `inputs.reduce(|a, b| a + b)` does.
#[inline]
fn reduce_plane(lease: &mut ArenaLease, plane: usize, out: u32, inputs: &[u32]) {
    match inputs {
        [] => lease.write(plane, out).fill(0.0),
        [single] => {
            if *single != out {
                let (output, input) = lease.write_read(plane, out, *single);
                output.copy_from_slice(input);
            }
        }
        [_, _, ..] => reduce_many::<FrameLane>(lease, plane, out, inputs),
    }
}

/// Reads one arena borrow forms at once: [`ArenaLease::write_read_many`] takes one to eight.
const REDUCE_GROUP: usize = 8;

/// Fan-in two or more: `((in0 + in1) + in2) + ...` per frame, in edge order (issue #898).
///
/// The inputs are taken in consecutive groups of up to [`REDUCE_GROUP`]. One `write_read_many`
/// call forms a group's output and input slices once, and one pass walks them together in
/// `chunks_exact` steps, so no slice is re-derived per vector. The first group stores
/// `in0 + in1 + ...`; each later group loads that running sum back from the output and keeps
/// adding. A store and a reload move an `f32`'s bits unchanged, and every add still takes the
/// running sum and the next input in the same order, so the result is the one left-to-right chain
/// at every fan-in. Starting a later group from a fresh subtotal instead would not be:
/// `reduction_is_left_to_right_bit_identical_to_scalar_reference` pins the difference.
#[inline(always)]
fn reduce_many<L: Lane>(lease: &mut ArenaLease, plane: usize, out: u32, inputs: &[u32]) {
    debug_assert!(
        inputs.len() >= 2,
        "fan-in zero is a fill and fan-in one a copy"
    );
    for (index, group) in inputs.chunks(REDUCE_GROUP).enumerate() {
        let initial_store = index == 0;
        let reduced = match group.len() {
            1 => reduce_group::<L, 1>(lease, plane, out, group, initial_store),
            2 => reduce_group::<L, 2>(lease, plane, out, group, initial_store),
            3 => reduce_group::<L, 3>(lease, plane, out, group, initial_store),
            4 => reduce_group::<L, 4>(lease, plane, out, group, initial_store),
            5 => reduce_group::<L, 5>(lease, plane, out, group, initial_store),
            6 => reduce_group::<L, 6>(lease, plane, out, group, initial_store),
            7 => reduce_group::<L, 7>(lease, plane, out, group, initial_store),
            8 => reduce_group::<L, 8>(lease, plane, out, group, initial_store),
            _ => false,
        };
        // Unreachable for a lowered program: a multi-input op's output is a fresh slot, never one
        // of its own reads, because a slot retires one op after its last reader. Were a group ever
        // refused, the later groups must not add into a running sum that was never stored.
        debug_assert!(reduced, "a reduction group's arena borrow was refused");
        if !reduced {
            return;
        }
    }
}

/// One group of `N` consecutive inputs: one arena borrow, one pass over the output.
#[inline]
fn reduce_group<L: Lane, const N: usize>(
    lease: &mut ArenaLease,
    plane: usize,
    out: u32,
    group: &[u32],
    initial_store: bool,
) -> bool {
    let Ok(ids) = <&[u32; N]>::try_from(group) else {
        return false;
    };
    let Some((output, sources)) = lease.write_read_many(plane, out, ids) else {
        return false;
    };
    accumulate_group::<L, N>(output, sources, initial_store)
}

/// `output = s0 + s1 + ...` when `initial_store`, otherwise `output = ((output + s0) + s1) + ...`,
/// per frame and left to right, in one pass over `output`.
///
/// The vector body runs at `L`, and the frames that do not fill a whole vector run the same body
/// at `L = f32`, as every D9 kernel finishes its tail, so the result does not depend on the width.
/// The one shape check happens before the first write.
#[inline(always)]
fn accumulate_group<L: Lane, const N: usize>(
    output: &mut [f32],
    sources: [&[f32]; N],
    initial_store: bool,
) -> bool {
    let frames = output.len();
    if sources.iter().any(|source| source.len() != frames) {
        return false;
    }
    let vectored = frames - frames % L::WIDTH;
    let (vectors, tail) = output.split_at_mut(vectored);
    accumulate_run::<L, N>(
        vectors,
        sources.map(|source| &source[..vectored]),
        initial_store,
    ) && accumulate_run::<f32, N>(
        tail,
        sources.map(|source| &source[vectored..]),
        initial_store,
    )
}

/// One width's share of [`accumulate_group`]. `output` and every source have one common length,
/// a multiple of `L::WIDTH`, and each source is walked by its own `chunks_exact` iterator in step
/// with the output's.
///
/// The store and accumulate forms are two loops rather than one branch per vector, so each loop
/// walks a fixed number of sources.
#[inline(always)]
fn accumulate_run<L: Lane, const N: usize>(
    output: &mut [f32],
    sources: [&[f32]; N],
    initial_store: bool,
) -> bool {
    let mut chunks = sources.map(|source| source.chunks_exact(L::WIDTH));
    if initial_store {
        let Some((first, rest)) = chunks.split_first_mut() else {
            return false;
        };
        for out in output.chunks_exact_mut(L::WIDTH) {
            let Some(acc) = first
                .next()
                .and_then(|chunk| add_chunks(L::load(chunk), rest))
            else {
                return false;
            };
            acc.store(out);
        }
    } else {
        for out in output.chunks_exact_mut(L::WIDTH) {
            let Some(acc) = add_chunks(L::load(out), &mut chunks) else {
                return false;
            };
            acc.store(out);
        }
    }
    true
}

/// `((acc + c0) + c1) + ...` over the next chunk of every source, or `None` if one has run out.
#[inline(always)]
fn add_chunks<L: Lane>(mut acc: L, chunks: &mut [core::slice::ChunksExact<'_, f32>]) -> Option<L> {
    for chunk in chunks {
        acc = acc.add(L::load(chunk.next()?));
    }
    Some(acc)
}

/// [`reduce_plane`] for the session Output op, whose destination is the host's plane `target`
/// rather than its arena buffer `out` (issue #916).
///
/// The same three arms, over the same inputs in the same order, with the same kernels:
///
/// * **Fan-in zero** fills `target`.
/// * **Fan-in one** copies the single input into `target`. The Output is dedicated storage, so the
///   lowering never puts it in place over its producer. A single input equal to `out` is
///   therefore only ever a folded master's own output (`build_sequential`). Its epilogues have
///   already accumulated into `target`, so this arm leaves it alone, as `reduce_plane` does.
/// * **Fan-in two or more** is [`reduce_many`] with the arena write swapped for `target`. Each
///   group's inputs come from [`ArenaLease::read`]. It takes `&self`, so a group's `N` shared
///   slices coexist. [`accumulate_group`] is the loop [`reduce_group`] runs: the first group
///   stores and each later group reloads and adds, so every frame is the one left-to-right chain.
#[inline]
fn reduce_plane_into(
    lease: &ArenaLease,
    plane: usize,
    out: u32,
    target: &mut [f32],
    inputs: &[u32],
) {
    match inputs {
        [] => target.fill(0.0),
        [single] => {
            if *single != out {
                target.copy_from_slice(lease.read(plane, *single));
            }
        }
        [_, _, ..] => reduce_many_into::<FrameLane>(lease, plane, target, inputs),
    }
}

/// [`reduce_many`] into the host's plane: the same [`REDUCE_GROUP`] chunking, the same
/// `initial_store` for the first group only, and the same refusal rule.
#[inline(always)]
fn reduce_many_into<L: Lane>(lease: &ArenaLease, plane: usize, target: &mut [f32], inputs: &[u32]) {
    debug_assert!(
        inputs.len() >= 2,
        "fan-in zero is a fill and fan-in one a copy"
    );
    for (index, group) in inputs.chunks(REDUCE_GROUP).enumerate() {
        let initial_store = index == 0;
        let reduced = match group.len() {
            1 => reduce_group_into::<L, 1>(lease, plane, target, group, initial_store),
            2 => reduce_group_into::<L, 2>(lease, plane, target, group, initial_store),
            3 => reduce_group_into::<L, 3>(lease, plane, target, group, initial_store),
            4 => reduce_group_into::<L, 4>(lease, plane, target, group, initial_store),
            5 => reduce_group_into::<L, 5>(lease, plane, target, group, initial_store),
            6 => reduce_group_into::<L, 6>(lease, plane, target, group, initial_store),
            7 => reduce_group_into::<L, 7>(lease, plane, target, group, initial_store),
            8 => reduce_group_into::<L, 8>(lease, plane, target, group, initial_store),
            _ => false,
        };
        // As in `reduce_many`: a refused group ends the reduction rather than let a later group
        // add into a running sum that was never stored. Unreachable: every input of a lowered op
        // is a reserved buffer, and a host plane is exactly `lease.frames()` words.
        debug_assert!(reduced, "a reduction group into the host plane was refused");
        if !reduced {
            return;
        }
    }
}

/// One group of `N` consecutive inputs into the host's plane: `N` shared reads, one pass.
#[inline]
fn reduce_group_into<L: Lane, const N: usize>(
    lease: &ArenaLease,
    plane: usize,
    target: &mut [f32],
    group: &[u32],
    initial_store: bool,
) -> bool {
    let Ok(ids) = <&[u32; N]>::try_from(group) else {
        return false;
    };
    let sources = (*ids).map(|input| lease.read(plane, input));
    accumulate_group::<L, N>(target, sources, initial_store)
}

/// Where the session Output op's fused reduction reads each of its inputs (issue #927): the arena
/// buffer [`route_reduce`] names, or, for an input whose source claim is read **in place**, the
/// source set's played planes for this block.
///
/// An input is read in place only when [`source_plane_table`] bound its claim there under clause
/// (b'): the retired route over its buffer was the claim's only reader, so the fused reduction is
/// the only read of the words the executor's copy loop would have written, and the loop skips the
/// claim. The words are the copy's: the played block's on `Some`, and on `None` (underrun, end of
/// region) the arena's silence buffer, which is the `+0.0` the copy fills an unplayed quantum with
/// -- exactly what [`ArenaMembers::plane`] serves a bank's gather (issue #918).
#[derive(Clone, Copy)]
struct OutputSources<'a> {
    /// `None` in a plan with no source set.
    planes: Option<&'a dyn crate::GraphSourcePlanes>,
    /// [`Runtime`]'s `output_sources`: empty, or one claim (or [`NO_SOURCE_CLAIM`]) per input of
    /// the Output op, in its edge order. Selected by input **position**, never by buffer: see
    /// [`source_plane_table`] for why a buffer does not identify a claim's value.
    claims: &'a [u32],
}

impl<'a> OutputSources<'a> {
    /// Every input from the arena: what every op but the session Output op is handed, and what the
    /// Output op is handed in a plan that reads no input in place.
    const NONE: Self = Self {
        planes: None,
        claims: &[],
    };

    /// Input `position`'s two planes: its claim's played planes, or the silence buffer on `None`,
    /// if that input is read in place; otherwise arena buffer `buffer`, as before the issue. One
    /// table read and one predictable branch per input per block; nothing is copied.
    #[inline]
    fn input<'b>(
        self,
        lease: &'b ArenaLease,
        position: usize,
        buffer: u32,
    ) -> (&'b [f32], &'b [f32])
    where
        'a: 'b,
    {
        let claim = self
            .claims
            .get(position)
            .copied()
            .unwrap_or(NO_SOURCE_CLAIM);
        if claim != NO_SOURCE_CLAIM
            && let Some(planes) = self.planes
        {
            return match planes.played_planes(claim as usize) {
                Some(played) => {
                    #[cfg(any(test, feature = "test-support"))]
                    test_only_count_source_plane(1);
                    played
                }
                None => {
                    #[cfg(any(test, feature = "test-support"))]
                    test_only_count_source_plane(2);
                    lease.read_stereo(ARENA_SILENCE_BUFFER)
                }
            };
        }
        lease.read_stereo(buffer)
    }
}

/// The session Output op's reduction with each contributor's route fused in, one **pair** of
/// inputs at a time (issue #926; the fold's eligibility is issue #920's, [`output_route_fold`]).
///
/// Input `i` is the buffer a plain route ran in place over, and `routes[i]` is that route's
/// folded 2x2. The route op itself is retired at bind, so the buffer holds the route's *input*.
/// This computes, per frame and per plane, exactly the value the route op and then
/// [`reduce_many_into`] computed:
///
/// * **The mix** is [`lane::kernels::mix2x2_block`]'s, operand for operand:
///   `left = lr.fma(r, ll.mul(l))` and `right = rr.fma(r, rl.mul(l))`, from the input's original
///   `l` and `r`. The route op stored that word and the reduction loaded it back; a store and a
///   load move an `f32`'s bits unchanged, so taking the word from a register instead is the same
///   word. `Lane::fma` is two roundings on every backend, so the vector body and the `f32` tail
///   agree lane for lane, as they do in `mix2x2_block`.
/// * **The sum** is [`reduce_many`]'s one left-to-right chain, `((m0 + m1) + m2) + ...` in edge
///   order. The first pair stores `m0 + m1`, with the first contributor taken as the value, so a
///   `-0.0` survives (the fan-in is at least two, so the first pair is always whole); every later
///   pair, the odd fan-in's lone last input included, reloads the running sum from the host plane
///   and adds its `m_i` left to right. Where the chain is stored and reloaded does not matter: a store and a
///   load move no bit, which is the argument [`reduce_many`] already makes for its group
///   boundaries. So groups of two here and groups of [`REDUCE_GROUP`] there are the same chain.
///
/// **Why pairs.** Each pair hoists its eight coefficients as splats before its chunk loop, and
/// that loop keeps eight coefficients, two running sums, two loads and two temporaries live,
/// fourteen of sixteen AVX2 registers. A group of four or eight spills its coefficients, and a
/// per-chunk broadcast costs a load-port operation per coefficient per chunk; the pair was the
/// fastest of every group size and coefficient policy measured (`docs/handoffs/plumbing-floor-
/// 2026-09-26/PLAN.md`, table C). An odd fan-in's last input is the same body at `G = 1`.
///
/// **Why the tail is outlined.** The frames that do not fill a vector run the same body at
/// `L = f32`, as every D9 kernel finishes its tail, but in [`route_tail`]: a separate, non-generic,
/// never-inlined function. Inlined here, LLVM unrolls the up-to-three `f32` tail frames of the
/// wasm build's four-lane instantiation into three scalar operations per vector one, and the
/// AudioWorklet artifact gate (`scripts/check-web-audioworklet-callgraph.py`, rule 3) rejects any
/// function instantiated at the four-lane type whose scalar arithmetic is not strictly below its
/// vector arithmetic. Issue #920's kernel failed it exactly that way (560 vector against 1,680
/// scalar). This function is itself never inlined, so its four-lane instantiation stays one named
/// symbol that the gate inspects on every build; the call is once per block.
///
/// Both planes are formed in one pass per pair, because both mixes read both input planes: each
/// input word is loaded once where the two-pass form would load it twice. Each output plane's
/// arithmetic does not depend on the other's, so the pass order changes no bit. Within a pair,
/// the vector frames run before the tail frames, and frames are independent, so each frame still
/// sees the pairs in edge order.
///
/// `false`, before the first write, for a table that does not match the inputs, a fan-in below
/// two, or host planes that are not `lease.frames()` words (which `HostMaster::new` already
/// rules out). The caller turns `false` into an error.
#[inline(never)]
fn route_reduce<L: Lane>(
    lease: &ArenaLease,
    inputs: &[u32],
    routes: &[[f32; 4]],
    sources: OutputSources<'_>,
    left: &mut [f32],
    right: &mut [f32],
) -> bool {
    let frames = lease.frames();
    if inputs.len() != routes.len()
        || inputs.len() < 2
        || (!sources.claims.is_empty() && sources.claims.len() != inputs.len())
        || left.len() != frames
        || right.len() != frames
    {
        return false;
    }
    let vectored = frames - frames % L::WIDTH;
    for (index, (pair, table)) in inputs.chunks(2).zip(routes.chunks(2)).enumerate() {
        let initial_store = index == 0;
        let first = 2 * index;
        let reduced = match (pair, table) {
            (&[first_input, second_input], &[first_route, second_route]) => route_pair::<L, 2>(
                lease,
                sources,
                first,
                [first_input, second_input],
                &[first_route, second_route],
                left,
                right,
                vectored,
                initial_store,
            ),
            (&[only], &[only_route]) => route_pair::<L, 1>(
                lease,
                sources,
                first,
                [only],
                &[only_route],
                left,
                right,
                vectored,
                initial_store,
            ),
            _ => false,
        };
        if !reduced {
            return false;
        }
    }
    true
}

/// One pair of routed inputs (`G = 2`), or an odd fan-in's last input (`G = 1`), into both host
/// planes: the vector frames at `L` here, then the tail frames, if any, in [`route_tail`].
///
/// `first` is the pair's first input position, and each input's planes are what
/// [`OutputSources::input`] serves for it: the arena buffer `ids` names, or its claim's played
/// block (issue #927). Both are formed once per input per block, before any word is read, and
/// every word the loops below read comes from them. Every input plane is `lease.frames()` words:
/// [`ArenaLease::read`]'s, and a played plane is held to the quantum by the source set
/// ([`crate::GraphSourcePlanes`]), which is the lease's frames. [`route_reduce`] has checked the
/// host planes against it, so the shape check below never fails; it is what lets the slicing below
/// compile without a bounds check.
#[expect(
    clippy::too_many_arguments,
    reason = "the pair's reads, its table and the host planes stay explicit parameters"
)]
#[inline(always)]
fn route_pair<L: Lane, const G: usize>(
    lease: &ArenaLease,
    sources: OutputSources<'_>,
    first: usize,
    ids: [u32; G],
    table: &[[f32; 4]; G],
    left: &mut [f32],
    right: &mut [f32],
    vectored: usize,
    initial_store: bool,
) -> bool {
    let planes: [(&[f32], &[f32]); G] =
        core::array::from_fn(|member| sources.input(lease, first + member, ids[member]));
    let lefts = planes.map(|(left, _)| left);
    let rights = planes.map(|(_, right)| right);
    let frames = left.len();
    if vectored > frames
        || right.len() != frames
        || lefts
            .iter()
            .chain(rights.iter())
            .any(|plane| plane.len() != frames)
    {
        return false;
    }
    let (left_vectors, left_tail) = left.split_at_mut(vectored);
    let (right_vectors, right_tail) = right.split_at_mut(vectored);
    route_run::<L, G>(
        left_vectors,
        right_vectors,
        lefts.map(|plane| &plane[..vectored]),
        rights.map(|plane| &plane[..vectored]),
        table,
        initial_store,
    ) && (left_tail.is_empty()
        || route_tail(
            left_tail,
            right_tail,
            &lefts.map(|plane| &plane[vectored..]),
            &rights.map(|plane| &plane[vectored..]),
            table,
            initial_store,
        ))
}

/// The tail frames of one pair (or lone input) at `f32`: [`route_run::<f32, G>`](route_run), the
/// same body the vector frames run, in a function of its own.
///
/// Non-generic and never inlined on purpose, and not for speed (it runs only when the quantum is
/// not a multiple of the lane width): see [`route_reduce`], "Why the tail is outlined". Its `f32`
/// arithmetic is therefore never counted against the four-lane instantiation of the vector kernel.
///
/// A tail is shorter than every lane width, so a run as long as the widest lane (eight frames) is
/// refused before any write. The bound is not a check of the caller: it is what stops LLVM from
/// vectorising this function's loops. Without it, the wasm build vectorised both accumulate forms
/// here (24 `f32x4` operations beside the scalar body); with it, the trip count is at most seven
/// and the function carries no vector arithmetic at all.
#[inline(never)]
fn route_tail(
    left: &mut [f32],
    right: &mut [f32],
    lefts: &[&[f32]],
    rights: &[&[f32]],
    table: &[[f32; 4]],
    initial_store: bool,
) -> bool {
    if left.len() >= <lane::Simd8 as Lane>::WIDTH {
        return false;
    }
    match (lefts, rights, table) {
        (
            &[first_left, second_left],
            &[first_right, second_right],
            &[first_route, second_route],
        ) => route_run::<f32, 2>(
            left,
            right,
            [first_left, second_left],
            [first_right, second_right],
            &[first_route, second_route],
            initial_store,
        ),
        (&[only_left], &[only_right], &[only_route]) => route_run::<f32, 1>(
            left,
            right,
            [only_left],
            [only_right],
            &[only_route],
            initial_store,
        ),
        _ => false,
    }
}

/// One width's share of one pair: every slice has one common length, a multiple of `L::WIDTH`.
///
/// The pair's coefficients are splatted once, before the loop: eight for a pair, four for a lone
/// input. The pair's four input planes and the two host planes are then walked together by
/// `chunks_exact(L::WIDTH)` in one loop. Per chunk: the first pair's value is `mix(in0)`, a later
/// pair's is `load(out) + mix(in0)`; then `+ mix(in1)`; then the store. As in
/// [`accumulate_run`], the store and accumulate forms are two loops, so neither branches per
/// chunk.
#[inline(always)]
fn route_run<L: Lane, const G: usize>(
    left: &mut [f32],
    right: &mut [f32],
    lefts: [&[f32]; G],
    rights: [&[f32]; G],
    table: &[[f32; 4]; G],
    initial_store: bool,
) -> bool {
    let coefficients = table.map(|route| route.map(L::splat));
    let mut left_chunks = lefts.map(|plane| plane.chunks_exact(L::WIDTH));
    let mut right_chunks = rights.map(|plane| plane.chunks_exact(L::WIDTH));
    let outputs = left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH));
    if initial_store {
        let (Some((first, rest)), Some((first_left, rest_left)), Some((first_right, rest_right))) = (
            coefficients.split_first(),
            left_chunks.split_first_mut(),
            right_chunks.split_first_mut(),
        ) else {
            return false;
        };
        for (out_left, out_right) in outputs {
            let Some(acc) = first_left
                .next()
                .zip(first_right.next())
                .map(|(l, r)| mix_chunk(first, l, r))
                .and_then(|acc| add_mixed_chunks(acc, rest, rest_left, rest_right))
            else {
                return false;
            };
            acc.0.store(out_left);
            acc.1.store(out_right);
        }
    } else {
        for (out_left, out_right) in outputs {
            let running = (L::load(out_left), L::load(out_right));
            let Some(acc) =
                add_mixed_chunks(running, &coefficients, &mut left_chunks, &mut right_chunks)
            else {
                return false;
            };
            acc.0.store(out_left);
            acc.1.store(out_right);
        }
    }
    true
}

/// One route's `mix2x2_block` over one chunk, in registers: `(lr.fma(r, ll.mul(l)),
/// rr.fma(r, rl.mul(l)))`, the kernel's frozen operation order.
#[inline(always)]
fn mix_chunk<L: Lane>([ll, lr, rl, rr]: &[L; 4], left: &[f32], right: &[f32]) -> (L, L) {
    let (l, r) = (L::load(left), L::load(right));
    (lr.fma(r, ll.mul(l)), rr.fma(r, rl.mul(l)))
}

/// `((acc + mix(c0)) + mix(c1)) + ...` per plane over the next chunk of every routed source, or
/// `None` if one has run out.
#[inline(always)]
fn add_mixed_chunks<L: Lane>(
    mut acc: (L, L),
    coefficients: &[[L; 4]],
    lefts: &mut [core::slice::ChunksExact<'_, f32>],
    rights: &mut [core::slice::ChunksExact<'_, f32>],
) -> Option<(L, L)> {
    for ((route, left), right) in coefficients.iter().zip(lefts).zip(rights) {
        let (mixed_left, mixed_right) = mix_chunk(route, left.next()?, right.next()?);
        acc = (acc.0.add(mixed_left), acc.1.add(mixed_right));
    }
    Some(acc)
}

// REALTIME_POLICY_END

/// Integer-sample plugin-delay compensation for one stereo edge.
///
/// The line is `samples` words per channel and the block is exchanged with it in at most two
/// contiguous segments per channel ([`pdc_delay_block`], #98 F3). A block longer than the line is
/// walked in `samples`-word segments, which is still slice-only: no per-sample loop and no `%`.
pub(crate) struct CompensationDelay {
    left: Box<[f32]>,
    right: Box<[f32]>,
    cursor: usize,
}

impl CompensationDelay {
    pub(crate) fn new(samples: usize) -> Self {
        Self {
            left: vec![0.0; samples].into_boxed_slice(),
            right: vec![0.0; samples].into_boxed_slice(),
            cursor: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn samples(&self) -> usize {
        self.left.len()
    }

    #[cfg(test)]
    pub(crate) fn reset(&mut self) {
        self.left.fill(0.0);
        self.right.fill(0.0);
        self.cursor = 0;
    }
}

// REALTIME_POLICY_BEGIN
impl CompensationDelay {
    pub(crate) fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let samples = self.left.len();
        if samples == 0 {
            return;
        }
        let mut offset = 0;
        while offset < left.len() {
            let take = core::cmp::min(samples, left.len() - offset);
            let cursor = self.cursor;
            let mut left_cursor = cursor;
            pdc_delay_block(
                &mut self.left,
                &mut left_cursor,
                &mut left[offset..offset + take],
            );
            let mut right_cursor = cursor;
            pdc_delay_block(
                &mut self.right,
                &mut right_cursor,
                &mut right[offset..offset + take],
            );
            debug_assert_eq!(left_cursor, right_cursor);
            self.cursor = left_cursor;
            offset += take;
        }
    }
}
// REALTIME_POLICY_END

/// One track's **input-side time alignment**: PDC's ring shape and PDC's kernel, deliberately
/// none of PDC's accounting.
///
/// # Why this is not a `CompensationDelay`
///
/// `CompensationDelay` is one length for both lanes -- PDC computes a single per-edge skew, so its
/// two rings are the same size and share one cursor (its `debug_assert_eq!` on the two cursors is
/// that invariant written down). Track delay is declared **per lane** (`builtins.left.delay_samples`
/// and `builtins.right.delay_samples` are independent words, per the dual-mono law), so the two
/// rings can differ in length and their cursors advance independently. Rather than loosen PDC's
/// type -- and move bytes under every plan that has nothing to do with this feature -- the two
/// lanes get one independent ring and cursor each, driven by the same `pdc_delay_block` kernel.
///
/// # Why it is not latency
///
/// PDC equalizes *unrequested* arrival-time skew: `pdc::timings` computes every path's arrival
/// from declared node latency and inserts compensating delays to make them agree. A track delay is
/// the opposite -- a time shift the session asked for. Declaring it as node latency would make PDC
/// insert matching delays on every other path and cancel exactly the alignment the user wanted. So
/// a `TrackDelay` node contributes **zero** to `GraphNode.latency`, contributes nothing to
/// `TimingResult::total_delay`, and never appears in `inserted_delays`. Its bytes are charged to
/// the estimate separately (see `estimate::resource_estimate`); its samples are charged to nothing.
pub(crate) struct TrackDelayLine {
    left: Box<[f32]>,
    right: Box<[f32]>,
    left_cursor: usize,
    right_cursor: usize,
}

impl TrackDelayLine {
    pub(crate) fn new(left: usize, right: usize) -> Self {
        Self {
            left: vec![0.0; left].into_boxed_slice(),
            right: vec![0.0; right].into_boxed_slice(),
            left_cursor: 0,
            right_cursor: 0,
        }
    }

    /// This line's designed-word comparison: the two lanes' declared delays, compared exactly.
    ///
    /// # The word list, and why it is exactly this
    ///
    /// `process` reads one designed word per lane -- the ring length, which **is**
    /// `delay_samples` for that lane. The cursors and the ring contents are running state, not
    /// designed words, and are excluded for the same reason `InputChainState` is excluded from the
    /// input stage's list.
    ///
    /// A track whose lanes declare different delays produces genuinely different left and right
    /// audio out of a single source channel, so it is not mono-collapsible. That verdict is also
    /// taken at prepare, from the session, by `session_structural_symmetry`; this is the same
    /// fact answered by the object that owns the rings.
    #[cfg(test)]
    pub(crate) const fn channels_agree(&self) -> bool {
        self.left.len() == self.right.len()
    }

    #[cfg(test)]
    pub(crate) const fn lane_samples(&self) -> [usize; 2] {
        [self.left.len(), self.right.len()]
    }
}

// REALTIME_POLICY_BEGIN
impl TrackDelayLine {
    pub(crate) fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        delay_lane(&mut self.left, &mut self.left_cursor, left);
        delay_lane(&mut self.right, &mut self.right_cursor, right);
    }
}

/// One lane of a track delay: the same two-segment swap `CompensationDelay::process` runs, over one
/// ring with its own cursor. No `%`, no per-sample loop, and a block longer than the ring is walked
/// in ring-sized takes exactly as the PDC caller loop walks it.
fn delay_lane(ring: &mut [f32], cursor: &mut usize, block: &mut [f32]) {
    let samples = ring.len();
    if samples == 0 {
        return;
    }
    let mut offset = 0;
    while offset < block.len() {
        let take = core::cmp::min(samples, block.len() - offset);
        pdc_delay_block(ring, cursor, &mut block[offset..offset + take]);
        offset += take;
    }
}
// REALTIME_POLICY_END

/// What an op does to its reduced output.
pub(crate) enum NodeKind {
    /// A stage boundary, a submix or the session output: the reduction is the whole node.
    Identity,
    /// A track input filled by the coordinator's source set: no reduction, no processing.
    SourceInput,
    /// A track input filled by the coordinator's source set, then time-aligned in place.
    ///
    /// This variant **subsumes** [`NodeKind::SourceInput`] rather than sitting beside it. An input
    /// node has no graph inputs and the coordinator has already written its output buffer for this
    /// block, so there is no reduction to run: `reduce_plane` over an empty input list would
    /// `fill(0.0)` straight over the source audio. `execute_op` therefore takes this arm in the
    /// same early-return position the `SourceInput` arm occupies, keeps that arm's fill semantics
    /// (there is no fill), and delays the buffer in place.
    ///
    /// A track that declares zero delay on both lanes is never lowered to this variant -- it falls
    /// through to `SourceInput` exactly as before, so the compiled program of an undelayed session
    /// is structurally identical to the program that session compiled to before this feature
    /// existed.
    TrackDelay {
        /// Index into the runtime's track-delay lines.
        line: u32,
        /// Whether the two lanes declared the **same** delay, cached from the lowering.
        ///
        /// The witness is asked without the delay lines in hand, and `TrackDelayLine` is where the
        /// truth lives; `the_node_witness_agrees_with_its_line` keeps the two from drifting.
        channels_agree: bool,
    },
    /// A host-supplied processor.
    Bound(Box<dyn GraphRuntimeProcessor>),
    /// A track-local prepared native effect.
    Effect(GraphPreparedEffect),
    /// A track-local prepared native effect that a live console drives (issue #140 A).
    ///
    /// A separate variant from [`NodeKind::Effect`] on purpose, in the shape #137 D1 fixed for
    /// `ConsoleMatrixProcessor`: the console-free arm keeps the exact `&[]` call and the exact
    /// storage it had before this issue, so "a session with no console renders the same bits and
    /// holds the same bytes" is a property of the code rather than a claim about it.
    ConsoleEffect(Box<ConsoleEffect>),
    /// A route's 2x2 matrix, with the route gain already folded in (D3).
    Route([f32; 4]),
    /// A homogeneous-bank member: the reduction gathers its input, the bank does the work.
    BankMember,
}

#[derive(Clone, Copy)]
pub(crate) enum SplitPairRole {
    Fader,
    Matrix,
}

#[derive(Clone, Copy)]
pub(crate) struct SplitPairSlot {
    pub(crate) pair: usize,
    pub(crate) role: SplitPairRole,
}

/// One prepared native effect plus everything its live-console channel needs (issue #140 A).
///
/// Sized once, at bind, from the effect's own prepared metadata: the staging window is exactly
/// `PreparedEffectMetadata::automation_capacity` spans and the shunt's delay line is exactly
/// `PreparedEffectMetadata::latency` samples. Render allocates nothing and frees nothing.
pub(crate) struct ConsoleEffect {
    pub(crate) effect: GraphPreparedEffect,
    control: Box<EffectControlLane>,
    /// `automation_capacity` spans; only `[..staged]` is ever handed to the effect.
    spans: Box<[PreparedAutomationSpan]>,
    /// Latency-preserving dry path, so live bypass keeps the effect's declared latency exactly
    /// and therefore leaves every compiled PDC route timing correct.
    shunt: BypassShunt,
    /// Issue #143 D3: this instance's observation taps, or `None` in a plan with no observation
    /// capacity. `None` is one null pointer and one predicted branch per block -- and, crucially,
    /// it is the *only* observation state such a plan holds.
    observation: Option<Box<ObservationLane>>,
}

impl ConsoleEffect {
    fn new(
        effect: GraphPreparedEffect,
        control: Box<EffectControlLane>,
        observation: Option<Box<ObservationLane>>,
        frames: usize,
    ) -> Self {
        let capacity = effect.metadata.automation_capacity as usize;
        let latency = usize::try_from(effect.metadata.latency.0).unwrap_or(usize::MAX);
        Self {
            observation,
            spans: vec![
                PreparedAutomationSpan {
                    kind: effect_contract::AutomationSpanKind::Point,
                    channel: effect_contract::ParameterChannel::Both,
                    parameter_index: 0,
                    start_sample: 0,
                    end_sample: 0,
                    start_value: 0.0,
                    end_value: 0.0,
                };
                capacity
            ]
            .into_boxed_slice(),
            shunt: BypassShunt::new(frames, latency),
            effect,
            control,
        }
    }
}

// REALTIME_POLICY_BEGIN
/// Publish every armed tap of one prepared instance, after `process` returned (issue #143 D2).
///
/// **After**, always. The reading is "the value at the end of the block", so taking it before
/// `process` would report the previous block's state against this block's window -- the exact
/// off-by-one #137's E1 caught on the command side, mirrored here (E3's red mutation).
///
/// The whole of level-2 zero is the `wants` call: an unarmed tap's effect state is never read,
/// never folded and never stored, and a plan with no capacity at all never reaches this function
/// because `observation` is `None`.
///
/// Issue #163 phase 4 item 6 adds the lane-level gate in front of that per-tap one. `wants` made
/// the *state read* free for an unarmed tap but still walked every declared tap of every driven
/// effect on every block, so a capable-but-unsubscribed plan paid O(taps) where #143 promises
/// "one predicted branch per driven effect per block". `any_armed` is that one branch, and it is
/// the branch its own doc comment has always described itself as. The per-tap `wants` stays: it
/// is what keeps an armed lane from reading an unarmed sibling tap, and the two gates are a
/// conjunction, never a replacement.
fn publish_observations(
    observation: &mut ObservationLane,
    processor: &dyn PreparedNativeEffect,
    first_sample: u64,
    frames: u64,
) {
    if !observation.any_armed() {
        return;
    }
    let mut sample = ObservationSample {
        left: 0.0,
        right: 0.0,
    };
    for tap in 0..observation.len() {
        if !observation.wants(tap) {
            continue;
        }
        let index = tap as u32;
        if processor.observe_resident(index, &mut sample) {
            observation.accumulate(tap, sample, first_sample, frames);
        }
    }
}
// REALTIME_POLICY_END

/// One delayed input, staged into a scratch buffer on the way in.
#[derive(Clone, Copy)]
pub(crate) struct StagedInput {
    /// Buffer the producer wrote.
    pub(crate) source: u32,
    /// Scratch buffer the delayed copy lands in; this is what the op reads.
    pub(crate) staging: u32,
    /// Index into [`Runtime::delays`].
    pub(crate) line: u32,
}

/// One executable step: reduce the inputs, do the node's work, then let the observers look.
pub(crate) struct RuntimeOp {
    /// Buffers to reduce, in stable edge-ID order (D9). These are the *effective* reads: a
    /// delayed input names its staging buffer, not the producer's.
    pub(crate) inputs: Box<[u32]>,
    /// Delayed inputs this op stages itself. Empty in the native executor, whose coordinator
    /// stages every edge between partitions.
    pub(crate) staged: Box<[StagedInput]>,
    pub(crate) sidechain: Option<u32>,
    pub(crate) output: u32,
    pub(crate) kind: NodeKind,
    pub(crate) split_pair: Option<SplitPairSlot>,
    /// This node's observers, by handle, followed by the observers of every alias that resolves
    /// to this op's output buffer, in schedule order (`program::Tap`).
    pub(crate) observers: Box<[GraphNodeObserverBinding]>,
}

/// Old-layout witness for one executable op before the split-pair slot was added. The graph
/// estimate derives the inline delta from this mirror and charges the larger op/unit delta once
/// for the bounded emitted-op population.
#[allow(dead_code)]
pub(crate) struct RuntimeOpWithoutSplitPairSlot {
    pub(crate) inputs: Box<[u32]>,
    pub(crate) staged: Box<[StagedInput]>,
    pub(crate) sidechain: Option<u32>,
    pub(crate) output: u32,
    pub(crate) kind: NodeKind,
    pub(crate) observers: Box<[GraphNodeObserverBinding]>,
}

/// One scheduling unit: a single op, or a whole homogeneous bank.
pub(crate) enum RuntimeUnit {
    Op(RuntimeOp),
    Bank {
        /// Slot major, `lanes` ops per slot. A single-slot chain has exactly `lanes` of them.
        ///
        /// Only the first slot's ops are *executed*: they reduce their graph inputs into their
        /// output buffers, which is what the chain then gathers. Every later slot's audio is
        /// computed by the chain itself, so running its op would overwrite the scatter with a
        /// copy of the chain's input. Later slots are still carried here because they still own
        /// their observers and their output buffers.
        members: Box<[RuntimeOp]>,
        /// Lanes per slot; `members.len()` is a whole multiple of it.
        lanes: usize,
        chain: BankChain,
        /// One entry per lane when this chain's epilogue folds its routes into the master bus,
        /// empty otherwise (issue #218). Decided once, at bind, by [`route_fold`].
        fold: Box<[FoldLane]>,
        /// Where a folded lane accumulates the master. Meaningless when `fold` is empty.
        master: FoldTarget,
    },
}

/// Where a folded chain's epilogue accumulates the master, decided once at bind (issue #916).
///
/// It is decided by the master **op**: [`route_fold`] names it, and `validate_fold_installation`
/// compares it with the session Output node's op. It is never decided by comparing the master's
/// buffer with the session output's, which a bus master can share (see [`HostMaster`]).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FoldTarget {
    /// The master op's arena buffer: the master is a bus, not the session output. It is also the
    /// inert placeholder of a chain that does not fold.
    Arena(u32),
    /// The master op is the session Output op, so the master is the host's planes.
    Output,
}

#[allow(dead_code)]
pub(crate) enum RuntimeUnitWithoutSplitPairSlot {
    Op(RuntimeOpWithoutSplitPairSlot),
    Bank {
        members: Box<[RuntimeOpWithoutSplitPairSlot]>,
        lanes: usize,
        chain: BankChain,
        fold: Box<[FoldLane]>,
        master: FoldTarget,
    },
}

fn response_snapshot_error(error: ResponseAnalysisError) -> ResponseSnapshotError {
    match error {
        ResponseAnalysisError::UnsupportedMode | ResponseAnalysisError::UnsupportedCapability => {
            ResponseSnapshotError::Unsupported
        }
        ResponseAnalysisError::Capacity | ResponseAnalysisError::ResourceLimit => {
            ResponseSnapshotError::Capacity
        }
        ResponseAnalysisError::InvalidFrequencyGrid | ResponseAnalysisError::OutputShape => {
            ResponseSnapshotError::InvalidShape
        }
        ResponseAnalysisError::Configuration(_) | ResponseAnalysisError::Numerical => {
            ResponseSnapshotError::Owner
        }
    }
}

fn response_snapshot_kind_code(kind: ResponseSnapshotKind) -> u32 {
    match kind {
        ResponseSnapshotKind::ParametricEq => 1,
        ResponseSnapshotKind::BuiltinInputFilters => 2,
    }
}

fn emit_response_snapshot_owner(
    binding: &ResponseOwnerBinding,
    sample_rate_hz: u32,
    summary: ResponseSnapshotSummary,
    left: &[ResponseSnapshotSection; effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS],
    right: &[ResponseSnapshotSection; effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS],
    sink: &mut dyn ResponseSnapshotSink,
) -> Result<(), ResponseSnapshotError> {
    if summary.sample_rate_hz != sample_rate_hz {
        return Err(ResponseSnapshotError::Owner);
    }
    let sections =
        usize::try_from(summary.sections).map_err(|_| ResponseSnapshotError::Capacity)?;
    if sections == 0 || sections > left.len() || sections > right.len() {
        return Err(ResponseSnapshotError::InvalidShape);
    }
    for index in 0..sections {
        let source_left = left[index];
        let source_right = right[index];
        if usize::from(source_left.word_count) > source_left.words.len()
            || usize::from(source_right.word_count) > source_right.words.len()
        {
            return Err(ResponseSnapshotError::InvalidShape);
        }
    }
    sink.copy_owner(
        ResponseSnapshotOwnerInfo {
            track_id: &binding.track_id,
            native_id: &binding.native_id,
            stable_id: &binding.stable_id,
            rack: binding.rack,
            slot: binding.slot,
            kind: response_snapshot_kind_code(summary.kind),
            bypassed: summary.bypassed,
            availability: ResponseSnapshotAvailability::Provided,
        },
        &left[..sections],
        &right[..sections],
    )
}

fn emit_unavailable_response_snapshot_owner(
    binding: &ResponseOwnerBinding,
    bypassed: bool,
    sink: &mut dyn ResponseSnapshotSink,
) -> Result<(), ResponseSnapshotError> {
    sink.copy_owner(
        ResponseSnapshotOwnerInfo {
            track_id: &binding.track_id,
            native_id: &binding.native_id,
            stable_id: &binding.stable_id,
            rack: binding.rack,
            slot: binding.slot,
            kind: 0,
            bypassed,
            availability: ResponseSnapshotAvailability::DeclaredUnavailable,
        },
        &[],
        &[],
    )
}

fn copy_scalar_response_snapshot(
    op: &RuntimeOp,
    binding: &ResponseOwnerBinding,
    sample_rate_hz: u32,
    sink: &mut dyn ResponseSnapshotSink,
) -> Result<(), ResponseSnapshotError> {
    let mut left = [ResponseSnapshotSection {
        id: 0,
        kind: 0,
        enabled: false,
        word_count: 0,
        words: [0; effect_contract::RESPONSE_SNAPSHOT_WORDS],
    }; effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS];
    let mut right = left;
    let section_capacity = if binding.rack == 0 {
        2
    } else {
        effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS
    };
    let (summary, bypassed) = match &op.kind {
        NodeKind::Bound(processor) => match processor.copy_response_snapshot(
            sample_rate_hz,
            OwnerSnapshotRequest {
                bypassed: false,
                left: &mut left[..section_capacity],
                right: &mut right[..section_capacity],
            },
        ) {
            Ok(summary) => (summary, false),
            Err(ResponseAnalysisError::UnsupportedMode)
            | Err(ResponseAnalysisError::UnsupportedCapability) => {
                if binding.response_snapshot_declared {
                    return Err(ResponseSnapshotError::Unsupported);
                }
                return emit_unavailable_response_snapshot_owner(binding, false, sink);
            }
            Err(error) => return Err(response_snapshot_error(error)),
        },
        NodeKind::Effect(effect) => {
            let bypassed = effect.metadata.bypass;
            match effect
                .processor
                .copy_response_snapshot(OwnerSnapshotRequest {
                    bypassed,
                    left: &mut left[..section_capacity],
                    right: &mut right[..section_capacity],
                }) {
                Ok(summary) => (summary, bypassed),
                Err(ResponseAnalysisError::UnsupportedMode)
                | Err(ResponseAnalysisError::UnsupportedCapability) => {
                    if binding.response_snapshot_declared {
                        return Err(ResponseSnapshotError::Unsupported);
                    }
                    return emit_unavailable_response_snapshot_owner(binding, bypassed, sink);
                }
                Err(error) => return Err(response_snapshot_error(error)),
            }
        }
        NodeKind::ConsoleEffect(console) => {
            let bypassed = console.control.bypassed();
            match console
                .effect
                .processor
                .copy_response_snapshot(OwnerSnapshotRequest {
                    bypassed,
                    left: &mut left[..section_capacity],
                    right: &mut right[..section_capacity],
                }) {
                Ok(summary) => (summary, bypassed),
                Err(ResponseAnalysisError::UnsupportedMode)
                | Err(ResponseAnalysisError::UnsupportedCapability) => {
                    if binding.response_snapshot_declared {
                        return Err(ResponseSnapshotError::Unsupported);
                    }
                    return emit_unavailable_response_snapshot_owner(binding, bypassed, sink);
                }
                Err(error) => return Err(response_snapshot_error(error)),
            }
        }
        _ => return emit_unavailable_response_snapshot_owner(binding, false, sink),
    };
    if summary.bypassed != bypassed {
        return Err(ResponseSnapshotError::Owner);
    }
    emit_response_snapshot_owner(binding, sample_rate_hz, summary, &left, &right, sink)
}

impl RuntimeUnit {
    fn copy_response_snapshot(
        &self,
        binding: &ResponseOwnerBinding,
        sample_rate_hz: u32,
        sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<(), ResponseSnapshotError> {
        match self {
            Self::Op(op) => copy_scalar_response_snapshot(op, binding, sample_rate_hz, sink),
            Self::Bank {
                members,
                lanes,
                chain,
                ..
            } => {
                if *lanes == 0 || binding.member >= members.len() {
                    return Err(ResponseSnapshotError::Owner);
                }
                let slot = binding.member / *lanes;
                let lane = binding.member % *lanes;
                let mut left = [ResponseSnapshotSection {
                    id: 0,
                    kind: 0,
                    enabled: false,
                    word_count: 0,
                    words: [0; effect_contract::RESPONSE_SNAPSHOT_WORDS],
                };
                    effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS];
                let mut right = left;
                let bypassed = chain.response_snapshot_bypassed(slot, lane);
                let section_capacity = if binding.rack == 0 {
                    2
                } else {
                    effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS
                };
                let summary = match chain.copy_response_snapshot_lane(
                    slot,
                    lane,
                    sample_rate_hz,
                    OwnerSnapshotRequest {
                        bypassed,
                        left: &mut left[..section_capacity],
                        right: &mut right[..section_capacity],
                    },
                ) {
                    Ok(summary) => summary,
                    Err(ResponseAnalysisError::UnsupportedMode)
                    | Err(ResponseAnalysisError::UnsupportedCapability) => {
                        if binding.response_snapshot_declared {
                            return Err(ResponseSnapshotError::Unsupported);
                        }
                        return emit_unavailable_response_snapshot_owner(binding, bypassed, sink);
                    }
                    Err(error) => return Err(response_snapshot_error(error)),
                };
                if summary.bypassed != bypassed {
                    return Err(ResponseSnapshotError::Owner);
                }
                emit_response_snapshot_owner(binding, sample_rate_hz, summary, &left, &right, sink)
            }
        }
    }
}

pub(crate) fn scalar_split_op_layout() -> (u64, u64) {
    (
        u64::try_from(
            core::mem::size_of::<RuntimeOp>()
                .checked_sub(core::mem::size_of::<RuntimeOpWithoutSplitPairSlot>())
                .expect("split op slot layout is retained"),
        )
        .expect("split op slot layout fits u64"),
        u64::try_from(
            core::mem::size_of::<RuntimeUnit>()
                .checked_sub(core::mem::size_of::<RuntimeUnitWithoutSplitPairSlot>())
                .expect("split op containing-unit layout is retained"),
        )
        .expect("split op containing-unit layout fits u64"),
    )
}

impl RuntimeUnit {
    /// Whether `observe_unit` would dispatch any observer for this unit (issue #900): the same
    /// op observer slices its walk visits. Bind-time only.
    fn has_observers(&self) -> bool {
        match self {
            Self::Op(op) => !op.observers.is_empty(),
            Self::Bank { members, .. } => members.iter().any(|member| !member.observers.is_empty()),
        }
    }

    pub(crate) fn qualification_counters(&self) -> [u64; 2] {
        match self {
            Self::Op(_) => [0, 0],
            Self::Bank { chain, .. } => chain.qualification_counters(),
        }
    }

    pub(crate) fn transposes(&self) -> u64 {
        match self {
            Self::Op(_) => 0,
            Self::Bank { chain, .. } => chain.transposes(),
        }
    }

    /// `[blocks rendered collapsed, cohorts that can collapse at all]` for this unit.
    ///
    /// A per-block count and a bind-time one, together, because either alone is unreadable: the
    /// block count says the collapse fired but not out of how many chains, and the cohort count
    /// says a chain *could* collapse but not that any block did. A single op contributes neither
    /// -- the per-instance path declines in this milestone.
    pub(crate) fn collapse_counters(&self) -> [u64; 2] {
        match self {
            Self::Op(_) => [0, 0],
            Self::Bank { chain, .. } => [chain.collapses(), u64::from(chain.can_collapse())],
        }
    }

    /// `[disengages, re-engages, agreement proofs]` for this unit (mono-collapse M3).
    ///
    /// The cycle's three edges. `collapse_counters` counts blocks, and a block count cannot tell a
    /// chain that collapsed throughout from one that collapsed, stopped and started again -- which
    /// is the transition M3 is about. See `BankChain::collapse_transitions`.
    pub(crate) fn collapse_transitions(&self) -> [u64; 3] {
        match self {
            Self::Op(_) => [0; 3],
            Self::Bank { chain, .. } => chain.collapse_transitions(),
        }
    }

    /// Force this unit's collapse off (or back on). Bind-time; see `BankChain::force_mono_collapse_off`.
    pub(crate) fn force_mono_collapse_off(&mut self, forced: bool) {
        if let Self::Bank { chain, .. } = self {
            chain.force_mono_collapse_off(forced);
        }
    }

    /// `[bank chains, bound bank slots]` this unit realises (issue #181).
    ///
    /// The two were the same number for every plan before a cohort chain could carry more than
    /// one slot, which is precisely why G5 could not tell "one round-trip per chain" from "one
    /// per slot". Reporting both is what lets the gate state the law it means.
    pub(crate) fn bank_shape(&self) -> [u64; 2] {
        match self {
            Self::Op(_) => [0, 0],
            Self::Bank { members, lanes, .. } => [1, (members.len() / lanes) as u64],
        }
    }

    /// Exact engine-owned observation bytes this unit retains (issue #143 R7).
    pub(crate) fn observation_retained_bytes(&self) -> usize {
        match self {
            Self::Op(op) => op.kind.observation_retained_bytes(),
            Self::Bank { members, chain, .. } => {
                members
                    .iter()
                    .map(|member| member.kind.observation_retained_bytes())
                    .sum::<usize>()
                    + chain.observation_retained_bytes()
            }
        }
    }

    /// `[collapse-eligible lanes, lanes] ` this unit realises. Evidence and gates only.
    ///
    /// A bank counts its active lanes; a single op counts one "lane", its own node. Nothing on
    /// the render path calls this.
    pub(crate) fn symmetry_counters(&self) -> [u64; 2] {
        match self {
            Self::Op(op) => [u64::from(op.kind.channel_symmetry().eligible()), 1],
            Self::Bank { chain, .. } => chain.symmetry_counters(),
        }
    }

    /// One flag per active lane, in lane order (mono-collapse M1).
    ///
    /// The localisable form of [`symmetry_counters`](Self::symmetry_counters); folding this gives
    /// that, which is what keeps the census and the per-unit rows from ever disagreeing.
    pub(crate) fn lane_eligibility(&self) -> Vec<bool> {
        match self {
            Self::Op(op) => vec![op.kind.channel_symmetry().eligible()],
            Self::Bank { chain, .. } => chain.active_lane_eligibility(),
        }
    }

    /// `[observed stages, declared taps, armed taps]` for this unit (issue #143 E5).
    pub(crate) fn observation_binding_counts(&self) -> [u64; 3] {
        match self {
            Self::Op(op) => op.kind.observation_binding_counts(),
            Self::Bank { members, chain, .. } => {
                let mut total = chain.observation_binding_counts();
                for member in members.iter() {
                    let counts = member.kind.observation_binding_counts();
                    for (slot, value) in total.iter_mut().zip(counts) {
                        *slot = slot.saturating_add(value);
                    }
                }
                total
            }
        }
    }
}

impl NodeKind {
    /// This op's channel-symmetry witness.
    ///
    /// A bank member says nothing: its work is the chain's, and the chain's slots carry the
    /// witness for it. An identity, a source input and a route are not upstream-of-seam track
    /// work at all. Everything else answers for itself, and the two effect variants answer with
    /// the effect's own designed-word comparison -- plus, for a console-driven one, the live terms
    /// its drain maintains.
    pub(crate) fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        let designed = |symmetric: bool| {
            if symmetric {
                ChannelSymmetryWitness::SYMMETRIC
            } else {
                ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::DESIGNED)
            }
        };
        match self {
            // The one non-bank stage that can be asymmetric upstream of the seam: two lanes with
            // different declared delays turn one source channel into two different signals, so the
            // track is not collapsible. The same verdict is taken at prepare, from the session, by
            // `session_structural_symmetry` -- which is what actually arms the chain; this arm
            // is the plan's own evidence row agreeing with it.
            Self::TrackDelay { channels_agree, .. } => designed(*channels_agree),
            Self::Effect(effect) => designed(effect.processor.channel_symmetry()),
            Self::ConsoleEffect(console) => designed(console.effect.processor.channel_symmetry())
                .and(console.control.symmetry()),
            Self::Bound(processor) => processor.channel_symmetry(),
            // Not a per-track upstream stage: nothing here can make the two channels disagree,
            // and nothing here is collapsed.
            Self::Identity | Self::SourceInput | Self::Route(_) | Self::BankMember => {
                ChannelSymmetryWitness::SYMMETRIC
            }
        }
    }

    /// `[observed stages, declared taps, armed taps]` for one op.
    pub(crate) fn observation_binding_counts(&self) -> [u64; 3] {
        let Self::ConsoleEffect(console) = self else {
            return [0, 0, 0];
        };
        let Some(observation) = console.observation.as_deref() else {
            return [0, 0, 0];
        };
        let armed = (0..observation.len())
            .filter(|tap| observation.is_armed(*tap))
            .count() as u64;
        [1, observation.len() as u64, armed]
    }

    /// Exact engine-owned observation bytes this op retains. Zero for an unobserved op.
    pub(crate) fn observation_retained_bytes(&self) -> usize {
        match self {
            Self::ConsoleEffect(console) => console
                .observation
                .as_deref()
                .map_or(0, ObservationLane::retained_bytes),
            _ => 0,
        }
    }
}

// REALTIME_POLICY_BEGIN
/// Planar per-lane view over the arena slots a bank chain gathers from and scatters to.
///
/// The two lists differ only for a multi-slot cohort chain (issue #181): the chain gathers the
/// first slot's member outputs and scatters the *last* slot's, because the last slot's buffer is
/// what the rest of the graph reads. A single-slot chain passes the same list twice, which is the
/// in-place round-trip it has always done.
struct ArenaMembers<'a> {
    lease: &'a mut ArenaLease,
    inputs: &'a [u32],
    outputs: &'a [u32],
    /// One entry per lane when this chain's epilogue folds its routes, empty otherwise.
    fold: &'a [FoldLane],
    /// Where a folded lane's routed tile lands. Meaningless when `fold` is empty.
    master: MasterPlanes<'a>,
    /// The played source planes a gather reads in place of an input buffer (issue #918).
    sources: SourceGather<'a>,
}

/// No source claim is read in place at this arena buffer: the `u32` [`Runtime`]'s
/// `source_plane_of_buffer` holds everywhere but at a claim bound in place.
pub(crate) const NO_SOURCE_CLAIM: u32 = u32::MAX;

/// What a bank's gather needs to read a source claim's played block in place (issue #918): the
/// source set's planes for this block and the bind-time table of the buffers they replace.
#[derive(Clone, Copy)]
struct SourceGather<'a> {
    /// `None` in a plan with no source set.
    planes: Option<&'a dyn crate::GraphSourcePlanes>,
    /// [`Runtime`]'s `source_plane_of_buffer`.
    of_buffer: &'a [u32],
    /// This unit's [`UnitIdentity::source_lanes`]: the only lanes that consult `of_buffer`.
    lanes: u8,
}

impl SourceGather<'_> {
    /// No source set: every gather reads the arena. What a hand-built `ArenaMembers` carries.
    #[cfg(test)]
    const NONE: Self = Self {
        planes: None,
        of_buffer: &[],
        lanes: 0,
    };

    /// The claim lane `lane` gathers in place from `buffer`, if it is a marked lane.
    #[inline]
    fn claim(&self, lane: usize, buffer: u32) -> Option<usize> {
        if lane >= 8 || self.lanes & (1 << lane) == 0 {
            return None;
        }
        let claim = *self.of_buffer.get(buffer as usize)?;
        (claim != NO_SOURCE_CLAIM).then_some(claim as usize)
    }
}

/// A folded chain's master for one block: the runtime form of [`FoldTarget`] (issue #916).
///
/// All three epilogues -- [`BankMembers::fold_plane`], [`BankMembers::fold_cohort`] and
/// [`BankMembers::fold_resident`] -- reach the master only through
/// [`ArenaMembers::master_planes`] and check it only through [`ArenaMembers::master_writable`].
/// Each writes the same words with the same kernel whichever variant it gets.
enum MasterPlanes<'a> {
    /// An arena buffer, written through the lease.
    Arena(u32),
    /// The host's planes, which are the session Output op's storage for this block.
    Host(HostMaster<'a>),
}

/// One folded lane's epilogue: the route's bind-folded 2x2, and how its tile meets the master.
///
/// `store` is the D9 association restated for a scatter-accumulate: the reduction this replaces is
/// `sum2_block(in0, in1)` then `sum_into_block` left to right, so the **first** contributor writes
/// the master and every later one adds into it. Writing then adding computes `in0 + in1` with the
/// same operation and the same rounding `sum2_block` does. Zero-filling first and accumulating
/// every contributor would not: `0.0 + (-0.0)` is `+0.0` where `in0 + in1` on a `-0.0` first
/// contributor is `-0.0`, and a fan-in-one master would lose its sign outright.
#[derive(Clone, Copy)]
pub(crate) struct FoldLane {
    /// The route's 2x2 with its linear gain already folded in (D3), exactly as `NodeKind::Route`
    /// carries it: the epilogue applies the same constants through the same `mix2x2_block`.
    coefficients: [f32; 4],
    /// This lane is the master's first contributor, so it stores rather than accumulates.
    store: bool,
}

impl BankMembers for ArenaMembers<'_> {
    /// A lane's gather source: the arena buffer `inputs[lane]`, unless the lane is marked as
    /// gathering a source claim bound in place (issue #918), whose planes are then the played
    /// block's.
    ///
    /// A claim is bound in place only when its input's every reader is such a gather
    /// ([`source_plane_table`]), so the words are the ones the executor's copy loop would have
    /// written into the buffer: the played block's on `Some`, and on `None` (underrun, end of
    /// region) the arena's silence buffer, which is the `+0.0` the copy fills an unplayed quantum
    /// with. One mask test per lane per block, and one table lookup for a marked lane; nothing is
    /// copied.
    fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
        #[cfg(any(test, feature = "test-support"))]
        TEST_ONLY_RESIDENT_COUNTS.with(|count| {
            let [gathers, residents] = count.get();
            count.set([gathers + 1, residents]);
        });
        let buffer = self.inputs[lane];
        if let Some(planes) = self.sources.planes
            && let Some(claim) = self.sources.claim(lane, buffer)
        {
            return match planes.played_planes(claim) {
                Some(played) => {
                    #[cfg(any(test, feature = "test-support"))]
                    test_only_count_source_plane(1);
                    played
                }
                None => {
                    #[cfg(any(test, feature = "test-support"))]
                    test_only_count_source_plane(2);
                    self.lease.read_stereo(ARENA_SILENCE_BUFFER)
                }
            };
        }
        self.lease.read_stereo(buffer)
    }
    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
        self.lease.write_stereo(self.outputs[lane])
    }
    fn distinct_planes_mut(&mut self, lanes: usize, frames: usize) -> Option<BankPlaneViews<'_>> {
        match lanes {
            4 => {
                let buffers: [u32; 4] = self.outputs.get(..4)?.try_into().ok()?;
                Some(BankPlaneViews::from_four(
                    self.lease.write_stereo_many(&buffers, frames)?,
                    frames,
                )?)
            }
            8 => {
                let buffers: [u32; 8] = self.outputs.get(..8)?.try_into().ok()?;
                Some(BankPlaneViews::from_eight(
                    self.lease.write_stereo_many(&buffers, frames)?,
                    frames,
                )?)
            }
            _ => None,
        }
    }
    /// The route and the master accumulation, in the lane's own transposed tile (issue #218).
    ///
    /// Three frozen facts make this the reduction it replaces rather than a re-derivation of it:
    ///
    /// * the tile is `frames` words long, exactly as `lease.write_stereo` hands the route op its
    ///   buffer, so `mix2x2_block::<FrameLane>` takes the same vector/tail split and emits the same
    ///   per-sample op order over the same constants;
    /// * the route stays its own arithmetic step. It is **not** merged into the matrix slot above
    ///   it: two 2x2s multiplied out is a different rounding, and D3 folds the *gain* into the
    ///   route's own coefficients and nothing else;
    /// * the master meets `sum_into_block::<FrameLane>`, the same kernel `reduce_plane`'s
    ///   left-to-right accumulation used, in the same order -- which `route_fold` proves rather
    ///   than assumes.
    ///
    /// The accumulation is two independent per-plane calls, so a mono collapse drops one of them
    /// and changes nothing else. The 2x2 above it is irreducibly cross-plane -- that is what a
    /// route *is* (D3) -- and is not something a plane-wise factoring could have separated.
    fn fold_plane(&mut self, lane: usize, left: &mut [f32], right: &mut [f32]) {
        let fold = self.fold[lane];
        mix2x2_block::<FrameLane>(left, right, fold.coefficients);
        let (master_left, master_right) = self.master_planes();
        if fold.store {
            master_left.copy_from_slice(left);
            master_right.copy_from_slice(right);
        } else {
            sum_into_block::<FrameLane>(master_left, left);
            sum_into_block::<FrameLane>(master_right, right);
        }
    }

    fn fold_cohort(&mut self, cohort: FoldCohort<'_>) {
        let lane_ids = cohort.lane_ids();
        let count = lane_ids.len();
        let frames = cohort.frames();
        let stride = cohort.stride();
        let Some(max_lane) = lane_ids.iter().copied().max() else {
            return;
        };
        if lane_ids
            .iter()
            .enumerate()
            .any(|(index, lane)| lane_ids[..index].contains(lane))
        {
            return;
        }
        let Some(required) = max_lane
            .checked_add(1)
            .and_then(|lanes| lanes.checked_mul(stride))
        else {
            return;
        };
        if count == 0
            || count > 8
            || stride < frames
            || cohort.left().len() < required
            || cohort.right().len() < required
            || frames > self.lease.frames()
            || !self.master_writable()
        {
            return;
        }
        let mut coefficients = [[0.0; 4]; 8];
        let mut stores = [false; 8];
        let mut ids = [0usize; 8];
        for (index, &lane) in lane_ids.iter().enumerate() {
            ids[index] = lane;
            let Some(fold) = self.fold.get(lane).copied() else {
                return;
            };
            if index != 0 && fold.store {
                return;
            }
            coefficients[index] = fold.coefficients;
            stores[index] = fold.store;
        }
        let mut left = cohort;
        for (index, coefficient) in coefficients[..count].iter().enumerate() {
            let Some((left_plane, right_plane)) = left.planes_mut(ids[index]) else {
                return;
            };
            mix2x2_block::<FrameLane>(left_plane, right_plane, *coefficient);
        }
        let mut left_inputs: [&[f32]; 8] = [&[]; 8];
        let mut right_inputs: [&[f32]; 8] = [&[]; 8];
        for index in 0..count {
            let start = ids[index] * stride;
            left_inputs[index] = &left.left()[start..start + frames];
            right_inputs[index] = &left.right()[start..start + frames];
        }
        let (master_left, master_right) = self.master_planes();
        let initial_store = stores[0];
        let valid_left = ordered_accumulate_block::<FrameLane>(
            &mut master_left[..frames],
            &left_inputs[..count],
            initial_store,
        );
        let valid_right = ordered_accumulate_block::<FrameLane>(
            &mut master_right[..frames],
            &right_inputs[..count],
            initial_store,
        );
        debug_assert!(valid_left && valid_right);
    }

    /// The whole bank's routes and master accumulation, straight from the resident block
    /// (issue #915).
    ///
    /// This is [`Self::fold_cohort`] over the staging block the scatter would have written, fused
    /// into the transpose that would have written it: each `W`-frame tile of both planes is
    /// transposed into its `W` lane rows, every lane's row is routed, and the routed rows are
    /// accumulated into the master's `W` words for that tile. The staging store, the
    /// `mix2x2_block` pass over it and the `ordered_accumulate_block` pass over it become one
    /// pass, and the staging block is never written.
    ///
    /// # Why this is `fold_cohort`'s arithmetic and not a re-derivation of it
    ///
    /// Both kernels it replaces treat frames independently: `mix2x2_block` computes a frame's two
    /// outputs from that frame's own `l`, `r` and the lane's constants, and
    /// `ordered_accumulate_block` computes a master word from that frame's contributors alone. So
    /// each master word is one fixed expression over its frame's words, and the vector width a
    /// pass happens to run at -- [`FrameLane`] over a `frames`-long staging plane there, `W` frames
    /// of a tile or one tail frame here -- does not enter it: the lane contract is IEEE per element
    /// on every backend, and `Lane::fma` is the unfused `(a * b) + c` on every backend. Per frame
    /// and per plane, that expression is:
    ///
    /// * each lane's routed word, `mix2x2_block`'s verbatim ([`route_word`]):
    ///   `l' = lr.fma(r, ll.mul(l))` and `r' = rr.fma(r, rl.mul(l))`, both from the frame's
    ///   original `l` and `r`;
    /// * the lanes in ascending order, `ordered_accumulate_block`'s verbatim ([`fold_words`]):
    ///   lane 0's routed word **is** the running value when lane 0 stores, and is added to the
    ///   live master word (master on the left) when it does not; every later lane is added to the
    ///   running value, running value on the left; the result is stored once.
    ///
    /// The staged path carries each routed word through a staging store and a reload, and a store
    /// and a load move an `f32`'s bits unchanged, so keeping it in a register moves no bit either.
    ///
    /// Every premise is checked before the first write. A failed one declines having written
    /// nothing, and the chain then takes the staged path -- whose `fold_cohort` applies its own
    /// checks exactly as it always has.
    fn fold_resident(&mut self, cohort: ResidentFoldCohort<'_>) -> bool {
        match cohort.width() {
            BankWidth::Four => self.fold_resident_tiles::<4, lane::Simd4>(cohort, transpose_tile_4),
            BankWidth::Eight => {
                self.fold_resident_tiles::<8, lane::Simd8>(cohort, transpose_tile_8)
            }
        }
    }
}

impl ArenaMembers<'_> {
    /// Whether this block's master may be written: an arena master only when it is in the lease's
    /// write set, the host's planes always. The premise check `lease.writes(master)` was, per
    /// variant.
    fn master_writable(&self) -> bool {
        match &self.master {
            MasterPlanes::Arena(buffer) => self.lease.writes(*buffer),
            MasterPlanes::Host(_) => true,
        }
    }

    /// Both master planes, exclusively: `lease.write_stereo(master)` for an arena master, the
    /// host's planes for the Output. Both are exactly `lease.frames()` words.
    fn master_planes(&mut self) -> (&mut [f32], &mut [f32]) {
        match &mut self.master {
            MasterPlanes::Arena(buffer) => self.lease.write_stereo(*buffer),
            MasterPlanes::Host(host) => host.planes_mut(),
        }
    }

    /// [`BankMembers::fold_resident`] at one bank width: `W` lanes, and `L` a `W`-wide lane, so one
    /// transposed lane row is one `L` value.
    #[inline(always)]
    fn fold_resident_tiles<const W: usize, L: Lane>(
        &mut self,
        cohort: ResidentFoldCohort<'_>,
        transpose: impl Fn([[f32; W]; W]) -> [[f32; W]; W],
    ) -> bool {
        let frames = cohort.frames();
        let (resident_left, resident_right) = (cohort.left(), cohort.right());
        // Every premise, before the first write. `W` is the lane count four ways -- the cohort's,
        // the tile's, the row lane type's and this chain's fold list's -- and the last is also
        // "every lane folds": a folded chain is given one `FoldLane` per lane and an unfolded one
        // none. Only lane 0 may store, which is the D9 association `route_fold` proved.
        let Some(words) = frames.checked_mul(W) else {
            return false;
        };
        if W > 8
            || L::WIDTH != W
            || cohort.lanes() != W
            || self.fold.len() != W
            || frames == 0
            || frames > self.lease.frames()
            || resident_left.len() != words
            || resident_right.len() != words
            || !self.master_writable()
            || self.fold[1..].iter().any(|lane| lane.store)
        {
            return false;
        }
        // Hoisted once per cohort: every lane's constants, as words for the ragged tail and as
        // splats for the tiles, and the one store flag that decides how the master starts.
        let initial_store = self.fold[0].store;
        let constants: [[f32; 4]; W] = core::array::from_fn(|lane| self.fold[lane].coefficients);
        let splats: [[L; 4]; W] = core::array::from_fn(|lane| constants[lane].map(L::splat));
        let (master_left, master_right) = self.master_planes();
        let tiled = frames - frames % W;
        for (tile, (block_left, block_right)) in resident_left[..tiled * W]
            .chunks_exact(W * W)
            .zip(resident_right[..tiled * W].chunks_exact(W * W))
            .enumerate()
        {
            let base = tile * W;
            // The same tile transpose the scatter runs: row `lane` holds that lane's `W` frames.
            let rows_left = transpose(tile_rows(block_left));
            let rows_right = transpose(tile_rows(block_right));
            fold_words::<L, W>(
                &splats,
                initial_store,
                &mut master_left[base..base + W],
                &mut master_right[base..base + W],
                |lane| (L::load(&rows_left[lane]), L::load(&rows_right[lane])),
            );
        }
        // The ragged tail: the same expressions at `L = f32`, one frame at a time, read straight
        // from the resident words (frame `f`, lane `k` is word `f * W + k`).
        for frame in tiled..frames {
            let first = frame * W;
            fold_words::<f32, W>(
                &constants,
                initial_store,
                &mut master_left[frame..=frame],
                &mut master_right[frame..=frame],
                |lane| (resident_left[first + lane], resident_right[first + lane]),
            );
        }
        true
    }
}

/// One `W * W` block of AoSoA words as its `W` frame rows, the input of a tile transpose.
#[inline(always)]
fn tile_rows<const W: usize>(block: &[f32]) -> [[f32; W]; W] {
    let mut rows = [[0.0_f32; W]; W];
    for (row, chunk) in rows.iter_mut().zip(block.chunks_exact(W)) {
        row.copy_from_slice(chunk);
    }
    rows
}

/// One group of master words -- the `W` frames of a tile at a `W`-wide `L`, or one tail frame at
/// `L = f32` -- accumulated from every lane of a folded bank in ascending lane order.
///
/// `ordered_accumulate_block`'s association verbatim: the first contributor is the running value
/// when `initial_store`, otherwise the running value starts as the live master plus the first
/// contributor; each later contributor is added on the right; the sum is stored once. Each
/// contributor is [`route_word`] of that lane's words. `master_left` and `master_right` are exactly
/// `L::WIDTH` words.
#[inline(always)]
fn fold_words<L: Lane, const W: usize>(
    constants: &[[L; 4]; W],
    initial_store: bool,
    master_left: &mut [f32],
    master_right: &mut [f32],
    lane_words: impl Fn(usize) -> (L, L),
) {
    let (left, right) = lane_words(0);
    let (routed_left, routed_right) = route_word(constants[0], left, right);
    let (mut sum_left, mut sum_right) = if initial_store {
        (routed_left, routed_right)
    } else {
        (
            L::load(master_left).add(routed_left),
            L::load(master_right).add(routed_right),
        )
    };
    for (lane, constant) in constants.iter().enumerate().skip(1) {
        let (left, right) = lane_words(lane);
        let (routed_left, routed_right) = route_word(*constant, left, right);
        sum_left = sum_left.add(routed_left);
        sum_right = sum_right.add(routed_right);
    }
    sum_left.store(master_left);
    sum_right.store(master_right);
}

/// One lane's routed words: `mix2x2_block`'s frozen per-frame expressions, verbatim and in its
/// operand order, `[ll, lr, rl, rr]` already carrying the route's gain (D3).
#[inline(always)]
fn route_word<L: Lane>([ll, lr, rl, rr]: [L; 4], left: L, right: L) -> (L, L) {
    (lr.fma(right, ll.mul(left)), rr.fma(right, rl.mul(left)))
}

// REALTIME_POLICY_END

/// The static half of one unit's [`PlanUnitEligibility`] row, fixed at bind.
///
/// Split from the dynamic half because the two move at different times and for different reasons.
/// Which track a lane renders, how many stages a unit has and which side of the seam each stage
/// sits on are decided when the plan is built and cannot change afterwards; how many of the unit's
/// lanes are *eligible* moves whenever a live-console record is drained. So the identity is
/// computed once, here, from the node ids the lowering already resolved -- and the counters are
/// pulled from the chain on demand.
pub(crate) struct UnitIdentity {
    pub(crate) banked: bool,
    /// Proven from final adjacent emitted units at bind; fits the existing identity padding.
    resident_input: bool,
    /// Whether any op of this unit holds an observer binding (issue #900). Derived from the final
    /// units by [`Runtime::new_with_observation_activation`], the one constructor every runtime
    /// passes through, so a caller's value is a placeholder; nothing changes an op's observer
    /// slice after that. Fits the existing identity padding.
    observed: bool,
    /// Issue #918: bit `l` is set when first-slot lane `l` of this bank unit gathers a source
    /// claim bound in place, whose planes [`ArenaMembers::plane`] then reads from the played block.
    /// Set by [`source_plane_table`] at bind; zero for a plain unit. A bank has at most eight lanes,
    /// and the byte fits the existing identity padding.
    source_lanes: u8,
    pub(crate) stages: u32,
    pub(crate) upstream_of_seam_stages: u32,
    pub(crate) lane_tracks: Box<[Box<str>]>,
}

/// The compact bind-time relation from a declared response owner to its runtime unit and lane.
///
/// The owner list is built by walking the lowered program in declared signal order. Runtime unit
/// execution may hoist bank members, but this table retains the caller-visible order and only
/// stores the indices needed to reach the already-owned processor.
pub(crate) struct ResponseOwnerBinding {
    track_id: Box<str>,
    native_id: Box<str>,
    stable_id: Box<str>,
    response_snapshot_declared: bool,
    rack: u8,
    slot: u32,
    unit: usize,
    member: usize,
}

/// Which side of the fader/matrix seam one graph node's stage sits on.
///
/// The seam is `effect_contract::SeamSide`'s: the 2x2 matrix is the earliest
/// genuinely cross-channel operation in the strip and the fader is immediately before it, so
/// everything from `PostFader` on reads the plane a collapsed track duplicated and may legitimately
/// differ between the channels. It is read off `TrackStage` rather than off the processor, because
/// the *stage* is what decides it: a fader bank and an EQ bank are the same kind of object and only
/// their position in the strip separates them.
///
/// A node that is not per-track strip work at all -- a route, a submix, the output, a compensation
/// delay -- is **not** upstream: it is not a stage a collapse would have computed once, so counting
/// it as upstream would let a route op's unconditionally-symmetric witness read as collapse
/// evidence, which is precisely what the seam classification exists to prevent.
fn upstream_of_seam(node: &GraphNodeId) -> bool {
    match node {
        GraphNodeId::TrackStage { stage, .. } => !matches!(
            stage,
            crate::TrackStage::PostFader | crate::TrackStage::PostMatrix
        ),
        GraphNodeId::Effect(_) => true,
        GraphNodeId::Route { .. }
        | GraphNodeId::Submix { .. }
        | GraphNodeId::Output { .. }
        | GraphNodeId::CompensationDelay { .. } => false,
    }
}

/// The track one graph node renders, or `""` for a node that names none.
fn node_track(node: &GraphNodeId) -> Box<str> {
    match node {
        GraphNodeId::TrackStage { track_id, .. } => Box::from(track_id.as_str()),
        GraphNodeId::Effect(effect) => Box::from(effect.track_id.as_str()),
        GraphNodeId::Route { .. }
        | GraphNodeId::Submix { .. }
        | GraphNodeId::Output { .. }
        | GraphNodeId::CompensationDelay { .. } => Box::from(""),
    }
}

/// Ops, their audio and their delay lines: everything one executor (or one native parcel) owns.
pub(crate) struct Runtime {
    /// This runtime's checked view of the plan's shared arena.
    pub(crate) lease: ArenaLease,
    pub(crate) delays: Box<[CompensationDelay]>,
    /// Input-side track alignment lines, one per track that declared a nonzero delay on either
    /// lane. Empty on every session that declared none, which is what keeps an undelayed plan on
    /// the bytes and the program it had before this feature existed.
    pub(crate) track_delays: Box<[TrackDelayLine]>,
    pub(crate) units: Box<[RuntimeUnit]>,
    /// Optional host-controlled observer dispatch state. Its snapshot contains only validated
    /// copyable coordinates; observer resources remain in the immutable runtime units.
    observation_activation: Option<RealtimeObservationActivation>,
    /// Cursor through the sorted active snapshot for the current render block.
    observation_cursor: usize,
    /// Internal failure invalidation was already performed before the outer plan wrapper saw the
    /// error. This prevents the wrapper's compatibility callback from invalidating twice.
    observation_failure_invalidated: bool,
    split_pairs: Box<[Box<dyn GraphRuntimeSplitPairProcessor>]>,
    /// One row per unit, in `units` order: the bind-time half of the collapse-eligibility query.
    pub(crate) identity: Box<[UnitIdentity]>,
    response_bindings: Box<[ResponseOwnerBinding]>,
    /// Scratch for a bank chain's gather-source buffers, sized to the widest bank at bind.
    bank_inputs: Box<[u32]>,
    /// Scratch for a bank chain's scatter-target buffers, sized to the widest bank at bind.
    bank_outputs: Box<[u32]>,
    /// Lanes whose scatter this bind pointed at their consumer's buffer (issue #202 rec 3).
    redirects: u64,
    /// Lanes whose route and master accumulation this bind folded into their chain's epilogue
    /// (issue #218).
    folds: u64,
    /// The unit that runs the session Output op, whose storage for a block is the host's planes
    /// (issue #916, [`HostMaster`]). `build_sequential` always finds it: the Output node is never
    /// elided, banked or retired, so it is always a plain unit of its own. `None` only in a test's
    /// hand-built runtime, whose units then all write the arena.
    output_unit: Option<usize>,
    /// Issue #918: arena buffer -> the source claim a bank's gather reads **in place** from the
    /// played transfer block when it would read that buffer, or [`NO_SOURCE_CLAIM`]. Built at
    /// bind by [`source_plane_table`]; empty when no claim is bound in place. See there for the
    /// mode each claim gets and why the rest keep the copy. A claim the Output op reads in place
    /// (issue #927, `output_sources`) is named here too, which is what makes the copy loop skip
    /// it; the Output op itself never looks a buffer up here.
    source_plane_of_buffer: Box<[u32]>,
    /// One folded 2x2 per input of the Output op, in its edge order, when this bind retired every
    /// route that feeds it and fused them into its reduction ([`route_reduce`], issue #926).
    /// Empty otherwise, and then the Output op reduces as every other op does. Only the Output
    /// unit reads it: [`output_route_fold`] admits no other master.
    output_routes: Box<[[f32; 4]]>,
    /// Issue #927: beside `output_routes`, one entry per input of the Output op, in the same edge
    /// order: the source claim whose played block the fused reduction reads **in place** for that
    /// input, or [`NO_SOURCE_CLAIM`] for an input read from the arena. Empty when no input is read
    /// in place. Built at bind by [`source_plane_table`], clause (b'); only the Output unit reads
    /// it ([`OutputSources`]).
    output_sources: Box<[u32]>,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestOnlySplitPairTableWitness {
    pub allocations: u64,
    pub entries: u64,
    pub bytes: u64,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static TEST_ONLY_SPLIT_PAIR_TABLE: std::cell::Cell<TestOnlySplitPairTableWitness> =
        const { std::cell::Cell::new(TestOnlySplitPairTableWitness { allocations: 0, entries: 0, bytes: 0 }) };
}

#[cfg(any(test, feature = "test-support"))]
pub fn test_only_reset_split_pair_table_witness() {
    TEST_ONLY_SPLIT_PAIR_TABLE.with(|value| {
        value.set(TestOnlySplitPairTableWitness {
            allocations: 0,
            entries: 0,
            bytes: 0,
        })
    });
}

#[cfg(any(test, feature = "test-support"))]
#[must_use]
pub fn test_only_split_pair_table_witness() -> TestOnlySplitPairTableWitness {
    TEST_ONLY_SPLIT_PAIR_TABLE.with(std::cell::Cell::get)
}

/// Layout witness for the retained [`Runtime`] owner without its split-owner table field. The
/// runtime resource report charges the difference between this same-field layout and `Runtime`;
/// the split table itself remains a separate boxed allocation.
#[allow(dead_code)]
pub(crate) struct RuntimeWithoutSplitPairTable {
    lease: ArenaLease,
    delays: Box<[CompensationDelay]>,
    track_delays: Box<[TrackDelayLine]>,
    units: Box<[RuntimeUnit]>,
    observation_activation: Option<RealtimeObservationActivation>,
    observation_cursor: usize,
    observation_failure_invalidated: bool,
    identity: Box<[UnitIdentity]>,
    response_bindings: Box<[ResponseOwnerBinding]>,
    bank_inputs: Box<[u32]>,
    bank_outputs: Box<[u32]>,
    redirects: u64,
    folds: u64,
    output_unit: Option<usize>,
    source_plane_of_buffer: Box<[u32]>,
    output_routes: Box<[[f32; 4]]>,
    output_sources: Box<[u32]>,
}

/// Layout witness for the retained [`Runtime`] owner without observation activation state.
///
/// This mirror intentionally keeps every current runtime field except the optional activation
/// endpoint, its dispatch cursor, and its failure-invalidation flag. The containing executor
/// witness in `lib.rs` uses the corresponding owner-level delta for the retained accounting term;
/// this runtime-level witness remains available to independently prove the nested layout.
#[allow(dead_code)]
pub(crate) struct RuntimeWithoutObservationActivation {
    pub(crate) lease: ArenaLease,
    pub(crate) delays: Box<[CompensationDelay]>,
    pub(crate) track_delays: Box<[TrackDelayLine]>,
    pub(crate) units: Box<[RuntimeUnit]>,
    pub(crate) split_pairs: Box<[Box<dyn GraphRuntimeSplitPairProcessor>]>,
    pub(crate) identity: Box<[UnitIdentity]>,
    pub(crate) response_bindings: Box<[ResponseOwnerBinding]>,
    pub(crate) bank_inputs: Box<[u32]>,
    pub(crate) bank_outputs: Box<[u32]>,
    pub(crate) redirects: u64,
    pub(crate) folds: u64,
    pub(crate) output_unit: Option<usize>,
    pub(crate) source_plane_of_buffer: Box<[u32]>,
    pub(crate) output_routes: Box<[[f32; 4]]>,
    pub(crate) output_sources: Box<[u32]>,
}

pub(crate) fn observation_runtime_layout() -> Option<u64> {
    u64::try_from(
        core::mem::size_of::<Runtime>()
            .checked_sub(core::mem::size_of::<RuntimeWithoutObservationActivation>())?,
    )
    .ok()
}

pub(crate) fn scalar_split_runtime_layout() -> (u64, u64) {
    (
        u64::try_from(core::mem::size_of::<Box<dyn GraphRuntimeSplitPairProcessor>>())
            .expect("split table entry fits u64"),
        u64::try_from(
            core::mem::size_of::<Runtime>()
                .checked_sub(core::mem::size_of::<RuntimeWithoutSplitPairTable>())
                .expect("split runtime field layout is retained"),
        )
        .expect("split runtime field layout fits u64"),
    )
}

impl Runtime {
    /// Lanes whose scatter this bind pointed at their consumer's buffer (issue #202 rec 3).
    pub(crate) const fn scatter_redirects(&self) -> u64 {
        self.redirects
    }

    /// `[blocks rendered collapsed, cohorts that can collapse at all]`, over every unit.
    pub(crate) fn collapse_counters(&self) -> [u64; 2] {
        self.units.iter().fold([0, 0], |mut total, unit| {
            let counters = unit.collapse_counters();
            for (value, add) in total.iter_mut().zip(counters) {
                *value = value.saturating_add(add);
            }
            total
        })
    }

    /// `[disengages, re-engages, agreement proofs]`, over every unit.
    pub(crate) fn collapse_transitions(&self) -> [u64; 3] {
        self.units.iter().fold([0; 3], |mut total, unit| {
            let counters = unit.collapse_transitions();
            for (value, add) in total.iter_mut().zip(counters) {
                *value = value.saturating_add(add);
            }
            total
        })
    }

    /// Perform the collapse's structural join and arm every chain it admits.
    ///
    /// `eligible` answers, for one track id, whether that track's **structural** witness holds --
    /// the `SOURCE` term, which lives on the control plane and is keyed by track id. This is the
    /// only place in the engine where the two halves of the channel-symmetry witness meet: the
    /// runtime half is per lane and source agnostic, the structural half is per track, and
    /// `UnitIdentity::lane_tracks` is the relation between the two keys.
    ///
    /// All lanes or nothing, exactly as `BankChain::all_lanes_symmetric` is: a cohort with one
    /// two-source lane saves nothing by collapsing the others, because the vector op runs every
    /// lane regardless. Making a cohort homogeneous is the planner's job (`CohortPoolClass`).
    ///
    /// A unit whose lane list is empty arms nothing: a chain that names no track is one this join
    /// cannot speak for.
    pub(crate) fn arm_mono_collapse(&mut self, eligible: &dyn Fn(&str) -> bool) {
        for (unit, identity) in self.units.iter_mut().zip(self.identity.iter()) {
            let RuntimeUnit::Bank { chain, .. } = unit else {
                continue;
            };
            let tracks = &identity.lane_tracks;
            let armed = !tracks.is_empty()
                && tracks
                    .iter()
                    .all(|track| !track.is_empty() && eligible(track));
            chain.arm_mono_collapse(armed);
        }
    }

    /// Force every chain's collapse off (or back on). Bind-time, off the render thread.
    pub(crate) fn force_mono_collapse_off(&mut self, forced: bool) {
        for unit in self.units.iter_mut() {
            unit.force_mono_collapse_off(forced);
        }
    }

    /// Lanes whose route and master accumulation this bind folded into the chain's epilogue.
    pub(crate) const fn route_folds(&self) -> u64 {
        self.folds
    }

    /// Routes this bind retired into the session Output op's fused reduction (issue #926): one
    /// per input of that op when the fold was admitted, zero otherwise.
    #[cfg(test)]
    pub(crate) fn output_route_folds(&self) -> u64 {
        self.output_routes.len() as u64
    }

    #[cfg(test)]
    #[expect(
        clippy::too_many_arguments,
        reason = "the prepared runtime constructor keeps its fixed ownership partitions explicit"
    )]
    pub(crate) fn new(
        lease: ArenaLease,
        delays: Vec<CompensationDelay>,
        track_delays: Vec<TrackDelayLine>,
        units: Vec<RuntimeUnit>,
        split_pairs: Vec<Box<dyn GraphRuntimeSplitPairProcessor>>,
        identity: Vec<UnitIdentity>,
        response_bindings: Vec<ResponseOwnerBinding>,
        redirects: u64,
        folds: u64,
    ) -> Self {
        Self::new_with_observation_activation(
            lease,
            delays,
            track_delays,
            units,
            split_pairs,
            identity,
            response_bindings,
            redirects,
            folds,
            None,
            None,
            Vec::new(),
        )
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "the prepared runtime constructor keeps its fixed ownership partitions explicit"
    )]
    pub(crate) fn new_with_observation_activation(
        lease: ArenaLease,
        delays: Vec<CompensationDelay>,
        track_delays: Vec<TrackDelayLine>,
        units: Vec<RuntimeUnit>,
        split_pairs: Vec<Box<dyn GraphRuntimeSplitPairProcessor>>,
        mut identity: Vec<UnitIdentity>,
        response_bindings: Vec<ResponseOwnerBinding>,
        redirects: u64,
        folds: u64,
        observation_activation: Option<RealtimeObservationActivation>,
        output_unit: Option<usize>,
        output_routes: Vec<[f32; 4]>,
    ) -> Self {
        debug_assert_eq!(identity.len(), units.len());
        debug_assert!(
            output_unit.is_none_or(|unit| matches!(units.get(unit), Some(RuntimeUnit::Op(_)))),
            "the session Output op is a plain unit of its own"
        );
        // Issue #926: a route table is the Output op's, and it names one route per input of a
        // plain identity reduction. `output_route_fold` admitted exactly that shape at preflight.
        debug_assert!(
            output_routes.is_empty()
                || output_unit
                    .and_then(|unit| units.get(unit))
                    .is_some_and(|unit| matches!(
                        unit,
                        RuntimeUnit::Op(op) if matches!(op.kind, NodeKind::Identity)
                            && op.split_pair.is_none()
                            && op.sidechain.is_none()
                            && op.staged.is_empty()
                            && op.inputs.len() == output_routes.len()
                    )),
            "a folded route table belongs to the Output op's identity reduction"
        );
        for (row, unit) in identity.iter_mut().zip(&units) {
            row.observed = unit.has_observers();
        }
        #[cfg(any(test, feature = "test-support"))]
        if !split_pairs.is_empty() {
            TEST_ONLY_SPLIT_PAIR_TABLE.with(|value| {
                let mut witness = value.get();
                witness.allocations = witness.allocations.saturating_add(1);
                witness.entries = witness
                    .entries
                    .saturating_add(u64::try_from(split_pairs.len()).expect("table length"));
                witness.bytes = witness.bytes.saturating_add(
                    u64::try_from(core::mem::size_of::<Box<dyn GraphRuntimeSplitPairProcessor>>())
                        .expect("table entry size")
                        .saturating_mul(u64::try_from(split_pairs.len()).expect("table length")),
                );
                value.set(witness);
            });
        }
        let widest = units
            .iter()
            .map(|unit| match unit {
                RuntimeUnit::Op(_) => 0,
                RuntimeUnit::Bank { lanes, .. } => *lanes,
            })
            .max()
            .unwrap_or(0);
        Self {
            lease,
            delays: delays.into_boxed_slice(),
            track_delays: track_delays.into_boxed_slice(),
            units: units.into_boxed_slice(),
            observation_activation,
            observation_cursor: 0,
            observation_failure_invalidated: false,
            split_pairs: split_pairs.into_boxed_slice(),
            identity: identity.into_boxed_slice(),
            response_bindings: response_bindings.into_boxed_slice(),
            bank_inputs: vec![0; widest].into_boxed_slice(),
            bank_outputs: vec![0; widest].into_boxed_slice(),
            redirects,
            folds,
            output_unit,
            source_plane_of_buffer: Box::default(),
            output_routes: output_routes.into_boxed_slice(),
            output_sources: Box::default(),
        }
    }

    /// Copy the selected track's response-capable owners in the declared program order. This is
    /// a boundary-only walk: it borrows prepared state, never touches the audio lease, and leaves
    /// all caller-owned storage behind the opaque engine sink.
    pub(crate) fn copy_response_snapshot(
        &self,
        track_id: &str,
        sample_rate_hz: u32,
        sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<u32, ResponseSnapshotError> {
        let mut owners = 0_u32;
        for binding in self
            .response_bindings
            .iter()
            .filter(|binding| binding.track_id.as_ref() == track_id)
        {
            let unit = self
                .units
                .get(binding.unit)
                .ok_or(ResponseSnapshotError::Owner)?;
            unit.copy_response_snapshot(binding, sample_rate_hz, sink)?;
            owners = owners
                .checked_add(1)
                .ok_or(ResponseSnapshotError::Capacity)?;
        }
        if owners == 0 {
            return Err(ResponseSnapshotError::MissingTrack);
        }
        Ok(owners)
    }

    /// Whether claim `claim`, whose input is arena buffer `buffer`, is bound in place: its bank
    /// gathers the played block (issue #918), or the Output op's fused reduction reads it (issue
    /// #927), so the executor's copy loop skips it.
    pub(crate) fn source_in_place(&self, claim: usize, buffer: u32) -> bool {
        self.source_plane_of_buffer
            .get(buffer as usize)
            .is_some_and(|tabled| usize::try_from(*tabled).ok() == Some(claim))
    }

    // REALTIME_POLICY_BEGIN
    /// The audio of one buffer, for the source set to fill.
    pub(crate) fn buffer_mut(&mut self, buffer: u32) -> (&mut [f32], &mut [f32]) {
        self.lease.write_stereo(buffer)
    }

    /// The audio of one buffer, shared. Since issue #916 no production path reads a buffer back
    /// out of the arena: the session output is the host's planes, not an arena buffer.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn buffer(&self, buffer: u32) -> (&[f32], &[f32]) {
        self.lease.read_stereo(buffer)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn test_only_capture_failed_buffer(&self) {
        let Some(buffer) = TEST_ONLY_FAILED_BUFFER_TARGET.with(std::cell::Cell::get) else {
            return;
        };
        let (left, right) = self.buffer(ARENA_BASE + buffer);
        let mut capture = TestOnlyFailedBufferCapture {
            captured: true,
            frames: left.len(),
            overflow: left.len() > TEST_ONLY_FAILED_BUFFER_CAPACITY,
            left: [0; TEST_ONLY_FAILED_BUFFER_CAPACITY],
            right: [0; TEST_ONLY_FAILED_BUFFER_CAPACITY],
        };
        for (index, sample) in left
            .iter()
            .take(TEST_ONLY_FAILED_BUFFER_CAPACITY)
            .enumerate()
        {
            capture.left[index] = sample.to_bits();
            capture.right[index] = right[index].to_bits();
        }
        TEST_ONLY_FAILED_BUFFER.with(|value| value.set(capture));
    }

    /// Apply admitted activation snapshots and reset the block-local dispatch cursor.
    /// The phase a unit's time is charged to by `crate::test_only_phase_profile`.
    #[cfg(any(test, feature = "test-support"))]
    #[inline]
    pub(crate) fn test_only_unit_phase(&self, index: usize) -> usize {
        use crate::test_only_phase_profile::{
            BANK, BOUND, IDENTITY_ALIAS, IDENTITY_COPY, OTHER_OP, OUTPUT, ROUTE,
        };
        if self.output_unit == Some(index) {
            return OUTPUT;
        }
        match &self.units[index] {
            RuntimeUnit::Op(op) => match (&op.kind, &*op.inputs) {
                (NodeKind::Bound(_), _) => BOUND,
                (NodeKind::Route(_), _) => ROUTE,
                (NodeKind::Identity, [single]) if op.split_pair.is_none() => {
                    if *single == op.output {
                        IDENTITY_ALIAS
                    } else {
                        IDENTITY_COPY
                    }
                }
                _ => OTHER_OP,
            },
            RuntimeUnit::Bank { .. } => BANK,
        }
    }

    pub(crate) fn begin_observation_block(&mut self, first_sample: u64) {
        self.observation_cursor = 0;
        self.observation_failure_invalidated = false;
        let Self {
            observation_activation,
            units,
            ..
        } = self;
        let Some(activation) = observation_activation.as_mut() else {
            return;
        };
        activation.apply_boundary(first_sample, |entry, active, revision, sample| {
            if let Some(observer) = observer_at_entry(units, entry) {
                observer
                    .observer
                    .activation_changed(active, revision, sample);
            }
        });
    }

    pub(crate) fn has_observation_activation(&self) -> bool {
        self.observation_activation.is_some()
    }

    pub(crate) fn has_active_observation(&self) -> bool {
        self.observation_activation
            .as_ref()
            .is_some_and(|activation| !activation.entries().is_empty())
    }

    /// Runs unit `index`. Every producer this unit reads precedes it in `units`, or was written
    /// by a strictly earlier wave.
    ///
    /// `host` is the block's [`HostMaster`]. Two kinds of unit write it: the Output op's own unit
    /// ([`Self::output_unit`]), and every folded chain whose master is the Output
    /// ([`FoldTarget::Output`]), which all run before it. Every other unit ignores it.
    ///
    /// `sources` is the source set's played planes for this block (issue #918), `None` in a plan
    /// with no source set. Two reads use it. A bank's gather, only on a lane its unit marks
    /// (`UnitIdentity::source_lanes`) for a buffer [`Self::source_plane_of_buffer`] names. And the
    /// Output op's fused reduction (issue #927), only for an input [`Self::output_sources`] names,
    /// by position. Every other read of the arena is unchanged.
    pub(crate) fn execute(
        &mut self,
        index: usize,
        first_sample: u64,
        host: HostMaster<'_>,
        sources: Option<&dyn crate::GraphSourcePlanes>,
    ) -> Result<(), RenderError> {
        let Self {
            lease,
            delays,
            track_delays,
            units,
            split_pairs,
            identity,
            bank_inputs,
            bank_outputs,
            output_unit,
            source_plane_of_buffer,
            output_routes,
            output_sources,
            ..
        } = self;
        // GraphExecutor reaches this unit only after the previous execute and observe both
        // returned Ok in this block. Splitting borrows the two resident owners disjointly.
        let (before, current) = units.split_at_mut(index);
        let admitted = identity[index].resident_input;
        #[cfg(any(test, feature = "test-support"))]
        let admitted = admitted && !TEST_ONLY_RESIDENT_DISABLED.with(std::cell::Cell::get);
        let predecessor = if admitted {
            match before.last() {
                Some(RuntimeUnit::Bank { chain, .. }) => Some(chain),
                _ => None,
            }
        } else {
            None
        };
        let delays: &mut [CompensationDelay] = delays;
        let track_delays: &mut [TrackDelayLine] = track_delays;
        match &mut current[0] {
            RuntimeUnit::Op(op) => {
                let output = *output_unit == Some(index);
                let host = output.then_some(host);
                // Issue #926: only the Output op carries a route table; every other op reduces.
                let routes: &[[f32; 4]] = if output { output_routes } else { &[] };
                // Issue #927: and only the Output op's fused reduction reads a source claim's
                // played block, for the inputs `output_sources` names.
                let sources = if output {
                    OutputSources {
                        planes: sources,
                        claims: output_sources,
                    }
                } else {
                    OutputSources::NONE
                };
                execute_op(
                    op,
                    lease,
                    delays,
                    track_delays,
                    split_pairs,
                    first_sample,
                    host,
                    routes,
                    sources,
                )
            }
            RuntimeUnit::Bank {
                members,
                lanes,
                chain,
                fold,
                master,
            } => {
                let lanes = *lanes;
                // Only the first slot reduces graph inputs; the chain computes the rest -- and a
                // member whose whole reduction *is* the dedication copy does not even do that:
                // the gather reads its producer's buffer directly. See `bank_gather_source`.
                for (lane, member) in members[..lanes].iter_mut().enumerate() {
                    if let Some(source) = bank_gather_source(member) {
                        bank_inputs[lane] = source;
                    } else {
                        // A bank member is never the Output op.
                        execute_op(
                            member,
                            lease,
                            delays,
                            track_delays,
                            split_pairs,
                            first_sample,
                            None,
                            &[],
                            OutputSources::NONE,
                        )?;
                        bank_inputs[lane] = member.output;
                    }
                }
                let last = members.len() - lanes;
                for lane in 0..lanes {
                    bank_outputs[lane] = members[last + lane].output;
                }
                let frames = lease.frames();
                let master = match *master {
                    FoldTarget::Arena(buffer) => MasterPlanes::Arena(buffer),
                    FoldTarget::Output => MasterPlanes::Host(host),
                };
                let mut planes = ArenaMembers {
                    lease,
                    inputs: &bank_inputs[..lanes],
                    outputs: &bank_outputs[..lanes],
                    fold,
                    master,
                    sources: SourceGather {
                        planes: sources,
                        of_buffer: source_plane_of_buffer,
                        lanes: identity[index].source_lanes,
                    },
                };
                let frames = u32::try_from(frames).unwrap_or(u32::MAX);
                if let Some(predecessor) = predecessor {
                    #[cfg(any(test, feature = "test-support"))]
                    TEST_ONLY_RESIDENT_COUNTS.with(|count| {
                        let [gathers, residents] = count.get();
                        count.set([gathers, residents + 1]);
                    });
                    chain.run_with_resident_input(predecessor, &mut planes, frames, first_sample)
                } else {
                    chain.run(&mut planes, frames, first_sample)
                }
            }
        }
    }

    /// Dispatch only the active observer entries for one completed unit, preserving the catalog's
    /// lowered direct/alias/member order while avoiding a walk over dormant prepared bindings.
    ///
    /// `host` is the block's [`HostMaster`], which the Output op's observers read (issue #916).
    pub(crate) fn observe_active_unit(
        &mut self,
        index: usize,
        first_sample: u64,
        validity: GraphObservationValidity,
        host: &HostMaster<'_>,
    ) -> Result<(), RenderError> {
        let output = self.output_unit == Some(index);
        let Self {
            lease,
            units,
            observation_activation,
            observation_cursor,
            ..
        } = self;
        let Some(activation) = observation_activation.as_ref() else {
            return Ok(());
        };
        let entries = activation.entries();
        let mut planar = false;
        let mut planar_member = None;
        while let Some(entry) = entries.get(*observation_cursor).copied() {
            if entry.unit > index {
                break;
            }
            *observation_cursor += 1;
            if entry.unit < index {
                continue;
            }
            if planar_member != Some(entry.member) {
                planar = false;
                planar_member = Some(entry.member);
            }
            let output = output.then(|| host.planes());
            observe_active_entry(
                units,
                lease,
                entry,
                first_sample,
                validity,
                &mut planar,
                output,
            )?;
        }
        Ok(())
    }

    /// Materializes every pending split fader in original schedule order before a render error
    /// escapes. The first product slice admits at most one interval, but the walk keeps this
    /// owner-side contract explicit for later disjoint intervals.
    pub(crate) fn complete_pending(&mut self, first_sample: u64) {
        let Self {
            lease,
            units,
            split_pairs,
            ..
        } = self;
        for unit in units.iter_mut() {
            let RuntimeUnit::Op(op) = unit else {
                continue;
            };
            let Some(slot) = op.split_pair else {
                continue;
            };
            if !matches!(slot.role, SplitPairRole::Fader) {
                continue;
            }
            let (left, right) = lease.write_stereo(op.output);
            split_pairs[slot.pair].complete_pending(GraphBindingBlock {
                left,
                right,
                first_sample,
            });
        }
    }

    /// Runs observers in their existing member/direct/alias binding order.
    /// GraphExecutor calls this only after this unit successfully executes in this block.
    ///
    /// `host` is the block's [`HostMaster`], which the Output op's observers read (issue #916).
    pub(crate) fn observe_unit(
        &mut self,
        index: usize,
        first_sample: u64,
        validity: GraphObservationValidity,
        host: &HostMaster<'_>,
    ) -> Result<(), RenderError> {
        // Issue #900: a unit bound without any observer has nothing to dispatch and the walk
        // below has no other production effect, so one bind-time flag answers it. `execute` has
        // already indexed this unit's identity row in this block.
        let observed = self.identity[index].observed;
        #[cfg(test)]
        let observed = observed || TEST_ONLY_OBSERVER_SKIP_DISABLED.with(std::cell::Cell::get);
        if !observed {
            return Ok(());
        }
        let output = self.output_unit == Some(index);
        let Self { lease, units, .. } = self;
        match &mut units[index] {
            RuntimeUnit::Op(op) if output => {
                observe_output(op, host.planes(), first_sample, validity)
            }
            RuntimeUnit::Op(op) => observe(op, lease, first_sample, None, false, validity),
            RuntimeUnit::Bank {
                members,
                lanes,
                chain,
                ..
            } => {
                let population = *lanes;
                let width = chain.width().lanes() as usize;
                // execute scatters exactly the final slot's output members. A redirected
                // member's output is its consumer's buffer, which holds those scattered words
                // until the consumer's later unit, so a planar read here is the same words
                // redirected or not (issue #886); extra readers and sends decline the redirect
                // and consume the unchanged scatter. A folded lane's scatter goes to the
                // epilogue instead, which leaves the resident words untouched: they are what its
                // post-matrix observers read (issue #885).
                let eligible = population > 0
                    && population <= width
                    && !members.is_empty()
                    && members.len().is_multiple_of(population)
                    && chain.active().len() == width
                    && chain
                        .active()
                        .iter()
                        .enumerate()
                        .all(|(lane, active)| *active == (lane < population))
                    && chain.aux_lanes().is_empty();
                let frames = u32::try_from(lease.frames()).ok();
                let final_start = members.len().checked_sub(population);
                let chain: &BankChain = chain;
                for (index, member) in members.iter_mut().enumerate() {
                    #[cfg(any(test, feature = "test-support"))]
                    test_only_observation_dispatch_member_access();
                    let final_lane = final_start.and_then(|start| index.checked_sub(start));
                    let resident = if eligible {
                        final_lane.and_then(|lane| chain.final_output_lane(frames?, lane))
                    } else {
                        None
                    };
                    // The member buffer a folded lane's scatter skipped (issue #885).
                    let folded =
                        final_lane.is_some_and(|lane| chain.fold_lanes().get(lane) == Some(&true));
                    observe(member, lease, first_sample, resident, folded, validity)?;
                }
                Ok(())
            }
        }
    }

    /// Invalidate every prepared observer after a render failure before the error leaves the
    /// executor. This preserves the one-shot capture boundary without touching audio state.
    pub(crate) fn invalidate_observers(&mut self) {
        if self.observation_activation.is_some() {
            if self.observation_failure_invalidated {
                self.observation_failure_invalidated = false;
                return;
            }
            let entries = self
                .observation_activation
                .as_ref()
                .map_or(&[][..], RealtimeObservationActivation::entries);
            for entry in entries.iter().copied() {
                if let Some(observer) = observer_at_entry(&mut self.units, entry) {
                    observer.observer.invalidate();
                }
            }
            return;
        }
        for unit in &mut self.units {
            match unit {
                RuntimeUnit::Op(op) => {
                    for observer in &mut op.observers {
                        observer.observer.invalidate();
                    }
                }
                RuntimeUnit::Bank { members, .. } => {
                    for member in members {
                        for observer in &mut member.observers {
                            observer.observer.invalidate();
                        }
                    }
                }
            }
        }
    }

    /// Invalidate observers after a failed block while distinguishing a window completed by
    /// that block from an older result that remains valid for the control-side reader.
    pub(crate) fn invalidate_observers_after_failure(&mut self, failed_sample: u64) {
        if let Some(activation) = self.observation_activation.as_ref() {
            // Every currently active observer may hold a partial capture for this block, even if
            // its unit has not been reached yet. Dormant catalog rows are excluded by the active
            // snapshot itself; each active row receives exactly one failure notification.
            let entries = activation.entries();
            for entry in entries.iter().copied() {
                if let Some(observer) = observer_at_entry(&mut self.units, entry) {
                    observer.observer.invalidate_after_failure(failed_sample);
                }
            }
            self.observation_failure_invalidated = true;
            return;
        }
        for unit in &mut self.units {
            match unit {
                RuntimeUnit::Op(op) => {
                    for observer in &mut op.observers {
                        observer.observer.invalidate_after_failure(failed_sample);
                    }
                }
                RuntimeUnit::Bank { members, .. } => {
                    for member in members {
                        for observer in &mut member.observers {
                            observer.observer.invalidate_after_failure(failed_sample);
                        }
                    }
                }
            }
        }
    }
    // REALTIME_POLICY_END
}

// REALTIME_POLICY_BEGIN
/// The buffer a first-slot bank member's gather may read **instead of** the member's own output,
/// or `None` when the member has to run.
///
/// The SIMD-rack effects sit in dedicated racks (`program::is_dedicated`), so their ops are never
/// `in_place` and `reduce_plane` memcpys the upstream output into a buffer whose only reader is
/// this bank's gather -- after which the chain's scatter fully overwrites that same buffer. The
/// copy is therefore pure cost, and pointing the gather at the producer removes it. `None` on any
/// doubt; every clause below is one way the copy could be load-bearing:
///
/// * **Nothing but the reduction.** [`NodeKind::BankMember`] is the kind whose `execute_op` body is
///   empty apart from `reduce_plane`, so skipping the op skips exactly the copy and nothing else.
/// * **One undelayed, unmixed input.** Two inputs is a sum and zero inputs is a `fill(0.0)`; either
///   way the gather source is a value no single buffer holds. A sidechain already blocks banking
///   (#96 F9) and a `staged` input owns a compensation delay line that must still be pumped.
/// * **Not already in place.** `inputs[0] == output` is the lowering having elided the copy
///   already, and redirecting the gather would change nothing.
///
/// What stays true of the member's own output buffer: for a single-slot chain it *is* the scatter
/// target, so every later reader -- an observer, a `program::Tap`'s observer, the session output --
/// sees exactly the bits it saw before. For a multi-slot cohort chain the scatter lands in the last
/// slot's buffer instead and this one is left holding the previous block's words, which is sound
/// for the reason `chains_into` already requires and checks: the first slot has exactly one reader
/// (the next slot's op, which never reduces), no tap, no observer, and is not the session output.
///
/// This does not touch `is_dedicated`, which stays a classification by node kind: dedication *by
/// bank membership* was measured and rejected, and `program::lower` records why (#169).
fn bank_gather_source(member: &RuntimeOp) -> Option<u32> {
    if !matches!(member.kind, NodeKind::BankMember)
        || !member.staged.is_empty()
        || member.sidechain.is_some()
    {
        return None;
    }
    match &*member.inputs {
        [single] if *single != member.output => Some(*single),
        _ => None,
    }
}

fn observer_at_entry(
    units: &mut [RuntimeUnit],
    entry: ActivationEntry,
) -> Option<&mut GraphNodeObserverBinding> {
    match units.get_mut(entry.unit)? {
        RuntimeUnit::Op(op) if entry.member.is_none() => op.observers.get_mut(entry.observer),
        RuntimeUnit::Bank { members, .. } => members
            .get_mut(entry.member?)
            .and_then(|member| member.observers.get_mut(entry.observer)),
        RuntimeUnit::Op(_) => None,
    }
}

/// One active observer entry. `output` is the host's planes when `entry.unit` runs the session
/// Output op (issue #916), and `None` for every other unit.
fn observe_active_entry(
    units: &mut [RuntimeUnit],
    lease: &mut ArenaLease,
    entry: ActivationEntry,
    first_sample: u64,
    validity: GraphObservationValidity,
    planar: &mut bool,
    output: Option<(&[f32], &[f32])>,
) -> Result<(), RenderError> {
    match units.get_mut(entry.unit) {
        Some(RuntimeUnit::Op(op)) if entry.member.is_none() => {
            let buffer = op.output;
            let observer = op
                .observers
                .get_mut(entry.observer)
                .ok_or(RenderError::InvalidEnvelope)?;
            #[cfg(any(test, feature = "test-support"))]
            test_only_observation_dispatch_observer_access();
            match output {
                Some(planes) => {
                    observe_output_one(observer, planes, first_sample, validity, planar)
                }
                None => observe_one(
                    observer,
                    lease,
                    buffer,
                    None,
                    false,
                    first_sample,
                    validity,
                    planar,
                ),
            }
        }
        Some(RuntimeUnit::Bank {
            members,
            lanes,
            chain,
            ..
        }) => {
            let member_index = entry.member.ok_or(RenderError::InvalidEnvelope)?;
            let eligible = {
                let population = *lanes;
                let width = chain.width().lanes() as usize;
                population > 0
                    && population <= width
                    && !members.is_empty()
                    && members.len().is_multiple_of(population)
                    && chain.active().len() == width
                    && chain
                        .active()
                        .iter()
                        .enumerate()
                        .all(|(lane, active)| *active == (lane < population))
                    && chain.aux_lanes().is_empty()
            };
            let frames = u32::try_from(lease.frames()).ok();
            let final_lane = members
                .len()
                .checked_sub(*lanes)
                .and_then(|start| member_index.checked_sub(start));
            let resident = if eligible {
                final_lane.and_then(|lane| BankChain::final_output_lane(chain, frames?, lane))
            } else {
                None
            };
            // The member buffer a folded lane's scatter skipped (issue #885).
            let folded = final_lane.is_some_and(|lane| chain.fold_lanes().get(lane) == Some(&true));
            let member = members
                .get_mut(member_index)
                .ok_or(RenderError::InvalidEnvelope)?;
            #[cfg(any(test, feature = "test-support"))]
            test_only_observation_dispatch_member_access();
            let output = member.output;
            let observer = member
                .observers
                .get_mut(entry.observer)
                .ok_or(RenderError::InvalidEnvelope)?;
            #[cfg(any(test, feature = "test-support"))]
            test_only_observation_dispatch_observer_access();
            observe_one(
                observer,
                lease,
                output,
                resident,
                folded,
                first_sample,
                validity,
                planar,
            )
        }
        Some(RuntimeUnit::Op(_)) | None => Err(RenderError::InvalidEnvelope),
    }
}

/// The one implementation of node semantics, shared by both executors.
///
/// `host` is `Some` for the session Output op alone (issue #916). The op's output is then the
/// host's planes for the whole op: its reduction ([`reduce_plane_into`]) and its processing write
/// them where every other op writes `op.output`. Every write of the op's own output goes through
/// [`output_planes`] or [`output_and_sidechain_planes`], so the two storages cannot diverge site by
/// site. Staging a delayed input still goes through the arena, because a staging buffer is the
/// op's input, not its output.
///
/// `routes` is empty for every op but the session Output op, and empty for that op too unless the
/// bind retired every route that feeds it (issue #926, [`output_route_fold`]). When it is not
/// empty, entry `i` is the folded 2x2 of the route whose in-place buffer is `op.inputs[i]`, and
/// the reduction is [`route_reduce`]: each route's `mix2x2_block` taken in registers on the way
/// into the sum, rather than as a store pass of its own before it. `sources` then says which of
/// those inputs [`route_reduce`] reads from a source claim's played block instead of the arena
/// (issue #927); it is [`OutputSources::NONE`] for every other op, and only `route_reduce` reads
/// it.
#[expect(
    clippy::too_many_arguments,
    reason = "the op's owners and its two optional Output-op storages stay explicit parameters"
)]
fn execute_op(
    op: &mut RuntimeOp,
    lease: &mut ArenaLease,
    delays: &mut [CompensationDelay],
    track_delays: &mut [TrackDelayLine],
    split_pairs: &mut [Box<dyn GraphRuntimeSplitPairProcessor>],
    first_sample: u64,
    mut host: Option<HostMaster<'_>>,
    routes: &[[f32; 4]],
    sources: OutputSources<'_>,
) -> Result<(), RenderError> {
    let output = op.output;
    if let NodeKind::TrackDelay { line, .. } = op.kind {
        // The delayed form of the `SourceInput` arm below, in the same position and for the same
        // reason: the coordinator's source set already wrote this node's output for this block, and
        // an input node has no graph inputs, so there is no reduction to run. Falling through would
        // reach `reduce_plane` with an empty input list, whose `[]` arm fills the buffer with `0.0`
        // -- straight over the source audio. The alignment therefore happens here, in place, and
        // this returns exactly where the undelayed arm returns.
        let (left, right) = output_planes(lease, &mut host, output);
        track_delays[line as usize].process(left, right);
        return Ok(());
    }
    if matches!(op.kind, NodeKind::SourceInput) {
        // The coordinator's source set already wrote this node's output for this block.
        return Ok(());
    }
    // A delayed edge is the only inter-parcel copy left, and the *consuming* parcel makes it.
    for staged in &op.staged {
        {
            let (destination, input) = lease.write_read(0, staged.staging, staged.source);
            destination.copy_from_slice(input);
        }
        {
            let (destination, input) = lease.write_read(1, staged.staging, staged.source);
            destination.copy_from_slice(input);
        }
        let (staged_left, staged_right) = lease.write_stereo(staged.staging);
        delays[staged.line as usize].process(staged_left, staged_right);
    }
    // The fan-in-zero fill is dead under a bound source (issue #218). `reduce_plane`'s `[]` arm
    // fills the buffer with `0.0` so that a node with no contributors renders silence; a bound
    // node with no graph inputs has a contributor, and it is the host's processor, which is
    // required to write every word of the block it is handed. Filling first and overwriting
    // second is two whole stereo blocks of stores per bound source per block -- 64 of them on the
    // intended fixture -- and not one of those words is ever read.
    //
    // The clause is exactly "no graph inputs *and* a host processor". A `NodeKind::Identity` with
    // no inputs is a submix nothing routes into and its fill **is** its audio; a `SourceInput` and
    // a `TrackDelay` are already skipped above; every other kind reduces first and processes in
    // place, so its fill is the value it processes.
    if !op.inputs.is_empty() || !matches!(op.kind, NodeKind::Bound(_)) {
        match host.as_mut() {
            None => {
                reduce_plane(lease, 0, output, &op.inputs);
                reduce_plane(lease, 1, output, &op.inputs);
            }
            Some(host) if routes.is_empty() => {
                let (left, right) = host.planes_mut();
                reduce_plane_into(lease, 0, output, left, &op.inputs);
                reduce_plane_into(lease, 1, output, right, &op.inputs);
            }
            Some(host) => {
                // Issue #926: the retired routes' mixes, fused into the Output's reduction. A
                // refused shape cannot occur for an admitted fold; were it to, the error path
                // silences the host planes rather than play a partial sum.
                let (left, right) = host.planes_mut();
                if !route_reduce::<FrameLane>(lease, &op.inputs, routes, sources, left, right) {
                    return Err(RenderError::InvalidEnvelope);
                }
            }
        }
    }
    match &mut op.kind {
        // `TrackDelay` returned above, before the reduction it must not run; it is named here only
        // because the match is exhaustive.
        NodeKind::TrackDelay { .. } | NodeKind::SourceInput | NodeKind::BankMember => {}
        NodeKind::Identity => {
            if let Some(slot) = op.split_pair {
                let (out_left, out_right) = output_planes(lease, &mut host, output);
                let block = GraphBindingBlock {
                    left: out_left,
                    right: out_right,
                    first_sample,
                };
                return match slot.role {
                    SplitPairRole::Fader => split_pairs[slot.pair].begin_fader(block),
                    SplitPairRole::Matrix => split_pairs[slot.pair].finish_matrix(block),
                };
            }
        }
        NodeKind::Route(coefficients) => {
            let (out_left, out_right) = output_planes(lease, &mut host, output);
            mix2x2_block::<FrameLane>(out_left, out_right, *coefficients);
        }
        NodeKind::Bound(processor) => {
            let (out_left, out_right) = output_planes(lease, &mut host, output);
            processor.process(GraphBindingBlock {
                left: out_left,
                right: out_right,
                first_sample,
            })?;
        }
        NodeKind::Effect(effect) => {
            let quantum = effect.metadata.quantum;
            match op.sidechain {
                None => {
                    let (out_left, out_right) = output_planes(lease, &mut host, output);
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        None,
                        first_sample,
                        &[],
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
                Some(sidechain) => {
                    let ((out_left, out_right), (side_left, side_right)) =
                        output_and_sidechain_planes(lease, &mut host, output, sidechain);
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        Some((side_left, side_right)),
                        first_sample,
                        &[],
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
            }
        }
        NodeKind::ConsoleEffect(console) => {
            // The drain runs before a single sample is touched, so an admitted record takes
            // effect on the first sample of this block -- the exact `applied_at_sample` the
            // control side was acknowledged with (#137 E1's rule, now for effects).
            // Issue #143 D3: the subscription and the parameter commands are drained by this one
            // call, so a batch that changes a threshold and arms its tap lands on one sample
            // timeline by construction rather than by two clocks agreeing.
            let staged = console.control.stage(
                &mut console.spans,
                first_sample,
                console.observation.as_deref_mut(),
            );
            // Preparation refuses a queue deeper than the effect's automation capacity, so a full
            // drain can never produce more distinct spans than the window holds. This is the
            // invariant, not a runtime policy: in release it costs nothing.
            debug_assert_eq!(staged.dropped, 0, "console staging window overflowed");
            if staged.target_error {
                return Err(RenderError::InvalidEnvelope);
            }
            let automation = &console.spans[..staged.staged];
            let bypassed = console.control.bypassed();
            // Issue #163 phase 4 item 4: the dry staging is read only by the `apply` below, and
            // only when this block is bypassed. `bypassed` is already decided here — the control
            // drain that could change it ran above — so the capture is skippable for an
            // un-bypassed block *unless* the shunt carries a latency line, which has to be fed on
            // every block whatever the bypass state. Both readers of `dry_*` are later in this
            // same block, so nothing crosses a block boundary and the skip moves no rendered bit.
            let capture_dry = bypassed || console.shunt.feeds_line();
            let effect = &mut console.effect;
            for target in console.control.prepared_targets() {
                effect
                    .processor
                    .apply_prepared_target(target)
                    .map_err(|_| RenderError::InvalidEnvelope)?;
            }
            let quantum = effect.metadata.quantum;
            match op.sidechain {
                None => {
                    let (out_left, out_right) = output_planes(lease, &mut host, output);
                    if capture_dry {
                        console.shunt.capture(out_left, out_right);
                    }
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        None,
                        first_sample,
                        automation,
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
                Some(sidechain) => {
                    let ((out_left, out_right), (side_left, side_right)) =
                        output_and_sidechain_planes(lease, &mut host, output, sidechain);
                    if capture_dry {
                        console.shunt.capture(out_left, out_right);
                    }
                    let block = EffectProcessBlock::new(
                        out_left,
                        out_right,
                        Some((side_left, side_right)),
                        first_sample,
                        automation,
                        quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
            }
            if bypassed {
                let (out_left, out_right) = output_planes(lease, &mut host, output);
                console.shunt.apply(out_left, out_right);
            }
            // Issue #143: after `process`, and after the bypass shunt, so an observed value always
            // describes the block that was actually emitted.
            if let Some(observation) = console.observation.as_deref_mut() {
                publish_observations(
                    observation,
                    console.effect.processor.as_ref(),
                    first_sample,
                    lease.frames() as u64,
                );
            }
        }
    }
    Ok(())
}

/// An op's output planes for this block: its arena buffer, or the host's planes when `host` is
/// `Some`, which it is for the session Output op alone (issue #916).
#[inline]
fn output_planes<'b>(
    lease: &'b mut ArenaLease,
    host: &'b mut Option<HostMaster<'_>>,
    output: u32,
) -> (&'b mut [f32], &'b mut [f32]) {
    match host {
        Some(host) => host.planes_mut(),
        None => lease.write_stereo(output),
    }
}

/// [`output_planes`] beside both planes of a sidechain read: `lease.write_read_stereo` for an arena
/// output, and the host's planes beside `lease.read_stereo` for the session Output op.
#[inline]
fn output_and_sidechain_planes<'b>(
    lease: &'b mut ArenaLease,
    host: &'b mut Option<HostMaster<'_>>,
    output: u32,
    sidechain: u32,
) -> ArenaStereoPair<'b> {
    match host {
        Some(host) => (host.planes_mut(), lease.read_stereo(sidechain)),
        None => lease.write_read_stereo(output, sidechain),
    }
}

// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN
/// Run one op's observers in binding order, offering each the resident view first.
///
/// `folded` says the chain folded this member's route (issue #885), so its scatter never wrote
/// `op.output`: the first observer that declines the resident view has the resident words written
/// there before any planar block is formed. `resident` is the only source of those words. The
/// planar block is re-sliced per declining observer rather than cached, because the lease has to
/// stay writable until that first acquisition.
fn observe(
    op: &mut RuntimeOp,
    lease: &mut ArenaLease,
    first_sample: u64,
    resident: Option<rack::ResidentOutputLane<'_>>,
    folded: bool,
    validity: GraphObservationValidity,
) -> Result<(), RenderError> {
    #[cfg(test)]
    TEST_ONLY_OBSERVE_CALLS.with(|calls| calls.set(calls.get() + 1));
    let words = resident;
    #[cfg(any(test, feature = "test-support"))]
    let resident =
        resident.filter(|_| !TEST_ONLY_METER_RESIDENT_DISABLED.with(std::cell::Cell::get));
    let mut planar = false;
    for observer in op.observers.iter_mut() {
        #[cfg(any(test, feature = "test-support"))]
        test_only_observation_dispatch_observer_access();
        if let Some(lane) = resident {
            #[cfg(any(test, feature = "test-support"))]
            TEST_ONLY_METER_INPUT_COUNTS.with(|value| {
                let mut counts = value.get();
                counts[1] += 1;
                value.set(counts);
            });
            if let Some(result) =
                observer
                    .observer
                    .observe_resident(crate::GraphResidentObservationBlock {
                        lane,
                        first_sample,
                        validity,
                    })
            {
                #[cfg(any(test, feature = "test-support"))]
                TEST_ONLY_METER_INPUT_COUNTS.with(|value| {
                    let mut counts = value.get();
                    counts[2] += 1;
                    value.set(counts);
                });
                result?;
                continue;
            }
        }
        if !planar {
            #[cfg(any(test, feature = "test-support"))]
            TEST_ONLY_METER_INPUT_COUNTS.with(|value| {
                let mut counts = value.get();
                counts[0] += 1;
                value.set(counts);
            });
            if folded {
                write_resident_lane(lease, op.output, words)?;
            }
            planar = true;
        }
        let (left, right) = lease.read_stereo(op.output);
        observer.observer.observe_with_validity(
            GraphObservationBlock {
                left,
                right,
                first_sample,
            },
            validity,
        )?;
    }
    Ok(())
}

// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN
/// One active observer entry: [`observe`]'s body for a single row, with `planar` recording whether
/// this member's planar block was already acquired -- and, when `folded`, written -- this block.
#[expect(
    clippy::too_many_arguments,
    reason = "issue #885 adds the folded-lane flag to the existing seven-parameter dispatch"
)]
fn observe_one(
    observer: &mut GraphNodeObserverBinding,
    lease: &mut ArenaLease,
    output: u32,
    resident: Option<rack::ResidentOutputLane<'_>>,
    folded: bool,
    first_sample: u64,
    validity: GraphObservationValidity,
    planar: &mut bool,
) -> Result<(), RenderError> {
    let words = resident;
    #[cfg(any(test, feature = "test-support"))]
    let resident =
        resident.filter(|_| !TEST_ONLY_METER_RESIDENT_DISABLED.with(std::cell::Cell::get));
    if let Some(lane) = resident {
        #[cfg(any(test, feature = "test-support"))]
        TEST_ONLY_METER_INPUT_COUNTS.with(|value| {
            let mut counts = value.get();
            counts[1] += 1;
            value.set(counts);
        });
        if let Some(result) =
            observer
                .observer
                .observe_resident(crate::GraphResidentObservationBlock {
                    lane,
                    first_sample,
                    validity,
                })
        {
            #[cfg(any(test, feature = "test-support"))]
            TEST_ONLY_METER_INPUT_COUNTS.with(|value| {
                let mut counts = value.get();
                counts[2] += 1;
                value.set(counts);
            });
            result?;
            return Ok(());
        }
    }
    if !*planar {
        #[cfg(any(test, feature = "test-support"))]
        TEST_ONLY_METER_INPUT_COUNTS.with(|value| {
            let mut counts = value.get();
            counts[0] += 1;
            value.set(counts);
        });
        if folded {
            write_resident_lane(lease, output, words)?;
        }
        *planar = true;
    }
    let (left, right) = lease.read_stereo(output);
    observer.observer.observe_with_validity(
        GraphObservationBlock {
            left,
            right,
            first_sample,
        },
        validity,
    )
}

/// Run the session Output op's observers over the host's planes (issue #916).
///
/// This is [`observe`] for the one op whose storage is not an arena buffer. The Output op is never
/// a bank member, so it has no resident view and no folded buffer to write first. What is left of
/// `observe` is its planar path, and that is all this is: the same binding order, the same
/// dispatch counters, and one planar acquisition per block. The block each observer reads is the
/// planes the op just reduced and processed into, which is the block `lease.read_stereo(output)`
/// used to return.
fn observe_output(
    op: &mut RuntimeOp,
    planes: (&[f32], &[f32]),
    first_sample: u64,
    validity: GraphObservationValidity,
) -> Result<(), RenderError> {
    #[cfg(test)]
    TEST_ONLY_OBSERVE_CALLS.with(|calls| calls.set(calls.get() + 1));
    let mut planar = false;
    for observer in op.observers.iter_mut() {
        #[cfg(any(test, feature = "test-support"))]
        test_only_observation_dispatch_observer_access();
        observe_output_one(observer, planes, first_sample, validity, &mut planar)?;
    }
    Ok(())
}

/// One observer of the session Output op: [`observe_one`]'s planar path over the host's planes.
fn observe_output_one(
    observer: &mut GraphNodeObserverBinding,
    (left, right): (&[f32], &[f32]),
    first_sample: u64,
    validity: GraphObservationValidity,
    planar: &mut bool,
) -> Result<(), RenderError> {
    if !*planar {
        #[cfg(any(test, feature = "test-support"))]
        TEST_ONLY_METER_INPUT_COUNTS.with(|value| {
            let mut counts = value.get();
            counts[0] += 1;
            value.set(counts);
        });
        *planar = true;
    }
    observer.observer.observe_with_validity(
        GraphObservationBlock {
            left,
            right,
            first_sample,
        },
        validity,
    )
}

/// Write one folded lane's resident final words into its own member buffer (issue #885).
///
/// This is that lane's share of the scatter the fold replaced, done only when an observer asks for
/// a planar block: a pure word copy out of the resident AoSoA block, lane `words.lane()` of every
/// frame, so the block a declining observer reads is bit for bit the one an unfolded chain would
/// have scattered there. Nothing else reads the buffer -- its sole reader was the route, which the
/// fold retired -- and the write cannot land on a live neighbour: an unfolded chain scatters these
/// same words into this buffer at this same point, and its observers read them here, so the
/// lowering's colouring already reserves the slot for this value from the chain's unit until the
/// retired route's position.
///
/// `words` is `None` only for a chain without the full-population lane shape `final_output_lane`
/// and the dispatchers' `eligible` clauses require -- a shape no rendering chain has, because a
/// chain gathers lane `l` from member `l`. The render fails rather than hand an observer last
/// block's words.
fn write_resident_lane(
    lease: &mut ArenaLease,
    output: u32,
    words: Option<rack::ResidentOutputLane<'_>>,
) -> Result<(), RenderError> {
    let Some(words) = words else {
        return Err(RenderError::InvalidEnvelope);
    };
    let width = words.width().lanes() as usize;
    let (left, right) = lease.write_stereo(output);
    for (plane, source) in [(left, words.left()), (right, words.right())] {
        for (word, value) in plane
            .iter_mut()
            .zip(source.iter().skip(words.lane()).step_by(width))
        {
            *word = *value;
        }
    }
    Ok(())
}

// REALTIME_POLICY_END

/// Ordered observers of one node, plus the observers of every alias resolving to its buffer.
pub(crate) fn take_observers(
    observers: &mut BTreeMap<crate::GraphNodeId, Vec<GraphNodeObserverBinding>>,
    nodes: impl Iterator<Item = crate::GraphNodeId>,
) -> Box<[GraphNodeObserverBinding]> {
    let mut collected = Vec::new();
    for node in nodes {
        if let Some(mut bound) = observers.remove(&node) {
            bound.sort_by_key(|observer| observer.handle);
            collected.append(&mut bound);
        }
    }
    collected.into_boxed_slice()
}

// ---------------------------------------------------------------------------------------------
// Build (control plane): one node builder, one bank builder, for both executors.
// ---------------------------------------------------------------------------------------------

use effect_contract::BankWidth;
use rack::{AoSoaScratch, BankBlock, BankSlot, BankStage, ConsoleEffectBankStage, EffectBankStage};

use crate::{
    GraphNodeBinding, GraphNodeId, GraphPreparedBuiltinBank, GraphPreparedBuiltinBankProcessor,
    GraphPreparedEffectBank, GraphSpec, PreparedRoute, RouteTransform, TrackStage,
    program::{ExecutionProgram, Op},
};

/// Adapter that lets a compiler-owned builtin bank act as a chain slot.
struct BuiltinStage(Box<dyn GraphPreparedBuiltinBankProcessor>);
impl BankStage for BuiltinStage {
    // REALTIME_POLICY_BEGIN
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(block.left, block.right, block.frames, block.first_sample)
    }
    fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        sample_rate_hz: u32,
        request: OwnerSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.0
            .copy_response_snapshot_lane(lane, sample_rate_hz, request)
    }
    // REALTIME_POLICY_END
    /// The drain, forwarded. `BankChain::run` calls this on every slot before it reads the
    /// collapse witness, which is the ordering the input bank's trim/polarity drain depends on.
    fn begin_block(&mut self, first_sample: u64) -> Result<(), RenderError> {
        self.0.begin_block(first_sample)
    }
    fn qualification_counters(&self) -> [u64; 2] {
        self.0.qualification_counters()
    }
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        self.0.lane_symmetry(lane)
    }
    fn seam_side(&self) -> effect_contract::SeamSide {
        self.0.seam_side()
    }
    fn supports_mono_collapse(&self) -> bool {
        self.0.supports_mono_collapse()
    }
    /// The one-plane body. `block.right` is the ungathered scratch and is not passed on: the
    /// builtin banks take a single plane, so a mis-wired collapse cannot reach a stale plane
    /// through this adapter at all.
    fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process_mono(block.left, block.frames, block.first_sample)
    }
    fn desymmetrize(&mut self) {
        self.0.desymmetrize();
    }
    fn channels_agree(&self) -> bool {
        self.0.channels_agree()
    }
}

/// A padded group's active mask **is** its membership: the planner emits `Some` members before
/// every `None`, so lane `i` is active exactly while `i < members`. The mask is therefore derived
/// here and stored nowhere (#86 F9's graph half).
pub(crate) fn trailing_active_mask(members: usize, width: BankWidth) -> Box<[bool]> {
    (0..width.lanes() as usize)
        .map(|lane| lane < members)
        .collect()
}

/// One chain over `slots`, in cascade order, sharing one resident block.
///
/// `active` is the chain's lane mask; every slot carries it too, because a cohort chain binds a
/// slot only when *every* lane of the group runs it (`banks::bind_rack_banks`, #96 F7). The rack
/// crate has supported multi-slot chains since it was written and unit-tests three of them; until
/// issue #181 nothing in the graph layer ever handed it more than one.
#[cfg(feature = "test-support")]
fn bank_chain(
    scratch: AoSoaScratch,
    active: Box<[bool]>,
    slots: Vec<Box<dyn BankStage>>,
) -> BankChain {
    bank_chain_with_fold(scratch, active, slots, None)
}

fn bank_chain_with_fold(
    scratch: AoSoaScratch,
    active: Box<[bool]>,
    slots: Vec<Box<dyn BankStage>>,
    fold: Option<rack::PreparedFoldConfiguration>,
) -> BankChain {
    let slot_count = slots.len();
    let mut prepared_slots = Vec::with_capacity(slot_count);
    for stage in slots {
        prepared_slots.push(BankSlot {
            stage,
            active_lanes: active.clone(),
        });
    }
    debug_assert_eq!(
        prepared_slots.capacity(),
        slot_count,
        "bank slot Vec must retain the explicitly requested capacity"
    );
    #[cfg(feature = "test-support")]
    BANK_CHAIN_CAPACITIES.with(|value| value.set([prepared_slots.capacity(), slot_count]));
    match fold {
        Some(configuration) => {
            BankChain::new_with_prepared_fold(scratch, configuration, prepared_slots)
        }
        None => BankChain::new(scratch, active, prepared_slots),
    }
    .expect("validated bank shape")
}

#[cfg(feature = "test-support")]
thread_local! {
    static BANK_CHAIN_CAPACITIES: std::cell::Cell<[usize; 2]> = const { std::cell::Cell::new([0, 0]) };
    static BANK_CHAIN_FACTS: std::cell::Cell<TestOnlyBankChainConstructionFacts> =
        const { std::cell::Cell::new(TestOnlyBankChainConstructionFacts::ZERO) };
}

#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct TestOnlyBankChainConstructionFacts {
    pub prepared_memberships: usize,
    pub run_memberships: usize,
    pub runtime_slots: usize,
    pub maximum_run_memberships: usize,
    pub maximum_runtime_slots: usize,
}

#[cfg(feature = "test-support")]
impl TestOnlyBankChainConstructionFacts {
    const ZERO: Self = Self {
        prepared_memberships: 0,
        run_memberships: 0,
        runtime_slots: 0,
        maximum_run_memberships: 0,
        maximum_runtime_slots: 0,
    };
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_reset_bank_chain_construction_facts() {
    BANK_CHAIN_FACTS.with(|facts| facts.set(TestOnlyBankChainConstructionFacts::ZERO));
}

#[cfg(feature = "test-support")]
#[must_use]
#[doc(hidden)]
pub fn test_only_bank_chain_construction_facts() -> TestOnlyBankChainConstructionFacts {
    BANK_CHAIN_FACTS.with(std::cell::Cell::get)
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub struct TestOnlyBankChainOwnership {
    chain: BankChain,
    pub slot_count: usize,
    pub requested_stage_capacity: usize,
    /// Capacity of the incoming public `Vec<BankSlot>`, before private conversion.
    pub runtime_slot_capacity: usize,
    /// The boxed slice receives all initialized Vec elements; this is its inferred retained count.
    pub inferred_retained_slot_count: usize,
    pub mask_bytes: usize,
    pub stage_pointer_bytes: usize,
    pub slot_bytes: usize,
}

#[cfg(feature = "test-support")]
impl TestOnlyBankChainOwnership {
    #[doc(hidden)]
    pub fn test_only_chain_mut(&mut self) -> &mut BankChain {
        &mut self.chain
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub struct TestOnlyBankChainInputs {
    scratch: AoSoaScratch,
    active: Box<[bool]>,
    stages: Vec<Box<dyn BankStage>>,
    stage_capacity: usize,
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_bank_chain_inputs(
    width: BankWidth,
    slot_count: usize,
) -> TestOnlyBankChainInputs {
    struct IdentityStage;
    impl BankStage for IdentityStage {
        fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }

        fn lane_symmetry(&self, _lane: usize) -> ChannelSymmetryWitness {
            ChannelSymmetryWitness::SYMMETRIC
        }

        fn supports_mono_collapse(&self) -> bool {
            true
        }

        fn process_mono(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }

        fn channels_agree(&self) -> bool {
            true
        }
    }

    let scratch = AoSoaScratch::new(width, 1).expect("test width and quantum");
    // Keep this ownership oracle ragged so tiled staging remains a separate owner. Slot count and
    // lane width are independent: S=0 still has a legal nonempty direct-caller chain mask.
    let active = trailing_active_mask(width.lanes() as usize - 1, width);
    let mut stages = Vec::with_capacity(slot_count);
    for _ in 0..slot_count {
        stages.push(Box::new(IdentityStage) as Box<dyn BankStage>);
    }
    let stage_capacity = stages.capacity();
    TestOnlyBankChainInputs {
        scratch,
        active,
        stages,
        stage_capacity,
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_bank_chain_ownership(
    inputs: TestOnlyBankChainInputs,
) -> TestOnlyBankChainOwnership {
    let TestOnlyBankChainInputs {
        scratch,
        active,
        stages,
        stage_capacity,
    } = inputs;
    let slot_count = stages.len();
    let mask_bytes = core::mem::size_of::<bool>() * active.len();
    let chain = bank_chain(scratch, active, stages);
    let [runtime_slot_capacity, inferred_retained_slot_count] =
        BANK_CHAIN_CAPACITIES.with(std::cell::Cell::get);
    TestOnlyBankChainOwnership {
        chain,
        slot_count,
        requested_stage_capacity: stage_capacity,
        runtime_slot_capacity,
        inferred_retained_slot_count,
        mask_bytes,
        stage_pointer_bytes: core::mem::size_of::<Box<dyn BankStage>>() * slot_count,
        slot_bytes: core::mem::size_of::<BankSlot>() * slot_count,
    }
}

/// Which unit a node belongs to, when it is a bank member.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Membership {
    Effect(usize),
    Builtin(usize),
}

/// Node index -> `(bank, member position)`, for every homogeneous-bank member of a plan.
pub(crate) type BankMembership = BTreeMap<u32, (Membership, usize)>;

/// One scheduling unit before it is built: its bank, if any, and the ops it owns.
pub(crate) type PlannedUnit = (Option<Membership>, Vec<usize>);

/// The one bank-membership map, built from the plan before either executor exists.
pub(crate) fn bank_membership(
    spec: &GraphSpec,
    banks: &[GraphPreparedEffectBank],
    builtin_banks: &[GraphPreparedBuiltinBank],
) -> BankMembership {
    let index_of =
        |node: &GraphNodeId| crate::program::node_index(spec, node).expect("validated member");
    let mut membership = BankMembership::new();
    for (bank, prepared) in banks.iter().enumerate() {
        for (member, id) in prepared.members.iter().enumerate() {
            membership.insert(
                index_of(&GraphNodeId::Effect(id.clone())),
                (Membership::Effect(bank), member),
            );
        }
    }
    for (bank, prepared) in builtin_banks.iter().enumerate() {
        for (member, id) in prepared.members.iter().enumerate() {
            membership.insert(index_of(id), (Membership::Builtin(bank), member));
        }
    }
    membership
}

/// Everything a bound plan hands the runtime, consumed exactly once per node.
pub(crate) struct RuntimeParts {
    pub(crate) routes: BTreeMap<GraphNodeId, RouteTransform>,
    pub(crate) effects: BTreeMap<GraphNodeId, GraphPreparedEffect>,
    /// Issue #140 A: live-console channels by effect node, taken by whichever owner renders that
    /// node -- the per-node `ConsoleEffect`, or the bank slot that holds the node's lane.
    effect_controls: BTreeMap<crate::EffectNodeId, Box<EffectControlLane>>,
    /// Issue #143 D3: observation lanes by effect node, taken by whichever owner renders that
    /// node. Empty for a plan with no observation capacity, so `node_kind` hands out `None`.
    effect_observations: BTreeMap<crate::EffectNodeId, Box<ObservationLane>>,
    pub(crate) bindings: BTreeMap<GraphNodeId, Option<Box<dyn GraphRuntimeProcessor>>>,
    pub(crate) split_pairs: BTreeMap<GraphNodeId, SplitPairSlot>,
    pub(crate) observers: BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>>,
    pub(crate) source_inputs: std::collections::BTreeSet<GraphNodeId>,
    /// Issue #210 phase 2: declared per-lane input delay, by track input node. Only tracks that
    /// declared a nonzero delay on at least one lane are present, so this map is empty -- and
    /// `node_kind` never leaves the `SourceInput` arm -- for every session that declares none.
    track_delays: BTreeMap<GraphNodeId, [u32; 2]>,
    /// The lines `node_kind` allocated, in the order it allocated them.
    track_delay_lines: Vec<TrackDelayLine>,
    banks: Vec<Option<GraphPreparedEffectBank>>,
    builtin_banks: Vec<Option<GraphPreparedBuiltinBank>>,
    membership: BankMembership,
    /// Prepared owner metadata captured before processors and banks are moved into runtime units.
    /// The runtime retains only this compact lowering map; response capture reads the same binding
    /// rows as execution and never asks a DSP stage for identity metadata.
    response_metadata: BTreeMap<GraphNodeId, (bool, &'static str)>,
    /// Render quantum, so a console-driven effect's staging and shunt are sized once, at bind.
    frames: usize,
}

impl RuntimeParts {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        spec: &GraphSpec,
        routes: Vec<PreparedRoute>,
        effects: Vec<GraphPreparedEffect>,
        effect_controls: Vec<crate::GraphEffectControlBinding>,
        effect_observations: Vec<crate::GraphEffectObservationBinding>,
        banks: Vec<GraphPreparedEffectBank>,
        builtin_banks: Vec<GraphPreparedBuiltinBank>,
        observers: Vec<GraphNodeObserverBinding>,
        bindings: Vec<GraphNodeBinding>,
        source_inputs: std::collections::BTreeSet<GraphNodeId>,
        track_delays: Vec<crate::PreparedTrackDelay>,
        frames: usize,
    ) -> Self {
        #[cfg(feature = "test-support")]
        BANK_CHAIN_FACTS.with(|facts| {
            let mut observed = facts.get();
            observed.prepared_memberships = banks.len() + builtin_banks.len();
            facts.set(observed);
        });
        let membership = bank_membership(spec, &banks, &builtin_banks);
        let mut response_metadata = BTreeMap::new();
        for effect in &effects {
            response_metadata.insert(
                GraphNodeId::Effect(effect.id.clone()),
                (effect.response_snapshot_declared, effect.native_id),
            );
        }
        for bank in &banks {
            for member in &bank.members {
                response_metadata.insert(
                    GraphNodeId::Effect(member.clone()),
                    (bank.response_snapshot_declared, bank.native_id),
                );
            }
        }
        for bank in &builtin_banks {
            let declared = bank.processor.response_snapshot_declared();
            let native_id = bank.processor.response_snapshot_native_id().unwrap_or("");
            for member in &bank.members {
                response_metadata.insert(member.clone(), (declared, native_id));
            }
        }
        for binding in &bindings {
            if let Some(processor) = binding.processor.as_ref() {
                response_metadata.insert(
                    binding.node.clone(),
                    (
                        processor.response_snapshot_declared(),
                        processor.response_snapshot_native_id().unwrap_or(""),
                    ),
                );
            }
        }
        let mut by_node: BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>> = BTreeMap::new();
        for observer in observers {
            by_node
                .entry(observer.node.clone())
                .or_default()
                .push(observer);
        }
        Self {
            routes: routes
                .into_iter()
                .map(|route| (route.node, route.transform))
                .collect(),
            effects: effects
                .into_iter()
                .map(|effect| (GraphNodeId::Effect(effect.id.clone()), effect))
                .collect(),
            effect_controls: effect_controls
                .into_iter()
                .map(|binding| (binding.node, binding.control))
                .collect(),
            effect_observations: effect_observations
                .into_iter()
                .map(|binding| (binding.node, binding.observation))
                .collect(),
            bindings: bindings
                .into_iter()
                .map(|binding| (binding.node, binding.processor))
                .collect(),
            split_pairs: BTreeMap::new(),
            observers: by_node,
            source_inputs,
            track_delays: track_delays
                .into_iter()
                .map(|delay| (delay.node, [delay.left_samples, delay.right_samples]))
                .collect(),
            track_delay_lines: Vec::new(),
            banks: banks.into_iter().map(Some).collect(),
            builtin_banks: builtin_banks.into_iter().map(Some).collect(),
            membership,
            response_metadata,
            frames,
        }
    }

    /// The one node-kind decision, shared by both executors.
    ///
    /// A route's linear gain is folded into its 2x2 coefficients here, once, at bind: render then
    /// spends two multiplies and one add per output word instead of re-applying the
    /// gain every frame (D3, #98 F4).
    fn node_kind(&mut self, node: &GraphNodeId, index: u32) -> NodeKind {
        if self.source_inputs.contains(node) {
            // The delay arm takes this branch's place rather than sitting after it: an input node
            // is a source input whether or not it is delayed, and `execute_op` must early-return
            // for both. A track with no declared delay is absent from the map and lowers to
            // `SourceInput`, byte for byte and op for op, exactly as it did before this feature.
            match self.track_delays.remove(node) {
                None => NodeKind::SourceInput,
                Some([left, right]) => {
                    let line =
                        u32::try_from(self.track_delay_lines.len()).expect("delay line index");
                    self.track_delay_lines
                        .push(TrackDelayLine::new(left as usize, right as usize));
                    NodeKind::TrackDelay {
                        line,
                        channels_agree: left == right,
                    }
                }
            }
        } else if self.membership.contains_key(&index) {
            NodeKind::BankMember
        } else if let Some(Some(processor)) = self.bindings.remove(node) {
            NodeKind::Bound(processor)
        } else if let Some(effect) = self.effects.remove(node) {
            let observation = self.effect_observations.remove(&effect.id);
            match self.effect_controls.remove(&effect.id) {
                // An observation lane is only ever created alongside a control channel -- a
                // subscription rides that queue -- so this arm is the unobserved, console-free
                // path it always was, byte for byte.
                None => NodeKind::Effect(effect),
                Some(control) => NodeKind::ConsoleEffect(Box::new(ConsoleEffect::new(
                    effect,
                    control,
                    observation,
                    self.frames,
                ))),
            }
        } else if let Some(transform) = self.routes.remove(node) {
            NodeKind::Route(folded_route(&transform))
        } else {
            NodeKind::Identity
        }
    }

    /// One chain for a whole cohort run, in slot order (issue #181).
    ///
    /// The scratch and the lane mask come from the run's first slot; every slot of a cohort
    /// covers the same lanes by construction, and `BankChain::new` re-checks it rather than
    /// trusting it.
    fn chain_for_with_fold(
        &mut self,
        run: &[Membership],
        members: usize,
        fold: Option<rack::PreparedFoldConfiguration>,
    ) -> BankChain {
        // Every slot arrives with its own `AoSoaScratch`, because a bound bank is prepared
        // without knowing whether it will end up sharing a chain. One chain has one resident
        // block, so the run keeps the first slot's scratch and drops the rest here, on the
        // control plane at bind. The compile-time estimate (`banks::effect_bank_resource`) still
        // charges one scratch per bound *slot*, so it now over-states what the plan retains by
        // one scratch per merged slot. Over-stating is the safe direction for a memory ceiling --
        // a plan admitted under the estimate always fits -- and it is left that way deliberately:
        // the estimate is computed before the lowered program exists, and whether a merge is
        // admissible is not knowable until it does.
        let mut scratch = None;
        let mut active: Option<Box<[bool]>> = None;
        let mut stages = Vec::with_capacity(run.len());
        let mut index = 0;
        while index < run.len() {
            let pair = if index + 1 < run.len() {
                match (run[index], run[index + 1]) {
                    (Membership::Builtin(a), Membership::Builtin(b)) => {
                        let left = self.builtin_banks[a].as_ref().expect("builtin owner");
                        let right = self.builtin_banks[b].as_ref().expect("builtin owner");
                        let same_tracks = left.members.len() == right.members.len()
                            && left.members.iter().zip(right.members.iter()).all(|(left, right)| {
                                matches!((left, right),
                                    (GraphNodeId::TrackStage { track_id: left_id, stage: TrackStage::PostFader },
                                     GraphNodeId::TrackStage { track_id: right_id, stage: TrackStage::PostMatrix })
                                    if left_id == right_id)
                            });
                        same_tracks
                            && left.backend == right.backend
                            && left.scratch.width() == right.scratch.width()
                            && left.scratch.quantum() == right.scratch.quantum()
                    }
                    _ => false,
                }
            } else {
                false
            };
            if pair {
                let a = match run[index] {
                    Membership::Builtin(i) => self.builtin_banks[i].take().expect("builtin owner"),
                    _ => unreachable!(),
                };
                let b = match run[index + 1] {
                    Membership::Builtin(i) => self.builtin_banks[i].take().expect("builtin owner"),
                    _ => unreachable!(),
                };
                let scratch_a = a.scratch;
                let active_a = trailing_active_mask(members, scratch_a.width());
                let factory = a.processor.pair_factory();
                let (slot_scratch, slot_active, stage) = match factory {
                    Some(factory) => match factory(a.processor, b.processor) {
                        Ok(processor) => (
                            scratch_a,
                            active_a,
                            Box::new(BuiltinStage(processor)) as Box<dyn BankStage>,
                        ),
                        Err((left, right)) => {
                            if scratch.is_none() {
                                scratch = Some(scratch_a);
                                active = Some(active_a);
                            }
                            stages.push(Box::new(BuiltinStage(left)) as Box<dyn BankStage>);
                            stages.push(Box::new(BuiltinStage(right)) as Box<dyn BankStage>);
                            index += 2;
                            continue;
                        }
                    },
                    None => {
                        if scratch.is_none() {
                            scratch = Some(scratch_a);
                            active = Some(active_a);
                        }
                        stages.push(Box::new(BuiltinStage(a.processor)) as Box<dyn BankStage>);
                        stages.push(Box::new(BuiltinStage(b.processor)) as Box<dyn BankStage>);
                        index += 2;
                        continue;
                    }
                };
                if scratch.is_none() {
                    scratch = Some(slot_scratch);
                    active = Some(slot_active);
                }
                stages.push(stage);
                index += 2;
                continue;
            }
            let (slot_scratch, slot_active, stage) = self.stage_for(run[index], members);
            if scratch.is_none() {
                scratch = Some(slot_scratch);
                active = Some(slot_active);
            }
            stages.push(stage);
            index += 1;
        }
        #[cfg(feature = "test-support")]
        BANK_CHAIN_FACTS.with(|facts| {
            let mut observed = facts.get();
            observed.run_memberships += run.len();
            observed.runtime_slots += stages.len();
            observed.maximum_run_memberships = observed.maximum_run_memberships.max(run.len());
            observed.maximum_runtime_slots = observed.maximum_runtime_slots.max(stages.len());
            facts.set(observed);
        });
        bank_chain_with_fold(
            scratch.expect("a unit has at least one slot"),
            active.expect("a unit has at least one slot"),
            stages,
            fold,
        )
    }

    fn stage_for(
        &mut self,
        membership: Membership,
        members: usize,
    ) -> (AoSoaScratch, Box<[bool]>, Box<dyn BankStage>) {
        match membership {
            Membership::Effect(index) => {
                let bank = self.banks[index].take().expect("one effect bank owner");
                let width = bank.scratch.width();
                let quantum = bank.scratch.quantum();
                // Every lane's channel is moved out of its own `GraphPreparedEffect` in lane
                // order, so the slot has exactly one drainer per lane and a lane the console does
                // not address stays `None` -- the bank's own `&[]` for that lane's offsets.
                let controls: Vec<Option<EffectControlLane>> = (0..width.lanes() as usize)
                    .map(|lane| {
                        bank.members
                            .get(lane)
                            .and_then(|member| self.effect_controls.remove(member))
                            .map(|control| *control)
                    })
                    .collect();
                // Issue #143: one observation lane per bank lane, moved out in the same lane order
                // so a slot has exactly one owner per lane and a lane nobody observes stays `None`.
                let observations: Vec<Option<ObservationLane>> = (0..width.lanes() as usize)
                    .map(|lane| {
                        bank.members
                            .get(lane)
                            .and_then(|member| self.effect_observations.remove(member))
                            .map(|observation| *observation)
                    })
                    .collect();
                if controls.iter().any(Option::is_some) {
                    let latency = usize::try_from(bank.processor.metadata().program_key.latency.0)
                        .unwrap_or(usize::MAX);
                    let stage = ConsoleEffectBankStage::new(
                        bank.processor,
                        width,
                        quantum,
                        controls,
                        observations,
                        latency,
                    )
                    .expect("validated width");
                    return (bank.scratch, bank.active_mask, Box::new(stage));
                }
                let stage =
                    EffectBankStage::new(bank.processor, width, quantum).expect("validated width");
                (bank.scratch, bank.active_mask, Box::new(stage))
            }
            Membership::Builtin(index) => {
                let bank = self.builtin_banks[index]
                    .take()
                    .expect("one builtin bank owner");
                let active = trailing_active_mask(members, bank.scratch.width());
                (bank.scratch, active, Box::new(BuiltinStage(bank.processor)))
            }
        }
    }
}

/// Which nodes alias each op's output buffer, in schedule order (`program::Tap`).
fn taps_by_op(program: &ExecutionProgram, spec: &GraphSpec) -> BTreeMap<u32, Vec<GraphNodeId>> {
    let mut by_op: BTreeMap<u32, Vec<GraphNodeId>> = BTreeMap::new();
    for tap in &program.taps {
        by_op
            .entry(tap.after_op)
            .or_default()
            .push(spec.nodes[tap.node as usize].id.clone());
    }
    by_op
}

/// Enumerate the observer rows at one op in the same order as [`build_op`].
///
/// A direct node's rows are followed by each elided alias's rows.  Each individual node is sorted
/// by stable handle by [`take_observers`] and by [`preflight_observation_activation`] before this
/// helper is used, so this function is only responsible for preserving the direct/alias layout.
fn observer_nodes<'a>(
    node: GraphNodeId,
    aliases: &'a [GraphNodeId],
) -> impl Iterator<Item = GraphNodeId> + 'a {
    core::iter::once(node).chain(aliases.iter().cloned())
}

/// The two owners created by borrowed activation preflight.
///
/// Both pieces are prepared while the caller still owns every processor, observer and source.
/// The controller remains on the control side; the realtime half is passed to the executor by the
/// transactional bind tranche that follows this one.
pub(crate) struct PreparedObservationActivation {
    pub(crate) controller: GraphObservationController,
    pub(crate) realtime: RealtimeObservationActivation,
}

/// Resolve the exact lowered observer coordinates before any caller-owned input is moved.
///
/// This is deliberately a borrowed walk over the immutable observer bindings and the already
/// frozen sequential plan.  It must remain the sole fallible activation preparation step: once it
/// succeeds, materialisation can consume the same plan without doing another lookup or allocation
/// that could fail after ownership has started moving.
pub(crate) fn preflight_observation_activation(
    plan: &crate::PreparedGraphPlan,
    program: &ExecutionProgram,
    bindings: &crate::GraphRuntimeBindings,
    planning: &SequentialPlan,
    config: Option<GraphObservationActivationConfig>,
) -> Result<Option<PreparedObservationActivation>, &'static str> {
    let has_controlled = plan
        .observers
        .iter()
        .chain(bindings.observers.iter())
        .any(GraphNodeObserverBinding::is_controlled);

    let Some(config) = config else {
        return if has_controlled {
            Err("graph.plan.observation_activation_required")
        } else {
            Ok(None)
        };
    };

    // A configured endpoint with no controlled row cannot ever admit a replacement.  Refuse it
    // before prepare_activation so a permanent-only graph does not acquire an unusable controller.
    if !has_controlled {
        return Err("graph.plan.observation_activation_capacity");
    }

    if planning.unit_of_run.len() != planning.run_units.len()
        || planning.op_slot.len() != program.ops.len()
    {
        return Err("graph.plan.observer");
    }
    if program.ops.iter().any(|op| {
        usize::try_from(op.node)
            .ok()
            .is_none_or(|node| node >= plan.spec.nodes.len())
    }) || program.taps.iter().any(|tap| {
        usize::try_from(tap.node)
            .ok()
            .is_none_or(|node| node >= plan.spec.nodes.len())
            || usize::try_from(tap.after_op)
                .ok()
                .is_none_or(|op| op >= program.ops.len())
    }) {
        return Err("graph.plan.observer");
    }

    // Keep only borrowed rows in this temporary map.  Clearing one node after it is emitted is
    // equivalent to take_observers' remove, while retaining the caller-owned bindings untouched.
    let mut observers_by_node: BTreeMap<GraphNodeId, Vec<&GraphNodeObserverBinding>> =
        BTreeMap::new();
    for observer in plan.observers.iter().chain(bindings.observers.iter()) {
        observers_by_node
            .entry(observer.node.clone())
            .or_default()
            .push(observer);
    }
    for observers in observers_by_node.values_mut() {
        observers.sort_by_key(|observer| observer.handle);
    }

    let taps = taps_by_op(program, &plan.spec);
    let mut catalog = Vec::new();
    let mut permanent = Vec::new();
    let mut ordinal = 0_usize;
    let mut expected_unit = 0_usize;
    let mut seen_ops = vec![false; program.ops.len()];
    let mut emitted_ops = vec![false; program.ops.len()];

    for (run, (membership, ops)) in planning.run_units.iter().enumerate() {
        let unit = planning
            .unit_of_run
            .get(run)
            .copied()
            .ok_or("graph.plan.observer")?;
        let Some(unit) = unit else {
            // A retired run has no RuntimeUnit and therefore no observer dispatch boundary.  Its
            // observer rows remain in the map and make the final complete-consumption check fail.
            for op in ops {
                let Some(seen) = seen_ops.get_mut(*op) else {
                    return Err("graph.plan.observer");
                };
                if *seen {
                    return Err("graph.plan.observer");
                }
                *seen = true;
                if planning
                    .op_slot
                    .get(*op)
                    .ok_or("graph.plan.observer")?
                    .is_some()
                {
                    return Err("graph.plan.observer");
                }
            }
            continue;
        };
        if unit != expected_unit {
            return Err("graph.plan.observer");
        }
        expected_unit = expected_unit.checked_add(1).ok_or("graph.plan.observer")?;
        if membership.is_empty() && ops.len() != 1 {
            return Err("graph.plan.observer");
        }

        for (member, op) in ops.iter().copied().enumerate() {
            let Some(seen) = seen_ops.get_mut(op) else {
                return Err("graph.plan.observer");
            };
            if *seen {
                return Err("graph.plan.observer");
            }
            *seen = true;
            emitted_ops[op] = true;

            let Some((mapped_unit, mapped_member)) = planning.op_slot.get(op).copied().flatten()
            else {
                return Err("graph.plan.observer");
            };
            if mapped_unit != unit || mapped_member != member {
                return Err("graph.plan.observer");
            }
            let runtime_member = (!membership.is_empty()).then_some(member);
            let node = plan
                .spec
                .nodes
                .get(program.ops[op].node as usize)
                .ok_or("graph.plan.observer")?
                .id
                .clone();
            let aliases = taps
                .get(&u32::try_from(op).map_err(|_| "graph.plan.observer")?)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let mut observer_index = 0_usize;
            for observed_node in observer_nodes(node, aliases) {
                let Some(observers) = observers_by_node.get_mut(&observed_node) else {
                    continue;
                };
                for observer in observers.iter().copied() {
                    let entry = ActivationEntry {
                        unit,
                        member: runtime_member,
                        observer: observer_index,
                        ordinal,
                    };
                    ordinal = ordinal.checked_add(1).ok_or("graph.plan.observer")?;
                    observer_index = observer_index.checked_add(1).ok_or("graph.plan.observer")?;
                    if observer.is_controlled() {
                        catalog.push(ActivationBinding {
                            handle: observer.handle,
                            entry,
                        });
                    } else {
                        permanent.push(entry);
                    }
                }
                // A direct node or alias can appear only once in the lowered sequence.  Clearing
                // here mirrors take_observers if malformed taps repeat the same node.
                observers.clear();
            }
        }
    }

    if seen_ops.iter().any(|seen| !seen)
        || planning
            .op_slot
            .iter()
            .enumerate()
            .any(|(op, slot)| slot.is_some() != emitted_ops[op])
        || observers_by_node
            .values()
            .any(|observers| !observers.is_empty())
    {
        return Err("graph.plan.observer");
    }

    if catalog.is_empty() {
        return Err("graph.plan.observation_activation_capacity");
    }
    let (controller, realtime) = prepare_activation(catalog.into_boxed_slice(), &permanent, config)
        .map_err(observation_activation_error_code)?;
    debug_assert_eq!(controller.resources(), realtime.resources());
    Ok(Some(PreparedObservationActivation {
        controller,
        realtime,
    }))
}

fn observation_activation_error_code(error: GraphObservationAdmissionError) -> &'static str {
    match error {
        GraphObservationAdmissionError::UnknownHandle => {
            "graph.plan.observation_activation_unknown"
        }
        GraphObservationAdmissionError::DuplicateHandle => {
            "graph.plan.observation_activation_duplicate"
        }
        GraphObservationAdmissionError::ActiveCapacity => {
            "graph.plan.observation_activation_capacity"
        }
        GraphObservationAdmissionError::RetainedBytes => {
            "graph.plan.observation_activation_retained"
        }
        GraphObservationAdmissionError::InvalidRemoval => {
            "graph.plan.observation_activation_removal"
        }
        GraphObservationAdmissionError::Backpressure => {
            "graph.plan.observation_activation_backpressure"
        }
        GraphObservationAdmissionError::RevisionExhausted => {
            "graph.plan.observation_activation_revision"
        }
        GraphObservationAdmissionError::OwnerClosed => "graph.plan.observation_activation_owner",
    }
}

/// The buffer-identity half of serialized scalar fader/matrix admission.
///
/// The composite receives one in-place block at the fader slot. It can preserve the later matrix
/// op only when that op's reduction was already a self-copy: one undelayed input, the same input
/// and output buffer as the fader, and the lowering's own `in_place` witness.
fn scalar_pair_is_in_place(program: &ExecutionProgram, fader: usize, matrix: usize) -> bool {
    let fader = &program.ops[fader];
    let matrix = &program.ops[matrix];
    let inputs = program.inputs_of(matrix);
    matches!(inputs, [input] if input.delay.is_none()
        && input.buffer == fader.output
        && matrix.output == fader.output
        && matrix.in_place)
}

/// The deferred fader's physical buffer must stay private until its original matrix operation.
/// Any intervening read, write, delayed staging slot or observer-visible alias declines the split
/// pair, leaving both original owners in the ordinary scalar path.
///
/// The fader's buffer is no longer compared with `program.output` (issue #916). The host reads
/// no arena buffer, and the Output op, being dedicated, never shares a live buffer with the
/// fader. A matching slot only means the colouring gave the Output that slot after the pair
/// retired it: see [`chains_into`]'s session-output bullet.
fn scalar_split_interval_is_clear(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &RuntimeParts,
    taps: &BTreeMap<u32, Vec<GraphNodeId>>,
    fader: usize,
    matrix: usize,
) -> bool {
    let buffer = program.ops[fader].output;
    let fader_node = &spec.nodes[program.ops[fader].node as usize].id;
    if parts.observers.contains_key(fader_node)
        || taps
            .get(&u32::try_from(fader).expect("op index"))
            .is_some_and(|aliases| {
                aliases
                    .iter()
                    .any(|alias| parts.observers.contains_key(alias))
            })
    {
        return false;
    }
    if spec.edges.iter().any(|edge| {
        edge.source.node == *fader_node
            && matches!(
                edge.id,
                GraphEdgeId::RouteSource { .. } | GraphEdgeId::EffectSidechain { .. }
            )
    }) {
        return false;
    }
    program.ops[fader + 1..matrix].iter().all(|op| {
        if op.output == buffer {
            return false;
        }
        if program.inputs_of(op).iter().any(|input| {
            input.buffer == buffer || input.delay.is_some_and(|delay| delay.staging == buffer)
        }) {
            return false;
        }
        if op.sidechain.is_some_and(|side| {
            side.buffer == buffer || side.delay.is_some_and(|delay| delay.staging == buffer)
        }) {
            return false;
        }
        true
    })
}

/// Groups the program's ops into units: a bank's members become one unit at the first member's
/// position, which the level-major schedule proves is after every member's producers (#98 F1).
///
/// Returns, for each unit, the ops it owns and the bank membership it belongs to.
pub(crate) fn units_of(
    program: &ExecutionProgram,
    membership_of: &BankMembership,
) -> Vec<PlannedUnit> {
    let mut units: Vec<PlannedUnit> = Vec::with_capacity(program.ops.len());
    let mut emitted: BTreeMap<usize, usize> = BTreeMap::new();
    let mut op_of_node: BTreeMap<u32, usize> = BTreeMap::new();
    for (index, op) in program.ops.iter().enumerate() {
        op_of_node.insert(op.node, index);
    }
    for (index, op) in program.ops.iter().enumerate() {
        match membership_of.get(&op.node) {
            None => units.push((None, vec![index])),
            Some((membership, _)) => {
                let key = match membership {
                    Membership::Effect(bank) => *bank,
                    Membership::Builtin(bank) => bank + program.ops.len(),
                };
                if emitted.contains_key(&key) {
                    continue;
                }
                emitted.insert(key, units.len());
                let mut members: Vec<(usize, usize)> = program
                    .ops
                    .iter()
                    .enumerate()
                    .filter_map(|(other, candidate)| {
                        membership_of.get(&candidate.node).and_then(
                            |(other_membership, position)| {
                                (*other_membership == *membership).then_some((*position, other))
                            },
                        )
                    })
                    .collect();
                members.sort_unstable();
                units.push((
                    Some(*membership),
                    members.into_iter().map(|(_, op)| op).collect(),
                ));
            }
        }
    }
    units
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum FoldFault {
    MissingRun,
    MissingUnit,
    WrongUnit,
    UnbankedRun,
    MissingBank,
    OversizedMask,
    InactiveLane,
    ActiveWidth,
    MissingMaster,
    BankedMaster,
    WrongMaster,
}

#[cfg(test)]
thread_local! {
    static FOLD_FAULT: std::cell::Cell<Option<FoldFault>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(crate) fn inject_fold_fault(fault: FoldFault) {
    FOLD_FAULT.with(|slot| {
        assert!(slot.replace(Some(fault)).is_none());
    });
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Issue #885's unfolded oracle: the same plan bound with the route fold declined, which is the
    /// path a post-matrix meter forced before that issue. Bind-time only; render never reads it.
    static ROUTE_FOLD_DECLINED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Decline (`true`) or restore (`false`) the route fold for every later bind on this thread.
///
/// The unfolded oracle for a fold test (issue #885): until then a post-matrix meter declined the
/// fold and doubled as that oracle, and it no longer does. Read once per bind, in
/// `preflight_sequential`, before any owner moves; render never reads it, and it does not exist
/// without `test-support`. Callers restore `false` after the bind they meant to decline.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_set_route_fold_declined(declined: bool) {
    ROUTE_FOLD_DECLINED.with(|slot| slot.set(declined));
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Issue #886's unredirected oracle: the same plan bound with every scatter redirect declined,
    /// which is the path an observer of a chain's last slot forced before that issue. Bind-time
    /// only; render never reads it.
    static SCATTER_REDIRECT_DECLINED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Decline (`true`) or restore (`false`) every scatter redirect for every later bind on this
/// thread.
///
/// The unredirected oracle for a direct-scatter test (issue #886), on the pattern of
/// [`test_only_set_route_fold_declined`]: read once per bind, in `build_sequential`, where the
/// redirects are decided and before any unit is built; render never reads it, and it does not exist
/// without `test-support`. Callers restore `false` after the bind they meant to decline.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_set_scatter_redirect_declined(declined: bool) {
    SCATTER_REDIRECT_DECLINED.with(|slot| slot.set(declined));
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Issue #926's unfused oracle: the same plan bound with the Output route fold declined, so
    /// every route op runs and the Output op reduces their outputs. Bind-time only; render never
    /// reads it.
    static OUTPUT_ROUTE_FOLD_DECLINED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Decline (`true`) or restore (`false`) the Output route fold for every later bind on this
/// thread.
///
/// The unfused oracle for an Output-fold test (issue #926), on the pattern of
/// [`test_only_set_route_fold_declined`]: read once per bind, in `preflight_sequential`, before
/// any owner moves; render never reads it, and it does not exist without `test-support`. Callers
/// restore `false` after the bind they meant to decline.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_set_output_route_fold_declined(declined: bool) {
    OUTPUT_ROUTE_FOLD_DECLINED.with(|slot| slot.set(declined));
}

#[cfg(test)]
thread_local! {
    /// Issue #916's arena oracle. Bind-time only; render never reads it.
    static HOST_MASTER_DECLINED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Decline (`true`) or restore (`false`) the host-plane master for every later bind on this
/// thread: issue #916's arena oracle, on the pattern of [`test_only_set_route_fold_declined`].
///
/// A plan bound declined has no Output unit, every folded master is [`FoldTarget::Arena`], and a
/// redirect into the Output op is not withheld. So its Output op and its folds write the Output's
/// arena buffer, as every plan did before the issue, and nothing writes the host's planes. The
/// test copies that buffer out itself, which is the executor's old end-of-block copy. The switch
/// is read once per bind, in `preflight_sequential`. Callers restore `false` after the bind they
/// meant to decline.
#[cfg(test)]
pub(crate) fn test_only_set_host_master_declined(declined: bool) {
    HOST_MASTER_DECLINED.with(|slot| slot.set(declined));
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Issue #918's copy oracle. Bind-time only; render never reads it.
    static SOURCE_IN_PLACE_DECLINED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// `[claims copied into the arena, reads from a played block, reads of silence]`.
    static SOURCE_PLANE_COUNTS: std::cell::Cell<[u64; 3]> = const { std::cell::Cell::new([0; 3]) };
}

/// Decline (`true`) or restore (`false`) the in-place source read for every later bind on this
/// thread: issue #918's copy oracle, on the pattern of [`test_only_set_route_fold_declined`],
/// which is issue #927's too.
///
/// A plan bound declined binds every source claim on the copy, the path every claim took before
/// issue #918: the executor copies each claim's played block into its arena buffer, and every
/// gather and the Output op's fused reduction read the arena. The switch is read once per bind, in `GraphExecutor::new`; render never
/// reads it, and it does not exist without `test-support`. Callers restore `false` after the bind
/// they meant to decline.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_set_source_in_place_declined(declined: bool) {
    SOURCE_IN_PLACE_DECLINED.with(|slot| slot.set(declined));
}

#[cfg(any(test, feature = "test-support"))]
pub(crate) fn test_only_source_in_place_declined() -> bool {
    SOURCE_IN_PLACE_DECLINED.with(std::cell::Cell::get)
}

/// Reset this thread's issue #918 source-plane counts.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_source_plane_reset() {
    SOURCE_PLANE_COUNTS.with(|counts| counts.set([0; 3]));
}

/// `[claims copied into the arena, reads served from a played block, reads served silence]` on
/// this thread since the last reset, where a read is a bank gather's lane (issue #918) or an input
/// of the Output op's fused reduction (issue #927): the mode counter that tells a claim bound in
/// place from one still copied when both render the same bits.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_source_plane_counts() -> [u64; 3] {
    SOURCE_PLANE_COUNTS.with(std::cell::Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
fn test_only_count_source_plane(slot: usize) {
    SOURCE_PLANE_COUNTS.with(|counts| {
        let mut value = counts.get();
        value[slot] += 1;
        counts.set(value);
    });
}

/// One claim copied into the arena by the executor's copy loop.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn test_only_count_source_copy() {
    test_only_count_source_plane(0);
}

/// Prepared before caller-owned processors, observers, banks or sources move. Emission consumes
/// this exact schedule and its owned fold configurations; it never replans route retirement.
pub(crate) struct SequentialPlan {
    run_units: Vec<(Vec<Membership>, Vec<usize>)>,
    fold: Option<RouteFold>,
    unit_of_run: Vec<Option<usize>>,
    op_slot: Vec<Option<(usize, usize)>>,
    installations: Vec<Option<FoldInstallation>>,
    /// The session Output node's op, whose storage is the host's planes (issue #916). Checked by
    /// `validate_fold_installation` to be a plain unit of its own.
    output_op: Option<usize>,
    /// The routes retired into that op's fused reduction, and their table (issue #926).
    output_fold: Option<OutputRouteFold>,
}

struct FoldInstallation {
    configuration: rack::PreparedFoldConfiguration,
    lanes: Box<[FoldLane]>,
    master: FoldTarget,
}

/// The session Output node's op (issue #916): `program::lower`'s `output_node`, the first
/// `GraphNodeId::Output` of the sorted spec, resolved through `node_op`.
///
/// This is the node identity the runtime uses to pick the one op whose storage is the host's
/// planes. It is deliberately not "the op whose output is `program.output`": the Output op's
/// physical slot may have belonged to a buffer that retired before it, as track zero's input
/// slot does on the standing console workloads. `None` if the lowering recorded no op for the
/// node, or an op that does not write `program.output`. Neither can happen for a lowered
/// program, and callers then leave every op on the arena.
fn output_op(program: &ExecutionProgram, spec: &GraphSpec) -> Option<usize> {
    let node = spec
        .nodes
        .iter()
        .position(|node| matches!(node.id, GraphNodeId::Output { .. }))?;
    let op = usize::try_from(program.node_op.get(node).copied().flatten()?).ok()?;
    (program.ops.get(op)?.output == program.output).then_some(op)
}

pub(crate) fn preflight_sequential(
    plan: &crate::PreparedGraphPlan,
    program: &ExecutionProgram,
    bindings: &crate::GraphRuntimeBindings,
    sources: Option<&crate::GraphPreparedSourceSet>,
) -> Result<SequentialPlan, &'static str> {
    let metadata = BorrowedPlanningMetadata {
        plan,
        bindings,
        sources,
        membership: bank_membership(&plan.spec, &plan.banks, &plan.builtin_banks),
    };
    let grouped = units_of(program, metadata.membership());
    let runs = cohort_runs(program, &plan.spec, &metadata, &grouped);
    let run_units: Vec<(Vec<Membership>, Vec<usize>)> = runs
        .iter()
        .map(|run| {
            (
                run.iter().filter_map(|index| grouped[*index].0).collect(),
                run.iter()
                    .flat_map(|index| grouped[*index].1.iter().copied())
                    .collect(),
            )
        })
        .collect();
    let fold = route_fold(program, &plan.spec, &metadata, &run_units);
    #[cfg(any(test, feature = "test-support"))]
    let fold = fold.filter(|_| !ROUTE_FOLD_DECLINED.with(std::cell::Cell::get));
    // Issue #916: the one op whose storage is the host's planes, by node. A lowered program always
    // has it, so its absence is a layout fault and fails the bind before any owner moves.
    let output_op = output_op(program, &plan.spec).ok_or("graph.scheduler.layout")?;
    if output_value_is_read(program, output_op) {
        return Err("graph.scheduler.layout");
    }
    let output_op = Some(output_op);
    #[cfg(test)]
    let output_op = output_op.filter(|_| !HOST_MASTER_DECLINED.with(std::cell::Cell::get));
    // Issue #926: the fused kernel writes the host's planes, so it needs the Output unit. The
    // arena oracle of #916 has none and keeps every route op.
    let output_fold = output_op
        .and_then(|master| output_route_fold(program, &plan.spec, &metadata, &run_units, master));
    #[cfg(any(test, feature = "test-support"))]
    let output_fold =
        output_fold.filter(|_| !OUTPUT_ROUTE_FOLD_DECLINED.with(std::cell::Cell::get));
    validate_fold_installation(plan, program, run_units, fold, output_op, output_fold)
}

/// Whether anything reads the session Output's value out of the arena (issue #916).
///
/// The Output op writes the host's planes, and its arena slot is never written on that path. So a
/// read of the Output's value through the arena would read words no op wrote this block. The
/// graph compiler never emits a consumer of the Output, but `crates/graph` binds hand-built plans,
/// and `PreparedGraphPlan::validate` accepts an edge out of the Output node. Two ways to read it,
/// and each refuses the bind:
///
/// * **The Output op has a reader.** `op_dataflow` counts every main input and sidechain that
///   names its buffer while it is that buffer's last writer, through elided aliases.
/// * **An op after the Output op names its slot**: as an input, a sidechain, a staging slot or an
///   output. The Output is dedicated storage, taken at its op and never freed. So from the
///   Output op onwards, the slot holds the Output's value and nothing else. This is the belt to the
///   first clause's braces.
///
/// Ops *before* the Output op may name the slot, and on the standing console workloads they do.
/// The colouring hands the Output the slot of a buffer that retired before it, and those ops use
/// that earlier buffer.
fn output_value_is_read(program: &ExecutionProgram, output_op: usize) -> bool {
    let (readers, _) = op_dataflow(program);
    !readers.get(output_op).is_none_or(Vec::is_empty)
        || program
            .ops
            .iter()
            .skip(output_op + 1)
            .any(|op| op_names_buffer(program, op, program.output))
}

fn validate_fold_installation(
    plan: &crate::PreparedGraphPlan,
    program: &ExecutionProgram,
    run_units: Vec<(Vec<Membership>, Vec<usize>)>,
    fold: Option<RouteFold>,
    output_op: Option<usize>,
    output_fold: Option<OutputRouteFold>,
) -> Result<SequentialPlan, &'static str> {
    // Issue #926: the Output fold is admitted on a bankless plan only and the chain fold on a
    // banked one, so the two never meet. The table is also only ever the Output op's.
    if output_fold.is_some() && (fold.is_some() || output_op.is_none()) {
        return Err("graph.route_fold.master");
    }
    let retired = fold.as_ref().map(|fold| &fold.retired);
    let output_retired = output_fold.as_ref().map(|fold| &fold.retired);
    let mut unit_of_run = vec![None; run_units.len()];
    let mut op_slot = vec![None; program.ops.len()];
    let mut emitted = 0;
    for (run, (membership, ops)) in run_units.iter().enumerate() {
        if ops.is_empty() {
            return Err("graph.route_fold.mapping");
        }
        if ops.iter().any(|op| {
            retired.is_some_and(|retired| retired.contains(op))
                || output_retired.is_some_and(|retired| retired.contains(op))
        }) {
            if !membership.is_empty() || ops.len() != 1 {
                return Err("graph.route_fold.bank");
            }
            continue;
        }
        unit_of_run[run] = Some(emitted);
        for (member, op) in ops.iter().enumerate() {
            let entry = op_slot.get_mut(*op).ok_or("graph.route_fold.mapping")?;
            if entry.replace((emitted, member)).is_some() {
                return Err("graph.route_fold.mapping");
            }
        }
        emitted += 1;
    }
    #[cfg(test)]
    let fault = FOLD_FAULT.with(std::cell::Cell::take);
    #[cfg(test)]
    let (run_units, fold) = {
        let mut run_units = run_units;
        let mut fold = fold;
        if let Some(fault) = fault {
            let fold = fold
                .as_mut()
                .expect("fault fixture must reach an admitted fold");
            let run = fold.runs[0].0;
            match fault {
                FoldFault::MissingRun => fold.runs[0].0 = run_units.len(),
                FoldFault::MissingUnit => unit_of_run[run] = None,
                FoldFault::WrongUnit => unit_of_run[run] = Some(usize::MAX),
                FoldFault::UnbankedRun => run_units[run].0.clear(),
                FoldFault::MissingBank => run_units[run].0[0] = Membership::Builtin(usize::MAX),
                FoldFault::MissingMaster => op_slot[fold.master_op] = None,
                FoldFault::BankedMaster => {
                    let master = unit_of_run
                        .iter()
                        .position(|unit| *unit == op_slot[fold.master_op].map(|slot| slot.0))
                        .expect("master run");
                    run_units[master].0.push(Membership::Builtin(0));
                }
                FoldFault::WrongMaster => op_slot[fold.master_op] = op_slot[run_units[run].1[0]],
                FoldFault::OversizedMask | FoldFault::InactiveLane | FoldFault::ActiveWidth => {}
            }
        }
        (run_units, fold)
    };
    let mut installations: Vec<Option<FoldInstallation>> =
        (0..run_units.len()).map(|_| None).collect();
    if let Some(fold) = &fold {
        let master_slot = op_slot
            .get(fold.master_op)
            .copied()
            .flatten()
            .ok_or("graph.route_fold.master")?;
        let master_run = unit_of_run
            .iter()
            .position(|unit| *unit == Some(master_slot.0))
            .ok_or("graph.route_fold.master")?;
        let (membership, ops) = &run_units[master_run];
        if !membership.is_empty() || ops.as_slice() != [fold.master_op] || master_slot.1 != 0 {
            return Err("graph.route_fold.master");
        }
        let mut owned = std::collections::BTreeSet::new();
        for (run, lanes) in &fold.runs {
            let (membership, ops) = run_units.get(*run).ok_or("graph.route_fold.mapping")?;
            if unit_of_run.get(*run).copied().flatten().is_none() {
                return Err("graph.route_fold.mapping");
            }
            if ops.iter().enumerate().any(|(member, op)| {
                op_slot.get(*op).copied().flatten() != unit_of_run[*run].map(|unit| (unit, member))
            }) {
                return Err("graph.route_fold.mapping");
            }
            if membership.is_empty()
                || lanes.is_empty()
                || ops.len() != membership.len() * lanes.len()
            {
                return Err("graph.route_fold.bank");
            }
            let mut first_shape = None;
            for bank in membership {
                let (key, width, active, members) = match bank {
                    Membership::Effect(index) => {
                        let bank = plan.banks.get(*index).ok_or("graph.route_fold.bank")?;
                        (
                            (false, *index),
                            bank.scratch.width(),
                            bank.active_mask.clone(),
                            bank.members.len(),
                        )
                    }
                    Membership::Builtin(index) => {
                        let bank = plan
                            .builtin_banks
                            .get(*index)
                            .ok_or("graph.route_fold.bank")?;
                        (
                            (true, *index),
                            bank.scratch.width(),
                            trailing_active_mask(bank.members.len(), bank.scratch.width()),
                            bank.members.len(),
                        )
                    }
                };
                if !owned.insert(key) || members != lanes.len() {
                    return Err("graph.route_fold.bank");
                }
                if first_shape.is_none() {
                    first_shape = Some((width, active));
                }
            }
            let (width, active) = first_shape.ok_or("graph.route_fold.bank")?;
            if lanes.len() > width.lanes() as usize {
                return Err("graph.route_fold.mask");
            }
            let mask = trailing_active_mask(lanes.len(), width);
            #[cfg(test)]
            let (active, mask) = {
                let mut active = active.into_vec();
                let mut mask = mask.into_vec();
                match fault {
                    Some(FoldFault::OversizedMask) => mask.push(true),
                    Some(FoldFault::InactiveLane) => active[0] = false,
                    Some(FoldFault::ActiveWidth) => {
                        active.pop();
                    }
                    _ => {}
                }
                (active.into_boxed_slice(), mask.into_boxed_slice())
            };
            let configuration = rack::PreparedFoldConfiguration::new(width, active, mask)
                .map_err(|_| "graph.route_fold.mask")?;
            let slot = installations
                .get_mut(*run)
                .ok_or("graph.route_fold.mapping")?;
            if slot.is_some() {
                return Err("graph.route_fold.mapping");
            }
            *slot = Some(FoldInstallation {
                configuration,
                lanes: lanes.clone().into_boxed_slice(),
                master: if output_op == Some(fold.master_op) {
                    FoldTarget::Output
                } else {
                    FoldTarget::Arena(fold.master.0 + ARENA_BASE)
                },
            });
        }
    }
    // Issue #916: the Output op must be a plain unit of its own, the one `Runtime::execute` hands
    // the host's planes. Checked last, so a fold fault above keeps its own code.
    if let Some(op) = output_op {
        let (unit, member) = op_slot
            .get(op)
            .copied()
            .flatten()
            .ok_or("graph.scheduler.layout")?;
        let run = unit_of_run
            .iter()
            .position(|emitted| *emitted == Some(unit))
            .ok_or("graph.scheduler.layout")?;
        let (membership, ops) = &run_units[run];
        if member != 0 || !membership.is_empty() || ops.as_slice() != [op] {
            return Err("graph.scheduler.layout");
        }
    }
    Ok(SequentialPlan {
        run_units,
        fold,
        unit_of_run,
        op_slot,
        installations,
        output_op,
        output_fold,
    })
}

/// Builds the sequential executor's runtime: one coloured arena, producers read in place.
///
/// `source_claims` are the source set's claim nodes in claim order when its driver lends its played
/// planes (issue #918), and empty otherwise; [`source_plane_table`] decides which of them a bank
/// gathers in place.
pub(crate) fn build_sequential(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: RuntimeParts,
    frames: usize,
    planning: SequentialPlan,
    observation_activation: Option<RealtimeObservationActivation>,
    source_claims: &[GraphNodeId],
) -> Runtime {
    #[cfg(any(test, feature = "test-support"))]
    test_only_reset_selected_split_fader();
    let mut parts = parts;
    // The arena reserves buffer 0 as the always-zero silence slot, so a coloured buffer `b` is
    // arena buffer `b + ARENA_BASE`.
    let arena = |buffer: u32| buffer + ARENA_BASE;
    let taps = taps_by_op(program, spec);
    let delays = program
        .delays
        .iter()
        .map(|line| CompensationDelay::new(line.samples as usize))
        .collect();
    let SequentialPlan {
        run_units,
        fold,
        unit_of_run,
        op_slot,
        installations,
        output_op,
        output_fold,
    } = planning;
    let folded_runs: std::collections::BTreeSet<usize> = fold
        .as_ref()
        .map(|fold| fold.runs.iter().map(|(run, _)| *run).collect())
        .unwrap_or_default();
    // Both folds' retired routes: a chain fold's (issue #218) and the Output fold's (issue #926).
    // A retired route emits no unit, and every pass below that pairs or redirects ops skips it.
    let mut retired: std::collections::BTreeSet<usize> = fold
        .as_ref()
        .map_or_else(Default::default, |fold| fold.retired.clone());
    let (output_routes, output_producers) =
        output_fold.map_or_else(Default::default, |output_fold| {
            retired.extend(output_fold.retired);
            (output_fold.routes, output_fold.producers)
        });
    // Issue #202 rec 3: decided here, before the scalar pairing passes below, which leave every
    // redirect consumer alone. Since issue #886 no clause asks about observers: an observer of the
    // last slot reads the redirected buffer (see `scatter_target`).
    //
    // A folded chain is excluded: the redirect points a lane's scatter at its consumer's buffer,
    // and a folded lane has no scatter to point anywhere -- its tile goes to the epilogue and its
    // consumer no longer runs. Excluding it keeps the two counters honest as well as the code:
    // `bank_scatter_redirects` reports the lanes that still relocate a scatter, not the lanes the
    // fold made the question moot for.
    //
    // A redirect whose consumer is the session Output op is excluded too (issue #916). The redirect
    // would scatter into the Output's arena buffer and turn its reduction into the no-op
    // `[own output]` read, but the Output's storage is the host's planes, so the lane would never
    // reach them. Declined, the chain scatters into its own last slot and the Output op copies that
    // into the host's planes. That is one block copy, the one the end-of-block copy used to make on
    // the redirected path. No compiled session reaches this shape: a strip's last slot feeds its
    // fader, and a route stands between every track and the output.
    let redirects: Vec<ScatterRedirect> = scatter_redirects(program, &parts.membership, &run_units)
        .into_iter()
        .filter(|(run, _, _)| !folded_runs.contains(run))
        .filter(|(_, _, consumer)| Some(*consumer) != output_op)
        .collect();
    #[cfg(any(test, feature = "test-support"))]
    let redirects = if SCATTER_REDIRECT_DECLINED.with(std::cell::Cell::get) {
        Vec::new()
    } else {
        redirects
    };
    let mut split_pairs: Vec<Box<dyn GraphRuntimeSplitPairProcessor>> = Vec::new();

    // Serialized scalar fader/matrix pairing is decided while both original owners and the
    // lowered graph are still available.  The schedule is intentionally left untouched: the
    // matrix binding becomes an identity at its original slot, while the composite runs from the
    // fader slot and the existing reduction/observer boundaries remain in place.
    let (readers, first_producer) = op_dataflow(program);
    for pair in run_units.windows(2) {
        let (first_membership, first_ops) = &pair[0];
        let (second_membership, second_ops) = &pair[1];
        if !first_membership.is_empty()
            || !second_membership.is_empty()
            || first_ops.len() != 1
            || second_ops.len() != 1
        {
            continue;
        }
        let first = first_ops[0];
        let second = second_ops[0];
        if retired.contains(&first)
            || retired.contains(&second)
            || second != first.saturating_add(1)
            || program.inputs_of(&program.ops[first]).is_empty()
            || !scalar_pair_is_in_place(program, first, second)
        {
            continue;
        }
        let first_node = &spec.nodes[program.ops[first].node as usize].id;
        let second_node = &spec.nodes[program.ops[second].node as usize].id;
        let (
            GraphNodeId::TrackStage {
                track_id: first_track,
                stage: TrackStage::PostFader,
            },
            GraphNodeId::TrackStage {
                track_id: second_track,
                stage: TrackStage::PostMatrix,
            },
        ) = (first_node, second_node)
        else {
            continue;
        };
        let crossing_reader = spec.edges.iter().any(|edge| {
            edge.source.node == *first_node
                && matches!(
                    edge.id,
                    GraphEdgeId::RouteSource { .. } | GraphEdgeId::EffectSidechain { .. }
                )
        });
        if crossing_reader
            || first_track != second_track
            || !chains_into(
                program,
                spec,
                &parts,
                &readers,
                &first_producer,
                &[first],
                &[second],
            )
        {
            continue;
        }
        let Some(Some(fader)) = parts.bindings.remove(first_node) else {
            continue;
        };
        let Some(Some(matrix)) = parts.bindings.remove(second_node) else {
            parts.bindings.insert(first_node.clone(), Some(fader));
            continue;
        };
        let Some(factory) = fader.scalar_pair_factory() else {
            parts.bindings.insert(first_node.clone(), Some(fader));
            parts.bindings.insert(second_node.clone(), Some(matrix));
            continue;
        };
        match factory(fader, matrix) {
            Ok(composite) => {
                parts.bindings.insert(first_node.clone(), Some(composite));
                parts.bindings.insert(second_node.clone(), None);
            }
            Err((fader, matrix)) => {
                parts.bindings.insert(first_node.clone(), Some(fader));
                parts.bindings.insert(second_node.clone(), Some(matrix));
            }
        }
    }

    // Keep the nonadjacent pair at both original schedule positions. A settled fader defers only
    // its private in-place arithmetic; the owner completes it at the matrix slot or before an
    // intervening execution/observer error escapes the render call. The first slice admits one
    // deterministic interval; all later candidates remain on the original separate path.
    'split: for fader_run in 0..run_units.len() {
        let (fader_membership, fader_ops) = &run_units[fader_run];
        if !fader_membership.is_empty() || fader_ops.len() != 1 {
            continue;
        }
        let fader = fader_ops[0];
        for (matrix_membership, matrix_ops) in run_units.iter().skip(fader_run + 2) {
            if !matrix_membership.is_empty() || matrix_ops.len() != 1 {
                continue;
            }
            let matrix = matrix_ops[0];
            if retired.contains(&fader)
                || retired.contains(&matrix)
                || program.inputs_of(&program.ops[fader]).is_empty()
                || !scalar_pair_is_in_place(program, fader, matrix)
                || redirects
                    .iter()
                    .any(|(_, _, consumer)| *consumer == fader || *consumer == matrix)
                || !chains_into(
                    program,
                    spec,
                    &parts,
                    &readers,
                    &first_producer,
                    &[fader],
                    &[matrix],
                )
                || !scalar_split_interval_is_clear(program, spec, &parts, &taps, fader, matrix)
            {
                continue;
            }
            let fader_node = &spec.nodes[program.ops[fader].node as usize].id;
            let matrix_node = &spec.nodes[program.ops[matrix].node as usize].id;
            let (
                GraphNodeId::TrackStage {
                    track_id: fader_track,
                    stage: TrackStage::PostFader,
                },
                GraphNodeId::TrackStage {
                    track_id: matrix_track,
                    stage: TrackStage::PostMatrix,
                },
            ) = (fader_node, matrix_node)
            else {
                continue;
            };
            if fader_track != matrix_track {
                continue;
            }
            let Some(Some(fader_owner)) = parts.bindings.remove(fader_node) else {
                continue;
            };
            let Some(Some(matrix_owner)) = parts.bindings.remove(matrix_node) else {
                parts.bindings.insert(fader_node.clone(), Some(fader_owner));
                continue;
            };
            let Some(factory) = fader_owner.scalar_split_pair_factory() else {
                parts.bindings.insert(fader_node.clone(), Some(fader_owner));
                parts
                    .bindings
                    .insert(matrix_node.clone(), Some(matrix_owner));
                continue;
            };
            match factory(fader_owner, matrix_owner) {
                Ok(owner) => {
                    #[cfg(any(test, feature = "test-support"))]
                    test_only_record_selected_split_fader(
                        fader_node.clone(),
                        program.ops[fader].output.0,
                    );
                    let pair = split_pairs.len();
                    split_pairs.push(owner);
                    parts.split_pairs.insert(
                        fader_node.clone(),
                        SplitPairSlot {
                            pair,
                            role: SplitPairRole::Fader,
                        },
                    );
                    parts.split_pairs.insert(
                        matrix_node.clone(),
                        SplitPairSlot {
                            pair,
                            role: SplitPairRole::Matrix,
                        },
                    );
                    break 'split;
                }
                Err((fader_owner, matrix_owner)) => {
                    parts.bindings.insert(fader_node.clone(), Some(fader_owner));
                    parts
                        .bindings
                        .insert(matrix_node.clone(), Some(matrix_owner));
                }
            }
        }
    }
    let mut units = Vec::with_capacity(run_units.len());
    // The bind-time half of the collapse-eligibility query, one row per emitted unit. Built here
    // rather than by a later walk because this is the only place the unit's ops and the spec's
    // node ids are both in hand: `RuntimeOp` deliberately carries no node id, and reconstructing
    // one from the arena buffers afterwards would be a second opinion about which lane is which.
    let mut identity: Vec<UnitIdentity> = Vec::with_capacity(run_units.len());
    for ((membership, ops), installation) in run_units.iter().zip(installations) {
        // A retired route op is absorbed by its cohort's epilogue, or by the Output op's fused
        // reduction: no unit, no dispatch, no reduction, no `mix2x2_block` pass of its own.
        if ops.iter().all(|index| retired.contains(index)) {
            continue;
        }
        let membership = membership.clone();
        {
            // `ops` is slot major with `lanes` ops per slot (`units_of` sorts by the member's
            // position within its bank), so slot `s`'s lane `l` is `ops[s * lanes + l]` and lane
            // `l`'s track is read off the first slot. A plain op is one stage over one "lane".
            let stages = membership.len().max(1);
            let lanes = ops.len() / stages;
            let node_of = |index: usize| &spec.nodes[program.ops[index].node as usize].id;
            identity.push(UnitIdentity {
                banked: !membership.is_empty(),
                resident_input: false,
                // Derived from the finished unit by the runtime constructor.
                observed: false,
                source_lanes: 0,
                stages: u32::try_from(stages).unwrap_or(u32::MAX),
                upstream_of_seam_stages: u32::try_from(
                    (0..stages)
                        .filter(|slot| upstream_of_seam(node_of(ops[slot * lanes])))
                        .count(),
                )
                .unwrap_or(u32::MAX),
                lane_tracks: (0..lanes)
                    .map(|lane| node_track(node_of(ops[lane])))
                    .collect(),
            });
        }
        let members: Vec<RuntimeOp> = ops
            .iter()
            .map(|index| {
                let op = &program.ops[*index];
                let mut inputs = Vec::with_capacity(op.input_count() as usize);
                let mut staged = Vec::new();
                for input in program.inputs_of(op) {
                    match input.delay {
                        None => inputs.push(arena(input.buffer.0)),
                        Some(delay) => {
                            inputs.push(arena(delay.staging.0));
                            staged.push(StagedInput {
                                source: arena(input.buffer.0),
                                staging: arena(delay.staging.0),
                                line: delay.line,
                            });
                        }
                    }
                }
                if fold.as_ref().is_some_and(|fold| fold.master_op == *index) {
                    inputs = vec![arena(op.output.0)];
                }
                let sidechain = op.sidechain.map(|side| match side.delay {
                    None => arena(side.buffer.0),
                    Some(delay) => {
                        staged.push(StagedInput {
                            source: arena(side.buffer.0),
                            staging: arena(delay.staging.0),
                            line: delay.line,
                        });
                        arena(delay.staging.0)
                    }
                });
                build_op(
                    &mut parts,
                    spec,
                    op,
                    inputs,
                    staged,
                    sidechain,
                    arena(op.output.0),
                    taps.get(&u32::try_from(*index).expect("op index"))
                        .map(Vec::as_slice),
                )
            })
            .collect();
        units.push(finish_unit(&mut parts, &membership, members, installation));
    }
    apply_scatter_redirects(program, &redirects, &unit_of_run, &op_slot, &mut units);
    let folds = fold.as_ref().map_or(0, |fold| {
        fold.runs.iter().map(|(_, lanes)| lanes.len() as u64).sum()
    });
    arm_resident_inputs(
        program,
        &first_producer,
        &run_units,
        &unit_of_run,
        &units,
        &mut identity,
    );
    let response_bindings = response_owner_bindings(
        program,
        spec,
        &op_slot,
        &units,
        &parts.response_metadata,
        &retired,
    );
    let mut builder = ArenaLeaseSetBuilder::new(
        NonZeroUsize::new(2).expect("stereo planes"),
        NonZeroUsize::new(frames.max(1)).expect("nonzero frames"),
    );
    let buffers: Vec<u32> = (0..program.buffers).map(|_| builder.reserve()).collect();
    builder.lease(0, buffers.clone(), buffers);
    let (_arena, mut leases) = builder
        .finish()
        .expect("one lease over one coloured arena is disjoint by construction");
    // Issue #916: the unit whose op writes the host's planes, found by node.
    // `validate_fold_installation` proved it a plain unit of its own.
    let output_unit = output_op
        .and_then(|op| op_slot.get(op).copied().flatten())
        .map(|(unit, _)| unit);
    // Issue #918: decided on the finished units, after every redirect has repointed its member.
    // Issue #927: and on the Output fold's retired routes, whose inputs the fused reduction reads.
    let SourcePlanes {
        of_buffer: source_plane_of_buffer,
        output: output_sources,
    } = source_plane_table(
        program,
        spec,
        &units,
        &op_slot,
        &readers,
        source_claims,
        &output_producers,
        &mut identity,
    );
    let mut runtime = Runtime::new_with_observation_activation(
        leases.pop().expect("the sequential lease"),
        delays,
        // Allocated by `node_kind` as it lowered each delayed input node, so the line indices the
        // ops carry and this vector's order are the same walk.
        core::mem::take(&mut parts.track_delay_lines),
        units,
        split_pairs,
        identity,
        response_bindings,
        redirects.len() as u64,
        folds,
        observation_activation,
        output_unit,
        output_routes,
    );
    runtime.source_plane_of_buffer = source_plane_of_buffer;
    runtime.output_sources = output_sources;
    runtime
}

/// Whether first-slot bank member `member` reads arena buffer `buffer` and nothing else, and does
/// not write it before its chain gathers it (issue #918, [`source_plane_table`] clause (b)).
///
/// A [`NodeKind::BankMember`] op's whole body is its reduction, and one undelayed, unmixed input
/// makes that reduction a copy of `buffer` into the member's output, or nothing when the output is
/// `buffer` itself. `Runtime::execute` then gathers `buffer` either way: through
/// [`bank_gather_source`] when the member owns a buffer of its own (the dedication copy, skipped),
/// and through the member's own output when that output *is* `buffer`. The second happens when the
/// member runs in place over its input, or when a scatter redirect repointed a single-slot chain's
/// member at a consumer that took `buffer`'s slot after the input retired: its reduction is then
/// the `[own output]` no-op, and the chain's scatter writes `buffer` only after the gather.
fn gathers_only(member: &RuntimeOp, buffer: u32) -> bool {
    matches!(member.kind, NodeKind::BankMember)
        && member.staged.is_empty()
        && member.sidechain.is_none()
        && *member.inputs == [buffer]
}

/// What [`source_plane_table`] bound in place, as the two tables the runtime keeps.
struct SourcePlanes {
    /// [`Runtime`]'s `source_plane_of_buffer`: arena buffer -> the claim bound in place there, or
    /// [`NO_SOURCE_CLAIM`]. The executor's copy loop skips exactly the claims it names
    /// ([`Runtime::source_in_place`]), and a bank's marked lane looks its buffer up in it.
    of_buffer: Box<[u32]>,
    /// [`Runtime`]'s `output_sources` (issue #927): one entry per input of the session Output
    /// op's fused reduction, in its edge order, naming the claim that input reads from the played
    /// block, or [`NO_SOURCE_CLAIM`] for an input read from the arena. Empty when no input is.
    output: Box<[u32]>,
}

/// Which source claims are read **in place** from the played transfer block: by a bank's gather
/// (issue #918), or by the session Output op's fused reduction (issue #927). Every other claim
/// keeps the copy.
///
/// A claim is bound in place, and the executor's copy loop skips it, only when nothing but one of
/// those two reads could ever read the words that copy writes. Each clause below is one way the
/// copy is still needed, and the claim keeps it:
///
/// * **(a) The input op is a plain [`NodeKind::SourceInput`].** A delayed input
///   ([`NodeKind::TrackDelay`]) runs its delay line in place over the copied words, so the arena
///   buffer holds the aligned block rather than the played one, and a gather must read it there.
///   The only other op that could write the input's value is an in-place consumer, and (b) admits
///   one only when it is a bank member whose reduction is the `[own output]` no-op: it writes the
///   buffer with its chain's scatter, after its own gather has read the input. (b') admits one
///   only when it is a retired route, which does not run at all.
/// * **(b) Every reader is a bank gather of exactly this buffer.** `op_dataflow`'s readers of the
///   input op are every op that reads its value: main inputs and sidechains, through elided aliases,
///   delayed or not. Each must be a first-slot member of a bank unit that reads this buffer and
///   nothing else and writes nothing before its chain gathers it ([`gathers_only`]), because
///   [`ArenaMembers::plane`] is the only bank read that consults the table. A route, a submix, a
///   dynamic-rack or bound processor, the Output op, any other in-place consumer, a delayed (staged)
///   or sidechain read all read the arena buffer itself, so they need the copy, unless (b') admits
///   the reader. A claim with no reader at all is bound in place too: nothing reads the copy.
/// * **(b') Or its one reader is a route the Output fold retired (issue #927).** `readers` is
///   exactly `[route]`, and `route` is `output_producers[i]`: the retired route that feeds input
///   `i` of the session Output op ([`output_route_fold`]). That fold admitted `route` only as a
///   plain route running in place over its one undelayed input, which is this claim's value, so
///   the Output op's input `i` is this buffer; read by the Output op alone; unobserved, the aliases
///   after it included; and with no unit between it and the Output op naming the buffer. Retired,
///   `route` never runs, so the fused reduction ([`route_reduce`]) is the buffer's only read, and
///   it reads input `i` from the played block instead ([`OutputSources`]). A claim whose route has
///   a second reader beside it -- a send, a sidechain -- cannot be here: two readers keep the route
///   from running in place, and the fold declines outright. Any other reader -- a submix, a bound
///   stage, a delayed (staged) edge, a route that runs, or two of them -- reads the arena, so the
///   claim keeps the copy.
/// * **(c) No observer at the input stage.** An observer bound to the input node, or to an elided
///   alias of its buffer, is dispatched after the input op and reads `lease.read_stereo(op.output)`:
///   the copied words.
/// * **(d) The driver lends its planes.** `source_claims` is empty unless
///   [`crate::GraphPreparedSourceSetDriver::provides_played_planes`] is `true`.
/// * **(e) Under (b'), nothing scheduled before the input op names its buffer.** The copy fills the
///   buffer before the first unit, but the input op, a no-op, sits at its own place in the
///   schedule, and the colouring may give its slot to a value that dies before it. That value's
///   producer overwrites the copied words, and the Output op then reads what it wrote. The fused
///   reduction must read exactly the words the copy leaves there, so such a claim keeps the copy.
///   The graph compiler puts every input in the first dependency level, ahead of every other node
///   of that level by node order, so this never fires on a compiled plan; a hand-built plan need
///   not. (b) has the same exposure and no such clause: it is issue #918's, which this issue does
///   not change.
///
/// The table is keyed by the physical arena buffer, and the colouring hands a retired input's slot
/// to later ops (a route, the Output, another cohort's member, which a later bank may gather; and a
/// claim with no reader, which (b) binds in place, frees its slot at once). So the key alone does
/// not identify the claim's value, and the table is never consulted by key alone. Each reader
/// gather of (b) marks its lane in its unit's [`UnitIdentity::source_lanes`], and
/// [`ArenaMembers::plane`] looks a buffer up only for a marked lane; every other gather of the same
/// slot reads the arena, as it always did. The Output op of (b') never looks a buffer up at all:
/// it reads input `i` from the claim `output[i]` names, and the table's entry for that claim is
/// only what makes the copy loop skip it.
///
/// Why the in-place words are the copy's: `op_dataflow`'s readers are every read of the claim's
/// value, and by (b) or (b') they are the marked gathers or the fused reduction. Under the copy the
/// played block is in the buffer from before the first unit through the input op (under (b'), by
/// (e)), and from the input op to the value's last reader the colouring gives the slot to nothing
/// else, while nothing that reads the value writes it before those reads: the input op is a no-op
/// (a), each gather's reduction only reads it (b), and the retired route does not run (b'). So a
/// marked gather or the fused reduction reads the same words from the block, and on an unplayed
/// quantum the silence buffer's `+0.0`, which is what the copy fills it with. A later read of the
/// slot is a read of another value -- the reader's own scatter, or an op the slot was recoloured
/// to -- and depends on the copied words no more than it did under the copy.
#[expect(
    clippy::too_many_arguments,
    reason = "the bind-time facts the table is decided on stay explicit parameters"
)]
fn source_plane_table(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    units: &[RuntimeUnit],
    op_slot: &[Option<(usize, usize)>],
    readers: &[Vec<usize>],
    claims: &[GraphNodeId],
    output_producers: &[usize],
    identity: &mut [UnitIdentity],
) -> SourcePlanes {
    // The first-slot lane `(unit, lane)` an op runs as, and its bank's members. A lane beyond the
    // eighth could not be marked in `UnitIdentity::source_lanes`; no bank is that wide.
    let first_slot = |op: usize| -> Option<(usize, usize, &RuntimeOp)> {
        let (unit, member) = op_slot.get(op).copied().flatten()?;
        match units.get(unit)? {
            RuntimeUnit::Bank { members, lanes, .. } if member < *lanes && member < 8 => {
                Some((unit, member, members.get(member)?))
            }
            _ => None,
        }
    };
    // `(claim, buffer)` of every claim clauses (a) to (e) admit, and the `(unit, lane)` of every
    // reader gather of those claims.
    let mut admitted: Vec<(u32, u32)> = Vec::new();
    let mut marked: Vec<(usize, usize)> = Vec::new();
    let mut output = vec![NO_SOURCE_CLAIM; output_producers.len()];
    let mut output_reads = false;
    for (claim, node) in claims.iter().enumerate() {
        let Ok(claim) = u32::try_from(claim) else {
            continue;
        };
        let Some(op) = crate::program::node_index(spec, node)
            .and_then(|index| program.node_op.get(index as usize).copied().flatten())
            .map(|op| op as usize)
        else {
            continue;
        };
        let Some(buffer) = program.ops.get(op).map(|op| op.output.0 + ARENA_BASE) else {
            continue;
        };
        // (a) and (c).
        let plain = op_slot
            .get(op)
            .copied()
            .flatten()
            .and_then(|(unit, _)| units.get(unit))
            .is_some_and(|unit| {
                matches!(unit, RuntimeUnit::Op(input)
                    if matches!(input.kind, NodeKind::SourceInput) && input.observers.is_empty())
            });
        if !plain || claim == NO_SOURCE_CLAIM {
            continue;
        }
        // (b).
        let mut gathers = Vec::with_capacity(readers[op].len());
        for reader in &readers[op] {
            match first_slot(*reader) {
                Some((unit, lane, member)) if gathers_only(member, buffer) => {
                    gathers.push((unit, lane));
                }
                _ => break,
            }
        }
        if gathers.len() == readers[op].len() {
            admitted.push((claim, buffer));
            marked.extend(gathers);
            continue;
        }
        // (b') and (e).
        if let [reader] = readers[op].as_slice()
            && let Some(input) = output_producers.iter().position(|route| route == reader)
            && !program.ops[..op]
                .iter()
                .any(|earlier| op_names_buffer(program, earlier, program.ops[op].output))
        {
            admitted.push((claim, buffer));
            output[input] = claim;
            output_reads = true;
        }
    }
    // Mark each reader gather's lane, so only these lanes ever consult the table.
    let Some(len) = admitted
        .iter()
        .map(|(_, buffer)| *buffer as usize + 1)
        .max()
    else {
        return SourcePlanes {
            of_buffer: Box::default(),
            output: Box::default(),
        };
    };
    let mut table = vec![NO_SOURCE_CLAIM; len];
    for (claim, buffer) in admitted {
        table[buffer as usize] = claim;
    }
    for (unit, lane) in marked {
        identity[unit].source_lanes |= 1 << lane;
    }
    SourcePlanes {
        of_buffer: table.into_boxed_slice(),
        output: if output_reads {
            output.into_boxed_slice()
        } else {
            Box::default()
        },
    }
}

fn response_owner_bindings(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    op_slot: &[Option<(usize, usize)>],
    units: &[RuntimeUnit],
    response_metadata: &BTreeMap<GraphNodeId, (bool, &'static str)>,
    retired: &std::collections::BTreeSet<usize>,
) -> Vec<ResponseOwnerBinding> {
    let mut next_slot = BTreeMap::<String, u32>::new();
    program
        .ops
        .iter()
        .enumerate()
        .filter_map(|(op_index, op)| {
            if retired.contains(&op_index) {
                return None;
            }
            let node = &spec.nodes[op.node as usize].id;
            let (track_id, stable_id, rack) = match node {
                GraphNodeId::TrackStage {
                    track_id,
                    stage: TrackStage::PostInputBuiltins,
                } => (track_id.as_str(), "input-filters", 0),
                GraphNodeId::Effect(effect) => (
                    effect.track_id.as_str(),
                    effect.effect_id.as_str(),
                    effect.rack as u8,
                ),
                _ => return None,
            };
            let (unit, member) = op_slot.get(op_index).copied().flatten()?;
            let runtime_member = match units.get(unit)? {
                RuntimeUnit::Op(_) => 0,
                RuntimeUnit::Bank { lanes, members, .. } => {
                    if *lanes == 0 || member >= members.len() {
                        return None;
                    }
                    member
                }
            };
            let (response_snapshot_declared, native_id) =
                response_metadata.get(node).copied().unwrap_or((false, ""));
            let slot = next_slot.entry(track_id.to_owned()).or_default();
            let signal_slot = *slot;
            *slot = slot.saturating_add(1);
            Some(ResponseOwnerBinding {
                track_id: Box::from(track_id),
                native_id: Box::from(native_id),
                stable_id: Box::from(stable_id),
                response_snapshot_declared,
                rack,
                slot: signal_slot,
                unit,
                member: runtime_member,
            })
        })
        .collect()
}

/// The admission walk uses emitted adjacency: a retired run owns no execution boundary.
fn arm_resident_inputs(
    program: &ExecutionProgram,
    first_producer: &[Option<usize>],
    run_units: &[(Vec<Membership>, Vec<usize>)],
    unit_of_run: &[Option<usize>],
    units: &[RuntimeUnit],
    identity: &mut [UnitIdentity],
) {
    // Work on final emitted adjacency, after redirects and folded epilogues. No new allocation:
    // the sole retained bit occupies UnitIdentity padding; all other proof inputs already exist.
    let mut previous: Option<(usize, &[usize])> = None;
    for (run, (_, ops)) in run_units.iter().enumerate() {
        let Some(unit) = unit_of_run[run] else {
            continue;
        };
        if let Some((before, earlier)) = previous {
            let lanes = identity[unit].lane_tracks.len();
            identity[unit].resident_input = resident_units_match(&units[before], &units[unit])
                && identity[before].lane_tracks == identity[unit].lane_tracks
                && earlier[earlier.len() - lanes..]
                    .iter()
                    .zip(&ops[..lanes])
                    .all(|(producer, consumer)| {
                        let op = &program.ops[*consumer];
                        op.input_count() == 1
                            && op.sidechain.is_none()
                            && program.inputs_of(op)[0].delay.is_none()
                            && first_producer[*consumer] == Some(*producer)
                    });
        }
        previous = Some((unit, ops));
    }
}

/// The separate resident proof deliberately permits extra readers and every observer. Their
/// planar scatter and schedule boundaries remain intact. Equal validated bank widths also prove
/// backend identity by BankWidth::for_backend; scalar units cannot enter this path.
fn resident_units_match(before: &RuntimeUnit, after: &RuntimeUnit) -> bool {
    let (
        RuntimeUnit::Bank {
            members: a,
            lanes: a_lanes,
            chain: a_chain,
            fold,
            ..
        },
        RuntimeUnit::Bank {
            members: b,
            lanes: b_lanes,
            chain: b_chain,
            ..
        },
    ) = (before, after)
    else {
        return false;
    };
    a_lanes == b_lanes
        && *a_lanes > 0
        && a_chain.width() == b_chain.width()
        && a_chain.quantum() == b_chain.quantum()
        && a_chain.active() == b_chain.active()
        && fold.is_empty()
        && a_chain.fold_lanes().is_empty()
        && a_chain.aux_lanes().is_empty()
        && a[a.len() - a_lanes..]
            .iter()
            .zip(&b[..*b_lanes])
            .all(|(a, b)| {
                matches!(b.kind, NodeKind::BankMember)
                    && b.staged.is_empty()
                    && b.sidechain.is_none()
                    && b.inputs.as_ref() == [a.output]
            })
}

/// Arm every admitted chain's epilogue and neutralise the reduction it performed.
///
/// One chain's scatter redirect: `(run, lane, consumer op)` for every lane whose scatter may land
/// in its consumer's buffer instead of the last slot's own (issue #202 rec 3).
type ScatterRedirect = (usize, usize, usize);

/// The buffers a chain's scatter may land in **instead of** the last slot's own outputs.
///
/// The mirror image of [`bank_gather_source`], on the other side of the round trip. The gather
/// side removed the copy a *member's* reduction made into a dedicated bank buffer; this removes
/// the copy the *consumer's* reduction makes out of one. On the 64-track intended strip that copy
/// is `reduce_plane` memcpying a whole stereo block from the limiter's dedicated buffer into the
/// fader op, once per track per block -- 64 stereo block copies that exist only because
/// `program::is_dedicated` refuses the in-place fold (`program::lower`, the `in_place` clause).
///
/// The redirect makes the consumer behave exactly as an `in_place` op would: the chain scatters
/// straight into the consumer's buffer, and the consumer's reduction becomes the no-op
/// `reduce_plane` already performs for a single input that is its own output. Nothing else about
/// the consumer changes -- it still runs, in its own unit, at its own position.
///
/// This does **not** touch `program::is_dedicated`. Dedication *by bank membership* was measured
/// and rejected (`program::lower` records why, #169), and this is not that: `is_dedicated` stays
/// exactly the classification by node kind it has always been, and what moves is where one chain
/// scatters. It is likewise not the #194 "scatter straight into the planes" null, which was about
/// staging the transpose itself and not about which buffer the scatter targets.
///
/// `None` on any doubt. Every clause is one way the redirect could be observed:
///
/// * **Sole readership.** The last slot's output must have exactly one reader, and that reader is
///   the consumer. A second reader -- a send, a meter, a sidechain source, all of which
///   `op_dataflow` counts -- would read a buffer the scatter no longer fills.
/// * **The consumer's reduction is a pure copy.** One main input, no sidechain, no
///   compensation-delay staging, and that input is the last slot's output. Two inputs is a sum and
///   zero is a `fill(0.0)`; either way the consumer's buffer is not simply the chain's output, and
///   neutralising its reduction would drop a summand. A delayed input owns a line that must still
///   be pumped.
/// * **Not already in place.** `consumer.output == producer.output` is the lowering having elided
///   the copy already, and redirecting the scatter would change nothing.
/// * **The consumer is not a bank member.** A banked consumer's own gather may already read the
///   producer's buffer directly ([`bank_gather_source`]), and redirecting the scatter away from it
///   would hand that gather the previous block's words. The two redirects are each other's only
///   incompatibility, so this is where they are kept apart.
/// * **Not the session output.** A producer whose buffer is `program.output` is declined. Since
///   issue #916 this is a slot comparison and nothing more. The Output is dedicated storage and
///   never in place, so no producer shares its *live* buffer. The clause can still fire when the
///   colouring hands the Output a producer's slot after that producer retired. That declines a
///   redirect that would have been sound. [`chains_into`] and `scalar_split_interval_is_clear`
///   dropped the same comparison in #916. This one is kept because the program-level model
///   mirrors it and the corpus drives the two against each other through
///   [`scatter_redirects_over_program`], so restating it is a change of its own. The consumer side
///   -- a redirect *into* the Output op, whose storage is the host's planes -- is declined in
///   `build_sequential`, outside this predicate.
/// * **Nothing between the scatter and the consumer names the consumer's buffer.** This is the one
///   clause with no counterpart on the gather side, and it is the load-bearing one. The scatter now
///   writes the consumer's buffer at the *chain's* position, which is earlier -- often much
///   earlier -- than the consumer's own op, and the colouring only owns that physical slot from the
///   consumer's op onwards. Whatever held the slot before then is still live for the ops in
///   between. So every op in `[chain's first op, consumer's op)` that is not part of the chain is
///   checked, and any mention of the buffer -- as an output, an input, a sidechain or a staging
///   slot -- declines. The chain's own ops are excluded because they all run *before* the scatter:
///   the first slot's reductions are the gather, and no later slot's op is executed at all.
///
///   Ops outside that range cannot be a hazard. One before the chain's first op belongs to a unit
///   emitted at or before that op, so it has already run. One at or after the consumer's op that
///   names the buffer is a legitimate reader of the value the consumer produces, and a bank that
///   hoisted it past the consumer would be the #169 defect the bank window already forbids.
fn scatter_redirects(
    program: &ExecutionProgram,
    bank_membership: &BankMembership,
    run_units: &[(Vec<Membership>, Vec<usize>)],
) -> Vec<ScatterRedirect> {
    let (readers, first_producer) = op_dataflow(program);
    let mut redirects = Vec::new();
    for (run, (membership, ops)) in run_units.iter().enumerate() {
        if membership.is_empty() || ops.is_empty() {
            continue;
        }
        let lanes = ops.len() / membership.len();
        let last = ops.len() - lanes;
        let Some(first_op) = ops.iter().min().copied() else {
            continue;
        };
        let mut chain: Vec<(usize, Option<usize>)> = Vec::with_capacity(lanes);
        for lane in 0..lanes {
            let producer = ops[last + lane];
            chain.push((
                producer,
                scatter_target(
                    program,
                    bank_membership,
                    &readers,
                    &first_producer,
                    ops,
                    first_op,
                    producer,
                ),
            ));
        }
        // A chain scatters every lane in one pass, so its targets have to stay pairwise distinct.
        // They can stop being so: a consumer's buffer is coloured after its own producer's is
        // released, so one lane's consumer may have been handed the *physical slot* another lane's
        // last slot still scatters into. Two lanes would then write one buffer in the same pass.
        // The colouring is what decides this and it is not knowable lane by lane, so it is checked
        // over the whole chain, and a collision declines every redirect in it rather than picking a
        // winner.
        let scattered: Vec<crate::program::BufferRef> = chain
            .iter()
            .map(|(producer, target)| program.ops[target.unwrap_or(*producer)].output)
            .collect();
        let distinct: std::collections::BTreeSet<_> = scattered.iter().collect();
        if distinct.len() != scattered.len() {
            continue;
        }
        for (lane, (_, target)) in chain.into_iter().enumerate() {
            if let Some(consumer) = target {
                redirects.push((run, last + lane, consumer));
            }
        }
    }
    redirects
}

/// # Which clauses a mutation makes red (the house ledger, measured)
///
/// Dropping a clause and running the graph and graph-compiler suites gives:
///
/// * **sole readership** -> `a_send_from_the_last_slots_alias_declines_that_lanes_scatter_redirect`
/// * **one main input** -> `id_ordered_bank_plan_rejects_transactionally_and_returned_ownership_\
///   is_reusable`
/// * **the consumer is not a bank member** -> `misaligned_lane_sets_decline_the_merge` and two
///   others
/// * **nothing in between names the buffer** -> seven session-level tests, including the fixture's
///   own `the_intended_strip_fuses_the_whole_signal_path_into_one_chain_per_cohort`, *and* --
///   for the scan's **end boundary**, which none of those seven pin --
///   `cohort_chain_merging_preserves_dataflow_on_random_graphs` through
///   [`scatter_redirects_over_program`]. Shortening the scan by a single op reddens it at graph 24.
/// * **no compensation delay on the consumer's input** -> the same differential eval, at graph 0.
///   No *compiled session* reaches a delayed consumer of a chain's last slot -- PDC is inserted on
///   route edges between tracks, never between a rack slot and the fader it feeds -- so this clause
///   has no session-level red test and never will. The corpus builds the case 38,725 times, so the
///   eval is where it is defended.
///
/// Four clauses have **no** red test, and each is kept for a stated reason rather than a measured
/// one. Saying so is the point of writing the ledger down:
///
/// * **no sidechain on the consumer** is conservative and nothing more. A sidechained consumer's
///   *reduction* is still a pure copy and the redirect does not touch which buffer its
///   `write_read_stereo` names, so no hazard is known here. It is kept because
///   [`bank_gather_source`] carries the same clause and a reader comparing the two should not have
///   to work out why one side omits it.
/// * **`first_producer` agrees** is redundant given the two above it: if the producer's sole reader
///   is this consumer and the consumer has exactly one main input, that input is the producer's
///   buffer. It re-derives the fact from the colouring rather than inferring it.
/// * **not already in place** is an early-out: if the consumer already writes the producer's
///   buffer, the redirect's target *is* that buffer and nothing changes.
/// * **not the session output** no longer defends a live hazard. Since issue #916 the Output is
///   dedicated storage, never folded onto its producer in place, and the host reads no arena
///   buffer. It can still fire by slot coincidence (see the clause's own bullet above), which only
///   declines. The hazard it once defended, a stale session output, now lives on the consumer
///   side, and `build_sequential` declines a redirect into the Output op.
///
/// **Pairwise-distinct scatter targets is defensive by construction, not merely unreached.** The
/// adversarial verification of issue #202 closed this one: for a bank that satisfies the contract
/// the collision cannot arise at all. A chain's last slot is `program::is_dedicated` storage, and a
/// dedicated buffer is never returned to the free list, so no consumer can ever be coloured onto a
/// slot another lane is still scattering into; and where two lanes' consumers *are* ordered such
/// that one could be, the earlier lane has already declined on another clause. The guard is
/// therefore a construction check rather than a hazard defence -- it costs one `BTreeSet` per chain
/// at bind and it is what makes "a chain scatters every lane in one pass" a checked fact rather
/// than an inherited assumption. It is deliberately **not** presented as a measured clause.
///
/// The consumer's *own* observers are deliberately **not** a clause. The redirect changes nothing
/// about what the consumer's buffer holds by the time its observers run, so there is nothing there
/// for one to see.
///
/// Nor, since issue #886, are the **producer's** observers -- bound to the last slot's own node,
/// or through a `program::Tap` to a stage boundary elided onto its buffer. Both used to decline,
/// on the reasoning that the last slot's own buffer goes unwritten after the redirect; but no
/// observer reads that buffer. Every one of them reads the chain's final output, and is dispatched
/// from the last slot's own `RuntimeOp`, whose `output` [`apply_scatter_redirects`] repoints at
/// the consumer's buffer. Both dispatchers (`Runtime::observe_unit`, `observe_active_entry`) run
/// them straight after the chain's unit: after its scatter wrote that buffer, and before the
/// consumer's unit -- later by construction -- rewrites it in place. Each is first offered the
/// chain's resident final lane (`BankChain::final_output_lane`, the words the scatter
/// transposed), and one that declines it reads the member's output, which is the consumer's buffer
/// holding those same words; [`scatter_redirects`]' in-between clause keeps that slot the
/// scatter's alone until the consumer runs. So an observer of the chain's final output publishes
/// the same bits whether the lane redirects or not --
/// `a_redirected_metered_plan_is_the_unredirected_plans_master_and_meters_bit_for_bit` holds it to
/// that (and, with the production meter, graph-compiler's
/// `a_meter_on_a_bank_member_keeps_that_lanes_scatter_redirect`), and restoring either clause
/// reddens `an_observer_of_the_scattered_lane_keeps_the_direct_scatter_armed`. A redirect consumer
/// stays out of the scalar split fader/matrix pair, so a metered plan now takes the redirect where
/// it took the split pair before -- the unmetered plan's shape, with the same bits
/// (`a_metered_redirect_consumer_stays_out_of_the_split_pair_and_keeps_the_bits`). A stage boundary
/// *inside* a chain is not this function's question: `chains_into` still declines the merge across
/// an observed one, unchanged.
///
/// The consumer whose buffer one lane's scatter may land in, or `None`. See [`scatter_redirects`]
/// for what each clause is defending.
fn scatter_target(
    program: &ExecutionProgram,
    membership: &BankMembership,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    run: &[usize],
    first_op: usize,
    producer: usize,
) -> Option<usize> {
    if readers[producer].len() != 1 {
        return None;
    }
    let consumer = readers[producer][0];
    let producer_op = &program.ops[producer];
    let consumer_op = &program.ops[consumer];
    if consumer_op.input_count() != 1
        || consumer_op.sidechain.is_some()
        || program.inputs_of(consumer_op)[0].delay.is_some()
        || first_producer[consumer] != Some(producer)
    {
        return None;
    }
    if consumer_op.output == producer_op.output || producer_op.output == program.output {
        return None;
    }
    if membership.contains_key(&consumer_op.node) {
        return None;
    }
    let target = consumer_op.output;
    for (index, op) in program.ops.iter().enumerate().take(consumer).skip(first_op) {
        if run.contains(&index) {
            continue;
        }
        if op_names_buffer(program, op, target) {
            return None;
        }
    }
    Some(consumer)
}

/// Whether `op` mentions `buffer` at all: as its output, a main input, a sidechain, or the staging
/// slot of either.
fn op_names_buffer(program: &ExecutionProgram, op: &Op, buffer: crate::program::BufferRef) -> bool {
    let staging_or_buffer = |input: &crate::program::InputRef| {
        input.buffer == buffer || input.delay.is_some_and(|delay| delay.staging == buffer)
    };
    op.output == buffer
        || program.inputs_of(op).iter().any(staging_or_buffer)
        || op.sidechain.as_ref().is_some_and(staging_or_buffer)
}

/// The **runtime's own** scatter-redirect decision, driven over a program that has no bindings.
///
/// `program::tests::cohort_chain_merging_preserves_dataflow_on_random_graphs` interprets the
/// executor through a *model* of [`scatter_target`]'s clauses. A model is an oracle only while it
/// and the thing it models agree, and the model cannot check that by itself: the adversarial
/// verification of issue #202 found that shortening [`scatter_target`]'s in-between scan by a
/// single op -- `take(consumer)` to `take(consumer - 1)`, which is the unsound direction -- reddens
/// nothing anywhere, while the same one-token change to the model reddens the corpus at once. The
/// corpus was building the hazard at exactly that boundary and then only ever asking the model
/// about it.
///
/// This is the missing half. The corpus now drives *this* function, and the two answers must be
/// equal graph for graph, so every clause the corpus exercises has the corpus as its red test on
/// both sides of the model/runtime pair -- the same correspondence #194 established on the gather
/// side.
///
/// Returns `producer op -> consumer op` for every admitted lane. Every clause is a question about
/// the lowered program and the bank membership, so this eval drives all of them. `_spec` keeps the
/// corpus's call site unchanged: the only clauses that named a node were the two observer clauses
/// issue #886 removed, and the program-level fixture bound no observers to ask them about anyway.
#[cfg(test)]
pub(crate) fn scatter_redirects_over_program(
    program: &ExecutionProgram,
    _spec: &GraphSpec,
    lanes: &BTreeMap<u32, (usize, usize)>,
    runs: &[Vec<Vec<usize>>],
) -> BTreeMap<usize, usize> {
    let bank_membership: BankMembership = lanes
        .iter()
        .map(|(node, (bank, lane))| (*node, (Membership::Effect(*bank), *lane)))
        .collect();
    // `build_sequential` builds this shape from `units_of`; here it is built from the model's runs,
    // so the only thing that differs between the two sides is which copy of the clauses answers.
    // A plain unit carries an empty membership list, which is what marks it as not a bank.
    let run_units: Vec<(Vec<Membership>, Vec<usize>)> = runs
        .iter()
        .map(|run| {
            let last = run.last().expect("a run has at least one slot");
            let banked = last.len() > 1 || lanes.contains_key(&program.ops[last[0]].node);
            let membership = if banked {
                vec![Membership::Effect(0); run.len()]
            } else {
                Vec::new()
            };
            (membership, run.iter().flatten().copied().collect())
        })
        .collect();
    scatter_redirects(program, &bank_membership, &run_units)
        .into_iter()
        .map(|(run, member, consumer)| (run_units[run].1[member], consumer))
        .collect()
}

/// Point each admitted lane's scatter at its consumer's buffer and neutralise the consumer's
/// reduction.
///
/// Two edits, and they are each other's counterpart: the member's `output` is what
/// `Runtime::execute` reads to build the chain's scatter list, and the consumer's single input
/// becomes its own output, which is the shape `reduce_plane` already treats as "nothing to copy".
/// The consumer therefore runs exactly as an `in_place` op does, which is what it would have been
/// had its producer not been dedicated storage.
///
/// A redirect names its chain by *run* (`run_units`), and `units` holds only the emitted ones: a
/// route the fold retired emits no unit, so every chain after it sits at a lower index than its
/// run. `unit_of_run` is that mapping, and indexing `units` by the run directly -- as this did
/// until it was found under issue #886 -- panics at bind, or repoints another unit's member, for
/// any chain that redirects after a folded one.
fn apply_scatter_redirects(
    program: &ExecutionProgram,
    redirects: &[ScatterRedirect],
    unit_of_run: &[Option<usize>],
    op_slot: &[Option<(usize, usize)>],
    units: &mut [RuntimeUnit],
) {
    for (run, member, consumer) in redirects.iter().copied() {
        let target = ARENA_BASE + program.ops[consumer].output.0;
        let Some(RuntimeUnit::Bank { members, .. }) = unit_of_run
            .get(run)
            .copied()
            .flatten()
            .and_then(|unit| units.get_mut(unit))
        else {
            continue;
        };
        members[member].output = target;
        let Some((unit, _)) = op_slot[consumer] else {
            continue;
        };
        match &mut units[unit] {
            RuntimeUnit::Op(op) => op.inputs = vec![target].into_boxed_slice(),
            // Unreachable by construction: `scatter_target` declines a banked consumer outright,
            // because such a consumer's own gather may already read the producer's buffer
            // (`bank_gather_source`) and the two redirects would then disagree about which buffer
            // holds this block's audio. Left inert rather than applying, so that if that clause is
            // ever loosened this arm does nothing instead of doing the wrong thing.
            RuntimeUnit::Bank { .. } => debug_assert!(false, "a banked consumer never redirects"),
        }
    }
}

/// A route's 2x2 with its linear gain folded in, once, at bind (D3, #98 F4).
///
/// One derivation, two callers: [`RuntimeParts::node_kind`] builds `NodeKind::Route` from it and
/// [`route_fold`] builds `FoldLane` from it, so a chain's epilogue cannot apply constants that
/// differ from the ones the route op it replaced would have applied.
const fn folded_route(transform: &RouteTransform) -> [f32; 4] {
    [
        transform.gain * transform.ll,
        transform.gain * transform.lr,
        transform.gain * transform.rl,
        transform.gain * transform.rr,
    ]
}

/// The route constants `node_kind` *would* hand this node, asked without consuming anything.
///
/// [`RuntimeParts::node_kind`] takes the node's binding, its prepared effect and its route out of
/// `parts` as it answers, so it can be asked exactly once and only while its op is being built.
/// The fold has to know before any op is built -- it decides which ops are built at all -- so this
/// restates the same cascade as a query, in the same precedence order, and returns `None` for
/// every arm that is not a plain route. Keep its exclusions coupled to new `node_kind` arms.
/// Bindings, banks, sources and effects take precedence regardless of the node's session name.
trait PlanningMetadata {
    fn membership(&self) -> &BankMembership;
    fn has_source(&self, node: &GraphNodeId) -> bool;
    fn has_binding(&self, node: &GraphNodeId) -> bool;
    fn has_effect(&self, node: &GraphNodeId) -> bool;
    fn has_observer(&self, node: &GraphNodeId) -> bool;
    fn route(&self, node: &GraphNodeId) -> Option<&RouteTransform>;
}

impl PlanningMetadata for RuntimeParts {
    fn membership(&self) -> &BankMembership {
        &self.membership
    }
    fn has_source(&self, node: &GraphNodeId) -> bool {
        self.source_inputs.contains(node)
    }
    fn has_binding(&self, node: &GraphNodeId) -> bool {
        matches!(self.bindings.get(node), Some(Some(_)))
    }
    fn has_effect(&self, node: &GraphNodeId) -> bool {
        self.effects.contains_key(node)
    }
    fn has_observer(&self, node: &GraphNodeId) -> bool {
        self.observers.contains_key(node)
    }
    fn route(&self, node: &GraphNodeId) -> Option<&RouteTransform> {
        self.routes.get(node)
    }
}

struct BorrowedPlanningMetadata<'a> {
    plan: &'a crate::PreparedGraphPlan,
    bindings: &'a crate::GraphRuntimeBindings,
    sources: Option<&'a crate::GraphPreparedSourceSet>,
    membership: BankMembership,
}

impl PlanningMetadata for BorrowedPlanningMetadata<'_> {
    fn membership(&self) -> &BankMembership {
        &self.membership
    }
    fn has_source(&self, node: &GraphNodeId) -> bool {
        self.sources
            .is_some_and(|set| set.claims().iter().any(|claim| &claim.node == node))
    }
    fn has_binding(&self, node: &GraphNodeId) -> bool {
        self.bindings
            .nodes
            .iter()
            .any(|binding| &binding.node == node && binding.processor.is_some())
    }
    fn has_effect(&self, node: &GraphNodeId) -> bool {
        self.plan
            .effects
            .iter()
            .any(|effect| matches!(node, GraphNodeId::Effect(id) if *id == effect.id))
    }
    fn has_observer(&self, node: &GraphNodeId) -> bool {
        self.plan
            .observers
            .iter()
            .chain(self.bindings.observers.iter())
            .any(|observer| &observer.node == node)
    }
    fn route(&self, node: &GraphNodeId) -> Option<&RouteTransform> {
        self.plan
            .routes
            .iter()
            .rev()
            .find(|route| &route.node == node)
            .map(|route| &route.transform)
    }
}

fn plain_route_gains(
    parts: &impl PlanningMetadata,
    node: &GraphNodeId,
    index: u32,
) -> Option<[f32; 4]> {
    if parts.has_source(node)
        || parts.membership().contains_key(&index)
        || parts.has_binding(node)
        || parts.has_effect(node)
    {
        return None;
    }
    parts.route(node).map(folded_route)
}

/// Whether anything can *see* the buffer op `index` writes other than by reading it as an input,
/// leaving out observers bound to a node `served` says the caller reaches some other way.
///
/// Two ways, and they are the pair [`chains_into`] carries (as [`scatter_target`] did until issue
/// #886; a redirected lane's observers read the redirected buffer): an observer bound to the
/// producing node, and an observer bound to an elided node whose alias resolves to this op's
/// buffer (`program::Tap`). `parts.observers` is keyed by node, so the second has to name the
/// alias node rather than the producing one. `served` is asked about the node an observer is bound
/// to, never about the op: the alias and the producer are one buffer, but only the node says which
/// boundary the observer declared.
fn observed(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &impl PlanningMetadata,
    index: usize,
    served: fn(&GraphNodeId) -> bool,
) -> bool {
    let node = &spec.nodes[program.ops[index].node as usize].id;
    if parts.has_observer(node) && !served(node) {
        return true;
    }
    program
        .taps
        .iter()
        .filter(|tap| tap.after_op as usize == index)
        .map(|tap| &spec.nodes[tap.node as usize].id)
        .any(|node| parts.has_observer(node) && !served(node))
}

/// The one observation boundary a folded lane still serves (issue #885): post-matrix.
///
/// A folded lane's last-slot buffer is never scattered, but the words it would have held are the
/// chain's resident final lane (`BankChain::final_output_lane`) -- untouched by the epilogue, which
/// mixes a transposed *copy* of them in the staging tile. `Runtime::observe_unit` offers exactly
/// those words to the lane's observers, and writes them into the member buffer first for one that
/// declines the resident view. Every other boundary keeps declining the fold: the route's own
/// output never exists once it folds, and any other stage tap is outside what this slice admits.
const fn served_by_the_resident_lane(node: &GraphNodeId) -> bool {
    matches!(
        node,
        GraphNodeId::TrackStage {
            stage: TrackStage::PostMatrix,
            ..
        }
    )
}

/// No observer is served another way: every one of them declines.
const fn served_by_nothing(_node: &GraphNodeId) -> bool {
    false
}

/// One chain's folded epilogue: which run unit it is, and one entry per rendered lane.
type FoldedRun = (usize, Vec<FoldLane>);

/// One candidate chain mid-proof: its run unit, and `(route op, folded 2x2)` for every lane.
type FoldCandidate = (usize, Vec<(usize, [f32; 4])>);

/// What [`route_fold`] admitted: the chains that fold, the route ops that stop running, and the
/// reduction that was replaced.
struct RouteFold {
    /// Folded chains in render order; the order the master is accumulated in.
    runs: Vec<FoldedRun>,
    /// Route ops the epilogues absorbed. These are not built into units at all.
    retired: std::collections::BTreeSet<usize>,
    /// The op whose reduction the epilogues performed. It still runs; its inputs become its own
    /// output, which is the no-op `reduce_plane` already performs for a single input that is its
    /// own output.
    master_op: usize,
    /// The buffer that op writes, in the lowering's numbering.
    master: crate::program::BufferRef,
}

/// The producer of each of op `target`'s main inputs, in `inputs` order.
///
/// The same last-writer walk [`op_dataflow`] does, stopped at `target` and kept *per input
/// position* rather than for position zero alone -- which is exactly the sequence the association
/// proof in [`route_fold`] has to compare against.
fn input_producers(program: &ExecutionProgram, target: usize) -> Vec<Option<usize>> {
    let mut owner: Vec<Option<usize>> = vec![None; program.buffers as usize];
    for (index, op) in program.ops.iter().enumerate() {
        if index == target {
            return program
                .inputs_of(op)
                .iter()
                .map(|input| owner[input.buffer.0 as usize])
                .collect();
        }
        owner[op.output.0 as usize] = Some(index);
    }
    Vec::new()
}

/// One lane's route, if that lane's chain may absorb it: `(route op, folded 2x2)`.
///
/// `producer` is the chain's last slot for this lane. Every clause is one way absorbing the route
/// could be observed; see [`route_fold`] for the ledger.
fn foldable_lane(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &impl PlanningMetadata,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    producer: usize,
) -> Option<(usize, [f32; 4])> {
    if readers[producer].len() != 1 {
        return None;
    }
    let route = readers[producer][0];
    let producer_op = &program.ops[producer];
    let route_op = &program.ops[route];
    if route_op.input_count() != 1
        || route_op.sidechain.is_some()
        || program.inputs_of(route_op)[0].delay.is_some()
        || first_producer[route] != Some(producer)
    {
        return None;
    }
    if producer_op.output == program.output || route_op.output == program.output {
        return None;
    }
    // A post-matrix observer on the last slot reads the chain's resident final lane instead of
    // the member buffer the fold stops writing (issue #885). The route has no such stand-in: its
    // output is mixed straight into the master and never exists on its own.
    if observed(program, spec, parts, producer, served_by_the_resident_lane)
        || observed(program, spec, parts, route, served_by_nothing)
    {
        return None;
    }
    let node = &spec.nodes[route_op.node as usize].id;
    let gains = plain_route_gains(parts, node, route_op.node)?;
    Some((route, gains))
}

/// Fold every cohort chain's routes and the master reduction into the chains' own epilogues.
///
/// The shape this replaces, on the intended 64-track strip, is one route op per track -- a whole
/// stereo block of `mix2x2_block` over a buffer the chain had just scattered -- followed by a
/// 63-pass `sum_into_block` reduction over those 64 buffers. The shape it renders instead is one
/// pass: each lane's tile is routed where the transpose left it and goes straight into the master,
/// the first contributor storing and the rest accumulating.
///
/// # The association proof, which is the whole of the correctness argument
///
/// D9 fixes the reduction as `sum2_block(in0, in1)` then `sum_into_block` left to right over
/// `spec.edges` order, and a floating-point sum is not associative, so "the same summands" is not
/// "the same bits". The epilogues accumulate in **chain execution order**, which is a different
/// sequence written down in a different place. They are only the same reduction if the two
/// sequences are equal, and this function proves that rather than assuming it: it walks the lowered
/// program, resolves the producer of *every* input position of the reduction (`input_producers`),
/// and requires that list to equal, element for element and in order, the route ops of the folded
/// chains taken in render order, lane by lane. Anything else -- a contributor that is not a folded
/// lane, a lane out of place, a cohort whose planner ordered its lanes differently from the edge
/// order -- fails the equality and declines the whole fold. This mirrors how `chains_into` proves
/// lane alignment on the lowered program instead of trusting two planners to agree.
///
/// Equality of the *whole* list is also what makes coverage total: a master with one contributor
/// that is not a folded lane cannot be folded at all, because there is no position in the chains'
/// order at which that contributor's summand could be inserted.
///
/// # Which clauses a mutation makes red (the house ledger, measured)
///
/// Dropping a clause and running the graph, graph-compiler and console-workload suites gives:
///
/// * **the association order** -> `route_ids_ordered_against_the_cohorts_decline_the_route_fold`,
///   plus `a_leased_stage_meter_declines_the_merge_and_still_meters` and
///   `a_pre_fader_meter_splits_its_cohorts_chain_and_reads_the_limiter` (named
///   `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect` until issue #886,
///   the name `crates/graph/tests/MUTATIONS.md` row 218-3 still records). Keeping the length check
///   and dropping the element-wise comparison is the unsound direction, and it is the one measured.
/// * **no observer on the route/output path other than post-matrix** (issue #885) ->
///   `a_post_matrix_meter_on_every_track_of_a_full_bank_keeps_the_fold_armed`, both ways:
///   excusing no observer leaves the metered arms unfolded, and excusing every observer folds the
///   post-fader-metered arm. What makes the post-matrix excuse sound -- the resident view, and
///   the member buffer written for an observer that declines it -- is
///   `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`, and with the
///   production meter `a_meter_on_the_matrix_keeps_the_route_fold_and_still_meters`. Before #885
///   this clause declined post-matrix meters too, and two fold oracles were such a metered plan;
///   they now decline the fold through `test_only_set_route_fold_declined` instead.
/// * **the opening chain's ops are excluded from the in-between scan** -> every fold in the tree
///   stops firing and `every_standing_workload_folds_one_route_per_track` goes red. That is the
///   conservative direction, and it is worth pinning: the colouring gives the session output the
///   physical slot of track zero's *input* buffer on every console fixture, so without the
///   exclusion the standing fixture never folds at all.
/// * **the first contributor stores** ->
///   `the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign`.
/// * **the whole fold** -> `every_standing_workload_folds_one_route_per_track`, on a count. There
///   is no output difference to see: that is the point of the counter.
///
/// Historically four clauses had no red test in compiled-session fixtures. Issue #220 now
/// drives the actual proof over the seeded corpus and bounded valid lowered-program shapes.
///
/// * **sole readership of a chain's last slot** ->
///   `route_fold_shadowed_clauses_over_valid_programs`, hazard 1. Removing the guard admits a
///   fold whose last-slot buffer still has another reader after its route. Compiled-session
///   ordering used to hide this: another route changed the association list, while a sidechain
///   reader scheduled before the route made the first reader fail the plain-route check.
///   The constructed order isolates the last-slot readership guard from those other refusals.
/// * **nothing in between names the master** ->
///   `route_fold_shadowed_clauses_over_valid_programs`, hazard 3. Removing the access check admits
///   a fold that overwrites a reused buffer before an intervening reader consumes its old value.
///   Existing compiled-session colouring hid this hazard in the opening cohort, which the scan
///   correctly excludes. The constructed program puts that reader in a later unit instead.
///   These are measured physical guard removals, not mutations of the independent oracle.
/// * **candidate retention requires exclusive route readership** ->
///   `route_fold_shadowed_clauses_over_valid_programs`, hazard 2. Removing the retain block admits
///   a route with an additional reader while the master's ordered contributors still match.
///   This does not independently prove the same-master equality conjunct: that conjunct remains
///   logically shadowed by the association proof. This implementation folds one reduction only.
/// * **the master buffer is distinct from every folded buffer.** The colouring cannot hand the
///   master a slot a folded lane still writes -- a chain's last slot is `program::is_dedicated`
///   storage and is never returned to the free list -- so this is a construction check, in the
///   same sense as `scatter_redirects`' pairwise-distinct guard, and is deliberately not presented
///   as a hazard defence.
fn route_fold(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &impl PlanningMetadata,
    run_units: &[(Vec<Membership>, Vec<usize>)],
) -> Option<RouteFold> {
    let (readers, first_producer) = op_dataflow(program);
    // (run unit, one (route op, folded 2x2) per rendered lane), in render order.
    let mut candidates: Vec<FoldCandidate> = Vec::new();
    for (run, (membership, ops)) in run_units.iter().enumerate() {
        if membership.is_empty() || ops.is_empty() {
            continue;
        }
        let lanes = ops.len() / membership.len();
        let last = ops.len() - lanes;
        let mut folded = Vec::with_capacity(lanes);
        for lane in 0..lanes {
            match foldable_lane(
                program,
                spec,
                parts,
                &readers,
                &first_producer,
                ops[last + lane],
            ) {
                Some(lane_fold) => folded.push(lane_fold),
                // A chain folds every lane or none: a half-folded chain would have to keep its
                // scatter for the rest, and its unfolded lanes' routes would then have to be
                // inserted into the master's order somewhere the chains' order has no room for.
                None => {
                    folded.clear();
                    break;
                }
            }
        }
        if folded.is_empty() {
            continue;
        }
        candidates.push((run, folded));
    }
    let first_route = candidates.first()?.1.first()?.0;
    if readers[first_route].len() != 1 {
        return None;
    }
    let master_op = readers[first_route][0];
    // Every candidate must reduce into that one master, and only into it.
    candidates.retain(|(_, lanes)| {
        lanes
            .iter()
            .all(|(route, _)| readers[*route].len() == 1 && readers[*route][0] == master_op)
    });
    if candidates.is_empty() {
        return None;
    }
    let master = &program.ops[master_op];
    if master.sidechain.is_some()
        || program
            .inputs_of(master)
            .iter()
            .any(|input| input.delay.is_some())
        || parts.membership().contains_key(&master.node)
        || parts.has_source(&spec.nodes[master.node as usize].id)
    {
        return None;
    }
    // The association proof: the reduction's contributors, in its own edge order, are exactly the
    // folded lanes in render order.
    let ordered: Vec<usize> = candidates
        .iter()
        .flat_map(|(_, lanes)| lanes.iter().map(|(route, _)| *route))
        .collect();
    let producers = input_producers(program, master_op);
    if producers.len() != ordered.len()
        || producers
            .iter()
            .zip(ordered.iter())
            .any(|(producer, route)| *producer != Some(*route))
    {
        return None;
    }
    let target = master.output;
    // A folded lane's own buffers stop being written, so the master must not be one of them.
    if candidates.iter().any(|(_, lanes)| {
        lanes
            .iter()
            .any(|(route, _)| program.ops[*route].output == target)
    }) {
        return None;
    }
    // The epilogues write the master at each chain's position, which is earlier -- often much
    // earlier -- than the reduction they replaced. Everything scheduled in between must therefore
    // leave the buffer alone: a reader would see a partial sum, and a writer would clobber one.
    //
    // The window is over *units*, not op indices, because a unit is what runs. Three exclusions,
    // each for a reason:
    //
    // * **the opening folded chain's own unit.** Every op it owns runs before its chain does, and
    //   its chain's scatter is the first write to the master, so nothing it names can be a hazard.
    //   This is load-bearing rather than tidy: the colouring reuses the session output's physical
    //   slot for track zero's *input* buffer on every console fixture in the tree, so the opening
    //   cohort's lane-zero gather reads the master's slot -- before the master exists -- on every
    //   block. Excluding only the retired routes declines the whole fold on the standing fixture.
    // * **every unit at or after the master's.** The master op is the value they are entitled to
    //   read, and the master's own unit holds nothing but the master op (a banked master is
    //   declined above).
    // * **the retired routes**, which no longer run at all.
    //
    // Everything else is checked, chain members included: a later cohort's first-slot reduction
    // runs *after* the opening cohort's scatter, and its gather source is one of the inputs
    // `op_names_buffer` counts, so a cohort whose gather read the master's slot declines here.
    let retired: std::collections::BTreeSet<usize> = ordered.iter().copied().collect();
    let opening = candidates.first().expect("a non-empty candidate list").0;
    let master_run = run_units
        .iter()
        .position(|(_, ops)| ops.contains(&master_op))?;
    if master_run <= opening {
        return None;
    }
    for (run, (_, ops)) in run_units.iter().enumerate() {
        if run <= opening || run >= master_run {
            continue;
        }
        if ops
            .iter()
            .filter(|index| !retired.contains(index))
            .any(|index| op_names_buffer(program, &program.ops[*index], target))
        {
            return None;
        }
    }
    let mut store = true;
    let runs = candidates
        .into_iter()
        .map(|(run, lanes)| {
            let fold = lanes
                .into_iter()
                .map(|(_, coefficients)| {
                    let lane = FoldLane {
                        coefficients,
                        store,
                    };
                    store = false;
                    lane
                })
                .collect();
            (run, fold)
        })
        .collect();
    Some(RouteFold {
        runs,
        retired,
        master_op,
        master: target,
    })
}

/// What [`output_route_fold`] admitted (issue #926): the route ops the session Output op's fused
/// reduction absorbs, and one folded 2x2 per input of that op in its edge order.
struct OutputRouteFold {
    /// Route ops that are not built into units at all. Their mixes run inside
    /// [`route_reduce`].
    retired: std::collections::BTreeSet<usize>,
    /// `routes[i]` is the folded 2x2 of the route whose in-place buffer is the Output op's input
    /// `i`: [`folded_route`], the constants `node_kind` would have handed that route op.
    routes: Vec<[f32; 4]>,
    /// `producers[i]` is that route op itself: the retired route that feeds the Output op's input
    /// `i`. Issue #927's [`source_plane_table`] reads it to find which input a source claim is.
    producers: Vec<usize>,
}

/// Retire every plain route that feeds the session Output op, and fuse their mixes into its
/// reduction (issue #926). The clauses are issue #920's, which designed this fold and never landed;
/// #926 kept them unchanged and replaced only the kernel.
///
/// The shape this replaces is the plumbing row's: one route op per track, each a whole stereo
/// block of `mix2x2_block` stored over a buffer it runs in place on, then the Output op's
/// reduction loading every one of those buffers back. The shape it renders instead is the
/// reduction alone, with each route's 2x2 applied to the input it loads ([`route_reduce`]). The
/// Output op keeps its inputs: an in-place route writes the buffer it reads, so the Output
/// already names the route's input buffer, and only what that buffer holds when it is read
/// changes -- the route's input rather than its output. Every clause below is one way that
/// difference could be seen, or one way the fused arithmetic could stop being the two ops' own.
/// Any failure declines the whole fold: the table has one entry per input or none.
///
/// **The master (`master_op`, the Output op by node).**
///
/// * **The plan binds no bank.** A banked plan's routes are [`route_fold`]'s, whose chain
///   epilogues absorb them and the master's reduction together. Where that fold declines (the
///   alternating half-mono row, or a test's declined oracle), this one declines too, so no
///   banked plan changes shape here.
/// * **Its kind is `NodeKind::Identity`**, by `node_kind`'s own cascade: no source, no bank, no
///   bound processor, no effect, no route. A processor would run after the reduction on the host
///   planes and is untouched by this change, but the issue admits the identity output only.
/// * **No sidechain, no delayed input, fan-in two or more.** A delayed input is staged through a
///   compensation line, and the op reads the staging buffer. Fused, the line would carry the
///   route's input rather than its output, and the kernel would mix the line's initial `+0.0`
///   words, which no route op ever mixed: a negative 2x2 turns them into `-0.0`. Fan-in one is the
///   Output's copy, which is not a reduction. A split pair never binds the Output node: only a
///   `PostFader`/`PostMatrix` pair does.
///
/// **Every input `i`, whose producer is `R` ([`input_producers`]).**
///
/// * **`R` is a plain route** ([`plain_route_gains`]): the table entry is the 2x2 `node_kind`
///   would have built, from the same [`folded_route`]. It has one undelayed input and no
///   sidechain, so it is exactly `mix2x2_block` over one buffer. Any other producer declines the
///   whole fold, a pass-through (a submix) included: mixing it through an identity 2x2 is not a
///   no-op, because `0 * r` is `+0.0` and turns a `-0.0` into `+0.0`.
/// * **`R` ran in place** (`in_place`, and its output is its input's buffer). The buffer's colour
///   is then owned from `R`'s producer through the Output op, its last reader, so the Output
///   reads the words the producer wrote. A route that copied would have let its input's colour
///   die at `R`.
/// * **The Output op is `R`'s only reader** (`readers[R] == [master_op]`, sidechains counted).
///   Any other reader, before the Output op or after it, would read the unmixed words where it
///   read the route's output.
/// * **Nothing observes `R`** (`observed`, `served_by_nothing`): neither an observer on the route
///   node nor one on an elided stage whose `program::Tap` aliases `R`'s buffer after `R`. Either
///   fires after `R` and would read the unmixed words. An observer on `R`'s *producer* fires
///   before `R` ran, and reads the same words either way.
/// * **`R` is a plain unit before the Output op's, and nothing in between names its buffer.**
///   `R` used to mix the buffer at its own position, which is earlier than the Output op; every
///   unit between the two now sees the unmixed words where it would have seen the mixed ones. So
///   every op of every unit strictly between them is checked with [`op_names_buffer`] -- as an
///   output, an input, a sidechain or a staging slot -- and any mention declines. The clauses
///   above already rule a mention out: a write would need the live buffer's slot, which the
///   colouring owns through the Output op, and a read is a second reader. So no test can make
///   this scan fire (`crates/graph/tests/MUTATIONS.md` row 926-14); it is the belt to that brace,
///   as [`route_fold`]'s in-between scan is. The other retired routes are scanned too: each names
///   only its own buffer, which is live at the same time and so is a different slot.
/// * **Each `R` feeds exactly one input position.** A route read twice has two readers.
fn output_route_fold(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &impl PlanningMetadata,
    run_units: &[(Vec<Membership>, Vec<usize>)],
    master_op: usize,
) -> Option<OutputRouteFold> {
    if !parts.membership().is_empty() {
        return None;
    }
    let master = program.ops.get(master_op)?;
    let node = &spec.nodes.get(master.node as usize)?.id;
    if !matches!(node, GraphNodeId::Output { .. })
        || parts.has_source(node)
        || parts.membership().contains_key(&master.node)
        || parts.has_binding(node)
        || parts.has_effect(node)
        || parts.route(node).is_some()
    {
        return None;
    }
    let inputs = program.inputs_of(master);
    if master.sidechain.is_some()
        || inputs.len() < 2
        || inputs.iter().any(|input| input.delay.is_some())
    {
        return None;
    }
    let producers = input_producers(program, master_op);
    if producers.len() != inputs.len() {
        return None;
    }
    let (readers, _) = op_dataflow(program);
    let run_of = |op: usize| run_units.iter().position(|(_, ops)| ops.contains(&op));
    let master_run = run_of(master_op)?;
    let mut retired = std::collections::BTreeSet::new();
    let mut routes = Vec::with_capacity(inputs.len());
    let mut retired_producers = Vec::with_capacity(inputs.len());
    for (input, producer) in inputs.iter().zip(&producers) {
        let route = (*producer)?;
        let route_op = &program.ops[route];
        let route_node = &spec.nodes[route_op.node as usize].id;
        let gains = plain_route_gains(parts, route_node, route_op.node)?;
        let [route_input] = program.inputs_of(route_op) else {
            return None;
        };
        if route_op.sidechain.is_some()
            || route_input.delay.is_some()
            || !route_op.in_place
            || route_op.output != route_input.buffer
            || route_op.output != input.buffer
            || readers[route].as_slice() != [master_op]
            || observed(program, spec, parts, route, served_by_nothing)
        {
            return None;
        }
        let route_run = run_of(route)?;
        let (membership, ops) = &run_units[route_run];
        if !membership.is_empty() || ops.as_slice() != [route] || route_run >= master_run {
            return None;
        }
        if run_units[route_run + 1..master_run]
            .iter()
            .flat_map(|(_, ops)| ops)
            .any(|index| op_names_buffer(program, &program.ops[*index], route_op.output))
        {
            return None;
        }
        if !retired.insert(route) {
            return None;
        }
        routes.push(gains);
        retired_producers.push(route);
    }
    Some(OutputRouteFold {
        retired,
        routes,
        producers: retired_producers,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_op(
    parts: &mut RuntimeParts,
    spec: &GraphSpec,
    op: &Op,
    inputs: Vec<u32>,
    staged: Vec<StagedInput>,
    sidechain: Option<u32>,
    output: u32,
    aliases: Option<&[GraphNodeId]>,
) -> RuntimeOp {
    let node = spec.nodes[op.node as usize].id.clone();
    let split_pair = parts.split_pairs.remove(&node);
    let kind = parts.node_kind(&node, op.node);
    let observers = take_observers(
        &mut parts.observers,
        observer_nodes(node, aliases.unwrap_or(&[])),
    );
    RuntimeOp {
        inputs: inputs.into_boxed_slice(),
        staged: staged.into_boxed_slice(),
        sidechain,
        output,
        kind,
        split_pair,
        observers,
    }
}

fn finish_unit(
    parts: &mut RuntimeParts,
    run: &[Membership],
    mut members: Vec<RuntimeOp>,
    installation: Option<FoldInstallation>,
) -> RuntimeUnit {
    if run.is_empty() {
        return RuntimeUnit::Op(members.pop().expect("one op per plain unit"));
    }
    let lanes = members.len() / run.len();
    let (configuration, fold, master) = installation.map_or_else(
        || (None, Box::default(), FoldTarget::Arena(0)),
        |installation| {
            (
                Some(installation.configuration),
                installation.lanes,
                installation.master,
            )
        },
    );
    let chain = parts.chain_for_with_fold(run, lanes, configuration);
    RuntimeUnit::Bank {
        members: members.into_boxed_slice(),
        lanes,
        chain,
        fold,
        master,
    }
}

/// For each op, the ops that read its output, and the op that produced its first main input.
///
/// The lowering gives an op output a *coloured* buffer and colours are reused, so "which ops name
/// this buffer" is not the question -- "which ops name it while this op is still its last writer"
/// is. Walking `program.ops` in schedule order and remembering the last writer of each colour
/// answers exactly that: liveness colouring never reassigns a colour while a consumer still needs
/// it, so the last writer at the moment a consumer is reached is that consumer's producer. This
/// reuses #98/#99's colouring rather than forming a second opinion from the semantic graph.
fn op_dataflow(program: &ExecutionProgram) -> (Vec<Vec<usize>>, Vec<Option<usize>>) {
    let mut owner: Vec<Option<usize>> = vec![None; program.buffers as usize];
    let mut readers: Vec<Vec<usize>> = vec![Vec::new(); program.ops.len()];
    let mut first_producer: Vec<Option<usize>> = vec![None; program.ops.len()];
    for (index, op) in program.ops.iter().enumerate() {
        for (position, input) in program.inputs_of(op).iter().enumerate() {
            if let Some(producer) = owner[input.buffer.0 as usize] {
                readers[producer].push(index);
                if position == 0 {
                    first_producer[index] = Some(producer);
                }
            }
        }
        if let Some(Some(producer)) = op.sidechain.map(|side| owner[side.buffer.0 as usize]) {
            readers[producer].push(index);
        }
        owner[op.output.0 as usize] = Some(index);
    }
    (readers, first_producer)
}

/// `true` when unit `later`'s ops are exactly the lane-wise consumers of unit `earlier`'s, so the
/// two may be rendered as consecutive slots of one chain (issue #181, widened by #202 rec 2).
///
/// The merge replaces two planar/AoSoA round-trips with one: the chain gathers `earlier`'s member
/// outputs, runs both stages over the resident block, and scatters into `later`'s. The price is
/// that `earlier`'s output buffers are left holding the *chain's input* rather than the first
/// stage's output, and `later`'s ops never reduce. Every clause below is one way that could be
/// observed, and any one of them declines the merge:
///
/// * **Lane count and lane order.** The two slots must cover the same lanes, one op each, *in the
///   same order*: the `first_producer` clause below is checked lane by lane, so two banks whose
///   planners disagreed about which track sits in which lane can never fuse. This is the whole of
///   the lane-alignment obligation and it is proved on the lowered program rather than assumed of
///   the planners.
/// * **`later` reads only `earlier`, undelayed and unmixed.** One main input, no sidechain, no
///   compensation-delay staging -- otherwise skipping `later`'s reduction would drop a summand or
///   a delay line. A sidechained slot already blocks banking (#96 F9); this re-checks it on the
///   lowered program rather than trusting the planner.
/// * **Nothing else reads `earlier`.** Exactly one reader, and it is `later`'s op. A send, a
///   second consumer or a sidechain source would read the pre-stage signal; `op_dataflow` counts
///   sidechain reads, so a sidechained consumer of `earlier` is one of these and not an omission.
/// * **No observer.** An observer bound to `earlier` fires after the unit and would see the
///   chain's input.
/// * **Not the session output, and not by buffer index (issue #916).** This used to decline
///   `earlier.output == program.output`, because the host copied that buffer out after the last
///   unit. The host no longer reads any arena buffer: the Output op writes the host's planes.
///   The Output is dedicated storage, so its op is never in place over a producer. It *reads*
///   its producer, and the readership clause above already counts that read. What the slot
///   comparison still matched was a producer whose physical slot the colouring hands the Output
///   *after* that producer retired. That is a colouring coincidence, not a hazard. On the
///   builtins harness, it moved the scalar pair from the track that retires last to the other
///   one. `crates/graph/tests/MUTATIONS.md` row 916-16 is the evidence.
/// * **No *observed* alias.** A `program::Tap` aliases an elided stage boundary onto `earlier`'s
///   buffer. The alias is a name, not a read -- an edge out of it resolves to `earlier` and is
///   already counted as a second reader above -- so a tap on its own is not a reason to decline.
///   What can read one is an **observer bound to the alias node**, which is how a leased stage
///   meter reaches `PostSimd1`, `PostDynamic` or `PostSimd2PreFader`
///   (`builtins::MeterTap`). `parts.observers` is keyed by *node*, so the check has to
///   name the alias node and not the producing one; keying it on the producer would miss exactly
///   the meter it exists to protect.
///
/// # The perf cliff this last clause buys, stated out loud
///
/// Issue #181 declined on the presence of a tap alone, which is why the intended 64-track strip
/// stopped at the `simd1`/`simd2` boundary: the three elided rack-boundary stages put a tap on the
/// compressor and the limiter never fused. Nothing planar reads those aliases in a session that
/// leases no stage meter, so the refusal was paying for an observer that was not there. It is now
/// paid only when the observer is: **leasing a meter at `PostSimd1`, `PostDynamic` or
/// `PostSimd2PreFader` costs that track's cohort one extra planar/AoSoA round-trip per block**,
/// because its chain can no longer span the stage the meter reads. That is the intended trade --
/// the meter must see post-compressor audio, and a merged chain would hand it the chain's input --
/// and `a_leased_stage_meter_declines_the_merge_and_still_meters` pins both halves of it.
///
/// Effect observation (`ObservationLane`) is *not* such an observer and must not be confused
/// with one: it reads the effect's own resident state through `observe_resident`, never a planar
/// stage buffer, so an armed console lane neither declines the merge nor is disturbed by one.
fn chains_into(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &impl PlanningMetadata,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    earlier: &[usize],
    later: &[usize],
) -> bool {
    if earlier.len() != later.len() || earlier.is_empty() {
        return false;
    }
    for (before, after) in earlier.iter().zip(later.iter()) {
        let producer = &program.ops[*before];
        let consumer = &program.ops[*after];
        if consumer.input_count() != 1
            || consumer.sidechain.is_some()
            || program.inputs_of(consumer)[0].delay.is_some()
            || first_producer[*after] != Some(*before)
        {
            return false;
        }
        if readers[*before].len() != 1 || readers[*before][0] != *after {
            return false;
        }
        let node = &spec.nodes[producer.node as usize].id;
        if parts.has_observer(node) {
            return false;
        }
        if program
            .taps
            .iter()
            .filter(|tap| tap.after_op as usize == *before)
            .any(|tap| parts.has_observer(&spec.nodes[tap.node as usize].id))
        {
            return false;
        }
    }
    true
}

/// Groups the planned units into cohort runs: consecutive slots of one chain become one unit.
///
/// Returns one entry per rendered unit, in render order, each listing the planned-unit indices it
/// covers. A unit that merges with nothing is a run of one, which is what every unit was before
/// issue #181.
///
/// # Candidacy is the program's dataflow, not the planner's grouping (issue #202 rec 2)
///
/// Issue #181 asked the cohort planner which bound slots came out of one group and offered only
/// those pairs to [`chains_into`]. That is a strictly narrower question than the one the merge
/// actually needs answered, and it left three quarters of the intended strip's round-trips on the
/// table: `plan_bank_groups` pools per `RackLocation`, so no candidate ever crossed a rack
/// boundary, and a builtin bank has no cohort group at all, so the `builtins -> simd1` boundary
/// was not even expressible. On the 64-track intended fixture that is 8 groups x {builtins, simd1,
/// simd2} = 24 chains where 8 will do.
///
/// The candidate successor of a bank unit is therefore taken from the lowered program itself: the
/// unit that owns the op reading lane 0's output. `chains_into` then has to prove the whole
/// lane-wise relation anyway, so nothing is trusted to the planners -- least of all that two banks
/// planned by two different planners agree about which track sits in which lane. Where the merge
/// is admissible the planners' lane orders coincide *because the proof says so*, and where they do
/// not the merge is simply declined.
///
/// Two structural facts make the run construction below well formed:
///
/// * **The successor relation is injective.** `chains_into` requires
///   `first_producer[later[i]] == Some(earlier[i])` for every lane, so two different predecessors
///   would have to share lane 0's op -- that is, be the same unit. No unit is ever appended to two
///   runs.
/// * **A successor is always later in unit order.** Every lane of `later` is scheduled after the
///   matching lane of `earlier`, and a unit is emitted at its members' minimum op index, so the
///   minimum over `later` strictly exceeds the minimum over `earlier`. The runs therefore have no
///   cycles and stay in render order.
///
/// The op range a merged run permutes is held by `program::lower`'s bank window, which forms the
/// same union from the same lane-wise relation (`program::chainable_bank_groups`).
fn cohort_runs(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    parts: &impl PlanningMetadata,
    units: &[PlannedUnit],
) -> Vec<Vec<usize>> {
    let (readers, first_producer) = op_dataflow(program);
    let mut unit_of_op: Vec<Option<usize>> = vec![None; program.ops.len()];
    for (index, (membership, ops)) in units.iter().enumerate() {
        if membership.is_none() {
            continue;
        }
        for op in ops {
            unit_of_op[*op] = Some(index);
        }
    }
    let mut successor: BTreeMap<usize, usize> = BTreeMap::new();
    for (earlier, (membership, ops)) in units.iter().enumerate() {
        if membership.is_none() {
            continue;
        }
        // The sole reader of lane 0 names the only unit this one can possibly chain into.
        // `chains_into` re-checks sole readership for every lane, so a `first()` here is a lookup
        // and not a decision.
        let Some(later) = ops
            .first()
            .and_then(|lane| readers[*lane].first())
            .and_then(|reader| unit_of_op[*reader])
        else {
            continue;
        };
        if later == earlier {
            continue;
        }
        if chains_into(
            program,
            spec,
            parts,
            &readers,
            &first_producer,
            ops,
            &units[later].1,
        ) {
            successor.insert(earlier, later);
        }
    }
    let merged: std::collections::BTreeSet<usize> = successor.values().copied().collect();
    debug_assert_eq!(
        merged.len(),
        successor.len(),
        "the successor relation is injective, so no unit joins two runs"
    );
    let mut runs = Vec::with_capacity(units.len());
    for index in 0..units.len() {
        if merged.contains(&index) {
            continue;
        }
        let mut run = vec![index];
        let mut cursor = index;
        while let Some(next) = successor.get(&cursor) {
            run.push(*next);
            cursor = *next;
        }
        runs.push(run);
    }
    runs
}

#[cfg(test)]
pub(crate) type RouteFoldObservation = (usize, Vec<(usize, usize, [u32; 4], bool)>);

/// Observe the actual route-fold proof for unbound program fixtures. Constants are supplied by
/// the fixture; no admission predicate is replicated here. Like the scatter seam, run layout
/// comes from the independent program interpreter. Observers and prepared effects are absent.
#[cfg(test)]
pub(crate) fn route_folds_over_program(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    lanes: &BTreeMap<u32, (usize, usize)>,
    runs: &[Vec<Vec<usize>>],
    routes: &BTreeMap<GraphNodeId, RouteTransform>,
) -> Option<RouteFoldObservation> {
    let mut parts = RuntimeParts::new(
        spec,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Default::default(),
        Vec::new(),
        1,
    );
    parts.routes.clone_from(routes);
    parts.membership = lanes
        .iter()
        .map(|(node, (bank, lane))| (*node, (Membership::Effect(*bank), *lane)))
        .collect();
    let units: Vec<_> = runs
        .iter()
        .map(|run| {
            let banked = lanes.contains_key(&program.ops[run[0][0]].node);
            (
                if banked {
                    vec![Membership::Effect(0); run.len()]
                } else {
                    Vec::new()
                },
                run.iter().flatten().copied().collect(),
            )
        })
        .collect();
    route_fold(program, spec, &parts, &units).map(|fold| {
        let mut routes = fold.retired.into_iter().collect::<Vec<_>>();
        // Retirement is a set, but lane order is render order, not op-index order.
        let (readers, _) = op_dataflow(program);
        let lanes = fold
            .runs
            .into_iter()
            .flat_map(|(run, folded)| {
                runs[run]
                    .last()
                    .expect("last slot")
                    .iter()
                    .zip(folded)
                    .map(|(producer, lane)| {
                        let route = readers[*producer][0];
                        routes.retain(|retired| *retired != route);
                        (run, route, lane.coefficients.map(f32::to_bits), lane.store)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        assert!(
            routes.is_empty(),
            "every retired route belongs to a folded lane"
        );
        (fold.master_op, lanes)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::{BufferRef, DelayRef, InputRef};
    use core::any::Any;
    use lane::kernels::sum2_block;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn runtime_response_capture_uses_declared_owner_mapping_and_opaque_sink() {
        struct Owner;
        impl GraphRuntimeProcessor for Owner {
            fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }

            fn copy_response_snapshot(
                &self,
                sample_rate_hz: u32,
                request: OwnerSnapshotRequest<'_>,
            ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
                assert!(!request.bypassed);
                assert_eq!(
                    request.left.len(),
                    effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS
                );
                assert_eq!(
                    request.right.len(),
                    effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS
                );
                request.left[0] = ResponseSnapshotSection {
                    id: 9,
                    kind: 3,
                    enabled: true,
                    word_count: 1,
                    words: [0x1234_5678; effect_contract::RESPONSE_SNAPSHOT_WORDS],
                };
                request.right[0] = request.left[0];
                Ok(ResponseSnapshotSummary {
                    kind: ResponseSnapshotKind::ParametricEq,
                    sample_rate_hz,
                    bypassed: false,
                    sections: 1,
                })
            }
        }

        struct Sink {
            calls: usize,
            owner: Option<(String, String, String, u8, u32, u32)>,
            word: u32,
        }
        impl ResponseSnapshotSink for Sink {
            fn copy_owner(
                &mut self,
                owner: ResponseSnapshotOwnerInfo<'_>,
                left: &[ResponseSnapshotSection],
                right: &[ResponseSnapshotSection],
            ) -> Result<(), ResponseSnapshotError> {
                assert_eq!(left, right);
                assert_eq!(left.len(), 1);
                self.calls += 1;
                self.owner = Some((
                    owner.track_id.to_owned(),
                    owner.native_id.to_owned(),
                    owner.stable_id.to_owned(),
                    owner.rack,
                    owner.slot,
                    owner.kind,
                ));
                self.word = left[0].words[0];
                Ok(())
            }
        }

        let runtime = Runtime::new(
            stereo_lease(1, 1),
            Vec::new(),
            Vec::new(),
            vec![RuntimeUnit::Op(RuntimeOp {
                inputs: Box::new([]),
                staged: Box::new([]),
                sidechain: None,
                output: ARENA_BASE,
                kind: NodeKind::Bound(Box::new(Owner)),
                split_pair: None,
                observers: Box::new([]),
            })],
            Vec::new(),
            vec![UnitIdentity {
                banked: false,
                resident_input: false,
                observed: false,
                source_lanes: 0,
                stages: 1,
                upstream_of_seam_stages: 1,
                lane_tracks: Box::new([Box::from("track")]),
            }],
            vec![ResponseOwnerBinding {
                track_id: Box::from("track"),
                native_id: Box::from("miso.test.owner"),
                stable_id: Box::from("eq"),
                response_snapshot_declared: true,
                rack: 2,
                slot: 4,
                unit: 0,
                member: 0,
            }],
            0,
            0,
        );
        let mut sink = Sink {
            calls: 0,
            owner: None,
            word: 0,
        };
        assert_eq!(
            runtime.copy_response_snapshot("track", 48_000, &mut sink),
            Ok(1)
        );
        assert_eq!(sink.calls, 1);
        assert_eq!(sink.word, 0x1234_5678);
        assert_eq!(
            sink.owner,
            Some((
                "track".to_owned(),
                "miso.test.owner".to_owned(),
                "eq".to_owned(),
                2,
                4,
                1,
            ))
        );
        assert_eq!(
            runtime.copy_response_snapshot("missing", 48_000, &mut sink),
            Err(ResponseSnapshotError::MissingTrack)
        );
    }

    #[test]
    fn declared_response_provider_refuses_unsupported_hook() {
        struct DecliningOwner;
        impl GraphRuntimeProcessor for DecliningOwner {
            fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
            fn response_snapshot_declared(&self) -> bool {
                true
            }
            fn response_snapshot_native_id(&self) -> Option<&'static str> {
                Some("miso.test.declared")
            }
        }

        struct Sink;
        impl ResponseSnapshotSink for Sink {
            fn copy_owner(
                &mut self,
                _owner: ResponseSnapshotOwnerInfo<'_>,
                _left: &[ResponseSnapshotSection],
                _right: &[ResponseSnapshotSection],
            ) -> Result<(), ResponseSnapshotError> {
                panic!("unsupported provider must not publish an exclusion")
            }
        }

        let runtime = Runtime::new(
            stereo_lease(1, 1),
            Vec::new(),
            Vec::new(),
            vec![RuntimeUnit::Op(RuntimeOp {
                inputs: Box::new([]),
                staged: Box::new([]),
                sidechain: None,
                output: ARENA_BASE,
                kind: NodeKind::Bound(Box::new(DecliningOwner)),
                split_pair: None,
                observers: Box::new([]),
            })],
            Vec::new(),
            vec![UnitIdentity {
                banked: false,
                resident_input: false,
                observed: false,
                source_lanes: 0,
                stages: 1,
                upstream_of_seam_stages: 1,
                lane_tracks: Box::new([Box::from("track")]),
            }],
            vec![ResponseOwnerBinding {
                track_id: Box::from("track"),
                native_id: Box::from("miso.test.declared"),
                stable_id: Box::from("declared"),
                response_snapshot_declared: true,
                rack: 2,
                slot: 0,
                unit: 0,
                member: 0,
            }],
            0,
            0,
        );
        assert_eq!(
            runtime.copy_response_snapshot("track", 48_000, &mut Sink),
            Err(ResponseSnapshotError::Unsupported)
        );
    }

    #[test]
    fn resident_meter_dispatch_preserves_binding_order_lazy_fallback_and_accepted_errors() {
        struct Observer {
            handle: u64,
            accept: Option<bool>,
            trace: Arc<std::sync::Mutex<Vec<(u64, bool)>>>,
        }
        impl crate::GraphRuntimeObserver for Observer {
            fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
                self.trace.lock().unwrap().push((self.handle, false));
                assert_eq!(block.left, &[7.0; 3]);
                assert_eq!(block.right, &[-9.0; 3]);
                Ok(())
            }
            fn observe_resident(
                &mut self,
                block: crate::GraphResidentObservationBlock<'_>,
            ) -> Option<Result<(), RenderError>> {
                let fail = self.accept?;
                self.trace.lock().unwrap().push((self.handle, true));
                assert_eq!(block.first_sample, 71);
                for frame in 0..3 {
                    let at = frame * block.lane.width().lanes() as usize + block.lane.lane();
                    assert_eq!(block.lane.left()[at], 7.0);
                    assert_eq!(block.lane.right()[at], -9.0);
                }
                Some(if fail {
                    Err(RenderError::InvalidEnvelope)
                } else {
                    Ok(())
                })
            }
        }
        let mut lease = stereo_lease(3, 1);
        lease.write(0, 1).fill(7.0);
        lease.write(1, 1).fill(-9.0);
        let mut chain = BankChain::new(
            AoSoaScratch::new(effect_contract::BankWidth::Four, 5).unwrap(),
            Box::new([true, false, false, false]),
            vec![],
        )
        .unwrap();
        chain
            .run(
                &mut ArenaMembers {
                    lease: &mut lease,
                    inputs: &[1],
                    outputs: &[1],
                    fold: &[],
                    master: MasterPlanes::Arena(0),
                    sources: SourceGather::NONE,
                },
                3,
                71,
            )
            .unwrap();
        let view = chain.final_output_lane(3, 0).unwrap();
        for accepts in [
            vec![],
            vec![Some(false), Some(false)],
            vec![None, None],
            vec![Some(false), None, Some(false), None],
            vec![Some(true), None, Some(false)],
            vec![None, Some(true), Some(false)],
        ] {
            let trace = Arc::new(std::sync::Mutex::new(Vec::new()));
            let observers = accepts
                .iter()
                .enumerate()
                .map(|(handle, accept)| {
                    GraphNodeObserverBinding::new(
                        GraphNodeId::Output {
                            output_id: crate::StableGraphId::parse("out").unwrap(),
                        },
                        handle as u64,
                        Box::new(Observer {
                            handle: handle as u64,
                            accept: *accept,
                            trace: Arc::clone(&trace),
                        }),
                    )
                })
                .collect();
            let mut op = RuntimeOp {
                inputs: Box::new([]),
                staged: Box::new([]),
                sidechain: None,
                output: 1,
                kind: NodeKind::BankMember,
                split_pair: None,
                observers,
            };
            test_only_meter_input_reset(false);
            let result = observe(
                &mut op,
                &mut lease,
                71,
                Some(view),
                false,
                GraphObservationValidity::CLEAR,
            );
            let take = accepts
                .iter()
                .position(|a| *a == Some(true))
                .map_or(accepts.len(), |i| i + 1);
            assert_eq!(result.is_err(), accepts.contains(&Some(true)));
            let seen = &accepts[..take];
            assert_eq!(
                *trace.lock().unwrap(),
                seen.iter()
                    .enumerate()
                    .map(|(i, a)| (i as u64, a.is_some()))
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                test_only_meter_input_counts(),
                [
                    u64::from(seen.contains(&None)),
                    take as u64,
                    seen.iter().filter(|a| a.is_some()).count() as u64,
                ]
            );
        }
        test_only_meter_input_reset(false);
    }

    #[test]
    fn resident_meter_entry_has_one_final_output_dispatch_and_admission_control() {
        fn valid(source: &str) -> bool {
            let production = source.split("\n#[cfg(test)]\nmod tests {").next().unwrap();
            let observation = production
                .split("pub(crate) fn observe_unit(")
                .nth(1)
                .unwrap()
                .split(concat!("// REALTIME_POLICY_", "END"))
                .next()
                .unwrap();
            // Effect-state publication has an unrelated method with the same name.
            // Count calls only inside the meter-observer dispatcher, whose body ends
            // at the existing realtime-region boundary.
            let Some(dispatcher) = production
                .split_once("\nfn observe(\n")
                .and_then(|(_, body)| body.split_once(concat!("// REALTIME_POLICY_", "END")))
                .map(|(body, _)| body)
            else {
                return false;
            };
            dispatcher.matches(".observe_resident(").count() == 1
                && dispatcher.contains("if folded {")
                && dispatcher.contains("write_resident_lane(lease, op.output, words)?;")
                && production.matches(".final_output_lane(").count() == 1
                && [
                    "let Self { lease, units, .. } = self;",
                    "RuntimeUnit::Op(op) => observe(op, lease, first_sample, None, false, validity)",
                    "let eligible = population > 0",
                    "population <= width",
                    "!members.is_empty()",
                    "members.len().is_multiple_of(population)",
                    "chain.active().len() == width",
                    "*active == (lane < population)",
                    "chain.aux_lanes().is_empty()",
                    "u32::try_from(lease.frames()).ok()",
                    "members.len().checked_sub(population)",
                    "let chain: &BankChain = chain;",
                    "let resident = if eligible",
                    "index.checked_sub(start)",
                    "chain.final_output_lane(frames?, lane)",
                    "chain.fold_lanes().get(lane) == Some(&true)",
                    "observe(member, lease, first_sample, resident, folded, validity)?;",
                ]
                .iter()
                .all(|term| observation.contains(term))
        }
        let source = include_str!("runtime.rs");
        assert!(valid(source));
        for (from, to) in [
            (
                ".observe_resident(crate::GraphResidentObservationBlock",
                ".observe_other(crate::GraphResidentObservationBlock",
            ),
            (
                "write_resident_lane(lease, op.output, words)?;",
                "write_resident_lane(lease, op.output, words)?; observer.observe_resident(block);",
            ),
            ("let resident = if eligible", "let resident = if true"),
            ("index.checked_sub(start)", "Some(index)"),
            (
                "chain.final_output_lane(frames?, lane)",
                "chain.final_output_lane(1, lane)",
            ),
            (
                "chain.fold_lanes().get(lane) == Some(&true)",
                "chain.fold_lanes().get(lane) == Some(&false)",
            ),
            (
                "write_resident_lane(lease, op.output, words)?;",
                "write_resident_lane(lease, op.output, None).ok();",
            ),
            (
                "observe(member, lease, first_sample, resident, folded, validity)?;",
                "observe(member, lease, first_sample, resident, false, validity).ok();",
            ),
        ] {
            assert!(!valid(&source.replacen(from, to, 1)), "control: {from}");
        }
    }

    #[test]
    fn rt9_resident_entry_has_one_guarded_production_caller_and_control() {
        fn valid(runtime: &str, graph: &str, rack: &str) -> bool {
            let production = |source: &str| {
                source
                    .split("\n#[cfg(test)]\nmod tests {")
                    .next()
                    .unwrap()
                    .to_owned()
            };
            let runtime = production(runtime);
            let count = [&runtime, &production(graph), &production(rack)]
                .iter()
                .map(|source| source.matches(".run_with_resident_input(").count())
                .sum::<usize>();
            let execute = runtime
                .split("pub(crate) fn execute(")
                .nth(1)
                .unwrap()
                .split("pub(crate) fn complete_pending(")
                .next()
                .unwrap();
            let graph = production(graph);
            let Some(render) = graph
                .split("impl PreparedPlanExecutor for GraphExecutor {")
                .nth(1)
                .and_then(|implementation| implementation.split("    fn render(").nth(1))
                .and_then(|render| render.split("    fn qualification_counters(").next())
            else {
                return false;
            };
            // Issue #916 removed the end-of-block copy the loop body used to end at; the loop is
            // now the last thing in the render region.
            let Some(loop_body) = render
                .split("for unit in 0..runtime.units.len() {")
                .nth(1)
                .and_then(|body| body.split(concat!("// REALTIME_POLICY_", "END")).next())
            else {
                return false;
            };
            let expected = [
                "if let Err(error) = runtime.execute(unit, time.absolute_sample, host.reborrow(), sources) {",
                "return Err(error);",
                "if let Err(error) = runtime.observe_unit(unit, time.absolute_sample, source_validity, &host) {",
                "return Err(error);",
            ];
            let normalized = |source: &str| source.split_whitespace().collect::<String>();
            let normalized_loop = normalized(loop_body);
            let mut remaining = normalized_loop.as_str();
            for statement in expected {
                let statement = normalized(statement);
                let Some((_, tail)) = remaining.split_once(statement.as_str()) else {
                    return false;
                };
                remaining = tail;
            }
            count == 1
                && render.matches("runtime.execute(").count() == 1
                && render.matches("runtime.observe_unit(").count() == 1
                && execute.contains("let (before, current) = units.split_at_mut(index);")
                && execute.contains("let admitted = identity[index].resident_input;")
                && execute.contains("let predecessor = if admitted {")
                && execute.contains("if let Some(predecessor) = predecessor {")
                && execute.contains(
                    "chain.run_with_resident_input(predecessor, &mut planes, frames, first_sample)",
                )
        }
        let runtime = include_str!("runtime.rs");
        let graph = include_str!("lib.rs");
        let rack = include_str!("../../rack/src/lib.rs");
        assert!(
            valid(runtime, graph, rack),
            "sole resident call must remain behind graph admission"
        );
        let bypass = runtime.replacen(
            "let admitted = identity[index].resident_input;",
            "let admitted = true;",
            1,
        );
        assert!(!valid(&bypass, graph, rack), "admission-bypass control");
        let skipped_observer = graph.replacen(
            "runtime.observe_unit(unit, time.absolute_sample, source_validity, &host)",
            "Ok::<(), RenderError>(())",
            1,
        );
        assert!(
            !valid(runtime, &skipped_observer, rack),
            "freshness requires predecessor observation"
        );
        let continued_error = graph.replacen(
            "runtime.complete_pending(time.absolute_sample);\n                return Err(error);",
            "runtime.complete_pending(time.absolute_sample);\n                continue;",
            1,
        );
        assert!(
            !valid(runtime, &continued_error, rack),
            "producer failure must prevent successor execution"
        );
        let next_observer = graph.replacen(
            "runtime.observe_unit(unit, time.absolute_sample, source_validity, &host)",
            "runtime.observe_unit(unit + 1, time.absolute_sample, source_validity, &host)",
            1,
        );
        assert!(
            !valid(runtime, &next_observer, rack),
            "observation must be for the unit just executed"
        );
        let second = format!("unrelated.run_with_resident_input();\n{graph}");
        assert!(
            !valid(runtime, &second, rack),
            "second-production-call control"
        );
    }

    #[test]
    fn rt9_final_adjacency_declines_scalar_and_shorter_populations_but_skips_retired_runs() {
        let op = |input, output| RuntimeOp {
            inputs: vec![input].into_boxed_slice(),
            staged: Box::new([]),
            sidechain: None,
            output,
            kind: NodeKind::BankMember,
            split_pair: None,
            observers: Box::new([]),
        };
        let bank = |population, width, predecessor: bool| {
            let members = (0..population)
                .map(|lane| {
                    op(
                        if predecessor { 0 } else { lane + 1 },
                        if predecessor { lane + 1 } else { lane + 9 },
                    )
                })
                .collect::<Vec<_>>();
            RuntimeUnit::Bank {
                members: members.into_boxed_slice(),
                lanes: population as usize,
                chain: BankChain::new(
                    AoSoaScratch::new(width, 17).expect("scratch"),
                    (0..width.lanes()).map(|lane| lane < population).collect(),
                    vec![],
                )
                .expect("chain"),
                fold: Box::new([]),
                master: FoldTarget::Arena(0),
            }
        };
        let identity = |population| UnitIdentity {
            banked: true,
            resident_input: false,
            observed: false,
            source_lanes: 0,
            stages: 1,
            upstream_of_seam_stages: 1,
            lane_tracks: (0..population)
                .map(|lane| format!("track{lane}").into_boxed_str())
                .collect(),
        };
        // Cases: ordinary adjacency; retired non-emitted run; intervening emitted scalar;
        // smaller/larger predecessor populations; incompatible widths; scalar predecessor.
        for case in 0..7 {
            let a = if case == 3 {
                1
            } else if case == 4 {
                3
            } else {
                2
            };
            let b = 2;
            let mut units = vec![if case == 6 {
                RuntimeUnit::Op(op(0, 1))
            } else {
                bank(
                    a,
                    if case == 5 {
                        effect_contract::BankWidth::Eight
                    } else {
                        effect_contract::BankWidth::Four
                    },
                    true,
                )
            }];
            let mut rows = vec![identity(a)];
            let mut runs = vec![(vec![], (0..a as usize).collect::<Vec<_>>())];
            let mut emitted = vec![Some(0)];
            if case == 1 || case == 2 {
                runs.push((vec![], vec![a as usize]));
                if case == 2 {
                    emitted.push(Some(1));
                    units.push(RuntimeUnit::Op(op(0, 8)));
                    rows.push(identity(1));
                } else {
                    emitted.push(None);
                }
            }
            emitted.push(Some(units.len()));
            units.push(bank(b, effect_contract::BankWidth::Four, false));
            rows.push(identity(b));
            runs.push((
                vec![],
                (a as usize + 1..a as usize + 1 + b as usize).collect(),
            ));
            let count = a + 1 + b;
            let program = ExecutionProgram {
                ops: (0..count)
                    .map(|index| Op {
                        node: index,
                        level: 0,
                        inputs: (index, index + 1),
                        sidechain: None,
                        output: BufferRef(index + 1),
                        in_place: false,
                    })
                    .collect(),
                inputs: (0..count)
                    .map(|_| InputRef {
                        buffer: BufferRef(1),
                        delay: None,
                    })
                    .collect(),
                delays: Box::new([]),
                node_buffer: Box::new([]),
                node_op: Box::new([]),
                taps: Box::new([]),
                buffers: count + 1,
                output: BufferRef(count),
            };
            let first_producer: Vec<_> = (0..count)
                .map(|index| (index > a).then(|| (index - a - 1) as usize))
                .collect();
            arm_resident_inputs(
                &program,
                &first_producer,
                &runs,
                &emitted,
                &units,
                &mut rows,
            );
            assert_eq!(
                rows.last().expect("successor").resident_input,
                case <= 1,
                "case={case}"
            );
        }
    }

    #[test]
    fn rt9_identity_metadata_has_no_retained_or_peak_layout_delta() {
        struct Before {
            _banked: bool,
            _stages: u32,
            _upstream: u32,
            _tracks: Box<[Box<str>]>,
        }
        assert_eq!(
            core::mem::size_of::<UnitIdentity>(),
            core::mem::size_of::<Before>()
        );
        assert_eq!(
            core::mem::align_of::<UnitIdentity>(),
            core::mem::align_of::<Before>()
        );
        // build_sequential retains the same vector capacity and boxes it once; no separate
        // resident table, per-block allocation, or transient acquisition buffer is introduced.
    }

    /// Issue #900: the constructor derives every unit's observed flag from the observer slices
    /// the unit actually holds, whatever the caller wrote -- each placeholder below is the wrong
    /// answer -- and `observe_unit` then walks exactly the units that hold one, including a bank
    /// whose only observer sits on its last member. Forcing the unconditional walk back on is the
    /// control that the counters see the skipped units.
    #[test]
    fn observed_flag_is_derived_at_construction_and_skips_only_unobserved_units() {
        struct Visit(Arc<AtomicUsize>);
        impl crate::GraphRuntimeObserver for Visit {
            fn observe(&mut self, _block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
                self.0.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        }
        let visits = Arc::new(AtomicUsize::new(0));
        let observers = |count: usize| -> Box<[GraphNodeObserverBinding]> {
            (0..count)
                .map(|handle| {
                    GraphNodeObserverBinding::new(
                        GraphNodeId::Output {
                            output_id: crate::StableGraphId::parse("out").expect("id"),
                        },
                        handle as u64,
                        Box::new(Visit(Arc::clone(&visits))),
                    )
                })
                .collect()
        };
        let op = |observers| RuntimeOp {
            inputs: Box::new([]),
            staged: Box::new([]),
            sidechain: None,
            output: ARENA_BASE,
            kind: NodeKind::BankMember,
            split_pair: None,
            observers,
        };
        let bank = |observed_member: Option<usize>| RuntimeUnit::Bank {
            members: (0..4)
                .map(|member| op(observers(usize::from(observed_member == Some(member)))))
                .collect(),
            lanes: 4,
            chain: BankChain::new(
                AoSoaScratch::new(effect_contract::BankWidth::Four, 2).expect("scratch"),
                Box::new([true; 4]),
                vec![],
            )
            .expect("chain"),
            fold: Box::new([]),
            master: FoldTarget::Arena(0),
        };
        let row = |observed| UnitIdentity {
            banked: false,
            resident_input: false,
            observed,
            source_lanes: 0,
            stages: 1,
            upstream_of_seam_stages: 0,
            lane_tracks: Box::new([]),
        };
        let mut runtime = Runtime::new(
            stereo_lease(2, 1),
            Vec::new(),
            Vec::new(),
            vec![
                RuntimeUnit::Op(op(observers(0))),
                RuntimeUnit::Op(op(observers(2))),
                bank(Some(3)),
                bank(None),
            ],
            Vec::new(),
            vec![row(true), row(false), row(false), row(true)],
            Vec::new(),
            0,
            0,
        );
        assert_eq!(
            runtime
                .identity
                .iter()
                .map(|row| row.observed)
                .collect::<Vec<_>>(),
            [false, true, true, false]
        );
        // Per unit: [observe calls, observer accesses, bank member accesses, observer visits].
        for (skip_disabled, expected) in [
            (
                false,
                [[0, 0, 0, 0], [1, 2, 0, 2], [4, 1, 4, 1], [0, 0, 0, 0]],
            ),
            (
                true,
                [[1, 0, 0, 0], [1, 2, 0, 2], [4, 1, 4, 1], [4, 0, 4, 0]],
            ),
        ] {
            // A hand-built runtime has no Output unit, so the host's planes are never read here.
            let (mut left, mut right) = ([0.0_f32; 2], [0.0_f32; 2]);
            let host = HostMaster::new(&mut left, &mut right, 2).expect("host planes");
            for (unit, expected) in expected.into_iter().enumerate() {
                test_only_observe_calls_reset(skip_disabled);
                test_only_observation_dispatch_reset();
                visits.store(0, Ordering::Relaxed);
                assert_eq!(
                    runtime.observe_unit(unit, 5, GraphObservationValidity::CLEAR, &host),
                    Ok(())
                );
                let [observers, members] = test_only_observation_dispatch_counts();
                assert_eq!(
                    [
                        test_only_observe_calls(),
                        observers,
                        members,
                        visits.load(Ordering::Relaxed) as u64,
                    ],
                    expected,
                    "unit {unit}, unconditional walk {skip_disabled}"
                );
            }
        }
        test_only_observe_calls_reset(false);
    }

    #[test]
    fn synthetic_distinct_matrix_destination_is_the_scalar_pair_identity_decline() {
        // This is a deliberately synthetic lowered program. It isolates the defensive identity
        // gate; #476 owns the separate question of whether production lowering can emit it.
        let mut program = ExecutionProgram {
            ops: vec![
                Op {
                    node: 0,
                    level: 0,
                    inputs: (0, 1),
                    sidechain: None,
                    output: BufferRef(1),
                    in_place: true,
                },
                Op {
                    node: 1,
                    level: 1,
                    inputs: (1, 2),
                    sidechain: None,
                    output: BufferRef(2),
                    in_place: false,
                },
            ]
            .into_boxed_slice(),
            inputs: vec![
                InputRef {
                    buffer: BufferRef(1),
                    delay: None,
                },
                InputRef {
                    buffer: BufferRef(1),
                    delay: None,
                },
            ]
            .into_boxed_slice(),
            delays: Box::new([]),
            node_buffer: vec![BufferRef(1), BufferRef(2)].into_boxed_slice(),
            node_op: vec![Some(0), Some(1)].into_boxed_slice(),
            taps: Box::new([]),
            buffers: 3,
            output: BufferRef(2),
        };
        assert!(
            !scalar_pair_is_in_place(&program, 0, 1),
            "an otherwise valid single undelayed edge declines on its distinct destination"
        );

        program.ops[1].output = BufferRef(1);
        program.ops[1].in_place = true;
        program.node_buffer[1] = BufferRef(1);
        assert!(
            scalar_pair_is_in_place(&program, 0, 1),
            "the same fixture admits only after the exact identity facts are restored"
        );
        program.inputs[1].delay = Some(DelayRef {
            line: 0,
            staging: BufferRef(2),
        });
        assert!(
            !scalar_pair_is_in_place(&program, 0, 1),
            "the explicit undelayed-input guard remains independent"
        );

        struct FaderOwner(Arc<AtomicUsize>);
        impl GraphRuntimeProcessor for FaderOwner {
            fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                self.0.fetch_add(1, Ordering::Relaxed);
                for sample in block.left.iter_mut().chain(block.right.iter_mut()) {
                    *sample *= 2.0;
                }
                Ok(())
            }
        }
        struct FailingMatrixOwner {
            calls: Arc<AtomicUsize>,
            queued: Arc<AtomicUsize>,
        }
        impl GraphRuntimeProcessor for FailingMatrixOwner {
            fn process(&mut self, _: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                self.calls.fetch_add(1, Ordering::Relaxed);
                self.queued.fetch_sub(1, Ordering::Relaxed);
                Err(RenderError::InvalidEnvelope)
            }
        }

        // Execute the original declined shape. The later reduction copies into its distinct
        // destination before its owner reports the first error; the fader source remains the
        // completed earlier state. This is the boundary a one-buffer early composite cannot own.
        let fader_calls = Arc::new(AtomicUsize::new(0));
        let matrix_calls = Arc::new(AtomicUsize::new(0));
        let queued = Arc::new(AtomicUsize::new(2));
        let mut fader = RuntimeOp {
            inputs: vec![ARENA_BASE].into_boxed_slice(),
            staged: Box::new([]),
            sidechain: None,
            output: ARENA_BASE,
            kind: NodeKind::Bound(Box::new(FaderOwner(Arc::clone(&fader_calls)))),
            split_pair: None,
            observers: Box::new([]),
        };
        let mut matrix = RuntimeOp {
            inputs: vec![ARENA_BASE].into_boxed_slice(),
            staged: Box::new([]),
            sidechain: None,
            output: ARENA_BASE + 1,
            kind: NodeKind::Bound(Box::new(FailingMatrixOwner {
                calls: Arc::clone(&matrix_calls),
                queued: Arc::clone(&queued),
            })),
            split_pair: None,
            observers: Box::new([]),
        };
        let mut lease = stereo_lease(2, 2);
        lease
            .write_stereo(ARENA_BASE)
            .0
            .copy_from_slice(&[0.25, -0.5]);
        lease
            .write_stereo(ARENA_BASE)
            .1
            .copy_from_slice(&[-0.75, 1.0]);
        lease.write_stereo(ARENA_BASE + 1).0.fill(91.0);
        lease.write_stereo(ARENA_BASE + 1).1.fill(-91.0);
        execute_op(
            &mut fader,
            &mut lease,
            &mut [],
            &mut [],
            &mut [],
            0,
            None,
            &[],
            OutputSources::NONE,
        )
        .expect("earlier fader");
        assert_eq!(
            execute_op(
                &mut matrix,
                &mut lease,
                &mut [],
                &mut [],
                &mut [],
                0,
                None,
                &[],
                OutputSources::NONE,
            ),
            Err(RenderError::InvalidEnvelope)
        );
        assert_eq!(lease.read_stereo(ARENA_BASE).0, &[0.5, -1.0]);
        assert_eq!(lease.read_stereo(ARENA_BASE).1, &[-1.5, 2.0]);
        assert_eq!(lease.read_stereo(ARENA_BASE + 1).0, &[0.5, -1.0]);
        assert_eq!(lease.read_stereo(ARENA_BASE + 1).1, &[-1.5, 2.0]);
        assert_eq!(fader_calls.load(Ordering::Relaxed), 1);
        assert_eq!(matrix_calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            queued.load(Ordering::Relaxed),
            1,
            "later failure retains its next record"
        );
    }

    #[test]
    fn split_pair_completion_walks_pending_faders_before_error_return() {
        struct Probe {
            completions: Arc<AtomicUsize>,
        }
        impl GraphRuntimeSplitPairProcessor for Probe {
            fn begin_fader(&mut self, _: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
            fn finish_matrix(&mut self, _: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
            fn complete_pending(&mut self, block: GraphBindingBlock<'_>) {
                self.completions.fetch_add(1, Ordering::Relaxed);
                block.left.fill(0.75);
                block.right.fill(-0.25);
            }
        }

        let completions = Arc::new(AtomicUsize::new(0));
        let mut runtime = Runtime::new(
            stereo_lease(2, 2),
            Vec::new(),
            Vec::new(),
            vec![RuntimeUnit::Op(RuntimeOp {
                inputs: Box::new([]),
                staged: Box::new([]),
                sidechain: None,
                output: ARENA_BASE,
                kind: NodeKind::Identity,
                split_pair: Some(SplitPairSlot {
                    pair: 0,
                    role: SplitPairRole::Fader,
                }),
                observers: Box::new([]),
            })],
            vec![Box::new(Probe {
                completions: Arc::clone(&completions),
            })],
            vec![UnitIdentity {
                banked: false,
                resident_input: false,
                observed: false,
                source_lanes: 0,
                stages: 1,
                upstream_of_seam_stages: 0,
                lane_tracks: Box::new([]),
            }],
            Vec::new(),
            0,
            0,
        );
        runtime.complete_pending(11);
        assert_eq!(completions.load(Ordering::Relaxed), 1);
        assert_eq!(runtime.buffer(ARENA_BASE).0, &[0.75, 0.75]);
        assert_eq!(runtime.buffer(ARENA_BASE).1, &[-0.25, -0.25]);
    }

    struct DecliningPairOwner(Arc<AtomicUsize>, bool);
    fn decline_pair(
        left: crate::BuiltinProcessor,
        right: crate::BuiltinProcessor,
    ) -> Result<crate::BuiltinProcessor, (crate::BuiltinProcessor, crate::BuiltinProcessor)> {
        Err((left, right))
    }
    impl GraphPreparedBuiltinBankProcessor for DecliningPairOwner {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn pair_factory(&self) -> Option<crate::BuiltinPairFactory> {
            Some(if self.1 { accept_pair } else { decline_pair })
        }
        fn process(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            _: u32,
            _: u64,
        ) -> Result<(), RenderError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            for sample in left.iter_mut().chain(right.iter_mut()) {
                *sample += 1.0;
            }
            Ok(())
        }
    }
    struct AcceptedPair(crate::BuiltinProcessor, crate::BuiltinProcessor);
    fn accept_pair(
        left: crate::BuiltinProcessor,
        right: crate::BuiltinProcessor,
    ) -> Result<crate::BuiltinProcessor, (crate::BuiltinProcessor, crate::BuiltinProcessor)> {
        Ok(Box::new(AcceptedPair(left, right)))
    }
    impl GraphPreparedBuiltinBankProcessor for AcceptedPair {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn process(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            frames: u32,
            sample: u64,
        ) -> Result<(), RenderError> {
            self.0.process(left, right, frames, sample)?;
            self.1.process(left, right, frames, sample)
        }
    }
    struct PlainPairOwner(Arc<AtomicUsize>);
    impl GraphPreparedBuiltinBankProcessor for PlainPairOwner {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn process(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            _: u32,
            _: u64,
        ) -> Result<(), RenderError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            for sample in left.iter_mut().chain(right.iter_mut()) {
                *sample *= 2.0;
            }
            Ok(())
        }
    }

    #[test]
    fn prepared_fold_retains_first_slots_scratch_through_pair_success_and_decline() {
        for accepted in [false, true] {
            for folded in [false, true] {
                let track = crate::StableGraphId::parse("decline").expect("id");
                let fader = GraphNodeId::TrackStage {
                    track_id: track.clone(),
                    stage: TrackStage::PostFader,
                };
                let matrix = GraphNodeId::TrackStage {
                    track_id: track,
                    stage: TrackStage::PostMatrix,
                };
                let spec = GraphSpec {
                    nodes: vec![
                        crate::GraphNode {
                            id: fader.clone(),
                            latency: effect_contract::LatencySamples(0),
                            tail: effect_contract::TailSamples::Finite(0),
                        },
                        crate::GraphNode {
                            id: matrix.clone(),
                            latency: effect_contract::LatencySamples(0),
                            tail: effect_contract::TailSamples::Finite(0),
                        },
                    ],
                    ports: Vec::new(),
                    edges: Vec::new(),
                };
                let bank = |member, processor: Box<dyn GraphPreparedBuiltinBankProcessor>| {
                    GraphPreparedBuiltinBank {
                        backend: lane::Backend::Simd4,
                        members: vec![member].into_boxed_slice(),
                        processor,
                        scratch: AoSoaScratch::new(effect_contract::BankWidth::Four, 8)
                            .expect("scratch"),
                    }
                };
                let first_calls = Arc::new(AtomicUsize::new(0));
                let second_calls = Arc::new(AtomicUsize::new(0));
                let mut parts = RuntimeParts::new(
                    &spec,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![
                        bank(
                            fader,
                            Box::new(DecliningPairOwner(Arc::clone(&first_calls), accepted)),
                        ),
                        bank(matrix, Box::new(PlainPairOwner(Arc::clone(&second_calls)))),
                    ],
                    Vec::new(),
                    Vec::new(),
                    Default::default(),
                    Vec::new(),
                    8,
                );
                let configuration = folded.then(|| {
                    rack::PreparedFoldConfiguration::new(
                        BankWidth::Four,
                        vec![true, false, false, false].into_boxed_slice(),
                        vec![true, false, false, false].into_boxed_slice(),
                    )
                    .expect("borrowed shape configuration")
                });
                let mut chain = parts.chain_for_with_fold(
                    &[Membership::Builtin(0), Membership::Builtin(1)],
                    1,
                    configuration,
                );
                assert_eq!(chain.width(), BankWidth::Four);
                assert_eq!(
                    chain.fold_lanes(),
                    if folded {
                        &[true, false, false, false][..]
                    } else {
                        &[]
                    }
                );
                assert!(
                    parts.builtin_banks.iter().all(Option::is_none),
                    "both original owners moved once"
                );
                const FRAMES: usize = 2;
                let mut lease = stereo_lease(FRAMES, 3);
                lease.write_stereo(1).0.copy_from_slice(&[1.0, 2.0]);
                lease.write_stereo(1).1.copy_from_slice(&[-1.0, -2.0]);
                let fold = [FoldLane {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                    store: true,
                }];
                let mut members = ArenaMembers {
                    lease: &mut lease,
                    inputs: &[1],
                    outputs: &[2],
                    fold: if folded { &fold } else { &[] },
                    master: MasterPlanes::Arena(2),
                    sources: SourceGather::NONE,
                };
                chain
                    .run(&mut members, FRAMES as u32, 0)
                    .expect("declined chain render");
                assert_eq!(
                    first_calls.load(Ordering::Relaxed),
                    1,
                    "first returned owner executes"
                );
                assert_eq!(
                    second_calls.load(Ordering::Relaxed),
                    1,
                    "second returned owner executes"
                );
                assert_eq!(members.lease.read_stereo(2).0, &[4.0, 6.0]);
                assert_eq!(members.lease.read_stereo(2).1, &[0.0, -2.0]);
            }
        }
    }

    /// The node's cached witness and the line's own answer are the same fact (#210 phase 2).
    ///
    /// `NodeKind::channel_symmetry` is asked without the delay lines in hand, so the lowering
    /// caches the verdict on the variant. This is what keeps the cache and the rings from drifting.
    #[test]
    fn the_node_witness_agrees_with_its_line() {
        for (left, right) in [(0_usize, 0_usize), (1, 1), (480, 480), (0, 1), (480, 481)] {
            let line = TrackDelayLine::new(left, right);
            assert_eq!(line.lane_samples(), [left, right]);
            assert_eq!(line.channels_agree(), left == right);
            let kind = NodeKind::TrackDelay {
                line: 0,
                channels_agree: line.channels_agree(),
            };
            assert_eq!(
                kind.channel_symmetry().eligible(),
                left == right,
                "the op witness for {left}/{right}"
            );
        }
    }

    /// A lane's ring is exactly a pure `N`-sample shift, whatever the block partitioning.
    ///
    /// Blocks shorter than, equal to and longer than the ring, and a ring of one -- the three cases
    /// `delay_lane`'s take loop exists for. FP-free by construction: the kernel swaps words, so
    /// this is an exact equality over a sequence that includes signed zeros and a NaN, none of
    /// which a ring is allowed to alter.
    #[test]
    fn a_delay_line_is_a_pure_shift_at_every_partitioning() {
        for delay in [1_usize, 3, 8, 32] {
            for block in [1_usize, 4, 8, 31, 64] {
                let source: Vec<f32> = (0..128)
                    .map(|index| match index {
                        5 => -0.0,
                        9 => f32::NAN,
                        other => other as f32 + 0.5,
                    })
                    .collect();
                let mut line = TrackDelayLine::new(delay, delay);
                let mut got = Vec::new();
                for chunk in source.chunks(block) {
                    let mut left = chunk.to_vec();
                    let mut right = chunk.to_vec();
                    line.process(&mut left, &mut right);
                    assert_eq!(
                        left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                        "equal-length lanes must stay in step"
                    );
                    got.extend(left);
                }
                for (index, value) in got.iter().enumerate() {
                    let want = if index < delay {
                        0.0_f32
                    } else {
                        source[index - delay]
                    };
                    assert_eq!(
                        value.to_bits(),
                        want.to_bits(),
                        "delay {delay}, block {block}, sample {index}"
                    );
                }
            }
        }
    }

    /// Deterministic 32-bit LCG, frozen here so the corpora do not depend on host RNG state.
    fn lcg(state: &mut u32) -> f32 {
        *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        f32::from(((*state >> 16) & 0xffff) as i16) / 3_276.8
    }

    /// Lays `inputs` out as one lease over `inputs.len() + 1` buffers and reduces into the first.
    fn reduce_case(frames: usize, inputs: &[Vec<f32>]) -> Vec<f32> {
        let mut lease = single_lease(frames, inputs.len() + 1);
        let refs: Vec<u32> = (2..=inputs.len() as u32 + 1).collect();
        // The output slot starts at a sentinel, never at zero: a reduction that forgets to write
        // it -- the fan-in-zero fill in particular -- must not be able to pass by accident.
        lease.write(0, 1).fill(f32::from_bits(0x7f7f_7f7f));
        for (index, input) in inputs.iter().enumerate() {
            lease.write(0, refs[index]).copy_from_slice(input);
        }
        reduce_plane(&mut lease, 0, 1, &refs);
        lease.read(0, 1).to_vec()
    }

    fn old_reduce_case(frames: usize, inputs: &[Vec<f32>]) -> Vec<f32> {
        let mut lease = single_lease(frames, inputs.len() + 1);
        let refs: Vec<u32> = (2..=inputs.len() as u32 + 1).collect();
        lease.write(0, 1).fill(f32::from_bits(0x7f7f_7f7f));
        for (index, input) in inputs.iter().enumerate() {
            lease.write(0, refs[index]).copy_from_slice(input);
        }
        old_reduce_plane(&mut lease, 0, 1, &refs);
        lease.read(0, 1).to_vec()
    }

    /// Frozen pre-RT-3 oracle: the old two-kernel left-associated reduction.
    fn old_reduce_plane(lease: &mut ArenaLease, plane: usize, out: u32, inputs: &[u32]) {
        match inputs {
            [] => lease.write(plane, out).fill(0.0),
            [single] => {
                if *single != out {
                    let (output, input) = lease.write_read(plane, out, *single);
                    output.copy_from_slice(input);
                }
            }
            [first, second, rest @ ..] => {
                {
                    let (output, a, b) = lease.write_read2(plane, out, *first, *second);
                    sum2_block::<FrameLane>(output, a, b);
                }
                for next in rest {
                    let (output, input) = lease.write_read(plane, out, *next);
                    sum_into_block::<FrameLane>(output, input);
                }
            }
        }
    }

    fn assert_width_matches_old<L: Lane>() {
        #[derive(Clone, Copy, Debug)]
        enum Family {
            Finite,
            NegativeZero,
            SmallNormal,
            Subnormal,
            Infinity,
            Nan,
        }

        let _fp_env = lane::fpenv::CanonicalFpEnv::enter();
        for frames in [
            1,
            L::WIDTH.saturating_sub(1).max(1),
            L::WIDTH,
            L::WIDTH + 1,
            L::WIDTH * 3 + 1,
            128,
        ] {
            for family in [
                Family::Finite,
                Family::NegativeZero,
                Family::SmallNormal,
                Family::Subnormal,
                Family::Infinity,
                Family::Nan,
            ] {
                let inputs: Vec<Vec<f32>> = (0..9)
                    .map(|input| {
                        (0..frames)
                            .map(|frame| match family {
                                Family::Finite => match (frame % 2, input) {
                                    (0, 0) => 2.0,
                                    (0, 1) => -0.5,
                                    (0, 2) => 0.25,
                                    (1, 0) => 16_777_216.0,
                                    (1, 1) => 1.0,
                                    (1, 2) => -16_777_216.0,
                                    _ => 0.0,
                                },
                                Family::NegativeZero => -0.0,
                                Family::SmallNormal => {
                                    if input < 2 {
                                        f32::MIN_POSITIVE
                                    } else {
                                        0.0
                                    }
                                }
                                Family::Subnormal => {
                                    if input < 2 {
                                        f32::from_bits(1)
                                    } else {
                                        0.0
                                    }
                                }
                                Family::Infinity => match (frame % 2, input) {
                                    (0, 0) => f32::INFINITY,
                                    (0, 1) => 1.0,
                                    (1, 0) => f32::NEG_INFINITY,
                                    (1, 1) => -1.0,
                                    _ => 0.0,
                                },
                                Family::Nan => {
                                    if input == 0 {
                                        f32::from_bits(0x7fc0_4201)
                                    } else {
                                        0.0
                                    }
                                }
                            })
                            .collect()
                    })
                    .collect();
                let mut actual = single_lease(frames, 11);
                let mut old = single_lease(frames, 11);
                let ids: Vec<u32> = (2..11).collect();
                for (index, values) in inputs.iter().enumerate() {
                    actual.write(0, ids[index]).copy_from_slice(values);
                    old.write(0, ids[index]).copy_from_slice(values);
                }
                reduce_many::<L>(&mut actual, 0, 1, &ids);
                {
                    let (output, first, second) = old.write_read2(0, 1, ids[0], ids[1]);
                    sum2_block::<L>(output, first, second);
                }
                for id in &ids[2..] {
                    let (output, input) = old.write_read(0, 1, *id);
                    sum_into_block::<L>(output, input);
                }
                for (frame, expected) in old.read(0, 1).iter().enumerate() {
                    let expected_bits = match family {
                        Family::Finite if frame % 2 == 0 => 1.75_f32.to_bits(),
                        Family::Finite => 0.0_f32.to_bits(),
                        Family::NegativeZero => (-0.0_f32).to_bits(),
                        Family::SmallNormal => 0x0100_0000,
                        Family::Subnormal => 2,
                        Family::Infinity if frame % 2 == 0 => f32::INFINITY.to_bits(),
                        Family::Infinity => f32::NEG_INFINITY.to_bits(),
                        Family::Nan => {
                            assert!(expected.is_nan(), "old NaN family output at frame {frame}");
                            expected.to_bits()
                        }
                    };
                    assert_eq!(
                        expected.to_bits(),
                        expected_bits,
                        "old {family:?} category at width {} frames {frames} frame {frame}",
                        L::WIDTH
                    );
                }
                assert_eq!(
                    actual
                        .read(0, 1)
                        .iter()
                        .map(|x| x.to_bits())
                        .collect::<Vec<_>>(),
                    old.read(0, 1)
                        .iter()
                        .map(|x| x.to_bits())
                        .collect::<Vec<_>>(),
                    "{family:?} width {} frames {frames}",
                    L::WIDTH
                );
            }
        }
    }

    #[test]
    fn every_lane_width_matches_the_frozen_old_kernel_on_hostile_values() {
        assert_width_matches_old::<f32>();
        assert_width_matches_old::<lane::Simd4>();
        assert_width_matches_old::<lane::Simd8>();
    }

    /// Frozen pre-#898 oracle: `reduce_many` exactly as it stood before issue #898, re-deriving
    /// every input's arena slice once per vector. It is the "before" side of the before/after gate.
    fn frozen_per_vector_reduce_many<L: Lane>(
        lease: &mut ArenaLease,
        plane: usize,
        out: u32,
        first: u32,
        second: u32,
        rest: &[u32],
    ) {
        let frames = lease.frames();
        let vectored = frames - frames % L::WIDTH;
        let mut index = 0;
        while index < vectored {
            let mut acc = {
                let source = lease.read(plane, first);
                L::load(&source[index..])
            };
            for input in std::iter::once(second).chain(rest.iter().copied()) {
                let value = {
                    let source = lease.read(plane, input);
                    L::load(&source[index..])
                };
                acc = acc.add(value);
            }
            acc.store(&mut lease.write(plane, out)[index..]);
            index += L::WIDTH;
        }
        while index < frames {
            let mut acc = <f32 as lane::Lane>::load(&lease.read(plane, first)[index..]);
            for input in std::iter::once(second).chain(rest.iter().copied()) {
                let value = <f32 as lane::Lane>::load(&lease.read(plane, input)[index..]);
                acc = acc.add(value);
            }
            acc.store(&mut lease.write(plane, out)[index..]);
            index += 1;
        }
    }

    /// The one NaN payload the #898 corpus uses; see `hoisting_word`.
    const HOISTING_NAN: u32 = 0x7fc0_4898;

    /// Xorshift32, frozen here so the #898 corpus does not depend on host RNG state.
    fn hoisting_draw(state: &mut u32) -> u32 {
        *state ^= *state << 13;
        *state ^= *state >> 17;
        *state ^= *state << 5;
        *state
    }

    /// One #898 corpus word. Most are finite, with magnitudes spread over `2^-24..2^25` so that
    /// association shows in the rounding. About one word in 170 is a signed zero, a subnormal,
    /// `infinity` or the one NaN payload.
    fn hoisting_word(state: &mut u32, infinity: f32) -> f32 {
        let draw = hoisting_draw(state);
        let sign = draw & 0x8000_0000;
        let mantissa = hoisting_draw(state) & 0x007f_ffff;
        match (draw >> 8) % 1024 {
            0 | 1 => f32::from_bits(sign),
            2 | 3 => f32::from_bits(sign | mantissa),
            4 => infinity,
            5 => f32::from_bits(HOISTING_NAN),
            _ => f32::from_bits(sign | (127 - 24 + (draw >> 18) % 49) << 23 | mantissa),
        }
    }

    /// Both planes of one reduction into buffer 1 of a fresh stereo lease whose buffers `2..` hold
    /// `contents`, as bit patterns. The output starts at a sentinel, and every input buffer must
    /// come out exactly as it went in.
    fn hoisting_reduction_bits(
        frames: usize,
        contents: &[[Vec<f32>; 2]],
        reduce: impl Fn(&mut ArenaLease, usize),
    ) -> [Vec<u32>; 2] {
        let mut lease = stereo_lease(frames, contents.len() + 1);
        for plane in 0..2 {
            lease.write(plane, 1).fill(f32::from_bits(0x7f7f_7f7f));
            for (index, words) in contents.iter().enumerate() {
                lease
                    .write(plane, index as u32 + 2)
                    .copy_from_slice(&words[plane]);
            }
        }
        for plane in 0..2 {
            reduce(&mut lease, plane);
        }
        let bits = |words: &[f32]| words.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
        for (index, words) in contents.iter().enumerate() {
            for (plane, plane_words) in words.iter().enumerate() {
                assert_eq!(
                    bits(lease.read(plane, index as u32 + 2)),
                    bits(plane_words),
                    "input buffer {} plane {plane} was written",
                    index + 2
                );
            }
        }
        [bits(lease.read(0, 1)), bits(lease.read(1, 1))]
    }

    /// One width of the #898 gate: the hoisted kernel against the frozen per-vector one.
    fn assert_hoisting_matches_frozen<L: Lane>(
        frames: usize,
        contents: &[[Vec<f32>; 2]],
        ids: &[u32],
        reference: &[Vec<u32>; 2],
    ) {
        let hoisted = hoisting_reduction_bits(frames, contents, |lease, plane| {
            reduce_many::<L>(lease, plane, 1, ids);
        });
        let frozen = hoisting_reduction_bits(frames, contents, |lease, plane| {
            frozen_per_vector_reduce_many::<L>(lease, plane, 1, ids[0], ids[1], &ids[2..]);
        });
        let context = format!(
            "width {} fan-in {} frames {frames} ids {ids:?}",
            L::WIDTH,
            ids.len()
        );
        assert_eq!(hoisted, frozen, "hoisted vs frozen, {context}");
        assert_eq!(
            &hoisted, reference,
            "hoisted vs scalar reference, {context}"
        );
    }

    /// Issue #898 gate 1: random N-input reductions are bit-identical before and after the arena
    /// slices were hoisted out of the vector loop.
    ///
    /// "Before" is `frozen_per_vector_reduce_many`, the pre-#898 kernel kept verbatim. The scalar
    /// left-to-right `reduce` is the D9 definition both must equal. Every lane width runs, and so
    /// does the production `reduce_plane`. Fan-in is random in `2..=64` and also pinned at every
    /// group edge up to 65. Block lengths are random, including non-multiples of every lane
    /// width. Edge lists repeat buffers and name the silence buffer, and both planes differ.
    /// Every NaN in the corpus carries one payload and each case uses infinities of one sign, so
    /// no expected bit depends on which operand of an add the compiler puts first.
    ///
    /// Red mutations: start each group after the first from a fresh subtotal, or take the groups
    /// in reverse order. The test checks that its own corpus tells both apart from the reference.
    #[test]
    fn random_fan_in_reductions_are_bit_identical_before_and_after_slice_hoisting() {
        use engine::realtime::ARENA_SILENCE_BUFFER;
        let _fp_env = lane::fpenv::CanonicalFpEnv::enter();
        let mut state = 0x0898_5eed_u32;
        let mut cases: Vec<(usize, usize)> = [2, 7, 8, 9, 15, 16, 17, 63, 64, 65]
            .into_iter()
            .flat_map(|fan_in| [(fan_in, 1), (fan_in, 13), (fan_in, 128)])
            .collect();
        for _ in 0..120 {
            let fan_in = 2 + hoisting_draw(&mut state) as usize % 63;
            let frames = 1 + hoisting_draw(&mut state) as usize % 131;
            cases.push((fan_in, frames));
        }
        let (mut fresh_subtotal_differs, mut reversed_groups_differ) = (0_usize, 0_usize);
        for (case, &(fan_in, frames)) in cases.iter().enumerate() {
            let infinity = if case % 2 == 0 {
                f32::INFINITY
            } else {
                f32::NEG_INFINITY
            };
            let contents: Vec<[Vec<f32>; 2]> = (0..fan_in)
                .map(|_| {
                    [0, 1].map(|_| {
                        (0..frames)
                            .map(|_| hoisting_word(&mut state, infinity))
                            .collect()
                    })
                })
                .collect();
            // Buffer 1 is the output and `2..=fan_in + 1` hold `contents`; about one edge in
            // sixteen reads the silence buffer, and the rest pick a content buffer with repeats.
            let ids: Vec<u32> = (0..fan_in)
                .map(|_| {
                    let draw = hoisting_draw(&mut state);
                    if draw.is_multiple_of(16) {
                        ARENA_SILENCE_BUFFER
                    } else {
                        2 + (draw >> 4) % fan_in as u32
                    }
                })
                .collect();
            let word = |plane: usize, id: u32, frame: usize| {
                if id == ARENA_SILENCE_BUFFER {
                    0.0_f32
                } else {
                    contents[id as usize - 2][plane][frame]
                }
            };
            let chain = |plane: usize, frame: usize, ids: &mut dyn Iterator<Item = u32>| {
                ids.map(|id| word(plane, id, frame))
                    .reduce(|a, b| a + b)
                    .expect("a non-empty edge list")
            };
            let reference: [Vec<u32>; 2] = [0, 1].map(|plane| {
                (0..frames)
                    .map(|frame| chain(plane, frame, &mut ids.iter().copied()).to_bits())
                    .collect()
            });
            for (plane, expected_plane) in reference.iter().enumerate() {
                for (frame, &expected) in expected_plane.iter().enumerate() {
                    let fresh = ids
                        .chunks(REDUCE_GROUP)
                        .map(|group| chain(plane, frame, &mut group.iter().copied()))
                        .reduce(|a, b| a + b)
                        .expect("a non-empty edge list");
                    let reversed = chain(
                        plane,
                        frame,
                        &mut ids.chunks(REDUCE_GROUP).rev().flatten().copied(),
                    );
                    fresh_subtotal_differs += usize::from(fresh.to_bits() != expected);
                    reversed_groups_differ += usize::from(reversed.to_bits() != expected);
                }
            }

            assert_hoisting_matches_frozen::<f32>(frames, &contents, &ids, &reference);
            assert_hoisting_matches_frozen::<lane::Simd4>(frames, &contents, &ids, &reference);
            assert_hoisting_matches_frozen::<lane::Simd8>(frames, &contents, &ids, &reference);
            let production = hoisting_reduction_bits(frames, &contents, |lease, plane| {
                reduce_plane(lease, plane, 1, &ids);
            });
            assert_eq!(
                production, reference,
                "reduce_plane vs scalar reference, fan-in {fan_in} frames {frames} ids {ids:?}"
            );
        }
        assert!(
            fresh_subtotal_differs > 0,
            "the corpus cannot tell a fresh subtotal per group from the chain"
        );
        assert!(
            reversed_groups_differ > 0,
            "the corpus cannot tell reversed groups from the chain"
        );
    }

    /// Issue #898: a fan-in of all `-0.0` stays `-0.0` across every group edge. A later group
    /// that restarted from a `0.0` fill, rather than from the stored running sum, would return
    /// `+0.0` here.
    #[test]
    fn negative_zero_survives_every_reduction_group_edge() {
        for fan_in in [2_usize, 8, 9, 16, 17, 64, 65] {
            for frames in [1_usize, 5, 8, 13] {
                let contents: Vec<[Vec<f32>; 2]> = (0..fan_in)
                    .map(|_| [vec![-0.0; frames], vec![-0.0; frames]])
                    .collect();
                let ids: Vec<u32> = (2..=fan_in as u32 + 1).collect();
                let reduced = hoisting_reduction_bits(frames, &contents, |lease, plane| {
                    reduce_plane(lease, plane, 1, &ids);
                });
                assert_eq!(
                    reduced,
                    [
                        vec![(-0.0_f32).to_bits(); frames],
                        vec![(-0.0_f32).to_bits(); frames]
                    ],
                    "fan-in {fan_in} frames {frames}"
                );
            }
        }
    }

    #[test]
    fn reduction_preserves_repeated_silence_self_and_unrelated_buffers() {
        const FRAMES: usize = 5;
        let build = || stereo_lease(FRAMES, 6);
        let mut actual = build();
        let mut old = build();
        for lease in [&mut actual, &mut old] {
            lease
                .write(0, 2)
                .copy_from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0]);
            lease
                .write(1, 2)
                .copy_from_slice(&[-1.0, -2.0, -3.0, -4.0, -5.0]);
            lease.write(0, 3).fill(99.0);
            lease.write(1, 3).fill(-99.0);
            lease.write(0, 4).fill(0.5);
            lease.write(1, 4).fill(-0.25);
            lease.write(0, 1).fill(f32::from_bits(0x7fc0_4202));
            lease.write(1, 1).fill(f32::from_bits(0xffc0_4202));
            lease.write(0, 5).fill(f32::from_bits(0x7fc0_4203));
            lease.write(1, 5).fill(f32::from_bits(0xffc0_4203));
        }
        let ids = [2, 2, 0, 3, 4];
        for plane in 0..2 {
            reduce_plane(&mut actual, plane, 1, &ids);
            old_reduce_plane(&mut old, plane, 1, &ids);
        }
        for plane in 0..2 {
            assert_eq!(
                actual
                    .read(plane, 1)
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                old.read(plane, 1)
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
            assert!(
                actual
                    .read(plane, 5)
                    .iter()
                    .all(|x| x.to_bits() == if plane == 0 { 0x7fc0_4203 } else { 0xffc0_4203 })
            );
        }
        let mut self_alias = single_lease(2, 2);
        self_alias
            .write(0, 2)
            .copy_from_slice(&[-0.0, f32::from_bits(0x7fc0_4204)]);
        reduce_plane(&mut self_alias, 0, 2, &[2]);
        reduce_plane(&mut self_alias, 0, 2, &[2]);
        assert_eq!(
            self_alias
                .read(0, 2)
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
            vec![(-0.0f32).to_bits(), 0x7fc0_4204]
        );
    }

    /// One lease that owns `buffers` buffers of one plane, as the sequential executor does.
    fn single_lease(frames: usize, buffers: usize) -> ArenaLease {
        let mut builder = ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(1).expect("one plane"),
            NonZeroUsize::new(frames).expect("frames"),
        );
        let owned: Vec<u32> = (0..buffers).map(|_| builder.reserve()).collect();
        builder.lease(0, owned.clone(), owned);
        let (_arena, mut leases) = builder.finish().expect("one disjoint lease");
        leases.pop().expect("the lease")
    }

    /// One stereo lease over `buffers` buffers, the shape `build_sequential` builds.
    fn stereo_lease(frames: usize, buffers: usize) -> ArenaLease {
        let mut builder = ArenaLeaseSetBuilder::new(
            NonZeroUsize::new(2).expect("stereo planes"),
            NonZeroUsize::new(frames).expect("frames"),
        );
        let owned: Vec<u32> = (0..buffers).map(|_| builder.reserve()).collect();
        builder.lease(0, owned.clone(), owned);
        let (_arena, mut leases) = builder.finish().expect("one disjoint lease");
        leases.pop().expect("the lease")
    }

    /// The compatibility callback is deliberately unusable here: a regression to per-lane
    /// dispatch must fail rather than quietly producing the same sum.
    ///
    /// Since issue #915 this probe declines the resident-fold offer (its `fold_resident` keeps
    /// the trait default), so what it pins is the **staged fallback** path through
    /// `fold_cohort`. The fused production path is pinned by
    /// `a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit`, whose counters
    /// assert which fold each arm took.
    #[test]
    fn all_active_folded_bank_chain_dispatches_the_real_graph_cohort() {
        struct Identity;
        impl BankStage for Identity {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
        }
        struct Probe<'a> {
            inner: ArenaMembers<'a>,
            cohorts: usize,
        }
        impl BankMembers for Probe<'_> {
            fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
                self.inner.plane(lane)
            }
            fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
                self.inner.plane_mut(lane)
            }
            fn fold_plane(&mut self, _lane: usize, _left: &mut [f32], _right: &mut [f32]) {
                panic!("cohort dispatch regressed to fold_plane")
            }
            fn fold_cohort(&mut self, cohort: FoldCohort<'_>) {
                self.cohorts += 1;
                self.inner.fold_cohort(cohort);
            }
        }
        const FRAMES: usize = 2;
        let mut lease = stereo_lease(FRAMES, 10);
        let inputs = [2, 3, 4, 5];
        let outputs = [6, 7, 8, 9];
        let coefficients = [1.0, 0.0, 0.0, 1.0];
        for (lane, input) in inputs.iter().copied().enumerate() {
            let (left, right) = lease.write_stereo(input);
            left.fill(lane as f32 + 1.0);
            right.fill(-(lane as f32 + 1.0));
        }
        let mut oracle_lease = stereo_lease(FRAMES, 5);
        let oracle_routes = [
            ARENA_BASE + 1,
            ARENA_BASE + 2,
            ARENA_BASE + 3,
            ARENA_BASE + 4,
        ];
        for (lane, route) in oracle_routes.iter().copied().enumerate() {
            let (left, right) = oracle_lease.write_stereo(route);
            left.fill(lane as f32 + 1.0);
            right.fill(-(lane as f32 + 1.0));
            mix2x2_block::<FrameLane>(left, right, coefficients);
        }
        old_reduce_plane(&mut oracle_lease, 0, ARENA_BASE, &oracle_routes);
        old_reduce_plane(&mut oracle_lease, 1, ARENA_BASE, &oracle_routes);
        let (oracle_left, oracle_right) = oracle_lease.read_stereo(ARENA_BASE);
        let expected_left = oracle_left.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        let expected_right = oracle_right.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        let fold: Vec<FoldLane> = (0..4)
            .map(|lane| FoldLane {
                coefficients,
                store: lane == 0,
            })
            .collect();
        let active = vec![true; 4].into_boxed_slice();
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, FRAMES as u32).expect("scratch"),
            active.clone(),
            vec![BankSlot {
                stage: Box::new(Identity),
                active_lanes: active.clone(),
            }],
        )
        .expect("chain");
        chain.arm_fold(active).expect("fold mask");
        let mut members = Probe {
            inner: ArenaMembers {
                lease: &mut lease,
                inputs: &inputs,
                outputs: &outputs,
                fold: &fold,
                master: MasterPlanes::Arena(ARENA_BASE),
                sources: SourceGather::NONE,
            },
            cohorts: 0,
        };
        chain.run(&mut members, FRAMES as u32, 0).expect("run");
        assert_eq!(members.cohorts, 1);
        let (left, right) = members.inner.lease.read_stereo(ARENA_BASE);
        assert_eq!(
            left.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            expected_left
        );
        assert_eq!(
            right.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            expected_right
        );
    }

    /// RT-1: real graph arena members gather one buffer set and scatter directly into another.
    #[test]
    fn arena_members_direct_scatter_preserves_redirected_identity_bits() {
        struct Identity;
        impl BankStage for Identity {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
        }

        const FRAMES: usize = 11;
        let mut lease = stereo_lease(FRAMES, 8);
        let inputs = [1, 2, 3, 4];
        let outputs = [5, 6, 7, 8];
        let mut expected = Vec::new();
        for (lane, input) in inputs.iter().copied().enumerate() {
            let left: Vec<f32> = (0..FRAMES)
                .map(|frame| f32::from_bits(0x7fc0_0000 | ((lane * FRAMES + frame) as u32 + 1)))
                .collect();
            let right: Vec<f32> = left
                .iter()
                .map(|word| f32::from_bits(word.to_bits() ^ 0x8000_3990))
                .collect();
            lease.write(0, input).copy_from_slice(&left);
            lease.write(1, input).copy_from_slice(&right);
            expected.push((left, right));
        }
        for output in outputs {
            lease.write(0, output).fill(f32::from_bits(0x7f80_3991));
            lease.write(1, output).fill(f32::from_bits(0x7f80_3992));
        }

        let active = vec![true; 4].into_boxed_slice();
        let mut chain = BankChain::new(
            AoSoaScratch::new(BankWidth::Four, FRAMES as u32).expect("scratch"),
            active.clone(),
            vec![BankSlot {
                stage: Box::new(Identity),
                active_lanes: active,
            }],
        )
        .expect("chain");
        let mut members = ArenaMembers {
            lease: &mut lease,
            inputs: &inputs,
            outputs: &outputs,
            fold: &[],
            master: MasterPlanes::Arena(0),
            sources: SourceGather::NONE,
        };
        chain
            .run(&mut members, FRAMES as u32, 0)
            .expect("direct graph run");

        for (lane, output) in outputs.iter().copied().enumerate() {
            let (left, right) = lease.read_stereo(output);
            assert_eq!(
                left.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
                expected[lane]
                    .0
                    .iter()
                    .map(|word| word.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                right.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
                expected[lane]
                    .1
                    .iter()
                    .map(|word| word.to_bits())
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(chain.transposes(), 1);
    }

    /// The folded epilogue is the route op followed by the D9 reduction, word for word.
    ///
    /// The oracle is the shape this replaces, built out of the very kernels the route op and
    /// `reduce_plane` call: `mix2x2_block` per contributor into its own buffer, then
    /// `sum2_block(first, second)` and `sum_into_block` for the rest, left to right. Nothing here
    /// is a restatement of the epilogue's arithmetic -- both sides are the production kernels, and
    /// what is under test is that the epilogue puts them in the same order.
    ///
    /// Block lengths that are not a multiple of the lane width are included, because the epilogue
    /// hands `mix2x2_block` a *staging* slice where the route op handed it an arena buffer: if the
    /// two lengths could differ the vector/tail split would differ with them, and the bits with it.
    ///
    /// Red mutations: accumulate the first contributor instead of storing it -- the `-0.0` case
    /// below fails; reverse the lane order -- the multi-contributor cases fail.
    #[test]
    fn a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit() {
        let coefficients = [
            [0.5_f32, -0.25, 0.125, 0.75],
            [1.0, 0.0, 0.0, 1.0],
            [-0.3, 0.9, 0.4, -0.6],
        ];
        for frames in [1_usize, 3, 7, 63, 64, 65, 128] {
            let mut state = 0x0bad_f00du32;
            let tiles: Vec<(Vec<f32>, Vec<f32>)> = (0..coefficients.len())
                .map(|_| {
                    (
                        (0..frames).map(|_| lcg(&mut state)).collect(),
                        (0..frames).map(|_| lcg(&mut state)).collect(),
                    )
                })
                .collect();

            // The oracle: one route buffer per contributor, then the D9 left-to-right reduction.
            let mut lease = stereo_lease(frames, coefficients.len() + 1);
            // The arena reserves buffer zero as the always-zero silence slot.
            let master = ARENA_BASE;
            let routes: Vec<u32> =
                (ARENA_BASE + 1..=ARENA_BASE + coefficients.len() as u32).collect();
            for (index, buffer) in routes.iter().enumerate() {
                let (left, right) = lease.write_stereo(*buffer);
                left.copy_from_slice(&tiles[index].0);
                right.copy_from_slice(&tiles[index].1);
                mix2x2_block::<FrameLane>(left, right, coefficients[index]);
            }
            old_reduce_plane(&mut lease, 0, master, &routes);
            old_reduce_plane(&mut lease, 1, master, &routes);
            let (oracle_left, oracle_right) = lease.read_stereo(master);
            let oracle: (Vec<u32>, Vec<u32>) = (
                oracle_left.iter().map(|value| value.to_bits()).collect(),
                oracle_right.iter().map(|value| value.to_bits()).collect(),
            );

            // The epilogue: the same tiles, in an opening cohort and a continuation cohort.
            let mut folded_lease = stereo_lease(frames, 1);
            let fold: Vec<FoldLane> = coefficients
                .iter()
                .enumerate()
                .map(|(index, coefficients)| FoldLane {
                    coefficients: *coefficients,
                    store: index == 0,
                })
                .collect();
            let mut members = ArenaMembers {
                lease: &mut folded_lease,
                inputs: &[],
                outputs: &[],
                fold: &fold,
                master: MasterPlanes::Arena(master),
                sources: SourceGather::NONE,
            };
            let mut staged_left: Vec<f32> = tiles
                .iter()
                .flat_map(|tile| tile.0.iter().copied())
                .collect();
            let mut staged_right: Vec<f32> = tiles
                .iter()
                .flat_map(|tile| tile.1.iter().copied())
                .collect();
            members.fold_cohort(
                FoldCohort::new(&[0, 1], &mut staged_left, &mut staged_right, frames, frames)
                    .expect("valid opening cohort"),
            );
            members.fold_cohort(
                FoldCohort::new(&[2], &mut staged_left, &mut staged_right, frames, frames)
                    .expect("valid continuation cohort"),
            );
            let (folded_left, folded_right) = folded_lease.read_stereo(master);
            assert_eq!(
                (
                    folded_left
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    folded_right
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>()
                ),
                oracle,
                "{frames} frames: the epilogue is not the route plus the reduction"
            );
        }
    }

    #[test]
    fn a_later_folded_cohort_continues_from_the_live_master_in_d9_order() {
        for frames in [1_usize, 3, 8, 11] {
            let master = ARENA_BASE;
            let fold = [
                FoldLane {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                    store: true,
                },
                FoldLane {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                    store: false,
                },
                FoldLane {
                    coefficients: [1.0, 0.0, 0.0, 1.0],
                    store: false,
                },
            ];
            let mut lease = stereo_lease(frames, 1);
            let mut left = [
                vec![16_777_216.0; frames],
                vec![1.0; frames],
                vec![-16_777_216.0; frames],
            ]
            .concat();
            let mut right = [
                vec![-16_777_216.0; frames],
                vec![-1.0; frames],
                vec![16_777_216.0; frames],
            ]
            .concat();
            let mut members = ArenaMembers {
                lease: &mut lease,
                inputs: &[],
                outputs: &[],
                fold: &fold,
                master: MasterPlanes::Arena(master),
                sources: SourceGather::NONE,
            };
            members.fold_cohort(
                FoldCohort::new(&[0], &mut left, &mut right, frames, frames)
                    .expect("opening cohort"),
            );
            members.fold_cohort(
                FoldCohort::new(&[1, 2], &mut left, &mut right, frames, frames)
                    .expect("continuation cohort"),
            );
            let (actual_left, actual_right) = lease.read_stereo(master);
            assert!(
                actual_left
                    .iter()
                    .all(|sample| sample.to_bits() == 0.0_f32.to_bits())
            );
            assert!(
                actual_right
                    .iter()
                    .all(|sample| sample.to_bits() == 0.0_f32.to_bits())
            );
            assert_eq!(
                (16_777_216.0_f32 + (1.0 - 16_777_216.0)).to_bits(),
                1.0_f32.to_bits()
            );
        }
    }

    #[test]
    fn malformed_folded_cohorts_are_rejected_before_route_or_master_mutation() {
        const FRAMES: usize = 4;
        let fold = [
            FoldLane {
                coefficients: [2.0, 0.0, 0.0, 2.0],
                store: true,
            },
            FoldLane {
                coefficients: [3.0, 0.0, 0.0, 3.0],
                store: true,
            },
        ];
        for ids in [&[0, 1][..], &[2][..]] {
            let mut lease = stereo_lease(FRAMES, 1);
            let (master_left, master_right) = lease.write_stereo(ARENA_BASE);
            master_left.fill(19.0);
            master_right.fill(-23.0);
            let mut left = vec![5.0_f32; FRAMES * 3];
            let mut right = vec![-7.0_f32; FRAMES * 3];
            let before_left: Vec<u32> = left.iter().map(|x| x.to_bits()).collect();
            let before_right: Vec<u32> = right.iter().map(|x| x.to_bits()).collect();
            let mut members = ArenaMembers {
                lease: &mut lease,
                inputs: &[],
                outputs: &[],
                fold: &fold,
                master: MasterPlanes::Arena(ARENA_BASE),
                sources: SourceGather::NONE,
            };
            members.fold_cohort(
                FoldCohort::new(ids, &mut left, &mut right, FRAMES, FRAMES)
                    .expect("representable invalid graph metadata"),
            );
            assert_eq!(
                left.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                before_left
            );
            assert_eq!(
                right.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                before_right
            );
            let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
            assert!(
                master_left
                    .iter()
                    .all(|x| x.to_bits() == 19.0_f32.to_bits())
            );
            assert!(
                master_right
                    .iter()
                    .all(|x| x.to_bits() == (-23.0_f32).to_bits())
            );
        }

        let mut lease = stereo_lease(FRAMES, 1);
        let (master_left, master_right) = lease.write_stereo(ARENA_BASE);
        master_left.fill(29.0);
        master_right.fill(-31.0);
        let mut left = vec![11.0_f32; FRAMES + 1];
        let mut right = vec![-13.0_f32; FRAMES + 1];
        let mut members = ArenaMembers {
            lease: &mut lease,
            inputs: &[],
            outputs: &[],
            fold: &fold,
            master: MasterPlanes::Arena(ARENA_BASE),
            sources: SourceGather::NONE,
        };
        members.fold_cohort(
            FoldCohort::new(&[0], &mut left, &mut right, FRAMES + 1, FRAMES + 1)
                .expect("shape is valid at the public boundary"),
        );
        assert!(left.iter().all(|x| x.to_bits() == 11.0_f32.to_bits()));
        assert!(right.iter().all(|x| x.to_bits() == (-13.0_f32).to_bits()));
        let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
        assert!(
            master_left
                .iter()
                .all(|x| x.to_bits() == 29.0_f32.to_bits())
        );
        assert!(
            master_right
                .iter()
                .all(|x| x.to_bits() == (-31.0_f32).to_bits())
        );
    }

    /// SplitMix64, frozen here so issue #915's corpora do not depend on host RNG state.
    fn splitmix(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A hostile finite sample: `+0.0` and `-0.0`, signed subnormals, and signed normals whose
    /// magnitude spans `2^-24 .. 2^25`, each with a random mantissa. Finite by construction, so
    /// every sum below stays finite and a mismatch is a rounding or an order, never a NaN payload.
    fn hostile_sample(state: &mut u64) -> f32 {
        let bits = splitmix(state);
        let sign = ((bits >> 63) as u32) << 31;
        let mantissa = (bits as u32) & 0x007f_ffff;
        match (bits >> 32) % 8 {
            0 => f32::from_bits(sign),
            1 => f32::from_bits(sign | mantissa.max(1)),
            _ => {
                // Biased exponents 103..=151 are the unbiased -24..=24.
                let exponent = 103 + ((bits >> 40) % 49) as u32;
                f32::from_bits(sign | (exponent << 23) | mantissa)
            }
        }
    }

    /// A hostile route constant: a signed zero one time in eight, otherwise a signed normal whose
    /// magnitude spans `2^-4 .. 2^2` -- a bind-folded gain times a pan law, with no structure.
    fn hostile_constant(state: &mut u64) -> f32 {
        let bits = splitmix(state);
        let sign = ((bits >> 63) as u32) << 31;
        if (bits >> 32).is_multiple_of(8) {
            return f32::from_bits(sign);
        }
        // Biased exponents 123..=128 are the unbiased -4..=1.
        let exponent = 123 + ((bits >> 40) % 6) as u32;
        f32::from_bits(sign | (exponent << 23) | ((bits as u32) & 0x007f_ffff))
    }

    /// Gate 1 of issue #915: a fully folded full bank's master is the staged path's master, bit
    /// for bit, whether the chain folds it from the resident block or through the staging block.
    ///
    /// Both arms run the **real** chain -- the same gather, the same stage, the same scatter entry
    /// -- over one arena layout and identical words. They differ only in whether the members accept
    /// the resident offer: the fused arm hands it to `ArenaMembers::fold_resident`, the staged arm
    /// declines it, so the chain takes the transpose into staging and `ArenaMembers::fold_cohort`
    /// it has always taken. The staged arm is therefore the production path before this issue,
    /// not a restatement of it.
    ///
    /// Shapes: a full W4 bank of four and a full W8 bank of eight; one cohort, and two cohorts into
    /// one master (the second continues from the first's live master); `frames` of exactly one
    /// tile, 13 and 16 (ragged tails at both widths, and none), and 128; the first contributor
    /// storing or the whole bank accumulating onto a live prior master. Samples are hostile --
    /// signed zeros, subnormals, magnitudes over `2^-24 .. 2^25` -- and every lane gets its own
    /// random 2x2. The chain's scratch is sized for 128 frames, so on every shorter block the
    /// staged path's stride is not the block length and the resident view is a strict prefix.
    ///
    /// Also pinned: the fused arm really took the offer (counted), the staged arm really took
    /// `fold_cohort` (counted), no lane's own buffer is written by either, and the master moved.
    /// That the staging block is left unwritten is observable only inside rack, so it is rack's
    /// `a_fully_folded_full_bank_offers_its_resident_block_before_writing_staging`.
    ///
    /// Red mutations (crates/graph/tests/MUTATIONS.md, issue #915): swap the coefficient roles,
    /// accumulate the lanes in reverse, seed a continuation from zero, skip the ragged tail, and
    /// return `true` having written nothing.
    #[test]
    fn a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit() {
        let _canonical = lane::fpenv::CanonicalFpEnv::enter();
        struct Identity;
        impl BankStage for Identity {
            fn process(&mut self, _block: BankBlock<'_>) -> Result<(), RenderError> {
                Ok(())
            }
        }
        /// `ArenaMembers`, counting which fold the chain took; `fused == false` declines the offer.
        struct Arm<'a> {
            inner: ArenaMembers<'a>,
            fused: bool,
            counts: [usize; 3],
        }
        impl BankMembers for Arm<'_> {
            fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
                self.inner.plane(lane)
            }
            fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
                self.inner.plane_mut(lane)
            }
            fn fold_plane(&mut self, _lane: usize, _left: &mut [f32], _right: &mut [f32]) {
                panic!("a fully folded full bank never folds lane by lane")
            }
            fn fold_cohort(&mut self, cohort: FoldCohort<'_>) {
                self.counts[2] += 1;
                self.inner.fold_cohort(cohort);
            }
            fn fold_resident(&mut self, cohort: ResidentFoldCohort<'_>) -> bool {
                self.counts[0] += 1;
                if !self.fused {
                    return false;
                }
                let accepted = self.inner.fold_resident(cohort);
                self.counts[1] += usize::from(accepted);
                accepted
            }
        }

        const QUANTUM: u32 = 128;
        const OUTPUT_POISON: u32 = 0x7fc0_3917;
        let bits = |words: &[f32]| words.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
        let mut state = 0x0915_f05e_d0e1_1095_u64;
        for width in [BankWidth::Four, BankWidth::Eight] {
            let lanes = width.lanes() as usize;
            for frames in [lanes, 13, 16, 128] {
                for cohorts in [1_usize, 2] {
                    for initial_store in [true, false] {
                        let what = format!(
                            "{width:?}, {frames} frames, {cohorts} cohort(s), store {initial_store}"
                        );
                        let tracks = cohorts * lanes;
                        let mut plane = || -> Vec<f32> {
                            (0..frames).map(|_| hostile_sample(&mut state)).collect()
                        };
                        let inputs: Vec<(Vec<f32>, Vec<f32>)> =
                            (0..tracks).map(|_| (plane(), plane())).collect();
                        let prior = (plane(), plane());
                        let fold: Vec<FoldLane> = (0..tracks)
                            .map(|track| FoldLane {
                                coefficients: core::array::from_fn(|_| {
                                    hostile_constant(&mut state)
                                }),
                                store: initial_store && track == 0,
                            })
                            .collect();
                        let master = ARENA_BASE;
                        let input_ids: Vec<u32> = (0..tracks)
                            .map(|track| ARENA_BASE + 1 + track as u32)
                            .collect();
                        let output_ids: Vec<u32> = (0..tracks)
                            .map(|track| ARENA_BASE + 1 + (tracks + track) as u32)
                            .collect();
                        let render = |fused: bool| {
                            let mut lease =
                                stereo_lease(frames, ARENA_BASE as usize + 1 + 2 * tracks);
                            for (buffer, (left, right)) in input_ids.iter().zip(&inputs) {
                                let (to_left, to_right) = lease.write_stereo(*buffer);
                                to_left.copy_from_slice(left);
                                to_right.copy_from_slice(right);
                            }
                            for buffer in &output_ids {
                                let (left, right) = lease.write_stereo(*buffer);
                                left.fill(f32::from_bits(OUTPUT_POISON));
                                right.fill(f32::from_bits(OUTPUT_POISON));
                            }
                            let (master_left, master_right) = lease.write_stereo(master);
                            master_left.copy_from_slice(&prior.0);
                            master_right.copy_from_slice(&prior.1);
                            let mut counts = [0_usize; 3];
                            for cohort in 0..cohorts {
                                let members = cohort * lanes..(cohort + 1) * lanes;
                                let active = vec![true; lanes].into_boxed_slice();
                                let mut chain = BankChain::new(
                                    AoSoaScratch::new(width, QUANTUM).expect("scratch"),
                                    active.clone(),
                                    vec![BankSlot {
                                        stage: Box::new(Identity),
                                        active_lanes: active.clone(),
                                    }],
                                )
                                .expect("chain");
                                chain.arm_fold(active).expect("fold every lane");
                                let mut arm = Arm {
                                    inner: ArenaMembers {
                                        lease: &mut lease,
                                        inputs: &input_ids[members.clone()],
                                        outputs: &output_ids[members.clone()],
                                        fold: &fold[members],
                                        master: MasterPlanes::Arena(master),
                                        sources: SourceGather::NONE,
                                    },
                                    fused,
                                    counts: [0; 3],
                                };
                                chain.run(&mut arm, frames as u32, 0).expect("run");
                                for (total, count) in counts.iter_mut().zip(arm.counts) {
                                    *total += count;
                                }
                            }
                            let (master_left, master_right) = lease.read_stereo(master);
                            let outputs: Vec<u32> = output_ids
                                .iter()
                                .flat_map(|buffer| {
                                    let (left, right) = lease.read_stereo(*buffer);
                                    bits(left).into_iter().chain(bits(right))
                                })
                                .collect();
                            (bits(master_left), bits(master_right), outputs, counts)
                        };
                        let (fused_left, fused_right, fused_outputs, fused_counts) = render(true);
                        let (staged_left, staged_right, staged_outputs, staged_counts) =
                            render(false);
                        assert_eq!(
                            fused_counts,
                            [cohorts, cohorts, 0],
                            "{what}: every cohort must take the resident fold"
                        );
                        assert_eq!(
                            staged_counts,
                            [cohorts, 0, cohorts],
                            "{what}: the oracle must take the staged fold_cohort"
                        );
                        assert_eq!(fused_left, staged_left, "{what}: left master");
                        assert_eq!(fused_right, staged_right, "{what}: right master");
                        assert_ne!(
                            (&fused_left, &fused_right),
                            (&bits(&prior.0), &bits(&prior.1)),
                            "{what}: the master must move, or the comparison is vacuous"
                        );
                        for outputs in [&fused_outputs, &staged_outputs] {
                            assert!(
                                outputs.iter().all(|word| *word == OUTPUT_POISON),
                                "{what}: a folded lane's own buffer must not be written"
                            );
                        }
                    }
                }
            }
        }
    }

    /// Every premise of `ArenaMembers::fold_resident` is checked before its first write: a broken
    /// one declines -- so the chain falls back to the staged path -- and leaves the master exactly
    /// as it was. The first case is the control that accepts, so every other case declines for its
    /// own premise and not for a shared one.
    #[test]
    fn a_resident_fold_declines_before_writing_on_a_broken_premise() {
        const FRAMES: usize = 13;
        const POISON: [u32; 2] = [0x7fc0_3918, 0x7fc0_3919];
        for width in [BankWidth::Four, BankWidth::Eight] {
            let lanes = width.lanes() as usize;
            let left: Vec<f32> = (0..FRAMES * lanes).map(|word| word as f32 + 0.5).collect();
            let right: Vec<f32> = left.iter().map(|word| -word).collect();
            let cohort = ResidentFoldCohort::new(&left, &right, width, FRAMES).expect("cohort");
            let fold_lane = |store| FoldLane {
                coefficients: [0.75, -0.25, 0.5, 1.25],
                store,
            };
            let full: Vec<FoldLane> = (0..lanes).map(|lane| fold_lane(lane == 0)).collect();
            let mut later_store = full.clone();
            later_store[1].store = true;
            let cases: [(&str, &[FoldLane], usize, u32); 6] = [
                ("control", &full, FRAMES, ARENA_BASE),
                (
                    "one fold entry short",
                    &full[..lanes - 1],
                    FRAMES,
                    ARENA_BASE,
                ),
                ("an unfolded chain", &[], FRAMES, ARENA_BASE),
                ("a later lane stores", &later_store, FRAMES, ARENA_BASE),
                (
                    "a block longer than the lease",
                    &full,
                    FRAMES - 1,
                    ARENA_BASE,
                ),
                ("a master outside the write set", &full, FRAMES, 99),
            ];
            for (case, fold, lease_frames, master) in cases {
                let mut lease = stereo_lease(lease_frames, 2);
                let (master_left, master_right) = lease.write_stereo(ARENA_BASE);
                master_left.fill(f32::from_bits(POISON[0]));
                master_right.fill(f32::from_bits(POISON[1]));
                let taken = ArenaMembers {
                    lease: &mut lease,
                    inputs: &[],
                    outputs: &[],
                    fold,
                    master: MasterPlanes::Arena(master),
                    sources: SourceGather::NONE,
                }
                .fold_resident(cohort);
                let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
                let untouched = master_left.iter().all(|word| word.to_bits() == POISON[0])
                    && master_right.iter().all(|word| word.to_bits() == POISON[1]);
                if case == "control" {
                    assert!(taken, "{width:?}: the control must accept");
                    assert!(!untouched, "{width:?}: the control must write the master");
                } else {
                    assert!(!taken, "{width:?}: {case} must decline");
                    assert!(untouched, "{width:?}: {case} wrote before declining");
                }
            }
        }
    }

    /// The fused fold's first contributor **stores**, exactly as `fold_cohort`'s does: a master
    /// whose every summand is `-0.0` stays `-0.0`, and a continuation reads the live master.
    ///
    /// Identity constants and `-0.0` samples make every routed word `-0.0` (`0 * -0` and `1 * -0`
    /// are both `-0`, and `-0 + -0` is `-0`), in every tile and in the ragged tail. Seeding the
    /// running value from `+0.0` instead of storing would give `+0.0`, which is invisible to a
    /// hostile-random differential and is why this is an absolute property rather than one.
    ///
    /// Red mutations: seed the first contributor from zero and add it (the `fold(0.0, +)` shape);
    /// seed a continuation from zero instead of the live master.
    #[test]
    fn a_resident_fold_stores_its_first_contributor_so_a_negative_zero_master_keeps_its_sign() {
        let _canonical = lane::fpenv::CanonicalFpEnv::enter();
        const FRAMES: usize = 13;
        for width in [BankWidth::Four, BankWidth::Eight] {
            let lanes = width.lanes() as usize;
            let zeros = vec![-0.0_f32; FRAMES * lanes];
            let cohort = ResidentFoldCohort::new(&zeros, &zeros, width, FRAMES).expect("cohort");
            for (store, start, expected) in [
                (true, 0.0_f32, 0x8000_0000_u32),
                (false, -0.0, 0x8000_0000),
                (false, 0.0, 0x0000_0000),
            ] {
                let fold: Vec<FoldLane> = (0..lanes)
                    .map(|lane| FoldLane {
                        coefficients: [1.0, 0.0, 0.0, 1.0],
                        store: store && lane == 0,
                    })
                    .collect();
                let mut lease = stereo_lease(FRAMES, 2);
                let (master_left, master_right) = lease.write_stereo(ARENA_BASE);
                master_left.fill(start);
                master_right.fill(start);
                assert!(
                    ArenaMembers {
                        lease: &mut lease,
                        inputs: &[],
                        outputs: &[],
                        fold: &fold,
                        master: MasterPlanes::Arena(ARENA_BASE),
                        sources: SourceGather::NONE,
                    }
                    .fold_resident(cohort)
                );
                let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
                for frame in 0..FRAMES {
                    assert_eq!(
                        (master_left[frame].to_bits(), master_right[frame].to_bits()),
                        (expected, expected),
                        "{width:?}, store {store}, start {start}: frame {frame}"
                    );
                }
            }
        }
    }

    /// The first contributor **stores**: a master whose only summand is `-0.0` stays `-0.0`.
    ///
    /// This is the law correction 1 of the plan states, and it is an absolute property rather than
    /// a comparison, because both arms of a differential move together under the mutation that
    /// breaks it. Zero-filling the master and accumulating every contributor computes
    /// `0.0 + (-0.0)`, which is `+0.0`; `sum2_block` computes `t0 + t1` directly, and a fan-in-one
    /// reduction is a *copy*, which is why `reduce_plane` has never been `fold(0.0, +)` either.
    ///
    /// Red mutation: replace the `store` arm's `copy_from_slice` with `sum_into_block` -- the
    /// master is `+0.0` (bits 0) where `-0.0` (bits 0x8000_0000) is required.
    #[test]
    fn the_first_contributor_stores_so_a_negative_zero_master_keeps_its_sign() {
        let _canonical = lane::fpenv::CanonicalFpEnv::enter();
        const FRAMES: usize = 9;
        let mut lease = stereo_lease(FRAMES, 1);
        // The arena starts at `+0.0`, which is exactly the value a zero-fill would leave.
        assert!(
            lease
                .read_stereo(ARENA_BASE)
                .0
                .iter()
                .all(|value| *value == 0.0)
        );
        let fold = [FoldLane {
            // Identity, so the route stage cannot itself manufacture a sign: `0.0 * (-0.0)` is
            // `-0.0` and `-0.0 + -0.0` is `-0.0`.
            coefficients: [1.0, 0.0, 0.0, 1.0],
            store: true,
        }];
        let mut members = ArenaMembers {
            lease: &mut lease,
            inputs: &[],
            outputs: &[],
            fold: &fold,
            master: MasterPlanes::Arena(ARENA_BASE),
            sources: SourceGather::NONE,
        };
        let mut left = vec![-0.0_f32; FRAMES];
        let mut right = vec![-0.0_f32; FRAMES];
        members.fold_plane(0, &mut left, &mut right);
        let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
        for frame in 0..FRAMES {
            assert_eq!(
                master_left[frame].to_bits(),
                0x8000_0000,
                "frame {frame}: the first contributor's sign was lost on the left"
            );
            assert_eq!(
                master_right[frame].to_bits(),
                0x8000_0000,
                "frame {frame}: the first contributor's sign was lost on the right"
            );
        }

        fn matrix_word(left: f32, right: f32, coefficients: [f32; 4]) -> (f32, f32) {
            let left_product = coefficients[0] * left;
            let left_cross = coefficients[1] * right;
            let right_cross = coefficients[2] * left;
            let right_product = coefficients[3] * right;
            (left_cross + left_product, right_product + right_cross)
        }

        let seed_values = [
            16_777_216.0,
            1.0,
            -16_777_216.0,
            -1.0,
            f32::MIN_POSITIVE,
            f32::from_bits(1),
            3.25,
            -7.5,
            0.0,
        ];
        let mut seed_left = seed_values.to_vec();
        let mut seed_right: Vec<f32> = seed_values.iter().rev().map(|value| -*value).collect();
        let seeded: Vec<(f32, f32)> = seed_left
            .iter()
            .zip(&seed_right)
            .map(|(left, right)| matrix_word(*left, *right, fold[0].coefficients))
            .collect();
        let mut seed_members = ArenaMembers {
            lease: &mut lease,
            inputs: &[],
            outputs: &[],
            fold: &fold,
            master: MasterPlanes::Arena(ARENA_BASE),
            sources: SourceGather::NONE,
        };
        seed_members.fold_plane(0, &mut seed_left, &mut seed_right);

        const ACCUMULATE_COEFFICIENTS: [f32; 4] = [0.9, -0.1, 0.2, 0.8];
        let accumulate_fold = [FoldLane {
            coefficients: ACCUMULATE_COEFFICIENTS,
            store: false,
        }];
        let mut added_left: Vec<f32> = (0..FRAMES)
            .map(|frame| seed_values[(frame + 2) % FRAMES])
            .collect();
        let mut added_right: Vec<f32> = (0..FRAMES)
            .map(|frame| -seed_values[(frame + 5) % FRAMES])
            .collect();
        let routed: Vec<(f32, f32)> = added_left
            .iter()
            .zip(&added_right)
            .map(|(left, right)| matrix_word(*left, *right, ACCUMULATE_COEFFICIENTS))
            .collect();
        let expected_left: Vec<u32> = seeded
            .iter()
            .zip(&routed)
            .map(|(old, added)| (old.0 + added.0).to_bits())
            .collect();
        let expected_right: Vec<u32> = seeded
            .iter()
            .zip(&routed)
            .map(|(old, added)| (old.1 + added.1).to_bits())
            .collect();
        let mut accumulate_members = ArenaMembers {
            lease: &mut lease,
            inputs: &[],
            outputs: &[],
            fold: &accumulate_fold,
            master: MasterPlanes::Arena(ARENA_BASE),
            sources: SourceGather::NONE,
        };
        accumulate_members.fold_plane(0, &mut added_left, &mut added_right);
        let (master_left, master_right) = lease.read_stereo(ARENA_BASE);
        assert_eq!(
            master_left
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected_left,
            "real fold_plane store=false must ordered-add the left contribution"
        );
        assert_eq!(
            master_right
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected_right,
            "real fold_plane store=false must ordered-add the right contribution"
        );

        let mut cohort_lease = stereo_lease(FRAMES, 1);
        let (poison_left, poison_right) = cohort_lease.write_stereo(ARENA_BASE);
        poison_left.fill(17.0);
        poison_right.fill(-19.0);
        let mut cohort_members = ArenaMembers {
            lease: &mut cohort_lease,
            inputs: &[],
            outputs: &[],
            fold: &fold,
            master: MasterPlanes::Arena(ARENA_BASE),
            sources: SourceGather::NONE,
        };
        let mut cohort_left = vec![-0.0_f32; FRAMES];
        let mut cohort_right = vec![-0.0_f32; FRAMES];
        cohort_members.fold_cohort(
            FoldCohort::new(&[0], &mut cohort_left, &mut cohort_right, FRAMES, FRAMES)
                .expect("valid signed-zero cohort"),
        );
        assert!(
            cohort_left
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
        assert!(
            cohort_right
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
        let (master_left, master_right) = cohort_lease.read_stereo(ARENA_BASE);
        assert!(
            master_left
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
        assert!(
            master_right
                .iter()
                .all(|value| value.to_bits() == 0x8000_0000)
        );
    }

    /// The fan-in-zero fill is skipped under a bound source and kept everywhere else.
    ///
    /// Both directions, because only one of them is the optimisation:
    ///
    /// * A **bound** node with no graph inputs is written entirely by its host processor, so the
    ///   `fill(0.0)` in front of it is two stereo blocks of dead stores. The buffer is left holding
    ///   the previous block's words on the way in, which is what the trait contract now says.
    /// * An **identity** node with no graph inputs is a submix nothing routes into, and the fill
    ///   *is* its audio. Dropping the `NodeKind::Bound` guard reddens this arm at once.
    #[test]
    fn the_fan_in_zero_fill_is_dead_only_under_a_bound_source() {
        /// A host source that writes every word it is handed, as the contract requires.
        struct Fill(f32);
        impl crate::GraphRuntimeProcessor for Fill {
            fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                block.left.fill(self.0);
                block.right.fill(-self.0);
                Ok(())
            }
        }
        const FRAMES: usize = 16;
        const STALE: f32 = 1.5;

        for (case, kind, expected) in [
            (
                "a bound source overwrites the block, so the fill is dead",
                NodeKind::Bound(Box::new(Fill(0.25))),
                (0.25_f32, -0.25_f32),
            ),
            (
                "an identity node with no contributors renders silence, and the fill is that",
                NodeKind::Identity,
                (0.0, 0.0),
            ),
        ] {
            let mut lease = stereo_lease(FRAMES, 1);
            lease.write(0, ARENA_BASE).fill(STALE);
            lease.write(1, ARENA_BASE).fill(STALE);
            let mut op = RuntimeOp {
                inputs: Box::new([]),
                staged: Box::new([]),
                sidechain: None,
                output: ARENA_BASE,
                kind,
                split_pair: None,
                observers: Box::new([]),
            };
            execute_op(
                &mut op,
                &mut lease,
                &mut [],
                &mut [],
                &mut [],
                0,
                None,
                &[],
                OutputSources::NONE,
            )
            .expect("op");
            let (left, right) = lease.read_stereo(ARENA_BASE);
            assert!(
                left.iter().all(|value| *value == expected.0)
                    && right.iter().all(|value| *value == expected.1),
                "{case}"
            );
        }
    }

    /// The dedication copy is skipped for exactly the shape that is only a copy, and for nothing
    /// else.
    ///
    /// Red mutations (`tests/MUTATIONS.md`): drop the `staged` clause -- the delayed case admits
    /// and the delay line stops being pumped; drop the `sidechain` clause -- the sidechained case
    /// admits; accept two inputs -- the summed case admits and a summand is lost; accept
    /// `inputs[0] == output` -- the already-in-place case admits, which is a no-op redirect that
    /// hides the clause; widen past `BankMember` -- the `Effect` case admits and the effect's own
    /// processing is skipped along with the copy.
    #[test]
    fn bank_gather_source_admits_only_the_dedication_copy() {
        fn member(
            inputs: Vec<u32>,
            staged: Vec<StagedInput>,
            sidechain: Option<u32>,
            output: u32,
            kind: NodeKind,
        ) -> RuntimeOp {
            RuntimeOp {
                inputs: inputs.into_boxed_slice(),
                staged: staged.into_boxed_slice(),
                sidechain,
                output,
                kind,
                split_pair: None,
                observers: Box::new([]),
            }
        }

        // The shape the SIMD racks actually produce: a dedicated member whose whole body is one
        // copy from its producer.
        assert_eq!(
            bank_gather_source(&member(vec![7], Vec::new(), None, 9, NodeKind::BankMember)),
            Some(7),
            "a single undelayed input into a distinct dedicated buffer is the copy"
        );

        // Every refusal, one clause each.
        for (case, op) in [
            (
                "no input at all is a fill(0.0), not a copy",
                member(Vec::new(), Vec::new(), None, 9, NodeKind::BankMember),
            ),
            (
                "two inputs are a sum; skipping it would drop a summand",
                member(vec![7, 8], Vec::new(), None, 9, NodeKind::BankMember),
            ),
            (
                "a staged input owns a delay line that must still be pumped",
                member(
                    vec![7],
                    vec![StagedInput {
                        source: 6,
                        staging: 7,
                        line: 0,
                    }],
                    None,
                    9,
                    NodeKind::BankMember,
                ),
            ),
            (
                "a sidechain is a second read the gather has no port for",
                member(vec![7], Vec::new(), Some(8), 9, NodeKind::BankMember),
            ),
            (
                "already in place: there is no copy left to skip",
                member(vec![9], Vec::new(), None, 9, NodeKind::BankMember),
            ),
            (
                "a kind that does work of its own must run",
                member(vec![7], Vec::new(), None, 9, NodeKind::Identity),
            ),
            (
                "a route does work of its own too",
                member(
                    vec![7],
                    Vec::new(),
                    None,
                    9,
                    NodeKind::Route([1.0, 0.0, 0.0, 1.0]),
                ),
            ),
        ] {
            assert_eq!(bank_gather_source(&op), None, "{case}");
        }
    }

    /// E5. Master plan #83 D9: the block reduction is bit-for-bit the scalar left-to-right
    /// reference `inputs.reduce(|a, b| a + b)`, at every fan-in and for every frame.
    ///
    /// Red mutations (`tests/MUTATIONS.md`): restore the balanced pairwise tree -- (a) fails;
    /// use `fold(0.0, +)` in the reference -- (c) fails; reverse the input order -- (b) fails.
    #[test]
    fn reduction_is_left_to_right_bit_identical_to_scalar_reference() {
        // (a) The classic discriminator: 1 + 2^-24 + 2^-24 + 0 is 1.0 left to right, but
        // 1 + 2^-23 = 1 + 2^-23 as a balanced tree.
        let frames = 128;
        let epsilon = f32::from_bits(0x3380_0000); // 2^-24
        let inputs = vec![
            vec![1.0f32; frames],
            vec![epsilon; frames],
            vec![epsilon; frames],
            vec![0.0f32; frames],
        ];
        for sample in reduce_case(frames, &inputs) {
            assert_eq!(
                sample.to_bits(),
                1.0f32.to_bits(),
                "left-to-right must round away the second epsilon"
            );
        }

        let order = [16_777_216.0_f32, 1.0, -16_777_216.0];
        let old = (order[0] + order[1]) + order[2];
        let wrong = order[0] + (order[1] + order[2]);
        assert_ne!(old.to_bits(), wrong.to_bits());
        let got = reduce_case(1, &order.map(|value| vec![value]));
        let old_kernel = old_reduce_case(1, &order.map(|value| vec![value]));
        assert_eq!(old_kernel[0].to_bits(), old.to_bits());
        assert_eq!(got[0].to_bits(), old_kernel[0].to_bits());

        let many = [
            16_777_216.0_f32,
            1.0,
            -16_777_216.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            16_777_216.0,
            1.0,
            -16_777_216.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ];
        let old = many.into_iter().reduce(|a, b| a + b).expect("inputs");
        let first = many[..8]
            .iter()
            .copied()
            .reduce(|a, b| a + b)
            .expect("group");
        let second = many[8..]
            .iter()
            .copied()
            .reduce(|a, b| a + b)
            .expect("group");
        let wrong_subtotal = first + second;
        assert_ne!(old.to_bits(), wrong_subtotal.to_bits());
        let inputs = many.map(|value| vec![value]);
        let old_kernel = old_reduce_case(1, &inputs);
        assert_eq!(old_kernel[0].to_bits(), old.to_bits());
        assert_eq!(
            reduce_case(1, &inputs)[0].to_bits(),
            old_kernel[0].to_bits()
        );

        // (b) Seeded corpora at several fan-ins, against the one-line scalar reference.
        let mut state = 0x6d69_736fu32;
        for count in [1usize, 2, 3, 5, 8, 9, 64, 65, 129] {
            for frames in [1usize, 7, 64, 128, 512] {
                let inputs: Vec<Vec<f32>> = (0..count)
                    .map(|_| (0..frames).map(|_| lcg(&mut state)).collect())
                    .collect();
                let reduced = reduce_case(frames, &inputs);
                for frame in 0..frames {
                    let reference = inputs
                        .iter()
                        .map(|input| input[frame])
                        .reduce(|a, b| a + b)
                        .unwrap_or(0.0);
                    assert_eq!(
                        reduced[frame].to_bits(),
                        reference.to_bits(),
                        "fan-in {count}, {frames} frames, frame {frame}"
                    );
                }
            }
        }

        // (c) Signed zero. `reduce` preserves it; `fold(0.0, +)` would not.
        assert_eq!(
            reduce_case(1, &[vec![-0.0f32]])[0].to_bits(),
            (-0.0f32).to_bits()
        );
        assert_eq!(
            reduce_case(1, &[vec![-0.0f32], vec![-0.0f32]])[0].to_bits(),
            (-0.0f32).to_bits()
        );
        assert_eq!(
            reduce_case(1, &[vec![-0.0f32], vec![0.0f32]])[0].to_bits(),
            0.0f32.to_bits()
        );
        // Fan-in zero is a zero fill, not a copy.
        assert_eq!(reduce_case(4, &[]), vec![0.0f32; 4]);
    }

    /// E5 continued: an in-place single input is the identity, not a copy through a scratch.
    #[test]
    fn a_single_in_place_input_is_left_untouched() {
        let mut lease = single_lease(2, 2);
        lease.write(0, 1).copy_from_slice(&[1.0, 2.0]);
        lease.write(0, 2).copy_from_slice(&[3.0, 4.0]);
        reduce_plane(&mut lease, 0, 2, &[2]);
        assert_eq!(lease.read(0, 1), &[1.0, 2.0]);
        assert_eq!(lease.read(0, 2), &[3.0, 4.0]);
    }

    /// E7. The two-segment slice PDC is bit-for-bit a per-sample ring delay, and the result does
    /// not depend on how the stream is partitioned into blocks (master plan #83 D5).
    ///
    /// Red mutations (`tests/MUTATIONS.md`): drop the cursor advance between segments; swap the
    /// two `pdc_delay_block` segments; carry one cursor across both channels instead of restarting
    /// the right channel at the block's cursor.
    #[test]
    fn compensation_delay_is_partition_invariant_and_matches_per_sample_reference() {
        const SAMPLES: usize = 4_096;
        let mut state = 0x0a05_1970u32;
        let signal: Vec<f32> = (0..SAMPLES).map(|_| lcg(&mut state)).collect();
        for delay_samples in [1usize, 3, 37, 127, 128, 129, 600] {
            // The independent oracle: a per-sample `VecDeque` ring, three lines, no slice algebra.
            let per_sample = |input: &[f32]| -> Vec<f32> {
                let mut ring: std::collections::VecDeque<f32> =
                    std::iter::repeat_n(0.0, delay_samples).collect();
                input
                    .iter()
                    .map(|sample| {
                        ring.push_back(*sample);
                        ring.pop_front().expect("non-empty ring")
                    })
                    .collect()
            };
            let negated: Vec<f32> = signal.iter().map(|sample| -sample).collect();
            let reference = per_sample(&signal);
            let reference_right = per_sample(&negated);
            let mut baseline: Option<Vec<u32>> = None;
            for block in [1usize, 7, 64, 128, 512] {
                let mut delay = CompensationDelay::new(delay_samples);
                assert_eq!(delay.samples(), delay_samples);
                let mut left = signal.clone();
                let mut right = negated.clone();
                for offset in (0..SAMPLES).step_by(block) {
                    let end = (offset + block).min(SAMPLES);
                    let (head, tail) = left.split_at_mut(end);
                    let _ = tail;
                    let (right_head, right_tail) = right.split_at_mut(end);
                    let _ = right_tail;
                    delay.process(&mut head[offset..], &mut right_head[offset..]);
                }
                for (index, sample) in left.iter().enumerate() {
                    assert_eq!(
                        sample.to_bits(),
                        reference[index].to_bits(),
                        "delay {delay_samples}, block {block}, sample {index}"
                    );
                    assert_eq!(right[index].to_bits(), reference_right[index].to_bits());
                }
                let bits: Vec<u32> = left.iter().map(|sample| sample.to_bits()).collect();
                match &baseline {
                    None => baseline = Some(bits),
                    Some(first) => assert_eq!(&bits, first, "partition invariance at {block}"),
                }
                delay.reset();
                let mut zero = vec![0.0f32; delay_samples];
                let mut zero_right = vec![0.0f32; delay_samples];
                delay.process(&mut zero, &mut zero_right);
                assert!(
                    zero.iter().all(|sample| *sample == 0.0),
                    "reset clears the line"
                );
            }
        }
    }

    /// E10. The route is two multiplies plus one add per output word, with the gain
    /// folded into the coefficients at bind (D3). The oracle is
    /// `softfma::unfused_multiply_add_via_f64`, which computes the same multiply-add through `f64` with
    /// no dependence on `mix2x2_block`'s vector body; the exact product and the innocuous double
    /// rounding of the sum make it bit-identical to the `f32` expression (issue #163 phase 2).
    ///
    /// Red mutation (`tests/MUTATIONS.md`): compute `gain * (ll * l + lr * r)` instead.
    #[test]
    fn route_applies_folded_gain_with_frozen_op_order() {
        const FRAMES: usize = 4_096;
        let transform = (0.707_9f32, 0.9f32, -0.1f32, 0.2f32, 0.8f32);
        let folded = [
            transform.0 * transform.1,
            transform.0 * transform.2,
            transform.0 * transform.3,
            transform.0 * transform.4,
        ];
        let mut state = 0x1234_5678u32;
        let left_in: Vec<f32> = (0..FRAMES).map(|_| lcg(&mut state)).collect();
        let right_in: Vec<f32> = (0..FRAMES).map(|_| lcg(&mut state)).collect();
        let mut left = left_in.clone();
        let mut right = right_in.clone();
        mix2x2_block::<FrameLane>(&mut left, &mut right, folded);
        for frame in 0..FRAMES {
            let (l, r) = (left_in[frame], right_in[frame]);
            let expected_left =
                lane::softfma::unfused_multiply_add_via_f64(folded[1], r, folded[0] * l);
            let expected_right =
                lane::softfma::unfused_multiply_add_via_f64(folded[3], r, folded[2] * l);
            assert_eq!(
                left[frame].to_bits(),
                expected_left.to_bits(),
                "frame {frame} left"
            );
            assert_eq!(
                right[frame].to_bits(),
                expected_right.to_bits(),
                "frame {frame} right"
            );
        }
        // Block lengths that are not a multiple of the lane width, so the scalar tail is walked
        // and has to agree with the vector body word for word.
        for block in [1usize, 3, 7, 63, 65, 129, 511] {
            let mut tail_left = left_in[..block].to_vec();
            let mut tail_right = right_in[..block].to_vec();
            mix2x2_block::<FrameLane>(&mut tail_left, &mut tail_right, folded);
            assert_eq!(
                (
                    tail_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
                    tail_right.iter().map(|s| s.to_bits()).collect::<Vec<_>>()
                ),
                (
                    left[..block]
                        .iter()
                        .map(|s| s.to_bits())
                        .collect::<Vec<_>>(),
                    right[..block]
                        .iter()
                        .map(|s| s.to_bits())
                        .collect::<Vec<_>>()
                ),
                "block {block}: the scalar tail must match the vector body"
            );
        }
        // The fold is not the unfolded form: this is the bit change #98 F4 declares.
        let unfolded: Vec<u32> = (0..FRAMES)
            .map(|frame| {
                (transform.0 * (transform.1 * left_in[frame] + transform.2 * right_in[frame]))
                    .to_bits()
            })
            .collect();
        assert_ne!(
            left.iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            unfolded,
            "the folded route is a distinct rounding, not a coincidence"
        );
    }

    // -----------------------------------------------------------------------------------------
    // Issue #885: the route fold stays armed when every track carries a post-matrix meter.
    // -----------------------------------------------------------------------------------------

    /// One published meter frame: the exact words the observer was handed, and the peak and
    /// energy a meter derives from them in one fixed order. The words are the strong half -- equal
    /// words through equal arithmetic are equal snapshots -- and the two numbers keep the claim
    /// about meters rather than about copies. The input path (resident or planar) is deliberately
    /// not part of a frame: the arms below differ in exactly that and must not differ in anything a
    /// meter publishes.
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MeterFrame {
        handle: u64,
        first_sample: u64,
        left: Vec<u32>,
        right: Vec<u32>,
        peak: [u32; 2],
        energy: [u32; 2],
    }

    type Published = Arc<std::sync::Mutex<Vec<MeterFrame>>>;

    /// A meter that reads the resident view when `accepts_resident`, and declines it otherwise --
    /// which is the host observer the materialization fallback exists for.
    struct WordMeter {
        handle: u64,
        accepts_resident: bool,
        published: Published,
    }

    impl WordMeter {
        fn publish(&self, first_sample: u64, left: &[f32], right: &[f32]) {
            let measure = |plane: &[f32]| {
                let mut peak = 0.0_f32;
                let mut energy = 0.0_f32;
                for sample in plane {
                    peak = peak.max(sample.abs());
                    energy += sample * sample;
                }
                (peak.to_bits(), energy.to_bits())
            };
            let (left_peak, left_energy) = measure(left);
            let (right_peak, right_energy) = measure(right);
            let bits = |plane: &[f32]| plane.iter().map(|sample| sample.to_bits()).collect();
            self.published.lock().unwrap().push(MeterFrame {
                handle: self.handle,
                first_sample,
                left: bits(left),
                right: bits(right),
                peak: [left_peak, right_peak],
                energy: [left_energy, right_energy],
            });
        }
    }

    impl crate::GraphRuntimeObserver for WordMeter {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            self.publish(block.first_sample, block.left, block.right);
            Ok(())
        }

        fn observe_resident(
            &mut self,
            block: crate::GraphResidentObservationBlock<'_>,
        ) -> Option<Result<(), RenderError>> {
            if !self.accepts_resident {
                return None;
            }
            // The documented strided layout, written out rather than shared with the production
            // copy: frame `f` of lane `l` is word `f * width + l`.
            let lane = block.lane;
            let width = lane.width().lanes() as usize;
            let pick = |plane: &[f32]| -> Vec<f32> {
                (0..lane.frames() as usize)
                    .map(|frame| plane[frame * width + lane.lane()])
                    .collect()
            };
            self.publish(block.first_sample, &pick(lane.left()), &pick(lane.right()));
            Some(Ok(()))
        }
    }

    /// Seeded noise per track and per block, so every lane and every block carries new words.
    struct NoiseInput(u32);

    impl GraphRuntimeProcessor for NoiseInput {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            let mut state = (self.0 + 1).wrapping_mul(0x9e37_79b9)
                ^ (block.first_sample as u32).wrapping_mul(0x85eb_ca6b);
            for sample in block.left.iter_mut().chain(block.right.iter_mut()) {
                *sample = lcg(&mut state);
            }
            Ok(())
        }
    }

    /// A builtin bank that is not an identity: the resident words are the chain's own output, and
    /// its cross term makes the two planes depend on each other.
    struct CrossTilt;

    impl GraphPreparedBuiltinBankProcessor for CrossTilt {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn process(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            _: u32,
            _: u64,
        ) -> Result<(), RenderError> {
            for (left, right) in left.iter_mut().zip(right.iter_mut()) {
                let (a, b) = (*left, *right);
                *left = a * 0.75 - b * 0.125;
                *right = b * 1.25 + a * 0.5;
            }
            Ok(())
        }
    }

    /// [`CrossTilt`] until `from_sample`, then a failed block: issue #916's mid-schedule failure.
    struct FailingTilt {
        from_sample: u64,
    }

    impl GraphPreparedBuiltinBankProcessor for FailingTilt {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn process(
            &mut self,
            left: &mut [f32],
            right: &mut [f32],
            frames: u32,
            first_sample: u64,
        ) -> Result<(), RenderError> {
            if first_sample >= self.from_sample {
                return Err(RenderError::InvalidEnvelope);
            }
            CrossTilt.process(left, right, frames, first_sample)
        }
    }

    #[derive(Clone, Copy)]
    struct FoldFixture {
        width: BankWidth,
        tracks: usize,
        frames: u32,
        /// The stage every bank member -- each chain's last slot -- sits at.
        stage: TrackStage,
        /// Meters on every track, at the stages `meter_at` names.
        metered: bool,
        accepts_resident: bool,
        /// Bind through the controlled-activation catalog and activate every meter.
        controlled: bool,
        /// Bind with the route fold declined: the path this plan took before issue #885.
        fold_declined: bool,
        /// Issue #886: an elided `PostSimd1` boundary after each member -- a later tap on the
        /// chain's last slot, which reads that slot's buffer.
        alias: bool,
        /// Issue #886: a bound scalar `PostFader` op between the member (or its alias) and the
        /// route, so no fold is possible and the member's consumer is a plain op.
        fader: bool,
        /// The stages each track is metered at, one meter per stage; empty is `stage` alone.
        meter_at: &'static [TrackStage],
        /// Bind with every scatter redirect declined: the path a metered last slot took before
        /// issue #886.
        scatter_declined: bool,
        /// Issue #886: a bound scalar `PostMatrix` after each fader, and a fader owner that offers
        /// the scalar split fader/matrix pair. Needs `fader`.
        split_pair: bool,
        /// Issue #916: two meters on the session Output node, bound after every track meter.
        output_meters: bool,
        /// Issue #916: this cohort's bank processor fails from the second block on.
        failing_cohort: Option<usize>,
    }

    impl FoldFixture {
        const fn metered(width: BankWidth, tracks: usize, frames: u32) -> Self {
            Self {
                width,
                tracks,
                frames,
                stage: TrackStage::PostMatrix,
                metered: true,
                accepts_resident: true,
                controlled: false,
                fold_declined: false,
                alias: false,
                fader: false,
                meter_at: &[],
                scatter_declined: false,
                split_pair: false,
                output_meters: false,
                failing_cohort: None,
            }
        }

        /// Issue #886's shape: `Input -> PostInputBuiltins (bank) -> PostSimd1 (elided) ->
        /// PostFader (bound) -> Route -> Output`, with a meter on the last slot's own node and one
        /// on its later tap. `PostInputBuiltins` is dedicated storage (`program::is_dedicated`), so
        /// the fader cannot run in place on it and the chain's scatter is redirected into the
        /// fader's buffer.
        const fn scattered(width: BankWidth, tracks: usize, frames: u32) -> Self {
            Self {
                stage: TrackStage::PostInputBuiltins,
                alias: true,
                fader: true,
                meter_at: &[TrackStage::PostInputBuiltins, TrackStage::PostSimd1],
                ..Self::metered(width, tracks, frames)
            }
        }
    }

    /// [`ScalarTilt`]'s arithmetic, from an owner that also offers the scalar split fader/matrix
    /// pair (issue #886's split-pair arm).
    struct SplitFader;

    impl GraphRuntimeProcessor for SplitFader {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            ScalarTilt.process(block)
        }

        fn scalar_split_pair_factory(&self) -> Option<crate::ScalarSplitPairFactory> {
            Some(|fader, matrix| {
                Ok(Box::new(DeferredSplit {
                    fader,
                    matrix,
                    pending: false,
                }))
            })
        }
    }

    /// A bound scalar matrix that is not an identity and mixes its planes.
    struct MatrixTilt;

    impl GraphRuntimeProcessor for MatrixTilt {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
                let (a, b) = (*left, *right);
                *left = a * 1.125 - b * 0.25;
                *right = b * 0.75 + a * 0.3125;
            }
            Ok(())
        }
    }

    /// A split pair that defers the fader's in-place arithmetic to the matrix slot, as the
    /// production owner may: the fader's buffer holds the fader's *input* in between, which the
    /// split pair's admission proves nothing reads.
    struct DeferredSplit {
        fader: Box<dyn GraphRuntimeProcessor>,
        matrix: Box<dyn GraphRuntimeProcessor>,
        pending: bool,
    }

    impl GraphRuntimeSplitPairProcessor for DeferredSplit {
        fn begin_fader(&mut self, _: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            self.pending = true;
            Ok(())
        }

        fn finish_matrix(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            if core::mem::take(&mut self.pending) {
                self.fader.process(GraphBindingBlock {
                    left: &mut *block.left,
                    right: &mut *block.right,
                    first_sample: block.first_sample,
                })?;
            }
            self.matrix.process(block)
        }

        fn complete_pending(&mut self, block: GraphBindingBlock<'_>) {
            if core::mem::take(&mut self.pending) {
                self.fader
                    .process(block)
                    .expect("the test fader cannot fail");
            }
        }
    }

    /// `Input -> <stage> (one builtin bank per cohort of width) -> Route -> Output`, per track,
    /// with issue #886's optional elided boundary, bound fader and bound matrix between the member
    /// and the route.
    ///
    /// Every route has its own non-trivial 2x2, so the master's association order is visible in
    /// its bits. Cohorts are the tracks in order, so the chains' render order is the reduction's
    /// edge order and the fold is admissible.
    fn fold_fixture(
        fixture: FoldFixture,
        published: &Published,
    ) -> (
        engine::realtime::PreparedRenderPlan,
        Option<crate::GraphObservationController>,
    ) {
        let (plan, bindings, observed) = fold_fixture_parts(fixture, published);
        test_only_set_route_fold_declined(fixture.fold_declined);
        test_only_set_scatter_redirect_declined(fixture.scatter_declined);
        let bound = if fixture.controlled {
            let (plan, mut controller) = plan
                .bind_with_observation_activation(
                    bindings,
                    crate::GraphObservationActivationConfig {
                        maximum_active_observers: observed.len(),
                        maximum_retained_bytes: u64::MAX,
                    },
                )
                .unwrap_or_else(|failure| panic!("controlled bind: {}", failure.code));
            let handles: Vec<u64> = observed.iter().map(|(_, handle)| *handle).collect();
            controller.replace(&handles).expect("activate every meter");
            (plan, Some(controller))
        } else {
            (
                plan.bind(bindings)
                    .unwrap_or_else(|failure| panic!("bind: {}", failure.code)),
                None,
            )
        };
        test_only_set_route_fold_declined(false);
        test_only_set_scatter_redirect_declined(false);
        bound
    }

    /// [`fold_fixture`]'s unbound plan, its bindings, and every observed `(node, handle)` in
    /// binding order: the parts a test that binds the executor itself needs (issue #916).
    fn fold_fixture_parts(
        fixture: FoldFixture,
        published: &Published,
    ) -> (
        crate::PreparedGraphPlan,
        crate::GraphRuntimeBindings,
        Vec<(GraphNodeId, u64)>,
    ) {
        let id = |text: String| crate::StableGraphId::parse(&text).expect("stable id");
        let stage_node = |track: usize, stage| GraphNodeId::TrackStage {
            track_id: id(format!("track{track:02}")),
            stage,
        };
        let tracks = fixture.tracks;
        let inputs: Vec<_> = (0..tracks)
            .map(|track| stage_node(track, TrackStage::Input))
            .collect();
        let members: Vec<_> = (0..tracks)
            .map(|track| stage_node(track, fixture.stage))
            .collect();
        let aliases: Vec<_> = (0..tracks)
            .filter(|_| fixture.alias)
            .map(|track| stage_node(track, TrackStage::PostSimd1))
            .collect();
        let faders: Vec<_> = (0..tracks)
            .filter(|_| fixture.fader)
            .map(|track| stage_node(track, TrackStage::PostFader))
            .collect();
        let matrices: Vec<_> = (0..tracks)
            .filter(|_| fixture.split_pair)
            .map(|track| stage_node(track, TrackStage::PostMatrix))
            .collect();
        let routes: Vec<_> = (0..tracks)
            .map(|track| GraphNodeId::Route {
                route_id: id(format!("route{track:02}")),
            })
            .collect();
        let output = GraphNodeId::Output {
            output_id: id("main".to_owned()),
        };
        let port = |node: &GraphNodeId, kind| crate::GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        let edge = |id, source: &GraphNodeId, destination: &GraphNodeId| crate::GraphEdge {
            id,
            source: port(source, crate::GraphPortKind::MainOutput),
            destination: port(destination, crate::GraphPortKind::MainInput),
            path: "$.issue885".to_owned(),
        };
        let mut edges = Vec::new();
        for track in 0..tracks {
            let route_id = id(format!("route{track:02}"));
            edges.push(edge(
                GraphEdgeId::TrackMain {
                    target: members[track].clone(),
                },
                &inputs[track],
                &members[track],
            ));
            let mut upstream = &members[track];
            for boundary in [aliases.get(track), faders.get(track), matrices.get(track)]
                .into_iter()
                .flatten()
            {
                edges.push(edge(
                    GraphEdgeId::TrackMain {
                        target: boundary.clone(),
                    },
                    upstream,
                    boundary,
                ));
                upstream = boundary;
            }
            edges.push(edge(
                GraphEdgeId::RouteSource {
                    route_id: route_id.clone(),
                },
                upstream,
                &routes[track],
            ));
            edges.push(edge(
                GraphEdgeId::RouteDestination { route_id },
                &routes[track],
                &output,
            ));
        }
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let outputs = vec![output.clone()];
        let levels: Vec<&Vec<GraphNodeId>> = [
            &inputs, &members, &aliases, &faders, &matrices, &routes, &outputs,
        ]
        .into_iter()
        .filter(|level| !level.is_empty())
        .collect();
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.iter().cloned())
            .collect();
        let mut nodes: Vec<_> = schedule
            .iter()
            .cloned()
            .map(|id| crate::GraphNode {
                id,
                latency: effect_contract::LatencySamples(0),
                tail: effect_contract::TailSamples::Finite(0),
            })
            .collect();
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        let envelope = engine::realtime::RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: engine::QuantumFrames(fixture.frames),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let backend = match fixture.width {
            BankWidth::Four => lane::Backend::Simd4,
            BankWidth::Eight => lane::Backend::Simd8,
        };
        let builtin_banks = members
            .chunks(fixture.width.lanes() as usize)
            .enumerate()
            .map(|(index, cohort)| {
                let processor: Box<dyn GraphPreparedBuiltinBankProcessor> =
                    if fixture.failing_cohort == Some(index) {
                        Box::new(FailingTilt {
                            from_sample: u64::from(fixture.frames),
                        })
                    } else {
                        Box::new(CrossTilt)
                    };
                GraphPreparedBuiltinBank {
                    backend,
                    members: cohort.to_vec().into_boxed_slice(),
                    processor,
                    scratch: AoSoaScratch::new(fixture.width, fixture.frames).expect("scratch"),
                }
            })
            .collect();
        let mut required_bindings = inputs.clone();
        required_bindings.extend(members.iter().cloned());
        required_bindings.extend(faders.iter().cloned());
        required_bindings.extend(matrices.iter().cloned());
        required_bindings.push(output.clone());
        let plan = crate::PreparedGraphPlan::new(crate::PreparedGraphPlanParts {
            plan_id: 885,
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels
                .iter()
                .enumerate()
                .map(|(level, nodes)| crate::DependencyLevel {
                    level: level as u64,
                    nodes: (*nodes).clone(),
                })
                .collect(),
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: crate::GraphResourceEstimate {
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
            },
            envelope,
            required_bindings,
            routes: routes
                .iter()
                .enumerate()
                .map(|(track, node)| crate::PreparedRoute {
                    node: node.clone(),
                    transform: RouteTransform {
                        gain: 0.5 + 0.0625 * track as f32,
                        ll: 0.875,
                        lr: -0.25 + 0.03125 * track as f32,
                        rl: 0.3,
                        rr: 1.125 - 0.046875 * track as f32,
                    },
                })
                .collect(),
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks,
            observers: Vec::new(),
        });
        let mut nodes: Vec<GraphNodeBinding> = inputs
            .iter()
            .enumerate()
            .map(|(track, node)| {
                GraphNodeBinding::new(node.clone(), Box::new(NoiseInput(track as u32)))
            })
            .collect();
        nodes.extend(faders.iter().map(|node| {
            let fader: Box<dyn GraphRuntimeProcessor> = if fixture.split_pair {
                Box::new(SplitFader)
            } else {
                Box::new(ScalarTilt)
            };
            GraphNodeBinding::new(node.clone(), fader)
        }));
        nodes.extend(
            matrices
                .iter()
                .map(|node| GraphNodeBinding::new(node.clone(), Box::new(MatrixTilt))),
        );
        nodes.push(GraphNodeBinding::identity(output.clone()));
        let meter_at = if fixture.meter_at.is_empty() {
            &[fixture.stage][..]
        } else {
            fixture.meter_at
        };
        let mut observed: Vec<(GraphNodeId, u64)> = Vec::new();
        for track in (0..tracks).filter(|_| fixture.metered) {
            for (meter, stage) in meter_at.iter().enumerate() {
                let handle = (track * meter_at.len() + meter) as u64 + 1;
                observed.push((stage_node(track, *stage), handle));
            }
        }
        for meter in (0..2).filter(|_| fixture.output_meters) {
            observed.push((output.clone(), OUTPUT_METER_HANDLE + meter));
        }
        let observers = observed
            .iter()
            .map(|(node, handle)| {
                let meter = Box::new(WordMeter {
                    handle: *handle,
                    accepts_resident: fixture.accepts_resident,
                    published: Arc::clone(published),
                });
                if fixture.controlled {
                    GraphNodeObserverBinding::controlled(node.clone(), *handle, meter)
                } else {
                    GraphNodeObserverBinding::new(node.clone(), *handle, meter)
                }
            })
            .collect();
        let bindings = crate::GraphRuntimeBindings {
            envelope,
            nodes,
            observers,
        };
        (plan, bindings, observed)
    }

    /// The first handle of [`FoldFixture::output_meters`]' two session-output meters, clear of
    /// every track meter's handle.
    const OUTPUT_METER_HANDLE: u64 = 1_000_000;

    /// Render `blocks` blocks and return the master output's bits, block after block.
    fn render_fold_fixture(
        plan: &mut engine::realtime::PreparedRenderPlan,
        frames: usize,
        blocks: u64,
    ) -> Vec<u32> {
        let mut bits = Vec::new();
        for block in 0..blocks {
            let mut pcm = vec![f32::from_bits(0x7fc0_0885); frames * 2];
            plan.render(
                engine::realtime::RenderIo {
                    input: None,
                    output: engine::realtime::PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                        .expect("stereo output"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: block * frames as u64,
                },
            )
            .expect("render");
            bits.extend(pcm.iter().map(|sample| sample.to_bits()));
        }
        bits
    }

    /// Gate 1 of issue #885: a post-matrix meter on every track of a full bank no longer declines
    /// the fold -- every track's route folds, through both observer dispatchers.
    ///
    /// The controls pin which clause each decline belongs to. An unmetered strip at another stage
    /// folds, so the decline with a meter there is the observer's and not the stage's; and the
    /// test-only switch the next gate's control arm uses really does decline. (A route observer is
    /// not constructible: bind admits observers on track stages and the output only, so the
    /// route clause of `foldable_lane` is unchanged and unexercised here.)
    ///
    /// Red mutation: `served_by_the_resident_lane` answering `false` -- the metered arms report
    /// zero folds. Answering `true` for every node -- the post-fader arm folds.
    #[test]
    fn a_post_matrix_meter_on_every_track_of_a_full_bank_keeps_the_fold_armed() {
        for (width, tracks) in [(BankWidth::Four, 4), (BankWidth::Eight, 8)] {
            let published = Published::default();
            let folds = |fixture| fold_fixture(fixture, &published).0.bank_route_folds();
            let metered = FoldFixture::metered(width, tracks, 13);
            assert_eq!(
                folds(metered),
                tracks as u64,
                "{width:?}: a post-matrix meter on every track must keep every route folded"
            );
            assert_eq!(
                folds(FoldFixture {
                    controlled: true,
                    ..metered
                }),
                tracks as u64,
                "{width:?}: the controlled catalog binds the same fold"
            );
            assert_eq!(
                folds(FoldFixture {
                    accepts_resident: false,
                    ..metered
                }),
                tracks as u64,
                "{width:?}: a declining observer is served by the written member buffer"
            );
            assert_eq!(
                folds(FoldFixture {
                    stage: TrackStage::PostFader,
                    metered: false,
                    ..metered
                }),
                tracks as u64,
                "{width:?}: the unmetered post-fader strip folds"
            );
            assert_eq!(
                folds(FoldFixture {
                    stage: TrackStage::PostFader,
                    ..metered
                }),
                0,
                "{width:?}: a meter at any other tap keeps declining the fold"
            );
            assert_eq!(
                folds(FoldFixture {
                    fold_declined: true,
                    ..metered
                }),
                0,
                "{width:?}: the control arm's switch declines the fold"
            );
            assert!(published.lock().unwrap().is_empty(), "nothing rendered");
        }
    }

    /// Gate 2 of issue #885: the folded, metered plan renders the master and publishes every meter
    /// frame bit for bit as the same plan with the fold declined -- the path it took before.
    ///
    /// The control arm is that plan: meters bound, fold declined, and every meter declining the
    /// resident view, so each frame is the words the unfolded chain really scattered into the
    /// member buffer. (The same unfolded plan with resident-reading meters -- its production shape
    /// before #885 -- is checked equal to it first.) Every candidate folds every route and differs
    /// from the control in the observer path only:
    ///
    /// * meters reading the resident view (the production shape; no planar acquisition at all);
    /// * observers that decline the resident view, which are served by the member buffer
    ///   `write_resident_lane` fills -- the buffer the fold otherwise leaves holding last block's
    ///   words;
    /// * the test-only switch that withdraws the resident offer, the same fallback reached the
    ///   way the builtins compiler's own resident-meter gate reaches it;
    /// * both of the above through the controlled-activation dispatcher (`observe_one`).
    ///
    /// Shapes: a full W4 bank, a full W8 bank, two full W4 cohorts (so a continuation cohort
    /// accumulates rather than stores), and a W4 strip of six whose second cohort is partial and
    /// takes the per-lane scatter. Frame counts that are not a multiple of the lane width are
    /// included so the transposes' tails are exercised.
    ///
    /// Red mutations: drop the `write_resident_lane` call from either dispatcher -- the declining
    /// arms publish the member buffer's stale words; offer no resident view to a folded lane
    /// (restore the `fold.is_empty()` eligibility clause) -- the resident arms read stale words.
    #[test]
    fn a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit() {
        const BLOCKS: u64 = 6;
        for (width, tracks, frames) in [
            (BankWidth::Four, 4, 13),
            (BankWidth::Eight, 8, 13),
            (BankWidth::Four, 8, 16),
            (BankWidth::Four, 6, 5),
        ] {
            let metered = FoldFixture::metered(width, tracks, frames);
            let run = |fixture: FoldFixture, resident_disabled: bool| {
                let published = Published::default();
                let (mut plan, _controller) = fold_fixture(fixture, &published);
                let folds = plan.bank_route_folds();
                test_only_meter_input_reset(resident_disabled);
                let master = render_fold_fixture(&mut plan, frames as usize, BLOCKS);
                let counts = test_only_meter_input_counts();
                test_only_meter_input_reset(false);
                let frames = published.lock().unwrap().clone();
                (folds, master, frames, counts)
            };
            // The oracle is anchored to the buffer the unfolded chain really scatters: its observers
            // decline the resident view, so every frame is the member buffer's own words.
            let (control_folds, control_master, control_frames, control_counts) = run(
                FoldFixture {
                    fold_declined: true,
                    accepts_resident: false,
                    ..metered
                },
                false,
            );
            assert_eq!(control_folds, 0, "the control arm is the unfolded plan");
            let observations = (tracks as u64) * BLOCKS;
            assert_eq!(control_counts, [observations, observations, 0]);
            assert_eq!(
                control_frames.len(),
                tracks * BLOCKS as usize,
                "one frame per track per block"
            );
            assert!(
                control_frames.iter().all(|frame| frame
                    .left
                    .iter()
                    .chain(&frame.right)
                    .any(|word| *word != 0)),
                "every meter frame carries audio"
            );
            assert!(control_master.iter().any(|word| *word != 0));
            // The same unfolded plan with meters that read the resident view -- the production
            // shape before #885 -- publishes the same frames: the view is the scattered words.
            let (_, resident_master, resident_frames, resident_counts) = run(
                FoldFixture {
                    fold_declined: true,
                    ..metered
                },
                false,
            );
            assert_eq!(resident_counts, [0, observations, observations]);
            assert_eq!(resident_master, control_master);
            assert_eq!(resident_frames, control_frames);
            for (name, fixture, resident_disabled, expected_counts) in [
                (
                    "resident meters",
                    metered,
                    false,
                    [0, observations, observations],
                ),
                (
                    "declining observers",
                    FoldFixture {
                        accepts_resident: false,
                        ..metered
                    },
                    false,
                    [observations, observations, 0],
                ),
                (
                    "resident offer withdrawn",
                    metered,
                    true,
                    [observations, 0, 0],
                ),
                (
                    "controlled resident meters",
                    FoldFixture {
                        controlled: true,
                        ..metered
                    },
                    false,
                    [0, observations, observations],
                ),
                (
                    "controlled declining observers",
                    FoldFixture {
                        controlled: true,
                        accepts_resident: false,
                        ..metered
                    },
                    false,
                    [observations, observations, 0],
                ),
            ] {
                let (folds, master, frames, counts) = run(fixture, resident_disabled);
                assert_eq!(folds, tracks as u64, "{width:?}/{tracks}/{name}: folded");
                assert_eq!(
                    counts, expected_counts,
                    "{width:?}/{tracks}/{name}: [planar, offered, accepted]"
                );
                assert_eq!(
                    master, control_master,
                    "{width:?}/{tracks}/{name}: the folded master is the unfolded master's bits"
                );
                assert_eq!(
                    frames, control_frames,
                    "{width:?}/{tracks}/{name}: every meter frame is the unfolded plan's"
                );
            }
        }
    }

    // -----------------------------------------------------------------------------------------
    // A scatter redirect in a chain rendered after a folded one (found under issue #886).
    // -----------------------------------------------------------------------------------------

    /// A bound scalar fader that is not an identity and mixes its planes: its buffer changes the
    /// moment it runs, so anything that read that buffer after the fader ran would see other words.
    struct ScalarTilt;

    impl GraphRuntimeProcessor for ScalarTilt {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
                let (a, b) = (*left, *right);
                *left = a * 0.625 + b * 0.0625;
                *right = b * 0.875 - a * 0.1875;
            }
            Ok(())
        }
    }

    /// Four tracks `Input -> PostMatrix (one W4 builtin bank) -> Route -> bus`, then the bus
    /// `Input (the four routes' reduction) -> PostInputBuiltins (a one-lane builtin bank) ->
    /// PostFader (bound) -> Route -> Output`.
    ///
    /// The tracks' routes fold into the bus input, which retires four route runs; the bus's bank
    /// renders after them, and its dedicated last slot redirects its scatter into the fader. Three
    /// dangling pad strips (`Input -> PostFader`, bound, read by nothing) take the three lowest
    /// arena slots and free them before the bus allocates, so the bus -- and the session output in
    /// place on it -- lands on slots no track uses: `foldable_lane` declines a lane that shares the
    /// session output's slot, and without the pads the colouring hands the output a track's.
    fn folded_bus_plan(fold_declined: bool) -> engine::realtime::PreparedRenderPlan {
        let (plan, bindings) = folded_bus_parts(3, None);
        test_only_set_route_fold_declined(fold_declined);
        let bound = plan
            .bind(bindings)
            .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
        test_only_set_route_fold_declined(false);
        bound
    }

    /// [`folded_bus_plan`]'s unbound plan and bindings with `pads` pad strips (it binds three),
    /// and two meters on the session Output node (handles [`OUTPUT_METER_HANDLE`] and one after)
    /// when `output_meters` is given (issue #916). The Output has one input, the bus route, so
    /// this is the fan-in-one shape.
    fn folded_bus_parts(
        pads: usize,
        output_meters: Option<&Published>,
    ) -> (crate::PreparedGraphPlan, crate::GraphRuntimeBindings) {
        const FRAMES: u32 = 13;
        let id = |text: &str| crate::StableGraphId::parse(text).expect("stable id");
        let stage = |track: &str, stage| GraphNodeId::TrackStage {
            track_id: id(track),
            stage,
        };
        let tracks: Vec<String> = (0..4).map(|track| format!("track{track:02}")).collect();
        let inputs: Vec<_> = tracks
            .iter()
            .map(|track| stage(track, TrackStage::Input))
            .collect();
        let members: Vec<_> = tracks
            .iter()
            .map(|track| stage(track, TrackStage::PostMatrix))
            .collect();
        let route_ids: Vec<_> = (0..4)
            .map(|track| id(&format!("route{track:02}")))
            .collect();
        let routes: Vec<_> = route_ids
            .iter()
            .map(|route_id| GraphNodeId::Route {
                route_id: route_id.clone(),
            })
            .collect();
        let pad_ends: Vec<_> = (0..pads)
            .map(|pad| stage(&format!("pad{pad}"), TrackStage::PostFader))
            .collect();
        let pads: Vec<_> = (0..pads)
            .map(|pad| stage(&format!("pad{pad}"), TrackStage::Input))
            .collect();
        let bus_input = stage("bus", TrackStage::Input);
        let bus_member = stage("bus", TrackStage::PostInputBuiltins);
        let bus_fader = stage("bus", TrackStage::PostFader);
        let bus_route_id = id("routebus");
        let bus_route = GraphNodeId::Route {
            route_id: bus_route_id.clone(),
        };
        let output = GraphNodeId::Output {
            output_id: id("main"),
        };
        let port = |node: &GraphNodeId, kind| crate::GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        let edge = |id, source: &GraphNodeId, destination: &GraphNodeId| crate::GraphEdge {
            id,
            source: port(source, crate::GraphPortKind::MainOutput),
            destination: port(destination, crate::GraphPortKind::MainInput),
            path: "$.issue886".to_owned(),
        };
        let main = |target: &GraphNodeId| GraphEdgeId::TrackMain {
            target: target.clone(),
        };
        let mut edges = Vec::new();
        for track in 0..4 {
            edges.push(edge(main(&members[track]), &inputs[track], &members[track]));
            edges.push(edge(
                GraphEdgeId::RouteSource {
                    route_id: route_ids[track].clone(),
                },
                &members[track],
                &routes[track],
            ));
            edges.push(edge(
                GraphEdgeId::RouteDestination {
                    route_id: route_ids[track].clone(),
                },
                &routes[track],
                &bus_input,
            ));
        }
        for (pad, end) in pads.iter().zip(&pad_ends) {
            edges.push(edge(main(end), pad, end));
        }
        edges.push(edge(main(&bus_member), &bus_input, &bus_member));
        edges.push(edge(main(&bus_fader), &bus_member, &bus_fader));
        edges.push(edge(
            GraphEdgeId::RouteSource {
                route_id: bus_route_id.clone(),
            },
            &bus_fader,
            &bus_route,
        ));
        edges.push(edge(
            GraphEdgeId::RouteDestination {
                route_id: bus_route_id,
            },
            &bus_route,
            &output,
        ));
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let levels: Vec<Vec<GraphNodeId>> = vec![
            pads.iter().chain(&inputs).cloned().collect(),
            pad_ends.iter().chain(&members).cloned().collect(),
            routes.clone(),
            vec![bus_input.clone()],
            vec![bus_member.clone()],
            vec![bus_fader.clone()],
            vec![bus_route.clone()],
            vec![output.clone()],
        ];
        let schedule: Vec<_> = levels.iter().flatten().cloned().collect();
        let mut nodes: Vec<_> = schedule
            .iter()
            .cloned()
            .map(|id| crate::GraphNode {
                id,
                latency: effect_contract::LatencySamples(0),
                tail: effect_contract::TailSamples::Finite(0),
            })
            .collect();
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        let envelope = engine::realtime::RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: engine::QuantumFrames(FRAMES),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let bank = |members: Vec<GraphNodeId>| GraphPreparedBuiltinBank {
            backend: lane::Backend::Simd4,
            members: members.into_boxed_slice(),
            processor: Box::new(CrossTilt),
            scratch: AoSoaScratch::new(BankWidth::Four, FRAMES).expect("scratch"),
        };
        let transform = |index: usize| RouteTransform {
            gain: 0.5 + 0.0625 * index as f32,
            ll: 0.875,
            lr: -0.25 + 0.03125 * index as f32,
            rl: 0.3,
            rr: 1.125 - 0.046875 * index as f32,
        };
        let required_bindings: Vec<_> = inputs
            .iter()
            .chain(&members)
            .chain(&pads)
            .chain(&pad_ends)
            .chain([&bus_input, &bus_member, &bus_fader, &output])
            .cloned()
            .collect();
        let plan = crate::PreparedGraphPlan::new(crate::PreparedGraphPlanParts {
            plan_id: 886,
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels
                .iter()
                .enumerate()
                .map(|(level, nodes)| crate::DependencyLevel {
                    level: level as u64,
                    nodes: nodes.clone(),
                })
                .collect(),
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: crate::GraphResourceEstimate {
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
            },
            envelope,
            required_bindings,
            routes: routes
                .iter()
                .chain([&bus_route])
                .enumerate()
                .map(|(index, node)| crate::PreparedRoute {
                    node: node.clone(),
                    transform: transform(index),
                })
                .collect(),
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: vec![bank(members.clone()), bank(vec![bus_member])],
            observers: Vec::new(),
        });
        let mut bindings: Vec<GraphNodeBinding> = inputs
            .iter()
            .chain(&pads)
            .enumerate()
            .map(|(seed, node)| {
                GraphNodeBinding::new(node.clone(), Box::new(NoiseInput(seed as u32)))
            })
            .collect();
        bindings.extend(
            pad_ends
                .iter()
                .chain([&bus_fader])
                .map(|node| GraphNodeBinding::new(node.clone(), Box::new(ScalarTilt))),
        );
        bindings.push(GraphNodeBinding::identity(bus_input));
        bindings.push(GraphNodeBinding::identity(output.clone()));
        let mut observers = Vec::new();
        if let Some(published) = output_meters {
            for handle in [OUTPUT_METER_HANDLE, OUTPUT_METER_HANDLE + 1] {
                observers.push(GraphNodeObserverBinding::new(
                    output.clone(),
                    handle,
                    Box::new(WordMeter {
                        handle,
                        accepts_resident: false,
                        published: Arc::clone(published),
                    }),
                ));
            }
        }
        (
            plan,
            crate::GraphRuntimeBindings {
                envelope,
                nodes: bindings,
                observers,
            },
        )
    }

    /// A chain that redirects its scatter after a folded chain binds, and redirects its own lane.
    ///
    /// `apply_scatter_redirects` indexed `units` by run, and the fold retires route runs without
    /// emitting units for them; in this plan the bus's run is 16 and `units` has 16 entries, so the
    /// folded bind panicked (index out of bounds). Where more units follow, the same index lands on
    /// another unit instead. The oracle is the same plan with the fold declined: the redirect is
    /// then the only index in play and both arms must render the master bit for bit.
    ///
    /// Red mutation: index `units[run]` again -- the folded arm panics at bind.
    #[test]
    fn a_redirect_after_a_retired_route_lands_on_its_own_chain() {
        const BLOCKS: u64 = 6;
        let mut unfolded = folded_bus_plan(true);
        assert_eq!(
            [
                unfolded.bank_scatter_redirects(),
                unfolded.bank_route_folds()
            ],
            [1, 0],
            "the oracle redirects the bus's lane and folds nothing"
        );
        let mut folded = folded_bus_plan(false);
        assert_eq!(
            [folded.bank_scatter_redirects(), folded.bank_route_folds()],
            [1, 4],
            "the tracks' routes fold into the bus, and the bus still redirects"
        );
        let oracle = render_fold_fixture(&mut unfolded, 13, BLOCKS);
        assert!(
            oracle.iter().any(|word| *word != 0),
            "the plan renders audio"
        );
        assert_eq!(
            render_fold_fixture(&mut folded, 13, BLOCKS),
            oracle,
            "the folded plan's master is the unfolded plan's bits"
        );
    }

    // -----------------------------------------------------------------------------------------
    // Issue #916: the session Output op's storage is the host's planes.
    // -----------------------------------------------------------------------------------------

    /// Bind `plan` straight into a [`crate::GraphExecutor`], the way `bind_optional_source_set`
    /// does, and hand back its lowered program. A test that reads the executor's arena after a
    /// render cannot go through `PreparedRenderPlan`, which hides the executor. `controlled`
    /// binds through the activation catalog and activates every observer, as `fold_fixture` does.
    fn bind_executor(
        plan: crate::PreparedGraphPlan,
        bindings: crate::GraphRuntimeBindings,
        controlled: bool,
        source_set: Option<crate::GraphPreparedSourceSet>,
    ) -> (
        crate::GraphExecutor,
        ExecutionProgram,
        Option<crate::GraphObservationController>,
    ) {
        let program = plan.lowered().expect("lowered");
        let planning = preflight_sequential(&plan, &program, &bindings, source_set.as_ref())
            .expect("preflight");
        let handles: Vec<u64> = plan
            .observers
            .iter()
            .chain(&bindings.observers)
            .map(|observer| observer.handle)
            .collect();
        let config = controlled.then_some(crate::GraphObservationActivationConfig {
            maximum_active_observers: handles.len(),
            maximum_retained_bytes: u64::MAX,
        });
        let prepared =
            preflight_observation_activation(&plan, &program, &bindings, &planning, config)
                .expect("activation preflight");
        let (controller, realtime) = prepared.map_or((None, None), |prepared| {
            (Some(prepared.controller), Some(prepared.realtime))
        });
        let mut plan = plan;
        let mut bindings = bindings;
        let mut observers = core::mem::take(&mut plan.observers);
        observers.append(&mut bindings.observers);
        let executor = crate::GraphExecutor::new(
            plan,
            &program,
            bindings.nodes,
            observers,
            source_set,
            planning,
            realtime,
        );
        let controller = controller.map(|mut controller| {
            controller
                .replace(&handles)
                .expect("activate every observer");
            controller
        });
        (executor, program, controller)
    }

    /// `Input -> PostInputBuiltins (a one-lane W4 builtin bank) -> Output`, one track, no route
    /// (issue #916). The bank's last slot is dedicated storage whose sole reader is the Output op,
    /// so `scatter_target` offers a redirect *into the Output op*, which `build_sequential`
    /// withholds. The arena oracle, bound with the host master declined, takes it.
    fn direct_bank_parts(
        output_meters: Option<&Published>,
    ) -> (crate::PreparedGraphPlan, crate::GraphRuntimeBindings) {
        const FRAMES: u32 = 13;
        let id = |text: &str| crate::StableGraphId::parse(text).expect("stable id");
        let input = GraphNodeId::TrackStage {
            track_id: id("track00"),
            stage: TrackStage::Input,
        };
        let member = GraphNodeId::TrackStage {
            track_id: id("track00"),
            stage: TrackStage::PostInputBuiltins,
        };
        let output = GraphNodeId::Output {
            output_id: id("main"),
        };
        let port = |node: &GraphNodeId, kind| crate::GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        let edge = |id, source: &GraphNodeId, destination: &GraphNodeId| crate::GraphEdge {
            id,
            source: port(source, crate::GraphPortKind::MainOutput),
            destination: port(destination, crate::GraphPortKind::MainInput),
            path: "$.issue916".to_owned(),
        };
        let mut edges = vec![
            edge(
                GraphEdgeId::TrackMain {
                    target: member.clone(),
                },
                &input,
                &member,
            ),
            edge(
                GraphEdgeId::RouteSource {
                    route_id: id("route00"),
                },
                &member,
                &output,
            ),
        ];
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let levels = [
            vec![input.clone()],
            vec![member.clone()],
            vec![output.clone()],
        ];
        let schedule: Vec<_> = levels.iter().flatten().cloned().collect();
        let mut nodes: Vec<_> = schedule
            .iter()
            .cloned()
            .map(|id| crate::GraphNode {
                id,
                latency: effect_contract::LatencySamples(0),
                tail: effect_contract::TailSamples::Finite(0),
            })
            .collect();
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        let envelope = engine::realtime::RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: engine::QuantumFrames(FRAMES),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let plan = crate::PreparedGraphPlan::new(crate::PreparedGraphPlanParts {
            plan_id: 916,
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels
                .iter()
                .enumerate()
                .map(|(level, nodes)| crate::DependencyLevel {
                    level: level as u64,
                    nodes: nodes.clone(),
                })
                .collect(),
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: crate::GraphResourceEstimate {
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
            },
            envelope,
            required_bindings: vec![input.clone(), member.clone(), output.clone()],
            routes: Vec::new(),
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: vec![GraphPreparedBuiltinBank {
                backend: lane::Backend::Simd4,
                members: vec![member].into_boxed_slice(),
                processor: Box::new(CrossTilt),
                scratch: AoSoaScratch::new(BankWidth::Four, FRAMES).expect("scratch"),
            }],
            observers: Vec::new(),
        });
        let mut observers = Vec::new();
        if let Some(published) = output_meters {
            for handle in [OUTPUT_METER_HANDLE, OUTPUT_METER_HANDLE + 1] {
                observers.push(GraphNodeObserverBinding::new(
                    output.clone(),
                    handle,
                    Box::new(WordMeter {
                        handle,
                        accepts_resident: false,
                        published: Arc::clone(published),
                    }),
                ));
            }
        }
        (
            plan,
            crate::GraphRuntimeBindings {
                envelope,
                nodes: vec![
                    GraphNodeBinding::new(input, Box::new(NoiseInput(7))),
                    GraphNodeBinding::identity(output),
                ],
                observers,
            },
        )
    }

    /// NaN padding with a recognisable payload: the words a host plane holds before a render.
    const HOST_PAD: u32 = 0x7fc0_0916;
    /// The pattern the arena buffer of the session Output is filled with before every block.
    const OUTPUT_SLOT_FILL: [u32; 2] = [0x7fc1_0916, 0x7fc2_0916];

    fn stereo_bits((left, right): (&[f32], &[f32])) -> Vec<u32> {
        left.iter()
            .chain(right)
            .map(|word| word.to_bits())
            .collect()
    }

    /// The session output's arena buffer of a bound executor.
    fn output_slot(program: &ExecutionProgram) -> u32 {
        program.output.0 + ARENA_BASE
    }

    fn fill_output_slot(executor: &mut crate::GraphExecutor, program: &ExecutionProgram) {
        let (left, right) = executor.runtime.buffer_mut(output_slot(program));
        left.fill(f32::from_bits(OUTPUT_SLOT_FILL[0]));
        right.fill(f32::from_bits(OUTPUT_SLOT_FILL[1]));
    }

    /// One host-plane render through `GraphExecutor::render`, the executor half of
    /// `PreparedRenderPlan::render`.
    fn render_host(
        executor: &mut crate::GraphExecutor,
        output: engine::realtime::PlanarBufferMut<'_>,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        use engine::realtime::PreparedPlanExecutor as _;
        let mut arena = engine::realtime::BufferArena::try_new(&[]).expect("empty arena");
        executor.render(
            &mut arena,
            None,
            output,
            engine::realtime::RenderTime {
                absolute_sample: first_sample,
            },
        )
    }

    /// Issue #916's arena oracle: the render loop `GraphExecutor::render` ran before the issue,
    /// over a runtime bound with the host master declined, then its end-of-block copy of the
    /// Output's arena buffer into `output`.
    ///
    /// Returns that buffer's words as they stood just before the first unit that writes the
    /// master into it. That unit is the Output op's own, unless the Output op's reduction was
    /// neutralised to its own slot. Then the master was written earlier, by the chains that fold
    /// into it or by a chain whose scatter was redirected into it. A host-plane render must leave
    /// the buffer holding exactly those words, because only the master's writers touch it after
    /// that point, and they now write the host's planes.
    fn render_arena_oracle(
        executor: &mut crate::GraphExecutor,
        program: &ExecutionProgram,
        mut output: engine::realtime::PlanarBufferMut<'_>,
        first_sample: u64,
    ) -> Vec<u32> {
        let slot = output_slot(program);
        let runtime = &mut executor.runtime;
        assert_eq!(
            runtime.output_unit, None,
            "the oracle binds with no Output unit"
        );
        let frames = runtime.lease.frames();
        let untouched = f32::from_bits(HOST_PAD);
        let (mut left, mut right) = (vec![untouched; frames], vec![untouched; frames]);
        let mut host = HostMaster::new(&mut left, &mut right, frames).expect("oracle host");
        // The Output op is the last op writing its slot: dedicated storage is never recycled.
        let output_unit = (0..runtime.units.len())
            .rev()
            .find(|unit| matches!(&runtime.units[*unit], RuntimeUnit::Op(op) if op.output == slot))
            .expect("the Output op's unit");
        let neutralised = matches!(
            &runtime.units[output_unit],
            RuntimeUnit::Op(op) if *op.inputs == [slot]
        );
        let writes_slot = |unit: &RuntimeUnit| match unit {
            RuntimeUnit::Bank {
                members,
                lanes,
                fold,
                master,
                ..
            } => {
                (!fold.is_empty() && *master == FoldTarget::Arena(slot))
                    || members[members.len() - lanes..]
                        .iter()
                        .any(|member| member.output == slot)
            }
            RuntimeUnit::Op(_) => false,
        };
        runtime.begin_observation_block(first_sample);
        let selective = runtime.has_observation_activation();
        let active = runtime.has_active_observation();
        let validity = GraphObservationValidity::CLEAR;
        let mut before_master = None;
        for unit in 0..runtime.units.len() {
            let writes_master =
                unit == output_unit || (neutralised && writes_slot(&runtime.units[unit]));
            if writes_master && before_master.is_none() {
                before_master = Some(stereo_bits(runtime.buffer(slot)));
            }
            runtime
                .execute(unit, first_sample, host.reborrow(), None)
                .expect("oracle unit");
            if !selective {
                runtime
                    .observe_unit(unit, first_sample, validity, &host)
                    .expect("oracle observer");
            } else if active {
                runtime
                    .observe_active_unit(unit, first_sample, validity, &host)
                    .expect("oracle observer");
            }
        }
        // The executor's end-of-block copy, exactly as it stood before issue #916.
        let (arena_left, arena_right) = runtime.buffer(slot);
        output
            .plane_mut(0)
            .expect("left")
            .copy_from_slice(arena_left);
        output
            .plane_mut(1)
            .expect("right")
            .copy_from_slice(arena_right);
        assert!(
            left.iter()
                .chain(&right)
                .all(|word| word.to_bits() == HOST_PAD),
            "an arena-oracle unit wrote the host planes"
        );
        before_master.expect("a master writer ran")
    }

    /// Which issue #916 gate-1 plan to bind.
    #[derive(Clone, Copy, Debug)]
    enum HostShape {
        /// 64 tracks in eight W8 cohorts, every route folded into the Output: the console shape.
        /// `frames`, and whether the meters bind through the activation catalog.
        Folded(u32, bool),
        /// The same plan with the fold declined: 64 route ops and a 64-input reduction.
        Unfolded,
        /// The bus plan with its three pads: the tracks fold into the bus, the bus redirects its
        /// scatter into its fader, and the Output's one input is the bus route. The Output's slot
        /// is a pad's.
        Submix,
        /// The bus plan with no pads: nothing folds, and the Output's slot is a track's input
        /// slot, which the tracks' bank gathers and whose route runs in place on it.
        SubmixOnATrackSlot,
        /// One track, its route folded into the Output: a fan-in-one master that only stores.
        OneRouteFolded,
        /// The same with the fold declined: the Output op's fan-in-one copy.
        OneRouteUnfolded,
        /// [`direct_bank_parts`]: a bank's last slot straight into the Output op.
        BankIntoOutput,
    }

    impl HostShape {
        const ALL: [Self; 9] = [
            Self::Folded(13, false),
            Self::Folded(13, true),
            Self::Folded(128, false),
            Self::Unfolded,
            Self::Submix,
            Self::SubmixOnATrackSlot,
            Self::OneRouteFolded,
            Self::OneRouteUnfolded,
            Self::BankIntoOutput,
        ];

        const fn frames(self) -> u32 {
            match self {
                Self::Folded(frames, _) => frames,
                _ => 13,
            }
        }

        /// `[bank_route_folds, bank_scatter_redirects]` of the host-plane plan.
        const fn shape(self) -> [u64; 2] {
            match self {
                Self::Folded(..) => [64, 0],
                Self::Unfolded | Self::OneRouteUnfolded | Self::BankIntoOutput => [0, 0],
                Self::Submix => [4, 1],
                Self::SubmixOnATrackSlot => [0, 1],
                Self::OneRouteFolded => [1, 0],
            }
        }

        /// Bind the host-plane plan, or with `oracle` the arena oracle of the same plan.
        fn bind(
            self,
            oracle: bool,
            published: &Published,
        ) -> (crate::GraphExecutor, ExecutionProgram) {
            let console = |tracks, width, frames, controlled| {
                let fixture = FoldFixture {
                    output_meters: true,
                    controlled,
                    ..FoldFixture::metered(width, tracks, frames)
                };
                let (plan, bindings, _) = fold_fixture_parts(fixture, published);
                (plan, bindings, controlled)
            };
            let (plan, bindings, controlled) = match self {
                Self::Folded(frames, controlled) => {
                    console(64, BankWidth::Eight, frames, controlled)
                }
                Self::Unfolded => console(64, BankWidth::Eight, 13, false),
                Self::OneRouteFolded | Self::OneRouteUnfolded => {
                    console(1, BankWidth::Four, 13, false)
                }
                Self::Submix => {
                    let (plan, bindings) = folded_bus_parts(3, Some(published));
                    (plan, bindings, false)
                }
                Self::SubmixOnATrackSlot => {
                    let (plan, bindings) = folded_bus_parts(0, Some(published));
                    (plan, bindings, false)
                }
                Self::BankIntoOutput => {
                    let (plan, bindings) = direct_bank_parts(Some(published));
                    (plan, bindings, false)
                }
            };
            let declined = matches!(self, Self::Unfolded | Self::OneRouteUnfolded);
            test_only_set_route_fold_declined(declined);
            test_only_set_host_master_declined(oracle);
            let (executor, program, controller) = bind_executor(plan, bindings, controlled, None);
            test_only_set_route_fold_declined(false);
            test_only_set_host_master_declined(false);
            // The controller only publishes the activation snapshot; the runtime owns the rest.
            drop(controller);
            (executor, program)
        }
    }

    /// Gate 1 of issue #916: the host's planes after a render are the arena oracle's master bit
    /// for bit, at a stride equal to the block and seven words wider, with every padding word
    /// untouched; every observer window, the two Output meters' included, is the oracle's; and
    /// the session output's arena buffer is left holding exactly what it held before the
    /// master's first writer ran, so neither the Output op nor a folded Output master writes it.
    ///
    /// The oracle is [`render_arena_oracle`]: the same plan bound with the host master declined,
    /// driven through the render loop that preceded the issue, then copied out of the arena as
    /// the executor's end-of-block copy did. The shapes cover the three the brief names -- 64
    /// folded routes, 64 unfolded routes, and one submix into the Output -- plus a submix whose
    /// Output slot is a gathered track input, a folded and an unfolded fan-in-one route, and a
    /// bank straight into the Output op (whose scatter redirect is withheld).
    ///
    /// Red mutations (`crates/graph/tests/MUTATIONS.md`, rows 916-1 to 916-6, 916-10, 916-11 and
    /// 916-15). They restore the arena path with the end-of-block copy, which only the
    /// output-slot check sees. They write the Output op's master to the arena and skip the host,
    /// install every folded Output master as an arena master, and read the fan-in-one input from
    /// the other plane. They identify the Output op by buffer index, stop withholding the redirect
    /// into the Output op, and fill the planes on success. They undedicate the Output, and point
    /// its observers at the arena.
    #[test]
    fn the_host_planes_are_the_arena_oracles_master_bit_for_bit_at_every_stride() {
        const BLOCKS: u64 = 4;
        for shape in HostShape::ALL {
            let frames = shape.frames() as usize;
            for stride in [frames, frames + 7] {
                let (oracle_published, published) = (Published::default(), Published::default());
                let (mut oracle, oracle_program) = shape.bind(true, &oracle_published);
                let (mut candidate, program) = shape.bind(false, &published);
                assert_eq!(program, oracle_program, "{shape:?}: one lowered program");
                let redirects = match shape {
                    // The oracle takes the redirect into the Output op; the host-plane plan
                    // withholds it and changes nothing else.
                    HostShape::BankIntoOutput => 1,
                    _ => shape.shape()[1],
                };
                assert_eq!(
                    [
                        candidate.runtime.route_folds(),
                        candidate.runtime.scatter_redirects()
                    ],
                    shape.shape(),
                    "{shape:?}: [route folds, scatter redirects]"
                );
                assert_eq!(
                    [
                        oracle.runtime.route_folds(),
                        oracle.runtime.scatter_redirects()
                    ],
                    [shape.shape()[0], redirects],
                    "{shape:?}: the oracle's [route folds, scatter redirects]"
                );
                {
                    use engine::realtime::PreparedPlanExecutor as _;
                    assert_eq!(candidate.bank_shape(), oracle.bank_shape(), "{shape:?}");
                }
                let unit = candidate.runtime.output_unit.expect("an Output unit");
                assert!(
                    matches!(&candidate.runtime.units[unit], RuntimeUnit::Op(op) if op.output == output_slot(&program)),
                    "{shape:?}: the Output unit runs the op that owns the output slot"
                );
                let mut masters = Vec::new();
                for block in 0..BLOCKS {
                    let first_sample = block * frames as u64;
                    fill_output_slot(&mut oracle, &oracle_program);
                    fill_output_slot(&mut candidate, &program);
                    let mut expected = vec![f32::from_bits(HOST_PAD); 2 * stride];
                    let before_master = render_arena_oracle(
                        &mut oracle,
                        &oracle_program,
                        engine::realtime::PlanarBufferMut::try_new(
                            &mut expected,
                            2,
                            frames,
                            stride,
                        )
                        .expect("oracle output"),
                        first_sample,
                    );
                    let mut actual = vec![f32::from_bits(HOST_PAD); 2 * stride];
                    render_host(
                        &mut candidate,
                        engine::realtime::PlanarBufferMut::try_new(&mut actual, 2, frames, stride)
                            .expect("host output"),
                        first_sample,
                    )
                    .expect("host-plane render");
                    let bits =
                        |words: &[f32]| words.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
                    assert_eq!(
                        bits(&actual),
                        bits(&expected),
                        "{shape:?}, stride {stride}, block {block}: the host planes are the oracle's"
                    );
                    for (index, word) in actual.iter().enumerate() {
                        if index % stride >= frames {
                            assert_eq!(
                                word.to_bits(),
                                HOST_PAD,
                                "{shape:?}, stride {stride}, block {block}: padding word {index}"
                            );
                        }
                    }
                    assert_eq!(
                        stereo_bits(candidate.runtime.buffer(output_slot(&program))),
                        before_master,
                        "{shape:?}, stride {stride}, block {block}: the output slot was written"
                    );
                    let master: Vec<u32> = actual[..frames]
                        .iter()
                        .chain(&actual[stride..stride + frames])
                        .map(|word| word.to_bits())
                        .collect();
                    assert!(
                        master.iter().any(|word| *word != 0 && *word != HOST_PAD),
                        "{shape:?}: the block carries audio"
                    );
                    masters.push(master);
                }
                let frames_of = |published: &Published| published.lock().unwrap().clone();
                let (oracle_frames, frames_seen) =
                    (frames_of(&oracle_published), frames_of(&published));
                assert_eq!(
                    frames_seen, oracle_frames,
                    "{shape:?}, stride {stride}: every observer window is the oracle's"
                );
                let output_windows: Vec<&MeterFrame> = frames_seen
                    .iter()
                    .filter(|frame| frame.handle >= OUTPUT_METER_HANDLE)
                    .collect();
                assert_eq!(
                    output_windows.len(),
                    2 * BLOCKS as usize,
                    "{shape:?}: both Output meters saw every block"
                );
                for window in output_windows {
                    let block = (window.first_sample / frames as u64) as usize;
                    let seen: Vec<u32> = window.left.iter().chain(&window.right).copied().collect();
                    assert_eq!(
                        seen, masters[block],
                        "{shape:?}: an Output meter reads the planes the host receives"
                    );
                }
            }
        }
    }

    /// `t00 Input -> Output`, and with `read_output` also `Output -> t01 PostFader`: a hand-built
    /// plan with a consumer of the session output, which the graph compiler never emits.
    fn output_reader_parts(
        read_output: bool,
    ) -> (crate::PreparedGraphPlan, crate::GraphRuntimeBindings) {
        const FRAMES: u32 = 13;
        let id = |text: &str| crate::StableGraphId::parse(text).expect("stable id");
        let input = GraphNodeId::TrackStage {
            track_id: id("t00"),
            stage: TrackStage::Input,
        };
        let output = GraphNodeId::Output {
            output_id: id("main"),
        };
        let reader = GraphNodeId::TrackStage {
            track_id: id("t01"),
            stage: TrackStage::PostFader,
        };
        let port = |node: &GraphNodeId, kind| crate::GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        let edge = |source: &GraphNodeId, destination: &GraphNodeId| crate::GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: destination.clone(),
            },
            source: port(source, crate::GraphPortKind::MainOutput),
            destination: port(destination, crate::GraphPortKind::MainInput),
            path: "$.issue916.output_reader".to_owned(),
        };
        let mut edges = vec![edge(&input, &output)];
        let mut levels = vec![vec![input.clone()], vec![output.clone()]];
        if read_output {
            edges.push(edge(&output, &reader));
            levels.push(vec![reader.clone()]);
        }
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let schedule: Vec<_> = levels.iter().flatten().cloned().collect();
        let mut nodes: Vec<_> = schedule
            .iter()
            .cloned()
            .map(|id| crate::GraphNode {
                id,
                latency: effect_contract::LatencySamples(0),
                tail: effect_contract::TailSamples::Finite(0),
            })
            .collect();
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        let envelope = engine::realtime::RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: engine::QuantumFrames(FRAMES),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let plan = crate::PreparedGraphPlan::new(crate::PreparedGraphPlanParts {
            plan_id: 9_160,
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule.clone(),
            dependency_levels: levels
                .iter()
                .enumerate()
                .map(|(level, nodes)| crate::DependencyLevel {
                    level: level as u64,
                    nodes: nodes.clone(),
                })
                .collect(),
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: crate::GraphResourceEstimate {
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
            },
            envelope,
            required_bindings: schedule,
            routes: Vec::new(),
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let mut nodes = vec![
            GraphNodeBinding::new(input, Box::new(NoiseInput(9))),
            GraphNodeBinding::identity(output),
        ];
        if read_output {
            nodes.push(GraphNodeBinding::identity(reader));
        }
        (
            plan,
            crate::GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            },
        )
    }

    /// Issue #916's precondition, enforced: a plan that reads the session output's value out of
    /// the arena is refused at bind with `graph.scheduler.layout`.
    ///
    /// The Output op writes the host's planes and never its arena slot, so a consumer of the
    /// Output would read words no op wrote this block. `PreparedGraphPlan::validate` accepts an
    /// edge out of the Output node, and the lowering counts it as a read like any other. The
    /// refusal is `preflight_sequential`'s `output_value_is_read`, before any owner moves. The
    /// control is the same plan without the reader: it binds and renders.
    ///
    /// Red mutation (`crates/graph/tests/MUTATIONS.md` row 916-18): drop the refusal. The plan
    /// then binds, and its reader reads the never-written slot.
    #[test]
    fn a_plan_that_reads_the_session_output_is_refused_at_bind() {
        let (plan, bindings) = output_reader_parts(true);
        let program = plan.lowered().expect("the plan lowers");
        let output_op = output_op(&program, &plan.spec).expect("an Output op");
        let (readers, _) = op_dataflow(&program);
        assert_eq!(
            readers[output_op].len(),
            1,
            "the lowering counts the edge out of the Output as a read"
        );
        let failure = plan
            .bind(bindings)
            .err()
            .expect("a plan that reads the session output must not bind");
        assert_eq!(failure.code, "graph.scheduler.layout");

        let (plan, bindings) = output_reader_parts(false);
        let mut bound = plan
            .bind(bindings)
            .unwrap_or_else(|failure| panic!("control bind: {}", failure.code));
        let bits = render_fold_fixture(&mut bound, 13, 2);
        assert!(bits.iter().any(|word| *word != 0), "the control renders");
    }

    /// An observer that accepts every block before `from_sample` and fails every block from it.
    struct FailingObserver {
        from_sample: u64,
    }

    impl crate::GraphRuntimeObserver for FailingObserver {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            if block.first_sample >= self.from_sample {
                return Err(RenderError::InvalidEnvelope);
            }
            Ok(())
        }
    }

    /// A one-claim source set that writes [`NoiseInput`]'s words and, from `from_sample` on,
    /// fails in `begin_block` (`in_begin`) or in `copy_track_input`.
    struct FailingSource {
        first_sample: u64,
        from_sample: u64,
        in_begin: bool,
    }

    impl crate::GraphPreparedSourceSetDriver for FailingSource {
        fn claim_count(&self) -> usize {
            1
        }
        fn begin_block(&mut self, first_sample: u64, _frames: u32) -> Result<(), RenderError> {
            self.first_sample = first_sample;
            if self.in_begin && first_sample >= self.from_sample {
                return Err(RenderError::InvalidEnvelope);
            }
            Ok(())
        }
        fn copy_track_input(
            &mut self,
            _claim: usize,
            left: &mut [f32],
            right: &mut [f32],
        ) -> Result<(), RenderError> {
            if !self.in_begin && self.first_sample >= self.from_sample {
                return Err(RenderError::InvalidEnvelope);
            }
            NoiseInput(0).process(GraphBindingBlock {
                left,
                right,
                first_sample: self.first_sample,
            })
        }
    }

    /// Where issue #916's gate-3 render fails.
    #[derive(Clone, Copy, Debug)]
    enum HostFailure {
        /// Cohort 5 of 8 fails its bank processor, after five folded cohorts wrote the host's
        /// planes.
        Unit,
        /// An Output observer fails after the Output op wrote the whole master; `true` binds it
        /// through the activation catalog (`observe_active_unit`).
        Observer(bool),
        /// The source set fails before any unit runs, in `begin_block` (`true`) or in
        /// `copy_track_input`, over planes still holding the previous block's master.
        Source(bool),
    }

    /// Gate 3 of issue #916: an executor-level failure fills both host planes with `+0.0` before
    /// it returns, and a rejection made before any unit runs leaves them holding their words.
    ///
    /// Every failing render follows a good one into the same storage, as a host renders, so the
    /// planes hold a whole master when the failing block starts. Each failure then leaves a
    /// different partial state for the fill to erase: part of the next master (a failed unit),
    /// all of it (a failed Output observer), or the previous block's (a failed source set). The
    /// padding past `frames` is never written, failure or not.
    ///
    /// Red mutations (`crates/graph/tests/MUTATIONS.md` rows 916-7, 916-8, 916-9, 916-12, 916-13
    /// and 916-14). One skips the fill on each failure path: on the unit path the `Unit` arm keeps
    /// five cohorts' partial master. Another drops the planes' length check, so the short
    /// rejection panics in the fold epilogue instead of being refused.
    #[test]
    fn a_failed_render_silences_the_host_planes_and_a_rejected_one_leaves_them_alone() {
        const FRAMES: u32 = 13;
        let frames = FRAMES as usize;
        let stride = frames + 7;
        let pad = f32::from_bits(HOST_PAD);
        let padding_kept = |storage: &[f32]| {
            (0..2 * stride)
                .filter(|index| index % stride >= frames)
                .all(|index| storage[index].to_bits() == HOST_PAD)
        };
        let silent = |storage: &[f32]| {
            (0..2 * stride)
                .filter(|index| index % stride < frames)
                .all(|index| storage[index].to_bits() == 0)
        };
        let bits = |storage: &[f32]| {
            storage
                .iter()
                .map(|word| word.to_bits())
                .collect::<Vec<_>>()
        };
        let output = GraphNodeId::Output {
            output_id: crate::StableGraphId::parse("main").expect("output id"),
        };
        let quiet = |width, tracks| FoldFixture {
            metered: false,
            ..FoldFixture::metered(width, tracks, FRAMES)
        };
        for failure in [
            HostFailure::Unit,
            HostFailure::Observer(false),
            HostFailure::Observer(true),
            HostFailure::Source(true),
            HostFailure::Source(false),
        ] {
            let published = Published::default();
            let (executor, folds) = match failure {
                HostFailure::Unit => {
                    let fixture = FoldFixture {
                        failing_cohort: Some(5),
                        ..quiet(BankWidth::Eight, 64)
                    };
                    let (plan, bindings, _) = fold_fixture_parts(fixture, &published);
                    (bind_executor(plan, bindings, false, None).0, 64)
                }
                HostFailure::Observer(controlled) => {
                    let (plan, mut bindings, _) =
                        fold_fixture_parts(quiet(BankWidth::Eight, 64), &published);
                    let observer = Box::new(FailingObserver {
                        from_sample: u64::from(FRAMES),
                    });
                    bindings.observers.push(if controlled {
                        GraphNodeObserverBinding::controlled(output.clone(), 7, observer)
                    } else {
                        GraphNodeObserverBinding::new(output.clone(), 7, observer)
                    });
                    (bind_executor(plan, bindings, controlled, None).0, 64)
                }
                HostFailure::Source(in_begin) => {
                    let (plan, mut bindings, _) =
                        fold_fixture_parts(quiet(BankWidth::Four, 1), &published);
                    let input = bindings.nodes.remove(0).node;
                    assert!(matches!(
                        input,
                        GraphNodeId::TrackStage {
                            stage: TrackStage::Input,
                            ..
                        }
                    ));
                    let source = crate::GraphPreparedSourceSet::new(
                        plan.envelope,
                        vec![crate::GraphSourceInputClaim { node: input }],
                        crate::GraphSourceSetResourceReport {
                            pcm_payload_already_charged_bytes: 0,
                            overhead_bytes: 0,
                            total_engine_owned_bytes: 0,
                            largest_allocation_bytes: 0,
                        },
                        Box::new(FailingSource {
                            first_sample: 0,
                            from_sample: u64::from(FRAMES),
                            in_begin,
                        }),
                    );
                    (bind_executor(plan, bindings, false, Some(source)).0, 1)
                }
            };
            let mut executor = executor;
            assert_eq!(executor.runtime.route_folds(), folds, "{failure:?}: folded");
            let mut storage = vec![pad; 2 * stride];
            fn view(
                storage: &mut [f32],
                frames: usize,
                stride: usize,
            ) -> engine::realtime::PlanarBufferMut<'_> {
                engine::realtime::PlanarBufferMut::try_new(storage, 2, frames, stride)
                    .expect("host output")
            }
            render_host(&mut executor, view(&mut storage, frames, stride), 0)
                .expect("the first block renders");
            assert!(
                !silent(&storage) && padding_kept(&storage),
                "{failure:?}: the first block wrote a master"
            );
            assert_eq!(
                render_host(
                    &mut executor,
                    view(&mut storage, frames, stride),
                    u64::from(FRAMES)
                ),
                Err(RenderError::InvalidEnvelope),
                "{failure:?}: the second block fails"
            );
            assert!(silent(&storage), "{failure:?}: both planes are +0.0");
            assert!(
                padding_kept(&storage),
                "{failure:?}: the padding is untouched"
            );
        }

        // Rejections made before any unit runs keep the planes' words: the executor's own length
        // check, its stereo check, and the plan's `OutputShape` check in front of both.
        let published = Published::default();
        let (plan, bindings, _) = fold_fixture_parts(quiet(BankWidth::Eight, 64), &published);
        let (mut executor, _, _) = bind_executor(plan, bindings, false, None);
        let mut storage = vec![pad; 2 * stride];
        render_host(
            &mut executor,
            engine::realtime::PlanarBufferMut::try_new(&mut storage, 2, frames, stride)
                .expect("host output"),
            0,
        )
        .expect("the first block renders");
        let rendered = bits(&storage);
        assert!(!silent(&storage));
        for (channels, block_frames, expected) in [
            (2, frames - 1, RenderError::InvalidEnvelope),
            (
                1,
                frames,
                RenderError::Buffer(engine::realtime::BufferArenaError::InvalidPlane),
            ),
        ] {
            let output = engine::realtime::PlanarBufferMut::try_new(
                &mut storage,
                channels,
                block_frames,
                stride,
            )
            .expect("a valid but mismatched layout");
            assert_eq!(
                render_host(&mut executor, output, u64::from(FRAMES)),
                Err(expected),
                "{channels} channels of {block_frames} frames"
            );
            assert_eq!(
                bits(&storage),
                rendered,
                "a rejection leaves the planes alone"
            );
        }
        let (mut plan, _) = fold_fixture(quiet(BankWidth::Eight, 64), &published);
        let mut storage = vec![pad; 2 * stride];
        let time = |block: u64| engine::realtime::RenderTime {
            absolute_sample: block * u64::from(FRAMES),
        };
        plan.render(
            engine::realtime::RenderIo {
                input: None,
                output: engine::realtime::PlanarBufferMut::try_new(&mut storage, 2, frames, stride)
                    .expect("host output"),
            },
            time(0),
        )
        .expect("the first block renders");
        let rendered = bits(&storage);
        assert_eq!(
            plan.render(
                engine::realtime::RenderIo {
                    input: None,
                    output: engine::realtime::PlanarBufferMut::try_new(
                        &mut storage,
                        2,
                        frames - 1,
                        stride
                    )
                    .expect("a valid but short layout"),
                },
                time(1),
            ),
            Err(RenderError::OutputShape)
        );
        assert_eq!(
            bits(&storage),
            rendered,
            "the plan's rejection leaves the planes alone"
        );
    }

    /// [`reduce_plane_into`] is [`reduce_plane`] with the host's plane as its destination, bit
    /// for bit, over hostile values at every fan-in from zero across two group boundaries (8 and
    /// 16) and at frame counts with and without a vector tail. The two planes carry different
    /// words, the inputs repeat one buffer and name the silence buffer, and the arena destination
    /// starts stale, so a wrong plane, a wrong order or a missed store all show. A single input
    /// that is the op's own buffer, the neutralised reduction of a folded master, leaves the
    /// host's plane untouched.
    #[test]
    fn a_host_plane_reduction_is_the_arena_reduction_bit_for_bit() {
        let mut state = 0x0916_u64;
        let bits = |words: &[f32]| words.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
        for frames in [1, 7, 8, 13, 16, 33] {
            for fan_in in 0..=19_usize {
                // Buffer 0 is the silence buffer and buffer 1 the op's own output.
                let buffers = fan_in + 2;
                let mut lease = stereo_lease(frames, buffers);
                let mut inputs: Vec<u32> = (0..fan_in).map(|input| 2 + input as u32).collect();
                if fan_in >= 3 {
                    inputs[1] = 0;
                    inputs[fan_in - 1] = inputs[0];
                }
                for buffer in 2..buffers as u32 {
                    for plane in 0..2 {
                        for word in lease.write(plane, buffer) {
                            *word = hostile_sample(&mut state);
                        }
                    }
                }
                for plane in 0..2 {
                    lease.write(plane, 1).fill(f32::from_bits(0x7fc0_0001));
                    let mut target = vec![f32::from_bits(HOST_PAD); frames];
                    reduce_plane_into(&lease, plane, 1, &mut target, &inputs);
                    reduce_plane(&mut lease, plane, 1, &inputs);
                    assert_eq!(
                        bits(&target),
                        bits(lease.read(plane, 1)),
                        "{frames} frames, fan-in {fan_in}, plane {plane}"
                    );
                }
            }
            let lease = stereo_lease(frames, 3);
            let mut target = vec![f32::from_bits(HOST_PAD); frames];
            reduce_plane_into(&lease, 0, 1, &mut target, &[1]);
            assert!(
                target.iter().all(|word| word.to_bits() == HOST_PAD),
                "a neutralised reduction leaves the host's plane alone"
            );
        }
    }

    // -----------------------------------------------------------------------------------------
    // Issue #886: the direct scatter stays armed when the chain's final output is observed.
    // -----------------------------------------------------------------------------------------

    /// `[scatter redirects, route folds]` of `fixture`'s bound plan.
    fn scatter_shape(fixture: FoldFixture) -> [u64; 2] {
        let published = Published::default();
        let (plan, _controller) = fold_fixture(fixture, &published);
        assert!(
            published.lock().unwrap().is_empty(),
            "binding renders nothing"
        );
        [plan.bank_scatter_redirects(), plan.bank_route_folds()]
    }

    /// Gate 1 of issue #886: a full bank metered at its last slot's own node and at that slot's
    /// later tap binds with every lane's scatter pointed at its consumer's buffer.
    ///
    /// The shape is the one the direct scatter exists for: a dedicated last slot
    /// (`PostInputBuiltins`) whose sole reader is a plain op that cannot run in place on it -- a
    /// bound fader, so the route fold is not possible either. A post-matrix meter cannot be this
    /// test's subject: a `PostMatrix` last slot is never dedicated storage and feeds only routes,
    /// which are not dedicated either, so its sole reader already runs in place on its buffer and
    /// `scatter_target` has nothing to redirect, metered or not (the last arm below).
    ///
    /// Arms: both meters, each alone, the controlled catalog, observers that decline the resident
    /// view, and a meter on the consumer's own node (never a clause) all keep every lane
    /// redirected; the test-only switch gate 2's oracle uses declines every lane. The last two
    /// arms are how the direct scatter composes with issue #885's fold on the route-consumer
    /// shape: unmetered, the fold takes every lane and leaves no scatter to redirect; metered at a
    /// boundary the fold does not excuse, the fold declines and the direct scatter takes every lane
    /// instead. No lane is ever both.
    ///
    /// Red mutations: restore the producer-node clause in `scatter_target` -- the arms metered at
    /// the last slot's own node report zero redirects; restore the alias clause -- the arms metered
    /// at the later tap do. Neither declines the consumer's-node arm.
    #[test]
    fn an_observer_of_the_scattered_lane_keeps_the_direct_scatter_armed() {
        for (width, tracks) in [(BankWidth::Four, 4), (BankWidth::Eight, 8)] {
            let lanes = tracks as u64;
            let metered = FoldFixture::scattered(width, tracks, 13);
            assert_eq!(
                scatter_shape(FoldFixture {
                    metered: false,
                    ..metered
                }),
                [lanes, 0],
                "{width:?}: unmetered, every lane's scatter lands in its fader"
            );
            for (name, fixture) in [
                ("the last slot and its later tap", metered),
                (
                    "the last slot's own node",
                    FoldFixture {
                        meter_at: &[TrackStage::PostInputBuiltins],
                        ..metered
                    },
                ),
                (
                    "the later tap alone",
                    FoldFixture {
                        meter_at: &[TrackStage::PostSimd1],
                        ..metered
                    },
                ),
                (
                    "the controlled catalog",
                    FoldFixture {
                        controlled: true,
                        ..metered
                    },
                ),
                (
                    "observers declining the resident view",
                    FoldFixture {
                        accepts_resident: false,
                        ..metered
                    },
                ),
                (
                    "the consumer's own node",
                    FoldFixture {
                        meter_at: &[TrackStage::PostFader],
                        ..metered
                    },
                ),
            ] {
                assert_eq!(
                    scatter_shape(fixture),
                    [lanes, 0],
                    "{width:?}: a meter at {name} keeps every lane redirected"
                );
            }
            assert_eq!(
                scatter_shape(FoldFixture {
                    scatter_declined: true,
                    ..metered
                }),
                [0, 0],
                "{width:?}: the oracle's switch declines every redirect"
            );
            let routed = FoldFixture {
                fader: false,
                ..metered
            };
            assert_eq!(
                scatter_shape(FoldFixture {
                    metered: false,
                    ..routed
                }),
                [0, lanes],
                "{width:?}: unmetered, the fold takes every lane and no scatter is left to redirect"
            );
            assert_eq!(
                scatter_shape(routed),
                [lanes, 0],
                "{width:?}: a meter the fold does not excuse declines it, and the direct scatter \
                 takes every lane instead"
            );
            // Issue #885's shape, for the record: a post-matrix last slot's route runs in place
            // on its buffer, so there is no scatter to redirect with or without the fold.
            assert_eq!(
                scatter_shape(FoldFixture {
                    fold_declined: true,
                    ..FoldFixture::metered(width, tracks, 13)
                }),
                [0, 0],
                "{width:?}: a post-matrix last slot has no redirect to keep"
            );
        }
    }

    /// Gate 2 of issue #886: the redirected, metered plan renders the master and publishes every
    /// meter frame bit for bit as the same plan with the direct scatter declined -- the path a
    /// metered last slot took before.
    ///
    /// The control arm is that plan with every observer declining the resident view, so each frame
    /// is the words the unredirected chain really scattered into the last slot's own buffer. (The
    /// same unredirected plan with resident-reading meters is checked equal to it first.) Every
    /// candidate redirects exactly the lanes the unmetered plan does, and differs from the control
    /// in where the scatter lands and in the observer path only:
    ///
    /// * meters reading the resident view (the production shape; no planar acquisition at all);
    /// * observers that decline the resident view and read the member's output -- after the
    ///   redirect, the consumer's buffer, read after the chain's unit and before the fader's;
    /// * the test-only switch that withdraws the resident offer;
    /// * both of the above through the controlled-activation dispatcher (`observe_one`).
    ///
    /// Each track carries two meters: one on the last slot's own node and one on its later tap
    /// (the elided `PostSimd1`), so both kinds of producer observer the two removed clauses
    /// declined are read in every block. Consumers: the bound fader (no fold is possible) and the
    /// route (the fold declines for the non-post-matrix meters, and the oracle for "the lanes the
    /// unmetered plan redirects" is then the unmetered plan with the fold declined). Shapes: a full
    /// W4 bank, a full W8 bank, two full W4 cohorts, and a W4 strip of six whose second cohort is
    /// partial -- frame counts off the lane width, so the transposes' tails are exercised.
    ///
    /// Red mutations: scatter into the consumer but leave the member's `output` on the last slot's
    /// own buffer -- the declining arms publish other words and the master is unchanged; move the
    /// redirected member's observers onto the consumer's op, so they run after it -- every arm's
    /// frames change (its counts first); hand the resident view the wrong lane -- the resident
    /// arms (and the unredirected resident control) publish other words.
    #[test]
    fn a_redirected_metered_plan_is_the_unredirected_plans_master_and_meters_bit_for_bit() {
        for (width, tracks, frames) in [
            (BankWidth::Four, 4, 13),
            (BankWidth::Eight, 8, 13),
            (BankWidth::Four, 8, 16),
            (BankWidth::Four, 6, 5),
        ] {
            let scattered = FoldFixture::scattered(width, tracks, frames);
            for (consumer, metered) in [
                ("fader", scattered),
                (
                    "route",
                    FoldFixture {
                        fader: false,
                        ..scattered
                    },
                ),
            ] {
                let shape = format!("{width:?}/{tracks}/{consumer}");
                assert_redirected_meters_are_the_declined_plans(metered, None, &shape);
            }
        }
    }

    /// The fader node `test_only_selected_split_fader` reports for the bind just made, if any.
    fn selected_split_fader() -> Option<GraphNodeId> {
        test_only_selected_split_fader().map(|selected| selected.node)
    }

    /// Gate 2's comparison for one metered shape: the declined oracle, then the five redirected
    /// arms, each against it bit for bit. `control_split` is the fader the oracle's bind hands the
    /// scalar split pair; every redirected arm must hand it none.
    fn assert_redirected_meters_are_the_declined_plans(
        metered: FoldFixture,
        control_split: Option<GraphNodeId>,
        shape: &str,
    ) {
        const BLOCKS: u64 = 6;
        let (tracks, frames) = (metered.tracks, metered.frames);
        let run = |fixture: FoldFixture, resident_disabled: bool| {
            let published = Published::default();
            let (mut plan, _controller) = fold_fixture(fixture, &published);
            let bound = [plan.bank_scatter_redirects(), plan.bank_route_folds()];
            let split = selected_split_fader();
            test_only_meter_input_reset(resident_disabled);
            let master = render_fold_fixture(&mut plan, frames as usize, BLOCKS);
            let counts = test_only_meter_input_counts();
            test_only_meter_input_reset(false);
            let frames = published.lock().unwrap().clone();
            (bound, split, master, frames, counts)
        };
        let redirected = scatter_shape(FoldFixture {
            metered: false,
            fold_declined: true,
            ..metered
        })[0];
        assert!(redirected > 0, "{shape}: the shape must redirect something");
        let meters = (tracks * metered.meter_at.len()) as u64;
        let offers = meters * BLOCKS;
        let planar = tracks as u64 * BLOCKS;
        let (control_bound, split, control_master, control_frames, control_counts) = run(
            FoldFixture {
                scatter_declined: true,
                accepts_resident: false,
                ..metered
            },
            false,
        );
        assert_eq!(
            control_bound,
            [0, 0],
            "{shape}: the control neither redirects nor folds"
        );
        assert_eq!(split, control_split, "{shape}: the control's split pair");
        assert_eq!(
            control_counts,
            [planar, offers, 0],
            "{shape}: control counts"
        );
        assert_eq!(
            control_frames.len(),
            offers as usize,
            "{shape}: one frame per meter per block"
        );
        assert!(
            control_frames.iter().all(|frame| frame
                .left
                .iter()
                .chain(&frame.right)
                .any(|word| *word != 0)),
            "{shape}: every meter frame carries audio"
        );
        assert!(control_master.iter().any(|word| *word != 0));
        let (_, _, resident_master, resident_frames, resident_counts) = run(
            FoldFixture {
                scatter_declined: true,
                ..metered
            },
            false,
        );
        assert_eq!(resident_counts, [0, offers, offers]);
        assert_eq!(resident_master, control_master);
        assert_eq!(resident_frames, control_frames);
        for (name, fixture, resident_disabled, expected_counts) in [
            ("resident meters", metered, false, [0, offers, offers]),
            (
                "declining observers",
                FoldFixture {
                    accepts_resident: false,
                    ..metered
                },
                false,
                [planar, offers, 0],
            ),
            ("resident offer withdrawn", metered, true, [planar, 0, 0]),
            (
                "controlled resident meters",
                FoldFixture {
                    controlled: true,
                    ..metered
                },
                false,
                [0, offers, offers],
            ),
            (
                "controlled declining observers",
                FoldFixture {
                    controlled: true,
                    accepts_resident: false,
                    ..metered
                },
                false,
                [planar, offers, 0],
            ),
        ] {
            let (bound, split, master, frames, counts) = run(fixture, resident_disabled);
            assert_eq!(
                bound,
                [redirected, 0],
                "{shape}/{name}: the unmetered plan's redirects, and no fold"
            );
            assert_eq!(split, None, "{shape}/{name}: no split pair");
            assert_eq!(
                counts, expected_counts,
                "{shape}/{name}: [planar, offered, accepted]"
            );
            assert_eq!(
                master, control_master,
                "{shape}/{name}: the redirected master is the unredirected master's bits"
            );
            assert_eq!(
                frames, control_frames,
                "{shape}/{name}: every meter frame is the unredirected plan's"
            );
        }
    }

    /// A fader that is a redirect consumer is kept out of the scalar split fader/matrix pair
    /// (`build_sequential` skips a pair whose fader or matrix consumes a redirect). Before issue
    /// #886 a meter on the last slot declined the redirect and so freed the fader for the pair: a
    /// metered plan took the split pair where the same plan unmetered took the redirect. Now both
    /// take the redirect, and the metered plan has the unmetered plan's shape.
    ///
    /// The shape adds a bound scalar `PostMatrix` after each fader, and a fader owner that offers
    /// the split pair (a deferring owner: the fader's arithmetic runs at the matrix slot). Faders
    /// are consecutive in the schedule, so track 0's fader and matrix are not adjacent, which is
    /// the only interval the split pair admits. Asserted on the bound plan:
    ///
    /// * unmetered: every lane redirected, no split pair;
    /// * metered at the last slot and its later tap: the same -- the redirect, not the split pair;
    /// * the declined oracle (the path a fully metered plan took before #886): no redirect, and
    ///   track 0's fader is the selected split fader.
    ///
    /// Then gate 2's comparison: every redirected, metered arm renders the master and every meter
    /// frame bit for bit as the declined oracle, which renders through the split pair.
    ///
    /// Red mutation: drop the redirect-consumer exclusion from the split pass -- the metered and
    /// unmetered arms select a split fader.
    #[test]
    fn a_metered_redirect_consumer_stays_out_of_the_split_pair_and_keeps_the_bits() {
        for (width, tracks, frames) in [(BankWidth::Four, 4, 13), (BankWidth::Eight, 8, 13)] {
            let shape = format!("{width:?}/{tracks}/split pair");
            let metered = FoldFixture {
                split_pair: true,
                ..FoldFixture::scattered(width, tracks, frames)
            };
            let track_zero_fader = GraphNodeId::TrackStage {
                track_id: crate::StableGraphId::parse("track00").expect("stable id"),
                stage: TrackStage::PostFader,
            };
            let lanes = tracks as u64;
            let path = |fixture: FoldFixture| (scatter_shape(fixture), selected_split_fader());
            assert_eq!(
                path(FoldFixture {
                    metered: false,
                    ..metered
                }),
                ([lanes, 0], None),
                "{shape}: unmetered, every lane redirects and no fader is split"
            );
            assert_eq!(
                path(metered),
                ([lanes, 0], None),
                "{shape}: metered, the same shape as unmetered -- the redirect, not the split pair"
            );
            assert_eq!(
                path(FoldFixture {
                    scatter_declined: true,
                    ..metered
                }),
                ([0, 0], Some(track_zero_fader.clone())),
                "{shape}: with the redirect declined, track 0's fader takes the split pair"
            );
            assert_redirected_meters_are_the_declined_plans(
                metered,
                Some(track_zero_fader),
                &shape,
            );
        }
    }

    // -----------------------------------------------------------------------------------------
    // Issue #918: a banked track's source input is gathered from the played transfer block.
    // -----------------------------------------------------------------------------------------

    /// What one fake source block holds, by block index.
    #[derive(Clone, Copy, Debug)]
    enum PlayedBlock {
        /// A whole quantum.
        Full,
        /// This many played frames, then `+0.0` to the quantum: a short end-of-region block.
        Short(usize),
        /// Nothing was played: the whole quantum is the underrun.
        Underrun,
    }

    /// Issue #918's gate-1 script: eight blocks, one of them an underrun and one short.
    const PLAYED_SCRIPT: [PlayedBlock; 8] = [
        PlayedBlock::Full,
        PlayedBlock::Full,
        PlayedBlock::Full,
        PlayedBlock::Underrun,
        PlayedBlock::Full,
        PlayedBlock::Short(9),
        PlayedBlock::Full,
        PlayedBlock::Full,
    ];

    /// A fake source set with the production driver's contract: `begin_block` plays one block of
    /// seeded noise per channel (tail zeroed in place on a short block, nothing on an underrun),
    /// `copy_track_input` copies a claim's `(left, right)` channels exactly as `copy_channel` does,
    /// and the played planes stay readable until the next `begin_block`.
    struct PlayedSource {
        frames: usize,
        /// Per claim, the `(left, right)` source channels. A mono claim names one channel twice.
        mapping: Vec<[usize; 2]>,
        /// `channels * frames` words: this block's played planes.
        planes: Vec<f32>,
        played: bool,
    }

    impl PlayedSource {
        fn new(frames: u32, mapping: Vec<[usize; 2]>) -> Self {
            let channels = mapping.iter().flatten().max().map_or(0, |max| max + 1);
            Self {
                frames: frames as usize,
                mapping,
                planes: vec![0.0; channels * frames as usize],
                played: false,
            }
        }

        fn channel(&self, channel: usize) -> &[f32] {
            &self.planes[channel * self.frames..(channel + 1) * self.frames]
        }
    }

    impl crate::GraphPreparedSourceSetDriver for PlayedSource {
        fn claim_count(&self) -> usize {
            self.mapping.len()
        }

        fn begin_block(&mut self, first_sample: u64, frames: u32) -> Result<(), RenderError> {
            let block = (first_sample / u64::from(frames)) as usize;
            let played = match PLAYED_SCRIPT[block % PLAYED_SCRIPT.len()] {
                PlayedBlock::Full => Some(self.frames),
                PlayedBlock::Short(played) => Some(played),
                PlayedBlock::Underrun => None,
            };
            self.played = played.is_some();
            if let Some(played) = played {
                for (channel, plane) in self.planes.chunks_exact_mut(self.frames).enumerate() {
                    let mut state = (channel as u32 + 1).wrapping_mul(0x9e37_79b9)
                        ^ (first_sample as u32).wrapping_mul(0x85eb_ca6b);
                    for word in &mut plane[..played] {
                        *word = lcg(&mut state);
                    }
                    // A negative zero survives only a bit-exact path.
                    plane[0] = -0.0;
                    plane[played..].fill(0.0);
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
            let [left_channel, right_channel] = self.mapping[claim];
            if self.played {
                left.copy_from_slice(self.channel(left_channel));
                right.copy_from_slice(self.channel(right_channel));
            } else {
                left.fill(0.0);
                right.fill(0.0);
            }
            Ok(())
        }

        fn provides_played_planes(&self) -> bool {
            true
        }

        fn played_planes(&self, claim: usize) -> Option<(&[f32], &[f32])> {
            let [left_channel, right_channel] = *self.mapping.get(claim)?;
            self.played
                .then(|| (self.channel(left_channel), self.channel(right_channel)))
        }
    }

    /// One issue #918 gate-1 plan: `Input -> <stages> -> Route -> Output` per track, each stage a
    /// builtin bank per cohort, every input claimed by a [`PlayedSource`].
    ///
    /// With the default single `PostInputBuiltins` stage, which is dedicated storage, each member's
    /// reduction is the dedication copy and its gather reads the input's buffer (the route takes the
    /// input's retired slot, so the scatter redirect repoints the member's output at it and the
    /// member's reduction becomes the `[own output]` no-op). The options each give one track a
    /// reason to keep the copy, or reach another way a gather reads an input.
    #[derive(Clone, Copy, Debug)]
    struct SourceShape {
        width: BankWidth,
        tracks: usize,
        frames: u32,
        /// The bank stages each banked track runs, in order.
        stages: &'static [TrackStage],
        /// The stages every banked track is metered at.
        meters: &'static [TrackStage],
        /// A bound scalar stage ([`ScalarTilt`]) each banked track runs between its bank stages,
        /// in stage order.
        scalar: Option<TrackStage>,
        /// Bind with every scatter redirect declined, so each member keeps its own buffer and its
        /// gather reads the input through `bank_gather_source`.
        redirect_declined: bool,
        /// This track's input is delayed in place (`NodeKind::TrackDelay`) before its bank reads it.
        delayed: Option<usize>,
        /// This track's input stage is metered.
        observed: Option<usize>,
        /// This track's edge from its input into its first bank stage carries a compensation
        /// delay, which the member stages from the input's buffer before it reduces.
        compensated: Option<usize>,
        /// This track has no builtin bank: its route reads the input in place.
        routed: Option<usize>,
    }

    impl SourceShape {
        const fn banked(width: BankWidth, tracks: usize, frames: u32) -> Self {
            Self {
                width,
                tracks,
                frames,
                stages: &[TrackStage::PostInputBuiltins],
                meters: &[TrackStage::PostInputBuiltins],
                scalar: None,
                redirect_declined: false,
                delayed: None,
                observed: None,
                compensated: None,
                routed: None,
            }
        }

        /// `Input -> PostInputBuiltins (bank) -> PostFader (bound scalar) -> PostMatrix (bank)`:
        /// the fader takes the input's retired slot, and the `PostMatrix` bank runs in place over
        /// it, so a later gather reads each input's slot for another value.
        const fn scalar_fader(
            width: BankWidth,
            tracks: usize,
            meters: &'static [TrackStage],
        ) -> Self {
            Self {
                stages: &[TrackStage::PostInputBuiltins, TrackStage::PostMatrix],
                scalar: Some(TrackStage::PostFader),
                meters,
                ..Self::banked(width, tracks, 13)
            }
        }

        /// Every third track reads one source channel on both sides; the rest read two.
        fn mapping(self) -> Vec<[usize; 2]> {
            (0..self.tracks)
                .map(|track| {
                    if track % 3 == 2 {
                        [2 * track, 2 * track]
                    } else {
                        [2 * track, 2 * track + 1]
                    }
                })
                .collect()
        }
    }

    const INPUT_METER_HANDLE: u64 = 500;

    /// [`SourceShape`]'s unbound plan, bindings and source set. Meters: one per banked track at
    /// each of `meters`, one on the observed input, and two on the Output.
    fn source_fixture_parts(
        shape: SourceShape,
        published: &Published,
    ) -> (
        crate::PreparedGraphPlan,
        crate::GraphRuntimeBindings,
        crate::GraphPreparedSourceSet,
    ) {
        let id = |text: String| crate::StableGraphId::parse(&text).expect("stable id");
        let stage_node = |track: usize, stage| GraphNodeId::TrackStage {
            track_id: id(format!("track{track:02}")),
            stage,
        };
        let banked: Vec<usize> = (0..shape.tracks)
            .filter(|track| shape.routed != Some(*track))
            .collect();
        let inputs: Vec<_> = (0..shape.tracks)
            .map(|track| stage_node(track, TrackStage::Input))
            .collect();
        // Every banked track's stages in order, and one level per stage: that stage's node of
        // every banked track.
        let mut chain: Vec<TrackStage> = shape.stages.iter().copied().chain(shape.scalar).collect();
        chain.sort_unstable();
        let stages: Vec<Vec<_>> = chain
            .iter()
            .map(|stage| {
                banked
                    .iter()
                    .map(|track| stage_node(*track, *stage))
                    .collect()
            })
            .collect();
        let scalars: Vec<_> = stages
            .iter()
            .zip(&chain)
            .filter(|(_, stage)| shape.scalar == Some(**stage))
            .flat_map(|(nodes, _)| nodes.iter().cloned())
            .collect();
        let routes: Vec<_> = (0..shape.tracks)
            .map(|track| GraphNodeId::Route {
                route_id: id(format!("route{track:02}")),
            })
            .collect();
        let output = GraphNodeId::Output {
            output_id: id("main".to_owned()),
        };
        let port = |node: &GraphNodeId, kind| crate::GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        let edge = |id, source: &GraphNodeId, destination: &GraphNodeId| crate::GraphEdge {
            id,
            source: port(source, crate::GraphPortKind::MainOutput),
            destination: port(destination, crate::GraphPortKind::MainInput),
            path: "$.issue918".to_owned(),
        };
        let mut edges = Vec::new();
        for track in 0..shape.tracks {
            let route_id = id(format!("route{track:02}"));
            let mut upstream = inputs[track].clone();
            if shape.routed != Some(track) {
                for stage in &chain {
                    let member = stage_node(track, *stage);
                    edges.push(edge(
                        GraphEdgeId::TrackMain {
                            target: member.clone(),
                        },
                        &upstream,
                        &member,
                    ));
                    upstream = member;
                }
            }
            edges.push(edge(
                GraphEdgeId::RouteSource {
                    route_id: route_id.clone(),
                },
                &upstream,
                &routes[track],
            ));
            edges.push(edge(
                GraphEdgeId::RouteDestination { route_id },
                &routes[track],
                &output,
            ));
        }
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let outputs = vec![output.clone()];
        let levels: Vec<&Vec<GraphNodeId>> = core::iter::once(&inputs)
            .chain(&stages)
            .chain([&routes, &outputs])
            .filter(|level| !level.is_empty())
            .collect();
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.iter().cloned())
            .collect();
        let mut nodes: Vec<_> = schedule
            .iter()
            .cloned()
            .map(|id| crate::GraphNode {
                id,
                latency: effect_contract::LatencySamples(0),
                tail: effect_contract::TailSamples::Finite(0),
            })
            .collect();
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        let envelope = engine::realtime::RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: engine::QuantumFrames(shape.frames),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let backend = match shape.width {
            BankWidth::Four => lane::Backend::Simd4,
            BankWidth::Eight => lane::Backend::Simd8,
        };
        let builtin_banks = stages
            .iter()
            .zip(&chain)
            .filter(|(_, stage)| shape.scalar != Some(**stage))
            .flat_map(|(members, _)| members.chunks(shape.width.lanes() as usize))
            .map(|cohort| GraphPreparedBuiltinBank {
                backend,
                members: cohort.to_vec().into_boxed_slice(),
                processor: Box::new(CrossTilt),
                scratch: AoSoaScratch::new(shape.width, shape.frames).expect("scratch"),
            })
            .collect();
        let mut required_bindings = inputs.clone();
        required_bindings.extend(stages.iter().flatten().cloned());
        required_bindings.push(output.clone());
        let plan = crate::PreparedGraphPlan::new(crate::PreparedGraphPlanParts {
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
                .map(|(level, nodes)| crate::DependencyLevel {
                    level: level as u64,
                    nodes: (*nodes).clone(),
                })
                .collect(),
            route_timings: Vec::new(),
            inserted_delays: shape
                .compensated
                .map(|track| {
                    let edge_id = GraphEdgeId::TrackMain {
                        target: stage_node(track, chain[0]),
                    };
                    crate::InsertedDelay {
                        node: GraphNodeId::CompensationDelay {
                            edge_id: Box::new(edge_id.clone()),
                        },
                        edge_id,
                        samples: effect_contract::LatencySamples(4),
                    }
                })
                .into_iter()
                .collect(),
            buffer_assignments: Vec::new(),
            estimate: crate::GraphResourceEstimate {
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
            },
            envelope,
            required_bindings,
            routes: routes
                .iter()
                .enumerate()
                .map(|(track, node)| crate::PreparedRoute {
                    node: node.clone(),
                    transform: RouteTransform {
                        gain: 0.5 + 0.0625 * track as f32,
                        ll: 0.875,
                        lr: -0.25 + 0.03125 * track as f32,
                        rl: 0.3,
                        rr: 1.125 - 0.046875 * track as f32,
                    },
                })
                .collect(),
            track_delays: shape
                .delayed
                .map(|track| crate::PreparedTrackDelay {
                    node: inputs[track].clone(),
                    left_samples: 3,
                    right_samples: 5,
                })
                .into_iter()
                .collect(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks,
            observers: Vec::new(),
        });
        let meter = |node: GraphNodeId, handle: u64| {
            GraphNodeObserverBinding::new(
                node,
                handle,
                Box::new(WordMeter {
                    handle,
                    accepts_resident: false,
                    published: Arc::clone(published),
                }),
            )
        };
        let mut observers = Vec::new();
        for (index, stage) in shape.meters.iter().enumerate() {
            for track in &banked {
                let handle = (index * shape.tracks + track) as u64 + 1;
                observers.push(meter(stage_node(*track, *stage), handle));
            }
        }
        if let Some(track) = shape.observed {
            observers.push(meter(
                inputs[track].clone(),
                INPUT_METER_HANDLE + track as u64,
            ));
        }
        for handle in [OUTPUT_METER_HANDLE, OUTPUT_METER_HANDLE + 1] {
            observers.push(meter(output.clone(), handle));
        }
        let source_set = crate::GraphPreparedSourceSet::new(
            envelope,
            inputs
                .iter()
                .map(|node| crate::GraphSourceInputClaim { node: node.clone() })
                .collect(),
            crate::GraphSourceSetResourceReport {
                pcm_payload_already_charged_bytes: 0,
                overhead_bytes: 0,
                total_engine_owned_bytes: 0,
                largest_allocation_bytes: 0,
            },
            Box::new(PlayedSource::new(shape.frames, shape.mapping())),
        );
        (
            plan,
            crate::GraphRuntimeBindings {
                envelope,
                nodes: scalars
                    .into_iter()
                    .map(|node| GraphNodeBinding::new(node, Box::new(ScalarTilt)))
                    .chain([GraphNodeBinding::identity(output)])
                    .collect(),
                observers,
            },
            source_set,
        )
    }

    /// Words no source path ever writes: the arena slot of every claim holds these before a block.
    const SOURCE_SLOT_POISON: [u32; 2] = [0x7fc1_0918, 0x7fc2_0918];

    /// What one issue #918 arm rendered: every block's master bits, and every published meter
    /// frame; and how it was bound: each claim's mode (`true` in place), the source-plane counts
    /// over the render, and `[route folds, scatter redirects]`.
    struct SourceRun {
        masters: Vec<Vec<u32>>,
        meters: Vec<MeterFrame>,
        in_place: Vec<bool>,
        counts: [u64; 3],
        shape: [u64; 2],
    }

    impl SourceRun {
        /// FNV-1a over every master word and every meter frame, in render order.
        fn digest(&self) -> u64 {
            let mut hash = 0xcbf2_9ce4_8422_2325_u64;
            let mut word = |value: u64| {
                for byte in value.to_le_bytes() {
                    hash ^= u64::from(byte);
                    hash = hash.wrapping_mul(0x0100_0000_01b3);
                }
            };
            for master in &self.masters {
                master.iter().for_each(|bits| word(u64::from(*bits)));
            }
            for frame in &self.meters {
                word(frame.handle);
                word(frame.first_sample);
                frame
                    .left
                    .iter()
                    .chain(&frame.right)
                    .chain(&frame.peak)
                    .chain(&frame.energy)
                    .for_each(|bits| word(u64::from(*bits)));
            }
            hash
        }
    }

    /// The arena slot of every track's input, in claim order.
    fn source_slots(plan: &crate::PreparedGraphPlan, tracks: usize) -> Vec<u32> {
        let program = plan.lowered().expect("lowered");
        (0..tracks)
            .map(|track| {
                let node = GraphNodeId::TrackStage {
                    track_id: crate::StableGraphId::parse(&format!("track{track:02}"))
                        .expect("stable id"),
                    stage: TrackStage::Input,
                };
                let index = crate::program::node_index(&plan.spec, &node).expect("input node");
                program.node_buffer[index as usize].0 + ARENA_BASE
            })
            .collect()
    }

    /// Bind `shape` -- in place, or with `declined` every claim on the copy -- and render
    /// [`PLAYED_SCRIPT`], poisoning every claim's arena slot before each block so that a read of
    /// a slot the copy no longer writes shows in the bits.
    fn render_source_shape(shape: SourceShape, declined: bool) -> SourceRun {
        let published = Published::default();
        let (plan, bindings, source_set) = source_fixture_parts(shape, &published);
        let slots = source_slots(&plan, shape.tracks);
        test_only_set_source_in_place_declined(declined);
        test_only_set_scatter_redirect_declined(shape.redirect_declined);
        let (mut executor, _, _) = bind_executor(plan, bindings, false, Some(source_set));
        test_only_set_source_in_place_declined(false);
        test_only_set_scatter_redirect_declined(false);
        let in_place = slots
            .iter()
            .enumerate()
            .map(|(claim, slot)| executor.runtime.source_in_place(claim, *slot))
            .collect();
        let frames = shape.frames as usize;
        let mut masters = Vec::new();
        test_only_source_plane_reset();
        for block in 0..PLAYED_SCRIPT.len() as u64 {
            for slot in &slots {
                let (left, right) = executor.runtime.buffer_mut(*slot);
                left.fill(f32::from_bits(SOURCE_SLOT_POISON[0]));
                right.fill(f32::from_bits(SOURCE_SLOT_POISON[1]));
            }
            let mut storage = vec![f32::from_bits(HOST_PAD); 2 * frames];
            render_host(
                &mut executor,
                engine::realtime::PlanarBufferMut::try_new(&mut storage, 2, frames, frames)
                    .expect("host output"),
                block * u64::from(shape.frames),
            )
            .expect("render");
            masters.push(storage.iter().map(|word| word.to_bits()).collect());
        }
        let counts = test_only_source_plane_counts();
        let meters = published.lock().unwrap().clone();
        SourceRun {
            masters,
            meters,
            in_place,
            counts,
            shape: [
                executor.runtime.route_folds(),
                executor.runtime.scatter_redirects(),
            ],
        }
    }

    /// Issue #918 gate 1: the copy arm's digest ([`SourceRun::digest`]) of each shape, recorded by
    /// this fixture on the executor as it stood before the issue (`63eeebf0`, where every claim was
    /// copied), and the claims each shape binds on the copy.
    const SOURCE_SHAPES: [(SourceShape, u64, &[usize]); 9] = [
        (
            SourceShape::banked(BankWidth::Eight, 8, 13),
            0x7da8_2488_c5b7_8876,
            &[],
        ),
        (
            SourceShape::banked(BankWidth::Four, 4, 16),
            0xc9da_ced7_80fa_e77f,
            &[],
        ),
        (
            SourceShape::banked(BankWidth::Four, 6, 13),
            0xf317_5c3f_c88e_6167,
            &[],
        ),
        (
            SourceShape {
                delayed: Some(1),
                observed: Some(2),
                routed: Some(4),
                ..SourceShape::banked(BankWidth::Four, 6, 13)
            },
            0x25f5_d764_10a6_b66a,
            &[1, 2, 4],
        ),
        (
            SourceShape {
                redirect_declined: true,
                ..SourceShape::banked(BankWidth::Eight, 8, 13)
            },
            0x7da8_2488_c5b7_8876,
            &[],
        ),
        (
            SourceShape {
                stages: &[TrackStage::PostMatrix],
                meters: &[TrackStage::PostMatrix],
                ..SourceShape::banked(BankWidth::Four, 6, 13)
            },
            0xf317_5c3f_c88e_6167,
            &[],
        ),
        (
            SourceShape::scalar_fader(BankWidth::Four, 6, &[TrackStage::PostMatrix]),
            0x6c18_6a5b_3585_e2a6,
            &[],
        ),
        (
            SourceShape::scalar_fader(BankWidth::Eight, 8, &[]),
            0x0be8_59e1_1a6e_5267,
            &[],
        ),
        (
            SourceShape {
                compensated: Some(3),
                ..SourceShape::banked(BankWidth::Four, 6, 13)
            },
            0xb0c7_a1d1_e6bd_959f,
            &[3],
        ),
    ];

    /// Gate 1 of issue #918: a banked track's gather reads the played block in place of the
    /// executor's ring-to-arena copy, and moves no rendered bit.
    ///
    /// Every shape renders [`PLAYED_SCRIPT`] (eight blocks: an underrun, a short block with a zeroed
    /// tail, and six whole ones) with every third claim a mono mapping (`left == right`). The
    /// shapes, in [`SOURCE_SHAPES`] order:
    ///
    /// 1. a full `W8 x 8` bank; 2. a full `W4 x 4` bank over a whole number of tiles; 3. a `W4 x 6`
    ///    plan whose second cohort is a partial bank, gathered lane by lane rather than tiled. In
    ///    all three the route takes the input's retired slot and the scatter redirect repoints each
    ///    member there, so the member gathers its input through its own output.
    /// 4. The `W4 x 6` plan with a delayed track, a metered input, and a track routed straight from
    ///    its input: the three claims that keep the copy.
    /// 5. The `W8 x 8` plan with the redirects declined: each member keeps its own buffer and gathers
    ///    through `bank_gather_source`, the dedication copy the brief names.
    /// 6. `W4 x 6` with the members at `PostMatrix`, which is not dedicated: each runs in place over
    ///    its input, and its route folds into the Output.
    /// 7. and 8. A bound fader between two bank stages ([`SourceShape::scalar_fader`]) takes each
    ///    input's retired slot, so the second bank gathers that slot for the fader's value. Only
    ///    the lanes marked at bind may be served a played block.
    /// 9. The `W4 x 6` plan with a compensation delay on one track's edge into its bank: that
    ///    member stages the input's buffer through its delay line, so the claim keeps the copy.
    ///
    /// Per shape:
    ///
    /// * **The mode table.** In place: every claim whose input only a bank gathers. On the copy:
    ///   the delayed claim (its delay line writes the arena buffer), the metered one (its meter
    ///   reads it), the routed one (its route reads it in place) and the compensated one (its
    ///   member's staging reads it). The declined arm binds every claim on the copy.
    /// * **The copy arm is the pre-change executor.** Its digest over every master word and every
    ///   meter frame is the one [`SOURCE_SHAPES`] recorded before the issue.
    /// * **The in-place arm is the copy arm, bit for bit**: every block's master, every meter window
    ///   (each bank stage's and both Output meters'), and the bound shape, with each claim's arena
    ///   slot poisoned before every block, so a gather that read the slot the copy no longer fills
    ///   would show.
    /// * **The mode counter.** The copy arm copies every claim every block and serves no gather
    ///   from a played block. The in-place arm copies only its copy claims, and serves each
    ///   in-place claim's gather from the played block on the seven played blocks and from the
    ///   silence buffer on the underrun. Bit identity alone cannot tell a kept copy from a skipped
    ///   one; this is what does.
    ///
    /// Red mutations: `crates/graph/tests/MUTATIONS.md`, issue #918.
    #[test]
    fn a_banked_source_gathers_the_played_block_bit_for_bit_with_the_copy() {
        let blocks = PLAYED_SCRIPT.len() as u64;
        let underruns = PLAYED_SCRIPT
            .iter()
            .filter(|block| matches!(block, PlayedBlock::Underrun))
            .count() as u64;
        for (shape, pre_change, copied) in SOURCE_SHAPES {
            let in_place = render_source_shape(shape, false);
            let copy = render_source_shape(shape, true);
            let expected: Vec<bool> = (0..shape.tracks)
                .map(|track| !copied.contains(&track))
                .collect();
            assert_eq!(in_place.in_place, expected, "{shape:?}: the mode table");
            assert_eq!(
                copy.in_place,
                vec![false; shape.tracks],
                "{shape:?}: declined, every claim is copied"
            );
            assert_eq!(
                copy.digest(),
                pre_change,
                "{shape:?}: the copy arm is the pre-change executor"
            );
            assert_eq!(in_place.shape, copy.shape, "{shape:?}: one bound shape");
            assert_eq!(in_place.masters.len(), copy.masters.len());
            for (block, (actual, expected)) in
                in_place.masters.iter().zip(&copy.masters).enumerate()
            {
                assert_eq!(
                    actual, expected,
                    "{shape:?}, block {block} ({:?}): the master is the copy arm's",
                    PLAYED_SCRIPT[block]
                );
                assert!(
                    matches!(PLAYED_SCRIPT[block], PlayedBlock::Underrun)
                        || actual.iter().any(|word| *word != 0),
                    "{shape:?}, block {block}: a played block carries audio"
                );
            }
            assert_eq!(
                in_place.meters, copy.meters,
                "{shape:?}: every meter window is the copy arm's"
            );
            assert_eq!(
                in_place.meters.len(),
                (shape.meters.len() * (shape.tracks - usize::from(shape.routed.is_some()))
                    + usize::from(shape.observed.is_some())
                    + 2)
                    * PLAYED_SCRIPT.len(),
                "{shape:?}: every meter published every block"
            );
            let lent = expected.iter().filter(|lent| **lent).count() as u64;
            let tracks = shape.tracks as u64;
            assert_eq!(
                copy.counts,
                [tracks * blocks, 0, 0],
                "{shape:?}: the copy arm's [copies, played gathers, silent gathers]"
            );
            assert_eq!(
                in_place.counts,
                [
                    (tracks - lent) * blocks,
                    lent * (blocks - underruns),
                    lent * underruns
                ],
                "{shape:?}: the in-place arm's [copies, played gathers, silent gathers]"
            );
        }
    }

    // Issue #926: in-place routes fused into the session Output op's reduction, in pairs.
    // -----------------------------------------------------------------------------------------

    /// One `route_reduce` case against the two production ops it replaces, at one width.
    ///
    /// The oracle is the production path itself: each route op is `execute_op`'s `Route` arm --
    /// the in-place `mix2x2_block::<FrameLane>` over a buffer holding the route's input -- and the
    /// reduction is `reduce_plane_into` over those mixed buffers, which #916's kernel test pins to
    /// the arena reduction. The candidate reads the unmixed buffers and the route table.
    fn assert_route_reduce_is_the_route_ops_and_the_reduction<L: Lane>() {
        let bits = |words: &[f32]| words.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
        let mut state = 0x0920_u64 ^ L::WIDTH as u64;
        for frames in [1, 3, 7, 8, 13, 16, 33, 64] {
            for fan_in in (2..=19_usize).chain([64]) {
                for negative_zero in [false, true] {
                    // Buffer 0 is the silence buffer and buffer 1 the op's own (unused) output;
                    // then the routes' inputs, then the buffers the route ops mix in place.
                    let mut lease = stereo_lease(frames, 2 + 2 * fan_in);
                    let inputs: Vec<u32> = (0..fan_in).map(|index| 2 + index as u32).collect();
                    let mixed: Vec<u32> =
                        inputs.iter().map(|input| input + fan_in as u32).collect();
                    // A signed-zero case: every word `-0.0` and every coefficient positive, so
                    // every mix is `-0.0` and so is the master. A sum seeded with `+0.0` loses
                    // the sign; the route ops and the reduction keep it.
                    let routes: Vec<[f32; 4]> = (0..fan_in)
                        .map(|_| {
                            core::array::from_fn(|_| {
                                let constant = hostile_constant(&mut state);
                                if negative_zero {
                                    constant.abs() + 0.5
                                } else {
                                    constant
                                }
                            })
                        })
                        .collect();
                    for &input in &inputs {
                        for plane in 0..2 {
                            for word in lease.write(plane, input) {
                                *word = if negative_zero {
                                    -0.0
                                } else {
                                    hostile_sample(&mut state)
                                };
                            }
                        }
                    }
                    for ((&input, &route), coefficients) in inputs.iter().zip(&mixed).zip(&routes) {
                        for plane in 0..2 {
                            let (route_plane, input_plane) = lease.write_read(plane, route, input);
                            route_plane.copy_from_slice(input_plane);
                        }
                        let (left, right) = lease.write_stereo(route);
                        mix2x2_block::<FrameLane>(left, right, *coefficients);
                    }
                    let pad = f32::from_bits(HOST_PAD);
                    let mut expected = [vec![pad; frames], vec![pad; frames]];
                    for (plane, target) in expected.iter_mut().enumerate() {
                        reduce_plane_into(&lease, plane, 1, target, &mixed);
                    }
                    let (mut left, mut right) = (vec![pad; frames], vec![pad; frames]);
                    assert!(
                        route_reduce::<L>(
                            &lease,
                            &inputs,
                            &routes,
                            OutputSources::NONE,
                            &mut left,
                            &mut right
                        ),
                        "{frames} frames, fan-in {fan_in}: an admitted shape reduces"
                    );
                    let case = format!(
                        "width {}, {frames} frames, fan-in {fan_in}, -0.0 {negative_zero}",
                        L::WIDTH
                    );
                    assert_eq!(bits(&left), bits(&expected[0]), "{case}: left");
                    assert_eq!(bits(&right), bits(&expected[1]), "{case}: right");
                    if negative_zero {
                        assert!(
                            left.iter()
                                .chain(&right)
                                .all(|word| word.to_bits() == 0x8000_0000),
                            "{case}: every mix is -0.0, so the master keeps the sign"
                        );
                    }
                }
            }
        }
        // Refused before any write: a table that does not match the inputs, and fan-in one.
        let lease = stereo_lease(5, 4);
        let pad = f32::from_bits(HOST_PAD);
        for (inputs, routes) in [
            (&[2_u32, 3][..], &[[1.0_f32; 4]][..]),
            (&[2, 3][..], &[[1.0; 4]; 3][..]),
            (&[2][..], &[[1.0; 4]][..]),
        ] {
            let (mut left, mut right) = (vec![pad; 5], vec![pad; 5]);
            assert!(!route_reduce::<L>(
                &lease,
                inputs,
                routes,
                OutputSources::NONE,
                &mut left,
                &mut right
            ));
            assert!(
                left.iter()
                    .chain(&right)
                    .all(|word| word.to_bits() == HOST_PAD),
                "a refused shape writes nothing"
            );
        }
    }

    /// Issue #926's kernel: the fused reduction is each route op's `mix2x2_block` then the
    /// Output's reduction, bit for bit, at every lane width.
    ///
    /// Hostile words (signed zeros, subnormals, magnitudes over `2^-24 .. 2^25`) and a hostile 2x2
    /// per route; `frames` with and without ragged tails at both widths, so the outlined
    /// `route_tail` runs for every pair shape; fan-in two to nineteen (one to nine pairs, and every
    /// odd fan-in's lone last input) and sixty-four (thirty-two pairs, the plumbing row's); and a
    /// signed-zero case whose master must stay `-0.0`. The oracle reduces in groups of eight and
    /// the candidate in pairs, so every fan-in above two also checks that moving the store/reload
    /// boundary moves no bit. The width is free because `Lane::fma` is two roundings on every
    /// backend, and the oracle's route ops run at `FrameLane` whatever the candidate's width.
    ///
    /// Red mutations (`crates/graph/tests/MUTATIONS.md`, issue #926): reverse the accumulation,
    /// seed the first pair from `+0.0`, mix the running sum instead of the input, swap the
    /// coefficient roles, store in every pair, skip an odd fan-in's lone last input.
    #[test]
    fn a_route_reduction_is_the_route_ops_and_the_reduction_bit_for_bit() {
        assert_route_reduce_is_the_route_ops_and_the_reduction::<f32>();
        assert_route_reduce_is_the_route_ops_and_the_reduction::<lane::Simd4>();
        assert_route_reduce_is_the_route_ops_and_the_reduction::<lane::Simd8>();
    }

    /// Issue #926: `route_tail` takes a tail and nothing longer. A run as long as the widest lane
    /// is refused before any write, and one frame shorter runs. The bound is what keeps the
    /// compiler from vectorising the outlined tail (see `route_reduce`, "Why the tail is
    /// outlined"), so the refusal is pinned here rather than left to the artifact gate alone.
    #[test]
    fn a_route_tail_refuses_a_run_as_long_as_the_widest_lane() {
        let widest = <lane::Simd8 as Lane>::WIDTH;
        let pad = f32::from_bits(HOST_PAD);
        let plane = vec![0.5_f32; widest];
        let identity = [1.0_f32, 0.0, 0.0, 1.0];
        for (frames, runs) in [(widest, false), (widest - 1, true)] {
            let input = &plane[..frames];
            for pair in [&[input, input][..], &[input][..]] {
                let (mut left, mut right) = (vec![pad; frames], vec![pad; frames]);
                assert_eq!(
                    route_tail(
                        &mut left,
                        &mut right,
                        pair,
                        pair,
                        &[identity; 2][..pair.len()],
                        true
                    ),
                    runs,
                    "{frames} frames, {} inputs",
                    pair.len()
                );
                let expected = if runs {
                    (0.5 * pair.len() as f32).to_bits()
                } else {
                    HOST_PAD
                };
                assert!(
                    left.iter()
                        .chain(&right)
                        .all(|word| word.to_bits() == expected),
                    "{frames} frames, {} inputs: a refused run writes nothing",
                    pair.len()
                );
            }
        }
    }

    /// Per track and per block: hostile words ([`hostile_sample`]), or one signed zero per plane
    /// throughout.
    struct HostileInput {
        seed: u64,
        zeros: Option<[f32; 2]>,
    }

    impl GraphRuntimeProcessor for HostileInput {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            if let Some([left, right]) = self.zeros {
                block.left.fill(left);
                block.right.fill(right);
                return Ok(());
            }
            let mut state = self.seed ^ block.first_sample.wrapping_mul(0x9e37_79b9_7f4a_7c15);
            for word in block.left.iter_mut().chain(block.right.iter_mut()) {
                *word = hostile_sample(&mut state);
            }
            Ok(())
        }
    }

    /// Which issue #926 plan to bind: `fan_in` tracks, each `Input -> Route -> Output`, with the
    /// shape's one change.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum RoutedShape {
        /// Every route plain, in place and read by the Output alone: the fold is admitted.
        Plain,
        /// Every input `-0.0` and every route coefficient positive: admitted, and the master is
        /// `-0.0` throughout.
        NegativeZero,
        /// A meter on every track's `Input` (each route's producer) and two on the Output, bound
        /// directly, or through the activation catalog when `true`. Admitted: none of them reads a
        /// route's output.
        Metered(bool),
        /// Track 0's route feeds an elided `PostSimd2PreFader` boundary that feeds the Output. The
        /// boundary's `program::Tap` aliases the route's buffer after the route, and a meter is
        /// bound to it, so it reads the route's output. Declined.
        ObservedRouteAlias,
        /// Track 1's route-to-Output edge carries a three-sample compensation delay. Declined.
        ///
        /// Over signed-zero data, with track 1's 2x2 negative: the delay line's warm-up words are
        /// `+0.0`, so the delayed route contributes `+0.0` there where its mix of them would be
        /// `-0.0`. That is the one place a fused delayed route differs, and this data makes it
        /// visible in the master.
        DelayedEdge,
        /// One more Output contributor: a submix (an identity op) fed straight by an extra track's
        /// `Input`. Declined: that input's producer is not a route.
        ///
        /// Over signed-zero data, with the submix's input `(-0.0, +0.0)`. Mixing a pass-through
        /// through an identity 2x2 is not a no-op: `0 * r` is `+0.0`, so its left plane would
        /// become `+0.0` and so would the master's.
        SubmixContributor,
        /// The last track's route feeds a submix that feeds the Output. Declined: the Output's
        /// producer is the submix.
        ///
        /// Over signed-zero data, with that route's 2x2 mixing `-0.0` to `(-0.0, +0.0)`, for the
        /// reason [`RoutedShape::SubmixContributor`] gives.
        RouteIntoSubmix,
        /// One more route, from track 0's `Input` to the Output. Track 0's input has two readers,
        /// so neither of its routes runs in place. Declined: a route that copies lets its input's
        /// colour die at the route, and its own output buffer is what the Output reads.
        SharedInput,
        /// Track 0's route also feeds track 0's `PostFader`, scheduled after the Output and
        /// metered. Declined: the route's output has a second reader, which would read the
        /// unmixed words.
        LateReader,
    }

    impl RoutedShape {
        /// Whether the Output route fold is admitted for this shape.
        const fn folds(self) -> bool {
            matches!(self, Self::Plain | Self::NegativeZero | Self::Metered(_))
        }

        /// Whether every input word is a signed zero rather than hostile.
        const fn negative_zero(self) -> bool {
            matches!(
                self,
                Self::NegativeZero
                    | Self::DelayedEdge
                    | Self::SubmixContributor
                    | Self::RouteIntoSubmix
            )
        }
    }

    /// The unbound plan and bindings of one [`RoutedShape`]. Every route has its own hostile 2x2
    /// (or a positive one for [`RoutedShape::NegativeZero`]), so the reduction's order is visible
    /// in its bits. Observers publish into `published`.
    fn routed_output_parts(
        shape: RoutedShape,
        fan_in: usize,
        frames: u32,
        published: &Published,
    ) -> (crate::PreparedGraphPlan, crate::GraphRuntimeBindings) {
        let id = |text: String| crate::StableGraphId::parse(&text).expect("stable id");
        let stage = |track: usize, stage| GraphNodeId::TrackStage {
            track_id: id(format!("track{track:02}")),
            stage,
        };
        let route_id = |track: usize| id(format!("route{track:02}"));
        let inputs: Vec<_> = (0..fan_in)
            .map(|track| stage(track, TrackStage::Input))
            .collect();
        let mut routes: Vec<_> = (0..fan_in)
            .map(|track| GraphNodeId::Route {
                route_id: route_id(track),
            })
            .collect();
        let output = GraphNodeId::Output {
            output_id: id("main".to_owned()),
        };
        let submix = GraphNodeId::Submix {
            submix_id: id("bus".to_owned()),
        };
        let alias = stage(0, TrackStage::PostSimd2PreFader);
        let extra = stage(fan_in, TrackStage::Input);
        let late = stage(0, TrackStage::PostFader);
        let port = |node: &GraphNodeId, kind| crate::GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        let edge = |id, source: &GraphNodeId, destination: &GraphNodeId| crate::GraphEdge {
            id,
            source: port(source, crate::GraphPortKind::MainOutput),
            destination: port(destination, crate::GraphPortKind::MainInput),
            path: "$.issue920".to_owned(),
        };
        let mut edges = Vec::new();
        for track in 0..fan_in {
            edges.push(edge(
                GraphEdgeId::RouteSource {
                    route_id: route_id(track),
                },
                &inputs[track],
                &routes[track],
            ));
            let destination = match shape {
                RoutedShape::ObservedRouteAlias if track == 0 => &alias,
                RoutedShape::RouteIntoSubmix if track == fan_in - 1 => &submix,
                _ => &output,
            };
            edges.push(edge(
                GraphEdgeId::RouteDestination {
                    route_id: route_id(track),
                },
                &routes[track],
                destination,
            ));
        }
        let mut first = inputs.clone();
        let mut between = Vec::new();
        let mut last = Vec::new();
        match shape {
            RoutedShape::SharedInput => {
                let shared = GraphNodeId::Route {
                    route_id: route_id(fan_in),
                };
                edges.push(edge(
                    GraphEdgeId::RouteSource {
                        route_id: route_id(fan_in),
                    },
                    &inputs[0],
                    &shared,
                ));
                edges.push(edge(
                    GraphEdgeId::RouteDestination {
                        route_id: route_id(fan_in),
                    },
                    &shared,
                    &output,
                ));
                routes.push(shared);
            }
            RoutedShape::LateReader => {
                edges.push(edge(
                    GraphEdgeId::TrackMain {
                        target: late.clone(),
                    },
                    &routes[0],
                    &late,
                ));
                last.push(late.clone());
            }
            RoutedShape::ObservedRouteAlias => {
                edges.push(edge(
                    GraphEdgeId::TrackMain {
                        target: output.clone(),
                    },
                    &alias,
                    &output,
                ));
                between.push(alias.clone());
            }
            RoutedShape::SubmixContributor => {
                edges.push(edge(
                    GraphEdgeId::TrackMain {
                        target: submix.clone(),
                    },
                    &extra,
                    &submix,
                ));
                edges.push(edge(
                    GraphEdgeId::TrackMain {
                        target: output.clone(),
                    },
                    &submix,
                    &output,
                ));
                first.push(extra.clone());
                between.push(submix.clone());
            }
            RoutedShape::RouteIntoSubmix => {
                edges.push(edge(
                    GraphEdgeId::TrackMain {
                        target: output.clone(),
                    },
                    &submix,
                    &output,
                ));
                between.push(submix.clone());
            }
            _ => {}
        }
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let outputs = vec![output.clone()];
        let levels: Vec<&Vec<GraphNodeId>> = [&first, &routes, &between, &outputs, &last]
            .into_iter()
            .filter(|level| !level.is_empty())
            .collect();
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.iter().cloned())
            .collect();
        let mut nodes: Vec<_> = schedule
            .iter()
            .cloned()
            .map(|id| crate::GraphNode {
                id,
                latency: effect_contract::LatencySamples(0),
                tail: effect_contract::TailSamples::Finite(0),
            })
            .collect();
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        let envelope = engine::realtime::RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: engine::QuantumFrames(frames),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let inserted_delays = if shape == RoutedShape::DelayedEdge {
            let delayed = GraphEdgeId::RouteDestination {
                route_id: route_id(1),
            };
            vec![crate::InsertedDelay {
                node: GraphNodeId::CompensationDelay {
                    edge_id: Box::new(delayed.clone()),
                },
                edge_id: delayed,
                samples: effect_contract::LatencySamples(3),
            }]
        } else {
            Vec::new()
        };
        // Hostile constants, except over the signed-zero data. There every route's 2x2 is
        // positive, so its mix of `-0.0` is `-0.0`, with two exceptions. On the delayed shape the
        // delayed route's 2x2 is negative, so its mix of the delay line's initial `+0.0` is
        // `-0.0`. On the route-into-submix shape that route's right row is negative, so it mixes
        // `-0.0` to `(-0.0, +0.0)`.
        let mut state = 0x0920_0000_u64 ^ fan_in as u64;
        let mut constant = |negative: bool| {
            let value = hostile_constant(&mut state);
            if !shape.negative_zero() {
                value
            } else if negative {
                -(value.abs() + 0.5)
            } else {
                value.abs() + 0.5
            }
        };
        let delayed = |track: usize| shape == RoutedShape::DelayedEdge && track == 1;
        let right_row = |track: usize| {
            delayed(track) || (shape == RoutedShape::RouteIntoSubmix && track + 1 == fan_in)
        };
        let prepared_routes = routes
            .iter()
            .enumerate()
            .map(|(track, node)| crate::PreparedRoute {
                node: node.clone(),
                transform: RouteTransform {
                    gain: constant(false),
                    ll: constant(delayed(track)),
                    lr: constant(delayed(track)),
                    rl: constant(right_row(track)),
                    rr: constant(right_row(track)),
                },
            })
            .collect();
        let mut required_bindings = first.clone();
        required_bindings.push(output.clone());
        required_bindings.extend(last.iter().cloned());
        if matches!(
            shape,
            RoutedShape::SubmixContributor | RoutedShape::RouteIntoSubmix
        ) {
            required_bindings.push(submix.clone());
        }
        let plan = crate::PreparedGraphPlan::new(crate::PreparedGraphPlanParts {
            plan_id: 920,
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels
                .iter()
                .enumerate()
                .map(|(level, nodes)| crate::DependencyLevel {
                    level: level as u64,
                    nodes: (*nodes).clone(),
                })
                .collect(),
            route_timings: Vec::new(),
            inserted_delays,
            buffer_assignments: Vec::new(),
            estimate: crate::GraphResourceEstimate {
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
            },
            envelope,
            required_bindings,
            routes: prepared_routes,
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let mut nodes: Vec<GraphNodeBinding> = first
            .iter()
            .enumerate()
            .map(|(track, node)| {
                GraphNodeBinding::new(
                    node.clone(),
                    Box::new(HostileInput {
                        seed: 0x0920 + track as u64,
                        // The submix contributor's input is `(-0.0, +0.0)`: an identity 2x2 would
                        // mix its left plane to `+0.0` (`0 * r` is `+0.0`), which a pass-through
                        // does not.
                        zeros: shape.negative_zero().then_some(
                            if shape == RoutedShape::SubmixContributor && track == fan_in {
                                [-0.0, 0.0]
                            } else {
                                [-0.0, -0.0]
                            },
                        ),
                    }),
                )
            })
            .collect();
        nodes.push(GraphNodeBinding::identity(output.clone()));
        nodes.extend(last.iter().cloned().map(GraphNodeBinding::identity));
        if matches!(
            shape,
            RoutedShape::SubmixContributor | RoutedShape::RouteIntoSubmix
        ) {
            nodes.push(GraphNodeBinding::identity(submix));
        }
        let mut observed: Vec<(GraphNodeId, u64)> = Vec::new();
        match shape {
            RoutedShape::Metered(_) => {
                observed.extend(
                    inputs
                        .iter()
                        .enumerate()
                        .map(|(track, node)| (node.clone(), track as u64 + 1)),
                );
                observed.push((output.clone(), OUTPUT_METER_HANDLE));
                observed.push((output, OUTPUT_METER_HANDLE + 1));
            }
            RoutedShape::ObservedRouteAlias => observed.push((alias, 1)),
            RoutedShape::LateReader => observed.push((late, 1)),
            _ => {}
        }
        let controlled = shape == RoutedShape::Metered(true);
        let observers = observed
            .into_iter()
            .map(|(node, handle)| {
                let meter = Box::new(WordMeter {
                    handle,
                    accepts_resident: false,
                    published: Arc::clone(published),
                });
                if controlled {
                    GraphNodeObserverBinding::controlled(node, handle, meter)
                } else {
                    GraphNodeObserverBinding::new(node, handle, meter)
                }
            })
            .collect();
        (
            plan,
            crate::GraphRuntimeBindings {
                envelope,
                nodes,
                observers,
            },
        )
    }

    /// One [`RoutedShape`] bound straight to its executor, with the Output route fold declined
    /// when `declined`: the unfused oracle, whose route ops run and whose Output op reduces their
    /// outputs.
    fn bind_routed(
        shape: RoutedShape,
        fan_in: usize,
        frames: u32,
        declined: bool,
        published: &Published,
    ) -> crate::GraphExecutor {
        let (plan, bindings) = routed_output_parts(shape, fan_in, frames, published);
        test_only_set_output_route_fold_declined(declined);
        let (executor, _, _) =
            bind_executor(plan, bindings, shape == RoutedShape::Metered(true), None);
        test_only_set_output_route_fold_declined(false);
        executor
    }

    /// Gates 1 and 2 of issue #926 (issue #920's, reused): the fused Output reduction renders the
    /// route ops' and the reduction's own bits, and every declining shape declines.
    ///
    /// Each case binds one plan twice -- as bound, and with the fold declined through
    /// `test_only_set_output_route_fold_declined` -- and renders eight blocks of each through the
    /// real `GraphExecutor::render` into host planes whose stride is `frames + 3`. Per block the
    /// host storage, padding included, must be bit-identical between the two, and no padding word
    /// may move. Per case: the fold count is the fan-in for an admitted shape and zero for a
    /// declining one (and zero on the oracle); the fold removes exactly one unit per route; every
    /// observer window is the oracle's.
    ///
    /// Shapes: every [`RoutedShape`], `frames` in `{1, 3, 7, 13, 16, 64, 128}`, fan-in sixty-four
    /// (thirty-two pairs), and also two (one pair) and nine (four pairs and a lone last input) for
    /// the admitted shapes.
    /// Hostile words and a hostile 2x2 per route, except on the signed-zero shapes, whose data is
    /// chosen so that the declined clause's hazard reaches the master's sign bit;
    /// [`RoutedShape::NegativeZero`] must keep `-0.0`. Each declining shape declines on its own
    /// clause of `output_route_fold` and no other: the alias meter on `observed`, the delayed edge
    /// on the master's delayed input, both submix shapes on the plain-route clause, the shared
    /// input on the in-place clause and the late reader on sole readership.
    ///
    /// Red mutations (`crates/graph/tests/MUTATIONS.md`, issue #926): the kernel rows, each red
    /// here as well as in the kernel test, and one row per declining clause, each red here on the
    /// host planes or an observer window with the fold-count assertions removed.
    #[test]
    fn an_output_route_fold_is_the_route_ops_and_the_reduction_bit_for_bit() {
        const BLOCKS: u64 = 8;
        let shapes = [
            RoutedShape::Plain,
            RoutedShape::NegativeZero,
            RoutedShape::Metered(false),
            RoutedShape::Metered(true),
            RoutedShape::ObservedRouteAlias,
            RoutedShape::DelayedEdge,
            RoutedShape::SubmixContributor,
            RoutedShape::RouteIntoSubmix,
            RoutedShape::SharedInput,
            RoutedShape::LateReader,
        ];
        for frames in [1_u32, 3, 7, 13, 16, 64, 128] {
            for shape in shapes {
                let fan_ins: &[usize] = if shape.folds() { &[64, 2, 9] } else { &[64] };
                for &fan_in in fan_ins {
                    let case = format!("{shape:?}, fan-in {fan_in}, {frames} frames");
                    let (oracle_published, published) =
                        (Published::default(), Published::default());
                    let mut oracle = bind_routed(shape, fan_in, frames, true, &oracle_published);
                    let mut folded = bind_routed(shape, fan_in, frames, false, &published);
                    let expected = if shape.folds() { fan_in as u64 } else { 0 };
                    assert_eq!(folded.output_route_folds(), expected, "{case}: folds");
                    assert_eq!(
                        oracle.output_route_folds(),
                        0,
                        "{case}: the oracle folds none"
                    );
                    assert_eq!(
                        folded.runtime.units.len() + expected as usize,
                        oracle.runtime.units.len(),
                        "{case}: each folded route is one unit fewer"
                    );
                    let frames = frames as usize;
                    let stride = frames + 3;
                    let mut audible = false;
                    for block in 0..BLOCKS {
                        let first_sample = block * frames as u64;
                        let render = |executor: &mut crate::GraphExecutor| {
                            let mut storage = vec![f32::from_bits(HOST_PAD); 2 * stride];
                            render_host(
                                executor,
                                engine::realtime::PlanarBufferMut::try_new(
                                    &mut storage,
                                    2,
                                    frames,
                                    stride,
                                )
                                .expect("host output"),
                                first_sample,
                            )
                            .expect("render");
                            storage
                                .iter()
                                .map(|word| word.to_bits())
                                .collect::<Vec<_>>()
                        };
                        let expected = render(&mut oracle);
                        let actual = render(&mut folded);
                        assert_eq!(actual, expected, "{case}, block {block}: the host planes");
                        for (index, word) in actual.iter().enumerate() {
                            let padding = index % stride >= frames;
                            assert!(
                                (*word == HOST_PAD) == padding,
                                "{case}, block {block}: word {index} is padding iff untouched"
                            );
                            if !padding && *word != 0 && *word != 0x8000_0000 {
                                audible = true;
                            }
                            if !padding && shape == RoutedShape::NegativeZero {
                                assert_eq!(*word, 0x8000_0000, "{case}: the master keeps -0.0");
                            }
                        }
                    }
                    assert_eq!(
                        audible,
                        !shape.negative_zero(),
                        "{case}: the master carries audio"
                    );
                    let windows = |published: &Published| published.lock().unwrap().clone();
                    let (expected, actual) = (windows(&oracle_published), windows(&published));
                    let observers = match shape {
                        RoutedShape::Metered(_) => fan_in + 2,
                        RoutedShape::ObservedRouteAlias | RoutedShape::LateReader => 1,
                        _ => 0,
                    };
                    assert_eq!(
                        actual.len(),
                        observers * BLOCKS as usize,
                        "{case}: every observer saw every block"
                    );
                    assert_eq!(
                        actual, expected,
                        "{case}: every observer window is the oracle's"
                    );
                }
            }
        }
    }

    // -----------------------------------------------------------------------------------------
    // Issue #927: a plain strip's source claim is read in place by the fused Output reduction.
    // -----------------------------------------------------------------------------------------

    /// What the claims of one issue #927 block hold, by block index.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum RingBlock {
        /// Every claim plays a whole quantum.
        Full,
        /// Every claim plays this many frames, then `+0.0` to the quantum (zeroed in place): a
        /// short end-of-region block.
        Short(usize),
        /// Claim [`RING_UNDERRUN_CLAIM`]'s source played nothing; every other claim plays a whole
        /// quantum. The production driver answers per source, so one claim can underrun alone.
        ClaimUnderrun,
        /// No claim plays.
        Underrun,
    }

    /// The claim whose source underruns alone on a [`RingBlock::ClaimUnderrun`] block. It is a
    /// plain track in every [`RingShape`], and its Output input is paired with an input read from
    /// a played block in the plain shape.
    const RING_UNDERRUN_CLAIM: usize = 9;

    /// Issue #927's script: gate 3's one-claim underrun at block 3, a short block, and a block in
    /// which nothing plays.
    const RING_SCRIPT: [RingBlock; 8] = [
        RingBlock::Full,
        RingBlock::Full,
        RingBlock::Full,
        RingBlock::ClaimUnderrun,
        RingBlock::Full,
        RingBlock::Short(9),
        RingBlock::Underrun,
        RingBlock::Full,
    ];

    /// Whether `claim` plays in `block` of [`RING_SCRIPT`].
    fn ring_plays(block: usize, claim: usize) -> bool {
        match RING_SCRIPT[block % RING_SCRIPT.len()] {
            RingBlock::Full | RingBlock::Short(_) => true,
            RingBlock::ClaimUnderrun => claim != RING_UNDERRUN_CLAIM,
            RingBlock::Underrun => false,
        }
    }

    /// A fake source set with the production driver's contract, per claim: `begin_block` plays
    /// one block of hostile words per channel ([`hostile_sample`]; tail zeroed in place on a short
    /// block), `copy_track_input` copies a playing claim's `(left, right)` channels and fills a
    /// silent one with `+0.0`, and `played_planes` lends a playing claim's channels, `None`
    /// otherwise, until the next `begin_block`. A silent claim's channels keep the previous
    /// block's words, so a read that ignored `None` would see them.
    struct RingSource {
        frames: usize,
        /// Per claim, the `(left, right)` source channels. A mono claim names one channel twice.
        mapping: Vec<[usize; 2]>,
        /// `channels * frames` words: this block's played planes.
        planes: Vec<f32>,
        /// Per claim, whether it played this block.
        played: Vec<bool>,
    }

    impl RingSource {
        /// Claim `c` reads channels `(2c, 2c + 1)`, and every fifth claim `(2c, 2c)`.
        fn new(frames: u32, claims: usize) -> Self {
            let mapping: Vec<[usize; 2]> = (0..claims)
                .map(|claim| {
                    if claim % 5 == 4 {
                        [2 * claim, 2 * claim]
                    } else {
                        [2 * claim, 2 * claim + 1]
                    }
                })
                .collect();
            Self {
                frames: frames as usize,
                planes: vec![0.0; 2 * claims * frames as usize],
                played: vec![false; claims],
                mapping,
            }
        }

        fn channel(&self, channel: usize) -> &[f32] {
            &self.planes[channel * self.frames..(channel + 1) * self.frames]
        }
    }

    impl crate::GraphPreparedSourceSetDriver for RingSource {
        fn claim_count(&self) -> usize {
            self.mapping.len()
        }

        fn begin_block(&mut self, first_sample: u64, frames: u32) -> Result<(), RenderError> {
            let block = (first_sample / u64::from(frames)) as usize;
            for (claim, played) in self.played.iter_mut().enumerate() {
                *played = ring_plays(block, claim);
            }
            let played = match RING_SCRIPT[block % RING_SCRIPT.len()] {
                RingBlock::Underrun => return Ok(()),
                RingBlock::Short(played) => played.min(self.frames),
                RingBlock::Full | RingBlock::ClaimUnderrun => self.frames,
            };
            for (channel, plane) in self.planes.chunks_exact_mut(self.frames).enumerate() {
                if self.mapping.iter().enumerate().any(|(claim, channels)| {
                    channels.contains(&channel) && !ring_plays(block, claim)
                }) {
                    // A silent claim's channels keep last block's words.
                    continue;
                }
                let mut state = 0x0927_u64 ^ ((channel as u64) << 32) ^ first_sample;
                for word in &mut plane[..played] {
                    *word = hostile_sample(&mut state);
                }
                plane[played..].fill(0.0);
            }
            Ok(())
        }

        fn copy_track_input(
            &mut self,
            claim: usize,
            left: &mut [f32],
            right: &mut [f32],
        ) -> Result<(), RenderError> {
            let [left_channel, right_channel] = self.mapping[claim];
            if self.played[claim] {
                left.copy_from_slice(self.channel(left_channel));
                right.copy_from_slice(self.channel(right_channel));
            } else {
                left.fill(0.0);
                right.fill(0.0);
            }
            Ok(())
        }

        fn provides_played_planes(&self) -> bool {
            true
        }

        fn played_planes(&self, claim: usize) -> Option<(&[f32], &[f32])> {
            let [left_channel, right_channel] = *self.mapping.get(claim)?;
            self.played[claim].then(|| (self.channel(left_channel), self.channel(right_channel)))
        }
    }

    /// The tracks of every issue #927 plan: `Input -> Route -> Output` each, sixty-four claims.
    const RING_TRACKS: usize = 64;
    /// The track each [`RingShape`] changes.
    const RING_SPECIAL: usize = 5;

    /// Which issue #927 plan to bind: [`RING_TRACKS`] tracks, each `Input -> Route -> Output`
    /// with every input claimed by a [`RingSource`], and the shape's one change to track
    /// [`RING_SPECIAL`] (`K`).
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum RingShape {
        /// Every claim's only reader is its route, which the Output fold retires: all read in place.
        Plain,
        /// [`RingShape::Plain`] bound with the Output route fold declined: the Output op reduces the
        /// route ops' outputs, so no input of it is a claim, and every claim keeps the copy.
        FoldDeclined,
        /// `K`'s input is delayed in place (`NodeKind::TrackDelay`, clause (a)).
        TrackDelayed,
        /// A meter on `K`'s `Input` (clause (c)).
        ObservedInput,
        /// A meter on `K`'s `PostFader`, an unlisted builtin stage and so an alias of the input's
        /// buffer after the input op (clause (c), through the alias).
        ObservedAlias,
        /// `K`'s input also feeds a send route into a submix bus, whose own route feeds the Output.
        /// `K`'s path runs through its bound `PostFader` (an identity), so `K`'s route stays plain
        /// and in place and the fold holds: sixty-five inputs. `K` has two readers (clause (b')).
        SendTap,
        /// `K`'s edge into its bound `PostFader` carries a compensation delay: the fader stages the
        /// input's buffer through its delay line (clause (b')).
        DelayedEdge,
        /// `K`'s input feeds a submix, whose route feeds the Output (clause (b')).
        SubmixReader,
        /// `K`'s input feeds its bound `PostFader`, a processor ([`ScalarTilt`]) that runs in place
        /// over the input's buffer, and the fader's route feeds the Output. The Output's input is
        /// then the claim's own slot, holding the fader's words (clause (b'): the reader is the
        /// fader, not a retired route).
        BoundStage,
        /// `K`'s edge into its own route carries a compensation delay: the route is staged, not in
        /// place, so issue #926's fold declines outright and every claim keeps the copy.
        RouteEdgeDelayed,
        /// `K`'s input is scheduled after an empty submix whose slot it then takes: the submix's
        /// `+0.0` overwrites the copied words before the Output reads them (clause (e)).
        LateInput,
        /// [`RingShape::SendTap`] plus a sixty-fifth claim with no reader, scheduled last among the
        /// inputs: issue #918 binds it in place, and the colouring hands its freed slot to the
        /// fader, whose buffer is then one of the Output's inputs. The Output must not read that
        /// input by the buffer-keyed table.
        DeadClaim,
    }

    impl RingShape {
        const ALL: [Self; 12] = [
            Self::Plain,
            Self::FoldDeclined,
            Self::TrackDelayed,
            Self::ObservedInput,
            Self::ObservedAlias,
            Self::SendTap,
            Self::DelayedEdge,
            Self::SubmixReader,
            Self::BoundStage,
            Self::RouteEdgeDelayed,
            Self::LateInput,
            Self::DeadClaim,
        ];

        /// The Output fold's input count, or zero when it declines.
        const fn folds(self) -> u64 {
            match self {
                Self::FoldDeclined | Self::RouteEdgeDelayed => 0,
                Self::SendTap | Self::DeadClaim => RING_TRACKS as u64 + 1,
                _ => RING_TRACKS as u64,
            }
        }

        /// The claims issue #927 reads in place: every live claim but `K`, unless the fold declined.
        fn output_reads(self) -> Vec<usize> {
            match self {
                Self::Plain => (0..RING_TRACKS).collect(),
                Self::FoldDeclined | Self::RouteEdgeDelayed => Vec::new(),
                _ => (0..RING_TRACKS)
                    .filter(|track| *track != RING_SPECIAL)
                    .collect(),
            }
        }

        /// Every claim, the dead one included.
        const fn claims(self) -> usize {
            RING_TRACKS + matches!(self, Self::DeadClaim) as usize
        }

        /// The claims bound in place: issue #927's, and the dead claim, which issue #918 binds in
        /// place because nothing reads it.
        fn in_place(self) -> Vec<bool> {
            let reads = self.output_reads();
            (0..self.claims())
                .map(|claim| reads.contains(&claim) || claim == RING_TRACKS)
                .collect()
        }
    }

    /// [`RingShape`]'s unbound plan, bindings and source set. Two meters on the Output in every
    /// shape, and one on the observed stage.
    fn ring_output_parts(
        shape: RingShape,
        frames: u32,
        published: &Published,
    ) -> (
        crate::PreparedGraphPlan,
        crate::GraphRuntimeBindings,
        crate::GraphPreparedSourceSet,
    ) {
        let id = |text: String| crate::StableGraphId::parse(&text).expect("stable id");
        let stage = |track: usize, stage| GraphNodeId::TrackStage {
            track_id: id(format!("track{track:02}")),
            stage,
        };
        let route_id = |name: String| id(name);
        let route_node = |name: String| GraphNodeId::Route {
            route_id: route_id(name),
        };
        let track_route = |track: usize| format!("route{track:02}");
        let special = RING_SPECIAL;
        let mut claimed: Vec<GraphNodeId> = (0..RING_TRACKS)
            .map(|track| stage(track, TrackStage::Input))
            .collect();
        if shape == RingShape::DeadClaim {
            claimed.push(stage(99, TrackStage::Input));
        }
        let output = GraphNodeId::Output {
            output_id: id("main".to_owned()),
        };
        let fader = stage(special, TrackStage::PostFader);
        let bus = GraphNodeId::Submix {
            submix_id: id("bus".to_owned()),
        };
        let late = GraphNodeId::Submix {
            submix_id: id("late".to_owned()),
        };
        let port = |node: &GraphNodeId, kind| crate::GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        let edge = |id, source: &GraphNodeId, destination: &GraphNodeId| crate::GraphEdge {
            id,
            source: port(source, crate::GraphPortKind::MainOutput),
            destination: port(destination, crate::GraphPortKind::MainInput),
            path: "$.issue927".to_owned(),
        };
        let mut nodes: Vec<GraphNodeId> = claimed.clone();
        nodes.push(output.clone());
        let mut edges = Vec::new();
        let mut routes: Vec<String> = Vec::new();
        // One route `name` from `source` into `destination`.
        let mut route = |name: String,
                         source: &GraphNodeId,
                         destination: &GraphNodeId,
                         nodes: &mut Vec<GraphNodeId>,
                         edges: &mut Vec<crate::GraphEdge>| {
            let node = route_node(name.clone());
            edges.push(edge(
                GraphEdgeId::RouteSource {
                    route_id: route_id(name.clone()),
                },
                source,
                &node,
            ));
            edges.push(edge(
                GraphEdgeId::RouteDestination {
                    route_id: route_id(name.clone()),
                },
                &node,
                destination,
            ));
            nodes.push(node);
            routes.push(name);
        };
        let bound_fader = matches!(
            shape,
            RingShape::SendTap
                | RingShape::DelayedEdge
                | RingShape::DeadClaim
                | RingShape::BoundStage
        );
        for track in 0..RING_TRACKS {
            let input = stage(track, TrackStage::Input);
            if track != special {
                route(track_route(track), &input, &output, &mut nodes, &mut edges);
                continue;
            }
            match shape {
                RingShape::ObservedAlias => {
                    edges.push(edge(
                        GraphEdgeId::TrackMain {
                            target: fader.clone(),
                        },
                        &input,
                        &fader,
                    ));
                    nodes.push(fader.clone());
                    route(track_route(track), &fader, &output, &mut nodes, &mut edges);
                }
                _ if bound_fader => {
                    edges.push(edge(
                        GraphEdgeId::TrackMain {
                            target: fader.clone(),
                        },
                        &input,
                        &fader,
                    ));
                    nodes.push(fader.clone());
                    route(track_route(track), &fader, &output, &mut nodes, &mut edges);
                    if matches!(shape, RingShape::SendTap | RingShape::DeadClaim) {
                        route("sendroute".to_owned(), &input, &bus, &mut nodes, &mut edges);
                        route("busroute".to_owned(), &bus, &output, &mut nodes, &mut edges);
                        nodes.push(bus.clone());
                    }
                }
                RingShape::SubmixReader => {
                    edges.push(edge(
                        GraphEdgeId::TrackMain {
                            target: bus.clone(),
                        },
                        &input,
                        &bus,
                    ));
                    nodes.push(bus.clone());
                    route("busroute".to_owned(), &bus, &output, &mut nodes, &mut edges);
                }
                _ => route(track_route(track), &input, &output, &mut nodes, &mut edges),
            }
        }
        if shape == RingShape::LateInput {
            nodes.push(late.clone());
        }
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        // Levels by longest path, except that the late shape holds its empty submix at level one
        // and track K's input at level two, after it.
        let mut level: BTreeMap<GraphNodeId, usize> = nodes
            .iter()
            .map(|node| {
                let floor = match shape {
                    RingShape::LateInput if *node == late => 1,
                    RingShape::LateInput if *node == stage(special, TrackStage::Input) => 2,
                    _ => 0,
                };
                (node.clone(), floor)
            })
            .collect();
        loop {
            let mut changed = false;
            for edge in &edges {
                let source = level[&edge.source.node];
                let destination = level.get_mut(&edge.destination.node).expect("node");
                if *destination <= source {
                    *destination = source + 1;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let depth = level.values().copied().max().unwrap_or(0);
        let levels: Vec<Vec<GraphNodeId>> = (0..=depth)
            .map(|depth| {
                level
                    .iter()
                    .filter(|(_, at)| **at == depth)
                    .map(|(node, _)| node.clone())
                    .collect::<Vec<_>>()
            })
            .filter(|nodes| !nodes.is_empty())
            .collect();
        let schedule: Vec<GraphNodeId> = levels.iter().flatten().cloned().collect();
        let mut spec_nodes: Vec<_> = schedule
            .iter()
            .cloned()
            .map(|id| crate::GraphNode {
                id,
                latency: effect_contract::LatencySamples(0),
                tail: effect_contract::TailSamples::Finite(0),
            })
            .collect();
        spec_nodes.sort_by(|left, right| left.id.cmp(&right.id));
        let envelope = engine::realtime::RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: engine::QuantumFrames(frames),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let inserted_delays = match shape {
            RingShape::DelayedEdge => Some((
                GraphEdgeId::TrackMain {
                    target: fader.clone(),
                },
                4,
            )),
            RingShape::RouteEdgeDelayed => Some((
                GraphEdgeId::RouteSource {
                    route_id: route_id(track_route(special)),
                },
                3,
            )),
            _ => None,
        }
        .map(|(edge_id, samples)| crate::InsertedDelay {
            node: GraphNodeId::CompensationDelay {
                edge_id: Box::new(edge_id.clone()),
            },
            edge_id,
            samples: effect_contract::LatencySamples(samples),
        })
        .into_iter()
        .collect();
        let mut state = 0x0927_0000_u64;
        let prepared_routes = routes
            .iter()
            .map(|name| crate::PreparedRoute {
                node: route_node(name.clone()),
                transform: RouteTransform {
                    gain: hostile_constant(&mut state),
                    ll: hostile_constant(&mut state),
                    lr: hostile_constant(&mut state),
                    rl: hostile_constant(&mut state),
                    rr: hostile_constant(&mut state),
                },
            })
            .collect();
        // Every claimed input, the Output, and every bound stage or submix.
        let mut bound: Vec<GraphNodeId> = vec![output.clone()];
        if bound_fader {
            bound.push(fader.clone());
        }
        if nodes.contains(&bus) {
            bound.push(bus.clone());
        }
        if shape == RingShape::LateInput {
            bound.push(late.clone());
        }
        let mut required_bindings = claimed.clone();
        required_bindings.extend(bound.iter().cloned());
        let plan = crate::PreparedGraphPlan::new(crate::PreparedGraphPlanParts {
            plan_id: 927,
            spec: GraphSpec {
                nodes: spec_nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels
                .iter()
                .enumerate()
                .map(|(level, nodes)| crate::DependencyLevel {
                    level: level as u64,
                    nodes: nodes.clone(),
                })
                .collect(),
            route_timings: Vec::new(),
            inserted_delays,
            buffer_assignments: Vec::new(),
            estimate: crate::GraphResourceEstimate {
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
            },
            envelope,
            required_bindings,
            routes: prepared_routes,
            track_delays: if shape == RingShape::TrackDelayed {
                vec![crate::PreparedTrackDelay {
                    node: stage(special, TrackStage::Input),
                    left_samples: 3,
                    right_samples: 5,
                }]
            } else {
                Vec::new()
            },
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let meter = |node: GraphNodeId, handle: u64| {
            GraphNodeObserverBinding::new(
                node,
                handle,
                Box::new(WordMeter {
                    handle,
                    accepts_resident: false,
                    published: Arc::clone(published),
                }),
            )
        };
        let mut observers = vec![
            meter(output.clone(), OUTPUT_METER_HANDLE),
            meter(output, OUTPUT_METER_HANDLE + 1),
        ];
        match shape {
            RingShape::ObservedInput => observers.push(meter(stage(special, TrackStage::Input), 1)),
            RingShape::ObservedAlias => observers.push(meter(fader.clone(), 1)),
            _ => {}
        }
        let source_set = crate::GraphPreparedSourceSet::new(
            envelope,
            claimed
                .iter()
                .map(|node| crate::GraphSourceInputClaim { node: node.clone() })
                .collect(),
            crate::GraphSourceSetResourceReport {
                pcm_payload_already_charged_bytes: 0,
                overhead_bytes: 0,
                total_engine_owned_bytes: 0,
                largest_allocation_bytes: 0,
            },
            Box::new(RingSource::new(frames, claimed.len())),
        );
        (
            plan,
            crate::GraphRuntimeBindings {
                envelope,
                nodes: bound
                    .into_iter()
                    .map(|node| {
                        if shape == RingShape::BoundStage && node == fader {
                            GraphNodeBinding::new(node, Box::new(ScalarTilt))
                        } else {
                            GraphNodeBinding::identity(node)
                        }
                    })
                    .collect(),
                observers,
            },
            source_set,
        )
    }

    /// The quanta every issue #927 shape renders at.
    const RING_FRAMES: [u32; 4] = [1, 7, 16, 128];

    /// Each shape's declined arm as the executor rendered it before the issue: FNV-1a over the
    /// [`RingRun::digest`] of the arm bound with every claim copied at each of [`RING_FRAMES`],
    /// recorded by this fixture compiled against the base tree's `runtime.rs` (`169a2486`, where
    /// the copy was the only path of a bankless plan). Equal pairs are expected: a declined fold
    /// is issue #926's class A, the two observed shapes meter the same words, and the dead claim
    /// contributes nothing.
    const RING_PRE_CHANGE: [(RingShape, u64); 12] = [
        (RingShape::Plain, 0xf2d8_ff22_1c55_f760),
        (RingShape::FoldDeclined, 0xf2d8_ff22_1c55_f760),
        (RingShape::TrackDelayed, 0x75f7_f01f_6f68_5c7e),
        (RingShape::ObservedInput, 0xa855_aedb_d442_640e),
        (RingShape::ObservedAlias, 0xa855_aedb_d442_640e),
        (RingShape::SendTap, 0xfeac_93e4_a545_cd95),
        (RingShape::DelayedEdge, 0xf6ae_27a3_c2f2_9571),
        (RingShape::SubmixReader, 0xa8e2_329f_9960_1a18),
        (RingShape::BoundStage, 0xa490_bc87_d515_7d79),
        (RingShape::RouteEdgeDelayed, 0xb08a_7a66_19e1_9459),
        (RingShape::LateInput, 0x98a4_f2f7_67bb_628b),
        (RingShape::DeadClaim, 0xfeac_93e4_a545_cd95),
    ];

    /// One more `u64` into a running FNV-1a hash, byte by byte.
    fn ring_fnv(hash: u64, value: u64) -> u64 {
        value.to_le_bytes().iter().fold(hash, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3)
        })
    }

    /// [`assert_ring_shape`] at every quantum of [`RING_FRAMES`], and the declined arm's digests
    /// against [`RING_PRE_CHANGE`]. Returns each quantum's runs.
    fn assert_ring_shape_at_every_quantum(shape: RingShape) -> Vec<(u32, RingRun, RingRun)> {
        let mut combined = 0xcbf2_9ce4_8422_2325_u64;
        let runs: Vec<_> = RING_FRAMES
            .iter()
            .map(|&frames| {
                let (in_place, copy) = assert_ring_shape(shape, frames);
                combined = ring_fnv(combined, copy.digest());
                (frames, in_place, copy)
            })
            .collect();
        let pre_change = RING_PRE_CHANGE
            .iter()
            .find(|(recorded, _)| *recorded == shape)
            .map(|(_, digest)| *digest);
        assert_eq!(
            Some(combined),
            pre_change,
            "{shape:?}: the declined arm is the pre-change executor"
        );
        runs
    }

    /// Words no source path ever writes: every claim's arena slot holds these before a block.
    const RING_SLOT_POISON: [u32; 2] = [0x7fc1_0927, 0x7fc2_0927];

    /// What one issue #927 arm rendered and how it was bound: every block's host storage (padding
    /// included), every published meter frame, each claim's mode (`true` in place), the
    /// source-plane counts of each block (`[copies, played reads, silent reads]`), and the Output
    /// route fold's input count.
    struct RingRun {
        hosts: Vec<Vec<u32>>,
        meters: Vec<MeterFrame>,
        in_place: Vec<bool>,
        counts: Vec<[u64; 3]>,
        folds: u64,
        /// The claim each Output input reads in place, per input (`None` for the arena), when the
        /// bind reads any.
        output_claims: Vec<Option<usize>>,
    }

    impl RingRun {
        /// FNV-1a over every host word and every meter frame, in render order.
        fn digest(&self) -> u64 {
            let mut hash = 0xcbf2_9ce4_8422_2325_u64;
            let mut word = |value: u64| {
                for byte in value.to_le_bytes() {
                    hash ^= u64::from(byte);
                    hash = hash.wrapping_mul(0x0100_0000_01b3);
                }
            };
            for host in &self.hosts {
                host.iter().for_each(|bits| word(u64::from(*bits)));
            }
            for frame in &self.meters {
                word(frame.handle);
                word(frame.first_sample);
                frame
                    .left
                    .iter()
                    .chain(&frame.right)
                    .chain(&frame.peak)
                    .chain(&frame.energy)
                    .for_each(|bits| word(u64::from(*bits)));
            }
            hash
        }
    }

    /// The arena slot of every claimed input, in claim order.
    fn ring_slots(plan: &crate::PreparedGraphPlan, claims: usize) -> Vec<u32> {
        let program = plan.lowered().expect("lowered");
        (0..claims)
            .map(|claim| {
                let track = if claim == RING_TRACKS { 99 } else { claim };
                let node = GraphNodeId::TrackStage {
                    track_id: crate::StableGraphId::parse(&format!("track{track:02}"))
                        .expect("stable id"),
                    stage: TrackStage::Input,
                };
                let index = crate::program::node_index(&plan.spec, &node).expect("input node");
                program.node_buffer[index as usize].0 + ARENA_BASE
            })
            .collect()
    }

    /// Bind `shape` -- in place, or with `declined` every claim on the copy -- and render
    /// [`RING_SCRIPT`] into host planes of stride `frames + 3`, poisoning every claim's arena slot
    /// before each block, so a read of a slot the copy no longer fills shows in the bits.
    fn render_ring_shape(shape: RingShape, frames: u32, declined: bool) -> RingRun {
        let published = Published::default();
        let (plan, bindings, source_set) = ring_output_parts(shape, frames, &published);
        let slots = ring_slots(&plan, shape.claims());
        test_only_set_source_in_place_declined(declined);
        test_only_set_output_route_fold_declined(shape == RingShape::FoldDeclined);
        let (mut executor, _, _) = bind_executor(plan, bindings, false, Some(source_set));
        test_only_set_source_in_place_declined(false);
        test_only_set_output_route_fold_declined(false);
        let in_place = slots
            .iter()
            .enumerate()
            .map(|(claim, slot)| executor.runtime.source_in_place(claim, *slot))
            .collect();
        let output_claims = executor
            .runtime
            .output_sources
            .iter()
            .map(|claim| (*claim != NO_SOURCE_CLAIM).then_some(*claim as usize))
            .collect();
        let frames = frames as usize;
        let stride = frames + 3;
        let mut hosts = Vec::new();
        let mut counts = Vec::new();
        for block in 0..RING_SCRIPT.len() as u64 {
            for slot in &slots {
                let (left, right) = executor.runtime.buffer_mut(*slot);
                left.fill(f32::from_bits(RING_SLOT_POISON[0]));
                right.fill(f32::from_bits(RING_SLOT_POISON[1]));
            }
            let mut storage = vec![f32::from_bits(HOST_PAD); 2 * stride];
            test_only_source_plane_reset();
            render_host(
                &mut executor,
                engine::realtime::PlanarBufferMut::try_new(&mut storage, 2, frames, stride)
                    .expect("host output"),
                block * frames as u64,
            )
            .expect("render");
            counts.push(test_only_source_plane_counts());
            hosts.push(storage.iter().map(|word| word.to_bits()).collect());
        }
        let meters = published.lock().unwrap().clone();
        RingRun {
            hosts,
            meters,
            in_place,
            counts,
            folds: executor.output_route_folds(),
            output_claims,
        }
    }

    /// Whether `shape` builds the colouring hazard it is named for, on its lowered program: the
    /// late input takes the empty submix's slot, and the dead claim's slot is an input of the
    /// Output op. Shapes without a hazard answer `true`.
    fn ring_hazard_is_built(shape: RingShape) -> bool {
        let (plan, _, _) = ring_output_parts(shape, 16, &Published::default());
        let program = plan.lowered().expect("lowered");
        let buffer = |node: &GraphNodeId| {
            let index = crate::program::node_index(&plan.spec, node).expect("node");
            program.node_buffer[index as usize]
        };
        let id = |text: &str| crate::StableGraphId::parse(text).expect("stable id");
        let input = |track: &str| GraphNodeId::TrackStage {
            track_id: id(track),
            stage: TrackStage::Input,
        };
        match shape {
            RingShape::LateInput => {
                buffer(&input(&format!("track{RING_SPECIAL:02}")))
                    == buffer(&GraphNodeId::Submix {
                        submix_id: id("late"),
                    })
            }
            RingShape::DeadClaim => {
                let output = output_op(&program, &plan.spec).expect("the Output op");
                let dead = buffer(&input("track99"));
                program
                    .inputs_of(&program.ops[output])
                    .iter()
                    .any(|input| input.buffer == dead)
            }
            _ => true,
        }
    }

    /// One issue #927 shape at one quantum: the in-place arm against the arm bound with
    /// `test_only_set_source_in_place_declined`, which copies every claim as before the issue.
    ///
    /// * **The bound shape.** The Output fold's input count is the shape's on both arms. The mode
    ///   table: in place exactly the claims the shape reads in place (and the dead claim, which
    ///   issue #918 binds); every claim copied on the declined arm. The Output's per-input claims
    ///   name exactly those claims on the in-place arm and nothing on the declined arm.
    /// * **The bits.** Every block's host storage, padding included, bit for bit, with every
    ///   claim's arena slot poisoned before the block; the padding untouched and every frame
    ///   written; and every meter window.
    /// * **The mode counter, per block.** The declined arm copies every claim and reads nothing in
    ///   place. The in-place arm copies exactly the claims it does not read in place, and reads
    ///   each claim it does from the played block when the claim plays and from the silence
    ///   buffer when it does not.
    fn assert_ring_shape(shape: RingShape, frames: u32) -> (RingRun, RingRun) {
        let case = format!("{shape:?}, {frames} frames");
        let in_place = render_ring_shape(shape, frames, false);
        let copy = render_ring_shape(shape, frames, true);
        assert_eq!(in_place.folds, shape.folds(), "{case}: Output route folds");
        assert_eq!(
            copy.folds,
            shape.folds(),
            "{case}: declined, Output route folds"
        );
        assert_eq!(
            in_place.in_place,
            shape.in_place(),
            "{case}: the mode table"
        );
        assert_eq!(
            copy.in_place,
            vec![false; shape.claims()],
            "{case}: declined, every claim is copied"
        );
        let reads = shape.output_reads();
        let mut read = in_place
            .output_claims
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        read.sort_unstable();
        assert_eq!(read, reads, "{case}: the Output inputs read in place");
        if !reads.is_empty() {
            assert_eq!(
                in_place.output_claims.len() as u64,
                shape.folds(),
                "{case}: one entry per Output input"
            );
        }
        assert!(copy.output_claims.is_empty(), "{case}: declined, none");
        let stride = frames as usize + 3;
        let mut audible = false;
        let copied = shape.in_place().iter().filter(|lent| !**lent).count() as u64;
        for (block, (actual, expected)) in in_place.hosts.iter().zip(&copy.hosts).enumerate() {
            assert_eq!(
                actual, expected,
                "{case}, block {block} ({:?}): the host planes are the copy arm's",
                RING_SCRIPT[block]
            );
            for (index, word) in actual.iter().enumerate() {
                let padding = index % stride >= frames as usize;
                assert!(
                    (*word == HOST_PAD) == padding,
                    "{case}, block {block}: word {index} is padding iff untouched"
                );
                audible |= !padding && *word != 0 && *word != 0x8000_0000;
            }
            let played = reads
                .iter()
                .filter(|claim| ring_plays(block, **claim))
                .count() as u64;
            assert_eq!(
                copy.counts[block],
                [shape.claims() as u64, 0, 0],
                "{case}, block {block}: declined [copies, played reads, silent reads]"
            );
            assert_eq!(
                in_place.counts[block],
                [copied, played, reads.len() as u64 - played],
                "{case}, block {block}: in place [copies, played reads, silent reads]"
            );
        }
        assert!(audible, "{case}: the master carries audio");
        assert_eq!(
            in_place.meters, copy.meters,
            "{case}: every meter window is the copy arm's"
        );
        let observed = usize::from(matches!(
            shape,
            RingShape::ObservedInput | RingShape::ObservedAlias
        ));
        assert_eq!(
            in_place.meters.len(),
            (2 + observed) * RING_SCRIPT.len(),
            "{case}: every meter published every block"
        );
        (in_place, copy)
    }

    /// Gates 1 and 3 of issue #927: a plain strip's source claim, whose only reader is the route
    /// the Output fold retired, is read in place from the played block by the fused Output
    /// reduction, and no rendered bit moves.
    ///
    /// Sixty-four tracks `Input -> Route -> Output`, each input claimed by a [`RingSource`] with
    /// hostile words (signed zeros, subnormals, `2^-24 .. 2^25`) and a hostile 2x2 per route, every
    /// fifth claim a mono mapping; frames `{1, 7, 16, 128}`; eight blocks of [`RING_SCRIPT`]. The
    /// in-place arm against the same plan bound with `test_only_set_source_in_place_declined`
    /// ([`assert_ring_shape`]): the host planes bit for bit with every claim's slot poisoned, and
    /// `output_route_folds() == 64` both ways; per block, sixty-four in-place reads and no copy
    /// in place, sixty-four copies and no in-place read declined.
    ///
    /// Gate 3, the underrun: at block 3 claim [`RING_UNDERRUN_CLAIM`]'s source played nothing,
    /// so the declined arm's `copy_track_input` wrote `+0.0` into its slot; the in-place arm reads
    /// the silence buffer for it (one silent read, sixty-three played, no copy) and renders the
    /// same block. At block 6 no claim plays: sixty-four silent reads.
    ///
    /// The declined arm is the executor as it stood before the issue: its digests are the ones
    /// [`RING_PRE_CHANGE`] recorded on the base tree.
    ///
    /// Red mutations: `crates/graph/tests/MUTATIONS.md`, issue #927.
    #[test]
    fn a_plain_strip_source_is_read_in_place_by_the_fused_output_with_the_copy_bits() {
        for (frames, in_place, copy) in assert_ring_shape_at_every_quantum(RingShape::Plain) {
            assert_eq!(in_place.digest(), copy.digest(), "{frames} frames");
            let tracks = RING_TRACKS as u64;
            assert_eq!(in_place.counts[0], [0, tracks, 0], "a whole block");
            assert_eq!(
                in_place.counts[3],
                [0, tracks - 1, 1],
                "gate 3: one claim underran; its read is the silence buffer, and nothing is copied"
            );
            assert_eq!(in_place.counts[6], [0, 0, tracks], "no claim played");
            assert_eq!(
                copy.counts[3],
                [tracks, 0, 0],
                "declined, block 3 copies all"
            );
        }
    }

    /// Gate 2 of issue #927: a claim with any reader but the retired route keeps the copy, and
    /// every other claim of the same plan is still read in place, with the copy arm's bits.
    ///
    /// Per [`RingShape`], at frames `{1, 7, 16, 128}` ([`assert_ring_shape`]): the brief's three --
    /// a send tap reader ([`RingShape::SendTap`]), a delayed edge ([`RingShape::DelayedEdge`]) and a
    /// submix reader ([`RingShape::SubmixReader`]) -- each copy track `K`'s claim alone (one copy
    /// per block) while the other sixty-three are read in place, and so does a bound stage running
    /// in place over the input ([`RingShape::BoundStage`]). Then the other clauses: a
    /// `TrackDelay` input (a), an observed input and an observed alias of it (c), and a late input
    /// whose slot an earlier op overwrites (e), each copying `K` alone; a delay on `K`'s own route
    /// edge and a declined Output fold, where no input is read in place at all because there is no
    /// fused reduction to read it; and a dead claim whose slot the colouring hands to an Output
    /// input, which is read from the arena because the Output selects by input position, never by
    /// buffer. The two colouring shapes are checked to build their hazard, and every shape's
    /// declined arm is the pre-change executor ([`RING_PRE_CHANGE`]).
    #[test]
    fn a_claim_with_another_reader_keeps_the_copy_and_the_copy_bits() {
        for shape in RingShape::ALL {
            assert!(
                ring_hazard_is_built(shape),
                "{shape:?}: the hazard is built"
            );
            if shape != RingShape::Plain {
                let _ = assert_ring_shape_at_every_quantum(shape);
            }
        }
    }

    /// A [`crate::GraphSourcePlanes`] over owned planes: claim `c` lends `planes[c]`, or nothing.
    struct LentPlanes(Vec<Option<(Vec<f32>, Vec<f32>)>>);

    impl crate::GraphSourcePlanes for LentPlanes {
        fn played_planes(&self, claim: usize) -> Option<(&[f32], &[f32])> {
            self.0
                .get(claim)?
                .as_ref()
                .map(|(left, right)| (left.as_slice(), right.as_slice()))
        }
    }

    /// One width's case of issue #927's kernel test: `route_reduce::<L>` reading some inputs from
    /// lent planes (and some of those from the silence buffer) against the same reduction reading
    /// every input from an arena that holds the words the copy would have written.
    fn assert_route_reduce_reads_lent_inputs_as_the_copy<L: Lane>() {
        let bits = |words: &[f32]| words.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
        let mut state = 0x0927_u64 ^ L::WIDTH as u64;
        let pad = f32::from_bits(HOST_PAD);
        let poison = [
            f32::from_bits(RING_SLOT_POISON[0]),
            f32::from_bits(RING_SLOT_POISON[1]),
        ];
        for frames in [1, 3, 7, 8, 13, 16, 33, 64] {
            for fan_in in [2_usize, 3, 5, 9, 64] {
                // Buffer 0 is the silence buffer and buffer 1 the op's own (unused) output.
                let inputs: Vec<u32> = (0..fan_in).map(|index| 2 + index as u32).collect();
                let routes: Vec<[f32; 4]> = (0..fan_in)
                    .map(|_| core::array::from_fn(|_| hostile_constant(&mut state)))
                    .collect();
                // Input `p` is lent unless `p % 3 == 1`, as claim `fan_in - 1 - p`; a lent input
                // with `p % 7 == 5` is an underrun, which the copy writes as `+0.0`.
                let lent = |position: usize| position % 3 != 1;
                let silent = |position: usize| lent(position) && position % 7 == 5;
                let mut oracle = stereo_lease(frames, 2 + fan_in);
                let mut candidate = stereo_lease(frames, 2 + fan_in);
                let mut lends: Vec<Option<(Vec<f32>, Vec<f32>)>> = vec![None; fan_in];
                let mut claims = vec![NO_SOURCE_CLAIM; fan_in];
                for (position, &input) in inputs.iter().enumerate() {
                    let words: [Vec<f32>; 2] = core::array::from_fn(|_| {
                        (0..frames).map(|_| hostile_sample(&mut state)).collect()
                    });
                    for plane in 0..2 {
                        let copied = oracle.write(plane, input);
                        if silent(position) {
                            copied.fill(0.0);
                        } else {
                            copied.copy_from_slice(&words[plane]);
                        }
                        let slot = candidate.write(plane, input);
                        if lent(position) {
                            slot.fill(poison[plane]);
                        } else {
                            slot.copy_from_slice(&words[plane]);
                        }
                    }
                    if lent(position) {
                        let claim = fan_in - 1 - position;
                        claims[position] = claim as u32;
                        let [left, right] = words;
                        lends[claim] = (!silent(position)).then_some((left, right));
                    }
                }
                let lent_planes = LentPlanes(lends);
                let sources = OutputSources {
                    planes: Some(&lent_planes),
                    claims: &claims,
                };
                let case = format!("width {}, {frames} frames, fan-in {fan_in}", L::WIDTH);
                let (mut expected_left, mut expected_right) =
                    (vec![pad; frames], vec![pad; frames]);
                assert!(route_reduce::<L>(
                    &oracle,
                    &inputs,
                    &routes,
                    OutputSources::NONE,
                    &mut expected_left,
                    &mut expected_right
                ));
                let (mut left, mut right) = (vec![pad; frames], vec![pad; frames]);
                test_only_source_plane_reset();
                assert!(
                    route_reduce::<L>(&candidate, &inputs, &routes, sources, &mut left, &mut right),
                    "{case}: an admitted shape reduces"
                );
                let lent_count = (0..fan_in).filter(|position| lent(*position)).count() as u64;
                let silent_count = (0..fan_in).filter(|position| silent(*position)).count() as u64;
                assert_eq!(
                    test_only_source_plane_counts(),
                    [0, lent_count - silent_count, silent_count],
                    "{case}: [copies, played reads, silent reads]"
                );
                assert_eq!(bits(&left), bits(&expected_left), "{case}: left");
                assert_eq!(bits(&right), bits(&expected_right), "{case}: right");
            }
        }
        // Refused before any write: a claim table that is neither empty nor one entry per input.
        let lease = stereo_lease(5, 4);
        let lent_planes = LentPlanes(Vec::new());
        for claims in [&[NO_SOURCE_CLAIM][..], &[NO_SOURCE_CLAIM; 3][..]] {
            let (mut left, mut right) = (vec![pad; 5], vec![pad; 5]);
            assert!(!route_reduce::<L>(
                &lease,
                &[2, 3],
                &[[1.0; 4]; 2],
                OutputSources {
                    planes: Some(&lent_planes),
                    claims,
                },
                &mut left,
                &mut right
            ));
            assert!(
                left.iter()
                    .chain(&right)
                    .all(|word| word.to_bits() == HOST_PAD),
                "a refused shape writes nothing"
            );
        }
    }

    /// Issue #927's kernel: an input read in place is read as the words the copy would have
    /// written, at every lane width.
    ///
    /// Every third input is read from the arena and the rest from lent planes under a claim index
    /// that is not the input's position, some of those lent inputs underrun (the kernel reads the
    /// silence buffer, the copy writes `+0.0`), and every lent input's arena slot is poisoned, so
    /// a read of the slot shows. Pairs therefore mix arena and lent inputs, and lent and silent
    /// ones, including the odd fan-in's lone last input. Frames with ragged tails at both widths;
    /// fan-in two to sixty-four. The oracle is the same reduction over an arena holding the copy's
    /// words, which issue #926's kernel test pins to the route ops and the reduction. Also: a claim
    /// table of the wrong length is refused before any write.
    #[test]
    fn a_route_reduction_reads_each_lent_input_as_the_copys_words() {
        assert_route_reduce_reads_lent_inputs_as_the_copy::<f32>();
        assert_route_reduce_reads_lent_inputs_as_the_copy::<lane::Simd4>();
        assert_route_reduce_reads_lent_inputs_as_the_copy::<lane::Simd8>();
    }
}
