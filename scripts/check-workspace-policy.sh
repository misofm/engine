#!/usr/bin/env bash
# Verify bootstrap naming and target-policy guardrails. This is a guard, not permission to hide a
# capacity limit or a global ISA choice elsewhere.
set -euo pipefail

workspace_root="${1:-.}"
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"
cd "$workspace_root"

fail() {
    GATE_FAILURE_PREFIX='workspace policy failure' gate_fail "$1"
    exit 1
}

scratch_dir="$(mktemp -d /tmp/workspace-policy.XXXXXX)"
trap 'rm -rf -- "$scratch_dir"' EXIT

# Run a producer to completion while keeping its stdout and stderr separate.  Callers inspect the
# status explicitly; a clean-looking partial result is never evidence that the producer finished.
capture() {
    local label="$1" out="$scratch_dir/$1.out" err="$scratch_dir/$1.err"
    shift
    if "$@" >"$out" 2>"$err"; then
        CAPTURE_STATUS=0
    else
        CAPTURE_STATUS=$?
    fi
    CAPTURE_OUT="$out"
    CAPTURE_ERR="$err"
}

execution_failure() {
    local operation="$1" status="$2" out="$3" err="$4"
    fail "$operation failed (status $status): stdout=$(<"$out") stderr=$(<"$err")"
}

checked_find() {
    local label="$1"; shift
    capture "$label" find "$@"
    (( CAPTURE_STATUS == 0 )) || execution_failure "find $label" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    CHECKED_FIND_OUT="$CAPTURE_OUT"
}

# `rg` exits 0 on a match, 1 when the pattern is clean, and 2 (or higher) on a search error --
# most commonly a search root that does not exist. `if rg ...; then fail; fi` reads both 1 and 2
# as "no violation", so a scan root that silently stops existing (a directory rename, a fixture
# missing a mkdir) reads as a clean pass instead of the scan never having run. This wrapper keeps
# the three outcomes distinct: 0 is a real violation, 1 is genuinely clean, and >=2 is a scan
# failure that must be loud, naming whichever of the given roots is actually missing.
scan_forbidden() {
    GATE_FAILURE_PREFIX='workspace policy failure' gate_scan_forbidden "$@" || exit 1
}

toml_name() {
    local section="$1"
    local manifest="$2"
    awk -v section="$section" '
        $0 == "[" section "]" { in_section = 1; next }
        in_section && /^\[/ { exit }
        in_section && /^[[:space:]]*name[[:space:]]*=/ {
            value = $0
            sub(/^[[:space:]]*name[[:space:]]*=[[:space:]]*"/, "", value)
            sub(/"[[:space:]]*$/, "", value)
            print value
            exit
        }
    ' "$manifest"
}

toml_array_names() {
    local section="$1"
    local manifest="$2"
    awk -v section="$section" '
        $0 == "[[" section "]]" { in_section = 1; next }
        in_section && /^\[/ { in_section = 0 }
        in_section && /^[[:space:]]*name[[:space:]]*=/ {
            value = $0
            sub(/^[[:space:]]*name[[:space:]]*=[[:space:]]*"/, "", value)
            sub(/"[[:space:]]*$/, "", value)
            print value
            in_section = 0
        }
    ' "$manifest"
}

# Issue #1760: FSL-1.1-ALv2 is the license for original project work; it replaced the Apache
# License of #314. The digest protects the legal text itself, not a prose claim: a truncated or
# edited LICENSE grants different rights.
fsl_license_sha256='3e1f69ef8217e8c1f6fe7e8659f4ca909941eefe09e236ade15c9cd975d963ac'
for required_license_file in LICENSE NOTICE THIRD_PARTY_LICENSES.md crates/math/LICENSE-libm.txt; do
    [[ -s "$required_license_file" ]] || fail "required license artifact is missing or empty: $required_license_file"
done
capture license_sha256 sha256sum LICENSE
(( CAPTURE_STATUS == 0 )) || execution_failure 'sha256sum LICENSE' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
capture license_digest awk '{print $1}' "$CAPTURE_OUT"
(( CAPTURE_STATUS == 0 )) || execution_failure 'LICENSE digest extraction' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
actual_license_sha256="$(<"$CAPTURE_OUT")"
[[ "$actual_license_sha256" == "$fsl_license_sha256" ]] || {
    fail "LICENSE is not the canonical FSL-1.1-ALv2 text"
}
required_search() {
    local label="$1" message="$2"; shift 2
    capture "$label" rg "$@"
    case "$CAPTURE_STATUS" in
        0) ;;
        1) fail "$message" ;;
        *) execution_failure "rg $label" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR" ;;
    esac
}
required_search workspace-license 'Cargo.toml workspace package license must be FSL-1.1-ALv2' -x 'license = "FSL-1.1-ALv2"' Cargo.toml
required_search fuzz-license 'fuzz/Cargo.toml license must be FSL-1.1-ALv2' -x 'license = "FSL-1.1-ALv2"' fuzz/Cargo.toml
required_search libm-inventory 'third-party inventory must retain the vendored libm license record' 'crates/math/LICENSE-libm\.txt' THIRD_PARTY_LICENSES.md

checked_find npm-manifests . -name package.json -type f -not -path '*/node_modules/*'
capture npm-manifests-sort sort "$CHECKED_FIND_OUT"
(( CAPTURE_STATUS == 0 )) || execution_failure 'sort npm manifests' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
sorted_npm_manifests="$CAPTURE_OUT"
while IFS= read -r npm_manifest; do
    [[ -n "$npm_manifest" ]] || continue
    capture "jq-npm-$RANDOM" jq -e '.license == "FSL-1.1-ALv2"' "$npm_manifest"
    case "$CAPTURE_STATUS" in
        0) ;;
        1) fail "$npm_manifest license must be FSL-1.1-ALv2" ;;
        *) execution_failure "jq $npm_manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR" ;;
    esac
