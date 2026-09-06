//! Deterministic control-plane lowering of an accepted session and prepared native effects.
#![allow(missing_docs)]

use crate::banks::rack_location;
use std::collections::{BTreeMap, BTreeSet};

use builtins::BuiltinTail;
use builtins_compiler::{
    PreparedBuiltinsGraphArtifact, PreparedBuiltinsGraphBindFailure, PreparedBuiltinsGraphBound,
    PreparedBuiltinsSession, SessionPoolClasses,
};
use effect_compiler::{EffectPreparedEntry, EffectPreparedSession, EffectRack};
use effect_contract::{BankWidth, ChannelSymmetryWitness};
use effect_contract::{
    LatencySamples, PrepareEffectBankRequest, PreparedSidechainPort, TailSamples,
};
use engine::realtime::RenderEnvelope;
use graph::{
    BufferAssignment, DependencyLevel, EffectNodeId, GraphCompileCaps, GraphDiagnostic,
    GraphDiagnosticSet, GraphEdge, GraphEdgeId, GraphNode, GraphNodeId, GraphPortId, GraphPortKind,
    GraphPreparedEffect, GraphResourceEstimate, GraphSpec, InsertedDelay, PreparedGraphPlan,
    PreparedGraphPlanParts, PreparedRoute, PreparedTrackDelay, RackId, ReductionRecord,
    RouteTiming, RouteTransform, StableGraphId, TrackStage,
};
/// Re-exported so a caller can name the compile input without taking a `lane`
/// dependency of its own: the backend is this crate's input now, so this crate publishes its type
/// (#99 F6). The build's backend is read by the caller -- `lane::Backend::current()`
/// -- and never inside the compiler.
pub use lane::Backend;
use rack::{RackLocation, RackProgram};
use rack_compiler::{BankGroup, BankPlan, CohortCandidate, CohortLevel, plan_bank_groups};
use session::{ChannelMatrix, RouteDestination, RouteSource, SendTap, SidechainDeclaration};
use sha2::{Digest, Sha256};

pub struct GraphCompileRequest {
    pub plan_id: u64,
    pub effects: EffectPreparedSession,
    pub caps: GraphCompileCaps,
    /// The kernel dispatch the SIMD-rack and builtin banks are planned for.
    ///
    /// Compile is a pure function of its inputs (#99 F6). The host CPU is read exactly once, by
    /// the caller that owns the render target -- `capi` and the web host do it at
    /// plan-build time -- never inside the compiler. Before this, the host backend was selected
    /// *inside* compile, so the same `GraphCompileRequest` produced
    /// different banks, a different scratch allocation and a different capped resource estimate
    /// on different machines, and the scalar fallback could not be exercised without feature
    /// injection. The semantic graph -- schedule, levels, PDC, reductions, canonical bytes -- is
    /// deliberately independent of this value; only the bank overlay and the bank half of the
    /// estimate depend on it.
    pub dispatch: Backend,
}
pub struct GraphCompiler;
pub struct PreparedGraphArtifact {
    pub graph: PreparedGraphPlan,
    pub report: GraphCompileReport,
    /// Every track's cohort pool class, as this compile derived it (mono-collapse M1).
    ///
    /// Published because it is the object `compile_with_builtins` must hand to the *second*
    /// planner: `bind_rack_banks` ran inside the compile and read this, and
    /// `PreparedBuiltinsSession::into_graph_artifact_with_banks` reads the same value rather than
    /// re-deriving one. See [`builtins_compiler::SessionPoolClasses`].
    pub pool_classes: SessionPoolClasses,
}
pub struct GraphCompileFailure {
    pub effects: EffectPreparedSession,
    pub diagnostics: GraphDiagnosticSet,
}
/// Compile a graph with internally prepared issue-007 processors and observers.
pub struct GraphBuiltinsCompileRequest {
    pub plan_id: u64,
    pub effects: EffectPreparedSession,
    pub builtins: PreparedBuiltinsSession,
    pub caps: GraphCompileCaps,
    /// See [`GraphCompileRequest::dispatch`] (#99 F6).
    pub dispatch: Backend,
}
/// The one-way, sealed builtin attachment result.
///
/// ```compile_fail
/// use graph_compiler::PreparedGraphBuiltinsArtifact;
///
/// // The compiler-owned graph and builtin parts are private: external bindings cannot create
/// // a value carrying internal-builtin provenance.
/// let _ = PreparedGraphBuiltinsArtifact {};
/// ```
///
/// ```compile_fail
/// fn mutate(mut artifact: graph_compiler::PreparedGraphBuiltinsArtifact) {
///     artifact.graph = panic!("private provenance field");
/// }
/// ```
///
/// ```compile_fail
/// fn extract(artifact: graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let graph_compiler::PreparedGraphBuiltinsArtifact { graph, .. } = artifact;
/// }
/// ```
///
/// ```compile_fail
/// fn clone_back(artifact: graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let _ = artifact.clone();
/// }
/// ```
///
/// ```compile_fail
/// fn back_convert(artifact: graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let _: graph::PreparedGraphPlan = artifact.into();
/// }
/// ```
///
/// ```compile_fail
/// fn generic_internal_attachment(plan: graph::PreparedGraphPlan) {
///     let _ = plan.attach_internal_bindings(Vec::new(), Vec::new());
/// }
/// ```
pub type PreparedGraphBuiltinsArtifact = PreparedBuiltinsGraphArtifact<GraphCompileReport>;
pub type PreparedGraphBuiltinsBound = PreparedBuiltinsGraphBound;
pub type GraphBuiltinsBindFailure = PreparedBuiltinsGraphBindFailure<GraphCompileReport>;
pub struct GraphBuiltinsCompileFailure {
    pub effects: EffectPreparedSession,
    pub builtins: PreparedBuiltinsSession,
    pub diagnostics: GraphDiagnosticSet,
}
/// What compile reports *in addition to* the plan it produced.
///
/// #99 F5: this used to carry a second copy of ten of `PreparedGraphPlan`'s vectors -- nodes,
/// ports, edges, schedule, levels, route timings, inserted delays, reductions, route transforms
/// and buffer assignments -- plus a multi-megabyte canonical text dump, its SHA-256 and a
/// Graphviz string, all built unconditionally on every structural mutation. The report is `Clone`
/// and the capi cloned it, so a compile's peak memory was roughly three times the plan.
///
/// Every deleted field was identical by construction to a field of the artifact's `graph`; read
/// it there. The evidence payload is now produced on demand by [`GraphCompiler::evidence`] and
/// [`GraphCompiler::sha256`], which are the only two things that ever wanted it.
#[derive(Clone, Debug, PartialEq)]
pub struct GraphCompileReport {
    /// Arrival of the sole session output after checked latency and PDC propagation.
    pub output_latency: LatencySamples,
    /// Propagated extent of the sole session output after latency and declared tails.
    pub output_tail: TailSamples,
    /// The retained estimate, including the bank overlay and capped against the session limits.
    pub estimate: GraphResourceEstimate,
    /// The pre-bank estimate that participates in the semantic hash. Dispatch-independent by
    /// construction, which is why the canonical bytes use it rather than [`Self::estimate`].
    pub semantic_estimate: GraphResourceEstimate,
    /// Off-render SIMD-rack cohort decision. It is deliberately absent from graph identity,
    /// schedule, PDC and reductions: changing a host backend cannot change graph semantics.
    pub rack_cohorts: GraphRackBankReport,
}

/// The human- and fixture-facing view of a compiled graph, produced on demand.
///
/// Never built by `compile` (#99 F5). `canonical_bytes` is the deterministic text the semantic
/// SHA-256 is taken over; `dot` is a Graphviz rendering that carries no schedule or buffer
/// content. Producing this is `O(nodes + edges)` allocations and, at scale, tens of megabytes --
/// which is why it is a method rather than a field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphEvidence {
    pub canonical_bytes: Vec<u8>,
    pub sha256: String,
    pub dot: String,
}

/// The bound SIMD-rack bank plan.
///
/// Deliberately absent from graph identity, schedule, PDC and reductions: changing a host backend
/// cannot change graph semantics. This report is the **bound** plan, from the same planner that
/// produced the banks - never a second planner's opinion (#96 F1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphRackBankReport {
    pub dispatch: Backend,
    /// The cohort plan, over whole **rack chains** (#99 F3): one candidate per `(track, rack)`
    /// whose slots are that track's rack program in session order.
    pub plan: BankPlan<RackChainId>,
    /// One entry per bound homogeneous bank, in bind order.
    pub bound_slots: Vec<GraphRackBoundSlot>,
    /// The effect node each chain runs at each of its own slots, in session order. Keyed the same
    /// way the plan is, so a group member can be resolved back to graph nodes.
    pub chains: BTreeMap<RackChainId, Vec<EffectNodeId>>,
}

/// Identifies one track's program in one bankable rack: the unit the cohort planner groups.
///
/// #96's planner takes one candidate per *effect*, which can only ever form single-slot banks.
/// AGENTS.md's cohort model is a whole-rack signature -- "slot types/order, quality, and
/// compatible routing", with absent slots as identity kernels -- so #99 passes whole chains and
/// lets `RackProgram::subsequence_mask` decide which lanes run which slot.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RackChainId {
    pub track_id: String,
    pub rack: RackId,
}

/// One bound homogeneous bank: a slot of a cohort, and the effect node each lane runs there.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphRackBoundSlot {
    /// Index into [`BankPlan::groups`].
    pub group: usize,
    /// Index into that group's leader program.
    pub slot: usize,
    /// One node per lane, in lane order.
    pub members: Vec<EffectNodeId>,
}

impl GraphRackBankReport {
    pub fn groups_in(&self, rack: RackLocation) -> impl Iterator<Item = &BankGroup<RackChainId>> {
        self.plan
            .groups
            .iter()
            .filter(move |group| group.rack == rack)
    }
    /// Groups with at least one slot actually bound as a bank.
    pub fn bound_groups_in(
        &self,
        rack: RackLocation,
    ) -> impl Iterator<Item = &BankGroup<RackChainId>> {
        self.plan
            .groups
            .iter()
            .enumerate()
            .filter(move |(index, group)| {
                group.rack == rack && self.bound_slots.iter().any(|bound| bound.group == *index)
            })
            .map(|(_, group)| group)
    }
    /// Banks bound in one rack, in bind order.
    pub fn bound_slots_in(&self, rack: RackLocation) -> impl Iterator<Item = &GraphRackBoundSlot> {
        self.bound_slots
            .iter()
            .filter(move |bound| self.plan.groups[bound.group].rack == rack)
    }
    /// Effect nodes that render on the per-node scalar path in one rack, in id order: every node
    /// of a candidate that never banked, plus every node at a slot that was not bound.
    #[must_use]
    pub fn scalar_in(&self, rack: RackLocation) -> Vec<EffectNodeId> {
        let banked: std::collections::BTreeSet<&EffectNodeId> = self
            .bound_slots
            .iter()
            .filter(|bound| self.plan.groups[bound.group].rack == rack)
            .flat_map(|bound| bound.members.iter())
            .collect();
        let mut ids: Vec<EffectNodeId> = self
            .chains
            .iter()
            .filter(|(chain, _)| rack_location(chain.rack) == rack)
            .flat_map(|(_, nodes)| nodes.iter())
            .filter(|node| !banked.contains(node))
            .cloned()
            .collect();
        ids.sort();
        ids
    }
}

