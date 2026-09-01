//! External compile coverage for the public controller response frame.

use protocol::ControllerResponse;

#[test]
fn complete_canonical_response_frame_is_publicly_accessible() {
    fn frame_bytes(response: &ControllerResponse) -> &[u8] {
        &response.frame
    }

    let accessor: for<'a> fn(&'a ControllerResponse) -> &'a [u8] = frame_bytes;
    let _ = accessor;
}
