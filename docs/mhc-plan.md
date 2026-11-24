# mHC in X_eTaL: the plan

Manifold-Constrained Hyper-Connections (mHC; Xie and others,
DeepSeek-AI, arXiv:2512.24880, v2 of 2026-01-05) re-implemented in
X_eTaL, as lessons with visualizations and inspectable code, from the
user's Python implementation (`mHC-poc`,
https://github.com/softwarewrighter/mHC-poc, and the post
https://blog.softwarewrighter.com/2026/02/01/deepseek-papers-part1-mhc/).
Written 2026-11-11; confirmed with the user on 2026-11-15 (the
planning step, "Decided" below), when the saga started.

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

(Read at the HTML of arXiv:2512.24880v2: sections 1 to 4 on
2026-11-11, sections 5 and 6 and the appendix on 2026-11-15, each
figure below copied from the text or a caption. The paper is linked,
never copied into this repository.)

- The paper's `H_pre`, `H_post` and `H_res` are **dynamic**: each is
  a static bias plus a gated projection of the layer's own input
  (`H~ = alpha (RMSNorm(vec x) phi) + b`, eq. 7; `phi_pre` and
  `phi_post` are `nC x n`, `phi_res` is `nC x n^2` reshaped to
  `n x n`), then `H_pre = sigmoid(H~pre)`, `H_post = 2 sigmoid(H~post)`,
  `H_res = SinkhornKnopp(H~res)` with `M(0) = exp(H~res)` and `t_max
  = 20` (eq. 8, 9). The original Hyper-Connections (eq. 5) are
  dynamic too, with `tanh`. The gating factors `alpha` start at 0.01
  (table 5); the paper states no starting value for the biases. The
  Python's static logits (no dependence on `x`), its row-softmax for
  `H_pre` and `H_post`, and its start at `2 I - 1` are the Python's
  own simplifications, and the pages say so. The toy follows the
  Python first (static, the mechanism visible) and adds the dynamic
  form as its own step.
- The paper's gain metric (section 3.1, figure 3; section 5.4,
  figure 7), the **Amax Gain Magnitude**: the largest absolute row
  sum of a mapping (the forward pass) and the largest absolute column
  sum (the backward pass), averaged over the tokens of a sequence,
  for one layer's `H_res` and for the composite product of the
  `H_res` from a layer to the top. HC's composite reaches "peaks of
  3000"; mHC's "remains bounded, reaching a maximum value of
  approximately 1.6". The Python's proxy is a different number: the
  log10 of the largest absolute entry of the product. For a doubly
  stochastic product every row sum is exactly 1, so the paper's
  forward gain of mHC's `H_res` chain is 1 by arithmetic, while the
  Python's proxy tends to `1/S`; for HC at the start every row of
  `exp(2 I - 1)` sums to 3.82, so the paper's forward gain across `L`
  layers is `3.82^L` exactly (row sums multiply when every row sum
  is the same). The lessons show both numbers, each named.
- The paper's reasons for the doubly stochastic manifold (section
  4.1): the spectral norm of a doubly stochastic matrix is at most 1;
  the set is closed under multiplication; a doubly stochastic matrix
  is a convex combination of permutation matrices (the Birkhoff
  polytope). Stated, not proved, in the paper; each is checkable on a
  4 x 4 in X_eTaL. With `n = 1` the condition is the scalar 1: the
  ordinary residual.
