//! The per-queue counted pop (D8b): the meter poll's twin passes, and each refusal breaks one of
//! conditions 1, 2, 4, 5 or 6 of the form and nothing else.

use realtime_policy::Class;

use super::super::common::{Edit, expect_failure, expect_valid};
use super::{ANCHOR, BUILTINS_COMPILER};

const DRAIN: Class = Class::UnboundedTryPop;

/// The text of a poll function: the places where a case differs from the passing form.
struct Poll {
    /// The root of the queues and the counters: `self`, or a `ready` parameter.
    root: &'static str,
    /// The set loop.
    set: String,
    /// Statements between the set loop and the sum loop.
    between: String,
    /// The pass loop's pattern and iterator.
    pass_header: String,
    /// The pass loop's guard, decrement and counted pop.
    pass_body: String,
    /// The statements after the pass loop, inside the outer loop.
    after: String,
}

impl Poll {
    /// The meter poll's shape: a set loop, the sum of the counts, `for _ in 0..passes` around a
    /// pass loop over three zipped collections, and checks that read the collections, write
    /// another field, `continue`, `break`, or reset and `return`.
    fn passing(root: &'static str) -> Self {
        Self {
            root,
            set: "for (lane, left) in R.lanes.iter().zip(&mut R.remaining) {\n    *left = lane.consumer.available_at_entry().min(DEPTH);\n}".to_owned(),
            between: String::new(),
            pass_header: "((lane, slot), left) in R\n    .lanes\n    .iter_mut()\n    .zip(&mut R.slots)\n    .zip(&mut R.remaining)".to_owned(),
            pass_body: "if slot.is_some() || *left == 0 {\n    continue;\n}\n*left -= 1;\nif let Ok(record) = lane.consumer.try_pop() {\n    *slot = Some(record);\n}".to_owned(),
            after: "if R.slots.iter().any(Option::is_none) {\n    break;\n}\nlet aligned = R.slots.iter().enumerate().all(|(index, slot)| {\n    slot.is_some() && R.lanes.get(index).is_some()\n});\nif !aligned {\n    R.lost = R.lost.saturating_add(1);\n    continue;\n}\nif R.lost > 9 {\n    R.reset_delivery(false);\n    return 0;\n}".to_owned(),
        }
    }

    fn render(&self) -> String {
        let signature = if self.root == "self" {
            "    fn poll(&mut self) -> u32 {"
        } else {
            "fn poll(ready: &mut Ready) -> u32 {"
        };
        let body = format!(
            "{set}\n{between}\nlet mut passes = 1_usize;\nfor left in R.remaining.iter() {{\n    passes = passes.saturating_add(*left as usize);\n}}\nfor _ in 0..passes {{\n    for {pass_header} {{\n{pass_body}\n    }}\n{after}\n}}\n0",
            set = self.set,
            between = self.between,
            pass_header = self.pass_header,
            pass_body = indent(&self.pass_body, 8),
            after = indent(&self.after, 4),
        )
        .replace("R.", &format!("{}.", self.root))
        .replace("R\n", &format!("{}\n", self.root));
        let text = format!("{signature}\n{}\n}}", indent(&body, 4));
        if self.root == "self" {
            format!("impl Meters {{\n{text}\n}}")
        } else {
            text
        }
    }
}

