//! The call rule (#1444): inside a marked region, a call to a function that pops, in a loop or a
//! closure, a path value that names one, and an iterator that pops.
//!
//! Each case adds `crates/pops/src/lib.rs` to the base tree. `HELPER` is the `v1302/adv/a08`
//! helper: one pop, in no loop, which the drain rule accepts.

use realtime_policy::Class;

use super::common::{BEGIN, END, Edit, expect_failure, expect_valid};

const LIB: &str = "crates/pops/src/lib.rs";
const CALL: Class = Class::PoppingCall;
const VALUE: Class = Class::PoppingPathValue;
const ITERATOR: Class = Class::PoppingIterator;

/// The helper, with its `impl` header and footer.
const HELPER: &str = "impl Lane {\n    fn next_record(&mut self) -> Option<Record> {\n        self.control.try_pop().ok()\n    }\n}\n";

fn marked(body: &str) -> String {
    format!("// {BEGIN}\n{body}// {END}\n")
}

fn refuses(name: &str, text: &str, classes: &[Class]) {
    expect_failure(name, &[Edit::Create { path: LIB, text }], classes);
}

fn accepts(name: &str, text: &str) {
    expect_valid(name, &[Edit::Create { path: LIB, text }]);
}

/// A caller that runs `body` in a method of `Lane`.
fn caller(body: &str) -> String {
    format!("impl Lane {{\n    fn run(&mut self) {{\n{body}    }}\n}}\n")
}

const WHILE_CALL: &str = "        while let Some(record) = self.next_record() {\n            apply(record);\n        }\n";

// ---- D5: the a08 shape and its restructures ----

#[test]
fn a_helper_and_its_looping_caller_in_one_region_are_refused() {
    let text = marked(&format!("{HELPER}{}", caller(WHILE_CALL)));
    refuses("calls-a08", &text, &[CALL]);
}

#[test]
fn a_helper_in_its_own_region_and_a_caller_in_another_are_refused() {
    let text = format!("{}{}", marked(HELPER), marked(&caller(WHILE_CALL)));
    refuses("calls-two-regions", &text, &[CALL]);
}

#[test]
fn a_pop_moved_out_of_the_loop_is_accepted() {
    let body = "        let record = self.next_record();\n        for _ in 0..4 {\n            apply(&record);\n        }\n";
    accepts(
        "calls-moved-out",
        &marked(&format!("{HELPER}{}", caller(body))),
    );
}

#[test]
fn a_caller_loop_outside_every_region_is_accepted() {
    // The limit "a loop in an unmarked caller" (D4): the helper's pop is marked.
    let text = format!("{}{}", marked(HELPER), caller(WHILE_CALL));
    accepts("calls-caller-outside", &text);
}

#[test]
fn the_name_set_counts_features_as_on_and_leaves_test_code_out() {
    let sweeper = |attribute: &str| {
        format!(
            "impl Lane {{\n    {attribute}\n    fn sweep(&mut self) -> Option<Record> {{\n        self.control.try_pop().ok()\n    }}\n}}\n"
        )
    };
    let loops = "        for _ in 0..4 {\n            self.sweep();\n        }\n";
    let feature = format!(
        "{}{}",
        marked(&sweeper("#[cfg(feature = \"x\")]")),
        marked(&caller(loops))
    );
    refuses("calls-sweep-feature", &feature, &[CALL]);
    // A `#[cfg(test)]` helper is not in the set: no build reaches it.
    let test_only = format!("{}{}", sweeper("#[cfg(test)]"), marked(&caller(loops)));
    accepts("calls-sweep-test", &test_only);
}

#[test]
fn a_closure_that_calls_a_helper_is_refused_without_a_loop() {
    let body =
        "        let again = |lane: &mut Lane| lane.next_record();\n        let _ = again(self);\n";
    refuses(
        "calls-closure",
        &marked(&format!("{HELPER}{}", caller(body))),
        &[CALL],
    );
}

#[test]
fn a_call_outside_a_loop_and_a_closure_is_accepted() {
    let body = "        let _ = self.next_record();\n        let _ = Lane::next_record(self);\n";
    accepts("calls-plain", &marked(&format!("{HELPER}{}", caller(body))));
}

#[test]
fn the_iterator_expression_of_a_for_is_outside_its_loop() {
    let body = "        for record in self.batch() {\n            apply(record);\n        }\n";
    let batch = "impl Lane {\n    fn batch(&mut self) -> Batch {\n        let _ = self.control.try_pop();\n        self.pending()\n    }\n}\n";
    accepts(
        "calls-for-expression",
        &marked(&format!("{batch}{}", caller(body))),
    );
}

