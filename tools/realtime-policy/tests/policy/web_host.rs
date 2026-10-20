//! The browser host rules that keep the wrap rule's file the file that ships (#1496 D2
//! Amendment 1): (a) conditional compilation, (b) `#[export_name]`, (c) `#[path]`, (d) glob `use`.

use realtime_policy::Class;

use super::common::{Edit, Tree, assert_classes, expect_failure, expect_valid};

const FFI: &str = "hosts/host-web/src/ffi.rs";
const LIB: &str = "hosts/host-web/src/lib.rs";
const RENDER_LOCK: &str = "hosts/host-web/src/render_lock.rs";
const IMPORT: &str = "use crate::render_lock::{render_allocation_count, render_locked};";
const RENDER_FN: &str = "pub extern \"C\" fn miso_engine_web_v1_render(handle: u32) -> u32 {";
const SEEK_FN: &str = "pub extern \"C\" fn miso_engine_web_v1_source_seek(handle: u32) -> u32 {";
const DISPOSE_FN: &str = "pub extern \"C\" fn miso_engine_web_v1_dispose(handle: u32) -> u32 {";
const FAKE: &str = "pub(crate) mod fake {\n    pub(crate) fn render_locked<R>(f: impl FnOnce() -> R) -> R {\n        f()\n    }\n}\n";

fn replace(path: &'static str, old: &'static str, new: &str) -> Edit<'static> {
    Edit::ReplaceLine {
        path,
        old,
        new: Box::leak(new.to_owned().into_boxed_str()),
    }
}

