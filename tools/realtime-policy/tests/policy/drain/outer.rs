//! The rest of the drain rule (B1b): finite outer loops (D1), attributes and items (D2) and
//! constructors (D3).

use realtime_policy::Class;

use super::super::common::{BEGIN, END, Edit, expect_failure, expect_valid};
use super::{accepts, each_row, refuses};

const BUFFER: &str = "crates/engine/src/realtime/buffer.rs";

/// The pop-and-apply body that every drain below shares.
const POP: &str = "let Ok(record) = control.try_pop() else {\n                break;\n            };\n            apply(record);";

/// A counted drain, with `opening` as the first statements of the outer loop's body.
fn counted_drain(opening: &str) -> String {
    format!(
        "{opening}        let available = control.available_at_entry();\n        for _ in 0..available {{\n            {POP}\n        }}\n"
    )
}

/// A drain function with the extra parameters `params` and `prelude` statements, whose counted
/// drain stands in the loop `header`.
fn around(params: &str, prelude: &str, header: &str, opening: &str) -> String {
    format!(
        "fn drain(control: &mut Consumer<Record>{params}) {{\n{prelude}    {header} {{\n{}    }}\n}}",
        counted_drain(opening)
    )
}

/// The same drain, with a block item after the loop that can name `name`.
fn shadowed_by(params: &str, header: &str, item: &str) -> String {
    format!(
        "fn drain(control: &mut Consumer<Record>{params}) {{\n    let available = control.available_at_entry();\n    {{\n        {header} {{\n            {POP}\n        }}\n        {item}\n    }}\n}}"
    )
}

/// A counted drain of each element of the slice parameter `rounds`, with a block item
/// after the loop. Only the item scan tells it from the finite outer loop of D1, so only a row
/// built this way defends the scan on the slice parameter.
fn slice_shadowed_by(item: &str) -> String {
    format!(
        "fn drain(rounds: &mut [Consumer<Record>]) {{\n    for control in rounds {{\n{}    }}\n    {item}\n}}",
        counted_drain("")
    )
}

const CONFIG: &str = ", cfg: &Config";

/// A drain of `controls[lane]` in each pass of the loop `header`, which binds `lane`.
fn by_lane(params: &str, header: &str) -> String {
    format!(
        "fn drain(controls: &mut [Consumer<Record>; 4]{params}) {{\n    {header} {{\n        let available = controls[lane].available_at_entry();\n        for _ in 0..available {{\n            let Ok(record) = controls[lane].try_pop() else {{\n                break;\n            }};\n            apply(record);\n        }}\n    }}\n}}"
    )
}

/// A drain of `control` in each pass of the loop `header`, which binds `control`, with `prelude`
/// statements before the loop.
fn by_element(params: &str, prelude: &str, header: &str) -> String {
    format!(
        "fn drain({params}) {{\n{prelude}    {header} {{\n{}    }}\n}}",
        counted_drain("")
    )
}

const CONFIG_MUT: &str = "cfg: &mut Config";