done <"$sorted_npm_manifests"

checked_find npm-locks . -name package-lock.json -type f -not -path '*/node_modules/*'
capture npm-locks-sort sort "$CHECKED_FIND_OUT"
(( CAPTURE_STATUS == 0 )) || execution_failure 'sort npm locks' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
sorted_npm_locks="$CAPTURE_OUT"
while IFS= read -r npm_lock; do
    [[ -n "$npm_lock" ]] || continue
    capture "jq-lock-$RANDOM" jq -e '.packages[""].license == "FSL-1.1-ALv2"' "$npm_lock"
    case "$CAPTURE_STATUS" in
        0) ;;
        1) fail "$npm_lock root package license must be FSL-1.1-ALv2" ;;
        *) execution_failure "jq $npm_lock" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR" ;;
    esac
done <"$sorted_npm_locks"

checked_find cargo-manifests crates hosts tools -name Cargo.toml -type f
[[ -s "$CHECKED_FIND_OUT" ]] || fail 'Cargo manifest discovery returned an empty workspace'
capture cargo-manifests-sort sort "$CHECKED_FIND_OUT"
(( CAPTURE_STATUS == 0 )) || execution_failure 'sort Cargo manifests' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
sorted_cargo_manifests="$CAPTURE_OUT"
while IFS= read -r manifest; do
    package_parent="${manifest%/*}"
    package_directory="${package_parent##*/}"
    capture "toml-package-$RANDOM" toml_name package "$manifest"
    (( CAPTURE_STATUS == 0 )) || execution_failure "package name extraction $manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    package_name="$(<"$CAPTURE_OUT")"

    required_search "package-license-$RANDOM" "$manifest must inherit the FSL-1.1-ALv2 workspace license" -x 'license\.workspace = true' "$manifest"

    # The retired prefix has no root-specific exemption: package and directory names are the
    # same short identity everywhere in the three package roots.
    [[ "$package_name" != miso-engine-* && "$package_name" != miso_engine_* ]] || fail "$manifest package name must not carry the retired miso-engine- prefix"
    [[ "$package_directory" == "$package_name" ]] || fail "$manifest directory ($package_directory) must equal its package name ($package_name)"
    # These names shadow Rust sysroot/prelude crates through Cargo's --extern binding.
    case "$package_name" in core|std|alloc|proc_macro|test) fail "$manifest package name '$package_name' collides with a Rust sysroot/prelude crate name";; esac
    # Delivery codecs and their retired migration tooling remain outside this PCM engine.
    case "$package_name" in flac-decoder|stem-publisher|catalog-migrate|flacenc|symphonia) fail "$manifest package name is a retired delivery-codec identity: $package_name";; esac
    expected_crate_name="${package_name//-/_}"
    capture "toml-lib-$RANDOM" toml_name lib "$manifest"
    (( CAPTURE_STATUS == 0 )) || execution_failure "lib name extraction $manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    lib_name="$(<"$CAPTURE_OUT")"
    [[ -z "$lib_name" || "$lib_name" == "$expected_crate_name" ]] || fail "$manifest lib name must be $expected_crate_name"
    capture "toml-bin-$RANDOM" toml_array_names bin "$manifest"
    (( CAPTURE_STATUS == 0 )) || execution_failure "bin name extraction $manifest" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    bin_names="$CAPTURE_OUT"
    while IFS= read -r bin_name; do
        [[ "$bin_name" == "$expected_crate_name" || "$bin_name" == "$expected_crate_name"_* ]] || fail "$manifest bin name must be $expected_crate_name or its underscored audit/tool suffix"
    done <"$bin_names"
