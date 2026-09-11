//! Read-only session authoring gate: check the session authoring pipeline over one JSON document and
//! report the engine's own typed diagnostics, stage by stage.
//!
//! # Why a separate tool
//!
//! `docs/SESSION_SCHEMA_V1.md` is normative but dense, and the stage that rejects a hand-authored
//! session is exactly the information an author needs: a JSON typo, a schema violation, a resource
//! cap and a builtins preparation failure are four different repairs. The engine already produces
//! stable typed diagnostics for these failures; nothing exposed them at a command line. This tool is that
//! command line and nothing more -- it reads one file, prepares nothing that outlives the process,
//! writes no artifact, and never renders audio.
//!
//! # The stages
//!
//! The stages are the real pipeline in the real order:
//!
//! 1. `json-grammar` -- the JSON grammar accepted by `json-syntax`, the first thing
//!    [`parse_session_json`] does. Its only diagnostic code is `json.syntax`.
//! 2. `typed-model` -- the strict V1 schema decode plus the issue-004 validation
//!    [`parse_session_json`] runs on the decoded model: unknown keys, ID syntax and uniqueness,
//!    references, closed enum tokens, finite/`f32`/unit-local domains, source bounds, automation
//!    ordering.
//! 3. `compile-session` -- [`compile_session`]: the checked resource preflight, the cap
//!    comparisons, and canonical normalization into a non-publishable [`CompiledSession`].
//! 4. `prepare-builtins` -- [`prepare_session_builtins`]: off-render preparation of the input
//!    builtins, fader/mute and 2x2 matrix stages. It is the cheapest evidence that the declared
//!    builtins are preparable and not merely well-formed.
//! 5. `prepare-effects` -- [`prepare_native_session_effects`] with the launch native registry.
//!
//! PASS does not certify graph/PDC compilation, source availability, or host resource budgets.
//!
//! Stages 1 and 2 are one function call, because the parser validates the model it just decoded.
//! They are still reported separately, and correctly: `json.syntax` is produced only by the grammar
//! and is returned alone, so a document that fails the grammar can never also carry a schema
//! diagnostic. Attributing by code therefore names the same stage a duplicated grammar parse would,
//! without pinning a second copy of the JSON dependency in this workspace.
//!
//! # Caps
//!
//! All cap structures are set to their maxima. A validator that imposed a host's budget would
//! reject documents that are perfectly legal sessions. Queue, ring, and aggregate memory budgets
//! are host policy rather than session-document fields, so this authoring tool validates the model
//! and its checked arithmetic without choosing a deployment budget.

use std::{
    collections::{HashMap, HashSet},
    fmt::Write as _,
    fs::File,
    io::{Read as _, Seek, SeekFrom},
    process::ExitCode,
};

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use session::{
    CompileCaps, CompiledSession, DiagnosticCode, DiagnosticSet, SourceBitDepth, StableId,
    canonical_session_json, compile_session, parse_session_json,
};

/// Maximum canonical session bytes accepted by the bounded fold-mono command.
pub const FOLD_MONO_MAX_SESSION_BYTES: usize = 8 * 1024 * 1024;
/// Maximum replacement-map bytes accepted by the bounded fold-mono command.
pub const FOLD_MONO_MAX_MAP_BYTES: usize = 256 * 1024;
/// Maximum source identities in one replacement map.
pub const FOLD_MONO_MAX_ENTRIES: usize = 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
struct IdentityReplacement {
    old: String,
    new: String,
}

/// A failed bounded fold-mono transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoldMonoError(String);

impl std::fmt::Display for FoldMonoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for FoldMonoError {}

/// The five pipeline stages, in execution order.
pub const STAGE_NAMES: [&str; 5] = [
    "json-grammar",
    "typed-model",
    "compile-session",
    "prepare-builtins",
    "prepare-effects",
];

const STAGE_SUMMARIES: [&str; 5] = [
    "JSON grammar (json-syntax)",
    "strict V1 schema decode and validation",
    "resource preflight, caps, canonical normalization",
    "off-render builtins preparation",
    "off-render launch native effect preparation",
];

/// How one stage ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageStatus {
    /// The stage accepted the document.
    Pass,
    /// The stage rejected the document; every diagnostic it produced is reported.
    Fail,
    /// An earlier stage failed, so this stage was never run.
    Skipped,
}