- The experiments (section 5, table 5): mixture-of-experts models of
  3B, 9B and 27B parameters (12, 18, 30 layers; 612M, 1.66B, 4.14B
  active), a 3B trained on 1T tokens, `n = 4` streams, AdamW. The 27B
  results (table 4; baseline / HC / mHC): BBH 43.8 / 48.9 / 51.0,
  DROP 47.0 / 51.6 / 53.9, GSM8K 46.7 / 53.2 / 53.8, MMLU 59.0 / 63.0
  / 63.4, MATH 22.0 / 26.4 / 26.0 (HC ahead there); mHC's final loss
  0.021 under the baseline's; HC's loss surges near step 12k (figure
  2) with its gradient norm; mHC's time overhead 6.7% at `n = 4`. The
  appendix is one table of hyperparameters (A.1): no proofs, no
  gradient of the Sinkhorn iteration (section 4.3.1 derives a custom
  backward kernel that re-traverses the iterations; no formula), and
  the paper names no limitations beyond the iteration count being
  cut at 20 for cost. Nothing of that scale is reproduced here; a
  page quotes these as the paper's, with the link.

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
- The gain by depth, analytical: products of 12, 24, 48 copies of
  HC's and mHC's `H_res` at initialization, and at the logits a run
  arrives at; the Python's proxy (largest entry) and the paper's Amax
  (largest row sum, largest column sum) of the same products, each
  named. A line chart, log scale, the Python's `gain_comparison`
  figure and the paper's figure 3 and 7 quantity, from matrices
  alone.