fn indent(text: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    text.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{pad}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn edit(text: &str) -> [Edit<'_>; 1] {
    [Edit::ReplaceLine {
        path: BUILTINS_COMPILER,
        old: ANCHOR,
        new: text,
    }]
}

fn accepts(name: &str, poll: &Poll) {
    expect_valid(name, &edit(&poll.render()));
}

fn refuses(name: &str, poll: &Poll) {
    expect_failure(name, &edit(&poll.render()), &[DRAIN]);
}

#[test]
fn per_queue_meter_poll() {
    accepts("per-queue-meter-poll", &Poll::passing("self"));
}

#[test]
fn per_queue_single_pass() {
    // No outer loop: the form is the set loop and the pass loop.
    let text = "impl Meters {\n    fn poll(&mut self) {\n        for (lane, left) in self.lanes.iter().zip(&mut self.remaining) {\n            *left = lane.consumer.available_at_entry();\n        }\n        for (lane, left) in self.lanes.iter_mut().zip(&mut self.remaining) {\n            if *left == 0 {\n                continue;\n            }\n            *left -= 1;\n            if let Ok(record) = lane.consumer.try_pop() {\n                apply(record);\n            }\n        }\n    }\n}";
    expect_valid("per-queue-single-pass", &edit(text));
}

/// The roots are a typed parameter, not `self`.
#[test]
fn per_queue_param_root() {
    accepts("per-queue-param-root", &Poll::passing("ready"));
}

/// Condition 1: the roots are spelled alike but bound differently. Defect: the set loop's and the
/// pass loop's roots compared by spelling.
#[test]
fn per_queue_root_rebound() {
    let mut poll = Poll::passing("ready");
    poll.between = "let ready = spare;".to_owned();
    poll.after = String::new();
    let text = poll.render().replace(
        "fn poll(ready: &mut Ready)",
        "fn poll(ready: &mut Ready, spare: &mut Ready)",
    );
    expect_failure("per-queue-root-rebound", &edit(&text), &[DRAIN]);
}

/// Condition 2: the counts are not read at entry. Defect: any set-loop initializer accepted.
#[test]
fn per_queue_count_not_from_entry() {
    let mut poll = Poll::passing("self");
    poll.set = "for (lane, left) in R.lanes.iter().zip(&mut R.remaining) {
    *left = lane.consumer.capacity();
}"
    .to_owned();
    refuses("per-queue-count-not-from-entry", &poll);
}

/// Condition 2: the set loop adjusts a count after reading it. Defect: a set loop judged by its
/// first statement only.
#[test]
fn per_queue_set_loop_adjusts_count() {
    let mut poll = Poll::passing("self");
    poll.set = "for (lane, left) in R.lanes.iter().zip(&mut R.remaining) {
    *left = lane.consumer.available_at_entry();
    *left += 4;
}"
    .to_owned();
    refuses("per-queue-set-loop-adjusts-count", &poll);
}

/// Condition 4: queue i is guarded by counter i + 1. Defect: adapters in the zip headers ignored.
#[test]
fn per_queue_counters_misaligned() {
    let mut poll = Poll::passing("self");
    poll.pass_header =
        "(lane, left) in R.lanes.iter_mut().zip(R.remaining.iter_mut().skip(1))".to_owned();
    poll.pass_body = "if *left == 0 {
    continue;
}
*left -= 1;
if let Ok(record) = lane.consumer.try_pop() {
    apply(record);
}"
    .to_owned();
    refuses("per-queue-counters-misaligned", &poll);
}

/// Condition 4: the counted queue is the element of a third collection. Defect: any binding of
/// the pass pattern taken as the counted queue.
#[test]
fn per_queue_pop_from_wrong_zip_position() {
    let mut poll = Poll::passing("self");
    poll.pass_header = "((lane, other), left) in R
    .lanes
    .iter_mut()
    .zip(&mut R.others)
    .zip(&mut R.remaining)"
        .to_owned();
    poll.pass_body = "if *left == 0 {
    continue;
}
*left -= 1;
if let Ok(record) = other.consumer.try_pop() {
    apply(record);
}"
    .to_owned();
    poll.after = String::new();
    refuses("per-queue-pop-from-wrong-zip-position", &poll);
}

/// Condition 5: a pop guarded by another queue's count. Defect: the pop's receiver path not
/// compared with the set loop's.
#[test]
fn per_queue_pop_other_queue() {
    let mut poll = Poll::passing("self");
    poll.pass_body = poll
        .pass_body
        .replace("lane.consumer.try_pop()", "lane.spare.try_pop()");
    refuses("per-queue-pop-other-queue", &poll);
}

/// Condition 5: a guard with no decrement. Defect: the decrement not required.
#[test]
fn per_queue_no_decrement() {
    let mut poll = Poll::passing("self");
    poll.pass_body = poll.pass_body.replace("*left -= 1;\n", "apply_all();\n");
    refuses("per-queue-no-decrement", &poll);
}

