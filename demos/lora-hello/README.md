# LoRA hello

How do you teach a model something new without it forgetting what it
knows, and without retraining all of it? This is the idea most often
used, LoRA, on one layer small enough to read in full: freeze the
weights and train a small correction beside them.

Live: [LoRA hello](https://softwarewrighter.github.io/X_eTaL-fine-tuning/lora-hello/)
(read the question and the background, then change the four numbers
of the input or alpha; the page runs the program again and shows the
frozen output, what the adapter adds and the adapted output, the cells
that changed outlined; it ends with what that showed and what is
next).

![LoRA hello](screenshot.png)

## The question

You have a language model running on your own machine and you want to
teach it something new: your project's conventions, a subject it is
weak in. Three worries come with that:

- **Will it forget?** Teaching a model one thing can damage what it
  already knew (catastrophic forgetting).
- **How long will it take?** Training a whole model is days of
  computing on hardware most people do not have.
- **What will it cost?** That computing is memory and electricity.

## Background

A language model is a very large collection of numbers, its weights,
kept in tables (matrices): several billion in a model that fits on a
laptop. It works by multiplying its input by one table after another;
each step is a layer. Fine-tuning changes those numbers so the
model's answers move toward the ones you want.

Changing all of them is where the worries come from. A number being
trained needs its gradient and, with the usual optimizer, two running
averages: about four numbers held for each one trained. And the new
numbers replace the old ones, where what the model knew was kept.

LoRA (low-rank adaptation) leaves the weights alone. Beside a table
`W` it puts a small correction made of two thin tables, `A` and `B`,
and trains only those:

```
y = W x + s B (A x)
```

`A` squeezes the input to a few numbers (how many is the rank, `r`),
`B` spreads them back out, and `s = alpha / r` is a strength. A
correction through so narrow a middle cannot say everything, but it
needs very few numbers; LoRA's bet is that what a model must change to
learn one thing fits.

This demo is one such layer shrunk to 4 x 4, with an adapter of rank
1 (`A` is 1 x 4, `B` is 4 x 1). Nothing is trained: the adapter's
numbers are made up, to show what an adapter is, where it sits, what
it adds and what it costs, before the lessons on how its numbers are
learned.

## The program

```
y := (W, A, B, s) lo:f_orward x
```

`lora-hello.xtl` makes `W` by a fixed ripple, writes `A`, `B` and one
input `x` as literals, and prints, with X_eTaL-ML's LoRA library:

| Line | What | Value |
| ---- | ---- | ----- |
| `s c_at h` | the scale `s`, then `h = A x`, the input squeezed to one number | `2.0 3.5` |
| `base` | the frozen layer's output, `W x` | 4 numbers |
| `y` | the adapted layer's output | `base` plus the correction |
| `correction` | what the adapter adds, `s B (A x)` | `7 0 -7 3.5` |
| the check | the outputs differ by exactly the correction | `1` |
| `lo:r_ank lo:d_elta (A, B, s)` | the rank of the update `s B A` | `1` |
| the merge check | `W + s B A` gives the same output as the factored layer | `1` |
| `counts` | numbers trained (`1 lo:p_arams 4 4`), against the matrix's | `8 16` |
| `big` | the same on a 4096 x 4096 layer at rank 16 | `131072 16777216` |
| the percentage | the first of those as a share of the second | `0.78125` |

## How it works

`A x` is one number, `0.5 - 1 + 3 + 1 = 3.5`: the input squeezed
through the adapter's rank, 1. `B` spreads it back over the 4 outputs,
and `s = 2` scales it: `2 * 3.5 * B = 7 0 -7 3.5`. That is all the
adapter adds; `W` is never touched.

The update `s B A` is a whole 4 x 4 matrix, but every row is a
multiple of `A`, so its rank is 1 (`lo:r_ank`, by Gram-Schmidt).
`lo:m_erge` folds it into `W`, giving one ordinary matrix that
computes the same layer: how an adapter is deployed, as fast as the
original.

The adapter trains `r (m + n)` numbers where the matrix has `m n`. On
this layer that is 8 of 16, no saving. On a 4096 x 4096 table (one of
a 7-billion-parameter model) at rank 16 it is 131,072 of 16,777,216,
0.78%: the adapter's count grows with the table's sides, the table's
with their product.

## Run it

```bash
just run lora-hello          # run the program
just show lora-hello         # as a notebook: each statement, then its output
just serve lora-hello        # its page at http://127.0.0.1:8212/
just test-demo lora-hello    # its CLI and browser baselines and the page's tests
```

## Summary

Against the three worries:

- **Forgetting.** `W` never changes. With alpha at 0, or the adapter
  taken away, the layer answers exactly as before: nothing is
  overwritten, and one model can carry an adapter per thing taught.
  Not promised: while an adapter is on, the output changes for every
  input its `A` responds to. Studies find LoRA forgets less than full
  fine-tuning, not nothing (Biderman and others, 2024, "LoRA Learns
  Less and Forgets Less").
- **Time and cost.** Under 1% of a real layer's numbers are trained at
  rank 16, so there is less to hold and update at each step and a
  small file to keep. That is a count, not a measurement of time or
  power, and the frozen weights still take part in every step:
  cheaper, not free.
- **The price.** A correction of rank `r` can only say so much.

Not shown here: how `A` and `B` are learned, how to choose the rank,
and how to tell what a model learned and lost. Those are the lessons
that follow ([the plan](../../docs/plan.md)). For the full mathematics
of one adapted layer, see X_eTaL-ML's
[LoRA explorer](https://softwarewrighter.github.io/X_eTaL-ML/lora-explorer/).

## Workarounds

None.
