# Issue-880 MB-2 benchmark runner selection

The MQ-1 runner keeps its original E1 default and offline recovery invocation.
For an approved MB-2 comparison, select both the source commit and its known
revision explicitly:

```sh
bash scripts/run-issue880-mq1-benchmark.sh \
  --preflight \
  --effect-source-commit <MB-2-source-commit> \
  --effect-revision mb2-fast-db \
  --output <fresh-record-path>
```

After the source commit and frozen output path are reviewed, `--run` uses the
same pair and the unchanged MQ-1 workload and protocol. The record keeps the
selected effect source commit and writes `MB-2 (R2 fast dB tier)` as its
revision. The only accepted selections are the frozen E1 commit/revision pair
or a distinct MB-2 commit paired with `mb2-fast-db`.

Preflight requires the selected source commit to be an ancestor of `HEAD` and
requires the current tracked render sources to match that commit. MB-2 source
also has to include both approved X7/X8 crossing markers and the fast level and
gain calls. The record validator accepts only the matching E1 or MB-2 revision
and commit shape. No benchmark timing was run while adding this selection path.
