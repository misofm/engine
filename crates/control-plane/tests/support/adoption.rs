//! Render until a committed revision is applied (#1324a): a rebuild that restarts a strip is a
//! duck-swap, adopted no earlier than its scheduled sample `S`, not at the next block. Shared by
//! the control plane's test binaries.

/// The most blocks a duck-swap's adoption may take in these tests: `S` is at most
/// `p + 2q + N + C`, far inside 64 blocks of 128 frames at any session `mute_ms` they use.
pub(crate) const MAX_BLOCKS: u64 = 64;

/// Renders `rig` with `render` until `applied(rig)` reaches `revision`, at most [`MAX_BLOCKS`]
/// blocks; the number of blocks rendered. Renders nothing when it is already applied.
///
/// # Panics
///
/// When the revision is not applied within [`MAX_BLOCKS`] blocks.
pub(crate) fn render_until_applied<R>(
    rig: &mut R,
    revision: u64,
    applied: impl Fn(&R) -> u64,
    render: impl Fn(&mut R),
) -> u64 {
    let mut blocks = 0;
    while applied(rig) < revision {
        assert!(
            blocks < MAX_BLOCKS,
            "revision {revision} is not applied after {MAX_BLOCKS} blocks"
        );
        render(rig);
        blocks += 1;
    }
    blocks
}