impl StageStatus {
    /// The four-character label used in the rendered report.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Skipped => "SKIP",
        }
    }
}

/// One typed diagnostic, flattened for display and for assertions in tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageDiagnostic {
    /// Stable dotted registry code, for example `schema.unknown_field`.
    pub code: String,
    /// Structured document path rendered in `$.a[0].b` form.
    pub path: String,
    /// One-based source line, when the producing stage still had source text.
    pub line: Option<usize>,
    /// One-based source column, when the producing stage still had source text.
    pub column: Option<usize>,
    /// Concise human explanation. It is not a compatibility contract.
    pub message: String,
}

impl StageDiagnostic {
    fn render(&self) -> String {
        let mut line = format!("{}  {}", self.code, self.path);
        if let (Some(row), Some(column)) = (self.line, self.column) {
            let _ = write!(line, "  (line {row}, column {column})");
        }
        // A `json.syntax` message carries the parser's multi-line source excerpt. Continuation
        // lines are indented rather than flattened, so one diagnostic still reads as one block.
        if self.message.contains('\n') {
            for part in self.message.lines() {
                line.push('\n');
                let trimmed = part.trim_end();
                if !trimmed.is_empty() {
                    let _ = write!(line, "          {trimmed}");
                }
            }
        } else if !self.message.is_empty() {
            let _ = write!(line, "  {}", self.message);
        }
        line
    }
}

/// The result of running one stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageOutcome {
    /// Stable stage name from [`STAGE_NAMES`].
    pub name: &'static str,
    /// One-line description of what the stage checks.
    pub summary: &'static str,
    /// Outcome of the stage.
    pub status: StageStatus,
    /// Diagnostics the stage produced, in the engine's own deterministic order.
    pub diagnostics: Vec<StageDiagnostic>,
}

/// A complete stage-by-stage verdict for one document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReport {
    stages: Vec<StageOutcome>,
    canonical: Option<String>,
}

impl ValidationReport {
    /// The five stage outcomes, in execution order.
    #[must_use]
    pub fn stages(&self) -> &[StageOutcome] {
        &self.stages
    }

    /// The canonical re-serialization, present only when every stage passed.
    #[must_use]
    pub fn canonical(&self) -> Option<&str> {
        self.canonical.as_deref()
    }

    /// True when no stage failed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.stages
            .iter()
            .all(|stage| stage.status != StageStatus::Fail)
    }

    /// The zero-based index of the failing stage, if any.
    #[must_use]
    pub fn failed_stage(&self) -> Option<usize> {
        self.stages
            .iter()
            .position(|stage| stage.status == StageStatus::Fail)
    }

    /// Render the deterministic human/agent-readable report for a document label.
    #[must_use]
    pub fn render(&self, label: &str) -> String {
        let width = STAGE_NAMES.iter().map(|name| name.len()).max().unwrap_or(0);
        let mut out = format!("session: {label}\n");
        for (index, stage) in self.stages.iter().enumerate() {
            let _ = writeln!(
                out,
                "  {}  stage {}  {:width$}  {}",
                stage.status.label(),
                index + 1,
                stage.name,
                if stage.status == StageStatus::Skipped {
                    "not reached"
                } else {
                    stage.summary
                },
            );
            for diagnostic in &stage.diagnostics {
                let _ = writeln!(out, "        {}", diagnostic.render());
            }
        }
        match self.failed_stage() {
            None => out.push_str("result: PASS\n"),
            Some(index) => {
                let count = self.stages[index].diagnostics.len();
                let _ = writeln!(
                    out,
                    "result: FAIL at stage {} ({}), {} diagnostic{}",
                    index + 1,
                    self.stages[index].name,
                    count,
                    if count == 1 { "" } else { "s" },
                );
            }
        }
        out
    }
}

fn stage(index: usize, status: StageStatus, diagnostics: Vec<StageDiagnostic>) -> StageOutcome {
    StageOutcome {
        name: STAGE_NAMES[index],
        summary: STAGE_SUMMARIES[index],
        status,
        diagnostics,
    }
}

