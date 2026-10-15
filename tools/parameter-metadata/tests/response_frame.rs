//! Issue #1623: the `RESPONSE_GET` frame table the SDK generates its codec from is the protocol
//! crate's own.

use std::path::Path;

use parameter_metadata::response_frame;

/// `sdk/assets/miso-engine-v1-response-frame.json` is what `--print-response-frame` writes. The SDK
/// checks of `check-sdk-generated.sh` compare the generated TypeScript with this asset; this holds
/// the asset to the protocol schema even where the check runs against an engine artifact directory,
/// which does not carry it.
#[test]
fn response_frame_asset_is_current() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../sdk/assets")
        .join(response_frame::OUTPUT_NAME);
    let checked = std::fs::read_to_string(&path).expect("read the checked response frame table");
    assert_eq!(
        checked,
        response_frame::render(),
        "{} is stale; run `node codegen/assets.mjs` in sdk/",
        path.display()
    );
}
