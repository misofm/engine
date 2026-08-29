//! Read-only session authoring gate: run every session pipeline stage over one TOML document and
//! report the engine's own typed diagnostics, stage by stage.
//!
//! # Why a separate tool
//!
//! `docs/SESSION_SCHEMA_V1.md` is normative but dense, and the stage that rejects a hand-authored
//! session is exactly the information an author needs: a TOML typo, a schema violation, a resource
//! cap and a builtins preparation failure are four different repairs. The engine already produces
//! stable typed diagnostics for all four; nothing exposed them at a command line. This tool is that
//! command line and nothing more -- it reads one file, prepares nothing that outlives the process,
//! writes no artifact, and never renders audio.
//!
//! # The stages
//!
//! The stages are the real pipeline in the real order:
//!
//! 1. `toml-grammar` -- the TOML grammar accepted by `toml_parser`, the first thing
//!    [`parse_session_toml`] does. Its only diagnostic code is `toml.syntax`.
//! 2. `typed-model` -- the strict V1 schema decode plus the issue-004 validation
//!    [`parse_session_toml`] runs on the decoded model: unknown keys, ID syntax and uniqueness,
//!    references, closed enum tokens, finite/`f32`/unit-local domains, source bounds, automation
//!    ordering.
//! 3. `compile-session` -- [`compile_session`]: the checked resource preflight, the cap
//!    comparisons, and canonical normalization into a non-publishable [`CompiledSession`].
//! 4. `prepare-builtins` -- [`prepare_session_builtins`]: off-render preparation of the input
//!    builtins, fader/mute and 2x2 matrix stages. It is the cheapest evidence that the declared
//!    session is preparable and not merely well-formed.
//!
//! Stages 1 and 2 are one function call, because the parser validates the model it just decoded.
//! They are still reported separately, and correctly: `toml.syntax` is produced only by the grammar
//! and is returned alone, so a document that fails the grammar can never also carry a schema
//! diagnostic. Attributing by code therefore names the same stage a duplicated grammar parse would,
//! without pinning a second copy of the TOML dependency in this workspace.
//!
//! # Caps
//!
//! Both cap structures are set to their maxima. A validator that imposed a host's budget would
//! reject documents that are perfectly legal sessions. Queue, ring, and aggregate memory budgets
//! are host policy rather than session-document fields, so this authoring tool validates the model
//! and its checked arithmetic without choosing a deployment budget.

use std::{fmt::Write as _, process::ExitCode};

use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_session::{
    CompileCaps, CompiledSession, DiagnosticCode, DiagnosticSet, compile_session,
    parse_session_toml,
};

/// The four pipeline stages, in execution order.
pub const STAGE_NAMES: [&str; 4] = [
    "toml-grammar",
    "typed-model",
    "compile-session",
    "prepare-builtins",
];

const STAGE_SUMMARIES: [&str; 4] = [
    "TOML grammar (toml_parser)",
    "strict V1 schema decode and validation",
    "resource preflight, caps, canonical normalization",
    "off-render builtins preparation",
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
        // A `toml.syntax` message carries the parser's multi-line source excerpt. Continuation
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
    /// The four stage outcomes, in execution order.
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

    let model = match parse_session_toml(source) {
        Ok(model) => {
            stages.push(stage(0, StageStatus::Pass, Vec::new()));
            stages.push(stage(1, StageStatus::Pass, Vec::new()));
            model
        }
        Err(set) => {
            let diagnostics = session_diagnostics(&set);
            // `toml.syntax` is produced only by the grammar parse, and it is returned alone, so a
            // document that fails the grammar can never also carry a schema diagnostic.
            let grammar_failed = set
                .diagnostics()
                .iter()
                .all(|diagnostic| diagnostic.code == DiagnosticCode::TomlSyntax);
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

    let canonical = compiled.canonical_toml().to_owned();
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

const USAGE: &str = "\
usage: miso_engine_session_validator validate [--canonical] <session.toml>

  validate <path>
      Run every session pipeline stage over <path> and print a PASS/FAIL line per stage
      followed by that stage's typed diagnostics (code, $.json.path, source line/column).

  validate --canonical <path>
      The same run, but the canonical re-serialization is written to stdout and the stage
      report to stderr, so `... --canonical draft.toml > session.toml` normalizes a document.
      Nothing is written when a stage fails.

Read-only: no file is written, no artifact is produced, no audio is rendered.

Exit codes: 0 every stage passed, 1 a stage failed, 2 usage or I/O error.
";

/// Run the command line. Returns the process exit code; `--help` prints the full contract.
#[must_use]
pub fn run(arguments: impl Iterator<Item = String>) -> ExitCode {
    let arguments: Vec<String> = arguments.collect();
    let mut canonical = false;
    let mut path: Option<&str> = None;
    let mut rest = arguments.iter().map(String::as_str);
    match rest.next() {
        Some("validate") => {}
        Some("--help" | "-h") => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        _ => return usage("expected the `validate` subcommand"),
    }
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

fn usage(reason: &str) -> ExitCode {
    eprintln!("{reason}");
    eprint!("{USAGE}");
    ExitCode::from(2)
}
