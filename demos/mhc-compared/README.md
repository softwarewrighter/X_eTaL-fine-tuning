# mHC compared

Summary D6, the last page of the depth track (a side track on
DeepSeek's mHC, `docs/mhc-plan.md`). The plain skip, Hyper-Connections
(HC) and mHC side by side at equal depth, width and steps: the
arithmetic of the starting tables, the nine training runs of
[Training at depth](../training-at-depth/README.md) read again, the
Python re-implementation's figures and the paper's quoted beside them,
each figure marked measured here, worked out, or quoted.

Live: [mHC compared](https://softwarewrighter.github.io/X_eTaL-fine-tuning/mhc-compared/)
(pick a depth). A state can be linked to: `?mode=explorer&depth=24`.

## The question

Set side by side, what did the three ways of connecting layers do, and
how much of it was arithmetic, how much training, and how much is only
reported by others: the Python re-implementation
([mHC-poc](https://github.com/softwarewrighter/mHC-poc), its
`docs/results.md`) and the paper (Xie and others, arXiv:2512.24880)?

## Background

The gain of a product of mixing tables: the paper's forward gain (the
largest row sum), its backward gain (the largest column sum), and the
Python's proxy (the largest number). The fourth page trained the three
ways on a tiny task (16 tokens, each followed by the next) at 12, 24
and 48 layers for 800 steps, offline, and recorded every step. This
page reads those records; it trains nothing. The Python's run is of
another task and size (256 tokens, sequences of 128, attention, a
model 32 times as wide, 32-bit numbers); the paper's of another scale
again (mixture-of-experts models of 3 to 27 billion weights).

## The program

`mhc-compared.xtl` works out HC's starting table (the exponential of
2I - 1) and mHC's (twenty Sinkhorn-Knopp rounds of it), their products
over 12, 24 and 48 layers and both measures of each; reads the nine
records (`data/SCHEME-L.txt`, copied from training-at-depth's, with
its `data/runs.txt`) and reduces each to ten figures; reads the
Python's final losses and proxies (`data/python-loss.txt`,
`data/python-proxy.txt`, quoted from its results page, rows the plain
skip, HC, mHC, columns 12, 24, 48 layers); draws two charts by depth;
counts the numbers each way trains; and runs six checks (Check). It
defines no function of its own.

## How it works

At the start HC's forward gain is 10^6.99, 10^13.97 and 10^27.95 at
12, 24 and 48 layers (its row sum 3.82 to the power of the depth) and
its proxy 10^6.39, 10^13.37, 10^27.35; mHC's forward gain is 1 and its
proxy 10^-0.602, one quarter. After training: final losses 0.0028,
0.0030 and 2.7726 for the plain skip, 0.0029, 0.0029, 0.0027 for HC,
0.0029, 0.0027, 0.0027 for mHC (guessing evenly, 2.77); HC's forward
gain 10^6.93, 10^14.01, 10^28.13; mHC's within 3.0e-6 of 1 at every
step; no NaN event. The Python's: final losses 1.78, 1.86, 3.79;
0.0001, 0.0001, 5.54; 0.002, 0.0002, 0.0002 and proxies 7.0, 14.4,
27.3 for HC and -0.5, -0.6, -0.6 for mHC. The paper's (table 4, 27B):
BBH, DROP, GSM8K, MMLU, MATH, mHC highest on four, HC on MATH; mHC's
loss 0.021 under the baseline's; HC's composite gain peaks near 3000,
mHC's near 1.6; 6.7% more time. Per layer the block has 280 numbers,
the static mixing 36, the paper's input-dependent mixing 795 at this
width.

## Run it

```bash
just run mhc-compared          # run the program
just test-demo mhc-compared    # its CLI and browser baselines (reg-rs)
just serve mhc-compared        # the page at http://127.0.0.1:8212/
```

## Summary

The bound is arithmetic: mHC's products keep a gain of exactly 1, HC's
grow by the row sum at every layer. On the toy task, training kept
both as the arithmetic said; all three learned at 12 and 24 layers,
and at 48 the plain skip did not while HC and mHC did. The Python's
run agrees on the gains and on mHC, and differs on HC and the plain
skip; the paper reports mHC ahead on four of five benchmarks at 27B.
What the toy did not show: attention, sequences, scale, training with
input-dependent tables, memory cost. This closes the depth track.

## Workarounds

The mixing, the Sinkhorn rounds and the gain measures are written out
in the program: ask FM13 of X_eTaL-ML,
[`docs/xetal-ML-asks.md`](../../docs/xetal-ML-asks.md).