fn append(path: &'static str, text: &str) -> Edit<'static> {
    Edit::Append {
        path,
        text: Box::leak(text.to_owned().into_boxed_str()),
    }
}

/// The cfg-split twin of the verifier's reproducer: the real wrapped `source_seek` is kept under
/// `cfg(not(wasm))` and the wasm build exports an unwrapped copy through `export_name`.
#[test]
fn a_cfg_split_twin_with_an_export_name_is_refused() {
    let twin = "#[cfg(target_family = \"wasm\")]\n#[unsafe(export_name = \"miso_engine_web_v1_source_seek\")]\npub extern \"C\" fn seek_wasm(handle: u32) -> u32 {\n    handle\n}\n";
    let report = expect_failure(
        "web-cfg-split-twin",
        &[
            replace(
                FFI,
                SEEK_FN,
                &format!("#[cfg(not(target_family = \"wasm\"))]\n{SEEK_FN}"),
            ),
            append(FFI, twin),
        ],
        &[Class::ConditionalWrapPath, Class::ExportNameOnWebExport],
    );
    let [cfg, name] = report.findings.as_slice() else {
        panic!("two findings expected: {:#?}", report.findings);
    };
    assert_eq!(
        cfg.export.as_deref(),
        Some("miso_engine_web_v1_source_seek")
    );
    assert_eq!(name.class, Class::ExportNameOnWebExport);
    assert!(name.line.is_some_and(|line| line > cfg.line.unwrap()));
}

/// Rule (a): `#[cfg]` or `#[cfg_attr]` on any guarded item is a finding, wherever it sits.
#[test]
fn conditional_compilation_on_a_guarded_item_is_refused() {
    let cfg = "#[cfg(not(any()))]";
    let cfg_attr = "#[cfg_attr(target_family = \"wasm\", inline)]";
    let cases: [(&str, Vec<Edit>); 8] = [
        (
            "import",
            vec![replace(FFI, IMPORT, &format!("{cfg}\n{IMPORT}"))],
        ),
        (
            "wrapped-export",
            vec![replace(FFI, RENDER_FN, &format!("{cfg}\n{RENDER_FN}"))],
        ),
        (
            "wrapped-export-cfg-attr",
            vec![replace(FFI, RENDER_FN, &format!("{cfg_attr}\n{RENDER_FN}"))],
        ),
        (
            "unwrapped-export",
            vec![replace(FFI, DISPOSE_FN, &format!("{cfg}\n{DISPOSE_FN}"))],
        ),
        (
            "wrap-call",
            vec![replace(
                FFI,
                "    render_locked(|| handle)",
                &format!("    {cfg}\n    render_locked(|| handle)"),
            )],
        ),
        (
            "mod-ffi",
            vec![replace(LIB, "mod ffi;", &format!("{cfg}\nmod ffi;"))],
        ),
        (
            "mod-render-lock",
            vec![replace(
                LIB,
                "mod render_lock;",
                &format!("{cfg_attr}\nmod render_lock;"),
            )],
        ),
        (
            "inner-attribute",
            vec![replace(
                FFI,
                "#![allow(unsafe_code)]",
                "#![allow(unsafe_code)]\n#![cfg(not(any()))]",
            )],
        ),
    ];
    for (name, edits) in &cases {
        let report = expect_failure(
            &format!("web-cfg-{name}"),
            edits,
            &[Class::ConditionalWrapPath],
        );
        assert_eq!(report.findings.len(), 1, "{name}");
    }
    // The verifier's second form: the import is gated off and a glob supplies a pass-through.
    expect_failure(
        "web-cfg-import-plus-glob",
        &[
            replace(FFI, IMPORT, &format!("#[cfg(any())]\n{IMPORT}")),
            append(LIB, FAKE),
            append(FFI, "use crate::fake::*;\n"),
        ],
        &[Class::ConditionalWrapPath, Class::GlobInWrapModule],
    );
    // An item the rule does not guard may be conditional: the test module of the base tree.
    expect_valid(
        "web-cfg-elsewhere",
        &[append(
            FFI,
            "#[cfg(test)]\nfn only_in_tests() {}\n#[cfg(not(target_family = \"wasm\"))]\nfn only_native() {}\n",
        )],
    );
}

/// Rule (b): an `#[export_name]` with the table prefix is refused in any spelling and any file,
/// and one without the prefix is not.
#[test]
fn an_export_name_with_the_table_prefix_is_refused_and_another_is_not() {
    let spellings = [
        "#[export_name = \"miso_engine_web_v1_x\"]",
        "#[unsafe(export_name = \"miso_engine_web_v1_x\")]",
        "#[cfg_attr(target_family = \"wasm\", unsafe(export_name = \"miso_engine_web_v1_x\"))]",
    ];
    for (index, spelling) in spellings.iter().enumerate() {
        let item = format!(
            "{spelling}\npub extern \"C\" fn other(handle: u32) -> u32 {{\n    handle\n}}\n"
        );
        // In ffi.rs, and in a file other than the two the wrap rule reads.
        for (place, edits) in [
            ("ffi", vec![append(FFI, &item)]),
            (
                "other-file",
                vec![Edit::Create {
                    path: "hosts/host-web/src/other.rs",
                    text: Box::leak(item.clone().into_boxed_str()),
                }],
            ),
            (
                "nested-module",
                vec![append(FFI, &format!("mod nested {{\n{item}}}\n"))],
            ),
        ] {
            let report = expect_failure(
                &format!("web-export-name-{index}-{place}"),
                &edits,
                &[Class::ExportNameOnWebExport],
            );
            assert_eq!(report.findings.len(), 1, "{spelling} in {place}");
        }
    }
    expect_valid(
        "web-export-name-other",
        &[append(
            FFI,
            "#[unsafe(export_name = \"memory_probe\")]\npub extern \"C\" fn other(handle: u32) -> u32 {\n    handle\n}\n",
        )],
    );
}

/// Rule (c): `#[path]` on either guarded module declaration; and a declaration the rule cannot
/// read as an out-of-line module.
#[test]
fn a_path_on_a_guarded_module_or_an_inline_module_is_refused() {
    for (name, module) in [("ffi", "mod ffi;"), ("render-lock", "mod render_lock;")] {
        let report = expect_failure(
            &format!("web-path-{name}"),
            &[replace(
                LIB,
                module,
                &format!("#[path = \"other.rs\"]\n{module}"),
            )],
            &[Class::PathOnWebModule],
        );
        assert_eq!(report.findings.len(), 1, "{name}");
    }
    expect_failure(
        "web-inline-ffi",
        &[replace(LIB, "mod ffi;", "mod ffi {}")],
        &[Class::WebModuleNotDeclared],
    );
    expect_failure(
        "web-undeclared-render-lock",
        &[replace(LIB, "mod render_lock;", "")],
        &[Class::WebModuleNotDeclared],
    );
}

/// A listed export must be a top-level item of `ffi.rs`: moved into a child module (where a glob
/// is not refused) it is missing, not accepted.
#[test]
fn an_export_moved_into_a_child_module_is_missing() {
    let tree = Tree::base("web-export-moved");
    let text = tree.read(FFI);
    let moved = text.replacen(
        "#[unsafe(no_mangle)]\npub extern \"C\" fn miso_engine_web_v1_render(handle: u32) -> u32 {\n    render_locked(|| handle)\n}\n",
        "mod inner {\n    use super::*;\n    #[unsafe(no_mangle)]\n    pub extern \"C\" fn miso_engine_web_v1_render(handle: u32) -> u32 {\n        render_locked(|| handle)\n    }\n}\n",
        1,
    );
    assert_ne!(text, moved);
    tree.write(FFI, moved.as_bytes());
    assert_classes(&tree, &[Class::ExportMissing]);
}

/// The files of rules (a) and (c) must exist.
#[test]
fn a_missing_host_module_file_is_a_finding() {
    expect_failure(
        "web-no-render-lock",
        &[Edit::Delete { path: RENDER_LOCK }],
        &[Class::MissingWebSource],
    );
}
