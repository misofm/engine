//! Checked source-derived bounds for the protected browser observation ingress.
//!
//! This module only prepares scalar bounds. It does not own a boot path, staging buffer, FFI
//! entry point, operation permit, or render-time state transition.

use core::mem::size_of;

use effect_contract::{RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS, RESPONSE_SNAPSHOT_WORDS};
use engine::realtime::ResponseSnapshotSection;
use host_core::{
    HostObservationController, ObservationAccepted, ObservationApplied, ObservationWorkCost,
    ObservedContinuousSpectrumWindow, SPECTRUM_WINDOW_FRAMES, SpectrumContinuousWindow,
    SpectrumWindow,
};

use super::{
    BootFailure, LIVE_RESPONSE_MAXIMUM_ID_BYTES, LIVE_RESPONSE_MAXIMUM_OWNERS,
    MAXIMUM_COMMAND_RECORDS, ObservationIngressLimits, RESULT_REFUSED_BUDGET,
    RESULT_REFUSED_OPTIONS, WebLiveResponseOwner, WebLiveResponseRequest, WebLiveResponseResult,
    WebLiveResponseSection, WebObservationAdmission, WebObservationCaptureIdentity,
    WebObservationDemand, WebObservationReceipt, WebSpectrumRequest, WebSpectrumStreamMetadata,
    WebSpectrumWindow,
};

const E: u64 = 1;
const S: u64 = 2;
const A: u64 = 1;
const P: u64 = 2;
const R: u64 = 4;
const Q: u64 = 1;
const O: u64 = 2;
const F: u64 = 4;

/// Prepared shape facts needed by the checked ingress projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ObservationIngressShape {
    /// Number of compiled tracks in the prepared session.
    pub(crate) all_compiled_tracks: u64,
    /// Number of compiled effect instances in the prepared session.
    pub(crate) all_compiled_effect_instances: u64,
    /// Number of effect instances on the selected response track.
    pub(crate) selected_track_effect_instances: u64,
}

/// Existing bridge and future protected staging bytes supplied by the boot transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ObservationIngressBridge {
    /// Full retained bridge/audio/document staging already projected by the host.
    pub(crate) projected_full_retained: u64,
    /// Actual additive allocations and containing-layout deltas for protected staging.
    pub(crate) additive_staging_bytes: u64,
    /// Actual private target identity allocation.
    pub(crate) private_target_allocation_bytes: u64,
}

/// Existing record sizes used by the projection's source-derived copy bounds.
///
/// The four canonical scalar ABI records are deliberately absent from this table. Their actual
/// `size_of` values are read directly by [`project_observation_ingress`] so a mirror layout or a
/// guessed byte constant cannot enter the budget arithmetic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
struct ObservationIngressSizes {
    host_controller: u64,
    response_snapshot_section: u64,
    live_response_section: u64,
    live_response_owner: u64,
    live_response_result: u64,
    live_response_request: u64,
    spectrum_request: u64,
    spectrum_window: u64,
    spectrum_stream_metadata: u64,
    spectrum_value_window: u64,
    spectrum_continuous_window: u64,
    observed_continuous_window: u64,
    observation_accepted: u64,
    observation_applied: u64,
    observation_work_cost: u64,
}

impl ObservationIngressSizes {
    /// Read the actual sizes of the existing native and browser records.
    #[allow(dead_code)]
    fn actual() -> Result<Self, BootFailure> {
        Ok(Self {
            host_controller: actual_size::<HostObservationController>()?,
            response_snapshot_section: actual_size::<ResponseSnapshotSection>()?,
            live_response_section: actual_size::<WebLiveResponseSection>()?,
            live_response_owner: actual_size::<WebLiveResponseOwner>()?,
            live_response_result: actual_size::<WebLiveResponseResult>()?,
            live_response_request: actual_size::<WebLiveResponseRequest>()?,
            spectrum_request: actual_size::<WebSpectrumRequest>()?,
            spectrum_window: actual_size::<WebSpectrumWindow>()?,
            spectrum_stream_metadata: actual_size::<WebSpectrumStreamMetadata>()?,
            spectrum_value_window: actual_size::<SpectrumWindow>()?,
            spectrum_continuous_window: actual_size::<SpectrumContinuousWindow>()?,
            observed_continuous_window: actual_size::<ObservedContinuousSpectrumWindow>()?,
            observation_accepted: actual_size::<ObservationAccepted>()?,
            observation_applied: actual_size::<ObservationApplied>()?,
            observation_work_cost: actual_size::<ObservationWorkCost>()?,
        })
    }
}

