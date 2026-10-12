//! The ported drain cases: the awk self-test's `marked-unbounded-try-pop-drain` and the names of
//! its drain loop, each one edit of the base tree. The fourteen at the end wait for the outer-loop,
//! attribute, item and constructor rules (B1b); the rest were refused by the first slice.

use super::Ported;

pub(super) const PORTED: &[Ported] = &[
    Ported {
        name: "marked-unbounded-try-pop-drain",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: "fn drain_fader_controls() {}",
        text: "fn drain_fader_controls() { while let Ok(record) = control.try_pop() { apply(record); } }",
    },
    Ported {
        name: "wrapped-while-let",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"impl MatrixBank {
    fn drain_matrix_controls(&mut self) {
        while let Ok(record) = self
            .some_long_receiver_name
            .control_consumer_for_this_lane
            .try_pop()
        {
            apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "let-else-loop",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    loop {
        let Ok(record) = control.try_pop() else { break };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "path-pop",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    while let Ok(record) = Consumer::try_pop(control) {
        apply(record);
    }
}"#,
    },
    Ported {
        name: "constant-bound",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    for _ in 0..64 {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "bound-not-at-entry",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = control.capacity();
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "second-loop-in-bounded-region",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_fader_controls() {}"#,
        text: r#"fn drain_fader_controls(control: &mut Consumer<Record>) {
    loop {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "loop-inside-bounded-for",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = control.available_at_entry();
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
        loop {
            let Ok(extra) = control.try_pop() else {
                break;
            };
            apply(extra);
        }
    }
}"#,
    },
    Ported {
        name: "count-from-another-function",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_fader_controls() {}"#,
        text: r#"fn drain_fader_prepare(control: &mut Consumer<Record>) {
    let available = control.available_at_entry();
    apply(available);
}
fn drain_fader_tail(control: &mut Consumer<Record>, available: usize) {
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "shadowed-count",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"        for _ in 0..available {"#,
        text: r#"        let available = usize::MAX;
        for _ in 0..available {"#,
    },
    Ported {
        name: "reassigned-count",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"        let available = control.available_at_entry();"#,
        text: r#"        let mut available = control.available_at_entry();
        available = usize::MAX;"#,
    },
    Ported {
        name: "while-without-decrement",
        kind: "edit",
        path: "crates/effect-contract/src/live.rs",
        anchor: r#"                remaining -= 1;"#,
        text: r#""#,
    },
    Ported {
        name: "while-decrement-not-first",
        kind: "edit",
        path: "crates/effect-contract/src/live.rs",
        anchor: r#"                remaining -= 1;"#,
        text: r#"                if staged == 0 {
                    remaining -= 1;
                }"#,
    },
    Ported {
        name: "labelled-loop",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    'drain: loop {
        let Ok(record) = control.try_pop() else {
            break 'drain;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "loop-expression",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) -> usize {
    let mut applied = 0;
    let last = loop {
        let Ok(record) = control.try_pop() else {
            break applied;
        };
        applied += apply(record);
    };
    last
}"#,
    },
    Ported {
        name: "wrapped-while-body",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"impl LaneDrain {
    fn drain_matrix_controls(&mut self) {
        while self
            .control_lane_consumer_for_this_strip
            .has_pending_records_for_the_current_block()
            && self.enabled
        {
            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {
                break;
            };
            self.apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "wrapped-for-bound",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"impl LaneDrain {
    fn drain_matrix_controls(&mut self) {
        let available = self
            .control_lane_consumer_for_this_strip
            .available_at_entry();
        for _ in 0..available
            + self
                .control_lane_consumer_for_this_strip
                .capacity_of_the_ring_in_records_total()
        {
            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {
                break;
            };
            self.apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "paren-led-header",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"impl LaneDrain {
    fn drain_matrix_controls(&mut self) {
        for _ in 0..usize::from(
            self.control_lane_consumer_for_this_strip
                .capacity_of_the_ring_in_records(),
        ) {
            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {
                break;
            };
            self.apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "continuation-led-header",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"impl LaneDrain {
    fn drain_matrix_controls(&mut self) {
        for _ in [
            self.first_lane_consumer_count,
            self.second_lane_consumer_count,
            self.third_lane_consumer_count,
        ]
        .into_iter()
        .take(64)
        {
            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {
                break;
            };
            self.apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "entry-name-in-binding",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let cap = control.not_available_at_entry_cap();
    for _ in 0..cap {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "entry-name-in-header",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    for _ in 0..control.not_available_at_entry_cap() {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "header-past-closed-body",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    for _ in 0..[(); 64].map(|()| control.try_pop()).len() {}
    if control.available_at_entry() != 0 {
        apply(control.peek());
    }
}"#,
    },
    Ported {
        name: "pattern-rebound-count",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>, limit: Option<usize>) {
    let available = control.available_at_entry();
    let Some(available) = limit else {
        return;
    };
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "while-match-body",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"impl LaneDrain {
    fn drain_matrix_controls(&mut self, control: &mut Consumer<Record>) {
        while match self.lane_selector_for_this_block {
            Lane::Left => &self.left_consumer,
            Lane::Right => &self.right_consumer,
        }
        .has_records()
        {
            let Ok(record) = control.try_pop() else {
                break;
            };
            self.apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "while-match-brace",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>, lane: Lane) {
    while match lane {
        Lane::Left => control.left_ready(),
        Lane::Right => control.right_ready(),
    } {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "struct-pattern-for",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>, lanes: &[Lane]) {
    for Lane { left, right } in lanes.iter().cycle() {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record, left, right);
    }
}"#,
    },
    Ported {
        name: "if-else-condition",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    while if control.left_first() {
        control.left_ready()
    } else {
        control.right_ready()
    } {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "as-cast-condition",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>, lane: Lane) {
    while match lane {
        Lane::Left => 0_u8,
        Lane::Right => 1_u8,
    } as usize
        != control.cursor()
    {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "block-operand-condition",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    while control.ready() && {
        control.refresh();
        control.more()
    } {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "brace-in-string-header",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    while control.name() != "};{" {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "brace-in-char-header",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    while control.tag() != '}' {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "brace-in-raw-string-header",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r##"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    while control.name() != r#"}";{"# {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"##,
    },
    Ported {
        name: "brace-in-block-comment",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    while control.more() {
        /* } closes nothing: a comment */
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "count-rebound-by-for-pattern",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>, limits: &[usize]) {
    let available = control.available_at_entry();
    for &available in limits.iter() {
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "inflated-count",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = control.available_at_entry() + control.capacity();
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "while-count-raised-in-body",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let mut remaining = control.available_at_entry();
    while remaining != 0 {
        remaining -= 1;
        let Ok(record) = control.try_pop() else {
            break;
        };
        if apply(record) {
            remaining += 1;
        }
    }
}"#,
    },
    Ported {
        name: "open-tail-last-file",
        kind: "append",
        path: "hosts/host-web/src/lib.rs",
        anchor: r#""#,
        text: r#"// @END@
// @BEGIN@
fn drain_tail(control: &mut Consumer<Record>) {
    loop {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}
"#,
    },
    Ported {
        name: "raw-c-string-literals",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r##################"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let _tags = (r#################"x"#################, cr#"a"b"#, cr"\");
    loop {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
    // "################
}"##################,
    },
    Ported {
        name: "region-ends-in-comment",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls() {}
/* A note that runs past the marker:
// @END@
// @BEGIN@
*/"#,
    },
    Ported {
        name: "count-in-user-macro",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = control.available_at_entry();
    rebind!(available == 0);
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "count-raised-by-max",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = control.available_at_entry().max(MAX_RECORDS_PER_BLOCK);
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "count-from-closed-region-function",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls() {}
// @END@
#[allow(non_upper_case_globals)]
const available: usize = usize::MAX;
fn prepare_matrix(control: &mut Consumer<Record>) {
    // @BEGIN@
    let available = control.available_at_entry();
}
fn drain_matrix(control: &mut Consumer<Record>) {
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "count-in-raw-identifier-macro",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"macro_rules! r#while {
    ($x:ident $($rest:tt)*) => {
        let $x = usize::MAX;
    };
}

fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = control.available_at_entry();
    r#while!(available == 0);
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "bounded-drain-inside-outer-loop",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    loop {
        let available = control.available_at_entry();
        if available == 0 {
            break;
        }
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "wrapped-outer-while",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"impl LaneDrain {
    fn drain_matrix_controls(&mut self) {
        while self
            .control_lane_consumer_for_this_strip
            .has_pending_records_for_the_current_block()
            && self.enabled
        {
            let available = self
                .control_lane_consumer_for_this_strip
                .available_at_entry();
            for _ in 0..available {
                let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {
                    break;
                };
                self.apply(record);
            }
        }
    }
}"#,
    },
    Ported {
        name: "paren-open-range-outer",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    for _ in (0..) {
        let available = control.available_at_entry();
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "outer-repeat",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    for _ in core::iter::repeat(()) {
        let available = control.available_at_entry();
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "outer-over-iterator-parameter",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>, rounds: RangeFrom<u32>) {
    for _ in rounds {
        let available = control.available_at_entry();
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "outer-over-rebound-slice-parameter",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>, rounds: &[u32]) {
    let rounds = 0_u32..;
    for _ in rounds {
        let available = control.available_at_entry();
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
    }
}"#,
    },
    Ported {
        name: "outer-zip-then-cycle",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(controls: &[Consumer<Record>], gains: &[f32]) {
    for (control, gain) in controls.iter().zip(gains).cycle() {
        let available = control.available_at_entry();
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply_scaled(*gain, record);
        }
    }
}"#,
    },
    Ported {
        name: "outer-over-slice-parameter-shadowed-by-block-const",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(rounds: &[u8], control: &mut Consumer<Record>) {
    {
        for _ in rounds {
            let available = control.available_at_entry();
            for _ in 0..available {
                let Ok(record) = control.try_pop() else {
                    break;
                };
                apply(record);
            }
        }
        #[allow(non_upper_case_globals)]
        const rounds: core::ops::RangeFrom<u32> = 0..;
    }
}"#,
    },
    Ported {
        name: "count-under-cfg",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = usize::MAX;
    #[cfg(debug_assertions)]
    let available = control.available_at_entry();
    for _ in 0..available {
        let Ok(record) = control.try_pop() else {
            break;
        };
        apply(record);
    }
}"#,
    },
    Ported {
        name: "count-shadowed-by-block-static",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let available = control.available_at_entry();
    {
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
        static available: usize = usize::MAX;
    }
}"#,
    },
    Ported {
        name: "from-fn",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    core::iter::from_fn(|| control.try_pop().ok()).for_each(apply);
}"#,
    },
    Ported {
        name: "bare-from-fn",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    from_fn(|| control.try_pop().ok()).for_each(apply);
}"#,
    },
    Ported {
        name: "repeat-with",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    core::iter::repeat_with(|| control.try_pop())
        .map_while(Result::ok)
        .for_each(apply);
}"#,
    },
    Ported {
        name: "array-from-fn-pop",
        kind: "edit",
        path: "crates/builtins-compiler/src/lib.rs",
        anchor: r#"fn drain_matrix_controls() {}"#,
        text: r#"fn drain_matrix_controls(control: &mut Consumer<Record>) {
    let records: [Result<Record, Empty>; 4] = core::array::from_fn(|_| control.try_pop());
    records.into_iter().flatten().for_each(apply);
}"#,
    },
];
