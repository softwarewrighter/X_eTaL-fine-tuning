# LoRA lab

Lesson 2. How is an adapter's correction worked out, number by number,
and what does its rank change? One layer of 64 frozen numbers with an
adapter of rank 1 to 8 beside it, every number in view.

Live: [LoRA lab](https://softwarewrighter.github.io/X_eTaL-fine-tuning/lora-lab/)
(take the calculation a stage at a time, click any square for the sum
behind its number, pick a letter of the formulas, move the rank). A
state can be linked to: `?mode=explorer&rank=5&stage=4&pick=4.3&letter=A`.

## The question

Lesson 1 showed what an adapter adds to a layer whose weights stay as
they are, and left two questions open: how exactly is that correction
worked out, and what does the adapter's size, its rank, change and
cost?

## Background

The layer is the one of X_eTaL-ML's LoRA explorer, with its made-up
numbers: a frozen table `W` of 8 rows and 8 columns, an adapter `A`
(`r` rows of 8) and `B` (8 rows of `r`), one input `x`, and the
strength `s = alpha / r`. The layer `y = W x + s B (A x)` is written
as a pipeline of four stages, so that every stage's numbers can be
shown: the input `x`; the squeeze `h = A x` (`r` numbers); the spread
`u = B h` (8 numbers); the output `y = W x + s u`.

Nothing is trained: `A` and `B` are made up, so the page shows how a
correction is computed and what a rank costs, not whether a rank makes
a model better at anything.

## The program

```
y := @ flow:p_ipe< c#"
   X $x
|> "h[k] | r" e:e_q< "sum j:=0..7 | A[k,j] * x[j]" $h
|> "u[i] | 8" e:e_q< "sum k:=0..q | B[i,k] * h[k]" $u
|> "y[i] | 8" e:e_q< "(sum j:=0..7 | W[i,j] * x[j]) + s * u[i]""#
```

`lora-lab.xtl` sets the rank, makes the layer, runs it as that
pipeline (X_eTaL-ML's Flow and X_eTaL-libraries' Equations), checks
the pipeline's output against the LoRA library's own `lo:f_orward`,
and prints: `y`; `W x` and the correction `s B (A x)`; the check;
`r lo:p_arams 8 8` beside 64, and `s`; the three tables; and every
product of the three sums (each number of `A` times the matching
number of `x`, of `B` times `h`, of `W` times `x`), so that the sum
behind any number of any stage can be written out from printed
numbers. The page runs the same program with the rank it sets, then
asks for every stage's value (`@ flow:s_tages<`) and for each stage's
formula with its symbols marked (`e:t_exSymbols<`, ask FL1, landed).
The program defines no function of its own.

| At rank r | numbers trained | the squeeze |
| --------- | --------------- | ----------- |
| 1 | 16 | 1 number |
| 2 | 32 | 2 numbers |
| 4 | 64, as many as W | 4 numbers |
| 8 | 128, twice W | 8 numbers |

## How it works

Each stage is one sum of products. `h[k]` is row `k` of `A` times
`x`, added up; `u[i]` is row `i` of `B` times `h`; `y[i]` is row `i`
of `W` times `x`, plus `s` times `u[i]`. The page writes any of them
out on request, to 3 decimals, from the products X_eTaL printed; a
test checks that every such sum, at ranks 1, 2, 5 and 8, adds up to
the number it is shown for.

The formulas are drawn by KaTeX from the LaTeX Equations writes, with
each array reference and named scalar wrapped in a class
(`\htmlClass{sym sym-A}{...}`): a click on a letter, or on its button,
says what it stands for and outlines its picture. Flow's own
`flow:m_ath<` cannot draw the marked form yet (ask FM11), so the page
asks Equations for each stage's formula itself.

Moving the rank runs the program again. What changed is said from the
two runs' numbers: the adapter's count beside W's 64, the strength
(alpha / r), the shape of `h`; and that W did not change.

## Run it

```bash
just run lora-lab          # run the program
just serve lora-lab        # its page at http://127.0.0.1:8212/
just test-demo lora-lab    # its CLI and browser baselines and the page's tests
```

## Summary

How the correction is worked out: three small sums of products,
squeeze, spread, add, every number of each in view. What the rank is:
the number of numbers in the middle stage, the rows of `A` and the
columns of `B`. What it costs: `r (8 + 8)` numbers here, passing W's
64 at rank 4. Not shown: whether a larger rank makes a model better at
anything, since nothing was trained; that is for the lessons on
training.

## Workarounds

- FM11 (X_eTaL-ML): Flow's `flow:m_ath<` draws through the unmarked
  `e:t_ex<`; the page calls `e:t_exSymbols<` on each stage's equation
  itself, with the stage text taken from the pipeline.
