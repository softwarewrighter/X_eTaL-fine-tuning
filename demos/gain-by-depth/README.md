# The gain by depth

Lesson D3, the third page of the depth track (a side track on
DeepSeek's mHC, `docs/mhc-plan.md`). What happens when the same mixing
table is applied at every one of 48 layers? HC's table and mHC's, each
multiplied by itself once per layer, every product shown and measured
two ways, from the tables alone; the Python re-implementation's
trained figures and the paper's quoted beside them; a slider shifting
the logits.

Live: [The gain by depth](https://softwarewrighter.github.io/X_eTaL-fine-tuning/gain-by-depth/)
(move the shift, step through the layers). A state can be linked to:
`?mode=explorer&shift=1&layer=48`.

## The question

The paper (Xie and others, arXiv:2512.24880) reports that with HC the
gain of the carried state reached about 3000 in its 27-billion-weight
model, and with mHC about 1.6; the Python re-implementation
([mHC-poc](https://github.com/softwarewrighter/mHC-poc)) measured its
own figure at 12, 24 and 48 layers. Both are from training. How much
of it is in the tables before any training at all?

## Background

Applying a table at every layer multiplies by it once per layer; the
product of k layers' tables is the composite mapping, H^k when every
layer has the same H. Two measures of a table, kept apart and named:
the paper's Amax Gain Magnitude (section 3.1: the largest absolute row
sum, the forward gain; the largest absolute column sum, the backward
gain) and the Python's proxy (log10 of the largest absolute entry; its
code says it is not the paper's metric). HC's table is the exponential
of the logits 2I - 1; mHC's is its Sinkhorn-Knopp projection (lesson
D2). The slider adds the same shift c to every logit.

## The program

`gain-by-depth.xtl` makes the two tables from the logits at the shift
set, multiplies each by itself 48 times in a `p_ower` loop keeping the
product and its three measures after every layer, prints the line
k log10 s that HC's forward gain follows, the row sum and 48-layer
gain at every shift the slider offers, draws the four measures as one
chart (Plot's `c_hart!`), and closes with four checks (Check's
`t_est`, `t_rue`, `a_nd`, `r_eport`). The Sinkhorn loop and the two
measures are written out (ask FM13). The program defines no function
of its own. The shift is a slider over -2 to 1 by a quarter; the
program's own setting is 0, the paper's start.

## How it works

Every row of HC's table holds the same numbers in another order, so
every row sums to the same s, and every row of H^k sums to s^k: the
forward gain after 48 layers is 3.82^48, about 10^27.9, at the paper's
start. mHC's rows and columns sum to 1, so both its gains are 1 at
every depth, and its entries flatten toward 1/4 (proxy -0.602, the
four streams averaged). The untrained proxy for HC, 6.39, 13.37 and
27.35 at 12, 24 and 48 layers, is within about one power of ten of the
Python's trained 7.0, 14.4 and 27.3; for mHC, -0.598, -0.602, -0.602
against -0.5, -0.6, -0.6. Shifting the logits multiplies HC's table by
e^c: its row sum crosses 1 between -1.5 (0.853, a gain of 4.8e-4 after
48 layers) and -1.25 (1.095, a gain of 78); mHC's table is the same at
every shift. Figures from this program are analytical; the Python's
are its run's, the paper's are quoted.

What the page does not show: a list sent through the layers (lesson
D1); tables that differ from layer to layer; training (lesson D4);
anything at the paper's scale.

## Run it

```bash
just run gain-by-depth          # run the program
just show gain-by-depth         # as a notebook: each statement, then its output
just test-demo gain-by-depth    # its CLI and browser baselines (reg-rs)
just serve gain-by-depth        # the page at http://127.0.0.1:8212/
```

## Summary

Repeating one table multiplies the gain by its row sum at every
layer: HC's grows without bound unless its rows happen to sum to
exactly 1, mHC's is 1 at any depth by construction. The paper's
measure and the Python's give different numbers for the same table
and are quoted by name. Most of the growth the Python measured after
training is already in the table it started from. Next: the three
schemes trained at three depths (lesson D4).

## Workarounds

The Sinkhorn-Knopp loop, the forward and backward gains and the proxy
are written out in the program: ask FM13 of X_eTaL-ML
(`hc:s_inkhorn`, `hc:a_maxGain`, `hc:g_ainProxy`),
[`docs/xetal-ML-asks.md`](../../docs/xetal-ML-asks.md).