done <"$sorted_cargo_manifests"

scan_forbidden "hardware ISA Cargo features are forbidden" \
    '^[[:space:]]*(simd128|neon|avx2|fma)[[:space:]]*=' Cargo.toml \
    crates hosts tools

# S8: every tracked (or freshly added, untracked-but-not-ignored) path in the tree, used below to
# find every Cargo.toml regardless of location -- not the prior six hard-coded roots, which missed
# a nested manifest under e.g. sdk/. `git ls-files` is the tree's own ground truth for "what exists
# here"; the `find` fallback keeps this working against a synthetic (non-git) fixture tree, exactly
# as `scripts/check-env-vocabulary.sh`'s `sources()` does.
tracked_paths() {
    local classify_status
    capture git-classify env LC_ALL=C git rev-parse --is-inside-work-tree
    classify_status="$CAPTURE_STATUS"
    if (( classify_status == 0 )); then
        capture git-list git ls-files -z --cached --others --exclude-standard
        (( CAPTURE_STATUS == 0 )) || execution_failure 'git tracked-path listing' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
        local nul_file="$CAPTURE_OUT"
        # #1527 (i): the index only. An untracked file depends on the checkout; CI checks out
        # tracked files.
        capture git-cached-list git ls-files --cached -z
        (( CAPTURE_STATUS == 0 )) || execution_failure 'git cached-path listing' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
        CACHED_PATHS_NUL="$CAPTURE_OUT"
    elif (( classify_status == 128 )) && [[ -z "${GIT_DIR+x}" && -z "${GIT_WORK_TREE+x}" &&
        "$(<"$CAPTURE_OUT")" == '' && "$(<"$CAPTURE_ERR")" == 'fatal: not a git repository (or any of the parent directories): .git' ]]; then
        capture fallback-list find . -type f -not -path './.git/*' -not -path './target/*' -print0
        (( CAPTURE_STATUS == 0 )) || execution_failure 'fallback tracked-path find' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
        local nul_file="$CAPTURE_OUT"
        # A tree without git has no index: every file is the cached set.
        CACHED_PATHS_NUL="$CAPTURE_OUT"
    else
        execution_failure 'git repository classification' "$classify_status" "$CAPTURE_OUT" "$CAPTURE_ERR"
    fi
    capture nul-to-lines tr '\0' '\n' <"$nul_file"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked-path NUL conversion' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    local lines="$CAPTURE_OUT"
    capture normalize-paths sed 's|^\./||' "$lines"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked-path normalization' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    local normalized="$CAPTURE_OUT"
    # Keep the complete normalized producer output before narrowing it to Cargo manifests. This is
    # the authoritative tracked-or-untracked path set for artifact bans and the manifest policy.
    TRACKED_ALL_PATHS_OUT="$normalized"
    capture cargo-path-filter awk -F/ '$NF == "Cargo.toml"' "$normalized"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked Cargo manifest filter' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    capture tracked-path-sort env LC_ALL=C sort "$CAPTURE_OUT"
    (( CAPTURE_STATUS == 0 )) || execution_failure 'tracked Cargo manifest sort' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    TRACKED_PATHS_OUT="$CAPTURE_OUT"
}

# Issue #356/#359: a bare retired-directory stub (no Cargo.toml, so invisible to the manifest
# loop above) is still a regression -- these five names carried the retired FLAC delivery stack
# and its migration tooling. A whole-tree `find -type d` (N16), not `-mindepth 1 -maxdepth 2`
# under four hard-coded roots: it reaches the repo root, sdk/, and any depth, and -- unlike a scan
# derived from `git ls-files`/tracked file paths -- it also catches a directory with nothing
# tracked inside it yet, which is exactly the shape of a freshly-created leftover stub.
capture retired-dirs find . \( -path './.git' -o -path './target' \) -prune -o -type d \( \
    -name flac-decoder -o -name stem-publisher -o -name catalog-migrate \
    -o -name flacenc -o -name symphonia \) -print
(( CAPTURE_STATUS == 0 )) || execution_failure 'retired-directory discovery' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
while IFS= read -r retired_directory; do
    [[ -z "$retired_directory" ]] && continue
    fail "retired delivery-codec directory remains: ${retired_directory#./}"
done <"$CAPTURE_OUT"

# Every Cargo.toml in the tree (S8: every tracked path named `Cargo.toml`, at any depth,
# excluding nothing -- not the prior six hard-coded roots) and the lockfile: a retired identity
# can also appear as a dependency key, a `package = "..."` rename, or (in Cargo.lock) a resolved
# package entry, none of which the manifest loop above sees.
retired_delivery_codec_pattern='\b(flac-decoder|stem-publisher|catalog-migrate|flacenc|symphonia)\b'

