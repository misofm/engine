# Console floor parity evidence

Source checkpoint: d9a3413911e6f95f98c2134a5fa17d85c4b337f2. Only floor.rs cfg(test) changes the executable test population; production floor arithmetic and jq inventory remain unchanged.

Root records contain exact commands, raw output and captured numeric exit codes. Debug/release execute the named parity case, the floor suite executes nine tests, affected bench Clippy uses --tests --no-deps, formatting passes, and the existing validator suite records zero real workload/timing invocations. All six root statuses are zero.

Luna records are original raw logs without separate captured numeric statuses. The first cargo invocation failed because PATH lacked cargo; full dependency Clippy failed at three inherited normal-feature builtins-compiler question_mark sites. Those failures are retained and are not claimed green. The empty fmt log is authentic successful quiet-tool output; the root status supplies separately captured verification. No benchmark capture or preparation was performed.