mod banks;
mod canonical;
mod compile;
mod estimate;
mod ids;
mod pdc;
mod schedule;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::banks::bind_rack_banks;
    use crate::canonical::{
        canonical_parts, edge_text, edge_text_len, hex_sha256, node_text, node_text_len,
        write_canonical,
    };
    use crate::ids::{gid, port, rack_id, stages, track_node};
    use crate::pdc::timings;
    use crate::schedule::{
        buffer_assignments, cycle_witness, cycle_witnesses, is_identity_boundary, topo,
    };

    /// The dispatch every in-crate test compiles with.
    ///
    /// #99 F6 made dispatch an explicit compile input; these tests keep the *previous* behaviour
    /// -- the host's detected capabilities -- so bank membership, scratch allocation and the
    /// capped estimate are unchanged by that move on any given machine.
    /// `scalar_dispatch_compiles_without_banks_on_any_host` is the test that exercises the other
    /// value, which was unreachable before without feature injection.
    fn host_dispatch() -> Backend {
        Backend::current()
    }

    /// Track stages the builtins compiler banks, one bank each per cohort.
    ///
    /// Three since issue #212: the post-input builtin stage, the fader, and the pan matrix. It was
    /// one, and the fader and matrix were 128 individually dispatched per-track ops sitting
    /// *between* the cohorts' chains.
    const BANKABLE_TRACK_STAGES: u64 = 3;

    /// Bank slots one cohort of the intended 64-track strip binds, in cascade order: the post-input
    /// builtins, the EQ, the compressor, the limiter, the fader, the pan matrix.
    ///
    /// The whole run fuses into one chain, so the number that must *not* move when this grows is
    /// `chains`, and the assertions below are written that way -- slots against this constant,
    /// chains against the cohort count.
    const STRIP_SLOTS_PER_COHORT: u64 = BANKABLE_TRACK_STAGES + 3;
    use builtins::{BuiltinLaneSelector, MeterConfig, MeterHandle, MeterSnapshot, MeterTap};
    use builtins_compiler::{
        BuiltinCompileCaps, MeterRequest, PreparedBuiltinsCorruption,
        PreparedBuiltinsCorruptionCase, TrackControlRequest, TrackFaderRecord,
        prepare_session_builtins, prepare_session_builtins_between_render_calls,
        prepare_session_builtins_with_console,
    };
    use conformance::DualAccumulatorDelayFactory;
    use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
    use effect_compiler::{
        EffectCompileCaps, EffectPreparedSession, launch_native_effect_registry,
        prepare_native_session_effects,
    };
    use effect_contract::{
        EffectPrepareError, EffectProcessBlock, NativeEffectFactory, NativeEffectRegistry,
        PrepareEffectBankRequest, PrepareEffectRequest, PreparedNativeEffect,
        PreparedNativeEffectBank, ProcessReport, StatePayloadOutput,
    };
    use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime, audit};
    use graph::{
        GraphBindingBlock, GraphNodeBinding, GraphNodeObserverBinding, GraphObservationBlock,
        GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
    };
    use session::{
        CompileCaps, EffectIdentity, EffectParam, ParameterChannel, ParameterUnit,
        RouteDestination, RouteSource, Sidechain, SidechainDeclaration, StableId, Submix,
        compile_session, parse_session_json,
    };
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    };

    const SESSION_FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.json");
    const CONSOLE_SIXTY_FOUR_TRACK_FIXTURE: &str =
        include_str!("../../../fixtures/session/v1/console-sixty-four-track.json");
    const CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE: &str =
        include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");
    const CONSOLE_SIXTY_FOUR_TRACK_MONO_FIXTURE: &str =
        include_str!("../../../fixtures/session/v1/console-sixty-four-track-mono.json");
    const PARAMETRIC_EQ_NINE_TRACK_FIXTURE: &str =
        include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

    struct IdentityBinding;
    impl GraphRuntimeProcessor for IdentityBinding {
        fn process(
            &mut self,
            _block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            Ok(())
        }
    }

    struct ImpulseBinding;
    impl GraphRuntimeProcessor for ImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            block.left[0] = 1.0;
            block.right[0] = -1.0;
            Ok(())
        }
    }

    struct AsymmetricTrackImpulseBinding {
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for AsymmetricTrackImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            block.left[0] = self.left;
            block.right[0] = self.right;
            Ok(())
        }
    }

    /// A per-track constant on every sample of every block.
    ///
    /// The console fixture's own binding is an impulse at sample 0 and silence after it, which is
    /// the right shape for testing state and the wrong one for testing *timing*: by the block a
    /// live command lands in there is nothing left on the track to scale, so moving the command a
    /// block either way moves no bit. A sustained input is what makes "the command landed on this
    /// block and not the next one" observable at the session output at all
    /// (`a_banked_fader_command_lands_on_the_block_it_was_admitted_in`).
    struct SustainedTrackBinding {
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for SustainedTrackBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            block.left.fill(self.left);
            block.right.fill(self.right);
            Ok(())
        }
    }

    /// The sustained counterpart of [`console_track_input_binding`].
    fn console_track_sustained_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("ch")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("console fixture track id");
        Box::new(SustainedTrackBinding {
            left: 0.03125 * (index % 7 + 1) as f32,
            right: -0.015625 * (index % 5 + 1) as f32,
        })
    }

    /// A one-shot above-ceiling impulse followed by silence. The delayed limiter output therefore
    /// crosses its fixed latency and continues through its frozen release state on later blocks.
    struct LimiterReleaseImpulseBinding {
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for LimiterReleaseImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            if block.first_sample == 0 {
                block.left[0] = self.left;
                block.right[0] = self.right;
            }
            Ok(())
        }
    }

    /// A loud two-band burst plus a later quiet probe. The probe reaches the output while the
    /// compressor release state from the first burst is still active.
    struct MultibandReleaseBinding {
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for MultibandReleaseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            if block.first_sample == 0 {
                for frame in 0..64 {
                    let polarity = if frame & 1 == 0 { 1.0 } else { -1.0 };
                    block.left[frame] = self.left * polarity;
                    block.right[frame] = self.right * polarity;
                }
            } else if block.first_sample == 1_280 {
                block.left[0] = self.left * 0.125;
                block.right[0] = self.right * 0.125;
            }
            Ok(())
        }
    }

    /// One asymmetric impulse followed by silence, exposing the complete finite soft-clip support.
    struct SoftClipImpulseBinding {
        left: f32,
        right: f32,
    }

    /// A level transition followed by a quieter alternating block, keeping both transient
    /// followers active across the consecutive-render boundary.
    struct TransientShaperBinding {
        left: f32,
        right: f32,
    }

    /// One asymmetric impulse followed by silence, exposing delay history across render blocks.
    struct DelayImpulseBinding {
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for DelayImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            if block.first_sample == 0 {
                block.left[0] = self.left;
                block.right[0] = self.right;
            }
            Ok(())
        }
    }
    impl GraphRuntimeProcessor for TransientShaperBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            for frame in 0..block.left.len() {
                let absolute = block.first_sample + frame as u64;
                let level = if absolute < 64 { 1.0 } else { 0.25 };
                let polarity = if absolute & 1 == 0 { 1.0 } else { -1.0 };
                block.left[frame] = self.left * level * polarity;
                block.right[frame] = self.right * level * polarity;
            }
            Ok(())
        }
    }
    impl GraphRuntimeProcessor for SoftClipImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            if block.first_sample == 0 {
                block.left[0] = self.left;
                block.right[0] = self.right;
            }
            Ok(())
        }
    }

    fn asymmetric_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("bank")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("bank fixture track id");
        Box::new(AsymmetricTrackImpulseBinding {
            left: 0.125 * (index + 1) as f32,
            right: -0.0625 * 12_u32.saturating_sub(index) as f32,
        })
    }

    fn parametric_eq_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("eq")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("parametric-EQ fixture track id");
        Box::new(AsymmetricTrackImpulseBinding {
            left: 0.03125 * (index + 1) as f32,
            right: -0.015625 * 9_u32.saturating_sub(index) as f32,
        })
    }

    fn true_peak_limiter_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("eq")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("true-peak limiter fixture track id");
        Box::new(LimiterReleaseImpulseBinding {
            left: 1.125 + 0.0625 * index as f32,
            right: -(1.0625 + 0.03125 * index as f32),
        })
    }

    fn multiband_compressor_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("eq")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("multiband-compressor fixture track id");
        Box::new(MultibandReleaseBinding {
            left: 0.5 + 0.025 * index as f32,
            right: -(0.4 + 0.02 * index as f32),
        })
    }

    fn soft_clip_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("eq")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("soft-clip fixture track id");
        Box::new(SoftClipImpulseBinding {
            left: 0.03125 * (index + 1) as f32,
            right: -0.015625 * (10 - index) as f32,
        })
    }

    fn transient_shaper_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("eq")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("transient-shaper fixture track id");
        Box::new(TransientShaperBinding {
            left: 0.2 + 0.025 * index as f32,
            right: -(0.15 + 0.02 * index as f32),
        })
    }

    fn delay_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("eq")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("delay fixture track id");
        Box::new(DelayImpulseBinding {
            left: 0.05 * (index + 1) as f32,
            right: -0.025 * (10 - index) as f32,
        })
    }

    fn accepted_compressor_graph_fixture() -> session::SessionModel {
        let mut model =
            parse_session_json(PARAMETRIC_EQ_NINE_TRACK_FIXTURE).expect("accepted base fixture");
        let mut tail = model.tracks[7].clone();
        tail.id = StableId::parse("eq9").expect("stable tail id");
        model.tracks.push(tail);
        let mut tail_route = model.routes[7].clone();
        tail_route.id = StableId::parse("eq9-main").expect("stable route id");
        tail_route.source = RouteSource::Track {
            track_id: StableId::parse("eq9").expect("stable tail id"),
            tap: SendTap::PostMatrix,
        };
        model.routes.push(tail_route);
        for track in &mut model.tracks {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("compressor").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("compressor id"),
            };
            effect.params.clear();
            effect.sidechain = SidechainDeclaration::None;
        }
        // `eq8` remains in the accepted graph but must never enter the homogeneous bank.
        model.tracks[8].simd1.effects[0].sidechain = SidechainDeclaration::Routed(Sidechain {
            source: RouteSource::Track {
                track_id: StableId::parse("eq0").expect("stable source id"),
                tap: SendTap::PostMatrix,
            },
            port_id: StableId::parse("sidechain-in").expect("stable sidechain port"),
        });
        model
    }

    /// The compressor fixture with every compressor moved from SIMD-1 to the **dynamic** rack.
    ///
    /// Nothing else moves: same effect ids, same parameters, same routes, same routed sidechain on
    /// `eq8`. The two racks a compressor is *not* in are empty on every track, so both placements
    /// describe the same signal chain -- `input -> builtins -> simd1 -> dynamic -> simd2 -> fader`
    /// with exactly one non-identity stage in it. That is what lets
    /// `rack_placement_changes_the_bank_but_never_the_samples` be a bit test instead of a
    /// tolerance.
    fn accepted_dynamic_rack_compressor_fixture() -> session::SessionModel {
        let mut model = accepted_compressor_graph_fixture();
        for track in &mut model.tracks {
            assert!(
                track.dynamic.effects.is_empty() && track.simd2.effects.is_empty(),
                "the compressor must be the only stage, or the placements are not comparable"
            );
            track.dynamic.effects = core::mem::take(&mut track.simd1.effects);
        }
        model
    }

    /// Compile one accepted model twice: once against the real registry (banks where it can) and
    /// once against a registry whose only difference is that `bind_homogeneous_bank` returns
    /// `Ok(None)` (`ScalarOnlyDelegateFactory`), which forces every instance onto the per-node
    /// path. Same session, same dispatch, same parameters -- the *only* variable is whether the
    /// arithmetic is done a lane at a time or `width` lanes at a time.
    fn compile_bank_and_per_node(
        model: &session::SessionModel,
        effect_id: &str,
        plan_id: u64,
    ) -> (PreparedGraphArtifact, PreparedGraphArtifact) {
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
        .expect("accepted fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let scalar_registry = NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory {
            delegate: registry
                .get_shared_ascii(effect_id)
                .expect("registered launch effect"),
        })
            as Box<dyn NativeEffectFactory>])
        .expect("per-node registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let bank_effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared bank-capable effects");
        let per_node_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared per-node effects");
        let bank = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id,
            effects: bank_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bank graph: {:?}", failure.diagnostics));
        let per_node = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: plan_id + 1,
            effects: per_node_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("per-node graph: {:?}", failure.diagnostics));
        (bank, per_node)
    }

    /// Compile one accepted model against the real launch registry.
    fn compile_bank_only(model: &session::SessionModel, plan_id: u64) -> PreparedGraphArtifact {
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
        .expect("accepted fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let effects = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("prepared effects");
        GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics))
    }

    /// Bind a compiled artifact over the shared impulse input bindings and render `blocks` blocks,
    /// returning the PCM of each block.
    fn render_blocks(artifact: PreparedGraphArtifact, blocks: u64) -> Vec<Vec<f32>> {
        let graph = artifact.graph;
        let envelope = graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let nodes = graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), parametric_eq_input_binding(node)))
            .collect();
        let mut plan = graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
        (0..blocks)
            .map(|block| {
                let mut pcm = vec![0.0_f32; frames * 2];
                plan.render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                            .expect("output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("render");
                pcm
            })
            .collect()
    }

    fn assert_pcm_bits_equal(left: &[Vec<f32>], right: &[Vec<f32>], what: &str) {
        assert_eq!(left.len(), right.len(), "{what}: block count");
        for (block, (left, right)) in left.iter().zip(right).enumerate() {
            assert_eq!(
                left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
                right.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
                "{what}: block {block} differs"
            );
        }
    }

    fn accepted_gate_expander_graph_fixture() -> session::SessionModel {
        let mut model = accepted_compressor_graph_fixture();
        for track in &mut model.tracks {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("gate-expander").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.gate-expander").expect("gate/expander id"),
            };
        }
        model
    }

    fn accepted_true_peak_limiter_graph_fixture() -> session::SessionModel {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("true-peak-limiter").expect("limiter effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.true-peak-limiter").expect("limiter id"),
            };
            effect.link_mode = session::LinkMode::Maximum;
            effect.sidechain = SidechainDeclaration::None;
            let index = index as f32;
            effect.params = vec![
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Db,
                    value: -1.0 - 0.1 * index,
                },
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Right,
                    unit: ParameterUnit::Db,
                    value: -1.5 - 0.1 * index,
                },
                EffectParam {
                    parameter_id: 2,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Milliseconds,
                    value: 100.0 + 10.0 * index,
                },
                EffectParam {
                    parameter_id: 3,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Milliseconds,
                    value: [0.0, 5.0, 10.0][index as usize % 3],
                },
            ];
        }
        model
    }

    fn accepted_multiband_compressor_graph_fixture() -> session::SessionModel {
        let mut model = accepted_compressor_graph_fixture();
        for track in &mut model.tracks {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("multiband-compressor").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.multiband-compressor")
                    .expect("multiband-compressor id"),
            };
            effect.params.clear();
            effect.sidechain = SidechainDeclaration::None;
        }
        model
    }

    fn accepted_soft_clip_graph_fixture() -> session::SessionModel {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("soft-clip").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.soft-clip").expect("soft-clip id"),
            };
            effect.link_mode = session::LinkMode::DualMono;
            effect.sidechain = SidechainDeclaration::None;
            let index = index as f32;
            effect.params = vec![
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Db,
                    value: -6.0 + 0.5 * index,
                },
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Right,
                    unit: ParameterUnit::Db,
                    value: -5.0 + 0.375 * index,
                },
            ];
        }
        model
    }

    fn accepted_transient_shaper_graph_fixture() -> session::SessionModel {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("transient-shaper").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.transient-shaper").expect("transient-shaper id"),
            };
            effect.link_mode = session::LinkMode::DualMono;
            effect.sidechain = SidechainDeclaration::None;
            effect.params = vec![
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Linear,
                    value: 0.75 - 0.05 * index as f32,
                },
                EffectParam {
                    parameter_id: 2,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Linear,
                    value: -0.5 + 0.025 * index as f32,
                },
                EffectParam {
                    parameter_id: 3,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Linear,
                    value: 1.0,
                },
            ];
        }
        model
    }

    fn accepted_delay_graph_fixture() -> session::SessionModel {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let mut effect = track.simd1.effects.remove(0);
            effect.id = StableId::parse("delay").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.delay").expect("delay id"),
            };
            effect.link_mode = session::LinkMode::DualMono;
            effect.sidechain = SidechainDeclaration::None;
            let parameter_index = index as f32;
            effect.params = vec![
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Milliseconds,
                    value: 1.0 + (index % 3) as f32,
                },
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Right,
                    unit: ParameterUnit::Milliseconds,
                    value: 2.0 + (index % 3) as f32,
                },
                EffectParam {
                    parameter_id: 2,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Linear,
                    value: 0.3 + 0.01 * parameter_index,
                },
                EffectParam {
                    parameter_id: 2,
                    channel: ParameterChannel::Right,
                    unit: ParameterUnit::Linear,
                    value: -0.2 - 0.01 * parameter_index,
                },
                EffectParam {
                    parameter_id: 3,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Linear,
                    value: 0.1 + 0.01 * parameter_index,
                },
                EffectParam {
                    parameter_id: 3,
                    channel: ParameterChannel::Right,
                    unit: ParameterUnit::Linear,
                    value: 0.2 + 0.01 * parameter_index,
                },
                EffectParam {
                    parameter_id: 4,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Linear,
                    value: 1.0,
                },
                EffectParam {
                    parameter_id: 4,
                    channel: ParameterChannel::Right,
                    unit: ParameterUnit::Linear,
                    value: 1.0,
                },
                EffectParam {
                    parameter_id: 5,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Linear,
                    value: [0.0, 0.5, 1.0][index % 3],
                },
            ];
            assert!(track.dynamic.effects.is_empty());
            track.dynamic.effects.push(effect);
        }
        model
    }

    /// A deterministic factory failure used to prove the bank binder leaves its already prepared
    /// scalar ownership intact for the caller's transactional failure path.
    struct BankBindErrorFactory;
    impl NativeEffectFactory for BankBindErrorFactory {
        fn descriptor(&self) -> &'static effect_contract::EffectDescriptor {
            DualAccumulatorDelayFactory::correct().descriptor()
        }
        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            DualAccumulatorDelayFactory::correct().prepare(request)
        }
        fn bind_homogeneous_bank(
            &self,
            _request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            Err(EffectPrepareError {
                code: "fixture.bank.bind_failure",
            })
        }
    }

    struct ScalarOnlyFactory;
    impl NativeEffectFactory for ScalarOnlyFactory {
        fn descriptor(&self) -> &'static effect_contract::EffectDescriptor {
            DualAccumulatorDelayFactory::correct().descriptor()
        }
        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            DualAccumulatorDelayFactory::correct().prepare(request)
        }
        fn bind_homogeneous_bank(
            &self,
            _request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            Ok(None)
        }
    }

    /// Test-only scalar fallback for an otherwise identical launch factory. This keeps the
    /// session descriptor and scalar processor identical while exercising graph bank selection.
    struct ScalarOnlyDelegateFactory {
        delegate: Arc<dyn NativeEffectFactory>,
    }
    impl NativeEffectFactory for ScalarOnlyDelegateFactory {
        fn descriptor(&self) -> &'static effect_contract::EffectDescriptor {
            self.delegate.descriptor()
        }
        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            self.delegate.prepare(request)
        }
        fn bind_homogeneous_bank(
            &self,
            _: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            Ok(None)
        }
    }

    struct OrderedPostBankObserver {
        expected_order: u64,
        order: Arc<AtomicU64>,
        observed_post_bank_audio: Arc<AtomicBool>,
    }
    impl GraphRuntimeObserver for OrderedPostBankObserver {
        fn observe(
            &mut self,
            block: GraphObservationBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            assert_eq!(
                self.order.fetch_add(1, Ordering::SeqCst),
                self.expected_order,
                "observers run in stable handle order"
            );
            self.observed_post_bank_audio.store(
                block.left.iter().any(|sample| *sample != 0.0)
                    && block.right.iter().any(|sample| *sample != 0.0),
                Ordering::SeqCst,
            );
            Ok(())
        }
    }

    struct RepeatedOrderedPostBankObserver {
        expected_order: u64,
        order: Arc<AtomicU64>,
        observed_post_bank_audio: Arc<AtomicBool>,
    }
    impl GraphRuntimeObserver for RepeatedOrderedPostBankObserver {
        fn observe(
            &mut self,
            block: GraphObservationBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            let order = self.order.fetch_add(1, Ordering::SeqCst);
            assert_eq!(
                order % 2,
                self.expected_order,
                "observers run in stable handle order on every block"
            );
            self.observed_post_bank_audio.store(
                block.left.iter().any(|sample| *sample != 0.0)
                    && block.right.iter().any(|sample| *sample != 0.0),
                Ordering::SeqCst,
            );
            Ok(())
        }
    }

    fn node(name: &str) -> GraphNodeId {
        GraphNodeId::Submix {
            submix_id: gid(name),
        }
    }

    fn graph_node(name: &str, latency: u64, tail: TailSamples) -> GraphNode {
        GraphNode {
            id: node(name),
            latency: LatencySamples(latency),
            tail,
        }
    }

    fn edge(name: &str, source: &str, destination: &str) -> GraphEdge {
        GraphEdge {
            id: GraphEdgeId::RouteDestination {
                route_id: gid(name),
            },
            source: port(node(source), GraphPortKind::MainOutput),
            destination: port(node(destination), GraphPortKind::MainInput),
            path: format!("$.routes[id={name}]"),
        }
    }

    /// #99 F5: `node_text_len`/`edge_text_len` agree with the formatters they replace, on every
    /// variant and on ids of every length.
    ///
    /// `graph_metadata_bytes` feeds `incremental_plan_bytes` and `session_plus_plan_bytes`, both
    /// of which are checked against the host's graph budget -- so a wrong length is a wrong
    /// admission decision, not a cosmetic drift. The two functions are separate code paths
    /// by design (one allocates, one does not), so they need a gate that keeps them in step.
    #[test]
    fn node_text_len_matches_node_text_for_every_variant() {
        let mut state = 0x1f2e_3d4c_5b6a_7988_u64;
        let mut checked = 0_usize;
        for _ in 0..1_000 {
            let length = (state % 24) as usize + 1;
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let a = "a".repeat(length);
            let b = "b".repeat((length % 7) + 1);
            let c = "c".repeat((length % 11) + 1);
            let effect = EffectNodeId {
                track_id: gid(&a),
                rack: match state % 3 {
                    0 => RackId::Simd1,
                    1 => RackId::Dynamic,
                    _ => RackId::Simd2,
                },
                effect_id: gid(&c),
            };
            let stage = stages()[(state % 7) as usize];
            let variants = [
                GraphNodeId::TrackStage {
                    track_id: gid(&a),
                    stage,
                },
                GraphNodeId::Effect(effect.clone()),
                GraphNodeId::Route { route_id: gid(&b) },
                GraphNodeId::Submix { submix_id: gid(&b) },
                GraphNodeId::Output { output_id: gid(&c) },
                GraphNodeId::CompensationDelay {
                    edge_id: Box::new(GraphEdgeId::TrackMain {
                        target: GraphNodeId::TrackStage {
                            track_id: gid(&a),
                            stage,
                        },
                    }),
                },
            ];
            for node in &variants {
                assert_eq!(
                    node_text_len(node),
                    node_text(node).len(),
                    "node_text_len disagrees for {node:?}"
                );
                checked += 1;
            }
            let edges = [
                GraphEdgeId::TrackMain {
                    target: variants[1].clone(),
                },
                GraphEdgeId::RouteSource { route_id: gid(&b) },
                GraphEdgeId::RouteDestination { route_id: gid(&b) },
                GraphEdgeId::EffectSidechain {
                    effect: effect.clone(),
                    port: b.clone(),
                },
            ];
            for edge in &edges {
                assert_eq!(
                    edge_text_len(edge),
                    edge_text(edge).len(),
                    "edge_text_len disagrees for {edge:?}"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 10_000);
    }

    /// Deterministic xorshift64: the same 500 graphs on every host, every run.
    fn xorshift(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    /// #99 F1 (the property behind the wave-0 fix, gated here rather than assumed).
    ///
    /// Binding rejects any dependency level whose nodes are not strictly
    /// ascending, and it runs for *both* native render modes -- so an unsorted level makes a valid
    /// multi-submix session unbindable on the native launch path. The `direct-route` fixture
    /// cannot see this: it is a chain with exactly one node per level. This test builds graphs
    /// where node-ID order and topological order are deliberately unrelated, and checks the four
    /// properties the executors actually rely on:
    ///
    /// 1. every level is strictly ascending by node id (the native layout contract);
    /// 2. `level(n) == 1 + max level(predecessors)` -- the longest-path definition, recomputed in
    ///    this test from the edge list alone, never from `topo`'s own bookkeeping;
    /// 3. the sequential schedule is the concatenation of the levels, is a permutation of the
    ///    nodes, and puts every edge's source before its destination;
    /// 4. the result does not depend on edge order.
    #[test]
    fn random_dags_have_strictly_ascending_levels_and_level_major_schedule() {
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        for graph in 0..500_u32 {
            let count = (xorshift(&mut state) % 64) as usize + 1;
            // A random topological rank per node, so a node's id says nothing about its depth.
            let mut rank: Vec<usize> = (0..count).collect();
            for index in (1..count).rev() {
                let swap = (xorshift(&mut state) % (index as u64 + 1)) as usize;
                rank.swap(index, swap);
            }
            let name = |index: usize| format!("n{index:02}");
            let nodes: Vec<GraphNode> = (0..count)
                .map(|index| graph_node(&name(index), 0, TailSamples::Finite(0)))
                .collect();
            let mut edges = Vec::new();
            for source in 0..count {
                let fanout = xorshift(&mut state) % 4;
                for _ in 0..fanout {
                    let destination = (xorshift(&mut state) % count as u64) as usize;
                    if rank[source] >= rank[destination] {
                        continue;
                    }
                    let id = format!("e{}-{}", name(source), name(destination));
                    if edges.iter().any(|existing: &GraphEdge| {
                        existing.id == GraphEdgeId::RouteDestination { route_id: gid(&id) }
                    }) {
                        continue;
                    }
                    edges.push(edge(&id, &name(source), &name(destination)));
                }
            }
            let mut nodes = nodes;
            nodes.sort_by(|a, b| a.id.cmp(&b.id));
            edges.sort_by(|a, b| a.id.cmp(&b.id));

            let levels = topo(&nodes, &edges).expect("acyclic by construction");
            let schedule: Vec<GraphNodeId> = levels
                .iter()
                .flat_map(|level| level.nodes.iter().cloned())
                .collect();

            // 1. strictly ascending within every level, and contiguous level numbering.
            for (index, level) in levels.iter().enumerate() {
                assert_eq!(level.level, index as u64, "graph {graph}: level numbering");
                assert!(!level.nodes.is_empty(), "graph {graph}: empty level");
                assert!(
                    level.nodes.windows(2).all(|pair| pair[0] < pair[1]),
                    "graph {graph}: level {index} is not strictly ascending"
                );
            }

            // 2. longest-path levels, recomputed here from the edges alone.
            let mut expected: BTreeMap<GraphNodeId, u64> =
                nodes.iter().map(|node| (node.id.clone(), 0)).collect();
            for _ in 0..count {
                for edge in &edges {
                    let source = expected[&edge.source.node];
                    let destination = expected.get_mut(&edge.destination.node).expect("node");
                    *destination = (*destination).max(source + 1);
                }
            }
            for level in &levels {
                for id in &level.nodes {
                    assert_eq!(expected[id], level.level, "graph {graph}: level of {id:?}");
                }
            }

            // 3. the schedule is the levels, is a permutation, and is topological.
            let mut sorted = schedule.clone();
            sorted.sort();
            let mut all: Vec<GraphNodeId> = nodes.iter().map(|node| node.id.clone()).collect();
            all.sort();
            assert_eq!(sorted, all, "graph {graph}: schedule is not a permutation");
            let position: BTreeMap<&GraphNodeId, usize> = schedule
                .iter()
                .enumerate()
                .map(|(at, id)| (id, at))
                .collect();
            for edge in &edges {
                assert!(
                    position[&edge.source.node] < position[&edge.destination.node],
                    "graph {graph}: edge runs backwards in the schedule"
                );
            }

            // 4. edge order is not an input.
            let mut reversed = edges.clone();
            reversed.reverse();
            assert_eq!(
                topo(&nodes, &reversed).expect("acyclic"),
                levels,
                "graph {graph}: level assignment depends on edge order"
            );
        }
    }

    fn caps(maximum_finite_tail_samples: u64) -> GraphCompileCaps {
        GraphCompileCaps {
            maximum_nodes: 100,
            maximum_edges: 100,
            maximum_schedule_items: 100,
            maximum_dependency_levels: 100,
            maximum_audio_buffer_samples: 100,
            maximum_delay_samples_per_edge: 100,
            maximum_total_delay_samples: 100,
            maximum_graph_bytes: 100,
            maximum_plan_bytes: 100,
            maximum_single_allocation_bytes: 100,
            maximum_finite_tail_samples,
        }
    }

    fn integration_caps() -> GraphCompileCaps {
        GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        }
    }

    fn compile_fixture(plan_id: u64) -> PreparedGraphArtifact {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
        model.tracks[0].dynamic.effects.clear();
        model.automation.clear();
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled session");
        GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id,
            effects: EffectPreparedSession {
                session: compiled,
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics))
    }

    fn compile_reverse_route_submix_fixture(plan_id: u64) -> PreparedGraphArtifact {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
        model.tracks[0].dynamic.effects.clear();
        model.automation.clear();
        model.submixes = vec![
            Submix {
                id: StableId::parse("a-submix").expect("submix id"),
            },
            Submix {
                id: StableId::parse("z-submix").expect("submix id"),
            },
        ];
        let base_route = model.routes[0].clone();
        let mut to_a = base_route.clone();
        to_a.id = StableId::parse("to-a-submix").expect("route id");
        to_a.destination = RouteDestination::SubmixInput {
            submix_id: StableId::parse("a-submix").expect("submix id"),
        };
        let mut to_z = base_route.clone();
        to_z.id = StableId::parse("to-z-submix").expect("route id");
        to_z.destination = RouteDestination::SubmixInput {
            submix_id: StableId::parse("z-submix").expect("submix id"),
        };
        let mut z_downstream = base_route.clone();
        z_downstream.id = StableId::parse("z-downstream").expect("route id");
        z_downstream.source = RouteSource::SubmixOutput {
            submix_id: StableId::parse("a-submix").expect("submix id"),
        };
        let mut a_downstream = base_route;
        a_downstream.id = StableId::parse("a-downstream").expect("route id");
        a_downstream.source = RouteSource::SubmixOutput {
            submix_id: StableId::parse("z-submix").expect("submix id"),
        };
        model.routes = vec![to_a, to_z, z_downstream, a_downstream];
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled reverse-route submix session");
        GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id,
            effects: EffectPreparedSession {
                session,
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics))
    }

    /// The dependency-level contract, checked against an *explicitly supplied* schedule and level
    /// list rather than against the plan's own.
    ///
    /// #99 F5 moved these vectors out of `GraphCompileReport` and onto `PreparedGraphPlan`, which
    /// is deliberately not `Clone` -- so the corruption cases below inject a mutated copy here
    /// instead of cloning a whole plan to poke one field. That is a better test anyway: it makes
    /// the corrupted input visible at the call site.
    fn dependency_level_contract(
        graph: &PreparedGraphPlan,
        schedule: &[GraphNodeId],
        levels: &[DependencyLevel],
    ) -> Result<(), &'static str> {
        if levels.is_empty() || levels.windows(2).any(|pair| pair[0].level >= pair[1].level) {
            return Err("level ordering");
        }
        let mut level_by_node = BTreeMap::new();
        for level in levels {
            if level.nodes.is_empty() || level.nodes.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err("member order");
            }
            for node in &level.nodes {
                if level_by_node.insert(node.clone(), level.level).is_some() {
                    return Err("duplicate level member");
                }
            }
        }
        let compiled_nodes: BTreeSet<_> = graph
            .spec
            .nodes
            .iter()
            .map(|node| node.id.clone())
            .collect();
        if level_by_node.keys().cloned().collect::<BTreeSet<_>>() != compiled_nodes {
            return Err("level membership");
        }
        let schedule_nodes: BTreeSet<_> = schedule.iter().cloned().collect();
        if schedule_nodes != compiled_nodes || schedule_nodes.len() != schedule.len() {
            return Err("schedule membership");
        }
        if graph
            .spec
            .edges
            .iter()
            .any(|edge| level_by_node[&edge.source.node] >= level_by_node[&edge.destination.node])
        {
            return Err("edge dependency");
        }
        if schedule
            != levels
                .iter()
                .flat_map(|level| level.nodes.iter().cloned())
                .collect::<Vec<_>>()
        {
            return Err("schedule level order");
        }
        Ok(())
    }

    /// The canonical text with an *injected* schedule and level list, so a test can prove a
    /// permuted assignment produces different bytes.
    fn canonical_with_levels(
        graph: &PreparedGraphPlan,
        report: &GraphCompileReport,
        schedule: &[GraphNodeId],
        levels: &[DependencyLevel],
    ) -> Vec<u8> {
        let reductions = GraphCompiler::reductions(graph);
        let mut parts = canonical_parts(graph, report, &reductions);
        parts.schedule = schedule;
        parts.levels = levels;
        let mut text = String::new();
        write_canonical(&mut text, parts);
        text.into_bytes()
    }

    #[allow(clippy::too_many_arguments)]
    fn reverse_fixture_identity_contract(
        graph: &PreparedGraphPlan,
        report: &GraphCompileReport,
        schedule: &[GraphNodeId],
        levels: &[DependencyLevel],
        canonical: &[u8],
        expected_schedule: &[&str],
        expected_sha256: &str,
    ) -> Result<(), &'static str> {
        dependency_level_contract(graph, schedule, levels)?;
        if schedule.iter().map(node_text).collect::<Vec<_>>() != expected_schedule {
            return Err("schedule identity");
        }
        let rebuilt = canonical_with_levels(graph, report, schedule, levels);
        if rebuilt != canonical
            || hex_sha256(&rebuilt) != hex_sha256(canonical)
            || hex_sha256(canonical) != expected_sha256
        {
            return Err("canonical identity");
        }
        Ok(())
    }

    fn render_reverse_route_submix(artifact: PreparedGraphArtifact) -> (Vec<u32>, u64, bool) {
        let envelope = artifact.graph.envelope;
        let nodes = artifact
            .graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor: Box<dyn GraphRuntimeProcessor> = if matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                ) {
                    Box::new(ImpulseBinding)
                } else {
                    Box::new(IdentityBinding)
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let observer_order = Arc::new(AtomicU64::new(0));
        let observed_audio = Arc::new(AtomicBool::new(false));
        let observed_node = track_node("vocal", TrackStage::PostMatrix);
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes,
            observers: vec![
                GraphNodeObserverBinding::new(
                    observed_node.clone(),
                    2,
                    Box::new(OrderedPostBankObserver {
                        expected_order: 1,
                        order: Arc::clone(&observer_order),
                        observed_post_bank_audio: Arc::clone(&observed_audio),
                    }),
                ),
                GraphNodeObserverBinding::new(
                    observed_node,
                    1,
                    Box::new(OrderedPostBankObserver {
                        expected_order: 0,
                        order: Arc::clone(&observer_order),
                        observed_post_bank_audio: Arc::clone(&observed_audio),
                    }),
                ),
            ],
        };
        let mut plan = artifact
            .graph
            .bind(bindings)
            .unwrap_or_else(|failure| panic!("reverse-route bind: {}", failure.code));
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        plan.render(
            RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("reverse-route submix render");
        drop(plan);
        (
            pcm.into_iter().map(f32::to_bits).collect(),
            observer_order.load(Ordering::SeqCst),
            observed_audio.load(Ordering::SeqCst),
        )
    }

    #[test]
    fn issue122_reverse_route_ids_emit_sorted_levels_and_bind() {
        let baseline = compile_reverse_route_submix_fixture(122_000);
        let existing_fixture = compile_fixture(122_001);
        dependency_level_contract(
            &existing_fixture.graph,
            &existing_fixture.graph.sequential_schedule,
            &existing_fixture.graph.dependency_levels,
        )
        .expect("existing deterministic fixture level contract");

        let expected_schedule = [
            "track:vocal:input",
            "track:vocal:post-input-builtins",
            "track:vocal:post-simd1",
            "track:vocal:post-dynamic",
            "track:vocal:post-simd2-pre-fader",
            "track:vocal:post-fader",
            "track:vocal:post-matrix",
            "route:to-a-submix",
            "route:to-z-submix",
            "submix:a-submix",
            "submix:z-submix",
            "route:a-downstream",
            "route:z-downstream",
            "output:main-out",
        ];
        // #241 re-pin: this graph identity commits the canonical session source shape; its
        // schedule, dependency levels, and rendered PCM remain independently fixed below.
        reverse_fixture_identity_contract(
            &baseline.graph,
            &baseline.report,
            &baseline.graph.sequential_schedule,
            &baseline.graph.dependency_levels,
            &GraphCompiler::evidence(&baseline.graph, &baseline.report).canonical_bytes,
            &expected_schedule,
            "14d73acde3dfc2a57a7a3c797151d675440b7c987aed85b2911ca94e5fac07c3",
        )
        .expect("sorted production identity");
        let level_transcript: Vec<_> = baseline
            .graph
            .dependency_levels
            .iter()
            .map(|level| {
                (
                    level.level,
                    level.nodes.iter().map(node_text).collect::<Vec<_>>(),
                )
            })
            .collect();
        let expected_levels = vec![
            (0, vec!["track:vocal:input".to_owned()]),
            (1, vec!["track:vocal:post-input-builtins".to_owned()]),
            (2, vec!["track:vocal:post-simd1".to_owned()]),
            (3, vec!["track:vocal:post-dynamic".to_owned()]),
            (4, vec!["track:vocal:post-simd2-pre-fader".to_owned()]),
            (5, vec!["track:vocal:post-fader".to_owned()]),
            (6, vec!["track:vocal:post-matrix".to_owned()]),
            (
                7,
                vec![
                    "route:to-a-submix".to_owned(),
                    "route:to-z-submix".to_owned(),
                ],
            ),
            (
                8,
                vec!["submix:a-submix".to_owned(), "submix:z-submix".to_owned()],
            ),
            (
                9,
                vec![
                    "route:a-downstream".to_owned(),
                    "route:z-downstream".to_owned(),
                ],
            ),
            (10, vec!["output:main-out".to_owned()]),
        ];
        assert_eq!(level_transcript, expected_levels);

        // A level-major schedule with two members of level 9 swapped: the pre-#99 pop-order
        // output. It must fail the contract, and it must hash differently.
        let baseline_canonical =
            GraphCompiler::evidence(&baseline.graph, &baseline.report).canonical_bytes;
        let mut legacy_schedule = baseline.graph.sequential_schedule.clone();
        legacy_schedule.swap(11, 12);
        legacy_schedule.swap(10, 11);
        assert_eq!(
            legacy_schedule.iter().map(node_text).collect::<Vec<_>>(),
            [
                "track:vocal:input",
                "track:vocal:post-input-builtins",
                "track:vocal:post-simd1",
                "track:vocal:post-dynamic",
                "track:vocal:post-simd2-pre-fader",
                "track:vocal:post-fader",
                "track:vocal:post-matrix",
                "route:to-a-submix",
                "route:to-z-submix",
                "submix:a-submix",
                "route:z-downstream",
                "submix:z-submix",
                "route:a-downstream",
                "output:main-out",
            ]
        );
        assert_eq!(
            dependency_level_contract(
                &baseline.graph,
                &legacy_schedule,
                &baseline.graph.dependency_levels
            ),
            Err("schedule level order")
        );
        let legacy_canonical = canonical_with_levels(
            &baseline.graph,
            &baseline.report,
            &legacy_schedule,
            &baseline.graph.dependency_levels,
        );
        assert_ne!(legacy_canonical, baseline_canonical);
        assert_eq!(
            GraphCompiler::sha256(&baseline.graph, &baseline.report),
            "14d73acde3dfc2a57a7a3c797151d675440b7c987aed85b2911ca94e5fac07c3"
        );
        let mut reversed = baseline.graph.dependency_levels.clone();
        reversed[9].nodes.reverse();
        assert_eq!(
            dependency_level_contract(
                &baseline.graph,
                &baseline.graph.sequential_schedule,
                &reversed
            ),
            Err("member order")
        );
        let mut omitted = baseline.graph.dependency_levels.clone();
        omitted[9].nodes.pop();
        assert_eq!(
            dependency_level_contract(
                &baseline.graph,
                &baseline.graph.sequential_schedule,
                &omitted
            ),
            Err("level membership")
        );
        let mut duplicate = baseline.graph.dependency_levels.clone();
        let duplicate_node = duplicate[9].nodes[0].clone();
        duplicate[10].nodes.insert(0, duplicate_node);
        assert_eq!(
            dependency_level_contract(
                &baseline.graph,
                &baseline.graph.sequential_schedule,
                &duplicate
            ),
            Err("duplicate level member")
        );
        assert_eq!(
            reverse_fixture_identity_contract(
                &baseline.graph,
                &baseline.report,
                &legacy_schedule,
                &baseline.graph.dependency_levels,
                &baseline_canonical,
                &expected_schedule,
                "14d73acde3dfc2a57a7a3c797151d675440b7c987aed85b2911ca94e5fac07c3",
            ),
            Err("schedule level order")
        );
        let mut canonical_corruption = baseline_canonical.clone();
        canonical_corruption[0] ^= 1;
        assert_eq!(
            reverse_fixture_identity_contract(
                &baseline.graph,
                &baseline.report,
                &baseline.graph.sequential_schedule,
                &baseline.graph.dependency_levels,
                &canonical_corruption,
                &expected_schedule,
                "14d73acde3dfc2a57a7a3c797151d675440b7c987aed85b2911ca94e5fac07c3",
            ),
            Err("canonical identity")
        );

        let repeated = compile_reverse_route_submix_fixture(122_003);
        assert_eq!(
            repeated.graph.sequential_schedule,
            baseline.graph.sequential_schedule
        );
        assert_eq!(
            GraphCompiler::evidence(&repeated.graph, &repeated.report).canonical_bytes,
            GraphCompiler::evidence(&baseline.graph, &baseline.report).canonical_bytes
        );
        assert_eq!(
            GraphCompiler::sha256(&repeated.graph, &repeated.report),
            GraphCompiler::sha256(&baseline.graph, &baseline.report)
        );
        assert_eq!(
            repeated.graph.buffer_assignments,
            baseline.graph.buffer_assignments
        );
        assert_eq!(repeated.report.output_latency, LatencySamples(0));
        assert!(repeated.graph.inserted_delays.is_empty());

        let single_artifact = compile_reverse_route_submix_fixture(122_004);
        let wave_artifact = compile_reverse_route_submix_fixture(122_005);
        for artifact in [&single_artifact, &wave_artifact] {
            assert_eq!(
                artifact.report.output_latency,
                baseline.report.output_latency
            );
            assert_eq!(
                artifact.graph.inserted_delays,
                baseline.graph.inserted_delays
            );
            assert_eq!(artifact.graph.route_timings, baseline.graph.route_timings);
            assert_eq!(
                GraphCompiler::evidence(&artifact.graph, &artifact.report).canonical_bytes,
                GraphCompiler::evidence(&baseline.graph, &baseline.report).canonical_bytes
            );
            assert_eq!(
                GraphCompiler::sha256(&artifact.graph, &artifact.report),
                GraphCompiler::sha256(&baseline.graph, &baseline.report)
            );
        }
        let single = render_reverse_route_submix(single_artifact);
        let repeat = render_reverse_route_submix(wave_artifact);
        assert_eq!(single.0, repeat.0);
        assert_eq!(single.0[0], 2.0_f32.to_bits());
        assert_eq!(single.0[128], (-2.0_f32).to_bits());
        assert!(single.0[1..128].iter().all(|sample| *sample == 0));
        assert!(single.0[129..].iter().all(|sample| *sample == 0));
        assert_eq!((single.1, single.2), (2, true));
        assert_eq!((repeat.1, repeat.2), (2, true));
    }

    #[test]
    fn direct_graph_report_exposes_zero_output_latency_and_tail_without_identity_change() {
        let first = compile_fixture(700);
        let second = compile_fixture(701);
        assert_eq!(first.report.output_latency, LatencySamples(0));
        assert_eq!(first.report.output_tail, TailSamples::Finite(0));
        assert_eq!(
            GraphCompiler::evidence(&first.graph, &first.report).canonical_bytes,
            GraphCompiler::evidence(&second.graph, &second.report).canonical_bytes
        );
        assert_eq!(
            GraphCompiler::sha256(&first.graph, &first.report),
            GraphCompiler::sha256(&second.graph, &second.report)
        );
        assert_eq!(
            GraphCompiler::evidence(&first.graph, &first.report).dot,
            GraphCompiler::evidence(&second.graph, &second.report).dot
        );
    }

    /// #99 F6: dispatch is an input, so the scalar path is reachable on any host and the
    /// semantic graph does not move when it is taken.
    ///
    /// The same twelve-track prepared session is compiled twice: once with the host's detected
    /// dispatch (which banks on an AVX2/NEON/simd128 machine) and once with the scalar dispatch.
    /// Scalar must produce zero banks, and every semantic output -- schedule, dependency levels,
    /// route timings, inserted delays, reductions, route transforms, buffer assignments and the
    /// canonical bytes/SHA -- must be byte-identical to the banked compile. That is the property
    /// the crate documents ("changing a host backend cannot change graph semantics") and it was
    /// previously untestable, because compile read the CPU itself.
    #[test]
    fn scalar_dispatch_compiles_without_banks_on_any_host() {
        let scalar = Backend::Scalar;
        assert!(
            BankWidth::for_backend(scalar).is_none(),
            "the scalar dispatch must not offer a bank width"
        );
        let (_, _, banked_effects) = twelve_track_bank_fixture();
        let banked = GraphCompiler::compile(GraphCompileRequest {
            plan_id: 771,
            effects: banked_effects,
            caps: integration_caps(),
            dispatch: host_dispatch(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics));
        let (_, _, scalar_effects) = twelve_track_bank_fixture();
        let plain = GraphCompiler::compile(GraphCompileRequest {
            plan_id: 771,
            effects: scalar_effects,
            caps: integration_caps(),
            dispatch: scalar,
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics));

        assert_eq!(plain.graph.prepared_bank_count(), 0);
        let expected_banked =
            BankWidth::for_backend(host_dispatch()).map_or(0, |width| 12 / width.lanes() as usize);
        assert_eq!(banked.graph.prepared_bank_count(), expected_banked);
        // Non-vacuity: on a host that cannot bank at all, both compiles bind zero banks and the
        // comparison proves nothing. Recorded rather than skipped, so a scalar CI host is visible
        // as a gap in the evidence instead of a silent pass. The delivery host is x86-64-v3
        // (AVX2+FMA), where this is 1 bank of 8 plus a 4-track scalar tail.
        assert!(
            expected_banked > 0,
            "host cannot form banks; scalar_dispatch_compiles_without_banks_on_any_host is \
             vacuous here and its evidence must be taken on an AVX2/NEON/simd128 host"
        );

        assert_eq!(
            plain.graph.sequential_schedule,
            banked.graph.sequential_schedule
        );
        assert_eq!(
            plain.graph.dependency_levels,
            banked.graph.dependency_levels
        );
        assert_eq!(plain.graph.route_timings, banked.graph.route_timings);
        assert_eq!(plain.graph.inserted_delays, banked.graph.inserted_delays);
        assert_eq!(
            GraphCompiler::reductions(&plain.graph),
            GraphCompiler::reductions(&banked.graph)
        );
        assert_eq!(plain.graph.routes(), banked.graph.routes());
        assert_eq!(
            plain.graph.buffer_assignments,
            banked.graph.buffer_assignments
        );
        assert_eq!(plain.report.output_latency, banked.report.output_latency);
        assert_eq!(plain.report.output_tail, banked.report.output_tail);
        assert_eq!(
            GraphCompiler::evidence(&plain.graph, &plain.report).canonical_bytes,
            GraphCompiler::evidence(&banked.graph, &banked.report).canonical_bytes
        );
        assert_eq!(
            GraphCompiler::sha256(&plain.graph, &plain.report),
            GraphCompiler::sha256(&banked.graph, &banked.report)
        );
        assert_eq!(
            GraphCompiler::evidence(&plain.graph, &plain.report).dot,
            GraphCompiler::evidence(&banked.graph, &banked.report).dot
        );
        assert_eq!(plain.report.rack_cohorts.dispatch, scalar);
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn live_scalar_owner_bytes_are_published_and_capped_before_binding() {
        let prepare = |caps: GraphCompileCaps, controlled: bool| {
            let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
            model.tracks[0].simd1.effects.clear();
            model.tracks[0].dynamic.effects.clear();
            model.tracks[0].simd2.effects.clear();
            model.automation.clear();
            let session = compile_session(
                &model,
                CompileCaps {
                    max_compiled_model_bytes: u64::MAX,
                    max_requested_runtime_bytes: u64::MAX,
                    max_single_allocation_bytes: u64::MAX,
                    max_queue_items: u64::MAX,
                    max_source_ring_frames: u64::MAX,
                    max_source_ring_bytes: u64::MAX,
                },
            )
            .expect("compiled scalar session");
            let controls = [TrackControlRequest {
                track_id: model.tracks[0].id.as_str().to_owned(),
                queue_capacity: NonZeroUsize::new(4).expect("queue"),
            }];
            let builtins = if controlled {
                prepare_session_builtins_between_render_calls(
                    &session,
                    &[],
                    &controls,
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
            } else {
                prepare_session_builtins(
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
            }
            .expect("prepared scalar builtins");
            GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                dispatch: Backend::Scalar,
                plan_id: if controlled { 443_001 } else { 443_000 },
                effects: EffectPreparedSession {
                    session,
                    entries: Vec::new(),
                },
                builtins,
                caps,
            })
        };

        let plain =
            prepare(integration_caps(), false).unwrap_or_else(|_| panic!("plain scalar graph"));
        let live =
            prepare(integration_caps(), true).unwrap_or_else(|_| panic!("live scalar graph"));
        let plain_resource = plain.graph_resource_estimate();
        let live_resource = live.graph_resource_estimate();
        let scalar_bytes = live_resource.graph_metadata_bytes - plain_resource.graph_metadata_bytes;
        assert!(
            scalar_bytes > 0,
            "live scalar owners add retained graph bytes"
        );
        assert_eq!(
            live_resource.incremental_plan_bytes - plain_resource.incremental_plan_bytes,
            scalar_bytes
        );
        assert_eq!(
            live_resource.session_plus_plan_bytes - plain_resource.session_plus_plan_bytes,
            scalar_bytes
        );
        assert_eq!(live_resource.builtin_bank_count, 0);
        assert_eq!(
            live.report().estimate,
            *live_resource,
            "published pre-bind estimate"
        );

        let mut exact = integration_caps();
        exact.maximum_graph_bytes = live_resource.graph_metadata_bytes;
        exact.maximum_plan_bytes = live_resource.incremental_plan_bytes;
        exact.maximum_single_allocation_bytes = live_resource.largest_allocation_bytes;
        let exact_artifact = prepare(exact, true).unwrap_or_else(|_| panic!("exact scalar caps"));
        assert_eq!(exact_artifact.graph_resource_estimate(), live_resource);

        for field in ["graph", "plan", "largest"] {
            let mut below = exact;
            match field {
                "graph" => below.maximum_graph_bytes -= 1,
                "plan" => below.maximum_plan_bytes -= 1,
                "largest" => below.maximum_single_allocation_bytes -= 1,
                _ => unreachable!(),
            }
            let failure = prepare(below, true).err().expect("one below rejects");
            assert!(failure.diagnostics.diagnostics().iter().any(|diagnostic| {
                diagnostic.code == "graph.resource.limit"
                    && diagnostic.path == "$.graph_compile_caps"
            }));
            assert_eq!(
                failure.builtins.tails().count(),
                1,
                "{field} ownership returned"
            );
        }
    }

    /// `slots` bankable SIMD-1 effects per track, on `tracks` tracks, plus a route per track.
    ///
    /// Generalises `twelve_track_bank_fixture` so #99 F3's evals can exercise a *chain*: with
    /// `slots > 1` every track's SIMD-1 rack is a multi-slot program, which is the case #96's
    /// per-effect candidates could not express at all.
    fn rack_chain_fixture(
        tracks: usize,
        slots: usize,
        depth_of: impl Fn(usize) -> usize,
    ) -> (NativeEffectRegistry, EffectPreparedSession) {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("fixture");
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..tracks)
            .map(|index| {
                let mut track = base_track.clone();
                track.id = StableId::parse(&format!("bank{index:02}")).expect("id");
                track.dynamic.effects.clear();
                let template = base_track.dynamic.effects[0].clone();
                track.simd1.effects = (0..depth_of(index).min(slots))
                    .map(|slot| {
                        let mut effect = template.clone();
                        // Reverse-alphabetical on purpose: `EffectPreparedSession::entries` is
                        // sorted by effect id, so session slot order and entries order disagree
                        // here. That is the trap #96's crate doc calls out for #99, and it is
                        // only detectable with a fixture where the two orders differ.
                        effect.id =
                            StableId::parse(&format!("chain{}", slots - 1 - slot)).expect("id");
                        effect.identity = EffectIdentity::Native {
                            effect_id: StableId::parse("conformance.delay").expect("id"),
                        };
                        effect.params = vec![EffectParam {
                            parameter_id: 1,
                            channel: ParameterChannel::Both,
                            unit: ParameterUnit::Linear,
                            value: 1.0 + index as f32 * 0.01 + slot as f32 * 0.001,
                        }];
                        effect
                    })
                    .collect();
                track
            })
            .collect();
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut route = base_route.clone();
                route.id = StableId::parse(&format!("chain-route{index:02}")).expect("id");
                route.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                route
            })
            .collect();
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled");
        let registry =
            NativeEffectRegistry::new([Box::new(DualAccumulatorDelayFactory::correct())
                as Box<dyn effect_contract::NativeEffectFactory>])
            .expect("registry");
        let effects = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("effects");
        (registry, effects)
    }

    fn compile_chain_fixture(effects: EffectPreparedSession) -> PreparedGraphArtifact {
        GraphCompiler::compile(GraphCompileRequest {
            plan_id: 4242,
            effects,
            caps: integration_caps(),
            dispatch: host_dispatch(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics))
    }

    /// #99 F2: every plan this crate compiles lowers to an executable program, and that program
    /// is strictly smaller than the per-edge model both executors run today.
    ///
    /// The program is derived inside `PreparedGraphPlan::new`, so this runs over whatever the
    /// compiler actually produced rather than over a hand-built spec. It is the gate that proves
    /// the seam is real before either executor is rebuilt against it (#98 owns the kernels).
    #[test]
    fn compiled_plans_always_lower_to_a_smaller_executable_program() {
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            panic!("delivery host must offer a bank width");
        };
        let lanes = width.lanes() as usize;
        let cases: Vec<(&str, PreparedGraphArtifact)> = vec![
            ("direct route", compile_fixture(9_100)),
            (
                "reverse submixes",
                compile_reverse_route_submix_fixture(9_101),
            ),
            ("twelve-track banks", {
                let (_, _, effects) = twelve_track_bank_fixture();
                GraphCompiler::compile(GraphCompileRequest {
                    plan_id: 9_102,
                    effects,
                    caps: integration_caps(),
                    dispatch: host_dispatch(),
                })
                .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics))
            }),
            ("two-slot rack chains", {
                let (_, effects) = rack_chain_fixture(lanes, 2, |_| 2);
                compile_chain_fixture(effects)
            }),
        ];
        let mut measured = Vec::new();
        for (label, artifact) in cases {
            let graph = &artifact.graph;
            let program = graph
                .program()
                .unwrap_or_else(|| panic!("{label}: compiled plan must lower"));

            // Every node is an op or an alias, never both and never neither.
            assert_eq!(
                program.ops.len() + program.taps.len(),
                graph.spec.nodes.len(),
                "{label}: op/alias partition"
            );
            // Ops stay level-major and id-sorted within a level -- the order both executors
            // consume, and the order the native blueprint's layout check requires.
            assert!(
                program
                    .ops
                    .windows(2)
                    .all(|pair| (pair[0].level, pair[0].node) < (pair[1].level, pair[1].node)),
                "{label}: ops are not level-major"
            );
            // The arena is smaller than what the executors allocate today.
            //
            // The comparison is deliberately against the *executor's* model, not against
            // `buffer_assignments` alone: that colouring only counts node outputs, while
            // `GraphExecutor` additionally allocates one contribution `StereoBuffer` per edge and
            // then re-buffers every bank member on top (`audio_buffer_samples` says as much --
            // `colored_outputs + logical_edges`). Comparing against the colouring alone would
            // flatter the program in some graphs and defame it in others: the program keeps a
            // dedicated buffer for each bank-eligible node where the colouring shared one and the
            // executor un-shared it again at bind time.
            let coloured = graph
                .buffer_assignments
                .iter()
                .map(|assignment| assignment.buffer_index)
                .max()
                .map_or(0, |maximum| maximum + 1) as usize;
            let bank_members: usize = graph.prepared_bank_count();
            let executor_buffers = coloured + graph.spec.edges.len() + bank_members;
            assert!(
                (program.buffers as usize) < executor_buffers,
                "{label}: arena {} is not smaller than the {executor_buffers} buffers the \
                 executor allocates ({coloured} coloured + {} edges + {bank_members} bank members)",
                program.buffers,
                graph.spec.edges.len()
            );
            // Identity stage boundaries really do disappear from the schedule.
            assert!(
                program.ops.len() < graph.sequential_schedule.len(),
                "{label}: no schedule item was elided"
            );
            // A bank member's output is never written again once defined: a homogeneous bank
            // keeps every member live from the first gather to the last scatter.
            let mut open = std::collections::BTreeSet::new();
            for op in &program.ops {
                assert!(
                    !open.contains(&op.output),
                    "{label}: an op writes open bank storage"
                );
                if matches!(
                    graph.spec.nodes[op.node as usize].id,
                    GraphNodeId::Effect(ref id) if !matches!(id.rack, RackId::Dynamic)
                ) || matches!(
                    graph.spec.nodes[op.node as usize].id,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::PostInputBuiltins,
                        ..
                    }
                ) {
                    open.insert(op.output);
                }
            }
            measured.push((
                label,
                graph.sequential_schedule.len(),
                program.ops.len(),
                program.taps.len(),
                coloured + graph.spec.edges.len() + bank_members,
                program.buffers,
                program.reduction_count(),
            ));
        }
        // Descriptive, printed under `--nocapture`: what lowering actually buys per fixture.
        for (label, nodes, ops, taps, executor_buffers, arena, reductions) in measured {
            println!(
                "{label}: {nodes} schedule items -> {ops} ops + {taps} aliases; \
                 {executor_buffers} executor buffers -> {arena} arena; {reductions} reductions"
            );
        }
    }

    /// #99 F3: a **multi-slot** rack chain forms one cohort and binds a bank at every slot.
    ///
    /// This is the case #96 could not express. Its planner takes one candidate per effect with a
    /// one-slot program, so a two-effect SIMD-1 rack on eight tracks produced two unrelated
    /// single-slot cohorts that happened to have the same members. #99 hands it the whole chain in
    /// **session order**, so the cohort is the rack program and each slot binds once.
    #[test]
    fn multi_slot_rack_chains_form_one_cohort_and_bind_every_slot() {
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            panic!("delivery host must offer a bank width; evidence is vacuous otherwise");
        };
        let lanes = width.lanes() as usize;
        let (_registry, effects) = rack_chain_fixture(lanes, 2, |_| 2);
        let artifact = compile_chain_fixture(effects);
        let report = &artifact.report.rack_cohorts;

        let groups: Vec<_> = report.groups_in(RackLocation::Simd1).collect();
        assert_eq!(groups.len(), 1, "one cohort for one shared rack program");
        assert_eq!(
            groups[0].program.len(),
            2,
            "the cohort is the two-slot chain"
        );
        assert!(groups[0].is_full());

        let bound: Vec<_> = report.bound_slots_in(RackLocation::Simd1).collect();
        assert_eq!(bound.len(), 2, "one bank per slot of the chain");
        assert_eq!(bound[0].slot, 0);
        assert_eq!(bound[1].slot, 1);
        for slot in &bound {
            assert_eq!(slot.members.len(), lanes);
            // Session order, not `entries` order: the fixture names slot 0 `chain1` and slot 1
            // `chain0`, so binding by entries order would swap these two banks.
            let expected = format!("chain{}", 1 - slot.slot);
            assert!(
                slot.members
                    .iter()
                    .all(|member| member.effect_id.as_str() == expected),
                "slot {} must bind every track's {expected}",
                slot.slot
            );
        }
        assert_eq!(artifact.graph.prepared_bank_count(), 2);
        assert!(report.scalar_in(RackLocation::Simd1).is_empty());
    }

    /// #99 F3: bank membership does not depend on `EffectPreparedSession::entries` order.
    ///
    /// The pre-#96 former chunked `entries` directly, so membership was an artefact of a sort by
    /// effect id. The planner keys on `(level, id, program)` only; this shuffles the prepared
    /// entries and requires the identical bound plan.
    #[test]
    fn bank_membership_is_independent_of_entry_order() {
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            panic!("delivery host must offer a bank width");
        };
        let lanes = width.lanes() as usize;
        let (_r1, effects) = rack_chain_fixture(lanes, 2, |_| 2);
        let baseline = compile_chain_fixture(effects);

        let (_r2, mut shuffled) = rack_chain_fixture(lanes, 2, |_| 2);
        let mut state = 0x2545_f491_4f6c_dd1d_u64;
        for index in (1..shuffled.entries.len()).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            shuffled
                .entries
                .swap(index, (state % (index as u64 + 1)) as usize);
        }
        let candidate = compile_chain_fixture(shuffled);

        assert_eq!(
            candidate.report.rack_cohorts.plan,
            baseline.report.rack_cohorts.plan
        );
        assert_eq!(
            candidate.report.rack_cohorts.bound_slots,
            baseline.report.rack_cohorts.bound_slots
        );
        assert_eq!(
            GraphCompiler::sha256(&candidate.graph, &candidate.report),
            GraphCompiler::sha256(&baseline.graph, &baseline.report)
        );
    }

    /// #99 F3: chains of different depth are bucketed by their first slot's level, and a shorter
    /// chain joins a longer cohort through its subsequence mask rather than being dropped.
    ///
    /// The pre-#96 former chunked before grouping by level, so a chunk straddling two levels
    /// discarded every member in it.
    #[test]
    fn chains_of_different_depths_share_a_cohort_through_identity_slots() {
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            panic!("delivery host must offer a bank width");
        };
        let lanes = width.lanes() as usize;
        // Half the tracks run both slots, half run only the first.
        let (_registry, effects) = rack_chain_fixture(lanes, 2, |index| 1 + index % 2);
        let artifact = compile_chain_fixture(effects);
        let report = &artifact.report.rack_cohorts;

        let groups: Vec<_> = report.groups_in(RackLocation::Simd1).collect();
        assert_eq!(
            groups.len(),
            1,
            "the short chains are a subsequence of the long one"
        );
        assert_eq!(groups[0].program.len(), 2);
        assert!(groups[0].is_full());
        // Every lane runs slot 0; only half run slot 1.
        assert!(groups[0].active_slots.iter().all(|lane| lane[0]));
        assert_eq!(
            groups[0].active_slots.iter().filter(|lane| lane[1]).count(),
            lanes / 2
        );

        // Slot 0 binds; slot 1 cannot, because the effect contract has no per-lane bypass mask
        // yet (#96 F7 / #95), so its members stay on the per-node scalar path.
        let bound: Vec<_> = report.bound_slots_in(RackLocation::Simd1).collect();
        assert_eq!(bound.len(), 1);
        assert_eq!(bound[0].slot, 0);
        assert_eq!(bound[0].members.len(), lanes);
        assert_eq!(report.scalar_in(RackLocation::Simd1).len(), lanes / 2);
    }

    /// Twelve tracks that each carry one bankable SIMD-1 effect, plus a route per track.
    ///
    /// Shared by the bank-binding test and by `scalar_dispatch_compiles_without_banks_on_any_host`
    /// (#99 F6), which needs the *same* prepared session compiled twice under two dispatches.
    fn twelve_track_bank_fixture() -> (
        session::CompiledSession,
        NativeEffectRegistry,
        EffectPreparedSession,
    ) {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("fixture");
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..12)
            .map(|index| {
                let mut track = base_track.clone();
                track.id = StableId::parse(&format!("bank{index}")).expect("id");
                track.dynamic.effects.clear();
                track.simd1.effects = base_track.dynamic.effects.clone();
                let effect = &mut track.simd1.effects[0];
                effect.id = StableId::parse("bank-delay").expect("id");
                effect.identity = EffectIdentity::Native {
                    effect_id: StableId::parse("conformance.delay").expect("id"),
                };
                effect.params = vec![EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Linear,
                    value: 1.0 + index as f32 * 0.01,
                }];
                track
            })
            .collect();
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut route = base_route.clone();
                route.id = StableId::parse(&format!("bank-route{index}")).expect("id");
                route.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                route
            })
            .collect();
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled");
        let registry =
            NativeEffectRegistry::new([Box::new(DualAccumulatorDelayFactory::correct())
                as Box<dyn effect_contract::NativeEffectFactory>])
            .expect("registry");
        let effects = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("effects");
        (session, registry, effects)
    }

    #[test]
    fn mixed_twelve_track_plan_binds_renders_full_banks_and_scalar_tails_without_graph_changes() {
        let (session, registry, effects) = twelve_track_bank_fixture();
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 998,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|_| panic!("graph"));
        let expected = BankWidth::for_backend(Backend::current())
            .map_or(0, |width| 12 / width.lanes() as usize);
        assert_eq!(artifact.graph.prepared_bank_count(), expected);
        let canonical = GraphCompiler::evidence(&artifact.graph, &artifact.report)
            .canonical_bytes
            .clone();
        assert_eq!(
            canonical,
            GraphCompiler::evidence(&artifact.graph, &artifact.report).canonical_bytes
        );
        let bank_delays = artifact.graph.inserted_delays.clone();
        let bank_output_latency = artifact.report.output_latency;
        let bank_output_tail = artifact.report.output_tail;
        let bank_tails: Vec<_> = artifact
            .graph
            .spec
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node.tail))
            .collect();
        let envelope = artifact.graph.envelope;
        let nodes = artifact
            .graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor = asymmetric_input_binding(&node);
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let observer_order = Arc::new(AtomicU64::new(0));
        let observed_post_bank_audio = Arc::new(AtomicBool::new(false));
        let observed_stage = track_node("bank0", TrackStage::PostSimd1);
        // `bind` consumes the plan, and #99 F5 moved the dependency levels onto it, so keep the
        // copy this test needs afterwards.
        let dependency_levels = artifact.graph.dependency_levels.clone();
        let mut plan = artifact
            .graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes,
                // Reverse input order proves executor sorting by stable handle. The stage is only
                // reached after the bank's gather/process/scatter completion.
                observers: vec![
                    GraphNodeObserverBinding::new(
                        observed_stage.clone(),
                        2,
                        Box::new(OrderedPostBankObserver {
                            expected_order: 1,
                            order: Arc::clone(&observer_order),
                            observed_post_bank_audio: Arc::clone(&observed_post_bank_audio),
                        }),
                    ),
                    GraphNodeObserverBinding::new(
                        observed_stage,
                        1,
                        Box::new(OrderedPostBankObserver {
                            expected_order: 0,
                            order: Arc::clone(&observer_order),
                            observed_post_bank_audio: Arc::clone(&observed_post_bank_audio),
                        }),
                    ),
                ],
            })
            .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        plan.render(
            RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("render full bank/tail graph");
        assert!(pcm.iter().any(|sample| *sample != 0.0));
        assert_eq!(observer_order.load(Ordering::SeqCst), 2);
        assert!(observed_post_bank_audio.load(Ordering::SeqCst));

        let scalar_registry =
            NativeEffectRegistry::new(
                [Box::new(ScalarOnlyFactory) as Box<dyn NativeEffectFactory>],
            )
            .expect("scalar registry");
        let scalar_effects = prepare_native_session_effects(
            &session,
            &scalar_registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("scalar effects");
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 999,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar graph: {:?}", failure.diagnostics));
        assert_eq!(
            GraphCompiler::evidence(&scalar_artifact.graph, &scalar_artifact.report)
                .canonical_bytes,
            canonical
        );
        assert_eq!(scalar_artifact.graph.inserted_delays, bank_delays);
        assert_eq!(scalar_artifact.report.output_latency, bank_output_latency);
        assert_eq!(scalar_artifact.report.output_tail, bank_output_tail);
        assert_eq!(
            scalar_artifact
                .graph
                .spec
                .nodes
                .iter()
                .map(|node| (node.id.clone(), node.tail))
                .collect::<Vec<_>>(),
            bank_tails
        );
        let scalar_envelope = scalar_artifact.graph.envelope;
        let scalar_nodes = scalar_artifact
            .graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor = asymmetric_input_binding(&node);
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let mut scalar_plan = scalar_artifact
            .graph
            .bind(GraphRuntimeBindings {
                envelope: scalar_envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("scalar bind: {}", failure.code));
        let mut scalar_pcm = vec![0.0_f32; frames * 2];
        scalar_plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                        .expect("scalar output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("scalar render");
        let worst = pcm
            .iter()
            .zip(&scalar_pcm)
            .enumerate()
            .max_by(|(_, (bank_a, scalar_a)), (_, (bank_b, scalar_b))| {
                (*bank_a - *scalar_a)
                    .abs()
                    .total_cmp(&(*bank_b - *scalar_b).abs())
            })
            .expect("pcm");
        assert!(
            (worst.1.0 - worst.1.1).abs() <= 1.0e-6 + 2.0e-5 * worst.1.1.abs(),
            "worst output mismatch at {}: bank={} scalar={}",
            worst.0,
            worst.1.0,
            worst.1.1
        );

        // Host dispatch is deliberately detected only while preparing the normal artifact above.
        // These two direct, off-render binding probes exercise both legal factory widths on every
        // development host without pretending that a four-lane runtime was executed on x86.
        for dispatch in [Backend::Simd4, Backend::Simd8] {
            let rebound = prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 20,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("reprepare effects");
            let ids = rebound
                .entries
                .iter()
                .map(|entry| {
                    (
                        (
                            entry.track_id.clone(),
                            rack_id(entry.rack),
                            entry.effect_id.clone(),
                        ),
                        EffectNodeId {
                            track_id: gid(&entry.track_id),
                            rack: rack_id(entry.rack),
                            effect_id: gid(&entry.effect_id),
                        },
                    )
                })
                .collect();
            let lanes = BankWidth::for_backend(dispatch)
                .expect("vector backend")
                .lanes() as usize;
            let (banks, report) = bind_rack_banks(
                &rebound,
                &ids,
                &dependency_levels,
                dispatch,
                &SessionPoolClasses::default(),
            )
            .expect("off-render factory bind");
            assert_eq!(banks.len(), 12 / lanes);
            assert!(banks.iter().all(|bank| {
                bank.members.len() == lanes && bank.active_mask.iter().all(|active| *active)
            }));
            // The report is the bound plan: one entry per bank actually bound, and the padded
            // remainder group is planned but deliberately left unbound (#96 F6/F7).
            assert_eq!(report.bound_slots.len(), banks.len());
            assert!(
                report
                    .bound_slots
                    .iter()
                    .all(|bound| bound.slot == 0 && bound.members.len() == lanes),
                "each track here has a one-slot rack chain"
            );
            assert_eq!(
                report.scalar_in(RackLocation::Simd1).len()
                    + report.scalar_in(RackLocation::Simd2).len(),
                12 % lanes
            );
        }

        let ids_for = |prepared: &EffectPreparedSession| {
            prepared
                .entries
                .iter()
                .map(|entry| {
                    (
                        (
                            entry.track_id.clone(),
                            rack_id(entry.rack),
                            entry.effect_id.clone(),
                        ),
                        EffectNodeId {
                            track_id: gid(&entry.track_id),
                            rack: rack_id(entry.rack),
                            effect_id: gid(&entry.effect_id),
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>()
        };
        let eight = Backend::Simd8;
        let mut connected_fallback = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("reprepare connected fallback");
        connected_fallback.entries[0].metadata.ports.sidechain = PreparedSidechainPort::Connected {
            id: effect_contract::PortId::new("sidechain").expect("static port"),
            required: false,
        };
        let connected_ids = ids_for(&connected_fallback);
        let connected_banks = bind_rack_banks(
            &connected_fallback,
            &connected_ids,
            &dependency_levels,
            eight,
            &SessionPoolClasses::default(),
        )
        .expect("connected sidechain is scalar fallback, not failure");
        assert!(connected_banks.0.iter().all(|bank| {
            bank.members
                .iter()
                .all(|member| member.track_id.as_str() != "bank0")
        }));
        assert!(
            connected_banks
                .1
                .scalar_in(RackLocation::Simd1)
                .iter()
                .any(|member| member.track_id.as_str() == "bank0"),
            "a connected sidechain never banks, and the report says so"
        );

        let same_wave = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("reprepare same-wave fallback");
        let same_wave_ids = ids_for(&same_wave);
        let first_id =
            same_wave_ids[&("bank0".to_owned(), RackId::Simd1, "bank-delay".to_owned())].clone();
        let first = GraphNodeId::Effect(first_id.clone());
        let mut incompatible_levels = dependency_levels.clone();
        for level in &mut incompatible_levels {
            level.nodes.retain(|node| node != &first);
        }
        // F12: a bank never crosses a dependency level. Before #96 the whole chunk holding a
        // level-incompatible member was dropped; the planner now partitions by level *before*
        // chunking, so the member itself never banks while its level-compatible peers still do.
        let (split_banks, split_report) = bind_rack_banks(
            &same_wave,
            &same_wave_ids,
            &incompatible_levels,
            eight,
            &SessionPoolClasses::default(),
        )
        .expect("a level split is a scalar fallback, not a failure");
        assert!(
            split_banks
                .iter()
                .all(|bank| bank.members.iter().all(|member| member != &first_id)),
            "an unscheduled effect never joins a bank"
        );
        let split_levels: BTreeMap<_, _> = incompatible_levels
            .iter()
            .flat_map(|level| {
                level
                    .nodes
                    .iter()
                    .cloned()
                    .map(move |node| (node, level.level))
            })
            .collect();
        for group in &split_report.plan.groups {
            for member in group.members.iter().flatten() {
                // A chain candidate carries its whole rack program; the group's level is the level
                // of its *first* slot, and slot k sits at level + k (#99 F3).
                let nodes = &split_report.chains[member];
                for (offset, node) in nodes.iter().enumerate() {
                    assert_eq!(
                        split_levels
                            .get(&GraphNodeId::Effect(node.clone()))
                            .copied(),
                        Some(group.level + offset as u64),
                        "every planned group is level-uniform at each slot"
                    );
                }
            }
        }

        let rejecting_registry = NativeEffectRegistry::new([
            Box::new(BankBindErrorFactory) as Box<dyn NativeEffectFactory>
        ])
        .expect("registry");
        let rejected = prepare_native_session_effects(
            &session,
            &rejecting_registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("prepare scalar ownership");
        let rejected_ids = ids_for(&rejected);
        let error = match bind_rack_banks(
            &rejected,
            &rejected_ids,
            &dependency_levels,
            eight,
            &SessionPoolClasses::default(),
        ) {
            Ok(_) => panic!("factory failure must reject transactionally"),
            Err(error) => error,
        };
        assert_eq!(error.code, "fixture.bank.bind_failure");
        assert_eq!(
            rejected.entries.len(),
            12,
            "factory failure retained every scalar input"
        );

        // The Issue-037 production audit is explicit-release-only. It intentionally binds the
        // sealed builtin artifact, rather than the old scalar fixture effect bank, and proves
        // that real TPT builtin-bank callbacks reached the prepared render plan.
        if std::env::var_os("MISO_ENGINE_AUDIT_037").is_some() {
            let audit_effects = prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 20,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("audit effects");
            let audit_builtins = prepare_session_builtins(
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
            .expect("audit builtins");
            let audit_artifact =
                GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                    dispatch: host_dispatch(),
                    plan_id: 1_000,
                    effects: audit_effects,
                    builtins: audit_builtins,
                    caps: integration_caps(),
                })
                .unwrap_or_else(|_| panic!("audit graph"));
            let audit_backend = Backend::current();
            let expected_effect_banks = BankWidth::for_backend(audit_backend)
                .map_or(0, |width| 12 / width.lanes() as usize);
            let expected_scalar_tails = BankWidth::for_backend(audit_backend)
                .map_or(12, |width| 12 % width.lanes() as usize);
            // #86 F3: every post-input node is a bank member, so the count is `T.div_ceil(W)`
            // (12 tracks: 2 banks at W8, one of them padded to 8 with 4 identity lanes;
            // 3 banks at W4) and there is no scalar tail at all on a vector host.
            let expected_builtin_banks = BankWidth::for_backend(audit_backend)
                .map_or(0, |width| 12_usize.div_ceil(width.lanes() as usize));
            let expected_builtin_tails = BankWidth::for_backend(audit_backend).map_or(12, |_| 0);
            assert_eq!(
                audit_artifact.prepared_builtin_bank_count(),
                expected_builtin_banks
            );
            assert_eq!(
                expected_effect_banks
                    * BankWidth::for_backend(audit_backend)
                        .map_or(0, |width| width.lanes() as usize)
                    + expected_scalar_tails,
                12
            );
            assert!(
                expected_builtin_banks != 0,
                "audit host needs a selected SIMD backend"
            );
            let actual_members: Vec<_> = audit_artifact
                .prepared_builtin_banks()
                .flat_map(|bank| {
                    assert_eq!(bank.backend, audit_backend);
                    assert_eq!(Some(bank.width), BankWidth::for_backend(audit_backend));
                    assert!(!bank.members.is_empty());
                    assert!(bank.members.len() <= bank.width.lanes() as usize);
                    bank.members.iter().map(|member| match member {
                        GraphNodeId::TrackStage { track_id, stage } => {
                            assert_eq!(*stage, TrackStage::PostInputBuiltins);
                            track_id.as_str().to_owned()
                        }
                        _ => panic!("audit builtin member kind"),
                    })
                })
                .collect();
            let mut expected_members: Vec<_> =
                (0..12).map(|index| format!("bank{index}")).collect();
            expected_members.sort();
            assert_eq!(actual_members, expected_members);
            assert_eq!(12 - actual_members.len(), expected_builtin_tails);
            // Independent oracle for the two frozen op orders this branch re-pins for (#98 F2/F4),
            // on the production 12-track shape: every track's post-matrix output is recorded, the
            // route's folded 2x2 is re-applied here through the f64 unfused oracle, and the output
            // must be exactly those contributions folded left to right in the plan's own stable
            // edge order. It runs on its own short bind, outside the allocation-audited loop.
            {
                let oracle_effects = prepare_native_session_effects(
                    &session,
                    &registry,
                    EffectCompileCaps {
                        maximum_total_state_bytes: 1 << 20,
                        maximum_scratch_bytes: 1 << 20,
                        maximum_automation_spans_per_block: 32,
                    },
                )
                .expect("oracle effects");
                let oracle_builtins = prepare_session_builtins(
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
                .expect("oracle builtins");
                let oracle_artifact =
                    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                        dispatch: host_dispatch(),
                        plan_id: 1_001,
                        effects: oracle_effects,
                        builtins: oracle_builtins,
                        caps: integration_caps(),
                    })
                    .unwrap_or_else(|_| panic!("oracle graph"));
                let routes: BTreeMap<String, [f32; 4]> = oracle_artifact
                    .graph()
                    .routes()
                    .iter()
                    .map(|route| {
                        let GraphNodeId::Route { route_id } = &route.node else {
                            panic!("route node kind")
                        };
                        let transform = route.transform;
                        (
                            route_id.as_str().to_owned(),
                            [
                                transform.gain * transform.ll,
                                transform.gain * transform.lr,
                                transform.gain * transform.rl,
                                transform.gain * transform.rr,
                            ],
                        )
                    })
                    .collect();
                // Output inputs, in the plan's own stable edge order: `(route id, source track)`.
                let contributions: Vec<(String, String)> = oracle_artifact
                    .graph()
                    .spec
                    .edges
                    .iter()
                    .filter(|edge| matches!(edge.destination.node, GraphNodeId::Output { .. }))
                    .map(|edge| {
                        let GraphNodeId::Route { route_id } = &edge.source.node else {
                            panic!("output input is not a route")
                        };
                        let source = oracle_artifact
                            .graph()
                            .spec
                            .edges
                            .iter()
                            .find(|candidate| candidate.destination.node == edge.source.node)
                            .expect("route source edge");
                        let GraphNodeId::TrackStage { track_id, stage } = &source.source.node
                        else {
                            panic!("route source is not a track stage")
                        };
                        assert_eq!(*stage, TrackStage::PostMatrix);
                        (route_id.as_str().to_owned(), track_id.as_str().to_owned())
                    })
                    .collect();
                assert!(
                    contributions.len() >= 4,
                    "the oracle must exercise fan-in >= 4"
                );
                let sinks: Vec<BitSink> = contributions
                    .iter()
                    .map(|_| Arc::new(std::sync::Mutex::new(Vec::new())))
                    .collect();
                let oracle_envelope = oracle_artifact.envelope();
                let oracle_nodes = oracle_artifact
                    .external_binding_nodes()
                    .map(|node| GraphNodeBinding::new(node.clone(), asymmetric_input_binding(node)))
                    .collect();
                let oracle_observers = contributions
                    .iter()
                    .zip(sinks.iter())
                    .enumerate()
                    .map(|(handle, ((_, track), sink))| {
                        GraphNodeObserverBinding::new(
                            GraphNodeId::TrackStage {
                                track_id: StableGraphId::parse(track).expect("track node id"),
                                stage: TrackStage::PostMatrix,
                            },
                            handle as u64,
                            Box::new(BitRecorder(Arc::clone(sink))),
                        )
                    })
                    .collect();
                let mut oracle_plan = oracle_artifact
                    .into_bound(GraphRuntimeBindings {
                        envelope: oracle_envelope,
                        nodes: oracle_nodes,
                        observers: oracle_observers,
                    })
                    .unwrap_or_else(|_| panic!("oracle bind"))
                    .plan;
                let mut oracle_pcm = vec![0.0_f32; frames * 2];
                for block in 0..4_u64 {
                    for sink in &sinks {
                        sink.lock().expect("oracle sink").clear();
                    }
                    oracle_plan
                        .render(
                            RenderIo {
                                input: None,
                                output: PlanarBufferMut::try_new(
                                    &mut oracle_pcm,
                                    2,
                                    frames,
                                    frames,
                                )
                                .expect("oracle output"),
                            },
                            RenderTime {
                                absolute_sample: block * frames as u64,
                            },
                        )
                        .expect("oracle render");
                    let taps: Vec<Vec<(u32, u32)>> = sinks
                        .iter()
                        .map(|sink| sink.lock().expect("oracle sink").clone())
                        .collect();
                    for frame in 0..frames {
                        let routed: Vec<(f32, f32)> = contributions
                            .iter()
                            .zip(taps.iter())
                            .map(|((route, _), tap)| {
                                let coefficients = routes[route];
                                let left = f32::from_bits(tap[frame].0);
                                let right = f32::from_bits(tap[frame].1);
                                (
                                    lane::softfma::unfused_multiply_add_via_f64(
                                        coefficients[1],
                                        right,
                                        coefficients[0] * left,
                                    ),
                                    lane::softfma::unfused_multiply_add_via_f64(
                                        coefficients[3],
                                        right,
                                        coefficients[2] * left,
                                    ),
                                )
                            })
                            .collect();
                        let left = routed
                            .iter()
                            .map(|pair| pair.0)
                            .reduce(|a, b| a + b)
                            .unwrap_or(0.0);
                        let right = routed
                            .iter()
                            .map(|pair| pair.1)
                            .reduce(|a, b| a + b)
                            .unwrap_or(0.0);
                        assert_eq!(
                            (
                                oracle_pcm[frame].to_bits(),
                                oracle_pcm[frames + frame].to_bits()
                            ),
                            (left.to_bits(), right.to_bits()),
                            "block {block} frame {frame}: folded-route + D9 reduction oracle"
                        );
                    }
                }
            }

            let audit_envelope = audit_artifact.envelope();
            let audit_nodes = audit_artifact
                .external_binding_nodes()
                .map(|node| GraphNodeBinding::new(node.clone(), asymmetric_input_binding(node)))
                .collect();
            let bound = audit_artifact
                .into_bound(GraphRuntimeBindings {
                    envelope: audit_envelope,
                    nodes: audit_nodes,
                    observers: Vec::new(),
                })
                .unwrap_or_else(|_| panic!("audit bind"));
            let mut audit_plan = bound.plan;
            let mut audit_pcm = vec![0.0_f32; frames * 2];
            let output_address = audit_pcm.as_ptr() as usize;
            audit::warm_up();
            audit::reset();
            let mut output_hash = 0xcbf2_9ce4_8422_2325_u64;
            for block in 0..100_000_u64 {
                audit_plan
                    .render(
                        RenderIo {
                            input: None,
                            output: PlanarBufferMut::try_new(&mut audit_pcm, 2, frames, frames)
                                .expect("audit output"),
                        },
                        RenderTime {
                            absolute_sample: block * frames as u64,
                        },
                    )
                    .expect("audit render");
                assert_eq!(audit_pcm.as_ptr() as usize, output_address);
                for sample in &audit_pcm {
                    output_hash ^= u64::from(sample.to_bits());
                    output_hash = output_hash.wrapping_mul(0x0000_0100_0000_01b3);
                }
            }
            assert!(
                !audit::is_render_scope_active(),
                "audit is disarmed before snapshots and qualification counters"
            );
            let audit_snapshot = audit::snapshot();
            assert_eq!(audit_snapshot.total(), 0);
            let counters = audit_plan.qualification_counters();
            assert_eq!(
                counters[0],
                100_000_u64 * expected_builtin_banks as u64,
                "exact retained builtin-bank process callbacks"
            );
            assert_eq!(
                counters[1],
                counters[0] * u64::from(audit_envelope.quantum.0),
                "exact frames processed by the retained builtin banks"
            );
            // Re-pinned by #98 F2/F4 (master plan #83 D9/D3, section-8 policy):
            // 0x2fd8_5286_518f_d13b -> 0x5b3e_672a_ae5d_97aa. The output's twelve route inputs
            // are now folded left to right instead of as a balanced pairwise tree, and each
            // route spends two multiplies and one add with the gain folded in at
            // bind. Neither value is pinned from production output: the oracle block above
            // re-derives the expected PCM for this exact session from the recorded per-track
            // post-matrix contributions, re-applying both frozen op orders with scalar
            // `softfma::unfused_multiply_add_via_f64` and `reduce`, and asserts it bit for bit before this
            // literal is compared. (The previous re-pin note stands: 0x9f30_db02_2065_6d79 was already
            // stale on `origin/main` before either branch existed.)
            assert_eq!(
                output_hash, 0x5b3e_672a_ae5d_97aa,
                "deterministic mixed output hash"
            );
        }
    }

    /// Where a [`BitRecorder`] writes: one `(left bits, right bits)` pair per rendered frame.
    type BitSink = Arc<std::sync::Mutex<Vec<(u32, u32)>>>;

    /// Records every rendered sample of one observed node, bit for bit.
    struct BitRecorder(BitSink);
    impl GraphRuntimeObserver for BitRecorder {
        fn observe(
            &mut self,
            block: GraphObservationBlock<'_>,
        ) -> Result<(), engine::realtime::RenderError> {
            let mut sink = self.0.lock().expect("observer sink");
            for (left, right) in block.left.iter().zip(block.right.iter()) {
                sink.push((left.to_bits(), right.to_bits()));
            }
            Ok(())
        }
    }

    /// G3 + G5. Adding a ninth track moves the cohort boundary (eight lanes are full, the ninth
    /// track becomes a padded, unbound group) without changing one bit of the eight tracks already
    /// in the bank, and the chain performs exactly one planar/AoSoA round-trip per block.
    #[test]
    fn add_a_track_keeps_existing_track_bits_and_one_transpose_per_chain() {
        const BLOCKS: u64 = 32;
        let nine = parse_session_json(PARAMETRIC_EQ_NINE_TRACK_FIXTURE)
            .expect("accepted parametric-EQ fixture");
        let mut eight = nine.clone();
        eight.tracks.retain(|track| track.id.as_str() != "eq8");
        eight.routes.retain(|route| {
            !matches!(
                &route.source,
                RouteSource::Track { track_id, .. } if track_id.as_str() == "eq8"
            )
        });

        let mut observed = Vec::new();
        for model in [&eight, &nine] {
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
            .expect("compiled cohort-boundary fixture");
            let registry = launch_native_effect_registry().expect("launch registry");
            let effects = prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 20,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("prepared cohort-boundary effects");
            let artifact = GraphCompiler::compile(GraphCompileRequest {
                dispatch: host_dispatch(),
                plan_id: 1_096,
                effects,
                caps: integration_caps(),
            })
            .unwrap_or_else(|failure| panic!("cohort-boundary graph: {:?}", failure.diagnostics));
            let bank_count =
                artifact.graph.prepared_bank_count() + artifact.graph.prepared_builtin_bank_count();
            let PreparedGraphArtifact {
                graph,
                report: _,
                pool_classes: _,
            } = artifact;
            let envelope = graph.envelope;
            let frames = envelope.quantum.0 as usize;
            let nodes = graph
                .required_bindings
                .iter()
                .cloned()
                .map(|node| {
                    let processor = parametric_eq_input_binding(&node);
                    GraphNodeBinding::new(node, processor)
                })
                .collect();
            let sinks: Vec<_> = (0..8)
                .map(|_| Arc::new(std::sync::Mutex::new(Vec::new())))
                .collect();
            let observers = sinks
                .iter()
                .enumerate()
                .map(|(index, sink)| {
                    GraphNodeObserverBinding::new(
                        track_node(&format!("eq{index}"), TrackStage::PostSimd1),
                        index as u64,
                        Box::new(BitRecorder(Arc::clone(sink))),
                    )
                })
                .collect();
            let mut plan = graph
                .bind(GraphRuntimeBindings {
                    envelope,
                    nodes,
                    observers,
                })
                .unwrap_or_else(|failure| panic!("cohort-boundary bind: {}", failure.code));
            let mut pcm = vec![0.0_f32; frames * 2];
            for block in 0..BLOCKS {
                plan.render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                            .expect("cohort-boundary output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("cohort-boundary render");
            }
            // G5: master plan §4.5 -- exactly one transpose per bank **chain** per block.
            //
            // Issue #181 strengthened this. `bank_count` is a count of bound *slots*, and while
            // every chain carried exactly one slot the two readings of G5 were the same number:
            // a runtime that had regressed to one chain per slot passed this gate unchanged.
            // `bank_shape` reports `[chains, slots]` separately, so the law is now asserted
            // against the chain count and the slot count is checked as the *other* quantity it
            // used to be confused with.
            let [chains, slots] = plan.bank_shape();
            assert_eq!(
                plan.bank_transposes(),
                BLOCKS * chains,
                "one planar/AoSoA round-trip per chain per block"
            );
            assert_eq!(
                slots, bank_count as u64,
                "every bound bank is a slot of exactly one realised chain"
            );
            assert!(
                chains <= slots,
                "a chain carries at least one slot, so chains can never exceed slots"
            );
            // This fixture is a one-slot cohort, so here the two coincide -- and saying so is the
            // point: the gate now records *which* reading it checked instead of relying on them
            // being indistinguishable. `intended_placement_merges_two_chains_into_one_bit_\
            // identically` is the counterpart where they differ.
            assert_eq!(
                chains, slots,
                "the cohort-boundary fixture binds one slot per chain"
            );
            assert!(bank_count > 0, "the eight-lane cohort must actually bank");
            observed.push(
                sinks
                    .iter()
                    .map(|sink| sink.lock().expect("observer sink").clone())
                    .collect::<Vec<_>>(),
            );
        }

        // G3: the ninth track changes the cohort boundary, not the bits of its eight neighbours.
        for (track, (eight_track, nine_track)) in
            observed[0].iter().zip(observed[1].iter()).enumerate()
        {
            assert_eq!(
                eight_track.len(),
                BLOCKS as usize * 128,
                "every block was observed"
            );
            assert_eq!(
                eight_track, nine_track,
                "eq{track} bits changed when a ninth track was added"
            );
        }
    }

    #[test]
    fn launch_parametric_eq_fixture_retains_banks_and_matches_scalar_across_blocks() {
        let model = parse_session_json(PARAMETRIC_EQ_NINE_TRACK_FIXTURE)
            .expect("accepted parametric-EQ fixture");
        assert_eq!(model.tracks.len(), 9);
        let first_effect = &model.tracks[0].simd1.effects[0];
        assert!(first_effect.params.iter().any(|parameter| {
            parameter.parameter_id == 3
                && parameter.channel == ParameterChannel::Left
                && parameter.value == 120.0
        }));
        assert!(first_effect.params.iter().any(|parameter| {
            parameter.parameter_id == 3
                && parameter.channel == ParameterChannel::Right
                && parameter.value == 2400.0
        }));
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled parametric-EQ fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let scalar_registry = NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory {
            delegate: registry
                .get_shared_ascii("miso.parametric-eq")
                .expect("registered launch parametric EQ"),
        })
            as Box<dyn NativeEffectFactory>])
        .expect("scalar launch registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let bank_effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared bank-capable effects");
        let scalar_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared scalar effects");
        let bank_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_042,
            effects: bank_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bank graph: {:?}", failure.diagnostics));
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_043,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar graph: {:?}", failure.diagnostics));

        let width = BankWidth::for_backend(bank_artifact.report.rack_cohorts.dispatch);
        let (expected_banks, expected_scalar_tails) = width.map_or((0, 9), |width| {
            let lanes = width.lanes() as usize;
            (9 / lanes, 9 % lanes)
        });
        assert_eq!(bank_artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(
            bank_artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            bank_artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .len(),
            expected_scalar_tails
        );
        assert_eq!(scalar_artifact.graph.prepared_bank_count(), 0);
        // #96: the report is the *bound* plan. Cohort planning is still independent of the
        // factory's legal scalar fallback -- the planned groups are identical -- but a group the
        // factory declined is now reported as unbound, so its members show up in the scalar set
        // instead of being invisible there.
        assert_eq!(
            scalar_artifact.report.rack_cohorts.plan.groups,
            bank_artifact.report.rack_cohorts.plan.groups,
            "cohort planning is independent of the factory's legal scalar fallback"
        );
        assert!(scalar_artifact.report.rack_cohorts.bound_slots.is_empty());
        assert_eq!(
            scalar_artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .len(),
            9,
            "a declined bind puts every member on the per-node scalar path"
        );
        assert_eq!(
            bank_artifact.graph.sequential_schedule,
            scalar_artifact.graph.sequential_schedule
        );
        assert_eq!(
            bank_artifact.graph.route_timings,
            scalar_artifact.graph.route_timings
        );
        assert_eq!(
            bank_artifact.graph.inserted_delays,
            scalar_artifact.graph.inserted_delays
        );
        let expected_schedule = bank_artifact.graph.sequential_schedule.clone();
        let expected_route_timings = bank_artifact.graph.route_timings.clone();

        let PreparedGraphArtifact {
            pool_classes: _,
            graph: bank_graph,
            report: _,
        } = bank_artifact;
        let envelope = bank_graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let bank_nodes = bank_graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor = parametric_eq_input_binding(&node);
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let observer_order = Arc::new(AtomicU64::new(0));
        let observed_post_bank_audio = Arc::new(AtomicBool::new(false));
        let mut bank_plan = bank_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bank_nodes,
                // Deliberately reverse insertion order: binding handles, not insertion order,
                // decide the stable observer schedule after the SIMD rack boundary.
                observers: vec![
                    GraphNodeObserverBinding::new(
                        track_node("eq0", TrackStage::PostSimd1),
                        2,
                        Box::new(RepeatedOrderedPostBankObserver {
                            expected_order: 1,
                            order: Arc::clone(&observer_order),
                            observed_post_bank_audio: Arc::clone(&observed_post_bank_audio),
                        }),
                    ),
                    GraphNodeObserverBinding::new(
                        track_node("eq0", TrackStage::PostSimd1),
                        1,
                        Box::new(RepeatedOrderedPostBankObserver {
                            expected_order: 0,
                            order: Arc::clone(&observer_order),
                            observed_post_bank_audio: Arc::clone(&observed_post_bank_audio),
                        }),
                    ),
                ],
            })
            .unwrap_or_else(|failure| panic!("bank graph bind: {}", failure.code));
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: scalar_graph,
            report: _,
        } = scalar_artifact;
        let scalar_nodes = scalar_graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor = parametric_eq_input_binding(&node);
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("scalar graph bind: {}", failure.code));
        let mut bank_blocks = Vec::new();
        for block in 0..2_u64 {
            let mut bank_pcm = vec![0.0_f32; frames * 2];
            let mut scalar_pcm = vec![0.0_f32; frames * 2];
            bank_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut bank_pcm, 2, frames, frames)
                            .expect("bank output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("bank render");
            scalar_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                            .expect("scalar output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("scalar render");
            for (sample, scalar_sample) in bank_pcm.iter().zip(&scalar_pcm) {
                assert_eq!(
                    sample.to_bits(),
                    scalar_sample.to_bits(),
                    "bank/scalar PCM must be exact"
                );
            }
            bank_blocks.push(bank_pcm);
        }
        assert!(
            bank_blocks[0]
                .iter()
                .zip(&bank_blocks[1])
                .any(|(first, second)| first.to_bits() != second.to_bits()),
            "the second block must retain the first block's EQ state"
        );
        assert_eq!(observer_order.load(Ordering::SeqCst), 4);
        assert!(observed_post_bank_audio.load(Ordering::SeqCst));

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.simd1.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled bypass fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass effects");
        let bypass_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_044,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bypass graph: {:?}", failure.diagnostics));
        assert_eq!(
            bypass_artifact.graph.sequential_schedule, expected_schedule,
            "bypass does not change graph scheduling"
        );
        assert_eq!(
            bypass_artifact.graph.route_timings, expected_route_timings,
            "bypass does not change PDC timings"
        );
        let bypass_graph = bypass_artifact.graph;
        let bypass_nodes = bypass_graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor = parametric_eq_input_binding(&node);
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let mut bypass_plan = bypass_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bypass_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("bypass graph bind: {}", failure.code));
        let mut bypass_pcm = vec![0.0_f32; frames * 2];
        bypass_plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut bypass_pcm, 2, frames, frames)
                        .expect("bypass output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("bypass render");
        assert_eq!(bypass_pcm[0].to_bits(), 1.40625_f32.to_bits());
        assert_eq!(bypass_pcm[frames].to_bits(), (-0.703125_f32).to_bits());
        assert!(
            bypass_pcm
                .iter()
                .enumerate()
                .all(|(index, sample)| index == 0 || index == frames || *sample == 0.0),
            "bypass retains the dry impulse without changing the rack graph"
        );
    }

    #[test]
    fn launch_compressor_fixture_retains_bank_tail_and_connected_scalar_without_pdc_change() {
        let model = accepted_compressor_graph_fixture();
        assert_eq!(model.tracks.len(), 10);
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("accepted compressor fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let compressor = registry
            .get_shared_ascii("miso.compressor")
            .expect("registered compressor");
        let scalar_registry = NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory {
            delegate: compressor,
        })
            as Box<dyn NativeEffectFactory>])
        .expect("scalar compressor registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared compressor effects");
        assert_eq!(effects.entries.len(), 10);
        assert!(
            effects
                .entries
                .iter()
                .all(|entry| entry.metadata.latency == LatencySamples(960))
        );
        let scalar_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared scalar compressor effects");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_013,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("compressor graph: {:?}", failure.diagnostics));
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_014,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar compressor graph: {:?}", failure.diagnostics));
        let width = BankWidth::for_backend(artifact.report.rack_cohorts.dispatch);
        if let Some(width) = width {
            let lanes = width.lanes() as usize;
            let expected_banks = 9 / lanes;
            let expected_scalar_tails = 1 + 9 % lanes;
            assert_eq!(artifact.graph.prepared_bank_count(), expected_banks);
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .bound_groups_in(RackLocation::Simd1)
                    .count(),
                expected_banks
            );
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .len(),
                expected_scalar_tails
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq8")
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq9")
            );
        } else {
            assert_eq!(artifact.graph.prepared_bank_count(), 0);
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .len(),
                10
            );
        }
        assert_eq!(
            artifact.graph.sequential_schedule,
            scalar_artifact.graph.sequential_schedule
        );
        assert_eq!(
            artifact.graph.route_timings,
            scalar_artifact.graph.route_timings
        );
        assert_eq!(
            artifact.graph.inserted_delays,
            scalar_artifact.graph.inserted_delays
        );
        let expected_schedule = artifact.graph.sequential_schedule.clone();
        let expected_route_timings = artifact.graph.route_timings.clone();

        let PreparedGraphArtifact {
            pool_classes: _,
            graph: bank_graph,
            report: _,
        } = artifact;
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: scalar_graph,
            report: _,
        } = scalar_artifact;
        let envelope = bank_graph.envelope;
        let bank_nodes = bank_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), parametric_eq_input_binding(node)))
            .collect();
        let scalar_nodes = scalar_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), parametric_eq_input_binding(node)))
            .collect();
        let mut bank_plan = bank_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("compressor bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("compressor scalar bind: {}", failure.code));
        let frames = envelope.quantum.0 as usize;
        let mut rendered_nonzero = false;
        for block in 0..16_u64 {
            let mut bank_pcm = vec![0.0_f32; frames * 2];
            let mut scalar_pcm = vec![0.0_f32; frames * 2];
            bank_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut bank_pcm, 2, frames, frames)
                            .expect("bank output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("bank render");
            scalar_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                            .expect("scalar output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("scalar render");
            assert_eq!(
                bank_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                scalar_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "retained compressor bank and scalar fallback render the same PCM"
            );
            rendered_nonzero |= bank_pcm.iter().any(|sample| *sample != 0.0);
        }
        assert!(
            rendered_nonzero,
            "the fixed-delay compressor path rendered after its latency"
        );

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.simd1.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("bypass compressor fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass compressor effects");
        assert!(
            bypass_effects
                .entries
                .iter()
                .all(|entry| entry.metadata.latency == LatencySamples(960))
        );
        let bypass_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_015,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bypass compressor graph: {:?}", failure.diagnostics));
        assert_eq!(bypass_artifact.graph.sequential_schedule, expected_schedule);
        assert_eq!(bypass_artifact.graph.route_timings, expected_route_timings);
    }

    /// Phase 1b: a native effect carrying the homogeneous-bank kernel contract banks in the
    /// **dynamic** rack, and every rendered sample is bit-identical to the per-node path.
    ///
    /// Before this, `rack_location` mapped `RackId::Dynamic` to `None`, so this exact session --
    /// ten identical sidechain-free compressors -- produced zero banks and ran scalar at width 1.
    /// Nothing about the compressor changed to make it bankable; only the candidacy gate did.
    ///
    /// The bar is class A. Banking changes lane *grouping*, not per-lane arithmetic:
    /// `PreparedCompressorBank<L>` runs the same coefficient and detector update per lane that the
    /// scalar instance runs, so a single differing bit would be a defect in the bank kernel or in
    /// the gather/scatter, never something to re-pin around. This renders sixteen blocks -- well
    /// past the compressor's 960-sample lookahead latency, so the comparison is over live
    /// compressed audio with retained detector state, not over a latency pad of zeros.
    #[test]
    fn dynamic_rack_compressors_bank_and_render_bit_identically_to_the_per_node_path() {
        let model = accepted_dynamic_rack_compressor_fixture();
        assert_eq!(model.tracks.len(), 10);
        assert!(model.tracks.iter().all(|track| {
            track.simd1.effects.is_empty()
                && track.simd2.effects.is_empty()
                && track.dynamic.effects.len() == 1
        }));
        let (bank, per_node) = compile_bank_and_per_node(&model, "miso.compressor", 1_610);

        // Structure: the dynamic rack is now a bank location, and it fills exactly as SIMD-1 does.
        // Nine tracks are bankable (`eq8` carries a routed sidechain); the tenth is the tail.
        let width = BankWidth::for_backend(bank.report.rack_cohorts.dispatch);
        if let Some(width) = width {
            let lanes = width.lanes() as usize;
            let cohorts = &bank.report.rack_cohorts;
            assert_eq!(bank.graph.prepared_bank_count(), 9 / lanes);
            assert_eq!(
                cohorts.bound_groups_in(RackLocation::Dynamic).count(),
                9 / lanes,
                "the dynamic rack binds full cohorts"
            );
            assert_eq!(
                cohorts.groups_in(RackLocation::Simd1).count(),
                0,
                "no compressor is left in SIMD-1"
            );
            let scalar = cohorts.scalar_in(RackLocation::Dynamic);
            assert_eq!(scalar.len(), 1 + 9 % lanes);
            assert!(
                scalar.iter().any(|id| id.track_id.as_str() == "eq8"),
                "a routed sidechain still blocks banking in the dynamic rack (#96 F9)"
            );
            assert!(scalar.iter().all(|id| id.rack == RackId::Dynamic));
        } else {
            assert_eq!(bank.graph.prepared_bank_count(), 0);
        }
        assert_eq!(per_node.graph.prepared_bank_count(), 0);
        assert!(per_node.report.rack_cohorts.bound_slots.is_empty());
        assert_eq!(
            per_node.report.rack_cohorts.plan.groups, bank.report.rack_cohorts.plan.groups,
            "cohort planning is independent of the factory's legal scalar fallback"
        );

        // Banking is an execution-layer decision: it must not move the graph.
        assert_eq!(
            bank.graph.sequential_schedule,
            per_node.graph.sequential_schedule
        );
        assert_eq!(bank.graph.route_timings, per_node.graph.route_timings);
        assert_eq!(bank.graph.inserted_delays, per_node.graph.inserted_delays);

        let banked = render_blocks(bank, 16);
        let scalar = render_blocks(per_node, 16);
        assert_pcm_bits_equal(&banked, &scalar, "dynamic-rack compressor bank vs per node");
        assert!(
            banked.iter().flatten().any(|sample| *sample != 0.0),
            "sixteen blocks must clear the compressor's lookahead latency"
        );
        assert!(
            banked[15]
                .iter()
                .zip(&banked[14])
                .any(|(a, b)| a.to_bits() != b.to_bits()),
            "the bank must carry detector state across blocks, not restart each one"
        );
    }

    /// Rack placement decides where an effect sits in the signal chain. It must never decide what
    /// the arithmetic produces -- and, after phase 1b, it no longer decides how wide that
    /// arithmetic is either.
    ///
    /// The same ten compressors are compiled twice, once placed in SIMD-1 and once in the dynamic
    /// rack. Both racks a compressor is not in are empty identity boundaries, so the two sessions
    /// are the same chain; both now bank the same number of lanes, and every rendered bit agrees.
    /// This is the gate that would catch a bank kernel that silently depended on rack identity.
    #[test]
    fn rack_placement_changes_the_bank_but_never_the_samples() {
        let simd1_model = accepted_compressor_graph_fixture();
        let dynamic_model = accepted_dynamic_rack_compressor_fixture();
        let (simd1, _) = compile_bank_and_per_node(&simd1_model, "miso.compressor", 1_620);
        let (dynamic, _) = compile_bank_and_per_node(&dynamic_model, "miso.compressor", 1_630);

        assert_eq!(
            simd1.graph.prepared_bank_count(),
            dynamic.graph.prepared_bank_count(),
            "the same session banks the same width wherever it is placed"
        );
        assert_eq!(
            simd1
                .report
                .rack_cohorts
                .bound_slots_in(RackLocation::Simd1)
                .count(),
            dynamic
                .report
                .rack_cohorts
                .bound_slots_in(RackLocation::Dynamic)
                .count(),
        );
        assert_eq!(
            simd1
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .len(),
            dynamic
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Dynamic)
                .len(),
            "the same tracks fall back, for the same reasons"
        );
        assert_eq!(
            simd1.report.output_latency, dynamic.report.output_latency,
            "PDC is a property of the chain, not of the rack the stage sits in"
        );

        // #96 state payloads are placement-independent *by construction*: a bank is built from
        // `PrepareEffectBankRequest`, which carries a backend, a width and one
        // `PrepareEffectRequest` per member -- and neither type has a rack in it. These pin the
        // observable consequence: the same session retains byte-for-byte the same bank state,
        // scratch and metadata under either placement, so a snapshot taken under one restores
        // under the other.
        assert_eq!(
            simd1.report.estimate.effect_bank_scratch_bytes,
            dynamic.report.estimate.effect_bank_scratch_bytes,
        );
        assert_eq!(
            simd1.report.estimate.effect_bank_runtime_buffer_bytes,
            dynamic.report.estimate.effect_bank_runtime_buffer_bytes,
        );
        assert_eq!(
            simd1.report.estimate.effect_bank_metadata_bytes,
            dynamic.report.estimate.effect_bank_metadata_bytes,
        );
        assert_eq!(
            simd1.report.estimate.declared_effect_bytes,
            dynamic.report.estimate.declared_effect_bytes,
            "identical declared state layout under either placement"
        );

        assert_pcm_bits_equal(
            &render_blocks(simd1, 16),
            &render_blocks(dynamic, 16),
            "SIMD-1 placement vs dynamic placement",
        );
    }

    /// A dynamic slot that differs from its bank-mates' falls back exactly as a SIMD slot does.
    ///
    /// One track's compressor becomes a gate/expander: a different `EffectProgramKey`, so a
    /// different cohort, so a pool of one that can never fill a group. It renders per node while
    /// its eight bank-mates bank -- the same subsequence/leader mechanism that already governs
    /// SIMD-rack heterogeneity, reached through the same code path. Nothing rack-specific decides
    /// this, which is the point.
    #[test]
    fn a_dynamic_slot_that_differs_from_its_bank_mates_falls_back_per_node() {
        let mut model = accepted_dynamic_rack_compressor_fixture();
        // `eq8` already falls back on its routed sidechain; make `eq7` fall back on its *program*.
        let odd = &mut model.tracks[7].dynamic.effects[0];
        odd.id = StableId::parse("gate-expander").expect("stable effect id");
        odd.identity = EffectIdentity::Native {
            effect_id: StableId::parse("miso.gate-expander").expect("gate/expander id"),
        };
        let bank = compile_bank_only(&model, 1_640);
        let cohorts = &bank.report.rack_cohorts;

        let Some(width) = BankWidth::for_backend(cohorts.dispatch) else {
            assert_eq!(bank.graph.prepared_bank_count(), 0);
            return;
        };
        let lanes = width.lanes() as usize;
        // Eight homogeneous compressors remain; the gate/expander and the sidechained compressor
        // are each alone in their cohort and bind nothing.
        assert_eq!(bank.graph.prepared_bank_count(), 8 / lanes);
        let scalar = cohorts.scalar_in(RackLocation::Dynamic);
        assert_eq!(scalar.len(), 2, "exactly the two odd tracks fall back");
        assert!(scalar.iter().any(|id| id.track_id.as_str() == "eq7"));
        assert!(scalar.iter().any(|id| id.track_id.as_str() == "eq8"));
        // A heterogeneous member is never quietly folded into a bank of the wrong program.
        for bound in cohorts.bound_slots_in(RackLocation::Dynamic) {
            assert!(
                bound
                    .members
                    .iter()
                    .all(|member| member.effect_id.as_str() == "compressor"),
                "a bound dynamic bank contains only its own program"
            );
        }
    }

    /// The level of one node in the compiled graph.
    fn level_of(artifact: &PreparedGraphArtifact, track: &str, effect: &str) -> u64 {
        let wanted = GraphNodeId::Effect(EffectNodeId {
            track_id: StableGraphId::parse(track).expect("track id"),
            rack: RackId::Dynamic,
            effect_id: StableGraphId::parse(effect).expect("effect id"),
        });
        artifact
            .graph
            .dependency_levels
            .iter()
            .find(|level| level.nodes.contains(&wanted))
            .unwrap_or_else(|| panic!("{track}/{effect} must be scheduled"))
            .level
    }

    /// A connected sidechain on a **non-first** chain slot lifts that slot past `level + k`, and
    /// the chain must fall back per node instead of failing the compile.
    ///
    /// This is the one edge that feeds a rack chain from outside its own path. A chain is a path,
    /// so slot `k` normally sits at `level + k`; `bind_rack_banks` asserts that arithmetic rather
    /// than assuming it. But `level` is read from the chain's *first* slot, so a sidechain on slot
    /// 0 lifts the whole chain uniformly and the arithmetic still holds -- which is why every
    /// pre-existing sidechain fixture (all single-slot, and one that forges
    /// `metadata.ports.sidechain` without an edge at all) leaves the fallback branch unreachable.
    ///
    /// Here `eq5` runs **two** dynamic compressors and the *second* takes its sidechain from
    /// `eq0`'s post-matrix tap -- the deepest tap there is, scheduled long after `eq5`'s first
    /// slot. That genuinely lifts slot 1, and the test asserts the lift explicitly so it can never
    /// go quietly vacuous: if a future scheduling change stops producing it, assertion (a) fails
    /// loudly rather than the test passing for the wrong reason.
    ///
    /// Opening the dynamic rack is what makes this matter. Sidechained compressors live in the
    /// dynamic rack, so before the fallback existed this session compiled to
    /// `graph.internal.invariant` -- a valid session rejected outright. Deleting the guard in
    /// `bind_rack_banks` turns this test red with exactly that diagnostic.
    #[test]
    fn a_sidechain_lifted_chain_slot_falls_back_instead_of_failing_the_compile() {
        let mut model = accepted_dynamic_rack_compressor_fixture();
        // `eq5` gets a second dynamic slot whose sidechain source is the deepest tap in the graph.
        let base = model.tracks[5].dynamic.effects[0].clone();
        let mut lifted = base.clone();
        lifted.id = StableId::parse("compressor-sc").expect("stable effect id");
        lifted.sidechain = SidechainDeclaration::Routed(Sidechain {
            source: RouteSource::Track {
                track_id: StableId::parse("eq0").expect("stable source id"),
                tap: SendTap::PostMatrix,
            },
            port_id: StableId::parse("sidechain-in").expect("stable sidechain port"),
        });
        model.tracks[5].dynamic.effects = vec![base, lifted];

        // (a) The compile succeeds -- this is the assertion the guard exists for.
        let artifact = compile_bank_only(&model, 1_660);

        // (b) And the lift is real: slot 1 sits strictly past `level(slot 0) + 1`, which is the
        // precondition the guard's branch tests. Without this the test could pass vacuously.
        let first = level_of(&artifact, "eq5", "compressor");
        let second = level_of(&artifact, "eq5", "compressor-sc");
        assert!(
            second > first + 1,
            "the sidechain must lift slot 1 past level + 1 (slot 0 at {first}, slot 1 at {second})"
        );
        // Descriptive, printed under `--nocapture`: the size of the lift the guard absorbs.
        println!(
            "eq5 dynamic chain: slot 0 at level {first}, slot 1 at level {second} \
             (lifted {} past the path arithmetic)",
            second - (first + 1)
        );

        // (c) The lifted chain renders per node, and the report says so for both its slots.
        let scalar = artifact
            .report
            .rack_cohorts
            .scalar_in(RackLocation::Dynamic);
        for effect in ["compressor", "compressor-sc"] {
            assert!(
                scalar
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq5" && id.effect_id.as_str() == effect),
                "eq5/{effect} must fall back per node"
            );
        }

        // (d) The lifted chain is isolated: every other track still banks, and no bound bank ever
        // contains one of its slots.
        if BankWidth::for_backend(artifact.report.rack_cohorts.dispatch).is_some() {
            assert!(
                artifact.graph.prepared_bank_count() > 0,
                "one awkward chain must not disband the rest of the rack"
            );
            for bound in artifact
                .report
                .rack_cohorts
                .bound_slots_in(RackLocation::Dynamic)
            {
                assert!(
                    bound.members.iter().all(|id| id.track_id.as_str() != "eq5"),
                    "a lifted chain is never a bank member"
                );
            }
        }
    }

    /// AGENTS.md's opacity boundary, gated structurally rather than incidentally.
    ///
    /// "Third-party core Wasm ... is permitted only in the dynamic rack: opaque per-instance Wasm
    /// breaks the known homogeneous/fused SIMD bank contract." Opening the dynamic rack to banking
    /// must not open it to *opaque* effects, and the reason must not be an accident of which rack
    /// the candidate loop happens to walk.
    ///
    /// Two independent gates, because either one alone would rot:
    ///
    /// 1. Identity: `banks_are_permitted` refuses every non-native identity, and it is consulted
    ///    for every rack, so a future rack gaining a location cannot re-open the boundary.
    /// 2. Reachability: a third-party effect in the dynamic rack does not survive preparation at
    ///    all (`effect.third_party.unavailable_at_launch`), so no `EffectPreparedEntry` -- and
    ///    therefore no bank member -- can exist for one today.
    ///
    /// The third gate is `launch_delay_fixture_closes_scalar_state_tail_pdc_and_transactional_caps`:
    /// a *native* dynamic-rack effect with no bank kernel forms full cohorts and still binds
    /// nothing. Together they pin that banking follows the kernel contract, not the rack.
    #[test]
    fn third_party_dynamic_effects_are_never_bank_candidates() {
        assert!(!crate::banks::banks_are_permitted(
            &EffectIdentity::ThirdPartyCid {
                cid: "bafy2bzaceexampleexampleexampleexampleexampleexampleexample".to_owned(),
            }
        ));
        assert!(crate::banks::banks_are_permitted(&EffectIdentity::Native {
            effect_id: StableId::parse("miso.compressor").expect("compressor id"),
        }));

        let mut model = accepted_dynamic_rack_compressor_fixture();
        model.tracks[0].dynamic.effects[0].identity = EffectIdentity::ThirdPartyCid {
            cid: "bafy2bzaceexampleexampleexampleexampleexampleexampleexample".to_owned(),
        };
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("a third-party CID is an accepted session, not a malformed one");
        let registry = launch_native_effect_registry().expect("launch registry");
        let Err(failure) = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        ) else {
            panic!("an opaque effect must not prepare at launch");
        };
        assert!(
            failure
                .0
                .iter()
                .any(|diagnostic| diagnostic.code == "effect.third_party.unavailable_at_launch"),
            "an opaque dynamic-rack effect never reaches bank candidacy: {:?}",
            failure.0
        );
    }

    fn console_track_input_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
        let GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::Input,
        } = node
        else {
            return Box::new(IdentityBinding);
        };
        let index = track_id
            .as_str()
            .strip_prefix("ch")
            .and_then(|value| value.parse::<u32>().ok())
            .expect("console fixture track id");
        Box::new(AsymmetricTrackImpulseBinding {
            left: 0.03125 * (index % 7 + 1) as f32,
            right: -0.015625 * (index % 5 + 1) as f32,
        })
    }

    /// Issue #169: the bank-window slot hold costs no arena.
    ///
    /// Colouring may not recycle a physical slot inside a bank's reordering window, so slots freed
    /// there are held until it closes. That could have cost buffers; on the sixty-four-track
    /// console fixture -- eight full eight-lane EQ banks and eight compressor banks, the floor
    /// pass's own workload -- it costs none, because holding a slot changes *which* slot an op
    /// gets rather than how many exist.
    ///
    /// Two assertions, and the first is the durable one: the banked plan's arena equals the arena
    /// of the same session compiled against a registry that refuses every bank, so banking is
    /// arena-neutral whatever lane width this host has. The literal pins the fixture's shape, so a
    /// colouring regression surfaces as a number rather than as a benchmark drifting.
    ///
    /// The rejected alternative in `program::lower` (dedicating every bank member) scored 257
    /// here: one extra buffer and one extra stereo block copy per block for each of the 64
    /// dynamic members whose consumer could no longer consume it in place.
    #[test]
    fn banking_a_dynamic_rack_costs_no_arena_buffers() {
        let model = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_FIXTURE).expect("console fixture");
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled console fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let per_node_registry =
            NativeEffectRegistry::new(["miso.parametric-eq", "miso.compressor"].map(|id| {
                Box::new(ScalarOnlyDelegateFactory {
                    delegate: registry
                        .get_shared_ascii(id)
                        .expect("registered launch effect"),
                }) as Box<dyn NativeEffectFactory>
            }))
            .expect("per-node registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let arena = |plan_id: u64, registry: &NativeEffectRegistry| {
            GraphCompiler::compile(GraphCompileRequest {
                dispatch: host_dispatch(),
                plan_id,
                effects: prepare_native_session_effects(&session, registry, effect_caps)
                    .expect("prepared console effects"),
                caps: integration_caps(),
            })
            .unwrap_or_else(|failure| panic!("console graph: {:?}", failure.diagnostics))
            .graph
            .program()
            .expect("lowers")
            .buffers
        };
        let banked = arena(1_690, &registry);
        let per_node = arena(1_691, &per_node_registry);
        assert_eq!(
            banked, per_node,
            "banking regrouped the lanes; it must not enlarge the arena"
        );
        assert_eq!(banked, 193);
    }

    /// The measured session: the 64-track console fixture the benchmark renders.
    ///
    /// Every track places `miso.parametric-eq` in SIMD-1 and `miso.compressor` in the **dynamic**
    /// rack, sidechain-free. Before phase 1b this compiled to 64/lanes EQ banks and *64 scalar
    /// compressors* -- the compressor ran a lane at a time purely because of its placement, which
    /// the profile measured as the majority of the block. It now compiles to the same number of
    /// compressor banks as EQ banks, with nothing left on the per-node path.
    ///
    /// The three things this pins, in the order they matter:
    ///
    /// 1. **Bits.** Rendered PCM is byte-identical to the same session compiled against a registry
    ///    that refuses every bank. Class A: banking regrouped the lanes, it did not change the
    ///    arithmetic.
    /// 2. **Structure.** Both racks bank fully; `scalar_in` is empty in both.
    /// 3. **G5** (master plan §4.5). One planar/AoSoA round-trip per bank per block, still exact
    ///    with the extra banks present. The pin is derived from the realised bank count, not a
    ///    literal, so the extra dynamic banks scale both sides of it -- there is no re-pin here.
    #[test]
    fn console_sixty_four_track_fixture_banks_its_dynamic_compressor_bit_identically() {
        // Past the compressor's 960-sample lookahead (8 blocks of 128), so the comparison is over
        // live compressed audio rather than over a latency pad of zeros.
        const BLOCKS: u64 = 12;
        let model = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_FIXTURE).expect("console fixture");
        assert_eq!(model.tracks.len(), 64);
        assert!(model.tracks.iter().all(|track| {
            track.simd1.effects.len() == 1
                && track.dynamic.effects.len() == 1
                && track.simd2.effects.is_empty()
                && matches!(
                    track.dynamic.effects[0].sidechain,
                    SidechainDeclaration::None
                )
        }));
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled console fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let per_node_registry =
            NativeEffectRegistry::new(["miso.parametric-eq", "miso.compressor"].map(|id| {
                Box::new(ScalarOnlyDelegateFactory {
                    delegate: registry
                        .get_shared_ascii(id)
                        .expect("registered launch effect"),
                }) as Box<dyn NativeEffectFactory>
            }))
            .expect("per-node registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let bank = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_650,
            effects: prepare_native_session_effects(&session, &registry, effect_caps)
                .expect("prepared console effects"),
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("console graph: {:?}", failure.diagnostics));
        let per_node = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_651,
            effects: prepare_native_session_effects(&session, &per_node_registry, effect_caps)
                .expect("prepared per-node console effects"),
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("per-node console graph: {:?}", failure.diagnostics));

        let width = BankWidth::for_backend(bank.report.rack_cohorts.dispatch);
        let Some(width) = width else {
            assert_eq!(bank.graph.prepared_bank_count(), 0);
            return;
        };
        let lanes = width.lanes() as usize;
        assert_eq!(64 % lanes, 0, "the fixture is a whole number of cohorts");
        let cohorts = &bank.report.rack_cohorts;
        assert_eq!(
            cohorts.bound_slots_in(RackLocation::Simd1).count(),
            64 / lanes,
            "the EQ banked before phase 1b and still does"
        );
        assert_eq!(
            cohorts.bound_slots_in(RackLocation::Dynamic).count(),
            64 / lanes,
            "and the dynamic-rack compressor now banks at the same width"
        );
        assert!(cohorts.scalar_in(RackLocation::Simd1).is_empty());
        assert!(
            cohorts.scalar_in(RackLocation::Dynamic).is_empty(),
            "no compressor is left on the per-node path"
        );
        assert_eq!(bank.graph.prepared_bank_count(), 2 * (64 / lanes));
        assert_eq!(per_node.graph.prepared_bank_count(), 0);
        assert_eq!(
            bank.graph.sequential_schedule, per_node.graph.sequential_schedule,
            "banking is an execution decision and must not move the graph"
        );
        assert_eq!(bank.report.output_latency, per_node.report.output_latency);

        let bank_count = bank.graph.prepared_bank_count();
        let builtin_bank_count = bank.graph.prepared_builtin_bank_count();
        let bound_slots = (bank_count + builtin_bank_count) as u64;
        // G5 derivation, issue #202 rec 2. This fixture binds `64 / lanes` EQ slots on `simd1` and
        // `64 / lanes` compressor slots in `dynamic`, and nothing else -- no builtin banks are
        // attached on this path. Lane `i` of the compressor bank reads lane `i` of the EQ bank
        // through the elided `PostSimd1` boundary, undelayed, unmixed, with no second reader and
        // no observer, so every cohort fuses into one chain:
        //
        //     chains = bound slots - one merge per cohort = 2 * (64 / lanes) - 64 / lanes
        //
        // Before rec 2 the merge was never proposed, because the cohort planner pools per
        // `RackLocation` and these two slots sit in different racks.
        let cohorts = 64 / lanes;
        let expected_chains = bound_slots - cohorts as u64;
        let banked = render_console_blocks(bank, BLOCKS);
        let scalar = render_console_blocks(per_node, BLOCKS);
        assert_pcm_bits_equal(&banked.0, &scalar.0, "64-track console: banked vs per node");
        assert!(
            banked.0.iter().flatten().any(|sample| *sample != 0.0),
            "the console fixture rendered audio"
        );
        assert_eq!(
            banked.3, bound_slots,
            "every bound bank is a slot of exactly one realised chain"
        );
        assert_eq!(
            banked.2, expected_chains,
            "the EQ and the dynamic compressor fuse into one chain per cohort"
        );
        // G5: one planar/AoSoA round-trip per bank *chain* per block.
        assert_eq!(
            banked.1,
            BLOCKS * expected_chains,
            "one planar/AoSoA round-trip per chain per block"
        );
        assert!(
            banked.2 < banked.3,
            "a cross-rack cohort must realise fewer chains than slots"
        );
        assert_eq!(scalar.1, 0, "the per-node arm transposes nothing");
        // Descriptive, printed under `--nocapture`: what phase 1b actually buys on the measured
        // session, and what the composed benchmark should expect to see.
        println!(
            "console 64-track @ {lanes} lanes: {} effect banks ({} EQ + {} compressor) \
             + {} builtin banks; {} scalar effect nodes; {} slots -> {} chains; {} transposes \
             over {BLOCKS} blocks",
            bank_count, cohorts, cohorts, builtin_bank_count, 0, banked.3, banked.2, banked.1,
        );
    }

    /// Issue #175: the intended production layout's chain structure and its G5 transpose count.
    ///
    /// The owner set the layout the product will ship: EQ and compressor as **one two-slot chain**
    /// on `simd1`, and a true-peak limiter alone on `simd2`. The retired fixture ran the same EQ
    /// and compressor as **two one-slot chains** (`simd1` + `dynamic`). This pins what that
    /// difference is, and -- just as importantly -- what it is not.
    ///
    /// 1. **Bits.** The two placements render byte-identically. Post-#166 bank eligibility follows
    ///    the effect's kernel contract and not the rack, and the strip order
    ///    `simd1 -> dynamic -> simd2` means appending the compressor to `simd1` and emptying
    ///    `dynamic` leaves the traversal order alone, so the merge regroups lanes without touching
    ///    any lane's arithmetic. The comparison is made against the *limiter-free* intended model,
    ///    because the limiter is genuinely new arithmetic and would mask the property under test.
    /// 2. **Structure.** The retired layout binds two bank *cohorts* per eight tracks (one per
    ///    rack); the merged layout binds one cohort of two slots. The slot *executions* are
    ///    unchanged -- the same two effects still run over the same lanes.
    /// 3. **G5** (master plan §4.5). One planar/AoSoA round-trip per bank chain per block. Since
    ///    issue #202 rec 2 both layouts realise **one chain per cohort**: the merge is proved on
    ///    the lowered program's dataflow rather than proposed from the cohort planner's per-rack
    ///    groups, and the EQ feeds the compressor across the elided `PostSimd1` boundary as
    ///    directly as it does inside one rack. So the round-trip counts are now equal, and the
    ///    equality is asserted for the same reason #181 asserted the inequality: if a rack-boundary
    ///    refusal ever comes back, this says so. Every count below is derived from the realised
    ///    bank count rather than written as a literal, so the assertion states the *law* and the
    ///    structural literals beside it state the shape.
    #[test]
    fn intended_placement_merges_two_chains_into_one_bit_identically() {
        const BLOCKS: u64 = 12;
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        assert_eq!(intended.tracks.len(), 64);
        assert!(
            intended.tracks.iter().all(|track| {
                track.simd1.effects.len() == 2
                    && track.dynamic.effects.is_empty()
                    && track.simd2.effects.len() == 1
            }),
            "the intended fixture is a two-slot simd1 chain, an empty dynamic rack and a \
             one-slot simd2 chain"
        );

        // The limiter-free intended model: the merged chain shape carrying exactly the retired
        // layout's arithmetic. This is the honest counterpart to the retired fixture.
        let mut merged = intended.clone();
        for track in &mut merged.tracks {
            track.simd2.effects.clear();
        }
        let split = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_FIXTURE).expect("retired fixture");

        let compile = compile_console_model;

        let split_artifact = compile(&split, 1_750);
        let merged_artifact = compile(&merged, 1_751);
        let intended_artifact = compile(&intended, 1_752);

        let Some(width) = BankWidth::for_backend(split_artifact.report.rack_cohorts.dispatch)
        else {
            assert_eq!(split_artifact.graph.prepared_bank_count(), 0);
            return;
        };
        let lanes = width.lanes() as usize;
        let cohorts_per_rack = 64 / lanes;

        // (2) Structure. The retired layout: one cohort in `simd1`, one in `dynamic`.
        let split_cohorts = &split_artifact.report.rack_cohorts;
        assert_eq!(
            split_cohorts.bound_slots_in(RackLocation::Simd1).count(),
            cohorts_per_rack
        );
        assert_eq!(
            split_cohorts.bound_slots_in(RackLocation::Dynamic).count(),
            cohorts_per_rack
        );
        assert_eq!(
            split_artifact.graph.prepared_bank_count(),
            2 * cohorts_per_rack
        );

        // The merged layout: both slots bound inside one `simd1` cohort, nothing in `dynamic`.
        let merged_cohorts = &merged_artifact.report.rack_cohorts;
        assert_eq!(
            merged_cohorts.bound_slots_in(RackLocation::Simd1).count(),
            2 * cohorts_per_rack,
            "both slots of the two-slot chain must bind"
        );
        assert_eq!(
            merged_cohorts.bound_slots_in(RackLocation::Dynamic).count(),
            0
        );
        assert!(merged_cohorts.scalar_in(RackLocation::Simd1).is_empty());
        // **The finding, and where it moved in #181.** Both layouts realise the *same number of
        // prepared banks*, because a prepared bank is a bound **slot**, not a chain: the retired
        // layout's sixteen are one EQ slot and one compressor slot per cohort in two racks, and
        // the merged layout's sixteen are two slots per cohort in one rack. The cohort planner
        // always did group them -- `bound_slots_in(Simd1)` doubles and `Dynamic` empties -- and
        // #175 measured that the grouping did not reach the runtime as one chain. It does now --
        // and since #202 rec 2 so does the retired layout's cross-rack pair, so the chain counts
        // asserted further down are equal as well. Same slots, same chains, different racks.
        assert_eq!(
            merged_artifact.graph.prepared_bank_count(),
            split_artifact.graph.prepared_bank_count(),
            "merging the racks regroups which cohort the slots belong to, not how many slots bind"
        );
        assert_eq!(
            intended_artifact.graph.prepared_bank_count(),
            3 * cohorts_per_rack,
            "the intended strip binds three slots per cohort: EQ, compressor and limiter"
        );

        // (1) Bits, and (3) G5. The slot counts are read before the artifacts are consumed, so
        // the G5 accounting below is derived from what this plan actually bound.
        let bound_slots_per_block = |artifact: &PreparedGraphArtifact| {
            artifact.graph.prepared_bank_count() + artifact.graph.prepared_builtin_bank_count()
        };
        let split_slots = bound_slots_per_block(&split_artifact);
        let merged_slots = bound_slots_per_block(&merged_artifact);
        assert_eq!(
            split_slots, merged_slots,
            "the merge regroups slots into chains; it does not change how many slots bind"
        );
        let split_render = render_console_blocks(split_artifact, BLOCKS);
        let merged_render = render_console_blocks(merged_artifact, BLOCKS);
        assert_pcm_bits_equal(
            &split_render.0,
            &merged_render.0,
            "64-track console: two one-slot chains vs one two-slot chain",
        );
        assert!(
            split_render.0.iter().flatten().any(|sample| *sample != 0.0),
            "the console fixture rendered audio"
        );

        // G5 holds on both sides, derived rather than pinned -- and since issue #181 the law can
        // finally tell its two readings apart. "One round-trip per bank *chain* per block" and
        // "one per bound *slot* per block" were the same number while every chain had one slot.
        //
        // **What issue #202 rec 2 moved here, and why it is not a weakening of #181.** #181
        // measured the retired layout at two chains per cohort and the merged one at one, and read
        // that gap as the value of moving the compressor into `simd1`. The gap was never the
        // compressor's placement. It was that `runtime::cohort_runs` offered only the cohort
        // planner's own groups as merge candidates, and `plan_bank_groups` pools per
        // `RackLocation` -- so `simd1 -> dynamic` could not even be proposed, however plainly
        // the EQ fed the compressor. Candidacy is now taken from the lowered program's dataflow,
        // and the EQ feeds the compressor across the elided `PostSimd1` boundary exactly as
        // directly as it does inside one rack. **Both layouts now realise one chain per cohort**,
        // so the saving #181 attributed to the rack move is taken wherever the compressor sits.
        //
        // What #181 established is untouched and still asserted below: a cohort chain carries more
        // slots than there are chains, and G5's per-chain reading is the one this gate checks.
        // What is deliberately no longer true is that the retired placement pays more for it.
        let chains = |render: &(Vec<Vec<f32>>, u64, u64, u64)| render.1 / BLOCKS;
        // The measured round-trip count and the realised structure must agree: `bank_shape`
        // counts the chains the runtime built, `bank_transposes` counts what they did.
        assert_eq!(
            chains(&split_render),
            split_render.2,
            "split: transposes per block == chains"
        );
        assert_eq!(
            chains(&merged_render),
            merged_render.2,
            "merged: transposes per block == chains"
        );
        assert_eq!(
            split_render.3, split_slots as u64,
            "split: realised slots == bound slots"
        );
        assert_eq!(
            merged_render.3, merged_slots as u64,
            "merged: realised slots == bound slots"
        );
        assert_eq!(split_render.1, BLOCKS * chains(&split_render));
        assert_eq!(merged_render.1, BLOCKS * chains(&merged_render));
        // Two bound slots per cohort on both sides, fused into one chain on both sides.
        // Derivation: 64 tracks / `lanes` = `cohorts_per_rack` cohorts; each binds an EQ slot and
        // a compressor slot (2 * cohorts_per_rack slots); the compressor's op is the sole reader
        // of the EQ's output, undelayed, unmixed and unobserved, so each cohort fuses to one
        // chain.
        assert_eq!(
            split_render.3,
            2 * cohorts_per_rack as u64,
            "the retired layout binds one EQ slot and one compressor slot per cohort"
        );
        assert_eq!(
            merged_render.3,
            2 * cohorts_per_rack as u64,
            "so does the merged layout; the racks differ, the slot count does not"
        );
        assert_eq!(
            chains(&split_render),
            cohorts_per_rack as u64,
            "the retired layout now fuses its two racks into one chain per cohort"
        );
        assert_eq!(
            chains(&merged_render),
            cohorts_per_rack as u64,
            "and the merged layout fuses its two slots into one chain per cohort"
        );
        assert!(
            chains(&merged_render) < merged_slots as u64,
            "G5 must distinguish a per-chain round-trip from a per-slot one; if these are equal \
             the merged side is still materialising one chain per bound slot"
        );
        assert!(
            chains(&split_render) < split_slots as u64,
            "and the retired side must no longer be materialising one chain per bound slot"
        );

        // **The measured answer to #175's hypothesis, taken in #181 and generalised in #202.**
        // #181 wrote this as a strict inequality so the day the graph layer took the saving it
        // would say so. #202 rec 2 takes the *same* saving on the retired layout, so the
        // difference is now exactly zero -- and that equality is the finding, written as an
        // assertion for the same reason: if a future change re-introduces a rack-boundary refusal
        // this goes red and names it.
        assert_eq!(
            split_render.1, merged_render.1,
            "which rack a slot was placed in no longer changes how many planar/AoSoA round-trips \
             the cohort pays"
        );
        println!(
            "#202 chain shape @ {lanes} lanes over {BLOCKS} blocks: \
             retired {} transposes, merged {} -- {} slots each, {} chains each",
            split_render.1, merged_render.1, merged_render.3, merged_render.2,
        );
    }

    /// A stage meter leased at an elided rack boundary declines the merge -- and still meters.
    ///
    /// This is the perf cliff `runtime::chains_into` buys, pinned from both sides. The three
    /// internal rack boundaries (`PostSimd1`, `PostDynamic`, `PostSimd2PreFader`) are elided into
    /// buffer aliases, and `builtins::MeterTap` admits all three, so a host may lease a
    /// meter that reads one. A merged chain leaves the aliased buffer holding the *chain's input*
    /// rather than the compressor's output, so such a meter must stop the merge.
    ///
    /// It is `parts.observers` keyed on the **alias node** that stops it. Keying the check on the
    /// producing node instead -- the compressor -- would miss this entirely: nobody observes the
    /// compressor, the observer is bound to `ch00/PostSimd1`. That mutation is the one this test
    /// exists to make red, and no digest comparison would catch it, because the merged plan would
    /// still render the correct session output and only the meter would read pre-compressor audio.
    ///
    /// The cost is stated rather than hidden: one leased stage meter costs *that track's cohort*
    /// one extra planar/AoSoA round-trip per block, and no other cohort anything.
    #[test]
    fn a_leased_stage_meter_declines_the_merge_and_still_meters() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let cohorts = 64 / width.lanes() as u64;
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let meters = vec![MeterRequest {
            handle: MeterHandle(NonZeroU64::new(1).expect("constant")),
            track_id: "ch00".to_owned(),
            tap: MeterTap::PostSimd1,
            config: MeterConfig {
                period_frames: NonZeroU32::new(128).expect("constant"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: NonZeroUsize::new(64).expect("constant"),
                reset_generation: 0,
            },
        }];
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&intended, 2_030, &meters, &registry);
        let (pcm, transposes, chains, slots, frames, redirects, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            slots,
            STRIP_SLOTS_PER_COHORT * cohorts,
            "the meter changes no bank's membership"
        );
        assert_eq!(
            chains,
            cohorts + 1,
            "ch00's cohort splits at the metered boundary; every other cohort stays one chain"
        );
        assert_eq!(transposes, BLOCKS * chains, "G5 on the declining plan");

        // And the meter reads what it is supposed to read. The oracle is the same session with
        // every effect on the per-node scalar path: no bank binds there, so no merge is even
        // expressible, and the meter can only be reading the compressor's output.
        let scalar_artifact = compile_console_model_with_builtins(
            &intended,
            2_031,
            &meters,
            &scalar_console_registry(),
        );
        assert_eq!(
            scalar_artifact.graph().prepared_bank_count(),
            0,
            "the oracle arm must bind no effect bank at all"
        );
        let (scalar_pcm, _, scalar_chains, _, scalar_frames, scalar_redirects, _) =
            render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        // Two chains per cohort on the oracle arm, not one. The three bankable track stages still
        // bind their banks -- only the *effects* are on the scalar path here -- and the fader bank
        // chains into the matrix bank because nothing planar reads between them. The post-input
        // bank cannot join them: its successor is a per-node EQ op, which is not a bank slot.
        const ORACLE_CHAINS_PER_COHORT: u64 = 2;
        assert_eq!(
            scalar_chains,
            ORACLE_CHAINS_PER_COHORT * cohorts,
            "the oracle arm realises its builtin banks and fuses the fader into the matrix"
        );
        assert_pcm_bits_equal(
            &pcm,
            &scalar_pcm,
            "64-track intended strip with a stage meter",
        );
        assert!(
            !frames.is_empty(),
            "the leased meter must publish windows, or this test proves nothing about it"
        );
        assert_eq!(
            frames, scalar_frames,
            "the metered boundary must read the compressor's output, not the chain's input"
        );
        assert_eq!(
            redirects, 0,
            "a meter at `PostSimd1` is upstream of the limiter: it declines the chain merge and \
             leaves the scatter accounting at the far end of the strip alone"
        );
        assert!(
            scalar_redirects > 0,
            "the bank-free arm still redirects its builtin banks' scatters"
        );
        assert!(
            frames
                .iter()
                .any(|frame| frame.left.sample_peak != 0.0 || frame.right.sample_peak != 0.0),
            "the metered windows must carry signal"
        );
        println!(
            "#202 leased stage meter: {slots} slots -> {chains} chains \
             ({} published windows over {BLOCKS} blocks), {redirects} scatter redirects",
            frames.len()
        );
    }

    /// A send taken from an elided rack boundary declines the merge, through the second-reader
    /// clause rather than the observer one.
    ///
    /// Issue #181 declined on the *presence* of a `program::Tap`, which made this case and the
    /// metered one indistinguishable. They are not the same: a tap is a name, and an edge out of
    /// the aliased stage resolves through it to the compressor, so it is counted by
    /// `runtime::op_dataflow` as an ordinary second reader of the compressor's output. That clause
    /// is what refuses here, and it has to, because a merged chain would send pre-compressor audio.
    #[test]
    fn a_send_from_a_rack_boundary_declines_the_merge() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let cohorts = 64 / width.lanes() as u64;
        let mut sent = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let template = sent.routes[0].clone();
        let mut route = template.clone();
        route.id = StableId::parse("ch00-simd1-send").expect("route id");
        route.source = RouteSource::Track {
            track_id: StableId::parse("ch00").expect("track id"),
            tap: session::SendTap::PostSimd1,
        };
        sent.routes.push(route);
        sent.routes.sort_by(|left, right| left.id.cmp(&right.id));

        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&sent, 2_040, &[], &registry);
        let (pcm, transposes, chains, slots, _, redirects, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            slots,
            STRIP_SLOTS_PER_COHORT * cohorts,
            "the send changes no bank's membership"
        );
        assert_eq!(
            chains,
            cohorts + 1,
            "ch00's cohort splits at the sent boundary; every other cohort stays one chain"
        );
        assert_eq!(transposes, BLOCKS * chains, "G5 on the declining plan");
        let scalar_artifact =
            compile_console_model_with_builtins(&sent, 2_041, &[], &scalar_console_registry());
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with a post-simd1 send");
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the sent strip rendered audio"
        );
        // The redirect count is zero for every lane on this fixture since issue #212 -- see
        // `the_intended_strip_fuses_the_whole_signal_path_into_one_chain_per_cohort` for why the
        // strip's chain now ends in a buffer its consumer already reads in place. What this test
        // still says is that a send taken from `PostSimd1` changes nothing at the *far end* of the
        // strip: it splits ch00's cohort at the observed boundary, and the scatter accounting on
        // every lane, ch00's included, is exactly what it is without the send.
        assert_eq!(
            redirects, 0,
            "a send taken from `PostSimd1` is upstream of the limiter, so it declines the chain \
             merge without touching the scatter accounting at the far end of the strip"
        );
    }

    /// Mono-collapse M1: the two bank planners classify every track alike, and the proof is the
    /// thing the merge actually needs -- their banks cover the same lanes in the same order.
    ///
    /// # Why this and not a map comparison
    ///
    /// `SessionPoolClasses` makes the two planners read one value, so comparing the map against
    /// itself would prove nothing. What can still go wrong is the *consequence*: a class that
    /// partitioned the strip-stage pools differently from the rack-chain pools would leave bank
    /// `{ch00, ch02, ...}` feeding bank `{ch00, ch01, ...}`, `runtime::chains_into` would decline
    /// every `builtins -> EQ -> compressor -> limiter` merge lane by lane, and the plan would
    /// render **byte-identical audio** while paying one planar/AoSoA round-trip per stage instead
    /// of one per cohort. That is the failure F6 named and the only kind a digest cannot see, so
    /// this asserts the lane sets and the chain count, not the map.
    ///
    /// Red mutation: derive the effect-chain candidate's class from anything but
    /// `classes.class_of(track)` -- for instance hard-code `CohortPoolClass::Stereo` in
    /// `bind_rack_banks` -- and the strip's `chains` jumps from one per cohort to one per stage
    /// per cohort while every digest here stays green.
    #[test]
    fn the_two_planners_agree_on_every_track_class() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let lanes = width.lanes() as usize;
        let mut model =
            parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_MONO_FIXTURE).expect("mono fixture");
        // The half-mono shape: alternate tracks get the standing fixture's stereo mapping back, so
        // every `lanes`-wide cohort would be half and half without the pool class.
        for (index, track) in model.tracks.iter_mut().enumerate() {
            if !index.is_multiple_of(2) {
                track.right_source_channel = 1;
            }
        }
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&model, 2_070, &[], &registry);

        // Every builtin bank is class-homogeneous, which is the strip-stage planner's half.
        let mono = |track: &str| {
            track
                .strip_prefix("ch")
                .and_then(|index| index.parse::<usize>().ok())
                .expect("fixture track id")
                .is_multiple_of(2)
        };
        let mut builtin_lane_sets: Vec<Vec<String>> = Vec::new();
        for bank in artifact.prepared_builtin_banks() {
            let tracks: Vec<String> = bank
                .members
                .iter()
                .map(|node| match node {
                    GraphNodeId::TrackStage { track_id, .. } => track_id.as_str().to_owned(),
                    other => panic!("a builtin bank named {other:?}"),
                })
                .collect();
            assert!(
                tracks.iter().all(|track| mono(track)) || tracks.iter().all(|track| !mono(track)),
                "a builtin bank mixed the two pool classes: {tracks:?}"
            );
            builtin_lane_sets.push(tracks);
        }
        assert_eq!(
            builtin_lane_sets.len(),
            3 * (64 / lanes),
            "three strip stages"
        );

        // And every effect bank covers one of those lane sets, in the same lane order. This is the
        // agreement in the only form the merge can use.
        let mut effect_lane_sets: Vec<Vec<String>> = Vec::new();
        for bank in artifact.graph().effect_bank_members() {
            let tracks: Vec<String> = bank
                .iter()
                .map(|member| member.track_id.as_str().to_owned())
                .collect();
            assert!(
                tracks.iter().all(|track| mono(track)) || tracks.iter().all(|track| !mono(track)),
                "an effect bank mixed the two pool classes: {tracks:?}"
            );
            assert!(
                builtin_lane_sets.contains(&tracks),
                "an effect bank's lane set matches no strip-stage bank's: {tracks:?}"
            );
            effect_lane_sets.push(tracks);
        }
        assert_eq!(
            effect_lane_sets.len(),
            3 * (64 / lanes),
            "eq, comp, limiter"
        );

        // The consequence, counted: the whole strip is still one chain per cohort.
        let (pcm, transposes, chains, slots, _, _, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            chains,
            (64 / lanes) as u64,
            "one chain per cohort, on a session whose cohorts are pooled by class"
        );
        assert_eq!(slots, 6 * chains, "six slots per cohort");
        assert_eq!(transposes, BLOCKS * chains, "G5 holds under class pooling");
        assert!(pcm.iter().flatten().any(|sample| *sample != 0.0));
    }

    /// Mono-collapse M1: what class pooling costs, and on which sessions -- the effect banks.
    ///
    /// # The finding, measured rather than argued
    ///
    /// An effect bank binds only when its group is **full**: every launch effect factory refuses
    /// `requests.len() != lanes` (#96 F7), so a group of fewer than `lanes` members renders on the
    /// per-node scalar path. Pooling by class therefore has a remainder cost that pooling by rack
    /// and level did not: a class whose pool is not a multiple of the lane width strands its tail,
    /// and *both* classes now have a tail where one cohort had none.
    ///
    /// One odd track out of 64 is the worst realistic case and it is what this measures: 63 tracks
    /// in one pool bind seven full banks and strand seven, and the lone track in the other pool
    /// strands too -- eight tracks' worth of effect slots lost out of 64, one cohort in eight.
    ///
    /// It is a **forfeited optimisation and not a wrong render**: the digest is asserted equal to
    /// the unsplit session's, because the split changes which tracks bank and never what a lane
    /// computes.
    ///
    /// This is the number a ruling on the class predicate has to be made against. The predicate
    /// M1 was briefed with is `SOURCE && DESIGNED`, both prepare-time terms; narrowing it to
    /// `SOURCE` alone would make this case cost nothing (a polarity flip would no longer split a
    /// pool) at the price of pooling some tracks as mono that will decline at dispatch. Neither is
    /// unsound; the choice is a measurement, and this is the measurement.
    #[test]
    fn a_single_odd_track_strands_both_pools_remainders() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let lanes = width.lanes() as usize;
        let registry = launch_native_effect_registry().expect("launch registry");

        let uniform =
            parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_MONO_FIXTURE).expect("mono fixture");
        let mut odd = uniform.clone();
        odd.tracks[7].right_source_channel = 1;

        let unsplit = compile_console_model_with_builtins(&uniform, 2_074, &[], &registry);
        let split = compile_console_model_with_builtins(&odd, 2_075, &[], &registry);
        let unsplit_slots = unsplit.graph().prepared_bank_count();
        let split_slots = split.graph().prepared_bank_count();

        // Three effect slots per full cohort (EQ, compressor, limiter).
        assert_eq!(
            unsplit_slots,
            3 * (64 / lanes),
            "every cohort of 64 is full"
        );
        let full_cohorts_after = (64 - 1) / lanes;
        assert_eq!(
            split_slots,
            3 * full_cohorts_after,
            "the 63-track pool binds {full_cohorts_after} full cohorts and strands its tail; the \
             one-track pool strands outright"
        );
        assert_eq!(
            unsplit_slots - split_slots,
            3,
            "one whole cohort's worth of effect slots, lost to a single odd track"
        );

        // Class A: the tracks that stopped banking render the same bits per lane.
        let (unsplit_pcm, ..) = render_console_builtins_blocks(unsplit, BLOCKS, Vec::new());
        let scalar =
            compile_console_model_with_builtins(&uniform, 2_076, &[], &scalar_console_registry());
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&unsplit_pcm, &scalar_pcm, "uniform mono strip");
        let (split_pcm, ..) = render_console_builtins_blocks(split, BLOCKS, Vec::new());
        let scalar_split =
            compile_console_model_with_builtins(&odd, 2_077, &[], &scalar_console_registry());
        let (scalar_split_pcm, ..) =
            render_console_builtins_blocks(scalar_split, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&split_pcm, &scalar_split_pcm, "one-odd-track strip");
    }

    /// Mono-collapse M1: what class pooling costs, and on which sessions -- the route fold.
    ///
    /// # The finding
    ///
    /// Issue #218's route fold is admissible only when the folded chains' lanes, taken in render
    /// order, are **exactly** the master reduction's contributor order (`route_fold`'s association
    /// proof: a floating-point sum is not associative, so "the same summands" is not "the same
    /// bits"). The reduction's order is track order. Pooling by class reorders a mixed session's
    /// lanes, so on an **interleaved** session the fold declines -- correctly, and for the reason
    /// the proof states: "a cohort whose planner ordered its lanes differently from the edge order
    /// ... declines the whole fold".
    ///
    /// That is a forfeited optimisation, not a wrong render: the digest is unchanged either way,
    /// which is exactly why it is asserted as a **count** here.
    ///
    /// # And it is a property of the interleaving, not of pooling
    ///
    /// The contiguous arm is the control. Split the same 64 tracks into the first 32 mono and the
    /// last 32 stereo -- the shape a real session takes, and the shape the alternating bench row
    /// deliberately is not -- and each pool's lane sets are contiguous runs of track order, so the
    /// chains' order still equals the reduction's and all 64 routes fold. A reader who saw only
    /// the alternating row would conclude pooling costs the fold outright; it does not.
    #[test]
    fn class_pooling_forfeits_the_route_fold_only_on_an_interleaved_session() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let lanes = width.lanes() as u64;
        let registry = launch_native_effect_registry().expect("launch registry");
        let mut folds = Vec::new();
        for (name, plan_id, stereo_at) in [
            (
                "uniform mono",
                2_071_u64,
                (|_: usize| false) as fn(usize) -> bool,
            ),
            ("alternating", 2_072, |index: usize| {
                !index.is_multiple_of(2)
            }),
            ("contiguous", 2_073, |index: usize| index >= 32),
        ] {
            let mut model =
                parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_MONO_FIXTURE).expect("mono fixture");
            for (index, track) in model.tracks.iter_mut().enumerate() {
                if stereo_at(index) {
                    track.right_source_channel = 1;
                }
            }
            let artifact = compile_console_model_with_builtins(&model, plan_id, &[], &registry);
            let (_, transposes, chains, slots, _, _, fold) =
                render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
            assert_eq!(chains, 64 / lanes, "{name}: one chain per cohort");
            assert_eq!(slots, 6 * chains, "{name}: six slots per cohort");
            assert_eq!(transposes, BLOCKS * chains, "{name}: G5");
            folds.push((name, fold));
        }
        assert_eq!(
            folds,
            vec![("uniform mono", 64), ("alternating", 0), ("contiguous", 64)],
            "the fold survives a class split whose lane sets stay in track order and declines the \
             one whose do not"
        );
    }

    /// Issue #206: a track whose rack program is a strict subsequence of its cohort leader's
    /// compiles, binds and renders instead of panicking.
    ///
    /// # The bug this is the end-to-end gate on
    ///
    /// `order_members` chose lane order by `(active_count desc, id)` and
    /// `PreparedGraphPlan::has_valid_structural_layout` requires every bank's members to be
    /// **strictly ascending**. On the issue's own repro shape -- the intended 64-track strip with
    /// one track's compressor removed -- the two disagreed: the short-program track sorted to the
    /// end of its cohort, the last bank came out `["ch57".."ch63", "ch00"]`, structural validation
    /// refused the plan, and `PreparedBuiltinsGraphArtifact::into_bound` reached
    /// `unreachable!("sealed wrapper prevalidated its complete graph bindings")`. A panic on a
    /// legal session, reachable from an ordinary console edit.
    ///
    /// Red mutation: drop the per-bank ascending pass from
    /// `rack_compiler::order_members` -> this test panics at `into_bound`, which is
    /// the exact failure the issue reports.
    ///
    /// The render is not decoration: a plan that bound the cohort and then rendered nothing would
    /// pass a bind-only assertion. It is compared against the scalar registry for the same reason
    /// every other cohort test here is -- banking regroups lanes and must move no rendered bit,
    /// and this change is a *permutation inside one bank*, which is the narrowest form of that
    /// claim.
    #[test]
    fn a_subsequence_program_track_binds_instead_of_panicking() {
        const BLOCKS: u64 = 12;
        let Some(_width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let mut model = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        // The issue's edit, exactly: one track loses one slot of its `simd1` chain, so its program
        // is a strict subsequence of every other track's and it joins their cohort with an
        // identity slot rather than forming one of its own.
        let leader_slots = model.tracks[1].simd1.effects.len();
        assert!(
            leader_slots > 1,
            "the fixture's simd1 must be a multi-slot chain for the subsequence to exist"
        );
        model.tracks[0].simd1.effects.remove(leader_slots - 1);
        assert_eq!(model.tracks[0].simd1.effects.len(), leader_slots - 1);

        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&model, 2_060, &[], &registry);
        // Every bank the plan retained is ascending; the graph would have refused the bind
        // otherwise, and this says so in the planner's own terms rather than as a panic message.
        for bank in artifact.prepared_builtin_banks() {
            let ids: Vec<_> = bank.members.to_vec();
            let mut sorted = ids.clone();
            sorted.sort();
            assert_eq!(ids, sorted, "a builtin bank's members are not ascending");
        }
        let (pcm, ..) = render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the subsequence session rendered audio"
        );
        let scalar =
            compile_console_model_with_builtins(&model, 2_061, &[], &scalar_console_registry());
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar, BLOCKS, Vec::new());
        assert_pcm_bits_equal(
            &pcm,
            &scalar_pcm,
            "64-track strip with one subsequence track",
        );
    }

    /// Misaligned lane sets decline the merge: the lane-alignment hole, made visible.
    ///
    /// A merge is only ever admissible when two banks cover the same lanes **in the same order**,
    /// and nothing makes two planners agree about that. The post-input builtin candidates all run
    /// the same one-slot program, so their banks are always plain id-ordered chunks of every track
    /// in the session. The effect cohorts are chunks of only the tracks that *carry* that chain,
    /// ordered by `order_members`. The two coincide on the intended fixture only because every
    /// track there carries every rack.
    ///
    /// Empty four tracks' `simd1` and `simd2` racks and they stop coinciding: the effect cohorts
    /// slide by four tracks, so builtin bank `{ch00..ch07}` no longer feeds effect bank
    /// `{ch00..ch07}` -- it feeds four lanes of one effect bank and four of another.
    /// `runtime::chains_into` proves the pairing lane by lane on the lowered program, so every
    /// `builtins -> EQ` merge is declined, while the effect banks, which do still align with each
    /// other, keep fusing. The plan is correct either way; it is only slower.
    ///
    /// This is the test that would catch a "merge" that silently never fires, because it asserts
    /// counts on both sides of the alignment. A digest comparison cannot: the session renders the
    /// same bits whether or not any merge is taken.
    #[test]
    fn misaligned_lane_sets_decline_the_merge() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let lanes = width.lanes() as u64;
        let cohorts = 64 / lanes;
        let mut ragged = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        // Half a bank's worth of tracks lose their whole effect strip, which is what slides every
        // effect cohort out of step with the builtin banks.
        for track in ragged.tracks.iter_mut().take((lanes / 2) as usize) {
            track.simd1.effects.clear();
            track.simd2.effects.clear();
        }
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&ragged, 2_050, &[], &registry);
        let effect_slots = artifact.graph().prepared_bank_count() as u64;
        let builtin_slots = artifact.graph().prepared_builtin_bank_count() as u64;
        let (pcm, transposes, chains, slots, _, redirects, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(slots, effect_slots + builtin_slots);
        // The three bankable stages do not group alike on a misaligned session, and that is the
        // point of the fixture. Every track's post-input node sits at one dependency level, so the
        // post-input stage banks all 64 into `cohorts` groups. The fader and the matrix sit one
        // level *later for a track that carries a strip than for a bare one*, so each of those two
        // stages splits into the 60 stripped tracks (`8` groups: seven full and a four-member tail)
        // plus the four bare ones (`1` group) -- nine apiece.
        let strip_stage_groups = (64 - lanes / 2).div_ceil(lanes) + 1;
        assert_eq!(
            builtin_slots,
            cohorts + 2 * strip_stage_groups,
            "every track still banks all three of its bankable stages"
        );
        assert_eq!(transposes, BLOCKS * chains, "G5 holds however little fuses");
        // The derivation. 60 tracks carry the strip, so each effect rack binds
        // `60 / lanes` full cohorts and strands the remainder in a padded group that never binds:
        // 7 EQ + 7 compressor + 7 limiter slots at eight lanes. Those three still align with each
        // other, so each cohort fuses into one chain. None of them aligns with a builtin bank, so
        // all `cohorts` builtin banks stay chains of their own.
        let strip_cohorts = (64 - lanes / 2) / lanes;
        assert_eq!(
            effect_slots,
            3 * strip_cohorts,
            "the tracks that keep their strip still bind three slots per full cohort"
        );
        // The derivation, continued. `strip_cohorts` of the fader groups hold exactly the tracks a
        // full effect cohort holds, so those chains run the whole strip; the two remaining fader
        // groups -- the stranded stripped tail and the four bare tracks -- have no effect bank to
        // join and fuse only into their own matrix group. The post-input banks join nothing, for
        // the reason they never did: their lane sets do not line up with any effect bank's.
        assert_eq!(
            chains,
            cohorts + strip_cohorts + 2,
            "the effect racks fuse with each other and into the fader and matrix banks whose lane \
             sets line up; the post-input banks and the two leftover strip groups stay their own"
        );
        assert!(
            chains > cohorts,
            "a misaligned session must realise more than the aligned one chain per cohort"
        );
        // Four lanes take the redirect here, and the number matters less than the fact that it is
        // neither 0 nor 64. It is what makes the intended fixture's *zero* a statement about that
        // fixture rather than about dead code: on a session whose stages line up, every chain ends
        // in a buffer its consumer already reads in place and no lane needs redirecting; on this
        // one, where they deliberately do not, the per-lane decision still fires. Both readings
        // come from the same `scatter_target` clauses over the same lowered program.
        assert_eq!(
            redirects, 4,
            "the scatter redirect is decided per lane on the lowered program, so a session whose \
             lane sets never line up still takes the lanes that qualify"
        );
        assert!(
            redirects > 0,
            "a fixture that redirects nothing cannot defend the redirect's clauses"
        );
        let scalar_artifact =
            compile_console_model_with_builtins(&ragged, 2_051, &[], &scalar_console_registry());
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with four bare tracks");
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the misaligned strip rendered audio"
        );
        println!(
            "#202 misaligned lane sets: {effect_slots} effect slots + {builtin_slots} builtin \
             slots = {slots} slots -> {chains} chains (aligned would be {cohorts}), \
             {redirects} scatter redirects"
        );
    }

    /// An observer bound to the last slot's **own node** declines that lane's scatter redirect.
    ///
    /// The producer-observer clause, reached the only way a session can reach it. A graph observer
    /// may only be bound to a `TrackStage` node, so the last slot has to *be* one -- which it is on
    /// the bank-free arm, where the post-input builtin bank is a one-slot chain whose consumer is a
    /// per-node EQ. A meter leased at `PostInputBuiltins` on that arm observes the bank member
    /// itself, and after a redirect that member's buffer is never written.
    ///
    /// Redirecting anyway would hand the meter the previous block's words, and the session output
    /// would be untouched: no digest comparison could see it, which is why the clause has a test of
    /// its own rather than resting on the strip's bits.
    #[test]
    fn a_meter_on_a_bank_member_declines_that_lanes_scatter_redirect() {
        const BLOCKS: u64 = 12;
        if BankWidth::for_backend(host_dispatch()).is_none() {
            return;
        }
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        // A late track on purpose: the *first* cohort's chain has every other cohort's ops between
        // its scatter and its consumer, so the in-between clause already declines all eight of its
        // lanes and a meter there would change nothing. Metering a lane that is admitted is what
        // makes this a test of the observer clause.
        let meter = |handle: u64, tap| MeterRequest {
            handle: MeterHandle(NonZeroU64::new(handle).expect("constant")),
            track_id: "ch63".to_owned(),
            tap,
            config: MeterConfig {
                period_frames: NonZeroU32::new(128).expect("constant"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: NonZeroUsize::new(64).expect("constant"),
                reset_generation: 0,
            },
        };
        // The unmetered bank-free arm is the reference the metered one is read against, so what
        // this test asserts is the *difference* the meter makes and not a transcribed count.
        let quiet =
            compile_console_model_with_builtins(&intended, 2_080, &[], &scalar_console_registry());
        let (quiet_pcm, _, _, _, _, quiet_redirects, _) =
            render_console_builtins_blocks(quiet, BLOCKS, Vec::new());
        assert!(
            quiet_redirects > 0,
            "the bank-free arm must redirect its builtin banks' scatters, or this test is vacuous"
        );

        let metered = compile_console_model_with_builtins(
            &intended,
            2_081,
            &[meter(1, MeterTap::PostInputBuiltins)],
            &scalar_console_registry(),
        );
        let (metered_pcm, _, _, _, frames, metered_redirects, _) =
            render_console_builtins_blocks(metered, BLOCKS, Vec::new());
        assert_eq!(
            metered_redirects,
            quiet_redirects - 1,
            "exactly ch63's lane declines when its bank member is observed"
        );
        assert_pcm_bits_equal(
            &metered_pcm,
            &quiet_pcm,
            "leasing a meter must move no rendered bit",
        );
        assert!(
            !frames.is_empty(),
            "the leased meter must publish windows, or this test proves nothing about it"
        );
        assert!(
            frames
                .iter()
                .any(|frame| frame.left.sample_peak != 0.0 || frame.right.sample_peak != 0.0),
            "the metered windows must carry signal"
        );
    }

    /// Issue #202 rec 3: an observer on the last slot's own alias declines that lane's scatter
    /// redirect -- and the observer still reads what it is supposed to read.
    ///
    /// `PostSimd2PreFader` is elided into a `program::Tap` on the limiter's op, so a meter leased
    /// there reads the limiter's own buffer. The redirect leaves that buffer unwritten -- the chain
    /// scatters into the fader instead -- so the meter would read the previous block's words. The
    /// clause is keyed on the **alias node**, exactly as `chains_into`'s is, and keying it on the
    /// producing node would miss it: nobody observes the limiter, the observer is bound to
    /// `ch00/PostSimd2PreFader`.
    ///
    /// The cost is one lane, not one chain: 63 of the 64 tracks still redirect.
    #[test]
    fn an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let cohorts = 64 / width.lanes() as u64;
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let meters = vec![MeterRequest {
            handle: MeterHandle(NonZeroU64::new(1).expect("constant")),
            track_id: "ch00".to_owned(),
            tap: MeterTap::PostSimd2PreFader,
            config: MeterConfig {
                period_frames: NonZeroU32::new(128).expect("constant"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: NonZeroUsize::new(64).expect("constant"),
                reset_generation: 0,
            },
        }];
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&intended, 2_060, &meters, &registry);
        let (pcm, transposes, chains, slots, frames, redirects, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            slots,
            STRIP_SLOTS_PER_COHORT * cohorts,
            "the meter changes no bank's membership"
        );
        // Issue #212 moved this boundary from the end of the chain to the middle of it. When the
        // chain was `builtins -> EQ -> compressor -> limiter`, `PostSimd2PreFader` aliased the
        // *last* slot's output and was downstream of every merge, so an observer there declined
        // nothing. The fader and the matrix are now slots of the same chain, so that alias sits
        // between two of them -- and an observer on it declines the limiter -> fader merge, exactly
        // as `chains_into`'s observed-alias clause says it must.
        //
        // This is the documented cliff, and it is a cost paid only when the observer is there:
        // leasing a stage meter at `PostSimd2PreFader` costs that track's cohort one extra
        // planar/AoSoA round-trip per block, because its chain can no longer span the stage the
        // meter reads. The meter must see pre-fader audio and a merged chain would hand it the
        // chain's input, so this is the intended trade -- and both halves of it are pinned here.
        assert_eq!(
            chains,
            cohorts + 1,
            "the meter sits on an alias *inside* the chain, so ch00's cohort splits there"
        );
        assert_eq!(transposes, BLOCKS * chains);
        assert_eq!(
            redirects, 0,
            "the strip's chains still end in buffers their consumers read in place, metered or not"
        );

        // And the meter reads the limiter's output. The oracle is the same session with every
        // effect on the per-node scalar path, where no chain -- and so no redirect -- exists.
        let scalar_artifact = compile_console_model_with_builtins(
            &intended,
            2_061,
            &meters,
            &scalar_console_registry(),
        );
        let (scalar_pcm, _, _, _, scalar_frames, _, _) =
            render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(
            &pcm,
            &scalar_pcm,
            "64-track strip with a pre-fader stage meter",
        );
        assert!(
            !frames.is_empty(),
            "the leased meter must publish windows, or this test proves nothing about it"
        );
        assert_eq!(
            frames, scalar_frames,
            "the metered boundary must read the limiter's output, not the previous block's words"
        );
        assert!(
            frames
                .iter()
                .any(|frame| frame.left.sample_peak != 0.0 || frame.right.sample_peak != 0.0),
            "the metered windows must carry signal"
        );
    }

    /// A second reader of the last slot's output declines that lane's scatter redirect.
    ///
    /// The sole-readership clause, reached the way a session reaches it: a send taken from
    /// `PostSimd2PreFader`. The alias resolves back to the limiter, so `op_dataflow` counts the
    /// send as a second reader of the limiter's output -- and after a redirect that buffer is never
    /// written, so the send would carry the previous block.
    #[test]
    fn a_send_from_the_last_slots_alias_declines_that_lanes_scatter_redirect() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let cohorts = 64 / width.lanes() as u64;
        let mut sent = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let mut route = sent.routes[0].clone();
        route.id = StableId::parse("ch00-pre-fader-send").expect("route id");
        route.source = RouteSource::Track {
            track_id: StableId::parse("ch00").expect("track id"),
            tap: session::SendTap::PostSimd2PreFader,
        };
        sent.routes.push(route);
        sent.routes.sort_by(|left, right| left.id.cmp(&right.id));

        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&sent, 2_070, &[], &registry);
        let (pcm, transposes, chains, slots, _, redirects, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            slots,
            STRIP_SLOTS_PER_COHORT * cohorts,
            "the send changes no bank's membership"
        );
        // The same boundary move as
        // `an_observed_alias_on_the_last_slot_declines_that_lanes_scatter_redirect`, through the
        // other clause. A send taken from `PostSimd2PreFader` gives ch00's limiter output a second
        // reader, and since #212 that alias sits *inside* the chain rather than at its end -- so
        // `chains_into`'s sole-readership clause declines the limiter -> fader merge and ch00's
        // cohort splits there. A send from a stage the chain does not span still costs nothing.
        assert_eq!(
            chains,
            cohorts + 1,
            "the send is taken from an alias inside the chain, so ch00's cohort splits there"
        );
        assert_eq!(transposes, BLOCKS * chains);
        assert_eq!(
            redirects, 0,
            "the strip's chains still end in buffers their consumers read in place, sent or not"
        );
        let scalar_artifact =
            compile_console_model_with_builtins(&sent, 2_071, &[], &scalar_console_registry());
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with a pre-fader send");
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the sent strip rendered audio"
        );
    }

    /// Issue #218: the intended strip folds every route and the whole master reduction into its
    /// cohorts' epilogues, and the bits do not move.
    ///
    /// # Two oracles, because the obvious one is not one
    ///
    /// `scalar_console_registry` is this file's standing oracle: it puts every *effect* on the
    /// per-node path. It does **not** put the strip's builtins, fader and matrix there -- those
    /// bank on any backend that has a width -- so the scalar arm still forms cohort chains and
    /// still folds all sixty-four routes. It is an oracle for the effects and not for the fold, and
    /// saying so is the point of asserting its fold count rather than assuming it.
    ///
    /// The fold's own oracle is a **post-matrix meter**, which binds an observer to the chain's
    /// last slot and declines the fold plan-wide (see
    /// `a_meter_on_the_matrix_declines_the_route_fold_and_still_meters`). That arm renders the
    /// route ops and the D9 reduction the fold replaced, and AGENTS.md requires a meter not to
    /// change signal flow, so the two arms differ in exactly the thing under test.
    #[test]
    fn the_intended_strip_folds_every_route_into_its_cohorts_epilogue() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let cohorts = 64 / width.lanes() as u64;
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&intended, 2_180, &[], &registry);
        let (pcm, transposes, chains, slots, _, _, folds) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            chains, cohorts,
            "the whole strip is still one chain per cohort"
        );
        assert_eq!(slots, STRIP_SLOTS_PER_COHORT * cohorts);
        assert_eq!(transposes, BLOCKS * chains, "G5 is unmoved by the fold");
        assert_eq!(
            folds, 64,
            "every track's route folds into its cohort's epilogue"
        );
        let scalar_artifact =
            compile_console_model_with_builtins(&intended, 2_181, &[], &scalar_console_registry());
        let (scalar_pcm, _, _, _, _, _, scalar_folds) =
            render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_eq!(
            scalar_folds, 64,
            "the per-node arm still banks the strip's builtins, fader and matrix, so it folds too"
        );
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with the route fold");

        // The fold's own oracle: the same session with a post-matrix meter, which declines it.
        let meters = vec![MeterRequest {
            handle: MeterHandle(NonZeroU64::new(1).expect("constant")),
            track_id: "ch00".to_owned(),
            tap: MeterTap::PostMatrix,
            config: MeterConfig {
                period_frames: NonZeroU32::new(128).expect("constant"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: NonZeroUsize::new(64).expect("constant"),
                reset_generation: 0,
            },
        }];
        let unfolded_artifact =
            compile_console_model_with_builtins(&intended, 2_188, &meters, &registry);
        let (unfolded_pcm, _, _, _, _, _, unfolded_folds) =
            render_console_builtins_blocks(unfolded_artifact, BLOCKS, Vec::new());
        assert_eq!(
            unfolded_folds, 0,
            "the metered arm must decline the fold, or it is not an oracle for it"
        );
        assert_pcm_bits_equal(
            &pcm,
            &unfolded_pcm,
            "the folded master against the reduction",
        );
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the folded strip rendered audio"
        );
    }

    /// A second reader of a track's matrix declines the route fold.
    ///
    /// The sole-readership clause, reached the way a session reaches it: ch00 is routed to the
    /// master twice. Its matrix -- the chain's last slot -- then has two readers, and a folded lane
    /// stops writing that buffer altogether, so the second route would carry the previous block.
    ///
    /// The decline is plan-wide rather than lane-wide on purpose, and this test pins that too: a
    /// half-folded plan would have to insert the unfolded lane's summand at a position in the
    /// chains' order that the chains' order has no room for, so `route_fold` folds every
    /// contributor of one reduction or none.
    ///
    /// Red mutation: drop the `readers[producer].len() != 1` clause -- ch00's first route folds,
    /// its second reads a buffer nothing writes any more, and the digest diverges from the scalar
    /// arm at the first block.
    #[test]
    fn a_second_route_from_a_tracks_matrix_declines_the_route_fold() {
        const BLOCKS: u64 = 12;
        if BankWidth::for_backend(host_dispatch()).is_none() {
            return;
        }
        let mut doubled = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let mut route = doubled.routes[0].clone();
        route.id = StableId::parse("ch00-second-main").expect("route id");
        doubled.routes.push(route);
        doubled.routes.sort_by(|left, right| left.id.cmp(&right.id));

        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&doubled, 2_182, &[], &registry);
        let (pcm, _, _, _, _, _, folds) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            folds, 0,
            "a second reader of one matrix declines the whole reduction's fold"
        );
        let scalar_artifact =
            compile_console_model_with_builtins(&doubled, 2_183, &[], &scalar_console_registry());
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with ch00 routed twice");
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the doubly-routed strip rendered audio"
        );
    }

    /// A meter on the matrix declines the route fold, and still meters.
    ///
    /// The observer clause. `MeterTap::PostMatrix` binds a `GraphNodeObserverBinding` to the chain's
    /// **last slot**, whose planar buffer a folded lane stops writing: the meter would read the
    /// previous block for ever. The cost is stated rather than hidden -- a console that leases a
    /// post-matrix meter gives up the fold for the whole plan -- and it is the same trade
    /// `a_leased_stage_meter_declines_the_merge_and_still_meters` records for the chain merge.
    ///
    /// Red mutation: drop the `observed(program, spec, parts, producer)` clause -- the plan folds
    /// and every metered window reports the previous block's peak, which the falsifiability
    /// assertion below turns red.
    #[test]
    fn a_meter_on_the_matrix_declines_the_route_fold_and_still_meters() {
        const BLOCKS: u64 = 12;
        if BankWidth::for_backend(host_dispatch()).is_none() {
            return;
        }
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let meters = vec![MeterRequest {
            handle: MeterHandle(NonZeroU64::new(1).expect("constant")),
            track_id: "ch00".to_owned(),
            tap: MeterTap::PostMatrix,
            config: MeterConfig {
                period_frames: NonZeroU32::new(128).expect("constant"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: NonZeroUsize::new(64).expect("constant"),
                reset_generation: 0,
            },
        }];
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&intended, 2_184, &meters, &registry);
        let (pcm, _, _, _, frames, _, folds) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(folds, 0, "a post-matrix meter declines the fold");

        let scalar_artifact = compile_console_model_with_builtins(
            &intended,
            2_185,
            &meters,
            &scalar_console_registry(),
        );
        let (scalar_pcm, _, _, _, scalar_frames, _, _) =
            render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with a post-matrix meter");
        assert_eq!(
            frames.len(),
            scalar_frames.len(),
            "the declining plan publishes the same meter windows"
        );
        for (banked, scalar) in frames.iter().zip(scalar_frames.iter()) {
            assert_eq!(
                (
                    banked.left.sample_peak.to_bits(),
                    banked.right.sample_peak.to_bits()
                ),
                (
                    scalar.left.sample_peak.to_bits(),
                    scalar.right.sample_peak.to_bits()
                ),
                "a post-matrix meter must read post-matrix audio"
            );
        }
        assert!(
            frames
                .iter()
                .any(|frame| frame.left.sample_peak != 0.0 || frame.right.sample_peak != 0.0),
            "the metered windows must carry signal"
        );
    }

    /// Route ids ordered against the cohorts decline the fold: the association proof, at session
    /// level.
    ///
    /// The reduction is D9 -- `sum2_block` then `sum_into_block`, left to right in stable edge-ID
    /// order -- and the epilogues accumulate in **chain** order. On every checked-in fixture those
    /// two orders coincide, because route ids sort the way tracks do; a floating-point sum is not
    /// associative, so that coincidence is a fact about the fixture and not a property of the
    /// engine. This session breaks it deliberately: track `chNN`'s route is named so that the
    /// routes sort in *reverse* track order while the cohorts still render in track order.
    ///
    /// `route_fold` must decline, and the render must be the reduction's own bits.
    ///
    /// Red mutation: keep the length check and drop the element-wise comparison in the association
    /// proof -- the plan folds all 64 lanes in chain order, sums 64 contributions in the reverse of
    /// the order the reduction would have, and diverges from the scalar arm at the first block.
    #[test]
    fn route_ids_ordered_against_the_cohorts_decline_the_route_fold() {
        const BLOCKS: u64 = 12;
        if BankWidth::for_backend(host_dispatch()).is_none() {
            return;
        }
        let mut reversed = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let count = reversed.routes.len();
        for (index, route) in reversed.routes.iter_mut().enumerate() {
            route.id = StableId::parse(&format!("r{:03}-main", count - 1 - index))
                .expect("reversed route id");
        }
        reversed
            .routes
            .sort_by(|left, right| left.id.cmp(&right.id));

        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&reversed, 2_186, &[], &registry);
        let (pcm, _, _, _, _, _, folds) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            folds, 0,
            "the chains accumulate in track order and the reduction sums in reverse: no fold"
        );
        let scalar_artifact =
            compile_console_model_with_builtins(&reversed, 2_187, &[], &scalar_console_registry());
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with reversed route ids");
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the reversed-route strip rendered audio"
        );
    }

    /// Issue #212: a banked fader honours the drain contract, bit-for-bit against the node form.
    ///
    /// # The contract, and why banking is where it could have been lost
    ///
    /// `TrackFaderRecord` rides a bounded SPSC queue with exactly one consumer, and the consumer
    /// drains it *at the top of the block*, before any audio is touched -- so a record admitted
    /// while block `N` is being prepared takes effect on the first sample of block `N` and not on
    /// block `N + 1`. That is what the control side is acknowledged with, and it is the property
    /// #210 Phase 1's solo depends on.
    ///
    /// Banking moves the consumer from a per-track node to a bank lane, which is exactly the kind
    /// of move that silently costs a block of latency: a drain placed after the gather, or once
    /// per bank instead of once per lane, or in the wrong order relative to the kernel, would all
    /// still render plausible audio.
    ///
    /// So the oracle is the node form of the same session -- `Backend::Scalar` binds no bank at
    /// all, so every track keeps its `ConsoleFaderProcessor` -- driven by the *same* commands at
    /// the *same* blocks, and the two must agree word for word. Lane identity (#83 D4/D5) is what
    /// makes the scalar arm a legitimate oracle for the vector one.
    ///
    /// # Two ways this test was vacuous before it was a gate, both recorded on purpose
    ///
    /// The falsifiability assertion below exists because the obvious shapes of this test do not
    /// detect a one-block error at all, and both of them *passed*:
    ///
    /// * **Driving one track.** One track's fader ramp is a few percent of one of sixty-four
    ///   contributions to the master sum and rounds away in `f32`. The node form was exactly as
    ///   insensitive as the banked one, which is how the vacuity was caught rather than shipped.
    /// * **Pushing commands during the priming window.** This plan's compensation delays and the
    ///   limiter's lookahead put the first non-silent output at block 11, so a command admitted
    ///   before then has settled long before anything it did could reach the session output.
    ///
    /// Red-mutation proven: draining `FaderBankProcessor`'s queues *after* `bank.process` instead
    /// of before it -- one block of latency, and nothing else -- fails at block 14, the block the
    /// first record was admitted in.
    #[test]
    fn a_banked_fader_command_lands_on_the_block_it_was_admitted_in() {
        // This fixture's plan is deeply latent -- the strip's compensation delays and the
        // limiter's lookahead put the first non-silent output at block 11 -- so a command pushed
        // before then is settled long before anything it did could reach the session output. The
        // script therefore lands *after* audio starts, and the render runs well past it.
        const BLOCKS: u64 = 28;
        const FIRST_AUDIBLE_BLOCK: u64 = 11;
        // Commands are admitted between blocks, so the strongest case is a move admitted at a
        // block boundary with a window that outlives the block: a ramp in flight across the
        // boundary is what a one-block drain error corrupts most visibly.
        let script: &[(u64, TrackFaderRecord)] = &[
            (
                FIRST_AUDIBLE_BLOCK + 3,
                TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -9.5,
                    smoothing_samples: 311,
                },
            ),
            (
                FIRST_AUDIBLE_BLOCK + 5,
                TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Left,
                    muted: true,
                    smoothing_samples: 97,
                },
            ),
            (
                FIRST_AUDIBLE_BLOCK + 8,
                TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Right,
                    db: 4.0,
                    smoothing_samples: 0,
                },
            ),
            (
                FIRST_AUDIBLE_BLOCK + 11,
                TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Left,
                    muted: false,
                    smoothing_samples: 41,
                },
            ),
        ];

        const TRACKS: usize = 64;
        let banked = render_console_fader_script(host_dispatch(), 2_130, script, TRACKS, BLOCKS);
        let node_form = render_console_fader_script(Backend::Scalar, 2_131, script, TRACKS, BLOCKS);
        assert_eq!(
            (node_form.1, node_form.2),
            (0, 0),
            "the scalar oracle must bind no bank at all, or it is not the node form"
        );
        assert!(
            banked.1 > 0 && banked.2 > 0,
            "the banked arm must actually bank, or this test compares two node forms"
        );
        // The gate is only a gate if a one-block error is visible in what it compares. The same
        // script pushed one block later must render *different* audio -- otherwise "the command
        // landed on the block it was admitted in" is unfalsifiable here and this test proves
        // nothing about the drain's position at all.
        let shifted: Vec<(u64, TrackFaderRecord)> = script
            .iter()
            .map(|(at, record)| (at + 1, *record))
            .collect();
        let late = render_console_fader_script(host_dispatch(), 2_135, &shifted, TRACKS, BLOCKS);
        assert!(
            late.0
                .iter()
                .zip(banked.0.iter())
                .any(|(left, right)| left != right),
            "a one-block command delay must be visible in this fixture, or the comparison below \
             cannot detect one"
        );
        assert_pcm_bits_equal(
            &banked.0,
            &node_form.0,
            "banked fader drain vs the per-node console",
        );
        assert!(
            banked.0.iter().flatten().any(|sample| *sample != 0.0),
            "the driven strip rendered audio"
        );

        // And the commands must actually have moved the audio, or the comparison above is between
        // two runs of the same silence. The quiet arm pushes nothing and must differ.
        let quiet = render_console_fader_script(host_dispatch(), 2_132, &[], TRACKS, BLOCKS);
        assert!(
            quiet
                .0
                .iter()
                .zip(banked.0.iter())
                .any(|(left, right)| left != right),
            "a fader script that moves no bit proves nothing about when it landed"
        );
    }

    /// Issue #212: a meter leased at `PostFader` splits the chain there, and still meters right.
    ///
    /// # The cliff, stated from both sides
    ///
    /// A meter at `PostMatrix` -- the tap the console actually leases, and the chain's *last*
    /// slot -- is downstream of every merge and costs nothing;
    /// `console_facilities_do_not_change_the_chain_shape_or_the_bits` pins that. A meter at
    /// `PostFader` is not: the fader is now a slot with the matrix slot behind it, so an observer
    /// on the fader node declines the fader -> matrix merge and that track's cohort renders as two
    /// chains instead of one. One extra planar/AoSoA round-trip per block, for that cohort only,
    /// paid only while the meter is leased.
    ///
    /// That is the same trade `a_leased_stage_meter_declines_the_merge_and_still_meters` documents
    /// one stage earlier, and it is intended rather than tolerated: the meter must see post-fader,
    /// pre-matrix audio, and a chain spanning that boundary would hand it the chain's input.
    ///
    /// Both halves are asserted, because either alone would pass for the wrong reason: a split
    /// that metered garbage, or a meter that read correctly because the merge never fired at all.
    #[test]
    fn a_meter_leased_at_post_fader_splits_the_chain_and_still_meters() {
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let cohorts = 64 / width.lanes() as u64;
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let meters = vec![MeterRequest {
            handle: MeterHandle(NonZeroU64::new(1).expect("constant")),
            track_id: "ch00".to_owned(),
            tap: MeterTap::PostFader,
            config: MeterConfig {
                period_frames: NonZeroU32::new(128).expect("constant"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: NonZeroUsize::new(64).expect("constant"),
                reset_generation: 0,
            },
        }];
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&intended, 2_120, &meters, &registry);
        let (pcm, transposes, chains, slots, frames, _, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert_eq!(
            slots,
            STRIP_SLOTS_PER_COHORT * cohorts,
            "the meter changes no bank's membership"
        );
        assert_eq!(
            chains,
            cohorts + 1,
            "ch00's cohort splits at the metered fader; every other cohort stays one chain"
        );
        assert_eq!(transposes, BLOCKS * chains, "G5 on the declining plan");

        // And the meter reads the fader's output. The oracle is the same session with every effect
        // on the per-node scalar path, where the strip's effect banks do not exist and the meter
        // can only be reading the fader.
        let scalar_artifact = compile_console_model_with_builtins(
            &intended,
            2_121,
            &meters,
            &scalar_console_registry(),
        );
        let (scalar_pcm, _, _, _, scalar_frames, _, _) =
            render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(&pcm, &scalar_pcm, "64-track strip with a post-fader meter");
        assert!(
            !frames.is_empty(),
            "the leased meter must publish windows, or this test proves nothing about it"
        );
        assert_eq!(
            frames, scalar_frames,
            "the metered fader must read the fader's output, not the chain's input"
        );
        assert!(
            frames
                .iter()
                .any(|frame| frame.left.sample_peak != 0.0 || frame.right.sample_peak != 0.0),
            "the metered windows must carry signal"
        );
    }

    /// Compile and render the intended console fixture with a live fader channel on `ch00`,
    /// pushing `script`'s records at the block they name, and return
    /// `(pcm, builtin banks, effect banks)`.
    ///
    /// The dispatch is a parameter because that is the whole oracle: `Backend::Scalar` binds no
    /// bank, so it renders the per-node console this test compares the banked one against.
    fn render_console_fader_script(
        dispatch: Backend,
        plan_id: u64,
        script: &[(u64, TrackFaderRecord)],
        tracks: usize,
        blocks: u64,
    ) -> (Vec<Vec<f32>>, usize, usize) {
        let model = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let session = compile_session(
            &model,
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
        // Every track gets a channel, and the script drives every one of them. Driving a single
        // track cannot gate this: one track's ramp is a few percent of one of sixty-four
        // contributions to the master sum, and it rounds away in `f32` -- the *node form* is just
        // as insensitive to command timing there as the banked one, which is what makes that shape
        // a vacuous gate rather than a passing one. Driving every lane also means a bank that
        // drained its lanes in the wrong order, or applied a record to the wrong lane, is caught
        // here rather than by luck.
        let controls: Vec<TrackControlRequest> = model
            .tracks
            .iter()
            .map(|track| TrackControlRequest {
                track_id: track.id.as_str().to_owned(),
                queue_capacity: NonZeroUsize::new(16).expect("constant"),
            })
            .collect();
        let builtins = prepare_session_builtins_with_console(
            &session,
            &[],
            &controls,
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
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch,
            plan_id,
            effects: prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 22,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("prepared console effects"),
            builtins,
            caps: integration_caps(),
        })
        .unwrap_or_else(|_| panic!("production console graph"));
        let builtin_banks = artifact.prepared_builtin_bank_count();
        let effect_banks = artifact.graph().prepared_bank_count();
        let envelope = artifact.envelope();
        let frames = envelope.quantum.0 as usize;
        let nodes = artifact
            .external_binding_nodes()
            .map(|node| GraphNodeBinding::new(node.clone(), console_track_sustained_binding(node)))
            .collect();
        let bound = artifact
            .into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("production console bind: {}", failure.code));
        let mut plan = bound.plan;
        let mut controls = bound.track_controls;
        let pcm = (0..blocks)
            .map(|block| {
                // Admitted *before* the block renders, which is the contract under test: every
                // sample of this block must be rendered by the post-command ramp.
                for (at, record) in script {
                    if *at != block {
                        continue;
                    }
                    for (index, channel) in controls.iter_mut().enumerate().take(tracks) {
                        // Each lane gets its own gain, so a record applied to the wrong lane
                        // changes the output rather than being masked by a uniform move.
                        let record = match *record {
                            TrackFaderRecord::FaderDb {
                                lanes,
                                db,
                                smoothing_samples,
                            } => TrackFaderRecord::FaderDb {
                                lanes,
                                db: db + index as f32 * 0.125,
                                smoothing_samples: smoothing_samples + index as u32,
                            },
                            other => other,
                        };
                        channel.fader.try_push(record).expect("queue has room");
                    }
                }
                let mut pcm = vec![0.0_f32; frames * 2];
                plan.render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                            .expect("output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("console render");
                pcm
            })
            .collect();
        (pcm, builtin_banks, effect_banks)
    }

    /// Compile one console session model into a prepared graph artifact.
    fn compile_console_model(model: &session::SessionModel, plan_id: u64) -> PreparedGraphArtifact {
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
        let registry = launch_native_effect_registry().expect("launch registry");
        GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id,
            effects: prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 22,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("prepared console effects"),
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("console graph: {:?}", failure.diagnostics))
    }

    /// Compile one console session model into a prepared graph artifact **with** its builtin
    /// banks, the way the production pipeline assembles a plan.
    ///
    /// [`compile_console_model`] attaches none, so the plans it builds carry effect banks only and
    /// the `builtins -> simd1` boundary does not exist in them at all. Issue #202 rec 2 fuses
    /// across exactly that boundary, so every test that measures it has to compile the production
    /// pair rather than the effect-only one.
    fn compile_console_model_with_builtins(
        model: &session::SessionModel,
        plan_id: u64,
        meters: &[MeterRequest],
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
            meters,
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
        GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch: host_dispatch(),
            plan_id,
            effects: prepare_native_session_effects(
                &session,
                registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 22,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("prepared console effects"),
            builtins,
            caps: integration_caps(),
        })
        .unwrap_or_else(|_| panic!("production console graph"))
    }

    /// The registry that forces every console effect onto the per-node scalar path.
    ///
    /// The bank-free arm is the oracle a merged chain is compared against: it binds no bank at
    /// all, so no merge is expressible in it and the audio it renders is the strip's arithmetic
    /// with none of this machinery in the way.
    fn scalar_console_registry() -> NativeEffectRegistry {
        let registry = launch_native_effect_registry().expect("launch registry");
        NativeEffectRegistry::new(
            [
                "miso.parametric-eq",
                "miso.compressor",
                "miso.true-peak-limiter",
            ]
            .map(|id| {
                Box::new(ScalarOnlyDelegateFactory {
                    delegate: registry
                        .get_shared_ascii(id)
                        .expect("registered launch effect"),
                }) as Box<dyn NativeEffectFactory>
            }),
        )
        .expect("scalar console registry")
    }

    /// Bind and render a production console artifact, returning what it rendered, what its chains
    /// did, and every meter frame its streams published.
    fn render_console_builtins_blocks(
        artifact: PreparedGraphBuiltinsArtifact,
        blocks: u64,
        observers: Vec<GraphNodeObserverBinding>,
    ) -> (Vec<Vec<f32>>, u64, u64, u64, Vec<MeterSnapshot>, u64, u64) {
        let envelope = artifact.envelope();
        let frames = envelope.quantum.0 as usize;
        let nodes = artifact
            .external_binding_nodes()
            .map(|node| GraphNodeBinding::new(node.clone(), console_track_input_binding(node)))
            .collect();
        let bound = artifact
            .into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers,
            })
            .unwrap_or_else(|failure| panic!("production console bind: {}", failure.code));
        let mut plan = bound.plan;
        let mut meter_consumers = bound.meter_consumers;
        let mut meter_frames: Vec<MeterSnapshot> = Vec::new();
        let pcm = (0..blocks)
            .map(|block| {
                let mut pcm = vec![0.0_f32; frames * 2];
                plan.render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                            .expect("output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("console render");
                for stream in &mut meter_consumers {
                    while let Ok(snapshot) = stream.consumer.try_pop() {
                        meter_frames.push(snapshot);
                    }
                }
                pcm
            })
            .collect();
        let transposes = plan.bank_transposes();
        let [chains, slots] = plan.bank_shape();
        let redirects = plan.bank_scatter_redirects();
        let folds = plan.bank_route_folds();
        (
            pcm,
            transposes,
            chains,
            slots,
            meter_frames,
            redirects,
            folds,
        )
    }

    /// Issue #202 rec 2: the intended strip is **one chain per cohort**, end to end.
    ///
    /// This is the finding the recommendation was written to take. The production 64-track plan
    /// binds four bank slots per cohort -- the post-input builtin stage, the EQ, the compressor and
    /// the limiter -- and issue #181 rendered them as three chains: the builtin bank had no cohort
    /// group at all, so `builtins -> simd1` was not an expressible candidate, and the cohort
    /// planner pools per `RackLocation`, so `simd1 -> simd2` was not either. Eight cohorts times
    /// three chains is the 24 planar/AoSoA round-trips a block that the audit measured, of which
    /// 16 separated stages with nothing planar reading between them.
    ///
    /// Sixteen of the 24 now go away, because candidacy comes from the lowered program's dataflow:
    ///
    /// * `builtins -> EQ`. The EQ's op is the sole reader of the builtin bank's output, undelayed
    ///   and unmixed. Nothing elides between them -- `PostInputBuiltins` is a bindable stage that
    ///   keeps its op, and that op *is* the bank member.
    /// * `EQ -> compressor`. The pair #181 already fused, inside `simd1`.
    /// * `compressor -> limiter`. Three elided stage boundaries sit between them (`PostSimd1`,
    ///   `PostDynamic`, `PostSimd2PreFader`), each contributing a `program::Tap` on the
    ///   compressor's op. A tap is a *name*, not a read: an edge out of one resolves back to the
    ///   compressor and is counted as a second reader, and the only other thing that can read one
    ///   is an observer bound to the alias node. This session leases no stage meter, so nothing
    ///   does, and the merge is admitted. `a_leased_stage_meter_declines_the_merge_and_still_meters`
    ///   is the other side of that clause.
    ///
    /// The count is asserted, not just the digest: a silent non-merge renders exactly the same
    /// bits, so a bit comparison alone cannot tell a working merge from a merge that never fires.
    #[test]
    fn the_intended_strip_fuses_the_whole_signal_path_into_one_chain_per_cohort() {
        // Enough blocks for the limiter's lookahead latency to clear, so the non-silence check
        // below is a real statement about the strip rather than about its priming window.
        const BLOCKS: u64 = 12;
        let Some(width) = BankWidth::for_backend(host_dispatch()) else {
            return;
        };
        let cohorts = 64 / width.lanes() as u64;
        let intended = parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE)
            .expect("intended fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let artifact = compile_console_model_with_builtins(&intended, 2_020, &[], &registry);
        let effect_slots = artifact.graph().prepared_bank_count() as u64;
        let builtin_slots = artifact.graph().prepared_builtin_bank_count() as u64;
        assert_eq!(
            effect_slots,
            3 * cohorts,
            "the intended strip binds three effect slots per cohort: EQ, compressor and limiter"
        );
        assert_eq!(
            builtin_slots,
            BANKABLE_TRACK_STAGES * cohorts,
            "and one builtin bank per bankable stage per cohort: post-input, fader, matrix"
        );

        let (pcm, transposes, chains, slots, _, redirects, _) =
            render_console_builtins_blocks(artifact, BLOCKS, Vec::new());
        assert!(
            pcm.iter().flatten().any(|sample| *sample != 0.0),
            "the intended strip rendered audio"
        );
        assert_eq!(
            slots,
            effect_slots + builtin_slots,
            "every bound bank is a slot of exactly one realised chain"
        );
        // The derivation, stated: 6 slots per cohort, 5 merges per cohort, 1 chain per cohort.
        assert_eq!(
            slots,
            STRIP_SLOTS_PER_COHORT * cohorts,
            "six bank slots per cohort"
        );
        assert_eq!(
            chains, cohorts,
            "the whole strip is one chain per cohort: builtins -> EQ -> compressor -> limiter -> \
             fader -> matrix"
        );
        assert_eq!(
            transposes,
            BLOCKS * chains,
            "G5: one planar/AoSoA round-trip per realised chain per block"
        );
        assert_eq!(
            slots - chains,
            (STRIP_SLOTS_PER_COHORT - 1) * cohorts,
            "every slot but the first of each cohort is fused into its predecessor"
        );

        // Issue #202 rec 3 removed 64 stereo block copies here by pointing each chain's scatter at
        // the fader op it fed, because `program::is_dedicated` refused to let the fader consume the
        // limiter's buffer in place. Issue #212 removes the same 64 copies a different way, and the
        // redirect count going to **zero** is how that says so.
        //
        // The fader is now a *slot* of the chain rather than its consumer, and a later slot's op is
        // never executed -- so the `reduce_plane` copy out of the limiter's dedicated buffer does
        // not happen at all, rather than happening into a redirected target. What the chain scatters
        // into is the last slot's buffer, and the last slot is now the matrix: the matrix op folds
        // in place onto the fader's buffer (the fader is not dedicated storage), and the track's
        // route op folds in place onto that. So the chain already scatters straight into the buffer
        // its consumer reads, `scatter_target` declines on its "not already in place" clause, and
        // there is nothing left for a redirect to remove.
        //
        // This is the one assertion in this test that would read the same if the merge had silently
        // stopped firing, so it is stated *with* the chain count above and never on its own.
        assert_eq!(
            redirects, 0,
            "the strip's chain ends in a buffer its consumer already reads in place, so no lane \
             needs its scatter redirected"
        );

        // Bits: the merged strip renders exactly what the bank-free strip renders. Necessary, and
        // on its own not sufficient -- which is why the chain count above is asserted too.
        let scalar_registry = scalar_console_registry();
        let scalar_artifact =
            compile_console_model_with_builtins(&intended, 2_021, &[], &scalar_registry);
        assert_eq!(
            scalar_artifact.graph().prepared_bank_count(),
            0,
            "the oracle arm must bind no effect bank at all"
        );
        let (scalar_pcm, ..) = render_console_builtins_blocks(scalar_artifact, BLOCKS, Vec::new());
        assert_pcm_bits_equal(
            &pcm,
            &scalar_pcm,
            "64-track intended strip: one fused chain per cohort vs the per-node path",
        );
        println!(
            "#202 intended strip: {effect_slots} effect slots + {builtin_slots} builtin slots \
             = {slots} slots -> {chains} chains ({transposes} transposes over {BLOCKS} blocks), \
             {redirects} scatter redirects"
        );
    }

    /// Render the console fixture, returning each block's PCM and the plan's transpose counter.
    fn render_console_blocks(
        artifact: PreparedGraphArtifact,
        blocks: u64,
    ) -> (Vec<Vec<f32>>, u64, u64, u64) {
        let graph = artifact.graph;
        let envelope = graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let nodes = graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), console_track_input_binding(node)))
            .collect();
        let mut plan = graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("console bind: {}", failure.code));
        let pcm = (0..blocks)
            .map(|block| {
                let mut pcm = vec![0.0_f32; frames * 2];
                plan.render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                            .expect("output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("console render");
                pcm
            })
            .collect();
        let transposes = plan.bank_transposes();
        // Issue #181: the counter and the structure are read together, so a test can assert that
        // the round-trips it measured really are one per realised chain and not one per slot.
        let [chains, slots] = plan.bank_shape();
        (pcm, transposes, chains, slots)
    }

    #[test]
    fn launch_gate_expander_fixture_retains_width_correct_banks_and_scalar_fallbacks() {
        let model = accepted_gate_expander_graph_fixture();
        assert_eq!(model.tracks.len(), 10);
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("accepted gate/expander fixture");
        let registry = launch_native_effect_registry().expect("launch registry");
        let gate_expander = registry
            .get_shared_ascii("miso.gate-expander")
            .expect("registered gate/expander");
        let scalar_registry = NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory {
            delegate: gate_expander,
        })
            as Box<dyn NativeEffectFactory>])
        .expect("scalar gate/expander registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared gate/expander effects");
        assert_eq!(effects.entries.len(), 10);
        assert!(effects.entries.iter().all(|entry| {
            entry.metadata.latency == LatencySamples(480)
                && entry.metadata.tail == TailSamples::Finite(0)
        }));
        let scalar_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared scalar gate/expander effects");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_014,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("gate/expander graph: {:?}", failure.diagnostics));
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_015,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar gate/expander graph: {:?}", failure.diagnostics));
        let width = BankWidth::for_backend(artifact.report.rack_cohorts.dispatch);
        if let Some(width) = width {
            let lanes = width.lanes() as usize;
            let expected_banks = 9 / lanes;
            let expected_scalar_tails = 1 + 9 % lanes;
            assert_eq!(artifact.graph.prepared_bank_count(), expected_banks);
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .bound_groups_in(RackLocation::Simd1)
                    .count(),
                expected_banks
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .bound_groups_in(RackLocation::Simd1)
                    .all(|bank| bank.active_count() == lanes)
            );
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .len(),
                expected_scalar_tails
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq8")
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq9")
            );
        } else {
            assert_eq!(artifact.graph.prepared_bank_count(), 0);
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocation::Simd1)
                    .len(),
                10
            );
        }
        assert_eq!(
            artifact.graph.sequential_schedule,
            scalar_artifact.graph.sequential_schedule
        );
        assert_eq!(
            artifact.graph.route_timings,
            scalar_artifact.graph.route_timings
        );
        assert_eq!(
            artifact.graph.inserted_delays,
            scalar_artifact.graph.inserted_delays
        );
        let expected_schedule = artifact.graph.sequential_schedule.clone();
        let expected_route_timings = artifact.graph.route_timings.clone();
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: bank_graph,
            report: _,
        } = artifact;
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: scalar_graph,
            report: _,
        } = scalar_artifact;
        let envelope = bank_graph.envelope;
        let bank_nodes = bank_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), parametric_eq_input_binding(node)))
            .collect();
        let scalar_nodes = scalar_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), parametric_eq_input_binding(node)))
            .collect();
        let mut bank_plan = bank_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("gate/expander bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("gate/expander scalar bind: {}", failure.code));
        let frames = envelope.quantum.0 as usize;
        let mut rendered_after_latency = false;
        for block in 0..16_u64 {
            let mut bank_pcm = vec![0.0_f32; frames * 2];
            let mut scalar_pcm = vec![0.0_f32; frames * 2];
            bank_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut bank_pcm, 2, frames, frames)
                            .expect("bank output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("bank render");
            scalar_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                            .expect("scalar output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("scalar render");
            assert_eq!(
                bank_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                scalar_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "bank and scalar gate/expander paths remain exact through carried state"
            );
            if block >= 3 {
                rendered_after_latency |= bank_pcm.iter().any(|sample| *sample != 0.0);
            }
        }
        assert!(
            rendered_after_latency,
            "the fixed ten-millisecond gate/expander delay renders only after its latency"
        );

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.simd1.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled bypass gate/expander fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass gate/expander effects");
        assert!(
            bypass_effects
                .entries
                .iter()
                .all(|entry| entry.metadata.latency == LatencySamples(480))
        );
        let bypass_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_016,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bypass gate/expander graph: {:?}", failure.diagnostics));
        assert_eq!(bypass_artifact.graph.sequential_schedule, expected_schedule);
        assert_eq!(bypass_artifact.graph.route_timings, expected_route_timings);
    }

    #[test]
    fn launch_true_peak_limiter_fixture_retains_banks_tails_latency_and_transactional_caps() {
        let model = accepted_true_peak_limiter_graph_fixture();
        assert_eq!(model.tracks.len(), 10);
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("accepted true-peak limiter fixture");
        assert_eq!(session.sample_rate().0, 48_000);
        assert_eq!(session.quantum().0, 128);

        let registry = launch_native_effect_registry().expect("launch registry");
        let limiter = registry
            .get_shared_ascii("miso.true-peak-limiter")
            .expect("registered true-peak limiter");
        let scalar_registry =
            NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory { delegate: limiter })
                as Box<dyn NativeEffectFactory>])
            .expect("scalar limiter registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared bank-capable limiter effects");
        assert_eq!(effects.entries.len(), 10);
        assert!(effects.entries.iter().all(|entry| {
            entry.metadata.latency == LatencySamples(486)
                && entry.metadata.tail == TailSamples::Infinite
        }));
        let scalar_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared scalar limiter effects");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_050,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("true-peak limiter graph: {:?}", failure.diagnostics));
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_051,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar limiter graph: {:?}", failure.diagnostics));

        let width = BankWidth::for_backend(artifact.report.rack_cohorts.dispatch);
        let (expected_banks, expected_scalar_tails) = width.map_or((0, 10), |width| {
            let lanes = width.lanes() as usize;
            (10 / lanes, 10 % lanes)
        });
        assert_eq!(artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members: Vec<Vec<String>> = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocation::Simd1)
            .map(|bank| {
                bank.members
                    .iter()
                    .flatten()
                    .map(|member| member.track_id.as_str().to_owned())
                    .collect()
            })
            .collect();
        let expected_members: Vec<Vec<String>> = width.map_or_else(Vec::new, |width| {
            let lanes = width.lanes() as usize;
            (0..expected_banks)
                .map(|bank| {
                    (bank * lanes..(bank + 1) * lanes)
                        .map(|index| format!("eq{index}"))
                        .collect()
                })
                .collect()
        });
        assert_eq!(
            actual_members, expected_members,
            "full banks retain stable membership"
        );
        let actual_tails: Vec<_> = artifact
            .report
            .rack_cohorts
            .scalar_in(RackLocation::Simd1)
            .iter()
            .map(|tail| tail.track_id.as_str().to_owned())
            .collect();
        let expected_tails: Vec<_> =
            (expected_banks * width.map_or(1, |width| width.lanes() as usize)..10)
                .map(|index| format!("eq{index}"))
                .collect();
        assert_eq!(actual_tails, expected_tails, "scalar tail order is stable");
        assert_eq!(scalar_artifact.graph.prepared_bank_count(), 0);
        let lanes = width.map_or(0_u64, |width| u64::from(width.lanes()));
        let bank_count = u64::try_from(expected_banks).expect("bank count");
        let quantum = u64::from(session.quantum().0);
        let expected_bank_scratch_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_runtime_buffer_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_metadata_bytes = bank_count
            * (u64::try_from(core::mem::size_of::<graph::GraphPreparedEffectBank>())
                .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .flat_map(|bank| bank.members.iter().flatten())
                .map(|member| {
                    u64::try_from(core::mem::size_of::<EffectNodeId>())
                        .expect("member metadata size")
                        + u64::try_from(member.track_id.as_str().len()).expect("track ID bytes")
                        + u64::try_from("true-peak-limiter".len()).expect("effect ID bytes")
                })
                .sum::<u64>();
        assert_eq!(artifact.report.estimate.effect_bank_count, bank_count);
        assert_eq!(
            artifact.report.estimate.effect_bank_scratch_bytes,
            expected_bank_scratch_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_runtime_buffer_bytes,
            expected_bank_runtime_buffer_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_metadata_bytes,
            expected_bank_metadata_bytes
        );
        assert_eq!(scalar_artifact.report.estimate.effect_bank_count, 0);
        assert_eq!(scalar_artifact.report.estimate.effect_bank_scratch_bytes, 0);
        assert_eq!(
            scalar_artifact
                .report
                .estimate
                .effect_bank_runtime_buffer_bytes,
            0
        );
        assert_eq!(
            scalar_artifact.report.estimate.effect_bank_metadata_bytes,
            0
        );
        assert_eq!(
            artifact.report.estimate.audio_buffer_samples,
            scalar_artifact.report.estimate.audio_buffer_samples
                + (expected_bank_scratch_bytes + expected_bank_runtime_buffer_bytes) / 4
        );
        assert_eq!(
            artifact.report.estimate.graph_metadata_bytes,
            scalar_artifact.report.estimate.graph_metadata_bytes + expected_bank_metadata_bytes
        );
        assert_eq!(
            artifact.report.estimate.incremental_plan_bytes,
            scalar_artifact.report.estimate.incremental_plan_bytes
                + expected_bank_scratch_bytes
                + expected_bank_runtime_buffer_bytes
                + expected_bank_metadata_bytes
        );
        assert_eq!(
            artifact.report.estimate.session_plus_plan_bytes,
            scalar_artifact.report.estimate.session_plus_plan_bytes
                + expected_bank_scratch_bytes
                + expected_bank_runtime_buffer_bytes
                + expected_bank_metadata_bytes
        );
        assert_eq!(
            artifact.graph.sequential_schedule,
            scalar_artifact.graph.sequential_schedule
        );
        assert_eq!(
            artifact.graph.route_timings,
            scalar_artifact.graph.route_timings
        );
        assert_eq!(
            artifact.graph.inserted_delays,
            scalar_artifact.graph.inserted_delays
        );
        assert_eq!(
            GraphCompiler::evidence(&artifact.graph, &artifact.report).canonical_bytes,
            GraphCompiler::evidence(&scalar_artifact.graph, &scalar_artifact.report)
                .canonical_bytes
        );
        assert_eq!(artifact.report.output_latency, LatencySamples(486));
        assert_eq!(
            artifact.report.output_latency,
            scalar_artifact.report.output_latency
        );
        assert_eq!(
            artifact.report.output_tail,
            scalar_artifact.report.output_tail
        );
        assert!(artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(486)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(486)
        }));
        let expected_schedule = artifact.graph.sequential_schedule.clone();
        let expected_route_timings = artifact.graph.route_timings.clone();
        let expected_delays = artifact.graph.inserted_delays.clone();
        let expected_canonical_bytes = GraphCompiler::evidence(&artifact.graph, &artifact.report)
            .canonical_bytes
            .clone();
        let expected_output_latency = artifact.report.output_latency;
        let expected_output_tail = artifact.report.output_tail;
        let minimum_plan_bytes = artifact.report.estimate.incremental_plan_bytes;

        let PreparedGraphArtifact {
            pool_classes: _,
            graph: bank_graph,
            report: _,
        } = artifact;
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: scalar_graph,
            report: _,
        } = scalar_artifact;
        let envelope = bank_graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let bank_nodes = bank_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), true_peak_limiter_input_binding(node)))
            .collect();
        let scalar_nodes = scalar_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), true_peak_limiter_input_binding(node)))
            .collect();
        let mut bank_plan = bank_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("limiter bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("limiter scalar bind: {}", failure.code));
        let mut reached_fixed_latency = false;
        for block in 0..16_u64 {
            let mut bank_pcm = vec![0.0_f32; frames * 2];
            let mut scalar_pcm = vec![0.0_f32; frames * 2];
            bank_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut bank_pcm, 2, frames, frames)
                            .expect("bank output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("bank render");
            scalar_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                            .expect("scalar output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("scalar render");
            assert_eq!(
                bank_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                scalar_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "bank, tails, and scalar limiter render exact carried release state"
            );
            for frame in 0..frames {
                let absolute = block * frames as u64 + frame as u64;
                let left = bank_pcm[frame];
                let right = bank_pcm[frames + frame];
                if absolute < 486 {
                    assert_eq!(left, 0.0, "left output before fixed limiter latency");
                    assert_eq!(right, 0.0, "right output before fixed limiter latency");
                }
                if absolute == 486 {
                    reached_fixed_latency = left != 0.0 && right != 0.0;
                }
            }
        }
        assert!(
            reached_fixed_latency,
            "one-shot limiter input first appears at the frozen T=486 samples"
        );

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.simd1.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled bypass limiter fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass limiter effects");
        assert!(
            bypass_effects
                .entries
                .iter()
                .all(|entry| entry.metadata.latency == LatencySamples(486))
        );
        let bypass_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_052,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bypass limiter graph: {:?}", failure.diagnostics));
        assert_eq!(bypass_artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(bypass_artifact.graph.sequential_schedule, expected_schedule);
        assert_eq!(bypass_artifact.graph.route_timings, expected_route_timings);
        assert_eq!(bypass_artifact.graph.inserted_delays, expected_delays);
        assert_eq!(
            bypass_artifact.report.output_latency,
            expected_output_latency
        );
        assert_eq!(bypass_artifact.report.output_tail, expected_output_tail);
        assert_eq!(
            GraphCompiler::evidence(&bypass_artifact.graph, &bypass_artifact.report)
                .canonical_bytes,
            expected_canonical_bytes
        );
        assert!(bypass_artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(486)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(486)
        }));

        let cap_effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared limiter effects for transactional cap");
        let mut constrained_caps = integration_caps();
        constrained_caps.maximum_plan_bytes = minimum_plan_bytes
            .checked_sub(1)
            .expect("nonzero full graph plan estimate");
        let cap_failure = match GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_053,
            effects: cap_effects,
            caps: constrained_caps,
        }) {
            Ok(_) => panic!("one-byte-below limiter graph cap must reject before publication"),
            Err(failure) => failure,
        };
        assert!(
            cap_failure
                .diagnostics
                .diagnostics()
                .iter()
                .any(|diagnostic| {
                    diagnostic.code == "graph.resource.limit"
                        && diagnostic.path == "$.graph_compile_caps"
                })
        );
        assert_eq!(cap_failure.effects.entries.len(), 10);
        assert_eq!(
            cap_failure.effects.session.normalized_model().tracks.len(),
            10,
            "cap rejection returns every prepared limiter input"
        );
    }

    #[test]
    fn launch_multiband_compressor_fixture_closes_bank_graph_and_transactional_caps() {
        let model = accepted_multiband_compressor_graph_fixture();
        assert_eq!(model.tracks.len(), 10);
        assert!(
            model.tracks.iter().all(|track| matches!(
                track.simd1.effects[0].sidechain,
                SidechainDeclaration::None
            ))
        );
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("accepted multiband-compressor fixture");
        assert_eq!(session.sample_rate().0, 48_000);
        assert_eq!(session.quantum().0, 128);

        let registry = launch_native_effect_registry().expect("launch registry");
        let multiband = registry
            .get_shared_ascii("miso.multiband-compressor")
            .expect("registered multiband compressor");
        let scalar_registry = NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory {
            delegate: multiband,
        })
            as Box<dyn NativeEffectFactory>])
        .expect("scalar multiband-compressor registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared bank-capable multiband effects");
        assert_eq!(effects.entries.len(), 10);
        assert!(effects.entries.iter().all(|entry| {
            entry.metadata.latency == LatencySamples(960)
                && entry.metadata.tail == TailSamples::Infinite
                && matches!(entry.metadata.ports.sidechain, PreparedSidechainPort::None)
        }));
        let scalar_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared scalar multiband effects");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_080,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("multiband graph: {:?}", failure.diagnostics));
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_081,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar multiband graph: {:?}", failure.diagnostics));

        let width = BankWidth::for_backend(artifact.report.rack_cohorts.dispatch);
        let (expected_banks, expected_scalar_tails) = width.map_or((0, 10), |width| {
            let lanes = width.lanes() as usize;
            (10 / lanes, 10 % lanes)
        });
        assert_eq!(artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members: Vec<Vec<String>> = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocation::Simd1)
            .map(|bank| {
                bank.members
                    .iter()
                    .flatten()
                    .map(|member| member.track_id.as_str().to_owned())
                    .collect()
            })
            .collect();
        let expected_members: Vec<Vec<String>> = width.map_or_else(Vec::new, |width| {
            let lanes = width.lanes() as usize;
            (0..expected_banks)
                .map(|bank| {
                    (bank * lanes..(bank + 1) * lanes)
                        .map(|index| format!("eq{index}"))
                        .collect()
                })
                .collect()
        });
        assert_eq!(
            actual_members, expected_members,
            "stable full-bank membership"
        );
        let expected_tail_start = expected_banks * width.map_or(1, |width| width.lanes() as usize);
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .iter()
                .map(|tail| tail.track_id.as_str().to_owned())
                .collect::<Vec<_>>(),
            (expected_tail_start..10)
                .map(|index| format!("eq{index}"))
                .collect::<Vec<_>>(),
            "stable scalar-tail membership"
        );
        assert_eq!(scalar_artifact.graph.prepared_bank_count(), 0);

        let lanes = width.map_or(0_u64, |width| u64::from(width.lanes()));
        let bank_count = u64::try_from(expected_banks).expect("bank count");
        let quantum = u64::from(session.quantum().0);
        let expected_bank_scratch_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_runtime_buffer_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_metadata_bytes = bank_count
            * (u64::try_from(core::mem::size_of::<graph::GraphPreparedEffectBank>())
                .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .flat_map(|bank| bank.members.iter().flatten())
                .map(|member| {
                    u64::try_from(core::mem::size_of::<EffectNodeId>())
                        .expect("member metadata size")
                        + u64::try_from(member.track_id.as_str().len()).expect("track ID bytes")
                        + u64::try_from("multiband-compressor".len()).expect("effect ID bytes")
                })
                .sum::<u64>();
        assert_eq!(artifact.report.estimate.effect_bank_count, bank_count);
        assert_eq!(
            artifact.report.estimate.effect_bank_scratch_bytes,
            expected_bank_scratch_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_runtime_buffer_bytes,
            expected_bank_runtime_buffer_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_metadata_bytes,
            expected_bank_metadata_bytes
        );
        assert_eq!(scalar_artifact.report.estimate.effect_bank_count, 0);
        assert_eq!(
            artifact.report.estimate.incremental_plan_bytes,
            scalar_artifact.report.estimate.incremental_plan_bytes
                + expected_bank_scratch_bytes
                + expected_bank_runtime_buffer_bytes
                + expected_bank_metadata_bytes
        );
        assert_eq!(
            artifact.report.estimate.session_plus_plan_bytes,
            scalar_artifact.report.estimate.session_plus_plan_bytes
                + expected_bank_scratch_bytes
                + expected_bank_runtime_buffer_bytes
                + expected_bank_metadata_bytes
        );
        assert_eq!(
            artifact.graph.sequential_schedule,
            scalar_artifact.graph.sequential_schedule
        );
        assert_eq!(
            artifact.graph.route_timings,
            scalar_artifact.graph.route_timings
        );
        assert_eq!(
            artifact.graph.inserted_delays,
            scalar_artifact.graph.inserted_delays
        );
        assert_eq!(
            GraphCompiler::evidence(&artifact.graph, &artifact.report).canonical_bytes,
            GraphCompiler::evidence(&scalar_artifact.graph, &scalar_artifact.report)
                .canonical_bytes
        );
        assert!(artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(960)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(960)
        }));
        let expected_schedule = artifact.graph.sequential_schedule.clone();
        let expected_route_timings = artifact.graph.route_timings.clone();
        let expected_delays = artifact.graph.inserted_delays.clone();
        let expected_canonical_bytes = GraphCompiler::evidence(&artifact.graph, &artifact.report)
            .canonical_bytes
            .clone();
        let minimum_plan_bytes = artifact.report.estimate.incremental_plan_bytes;

        let PreparedGraphArtifact {
            pool_classes: _,
            graph: bank_graph,
            ..
        } = artifact;
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: scalar_graph,
            ..
        } = scalar_artifact;
        let envelope = bank_graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let bank_nodes = bank_graph
            .required_bindings
            .iter()
            .map(|node| {
                GraphNodeBinding::new(node.clone(), multiband_compressor_input_binding(node))
            })
            .collect();
        let scalar_nodes = scalar_graph
            .required_bindings
            .iter()
            .map(|node| {
                GraphNodeBinding::new(node.clone(), multiband_compressor_input_binding(node))
            })
            .collect();
        let mut bank_plan = bank_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("multiband bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("multiband scalar bind: {}", failure.code));
        let mut reached_latency = false;
        let mut reached_release_probe = false;
        for block in 0..20_u64 {
            let mut bank_pcm = vec![0.0_f32; frames * 2];
            let mut scalar_pcm = vec![0.0_f32; frames * 2];
            bank_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut bank_pcm, 2, frames, frames)
                            .expect("bank output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("bank render");
            scalar_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                            .expect("scalar output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("scalar render");
            for (&bank, &scalar) in bank_pcm.iter().zip(&scalar_pcm) {
                assert!(bank.is_finite() && scalar.is_finite());
                let bound = 1.0e-5 + 2.0e-5 * scalar.abs();
                assert!(
                    (bank - scalar).abs() <= bound,
                    "ten-track accumulated bank/scalar error: bank={bank} scalar={scalar} bound={bound}"
                );
            }
            for frame in 0..frames {
                let absolute = block * frames as u64 + frame as u64;
                let left = bank_pcm[frame];
                let right = bank_pcm[frames + frame];
                if absolute < 960 {
                    assert_eq!(left, 0.0, "left output before fixed latency");
                    assert_eq!(right, 0.0, "right output before fixed latency");
                } else if absolute < 1_024 {
                    reached_latency |= left != 0.0 && right != 0.0;
                } else if (2_240..2_304).contains(&absolute) {
                    reached_release_probe |= left != 0.0 && right != 0.0;
                }
            }
        }
        assert!(
            reached_latency,
            "active burst crosses fixed 960-sample latency"
        );
        assert!(
            reached_release_probe,
            "later probe crosses latency while carried release state remains active"
        );

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.simd1.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled bypass multiband fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass multiband effects");
        let bypass_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_082,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bypass multiband graph: {:?}", failure.diagnostics));
        assert_eq!(bypass_artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(bypass_artifact.graph.sequential_schedule, expected_schedule);
        assert_eq!(bypass_artifact.graph.route_timings, expected_route_timings);
        assert_eq!(bypass_artifact.graph.inserted_delays, expected_delays);
        assert_eq!(
            GraphCompiler::evidence(&bypass_artifact.graph, &bypass_artifact.report)
                .canonical_bytes,
            expected_canonical_bytes
        );
        assert!(bypass_artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(960)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(960)
        }));

        let cap_effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared multiband effects for transactional cap");
        let mut constrained_caps = integration_caps();
        constrained_caps.maximum_plan_bytes = minimum_plan_bytes
            .checked_sub(1)
            .expect("nonzero full graph plan estimate");
        let cap_failure = match GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_083,
            effects: cap_effects,
            caps: constrained_caps,
        }) {
            Ok(_) => panic!("one-byte-below multiband graph cap must reject before publication"),
            Err(failure) => failure,
        };
        assert!(
            cap_failure
                .diagnostics
                .diagnostics()
                .iter()
                .any(|diagnostic| {
                    diagnostic.code == "graph.resource.limit"
                        && diagnostic.path == "$.graph_compile_caps"
                })
        );
        assert_eq!(cap_failure.effects.entries.len(), 10);
        assert_eq!(
            cap_failure.effects.session.normalized_model().tracks.len(),
            10,
            "cap rejection returns every prepared multiband input"
        );
    }

    #[test]
    fn launch_soft_clip_fixture_closes_banks_tails_pdc_support_and_transactional_caps() {
        let model = accepted_soft_clip_graph_fixture();
        assert_eq!(model.tracks.len(), 10);
        assert!(
            model.tracks.iter().all(|track| matches!(
                track.simd1.effects[0].sidechain,
                SidechainDeclaration::None
            ))
        );
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("accepted soft-clip fixture");
        assert_eq!(session.sample_rate().0, 48_000);
        assert_eq!(session.quantum().0, 128);

        let registry = launch_native_effect_registry().expect("launch registry");
        let soft_clip = registry
            .get_shared_ascii("miso.soft-clip")
            .expect("registered soft clip");
        let scalar_registry = NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory {
            delegate: soft_clip,
        })
            as Box<dyn NativeEffectFactory>])
        .expect("scalar soft-clip registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared bank-capable soft-clip effects");
        assert_eq!(effects.entries.len(), 10);
        assert!(effects.entries.iter().all(|entry| {
            entry.metadata.latency == LatencySamples(31)
                && entry.metadata.tail == TailSamples::Finite(29)
                && matches!(entry.metadata.ports.sidechain, PreparedSidechainPort::None)
        }));
        let scalar_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared scalar soft-clip effects");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_100,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("soft-clip graph: {:?}", failure.diagnostics));
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_101,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("scalar soft-clip graph: {:?}", failure.diagnostics));

        let width = BankWidth::for_backend(artifact.report.rack_cohorts.dispatch);
        let (expected_banks, expected_scalar_tails) = width.map_or((0, 10), |width| {
            let lanes = width.lanes() as usize;
            (10 / lanes, 10 % lanes)
        });
        assert_eq!(artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocation::Simd1)
            .map(|bank| {
                bank.members
                    .iter()
                    .flatten()
                    .map(|member| member.track_id.as_str().to_owned())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let expected_members = width.map_or_else(Vec::new, |width| {
            let lanes = width.lanes() as usize;
            (0..expected_banks)
                .map(|bank| {
                    (bank * lanes..(bank + 1) * lanes)
                        .map(|index| format!("eq{index}"))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(actual_members, expected_members, "stable bank membership");
        let tail_start = expected_banks * width.map_or(1, |width| width.lanes() as usize);
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .iter()
                .map(|tail| tail.track_id.as_str().to_owned())
                .collect::<Vec<_>>(),
            (tail_start..10)
                .map(|index| format!("eq{index}"))
                .collect::<Vec<_>>(),
            "stable scalar-tail order"
        );
        assert_eq!(scalar_artifact.graph.prepared_bank_count(), 0);

        let lanes = width.map_or(0_u64, |width| u64::from(width.lanes()));
        let bank_count = u64::try_from(expected_banks).expect("bank count");
        let quantum = u64::from(session.quantum().0);
        let expected_bank_scratch_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_runtime_buffer_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_metadata_bytes = bank_count
            * (u64::try_from(core::mem::size_of::<graph::GraphPreparedEffectBank>())
                .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .flat_map(|bank| bank.members.iter().flatten())
                .map(|member| {
                    u64::try_from(core::mem::size_of::<EffectNodeId>())
                        .expect("member metadata size")
                        + u64::try_from(member.track_id.as_str().len()).expect("track ID bytes")
                        + u64::try_from("soft-clip".len()).expect("effect ID bytes")
                })
                .sum::<u64>();
        assert_eq!(artifact.report.estimate.effect_bank_count, bank_count);
        assert_eq!(
            artifact.report.estimate.effect_bank_scratch_bytes,
            expected_bank_scratch_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_runtime_buffer_bytes,
            expected_bank_runtime_buffer_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_metadata_bytes,
            expected_bank_metadata_bytes
        );
        assert_eq!(scalar_artifact.report.estimate.effect_bank_count, 0);
        assert_eq!(
            artifact.report.estimate.incremental_plan_bytes,
            scalar_artifact.report.estimate.incremental_plan_bytes
                + expected_bank_scratch_bytes
                + expected_bank_runtime_buffer_bytes
                + expected_bank_metadata_bytes
        );
        assert_eq!(
            artifact.report.estimate.session_plus_plan_bytes,
            scalar_artifact.report.estimate.session_plus_plan_bytes
                + expected_bank_scratch_bytes
                + expected_bank_runtime_buffer_bytes
                + expected_bank_metadata_bytes
        );
        assert_eq!(
            artifact.graph.sequential_schedule,
            scalar_artifact.graph.sequential_schedule
        );
        assert_eq!(
            artifact.graph.route_timings,
            scalar_artifact.graph.route_timings
        );
        assert_eq!(
            artifact.graph.inserted_delays,
            scalar_artifact.graph.inserted_delays
        );
        assert_eq!(
            GraphCompiler::evidence(&artifact.graph, &artifact.report).canonical_bytes,
            GraphCompiler::evidence(&scalar_artifact.graph, &scalar_artifact.report)
                .canonical_bytes
        );
        assert!(artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(31)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(31)
        }));
        let expected_schedule = artifact.graph.sequential_schedule.clone();
        let expected_route_timings = artifact.graph.route_timings.clone();
        let expected_delays = artifact.graph.inserted_delays.clone();
        let expected_canonical_bytes = GraphCompiler::evidence(&artifact.graph, &artifact.report)
            .canonical_bytes
            .clone();
        let minimum_plan_bytes = artifact.report.estimate.incremental_plan_bytes;

        let PreparedGraphArtifact {
            pool_classes: _,
            graph: bank_graph,
            ..
        } = artifact;
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: scalar_graph,
            ..
        } = scalar_artifact;
        let envelope = bank_graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let bank_nodes = bank_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), soft_clip_input_binding(node)))
            .collect();
        let scalar_nodes = scalar_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), soft_clip_input_binding(node)))
            .collect();
        let mut bank_plan = bank_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("soft-clip bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("soft-clip scalar bind: {}", failure.code));
        for block in 0..2_u64 {
            let mut bank_pcm = vec![0.0_f32; frames * 2];
            let mut scalar_pcm = vec![0.0_f32; frames * 2];
            bank_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut bank_pcm, 2, frames, frames)
                            .expect("bank output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("bank render");
            scalar_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                            .expect("scalar output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("scalar render");
            assert_eq!(
                bank_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                scalar_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "bank/tails and scalar delegates preserve consecutive carried state"
            );
            if block == 0 {
                let left = &bank_pcm[..frames];
                let right = &bank_pcm[frames..];
                let left_peak = left
                    .iter()
                    .enumerate()
                    .max_by(|(_, left), (_, right)| left.abs().total_cmp(&right.abs()))
                    .map(|(index, _)| index)
                    .expect("left output");
                let right_peak = right
                    .iter()
                    .enumerate()
                    .max_by(|(_, left), (_, right)| left.abs().total_cmp(&right.abs()))
                    .map(|(index, _)| index)
                    .expect("right output");
                assert_eq!(left_peak, 31);
                assert_eq!(right_peak, 31);
                assert_ne!(left[60].to_bits(), 0.0_f32.to_bits());
                assert_ne!(right[60].to_bits(), 0.0_f32.to_bits());
                assert!(left[61..].iter().all(|sample| *sample == 0.0));
                assert!(right[61..].iter().all(|sample| *sample == 0.0));
            } else {
                assert!(bank_pcm.iter().all(|sample| *sample == 0.0));
            }
        }

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.simd1.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled bypass soft-clip fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass soft-clip effects");
        assert!(bypass_effects.entries.iter().all(|entry| {
            entry.metadata.latency == LatencySamples(31)
                && entry.metadata.tail == TailSamples::Finite(29)
        }));
        let bypass_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_102,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bypass soft-clip graph: {:?}", failure.diagnostics));
        assert_eq!(bypass_artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(bypass_artifact.graph.sequential_schedule, expected_schedule);
        assert_eq!(bypass_artifact.graph.route_timings, expected_route_timings);
        assert_eq!(bypass_artifact.graph.inserted_delays, expected_delays);
        assert_eq!(
            GraphCompiler::evidence(&bypass_artifact.graph, &bypass_artifact.report)
                .canonical_bytes,
            expected_canonical_bytes
        );
        assert!(bypass_artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(31)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(31)
        }));

        let cap_effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared soft-clip effects for transactional cap");
        let mut constrained_caps = integration_caps();
        constrained_caps.maximum_plan_bytes = minimum_plan_bytes
            .checked_sub(1)
            .expect("nonzero full graph plan estimate");
        let cap_failure = match GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_103,
            effects: cap_effects,
            caps: constrained_caps,
        }) {
            Ok(_) => panic!("one-byte-below soft-clip graph cap must reject before publication"),
            Err(failure) => failure,
        };
        assert!(
            cap_failure
                .diagnostics
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.code == "graph.resource.limit"
                    && diagnostic.path == "$.graph_compile_caps")
        );
        assert_eq!(cap_failure.effects.entries.len(), 10);
        assert_eq!(
            cap_failure.effects.session.normalized_model().tracks.len(),
            10,
            "cap rejection returns every prepared soft-clip input"
        );
    }

    #[test]
    fn launch_transient_shaper_fixture_closes_banks_tails_pdc_and_transactional_caps() {
        let model = accepted_transient_shaper_graph_fixture();
        assert_eq!(model.tracks.len(), 10);
        assert!(
            model.tracks.iter().all(|track| matches!(
                track.simd1.effects[0].sidechain,
                SidechainDeclaration::None
            ))
        );
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("accepted transient-shaper fixture");
        assert_eq!(session.sample_rate().0, 48_000);
        assert_eq!(session.quantum().0, 128);

        let registry = launch_native_effect_registry().expect("launch registry");
        let transient_shaper = registry
            .get_shared_ascii("miso.transient-shaper")
            .expect("registered transient shaper");
        let scalar_registry = NativeEffectRegistry::new([Box::new(ScalarOnlyDelegateFactory {
            delegate: transient_shaper,
        })
            as Box<dyn NativeEffectFactory>])
        .expect("scalar transient-shaper registry");
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        };
        let effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared bank-capable transient-shaper effects");
        assert_eq!(effects.entries.len(), 10);
        assert!(effects.entries.iter().all(|entry| {
            entry.metadata.latency == LatencySamples(0)
                && entry.metadata.tail == TailSamples::Finite(0)
                && matches!(entry.metadata.ports.sidechain, PreparedSidechainPort::None)
        }));
        let scalar_effects =
            prepare_native_session_effects(&session, &scalar_registry, effect_caps)
                .expect("prepared scalar transient-shaper effects");
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_120,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("transient-shaper graph: {:?}", failure.diagnostics));
        let scalar_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_121,
            effects: scalar_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| {
            panic!("scalar transient-shaper graph: {:?}", failure.diagnostics)
        });

        let width = BankWidth::for_backend(artifact.report.rack_cohorts.dispatch);
        let (expected_banks, expected_scalar_tails) = width.map_or((0, 10), |width| {
            let lanes = width.lanes() as usize;
            (10 / lanes, 10 % lanes)
        });
        assert_eq!(artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocation::Simd1)
            .map(|bank| {
                bank.members
                    .iter()
                    .flatten()
                    .map(|member| member.track_id.as_str().to_owned())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let expected_members = width.map_or_else(Vec::new, |width| {
            let lanes = width.lanes() as usize;
            (0..expected_banks)
                .map(|bank| {
                    (bank * lanes..(bank + 1) * lanes)
                        .map(|index| format!("eq{index}"))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(actual_members, expected_members, "stable bank membership");
        let tail_start = expected_banks * width.map_or(1, |width| width.lanes() as usize);
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Simd1)
                .iter()
                .map(|tail| tail.track_id.as_str().to_owned())
                .collect::<Vec<_>>(),
            (tail_start..10)
                .map(|index| format!("eq{index}"))
                .collect::<Vec<_>>(),
            "stable scalar-tail order"
        );
        assert_eq!(scalar_artifact.graph.prepared_bank_count(), 0);

        let lanes = width.map_or(0_u64, |width| u64::from(width.lanes()));
        let bank_count = u64::try_from(expected_banks).expect("bank count");
        let quantum = u64::from(session.quantum().0);
        let expected_bank_scratch_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_runtime_buffer_bytes = bank_count * lanes * quantum * 2 * 4;
        let expected_bank_metadata_bytes = bank_count
            * (u64::try_from(core::mem::size_of::<graph::GraphPreparedEffectBank>())
                .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocation::Simd1)
                .flat_map(|bank| bank.members.iter().flatten())
                .map(|member| {
                    u64::try_from(core::mem::size_of::<EffectNodeId>())
                        .expect("member metadata size")
                        + u64::try_from(member.track_id.as_str().len()).expect("track ID bytes")
                        + u64::try_from("transient-shaper".len()).expect("effect ID bytes")
                })
                .sum::<u64>();
        assert_eq!(artifact.report.estimate.effect_bank_count, bank_count);
        assert_eq!(
            artifact.report.estimate.effect_bank_scratch_bytes,
            expected_bank_scratch_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_runtime_buffer_bytes,
            expected_bank_runtime_buffer_bytes
        );
        assert_eq!(
            artifact.report.estimate.effect_bank_metadata_bytes,
            expected_bank_metadata_bytes
        );
        assert_eq!(scalar_artifact.report.estimate.effect_bank_count, 0);
        let bank_overhead = expected_bank_scratch_bytes
            + expected_bank_runtime_buffer_bytes
            + expected_bank_metadata_bytes;
        assert_eq!(
            artifact.report.estimate.incremental_plan_bytes,
            scalar_artifact.report.estimate.incremental_plan_bytes + bank_overhead
        );
        assert_eq!(
            artifact.report.estimate.session_plus_plan_bytes,
            scalar_artifact.report.estimate.session_plus_plan_bytes + bank_overhead
        );
        assert_eq!(
            artifact.graph.sequential_schedule,
            scalar_artifact.graph.sequential_schedule
        );
        assert_eq!(
            artifact.graph.route_timings,
            scalar_artifact.graph.route_timings
        );
        assert_eq!(
            artifact.graph.inserted_delays,
            scalar_artifact.graph.inserted_delays
        );
        assert_eq!(
            GraphCompiler::evidence(&artifact.graph, &artifact.report).canonical_bytes,
            GraphCompiler::evidence(&scalar_artifact.graph, &scalar_artifact.report)
                .canonical_bytes
        );
        assert!(artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(0)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(0)
        }));
        let expected_schedule = artifact.graph.sequential_schedule.clone();
        let expected_route_timings = artifact.graph.route_timings.clone();
        let expected_delays = artifact.graph.inserted_delays.clone();
        let expected_canonical_bytes = GraphCompiler::evidence(&artifact.graph, &artifact.report)
            .canonical_bytes
            .clone();
        let minimum_plan_bytes = artifact.report.estimate.incremental_plan_bytes;

        let PreparedGraphArtifact {
            pool_classes: _,
            graph: bank_graph,
            ..
        } = artifact;
        let PreparedGraphArtifact {
            pool_classes: _,
            graph: scalar_graph,
            ..
        } = scalar_artifact;
        let envelope = bank_graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let bank_nodes = bank_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), transient_shaper_input_binding(node)))
            .collect();
        let scalar_nodes = scalar_graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), transient_shaper_input_binding(node)))
            .collect();
        let mut bank_plan = bank_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("transient bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: scalar_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("transient scalar bind: {}", failure.code));
        for block in 0..2_u64 {
            let mut bank_pcm = vec![0.0_f32; frames * 2];
            let mut scalar_pcm = vec![0.0_f32; frames * 2];
            bank_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut bank_pcm, 2, frames, frames)
                            .expect("bank output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("bank render");
            scalar_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut scalar_pcm, 2, frames, frames)
                            .expect("scalar output"),
                    },
                    RenderTime {
                        absolute_sample: block * frames as u64,
                    },
                )
                .expect("scalar render");
            assert_eq!(
                bank_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                scalar_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "consecutive bank/tail PCM and carried state match scalar delegates"
            );
            assert!(bank_pcm.iter().any(|sample| *sample != 0.0));
        }

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.simd1.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled bypass transient-shaper fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass transient-shaper effects");
        let bypass_artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_122,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| {
            panic!("bypass transient-shaper graph: {:?}", failure.diagnostics)
        });
        assert_eq!(bypass_artifact.graph.prepared_bank_count(), expected_banks);
        assert_eq!(bypass_artifact.graph.sequential_schedule, expected_schedule);
        assert_eq!(bypass_artifact.graph.route_timings, expected_route_timings);
        assert_eq!(bypass_artifact.graph.inserted_delays, expected_delays);
        assert_eq!(
            GraphCompiler::evidence(&bypass_artifact.graph, &bypass_artifact.report)
                .canonical_bytes,
            expected_canonical_bytes
        );
        assert!(bypass_artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(0)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(0)
        }));

        let cap_effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared transient-shaper effects for transactional cap");
        let mut constrained_caps = integration_caps();
        constrained_caps.maximum_plan_bytes = minimum_plan_bytes
            .checked_sub(1)
            .expect("nonzero full graph plan estimate");
        let cap_failure = match GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_123,
            effects: cap_effects,
            caps: constrained_caps,
        }) {
            Ok(_) => panic!("one-byte-below transient graph cap must reject before publication"),
            Err(failure) => failure,
        };
        assert!(
            cap_failure
                .diagnostics
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.code == "graph.resource.limit"
                    && diagnostic.path == "$.graph_compile_caps")
        );
        assert_eq!(cap_failure.effects.entries.len(), 10);
        assert_eq!(
            cap_failure.effects.session.normalized_model().tracks.len(),
            10,
            "cap rejection returns every prepared transient-shaper input"
        );
    }

    #[test]
    fn launch_delay_fixture_closes_scalar_state_tail_pdc_and_transactional_caps() {
        let model = accepted_delay_graph_fixture();
        assert_eq!(model.tracks.len(), 10);
        assert!(model.tracks.iter().all(|track| {
            track.simd1.effects.is_empty()
                && track.dynamic.effects.len() == 1
                && matches!(
                    track.dynamic.effects[0].sidechain,
                    SidechainDeclaration::None
                )
        }));
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("accepted delay fixture");
        assert_eq!(session.sample_rate().0, 48_000);
        assert_eq!(session.quantum().0, 128);

        let registry = launch_native_effect_registry().expect("launch registry");
        assert!(registry.get_ascii("miso.delay").is_some());
        let effect_caps = EffectCompileCaps {
            maximum_total_state_bytes: 768_168,
            maximum_scratch_bytes: 36,
            maximum_automation_spans_per_block: 32,
        };
        let effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared delay effects");
        let mut direct = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared direct scalar delays");
        assert_eq!(effects.entries.len(), 10);
        assert!(effects.entries.iter().all(|entry| {
            entry.rack == effect_compiler::EffectRack::Dynamic
                && entry.metadata.latency == LatencySamples(0)
                && entry.metadata.tail == TailSamples::Infinite
                && entry.metadata.state_sizes.total() == Some(768_168)
                && entry.metadata.scratch_bytes == 36
                && matches!(entry.metadata.ports.sidechain, PreparedSidechainPort::None)
        }));

        let artifact = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_130,
            effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("delay graph: {:?}", failure.diagnostics));
        assert_eq!(artifact.graph.prepared_bank_count(), 0);
        // Every delay lives in the dynamic rack, so neither SIMD rack has a candidate at all: the
        // planner sees an empty pool and produces no groups and no scalar members.
        for rack in [RackLocation::Simd1, RackLocation::Simd2] {
            assert_eq!(artifact.report.rack_cohorts.groups_in(rack).count(), 0);
            assert!(artifact.report.rack_cohorts.scalar_in(rack).is_empty());
        }
        // The dynamic rack *is* a bank location now, so this fixture is the gate on the thing that
        // actually disqualifies a bank: the kernel contract, not the rack. Ten identical
        // sidechain-free `conformance.delay` chains are a perfectly homogeneous cohort -- the
        // planner forms full groups for them -- and every one of them still renders per node,
        // because `DualAccumulatorDelayFactory::bind_homogeneous_bank` returns `Ok(None)`. If
        // candidacy ever bound a bank from mere homogeneity, `prepared_bank_count` above and
        // `scalar_in(Dynamic)` below both move.
        assert!(
            artifact
                .report
                .rack_cohorts
                .groups_in(RackLocation::Dynamic)
                .count()
                > 0,
            "the dynamic rack must reach the planner, or this fixture gates nothing"
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .bound_slots_in(RackLocation::Dynamic)
                .count(),
            0,
            "an effect with no homogeneous bank kernel must bind no bank in any rack"
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocation::Dynamic)
                .len(),
            10,
            "every delay stays on the per-node path"
        );
        assert_eq!(artifact.report.estimate.effect_bank_count, 0);
        assert_eq!(artifact.report.estimate.effect_bank_scratch_bytes, 0);
        assert_eq!(artifact.report.estimate.effect_bank_runtime_buffer_bytes, 0);
        assert_eq!(artifact.report.estimate.effect_bank_metadata_bytes, 0);
        assert_eq!(artifact.report.estimate.effects, 10);
        assert_eq!(artifact.report.estimate.declared_effect_bytes, 7_682_040);
        assert_eq!(artifact.report.output_latency, LatencySamples(0));
        assert_eq!(artifact.report.output_tail, TailSamples::Infinite);
        let effect_nodes = artifact
            .graph
            .spec
            .nodes
            .iter()
            .filter(|node| matches!(node.id, GraphNodeId::Effect(_)))
            .collect::<Vec<_>>();
        assert_eq!(effect_nodes.len(), 10);
        assert!(effect_nodes.iter().all(|node| {
            node.latency == LatencySamples(0)
                && node.tail == TailSamples::Infinite
                && matches!(&node.id, GraphNodeId::Effect(id) if id.rack == RackId::Dynamic)
        }));
        assert!(artifact.graph.route_timings.iter().all(|route| {
            route.source_arrival == LatencySamples(0)
                && route.compensation_delay == LatencySamples(0)
                && route.destination_arrival == LatencySamples(0)
        }));
        assert!(artifact.graph.inserted_delays.is_empty());
        let dynamic_order = artifact
            .graph
            .sequential_schedule
            .iter()
            .filter_map(|node| match node {
                GraphNodeId::Effect(id) if id.rack == RackId::Dynamic => {
                    Some(id.track_id.as_str().to_owned())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            dynamic_order,
            (0..10)
                .map(|index| format!("eq{index}"))
                .collect::<Vec<_>>()
        );
        let expected_schedule = artifact.graph.sequential_schedule.clone();
        let expected_route_timings = artifact.graph.route_timings.clone();
        let expected_delays = artifact.graph.inserted_delays.clone();
        let expected_canonical = GraphCompiler::evidence(&artifact.graph, &artifact.report)
            .canonical_bytes
            .clone();
        let minimum_plan_bytes = artifact.report.estimate.incremental_plan_bytes;

        let PreparedGraphArtifact { graph, .. } = artifact;
        let envelope = graph.envelope;
        let frames = envelope.quantum.0 as usize;
        let nodes = graph
            .required_bindings
            .iter()
            .map(|node| GraphNodeBinding::new(node.clone(), delay_input_binding(node)))
            .collect();
        let mut plan = graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("delay bind: {}", failure.code));
        assert_eq!(
            direct
                .entries
                .iter()
                .map(|entry| entry.track_id.clone())
                .collect::<Vec<_>>(),
            (0..10)
                .map(|index| format!("eq{index}"))
                .collect::<Vec<_>>()
        );
        for block in 0..2_u64 {
            let mut graph_pcm = vec![0.0_f32; frames * 2];
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut graph_pcm, 2, frames, frames)
                        .expect("delay graph output"),
                },
                RenderTime {
                    absolute_sample: block * frames as u64,
                },
            )
            .expect("delay graph render");

            let mut direct_tracks_left = Vec::with_capacity(10);
            let mut direct_tracks_right = Vec::with_capacity(10);
            for entry in &mut direct.entries {
                let index = entry
                    .track_id
                    .strip_prefix("eq")
                    .and_then(|value| value.parse::<u32>().ok())
                    .expect("direct delay track id");
                let mut left = vec![0.0_f32; frames];
                let mut right = vec![0.0_f32; frames];
                if block == 0 {
                    left[0] = 0.05 * (index + 1) as f32;
                    right[0] = -0.025 * (10 - index) as f32;
                }
                let report = entry.processor.process(
                    EffectProcessBlock::new(
                        &mut left,
                        &mut right,
                        None,
                        block * frames as u64,
                        &[],
                        128,
                    )
                    .expect("direct delay block"),
                );
                assert_eq!(report, ProcessReport::default());
                direct_tracks_left.push(left);
                direct_tracks_right.push(right);
            }
            let mut direct_left = vec![0.0_f32; frames];
            let mut direct_right = vec![0.0_f32; frames];
            for frame in 0..frames {
                let mut left = [0.0_f32; 10];
                let mut right = [0.0_f32; 10];
                for track in 0..10 {
                    left[track] = direct_tracks_left[track][frame];
                    right[track] = direct_tracks_right[track][frame];
                }
                // The independent scalar oracle for D9: stable track order, left to right.
                direct_left[frame] = left.iter().copied().reduce(|a, b| a + b).unwrap_or(0.0);
                direct_right[frame] = right.iter().copied().reduce(|a, b| a + b).unwrap_or(0.0);
            }
            assert_eq!(
                graph_pcm[..frames]
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                direct_left
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                graph_pcm[frames..]
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                direct_right
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>()
            );
            assert!(graph_pcm.iter().any(|sample| *sample != 0.0));
        }
        for entry in &direct.entries {
            let sizes = entry.metadata.state_sizes;
            let mut common = vec![0; sizes.common_bytes as usize];
            let mut left = vec![0; sizes.left_bytes as usize];
            let mut right = vec![0; sizes.right_bytes as usize];
            entry
                .processor
                .snapshot_state_payload(
                    StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
                        .expect("direct delay state output"),
                )
                .expect("direct delay snapshot");
            assert_eq!(
                u32::from_le_bytes(common[..4].try_into().expect("cursor")),
                256
            );
            assert_eq!(
                u32::from_le_bytes(left[24..28].try_into().expect("left valid history")),
                256
            );
            assert_eq!(
                u32::from_le_bytes(right[24..28].try_into().expect("right valid history")),
                256
            );
        }

        let mut bypass_model = model.clone();
        for track in &mut bypass_model.tracks {
            track.dynamic.effects[0].bypass = true;
        }
        let bypass_session = compile_session(
            &bypass_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled bypass delay fixture");
        let bypass_effects =
            prepare_native_session_effects(&bypass_session, &registry, effect_caps)
                .expect("prepared bypass delays");
        let bypass = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_131,
            effects: bypass_effects,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("bypass delay graph: {:?}", failure.diagnostics));
        assert_eq!(bypass.graph.prepared_bank_count(), 0);
        assert_eq!(bypass.graph.sequential_schedule, expected_schedule);
        assert_eq!(bypass.graph.route_timings, expected_route_timings);
        assert_eq!(bypass.graph.inserted_delays, expected_delays);
        assert_eq!(
            GraphCompiler::evidence(&bypass.graph, &bypass.report).canonical_bytes,
            expected_canonical
        );

        let cap_effects = prepare_native_session_effects(&session, &registry, effect_caps)
            .expect("prepared delay effects for transactional cap");
        let mut constrained = integration_caps();
        constrained.maximum_plan_bytes = minimum_plan_bytes
            .checked_sub(1)
            .expect("nonzero delay plan estimate");
        let failure = match GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1_132,
            effects: cap_effects,
            caps: constrained,
        }) {
            Ok(_) => panic!("one-byte-below delay graph cap must reject"),
            Err(failure) => failure,
        };
        assert!(failure.diagnostics.diagnostics().iter().any(|diagnostic| {
            diagnostic.code == "graph.resource.limit" && diagnostic.path == "$.graph_compile_caps"
        }));
        assert_eq!(failure.effects.entries.len(), 10);
        assert_eq!(failure.effects.session.normalized_model().tracks.len(), 10);
    }

    #[test]
    fn builtins_replace_only_the_three_internal_track_bindings() {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
        model.tracks[0].dynamic.effects.clear();
        model.automation.clear();
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled");
        let builtins = prepare_session_builtins(
            &compiled,
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
        .expect("builtins");
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 77,
            effects: EffectPreparedSession {
                session: compiled,
                entries: Vec::new(),
            },
            builtins,
            caps: integration_caps(),
        })
        .unwrap_or_else(|_| panic!("graph"));
        assert_eq!(artifact.external_binding_nodes().count(), 2);
        assert_eq!(artifact.report().output_tail, TailSamples::Infinite);
        let tail = artifact
            .graph()
            .spec
            .nodes
            .iter()
            .find(|node| node.id == track_node("vocal", TrackStage::PostInputBuiltins))
            .expect("input builtins node")
            .tail;
        assert_eq!(tail, TailSamples::Infinite);
    }

    #[test]
    fn production_builtin_banks_replace_full_post_input_groups_and_render() {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..12)
            .map(|index| {
                let mut track = base_track.clone();
                track.id = StableId::parse(&format!("bank{index}")).expect("id");
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
                track.simd2.effects.clear();
                track
            })
            .collect();
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut route = base_route.clone();
                route.id = StableId::parse(&format!("builtin-route-{index}")).expect("route id");
                route.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                route
            })
            .collect();
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled");
        let prepare_artifact = |plan_id, session: session::CompiledSession| {
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
            .expect("builtins");
            GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                dispatch: host_dispatch(),
                plan_id,
                effects: EffectPreparedSession {
                    session,
                    entries: Vec::new(),
                },
                builtins,
                caps: integration_caps(),
            })
            .unwrap_or_else(|_| panic!("graph"))
        };
        let artifact = prepare_artifact(78, compiled.clone());
        let repeat_artifact = prepare_artifact(79, compiled);
        let dispatch = Backend::current();
        // #86 F3: `T.div_ceil(W)` banks per bankable stage, the last one of each padded with
        // identity lanes, and no scalar post-input tail on a vector host. Three bankable stages
        // since #212 -- post-input builtins, fader, matrix -- all grouping the same twelve tracks.
        let expected_banks = BankWidth::for_backend(dispatch).map_or(0, |width| {
            BANKABLE_TRACK_STAGES as usize * 12_usize.div_ceil(width.lanes() as usize)
        });
        let expected_tail = BankWidth::for_backend(dispatch).map_or(12, |_| 0);
        assert_eq!(artifact.prepared_builtin_bank_count(), expected_banks);
        assert_eq!(
            repeat_artifact.prepared_builtin_bank_count(),
            expected_banks
        );
        assert_eq!(
            artifact.graph().sequential_schedule,
            artifact
                .graph()
                .dependency_levels
                .iter()
                .flat_map(|level| level.nodes.iter().cloned())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            artifact.graph().sequential_schedule,
            repeat_artifact.graph().sequential_schedule
        );
        let assigned: BTreeMap<_, _> = artifact
            .graph()
            .buffer_assignments
            .iter()
            .map(|assignment| (assignment.port.node.clone(), assignment.buffer_index))
            .collect();
        for bank in artifact.prepared_builtin_banks() {
            let colors: BTreeSet<_> = bank.members.iter().map(|member| assigned[member]).collect();
            assert_eq!(
                colors.len(),
                bank.members.len(),
                "simultaneously active builtin-bank members have distinct colors"
            );
        }
        // Membership is checked per bankable stage. Every bank names exactly one stage on every
        // lane -- `with_builtin_banks` refuses a bank that mixes them -- and each stage banks the
        // same twelve tracks, so the three per-stage member lists are identical.
        let mut member_ids_by_stage: BTreeMap<TrackStage, Vec<String>> = BTreeMap::new();
        for bank in artifact.prepared_builtin_banks() {
            assert_eq!(bank.backend, dispatch);
            assert_eq!(Some(bank.width), BankWidth::for_backend(dispatch));
            assert!(!bank.members.is_empty());
            assert!(bank.members.len() <= bank.width.lanes() as usize);
            let mut stage_of_bank = None;
            for member in bank.members.iter() {
                let GraphNodeId::TrackStage { track_id, stage } = member else {
                    panic!("builtin bank member kind");
                };
                assert!(
                    matches!(
                        stage,
                        TrackStage::PostInputBuiltins
                            | TrackStage::PostFader
                            | TrackStage::PostMatrix
                    ),
                    "a builtin bank may only name a bankable track stage"
                );
                assert_eq!(
                    *stage_of_bank.get_or_insert(*stage),
                    *stage,
                    "one bank renders one stage on every lane"
                );
                member_ids_by_stage
                    .entry(*stage)
                    .or_default()
                    .push(track_id.as_str().to_owned());
            }
        }
        let mut expected_member_ids: Vec<_> = (0..12).map(|index| format!("bank{index}")).collect();
        expected_member_ids.sort();
        if BankWidth::for_backend(dispatch).is_none() {
            expected_member_ids.clear();
            assert!(member_ids_by_stage.is_empty());
        } else {
            assert_eq!(member_ids_by_stage.len(), BANKABLE_TRACK_STAGES as usize);
            for (stage, member_ids) in &member_ids_by_stage {
                assert_eq!(
                    *member_ids, expected_member_ids,
                    "stage {stage:?} membership"
                );
            }
        }
        assert_eq!(12 - expected_member_ids.len(), expected_tail);
        let resource = artifact.graph_resource_estimate();
        assert_eq!(resource.builtin_bank_count, expected_banks as u64);
        if expected_banks != 0 {
            assert!(resource.builtin_bank_bytes != 0);
            let width = u64::from(
                BankWidth::for_backend(dispatch)
                    .expect("bank width")
                    .lanes(),
            );
            // Two planes, not four: a fixed-stage bank has no sidechain surface (#86 F4).
            assert_eq!(
                resource.builtin_bank_scratch_bytes,
                expected_banks as u64 * u64::from(artifact.envelope().quantum.0) * width * 2 * 4
            );
        }
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .cloned()
            .map(|node| {
                let processor = match node {
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    } => asymmetric_input_binding(&node),
                    _ => Box::new(IdentityBinding) as Box<dyn GraphRuntimeProcessor>,
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let bound = match artifact.into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(bound) => bound,
            Err(_) => panic!("sealed builtin bank bind"),
        };
        let mut plan = bound.plan;
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0; frames * 2 * 3];
        for block in 0..3 {
            let range = block * frames * 2..(block + 1) * frames * 2;
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut pcm[range], 2, frames, frames)
                        .expect("output"),
                },
                RenderTime {
                    absolute_sample: (block * frames) as u64,
                },
            )
            .expect("production builtin-bank render");
        }
        assert!(pcm[..frames * 2].iter().any(|sample| *sample != 0.0));
    }

    #[test]
    fn post_bank_graph_cap_rejects_transactionally_with_both_prepared_inputs() {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..8)
            .map(|index| {
                let mut track = base_track.clone();
                track.id = StableId::parse(&format!("bank{index}")).expect("id");
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
                track.simd2.effects.clear();
                track
            })
            .collect();
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut route = base_route.clone();
                route.id = StableId::parse(&format!("cap-route-{index}")).expect("route id");
                route.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                route
            })
            .collect();
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled cap session");
        let base = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 79,
            effects: EffectPreparedSession {
                session: session.clone(),
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("base graph: {:?}", failure.diagnostics));
        let baseline_builtins = prepare_session_builtins(
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
        .expect("baseline builtins");
        let baseline = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 80,
            effects: EffectPreparedSession {
                session: session.clone(),
                entries: Vec::new(),
            },
            builtins: baseline_builtins,
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("baseline bank graph: {:?}", failure.diagnostics));
        let final_samples = baseline.graph_resource_estimate().audio_buffer_samples;
        assert!(final_samples > base.report.estimate.audio_buffer_samples);
        let mut constrained = integration_caps();
        constrained.maximum_audio_buffer_samples = final_samples - 1;
        assert!(
            base.report.estimate.audio_buffer_samples <= constrained.maximum_audio_buffer_samples
        );
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
        .expect("returned builtins");
        let failure = match GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 81,
            effects: EffectPreparedSession {
                session: session.clone(),
                entries: Vec::new(),
            },
            builtins,
            caps: constrained,
        }) {
            Ok(_) => panic!("post-bank cap must reject"),
            Err(failure) => failure,
        };
        assert!(failure.diagnostics.diagnostics().iter().any(|diagnostic| {
            diagnostic.code == "graph.resource.limit" && diagnostic.path == "$.graph_compile_caps"
        }));
        assert_eq!(failure.effects.session.normalized_model().tracks.len(), 8);
        assert!(failure.effects.entries.is_empty());
        assert_eq!(failure.builtins.tails().count(), 8);
        assert!(
            failure
                .builtins
                .validate_for_session(&failure.effects.session)
                .0
                .is_empty(),
            "returned builtin ownership remains sealed and valid"
        );
    }

    #[test]
    fn frozen_issue_037_seeded_builtin_bank_layouts_have_exact_membership_and_counters() {
        const SEED: u64 = 0x0000_0000_8a05_0a08;
        const COUNTS: [usize; 9] = [1, 2, 3, 4, 5, 7, 8, 9, 17];
        let mut state = SEED;
        let mut transcript = 0xcbf2_9ce4_8422_2325_u64;
        let mut completed = 0_u32;
        for layout in 0..100_u32 {
            // SplitMix64, frozen locally so this suite has no dependency on host RNG state.
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut value = state;
            value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            value ^= value >> 31;
            let count = COUNTS[layout as usize % COUNTS.len()];
            let mut model = parse_session_json(SESSION_FIXTURE).expect("fixture");
            let base_track = model.tracks[0].clone();
            let base_route = model.routes[0].clone();
            model.automation.clear();
            model.tracks = (0..count)
                .map(|index| {
                    let mut track = base_track.clone();
                    track.id = StableId::parse(&format!("bank{index}")).expect("id");
                    track.simd1.effects.clear();
                    track.dynamic.effects.clear();
                    track.simd2.effects.clear();
                    // The seeded corpus includes identity filters, enabled filters, and
                    // intentionally asymmetric L/R coefficients without changing topology.
                    if ((value >> (index % 31)) & 1) != 0 {
                        track.builtins.left.hpf_hz = 0.0;
                    }
                    if ((value >> ((index + 7) % 31)) & 1) != 0 {
                        track.builtins.right.lpf_hz = 0.0;
                    }
                    if ((value >> ((index + 13) % 31)) & 1) != 0 {
                        track.builtins.right.polarity_invert =
                            !track.builtins.right.polarity_invert;
                    }
                    track
                })
                .collect();
            model.routes = model
                .tracks
                .iter()
                .enumerate()
                .map(|(index, track)| {
                    let mut route = base_route.clone();
                    route.id =
                        StableId::parse(&format!("seed-route-{layout}-{index}")).expect("route id");
                    route.source = RouteSource::Track {
                        track_id: track.id.clone(),
                        tap: SendTap::PostMatrix,
                    };
                    route
                })
                .collect();
            let compiled = compile_session(
                &model,
                CompileCaps {
                    max_compiled_model_bytes: u64::MAX,
                    max_requested_runtime_bytes: u64::MAX,
                    max_single_allocation_bytes: u64::MAX,
                    max_queue_items: u64::MAX,
                    max_source_ring_frames: u64::MAX,
                    max_source_ring_bytes: u64::MAX,
                },
            )
            .expect("compiled seeded layout");
            let builtins = prepare_session_builtins(
                &compiled,
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
            .expect("prepared seeded builtins");
            let artifact = match GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                dispatch: host_dispatch(),
                plan_id: u64::from(layout) + 50_000,
                effects: EffectPreparedSession {
                    session: compiled.clone(),
                    entries: Vec::new(),
                },
                builtins,
                caps: integration_caps(),
            }) {
                Ok(artifact) => artifact,
                Err(_) => panic!("seeded graph"),
            };
            let native_builtins = prepare_session_builtins(
                &compiled,
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
            .expect("independently prepared native seeded builtins");
            let repeat_artifact =
                GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                    dispatch: host_dispatch(),
                    plan_id: u64::from(layout) + 60_000,
                    effects: EffectPreparedSession {
                        session: compiled,
                        entries: Vec::new(),
                    },
                    builtins: native_builtins,
                    caps: integration_caps(),
                })
                .unwrap_or_else(|_| panic!("native seeded graph"));
            let width = BankWidth::for_backend(Backend::current());
            // #86 F3: `count.div_ceil(W)` banks per level (one level per stage here), last one
            // padded -- times the three bankable track stages since #212.
            let expected_banks = width.map_or(0, |width| {
                BANKABLE_TRACK_STAGES as usize * count.div_ceil(width.lanes() as usize)
            });
            let expected_tail = width.map_or(count, |_| 0);
            assert_eq!(artifact.prepared_builtin_bank_count(), expected_banks);
            assert_eq!(
                repeat_artifact.prepared_builtin_bank_count(),
                expected_banks
            );
            assert_eq!(
                artifact.graph().sequential_schedule,
                artifact
                    .graph()
                    .dependency_levels
                    .iter()
                    .flat_map(|level| level.nodes.iter().cloned())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                artifact.graph().sequential_schedule,
                repeat_artifact.graph().sequential_schedule
            );
            let envelope = artifact.envelope();
            let nodes = artifact
                .external_binding_nodes()
                .cloned()
                .map(|node| {
                    let processor = match node {
                        GraphNodeId::TrackStage {
                            stage: TrackStage::Input,
                            ..
                        } => asymmetric_input_binding(&node),
                        _ => Box::new(IdentityBinding) as Box<dyn GraphRuntimeProcessor>,
                    };
                    GraphNodeBinding::new(node, processor)
                })
                .collect();
            // Independent oracle for the D9 reduction (#98 F2). Every track's post-matrix output
            // is recorded, the routes are proven bit-transparent, and the session output must be
            // exactly those contributions folded left to right in the plan's own stable edge order
            // -- `reduce(|a, b| a + b)`, never `fold(0.0, +)`, so `-0.0` survives.
            let route_order: Vec<String> = artifact
                .graph()
                .spec
                .edges
                .iter()
                .filter(|edge| matches!(edge.destination.node, GraphNodeId::Output { .. }))
                .map(|edge| match &edge.source.node {
                    GraphNodeId::Route { route_id } => route_id.as_str().to_owned(),
                    other => panic!("output input is not a route: {other:?}"),
                })
                .collect();
            assert_eq!(route_order.len(), count);
            for route in artifact.graph().routes() {
                assert_eq!(
                    (
                        route.transform.gain,
                        route.transform.ll,
                        route.transform.lr,
                        route.transform.rl,
                        route.transform.rr
                    ),
                    (1.0, 1.0, 0.0, 0.0, 1.0),
                    "the oracle needs a bit-transparent route"
                );
            }
            let track_of_route: BTreeMap<String, String> = model
                .routes
                .iter()
                .map(|route| {
                    let RouteSource::Track { track_id, .. } = &route.source else {
                        panic!("seeded route source")
                    };
                    (route.id.as_str().to_owned(), track_id.as_str().to_owned())
                })
                .collect();
            let taps: Vec<(String, BitSink)> = route_order
                .iter()
                .map(|route| {
                    (
                        track_of_route[route].clone(),
                        Arc::new(std::sync::Mutex::new(Vec::new())),
                    )
                })
                .collect();
            let tap_observers: Vec<GraphNodeObserverBinding> = taps
                .iter()
                .enumerate()
                .map(|(handle, (track, sink))| {
                    GraphNodeObserverBinding::new(
                        GraphNodeId::TrackStage {
                            track_id: StableGraphId::parse(track).expect("track node id"),
                            stage: TrackStage::PostMatrix,
                        },
                        handle as u64,
                        Box::new(BitRecorder(Arc::clone(sink))),
                    )
                })
                .collect();
            let mut plan = match artifact.into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: tap_observers,
            }) {
                Ok(bound) => bound.plan,
                Err(_) => panic!("seeded bind"),
            };
            let frames = envelope.quantum.0 as usize;
            let mut pcm = vec![0.0; frames * 2];
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                        .expect("seeded output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("seeded render");
            let contributions: Vec<Vec<(u32, u32)>> = taps
                .iter()
                .map(|(_, sink)| sink.lock().expect("tap sink").clone())
                .collect();
            for (index, contribution) in contributions.iter().enumerate() {
                assert_eq!(contribution.len(), frames, "tap {index} block length");
            }
            for frame in 0..frames {
                let left = contributions
                    .iter()
                    .map(|tap| f32::from_bits(tap[frame].0))
                    .reduce(|a, b| a + b)
                    .unwrap_or(0.0);
                let right = contributions
                    .iter()
                    .map(|tap| f32::from_bits(tap[frame].1))
                    .reduce(|a, b| a + b)
                    .unwrap_or(0.0);
                assert_eq!(
                    (pcm[frame].to_bits(), pcm[frames + frame].to_bits()),
                    (left.to_bits(), right.to_bits()),
                    "layout {layout} frame {frame}: D9 left-to-right reduction oracle"
                );
            }
            let counters = plan.qualification_counters();
            assert_eq!(counters[0], expected_banks as u64);
            assert_eq!(counters[1], counters[0] * u64::from(envelope.quantum.0));
            let pcm_hash = pcm.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, sample| {
                (hash ^ u64::from(sample.to_bits())).wrapping_mul(0x0000_0100_0000_01b3)
            });
            for byte in format!(
                "{layout}:{value:016x}:{count}:{expected_banks}:{expected_tail}:{pcm_hash:016x}:{:?}",
                counters
            )
            .bytes()
            {
                transcript ^= u64::from(byte);
                transcript = transcript.wrapping_mul(0x0000_0100_0000_01b3);
            }
            completed += 1;
        }
        assert_eq!(completed, 100);
        // Re-pin chain (master plan #83 D9 and the section-8 policy):
        //
        //   0x0fc9_bdc8_ff12_0f6e  original
        //   0x9dfc_dcf2_0e37_0ef5  #98 F2 -- the session output's reduction became a
        //                          left-to-right recursive sum instead of a balanced pairwise
        //                          tree, moving `pcm_hash` for every layout with output fan-in
        //                          four or more; layouts with `count <= 3` were unmoved.
        //   0x0b9d_839a_7df9_3ac8  #163 phase 2 -- the numeric contract became unfused, so every
        //                          rendered sample moved and with it every layout's `pcm_hash`,
        //                          for all 100 layouts and every fan-in.
        //
        //   0xe095_f3ad_a9cc_cf46  #212 -- the fader and the matrix became bankable stages, so
        //                          `expected_banks` and the `counters` pair tripled for every
        //                          layout. This is the **first** link that moves those fields and
        //                          not `pcm_hash`: it is a structural change, not a numeric one.
        //
        // Every link before #212 moved `pcm_hash` and left the membership and counter halves
        // alone; #212 does the opposite, and the two halves being separable is what makes this a
        // chain rather than a sequence of unrelated numbers. That `pcm_hash` did not move is not
        // inferred from the change's intent: all 100 layouts' `pcm_hash` values were captured on
        // the base commit and on this one and compared, and every one of the 100 is identical.
        // The per-layout membership, bank-count, tail and counter assertions above still hold
        // against their own expectations, which are derived rather than written down.
        //
        // It is *not* pinned from production output: the per-layout `assert_eq!` above derives the
        // expected output from the recorded per-track post-matrix contributions folded left to
        // right in the plan's own stable edge order -- through `softfma::unfused_multiply_add_via_f64`,
        // an `f64` restatement independent of the `f32` vector body -- for all 100 layouts, before
        // this literal is compared.
        assert_eq!(
            transcript, 0xe095_f3ad_a9cc_cf46,
            "frozen Issue-037 seeded layout transcript"
        );
    }

    #[test]
    fn each_forged_builtin_seal_tuple_is_rejected_before_graph_attachment() {
        let cases = [
            (
                PreparedBuiltinsCorruptionCase::SessionHash,
                "builtin.session.mismatch",
            ),
            (
                PreparedBuiltinsCorruptionCase::SessionRate,
                "builtin.session.mismatch",
            ),
            (
                PreparedBuiltinsCorruptionCase::SessionQuantum,
                "builtin.session.mismatch",
            ),
            (
                PreparedBuiltinsCorruptionCase::TrackMissing,
                "builtin.prepared.track_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TrackExtra,
                "builtin.prepared.track_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TrackDuplicate,
                "builtin.prepared.track_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ProcessorMissing,
                "builtin.prepared.processor_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ProcessorExtra,
                "builtin.prepared.processor_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ProcessorChangedStage,
                "builtin.prepared.processor_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TailMissing,
                "builtin.prepared.tail_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TailExtra,
                "builtin.prepared.tail_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::TailChanged,
                "builtin.prepared.tail_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::RequestMissing,
                "builtin.prepared.request_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::RequestExtra,
                "builtin.prepared.request_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::RequestDuplicate,
                "builtin.prepared.request_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ObserverMissing,
                "builtin.prepared.observer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ObserverExtra,
                "builtin.prepared.observer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ObserverChangedNode,
                "builtin.prepared.observer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerMissing,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerExtra,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerChangedMetadata,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ConsumerDuplicateHandle,
                "builtin.prepared.consumer_set",
            ),
            (
                PreparedBuiltinsCorruptionCase::ResourceReport,
                "builtin.prepared.resource_report",
            ),
        ];
        let mut categories = BTreeSet::new();
        for (corruption, expected) in cases {
            categories.insert(corruption.category());
            let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
            model.tracks[0].dynamic.effects.clear();
            model.automation.clear();
            let compiled = compile_session(
                &model,
                CompileCaps {
                    max_compiled_model_bytes: u64::MAX,
                    max_requested_runtime_bytes: u64::MAX,
                    max_single_allocation_bytes: u64::MAX,
                    max_queue_items: u64::MAX,
                    max_source_ring_frames: u64::MAX,
                    max_source_ring_bytes: u64::MAX,
                },
            )
            .expect("compiled");
            let mut builtins = prepare_session_builtins(
                &compiled,
                &[
                    MeterRequest {
                        handle: MeterHandle(NonZeroU64::new(10).expect("constant")),
                        track_id: "vocal".to_owned(),
                        tap: MeterTap::Input,
                        config: MeterConfig {
                            period_frames: NonZeroU32::new(16).expect("constant"),
                            peak_hold_frames: 0,
                            peak_decay_db_per_second: 0.0,
                            queue_capacity: NonZeroUsize::new(4).expect("constant"),
                            reset_generation: 10,
                        },
                    },
                    MeterRequest {
                        handle: MeterHandle(NonZeroU64::new(11).expect("constant")),
                        track_id: "vocal".to_owned(),
                        tap: MeterTap::PostMatrix,
                        config: MeterConfig {
                            period_frames: NonZeroU32::new(32).expect("constant"),
                            peak_hold_frames: 4,
                            peak_decay_db_per_second: 12.0,
                            queue_capacity: NonZeroUsize::new(4).expect("constant"),
                            reset_generation: 11,
                        },
                    },
                ],
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
            .expect("builtins");
            builtins.test_only_corrupt_for_compiler_test(corruption);
            let Err(failure) = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                dispatch: host_dispatch(),
                plan_id: 78,
                effects: EffectPreparedSession {
                    session: compiled,
                    entries: Vec::new(),
                },
                builtins,
                caps: integration_caps(),
            }) else {
                panic!("forged builtin artifact must reject: {corruption:?}");
            };
            assert_eq!(
                failure
                    .diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| diagnostic.code)
                    .collect::<Vec<_>>(),
                vec![expected]
            );
            // Rejection is transactional: the compiler returns both inputs rather than consuming
            // either one into graph bindings.
            assert_eq!(failure.effects.entries.len(), 0);
            assert!(failure.builtins.processor_count() <= 3);
        }
        assert_eq!(
            categories,
            BTreeSet::from([
                PreparedBuiltinsCorruption::SessionIdentity,
                PreparedBuiltinsCorruption::Tracks,
                PreparedBuiltinsCorruption::Processors,
                PreparedBuiltinsCorruption::Tails,
                PreparedBuiltinsCorruption::Requests,
                PreparedBuiltinsCorruption::Observers,
                PreparedBuiltinsCorruption::Consumers,
                PreparedBuiltinsCorruption::Resources,
            ])
        );
    }

    #[test]
    fn cycle_witness_skips_acyclic_residual_nodes_downstream_of_cycle() {
        let nodes = [
            graph_node("a", 0, TailSamples::Finite(0)),
            graph_node("b", 0, TailSamples::Finite(0)),
            graph_node("c", 0, TailSamples::Finite(0)),
        ];
        // `a` sorts first and is downstream of the b/c cycle. Kahn leaves all three residual.
        let edges = [
            edge("to-a", "b", "a"),
            edge("to-b", "c", "b"),
            edge("to-c", "b", "c"),
        ];
        let (witness, paths) = cycle_witness(&nodes, &edges).expect("cycle");
        assert_eq!(witness, [node("b"), node("c"), node("b")]);
        assert_eq!(paths, ["$.routes[id=to-c]", "$.routes[id=to-b]"]);
    }

    #[test]
    fn every_cyclic_scc_has_one_closed_sorted_witness_and_edge_paths() {
        let nodes: Vec<_> = ["a", "b", "c", "d", "e", "z"]
            .into_iter()
            .map(|name| graph_node(name, 0, TailSamples::Finite(0)))
            .collect();
        let mut edges = vec![
            edge("ab", "a", "b"),
            edge("ba", "b", "a"),
            edge("cc", "c", "c"),
            edge("de", "d", "e"),
            edge("ed", "e", "d"),
            edge("za", "a", "z"),
        ];
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        let witnesses = cycle_witnesses(&nodes, &edges);
        assert_eq!(witnesses.len(), 3);
        assert_eq!(witnesses[0].0, [node("a"), node("b"), node("a")]);
        assert_eq!(witnesses[0].1, ["$.routes[id=ab]", "$.routes[id=ba]"]);
        assert_eq!(witnesses[1].0, [node("c"), node("c")]);
        assert_eq!(witnesses[1].1, ["$.routes[id=cc]"]);
        assert_eq!(witnesses[2].0, [node("d"), node("e"), node("d")]);
        assert_eq!(witnesses[2].1, ["$.routes[id=de]", "$.routes[id=ed]"]);
    }

    #[test]
    fn timing_applies_declared_tail_after_node_latency() {
        let nodes = [
            graph_node("source", 0, TailSamples::Finite(0)),
            graph_node("effect", 3, TailSamples::Finite(5)),
        ];
        let edges = [edge("serial", "source", "effect")];
        let levels = topo(&nodes, &edges).expect("acyclic");
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let latencies = nodes
            .iter()
            .map(|node| (node.id.clone(), node.latency))
            .collect();
        let tails = nodes
            .iter()
            .map(|node| (node.id.clone(), node.tail))
            .collect();
        let error = timings(&schedule, &edges, &latencies, &tails, &caps(7))
            .err()
            .expect("latency plus tail exceeds cap");
        assert_eq!(error.code, "graph.tail.limit");
    }

    #[test]
    fn timing_reports_the_checked_sole_output_arrival_and_extent() {
        let early = node("early");
        let late = node("late");
        let output = GraphNodeId::Output {
            output_id: gid("main"),
        };
        let nodes = [
            GraphNode {
                id: early.clone(),
                latency: LatencySamples(3),
                tail: TailSamples::Finite(5),
            },
            GraphNode {
                id: late.clone(),
                latency: LatencySamples(7),
                tail: TailSamples::Finite(1),
            },
            GraphNode {
                id: output.clone(),
                latency: LatencySamples(0),
                tail: TailSamples::Finite(0),
            },
        ];
        let edges = [
            GraphEdge {
                id: GraphEdgeId::RouteDestination {
                    route_id: gid("early-main"),
                },
                source: port(early, GraphPortKind::MainOutput),
                destination: port(output.clone(), GraphPortKind::MainInput),
                path: "$.routes[id=early-main]".to_owned(),
            },
            GraphEdge {
                id: GraphEdgeId::RouteDestination {
                    route_id: gid("late-main"),
                },
                source: port(late, GraphPortKind::MainOutput),
                destination: port(output, GraphPortKind::MainInput),
                path: "$.routes[id=late-main]".to_owned(),
            },
        ];
        let levels = topo(&nodes, &edges).expect("acyclic output graph");
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let latencies = nodes
            .iter()
            .map(|node| (node.id.clone(), node.latency))
            .collect();
        let tails = nodes
            .iter()
            .map(|node| (node.id.clone(), node.tail))
            .collect();
        let timing = timings(&schedule, &edges, &latencies, &tails, &caps(100))
            .expect("checked output timing");
        assert_eq!(timing.output_latency, LatencySamples(7));
        assert_eq!(timing.output_tail, TailSamples::Finite(12));
        assert_eq!(timing.delays.len(), 1);

        let mut infinite_tails = tails;
        infinite_tails.insert(node("late"), TailSamples::Infinite);
        let infinite = timings(&schedule, &edges, &latencies, &infinite_tails, &caps(100))
            .expect("infinite output tail");
        assert_eq!(infinite.output_latency, LatencySamples(7));
        assert_eq!(infinite.output_tail, TailSamples::Infinite);
    }

    #[test]
    fn buffer_coloring_aliases_identity_and_preserves_fanout_liveness() {
        let source = track_node("track", TrackStage::Input);
        let identity = track_node("track", TrackStage::PostSimd1);
        let route_a = GraphNodeId::Route { route_id: gid("a") };
        let route_b = GraphNodeId::Route { route_id: gid("b") };
        let output = GraphNodeId::Output {
            output_id: gid("main"),
        };
        let schedule = vec![
            source.clone(),
            identity.clone(),
            route_a.clone(),
            route_b.clone(),
            output.clone(),
        ];
        let make_edge = |id, source, destination| GraphEdge {
            id,
            source: port(source, GraphPortKind::MainOutput),
            destination: port(destination, GraphPortKind::MainInput),
            path: "$.coloring".to_owned(),
        };
        let edges = vec![
            make_edge(
                GraphEdgeId::TrackMain {
                    target: identity.clone(),
                },
                source,
                identity.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteSource { route_id: gid("a") },
                identity.clone(),
                route_a.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteSource { route_id: gid("b") },
                identity.clone(),
                route_b.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteDestination { route_id: gid("a") },
                route_a.clone(),
                output.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteDestination { route_id: gid("b") },
                route_b.clone(),
                output.clone(),
            ),
        ];
        let assigned: BTreeMap<_, _> = buffer_assignments(&schedule, &edges)
            .into_iter()
            .map(|assignment| (assignment.port.node, assignment.buffer_index))
            .collect();
        assert_eq!(assigned[&identity], 0);
        assert_eq!(assigned[&route_a], 1);
        assert_eq!(assigned[&route_b], 2);
        assert_eq!(assigned[&output], 0);
    }

    #[test]
    fn level_major_compiler_coloring_matches_independent_live_intervals() {
        let artifact = compile_reverse_route_submix_fixture(123_200);
        let report = &artifact.graph;
        let flattened: Vec<_> = report
            .dependency_levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        assert_eq!(report.sequential_schedule, flattened);
        assert_eq!(
            report.buffer_assignments,
            buffer_assignments(&flattened, &report.spec.edges)
        );

        let mut old_kahn = flattened.clone();
        old_kahn.swap(11, 12);
        old_kahn.swap(10, 11);
        assert_ne!(old_kahn, flattened);
        assert_ne!(
            buffer_assignments(&old_kahn, &report.spec.edges),
            report.buffer_assignments,
            "old Kahn coloring cannot be retained under level-major execution"
        );

        let positions: BTreeMap<_, _> = flattened
            .iter()
            .cloned()
            .enumerate()
            .map(|(position, node)| (node, position))
            .collect();
        let assigned: BTreeMap<_, _> = report
            .buffer_assignments
            .iter()
            .map(|assignment| (assignment.port.node.clone(), assignment.buffer_index))
            .collect();
        let intervals: Vec<_> = flattened
            .iter()
            .map(|node| {
                let start = positions[node];
                let end = report
                    .spec
                    .edges
                    .iter()
                    .filter(|edge| edge.source.node == *node)
                    .map(|edge| positions[&edge.destination.node])
                    .max()
                    .unwrap_or(start);
                (node, assigned[node], start, end)
            })
            .collect();
        for (index, left) in intervals.iter().enumerate() {
            for right in &intervals[index + 1..] {
                if left.1 != right.1 || left.3 < right.2 {
                    continue;
                }
                let aliases_identity_boundary = is_identity_boundary(right.0)
                    && report
                        .spec
                        .edges
                        .iter()
                        .filter(|edge| edge.destination.node == *right.0)
                        .count()
                        == 1
                    && report
                        .spec
                        .edges
                        .iter()
                        .filter(|edge| edge.source.node == *left.0)
                        .count()
                        == 1
                    && report.spec.edges.iter().any(|edge| {
                        edge.source.node == *left.0 && edge.destination.node == *right.0
                    });
                assert!(
                    aliases_identity_boundary,
                    "overlapping non-alias live intervals"
                );
            }
        }
    }

    #[test]
    fn accepted_session_compiles_binds_and_renders_direct_route() {
        let artifact = compile_fixture(123);
        assert_eq!(artifact.report.estimate.routes, 1);
        assert_eq!(artifact.report.estimate.effects, 0);
        assert_eq!(artifact.report.estimate.reductions, 0);
        assert!(artifact.report.estimate.audio_buffer_samples > 0);
        assert!(artifact.report.estimate.graph_metadata_bytes > 0);
        assert!(artifact.report.estimate.incremental_plan_bytes > 0);
        let assigned: BTreeMap<_, _> = artifact
            .graph
            .buffer_assignments
            .iter()
            .map(|assignment| (assignment.port.node.clone(), assignment.buffer_index))
            .collect();
        let track = |stage| track_node("vocal", stage);
        assert_eq!(
            assigned[&track(TrackStage::PostInputBuiltins)],
            assigned[&track(TrackStage::PostSimd1)]
        );
        assert_eq!(
            assigned[&track(TrackStage::PostSimd1)],
            assigned[&track(TrackStage::PostDynamic)]
        );
        assert_eq!(
            assigned[&track(TrackStage::PostDynamic)],
            assigned[&track(TrackStage::PostSimd2PreFader)]
        );
        let colored_buffer_count = assigned.values().copied().max().expect("buffers") + 1;
        assert_eq!(colored_buffer_count, 2);
        assert!(colored_buffer_count < artifact.report.estimate.logical_nodes);
        assert_eq!(artifact.graph.required_bindings.len(), 5);
        let envelope = artifact.graph.envelope;
        let nodes = artifact
            .graph
            .required_bindings
            .iter()
            .cloned()
            .map(|node| {
                let processor: Box<dyn GraphRuntimeProcessor> = if matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                ) {
                    Box::new(ImpulseBinding)
                } else {
                    Box::new(IdentityBinding)
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let mut plan = match artifact.graph.bind(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(plan) => plan,
            Err(failure) => panic!("bind: {}", failure.code),
        };
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        let output = PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output");
        plan.render(
            RenderIo {
                input: None,
                output,
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("render");
        assert_eq!(pcm[0], 1.0);
        assert_eq!(pcm[frames], -1.0);
        assert!(pcm[1..frames].iter().all(|sample| *sample == 0.0));
        assert!(pcm[frames + 1..].iter().all(|sample| *sample == 0.0));
    }

    #[test]
    fn canonical_artifacts_are_complete_and_repeatable_100_times() {
        let baseline = compile_fixture(0);
        // #99 F5: the evidence is produced here, by an explicit call, not carried by the report.
        let baseline_evidence = GraphCompiler::evidence(&baseline.graph, &baseline.report);
        let canonical = core::str::from_utf8(&baseline_evidence.canonical_bytes).expect("UTF-8");
        for section in [
            "envelope\t",
            "node\t",
            "port\t",
            "edge\t",
            "order\t",
            "level\t",
            "route-transform\t",
            "route-timing\t",
            "tail\t",
            "buffer\t",
            "estimate\t",
        ] {
            assert!(canonical.contains(section), "missing {section}");
        }
        assert!(!canonical.contains("Simd"));
        assert!(!canonical.contains("Finite"));
        assert!(
            baseline_evidence
                .sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        );
        assert_eq!(baseline_evidence.sha256.len(), 64);
        assert!(baseline_evidence.dot.ends_with("}\n"));
        // The streaming hash and the materialised one agree: `GraphCompiler::sha256` never builds
        // the text, so this is the gate that keeps the two writers in step.
        assert_eq!(
            GraphCompiler::sha256(&baseline.graph, &baseline.report),
            baseline_evidence.sha256
        );
        for plan_id in 1..=100 {
            let candidate = compile_fixture(plan_id);
            let evidence = GraphCompiler::evidence(&candidate.graph, &candidate.report);
            assert_eq!(evidence.canonical_bytes, baseline_evidence.canonical_bytes);
            assert_eq!(evidence.sha256, baseline_evidence.sha256);
            assert_eq!(
                candidate.graph.sequential_schedule,
                baseline.graph.sequential_schedule
            );
            assert_eq!(
                candidate.graph.dependency_levels,
                baseline.graph.dependency_levels
            );
            assert_eq!(candidate.graph.route_timings, baseline.graph.route_timings);
            assert_eq!(
                candidate.graph.buffer_assignments,
                baseline.graph.buffer_assignments
            );
            assert_eq!(evidence.dot, baseline_evidence.dot);
        }
    }

    #[test]
    fn route_transform_bits_participate_in_semantic_hash() {
        let baseline = compile_fixture(1);
        let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
        model.tracks[0].dynamic.effects.clear();
        model.automation.clear();
        model.routes[0].gain_db = -6.0;
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("session");
        let changed = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1,
            effects: EffectPreparedSession {
                session,
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics));
        assert_ne!(
            GraphCompiler::sha256(&changed.graph, &changed.report),
            GraphCompiler::sha256(&baseline.graph, &baseline.report)
        );
        assert_ne!(
            GraphCompiler::evidence(&changed.graph, &changed.report).canonical_bytes,
            GraphCompiler::evidence(&baseline.graph, &baseline.report).canonical_bytes
        );
    }

    /// #99 F4: the compiled route gain is `math::db_to_gain_f32`, bit for bit.
    ///
    /// -19 dB is the witness: the platform `f64::powf` form this replaced produced
    /// `0x3de5_ca15` on this host, one ulp below the canonical `0x3de5_ca16`, and it produced
    /// whatever the *host's* libm produced on any other. `tests/route_gain.rs` pins both
    /// literals against a live `powf` oracle so this witness cannot go stale silently.
    #[test]
    fn route_transform_uses_the_canonical_db_to_gain_conversion() {
        let mut model = parse_session_json(SESSION_FIXTURE).expect("session fixture");
        model.tracks[0].dynamic.effects.clear();
        model.automation.clear();
        model.routes[0].gain_db = -19.0;
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("session");
        let compiled = GraphCompiler::compile(GraphCompileRequest {
            dispatch: host_dispatch(),
            plan_id: 1,
            effects: EffectPreparedSession {
                session,
                entries: Vec::new(),
            },
            caps: integration_caps(),
        })
        .unwrap_or_else(|failure| panic!("graph diagnostics: {:?}", failure.diagnostics));
        let gains: Vec<u32> = compiled
            .graph
            .routes()
            .iter()
            .map(|route| route.transform.gain.to_bits())
            .collect();
        assert_eq!(gains, vec![0x3de5_ca16]);
        assert_eq!(
            gains[0],
            math::db_to_gain_f32(-19.0).to_bits(),
            "route gain must be the canonical conversion, not a local one"
        );
    }

    #[test]
    fn ten_thousand_graph_mutations_are_panic_free_and_repeatable() {
        let mut state = 0x6d69_736f_6d75_7461_u64;
        for mutation in 0..10_000_u32 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let node_count = (state as usize % 8) + 1;
            let nodes: Vec<_> = (0..node_count)
                .map(|index| {
                    graph_node(
                        &format!("n{index}"),
                        (state >> (index % 16)) & 7,
                        TailSamples::Finite((state >> ((index + 3) % 16)) & 7),
                    )
                })
                .collect();
            let mut edges = Vec::new();
            for edge_index in 0..node_count.saturating_mul(2) {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let source = state as usize % node_count;
                let destination = (state >> 11) as usize % node_count;
                edges.push(edge(
                    &format!("m{mutation}-{edge_index}"),
                    &format!("n{source}"),
                    &format!("n{destination}"),
                ));
            }
            edges.sort_by(|left, right| left.id.cmp(&right.id));
            let first_cycle = cycle_witness(&nodes, &edges);
            let second_cycle = cycle_witness(&nodes, &edges);
            assert_eq!(first_cycle, second_cycle);
            if first_cycle.is_none() {
                let first_levels = topo(&nodes, &edges).expect("acyclic");
                let second_levels = topo(&nodes, &edges).expect("repeat");
                assert_eq!(first_levels, second_levels);
                let first_schedule: Vec<_> = first_levels
                    .iter()
                    .flat_map(|level| level.nodes.iter().cloned())
                    .collect();
                let second_schedule: Vec<_> = second_levels
                    .iter()
                    .flat_map(|level| level.nodes.iter().cloned())
                    .collect();
                assert_eq!(first_schedule, second_schedule);
                let latencies = nodes
                    .iter()
                    .map(|node| (node.id.clone(), node.latency))
                    .collect();
                let tails = nodes
                    .iter()
                    .map(|node| (node.id.clone(), node.tail))
                    .collect();
                let first = timings(
                    &first_schedule,
                    &edges,
                    &latencies,
                    &tails,
                    &caps(1_000_000),
                );
                let second = timings(
                    &second_schedule,
                    &edges,
                    &latencies,
                    &tails,
                    &caps(1_000_000),
                );
                assert_eq!(
                    first
                        .as_ref()
                        .map(|result| (&result.routes, &result.delays)),
                    second
                        .as_ref()
                        .map(|result| (&result.routes, &result.delays))
                );
                assert_eq!(first.err(), second.err());
            }
        }
    }
}