# N17: strip TOML comments before matching, line by line, tracking whether each character is
# inside a quoted string -- a retired name's mere mention in prose ("# we deliberately do not
# depend on symphonia") must not trip this gate, while a real dependency/rename entry (which is
# never itself preceded by an unquoted '#' on its own line) still does. This is intentionally
# line-oriented: Cargo.toml does not use multi-line strings for dependency names or package
# identifiers, which is everything this scan cares about.
tracked_paths
tracked_manifest_paths="$TRACKED_PATHS_OUT"
capture artifact-llvm-scan rg -n '^artifacts/.*\.ll$' "$TRACKED_ALL_PATHS_OUT"
case "$CAPTURE_STATUS" in
  0) fail "tracked LLVM IR artifact is forbidden: $(<"$CAPTURE_OUT")" ;;
  1) ;;
  *) execution_failure 'artifact LLVM path scan' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR" ;;
esac
while IFS= read -r manifest_path; do
    [[ -z "$manifest_path" ]] && continue
    [[ -f "$manifest_path" ]] || continue
    capture strip-comments awk ' {
        in_string=0; out=""
        for (i=1; i<=length($0); i++) { c=substr($0,i,1); if (c=="\"") in_string=!in_string; if (c=="#" && !in_string) break; out=out c }
        print out
    }' "$manifest_path"
    (( CAPTURE_STATUS == 0 )) || execution_failure "comment stripping $manifest_path" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    stripped_manifest="$CAPTURE_OUT"
    capture retired-scan rg -n "$retired_delivery_codec_pattern" "$stripped_manifest"
    case "$CAPTURE_STATUS" in
      0) printf '%s\n' "$(<"$CAPTURE_OUT")" >&2; fail "retired delivery-codec Cargo identity is forbidden: $manifest_path";;
      1) ;;
      *) execution_failure "retired identity scan $manifest_path" "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR";;
    esac
done <"$tracked_manifest_paths"

