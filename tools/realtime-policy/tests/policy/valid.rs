//! Trees that pass.

use super::common::{BASE_FILES, BASE_REGIONS, Tree, assert_classes, expect_valid};

#[test]
fn the_base_tree_passes_with_its_counts() {
    let report = expect_valid("valid", &[]);
    assert_eq!((report.regions, report.files), (BASE_REGIONS, BASE_FILES));
}

/// A region whose body is empty is still a region.
#[test]
fn regions_with_empty_bodies_count() {
    let tree = Tree::base("empty-bodies");
    let emptied = [
        "crates/engine/src/realtime/buffer.rs",
        "crates/engine/src/realtime/disjoint.rs",
        "crates/engine/src/realtime/observe.rs",
        "crates/engine/src/realtime/plan.rs",
        "crates/graph/src/lib.rs",
        "crates/graph/src/runtime.rs",
        "crates/effect-contract/src/live.rs",
        "crates/rack/src/lib.rs",
        "crates/builtins/src/lib.rs",
        "crates/builtins-compiler/src/lib.rs",
        "hosts/host-web/src/lib.rs",
    ];
    for path in emptied {
        let kept: Vec<String> = tree
            .read(path)
            .lines()
            .filter(|line| !is_empty_function(line))
            .map(str::to_owned)
            .collect();
        tree.write(path, format!("{}\n", kept.join("\n")).as_bytes());
    }
    let report = assert_classes(&tree, &[]);
    assert_eq!((report.regions, report.files), (BASE_REGIONS, BASE_FILES));
}

/// A line of the form `fn name() {}` with an optional indent.
fn is_empty_function(line: &str) -> bool {
    line.trim()
        .strip_prefix("fn ")
        .and_then(|rest| rest.strip_suffix("() {}"))
        .is_some_and(|name| {
            !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        })
}
