//! Allocation boundary proof for package-native operations.

#![allow(unsafe_code)]

use core::{alloc::Layout, cell::Cell};
use std::alloc::{GlobalAlloc, System};

use miso_engine_effect_package::{
    ArtifactSelectionRequestV1, EffectArtifactAuthoringV1, EffectArtifactKindV1,
    EffectPackageAuthoringV1, EffectPackageLimitsV1, effect_descriptor_identity_v1,
    effect_package_cid_v1, effect_package_v1_required_size, encode_effect_package_v1,
    select_effect_package_artifact_v1, verify_effect_package_v1,
};

struct TrackingAllocator;

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static LIVE_BYTES: Cell<i64> = const { Cell::new(0) };
}

fn when_active(action: impl FnOnce()) {
    ACTIVE.with(|active| {
        if active.get() {
            action();
        }
    });
}

// SAFETY: every operation delegates its original pointer/layout unchanged to `System`. The
// thread-local counters are observational and active only around a single test-thread call.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            when_active(|| {
                ALLOCATIONS.set(ALLOCATIONS.get() + 1);
                LIVE_BYTES.set(LIVE_BYTES.get() + layout.size() as i64);
            });
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            when_active(|| {
                ALLOCATIONS.set(ALLOCATIONS.get() + 1);
                LIVE_BYTES.set(LIVE_BYTES.get() + layout.size() as i64);
            });
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        when_active(|| {
            DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
            LIVE_BYTES.set(LIVE_BYTES.get() - layout.size() as i64);
        });
        // SAFETY: delegates the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: delegates the original pointer/layout and requested size unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            when_active(|| {
                ALLOCATIONS.set(ALLOCATIONS.get() + 1);
                DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
                LIVE_BYTES.set(LIVE_BYTES.get() - layout.size() as i64 + new_size as i64);
            });
        }
        replacement
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Snapshot {
    allocations: u64,
    deallocations: u64,
    live_bytes: i64,
}

fn measure<T>(operation: impl FnOnce() -> T) -> (T, Snapshot) {
    ALLOCATIONS.set(0);
    DEALLOCATIONS.set(0);
    LIVE_BYTES.set(0);
    ACTIVE.set(true);
    let result = operation();
    ACTIVE.set(false);
    (
        result,
        Snapshot {
            allocations: ALLOCATIONS.get(),
            deallocations: DEALLOCATIONS.get(),
            live_bytes: LIVE_BYTES.get(),
        },
    )
}

fn fixture() -> Vec<u8> {
    let compact: Vec<_> =
        include_str!("../../../fixtures/effect-package/v1/comprehensive-a.package.hex")
            .bytes()
            .filter(|byte| !byte.is_ascii_whitespace())
            .collect();
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("lowercase hex fixture"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

#[test]
fn each_package_publication_has_one_nested_descriptor_pass_and_no_native_allocation() {
    let bytes = fixture();
    let descriptor_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap()) as usize;
    let descriptor = &bytes[96..96 + descriptor_len];
    let (_, descriptor_pass) = measure(|| effect_descriptor_identity_v1(descriptor, 4_194_304));
    assert!(descriptor_pass.allocations > 0);
    assert_eq!(descriptor_pass.live_bytes, 0);

    let verified = verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
    let artifacts: Vec<_> = verified
        .artifacts()
        .map(|artifact| EffectArtifactAuthoringV1 {
            kind: artifact.kind(),
            path: artifact.path(),
            target: artifact.target(),
            features: artifact.features(),
            content: artifact.content(),
        })
        .collect();
    let authoring = EffectPackageAuthoringV1 {
        descriptor,
        artifacts: &artifacts,
    };
    let mut output = vec![0; bytes.len()];

    let (required, required_snapshot) =
        measure(|| effect_package_v1_required_size(&authoring, EffectPackageLimitsV1::default()));
    assert_eq!(required.unwrap(), bytes.len() as u64);
    assert_eq!(required_snapshot, descriptor_pass);

    let (encoded, encode_snapshot) = measure(|| {
        encode_effect_package_v1(&authoring, EffectPackageLimitsV1::default(), &mut output)
    });
    assert_eq!(encoded.unwrap(), bytes.len());
    assert_eq!(encode_snapshot, descriptor_pass);

    let (verified, verify_snapshot) =
        measure(|| verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()));
    let verified = verified.unwrap();
    assert_eq!(verify_snapshot, descriptor_pass);

    let (cid, cid_snapshot) =
        measure(|| effect_package_cid_v1(&bytes, EffectPackageLimitsV1::default()));
    cid.unwrap();
    assert_eq!(cid_snapshot, descriptor_pass);

    let (selected, select_snapshot) = measure(|| {
        select_effect_package_artifact_v1(
            &verified,
            ArtifactSelectionRequestV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                target: "wasm32-unknown-unknown",
                capabilities: &["bulk-memory", "simd128"],
            },
        )
    });
    assert_eq!(selected.unwrap().path(), "wasm/bulk.wasm");
    assert_eq!(
        select_snapshot,
        Snapshot {
            allocations: 0,
            deallocations: 0,
            live_bytes: 0,
        }
    );
}