/// Checked source-derived ingress bounds cached by a protected owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ObservationIngressBounds {
    pub(crate) admission_entry_visits: u64,
    pub(crate) response_binding_visits: u64,
    pub(crate) response_section_visits: u64,
    pub(crate) response_copy_bytes: u64,
    pub(crate) handler_copy_bytes_per_boundary: u64,
    pub(crate) cleanup_entry_visits_per_boundary: u64,
    pub(crate) retained_bytes: u64,
}

/// Fixed per-call work intentionally excluded from the per-boundary payload-copy bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ObservationIngressPerCallFacts {
    pub(crate) diagnostic_bytes_per_call: u64,
    pub(crate) raw_kind_scan_visits_per_call: u64,
    pub(crate) raw_kind_scan_bytes_per_call: u64,
}

/// Complete checked ingress projection returned to the later protected boot step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ObservationIngressProjection {
    pub(crate) bounds: ObservationIngressBounds,
    pub(crate) packed_response_bytes: u64,
    pub(crate) packed_spectrum_bytes: u64,
    pub(crate) per_call: ObservationIngressPerCallFacts,
}

/// Scalar protected ingress state prepared for the later operation-mediation checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ObservationIngressState {
    pub(crate) epoch: u64,
    pub(crate) ordinary_used: bool,
    pub(crate) removal_used: bool,
    pub(crate) limits: ObservationIngressLimits,
    pub(crate) bounds: ObservationIngressBounds,
}

#[allow(dead_code)]
impl ObservationIngressState {
    pub(crate) const INITIAL_EPOCH: u64 = 1;

    pub(crate) const fn new(
        limits: ObservationIngressLimits,
        bounds: ObservationIngressBounds,
    ) -> Self {
        Self {
            epoch: Self::INITIAL_EPOCH,
            ordinary_used: false,
            removal_used: false,
            limits,
            bounds,
        }
    }
}

