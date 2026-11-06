Step rank-and-scaling (docs/plan.md, Saga 1 row 5, principle 4,
decision F8; docs/research.txt section 6). Track "lora", lesson 4.

1. No library and no function of its own (CLAUDE.md rule 3): rsLoRA's
   scale (Kalajdzievski, 2023) is X_eTaL-ML's `alpha lo:s_calingRs r`
   (ask FM3, landed and pinned), beside `lo:s_caling`'s alpha / r.
2. demos/rank-and-scaling: on the learning task of step lora-training
   (written out as in step lora-training), rank 1 .. 8 and a few alphas swept: for each, the
   trainable numbers (analytical), the train and validation loss after
   a fixed number of steps (measured), the size of the update s B A
   (its Frobenius norm, measured) under each scale rule.
3. The page: the rank slider and the alpha slider with a sentence
   under each; a table and curves with every figure marked measured or
   analytical and a note of what a parameter count does not predict;
   the narrator's account of each move from the measurements (more
   rank, more reach and more cost; where validation stops improving;
   how alpha / r shrinks the update as r grows and alpha / sqrt r does
   not), never "higher rank is better". A prediction question first.
4. Tests: the sweep's baselines, the scales' values, the narrator's
   sentences against the measured table, the browser check.

Docs, CHANGES.md, the plan; publish and check-live.
