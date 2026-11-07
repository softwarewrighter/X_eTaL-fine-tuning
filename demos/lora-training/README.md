# LoRA training

Lesson 3. Where do the numbers of an adapter come from? One step of
training told stage by stage on an 8 x 8 layer, every number quoted
from the run; then training by Adam with a record to step back
through, the loss on the examples trained on beside the loss on
examples held out, and three experiments set up as data.

Live: [LoRA training](https://softwarewrighter.github.io/X_eTaL-fine-tuning/lora-training/)
(step through the six stages of one step, pick an experiment, move
the record). A state can be linked to:
`?mode=explorer&preset=2&stage=5&record=3`.

## The question

Lessons 1 and 2 used adapters whose numbers were made up. This lesson
is about where the numbers of A and B come from, and about the
question the lessons began with: if a model can learn something new
this way, how does anyone know it learned it rather than memorized
the few examples it was shown?

## Background

A layer `y = W x + s B (A x)` with W frozen, as in the lessons
before. Training is practice with correction: the layer answers an
example (an input with the output wanted for it), the answer is
compared with the one wanted, and the difference becomes a small
change of A and B. Here the examples come from a second layer, W + T,
where T is a shift of W (8 x 8, of rank 4) the adapter never sees:
12 inputs by a fixed ripple, their outputs from W + T, the first n
trained on and the last 4 held out, never trained on and only
measured. The adapter starts as LoRA starts, B at 0 and A seeded, so
the correction is 0 and the layer begins by giving exactly what W
gives.

## The program

```
step := @ flow:p_ipe< c#"
   Xt $x
|> "h[k,m] | r n" e:e_q< "sum j:=0..7 | A[k,j] * x[j,m]" $h
|> "u[i,m] | 8 n" e:e_q< "sum k:=0..q | B[i,k] * h[k,m]" $u
|> "p[i,m] | 8 n" e:e_q< "(sum j:=0..7 | W[i,j] * x[j,m]) + s * u[i,m]" $p
|> "g[i,m] | 8 n" e:e_q< "p[i,m] - Yt[i,m]" $g
|> "b[k,m] | r n" e:e_q< "sum i:=0..7 | B[i,k] * g[i,m]" $b
|> "dA[k,j] | r 8" e:e_q< "sum m:=0..n-1 | s * b[k,m] * x[j,m]" $dA
|> "dB[i,k] | 8 r" e:e_q< "sum m:=0..n-1 | s * g[i,m] * h[k,m]" $dB
|> "A2[k,j] | r 8" e:e_q< "A[k,j] - lr * dA[k,j]" $A2
|> "B2[i,k] | 8 r" e:e_q< "B[i,k] - lr * dB[i,k]" $B2
|> (W, A2, B2, s) $(W2, A3, B3, s2)"#
```

`lora-training.xtl` makes the task (W, T, the 12 examples, the split
by taking columns with `t_ake_2`), writes one step of gradient
descent on the n examples trained on as that pipeline of equations
(X_eTaL-ML's Flow, X_eTaL-libraries' Equations), and prints: the
task's tables; the loss per example before and after the step; how
far the step's A and B are from the LoRA library's own step
(`lo:s_gd`, 0 to rounding) and how far its W is from W (exactly 0);
then trains by Adam (`lo:a_dam`), chunks of every steps, keeping a
record after each chunk (the loss per example on the examples
trained on, on the held-out ones, and A, B and the correction s B A
as they are then), and draws the two losses as one chart (Plot's
`c_hart!`). The page runs the same program at a preset's settings,
then asks for every stage of the step (`@ flow:s_tages<`) and the
step drawn as mathematics (`@ flow:m_ath<`). The program defines no
function of its own; the task and its measures are written out (ask
FM7).

A preset is a file of settings, `presets/*.toml`, not code:

| Preset | rank | learning rate | examples trained on |
| ------ | ---- | ------------- | ------------------- |
| A first training run | 4 | 0.05 | 8 |
| Too large a learning rate | 4 | 1.0 | 8 |
| Too few examples, too large a rank | 8 | 0.05 | 3 |

## How it works

The step's stages are sums of products like lesson 2's, with one
column per example. The error g = p - y is also the loss's gradient
at the output, since the loss is half the sum of the squared errors.
Carried back through B it gives b = B' g; A's gradient is s b x' and
B's is s g h', each summed over the examples; the update moves A and
B against their gradients by the learning rate. At the start B is 0,
so b and dA are 0: on the first step only B moves. W is not in the
update, and the step gives it back as it was.

The page tells the step in six stages, each sentence quoting the
run's numbers; shows each stage's tables; and for training shows
X_eTaL's chart, says what the record shows from its numbers (the two
losses from start to end, whether the trained-on loss fell steadily,
where the held-out loss was lowest), and lets the record be moved to
show A, B and the correction at any point beside T. Moving the record
trains nothing again.

## Run it

```bash
just run lora-training       # run the program (the first preset's settings)
just serve lora-training     # its page at http://127.0.0.1:8212/
just test-demo lora-training # its CLI and browser baselines and the page's tests
```

## Summary

Where the numbers come from: from the examples, a step at a time,
predict, measure the error, carry it back to each table's gradient,
move each table a little against it, W untouched. What the held-out
loss is for: only the loss on examples never trained on says whether
what was learned carries over. On this task, at rank 4 on 8 examples
both losses fell (the held-out one more slowly); at a learning rate
of 1.0 the trained-on loss did not fall steadily; on 3 examples at
rank 8 the trained-on loss reached 0 while the held-out loss rose and
stayed up. Not shown: anything about a real model, with many
examples and noise in them; whether these settings are good ones in
general; and what happens to a layer's answers on its old inputs,
which no lesson has measured yet.

## Workarounds

- FM7 (X_eTaL-ML): no library for the learning task, its split, its
  held-out loss or its record; written out in the program as the
  LoRA explorer writes its training.
