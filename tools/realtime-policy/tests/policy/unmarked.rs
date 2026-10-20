//! Every non-test pop is in a marked region or in a function on the control-side allowlist
//! (#1443 D1 to D3): test code, the allowlist, the token census and the helper outside a region.
//!
//! Each case adds files under `crates/pops` to the base tree, which has no pop outside a region.

use realtime_policy::Class;

use super::common::{BEGIN, END, Edit, Tree, entry};

const LIB: &str = "crates/pops/src/lib.rs";
const MANIFEST: &str = "crates/pops/Cargo.toml";
const UNMARKED: Class = Class::UnmarkedTryPop;
const STALE: Class = Class::StaleAllowlistEntry;
const CENSUS: Class = Class::UnclassifiedTryPop;

fn create<'a>(path: &'a str, text: &'a str) -> Edit<'a> {
    Edit::Create { path, text }
}

/// The base tree plus `edits`, checked with no allowlist entry, must have the classes `expected`.
fn classes(name: &str, edits: &[Edit], expected: &[Class]) {
    entries(name, edits, Vec::new(), expected);
}

/// The same with the control-side allowlist `list`.
fn entries(
    name: &str,
    edits: &[Edit],
    list: Vec<realtime_policy::ControlSideEntry>,
    expected: &[Class],
) {
    let tree = Tree::base(name);
    tree.apply(edits);
    let report = tree.check_with(list);
    let found: std::collections::BTreeSet<Class> = report.classes();
    let expected: std::collections::BTreeSet<Class> = expected.iter().copied().collect();
    assert_eq!(found, expected, "case {name}: {:#?}", report.findings);
}

const POP: &str = "fn drain(c: &mut Consumer) {\n    while let Ok(r) = c.try_pop() {\n        apply(r);\n    }\n}\n";

// ---- D1: test code ----

#[test]
fn test_attributes_make_pops_test_code() {
    let text = format!(
        "#[cfg(test)]\n{POP}#[cfg(all(test, loom))]\nmod model {{\n    {POP}}}\nmod plain {{\n    #[test]\n    {POP}}}\n\
         fn stmt(c: &mut Consumer) {{\n    #[cfg(test)]\n    let _ = c.try_pop();\n    match 1 {{\n        #[cfg(test)]\n        1 => {{ c.try_pop(); }}\n        _ => {{}}\n    }}\n}}\n"
    );
    classes("test-attributes", &[create(LIB, &text)], &[]);
}

#[test]
fn only_a_false_cfg_is_test_code() {
    let rows = [
        ("not-test", format!("#[cfg(not(test))]\n{POP}")),
        (
            "module-named-tests",
            format!("pub mod tests {{\n    {POP}}}\n"),
        ),
        (
            "test-support",
            format!("#[cfg(any(test, feature = \"test-support\"))]\n{POP}"),
        ),
        (
            "not-feature",
            format!("#[cfg(not(feature = \"x\"))]\n{POP}"),
        ),
        (
            "unknown-atom",
            format!("#[cfg(all(unix, not(unix)))]\n{POP}"),
        ),
    ];
    for (name, text) in rows {
        classes(name, &[create(LIB, &text)], &[UNMARKED]);
    }
}

#[test]
fn a_file_that_no_declaration_reaches_is_not_test_code() {
    classes(
        "orphan-file",
        &[
            create(LIB, "mod known;\n"),
            create("crates/pops/src/known.rs", "fn quiet() {}\n"),
            create("crates/pops/src/orphan.rs", POP),
        ],
        &[UNMARKED],
    );
}

