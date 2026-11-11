# The landscape

Lesson 6. LoRA is one way to change a model; what are the others, and
how would you choose? The map of the ways to change a layer's weights,
each family's cost counted by X_eTaL, and the three worries against
what this course has measured so far.

Live: [The landscape](https://softwarewrighter.github.io/X_eTaL-fine-tuning/landscape/)
(`?mode=advanced` opens it with the program shown).

## The question

Five lessons taught one method, LoRA, in detail. A newcomer who asked
how to teach a model something new without catastrophic forgetting or
excessive time and electricity now knows one answer and wants the
others: LoRA is one way; what are the rest, and how would I choose?
This page does not choose; it lays the ways out as a map.

## Background

A layer is a table W; fine-tuning changes what it does by changing
numbers, W's own or an adapter's beside it; back-propagation finds the
gradient and Adam moves the numbers, holding two averages per number
trained. Two words added: a forward pass (one run of the layer from
input to output, nothing traced back) and a closed-form edit (a change
to W by a formula, applied once). The map's axes, from
`docs/research4.txt`: where may the change live (every weight; two
thin tables beside frozen weights; a few numbers scaling fixed, shared
tables; one rank-one edit) and how is it found (back-propagation;
forward passes alone; a formula; an AI proposing and testing), kept
apart because any answer to one pairs with any answer to the other.

## The program

```
lora := (4 lo:p_arams 8 8) c_at 16 lo:p_arams 4096 4096
trained := 9 2 r_eshape all c_at some c_at lora c_at (r * d) c_at lora c_at (lora + d) c_at (d + r) c_at all c_at 0 0
held := trained * (3 3 3 3 3 3 3 0 0) '* t_able 1 1
```

`landscape.xtl` counts, for two layers side by side (this course's
8 x 8 at rank 4 and one real 4096 x 4096 at rank 16), the numbers each
family with a count trains, holds while training beyond the weights
(the gradient and Adam's two averages under back-propagation; nothing
for a forward-only method beyond a seed; nothing for an edit) and
stores afterward when the change is kept apart from W: full and
selective fine-tuning, LoRA, LoRA-FA, QLoRA, DoRA, VeRA, learning
without back-propagation on every weight, a closed-form rank-one edit.
Then the layer's bytes at 16 and at 4 bits (`lo:b_ytes`) and the
adapters' shares of the full count. LoRA's count is the LoRA library's
`lo:p_arams`; the others are written out as their definitions give
them. Nothing is trained; every figure is analytical.

## How it works

The page: the six families, each in two or three plain sentences with
one line of its arithmetic on W and where it stands in this course
(lessons here, Saga 2 next, Sagas 3 to 5 to be planned); the grid of
the two questions with every lesson, live (a link) or to come (its
saga), in its cell, an empty cell marked as no lesson planned, and
shared bases a row of the grid rather than a variation; the account
table, every figure marked A, with the two families whose cost is not
a count named instead of shown; the three worries (forgetting, time,
cost) for each family as what its definition gives, what this course
has measured (a link to the lesson) and what is still to be measured;
the curriculum. The sentences under the table are written from the
run's counts. Tests check every count against the formula its
definition gives, that every live lesson the map names is a demo
here, and that no sentence uses a verdict word.

## Run it

```bash
just run landscape        # the counts at 8 x 8 and 4096 x 4096
just serve landscape      # its page at http://127.0.0.1:8212/
just test-demo landscape  # its CLI and browser baselines and the page's tests
```

## Summary

Six families, two questions underneath them. On a wide layer an
adapter trains and holds a small share of the layer's numbers (LoRA
at rank 16 on 4096 x 4096: 1 in 128), and the forward-only and
closed-form families hold nothing while they work; on the 8 x 8 layer
a rank-4 adapter is as large as the layer. The counts predict no loss,
no forgetting and no runtime. By definition only changing no weight
guarantees nothing is forgotten; measured so far, on lesson 5's task
every method that changed weights moved the old outputs. Not shown:
any family's results beyond what lessons 3 to 5 measured; any real
model; anything about the families not yet run beyond their
definitions.
