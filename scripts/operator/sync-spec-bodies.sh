#!/usr/bin/env bash
# Sync committed issue specs to their GitHub issue bodies and titles, with checks (issue #1445).
#
# Invoked by hand by an operator, never by a workflow: it needs a token with issue write access,
# which CI does not hold. It produces one line per selected spec on stdout, `<n> <class>`, then a
# `summary` line, and with --apply it also writes the old raw issue JSON of every issue it edits
# to a backup directory and edits that issue's body (and title, when it differs).
#
# Usage:
#   sync-spec-bodies.sh [--check] SELECTION [--reconcile-dir DIR] [--reviewed DIR]
#   sync-spec-bodies.sh --apply --backup-dir DIR SELECTION [--reconcile-dir DIR] [--reviewed DIR]
# SELECTION is exactly one of: --all | --range A..B | ISSUE_NUMBER...
#
# The source of every spec is the blob on refs/remotes/origin/main, read with `git cat-file`,
# never the working tree or the index. The script refuses (exit 2, before any `gh` call) unless
# that ref equals what `git ls-remote origin refs/heads/main` returns now. Run it from inside the
# repository checkout.
#
# Classes (per selected spec, see the "Amendment 1" section of
# .github/ISSUE_SPECS/1445-sync-edited-issue-specs-to-their-github-bodies-with-a-checked-operator-script.md):
#   closed-skipped  the issue is not OPEN; never edited
#   no-issue        no issue has that number; never edited
#   oversize        the source blob is above the byte ceiling, or its title is above 256 characters,
#                   or its title has a control character or leading or trailing whitespace
#   in-sync         the body matches the source blob (one trailing newline allowed) and the title
#                   equals the spec's title (or the spec has no title)
#   fast-forward    not in-sync, but body and title are the source's or an earlier committed state;
#                   a body that is such a blob with its H1 and the blank line after it removed
#                   counts as a body match too (Amendment 2, A10), for fast-forward only, never in-sync
#   unmatched       anything else; never edited (--reconcile-dir writes the evidence for it)
# --reviewed DIR (Amendment 2, A11) takes a directory that an earlier --reconcile-dir run wrote. An
# unmatched issue named explicitly on the command line (never through --all or --range) becomes
# `fast-forward reviewed` when its current GitHub body is byte-equal to DIR/<n>.github.md and its
# current title is byte-equal to DIR/<n>.github-title.txt. The write is unchanged.
# --apply writes the fast-forward set only. It never closes, reopens or comments, and it handles
# every body through files, never through an argument, an environment variable or a command
# substitution, so bytes and trailing newlines are exact.
#
# Exit status: 0 every selected spec is in-sync or closed-skipped at the end of the run; 3 the run
# completed but fast-forward (--check), unmatched, oversize or no-issue specs remain; 1 a write,
# read-back or race check failed; 2 a refusal before any write.
#
# Every gh call carries `--repo OWNER/REPO`, derived from `git remote get-url origin` (github.com
# URLs only; the script exits 2 if it cannot derive one), and GH_REPO is overwritten with the same
# value, so neither the environment nor `gh repo set-default` can redirect a read or a write.
# --backup-dir, --reconcile-dir and --reviewed are resolved against the caller's directory before the script
# changes into the repository root. The oversize class also holds a title that GitHub could
# normalize (a control character or CR, or leading or trailing whitespace); it is never edited.
#
# SYNC_SPEC_BODIES_TEST_CEILING may only lower the byte ceiling, and only the self-test sets it.
set -euo pipefail

CEILING_DEFAULT=261693
TITLE_MAX=256
SPEC_DIR=.github/ISSUE_SPECS

die2() {
    printf 'sync-spec-bodies: %s\n' "$*" >&2
    exit 2
}

usage() {
    printf 'usage: %s [--check | --apply --backup-dir DIR] [--reconcile-dir DIR] [--reviewed DIR] (--all | --range A..B | N...)\n' "$0" >&2
    exit 2
}