#[test]
fn a_test_module_file_is_test_code_only_through_its_cfg() {
    // Default lookup and `#[path]` under `#[cfg(test)]`: ignored.
    classes(
        "cfg-test-module-files",
        &[
            create(
                LIB,
                "#[cfg(test)]\nmod tests;\n#[cfg(test)]\n#[path = \"x_tests.rs\"]\nmod more;\n",
            ),
            create("crates/pops/src/tests.rs", POP),
            create("crates/pops/src/x_tests.rs", POP),
        ],
        &[],
    );
    // The same module without the cfg is not exempt by its name.
    classes(
        "module-tests-without-cfg",
        &[
            create(LIB, "mod tests;\n"),
            create("crates/pops/src/tests.rs", POP),
        ],
        &[UNMARKED],
    );
}

#[test]
fn a_tests_directory_inside_src_is_not_a_package_tests_directory() {
    classes(
        "src-foo-tests-bar",
        &[
            create(MANIFEST, "[package]\n"),
            create(LIB, "mod foo;\n"),
            create("crates/pops/src/foo/mod.rs", "mod tests;\n"),
            create("crates/pops/src/foo/tests/mod.rs", "mod bar;\n"),
            create("crates/pops/src/foo/tests/bar.rs", POP),
        ],
        &[UNMARKED],
    );
}

#[test]
fn a_file_under_a_package_tests_directory_is_test_code() {
    let integration = "crates/pops/tests/it.rs";
    classes(
        "package-tests-dir",
        &[create(MANIFEST, "[package]\n"), create(integration, POP)],
        &[],
    );
    classes(
        "no-package-manifest",
        &[create(integration, POP)],
        &[UNMARKED],
    );
    // A `#[path]` in a file outside every `<package>/tests/` directory reaches it as a module.
    classes(
        "package-tests-dir-reached-by-path",
        &[
            create(MANIFEST, "[package]\n"),
            create(integration, POP),
            create(
                "crates/pops/build.rs",
                "#[path = \"tests/it.rs\"]\nmod it;\n",
            ),
        ],
        &[UNMARKED],
    );
}

#[test]
fn every_declaration_must_be_test_code() {
    let shared = "crates/pops/src/shared.rs";
    classes(
        "all-declarations-test",
        &[
            create(
                LIB,
                "#[cfg(test)]\nmod one;\n#[cfg(test)]\n#[path = \"one.rs\"]\nmod two;\n",
            ),
            create("crates/pops/src/one.rs", POP),
        ],
        &[],
    );
    classes(
        "one-declaration-not-test",
        &[
            create(
                LIB,
                "#[cfg(test)]\nmod shared;\n#[path = \"shared.rs\"]\nmod again;\n",
            ),
            create(shared, POP),
        ],
        &[UNMARKED],
    );
    // A module that only test code declares is test code, and so is a file with an inner cfg.
    classes(
        "nested-and-inner",
        &[
            create(LIB, "#[cfg(test)]\nmod tests;\nmod gated;\n"),
            create("crates/pops/src/tests.rs", "mod helpers;\n"),
            create("crates/pops/src/tests/helpers.rs", POP),
            create("crates/pops/src/gated.rs", &format!("#![cfg(test)]\n{POP}")),
        ],
        &[],
    );
}

// ---- D3: the allowlist ----

const QUEUES: &str = "struct Queues;\nimpl Queues {\n    fn dequeue(&mut self, c: &mut Consumer) {\n        while let Ok(r) = c.try_pop() {\n            apply(r);\n        }\n    }\n}\n";

#[test]
fn an_entry_admits_one_function_only() {
    let text = format!(
        "{QUEUES}impl Queues {{\n    fn renamed(&mut self, c: &mut Consumer) {{\n        c.try_pop();\n    }}\n}}\n"
    );
    entries(
        "other-function",
        &[create(LIB, &text)],
        vec![entry(LIB, "Queues::dequeue", 1)],
        &[UNMARKED],
    );
}

