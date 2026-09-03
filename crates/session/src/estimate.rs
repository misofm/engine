//! Checked, allocation-free preflight estimates for control-plane compilation.

use core::mem::size_of;

use crate::{
    Diagnostic, DiagnosticCode, DiagnosticPath, DiagnosticSet, FieldKey, ModelVisitor,
    SessionModel, StableId, Token, VisitModel, WalkOrder,
};

/// Resource requirements of a normalized session declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceEstimate {
    /// All source, track, submix, and output entities; this is not a product track limit.
    pub entity_count: u64,
    /// Source entity count.
    pub source_count: u64,
    /// Track entity count, reported for observability only.
    pub track_count: u64,
    /// Declared effect instances across all ordered racks.
    pub effect_count: u64,
    /// Declarative route count.
    pub route_count: u64,
    /// Ordered automation segment count.
    pub automation_segment_count: u64,
    /// UTF-8 bytes retained by every owned string/newtype in the model.
    pub retained_string_bytes: u64,
    /// Effect parameter declaration count.
    pub parameter_count: u64,
    /// Session-declared queue items; zero because queues are host policy.
    pub queue_items: u64,
    /// Session-declared queue storage; zero because queues are host policy.
    pub queue_bytes: u64,
    /// Session-declared source-ring frames; zero because the host chooses the ring.
    pub source_ring_frames: u64,
    /// Session-declared source-ring bytes; zero because the host chooses the ring.
    pub source_ring_bytes: u64,
    /// Conservative canonical-writer allocation bound used before writing.
    pub canonical_upper_bound_bytes: u64,
    /// Actual retained canonical bytes after successful compilation.
    pub canonical_bytes: u64,
    /// Model, retained strings, indexes, and canonical snapshot bytes.
    pub compiled_model_bytes: u64,
    /// Session-declared runtime storage; zero because runtime storage is host policy.
    pub requested_runtime_bytes: u64,
    /// Largest individual model/runtime allocation request.
    pub single_allocation_bytes: u64,
    /// Platform allocation ceiling used for `usize`/`isize` checks.
    pub platform_allocation_limit_bytes: u64,
    /// Checked total bytes retained/requested by issue 004.
    pub estimated_bytes: u64,
}

