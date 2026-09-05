#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
scratch=$(mktemp -d); trap 'rm -rf -- "$scratch"' EXIT
base="$scratch/base"; work="$scratch/work"
mkdir -p "$base/crates/graph/src" "$base/crates/graph-compiler/src" "$base/crates/other/src"
cat >"$base/crates/graph/Cargo.toml" <<'EOF'
[dependencies]
rack.workspace = true
lane.workspace = true
engine.workspace = true
effect-contract.workspace = true
EOF
cat >"$base/crates/graph-compiler/Cargo.toml" <<'EOF'
[dependencies]
sha2.workspace = true
EOF
cat >"$base/crates/graph/src/lib.rs" <<'EOF'
impl PreparedPlanExecutor for Graph {}
#[cfg(test)]
mod tests { fn ignored() { std::fs::read("x"); } }
EOF
printf 'pub fn compile() {}\n' >"$base/crates/graph-compiler/src/lib.rs"
printf 'pub fn other() {}\n' >"$base/crates/other/src/lib.rs"
reset() { rm -rf "$work"; cp -R "$base" "$work"; }
pass() { "$@" >/dev/null; }
fail() { local expected=$1; shift; local output; if output=$("$@" 2>&1); then printf 'graph fixture unexpectedly passed: %s\n' "$expected" >&2; exit 1; fi; [[ "$output" == *"$expected"* ]] || { printf 'wrong graph diagnostic for %s: %s\n' "$expected" "$output" >&2; exit 1; }; }
checker="$root/scripts/check-graph-policy.sh"
pass bash "$checker" "$base"
(cd "$root" && pass bash scripts/check-graph-policy.sh "$base")
reset; rm "$work/crates/graph/Cargo.toml"; fail 'missing graph manifests' bash "$checker" "$work"
reset; rm -rf "$work/crates/graph-compiler/src"; fail 'graph source discovery traversal errored' bash "$checker" "$work"
reset; sed -i '1i pub fn bad() { std::fs::read("x"); }' "$work/crates/graph/src/lib.rs"; fail 'publication, I/O, threading' bash "$checker" "$work"
reset; printf 'impl PreparedPlanExecutor for Other {}\n' >"$work/crates/other/src/lib.rs"; fail 'production prepared-plan executor must remain graph-owned' bash "$checker" "$work"
reset; sed -i 's/sha2.workspace = true/sha2.workspace=true/' "$work/crates/graph-compiler/Cargo.toml"; fail 'SHA-256 dependency search failed' bash "$checker" "$work"
reset; sed -i 's/rack.workspace = true/rack.workspace=true/' "$work/crates/graph/Cargo.toml"; fail 'render graph dependency boundary changed' bash "$checker" "$work"
make_shim() { local dir=$1 name=$2 body=$3; mkdir -p "$dir"; printf '#!/usr/bin/env bash\n%s\n' "$body" >"$dir/$name"; chmod +x "$dir/$name"; }
reset; make_shim "$scratch/find-error" find 'exit 7'; fail 'traversal errored (find status 7)' env PATH="$scratch/find-error:$PATH" bash "$checker" "$work"
reset; make_shim "$scratch/find-partial" find '/usr/bin/find "$@"; printf "plausible.rs\n"; exit 7'; fail 'plausible.rs' env PATH="$scratch/find-partial:$PATH" bash "$checker" "$work"
reset; make_shim "$scratch/find-workspace" find 'case "$*" in "crates -name"*) [[ -z ${SHIM_PARTIAL:-} ]] || { /usr/bin/find "$@"; printf "crates/other/src/lib.rs\n"; }; exit 7;; esac; exec /usr/bin/find "$@"'; for partial in '' 1; do fail 'workspace Rust discovery traversal errored (find status 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/find-workspace:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/sed-error" sed '[[ -z ${SHIM_PARTIAL:-} ]] || printf "impl PreparedPlanExecutor for Graph {}\n"; exit 8'; for partial in '' 1; do fail 'source read failed' env SHIM_PARTIAL="$partial" PATH="$scratch/sed-error:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/sort-error" sort '[[ -z ${SHIM_PARTIAL:-} ]] || /usr/bin/sort "$@"; exit 9'; for partial in '' 1; do fail 'dependency extraction failed for crates/graph/Cargo.toml (sort status 9)' env SHIM_PARTIAL="$partial" PATH="$scratch/sort-error:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/awk-error" awk '[[ -z ${SHIM_PARTIAL:-} ]] || printf "effect-contract.workspace\nengine.workspace\nlane.workspace\nrack.workspace\n"; exit 7'; for partial in '' 1; do fail 'dependency extraction failed for crates/graph/Cargo.toml (awk status 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/awk-error:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-publication" rg 'case "$*" in *PlanPublisher*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "1:std::fs\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'graph publication predicate scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-publication:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-executor" rg 'case "$*" in *"impl PreparedPlanExecutor for"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "1:impl PreparedPlanExecutor for Graph\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'prepared-plan executor predicate' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-executor:$PATH" bash "$checker" "$work"; done
printf 'graph policy fixtures: ok\n'
