//! The table of the exports either AudioWorklet calls after construction.
//!
//! `render-thread-exports.json` is the one list. This tool reads it to prove that each wrapped
//! entry's body is a `render_locked` call (`render_wrap`), and the two worklet harnesses
//! (`scripts/test-web-audioworklet.mjs` and `sdk/test/browser-pcm-evals.mjs`) read the same file
//! to fail when a worklet calls an export that the table does not list.

use std::collections::BTreeSet;

use serde_json::{Map, Value};

/// The prefix of every export's name; the table spells names without it.
pub(crate) const PREFIX: &str = "miso_engine_web_v1_";

/// The worklets that call an export after construction.
const WORKLETS: [&str; 2] = ["engine", "feed"];

/// One export a worklet calls after construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderThreadExport {
    /// The export's full name, with the table's prefix.
    pub name: String,
    /// Whether the export's body is one `render_locked` call.
    pub wrapped: bool,
    /// Every worklet and handler that calls it.
    pub callers: Vec<Caller>,
    /// Why an unwrapped export is unwrapped. Set exactly when `wrapped` is false.
    pub reason: Option<String>,
}

/// A worklet's handler that calls an export.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Caller {
    /// `engine` (the engine AudioWorklet) or `feed` (the SDK's PCM-feed worklet).
    pub worklet: String,
    /// The method of the worklet's class that makes the call.
    pub handler: String,
}

/// Parse the table. The shape is strict: an unknown key, a wrong type, a duplicate name, an entry
/// with no caller, a wrapped entry with a reason and an unwrapped one without are all errors.
pub(crate) fn parse(text: &str) -> Result<Vec<RenderThreadExport>, String> {
    let root: Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let root = root.as_object().ok_or("the table is not an object")?;
    only_keys(root, &["table", "prefix", "exports"], "the table")?;
    if root.get("table").and_then(Value::as_str) != Some("render-thread-exports") {
        return Err("`table` is not \"render-thread-exports\"".to_owned());
    }
    if root.get("prefix").and_then(Value::as_str) != Some(PREFIX) {
        return Err(format!("`prefix` is not \"{PREFIX}\""));
    }
    let entries = root
        .get("exports")
        .and_then(Value::as_array)
        .ok_or("`exports` is not an array")?;
    let mut seen = BTreeSet::new();
    let mut exports = Vec::new();
    for entry in entries {
        let entry = entry
            .as_object()
            .ok_or("an export entry is not an object")?;
        only_keys(
            entry,
            &["name", "callers", "wrapped", "reason"],
            "an export entry",
        )?;
        let name = entry
            .get("name")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .ok_or("an export entry has no `name`")?;
        if !seen.insert(name.to_owned()) {
            return Err(format!("`{name}` is listed twice"));
        }
        let wrapped = entry
            .get("wrapped")
            .and_then(Value::as_bool)
            .ok_or_else(|| format!("`{name}`: `wrapped` is not a boolean"))?;
        let reason = match (wrapped, entry.get("reason")) {
            (true, None) => None,
            (false, Some(Value::String(reason))) if !reason.is_empty() => Some(reason.clone()),
            (true, Some(_)) => return Err(format!("`{name}` is wrapped and has a reason")),
            (false, _) => return Err(format!("`{name}` is unwrapped and has no reason")),
        };
        let callers = entry
            .get("callers")
            .and_then(Value::as_array)
            .filter(|callers| !callers.is_empty())
            .ok_or_else(|| format!("`{name}` has no callers"))?
            .iter()
            .map(|caller| parse_caller(name, caller))
            .collect::<Result<Vec<_>, _>>()?;
        exports.push(RenderThreadExport {
            name: format!("{PREFIX}{name}"),
            wrapped,
            callers,
            reason,
        });
    }
    Ok(exports)
}

fn parse_caller(name: &str, caller: &Value) -> Result<Caller, String> {
    let caller = caller
        .as_object()
        .ok_or_else(|| format!("`{name}`: a caller is not an object"))?;
    only_keys(caller, &["worklet", "handler"], "a caller")?;
    let text = |key: &str| {
        caller
            .get(key)
            .and_then(Value::as_str)
            .filter(|text| !text.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| format!("`{name}`: a caller has no `{key}`"))
    };
    let worklet = text("worklet")?;
    if !WORKLETS.contains(&worklet.as_str()) {
        return Err(format!("`{name}`: unknown worklet `{worklet}`"));
    }
    Ok(Caller {
        worklet,
        handler: text("handler")?,
    })
}

fn only_keys(object: &Map<String, Value>, allowed: &[&str], what: &str) -> Result<(), String> {
    match object.keys().find(|key| !allowed.contains(&key.as_str())) {
        Some(key) => Err(format!("{what} has an unknown key `{key}`")),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    const GOOD: &str = r#"{"table": "render-thread-exports", "prefix": "miso_engine_web_v1_", "exports": [
        {"name": "render", "callers": [{"worklet": "engine", "handler": "process"}], "wrapped": true},
        {"name": "dispose", "callers": [{"worklet": "engine", "handler": "receive"}], "wrapped": false, "reason": "teardown"}
    ]}"#;

    #[test]
    fn the_good_table_parses_with_prefixed_names() {
        let exports = parse(GOOD).expect("the good table parses");
        let names: Vec<&str> = exports.iter().map(|export| export.name.as_str()).collect();
        assert_eq!(
            names,
            ["miso_engine_web_v1_render", "miso_engine_web_v1_dispose"]
        );
        assert_eq!(exports[1].reason.as_deref(), Some("teardown"));
    }

    /// Each edit turns the good table into one the loader must refuse. A loader that kept the
    /// default of a misspelled key (`wraped`) or accepted an unwrapped entry without its reason
    /// would let the table lose a wrap without a word.
    #[test]
    fn a_table_that_could_hide_a_missing_wrap_is_refused() {
        for (old, new) in [
            ("\"wrapped\": true", "\"wrapped\": true, \"wraped\": false"),
            (
                "\"handler\": \"process\"",
                "\"handler\": \"process\", \"hander\": \"x\"",
            ),
            (
                "\"table\": \"render-thread-exports\",",
                "\"table\": \"render-thread-exports\", \"extra\": 1,",
            ),
            ("\"wrapped\": true", "\"wrapped\": \"yes\""),
            ("\"wrapped\": true", "\"wrapped\": true, \"reason\": \"r\""),
            (", \"reason\": \"teardown\"", ""),
            (", \"reason\": \"teardown\"", ", \"reason\": \"\""),
            ("\"name\": \"dispose\"", "\"name\": \"render\""),
            (
                "\"callers\": [{\"worklet\": \"engine\", \"handler\": \"process\"}]",
                "\"callers\": []",
            ),
            (
                "\"worklet\": \"engine\", \"handler\": \"process\"",
                "\"worklet\": \"main\", \"handler\": \"process\"",
            ),
            (
                "\"worklet\": \"engine\", \"handler\": \"process\"",
                "\"worklet\": \"engine\"",
            ),
            (
                "\"prefix\": \"miso_engine_web_v1_\"",
                "\"prefix\": \"miso_\"",
            ),
            ("\"table\": \"render-thread-exports\"", "\"table\": \"x\""),
        ] {
            assert!(GOOD.contains(old), "{old} is in the good table");
            assert!(
                parse(&GOOD.replacen(old, new, 1)).is_err(),
                "{old} -> {new}"
            );
        }
        assert!(parse("[]").is_err());
        assert!(parse("{").is_err());
    }
}
