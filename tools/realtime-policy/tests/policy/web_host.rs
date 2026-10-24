//! The browser host rules that keep the wrap rule's file the file that ships (#1496 D2
//! Amendment 1): (a) conditional compilation, (b) `#[export_name]`, (c) `#[path]`, (d) glob `use`;
//! and the `render_lock.rs` rules that keep `render_locked` and the allocator real (#1528 D2).

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
    // The verifier's second form: the import is gated off and a glob supplies a pass-through,
    // which is also a second `render_locked` (#1528 D2).
    expect_failure(
        "web-cfg-import-plus-glob",
        &[
            replace(FFI, IMPORT, &format!("#[cfg(any())]\n{IMPORT}")),
            append(LIB, FAKE),
            append(FFI, "use crate::fake::*;\n"),
        ],
        &[
            Class::ConditionalWrapPath,
            Class::GlobInWrapModule,
            Class::RenderLockedNotUnique,
        ],
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

/// Rule (b) (#1528 D1(b)): every `#[export_name]` under `hosts/host-web/src` is refused, whatever
/// its value and spelling and in any file, so an exported name is always a function's own.
#[test]
fn every_export_name_is_refused() {
    let spellings = [
        "#[export_name = \"miso_engine_web_v1_x\"]",
        "#[unsafe(export_name = \"miso_engine_web_v1_x\")]",
        "#[unsafe(export_name = \"memory_probe\")]",
        "#[cfg_attr(target_family = \"wasm\", unsafe(export_name = \"memory_probe\"))]",
        "#[cfg_attr(test, cfg_attr(target_family = \"wasm\", export_name = \"retired\"))]",
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
    // The verifier's `export_name_override_twin`: the listed export renamed away, and a child
    // twin that takes its name.
    expect_failure(
        "web-export-name-override-twin",
        &[
            replace(
                FFI,
                SEEK_FN,
                &format!("#[unsafe(export_name = \"retired_source_seek\")]\n{SEEK_FN}"),
            ),
            append(
                FFI,
                "mod inner {\n    #[unsafe(no_mangle)]\n    pub extern \"C\" fn miso_engine_web_v1_source_seek(handle: u32) -> u32 {\n        handle\n    }\n}\n",
            ),
        ],
        &[Class::ExportNameOnWebExport, Class::ExportTwinOutsideFfi],
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
/// is not refused) it is missing, not accepted, and it is a twin.
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
    // The moved export is also a twin outside the top level of `ffi.rs` (#1528 D1(c)).
    assert_classes(&tree, &[Class::ExportMissing, Class::ExportTwinOutsideFfi]);
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

/// `render_lock.rs` text: the real `render_locked` line of the base tree.
const RENDER_LOCKED_FN: &str = "pub(crate) fn render_locked<R>(body: impl FnOnce() -> R) -> R {";
const ALLOCATOR_CFG: &str = "#[cfg(all(target_family = \"wasm\", not(test)))]";
const ALLOCATOR_ATTRIBUTE: &str = "#[global_allocator]";
const ALLOCATOR_STATIC: &str = "static ALLOCATOR: RenderLockedAllocator<std::alloc::System> =";
const ALLOCATOR_VALUE: &str = "    RenderLockedAllocator(std::alloc::System);";
/// The render-lock probe in #1529 D3's exact spelling, with `{attributes}` and `{item}` to vary.
const PROBE: &str = "/// The probe.\n{attributes}\n{item} {\n    if inside != 0 {\n        render_locked(|| drop(Box::new(inside)));\n    }\n    render_allocation_count()\n}\n";
const PROBE_ATTRIBUTES: &str = "#[cfg(feature = \"render-lock-probe\")]\n#[unsafe(no_mangle)]";
const PROBE_ITEM: &str =
    "pub extern \"C\" fn miso_engine_web_v1_render_lock_probe(inside: u32) -> u32";

fn probe(attributes: &str, item: &str) -> String {
    PROBE
        .replace("{attributes}", attributes)
        .replace("{item}", item)
}

/// The allocator removed from `render_lock.rs`.
fn without_allocator() -> Vec<Edit<'static>> {
    [
        ALLOCATOR_CFG,
        ALLOCATOR_ATTRIBUTE,
        ALLOCATOR_STATIC,
        ALLOCATOR_VALUE,
    ]
    .into_iter()
    .map(|line| replace(RENDER_LOCK, line, ""))
    .collect()
}

/// #1528 D2, refused: conditional compilation that could make `render_locked`, its helpers or the
/// counting allocator a pass-through in the shipped build, an allocator of another form, place or
/// count, and a second `render_locked`.
#[test]
fn a_render_lock_pass_through_or_unregistered_allocator_is_refused() {
    use Class::{
        AllocatorOutsideRenderLock as Elsewhere, ConditionalInRenderLockBody as Nested,
        ConditionalRenderLockItem as TopLevel, RenderLockAllocatorForm as Form,
        RenderLockedNotUnique as Unique,
    };
    let lock = |old: &'static str, new: &str| replace(RENDER_LOCK, old, new);
    let cases: Vec<(&str, Vec<Edit>, Vec<Class>)> = vec![
        // The verifier's `render_lock_cfg_split`.
        (
            "cfg-split",
            vec![
                lock(RENDER_LOCKED_FN, &format!("#[cfg(test)]\n{RENDER_LOCKED_FN}")),
                append(
                    RENDER_LOCK,
                    &format!("#[cfg(not(test))]\n{RENDER_LOCKED_FN}\n    body()\n}}\n"),
                ),
            ],
            vec![TopLevel, Unique],
        ),
        (
            "cfg-attr-on-render-locked",
            vec![lock(RENDER_LOCKED_FN, &format!("#[cfg_attr(test, inline)]\n{RENDER_LOCKED_FN}"))],
            vec![TopLevel],
        ),
        (
            "cfg-not-test-mod-tests",
            vec![lock("#[cfg(test)]", "#[cfg(not(test))]")],
            vec![TopLevel],
        ),
        (
            "cfg-on-statement",
            vec![lock("    body()", "    #[cfg(any())]\n    let _ = 0;\n    body()")],
            vec![Nested],
        ),
        (
            "cfg-macro-in-body",
            vec![lock("    body()", "    debug_assert!(cfg!(test));\n    body()")],
            vec![Nested],
        ),
        (
            "second-render-locked-in-lib",
            vec![append(LIB, "fn render_locked() {}\n")],
            vec![Unique],
        ),
        (
            "second-render-locked-in-other-file",
            vec![Edit::Create {
                path: "hosts/host-web/src/other.rs",
                text: "mod inner {\n    pub(crate) fn render_locked() {}\n}\n",
            }],
            vec![Unique],
        ),
        (
            "second-render-locked-in-impl",
            vec![append(
                RENDER_LOCK,
                "impl<A> RenderLockedAllocator<A> {\n    fn render_locked() {}\n}\n",
            )],
            vec![Unique],
        ),
        (
            "second-render-locked-in-tests",
            vec![lock("    use super::*;", "    use super::*;\n    fn render_locked() {}")],
            vec![Unique],
        ),
        (
            "render-locked-missing",
            vec![lock(RENDER_LOCKED_FN, "pub(crate) fn render_unlocked<R>(body: impl FnOnce() -> R) -> R {")],
            vec![Unique],
        ),
        (
            "allocator-under-test",
            vec![lock(ALLOCATOR_CFG, "#[cfg(all(target_family = \"wasm\", test))]")],
            vec![Form],
        ),
        (
            "allocator-second-cfg",
            vec![lock(ALLOCATOR_CFG, &format!("{ALLOCATOR_CFG}\n#[cfg(any())]"))],
            vec![Form],
        ),
        (
            "allocator-in-cfg-attr",
            vec![lock(
                ALLOCATOR_ATTRIBUTE,
                "#[cfg_attr(target_family = \"wasm\", global_allocator)]",
            )],
            vec![Form],
        ),
        (
            "allocator-typed-system",
            vec![lock(ALLOCATOR_STATIC, "static ALLOCATOR: std::alloc::System =")],
            vec![Form],
        ),
        ("allocator-absent", without_allocator(), vec![Form]),
        (
            "allocator-in-lib",
            without_allocator()
                .into_iter()
                .chain([append(
                    LIB,
                    &format!("{ALLOCATOR_CFG}\n{ALLOCATOR_ATTRIBUTE}\n{ALLOCATOR_STATIC}\n{ALLOCATOR_VALUE}\n"),
                )])
                .collect(),
            vec![Form, Elsewhere],
        ),
        (
            "allocator-path-qualified-in-lib",
            vec![append(
                LIB,
                "mod other {\n    #[core::prelude::v1::global_allocator]\n    static ALLOCATOR: u8 = 0;\n}\n",
            )],
            vec![Elsewhere],
        ),
        (
            "allocator-renamed-by-use",
            vec![append(
                LIB,
                "#[cfg(test)]\nmod other {\n    use core::prelude::v1::global_allocator as allocator;\n}\n",
            )],
            vec![Elsewhere],
        ),
        (
            "allocator-in-a-module",
            vec![append(
                FFI,
                "mod other {\n    #[global_allocator]\n    static ALLOCATOR: u8 = 0;\n}\n",
            )],
            vec![Elsewhere],
        ),
        (
            "probe-other-predicate",
            vec![append(
                RENDER_LOCK,
                &probe("#[cfg(feature = \"other\")]\n#[unsafe(no_mangle)]", PROBE_ITEM),
            )],
            vec![TopLevel],
        ),
        (
            "probe-other-name",
            vec![append(
                RENDER_LOCK,
                &probe(
                    PROBE_ATTRIBUTES,
                    "pub extern \"C\" fn miso_engine_web_v1_render_lock_probe2(inside: u32) -> u32",
                ),
            )],
            vec![TopLevel],
        ),
        (
            "probe-static",
            vec![append(
                RENDER_LOCK,
                &format!("{PROBE_ATTRIBUTES}\npub static miso_engine_web_v1_render_lock_probe: u32 = 0;\n"),
            )],
            vec![TopLevel],
        ),
        (
            "probe-extra-cfg",
            vec![append(
                RENDER_LOCK,
                &probe(&format!("{PROBE_ATTRIBUTES}\n#[cfg(any())]"), PROBE_ITEM),
            )],
            vec![TopLevel],
        ),
        (
            "probe-twice",
            vec![
                append(RENDER_LOCK, &probe(PROBE_ATTRIBUTES, PROBE_ITEM)),
                append(RENDER_LOCK, &probe(PROBE_ATTRIBUTES, PROBE_ITEM)),
            ],
            vec![TopLevel],
        ),
    ];
    for (name, edits, expected) in &cases {
        expect_failure(&format!("render-lock-{name}"), edits, expected);
    }
}

/// #1528 D2, accepted: the base `render_lock.rs` (the allocator and `#[cfg(test)] mod tests`,
/// without the probe, which the base tree's validity covers), the same with #1529's probe in its
/// exact spelling, and a test allocator inside a `#[cfg(test)]` module elsewhere.
#[test]
fn the_exact_render_lock_forms_are_accepted() {
    expect_valid(
        "render-lock-probe",
        &[append(RENDER_LOCK, &probe(PROBE_ATTRIBUTES, PROBE_ITEM))],
    );
    expect_valid(
        "render-lock-test-allocator",
        &[append(
            FFI,
            "#[cfg(test)]\nmod counting {\n    #[global_allocator]\n    static ALLOCATOR: u8 = 0;\n}\n",
        )],
    );
}
