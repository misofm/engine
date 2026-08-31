//! The shipped `author-session` skill must name commands that actually exist.
//!
//! The skill is the repo's answer to "how does an agent build a session file", and its whole value
//! is that an agent can paste its commands verbatim. A renamed tool or flag that left the skill
//! stale would be discovered by the next agent that tried to use it, which is the worst possible
//! place to discover it.

use std::path::{Path, PathBuf};

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root is two levels above tools/<crate>")
        .to_path_buf()
}

#[test]
fn the_shipped_skill_names_the_real_commands() {
    let path = repository_root().join(".claude/skills/author-session/SKILL.md");
    let skill = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    for required in [
        "cargo run -q -p session-validator -- validate",
        "--canonical",
        "cargo run -q -p parameter-metadata -- --print",
        "docs/SESSION_SCHEMA_V1.md",
        "fixtures/session/v1/canonical.toml",
    ] {
        assert!(
            skill.contains(required),
            "the skill must name {required:?} so an agent can run it verbatim"
        );
    }
}