/// Estimate issue-004-owned model/runtime resources without cloning or canonicalizing the model.
pub fn estimate_session_resources(
    session: &SessionModel,
) -> Result<ResourceEstimate, DiagnosticSet> {
    let mut errors = Vec::new();
    let source_count = count(session.sources.len(), "$.sources", &mut errors);
    let track_count = count(session.tracks.len(), "$.tracks", &mut errors);
    let submix_count = count(session.submixes.len(), "$.submixes", &mut errors);
    let output_count = count(session.outputs.len(), "$.outputs", &mut errors);
    let route_count = count(session.routes.len(), "$.routes", &mut errors);
    let automation_count = count(session.automation.len(), "$.automation", &mut errors);
    let effect_count = sum_counts(
        session
            .tracks
            .iter()
            .flat_map(|track| [&track.simd1, &track.dynamic, &track.simd2]),
        |rack| rack.effects.len(),
        "$.tracks",
        &mut errors,
    );
    let parameter_count = sum_counts(
        session
            .tracks
            .iter()
            .flat_map(|track| [&track.simd1, &track.dynamic, &track.simd2])
            .flat_map(|rack| rack.effects.iter()),
        |effect| effect.params.len(),
        "$.tracks",
        &mut errors,
    );
    let automation_segment_count = sum_counts(
        session.automation.iter(),
        |automation| automation.segments.len(),
        "$.automation",
        &mut errors,
    );
    let entity_count = checked_add(
        checked_add(source_count, track_count, "$.entities", &mut errors),
        checked_add(submix_count, output_count, "$.entities", &mut errors),
        "$.entities",
        &mut errors,
    );

    let (retained_string_bytes, largest_string_bytes) = retained_strings(session, &mut errors);
    let mut model_vector_bytes = 0_u64;
    let mut largest_model_allocation = largest_string_bytes;
    macro_rules! vector {
        ($values:expr, $type:ty, $path:literal) => {{
            let bytes = checked_mul(
                count($values.len(), $path, &mut errors),
                size::<$type>(),
                $path,
                &mut errors,
            );
            model_vector_bytes = checked_add(model_vector_bytes, bytes, $path, &mut errors);
            largest_model_allocation = largest_model_allocation.max(bytes);
        }};
    }
    vector!(session.sources, crate::Source, "$.sources");
    vector!(session.tracks, crate::Track, "$.tracks");
    vector!(session.submixes, crate::Submix, "$.submixes");
    vector!(session.outputs, crate::Output, "$.outputs");
    vector!(session.routes, crate::Route, "$.routes");
    vector!(session.automation, crate::Automation, "$.automation");
    for track in &session.tracks {
        for rack in [&track.simd1, &track.dynamic, &track.simd2] {
            let path = "$.tracks.racks.effects";
            let bytes = checked_mul(
                count(rack.effects.len(), path, &mut errors),
                size::<crate::Effect>(),
                path,
                &mut errors,
            );
            model_vector_bytes = checked_add(model_vector_bytes, bytes, path, &mut errors);
            largest_model_allocation = largest_model_allocation.max(bytes);
            for effect in &rack.effects {
                let path = "$.tracks.racks.effects.params";
                let bytes = checked_mul(
                    count(effect.params.len(), path, &mut errors),
                    size::<crate::EffectParam>(),
                    path,
                    &mut errors,
                );
                model_vector_bytes = checked_add(model_vector_bytes, bytes, path, &mut errors);
                largest_model_allocation = largest_model_allocation.max(bytes);
            }
        }
    }
    for automation in &session.automation {
        let path = "$.automation.segments";
        let bytes = checked_mul(
            count(automation.segments.len(), path, &mut errors),
            size::<crate::AutomationSegment>(),
            path,
            &mut errors,
        );
        model_vector_bytes = checked_add(model_vector_bytes, bytes, path, &mut errors);
        largest_model_allocation = largest_model_allocation.max(bytes);
    }

    let index_node_bytes = checked_mul(entity_count, 128, "$.compiled_indexes", &mut errors);
    let structural_items = checked_add(
        checked_add(entity_count, route_count, "$.canonical", &mut errors),
        checked_add(
            checked_add(effect_count, parameter_count, "$.canonical", &mut errors),
            checked_add(
                automation_count,
                automation_segment_count,
                "$.canonical",
                &mut errors,
            ),
            "$.canonical",
            &mut errors,
        ),
        "$.canonical",
        &mut errors,
    );
    let canonical_upper_bound_bytes = checked_add(
        4_096,
        checked_add(
            checked_mul(retained_string_bytes, 10, "$.canonical", &mut errors),
            checked_mul(structural_items, 1_024, "$.canonical", &mut errors),
            "$.canonical",
            &mut errors,
        ),
        "$.canonical",
        &mut errors,
    );
    largest_model_allocation = largest_model_allocation.max(canonical_upper_bound_bytes);

    // Queue depths and the actual chosen ring are host policy. #240 re-derives the exact retained
    // ring bytes after that choice, so the session compiler contributes no declarative runtime
    // projection here.
    let queue_bytes = 0;
    let source_ring_frames = 0;
    let source_ring_bytes = 0;
    let requested_runtime_bytes = 0;
    let compiled_model_upper_bound = checked_add(
        checked_add(
            model_vector_bytes,
            retained_string_bytes,
            "$.compiled_model",
            &mut errors,
        ),
        checked_add(
            index_node_bytes,
            canonical_upper_bound_bytes,
            "$.compiled_model",
            &mut errors,
        ),
        "$.compiled_model",
        &mut errors,
    );
    let single_allocation_bytes = largest_model_allocation;
    let platform_allocation_limit_bytes = u64::try_from(isize::MAX).unwrap_or(u64::MAX);
    for (path, bytes) in [
        ("$.compiled_model", compiled_model_upper_bound),
        ("$.single_allocation", single_allocation_bytes),
        ("$.runtime", requested_runtime_bytes),
    ] {
        if usize::try_from(bytes).is_err() || bytes > platform_allocation_limit_bytes {
            overflow(
                &mut errors,
                path,
                "allocation does not fit the target usize/isize allocation domain",
            );
        }
    }
    let estimated_bytes = checked_add(
        compiled_model_upper_bound,
        requested_runtime_bytes,
        "$.estimated_bytes",
        &mut errors,
    );

    if !errors.is_empty() {
        return Err(DiagnosticSet::from_vec(errors));
    }
    Ok(ResourceEstimate {
        entity_count,
        source_count,
        track_count,
        effect_count,
        route_count,
        automation_segment_count,
        retained_string_bytes,
        parameter_count,
        queue_items: 0,
        queue_bytes,
        source_ring_frames,
        source_ring_bytes,
        canonical_upper_bound_bytes,
        canonical_bytes: 0,
        compiled_model_bytes: compiled_model_upper_bound,
        requested_runtime_bytes,
        single_allocation_bytes,
        platform_allocation_limit_bytes,
        estimated_bytes,
    })
}

pub(crate) use estimate_session_resources as estimate_session;