#[test]
fn a_stale_entry_is_a_finding() {
    let two = QUEUES.replace("apply(r);", "apply(r);\n            c.try_pop();");
    entries(
        "one-pop-more",
        &[create(LIB, &two)],
        vec![entry(LIB, "Queues::dequeue", 1)],
        &[STALE],
    );
    let none = QUEUES.replace(
        "while let Ok(r) = c.try_pop()",
        "while let Ok(r) = c.next()",
    );
    entries(
        "no-longer-pops",
        &[create(LIB, &none)],
        vec![entry(LIB, "Queues::dequeue", 1)],
        &[STALE],
    );
    entries(
        "file-missing",
        &[create(LIB, "fn quiet() {}\n")],
        vec![entry("crates/pops/src/gone.rs", "Queues::dequeue", 1)],
        &[STALE],
    );
    entries(
        "two-functions-one-path",
        &[create(
            LIB,
            "#[cfg(unix)]\nfn dequeue(c: &mut Consumer) { c.try_pop(); }\n#[cfg(not(unix))]\nfn dequeue(c: &mut Consumer) { c.try_pop(); }\n",
        )],
        vec![entry(LIB, "dequeue", 1)],
        // An ambiguous path admits neither function.
        &[UNMARKED, STALE],
    );
    entries(
        "listed-twice",
        &[create(LIB, QUEUES)],
        vec![
            entry(LIB, "Queues::dequeue", 1),
            entry(LIB, "Queues::dequeue", 1),
        ],
        &[STALE],
    );
}

#[test]
fn a_closure_pop_in_a_listed_function_is_not_admitted() {
    let text = "fn dequeue(c: &mut Consumer) {\n    let _ = c.try_pop();\n    let later = || c.try_pop();\n    later();\n}\n";
    entries(
        "closure-pop",
        &[create(LIB, text)],
        vec![entry(LIB, "dequeue", 1)],
        &[UNMARKED],
    );
}

#[test]
fn a_trait_method_and_a_module_have_canonical_paths() {
    let text = "mod inner {\n    impl<T> Drop for Retirer<T> {\n        fn drop(&mut self) {\n            while let Ok(r) = self.q.try_pop() {\n                drop(r);\n            }\n        }\n    }\n}\n";
    entries(
        "trait-method-path",
        &[create(LIB, text)],
        vec![entry(LIB, "inner::Drop for Retirer::drop", 1)],
        &[],
    );
}

// ---- D2: the census ----

#[test]
fn a_try_pop_token_that_is_no_pop_is_a_finding() {
    classes(
        "attribute-token",
        &[create(LIB, "#[note(try_pop)]\nstruct S;\n")],
        // The trusted-name scan refuses the token as well.
        &[Class::UnboundedTryPop, CENSUS],
    );
    classes(
        "field-call",
        &[create(LIB, "fn f(s: &S) {\n    (s.try_pop)(1);\n}\n")],
        &[CENSUS],
    );
    classes(
        "definition-is-classified",
        &[create(LIB, "fn try_pop(queue: &mut Q) {}\n")],
        // Only the trusted-name scan refuses the definition; the census classifies it.
        &[Class::UnboundedTryPop],
    );
}

#[test]
fn a_file_with_a_pop_that_does_not_parse_is_a_finding() {
    classes(
        "parse-failure",
        &[create(
            LIB,
            "fn f(c: &mut Consumer) {\n    c.try_pop() + ;\n}\n",
        )],
        &[Class::ParseError],
    );
}

// ---- the helper outside every region ----

#[test]
fn a_helper_outside_every_region_is_refused() {
    let text = format!(
        "// {BEGIN}\nimpl Lane {{\n    fn drain(&mut self) {{\n        while let Some(record) = self.next_record() {{\n            apply(record);\n        }}\n    }}\n}}\n// {END}\nimpl Lane {{\n    fn next_record(&mut self) -> Option<Record> {{\n        self.control.try_pop().ok()\n    }}\n}}\n"
    );
    // B2b-2 adds the call-rule class: the caller loops over the helper.
    classes(
        "helper-outside",
        &[create(LIB, &text)],
        &[UNMARKED, Class::PoppingCall],
    );
}