# One TOML reader for every rule about what Cargo reads (#1519 root Amendment 2; #1527; #1528 D4).
# The realtime-policy walk reads the `.rs` files under the directories `SCAN_ROOTS` lists and skips
# the ones `SKIPPED_DIRECTORIES` lists (tools/realtime-policy/src/walk.rs); both are read from
# walk.rs, and the read fails closed if either constant cannot be found. Python's `tomllib` reads
# each manifest and build configuration the way Cargo does, so an inline table, a dotted key, a
# spaced or quoted header and an escaped key are read as what they are. Rules:
# - Target paths (#1519): `path =` under `[lib]`, `[[bin]]`, `[[test]]`, `[[example]]` and
#   `[[bench]]`, and `[package] build =`, are relative, inside the package (no `..`) and name no
#   skipped directory. A path-less `[[bin]]`, `[[test]]`, `[[example]]` or `[[bench]]` whose `name`
#   is a skipped word is refused: Cargo then picks `src/bin/<name>/` and the like (#1527 (ii)).
#   `hosts/host-web/Cargo.toml` `[lib] path` is absent or exactly `src/lib.rs` (#1528 D4).
# - Dependency keys (#1442 root Amendment 4, #1527 D1): a key `core`, `std` or `alloc` (after `-`
#   becomes `_`) in any dependency table makes Cargo pass `--extern core=..`, which redirects the
#   prelude's `core` for the whole crate; a `package =` rename to one of them is refused too.
# - Containment (#1527 D4/(iii)): every root-workspace member and every dependency `path` (every
#   dependency table, `[patch.*]`, `[replace]`) of every tracked manifest resolves, lexically and
#   through symlinks, to a directory under a scan root and through no skipped directory.
# - Placement (#1527 (i)): no tracked (index-only) path under a scan root has a skipped directory
#   component.
# - Build configuration (#1527 D5): every `.cargo/config` or `.cargo/config.toml`, at any depth, must
#   parse, and has no crate attribute or bootstrap word, no `@` argument file in a flag key and no
#   `include`. The ISA-pin rule below is separate and unchanged.
# - Workflows and scripts (#1527 D6): no file under `.github/workflows/` or `scripts/` holds the
#   crate attribute or bootstrap word. This file and its self-test build both words from fragments.
# The rules judge written forms; a word assembled at run time, a generated flag file, and a
# `$CARGO_HOME` configuration or `RUSTFLAGS` set outside the repository are not read (#1527 D7).
# `tomllib` (Python 3.11+) is imported unconditionally (#1527 D2). Status 3 is a violation; any
# other non-zero status (a missing interpreter or module, an unreadable constant) is an execution
# failure of the gate.
manifest_checker=$(cat <<'PY'
import glob
import os
import re
import sys
import tomllib

walk_source, manifest_list, path_list, cached_nul = sys.argv[1:5]
with open(walk_source, encoding="utf-8") as handle:
    walk_text = handle.read()


def constant(name):
    found = re.search(r"const\s+" + name + r"\b[^=]*=\s*\[(.*?)\]\s*;", walk_text, re.S)
    values = re.findall(r'"([^"]+)"', found.group(1)) if found else []
    if not values:
        sys.exit(f"cannot read {name} from {walk_source}")  # status 1: not a violation
    return values


skipped = [name.lower() for name in constant("SKIPPED_DIRECTORIES")]
roots = constant("SCAN_ROOTS")
# Built from fragments, so this file does not hold the words it refuses.
FORBIDDEN_WORDS = ("crate-" "attr", "rustc-" "bootstrap")
SYSROOT = {"core", "std", "alloc"}
DEPENDENCY_TABLES = ("dependencies", "dev-dependencies", "dev_dependencies",
                     "build-dependencies", "build_dependencies")
lexical_base = os.path.abspath(".")
real_base = os.path.realpath(".")
bad = []


def lines(path):
    with open(path, encoding="utf-8") as handle:
        return [line.rstrip("\n") for line in handle if line.strip()]


manifests = lines(manifest_list)
all_paths = lines(path_list)
with open(cached_nul, "rb") as handle:
    cached = [entry.decode("utf-8", "surrogateescape").removeprefix("./")
              for entry in handle.read().split(b"\0") if entry]


def components(path):
    return [part for part in re.split(r"[/\\]", path) if part not in ("", ".")]


def target_refusal(value):
    if not isinstance(value, str) or not value:
        return "is not a non-empty string"
    if value.startswith(("/", "\\")) or re.match(r"[A-Za-z]:", value):
        return "is not a relative path"
    for component in re.split(r"[/\\]", value):
        if component == "..":
            return "has a '..' component"
        if component.lower() in skipped:
            return f"names the directory '{component}', which the realtime walk skips"
    return None


def placement_refusal(directory, value):
    """Where a member or dependency `path` lands: under a scan root, through no skipped name."""
    if not isinstance(value, str) or not value:
        return "is not a non-empty string"
    for component in components(value):
        if component.lower() in skipped:
            return f"names the directory '{component}', which the realtime walk skips"
    joined = os.path.join(directory, value)
    for kind, base, resolved in (("lexical", lexical_base, os.path.normpath(os.path.abspath(joined))),
                                 ("real", real_base, os.path.realpath(joined))):
        parts = os.path.relpath(resolved, base).split(os.sep)
        if parts[0] == "..":
            return f"resolves ({kind}) outside the repository"
        if parts[0] not in roots:
            return f"resolves ({kind}) to '{'/'.join(parts)}', outside the scan roots {roots}"
        for part in parts:
            if part.lower() in skipped:
                return f"resolves ({kind}) through '{part}', which the realtime walk skips"
    return None


def dependency_tables(manifest, data):
    """Every (label, table) that Cargo reads as dependencies, with its `target.<cfg>` copies."""
    found = [(name, data[name]) for name in DEPENDENCY_TABLES if name in data]

    def under_target(label, table):
        for key, value in table.items():
            if not isinstance(value, dict):
                continue
            if key in DEPENDENCY_TABLES:
                found.append((f"{label}.{key}", value))
            under_target(f"{label}.{key}", value)

    target = data.get("target", {})
    if isinstance(target, dict):
        under_target("target", target)
    else:
        bad.append(f"{manifest}: [target] is not a table")
    workspace = data.get("workspace", {})
    if isinstance(workspace, dict) and "dependencies" in workspace:
        found.append(("workspace.dependencies", workspace["dependencies"]))
    return found


def check_dependencies(manifest, directory, label, table, names):
    if not isinstance(table, dict):
        bad.append(f"{manifest}: [{label}] is not a table")
        return
    for key, value in table.items():
        if names and key.replace("-", "_") in SYSROOT:
            bad.append(f"{manifest}: dependency key '{key}' in [{label}] redirects the sysroot crate")
        if isinstance(value, str):
            continue
        if not isinstance(value, dict):
            bad.append(f"{manifest}: dependency '{key}' in [{label}] is not a string or a table")
            continue
        package = value.get("package")
        if names and isinstance(package, str) and package.replace("-", "_") in SYSROOT:
            bad.append(f"{manifest}: dependency '{key}' in [{label}] renames package '{package}', a sysroot crate name")
        if "path" in value and (why := placement_refusal(directory, value["path"])):
            bad.append(f"{manifest}: dependency '{key}' in [{label}] path = {value['path']!r} {why}")


for manifest in manifests:
    try:
        with open(manifest, "rb") as handle:
            data = tomllib.load(handle)
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as error:
        bad.append(f"{manifest}: cannot be read as TOML: {error}")
        continue
    directory = os.path.dirname(manifest) or "."
    targets = []
    if "lib" in data:
        targets.append(("lib", data["lib"]))
    for kind in ("bin", "test", "example", "bench"):
        if kind in data:
            if not isinstance(data[kind], list):
                bad.append(f"{manifest}: [{kind}] is not an array of tables")
                continue
            targets.extend((kind, table) for table in data[kind])
    for kind, table in targets:
        if not isinstance(table, dict):
            bad.append(f"{manifest}: [{kind}] is not a table")
        elif "path" in table:
            if why := target_refusal(table["path"]):
                bad.append(f"{manifest}: [{kind}] path = {table['path']!r} {why}")
        elif kind != "lib" and isinstance(table.get("name"), str) and table["name"].lower() in skipped:
            bad.append(f"{manifest}: [[{kind}]] name = {table['name']!r} has no path, so Cargo reads a directory the realtime walk skips")
    if manifest == "hosts/host-web/Cargo.toml" and isinstance(data.get("lib"), dict) \
            and "path" in data["lib"] and data["lib"]["path"] != "src/lib.rs":
        bad.append(f"{manifest}: [lib] path = {data['lib']['path']!r} must be absent or exactly 'src/lib.rs'")
    package = data.get("package", {})
    if not isinstance(package, dict):
        bad.append(f"{manifest}: [package] is not a table")
    elif "build" in package and not isinstance(package["build"], bool) and (why := target_refusal(package["build"])):
        bad.append(f"{manifest}: [package] build = {package['build']!r} {why}")
    for label, table in dependency_tables(manifest, data):
        check_dependencies(manifest, directory, label, table, names=True)
    patch = data.get("patch", {})
    if not isinstance(patch, dict):
        bad.append(f"{manifest}: [patch] is not a table")
    else:
        for registry, table in patch.items():
            check_dependencies(manifest, directory, f"patch.{registry}", table, names=False)
    if "replace" in data:
        check_dependencies(manifest, directory, "replace", data["replace"], names=False)
    workspace = data.get("workspace", {})
    if not isinstance(workspace, dict):
        bad.append(f"{manifest}: [workspace] is not a table")
    elif manifest == "Cargo.toml" and "members" in workspace:
        # The root workspace only: another manifest's own `[workspace]` (fuzz) is not this build.
        members = workspace["members"]
        if not isinstance(members, list):
            bad.append(f"{manifest}: [workspace] members is not an array")
            members = []
        for member in members:
            if not isinstance(member, str) or not member:
                bad.append(f"{manifest}: workspace member {member!r} is not a non-empty string")
                continue
            expanded = [member]
            if any(character in member for character in "*?["):
                # Cargo's glob matches a leading dot too.
                expanded = sorted(glob.glob(member, recursive=True, include_hidden=True))
            for entry in expanded:
                if why := placement_refusal(".", entry):
                    bad.append(f"{manifest}: workspace member {entry!r} (from {member!r}) {why}")

for path in cached:
    parts = components(path)
    if len(parts) > 1 and parts[0] in roots:
        for part in parts[1:-1]:
            if part.lower() in skipped:
                bad.append(f"{path}: tracked path under a scan root has the directory '{part}', which the realtime walk skips")
                break


def normalized(text):
    return text.lower().replace("_", "-")


def config_strings(value, flags=False):
    """Every (string, in_flags) in a config value, keys included; `in_flags` is true under a key
    that names rustc or rustdoc flags (`rustflags`, `[env] RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`)."""
    if isinstance(value, str):
        yield value, flags
    elif isinstance(value, list):
        for item in value:
            yield from config_strings(item, flags)
    elif isinstance(value, dict):
        for name, item in value.items():
            yield name, False
            yield from config_strings(item, flags or any(word in name.lower() for word in ("rustflags", "rustdocflags")))


for path in all_paths:
    parts = path.split("/")
    if len(parts) < 2 or parts[-2] != ".cargo" or parts[-1] not in ("config", "config.toml"):
        continue
    try:
        with open(path, "rb") as handle:
            config = tomllib.load(handle)
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as error:
        bad.append(f"{path}: Cargo configuration cannot be read as TOML: {error}")
        continue
    if "include" in config:
        bad.append(f"{path}: Cargo configuration 'include' reads a file this gate does not")
    for text, in_flags in config_strings(config):
        if any(word in normalized(text) for word in FORBIDDEN_WORDS):
            bad.append(f"{path}: Cargo configuration {text!r} injects a crate attribute or unlocks unstable rustc")
        if in_flags and any(token.startswith("@") for token in re.split(r"[\s\x1f]+", text)):
            bad.append(f"{path}: Cargo configuration flag {text!r} reads a rustc argument file")

for path in all_paths:
    if not path.startswith((".github/workflows/", "scripts/")):
        continue
    try:
        with open(path, "rb") as handle:
            text = handle.read().decode("utf-8", "replace")
    except OSError as error:
        bad.append(f"{path}: cannot be read: {error}")
        continue
    if any(word in normalized(text) for word in FORBIDDEN_WORDS):
        bad.append(f"{path}: names a crate attribute or bootstrap flag word, which can inject a crate attribute or unlock unstable rustc")

if bad:
    print("\n".join(bad))
    sys.exit(3)
PY
)
capture manifest-policy python3 -c "$manifest_checker" \
    "$script_directory/../tools/realtime-policy/src/walk.rs" "$tracked_manifest_paths" \
    "$TRACKED_ALL_PATHS_OUT" "$CACHED_PATHS_NUL"
