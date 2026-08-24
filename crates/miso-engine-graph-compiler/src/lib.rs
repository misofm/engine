//! Deterministic control-plane lowering of an accepted session and prepared native effects.
#![allow(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

use miso_engine_builtins::BuiltinTail;
use miso_engine_builtins_compiler::{
    PreparedBuiltinsGraphArtifact, PreparedBuiltinsGraphBindFailure, PreparedBuiltinsGraphBound,
    PreparedBuiltinsSession,
};
use miso_engine_core::realtime::RenderEnvelope;
use miso_engine_effect_compiler::{EffectPreparedEntry, EffectPreparedSession, EffectRack};
use miso_engine_effect_contract::BankWidth;
use miso_engine_effect_contract::{
    LatencySamples, PrepareEffectBankRequest, PreparedSidechainPort, TailSamples,
};
use miso_engine_graph::{
    BufferAssignment, DependencyLevel, EffectNodeId, GraphCompileCaps, GraphDiagnostic,
    GraphDiagnosticSet, GraphEdge, GraphEdgeId, GraphNode, GraphNodeId, GraphPortId, GraphPortKind,
    GraphPreparedEffect, GraphResourceEstimate, GraphSpec, InsertedDelay, PreparedGraphPlan,
    PreparedGraphPlanParts, PreparedRoute, RackId, ReductionRecord, RouteTiming, RouteTransform,
    StableGraphId, TrackStage,
};
/// Re-exported so a caller can name the compile input without taking a `miso-engine-lane`
/// dependency of its own: the backend is this crate's input now, so this crate publishes its type
/// (#99 F6). The build's backend is read by the caller -- `miso_engine_lane::Backend::current()`
/// -- and never inside the compiler.
pub use miso_engine_lane::Backend;
use miso_engine_rack::{RackLocationV1, RackProgramV1};
use miso_engine_rack_compiler::{
    BankGroup, BankPlan, CohortCandidate, CohortLevel, plan_bank_groups,
};
use miso_engine_session::{
    ChannelMatrix, RouteDestination, RouteSource, SendTap, SidechainDeclaration,
};
use sha2::{Digest, Sha256};

