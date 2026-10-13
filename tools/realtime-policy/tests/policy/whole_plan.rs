//! The whole-plan rule (A2-D3).

use realtime_policy::Class;

use super::common::{Edit, UNSAFE, expect_failure, expect_valid};

const FFI: &str = "crates/capi/src/ffi.rs";
const WHOLE_PLAN: Class = Class::WholePlanReference;

/// The render entry point's signature line in the base file.
fn render_entry() -> String {
    format!(
        "pub {UNSAFE} extern \"C\" fn miso_engine_v1_render_f32_planar(plan: *mut Plan) -> u32 {{"
    )
}

/// The last line of the render entry point in the base file.
const RETURN: &str = "    0";

/// `text` in the render entry point, before its return.
fn in_render_entry(text: &str) -> Edit<'_> {
    Edit::ReplaceLine {
        path: FFI,
        old: RETURN,
        new: text,
    }
}

fn append(text: &str) -> Edit<'_> {
    Edit::Append { path: FFI, text }
}

/// The script's five forms, as the tree reads them.
#[test]
fn every_form_of_a_whole_plan_borrow_is_a_finding() {
    for (index, form) in [
        "&*plan",
        "&mut *plan",
        "&(*plan)",
        "&mut (*plan)",
        "&mut  *plan",
    ]
    .iter()
    .enumerate()
    {
        let line = format!("    let _ = {UNSAFE} {{ {form} }};\n    0");
        expect_failure(
            &format!("whole-plan-form-{index}"),
            &[in_render_entry(&line)],
            &[WHOLE_PLAN],
        );
    }
}

/// A line break or a comment line above is not an exemption, and the finding names the line of
/// the `&`.
#[test]
fn layout_does_not_hide_a_whole_plan_borrow() {
    let at_line_end = format!("    let whole = {UNSAFE} {{\n        &*plan\n    }};\n    0");
    let report = expect_failure(
        "whole-plan-at-line-end",
        &[in_render_entry(&at_line_end)],
        &[WHOLE_PLAN],
    );
    let [finding] = report.findings.as_slice() else {
        panic!("one finding expected: {:#?}", report.findings);
    };
    assert_eq!(finding.path.as_deref(), Some(FFI));
    // The base file's return is on line 8; the `&*plan` is the second line that replaces it.
    assert_eq!(finding.line, Some(9));
    assert_eq!(finding.source.as_deref(), Some("&*plan"));
    let after_comment = format!(
        "    // a comment line above is not an exemption\n    let _ = {UNSAFE} {{ &*plan }};\n    0"
    );
    expect_failure(
        "whole-plan-after-a-comment-line",
        &[in_render_entry(&after_comment)],
        &[WHOLE_PLAN],
    );
}

/// The file must exist, and its render entry point must be in the scanned items.
#[test]
fn a_missing_file_or_a_render_entry_only_in_tests_is_a_finding() {
    expect_failure(
        "capi-ffi-missing",
        &[Edit::Delete { path: FFI }],
        &[Class::MissingCapiFfi],
    );
    let entry = render_entry();
    let fixture = format!("    fn fixture(plan: *mut Plan) {{ let _ = {UNSAFE} {{ &*plan }}; }}");
    expect_failure(
        "capi-render-entry-only-in-tests",
        &[
            Edit::ReplaceLine {
                path: FFI,
                old: &entry,
                new: "fn moved_render(plan: *mut Plan) -> u32 {",
            },
            Edit::ReplaceLine {
                path: FFI,
                old: &fixture,
                new: "    fn miso_engine_v1_render_f32_planar() {}",
            },
        ],
        &[Class::NoRenderEntry],
    );
}

/// The test module no longer ends the scan: items after it are production code.
#[test]
fn a_whole_plan_borrow_after_the_test_module_is_a_finding() {
    let after = format!("fn after_tests(plan: *mut Plan) {{ let _ = {UNSAFE} {{ &*plan }}; }}\n");
    expect_failure("whole-plan-after-tests", &[append(&after)], &[WHOLE_PLAN]);
}