/// D1 (B1b) and #1418 D1: a loop around a counted drain that runs a finite number of passes by its
/// form, and drains a different queue on each pass. Each row keeps one finite form.
#[test]
fn the_finite_outer_loops_pass() {
    let rows: Vec<(&'static str, String)> = vec![
        ("outer-literal-bound", by_lane("", "for lane in 0..4")),
        (
            "outer-suffixed-literal-bound",
            by_lane("", "for lane in 0..4_usize"),
        ),
        (
            "outer-identifier-bound",
            by_lane(", lanes: usize", "for lane in 0..lanes"),
        ),
        (
            "outer-field-bound",
            by_lane(CONFIG, "for lane in 0..cfg.lanes"),
        ),
        (
            "outer-segment-bound",
            by_lane("", "for lane in 0..Bank::LANES"),
        ),
        (
            "outer-reference",
            by_element("cfg: &Config", "", "for control in &cfg.controls"),
        ),
        (
            "outer-mutable-reference",
            by_element(CONFIG_MUT, "", "for control in &mut cfg.controls"),
        ),
        (
            "outer-iter",
            by_element("cfg: &Config", "", "for control in cfg.controls.iter()"),
        ),
        (
            "outer-iter-mut-enumerate",
            by_element(
                CONFIG_MUT,
                "",
                "for (_, control) in cfg.controls.iter_mut().enumerate()",
            ),
        ),
        (
            // The queue on the zip chain's receiver.
            "outer-zip-chain",
            by_element(
                CONFIG_MUT,
                "",
                "for ((((_, (control, _)), _), _), _) in cfg.a.iter_mut().zip(&cfg.b).enumerate().zip(&mut cfg.c).zip(cfg.d).zip(4)",
            ),
        ),
        (
            "outer-slice-reference",
            by_element("rounds: &[Consumer<Record>]", "", "for control in rounds"),
        ),
        (
            "outer-slice-mutable",
            by_element("rounds: &mut [Consumer<Record>]", "", "for control in rounds"),
        ),
        (
            "outer-slice-lifetime",
            by_element(
                "rounds: &'static [Consumer<Record>]",
                "",
                "for control in rounds",
            ),
        ),
        (
            "outer-array-value",
            by_element("rounds: [Consumer<Record>; 4]", "", "for control in rounds"),
        ),
        (
            "outer-mutable-array-value",
            by_element(
                "mut rounds: [Consumer<Record>; 4]",
                "",
                "for control in rounds",
            ),
        ),
        (
            "outer-array-reference",
            by_element("rounds: &[Consumer<Record>; 4]", "", "for control in rounds"),
        ),
        ("outer-labelled", by_lane("", "'lanes: for lane in 0..4")),
        (
            "outer-counted-for-header",
            by_lane(
                ", gate: &mut Consumer<Record>",
                "for lane in 0..gate.available_at_entry()",
            ),
        ),
        (
            // Two levels: the inner collection is the outer loop's element.
            "outer-two-finite-loops",
            format!(
                "fn drain({CONFIG_MUT}) {{\n    for bank in &mut cfg.banks {{\n        for control in &mut bank.controls {{\n{}        }}\n    }}\n}}",
                counted_drain("")
            ),
        ),
        (
            "outer-slice-parameter-of-a-method",
            format!(
                "impl Drain {{\n    fn run(&mut self, rounds: &mut [Consumer<Record>]) {{\n        for control in rounds {{\n{}        }}\n    }}\n}}",
                counted_drain("")
            ),
        ),
        (
            // NIT-3: the index form followed by one `.as_mut()`, in a `let .. else`.
            "outer-index-as-mut-let-else",
            "impl Bank {\n    fn drain(&mut self) {\n        for lane in 0..LANES {\n            let Some(control) = self.controls[lane].as_mut() else {\n                continue;\n            };\n            let available = control.available_at_entry();\n            for _ in 0..available {\n                let Ok(record) = control.try_pop() else {\n                    break;\n                };\n                apply(record);\n            }\n        }\n    }\n}".to_owned(),
        ),
        (
            // NIT-3: an `if let` in an `else` branch binds as `if let` does.
            "outer-else-if-let",
            "fn drain(controls: &mut [Option<Consumer<Record>>], skip: bool) {\n    for control in controls.iter_mut() {\n        if skip {\n            continue;\n        } else if let Some(control) = control.as_mut() {\n            let available = control.available_at_entry();\n            for _ in 0..available {\n                let Ok(record) = control.try_pop() else {\n                    break;\n                };\n                apply(record);\n            }\n        }\n    }\n}".to_owned(),
        ),
        (
            // Two levels through an index: the base of `bank.controls[lane]` is the outer element.
            "outer-index-in-an-element",
            "impl Set {\n    fn drain(&mut self) {\n        for bank in &mut self.banks {\n            for lane in 0..LANES {\n                let available = bank.controls[lane].available_at_entry();\n                for _ in 0..available {\n                    let Ok(record) = bank.controls[lane].try_pop() else {\n                        break;\n                    };\n                    apply(record);\n                }\n            }\n        }\n    }\n}".to_owned(),
        ),
    ];
    each_row(rows, |name, text| accepts(name, &text));
}

