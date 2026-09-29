//! The shipped `author-session` skill's validator commands must run as written.
//!
//! The skill is the repo's answer to "how does an agent build a session file", and its whole value
//! is that an agent can paste its commands verbatim. A renamed subcommand or flag that left the
//! skill stale would be discovered by the next agent that tried to use it, which is the worst
//! possible place to discover it. So every `session-validator` command in the skill's fenced shell
//! blocks is run here, through the validator's own binary, on a launch fixture: the command line
//! must parse and the fixture must pass. Commands of other packages are not this binary's to run.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root is two levels above tools/<crate>")
        .to_path_buf()
}

/// Every line inside a fenced shell block of `markdown`.
fn fenced_shell_lines(markdown: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut inside = false;
    for line in markdown.lines().map(str::trim) {
        if let Some(fence) = line.strip_prefix("```") {
            inside = !inside && matches!(fence, "sh" | "bash" | "shell");
        } else if inside && !line.is_empty() {
            lines.push(line);
        }
    }
    assert!(!inside, "a fenced shell block is not closed");
    lines
}

/// The package and argv of a `cargo run ... -p <package> -- <argv>` line, less a trailing
/// `> file` redirection.
fn cargo_run(line: &str) -> Option<(&str, Vec<&str>)> {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.get(..2) != Some(&["cargo", "run"][..]) {
        return None;
    }
    assert!(
        !line.contains(['\'', '"', '$', '|', '<', '\\']),
        "the skill command {line:?} needs a shell this test does not model"
    );
    let separator = words.iter().position(|word| *word == "--")?;
    let package = words[..separator]
        .windows(2)
        .find(|pair| pair[0] == "-p" || pair[0] == "--package")
        .map(|pair| pair[1])?;
    let mut argv = words[separator + 1..].to_vec();
    if let Some(redirect) = argv.iter().position(|word| *word == ">") {
        assert_eq!(redirect + 2, argv.len(), "{line:?}: one redirection target");
        argv.truncate(redirect);
    }
    Some((package, argv))
}

#[test]
fn the_shipped_skill_validator_commands_run_verbatim() {
    let root = repository_root();
    let path = root.join(".claude/skills/author-session/SKILL.md");
    let skill = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    // A launch fixture that passes every stage stands in for the skill's placeholder paths.
    let fixture = root.join("fixtures/session/v1/canonical-minimal.json");
    let canonical = std::fs::read_to_string(&fixture)
        .unwrap_or_else(|error| panic!("read {}: {error}", fixture.display()));

    let mut ran = 0;
    for line in fenced_shell_lines(&skill) {
        let Some(("session-validator", argv)) = cargo_run(line) else {
            continue;
        };
        let output = Command::new(env!("CARGO_BIN_EXE_session_validator"))
            .args(argv.iter().map(|word| {
                if word.ends_with(".json") {
                    fixture.as_os_str()
                } else {
                    OsStr::new(word)
                }
            }))
            .output()
            .expect("run the validator binary");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            output.status.code(),
            Some(0),
            "the skill command {line:?} does not run as written:\n{stderr}"
        );
        let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
        if argv.contains(&"--canonical") {
            assert_eq!(
                stdout, canonical,
                "{line:?}: stdout is the canonical document"
            );
        } else {
            assert!(
                stdout.contains("PASS"),
                "{line:?}: the stage report\n{stdout}"
            );
        }
        ran += 1;
    }
    assert!(
        ran > 0,
        "the skill names no session-validator command to run"
    );
}
