//! The render-locked wrap rule (#1496 D2): each wrapped export of the render-thread table is one
//! `render_locked` call, resolved to `crate::render_lock::render_locked`.

use realtime_policy::{Class, check};

use super::common::{Edit, Tree, expect_failure, expect_valid, policy, render_thread_export};

const FFI: &str = "hosts/host-web/src/ffi.rs";
const IMPORT: &str = "use crate::render_lock::{render_allocation_count, render_locked};";

fn replace(old: &'static str, new: &'static str) -> Edit<'static> {
    Edit::ReplaceLine {
        path: FFI,
        old,
        new,
    }
}

/// The failure lines of `report` that name `export`, in report order.
fn naming(report: &realtime_policy::Report, export: &str) -> Vec<Class> {
    report
        .findings
        .iter()
        .filter(|finding| {
            finding.export.as_deref() == Some(&format!("miso_engine_web_v1_{export}"))
        })
        .map(|finding| finding.class)
        .collect()
}

/// The #1488 gate-2 shape: the export runs its body without the wrap.
#[test]
fn an_export_whose_body_lost_its_wrap_is_a_finding_naming_it() {
    let report = expect_failure(
        "wrap-removed",
        &[
            replace("    render_locked(|| {", "    {"),
            replace("    })", "    }"),
        ],
        &[Class::RenderLockedBody],
    );
    assert_eq!(naming(&report, "source_seek"), [Class::RenderLockedBody]);
    let [finding] = report.findings.as_slice() else {
        panic!("one finding expected: {:#?}", report.findings);
    };
    assert_eq!(finding.path.as_deref(), Some(FFI));
    assert!(
        finding
            .source
            .as_deref()
            .unwrap()
            .contains("miso_engine_web_v1_source_seek")
    );
}

/// A statement before the wrap, or after it, is not the whole body.
#[test]
fn a_statement_beside_the_wrap_is_a_finding_naming_the_export() {
    let before = expect_failure(
        "statement-before",
        &[replace(
            "    render_locked(move || handle)",
            "    let _ = handle + 1;\n    render_locked(move || handle)",
        )],
        &[Class::RenderLockedBody],
    );
    assert_eq!(
        naming(&before, "spectrum_select"),
        [Class::RenderLockedBody]
    );
    assert_eq!(before.findings.len(), 1);
    let after = expect_failure(
        "statement-after",
        &[replace(
            "    render_locked(|| handle)",
            "    render_locked(|| handle);\n    0",
        )],
        &[Class::RenderLockedBody],
    );
    assert_eq!(naming(&after, "render"), [Class::RenderLockedBody]);
    assert_eq!(after.findings.len(), 1);
}

/// The #1488 verdict's "D1 reverted" shape: a local `render_locked` that only calls the closure
/// takes the import's place. Every wrapped export is red.
#[test]
fn a_local_render_locked_in_place_of_the_import_turns_every_wrapped_export_red() {
    let report = expect_failure(
        "local-render-locked",
        &[replace(
            IMPORT,
            "use crate::render_lock::render_allocation_count;\nfn render_locked<R>(f: impl FnOnce() -> R) -> R {\n    f()\n}",
        )],
        &[
            Class::RenderLockedNotImported,
            Class::RenderLockedShadowed,
            Class::RenderLockedUnresolved,
        ],
    );
    for export in ["render", "source_seek", "spectrum_select"] {
        assert_eq!(
            naming(&report, export),
            [Class::RenderLockedUnresolved],
            "{export}"
        );
    }
}

/// A shadow elsewhere in the file reaches the bare callee too, whatever its kind.
#[test]
fn any_other_definition_of_the_name_is_a_shadow() {
    let shadows = [
        "macro_rules! render_locked {\n    () => {};\n}",
        "const render_locked: u32 = 0;",
        "mod render_locked {}",
        "fn other() {\n    let render_locked = 1;\n    let _ = render_locked;\n}",
        "fn other(render_locked: u32) -> u32 {\n    render_locked\n}",
        "use crate::elsewhere::lock as render_locked;",
    ];
    for (index, shadow) in shadows.iter().enumerate() {
        let report = expect_failure(
            &format!("shadow-{index}"),
            &[Edit::Append {
                path: FFI,
                text: &format!("{shadow}\n"),
            }],
            &[Class::RenderLockedShadowed, Class::RenderLockedUnresolved],
        );
        assert_eq!(naming(&report, "render"), [Class::RenderLockedUnresolved]);
    }
}

/// Importing the real function under another name does not import the name the exports call.
#[test]
fn a_renamed_import_is_not_the_import() {
    let report = expect_failure(
        "renamed-import",
        &[replace(
            IMPORT,
            "use crate::render_lock::render_allocation_count;\nuse crate::render_lock::render_locked as locked;",
        )],
        &[
            Class::RenderLockedNotImported,
            Class::RenderLockedUnresolved,
        ],
    );
    assert_eq!(
        naming(&report, "source_seek"),
        [Class::RenderLockedUnresolved]
    );
}

