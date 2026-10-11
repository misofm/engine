#!/usr/bin/env bash
# Self-test of sync-spec-bodies.sh (issue #1445, amendment 1, item A8).
#
# Invoked by hand (and by an operator before a first use); no workflow runs it. It builds a
# temporary git repository with a bare local `origin` (so `git ls-remote` never leaves the machine)
# and puts a stub `gh` first on PATH. The stub serves issues from files, logs every argv, and
# fails the test on any command it does not model, so the real `gh` and the network are never
# reached. It prints `ok - NAME` or `FAIL - NAME` per assertion, removes its temporary directory
# on exit, and exits 1 if any assertion failed.
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
SCRIPT=$here/sync-spec-bodies.sh
[[ -f $SCRIPT ]] || { printf 'missing %s\n' "$SCRIPT" >&2; exit 2; }

T=$(mktemp -d "${TMPDIR:-/tmp}/test-sync-spec-bodies.XXXXXX")
trap 'rm -rf "$T"' EXIT
R=$T/repo
O=$T/origin.git
G=$T/gh
mkdir -p "$T/bin" "$G/issues" "$T/bodies"

export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
export GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@example.invalid GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@example.invalid

failures=0
ok() { printf 'ok - %s\n' "$1"; }
bad() { printf 'FAIL - %s\n' "$1"; failures=$((failures + 1)); }
assert() { # name command...
    local name=$1
    shift
    if "$@"; then ok "$name"; else bad "$name"; fi
}