/// Project the frozen protected observation ingress envelope with checked arithmetic.
#[allow(dead_code)]
pub(crate) fn project_observation_ingress(
    shape: ObservationIngressShape,
    bridge: ObservationIngressBridge,
    native_reserved: u64,
) -> Result<ObservationIngressProjection, BootFailure> {
    let sizes = ObservationIngressSizes::actual()?;
    let l = actual_u64(LIVE_RESPONSE_MAXIMUM_ID_BYTES)?;
    let maximum_owners = actual_u64(LIVE_RESPONSE_MAXIMUM_OWNERS)?;
    let n = actual_u64(RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS)?;
    let response_words = actual_u64(RESPONSE_SNAPSHOT_WORDS)?;
    let spectrum_frames = actual_u64(SPECTRUM_WINDOW_FRAMES)?;
    let word_bytes = actual_size::<u32>()?;
    let sample_bytes = actual_size::<f32>()?;

    let b = add(
        shape.all_compiled_tracks,
        shape.all_compiled_effect_instances,
    )?;
    let u = add(1, shape.selected_track_effect_instances)?;
    if u > maximum_owners {
        return Err(option_failure("web.observation.response_owner_count"));
    }
    // The later sink uses this count as an indexable bounded owner count. Keep the conversion
    // checked even though the frozen <=256 guard makes it small on supported targets.
    let _owner_count = usize::try_from(u).map_err(|_| arithmetic_failure())?;
    let _section_count = usize::try_from(n).map_err(|_| arithmetic_failure())?;

    let w = mul(response_words, word_bytes)?;
    let pcm = mul(mul(2, spectrum_frames)?, sample_bytes)?;
    let packed_response = add(
        sizes.live_response_result,
        add(
            mul(maximum_owners, sizes.live_response_owner)?,
            mul(
                u,
                add(mul(3, l)?, mul(mul(2, n)?, sizes.live_response_section)?)?,
            )?,
        )?,
    )?;
    let packed_spectrum = add(sizes.spectrum_window, pcm)?;

    let admission_entry_visits = mul(
        O,
        add(
            add(add(mul(3, R)?, mul(4, P)?)?, E)?,
            add(
                add(mul(5, S)?, mul(A, add(8, mul(2, add(S, 1)?)?)?)?)?,
                mul(2, Q)?,
            )?,
        )?,
    )?;
    let response_binding_visits = b;
    let response_section_visits = mul(mul(8, u)?, n)?;
    let response_copy_bytes = add(
        add(
            add(
                add(
                    mul(2, sizes.live_response_request)?,
                    mul(mul(2, add(b, 2)?)?, l)?,
                )?,
                mul(
                    mul(mul(8, u)?, n)?,
                    add(
                        add(sizes.response_snapshot_section, w)?,
                        sizes.live_response_section,
                    )?,
                )?,
            )?,
            add(
                mul(mul(2, u)?, sizes.live_response_owner)?,
                mul(mul(3, u)?, l)?,
            )?,
        )?,
        add(
            add(
                mul(2, sizes.live_response_result)?,
                mul(2, packed_response)?,
            )?,
            mul(4, actual_size::<WebObservationCaptureIdentity>()?)?,
        )?,
    )?;

    let packed_spectrum_for_copy = mul(2, packed_spectrum)?;
    let spectrum_copy = add(
        add(
            add(
                add(
                    mul(6, sizes.observed_continuous_window)?,
                    add(
                        mul(2, sizes.spectrum_continuous_window)?,
                        mul(2, sizes.spectrum_value_window)?,
                    )?,
                )?,
                add(mul(2, pcm)?, mul(2, sizes.spectrum_window)?)?,
            )?,
            add(
                mul(4, sizes.spectrum_stream_metadata)?,
                packed_spectrum_for_copy,
            )?,
        )?,
        add(
            add(
                mul(4, actual_size::<WebObservationCaptureIdentity>()?)?,
                mul(2, sizes.spectrum_request)?,
            )?,
            mul(2, l)?,
        )?,
    )?;

    let control_copy = add(
        add(
            add(
                mul(4, sizes.host_controller)?,
                mul(8, sizes.observation_accepted)?,
            )?,
            add(
                mul(8, sizes.observation_work_cost)?,
                mul(4, actual_size::<WebObservationReceipt>()?)?,
            )?,
        )?,
        add(
            mul(
                2,
                add(
                    add(
                        sizes.spectrum_request,
                        actual_size::<WebObservationDemand>()?,
                    )?,
                    l,
                )?,
            )?,
            mul(2, sizes.observed_continuous_window)?,
        )?,
    )?;

    let receipt_copy = add(
        mul(
            F,
            add(
                add(
                    mul(2, sizes.host_controller)?,
                    mul(4, sizes.observation_applied)?,
                )?,
                mul(4, actual_size::<WebObservationReceipt>()?)?,
            )?,
        )?,
        mul(4, mul(add(R, F)?, actual_size::<WebObservationReceipt>()?)?)?,
    )?;
    let retirement_copy = mul(
        add(mul(mul(F, S)?, Q)?, mul(S, Q)?)?,
        sizes.observed_continuous_window,
    )?;
    let handler_copy_bytes_per_boundary = add(
        add(response_copy_bytes, spectrum_copy)?,
        add(mul(O, control_copy)?, add(receipt_copy, retirement_copy)?)?,
    )?;
    let cleanup_entry_visits_per_boundary = add(
        add(
            mul(F, add(add(add(add(1, P)?, mul(2, S)?)?, mul(S, Q)?)?, R)?)?,
            mul(mul(2, R)?, add(R, F)?)?,
        )?,
        add(
            add(add(add(P, R)?, mul(2, S)?)?, mul(S, Q)?)?,
            add(mul(2, Q)?, S)?,
        )?,
    )?;

    let corrected_bridge = add(
        add(
            bridge.projected_full_retained,
            bridge.additive_staging_bytes,
        )?,
        bridge.private_target_allocation_bytes,
    )?
    .checked_sub(sizes.host_controller)
    .ok_or_else(arithmetic_failure)?;
    let retained_bytes = add(native_reserved, corrected_bridge)?;

    let diagnostic_bytes_per_call = mul(2, actual_size::<WebObservationAdmission>()?)?;
    let raw_kind_scan_visits_per_call = u64::from(MAXIMUM_COMMAND_RECORDS);
    let raw_kind_scan_bytes_per_call = mul(raw_kind_scan_visits_per_call, actual_size::<u32>()?)?;

    Ok(ObservationIngressProjection {
        bounds: ObservationIngressBounds {
            admission_entry_visits,
            response_binding_visits,
            response_section_visits,
            response_copy_bytes,
            handler_copy_bytes_per_boundary,
            cleanup_entry_visits_per_boundary,
            retained_bytes,
        },
        packed_response_bytes: packed_response,
        packed_spectrum_bytes: packed_spectrum,
        per_call: ObservationIngressPerCallFacts {
            diagnostic_bytes_per_call,
            raw_kind_scan_visits_per_call,
            raw_kind_scan_bytes_per_call,
        },
    })
}

