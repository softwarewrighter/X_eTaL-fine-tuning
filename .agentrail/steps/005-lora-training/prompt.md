Step lora-training (docs/plan.md, Saga 1 row 4, principle 5;
docs/research.txt section 7; CLAUDE.md rule 8).

Experiment B, learning from examples, kept apart from approximating a
matrix. Track "lora", lesson 3.

1. libs/FineTune (`ft:`), its first functions, with tests, types.rgt
   and a reference page: a learning task from a seed (N inputs of 8
   numbers, targets from a hidden shifted layer W + T, optional
   noise), a deterministic train and held-out split, the train and
   validation losses of an adapter (through lo:l_oss), and a run's
   record (the losses every k steps, the states kept for stepping
   back). Each name bound once; doc comments with `## >>` examples.
2. demos/lora-training/lora-training.xtl: the task, B at 0 and a
   seeded A (as LoRA starts), one training step as a Flow pipeline
   (forward, loss, g, lo:g_rads' dA and dB, the SGD update), checked
   against lo:s_gd; then training by Adam (lo:a_dam) with the record.
3. The page: one step narrated in six stages (make a prediction,
   measure the error, the gradient at the output, the gradients of A
   and B, update A and B, W unchanged: shown by the difference of W
   before and after being exactly 0), each sentence quoting the run's
   numbers ("the loss went from 0.184 to 0.171": from the run, never
   typed). Then training: train and validation loss curves (Plot), the
   matrices at any recorded step, stepping back without rerunning.
4. Presets as data (demos/lora-training/presets/*.toml): first LoRA
   experiment, too large a learning rate (oscillation), overfitting
   (few examples, high rank: validation rises). The narrator explains
   each outcome from its measured curve, hedged as rule 8 says.
5. Tests: the gradients against central differences (FineTune and the
   pipeline), a replay test per preset (the same preset, the same
   trace), W unchanged, the page's model natively, the browser check.

Docs, CHANGES.md, the plan, the asks; publish and check-live.
