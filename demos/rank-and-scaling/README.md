# Rank and scaling

Lesson 4. What does the rank buy, and what does it cost? The adapter's
rank swept from 1 to 8 on the learning task of lesson 3, under LoRA's
strength s = alpha / r and rsLoRA's s = alpha / sqrt r, every figure
marked measured or analytical.

Live: [Rank and scaling](https://softwarewrighter.github.io/X_eTaL-fine-tuning/rank-and-scaling/)
(guess first, move alpha and the steps, pick a rank to read about). A
state can be linked to: `?mode=advanced&alpha=4&steps=0&rank=8`
(alpha and steps as positions in the sliders' lists).

## The question

Lesson 2 showed what the rank is and what it costs; lesson 3 trained
one adapter at one rank. What does a larger rank buy when the adapter
is trained, and does the rule that sets the strength from the rank
matter? Behind both: what rank would a newcomer pick, and how would
they know?

## Background

The task of lesson 3 (a frozen 8 x 8 W, a hidden shift T of rank 4,
8 examples trained on and 4 held out) is trained 8 times, at rank 1
to 8, each for the same number of Adam steps from the same kind of
start, once under each rule for s: `alpha lo:s_caling r` (alpha / r)
and `alpha lo:s_calingRs r` (alpha / sqrt r; ask FM3, landed). A
figure that follows from the setup alone (the numbers trained, s) is
analytical; one that comes out of training (a loss, the size of the
adapter's table s B A) is measured.

## The program

```
(_, table) := 8 '{ (k, rows) ->
  a1 := (W, 1 lo:s_eeded k c_at 8, (8 c_at k) r_eshape 0.0, alpha lo:s_caling k)
  a2 := (W, 1 lo:s_eeded k c_at 8, (8 c_at k) r_eshape 0.0, alpha lo:s_calingRs k)
  (t1, _, _) := steps '{ e -> lr lo:a_dam (e, (Xt, Yt)) } p_ower lo:a_damStart a1
  (t2, _, _) := steps '{ e -> lr lo:a_dam (e, (Xt, Yt)) } p_ower lo:a_damStart a2
  ...
  (k + 1, rows c_at row)
} p_ower (1, 0 r_eshape 0.0)
```

`rank-and-scaling.xtl` makes the task as lesson 3 does, sweeps the
rank in a `p_ower` loop that adds a row per rank (the numbers
trained; then under each rule: s, the loss per example on the
examples trained on, on the held-out ones, and the size of s B A, the
square root of the sum of its squares), prints the table, the size of
T and the losses before any training, and draws three charts with
Plot's `c_hart!`: the two losses by rank under each rule, and the
sizes beside T's. The page runs the same program at the alpha and
steps its sliders set (1, 2, 4, 8, 16; 10, 30, 100, 300). It defines
no function of its own; the task is written out (ask FM7).

## How it works

The table marks every figure A or M. The narrator's sentences are
built from the rows' numbers (`lesson::narrate::moved`): what changed
from the rank before the one picked, and the account of the whole
table: reach and cost from rank 1 to 8; from which rank the examples
trained on are fit to under 0.001 per example; where the held-out
loss is lowest and whether it fell at every step up in rank (so
whether a count predicted it here); the two rules' strengths at rank
1 and 8 and the losses at rank 8 under each; and the sizes of s B A
against T's, with the reading that at rank 8 both rules ended within
a tenth of T's size when they did. A test checks the sentences
against the table and that none uses a verdict word.

What the measurements show on this task: the strength shrinks with
rank analytically (1 / r against 1 / sqrt r); the trained adapter's
table comes to about T's size under either rule once the rank can
fit; the rule's effect is in the early steps (at 10 steps and rank 8,
the trained-on loss differs between the rules), not after 300.

## Run it

```bash
just run rank-and-scaling       # the sweep at alpha 4, 100 steps
just serve rank-and-scaling     # its page at http://127.0.0.1:8212/
just test-demo rank-and-scaling # its CLI and browser baselines and the page's tests
```

## Summary

What the rank buys: numbers trained, 16 per 1 of rank (analytical),
and reach: the examples trained on are fit from a rank near T's rank,
4. What it buys on examples held out is measured, and need not fall
with every rank. What the rule for s changes: the strength
analytically, and how fast the adapter gets where it is going; where
it ends, on this task after enough steps, hardly. Not shown: any real
model, other alphas and steps than the sliders offer, and which rank
or rule is right for anything.

## Workarounds

- FM7 (X_eTaL-ML): the task, its split and its losses written out, as
  in lesson 3.