/// Validate explicit caller-supplied ingress limits against a checked projection.
#[allow(dead_code)]
pub(crate) fn validate_observation_ingress_limits(
    limits: ObservationIngressLimits,
    projection: &ObservationIngressProjection,
) -> Result<(), BootFailure> {
    let id_bytes = actual_u64(LIVE_RESPONSE_MAXIMUM_ID_BYTES)?;
    let minimum_control_bytes = actual_size::<WebObservationDemand>()?
        .max(add(actual_size::<WebLiveResponseRequest>()?, id_bytes)?)
        .max(add(actual_size::<WebSpectrumRequest>()?, id_bytes)?);
    if limits.ordinary_operations_per_boundary != 1 || limits.removal_operations_per_boundary != 1 {
        return Err(option_failure("web.observation.ingress.operation_counts"));
    }
    if limits.maximum_control_bytes > 8_192 {
        return Err(option_failure(
            "web.observation.ingress.maximum_control_bytes",
        ));
    }
    if u64::from(limits.maximum_control_bytes) < minimum_control_bytes {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_control_bytes",
        ));
    }
    if limits.maximum_observation_rows > 32 {
        return Err(option_failure(
            "web.observation.ingress.maximum_observation_rows",
        ));
    }
    if limits.maximum_observation_rows < R as u32 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_observation_rows",
        ));
    }
    if limits.maximum_result_bytes > 65_536 {
        return Err(option_failure(
            "web.observation.ingress.maximum_result_bytes",
        ));
    }
    if limits.maximum_result_bytes == 0 {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_result_bytes",
        ));
    }
    let maximum_result_bytes = u64::from(limits.maximum_result_bytes);
    if projection.packed_response_bytes > maximum_result_bytes
        || projection.packed_spectrum_bytes > maximum_result_bytes
    {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_result_bytes",
        ));
    }
    if projection.bounds.admission_entry_visits > limits.maximum_admission_entry_visits {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_admission_entry_visits",
        ));
    }
    if projection.bounds.response_binding_visits > limits.maximum_response_binding_visits {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_response_binding_visits",
        ));
    }
    if projection.bounds.response_section_visits > limits.maximum_response_section_visits {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_response_section_visits",
        ));
    }
    if projection.bounds.response_copy_bytes > limits.maximum_response_copy_bytes {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_response_copy_bytes",
        ));
    }
    if projection.bounds.handler_copy_bytes_per_boundary
        > limits.maximum_handler_copy_bytes_per_boundary
    {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_handler_copy_bytes_per_boundary",
        ));
    }
    if projection.bounds.cleanup_entry_visits_per_boundary
        > limits.maximum_cleanup_entry_visits_per_boundary
    {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_cleanup_entry_visits_per_boundary",
        ));
    }
    if projection.bounds.retained_bytes > limits.maximum_retained_bytes {
        return Err(BootFailure::fixed(
            RESULT_REFUSED_BUDGET,
            "web.observation.ingress.maximum_retained_bytes",
        ));
    }
    Ok(())
}

