# LoRA beside LoRA-FA

Lesson 5. Freeze half the adapter and train the other half: what does
it cost? LoRA and LoRA-FA in two experiments kept apart, fitting a
known shift and learning from examples, with the first retention
measure: what training does to the layer's outputs for its old
examples, LoRA and LoRA-FA beside full fine-tuning of the layer
itself.

Live: [LoRA beside LoRA-FA](https://softwarewrighter.github.io/X_eTaL-fine-tuning/lora-vs-fa/)
(guess first, move the rank and the examples trained on). A state can
be linked to: `?mode=advanced&rank=2&n=3`.

## The question

An adapter has two small tables, A and B. LoRA-FA keeps A as seeded
and trains B alone: half the numbers to train, half the optimizer's
memory, a checkpoint without A. What does freezing A cost: is there
something B alone cannot reach? And the course's first question, not
measured until now: when the layer learns new examples, what happens
to what it did before?

## Background

The adapter of the lessons before: a frozen 8 x 8 W, the correction
s B (A x), s = alpha / r with alpha 4, B at 0 and A seeded, so both
methods start from the same adapter; LoRA trains A and B
(`lo:a_dam`), LoRA-FA trains B alone (`lo:a_damFA`). Experiment A,
approximating a known matrix: the adapter is to come to a known shift
T (rank 4) directly, the inputs the identity's columns, so the loss
is half the squared size of s B A - T; linear algebra, with a known
answer. The lowest loss any update of rank k can reach is set by T's
singular values (Eckart-Young), analytical. Every row of s B A is a
mix of A's rows, so the update lies in A's row space, a subspace; when
A is frozen that subspace is fixed, and the closest LoRA-FA can come
to T is T's projection onto it, LoRA-FA's floor, analytical.
Experiment B, learning from examples: the task of lesson 3 (12
examples from the hidden shift W + T, the first n trained on, the
last 4 held out), plus 4 old examples, inputs with the outputs W gave
them before any training; the loss on them after training is the
retention measure. Three methods on the same examples from the same W
for the same steps: LoRA, LoRA-FA and full fine-tuning (W itself
moved by Adam).

## The program

```
P := (o_\ A) '+ '* i_nner (mx:i_nverse A '+ '* i_nner o_\ A) '+ '* i_nner A
rem := '{ Mb -> M2 := d_isclose Mb
  s_qrt '+ r_/ r_avel (M2 - M2 '+ '* i_nner P) ^ 2 } e_ach (e_nclose D1) c_at (e_nclose D2) c_at e_nclose T
```

`lora-vs-fa.xtl` makes W, T and the adapter as lesson 3 does; works
out T's singular values by power iteration with deflation on T' T,
the lowest loss at each rank from them, and checks the same lines on
a matrix built from known singular values (two Householder
reflections about a diagonal); fits T by `lo:a_dam` and `lo:a_damFA`
with a record every 10 steps; projects each learned update and T
onto the frozen A's row space through `mx:i_nverse` and prints the
remainders' sizes, LoRA-FA's floor beside the lowest for the rank,
and the counts (`lo:p_arams`, the shape of B); then trains the three
methods on the learning task with a record of the loss per example on
the examples trained on, held out and old, and the size of each
method's change to the layer; and draws four charts with Plot's
`c_hart!`. The page runs the same program at the rank (1 to 8) and the
examples trained on (2 to 8) its sliders set. It defines no function
of its own: the spectrum (ask FL2), the projection (ask FL3) and full
fine-tuning (ask FM7) are written out where they are used.

## How it works

The page marks every figure A (analytical) or M (measured). Experiment
A's account is built from the run (`lesson::narrate::moved`): each
method's loss from start to end against the lowest possible at the
rank and against LoRA-FA's floor, whether each came to its line to 3
decimals, the three remainders, the counts. Experiment B's: each
method's three losses before and after, whether its held-out loss
ended above where it started, the size of its change, which method
moved the old outputs least and most, and the note that LoRA's
merged layer changes old outputs as a moved W does. Tests check the
projection's identity (LoRA-FA's remainder under 1e-9, LoRA's not),
that LoRA ends at the lowest possible and LoRA-FA at its floor, the
spectrum's check, and that no sentence uses a verdict word.

What the measurements show on this task: at rank 4 and up LoRA fits T
to rounding while LoRA-FA stops at its floor, which it reaches to 13
decimals; the floor is a property of the seeded A, not of the rank
alone. On the learning task every method moves the old outputs (at rank
4 on 8 examples, the retention loss ends at 0.93 for LoRA, 1.25 for
LoRA-FA and 1.14 for full fine-tuning): nothing in the three methods
keeps the change away from the old inputs. With fewer examples
trained on the picture changes at each setting; the page says what
the numbers are.

## Run it

```bash
just run lora-vs-fa       # both experiments at rank 4, 8 examples
just serve lora-vs-fa     # its page at http://127.0.0.1:8212/
just test-demo lora-vs-fa # its CLI and browser baselines and the page's tests
```

## Summary

Freezing A fixes the subspace the update can lie in, so LoRA-FA has
an analytical floor against a known target, which it reaches and does
not pass; LoRA, moving A, reaches the lowest loss possible for the
rank. On the learning task the cost of the frozen half is measured,
not derived. Retention, measured for the first time: on this task
every method moves the old outputs; here an adapter was a cheap way
to store and train a change, not a guard of old behavior. Not shown: any
real model, noise, other seeds for A, a task whose new examples leave
an old region alone (the shared-subspaces lessons).

## Workarounds

- FL2 (X_eTaL-libraries): the spectrum by power iteration with
  deflation, as X_eTaL-ML's lora-explorer writes it, checked in the
  program against known singular values.
- FL3 (X_eTaL-libraries): the projection onto A's row space through
  `mx:i_nverse`, written out.
- FM7 (X_eTaL-ML): the task, its split and its losses, and full
  fine-tuning of W by `op:a_dam`, written out.
