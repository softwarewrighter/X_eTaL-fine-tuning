Step lesson-shell (docs/plan.md, Saga 1 row 2; decisions F5, F6;
CLAUDE.md rule 8; docs/research.txt sections 2, 5, 6, 8).

Build shared/lesson, the teaching shell every page uses, on top of
X_eTaL-ML's microscope (read work/xetal-ml/shared/microscope/README.md
and its stages.rs, stageview.rs, source.rs, color.rs first; reuse, do
not copy).

Deliver, each a module with native tests:
1. The glossary: shared/lesson/glossary.tsv, a row per term (id, term,
   definition in one line, why it matters, a worked example, the
   lesson that teaches it). Start with the research's inventory
   (weight, parameter, matrix, tensor, projection, rank, frozen,
   adapter, gradient, backpropagation, learning rate, loss, optimizer,
   epoch, quantization, SVD, merge) and every symbol of the LoRA layer
   (x, W, A, B, s, alpha, r, h, u, y). Plain words, American spelling.
   A test: no empty field, ids unique.
2. `term`: any text or symbol wrapped as a glossary term, at four
   depths: hover or focus (definition), click or Enter (short
   explanation), expand (worked example), learn more (link to the
   lesson). Focusable, operable by keyboard and touch; never hover
   alone.
3. `mode`: the Beginner, Explorer and Advanced switch, a context the
   parts read to choose how much to show; switching keeps all state.
4. `heat`: a matrix as a heat map (microscope's diverging colors) with
   a legend, a note saying whether the scale is shared with the maps
   beside it, and a cell inspector (matrix name, row, column, value,
   meaning) on hover, focus and tap; cells highlightable as read or
   written. Adapted from the LoRA explorer's `heat` with a note.
5. `stepper`: play, pause, next, previous, reset over a recorded trace
   (a list of stages: name, value, shape, the source range, the text
   id of its explanation); stepping never runs X_eTaL again. The trace
   is built from flow:s_tages< output (F6).
6. `arithmetic`: one output cell's sum written out in symbols and then
   with the run's numbers (z0 = A00 x0 + ... ), from the arrays the
   page already holds; a test that the terms add up to the cell's
   value as X_eTaL printed it.
7. `narrate`: sentences about a change from measured before and after
   values (a count, a loss, a percentage), with rules for what may be
   said (no "better" without a measured figure; an increase in
   validation loss is "possibly overfitting", not a verdict).
8. `predict`: a prediction question with choices, answered before the
   experiment is shown, then feedback from the measured result.
9. The footer of this repository (ask FM2) and the library store (ask
   FM1), as step foundation found them.

No lesson page yet: a small gallery page (demos/shell-gallery, status
draft, not in the catalog) shows each part on fixed data for the
browser check and for review. Update docs/xetal-ML-asks.md (FM5: the
heat map offered upstream) and the plan.

Since this step was written (the steps hello-framing and
hello-new-eyes): shared/lesson already has `chrome` (the footer, the
six frames, asides, a panel), `eli5` (glossary.tsv with 32 ideas, a
word that opens its explanation, marked prose, the list of ideas),
`explain` (a library function from the pinned library's own source)
and `code` (X_eTaL drawn with every function linked to the
cross-reference). Build on them: the glossary and the term of items 1
and 2 exist (extend the glossary; add the depths still missing: a
worked example, the link to the lesson that teaches it). Everything
is presentation: no number is computed in Rust (CLAUDE.md rule 3a).
Every part is written and read against docs/new-eyes.md.

Since X_eTaL-ML's first delivery (pinned 2026-11-05): the heat map of
item 4 builds on `microscope::heat` (`Matrix`, `heat`, `largest`,
`column`, `fill`), adding the legend, the scale note, the cell
inspector and keyboard focus; offer those back as a patch or a further
ask (docs/xetal-ML-asks.md, FM5). shared/lesson no longer has `libs`
or `run`: libraries come through MICROSCOPE_LIBS, and a page runs
programs with `microscope::run`. The header is `lesson::chrome::header`.