/// #1418 D1 (MINOR-2): the walk has no depth cap. A 64-link rebinding chain and a 60-level nest of
/// distinctly named bank loops each give one answer.
#[test]
fn deep_selecting_chains_pass() {
    let links: String = (0..63)
        .map(|_| "        let control = &mut control;\n")
        .collect();
    let chain = format!(
        "fn drain(controls: &mut [Consumer<Record>; 2]) {{\n    for lane in 0..2 {{\n        let control = &mut controls[lane];\n{links}{}    }}\n}}",
        counted_drain("")
    );
    let mut nest = format!(
        "        for control in &mut b59.controls {{\n{}        }}\n",
        counted_drain("")
    );
    for level in (0..60).rev() {
        let outer = if level == 0 {
            "cfg".to_owned()
        } else {
            format!("b{}", level - 1)
        };
        nest = format!("    for b{level} in &mut {outer}.banks {{\n{nest}    }}\n");
    }
    let nest = format!("fn drain({CONFIG_MUT}) {{\n{nest}}}");
    each_row(
        [
            ("outer-64-link-chain", chain),
            ("outer-60-level-nest", nest),
        ],
        |name, text| accepts(name, &text),
    );
}

/// #1418 D1: a finite loop around a counted drain that can drain one queue on more than one pass is
/// refused. One row per way the loop can fail to select, or let the selected queue move.
#[test]
fn the_outer_loops_that_do_not_select_the_queue_are_refused() {
    let lane_drain = |queue: &str| {
        format!(
            "        let available = {queue}.available_at_entry();\n        for _ in 0..available {{\n            let Ok(record) = {queue}.try_pop() else {{\n                break;\n            }};\n            apply(record);\n        }}\n"
        )
    };
    let rows: Vec<(&'static str, String)> = vec![
        // D4: a pop-until-empty drain through a finite but endless-in-practice loop.
        (
            "outer-usize-max",
            around("", "", "for _ in 0..usize::MAX", ""),
        ),
        (
            "outer-unit-array-parameter",
            around(", rounds: [(); usize::MAX]", "", "for () in rounds", ""),
        ),
        (
            // A limit of B1b: `&mut` of an endless iterator is finite by its form.
            "outer-borrowed-open-range",
            around(
                "",
                "    let mut rounds = 0_u32..;\n",
                "for _ in &mut rounds",
                "",
            ),
        ),
        (
            "outer-counted-loop",
            around(
                "",
                "    let outer_available = control.available_at_entry();\n",
                "for _ in 0..outer_available",
                "",
            ),
        ),
        (
            "outer-counted-while",
            around(
                "",
                "    let mut passes = control.available_at_entry();\n",
                "while passes != 0",
                "        passes -= 1;\n",
            ),
        ),
        // D4: the old base-tree `drain_lane_pairs`, one queue in each of two lanes.
        (
            "same-queue-lane-pairs",
            around("", "", "for lane in 0..2", ""),
        ),
        (
            "outer-index-expression",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2]) {{\n    for lane in 0..2 {{\n        let control = &mut controls[lane % 2];\n{}    }}\n}}",
                counted_drain("")
            ),
        ),
        (
            // Attempt 2's mutation H: a `mut` index can be set to one value on every pass.
            "outer-mutable-index",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2]) {{\n    for mut lane in 0..2 {{\n        lane = 0;\n{}    }}\n}}",
                lane_drain("controls[lane]")
            ),
        ),
        (
            "outer-index-shadowed-by-block-const",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2]) {{\n    for lane in 0..2 {{\n        const lane: usize = 0;\n{}    }}\n}}",
                lane_drain("controls[lane]")
            ),
        ),
        (
            // Attempt 2's L2: the element slot of an `.enumerate()` is a table value, not the index.
            "outer-enumerated-index-table",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2], order: &[usize]) {{\n    for (_, lane) in order.iter().enumerate() {{\n{}    }}\n}}",
                lane_drain("controls[lane]")
            ),
        ),
        (
            "outer-index-base-swapped",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2]) {{\n    for lane in 0..2 {{\n        controls.swap(0, 1);\n{}    }}\n}}",
                lane_drain("controls[lane]")
            ),
        ),
        (
            // The same reorder through a slice of the base: `[..]` is not the receiver's index.
            "outer-index-base-slice-swapped",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2]) {{\n    for lane in 0..2 {{\n        controls[..].swap(0, 1);\n{}    }}\n}}",
                lane_drain("controls[lane]")
            ),
        ),
        (
            // Lane 1 drains lane 0's queue again, through the spare.
            "outer-element-swapped-with-spare",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2], spare: &mut Consumer<Record>) {{\n    for (lane, control) in controls.iter_mut().enumerate() {{\n        if lane == 1 {{\n            core::mem::swap(control, spare);\n        }}\n{}        if lane == 0 {{\n            core::mem::swap(control, spare);\n        }}\n    }}\n}}",
                lane_drain("control")
            ),
        ),
        (
            // Attempt 2's D2: the outer loop must select the base, not the inner loop's collection.
            "outer-loop-around-unselected-base",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2], cfg: &mut Config) {{\n    for bank in &mut cfg.banks {{\n        for (lane, _) in bank.order.iter().enumerate() {{\n{}        }}\n    }}\n}}",
                lane_drain("controls[lane]")
            ),
        ),
        (
            // MAJOR-2: a zip argument's element selects its own loop only; every bank re-drains
            // `cfg.controls`.
            "outer-zip-carried-outward",
            format!(
                "fn drain({CONFIG_MUT}) {{\n    for bank in &mut cfg.banks {{\n        for (_, control) in bank.order.iter().zip(&mut cfg.controls) {{\n{}        }}\n    }}\n}}",
                counted_drain("")
            ),
        ),
        (
            // MAJOR-1: with the feature off, the queue is the spare on every pass.
            "outer-cfg-let",
            format!(
                "fn drain(controls: &mut [Consumer<Record>; 2], spare: &mut Consumer<Record>) {{\n    for lane in 0..2 {{\n        let control = &mut *spare;\n        #[cfg(feature = \"lanes\")]\n        let control = &mut controls[lane];\n{}    }}\n}}",
                counted_drain("")
            ),
        ),
        (
            // MINOR-1: a local macro moves the base with no name in the loop body. `self` is
            // never a read, so the item scan leaves the `macro_rules!` alone.
            "outer-macro-call",
            format!(
                "impl Bank {{\n    fn drain(&mut self) {{\n        macro_rules! rotate {{\n            () => {{\n                self.controls.swap(0, 1)\n            }};\n        }}\n        for lane in 0..2 {{\n            let () = rotate!();\n{}        }}\n    }}\n}}",
                lane_drain("self.controls[lane]")
            ),
        ),
    ];
    each_row(rows, |name, text| refuses(name, &text));
}

