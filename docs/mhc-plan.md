# mHC in X_eTaL: the plan

Manifold-Constrained Hyper-Connections (mHC; Xie and others,
DeepSeek-AI, arXiv:2512.24880, v2 of 2026-01-05) re-implemented in
X_eTaL, as lessons with visualizations and inspectable code, from the
user's Python implementation (`mHC-poc`,
https://github.com/softwarewrighter/mHC-poc, and the post
https://blog.softwarewrighter.com/2026/02/01/deepseek-papers-part1-mhc/).
Written 2026-11-11; this is the plan, to be confirmed with the user
before the saga starts (its first step).

## What the Python did (read at source, 2026-11-11)

`mHC-poc` (MLX and PyTorch, ~2.5 KLOC) trains a tiny decoder-only
transformer on a synthetic task and compares three residual schemes at
three depths.

- The task: sequences of tokens `(start + i) mod 256`, length 128;
  predict the next token. Learnable in a few hundred steps.
- The model: `d_model` 256, 4 heads, `d_ff` 1024, 12, 24 or 48 blocks
  (pre-norm attention and GELU MLP, each with its own ordinary
  residual inside). Adam, 800 steps, batch 16 (`lr` 2e-3 in the
  configs; its results page says 1e-4).
- The three variants, per block, with `S = 4` streams and the stream
  state `x` of shape `[B, T, S, D]` (the base hidden state repeated S
  times at the start, the mean over streams at the end):
  - baseline: one stream, `x = x + block(x)`;
  - HC: `x_pre = H_pre x` (rows mixed over streams), `merged = mean
    over streams of x_pre`, `f = block(merged)`, `f_mixed = H_post'
    (f repeated over streams)`, `x = H_res x + f_mixed`. `H_pre` and
    `H_post` are row-softmax of logits (nonnegative, rows summing to
    1); `H_res = exp(logits)`, positive and unconstrained;
  - mHC: the same with `H_res = SinkhornKnopp(exp(logits))`, 20
    iterations of row normalization then column normalization,
    giving a matrix whose rows and columns each sum to 1 (doubly
    stochastic, to the tolerance the iterations reach).
  - Every logit table starts at `2 I - 1` (the diagonal at 1, the rest
    at -1): near the identity after `exp`.
- The metrics recorded every step: the loss, the global gradient norm,
  NaN or Inf events, and a **gain proxy**: `log10` of the largest
  absolute entry of the product `H_res[L] ... H_res[2] H_res[1]`
  across the depth. The code says this is "NOT the paper's exact
  metric".
- Its results (`docs/results.md` of the repo): the gain proxy at 12,
  24, 48 layers is 7.0, 14.4, 27.3 for HC and -0.5, -0.6, -0.6 for
  mHC (0.0 for the baseline, which has no H_res); the final loss at 48
  layers is 3.79 (baseline), 5.54 (HC), 0.0002 (mHC); zero NaN events
  in every run; gradient norms erratic for HC at 48 layers, smooth for
  mHC.

One thing the toy can say exactly that the Python only measured
(checked in X_eTaL on 2026-11-11, twelve lines, before this plan was
written): at the initial logits `2 I - 1`, HC's `H_res = exp(2 I - 1)`
has `e` on the diagonal and `1/e` off it, each row summing to 3.82;
the largest entry of its product across 12, 24 and 48 layers is
10^6.39, 10^13.37 and 10^27.35. The Python measured 7.0, 14.4 and 27.3
after 800 steps of training: the logits barely moved. The Sinkhorn
projection of the same matrix (20 iterations, 0.711 on the diagonal
and 0.096 off it, rows and columns summing to 1.0 to the last digit)
has a product across 48 layers whose largest entry is 10^-0.602, and
-0.602 is log10 of 1/4: products of doubly stochastic matrices
converge to the matrix of all 1/S, the streams averaged. The Python's
CUDA run reports -0.602 to the digit. **The headline result is linear
algebra at initialization; training confirms it.** That is the first
lesson, and it needs no training at all.

## What the paper does that the Python did not

(Read at the HTML of arXiv:2512.24880 on 2026-11-11, sections 1 to 4
verbatim; section 5 and the appendix through a summarizer, not to be
quoted until read.)

