# Residual streams

Lesson D1, the first page of the depth track (a side track on
DeepSeek's mHC, `docs/mhc-plan.md`). Why does a deep model need a skip
connection, and what happens when there are four of them? A list of 8
numbers sent through 48 small layers with the skip and without,
measured after every layer at three weight scales set up as data;
then Hyper-Connections: four streams, the three tables that mix them,
and one layer stepped through in nine stages with every number shown,
its carried state mixed by HC's table or by mHC's.

Live: [Residual streams](https://softwarewrighter.github.io/X_eTaL-fine-tuning/residual-streams/)
(pick a weight scale, choose HC or mHC, step through the layer). A
state can be linked to: `?mode=explorer&preset=2&hres=hc&stage=8`.

## The question

The lessons before changed one layer of a model. A real model has
dozens in a row, and before any of them can be taught anything a list
of numbers must get through all of them without being lost on the
way or blown up out of proportion. Almost every deep model does this
with a skip connection; mHC (Xie and others, arXiv:2512.24880) is
about what happens when that connection is widened into several
streams side by side.

## Background

A layer's work here is a small block: a table turning 8 numbers into
16, GELU applied to each, a second table turning them back into 8;
the tables' numbers are fixed by a formula, a different set for each
of the 48 layers, and nothing is trained. A stack is the layers one
after another. Two lists are sent through, the second a hundredth
away from the first in its first number; the gap between them after
each layer, as a share of the gap sent in, says whether the stack
still tells its inputs apart (1.0: the difference went through as it
was; 0: the two inputs give the same output). Each layer is used two
ways: replacing the list by the block's output, or adding the block's
output to the list it was given, which is the skip (residual)
connection.

Hyper-Connections widen the one list into S = 4 streams, a 4 x 8
table. A layer mixes the streams by a table H_pre, averages them into
one list for its block, runs the block once, hands the output back to
the streams in the shares H_post, mixes the carried state by H_res,
and adds the two. The tables start from the logits 2I - 1: H_pre and
H_post are their row softmax (rows summing to 1); H_res is their
exponential (HC: rows summing to 3.82) or that exponential projected
by 20 rounds of Sinkhorn-Knopp (mHC: rows and columns summing to 1).
The row sums are the identity-mapping property the paper names: a row
summing to 1 keeps the streams' average across any depth.

## The program

```
layer := @ flow:p_ipe< c#"
   X $X
|> "P[s,j] | S D" e:e_q< "sum t:=0..3 | Hpre[s,t] * X[t,j]" $P
|> "m[j] | D" e:e_q< "(sum s:=0..3 | P[s,j]) / 4" $m
|> "z[k] | F" e:e_q< "(sum j:=0..7 | m[j] * W1[j,k]) + W1[8,k]" $z
|> act:g_elu z $h
|> "f[j] | D" e:e_q< "(sum k:=0..15 | h[k] * W2[k,j]) + W2[16,j]" $f
|> "Fs[s,j] | S D" e:e_q< "Hpost[s] * f[j]" $Fs
|> "R[s,j] | S D" e:e_q< "sum t:=0..3 | Hres[s,t] * X[t,j]" $R
|> "X2[s,j] | S D" e:e_q< "R[s,j] + Fs[s,j]" $X2"#
```

`residual-streams.xtl` makes the 48 blocks' tables from one ripple
at the scale set, sends the two lists through with the skip and
without (NN's `d_ense`, Activations' `g_elu`, a `p_ower` loop keeping
the size and the gap after each layer), draws the gaps as one chart
(Plot's `c_hart!`); makes the three mixing tables from the logits
2I - 1 (NN's `s_oftmax`; the exponential; the Sinkhorn projection as
a `p_ower` loop of a row normalization and a column normalization,
written out: ask FM13), prints their row and column sums; runs the
first layer on four copies of the list, and writes the second layer
as that pipeline of equations (X_eTaL-ML's Flow, X_eTaL-libraries'
Equations) on the state after the first, checking it against the same
layer by matrix products (0 to rounding). The page runs the same
program at a preset's scale and with the table chosen, then asks for
every stage of the layer (`@ flow:s_tages<`) and the layer drawn as
mathematics (`@ flow:m_ath<`). The program defines no function of its
own.

A preset is a file of settings, `presets/*.toml`, not code:

| Preset | scale of the blocks' weights |
| ------ | ---------------------------- |
| Small weights | 0.2 |
| Larger weights | 0.5 |
| Largest weights | 0.8 |

## How it works

Part 1 is forward only: no loss, no gradient. At scale 0.2 the gap
between the two lists is 0.09 of what was sent in after one layer
with the blocks replacing the list and is gone to the last digit by
layer 24, while with the skip it stays between 1.0 and 1.6 through all
48 layers: the plain stack forgets its input, the skip keeps it. At
0.8 the gap grows past 1 both ways: the skip does not keep the
corrections from outgrowing the list. The page quotes the run at
five depths and says only what the numbers show.

Part 2's three tables are 4 x 4 and 1 x 4; H_res's rows sum to
e + 3/e = 3.82 for HC and to 1.0 for mHC, both ways, printed by the
program. Part 3's stages are sums of products on the 4 x 8 state,
one GELU, and the shares; the stage texts quote H_pre's and H_post's
weights and HC's row sum from the run.

What the page does not show: anything trained (the tables are fixed
by a formula); what a product of 48 mixing tables does (lesson D3);
the paper's input-dependent mixing (lesson D5); anything at the
paper's scale.

## Run it

```bash
just run residual-streams          # run the program
just show residual-streams         # as a notebook: each statement, then its output
just test-demo residual-streams    # its CLI and browser baselines (reg-rs)
just serve residual-streams        # the page at http://127.0.0.1:8212/
```

## Summary

A skip connection keeps a deep stack from forgetting its input,
measured: the gap between two inputs a hundredth apart survives 48
layers with it and is gone within a dozen without. It does not keep
corrections from outgrowing the list. Hyper-Connections carry four
streams and three small mixing tables; HC and mHC differ in one, the
table mixing the carried state, and in whether its rows and columns
sum to 1, which is what keeps the streams' average across depth.
Next: Sinkhorn-Knopp, the projection mHC uses, step by step (lesson
D2).

## Workarounds

The stream mixing and the Sinkhorn-Knopp projection are written out
in the program (the mixes as equations and matrix products, the
projection as a `p_ower` loop of two normalizations): ask FM13 of
X_eTaL-ML, [`docs/xetal-ML-asks.md`](../../docs/xetal-ML-asks.md).