# --- arguments ---------------------------------------------------------------------------------
mode=check
mode_given=
select_all=
range=
backup_dir=
reconcile_dir=
reviewed_dir=
numbers=()
while (($#)); do
    case $1 in
    --check | --apply)
        [[ -z $mode_given ]] || die2 '--check and --apply are exclusive and may be given once'
        mode=${1#--}
        mode_given=1
        ;;
    --backup-dir)
        (($# >= 2)) || usage
        backup_dir=$2
        shift
        ;;
    --reconcile-dir)
        (($# >= 2)) || usage
        reconcile_dir=$2
        shift
        ;;
    --reviewed)
        (($# >= 2)) || usage
        reviewed_dir=$2
        shift
        ;;
    --all) select_all=1 ;;
    --range)
        (($# >= 2)) || usage
        range=$2
        shift
        ;;
    -*) usage ;;
    *)
        [[ $1 =~ ^[0-9]{1,9}$ ]] || die2 "not an issue number: $1"
        numbers+=("$((10#$1))")
        ;;
    esac
    shift
done
kinds=0
[[ -z $select_all ]] || kinds=$((kinds + 1))
[[ -z $range ]] || kinds=$((kinds + 1))
((${#numbers[@]} == 0)) || kinds=$((kinds + 1))
((kinds == 1)) || die2 'give exactly one of --all, --range A..B, or issue numbers'
if [[ $mode == apply ]]; then
    [[ -n $backup_dir ]] || die2 '--apply needs --backup-dir DIR'
else
    [[ -z $backup_dir ]] || die2 '--backup-dir is only for --apply'
fi
# Both directories are resolved against the caller's directory now, before the `cd` below, so the
# path that is checked is the path that is used.
abs_path() {
    case $1 in
    /*) printf '%s' "$1" ;;
    *) printf '%s/%s' "$PWD" "$1" ;;
    esac
}
[[ -z $backup_dir ]] || backup_dir=$(abs_path "$backup_dir")
[[ -z $reconcile_dir ]] || reconcile_dir=$(abs_path "$reconcile_dir")
[[ -z $reviewed_dir ]] || reviewed_dir=$(abs_path "$reviewed_dir")
if [[ -n $reviewed_dir ]]; then
    [[ -d $reviewed_dir ]] || die2 "--reviewed must be an existing directory: $reviewed_dir"
fi
if [[ -n $reconcile_dir ]]; then
    [[ ! -e $reconcile_dir && ! -L $reconcile_dir ]] || die2 "--reconcile-dir must not exist: $reconcile_dir"
fi

ceiling=$CEILING_DEFAULT
if [[ -n ${SYNC_SPEC_BODIES_TEST_CEILING-} ]]; then
    [[ $SYNC_SPEC_BODIES_TEST_CEILING =~ ^[0-9]{1,9}$ ]] || die2 'SYNC_SPEC_BODIES_TEST_CEILING is not a number'
    ((10#$SYNC_SPEC_BODIES_TEST_CEILING <= CEILING_DEFAULT)) || die2 'SYNC_SPEC_BODIES_TEST_CEILING may only lower the ceiling'
    ceiling=$((10#$SYNC_SPEC_BODIES_TEST_CEILING))
fi

for tool in awk cat cmp cp git gh head jq mktemp sort tail wc; do
    command -v "$tool" >/dev/null 2>&1 || die2 "required tool is unavailable: $tool"
done
repo=$(git rev-parse --show-toplevel) || die2 'not inside a git checkout'
cd "$repo"

# --- the GitHub repository: origin's, for every gh call -------------------------------------------
origin_url=$(git remote get-url origin 2>/dev/null) || die2 'cannot read the url of remote origin'
gh_owner=
gh_name=
if [[ $origin_url =~ ^(https|ssh|git)://([^/@]+@)?github\.com(:[0-9]+)?/([A-Za-z0-9._-]+)/([A-Za-z0-9._-]+)/?$ ]]; then
    gh_owner=${BASH_REMATCH[4]}
    gh_name=${BASH_REMATCH[5]}
elif [[ $origin_url =~ ^[A-Za-z0-9._-]+@github\.com:([A-Za-z0-9._-]+)/([A-Za-z0-9._-]+)/?$ ]]; then
    gh_owner=${BASH_REMATCH[1]}
    gh_name=${BASH_REMATCH[2]}
fi
gh_name=${gh_name%.git}
[[ -n $gh_owner && -n $gh_name && $gh_owner != *[!A-Za-z0-9_-]* && $gh_name != . && $gh_name != .. ]] ||
    die2 "cannot derive OWNER/REPO from the url of remote origin: $origin_url"
gh_repo=$gh_owner/$gh_name
export GH_REPO=$gh_repo

work=$(mktemp -d "${TMPDIR:-/tmp}/sync-spec-bodies.XXXXXX") || die2 'mktemp failed'
trap 'rm -rf "$work"' EXIT

# --- source: the origin/main blob, resolved here and proven current below ------------------------
src=$(git rev-parse --verify -q 'refs/remotes/origin/main^{commit}') || die2 'refs/remotes/origin/main does not exist'

# --- selection -----------------------------------------------------------------------------------
# Emits `number<TAB>path` for each top-level `<digits>-<slug>.md` spec on stdin (NUL-separated
# paths). A path with a directory below the spec directory is never an issue.
filter_specs() {
    local p base name
    while IFS= read -r -d '' p; do
        [[ $p == "$SPEC_DIR"/* ]] || continue
        base=${p#"$SPEC_DIR"/}
        [[ $base != */* ]] || continue
        name=${base##*/}
        [[ $name =~ ^0*([1-9][0-9]{0,8})-[A-Za-z0-9._-]+\.md$ ]] || continue
        printf '%s\t%s\n' "${BASH_REMATCH[1]}" "$p"
    done
}

if [[ -n $range ]]; then
    range_a=${range%%..*}
    range_b=${range#*..}
    [[ $range == *..* && -n $range_a && -n $range_b && $range_b != *..* && $range_b != .* ]] || die2 "--range must be A..B: $range"
    [[ $range_a != -* && $range_b != -* ]] || die2 "--range must be A..B: $range"
    range_b_hash=$(git rev-parse --verify -q "$range_b^{commit}") || die2 "cannot resolve $range_b"
    [[ $range_b_hash == "$src" ]] || die2 "--range must end at origin/main ($src), not $range_b_hash"
    range_a_hash=$(git rev-parse --verify -q "$range_a^{commit}") || die2 "cannot resolve $range_a"
    git diff --name-only -z --diff-filter=AM --no-renames "$range_a_hash" "$range_b_hash" -- "$SPEC_DIR/" |
        filter_specs >"$work/sel.raw"
else
    git ls-tree --name-only -z "$src" -- "$SPEC_DIR/" | filter_specs >"$work/sel.raw"
    if [[ -z $select_all ]]; then
        : >"$work/sel.pick"
        for n in "${numbers[@]}"; do
            awk -F'\t' -v n="$n" '$1 == n' "$work/sel.raw" >"$work/sel.one"
            [[ -s $work/sel.one ]] || die2 "no top-level spec for issue $n at origin/main"
            cat "$work/sel.one" >>"$work/sel.pick"
        done
        mv "$work/sel.pick" "$work/sel.raw"
    fi
fi
sort -u -k1,1n -k2,2 "$work/sel.raw" >"$work/sel.tsv"
dup=$(awk -F'\t' 'seen[$1]++ { print $1 }' "$work/sel.tsv")
[[ -z $dup ]] || die2 "more than one top-level spec file for issue $dup"

# --- backup directory pre-check (before ls-remote and before any GitHub call) --------------------
if [[ $mode == apply ]]; then
    if [[ -e $backup_dir || -L $backup_dir ]]; then
        [[ -d $backup_dir && ! -L $backup_dir ]] || die2 "--backup-dir is not a directory: $backup_dir"
    fi
    while IFS=$'\t' read -r n _; do
        [[ ! -e $backup_dir/$n.json && ! -L $backup_dir/$n.json ]] || die2 "backup file already exists: $backup_dir/$n.json"
    done <"$work/sel.tsv"
fi

# --- the source ref must be origin's main now (nothing is read from GitHub before this) ----------
git ls-remote origin refs/heads/main >"$work/ls-remote" 2>"$work/ls-remote.err" || die2 "git ls-remote origin failed: $(cat "$work/ls-remote.err")"
remote_hash=$(awk -F'\t' '$2 == "refs/heads/main" { print $1 }' "$work/ls-remote")
[[ $remote_hash =~ ^[0-9a-f]{40,64}$ ]] || die2 'git ls-remote origin returned no refs/heads/main'
[[ $remote_hash == "$src" ]] || die2 "refs/remotes/origin/main ($src) is not origin's main now ($remote_hash); fetch first"

# --- history: every blob each issue number's top-level spec file held, newest commit first -------
# hist.tsv: number<TAB>blob<TAB>commit<TAB>path, one row per distinct blob per number. Keyed by the
# issue number, so a renamed spec keeps its history.
git -c core.quotepath=false log --raw --root --no-abbrev --no-renames --full-history -m \
    --format='commit %H' "$src" -- "$SPEC_DIR/" |
    awk -F'\t' -v dir="$SPEC_DIR" '
        /^commit / { split($0, c, " "); commit = c[2]; next }
        /^:/ {
            split($1, f, " ")
            status = f[5]; blob = f[4]; path = $2
            if (status !~ /^[AMT]$/ || blob ~ /^0+$/) next
            prefix = dir "/"
            if (substr(path, 1, length(prefix)) != prefix) next
            name = substr(path, length(prefix) + 1)
            if (name ~ /\//) next
            if (name !~ /^[0-9]+-[A-Za-z0-9._-]+\.md$/) next
            num = name; sub(/-.*/, "", num); sub(/^0+/, "", num)
            if (num == "" || length(num) > 9) next
            key = num SUBSEP blob
            if (key in seen) next
            seen[key] = 1
            print num "\t" blob "\t" commit "\t" path
        }' >"$work/hist.tsv"

history_of() { # number: rows of hist.tsv for that issue, newest commit first
    awk -F'\t' -v k="$1" '$1 == k' "$work/hist.tsv"
}

read_source() { # path: the origin/main blob of a spec
    git cat-file blob "$src:$1"
}

# --- comparisons -----------------------------------------------------------------------------------
# matches_blob BODY BLOB: the GitHub body equals the blob, or equals it once one trailing newline is
# added back (GitHub drops one). Nothing else is normalized.
matches_blob() {
    cmp -s "$1" "$2" && return 0
    cp "$1" "$work/body-plus-newline"
    printf '\n' >>"$work/body-plus-newline"
    cmp -s "$work/body-plus-newline" "$2"
}

body_in_history() { # number bodyfile
    local h0 h1 blob
    h0=$(git hash-object --no-filters "$2")
    cp "$2" "$work/hist-body-plus-newline"
    printf '\n' >>"$work/hist-body-plus-newline"
    h1=$(git hash-object --no-filters "$work/hist-body-plus-newline")
    while IFS=$'\t' read -r _ blob _ _; do
        [[ $blob == "$h0" || $blob == "$h1" ]] || continue
        git cat-file blob "$blob" >"$work/hist-candidate"
        if matches_blob "$2" "$work/hist-candidate"; then
            return 0
        fi
    done < <(history_of "$1")
    return 1
}

# h1_removed_matches BODY BLOB: A10. The blob minus its first two lines, defined only when line 1 is
# `# <title>` (A2, with the same title sanity as the classification: at most TITLE_MAX characters, no
# control character, no leading or trailing whitespace) and line 2 is empty. The GitHub body matches
# it with the one-trailing-newline allowance of matches_blob. Used for fast-forward only.
h1_removed_matches() {
    local first second nl chars
    first=$(head -n1 "$2")
    [[ $first =~ ^'# '(.+)$ ]] || return 1
    printf '%s' "${BASH_REMATCH[1]}" >"$work/h1-title"
    nl=$(wc -l <"$2")
    ((nl >= 2)) || return 1
    second=$(head -n2 "$2" | tail -n1)
    [[ -z $second ]] || return 1
    tail -n +3 "$2" >"$work/h1-removed"
    matches_blob "$1" "$work/h1-removed" || return 1
    chars=$(jq -R length "$work/h1-title")
    ((chars <= TITLE_MAX)) || return 1
    ! title_unsafe "$work/h1-title"
}

# body_in_history_h1_removed NUMBER BODYFILE SRCFILE: the body is the H1-removed form of the source
# blob or of any blob in the issue's history.
body_in_history_h1_removed() {
    local blob
    h1_removed_matches "$2" "$3" && return 0
    while IFS=$'\t' read -r _ blob _ _; do
        git cat-file blob "$blob" >"$work/hist-candidate"
        if h1_removed_matches "$2" "$work/hist-candidate"; then
            return 0
        fi
    done < <(history_of "$1")
    return 1
}

title_in_history() { # number titlefile
    local blob line
    while IFS=$'\t' read -r _ blob _ _; do
        git cat-file blob "$blob" >"$work/hist-candidate"
        line=$(head -n1 "$work/hist-candidate")
        if [[ $line =~ ^'# '(.+)$ ]]; then
            printf '%s' "${BASH_REMATCH[1]}" >"$work/hist-title"
            if cmp -s "$2" "$work/hist-title"; then
                return 0
            fi
        fi
    done < <(history_of "$1")
    return 1
}

# --- GitHub reads ----------------------------------------------------------------------------------
# fetch_issue N OUTFILE: 0 fetched, 1 no such issue, 2 any other failure (message in gh.err).
fetch_issue() {
    local n=$1 out=$2
    if gh issue view "$n" --repo "$gh_repo" --json number,state,title,body >"$out" 2>"$work/gh.err"; then
        if jq -e --argjson n "$n" \
            'type == "object" and .number == $n and (.state | type == "string") and (.title | type == "string") and (.body | type == "string")' \
            "$out" >/dev/null 2>&1; then
            return 0
        fi
        printf 'unexpected JSON from gh issue view %s\n' "$n" >"$work/gh.err"
        return 2
    fi
    if grep -q 'Could not resolve to an issue' "$work/gh.err"; then
        return 1
    fi
    return 2
}

split_issue() { # jsonfile prefix: writes PREFIX.state PREFIX.title PREFIX.body
    jq -r .state "$1" >"$2.state"
    jq -j .title "$1" >"$2.title"
    jq -j .body "$1" >"$2.body"
}

# --- nearest history blob (for unmatched) --------------------------------------------------------
numstat_pair() { # fileA fileB: prints "added deleted"; binary or unreadable counts as huge
    local out rc=0 a d
    out=$(git -c core.quotepath=false diff --no-index --no-ext-diff --numstat -- "$1" "$2") || rc=$?
    ((rc <= 1)) || die2 "git diff --no-index failed ($rc)"
    if [[ -z $out ]]; then
        printf '0 0\n'
        return 0
    fi
    IFS=$'\t' read -r a d _ <<<"$out"
    if [[ ! $a =~ ^[0-9]+$ || ! $d =~ ^[0-9]+$ ]]; then
        a=1000000000
        d=0
    fi
    printf '%s %s\n' "$a" "$d"
}

reconcile() { # number dir: sets near_* and writes the evidence when --reconcile-dir is given
    local n=$1 d=$2 blob commit path a dd best=-1 total
    mkdir -p "$d/h"
    while IFS=$'\t' read -r _ blob commit path; do
        git cat-file blob "$blob" >"$d/h/$blob"
        numstat_pair "$d/body.gh" "$d/h/$blob" >"$work/numstat"
        read -r a dd <"$work/numstat"
        total=$((a + dd))
        if ((best < 0 || total < best)); then
            best=$total
            near_blob=$blob
            near_commit=$commit
            near_path=$path
            near_gh="+$a/-$dd"
        fi
    done < <(history_of "$n")
    if ((best < 0)); then
        near_blob=
        return 0
    fi
    numstat_pair "$d/h/$near_blob" "$d/src.md" >"$work/numstat"
    read -r a dd <"$work/numstat"
    near_src="+$a/-$dd"
    if [[ -n $reconcile_dir ]]; then
        mkdir -p "$reconcile_dir"
        cp "$d/body.gh" "$reconcile_dir/$n.github.md"
        cp "$d/title.gh" "$reconcile_dir/$n.github-title.txt"
        cp "$d/src.md" "$reconcile_dir/$n.source.md"
        cp "$d/h/$near_blob" "$reconcile_dir/$n.nearest.md"
        printf 'nearest blob %s commit %s path %s; github-vs-nearest %s, nearest-vs-source %s\n' \
            "$near_blob" "$near_commit" "$near_path" "$near_gh" "$near_src" >"$reconcile_dir/$n.header.txt"
    fi
}

# title_unsafe TITLEFILE: the title has a control character (CR included) or leading or trailing
# whitespace, which GitHub may normalize; such a title is refused before any write.
title_unsafe() {
    local LC_ALL=C t
    t=$(<"$1")
    [[ $t =~ [[:cntrl:]] || $t =~ ^[[:space:]] || $t =~ [[:space:]]$ ]]
}

# reviewed_unchanged NUMBER: A11. Only for an issue named explicitly on the command line, and only
# while the GitHub body and title are still byte-equal to what the reviewer saw (files that an
# earlier --reconcile-dir run wrote).
reviewed_unchanged() {
    local n=$1 d="$work/$1"
    [[ -n $reviewed_dir ]] && ((${#numbers[@]} > 0)) || return 1
    [[ -f $reviewed_dir/$n.github.md && -f $reviewed_dir/$n.github-title.txt ]] || return 1
    cmp -s "$d/body.gh" "$reviewed_dir/$n.github.md" && cmp -s "$d/title.gh" "$reviewed_dir/$n.github-title.txt"
}

# --- classification ----------------------------------------------------------------------------------
cnt_in_sync=0 cnt_fast_forward=0 cnt_closed_skipped=0 cnt_no_issue=0 cnt_oversize=0 cnt_unmatched=0
oversize_why=
: >"$work/ff.list"

classify() { # number path
    local n=$1 path=$2 d="$work/$n" class size title_chars title_bad= body_ok= title_ok= has_title= why tag=
    mkdir -p "$d"
    read_source "$path" >"$d/src.md"
    local rc=0
    fetch_issue "$n" "$d/cls.json" || rc=$?
    if ((rc == 1)); then
        class=no-issue
    elif ((rc == 2)); then
        die2 "gh issue view $n failed: $(cat "$work/gh.err")"
    else
        split_issue "$d/cls.json" "$d/gh"
        mv "$d/gh.title" "$d/title.gh"
        mv "$d/gh.body" "$d/body.gh"
        state=$(cat "$d/gh.state")
        local line
        line=$(head -n1 "$d/src.md")
        if [[ $line =~ ^'# '(.+)$ ]]; then
            has_title=1
            printf '%s' "${BASH_REMATCH[1]}" >"$d/title.spec"
        fi
        size=$(wc -c <"$d/src.md")
        title_chars=0
        title_bad=
        if [[ -n $has_title ]]; then
            title_chars=$(jq -R length "$d/title.spec")
            if title_unsafe "$d/title.spec"; then
                title_bad=1
            fi
        fi
        if [[ $state != OPEN ]]; then
            class=closed-skipped
        elif ((size > ceiling || title_chars > TITLE_MAX)) || [[ -n $title_bad ]]; then
            class=oversize
            why=
            ((size <= ceiling)) || why="$why size=above-ceiling"
            ((title_chars <= TITLE_MAX)) || why="$why title=above-$TITLE_MAX-characters"
            [[ -z $title_bad ]] || why="$why title=control-character-or-edge-whitespace"
            oversize_why=$why
        else
            body_ok=
            title_ok=
            matches_blob "$d/body.gh" "$d/src.md" && body_ok=1
            if [[ -z $has_title ]] || cmp -s "$d/title.gh" "$d/title.spec"; then
                title_ok=1
            fi
            if [[ -n $body_ok && -n $title_ok ]]; then
                class=in-sync
            else
                if [[ -z $body_ok ]] && body_in_history "$n" "$d/body.gh"; then
                    body_ok=1
                fi
                if [[ -z $body_ok ]] && body_in_history_h1_removed "$n" "$d/body.gh" "$d/src.md"; then
                    body_ok=1
                fi
                if [[ -z $title_ok ]] && title_in_history "$n" "$d/title.gh"; then
                    title_ok=1
                fi
                if [[ -n $body_ok && -n $title_ok ]]; then
                    class=fast-forward
                else
                    class=unmatched
                    if reviewed_unchanged "$n"; then
                        class=fast-forward
                        tag=' reviewed'
                    fi
                fi
            fi
        fi
    fi
    case $class in
    in-sync) cnt_in_sync=$((cnt_in_sync + 1)) ;;
    fast-forward)
        cnt_fast_forward=$((cnt_fast_forward + 1))
        printf '%s\t%s\n' "$n" "$path" >>"$work/ff.list"
        ;;
    closed-skipped) cnt_closed_skipped=$((cnt_closed_skipped + 1)) ;;
    no-issue) cnt_no_issue=$((cnt_no_issue + 1)) ;;
    oversize) cnt_oversize=$((cnt_oversize + 1)) ;;
    unmatched) cnt_unmatched=$((cnt_unmatched + 1)) ;;
    esac
    if [[ $class == unmatched ]]; then
        near_blob=
        reconcile "$n" "$d"
        why=
        [[ -n $body_ok ]] || why="$why body=no-committed-state-matches"
        [[ -n $title_ok ]] || why="$why title=no-committed-title-matches"
        if [[ -n $near_blob ]]; then
            printf '%s unmatched nearest %s %s github-vs-nearest %s nearest-vs-source %s%s\n' \
                "$n" "$near_commit" "$near_path" "$near_gh" "$near_src" "$why"
        else
            printf '%s unmatched%s\n' "$n" "$why"
        fi
    elif [[ $class == oversize ]]; then
        printf '%s oversize%s\n' "$n" "$oversize_why"
    else
        printf '%s %s%s\n' "$n" "$class" "$tag"
    fi
}

while IFS=$'\t' read -r n path; do
    classify "$n" "$path"
done <"$work/sel.tsv"

# --- the write: serial, one issue at a time, ascending -------------------------------------------
synced=0

backup_saved=
save_backup() { # jsonfile number: never overwrites a file (noclobber opens with O_EXCL)
    mkdir -p "$backup_dir"
    (
        set -o noclobber
        cat -- "$1" >"$backup_dir/$2.json"
    ) 2>/dev/null || return 1
    cmp -s "$1" "$backup_dir/$2.json" || return 1
    backup_saved=$2
}

stop_failed() { # number word detail
    printf '%s %s\n' "$1" "$2"
    if [[ $backup_saved == "$1" ]]; then
        printf 'sync-spec-bodies: %s: %s; no further write. Rollback copy: %s/%s.json\n' "$1" "$3" "$backup_dir" "$1" >&2
    else
        printf 'sync-spec-bodies: %s: %s; no further write. Nothing was written to issue %s, and no rollback copy exists for it.\n' "$1" "$3" "$1" >&2
    fi
    exit 1
}

apply_one() { # number
    local n=$1 d="$work/$1" rc=0
    fetch_issue "$n" "$d/pre.json" || rc=$?
    ((rc == 0)) || stop_failed "$n" refused "re-fetch failed: $(cat "$work/gh.err")"
    split_issue "$d/pre.json" "$d/pre"
    [[ $(cat "$d/pre.state") == OPEN ]] || stop_failed "$n" refused 'issue is no longer OPEN'
    if ! { cmp -s "$d/pre.title" "$d/title.gh" && cmp -s "$d/pre.body" "$d/body.gh"; }; then
        stop_failed "$n" refused 'title or body changed since classification'
    fi
    save_backup "$d/pre.json" "$n" || stop_failed "$n" refused 'could not save the rollback copy'
    local args=(issue edit "$n" --repo "$gh_repo" --body-file "$d/src.md")
    if [[ -f $d/title.spec ]] && ! cmp -s "$d/title.gh" "$d/title.spec"; then
        local title
        title=$(<"$d/title.spec")
        args+=(--title "$title")
    fi
    gh "${args[@]}" >/dev/null 2>"$work/gh.err" || stop_failed "$n" MISMATCH "gh issue edit failed: $(cat "$work/gh.err")"
    rc=0
    fetch_issue "$n" "$d/post.json" || rc=$?
    ((rc == 0)) || stop_failed "$n" MISMATCH "read-back failed: $(cat "$work/gh.err")"
    split_issue "$d/post.json" "$d/post"
    [[ $(cat "$d/post.state") == OPEN ]] || stop_failed "$n" MISMATCH 'issue is not OPEN after the edit'
    matches_blob "$d/post.body" "$d/src.md" || stop_failed "$n" MISMATCH 'read-back body differs from the spec'
    if [[ -f $d/title.spec ]]; then
        cmp -s "$d/post.title" "$d/title.spec" || stop_failed "$n" MISMATCH 'read-back title differs from the spec'
    else
        cmp -s "$d/post.title" "$d/title.gh" || stop_failed "$n" MISMATCH 'title changed although the spec has none'
    fi
    printf '%s synced\n' "$n"
    synced=$((synced + 1))
}

if [[ $mode == apply ]]; then
    while IFS=$'\t' read -r n _; do
        apply_one "$n"
    done <"$work/ff.list"
    if ((synced != cnt_fast_forward)); then
        printf 'sync-spec-bodies: internal error: synced %s of %s fast-forward issues\n' "$synced" "$cnt_fast_forward" >&2
        exit 1
    fi
fi

# --- summary and exit status ---------------------------------------------------------------------------
remaining=$((cnt_unmatched + cnt_oversize + cnt_no_issue))
if [[ $mode == check ]]; then
    remaining=$((remaining + cnt_fast_forward))
fi
printf 'summary in-sync=%s fast-forward=%s closed-skipped=%s no-issue=%s oversize=%s unmatched=%s synced=%s\n' \
    "$cnt_in_sync" "$cnt_fast_forward" "$cnt_closed_skipped" "$cnt_no_issue" "$cnt_oversize" "$cnt_unmatched" "$synced"
if ((remaining > 0)); then
    exit 3
fi
exit 0