/// D1, D7: a loop around a counted drain that may run forever, or that the form cannot prove
/// finite, is refused. Each row names the form it breaks. Where the loop could select the queue,
/// it does, so only the finite check refuses it; `loop`, `while` and the adapters other than
/// `.enumerate()` and `.zip(..)` never select, and the selection walk refuses them too.
#[test]
fn the_outer_loops_that_may_not_end_are_refused() {
    let rows: Vec<(&'static str, String)> = vec![
        ("outer-loop", around("", "", "loop", "")),
        (
            "outer-while-condition",
            around(CONFIG, "", "while cfg.enabled", ""),
        ),
        (
            "outer-while-let",
            around(CONFIG, "", "while let Some(_) = cfg.next()", ""),
        ),
        ("outer-open-range", by_lane("", "for lane in 0..")),
        (
            "outer-range-from-one",
            by_lane(CONFIG, "for lane in 1..cfg.lanes"),
        ),
        (
            "outer-inclusive-range",
            by_lane(CONFIG, "for lane in 0..=cfg.lanes"),
        ),
        (
            "outer-reversed",
            around(CONFIG, "", "for _ in cfg.controls.iter().rev()", ""),
        ),
        (
            "outer-taken",
            around(CONFIG, "", "for _ in cfg.controls.iter().take(4)", ""),
        ),
        (
            "outer-cycled",
            around(CONFIG, "", "for _ in cfg.controls.iter().cycle()", ""),
        ),
        (
            "outer-vec-parameter-by-value",
            by_element("rounds: Vec<Consumer<Record>>", "", "for control in rounds"),
        ),
        (
            "outer-field-by-value",
            by_element("cfg: Config", "", "for control in cfg.controls"),
        ),
        (
            "outer-slice-parameter-rebound-by-a-macro",
            by_element(
                "rounds: &mut [Consumer<Record>]",
                "    macro_rules! shadow {\n        ($x:ident) => {\n            let $x = 0_u32..;\n        };\n    }\n    shadow!(rounds);\n",
                "for control in rounds",
            ),
        ),
        (
            "outer-slice-parameter-with-a-pattern",
            by_element(
                "[rounds, other]: [core::ops::RangeFrom<u32>; 2]",
                "",
                "for control in rounds",
            ),
        ),
        (
            "outer-finite-loop-inside-an-unbounded-one",
            format!(
                "fn drain(control: &mut Consumer<Record>) {{\n    loop {{\n        for _ in 0..4 {{\n{}        }}\n    }}\n}}",
                counted_drain("")
            ),
        ),
    ];
    each_row(rows, |name, text| refuses(name, &text));
}