# --- the stub gh ------------------------------------------------------------------------------------
cat >"$T/bin/gh" <<'STUB'
#!/usr/bin/env bash
# Models exactly: `issue view N --repo test-owner/test-repo --json number,state,title,body` and
# `issue edit N --repo test-owner/test-repo --body-file F [--title T]`. Anything else (another
# --repo included) is logged and fails with status 99.
d=${GH_STUB_DIR:?}
printf '%s\n' "$*" >>"$d/log"
printf '%s\n' "${GH_REPO-unset}" >>"$d/ghrepo-env"
unmodelled() {
    printf '%s\n' "$*" >>"$d/unmodelled"
    echo "gh stub: unmodelled command: $*" >&2
    exit 99
}
[[ ${1-} == issue ]] || unmodelled "$@"
case ${2-} in
view)
    [[ $# == 7 && $3 =~ ^[0-9]+$ && $4 == --repo && $5 == test-owner/test-repo && $6 == --json && $7 == number,state,title,body ]] || unmodelled "$@"
    n=$3
    f=$d/issues/$n.json
    if [[ ! -f $f ]]; then
        echo "GraphQL: Could not resolve to an issue or pull request with the number of $n. (repository.issue)" >&2
        exit 1
    fi
    c=$(($(cat "$d/views-$n" 2>/dev/null || echo 0) + 1))
    echo "$c" >"$d/views-$n"
    if [[ -f $d/race-$n && $c -ge 2 ]]; then
        rm "$d/race-$n"
        jq -c '.body = "edited on GitHub meanwhile"' "$f" >"$f.new"
        mv "$f.new" "$f"
    fi
    if [[ -f $d/close-race-$n && $c -ge 2 ]]; then
        rm "$d/close-race-$n"
        jq -c '.state = "CLOSED"' "$f" >"$f.new"
        mv "$f.new" "$f"
    fi
    if [[ -f $d/plant-$n && $c -ge 2 ]]; then
        rm "$d/plant-$n"
        mkdir -p "${STUB_BACKUP_DIR:?}"
        printf 'planted by a concurrent writer\n' >"$STUB_BACKUP_DIR/$n.json"
    fi
    cat "$f"
    ;;
edit)
    [[ $3 =~ ^[0-9]+$ && $4 == --repo && $5 == test-owner/test-repo && $6 == --body-file && -f $7 ]] || unmodelled "$@"
    n=$3
    title=
    if [[ $# == 9 ]]; then
        [[ $8 == --title ]] || unmodelled "$@"
        title=$9
    else
        [[ $# == 7 ]] || unmodelled "$@"
    fi
    f=$d/issues/$n.json
    [[ -f $f ]] || unmodelled "$@"
    printf 'editcheck %s backup-present=%s\n' "$n" "$([[ -f ${STUB_BACKUP_DIR:-/nonexistent}/$n.json ]] && echo yes || echo no)" >>"$d/editcheck"
    keep=false
    [[ ! -f $d/keepnl ]] || keep=true
    jq -c --rawfile b "$7" --argjson keep "$keep" \
        '.body = (if $keep then $b else ($b | sub("\\n\\z"; "")) end)' "$f" >"$f.new"
    mv "$f.new" "$f"
    if [[ -f $d/corrupt-$n ]]; then
        jq -c '.body += "X"' "$f" >"$f.new"
        mv "$f.new" "$f"
    fi
    if [[ -n $title ]]; then
        jq -c --arg t "$title" '.title = $t' "$f" >"$f.new"
        mv "$f.new" "$f"
    fi
    if [[ -f $d/corrupt-title-$n ]]; then
        jq -c '.title += "X"' "$f" >"$f.new"
        mv "$f.new" "$f"
    fi
    if [[ -f $d/close-after-$n ]]; then
        jq -c '.state = "CLOSED"' "$f" >"$f.new"
        mv "$f.new" "$f"
    fi
    echo "https://example.invalid/issues/$n"
    ;;
*) unmodelled "$@" ;;
esac
STUB
chmod +x "$T/bin/gh"

# A git wrapper that (a) answers `git remote get-url origin` with a github.com URL, because the test
# origin is a local bare repository (STUB_ORIGIN_URL overrides it), (b) logs each `ls-remote`, and
# (c) hands every other command to the real git.
REAL_GIT=$(command -v git)
export REAL_GIT
cat >"$T/bin/git" <<'STUB'
#!/usr/bin/env bash
d=${GH_STUB_DIR:?}
if [[ ${1-} == ls-remote ]]; then
    printf 'ls-remote\n' >>"$d/gitlog"
fi
if [[ ${1-} == remote && ${2-} == get-url && ${3-} == origin && $# == 3 ]]; then
    printf '%s\n' "${STUB_ORIGIN_URL:-https://github.com/test-owner/test-repo.git}"
    exit 0
fi
exec "${REAL_GIT:?}" "$@"
STUB
chmod +x "$T/bin/git"

# --- the repository --------------------------------------------------------------------------------
tick=0
commit() { # message
    tick=$((tick + 1))
    export GIT_AUTHOR_DATE="2026-01-01T00:00:0$tick +0000" GIT_COMMITTER_DATE="2026-01-01T00:00:0$tick +0000"
    git -C "$R" add -A .github
    git -C "$R" commit -q -m "$1"
    git -C "$R" rev-parse HEAD
}
S=$R/.github/ISSUE_SPECS
git init -q --bare -b main "$O"
git init -q -b main "$R"
git -C "$R" remote add origin "$O"
mkdir -p "$S/BRIEFS"

printf '# Ten\n\nten v1 line\nshared\n' >"$S/10-sync.md"
printf '# Eleven\n\neleven\n' >"$S/11-insync.md"
printf '# Twelve\n\ntwelve v1\n' >"$S/12-closed.md"
printf '# Thirteen\n\nno issue\n' >"$S/13-gone.md"
printf '# Fourteen\n\nsmall\n' >"$S/14-big.md"
printf '# Fifteen\n\na1\na2\na3\n' >"$S/15-unmatched.md"
printf '# Sixteen\n\nsixteen v1\n' >"$S/16-rename-old.md"
printf '# Seventeen\n\nseventeen\n' >"$S/17-title.md"
printf '# Eighteen old\n\neighteen\n' >"$S/18-tonly.md"
printf '# Twenty\n\ntwenty\n' >"$S/20-newline.md"
printf '# Brief 36 v1\n' >"$S/BRIEFS/036-x.md"
printf 'index v1\n' >"$S/README.md"
# Titles GitHub could normalize, and the 256-character title limit.
printf '# Twenty-one \n\nt21 v1\n' >"$S/21-title-trailing-space.md"
printf '# Twenty-two\r\n\nt22 v1\n' >"$S/22-title-cr.md"
printf '#  Twenty-three\n\nt23 v1\n' >"$S/23-title-leading-space.md"
printf '# Twenty-four\ax\n\nt24\n' >"$S/24-title-bell.md"
T257=$(printf 'a%.0s' $(seq 1 257))
T256=${T257%a}
printf '# %s\n\nt25\n' "$T257" >"$S/25-title-257.md"
printf '# %s\n\nt26\n' "$T256" >"$S/26-title-256.md"
# Bodies that are another issue's committed blob, or a BRIEFS/ blob.
printf '# Twenty-seven\n\nspec 27\n' >"$S/27-other-issues-blob.md"
printf '# Twenty-nine\n\nspec 29\n' >"$S/29-brief-blob.md"
printf '# Brief 29\n\nbrief 29 body\n' >"$S/BRIEFS/029-y.md"
# A spec with no H1: its title is never changed.
printf 'no heading v1\n' >"$S/30-no-h1.md"
C1=$(commit 'specs v1')

printf '# Ten\n\nten v2 line\nshared\n' >"$S/10-sync.md"
printf '# Twelve\n\ntwelve v2\n' >"$S/12-closed.md"
{ printf '# Fourteen\n\n'; for _ in 1 2 3 4 5 6 7 8; do printf '%s\n' 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'; done; } >"$S/14-big.md"
printf '# Fifteen\n\na1\na2 changed\na3\n' >"$S/15-unmatched.md"
git -C "$R" mv "$S/16-rename-old.md" "$S/16-rename-new.md"
printf '# Sixteen renamed\n\nsixteen v2\n' >"$S/16-rename-new.md"
printf '# Eighteen\n\neighteen\n' >"$S/18-tonly.md"
printf '# Brief 36 v2\n' >"$S/BRIEFS/036-x.md"
printf 'index v2\n' >"$S/README.md"
printf '# Twenty-one \n\nt21 v2\n' >"$S/21-title-trailing-space.md"
printf '# Twenty-two\r\n\nt22 v2\n' >"$S/22-title-cr.md"
printf '#  Twenty-three\n\nt23 v2\n' >"$S/23-title-leading-space.md"
printf 'no heading v2\n' >"$S/30-no-h1.md"
C2=$(commit 'specs v2, rename 16')

printf '# Fifteen\n\na1\na2 changed\na3 changed\na4\n' >"$S/15-unmatched.md"
C3=$(commit 'spec 15 v3')
git -C "$R" push -q origin main

# An unmerged branch holds a body no commit reachable from origin/main ever held.
git -C "$R" checkout -q -b feature
printf '# Fifteen\n\na1\na2 changed\na3\nbranch-only line\n' >"$S/15-unmatched.md"
CF=$(commit 'unmerged edit of 15')
git -C "$R" checkout -q main
# The working tree copy of spec 10 differs from origin/main's blob.
printf '# Ten\n\nWORKING TREE COPY\n' >"$S/10-sync.md"

blob() { git -C "$R" show "$1:.github/ISSUE_SPECS/$2"; }
strip_nl() { head -c -1; } # GitHub drops one trailing newline

blob "$C1" 10-sync.md | strip_nl >"$T/bodies/10"
blob "$C3" 11-insync.md | strip_nl >"$T/bodies/11"
blob "$C1" 12-closed.md | strip_nl >"$T/bodies/12"
blob "$C1" 14-big.md | strip_nl >"$T/bodies/14"
blob "$CF" 15-unmatched.md | strip_nl >"$T/bodies/15"
blob "$C1" 16-rename-old.md | strip_nl >"$T/bodies/16"
blob "$C3" 17-title.md | strip_nl >"$T/bodies/17"
blob "$C3" 18-tonly.md | strip_nl >"$T/bodies/18"
{ blob "$C3" 20-newline.md | strip_nl; printf '\n\n'; } >"$T/bodies/20"
printf 'a brief body\n' >"$T/bodies/36"
blob "$C3" 20-newline.md >"$T/bodies/20.src"
blob "$C1" 21-title-trailing-space.md | strip_nl >"$T/bodies/21"
blob "$C1" 22-title-cr.md | strip_nl >"$T/bodies/22"
blob "$C1" 23-title-leading-space.md | strip_nl >"$T/bodies/23"
blob "$C3" 24-title-bell.md | strip_nl >"$T/bodies/24"
blob "$C3" 25-title-257.md | strip_nl >"$T/bodies/25"
blob "$C3" 26-title-256.md | strip_nl >"$T/bodies/26"
blob "$C3" 11-insync.md | strip_nl >"$T/bodies/27" # another issue's committed blob
blob "$C1" BRIEFS/029-y.md | strip_nl >"$T/bodies/29" # a BRIEFS/ blob
blob "$C1" 30-no-h1.md | strip_nl >"$T/bodies/30"

mk_issue() { # n state title bodyfile
    jq -n -c --argjson n "$1" --arg s "$2" --arg t "$3" --rawfile b "$4" \
        '{number: $n, state: $s, title: $t, body: $b}' >"$G/issues/$1.json"
}
reset_issues() {
    rm -rf "$G/issues" "$G"/race-* "$G"/corrupt-* "$G"/close-* "$G"/plant-* "$G/keepnl"
    mkdir -p "$G/issues"
    mk_issue 10 OPEN 'Ten' "$T/bodies/10"
    mk_issue 11 OPEN 'Eleven' "$T/bodies/11"
    mk_issue 12 CLOSED 'Twelve' "$T/bodies/12"
    mk_issue 14 OPEN 'Fourteen' "$T/bodies/14"
    mk_issue 15 OPEN 'Fifteen' "$T/bodies/15"
    mk_issue 16 OPEN 'Sixteen' "$T/bodies/16"
    mk_issue 17 OPEN 'Seventeen hand edited' "$T/bodies/17"
    mk_issue 18 OPEN 'Eighteen old' "$T/bodies/18"
    mk_issue 20 OPEN 'Twenty' "$T/bodies/20"
    mk_issue 21 OPEN 'Twenty-one ' "$T/bodies/21"
    mk_issue 22 OPEN $'Twenty-two\r' "$T/bodies/22"
    mk_issue 23 OPEN ' Twenty-three' "$T/bodies/23"
    mk_issue 24 OPEN $'Twenty-four\ax' "$T/bodies/24"
    mk_issue 25 OPEN "$T257" "$T/bodies/25"
    mk_issue 26 OPEN "$T256" "$T/bodies/26"
    mk_issue 27 OPEN 'Twenty-seven' "$T/bodies/27"
    mk_issue 29 OPEN 'Twenty-nine' "$T/bodies/29"
    mk_issue 30 OPEN 'Thirty, hand titled' "$T/bodies/30"
    mk_issue 36 OPEN 'Thirty-six' "$T/bodies/36"
    rm -rf "$T/snap"
    cp -r "$G/issues" "$T/snap"
}

rc=0
CEILING=
STUB_BACKUP_DIR=
RUNCWD=
STUB_ORIGIN_URL=
run() { # args...: runs the tool inside RUNCWD (default: the repository) against the stub
    : >"$G/log"
    rm -f "$G/unmodelled" "$G/editcheck" "$G/gitlog" "$G/ghrepo-env" "$G"/views-*
    rc=0
    (
        cd "${RUNCWD:-$R}"
        PATH="$T/bin:$PATH" GH_STUB_DIR=$G STUB_BACKUP_DIR=$STUB_BACKUP_DIR STUB_ORIGIN_URL=$STUB_ORIGIN_URL \
            SYNC_SPEC_BODIES_TEST_CEILING=$CEILING bash "$SCRIPT" "$@"
    ) >"$T/out" 2>"$T/err" || rc=$?
    if [[ -e $G/unmodelled ]]; then
        bad "stub saw an unmodelled gh command (run: $*)"
    fi
}
has_line() { grep -Eq "^$1( |\$)" "$T/out"; }
edited() { awk '$1 == "issue" && $2 == "edit" { printf "%s ", $3 }' "$G/log"; }
viewed() { awk '$1 == "issue" && $2 == "view" { printf "%s ", $3 }' "$G/log"; }
no_gh_call() { [[ ! -s $G/log ]]; }
log_shapes_ok() {
    ! grep -Ev '^issue view [0-9]+ --repo test-owner/test-repo --json number,state,title,body$|^issue edit [0-9]+ --repo test-owner/test-repo --body-file /[^ ]+( --title .+)?$' "$G/log" | grep -q .
}
log_has() { grep -Eq "$1" "$G/log"; }

# --- A: --check --all, default ceiling -----------------------------------------------------------
reset_issues
run --check --all
assert 'check-exit-3-while-work-remains' test "$rc" = 3
assert 'class-fast-forward-body-in-history' has_line '10 fast-forward'
assert 'class-in-sync' has_line '11 in-sync'
assert 'class-closed-skipped' has_line '12 closed-skipped'
assert 'class-no-issue' has_line '13 no-issue'
assert 'default-ceiling-keeps-14-in-play' has_line '14 fast-forward'
assert 'renamed-spec-body-is-fast-forward' has_line '16 fast-forward'
assert 'class-unmatched-branch-only-blob' has_line '15 unmatched'
assert 'class-unmatched-hand-edited-title' has_line '17 unmatched'
assert 'class-fast-forward-title-in-history' has_line '18 fast-forward'
assert 'extra-newline-is-unmatched' has_line '20 unmatched'
assert 'unmatched-line-names-nearest-commit' grep -q "^15 unmatched nearest $C2 .github/ISSUE_SPECS/15-unmatched.md " "$T/out"
assert 'check-no-edit' test -z "$(edited)"
assert 'check-gh-calls-are-views-only' log_shapes_ok
assert 'briefs-not-issue-36-in-all' test "$(viewed | grep -c '\b36\b' || true)" = 0
assert 'summary-counts' grep -q '^summary in-sync=2 fast-forward=5 closed-skipped=1 no-issue=1 oversize=5 unmatched=5 synced=0$' "$T/out"
assert 'class-no-h1-spec-body-in-history-is-fast-forward' has_line '30 fast-forward'
assert 'class-title-256-characters-is-in-sync' has_line '26 in-sync'
assert 'class-title-257-characters-is-oversize' has_line '25 oversize'
assert 'class-title-trailing-space-is-oversize' has_line '21 oversize'
assert 'class-title-cr-is-oversize' has_line '22 oversize'
assert 'class-title-embedded-control-character-is-oversize' has_line '24 oversize'
assert 'class-title-leading-space-is-oversize' has_line '23 oversize'
assert 'unsafe-title-line-names-the-reason' grep -q '^21 oversize title=control-character-or-edge-whitespace$' "$T/out"
assert 'history-is-keyed-by-issue-number' has_line '27 unmatched'
assert 'brief-blob-is-not-history' has_line '29 unmatched\b'

# --- B: --check --all, lowered ceiling -----------------------------------------------------------
reset_issues
CEILING=300 run --check --all
assert 'oversize-class' has_line '14 oversize'
assert 'oversize-leaves-others' has_line '10 fast-forward'
CEILING=999999999 run --check --all
assert 'ceiling-cannot-be-raised' test "$rc" = 2
CEILING=

# --- C: --apply writes the fast-forward set only --------------------------------------------------
reset_issues
B1=$T/B1
STUB_BACKUP_DIR=$B1 run --apply --backup-dir "$B1" --all
assert 'apply-exit-3-unmatched-remain' test "$rc" = 3
assert 'apply-edits-exactly-the-fast-forward-set' test "$(edited)" = '10 14 16 18 30 '
assert 'apply-reports-synced' bash -c 'grep -q "^10 synced$" "$1" && grep -q "^14 synced$" "$1" && grep -q "^16 synced$" "$1" && grep -q "^18 synced$" "$1" && grep -q "^30 synced$" "$1"' _ "$T/out"
assert 'edit-argv-shape' log_shapes_ok
assert 'edit-title-only-when-it-differs' bash -c '! grep -E "^issue edit (10|14) .*--title" "$1" && grep -Eq "^issue edit 16 --repo test-owner/test-repo --body-file /[^ ]+ --title Sixteen renamed$" "$1" && grep -Eq "^issue edit 18 --repo test-owner/test-repo --body-file /[^ ]+ --title Eighteen$" "$1" && ! grep -E "^issue edit 30 .*--title" "$1"' _ "$G/log"
assert 'closed-never-edited' bash -c '! grep -E "^issue edit 12 " "$1"' _ "$G/log"
assert 'unmatched-never-edited' bash -c '! grep -E "^issue edit (15|17|20|21|22|23|24|25|26|27|29) " "$1"' _ "$G/log"
assert 'no-issue-and-brief-never-edited' bash -c '! grep -E "^issue edit (13|36) " "$1"' _ "$G/log"
assert 'backup-saved-before-each-edit' test "$(grep -c 'backup-present=yes' "$G/editcheck")" = 5
assert 'backup-equals-served-raw-json-10' cmp -s "$B1/10.json" "$T/snap/10.json"
assert 'backup-equals-served-raw-json-16' cmp -s "$B1/16.json" "$T/snap/16.json"
assert 'backup-holds-fast-forward-set-only' test "$(ls "$B1" | tr '\n' ' ')" = '10.json 14.json 16.json 18.json 30.json '
blob "$C3" 10-sync.md | strip_nl >"$T/expect10"
jq -j .body "$G/issues/10.json" >"$T/stored10"
assert 'source-is-origin-main-blob-not-working-tree' cmp -s "$T/expect10" "$T/stored10"
assert 'edited-title-landed' test "$(jq -r .title "$G/issues/16.json")" = 'Sixteen renamed'
reset_after=$(cat "$G/issues/12.json")
assert 'closed-issue-untouched' test "$reset_after" = "$(cat "$T/snap/12.json")"
run --check --all
assert 'after-apply-fast-forward-set-is-in-sync' bash -c 'grep -q "^10 in-sync" "$1" && grep -q "^14 in-sync" "$1" && grep -q "^16 in-sync" "$1" && grep -q "^18 in-sync" "$1" && grep -q "^30 in-sync" "$1"' _ "$T/out"
assert 'after-apply-unmatched-still-unmatched' has_line '15 unmatched'

# --- D: refusals before any GitHub call -----------------------------------------------------------
reset_issues
mkdir -p "$T/B2"
: >"$T/B2/10.json"
run --apply --backup-dir "$T/B2" --all
assert 'existing-backup-file-refuses-the-run' test "$rc" = 2
assert 'existing-backup-file-refuses-before-any-gh-call' no_gh_call
run --apply --all
assert 'apply-without-backup-dir-refused' test "$rc" = 2
assert 'apply-without-backup-dir-no-gh-call' no_gh_call

# --- E: oversize is never edited -----------------------------------------------------------------
reset_issues
CEILING=300 run --apply --backup-dir "$T/B3" --all
assert 'oversize-never-edited' bash -c '! grep -E "^issue edit 14 " "$1"' _ "$G/log"
assert 'oversize-other-fast-forwards-still-written' test "$(edited)" = '10 16 18 30 '
CEILING=

# --- F: a body that changed between classification and the write ----------------------------------
reset_issues
: >"$G/race-10"
run --apply --backup-dir "$T/B4" 10
assert 'race-refused-exit-1' test "$rc" = 1
assert 'race-refused-line' has_line '10 refused'
assert 'race-no-edit' test -z "$(edited)"
assert 'race-no-backup-written' test ! -e "$T/B4/10.json"

# --- G: a read-back that differs by one byte fails and stops later writes -------------------------
reset_issues
: >"$G/corrupt-10"
STUB_BACKUP_DIR=$T/B5 run --apply --backup-dir "$T/B5" 10 14 16
assert 'readback-one-byte-fails' test "$rc" = 1
assert 'readback-mismatch-line' has_line '10 MISMATCH'
assert 'readback-stops-later-writes' test "$(edited)" = '10 '
assert 'readback-names-the-backup-file' grep -q "$T/B5/10.json" "$T/err"
assert 'readback-failure-keeps-the-backup' cmp -s "$T/B5/10.json" "$T/snap/10.json"

# --- H: one trailing newline of difference passes, both ways --------------------------------------
reset_issues
run --apply --backup-dir "$T/B6" 10
assert 'readback-trailing-newline-dropped-passes' test "$rc" = 0
assert 'readback-trailing-newline-dropped-synced' has_line '10 synced'
reset_issues
: >"$G/keepnl"
run --apply --backup-dir "$T/B7" 10
assert 'readback-exact-bytes-passes' test "$rc" = 0

# --- I: --range, and the brief under BRIEFS/ ------------------------------------------------------
reset_issues
run --check --range "$C1..origin/main"
assert 'range-selects-changed-top-level-specs' test "$(awk '$2 != "summary" && $1 != "summary" { printf "%s ", $1 }' "$T/out")" = '10 12 14 15 16 18 21 22 23 30 '
assert 'range-reads-brief-as-nothing' test "$(viewed | grep -c '\b36\b' || true)" = 0
reset_issues
run --apply --backup-dir "$T/B9" --range "$C1..origin/main"
assert 'range-apply-never-edits-issue-36' test "$(edited)" = '10 14 16 18 30 '
assert 'range-apply-never-reads-issue-36' test "$(viewed | grep -c '\b36\b' || true)" = 0
run --check 11 12
assert 'explicit-numbers-selection' test "$(viewed)" = '11 12 '

# --- J: stale or moved source, and a range ending elsewhere ---------------------------------------
reset_issues
run --check --range "$C1..feature"
assert 'range-end-not-origin-main-refused' test "$rc" = 2
assert 'range-end-not-origin-main-no-gh-call' no_gh_call
run --check --range "$C1..$C2"
assert 'range-end-ancestor-refused' test "$rc" = 2
assert 'range-end-ancestor-no-gh-call' no_gh_call
git -C "$R" update-ref refs/remotes/origin/main "$C1"
run --check --all
assert 'locally-moved-origin-ref-refused' test "$rc" = 2
assert 'locally-moved-origin-ref-no-gh-call' no_gh_call
git -C "$R" fetch -q origin
git clone -q "$O" "$T/clone2"
printf '# Eleven\n\neleven moved on\n' >"$T/clone2/.github/ISSUE_SPECS/11-insync.md"
git -C "$T/clone2" add -A .github
git -C "$T/clone2" commit -q -m 'remote main moves'
git -C "$T/clone2" push -q origin main
run --apply --backup-dir "$T/B8" --all
assert 'stale-origin-ref-refused' test "$rc" = 2
assert 'stale-origin-ref-no-gh-call' no_gh_call
assert 'stale-origin-ref-no-backup-dir-written' test ! -e "$T/B8/10.json"

# --- K: reconcile evidence for an unmatched issue --------------------------------------------------
git -C "$R" fetch -q origin
reset_issues
RD=$T/RD
run --check --reconcile-dir "$RD" 15
assert 'reconcile-exit-3' test "$rc" = 3
assert 'reconcile-github-body-file' cmp -s "$RD/15.github.md" "$T/bodies/15"
git -C "$R" show origin/main:.github/ISSUE_SPECS/15-unmatched.md >"$T/src15"
assert 'reconcile-source-blob-file' cmp -s "$RD/15.source.md" "$T/src15"
git -C "$R" show "$C2:.github/ISSUE_SPECS/15-unmatched.md" >"$T/near15"
assert 'reconcile-nearest-blob-file' cmp -s "$RD/15.nearest.md" "$T/near15"
assert 'reconcile-header-names-nearest-commit' grep -q "commit $C2 path .github/ISSUE_SPECS/15-unmatched.md" "$RD/15.header.txt"
assert 'reconcile-dir-must-not-exist' bash -c '! (cd "$1" && PATH="$2:$PATH" GH_STUB_DIR="$3" bash "$4" --check --reconcile-dir "$5" 15) >/dev/null 2>&1' _ "$R" "$T/bin" "$G" "$SCRIPT" "$RD"

# --- L: every gh call is bound to origin's repository --------------------------------------------
# (the stub refuses any argv whose --repo is not test-owner/test-repo, so every run above proves the flag)
reset_issues
GH_REPO=evil/other run --check 11 12
assert 'ambient-gh-repo-cannot-redirect' test "$(sort -u "$G/ghrepo-env" | tr '\n' ' ')" = 'test-owner/test-repo '
assert 'ambient-gh-repo-run-completes' test "$rc" = 3 # 11 is fast-forward since section J moved remote main
for url in git@github.com:test-owner/test-repo.git ssh://git@github.com/test-owner/test-repo https://github.com/test-owner/test-repo/; do
    STUB_ORIGIN_URL=$url run --check 11
    assert "origin-url-form-accepted-$url" has_line '11 fast-forward'
done
for url in "$O" https://example.com/test-owner/test-repo https://github.com/test-owner; do
    STUB_ORIGIN_URL=$url run --check 11
    assert "underivable-origin-url-refused-exit-2-$url" test "$rc" = 2
    assert "underivable-origin-url-no-gh-call-$url" no_gh_call
done

# --- M: --reconcile-dir and --backup-dir resolve against the caller's directory --------------------
reset_issues
mkdir -p "$R/rdexist" "$R/.github/rdexist2"
RUNCWD=$R/.github run --check --reconcile-dir rdexist 15
assert 'relative-reconcile-dir-lands-in-the-callers-directory' test -f "$R/.github/rdexist/15.github.md"
assert 'relative-reconcile-dir-leaves-the-repo-root-copy-alone' test -z "$(ls -A "$R/rdexist")"
RUNCWD=$R/.github run --check --reconcile-dir rdexist2 15
assert 'relative-reconcile-dir-existing-in-callers-directory-refused' test "$rc" = 2
mkdir -p "$R/.github/relb2"
: >"$R/.github/relb2/10.json"
RUNCWD=$R/.github STUB_BACKUP_DIR=$R/.github/relb run --apply --backup-dir relb 10
assert 'relative-backup-dir-lands-in-the-callers-directory' test -f "$R/.github/relb/10.json"
assert 'relative-backup-dir-leaves-the-repo-root-alone' test ! -e "$R/relb"
RUNCWD=$R/.github run --apply --backup-dir relb2 10
assert 'relative-backup-dir-existing-file-refused' test "$rc" = 2
assert 'relative-backup-dir-existing-file-no-gh-call' no_gh_call
rm -rf "$R/rdexist" "$R/.github/rdexist" "$R/.github/rdexist2" "$R/.github/relb" "$R/.github/relb2"

# --- N: the write phase checks state, title and the backup file itself ---------------------------
reset_issues
: >"$G/close-race-10"
run --apply --backup-dir "$T/B10" 10
assert 'closed-before-the-write-refused-exit-1' test "$rc" = 1
assert 'closed-before-the-write-refused-line' has_line '10 refused'
assert 'closed-before-the-write-no-edit' test -z "$(edited)"
reset_issues
: >"$G/close-after-10"
run --apply --backup-dir "$T/B11" 10
assert 'closed-by-the-edit-fails-the-read-back' test "$rc" = 1
assert 'closed-by-the-edit-mismatch-line' has_line '10 MISMATCH'
reset_issues
: >"$G/corrupt-title-16"
run --apply --backup-dir "$T/B12" 16
assert 'readback-title-differs-fails' test "$rc" = 1
assert 'readback-title-differs-mismatch-line' has_line '16 MISMATCH'
assert 'readback-title-differs-says-title' grep -q 'read-back title differs' "$T/err"
reset_issues
: >"$G/corrupt-title-30"
run --apply --backup-dir "$T/B13" 30
assert 'title-changed-though-spec-has-none-fails' test "$rc" = 1
assert 'title-changed-though-spec-has-none-says-so' grep -q 'title changed although the spec has none' "$T/err"
reset_issues
: >"$G/plant-10"
STUB_BACKUP_DIR=$T/B14 run --apply --backup-dir "$T/B14" 10
assert 'backup-never-overwrites-a-file-that-appeared' test "$rc" = 1
assert 'backup-that-appeared-is-left-as-it-was' test "$(cat "$T/B14/10.json")" = 'planted by a concurrent writer'
assert 'backup-that-appeared-no-edit' test -z "$(edited)"
reset_issues
: >"$G/race-10"
run --apply --backup-dir "$T/B15" 10
assert 'refusal-before-the-backup-names-no-backup-file' bash -c '! grep -q "Rollback copy" "$1" && grep -q "no rollback copy exists" "$1"' _ "$T/err"
reset_issues
: >"$G/corrupt-10"
run --apply --backup-dir "$T/B16" 10
assert 'refusal-after-the-backup-names-the-backup-file' grep -q "Rollback copy: $T/B16/10.json" "$T/err"

# --- O: the backup-dir pre-check runs before ls-remote; ls-remote runs once otherwise -------------
reset_issues
run --apply --backup-dir "$T/B2" --all
assert 'existing-backup-file-refuses-before-ls-remote' test ! -e "$G/gitlog"
run --check 11
assert 'ls-remote-runs-once-in-a-normal-run' test "$(wc -l <"$G/gitlog")" = 1

# --- P: two top-level spec files for one issue number are refused --------------------------------
O2=$T/origin2.git
R2=$T/repo2
git init -q --bare -b main "$O2"
git init -q -b main "$R2"
git -C "$R2" remote add origin "$O2"
mkdir -p "$R2/.github/ISSUE_SPECS"
printf '# Thirty-one\n\na\n' >"$R2/.github/ISSUE_SPECS/31-a.md"
printf '# Thirty-one\n\nb\n' >"$R2/.github/ISSUE_SPECS/31-b.md"
git -C "$R2" add -A .github
git -C "$R2" commit -q -m 'two specs for 31'
git -C "$R2" push -q origin main
RUNCWD=$R2 run --check --all
assert 'duplicate-spec-for-one-issue-refused' test "$rc" = 2
assert 'duplicate-spec-for-one-issue-no-gh-call' no_gh_call

# --- Q: the history does not depend on the user's log.showRoot -------------------------------------
reset_issues
GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=log.showRoot GIT_CONFIG_VALUE_0=false run --check 10
assert 'root-commit-blobs-are-history-despite-showroot-false' has_line '10 fast-forward'

if ((failures > 0)); then
    printf '%s assertion(s) failed\n' "$failures" >&2
    exit 1
fi
printf 'all assertions passed\n'