/// The full path needs no import and cannot be shadowed; it is the same call.
#[test]
fn the_full_path_is_a_wrap_and_the_import_may_be_a_separate_item() {
    expect_valid(
        "full-path",
        &[
            replace(
                "    render_locked(|| handle)",
                "    crate::render_lock::render_locked(|| handle)",
            ),
            replace(
                IMPORT,
                "use crate::render_lock::render_allocation_count;\nuse crate::render_lock::render_locked;",
            ),
        ],
    );
    // With no import and a shadow, the full path stays resolved while the bare callees do not.
    let report = expect_failure(
        "full-path-shadow",
        &[
            replace(
                "    render_locked(|| handle)",
                "    crate::render_lock::render_locked(|| handle)",
            ),
            replace(
                IMPORT,
                "use crate::render_lock::render_allocation_count;\nfn render_locked<R>(f: impl FnOnce() -> R) -> R {\n    f()\n}",
            ),
        ],
        &[
            Class::RenderLockedNotImported,
            Class::RenderLockedShadowed,
            Class::RenderLockedUnresolved,
        ],
    );
    assert_eq!(naming(&report, "render"), []);
    assert_eq!(
        naming(&report, "source_seek"),
        [Class::RenderLockedUnresolved]
    );
}

/// The call must be `render_locked` with one closure of no parameters.
#[test]
fn the_call_must_be_render_locked_of_one_closure() {
    let bodies = [
        "    other(|| handle)",
        "    render_locked(handle)",
        "    render_locked(|value| value)",
        "    render_locked(|| handle, 1)",
        "    render_locked()",
        "    self::render_locked(|| handle)",
        "    (render_locked)(|| handle)",
        "    handle",
    ];
    for (index, body) in bodies.iter().enumerate() {
        let report = expect_failure(
            &format!("call-shape-{index}"),
            &[replace("    render_locked(|| handle)", body)],
            &[Class::RenderLockedBody],
        );
        assert_eq!(
            naming(&report, "render"),
            [Class::RenderLockedBody],
            "{body}"
        );
    }
}

/// A table entry must name a top-level `#[unsafe(no_mangle)]` export, once, wrapped or not.
#[test]
fn a_table_entry_that_names_no_export_or_two_is_a_finding() {
    let missing = expect_failure(
        "export-deleted",
        &[replace(
            "pub extern \"C\" fn miso_engine_web_v1_dispose(handle: u32) -> u32 {",
            "pub extern \"C\" fn miso_engine_web_v1_disposed(handle: u32) -> u32 {",
        )],
        &[Class::ExportMissing],
    );
    assert_eq!(naming(&missing, "dispose"), [Class::ExportMissing]);
    let tree = Tree::base("export-without-attribute");
    let text = tree.read(FFI).replacen(
        "#[unsafe(no_mangle)]\npub extern \"C\" fn miso_engine_web_v1_render(",
        "pub extern \"C\" fn miso_engine_web_v1_render(",
        1,
    );
    tree.write(FFI, text.as_bytes());
    let report = tree.check();
    assert_eq!(naming(&report, "render"), [Class::ExportMissing]);
    let twice = expect_failure(
        "export-twice",
        &[Edit::Append {
            path: FFI,
            text: "#[unsafe(no_mangle)]\npub extern \"C\" fn miso_engine_web_v1_render(handle: u32) -> u32 {\n    render_locked(|| handle)\n}\n",
        }],
        &[Class::ExportDefinedTwice],
    );
    assert_eq!(naming(&twice, "render"), [Class::ExportDefinedTwice]);
}

/// An unwrapped entry is allowed only when the policy names it as an exception.
#[test]
fn an_unwrapped_entry_outside_the_named_exceptions_is_a_finding() {
    let tree = Tree::base("unnamed-exception");
    let mut case = policy();
    case.render_thread_exports
        .push(render_thread_export("source_seek_twin", false));
    tree.apply(&[Edit::Append {
        path: FFI,
        text: "#[unsafe(no_mangle)]\npub extern \"C\" fn miso_engine_web_v1_source_seek_twin(handle: u32) -> u32 {\n    handle\n}\n",
    }]);
    let report = check(tree.root(), &case);
    assert_eq!(
        naming(&report, "source_seek_twin"),
        [Class::UnwrappedNotPermitted]
    );
    assert_eq!(report.findings.len(), 1);
    // The same policy with the exception named passes.
    case.unwrapped_exports
        .push("miso_engine_web_v1_source_seek_twin".to_owned());
    assert!(check(tree.root(), &case).is_ok());
}

/// The file is required.
#[test]
fn a_tree_without_the_host_web_ffi_is_a_finding() {
    expect_failure(
        "no-web-ffi",
        &[Edit::Delete { path: FFI }],
        &[Class::MissingWebFfi],
    );
}