/// D1: the slice parameter's `fn` signature lies in the pop's region, like the loop.
#[test]
fn a_slice_parameter_of_a_function_outside_the_region_is_not_read() {
    let inside = format!(
        "fn drain_params(rounds: &mut [Consumer<Record>]) {{\n    // {BEGIN}\n    for control in rounds {{\n{}    }}\n    // {END}\n}}\n",
        counted_drain("")
    );
    expect_failure(
        "slice-parameter-outside-the-region",
        &[Edit::Append {
            path: BUFFER,
            text: &inside,
        }],
        &[Class::UnboundedTryPop],
    );
    let whole = format!(
        "// {BEGIN}\nfn drain_params(rounds: &mut [Consumer<Record>]) {{\n    for control in rounds {{\n{}    }}\n}}\n// {END}\n",
        counted_drain("")
    );
    expect_valid(
        "slice-parameter-inside-the-region",
        &[Edit::Append {
            path: BUFFER,
            text: &whole,
        }],
    );
}

/// D2: an item of the region that can name the count or the slice parameter (a `const`, `static`,
/// `struct`, `enum`, `union`, `fn`, `type`, `trait` or `mod` of that name, or a `use`) shadows it
/// for its whole block, wherever it is declared; an item that names something else does not.
#[test]
fn an_item_that_can_name_the_count_refuses_the_drain() {
    let count = "for _ in 0..available";
    let rows: Vec<(&'static str, String)> = vec![
        (
            "count-shadowed-by-block-const",
            shadowed_by("", count, "const available: usize = usize::MAX;"),
        ),
        (
            "count-shadowed-by-block-static-mut",
            shadowed_by("", count, "static mut available: usize = usize::MAX;"),
        ),
        (
            "count-shadowed-by-use",
            shadowed_by("", count, "use other::available;"),
        ),
        (
            "count-shadowed-by-glob-use",
            shadowed_by("", count, "use other::*;"),
        ),
        (
            "count-shadowed-by-nested-use",
            shadowed_by("", count, "use other::{first, available as third};"),
        ),
        (
            "count-shadowed-by-block-struct",
            shadowed_by("", count, "struct available;"),
        ),
        (
            "count-shadowed-by-block-enum",
            shadowed_by("", count, "enum available { One }"),
        ),
        (
            "count-shadowed-by-block-union",
            shadowed_by("", count, "union available { one: u8 }"),
        ),
        (
            "count-shadowed-by-block-fn",
            shadowed_by("", count, "fn available() {}"),
        ),
        (
            "count-shadowed-by-block-type",
            shadowed_by("", count, "type available = usize;"),
        ),
        (
            "count-shadowed-by-block-trait",
            shadowed_by("", count, "trait available {}"),
        ),
        (
            "count-shadowed-by-block-mod",
            shadowed_by("", count, "mod available {}"),
        ),
        (
            "slice-shadowed-by-block-struct",
            slice_shadowed_by("struct rounds;"),
        ),
        (
            "slice-shadowed-by-block-static",
            slice_shadowed_by("static rounds: [u8; 1] = [0];"),
        ),
        (
            "slice-shadowed-by-use",
            slice_shadowed_by("use other::rounds;"),
        ),
        (
            "slice-shadowed-by-glob-use",
            slice_shadowed_by("use other::*;"),
        ),
    ];
    each_row(rows, |name, text| refuses(name, &text));
    let unrelated: Vec<(&'static str, String)> = vec![
        (
            "unrelated-const-after-the-loop",
            shadowed_by("", count, "const LIMIT: usize = 4;"),
        ),
        (
            "unrelated-struct-after-the-loop",
            shadowed_by("", count, "struct Lane;"),
        ),
        (
            "unrelated-fn-after-the-loop",
            shadowed_by("", count, "fn note() {}"),
        ),
        (
            "unrelated-use-after-the-loop",
            shadowed_by("", count, "use core::cmp::max;"),
        ),
        (
            "unrelated-static-after-a-slice-loop",
            slice_shadowed_by("static LIMIT: usize = 4;"),
        ),
    ];
    each_row(unrelated, |name, text| accepts(name, &text));
}

