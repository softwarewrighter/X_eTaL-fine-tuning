# Shell gallery

The parts every lesson here is built from, each shown working on one
small computation. It is a page for looking the parts over, not a
lesson, and it is not listed in the catalog.

Live: [Shell gallery](https://softwarewrighter.github.io/X_eTaL-fine-tuning/shell-gallery/)

## The question

A lesson has to explain a calculation to someone who knows neither
machine learning nor X_eTaL. What does it have to work with, and does
each piece work, read clearly, and answer to a keyboard and to touch?

## Background

The parts are `shared/lesson`, the teaching shell on top of
X_eTaL-ML's page shell. All of them are presentation: they arrange
what one run of an X_eTaL program printed, and compute no number
themselves.

| Part | What it does here |
| ---- | ----------------- |
| words that open (`eli5`) | a dotted-underlined word opens a plain explanation; some have a worked example to open, and the lesson that teaches them |
| a question first (`predict`) | how many numbers come out of the squeeze? The answer is read from the run |
| the stepper (`stepper`) | the pipeline's four stages, forward, back and played, from the record `flow:s_tages<` gave; nothing is run again |
| heat maps (`heat`) | each stage's value, with a legend, a note on the shared scale, and an inspector (click a cell, or tab to a map and use the arrow keys); what the stage writes is outlined solid, what it reads dashed |
| a sum written out (`arithmetic`) | the first row of W times x, term by term |
| a change in words (`narrate`) | the numbers trained at rank 1 and rank 2, compared without a verdict |
| three depths (`mode`) | Beginner, Explorer (adds the tables W, A, B), Advanced (adds the symbols and the code); switching keeps the step you are on |

The computation is LoRA hello's layer, written as a pipeline with
X_eTaL-ML's Flow: the input, the squeeze `h = A x`, the spread
`u = B h`, the output `y = W x + s u`.

## The program

```
y := @ flow:p_ipe< c#"
   X $x
|> "h[k] | 1" e:e_q< "sum j:=0..3 | A[k,j] * x[j]" $h
|> "u[i] | 4" e:e_q< "sum k:=0..0 | B[i,k] * h[k]" $u
|> "y[i] | 4" e:e_q< "(sum j:=0..3 | W[i,j] * x[j]) + s * u[i]""#
```

`shell-gallery.xtl` makes the layer, runs it as that pipeline, and
prints `y`, the layer's numbers, the first row of W times x with its
sum, and the numbers an adapter trains at rank 1 and 2. The page runs
the same program and then asks Flow for the value of every stage
(`@ flow:s_tages<`), which is the trace the stepper moves through. The
program defines no function of its own.

## Run it

```bash
just run shell-gallery          # run the program
just serve shell-gallery        # its page at http://127.0.0.1:8212/
just test-demo shell-gallery    # its CLI and browser baselines and the page's tests
```

## Summary

Six parts, each only arranging what one run printed. What it does not
show: a lesson. The parts are checked here for working; whether they
teach is checked lesson by lesson, by a cold reading
([`docs/new-eyes.md`](../../docs/new-eyes.md)).

## Workarounds

None.