fn actual_size<T>() -> Result<u64, BootFailure> {
    u64::try_from(size_of::<T>()).map_err(|_| arithmetic_failure())
}

fn actual_u64(value: usize) -> Result<u64, BootFailure> {
    u64::try_from(value).map_err(|_| arithmetic_failure())
}

fn add(left: u64, right: u64) -> Result<u64, BootFailure> {
    left.checked_add(right).ok_or_else(arithmetic_failure)
}

fn mul(left: u64, right: u64) -> Result<u64, BootFailure> {
    left.checked_mul(right).ok_or_else(arithmetic_failure)
}

fn arithmetic_failure() -> BootFailure {
    BootFailure::fixed(RESULT_REFUSED_BUDGET, "host.budget.arithmetic")
}

fn option_failure(code: &str) -> BootFailure {
    BootFailure::fixed(RESULT_REFUSED_OPTIONS, code)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape() -> ObservationIngressShape {
        ObservationIngressShape {
            all_compiled_tracks: 3,
            all_compiled_effect_instances: 5,
            selected_track_effect_instances: 2,
        }
    }

    fn bridge() -> ObservationIngressBridge {
        ObservationIngressBridge {
            projected_full_retained: actual_size::<HostObservationController>().unwrap() + 10,
            additive_staging_bytes: 20,
            private_target_allocation_bytes: 30,
        }
    }

    fn limits() -> ObservationIngressLimits {
        ObservationIngressLimits {
            maximum_control_bytes: 8_192,
            maximum_observation_rows: 32,
            maximum_result_bytes: 65_536,
            ordinary_operations_per_boundary: 1,
            removal_operations_per_boundary: 1,
            maximum_admission_entry_visits: u64::MAX,
            maximum_response_binding_visits: u64::MAX,
            maximum_response_section_visits: u64::MAX,
            maximum_response_copy_bytes: u64::MAX,
            maximum_handler_copy_bytes_per_boundary: u64::MAX,
            maximum_cleanup_entry_visits_per_boundary: u64::MAX,
            maximum_retained_bytes: u64::MAX,
        }
    }

    fn minimum_control_bytes() -> u32 {
        let id_bytes = LIVE_RESPONSE_MAXIMUM_ID_BYTES as u64;
        actual_size::<WebObservationDemand>()
            .unwrap()
            .max(actual_size::<WebLiveResponseRequest>().unwrap() + id_bytes)
            .max(actual_size::<WebSpectrumRequest>().unwrap() + id_bytes) as u32
    }

    fn project(
        ingress_limits: ObservationIngressLimits,
    ) -> Result<ObservationIngressProjection, BootFailure> {
        let projection = project_observation_ingress(shape(), bridge(), 40)?;
        validate_observation_ingress_limits(ingress_limits, &projection)?;
        Ok(projection)
    }

    #[test]
    fn frozen_entry_and_cleanup_visit_bounds_are_exact() {
        let result = project(limits()).unwrap();
        assert_eq!(result.bounds.admission_entry_visits, 94);
        assert_eq!(result.bounds.cleanup_entry_visits_per_boundary, 132);
    }

    #[test]
    fn exact_limits_are_admitted_and_one_below_each_bound_is_refused() {
        let broad = project(limits()).unwrap();
        let bounds = broad.bounds;
        let exact = ObservationIngressLimits {
            maximum_control_bytes: minimum_control_bytes(),
            maximum_observation_rows: 4,
            maximum_result_bytes: broad.packed_response_bytes.max(broad.packed_spectrum_bytes)
                as u32,
            ordinary_operations_per_boundary: 1,
            removal_operations_per_boundary: 1,
            maximum_admission_entry_visits: bounds.admission_entry_visits,
            maximum_response_binding_visits: bounds.response_binding_visits,
            maximum_response_section_visits: bounds.response_section_visits,
            maximum_response_copy_bytes: bounds.response_copy_bytes,
            maximum_handler_copy_bytes_per_boundary: bounds.handler_copy_bytes_per_boundary,
            maximum_cleanup_entry_visits_per_boundary: bounds.cleanup_entry_visits_per_boundary,
            maximum_retained_bytes: bounds.retained_bytes,
        };
        assert!(project(exact).is_ok());

        macro_rules! one_below {
            ($field:ident, $value:expr) => {{
                let mut below = exact;
                below.$field = $value.checked_sub(1).unwrap();
                assert_eq!(project(below).unwrap_err().result(), RESULT_REFUSED_BUDGET);
            }};
        }
        one_below!(
            maximum_admission_entry_visits,
            bounds.admission_entry_visits
        );
        one_below!(
            maximum_response_binding_visits,
            bounds.response_binding_visits
        );
        one_below!(
            maximum_response_section_visits,
            bounds.response_section_visits
        );
        one_below!(maximum_response_copy_bytes, bounds.response_copy_bytes);
        one_below!(
            maximum_handler_copy_bytes_per_boundary,
            bounds.handler_copy_bytes_per_boundary
        );
        one_below!(
            maximum_cleanup_entry_visits_per_boundary,
            bounds.cleanup_entry_visits_per_boundary
        );
        one_below!(maximum_retained_bytes, bounds.retained_bytes);

        let mut below_result = exact;
        below_result.maximum_result_bytes = exact.maximum_result_bytes - 1;
        assert_eq!(
            project(below_result).unwrap_err().result(),
            RESULT_REFUSED_BUDGET
        );
    }

    #[test]
    fn fixed_request_and_receipt_row_minima_are_checked() {
        let mut ingress_limits = limits();
        ingress_limits.maximum_control_bytes = minimum_control_bytes();
        ingress_limits.maximum_observation_rows = 4;
        assert!(project(ingress_limits).is_ok());

        ingress_limits.maximum_control_bytes -= 1;
        assert_eq!(
            project(ingress_limits).unwrap_err().result(),
            RESULT_REFUSED_BUDGET
        );
        ingress_limits.maximum_control_bytes = minimum_control_bytes();
        ingress_limits.maximum_observation_rows = 3;
        assert_eq!(
            project(ingress_limits).unwrap_err().result(),
            RESULT_REFUSED_BUDGET
        );
    }

    #[test]
    fn checked_overflow_and_full_controller_deduction_refuse_or_project_exactly() {
        let mut overflowing_shape = shape();
        overflowing_shape.all_compiled_tracks = u64::MAX;
        assert_eq!(
            project_observation_ingress(overflowing_shape, bridge(), 40)
                .unwrap_err()
                .result(),
            RESULT_REFUSED_BUDGET
        );

        let host_controller = actual_size::<HostObservationController>().unwrap();
        let mut underflowing_bridge = bridge();
        underflowing_bridge.projected_full_retained = host_controller - 1;
        underflowing_bridge.additive_staging_bytes = 0;
        underflowing_bridge.private_target_allocation_bytes = 0;
        assert_eq!(
            project_observation_ingress(shape(), underflowing_bridge, 40)
                .unwrap_err()
                .result(),
            RESULT_REFUSED_BUDGET
        );

        let result = project_observation_ingress(shape(), bridge(), 40).unwrap();
        assert_eq!(result.bounds.retained_bytes, 100);
        assert_eq!(
            result.per_call.diagnostic_bytes_per_call,
            2 * size_of::<WebObservationAdmission>() as u64
        );
        assert_eq!(
            result.per_call.raw_kind_scan_visits_per_call,
            u64::from(MAXIMUM_COMMAND_RECORDS)
        );
    }
}
