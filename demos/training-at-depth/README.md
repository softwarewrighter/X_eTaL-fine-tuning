# Training at depth

Experiment D4, the fourth page of the depth track (a side track on
DeepSeek's mHC, `docs/mhc-plan.md`). Does the bound on the gain matter
once a model is trained? The plain skip, HC and mHC trained on one
tiny task at 12, 24 and 48 layers, from one start; the long runs
trained offline by the same program and replayed, a short run
computed on the page.

Live: [Training at depth](https://softwarewrighter.github.io/X_eTaL-fine-tuning/training-at-depth/)
(pick a kind of connection for the short run, a depth for the
recorded ones). A state can be linked to:
`?mode=explorer&scheme=hc&depth=24`.

## The question

Lesson D3 showed that HC's mixing table, repeated over 48 layers,
multiplies the carried state's gain to about 10^28 while mHC's keeps
it at 1. Does that matter once a model is trained? The paper (Xie
and others, arXiv:2512.24880) reports HC's training loss and gradient
jumping in its large models; the Python re-implementation
([mHC-poc](https://github.com/softwarewrighter/mHC-poc)) reports that
at 48 layers its HC model did not learn and its mHC model did.

## Background

The task is the Python's, made tiny: each of 16 tokens, its successor
the target (16 wraps to 1); an embedding (16 x 8), L layers of lesson
D1 (a block 8 -> 16 -> 8 with GELU and the three mixing tables), the
streams averaged and scaled to size 1, an unembedding (8 x 16),
softmax and cross-entropy; the batch is the whole vocabulary. Every
weight is trained with Adam (Optim's), the mixing tables' logits too.
A scheme is two settings: one stream with the projection on is the
plain skip (every table is the number 1); four streams with H_res the
exponential of its logits is HC; with its Sinkhorn projection, mHC.

## The program

`training-at-depth.xtl` writes the training step out: the forward pass
layer by layer, keeping the state each layer was given and the
product of the H_res tables; the loss; the backward pass from the top
down, each layer recomputed from its state, the twenty Sinkhorn rounds
undone in turn; Adam's step; a record per step (the loss, the
gradient size, the composite's forward and backward gains and its
proxy, a NaN or infinite flag). A lambda can be applied only where
it is written (a program defines no function of its own), so the
training run and the gradient check are one lambda mapped over a list
of jobs: the run, and nine weights moved by a millionth up and down
at a point near the start, for central differences, checked by the
Check library. `replay.xtl` reads the nine records for one depth
(`[]N_GET`, `n_umbers`) and draws four charts. `train/train.sh` (`just
train training-at-depth`) runs the training program nine times, 800
steps, the depth, the steps, the scheme and no gradient check changed,
and writes `data/SCHEME-L.txt` and `data/runs.txt` (the steps, both
pins and the program's hash); about four minutes a run at 48 layers.

A preset is a file of settings, `presets/*.toml`, not code:

| Preset | streams | H_res |
| ------ | ------- | ----- |
| The plain skip | 1 | the number 1 |
| HC | 4 | the exponential of its logits |
| mHC | 4 | its Sinkhorn projection |

## How it works

Measured, after 800 steps (guessing evenly gives 2.77):

| Depth | plain skip | HC | mHC |
| ----- | ---------- | -- | --- |
| 12 | 0.0028 | 0.0029 | 0.0029 |
| 24 | 0.0030 | 0.0029 | 0.0027 |
| 48 | 2.7726 | 0.0027 | 0.0027 |

At 48 layers the plain skip never learns (its gradient size falls to
3e-7); HC and mHC learn, HC with its composite gain near 10^27.5
through all 800 steps, mHC's at 1. No step of any run had a NaN
event. The Python's final losses at 48 layers (its task: 256 tokens,
sequences, attention, a model 32 times as wide) were 3.79, 5.54 and
0.0002: the toy reproduces the plain skip failing and mHC learning,
not HC failing. The page names differences that could account for it
(the Python's 32-bit numbers against 64-bit here, its blocks' own
skips and scaling, its task) and does not claim which. The short run
on the page, 50 steps at 12 layers, matches the first 50 steps of its
recorded run to the last digit (tested).

What the page does not show: attention, sequences or real text; any
model of size; more than one start and one task; the paper's dynamic
mixing (lesson D5).

## Run it

```bash
just run training-at-depth              # the short run, with its gradient check
just run training-at-depth replay.xtl   # the recorded runs at 48 layers, replayed
just train training-at-depth            # the nine offline runs again (about 9 minutes)
just test-demo training-at-depth        # its CLI and browser baselines (reg-rs)
just serve training-at-depth            # the page at http://127.0.0.1:8212/
```

## Summary

On this task the three kinds of connection learn alike at 12 and 24
layers; at 48 the plain skip does not learn, HC and mHC do. mHC's
gains stay at 1 through training; HC's stay near 10^27.5 at 48 layers
without stopping it learning here, unlike the Python's run. Next: the
paper's own mixing, which depends on the input (lesson D5).

## Workarounds

The forward and backward pass of a Hyper-Connections layer, the
gradient of the Sinkhorn rounds and the gains are written out in the
program: ask FM13 of X_eTaL-ML (`hc:l_ayer`, `hc:b_ackTop`,
`hc:b_ackBottom`, `hc:s_inkhornBack`, `hc:a_maxGain`,
`hc:g_ainProxy`), [`docs/xetal-ML-asks.md`](../../docs/xetal-ML-asks.md).
