# LoRA hello

How do you teach a model something new without it forgetting what it
knows, and without retraining all of it? This is the idea most often
used, LoRA, on one layer small enough to read in full: freeze the
weights and train a small correction beside them.

Live: [LoRA hello](https://softwarewrighter.github.io/X_eTaL-fine-tuning/lora-hello/)
(no machine learning and no programming assumed: read the question
and the background, opening any underlined word for a plain
explanation; see the layer's 24 numbers; change the input or the
strength and follow one calculation worked in five steps; then what
that showed and what it did not. The code is folded into asides for
those who want it, each library function explained from its own
source).

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

`lora-hello.xtl` is short. It opens one library, makes the layer, and
prints seventeen lines, which are the numbers the page shows. You do
not need to read X_eTaL to follow the page; if you want to, the page's
last section says how, and every function name there is a link to its
entry in the cross-reference.

Two lines carry the idea. The first opens the LoRA library of
X_eTaL-ML and gives it the short name `lo:`. The second hands the
layer and its adapter, as one bundle, and the input `x` to that
library's function `f_orward`, and calls the result `y`:

```
"lo:" u_se< "LoRA"
y := (W, A, B, s) lo:f_orward x
```

`lo:f_orward` is one line in the library
([its source](https://github.com/softwarewrighter/X_eTaL-ML/blob/6bac4529ddd73e8d2d16925107ada8083f84f99d/libs/LoRA/src/LoRA.xtl#L36)):

```
l:f_orward := { (W, A, B, s) x -> (W '+ '* i_nner x) + s * B '+ '* i_nner A '+ '* i_nner x }
```

Read from the right, as X_eTaL does: `A '+ '* i_nner x` is A times x
(the squeeze); `B '+ '* i_nner` that is B times it (the spread); `s *`
scales it; and `(W '+ '* i_nner x) +` adds what the frozen layer
gives. `'+ '* i_nner` is how "multiply a table by a list" is written:
pair the numbers up, multiply, add. The table `s B A` is never built.

What the program prints, in order:

| Line | What | Value at the start |
| ---- | ---- | ------------------ |
| `r_avel W`, `r_avel A`, `r_avel B` | the layer's numbers: the frozen table, the adapter's row and column | 16, 4 and 4 numbers |
| `s c_at h` | the strength `s`, then `h = A x`, the input squeezed to one number | `2.0 3.5` |
| `x * r_avel A` | the squeeze written out: each number of A times the matching number of x | `0.5 -1.0 3.0 1.0` |
| `s * h` | the squeezed number times the strength | `7.0` |
| `x * 4 t_ake r_avel W` | the first row of W times x, written out | 4 numbers, adding up to the first of `base` |
| `base` | what the frozen layer gives, `W x` | 4 numbers |
| `y` | what the adapted layer gives | `base` plus the correction |
| `correction` | what the adapter adds, `s B (A x)` | `7 0 -7 3.5` |
| a check | the two outputs differ by exactly the correction | `1` (yes) |
| `r_avel table` | the correction as a table, `s B A` (`lo:d_elta`) | 16 numbers: every row is A times one number |
| `lo:r_ank table` | the rank of that table | `1` |
| a check | the merged table `W + s B A` (`lo:m_erge`) gives the same output | `1` (yes) |
| `counts` | numbers trained (`1 lo:p_arams 4 4`), beside the table's | `8 16` |
| `big` | the same for a table of 4096 rows and columns at rank 16 | `131072 16777216` |
| the percentage | the first of those as a share of the second | `0.78125` |

The program defines no function of its own: it uses six functions of
the LoRA library (`s_caling`, `f_orward`, `d_elta`, `m_erge`,
`r_ank`, `p_arams`) and X_eTaL's built-in words.

## How it works

`A x` is one number, `0.5 x 1 + -0.5 x 2 + 1 x 3 + 0.25 x 4 = 3.5`: the
input squeezed through the adapter's rank, 1. Times the strength,
`s = 2`, that is 7. `B` spreads it over the 4 outputs: `7 x 1, 7 x 0,
7 x -1, 7 x 0.5 = 7 0 -7 3.5`. That is the correction, all the adapter
adds; where B has a 0 (the second output) nothing is added. `W` is
never touched.

Written out as a table, the correction is `s B A` (the library calls
it the update; `lo:d_elta`): 4 rows and 4 columns, but every row is
`A` times one number, so its rank is 1 (`lo:r_ank` counts the rows
that say something new). `lo:m_erge` adds it to `W`, giving one
ordinary table that does the work of both: how an adapted model is
made to run as fast as the original. It makes a new table; `W` itself
is kept.

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

- **Forgetting.** Not shown: nothing is trained here. What is shown
  is why the idea helps: the adapter never touches `W`. With alpha at
  0, or the adapter taken away, the layer answers exactly as before:
  nothing is overwritten, and one model can carry an adapter per
  thing taught. Not promised: while an adapter is on, the output changes for every
  input its `A` responds to. Studies find LoRA forgets less than full
  fine-tuning, not nothing (Biderman and others, 2024, "LoRA Learns
  Less and Forgets Less").
- **Time and cost.** Not measured. What is shown is a count: under 1%
  of a real layer's numbers would be trained at rank 16, so there is
  less to hold and update at each step and a small file to keep. The
  frozen weights still take part in every step: cheaper, not free.
- **What you give up.** A correction of rank `r` can only say so much.

Not shown here: how `A` and `B` are learned, how to choose the rank,
and how to tell what a model learned and lost. Those are the lessons
that follow ([the plan](../../docs/plan.md)). For the full mathematics
of one adapted layer, see X_eTaL-ML's
[LoRA explorer](https://softwarewrighter.github.io/X_eTaL-ML/lora-explorer/).

## Workarounds

None.
