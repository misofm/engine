//! One-way pre-launch catalog re-hash oracle from container hashes to canonical-PCM identities.

use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use miso_engine_stem_hasher::canonicalize_wave;
use sha2::{Digest, Sha256};

const OUTPUT_FILES: [&str; 3] = [
    "identity-mapping.tsv",
    "manifest.tsv",
    "document-replacements.tsv",
];
const EMBEDDING_KINDS: [&str; 5] = [
    "manifest_row",
    "mix_document",
    "app_fixture",
    "package_pin",
    "server_record",
];

/// Stable migration refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationError {
    code: &'static str,
}

impl MigrationError {
    const fn new(code: &'static str) -> Self {
        Self { code }
    }

    /// Stable dotted reason.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "miso.catalog.migrate.v1\t{}", self.code)
    }
}

impl std::error::Error for MigrationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CatalogRow {
    name: String,
    old_identity: String,
    master_wave: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct EmbeddingRow {
    old_identity: String,
    kind: String,
    target: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MigratedRow {
    name: String,
    old_identity: String,
    new_identity: String,
    channels: u16,
    bit_depth: u16,
    frames: u64,
    pcm_bytes: u64,
}

/// Execute a one-way migration into a new output directory.
pub fn migrate_catalog(
    catalog: &Path,
    embeddings: &Path,
    output_directory: &Path,
) -> Result<(), MigrationError> {
    let catalog_rows = parse_catalog(catalog)?;
    let embedding_rows = parse_embeddings(embeddings, &catalog_rows)?;
    let mut migrated = Vec::with_capacity(catalog_rows.len());
    let catalog_directory = catalog
        .parent()
        .ok_or_else(|| MigrationError::new("catalog.path.invalid"))?;
    for row in &catalog_rows {
        let master_path = catalog_directory.join(&row.master_wave);
        let container = fs::read(&master_path).map_err(|_| MigrationError::new("master.read"))?;
        let observed_old = format!(
            "sha256:{}",
            lowercase_hex(Sha256::digest(&container).into())
        );
        if observed_old != row.old_identity {
            return Err(MigrationError::new("old_identity.container_mismatch"));
        }
        let mut wave = File::open(&master_path).map_err(|_| MigrationError::new("master.open"))?;
        let report = canonicalize_wave(&mut wave, &mut std::io::sink())
            .map_err(|_| MigrationError::new("master.canonicalize"))?;
        let new_identity = report.identity();
        if !identity_format(&new_identity) {
            return Err(MigrationError::new("new_identity.format"));
        }
        migrated.push(MigratedRow {
            name: row.name.clone(),
            old_identity: row.old_identity.clone(),
            new_identity,
            channels: report.shape.channels,
            bit_depth: report.shape.bit_depth.bits(),
            frames: report.shape.frames,
            pcm_bytes: report.canonical_bytes,
        });
    }
    migrated.sort_by(|left, right| left.name.cmp(&right.name));
    let by_old = migrated
        .iter()
        .map(|row| (row.old_identity.as_str(), row.new_identity.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mapping = render_mapping(&migrated);
    let manifest = render_manifest(&migrated);
    let replacements = render_replacements(&embedding_rows, &by_old)?;

    fs::create_dir(output_directory).map_err(|_| MigrationError::new("output.create"))?;
    let result = (|| {
        write_new(&output_directory.join(OUTPUT_FILES[0]), mapping.as_bytes())?;
        write_new(&output_directory.join(OUTPUT_FILES[1]), manifest.as_bytes())?;
        write_new(
            &output_directory.join(OUTPUT_FILES[2]),
            replacements.as_bytes(),
        )?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(output_directory);
    }
    result
}

/// Recompute the migration and require byte-identical pinned outputs.
pub fn check_catalog(
    catalog: &Path,
    embeddings: &Path,
    expected_directory: &Path,
) -> Result<(), MigrationError> {
    let temporary = create_temp_directory()?;
    let actual_directory = temporary.join("actual");
    let result = (|| {
        migrate_catalog(catalog, embeddings, &actual_directory)?;
        for name in OUTPUT_FILES {
            let actual = fs::read(actual_directory.join(name))
                .map_err(|_| MigrationError::new("check.actual.read"))?;
            let expected = fs::read(expected_directory.join(name))
                .map_err(|_| MigrationError::new("check.expected.read"))?;
            if actual != expected {
                return Err(MigrationError::new("check.output.mismatch"));
            }
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(&temporary);
    result
}

fn parse_catalog(path: &Path) -> Result<Vec<CatalogRow>, MigrationError> {
    let document = fs::read_to_string(path).map_err(|_| MigrationError::new("catalog.read"))?;
    let mut lines = document.lines();
    if lines.next() != Some("schema_version\t1")
        || lines.next() != Some("name\told_identity\tmaster_wave")
    {
        return Err(MigrationError::new("catalog.header"));
    }
    let mut names = BTreeSet::new();
    let mut identities = BTreeSet::new();
    let mut rows = Vec::new();
    for line in lines {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 3
            || fields.iter().any(|field| field.is_empty())
            || !identity_format(fields[1])
        {
            return Err(MigrationError::new("catalog.row.invalid"));
        }
        if !names.insert(fields[0]) || !identities.insert(fields[1]) {
            return Err(MigrationError::new("catalog.row.duplicate"));
        }
        let master_wave = PathBuf::from(fields[2]);
        if master_wave.is_absolute()
            || master_wave
                .components()
                .any(|part| matches!(part, std::path::Component::ParentDir))
        {
            return Err(MigrationError::new("catalog.master.path"));
        }
        rows.push(CatalogRow {
            name: fields[0].to_owned(),
            old_identity: fields[1].to_owned(),
            master_wave,
        });
    }
    if rows.is_empty() {
        return Err(MigrationError::new("catalog.empty"));
    }
    Ok(rows)
}

fn parse_embeddings(
    path: &Path,
    catalog: &[CatalogRow],
) -> Result<Vec<EmbeddingRow>, MigrationError> {
    let document = fs::read_to_string(path).map_err(|_| MigrationError::new("embeddings.read"))?;
    let mut lines = document.lines();
    if lines.next() != Some("schema_version\t1")
        || lines.next() != Some("old_identity\tkind\ttarget")
    {
        return Err(MigrationError::new("embeddings.header"));
    }
    let identities = catalog
        .iter()
        .map(|row| row.old_identity.as_str())
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    for line in lines {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 3
            || fields.iter().any(|field| field.is_empty())
            || !identities.contains(fields[0])
            || !EMBEDDING_KINDS.contains(&fields[1])
        {
            return Err(MigrationError::new("embeddings.row.invalid"));
        }
        let row = EmbeddingRow {
            old_identity: fields[0].to_owned(),
            kind: fields[1].to_owned(),
            target: fields[2].to_owned(),
        };
        if !seen.insert(row.clone()) {
            return Err(MigrationError::new("embeddings.row.duplicate"));
        }
        rows.push(row);
    }
    let observed_kinds = rows
        .iter()
        .map(|row| row.kind.as_str())
        .collect::<BTreeSet<_>>();
    if EMBEDDING_KINDS
        .iter()
        .any(|kind| !observed_kinds.contains(kind))
        || identities
            .iter()
            .any(|identity| !rows.iter().any(|row| row.old_identity == **identity))
    {
        return Err(MigrationError::new("embeddings.inventory.incomplete"));
    }
    rows.sort();
    Ok(rows)
}

fn render_mapping(rows: &[MigratedRow]) -> String {
    let mut output = String::from("schema_version\t1\nold_identity\tnew_identity\n");
    for row in rows {
        output.push_str(&row.old_identity);
        output.push('\t');
        output.push_str(&row.new_identity);
        output.push('\n');
    }
    output
}

fn render_manifest(rows: &[MigratedRow]) -> String {
    let mut output =
        String::from("schema_version\t1\nname\tidentity\tchannels\tbit_depth\tframes\tpcm_bytes\n");
    for row in rows {
        output.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            row.name, row.new_identity, row.channels, row.bit_depth, row.frames, row.pcm_bytes
        ));
    }
    output
}

fn render_replacements(
    rows: &[EmbeddingRow],
    mapping: &BTreeMap<&str, &str>,
) -> Result<String, MigrationError> {
    let mut output = String::from("schema_version\t1\nkind\ttarget\told_identity\tnew_identity\n");
    for row in rows {
        let new_identity = mapping
            .get(row.old_identity.as_str())
            .ok_or_else(|| MigrationError::new("embeddings.identity.missing"))?;
        output.push_str(&format!(
            "{}\t{}\t{}\t{}\n",
            row.kind, row.target, row.old_identity, new_identity
        ));
    }
    Ok(output)
}

fn identity_format(identity: &str) -> bool {
    identity.len() == 71
        && identity.starts_with("sha256:")
        && identity[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn lowercase_hex(digest: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in digest {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), MigrationError> {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| MigrationError::new("output.create"))?;
    output
        .write_all(bytes)
        .and_then(|()| output.flush())
        .and_then(|()| output.sync_all())
        .map_err(|_| MigrationError::new("output.write"))
}

fn create_temp_directory() -> Result<PathBuf, MigrationError> {
    for nonce in 0_u32..100 {
        let path = std::env::temp_dir().join(format!(
            "miso-engine-catalog-migrate-{}-{nonce}",
            std::process::id()
        ));
        if fs::create_dir(&path).is_ok() {
            return Ok(path);
        }
    }
    Err(MigrationError::new("check.temp.create"))
}

/// Exact closed migration command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationCommand {
    /// Create the three migration oracle outputs in a new directory.
    Migrate {
        /// Old catalog rows.
        catalog: PathBuf,
        /// Complete embedding inventory.
        embeddings: PathBuf,
        /// New output directory.
        output_directory: PathBuf,
    },
    /// Recompute and compare against a pinned oracle directory.
    Check {
        /// Old catalog rows.
        catalog: PathBuf,
        /// Complete embedding inventory.
        embeddings: PathBuf,
        /// Pinned expected output directory.
        expected_directory: PathBuf,
    },
}

/// Parse the exact `migrate`/`check` CLI.
pub fn parse_cli(
    arguments: impl IntoIterator<Item = OsString>,
) -> Result<MigrationCommand, MigrationError> {
    let mut arguments = arguments.into_iter();
    let mode = arguments
        .next()
        .ok_or_else(|| MigrationError::new("cli.mode.missing"))?;
    let mut catalog = None;
    let mut embeddings = None;
    let mut output_directory = None;
    let mut expected_directory = None;
    while let Some(option) = arguments.next() {
        let value = arguments
            .next()
            .ok_or_else(|| MigrationError::new("cli.option.value.missing"))?;
        match option.to_str() {
            Some("--catalog") if catalog.is_none() => catalog = Some(PathBuf::from(value)),
            Some("--embeddings") if embeddings.is_none() => {
                embeddings = Some(PathBuf::from(value));
            }
            Some("--output-dir") if output_directory.is_none() => {
                output_directory = Some(PathBuf::from(value));
            }
            Some("--expected-dir") if expected_directory.is_none() => {
                expected_directory = Some(PathBuf::from(value));
            }
            Some("--catalog" | "--embeddings" | "--output-dir" | "--expected-dir") => {
                return Err(MigrationError::new("cli.option.duplicate"));
            }
            Some(_) | None => return Err(MigrationError::new("cli.option.unknown")),
        }
    }
    let catalog = catalog.ok_or_else(|| MigrationError::new("cli.catalog.missing"))?;
    let embeddings = embeddings.ok_or_else(|| MigrationError::new("cli.embeddings.missing"))?;
    match mode.to_str() {
        Some("migrate") if expected_directory.is_none() => Ok(MigrationCommand::Migrate {
            catalog,
            embeddings,
            output_directory: output_directory
                .ok_or_else(|| MigrationError::new("cli.output_dir.missing"))?,
        }),
        Some("check") if output_directory.is_none() => Ok(MigrationCommand::Check {
            catalog,
            embeddings,
            expected_directory: expected_directory
                .ok_or_else(|| MigrationError::new("cli.expected_dir.missing"))?,
        }),
        Some("migrate" | "check") => Err(MigrationError::new("cli.option.forbidden")),
        Some(_) | None => Err(MigrationError::new("cli.mode.unknown")),
    }
}

/// Execute the closed migration CLI.
pub fn run_cli(arguments: impl IntoIterator<Item = OsString>) -> Result<(), MigrationError> {
    match parse_cli(arguments)? {
        MigrationCommand::Migrate {
            catalog,
            embeddings,
            output_directory,
        } => migrate_catalog(&catalog, &embeddings, &output_directory),
        MigrationCommand::Check {
            catalog,
            embeddings,
            expected_directory,
        } => check_catalog(&catalog, &embeddings, &expected_directory),
    }
}
