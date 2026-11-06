# Welcome: how these lessons work

The first page for someone new. It needs no machine learning and no
programming, takes about five minutes, and shows how the lessons work
so that nothing in the first one is a surprise.

Live: [Welcome](https://softwarewrighter.github.io/X_eTaL-fine-tuning/welcome/)

## The question

These lessons are about teaching an AI model something new without it
forgetting what it knows and without days of computing. Before the
first of them: what is a lesson here like, and how do you use what is
on its page?

## Background

Every lesson asks a question, shows one small calculation with every
number on the screen, lets you change something, and says plainly what
that showed. The numbers are real: each page runs a short X_eTaL
program in the browser and shows what it worked out.

The welcome page lets a newcomer try the seven things every lesson
uses, on one small calculation to practice on. It does not teach that
calculation; the first lesson, LoRA hello, does.

| Part | What the reader does |
| ---- | -------------------- |
| 1. Words that open | clicks a dotted-underlined word for a plain explanation; some hold a worked example |
| 2. A guess first | predicts how many numbers a stage gives, then sees |
| 3. A calculation, step by step | presses Next, Back or Play: the stage being worked out is ringed, a dot travels to it from each stage it uses, its numbers appear one after another, later stages stay blank until reached |
| 4. Pictures of the numbers | reads squares colored by value with the number in each; clicks one, or uses the arrow keys, to see which number it is |
| 5. A sum written out | sees stage 2 term by term |
| 6. A change, in words | reads what changed between two runs, compared without a verdict |
| 7. How much to show | switches between Beginner, Explorer and Advanced; the stage on show stays where it was |

Motion is switched off for a reader whose system asks for less. The
animation replays a recording of one run; it works nothing out and
changes no value.

## The program

```
y := @ flow:p_ipe< c#"
   X $x
|> "h[k] | 1" e:e_q< "sum j:=0..3 | A[k,j] * x[j]" $h
|> "u[i] | 4" e:e_q< "sum k:=0..0 | B[i,k] * h[k]" $u
|> "y[i] | 4" e:e_q< "(sum j:=0..3 | W[i,j] * x[j]) + s * u[i]""#
```

`welcome.xtl` is LoRA hello's layer written as a pipeline with
X_eTaL-ML's Flow: the input, a stage that boils it down to one number
(`h = A x`), a stage that spreads that back out (`u = B h`), and the
output (`y = W x + s u`). It prints `y`, the three tables, stage 2
written out with its sum, and the first output at a strength of 4. The
page runs the same program and then asks Flow for the value of every
stage (`@ flow:s_tages<`): that record is what the stepper replays.
The program defines no function of its own.

## Run it

```bash
just run welcome          # run the program
just serve welcome        # its page at http://127.0.0.1:8212/
just test-demo welcome    # its CLI and browser baselines and the page's tests
```

## Summary

A newcomer leaves knowing what the lessons are for, that they are
meant for them, and how to use each thing a lesson puts on its page.
They have not been taught the calculation, or anything about adapting
a model: that begins with LoRA hello.

## Workarounds

None.
