//! Region markers and floors (D6, D8).

use realtime_policy::Class;

use super::common::{
    BASE_FILES, BASE_REGIONS, BEGIN, END, Edit, FILE_FLOOR, REGION_FLOOR, Tree, expect_failure,
    expect_valid,
};

const MALFORMED: Class = Class::MalformedMarker;
const BUFFER: &str = "crates/engine/src/realtime/buffer.rs";

fn append(text: &str) -> Vec<Edit<'_>> {
    vec![Edit::Append { path: BUFFER, text }]
}

/// Replace a whole file: delete it, then create it with `text`.
fn rewrite<'a>(path: &'a str, text: &'a str) -> [Edit<'a>; 2] {
    [Edit::Delete { path }, Edit::Create { path, text }]
}

#[test]
fn a_marker_written_in_a_string_literal_is_malformed() {
    // One line: the marker word inside a literal.
    let one_line = format!("const S: &str = \"// {BEGIN}\";\n");
    expect_failure("marker-in-string", &append(&one_line), &[MALFORMED]);
    // Across lines: the middle line is a valid-looking marker line, and only the literal that
    // spans it says otherwise.
    let spanning = format!("const S: &str = \"\n// {BEGIN}\n\";\n");
    expect_failure("marker-line-in-string", &append(&spanning), &[MALFORMED]);
    let raw = format!("const S: &str = r#\"\n// {END}\n\"#;\n");
    expect_failure("marker-line-in-raw-string", &append(&raw), &[MALFORMED]);
}

#[test]
fn a_marker_written_in_a_doc_comment_is_malformed() {
    let line_doc = format!("/// {BEGIN}\nfn documented() {{}}\n");
    expect_failure("marker-in-doc-comment", &append(&line_doc), &[MALFORMED]);
    // A block doc comment is one literal token to the lexer: its middle line is valid-looking.
    let block_doc = format!("/**\n// {BEGIN}\n*/\nfn documented() {{}}\n");
    expect_failure(
        "marker-line-in-block-doc-comment",
        &append(&block_doc),
        &[MALFORMED],
    );
}

#[test]
fn a_marker_line_inside_a_block_comment_is_malformed() {
    let block = format!("/*\n// {BEGIN}\n*/\n");
    expect_failure("marker-in-block-comment", &append(&block), &[MALFORMED]);
    // Block comments nest: the marker is still inside the outer one after an inner one closes.
    let nested = format!("/* outer /* inner */\n// {END}\n*/\n");
    expect_failure(
        "marker-in-nested-block-comment",
        &append(&nested),
        &[MALFORMED],
    );
}

/// A line comment that mentions `/*` opens nothing; the marker after it is a real marker.
#[test]
fn a_line_comment_opening_a_block_comment_text_does_not_open_one() {
    let text = format!(
        "// looks like the start of /* a block comment\n// {BEGIN}\nfn inside() {{}}\n// {END}\n"
    );
    let report = expect_valid("line-comment-with-block-opener", &append(&text));
    assert_eq!(
        (report.regions, report.files),
        (BASE_REGIONS + 1, BASE_FILES)
    );
}

#[test]
fn a_marker_after_code_is_malformed() {
    let text = format!("fn after() {{}} // {END}\n");
    expect_failure("marker-after-code", &append(&text), &[MALFORMED]);
}

#[test]
fn a_longer_word_or_a_bare_colon_is_malformed() {
    let longer = format!("// {BEGIN}NING\n");
    expect_failure("marker-longer-word", &append(&longer), &[MALFORMED]);
    let colon = format!("// {BEGIN}:\n");
    expect_failure("marker-bare-colon", &append(&colon), &[MALFORMED]);
}

/// Markers that sit inside an `impl`, and markers that carry a reason, are markers.
#[test]
fn indented_markers_and_markers_with_a_reason_are_regions() {
    let text = format!(
        "impl Indented {{\n    // {BEGIN}: the reason\n    fn inside() {{}}\n    // {END}\n}}\n"
    );
    let report = expect_valid("marker-in-impl-with-reason", &append(&text));
    assert_eq!(
        (report.regions, report.files),
        (BASE_REGIONS + 1, BASE_FILES)
    );
}

