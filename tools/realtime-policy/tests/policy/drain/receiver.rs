//! #1426 D1, D2: a counted drain pops the queue it counts, spelled `<recv>.try_pop()`, and nothing
//! replaces that queue's place between the count and the end of the loop.

use super::{each_row, refuses};

/// The pop-and-apply body of the rows below, popping `receiver`.
fn pop(receiver: &str) -> String {
    format!(
        "let Ok(record) = {receiver}.try_pop() else {{\n            break;\n        }};\n        apply(record);"
    )
}

/// Each row is red if the rule again accepts a counted drain that pops another queue than the one
/// it counts, so the pop runs past the popped queue's entry count:
///
/// - `pop-receiver-differs`, `-differs-header`, `-differs-index`: the pop's receiver is compared
///   with the count's at all (D1.1), in the `let` form, the header form and an index;
/// - `pop-receiver-unreadable-call`, `-unreadable-paren`: a pop outside `<recv>.try_pop()` is
///   refused, not read as some receiver (D2);
/// - `pop-receiver-rebound`: the roots resolve to one binding, not one spelling (D1.2);
/// - `pop-receiver-swapped-with-spare`, `-swapped-before-loop`: the counted place is not replaced
///   by an explicit `&mut` or a by-value use between the count and the end of the loop (D1.3).
#[test]
fn a_counted_drain_pops_the_queue_it_counts() {
    let two = "fn drain(control: &mut Consumer<Record>, other: &mut Consumer<Record>) {";
    let rows: [(&str, String); 8] = [
        (
            "pop-receiver-differs",
            format!(
                "{two}\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        {}\n    }}\n}}",
                pop("other")
            ),
        ),
        (
            "pop-receiver-differs-header",
            format!(
                "{two}\n    for _ in 0..control.available_at_entry() {{\n        {}\n    }}\n}}",
                pop("other")
            ),
        ),
        (
            "pop-receiver-unreadable-call",
            format!(
                "{two}\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        let Ok(record) = Consumer::try_pop(control) else {{\n            break;\n        }};\n        apply(record);\n    }}\n}}"
            ),
        ),
        (
            "pop-receiver-unreadable-paren",
            format!(
                "{two}\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        {}\n    }}\n}}",
                pop("(control)")
            ),
        ),
        (
            "pop-receiver-differs-index",
            format!(
                "fn drain(controls: &mut [Consumer<Record>], lane: usize, other: usize) {{\n    let available = controls[lane].available_at_entry();\n    for _ in 0..available {{\n        {}\n    }}\n}}",
                pop("controls[other]")
            ),
        ),
        (
            "pop-receiver-rebound",
            format!(
                "{two}\n    let available = control.available_at_entry();\n    for _ in 0..available {{\n        let control = other;\n        {}\n    }}\n}}",
                pop("control")
            ),
        ),
        (
            "pop-receiver-swapped-with-spare",
            format!(
                "impl Drain {{\n    fn run(&mut self) {{\n        let available = self.control.available_at_entry();\n        for _ in 0..available {{\n            {}\n            core::mem::swap(&mut self.control, &mut self.spare);\n        }}\n    }}\n}}",
                pop("self.control")
            ),
        ),
        (
            "pop-receiver-swapped-before-loop",
            format!(
                "{two}\n    let available = control.available_at_entry();\n    core::mem::swap(control, other);\n    for _ in 0..available {{\n        {}\n    }}\n}}",
                pop("control")
            ),
        ),
    ];
    each_row(rows, |name, text| refuses(name, &text));
}
