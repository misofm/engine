# Verdict for the browser arm of `console_mixing_automation` (#1003, #1011): exit 0 for the two
# measured rounds `run-web-mixing-automation-benchmark.sh run` writes, slurped (`jq -s -e`), and 1
# for anything else. The claims live in `web-mixing-automation-lib.jq`, beside the reasons the
# runner prints when this refuses.
include "web-mixing-automation-lib";
web_mixing_rounds_valid