/// Only a module named `tests` under exactly `#[cfg(test)]` is test code.
#[test]
fn only_a_cfg_test_module_named_tests_is_skipped() {
    let any = Edit::ReplaceLine {
        path: FFI,
        old: "#[cfg(test)]",
        new: "#[cfg(any(test, feature = \"x\"))]",
    };
    expect_failure("whole-plan-in-cfg-any-test", &[any], &[WHOLE_PLAN]);
    let other_name = Edit::ReplaceLine {
        path: FFI,
        old: "mod tests {",
        new: "mod helpers {",
    };
    expect_failure(
        "whole-plan-in-cfg-test-helpers",
        &[other_name],
        &[WHOLE_PLAN],
    );
}

/// Another outer attribute beside `#[cfg(test)]`, before or after it, leaves the module test
/// code. The older awk gate stops only at a `#[cfg(test)]` line that comes directly before
/// `mod tests {`, so it scans such a module: a deliberate difference.
#[test]
fn another_attribute_beside_cfg_test_still_skips_the_module() {
    let after = Edit::ReplaceLine {
        path: FFI,
        old: "#[cfg(test)]",
        new: "#[cfg(test)]\n#[allow(dead_code)]",
    };
    expect_valid("whole-plan-cfg-test-then-allow", &[after]);
    let before = Edit::ReplaceLine {
        path: FFI,
        old: "#[cfg(test)]",
        new: "#[allow(dead_code)]\n#[cfg(test)]",
    };
    expect_valid("whole-plan-allow-then-cfg-test", &[before]);
}

/// A macro's tokens are not parsed, so the rule reads them as tokens.
#[test]
fn a_whole_plan_borrow_inside_a_macro_is_a_finding() {
    let rows = [
        ("whole-plan-in-macro-call", "    check!(&*plan);\n    0"),
        ("whole-plan-in-macro-mut", "    check!(&mut *plan);\n    0"),
        ("whole-plan-in-macro-parens", "    check!(&(*plan));\n    0"),
        (
            "whole-plan-in-macro-braces",
            "    check! { first, &*plan }\n    0",
        ),
        (
            "whole-plan-in-nested-group",
            "    check!([first, (&*plan)]);\n    0",
        ),
        // A single colon does not continue the operand as a path would.
        (
            "whole-plan-before-a-colon",
            "    check!(&*plan : Plan);\n    0",
        ),
    ];
    for (name, text) in rows {
        expect_failure(name, &[in_render_entry(text)], &[WHOLE_PLAN]);
    }
    let definition =
        "macro_rules! borrow_plan {\n    ($plan:expr) => {\n        &*plan\n    };\n}\n";
    expect_failure(
        "whole-plan-in-macro-rules",
        &[append(definition)],
        &[WHOLE_PLAN],
    );
}

/// Another name, a projection after the dereference, a call, an index and a path are not a
/// reference to the whole plan, in the tree and in a macro's tokens.
#[test]
fn borrows_that_are_not_the_whole_plan_pass() {
    let tree = format!(
        "    let _ = {UNSAFE} {{ &*planner }};\n    let _ = {UNSAFE} {{ &*plan_state(plan) }};\n    let _ = {UNSAFE} {{ &(*plan).queries }};\n    let _ = {UNSAFE} {{ &mut (*plan).queries }};\n    let _ = {UNSAFE} {{ &*plan.cast::<HandleHeader>() }};\n    let _ = {UNSAFE} {{ &*plan[0] }};\n    let _ = {UNSAFE} {{ &*plan::DEFAULT }};\n    let _ = {UNSAFE} {{ &raw const *plan }};\n    let _ = {UNSAFE} {{ &raw mut *plan }};\n    0"
    );
    expect_valid("whole-plan-tree-lookalikes", &[in_render_entry(&tree)]);
    let rows = [
        ("whole-plan-macro-planner", "    check!(&*planner);\n    0"),
        ("whole-plan-macro-call", "    check!(&*plan(1));\n    0"),
        ("whole-plan-macro-index", "    check!(&*plan[0]);\n    0"),
        (
            "whole-plan-macro-path",
            "    check!(&*plan::DEFAULT);\n    0",
        ),
        (
            "whole-plan-macro-method",
            "    check!(&*plan.cast::<HandleHeader>());\n    0",
        ),
        (
            "whole-plan-macro-field",
            "    check!(&(*plan).queries);\n    0",
        ),
        (
            "whole-plan-macro-raw",
            "    check!(&raw const *plan);\n    0",
        ),
        (
            "whole-plan-macro-double-deref",
            "    check!(&**plan);\n    0",
        ),
    ];
    for (name, text) in rows {
        expect_valid(name, &[in_render_entry(text)]);
    }
}
