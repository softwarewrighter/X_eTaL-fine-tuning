Step lora-vs-fa (docs/plan.md, Saga 1 row 6, principle 5;
docs/research.txt section 7, "training versus fitting a known
matrix"). Track "lora", lesson 5.

Experiment A, approximating a known matrix, named as that on the page,
then compared with Experiment B.

1. The spectrum (ask FL2's workaround): in FineTune, the singular
   values of a matrix by power iteration with deflation on M' M and
   the Eckart-Young optimum at each rank, checked against a matrix
   built from known singular values; adapted from the LoRA explorer's
   inline code with a note.
2. demos/lora-vs-fa: LoRA and LoRA-FA (lo:a_dam, lo:a_damFA, the same
   seeded A, B at 0) fitting a known rank-4 update, loss curves
   against the optimum at the rank chosen; each learned update
   projected onto the frozen A's row space, with the remainder: LoRA's
   leaves it, LoRA-FA's cannot, which is why it stops short. The
   trainable numbers and Adam's state for each (analytical).
3. Then the same two methods on the learning task of step
   lora-training, with validation loss: what the frozen A costs when
   learning from examples, and a short text on what each experiment
   does and does not show.
4. The page: both experiments, clearly titled and separated, a
   prediction question before each, the narrator from the measured
   figures.
5. Tests: baselines, the projection's identity (LoRA-FA's remainder is
   0 within tolerance, LoRA's is not), LoRA at the optimum, the
   browser check.

Docs, CHANGES.md, the plan, ask FL2; publish and check-live.
