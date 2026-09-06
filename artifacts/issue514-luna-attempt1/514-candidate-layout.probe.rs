
#[test]
fn issue514_native_layout_probe() {
    println!("ReadyOwnership size={} align={}", size_of::<ReadyOwnership>(), core::mem::align_of::<ReadyOwnership>());
    println!("OptionReadyOwnership size={} align={}", size_of::<Option<ReadyOwnership>>(), core::mem::align_of::<Option<ReadyOwnership>>());
    println!("AudioWorkletEngineHost size={} align={}", size_of::<AudioWorkletEngineHost>(), core::mem::align_of::<AudioWorkletEngineHost>());
    let host = prepared_host(128);
    let r = host.resources();
    println!("bridge_metadata_bytes={} bridge_retained_bytes={} largest_bridge_allocation_bytes={} largest_named_allocation_bytes={}", r.bridge_metadata_bytes, r.bridge_retained_bytes, r.largest_bridge_allocation_bytes, r.largest_named_allocation_bytes);
}