- The forward signal through depth without training: random small
  blocks, the size of the stream state after each of 48 layers under
  the three schemes (measured on the toy; the paper's figure 3 shows
  its gain at 27B). Measured on 2026-11-15 in the timing program
  below: from the start `2 I - 1`, HC's stream state after 48 layers
  has a largest entry of 9.0 x 10^27 (the analytical 10^27.35 above,
  the block's small contributions aside); mHC's, 2.29.
- Training the three schemes at depths 12, 24, 48 on a tiny task with
  hand-written gradients (X_eTaL has no automatic differentiation):
  the loss, the gradient norm and the gain per step, the Python's
  three plots. The block is a tiny MLP (not attention): the
  stability question is about the residual mixing, not the block, the
  paper's figure 3 argument does not depend on the block, and the
  Python's task does not need attention (below).

**Timed (2026-11-15, natively, `bin/xetal` at the pin, Apple
silicon).** `S = 4` streams, width `D = 8`, a block `8 -> 16 -> 8`
with GELU (NN's `d_ense`, Activations' `g_elu` and `d_gelu`), a batch
of 16, static `H`, the Sinkhorn projection of `exp(2 I - 1)` at 20
iterations, and a layer's forward and backward written out (the
block's weights and the stream state; `H` held fixed): 2.4 ms per
layer per training step, so 0.03 s, 0.06 s and 0.12 s a step at 12, 24
and 48 layers; the forward alone through 48 layers, 2 ms. The Python's
800 steps at 48 layers are then about 90 s natively for one scheme,
and the three schemes at the three depths together about 8 minutes;
the pages run X_eTaL as WebAssembly, measured on the first page
(2026-11-17, step 2): its program takes 0.60 s at the command line and
0.66 s in headless Chrome on the same machine, a factor of 1.1, so the
native timings above hold in the browser to within about a tenth. Too
long for a page to compute at load either way. So: the training experiment (step 5)
is **trained offline by X_eTaL** (`just train`, a `train/` program as
the layout allows, the record committed under `demos/<slug>/data/`
with the seed, the steps and the X_eTaL commit that made it) and
**replayed** on the page, which also runs a short live training (12
layers, about 100 steps, a few seconds) so a reader sees the same
program compute in front of them and the recorded run begin the same
way. Every number is still X_eTaL's; the page says which run each
chart is from. The lessons without training (steps 2 to 4) compute
everything at load.

Omitted, and said so on every page: attention and the language task
(the toy's task is the Python's incrementing tokens, below, on a
vocabulary of 16); the paper's scale and its MoE models; the dynamic
`H` (until its own step); memory-access cost and the infrastructure
(DualPipe, recomputation), which are the paper's section 4.3 and not
arithmetic; the paper's gain measured over the tokens of a real
sequence (the toy has no sequence: its Amax is over its batch).

## Measured in step 5 (2026-11-22)

The toy as built: lesson D1's layer (a bare MLP block; H_post a
softmax of four logits; the streams averaged, then scaled to size 1
(RMS) before the unembedding, in place of the Python's final
LayerNorm), every weight trained with Adam at rate 0.01, 800 steps,
float64. The cost: about 7 ms per layer per training step natively,
not the planned 2.4 (the Sinkhorn rounds recomputed and
differentiated in every layer), so 278 s a run at 48 layers and 8.6
minutes for the nine on three cores; the page's run of 50 steps at
12 layers takes about 6 s in Chrome. Final losses (guessing evenly,
2.77): 12 layers 0.0028, 0.0029, 0.0029; 24 layers 0.0030, 0.0029,
0.0027; 48 layers 2.7726, 0.0027, 0.0027 (plain skip, HC, mHC). The
plain skip at 48 layers has its gradient swing in the first two
hundred steps and then fall to 3e-7. No NaN event in any run. HC's
composite gain stays near the analytical 10^27.5 through training and
does not stop it learning, unlike the Python's run (5.54 at 48
layers); not explained here (candidates named on the page: 32-bit
numbers there, the blocks' own skips and LayerNorms, the task).

## Read again at source in step 6 (2026-11-23)

H_pre and H_post are rows of n (1 x n), H_res is n x n (eq. 3: the
block's input H_pre x_l, the new state H_res x_l + H_post' F). Eq. 7:
the stream state flattened to 1 x nC, RMSNorm, then alpha (x' phi) + b
for each, phi_pre and phi_post nC x n, phi_res nC x n^2 reshaped by
mat(); the fused form (eq. 14 to 16) is one phi of nC x (n^2 + 2n),
the division by the norm moved after the product. Eq. 8: sigmoid,
2 sigmoid, Sinkhorn-Knopp. Eq. 9: M(0) = exp, T_r(T_c(...)), t_max 20.
Not stated: the starting biases, why 2, which alpha table 5's 0.01 is
(lesson D5 uses it for all three), the order of mat(), a learnable
RMSNorm gain (said to be absorbed into phi). Eq. 15 has no epsilon
though table 5 lists 1e-20. Eq. 5 (the original HC) has tanh in place
of the sigmoids and no Sinkhorn.

## Decided (with the user, 2026-11-15)

1. **Placement**: the mHC saga runs now, before Saga 2 (the user,
   2026-11-14); Saga 1 is archived.
2. **The block**: a tiny MLP, `8 -> 16 -> 8` with GELU. The Python's
   task, next token `= (token + 1) mod V`, depends on the current
   token alone, so an MLP block learns it as attention would; the
   stability question is about the residual mixing, not the block.
3. **The task**: the Python's incrementing tokens, made tiny: a
   vocabulary of 16, a one-hot token in, an embedding `16 x 8`, the
   layers, an unembedding `8 x 16`, softmax and cross-entropy (NN's
   `o_neHot`, `s_oftmax`, `c_rossEntropy`); a batch of 16 covers the
   vocabulary. The Python's losses are then on the same task, at
   another width and depth.
4. **Static and dynamic `H`**: both; the Python's static logits
   first (steps 2 to 5, the mechanism visible), the paper's dynamic
   `H` in step 6.
5. **The Python's figures**: quoted beside the toy's and the paper's,
   each marked "the Python's run" (the user's own, `mHC-poc`), with
   the link.

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

Confirmed on 2026-11-15 with these changes from the planning step:
step 4 shows the paper's Amax gain beside the Python's proxy; step 5
trains offline and replays, with a short live run; step 2 measures the
WebAssembly slowdown on its page; every demo is `track = depth`, with
`lesson` numbers D1 to D6 in the curriculum's depth track. The first
page waits on nothing: FM13's workaround is the expressions written
out, and the timing program above is the shape of them.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | mhc-plan | DONE (2026-11-15): the paper's sections 5, 6 and appendix read at source; the toy timed (2.4 ms per layer per training step natively); the five questions decided with the user; ask FM13 written with its signatures; this table confirmed; Saga 1 archived, this saga started |
| 2 | residual-streams | DONE (2026-11-17), live: lesson D1, "why a skip connection, and what if there were four?" The ordinary residual on a tiny block; the signal through 12, 24, 48 random layers with and without it (measured, forward only); then the stream state of HC as a 4 x 8 table, `H_pre`, the merge, the block, `H_post'`, `H_res` stepped through as a Flow pipeline with every number shown; the paper's identity-mapping property named: what a product of `H_res` across depth must keep. Measures the page's X_eTaL time against the command line's (the WebAssembly factor), for step 5's budget |
| 3 | sinkhorn | DONE (2026-11-17), live: lesson D2: "a matrix whose rows and columns each sum to 1". A positive 4 x 4; one row normalization, one column normalization, shown; the iteration stepped through, the row and column sums converging (measured, with the count of iterations to a tolerance); the three properties checked in the program (largest singular value at most 1, FL2 or written out; closure; the convex combination of permutations, as an aside); the Python's `sinkhorn_doubly_stochastic` (`mlx/src/mhc.py`; the plan first named it `make_doubly_stochastic`) beside the X_eTaL `p_ower` loop, line for line. Measured: one round from `exp(2 I - 1)`, 11 rounds to a millionth from a lopsided start, more than 20 from a nearly split one (2.5e-4 off at round 20) |
| 4 | gain-by-depth | DONE (2026-11-20), live: lesson D3: the gain by depth, analytical. `exp(2 I - 1)` and its Sinkhorn projection; products across 12, 24, 48 layers; two measures of each product, named: the paper's Amax (largest row sum: the `3.82^L` line for HC, exactly 1 for mHC) and the Python's proxy (largest entry: 10^6.39, 10^13.37, 10^27.35 against 10^-0.602, the limit 1/S); the Python's table (7.0, 14.4, 27.3 against -0.5, -0.6, -0.6) set beside, the differences being what training moved; a slider on the starting logits (how far from the identity before HC's gain is unbounded: at every setting, by the arithmetic). Built with the slider as a shift c of every logit: HC's row sum is 3.82 e^c, so its gain grows above c = -1.34 and fades below it, while mHC's table is the same at every c; moving the diagonal or the off-diagonal alone keeps HC's row sum above 1 at every setting |
| 5 | training-at-depth | DONE (2026-11-22), live: experiment D4: the three schemes trained at depths 12, 24, 48 on the toy task (incrementing tokens, vocabulary 16, MLP block) with Adam, from the same seed (ask FM13 or its workaround): trained offline by X_eTaL (`just train`, the record committed as data with its seed, steps and X_eTaL commit) and replayed; a short live run (12 layers, about 100 steps) computed on the page; the loss, the gradient norm, both gains per step as X_eTaL's charts; the final losses in a table beside the Python's, each marked measured here or the Python's; NaN events counted; what the toy reproduces of the Python's result and what it cannot (the task, the scale) |
| 6 | dynamic-mixing | DONE (2026-11-23), live, without the training rerun (split off below, BLOCKED on FM13): lesson D5: the paper's own `H`: a gated projection of the layer's input plus a static bias (`alpha` from 0.01), sigmoid for `H_pre`, `2 sigmoid` for `H_post`, Sinkhorn of `exp` for `H_res`; the gradient through the projection; the toy of step 5 rerun with it (offline, replayed); what changes (measured) |
| 6a | dynamic-training | BLOCKED on FM13 (reserved in the saga, 2026-11-23): the toy of step 5 rerun with the dynamic H, offline and replayed. Split from step 6 because the batched backward pass (a table per token per layer, the normalization and projection added to step 5's ~40 lines) and the cost (several times step 5's 8.6 minutes) are past what reads as a page's program (rule 3) |
| 7 | mhc-compared | summary D6: baseline, HC, mHC at equal depth, equal width and equal steps, with the paper's and the Python's reported figures quoted as theirs beside the toy's; what the toy showed (the bound is arithmetic; training at depth follows it on the toy task), what it did not (attention, a sequence, scale, memory cost); the X_eTaL program beside the Python, function for function |

Asks: FM13 (X_eTaL-ML, `docs/xetal-ML-asks.md`, signatures written
2026-11-15): stream mixing, Sinkhorn-Knopp with its gradient, the HC
and mHC layer step with gradients checked by central differences, both
gain measures, the dynamic `H`; FL2 (X_eTaL-libraries, open): singular
values, for the spectral-norm check.

## The questions this plan had

Five, put to the user on 2026-11-11 and answered on 2026-11-14 and
2026-11-15: "Decided" above. None is open.
