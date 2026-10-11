//! The count, loop and pop shapes that the rule accepts or refuses, and how a pop maps to its
//! region (D1), the scope that resolves a count (D2), the syntax it visits (D4) and what counts as
//! a pop (D5).

use realtime_policy::Class;

use super::super::common::{
    BASE_FILES, BASE_REGIONS, BEGIN, END, Edit, expect_failure, expect_valid,
};
use super::{ANCHOR, BUILTINS_COMPILER, DRAIN, accepts, refuses};

const BUFFER: &str = "crates/engine/src/realtime/buffer.rs";

/// The pop-and-apply body that every drain below shares.
const POP: &str = "let Ok(record) = control.try_pop() else {\n            break;\n        };\n        apply(record);";

/// A drain function whose body is `body`.
fn drain(body: &str) -> String {
    format!("fn drain(control: &mut Consumer<Record>) {{\n    {body}\n}}")
}

/// A counted `for` loop over `available`, with the entry binding `binding`.
fn counted(binding: &str) -> String {
    drain(&format!(
        "{binding}\n    for _ in 0..available {{\n        {POP}\n    }}"
    ))
}

#[test]
fn the_forms_that_d6_and_d8_state_pass() {
    let rows: [(&str, String); 11] = [
        (
            "form-header-entry-count",
            drain(&format!(
                "for _ in 0..control.available_at_entry() {{\n        {POP}\n    }}"
            )),
        ),
        (
            "form-while-greater-than-zero",
            drain(&format!(
                "let mut remaining = control.available_at_entry();\n    while remaining > 0 {{\n        remaining -= 1;\n        {POP}\n    }}"
            )),
        ),
        (
            "form-typed-count-capped-by-a-field",
            format!(
                "impl Drain {{\n    fn run(&mut self, control: &mut Consumer<Record>) {{\n        let available: u32 = control.available_at_entry().min(self.limit);\n        for _ in 0..available {{\n            {POP}\n        }}\n    }}\n}}"
            ),
        ),
        (
            "form-count-capped-by-a-literal",
            counted("let available = control.available_at_entry().min(64);"),
        ),
        (
            "form-count-capped-by-a-path",
            counted("let available = control.available_at_entry().min(Self::LIMIT);"),
        ),
        (
            "form-count-from-map-or",
            "impl Drain {\n    fn run(&mut self) {\n        let available = self.control.as_ref().map_or(0, Consumer::available_at_entry).min(CAP);\n        for _ in 0..available {\n            let Some(Ok(record)) = self.control.as_mut().map(Consumer::try_pop) else {\n                break;\n            };\n            apply(record);\n        }\n    }\n}"
                .to_owned(),
        ),
        (
            "form-labelled-loop",
            drain(
                "let available = control.available_at_entry();\n    'drain: for _ in 0..available {\n        let Ok(record) = control.try_pop() else {\n            break 'drain;\n        };\n        apply(record);\n    }",
            ),
        ),
        (
            "form-count-compared-before-the-loop",
            counted(
                "let available = control.available_at_entry();\n    if available == 0 {\n        return;\n    }\n    debug_assert!(available <= 8, \"too many\");",
            ),
        ),
        (
            "form-alias-with-a-type",
            drain(&format!(
                "let available = control.available_at_entry();\n    let mut remaining: usize = available;\n    while remaining != 0 {{\n        remaining -= 1;\n        {POP}\n    }}"
            )),
        ),
        (
            "form-pop-token-in-a-macro-in-a-counted-loop",
            drain(
                "let available = control.available_at_entry();\n    for _ in 0..available {\n        note!(control.try_pop());\n    }",
            ),
        ),
        (
            "form-shadow-in-a-closed-block",
            counted(
                "let available = control.available_at_entry();\n    {\n        let available = usize::MAX;\n        note(available);\n    }",
            ),
        ),
    ];
    for (name, text) in rows {
        accepts(name, &text);
    }
}