fn session_diagnostics(set: &DiagnosticSet) -> Vec<StageDiagnostic> {
    set.diagnostics()
        .iter()
        .map(|diagnostic| StageDiagnostic {
            code: diagnostic.code.to_string(),
            path: diagnostic.path.to_string(),
            line: diagnostic.span.map(|span| span.line),
            column: diagnostic.span.map(|span| span.column),
            message: diagnostic.message.clone(),
        })
        .collect()
}

/// Cap structures wide open: deployment limits are host policy. See the module docs.
fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn builtin_caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

/// Run every stage over one session document.
///
/// The walk stops at the first failing stage: a later stage's input is the earlier stage's output,
/// so running one on a rejected model would report invented diagnostics.
#[must_use]
pub fn validate_session_document(source: &str) -> ValidationReport {
    let mut stages = Vec::with_capacity(STAGE_NAMES.len());

    let model = match parse_session_json(source) {
        Ok(model) => {
            stages.push(stage(0, StageStatus::Pass, Vec::new()));
            stages.push(stage(1, StageStatus::Pass, Vec::new()));
            model
        }
        Err(set) => {
            let diagnostics = session_diagnostics(&set);
            // `json.syntax` is produced only by the grammar parse, and it is returned alone, so a
            // document that fails the grammar can never also carry a schema diagnostic.
            let grammar_failed = set
                .diagnostics()
                .iter()
                .all(|diagnostic| diagnostic.code == DiagnosticCode::JsonSyntax);
            if grammar_failed {
                stages.push(stage(0, StageStatus::Fail, diagnostics));
                stages.push(stage(1, StageStatus::Skipped, Vec::new()));
            } else {
                stages.push(stage(0, StageStatus::Pass, Vec::new()));
                stages.push(stage(1, StageStatus::Fail, diagnostics));
            }
            return skipped_tail(stages, 2);
        }
    };

    let compiled: CompiledSession = match compile_session(&model, compile_caps()) {
        Ok(compiled) => {
            stages.push(stage(2, StageStatus::Pass, Vec::new()));
            compiled
        }
        Err(set) => {
            stages.push(stage(2, StageStatus::Fail, session_diagnostics(&set)));
            return skipped_tail(stages, 3);
        }
    };

    if let Err(set) = prepare_session_builtins(&compiled, &[], builtin_caps()) {
        let mut diagnostics = set.0;
        diagnostics.sort();
        stages.push(stage(
            3,
            StageStatus::Fail,
            diagnostics
                .into_iter()
                .map(|diagnostic| StageDiagnostic {
                    code: diagnostic.code.to_owned(),
                    path: diagnostic.path,
                    line: None,
                    column: None,
                    message: String::new(),
                })
                .collect(),
        ));
        return skipped_tail(stages, 4);
    }
    stages.push(stage(3, StageStatus::Pass, Vec::new()));

    let registry = match launch_native_effect_registry() {
        Ok(registry) => registry,
        Err(_) => {
            stages.push(stage(
                4,
                StageStatus::Fail,
                vec![StageDiagnostic {
                    code: "effect.registry.unavailable".to_owned(),
                    path: "$".to_owned(),
                    line: None,
                    column: None,
                    message: "launch native registry construction failed".to_owned(),
                }],
            ));
            return skipped_tail(stages, 5);
        }
    };
    if let Err(set) = prepare_native_session_effects(
        &compiled,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: u32::MAX,
        },
    ) {
        let mut diagnostics = set.0;
        diagnostics.sort();
        stages.push(stage(
            4,
            StageStatus::Fail,
            diagnostics
                .into_iter()
                .map(|diagnostic| StageDiagnostic {
                    code: diagnostic.code.to_owned(),
                    path: diagnostic.path,
                    line: None,
                    column: None,
                    message: String::new(),
                })
                .collect(),
        ));
        return skipped_tail(stages, 5);
    }
    stages.push(stage(4, StageStatus::Pass, Vec::new()));

    let canonical = compiled.canonical_json().to_owned();
    ValidationReport {
        stages,
        canonical: Some(canonical),
    }
}

fn skipped_tail(mut stages: Vec<StageOutcome>, from: usize) -> ValidationReport {
    for index in from..STAGE_NAMES.len() {
        stages.push(stage(index, StageStatus::Skipped, Vec::new()));
    }
    ValidationReport {
        stages,
        canonical: None,
    }
}

