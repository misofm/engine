# Dispose limiter reciprocal substitution as a class-B owner decision

## Status

Ready for Astra LOW review. This is the documentation-only lane-A child for audit #559 finding FX3. It is based on delivered main `d98646db47bc603c32431d999cd08f43a0168043` after the eight-partial barrier and FX1/FX2 delivery. Companion coordination issue: #560.

## Problem and current evidence

The limiter still divides its running box sum by the prepared window in both current kernel shapes:

- `crates/true-peak-limiter/src/lib.rs:1413` in the per-lane/ragged step;
- `crates/true-peak-limiter/src/lib.rs:1656` in the uniform-cohort step.

The historical candidate proposes splatting `1 / hot.window` once per block and multiplying per sample. `docs/rulings/effect-floor-accounting.md:255-259` and `:804-818` already identify that substitution as class B because it changes the rounding of every rendered gain word. The exact-unity regression commentary near `crates/true-peak-limiter/src/lib.rs:4028` also makes the current division law observable for non-power-of-two windows such as 97.

## Smallest closable outcome

Record the existing owner decision as the disposition of FX3: preserve `hot.box_sum.div(hot.window)` and defer reciprocal multiplication unless a separately numbered class-B research issue first defines a derived numerical tolerance, objective audio fixtures, and listening qualification. Closing this issue closes the audit disposition only; it does not claim an optimization or a performance improvement.

## Exact ownership

This issue owns only:

- `.github/ISSUE_SPECS/632-dispose-limiter-reciprocal-class-b.md`;
- the FX3 status and terminal coordination record in the tracker specs for #559/#560.

It does not own product source, manifests, locks, generated resources, SDK/browser files, artifact pins, benchmark runners, or qualification artifacts. Lane B retains all artifact qualification and pin authority.

## Decision and prohibitions

Under the standing class-A optimization authority, retain division exactly. Do not prototype reciprocal multiplication, change the limiter algorithm, derive or weaken tolerances, add audio fixtures, run listening work, time the candidate, or quote projected savings. A future class-B issue would require an explicit owner decision before implementation and must qualify its changed arithmetic on scalar, AVX2, Wasm SIMD, and supported sample rates.

## Objective gates

1. Verify the two current division sites and exact-unity regression commentary at base `d98646db47bc603c32431d999cd08f43a0168043`.
2. Verify the standing floor ruling explicitly names reciprocal substitution as class B and requires owner ruling, tolerance, and listening qualification before benchmarking.
3. Prove the branch changes no product, test, dependency, generated, artifact, benchmark, or pin path.
4. Record an Astra LOW adversarial PASS on the exact pushed documentation head.
5. Synchronize the local issue body and GitHub issue; merge only after required PR qualification, verify post-main qualification, close the GitHub issue, update #559/#560, and remove the clean delivered worktree.

No build, test, benchmark, artifact qualification, browser run, or audio listening run is necessary for this documentation-only disposition. Source and ruling inspection plus diff hygiene are the discriminating gates.

## Attempt record

Attempt 1 records the standing owner disposition without changing its arithmetic.
The pre-issue Astra LOW scope review passed this bounded documentation-only shape
against main `d98646db47bc603c32431d999cd08f43a0168043` and found no lane-B
path overlap.

At that base, exact source inspection found the two division sites at lines 1413
and 1656. The exact-unity regression explains that `Wb = 97` makes
`97 * (1 / 97)` differ from exact `1.0` in `f32`, so the current division is an
observable bit contract. The adopted floor ruling says reciprocal substitution
moves rendered bits, is outside class-A authority, and requires derived tolerance
plus listening qualification before benchmarking. It expressly instructs the
optimization loop to flag the candidate and stop.

The owner disposition is therefore **defer and preserve division**. FX3 is fully
disposed for this class-A audit; no optimization or speedup is claimed. Any future
proposal must be a separately numbered class-B issue rather than reopening this
implementation round.

The documentation gate ran against this branch before the evidence checkpoint:

- both division sites and the exact-unity commentary were present;
- the class-B rule and its limiter reciprocal table row were present;
- `git diff --exit-code d98646db --` over limiter source and the floor ruling
  returned zero;
- `git diff --name-only d98646db` named only this issue spec;
- `git diff --check` returned zero;
- the unchanged limiter source SHA-256 was
  `32ab4abf975b32d47c85a748e617e74c9547b22e1b585f0d36713be439a62908`;
- the unchanged ruling SHA-256 was
  `36010df13d28a913847d44d12dc12b2d6090ac21cd8743a5a01577a455078c89`.

No build, test, benchmark, artifact, browser, listening or product command ran.
Astra LOW returned **PASS** for attempt 1 at exact clean pushed head
`3fa7f13bc4889552113b95f467dff8597f4f65ed`, base
`d98646db47bc603c32431d999cd08f43a0168043`. The independent review verified
both division sites, the window-97 exact-unity rationale, the explicit class-B
authority, both recorded hashes, one-file diff hygiene, exact GitHub body and
tracker synchronization, and no lane-B overlap. No correction is required.
Exact-head/current-main PR readiness remains required before PR creation.
