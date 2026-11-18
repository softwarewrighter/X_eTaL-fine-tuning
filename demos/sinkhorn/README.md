# Sinkhorn-Knopp

Lesson D2, the second page of the depth track (a side track on
DeepSeek's mHC, `docs/mhc-plan.md`). How is a table of positive
numbers made into one whose rows and columns each sum to 1, the table
mHC mixes its streams with? Two divisions, repeated: one round shown
with every number, twenty rounds stepped through with the sums
converging from three starts set up as data, and the three properties
the paper states of the result checked by the program.

Live: [Sinkhorn-Knopp](https://softwarewrighter.github.io/X_eTaL-fine-tuning/sinkhorn/)
(pick a start, step through the rounds, read the checks). A state can
be linked to: `?mode=explorer&preset=2&round=20`.

## The question

Lesson D1 ended on a rule: mHC replaces the table that mixes its
carried state by one whose rows and columns each sum to 1 (doubly
stochastic), and that keeps four streams in bounds across any depth.
How is such a table made from whatever numbers a model has learned,
and what can be said for certain about the table that comes out? The
paper (Xie and others, arXiv:2512.24880) makes it by Sinkhorn-Knopp
(1967) and cuts it at twenty rounds.

## Background

A table here is 4 x 4, the size mHC uses for four streams. The start
is the exponential of a table of logits, so it is positive whatever
the logits are. A round is two steps: every row divided by its sum
(the rows then sum to 1), then every column divided by its sum (the
columns then sum to 1, and the rows are off again, by less than
before). A sum counts as 1 here when it is within a millionth. The
paper states three properties of a doubly stochastic table (section
4.1): its largest singular value is at most 1, so it lengthens no
list; the product of two is one; it is a weighted mix of permutation
tables (Birkhoff, 1946). The program checks each on the table it
produced, with X_eTaL-libraries' Check library.

## The program

```
round := @ flow:p_ipe< c#"
   A $A
|> "R[i,j] | S S" e:e_q< "A[i,j] / (sum t:=0..3 | A[i,t])" $R
|> "C[i,j] | S S" e:e_q< "R[i,j] / (sum t:=0..3 | R[t,j])" $C"#
```

`sinkhorn.xtl` makes the start from the logits, prints its row and
column sums, runs one round as that pipeline of equations (X_eTaL-ML's
Flow, X_eTaL-libraries' Equations), then the twenty rounds as a
`p_ower` loop keeping every table and every sum after each half round
and how far the farthest sum is from 1 after each round (Plot's
`c_hart!` draws it), and counts the rounds to a millionth. Then the
three properties: the largest singular value of the start and of the
result by power iteration on `H' H`, written out (ask FL2), and one
list before and after; a second table projected and the product's
sums; a greedy search over the 24 orderings of four
(Combinatorics' `p_ermutations`) writing the result as a weighted mix
of permutation tables. Eight checks (Check's `t_est`, `t_rue`,
`a_nd`, `r_eport`) close the program. The page runs the same program
at a preset's start, then asks for every stage of the round (`@
flow:s_tages<`) and the round drawn as mathematics (`@ flow:m_ath<`).
The program defines no function of its own.

A preset is a file of settings, `presets/*.toml`, not code:

| Preset | the logits |
| ------ | ---------- |
| The paper's start | 1 on the diagonal, -1 off it (2I - 1) |
| A lopsided start | sixteen numbers between -1 and 2 with no pattern |
| A nearly split start | 0 in a 3 x 3 block and in the last corner, -5 elsewhere |

## How it works

From the paper's start every row and column of the exponential sums
to the same 3.82, so one round is exact: 0.711 on the diagonal, 0.096
elsewhere, the same table lesson D1 showed. From the lopsided start
the farthest row sum after each round falls by a factor of about 3.5
a round and is within a millionth after eleven rounds. From the
nearly split start (the fourth row and column barely connect to the
rest) it falls slowly and is still about 2.5e-4 off after twenty:
four of the eight checks report FAIL, as they should, and the page
says that twenty rounds is the paper's practical cut, not a
guarantee. The largest singular value of the result is 1.000000 in
every case (the start's is 3.82, 9.59 and 3.50); the Python's
`sinkhorn_doubly_stochastic` (mHC-poc, `mlx/src/mhc.py`, MIT) is set
beside the loop line for line in the page's Advanced aside.

What the page does not show: why mHC needs the properties at depth
(lesson D3); the gradient of the projection (lesson D4); any proof
(Birkhoff's theorem is shown on one table; the singular value is found
by iteration); anything about a real model.

## Run it

```bash
just run sinkhorn          # run the program
just show sinkhorn         # as a notebook: each statement, then its output
just test-demo sinkhorn    # its CLI and browser baselines (reg-rs)
just serve sinkhorn        # the page at http://127.0.0.1:8212/
```

## Summary

A doubly stochastic table is made by two divisions repeated; how many
rounds it takes depends on the start, from one to more than the
paper's twenty. What comes out stretches no list, makes another such
table when multiplied by one, and is a weighted mix of reorderings,
each checked by the program on the table produced. Next: what 48 such
tables multiplied do, against 48 of HC's (lesson D3).

## Workarounds

The Sinkhorn-Knopp loop is written out in the program (a `p_ower`
loop of two normalizations): ask FM13 of X_eTaL-ML,
[`docs/xetal-ML-asks.md`](../../docs/xetal-ML-asks.md). The largest
singular value is power iteration written out: ask FL2 of
X_eTaL-libraries,
[`docs/xetal-libraries-asks.md`](../../docs/xetal-libraries-asks.md).