case "$CAPTURE_STATUS" in
  0) ;;
  3) printf '%s\n' "$(<"$CAPTURE_OUT")" >&2
     fail "a Cargo manifest, Cargo configuration, workflow or script lets rustc build source the realtime walk does not read (target paths, dependency keys and paths, members, skipped directories, build flags); see the findings above";;
  *) execution_failure 'manifest policy scan' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR";;
esac

scan_forbidden "retired delivery-codec Cargo identity is forbidden in the lockfile" \
    "$retired_delivery_codec_pattern" 'Cargo.lock' \
    Cargo.lock

scan_forbidden "compiled track-capacity identifiers are forbidden" \
    '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' '*.rs' \
    crates hosts tools

# #313 owner ruling: this is the first prelaunch engine identity. Internal names are unversioned,
# and a boundary that genuinely needs a generation is V1. Build the expression in fragments so
# this policy file does not contain the forbidden spellings it scans for.
prelaunch_later_generation_pattern='(miso_engine_'v'2|MISO_ENGINE_'V'2|miso-engine-'v'2|ENGINE_'V'2|Engine 'V'2|boot[- ]'v'2|Boot 'v'2|schema-'v'2|@miso/engine-'v'2)'
scan_forbidden "prelaunch live-product identities must not claim a later generation" \
    "$prelaunch_later_generation_pattern" '*' crates hosts tools

