**Status: post-launch.** Recorded now so the reasoning survives; not scheduled, not blocking anything.

## The idea

A named-token layer over the parameter lattice, in the spirit of Tailwind's design tokens. Tailwind gives CSS properties a constrained, named scale — `p-4`, `text-lg` — instead of arbitrary values, and the scale lives in a config the rendering engine knows nothing about. The same shape fits miso's effects library: a vocabulary of named, musically-sensible values per parameter, resolving client-side to exact points on the lattice.

The mapping is direct:

> **Perceptual tokens are to a base-unit document what Tailwind classes are to compiled CSS.**

Tokens resolve in the client, the document stores objective integers, and the "compile" step is the snap to grid that #291 §6 already specifies.

## Why this is possible now, and would not have been

#291 moved perceptual stepping out of the descriptor and into the SDK, on the ruling quoted in its §0:

> "Maybe the step shouldn't be perceptual and should be unit-specific in an objective way instead. The perceptual layer could be built on the client side instead as an abstraction over the base step unit."

That placement is the enabling condition. #141 (closed) took the other path — ladders declared in effect-descriptor wire V2, participating in descriptor identity and CID. Under that design a token vocabulary is part of the document format: retuning the scale changes descriptor identity and mints new content addresses. Every theme edit becomes a wire change.

With the ladder client-side, a theme is versioned independently of documents. #291 §7's stated goal for the A7 re-basing — "UX retuning never touches the schema" — extends to the wire.

## What it would be stronger at than Tailwind

Tailwind emits values that can land subpixel and round unpredictably at paint. Because #291 persists an integer count of a base unit, a token always resolves to an exact grid point. No drift, by construction rather than by containment.

The per-property scales are also already in embryo. §6's `Gesture` is typed per row (`units` / `cents` / `ratio`), which is the analogue of Tailwind keeping separate scales for spacing, color, and shadow rather than one universal ramp.

## Three things to settle before building

**1. Absolute vs. relative — these are different features.** Tailwind is absolute: `p-4` is a specific padding, not "nudge padding up a bit." #127's xs–xl ladder is a *delta*. The Tailwind-shaped version is named points on a scale (`reverb-lg` meaning a value), which is probably the more valuable half and is not what #127 specified. Decide which is wanted; possibly both, but they are separate vocabularies.

**2. Tokens buy consistency, not perceptual equivalence.** `p-4` is always 16px. An `eq-lg` boost is not always audible — audio parameters are perceptually nonlinear and context-dependent, and a boost that reads as large in isolation can vanish in a dense mix. The real value is learnability and consistency across a UI. Naming should not imply a guarantee the physics will not honor.

**3. Many audio parameters are bipolar.** Boost and cut both need expression, so the vocabulary needs a sign. T-shirt sizes are one-dimensional and mostly dodge this.

## The concrete payoff

Themes version independently of documents, so different clients can ship different scales over identical sessions — a beginner UI with coarse rungs, a pro UI with fine ones, no divergence in what gets stored.

## Scope

A layer *above* `sdk/src/core/perceptual.ts`. Touches nothing the lattice work settles: no descriptor change, no wire change, no CID impact. Depends on #291 landing.

Related: #291 (parameter lattice v2), #127 (named nudge sizes — still open; its JND research is re-homed by #291 §7, not overruled), #141 (closed as superseded).
