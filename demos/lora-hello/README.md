# LoRA hello

A frozen 4 x 4 layer adapted by a rank-1 update, at the command line,
with X_eTaL-ML's LoRA library. It answers the first question of
low-rank adaptation in eight printed lines: what does a small adapter
add to a layer whose weights do not change?

The layer is `y = W x + s B (A x)`. `W` (4 x 4) is frozen. The adapter
is `A` (1 x 4) and `B` (4 x 1), the only numbers that would be
trained, and the scale `s = alpha / r`.

This is the first program of this repository: it also proves the
chain from the pinned X_eTaL-ML to a tested program and a published
page.

Live: [LoRA hello](https://softwarewrighter.github.io/X_eTaL-fine-tuning/lora-hello/)
(change the four numbers of the input or alpha; the page runs the
program again and shows the frozen output, what the adapter adds and
the adapted output, the cells that changed outlined).

## The program

```
y := (W, A, B, s) lo:f_orward x
```

`lora-hello.xtl` makes `W` by a fixed ripple, writes `A`, `B` and one
input `x` as literals, and prints:

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

## How it works

`A x` is one number, `0.5 - 1 + 3 + 1 = 3.5`: the input squeezed
through the adapter's rank, 1. `B` spreads it back over the 4 outputs,
and `s = 2` scales it: `2 * 3.5 * B = 7 0 -7 3.5`. That is all the
adapter adds; `W` is never touched.

The update `s B A` is a whole 4 x 4 matrix, but every row is a
multiple of `A`, so its rank is 1 (`lo:r_ank`, by Gram-Schmidt).
`lo:m_erge` folds it into `W`, giving one ordinary matrix that
computes the same layer: how an adapter is deployed.

The adapter trains `r (m + n) = 1 * (4 + 4) = 8` numbers where the
matrix has 16. On a layer this small that is half; the saving grows
with the layer, since the matrix's count is `m n`.

## Run it

```bash
just run lora-hello          # run the program
just show lora-hello         # as a notebook: each statement, then its output
just serve lora-hello        # its page at http://127.0.0.1:8212/
just test-demo lora-hello    # its CLI and browser baselines and the page's tests
```

## Workarounds

None.
