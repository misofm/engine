//! Deterministic off-render cohort selection for homogeneous AoSoA racks.
#![allow(missing_docs)]

use miso_engine_rack::{KernelDispatch, RackProgramSignatureV1};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RackTrackInputV1 {
    pub track_id: Box<str>,
    pub signature: RackProgramSignatureV1,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RackMemberV1 {
    pub track_id: Box<str>,
    pub active_slots: Box<[bool]>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RackBankV1 {
    pub signature: RackProgramSignatureV1,
    pub members: Box<[RackMemberV1]>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledRackCohortsV1 {
    pub banks: Box<[RackBankV1]>,
    pub scalar_tails: Box<[RackMemberV1]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RackCompileError {
    DuplicateTrackId,
    ScalarDispatch,
}

/// Compile stable-ID inputs into full-width cohorts followed by stable scalar tails.
pub fn compile_rack_cohorts_v1(
    mut tracks: Vec<RackTrackInputV1>,
    dispatch: KernelDispatch,
) -> Result<CompiledRackCohortsV1, RackCompileError> {
    tracks.sort_by(|a, b| a.track_id.cmp(&b.track_id));
    if tracks
        .windows(2)
        .any(|pair| pair[0].track_id == pair[1].track_id)
    {
        return Err(RackCompileError::DuplicateTrackId);
    }
    let Some(width) = dispatch.bank_width() else {
        return Ok(CompiledRackCohortsV1 {
            banks: Box::new([]),
            scalar_tails: tracks
                .into_iter()
                .map(|track| RackMemberV1 {
                    track_id: track.track_id,
                    active_slots: vec![true; track.signature.slots.len()].into_boxed_slice(),
                })
                .collect(),
        });
    };
    let mut remaining = tracks;
    let mut banks = Vec::new();
    let mut tails = Vec::new();
    while !remaining.is_empty() {
        let mut candidate = 0usize;
        for index in 1..remaining.len() {
            let a = &remaining[index].signature;
            let b = &remaining[candidate].signature;
            if a.slots.len() > b.slots.len()
                || (a.slots.len() == b.slots.len() && a.slots < b.slots)
            {
                candidate = index;
            }
        }
        let signature = remaining[candidate].signature.clone();
        let mut compatible = Vec::new();
        let mut incompatible = Vec::new();
        for track in remaining {
            if let Some(mask) = track.signature.is_subsequence_of(&signature) {
                compatible.push(RackMemberV1 {
                    track_id: track.track_id,
                    active_slots: mask,
                });
            } else {
                incompatible.push(track);
            }
        }
        let lanes = width.lanes() as usize;
        while compatible.len() >= lanes {
            let members: Box<[RackMemberV1]> = compatible.drain(..lanes).collect();
            banks.push(RackBankV1 {
                signature: signature.clone(),
                members,
            });
        }
        tails.extend(compatible);
        remaining = incompatible;
    }
    Ok(CompiledRackCohortsV1 {
        banks: banks.into_boxed_slice(),
        scalar_tails: tails.into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::{KernelBackendV1, TargetCapabilities};
    use miso_engine_rack::{RackLocationV1, RoutingClassV1};
    fn signature(_slots: usize) -> RackProgramSignatureV1 {
        RackProgramSignatureV1::new(
            RackLocationV1::Simd1,
            48_000,
            128,
            Vec::new(),
            RoutingClassV1::MainOnly,
        )
        .unwrap_or_else(|_| unreachable!())
    }
    #[test]
    fn scalar_dispatch_keeps_every_track_as_tail() {
        let dispatch = KernelDispatch::select(TargetCapabilities::from_detected(
            false, false, false, false,
        ));
        assert_eq!(dispatch.backend(), KernelBackendV1::Scalar);
        let result = compile_rack_cohorts_v1(
            vec![RackTrackInputV1 {
                track_id: "a".into(),
                signature: signature(0),
            }],
            dispatch,
        )
        .unwrap();
        assert_eq!(result.scalar_tails.len(), 1);
    }

    #[test]
    fn four_lane_banks_leave_only_stable_scalar_tails_for_every_count() {
        let dispatch =
            KernelDispatch::select(TargetCapabilities::from_detected(true, false, false, false));
        for count in [1usize, 2, 3, 4, 5, 7, 8, 9, 17] {
            let tracks = (0..count)
                .map(|index| RackTrackInputV1 {
                    track_id: format!("t{index:03}").into(),
                    signature: signature(0),
                })
                .collect();
            let compiled = compile_rack_cohorts_v1(tracks, dispatch).expect("compile");
            assert_eq!(compiled.banks.len(), count / 4);
            assert_eq!(compiled.scalar_tails.len(), count % 4);
        }
    }
}