/// Condition 5: two pops after one decrement, both in the third statement. Defect: more than one
/// pop per pass body accepted.
#[test]
fn per_queue_two_pops_one_decrement() {
    let mut poll = Poll::passing("self");
    poll.pass_body = poll.pass_body.replace(
        "if let Ok(record) = lane.consumer.try_pop() {\n    *slot = Some(record);\n}",
        "if let Ok(record) = lane.consumer.try_pop() {\n    *slot = Some(record);\n} else if let Ok(extra) = lane.consumer.try_pop() {\n    apply(extra);\n}",
    );
    refuses("per-queue-two-pops-one-decrement", &poll);
}

/// Condition 5: the receiver is spelled `lane.consumer`, but the block's `let` binds it. Defect:
/// receivers compared by spelling, not by binding.
#[test]
fn per_queue_receiver_rebound() {
    let mut poll = Poll::passing("self");
    poll.pass_body = "if slot.is_some() || *left == 0 {\n    continue;\n}\n*left -= 1;\n{\n    let lane = &mut R.spare;\n    if let Ok(record) = lane.consumer.try_pop() {\n        *slot = Some(record);\n    }\n}".to_owned();
    refuses("per-queue-receiver-rebound", &poll);
}

/// Condition 5: the counted queue is swapped for the spare after the pop. Defect: the queue
/// binding checked only in the pop's receiver.
#[test]
fn per_queue_element_swapped_with_spare() {
    let mut poll = Poll::passing("self");
    poll.pass_body
        .push_str("\ncore::mem::swap(&mut lane.consumer, &mut R.spare);");
    refuses("per-queue-element-swapped-with-spare", &poll);
}

/// Condition 6: a second set loop after the pass loop re-reads every count each pass. Defect: the
/// window cut short at the end of the pass loop.
#[test]
fn per_queue_count_reread_in_pass() {
    let mut poll = Poll::passing("self");
    poll.after = "for (lane, left) in R.lanes.iter().zip(&mut R.remaining) {\n    *left = lane.consumer.available_at_entry();\n}".to_owned();
    refuses("per-queue-count-reread-in-pass", &poll);
}

/// Condition 6: the queues' collection is swapped between the loops. Defect: item 1 read as "any
/// path that leaves `<K>`", so `<Q>` can be replaced in the window.
#[test]
fn per_queue_collection_swapped() {
    let mut poll = Poll::passing("self");
    poll.between = "core::mem::swap(&mut R.lanes, &mut R.spare_lanes);".to_owned();
    refuses("per-queue-collection-swapped", &poll);
}

/// Condition 6: a call that rewrites the counters, with no `return` after it. Defect: the
/// terminal-call exemption granted without its tail checked.
#[test]
fn per_queue_counter_written_by_call() {
    let mut poll = Poll::passing("self");
    poll.between = "R.refill_counters();".to_owned();
    refuses("per-queue-counter-written-by-call", &poll);
}

/// Condition 6: the `return` that follows the call leaves a closure, not the form. Defect: the
/// terminal-call exemption applied inside a closure.
#[test]
fn per_queue_counters_refilled_in_closure() {
    let mut poll = Poll::passing("ready");
    poll.after =
        "let mut refill = || -> u32 {\n    ready.refill_counters();\n    return 0;\n};\nrefill();"
            .to_owned();
    refuses("per-queue-counters-refilled-in-closure", &poll);
}

/// Condition 6: a reborrow of the root writes the counters. Defect: a reborrowed root not read as
/// a mention of `<K>`.
#[test]
fn per_queue_root_reborrowed() {
    let mut poll = Poll::passing("ready");
    poll.after = "let me = &mut *ready;\nfor left in me.remaining.iter_mut() {\n    *left = 8;\n}"
        .to_owned();
    refuses("per-queue-root-reborrowed", &poll);
}

/// Condition 6: a local macro writes the counters. Defect: macro token trees skipped.
#[test]
fn per_queue_counters_written_by_macro() {
    let mut poll = Poll::passing("ready");
    poll.after = "macro_rules! refill {\n    ($r:expr) => {\n        for left in $r.remaining.iter_mut() {\n            *left = 8;\n        }\n    };\n}\nrefill!(ready);".to_owned();
    refuses("per-queue-counters-written-by-macro", &poll);
}