fn fold_error(message: impl Into<String>) -> FoldMonoError {
    FoldMonoError(message.into())
}

fn diagnostic_text(set: &DiagnosticSet) -> String {
    set.diagnostics()
        .iter()
        .map(|diagnostic| {
            format!(
                "{} {}{}",
                diagnostic.code,
                diagnostic.path,
                if diagnostic.message.is_empty() {
                    String::new()
                } else {
                    format!(": {}", diagnostic.message)
                }
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn bounded_file(path: &str, maximum: usize) -> Result<Vec<u8>, FoldMonoError> {
    let mut file =
        File::open(path).map_err(|error| fold_error(format!("cannot read {path}: {error}")))?;
    let byte_length = file
        .metadata()
        .map_err(|error| fold_error(format!("cannot stat {path}: {error}")))?
        .len();
    if byte_length > u64::try_from(maximum).expect("control-tool limit fits u64") {
        return Err(fold_error(format!(
            "{path} exceeds the {}-byte limit",
            maximum
        )));
    }
    // A bounded read remains bounded even if the file changes after metadata was observed.
    let mut bytes = Vec::with_capacity(usize::try_from(byte_length).unwrap_or(maximum));
    file.seek(SeekFrom::Start(0))
        .map_err(|error| fold_error(format!("cannot seek {path}: {error}")))?;
    file.take(u64::try_from(maximum).expect("control-tool limit fits u64") + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| fold_error(format!("cannot read {path}: {error}")))?;
    if bytes.len() > maximum {
        return Err(fold_error(format!(
            "{path} exceeds the {}-byte limit",
            maximum
        )));
    }
    Ok(bytes)
}

fn valid_sha256_identity(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn parse_fold_map(bytes: &[u8]) -> Result<Vec<IdentityReplacement>, FoldMonoError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    if !bytes.ends_with(b"\n") {
        return Err(fold_error("replacement map must be LF-terminated"));
    }
    let text =
        std::str::from_utf8(bytes).map_err(|_| fold_error("replacement map must be UTF-8"))?;
    let mut replacements = Vec::new();
    let mut old_identities = HashSet::new();
    for line in text.split_terminator('\n') {
        if line.is_empty() {
            return Err(fold_error("replacement map has a blank line"));
        }
        let Some((old, new)) = line.split_once('\t') else {
            return Err(fold_error(
                "replacement map lines require exactly old<TAB>new",
            ));
        };
        validate_replacement_identity(old, new, &mut old_identities)?;
        if replacements.len() == FOLD_MONO_MAX_ENTRIES {
            return Err(fold_error(format!(
                "replacement map exceeds the {}-entry limit",
                FOLD_MONO_MAX_ENTRIES
            )));
        }
        replacements.push(IdentityReplacement {
            old: old.to_owned(),
            new: new.to_owned(),
        });
    }
    Ok(replacements)
}

fn validate_replacement_identity<'a>(
    old: &'a str,
    new: &str,
    old_identities: &mut HashSet<&'a str>,
) -> Result<(), FoldMonoError> {
    if !valid_sha256_identity(old) || !valid_sha256_identity(new) {
        return Err(fold_error(
            "replacement identities must be sha256: followed by 64 lowercase hex digits",
        ));
    }
    if old == new {
        return Err(fold_error("replacement identity cannot map to itself"));
    }
    if !old_identities.insert(old) {
        return Err(fold_error("replacement map repeats an old identity"));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SourceShape {
    bit_depth: SourceBitDepth,
    channels: u8,
    frames: u64,
}

fn fold_mono_document(
    source_text: &str,
    replacements: &[IdentityReplacement],
) -> Result<String, FoldMonoError> {
    if source_text.len() > FOLD_MONO_MAX_SESSION_BYTES {
        return Err(fold_error(format!(
            "session exceeds the {}-byte limit",
            FOLD_MONO_MAX_SESSION_BYTES
        )));
    }
    let mut model = parse_session_json(source_text)
        .map_err(|set| fold_error(format!("invalid session: {}", diagnostic_text(&set))))?;
    let canonical = canonical_session_json(&model)
        .map_err(|set| fold_error(format!("invalid session: {}", diagnostic_text(&set))))?;
    if canonical != source_text {
        return Err(fold_error("session input must be canonical JSON"));
    }
    // Even an empty map must pass the same complete control-plane compile gate. Its accepted
    // result is returned byte-for-byte below, so no canonical writer can alter a no-op.
    compile_session(&model, compile_caps()).map_err(|set| {
        fold_error(format!(
            "session compilation failed: {}",
            diagnostic_text(&set)
        ))
    })?;
    if replacements.is_empty() {
        return Ok(source_text.to_owned());
    }

    let by_old: HashMap<&str, &IdentityReplacement> = replacements
        .iter()
        .map(|replacement| (replacement.old.as_str(), replacement))
        .collect();
    let mut matched = HashSet::new();
    let mut original_shapes: HashMap<&str, SourceShape> = HashMap::new();
    for source in &model.sources {
        let shape = SourceShape {
            bit_depth: source.bit_depth,
            channels: source.channels,
            frames: source.frames,
        };
        if let Some(replacement) = by_old.get(source.content.as_str()) {
            matched.insert(replacement.old.as_str());
            if shape.channels != 2
                || !matches!(
                    shape.bit_depth,
                    SourceBitDepth::Pcm16 | SourceBitDepth::Pcm24
                )
            {
                return Err(fold_error(format!(
                    "mapped source {} must be two-channel PCM16 or PCM24",
                    source.id
                )));
            }
            if let Some(previous) = original_shapes.insert(source.content.as_str(), shape)
                && previous != shape
            {
                return Err(fold_error(format!(
                    "sources sharing {} have conflicting shapes",
                    source.content
                )));
            }
        }
    }
    if matched.len() != replacements.len() {
        let missing = replacements
            .iter()
            .find(|replacement| !matched.contains(replacement.old.as_str()))
            .expect("matched count differs only when one replacement is missing");
        return Err(fold_error(format!(
            "mapped identity {} does not occur in the session",
            missing.old
        )));
    }

    // Mapping is simultaneous: every lookup uses the original content identity, so A->B and
    // B->C never cascade A through B to C.
    let affected_source_ids: HashSet<StableId> = model
        .sources
        .iter()
        .filter(|source| by_old.contains_key(source.content.as_str()))
        .map(|source| source.id.clone())
        .collect();
    let mut final_shapes: HashMap<&str, SourceShape> = HashMap::new();
    for source in &mut model.sources {
        let replacement = by_old.get(source.content.as_str()).copied();
        if let Some(replacement) = replacement {
            source.content = replacement.new.clone();
            source.channels = 1;
        }
        let shape = SourceShape {
            bit_depth: source.bit_depth,
            channels: source.channels,
            frames: source.frames,
        };
        if let Some(previous) = final_shapes.insert(source.content.as_str(), shape)
            && previous != shape
        {
            return Err(fold_error(format!(
                "resulting identity {} has conflicting source shapes",
                source.content
            )));
        }
    }
    for track in &mut model.tracks {
        if affected_source_ids.contains(&track.source_id) {
            track.left_source_channel = 0;
            track.right_source_channel = 0;
        }
    }
    model.revision = model
        .revision
        .checked_add(1)
        .ok_or_else(|| fold_error("session revision exhausted"))?;
    let compiled = compile_session(&model, compile_caps()).map_err(|set| {
        fold_error(format!(
            "transformed session is invalid: {}",
            diagnostic_text(&set)
        ))
    })?;
    let output = compiled.canonical_json().to_owned();
    if output.len() > FOLD_MONO_MAX_SESSION_BYTES {
        return Err(fold_error(format!(
            "transformed session exceeds the {}-byte limit",
            FOLD_MONO_MAX_SESSION_BYTES
        )));
    }
    Ok(output)
}

/// Apply complete, pre-validated identity replacements to one canonical session document.
///
/// The replacement slice is intentionally kept internal to the command parser; callers should
/// use the bounded CLI map protocol rather than constructing a migration manifest of their own.
pub fn fold_mono_session_document(
    source_text: &str,
    replacements: &[(String, String)],
) -> Result<String, FoldMonoError> {
    if replacements.len() > FOLD_MONO_MAX_ENTRIES {
        return Err(fold_error(format!(
            "replacement map exceeds the {}-entry limit",
            FOLD_MONO_MAX_ENTRIES
        )));
    }
    let mut old_identities = HashSet::new();
    let parsed = replacements
        .iter()
        .map(|(old, new)| {
            validate_replacement_identity(old, new, &mut old_identities)?;
            Ok(IdentityReplacement {
                old: old.clone(),
                new: new.clone(),
            })
        })
        .collect::<Result<Vec<_>, FoldMonoError>>()?;
    fold_mono_document(source_text, &parsed)
}

const USAGE: &str = "\
usage: session_validator validate [--canonical] <session.json>

  session_validator fold-mono --map <folds.tsv> <canonical-session.json>
      Apply bounded producer-verified stereo-to-mono identity replacements and write the
      complete canonical transformed session to stdout. Diagnostics are written to stderr.

  validate <path>
      Run every session pipeline stage over <path> and print a PASS/FAIL line per stage
      followed by that stage's typed diagnostics (code, $.json.path, source line/column).

  validate --canonical <path>
      The same run, but the canonical re-serialization is written to stdout and the stage
      report to stderr, so `... --canonical draft.json > session.json` normalizes a document.
      Nothing is written when a stage fails.

Read-only: no file is written, no artifact is produced, no audio is rendered.

Exit codes: 0 accepted, 1 validation/transformation failure, 2 usage or I/O error.
";

/// Run the command line. Returns the process exit code; `--help` prints the full contract.
#[must_use]
pub fn run(arguments: impl Iterator<Item = String>) -> ExitCode {
    let arguments: Vec<String> = arguments.collect();
    let mut rest = arguments.iter().map(String::as_str);
    match rest.next() {
        Some("fold-mono") => return run_fold_mono(rest.collect()),
        Some("validate") => {}
        Some("--help" | "-h") => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        _ => return usage("expected the `validate` subcommand"),
    }
    let mut canonical = false;
    let mut path: Option<&str> = None;
    for argument in rest {
        match argument {
            "--canonical" if !canonical => canonical = true,
            "--help" | "-h" => {
                print!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            other if other.starts_with('-') => {
                return usage(&format!("unknown option: {other}"));
            }
            other if path.is_none() => path = Some(other),
            other => return usage(&format!("unexpected extra argument: {other}")),
        }
    }
    let Some(path) = path else {
        return usage("expected exactly one session path");
    };

    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("cannot read {path}: {error}");
            return ExitCode::from(2);
        }
    };

    let report = validate_session_document(&source);
    let rendered = report.render(path);
    match report.canonical() {
        Some(document) if canonical => {
            eprint!("{rendered}");
            print!("{document}");
        }
        _ if canonical => eprint!("{rendered}"),
        _ => print!("{rendered}"),
    }
    if report.passed() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run_fold_mono(arguments: Vec<&str>) -> ExitCode {
    let mut arguments = arguments.into_iter();
    if arguments.next() != Some("--map") {
        return usage("fold-mono requires `--map <folds.tsv> <canonical-session.json>`");
    }
    let Some(map_path) = arguments.next() else {
        return usage("fold-mono requires a replacement-map path");
    };
    let Some(session_path) = arguments.next() else {
        return usage("fold-mono requires a canonical-session path");
    };
    if arguments.next().is_some() {
        return usage("fold-mono accepts exactly one map and one session path");
    }
    let map = match bounded_file(map_path, FOLD_MONO_MAX_MAP_BYTES) {
        Ok(map) => map,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(2);
        }
    };
    let session = match bounded_file(session_path, FOLD_MONO_MAX_SESSION_BYTES) {
        Ok(session) => session,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(2);
        }
    };
    let map = match parse_fold_map(&map) {
        Ok(map) => map,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    let source = match std::str::from_utf8(&session) {
        Ok(source) => source,
        Err(_) => {
            eprintln!("session must be UTF-8");
            return ExitCode::FAILURE;
        }
    };
    match fold_mono_document(source, &map) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn usage(reason: &str) -> ExitCode {
    eprintln!("{reason}");
    eprint!("{USAGE}");
    ExitCode::from(2)
}
