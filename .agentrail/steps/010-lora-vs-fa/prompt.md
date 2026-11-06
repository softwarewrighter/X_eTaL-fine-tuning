Step lora-vs-fa (docs/plan.md, Saga 1 row 6, principle 5;
docs/research.txt section 7, "training versus fitting a known
matrix"). Track "lora", lesson 5.

Experiment A, approximating a known matrix, named as that on the page,
then compared with Experiment B.

1. The spectrum (ask FL2's workaround; no function of its own,
   CLAUDE.md rule 3): the singular values of a matrix by power
   iteration with deflation on M' M and the Eckart-Young optimum at
   each rank, written out in the program as the LoRA explorer writes
   it (with a note of where it came from), and checked there against
   a matrix built from known singular values. If FL2 has landed at
   the pin, use it.
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

Added 2026-11-05 (docs/research4.txt, shared subspaces; docs/plan.md,
Saga 3): the row space of the frozen A is the first fixed subspace a
reader meets. Name it as that, in new-eyes words: the glossary gains
subspace, basis and projection, each a few plain sentences. Say only
what this lesson shows (LoRA-FA's update cannot leave that subspace,
LoRA's can); the general idea of sharing a subspace between tasks is
Saga 3's, pointed to under "where next". The projection onto A's row
space is written out with Matrix's inverse (ask FL3 in
docs/xetal-libraries-asks.md has the expression, tried at the pin).