#[test]
fn a_call_in_a_macro_token_tree_is_refused_in_a_loop() {
    let body =
        "        for _ in 0..4 {\n            log!(\"{:?}\", self.next_record());\n        }\n";
    refuses(
        "calls-macro",
        &marked(&format!("{HELPER}{}", caller(body))),
        &[CALL],
    );
    // Outside a loop and a closure the same call is accepted.
    let once = "        log!(\"{:?}\", self.next_record());\n";
    accepts(
        "calls-macro-once",
        &marked(&format!("{HELPER}{}", caller(once))),
    );
}

#[test]
fn a_free_function_and_a_path_call_are_refused_in_a_loop() {
    let free =
        "fn next_record(lane: &mut Lane) -> Option<Record> {\n    lane.control.try_pop().ok()\n}\n";
    let body = "        for _ in 0..4 {\n            let _ = next_record(self);\n            let _ = Lane::next_record(self);\n        }\n";
    refuses(
        "calls-paths",
        &marked(&format!("{free}{}", caller(body))),
        &[CALL],
    );
}

// ---- D5: function values ----

#[test]
fn a_path_value_is_refused_wherever_it_stands() {
    let body = "        for _ in 0..4 {\n            let _ = self.items().map(Lane::next_record);\n        }\n";
    refuses(
        "calls-map-path",
        &marked(&format!("{HELPER}{}", caller(body))),
        &[VALUE],
    );
    // Not in a loop: still a value that can be called anywhere.
    let once = "        let _ = self.items().map(Lane::next_record);\n";
    refuses(
        "calls-map-path-once",
        &marked(&format!("{HELPER}{}", caller(once))),
        &[VALUE],
    );
}

#[test]
fn a_hoisted_function_value_is_refused_twice_in_one_region() {
    let body = "        let next = Self::next_record;\n        while let Some(record) = next(self) {\n            apply(record);\n        }\n";
    refuses(
        "calls-hoisted-in-region",
        &marked(&format!("{HELPER}{}", caller(body))),
        &[CALL, VALUE],
    );
}

/// The `let` stands above BEGIN, the loop inside the region.
fn hoisted_above(initializer: &str, local: &str) -> String {
    let body = format!(
        "        let {local} = {initializer};\n// {BEGIN}\n        while let Some(record) = {local}(self) {{\n            apply(record);\n        }}\n// {END}\n"
    );
    format!("{}{}", marked(HELPER), caller(&body))
}

#[test]
fn a_hoisted_value_taken_above_the_region_is_refused_when_called() {
    refuses(
        "calls-hoisted-above",
        &hoisted_above("Self::next_record", "next"),
        &[CALL],
    );
    refuses(
        "calls-hoisted-closure",
        &hoisted_above("|lane: &mut Lane| lane.next_record()", "next"),
        &[CALL],
    );
    refuses(
        "calls-hoisted-cast",
        &hoisted_above("Lane::next_record as fn(&mut Lane) -> Option<u32>", "cast"),
        &[CALL],
    );
    // A local copied from a hot local is hot.
    let body = format!(
        "        let first = Self::next_record;\n        let next = first;\n// {BEGIN}\n        while let Some(record) = next(self) {{\n            apply(record);\n        }}\n// {END}\n"
    );
    refuses(
        "calls-hoisted-copied",
        &format!("{}{}", marked(HELPER), caller(&body)),
        &[CALL],
    );
}

#[test]
fn a_local_that_shares_a_name_with_a_popping_function_is_exempt() {
    // The real tree's `meters` and `stage` shapes: a local, a pattern and a parameter.
    let helper = "impl Lane {\n    fn meters(&mut self) -> Option<Record> {\n        self.control.try_pop().ok()\n    }\n}\n";
    let body = "        let meters = self.items();\n        for item in meters {\n            consume(item);\n        }\n        if let Some(meters) = self.pending() {\n            consume(meters);\n        }\n";
    let function =
        "fn each(meters: &mut Items) {\n    for _ in 0..4 {\n        consume(meters);\n    }\n}\n";
    accepts(
        "calls-local-names",
        &marked(&format!("{helper}{}{function}", caller(body))),
    );
    // The shadow ends with its block: after it the name is the function again.
    let shadowed = "        {\n            let meters = 0;\n            consume(meters);\n        }\n        for _ in 0..4 {\n            let _ = self.items().map(Lane::meters);\n        }\n";
    refuses(
        "calls-shadow-ends",
        &marked(&format!("{helper}{}", caller(shadowed))),
        &[VALUE],
    );
}