- The paper's `H_pre`, `H_post` and `H_res` are **dynamic**: each is
  a static bias plus a gated projection of the layer's own input
  (`H~ = alpha (RMSNorm(vec x) phi) + b`, eq. 7), then `H_pre =
  sigmoid(H~pre)`, `H_post = 2 sigmoid(H~post)`, `H_res =
  SinkhornKnopp(H~res)` with `M(0) = exp(H~res)` and `t_max = 20`
  (eq. 8, 9). The Python uses static logits only (no dependence on
  x) and row-softmax where the paper uses a sigmoid. The toy follows
  the Python first (static, the mechanism visible) and adds the
  dynamic form as a later step.
- The paper's reasons for the doubly stochastic manifold (section
  4.1): the spectral norm of a doubly stochastic matrix is at most 1;
  the set is closed under multiplication; a doubly stochastic matrix
  is a convex combination of permutation matrices (Birkhoff). Each
  is checkable on a 4 x 4 in X_eTaL.
- The paper's experiments are 3B to 27B mixture-of-experts models;
  its "Amax Gain Magnitude" of HC's composite mapping peaks near 3000
  and mHC's stays near 1.6 (the second figure is from the summarizer).
  Nothing of that scale is reproduced here; the page quotes it as the
  paper's.

## What fits at toy scale, and what the toy omits

Fits, exactly:

- The stream mixing arithmetic on `S = 4` streams of width `D = 8`:
  `x` as a 4 x 8 table, `H x` as a mix of its rows, the mean over
  streams, the repeat, `H_post'`. Every number visible.
- Sinkhorn-Knopp on a 4 x 4 positive matrix: each iteration's row sums
  and column sums shown converging to 1; how many iterations it takes
  from `exp(2 I - 1)` (and from a worse start); the three properties
  of the result checked (largest singular value at most 1, FL2;
  closure under products; a decomposition into permutation matrices,
  as an aside).
- The gain proxy by depth, analytical: products of 12, 24, 48 copies
  of HC's and mHC's `H_res` at initialization, and at the logits a
  run arrives at. A line chart, log scale, exactly the Python's
  `gain_comparison` figure, from matrices alone.
- The forward signal through depth without training: random small
  blocks, the size of the stream state after each of 48 layers under
  the three schemes (measured on the toy; the paper's figure 2 shows
  the same quantity at 27B).
- Training the three schemes at depths 12, 24, 48 on a tiny task with
  hand-written gradients (X_eTaL has no automatic differentiation):
  the loss, the gradient norm and the gain proxy per step, the
  Python's three plots. The block is a tiny MLP (not attention): the
  stability question is about the residual mixing, not the block, and
  the paper's figure 2 argument does not depend on the block. Cost:
  48 blocks of `D = 8` on a batch of a few examples is thousands of
  small matrix products a step; X_eTaL on the lessons so far does a
  comparable amount in under a second. To be measured in the planning
  step before the step count is set.

Omitted, and said so on every page: attention and the language task
(the toy predicts a next number from a tiny sequence or regresses a
function); the paper's scale and its MoE models; the dynamic `H`
(until its own step); memory-access cost and the infrastructure
(DualPipe, recomputation), which are the paper's section 4.3 and not
arithmetic; the paper's exact gain metric (the Python's proxy is used,
named as a proxy, and the paper's definition read before any page
calls anything the paper's).

## The rules this plan keeps

- F3: this repository defines no function. The stream mixing, the
  Sinkhorn-Knopp projection with its gradient, the HC and mHC layer
  steps with gradients checked by central differences, and the gain
  proxy are **asked of X_eTaL-ML** (ask FM13, a library `HC` or a
  part of `Net`), as the LoRA mathematics was. Until it lands, each
  expression is written out in the program where it is used (the
  Sinkhorn iteration is a `p_ower` loop of two normalizations; the
  mixes are matrix products); if the hand-written gradients make the
  program unreadable, the training step waits on the ask and the
  lessons without training ship first.
- Every number a page shows is X_eTaL's; Rust lays out (the stream
  state as a heat map per layer, the gain chart, the Sinkhorn
  iteration stepped through).
- New eyes: the pages open from "why do deep networks need a skip
  connection?" and build residual, streams, mixing matrix, doubly
  stochastic, Sinkhorn-Knopp, gain, one plain explanation each.
- Measured or analytical, every figure marked; the Python's and the
  paper's numbers quoted only as theirs.

