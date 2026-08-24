# Honest-null optimization rulings

This directory preserves optimization hypotheses that predicted a win but measured no useful
improvement, or measured a regression. Keeping the null result prevents the same candidate from
being repeatedly proposed without new evidence.

Use one Markdown file per ruling. Each ruling must state:

- the exact candidate and the optimization claim that motivated it;
- the frozen workload, baseline, candidate, units, and measured result;
- the source issue/spec/benchmark record and its date or commit identity;
- the decision and the boundary of that decision (what was rejected, not adjacent designs);
- what materially new evidence or changed precondition would justify reopening it; and
- links back to the optimization issue or document that cited the candidate.

A null or negative measurement is a result, not a failed record. Do not replace it with a vague
“did not work,” omit an unfavorable row, or generalize beyond the measured candidate. If a later
implementation changes the relevant shape and wins, append a supersession note while retaining
the original ruling.