#[test]
fn a_nested_function_sees_no_local_of_its_parent() {
    let body = "        let next = Self::next_record;\n        let _ = next;\n        fn inner(lane: &mut Lane) {\n            for _ in 0..4 {\n                let _ = Lane::next_record(lane);\n            }\n        }\n";
    refuses(
        "calls-nested-fn",
        &marked(&format!("{HELPER}{}", caller(body))),
        &[CALL, VALUE],
    );
}

// ---- D5: iterators ----

const ITER: &str = "impl Iterator for Lane {\n    type Item = Record;\n    fn next(&mut self) -> Option<Record> {\n        self.control.try_pop().ok()\n    }\n}\n";

#[test]
fn an_iterator_that_pops_is_refused() {
    let with_for = marked(&format!(
        "{ITER}{}",
        "fn each(lane: &mut Lane) {\n    for record in &mut *lane {\n        apply(record);\n    }\n}\n"
    ));
    refuses("calls-iterator-for", &with_for, &[ITERATOR]);
    let with_count = marked(&format!(
        "{ITER}{}",
        "fn count(lane: &mut Lane) -> usize {\n    lane.by_ref().count()\n}\n"
    ));
    refuses("calls-iterator-count", &with_count, &[ITERATOR]);
}

#[test]
fn every_iterator_trait_and_every_rename_of_one_is_refused() {
    for (name, header) in [
        ("double-ended", "impl DoubleEndedIterator for Lane"),
        ("into", "impl IntoIterator for Lane"),
        ("path", "impl core::iter::Iterator for Lane"),
        ("raw", "impl r#Iterator for Lane"),
    ] {
        let text = marked(&format!(
            "{header} {{\n    fn next_back(&mut self) -> Option<Record> {{\n        self.control.try_pop().ok()\n    }}\n}}\n"
        ));
        refuses(&format!("calls-trait-{name}"), &text, &[ITERATOR]);
    }
    // A `use` renames the trait: the impl names the new spelling, in another file.
    let tree = super::common::Tree::base("calls-trait-renamed");
    tree.apply(&[
        Edit::Create {
            path: "crates/pops/src/lib.rs",
            text: "mod alias;\nmod model;\n",
        },
        Edit::Create {
            path: "crates/pops/src/alias.rs",
            text: "use core::iter::Iterator as Walk;\n",
        },
        Edit::Create {
            path: "crates/pops/src/model.rs",
            text: &marked(
                "impl Walk for Lane {\n    fn next(&mut self) -> Option<Record> {\n        self.control.try_pop().ok()\n    }\n}\n",
            ),
        },
    ]);
    let report = tree.check();
    assert_eq!(
        report.classes(),
        [ITERATOR].into_iter().collect(),
        "{:#?}",
        report.findings
    );
}

#[test]
fn an_iterator_that_does_not_pop_and_a_pop_in_another_trait_are_accepted() {
    let quiet = "impl Iterator for Lane {\n    type Item = Record;\n    fn next(&mut self) -> Option<Record> {\n        self.items.pop()\n    }\n}\n";
    accepts("calls-iterator-quiet", &marked(quiet));
    let other = "impl Source for Lane {\n    fn next_record(&mut self) -> Option<Record> {\n        self.control.try_pop().ok()\n    }\n}\n";
    accepts("calls-other-trait", &marked(other));
}

// ---- D5: twins of the real drains ----

/// #1345 Amendment 1 D8: a pop-free loop that stages each lane's cells, then a drain-only
/// observation loop that names the queue in its binding, its count and its pop, and hands each
/// record to a sink that the lane does not reach.
#[test]
fn the_bank_drain_twin_is_accepted() {
    let text = "impl Stage {\n    fn drain(&mut self, first_sample: u64, snapshot: LiveSnapshot) -> Result<(), RenderError> {\n        let mut packed = 0_usize;\n        for (lane, channel) in self.lanes.iter_mut().enumerate() {\n            if let Some(channel) = channel.as_mut() {\n                let staged = channel.stage(&mut self.staging, first_sample, snapshot);\n                packed += staged.staged;\n            }\n            self.offsets[lane + 1] = packed as u32;\n        }\n        let mut unbound = 0_u64;\n        for (index, queue) in self.observe_queues.iter_mut() {\n            let mut available = queue.available_at_entry();\n            while available != 0 {\n                available -= 1;\n                let Ok(record) = queue.try_pop() else {\n                    break;\n                };\n                if !apply_observe(record, first_sample, self.observations.get_mut(*index)) {\n                    unbound = unbound.saturating_add(1);\n                }\n            }\n        }\n        self.unbound = self.unbound.saturating_add(unbound);\n        Ok(())\n    }\n}\n";
    accepts("calls-bank-drain-twin", &marked(text));
}