#[test]
fn a_count_that_the_rule_cannot_prove_is_refused() {
    let rows: [(&str, String); 10] = [
        (
            "count-on-the-right-of-a-comparison",
            counted(
                "let available = control.available_at_entry();\n    if 0 == available {\n        return;\n    }",
            ),
        ),
        (
            "count-in-arithmetic",
            counted(
                "let available = control.available_at_entry();\n    let doubled = available * 2;",
            ),
        ),
        (
            "count-passed-to-a-function",
            counted("let available = control.available_at_entry();\n    touch(available);"),
        ),
        (
            "count-in-a-debug-assert-call",
            counted(
                "let available = control.available_at_entry();\n    debug_assert!(touch(available));",
            ),
        ),
        (
            "count-in-another-assert-macro",
            counted(
                "let available = control.available_at_entry();\n    debug_assert_eq!(available, 3);",
            ),
        ),
        (
            "count-capped-by-a-method",
            counted("let available = control.available_at_entry().min(self.limit());"),
        ),
        (
            "count-declared-then-assigned",
            counted("let available;\n    available = control.available_at_entry();"),
        ),
        (
            "count-also-the-loop-pattern",
            drain(&format!(
                "let available = control.available_at_entry();\n    for available in 0..available {{\n        {POP}\n    }}"
            )),
        ),
        (
            "inclusive-range-count",
            drain(&format!(
                "let available = control.available_at_entry();\n    for _ in 0..=available {{\n        {POP}\n    }}"
            )),
        ),
        (
            "while-count-decremented-in-another-form",
            drain(&format!(
                "let mut remaining = control.available_at_entry();\n    while remaining != 0 {{\n        remaining = remaining - 1;\n        {POP}\n    }}"
            )),
        ),
    ];
    for (name, text) in rows {
        refuses(name, &text);
    }
}

/// A collection loop with no count pops once per element, not once per record counted at entry.
#[test]
fn a_loop_with_no_count_does_not_bound_a_pop() {
    refuses(
        "collection-loop-without-a-count",
        "fn drain(controls: &mut [Consumer<Record>]) {\n    for control in controls.iter_mut() {\n        let Ok(record) = control.try_pop() else {\n            continue;\n        };\n        apply(record);\n    }\n}",
    );
}

/// D2: the nearest binding of the count is the binding that the loop sees. A binder of any kind
/// between the entry binding and the loop makes the count that binder's.
#[test]
fn every_kind_of_binder_takes_the_count() {
    let inner = format!("for _ in 0..available {{\n                {POP}\n            }}");
    let rows: [(&str, String); 6] = [
        (
            "binder-match-arm",
            drain(&format!(
                "let available = control.available_at_entry();\n    match limit {{\n        Some(available) => {{\n            {inner}\n        }}\n        None => {{}}\n    }}"
            )),
        ),
        (
            "binder-if-let",
            drain(&format!(
                "let available = control.available_at_entry();\n    if let Some(available) = limit {{\n        {inner}\n    }}"
            )),
        ),
        (
            "binder-while-let",
            drain(&format!(
                "let available = control.available_at_entry();\n    while let Some(available) = limits.next() {{\n        {inner}\n    }}"
            )),
        ),
        (
            "binder-closure-parameter",
            drain(&format!(
                "let available = control.available_at_entry();\n    let run = |available: usize| {{\n        {inner}\n    }};\n    run(3);"
            )),
        ),
        (
            "binder-tuple-let",
            drain(&format!(
                "let available = control.available_at_entry();\n    let (available, other) = (usize::MAX, 0);\n    {inner}"
            )),
        ),
        (
            "binder-function-parameter",
            format!(
                "fn drain(control: &mut Consumer<Record>, available: usize) {{\n    {inner}\n}}"
            ),
        ),
    ];
    for (name, text) in rows {
        refuses(name, &text);
    }
}

/// D4: the nodes on a pop's path are an allow-list. A pop below any other node is not reached.
#[test]
fn a_pop_below_a_node_outside_the_allow_list_is_refused() {
    let rows: [(&str, String); 3] = [
        (
            "allow-list-const-block",
            drain(
                "let available = control.available_at_entry();\n    for _ in 0..available {\n        let record = const { control.try_pop() };\n        apply(record);\n    }",
            ),
        ),
        (
            "allow-list-const-item",
            "const DRAINED: bool = {\n    let _ = CONTROL.try_pop();\n    true\n};".to_owned(),
        ),
        (
            "allow-list-macro-item",
            "macro_rules! drain_once {\n    () => {\n        let _ = control.try_pop();\n    };\n}"
                .to_owned(),
        ),
    ];
    for (name, text) in rows {
        refuses(name, &text);
    }
}

/// D4: `Expr::Async` is not on the list. The `async` keyword is forbidden too (A2), so both
/// classes report.
#[test]
fn a_counted_drain_in_an_async_block_is_refused_twice() {
    let text = "fn drain(control: &mut Consumer<Record>) {\n    let _ = async {\n        let available = control.available_at_entry();\n        for _ in 0..available {\n            let Ok(record) = control.try_pop() else {\n                break;\n            };\n            apply(record);\n        }\n    };\n}";
    expect_failure(
        "drain-in-async-block",
        &[Edit::ReplaceLine {
            path: BUILTINS_COMPILER,
            old: ANCHOR,
            new: text,
        }],
        &[Class::ForbiddenBody, DRAIN],
    );
}

/// D5: a raw-identifier method name is the same method.
#[test]
fn a_raw_identifier_pop_is_a_pop() {
    refuses(
        "raw-identifier-pop",
        "fn drain(control: &mut Consumer<Record>) {\n    loop {\n        let Ok(record) = control.r#try_pop() else {\n            break;\n        };\n        apply(record);\n    }\n}",
    );
}