pub(crate) fn with_canonical_bytes(
    mut estimate: ResourceEstimate,
    actual: usize,
) -> Result<ResourceEstimate, DiagnosticSet> {
    let actual = u64::try_from(actual).map_err(|_| {
        DiagnosticSet::from_vec(vec![Diagnostic::new(
            DiagnosticCode::CapacityArithmeticOverflow,
            DiagnosticPath::root().key("canonical"),
            None,
            "canonical byte length cannot convert to u64",
        )])
    })?;
    if actual > estimate.canonical_upper_bound_bytes {
        return Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
            DiagnosticCode::CapacityArithmeticOverflow,
            DiagnosticPath::root().key("canonical"),
            None,
            "canonical writer exceeded its conservative preflight bound",
        )]));
    }
    estimate.canonical_bytes = actual;
    estimate.compiled_model_bytes =
        estimate.compiled_model_bytes - estimate.canonical_upper_bound_bytes + actual;
    estimate.estimated_bytes = estimate
        .compiled_model_bytes
        .checked_add(estimate.requested_runtime_bytes)
        .ok_or_else(|| {
            DiagnosticSet::from_vec(vec![Diagnostic::new(
                DiagnosticCode::CapacityArithmeticOverflow,
                DiagnosticPath::root().key("estimated_bytes"),
                None,
                "final resource sum overflows u64",
            )])
        })?;
    Ok(estimate)
}

fn retained_strings(session: &SessionModel, errors: &mut Vec<Diagnostic>) -> (u64, u64) {
    let mut visitor = StringBytes {
        total: 0,
        largest: 0,
        errors,
    };
    let _ = session.visit(WalkOrder::Declared, &mut visitor);
    (visitor.total, visitor.largest)
}

struct StringBytes<'a> {
    total: u64,
    largest: u64,
    errors: &'a mut Vec<Diagnostic>,
}
impl StringBytes<'_> {
    fn add(&mut self, value: &str) {
        let path = "$.retained_strings";
        let bytes = count(value.len(), path, self.errors);
        self.total = checked_add(self.total, bytes, path, self.errors);
        self.largest = self.largest.max(bytes);
    }
}
macro_rules! noop_visitor_methods { ($(fn $name:ident($($arg:ident:$ty:ty),*);)+) => {$(
    fn $name(&mut self, $($arg:$ty),*) -> Result<(), Self::Error> { Ok(()) }
)+}; }
impl ModelVisitor for StringBytes<'_> {
    type Error = ();
    noop_visitor_methods! {
        fn record_begin(_key:Option<FieldKey>, _fields:u32);
        fn record_end();
        fn array_begin(_key:FieldKey, _len:usize);
        fn array_end();
        fn wire_tag(_tag:Token);
        fn bool(_key:FieldKey, _value:bool);
        fn u8(_key:FieldKey, _value:u8);
        fn u32(_key:FieldKey, _value:u32);
        fn u64(_key:FieldKey, _value:u64);
        fn source_bit_depth(_key:FieldKey, _value:crate::SourceBitDepth);
        fn f32(_key:FieldKey, _value:f32);
        fn token(_key:FieldKey, _value:Token);
    }
    fn id(&mut self, _: FieldKey, value: &StableId) -> Result<(), Self::Error> {
        self.add(value.as_str());
        Ok(())
    }
    fn text(&mut self, _: FieldKey, value: &str) -> Result<(), Self::Error> {
        self.add(value);
        Ok(())
    }
}

fn size<T>() -> u64 {
    u64::try_from(size_of::<T>()).expect("type size fits u64")
}

fn count(value: usize, path: &str, errors: &mut Vec<Diagnostic>) -> u64 {
    u64::try_from(value).unwrap_or_else(|_| {
        overflow(errors, path, "usize count cannot convert to u64");
        0
    })
}

fn sum_counts<T>(
    values: impl Iterator<Item = T>,
    get_count: impl Fn(T) -> usize,
    path: &str,
    errors: &mut Vec<Diagnostic>,
) -> u64 {
    values.fold(0, |total, value| {
        checked_add(total, count(get_count(value), path, errors), path, errors)
    })
}

fn checked_add(left: u64, right: u64, path: &str, errors: &mut Vec<Diagnostic>) -> u64 {
    left.checked_add(right).unwrap_or_else(|| {
        overflow(errors, path, "resource addition overflows u64");
        0
    })
}

fn checked_mul(left: u64, right: u64, path: &str, errors: &mut Vec<Diagnostic>) -> u64 {
    left.checked_mul(right).unwrap_or_else(|| {
        overflow(errors, path, "resource multiplication overflows u64");
        0
    })
}

fn overflow(errors: &mut Vec<Diagnostic>, path: &str, message: &str) {
    errors.push(Diagnostic::new(
        DiagnosticCode::CapacityArithmeticOverflow,
        DiagnosticPath::from_dotted(path),
        None,
        message,
    ));
}
