#!/usr/bin/env bash
set -euo pipefail

cd "${1:-.}"
records=(
  dsp-research/listening/issue007-filter-abx-preregistration.md
  dsp-research/listening/issue007-matrix-ramp-preregistration.md
)
for record in "${records[@]}"; do
  [[ -f "$record" ]] || { printf 'missing issue-007 listening preregistration: %s\n' "$record" >&2; exit 1; }
  rg -Fqx -- '- Evidence kind: real listening' "$record"
  rg -Fqx -- '- Status: preregistered' "$record"
  rg -Fqx -- '- Sound-quality claim: none' "$record"
  rg -Fq 'No human trial has been run.' "$record"
  ! rg -Fq '| 1 |' "$record" || { printf 'preregistration contains fabricated trial row: %s\n' "$record" >&2; exit 1; }
done

# #104 phase A. `check-builtins-listening-033.sh` and `check-builtins-listening-111.sh` were
# retired: both opened with the sha256 of `Cargo.lock`, of production sources the lane waves
# rewrote, and of seven `target/issue110/` build artifacts that only ever existed on the branch
# that produced them (#83 wave-4 decision W4-D2). Their live half is here, so retiring them lost no
# coverage: the two packet validators still self-test, and the facilitator packet's schemas are
# still required to be canonical and answer-free.
for validator in scripts/check-builtins-listening-033.py scripts/check-builtins-listening-111.py; do
  [[ -f "$validator" ]] || { printf 'missing listening validator: %s\n' "$validator" >&2; exit 1; }
  python3 -I -B "$validator" --self-test >/dev/null
done

python3 -I -B - <<'PYEOF'
import json
from pathlib import Path

root = Path("dsp-research/listening/issue033")
for path in sorted(root.glob("*.schema.json")):
    raw = path.read_bytes()
    value = json.loads(raw.decode("utf-8", "strict"))
    expected = (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()
    assert raw == expected, f"non-canonical schema: {path}"
assert (root / "response-form.jsonl").read_bytes() == b"", "response form is not empty"
template = json.loads((root / "provenance.template.json").read_text(encoding="utf-8"))
assert template["permission_confirmed"] is False, "provenance template pre-confirms permission"
assert all("answer" not in path.name for path in root.iterdir()), "answer key in the public packet"
PYEOF

printf 'issue-007 listening preregistrations: ok (human evidence pending)\n'
