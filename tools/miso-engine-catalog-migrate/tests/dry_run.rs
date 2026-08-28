//! Fixture mini-catalog dry-run and pinned mapping checks.

use std::{fs, path::Path};

use miso_engine_catalog_migrate::check_catalog;

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/flac-delivery/v1"
);

#[test]
fn mini_catalog_reproduces_the_pinned_one_way_oracle() {
    let root = Path::new(FIXTURES);
    check_catalog(
        &root.join("mini-catalog/catalog.tsv"),
        &root.join("mini-catalog/embeddings.tsv"),
        &root.join("mini-catalog/expected"),
    )
    .expect("pinned dry-run");
}

#[test]
fn container_hash_mutation_makes_the_mapping_gate_red() {
    let root = Path::new(FIXTURES);
    let original = fs::read_to_string(root.join("mini-catalog/catalog.tsv")).expect("catalog");
    let mut lines = original.lines();
    let mut mutated = format!("{}\n{}\n", lines.next().unwrap(), lines.next().unwrap());
    for (index, line) in lines.enumerate() {
        let mut fields = line.split('\t').collect::<Vec<_>>();
        if index == 0 {
            fields[1] = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
        }
        mutated.push_str(&fields.join("\t"));
        mutated.push('\n');
    }
    let temporary = create_temp_dir();
    let catalog = temporary.join("catalog.tsv");
    fs::write(&catalog, mutated).expect("write mutation");
    let error = check_catalog(
        &catalog,
        &root.join("mini-catalog/embeddings.tsv"),
        &root.join("mini-catalog/expected"),
    )
    .expect_err("container-hash mutation must be red");
    assert!(["old_identity.container_mismatch", "embeddings.row.invalid"].contains(&error.code()));
    fs::remove_dir_all(temporary).expect("remove test directory");
}

fn create_temp_dir() -> std::path::PathBuf {
    for nonce in 0_u32..100 {
        let path = std::env::temp_dir().join(format!(
            "miso-engine-catalog-migrate-test-{}-{nonce}",
            std::process::id()
        ));
        if fs::create_dir(&path).is_ok() {
            return path;
        }
    }
    panic!("could not create test directory");
}