/// D2 (root Amendment 1, NEW-1): a unit struct that compares with `i32` and takes `-=` stands in
/// for an alias source as a `const` does. The loop below pops without bound, because the struct's
/// `!=` never says the count is zero and its `-=` does nothing. Red if the item scan reads only
/// `static`, `const` and `use`.
#[test]
fn unit_struct_shadows_the_alias_source() {
    let text = format!(
        "fn drain(control: &mut Consumer<Record>) {{\n    let available = control.available_at_entry();\n    {{\n        struct available;\n        impl PartialEq<i32> for available {{\n            fn eq(&self, _: &i32) -> bool {{\n                false\n            }}\n        }}\n        impl core::ops::SubAssign<i32> for available {{\n            fn sub_assign(&mut self, _: i32) {{}}\n        }}\n        let mut remaining = available;\n        while remaining != 0 {{\n            remaining -= 1;\n            {POP}\n        }}\n    }}\n}}"
    );
    refuses("unit-struct-shadows-the-alias-source", &text);
}

/// A unit struct named `controls`, whose `iter_mut` repeats, and a counted drain in each pass of a
/// loop over `controls`: the attempt-2 verdict's `r01` (#1441), which pops without end.
const REPEATING_ROOT: &str = "struct controls;\n        impl controls {\n            fn iter_mut(&self) -> core::iter::Repeat<u8> {\n                core::iter::repeat(0)\n            }\n        }\n";

/// A drain whose outer loop is `header` over the collection `controls`, destructured from `state`
/// (a `let` binding the root), with `item` first in an inner block.
fn over_a_root(item: &str, header: &str) -> String {
    format!(
        "fn drain(state: &mut State) {{\n    let State {{ controls, .. }} = state;\n    {{\n        {item}        {header} {{\n            let available = control.available_at_entry();\n            for _ in 0..available {{\n                {POP}\n            }}\n        }}\n    }}\n}}"
    )
}