## Where it sits

This repository teaches how a model is changed; mHC is about how a
deep model is trained stably, which is a step before any of it. It
belongs here as a side track, `depth` (the user, 2026-11-11: "I want
to revisit this but with visualizations and inspectable code"), with
the mathematics in X_eTaL-ML as every other method's is. Its place in
the road (before or after Saga 2) is the user's call; nothing in it
depends on Sagas 2 to 5, and nothing in them depends on it. The
pages' shell (`shared/lesson`) is ready for it.

## The saga: mhc

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | mhc-plan | a planning step, with the user: the paper's sections 5 and 6 and appendix read at source (what was summarized above re-read, every figure a page will quote verified); `mHC-poc`'s configs and results re-run or read once more; the toy's block, task, widths and depths chosen and timed in X_eTaL (can 48 layers train in a page?); ask FM13 written with the signatures wanted; this table confirmed or replaced |
| 2 | residual-streams | lesson: "why a skip connection, and what if there were four?" The ordinary residual on a tiny block; the signal through 12, 24, 48 random layers with and without it (measured, forward only); then the stream state of HC as a 4 x 8 table, `H_pre`, the merge, the block, `H_post'`, `H_res` stepped through as a Flow pipeline with every number shown; the paper's identity-mapping property named: what a product of `H_res` across depth must keep |
| 3 | sinkhorn | lesson: "a matrix whose rows and columns each sum to 1". A positive 4 x 4; one row normalization, one column normalization, shown; the iteration stepped through, the row and column sums converging (measured, with the count of iterations to a tolerance); the three properties checked in the program (largest singular value at most 1, FL2; closure; the convex combination of permutations, as an aside); the Python's `make_doubly_stochastic` beside the X_eTaL `p_ower` loop, line for line |
| 4 | gain-by-depth | lesson: the gain proxy, analytical. `exp(2 I - 1)` and its Sinkhorn projection; products across 12, 24, 48 layers; the `3.82^L` line and the bound at 1 (and the limit at 1/S); the Python's table (7.0, 14.4, 27.3 against -0.5, -0.6, -0.6) set beside the toy's analytical 6.39, 13.37, 27.35 against -0.602 at every depth (the limit log10(1/S), the streams averaged), the differences being what training moved; a slider on the starting logits (how far from the identity before HC's gain is unbounded: at every setting, by the arithmetic) |
| 5 | training-at-depth | experiment: the three schemes trained at depths 12, 24, 48 on the toy task with Adam, from the same seed (ask FM13 or its workaround): the loss, the gradient norm, the gain proxy per step as X_eTaL's charts; the final losses in a table beside the Python's, each marked measured here or the Python's; NaN events counted; what the toy reproduces of the Python's result and what it cannot (the task, the scale) |
| 6 | dynamic-mixing | the paper's own `H`: a gated projection of the layer's input plus a static bias, sigmoid for `H_pre`, `2 sigmoid` for `H_post`, Sinkhorn of `exp` for `H_res`; the gradient through the projection; the toy of step 5 rerun with it; what changes (measured) |
| 7 | mhc-compared | the summary page: baseline, HC, mHC at equal depth, equal width and equal steps, with the paper's and the Python's reported figures quoted as theirs beside the toy's; what the toy showed (the bound is arithmetic; training at depth follows it on the toy task), what it did not (attention, scale, memory cost); the X_eTaL program beside the Python, function for function |

Asks: FM13 (X_eTaL-ML): stream mixing, Sinkhorn-Knopp with its
gradient, the HC and mHC layer step with gradients checked by central
differences, the gain proxy; FL2 (X_eTaL-libraries, open): singular
values, for the spectral-norm check.

## Open questions for the user

1. Placement: before Saga 2 (the variations), or after it?
2. The block: a tiny MLP (fast, the mechanism unchanged) or a tiny
   attention block from X_eTaL-ML's Attention library (nearer the
   Python, slower, more gradient code)?
3. The task: the Python's incrementing tokens on a tiny vocabulary
   (next-token, cross-entropy) or a regression (simpler loss)? The
   planning step times both.
4. Whether the Python's static `H` (first) and the paper's dynamic
   `H` (later) are both wanted, or only the Python's.
5. Whether the lessons may quote the Python's figures as a comparison
   (they are the user's own runs) or only the paper's.
