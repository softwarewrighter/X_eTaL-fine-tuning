# Asks for X_eTaL-ML

What the lessons and experiments here need from
[X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML), pinned by
`XETAL_ML_COMMIT` (`../X_eTaL-ML`, read only from here: its own plan
takes them). This repository does not change X_eTaL-ML: each need is
filed here, the workaround named below is used and shown on the page
meanwhile, and it is removed when the ask lands in the commit pinned.

General needs (mathematics, notation, plotting) are in
[`xetal-libraries-asks.md`](xetal-libraries-asks.md). A need of the
language itself would go in `docs/xetal-asks.md`, which does not exist
because there is none: the order of preference is a library here, an
ask here, an ask to X_eTaL-libraries (a function, or an `.xtlm` macro
where a function cannot do it), and a language change last
(`docs/plan.md`, F3).

Each entry: status, kind (feature, bug, speed), the ask, which demos
or libraries here need it, and the workaround in use. Status: **not
yet taken** (filed here, to be taken to X_eTaL-ML's plan), **planned**
or **in progress** (in its plan), **landed** (in the commit pinned
here, the workaround removed), **not planned** (X_eTaL-ML has read it
and will judge it later: when is said below), **declined** (with its
reason), **withdrawn** (no longer needed here),
**blocker** (no workaround: the step waits, and the user is told).

This repository defines no functions of its own (`docs/plan.md`, F3):
what a lesson needs and X_eTaL-ML lacks is asked for here, with the
signature wanted. FM1, FM2 and FM5 are about the pages' presentation
and have workarounds here. FM3 and, for Saga 1, FM7 are written out in the
programs that need them. FM4 and FM6 are whole methods a lesson
cannot reasonably write out: those lessons wait. FM8, FM9 and FM10 are
judged in their sagas' planning steps. No step of Saga 1 is blocked. They were found by reading X_eTaL-ML at 6bac452 (2026-11-05); FM1 and
FM2 are confirmed by `shared/lesson`'s tests.

| # | Status | Kind | Ask | Demos, libraries | Workaround |
| - | ------ | ---- | --- | ---------------- | ---------- |
| FM1 | landed | feature | `shared/microscope` embeds the libraries of its own repository only: its `build.rs` reads `../../libs` and `../../work/libs` beside its manifest, so a page of another repository gets X_eTaL-ML's libraries and the seven of X_eTaL-libraries it links (Check, Plot, Strings, Format, Lists, Equations, Parsing), but not that repository's own (`FineTune`, `PEFT`, `Edit`) nor another general one (Matrix). Asked: more library roots named by the dependent (an environment variable read by `build.rs`, a list of directories), or `microscope::libs::add` documented as the way to add a library's source | every page here that imports a library of this repository; confirmed in step foundation (2026-11-05): `"mx:" u_se< "Matrix"` is found only once added | none left (removed 2026-11-05): `.cargo/config.toml` names `work/xetal-libraries/libs` in `MICROSCOPE_LIBS`, and microscope embeds it; `shared/lesson` no longer embeds or adds any library |
| FM2 | landed | feature | `microscope::chrome::footer` names X_eTaL-ML: `REPO` is a constant (`https://github.com/softwarewrighter/X_eTaL-ML`), and `build.rs` takes the build's commit with `git rev-parse` where the crate lies, which from here is the X_eTaL-ML clone's commit, not this repository's. Asked: the footer (and the header's catalog link) taking the repository's name, URL and commit from the page | every page here; confirmed in step foundation (2026-11-05): microscope's `BUILD_SHA` is taken where its crate lies, the clone | the header is microscope's `header_of` with this repository's `Site` (`lesson::chrome::SITE`). The footer stays this repository's own, by choice and not as a workaround: `footer_of` words the build as `build (host .., sha .., ..)`, and the user asked here for the compact form without labels |
| FM3 | landed | feature | LoRA has one scale, `alpha lo:s_caling r` (alpha / r). Asked: rsLoRA's rank-stabilized scale alpha / sqrt r beside it (a function of the same shape, `lo:s_calingRs`), since an adapter is `(W, A, B, s)` and nothing else changes | rank-and-scaling (Saga 1) | none left: `alpha lo:s_calingRs r` (the lesson that needs it, rank-and-scaling, is not built yet and will start from it) |
| FM4 | planned | feature | QLoRA (the NF4 codebook, double quantization, an adapter on a quantized base) and VeRA are X_eTaL-ML's Saga 6 steps quant-nf4, qlora and vera, not built at 6bac452. DoRA (magnitude and direction) is in no plan of X_eTaL-ML. It was first asked "after this repository has built and checked it"; since this repository defines no functions (2026-11-05), DoRA is asked for outright: `pe`-style functions in LoRA or beside it, the weight as a magnitude per column times a direction, the adapter on the direction, gradients checked. The LoRA variations are this repository's next saga (the user's order, 2026-11-05), so these are now on its path | qlora, dora, vera, variants-compared (Saga 2) | at Saga 2's start the pin is moved and what has landed is used; a lesson whose mathematics has not landed waits (this repository defines no functions): a blocker for that lesson, not for the others |
| FM5 | landed | feature | The matrix heat map (diverging colors about 0, a shared scale across maps side by side, each cell's value) is private to `demos/lora-explorer/web/src/app.rs`, so a second page cannot use it. Offered rather than asked: `shared/lesson`'s heat map (the same colors through `microscope::color`, with a legend, a cell inspector and keyboard focus) can move into `microscope` | every page here | none left: `microscope::heat` (`Matrix`, `heat`, `largest`, `column`, `fill`). lesson-shell builds its legend, cell inspector and keyboard focus on it, to be offered back as a patch or a further ask, as X_eTaL-ML invites |
| FM6 | planned | feature | A mixture-of-experts library (scores, top-k, dispatch, an adapter per expert, routing statistics) and a small trained base model are X_eTaL-ML's Saga 6 steps moe-lib, moe-base and moe-finetune, not built at 6bac452. Added 2026-11-17: the router's logits `z = G(h)` readable by a caller (FM15 routes on them) | adapters-in-moe (Saga 2 step 8), moe-lora-routing (step 9) | the steps wait for them; the lessons are the last of their saga so nothing else waits on them |
| FM7 | not planned | feature | The learning-from-examples task and its measures, as a library (FineTune, alias `ft:`, or in Learn): `seed ft:t_ask (n, N, noise)` giving `(x, y)` (N inputs of n numbers as columns, targets from a hidden shifted layer W + T); `seed ft:s_plit (x, y)` giving `((x_train, y_train), (x_held, y_held))`; `adapter ft:v_alidation (x_held, y_held)` (through `lo:l_oss`); `adapter ft:r_etention (x_old, y_old)` (the old task's loss after learning the new one); a run's record (the losses every k steps and the states kept, as a `p_ower` step so a page can step back); a comparison harness (a method as `(start, step, counts)`, every method run on the same task, split and seeds); and full and selective fine-tuning of one layer (`ft:g_rad`, an SGD and an Adam step through Optim, a mask of the rows trained), each gradient checked by central differences | lora-training, rank-and-scaling, lora-vs-fa (Saga 1); variant-harness and every comparison (Saga 2); backprop-baseline (Saga 4) | written out in each program, as X_eTaL-ML's lora-explorer writes its training (the task's inputs and targets as plain expressions, the split by taking columns with `t_ake_2`, the loop as unnamed lambdas given to `p_ower`, `lo:l_oss` for every loss, the record as the loop's state; done so in lora-training, 2026-11-07: 20 lines of a 110-line program; in lora-vs-fa, 2026-11-10, full fine-tuning of W is 5 lines, Adam through `op:a_dam` on the gradient `g x'`, and the retention measure is one more loss on old examples, `lo:l_oss` with W's own outputs as the targets). That serves Saga 1; the comparison harness of Saga 2 is where writing it out stops being readable, and is judged then |
| FM8 | not planned | feature | Learning without back-propagation, as a library (WeightSpace, alias `ws:`; `docs/research3.txt`): a perturbation regenerated from its seed (`seed ws:n_oise shape`); the two-point estimate and step (MeZO: `ws:t_woPoint`, `ws:s_tep`); a population with antithetic pairs and centered ranks (evolution strategies: `ws:e_s`); difficulty-aware reweighting and replay (DART-ES); perturbations inside a layer's input span or a structured, conditioned subspace (AGZO, StructZO); low-rank perturbations (EGGROLL); each returning the forward passes it used and the numbers it held, so a lesson can compare them with back-propagation. To be specified in Saga 4's planning step, each from its paper | forward-only, reward-not-loss, better-directions, selection-noise, weight-space-compared (Saga 4) | the first lesson (a two-point estimate, a small population) can be written out in its program; the saga as a whole repeats these in every lesson and is to wait for the library unless its planning step decides otherwise |
| FM9 | not planned | feature | Closed-form editing, as a library (Edit, alias `ed:`): a layer as a linear associative memory (keys to values); the rank-one edit with a key covariance (ROME), through X_eTaL-libraries' Matrix solve; many edits by least squares (MEMIT); the update projected off preserved keys (AlphaEdit); locality and retention measures. To be specified in Saga 5's planning step, each from its paper | editing-a-fact, many-edits, gated-edits (Saga 5; direction-removal does not need it) | a rank-one edit is a few lines with Matrix's solve and can be written out; batch and projected edits are to wait for the library unless the planning step decides otherwise |
| FM10 | not planned | feature | Shared-basis adapters (`docs/research4.txt`), in LoRA or a library of its own: an adapter in a fixed shared basis, `(W, U, C, V) f_orward x` giving `W x + U (C (V' x))` with C a full r x r table or a diagonal (r numbers); its gradient for C and SGD and Adam steps through Optim; bases from a seed (as `lo:s_eeded`), from W's leading singular directions (LoRA-XS; needs FL2), and chosen from the data (Fisher-guided); a family of tasks from seeds (related ones sharing directions, unrelated ones, taken in sequence) with what transfers and what is forgotten measured (the loss of every earlier task after each new one). Later, from their papers: a basis that evolves as tasks arrive (Share), shared with task-specific directions (LoDA), a subspace per task (SuLoRA), dominant pretrained directions protected (OPLoRA). To be specified in Saga 3's planning step | subspace-intro (no training: needs FL3 only), shared-vs-separate, subspace-selection, continual-subspaces (Saga 3); better-directions' shared reduced space (Saga 4) | shared-vs-separate with a fixed basis is short enough to write out (the forward pass is one expression, the gradient of C is `U' g (V' x)'`); the continual methods are to wait for the library unless the planning step decides otherwise |
| FM11 | not yet taken | feature | Flow's mathematics with its symbols marked: `flow:m_ath<` draws a pipeline's stages through Equations' `e:t_ex<`; X_eTaL-libraries is adding `e:t_exSymbols<` (its Saga 26, this repository's ask FL1), the same LaTeX with each array reference and named scalar wrapped as `\htmlClass{sym sym-NAME}{...}`. Asked: a variant of `flow:m_ath<` (a macro beside it, as Equations did: `flow:m_athSymbols<`) that draws through it, each stage's row still wrapped as microscope's `stages` does, so a page can select a symbol and its stage | lora-lab (Saga 1: the equation's symbols selectable), every lesson that draws a pipeline | the page wraps the names it knows in the LaTeX by text (FL1's own workaround), with the test that every glossary symbol was found |
| FM13 | not yet taken | feature | Hyper-Connections and mHC (Xie and others, arXiv:2512.24880; `docs/mhc-plan.md`), a library `HC` (alias `hc:`) or a part of Net; signatures below, written 2026-11-15 after the toy was timed, to be spelled as X_eTaL-ML prefers. A stream state is an `S x N` matrix, a stream a row (`N` the width `D`, or `B * D` for a batch flattened); a mixing table `H` is `S x S`. **The projection**: `n hc:s_inkhorn M`, `n` iterations of row normalization then column normalization of a positive `M` (the paper's eq. 9, the Python's `make_doubly_stochastic`), and its gradient `(n, M) hc:s_inkhornBack G` (the gradient of a loss with respect to `M`, given `G` its gradient with respect to the result; the pass back through the `n` iterations), **checked against central differences**. **The layer** (static `H`, the Python's): `'f (Hpre, Hpost, Hres) hc:l_ayer X` gives the next stream state `Hres X + Hpost' f(merged)` with `merged = mean over streams of Hpre X` (a `B x D` matrix, examples by rows, for a block `f` from `B x D` to `B x D`), and also returns `merged` and `f merged` for the backward; its backward in two halves around the block's own: `(Hpost, Hres) hc:b_ackTop (X, F, G)` gives `(dXres, dF, dHres, dHpost)` from `G` the gradient at the output (`dXres = Hres' G`, `dF = Hpost . G` by rows, `dHres = G X'`, `dHpost = G F'` summed as the shapes need), the caller carries `dF` through the block, and `Hpre hc:b_ackBottom (X, dMerged)` gives `(dXpre, dHpre)` (`dP` = `dMerged` repeated over the streams over `S`, `dXpre = Hpre' dP`, `dHpre = dP X'`); every gradient checked against central differences on a small `S`, `D`, `B`. **The start**: `S hc:s_tart` the Python's `2 I - 1`; HC's `H_res` is `e_xp` of it and mHC's is `20 hc:s_inkhorn e_xp` of it (`H_pre`, `H_post` by row softmax, NN's). **The gains** of a chain of mixing tables `Hs` (boxed, first layer first): `hc:g_ainProxy Hs`, log10 of the largest absolute entry of the product (the Python's proxy, named as a proxy), and `hc:a_maxGain Hs`, `(forward, backward)` = the largest absolute row sum and the largest absolute column sum of the product (the paper's Amax Gain Magnitude, section 3.1). **Later** (the saga's step 6, the paper's eq. 7 and 8): `(alpha, phi, b) hc:d_ynamic X` gives `(Hpre, Hpost, Hres)` from the stream state flattened and RMS-normalized (Norm's), `Hpre = sigmoid`, `Hpost = 2 sigmoid`, `Hres = 20 hc:s_inkhorn e_xp` of the three affine maps, and its backward to `alpha`, `phi`, `b` and `X`, checked. Optim's `a_dam` and `s_gd` serve the updates; nothing is asked there | the mhc saga (`docs/mhc-plan.md`): step 2 (the forward only), step 3 (`s_inkhorn`), step 4 (the gains), step 5 (the backward: here the ask matters), step 6 (`d_ynamic`) | the expressions written out in the program (the Sinkhorn loop as `p_ower` over two normalizations, about 3 lines; the forward as matrix products, 3 lines; the backward of one layer, about 12 lines, as the planning step's timing program had it); the training step (step 5) is written out if that stays readable in one program, else it waits on this ask and is marked blocked. In use: residual-streams (2026-11-17: the forward, the Sinkhorn loop in one line) and sinkhorn (2026-11-17: the loop with its record of every half round, 4 lines, and once more without the record for a second table) |
| FM14 | not yet taken | feature | A tiny transformer a page can run: X_eTaL-ML's micro-gpt as an importable library (its model as data, a forward pass over a short sequence, its loss; later a training step), so a lesson on Dust (`docs/research3.txt`: perturbing activations token by token) runs on a sequence model (the user, 2026-11-14: a lesson on a tiny transformer, not an explanation without a run). To be specified at Saga 4's planning step; filed now so the order of asks is complete | Saga 4 (weight-space), the Dust lesson | none yet: Saga 4 is not planned; if micro-gpt stays a demo, the lesson would write a one-layer attention block out from the Attention library (long) or wait |
| FM15 | not yet taken | feature | MoE-style LoRA for a mixture of experts (`docs/research6.txt`, a chat summary; the paper MoE^2-LoRA, Yang and others, arXiv:2607.21978, to be read at source at Saga 2's planning step before the signatures are fixed): a pool of n LoRA experts (`A_j`, `B_j`, rank r) and the adapter output `sum_j p_j B_j A_j h` for the k experts a router picks; three routers: none (an adapter rides a base expert and fires when the base router picks it: DAS-LoRA), from the input (`p = softmax(W_l h)`, MoELoRA), and MoE^2-LoRA's routing-conditioned projection (`p = softmax(W_l [z; W_h h])`, z the base router's logits, `W_h` a low-rank projection of the input); the gradients of `A_j`, `B_j`, `W_l`, `W_h` checked by central differences; the routing-agreement measure between the adapter router and the base router (the paper's alignment); one pool shared across layers (an index per layer into the same pool) against a pool per layer. Wanted beside FM6's library, which must expose z | moe-lora-routing (Saga 2 step 9, lesson 12) | none yet: the step is last in its saga and waits; if FM6 lands without FM15, the routers are written out in the program if that stays under about 20 lines (the pool's forward is three lines; the router gradients are the long part) |
| FM12 | withdrawn | build | Move X_eTaL-ML's pin of X_eTaL-libraries (`XETAL_LIBRARIES_COMMIT`, now 6b09aad) once that repository's Sagas 26 and 27 have landed (`e:t_exSymbols<`; Matrix's `o_rthonormal`, `p_roject`, `r_emainder`, `c_oordinates`, `e_igenvalues`, `s_ingular`, `t_runcate`, `a_ngles`), and link Matrix among the libraries it uses if its own demos want them. This repository reaches X_eTaL-libraries only through X_eTaL-ML's pin (`docs/plan.md`, F1), so a landed library ask arrives here only after this | lora-lab (FL1), lora-vs-fa (FL2, FL3), Saga 3 | not needed here any more (2026-11-05): this repository now pins X_eTaL-libraries itself (`XETAL_LIBRARIES_COMMIT`, `docs/plan.md` F1), so a landed library ask is taken by `just libs-pin`. X_eTaL-ML moves its own libraries pin when it wants them itself (FM11 needs FL1 there) |

## Where each stands (checked 2026-11-05 at X_eTaL-ML bd178de)

X_eTaL-ML delivers these in parts (the user, 2026-11-05); its own word
on each is its `docs/fine-tuning-asks.md`. At each delivery here: move
the pin (`just ml-pin`), run `just gate --full`, commit the pin alone,
remove the workaround of what landed, mark it landed.

**First delivery, pinned 2026-11-05 (bd178de, its step
fine-tuning-asks): FM1, FM2, FM3, FM5.** Each is complete against what
was asked. What taking it found:

- FM1: its `MICROSCOPE_LIBS` (library roots a dependent names, a
  library there replacing one of the same name) is exactly what this
  repository's libraries pin needs: `shared/lesson` embeds nothing
  now, and a test checks every library microscope embedded against
  the pinned clone.
- FM2: `chrome::Site`, `header_of` and `footer_of`. The header is
  used. The footer is not: its wording of the build has labels, and
  the user asked for the compact form here. Not an ask: X_eTaL-ML's
  footer is X_eTaL-ML's.
- The pin moved a line number the LoRA hello page quotes (the library
  gained `s_calingRs` above `m_erge`); the page reads the number from
  the source, so it was right, and its check no longer expects the
  number itself.

Still open there: FM4 (QLoRA and VeRA planned in its Saga 6; DoRA now
asked outright, above), FM6 (planned, its Saga 6), FM7 to FM10 (not
planned: it will judge FM7 when Saga 2 here needs the harness, and
FM8 to FM10 after each saga's planning step specifies them; they would
be a saga of its own after its Saga 6, in the order FM7, FM10, FM8,
FM9), FM11 (filed after its reading; not yet taken).

**At Saga 1's close (2026-11-13).** Saga 1 used what landed (FM1,
FM2, FM3, FM5) and wrote FM7 and FM11 out; nothing it built waits on
X_eTaL-ML. What the next sagas take there, in the order they are
needed: FM7 (the task, the measures and the comparison harness; Saga
2's step 2 is where writing it out stops being readable), FM4 (QLoRA,
DoRA and VeRA; Saga 2's steps 3 to 5, planned in X_eTaL-ML's Saga 6
except DoRA), FM13 (Hyper-Connections and mHC; the mhc saga's step 3
on, if that saga goes first), FM6 (the mixture of experts; Saga 2's
last step), then FM10, FM8 and FM9 for Sagas 3 to 5, each to be
specified in its saga's planning step. The pin is still bd178de; it
moves at the next saga's planning step.

**At the mHC saga's start (2026-11-15).** The mHC saga goes first
(the user, 2026-11-14), so the order of need is now FM13 (its steps 3
to 6; the forward of step 2 is written out), then FM7 and FM4 for Saga
2, FM6 and FM15 (adapters in a mixture, and their routing), FM10, FM8 (with FM14, the tiny transformer, for Dust), FM9.
How asks get there is settled (the user, 2026-11-14): a session in
X_eTaL-ML works from this file, as the first delivery did; its record
is its `docs/fine-tuning-asks.md`. ML functions belong there,
general ones in X_eTaL-libraries (F3), confirmed. The pin stays at
bd178de: nothing asked has landed since; it moves when FM13 lands,
after `just gate --full`, in its own commit.

**Offered back (FM5).** X_eTaL-ML invited the heat map's additions "as
a patch to it or a further ask". They exist now in
`shared/lesson/src/heat.rs`, written on `microscope::heat`: a legend
(`legend`, `key`), a note on whether the scale is shared, cells marked
as read or written by the step on show, an inspector naming the
matrix, row, column and value (`inspect`), and one tab stop with the
arrow keys moving over the cells (`step`). X_eTaL-ML is welcome to
take any of it; nothing here waits on that.
