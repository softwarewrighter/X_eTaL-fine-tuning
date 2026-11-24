# Dynamic mixing

Lesson D5, the fifth page of the depth track (a side track on
DeepSeek's mHC, `docs/mhc-plan.md`). In the paper the tables that mix
the streams are not fixed: each layer makes them afresh from the
stream state it is given. Two inputs sent through one layer, their
tables side by side, a slider on the gating factor, every stage of the
layer, and the gradient worked out by hand and checked.

Live: [Dynamic mixing](https://softwarewrighter.github.io/X_eTaL-fine-tuning/dynamic-mixing/)
(move the gating factor, step through the layer). A state can be
linked to: `?mode=explorer&alpha=1&stage=4`.

## The question

The earlier pages used mixing tables that are the same for every
input, as the Python re-implementation
([mHC-poc](https://github.com/softwarewrighter/mHC-poc)) has them.
The paper (Xie and others, arXiv:2512.24880, eq. 7 to 9, read at
source) makes them from the input. How, and does the rule that keeps
H_res doubly stochastic survive it?

## Background

The paper's recipe, from a stream state X of 4 streams of 8: flatten
to 32 numbers, divide by their root mean square, multiply by one
projection (32 x 24: 4 numbers for H_pre, 4 for H_post, 16 for H_res;
the paper's fused form, its eq. 14), multiply each group by its gating
factor and add a bias; then H_pre = sigmoid, H_post = 2 sigmoid,
H_res = Sinkhorn-Knopp of the exponential (20 rounds). The layer is
the paper's eq. 3: the block's input is H_pre X (a row of four
weights), the new state H_res X + H_post' f. The paper starts the
gating factors at 0.01 and states no starting biases; this page uses
the Python's static start for them (0, 0 and 2I - 1). The original
Hyper-Connections (the paper's eq. 5) used a tanh; the Python dropped
the dynamic part altogether.

## The program

`dynamic-mixing.xtl` writes the layer and its way back as one lambda
mapped over a list of jobs (a lambda can be applied only where it is
written): the two inputs, then the first input's numbers moved by a
millionth up and down at ten places, for central differences. The way
back goes from a fixed weighted sum of the new state (a stand-in for a
loss) through the block, H_pre's weighted sum, the sigmoids, the
twenty Sinkhorn rounds and the exponential, the gating factors and
biases, the projection and the normalization to the input. It prints
both inputs' stages, how far their tables are apart, H_res's sums,
the ten gradient numbers both ways, and two checks (Check). The
program defines no function of its own.

## How it works

At a gating factor of 0 the two inputs make the same tables, the
earlier pages' static ones. At the paper's 0.01 their H_pre weights
differ by at most 0.010, their H_post weights by 0.020 and their H_res
numbers by 0.0055; at 1 by 0.60, 1.40 and 0.73. H_res stays within a
millionth of doubly stochastic for both inputs up to 0.3; at 1 and 3
twenty Sinkhorn rounds do not reach it for one of them, and the check
says FAIL. The hand-worked gradient agrees with central differences on
all ten numbers at every gating factor, to about 1e-9.

What the page does not show: training with dynamic tables. Lesson D4's
training, repeated with them, needs a table per token per layer, so
its hand-written backward pass (about 40 lines) grows by the
normalization, the projection and a batch of Sinkhorn rounds, and each
run takes several times as long; that waits on ask FM13 (a
Hyper-Connections layer, static or dynamic, with its gradient) and
becomes a page of its own when it lands.

## Run it

```bash
just run dynamic-mixing          # run the program
just test-demo dynamic-mixing    # its CLI and browser baselines (reg-rs)
just serve dynamic-mixing        # the page at http://127.0.0.1:8212/
```

## Summary

A dynamic table is made from the input by a normalization, one
projection, a gating factor and a bias, and a squash per table. At
the paper's starting gating factor the input moves the tables a
little; larger factors move them more, and at 1 twenty Sinkhorn rounds
can fall short. The gradient through all of it is worked out and
checked. Next: the track's comparison (lesson D6).

## Workarounds

The dynamic layer and its gradient are written out in the program:
ask FM13 of X_eTaL-ML (`hc:d_ynamic`, `hc:s_inkhornBack`),
[`docs/xetal-ML-asks.md`](../../docs/xetal-ML-asks.md). Training with
dynamic tables waits on it.