/// D9 (root Amendment 3, SP-2): the collection root of an outer `for` is a read like the count. An
/// item of the region that declares its name, of any kind or a `use`, redirects the loop: a unit
/// struct with an `iter_mut` that repeats turns a finite outer loop into an endless one around a
/// counted drain. Red if the item rule reads only the count, its alias source and the slice
/// parameter, and not the root of an outer loop's collection.
#[test]
fn an_item_that_declares_the_collection_root_refuses_the_drain() {
    let each = "for control in controls.iter_mut()";
    let rows: Vec<(&'static str, String)> = vec![
        // The attempt-2 verdict's r01.
        (
            "unit-struct-shadows-the-collection-root",
            over_a_root(REPEATING_ROOT, each),
        ),
        (
            "collection-root-shadowed-by-a-const",
            over_a_root(
                "const controls: core::ops::RangeFrom<u64> = 0..;\n        ",
                "for control in &controls",
            ),
        ),
        // The attempt-2 verdict's b06: a slice parameter iterated with `.iter()` and shadowed by a
        // written struct after the loop.
        (
            "slice-parameter-collection-root-shadowed-by-a-struct",
            format!(
                "fn drain(lanes: &mut [Consumer<Record>]) {{\n    for control in lanes.iter() {{\n        let available = control.available_at_entry();\n        for _ in 0..available {{\n            {POP}\n        }}\n    }}\n    struct lanes;\n    impl lanes {{\n        fn iter(&self) -> core::iter::Repeat<u8> {{\n            core::iter::repeat(0)\n        }}\n    }}\n}}"
            ),
        ),
        (
            "parameter-collection-root-shadowed-by-a-struct",
            format!(
                "fn drain(cfg: &Config) {{\n    for control in &cfg.controls {{\n        let available = control.available_at_entry();\n        for _ in 0..available {{\n            {POP}\n        }}\n    }}\n    struct cfg;\n}}"
            ),
        ),
        (
            "pattern-bound-collection-root-shadowed-by-a-struct",
            format!(
                "fn drain(groups: &mut [Group]) {{\n    for group in groups {{\n        for control in &mut group.controls {{\n            let available = control.available_at_entry();\n            for _ in 0..available {{\n                {POP}\n            }}\n        }}\n        struct group;\n    }}\n}}"
            ),
        ),
    ];
    each_row(rows, |name, text| refuses(name, &text));
}

/// D9: the rule refuses the shadow and only the shadow. The same drains pass when no item names the
/// root, when the item names something else, and for a root that is a pattern or `self`.
#[test]
fn a_collection_root_that_no_item_names_passes() {
    let each = "for control in controls.iter_mut()";
    let rows: Vec<(&'static str, String)> = vec![
        // The attempt-2 verdict's r03.
        ("collection-root-destructured", over_a_root("", each)),
        (
            "collection-root-with-an-unrelated-item",
            over_a_root("struct Lane;\n        ", each),
        ),
        (
            "collection-root-of-a-pattern",
            format!(
                "fn drain(groups: &mut [Group]) {{\n    for group in groups {{\n        for control in &mut group.controls {{\n            let available = control.available_at_entry();\n            for _ in 0..available {{\n                {POP}\n            }}\n        }}\n    }}\n}}"
            ),
        ),
        (
            "collection-root-self",
            format!(
                "impl Strip {{\n    fn drain(&mut self) {{\n        for control in self.controls.iter_mut() {{\n            let available = control.available_at_entry();\n            for _ in 0..available {{\n                {POP}\n            }}\n        }}\n        struct Unrelated;\n    }}\n}}"
            ),
        ),
    ];
    each_row(rows, |name, text| accepts(name, &text));
}

/// D2: a count binding under a `cfg`-family attribute, on the entry binding or on the one alias
/// of it, may not be compiled; any other attribute is no reason to refuse.
#[test]
fn a_count_binding_that_may_not_be_compiled_is_refused() {
    let drain = |binding: &str| {
        format!(
            "fn drain(control: &mut Consumer<Record>) {{\n{binding}    for _ in 0..available {{\n        {POP}\n    }}\n}}"
        )
    };
    let refused: Vec<(&'static str, String)> = vec![
        (
            "count-under-cfg-attr",
            drain(
                "    let available = usize::MAX;\n    #[cfg_attr(debug_assertions, allow(unused))]\n    let available = control.available_at_entry();\n",
            ),
        ),
        (
            "count-under-a-path-ending-in-cfg",
            drain(
                "    let available = usize::MAX;\n    #[core::cfg(debug_assertions)]\n    let available = control.available_at_entry();\n",
            ),
        ),
        (
            "count-under-cfg-after-another-attribute",
            drain(
                "    let available = usize::MAX;\n    #[allow(unused)]\n    #[cfg(debug_assertions)]\n    let available = control.available_at_entry();\n",
            ),
        ),
        (
            "alias-source-under-cfg",
            drain(
                "    #[cfg(debug_assertions)]\n    let entry = control.available_at_entry();\n    let available = entry;\n",
            ),
        ),
    ];
    each_row(refused, |name, text| refuses(name, &text));
    let accepted: Vec<(&'static str, String)> = vec![
        (
            "count-under-allow",
            drain("    #[allow(unused)]\n    let available = control.available_at_entry();\n"),
        ),
        (
            "alias-source-under-allow",
            drain(
                "    #[allow(unused)]\n    let entry = control.available_at_entry();\n    let available = entry;\n",
            ),
        ),
    ];
    each_row(accepted, |name, text| accepts(name, &text));
}