#[test]
fn an_end_before_its_begin_is_misordered() {
    let swapped = format!("// {END}\nfn render() {{}}\n// {BEGIN}\n");
    let edits = rewrite(BUFFER, &swapped);
    // The file leaves both counts: a misordered file has no regions.
    expect_failure(
        "markers-end-before-begin",
        &edits,
        &[Class::MisorderedMarkers, FILE_FLOOR, REGION_FLOOR],
    );
}

#[test]
fn two_begins_in_a_row_are_misordered() {
    let doubled = format!("// {BEGIN}\nfn a() {{}}\n// {BEGIN}\nfn b() {{}}\n// {END}\n// {END}\n");
    expect_failure(
        "markers-nested",
        &rewrite(BUFFER, &doubled),
        &[Class::MisorderedMarkers, FILE_FLOOR, REGION_FLOOR],
    );
}

/// Today's open-tail allowance is gone: out-of-order markers at the end of a file make the file
/// unmarked, and an unmarked drain above them is not part of any region.
#[test]
fn an_open_tail_is_misordered() {
    let tree = Tree::base("open-tail");
    let pad = "crates/floor/src/pad_1.rs";
    tree.apply(&[Edit::Append {
        path: pad,
        text: &format!("// {END}\n// {BEGIN}\nfn render_tail() {{}}\n"),
    }]);
    let report =
        super::common::assert_classes(&tree, &[Class::MisorderedMarkers, FILE_FLOOR, REGION_FLOOR]);
    assert_eq!(
        (report.regions, report.files),
        (BASE_REGIONS - 14, BASE_FILES - 1)
    );
}

#[test]
fn unmatched_markers_name_the_file_and_drop_it_from_the_floors() {
    let report = expect_failure(
        "unmatched-markers-outside-root",
        &[Edit::ReplaceLine {
            path: "hosts/host-web/src/lib.rs",
            old: &format!("// {END}"),
            new: "",
        }],
        &[Class::UnmatchedMarkers, FILE_FLOOR, REGION_FLOOR],
    );
    let unmatched = report
        .findings
        .iter()
        .find(|finding| finding.class == Class::UnmatchedMarkers)
        .expect("an unmatched finding");
    assert_eq!(unmatched.path.as_deref(), Some("hosts/host-web/src/lib.rs"));
}

#[test]
fn the_file_floor_fires_when_a_marked_file_loses_its_markers() {
    // Removing every marker of one file drops the file and its regions.
    expect_failure(
        "marked-file-count-floor",
        &rewrite("crates/builtins/src/lib.rs", "fn builtins() {}\n"),
        &[FILE_FLOOR, REGION_FLOOR],
    );
}

#[test]
fn the_floors_fire_when_no_file_is_marked() {
    let tree = Tree::base("no-marked-files");
    for path in marked_paths(&tree) {
        let kept: Vec<String> = tree
            .read(&path)
            .lines()
            .filter(|line| !line.contains(BEGIN) && !line.contains(END))
            .map(str::to_owned)
            .collect();
        tree.write(&path, format!("{}\n", kept.join("\n")).as_bytes());
    }
    let report = super::common::assert_classes(&tree, &[FILE_FLOOR, REGION_FLOOR]);
    assert_eq!((report.regions, report.files), (0, 0));
}

fn marked_paths(tree: &Tree) -> Vec<String> {
    let mut found = Vec::new();
    collect_marked(tree.root(), tree.root(), &mut found);
    found.sort();
    found
}

fn collect_marked(root: &std::path::Path, directory: &std::path::Path, found: &mut Vec<String>) {
    for entry in std::fs::read_dir(directory).expect("list a tree directory") {
        let path = entry.expect("read a tree entry").path();
        if path.is_dir() {
            collect_marked(root, &path, found);
        } else if std::fs::read_to_string(&path)
            .expect("read a tree file")
            .contains(BEGIN)
        {
            let relative = path.strip_prefix(root).expect("under the root");
            found.push(relative.to_string_lossy().into_owned());
        }
    }
}

#[test]
fn the_region_floor_fires_when_a_region_is_dropped_and_the_markers_still_pair() {
    let tree = Tree::base("marked-region-count-floor");
    let path = "crates/rack/src/lib.rs";
    let text = tree.read(path);
    let first_end = text.find(END).expect("a first region end");
    let after = text[first_end..]
        .find('\n')
        .expect("a newline after the end marker")
        + first_end
        + 1;
    tree.apply(&rewrite(path, &text[after..]));
    super::common::assert_classes(&tree, &[REGION_FLOOR]);
}
