Step landscape (docs/plan.md, Saga 1 row 7, "The road", principle 7;
added 2026-11-05 after docs/research2.txt and the user's order: LoRA
and its variations, then alternatives, then bleeding-edge
alternatives). Track "lora", lesson 6.

Build demos/landscape, live: the map. A newcomer who has finished the
LoRA lessons should leave knowing where LoRA sits among the ways to
change a model and what the next lessons will compare. Framed as
every page is (CLAUDE.md rule 8): the question is the newcomer's own,
"LoRA is one way; what are the others, and how would I choose?".

1. The families, each in two or three plain sentences and one line of
   its arithmetic on a layer W:
   - change no weight (examples in the context, a lookup);
   - change every weight, or chosen ones (full and selective
     fine-tuning, by back-propagation);
   - train a small part beside frozen weights (LoRA, and its
     variations: QLoRA, DoRA, VeRA, named with what each changes);
   - learn without back-propagation (weight-space methods: forward
     passes only);
   - edit in closed form (a rank-one edit: no training at all);
   - let an AI do the editing.
   Say which are lessons here already, which come next (Saga 2), and
   which are planned to be planned (Sagas 3, 4 and 5): no claim about a
   method this repository has not yet measured beyond what its
   definition gives.
2. The accounting, computed by X_eTaL for this saga's small task and
   for one real layer (4096 x 4096), each figure marked analytical:
   for each family whose count follows from its definition, the
   numbers trained, the numbers held while training (the gradient and
   the optimizer's averages where there are any), and what is stored
   afterward. A program, demos/landscape/landscape.xtl, with baselines.
   Families whose cost is not a count (changing no weight; an AI
   editing) say so instead of showing a number.
3. The three worries (forgetting, time, cost) as the columns of a
   table: for each family, what its definition guarantees, what has
   been measured in this repository so far (with a link to the
   lesson), and what is still to be measured.
4. Where each later lesson sits on the map (the curriculum), as links
   that are live or marked "to come".

Tests: the model natively (the counts against the formulas), the
frames, the browser check. README (Demos), the demo's README,
CHANGES.md, the plan; publish and check-live.

Added 2026-11-05 (docs/research4.txt; docs/plan.md, Saga 3): the map
is sorted by two questions, kept apart, since any answer to the first
can be paired with any answer to the second:
- where may the change live (the representation): every weight; two
  thin tables beside frozen weights (LoRA); small coefficients in a
  fixed basis that tasks share (shared subspaces: VeRA is the first
  the reader has met); one closed-form edit;
- how is the change found (the optimizer): back-propagation; forward
  passes only; a formula; an AI proposing and testing.
Draw the map as that grid, with each lesson, live or to come, in its
cell. Shared subspaces are a column of it, not one more variation,
and are not presented as better than separate adapters.
