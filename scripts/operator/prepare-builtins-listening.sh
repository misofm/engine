#!/usr/bin/env bash
# Render one Issue-007 builtin listening packet.
#
# #104 phase A. This replaces `prepare-builtins-listening-033.sh` and
# `prepare-builtins-listening-111.sh`, which were retired with the rest of the sealed 033/111
# families (#83 wave-4 decision W4-D2): they pinned the sha256 of `Cargo.lock`, of the renderer
# source, and of seven `target/issue110/` build artifacts, refused to run off the branch
# `codex/listening-111`, and demanded a pre-existing `target/issue111/{preflight,preparation}.seal.json`
# pair that no longer exists anywhere. None of that is refreshable and none of it was the
# capability; the capability is "render a packet and prove it is well-formed and answer-free".
#
# usage: prepare-builtins-listening.sh INBOX OUT_DIR
#   INBOX   a directory holding `source.mepcm`, `provenance.json` and `seed.txt` supplied by the
#           facilitator. It is never read into the repository and never played back.
#   OUT_DIR must not exist; the assembled public packet is written there.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

fail() { printf 'listening preparation failure: %s\n' "$1" >&2; exit 1; }

[[ $# -eq 2 ]] || { printf 'usage: %s INBOX OUT_DIR\n' "$0" >&2; exit 2; }
inbox="$1"
out="$2"

for tool in cargo git python3 sha256sum; do
    command -v "$tool" >/dev/null 2>&1 || fail "missing tool: $tool"
done
[[ -d "$inbox" && ! -L "$inbox" ]] || fail 'inbox must be a non-symlink directory'
for member in source.mepcm provenance.json seed.txt; do
    [[ -f "$inbox/$member" && ! -L "$inbox/$member" ]] || fail "missing inbox member: $member"
done
[[ ! -e "$out" && ! -L "$out" ]] || fail 'output directory already exists'

# The renderer, the packet validators and the public schemas must be self-consistent before a
# private source is opened at all.
bash scripts/check-builtins-listening.sh >/dev/null || fail 'listening policy'

umask 077
python3 -I -B scripts/check-builtins-listening-033.py --source \
    "$inbox/source.mepcm" "$inbox/provenance.json" || fail 'source or provenance'

cargo build --locked --release --bin audit \
    --manifest-path tools/audit/Cargo.toml >&2
binary=target/release/audit
[[ -x "$binary" ]] || fail 'renderer binary'

commit="$(git rev-parse --verify HEAD)"
tree="$(git rev-parse 'HEAD^{tree}')"
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'dirty candidate tree'

partial="$out.partial"
[[ ! -e "$partial" && ! -L "$partial" ]] || fail 'partial packet directory already exists'
trap 'rm -rf -- "$partial"' EXIT

"$binary" fixture-builtins-listening --render "$inbox/source.mepcm" \
    fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm \
    "$inbox/provenance.json" "$inbox/seed.txt" "$partial" || fail 'render'

python3 -I -B scripts/check-builtins-listening-033.py --renderer-output "$partial" ||
    fail 'renderer output'

for public in dsp-research/listening/issue033/FACILITATOR.md \
    dsp-research/listening/issue007-filter-abx-preregistration.md \
    dsp-research/listening/issue007-matrix-ramp-preregistration.md \
    dsp-research/listening/issue033/preparation.schema.json \
    dsp-research/listening/issue033/response.schema.json \
    dsp-research/listening/issue033/reveal.schema.json \
    dsp-research/listening/issue033/qualification.schema.json \
    dsp-research/listening/issue033/response-form.jsonl; do
    cp "$public" "$partial/public/"
done
mv "$partial/public/issue007-filter-abx-preregistration.md" "$partial/public/filter-preregistration.md"
mv "$partial/public/issue007-matrix-ramp-preregistration.md" "$partial/public/matrix-preregistration.md"
chmod 0444 "$partial/public/"*

python3 -I -B scripts/check-builtins-listening-033.py --assemble \
    "$partial/public/render-manifest.json" "$commit" "$tree" "$partial/public/preparation.json" ||
    fail 'packet assembly'
chmod 0444 "$partial/public/preparation.json"
python3 -I -B scripts/check-builtins-listening-111.py --packet "$partial" || fail 'packet validation'

mv -T -n "$partial" "$out"
trap - EXIT
[[ -d "$out" && ! -L "$out" ]] || fail 'packet publication'
printf '%s\n' "$out/public/preparation.json"
