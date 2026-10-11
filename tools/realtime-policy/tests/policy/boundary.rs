//! Where a region marker may sit (A2-D1).

use realtime_policy::Class;

use super::common::{
    BASE_FILES, BASE_REGIONS, BEGIN, END, Edit, expect_failure, expect_valid, region,
};

const BUFFER: &str = "crates/engine/src/realtime/buffer.rs";
const MACRO_TREE: Class = Class::RegionSplitsMacroTree;

fn append(text: &str) -> Vec<Edit<'_>> {
    vec![Edit::Append { path: BUFFER, text }]
}

/// Assert that the base tree plus `text` passes and has one more region.
fn expect_one_more_region(name: &str, text: &str) {
    let report = expect_valid(name, &append(text));
    assert_eq!(
        (report.regions, report.files),
        (BASE_REGIONS + 1, BASE_FILES),
        "case {name}"
    );
}

/// Each row puts both markers inside one node of the named kind, where no child boundary is.
/// The kinds are the outermost statement or item that holds the marker.
#[test]
fn a_marker_inside_a_node_is_refused() {
    let rows: [(&str, String, &'static str); 11] = [
        (
            "boundary-in-match-arms",
            format!(
                "fn probe(x: u8) {{\n    match x {{\n        0 => {{}}\n        // {BEGIN}\n        _ => {{}}\n        // {END}\n    }}\n}}\n"
            ),
            "a match expression",
        ),
        (
            "boundary-in-call-arguments",
            format!(
                "fn probe() {{\n    call(1,\n    // {BEGIN}\n    2,\n    // {END}\n    3);\n}}\n"
            ),
            "a call expression",
        ),
        (
            "boundary-in-struct-fields",
            format!("struct Fields {{\n    a: u8,\n    // {BEGIN}\n    b: u8,\n    // {END}\n}}\n"),
            "a struct",
        ),
        (
            "boundary-in-enum-variants",
            format!("enum Variants {{\n    A,\n    // {BEGIN}\n    B,\n    // {END}\n}}\n"),
            "an enum",
        ),
        (
            "boundary-in-parameters",
            format!("fn probe(\n    a: u8,\n    // {BEGIN}\n    b: u8,\n    // {END}\n) {{}}\n"),
            "a function",
        ),
        (
            "boundary-in-let-initializer",
            format!("fn probe() {{\n    let x =\n    // {BEGIN}\n    1;\n    // {END}\n}}\n"),
            "a let statement",
        ),
        (
            "boundary-before-else",
            format!(
                "fn probe(x: bool) {{\n    if x {{\n    }}\n    // {BEGIN}\n    else {{\n    }}\n    // {END}\n}}\n"
            ),
            "an if expression",
        ),
        (
            "boundary-in-method-chain",
            format!(
                "fn probe() {{\n    value\n        // {BEGIN}\n        .method()\n        // {END}\n        .other();\n}}\n"
            ),
            "a method call",
        ),
        (
            "boundary-in-impl-header",
            format!("impl\n// {BEGIN}\nStruct {{}}\n// {END}\n"),
            "an impl",
        ),
        (
            "boundary-in-struct-expression",
            format!(
                "fn probe() {{\n    let _ = Point {{\n        x: 1,\n        // {BEGIN}\n        y: 2,\n        // {END}\n    }};\n}}\n"
            ),
            "a let statement",
        ),
        (
            "boundary-in-closure-parameters",
            format!(
                "fn probe() {{\n    run(|a,\n    // {BEGIN}\n    b| a + b,\n    // {END}\n    1);\n}}\n"
            ),
            "a call expression",
        ),
    ];
    for (name, text, node) in rows {
        expect_failure(name, &append(&text), &[Class::RegionSplits(node)]);
    }
}

/// Each marker is judged alone: a region with one good marker and one bad is refused.
#[test]
fn either_marker_alone_can_split_a_node() {
    let call = Class::RegionSplits("a call expression");
    let end_splits = format!(
        "// {BEGIN}\nfn inside() {{}}\nfn holder() {{\n    call(1,\n    // {END}\n    2);\n}}\n"
    );
    let report = expect_failure(
        "boundary-only-end-splits",
        &append(&end_splits),
        &[call, Class::RegionEndsInsideAContainer],
    );
    // B1b-D9: the end marker is inside `holder`, a block that does not enclose the begin marker's
    // file, so the same marker is also refused for where it ends. Both findings are on its line,
    // the base file's three lines and four more below.
    assert_eq!(report.findings.len(), 2);
    assert!(
        report
            .findings
            .iter()
            .all(|finding| finding.line == Some(8))
    );
    let begin_splits = format!(
        "fn holder() {{\n    call(1,\n    // {BEGIN}\n    2);\n}}\nfn inside() {{}}\n// {END}\n"
    );
    expect_failure(
        "boundary-only-begin-splits",
        &append(&begin_splits),
        &[call],
    );
}

/// A marker inside an attribute splits it, although the attribute comes before its item.
#[test]
fn a_marker_inside_an_attribute_is_refused() {
    let text = format!(
        "#[derive(\n    Clone,\n    // {BEGIN}\n    Copy,\n    // {END}\n)]\nstruct Attributed;\n"
    );
    expect_failure(
        "boundary-in-attribute",
        &append(&text),
        &[Class::RegionSplits("an attribute")],
    );
}

/// An item starts after all its outer attributes, so a marker between two of them, or between
/// the last and the item, is between children. The second shape is the real tree's, in `rack`.
#[test]
fn a_marker_between_attributes_and_their_item_is_a_boundary() {
    let documented = format!("/// A doc comment.\n// {BEGIN}\nfn documented() {{}}\n// {END}\n");
    expect_one_more_region("boundary-after-doc-comment", &documented);
    let between = format!("#[inline]\n// {BEGIN}\n#[must_use]\nfn between() {{}}\n// {END}\n");
    expect_one_more_region("boundary-between-attributes", &between);
    let several = format!(
        "/// A doc comment.\n///\n/// Its second paragraph.\n#[inline]\n#[must_use]\n// {BEGIN}\nfn several() {{}}\n// {END}\n"
    );
    expect_one_more_region("boundary-after-several-attributes", &several);
    let in_impl = format!(
        "impl Marked {{\n    /// Documented.\n    // {BEGIN}\n    fn documented(&self) {{}}\n    // {END}\n}}\n"
    );
    expect_one_more_region("boundary-after-doc-comment-in-impl", &in_impl);
}

/// Every container kind takes markers between its children, and a region may sit in one.
#[test]
fn a_marker_between_the_children_of_a_container_is_a_boundary() {
    // Each row: the lines that open the container, the child, the lines that close it.
    let rows = [
        ("boundary-in-mod", "mod inner {", "fn child() {}", "}"),
        ("boundary-in-impl", "impl Inner {", "fn child() {}", "}"),
        ("boundary-in-trait", "trait Inner {", "fn child();", "}"),
        (
            "boundary-in-extern-block",
            "extern \"C\" {",
            "fn child();",
            "}",
        ),
        ("boundary-in-fn-block", "fn probe() {", "work();", "}"),
        (
            "boundary-in-nested-block",
            "fn probe() {\n    {",
            "work();",
            "    }\n}",
        ),
        (
            "boundary-in-loop-block",
            "fn probe() {\n    loop {",
            "work();",
            "    }\n}",
        ),
        (
            "boundary-in-match-arm-block",
            "fn probe(x: u8) {\n    match x {\n        _ => {",
            "work();",
            "        }\n    }\n}",
        ),
    ];
    for (name, open, child, close) in rows {
        let text = format!("{open}\n{}{close}\n", region(child));
        expect_one_more_region(name, &text);
    }
}

/// A region may start in one container and end in another when the end's container is the
/// begin's or encloses it (B1b-D9). The first shape is the real `live.rs` and `transient-shaper`.
#[test]
fn a_region_may_span_the_end_of_an_impl() {
    let text = format!(
        "impl Spanning {{\n    // {BEGIN}\n    fn first(&self) {{}}\n}}\nfn second() {{}}\n// {END}\n"
    );
    expect_one_more_region("boundary-spans-impl-end", &text);
    let enclosing = format!(
        "fn one() {{\n    {{\n    // {BEGIN}\n    work();\n    }}\n    work();\n    // {END}\n}}\n"
    );
    expect_one_more_region("boundary-ends-in-the-enclosing-block", &enclosing);
}

/// Assert that the base tree plus `text` is refused for where its region ends, and for nothing
/// else (B1b-D9), with one finding on the end marker's line.
fn expect_end_refused(name: &str, text: &str) {
    let report = expect_failure(name, &append(text), &[Class::RegionEndsInsideAContainer]);
    // The base file's three lines come first.
    let end_line = 3
        + text
            .lines()
            .position(|line| line.contains(END))
            .expect("the case holds an end marker")
        + 1;
    let [finding] = report.findings.as_slice() else {
        panic!("case {name}: one finding expected: {:#?}", report.findings);
    };
    assert_eq!(finding.line, Some(end_line), "case {name}");
}

/// A region whose end marker sits in a container that does not enclose the begin marker's is
/// refused whatever it holds (B1b-D9). Red if it passes: the region ends inside a block that
/// continues past the marker, so the item scan, which reads the region only, could not see an item
/// after it.
#[test]
fn a_region_that_ends_in_another_block_is_refused() {
    let text = format!(
        "fn one() {{\n    // {BEGIN}\n    work();\n}}\nfn two() {{\n    work();\n    // {END}\n}}\n"
    );
    expect_end_refused("boundary-spans-two-blocks", &text);
}

/// The first verdict's probe: a `const` after the end marker, in the block that holds the loop,
/// shadows the count for the whole block. Nothing but the boundary rule can see it.
#[test]
fn a_region_that_ends_in_a_deeper_block_is_refused() {
    let text = format!(
        "fn probe(q: &mut Consumer) {{\n    // {BEGIN}\n    let available = q.available_at_entry();\n    {{\n        for _ in 0..available {{ let _ = q.try_pop(); }}\n    // {END}\n        const available: usize = usize::MAX;\n    }}\n}}\n"
    );
    expect_end_refused("region-ends-in-a-deeper-block", &text);
}

/// The shape of the real `gate-expander`: a region inside a `macro_rules!` body.
#[test]
fn a_region_inside_a_macro_body_is_tokens_and_passes() {
    let text = format!(
        "macro_rules! bank_like {{\n    ($lane:ty) => {{\n        fn before() {{}}\n        // {BEGIN}\n        fn inside() {{}}\n        // {END}\n        fn after() {{}}\n    }};\n}}\n"
    );
    expect_one_more_region("boundary-in-macro-body", &text);
    // The same holds in a macro call's arguments and between the tokens of a nested group.
    let call =
        format!("fn probe() {{\n    emit!(a,\n    // {BEGIN}\n    b,\n    // {END}\n    c);\n}}\n");
    expect_one_more_region("boundary-in-macro-call-arguments", &call);
    let nested = format!(
        "fn probe() {{\n    emit!(a, {{ b,\n    // {BEGIN}\n    c,\n    // {END}\n    d }});\n}}\n"
    );
    expect_one_more_region("boundary-in-nested-macro-group", &nested);
    let deeper = format!(
        "fn probe() {{\n    emit!(a, {{ b, [ c, ( d,\n    // {BEGIN}\n    e,\n    // {END}\n    f ) ] }});\n}}\n"
    );
    expect_one_more_region("boundary-in-deeply-nested-macro-group", &deeper);
}

/// A region whose markers sit in different containers, one a macro token tree, mixes tokens and
/// syntax.
#[test]
fn a_region_that_crosses_a_macro_token_tree_is_refused() {
    let rows = [
        (
            "boundary-begin-in-macro-end-after",
            6,
            format!(
                "macro_rules! crossing {{\n    () => {{\n        // {BEGIN}\n        fn inside() {{}}\n    }};\n}}\n// {END}\n"
            ),
        ),
        (
            "boundary-begin-before-macro-end-inside",
            4,
            format!(
                "// {BEGIN}\nmacro_rules! crossing {{\n    () => {{\n        fn inside() {{}}\n        // {END}\n    }};\n}}\n"
            ),
        ),
        (
            "boundary-begin-in-outer-group-end-in-inner",
            6,
            format!(
                "fn probe() {{\n    emit!(a,\n    // {BEGIN}\n    b, {{ c,\n    // {END}\n    d }});\n}}\n"
            ),
        ),
        (
            "boundary-begin-in-group-end-two-groups-deeper",
            6,
            format!(
                "fn probe() {{\n    emit!(a, {{ b,\n    // {BEGIN}\n    c, [ d, ( e,\n    // {END}\n    f ) ] }});\n}}\n"
            ),
        ),
        (
            "boundary-between-two-macro-calls",
            6,
            format!(
                "fn probe() {{\n    first!(\n    // {BEGIN}\n    a);\n    second!(\n    b,\n    // {END}\n    c);\n}}\n"
            ),
        ),
        (
            "boundary-block-to-macro-group",
            5,
            format!(
                "fn probe() {{\n    // {BEGIN}\n    work();\n    emit!(\n    a,\n    // {END}\n    b);\n}}\n"
            ),
        ),
    ];
    for (name, begin_line, text) in rows {
        let report = expect_failure(name, &append(&text), &[MACRO_TREE]);
        // One finding, on the begin marker's line.
        assert_eq!(report.findings.len(), 1, "case {name}");
        assert_eq!(report.findings[0].line, Some(begin_line), "case {name}");
    }
}

/// A finding names the marker's line and text.
#[test]
fn a_boundary_finding_names_the_marker_line() {
    let text = format!("fn probe() {{\n    call(1,\n    // {BEGIN}\n    2);\n}}\n// {END}\n");
    let report = expect_failure(
        "boundary-finding-place",
        &append(&text),
        &[Class::RegionSplits("a call expression")],
    );
    let [finding] = report.findings.as_slice() else {
        panic!("one finding expected: {:#?}", report.findings);
    };
    assert_eq!(finding.path.as_deref(), Some(BUFFER));
    // The base file's three lines, then the appended text's third line.
    assert_eq!(finding.line, Some(6));
    assert_eq!(
        finding.source.as_deref(),
        Some(format!("// {BEGIN}").as_str())
    );
}