pub struct GraphCompileRequest {
    pub plan_id: u64,
    pub effects: EffectPreparedSession,
    pub caps: GraphCompileCaps,
    /// The kernel dispatch the SIMD-rack and builtin banks are planned for.
    ///
    /// Compile is a pure function of its inputs (#99 F6). The host CPU is read exactly once, by
    /// the caller that owns the render target -- `miso-engine-capi` and the web host do it at
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
/// use miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact;
///
/// // The compiler-owned graph and builtin parts are private: external bindings cannot create
/// // a value carrying internal-builtin provenance.
/// let _ = PreparedGraphBuiltinsArtifact {};
/// ```
///
/// ```compile_fail
/// fn mutate(mut artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     artifact.graph = panic!("private provenance field");
/// }
/// ```
///
/// ```compile_fail
/// fn extract(artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact { graph, .. } = artifact;
/// }
/// ```
///
/// ```compile_fail
/// fn clone_back(artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let _ = artifact.clone();
/// }
/// ```
///
/// ```compile_fail
/// fn back_convert(artifact: miso_engine_graph_compiler::PreparedGraphBuiltinsArtifact) {
///     let _: miso_engine_graph::PreparedGraphPlan = artifact.into();
/// }
/// ```
///
/// ```compile_fail
/// fn generic_internal_attachment(plan: miso_engine_graph::PreparedGraphPlan) {
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
/// lets `RackProgramV1::subsequence_mask` decide which lanes run which slot.
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
    pub fn groups_in(&self, rack: RackLocationV1) -> impl Iterator<Item = &BankGroup<RackChainId>> {
        self.plan
            .groups
            .iter()
            .filter(move |group| group.rack == rack)
    }
    /// Groups with at least one slot actually bound as a bank.
    pub fn bound_groups_in(
        &self,
        rack: RackLocationV1,
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
    pub fn bound_slots_in(
        &self,
        rack: RackLocationV1,
    ) -> impl Iterator<Item = &GraphRackBoundSlot> {
        self.bound_slots
            .iter()
            .filter(move |bound| self.plan.groups[bound.group].rack == rack)
    }
    /// Effect nodes that render on the per-node scalar path in one rack, in id order: every node
    /// of a candidate that never banked, plus every node at a slot that was not bound.
    #[must_use]
    pub fn scalar_in(&self, rack: RackLocationV1) -> Vec<EffectNodeId> {
        let banked: std::collections::BTreeSet<&EffectNodeId> = self
            .bound_slots
            .iter()
            .filter(|bound| self.plan.groups[bound.group].rack == rack)
            .flat_map(|bound| bound.members.iter())
            .collect();
        let mut ids: Vec<EffectNodeId> = self
            .chains
            .iter()
            .filter(|(chain, _)| rack_location(chain.rack) == Some(rack))
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

#[allow(unused_imports)]
use crate::{banks::*, canonical::*, compile::*, estimate::*, ids::*, pdc::*, schedule::*};
#[cfg(test)]
mod tests {
    use super::*;

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
    use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
    use miso_engine_builtins::{MeterConfig, MeterHandle, MeterTap};
    use miso_engine_builtins_compiler::{
        BuiltinCompileCaps, MeterRequest, PreparedBuiltinsCorruption,
        PreparedBuiltinsCorruptionCase, prepare_session_builtins,
    };
    use miso_engine_conformance::DualAccumulatorDelayFactory;
    use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime, audit};
    use miso_engine_effect_compiler::{
        EffectCompileCaps, EffectPreparedSession, launch_native_effect_registry_v1,
        prepare_native_session_effects,
    };
    use miso_engine_effect_contract::{
        EffectPrepareError, EffectProcessBlock, NativeEffectFactory, NativeEffectRegistry,
        PrepareEffectBankRequest, PrepareEffectRequest, PreparedNativeEffect,
        PreparedNativeEffectBank, ProcessReport, StatePayloadOutput,
    };
    use miso_engine_graph::{
        FallbackReasonV1, GraphBindingBlock, GraphNodeBinding, GraphNodeObserverBinding,
        GraphObservationBlock, GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
        NativeGraphBindConfigV1, NativeGraphRenderModeV1, NativeSchedulerConfigV1,
        SchedulerSelectionV1,
    };
    use miso_engine_session::{
        CompileCaps, EffectIdentity, EffectParam, ParameterChannel, ParameterUnit,
        RouteDestination, RouteSource, Sidechain, SidechainDeclaration, StableId, Submix,
        compile_session, parse_session_toml,
    };
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    };

    const SESSION_FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
    const PARAMETRIC_EQ_NINE_TRACK_FIXTURE: &str =
        include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");

    struct IdentityBinding;
    impl GraphRuntimeProcessor for IdentityBinding {
        fn process(
            &mut self,
            _block: GraphBindingBlock<'_>,
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
            Ok(())
        }
    }

    struct ImpulseBinding;
    impl GraphRuntimeProcessor for ImpulseBinding {
        fn process(
            &mut self,
            block: GraphBindingBlock<'_>,
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            block.left[0] = self.left;
            block.right[0] = self.right;
            Ok(())
        }
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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

    fn accepted_compressor_graph_fixture() -> miso_engine_session::SessionTomlV1 {
        let mut model =
            parse_session_toml(PARAMETRIC_EQ_NINE_TRACK_FIXTURE).expect("accepted base fixture");
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

    fn accepted_gate_expander_graph_fixture() -> miso_engine_session::SessionTomlV1 {
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

    fn accepted_true_peak_limiter_graph_fixture() -> miso_engine_session::SessionTomlV1 {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("true-peak-limiter").expect("limiter effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.true-peak-limiter").expect("limiter id"),
            };
            effect.link_mode = miso_engine_session::LinkMode::Maximum;
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

    fn accepted_multiband_compressor_graph_fixture() -> miso_engine_session::SessionTomlV1 {
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

    fn accepted_soft_clip_graph_fixture() -> miso_engine_session::SessionTomlV1 {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("soft-clip").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.soft-clip").expect("soft-clip id"),
            };
            effect.link_mode = miso_engine_session::LinkMode::DualMono;
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

    fn accepted_transient_shaper_graph_fixture() -> miso_engine_session::SessionTomlV1 {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let effect = &mut track.simd1.effects[0];
            effect.id = StableId::parse("transient-shaper").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.transient-shaper").expect("transient-shaper id"),
            };
            effect.link_mode = miso_engine_session::LinkMode::DualMono;
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

    fn accepted_delay_graph_fixture() -> miso_engine_session::SessionTomlV1 {
        let mut model = accepted_compressor_graph_fixture();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let mut effect = track.simd1.effects.remove(0);
            effect.id = StableId::parse("delay").expect("stable effect id");
            effect.identity = EffectIdentity::Native {
                effect_id: StableId::parse("miso.delay").expect("delay id"),
            };
            effect.link_mode = miso_engine_session::LinkMode::DualMono;
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
        fn descriptor(&self) -> &'static miso_engine_effect_contract::EffectDescriptorV1 {
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
        fn descriptor(&self) -> &'static miso_engine_effect_contract::EffectDescriptorV1 {
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
        fn descriptor(&self) -> &'static miso_engine_effect_contract::EffectDescriptorV1 {
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
    /// of which are checked against caps and against `limits.memory_bytes` -- so a wrong length is
    /// a wrong admission decision, not a cosmetic drift. The two functions are separate code paths
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
    /// `NativeGraphBlueprint::prepare` rejects any dependency level whose nodes are not strictly
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
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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

    fn render_reverse_route_submix(
        artifact: PreparedGraphArtifact,
        render_mode: NativeGraphRenderModeV1,
    ) -> (Vec<u32>, SchedulerSelectionV1, u64, bool) {
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
        // The dependency-wave mode needs a real pool; the pool outlives the plan it leases to.
        let (pool, lease) = match render_mode {
            NativeGraphRenderModeV1::DependencyWaves => {
                let (pool, lease) = miso_engine_graph::NativeGraphWorkerPoolV1::start(
                    miso_engine_graph::NativeWorkerPoolConfigV1 {
                        requested_workers: NonZeroUsize::new(3),
                        ..miso_engine_graph::NativeWorkerPoolConfigV1::default()
                    },
                )
                .unwrap_or_else(|_| panic!("reverse-route worker pool"));
                (Some(pool), Some(lease))
            }
            _ => (None, None),
        };
        let pool_shape = pool
            .as_ref()
            .map(miso_engine_graph::NativeGraphWorkerPoolV1::shape)
            .unwrap_or_default();
        let bindings = GraphRuntimeBindings {
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: lease,
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
        let prepared = artifact
            .graph
            .bind_native(
                bindings,
                NativeGraphBindConfigV1 {
                    render_mode,
                    scheduler: NativeSchedulerConfigV1::new(
                        NonZeroUsize::new(4).expect("four lanes"),
                        true,
                        pool_shape,
                    )
                    .with_recovery_deadline_ns(5_000_000_000),
                    maximum_retained_bytes: 1 << 20,
                },
            )
            .unwrap_or_else(|failure| panic!("native bind: {}", failure.code));
        let selection = prepared.metadata.selection;
        let mut plan = prepared.into_plan();
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
        if let Some(pool) = pool {
            pool.stop_and_join();
        }
        (
            pcm.into_iter().map(f32::to_bits).collect(),
            selection,
            observer_order.load(Ordering::SeqCst),
            observed_audio.load(Ordering::SeqCst),
        )
    }

    #[test]
    fn issue122_reverse_route_ids_emit_sorted_levels_and_bind_both_native_modes() {
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
        reverse_fixture_identity_contract(
            &baseline.graph,
            &baseline.report,
            &baseline.graph.sequential_schedule,
            &baseline.graph.dependency_levels,
            &GraphCompiler::evidence(&baseline.graph, &baseline.report).canonical_bytes,
            &expected_schedule,
            "464022a08d25cab733387983fc6c3d78da0fee1c3427698949dc8209339fe1c5",
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
            "464022a08d25cab733387983fc6c3d78da0fee1c3427698949dc8209339fe1c5"
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
                "464022a08d25cab733387983fc6c3d78da0fee1c3427698949dc8209339fe1c5",
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
                "464022a08d25cab733387983fc6c3d78da0fee1c3427698949dc8209339fe1c5",
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
        let single =
            render_reverse_route_submix(single_artifact, NativeGraphRenderModeV1::SingleThread);
        let wave =
            render_reverse_route_submix(wave_artifact, NativeGraphRenderModeV1::DependencyWaves);
        assert!(matches!(single.1, SchedulerSelectionV1::Sequential(_)));
        assert_eq!(wave.1, SchedulerSelectionV1::Parallel);
        assert_eq!(single.0, wave.0);
        assert_eq!(single.0[0], 2.0_f32.to_bits());
        assert_eq!(single.0[128], (-2.0_f32).to_bits());
        assert!(single.0[1..128].iter().all(|sample| *sample == 0));
        assert!(single.0[129..].iter().all(|sample| *sample == 0));
        assert_eq!((single.2, single.3), (2, true));
        assert_eq!((wave.2, wave.3), (2, true));
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
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("fixture");
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
                as Box<dyn miso_engine_effect_contract::NativeEffectFactory>])
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

        let groups: Vec<_> = report.groups_in(RackLocationV1::Simd1).collect();
        assert_eq!(groups.len(), 1, "one cohort for one shared rack program");
        assert_eq!(
            groups[0].program.len(),
            2,
            "the cohort is the two-slot chain"
        );
        assert!(groups[0].is_full());

        let bound: Vec<_> = report.bound_slots_in(RackLocationV1::Simd1).collect();
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
        assert!(report.scalar_in(RackLocationV1::Simd1).is_empty());
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

        let groups: Vec<_> = report.groups_in(RackLocationV1::Simd1).collect();
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
        let bound: Vec<_> = report.bound_slots_in(RackLocationV1::Simd1).collect();
        assert_eq!(bound.len(), 1);
        assert_eq!(bound[0].slot, 0);
        assert_eq!(bound[0].members.len(), lanes);
        assert_eq!(report.scalar_in(RackLocationV1::Simd1).len(), lanes / 2);
    }

    /// Twelve tracks that each carry one bankable SIMD-1 effect, plus a route per track.
    ///
    /// Shared by the bank-binding test and by `scalar_dispatch_compiles_without_banks_on_any_host`
    /// (#99 F6), which needs the *same* prepared session compiled twice under two dispatches.
    fn twelve_track_bank_fixture() -> (
        miso_engine_session::CompiledSession,
        NativeEffectRegistry,
        EffectPreparedSession,
    ) {
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("fixture");
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
                as Box<dyn miso_engine_effect_contract::NativeEffectFactory>])
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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
            let (banks, report) = bind_rack_banks(&rebound, &ids, &dependency_levels, dispatch)
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
                report.scalar_in(RackLocationV1::Simd1).len()
                    + report.scalar_in(RackLocationV1::Simd2).len(),
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
            id: miso_engine_effect_contract::PortId::new("sidechain").expect("static port"),
            required: false,
        };
        let connected_ids = ids_for(&connected_fallback);
        let connected_banks = bind_rack_banks(
            &connected_fallback,
            &connected_ids,
            &dependency_levels,
            eight,
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
                .scalar_in(RackLocationV1::Simd1)
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
        let (split_banks, split_report) =
            bind_rack_banks(&same_wave, &same_wave_ids, &incompatible_levels, eight)
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
        let error = match bind_rack_banks(&rejected, &rejected_ids, &dependency_levels, eight) {
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
        if std::env::var_os("MISO_ENGINE_ISSUE37_AUDIT").is_some() {
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
            // route's folded 2x2 is re-applied here with the exact software FMA, and the output
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
                        #[cfg(not(target_arch = "wasm32"))]
                        worker_lease: None,
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
                                    miso_engine_lane::softfma::fma_f32_via_f64(
                                        coefficients[1],
                                        right,
                                        coefficients[0] * left,
                                    ),
                                    miso_engine_lane::softfma::fma_f32_via_f64(
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
                    #[cfg(not(target_arch = "wasm32"))]
                    worker_lease: None,
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
            // route spends one multiply and one fused multiply-add with the gain folded in at
            // bind. Neither value is pinned from production output: the oracle block above
            // re-derives the expected PCM for this exact session from the recorded per-track
            // post-matrix contributions, re-applying both frozen op orders with scalar
            // `softfma::fma_f32_via_f64` and `reduce`, and asserts it bit for bit before this
            // literal is compared. (The previous re-pin note stands: 0x9f30_db02_2065_6d79 was already
            // stale on `origin/main` before either branch existed.)
            assert_eq!(
                output_hash, 0x5b3e_672a_ae5d_97aa,
                "deterministic mixed output hash"
            );

            // The same session through the native dependency-wave executor: bit-identical PCM
            // over the same 100,000 blocks, and the same zero-allocation render (#98 F2/F7).
            let native_effects = prepare_native_session_effects(
                &session,
                &registry,
                EffectCompileCaps {
                    maximum_total_state_bytes: 1 << 20,
                    maximum_scratch_bytes: 1 << 20,
                    maximum_automation_spans_per_block: 32,
                },
            )
            .expect("native audit effects");
            let native_builtins = prepare_session_builtins(
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
            .expect("native audit builtins");
            let native_artifact =
                GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
                    dispatch: host_dispatch(),
                    plan_id: 1_002,
                    effects: native_effects,
                    builtins: native_builtins,
                    caps: integration_caps(),
                })
                .unwrap_or_else(|_| panic!("native audit graph"));
            let native_envelope = native_artifact.envelope();
            let native_nodes = native_artifact
                .external_binding_nodes()
                .map(|node| GraphNodeBinding::new(node.clone(), asymmetric_input_binding(node)))
                .collect();
            let mut native_plan = native_artifact
                .into_bound_native(
                    GraphRuntimeBindings {
                        #[cfg(not(target_arch = "wasm32"))]
                        worker_lease: None,
                        envelope: native_envelope,
                        nodes: native_nodes,
                        observers: Vec::new(),
                    },
                    NativeGraphBindConfigV1 {
                        render_mode: NativeGraphRenderModeV1::SingleThread,
                        scheduler: NativeSchedulerConfigV1::new(
                            NonZeroUsize::new(4).expect("four lanes"),
                            true,
                            miso_engine_graph::NativeWorkerPoolShapeV1::default(),
                        ),
                        maximum_retained_bytes: 1 << 28,
                    },
                )
                .unwrap_or_else(|failure| panic!("native audit bind: {}", failure.code))
                .prepared
                .into_plan();
            let mut native_pcm = vec![0.0_f32; frames * 2];
            audit::warm_up();
            audit::reset();
            let mut native_hash = 0xcbf2_9ce4_8422_2325_u64;
            for block in 0..100_000_u64 {
                native_plan
                    .render(
                        RenderIo {
                            input: None,
                            output: PlanarBufferMut::try_new(&mut native_pcm, 2, frames, frames)
                                .expect("native audit output"),
                        },
                        RenderTime {
                            absolute_sample: block * frames as u64,
                        },
                    )
                    .expect("native audit render");
                for sample in &native_pcm {
                    native_hash ^= u64::from(sample.to_bits());
                    native_hash = native_hash.wrapping_mul(0x0000_0100_0000_01b3);
                }
            }
            assert_eq!(
                audit::snapshot().total(),
                0,
                "native render allocates nothing"
            );
            assert_eq!(
                native_hash, output_hash,
                "sequential and native disagree on the production twelve-track session"
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
        ) -> Result<(), miso_engine_core::realtime::RenderError> {
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
        let nine = parse_session_toml(PARAMETRIC_EQ_NINE_TRACK_FIXTURE)
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
            let registry = launch_native_effect_registry_v1().expect("launch registry");
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
            let PreparedGraphArtifact { graph, report: _ } = artifact;
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
                    #[cfg(not(target_arch = "wasm32"))]
                    worker_lease: None,
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
            // G5: master plan §4.5 -- exactly one transpose per bank chain per block.
            assert_eq!(
                plan.bank_transposes(),
                BLOCKS * bank_count as u64,
                "one planar/AoSoA round-trip per chain per block"
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
        let model = parse_session_toml(PARAMETRIC_EQ_NINE_TRACK_FIXTURE)
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
        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
                .bound_groups_in(RackLocationV1::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            bank_artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocationV1::Simd1)
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
                .scalar_in(RackLocationV1::Simd1)
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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
        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
                    .bound_groups_in(RackLocationV1::Simd1)
                    .count(),
                expected_banks
            );
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
                    .len(),
                expected_scalar_tails
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq8")
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq9")
            );
        } else {
            assert_eq!(artifact.graph.prepared_bank_count(), 0);
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
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
            graph: bank_graph,
            report: _,
        } = artifact;
        let PreparedGraphArtifact {
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("compressor bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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
        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
                    .bound_groups_in(RackLocationV1::Simd1)
                    .count(),
                expected_banks
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .bound_groups_in(RackLocationV1::Simd1)
                    .all(|bank| bank.active_count() == lanes)
            );
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
                    .len(),
                expected_scalar_tails
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq8")
            );
            assert!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
                    .iter()
                    .any(|id| id.track_id.as_str() == "eq9")
            );
        } else {
            assert_eq!(artifact.graph.prepared_bank_count(), 0);
            assert_eq!(
                artifact
                    .report
                    .rack_cohorts
                    .scalar_in(RackLocationV1::Simd1)
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
            graph: bank_graph,
            report: _,
        } = artifact;
        let PreparedGraphArtifact {
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("gate/expander bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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

        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
                .bound_groups_in(RackLocationV1::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocationV1::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members: Vec<Vec<String>> = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocationV1::Simd1)
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
            .scalar_in(RackLocationV1::Simd1)
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
            * (u64::try_from(core::mem::size_of::<
                miso_engine_graph::GraphPreparedEffectBank,
            >())
            .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocationV1::Simd1)
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
            graph: bank_graph,
            report: _,
        } = artifact;
        let PreparedGraphArtifact {
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("limiter bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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

        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
                .bound_groups_in(RackLocationV1::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocationV1::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members: Vec<Vec<String>> = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocationV1::Simd1)
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
                .scalar_in(RackLocationV1::Simd1)
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
            * (u64::try_from(core::mem::size_of::<
                miso_engine_graph::GraphPreparedEffectBank,
            >())
            .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocationV1::Simd1)
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
            graph: bank_graph, ..
        } = artifact;
        let PreparedGraphArtifact {
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("multiband bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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

        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
                .bound_groups_in(RackLocationV1::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocationV1::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocationV1::Simd1)
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
                .scalar_in(RackLocationV1::Simd1)
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
            * (u64::try_from(core::mem::size_of::<
                miso_engine_graph::GraphPreparedEffectBank,
            >())
            .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocationV1::Simd1)
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
            graph: bank_graph, ..
        } = artifact;
        let PreparedGraphArtifact {
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("soft-clip bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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

        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
                .bound_groups_in(RackLocationV1::Simd1)
                .count(),
            expected_banks
        );
        assert_eq!(
            artifact
                .report
                .rack_cohorts
                .scalar_in(RackLocationV1::Simd1)
                .len(),
            expected_scalar_tails
        );
        let actual_members = artifact
            .report
            .rack_cohorts
            .bound_groups_in(RackLocationV1::Simd1)
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
                .scalar_in(RackLocationV1::Simd1)
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
            * (u64::try_from(core::mem::size_of::<
                miso_engine_graph::GraphPreparedEffectBank,
            >())
            .expect("bank metadata size")
                + lanes)
            + artifact
                .report
                .rack_cohorts
                .bound_groups_in(RackLocationV1::Simd1)
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
            graph: bank_graph, ..
        } = artifact;
        let PreparedGraphArtifact {
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes: bank_nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("transient bank bind: {}", failure.code));
        let mut scalar_plan = scalar_graph
            .bind(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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

        let registry = launch_native_effect_registry_v1().expect("launch registry");
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
            entry.rack == miso_engine_effect_compiler::EffectRack::Dynamic
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
        for rack in [RackLocationV1::Simd1, RackLocationV1::Simd2] {
            assert_eq!(artifact.report.rack_cohorts.groups_in(rack).count(), 0);
            assert!(artifact.report.rack_cohorts.scalar_in(rack).is_empty());
        }
        assert!(artifact.report.rack_cohorts.plan.groups.is_empty());
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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
        let prepare_artifact = |plan_id, session: miso_engine_session::CompiledSession| {
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
        let native_artifact = prepare_artifact(79, compiled);
        let dispatch = Backend::current();
        // #86 F3: `T.div_ceil(W)` banks, the last one padded with identity lanes, and no
        // scalar post-input tail on a vector host.
        let expected_banks = BankWidth::for_backend(dispatch)
            .map_or(0, |width| 12_usize.div_ceil(width.lanes() as usize));
        let expected_tail = BankWidth::for_backend(dispatch).map_or(12, |_| 0);
        assert_eq!(artifact.prepared_builtin_bank_count(), expected_banks);
        assert_eq!(
            native_artifact.prepared_builtin_bank_count(),
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
            native_artifact.graph().sequential_schedule
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
        let member_ids: Vec<_> = artifact
            .prepared_builtin_banks()
            .flat_map(|bank| {
                assert_eq!(bank.backend, dispatch);
                assert_eq!(Some(bank.width), BankWidth::for_backend(dispatch));
                assert!(!bank.members.is_empty());
                assert!(bank.members.len() <= bank.width.lanes() as usize);
                bank.members.iter().map(|member| match member {
                    GraphNodeId::TrackStage { track_id, stage } => {
                        assert_eq!(*stage, TrackStage::PostInputBuiltins);
                        track_id.as_str().to_owned()
                    }
                    _ => panic!("builtin bank member kind"),
                })
            })
            .collect();
        let mut expected_member_ids: Vec<_> = (0..12).map(|index| format!("bank{index}")).collect();
        expected_member_ids.sort();
        if BankWidth::for_backend(dispatch).is_none() {
            expected_member_ids.clear();
        }
        assert_eq!(member_ids, expected_member_ids);
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
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(bound) => bound,
            Err(_) => panic!("sealed builtin bank bind"),
        };
        let native_envelope = native_artifact.envelope();
        let native_nodes = native_artifact
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
        let native_bound = match native_artifact.into_bound_native(
            GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope: native_envelope,
                nodes: native_nodes,
                observers: Vec::new(),
            },
            NativeGraphBindConfigV1 {
                render_mode: NativeGraphRenderModeV1::SingleThread,
                scheduler: NativeSchedulerConfigV1::new(
                    NonZeroUsize::new(4).expect("four lanes"),
                    true,
                    miso_engine_graph::NativeWorkerPoolShapeV1::default(),
                ),
                maximum_retained_bytes: 1 << 28,
            },
        ) {
            Ok(bound) => bound,
            Err(_) => panic!("sealed native builtin bank bind"),
        };
        assert_eq!(
            native_bound.prepared.metadata.selection,
            SchedulerSelectionV1::Sequential(FallbackReasonV1::SingleThread)
        );
        let mut plan = bound.plan;
        let mut native_plan = native_bound.prepared.into_plan();
        let frames = envelope.quantum.0 as usize;
        let mut pcm = vec![0.0; frames * 2 * 3];
        let mut native_pcm = vec![0.0; frames * 2 * 3];
        for block in 0..3 {
            let range = block * frames * 2..(block + 1) * frames * 2;
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut pcm[range.clone()], 2, frames, frames)
                        .expect("output"),
                },
                RenderTime {
                    absolute_sample: (block * frames) as u64,
                },
            )
            .expect("production builtin-bank render");
            native_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut native_pcm[range], 2, frames, frames)
                            .expect("native output"),
                    },
                    RenderTime {
                        absolute_sample: (block * frames) as u64,
                    },
                )
                .expect("native production builtin-bank render");
        }
        assert!(pcm[..frames * 2].iter().any(|sample| *sample != 0.0));
        assert_eq!(
            pcm.iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            native_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            plan.qualification_counters(),
            native_plan.qualification_counters()
        );
    }

    #[test]
    fn post_bank_graph_cap_rejects_transactionally_with_both_prepared_inputs() {
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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
            let mut model = parse_session_toml(SESSION_FIXTURE).expect("fixture");
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
            let native_artifact =
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
            // #86 F3: `count.div_ceil(W)` banks per level (one level here), last one padded.
            let expected_banks = width.map_or(0, |width| count.div_ceil(width.lanes() as usize));
            let expected_tail = width.map_or(count, |_| 0);
            assert_eq!(artifact.prepared_builtin_bank_count(), expected_banks);
            assert_eq!(
                native_artifact.prepared_builtin_bank_count(),
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
                native_artifact.graph().sequential_schedule
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes,
                observers: tap_observers,
            }) {
                Ok(bound) => bound.plan,
                Err(_) => panic!("seeded bind"),
            };
            let native_envelope = native_artifact.envelope();
            let native_nodes = native_artifact
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
            let native_bound = native_artifact
                .into_bound_native(
                    GraphRuntimeBindings {
                        #[cfg(not(target_arch = "wasm32"))]
                        worker_lease: None,
                        envelope: native_envelope,
                        nodes: native_nodes,
                        observers: Vec::new(),
                    },
                    NativeGraphBindConfigV1 {
                        render_mode: NativeGraphRenderModeV1::SingleThread,
                        scheduler: NativeSchedulerConfigV1::new(
                            NonZeroUsize::new(4).expect("four lanes"),
                            true,
                            miso_engine_graph::NativeWorkerPoolShapeV1::default(),
                        ),
                        maximum_retained_bytes: 1 << 28,
                    },
                )
                .unwrap_or_else(|failure| panic!("native seeded bind: {}", failure.code));
            assert!(matches!(
                native_bound.prepared.metadata.selection,
                SchedulerSelectionV1::Sequential(_)
            ));
            let mut native_plan = native_bound.prepared.into_plan();
            let frames = envelope.quantum.0 as usize;
            let mut pcm = vec![0.0; frames * 2];
            let mut native_pcm = vec![0.0; frames * 2];
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                        .expect("seeded output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("seeded render");
            native_plan
                .render(
                    RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut native_pcm, 2, frames, frames)
                            .expect("native seeded output"),
                    },
                    RenderTime { absolute_sample: 0 },
                )
                .expect("native seeded render");
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
            let native_counters = native_plan.qualification_counters();
            assert_eq!(counters[0], expected_banks as u64);
            assert_eq!(counters[1], counters[0] * u64::from(envelope.quantum.0));
            assert_eq!(counters, native_counters);
            assert_eq!(
                pcm.iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                native_pcm
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>()
            );
            let pcm_hash = native_pcm
                .iter()
                .fold(0xcbf2_9ce4_8422_2325_u64, |hash, sample| {
                    (hash ^ u64::from(sample.to_bits())).wrapping_mul(0x0000_0100_0000_01b3)
                });
            for byte in format!(
                "{layout}:{value:016x}:{count}:{expected_banks}:{expected_tail}:{pcm_hash:016x}:{:?}",
                native_counters
            )
            .bytes()
            {
                transcript ^= u64::from(byte);
                transcript = transcript.wrapping_mul(0x0000_0100_0000_01b3);
            }
            completed += 1;
        }
        assert_eq!(completed, 100);
        // Re-pinned once by #98 F2 (master plan #83 D9 and the section-8 policy). The membership
        // and counter halves of the transcript string are unchanged; the `pcm_hash` half moved for
        // every layout whose output fan-in is four or more, because the session output's reduction
        // became a left-to-right recursive sum instead of a balanced pairwise tree. It is *not*
        // pinned from production output: the per-layout `assert_eq!` above derives the expected
        // output from the recorded per-track post-matrix contributions folded left to right in the
        // plan's own stable edge order, for all 100 layouts, before this literal is compared.
        // Old value `0x0fc9_bdc8_ff12_0f6e`; layouts with `count <= 3` are bit-identical to it.
        assert_eq!(
            transcript, 0x9dfc_dcf2_0e37_0ef5,
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
            let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
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
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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

    /// #99 F4: the compiled route gain is `miso_engine_math::db_to_gain_f32`, bit for bit.
    ///
    /// -19 dB is the witness: the platform `f64::powf` form this replaced produced
    /// `0x3de5_ca15` on this host, one ulp below the canonical `0x3de5_ca16`, and it produced
    /// whatever the *host's* libm produced on any other. `tests/route_gain.rs` pins both
    /// literals against a live `powf` oracle so this witness cannot go stale silently.
    #[test]
    fn route_transform_uses_the_canonical_db_to_gain_conversion() {
        let mut model = parse_session_toml(SESSION_FIXTURE).expect("session fixture");
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
            miso_engine_math::db_to_gain_f32(-19.0).to_bits(),
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