# Implementation class names are private even when their containing script is shipped. Only the
# registered processor token is boundary identity; an internal JavaScript class is born
# unversioned under #215's rule.
versioned_worklet_implementation_pattern='class[[:space:]]+MisoEngine'V'[0-9]+AudioWorkletProcessor'
scan_forbidden "AudioWorklet processor implementation classes must be unversioned" \
    "$versioned_worklet_implementation_pattern" '*.js' crates hosts tools

# Issue #1052 (AGENTS.md "Test value"): code must not read Rust source or a Cargo manifest as
# text. This refuses the literal-path form: `include_str!`, `include_bytes!`, `read_to_string`,
# `read` or `open` of a plain or raw `.rs` or `.toml` string, with any macro delimiter, however
# rustfmt wraps it, and through `concat!(env!("CARGO_MANIFEST_DIR"), ...)`. A path through a
# `fixtures/` directory is data. A path built at run time, through a helper, `Path::join`,
# `format!` or a `const`, is left to review. The scrapes that predate the rule are allowed per
# file and literal path, each row naming the issue that removes it. The count must match
# exactly, so a removed scrape must lower or delete its row in the same change and a freed
# allowance cannot be spent again. #1047 deletes the last row.
source_scrape_allowlist='
'
source_scrape_pattern='(?:\binclude_str!|\binclude_bytes!|\bread_to_string|\bread|\bopen)\s*[(\[{]\s*(?:concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*)?r?#*"([^"\n]*\.(?:rs|toml))"'
capture source-scrapes rg -U -n -o --no-heading --with-filename --glob '*.rs' -r '$1' "$source_scrape_pattern" crates hosts tools
(( CAPTURE_STATUS <= 1 )) || execution_failure 'source-scrape scan' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
source_scrape_hits="$CAPTURE_OUT"
printf '%s\n' "$source_scrape_allowlist" >"$scratch_dir/source-scrape-allowlist"
capture source-scrape-accounting awk '
    NR == FNR { if (NF) { allowed[$2 " " $3] = $1; owner[$2 " " $3] = $4 }; next }
    {
        file = $0; sub(/:.*/, "", file)
        path = $0; sub(/^[^:]*:[^:]*:/, "", path)
        if (path ~ /(^|\/)fixtures\//) next
        key = file " " path; found[key]++; rows[key] = rows[key] "read as text: " $0 "\n"
    }
    END {
        for (key in found) if (found[key] > allowed[key] + 0) printf "%s", rows[key]
        for (key in allowed) if (found[key] + 0 < allowed[key] + 0)
            printf "stale allow-list row: %s allows %d, found %d; lower or delete it (%s)\n", key, allowed[key], found[key] + 0, owner[key]
    }
' "$scratch_dir/source-scrape-allowlist" "$source_scrape_hits"
(( CAPTURE_STATUS == 0 )) || execution_failure 'source-scrape accounting' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
[[ ! -s "$CAPTURE_OUT" ]] || {
    sort "$CAPTURE_OUT" >&2
    fail 'Rust source or a Cargo manifest is read as text, or the source-scrape allow-list is not exact: assert the behavior instead, and lower or delete a stale row (AGENTS.md "Test value")'
}

# Master plan #83 D4 (revision 4): exactly one global ISA configuration is approved, the
# x86-64-v3 pin that lets `wide` lower `Lane` to AVX2 and `Lane::fma` to `vfmadd` with no runtime
# dispatch (crates/lane refuses to compile without it, and every host attests the CPU
# at boot). Anything else -- `target-cpu`, a global `[build]` table, another feature set -- stays
# forbidden: it would make the shipped ISA implicit again.
approved_isa_pin='^\.cargo/config\.toml:[0-9]+:rustflags = \["-C", "target-feature=\+avx2,\+fma"\]$'
if [[ -d .cargo ]]; then
    capture isa-directives rg -n '(target-cpu|target-feature|rustflags|RUSTFLAGS)' .cargo
    (( CAPTURE_STATUS == 0 || CAPTURE_STATUS == 1 )) || execution_failure 'ISA directive search' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    capture isa-comments-filter rg -v ':[0-9]+:[[:space:]]*#' "$CAPTURE_OUT"
    (( CAPTURE_STATUS == 0 || CAPTURE_STATUS == 1 )) || execution_failure 'ISA comment filtering' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    isa_directives="$(<"$CAPTURE_OUT")"
    capture isa-allowlist-filter rg -v "$approved_isa_pin" <<<"$isa_directives"
    (( CAPTURE_STATUS == 0 || CAPTURE_STATUS == 1 )) || execution_failure 'ISA allowlist filtering' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
    unapproved_directives="$(<"$CAPTURE_OUT")"
    [[ -z "$unapproved_directives" ]] || {
        printf '%s\n' "$unapproved_directives" >&2
        fail "global native CPU or ISA configuration is forbidden outside the approved x86-64-v3 pin"
    }

    if [[ -n "$isa_directives" ]]; then
        required_search isa-target-scope "the approved ISA pin must stay scoped to [target.'cfg(target_arch = \"x86_64\")']" "^\[target\.'cfg\(target_arch = \"x86_64\"\)'\]\$" .cargo/config.toml
    fi

    capture build-table rg -n '^\[build\]' .cargo
    case "$CAPTURE_STATUS" in
      0) fail "a global [build] rustflags table is forbidden";;
      1) ;;
      *) execution_failure 'global build-table search' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR";;
    esac
fi


# #104 phase D. `CARGO_TARGET_DIR=.` (or a stray `--target-dir .`) writes a cargo target tree next
# to `Cargo.toml`: `.rustc_info.json`, `CACHEDIR.TAG`, and one directory per profile/target, each
# holding `.fingerprint/`. 235 such files were committed to `main`. `.gitignore` stops them being
# added again; this gate stops them existing at all, because an ignored spill still poisons every
# `find`/`rg` gate that walks the tree from the workspace root.
for marker in .rustc_info.json CACHEDIR.TAG; do
    [[ ! -e "$marker" ]] || fail "cargo target-dir spill at the workspace root: $marker"
done
capture fingerprints find . -mindepth 2 -maxdepth 2 -type d -name .fingerprint -not -path './target/*' -printf '%P\n'
(( CAPTURE_STATUS == 0 )) || execution_failure 'fingerprint discovery' "$CAPTURE_STATUS" "$CAPTURE_OUT" "$CAPTURE_ERR"
while IFS= read -r fingerprint; do
    [[ -z "$fingerprint" ]] && continue
    fail "cargo target-dir spill at the workspace root: ${fingerprint%/.fingerprint}"
done <"$CAPTURE_OUT"

printf 'workspace policy: ok\n'