/// D3: `from_fn`, `repeat_with` and `successors` run a closure repeatedly with no loop keyword.
/// A region refuses them anywhere, a pop or not, except `core::array::from_fn` and
/// `std::array::from_fn`, whose token is allowed. (A pop in that closure is refused as any pop in
/// a closure is: see `a_drain_in_the_closure_of_array_from_fn_is_refused`.)
#[test]
fn a_constructor_that_loops_is_refused_anywhere_in_a_region() {
    let rows: Vec<(&'static str, &'static str)> = vec![
        (
            "successors-without-a-pop",
            "fn build() {\n    let _ = core::iter::successors(Some(1_u32), |n| n.checked_mul(2));\n}",
        ),
        (
            "from-fn-without-a-pop",
            "fn build() {\n    let _ = core::iter::from_fn(|| None::<u8>);\n}",
        ),
        (
            "repeat-with-without-a-pop",
            "fn build() {\n    let _ = core::iter::repeat_with(|| 0_u8);\n}",
        ),
        (
            "raw-identifier-from-fn",
            "fn build() {\n    let _ = core::iter::r#from_fn(|| None::<u8>);\n}",
        ),
        (
            "array-from-fn-after-a-use",
            "fn build() {\n    let _: [u8; 2] = array::from_fn(|_| 0);\n}",
        ),
        (
            "array-from-fn-after-a-field-named-std",
            "fn build() {\n    let _ = S { std: &array::from_fn(|_| 0) };\n}",
        ),
        (
            "a-longer-path-ending-in-array-from-fn",
            "fn build() {\n    let _: [u8; 2] = other::core::array::from_fn(|_| 0);\n}",
        ),
        (
            "array-from-fn-with-another-root",
            "fn build() {\n    let _: [u8; 2] = alloc::array::from_fn(|_| 0);\n}",
        ),
        (
            "from-fn-in-a-macro-call",
            "fn build() {\n    note!(core::iter::from_fn);\n}",
        ),
        ("a-function-named-from-fn", "fn from_fn() {}"),
    ];
    each_row(rows, refuses);
    let accepted: Vec<(&'static str, &'static str)> = vec![
        (
            "core-array-from-fn-without-a-pop",
            "fn build() {\n    let _: [u8; 2] = core::array::from_fn(|i| 0);\n}",
        ),
        (
            "std-array-from-fn-without-a-pop",
            "fn build() {\n    let _: [u8; 2] = std::array::from_fn(|i| 0);\n}",
        ),
        (
            "rooted-array-from-fn-without-a-pop",
            "fn build() {\n    let _: [u8; 2] = ::core::array::from_fn(|i| 0);\n}",
        ),
        (
            "rooted-std-array-from-fn",
            "fn build() {\n    let _: [u8; 2] = (1, ::std::array::from_fn(|i| 0)).1;\n}",
        ),
    ];
    each_row(accepted, accepts);
    // Outside every region the constructors are no concern of this rule.
    let outside = "fn unmarked() {\n    let _ = core::iter::from_fn(|| None::<u8>);\n    let _ = core::iter::successors(Some(1_u32), |n| n.checked_mul(2));\n}\n";
    expect_valid(
        "constructors-outside-every-region",
        &[Edit::Append {
            path: BUFFER,
            text: outside,
        }],
    );
}