/// D5: a pop in a macro's token tree is judged by the loops around the macro call, and a closure
/// does not hide the loop around it.
#[test]
fn a_pop_in_a_macro_or_a_closure_is_judged_by_the_loops_around_it() {
    refuses(
        "pop-token-in-a-macro-in-an-unbounded-loop",
        "fn drain(control: &mut Consumer<Record>) {\n    loop {\n        note!(control.try_pop());\n    }\n}",
    );
    refuses(
        "pop-in-a-closure-in-an-unbounded-loop",
        "fn drain(control: &mut Consumer<Record>) {\n    loop {\n        let pop = || control.try_pop();\n        apply(pop());\n    }\n}",
    );
}

/// D3: a loop outside a nested `fn` counts as around a pop inside it.
#[test]
fn a_pop_in_a_nested_function_is_judged_by_the_loops_outside_it() {
    refuses(
        "pop-in-a-nested-function-in-an-unbounded-loop",
        "fn drain(control: &mut Consumer<Record>) {\n    loop {\n        fn helper(control: &mut Consumer<Record>) {\n            let _ = control.try_pop();\n        }\n        helper(control);\n        break;\n    }\n}",
    );
}

/// D1: every node the rule reads lies in the pop's region. The same drain passes when its count
/// binding is inside the region.
#[test]
fn a_count_binding_above_the_region_is_not_read() {
    let above = format!(
        "fn drain_above(control: &mut Consumer<Record>) {{\n    let available = control.available_at_entry();\n    // {BEGIN}\n    for _ in 0..available {{\n        {POP}\n    }}\n    // {END}\n}}\n"
    );
    expect_failure(
        "binding-above-begin",
        &[Edit::Append {
            path: BUFFER,
            text: &above,
        }],
        &[DRAIN],
    );
    let inside = format!(
        "fn drain_inside(control: &mut Consumer<Record>) {{\n    // {BEGIN}\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        {POP}\n    }}\n    // {END}\n}}\n"
    );
    let report = expect_valid(
        "binding-inside-region",
        &[Edit::Append {
            path: BUFFER,
            text: &inside,
        }],
    );
    assert_eq!(
        (report.regions, report.files),
        (BASE_REGIONS + 1, BASE_FILES)
    );
}

/// D1: a region opened inside a loop body does not read that loop as a bounded outer loop.
#[test]
fn a_loop_that_encloses_the_region_is_refused() {
    let body = format!(
        "    // {BEGIN}\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        {POP}\n    }}\n    // {END}\n"
    );
    let around = format!(
        "fn drain_around(control: &mut Consumer<Record>) {{\n    loop {{\n{body}        break;\n    }}\n}}\n"
    );
    expect_failure(
        "loop-around-the-region",
        &[Edit::Append {
            path: BUFFER,
            text: &around,
        }],
        &[DRAIN],
    );
    let plain = format!("fn drain_plain(control: &mut Consumer<Record>) {{\n{body}}}\n");
    expect_valid(
        "no-loop-around-the-region",
        &[Edit::Append {
            path: BUFFER,
            text: &plain,
        }],
    );
}

/// D1: a region inside a macro's token tree is tokens, not syntax. Any pop in it is refused, even
/// a counted drain.
#[test]
fn a_pop_in_a_region_inside_a_macro_body_is_refused() {
    let text = format!(
        "macro_rules! drain_all {{\n    () => {{\n        // {BEGIN}\n        let available = control.available_at_entry();\n        for _ in 0..available {{\n            {POP}\n        }}\n        // {END}\n    }};\n}}\n"
    );
    expect_failure(
        "pop-in-a-macro-body-region",
        &[Edit::Append {
            path: BUFFER,
            text: &text,
        }],
        &[DRAIN],
    );
}

/// D1: the nodes of a region that starts inside an `impl` and ends after it are the maximal nodes
/// between the markers, so a counted drain in the last method passes.
#[test]
fn a_region_spanning_the_end_of_an_impl_reads_its_nodes() {
    let text = format!(
        "impl Drain {{\n    fn first(&self) {{}}\n    // {BEGIN}\n    fn run(&mut self, control: &mut Consumer<Record>) {{\n        let available = control.available_at_entry();\n        for _ in 0..available {{\n            {POP}\n        }}\n    }}\n}}\nfn later() {{}}\n// {END}\n"
    );
    let report = expect_valid(
        "region-spanning-an-impl",
        &[Edit::Append {
            path: BUFFER,
            text: &text,
        }],
    );
    assert_eq!(
        (report.regions, report.files),
        (BASE_REGIONS + 1, BASE_FILES)
    );
}
